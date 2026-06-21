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
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | pass | Exit 0; 32 passed, 0 failed. FIRST-PROOF-05 witness command rerun. |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | pass | Exit 0; 17 passed, 0 failed. FIRST-PROOF-05 witness command rerun. |
| `cargo test --locked -p tracewake-core --test generative_lock` | pass | Exit 0; 5 passed, 0 failed. FIRST-PROOF-05 deterministic/generative witness command. |
| `cargo test --locked -p tracewake-core --test negative_fixture_runner` | pass | Exit 0; 5 passed, 0 failed. FIRST-PROOF-05 compile-fail witness command rerun. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-05 fixture/replay witness command rerun. |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | pass | Exit 0; 17 passed, 0 failed. FIRST-PROOF-06 hidden-truth firewall witness command rerun. |
| `cargo test --locked -p tracewake-core --test anti_regression_guards` | pass | Exit 0; 85 passed, 0 failed. FIRST-PROOF-06 structural guard witness command. |
| `cargo test --locked -p tracewake-content --test forbidden_content` | pass | Exit 0; 24 passed, 0 failed. FIRST-PROOF-06 semantic content rejection witness command rerun. |
| `cargo test --locked -p tracewake-content --test schema_conformance` | pass | Exit 0; 3 passed, 0 failed. FIRST-PROOF-06 schema mapping witness command rerun. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-06 fixture/hidden-truth witness command rerun. |
| `cargo test --locked -p tracewake-tui --test adversarial_gates` | pass | Exit 0; 15 passed, 0 failed. FIRST-PROOF-06 debug/TUI quarantine witness command rerun. |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | pass | Exit 0; 17 passed, 0 failed. FIRST-PROOF-07 actor-known/firewall witness command rerun. |
| `cargo test --locked -p tracewake-core --test acceptance_gates` | pass | Exit 0; 12 passed, 0 failed. FIRST-PROOF-07 action-pipeline/replay witness command. |
| `cargo test --locked -p tracewake-core --test generative_lock` | pass | Exit 0; 5 passed, 0 failed. FIRST-PROOF-07 determinism/metamorphic witness command rerun. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-07 fixture/context-hash witness command rerun. |
| `cargo test --locked -p tracewake-tui --test adversarial_gates` | pass | Exit 0; 15 passed, 0 failed. FIRST-PROOF-07 debug-on/off quarantine witness command rerun. |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | pass | Exit 0; 17 passed, 0 failed. FIRST-PROOF-08 actor-known/debug witness command rerun. |
| `cargo test --locked -p tracewake-core --test golden_scenarios` | pass | Exit 0; 16 passed, 0 failed. FIRST-PROOF-08 possession/replay witness command rerun. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-08 possession fixture witness command rerun. |
| `cargo test --locked -p tracewake-tui --test adversarial_gates` | pass | Exit 0; 15 passed, 0 failed. FIRST-PROOF-08 possession/debug negative witness command rerun. |
| `cargo test --locked -p tracewake-tui --test embodied_flow` | pass | Exit 0; 6 passed, 0 failed. FIRST-PROOF-08 embodied view witness command rerun. |
| `cargo test --locked -p tracewake-tui --test tui_acceptance` | pass | Exit 0; 11 passed, 0 failed. FIRST-PROOF-08 TUI possession/debug acceptance witness command. |
| `cargo test --locked -p tracewake-tui --test transcript_snapshot` | pass | Exit 0; 3 passed, 0 failed. FIRST-PROOF-08 transcript determinism witness command rerun. |
| `cargo test --locked -p tracewake-core --test no_human_capstone` | pass | Exit 0; 2 passed, 0 failed. FIRST-PROOF-09 no-human capstone witness command. |
| `cargo test --locked -p tracewake-core --test acceptance_gates` | pass | Exit 0; 12 passed, 0 failed. FIRST-PROOF-09 pipeline/no-marker witness command rerun. |
| `cargo test --locked -p tracewake-core --test spine_conformance` | pass | Exit 0; 6 passed, 0 failed. FIRST-PROOF-09 no-direct-dispatch conformance witness command rerun. |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | pass | Exit 0; 32 passed, 0 failed. FIRST-PROOF-09 replay/event ancestry witness command rerun. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-09 fixture/progress witness command rerun. |
| `cargo test --locked -p tracewake-tui --test tui_acceptance` | pass | Exit 0; 11 passed, 0 failed. FIRST-PROOF-09 TUI no-human/debug witness command rerun. |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | pass | Exit 0; 32 passed, 0 failed. FIRST-PROOF-10 event/replay witness command rerun. |
| `cargo test --locked -p tracewake-core --test generative_lock` | pass | Exit 0; 5 passed, 0 failed. FIRST-PROOF-10 deterministic/metamorphic witness command rerun. |
| `cargo test --locked -p tracewake-core --test golden_scenarios` | pass | Exit 0; 16 passed, 0 failed. FIRST-PROOF-10 scenario replay witness command rerun. |
| `cargo test --locked -p tracewake-core --test no_human_capstone` | pass | Exit 0; 2 passed, 0 failed. FIRST-PROOF-10 integrated capstone replay witness command rerun. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-10 fixture replay/determinism witness command rerun. |
| `cargo test --locked -p tracewake-tui --test transcript_snapshot` | pass | Exit 0; 3 passed, 0 failed. FIRST-PROOF-10 deterministic transcript witness command rerun. |
| `cargo test --locked -p tracewake-core --test negative_fixture_runner` | pass | Exit 0; 5 passed, 0 failed. FIRST-PROOF-11 compile-fail/negative-fixture witness command rerun. |
| `cargo test --locked -p tracewake-core --test anti_regression_guards` | pass | Exit 0; 85 passed, 0 failed. FIRST-PROOF-11 structural guard witness command rerun. |
| `cargo test --locked -p tracewake-content --test fixtures_load` | pass | Exit 0; 34 passed, 0 failed. FIRST-PROOF-11 fixture load/canonicalization witness command rerun. |
| `cargo test --locked -p tracewake-content --test forbidden_content` | pass | Exit 0; 24 passed, 0 failed. FIRST-PROOF-11 semantic rejection witness command rerun. |
| `cargo test --locked -p tracewake-content --test schema_conformance` | pass | Exit 0; 3 passed, 0 failed. FIRST-PROOF-11 schema conformance witness command rerun. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | Exit 0; 42 passed, 0 failed. FIRST-PROOF-11 golden fixture/corpus witness command rerun. |

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
| `EVENT` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-002-physical-replay`, `E-0044-004-observation`, `E-0044-005-contradiction-replay`, `E-0044-009-no-human-progress`, `E-0044-009-no-direct-dispatch`, `E-0044-010-composite-replay`, `E-0044-010-controlled-divergence` | pending full integrated point evidence |
| `TRUTH-FIREWALL` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-003-provenance`, `E-0044-003-content-negative`, `E-0044-004-observation`, `E-0044-004-truth-negative`, `E-0044-006-no-culprit`, `E-0044-006-content-negative`, `E-0044-007-actor-known-noninterference`, `E-0044-007-validation-fail-closed` | pending full integrated point evidence |
| `ACTOR-KNOWN` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-003-provenance`, `E-0044-004-observation`, `E-0044-005-contradiction-replay`, `E-0044-007-actor-known-noninterference`, `E-0044-007-validation-fail-closed` | pending full integrated point evidence |
| `POSSESSION-PARITY` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-008-possession-parity`, `E-0044-008-debug-split` | pending integrated point evidence |
| `NO-HUMAN-ORDINARY-LIFE` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-009-no-human-progress`, `E-0044-009-no-direct-dispatch` | pending integrated point evidence |
| `MISSING-PROPERTY` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-003-provenance`, `E-0044-004-observation`, `E-0044-005-contradiction-replay`, `E-0044-006-no-culprit` | pending full integrated point evidence |
| `VIEW-DEBUG-SPLIT` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-006-no-culprit`, `E-0044-007-actor-known-noninterference`, `E-0044-007-validation-fail-closed`, `E-0044-008-possession-parity`, `E-0044-008-debug-split` | pending integrated point evidence |
| `REPLAY` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-002-physical-replay`, `E-0044-004-observation`, `E-0044-005-contradiction-replay`, `E-0044-009-no-human-progress`, `E-0044-009-no-direct-dispatch`, `E-0044-010-composite-replay`, `E-0044-010-controlled-divergence` | pending full integrated point evidence |
| `FIXTURE-NEGATIVE` | `E-0044-001-command-ledger`, `E-0044-001-census`, `E-0044-003-content-negative`, `E-0044-004-truth-negative`, `E-0044-006-content-negative`, `E-0044-011-load-schema-canonicalization`, `E-0044-011-semantic-rejection-compilefail` | pending full integrated point evidence |

## Scenario Family Results

| Scenario family | Evidence item IDs | Result |
|---|---|---|
| Physical custody baseline | `E-0044-001-census`, `E-0044-002-physical-replay`, `E-0044-002-embodied-view` | pass for FIRST-PROOF-02 scope |
| Expectation contradiction | `E-0044-001-census`, `E-0044-003-provenance`, `E-0044-003-content-negative`, `E-0044-004-observation`, `E-0044-004-truth-negative`, `E-0044-005-contradiction-replay`, `E-0044-005-contradiction-negative`, `E-0044-006-no-culprit`, `E-0044-006-content-negative` | pass for expectation/absence/contradiction/no-culprit scope |
| Possession parity | `E-0044-001-census`, `E-0044-008-possession-parity`, `E-0044-008-debug-split` | pass for FIRST-PROOF-08 scope |
| Epistemic filtering | `E-0044-001-census`, `E-0044-003-provenance`, `E-0044-007-actor-known-noninterference`, `E-0044-007-validation-fail-closed`, `E-0044-008-possession-parity` | pass for FIRST-PROOF-07 actor-known/firewall and FIRST-PROOF-08 possession scope |
| No-hidden-truth planning | `E-0044-001-census`, `E-0044-004-truth-negative`, `E-0044-006-no-culprit`, `E-0044-006-content-negative`, `E-0044-007-actor-known-noninterference`, `E-0044-007-validation-fail-closed` | pass for FIRST-PROOF-07 non-interference scope |
| No-human ordinary day | `E-0044-001-census`, `E-0044-009-no-human-progress`, `E-0044-009-no-direct-dispatch` | pass for FIRST-PROOF-09 scope |
| Routine blocking | `E-0044-001-census`, `E-0044-009-no-human-progress`, `E-0044-009-no-direct-dispatch` | pass for FIRST-PROOF-09 scope; pending temporal/routine capstone in `FIRST-PROOF-13`/`FIRST-PROOF-16` |
| Replay rebuild | `E-0044-001-census`, `E-0044-009-no-human-progress`, `E-0044-009-no-direct-dispatch`, `E-0044-010-composite-replay`, `E-0044-010-controlled-divergence` | pass for FIRST-PROOF-10 scope |
| Content rejection | `E-0044-001-census`, `E-0044-006-content-negative`, `E-0044-011-load-schema-canonicalization`, `E-0044-011-semantic-rejection-compilefail` | pass for FIRST-PROOF-11 scope |

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

**Result**: pass for FIRST-PROOF-05 scope.

**Positive evidence**: The contradiction-path command set passed at the current
audit tree. `epistemics/contradiction.rs` unit tests prove contradiction records
link the prior expectation belief and contradicting observation, require matching
holder and expectation stance, and require relevant item/container absence.
`event_schema_replay_gates::starting_observation_and_contradiction_events_survive_replay_with_sources`
proves the ordered expectation -> observation -> contradiction segment applies
through normal event application and survives replay with source refs intact.
`event_schema_replay_gates::belief_stale_frontier_and_witness_links_survive_projection_debug_and_replay`
proves the resulting belief links, contradiction IDs, debug rendering, checksum
input, and replay projection remain stable. `hidden_truth_gates` also proves the
holder-known/debug projection boundary for contradiction records.

**Adversarial evidence**: The contradiction unit tests cover wrong holder,
wrong stance, wrong container/place, present item, and absent expectation cases
returning no detection. `negative_fixture_runner` covers the
`external_crate_cannot_mutate_contradiction_links` compile-fail boundary, so
external code cannot mutate the private prior-expectation or observation links.
`generative_lock` passed deterministic ordering/replay metamorphic checks, and
`golden_fixtures_run` passed replay tamper, source-event tamper, and fixture
determinism checks around the same epistemic/event substrate.

**Event/replay/projection evidence**: The passing tests record event IDs for the
source expectation, observation, contradiction, and updated belief; event causes
and payload versions; holder IDs; prior expectation and contradicting
observation IDs; projection checksum lines containing contradiction links; debug
belief/contradiction rows; and replay equality from a rebuilt projection. Replay
tamper tests provide localized first-divergence evidence for perturbed logs.

**Responsible layers**: `event_append`, `event_application`, `projection`,
`holder_known_context`, `replay`, `test_oracle`.

### FIRST-PROOF-06 - No culprit, suspect, clue, theft, quest, or story-sifting truth

**Result**: pass for FIRST-PROOF-06 scope.

**Positive evidence**: The no-culprit command set passed at the current audit
tree. `hidden_truth_gates` proved hidden/debug truth stays out of actor-known
contexts, transaction targets, embodied affordances, and holder-known context
hashes. `golden_fixtures_run` proved the no-hidden-truth fixture keeps hidden
food out of planner inputs, the planner trace exposes hidden-truth audit and
selection rejections, and hidden food/route blockers remain local
knowledge-blockers rather than suspect-selection machinery. The missing-property
fixture family therefore represents physical custody plus expectation, absence,
and contradiction records without adding a culprit, suspect, clue, quest, theft,
lead-board, or story objective fact.

**Adversarial evidence**: `forbidden_content` passed fail-closed tests for
quest/reward/player/script constructs, outcome-chain routine markers,
prose-born facts, hidden-truth source seeding, shortcut truth fields, and
planner-intended facts without provenance. `anti_regression_guards` passed
structural guards that keep hidden-truth audit derived from provenance instead
of tags, keep cognition inputs context-backed, and prevent debug/display labels
from becoming authority. `adversarial_gates` proved debug truth does not enter
actor surfaces, debug command strings are not embodied commands, TUI rule
inference cannot apply a hidden food target, and debug/no-human output remains
non-diegetic.

**Event/replay/projection evidence**: The passing tests exercise ordinary
physical and epistemic event records, holder-known projections, planner
contexts, debug rendering, fixture validation, and replayed fixture contracts.
They do not certify institutional investigation, accusation success, sanctions,
or a story-sifting objective; any future suspicion-like behavior must still be
actor-known and provenance-backed rather than an authoritative guilt fact.

**Responsible layers**: `content_schema`, `content_validation`,
`fixture_contract`, `holder_known_context`, `candidate_generation`,
`proposal_construction`, `action_validation`, `projection`, `view_model`,
`tui_input_binding`, `debug_adapter`, `test_oracle`.

### FIRST-PROOF-07 - Truth-firewall and actor-known participation across the combined corpus

**Result**: pass for FIRST-PROOF-07 scope.

**Positive evidence**: The actor-known/truth-firewall command set passed at the
current audit tree. `hidden_truth_gates` proved actor-known contexts are
unforgeable from hidden truth, debug omniscience is excluded from planner
contexts, hidden food and hidden routes do not become transaction targets or
route plans, workplaces require assignment or observation provenance, and
epistemic event fields survive into sealed context and replay. `acceptance_gates`
proved accepted proposals append deterministic versioned events, rejected
physical proposals return structured rejections, human/nonhuman proposals share
validation paths, and Phase-3A agent events apply live and replay to the same
agent checksum. `golden_fixtures_run` proved no-human decision inputs cite log
events and recompute context hashes, source-evidence tamper fails the decision
context-hash gate, and no-hidden-truth fixture planner inputs stay actor-known.

**Adversarial evidence**: `generative_lock` passed deterministic seed-order,
replay, and metamorphic locks. `hidden_truth_gates` passed hidden-truth
metamorphism/provenance-deletion, hidden-route, hidden-food, debug-truth, and
sealed-context negatives. `adversarial_gates` passed debug-truth non-entry,
typed non-leaking why-not facts, hidden-food TUI rule-inference rejection,
stale/forged privileged input rejection, debug-panel no-mutation, and
debug-output quarantine tests. Validation truth therefore rejects stale or
impossible proposals without authoring a fallback goal or revealing
actor-illegible alternatives.

**Event/replay/projection evidence**: The passing tests cover sealed context
IDs/hashes/frontiers, source-event refs, candidate/proposal provenance,
structured rejection records, accepted events, replay checksums, fixture
fingerprints, and debug/TUI outputs. A modeled observation or notice can legally
change a later decision; hidden truth, debug rows, or validator-only data do not
change pre-observation actor-known outputs.

**Responsible layers**: `holder_known_context`, `candidate_generation`,
`proposal_construction`, `action_validation`, `event_append`,
`event_application`, `projection`, `replay`, `view_model`, `debug_adapter`,
`test_oracle`.

### FIRST-PROOF-08 - Possession parity, embodied view, and debug split in the missing-property run

**Result**: pass for FIRST-PROOF-08 scope.

**Positive evidence**: The possession/debug command set passed at the current
audit tree. `golden_scenarios` proved controller binding and possession
controller binding are not world state, replay/projection checksums remain
stable, and the missing-property belief path still does not support truthful
accusation. `golden_fixtures_run::possession_fixture_preserves_intention_needs_and_can_continue`
proved possession preserves the actor's intention/needs/routine continuity.
`embodied_flow` proved embodied views derive from actor-known context rather
than hidden truth, including workplace belief/truth split cases and
bind-render-submit-rerender/why-not flow. `tui_acceptance` proved deterministic
possession/debug transcripts and absence discovery without culprit leakage.

**Adversarial evidence**: `hidden_truth_gates` proved debug truth does not enter
holder-known context hashes, embodied affordances, or planner inputs.
`adversarial_gates` proved possession rebind does not transfer notebook/debug
truth or rejection why-not state, debug panels do not change embodied
affordances, debug command strings are not embodied commands, stale/forged TUI
inputs reject, and rendering does not change typed order or replay.
`transcript_snapshot` proved byte-identical debug and TUI transcripts across
runs.

**Event/replay/projection evidence**: The passing tests cover controller
ownership/input-origin routing, actor-known context hashes, intention/need
continuity, embodied view fingerprints, debug transcript snapshots, structured
TUI submissions, authoritative event checksums, and replay equality. Binding,
unbinding, rebinding, and debug attach/detach change only the appropriate
operator/debug surfaces; they do not reset cognition, add memory, reveal hidden
custody, create a special action, or mutate authoritative world state.

**Responsible layers**: `controller_binding`, `holder_known_context`,
`proposal_construction`, `action_validation`, `event_append`,
`event_application`, `projection`, `replay`, `view_model`, `tui_input_binding`,
`debug_adapter`, `test_oracle`.

### FIRST-PROOF-09 - No-human ordinary-life and no-direct-dispatch participation

**Result**: pass for FIRST-PROOF-09 scope.

**Positive evidence**: The no-human ordinary-life command set passed at the
current audit tree. `no_human_capstone` proved typed ancestry and replay for the
capstone run and proved debug/replay perturbations are non-interfering.
`acceptance_gates` proved the no-human day runner uses no controller and
pipeline events, the integrated no-human day emerges from one autonomous run,
human and nonhuman proposals share the validation path, sleep proposals share
the pipeline, and `continue_routine` markers alone are not behavioral progress.
`golden_fixtures_run` proved no-human decision inputs cite log events and
recompute context hashes, no-human workplace and sleep knowledge require notice
or observation channels, no-human day metrics and need-ledger outputs validate,
and the no-human day real run replays metrics and trace projection.

**Adversarial evidence**: `spine_conformance` proved the conformance matrix
requires negative evidence for no-direct and debug quarantine, and maps spine
requirements to named evidence. `event_schema_replay_gates` proved agent no-op
allowlists remain narrow, materialized agent events reject forged schemas,
duration/need/routine tamper rejects live and replay, no-human metrics rebuild
from typed diagnostic fields, and no-human replay checksums match originals.
`golden_fixtures_run` also covered routine blocked diagnostics, no-teleport
remote work rejection, food-unavailable typed failure without refill, and
continue-routine tamper poisoning. `tui_acceptance` proved no-human day runs
through the TUI and inspects real post-run panels while debug surfaces remain
read-only.

**Event/replay/projection evidence**: The passing tests cover ordinary
transactions, modeled wait/stuck/failure outcomes, need-accounting ledgers,
intention/routine events, typed diagnostics, no-human metrics, projection
checksums, event-log replay, and localized tamper rejection. Progress is a real
accepted action, modeled wait, or typed blocked/failure outcome; scheduler and
routine labels do not author primitive state deltas or count marker-only output
as progress.

**Responsible layers**: `scheduler`, `candidate_generation`,
`proposal_construction`, `action_validation`, `event_append`,
`event_application`, `need_accounting`, `projection`, `replay`,
`view_model`, `debug_adapter`, `test_oracle`.

### FIRST-PROOF-10 - Composite replay, projection rebuild, deterministic diagnostics, and divergence localization

**Result**: pass for FIRST-PROOF-10 scope.

**Positive evidence**: The composite replay command set passed at the current
audit tree. `event_schema_replay_gates` proved event-envelope identity,
epistemic/materialized-agent event registration and replay routing,
live/replay rejection parity, physical checksum partitioning, deterministic
context hashes, replay report semantic fingerprints, no-human metric rebuilds,
and no-human day checksum equality. `golden_scenarios` proved accepted actions
append versioned events, projection rebuilds match live state, replay checksums
match, Phase-3A agent projections are deterministic, and no-human metrics are
byte-identical after log replay. `no_human_capstone` proved typed ancestry and
replay for the integrated capstone and debug/replay perturbation
non-interference. `golden_fixtures_run` proved serialized event logs replay to
identical state, fixture fingerprints match frozen goldens, fixture loading is
deterministic, and every golden fixture family validates and replays.

**Adversarial evidence**: `event_schema_replay_gates` passed unsupported schema
append/replay rejection, unsupported epistemic payload rejection, duplicate
duration/need ticks, missing intention/routine progress, malformed elapsed
ticks, stream mismatch, forged schema versions, payload lies, and replay
mismatch first-divergence tests. `generative_lock` passed generated sequence
replay, seed-order determinism, progress/lifecycle relational locks, and
duration-terminal tamper checks. `golden_scenarios` passed missing/reordered
event divergence, while `golden_fixtures_run` passed continuation, episode, and
source-evidence tamper tests. `transcript_snapshot` proved byte-stable TUI and
debug transcripts.

**Event/replay/projection evidence**: The passing tests cover stable event IDs,
causes, stream/phase metadata, schema versions, deterministic ordering,
physical checksums, epistemic projection checksums, agent/routine/accounting
checksums, no-human metrics, diagnostic fingerprints, serialized logs, replay
reports, and transcript snapshots. Controlled perturbations reject or localize
the first divergence; replay starts from declared fixture/content data and does
not repair missing actor-known facts from current truth.

**Responsible layers**: `event_append`, `event_application`, `projection`,
`replay`, `checksum`, `fixture_contract`, `content_serialization`,
`debug_adapter`, `view_model`, `test_oracle`.

### FIRST-PROOF-11 - Fixture-negative, schema, compile-fail, and semantic content rejection

**Result**: pass for FIRST-PROOF-11 scope.

**Positive evidence**: The fixture-negative/schema command set passed at the
current audit tree. `fixtures_load` proved all fixtures declare contracts,
scope, action/report/assertion metadata, and explicit need seeds; all fixtures
load deterministically and validate; authored prehistory seed events are
emitted; source refs use stable typed canonical encoding; raw and secondary
fixture bytes reprice fingerprints; and valid epistemic seeds validate and
round-trip canonically. `schema_conformance` proved fixture scope registration
and content-spine requirements map to named tests. `golden_fixtures_run` proved
the canonical fixture corpus validates, rejects missing IDs/bad references, and
keeps fixture fingerprints frozen.

**Adversarial evidence**: `forbidden_content` passed fail-closed tests for
prose-born facts, malformed epistemic seed fields, hidden-truth source seeding,
planner-intended facts without provenance, shortcut truth fields, quest/reward
player/script constructs, outcome-chain routine markers, debug-label
acceptance gaming, unknown tokens, truncated serialization, and validator
branch-matrix guards. `fixtures_load` passed unsupported schema, forbidden
top-level key, unknown field, prose-born raw-line, duplicate/dangling reference,
missing need/tuning, blanket known-food helper, ambiguous assignment, and
contract denial rejections. `negative_fixture_runner` proved registered
compile-fail fixtures and banned API negative fixtures fail as expected, while
`anti_regression_guards` kept protected state mutation, hidden-truth audit,
actor-known construction, and source-guard boundaries live.

**Event/replay/projection evidence**: The passing tests distinguish raw-byte,
canonical-content, fixture-registry, behavior, compile-fail, and replay
fingerprint scopes. Rejected content produces no accepted event log,
authoritative state, projection, or actor-known fact; compile-fail fixtures
prove external callers cannot forge protected belief, observation,
contradiction, or debug records.

**Responsible layers**: `content_schema`, `content_validation`,
`fixture_contract`, `event_append`, `projection`, `replay`, `test_oracle`.

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

### E-0044-005-contradiction-replay

- Requirement IDs: `FIRST-PROOF-05`
- Evidence status: pass
- Fingerprint scope: command transcript; replay artifact; parsed semantic content
- Evidence summary: `event_schema_replay_gates`, `hidden_truth_gates`, and
  `golden_fixtures_run` passed, covering event-sourced contradiction creation,
  linked belief/observation records, holder-scoped projections, debug rendering,
  replay equality, and localized replay divergence for tampered logs.
- Path under test and behavior witness: `epistemics/contradiction.rs`,
  `epistemics/belief.rs`, `epistemics/observation.rs`,
  `epistemics/proposition.rs`, `epistemics/projection.rs`, `events/envelope.rs`,
  `events/log.rs`, `events/apply.rs`, `replay/rebuild.rs`, `replay/report.rs`,
  `checksum.rs`, and `expectation_contradiction_001`.
- Replay/provenance ancestry: source expectation event, observation event,
  contradiction event, and updated belief event IDs/causes are applied and then
  replayed from serialized event-log input with stable projection checksums.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-05
  event/replay commands.
- Certification use: counted as certifying pass for FIRST-PROOF-05.

### E-0044-005-contradiction-negative

- Requirement IDs: `FIRST-PROOF-05`
- Evidence status: pass
- Fingerprint scope: command transcript; compile-fail boundary; parsed semantic content
- Evidence summary: `generative_lock` and `negative_fixture_runner` passed,
  while contradiction unit tests covered wrong-holder, wrong-stance,
  wrong-target, present-item, absent-expectation, determinism, and link-forgery
  rejection.
- Path under test and behavior witness: `epistemics/contradiction.rs`,
  `tests/negative-fixtures/external_crate_cannot_mutate_contradiction_links/src/lib.rs`,
  generative replay/order tests, and golden fixture tamper tests.
- Replay/provenance ancestry: invalid or unrelated premises produce no accepted
  contradiction; external mutation of private contradiction links fails at
  compile time; deterministic generated runs and replay tamper checks preserve
  stable semantics.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-05
  negative/generative commands.
- Certification use: counted as certifying pass for FIRST-PROOF-05.

### E-0044-006-no-culprit

- Requirement IDs: `FIRST-PROOF-06`
- Evidence status: pass
- Fingerprint scope: command transcript; parsed semantic content; debug/view output
- Evidence summary: `hidden_truth_gates`, `anti_regression_guards`,
  `golden_fixtures_run`, and `adversarial_gates` passed, covering the absence of
  actor-visible culprit/suspect/hidden-target machinery in holder-known,
  planner, embodied, debug, and TUI surfaces.
- Path under test and behavior witness: `agent/actor_known.rs`,
  `agent/transaction.rs`, `epistemics/knowledge_context.rs`,
  `epistemics/projection.rs`, `actions/defs/accuseprobe.rs`, TUI adversarial
  surfaces, and the no-hidden-truth / knowledge-blocker fixture families.
- Replay/provenance ancestry: ordinary physical and epistemic records remain
  source-backed; debug truth and hidden physical truth do not synthesize
  actor-known accusation targets or decision inputs.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-06
  no-culprit/debug/TUI commands.
- Certification use: counted as certifying pass for FIRST-PROOF-06.

### E-0044-006-content-negative

- Requirement IDs: `FIRST-PROOF-06`
- Evidence status: pass
- Fingerprint scope: command transcript; parsed semantic content
- Evidence summary: `forbidden_content`, `schema_conformance`, and
  `golden_fixtures_run` passed, covering fail-closed rejection of quest,
  script, outcome-chain, shortcut truth, hidden-truth seed, planner-intended
  fact, and prose-born fact content.
- Path under test and behavior witness: content `schema.rs`, `validate.rs`,
  `serialization.rs`, `forbidden_content` fixtures,
  `knowledge_blocker_accuse_001`, `prose_born_fact_rejected_001`, and
  `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`.
- Replay/provenance ancestry: rejected content creates no runtime event,
  projection, or actor-known fact; accepted fixture runs keep blockers
  provenance-backed and hidden truth non-actor-known.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-06
  content/schema/fixture commands.
- Certification use: counted as certifying pass for FIRST-PROOF-06.

### E-0044-007-actor-known-noninterference

- Requirement IDs: `FIRST-PROOF-07`
- Evidence status: pass
- Fingerprint scope: command transcript; replay artifact; parsed semantic content
- Evidence summary: `hidden_truth_gates`, `acceptance_gates`,
  `generative_lock`, `golden_fixtures_run`, and `adversarial_gates` passed,
  covering sealed actor-known context hashes/frontiers, context-backed
  transaction inputs, deterministic proposal/replay behavior, and debug-on/off
  quarantine.
- Path under test and behavior witness: `agent/actor_known.rs`,
  `agent/transaction.rs`, `agent/decision.rs`, `agent/candidate.rs`,
  `agent/generation.rs`, `agent/planner.rs`, `agent/trace.rs`,
  `epistemics/knowledge_context.rs`, `epistemics/knowledge_basis.rs`,
  `actions/proposal.rs`, `actions/pipeline.rs`, and the hidden-food,
  hidden-route, workplace, no-human, and planner-trace fixture families.
- Replay/provenance ancestry: actor-known contexts enumerate source-backed
  facts, source refs, context hashes, and frontiers; accepted decisions append
  versioned events and replay to matching checksums.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-07
  actor-known/non-interference commands.
- Certification use: counted as certifying pass for FIRST-PROOF-07.

### E-0044-007-validation-fail-closed

- Requirement IDs: `FIRST-PROOF-07`
- Evidence status: pass
- Fingerprint scope: command transcript; parsed semantic content; debug/view output
- Evidence summary: `hidden_truth_gates`, `acceptance_gates`,
  `generative_lock`, `golden_fixtures_run`, and `adversarial_gates` passed,
  covering forged/stale context rejection, hidden-truth provenance-deletion
  failure, structured proposal rejection, source-evidence tamper rejection, and
  non-leaking actor-visible diagnostics.
- Path under test and behavior witness: action validation and proposal
  pipeline tests, hidden-truth/provenance deletion gates, no-human context-hash
  tamper tests, stale TUI selection and forged privileged semantic ID tests,
  debug rendering, and why-not diagnostic surfaces.
- Replay/provenance ancestry: invalid or stale inputs produce structured
  rejections or no accepted event; debug/validator truth does not create a
  fallback goal, hidden alternative, actor-known fact, or replayed event.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-07
  fail-closed/debug-quarantine commands.
- Certification use: counted as certifying pass for FIRST-PROOF-07.

### E-0044-008-possession-parity

- Requirement IDs: `FIRST-PROOF-08`
- Evidence status: pass
- Fingerprint scope: command transcript; replay artifact; view-model output
- Evidence summary: `golden_scenarios`, `golden_fixtures_run`,
  `embodied_flow`, `tui_acceptance`, and `transcript_snapshot` passed, covering
  input-only controller binding, possession continuity, embodied actor-known
  views, deterministic transcripts, and replay/projection stability.
- Path under test and behavior witness: `controller.rs`, `view_models.rs`,
  TUI `app.rs`, `input.rs`, `render.rs`, `run.rs`, `transcript.rs`,
  `possession_parity_001`, `possession_does_not_reset_intention_001`,
  `view_filtering_001`, and possession/debug transcript tests.
- Replay/provenance ancestry: possessed and autonomous paths preserve
  source-backed holder-known facts, intention/need state, event sequence,
  projection checksums, and transcript fingerprints.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-08
  possession/embodied positive commands.
- Certification use: counted as certifying pass for FIRST-PROOF-08.

### E-0044-008-debug-split

- Requirement IDs: `FIRST-PROOF-08`
- Evidence status: pass
- Fingerprint scope: command transcript; debug/view output; replay artifact
- Evidence summary: `hidden_truth_gates`, `adversarial_gates`,
  `tui_acceptance`, and `transcript_snapshot` passed, covering debug
  non-diegetic rendering, no debug-to-actor-known leakage, stale/forged TUI
  rejection, possession rebind non-transfer, and transcript determinism.
- Path under test and behavior witness: `debug_capability.rs`,
  `debug_reports.rs`, TUI `debug_panels.rs`, adversarial TUI tests,
  `debug_attach_001`, `debug_omniscience_excluded_001`, and
  `embodied_menu_lags_truth_change_without_perception_001`.
- Replay/provenance ancestry: debug attach/detach and render paths produce no
  authoritative event mutation, context-hash mutation, hidden-truth affordance,
  or replay checksum change.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-08
  debug/adversarial commands.
- Certification use: counted as certifying pass for FIRST-PROOF-08.

### E-0044-009-no-human-progress

- Requirement IDs: `FIRST-PROOF-09`
- Evidence status: pass
- Fingerprint scope: command transcript; replay artifact; parsed semantic content
- Evidence summary: `no_human_capstone`, `acceptance_gates`,
  `event_schema_replay_gates`, `golden_fixtures_run`, and `tui_acceptance`
  passed, covering no-controller ordinary-life progress, shared action
  pipeline validation, typed need/routine/wait/blocker outcomes, no-human
  metrics, projection checksums, and replay equality.
- Path under test and behavior witness: `scheduler.rs`, `time.rs`,
  `need_accounting.rs`, `agent/no_human_surface.rs`, `agent/need.rs`,
  `agent/routine.rs`, `agent/intention.rs`, `agent/planner.rs`,
  `agent/trace.rs`, `actions/pipeline.rs`, ordinary action defs,
  `no_human_day_001`, `no_human_advance_001`, `food_unavailable_replan_001`,
  `routine_blocked_diagnostic_001`, and `routine_no_teleport_001`.
- Replay/provenance ancestry: ordinary no-human actions append typed events,
  need/routine/intention records, diagnostics, and metrics that replay to
  matching checksums and trace projections.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-09
  no-human/progress commands.
- Certification use: counted as certifying pass for FIRST-PROOF-09.

### E-0044-009-no-direct-dispatch

- Requirement IDs: `FIRST-PROOF-09`
- Evidence status: pass
- Fingerprint scope: command transcript; parsed semantic content; replay artifact
- Evidence summary: `acceptance_gates`, `spine_conformance`,
  `event_schema_replay_gates`, `golden_fixtures_run`, and `tui_acceptance`
  passed, covering no scheduler primitive dispatch, no marker-only progress,
  shared validation paths, typed blocked/failure diagnostics, no teleport,
  no silent starvation/spin, and debug read-only behavior.
- Path under test and behavior witness: scheduler and action-pipeline guards,
  conformance evidence matrix, event replay gates, continue-routine tamper
  fixtures, no-teleport and routine-blocked fixtures, no-human TUI acceptance,
  and replay checksum/diagnostic surfaces.
- Replay/provenance ancestry: invalid or marker-only progress paths produce no
  accepted primitive state delta; tampered or malformed no-human events reject
  live and replay before state change.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-09
  no-direct-dispatch and negative commands.
- Certification use: counted as certifying pass for FIRST-PROOF-09.

### E-0044-010-composite-replay

- Requirement IDs: `FIRST-PROOF-10`
- Evidence status: pass
- Fingerprint scope: command transcript; replay artifact; parsed semantic content
- Evidence summary: `event_schema_replay_gates`, `golden_scenarios`,
  `no_human_capstone`, `golden_fixtures_run`, and `transcript_snapshot` passed,
  covering scenario-by-scenario and integrated-capstone replay across physical,
  epistemic, agent/routine/accounting, diagnostic, and view-support surfaces.
- Path under test and behavior witness: `events/envelope.rs`, `events/log.rs`,
  `events/apply.rs`, `events/mutation.rs`, `replay/rebuild.rs`,
  `replay/report.rs`, `checksum.rs`, `projections.rs`, `need_accounting.rs`,
  `debug_reports.rs`, `epistemics/projection.rs`, golden scenarios,
  no-human capstone, golden fixtures, and transcript snapshots.
- Replay/provenance ancestry: fixture/content inputs, ordered event logs,
  event causes, schema versions, projection checksums, diagnostic fingerprints,
  serialized logs, replay reports, and transcript snapshots are deterministic
  for identical inputs.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-10
  positive replay commands.
- Certification use: counted as certifying pass for FIRST-PROOF-10.

### E-0044-010-controlled-divergence

- Requirement IDs: `FIRST-PROOF-10`
- Evidence status: pass
- Fingerprint scope: command transcript; replay artifact; parsed semantic content
- Evidence summary: `event_schema_replay_gates`, `generative_lock`,
  `golden_scenarios`, and `golden_fixtures_run` passed, covering unsupported
  schema rejection, malformed/duplicate/missing event rejection, stream
  mismatch, payload lies, source-evidence tamper, omitted/reordered event
  divergence, and generated metamorphic locks.
- Path under test and behavior witness: replay parser/apply gates, event schema
  guards, checksum/report comparators, golden scenario replay divergence tests,
  no-human tamper tests, continuation/episode/source-evidence tamper fixtures,
  and generated sequence replay locks.
- Replay/provenance ancestry: controlled perturbations reject before state
  change or produce localized first-divergence reports; equal final physical
  state with different event/provenance history is not treated as equal
  evidence.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-10
  negative/tamper commands.
- Certification use: counted as certifying pass for FIRST-PROOF-10.

### E-0044-011-load-schema-canonicalization

- Requirement IDs: `FIRST-PROOF-11`
- Evidence status: pass
- Fingerprint scope: raw bytes; canonical content; fixture registry; behavior
- Evidence summary: `fixtures_load`, `schema_conformance`, and
  `golden_fixtures_run` passed, covering canonical fixture registry loading,
  supported schema/scope declarations, deterministic fixture load and
  validation, source-backed epistemic seed canonicalization, fixture fingerprint
  repricing, and frozen golden fixture fingerprints.
- Path under test and behavior witness: content `schema.rs`, `validate.rs`,
  `load.rs`, `serialization.rs`, the fixture registry, `fixtures_load.rs`,
  `schema_conformance.rs`, `golden_fixtures_run.rs`, and first-proof fixture
  families.
- Replay/provenance ancestry: accepted fixture content produces deterministic
  canonical seeds, authored prehistory events, source refs, behavior
  fingerprints, and replayable fixture outputs.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-11
  positive load/schema/canonicalization commands.
- Certification use: counted as certifying pass for FIRST-PROOF-11.

### E-0044-011-semantic-rejection-compilefail

- Requirement IDs: `FIRST-PROOF-11`
- Evidence status: pass
- Fingerprint scope: command transcript; compile-fail boundary; parsed semantic content
- Evidence summary: `forbidden_content`, `fixtures_load`,
  `negative_fixture_runner`, `anti_regression_guards`, and
  `golden_fixtures_run` passed, covering semantic rejection of forbidden
  authority data, unsupported/unknown schema and field rejection, prose-born and
  hidden-truth-source rejection, quest/script/outcome-chain rejection, and
  external compile-fail boundaries for protected records.
- Path under test and behavior witness: content validation/serialization
  guards, `prose_born_fact_rejected_001`,
  `forbidden_provenance_input_fails_closed_001`,
  `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`,
  compile-fail fixtures for protected belief/observation/contradiction/debug
  records, and anti-regression source guards.
- Replay/provenance ancestry: rejected fixtures and forged public-API paths
  produce no accepted event, log, projection, actor-known fact, or partial
  runtime state.
- Sampling/exhaustiveness scope: exhaustive over the named FIRST-PROOF-11
  negative and compile-fail commands.
- Certification use: counted as certifying pass for FIRST-PROOF-11.

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
