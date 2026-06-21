# 0044 FIRST-PROOF-CERT acceptance artifact

**Status**: IN PROGRESS
**Spec**: `specs/0044_FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC.md`
**Repository**: `joeloverbeck/tracewake`
**Authoring target commit**: `1541da274180ecd40f52583d86704990cb55e74c`
**Unified code/evidence commit U**: `9b4ac95fb5f967f545b4733911f3a3ff7a8a075c`
**Documentation-only closeout commit**: pending
**Freshness statement**: This artifact certifies only the unified baseline named above. It does not independently verify latest `main`.

This artifact is the shared evidence packet for the `0044FIRPROCER` ticket
series. It consumes predecessor certification artifacts only within their stated
scope and does not splice predecessor command output into the FIRST-PROOF-CERT
result at `U`. Archived implementation specs remain historical evidence, not
live authority.

## Header And Provenance

### Baseline Identity

| Item | Evidence |
|---|---|
| Candidate `U` | `git rev-parse HEAD` -> `9b4ac95fb5f967f545b4733911f3a3ff7a8a075c` |
| Pre-command worktree | `git status --porcelain=v1` -> empty |
| Post-command worktree | `git status --porcelain=v1` -> empty |
| `rustc` | `rustc 1.93.0 (254b59607 2026-01-19)`, host `x86_64-unknown-linux-gnu`, LLVM `21.1.8` |
| `cargo` | `cargo 1.93.0 (083ac5135 2025-12-15)` |
| `cargo-mutants` | `cargo-mutants 27.1.0` |

### Configuration Hashes

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `dfd7c841cfa60c0fee22a94b42c4cb2d4ec638cf59363f5e00bcc5ea5f99df66` |
| `Cargo.lock` | `f207f6208d64c1a7cce0daa27c00add40a52edb99b1a6ce3a1301cb44ef60a59` |
| `rust-toolchain.toml` | `027f4900829924c77f79205ed5c8d7eca01388c4737ea9c38bc509a62dab5edd` |
| `.cargo/mutants.toml` | `258109189fc4500ab88c6fa28f47d06292632ac9b274a237589e49a90ef05b11` |
| `.github/workflows/ci.yml` | `ca7274d4acebf19f388a336446aeac66dac089b8338aa8c584f6a05ef56e4639` |

Fixture source and test-source fingerprints were captured with:

```bash
find crates/tracewake-content/src/fixtures -type f -name '*.rs' -print | sort | xargs sha256sum
find crates/tracewake-core/tests crates/tracewake-content/tests crates/tracewake-tui/tests -maxdepth 2 -type f \( -name '*.rs' -o -name '*.serialized' \) -print | sort | xargs sha256sum
find tests/negative-fixtures -type f -name '*.rs' -print | sort | xargs sha256sum
```

The fixture fingerprint command covered 59 Rust fixture files plus the fixture
registry module. The integrated test-source command covered all named core,
content, and TUI integration tests, support modules, and committed serialized
golden data. The negative-fixture command covered the compile-fail/public-API
boundary fixtures.

### Predecessor Certification Consumption

| Consumed certification | Accepted implementation commit | Artifact | Use here |
|---|---|---|---|
| `P0-CERT passed` | `a3b5e3c9e896d09ed9f5b6591fb6cd7a4edd0441` | `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` | Admissibility only; no isolated command output reused. |
| `SPINE-CERT passed` | `92ba47f14998e0ea2fc95502bc3b76c5909478ca` | `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` | Admissibility only; integrated event/replay participation is re-proven at `U`. |
| `EPI-CERT passed` | `726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3` | `archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md` | Admissibility only; actor-known and possession participation are re-proven at `U`. |
| `ORD-LIFE-CERT passed` | `c819bbee0282eb83386f7b58cab752b9e639a4af` | `archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md` | Admissibility only; no-human ordinary-life participation is re-proven at `U`. |

## Command Ledger

| Command | Result | Evidence summary |
|---|---|---|
| `git rev-parse HEAD` | pass | Returned `9b4ac95fb5f967f545b4733911f3a3ff7a8a075c`. |
| `git status --porcelain=v1` | pass | Empty before certifying commands and after the required command set. |
| `rustc --version --verbose` | pass | `rustc 1.93.0`; full host/LLVM details captured in the provenance table. |
| `cargo --version --verbose` | pass | `cargo 1.93.0`; full libgit2/libcurl/OpenSSL details captured. |
| `cargo mutants --version` | pass | `cargo-mutants 27.1.0`. |
| `cargo fmt --all --check` | pass | Exit 0. |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | Exit 0; finished with no warnings promoted to errors. |
| `cargo build --workspace --all-targets --locked` | pass | Exit 0; workspace all-targets build completed. |
| `cargo test --workspace --locked` | pass | Exit 0; workspace unit, integration, and doc tests completed. |
| `cargo test --locked -p tracewake-core --test acceptance_gates --test anti_regression_guards --test ci_workflow_guards --test doc_invariant_references --test spine_conformance` | pass | Exit 0; ran the five named core tests with 12, 85, 1, 4, and 6 passing tests respectively. |
| `cargo test --locked -p tracewake-content --test fixtures_load` | pass | Exit 0; 34 passed, 0 failed. |
| `cargo test --locked -p tracewake-tui --test tui_seam_conformance` | pass | Exit 0; 2 passed, 0 failed. |
| `cargo mutants --workspace --no-shuffle --list-files` | pass | Exit 0; 60 protected files listed. |
| `cargo mutants --workspace --no-shuffle --list` | pass | Exit 0; 2,878 mutant rows listed. Full mutation execution is owned by `0044FIRPROCER-018`. |
| `cargo test --locked -p tracewake-core --test golden_scenarios` | pass | Exit 0; 16 passed, 0 failed. FIRST-PROOF-02 witness command. |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | pass | Exit 0; 32 passed, 0 failed. FIRST-PROOF-02 witness command. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-02 witness command. |
| `cargo test --locked -p tracewake-tui --test embodied_flow` | pass | Exit 0; 6 passed, 0 failed. FIRST-PROOF-02 witness command. |
| `cargo test --locked -p tracewake-tui --test command_loop_session` | pass | Exit 0; 7 passed, 0 failed. FIRST-PROOF-02 witness command. |
| `cargo test --locked -p tracewake-tui --test transcript_snapshot` | pass | Exit 0; 3 passed, 0 failed. FIRST-PROOF-02 witness command. |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | pass | Exit 0; 17 passed, 0 failed. FIRST-PROOF-03 witness command. |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | pass | Exit 0; 32 passed, 0 failed. FIRST-PROOF-03 witness command rerun. |
| `cargo test --locked -p tracewake-core --test negative_fixture_runner` | pass | Exit 0; 5 passed, 0 failed. FIRST-PROOF-03 compile-fail/negative-fixture witness command. |
| `cargo test --locked -p tracewake-content --test schema_conformance` | pass | Exit 0; 3 passed, 0 failed. FIRST-PROOF-03 schema witness command. |
| `cargo test --locked -p tracewake-content --test forbidden_content` | pass | Exit 0; 24 passed, 0 failed. FIRST-PROOF-03 fail-closed content witness command. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-03 fixture/provenance witness command rerun. |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | pass | Exit 0; 17 passed, 0 failed. FIRST-PROOF-04 witness command rerun. |
| `cargo test --locked -p tracewake-core --test golden_scenarios` | pass | Exit 0; 16 passed, 0 failed. FIRST-PROOF-04 witness command rerun. |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | pass | Exit 0; 32 passed, 0 failed. FIRST-PROOF-04 witness command rerun. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-04 fixture witness command rerun. |
| `cargo test --locked -p tracewake-tui --test adversarial_gates` | pass | Exit 0; 15 passed, 0 failed. FIRST-PROOF-04 TUI adversarial witness command. |

The clippy command briefly waited for Cargo's build-directory lock while another
read-only Cargo command finished. No tracked or generated content changed.

## Audit Inventory Census

The `0044FIRPROCER-001` census establishes the starting inventory. Downstream
point tickets append behavior-specific symbols, fixtures, events, projections,
checksums, and responsible layers under their point sections.

| Census group | Paths covered | Mutation posture at `U` | Dependent audit points | Initial status |
|---|---:|---|---|---|
| Workspace/configuration | 5 pinned config files | `.cargo/mutants.toml` included as configuration, not mutated | `FIRST-PROOF-01`, `FIRST-PROOF-18` | Hashes recorded; unchanged from command start to finish. |
| Content fixtures | 59 fixture files plus registry module | Fixture bytes are fingerprinted; behavior exercised by content/core/TUI tests, not mutation targets | `FIRST-PROOF-02` through `FIRST-PROOF-17` | Registry and fingerprint inventory captured. |
| Core integration tests | 13 named core integration tests plus support modules | Test sources are fingerprinted; production carriers are mutation targets | `FIRST-PROOF-01` through `FIRST-PROOF-17` | Workspace and named tests passed. |
| Content integration tests | 4 named content integration tests plus committed golden data | Test sources are fingerprinted; content production carriers are mutation targets | `FIRST-PROOF-01`, `FIRST-PROOF-03`, `FIRST-PROOF-06`, `FIRST-PROOF-11`, `FIRST-PROOF-15` | Workspace and named `fixtures_load` test passed. |
| TUI integration tests | 7 named TUI integration tests | Test sources are fingerprinted; TUI production carriers are mutation targets | `FIRST-PROOF-01`, `FIRST-PROOF-02`, `FIRST-PROOF-08`, `FIRST-PROOF-14`, `FIRST-PROOF-17` | Workspace and named `tui_seam_conformance` test passed. |
| Negative fixtures | compile-fail and banned-API fixture tree | Fixture/test boundary, not production mutation target | `FIRST-PROOF-03`, `FIRST-PROOF-05`, `FIRST-PROOF-11` | Fingerprints captured; workspace tests passed. |
| Production mutation file census | 60 files from `cargo mutants --workspace --no-shuffle --list-files` | Included in checked-in mutation perimeter | `FIRST-PROOF-01`, `FIRST-PROOF-18` | Census captured; full execution deferred to `0044FIRPROCER-018`. |
| Mutation row census | 2,878 rows from `cargo mutants --workspace --no-shuffle --list` | Candidate mutants for standing perimeter | `FIRST-PROOF-18` | Census captured; survivor outcome not claimed here. |

Cross-crate dependency direction is mechanically guarded by
`crates/tracewake-core/tests/anti_regression_guards.rs::workspace_dependency_posture_matches_allowlist`
and `crates/tracewake-tui/tests/tui_seam_conformance.rs`. Both ran in the
required command set and passed at `U`.

## Gate Results

| Gate | Evidence item IDs | Result |
|---|---|---|
| `EVENT` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-002-physical-replay`, `E-0044-004-observation` | pending full integrated point evidence |
| `TRUTH-FIREWALL` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-003-provenance`, `E-0044-003-content-negative`, `E-0044-004-observation`, `E-0044-004-truth-negative` | pending full integrated point evidence |
| `ACTOR-KNOWN` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-003-provenance`, `E-0044-004-observation` | pending full integrated point evidence |
| `POSSESSION-PARITY` | `E-0044-001-command-ledger`, `E-0044-001-census` | pending integrated point evidence |
| `NO-HUMAN-ORDINARY-LIFE` | `E-0044-001-command-ledger`, `E-0044-001-census` | pending integrated point evidence |
| `MISSING-PROPERTY` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-003-provenance`, `E-0044-004-observation` | pending full integrated point evidence |
| `VIEW-DEBUG-SPLIT` | `E-0044-001-command-ledger`, `E-0044-001-census` | pending integrated point evidence |
| `REPLAY` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-002-physical-replay`, `E-0044-004-observation` | pending full integrated point evidence |
| `FIXTURE-NEGATIVE` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-003-content-negative`, `E-0044-004-truth-negative` | pending full integrated point evidence |

## Scenario Family Results

| Scenario family | Evidence item IDs | Result |
|---|---|---|
| Physical custody baseline | `E-0044-001-census`, `E-0044-002-physical-replay`, `E-0044-002-embodied-view` | pass for FIRST-PROOF-02 scope |
| Expectation contradiction | `E-0044-001-census`, `E-0044-003-provenance`, `E-0044-003-content-negative`, `E-0044-004-observation`, `E-0044-004-truth-negative` | pending `FIRST-PROOF-05` and `FIRST-PROOF-06` |
| Possession parity | `E-0044-001-census` | pending `FIRST-PROOF-08` |
| Epistemic filtering | `E-0044-001-census`, `E-0044-003-provenance` | pending downstream point evidence |
| No-hidden-truth planning | `E-0044-001-census`, `E-0044-004-truth-negative` | pending downstream point evidence |
| No-human ordinary day | `E-0044-001-census` | pending `FIRST-PROOF-09` |
| Routine blocking | `E-0044-001-census` | pending `FIRST-PROOF-09`, `FIRST-PROOF-13`, `FIRST-PROOF-16` |
| Replay rebuild | `E-0044-001-census` | pending `FIRST-PROOF-10` |
| Content rejection | `E-0044-001-census` | pending `FIRST-PROOF-11` |

## Audit Point Results

### FIRST-PROOF-01 - Unified baseline, predecessor consumption, and coherent artifact set

**Result**: pass for scaffold and baseline-freeze scope.

**Positive evidence**: one baseline identity was recorded; the worktree was clean
before and after commands; the mandatory baseline, workspace, named-test, and
census commands passed at `U`; configuration, fixtures, tests, and negative
fixtures were fingerprinted; mutation file and row census commands completed;
crate direction was checked by passing conformance tests.

**Adversarial evidence**: mixed-SHA, omitted-test, and reverse-dependency claims
are guarded by the command ledger, post-command status, the explicit named test
list, `workspace_dependency_posture_matches_allowlist`, and
`tui_seam_conformance`. Any later audited source/config/test/fixture edit after
these command results requires rerunning the affected evidence before it can be
counted as certifying.

**Replay/projection/determinism**: FIRST-PROOF-01 establishes only the aggregate
baseline and command inventory. Per-scenario replay/projection ancestry remains
pending in the relevant downstream point sections.

**Responsible layers**: `doctrine`, `fixture_contract`, `test_oracle`,
`documentation status`.

### FIRST-PROOF-02 - Physical custody baseline in EVENT, embodied play, and replay

**Result**: pass for FIRST-PROOF-02 scope.

**Positive evidence**: The physical custody, action pipeline, replay, fixture,
and TUI witness commands all passed at the current audit tree. The evidence
covers a legal check/open/take/place/inspect substrate through:

- `golden_scenarios::accepted_actions_append_versioned_events`, proving accepted
  actions append versioned events;
- `golden_scenarios::check_container_records_observation_but_open_alone_does_not`,
  proving check-container observation behavior is event-backed while open alone
  is not a false observation;
- `golden_scenarios::projection_rebuild_matches_live_state`,
  `replay_checksum_matches`, and `phase3a_agent_state_replay_projection_is_deterministic`,
  proving live/replay physical and agent projection agreement;
- `golden_fixtures_run::serialized_event_log_replays_to_identical_state`,
  `fixture_fingerprints_match_frozen_goldens`, and the fixture-family load tests
  for the canonical fixture registry;
- TUI `embodied_flow`, `command_loop_session`, and `transcript_snapshot` tests,
  proving embodied local affordances, command-loop execution, debug separation,
  and deterministic transcript fingerprints.

**Adversarial evidence**: The same command set exercised locked/closed/nonlocal
and replay-tamper negatives through:

- unit and integration witnesses in `checkcontainer.rs`, `openclose.rs`, and
  `takeplace.rs` for locked, closed, wrong-source, nonlocal, and not-carried
  rejections with no accepted mutation event;
- `event_schema_replay_gates::world_item_apply_matrix_observes_state_and_precondition_failures`
  for item take/place event application and precondition failure behavior;
- `event_schema_replay_gates::replay_report_match_mismatch_pair_exposes_semantic_fingerprints`
  and `checksum_identity_distinguishes_location_routine_status_and_replay_fingerprints`
  for localized divergence and item-location checksum scope;
- `golden_scenarios::replay_detects_missing_or_reordered_event`;
- `golden_fixtures_run::routine_no_teleport_fixture_fails_remote_work_without_movement_ancestry`;
- `command_loop_session::debug_item_does_not_leak_to_following_view_or_change_checksum`
  and `transcript_snapshot` deterministic debug/embodied split tests.

**Event/replay/projection evidence**: The relevant event kinds are
`ContainerChecked`, `ItemTakenFromPlace`, `ItemRemovedFromContainer`,
`ItemPlacedInPlace`, `ItemPlacedInContainer`, open/close events, and typed
non-world inspect responses. The passing replay tests compare physical
checksums, agent checksums, event-log serialization, projection rebuild output,
and first-divergence/diff fields for deliberate perturbations.

**Responsible layers**: `fixture_contract`, `proposal_construction`,
`action_validation`, `event_append`, `event_application`, `projection`,
`replay`, `view_model`, `tui_input_binding`.

### FIRST-PROOF-03 - Source-backed expectation provenance

**Result**: pass for FIRST-PROOF-03 scope.

**Positive evidence**: The provenance command set passed at the current audit
tree. The canonical expectation path is covered by
`expectation_contradiction_001`, whose fixture metadata states that Tomas has
only a source-backed authored-prehistory expectation before absence discovery.
`golden_fixtures_run::phase2a_initial_beliefs_are_holder_and_source_backed`
checks holder and source-backed seed belief construction, and
`event_schema_replay_gates::starting_observation_and_contradiction_events_survive_replay_with_sources`
plus `belief_stale_frontier_and_witness_links_survive_projection_debug_and_replay`
prove source-event preservation through projection/debug/replay surfaces.
`hidden_truth_gates::epistemic_event_fields_survive_into_sealed_context_and_replay`,
`actor_known_local_actor_reaches_embodied_view_model_with_context_provenance`,
and `epistemic_context_projection_and_records_remain_sealed` cover the sealed
holder-known/actor-known context behavior and exclusion of unrelated/debug data.

**Adversarial evidence**: `forbidden_content` passed fail-closed tests for
malformed/missing epistemic seed fields, hidden-truth source seeding,
planner-intended initial facts without provenance, and prose-born facts.
`negative_fixture_runner` passed the compile-fail boundary registry including
`external_crate_cannot_construct_belief_literal` and
`external_crate_cannot_mutate_belief_source_or_scope`. `hidden_truth_gates`
also passed debug and hidden-truth non-contamination tests, including
`debug_truth_never_enters_holder_known_context_hash` and
`hidden_truth_metamorphism_and_provenance_deletion_are_relational`.

**Event/replay/projection evidence**: The passing tests record expectation
belief source refs, holder identities, source-event IDs, acquisition/frontier
data, context hashes, projection checksums, canonical serialization, and replay
equality for epistemic events. Seed authoring provenance remains distinguished
from live actor observation by content schema and fixture validation tests.

**Responsible layers**: `content_schema`, `content_validation`,
`fixture_contract`, `event_append`, `event_application`, `projection`,
`holder_known_context`, `replay`.

### FIRST-PROOF-04 - Absence is discovered by modeled observation, not authoritative truth

**Result**: pass for FIRST-PROOF-04 scope.

**Positive evidence**: The modeled-observation command set passed at the current
audit tree. `checkcontainer.rs::observation_event_is_caused_by_check_event`
proves `ObservationRecorded` events carry the `ContainerChecked` event as their
cause and source event. `golden_scenarios::check_container_records_observation_but_open_alone_does_not`
proves open alone does not create a false observation, while
`golden_scenarios::expected_absence_check_creates_contradiction_and_missing_belief`
proves the missing-property absence path begins with a modeled check event and
then records observation and contradiction events. `event_schema_replay_gates`
proves observation source fields, projection application, and replay equality.

**Adversarial evidence**: Closed/opaque and locked checks reject before
observation emission in `checkcontainer.rs`; hidden food/route/workplace tests
in `hidden_truth_gates` prove unobserved truth does not become actor-known input
or a transaction target; `golden_fixtures_run` covers hidden-food, unknown-route,
and actor-known source-event checks; `adversarial_gates` proves debug and TUI
surfaces do not turn hidden/debug truth into actor-visible affordances or
semantic commands.

**Event/replay/projection evidence**: The passing tests record the chain
`ContainerChecked` -> `ObservationRecorded`, including actor, container/place,
channel, observed tick, `source_event_id`, and holder-scoped projection
application. Rejected checks produce no observation event; live/replay
epistemic projections and debug observation views remain deterministic.

**Responsible layers**: `holder_known_context`, `candidate_generation`,
`proposal_construction`, `action_validation`, `event_append`,
`event_application`, `projection`, `replay`.

### FIRST-PROOF-05 - Event-sourced expectation contradiction and belief update

**Result**: pending `0044FIRPROCER-005`.

### FIRST-PROOF-06 - No culprit, suspect, clue, theft, quest, or story-sifting truth

**Result**: pending `0044FIRPROCER-006`.

### FIRST-PROOF-07 - Truth-firewall and actor-known participation across the combined corpus

**Result**: pending `0044FIRPROCER-007`.

### FIRST-PROOF-08 - Possession parity, embodied view, and debug split in the missing-property run

**Result**: pending `0044FIRPROCER-008`.

### FIRST-PROOF-09 - No-human ordinary-life and no-direct-dispatch participation

**Result**: pending `0044FIRPROCER-009`.

### FIRST-PROOF-10 - Composite replay, projection rebuild, deterministic diagnostics, and divergence localization

**Result**: pending `0044FIRPROCER-010`.

### FIRST-PROOF-11 - Fixture-negative, schema, compile-fail, and semantic content rejection

**Result**: pending `0044FIRPROCER-011`.

### FIRST-PROOF-12 - Temporal firewall: modeled temporal premises, ancestry, freshness, and non-contamination

**Result**: pending `0044FIRPROCER-012`.

### FIRST-PROOF-13 - Routine temporal premises and adaptation in the integrated no-human run

**Result**: pending `0044FIRPROCER-013`.

### FIRST-PROOF-14 - Embodied temporal rendering, possession neutrality, and debug quarantine

**Result**: pending `0044FIRPROCER-014`.

### FIRST-PROOF-15 - Temporal positive/adversarial fixture families and replay pairing

**Result**: pending `0044FIRPROCER-015`.

### FIRST-PROOF-16 - Temporal diagnostics and consolidated five-source acceptance line

**Result**: pending `0044FIRPROCER-016`.

### FIRST-PROOF-17 - Cross-gate relational, property-based, metamorphic, and integration closure

**Result**: pending `0044FIRPROCER-017`.

## Temporal Source Results

| Routed execution source | Evidence item IDs | Result |
|---|---|---|
| `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` | pending | pending `FIRST-PROOF-12` |
| `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | pending | pending `FIRST-PROOF-13` |
| `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | pending | pending `FIRST-PROOF-14` |
| `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` | pending | pending `FIRST-PROOF-15` |
| `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | pending | pending `FIRST-PROOF-16` |

## Evidence Item Ledger

### E-0044-001-baseline-freeze

- Requirement IDs: `FIRST-PROOF-01`
- Evidence status: pass
- Fingerprint scope: command transcript; raw bytes for configuration/fixture/test fingerprints
- Evidence summary: baseline identity, tool versions, clean worktree status, configuration hashes, fixture/test/negative-fixture hash commands.
- Path under test and behavior witness: workspace root; command-ledger provenance only.
- Replay/provenance ancestry: not applicable to this scaffold item.
- Sampling/exhaustiveness scope: exhaustive for the files selected by the commands named in this artifact; downstream tickets refine behavior-specific symbol census.
- Certification use: counted as certifying pass for FIRST-PROOF-01 baseline-freeze scope only.

### E-0044-001-command-ledger

- Requirement IDs: `FIRST-PROOF-01`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: mandatory baseline, workspace, named core/content/TUI tests, and mutation census commands passed at `U`.
- Path under test and behavior witness: workspace gate and named integration suites; accepted/rejected behavior details are owned by downstream point sections.
- Replay/provenance ancestry: command output includes live test execution at `U`; no predecessor output counted.
- Sampling/exhaustiveness scope: exact command set required by `0044FIRPROCER-001`.
- Certification use: counted as certifying pass for FIRST-PROOF-01 command-ledger scope only.

### E-0044-001-census

- Requirement IDs: `FIRST-PROOF-01`
- Evidence status: pass
- Fingerprint scope: raw bytes; mutation file list; mutation row list
- Evidence summary: fixture/test/negative-fixture fingerprints captured; mutation list-files reports 60 files; mutation list reports 2,878 rows.
- Path under test and behavior witness: section 6 inventory families plus mutation census; behavior witnesses are pending downstream point sections.
- Replay/provenance ancestry: not applicable.
- Sampling/exhaustiveness scope: exhaustive over the command-selected inventory; mutation full-run outcomes are pending `0044FIRPROCER-018`.
- Certification use: counted as certifying pass for FIRST-PROOF-01 census scope only.

### E-0044-002-physical-replay

- Requirement IDs: `FIRST-PROOF-02`
- Evidence status: pass
- Fingerprint scope: command transcript; replay artifact; parsed semantic content
- Evidence summary: `golden_scenarios`, `event_schema_replay_gates`, and
  `golden_fixtures_run` passed, covering event-backed physical custody,
  take/place/open/check behavior, projection rebuilds, replay checksums,
  fixture fingerprints, and replay divergence localization.
- Path under test and behavior witness: `location.rs`, `state.rs`,
  `actions/defs/openclose.rs`, `actions/defs/takeplace.rs`,
  `actions/defs/checkcontainer.rs`, `actions/defs/inspect.rs`,
  `events/apply.rs`, `projections.rs`, `replay/rebuild.rs`, and the positive
  physical-custody fixture family.
- Replay/provenance ancestry: event-log serialization, live/replay physical and
  agent checksums, first-divergence reports, and fixture fingerprints from the
  passing commands.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-02 test
  commands; not a substitute for later cross-gate capstone coverage.
- Certification use: counted as certifying pass for FIRST-PROOF-02.

### E-0044-002-embodied-view

- Requirement IDs: `FIRST-PROOF-02`
- Evidence status: pass
- Fingerprint scope: command transcript; transcript snapshot; view-model output
- Evidence summary: `embodied_flow`, `command_loop_session`, and
  `transcript_snapshot` passed, covering actor-legible local affordances,
  debug-item non-leakage, semantic command execution, and deterministic TUI
  transcript fingerprints.
- Path under test and behavior witness: `view_models.rs`, TUI `input.rs`,
  `render.rs`, `run.rs`, and `transcript.rs`.
- Replay/provenance ancestry: transcript and checksum witnesses from the
  passing TUI tests; debug rows remain non-diegetic.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-02 TUI
  commands.
- Certification use: counted as certifying pass for FIRST-PROOF-02.

### E-0044-003-provenance

- Requirement IDs: `FIRST-PROOF-03`
- Evidence status: pass
- Fingerprint scope: command transcript; parsed semantic content; replay artifact
- Evidence summary: `hidden_truth_gates`, `event_schema_replay_gates`, and
  `golden_fixtures_run` passed, covering source-backed expectation records,
  holder-known context sealing, context hashes/frontiers, projection/replay
  equality, and hidden-truth/debug exclusion.
- Path under test and behavior witness: `epistemics/belief.rs`,
  `epistemics/proposition.rs`, `epistemics/knowledge_basis.rs`,
  `epistemics/knowledge_context.rs`, `epistemics/projection.rs`,
  `agent/actor_known.rs`, `agent/transaction.rs`, `events/apply.rs`, and
  `expectation_contradiction_001`.
- Replay/provenance ancestry: source-event IDs, holder IDs, belief/proposition
  fields, context hash/frontier, projection checksums, and replay equality from
  the passing commands.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-03
  provenance commands.
- Certification use: counted as certifying pass for FIRST-PROOF-03.

### E-0044-003-content-negative

- Requirement IDs: `FIRST-PROOF-03`
- Evidence status: pass
- Fingerprint scope: command transcript; compile-fail boundary; parsed semantic content
- Evidence summary: `negative_fixture_runner`, `schema_conformance`, and
  `forbidden_content` passed, covering missing/dangling/prose/hidden-truth
  provenance rejection and public-API compile-fail guards against raw belief
  construction or source/scope mutation.
- Path under test and behavior witness: content `schema.rs`, `serialization.rs`,
  `validate.rs`, `forbidden_provenance_input_fails_closed_001`,
  `prose_born_fact_rejected_001`, and negative fixtures
  `external_crate_cannot_construct_belief_literal` /
  `external_crate_cannot_mutate_belief_source_or_scope`.
- Replay/provenance ancestry: rejected content produces no accepted runtime
  projection state; compile-fail boundaries prove protected belief records are
  not forgeable through public APIs.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-03
  negative commands.
- Certification use: counted as certifying pass for FIRST-PROOF-03.

### E-0044-004-observation

- Requirement IDs: `FIRST-PROOF-04`
- Evidence status: pass
- Fingerprint scope: command transcript; replay artifact; parsed semantic content
- Evidence summary: `hidden_truth_gates`, `golden_scenarios`,
  `event_schema_replay_gates`, and `golden_fixtures_run` passed, covering legal
  check/inspect observation, source-event closure, holder-scoped projection,
  context hashes, and replay equality.
- Path under test and behavior witness: `agent/perception.rs`,
  `epistemics/observation.rs`, `epistemics/knowledge_context.rs`,
  `epistemics/projection.rs`, `actions/defs/checkcontainer.rs`,
  `actions/defs/inspect.rs`, `actions/proposal.rs`, `actions/pipeline.rs`,
  `events/apply.rs`, and `expectation_contradiction_001`.
- Replay/provenance ancestry: `ContainerChecked` causes
  `ObservationRecorded`; observation payload records actor/container/place,
  channel, observed tick, and `source_event_id`; replay preserves the
  holder-scoped epistemic projection.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-04
  observation commands.
- Certification use: counted as certifying pass for FIRST-PROOF-04.

### E-0044-004-truth-negative

- Requirement IDs: `FIRST-PROOF-04`
- Evidence status: pass
- Fingerprint scope: command transcript; parsed semantic content
- Evidence summary: closed/locked/nonlocal/hidden/debug paths rejected or stayed
  actor-illegible in `hidden_truth_gates`, `golden_scenarios`,
  `golden_fixtures_run`, and `adversarial_gates`.
- Path under test and behavior witness: `checkcontainer.rs`,
  `hidden_food_closed_container_001`, `hidden_food_unknown_route_001`,
  `hidden_route_edge_001`, and TUI adversarial surfaces.
- Replay/provenance ancestry: rejected checks have no accepted observation
  event; debug/validator/replay truth does not synthesize actor-known absence.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-04
  negative commands.
- Certification use: counted as certifying pass for FIRST-PROOF-04.

## Replay Package

Pending downstream point evidence. `FIRST-PROOF-01` records the unified baseline
and command inventory but does not claim per-scenario replay/projection pass
outside the commands already listed.

## Command And Mutation Package

The command ledger above is the current package for `0044FIRPROCER-001`.
Mutation execution, output directories, survivor triage, and replacement verdict
posture are owned by `0044FIRPROCER-018` and the capstone ticket.

## Staged-Abstraction Declaration

This artifact currently proves only that the baseline, command ledger, fixture
inventory, test inventory, mutation census, and report anchors exist at one
clean `U`. It deliberately abstracts the downstream behavioral witnesses until
their owning tickets append positive, adversarial, replay/projection,
diagnostic, and command evidence.

This scaffold must not be read as a FIRST-PROOF-CERT pass. Pending sections
cannot count as certification evidence, and observer-only/debug/emergence rows
cannot become scheduler objectives, quality thresholds, or actor-known facts.

Failure diagnostics distinguish:

- `not implemented yet`: downstream point section still pending;
- `intentionally abstracted`: staged field is present but explicitly
  non-certifying until its owner ticket appends evidence;
- `implemented but broken`: a required command or behavior witness fails;
- `overclaimed`: a pending, sampled, observer-only, or historical row is counted
  as pass.

## EMERGE-OBS Package

Pending capstone packaging. Any `EMERGE-OBS` row added later must remain
observer-only and non-certifying unless a future upstream spec changes that
doctrine.

## Verdict

`FIRST-PROOF-CERT` verdict: pending. `FIRST-PROOF-01` scaffold and
baseline-freeze scope passed at `U`; all other audit points and mutation
execution remain pending their owning tickets.

## Residual Convention-Only Items

Manual review remains required to confirm downstream narrative wording does not
overclaim beyond evidence status. The current scaffold intentionally uses
`pending` for all not-yet-owned sections.
