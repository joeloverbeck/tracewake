# 0038 SPINE-CERT event-log, replay, projection, pipeline, and no-direct-dispatch certification acceptance report

```text
Title: 0038 SPINE-CERT event-log, replay, projection, pipeline, and no-direct-dispatch certification acceptance report
Spec under execution: specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md
Implementation repository: joeloverbeck/tracewake
Implementation commit: b4b59c92d126692c9f2fa4c986695b9f2e20db2c
Commit freshness claim: local checkout HEAD at 0038SPICEREVE-001 scaffold command capture; not independently verified as latest main
Spec posture consumed: P0-CERT passed
Gate label under certification: SPINE-CERT
Verdict: <pending>
Non-executable spec note: this report executes the audit; the spec did not certify results
```

This report instantiates the 0038 acceptance-artifact contract. The 0038 spec
defined the audit plan and withheld a verdict; this report is the artifact that
will render the eventual SPINE-CERT verdict after tickets
`0038SPICEREVE-002` through `0038SPICEREVE-011` fill the seam evidence,
mutation package, and capstone sections.

## Evidence-Status Ledger Legend

Every evidence item added to this report must use the fields below.

```text
Evidence ID:
SPINE seam(s):
Evidence status: <observed run | static review | negative fixture | mutation evidence | historical only | pending>
Fingerprint scope:
Behavior witness:
Replay/provenance record:
Sampling/exhaustiveness claim:
Pending/historical caveat:
Certification use:
Staged-abstraction note:
Artifact path or command transcript:
```

`historical only` evidence may explain lineage but cannot satisfy a live
SPINE-CERT requirement. `static review` may inform risk, but cannot replace a
required behavior witness unless the code path genuinely has no behavior to
execute and the absence itself is the certified fact. `pending` rows are not
counted as certifying evidence.

## Command Transcript And Environment

### Environment

| Field | Value |
|---|---|
| Rust toolchain | `rustc 1.93.0 (254b59607 2026-01-19)`; host `x86_64-unknown-linux-gnu`; LLVM `21.1.8`; full output captured in session transcript |
| Cargo | `cargo 1.93.0 (083ac5135 2025-12-15)` |
| Cargo mutants | `cargo-mutants 27.1.0` from `cargo mutants --version`; standalone `cargo-mutants --version` is not available in this environment and exits 1 via Cargo argument parsing |
| Host OS / architecture | `Linux JOELOVERBECK 6.6.114.1-microsoft-standard-WSL2 #1 SMP PREEMPT_DYNAMIC Mon Dec 1 20:46:23 UTC 2025 x86_64 x86_64 x86_64 GNU/Linux` |
| Implementation commit under test | `b4b59c92d126692c9f2fa4c986695b9f2e20db2c` |
| Transcript directory | `archive/reports/0038_spine_cert_command_transcripts/` |

### Command Transcript Index

| Command | Evidence status | Exit status | Transcript |
|---|---:|---:|---|
| `cargo fmt --all --check` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/cargo_fmt_all_check.txt` |
| `cargo clippy --workspace --all-targets -- -D warnings` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/cargo_clippy_workspace_all_targets.txt` |
| `cargo build --workspace --all-targets --locked` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/cargo_build_workspace_all_targets_locked.txt` |
| `cargo test --workspace --locked` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/cargo_test_workspace_locked.txt` |
| `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_acceptance_artifact_wording.txt` |
| `cargo test --locked -p tracewake-core --test acceptance_gates` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_acceptance_gates.txt` |
| `cargo test --locked -p tracewake-core --test anti_regression_guards` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_anti_regression_guards.txt` |
| `cargo test --locked -p tracewake-core --test ci_workflow_guards` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_ci_workflow_guards.txt` |
| `cargo test --locked -p tracewake-core --test doc_invariant_references` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_doc_invariant_references.txt` |
| `cargo test --locked -p tracewake-core --test emergence_ledger` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_emergence_ledger.txt` |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_event_schema_replay_gates.txt` |
| `cargo test --locked -p tracewake-core --test generative_lock` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_generative_lock.txt` |
| `cargo test --locked -p tracewake-core --test golden_scenarios` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_golden_scenarios.txt` |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_hidden_truth_gates.txt` |
| `cargo test --locked -p tracewake-core --test negative_fixture_runner` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_negative_fixture_runner.txt` |
| `cargo test --locked -p tracewake-core --test no_human_capstone` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_no_human_capstone.txt` |
| `cargo test --locked -p tracewake-core --test spine_conformance` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/core_spine_conformance.txt` |
| `cargo test --locked -p tracewake-content --test fixtures_load` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/content_fixtures_load.txt` |
| `cargo test --locked -p tracewake-content --test forbidden_content` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/content_forbidden_content.txt` |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` |
| `cargo test --locked -p tracewake-content --test schema_conformance` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/content_schema_conformance.txt` |
| `cargo test --locked -p tracewake-tui --test adversarial_gates` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/tui_adversarial_gates.txt` |
| `cargo test --locked -p tracewake-tui --test command_loop_session` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/tui_command_loop_session.txt` |
| `cargo test --locked -p tracewake-tui --test embodied_flow` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/tui_embodied_flow.txt` |
| `cargo test --locked -p tracewake-tui --test readme_sample_session` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/tui_readme_sample_session.txt` |
| `cargo test --locked -p tracewake-tui --test transcript_snapshot` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/tui_transcript_snapshot.txt` |
| `cargo test --locked -p tracewake-tui --test tui_acceptance` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/tui_acceptance.txt` |
| `cargo test --locked -p tracewake-tui --test tui_seam_conformance` | observed run | 0 | `archive/reports/0038_spine_cert_command_transcripts/tui_tui_seam_conformance.txt` |

Exact test filters: none beyond the integration-test targets listed above.
Filtered command rows are paired with the unfiltered required
`cargo test --workspace --locked` row.

## SPINE-01 Event Log, Event Envelope, And Append-Only Causal Stream

Status: evidence captured by `0038SPICEREVE-002`; per-seam verdict remains
pending until capstone `0038SPICEREVE-011`.

### SPINE-01 Evidence Summary

SPINE-01 certifying evidence is drawn from observed command transcripts already
captured in this report plus the supplemental seed-log fingerprint table at
`archive/reports/0038_spine_cert_spine01_seed_log_fingerprints.md`.

The positive corpus named by spec §5 is covered as follows:

| Fixture | Evidence status | Fingerprint scope | Behavior witness | Replay/provenance record | Certification use |
|---|---|---|---|---|---|
| `replay_item_location_001` | observed run + static review | seed log raw canonical bytes; runtime behavior transcript | `golden_fixtures_run` exercises fixture load/action/replay paths; seed log is empty and recorded as such | `archive/reports/0038_spine_cert_spine01_seed_log_fingerprints.md`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` | counted as certifying pass for fixture presence/runtime gate; seed fingerprint is supplemental |
| `container_item_move_001` | observed run + static review | seed log raw canonical bytes; runtime behavior transcript | `golden_fixtures_run` covers item/container movement behavior; seed log is empty and recorded as such | `archive/reports/0038_spine_cert_spine01_seed_log_fingerprints.md`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` | counted as certifying pass for fixture presence/runtime gate; seed fingerprint is supplemental |
| `door_access_001` | observed run + static review | seed log raw canonical bytes; runtime behavior transcript | `golden_fixtures_run` covers door access behavior; seed log is empty and recorded as such | `archive/reports/0038_spine_cert_spine01_seed_log_fingerprints.md`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` | counted as certifying pass for fixture presence/runtime gate; seed fingerprint is supplemental |
| `strongbox_001` | observed run + static review | seed log raw canonical bytes; runtime behavior transcript | `golden_fixtures_run` covers strongbox behavior; seed log contains one `initial_belief_seeded` event | `archive/reports/0038_spine_cert_spine01_seed_log_fingerprints.md`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` | counted as certifying pass for fixture presence/runtime gate |
| `ordinary_workday_001` | observed run + static review | seed log raw canonical bytes; runtime behavior transcript | `golden_fixtures_run` covers ordinary-workday behavior; seed log carries starting belief and role-assignment notice events | `archive/reports/0038_spine_cert_spine01_seed_log_fingerprints.md`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` | counted as certifying pass for fixture presence/runtime gate |
| `sleep_eat_work_001` | observed run + static review | seed log raw canonical bytes; runtime behavior transcript | `sleep_eat_work_fixture_logs_need_effects_and_replays` in `golden_fixtures_run` appends sleep/eat/work events, serializes the log, deserializes it, and compares replay checksums | `archive/reports/0038_spine_cert_spine01_seed_log_fingerprints.md`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` | counted as certifying pass |
| `no_human_day_001` | observed run + static review | seed log raw canonical bytes; runtime behavior transcript | `no_human_day_real_run_replays_metrics_and_trace_projection` in `golden_fixtures_run` runs the no-human day, rebuilds projection, compares physical/agent checksums, verifies metrics replay, and rejects a truncated replay | `archive/reports/0038_spine_cert_spine01_seed_log_fingerprints.md`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` | counted as certifying pass |

Envelope field coverage is structurally present in
`crates/tracewake-core/src/events/envelope.rs`: `EventEnvelope` carries event
ID, event type, schema version, stream, stream position, global order,
simulation tick, ordering key, actor ID, process ID, participants, place ID,
causes, proposal ID, validation report ID, random draw refs, payload fields,
effects summary, and content manifest ID. `EventEnvelope::serialize_canonical`
serializes those fields with stable keys, and `EventLog::append` assigns
monotonic global order and per-stream positions before storing the envelope.
`EventEnvelope::new_caused_v1` rejects cause-required event kinds without
causes.

Adversarial and loud-failure evidence:

| Requirement | Evidence status | Behavior witness | Failure layer | Transcript |
|---|---|---|---|---|
| Missing/unsupported schema versions reject loudly | observed run | `unsupported_event_schema_append_rejected`, `unsupported_event_schema_replay_rejected`, and `unsupported_epistemic_payload_schema_replay_is_loud_and_not_applied` | `event application` / `projection/replay` | `archive/reports/0038_spine_cert_command_transcripts/core_event_schema_replay_gates.txt` |
| Duplicate or corrupted ordering rejects loudly | observed run | `EventLog::deserialize_canonical` rejects reordered global order; `rebuild_projection` verifies global order and stream position; `no_human_day_real_run_replays_metrics_and_trace_projection` rejects a missing-tail replay | `projection/replay` | `archive/reports/0038_spine_cert_command_transcripts/core_event_schema_replay_gates.txt`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` |
| Missing causes are rejected before envelope construction | static review + observed run | `EventKind::requires_cause` + `EventEnvelope::new_caused_v1` cause check; action-emitted cause disposition is locked by `action_emitted_event_kinds_have_cause_disposition` | `event application` | `archive/reports/0038_spine_cert_command_transcripts/core_anti_regression_guards.txt` |
| Mutation-implying payloads require typed apply arms/effects surfaces | observed run | `event_kind_metadata_is_total`, `physical_mutating_event_kinds_have_explicit_world_apply_arms`, `agent_stream_event_kinds_have_explicit_agent_apply_arms`, and `non_world_stream_cannot_change_physical_checksum` | `event application` / `projection/replay` | `archive/reports/0038_spine_cert_command_transcripts/core_anti_regression_guards.txt`; `archive/reports/0038_spine_cert_command_transcripts/core_event_schema_replay_gates.txt` |
| Hidden-truth or prose-born payload insertion rejects or stays non-authoritative | observed run | `content_prose_born_fact_rejected` and hidden-truth adversarial gates in the content/core suites | `content/schema validation` / `tests/fixtures` | `archive/reports/0038_spine_cert_command_transcripts/content_forbidden_content.txt`; `archive/reports/0038_spine_cert_command_transcripts/core_hidden_truth_gates.txt`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` |

Sampling/exhaustiveness claim: field coverage and event-kind metadata coverage
are exhaustive over `EventEnvelope` fields and `EventKind::all()` at this
commit. Positive fixture behavior is the spec-mandated named corpus, not a
random sample. The seed-log fingerprint table is supplemental for initial
fixture logs and does not replace runtime appended-event evidence.

Pending/historical caveat: none for SPINE-01 evidence capture. The seam verdict
is still pending only because the capstone owns the cross-seam verdict table.

## SPINE-02 Replay Rebuild, Divergence Reporting, And Deterministic Replay Gates

Status: evidence captured by `0038SPICEREVE-003`; per-seam verdict remains
pending until capstone `0038SPICEREVE-011`.

### SPINE-02 Evidence Summary

Replay evidence is drawn from the replay report API in
`crates/tracewake-core/src/replay/report.rs`, the replay rebuild path in
`crates/tracewake-core/src/replay/rebuild.rs`, and observed command
transcripts. `ReplayReport` records fixture ID, content manifest ID, initial
checksum, event count, diagnostic event count, unsupported versions,
application errors, final physical checksum, final epistemic checksum, final
agent checksum, expected checksums, `matches_expected`, state diff, and first
divergence detail. `run_replay` computes `matches_expected` only when physical
checksum, agent checksum, state diff, unsupported-version lists, application
errors, epistemic errors, agent errors, and decision-context hash failures all
remain clean.

| Evidence ID | SPINE seam(s) | Evidence status | Fingerprint scope | Behavior witness | Replay/provenance record | Sampling/exhaustiveness claim | Pending/historical caveat | Certification use | Staged-abstraction note | Artifact path or command transcript |
|---|---|---|---|---|---|---|---|---|---|---|
| SPINE02-REPLAY-REPORT-FIELDS | SPINE-02 | static review | parsed semantic content | `ReplayReport` field set and `run_replay` `matches_expected` predicate | `crates/tracewake-core/src/replay/report.rs` | exhaustive over replay report fields at this commit | none | counted as certifying support with observed tests below | none | `crates/tracewake-core/src/replay/report.rs` |
| SPINE02-SCHEMA-REPLAY-GATES | SPINE-02 | observed run | command transcript | `event_schema_replay_gates` covers unsupported schema replay, stream mismatch, non-world checksum no-op, no-human replay checksum match, and typed diagnostic rebuild | replay rebuild output from test assertions | focused replay/schema gate suite | none | counted as certifying pass | none | `archive/reports/0038_spine_cert_command_transcripts/core_event_schema_replay_gates.txt` |
| SPINE02-GOLDEN-SCENARIOS | SPINE-02 | observed run | command transcript | `golden_scenarios` exercises phase-one and phase-three replay scenarios through `run_replay` and `rebuild_projection` | scenario event logs and checksum contexts built in `crates/tracewake-core/tests/golden_scenarios.rs` | named golden scenario suite | none | counted as certifying pass | none | `archive/reports/0038_spine_cert_command_transcripts/core_golden_scenarios.txt` |
| SPINE02-GENERATIVE-LOCK | SPINE-02 | observed run | run seed + command transcript | `generated_sequences_replay_and_satisfy_metamorphic_locks` iterates `GENERATIVE_SEEDS`, runs replay/rebuild checks, serialization round trips, prefix replay, terminal payload tamper checks, and marker no-op checksum checks | generated event logs and replay reports per seed | multi-seed generated corpus with recorded contributor floors | none | counted as certifying pass | generated corpus is property evidence, not a replacement for required named fixtures | `archive/reports/0038_spine_cert_command_transcripts/core_generative_lock.txt` |
| SPINE02-FIXTURE-LOAD | SPINE-02 | observed run | command transcript | `fixtures_load` validates deterministic fixture loading, manifest-scoped paths, schema validation, and fixture census rules | loaded fixture packages and manifests | full content fixture constructor/load suite | none | counted as certifying pass | none | `archive/reports/0038_spine_cert_command_transcripts/content_fixtures_load.txt` |
| SPINE02-GOLDEN-FIXTURE-CORPUS | SPINE-02 | observed run | command transcript | `golden_fixtures_run` covers all fixtures from `fixtures::all()`, including `replay_item_location_001`, `ordinary_workday_001`, `sleep_eat_work_001`, `wait_then_window_passive_charges_each_tick_once_001`, and `sleep_spanning_window_boundary_charges_each_tick_once_001` | runtime logs, replay checksums, tamper checks, and fixture fingerprints inside the test suite | full golden fixture corpus, not sampled | none | counted as certifying pass | none | `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` |
| SPINE02-NONDETERMINISM-BANS | SPINE-02 | negative fixture | command transcript | `negative_fixture_runner` proves banned environment, filesystem, process, wall-clock time, network, and random entry-point fixtures fail under the configured lint/negative-fixture harness | negative fixture stderr assertions | registered negative fixture census | none | counted as certifying pass for host-input isolation | none | `archive/reports/0038_spine_cert_command_transcripts/core_negative_fixture_runner.txt` |

Required corruptions are covered by observed tests:

| Corruption | Evidence status | Witness | Failure layer | Certification use |
|---|---|---|---|---|
| Drop the last event from a passing log | observed run | `no_human_day_real_run_replays_metrics_and_trace_projection` truncates the serialized no-human log and asserts replay does not match expected checksums | `projection/replay` | counted as certifying pass |
| Swap/reorder events | observed run | `EventLog::deserialize_canonical` rejects reordered global order; `rebuild_projection` checks global order and stream positions before applying events | `projection/replay` / `scheduler ordering` | counted as certifying pass |
| Change content manifest ID / package identity | observed run | `fixtures_load` and `golden_fixtures_run` validate manifest-scoped loading and frozen fixture fingerprints; SPINE-05 owns the full save/manifest contract | `content/schema validation` | counted as supporting evidence for replay package identity, not the full SPINE-05 manifest seam |
| Unsupported future schema version | observed run | `unsupported_event_schema_replay_rejected` and related schema-version tests | `projection/replay` | counted as certifying pass |
| Dangling or tampered event provenance | observed run | `no_human_decision_context_hash_gate_fails_when_source_evidence_tampered`, generative payload tamper checks, and event-schema missing payload/cause-adjacent tests | `event application` / `projection/replay` | counted as certifying pass |
| Host time/timezone or external input perturbation | negative fixture | `negative_fixture_runner` includes banned `SystemTime`, `Instant`, `env`, filesystem, network, process, and random entry-point fixtures | `tests/fixtures` | counted as certifying pass for fail-closed input isolation |

Sampling/exhaustiveness claim: the named positive fixture corpus is explicitly
covered by `golden_fixtures_run`; the broader fixture corpus is covered by
`fixtures::all()` tests and fixture census checks. `generative_lock` supplies
multi-seed replay/tamper breadth over generated ordinary-life sequences and
records contributor floors; it is additional property evidence, not a
substitute for named fixture coverage.

Pending/historical caveat: none for SPINE-02 evidence capture. The seam verdict
is still pending only because the capstone owns the cross-seam verdict table.

## SPINE-03 Projection Rebuild And Non-Truth-Writer Quarantine

Status: evidence captured by `0038SPICEREVE-004`; per-seam verdict remains
pending until capstone `0038SPICEREVE-011`.

### SPINE-03 Evidence Summary

Projection evidence is drawn from `KnowledgeContext` sealing,
`EpistemicProjection`/view-model construction, golden fixture runtime tests, and
TUI debug-quarantine adversarial gates. `KnowledgeContext` records holder-known
context ID, event frontier, source scopes, provenance entries, debug flag, and
holder-known context hash. `EmbodiedViewModel` carries the sealed context ID,
hash, frontier, source summary, actor-scoped notebook, and debug availability
without exposing raw projection storage.

| Evidence ID | SPINE seam(s) | Evidence status | Fingerprint scope | Behavior witness | Replay/provenance record | Sampling/exhaustiveness claim | Pending/historical caveat | Certification use | Staged-abstraction note | Artifact path or command transcript |
|---|---|---|---|---|---|---|---|---|---|---|
| SPINE03-CONTEXT-SEAL | SPINE-03 | static review + observed run | parsed semantic content | `KnowledgeContext` seals `holder_known_context_id`, `event_frontier`, `holder_known_context_hash`, `provenance_entries`, and `debug_non_diegetic`; unit tests include `embodied_context_seals_id_hash_provenance_frontier_and_audit` and hash-change checks | `crates/tracewake-core/src/epistemics/knowledge_context.rs`; `hidden_truth_gates` and `anti_regression_guards` transcripts | exhaustive over context seal fields at this commit | none | counted as certifying support with observed tests below | none | `archive/reports/0038_spine_cert_command_transcripts/core_hidden_truth_gates.txt`; `archive/reports/0038_spine_cert_command_transcripts/core_anti_regression_guards.txt` |
| SPINE03-PROJECTION-FIELDS | SPINE-03 | static review + observed run | parsed semantic content | `EmbodiedProjectionSource::from_sealed_context` derives actor-known food, sleep, route, workplace, door, container, item, and local-actor surfaces from sealed context rather than raw truth | `crates/tracewake-core/src/projections.rs`; `crates/tracewake-core/src/view_models.rs` | exhaustive over the embodied projection source fields used by current view construction | none | counted as certifying support with observed tests below | none | `archive/reports/0038_spine_cert_command_transcripts/core_anti_regression_guards.txt` |
| SPINE03-HIDDEN-TRUTH-GATES | SPINE-03 | observed run | command transcript | `hidden_truth_gates` exercises actor-known filtering and forbidden-truth audit paths using real applied epistemic events | hidden-truth harness provenance checked by `guard_0021_hidden_truth_gates_use_event_log_provenance` | focused hidden-truth gate suite | none | counted as certifying pass | none | `archive/reports/0038_spine_cert_command_transcripts/core_hidden_truth_gates.txt` |
| SPINE03-GOLDEN-PROJECTION-CORPUS | SPINE-03 | observed run | command transcript | `golden_fixtures_run` covers `view_filtering_001`, `view_model_local_actions_001`, `possession_parity_001`, `no_human_observation_facts_cite_log_events_001`, `workplace_assignment_provenance_001`, and `stale_workplace_notice_superseded_by_newer_001` plus replay/tamper checks | fixture logs, context hashes, actor-known inputs, and projection-derived assertions inside the test suite | spec-mandated positive projection fixture corpus plus full golden fixture run | none | counted as certifying pass | none | `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` |
| SPINE03-SPINE-CONFORMANCE | SPINE-03 | observed run | command transcript | conformance matrix maps high-risk projection/debug requirements to named positive and negative evidence | `spine_conformance` matrix includes `core/agent`, `tui/view-model`, and `tui/debug` layers | full conformance matrix at this commit | none | counted as certifying pass | none | `archive/reports/0038_spine_cert_command_transcripts/core_spine_conformance.txt` |
| SPINE03-NON-TRUTH-WRITER-GUARDS | SPINE-03 | observed run | command transcript | `anti_regression_guards` proves actor-known context producers are projection-backed, fabricated visible-local event IDs are retired, hidden-truth audit derives from provenance, and direct state mutation remains confined to event application | source guards and synthetic negative mutations in `anti_regression_guards` | guarded production-source census and synthetic negatives | none | counted as certifying pass | none | `archive/reports/0038_spine_cert_command_transcripts/core_anti_regression_guards.txt` |
| SPINE03-TUI-DEBUG-QUARANTINE | SPINE-03 | observed run | command transcript | `tui_seam_conformance` and TUI adversarial gates prove debug epistemics use the core debug builder, debug command strings are not embodied commands, and debug panels do not change embodied affordances/context | holder-known context IDs/hashes/frontiers and debug-only views in TUI tests | TUI seam conformance matrix plus adversarial debug fixture | none | counted as certifying pass for projection-level quarantine; SPINE-07 owns full TUI render/transcript seam | none | `archive/reports/0038_spine_cert_command_transcripts/tui_tui_seam_conformance.txt`; `archive/reports/0038_spine_cert_command_transcripts/tui_adversarial_gates.txt` |

Required positive fixtures are covered by observed tests:

| Fixture | Evidence status | Behavior witness | Projection/provenance record | Certification use |
|---|---|---|---|---|
| `view_filtering_001` | observed run | `golden_fixtures_run` fixture corpus and view filtering assertions | actor-visible view filtered through holder-known facts | counted as certifying pass |
| `view_model_local_actions_001` | observed run | `golden_fixtures_run` and TUI stale/current-view tests | semantic actions include holder-known context ID/hash/frontier provenance | counted as certifying pass |
| `possession_parity_001` | observed run | `golden_fixtures_run` possession fixture and TUI possession adversarial gates | possession does not transfer notebook/debug truth across actor contexts | counted as certifying pass |
| `no_human_observation_facts_cite_log_events_001` | observed run | `golden_fixtures_run` no-human provenance tests | actor-known inputs cite source event IDs and recompute context hashes | counted as certifying pass |
| `workplace_assignment_provenance_001` | observed run | `golden_fixtures_run` workplace provenance fixture | workplace availability carries source-event-backed provenance | counted as certifying pass |
| `stale_workplace_notice_superseded_by_newer_001` | observed run | `fixtures_load`/`golden_fixtures_run` stale-notice tests | newer source event supersedes stale workplace notice | counted as certifying pass |

Required adversarial failures are covered by observed tests:

| Failure family | Evidence status | Witness | Failure layer | Certification use |
|---|---|---|---|---|
| Projection writes authoritative state or appends events | observed run | `event_apply_remains_only_post_seed_mutation_path`, `guard_001_no_direct_state_collection_insert_outside_event_application`, and TUI source direct-apply guards | `projection/replay` / `view-model rendering` | counted as certifying pass |
| Hidden facts absent from holder-known context enter actor-visible projection | observed run | `hidden_truth_gates`; `no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs`; TUI `adversarial_gates_debug_truth_does_not_enter_actor_surfaces` | `actor-known context construction` / `view-model rendering` | counted as certifying pass |
| Actor-visible fact lacks provenance | observed run | holder-known context provenance fields, `no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`, and availability provenance tests | `actor-known context construction` | counted as certifying pass |
| Provenance gap silently falls back to plan | observed run | `method_fallback_requires_new_trace_or_stuck_001`, hidden-truth audit gates, and no-human workplace/food provenance fixtures | `planning/method selection` / `candidate generation` | counted as certifying pass |
| Debug projection content enters embodied view | observed run | `debug_panel_does_not_change_embodied_affordances`, `debug_command_strings_are_not_embodied_commands`, and `tui_epistemic_debug_uses_core_builder_not_raw_projection_storage` | `debug quarantine` / `view-model rendering` | counted as certifying pass |

Sampling/exhaustiveness claim: projection seal and view-model field coverage are
static over the current source fields, while behavior evidence covers the
spec-mandated fixture set plus TUI/debug adversarial fixtures. SPINE-03 cites
TUI debug quarantine only to prove projection-level non-contamination; SPINE-07
owns the full TUI embodied/debug split.

Pending/historical caveat: none for SPINE-03 evidence capture. The seam verdict
is still pending only because the capstone owns the cross-seam verdict table.

## SPINE-04 Randomness And Random-Stream Discipline

Status: evidence captured by `0038SPICEREVE-005`; per-seam verdict remains
pending until capstone `0038SPICEREVE-011`.

### SPINE-04 Evidence Summary

SPINE-04 resolves to the spec's no-state-affecting-RNG branch at this commit:
not exercised because no state-affecting random draw site exists at this
commit. This is an explicit certified absence, not a pass-by-silence. Production
source does include replay-visible event-envelope random draw reference fields
for future random-stream records, but the production source scan found no RNG
entry point or production `Lcg` draw site to exercise.

| Evidence ID | SPINE seam(s) | Evidence status | Fingerprint scope | Behavior witness | Replay/provenance record | Sampling/exhaustiveness claim | Pending/historical caveat | Certification use | Staged-abstraction note | Artifact path or command transcript |
|---|---|---|---|---|---|---|---|---|---|---|
| SPINE04-NO-RNG-SOURCE-SCAN | SPINE-04 | static review | production source text under `crates/*/src` | `grep -rniE '\brng\b|rand::|Lcg' crates/*/src` returns zero matches | current checkout source tree | exhaustive over production crate source files matched by `crates/*/src` | none | counted as branch-1 certified absence of state-affecting RNG draw sites | comments about non-random stable IDs and event-envelope random draw references are not RNG entry points | command run during `0038SPICEREVE-005` closeout |
| SPINE04-ENVELOPE-DRAW-REF-SURFACE | SPINE-04 | static review | event-envelope schema fields | `RandomDrawRef` and `EventEnvelope::random_draws` are present for replay-visible draw records | `crates/tracewake-core/src/events/envelope.rs` | exhaustive over current event-envelope random-draw surface | no exercised random stream exists at this commit | counted as support for future branch-2 replay-visible draw records, not as an exercised RNG witness | no production state-affecting draw site currently populates the field | `crates/tracewake-core/src/events/envelope.rs` |
| SPINE04-RANDOMNESS-BAN-FIXTURES | SPINE-04 | negative fixture | command transcript | `negative_fixture_runner` proves `banned_rand_entry_points` fails under the configured lint/negative-fixture harness, alongside banned env/fs/process/time/network entry points | clippy ban proof table and negative fixture stderr assertions | registered negative fixture census | none | counted as certifying fail-closed hidden random and host-input entry points | none | `archive/reports/0038_spine_cert_command_transcripts/core_negative_fixture_runner.txt` |
| SPINE04-NONDETERMINISM-SOURCE-GUARDS | SPINE-04 | observed run | command transcript | `anti_regression_guards` rejects `HashMap`, `HashSet`, `SystemTime`, `Instant`, `rand::`, environment variables, and thread spawning in guarded production sources | banned nondeterminism token census in `crates/tracewake-core/tests/anti_regression_guards.rs` | guarded production-source census | none | counted as certifying deterministic source discipline under branch 1 | none | `archive/reports/0038_spine_cert_command_transcripts/core_anti_regression_guards.txt` |
| SPINE04-DETERMINISTIC-REPLAY-CORPUS | SPINE-04 | observed run | command transcript | `event_schema_replay_gates` and `golden_fixtures_run` replay the current event corpus deterministically with no random divergence | replay reports, checksums, fixture logs, and tamper checks inside the core/content test suites | spec-required replay/golden evidence for the current no-RNG corpus | no random-stream perturbation witness exists because branch 1 applies | counted as supporting deterministic replay evidence; SPINE-02 owns the full replay seam | none | `archive/reports/0038_spine_cert_command_transcripts/core_event_schema_replay_gates.txt`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` |

The only seeded generator found at this commit is the test-support `Lcg` in
`crates/tracewake-core/tests/support/generative.rs`. It synthesizes generated
test inputs for `generative_lock`; it is not production simulation state,
scheduling, validation, or view-visible randomness.

Adversarial and loud-failure evidence:

| Adversarial case | Evidence status | Witness | Failure layer | Certification use |
|---|---|---|---|---|
| Direct `rand::random`/ambient RNG entry point | negative fixture | `banned_rand_entry_points` in `negative_fixture_runner` | negative fixture / clippy disallowed-method guard | counted as certifying pass |
| OS entropy, wall-clock time, environment, filesystem, network, process, or thread scheduling input | negative fixture + observed run | `negative_fixture_runner` fixtures and `anti_regression_guards` nondeterminism token census | negative fixture / production-source guard | counted as certifying pass |
| Nondeterministic hash iteration in outcome paths | observed run | `anti_regression_guards` bans `HashMap` and `HashSet` in guarded production sources | production-source guard | counted as certifying pass |
| Event with random effect but missing draw reference | not applicable under branch 1 | no production state-affecting random draw site exists at this commit | event application / replay | recorded as not exercised because no state-affecting random draw site exists at this commit |
| Replay random draw record differs from event log | not applicable under branch 1 | no production random-stream record is emitted at this commit | projection/replay | recorded as not exercised because no state-affecting random draw site exists at this commit |

Sampling/exhaustiveness claim: the source scan is exhaustive over production
crate sources matched by `crates/*/src`; negative fixtures are the registered
fixture census for banned random and host-input entry points; replay/golden
commands exercise the current no-RNG corpus.

Pending/historical caveat: none for SPINE-04 evidence capture. The seam verdict
is still pending only because the capstone owns the cross-seam verdict table.

## SPINE-05 Save Package, Manifest Integrity, Schema Versioning, And Upcast/Read Discipline

Status: evidence captured by `0038SPICEREVE-006`; per-seam verdict remains
pending until capstone `0038SPICEREVE-011`.

### SPINE-05 Evidence Summary

SPINE-05 evidence is drawn from `ContentManifest`, fixture package loading,
content schema validation, replay report/schema gates, and the golden fixture
fingerprint corpus. At this commit Tracewake has content manifests and replay
packages, but no production save-package or snapshot-assisted-load subsystem.
Snapshot-assisted load is therefore not applicable: no snapshot subsystem at
this commit. The current save-manifest gap is named explicitly: certification
evidence covers manifest-bound replay packages and event-log ancestry, not a
general persisted save package with snapshot ancestry pointers.

| Evidence ID | SPINE seam(s) | Evidence status | Fingerprint scope | Behavior witness | Replay/provenance record | Sampling/exhaustiveness claim | Pending/historical caveat | Certification use | Staged-abstraction note | Artifact path or command transcript |
|---|---|---|---|---|---|---|---|---|---|---|
| SPINE05-MANIFEST-IDENTITY-FINGERPRINT | SPINE-05 | static review + observed run | content manifest fields and raw source bytes | `ContentManifest::new` records manifest ID, fixture ID, schema version, content version, canonical paths, actor roster, windows, and `twf1-` content fingerprint; tests prove sorted canonical paths and raw-byte sensitivity | `crates/tracewake-content/src/manifest.rs`; `fixtures_load` transcript | exhaustive over current manifest fields and all loaded fixtures in `fixtures::all()` | no persisted save package beyond content manifest + replay package exists at this commit | counted as certifying manifest identity and fingerprint behavior | none | `archive/reports/0038_spine_cert_command_transcripts/content_fixtures_load.txt` |
| SPINE05-FIXTURE-CORPUS-LOADS | SPINE-05 | observed run | command transcript | `all_fixtures_load_deterministically_and_validate` verifies every registered positive fixture constructor is present, validates, and reloads with identical canonical world and manifest fingerprint | fixture manifest IDs of the form `manifest_<fixture_id>` and loaded canonical worlds | full registered positive fixture corpus, not sampled | none | counted as certifying pass for fixture-package load and stable manifest fingerprinting | none | `archive/reports/0038_spine_cert_command_transcripts/content_fixtures_load.txt` |
| SPINE05-FROZEN-FINGERPRINTS | SPINE-05 | observed run | command transcript | `fixture_fingerprints_match_frozen_goldens` compares all fixture fingerprints to the frozen table and proves a synthetic mismatched fingerprint fails the seam | frozen fixture fingerprint table and loaded manifest fingerprints | full frozen fixture fingerprint table | none | counted as certifying pass for manifest-fingerprint regression detection | none | `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` |
| SPINE05-SCHEMA-CONFORMANCE | SPINE-05 | observed run | command transcript | `schema_conformance` maps content/schema and content/validation requirements to named tests and proves fixture-scope schema registration/canonical serialization | content schema registry and serialized fixture bytes | schema conformance matrix for current content fields | none | counted as certifying pass for content schema discipline | none | `archive/reports/0038_spine_cert_command_transcripts/content_schema_conformance.txt` |
| SPINE05-CONTENT-SCHEMA-REJECTIONS | SPINE-05 | observed run | command transcript | `fixtures_load_unsupported_schema_version_rejected_001` rejects `schema_v999` at validation and package load; `forbidden_content` rejects unsupported/prohibited content fields and validates canonical serialization metadata | validation reports with parse/schema diagnostic codes | focused adversarial content schema fixtures | none | counted as certifying loud failure for unsupported content schema and bad content | none | `archive/reports/0038_spine_cert_command_transcripts/content_fixtures_load.txt`; `archive/reports/0038_spine_cert_command_transcripts/content_forbidden_content.txt` |
| SPINE05-EVENT-SCHEMA-REPLAY-GATES | SPINE-05 | observed run | command transcript | `unsupported_event_schema_append_rejected`, `unsupported_event_schema_replay_rejected`, and `unsupported_epistemic_payload_schema_replay_is_loud_and_not_applied` fail loudly for unsupported event/payload schemas | event log append/replay errors and replay report diagnostics | focused event schema replay gate suite | none | counted as certifying loud failure for unsupported event schema and missing upcast/read support | none | `archive/reports/0038_spine_cert_command_transcripts/core_event_schema_replay_gates.txt` |
| SPINE05-REPLAY-PACKAGE-MANIFEST-ID | SPINE-05 | static review + observed run | event envelope and replay report content identity fields | `EventEnvelope` serializes `content_manifest_id`; `ReplayReport` records `content_manifest_id`; golden fixture tests load with fixture manifest IDs and replay serialized logs through the same manifest-scoped package context | `crates/tracewake-core/src/events/envelope.rs`; `crates/tracewake-core/src/replay/report.rs`; `golden_fixtures_run` transcript | exhaustive over current envelope/report identity fields with runtime fixture coverage | no explicit save-package manifest object exists yet | counted as certifying manifest identity reuse for current replay packages | replay package identity is represented by content manifest + event log + replay context, not a general save manifest | `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt`; `archive/reports/0038_spine_cert_command_transcripts/core_event_schema_replay_gates.txt` |

Adversarial and loud-failure evidence:

| Adversarial case | Evidence status | Witness | Failure layer | Certification use |
|---|---|---|---|---|
| Content manifest file order changes without semantic changes | static review + observed run | `ContentManifest::new` sorts source files into canonical paths; `manifest_carries_identity_and_fingerprint` asserts sorted path order | content/schema validation | counted as certifying pass |
| Content byte change must change fingerprint | observed run | `manifest_fingerprint_reprices_raw_file_bytes`, `fixture_fingerprint_reprices_secondary_file_bytes`, and `fixture_fingerprint_reprices_raw_primary_bytes_with_same_parsed_fixture` | content/schema validation | counted as certifying pass |
| Mismatched manifest fingerprint must fail | observed run | `fixture_fingerprints_match_frozen_goldens` mutates the expected fingerprint table and requires an error mentioning the changed fixture | tests/fixtures | counted as certifying pass |
| Missing/extra/prohibited or unsupported content fields | observed run | `forbidden_content` and `golden_fixtures_run` validation rejection tests for bad IDs, unsupported action targets, quest/script/player-only content, hidden truth, and non-canonical ordering | content/schema validation | counted as certifying pass |
| Unsupported content schema | observed run | `fixtures_load_unsupported_schema_version_rejected_001` rejects `schema_v999` at validation and package load | content/schema validation | counted as certifying pass |
| Unsupported event or payload schema | observed run | `event_schema_replay_gates` append, deserialize/replay, and epistemic payload schema gates | event application / projection/replay | counted as certifying pass |
| Snapshot without event ancestry | not applicable at this commit | no production snapshot-assisted-load subsystem exists | documentation status | recorded as not applicable: no snapshot subsystem at this commit; event-log ancestry remains the current replay authority |

Sampling/exhaustiveness claim: fixture load and frozen fingerprint checks cover
the registered positive fixture corpus; schema-conformance covers the current
content schema registry; replay schema gates cover the current unsupported
event/payload schema failure paths.

Pending/historical caveat: none for SPINE-05 evidence capture. The seam verdict
is still pending only because the capstone owns the cross-seam verdict table.

## SPINE-06 Action Proposal, Validation, Scheduling, Event Append, Application, And Feedback Pipeline

Status: evidence captured by `0038SPICEREVE-007`; per-seam verdict remains
pending until capstone `0038SPICEREVE-011`.

### SPINE-06 Evidence Summary

SPINE-06 evidence is drawn from the shared action pipeline, scheduler no-human
proposal loop, source-context gates, hidden-truth planning gates, no-human
capstone ancestry checks, golden scenarios, and the content golden fixture
runner. The canonical action path at this commit is proposal -> validation ->
scheduling -> event append -> event application -> feedback. Exhaustive
mutation-bypass closure is owned by SPINE-08; this section records the
pipeline-owned append-before-apply and representative direct-dispatch evidence.

| Evidence ID | SPINE seam(s) | Evidence status | Fingerprint scope | Behavior witness | Replay/provenance record | Sampling/exhaustiveness claim | Pending/historical caveat | Certification use | Staged-abstraction note | Artifact path or command transcript |
|---|---|---|---|---|---|---|---|---|---|---|
| SPINE06-PIPELINE-STAGES | SPINE-06 | static review + observed run | parsed semantic content | `PipelineStage::all()` includes origin intake, controller binding, source context, action lookup, validation, mutation plan, validation report, event envelope construction, append, application, projection update, and debug linkage; `pipeline_contains_epistemic_slot_and_later_inert_slots` locks the current staged shape | `crates/tracewake-core/src/actions/pipeline.rs` | exhaustive over current pipeline stage enum | none | counted as certifying support with observed transaction tests below | staged placeholders are recorded as inert slots, not unimplemented mutation authority | `archive/reports/0038_spine_cert_command_transcripts/core_golden_scenarios.txt` |
| SPINE06-SOURCE-CONTEXT-GATES | SPINE-06 | observed run | command transcript | source-context tests reject missing, stale, forged, cross-actor, and stale-tick human proposals by reason code; current-view semantic action proposals are required for privileged TUI-origin proposals | validation report reason codes and source context fields | focused source-context adversarial cases | none | counted as certifying pass for source-bound proposal intake | none | `archive/reports/0038_spine_cert_command_transcripts/core_anti_regression_guards.txt`; `archive/reports/0038_spine_cert_command_transcripts/core_spine_conformance.txt` |
| SPINE06-VALIDATE-NONMUTATING | SPINE-06 | observed run | command transcript | `validate_proposal_does_not_commit_or_mutate_state_log_or_epistemics` covers accepted and rejected preflight validation and proves state, log, and projection are unchanged | validation reports for accepted check/wait and rejected check proposals | focused preflight non-mutation cases | none | counted as certifying pass for validation-as-read-only | none | `archive/reports/0038_spine_cert_command_transcripts/core_golden_scenarios.txt` |
| SPINE06-ACCEPTED-TRANSACTION | SPINE-06 | observed run | command transcript | `scheduler_origin_check_container_commits_without_controller_binding` records `proposal_scheduler_check`, accepted report, and appended `ContainerChecked` plus `ObservationRecorded` events; `open_then_move_log_replays_to_same_state` records accepted `proposal_open` and `proposal_move` events that replay to live state | proposal IDs, event IDs in appended envelopes, validation reports, event log replay | representative accepted pipeline transactions | none | counted as certifying pass for proposal -> validation -> append -> apply -> replay feedback | none | `archive/reports/0038_spine_cert_command_transcripts/core_golden_scenarios.txt` |
| SPINE06-REJECTED-TRANSACTION | SPINE-06 | observed run | command transcript | `rejected_proposal_is_structured_and_mutates_no_world_state` records rejected unknown-action proposal, `ActionDefinitionLookup` failed stage, `UnknownActionId` reason, actor/debug summaries, `ActionRejected` event, and unchanged physical state | validation report and rejection event envelope | representative rejected transaction | none | counted as certifying pass for modeled rejection without world mutation | none | `archive/reports/0038_spine_cert_command_transcripts/core_golden_scenarios.txt` |
| SPINE06-SCHEDULER-PIPELINE-HANDOFF | SPINE-06 | static review + observed run | parsed semantic content + command transcript | scheduler builds agent proposals from `ActorDecisionTransaction::run`, assigns deterministic ordering keys, calls `run_pipeline(&mut context, &proposal)`, then appends decision/routine trace events with proposal ancestry; scheduler helper append/apply paths append before applying agent events | no-human event log, proposal ancestry assertions, decision trace records | representative scheduler/no-human path plus source guards | none | counted as certifying pass for scheduler ordering and no scheduler rewrite of actor reason | SPINE-08 owns exhaustive bypass closure | `archive/reports/0038_spine_cert_command_transcripts/core_no_human_capstone.txt`; `archive/reports/0038_spine_cert_command_transcripts/core_anti_regression_guards.txt` |
| SPINE06-HIDDEN-TRUTH-FIREWALL | SPINE-06 | observed run | command transcript | `hidden_truth_gates`, `no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs`, and hidden-truth audit transaction guards prove hidden truth blocks planning or stays diagnostic, not actor-visible fallback input | hidden-truth audit records, provenance-bearing contexts, planner traces | spec-required hidden-truth adversarial fixtures | none | counted as certifying pass for truth-validates-but-does-not-plan | none | `archive/reports/0038_spine_cert_command_transcripts/core_hidden_truth_gates.txt`; `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` |
| SPINE06-GOLDEN-PIPELINE-CORPUS | SPINE-06 | observed run | command transcript | `golden_fixtures_run` exercises sleep/eat/work, ordinary workday, wait/passive charges, blocked work then sleep, interrupted sleep, displaced work completion, safety move, routine blocked diagnostics, method fallback diagnostics, and scheduler rewrite adversarial fixtures | fixture event logs, replay checksums, proposal IDs, trace diagnostics, and actor-visible/debug summaries | spec-mandated positive and adversarial content fixture corpus for SPINE-06 | none | counted as certifying pass for the broad pipeline behavior corpus | none | `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt` |

Representative accepted transaction trace:

| Field | Evidence |
|---|---|
| Proposal ID/source | `proposal_scheduler_check`, scheduler origin; no controller binding required |
| Ordering/source context | deterministic `OrderingKey` built from requested tick, scheduler phase/source, proposal sequence, action ID, target IDs, and proposal ID |
| Validation report | accepted, action `check_container`, target `strongbox_tomas` |
| Appended events | `ContainerChecked`, then `ObservationRecorded` |
| Application/replay | log length reaches 2; open/move accepted transaction replay reconstructs live physical state |
| Actor-visible/debug feedback | validation reports include actor-visible and debug summaries; projection/debug quarantine is also covered by SPINE-03 and SPINE-07 |

Representative rejected transaction trace:

| Field | Evidence |
|---|---|
| Proposal ID/source | test-origin rejected proposal using an unknown action ID |
| Validation report | rejected at `ActionDefinitionLookup` with `UnknownActionId`; `would_mutate == false` |
| Appended events | first appended event is `ActionRejected` |
| Application/replay | physical state remains unchanged |
| Actor-visible/debug feedback | report carries non-empty actor-visible and debug summaries without exposing hidden planner truth |

Adversarial and loud-failure evidence:

| Adversarial case | Evidence status | Witness | Failure layer | Certification use |
|---|---|---|---|---|
| Hidden truth used as planner input | observed run | `hidden_truth_gates` and `no_hidden_truth_planning_001` keep hidden food out of planner inputs or emit typed hidden-truth diagnostics | planning/method selection | counted as certifying pass |
| Prose-born or forbidden provenance fact enters actor-visible pipeline | observed run | content golden and forbidden-content fixtures reject prose-born facts and forbidden provenance inputs | content/schema validation / actor-known context construction | counted as certifying pass |
| Scheduler rewrites actor reason after transaction | observed run | `scheduler_cannot_rewrite_wait_reason_after_transaction_001` and `guard_014_scheduler_cannot_rewrite_transaction_proposals_after_cognition` | scheduler ordering | counted as certifying pass |
| Direct action definition mutates without `run_pipeline` | observed run + static review | `scheduler_never_direct_dispatches_primitive_action`, `guard_006_scheduler_has_no_direct_routine_or_need_proposal_bypass`, and `no_direct_apply_event_outside_event_replay_or_pipeline` | scheduler ordering / event application | counted as SPINE-06 representative pass; exhaustive closure remains SPINE-08 |
| Validation proposes fallback or hidden actor-visible fact | observed run | transaction and hidden-truth audit guards require stuck diagnostics/replanning inputs without actor-visible hidden fact leakage | action validation / planning/method selection | counted as certifying pass |

Sampling/exhaustiveness claim: the accepted/rejected traces are representative
transaction witnesses; `golden_fixtures_run` covers the spec-mandated positive
and adversarial fixture corpus; `no_human_capstone` covers no-human proposal
ancestry and hidden-truth leakage checks; `spine_conformance` maps the seam to
named evidence.

Pending/historical caveat: none for SPINE-06 evidence capture. The seam verdict
is still pending only because the capstone owns the cross-seam verdict table.

## SPINE-07 TUI, Embodied View Models, Transcript Surface, And Debug Split

Status: evidence captured by `0038SPICEREVE-008`; per-seam verdict remains
pending until capstone `0038SPICEREVE-011`.

### SPINE-07 Evidence Summary

SPINE-07 evidence is drawn from TUI seam conformance, adversarial TUI gates,
the real command-loop session tests, transcript snapshots, TUI acceptance
tests, hidden-truth gates, and negative fixture guards. The TUI remains a
client/proof surface: it renders embodied view models from holder-known
contexts, submits semantic actions through the shared proposal pipeline, and
renders debug panels only through non-diegetic debug capability gates.

| Evidence ID | SPINE seam(s) | Evidence status | Fingerprint scope | Behavior witness | Replay/provenance record | Sampling/exhaustiveness claim | Pending/historical caveat | Certification use | Staged-abstraction note | Artifact path or command transcript |
|---|---|---|---|---|---|---|---|---|---|---|
| SPINE07-TRANSCRIPT-SNAPSHOTS | SPINE-07 | observed run | command transcript | `transcript_snapshot` proves representative transcript sections are byte-identical across runs, include embodied `view`/`notebook` sections, and separately include `DEBUG NON-DIEGETIC` event-log, replay, epistemics, beliefs, observations, needs, routines, planner, stuck, and no-human-day sections | captured transcript sections and rendered transcript bytes | representative embodied/debug transcript corpus | none | counted as certifying pass for paired embodied/debug channel evidence | none | `archive/reports/0038_spine_cert_command_transcripts/tui_transcript_snapshot.txt` |
| SPINE07-SEMANTIC-ACTION-PATH | SPINE-07 | observed run + static review | real binary command loop and TUI app path | `command_loop_session` drives the `tracewake-tui` binary, executes `do <semantic_action_id>`, numeric selection, and wait aliases; `TuiApp::submit_semantic_action` builds `proposal_from_current_view_semantic_action`, calls `run_pipeline`, records appended events, and refreshes from the current event frontier | proposal source context, validation report, event log, refreshed view | representative TUI input paths plus static path review | none | counted as certifying pass for input -> semantic proposal -> pipeline -> appended event -> refreshed view | none | `archive/reports/0038_spine_cert_command_transcripts/tui_command_loop_session.txt`; `crates/tracewake-tui/src/app.rs` |
| SPINE07-DEBUG-QUARANTINE | SPINE-07 | observed run | command transcript | `adversarial_gates_debug_truth_does_not_enter_actor_surfaces` renders debug item/projection/epistemics/planner/replay panels, verifies `debug_only`, checks physical checksum is unchanged, and proves actor view/notebook/semantic actions omit `food_hidden_pantry`, `debug_omniscience`, and debug markers | adversarial review artifact with context ID/hash, provenance refs, actor/debug surfaces, and contamination failure mode | spec-required debug omniscience adversarial fixture | none | counted as certifying pass for debug-only quarantine | none | `archive/reports/0038_spine_cert_command_transcripts/tui_adversarial_gates.txt` |
| SPINE07-DEBUG-CAPABILITY | SPINE-07 | static review + negative fixture | debug capability API and compile-fail docs | `DebugCapability::mint` is `pub(crate)`, `debug_only()` returns true, marker is `DEBUG NON-DIEGETIC`, and compile-fail examples prevent external construction or minting | `crates/tracewake-core/src/debug_capability.rs`; negative fixture transcript | exhaustive over current capability constructor surface | none | counted as certifying support for debug capability quarantine | none | `archive/reports/0038_spine_cert_command_transcripts/core_negative_fixture_runner.txt`; `crates/tracewake-core/src/debug_capability.rs` |
| SPINE07-EMBODIED-FILTERING-CORPUS | SPINE-07 | observed run | command transcript | `golden_fixtures_run`, `hidden_truth_gates`, and TUI seam tests cover `view_model_local_actions_001`, raw-assignment omission, menu lag without perception, possession parity, `debug_attach_001`, hidden-truth render omissions, and actor-known context provenance | holder-known context IDs/hashes/frontiers, notebook entries, semantic actions, and debug-only diagnostics | spec-mandated embodied filtering fixtures plus hidden-truth gates | none | counted as certifying pass for actor-known embodied view construction | SPINE-03 owns projection-level non-truth-writer quarantine | `archive/reports/0038_spine_cert_command_transcripts/content_golden_fixtures_run.txt`; `archive/reports/0038_spine_cert_command_transcripts/core_hidden_truth_gates.txt`; `archive/reports/0038_spine_cert_command_transcripts/tui_tui_seam_conformance.txt` |
| SPINE07-TUI-NO-AUTHORITY-GUARDS | SPINE-07 | observed run + static review | command transcript and source guard | `tui_acceptance` and `adversarial_gates` prove TUI sources do not call event application directly, debug panels do not mutate checksums or embodied affordances, debug commands refuse without debug availability, and top-level no-human-day is not a play verb | TUI command outputs, physical checksums, and static source guard results | focused TUI bypass/leak corpus | none | counted as certifying pass for TUI client-not-authority posture | SPINE-08 owns full mutation-path closure | `archive/reports/0038_spine_cert_command_transcripts/tui_tui_acceptance.txt`; `archive/reports/0038_spine_cert_command_transcripts/tui_adversarial_gates.txt` |

Paired channel witness:

| Channel | Evidence |
|---|---|
| Embodied | representative transcript contains `== view.initial ==`, `== notebook.actor_sena ==`, and why-not lines derived from actor-visible context |
| Debug | representative transcript and phase3a debug snapshot contain `DEBUG NON-DIEGETIC` panels for event log, replay, epistemics, beliefs, observations, needs, routines, planner, stuck, and no-human-day |
| Separation | adversarial debug panels leave physical checksum unchanged and do not add debug markers, hidden pantry IDs, or debug omniscience tokens to embodied view/notebook/semantic actions |

Adversarial and loud-failure evidence:

| Adversarial case | Evidence status | Witness | Failure layer | Certification use |
|---|---|---|---|---|
| Debug omniscience enters embodied view | observed run | `adversarial_gates_debug_truth_does_not_enter_actor_surfaces` over `debug_omniscience_excluded_001` | debug quarantine / view-model rendering | counted as certifying pass |
| Hidden truth rendered without holder-known provenance | observed run | `hidden_truth_gates`, `golden_fixtures_run`, and TUI adversarial surfaces omit hidden truth from actor view/notebook/semantic actions | actor-known context construction / view-model rendering | counted as certifying pass |
| TUI mutates state or bypasses validation | observed run + static review | `tui_sources_do_not_call_event_application_directly`, debug checksum checks, and `submit_semantic_action` path through `run_pipeline` | proposal construction / action validation | counted as SPINE-07 representative pass; exhaustive closure remains SPINE-08 |
| Debug-only tokens in ordinary embodied transcript | observed run | transcript/adversarial gates assert debug markers appear only in debug sections and not in actor surfaces | debug quarantine | counted as certifying pass |
| External crate/content constructs debug capability | negative fixture + static review | `DebugCapability` compile-fail examples and `negative_fixture_runner` guards | debug quarantine / tests/fixtures | counted as certifying pass |

Sampling/exhaustiveness claim: transcript snapshots are representative paired
channel outputs; command-loop tests exercise real binary input paths; TUI seam
conformance maps SPINE-07 requirements to named evidence; adversarial gates
cover the spec-mandated debug omniscience and TUI contamination cases.

Pending/historical caveat: none for SPINE-07 evidence capture. The seam verdict
is still pending only because the capstone owns the cross-seam verdict table.

## SPINE-08 No Direct Dispatch And Full Mutation-Path Closure

Status: pending. Owned by `0038SPICEREVE-009`.

## Per-Seam Verdict Table

Status: pending. Owned by `0038SPICEREVE-011`.

| Seam | Required status for SPINE-CERT passed | Required artifacts | Current status |
|---|---|---|---|
| SPINE-01 event log | all positive and adversarial event-log evidence passed | canonical event log fingerprints, corrupted-log rejection | pending |
| SPINE-02 replay | deterministic replay and loud divergence evidence passed | replay reports, duplicate-run checksums, first-divergence artifact | pending |
| SPINE-03 projection | rebuildable non-truth-writer projection evidence passed | projection fingerprints, holder-known context/provenance records | pending |
| SPINE-04 randomness | RNG stream discipline proven or no-RNG absence proven | seed/stream records, banned entry-point guards, duplicate-run evidence | pending |
| SPINE-05 save/manifest | manifest/schema/replay package integrity passed | manifest fingerprints, schema diagnostics, mismatch failure artifact | pending |
| SPINE-06 pipeline | canonical action pipeline and validation evidence passed | proposal/validation/event trace records, accepted/rejected witnesses | pending |
| SPINE-07 TUI/debug | embodied/debug quarantine evidence passed | transcript snapshots, debug-only artifacts, semantic-action path evidence | pending |
| SPINE-08 no direct dispatch | bypass closure and mutation capability evidence passed | compile-fail negative fixtures, API-boundary evidence, mutation results | pending |

## Replay And Provenance Package

Status: pending. Owned by `0038SPICEREVE-011` after seam evidence lands.

## Mutation Package

Status: pending. Owned by `0038SPICEREVE-010`.

## EMERGE-OBS Handling

Status: pending. Owned by `0038SPICEREVE-011`.

`EMERGE-OBS` is observer-only evidence. It is not a phase gate, is not a
pass/fail threshold, and cannot substitute for any SPINE seam.
