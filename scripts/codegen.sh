#!/usr/bin/env bash
# Run HPXML code generation from XSD schemas.
#
# This script:
# 1. Fetches schemas if needed (via fetch-schema.sh)
# 2. Runs codegen-runner to generate Rust code
#
# Usage:
#   ./scripts/codegen.sh [versions]
#
# Arguments:
#   versions  Comma-separated list of versions (default: v2,v3,v4,v5)
#
# Example:
#   ./scripts/codegen.sh v3,v4,v5

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SCHEMAS_DIR="$REPO_ROOT/schemas"

# Parse version list (default: v2,v3,v4,v5)
IFS=',' read -ra VERSIONS <<< "${1:-v2,v3,v4,v5}"

# Fetch any missing schemas
NEED_FETCH=()
for ver in "${VERSIONS[@]}"; do
    if [[ ! -f "$SCHEMAS_DIR/$ver/HPXML.xsd" ]]; then
        NEED_FETCH+=("$ver")
    fi
done

if [[ ${#NEED_FETCH[@]} -gt 0 ]]; then
    echo "Fetching HPXML schemas for: ${NEED_FETCH[*]}..."
    for ver in "${NEED_FETCH[@]}"; do
        "$SCRIPT_DIR/fetch-schema.sh" "$ver" "$SCHEMAS_DIR/$ver"
    done
    echo "Schemas fetched."
else
    echo "Schemas already present."
fi

# Run codegen-runner
echo "Running codegen..."
cd "$REPO_ROOT"
cargo run --release --manifest-path scripts/codegen-runner/Cargo.toml -- "$@"

exit 0
