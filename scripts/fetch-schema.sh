#!/usr/bin/env bash
# Fetch HPXML XSD schema files for a given release tag from hpxmlwg/hpxml.
#
# Usage:
#   ./scripts/fetch-schema.sh <version> [output-dir]
#   ./scripts/fetch-schema.sh --list <version>   # just list files, don't download
#
# Options:
#   --list, -l    List the files that would be fetched (no download)
#
# <version> can be:
#   - A full tag:        v4.2  v3.1  v5.0-rc1
#   - A major version:  4  v4  3  v3   (resolves to latest stable minor)
#
# Examples:
#   ./scripts/fetch-schema.sh 4              # resolves to latest stable v4.x
#   ./scripts/fetch-schema.sh v4.2           # exact tag
#   ./scripts/fetch-schema.sh v3.1 /tmp/hpxml-schema-v3
#   ./scripts/fetch-schema.sh v5.0-rc1       # pre-release, exact tag required
#
# The XSD files are auto-detected from the repository, so this works for
# any future version without updates.
#
# No API key required. Full tags use raw.githubusercontent.com (no API call).
# Major-only resolution uses the GitHub REST API (60 req/hr unauthenticated).
# Set GITHUB_TOKEN to increase the limit to 5,000 req/hr if needed.

set -euo pipefail

GITHUB_TAGS_URL="https://api.github.com/repos/hpxmlwg/hpxml/tags?per_page=100"
GITHUB_API_URL="https://api.github.com/repos/hpxmlwg/hpxml/contents/schemas"

INPUT="${1:-}"
LIST_ONLY=false

# Handle --list/-l flag
if [[ "$INPUT" == "--list" || "$INPUT" == "-l" ]]; then
  LIST_ONLY=true
  INPUT="${2:-}"
  if [[ -z "$INPUT" ]]; then
    echo "Usage: $0 --list <version>" >&2
    exit 1
  fi
fi

if [[ -z "$INPUT" ]]; then
  echo "Usage: $0 <version> [output-dir]" >&2
  echo "       $0 --list <version>" >&2
  echo "  Full tag:      v4.2  v3.1  v5.0-rc1" >&2
  echo "  Major only:    4  v4  (resolves to latest stable minor, uses GitHub API)" >&2
  exit 1
fi

# Build curl auth header if GITHUB_TOKEN is available (optional, raises rate limit).
CURL_AUTH=()
if [[ -n "${GITHUB_TOKEN:-}" ]]; then
  CURL_AUTH=(-H "Authorization: Bearer ${GITHUB_TOKEN}")
fi

# Determine if input is major-only (e.g. "4" or "v4") or a full tag.
STRIPPED="${INPUT#v}"
if [[ "$STRIPPED" =~ ^[0-9]+$ ]]; then
  MAJOR="$STRIPPED"
  echo "Resolving latest stable v${MAJOR}.x tag via GitHub API..."
  # || true prevents grep's exit code 1 (no match) from triggering set -e.
  TAG="$(curl -fsSL "${CURL_AUTH[@]}" "$GITHUB_TAGS_URL" \
    | jq -r '.[].name' \
    | grep -E "^v${MAJOR}\.[0-9]+(\.[0-9]+)?$" || true)"
  TAG="$(printf '%s\n' "$TAG" | sort -V | tail -1)"
  if [[ -z "$TAG" ]]; then
    echo "ERROR: no stable v${MAJOR}.x tags found in hpxmlwg/hpxml" >&2
    exit 1
  fi
  echo "Resolved: ${TAG}"
else
  # Full tag provided, no API call needed.
  TAG="$INPUT"
  [[ "$TAG" == v* ]] || TAG="v${TAG}"
  MAJOR=$(echo "$TAG" | sed 's/^v//; s/\..*//')
fi

# Auto-detect XSD files by listing the schemas directory via GitHub API
echo "Detecting XSD files for ${TAG}..."
XSD_FILES_JSON=$(curl -fsSL "${CURL_AUTH[@]}" "${GITHUB_API_URL}?ref=${TAG}" || echo "[]")

# Extract .xsd file names (skip directories)
XSD_FILES=()
while IFS= read -r name; do
  if [[ -n "$name" && "$name" == *.xsd ]]; then
    XSD_FILES+=("$name")
  fi
done < <(echo "$XSD_FILES_JSON" | jq -r '.[] | select(.type == "file") | .name' 2>/dev/null || true)

if [[ ${#XSD_FILES[@]} -eq 0 ]]; then
  echo "ERROR: no .xsd files found in schemas/ for tag ${TAG}" >&2
  exit 1
fi

BASE_URL="https://raw.githubusercontent.com/hpxmlwg/hpxml/${TAG}/schemas"

# List-only mode: just print the URLs and exit
if [[ "$LIST_ONLY" == true ]]; then
  for xsd in "${XSD_FILES[@]}"; do
    echo "${BASE_URL}/${xsd}"
  done
  exit 0
fi

# Default output dir: schemas/v<major>. One directory per major covers the
# namespace/codegen boundary; minor updates just overwrite in place.
OUT_DIR="${2:-schemas/v${MAJOR}}"

# Download into a temp dir first; move atomically on success to avoid leaving
# a partial or mixed schema set on failure.
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Fetching HPXML schema ${TAG} -> ${OUT_DIR}/"
for xsd in "${XSD_FILES[@]}"; do
  url="${BASE_URL}/${xsd}"
  dest="${TMP_DIR}/${xsd}"
  echo "  ${url}"
  if ! curl -fsSL "$url" -o "$dest"; then
    echo "ERROR: failed to fetch $url" >&2
    echo "Check that the tag '${TAG}' exists: https://github.com/hpxmlwg/hpxml/tags" >&2
    exit 1
  fi
done

# Sanity check all files before touching the output directory.
for xsd in "${XSD_FILES[@]}"; do
  if ! grep -q "xs:schema" "${TMP_DIR}/${xsd}"; then
    echo "ERROR: ${xsd} does not look like a valid XSD (missing xs:schema)" >&2
    exit 1
  fi
done

mkdir -p -- "$OUT_DIR"
cp "${TMP_DIR}"/*.xsd "$OUT_DIR/"

# Extract namespace and version from the xs:schema opening element.
# Use -A5 because the element spans multiple lines (version= is on a continuation line).
# Exclude the XML declaration's version="1.0" by requiring a dot-free version number won't
# match, so instead just take the last match (schema version comes after the declaration).
NAMESPACE=$(grep -o 'targetNamespace="[^"]*"' "${OUT_DIR}/HPXML.xsd" | head -1 | sed 's/targetNamespace="//;s/"//')
VERSION_ATTR=$(grep -A5 "<xs:schema" "${OUT_DIR}/HPXML.xsd" | grep -o 'version="[^"]*"' | tail -1 | sed 's/version="//;s/"//')
echo "OK: namespace=${NAMESPACE}  schema version attr=${VERSION_ATTR}"
