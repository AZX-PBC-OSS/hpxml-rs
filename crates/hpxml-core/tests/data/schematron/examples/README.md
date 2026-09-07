# Example Schematron Profiles (Planning Only)

These files are **not** official HPXML Toolbox rule bundles.
They are local planning examples to unblock validation API design and test wiring.

## Files

- `ira_modeled_required_6.example.sch`
- `ira_measured_required_5.example.sch`
- `home_energy_score_use_case_1.example.sch`
- `der_custom.example.sch`

## Expected Behavior Against Existing Fixtures

These expectations are heuristic checks based on fixture content:

| Rule file | Expected pass examples | Expected fail examples |
|---|---|---|
| `ira_modeled_required_6.example.sch` | `tests/data/v4/openst-base.xml` | `tests/data/v4/minimal.xml` |
| `ira_measured_required_5.example.sch` | `tests/data/v4/openst-base.xml` | `tests/data/v4/minimal.xml` |
| `home_energy_score_use_case_1.example.sch` | `tests/data/v4/openst-base.xml` | `tests/data/v4/minimal.xml` |
| `der_custom.example.sch` | `tests/data/v4/openst-pv.xml`, `tests/data/v4/openst-battery.xml`, `tests/data/v4/openst-ev.xml` | `tests/data/v4/openst-base.xml`, `tests/data/v4/minimal.xml` |

## Provenance

No public `.sch` files for these exact Toolbox profile names were located in:
- `hpxmlwg/hpxml` public repo
- `NatLabRockies/OpenStudio-HPXML` public repo (contains `EPvalidator.sch`)
- HPXML Toolbox public validator/API pages

If official IRA/HES Schematron bundles become available, replace these examples.
