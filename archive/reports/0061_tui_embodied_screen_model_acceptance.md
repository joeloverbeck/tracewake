# 0061 TUI Embodied Screen Model Acceptance

**Status**: ACCEPTED

## Scope

This artifact records the scoped Spec 0061 TUI embodied screen-model and
headless dump evidence. It is not a certification audit, latest-main
certification, Phase-4 entry claim, or doctrine amendment.

Implementation/conformance baseline commit:
`5b479c8b62499a62f7ec8aa6b6640142402672be`.

Evidence/report commit role: the capstone commit that adds this report, the
fixed-size dump goldens, the structured snapshot, and the source-census
classification update needed by the existing workspace guard.

## Evidence Items

### Fixed-size plain-text dump goldens

The `ordinary_workday_001` fixture is rendered through:

`EmbodiedViewModel -> EmbodiedScreenModel -> render_embodied_screen_dump`.

Tracked goldens:

- `crates/tracewake-tui/tests/goldens/embodied_screen_80x24.txt`
- `crates/tracewake-tui/tests/goldens/embodied_screen_100x30.txt`
- `crates/tracewake-tui/tests/goldens/embodied_screen_60x20.txt`

The current framework-free renderer carries the terminal size in the `SCREEN`
header. Pane content is otherwise byte-identical across the three fixed sizes.

### Structured ScreenDump snapshot

Tracked snapshot:

- `crates/tracewake-tui/tests/goldens/embodied_screen_dump.snapshot.txt`

The structured snapshot is projected from the same `EmbodiedScreenModel` pane
source as the plain-text dump. It carries mode, terminal size, focused pane,
per-pane dumps, action refs, `debug_marker_present`, view-model id, and
holder-known context hash. It deliberately omits holder-known frontier and
source-summary fields.

### Field-disposition conformance

`crates/tracewake-tui/tests/tui_seam_conformance.rs` registers
`embodied_screen_model_field_disposition` in `TUI_SEAM_EVIDENCE` under
`TUI-0061-004`. The guard enumerates the current 23-field
`EmbodiedViewModel` disposition set and verifies every field maps to a named
`EmbodiedScreenModel` pane or metadata disposition without wildcard/default
laundering.

### Debug-token absence and determinism

`crates/tracewake-tui/tests/embodied_screen_dump.rs` proves:

- no plain-text dump contains `DEBUG NON-DIEGETIC`;
- no structured per-pane dump contains `DEBUG NON-DIEGETIC`;
- `debug_marker_present` is `false` for the embodied fixture dump;
- repeated screen-model build plus plain-text and structured dumps are
  byte-identical for identical inputs.

## Verification

Commands run on 2026-07-01 after this report file existed:

- `cargo test -p tracewake-tui --test embodied_screen_dump` passed.
- `cargo test -p tracewake-tui` passed.
- `cargo test --workspace` passed.

Pre-report corrective run:

- `cargo test --workspace` first failed because the existing production-source
  census did not yet classify the new `crates/tracewake-tui/src/screen/*.rs`
  files.
- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree` passed after adding the screen files to the existing TUI source classification table.

## Verdict

Spec 0061 screen dumps are accepted as first-class scoped review artifacts for
the implemented TUI screen-model seam. The evidence proves the requested
framework-free dump seam, structured snapshot, field-disposition guard,
debug-token hygiene, and deterministic dump behavior for `ordinary_workday_001`.

This verdict does not certify latest main, Phase-4 entry, second-proof entry,
institutions, notices, travel, LOD, LLM/speech, story-sifting,
`P0-CERT`, `FIRST-PROOF-CERT`, or any whole-project status.
