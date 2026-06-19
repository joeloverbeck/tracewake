# 0040 EPI-CERT holder-known contexts, beliefs, observations, provenance, possession parity, view models, and debug-quarantine certification acceptance artifact

Spec: `specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`
Spec number: `0040`
Target/source baseline: `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`
Implementation commit tested for scaffold baseline: `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`
Worktree at baseline command start: clean
Spec posture consumed: P0-CERT passed from `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`; SPINE-CERT passed from `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` at exact implementation commit `92ba47f14998e0ea2fc95502bc3b76c5909478ca`.
Gate label under certification: `EPI-CERT`
Verdict: `<pending>`

This artifact renders no EPI-CERT verdict until the capstone ticket fills the per-seam verdict table and aggregate verdict. It is an implementation-session acceptance artifact for spec `0040`; it does not independently verify current `main`, does not certify latest main, and does not advance ORD-LIFE-CERT, FIRST-PROOF-CERT, Phase-4 entry, second-proof entry, institutions, notices, travel, LOD, or LLM/speech surfaces.

## Scope And Baseline Delta

- Artifact staging path: `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`
- Artifact closeout path: `archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`
- Evidence/test/configuration deltas from target baseline recorded by this ticket: new acceptance artifact only.
- Changed files for ticket `0040EPICERHOL-001`: this file and the archived ticket closeout document.
- Source perimeter/fixture/test inventory note: 176 files were enumerated across `crates/tracewake-content/src/fixtures`, `tests/negative-fixtures`, and the three crate test directories.

## Environment And Static Fingerprints

| Item | Value | Fingerprint scope |
|---|---|---|
| Rust | `rustc 1.93.0 (254b59607 2026-01-19)`; host `x86_64-unknown-linux-gnu`; LLVM `21.1.8` | command transcript |
| Cargo | `cargo 1.93.0 (083ac5135 2025-12-15)` | command transcript |
| cargo-mutants | `cargo-mutants 27.1.0` | command transcript |
| OS/runner | `Linux JOELOVERBECK 6.6.114.1-microsoft-standard-WSL2 #1 SMP PREEMPT_DYNAMIC Mon Dec 1 20:46:23 UTC 2025 x86_64` | command transcript |
| `Cargo.lock` | `f207f6208d64c1a7cce0daa27c00add40a52edb99b1a6ce3a1301cb44ef60a59` | raw bytes |
| `.cargo/mutants.toml` | `0726fc1470a5b2d9d5625394bf091105103f662c0ac5219b09cfb618af44c5d6` | raw bytes |
| `.github/workflows/ci.yml` | `5f20abf363f08b7369b6975ef25ee9398ee7c397fb32214cbfb03f1f310bbcc2` | raw bytes |

## Evidence Item Ledger Legend

Every evidence item cited by this artifact must instantiate the fields below. The allowed evidence statuses are exactly: `pass`, `fail`, `pending`, `sampled`, `observer-only`, and `historical`.

- Evidence item ID: stable local identifier used by the report.
- EPI cross-references: one or more of `EPI-01` through `EPI-11`, mutation, or artifact completeness.
- Evidence status: one allowed status.
- Fingerprint scope: raw bytes, normalized serialization, parsed semantic content, command transcript, run seed, replay artifact, or justified not applicable.
- Evidence summary: command, artifact path, report section, file/symbol, fixture, paired run, or replay reference.
- Path under test and behavior witness: production entry, event/trigger/emitter, responsible layer, accepted/rejected stage, negative or mutation-style control, and checked semantic consequence.
- Replay/provenance ancestry: event-log segment/IDs, replay artifact, seed/randomness/content/ruleset versions, projection/schema version, context ID/hash/frontier, source provenance, and downstream consumer.
- Sampling/exhaustiveness: exhaustive finite perimeter, exhaustive fixture registry, mutation census, finite named matrix, or sampled population with basis and omissions.
- Pending or historical handling: missing proof/owner/blocker for pending rows; context/lineage/precedent role for historical rows.
- Certification use: counted as certifying pass, counted as certifying fail, or not counted as certifying evidence.
- Staged-abstraction declaration: what is proven, abstracted, forbidden to fake, future route, anti-overclaim evidence, and diagnostic split where an abstraction is relied on.

Pending, sampled, observer-only, and historical evidence may inform review but may not silently make a seam pass. A fingerprint may not be cited outside its stated scope.

## Command And Environment Ledger

| Command | Exact commit/worktree | Result | Transcript/artifact | Evidence status | Notes |
|---|---|---|---|---|---|
| `cargo fmt --all --check` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; no output | pass | none |
| `cargo clippy --workspace --all-targets -- -D warnings` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; `Finished dev profile` | pass | none |
| `cargo build --workspace --all-targets --locked` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; `Finished dev profile` | pass | none |
| `cargo test --workspace --locked` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; workspace suite completed green | pass | none |
| `cargo test --locked -p tracewake-core --doc` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 9 doc tests passed | pass | none |
| `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 6 passed | pass | none |
| `cargo test --locked -p tracewake-core --test acceptance_gates` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 12 passed | pass | none |
| `cargo test --locked -p tracewake-core --test anti_regression_guards` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 80 passed | pass | none |
| `cargo test --locked -p tracewake-core --test ci_workflow_guards` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 1 passed | pass | none |
| `cargo test --locked -p tracewake-core --test doc_invariant_references` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 4 passed | pass | none |
| `cargo test --locked -p tracewake-core --test emergence_ledger` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 1 passed | pass | none |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 30 passed | pass | none |
| `cargo test --locked -p tracewake-core --test generative_lock` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 2 passed | pass | none |
| `cargo test --locked -p tracewake-core --test golden_scenarios` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 16 passed | pass | none |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 16 passed | pass | none |
| `cargo test --locked -p tracewake-core --test negative_fixture_runner` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 5 passed | pass | none |
| `cargo test --locked -p tracewake-core --test no_human_capstone` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 1 passed | pass | none |
| `cargo test --locked -p tracewake-core --test spine_conformance` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 6 passed | pass | none |
| `cargo test --locked -p tracewake-content --test fixtures_load` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 34 passed | pass | none |
| `cargo test --locked -p tracewake-content --test forbidden_content` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 24 passed | pass | none |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 42 passed | pass | none |
| `cargo test --locked -p tracewake-content --test schema_conformance` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 3 passed | pass | none |
| `cargo test --locked -p tracewake-tui --test adversarial_gates` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 15 passed | pass | none |
| `cargo test --locked -p tracewake-tui --test command_loop_session` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 7 passed | pass | none |
| `cargo test --locked -p tracewake-tui --test embodied_flow` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 6 passed | pass | none |
| `cargo test --locked -p tracewake-tui --test readme_sample_session` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 1 passed | pass | none |
| `cargo test --locked -p tracewake-tui --test transcript_snapshot` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 3 passed | pass | none |
| `cargo test --locked -p tracewake-tui --test tui_acceptance` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 11 passed | pass | none |
| `cargo test --locked -p tracewake-tui --test tui_seam_conformance` | `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`; clean | exit 0 | this command ledger; 2 passed | pass | none |

## Artifact-Completeness Evidence Items

- Evidence item ID: `0040-BASELINE-001`
- EPI cross-references: artifact completeness
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: section 4.1 clean baseline commands all exited 0 at implementation commit `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`.
- Path under test and behavior witness: workspace formatting, lint, build, test, and core doctest entry points; responsible layer `workspace/ci`; accepted stage was successful completion of each required command; negative control not applicable to baseline scaffold.
- Replay/provenance ancestry: event/replay-specific ancestry is owned by later EPI rows; this row only establishes the baseline command transcript.
- Sampling/exhaustiveness: exhaustive over spec 0040 section 4.1 commands.
- Pending or historical handling: none.
- Certification use: not counted as an EPI seam pass; counted as artifact-completeness baseline pass.

- Evidence item ID: `0040-NAMED-BINS-001`
- EPI cross-references: artifact completeness
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: every spec 0040 section 4.2 named test binary was run with the exact `cargo test --locked -p <crate> --test <binary>` invocation and exited 0.
- Path under test and behavior witness: named core, content, and TUI test binaries; responsible layer `workspace/tests`; accepted stage was successful completion of each named binary; per-EPI behavioral interpretation is deferred to the owning EPI ticket sections.
- Replay/provenance ancestry: specific event/replay/provenance ancestry is owned by later EPI rows.
- Sampling/exhaustiveness: exhaustive over spec 0040 section 4.2 named binary list.
- Pending or historical handling: none.
- Certification use: not counted as an EPI seam pass; counted as artifact-completeness named-binary pass.

## EPI-01 - Sealed Holder-Known Context Construction, Scope, Identity, Hash, And Frontier

Status: pending. Owned by `0040EPICERHOL-002`.

## EPI-02 - Typed Propositions, Beliefs, Stance/Confidence, Privacy, And Freshness

Status: pending. Owned by `0040EPICERHOL-003`.

## EPI-03 - Observation Channels, Capture Boundaries, And Event-Backed Insertion

Status: pending. Owned by `0040EPICERHOL-004`.

## EPI-04 - Expectation Contradiction, Mismatch Linkage, And Absence-Without-Culprit Discipline

Status: pending. Owned by `0040EPICERHOL-005`.

## EPI-05 - Provenance Witnesses, Source-Event Sufficiency, Freshness, And Hidden-Truth Audit

Status: pending. Owned by `0040EPICERHOL-006`.

## EPI-06 - Epistemic Projection Rebuild, Checksum Determinism, Context Filtering, And Non-Truth-Writer Quarantine

Status: pending. Owned by `0040EPICERHOL-007`.

## EPI-07 - Actor Decision Transaction, Proposal Context Parity, Validation-Truth Firewall, And Feedback Split

Status: pending. Owned by `0040EPICERHOL-008`.

## EPI-08 - Possession Parity And Cognition-Neutral Controller Binding

Status: pending. Owned by `0040EPICERHOL-009`.

## EPI-09 - Embodied Projection Source, Notebook, Action Availability, Why-Not, And Stale-Snapshot Behavior

Status: pending. Owned by `0040EPICERHOL-010`.

## EPI-10 - Debug Capability Isolation, Report/View Separation, TUI Quarantine, And No Feedback Path

Status: pending. Owned by `0040EPICERHOL-011`.

## EPI-11 - Relational Anti-Contamination And Possession-Parity Capstone

Status: pending. Owned by `0040EPICERHOL-013`.

## Section 6.1 Compile-Fail Boundary Corpus Matrix

Status: pending. Owned by `0040EPICERHOL-012`.

## Section 9.4 Per-Seam Verdict Table

| Requirement | Responsible layer(s) | Positive evidence | Adversarial/negative evidence | Replay/provenance evidence | Mutation evidence | Result from certifying evidence |
|---|---|---|---|---|---|---|
| `EPI-01` sealed context identity/scope/hash/frontier | actor-known context construction; proposal construction | pending | pending | pending | pending | pending |
| `EPI-02` beliefs/privacy/freshness | content/schema validation; projection/replay | pending | pending | pending | pending | pending |
| `EPI-03` observation channels/event capture | event application; projection/replay | pending | pending | pending | pending | pending |
| `EPI-04` contradiction/absence discipline | projection/replay; view-model rendering | pending | pending | pending | pending | pending |
| `EPI-05` provenance/witness sufficiency | actor-known context construction; proposal/action validation | pending | pending | pending | pending | pending |
| `EPI-06` projection rebuild/non-writer | event application; projection/replay | pending | pending | pending | pending | pending |
| `EPI-07` decision/proposal parity/truth firewall | candidate/planning/proposal/action validation | pending | pending | pending | pending | pending |
| `EPI-08` possession parity | actor-known context; view-model; proposal/action validation | pending | pending | pending | pending | pending |
| `EPI-09` embodied view/notebook/why-not | projection/replay; view-model rendering | pending | pending | pending | pending | pending |
| `EPI-10` debug quarantine | debug quarantine; view-model rendering | pending | pending | pending | pending | pending |
| `EPI-11` relational capstone | first responsible layer | pending | pending | pending | pending | pending |
| Configured EPI mutation perimeter | layer by survivor | pending | pending | pending | pending | pending |
| Artifact/evidence honesty | documentation status; tests/fixtures | `0040-BASELINE-001`, `0040-NAMED-BINS-001` | pending | pending | pending | pending |

## Section 9.5 Replay And Provenance Package

Status: pending. Owned by `0040EPICERHOL-015` after the EPI sections, compile-fail matrix, relational package, and mutation package are populated.

## Section 9.6 Mutation Package

Status: pending. Owned by `0040EPICERHOL-014`.

## Section 9.7 Relational Capstone Package

Status: pending. Owned by `0040EPICERHOL-013`; consolidated by `0040EPICERHOL-015`.

## Section 9.8 EMERGE-OBS Handling

Status: pending. Owned by `0040EPICERHOL-015`.

## Section 9.9 Aggregate Verdict

Verdict: `<pending>`

The aggregate verdict remains pending because `EPI-01` through `EPI-11`, the compile-fail matrix, the mutation package, the relational capstone package, and the final replay/provenance package have not yet been populated.
