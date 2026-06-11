# 0020 Phase 3A Cognition-Surface Freshness Parity, Derived-Census Closure, Mutation-Perimeter Completion, and Generative Fidelity Hardening Spec

**Status**: COMPLETED

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `96bc387` (post-0019 closeout; all `0019PHA3AGENREA` tickets landed, merge PR #27).
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the eighth Phase 3A alignment pass — the verification-leaning pass that
0019 §9 itself warranted. Spec 0014 closed `ORD-HARD-001`–`007`; 0015 closed `008`–`013`;
0016 closed `014`–`025`; 0017 closed `026`–`034`; 0018 closed `035`–`043`; 0019 closed
`044`–`052`. This audit re-derived the normative contract from `docs/0-foundation/*`,
`docs/1-architecture/*`, `docs/2-execution/*`, and `docs/3-reference/*`, re-examined the
full surface introduced by archived spec 0005 at the post-0019 baseline, and — per the
audit directive — treated every prior correction (including 0019's own deliverables) as
unverified until proven in code. Findings continue the `ORD-HARD` series at `053`.

All evidence below was verified against local sources at the target baseline. Citations
use named symbols, which are grep-stable; line numbers are omitted deliberately. The
audit was conducted as an eight-slice delegated review (generative lock tier, mutation
perimeter/CI, epistemics/truth firewall, replay/checksum/versioning, needs/scheduler/
actions kernel, content seeding, TUI/possession/debug, evidence-document honesty). Every
blocker/high/medium finding below was independently operator-verified at its load-bearing
cited symbols; three low findings initially rested partly on delegated evidence
(`ORD-HARD-060`, `ORD-HARD-061`'s per-family seed mapping, `ORD-HARD-064`). A
reassessment pass at the same baseline operator-verified all three mechanisms at
source, so every finding below is operator-verified; `ORD-HARD-064`'s remaining open
question is its consequence (whether the spurious diagnostic fires), which its own
boundary fixture decides.

The root pattern this pass: **0019's corrections were real but three of them landed
incompletely — fixed on one surface or one member of a family while a sibling surface
kept the defect.** The workplace freshness/supersession correction (`ORD-HARD-046`)
landed on the embodied/validation path but never reached the no-human planner surface;
the derived-census correction (`ORD-HARD-047`) extended a hand-typed list instead of
deriving it, leaving four materializing apply arms ungated and unexempted; the mutation
perimeter (`ORD-HARD-045`) admitted `sleep.rs` and `work.rs` but silently dropped the
spec's own "`eat.rs` if nontrivial" clause. None of this is new kernel rot, and — for
the first time in the lineage — **the evidence documents verified fully honest**: every
checkable claim in the 0019 acceptance report, the 0018 correction section, the
conformance-index rows, and the spec ledger matched the code.

Two notes that distinguish this pass from its predecessors:

- The kernel product-behavior surface verified clean for the **third consecutive pass**:
  no direct dispatch, no hidden-truth cognition reads, single-charge need accounting,
  duration/reservation discipline, possession parity, and debug quarantine all
  re-verified at their symbols.
- The highest-severity finding is, for the first time, an **incomplete landing of a
  prior named correction** rather than a fresh defect or a fresh overstatement. The
  lesson is about correction-closure: a fix to a classified defect must enumerate every
  surface the defect class touches, and the lock must cover the family, not the
  instance.

## 1. Scope

### In scope

- The no-human cognition surface and its freshness/supersession parity with the
  embodied path (`crates/tracewake-core/src/agent/no_human_surface.rs`,
  `src/agent/perception.rs`, `src/epistemics/projection.rs`).
- Materialized-payload version closure and census derivation
  (`src/events/apply.rs`, `crates/tracewake-core/tests/anti_regression_guards.rs`).
- Mutation-testing perimeter completeness, guard scope, and baseline governance
  (`.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`,
  `.github/workflows/ci.yml`, the perimeter consistency guard).
- Generative lock tier fidelity: tamper-relation specificity, completion-path
  fidelity, corpus robustness (`crates/tracewake-core/tests/generative_lock.rs`,
  `tests/support/generative.rs`, `src/scheduler.rs` no-human entry points).
- Content seeding allowlist durability (`crates/tracewake-content/src/fixtures/mod.rs`,
  `crates/tracewake-content/tests/fixtures_load.rs`).
- Embodied surface completeness residue (`src/projections.rs`, the TUI render path).
- Scheduler boundary semantics (`src/scheduler.rs` window stuck-sweep and
  routine-window selection).
- Risk-register memory for the lineage's recurring evidence-overstatement relapse
  (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`).
- The first `EMERGE-OBS` emergence-evidence ledger baseline (§7 item 10, per
  `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
  §Emergence-evidence ledger) — an acceptance-artifact deliverable, not an audited
  surface.

### Out of scope

- Re-auditing Phase 1 / 1A spine internals (0010–0012) and Phase 2A epistemic internals
  (0013), except where Phase 3A consumes them.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014–0019 fixed that this audit verified as holding
  (see §3 "Verified holding").

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-020, INV-024/025/026/028,
  INV-038, INV-070/071, INV-091…INV-098, INV-099…INV-106, INV-108.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — one
  freshness rule: staleness reclassifies knowledge, it does not delete it; and the
  supersession contract established by `ORD-HARD-042`/`046`: the newest record per
  workplace wins.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — replay
  must reject unsupported history loudly; censuses that claim closure must derive their
  membership.
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` and
  `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — a
  gate's evidence must be produced by the path under test; a lock that asserts an
  instance while the defect is a family has not locked the family.
- The lineage's enforcement reading: a correction to a classified defect is complete
  only when every surface in the defect's class is corrected or explicitly exempted
  with rationale.

No new external research was consulted this pass; the 0017/0018 lock-design corpus
remains the reference for lock design.

## 3. Determination

**Positive verdict.** The in-scope Phase 3A surface is not maximally locked, and two
findings are letter-level foundational/architecture violations (`ORD-HARD-053`
freshness-parity on the cognition surface, `ORD-HARD-057` remembered-knowledge deletion).
Thirteen findings follow, `ORD-HARD-053` … `ORD-HARD-065`: one high, four medium, eight
low, zero blockers.

### Verified holding (no action; recorded so they are not re-litigated)

- **ORD-HARD-044/048 (generative reachability) are substantive.** `run_case` drives
  every generated case through `advance_no_human` only; duration terminals are emitted
  by `append_due_completions` → `build_sleep_completion_events` /
  `build_work_completion_events` on the scheduler path; `TerminalCounts::record` derives
  per-family floors (`sleep_completed`, `sleep_interrupted`, `work_completed`,
  `work_failed` each `> 0`) from `run.log` only;
  `generative_lock_cannot_fabricate_duration_terminals` bans the four fabricator
  symbols from `generative_lock.rs` via `include_str!`;
  `assert_payload_tamper_poisons_replay` proves per-case divergence. (Residual
  fidelity gaps are `ORD-HARD-059`/`060`/`061`.)
- **ORD-HARD-045 (mutation CI semantics) landed its loud-failure and push-gap halves.**
  The in-diff job captures `mutants_status=$?`, separates exit codes 0/2 from tool
  failure, fails on missing `mutants.out`; the job runs on `pull_request || push` with
  `HEAD^..HEAD` on push; `mutation_perimeter_matches_duration_action_rationale_and_ci_filters`
  parses the real `.cargo/mutants.toml` and `ci.yml`; `exclude_globs` no longer excludes
  `actions/defs/**`. (The perimeter's membership and the guard's scope are
  `ORD-HARD-055`; baseline governance is `ORD-HARD-056`.)
- **ORD-HARD-046 (workplace freshness) is substantive on the embodied/validation path.**
  `perception.rs::current_place_knowledge_context` no longer place-gates workplace
  records; newest-per-workplace supersession via `acquired_tick` comparison;
  `current_place_knowledge_context_keeps_remembered_workplace_notice_after_newer_perception`
  proves the non-coincident-tick case (notice@4, perception@9, fact survives as
  Remembered with `acquired_tick()==4`). (The planner-surface gap is `ORD-HARD-053`;
  the tie-break is `ORD-HARD-058`.)
- **ORD-HARD-047 (payload versioning) landed its named kind and one derived census.**
  The `EventKind::NeedThresholdCrossed` apply arm calls
  `require_payload_version(&payload, "payload_schema_version", "1")`;
  `forged_threshold_payload_schema_version_rejected_for_materialized_agent_replay_001`
  proves loud live and replay rejection;
  `materialized_agent_payload_records_keep_payload_fields` genuinely derives the
  EventId-keyed record-struct census from `state.rs` source. (The apply-arm census's
  hand-enumeration and the ungated sibling arms are `ORD-HARD-054`.)
- **ORD-HARD-049 (bidirectional witness census), ORD-HARD-050 (prose out of checksum),
  ORD-HARD-052 (embodied interruption) are substantive.** The witness guard is
  bidirectional; the three agent-record checksum lines contain no `summary` component
  and `materialized_agent_summary_prose_is_excluded` proves prose-invariance;
  `projections.rs::phase3a_salient_interruption` derives viewer-keyed from materialized
  episode records with `(sim_tick, event_id)` ordering, the render arm is reachable,
  and `view_models_embodied_phase3a_salient_interruption_is_viewer_scoped` proves no
  cross-actor leakage.
- **ORD-HARD-051 (seed-knowledge containment) landed its census and fixture.**
  `known_food_source_blanket_helper_call_sites_are_allowlisted` with synthetic-injection
  and stale-entry directions; `partial_food_source_knowledge_001` genuinely exercises
  partial knowledge (`partial_food_source_knowledge_seeds_only_authored_actor_edge`:
  the edge-bearing actor plans `eat`, the edge-less actor is planner-unreachable with
  `"food source is not actor-known"`). (The indirection bypass is `ORD-HARD-062`.)
- **Cross-cutting kernel checks clean for the third consecutive pass:** every
  production state write in `scheduler.rs` goes through
  `apply_agent_event`/`append_and_apply_agent_event`/`run_pipeline`; every autonomous
  proposal is built through `ActorDecisionTransaction::run` with
  `source_context_check` rejecting agent proposals lacking a clean hidden-truth audit
  (INV-103/104); `planner_goal_for` and `htn::resolve_condition` read only actor-known
  context; single-charge-per-tick holds (`classify_actor_tick_regimes`, `action_effect`
  intervals, `latest_action_tick_delta_tick`); duplicate duration terminals are typed
  errors (`DuplicateDurationTerminal`); body-exclusive reservation conflicts are typed
  and reopened on terminal; work/sleep need-gates read authoritative state, not forged
  proposal params; deterministic ordering keys are total; intention lifecycle never
  silently overwrites; provenance dangling/witness/class-mismatch fail closed
  pre-proposal; debug quarantine (`DebugCapability::mint()` crate-private, compile-fail
  doctests, `debug_truth_never_enters_holder_known_context_hash`) and possession parity
  (`human_after_authorization_matches_scheduler_validation` asserting identical status,
  reason codes, appended events) hold; the embodied why-not path forwards only
  `actor_visible_summary`/`actor_visible_facts`; INV-063 prehistory provenance marking
  holds at `load.rs` seed minting.
- **Evidence documents verified honest — first clean pass for this slice.**
  `reports/0019_ord_life_cert_scoped_acceptance.md` exists and every checkable claim
  matched code; `reports/0018_ord_life_cert_scoped_acceptance.md` carries the accurate
  `## 2026 correction (spec 0019)` section; the three 0019 conformance rows
  (`0019-corrected generative reachability contract`, `0019 mutation-perimeter
  honesty`, `0019 seed-knowledge helper containment`) describe what the code proves;
  the `SPEC_LEDGER` 0019 row is accurate and per convention.

### Validated — no action (checked, found intentional or correctly scoped)

- Commit `b03ac69` ("Fixed CI.") added only a missing perception test
  (`current_place_knowledge_context_uses_latest_projection_window_for_sleep_affordances`,
  +72 lines, no deletions) — no weakening.
- The tamper relation recomputes live checksums from the untampered final world and
  replays the tampered log — the comparison direction is correct.
- Authored affordances may still target items no actor knows (truth-side possibility
  space, INV-044); re-confirmed not a validation gap.
- `docs/2-execution/01_POST_0008_BASELINE_*` remains accurate post-0019; it is scoped
  to the 0005–0008 boundary and carries no stale claim.

## 4. Findings and remediation requirements

### ORD-HARD-053 — Workplace knowledge supersession is absent on the no-human cognition surface: contradictory believed-access facts reach the planner

**Severity:** high.

**Responsible layers:** `holder_known_context`, `agent_cognition` (no-human surface).

**Doctrine breached:** architecture doc 03's one-freshness-rule and the
`ORD-HARD-042`/`046` supersession contract (newest record per workplace wins) — applied
on one of two surfaces only; INV-026/028 (provenance-coherent knowledge; staleness
handled by classification, not by accident); INV-108 in spirit (the possessed/embodied
view supersedes, the autonomous planner does not — divergent epistemic semantics across
control modes).

**Evidence:** the `ORD-HARD-046` correction landed in
`perception.rs::current_place_knowledge_context` (newest-per-workplace collapse via
`acquired_tick` comparison), which is consumed by `actions/pipeline.rs` and
`actions/defs/continue_routine.rs` — the embodied/validation path. The planner surface
that feeds `ActorDecisionTransaction::run` is built by
`no_human_surface.rs::NoHumanActorKnownSurfaceBuilder`, whose
`consume_projection_records` iterates
`projection.classified_actor_known_records_for_context` — which classifies freshness
per record and performs **no supersession** — and whose
`consume_projection_record` `Workplace` arm calls `add_role_assignment_notice` once
**per record**. The projection store is
`BTreeMap<ActorId, BTreeSet<ActorKnownProjectionRecord>>`, so two notices for the same
`workplace_id` differing in tick or `believed_access_open` coexist as distinct set
members; both mint `workplace_believed_accessible` facts (e.g. `id:false` and
`id:true`) into the actor-known surface, both pass the hidden-truth audit (both are
genuinely actor-known), and HTN condition resolution over the fact set is decided by
iteration accident rather than by recency.

**Why existing gates miss it:** the proving test for `ORD-HARD-046` exercises only
`current_place_knowledge_context`; the supersession golden
(`stale_workplace_notice_superseded_by_newer_001`) is asserted through the
embodied/projection path; no test drives stale-plus-fresh contradictory workplace
notices through `NoHumanActorKnownSurfaceBuilder`, and duplicate actor-known facts are
invisible to the hidden-truth audit because neither fact is hidden truth.

**Required correction:** apply the same newest-`source_tick`-per-`workplace_id`
collapse inside `consume_projection_records` (or hoist supersession into
`classified_actor_known_records_for_context` so both surfaces share one classifier —
preferred, since it makes a third surface impossible to add un-superseded). Audit the
correction as a family: every record kind with a supersession semantic must supersede
identically on both surfaces.

**Structural lock:**

- A builder-level test seeding a closed@T notice and an open@T+n notice for the same
  workplace and asserting the surface contains exactly one
  `workplace_believed_accessible` fact carrying the newer value and the newer notice's
  source event id.
- A parity test asserting the no-human surface's workplace fact set equals the
  embodied `current_place_knowledge_context` workplace fact set for the same actor,
  tick, and projection (the two-surfaces-one-classifier contract, INV-108-flavored).

### ORD-HARD-054 — The apply-arm payload-version census is still hand-enumerated, and the need/intention/routine materializing arms are neither gated nor exempted

**Severity:** medium.

**Responsible layers:** `replay`, `events`, `test_oracle`.

**Doctrine breached:** INV-020 (replay rejects unsupported history loudly); lock
durability — 0019's required correction for `ORD-HARD-047` ("derive both censuses
structurally — every apply arm containing an insert into an `AgentState` map must call
the version-requiring helper… or register a typed-column-closure exemption with
rationale") was implemented for one census only; the apply-arm census had the missing
map appended to its literal list instead.

**Evidence:** `materialized_agent_apply_arms_require_payload_schema_version`
(`anti_regression_guards.rs`) iterates the literal array
`["need_threshold_crossings", "ordinary_life_episodes", "candidate_goal_evaluations",
"continue_routine_arbitrations"]`. Meanwhile `events/apply.rs` routes
`NeedDeltaApplied → apply_need_delta` (writes `state.needs_by_actor`,
`state.need_tick_charges`), `IntentionStarted → apply_intention_started`
(`state.intentions`, `state.active_intention_by_actor`), the `Intention*` family →
`apply_intention_transition`, and `RoutineStep* → apply_routine_step_transition`
(`state.routine_executions`) — all writing `AGENT_STATE_CHECKSUM_COVERAGE` maps, none
calling `require_payload_version`, none registered in any exemption registry. The
`DecisionTraceRecorded`/`StuckDiagnosticRecorded` arms gate on different keys
(`trace_schema_version`/`diagnostic_schema_version`), which the census's literal
key-string match would also not recognize.

**Why existing gates miss it:** census membership is enumerated, not derived from the
set of apply arms that write `AgentState` maps — the precise defect shape `ORD-HARD-047`
named, surviving its own correction.

**Required correction:** derive the census structurally: scan `events/apply.rs` for
every insert/`get_mut` into an `AgentState` map named in
`AGENT_STATE_CHECKSUM_COVERAGE`, and require the enclosing arm to call a
version-requiring helper (accepting the `trace_`/`diagnostic_` key variants) **or**
appear in an explicit `TYPED_COLUMN_CLOSURE_EXEMPTIONS` registry with a per-arm
rationale. Then, per arm, either stamp-and-require `payload_schema_version` on the
need/intention/routine payloads (builder + arm; reprices golden event logs once — batch
the churn) or register the typed-column-closure exemption where the record genuinely
persists typed columns only. The choice is the implementer's per arm; the census must
make silent-third-option impossible.

**Structural lock:** the derived census is the lock; a synthetic-regression case (an
arm writing a covered map with neither gate nor exemption must fail the guard).

### ORD-HARD-055 — `eat.rs` is outside the mutation perimeter under a rationale that claims coverage it does not have, and the consistency guard verifies only the sleep/work subset

**Severity:** medium.

**Responsible layers:** `test_oracle`, CI.

**Doctrine breached:** lock durability — residue of `ORD-HARD-045`, whose required
correction said "add `actions/defs/sleep.rs` + `actions/defs/work.rs` (and `eat.rs` if
nontrivial)". `eat.rs` is nontrivial.

**Evidence:** `crates/tracewake-core/src/actions/defs/eat.rs` is ~465 lines with ~15
branch/return sites (`build_eat_events`, `eat_failed_event`, `food_supply_target`) and
reads authoritative `state.food_supplies` under the truth-accessor allowlist. It
appears in neither the scheduled job's `-f` list (only `defs/sleep.rs`, `defs/work.rs`)
nor the in-diff grep alternation (`actions/defs/(sleep|work)\.rs`), and
`MUTATION_PERIMETER_DURATION_DEFS` lists only sleep/work. Its source classification is
`Exempt { rationale: CORE_ACTION_RATIONALE }`, whose text asserts coverage by "targeted
action, pipeline, and duration-definition mutation guards" — which mutation never
reaches for eat. Additionally, `mutation_perimeter_consistency_violations` iterates
only `MUTATION_PERIMETER_DURATION_DEFS`: silently dropping the `agent/`, `scheduler`,
`projections`, or `pipeline.rs` filters from either CI job would pass the guard; and
nothing proves the in-diff grep regex still matches a representative guarded path.

**Why existing gates miss it:** the guard's required-filter loop is hard-scoped to the
two-file duration list, and the blanket rationale string satisfies the rationale check
textually without being true for eat.

**Required correction:** add `actions/defs/eat.rs` to `MUTATION_PERIMETER_DURATION_DEFS`
(rename the constant to reflect the wider set) and to both CI jobs' filters, refreshing
the baseline honestly; extend the guard's required-filter set to the full perimeter
(agent/, scheduler, projections, pipeline.rs, and the defs set) in both jobs; add a
regex canary that applies the in-diff alternation to each perimeter path and asserts a
match. Either way, replace eat's rationale with one that is true.

**Structural lock:** the widened guard plus the regex canary; a synthetic case removing
one perimeter filter must fail.

### ORD-HARD-056 — The mutation miss-baseline is a 148-entry accepted-miss set with no size, content, or rationale governance

**Severity:** medium.

**Responsible layers:** `test_oracle`, CI.

**Doctrine breached:** lock durability (an unbounded, unledgered allowlist is the
recurring `ORD-HARD-034`/`043`/`047` shape applied to mutants); 0019 §9's own warning
that the baseline "must be reviewed, not bulk-accepted".

**Evidence:** `.cargo/mutants-baseline-misses.txt` holds 148 accepted misses spanning
`agent/trace.rs`, `agent/routine.rs`, `scheduler.rs`, `projections.rs`, `pipeline.rs`
(including deleted match arms and `&&`→`||` operator swaps in guarded-layer files).
Both CI jobs `comm -23` new misses against it, so every listed mutant is permanently
green; no test references the file (no guard bounds its size, asserts its entries still
correspond to real symbols, or requires per-entry rationale), so appending lines is a
silent gate-silencing channel.

**Why existing gates miss it:** the consistency guard checks filter strings and shell
semantics, never the baseline's content; the conformance row describes a "pinned
mutation baseline" without a pinned count.

**Required correction:** add a baseline-governance guard: pin the entry count and a
content hash (or per-entry ledger with rationale, the
`CONTENT_NEGATIVE_PROOFS` pattern); a baseline change must update the ledger or fail
CI. Triage the existing 148 entries once — entries in interruption predicates and
decision logic warrant tests rather than baseline residence; record the disposition in
the acceptance artifact.

**Structural lock:** the count/hash pin plus ledger parity census.

### ORD-HARD-057 — The no-human surface deletes remembered food-source knowledge when the actor is elsewhere, while sleep-place knowledge survives: asymmetric staleness deletion on the cognition surface

**Severity:** medium.

**Responsible layers:** `holder_known_context`, `agent_cognition`.

**Doctrine breached:** INV-028 and architecture doc 03 (staleness reclassifies
knowledge — Remembered — it does not delete it); the same class as `ORD-HARD-046`, on
the surface that feeds planning; foundation doc 05/0005-lineage `FindFood` contract
("use actor-known food memories"), which remembered-only food knowledge should serve.

**Evidence:** `no_human_surface.rs::consume_projection_record`'s `FoodSource` arm
early-returns when the record's `place_id`
`is_some_and(|place_id| place_id != &self.current_place_id)` — an observation-derived
food memory at a place the actor has left never reaches `known_food_sources` or the
fact set, so it is planner-unreachable from memory alone. The `SleepPlace` arm in the
same `match` has no such gate and surfaces remembered sleep places from anywhere. The
freshness classifier (`projection.rs::record_freshness`) already produces `Remembered`
for exactly this case; the builder then discards the classified record. Seeded
`household_food_source` beliefs mask the gap for fixture actors whose knowledge is
authored, so canonical days never exercise observation-derived remembered food.

**Why existing gates miss it:** the place-gate reads as visibility filtering rather
than knowledge deletion; `aged_food_record_surfaces_as_remembered_belief_not_observation`
proves the remembered classification only for current-place records; no test asserts a
food source observed at place A survives as remembered knowledge after moving to B.

**Required correction:** replace the hard place-gate with freshness reclassification
(current-place → observed-now, elsewhere → remembered belief), mirroring the workplace
decoupling — unless a phase-ladder deferral cite for remembered-food planning exists in
`docs/2-execution/` (none was found by this audit); if the implementer locates one,
downgrade to a recorded deferral in the conformance index instead. Resolve the
sleep/food asymmetry in either direction explicitly — the two arms must encode the same
doctrine.

**Structural lock:** a builder test asserting a food record observed at place A
surfaces as `remembered_belief` (and is planner-reachable for `FindFood` candidate
generation) after the actor moves to place B; a symmetry assertion covering all
projection record kinds' gating behavior against a declared per-kind policy table.

### ORD-HARD-058 — Embodied workplace supersession tie-break at equal ticks is provenance-arbitrary

**Severity:** low.

**Responsible layers:** `holder_known_context`.

**Doctrine breached:** INV-026 (provenance-grounded knowledge); replay-deterministic
but semantically arbitrary.

**Evidence:** `perception.rs::current_place_knowledge_context` replaces the held
workplace fact when `previous.acquired_tick() <= fact.acquired_tick()` — at equal
ticks, the `BTreeSet` iteration order decides, and `ActorKnownProjectionRecord`'s
derived ordering places `believed_access_open: false` before `true`, so the open
record wins ties for no modeled reason.

**Why existing gates miss it:** the proving test uses distinct ticks (4 vs 7); the tie
path is unexercised.

**Required correction:** break equal-tick ties on `source_event_id` (stable and
replay-deterministic) and document the rule; apply the same rule in the `ORD-HARD-053`
correction so both surfaces tie-break identically.

**Structural lock:** a same-tick contradictory-notice test asserting the
higher-`source_event_id` notice wins on both surfaces.

### ORD-HARD-059 — The generative tamper relation is existential: it returns on the first poisoning field and never specifically perturbs a duration-terminal payload

**Severity:** low.

**Responsible layers:** `test_oracle` (generative tier).

**Doctrine breached:** INV-017/018 spirit; lock durability.

**Evidence:** `generative_lock.rs::assert_payload_tamper_poisons_replay` iterates
events/fields and `return`s on the first perturbation yielding
`!replay.matches_expected || !replay.agent_checksum_matches`. The earliest non-empty
payload in every generated case is the advance marker, so the relation is discharged
without ever touching `SleepCompleted`/`SleepInterrupted`/`WorkBlockCompleted`/
`WorkBlockFailed` payloads — the exact surface `ORD-HARD-044`/`048` were about. If
terminal payload fields became checksum-inert, the relation would still pass.

**Why existing gates miss it:** the metamorphic assertion requires *some* field to
poison replay, not the fields the tier exists to protect.

**Required correction:** add a per-case second pass selecting one duration-terminal
event (failing loudly if the floors claim one exists and none is found) and asserting
each of its payload fields individually poisons replay.

**Structural lock:** the terminal-targeted tamper pass.

### ORD-HARD-060 — The generative advance path evaluates all duration completions against final-tick state, leaving the continuity-failure branches unexercised by the corpus

**Severity:** low. *(Operator-verified at reassessment: `advance_no_human` runs a bare
per-tick loop and calls `append_due_completions` once at the final tick, while
`run_no_human_day` flushes per window plus a final flush.)*

**Responsible layers:** `test_oracle` (generative tier), scheduler test-fidelity.

**Doctrine breached:** lock durability (the reachability contract proves need-driven
terminal branches; the physical-continuity branches of
`work_completion_failure`/`sleep_interruption_reason` are never reached by a generated
case because no case mutates physical truth between a start and its due tick, and the
single `append_due_completions` flush evaluates continuity at the final state).

**Why existing gates miss it:** all four family floors are satisfiable by need-band
branches; corpus sequences contain no displacement/closure events.

**Required correction:** add at least one seed/mask whose sequence changes physical
truth between a duration start and its completion (workplace becomes unusable, actor
displaced) and assert the resulting terminal reason is the continuity reason, counted
from advance-emitted events. If the advance entry's single-flush semantics make
mid-stream continuity unreachable by construction, extend the generative advance to
flush completions per tick (production `run_no_human_day` parity) rather than weakening
the assertion.

**Structural lock:** the continuity-reason floor (nonzero across the corpus).

### ORD-HARD-061 — Per-family generative floors are satisfied with zero margin by hardcoded seed pairs

**Severity:** low. *(Operator-verified at reassessment: `GENERATIVE_SEEDS` ×
`mask_for_seed` yields exactly `{0x…13, 0x…23}` for the interrupt family and exactly
`{0x…11, 0x…21}` for the wait+work family.)*

**Responsible layers:** `test_oracle` (generative tier).

**Doctrine breached:** lock durability.

**Evidence:** `GENERATIVE_SEEDS`/`mask_for_seed` (`tests/support/generative.rs`)
provide exactly two interruption-producing seeds and exactly two work-failure seeds;
`assert_multi_seed_contributors` requires `>= 2`. Removing or editing a single seed
un-locks a family while the suite stays green up to the floor assertion's bare
minimum; the hunger constants (e.g. 820/930) sit in the Severe band by hardcoded
coincidence with `NeedBand::for_value`.

**Why existing gates miss it:** the floors are met exactly; nothing asserts margin or
derives contributing seeds from a predicate.

**Required correction:** either derive the interruption/failure seed sets from a mask
predicate and assert the derived count, or add a third contributing seed per
exactly-met family; assert the hunger constants' band membership explicitly rather
than by coincidence.

**Structural lock:** the predicate-derived contributor count.

### ORD-HARD-062 — The food-omniscience allowlist census is bypassable through the shared adversarial-fixture builder

**Severity:** low.

**Responsible layers:** `content_seeding`, `test_oracle`.

**Doctrine breached:** lock durability of `ORD-HARD-051` (INV-025/062 posture).

**Evidence:** `fixtures/mod.rs::hidden_truth_adversarial_fixture` internally calls
`populate_known_food_sources_for_all_actors`; five fixture files delegate to it. The
census `known_food_source_helper_call_sites_from_source` (`fixtures_load.rs`) scans
each fixture file for the literal `.populate_known_food_sources_for_all_actors(` call,
attributing all indirect users to the single `mod.rs` allowlist entry — a new fixture
delegating to the builder inherits cross-product food omniscience with no census
trip and no rationale obligation.

**Why existing gates miss it:** the census is a per-file textual scan for the direct
call; indirection through a sanctioned wrapper is invisible, and the synthetic-injection
guard proves only the direct-call direction.

**Required correction:** census the callers of `hidden_truth_adversarial_fixture` (and
any future wrapper that transitively reaches the helper) into the same allowlist with
per-entry rationale, or remove the internal blanket call and have each consumer author
its explicit per-actor edges (these are single-actor fixtures; the edge set is tiny).
Add a synthetic indirect-consumer injection case.

**Structural lock:** the transitive-call-site census with the indirect synthetic case.

### ORD-HARD-063 — `VisibleExit.blocker_summary` is a dead embodied surface: hardwired `None` at its only producer

**Severity:** low.

**Responsible layers:** `projection`, TUI.

**Doctrine breached:** INV-070/071 (why-not explanations mandatory; a mechanic hidden
from play is unfinished) — the same defect shape as `ORD-HARD-052`, one field over.

**Evidence:** `projections.rs::build_embodied_view_model` constructs every
`VisibleExit { …, blocker_summary: None }`; the field has no other producer. The
`render.rs::render_embodied_view` arm formatting `" blocked: {summary}"` is therefore
unreachable. Door/access blockers on actor-known routes — a live mechanic — have no
embodied surfacing through this channel.

**Why existing gates miss it:** no test asserts a known route through a
closed/locked door yields `Some(blocker_summary)`; struct shape and renderer presence
satisfy shallow checks (the exact `ORD-HARD-052` miss pattern).

**Required correction:** derive `blocker_summary` from the viewer's actor-known door
state on the route (perceived closed/locked), viewer-keyed exactly as the
`ORD-HARD-052` fix; if exit-blocker surfacing is genuinely deferred by a phase-ladder
cite (none was found by this audit), delete the field and render arm rather than
leaving a dead surface, and record the deferral in the conformance index.

**Structural lock:** a fixture with a known route through a perceived-closed door
asserting `Some(...)` and the render line; a dead-field sweep is `ORD-HARD-065`'s
companion concern — see §5 tier 3.

### ORD-HARD-064 — The window stuck-sweep may emit a spurious `WindowNoProgress` for a window whose only legitimate activity was a duration completion

**Severity:** low. *(Mechanism operator-verified at reassessment:
`progress_by_window_actor` is written only at the proposal-results site and the skip
set only pre-decision, so a boundary completion feeds neither. Whether the spurious
diagnostic actually fires remains test-decided — the first implementation step is the
boundary fixture.)*

**Responsible layers:** scheduler diagnostics.

**Doctrine breached:** typed-diagnostic accuracy (INV-105 spirit) — an over-aggressive
false diagnostic erodes the stuck-detector's evidentiary value.

**Evidence:** the final sweep suppresses `WindowNoProgress` only for actors in
`duration_skip_by_window_actor` (open body-exclusive duration at `window.start_tick`)
or with recorded per-window proposal progress; a duration terminal landing inside a
window via `append_due_completions` is recorded under the completion's process
ordering, not the per-window progress map, so a window advanced solely by a completion
may be flagged.

**Why existing gates miss it:** golden days happen to pair completions with same-window
decisions; no fixture isolates a completion-only window.

**Required correction:** write the boundary fixture first (sleep completes at a window
boundary with no further proposal that window). If the spurious diagnostic fires,
record completion-bearing windows as progress; if it does not, record the verified
behavior in §3-style validated-no-action of the acceptance artifact and close.

**Structural lock:** the boundary fixture either way.

### ORD-HARD-065 — `routine_window_family` admits not-yet-started routine executions as the active window duty

**Severity:** low.

**Responsible layers:** `agent_cognition` (candidate generation inputs).

**Doctrine breached:** INV-035/038 (defeasible routines; deterministic, justified goal
selection — a duty whose window has not opened at the decision tick is not yet a duty).

**Evidence:** `scheduler.rs::routine_window_family` filters executions by
`execution.start_tick <= window.end_tick` (plus a deadline lower bound) and selects
`min_by(start_tick)`; an execution authored to start mid-window is eligible at
`window.start_tick`, before its own start. The sibling selection site in the same file
uses `execution.start_tick <= window.start_tick` — the two bounds encode different
semantics for the same question.

**Why existing gates miss it:** canonical fixtures author executions aligned to window
starts; the mid-window case is unexercised.

**Required correction:** align the bound to the decision tick
(`execution.start_tick <= window.start_tick`, matching the sibling site) or document
why window-scoped eligibility is intended and make both sites consistent.

**Structural lock:** a two-execution test (one starting mid-window) asserting the
already-open execution is selected at `window.start_tick`.

## 5. Anti-contamination lock layer (consolidated)

Tiers extend the 0016–0019 layer.

1. **Compile-time impossibility:** none required this pass — the 0018/0019 type-level
   locks verified substantive.
2. **Runtime gates:** per-arm `payload_schema_version` gates or registered
   typed-column-closure exemptions for the need/intention/routine apply arms
   (`ORD-HARD-054`).
3. **Test-oracle corrections:** the two-surfaces-one-classifier supersession parity
   test (`ORD-HARD-053`); the remembered-food builder test and per-kind gating policy
   table (`ORD-HARD-057`); the same-tick tie-break test (`ORD-HARD-058`); the
   terminal-targeted tamper pass (`ORD-HARD-059`); the continuity-reason floor
   (`ORD-HARD-060`); predicate-derived seed-contributor counts (`ORD-HARD-061`); the
   indirect-consumer census injection case (`ORD-HARD-062`); the exit-blocker fixture
   (`ORD-HARD-063`) **plus a dead-embodied-field sweep**: a guard asserting every
   `Option`/collection field on embodied status/view structs has at least one
   non-default producer or a registered deferral cite — the third dead-surface finding
   in two passes (`salient_interruption`, now `blocker_summary`) warrants the family
   lock, not another instance fix; the completion-only-window boundary fixture
   (`ORD-HARD-064`); the mid-window routine-eligibility test (`ORD-HARD-065`).
4. **Source guards:** the derived apply-arm census (`ORD-HARD-054`); the widened
   mutation-perimeter guard with regex canary (`ORD-HARD-055`); the baseline
   count/hash pin with ledger parity (`ORD-HARD-056`); the transitive helper-call
   census (`ORD-HARD-062`).
5. **Mutation/CI posture:** `eat.rs` inside the perimeter with an honestly refreshed,
   ledgered baseline (`ORD-HARD-055`/`056`).
6. **Evidence honesty:** none owed — the evidence documents verified clean this pass;
   the new risk-register entry (§6) preserves the memory of why.

## 6. Documentation corrections (housekeeping, same package)

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: add a Watch risk
  ("Acceptance-Evidence Reachability Overstatement" or similar) naming the recurring
  relapse the lineage has now seen twice (0016, 0018→0019): an acceptance artifact or
  conformance row presenting harness-fabricated state as path-under-test evidence; name
  the guardrail (advance-emitted-only counting, fabricator source bans) and the
  escalation trigger (any acceptance artifact citing reachability without proving the
  emitting path). A second Watch entry for the new pattern this pass surfaced is
  warranted: **incomplete correction-closure** — a fix landing on one surface of a
  multi-surface defect class (`ORD-HARD-046`→`053`/`057`, `047`→`054`, `045`→`055`).
- Conformance index (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`):
  add/update rows for the supersession-parity contract, the derived apply-arm census,
  the widened mutation perimeter + baseline governance, and the dead-embodied-field
  sweep; correct the "pinned mutation baseline" wording to reflect the count/ledger
  pin once it exists.
- No doctrine amendment; INV-001…INV-110 are applied, not changed.

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0020_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. The supersession-parity proof: the builder-level contradictory-notice test output
   and the two-surfaces-equality assertion (`ORD-HARD-053`/`058`).
2. The remembered-food reclassification proof (or the located deferral cite and its
   conformance row) and the per-kind gating policy table (`ORD-HARD-057`).
3. The derived apply-arm census output: every materializing arm's gate-or-exemption
   disposition, with rationale quoted for each exemption; the golden repricing diff
   per-actor ledger if any payloads were stamped (`ORD-HARD-054`).
4. The expanded mutation run: `eat.rs` mutants caught/missed, the triaged baseline
   ledger with per-entry dispositions, the regex-canary and widened-guard outputs
   (`ORD-HARD-055`/`056`).
5. The generative tier deltas: terminal-targeted tamper output, continuity-reason
   floor counts, predicate-derived seed contributors (`ORD-HARD-059`–`061`).
6. The transitive helper census output (`ORD-HARD-062`).
7. The exit-blocker surfacing proof (or deferral cite) and the dead-embodied-field
   sweep output (`ORD-HARD-063`).
8. The completion-only-window boundary result, whichever way it lands
   (`ORD-HARD-064`), and the routine-eligibility consistency proof (`ORD-HARD-065`).
9. The risk-register and conformance-index diffs, quoted (§6).
10. The first `EMERGE-OBS` emergence-evidence ledger baseline (per
    `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
    §Emergence-evidence ledger): per-seed counters over the canonical no-human
    corpus, recorded as measurement only — no thresholds, no gating.
11. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`; not
    full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment.
- Crate direction preserved: core depends on nothing at runtime; content on core; tui
  on core + content. No new dependencies, dev or production (the tier-6 census makes
  this testable).
- The formerly delegated-evidence details (`ORD-HARD-060` advance-flush semantics,
  `ORD-HARD-061` per-family seed mapping, `ORD-HARD-064` sweep bookkeeping) were
  operator-verified at reassessment; no re-verification step is owed. `ORD-HARD-064`'s
  boundary fixture remains the deciding first step for whether the spurious diagnostic
  fires (fix vs. validated-no-action).
- Recommended ticket ordering: `ORD-HARD-053` + `057` + `058` first and together (one
  builder/classifier surface, one batched embodied-context/golden churn); then `054`
  (derived census + arm dispositions — batch any payload-stamp repricing with the
  first group if both reprice); then `055` + `056` together (one honest baseline
  refresh after the new tests exist); then the generative trio `059`–`061`; then
  `062`; then `063` (with the dead-field sweep); then `064`/`065`; documentation
  corrections (§6) land with the first group. The first `EMERGE-OBS` ledger
  baseline (§7 item 10) is a small read-only reporting deliverable with no
  finding of its own; it lands with the acceptance artifact, after all other
  tickets, so the baseline measures the corrected surface.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **`ORD-HARD-053`'s shared-classifier hoist may change embodied behavior.** Hoisting
  supersession into `classified_actor_known_records_for_context` makes the no-human
  surface correct but also changes what the embodied path iterates; the existing
  embodied tests (notice@4/perception@9, supersession goldens) must stay green, and
  any embodied-context hash churn gets the per-actor ledger treatment.
- **`ORD-HARD-054`'s per-arm disposition is a real design choice.** Stamping versions
  on high-frequency payloads (`NeedDeltaApplied`) reprices every golden; exempting
  them documents a typed-column closure that must actually hold. The census forces the
  choice to be explicit; it does not pre-decide it.
- **`ORD-HARD-055`/`056`'s baseline triage may surface real test debt.** 148 accepted
  misses include guarded-layer decision logic; honest triage may spawn follow-up test
  tickets. Budget for that rather than bulk-re-accepting.
- **`ORD-HARD-057`'s deferral question:** if remembered-food planning is
  phase-deferred by a cite this audit missed, downgrade to a recorded deferral — but
  the sleep/food asymmetry must be resolved explicitly either way.
- **`ORD-HARD-064`'s consequence is test-decided.** Its mechanism is operator-verified,
  but whether the spurious `WindowNoProgress` diagnostic actually fires is decided by
  the boundary fixture; a no-fire outcome downgrades the finding to validated-no-action
  in the acceptance artifact, not silent omission.
- **Next-iteration assessment (the recurring question):** findings were found, so a
  ninth pass is warranted by the lineage's own rule. The honest read of the trend:
  zero blockers for three consecutive passes; the kernel clean for three; the evidence
  documents clean for the first time; and this pass's residue is dominated by
  **incomplete correction-closure of 0019's own fixes**, found one pass later. That
  argues the remaining systematic risk is implementation-completion verification, not
  undiscovered kernel rot. The ninth pass should therefore be a **narrow
  verification-only audit of 0020's deliverables** — checking that each correction
  closed its full defect family (the failure mode this pass caught), materially
  cheaper again. If it returns clean or low-only, drop the per-pass cadence and move
  to per-phase-entry audits (Phase 3B / Phase 4 boundaries), as 0018 and 0019 both
  anticipated.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (or is explicitly
  lock-durability) and names responsible layers from the execution diagnostic
  vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted); every finding was independently operator-verified at its
  load-bearing symbols (the three initially delegated sub-details — `ORD-HARD-060`,
  `ORD-HARD-061` seed mapping, `ORD-HARD-064` mechanism — were operator-verified at
  the reassessment pass).
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression.
- [x] Verified-holding items from 0014–0019 are recorded to prevent re-litigation; the
  evidence documents' first clean pass is recorded; the incomplete-correction-closure
  pattern is named and given a risk-register entry.
- [x] No doctrine amendment; no compatibility shims; no new dependencies; crate
  direction preserved.
- [x] Scope stays within the Phase 3A ordinary-life surface and its lock/evidence
  layer.

## Outcome

Completed on 2026-06-11.

Implementation and evidence landed through ticket commits
`01cfcddbe9eb7f25e854247e54bf2e62dd0d3db0` through
`cd4e7ceb2f78d4de861a75ee875466d74b233306`.

Changed:

- Closed `ORD-HARD-053`/`058` with shared actor-known supersession policy and parity
  tests across no-human and embodied surfaces.
- Closed `ORD-HARD-057` with remembered food-source planning policy and per-kind
  actor-known freshness treatment.
- Closed `ORD-HARD-054` with a derived apply-arm census and typed-column-closure
  exemption ledger.
- Closed `ORD-HARD-055`/`056` with eat/sleep/work mutation-perimeter coverage,
  baseline governance, and `reports/0020_mutants_baseline_disposition.md`.
- Closed `ORD-HARD-059`-`061` with terminal-targeted generative tamper checks,
  continuity-reason floors, and predicate-derived seed-contributor accounting.
- Closed `ORD-HARD-062` with transitive known-food helper census coverage.
- Closed `ORD-HARD-063` with visible-exit blocker surfacing and a dead embodied-field
  sweep.
- Closed `ORD-HARD-064`/`065` with duration-completion window progress accounting and
  routine eligibility by window start.
- Added the scoped acceptance artifact
  `reports/0020_ord_life_cert_scoped_acceptance.md`, including the first read-only
  EMERGE-OBS baseline and the explicit non-certification boundary.

Deviation:

- The EMERGE-OBS ledger implementation is core-only over the `GENERATIVE_SEEDS`
  no-human corpus. Content canonical no-human fixtures remain covered separately by
  content golden fixture tests and the workspace gate because `tracewake-core` cannot
  depend on `tracewake-content`.

Verification:

- `cargo test -p tracewake-core --test emergence_ledger -- --nocapture`
- `rg -n "EmergeObs|emerge_obs|emergence_ledger" crates/tracewake-core/src crates/tracewake-content/src crates/tracewake-tui/src` (no production call sites; exit 1 with no matches)
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
