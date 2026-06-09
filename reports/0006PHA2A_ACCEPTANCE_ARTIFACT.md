# 0006 Phase 2A Epistemic Hardening Acceptance Artifact

## Exact commit under test

- Commit: `9e0590d056b15d879ac02eb2556c855c080f27e4`
- Branch or PR: `main` local working branch
- Scope: tickets `0006PHA2AEPISUB-001` through `0006PHA2AEPISUB-006`, with ticket `0006PHA2AEPISUB-007` recording this artifact and runbook.

## Gates run

- `cargo fmt --all --check` - pass; formatting check completed with no diffs.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass; workspace clippy completed with no warnings.
- `cargo build --workspace --all-targets --locked` - pass; locked workspace build completed.
- `cargo test --workspace` - pass; workspace unit, integration, and doc tests completed.

## Lock-layer suites run

- `cargo test -p tracewake-core --test negative_fixture_runner --test hidden_truth_gates --test event_schema_replay_gates` - pass; compile-fail fixtures, source guards, and epistemic replay gates completed.
- `cargo test -p tracewake-tui --test adversarial_gates` - pass; TUI actor/debug contamination adversarial gates completed.
- `cargo test -p tracewake-content --test forbidden_content` - pass; content shortcut/provenance rejection gates completed.
- `cargo test -p tracewake-core --test acceptance_artifact_wording` - pass before this artifact was added; the committed artifact is also covered by the updated validator.

## Changed files

Implementation files under review:

- `clippy.toml`
- `crates/tracewake-content/src/schema.rs`
- `crates/tracewake-content/src/validate.rs`
- `crates/tracewake-content/tests/forbidden_content.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/epistemics/belief.rs`
- `crates/tracewake-core/src/epistemics/contradiction.rs`
- `crates/tracewake-core/src/epistemics/knowledge_basis.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/epistemics/observation.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs`
- `crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-core/tests/negative_fixture_runner.rs`
- `crates/tracewake-core/tests/golden_scenarios.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/tests/adversarial_gates.rs`
- `tests/negative-fixtures/external_crate_cannot_build_debug_knowledge_context/*`
- `tests/negative-fixtures/external_crate_cannot_build_debug_projection_view_without_core_debug_api/*`
- `tests/negative-fixtures/external_crate_cannot_construct_belief_literal/*`
- `tests/negative-fixtures/external_crate_cannot_construct_observation_without_source/*`
- `tests/negative-fixtures/external_crate_cannot_insert_raw_epistemic_records/*`
- `tests/negative-fixtures/external_crate_cannot_mutate_belief_source_or_scope/*`
- `tests/negative-fixtures/external_crate_cannot_mutate_contradiction_links/*`
- `tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_mode/*`
- `tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_viewer/*`
- `tests/negative-fixtures/external_crate_cannot_read_raw_epistemic_projection_maps/*`

Archive and evidence files:

- `archive/tickets/0006PHA2AEPISUB-001.md`
- `archive/tickets/0006PHA2AEPISUB-002.md`
- `archive/tickets/0006PHA2AEPISUB-003.md`
- `archive/tickets/0006PHA2AEPISUB-004.md`
- `archive/tickets/0006PHA2AEPISUB-005.md`
- `archive/tickets/0006PHA2AEPISUB-006.md`
- `reports/0006PHA2A_ACCEPTANCE_ARTIFACT.md`

## Per-requirement acceptance evidence

| Requirement | Responsible layer | Evidence | Result |
|---|---|---|---|
| `EPI-HARD-001` | `core/epistemics/knowledge_context.rs`, `core/debug_capability.rs` | Private authority-bearing `KnowledgeContext` fields, capability-gated debug context, runtime hash/scope tests, negative fixtures for context mutation/debug construction, and `hidden_truth_gates` source assertions. | pass |
| `EPI-HARD-002` | `core/epistemics/projection.rs`, `core/view_models.rs`, `tui/app.rs` | Private raw projection maps, crate-private projection inserts, public filtered context APIs, core debug view builders, TUI debug migration, and negative fixtures for raw map read/insert/debug projection access. | pass |
| `EPI-HARD-003` | `core/epistemics/{belief,observation,contradiction}.rs`, `content/schema.rs` | Private record fields, accessors/builders, content conversion without public record mutation, provenance/schema/scope unit tests, and compile-fail fixtures for literal construction and mutation. | pass |
| `EPI-HARD-004` | `clippy.toml`, `core/tests/*`, `content/tests/*`, `tui/tests/*`, negative fixtures | Source guards walk epistemic paths, raw float/hash/public-field leaks are guarded, event-schema replay gates cover epistemic kinds and unsupported payload schemas, TUI adversarial gates block debug/notebook leakage, and content aliases are rejected. | pass |
| Event/replay parity | `core/events`, `core/replay` | `event_schema_replay_gates`, replay rebuild tests, unsupported schema checks, and projection checksum tests passed under `cargo test --workspace`. | pass |
| Content provenance/no-script parity | `content/schema`, `content/validate`, `content/tests` | `forbidden_content` and `golden_fixtures_run` prove missing provenance, shortcut truth fields, prose-born facts, unsupported epistemic schema, and alias shortcut keys are rejected. | pass |
| TUI/debug proof surface | `core/view_models`, `tui/input`, `tui/debug_panels`, `tui/app` | `adversarial_gates`, TUI acceptance, and seam conformance prove actor-scoped notebook output, non-diegetic debug surfaces, possession no-transfer, and no actor-visible debug truth. | pass |
| Consumer-seam firewall | `core/actions`, `core/agent`, `tui` | Source-context validation, actor-known planner tests, hidden-truth gates, and TUI stale/direct-dispatch adversarial tests passed under the workspace run. | pass |

## Residual convention-only items

- Review must still confirm future acceptance artifacts use scoped wording for the exact commit they test. The wording validator prevents known overclaims, but it cannot prove every future narrative claim is semantically scoped.
- Review must still treat this artifact as evidence for commit `9e0590d056b15d879ac02eb2556c855c080f27e4`, not as evidence for later commits or unrelated phases.

## Scoped certification wording

Phase 2A epistemic substrate hardening remediation accepted for exact commit `9e0590d056b15d879ac02eb2556c855c080f27e4`. This contributes scoped evidence toward `EPI-CERT` and `P0-CERT`; it does not certify latest main, later-phase scope, or the full project.

Forbidden wording:

- "Tracewake is fully certified."
- "Latest main was independently verified."
- "Later Phase 2+ / Phase 3A+ systems are certified by this pass."
- "Archived specs are live authority."
- "Project is P0 certified."
- "SPINE-CERT passed."

Do not use the forbidden wording as a result claim unless a separate upstream certification process declares that outcome.
