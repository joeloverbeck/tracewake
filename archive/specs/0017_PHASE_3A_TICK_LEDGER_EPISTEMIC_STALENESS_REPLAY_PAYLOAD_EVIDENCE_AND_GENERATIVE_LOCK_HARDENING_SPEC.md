# 0017 Phase 3A Tick Ledger, Epistemic Staleness, Replay Payload Evidence, and Generative Lock Hardening Spec

**Status**: COMPLETED
**Staging path:** `archive/specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md`

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `114e2af` (post-0016 closeout; all `0016PHA3ANEEACC` tickets landed).
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the fifth Phase 3A alignment pass — the verification-leaning pass that 0016 §9
itself warranted. Spec 0014 closed `ORD-HARD-001`–`007`; 0015 closed `008`–`013`; 0016
closed `014`–`025`. This audit re-derived the normative contract from `docs/0-foundation/*`,
`docs/1-architecture/*`, `docs/2-execution/*`, and `docs/3-reference/*`, re-examined the full
surface introduced by archived spec 0005 at the post-0016 baseline, and — per the audit
directive — treated every prior correction (including 0016's own deliverables) as unverified
until proven in code. Findings continue the `ORD-HARD` series at `026`.

All evidence below was verified against local sources at the target baseline. Citations use
named symbols, which are grep-stable; line numbers are omitted deliberately.

Two of 0016's delivered *locks* are themselves findings in this pass:

- The need-regime reconciliation gate (`assert_no_duplicate_need_regime_charges`) is
  structurally blind to same-regime double charges — and a real double charge is live in the
  canonical day; see `ORD-HARD-026`.
- The mandated `embodied_workplace_availability_reflects_belief_not_truth_001` adversarial
  fixture was delivered as a placeholder that cannot test divergence; see `ORD-HARD-029`.

The root pattern this pass: **0016 fixed the channels but the evidence layer trusts labels
and recorded histories.** Charges are deduped by regime *label* rather than counted; facts
are stamped `observed_now` regardless of source age; replay verifies the events whose effects
are materialized and is blind to the payloads of those that are not; and the lock layer's
gates only exercise the histories the golden fixtures happen to record.

## 1. Scope

### In scope

- Tick-charge attribution across actions, windows, and the passive accountant
  (`crates/tracewake-core/src/scheduler.rs` no-human runner, `src/need_accounting.rs`,
  `src/actions/defs/wait.rs`, the reconciliation gate in
  `crates/tracewake-content/tests/golden_fixtures_run.rs`).
- Epistemic staleness and provenance-class integrity on the unified no-human actor-known
  surface (`src/epistemics/projection.rs`, `src/agent/no_human_surface.rs`,
  `src/agent/perception.rs`, `src/agent/actor_known.rs`, `src/agent/transaction.rs`).
- Replay verifiability of agent event payloads and capstone gate coverage
  (`src/events/apply.rs`, `src/checksum.rs`, `crates/tracewake-core/tests/no_human_capstone.rs`).
- Embodied workplace believed-access modeling and Phase 3A affordance availability
  (`src/epistemics/knowledge_context.rs`, `src/projections.rs`, the TUI tests).
- Content-schema numeric policy and free-text scan routing
  (`crates/tracewake-content/src/validate.rs`, `schema.rs`).
- Duration open/closed keying coherence (`scheduler.rs`, `actions/pipeline.rs`,
  `need_accounting.rs`).
- Lock-layer durability residuals (`tests/anti_regression_guards.rs`, the mutants CI job,
  guard text fragility) and a new generative-testing lock tier.
- Acceptance-evidence integrity of the 0016 report
  (`reports/0016_ord_life_cert_scoped_acceptance.md`).

### Out of scope

- Re-auditing Phase 1 / 1A spine internals (0010–0012) and Phase 2A epistemic internals
  (0013), except where Phase 3A consumes them.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014/0015/0016 fixed that this audit verified as holding
  (see §3 "Verified holding").

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-009, INV-015, INV-018/020,
  INV-022, INV-026/028, INV-039, INV-043/044/045, INV-061/062/063, INV-067/069/070,
  INV-080, INV-099…INV-110 (especially INV-102 provenance, INV-105 diagnostics are
  authoritative).
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` —
  "a string label is never sufficient proof"; provenance classes must be causally honest.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` —
  the embodied-affordance formula; one holder-known cognition source, one freshness rule.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — replay
  must be able to detect tampered history, not merely re-apply it.
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` —
  ORD-LIFE-CERT clauses; need accounting honesty.

External research consulted for lock design, not authority override:

- proptest stateful/model-based testing: `https://github.com/proptest-rs/proptest`
  (proptest-state-machine); `https://readyset.io/blog/stateful-property-testing-in-rust`
- Factorio per-tick desync oracle (live-vs-replay differential):
  `https://wiki.factorio.com/Desynchronization`
- Metamorphic testing for event sequences:
  MTES, `https://www.researchgate.net/publication/331213575`
- TigerBeetle Tiger Style (release-build invariant assertions; negative-space asserts):
  `https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md`
- Antithesis "sometimes assertions" (reachability evidence):
  `https://antithesis.com/docs/best_practices/sometimes_assertions`
- cargo-mutants diff-scoped PR runs: `https://mutants.rs/pr-diff.html`
- Kani model checking for narrow arithmetic invariants:
  `https://github.com/model-checking/kani`
- Event versioning discipline (Greg Young): `https://leanpub.com/esversioning/read`

## 3. Determination

**Positive verdict.** The in-scope Phase 3A ordinary-life substrate is not fully aligned
with the foundation pack and not maximally locked. Nine findings follow,
`ORD-HARD-026` … `ORD-HARD-034`.

### Verified holding (no action; recorded so they are not re-litigated)

- Duration terminal classification: `events/envelope.rs::is_duration_terminal` is an
  exhaustive wildcard-free `EventKind` match shared by
  `pipeline.rs::body_exclusive_reservation_conflict` and
  `need_accounting.rs::terminal_ticks_by_start`; `WorkBlockFailed` closes reservations
  (`work_block_failed_closes_body_exclusive_reservation`). (But see `ORD-HARD-033` for the
  window-skip predicate's divergent keying.)
- Reasonless autonomous waits rejected: `wait.rs::build_wait_events` returns
  `ReasonCode::MissingWaitReason` for Scheduler/Agent origins
  (`scheduler_wait_without_actor_supplied_reason_is_rejected`).
- Authoritative recomputation: `work.rs::build_work_start_events` reads `AgentState`, not
  proposal need params (`work_forged_need_parameters_do_not_override_authoritative_state`).
- Typed payload errors: `payload_i32` returns `ApplyError::MissingPayload`/`BadPayload`
  (`malformed_sleep_start_payload_returns_typed_error`,
  `malformed_work_start_payload_returns_typed_error`).
- Cross-window stuck detection exists and consumes `RoutineExecution` fields:
  `scheduler.rs::routine_stuck_diagnostic_kinds` reads `expected_next_progress_tick`,
  `last_progress_tick`, `fallback_attempts`.
- The decision context-hash gate is genuinely non-tautological:
  `replay/rebuild.rs::rebuild_decision_context_hash` replays a prefix, rebuilds the surface
  via `NoHumanActorKnownSurfaceBuilder::from_projection` over the prefix projection,
  recomputes `compute_holder_known_context_hash`, and byte-compares; it covers every
  `DecisionTraceRecorded` event; the tamper test
  `no_human_decision_context_hash_gate_fails_when_source_evidence_tampered` proves it can
  fail; `report.rs::run_replay::matches_expected` includes it. (But the capstone never
  asserts it — `ORD-HARD-028`.)
- Stored ordering is verified, not reassigned: `log.rs::append_deserialized` returns
  `GlobalOrderMismatch`/`StreamPositionMismatch`
  (`deserialization_rejects_reordered_global_order`); in-memory reorder caught by
  `verify_event_ordering`.
- Rebuild poisons on the first agent/epistemic/world application error
  (`corrupt_midstream_agent_event_poisons_rebuild`).
- Unbacked-fact constructors deleted: `with_role_assignment_notice` /
  `with_sleep_place_knowledge` exist only as `compile_fail` doctests; production paths use
  private projection-fed builders; `SourceEventIds::checked` rejects empty. (But the fact
  *field* is a raw `Vec` — `ORD-HARD-032`.)
- The hidden-truth audit gate runs before `select_phase3a_method` with a typed stamp;
  `htn.rs::resolve_condition` requires `modeled_fact_proof` for
  `FixtureAuthoredPossibility`/`SharedPipelinePreconditions`
  (`fixture_and_shared_conditions_require_actor_known_facts`).
- Severe safety flight is real: `planner.rs::plan_leave_unsafe_place` BFS over
  `known_edges` with a typed `Knowledge` blocker fallback
  (`severe_safety_with_known_exit_proposes_move_before_hunger`).
- Embodied need scalar leak fixed: `NeedStatusEntry` carries `kind/band_label/last_cause`
  only; TUI render prints no `value=` token.
- Possession parity: `human_after_authorization_matches_scheduler_validation`,
  `human_binding_adds_no_extra_semantic_actions`.
- Debug quarantine: `Debug*View` structs hold crate-private `DebugCapability` with
  compile-fail doctests.
- Lock-layer census holds where scoped: workspace classification census
  (`workspace_source_classification_census_matches_production_tree`) catches new and
  relocated files; negative-fixture directory census
  (`registered_negative_fixtures_match_directory_census`); clippy↔fixture parity
  (`clippy_ban_entries_have_proving_negative_fixtures`); alias and macro evasion proven by
  `banned_hashmap_import_alias` and `banned_macro_expands_to_spawn_or_fs`; clippy bans
  cover `rand`, `std::fs::write`, `OpenOptions`, `std::env::var` workspace-wide.
- CI `lock-layer-gates` runs all gates `--locked`, no `continue-on-error`, no path filters.
- Content tuning constructors: `NeedModelState` has no `Default`; `from_seed_parts` takes
  an explicit `NeedModelState`; `display_label` is scanned by `reject_script_marker_text`
  (`display_label_script_marker_is_rejected`); schema-field registry parity holds
  (`content_new_field_requires_typed_validation_and_canonical_serialization_metadata`).
- The conformance index carries the rows 0016 §5.6 required.

## 4. Findings and remediation requirements

### ORD-HARD-026 — Wait-advanced ticks are double-charged as awake, and the reconciliation gate cannot see it

**Severity:** blocker for scoped `ORD-LIFE-CERT` evidence.

**Responsible layers:** `scheduler`, `action_validation`, `test_oracle`.

**Doctrine breached:** INV-039, INV-043/044/045 (need accounting must be causally honest),
INV-009 (the duplicate charge is eventful but causally false — two event ancestries each
claim the same elapsed tick). Same class as `ORD-HARD-014`, which 0016 fixed for
sleep/work regimes but not for action-emitted awake deltas.

**Evidence:** `actions/defs/wait.rs::build_wait_events` resolves the `ticks` parameter
with `.unwrap_or(1)` and rejects `0` — every autonomous wait (the planner's
`PlannerGoal::WaitWithReason` supplies no `ticks` param) advances one tick and emits a
`NeedDeltaApplied` with `cause_kind="tick_delta"` covering that tick. In
`scheduler.rs::run_no_human_day`, `last_decision_tick_by_actor` is set to
`window.start_tick` unconditionally after the passive charge; the next window's
`append_passive_need_events_before_decision` charges
`(window.start_tick, next_window.start_tick]` as awake via the regime classifier, which
classifies any tick not inside a Sleep/Work interval as `TickRegime::Awake` — including the
tick the wait already charged. Both deltas materialize into `AgentState` need values. The
canonical day contains `ActorWaited` (asserted in `no_human_capstone.rs`), so the doubled
values are encoded in today's golden checksums.

The gate that 0016 delivered to prevent exactly this,
`golden_fixtures_run.rs::assert_no_duplicate_need_regime_charges`, accumulates a
`BTreeSet<&'static str>` of regime labels per `(actor, need, tick)` and fails only when
`len() > 1`. Two `tick_delta` charges on one tick both insert the literal `"awake"` and
collapse to a set of size one. The gate asserts the negation of its name.

**Why existing gates miss it:** the gate IS the miss;
`passive_awake_need_deltas_are_deterministic_and_non_reducing` asserts sign only; golden
checksums byte-match the wrong values consistently.

**Required correction:** make the tick-regime accounting authority cover *all*
`tick_delta`-class charges, not only window passives: either (a) the scheduler consumes
committed `ActorWaited`/`NeedDeltaApplied` coverage when computing each actor's
`elapsed_ticks` (advance `last_decision_tick_by_actor` past action-charged ticks), or
(b) `append_passive_need_events_before_decision` subtracts ticks already charged by an
action-emitted `tick_delta` event for that actor/need. Either way the classifier in
`need_accounting.rs` remains the single attribution authority and gains an awake-by-action
interval source. Golden checksums will change; every fixture diff must be explained in the
acceptance artifact with a per-actor need ledger (the 0016 §7.1 format).

**Structural lock:**

- Rewrite `assert_no_duplicate_need_regime_charges` to count charge *occurrences* per
  `(actor, need, tick)` and assert exactly ≤ 1, reconciling the event ledger against
  `classify_actor_tick_regimes` output — a true reconciliation, not a label-set.
- Adversarial fixture `wait_then_window_passive_charges_each_tick_once_001`: an autonomous
  wait mid-window followed by a window boundary; assert single-charge per tick.
- A release-build (`assert!`, not `debug_assert!`) single-charge invariant inside the
  need-accounting apply path (see §5 tier 2).

### ORD-HARD-027 — The unified no-human surface has no staleness rule and re-stamps aged records as `observed_now`

**Severity:** high.

**Responsible layers:** `holder_known_context`, `projection`.

**Doctrine breached:** INV-026 (provenance must record acquisition/observation time
honestly), INV-102 (a fact stamped `observed_now` at a tick when no observation occurred is
false provenance in the authoritative trace), INV-028 read precisely (beliefs persist —
*as remembered beliefs*, not as perpetually-current observations); foundation doc 14 and
architecture doc 03 (one cognition source implies one freshness semantics). This is the
unfinished half of `ORD-HARD-021` / ticket `0016PHA3ANEEACC-015`.

**Evidence:** `epistemics/projection.rs::actor_known_records_for_context` returns every
`ActorKnownProjectionRecord` for the viewer actor; the `KnowledgeContext` argument is
consulted only for `viewer_actor_id()` — no tick comparison, no place comparison. The
consumer `no_human_surface.rs::consume_projection_record` then mints
`ActorKnownFact::observed_now(..., self.decision_tick, ...)` for every record (routes, food
sources, sleep places, workplaces) — a record projected at tick 0 surfaces at tick 95 as an
observation made at tick 95. Meanwhile the embodied path,
`perception.rs::current_place_knowledge_context`, filters to the latest current-place
perception window (`current_place_knowledge_context_uses_latest_projection_window_not_live_truth`).
The two consumers of the "unified" projection therefore apply divergent freshness rules,
and the no-human decision trace records a false observation class and time.

**Why existing gates miss it:** all `no_human_surface.rs` tests build from a fresh log at a
single tick; the staleness test exists only for the perception path; the context-hash gate
re-derives the same mis-stamped facts deterministically, so replay confirms the
misstamping byte-identically.

**Required correction:** define one freshness rule on the projection and make both
consumers use it: `actor_known_records_for_context` (or a successor accessor taking the
context tick/place) classifies each record as *currently-perceived* (source observation at
the actor's latest perception window for the relevant place) or *remembered* (anything
older). The surface builder mints `observed_now` only for currently-perceived records and
`remembered_belief` — carrying the ORIGINAL source observation tick, not the decision tick
— for the rest. Believed-but-stale knowledge remains available to planning (INV-028); only
its provenance class becomes honest.

**Structural lock:**

- Fixture `aged_food_record_surfaces_as_remembered_belief_not_observation_001`: food
  observed at tick N, actor leaves, decision at tick N+k ⇒ the consumed fact is
  `remembered_belief` with acquisition tick N; replay byte-match.
- A provenance-class audit in `ActorDecisionTransaction::run`: any `observed_now` fact
  whose cited source events all precede the actor's latest perception event for that place
  fails closed with a typed `Stuck` (`blocker_code = provenance_class_mismatch`).
- Guard banning direct `ActorKnownFact::observed_now` construction in
  `no_human_surface.rs` outside the currently-perceived branch (positive-presence with the
  runtime audit above as backstop).

### ORD-HARD-028 — Agent event payloads outside materialized state are replay-unverifiable; the capstone ignores the context-hash gate

**Severity:** high.

**Responsible layers:** `replay`, `test_oracle`.

**Doctrine breached:** INV-018, INV-105 (an event class whose semantic content can be
tampered without any gate failing is decorative history); 0005 §15.1 (replay must
reconstruct continue-routine and candidate-goal evaluations). Narrows the remaining half of
`ORD-HARD-024`.

**Evidence:** `events/apply.rs::AGENT_WORLD_NOOP_ALLOWLIST` still contains
`CandidateGoalsEvaluated`, `ContinueRoutineProposed`, `ContinueRoutineAccepted`,
`ContinueRoutineRejected` (plus the genuinely-marker `NoHumanDayStarted/Completed` and the
dual-stream `FoodConsumed`). These four mutate no rebuilt state and appear in no checksum
family (`checksum.rs::compute_agent_state_checksum` has no continue-routine or
candidate-goal coverage). Flipping `ContinueRoutineAccepted` ↔ `ContinueRoutineRejected`
in a stored log, or rewriting a rejection `reason`, survives replay: checksums unchanged,
metrics count unchanged (same kind census), context-hash gate untouched (it covers only
`DecisionTraceRecorded`). Separately,
`tests/no_human_capstone.rs::no_human_capstone_proves_typed_ancestry_and_replay` asserts
checksums, diffs, and application errors but contains no reference to
`decision_context_hash_failures` — the flagship no-human acceptance test computes the
0016 gate's result and discards it.

**Why existing gates miss it:**
`agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state` asserts
list membership, not that allowlisted kinds are payload-verifiable; the capstone predates
the gate and was never extended.

**Required correction:** materialize continue-routine arbitration and candidate-goal
evaluation summaries into `AgentState` (event-id-keyed ledgers, mirroring
`ordinary_life_episodes`) with `AGENT_STATE_CHECKSUM_COVERAGE` families — shrinking the
allowlist to `NoHumanDayStarted/Completed` plus the classified dual-stream `FoodConsumed`.
Add the missing capstone assertion
(`assert!(rebuild.decision_context_hash_failures.is_empty())`).

**Structural lock:**

- Tamper tests: flip an `Accepted`→`Rejected` and rewrite a `reason` in a copied golden
  log; assert `!matches_expected` in both.
- Extend the allowlist census so every remaining entry must cite the projection/checksum
  capturing its effect or be a registered pure-marker kind with a payload-free schema.
- Extend `assert_phase3a_checklist_is_mapped` (capstone) with a
  `decision_context_hash_gate` item.

### ORD-HARD-029 — Embodied workplace believed-access was never modeled; the 0016 divergence fixture is a placeholder; Phase 3A affordances hard-code availability

**Severity:** high.

**Responsible layers:** `view_model`, `projection`, `holder_known_context`.

**Doctrine breached:** INV-067, INV-070; architecture doc 03's embodied-affordance formula
(availability from believed conditions, commit-time validation from truth). Completes —
genuinely, this time — `ORD-HARD-019`'s unfinished half. The direction of the current
error is leak-safe (no truth read; `guard_014` holds) but the mandated belief-side
substitute does not exist.

**Evidence:** `epistemics/knowledge_context.rs::ActorKnownWorkplaceFact` carries
`workplace_id` and `place_id` only — no believed-access dimension, so belief-vs-truth
divergence is structurally unrepresentable. `projections.rs::phase3a_semantic_actions`
computes workplace `enabled = at_workplace` alone, and builds the sleep/eat/continue
entries with hard-coded `SemanticActionEntry::new(..., true, None)` — availability asserted
by projection-local fiat rather than computed from actor-known conditions (known servings,
reservation beliefs, intention state), unlike the Phase-1/2 entries that route through
`with_validator_availability`. The 0016-mandated adversarial fixture
`embodied_workplace_availability_reflects_belief_not_truth_001` seeds
`initial_beliefs: Vec::new()` and its TUI test asserts only absence-without-context — the
identical shape to the pre-existing
`embodied_view_omits_raw_workplace_assignment_without_context`. The "truth open, belief
closed ⇒ embodied shows closed" case named in 0016's structural-lock clause is untested,
and `reports/0016_ord_life_cert_scoped_acceptance.md` §5 overstates it as proven.

**Why existing gates miss it:** `guard_014_embodied_projection_*` bans the truth-read
token; no gate can assert a belief dimension that has no type. The fixture exists by name,
so name-census checks pass.

**Required correction:** add believed-access to the workplace fact (sourced from
notice/observation events through the projection); compute embodied workplace availability
from it; replace the hard-coded `true` entries with availability derived from actor-known
facts (an actor-known preflight — belief decides the menu; the validator still decides the
commit, per INV-099). Deliver the real divergence fixture.

**Structural lock:**

- Fixture (reusing the 0016 name, now with substance): seed a context workplace fact with
  believed-access closed while `state.workplaces` truth is open; assert embodied shows
  unavailable-with-reason; debug shows the divergence non-diegetically; replay byte-match.
- Inverse fixture: believed-access open, truth closed ⇒ embodied shows available, commit
  fails with the modeled `access` consequence (INV-106 learning loop).
- Guard: no `SemanticActionEntry::new` call with a literal `true` enabled flag in
  `phase3a_semantic_actions` (runtime backstop: a test seeding zero known servings asserts
  the eat entry is disabled with a typed reason).

### ORD-HARD-030 — Content numeric policy and free-text scan routing are partial

**Severity:** high.

**Responsible layers:** `content_seeding`.

**Doctrine breached:** INV-022, INV-061/062 (authored machinery, not authored outcomes),
INV-080; foundation doc 09. Continues `ORD-HARD-022`, whose fix validated direction for
exactly four delta fields and one label.

**Evidence (all `crates/tracewake-content/src/validate.rs` unless noted):**

- Scan routing is asymmetric: `validate_phase3a_no_shortcuts` routes
  `WorkplaceSchema.output_tag` through `reject_shortcut_text` only (the
  `PHASE3A_SHORTCUT_MARKERS` set), while `display_label` goes through
  `reject_script_marker_text` only — two disjoint marker sets, the union applied to no
  field. The `SCANNED_STRING_FIELDS` registry and its census test assert each field is
  scanned by *something*, conflating the two policies and giving false assurance.
- `WorkplaceSchema.work_duration_ticks`, `max_fatigue_to_start`, `max_hunger_to_start`
  are entirely unvalidated: a `0` duration authors a zero-length work block; a negative
  threshold silently dead-ends a routine; `i32::MAX` makes the need gate vacuous.
- `InitialNeedSchema.value` (u16) is converted via `NeedState::initial` →
  `clamp_need_value`, silently materializing `5000` as `1000`. The authored seed and the
  materialized seed disagree with no rejection — a fact mutated by load.
- `validate_nonnegative_tuning` permits `0` for relief-direction fields
  (`hunger_reduction_per_serving`, `fatigue_recovery_per_tick`) — silently-dead resources
  that can never satisfy their need — and caps magnitude at a magic `10_000`, ten times
  `NEED_MAX` (1000), so one authored tick can swing a need across its entire band ten times
  over.

**Why existing gates miss it:** `negative_need_tuning_direction_is_rejected` asserts the
`< 0` branch only; the string-field census asserts presence-in-registry, not
which-scan-applies; no test touches workplace duration/threshold bounds or the initial
need bound.

**Required correction:** introduce a numeric field-policy registry parallel to the string
one: every numeric tuning/seed field in `FixtureSchema` is classified (pressure-direction
`>= 0`, relief-direction `> 0`, duration `>= 1`, need-band `0..=NEED_MAX`) with caps
derived from `NEED_MAX`, not magic constants; reject out-of-band `InitialNeedSchema.value`
instead of clamping. Re-route every free-text field through the *union* scanner
(shortcut ∪ script ∪ authored-outcome markers), with the registry recording the policy per
field rather than mere membership.

**Structural lock:** a schema test enumerating every numeric field against the numeric
registry (a new unclassified field fails compilation of the census); negative fixtures
`fixture_workplace_zero_duration_rejected_001`,
`fixture_initial_need_out_of_band_rejected_001`,
`fixture_output_tag_script_marker_rejected_001`; the registry census asserts each string
field's policy is the union scan or carries a recorded narrower-policy rationale.

### ORD-HARD-031 — Provenance citations are existence-checked, not witness-checked

**Severity:** medium.

**Responsible layers:** `holder_known_context`, `proposal_construction`.

**Doctrine breached:** INV-102 (provenance must be *sufficient for review* — a citation to
a non-witnessing event reviews to nothing); foundation doc 14.

**Evidence:**

- `no_human_surface.rs::add_window_framing_facts` stamps `actor_current_place_visible`
  (an `observed_now` fact whose value is the builder-supplied `current_place_id`) and
  `agent_needs_present` (read directly from `agent_state.needs_by_actor()`) with the
  day/advance *frame marker* event as their sole `source_event_ids`. The frame event
  witnesses neither the actor's place nor their needs. The same-tick
  `record_current_place_perception` event — which DOES witness the place — exists but is
  not the cited source.
- `transaction.rs::dangling_provenance_diagnostic` checks only that cited ids exist in the
  available frontier — citation *existence*, not citation *support*; any fact citing any
  real event passes.
- The facts↔structured reconciliation is one-directional: the audit's
  `structured_fact_gaps` catches structured-entries-without-facts, but facts with no
  structured counterpart (`actor_at_workplace`, `assigned_workplace_known` minted by
  `add_role_assignment_notice`) bypass it, and `known_closed_doors` /
  `known_containers_by_place` are hardcoded empty in `build` while facts could assert them.
- `add_window_framing_facts` early-returns on a `None` frame id, silently building a
  surface with no current-place fact instead of failing closed.

**Required correction:** every framing fact cites its genuine witnessing event
(current-place fact cites the same-tick perception event; needs-present cites the latest
need event for the actor; intention facts cite the adoption/lifecycle event). Add a
provenance-kind compatibility table to the transaction audit: each fact kind declares the
event kinds that can witness it, and a mismatch fails closed (typed
`provenance_witness_mismatch`). Derive structured context fields from the facts via a
single builder function so the reconciliation is bidirectional by construction. A missing
frame event fails closed with a typed `Stuck`, not a degraded surface.

**Structural lock:** the witness-compatibility audit (runtime, fail-closed); a test
constructing a fact citing a real-but-unrelated event and asserting the typed failure;
a fixture with an empty prefix log asserting the typed limitation rather than a fact-less
decision.

### ORD-HARD-032 — Lock-layer residuals: raw witness field, exempt-file relocation, fragile string guards, unpinned mutants

**Severity:** medium.

**Responsible layers:** `test_oracle`.

**Doctrine breached:** none directly; lock-layer durability. Continues `ORD-HARD-025`.

**Evidence:**

- `agent/actor_known.rs::ActorKnownFact` stores `source_event_ids: Vec<EventId>` (raw)
  even though constructors take the `SourceEventIds` witness; the in-crate
  `unproven`/`unbacked_for_rejected_test_only` path writes `Vec::new()` directly. The
  runtime backstop `dangling_provenance_diagnostic` flat-maps the ids and `find`s
  non-resolving entries — an *empty* list yields nothing and passes. The only guard on the
  empty path is a single-spelling string count
  (`guard_018_actor_known_facts_require_source_event_witness` counts the literal
  `"source_event_ids: Vec::new()"`), defeated by `vec![]`, `Default::default()`, or
  `Vec::with_capacity(0)`.
- The truth-read bans (`assert_absent(builder, "state.workplaces")` etc.) run only against
  the GuardedLayer globs (`agent/`, `scheduler.rs`, `projections.rs`). Cognition logic
  added to an already-*Exempt* core file (`state.rs`, `view_models.rs`,
  `actions/pipeline.rs`) escapes every targeted ban; the census proves classification
  completeness, not that exempt files perform no cognition.
- Several positive-presence guards assert indentation-coupled multi-line literals (e.g.
  the `build_sleep_completion_events(\n                    state,` call-shape checks) — a
  rustfmt configuration change voids them silently.
- The `mutants-lock-layer` CI job installs `cargo-mutants` unpinned (baseline recorded
  against 27.1.0) and nothing fails when the missed-mutant count grows past the recorded
  baseline (145 misses, dispositioned once).

**Required correction:** store the `SourceEventIds` newtype on the fact itself and make
the runtime diagnostic treat an empty witness list as `ProvenanceDangling`; extend the
raw-truth-table-accessor bans workspace-wide with an explicit recorded allowlist of
legitimate readers; whitespace-normalize source text before guard substring checks (and
document the normalization); pin the `cargo-mutants` version in CI and commit the
baseline miss-set, failing the scheduled job when a previously-caught mutant is missed.

**Structural lock:** the field-type change is the lock (empty unconstructable);
`cognition_inputs_are_context_backed` test over all `production_sources()` with the reader
allowlist; the committed miss-set diff gate.

### ORD-HARD-033 — Duration open/closed keying drifts across the three consumers; duplicate terminals are silently reconciled

**Severity:** medium.

**Responsible layers:** `scheduler`, `action_validation`.

**Doctrine breached:** INV-009 (silent reconciliation of contradictory history); the 0016
§5 single-classification-authority commitment (`is_duration_terminal` was meant to prevent
three-way drift — the predicate is shared, but the start/terminal *pairing key* is not).

**Evidence:** `pipeline.rs::body_exclusive_reservation_conflict` and
`need_accounting.rs::terminal_ticks_by_start` pair terminals to starts via
`EventCause::Event(start_id)`; `scheduler.rs::actor_has_open_body_exclusive_duration`
pairs via `proposal_id`. A terminal carrying the correct event-id cause but a divergent or
absent proposal id closes the duration for two authorities and not the third.
Additionally, `terminal_ticks_by_start` resolves multiple terminals for one start with
`.and_modify(|tick| *tick = (*tick).min(...))` — contradictory duplicate terminals (a
`WorkBlockCompleted` *and* a `WorkBlockFailed` for the same start) are silently clamped to
the earliest instead of rejected.

**Why existing gates miss it:** all fixtures emit exactly one terminal per start with
coinciding keys; no test forges a key divergence or a double terminal.

**Required correction:** a single shared
`open_body_exclusive_starts(log, actor, tick) -> BTreeSet<EventId>` authority (keyed by
`EventCause::Event`) consumed by the reservation check, the window-skip, and the
classifier; more than one terminal per start becomes a typed application/replay error.

**Structural lock:** the shared function is the lock (one keying, by construction); a
forged-divergent-key test and a double-terminal test asserting the typed error; replay
gate `duplicate_duration_terminal_poisons_rebuild_001`.

### ORD-HARD-034 — The 0016 acceptance report overstates three delivered proofs

**Severity:** medium (documentation / audit-history integrity).

**Responsible layers:** `test_oracle` (acceptance evidence).

**Doctrine breached:** INV-105 in spirit (acceptance artifacts are authoritative
diagnostic data; an artifact claiming unproven evidence is decorative); 0016 §6's own rule
that audits must not silently rewrite audit history.

**Evidence:**

- `reports/0016_ord_life_cert_scoped_acceptance.md` claims a "synthetic V0→V1 upcast
  fixture locking the migration path's loud-failure contract"; no such fixture exists —
  `EVENT_SCHEMA_REGISTRY` holds a single V1 entry with `CurrentNoMigrationRequired`, and
  only the unknown-version loud-rejection path is tested
  (`unsupported_event_schema_version_replay_fails_loudly`). The genuine rejection contract
  is healthy; the upcast claim is unbacked.
- The report's §5 presents `embodied_workplace_availability_reflects_belief_not_truth_001`
  as proving belief-over-truth; the fixture tests absence-without-belief
  (see `ORD-HARD-029`).
- 0016's named content negative fixtures (`fixture_negative_awake_delta_rejected_001`,
  `fixture_display_label_script_marker_rejected_001`) were delivered as inline `validate.rs`
  unit tests, not registered fixtures; coverage is real but the census the names imply does
  not include them.

**Required correction:** amend the 0016 acceptance report with a dated correction section
recording the three overstatements (citing this spec's findings), and record the
overturned claims in the conformance index per the 0016 §6 precedent. For the upcast: this
spec takes the position that the loud-rejection contract is the durable requirement;
either implement a real synthetic prior-version upcast fixture as part of `ORD-HARD-028`'s
work or replace the report claim with the rejection-only contract — implementer's choice,
recorded either way.

## 5. Anti-contamination lock layer (consolidated)

Tiers 1–4 extend the 0016 layer; tier 5 is new and is this spec's structural answer to the
recurring "delivered lock was hollow" pattern — locks themselves must be exercised by
histories no golden fixture recorded.

1. **Compile-time impossibility:** `SourceEventIds` stored on the fact (empty witness
   unconstructable, `ORD-HARD-032`); believed-access as a required typed dimension of the
   workplace fact (`ORD-HARD-029`); numeric field-policy registry whose census fails on
   unclassified fields (`ORD-HARD-030`); single shared open-duration authority
   (`ORD-HARD-033`); structured context fields derived from facts by one builder
   (`ORD-HARD-031`).
2. **Runtime gates (release-build asserts, Tiger-Style):** single-charge-per-tick
   invariant in the need-accounting apply path (`ORD-HARD-026`); provenance-class and
   witness-compatibility audits failing closed in the decision transaction
   (`ORD-HARD-027`/`031`); duplicate-terminal typed error (`ORD-HARD-033`).
3. **Test-oracle corrections:** occurrence-counting charge reconciliation
   (`ORD-HARD-026`); capstone context-hash assertion and the shrunken, payload-verifiable
   `WorldNoOp` allowlist with tamper-flip gates (`ORD-HARD-028`); the real divergence
   fixture pair (`ORD-HARD-029`).
4. **Source guards (`guard_017_*`):** workspace-wide truth-accessor bans with recorded
   reader allowlist; whitespace-normalized scans with documented evasion modes; no-literal
   `true` enabled flags in the Phase 3A semantic-action builder.
5. **Generative lock tier (new):**
   - **Deterministic sequence generator:** an in-tree, explicitly-seeded generator (or
     `proptest`/`proptest-state-machine` as a dev-dependency — see §9) producing valid
     bounded action/window sequences over small fixtures: interleaved waits, sleeps, work
     blocks, eats, window boundaries, interruptions.
   - **Differential oracle:** for every generated sequence — run live, serialize, replay,
     and compare full agent+physical checksums at every window boundary; report the first
     divergent tick (the Factorio desync oracle). This is the generator-scoped complement
     to the fixture-scoped golden gates.
   - **Metamorphic relations:** prefix-replay + suffix-replay ≡ full replay; an appended
     pure-marker event changes no checksum; single-charge-per-tick holds across every
     generated history (the property that would have caught `ORD-HARD-026` without anyone
     authoring the adversarial fixture).
   - **Reachability assertions:** "sometimes"-style counters in the generative harness
     asserting the generated corpus actually exercised regime transitions, interruptions,
     and terminal kinds — coverage evidence, not hope.
   - **Mutation cadence:** `cargo mutants --in-diff` against the merge base on every PR
     touching guarded layers; the scheduled full run diffs against the committed baseline
     miss-set and fails on growth; the `cargo-mutants` version pinned.
   - **Optional (recorded decision either way):** one or two Kani proof harnesses for the
     pure tick-regime arithmetic (`classify_actor_tick_regimes` exclusivity) if the
     adoption cost stays contained; otherwise the metamorphic property stands alone.
6. **Conformance index update:** rows for the tick-charge attribution authority (extended
   to action-emitted deltas), the projection freshness rule and provenance-class audit,
   the shrunken `WorldNoOp` allowlist contract, believed-access embodied availability, the
   numeric field-policy registry, the shared open-duration authority, and the generative
   lock tier.

## 6. Documentation corrections (housekeeping, same package)

- Amend `reports/0016_ord_life_cert_scoped_acceptance.md` with the dated correction
  section (`ORD-HARD-034`) and add conformance-index rows recording that 0016's
  divergence-fixture and upcast-fixture claims were overturned by this audit.
- Execution doc 06: add the single-charge-per-tick clause covering action-emitted awake
  deltas (execution-tier clarification; no doctrine amendment).
- Architecture doc 03 conformance notes: record the projection freshness rule as the
  single staleness semantics for both embodied and no-human consumers.

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0017_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. Per-actor need ledgers for `no_human_day_001` and the wait-boundary fixture,
   before/after `ORD-HARD-026`, with every golden checksum diff explained.
2. The provenance-class proof: an aged record surfacing as `remembered_belief` with its
   original acquisition tick, live and under replay; the `provenance_class_mismatch` and
   `provenance_witness_mismatch` fail-closed cases demonstrated.
3. The tamper-flip proofs for `ContinueRoutineAccepted`↔`Rejected` and a rewritten
   rejection reason, both poisoning replay; the capstone run with the context-hash
   assertion active.
4. The embodied divergence pair (belief-closed/truth-open and belief-open/truth-closed)
   rendered embodied and debug.
5. The numeric-policy census output and the new negative-fixture rejections.
6. The generative tier's first corpus summary: seed set, sequence count, reachability
   counters, zero differential divergences, metamorphic relations passing.
7. The pinned-mutants diff-gate run and the committed baseline miss-set.
8. The corrected 0016 report section.
9. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`; not
   full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment; INV-001…INV-110 are applied, not changed.
- Crate direction preserved: core depends on nothing at runtime; content on core; tui on
  core + content. Dev-dependencies for the generative tier do not alter the production
  dependency graph, but the zero-dependency posture decision is recorded (§9).
- Recommended ticket ordering: `ORD-HARD-033` first (the shared open-duration authority
  feeds 026's classifier), then `026` (golden checksums change once), then `027`/`031`
  (epistemic provenance), then `028` (replay evidence — its gates replay the corrected
  durations and provenance), then `029`/`030` (behavioral/content), then `032` and the
  generative tier last so the new locks exercise the finished surface, with `034`'s report
  corrections landing alongside the acceptance artifact.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **ORD-HARD-026 reprices the canonical day again.** Every golden no-human checksum
  changes; the per-actor ledger is the only honest review surface. Budget for fixture
  churn; land it early in the ordering so later gates replay corrected values.
- **ORD-HARD-027's remembered-belief downgrade may change planner choices.** A
  previously-"observed" stale food source becomes a remembered belief; if any selection
  logic weights observation over memory, traces and golden runs shift. That shift is the
  honest behavior; the ledger explains it.
- **Generative-tier dependency posture (open question):** `proptest` as a dev-dependency
  keeps the production graph empty but introduces external code into the test substrate;
  an in-tree seeded LCG generator is dependency-free but more work and weaker shrinking.
  Recommended default: in-tree generator for the differential/metamorphic oracles (they
  need valid sequences, not shrinking), revisit `proptest` only if failure triage proves
  painful. Record the choice in the conformance index either way.
- **Kani adoption is optional** and scoped to one pure function family; skip without
  prejudice if toolchain integration costs exceed an afternoon — the metamorphic property
  covers the same invariant probabilistically.
- **Severity-drift risk persists but the posture changes here.** This fifth pass found
  blocker/high issues *inside locks delivered by the fourth pass*. The generative tier
  exists to break that recurrence: future passes should verify properties over generated
  histories rather than auditing ever-more recorded ones. A sixth, verification-only audit
  after 0017 lands is still warranted, and should be materially cheaper.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` and names responsible layers from
  the execution diagnostic vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted); every blocker/high finding was independently re-verified against
  the code at the target baseline, not taken from delegated review alone.
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression.
- [x] Verified-holding items from 0014/0015/0016 are recorded to prevent re-litigation;
  overstated 0016 acceptance claims are explicitly recorded as overturned
  (`ORD-HARD-034`), not silently rewritten.
- [x] No doctrine amendment; no compatibility shims; crate direction preserved.
- [x] Scope stays within the Phase 3A ordinary-life surface.

## Outcome

Completed: 2026-06-10

What changed:

- Landed shared duration-open authority, duplicate-terminal rejection, and
  single-charge tick-ledger coverage for action-emitted awake deltas.
- Added projection freshness classification, provenance-class and witness
  audits, replay payload materialization, and tamper-flip replay gates.
- Added believed-access embodied availability, content numeric/string policy
  registries, lock-layer durability updates, and a bounded in-tree generative
  lock tier.
- Added 0017 conformance rows, corrected the overstated 0016 acceptance
  evidence, and created `reports/0017_ord_life_cert_scoped_acceptance.md`.
- Refreshed `.cargo/mutants-baseline-misses.txt` to the raw finished-tree
  missed-mutant set consumed by the scheduled mutation workflow.

Deviations from original plan:

- The generative tier uses an in-tree seeded LCG generator rather than
  `proptest`; this preserves the production and test dependency posture for
  this pass.
- Kani was skipped without prejudice; the acceptance report records the
  bounded metamorphic generator as the delivered lock tier.
- The mutation baseline had to be refreshed during capstone closeout because
  the committed file was Markdown-formatted while CI diffs raw
  `mutants.out/missed.txt`.

Verification results:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.
- `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' --no-shuffle` completed with 937 tested, 593 caught, 148 missed, 195 unviable, and 1 timeout.
- `diff -u .cargo/mutants-baseline-misses.txt mutants.out/missed.txt` passed
  after the baseline refresh.
