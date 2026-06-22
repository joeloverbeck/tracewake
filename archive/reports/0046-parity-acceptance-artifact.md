# 0046 TUI Simulation Playable-Capability Parity Acceptance Artifact

## Exact commit under test

- Implementation baseline commit: `3b9241f6c7484fcc771288f69880385b8556b056`
- Capstone evidence files added in this ticket: `crates/tracewake-tui/tests/parity_adversarial.rs`
  and this report.

## Gates run

- `cargo test -p tracewake-tui --test parity_adversarial` - pass; 4 passed, 0 failed, 0 ignored.
- `cargo test -p tracewake-tui --test playable_capability_parity` - pass; 8 passed, 0 failed, 0 ignored.
- `cargo fmt --all --check` - pass.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass.
- `cargo build --workspace --all-targets --locked` - pass.
- `cargo test --workspace` - pass; includes `parity_adversarial`, `playable_capability_parity`,
  `adversarial_gates`, `tui_seam_conformance`, transcript, replay, no-human, fixture, and doc tests.

## Changed files

- `crates/tracewake-tui/tests/parity_adversarial.rs`
- `reports/0046-parity-acceptance-artifact.md`

## Parity evidence block

- Target implementation commit: `3b9241f6c7484fcc771288f69880385b8556b056`
- Fixture/content fingerprints: frozen fixture fingerprint test passed; representative parity fixtures
  include `strongbox_001 = twf1-c89a51158996ae87`, `place_carried_item_001 =
  twf1-4dfecc00c36642db`, and `no_human_day_001 = twf1-9b8d796c5201bb66`.
- Capability entries in scope: 21 base entries: 15 registered action entries and 6 non-action family
  entries. The non-action families are debug quarantine, epistemic filtering, needs/routines,
  no-human autonomy, notebook/leads, and rejection why-not.
- Generated coverage report: `playable_capability_runner_passes_live_registry_and_reports_deterministically`
  proves deterministic report emission with `capability_count=21`.
- Typed causal witnesses: every entry carries a typed witness, and the real-pipeline scenario test
  asserts typed -> actor-knowledge -> rendered witness ordering.
- Actor-known witnesses: every entry carries an actor-knowledge witness; adversarial witness deletion
  fails with `missing_actor_knowledge_witness`.
- Rendered golden paths/digests: 21 checked-in references under
  `crates/tracewake-tui/tests/goldens/`; the scenario test compares rendered output to those files.
- Anti-leak and debug-quarantine evidence: anti-leak entries reject debug/hidden tokens; injected
  `DEBUG NON-DIEGETIC` / `food_hidden_pantry` text fails the actor-surface check.
- Replay/no-human disposition: parity entries record required or not-applicable replay/no-human
  evidence; `cargo test --workspace` passed the no-human, replay, transcript, and determinism suites.
- Compiler/source-conformance evidence: `tui_seam_conformance` passed; adversarial source-guard
  removal fails; controlled compile breaks below show new fields/variants are rejected until handled.
- Exact commands and verdicts: listed under `Gates run`.

This package does not reduce parity acceptance to screenshots or display strings. Rendered references
are counted only with typed causal, actor-known, anti-leak, replay/no-human, and source-conformance
evidence.

## Per-requirement acceptance evidence

| Requirement | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `PAR-001` | `tui/view-model` | `E-001`, `E-006` | pass |
| `PAR-002` | `tui/render` | `E-004`, `E-005`, `E-006` | pass |
| `PAR-003` | `tui/render`, `core/view-model` | `E-005`, `E-006` | pass |
| `PAR-004` through `PAR-009` | `tui/parity-suite` | `E-001`, `E-002`, `E-003`, `E-007` | pass |
| `PAR-010` and `PAR-011` | `docs/ci` | `E-008` | pass |
| `PAR-012` | `tui/parity-suite`, `compiler` | `E-003`, `E-004`, `E-005` | pass |

## Evidence item ledger

- Evidence item ID: `E-001`
- Requirement IDs: `PAR-004`, `PAR-005`, `PAR-006`, `PAR-007`, `PAR-008`, `PAR-009`
- Evidence status: `pass`
- Fingerprint scope: `command transcript`
- Evidence summary: `cargo test -p tracewake-tui --test playable_capability_parity` passed 8 tests.
- Path under test and behavior witness:
  - path under test: live parity registry, conformance runner, real TUI pipeline, checked-in goldens;
  - command: `cargo test -p tracewake-tui --test playable_capability_parity`;
  - responsible layer: `tui/parity-suite`;
  - accepted/rejected stage witnessed: registry validation, registered-action coverage, real pipeline
    scenario rendering, golden comparison, deterministic report output, synthetic gap failures;
  - live negative: synthetic gaps fail for duplicate keys, missing fixtures, missing rendered witness,
    missing actor-knowledge witness, missing anti-leak, missing action, empty render, and uncovered
    action;
  - checked facts: 15 action definitions and 6 family entries are covered with zero uncovered entries.
- Replay/provenance ancestry: scenario witnesses are produced through `TuiApp::from_golden`, actor bind,
  current view, action submission where applicable, render, and checked-in golden comparison.
- Sampling/exhaustiveness scope: exhaustive over current `ActionRegistry::definitions()` plus the base
  non-action family census.
- Certification use: `counted as certifying pass`.

- Evidence item ID: `E-002`
- Requirement IDs: `PAR-005`, `PAR-006`, `PAR-007`
- Evidence status: `pass`
- Fingerprint scope: `raw bytes`
- Evidence summary: 21 checked-in render references under `crates/tracewake-tui/tests/goldens/`.
- Path under test and behavior witness:
  - path under test: rendered output from the real TUI pipeline;
  - command: `cargo test -p tracewake-tui --test playable_capability_parity`;
  - responsible layer: `tui/render`;
  - accepted/rejected stage witnessed: actual rendered text must match checked-in references;
  - live negative: changed render output fails golden comparison;
  - checked facts: action, family, why-not, notebook, debug, and no-human surfaces are pinned.
- Replay/provenance ancestry: golden files are read from package-local test assets and compared against
  render output after fixture load and actor binding.
- Sampling/exhaustiveness scope: exhaustive over the 21-entry parity registry.
- Certification use: `counted as certifying pass`.

- Evidence item ID: `E-003`
- Requirement IDs: `PAR-012`
- Evidence status: `pass`
- Fingerprint scope: `command transcript`
- Evidence summary: `cargo test -p tracewake-tui --test parity_adversarial` passed 4 tests.
- Path under test and behavior witness:
  - path under test: conformance runner, registered-action coverage check, anti-leak surface check,
    render source guard;
  - command: `cargo test -p tracewake-tui --test parity_adversarial`;
  - responsible layer: `tui/parity-suite`;
  - accepted/rejected stage witnessed: witness deletion, uncovered action, hidden/debug leak injection,
    removed source guard target, and rest-pattern insertion all fail;
  - live negative: each adversarial mutation is executed in the test target;
  - checked facts: guards bite on the violation shapes named by `PAR-012`.
- Replay/provenance ancestry: negative tests are observer-only and do not create actor knowledge or
  passing embodied witnesses.
- Sampling/exhaustiveness scope: targeted adversarial closure for the named guard classes.
- Certification use: `counted as certifying pass`.

- Evidence item ID: `E-004`
- Requirement IDs: `PAR-002`, `PAR-012`
- Evidence status: `pass`
- Fingerprint scope: `command transcript`
- Evidence summary: controlled compile-break, temporary `EmbodiedViewModel` field.
- Transcript:
  - Temporary edit: add `pub parity_compile_break_field: String` to `EmbodiedViewModel`.
  - First `cargo check -p tracewake-tui --tests`: failed with `E0063`, missing field
    `parity_compile_break_field` in the production `EmbodiedViewModel` initializer.
  - Temporary constructor disposition added only to reach the render tripwire.
  - Second `cargo check -p tracewake-tui --tests`: failed with `E0027`, pattern does not mention field
    `parity_compile_break_field` in `crates/tracewake-tui/src/render.rs:14`.
  - Temporary edits reverted; `git diff -- crates/tracewake-core/src/view_models.rs
    crates/tracewake-core/src/projections.rs` produced no output.
- Path under test and behavior witness:
  - path under test: core view-model growth and TUI exhaustive destructure;
  - responsible layer: `core/view-model`, `tui/render`;
  - accepted/rejected stage witnessed: compile rejects undispositioned field growth;
  - live negative: temporary field cannot be silently ignored.
- Replay/provenance ancestry: compile transcript only; no simulation state created.
- Sampling/exhaustiveness scope: targeted compile-break runbook.
- Certification use: `counted as certifying pass`.

- Evidence item ID: `E-005`
- Requirement IDs: `PAR-003`, `PAR-012`
- Evidence status: `pass`
- Fingerprint scope: `command transcript`
- Evidence summary: controlled compile-break, temporary `VisibleItemSource` variant.
- Transcript:
  - Temporary edit: add `TemporaryCompileBreak` to closed enum `VisibleItemSource`.
  - First `cargo check -p tracewake-tui --tests`: failed with `E0004`, non-exhaustive pattern in
    `crates/tracewake-core/src/projections.rs:1148`.
  - Temporary upstream arm added only to reach the TUI render owner.
  - Second `cargo check -p tracewake-tui --tests`: failed with `E0004`, non-exhaustive pattern in
    `crates/tracewake-tui/src/render.rs:213`.
  - Temporary edits reverted; `git diff -- crates/tracewake-core/src/view_models.rs
    crates/tracewake-core/src/projections.rs` produced no output.
- Path under test and behavior witness:
  - path under test: closed presentation enum growth and TUI render match;
  - responsible layer: `core/view-model`, `tui/render`;
  - accepted/rejected stage witnessed: compile rejects undispositioned closed enum variant;
  - live negative: temporary variant cannot be swallowed by wildcard handling.
- Replay/provenance ancestry: compile transcript only; no simulation state created.
- Sampling/exhaustiveness scope: targeted compile-break runbook.
- Certification use: `counted as certifying pass`.

- Evidence item ID: `E-006`
- Requirement IDs: `PAR-002`, `PAR-003`
- Evidence status: `pass`
- Fingerprint scope: `command transcript`
- Evidence summary: `cargo test --workspace` passed `tui_seam_conformance`.
- Path under test and behavior witness:
  - path under test: render destructure, closed presentation enum matches, TUI debug source boundary;
  - command: `cargo test --workspace`;
  - responsible layer: `tui/view-model`, `tui/debug`;
  - accepted/rejected stage witnessed: no `..` rest pattern, no wildcard enum arms, no raw projection
    storage access;
  - live negative: source-guard synthetics in tests fail on wildcard/rest/removal shapes.
- Replay/provenance ancestry: source guard only.
- Sampling/exhaustiveness scope: enrolled TUI seam source guards.
- Certification use: `counted as certifying pass`.

- Evidence item ID: `E-007`
- Requirement IDs: `PAR-009`, `PAR-011`
- Evidence status: `pass`
- Fingerprint scope: `command transcript`
- Evidence summary: CI lane names `TUI playable-capability parity` and workspace tests include the
  parity target; local `cargo test --workspace` also ran both parity targets.
- Path under test and behavior witness:
  - path under test: ordinary CI evidence lane;
  - command: `cargo test --workspace`;
  - responsible layer: `workspace/ci`;
  - accepted/rejected stage witnessed: parity target is not ignored and runs under ordinary tests;
  - live negative: not applicable for the workflow itself; adversarial target covers guard failures.
- Replay/provenance ancestry: command transcript.
- Sampling/exhaustiveness scope: workspace test lane.
- Certification use: `counted as certifying pass`.

- Evidence item ID: `E-008`
- Requirement IDs: `PAR-010`, `PAR-DOC-001` through `PAR-DOC-007`
- Evidence status: `pass`
- Fingerprint scope: `file/line doctrine`
- Evidence summary: tickets 009 through 011 amended architecture, execution, reference, and template
  doctrine so future Expansion specs carry parity-impact declarations and acceptance artifacts carry
  parity evidence.
- Path under test and behavior witness:
  - path under test: docs authority surfaces;
  - command: doc grep checks and `cargo test -p tracewake-core --test doc_invariant_references --locked`;
  - responsible layer: `doctrine`;
  - accepted/rejected stage witnessed: no new gate/risk/invariant code minted; glossary term remains
    deferred;
  - live negative: doc invariant reference test rejects malformed invariant references.
- Replay/provenance ancestry: archived ticket outcomes for 009, 010, and 011.
- Sampling/exhaustiveness scope: doctrine surfaces named by spec 0046 section 5.
- Certification use: `counted as certifying pass`.

## Staged-abstraction declaration

This artifact does not stage a future mechanic. It certifies the parity guard mechanism and evidence
shape for current TUI-facing simulation capabilities. The glossary term remains deliberately deferred
until an owner ratifies the final mechanism name; that deferral does not weaken the implemented
registry, runner, CI lane, or acceptance-template evidence block.

## Residual convention-only items

- The controlled compile-break transcript is manual by design. It temporarily breaks the tree and is
  recorded here instead of running in CI.
- The final report cannot self-reference a commit hash that includes itself. The implementation
  baseline commit is recorded above; the ticket closeout commit contains this report and the
  adversarial target.

## Scoped certification wording

Allowed wording:

- "Spec 0046 TUI simulation playable-capability parity contract accepted for the implementation
  baseline and capstone evidence recorded here. This contributes scoped evidence toward TUI/view-model
  acceptance and future Expansion parity-impact review; it does not certify latest main after this
  commit, later feature scope, Phase 4 entry, or the full project."

Forbidden wording:

- "Tracewake is fully certified."
- "Latest main was independently verified."
- "The parity contract certifies every future feature automatically."
- "Screenshots or display strings alone satisfy TUI parity."
- "Debug truth can satisfy embodied parity."
