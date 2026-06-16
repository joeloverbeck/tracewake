# P0-CERT post-0008 baseline certification acceptance artifact

Spec: specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md
Repository: joeloverbeck/tracewake
Target commit: 9f1622244c91c5952bd735da76f29fbe58f39f4b
Freshness claim: user-supplied target commit only; not independently verified as latest main
Verdict: <pending>
Verdict scope: post-0008 baseline only
Archived evidence posture: historical only
EMERGE-OBS posture: observer-only, non-gating, non-threshold

## Evidence Status Legend

Every evidence item carries exactly one status:

- `pass` - the artifact actually certifies the checked claim.
- `fail` - the checked claim failed and requires remediation.
- `pending` - the check has not yet produced certifying evidence.
- `sampled` - representative, not exhaustive.
- `observer-only` - review evidence that cannot certify behavior.
- `historical` - archive/spec/ticket evidence used only as context.

`pending`, `sampled`, `observer-only`, and `historical` are not counted as pass.

## Fingerprint Scope Legend

Fingerprints and stable artifacts declare one scope:

- `raw bytes`
- `normalized serialization`
- `parsed semantic content`
- `command transcript`
- `run seed`
- `replay artifact`
- `context hash/frontier`
- `projection checksum`
- `event log checksum`
- `not applicable`

Byte-stable goldens are not semantic behavior proof unless paired with behavior
witnesses and replay/provenance ancestry.

## Command And Lock-Layer Transcript

| Command | Status | Fingerprint scope | Transcript summary |
|---|---|---|---|
| `cargo fmt --all --check` | pass | command transcript | Passed on 2026-06-16 with no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | command transcript | Passed on 2026-06-16; `tracewake-core` checked, dev profile finished successfully. |
| `cargo build --workspace --all-targets --locked` | pass | command transcript | Passed on 2026-06-16; workspace all-targets locked build finished successfully. |
| `cargo test --workspace --locked` | pass | command transcript | Passed on 2026-06-16; workspace unit, integration, TUI, content, and doctest suites all reported `ok`. |
| `cargo test -p tracewake-core --test anti_regression_guards` | pass | command transcript | Passed on 2026-06-16; 80 passed, 0 failed. |
| `cargo test -p tracewake-core --test ci_workflow_guards` | pass | command transcript | Passed on 2026-06-16; 1 passed, 0 failed. |
| `cargo test -p tracewake-core --test spine_conformance` | pass | command transcript | Passed on 2026-06-16; 6 passed, 0 failed. |
| `cargo test -p tracewake-core --test hidden_truth_gates` | pass | command transcript | Passed on 2026-06-16; 13 passed, 0 failed. |
| `cargo test -p tracewake-core --test event_schema_replay_gates` | pass | command transcript | Passed on 2026-06-16; 17 passed, 0 failed. |
| `cargo test -p tracewake-content --test golden_fixtures_run` | pass | replay artifact | Passed on 2026-06-16; 40 passed, 0 failed, including fixture fingerprints, no-human replay, context hash, and replay tamper negatives. |
| `cargo test -p tracewake-content --test forbidden_content` | pass | command transcript | Passed on 2026-06-16; 20 passed, 0 failed. |
| `cargo test -p tracewake-content --test schema_conformance` | pass | command transcript | Passed on 2026-06-16; 2 passed, 0 failed. |
| `cargo test -p tracewake-tui --test adversarial_gates` | pass | command transcript | Passed on 2026-06-16; 15 passed, 0 failed. |
| `cargo mutants --version` | pass | command transcript | `cargo-mutants 27.1.0` is installed. |
| `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle` | pending | command transcript | Attempted on 2026-06-16. It found 1128 mutants, passed the unmutated baseline in 9s build + 31s test, then emitted one miss (`crates/tracewake-core/src/projections.rs:336:5 replace actor_known_local_actors_for_context -> Vec<ActorId> with vec![]`) before the local interactive attempt was stopped as too large for this ticket turn. Full scheduled mutation baseline remains pending; this row is not counted as pass. |
| Static guard scans | sampled | command transcript | Bounded `rg` scans on 2026-06-16 covered event append/application call sites, hidden-truth/debug terms, diagnostic typing, controller/debug/player terms, and nondeterminism/collection APIs. Results were reviewed as supporting scan evidence only; lock-layer tests remain the certifying source for these guard classes. |

## Global Replay And Provenance Fingerprints

| Evidence item ID | Evidence status | Fingerprint scope | Evidence summary | Certification use |
|---|---|---|---|---|
| `0036-GLOBAL-MANIFEST` | pass | raw bytes | `cargo test -p tracewake-content --test golden_fixtures_run` passed `fixture_fingerprints_match_frozen_goldens`; fixture manifest fingerprints match the frozen table. | counted as certifying pass for scaffold transcript only |
| `0036-GLOBAL-SEED` | sampled | run seed | `cargo test -p tracewake-core --test generative_lock` passed inside `cargo test --workspace --locked`; detailed seed enumeration remains for capstone/gate evidence. | not counted as certifying evidence |
| `0036-GLOBAL-EVENT-LOG` | pass | event log checksum | `golden_fixtures_run`, `golden_scenarios`, and `event_schema_replay_gates` exercised canonical event-log serialization, tamper negatives, and replay checksums. | counted as certifying pass for scaffold transcript only |
| `0036-GLOBAL-LIVE-PROJECTION` | pass | projection checksum | `golden_fixtures_run` and `golden_scenarios` computed live physical/agent checksums for replay comparisons. | counted as certifying pass for scaffold transcript only |
| `0036-GLOBAL-REPLAY-PROJECTION` | pass | projection checksum | Replay tests reported matching live/replay checksums for positive cases and mismatch for tampered/corrupt cases. Gate-specific divergence rows remain owned by `-002`...`-011`. | counted as certifying pass for scaffold transcript only |
| `0036-GLOBAL-PROVENANCE` | sampled | context hash/frontier | `golden_fixtures_run`, `hidden_truth_gates`, and `event_schema_replay_gates` exercised context hash/frontier and provenance negatives. Full per-gate provenance rows remain owned by `-002`...`-011`. | not counted as certifying evidence |
| `0036-GLOBAL-DEBUG` | observer-only | replay artifact | Debug-only truth/belief comparison rows, if present, remain observer-only and quarantined from certification. | not counted as certifying evidence |

## Per-Requirement Acceptance Evidence

Each requirement row cites evidence item IDs from this artifact. The result is
computed only from certifying evidence items.

| Requirement | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `P0-01` world-affecting behavior enters through proposal -> validation -> event -> application -> projection -> replay | `proposal_construction`, `action_validation`, `event_append`, `event_application`, `projection`, `replay` | `0036-P0-01-PIPELINE-WITNESS`, `0036-P0-01-REPLAY-CHECKSUM`, `0036-P0-01-NO-DIRECT-NEGATIVES` | pass |
| `P0-02` autonomous decisions use sealed actor-known contexts with provenance | `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction` | `0036-P0-02-ACTOR-KNOWN-PROVENANCE`, `0036-P0-02-HIDDEN-TRUTH-NEGATIVES`, `0036-P0-02-CONTEXT-HASH-REPLAY` | pass |
| `P0-03` human-origin commands bind to ordinary actors and share action rules | `tui_input_binding`, `view_model`, `proposal_construction`, `action_validation`, `event_append` | `0036-P0-03-HUMAN-AUTONOMOUS-PARITY`, `0036-P0-03-TUI-BINDING-VIEWMODEL`, `0036-P0-03-DEBUG-PRIVILEGE-NEGATIVES` | pass |
| `P0-04` possession never resets needs, intentions, memories, routines, or world-fact access | `tui_input_binding`, `holder_known_context`, `intention_lifecycle`, `view_model`, `projection` | `0036-P0-04-PREPOST-STATE-FINGERPRINTS`, `0036-P0-04-HOLDER-KNOWN-VIEW-FILTERING`, `0036-P0-04-DEBUG-REBIND-NEGATIVES` | pass |
| `P0-05` scheduler paths cannot emit primitive world actions from raw truth, routine labels, need thresholds, or fixture tables | `scheduler`, `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, `action_validation`, `event_append`, `replay` | `0036-P0-05-NOHUMAN-CAPSTONE-ANCESTRY`, `0036-P0-05-PASSIVE-ACCOUNTING-SINGLE-CHARGE`, `0036-P0-05-SCHEDULER-SHORTCUT-NEGATIVES` | pass |
| `P0-06` validation truth may accept/reject/mutate through events but may not propose fallback plans or actor-visible hidden facts | `action_validation`, `event_application`, `holder_known_context`, `candidate_generation`, `local_planning`, `proposal_construction`, `debug_quarantine` | `0036-P0-06-ACCEPT-REJECT-EVENT-PATH`, `0036-P0-06-HOLDER-KNOWN-NO-FALLBACK`, `0036-P0-06-ACTOR-VISIBLE-DEBUG-NEGATIVES` | pass |
| `P0-07` debug surfaces are non-diegetic and cannot feed embodied/world surfaces | `debug_quarantine`, `view_model`, `holder_known_context`, `tui_input_binding`, `test_oracle` | `0036-P0-07-CARRIER-CENSUS`, `0036-P0-07-DEBUG-QUARANTINE-NEGATIVES`, `0036-P0-07-OBSERVER-ONLY-ROWS` | pass |
| `P0-08` golden fixtures include adversarial hidden-truth, no-human, possession, replay, view-model, content-validation, and direct-dispatch rejection cases | `fixture_contract`, `content_schema`, `content_validation`, `holder_known_context`, `test_oracle`, `view_model`, `debug_quarantine`, `replay` | `0036-P0-08-POSITIVE-SEMANTIC-WITNESSES`, `0036-P0-08-NEGATIVE-INTENDED-LAYERS`, `0036-P0-08-REPLAY-SCHEMA-CONTENT` | pass |
| `P0-09` failure diagnostics name responsible layer | `doctrine`, `content_schema`, `content_validation`, `fixture_contract`, `holder_known_context`, `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `event_append`, `event_application`, `projection`, `replay`, `view_model`, `debug_quarantine`, `tui_input_binding`, `test_oracle` | pending gate evidence from `0036P0CERPOS0008-010` | pending |
| `P0-10` archived specs and tickets are cited only as history | `doctrine`, `documentation status`, `test_oracle` | pending gate evidence from `0036P0CERPOS0008-011` | pending |

## Gate Evidence Sections

### P0-01 - Proposal, Validation, Event, Projection, Replay

Status: pass

Evidence item ID: `0036-P0-01-PIPELINE-WITNESS`
Requirement IDs: `P0-01`
Evidence status: pass
Fingerprint scope: parsed semantic content
Evidence summary: Production action pipeline witness in `crates/tracewake-core/src/actions/pipeline.rs` appends candidate events to the append-only log, applies the appended event stream through `EventApplicationContext`, and rejects at typed stages `event_append` / `event_application` if append or application fails. Positive behavior coverage came from `cargo test -p tracewake-core --test golden_scenarios` (16 passed), `cargo test -p tracewake-content --test golden_fixtures_run` (40 passed), and `cargo test -p tracewake-core --test acceptance_gates` (12 passed).
Path under test and behavior witness:
- path under test: `actions::run_pipeline`, `EventLog::append`, `apply_event_stream`, `replay::run_replay`, `replay::rebuild_projection`.
- command, event, trigger, emitter, or scheduler entry that exercised it: `accepted_actions_append_versioned_events`, `sleep_eat_work_fixture_logs_need_effects_and_replays`, `ordinary_workday_fixture_moves_before_work_completion`, `no_human_day_real_run_replays_metrics_and_trace_projection`, and `no_human_day_runner_smoke_uses_no_controller_and_pipeline_events`.
- responsible layer: `proposal_construction`, `action_validation`, `event_append`, `event_application`, `projection`, `replay`.
- accepted/rejected action or validation stage witnessed: accepted take/place/sleep/eat/work/no-human ordinary actions append versioned events before application; rejected direct/invalid cases return structured rejection reports.
- live negative, mutation-style failure, or reason no negative is applicable: direct-dispatch and append/application bypass negatives passed through `acceptance_gates`, `anti_regression_guards`, and `event_schema_replay_gates`.
- checked facts or invariants the witness supports: `INV-009`, `INV-011`, and `INV-018`.
Replay/provenance ancestry:
- event-log segment or event identifiers: test-created `EventLog` sequences for take/place, no-human day, and fixture runs.
- replay artifact or serialized-log reference: canonical event-log serialization and replay rebuild in `golden_scenarios::replay_checksum_matches`, `golden_scenarios::replay_detects_missing_or_reordered_event`, and `golden_fixtures_run::no_human_day_real_run_replays_metrics_and_trace_projection`.
- seed, randomness, content version, or ruleset version where applicable: fixture manifest IDs and content fingerprints from `tracewake-content` fixture loading; no unscoped random source used by these deterministic witnesses.
- extraction/projection version for derived evidence: projection and agent-state rebuilds from the event log under current crate code.
- source provenance for any claim crossing from artifact to semantic behavior: commands above plus source seams read in this ticket.
Sampling/exhaustiveness scope: sampled production-path witness across golden/core/content/no-human paths plus exhaustive lock-layer scans from `anti_regression_guards` for direct dispatch / event append bypass classes.
Pending or historical handling: historical tickets/specs not used for this pass claim.
Certification use: counted as certifying pass for `P0-01` only.

Evidence item ID: `0036-P0-01-REPLAY-CHECKSUM`
Requirement IDs: `P0-01`
Evidence status: pass
Fingerprint scope: projection checksum
Evidence summary: `cargo test -p tracewake-core --test event_schema_replay_gates` passed 17 tests and `cargo test -p tracewake-core --test golden_scenarios` passed replay checksum tests. `golden_scenarios::replay_checksum_matches` computes the live physical checksum after a world action, replays from the serialized log, and asserts `matches_expected`; `replay_detects_missing_or_reordered_event` proves a missing/reordered event fails. `golden_fixtures_run::no_human_day_real_run_replays_metrics_and_trace_projection` computes live physical/agent checksums, rebuilds projection, serializes/deserializes the log, and compares no-human metrics from live and replayed logs.
Path under test and behavior witness:
- path under test: `tracewake_core::replay::{run_replay, rebuild_projection}`, `compute_physical_checksum`, `compute_agent_state_checksum`.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test event_schema_replay_gates`; `cargo test -p tracewake-core --test golden_scenarios`; `cargo test -p tracewake-content --test golden_fixtures_run`.
- responsible layer: `replay`, `projection`, `event_application`.
- accepted/rejected action or validation stage witnessed: live/replay checksum matches accepted for valid logs; missing/reordered/tampered logs rejected.
- live negative, mutation-style failure, or reason no negative is applicable: tamper and missing-event tests fail closed.
- checked facts or invariants the witness supports: deterministic replay (`INV-018`) and no current-state-only authority (`INV-011`).
Replay/provenance ancestry:
- event-log segment or event identifiers: canonical logs produced by action/no-human/fixture runs.
- replay artifact or serialized-log reference: `EventLog::serialize_canonical` and `EventLog::deserialize_canonical` in fixture replay tests.
- seed, randomness, content version, or ruleset version where applicable: fixture manifest and content fingerprint evidence from `golden_fixtures_run`.
- extraction/projection version for derived evidence: current replay/projection code under `crates/tracewake-core/src/replay` and `crates/tracewake-core/src/projections.rs`.
- source provenance for any claim crossing from artifact to semantic behavior: test source lines inspected for live checksum, replay checksum, and tamper assertions.
Sampling/exhaustiveness scope: sampled behavior witness plus replay lock-layer suite.
Pending or historical handling: no divergence recorded for positive witnesses; tamper negatives deliberately diverge.
Certification use: counted as certifying pass for `P0-01` only.

Evidence item ID: `0036-P0-01-NO-DIRECT-NEGATIVES`
Requirement IDs: `P0-01`
Evidence status: pass
Fingerprint scope: command transcript
Evidence summary: Direct-dispatch and bypass negatives passed: `cargo test -p tracewake-core --test acceptance_gates` (12 passed), `cargo test -p tracewake-core --test hidden_truth_gates` (13 passed), and `cargo test -p tracewake-core --test anti_regression_guards` (80 passed). Relevant named tests include `human_and_nonhuman_proposals_share_validation_path`, `sleep_proposals_share_pipeline_across_human_and_nonhuman_origins`, `no_human_day_runner_smoke_uses_no_controller_and_pipeline_events`, `scheduler_never_direct_dispatches_primitive_action`, `event_apply_remains_only_post_seed_mutation_path`, and `no_direct_apply_event_outside_event_replay_or_pipeline`.
Path under test and behavior witness:
- path under test: controller/proposal/pipeline/action registry/scheduler guard seams.
- command, event, trigger, emitter, or scheduler entry that exercised it: acceptance/hidden-truth/anti-regression guard tests listed above.
- responsible layer: `proposal_construction`, `action_validation`, `event_append`, `event_application`, `scheduler`, `test_oracle`.
- accepted/rejected action or validation stage witnessed: direct-dispatch shapes and bypasses fail at guard/validation layers; ordinary human/nonhuman actions share validation.
- live negative, mutation-style failure, or reason no negative is applicable: static and runtime lock-layer negatives passed.
- checked facts or invariants the witness supports: every world mutation counted for this line has proposal/pipeline/event/replay ancestry.
Replay/provenance ancestry: command transcript only for this negative row; positive replay ancestry is in `0036-P0-01-REPLAY-CHECKSUM`.
Sampling/exhaustiveness scope: static guard coverage is broad over production source classes; runtime behavior remains sampled by named tests.
Pending or historical handling: mutation baseline remains pending under ticket `-001`; this row does not claim mutation completion.
Certification use: counted as certifying pass for `P0-01` direct-dispatch negative evidence.

### P0-02 - Actor-Known Contexts And Provenance

Status: pass

Evidence item ID: `0036-P0-02-ACTOR-KNOWN-PROVENANCE`
Requirement IDs: `P0-02`
Evidence status: pass
Fingerprint scope: context hash/frontier
Evidence summary: `ActorKnownPlanningContext` derives hidden-truth audit from actor-known facts rather than stored booleans, and `ActorDecisionTransaction::run` returns a typed stuck diagnostic before method selection when `selection.trace.hidden_truth_audit_result.actor_known_only` is false. `cargo test -p tracewake-core --test hidden_truth_gates` passed 13 tests, including `actor_known_context_unforgeable_from_truth`, `debug_truth_never_enters_holder_known_context_hash`, and `epistemic_context_projection_and_records_remain_sealed`.
Path under test and behavior witness:
- path under test: `agent/actor_known.rs`, `agent/transaction.rs`, `agent/decision.rs`, `agent/planner.rs`, `agent/htn.rs`, `agent/no_human_surface.rs`.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test hidden_truth_gates`; `cargo test -p tracewake-content --test golden_fixtures_run`; focused fixture loader test `seeded_food_source_unknown_to_all_actors_omits_seed_belief_and_actor_known_food`.
- responsible layer: `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`.
- accepted/rejected action or validation stage witnessed: autonomous proposals proceed only from actor-known context; unproven hidden facts fail as context/provenance errors before becoming proposal targets.
- live negative, mutation-style failure, or reason no negative is applicable: unproven facts, debug omniscience, hidden food, hidden route, and missing workplace provenance negatives passed.
- checked facts or invariants the witness supports: `INV-024`, `INV-099`, `INV-100`, `INV-101`, and `INV-102`.
Replay/provenance ancestry:
- event-log segment or event identifiers: no-human observation and workplace fixtures cite log events for actor-known facts.
- replay artifact or serialized-log reference: `golden_fixtures_run::no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash` rebuilds decision context hashes from the log.
- seed, randomness, content version, or ruleset version where applicable: deterministic fixture manifest load; no random source used for this evidence.
- extraction/projection version for derived evidence: actor-known contexts are built from `EpistemicProjection` / no-human surface builders in current source.
- source provenance for any claim crossing from artifact to semantic behavior: source snippets inspected for hidden-truth audit and transaction rejection path.
Sampling/exhaustiveness scope: sampled golden/no-human/hidden-truth fixtures plus hidden-truth lock-layer suite.
Pending or historical handling: mutation baseline remains pending; no historical evidence counted as pass.
Certification use: counted as certifying pass for `P0-02` only.

Evidence item ID: `0036-P0-02-HIDDEN-TRUTH-NEGATIVES`
Requirement IDs: `P0-02`
Evidence status: pass
Fingerprint scope: command transcript
Evidence summary: Hidden-truth and provenance negatives passed. `hidden_truth_gates` proved hidden closed-container food is absent from actor-known food sources, hidden route edges are absent from route planning, debug truth does not enter context hashes, and workplace planning requires assignment/observation provenance. `golden_fixtures_run` passed the frozen fixture corpus containing `forbidden_provenance_input_fails_closed_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`, `hidden_food_closed_container_001`, `hidden_route_edge_001`, `no_human_known_workplace_requires_provenance_001`, `no_human_observation_facts_cite_log_events_001`, and `no_human_unseen_workplace_assignment_does_not_plan_work_001`.
Path under test and behavior witness:
- path under test: planner/local planning, actor-known context derivation, no-human actor-known surface construction.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test hidden_truth_gates`; `cargo test -p tracewake-content --test golden_fixtures_run`; `cargo test -p tracewake-core --test negative_fixture_runner`.
- responsible layer: `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `test_oracle`.
- accepted/rejected action or validation stage witnessed: planner accepts visible actor-known food but rejects hidden food with `food source is not actor-known`; route planning rejects hidden edge; workplace planning rejects hidden workplace without provenance.
- live negative, mutation-style failure, or reason no negative is applicable: banned-word-independent hidden-truth audit, forbidden provenance inputs, and fixture negative registration all passed.
- checked facts or invariants the witness supports: autonomous decisions do not use validation truth, raw fixture truth, debug panels, display strings, or hidden-truth comparison rows.
Replay/provenance ancestry:
- event-log segment or event identifiers: no-human observation/workplace fixture tests require log-event evidence for actor-known inputs.
- replay artifact or serialized-log reference: context hash replay row below.
- seed, randomness, content version, or ruleset version where applicable: deterministic fixtures only.
- extraction/projection version for derived evidence: current `EpistemicProjection` and `ActorKnownPlanningContext` source.
- source provenance for any claim crossing from artifact to semantic behavior: fixture/test names and inspected source.
Sampling/exhaustiveness scope: fixture corpus is sampled by behavior but covers all named P0-02 positive and negative fixture families from the ticket.
Pending or historical handling: no archive history used as certification evidence.
Certification use: counted as certifying pass for `P0-02` hidden-truth negative evidence.

Evidence item ID: `0036-P0-02-CONTEXT-HASH-REPLAY`
Requirement IDs: `P0-02`
Evidence status: pass
Fingerprint scope: context hash/frontier
Evidence summary: `cargo test -p tracewake-content --test golden_fixtures_run` passed `no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`, which runs a no-human fixture, gathers decision trace records, and asserts `rebuild_decision_context_hashes` has no failures. `fixtures_load::stale_workplace_notice_superseded_by_newer_001` passed, adding the spec's hedged freshness check.
Path under test and behavior witness:
- path under test: decision trace records, context hash rebuild, actor-known projection freshness.
- command, event, trigger, emitter, or scheduler entry that exercised it: full `golden_fixtures_run` plus focused `stale_workplace_notice_superseded_by_newer_001` loader test.
- responsible layer: `holder_known_context`, `projection`, `replay`.
- accepted/rejected action or validation stage witnessed: decision traces retain actor-known inputs and replay-rebuildable context hashes; stale workplace notice handling remains load-validated.
- live negative, mutation-style failure, or reason no negative is applicable: context hash tamper/failure checks in the golden suite passed.
- checked facts or invariants the witness supports: replay-sufficient provenance for selected goals/plans and freshness/staleness handling.
Replay/provenance ancestry:
- event-log segment or event identifiers: no-human fixture log events are collected into a `BTreeSet` and checked against decision trace actor-known inputs.
- replay artifact or serialized-log reference: `rebuild_decision_context_hashes` over initial state, initial agent state, and log.
- seed, randomness, content version, or ruleset version where applicable: fixture manifest deterministic load.
- extraction/projection version for derived evidence: current context-hash rebuild path.
- source provenance for any claim crossing from artifact to semantic behavior: inspected test source and passing command transcript.
Sampling/exhaustiveness scope: sampled no-human/provenance fixtures plus load-time freshness check.
Pending or historical handling: none.
Certification use: counted as certifying pass for `P0-02` context hash/replay evidence.

### P0-03 - Human-Origin Ordinary Actor Rules

Status: pass

Evidence item ID: `0036-P0-03-HUMAN-AUTONOMOUS-PARITY`
Requirement IDs: `P0-03`
Evidence status: pass
Fingerprint scope: command transcript
Evidence summary: Human-origin and autonomous-origin ordinary action witnesses share the same proposal validation and event append path. `cargo test -p tracewake-core --test acceptance_gates` passed 12 tests, including `human_and_nonhuman_proposals_share_validation_path`, `sleep_proposals_share_pipeline_across_human_and_nonhuman_origins`, and `no_human_day_runner_smoke_uses_no_controller_and_pipeline_events`. The named witnesses compare human/test and scheduler-origin proposals for accepted status, action IDs, target IDs, appended-event counts, and sleep event kind; the no-human smoke confirms autonomous execution emits pipeline events without a controller.
Path under test and behavior witness:
- path under test: `actions::Proposal`, `actions::run_pipeline`, action validation, `EventLog::append`, no-human scheduler runner.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test acceptance_gates`.
- responsible layer: `proposal_construction`, `action_validation`, `event_append`, `scheduler`.
- accepted/rejected action or validation stage witnessed: ordinary take and sleep proposals accept through the same pipeline whether produced by a human/test origin or scheduler origin; no-human run produces ordinary pipeline events without controller authority.
- live negative, mutation-style failure, or reason no negative is applicable: direct-dispatch negatives remain under `0036-P0-01-NO-DIRECT-NEGATIVES`; P0-03 adds origin-parity witnesses for human/autonomous action rules.
- checked facts or invariants the witness supports: `INV-005`, `INV-007`, `INV-008`, and `INV-043`.
Replay/provenance ancestry: command transcript and event append comparisons from `acceptance_gates`; replay coverage is counted under P0-01 and P0-02 rather than duplicated here.
Sampling/exhaustiveness scope: sampled ordinary take/sleep/no-human action witnesses, paired with TUI and fixture coverage below.
Pending or historical handling: mutation baseline remains pending under ticket `-001`; no mutation result is counted for P0-03.
Certification use: counted as certifying pass for `P0-03` origin-parity evidence.

Evidence item ID: `0036-P0-03-TUI-BINDING-VIEWMODEL`
Requirement IDs: `P0-03`
Evidence status: pass
Fingerprint scope: parsed semantic content
Evidence summary: TUI-origin commands bind to the current embodied actor and current view model before proposal construction. `proposal_from_semantic_action_entry` rejects human semantic-action construction without a current embodied view source context, and records the source view model ID, holder-known context ID/hash/frontier, actor ID, semantic action ID, and provenance ancestry on the proposal. `cargo test -p tracewake-tui --test tui_acceptance` passed 11 tests, `cargo test -p tracewake-tui --test tui_seam_conformance` passed 2 tests, `cargo test -p tracewake-tui --test command_loop_session` passed 7 tests, `cargo test -p tracewake-tui --test embodied_flow` passed 6 tests, and `cargo test -p tracewake-content --test golden_fixtures_run` passed 40 tests.
Path under test and behavior witness:
- path under test: `tracewake-tui` binding/render/submit loop, `build_embodied_view_model`, `proposal_from_current_view_semantic_action`, `proposal_from_semantic_action_entry`, possession and view-model fixtures.
- command, event, trigger, emitter, or scheduler entry that exercised it: `tui_selects_semantic_action_id_not_menu_index`, `numeric_selection_executes_stable_semantic_action_id`, `wait_command_executes_current_view_wait_action`, `bind_render_submit_rerender_and_show_why_not`, `phase3a_possess_continue_and_debug_transcript_is_deterministic`, `positive_proof_fixtures_emit_typed_artifacts_first`, and `possession_fixture_preserves_intention_needs_and_can_continue`.
- responsible layer: `tui_input_binding`, `view_model`, `proposal_construction`, `action_validation`, `event_append`.
- accepted/rejected action or validation stage witnessed: numeric selection and typed `wait` resolve to current-view semantic IDs; possessed `actor_mara` executes an ordinary `take` through a semantic action and later sees ordinary follow-up actions; continue-routine possession fixture preserves ordinary actor state and event ancestry.
- live negative, mutation-style failure, or reason no negative is applicable: stale current-view and direct-dispatch negatives are recorded in `0036-P0-03-DEBUG-PRIVILEGE-NEGATIVES`.
- checked facts or invariants the witness supports: the human command can only act through the possessed actor's current embodied view and holder-known context; display ordering and menu indices are not semantic authority.
Replay/provenance ancestry:
- event-log segment or event identifiers: TUI acceptance artifacts record non-empty event IDs for ordinary accepted semantic actions.
- replay artifact or serialized-log reference: `golden_fixtures_run` replay fixtures for possession/non-reset and no-human context; checksum-specific rows remain owned by P0-01/P0-02/P0-04.
- seed, randomness, content version, or ruleset version where applicable: deterministic fixture manifest and frozen golden fixture fingerprints.
- extraction/projection version for derived evidence: current `tracewake-core` embodied view-model and TUI submit code.
- source provenance for any claim crossing from artifact to semantic behavior: inspected source plus passing commands listed above.
Sampling/exhaustiveness scope: sampled fixture and TUI transcript coverage over possession parity, current-view semantic actions, wait/open/take/continue, and embodied why-not rendering.
Pending or historical handling: none for this P0-03 evidence item.
Certification use: counted as certifying pass for `P0-03` TUI binding and view-model evidence.

Evidence item ID: `0036-P0-03-DEBUG-PRIVILEGE-NEGATIVES`
Requirement IDs: `P0-03`
Evidence status: pass
Fingerprint scope: command transcript
Evidence summary: Debug/player authority remains quarantined from possessed actor affordances and human command semantics. `cargo test -p tracewake-tui --test adversarial_gates` passed 15 tests, including `adversarial_gates_debug_truth_does_not_enter_actor_surfaces`, `debug_panel_does_not_change_embodied_affordances`, `adversarial_gates_forged_privileged_semantic_id_is_not_current_action`, `adversarial_gates_public_tui_boundary_rejects_direct_dispatch_shape`, `debug_command_strings_are_not_embodied_commands`, `adversarial_gates_possession_rebind_does_not_transfer_notebook_or_debug_truth`, and `tui_current_view_submission_rejects_stale_selection`. `tui_acceptance` also passed `leakage_debug_truth_does_not_enter_embodied_view`, `debug_truth_does_not_enter_embodied_view`, and `phase3a_debug_surfaces_render_deterministically_and_read_only`.
Path under test and behavior witness:
- path under test: debug panel rendering, TUI command loop, current-view semantic action submission, possession rebind.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-tui --test adversarial_gates`; `cargo test -p tracewake-tui --test tui_acceptance`; `cargo test -p tracewake-tui --test command_loop_session`.
- responsible layer: `debug_quarantine`, `tui_input_binding`, `view_model`, `proposal_construction`, `action_validation`.
- accepted/rejected action or validation stage witnessed: forged privileged semantic IDs, stale semantic IDs, direct dispatch shapes, hidden-food semantic IDs, and debug command strings fail as non-current/non-embodied commands; debug panel reads leave current view, checksums, event counts, and embodied affordance sets unchanged.
- live negative, mutation-style failure, or reason no negative is applicable: live adversarial negative suite passed.
- checked facts or invariants the witness supports: possession imports no player/debug omniscience, no fixture truth, and no direct event-append authority into the possessed actor.
Replay/provenance ancestry: command transcript and current-view/checksum comparisons; non-diegetic debug panels are observer-only and not counted as world-state proof.
Sampling/exhaustiveness scope: sampled debug, possession-rebind, direct-dispatch, stale-view, forged-semantic, and command-loop negatives.
Pending or historical handling: debug observer rows remain non-diegetic; full debug carrier census is owned by P0-07 and remains pending outside this P0-03 evidence item.
Certification use: counted as certifying pass for `P0-03` debug/player privilege negative evidence.

### P0-04 - Possession Non-Reset

Status: pass

Evidence item ID: `0036-P0-04-PREPOST-STATE-FINGERPRINTS`
Requirement IDs: `P0-04`
Evidence status: pass
Fingerprint scope: agent-state checksum / parsed semantic content
Evidence summary: Possession bind/detach changes control routing and emits controller events, but does not rewrite the possessed actor's needs, intentions, routines, or agent-state carriers. `cargo test -p tracewake-content --test golden_fixtures_run` passed 40 tests, including `possession_fixture_preserves_intention_needs_and_can_continue`, `continue_routine_tamper_kind_flip_poisons_replay`, and `continue_routine_tamper_reason_rewrite_poisons_replay`. The possession fixture asserts `actor_mara` starts with active intention `intention_mara_work` and routine execution `routine_exec_mara_work`, snapshots `AgentState`, attaches and detaches `controller_human`, then asserts agent state is unchanged. The subsequent ordinary `continue_routine` proposal emits `ContinueRoutineProposed` and leaves intentions, routine executions, and needs equal to the pre-action snapshot while recording one continue-routine arbitration.
Path under test and behavior witness:
- path under test: `ControllerBindings::attach`, `ControllerBindings::detach`, `continue_routine` action path, `AgentState` needs/intentions/routine executions, replay checksums for possession continuation.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-content --test golden_fixtures_run`.
- responsible layer: `tui_input_binding`, `intention_lifecycle`, `projection`, `event_append`, `replay`.
- accepted/rejected action or validation stage witnessed: bind/detach emits controller attachment events without agent-state mutation; continuation proceeds as an ordinary eventful action with replay-detectable ancestry.
- live negative, mutation-style failure, or reason no negative is applicable: tampering the continue-routine event kind or reason poisons replay and agent checksums.
- checked facts or invariants the witness supports: `INV-006`, `INV-108`, and `INV-094`.
Replay/provenance ancestry:
- event-log segment or event identifiers: `ControllerAttached`, `ControllerDetached`, and `ContinueRoutineProposed` in the possession fixture.
- replay artifact or serialized-log reference: `continue_routine_tamper_kind_flip_poisons_replay` and `continue_routine_tamper_reason_rewrite_poisons_replay` compare live physical/agent checksums against tampered replay.
- seed, randomness, content version, or ruleset version where applicable: deterministic `possession_does_not_reset_intention_001` fixture package under `content_v1`.
- extraction/projection version for derived evidence: current core agent-state and replay code.
- source provenance for any claim crossing from artifact to semantic behavior: inspected fixture-run test source plus passing command transcript.
Sampling/exhaustiveness scope: sampled possession non-reset fixture over needs, active intention, routine execution, controller bind/detach, ordinary continuation, and replay tamper negatives.
Pending or historical handling: mutation baseline remains pending under ticket `-001`; no mutation result is counted for P0-04.
Certification use: counted as certifying pass for `P0-04` pre/post state evidence.

Evidence item ID: `0036-P0-04-HOLDER-KNOWN-VIEW-FILTERING`
Requirement IDs: `P0-04`
Evidence status: pass
Fingerprint scope: context hash/frontier
Evidence summary: Actor-visible view and holder-known context stay actor-filtered across possession and do not refresh hidden world facts. `cargo test -p tracewake-content --test fixtures_load` passed 32 tests, including deterministic load/validation coverage for `possession_parity_001`, `view_filtering_001`, and all registered fixtures. `cargo test -p tracewake-core --test hidden_truth_gates` passed 13 tests, including `debug_truth_never_enters_holder_known_context_hash`, `embodied_affordances_exclude_hidden_food_in_closed_container`, and `epistemic_context_projection_and_records_remain_sealed`. `cargo test -p tracewake-tui --test tui_acceptance` passed 11 tests, including `phase3a_possess_continue_and_debug_transcript_is_deterministic` and `positive_proof_fixtures_emit_typed_artifacts_first`.
Path under test and behavior witness:
- path under test: `current_place_knowledge_context`, `build_embodied_view_model`, holder-known context hash/source summary, notebook view, fixture load validation.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-content --test fixtures_load`; `cargo test -p tracewake-core --test hidden_truth_gates`; `cargo test -p tracewake-tui --test tui_acceptance`.
- responsible layer: `holder_known_context`, `view_model`, `projection`, `tui_input_binding`.
- accepted/rejected action or validation stage witnessed: possessed actor context and actor-visible affordances derive from holder-known source IDs/frontier; hidden food/debug facts do not become semantic action targets or context source summaries.
- live negative, mutation-style failure, or reason no negative is applicable: debug truth report leaves holder-known context hash/source summary unchanged and excludes hidden food from semantic action targets.
- checked facts or invariants the witness supports: possession may change control routing only; it may not create, refresh, delete, or reclassify beliefs, known facts, or affordances.
Replay/provenance ancestry: holder-known context hash/source summary and fixture fingerprint evidence; detailed decision-context replay rows remain counted under P0-02.
Sampling/exhaustiveness scope: sampled fixture corpus plus hidden-truth lock-layer and TUI proof artifacts over possession, view filtering, hidden food omission, raw assignment omission, and deterministic transcripts.
Pending or historical handling: none for this P0-04 evidence item.
Certification use: counted as certifying pass for `P0-04` holder-known and view-filtering evidence.

Evidence item ID: `0036-P0-04-DEBUG-REBIND-NEGATIVES`
Requirement IDs: `P0-04`
Evidence status: pass
Fingerprint scope: command transcript
Evidence summary: Debug attach/rebind remains non-diegetic and does not transfer one actor's notebook, rejection, needs, intention, or debug truth into another actor's possessed view. `cargo test -p tracewake-tui --test adversarial_gates` passed 15 tests, including `adversarial_gates_possession_rebind_does_not_transfer_notebook_or_debug_truth`, `adversarial_gates_possession_rebind_does_not_transfer_rejection_why_not`, `debug_panel_does_not_change_embodied_affordances`, and `adversarial_gates_debug_truth_does_not_enter_actor_surfaces`. `cargo test -p tracewake-tui --test tui_acceptance` passed debug panel determinism and possession transcript tests.
Path under test and behavior witness:
- path under test: `TuiApp::bind_actor`, `TuiApp::bind_debug_actor`, current embodied view, notebook view, debug panels, holder-known context fields.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-tui --test adversarial_gates`; `cargo test -p tracewake-tui --test tui_acceptance`.
- responsible layer: `debug_quarantine`, `tui_input_binding`, `holder_known_context`, `view_model`.
- accepted/rejected action or validation stage witnessed: after rebinding from `actor_tomas` to `actor_mara`, the new actor view has `actor_mara` context, empty notebook/source-bound beliefs where expected, no transferred rejection why-not, no debug text, and unchanged physical checksum.
- live negative, mutation-style failure, or reason no negative is applicable: debug item panels are explicitly non-diegetic; current actor surfaces reject debug/non-current contamination.
- checked facts or invariants the witness supports: debug attach alters no actor-visible carrier or holder-known context (`INV-107`) and possession remains cognition-neutral.
Replay/provenance ancestry: command transcript and checksum/current-view comparisons; debug rows remain observer-only and non-certifying for world-state facts.
Sampling/exhaustiveness scope: sampled possession rebind, debug panel, rejection why-not, notebook, needs/intention, context ID/hash, and semantic action contamination negatives.
Pending or historical handling: broad debug carrier census is owned by P0-07; this item counts only the P0-04 possession-boundary negatives.
Certification use: counted as certifying pass for `P0-04` debug/rebind negative evidence.

### P0-05 - Scheduler And No-Human Boundaries

Status: pass

Evidence item ID: `0036-P0-05-NOHUMAN-CAPSTONE-ANCESTRY`
Requirement IDs: `P0-05`
Evidence status: pass
Fingerprint scope: replay artifact / parsed semantic content
Evidence summary: The no-human capstone proves scheduler order, ordinary actor decision ancestry, event append ancestry, and replay equivalence for the no-human ordinary-life interval. `cargo test -p tracewake-core --test no_human_capstone` passed 1 test. The capstone asserts roster/window order, `ordinary_pipeline_events > initial_actor_count`, required events including no-human markers, need deltas, sleep/eat/move/work/wait, intentions/routine steps, and decision traces. It then asserts ordinary no-human events with process and proposal IDs carry `EventCause::Proposal`, validates decision trace/intention/routine ancestry, rejects controller/hidden-truth leakage, rebuilds physical and agent projections from the log, and compares no-human metrics after canonical log serialization/deserialization.
Path under test and behavior witness:
- path under test: `scheduler.rs::no_human::run_no_human_day`, actor decision transaction, shared action pipeline, event log, replay rebuild, no-human metrics.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test no_human_capstone`.
- responsible layer: `scheduler`, `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, `action_validation`, `event_append`, `replay`.
- accepted/rejected action or validation stage witnessed: no-human ordinary actions appear only as transaction/pipeline outputs with proposal/process ancestry; no-human markers are diagnostic/agent no-op markers, not primitive world actions.
- live negative, mutation-style failure, or reason no negative is applicable: controller/hidden-truth leakage assertions and replay/context-hash failure checks fail closed in the capstone.
- checked facts or invariants the witness supports: `INV-103`, `INV-104`, and deterministic replay (`INV-018`).
Replay/provenance ancestry:
- event-log segment or event identifiers: no-human day start/completion, ordinary action events, decision trace events, routine/intention events, and proposal/process-caused ordinary events from the capstone log.
- replay artifact or serialized-log reference: `rebuild_projection` over initial world/agent state and canonical log serialization/deserialization.
- seed, randomness, content version, or ruleset version where applicable: deterministic capstone world/agent setup, `phase3a_capstone_manifest`, `content_v1`.
- extraction/projection version for derived evidence: current no-human scheduler, replay, checksum, and metrics code.
- source provenance for any claim crossing from artifact to semantic behavior: inspected capstone test source plus passing command transcript.
Sampling/exhaustiveness scope: sampled capstone over one no-human ordinary-life interval; broader fixture and guard coverage below covers additional named corpus and shortcut classes.
Pending or historical handling: mutation baseline remains pending under ticket `-001`; no mutation result is counted for P0-05.
Certification use: counted as certifying pass for `P0-05` no-human ancestry/replay evidence.

Evidence item ID: `0036-P0-05-PASSIVE-ACCOUNTING-SINGLE-CHARGE`
Requirement IDs: `P0-05`
Evidence status: pass
Fingerprint scope: parsed semantic content
Evidence summary: Passive accounting is typed, replayable, and charged once per tick rather than used as a primitive action shortcut. `cargo test -p tracewake-content --test golden_fixtures_run` passed 40 tests, including `no_human_day_fixture_has_roster_activity_and_metrics_envelope`, `no_human_day_real_run_replays_metrics_and_trace_projection`, `wait_then_window_passive_charges_each_tick_once`, `sleep_spanning_window_boundary_charges_each_tick_once`, and `no_human_need_ledger_has_no_duplicate_regime_charges`.
Path under test and behavior witness:
- path under test: no-human fixture runner, passive need accounting, wait/sleep window handling, no-human metrics, replay checksums.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-content --test golden_fixtures_run`.
- responsible layer: `scheduler`, `event_append`, `event_application`, `projection`, `replay`.
- accepted/rejected action or validation stage witnessed: passive `NeedDeltaApplied` / `NeedThresholdCrossed` rows carry typed causes and are checked for duplicate charges; sleep spanning a no-human window does not receive passive awake deltas for slept ticks.
- live negative, mutation-style failure, or reason no negative is applicable: duplicate regime charges and tampered replay/context hashes fail closed.
- checked facts or invariants the witness supports: passive scheduler accounting is non-cognitive and does not become an actor decision or primitive action.
Replay/provenance ancestry:
- event-log segment or event identifiers: no-human fixture logs containing no-human markers, ordinary action events, need deltas, threshold crossings, and metrics rows.
- replay artifact or serialized-log reference: `no_human_day_real_run_replays_metrics_and_trace_projection` and fixture checksum/replay assertions.
- seed, randomness, content version, or ruleset version where applicable: deterministic golden fixture corpus and frozen fixture fingerprints.
- extraction/projection version for derived evidence: current no-human metrics and need-accounting code.
- source provenance for any claim crossing from artifact to semantic behavior: inspected fixture-run test source plus passing command transcript.
Sampling/exhaustiveness scope: sampled no-human/day, wait-window, sleep-window, and need-ledger fixture coverage from the golden corpus.
Pending or historical handling: no-human metrics remain derived observer evidence unless explicitly cited here for replay/accounting shape; they are not actor cognition or scheduler input.
Certification use: counted as certifying pass for `P0-05` passive-accounting evidence.

Evidence item ID: `0036-P0-05-SCHEDULER-SHORTCUT-NEGATIVES`
Requirement IDs: `P0-05`
Evidence status: pass
Fingerprint scope: command transcript / static seam scan
Evidence summary: Scheduler shortcut negatives and static guards passed. `cargo test -p tracewake-core --test anti_regression_guards` passed 80 tests, including `scheduler_never_direct_dispatches_primitive_action`, `guard_006_scheduler_has_no_direct_routine_or_need_proposal_bypass`, `guard_006_scheduler_has_no_routine_family_to_primitive_dispatch`, `guard_014_scheduler_cannot_rewrite_transaction_proposals_after_cognition`, `guard_006_scheduler_does_not_fabricate_empty_epistemic_projection`, `guard_006_duration_need_deltas_route_through_shared_emitter`, `guard_014_no_human_cognition_surface_does_not_read_raw_assignment_or_sleep_truth`, `guard_014_no_human_metrics_do_not_scan_display_text`, and `guard_011_no_human_day_runner_only_evidence`. `golden_fixtures_run` also passed the named scheduler negatives for unseen workplace assignment, workplace notice/provenance, severe safety blocker, method fallback, and scheduler wait-reason rewrite fixtures. TUI/adversarial no-human debug-only checks passed in earlier P0-03/P0-04 evidence and are not double-counted here.
Path under test and behavior witness:
- path under test: `crates/tracewake-core/src/scheduler.rs`, no-human actor-known surface builder, actor decision transaction sealed proposal handoff, need delta emitter, no-human metrics, golden fixture runner.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test anti_regression_guards`; `cargo test -p tracewake-content --test golden_fixtures_run`.
- responsible layer: `scheduler`, `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, `action_validation`, `event_append`, `test_oracle`.
- accepted/rejected action or validation stage witnessed: scheduler source must contain `ActorDecisionTransaction::run` and `run_pipeline(&mut context, &proposal)` for ordinary proposals; source guards reject routine-family-to-action mappings, direct primitive action IDs, direct need/routine proposal bypasses, fabricated empty epistemic projections, direct duration need-delta construction, and post-cognition proposal rewriting.
- live negative, mutation-style failure, or reason no negative is applicable: synthetic guard insertions fail the static scans; golden negative fixtures fail closed for hidden workplace/assignment/provenance and fallback shortcut classes.
- checked facts or invariants the witness supports: no scheduler path emits primitive actions directly from raw truth, routine labels, need thresholds, or fixture tables.
Replay/provenance ancestry: primarily static guard and command transcript; behavior replay ancestry comes from the capstone and golden fixture rows above.
Sampling/exhaustiveness scope: broad static source guards over scheduler and action-def surfaces plus sampled behavior negatives.
Pending or historical handling: mutation baseline remains pending; no survived/incomplete mutation evidence is counted as pass.
Certification use: counted as certifying pass for `P0-05` scheduler shortcut negative evidence.

### P0-06 - Validation Truth Boundary

Status: pass

Evidence item ID: `0036-P0-06-ACCEPT-REJECT-EVENT-PATH`
Requirement IDs: `P0-06`
Evidence status: pass
Fingerprint scope: command transcript / replay artifact
Evidence summary: Validation accepts, rejects, and mutates only through the proposal/pipeline/event path. `cargo test -p tracewake-core --test acceptance_gates` passed 12 tests, including structured rejection, event append ordering, phase-boundary rejection, human/scheduler shared validation, sleep shared pipeline, and live/replay agent checksum coverage. `cargo test -p tracewake-content --test golden_fixtures_run` passed 40 tests, including workplace rejection, displacement validation, interrupted sleep duration, routine blocked diagnostics, severe-safety blockers, and replay/tamper checks.
Path under test and behavior witness:
- path under test: `validate_proposal`, action defs for movement/take/place/sleep/eat/work/wait/continue-routine, `run_pipeline`, event append/application, replay rebuild.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test acceptance_gates`; `cargo test -p tracewake-content --test golden_fixtures_run`.
- responsible layer: `action_validation`, `event_append`, `event_application`, `projection`, `replay`.
- accepted/rejected action or validation stage witnessed: accepted world-affecting actions append events before authoritative application; rejected/out-of-scope/blocked actions return typed reports and do not become fallback plans; interrupted duration and displacement cases replay through event/application checks.
- live negative, mutation-style failure, or reason no negative is applicable: replay tamper and checksum negatives fail when accepted/rejected outcomes are changed.
- checked facts or invariants the witness supports: validation truth can accept/reject/mutate through events but cannot be an independent cognition or planning source.
Replay/provenance ancestry:
- event-log segment or event identifiers: accepted/rejected action events, routine/need events, and fixture replay logs from `golden_fixtures_run`.
- replay artifact or serialized-log reference: live/replay agent checksum assertions and golden fixture replay/tamper checks.
- seed, randomness, content version, or ruleset version where applicable: deterministic golden fixture corpus and current content manifests.
- extraction/projection version for derived evidence: current action pipeline, event application, projection, and replay code.
- source provenance for any claim crossing from artifact to semantic behavior: inspected tests/source plus passing command transcripts.
Sampling/exhaustiveness scope: sampled action validation positives/negatives across core acceptance gates and golden fixtures.
Pending or historical handling: mutation baseline remains pending under ticket `-001`; no mutation result is counted for P0-06.
Certification use: counted as certifying pass for `P0-06` accept/reject/event-path evidence.

Evidence item ID: `0036-P0-06-HOLDER-KNOWN-NO-FALLBACK`
Requirement IDs: `P0-06`
Evidence status: pass
Fingerprint scope: context hash/frontier
Evidence summary: Selected actions and fallback/stuck outcomes are justified by pre-validation holder-known context; validator truth does not add candidate goals, route hints, food targets, workplace facts, or hidden facts to actor-known context. `cargo test -p tracewake-core --test hidden_truth_gates` passed 13 tests, including `hidden_food_closed_container_is_not_actor_known_food_source`, `hidden_route_edge_absent_from_actor_context_blocks_route_plan`, `hidden_food_unknown_route_does_not_become_transaction_target`, `workplace_requires_assignment_or_observation_provenance`, and `debug_omniscience_facts_are_excluded_from_planner_context`. `golden_fixtures_run` passed `no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs`, `phase2a_validation_rejects_shortcut_truth_fields`, `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit`, and the scheduler/fallback negative fixtures.
Path under test and behavior witness:
- path under test: actor-known context derivation, candidate generation, goal selection trace, local planning, hidden-truth audit, validation preflight.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test hidden_truth_gates`; `cargo test -p tracewake-content --test golden_fixtures_run`.
- responsible layer: `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, `action_validation`.
- accepted/rejected action or validation stage witnessed: hidden food remains absent from known food sources and semantic/planner targets; hidden route edges are absent from route planning; planner failures report actor-known blockers rather than validation-truth fallback targets.
- live negative, mutation-style failure, or reason no negative is applicable: forbidden provenance, hidden route/food, shortcut truth fields, and method-fallback shortcut fixtures fail closed.
- checked facts or invariants the witness supports: validator truth inspects after proposal construction and never updates cognition or selected targets.
Replay/provenance ancestry: actor-known context hash/frontier and decision-trace proof rows; detailed context-hash replay is counted under P0-02.
Sampling/exhaustiveness scope: sampled hidden-food, hidden-route, workplace-provenance, shortcut-field, and fallback-negative fixtures plus hidden-truth lock-layer suite.
Pending or historical handling: none for this P0-06 evidence item.
Certification use: counted as certifying pass for `P0-06` no-fallback/no-hidden-target evidence.

Evidence item ID: `0036-P0-06-ACTOR-VISIBLE-DEBUG-NEGATIVES`
Requirement IDs: `P0-06`
Evidence status: pass
Fingerprint scope: command transcript
Evidence summary: Actor-visible validation failures expose typed, non-leaking reasons, while debug truth remains quarantined. `cargo test -p tracewake-tui --test adversarial_gates` passed 15 tests, including `adversarial_gates_why_not_actor_surface_uses_typed_non_leaking_facts`, `adversarial_gates_tui_rule_inference_cannot_apply_hidden_food_target`, `adversarial_gates_debug_truth_does_not_enter_actor_surfaces`, and `debug_panel_does_not_change_embodied_affordances`. Source review of embodied semantic-action preflight shows validation reports become disabled action availability through reason codes, actor-visible summary, and explicit provenance refs to holder-known context, validation report, and actor-visible facts; they do not add hidden facts to the actor context.
Path under test and behavior witness:
- path under test: embodied semantic-action preflight, `ActionAvailability`, current-view why-not rendering, debug reports/panels, TUI submit.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-tui --test adversarial_gates`; `cargo test -p tracewake-core --test hidden_truth_gates`.
- responsible layer: `action_validation`, `view_model`, `debug_quarantine`, `tui_input_binding`.
- accepted/rejected action or validation stage witnessed: actor-visible why-not contains typed reason codes and non-leaking facts; hidden food semantic IDs are not current actions and do not change checksums; debug facts do not enter holder-known context hashes or semantic action targets.
- live negative, mutation-style failure, or reason no negative is applicable: forged/hidden semantic actions fail as non-current; debug reports are debug-only and leave actor surfaces unchanged.
- checked facts or invariants the witness supports: validation/debug truth never becomes actor-visible hidden target, clue, suspect, workplace fact, or belief without modeled information flow.
Replay/provenance ancestry: command transcript and checksum/current-view comparisons; debug rows are observer-only and not counted as world-state facts.
Sampling/exhaustiveness scope: sampled TUI validation why-not, hidden target, debug quarantine, and current-view contamination negatives.
Pending or historical handling: broad debug carrier census is owned by P0-07; this row counts only validation-boundary debug/actor-visible negatives.
Certification use: counted as certifying pass for `P0-06` actor-visible/debug negative evidence.

### P0-07 - Debug Quarantine

Status: pass

Evidence item ID: `0036-P0-07-CARRIER-CENSUS`
Requirement IDs: `P0-07`
Evidence status: pass
Fingerprint scope: parsed semantic content and typed surface boundaries
Evidence summary: The debug/embodied-view corpus keeps actor-visible surfaces separate from debug-only and observer-only surfaces. `cargo test -p tracewake-content --test golden_fixtures_run` passed 40 tests, including the registered `debug_attach_001`, `debug_omniscience_excluded_001`, `view_filtering_001`, `view_model_local_actions_001`, `embodied_view_omits_raw_assignment_without_context_001`, `embodied_view_omits_unobserved_food_at_open_place_001`, and `embodied_view_omits_unknown_sleep_affordance_001` fixture coverage. `cargo test -p tracewake-tui --test adversarial_gates` passed 15 tests, including `adversarial_gates_debug_truth_does_not_enter_actor_surfaces`, `debug_panel_does_not_change_embodied_affordances`, `tui_transcript_marks_debug_sections_non_diegetic`, and `adversarial_gates_no_human_operator_output_stays_debug_only`.
Path under test and behavior witness:
- path under test: debug capability/fixture attachment, embodied projection/view-model construction, current-view action submission, debug panels, and no-human/debug transcript rendering.
- command, event, trigger, emitter, or scheduler entry that exercised it: `debug_attach_001`, `debug_omniscience_excluded_001`, `view_filtering_001`, `view_model_local_actions_001`, and the embodied-view omission fixtures through `golden_fixtures_run`; adversarial TUI gates through `TuiApp::from_golden`.
- responsible layer: `debug_quarantine`, `view_model`, `holder_known_context`, `tui_input_binding`, `test_oracle`.
- accepted/rejected action or validation stage witnessed: debug capability attaches as a quarantined capability; actor-visible view-model/action rows are derived from embodied state and current source context; debug panel rows are rendered as non-diegetic observer information.
- live negative, mutation-style failure, or reason no negative is applicable: core and TUI negatives fail closed when debug truth, stale source contexts, raw assignment truth, unobserved food, unknown sleep affordances, or command strings attempt to masquerade as actor-visible knowledge or embodied commands.
- checked facts or invariants the witness supports: actor-visible rows are holder-known/context-backed; debug-only rows may have replay ancestry but are labeled observer-only and are not accepted as actor cognition, affordances, validator/scheduler/content-selection inputs, records, or institutional state.
Replay/provenance ancestry: golden fixture fingerprints and adversarial TUI current-view/source-context checks; debug rows remain observer-only and separate from modeled knowledge.
Sampling/exhaustiveness scope: representative debug attach, debug omniscience, view filtering, local action, omitted assignment, omitted hidden food, and omitted sleep-affordance fixtures plus TUI adversarial debug gates.
Pending or historical handling: this item certifies the P0-07 debug carrier boundary only; it does not certify the capstone `EMERGE-OBS` staged-abstraction declaration owned by `0036P0CERPOS0008-012`.
Certification use: counted as certifying pass for `P0-07` carrier census evidence.

Evidence item ID: `0036-P0-07-DEBUG-QUARANTINE-NEGATIVES`
Requirement IDs: `P0-07`
Evidence status: pass
Fingerprint scope: typed negative guards
Evidence summary: `cargo test -p tracewake-core --test anti_regression_guards` passed 80 tests, including `embodied_projection_never_uses_unfiltered_checked_facts_as_actor_provenance`, `diagnostics_never_assert_display_label_as_authority`, `privileged_tui_proposal_requires_current_view_source_context`, `guard_014_no_human_metrics_do_not_scan_display_text`, `guard_014_no_human_cognition_surface_does_not_read_raw_assignment_or_sleep_truth`, and `validation_report_keeps_typed_provenance_and_actor_debug_split`. `adversarial_gates` also passed the direct debug negatives: debug truth does not enter actor surfaces, the debug panel does not alter embodied affordances, possession rebind does not transfer notebook/debug truth, no-human operator output stays debug-only, and debug command strings are not embodied commands.
Path under test and behavior witness:
- path under test: actor-known projection, embodied projection, validation report provenance, TUI privileged proposal/current-view source context, no-human metrics, debug panel rendering, and adversarial TUI input binding.
- command, event, trigger, emitter, or scheduler entry that exercised it: `anti_regression_guards` static/semantic guards and `adversarial_gates` TUI flows.
- responsible layer: `holder_known_context`, `view_model`, `debug_quarantine`, `tui_input_binding`, `test_oracle`.
- accepted/rejected action or validation stage witnessed: stale/forged source contexts and direct debug command strings are rejected; hidden/debug facts are not accepted as semantic targets or actor-surface proof.
- live negative, mutation-style failure, or reason no negative is applicable: the named guards would fail if debug reports, display labels, no-human metrics, raw assignment/sleep truth, or debug panel output were used as actor provenance or test-oracle authority.
- checked facts or invariants the witness supports: debug-derived facts do not become actor cognition, embodied affordances, validators, scheduler inputs, content selectors, records, institutions, or actor-knowledge test oracles.
Replay/provenance ancestry: anti-regression source classification/provenance guards and adversarial TUI typed source-context checks.
Sampling/exhaustiveness scope: targeted carrier and oracle negatives for the debug/actor-visible split.
Pending or historical handling: mutation baseline remains pending from `0036P0CERPOS0008-001` and is not counted as P0-07 pass evidence.
Certification use: counted as certifying pass for `P0-07` debug quarantine negatives.

Evidence item ID: `0036-P0-07-OBSERVER-ONLY-ROWS`
Requirement IDs: `P0-07`
Evidence status: pass
Fingerprint scope: observer-only artifact boundary
Evidence summary: No-human metrics, debug reports, hidden-truth comparison displays, transcript debug sections, and `EMERGE-OBS` rows are treated as observer-only review artifacts rather than simulation inputs. The passing `adversarial_gates_no_human_operator_output_stays_debug_only`, `tui_transcript_marks_debug_sections_non_diegetic`, `guard_014_no_human_metrics_do_not_scan_display_text`, and `validation_report_keeps_typed_provenance_and_actor_debug_split` checks cover this boundary.
Path under test and behavior witness:
- path under test: no-human metrics/debug transcript output, validation report provenance, debug panels, and observer-only report rows.
- command, event, trigger, emitter, or scheduler entry that exercised it: adversarial TUI no-human/debug transcript gates and core anti-regression diagnostics guards.
- responsible layer: `debug_quarantine`, `test_oracle`, `holder_known_context`.
- accepted/rejected action or validation stage witnessed: observer-only rows may explain or review replay-derived behavior but do not become pass/fail thresholds, actor-visible facts, cognition inputs, scheduler inputs, validators, or content selectors.
- live negative, mutation-style failure, or reason no negative is applicable: no-human/display-text and validation/debug split guards fail if observer-only data is used as authority for modeled actor knowledge.
- checked facts or invariants the witness supports: `EMERGE-OBS` remains observer-only, non-gating, non-threshold, and quarantined from simulation inputs for the P0-07 surfaces.
Replay/provenance ancestry: observer-only rows are replay/report artifacts; actor-facing rows must still cite holder-known/source-context provenance.
Sampling/exhaustiveness scope: observer-only boundary for debug, no-human metrics, transcript debug sections, hidden-truth comparison displays, and `EMERGE-OBS` posture.
Pending or historical handling: full `EMERGE-OBS` capstone declaration remains owned by `0036P0CERPOS0008-012`; this row certifies only that P0-07 debug/observer evidence is non-diegetic.
Certification use: counted as certifying pass for `P0-07` observer-only quarantine evidence.

### P0-08 - Golden And Adversarial Fixture Corpus

Status: pass

Evidence item ID: `0036-P0-08-POSITIVE-SEMANTIC-WITNESSES`
Requirement IDs: `P0-08`
Evidence status: pass
Fingerprint scope: semantic behavior witness plus fixture fingerprint
Evidence summary: Positive golden families ran with semantic witnesses rather than byte fixtures alone. `cargo test -p tracewake-content` passed content unit tests, `fixtures_load` (32), `golden_fixtures_run` (40), `schema_conformance` (2), forbidden-content tests (20), and doctests (3). `cargo test -p tracewake-core --test golden_scenarios` passed 16 tests. The passing corpus covers hidden-truth resistance (`no_hidden_truth_planning_001`, `seeded_food_source_unknown_to_all_actors_001`), no-human ordinary life (`no_human_day_001`, `ordinary_workday_001`, `sleep_eat_work_001`), possession parity/non-reset (`possession_parity_001`, `possession_does_not_reset_intention_001`), replay (`replay_item_location_001` and replay checksum/projection tests), view-model filtering (`view_filtering_001`, `view_model_local_actions_001`), content validation, and direct-dispatch rejection behavior.
Path under test and behavior witness:
- path under test: fixture registration/loading, fixture contract validation, golden fixture runner, golden core scenarios, holder-known projection, action pipeline, no-human runner, possession binding, view model, and replay projection.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-content` and `cargo test -p tracewake-core --test golden_scenarios`.
- responsible layer: `fixture_contract`, `content_schema`, `content_validation`, `holder_known_context`, `view_model`, `replay`, `test_oracle`.
- accepted/rejected action or validation stage witnessed: ordinary hidden-truth, no-human, possession, view-model, and replay fixtures execute production behavior and assert modeled semantic outcomes, context hashes, provenance, replay/projection checksums, and stable fixture fingerprints.
- live negative, mutation-style failure, or reason no negative is applicable: paired hostile-family rows below cover the negative side; this item counts only positive semantic witnesses.
- checked facts or invariants the witness supports: certification goldens are not counted merely because serialized bytes match; each counted positive has an observed semantic behavior assertion.
Replay/provenance ancestry: fixture manifest fingerprints, golden fixture fingerprints, event-log replay checksums, and live/replay projection comparisons.
Sampling/exhaustiveness scope: representative positive families named by spec section 7.8 and ticket `0036P0CERPOS0008-009`; deeper per-gate semantics remain in P0-01 through P0-07 rows.
Pending or historical handling: mutation baseline remains pending from `0036P0CERPOS0008-001` and is not counted as P0-08 pass evidence.
Certification use: counted as certifying pass for `P0-08` positive semantic witness coverage.

Evidence item ID: `0036-P0-08-NEGATIVE-INTENDED-LAYERS`
Requirement IDs: `P0-08`
Evidence status: pass
Fingerprint scope: intended responsible-layer failure
Evidence summary: Hostile families fail closed for their intended layers rather than incidental parse, panic, or display-string behavior. `cargo test -p tracewake-core --test negative_fixture_runner` passed 5 tests, proving registered negative fixture census, clippy-ban proving fixtures, loader registration mutation guard, debug-report capability compile-fail coverage, and banned-API lints. `cargo test -p tracewake-tui --test adversarial_gates` passed 15 tests, including public TUI direct-dispatch rejection, forged/stale semantic-action rejection, hidden-food target rejection, debug command-string rejection, and typed diagnostics independent of display text. The content crate lane also passed forbidden-content and fixture-load negatives for prose-born facts, shortcut truth fields, player/script markers, malformed epistemic seeds, unknown fields, unsupported schema versions, bad references, and canonical ordering hazards.
Path under test and behavior witness:
- path under test: negative fixture registry, forbidden content validator, schema conformance, TUI adversarial boundary, direct-dispatch rejection, debug capability guard, and banned API negative fixtures.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test negative_fixture_runner`, `cargo test -p tracewake-tui --test adversarial_gates`, and `cargo test -p tracewake-content`.
- responsible layer: `fixture_contract`, `content_schema`, `content_validation`, `holder_known_context`, `debug_quarantine`, `view_model`, `tui_input_binding`, `test_oracle`, `replay`, plus the action/scheduler layer under test.
- accepted/rejected action or validation stage witnessed: hidden food, hidden routes, unproven facts, forbidden provenance, prose-born facts, debug omniscience, embodied-view omissions, scheduler wait-reason rewrite, direct dispatch, and forbidden API uses reject at typed boundaries.
- live negative, mutation-style failure, or reason no negative is applicable: each named negative has a proving test or fixture-run assertion that would fail if the protected shortcut were accepted.
- checked facts or invariants the witness supports: the corpus is not friendly-only and adversarial cases fail for responsible-layer reasons that match the shortcut under test.
Replay/provenance ancestry: negative fixture registry census, TUI source-context checks, content validation diagnostics, and compile/lint negative fixture outputs.
Sampling/exhaustiveness scope: representative hostile families named by spec section 7.8, including hidden-truth, no-human, possession/view-model, direct-dispatch, content-validation, debug-quarantine, scheduler, and forbidden-API classes.
Pending or historical handling: this row does not claim exhaustive mutation coverage; it records live intended-layer hostile corpus coverage.
Certification use: counted as certifying pass for `P0-08` negative intended-layer coverage.

Evidence item ID: `0036-P0-08-REPLAY-SCHEMA-CONTENT`
Requirement IDs: `P0-08`
Evidence status: pass
Fingerprint scope: replay/schema/content validation
Evidence summary: Replay, schema, and content-validation coverage paired byte stability with semantic checks. `cargo test -p tracewake-core --test event_schema_replay_gates` passed 17 tests, including duplicate need tick, duplicate duration terminal, malformed elapsed ticks, missing access/routine/intention events, unsupported schema versions, forged payload schema versions, non-world checksum protection, stream mismatch rejection, typed no-human metrics rebuild, and no-human replay checksum equivalence. `cargo test -p tracewake-content` passed schema conformance and canonical serialization/load tests.
Path under test and behavior witness:
- path under test: event schema registry, append/replay validation, replay projection rebuild, fixture manifest/load/serialization, schema conformance, and content validation.
- command, event, trigger, emitter, or scheduler entry that exercised it: `cargo test -p tracewake-core --test event_schema_replay_gates` and `cargo test -p tracewake-content`.
- responsible layer: `content_schema`, `content_validation`, `fixture_contract`, `replay`, `test_oracle`.
- accepted/rejected action or validation stage witnessed: live and replay paths reject malformed/missing/unsupported event payloads; content loading rejects unsupported schemas, bad references, unknown fields, script/player/prose-born shortcuts, and nondeterministic ordering hazards.
- live negative, mutation-style failure, or reason no negative is applicable: event-schema replay gates and content conformance tests would fail if golden bytes were accepted without corresponding semantic replay or validation behavior.
- checked facts or invariants the witness supports: stable serialization and fixture fingerprints are paired with replay/projection and validation semantics.
Replay/provenance ancestry: event-log replay rebuild checksums, manifest fingerprints, fixture fingerprints, canonical serialization round trips, and schema conformance maps.
Sampling/exhaustiveness scope: replay/schema/content-validation subset of the P0-08 corpus, complementing the positive and hostile family rows above.
Pending or historical handling: no historical-only evidence is counted; archived fixture/spec history remains lineage only.
Certification use: counted as certifying pass for `P0-08` replay/schema/content evidence.

### P0-09 - Responsible-Layer Diagnostics

Status: pending

Evidence will be filled by `0036P0CERPOS0008-010`.

### P0-10 - Historical-Only Archive Use

Status: pending

Evidence will be filled by `0036P0CERPOS0008-011`.

## Sampling And Exhaustiveness

| Evidence class | Status | Scope |
|---|---|---|
| Static seam scans | sampled | Bounded source scans ran for append/apply entry points, hidden-truth/debug terms, diagnostic typing, controller/player/debug terms, and nondeterminism/collection APIs. The scans are review evidence; the certifying lock-layer is the named guard tests. |
| Golden behavior witnesses | sampled | `golden_fixtures_run`, `golden_scenarios`, and `event_schema_replay_gates` passed. Per-gate semantic witness rows remain owned by gate tickets `-002`...`-011`. |
| Property/generative evidence | sampled | `generative_lock` passed as part of `cargo test --workspace --locked`; explicit seed/accounting rows remain capstone-owned. |
| TUI transcript evidence | sampled | `adversarial_gates` passed separately and all TUI tests passed inside `cargo test --workspace --locked`; exhaustive carrier census remains gate-owned. |
| Mutation testing | pending | `cargo-mutants 27.1.0` is installed and the scheduled CI baseline command was attempted locally. The run found 1128 mutants and passed the unmutated baseline, then was stopped after emitting one missed mutant because the configured baseline is too large for this interactive ticket turn. Full scheduled baseline remains pending and is not counted as pass. |

## Pending And Historical Evidence

Archived specs, tickets, and reports are `historical` only. They may explain
lineage and prior remediation context, but they do not supply live P0-CERT pass
evidence. Unavailable command runs, unsupported mutation execution, untriaged
survived mutants, or incomplete property/generative evidence remain `pending`
or `fail`, never pass.

Current pending item from ticket `0036P0CERPOS0008-001`: the full configured
mutation baseline did not complete locally on 2026-06-16. The local attempt
emitted one miss before interruption:
`crates/tracewake-core/src/projections.rs:336:5 replace actor_known_local_actors_for_context -> Vec<ActorId> with vec![]`.
That miss is untriaged in this artifact revision and must not be counted as a
certifying pass.

## Certification Use

No later spec may cite this artifact as `P0-CERT passed` while the verdict is
`<pending>`. If the capstone renders `P0-CERT passed`, later specs may cite this
artifact and name the certified gates consumed, but still must satisfy stricter
entry gates for `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`,
`FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, or `SECOND-PROOF-ENTRY` as applicable. If
the capstone renders `P0-CERT scoped remediation`, only remediation specs
addressing named failures are admissible until a replacement certification
artifact passes.

No later spec may cite `EMERGE-OBS` counters as gate pass/fail thresholds. No
later spec may cite archived specs or tickets as live certification.

## Staged-Abstraction Declarations

Staged abstraction: `EMERGE-OBS`
Evidence status: observer-only
What it proves now: Pending execution of observer-only emergence ledger evidence, if the corpus exercises it.
What it abstracts: It does not model or certify drama goals, thresholds, or phase-entry criteria.
What it must not fake: It must not turn observer counters into pass/fail thresholds or simulation inputs.
Future tier/feature it must not certify by implication: `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, and any gameplay expansion gate.
Diagnostic that fails if it leaks: `debug_quarantine` / `test_oracle` if observer rows feed cognition, scheduling, validation, authoring, content selection, LOD promotion, or pacing.

Staged abstraction: no-human canonical corpus sampling
Evidence status: pending
What it proves now: Pending sampled/golden no-human behavior evidence.
What it abstracts: Exhaustive coverage over all fixtures, seeds, schedules, and ordinary-life interactions.
What it must not fake: Friendly no-human success must not hide scheduler primitive-action shortcuts or missing actor-known provenance.
Future tier/feature it must not certify by implication: `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, or second-proof ordinary-life expansion.
Diagnostic that fails if it leaks: `scheduler` / `holder_known_context` if no-human metrics or routine labels become action sources.

Staged abstraction: mutation testing
Evidence status: pending
What it proves now: Pending configured mutation posture against `.cargo/mutants.toml`.
What it abstracts: Exhaustive semantic proof over every protected seam.
What it must not fake: A green or unavailable mutation run must not substitute for behavior witnesses, provenance rows, or replay evidence.
Future tier/feature it must not certify by implication: Any future cert gate beyond the seams actually mutated and triaged.
Diagnostic that fails if it leaks: `test_oracle` if survived/uncovered critical mutants are counted as pass without triage.

Staged abstraction: TUI transcript evidence
Evidence status: pending
What it proves now: Pending possession/debug-quarantine transcript evidence.
What it abstracts: Exhaustive coverage over every actor-visible carrier, debug panel, and input path.
What it must not fake: Transcript snapshots must not rely on debug truth as actor knowledge or imply carrier surfaces outside the run.
Future tier/feature it must not certify by implication: `PHASE-4-ENTRY`, second-proof UI surfaces, or LLM/speech surfaces.
Diagnostic that fails if it leaks: `debug_quarantine` / `tui_input_binding` if debug output or player knowledge feeds embodied affordances.

Staged abstraction: temporal evidence in first-proof surfaces
Evidence status: pending
What it proves now: Pending first-proof temporal evidence where current tests exercise scheduler/replay time.
What it abstracts: Phase-4 procedural time, calendars, LOD time acceleration, and second-proof temporal ancestry.
What it must not fake: Event/replay time evidence must not become holder-known planning authority without modeled provenance.
Future tier/feature it must not certify by implication: `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, and LOD/time-acceleration work.
Diagnostic that fails if it leaks: `scheduler` / `holder_known_context` if truth-time validates by becoming cognition input.

## Implementer Self-Check

- [ ] Every exact source path used for evidence is present in the manifest and fetched from `joeloverbeck/tracewake` at `9f1622244c91c5952bd735da76f29fbe58f39f4b` or from an exact exported tree supplied by the user.
- [ ] No branch-name fetch, default-branch lookup, repository metadata, code search, snippets, prior chat memory, or connector namespace label was used as content proof.
- [ ] All ten P0-CERT proof requirements in Section 7 have evidence-status entries.
- [ ] Every canonical gate and first-proof acceptance label is composed only as a cross-reference; no new gate code/status vocabulary is minted.
- [ ] `EMERGE-OBS` is observer-only, non-gating, non-threshold, replay-derived, and quarantined from simulation inputs.
- [ ] Positive and negative fixtures both ran, and negatives failed for the intended responsible layer.
- [ ] Event/replay/projection evidence includes semantic behavior witnesses, not only bytes/checksums.
- [ ] Actor-known context evidence includes provenance, freshness/staleness, source IDs, and hidden-truth exclusion.
- [ ] Debug quarantine evidence includes actor-visible/debug/transcript/observer carrier separation.
- [ ] Diagnostics name responsible layer, expected input source, actual input source, actor-visible output, debug-only output, hidden truth excluded/leaked, replay divergence if any, and remediation category.
- [ ] Archived specs/tickets/reports are labeled historical only.
- [ ] Deferrals are tied to `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, or observer-only obligations as appropriate.
- [ ] No pass/fail result relies on this spec's existence rather than this generated acceptance artifact.
