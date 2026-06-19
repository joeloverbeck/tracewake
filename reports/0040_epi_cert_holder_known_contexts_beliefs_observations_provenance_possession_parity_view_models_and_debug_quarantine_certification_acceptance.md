# 0040 EPI-CERT holder-known contexts, beliefs, observations, provenance, possession parity, view models, and debug-quarantine certification acceptance artifact

Spec: `specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`
Spec number: `0040`
Target/source baseline: `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`
Implementation commit tested for scaffold baseline: `f9858fca35e6136bd825f59de95e6d8ff87f3a8d`
Worktree at baseline command start: clean
Spec posture consumed: P0-CERT passed from `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`; SPINE-CERT passed from `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` at exact implementation commit `92ba47f14998e0ea2fc95502bc3b76c5909478ca`.
Gate label under certification: `EPI-CERT`
Verdict: `EPI-CERT scoped remediation`

This artifact renders `EPI-CERT scoped remediation` at the capstone ticket because the configured EPI mutation perimeter completed with 30 untriaged missed mutants. It is an implementation-session acceptance artifact for spec `0040`; it does not independently verify current `main`, does not certify latest main, and does not advance ORD-LIFE-CERT, FIRST-PROOF-CERT, Phase-4 entry, second-proof entry, institutions, notices, travel, LOD, or LLM/speech surfaces.

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

Status: evidence collected by `0040EPICERHOL-002`; aggregate row remains pending for the capstone and mutation package.

- Evidence item ID: `EPI01-POS-001`
- EPI cross-references: `EPI-01`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates` exited 0 at commit `d63019bfb8d801cc38f1da6398f975563d6ed5b4`; 16 tests passed, including `actor_known_context_unforgeable_from_truth`, `actor_known_local_actor_reaches_embodied_view_model_with_context_provenance`, `holder_known_fact_family_report_fingerprint_covers_keys_fields_and_audit`, and `epistemic_context_projection_and_records_remain_sealed`.
- Path under test and behavior witness: sealed holder-known context construction, holder-known fact family fingerprinting, embodied view-model context provenance, and sealed epistemic records; responsible layer `actor-known context construction`; witnesses exercised accepted context construction and actor-visible reads from the sealed packet.
- Replay/provenance ancestry: source-event and projection ancestry are covered by the named hidden-truth tests plus `EPI01-REPLAY-001`; this row records the positive construction and packet/fingerprint evidence.
- Sampling/exhaustiveness: finite named EPI-01 test perimeter.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-01 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI01-ADV-001`
- EPI cross-references: `EPI-01`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates` exited 0 at commit `d63019bfb8d801cc38f1da6398f975563d6ed5b4`; relevant passing tests include `context_rejects_hidden_counterpart_injection`, `debug_truth_never_enters_holder_known_context_hash`, `hidden_food_closed_container_is_not_actor_known_food_source`, `hidden_food_unknown_route_does_not_become_transaction_target`, `hidden_route_edge_absent_from_actor_context_blocks_route_plan`, `planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context`, and `workplace_requires_assignment_or_observation_provenance`.
- Path under test and behavior witness: hidden-truth rejection and no-leak context filtering for unobserved food, route, workplace, debug, and forged counterpart inputs; responsible layers `actor-known context construction` and `proposal construction`; negative controls prove hidden or validator-only facts do not become actor-known context/proposal inputs.
- Replay/provenance ancestry: source-event ancestry is checked by `epistemic_event_fields_survive_into_sealed_context_and_replay`; replay equality is also recorded in `EPI01-REPLAY-001`.
- Sampling/exhaustiveness: finite named adversarial EPI-01 perimeter from spec section 5.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-01 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI01-COMPILE-001`
- EPI cross-references: `EPI-01`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test negative_fixture_runner` exited 0 at commit `d63019bfb8d801cc38f1da6398f975563d6ed5b4`; 5 tests passed, including the registered corpus check covering `external_crate_cannot_build_debug_knowledge_context`, `external_crate_cannot_mutate_knowledge_context_mode`, and `external_crate_cannot_mutate_knowledge_context_viewer`.
- Path under test and behavior witness: external-crate compile-fail boundary for debug context construction and sealed context mode/viewer identity mutation; responsible layer `debug quarantine` / `actor-known context construction`; negative fixture compilation remains rejected by Rust visibility/lint boundaries rather than by runtime string matching.
- Replay/provenance ancestry: not applicable to compile-fail evidence.
- Sampling/exhaustiveness: registered negative-fixture corpus member set for EPI-01 context boundaries.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-01 compile-fail negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI01-REPLAY-001`
- EPI cross-references: `EPI-01`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates`, `cargo test --locked -p tracewake-core --test golden_scenarios`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` exited 0 at commit `d63019bfb8d801cc38f1da6398f975563d6ed5b4`; 30, 16, and 42 tests passed respectively. Relevant witnesses include `deterministic_rebuild_context_hash_uses_causal_and_latest_witnesses`, `epistemic_apply_matrix_preserves_fields_and_rejects_unknown_tokens`, `projection_rebuild_matches_live_state`, `no_human_epistemic_check_records_evidence_without_controller`, `no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`, and `no_human_decision_context_hash_gate_fails_when_source_evidence_tampered`.
- Path under test and behavior witness: event-backed epistemic application, deterministic context-hash rebuild, projection rebuild equality, and source-event tamper rejection; responsible layer `projection/replay`; accepted event streams replay to the same context/projection evidence, while source-event tampering is rejected.
- Replay/provenance ancestry: event-envelope and source-event witness ancestry is exercised by `event_schema_replay_gates`; content fixture source and replay ancestry are exercised by `golden_fixtures_run`.
- Sampling/exhaustiveness: finite named replay/golden command set required by ticket `0040EPICERHOL-002`.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-01 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI01-PROPOSAL-001`
- EPI cross-references: `EPI-01`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test acceptance_gates` exited 0 at commit `d63019bfb8d801cc38f1da6398f975563d6ed5b4`; 12 tests passed, including `human_and_nonhuman_proposals_share_validation_path`, `sleep_proposals_share_pipeline_across_human_and_nonhuman_origins`, and `event_append_order_is_deterministic`.
- Path under test and behavior witness: proposal source context parity and append-before-apply command path; responsible layers `proposal construction` and `action validation`; the command validates that accepted proposals travel through the shared pipeline and deterministic event append order.
- Replay/provenance ancestry: shared-pipeline event ancestry is covered by `event_append_order_is_deterministic`; deeper context-hash replay is recorded in `EPI01-REPLAY-001`.
- Sampling/exhaustiveness: finite named acceptance-gate command for EPI-01 proposal/path parity.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as supporting certifying evidence for EPI-01 proposal construction; aggregate seam verdict remains pending until mutation/capstone consolidation.

## EPI-02 - Typed Propositions, Beliefs, Stance/Confidence, Privacy, And Freshness

Status: evidence collected by `0040EPICERHOL-003`; aggregate row remains pending for the capstone and mutation package.

- Evidence item ID: `EPI02-POS-001`
- EPI cross-references: `EPI-02`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run` exited 0 at commit `9acec3385b4f4eb925a1c58a37838ce124f108b6`; 42 tests passed, including `aged_food_record_surfaces_as_remembered_belief`, `phase2a_initial_beliefs_are_holder_and_source_backed`, `partial_food_source_knowledge_seeds_only_authored_actor_edge`, `seeded_food_source_unknown_to_all_actors_omits_seed_belief_and_actor_known_food`, and `no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`.
- Path under test and behavior witness: typed belief/proposition serialization, holder/source-backed initial belief projection, partial/absent knowledge preservation, and actor-known input hash recomputation; responsible layers `content/schema validation` and `projection/replay`.
- Replay/provenance ancestry: content fixtures carry source files and event-backed actor-known inputs; replay-specific record equality is recorded in `EPI02-REPLAY-001`.
- Sampling/exhaustiveness: finite named golden fixture runner required by ticket `0040EPICERHOL-003`.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-02 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI02-FRESH-001`
- EPI cross-references: `EPI-02`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-tui --test embodied_flow` exited 0 at commit `9acec3385b4f4eb925a1c58a37838ce124f108b6`; relevant passing witnesses include `stale_workplace_notice_superseded_by_newer_001` through the content suite and TUI tests `embodied_workplace_availability_reflects_belief_not_truth` and `embodied_workplace_believed_open_truth_closed`.
- Path under test and behavior witness: stale/superseded workplace belief handling, wrong-belief actor-visible availability, and authoritative validation rejection when truth diverges; responsible layers `projection/replay` and `view-model rendering`.
- Replay/provenance ancestry: source tick and stable event identity are exercised by the fixture; TUI embodied-flow witnesses record actor-visible behavior separately from authoritative validation truth.
- Sampling/exhaustiveness: finite named EPI-02 freshness and record-vs-truth fixture set.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for EPI-02 positive/adversarial freshness evidence; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI02-ADV-001`
- EPI cross-references: `EPI-02`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates` exited 0 at commit `9acec3385b4f4eb925a1c58a37838ce124f108b6`; 16 tests passed, including `epistemic_confidence_paths_do_not_use_raw_floats_or_hash_ordering`, `debug_omniscience_facts_are_excluded_from_planner_context`, `context_rejects_hidden_counterpart_injection`, and hidden-food/route/workplace no-leak tests.
- Path under test and behavior witness: hidden-truth changes and debug facts do not create, strengthen, refresh, or leak actor beliefs into holder-known context or planner inputs; responsible layers `actor-known context construction` and `projection/replay`.
- Replay/provenance ancestry: event-backed epistemic fields and replay survival are covered by hidden-truth tests and `EPI02-REPLAY-001`.
- Sampling/exhaustiveness: finite named hidden-truth EPI-02 adversarial perimeter.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-02 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI02-COMPILE-001`
- EPI cross-references: `EPI-02`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test negative_fixture_runner` exited 0 at commit `9acec3385b4f4eb925a1c58a37838ce124f108b6`; 5 tests passed, including registered compile-fail coverage for `banned_float_confidence_types`, `external_crate_cannot_construct_belief_literal`, `external_crate_cannot_mutate_belief_source_or_scope`, and `external_crate_cannot_read_raw_epistemic_projection_maps`.
- Path under test and behavior witness: external crates cannot introduce float confidence types, construct raw belief literals, mutate belief source/scope, or read raw epistemic projection maps; responsible layer `content/schema validation` / `actor-known context construction`.
- Replay/provenance ancestry: not applicable to compile-fail evidence.
- Sampling/exhaustiveness: registered negative-fixture corpus member set for EPI-02 belief boundaries.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-02 compile-fail negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI02-REPLAY-001`
- EPI cross-references: `EPI-02`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates` exited 0 at commit `9acec3385b4f4eb925a1c58a37838ce124f108b6`; 30 tests passed, including `epistemic_apply_matrix_preserves_fields_and_rejects_unknown_tokens`, `starting_observation_and_contradiction_events_survive_replay_with_sources`, `unsupported_epistemic_payload_schema_replay_is_loud_and_not_applied`, and `deterministic_rebuild_context_hash_uses_causal_and_latest_witnesses`.
- Path under test and behavior witness: epistemic record replay, unknown-token/schema rejection, source-backed replay survival, and deterministic context-hash rebuild; responsible layer `projection/replay`.
- Replay/provenance ancestry: event envelope/source-event ancestry is exercised by `event_schema_replay_gates`; record-vs-truth view behavior is recorded in `EPI02-FRESH-001`.
- Sampling/exhaustiveness: finite named replay command required by ticket `0040EPICERHOL-003`.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-02 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

## EPI-03 - Observation Channels, Capture Boundaries, And Event-Backed Insertion

Status: evidence collected by `0040EPICERHOL-004`; aggregate row remains pending for the capstone and mutation package.

- Evidence item ID: `EPI03-POS-001`
- EPI cross-references: `EPI-03`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test golden_scenarios` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` exited 0 at commit `b231d1b9591dd00d7d14749ea19d5ed130da8c52`; 16 and 42 tests passed respectively. Relevant witnesses include `accepted_actions_append_versioned_events`, `check_container_records_observation_but_open_alone_does_not`, `sound_uncertainty_records_low_confidence_evidence_without_culprit_knowledge`, `expected_absence_check_creates_contradiction_and_missing_belief`, `no_human_epistemic_check_records_evidence_without_controller`, and `no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`.
- Path under test and behavior witness: modeled observation channels, source-backed observation facts, sound uncertainty, modeled absence, and append-before-projection behavior; responsible layers `action validation`, `event application`, and `projection/replay`.
- Replay/provenance ancestry: event append and source-event IDs are exercised by the scenario/fixture tests; replay equality is recorded in `EPI03-REPLAY-001`.
- Sampling/exhaustiveness: finite named observation-channel fixture/test set required by ticket `0040EPICERHOL-004`.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-03 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI03-ADV-001`
- EPI cross-references: `EPI-03`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates` exited 0 at commit `b231d1b9591dd00d7d14749ea19d5ed130da8c52`; 16 tests passed, including `embodied_affordances_exclude_hidden_food_in_closed_container`, `hidden_food_closed_container_is_not_actor_known_food_source`, `hidden_food_unknown_route_does_not_become_transaction_target`, `hidden_route_edge_absent_from_actor_context_blocks_route_plan`, `workplace_requires_assignment_or_observation_provenance`, and `planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context`.
- Path under test and behavior witness: channel-gated observation/no-telepathy controls for closed containers, hidden food, hidden routes, workplace knowledge, and empty adversarial context; responsible layers `actor-known context construction` and `projection/replay`.
- Replay/provenance ancestry: hidden-truth tests include event-backed epistemic fields and sealed-context replay witnesses; deeper replay coverage is recorded in `EPI03-REPLAY-001`.
- Sampling/exhaustiveness: finite named EPI-03 adversarial perimeter.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-03 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI03-COMPILE-001`
- EPI cross-references: `EPI-03`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test negative_fixture_runner` exited 0 at commit `b231d1b9591dd00d7d14749ea19d5ed130da8c52`; 5 tests passed, including registered compile-fail coverage for `external_crate_cannot_construct_observation_without_source` and `external_crate_cannot_insert_raw_epistemic_records`.
- Path under test and behavior witness: external crates cannot construct observations without a source or insert raw epistemic records; responsible layers `content/schema validation` and `event application`.
- Replay/provenance ancestry: not applicable to compile-fail evidence.
- Sampling/exhaustiveness: registered negative-fixture corpus member set for EPI-03 observation boundaries.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-03 compile-fail negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI03-REPLAY-001`
- EPI cross-references: `EPI-03`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates` exited 0 at commit `b231d1b9591dd00d7d14749ea19d5ed130da8c52`; 30 tests passed, including `epistemic_apply_matrix_preserves_fields_and_rejects_unknown_tokens`, `starting_observation_and_contradiction_events_survive_replay_with_sources`, `unsupported_epistemic_payload_schema_replay_is_loud_and_not_applied`, `unsupported_event_schema_append_rejected`, `replay_rebuild_checksum_matches_original_after_no_human_day`, and `replay_report_match_mismatch_pair_exposes_semantic_fingerprints`.
- Path under test and behavior witness: observation event application, append/schema rejection, source-bearing observation/contradiction replay, and replay mismatch reporting; responsible layer `projection/replay`.
- Replay/provenance ancestry: event-envelope/source-event ancestry and replay rebuild reports are exercised by `event_schema_replay_gates`; scenario/fixture channels are recorded in `EPI03-POS-001`.
- Sampling/exhaustiveness: finite named replay command required by ticket `0040EPICERHOL-004`.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-03 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI03-STAGED-READING-001`
- EPI cross-references: `EPI-03`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates` exited 0 at commit `b231d1b9591dd00d7d14749ea19d5ed130da8c52`; the apply/replay matrix includes `reading_placeholder_schema_only` as a typed schema value while this artifact does not count it as completed live reading behavior.
- Path under test and behavior witness: staged reading channel placeholder in epistemic schema/application; responsible layer `event application`; the witness proves schema/application handling only.
- Replay/provenance ancestry: source-event handling for the placeholder is exercised in the apply matrix; no live reading affordance/event path is certified by this row.
- Sampling/exhaustiveness: bounded staged abstraction declared for the current schema placeholder.
- Pending or historical handling: staged abstraction: proven now = schema/application value is typed and replayed; abstracted = complete live reading behavior; forbidden to fake = may not be used as evidence of completed reading; future route = separate scoped implementation that supplies modeled event/provenance path; anti-overclaim evidence = EPI-03 row remains tied to this staged declaration.
- Certification use: counted only for schema/application placeholder honesty; not counted as completed live reading evidence.

## EPI-04 - Expectation Contradiction, Mismatch Linkage, And Absence-Without-Culprit Discipline

Status: evidence collected by `0040EPICERHOL-005`; aggregate row remains pending for the capstone and mutation package.

- Evidence item ID: `EPI04-POS-001`
- EPI cross-references: `EPI-04`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run` exited 0 at commit `374fd926d83763b74cff6592dd3ec4812ec3f5ec`; 42 tests passed, including the registered `expectation_contradiction_001` fixture and Phase 2A fixture coverage.
- Path under test and behavior witness: expected-item absence contradiction fixture from prior belief through modeled search/action event to absence observation and contradiction linkage; responsible layers `tests/fixtures`, `event application`, and `projection/replay`.
- Replay/provenance ancestry: fixture source and event provenance are exercised by the golden runner; replay equality is recorded in `EPI04-REPLAY-001`.
- Sampling/exhaustiveness: finite single active contradiction-kind fixture plus runner registry coverage.
- Pending or historical handling: bounded staged abstraction: the implemented active contradiction kind is `ExpectedItemAbsentFromContainer`; this row does not claim general belief revision.
- Certification use: counted as certifying pass for the EPI-04 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI04-ADV-001`
- EPI cross-references: `EPI-04`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates` exited 0 at commit `374fd926d83763b74cff6592dd3ec4812ec3f5ec`; 16 tests passed, including hidden food/route/workplace no-leak tests, `planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context`, and `epistemic_context_projection_and_records_remain_sealed`.
- Path under test and behavior witness: absence/no-telepathy controls ensure hidden culprit, hidden destination, or unobserved cause are not inferred from an absence contradiction; responsible layers `actor-known context construction` and `projection/replay`.
- Replay/provenance ancestry: hidden-truth gate evidence is paired with replay/source-event evidence in `EPI04-REPLAY-001`.
- Sampling/exhaustiveness: finite named no-leak perimeter relevant to EPI-04 absence-without-culprit discipline.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-04 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI04-COMPILE-001`
- EPI cross-references: `EPI-04`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test negative_fixture_runner` exited 0 at commit `374fd926d83763b74cff6592dd3ec4812ec3f5ec`; 5 tests passed, including registered compile-fail coverage for `external_crate_cannot_mutate_contradiction_links`.
- Path under test and behavior witness: external crates cannot mutate contradiction links directly; responsible layer `projection/replay` / `epistemic record integrity`.
- Replay/provenance ancestry: not applicable to compile-fail evidence.
- Sampling/exhaustiveness: registered negative-fixture corpus member set for EPI-04 contradiction-link boundary.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-04 compile-fail negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI04-REPLAY-001`
- EPI cross-references: `EPI-04`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates` exited 0 at commit `374fd926d83763b74cff6592dd3ec4812ec3f5ec`; 30 tests passed, including `starting_observation_and_contradiction_events_survive_replay_with_sources`, `epistemic_apply_matrix_preserves_fields_and_rejects_unknown_tokens`, `replay_rebuild_checksum_matches_original_after_no_human_day`, and `replay_report_match_mismatch_pair_exposes_semantic_fingerprints`.
- Path under test and behavior witness: contradiction source records survive replay with linked observation/source events; replay rejects unsupported/unknown epistemic payloads loudly and reports mismatches; responsible layer `projection/replay`.
- Replay/provenance ancestry: source-event and linked-record ancestry are exercised by `event_schema_replay_gates`.
- Sampling/exhaustiveness: finite named replay command required by ticket `0040EPICERHOL-005`.
- Pending or historical handling: none for this evidence row.
- Certification use: counted as certifying pass for the EPI-04 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI04-STAGED-001`
- EPI cross-references: `EPI-04`
- Evidence status: pass
- Fingerprint scope: not applicable
- Evidence summary: The current EPI-04 evidence is explicitly bounded to `ExpectedItemAbsentFromContainer`; the artifact does not claim generalized contradiction or belief-revision coverage beyond that implemented kind.
- Path under test and behavior witness: staged abstraction declaration for the contradiction relation; responsible layer `doctrine mismatch` guard.
- Replay/provenance ancestry: positive/replay evidence for the implemented kind is in `EPI04-POS-001` and `EPI04-REPLAY-001`.
- Sampling/exhaustiveness: bounded staged abstraction over the single active contradiction kind named by spec 0040.
- Pending or historical handling: future route requires a separate scoped implementation/certification package for broader contradiction families; current diagnostics distinguish "not implemented yet" from "implemented but broken" by naming this bounded kind.
- Certification use: counted only for abstraction honesty; not counted as proof of general belief revision.

## EPI-05 - Provenance Witnesses, Source-Event Sufficiency, Freshness, And Hidden-Truth Audit

Status: evidence collected by `0040EPICERHOL-006`; aggregate row remains pending for the mutation package and capstone verdict.

- Evidence item ID: `EPI05-POS-001`
- EPI cross-references: `EPI-05`
- Evidence status: pass
- Fingerprint scope: `ActorKnownFact`/`ActorKnownProvenance` records, `HiddenTruthAudit`, proposal source-context tuple, accepted event ancestry, and actor-visible feedback.
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates`, `cargo test --locked -p tracewake-core --test acceptance_gates`, and `cargo test --locked -p tracewake-core --test no_human_capstone` passed. These gates prove actor-known workplace, route, food, and embodied context facts remain source-backed, holder-scoped, and frontier-bounded; proposal consumers retain the sealed context ID/hash/frontier and source-event ancestry through validation, append, replay-visible diagnostics, and actor-safe feedback.
- Path under test and behavior witness: `agent/actor_known.rs`, `agent/no_human_surface.rs`, `agent/transaction.rs`, `actions/proposal.rs`, `actions/pipeline.rs`, `agent/trace.rs`, and no-human capstone consumers. Representative witnesses include `workplace_requires_assignment_or_observation_provenance`, `actor_known_local_actor_reaches_embodied_view_model_with_context_provenance`, `holder_known_fact_family_report_fingerprint_covers_keys_fields_and_audit`, `integrated_no_human_day_capstone_emerges_from_one_autonomous_run`, and `no_human_capstone_proves_typed_ancestry_and_replay`.
- Replay/provenance ancestry: every certified positive row is derived from accepted log events or declared starting evidence; source-event IDs remain non-empty where required, canonical, resolvable, and no later than the sealed context frontier.
- Sampling/exhaustiveness: finite EPI-05 positive command set required by ticket `0040EPICERHOL-006`.
- Pending or historical handling: ordinary-life completeness is not inferred from this row; it certifies only the EPI-05 provenance boundary.
- Certification use: counted as certifying pass for the EPI-05 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI05-ADV-001`
- EPI cross-references: `EPI-05`
- Evidence status: pass
- Fingerprint scope: forbidden provenance, prose-born fact, hidden-truth-audit contamination, debug-only truth, validation-only truth, and unproven physical truth rejection surfaces.
- Evidence summary: `cargo test --locked -p tracewake-content --test forbidden_content` and `cargo test --locked -p tracewake-core --test negative_fixture_runner` passed, including the semantic-contamination guards that reject typed-but-forbidden provenance even without relying on banned strings.
- Path under test and behavior witness: `forbidden_content` covers `forbidden_content_hidden_truth_source_cannot_seed_actor_known_planner_input`, `forbidden_content_planner_intended_initial_facts_require_provenance`, and `content_prose_born_fact_rejected`; `negative_fixture_runner` confirms the registered negative corpus and compile-fail guards still fire.
- Replay/provenance ancestry: adversarial inputs do not create accepted proposals/events or actor-known facts; typed diagnostics keep the responsible layer explicit.
- Sampling/exhaustiveness: finite named forbidden-content and negative-fixture corpus required by ticket `0040EPICERHOL-006`.
- Pending or historical handling: no mutation-survivor conclusion is made here; mutation package remains owned by `0040EPICERHOL-014`.
- Certification use: counted as certifying pass for the EPI-05 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI05-REPLAY-001`
- EPI cross-references: `EPI-05`
- Evidence status: pass
- Fingerprint scope: event envelope source/cause IDs, replay rebuilding, source-event tamper/deletion, dangling-provenance diagnostics, and deterministic context hashes.
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. Replay gates prove epistemic event source fields survive accepted-log serialization/rebuild, deterministic context hashes use causal/latest witnesses, and source-event tampering or deletion poisons replay instead of preserving derived facts by checksum coincidence.
- Path under test and behavior witness: `event_schema_replay_gates` covers `starting_observation_and_contradiction_events_survive_replay_with_sources`, `deterministic_rebuild_context_hash_uses_causal_and_latest_witnesses`, and `typed_diagnostic_hidden_truth_true_is_validated_and_replayed`; `golden_fixtures_run` covers `no_human_decision_context_hash_gate_fails_when_source_evidence_tampered`, `no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`, and provenance-bearing fixture families.
- Replay/provenance ancestry: accepted log event IDs, payload schema, payload fingerprints, source ticks, context frontiers, freshness/supersession records, and downstream proposal consumers are exercised through replayed fixtures.
- Sampling/exhaustiveness: finite EPI-05 replay/golden-fixture command set required by ticket `0040EPICERHOL-006`.
- Pending or historical handling: replay evidence is current to this ticket's commands; full cross-seam replay package remains owned by the capstone.
- Certification use: counted as certifying pass for the EPI-05 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

## EPI-06 - Epistemic Projection Rebuild, Checksum Determinism, Context Filtering, And Non-Truth-Writer Quarantine

Status: evidence collected by `0040EPICERHOL-007`; aggregate row remains pending for the mutation package and capstone verdict.

- Evidence item ID: `EPI06-POS-001`
- EPI cross-references: `EPI-06`
- Evidence status: pass
- Fingerprint scope: `EpistemicProjection` version, content-manifest identity, applied event range/count, typed observation/belief/contradiction/notebook/actor-known records, context-filtered reads, freshness/supersession records, and canonical projection checksums.
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates`, `cargo test --locked -p tracewake-core --test spine_conformance`, `cargo test --locked -p tracewake-core --test golden_scenarios`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. These gates prove live projections and clean rebuilds agree on replay-visible semantic fields and checksums, and that serialized accepted logs replay to identical projection/state outcomes.
- Path under test and behavior witness: `epistemics/projection.rs`, `events/apply.rs`, `events/log.rs`, `replay/rebuild.rs`, `replay/report.rs`, `checksum.rs`, and fixture runners. Representative witnesses include `projection_rebuild_matches_live_state`, `phase3a_agent_state_replay_projection_is_deterministic`, `serialized_event_log_replays_to_identical_state`, `replay_rebuild_checksum_matches_original_after_no_human_day`, and `spine_conformance_maps_every_spine_requirement_to_named_evidence`.
- Replay/provenance ancestry: accepted event prefixes, event envelope identity, source/cause fields, projection checksums, replay reports, and fixture fingerprints are reconstructed from the serialized log rather than direct projection writes.
- Sampling/exhaustiveness: finite EPI-06 replay/projection command set required by ticket `0040EPICERHOL-007`.
- Pending or historical handling: cross-seam replay package remains owned by the capstone; mutation proof for projection bookkeeping remains owned by `0040EPICERHOL-014`.
- Certification use: counted as certifying pass for the EPI-06 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI06-ADV-001`
- EPI cross-references: `EPI-06`
- Evidence status: pass
- Fingerprint scope: hidden-state no-leak boundary, context-filtered actor-known reads, debug-truth exclusion, raw projection insertion/read quarantine, and omitted/reordered/corrupt event replay controls.
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates`, `cargo test --locked -p tracewake-core --test golden_scenarios`, `cargo test --locked -p tracewake-content --test golden_fixtures_run`, and `cargo test --locked -p tracewake-core --test negative_fixture_runner` passed. These gates show hidden authoritative state does not rewrite focal actor projection/context without an admissible epistemic event, and raw insertion/read negative fixtures remain enforced.
- Path under test and behavior witness: `actor_known_context_unforgeable_from_truth`, `debug_truth_never_enters_holder_known_context_hash`, `epistemic_context_projection_and_records_remain_sealed`, `replay_detects_missing_or_reordered_event`, `no_human_decision_context_hash_gate_fails_when_source_evidence_tampered`, and registered negative fixtures for external raw projection access.
- Replay/provenance ancestry: malformed or tampered event streams are rejected or reported with divergence rather than silently preserving the original projection checksum; hidden/debug-only facts remain outside holder-known projections.
- Sampling/exhaustiveness: finite named no-leak, replay-tamper, and compile-fail corpus required by ticket `0040EPICERHOL-007`.
- Pending or historical handling: configured mutation campaign is not claimed here; this row only records existing adversarial/negative gates.
- Certification use: counted as certifying pass for the EPI-06 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI06-REPLAY-001`
- EPI cross-references: `EPI-06`
- Evidence status: pass
- Fingerprint scope: replay report match/mismatch pairs, first divergence, checksum sensitivity, event order/duplication/schema rejection, and canonical serialization round trips.
- Evidence summary: `event_schema_replay_gates`, `golden_scenarios`, and `golden_fixtures_run` passed replay controls that remove, duplicate, reorder, corrupt, schema-change, or source-tamper relevant events. The controls either reject the input or produce replay divergence instead of retaining projection equality by coincidence.
- Path under test and behavior witness: `replay_report_match_mismatch_pair_exposes_semantic_fingerprints`, `unsupported_event_schema_replay_rejected`, `stream_mismatch_replay_rejected`, `duplicate_duration_terminal_poisons_rebuild_001`, `replay_detects_missing_or_reordered_event`, and fixture tamper tests.
- Replay/provenance ancestry: replay inputs and outputs remain bound to event envelope identity, payload schema, source/cause fields, and canonical projection/state checksum scope.
- Sampling/exhaustiveness: finite EPI-06 replay-tamper command set required by ticket `0040EPICERHOL-007`.
- Pending or historical handling: none for this evidence row beyond the separate mutation/capstone gates.
- Certification use: counted as certifying pass for the EPI-06 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

## EPI-07 - Actor Decision Transaction, Proposal Context Parity, Validation-Truth Firewall, And Feedback Split

Status: evidence collected by `0040EPICERHOL-008`; aggregate row remains pending for the mutation package and capstone verdict.

- Evidence item ID: `EPI07-POS-001`
- EPI cross-references: `EPI-07`
- Evidence status: pass
- Fingerprint scope: sealed actor-known input, candidate/decision trace boundary, selected proposal, `ProposalSourceContext` ID/hash/frontier tuple, validation stage, appended event, actor-visible outcome, and actor-safe/debug feedback split.
- Evidence summary: `cargo test --locked -p tracewake-core --test acceptance_gates`, `cargo test --locked -p tracewake-core --test no_human_capstone`, `cargo test --locked -p tracewake-core --test golden_scenarios`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. These gates prove human/nonhuman proposals share validation, no-human decision traces retain typed ancestry, accepted actions append versioned replay-visible events, and fixture decisions remain bound to actor-known context rather than validation/debug truth.
- Path under test and behavior witness: `agent/transaction.rs`, `agent/no_human_surface.rs`, `agent/candidate.rs`, `agent/decision.rs`, `agent/planner.rs`, `actions/proposal.rs`, `actions/pipeline.rs`, `actions/report.rs`, and `events/envelope.rs`. Representative witnesses include `human_and_nonhuman_proposals_share_validation_path`, `phase3a_agent_events_apply_live_and_replay_to_same_agent_checksum`, `no_human_capstone_proves_typed_ancestry_and_replay`, `accepted_actions_append_versioned_events`, and `no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`.
- Replay/provenance ancestry: the proposal path retains context identity, hash/frontier, source-event ancestry, semantic action, validation result, event append, and actor-safe feedback through accepted log replay.
- Sampling/exhaustiveness: finite EPI-07 positive command set required by ticket `0040EPICERHOL-008`.
- Pending or historical handling: planner quality and ordinary-life completeness remain outside this EPI row and are not inferred.
- Certification use: counted as certifying pass for the EPI-07 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI07-ADV-001`
- EPI cross-references: `EPI-07`
- Evidence status: pass
- Fingerprint scope: paired hidden-food/route/workplace/debug-state worlds, forbidden provenance contamination, typed-wrapped hidden input, stale view token, forged privileged action shape, and actor-safe why-not feedback.
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates`, `cargo test --locked -p tracewake-content --test golden_fixtures_run`, and `cargo test --locked -p tracewake-tui --test adversarial_gates` passed. These gates show hidden truth does not enter candidate/proposal inputs or actor-safe reasons before a legal observation channel, and typed-wrapped contamination/stale or forged TUI submissions are rejected.
- Path under test and behavior witness: `actor_known_context_unforgeable_from_truth`, `hidden_route_edge_absent_from_actor_context_blocks_route_plan`, `hidden_food_unknown_route_does_not_become_transaction_target`, `debug_truth_never_enters_holder_known_context_hash`, `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit`, `adversarial_gates_tui_rule_inference_cannot_apply_hidden_food_target`, `adversarial_gates_stale_view_token_fails_after_state_change`, and `adversarial_gates_why_not_actor_surface_uses_typed_non_leaking_facts`.
- Replay/provenance ancestry: rejected or blocked paths produce typed actor-safe blockers and separately authorized debug diagnostics without appending accepted events or backfilling hidden validation facts into the next holder-known context.
- Sampling/exhaustiveness: finite named paired/no-leak/TUI adversarial corpus required by ticket `0040EPICERHOL-008`.
- Pending or historical handling: relational anti-contamination breadth remains owned by `0040EPICERHOL-013`.
- Certification use: counted as certifying pass for the EPI-07 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI07-REPLAY-001`
- EPI cross-references: `EPI-07`
- Evidence status: pass
- Fingerprint scope: accepted/rejected event prefixes, context parity tamper, payload schema/version, proposal eligibility, validation result, replay report, and actor-safe feedback determinism.
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates`, `cargo test --locked -p tracewake-core --test golden_scenarios`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. Replay gates reproduce proposal/action outcomes from accepted event prefixes and reject forged schema, stream, stale context, or tampered source evidence before accepted mutation.
- Path under test and behavior witness: `agent_apply_matrix_observes_parser_arms_transitions_and_causality`, `forged_trace_and_diagnostic_schema_versions_are_rejected_for_materialized_agent_replay_001`, `replay_report_match_mismatch_pair_exposes_semantic_fingerprints`, `replay_checksum_matches`, `debug_rejection_report_names_failed_stage`, and `no_human_decision_context_hash_gate_fails_when_source_evidence_tampered`.
- Replay/provenance ancestry: replayed prefixes preserve the same source context, proposal eligibility, validation result, appended-event semantics, and actor-safe feedback; tampered parity/source fields fail before accepted state mutation.
- Sampling/exhaustiveness: finite EPI-07 replay/proposal command set required by ticket `0040EPICERHOL-008`.
- Pending or historical handling: none for this evidence row beyond mutation/capstone consolidation.
- Certification use: counted as certifying pass for the EPI-07 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

## EPI-08 - Possession Parity And Cognition-Neutral Controller Binding

Status: evidence collected by `0040EPICERHOL-009`; aggregate row remains pending for the mutation package and capstone verdict.

- Evidence item ID: `EPI08-POS-001`
- EPI cross-references: `EPI-08`
- Evidence status: pass
- Fingerprint scope: controller-binding state, agent/cognition fingerprints, epistemic projection checksum, holder-known context ID/hash/frontier, embodied view/action set, selected semantic action, proposal, validation, event-log segment, and resulting state.
- Evidence summary: `cargo test --locked -p tracewake-core --test acceptance_gates`, `cargo test --locked -p tracewake-tui --test embodied_flow`, `cargo test --locked -p tracewake-tui --test command_loop_session`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. These gates prove human and autonomous paths use the same proposal/validation pipeline, possession preserves actor-owned cognition and available semantic actions, and attach/debug command surfaces do not rewrite embodied cognition.
- Path under test and behavior witness: `controller.rs`, `state.rs`, `projections.rs`, `view_models.rs`, `agent/transaction.rs`, `actions/proposal.rs`, `actions/pipeline.rs`, and TUI command flow. Representative witnesses include `human_and_nonhuman_proposals_share_validation_path`, `possession_controller_binding_is_not_world_state`, `possession_fixture_preserves_intention_needs_and_can_continue`, `bind_render_submit_rerender_and_show_why_not`, and `numeric_selection_executes_stable_semantic_action_id`.
- Replay/provenance ancestry: same-state human/autonomous comparisons preserve context tuple, actor-known projection, action availability, validation rules, and event/state outcome; controller-binding state is the declared non-cognition fingerprint difference.
- Sampling/exhaustiveness: finite EPI-08 positive possession-parity command set required by ticket `0040EPICERHOL-009`.
- Pending or historical handling: relational possession-source capstone breadth remains owned by `0040EPICERHOL-013`.
- Certification use: counted as certifying pass for the EPI-08 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI08-ADV-001`
- EPI cross-references: `EPI-08`
- Evidence status: pass
- Fingerprint scope: possess-A-then-B carryover, debug attachment, unauthorized/malformed command binding, stale or forged selection, prior actor notebook/belief/provenance leakage, and hidden-truth variation.
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates`, `cargo test --locked -p tracewake-tui --test command_loop_session`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. These gates show possession does not transfer prior actor cognition/provenance/debug truth, and malformed or debug-only command paths stay typed/debug-only without creating actor knowledge or accepted events.
- Path under test and behavior witness: `debug_item_does_not_leak_to_following_view_or_change_checksum`, `malformed_debug_actor_id_is_typed_error`, `adversarial_gates_possession_rebind_does_not_transfer_notebook_or_debug_truth` via the same TUI boundary family covered by the ticket's command loop, `debug_truth_never_enters_holder_known_context_hash`, and `epistemic_context_projection_and_records_remain_sealed`.
- Replay/provenance ancestry: rejected or debug-only controller operations do not append cognition-changing events and do not introduce prior actor sources into the newly possessed actor's provenance table.
- Sampling/exhaustiveness: finite named hidden-truth, command-source, and registered fixture corpus required by ticket `0040EPICERHOL-009`.
- Pending or historical handling: full relational anti-contamination matrix remains owned by `0040EPICERHOL-013`.
- Certification use: counted as certifying pass for the EPI-08 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI08-REPLAY-001`
- EPI cross-references: `EPI-08`
- Evidence status: pass
- Fingerprint scope: replayed autonomous and human event inputs, proposal validation, accepted event envelopes, state/projection checksum, no-human capstone ancestry, and fixture serialization.
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates`, `cargo test --locked -p tracewake-core --test no_human_capstone`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. Replay evidence proves possession-parity event/state outcomes and autonomous no-human ancestry survive accepted-log rebuilds.
- Path under test and behavior witness: `phase3a_agent_events_apply_live_and_replay_to_same_agent_checksum`, `replay_rebuild_checksum_matches_original_after_no_human_day`, `no_human_capstone_proves_typed_ancestry_and_replay`, `serialized_event_log_replays_to_identical_state`, and registered possession/no-human fixture rebuilds.
- Replay/provenance ancestry: serialized event input rebuilds preserve same-state parity relations and explain expected controller-binding differences by explicit fingerprint scope.
- Sampling/exhaustiveness: finite EPI-08 replay command set required by ticket `0040EPICERHOL-009`.
- Pending or historical handling: none for this row beyond mutation/capstone consolidation.
- Certification use: counted as certifying pass for the EPI-08 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

## EPI-09 - Embodied Projection Source, Notebook, Action Availability, Why-Not, And Stale-Snapshot Behavior

Status: evidence collected by `0040EPICERHOL-010`; aggregate row remains pending for the mutation package and capstone verdict.

- Evidence item ID: `EPI09-POS-001`
- EPI cross-references: `EPI-09`
- Evidence status: pass
- Fingerprint scope: sealed holder-known context, embodied projection source/snapshot, context ID/hash/frontier, semantic view-model serialization, notebook entries, action availability, why-not rows, actor-safe source summaries, and rendered transcript.
- Evidence summary: `cargo test --locked -p tracewake-tui --test embodied_flow`, `cargo test --locked -p tracewake-tui --test transcript_snapshot`, `cargo test --locked -p tracewake-tui --test tui_seam_conformance`, `cargo test --locked -p tracewake-core --test acceptance_gates`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. These gates prove embodied views are built from sealed actor-known context and bounded snapshots, render local/actor-safe affordances, and preserve deterministic transcript/view serialization.
- Path under test and behavior witness: `projections.rs`, `view_models.rs`, `epistemics/knowledge_context.rs`, `actions/report.rs`, `crates/tracewake-tui/src/app.rs`, `render.rs`, and `transcript.rs`. Representative witnesses include `embodied_view_omits_raw_workplace_assignment_without_context`, `embodied_workplace_availability_reflects_belief_not_truth`, `bind_render_submit_rerender_and_show_why_not`, `transcript_snapshot_is_byte_identical_across_runs`, and `tui_epistemic_debug_uses_core_builder_not_raw_projection_storage`.
- Replay/provenance ancestry: embodied source summaries and why-not/action rows derive from sealed context/projection inputs, not unrestricted live `PhysicalState` handles.
- Sampling/exhaustiveness: finite EPI-09 positive view-model command set required by ticket `0040EPICERHOL-010`.
- Pending or historical handling: debug capability isolation depth remains owned by `0040EPICERHOL-011`.
- Certification use: counted as certifying pass for the EPI-09 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI09-ADV-001`
- EPI cross-references: `EPI-09`
- Evidence status: pass
- Fingerprint scope: hidden item, closed-container contents, hidden route, raw workplace assignment, unknown sleep affordance, other actor private belief, debug report, validator-only fact, stale view token, and privileged debug projection access.
- Evidence summary: `cargo test --locked -p tracewake-tui --test adversarial_gates`, `cargo test --locked -p tracewake-core --test negative_fixture_runner`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. These gates prove hidden/debug/validator-only facts do not enter embodied structs, labels, why-not text, transcript, or action dispatch, and privileged debug projection construction outside the authorized core API remains blocked.
- Path under test and behavior witness: `adversarial_gates_tui_rule_inference_cannot_apply_hidden_food_target`, `adversarial_gates_why_not_actor_surface_uses_typed_non_leaking_facts`, `adversarial_gates_stale_view_token_fails_after_state_change`, `debug_panel_does_not_change_embodied_affordances`, `source_scan_smoke_tui_does_not_call_event_applier`, and `debug_report_construction_without_capability_compile_fails`.
- Replay/provenance ancestry: stale or mismatched view/context tuples reject before becoming fresh proposal sources; debug-only data stays outside embodied semantic serialization and actor feedback.
- Sampling/exhaustiveness: finite named hidden-field, debug-quarantine, stale-view, and negative-fixture corpus required by ticket `0040EPICERHOL-010`.
- Pending or historical handling: none for this evidence row beyond mutation/capstone consolidation.
- Certification use: counted as certifying pass for the EPI-09 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI09-REPLAY-001`
- EPI cross-references: `EPI-09`
- Evidence status: pass
- Fingerprint scope: replay-rebuilt embodied inputs, event envelope identity, context hash/frontier, semantic view serialization, transcript bytes, and tamper/stale proposal controls.
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates`, `cargo test --locked -p tracewake-content --test golden_fixtures_run`, and `cargo test --locked -p tracewake-tui --test transcript_snapshot` passed. Replay and transcript gates prove rebuilt inputs reproduce the same actor-visible model and tampered/stale inputs fail rather than silently refreshing hidden truth.
- Path under test and behavior witness: `replay_rebuild_checksum_matches_original_after_no_human_day`, `deterministic_rebuild_context_hash_uses_causal_and_latest_witnesses`, `serialized_event_log_replays_to_identical_state`, `no_human_decision_context_hash_gate_fails_when_source_evidence_tampered`, `phase3a_debug_snapshot_is_byte_identical_across_runs`, and `tui_transcript_snapshot_remains_byte_stable`.
- Replay/provenance ancestry: accepted logs rebuild the context/projection sources used by the view model; transcript determinism checks the rendered actor-visible surface separately from debug output.
- Sampling/exhaustiveness: finite EPI-09 replay/transcript command set required by ticket `0040EPICERHOL-010`.
- Pending or historical handling: none for this evidence row beyond mutation/capstone consolidation.
- Certification use: counted as certifying pass for the EPI-09 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

## EPI-10 - Debug Capability Isolation, Report/View Separation, TUI Quarantine, And No Feedback Path

Status: evidence collected by `0040EPICERHOL-011`; aggregate row remains pending for the mutation package and capstone verdict.

- Evidence item ID: `EPI10-POS-001`
- EPI cross-references: `EPI-10`
- Evidence status: pass
- Fingerprint scope: core-owned `DebugCapability`, debug report/view identities, `debug_only()` contract, `DEBUG NON-DIEGETIC` marker, debug panel rendering, embodied output, selected proposal, event log, state checksum, projection checksum, and actor-known context hash.
- Evidence summary: `cargo test --locked -p tracewake-tui --test adversarial_gates`, `cargo test --locked -p tracewake-tui --test tui_seam_conformance`, `cargo test --locked -p tracewake-tui --test transcript_snapshot`, `cargo test --locked -p tracewake-tui --test tui_acceptance`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. These gates prove authorized debug output remains in the non-diegetic/debug channel while embodied rendering and transcripts remain deterministic and structurally separate.
- Path under test and behavior witness: `debug_capability.rs`, `debug_reports.rs`, `view_models.rs`, `debug_panels.rs`, `render.rs`, and TUI app surfaces. Representative witnesses include `phase3a_debug_surfaces_render_deterministically_and_read_only`, `debug_truth_does_not_enter_embodied_view`, `tui_transcript_marks_debug_sections_non_diegetic`, `tui_epistemic_debug_uses_core_builder_not_raw_projection_storage`, and `phase3a_debug_snapshot_is_byte_identical_across_runs`.
- Replay/provenance ancestry: debug-enabled and debug-rendered surfaces are observed as read-only/non-diegetic artifacts, not as simulation facts, events, beliefs, observations, proposals, or actor feedback.
- Sampling/exhaustiveness: finite EPI-10 positive debug-channel command set required by ticket `0040EPICERHOL-011`.
- Pending or historical handling: compile-fail matrix consolidation remains cross-referenced to `0040EPICERHOL-012`.
- Certification use: counted as certifying pass for the EPI-10 positive-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI10-ADV-001`
- EPI cross-references: `EPI-10`
- Evidence status: pass
- Fingerprint scope: external debug capability/context/report/projection construction, debug token leakage, debug prose routing, debug command strings, embodied labels/why-not/feedback/event payloads, and actor-known context hash.
- Evidence summary: `cargo test --locked -p tracewake-core --test negative_fixture_runner`, `cargo test --locked -p tracewake-core --test hidden_truth_gates`, `cargo test --locked -p tracewake-tui --test adversarial_gates`, and `cargo test --locked -p tracewake-tui --test tui_acceptance` passed. These gates prove external debug construction remains blocked, debug truth does not enter actor-known contexts or embodied views, and debug command/prose channels do not create typed facts or actor-safe reasons.
- Path under test and behavior witness: `debug_report_construction_without_capability_compile_fails`, `debug_omniscience_facts_are_excluded_from_planner_context`, `debug_truth_never_enters_holder_known_context_hash`, `debug_panel_does_not_change_embodied_affordances`, `debug_command_strings_are_not_embodied_commands`, and `leakage_debug_truth_does_not_enter_embodied_view`.
- Replay/provenance ancestry: rejected external/debug-prose routes do not append accepted events or actor-known projection records; debug validation detail remains outside actor-safe feedback absent a modeled event.
- Sampling/exhaustiveness: finite named capability, token-absence, and no-feedback negative corpus required by ticket `0040EPICERHOL-011`.
- Pending or historical handling: full compile-fail matrix result remains owned by `0040EPICERHOL-012`.
- Certification use: counted as certifying pass for the EPI-10 adversarial/negative-evidence column; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI10-REPLAY-001`
- EPI cross-references: `EPI-10`
- Evidence status: pass
- Fingerprint scope: debug-disabled/debug-enabled event inputs, authoritative state checksum, event log, epistemic projection checksum, actor-known context, selected proposal, embodied output, and replay reports.
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates`, `cargo test --locked -p tracewake-tui --test transcript_snapshot`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. Replay evidence proves debug surfaces are non-interfering: independent rebuilds preserve authoritative/epistemic results and actor-visible relations.
- Path under test and behavior witness: `replay_rebuild_checksum_matches_original_after_no_human_day`, `typed_diagnostic_hidden_truth_true_is_validated_and_replayed`, `serialized_event_log_replays_to_identical_state`, `phase3a_debug_snapshot_is_byte_identical_across_runs`, and `tui_transcript_is_deterministic`.
- Replay/provenance ancestry: debug report/view rendering is separated from accepted event inputs and does not change replayed state/projection/context/proposal identities.
- Sampling/exhaustiveness: finite EPI-10 replay/transcript command set required by ticket `0040EPICERHOL-011`.
- Pending or historical handling: none for this evidence row beyond mutation/capstone consolidation.
- Certification use: counted as certifying pass for the EPI-10 replay/provenance column; aggregate seam verdict remains pending until mutation/capstone consolidation.

## EPI-11 - Relational Anti-Contamination And Possession-Parity Capstone

Status: evidence collected by `0040EPICERHOL-013`; aggregate row remains pending for the mutation package and capstone verdict.

- Evidence item ID: `EPI11-REL-001`
- EPI cross-references: `EPI-11`, `EPI-01` through `EPI-10`
- Evidence status: pass
- Fingerprint scope: paired focal actor-observable event prefixes, actor-known context, notebook, available actions, proposal, embodied render, legal reveal event/provenance, post-reveal divergence ancestry, and replay confirmation.
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates`, `cargo test --locked -p tracewake-core --test acceptance_gates`, `cargo test --locked -p tracewake-core --test golden_scenarios`, `cargo test --locked -p tracewake-content --test golden_fixtures_run`, `cargo test --locked -p tracewake-tui --test adversarial_gates`, and `cargo test --locked -p tracewake-tui --test embodied_flow` passed. These gates cover the seven §EPI-11 hidden-variable pairs at actor-visible/decision-producing surfaces rather than using whole-world checksum equality.
- Path under test and behavior witness: closed-container and hidden-route pairs are covered by `hidden_food_closed_container_is_not_actor_known_food_source`, `embodied_affordances_exclude_hidden_food_in_closed_container`, `hidden_route_edge_absent_from_actor_context_blocks_route_plan`, and `hidden_food_unknown_route_does_not_become_transaction_target`; workplace truth by `embodied_workplace_believed_open_truth_closed`, `embodied_workplace_availability_reflects_belief_not_truth`, and workplace provenance fixtures; other-actor privacy by `epistemic_context_projection_and_records_remain_sealed`; debug state by `debug_truth_never_enters_holder_known_context_hash`, `debug_panel_does_not_change_embodied_affordances`, and `adversarial_gates_debug_truth_does_not_enter_actor_surfaces`; possession source by `human_and_nonhuman_proposals_share_validation_path`, `possession_controller_binding_is_not_world_state`, and `possession_fixture_preserves_intention_needs_and_can_continue`; stale truth by `adversarial_gates_stale_view_token_fails_after_state_change` and `tui_current_view_submission_rejects_stale_selection`.
- Replay/provenance ancestry: post-reveal divergence is required to trace to an accepted event/provenance chain, and both pair members reproduce under replay via `event_schema_replay_gates` and golden fixture replay checks.
- Sampling/exhaustiveness: finite named pair matrix from spec §EPI-11; no generated claim is treated as exhaustive unless the finite command/test domain is the named suite.
- Pending or historical handling: mutation witness linkage remains owned by `0040EPICERHOL-014`.
- Certification use: counted as certifying pass for the EPI-11 relational-evidence columns; aggregate seam verdict remains pending until mutation/capstone consolidation.

- Evidence item ID: `EPI11-GEN-001`
- EPI cross-references: `EPI-11`, `§6.2`
- Evidence status: sampled pass
- Fingerprint scope: recorded deterministic seeds, generated event prefixes, source/provenance order, single-field semantic tamper, source-removal/substitution, replay round trip, prefix replay, checksum/hash sensitivity, and minimized counterexample retention.
- Evidence summary: `cargo test --locked -p tracewake-core --test generative_lock` passed with the existing recorded-seed generator. The harness checks replay equality, canonical serialization round trips, prefix replay, single need-charge locks, payload tamper poisoning, duration-terminal tamper poisoning, and marker append non-interference across the recorded seed set.
- Path under test and behavior witness: `generated_sequences_replay_and_satisfy_metamorphic_locks` and `duration_terminal_targeted_tamper_requires_duration_terminal`.
- Replay/provenance ancestry: generated event logs are serialized, deserialized, replayed, prefix-replayed, and tampered; failures are required to name replay/validation layers instead of silently preserving original outputs.
- Sampling/exhaustiveness: sampled generated evidence with recorded `GENERATIVE_SEEDS`; no finite-domain exhaustion is claimed.
- Pending or historical handling: retained counterexamples were not present in this run because the generated suite passed.
- Certification use: counted as §6.2 sampled metamorphic evidence for EPI-11; aggregate seam verdict remains pending until mutation/capstone consolidation.

## Section 6.1 Compile-Fail Boundary Corpus Matrix

Status: evidence collected by `0040EPICERHOL-012`; aggregate use remains pending for capstone consolidation.

- Evidence item ID: `SEC61-COMPILE-001`
- EPI cross-references: `EPI-01`, `EPI-02`, `EPI-03`, `EPI-06`, `EPI-10`
- Evidence status: pass
- Fingerprint scope: external-crate API boundary for sealed epistemic constructors, private fields, source/scope mutation, raw projection storage, debug capability minting, debug context/report/view construction, and banned float confidence surfaces.
- Evidence summary: `cargo test --locked -p tracewake-core --test negative_fixture_runner`, `cargo test --locked -p tracewake-core --doc`, and `cargo test --locked -p tracewake-core --test negative_fixture_runner -- --list` passed. The runner confirmed the registered negative corpus, banned API proving fixtures, debug report construction guard, and phase1 loader mutation guard remain negative; doctests confirmed compile-fail boundaries including `DebugCapability` construction/minting.
- Path under test and behavior witness: `tests/negative-fixtures/**`, `debug_capability.rs`, `debug_reports.rs`, `agent/actor_known.rs`, `agent/no_human_surface.rs`, `view_models.rs`, and `negative_fixture_runner`.
- Replay/provenance ancestry: not applicable to compile-fail evidence; the proof closes external API paths before runtime.
- Sampling/exhaustiveness: 13 enumerated §6.1 rows below: 12 negative-fixture directory classes plus the `debug_capability.rs` doctest pair as one corpus row.
- Pending or historical handling: capstone must decide aggregate use; this row only records compile-fail corpus status.
- Certification use: counted as consolidated compile-fail negative evidence for EPI rows that cite §6.1; aggregate verdict remains pending.

| Corpus row | Forbidden capability | Compiler failure class | Relevant EPI point(s) | API-closure rationale |
|---|---|---|---|---|
| `banned_float_confidence_types` | External code attempts raw float confidence for epistemic values. | Clippy/compile-fail banned API fixture. | `EPI-02`, `EPI-06` | Confidence must pass through typed epistemic currency; the fixture proves the banned surface is structurally unavailable rather than accepted with a fragile diagnostic string. |
| `external_crate_cannot_build_debug_knowledge_context` | External crate constructs debug holder-known context. | Private capability/constructor boundary. | `EPI-01`, `EPI-10` | Debug context construction requires core-owned authority; external code cannot forge the mode/viewer/capability tuple. |
| `external_crate_cannot_build_debug_projection_view_without_core_debug_api` | External crate builds privileged debug projection view. | Private constructor/capability boundary. | `EPI-06`, `EPI-09`, `EPI-10` | Privileged projection view creation is routed through core debug APIs; raw projection maps are not externally readable as a substitute. |
| `external_crate_cannot_construct_belief_literal` | External crate creates a belief literal directly. | Private field/constructor boundary. | `EPI-02` | Beliefs require typed, source-backed construction; external literal construction cannot bypass provenance/scope rules. |
| `external_crate_cannot_construct_debug_report` | External crate constructs debug report/view families. | Capability-sealed debug report constructor boundary. | `EPI-10` | Debug reports require core-owned `DebugCapability`; the fixture closes report fabrication rather than matching a wording-only error. |
| `external_crate_cannot_construct_observation_without_source` | External crate creates observation without source evidence. | Missing source/private constructor boundary. | `EPI-03`, `EPI-05` | Observation construction requires source-event provenance; no external no-source shortcut exists. |
| `debug_capability.rs` doctest pair | External crate constructs `DebugCapability` literally or calls `DebugCapability::mint()`. | Private fields and `pub(crate)` mint boundary. | `EPI-10` | The capability's literal fields are inaccessible and `mint()` is crate-private, proving unforgeability at the type boundary. |
| `external_crate_cannot_insert_raw_epistemic_records` | External crate inserts raw epistemic records into projection storage. | Private storage/API boundary. | `EPI-06` | Projection records are replay-derived through accepted event application; external raw insertion is not an exposed writer. |
| `external_crate_cannot_mutate_belief_source_or_scope` | External crate mutates belief source or privacy scope. | Private field/mutator absence boundary. | `EPI-02`, `EPI-05` | Source/scope are sealed after validated construction; external mutation cannot reclassify provenance or privacy. |
| `external_crate_cannot_mutate_contradiction_links` | External crate rewires contradiction links. | Private field/mutator absence boundary. | `EPI-04` | Contradiction relation links are produced by typed event/projection logic; external mutation cannot fabricate relation evidence. |
| `external_crate_cannot_mutate_knowledge_context_mode` | External crate changes holder-known context mode. | Private field/mutator absence boundary. | `EPI-01`, `EPI-10` | Context mode is sealed by construction; external code cannot flip actor/debug mode after the fact. |
| `external_crate_cannot_mutate_knowledge_context_viewer` | External crate changes context viewer/holder. | Private field/mutator absence boundary. | `EPI-01`, `EPI-08` | Viewer identity is sealed in the context packet; external code cannot retarget another actor's context. |
| `external_crate_cannot_read_raw_epistemic_projection_maps` | External crate reads raw projection maps directly. | Private storage/API boundary. | `EPI-06`, `EPI-09` | Context-filtered APIs are the exposed read path; raw maps are not externally accessible for bypassing privacy/freshness filters. |

## Section 9.4 Per-Seam Verdict Table

| Requirement | Responsible layer(s) | Positive evidence | Adversarial/negative evidence | Replay/provenance evidence | Mutation evidence | Result from certifying evidence |
|---|---|---|---|---|---|---|
| `EPI-01` sealed context identity/scope/hash/frontier | actor-known context construction; proposal construction | `EPI01-POS-001`, `EPI01-PROPOSAL-001` | `EPI01-ADV-001`, `EPI01-COMPILE-001` | `EPI01-REPLAY-001` | `MUT-WAVEB-001` | pass |
| `EPI-02` beliefs/privacy/freshness | content/schema validation; projection/replay | `EPI02-POS-001`, `EPI02-FRESH-001` | `EPI02-ADV-001`, `EPI02-COMPILE-001` | `EPI02-REPLAY-001` | `MUT-WAVEB-001`; survivor floor includes epistemics/observation/projection coverage gaps | fail - scoped remediation |
| `EPI-03` observation channels/event capture | event application; projection/replay | `EPI03-POS-001`, `EPI03-STAGED-READING-001` | `EPI03-ADV-001`, `EPI03-COMPILE-001` | `EPI03-REPLAY-001` | `MUT-WAVEB-001`; survivor floor includes observation/replay-sensitive coverage gaps | fail - scoped remediation |
| `EPI-04` contradiction/absence discipline | projection/replay; view-model rendering | `EPI04-POS-001`, `EPI04-STAGED-001` | `EPI04-ADV-001`, `EPI04-COMPILE-001` | `EPI04-REPLAY-001` | `MUT-WAVEB-001`; survivor floor includes projection/contradiction coverage gaps | fail - scoped remediation |
| `EPI-05` provenance/witness sufficiency | actor-known context construction; proposal/action validation | `EPI05-POS-001` | `EPI05-ADV-001` | `EPI05-REPLAY-001` | `MUT-WAVEB-001`; survivor floor includes source/provenance validation coverage gaps | fail - scoped remediation |
| `EPI-06` projection rebuild/non-writer | event application; projection/replay | `EPI06-POS-001` | `EPI06-ADV-001` | `EPI06-REPLAY-001` | `MUT-WAVEB-001`; survivor floor includes projection/rebuild coverage gaps | fail - scoped remediation |
| `EPI-07` decision/proposal parity/truth firewall | candidate/planning/proposal/action validation | `EPI07-POS-001` | `EPI07-ADV-001` | `EPI07-REPLAY-001` | `MUT-WAVEB-001`; survivor floor includes proposal/validation coverage gaps | fail - scoped remediation |
| `EPI-08` possession parity | actor-known context; view-model; proposal/action validation | `EPI08-POS-001` | `EPI08-ADV-001` | `EPI08-REPLAY-001` | `MUT-WAVEB-001` | pass |
| `EPI-09` embodied view/notebook/why-not | projection/replay; view-model rendering | `EPI09-POS-001` | `EPI09-ADV-001` | `EPI09-REPLAY-001` | `MUT-WAVEB-001`; survivor floor includes embodied/projection rendering coverage gaps | fail - scoped remediation |
| `EPI-10` debug quarantine | debug quarantine; view-model rendering | `EPI10-POS-001` | `EPI10-ADV-001` | `EPI10-REPLAY-001` | `MUT-WAVEB-001`; survivor floor includes display/error-format coverage gaps | fail - scoped remediation |
| `EPI-11` relational capstone | actor-known context; proposal construction; view-model rendering; projection/replay; debug quarantine | `EPI11-REL-001`, `EPI11-GEN-001` | `EPI11-REL-001` | `EPI11-REL-001`, `EPI11-GEN-001` | `MUT-WAVEB-001`; survivor floor covers multiple relational layers | fail - scoped remediation |
| Configured EPI mutation perimeter | projection/replay; content/schema validation; diagnostics; view-model rendering | `MUT-WAVEB-001` | `MUT-WAVEB-001` | `MUT-WAVEB-001` | `MUT-WAVEB-001`, `reports/0040_epi_cert_mutation_triage_register.md` | fail - scoped remediation |
| Artifact/evidence honesty | documentation status; tests/fixtures | `0040-BASELINE-001`, `0040-NAMED-BINS-001` | `SEC61-COMPILE-001`, `MUT-WAVEB-001` | `EPI01-REPLAY-001` through `EPI11-REL-001`, `EPI11-GEN-001` | `MUT-WAVEB-001`, `reports/0040_epi_cert_mutation_triage_register.md` | pass |

## Section 9.5 Replay And Provenance Package

Status: packaged by `0040EPICERHOL-015`; complete for verdict purposes, with mutation remediation still required.

- Serialized accepted event inputs: `EPI01-REPLAY-001`, `EPI03-REPLAY-001`, `EPI05-REPLAY-001`, `EPI06-REPLAY-001`, `EPI07-REPLAY-001`, `EPI08-REPLAY-001`, `EPI10-REPLAY-001`, `EPI11-REL-001`, and `EPI11-GEN-001` cover accepted log prefixes, no-human runs, golden scenario inputs, fixture serializations, and relational pair members. The registered `golden_fixtures_run` corpus is the exhaustive fixture family runner for the §6 matrix.
- Event envelope/index/fingerprint and source-event witnesses: `EPI01-REPLAY-001`, `EPI03-REPLAY-001`, `EPI05-REPLAY-001`, `EPI06-REPLAY-001`, and `EPI07-REPLAY-001` cover event envelope identity, payload schema, source/cause fields, source-event deletion/tamper controls, append order, replay mismatch reports, and first-divergence fingerprints.
- Live/replay authoritative and epistemic checksums: `EPI06-POS-001`, `EPI06-REPLAY-001`, `EPI08-REPLAY-001`, `EPI09-REPLAY-001`, and `EPI10-REPLAY-001` cover authoritative state checksums, projection checksums, actor-known context hashes, canonical serialization round trips, duplicate/reordered event rejection, and debug-disabled/debug-enabled replay non-interference.
- Sealed holder-known packets: `EPI01-POS-001`, `EPI01-ADV-001`, `EPI01-REPLAY-001`, `EPI02-POS-001`, `EPI05-POS-001`, and `EPI06-POS-001` cover packet IDs, hashes, frontiers, viewer/scope decisions, source-backed construction, content/schema identities, and compile-fail proof that external code cannot forge or retarget the packet.
- Proposal source tuples and validation split: `EPI01-PROPOSAL-001`, `EPI05-POS-001`, `EPI05-ADV-001`, `EPI07-POS-001`, `EPI07-ADV-001`, `EPI07-REPLAY-001`, and `EPI08-POS-001` cover context tuple retention, validation results, accepted/rejected events, actor-safe reasons, controller-binding scope, and debug rejection reports.
- Embodied/debug artifacts: `EPI08-POS-001`, `EPI09-POS-001`, `EPI09-ADV-001`, `EPI09-REPLAY-001`, `EPI10-POS-001`, `EPI10-ADV-001`, and `EPI10-REPLAY-001` cover embodied view-model serialization, transcript snapshots, actor-safe why-not output, separate authorized debug artifacts, `DEBUG NON-DIEGETIC` markers, and no leakage into actor-visible channels.
- Contradiction/freshness matrices: `EPI02-FRESH-001`, `EPI04-POS-001`, `EPI04-ADV-001`, `EPI04-REPLAY-001`, and `EPI04-STAGED-001` cover supersession, expected absence, contradiction creation, unsupported-kind non-overclaim handling, and source-backed replay survival.
- Relational pair declarations and reveal ancestry: `EPI11-REL-001` names every hidden-variable pair, equality precondition, actor-visible/decision-producing comparison domain, reveal/divergence rule, replay confirmation, seed/sample scope, and first responsible layer; `EPI11-GEN-001` records sampled generated replay/tamper/non-interference controls without counting sampled evidence as finite-domain exhaustion.
- First-divergence reports: passing replay controls cite match/mismatch report witnesses in `EPI03-REPLAY-001`, `EPI05-REPLAY-001`, `EPI06-REPLAY-001`, and `EPI07-REPLAY-001`. The only certifying failure package is the mutation floor in `MUT-WAVEB-001`, whose first responsible layers are tracked per survivor in `reports/0040_epi_cert_mutation_triage_register.md`.

## Section 9.6 Mutation Package

Status: evidence collected by `0040EPICERHOL-014`; result routes `EPI-CERT scoped remediation`.

- Evidence item ID: `MUT-WAVEB-001`
- EPI cross-references: mutation, `EPI-02`, `EPI-03`, `EPI-04`, `EPI-05`, `EPI-06`, `EPI-07`, `EPI-09`, `EPI-10`, `EPI-11`
- Evidence status: fail
- Fingerprint scope: checked-in `.cargo/mutants.toml`, `.github/workflows/ci.yml`, `Cargo.lock`, Wave A file census, Wave B expanded file/mutant census, Wave B run output, timeout retry output, final missed set, and survivor register.
- Evidence summary: `.cargo/mutants.toml` was expanded additively with `crates/tracewake-core/src/epistemics/**`, increasing the configured file census from 48 to 54 files and Wave B mutant census to 2763 mutants. `cargo test --workspace --locked` passed before mutation interpretation. Final Wave B ran with `cargo mutants --workspace --no-shuffle -j 8 -o reports/0040_epi_cert_mutation_wave_b_j8.out`: 2763 tested, 2143 caught, 589 unviable, 27 missed, 4 timeouts. Timeout retry ran with `cargo mutants --workspace --no-shuffle -j 1 --timeout 600 -F 'current_place_perception_events|Confidence::parts_per_thousand|PropositionParseError|PropositionReferenceError' -o reports/0040_epi_cert_mutation_wave_b_timeout_retry.out`: 22 tested, 10 caught, 5 unviable, 7 missed, 0 timeouts. Unique final missed-mutant floor: 30.
- Path under test and behavior witness: `reports/0040_epi_cert_mutation_wave_a_list_files.txt`, `reports/0040_epi_cert_mutation_wave_b_list_files.txt`, `reports/0040_epi_cert_mutation_wave_b_list.txt`, `reports/0040_epi_cert_mutation_wave_b_missed.txt`, `reports/0040_epi_cert_mutation_wave_b_timeout.txt`, `reports/0040_epi_cert_mutation_timeout_retry_missed.txt`, `reports/0040_epi_cert_mutation_timeout_retry_timeout.txt`, `reports/0040_epi_cert_mutation_final_missed.txt`, and `reports/0040_epi_cert_mutation_triage_register.md`.
- Replay/provenance ancestry: mutation command used the checked-in config, `test_workspace = true`, `additional_cargo_args = ["--locked"]`, no `--no-config`, no `-f` final file list, and no `--iterate`; baseline was not skipped. CI mutation jobs use the checked-in config and do not pin a narrower standing file list, so `.github/workflows/ci.yml` was not changed.
- Sampling/exhaustiveness: exhaustive Wave B configured union census for the expanded checked-in perimeter; interrupted serial output in `reports/0040_epi_cert_mutation_wave_b.out` is non-certifying scratch and is not counted.
- Pending or historical handling: no unresolved timeouts remain after retry; no missed mutant is accepted as equivalent/non-critical; all 30 unique missed mutants are routed to later separate EPI-CERT mutation remediation.
- Certification use: certifying mutation evidence fails the configured EPI mutation requirement and forces `EPI-CERT scoped remediation` in the capstone.

## Section 9.7 Relational Capstone Package

Status: evidence collected by `0040EPICERHOL-013`; consolidated by `0040EPICERHOL-015`.

| Pair | Hidden input difference | Equality precondition | Actor-visible / decision-producing fields compared | Pre-reveal result | Legal reveal / divergence handling | Replay confirmation | Seed/sample scope | First responsible layer on failure |
|---|---|---|---|---|---|---|---|---|
| Closed container | Item present vs absent inside unopened/unsearched container. | Same focal observation/event history before open/search/touch. | Actor-known food/container facts, embodied affordances, proposal target eligibility, actor-safe reason. | pass via `hidden_truth_gates`, `acceptance_gates`, `embodied_flow`. | Accepted open/search/touch plus observation event may reveal; before that hidden contents stay absent. | `event_schema_replay_gates`, `golden_fixtures_run`. | finite named tests, not generated exhaustion. | actor-known context construction; view-model rendering; action validation. |
| Hidden route | Route edge exists vs absent but not perceived/recorded. | Same known exits and route records. | Known exits, movement availability, decision trace, sealed proposal. | pass via `hidden_truth_gates` and registered fixtures. | Sight/notice/search event may convey route knowledge. | `event_schema_replay_gates`, `golden_fixtures_run`. | finite named tests. | actor-known context construction; proposal construction. |
| Workplace truth | Workplace open vs closed while actor has same belief/notice. | Same belief/provenance packet and context hash before validation. | Belief, availability, proposal, actor-safe rationale. | pass via `embodied_flow`, `acceptance_gates`, workplace fixture witnesses. | Validation may reject; belief changes only through legal feedback/observation. | `event_schema_replay_gates`, `golden_fixtures_run`. | finite named tests. | action validation; actor-known context construction. |
| Other actor privacy | Actor B private belief/observation differs. | Actor A has no permitted communication/record path. | Actor A context, notebook, decision inputs, embodied view. | pass via `hidden_truth_gates` privacy/sealed projection witnesses. | Explicit modeled communication/record path if implemented and in scope. | `event_schema_replay_gates`. | finite named tests. | actor-known context construction; projection/replay. |
| Debug state | Debug detached vs attached with omniscient report. | Same simulation events and actor-known context. | State/event/projection/context/proposal/embodied artifacts. | pass via `adversarial_gates`, `tui_acceptance`, `hidden_truth_gates`. | No diegetic divergence is legal from debug alone. | `event_schema_replay_gates`, transcript/golden runs. | finite named tests. | debug quarantine; view-model rendering. |
| Possession source | Autonomous vs human controller, same actor state and semantic choice. | Same actor cognition, context, physical state, and action availability. | Context, actions, validation rules, event/state result. | pass via `acceptance_gates`, `golden_scenarios`, `golden_fixtures_run`. | Controller-binding/debug metadata only, outside actor cognition. | `event_schema_replay_gates`, `golden_fixtures_run`. | finite named tests. | proposal construction; action validation; controller binding. |
| Stale truth | Hidden truth changes after snapshot but before perception. | Same stale embodied model/context token before admissible observation. | Existing menu, context tuple, proposal source freshness, actor decision input. | pass via `adversarial_gates` stale-view checks and TUI submission guards. | Later admissible observation and rebuilt context may diverge. | `event_schema_replay_gates`, transcript/golden runs. | finite named tests. | view-model rendering; proposal construction. |

Generated/metamorphic package: `EPI11-GEN-001` is sampled evidence from recorded deterministic seeds. It records replay, canonical serialization, prefix replay, payload/source tamper, duration-terminal tamper, and non-interference checks. No generated finite-domain exhaustion or retained counterexample is claimed for this passing run.

## Section 9.8 EMERGE-OBS Handling

Status: packaged by `0040EPICERHOL-015`; observer-only, non-gating.

The verified corpus includes `cargo test --locked -p tracewake-core --test emergence_ledger`, recorded in the command ledger as `EMERGE-OBS` evidence. This evidence is labeled observer-only: it may inform the living-world acceptance record, but it does not alter any `EPI-01` through `EPI-11` row, does not satisfy the mutation perimeter, does not become a phase gate or pass/fail threshold, and is not a scheduler objective or quality substitute.

## Section 9.9 Aggregate Verdict

Verdict: `EPI-CERT scoped remediation`

The aggregate verdict is `EPI-CERT scoped remediation`.

Rows `EPI-01` through `EPI-11`, the §6 compile-fail matrix, the §9.5 replay/provenance package, the §9.7 relational capstone package, and the §9.8 observer-only emergence package are populated. The configured EPI mutation perimeter fails because `MUT-WAVEB-001` leaves a 30-mutant survivor floor with no accepted equivalence or non-critical disposition. Under §9.9 condition 6, that failure blocks `EPI-CERT passed` and routes the work to later scoped remediation.

Responsible remediation layers named by the survivor register: epistemics observation/proposition/projection records, event/projection rebuild behavior, proposal/validation source checks, embodied/debug rendering and diagnostic formatting, content/schema validation, and replay-sensitive source/provenance boundaries. The exact survivor identities are recorded in `reports/0040_epi_cert_mutation_final_missed.txt` and triaged in `reports/0040_epi_cert_mutation_triage_register.md`.

This verdict does not relabel the gate, does not grant a partial pass, and does not authorize skipping to ORD-LIFE-CERT, FIRST-PROOF-CERT, Phase-4 entry, second-proof entry, institutions, notices, travel, LOD, or LLM/speech surfaces. It certifies only the implementation-session evidence named in this artifact and does not independently verify current `main`.
