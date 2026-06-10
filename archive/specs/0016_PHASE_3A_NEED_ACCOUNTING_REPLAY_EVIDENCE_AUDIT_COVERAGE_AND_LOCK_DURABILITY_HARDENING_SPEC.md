# 0016 Phase 3A Need Accounting, Replay Evidence, Audit Coverage, and Lock Durability Hardening Spec

**Status**: COMPLETED
**Staging path:** `specs/0016_PHASE_3A_NEED_ACCOUNTING_REPLAY_EVIDENCE_AUDIT_COVERAGE_AND_LOCK_DURABILITY_HARDENING_SPEC.md`

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `ba84e75` (post-0015 closeout; all `0015PHA3AEVECOG` tickets landed).
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the fourth Phase 3A alignment pass. Spec 0014 closed `ORD-HARD-001`–`007`;
spec 0015 closed `ORD-HARD-008`–`013`. This audit re-derived the normative contract from
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, and `docs/3-reference/*`,
re-examined the full surface introduced by archived spec 0005 at the local baseline, and —
per the audit directive — treated every prior correction (including 0015's) as unverified
until proven in code. Findings continue the `ORD-HARD` series at `014`.

All evidence below was verified against local sources at the target baseline. Citations
use named symbols, which are grep-stable; line numbers are omitted deliberately.

Two of 0015's recorded "verified holding" claims are **overturned** by this audit and are
re-opened as findings here:

- "Passive need decay … do[es] not double-count sleep ticks" — refuted; see `ORD-HARD-014`.
- The 0015 outcome claim that "context-hash rebuild checks lock the event-backed cognition
  channel" — refuted; the gate is tautological; see `ORD-HARD-016`.

## 1. Scope

### In scope

- Need accounting across windows and scheduled durations
  (`crates/tracewake-core/src/scheduler.rs` no-human runner, `src/time.rs`,
  `actions/defs/sleep.rs`, `actions/defs/work.rs`, `actions/defs/wait.rs`).
- Body-exclusive reservation lifecycle (`actions/pipeline.rs`).
- Replay evidence quality: the decision context-hash gate, rebuild ordering,
  `WorldNoOp` agent event kinds, error accumulation
  (`src/replay/*`, `src/events/*`, `src/checksum.rs`, the golden/capstone tests).
- Severe-safety goal resolution (`agent/generation.rs`, `agent/methods.rs`,
  `agent/transaction.rs`).
- Residual unbacked-fact channels on the sealed actor-known surface
  (`agent/no_human_surface.rs`, `agent/actor_known.rs`).
- Hidden-truth audit coverage and ordering (`agent/decision.rs`, `agent/transaction.rs`,
  `agent/htn.rs`, `actions/pipeline.rs`).
- Embodied workplace availability and embodied need display (`src/projections.rs`,
  `src/view_models.rs`, TUI render insofar as it consumes them).
- Cognition/epistemic substrate unification status and its recorded-deferral obligation.
- Content-schema numeric validation, latent tuning defaults, and free-text channels
  (`crates/tracewake-content/src/schema.rs`, `validate.rs`; `tracewake-core/src/state.rs`).
- Stuck-actor cross-tick detection and wait discipline.
- Durability of the anti-contamination lock layer itself
  (`crates/tracewake-core/tests/anti_regression_guards.rs`,
  `tests/negative_fixture_runner.rs`, `clippy.toml`, CI).

### Out of scope

- Re-auditing Phase 1 / Phase 1A spine internals (covered by 0010–0012).
- Re-auditing Phase 2A epistemic internals (covered by 0013), except where Phase 3A
  must consume that substrate instead of forking it (`ORD-HARD-021`).
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014/0015 fixed that this audit verified as holding
  (see §3 "Verified holding").

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-009, INV-015, INV-017/018/020,
  INV-024, INV-034/035/036/037, INV-039/040/041, INV-043/044/045, INV-048, INV-061,
  INV-063, INV-067/069, INV-080, INV-099…INV-110 (especially INV-102 provenance,
  INV-103 scheduler limits, INV-105 diagnostics are authoritative, not decorative).
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` —
  "a string label is never sufficient proof"; the decision-transaction contract.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` —
  the embodied-affordance formula; the holder-known projection as the cognition source.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` —
  scheduler powers and limits; reservation semantics.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` —
  TFW provenance table; anti-contamination gate inventory.
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` —
  ORD-LIFE-CERT clauses; stuck-diagnostic and wait-reason requirements.

External research consulted for lock design, not authority override:

- Orkin, "Three States and a Plan: The A.I. of F.E.A.R." (sensor→working-memory seam):
  `https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf`
- Ryan et al., Talk of the Town knowledge-provenance model:
  `http://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf`
- Marten projection rewind-and-rebuild testing: `https://martendb.io/events/projections/testing`
- FoundationDB / TigerBeetle deterministic simulation testing:
  `https://apple.github.io/foundationdb/testing.html`,
  `https://tigerbeetle.com/blog/2023-07-06-simulation-testing-for-liveness`
- Trail of Bits, dylint custom lints:
  `https://blog.trailofbits.com/2021/11/09/write-rust-lints-without-forking-clippy/`
- cargo-mutants (mutation testing — guards must kill mutants): `https://mutants.rs/`
- Sealed traits / typestate / capability-token patterns:
  `https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/`,
  `https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html`

## 3. Determination

**Positive verdict.** The in-scope Phase 3A ordinary-life substrate is not fully aligned
with the foundation pack and not maximally locked. Twelve findings follow,
`ORD-HARD-014` … `ORD-HARD-025`.

The root pattern behind the two largest findings: **0015 fixed the channel but not the
ledger or the evidence.** Cognition inputs are now genuinely event-backed, but the need
arithmetic those decisions run on is double-charged across durations, and the replay gate
that was supposed to prove the channel rebuilds from the log compares a stored value with
itself.

### Verified holding (no action; recorded so they are not re-litigated)

- `ORD-HARD-008` core fix holds: `NoHumanActorKnownSurfaceBuilder::from_event_log`
  consumes only the event log and `AgentState`; raw-table reads are gone; proven by
  `raw_workplace_assignment_is_not_actor_known_without_notice` and
  `current_place_is_not_implicit_sleep_knowledge`. (But see `ORD-HARD-018` for residual
  unbacked side channels on the same builder.)
- Seed-time prehistory knowledge events are real: `seed_event_log` emits
  `StartingBeliefRecorded`/`RoleAssignmentNoticeRecorded`/`InitialBeliefSeeded` with
  `source_kind=authored_prehistory`; content depends on core only (Cargo.toml verified).
- Modeled perception is evented and deterministic:
  `record_current_place_perception` runs before `build_agent_proposal`; proven by
  `current_place_perception_emits_window_keyed_observation_events` and
  `current_place_perception_appends_byte_identically`.
- `ORD-HARD-009` fail-closed core holds: `ActorDecisionTransaction::run` returns typed
  `Stuck { blocker_code: HiddenTruthInput }` with no proposal on a dirty audit; proven by
  `forbidden_actor_known_input_fails_closed_before_proposal`. (But see `ORD-HARD-020`
  for audit scope/ordering gaps around it.)
- `ORD-HARD-010` holds for food/sleep/route: `phase3a_semantic_actions` reads
  `actor_known_food_sources` / `actor_known_sleep_affordances` / `actor_known_routes`;
  proven by `embodied_view_omits_unobserved_food_at_open_place`,
  `embodied_view_omits_unknown_sleep_affordance`,
  `embodied_exits_require_perceived_or_known_route`. (Workplace availability is the
  exception — `ORD-HARD-019`.)
- `ORD-HARD-011` continuity core holds: `sleep_interruption_reason` and
  `work_completion_failure` take live state, and severe-need interruption is reachable;
  proven by `sleep_completion_interrupts_on_severe_need_with_prorated_recovery` and
  `work_completion_fails_when_actor_displaced_with_prorated_costs`. (But the failure
  event leaks the reservation — `ORD-HARD-015` — and proration interacts with the
  double-count — `ORD-HARD-014`.)
- `ORD-HARD-012` holds: `need_model` and sleep tuning are required content fields
  (`MissingField("need_model")`, positional sleep-tuple rejection); core consumes
  authored values. (But latent `Default` constructors remain — `ORD-HARD-022`.)
- Determinism layer: zero `HashMap`/`HashSet` in core production sources; `AgentState`
  is all `BTreeMap`; `OrderingKey` covers tick/phase/source/sequence; tie-breaks proven
  by `stable_ids_terminate_ties` and `repeated_selection_is_deterministic`.
- Needs bounded: `clamp_need_value` with exhaustive proof
  (`apply_delta_clamps_without_underflow_or_overflow`); bands match Phase 3A boundaries.
- Intention durability: at most one active intention (`adopt_active` rejects a second);
  `ContinueCurrentIntention` outranks mild pressures; switches emit typed lifecycle
  effects and traces.
- Planner discipline: `plan_route` reads only `state.known_edges()` of the planning
  context; typed `PlannerBudgetExhausted`; no truth fallback
  (`believed_but_wrong_food_source_fails_without_truth_lookup`).
- Debug capability privacy: `DebugCapability`/mutation-capability minting is
  crate-private with compile-fail fixtures
  (`external_crate_cannot_forge_mutation_capability`).
- Possession parity: `bind_actor` attaches a controller binding only; human and agent
  proposals share `run_pipeline` validation (INV-108).
- CI: fmt/clippy/test/lock-layer jobs on every `pull_request`, no path filters,
  `-D warnings`, `--locked`; toolchain pinned once in `rust-toolchain.toml`.
- Checksum census: `new_authoritative_field_without_checksum_registry_fails` makes
  adding an unregistered authoritative field a test failure.

## 4. Findings and remediation requirements

### ORD-HARD-014 — Passive awake need deltas double-charge ticks spanned by accepted durations

**Severity:** blocker for scoped `ORD-LIFE-CERT` evidence.

**Responsible layers:** `scheduler`, `intention_lifecycle`.

**Doctrine breached:** INV-039, INV-043/044/045 (need changes must be causally honest;
no fake accounting), INV-009 (the doubled deltas are eventful but causally false —
event ancestry asserts "awake time" for ticks the actor verifiably spent asleep).
Overturns 0015 §3's "do not double-count sleep ticks" verified-holding claim.

**Evidence:** In `scheduler.rs::no_human::run_no_human_day`, every window × actor calls
`append_passive_need_events_before_decision` with
`elapsed_ticks = window.start_tick − last_decision_tick`, unconditionally applying
`passive_awake_need_deltas` (authored awake hunger/fatigue rates). There is no check for
an open `SleepStarted`/`WorkBlockStarted` spanning those ticks. `append_due_completions`
runs immediately after, and `build_sleep_completion_events` /
`build_work_completion_events` independently apply per-tick recovery/cost for the same
span. Net effect: a slept tick is charged awake hunger (+rate) *and* sleep hunger-rise;
awake fatigue (+rate) *and* sleep recovery (−rate). `clamp_need_value(0, 1000)` masks the
arithmetic at the bounds, so extreme states hide the error while mid-range states encode
it into golden checksums.

**Why existing gates miss it:** the only direct completion-builder invocations are unit
tests in `sleep.rs`/`work.rs` with hand-picked ticks that never cross a window boundary.
The golden no-human runs *do* drive `append_due_completions` through window boundaries
spanned by open durations (`no_human_day_001` holds a sleep spanning ticks 24–32 and a
work block spanning 10–18), but they assert only byte-stability — the double-charged
values are already encoded in today's golden checksums;
`passive_awake_need_deltas_are_deterministic_and_non_reducing` asserts sign only. Replay
byte-matches the wrong values consistently, so determinism gates cannot see a semantic
error.

**Required correction:** introduce a single per-actor elapsed-tick accounting authority:
for each charge interval `(from, to]`, classify every tick as awake / asleep / working
from the open-duration events in the log, and route **all** need deltas (passive awake,
sleep recovery, sleep hunger-rise, work cost) through it so each tick is charged by
exactly one regime. Passive awake deltas apply only to awake-classified ticks. Completion
and interruption proration consume the same classifier, so `ORD-HARD-011`'s prorated
paths stay consistent. Golden checksums will change; each fixture diff must be explained
in the acceptance artifact with a per-actor need ledger.

**Structural lock:**

- A need-ledger invariant test: for every no-human golden run, reconstruct per-actor
  per-tick charge attribution from the event log and assert no tick is charged by two
  regimes; run it over `no_human_day_001` and an adversarial fixture whose sleep spans a
  window boundary (`sleep_spanning_window_boundary_charges_each_tick_once_001`).
- Source guard (positive presence + runtime): the passive-delta path must reference the
  tick classifier; a runtime test feeds a mid-sleep window and asserts the passive event
  payload's `elapsed_ticks` excludes slept ticks.

### ORD-HARD-015 — `WorkBlockFailed` never closes its body-exclusive reservation

**Severity:** high.

**Responsible layers:** `action_validation`, `scheduler`.

**Doctrine breached:** INV-043/045; architecture doc 04 reservation semantics
(a typed `scheduling/reservation` blocker must reflect a *live* conflict).

**Evidence:** `actions/pipeline.rs::body_exclusive_reservation_conflict` builds
`closed_starts` from causes of `SleepCompleted | SleepInterrupted | WorkBlockCompleted`
only. `work_completion_failure` emits `WorkBlockFailed` (caused by the original
`WorkBlockStarted`), which is absent from the closed set — the start stays "open"
forever, and every subsequent body-exclusive proposal (sleep, work) for that actor is
rejected with `ReservationConflict` for the rest of the run.

**Why existing gates miss it:** `overlapping_body_exclusive_action_is_reservation_conflict`
exercises only the positive conflict on a still-running block; no test fails a work block
mid-duration and then proposes a later body-exclusive action.

**Required correction:** derive duration open/closed pairing from a single
`is_duration_terminal(kind)` predicate (terminal set: `SleepCompleted`,
`SleepInterrupted`, `WorkBlockCompleted`, `WorkBlockFailed`, plus any future variants),
shared by the reservation check and the completion appender so they cannot drift.

**Structural lock:**

- The predicate matches on `EventKind` without a wildcard arm for duration kinds, so a
  new duration event kind fails compilation until classified.
- Fixture `work_block_failed_then_sleep_succeeds_001`: displaced work block fails, the
  actor later sleeps successfully; replay byte-match.

### ORD-HARD-016 — The decision context-hash replay gate is tautological

**Severity:** high.

**Responsible layers:** `test_oracle`, `replay`.

**Doctrine breached:** INV-018, INV-102, INV-105 (a proof artifact that gates nothing is
a decorative proof surface). The 0015 `ORD-HARD-008` structural lock — "a rebuild test
that recomputes every decision's context from replayed events and byte-matches the
recorded hash" — was not implemented as specified.

**Evidence:** `crates/tracewake-content/tests/golden_fixtures_run.rs::no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`
computes `compute_holder_known_context_hash(record.actor_known_inputs.clone())` and
asserts equality with `record.actor_known_context_hash` — but
`DecisionTraceRecord::from_trace` and `deserialize_canonical` (`agent/trace.rs`) derive
the stored hash from that same stored input vector. Both sides read identical strings;
the assertion is `hash(x) == hash(x)` and can never fail. The only non-tautological part
is the cited-`source_events`-exist check, which proves citation integrity, not
derivability. No test re-derives any decision's context from the replayed log.

**Required correction:** during replay rebuild, at each recorded decision frontier,
re-run the production surface builder (`NoHumanActorKnownSurfaceBuilder::from_event_log`)
against the replayed event prefix and rebuilt `AgentState`, serialize the resulting
context inputs, and byte-match the recorded `actor_known_context_hash`. Every decision in
every golden no-human run, not a sample.

**Structural lock:**

- The rebuilt-hash gate lives in the replay path (`replay/rebuild.rs` or the golden
  runner) and uses the production builder — no parallel reimplementation.
- Tamper test: corrupt one seed knowledge event in a copied log and assert the gate
  fails at the first affected decision (proves the gate can fail).
- Stop persisting trust in stored inputs: `deserialize_canonical` may keep its
  consistency check, but the acceptance gate is the from-log re-derivation.

### ORD-HARD-017 — Severe safety pressure resolves to waiting, never flight

**Severity:** high.

**Responsible layers:** `proposal_construction`, `intention_lifecycle`.

**Doctrine breached:** INV-039 (pressures must be able to redirect behavior — a pressure
that cannot produce its own remedy is a label, not a pressure); foundation doc 05 and
0005 §8.1 safety semantics ("a pressure to avoid or *leave* obviously unsafe …
situations"); 0005 §11.1 (`leave unsafe/blocked/unknown situation` is a required
candidate goal with real behavior).

**Evidence:** `agent/generation.rs` emits `GoalKind::LeaveUnsafePlace` at
`GoalPriority::SevereSafety` (rank 1), but `agent/methods.rs::family_for_goal` maps
`LeaveUnsafePlace → RoutineFamily::Wait` and
`agent/transaction.rs::planner_goal_for` maps it to
`PlannerGoal::WaitWithReason("actor_decision_reevaluation")`. The single
highest-priority survival goal in the vocabulary plans an in-place wait.

**Why existing gates miss it:** method-selection tests assert a coherent proposal is
produced, never that the safety goal's planned `action_id` is `move`.

**Required correction:** add a `LeaveUnsafePlace` routine family whose step moves the
actor along a *known* edge (actor-known routes only, per INV-048) toward a
known/believed-safer place; when no known edge exists, fall back to wait with a typed
`knowledge` blocker and a stuck diagnostic — an explained inability, not a silent one.

**Structural lock:**

- Fixture `severe_safety_with_known_exit_produces_move_001`: severe safety pressure +
  one known edge ⇒ committed `move` proposal with trace ancestry; replay byte-match.
- Fixture `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`.

### ORD-HARD-018 — Unbacked fact channels survive on the sealed actor-known surface

**Severity:** high.

**Responsible layers:** `holder_known_context`.

**Doctrine breached:** INV-101, INV-102, INV-063; foundation doc 14 ("a string label is
never sufficient proof").

**Evidence (all in `crates/tracewake-core/src/agent/`):**

- `no_human_surface.rs::with_role_assignment_notice` and `::with_sleep_place_knowledge`
  are `pub`, re-exported through `agent/mod.rs`, and mint facts labeled
  `remembered_belief`/`modeled_observation:*` with `Vec::new()` for `source_event_ids`.
  They have zero callers at the baseline but remain a live production API on the
  certified surface — an authoring convenience one refactor away from re-opening the
  raw-truth channel under provenance-claiming labels.
- `ActorKnownFact::with_source_event_ids` stores ids without any consumer validating
  that they resolve to events in the log; the runtime contract "every consumed fact
  cites real events" is enforced only by one golden test's string parsing, not at the
  transaction boundary.
- `actor_known.rs::observe_visible_local` stamps every entry of caller-supplied
  `VisibleLocalPlanningState` as `ActorKnownFact::observed_now`; provenance is asserted
  by the constructor's caller, not derived. The production no-human path does not use
  it, but `build_actor_known_planning_state` is `pub` and would launder raw adjacency
  into "observed" facts that pass the audit.

**Why existing gates miss it:** the guard families ban `extend_actor_known_facts`,
`add_actor_known_fact`, and `PhysicalState` tokens; these methods use none of those
tokens. The audit checks the provenance *enum*, which these channels set to an
actor-known class at construction.

**Required correction:**

1. Delete `with_role_assignment_notice` and `with_sleep_place_knowledge`.
2. Make unbacked facts unconstructable: fact-insertion paths take a non-empty,
   typed source-id witness (e.g. `SourceEventIds` newtype constructable only from
   `&EventEnvelope` references or a checked non-empty list) — the F.E.A.R.-style
   sensor→working-memory seam expressed in the type system.
3. Enforce resolution at the boundary: `ActorDecisionTransaction::run` (or the surface
   `seal` step) validates that every consumed fact's `source_event_ids` resolve against
   the log, failing closed with a typed `Stuck` (`blocker_code = provenance_dangling`);
   window-framing facts cite their genuine causal events, not a convenience frame marker.
4. Demote `VisibleLocalPlanningState::new`/`observe_visible_local` — and the `pub`
   laundering entry points `build_actor_known_planning_state` and
   `build_actor_known_planning_state_with_projection_limitation` — behind a
   perception-derivation function (event/projection-sourced); test-only raw construction
   moves to `#[cfg(test)]`.

**Structural lock:** compile-fail doctests for the deleted/demoted constructors
(mirroring `from_observed_parts`); negative fixture
`dangling_source_event_ids_fail_closed_001`; guard banning empty-source-id fact
construction in `agent/**` production sources.

### ORD-HARD-019 — Embodied workplace availability is ground-truth-gated; embodied need display leaks exact values

**Severity:** high.

**Responsible layers:** `view_model`, `projection`.

**Doctrine breached:** INV-067, INV-069; architecture doc 03's embodied-affordance
formula; foundation doc 08 (banded need display embodied; exact numbers debug-only).
Completes the unfinished half of `ORD-HARD-010`.

**Evidence (`crates/tracewake-core/src/projections.rs`):**

- `phase3a_semantic_actions` workplace branch iterates `source.actor_known_workplaces`
  (context-backed identity — the 0014/0015 fix) but then calls
  `state.workplaces.get(workplace_id)` and computes
  `enabled = at_workplace && workplace.access_open` — ground-truth location and
  open/closed truth decide the embodied affordance's availability. An actor with stale
  knowledge ("workplace closed") sees truth, not belief, the moment workplace facts
  reach the context.
- `phase3a_status` builds `NeedStatusEntry { value: need.value(), … }` and the embodied
  render prints `value={} band={}` — the exact internal scalar on the embodied surface.

**Why existing gates miss it:** the embodied-workplace tests assert *absence without
context facts* (`embodied_view_omits_raw_workplace_assignment_without_context`); no test
seeds a workplace fact with belief/truth divergence. Render tests assert the band label
is present, never that the raw scalar is absent.

**Required correction:** carry place and believed-access on the context's workplace
facts and compute embodied availability from them; debug projection may show the
belief-vs-truth comparison non-diegetically. Drop `value` from the embodied need entry
(band + dominant cause only); exact values remain in the debug needs report.

**Structural lock:** extend the `guard_014_embodied_projection_*` family to ban
`state.workplaces` in the embodied semantic-action builder; adversarial fixture
`embodied_workplace_availability_reflects_belief_not_truth_001` (truth open, belief
closed ⇒ embodied shows closed); render test asserting embodied output contains the band
but not the `value=` token.

### ORD-HARD-020 — Hidden-truth audit scope and ordering gaps

**Severity:** medium.

**Responsible layers:** `proposal_construction`, `action_validation`.

**Doctrine breached:** INV-102 ("for action-relevant cognition, a rejection condition"
— the condition must cover what cognition actually consumes); INV-099; foundation doc 14
(the provenance graph must be inspectable end-to-end).

**Evidence:**

- `ActorKnownPlanningContext` stores six structured fields (`known_food_sources`,
  `known_sleep_places`, `known_workplaces`, `known_edges`,
  `known_containers_by_place`, `known_closed_doors`) *independently* of
  `actor_known_facts`. The audit
  (`audit_with`, `hidden_truth_audit_from_actor_known_inputs`) inspects only the facts;
  the planner and `htn.rs::resolve_condition` read the structured fields directly. A
  context with populated structured fields and clean facts audits `actor_known_only =
  true` while driving real proposals. Today builders co-populate both — discipline, not
  structure.
- `actions/pipeline.rs::source_context_check` rejects agent-origin proposals only when
  `hidden_truth_audit_actor_known_only` is present **and** equals `"false"`; an absent
  key or any other value passes the defense-in-depth stage.
- In `ActorDecisionTransaction::run`, `select_phase3a_method` (which consumes the
  structured context) executes before the audit gate is evaluated; a forbidden input
  shapes method selection even though the proposal is ultimately blocked.
- `htn.rs::resolve_condition` returns satisfied unconditionally for
  `FixtureAuthoredPossibility` and `SharedPipelinePreconditions` — a latent
  knowledge-free template escape hatch (unused by `phase3a_routine_templates()` but
  reachable by any future template).

**Required correction:** make all six structured fields computed accessors over the
audited facts (single source of truth) — or have `audit_with` reject any structured
entry lacking a matching actor-known fact. Pipeline requires the audit parameter present and
`"true"` for agent-origin proposals (better: a typed audit stamp, not a stringly
parameter). Hoist the audit gate before method selection.
`FixtureAuthoredPossibility` requires a matching fixture-possibility-provenance fact.

**Structural lock:** a divergence test constructing (in-crate) a context whose
structured fields exceed its facts and asserting the audit fails; pipeline negative test
with the parameter absent; an ordering assertion that no method-selection symbol
executes pre-audit (runtime: forbidden input ⇒ trace records no method candidates).

### ORD-HARD-021 — Phase 3A cognition and embodied perception fork the epistemic substrate; the 0015 deferral was never recorded

**Severity:** medium.

**Responsible layers:** `holder_known_context`, `projection`.

**Doctrine breached:** foundation doc 14 and architecture doc 03 (the holder-known
projection is the cognition source); 0015 §9's own condition — "deferring deeper
unification with Phase 2A belief structures to a recorded follow-up, not silently."

**Evidence:**

- `NoHumanActorKnownSurfaceBuilder` re-derives knowledge by parsing raw event payloads
  (`consume_observation`, `consume_starting_belief`, `consume_role_assignment_notice`)
  into its own tables, bypassing `EpistemicProjection`/`KnowledgeContext` — a parallel
  shadow store over the same events. Event-backed (so INV-102 holds) but structurally
  forked from the substrate the rest of the system uses.
- `agent/perception.rs::current_place_knowledge_context` (consumed by
  `actions/pipeline.rs` and `actions/defs/continue_routine.rs`) recomputes embodied food/sleep/route
  facts from live state at view time, not from the durable epistemic projection: a food
  supply removed from truth this tick vanishes from the menu without the actor learning
  it; one added appears with no perception latency. Same-tick coincidence of
  perception == truth is what the tests cover; multi-tick staleness is untested and
  wrong.
- The promised recorded follow-up does not exist: `tickets/` contains only the template
  and README; `reports/0015_ord_life_cert_scoped_acceptance.md` does not mention the
  cut; no conformance-index row records the fork. The deferral lives only in archived
  ticket prose.

**Required correction:** unify on the holder-known projection: the no-human surface
builder and the embodied context builder both consume the Phase 2A epistemic projection
(observations/beliefs with staleness), not parallel re-derivations. If implementation
proves full unification too large for this pass, the minimum acceptable cut is:
(a) embodied facts gain multi-tick durability/staleness from the projection, and
(b) the remaining unification is recorded as a real ticket in `tickets/` plus a
conformance-index row — the recording is **not optional this time** and is itself an
acceptance criterion.

**Structural lock:** multi-tick fixture
`embodied_menu_lags_truth_change_without_perception_001` (truth changes; no perception
event; embodied output must not change until perception fires); conformance-index row
for the cognition-substrate source; ticket-existence check in the acceptance artifact.

### ORD-HARD-022 — Content validation permits refill-direction tuning, latent kernel defaults, and unscanned prose channels

**Severity:** medium.

**Responsible layers:** `content_seeding`, `action_validation`.

**Doctrine breached:** INV-061, INV-080; 0005 §16.5's rejection class ("hunger refills
without food/service/action" — reachable today through legal *numeric* fields);
foundation doc 09 (no simulation fact born from prose; "do not hide scripts behind
prettier names").

**Evidence:**

- `validate.rs::validate_state` checks `duration_ticks == 0` but imposes no sign or
  bound on `awake_hunger_delta_per_tick`, `awake_fatigue_delta_per_tick`,
  `fatigue_recovery_per_tick`, `hunger_rise_per_tick`, workplace
  `fatigue_delta_per_tick`/`hunger_delta_per_tick`, or
  `hunger_reduction_per_serving`. `awake_hunger_delta_per_tick: -50` is a passive
  hunger refill with no food ancestry and no banned word — the forbidden-content scans
  (`PHASE3A_SHORTCUT_MARKERS`, `is_script_key`) are name blacklists and never see it.
- Latent kernel tuning: `NeedModelState::default()` returns `{5, 3}`;
  `SleepAffordanceState::new` and `WorkplaceState::new` hardcode tuning values;
  `PhysicalState::from_seed_parts` seeds `NeedModelState::default()`. The content path
  overrides all of them, but the defaults remain reachable by any non-content
  construction site — the `ORD-HARD-012` shortcut surviving as fallbacks.
- `PlaceSchema.display_label` flows into `PlaceState` unvisited by `validate_no_script`,
  `contains_prose_born_fact_marker`, or `reject_script_marker_text` — a free-text
  channel into authoritative state.

**Required correction:** directional and bounded numeric validation in `validate_state`
(pressure-direction deltas `>= 0`, relief-direction fields `>= 0` in their modeled
direction, magnitude caps); delete `Default for NeedModelState` and make tuning-bearing
constructors take all tuning as required parameters (`from_seed_parts` takes an explicit
`NeedModelState`); route `display_label` (and every fixture `String` field) through the
script/prose marker scans via a must-be-scanned field registry.

**Structural lock:** negative fixtures `fixture_negative_awake_delta_rejected_001` and
`fixture_display_label_script_marker_rejected_001`; a schema test enumerating every
`String` field in `FixtureSchema` against the scanned-field registry so a new free-text
field fails until classified; no `Default`/zero-arg constructor for tuning-bearing state
structs (compile-level).

### ORD-HARD-023 — Stuck-actor detection lacks its cross-tick categories; wait discipline gaps

**Severity:** medium.

**Responsible layers:** `intention_lifecycle`, `scheduler`, `action_validation`.

**Doctrine breached:** INV-040/041 and execution doc 06's stuck-diagnostic clauses
(0005 §8.8 mandated diagnostics for "no progress past expected progress window" and
"repeats idle/wait without typed reason"; §9.10 made autonomous wait reasons mandatory).

**Evidence:**

- All current stuck diagnostics are single-window fail-closed cases (no candidate, no
  method, hidden truth, plan failure, budget). `RoutineExecution` carries
  `last_progress_tick`, `expected_next_progress_tick`, and `fallback_attempts`, but no
  code consumes them — the no-progress and repeated-idle detectors do not exist.
- `wait.rs::build_wait_events` fills a missing reason with
  `unwrap_or("unspecified_wait")` — the action layer fabricates the reason the doctrine
  requires the actor to supply; a reasonless autonomous wait is accepted, not rejected.
- `build_threshold_events` reads `current_hunger`/`current_fatigue` from proposal
  parameters; absent or malformed params silently skip the crossing event (and the
  parameters are forgeable in principle), instead of recomputing from authoritative
  `AgentState`.
- `run_no_human_day` generates a decision proposal for an actor every window even while
  a body-exclusive duration is open; the rejection churn pollutes progress accounting.
- `payload_i32` panics on absent/malformed payload fields in completion builders — a
  latent crash where a typed application failure is required.

**Required correction:** a cross-window no-progress detector consuming the existing
`RoutineExecution` fields plus a repeated-idle counter, emitting the canonical
stuck-diagnostic categories; reject reasonless autonomous waits with a typed reason
code; recompute threshold crossings from `AgentState` inside the wait/duration paths
(ignore proposal-supplied need params, as work validation already does); skip decision
generation for actors with an open body-exclusive duration spanning the window; convert
`payload_i32` panics into typed event-application errors.

**Structural lock:** fixtures `repeated_wait_without_progress_emits_stuck_001` and
`stalled_intention_past_expected_progress_emits_stuck_001`; a rejection test for a
reasonless Scheduler-origin wait; a forged-need-param test asserting authoritative
recomputation; capstone extension asserting the canonical day contains zero
`unspecified_wait` reasons.

### ORD-HARD-024 — Replay robustness gaps: trusted ordering, unverifiable lifecycle kinds, error accumulation

**Severity:** medium.

**Responsible layers:** `replay`, `test_oracle`.

**Doctrine breached:** INV-018, INV-020; foundation doc 03 (reject unsupported history;
replay failure is loud); 0005 §15 (replay must reconstruct lifecycle state, and a replay
that silently drops a Phase 3A event class is a failure).

**Evidence:**

- `replay/rebuild.rs::rebuild_projection` iterates stored `Vec` order and never
  validates `global_order` monotonicity; `EventLog` deserialization re-appends events,
  *reassigning* `global_order`/`stream_position` from load order — a reordered
  serialized log is undetectable by construction.
- `events/apply.rs::apply_agent_event` returns `Ok(WorldNoOp)` for
  `NeedThresholdCrossed`, `CandidateGoalsEvaluated`, `SleepStarted/Completed/Interrupted`,
  `FoodConsumed`, `FoodServiceUsed`, `EatFailed`, `WorkBlockStarted/Completed/Failed`,
  `ContinueRoutine{Proposed,Accepted,Rejected}`, `NoHumanDay{Started,Completed}`.
  Excepting `FoodConsumed` — which is world-applied on the physical stream and captured
  by the physical checksum, so the allowlist census must classify it as a dual-stream
  kind — these kinds mutate no rebuilt projection and sit outside both checksums; their
  semantic content is unverifiable under replay except via decision-trace strings.
- On agent/epistemic application errors, `rebuild_projection` accumulates and continues,
  producing a usable `final_agent_state`/`final_checksum` from a partial replay; the
  capstone institutionalizes filtered "marker" errors (`non_marker_agent_errors`).
- `EVENT_SCHEMA_REGISTRY` has a single V1 entry with `CurrentNoMigrationRequired`; the
  migration/loud-failure branch is vacuously untested.

**Required correction:** replay asserts stored `global_order` is monotonic and
deserialization verifies (not reassigns) stored ordering, failing loudly on mismatch;
add a registry test asserting no agent `EventKind` applies as `WorldNoOp` unless named
in an explicit allowlist whose entries each cite the projection/checksum that captures
their effect (and shrink that allowlist by materializing sleep/work episode and
threshold-crossing state into `AgentState` where 0005 §15.1 requires reconstruction);
rebuild marks the report non-authoritative (and `matches_expected` fails) on the first
agent/epistemic application error rather than building past it; add a synthetic V0→V1
upcast fixture locking the migration path's loud-failure contract.

**Structural lock:** gates `reordered_log_is_detected_001`,
`corrupt_midstream_agent_event_poisons_rebuild_001`; the `WorldNoOp` allowlist census
test; the upcast fixture.

### ORD-HARD-025 — Lock-layer durability: census perimeter, alias evasion, string-presence guards, unregistered fixtures

**Severity:** medium.

**Responsible layers:** `test_oracle`.

**Doctrine breached:** none directly; lock-layer durability (the anti-contamination
measure itself must resist drift). Continues `ORD-HARD-013`.

**Evidence:**

- `anti_regression_guards.rs::production_sources()` roots at `tracewake-core/src` only,
  and `is_guarded_layer_source` covers `src/agent/`, `src/scheduler*`,
  `src/projections*`. Cognition logic placed in a sibling file (`src/cognition.rs`), in
  `tracewake-content`/`tracewake-tui`, or in a new crate escapes every targeted
  truth-firewall rule; the census only fails for unclassified files *inside* the three
  globs — relocation just outside them is the natural evasion.
- Token bans are source-text scans: `use crate::state::PhysicalState as Truth;` defeats
  `assert_absent(…, "PhysicalState")`; wrapper functions in unguarded files and macro
  indirection defeat the rest. The `production()` cfg-stripper recognizes only the
  literal `#[cfg(test)]` form.
- Several positive-presence guards assert substring presence
  (e.g. the transaction audit-gate guard checks
  `contains("!selection.trace.hidden_truth_audit_result.actor_known_only")`) — a comment
  satisfies them. Most have runtime backstops (the audit and embodied families do); the
  guard layer itself should not depend on remembering which do.
- `crates/tracewake-core/tests/negative_fixture_runner.rs::FIXTURES` is a
  hand-maintained array; nothing reconciles it against the repo-root
  `tests/negative-fixtures/` directory on disk (25 == 25 today by discipline) — a
  deleted entry or unregistered new fixture passes silently; nor is there parity between
  `clippy.toml` ban entries and proving fixtures.
- `clippy.toml` lacks `rand` entry points (caught only by the core-only text scan —
  content/tui are uncovered for `rand`), `std::fs::write`/`OpenOptions`,
  `std::env::var`.

**Required correction:**

1. **Workspace-wide census with classification:** every production source file across
   all three crates is classified (guarded-layer / exempt-with-recorded-rationale) in a
   census table the suite asserts equals the actual tree — relocating cognition out of a
   guarded glob forces a reviewed reclassification.
2. **Type-level boundaries over text bans where it matters most:** the corrections in
   `ORD-HARD-018`/`020` (non-empty source-id witnesses, computed structured accessors,
   sealed constructors) convert the highest-value string bans into compile errors; keep
   the text scans as tripwires and document their known evasion modes inline.
3. **Negative-fixture census:** a test asserting a `read_dir` of the repo-root
   `tests/negative-fixtures/` directory (resolved via `CARGO_MANIFEST_DIR/../..` from
   the core test crate) equals `FIXTURES` exactly, plus a parity test that every
   `clippy.toml` `disallowed-types`/`-methods` entry has at least one proving fixture.
4. **Mutation-testing the lock layer:** add a `cargo mutants` configuration scoped to
   the guarded layers (`agent/**`, `scheduler*`, `projections*`, `pipeline.rs`) with a
   recorded baseline of caught/missed mutants in the acceptance artifact; a guard that
   kills no mutants is theater. Run as a non-PR scheduled/manual CI job if PR latency is
   a concern, but the baseline run is mandatory for this spec's acceptance.
5. **clippy.toml additions:** `rand::*` entry points, `std::fs::write`,
   `std::fs::OpenOptions`, `std::env::var`; extend the banned-token scan (or clippy
   config) to content and tui production sources for nondeterminism classes.
6. Where a positive-presence guard has no runtime backstop, add one or replace the
   string check with the runtime test.

**Structural lock:** the census tests above are themselves the lock; the mutants
baseline makes the lock layer's effectiveness measurable across future iterations.

## 5. Anti-contamination lock layer (consolidated)

1. **Compile-time impossibility (preferred tier):** non-empty `SourceEventIds` witness
   for fact construction; structured context fields as computed accessors over audited
   facts; no `Default`/zero-arg constructors for tuning-bearing state; duration-terminal
   classification via wildcard-free `EventKind` match; sealed/demoted raw constructors
   with compile-fail doctests. The duration-terminal predicate (`ORD-HARD-015`) is the
   single open/closed classification authority: `ORD-HARD-014`'s tick-regime classifier
   and `ORD-HARD-023`'s open-duration window-skip both consume it, so the classification
   cannot drift three ways.
2. **Runtime gates:** dangling-provenance fail-closed in the transaction; need-ledger
   single-charge invariant; from-log context-hash re-derivation at every decision;
   rebuild poisoning on first application error; reordered-log detection.
3. **Source guards** (extend `anti_regression_guards.rs`, `guard_016_*`): embodied
   `state.workplaces` ban; empty-source-id construction ban in `agent/**`;
   workspace-wide census; documented evasion modes for retained text scans.
4. **Negative/adversarial fixtures:** the fourteen named in §4, run through the
   existing fixture runners, with the fixture-directory census closing the registration
   hole.
5. **Mutation-testing baseline:** `cargo mutants` over the guarded layers, results
   recorded in the acceptance artifact; missed mutants on guard-relevant code become
   explicit accepted-risk entries or new tests.
6. **Conformance index update:** rows for need-tick accounting authority, duration
   terminal-set, context-hash re-derivation gate, cognition-substrate source (or its
   recorded deferral), embodied workplace availability source, and the workspace census.

## 6. Documentation corrections (housekeeping, same package)

- Record in the conformance index that 0015's "passive deltas do not double-count" and
  "context-hash rebuild" verified-holding/outcome claims were overturned by this audit,
  citing `ORD-HARD-014`/`016` — audits must not silently rewrite prior audit history.
- If `ORD-HARD-021` takes the minimum cut, the unification deferral gets a real ticket
  under `tickets/` and a conformance-index row (acceptance criterion, not advice).
- Execution doc 06: add the cross-tick stuck-detection categories
  (no-progress-past-expected-window, repeated-idle) to the ORD-LIFE-CERT clause list if
  not already explicit, as an execution-tier clarification — no doctrine amendment.

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0016_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. Per-actor need ledgers for `no_human_day_001` and the window-spanning sleep fixture,
   before/after `ORD-HARD-014`, with every golden checksum diff explained.
2. The tamper test demonstrating the context-hash re-derivation gate failing on a
   corrupted seed event, and passing on the genuine log (proof the gate can fail).
3. The `work_block_failed_then_sleep_succeeds_001` event-log excerpt.
4. The severe-safety flight trace (move proposal with ancestry) live and under replay.
5. Embodied/debug comparison for stale workplace belief vs truth; embodied render
   without numeric need values.
6. Source-guard inventory including the workspace census and fixture-directory census.
7. The `cargo mutants` baseline over guarded layers with caught/missed counts and
   dispositions for misses.
8. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`; not
   full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment; INV-001…INV-110 are applied, not changed.
- Crate direction preserved: core depends on nothing; content on core; tui on
  core + content.
- Recommended ticket ordering: `ORD-HARD-014`/`015` first (they change golden
  checksums and the duration model that `016`'s gate must replay), then `016`/`018`/
  `020` (provenance and evidence), then `017`/`019`/`023` (behavioral), then
  `021`/`022`/`024`, with `025` last so the census/mutants baseline locks the finished
  surface.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **ORD-HARD-014 reprices the whole day.** Every golden no-human checksum changes; the
  acceptance ledger is the only honest way to review the diff. Budget for fixture churn.
- **ORD-HARD-016's gate needs the builder at the replay frontier.** Replay already
  rebuilds `AgentState`; re-running `from_event_log` per decision is feasible but may be
  slow on large logs — acceptable for golden runs; do not sample.
- **ORD-HARD-021 remains the expensive one.** The minimum cut is defined and its
  recording is an acceptance criterion; full unification may become spec 0017's core if
  deferred.
- **Mutation-testing cost:** the first `cargo mutants` baseline run is slow; scope it to
  guarded layers and run it outside the PR-blocking path.
- **Severity drift risk:** this is the fourth Phase 3A pass, and each pass has found
  blocker/high issues in surfaces previously marked verified. A fifth, verification-only
  audit after 0016 lands is warranted; the mutants baseline and census locks introduced
  here should make it materially cheaper and are partly justified by that goal.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` and names responsible layers from
  the execution diagnostic vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted).
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression.
- [x] Verified-holding items from 0014/0015 are recorded to prevent re-litigation;
  overturned 0015 claims are explicitly recorded as overturned.
- [x] No doctrine amendment; no compatibility shims; crate direction preserved.
- [x] Scope stays within the Phase 3A ordinary-life surface.

## Outcome

Completed: 2026-06-10.

Tickets `0016PHA3ANEEACC-001` through `0016PHA3ANEEACC-015` were implemented
and archived. The remediation landed the duration terminal-set and reservation
closure, single-regime need accounting, decision context-hash re-derivation,
source-event witnesses, fail-closed audit coverage, severe-safety flight,
embodied projection-backed affordances, content validation, stuck-diagnostic
discipline, replay robustness, lock-layer census and mutation baseline, the
scoped acceptance artifact, conformance documentation, and the final no-human
actor-known unification onto `EpistemicProjection`.

Deviation from the original plan: the recorded ORD-HARD-021 deferral was closed
inside this spec family as ticket `0016PHA3ANEEACC-015`, instead of being left
for a later spec.

Verification results included the ticket-local targeted checks and the final
gates:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
