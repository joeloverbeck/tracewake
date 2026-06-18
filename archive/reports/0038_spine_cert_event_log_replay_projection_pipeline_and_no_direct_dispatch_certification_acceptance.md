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

Status: pending. Owned by `0038SPICEREVE-005`.

## SPINE-05 Save Package, Manifest Integrity, Schema Versioning, And Upcast/Read Discipline

Status: pending. Owned by `0038SPICEREVE-006`.

## SPINE-06 Action Proposal, Validation, Scheduling, Event Append, Application, And Feedback Pipeline

Status: pending. Owned by `0038SPICEREVE-007`.

## SPINE-07 TUI, Embodied View Models, Transcript Surface, And Debug Split

Status: pending. Owned by `0038SPICEREVE-008`.

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
