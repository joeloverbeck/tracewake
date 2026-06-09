# 0015 Phase 3A Evented Cognition Channels, Audit Enforcement, and Anti-Contamination Hardening Spec

**Status**: COMPLETED
**Staging path:** `archive/specs/0015_PHASE_3A_EVENTED_COGNITION_CHANNELS_AUDIT_ENFORCEMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `1e3a948` (post-0014 closeout; all `0014PHA3AORDLIF` tickets landed).
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the third Phase 3A alignment pass. Spec 0014 closed `ORD-HARD-001`
through `ORD-HARD-007`. This audit re-derived the normative contract from
`docs/0-foundation/*`, `docs/1-architecture/*`, and `docs/2-execution/*` and
re-examined the code at the local baseline. Findings continue the `ORD-HARD`
series at `008`.

All evidence below was verified against local sources at the target baseline.
Citations use named symbols, which are grep-stable; line numbers are omitted
deliberately.

## 1. Scope

### In scope

- The no-human actor-known surface builder and its provenance model
  (`crates/tracewake-core/src/agent/no_human_surface.rs`, the `no_human`
  module of `crates/tracewake-core/src/scheduler.rs`).
- Enforcement of the hidden-truth audit inside the actor decision transaction
  (`crates/tracewake-core/src/agent/decision.rs`,
  `crates/tracewake-core/src/agent/transaction.rs`).
- Embodied/actor-known projection surfaces for food, sleep, and routes
  (`crates/tracewake-core/src/projections.rs`, `view_models.rs`, TUI render
  insofar as it consumes them).
- Scheduled duration completions and interruption machinery
  (`crates/tracewake-core/src/scheduler.rs`,
  `crates/tracewake-core/src/actions/defs/sleep.rs`, `work.rs`).
- Kernel-hardcoded ordinary-life tuning constants vs content-authored
  equivalents (`crates/tracewake-core/src/time.rs`, `actions/defs/sleep.rs`,
  `agent/need.rs`; `crates/tracewake-content/src/schema.rs`).
- Durability of the Phase 3A anti-contamination lock layer
  (`crates/tracewake-core/tests/anti_regression_guards.rs`, `clippy.toml`).

### Out of scope

- Re-auditing Phase 1 / Phase 1A spine internals (covered by 0010–0012).
- Re-auditing Phase 2A epistemic internals (covered by 0013), except where
  Phase 3A must start *consuming* the Phase 2A epistemic substrate instead of
  bypassing it.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014 fixed that this audit verified as holding
  (see §3 "Verified holding").

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-009, INV-015,
  INV-018, INV-035, INV-043/044/045, INV-048, INV-061, INV-067, INV-069,
  INV-080, INV-093, and the 2026 hardening set INV-099…INV-110, especially:
  - INV-099 truth may validate, not plan;
  - INV-101 actor-known context is sealed;
  - INV-102 cognition inputs require provenance — "Missing provenance is a
    design smell and, for action-relevant cognition, a rejection condition";
  - INV-103 the scheduler is not a cognition authority.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` —
  forbidden context sources; the decision-transaction contract.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` —
  "raw route/workplace/food/sleep tables used directly for cognition" is a
  forbidden source; embodied affordance menus must be generated from the bound
  actor's visible/perceived/known context.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` —
  scheduler powers and limits.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` —
  TFW provenance table (workplace/sleep/food/route allowed vs forbidden
  sources).
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` —
  ORD-LIFE-CERT clauses; Phase 3A deferral boundaries.

## 3. Determination

**Positive verdict.** The in-scope Phase 3A ordinary-life substrate is not
fully aligned with the foundation pack and not maximally locked. Six findings
follow, `ORD-HARD-008` … `ORD-HARD-013`.

The root pattern behind the two largest findings: **0014 fixed the call sites,
not the channel.** The actor-known surface no longer copies raw truth at the
scheduler seam, but the replacement builder mints provenance-labeled facts
from the same raw tables, and the audit that is supposed to catch forbidden
inputs is computed but never enforced. The Phase 2A epistemic substrate
(events → beliefs → holder-known projection) exists and is hardened; the
Phase 3A cognition path bypasses it.

### Verified holding (no action; recorded so they are not re-litigated)

- `ORD-HARD-003` fix holds: the method-fallback loop re-runs
  `select_goal_and_trace`, the failed candidate is marked `Inapplicable` with
  a typed `method_selection_rejected:` reason, and `SelectedGoalBundle` keeps
  trace/lifecycle/method/plan/proposal coherent. Proven by
  `method_fallback_reruns_selection_with_coherent_trace_and_candidate`.
- `ORD-HARD-002` fix holds for direct mutation: `SealedProposal` prevents the
  scheduler from rewriting transaction proposals (but see `ORD-HARD-008` for
  the `extend_actor_known_facts` pre-transaction seam).
- INV-034/035 intention durability: `ContinueCurrentIntention` outranks mild
  pressures, severe pressures interrupt with typed `IntentionLifecycleEffect`s.
- INV-106 rejection feedback: `append_rejection_event` splits actor-visible
  from debug-only checked facts; typed `ReasonCode`s do not name hidden
  targets.
- INV-108 possession parity: human and agent proposals share validation; no
  player-privilege branch found.
- INV-017/018 determinism layer: `clippy.toml` `disallowed-types` +
  recursive banned-token scans are alias-proof for the covered classes.
- INV-046: `service_completed_placeholder` work output is a documented
  Phase 4 deferral (`docs/2-execution/03`, `docs/2-execution/11`), not a
  violation.
- Passive need decay: lumped per-window `NeedDeltaApplied` deltas are
  event-sourced, deterministic, and do not double-count sleep ticks.

## 4. Findings and remediation requirements

### ORD-HARD-008 — No-human actor-known surface mints cognition facts from raw truth without event ancestry

**Severity:** blocker for scoped `ORD-LIFE-CERT` evidence.

**Responsible layers:** `holder_known_context`, `scheduler`, `content_seeding`.

**Doctrine breached:** INV-099, INV-100, INV-101, INV-102, INV-103; INV-009 and
INV-018 (knowledge with no event ancestry cannot be replay-derived); TFW
provenance table (workplace/sleep knowledge must come from observation,
memory, assignment notice, or record — not raw tables).

**Evidence (all in `crates/tracewake-core/src/agent/no_human_surface.rs`
unless noted):**

- `observe_workplace_notices_from_active_routines` iterates
  `state.workplaces()` and tests `workplace.assigned_actor_ids.contains(&self.actor_id)`
  — raw assignment truth, over **all** workplaces regardless of location —
  gated only by the actor having a live `RoutineFamily::GoToWork`/`WorkBlock`
  routine. It then mints a `routine_assignment_workplace_notice` fact. No
  assignment-notice event exists anywhere in the log.
- `observe_sleep_notice_from_active_routine` reads raw
  `state.sleep_affordances()` and the `access_open` flag, then mints facts
  labeled `modeled_observation:open_sleep_affordance_at_current_place` and
  `routine_assignment_notice:sleep_window_current_rest_surface`. No
  observation event is appended; the label asserts a channel that never ran.
- `observe_visible_food_sources_from_current_place` scans raw
  `state.food_supplies()` filtered by current place. Plausible as perception,
  but no `ObservationRecorded` event is appended, so the "observation" leaves
  no trace and cannot be replayed from the log.
- In `scheduler.rs::no_human::build_agent_proposal`, after the surface is
  built and converted into a context, the scheduler calls
  `extend_actor_known_facts(transaction_facts)` — re-opening the sealed
  context — and mints its own `observed_now` fact
  (`food_source_believed_accessible`) by introspecting the context's
  `known_food_sources()`. The scheduler is manufacturing cognition input
  (INV-103) and the seal is decorative at this seam (INV-101).

**Why 0014's locks did not catch this:** the 0014 guard bans raw reads "inside
the no-human context builder except through an allowlisted
perception/observation function" — and these methods *are* the allowlisted
functions. The fixture `no_human_known_workplace_requires_provenance_001`
passes because routine-template presence is treated as the modeled channel.
A routine label is not an information channel; INV-104 separates routine
structure from knowledge, and the TFW table requires an in-world source.

**Required correction (the channel fix, not another call-site fix):**

1. **Seed knowledge as events.** At content load/seed time, materialize the
   knowledge an authored actor legitimately starts with — workplace
   assignment, home/sleep place, household food knowledge — as typed
   prehistory knowledge events (e.g. `RoleAssignmentNoticeRecorded`,
   `StartingBeliefRecorded`) marked as authored prehistory per INV-063.
   Fixtures already author routines and assignments; the load step must emit
   the corresponding knowledge events instead of letting the builder
   reverse-engineer them from raw tables at decision time.
2. **Perceive as events.** Current-place perception (visible food, visible
   sleep affordance, visible exits) must run as a modeled perception pass that
   appends observation events (the Phase 2A observation machinery already
   exists) before or within the decision window, deterministically keyed to
   the window ordering key.
3. **Build the surface from the epistemic substrate.** The actor-known surface
   builder consumes beliefs/observations/notices derived from the event log
   (the Phase 2A holder-known projection), never `PhysicalState` tables. Every
   `ActorKnownFact` carries `source_event_ids` that exist in the log.
4. **Close the seal.** Remove `extend_actor_known_facts` from the scheduler
   path. Framing facts the transaction needs (wait reason, reevaluation
   window, active-intention summary) are built inside the sealed builder with
   typed provenance classes, or passed as explicitly non-cognitive transaction
   parameters — not as minted `observed_now` facts.

**Structural lock:**

- Source guard: in `no_human_surface.rs`, ban `state.workplaces()`,
  `state.sleep_affordances()`, `state.food_supplies()`, and any
  `PhysicalState` table accessor; the builder's only inputs are the epistemic
  projection, the event log, and `AgentState`.
- Source guard: ban `extend_actor_known_facts` (and any post-`into_context`
  fact insertion) in `scheduler.rs`.
- Replay gate: the actor-known context hash used by each decision must be
  reconstructable from the event log alone; add a rebuild test that recomputes
  every decision's context from replayed events and byte-matches the recorded
  hash.
- Negative fixtures:
  - `no_human_workplace_knowledge_requires_notice_event_001`: actor has a live
    work routine and a raw assignment, but the seed emits no assignment-notice
    event → no workplace fact, no work proposal; stuck diagnostic names
    `holder_known_context`.
  - `no_human_sleep_knowledge_requires_observation_or_record_001`: open sleep
    affordance at current place, but perception pass disabled for the fixture
    → no sleep fact without an observation event.
  - `no_human_observation_facts_cite_log_events_001`: every `ActorKnownFact`
    consumed by a committed decision cites `source_event_ids` present in the
    log.

### ORD-HARD-009 — Hidden-truth audit is computed but never enforced

**Severity:** high.

**Responsible layers:** `proposal_construction`, `action_validation`,
`test_oracle`.

**Doctrine breached:** INV-102 ("for action-relevant cognition, a rejection
condition"); INV-099; INV-105 (a proof artifact that gates nothing is a
decorative proof surface).

**Evidence:** `hidden_truth_audit_from_actor_known_inputs`
(`crates/tracewake-core/src/agent/decision.rs`) computes `actor_known_only`
and a forbidden-source count. Consumption sites at the baseline: stored in
the trace, serialized into event payloads
(`hidden_truth_audit_actor_known_only` in `scheduler.rs`), rendered in debug
reports, asserted in unit tests. **No site rejects, stuck-fails, or blocks a
proposal.** A transaction whose inputs include `DebugOmniscience` or
`UnprovenPhysicalTruth` provenance proceeds through the pipeline and commits.

**Required correction:** Fail closed inside the transaction: when the audit
finds any forbidden source class among action-relevant inputs,
`ActorDecisionTransaction::run` returns a typed `Stuck` outcome with
`blocker_code = hidden_truth_input` and `responsible_layer` naming the layer
that introduced the input. No proposal is constructed. As defense in depth,
the pipeline's source-context validation stage asserts the attached decision
trace's audit is clean for agent-origin proposals and rejects with a typed
reason otherwise.

**Structural lock:**

- Source guard: `transaction.rs` must contain the audit gate (assert the
  symbol/pattern exists, mirroring the existing positive-presence guards such
  as the `SelectedGoalBundle` check).
- Negative fixture `forbidden_provenance_input_fails_closed_001`: inject a
  typed forbidden-class input (no banned words in display text); the
  transaction must produce a stuck diagnostic, the log must contain no
  proposal/commit for that window, and replay must rebuild the same stuck
  diagnostic.
- Event/replay gate: the stuck diagnostic carries the existing typed fields
  (`responsible_layer`, `blocker_code`, `hidden_truth_referenced = true`).

### ORD-HARD-010 — Embodied food/sleep/route surfaces still derive from raw state (ORD-HARD-006 fixed only workplaces)

**Severity:** high.

**Responsible layers:** `view_model`, `holder_known_context`, `projection`.

**Doctrine breached:** INV-067, INV-069, INV-008, INV-099–102, INV-048
(routes), INV-093; architecture doc 03's embodied-affordance formula.

**Evidence (`crates/tracewake-core/src/projections.rs`):**

- `actor_known_food_sources_for_context` takes the sealed `KnowledgeContext`
  but uses it only for a mode check; the food list comes from iterating raw
  `state.food_supplies` filtered by physical visibility.
- `visible_open_sleep_affordance` takes no context at all; it scans raw
  `state.sleep_affordances()` and gates the embodied "sleep here" semantic
  action on `access_open` truth.
- Visible exits are built from raw `place.adjacent_place_ids` with no
  knowledge filter — every adjacency is offered regardless of what the actor
  has perceived or learned (INV-048 route knowledge).
- `KnowledgeContext` carries `actor_known_workplaces` (the 0014 fix) but has
  no food/sleep/route equivalents to consume.

Physical co-location makes the *behavior* approximate perception, but the
structural pattern is exactly what 0014 ruled a violation for workplaces: the
projection computes "what the actor knows" from authoritative truth inline,
with no provenance and no seal, and the TUI affordance menu is generated from
it.

**Required correction:** Extend the sealed context packet with context-backed
food, sleep-affordance, and route/exit facts, populated through the same
evented channels as `ORD-HARD-008` (perception events for current-place
visibility; memory/records for non-local knowledge). Embodied projection
renders only context-backed facts plus actor-visible action definitions.
Debug projection may compare context vs truth non-diegetically.

**Structural lock:**

- Source guard: ban `state.food_supplies`, `state.sleep_affordances`, and
  `adjacent_place_ids` in embodied actor-known projection helpers (extend the
  existing `guard_014_embodied_projection_workplaces_are_context_backed`
  family to all four surfaces).
- Adversarial fixtures:
  - `embodied_view_omits_unobserved_food_at_open_place_001`
  - `embodied_view_omits_unknown_sleep_affordance_001`
  - `embodied_exits_require_perceived_or_known_route_001`
  Each: raw state contains the entity, context contains no fact, embodied
  output omits it, debug output may show the discrepancy.

### ORD-HARD-011 — Scheduled completions skip revalidation; sleep interruption is unreachable

**Severity:** high.

**Responsible layers:** `scheduler`, `action_validation`,
`intention_lifecycle`.

**Doctrine breached:** INV-001, INV-035 (interruption points are part of the
routine contract), INV-043/044/045 (a refill disconnected from current world
state), INV-015.

**Evidence:** `scheduler.rs::no_human::append_scheduled_completion` builds
completion events from the started-event payload
(`build_sleep_completion_events`, `build_work_completion_events` in
`actions/defs/sleep.rs` / `work.rs`) and appends them directly. Neither
builder receives current state: no check that the actor is still at the
place, still in the sleeping/working condition, or that no interruption
occurred. `build_sleep_interruption_events` exists in `sleep.rs` but is never
invoked from production code — INV-035's interruption points cannot fire, so
every accepted sleep yields full scheduled recovery unconditionally.

The guard allowlist rationale ("scheduler may complete previously accepted
duration actions") covers *who* appends the completion, which is a permitted
scheduler power; it does not license completing a duration whose continuity
no longer holds.

**Required correction:**

1. Completion builders take current `PhysicalState`/`AgentState` and validate
   continuity (actor present, condition uninterrupted, affordance still
   usable). A broken continuity produces a typed
   `SleepInterrupted`/`WorkBlockFailed`-class event with prorated need deltas
   for the elapsed ticks, not the full scheduled recovery.
2. Wire a modeled interruption path for Phase 3A's interruption sources:
   severe need/safety pressure crossing during a sleep/work duration, and any
   event that displaces the actor. Interruptions are events (INV-015) and
   feed the next decision window.

**Structural lock:**

- Source guard: completion builders must reference current-state continuity
  checks (positive-presence assertion), and the scheduler completion path
  must not append a completion whose continuity check did not run.
- Fixtures: `sleep_interrupted_by_severe_need_prorates_recovery_001`,
  `work_completion_fails_when_actor_displaced_001`.
- Capstone extension: the no-human capstone asserts at least one interrupted
  duration in an adversarial fixture run, with replay byte-match.

### ORD-HARD-012 — Kernel hardcodes ordinary-life tuning facts the content schema already authors elsewhere

**Severity:** medium.

**Responsible layers:** `content_seeding`, `action_validation`, kernel/content
authority boundary.

**Doctrine breached:** INV-061 (designers author needs/routines/affordances),
INV-080 (domain packs own flavor); architecture doc 01 (content packs define
possible objects, places, routines, fixtures, initial conditions).

**Evidence:** `AWAKE_HUNGER_DELTA_PER_TICK = 5` and
`AWAKE_FATIGUE_DELTA_PER_TICK = 3` in `crates/tracewake-core/src/time.rs`;
`DEFAULT_SLEEP_DURATION_TICKS = 4`, `FATIGUE_RECOVERY_PER_SLEEP_TICK = 20`,
`HUNGER_RISE_PER_SLEEP_TICK = 2` in `actions/defs/sleep.rs`. Meanwhile
`WorkplaceSchema` authors `fatigue_delta_per_tick`, `hunger_delta_per_tick`,
`work_duration_ticks`, and `FoodSupplySchema` authors
`hunger_reduction_per_serving` — the same fact class, authored for work/food
but hardcoded for sleep/idle. The asymmetry proves the authoring path exists
and the kernel constants are a shortcut.

**Required correction:** Author passive need rates and sleep
duration/recovery in content: a need-model config block in the fixture/domain
schema, and duration/recovery fields on `SleepAffordance` (mirroring
`WorkplaceSchema`). Core consumes authored values carried in seeded state; no
silent defaults (per the no-shims convention, the schema fields are
required and validation rejects their absence). Need band thresholds in
`agent/need.rs` may remain kernel semantics (they define the pressure
vocabulary, not domain flavor) — record this boundary decision in the
conformance index.

**Structural lock:** Source guard banning integer need-delta/duration
constants in `time.rs` and `actions/defs/` (allowlist: identity/zero values);
content validation requiring the new fields; golden fixtures updated with
explicit values matching current behavior so replay checksums stay
explainable.

### ORD-HARD-013 — Targeted source guards scan hardcoded file constants; new modules escape targeted rules

**Severity:** medium-low.

**Responsible layers:** `test_oracle`.

**Doctrine breached:** none directly; this is lock-layer durability (the
anti-contamination measure itself must resist drift).

**Evidence:** `anti_regression_guards.rs` defines per-file constants
(`SCHEDULER_RS`, `TRANSACTION_RS`, …) for the targeted guards; only the
banned-nondeterminism scan walks `production_sources()` recursively. A new
`scheduler/` or `agent/` submodule (e.g. a future
`scheduler/completions.rs`) would be covered by the determinism scan but by
none of the targeted truth-firewall rules. (`clippy.toml` and CI were
verified strong: `disallowed-types`/`disallowed-methods` are alias-proof and
CI denies warnings — no action there.)

**Required correction:** Targeted guards take module-tree globs
(`src/agent/**`, `src/scheduler*`, `src/projections*`) instead of single-file
constants wherever the rule is about a layer rather than one file. Add a
census guard: the guard file-list must equal the actual module tree for the
guarded layers, so adding a file without classifying it fails the suite.

## 5. Anti-contamination lock layer (consolidated)

1. **Source guards** (extend `anti_regression_guards.rs`, `guard_015_*`):
   raw-table bans in `no_human_surface.rs`; `extend_actor_known_facts` ban in
   `scheduler.rs`; audit-gate positive presence in `transaction.rs`; raw
   food/sleep/route bans in embodied projection helpers; completion
   continuity-check positive presence; tuning-constant bans in `time.rs` /
   `actions/defs/`; glob-based file discovery + census guard.
2. **Sealed types:** the actor-known surface builder's constructor accepts
   the epistemic projection + event log, not `PhysicalState` (make the wrong
   input uncompilable, mirroring `ActorKnownPlanningContext`); context
   becomes immutable at `into_context` (no post-seal extension API at all).
3. **Negative/adversarial fixtures:** the nine named in §4, run through
   normal `cargo test` via the existing fixture runner.
4. **Replay/schema gates:** context-hash rebuild-from-log byte-match; stuck
   diagnostics for `hidden_truth_input` rebuild under replay; interrupted
   durations replay byte-identical; new/changed event kinds versioned per
   INV-020.
5. **Conformance index update:** extend the 0014 conformance index rows —
   workplace/sleep/food/route knowledge allowed-source column changes from
   "builder reads with label" to "event-sourced notice/observation/record
   with `source_event_ids`"; add rows for completion continuity and audit
   enforcement.

## 6. Documentation corrections (housekeeping, same package)

- `CLAUDE.md` (project root): the invariants range was already advanced from
  "INV-001…INV-098" to "INV-001…INV-110" in the working tree after the
  `1e3a948` baseline (uncommitted at audit time). Confirm this change lands
  with this package's commit rather than re-applying it.
- If implementation reveals the TFW provenance table needs an explicit
  sentence that routine-template presence is not an information channel, add
  it to `docs/2-execution/04` as a clarification (execution tier; no doctrine
  amendment required — INV-104 already implies it).

## 7. Acceptance artifact

A new report under `reports/` (e.g.
`reports/0015_ord_life_cert_scoped_acceptance.md`) recording, for the
implementation commits:

1. Source-guard inventory and passing output, including the census guard.
2. For each adversarial fixture: event-log excerpt with proposal ancestry or
   stuck diagnostic, and replayed event ids.
3. Per-decision actor-known context hashes with their rebuilt-from-log
   counterparts (byte-match evidence).
4. Seed-time knowledge events for one canonical fixture actor, demonstrating
   the INV-063 prehistory marks.
5. An interrupted-sleep trace showing prorated deltas live and under replay.
6. TUI embodied/debug artifact: raw food/sleep/route present in state, absent
   from context, omitted from embodied output, visible in debug comparison.
7. Explicit non-certification statement: scoped evidence toward
   `ORD-LIFE-CERT`; not full-project certification, not Phase 4 entry, not
   `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment; INV-099…INV-110 are applied, not changed.
- Crate direction preserved: core depends on nothing; content on core; tui on
  core + content. Seed-time knowledge-event emission lives in content's load
  path calling core event constructors, not in core reading content.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **ORD-HARD-008 is the expensive one.** Routing no-human cognition through
  the Phase 2A epistemic substrate touches the seed path, the perception
  pass, the builder, and every no-human fixture. If implementation proves the
  full unification too large for one pass, the minimum acceptable cut is:
  evented seed knowledge + evented perception + builder consumes only those
  events — deferring deeper unification with Phase 2A belief structures to a
  recorded follow-up, not silently.
- **Fixture churn:** golden checksums will change (new seed/observation
  events). Each fixture diff must be explainable in the acceptance artifact.
- **ORD-HARD-012 ordering:** do it after 008/011 so the authored values land
  in the schema the continuity/perception work already touches.

## 10. Self-check

- [x] Determination is positive; spec authored under the
  produce-only-if-positive rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` and names responsible
  layers from the execution diagnostic vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line
  numbers deliberately omitted).
- [x] Every finding includes a structural lock that fails the build or test
  suite on regression.
- [x] Verified-holding items from 0014 are recorded to prevent re-litigation.
- [x] No doctrine amendment; no compatibility shims; crate direction
  preserved.
- [x] Scope stays within the Phase 3A ordinary-life surface.

## Outcome (2026-06-09)

Completed. The `0015PHA3AEVECOG` ticket family implemented the scoped Phase 3A
evented cognition, audit enforcement, and anti-contamination hardening package
across tickets 001-011, archived each ticket, and added the required
`reports/0015_ord_life_cert_scoped_acceptance.md` acceptance artifact.

What changed:

- Authored starting knowledge is materialized as seed-time knowledge events.
- Modeled current-place perception emits observation events.
- The no-human actor-known surface consumes event-backed epistemic inputs and
  remains sealed through the transaction boundary.
- Source guards, context-hash rebuild checks, and adversarial fixtures lock the
  event-backed cognition channel against raw-truth regression.
- Hidden-truth audit failures fail closed inside the decision transaction.
- Embodied food, sleep, route, and workplace surfaces are context-backed.
- Scheduled sleep/work completions revalidate continuity and emit typed
  interruption/effect events.
- Ordinary-life tuning moved to authored content with the kernel/content
  boundary documented.
- Guard discovery now uses module-tree coverage with a census guard.
- Conformance docs and the scoped ORD-LIFE-CERT evidence report were updated.

Deviations:

- The capstone did not add a new verification-only test. Existing capstone,
  replay, anti-regression, hidden-truth, and fixture tests already exercise the
  required no-human, replay byte-match, source-guard, interruption, and
  embodied/debug evidence surfaces.

Verification:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
