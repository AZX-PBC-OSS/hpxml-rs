# Test Fixtures

HPXML example files for parser development and testing. All files have been validated
against their respective published XSD schemas.

Planning note: these assets are for research/fixture preparation only. They do not imply
runtime validator implementation in `hpxml-rs` yet.

## Directory structure

```
tests/data/
  v2/    -- schemaVersion 2.3, namespace http://hpxmlonline.com/2014/6
  v3/    -- schemaVersion 3.1, namespace http://hpxmlonline.com/2019/10
  v4/    -- schemaVersion 4.2, namespace http://hpxmlonline.com/2023/09
  v5/ -- schemaVersion 5.0, namespace http://hpxmlonline.com/2025/12 (from v5.0-rc1 tag)
```

## `v2/` from [hpxmlwg/hpxml@v2.3](https://github.com/hpxmlwg/hpxml/tree/v2.3/examples)

License: Apache-2.0

| File | Coverage |
|------|----------|
| `audit.xml` | Home energy audit scenario |
| `bpi2101.xml` | BPI-2101 standard scenario; multiple buildings |
| `upgrade.xml` | Upgrade package scenario; multiple buildings |

## `v3/` from [hpxmlwg/hpxml@v3.1](https://github.com/hpxmlwg/hpxml/tree/v3.1/examples)

License: Apache-2.0

| File | Coverage |
|------|----------|
| `audit.xml` | Home energy audit scenario |
| `bpi2101.xml` | BPI-2101 standard scenario; multiple buildings, extension elements |
| `upgrade.xml` | Upgrade package scenario; multiple buildings |

## `v4/` official examples from [hpxmlwg/hpxml@v4.2](https://github.com/hpxmlwg/hpxml/tree/v4.2/examples)

License: Apache-2.0

| File | Coverage |
|------|----------|
| `audit.xml` | Home energy audit scenario |
| `bpi2101.xml` | BPI-2101; 3 buildings, extension elements with idref patterns |
| `invalid.xml` | Intentionally invalid (for rejection / error-handling tests) |
| `upgrade.xml` | Upgrade package; 2 buildings |

## `v5/` official examples from [hpxmlwg/hpxml@v5.0-rc1](https://github.com/hpxmlwg/hpxml/tree/v5.0-rc1/examples)

License: Apache-2.0
Note: upstream rc1 examples use `schemaVersion="5.0"` with the v5 namespace URI.

| File | Coverage |
|------|----------|
| `audit.xml` | Home energy audit scenario (v5 enum sentinels in schema) |
| `bpi2101.xml` | BPI-2101; extension elements with idref patterns |
| `invalid.xml` | Intentionally invalid (for rejection / error-handling tests) |
| `upgrade.xml` | Upgrade package scenario; multiple buildings |

## `v4/` samples from [NREL/OpenStudio-HPXML@v1.10.0](https://github.com/NREL/OpenStudio-HPXML/tree/v1.10.0/workflow/sample_files)

License: BSD-3-Clause

### Core

| File | Source filename | Coverage |
|------|----------------|----------|
| `minimal.xml` | *(hand-crafted)* | Smallest valid HPXML file; only required elements |
| `maximal.xml` | *(generated from `schemas/v4/HPXML.xsd`)* | Synthetic maximal-style v4 fixture for broad schema coverage |
| `openst-base.xml` | `base.xml` | Full baseline single-family home; all major sections |

### Per-section variants

| File | Source filename | Coverage |
|------|----------------|----------|
| `openst-appliances.xml` | `base-appliances-gas.xml` | Appliances with gas fuel |
| `openst-appliances-none.xml` | `base-appliances-none.xml` | Empty appliances section |
| `openst-atticroof.xml` | `base-atticroof-cathedral.xml` | Cathedral ceiling / attic roof |
| `openst-battery.xml` | `base-battery.xml` | Battery storage |
| `openst-bldgtype.xml` | `base-bldgtype-mf-unit-adjacent-to-other-housing-unit.xml` | Multifamily unit |
| `openst-dhw.xml` | `base-dhw-tankless-gas.xml` | Domestic hot water (tankless gas) |
| `openst-dhw-multiple.xml` | `base-dhw-multiple.xml` | Multiple water heaters |
| `openst-dhw-none.xml` | `base-dhw-none.xml` | No DHW systems |
| `openst-enclosure.xml` | `base-enclosure-windows-none.xml` | Enclosure without windows |
| `openst-ev.xml` | `base-ev-charger.xml` | EV charger |
| `openst-foundation.xml` | `base-foundation-slab.xml` | Slab foundation |
| `openst-foundation-multiple.xml` | `base-foundation-multiple.xml` | Multiple foundation types |
| `openst-hvac.xml` | `base-hvac-central-ac-only-1-speed.xml` | Single HVAC system |
| `openst-hvac-multiple.xml` | `base-hvac-multiple.xml` | Multiple HVAC systems |
| `openst-hvac-none.xml` | `base-hvac-none.xml` | No HVAC systems |
| `openst-lighting.xml` | `base-lighting-none.xml` | No lighting elements |
| `openst-mechvent.xml` | `base-mechvent-cfis.xml` | Mechanical ventilation (CFIS) |
| `openst-mechvent-multiple.xml` | `base-mechvent-multiple.xml` | Multiple ventilation systems |
| `openst-misc-loads-none.xml` | `base-misc-loads-none.xml` | No misc loads |
| `openst-pv.xml` | `base-pv.xml` | Photovoltaic system |
| `openst-schedules.xml` | `base-schedules-detailed-setpoints.xml` | Detailed schedule setpoints |
| `openst-additional-properties.xml` | `base-misc-additional-properties.xml` | `extensionType` / `xs:any` with special chars |
| `openst-zones-spaces.xml` | `base-zones-spaces.xml` | Zones and Spaces (v4.2 feature) |
| `openst-zones-spaces-multiple.xml` | `base-zones-spaces-multiple.xml` | Multiple zones and spaces |

## Coverage matrix

| Concern | Files |
|---------|-------|
| Version detection (v2) | `v2/*` |
| Version detection (v3) | `v3/*` |
| Version detection (v4) | `v4/*` |
| Version detection (v5) | `v5/*` |
| Multiple buildings | `v4/bpi2101.xml` (3), `v4/upgrade.xml` (2) |
| Empty / none sections | `openst-hvac-none`, `openst-dhw-none`, `openst-appliances-none`, `openst-misc-loads-none`, `openst-lighting` |
| Multiple items per collection | `openst-hvac-multiple`, `openst-dhw-multiple`, `openst-mechvent-multiple`, `openst-foundation-multiple`, `openst-zones-spaces-multiple` |
| `extensionType` / `xs:any` | `openst-additional-properties`, `v4/bpi2101.xml`, most `openst-*` files |
| Zones / Spaces (v4.2) | `openst-zones-spaces`, `openst-zones-spaces-multiple` |
| Invalid input rejection | `v4/invalid.xml` |
| Minimum required elements | `v4/minimal.xml` |
| High-coverage large fixture | `v4/maximal.xml` |
| Schematron business-rule testing | `schematron/openstudio/EPvalidator.sch` |

## `schematron/openstudio/` from [NatLabRockies/OpenStudio-HPXML](https://github.com/NatLabRockies/OpenStudio-HPXML/tree/master/HPXMLtoOpenStudio/resources/hpxml_schematron)

License: BSD-3-Clause

| File | Coverage |
|------|----------|
| `EPvalidator.sch` | Use-case/business-rule assertions used by OpenStudio-HPXML validation flows |
| `iso-schematron.xsd` | Schematron schema support file used by the rule set/tooling |

Direct sources:
- `EPvalidator.sch`: https://raw.githubusercontent.com/NatLabRockies/OpenStudio-HPXML/master/HPXMLtoOpenStudio/resources/hpxml_schematron/EPvalidator.sch
- `iso-schematron.xsd`: https://raw.githubusercontent.com/NatLabRockies/OpenStudio-HPXML/master/HPXMLtoOpenStudio/resources/hpxml_schematron/iso-schematron.xsd

## `validators/openstudio/` from [NatLabRockies/OpenStudio-HPXML](https://github.com/NatLabRockies/OpenStudio-HPXML/tree/master/HPXMLtoOpenStudio/resources)

License: BSD-3-Clause

| File | Coverage |
|------|----------|
| `xmlvalidator.rb` | Validator wrapper used by OpenStudio-HPXML to execute XSD/Schematron checks |

Direct source:
- `xmlvalidator.rb`: https://raw.githubusercontent.com/NatLabRockies/OpenStudio-HPXML/master/HPXMLtoOpenStudio/resources/xmlvalidator.rb

## Schema Sources (`schemas/v4/`)

Current local schema files are stored under `schemas/v4/` and sourced from the HPXMLWG repo:
- `HPXML.xsd`: https://raw.githubusercontent.com/hpxmlwg/hpxml/v4.2/schemas/HPXML.xsd
- `HPXMLBaseElements.xsd`: https://raw.githubusercontent.com/hpxmlwg/hpxml/v4.2/schemas/HPXMLBaseElements.xsd
- `HPXMLDataTypes.xsd`: https://raw.githubusercontent.com/hpxmlwg/hpxml/v4.2/schemas/HPXMLDataTypes.xsd

## `schematron/examples/` local planning profiles (non-official)

These are local planning fixtures, not official HPXML Toolbox rule bundles:
- `ira_modeled_required_6.example.sch`
- `ira_measured_required_5.example.sch`
- `home_energy_score_use_case_1.example.sch`
- `der_custom.example.sch`

See `tests/data/schematron/examples/README.md` for expected pass/fail examples.

Source-search note: public repositories/pages searched on 2026-03-07 did not yield
official downloadable `.sch` files for Toolbox profile names
`IraModeledRequired`, `IraMeasuredRequired`, or `HomeEnergyScoreUseCase`.

## Refreshing

To re-download all upstream files, run from the repo root:

```sh
BASE2="https://raw.githubusercontent.com/hpxmlwg/hpxml/v2.3/examples"
curl -s "$BASE2/audit.xml"   -o tests/data/v2/audit.xml
curl -s "$BASE2/bpi2101.xml" -o tests/data/v2/bpi2101.xml
curl -s "$BASE2/upgrade.xml" -o tests/data/v2/upgrade.xml

BASE3="https://raw.githubusercontent.com/hpxmlwg/hpxml/v3.1/examples"
curl -s "$BASE3/audit.xml"   -o tests/data/v3/audit.xml
curl -s "$BASE3/bpi2101.xml" -o tests/data/v3/bpi2101.xml
curl -s "$BASE3/upgrade.xml" -o tests/data/v3/upgrade.xml

BASE4="https://raw.githubusercontent.com/hpxmlwg/hpxml/v4.2/examples"
curl -s "$BASE4/audit.xml"   -o tests/data/v4/audit.xml
curl -s "$BASE4/bpi2101.xml" -o tests/data/v4/bpi2101.xml
curl -s "$BASE4/invalid.xml" -o tests/data/v4/invalid.xml
curl -s "$BASE4/upgrade.xml" -o tests/data/v4/upgrade.xml

BASE5="https://raw.githubusercontent.com/hpxmlwg/hpxml/v5.0-rc1/examples"
curl -s "$BASE5/audit.xml"   -o tests/data/v5/audit.xml
curl -s "$BASE5/bpi2101.xml" -o tests/data/v5/bpi2101.xml
curl -s "$BASE5/invalid.xml" -o tests/data/v5/invalid.xml
curl -s "$BASE5/upgrade.xml" -o tests/data/v5/upgrade.xml

OSTS="https://raw.githubusercontent.com/NREL/OpenStudio-HPXML/v1.10.0/workflow/sample_files"
curl -s "$OSTS/base.xml"                                                     -o tests/data/v4/openst-base.xml
curl -s "$OSTS/base-appliances-gas.xml"                                      -o tests/data/v4/openst-appliances.xml
curl -s "$OSTS/base-appliances-none.xml"                                     -o tests/data/v4/openst-appliances-none.xml
curl -s "$OSTS/base-atticroof-cathedral.xml"                                 -o tests/data/v4/openst-atticroof.xml
curl -s "$OSTS/base-battery.xml"                                             -o tests/data/v4/openst-battery.xml
curl -s "$OSTS/base-bldgtype-mf-unit-adjacent-to-other-housing-unit.xml"    -o tests/data/v4/openst-bldgtype.xml
curl -s "$OSTS/base-dhw-tankless-gas.xml"                                    -o tests/data/v4/openst-dhw.xml
curl -s "$OSTS/base-dhw-multiple.xml"                                        -o tests/data/v4/openst-dhw-multiple.xml
curl -s "$OSTS/base-dhw-none.xml"                                            -o tests/data/v4/openst-dhw-none.xml
curl -s "$OSTS/base-enclosure-windows-none.xml"                              -o tests/data/v4/openst-enclosure.xml
curl -s "$OSTS/base-ev-charger.xml"                                          -o tests/data/v4/openst-ev.xml
curl -s "$OSTS/base-foundation-slab.xml"                                     -o tests/data/v4/openst-foundation.xml
curl -s "$OSTS/base-foundation-multiple.xml"                                 -o tests/data/v4/openst-foundation-multiple.xml
curl -s "$OSTS/base-hvac-central-ac-only-1-speed.xml"                        -o tests/data/v4/openst-hvac.xml
curl -s "$OSTS/base-hvac-multiple.xml"                                       -o tests/data/v4/openst-hvac-multiple.xml
curl -s "$OSTS/base-hvac-none.xml"                                           -o tests/data/v4/openst-hvac-none.xml
curl -s "$OSTS/base-lighting-none.xml"                                       -o tests/data/v4/openst-lighting.xml
curl -s "$OSTS/base-mechvent-cfis.xml"                                       -o tests/data/v4/openst-mechvent.xml
curl -s "$OSTS/base-mechvent-multiple.xml"                                   -o tests/data/v4/openst-mechvent-multiple.xml
curl -s "$OSTS/base-misc-loads-none.xml"                                     -o tests/data/v4/openst-misc-loads-none.xml
curl -s "$OSTS/base-misc-additional-properties.xml"                          -o tests/data/v4/openst-additional-properties.xml
curl -s "$OSTS/base-pv.xml"                                                  -o tests/data/v4/openst-pv.xml
curl -s "$OSTS/base-schedules-detailed-setpoints.xml"                        -o tests/data/v4/openst-schedules.xml
curl -s "$OSTS/base-zones-spaces.xml"                                        -o tests/data/v4/openst-zones-spaces.xml
curl -s "$OSTS/base-zones-spaces-multiple.xml"                               -o tests/data/v4/openst-zones-spaces-multiple.xml
UV_CACHE_DIR=/home/rich/src/hpxml-rs/.uv-cache ./scripts/gen_maximal_v4_from_xsd.py

SCH="https://raw.githubusercontent.com/NatLabRockies/OpenStudio-HPXML/master/HPXMLtoOpenStudio/resources/hpxml_schematron"
mkdir -p tests/data/schematron/openstudio
curl -s "$SCH/EPvalidator.sch"    -o tests/data/schematron/openstudio/EPvalidator.sch
curl -s "$SCH/iso-schematron.xsd" -o tests/data/schematron/openstudio/iso-schematron.xsd

VAL="https://raw.githubusercontent.com/NatLabRockies/OpenStudio-HPXML/master/HPXMLtoOpenStudio/resources"
mkdir -p tests/data/validators/openstudio
curl -s "$VAL/xmlvalidator.rb" -o tests/data/validators/openstudio/xmlvalidator.rb
```

`tests/data/v4/minimal.xml` is hand-crafted. Do not overwrite it from upstream.
