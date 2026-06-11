# 0019 Phase 3A Generative Reachability Honesty, Mutation Perimeter, Workplace Freshness, and Evidence Closure Hardening Spec

**Status**: COMPLETED

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `5af8660` (post-0018 closeout; all `0018PHA3APROWIT` tickets landed, merge PR #26).
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the seventh Phase 3A alignment pass — the verification-leaning pass that
0018 §9 itself warranted. Spec 0014 closed `ORD-HARD-001`–`007`; 0015 closed `008`–`013`;
0016 closed `014`–`025`; 0017 closed `026`–`034`; 0018 closed `035`–`043`. This audit
re-derived the normative contract from `docs/0-foundation/*`, `docs/1-architecture/*`,
`docs/2-execution/*`, and `docs/3-reference/*`, re-examined the full surface introduced by
archived spec 0005 at the post-0018 baseline, and — per the audit directive — treated
every prior correction (including 0018's own deliverables) as unverified until proven in
code. Findings continue the `ORD-HARD` series at `044`.

All evidence below was verified against local sources at the target baseline. Citations
use named symbols, which are grep-stable; line numbers are omitted deliberately. The audit
was conducted as a six-slice delegated review (provenance/witness, replay/checksum,
needs/scheduler/actions, content seeding, lock layer/CI, TUI/possession); every finding
below was independently operator-verified at its load-bearing cited symbols. One
sub-detail rests on delegated evidence alone and is marked `agent-reported` inline where
it appears (`ORD-HARD-048`'s single-seed interruption count, which is
execution-dependent); the `ORD-HARD-046` test-tick detail was operator-verified at
reassessment.

The root pattern this pass: **the product-behavior surface verified overwhelmingly clean,
but the lock layer asserts more than it proves.** The generative tier's duration-terminal
and interruption "reachability" is fabricated by a test-side helper rather than reached by
the scheduler path under test, and both the conformance index and the 0018 acceptance
report present it as scheduler-path reachability — the first re-appearance of the
evidence-overstatement pattern since 0016. The mutation perimeter excludes the exact
builders that fabrication leans on. One genuine epistemic-behavior defect was found on the
embodied path (workplace knowledge silently dropped rather than classified as remembered),
and one INV-020 letter violation (an unversioned materialized payload arm the 0018 census
quietly skips).

Two notes that distinguish this pass from its predecessors:

- For the first time, the **highest-severity residue is entirely inside the lock layer
  and its evidence documents**, not in kernel behavior. No canonical-day value is wrong;
  no cognition path reads hidden truth; possession parity, debug quarantine, witness
  fail-closed behavior, and the content provenance gates all verified substantive.
- The blocker count is zero for the second consecutive pass, and the high count is one.
  0018's prediction that the seventh pass would be "verification-only and materially
  cheaper still" mostly held: of 0018's nine findings, eight verified substantive without
  qualification; the ninth (`ORD-HARD-037`) landed its vocabulary, masks, and seeds but
  fabricated its terminal reachability.

## 1. Scope

### In scope

- Generative lock tier honesty: terminal/interruption reachability provenance, oracle
  negative coverage, corpus diversity floors
  (`crates/tracewake-core/tests/generative_lock.rs`, `tests/support/generative.rs`,
  `crates/tracewake-core/src/scheduler.rs` no-human entry points).
- Mutation-testing perimeter and CI gating semantics
  (`.cargo/mutants.toml`, `.github/workflows/ci.yml` `mutants-in-diff` and
  `mutants-lock-layer` jobs).
- Embodied freshness classification of workplace knowledge
  (`crates/tracewake-core/src/agent/perception.rs`,
  `src/epistemics/projection.rs`).
- Materialized-payload version closure and census derivation
  (`src/events/apply.rs`, `src/state.rs`,
  `crates/tracewake-core/tests/anti_regression_guards.rs`).
- Checksum canonicalization of agent record prose (`src/checksum.rs`).
- Witness-census direction completeness (`tests/anti_regression_guards.rs`,
  `src/agent/no_human_surface.rs`).
- Seed-knowledge authoring posture (`crates/tracewake-content/src/schema.rs`,
  `src/fixtures/*`).
- Embodied interruption surfacing (`src/projections.rs`, the TUI render path).
- Evidence-document honesty: the 0018 acceptance report and the conformance index rows
  it feeds (`reports/0018_ord_life_cert_scoped_acceptance.md`,
  `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`).

### Out of scope

- Re-auditing Phase 1 / 1A spine internals (0010–0012) and Phase 2A epistemic internals
  (0013), except where Phase 3A consumes them.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014–0018 fixed that this audit verified as holding
  (see §3 "Verified holding").

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-009, INV-017/018/019/020,
  INV-024/025/026/028, INV-044, INV-062, INV-066/070/071, INV-091…INV-098, INV-105.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — one
  freshness rule: staleness reclassifies knowledge, it does not delete it.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — replay
  must detect tampered history; checksum identity must derive from typed state, not prose.
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` and
  `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — a
  gate's evidence must be produced by the path under test; acceptance evidence must not
  overstate.
- The lineage's enforcement reading: a lock that is green because the test harness
  manufactured the state it asserts on has failed even though it passes.

No new external research was consulted this pass; the 0017/0018 lock-design corpus
(swarm testing, deterministic chaos points, policy lints, dependency-policy censuses)
remains the reference for lock design.

## 3. Determination

**Positive verdict.** The in-scope Phase 3A surface is not maximally locked, and two
findings are letter-level foundational violations (`ORD-HARD-046` epistemic freshness,
`ORD-HARD-047` payload versioning). Nine findings follow,
`ORD-HARD-044` … `ORD-HARD-052`: one high, four medium, four low, zero blockers.

### Verified holding (no action; recorded so they are not re-litigated)

- **ORD-HARD-035 (witness honesty) is substantive.** `transaction.rs::witness_kind_allowed`
  defaults `_ => false` with every legitimate stable_id enumerated;
  `no_human_surface.rs::add_role_assignment_notice` mints the presence facts as
  `observed_now` citing a genuine same-tick same-place `ObservationRecorded` resolved by
  `scheduler.rs::latest_current_place_perception_event_id` — not the role notice;
  `workplace_presence_facts_cite_current_place_observation_not_role_notice` and
  `provenance_witness_notice_only_workplace_presence_fails_closed_before_proposal` prove
  both directions; `guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms`
  asserts per-id arms, the presence of `_ => false`, and the absence of any
  wildcard-true arm. (The census's missing reverse direction is `ORD-HARD-049`.)
- **ORD-HARD-036 (episode replay evidence) is substantive.**
  `OrdinaryLifeEpisodeRecord.payload_fields` is populated from the shared
  `payload_fields(event)` helper and joined into the canonical checksum line;
  `episode_tamper_output_tag_poisons_replay` and `episode_tamper_proration_poisons_replay`
  mutate genuinely-emitted fields (panicking if absent) and assert
  `!replay.agent_checksum_matches`;
  `materialized_agent_payload_records_keep_payload_fields` guards the three record
  structs. (The fourth materialized record's omission is `ORD-HARD-047`; the prose
  `summary` component is `ORD-HARD-050`.)
- **ORD-HARD-037 (generative vocabulary) landed its registry, world seeding, masks, and
  seeds.** `registry()` registers the Phase 3A sleep/eat/work/continue vocabulary;
  `initial_world` seeds sleep affordances, food supplies, and workplaces; `mask_for_seed`
  yields six distinct swarm masks across the expanded `GENERATIVE_SEEDS`;
  `assert_marker_append_does_not_change_physical_checksum` asserts **both** physical and
  agent checksums. (The fabricated terminal reachability is `ORD-HARD-044`; oracle
  residuals are `ORD-HARD-048`.)
- **ORD-HARD-038 (duration charge coverage) is substantive.** `sleep.rs::need_delta_event`
  and the `work.rs` delta builders emit `elapsed_ticks` with `cause_kind="action_effect"`;
  `apply.rs::assert_single_tick_delta_charge`'s `action_effect` arm is reachable and
  writes checksummed `need_tick_charges`;
  `golden_fixtures_run.rs::assert_no_duplicate_need_regime_charges` derives
  `action_effect` intervals from the delta's own payload (no sibling-terminal
  `unwrap_or(0)` remains) and requires a positive interval;
  `single_tick_delta_charge_rejects_overlapping_action_effect_duration` proves the panic.
- **ORD-HARD-039 (payload versioning) landed for the named kinds.** The episode,
  `CandidateGoalsEvaluated`, and `ContinueRoutine*` arms call
  `require_payload_version(&payload, "payload_schema_version", "1")`;
  `forged_payload_schema_version_rejected_for_materialized_agent_replay_001` proves loud
  live and replay rejection. (The `NeedThresholdCrossed` arm it skips is `ORD-HARD-047`.)
- **ORD-HARD-040 (seed knowledge grammar) landed at the loader.**
  `load.rs::seed_event_log` mints `household_food_source` beliefs only from authored
  `known_food_sources` edges — the omniscient nested loop is gone;
  `seeded_food_source_unknown_to_all_actors_omits_seed_belief_and_actor_known_food`
  proves an unknown food source mints no belief and is planner-unreachable; the new
  field is registered in `CONTENT_FIELD_REGISTRY` and passes the field censuses. (The
  default-on omniscience authoring helper is `ORD-HARD-051`.)
- **ORD-HARD-041 (validation token) is substantive.**
  `FixtureSchema::to_agent_state` consumes a `FixtureValidationToken` whose `mint()` is
  `pub(crate)` and minted only by `accepted_world` after `validate_fixture_errors`
  returns empty; compile-fail doctests block external construction; `validate_need_band_u16`
  rejects out-of-band values before the token exists, so `clamp_need_value` is a no-op on
  every authored seed.
- **ORD-HARD-042 (workplace witness/freshness) landed its fact shape.**
  `ActorKnownWorkplaceFact` carries `source_event_ids: SourceEventIds` + `acquired_tick`,
  both folded into `canonical_key`;
  `projections.rs::workplace_availability_provenance_refs` threads the witnessing notice
  event ids into the embodied why-not provenance;
  `stale_workplace_notice_superseded_by_newer_001` proves the newer notice wins. (The
  visibility filter the shared classifier sits behind is `ORD-HARD-046`.)
- **ORD-HARD-043 (census durability) is substantive.**
  `CONTENT_NEGATIVE_PROOFS` ties every `NumericFieldPolicy`/`StringScanPolicy` variant to
  a named `_rejected_001` proving test with a parity census;
  `positive_fixture_constructor_ids_from_source` derives the fixture census from
  `src/fixtures/*.rs` source and asserts equality with `fixtures::all()`; the scheduled
  mutation job uses the normalized `comm -23` ratchet against
  `.cargo/mutants-baseline-misses.txt`. (The perimeter and in-diff semantics are
  `ORD-HARD-045`.)
- **0018 §5 tier 6 posture locks landed.**
  `workspace_dependency_posture_matches_allowlist` asserts `tracewake-core`'s
  `[dependencies]` is empty and the workspace set matches
  `WORKSPACE_DEPENDENCY_ALLOWLIST`; conformance-index rows exist for the witness census,
  episode payload coverage, seed-knowledge grammar, zero-dependency census, and
  generative reachability (the last one overstates — `ORD-HARD-044`).
- **Cross-cutting checks clean this pass:** the no-human scheduler builds every
  autonomous proposal through `ActorDecisionTransaction::run` (INV-103/104);
  `transaction.rs::planner_goal_for` and `htn.rs::resolve_condition` read only
  actor-known facts; `decision.rs::source_class_for_provenance` maps all forbidden
  provenances to `is_forbidden_for_cognition`; goal selection is a deterministic total
  order (`candidate_goal_order_is_total_and_stable`, INV-038); passive charges exclude
  asleep/working ticks (`classify_actor_tick_regimes`), so duration and passive charges
  cannot double-bill a tick; duplicate duration terminals are typed errors; work
  need-gates read authoritative state, not forged proposal params; debug quarantine
  holds (`DebugCapability::mint()` crate-private, compile-fail doctests,
  `debug_truth_never_enters_holder_known_context_hash`, `DEBUG NON-DIEGETIC` panel
  prefixes); possession parity holds
  (`human_binding_adds_no_extra_semantic_actions`,
  `human_after_authorization_matches_scheduler_validation` asserting identical status,
  reason codes, and appended events); the embodied why-not path forwards only
  `actor_visible_summary`/`actor_visible_facts`; event ordering, cause linkage, and
  envelope schema-version gates verified; seed epistemics provenance (INV-063) and the
  forbidden-content scanners verified.

### Validated — no action (checked, found intentional or correctly scoped)

- `assert_single_tick_delta_charge` panics rather than returning a typed `ApplyError` on
  a forged duplicate duration charge. This is the release-build-assert mechanism 0017 §5
  tier 2 deliberately mandated; converting it to a typed rejection is a design option,
  not a defect. Recorded; no action this pass.
- Instantaneous `action_effect` deltas (eat) carry no `elapsed_ticks` and are exempt from
  the per-tick regime locks. Correct semantics — eat is not a per-tick regime charge.
  Forward note: a future duration-eat must adopt the `elapsed_ticks` contract or it will
  silently escape the lock.
- Authored affordances may target items no actor knows
  (`seeded_food_source_unknown_to_all_actors_001` authors an `eat` affordance toward
  hidden food deliberately). Affordances are truth-side possibility space (INV-044);
  knowledge gating correctly happens at the actor-known projection, and the hidden-food
  gate proves the planner cannot target it. Not a validation gap.
- Envelope-level `participants`/`place_id`/`process_id`/`random_draws` on materialized
  agent events are not in the agent checksum. Currently safe: episode semantics
  (including place) ride in the payload, which is checksum-covered, and Phase 3A has no
  authoritative randomness. Forward note recorded for the phase that makes any of these
  load-bearing on agent events.
- `run_no_human_day` (the production day loop, also the TUI's path via
  `app.rs::run_no_human_day`) processes due completions via `append_due_completions` —
  the completion machinery itself is sound; `ORD-HARD-044` is strictly about what the
  generative oracle proves, not about production reach.

## 4. Findings and remediation requirements

### ORD-HARD-044 — Generative duration-terminal and interruption reachability is fabricated by a test-side helper, and the conformance index and acceptance report present it as scheduler-path reachability

**Severity:** high.

**Responsible layers:** `test_oracle` (generative tier), evidence documents.

**Doctrine breached:** lock durability; INV-091…098 spirit (a gate's evidence must be
produced by the path under test); the lineage's acceptance-evidence honesty contract —
this is the first overstatement re-appearance since the 0016 corrections.

**Evidence:** `generative_lock.rs` advances each generated case via
`no_human::advance_no_human`, whose body contains no `PendingCompletion` handling — the
completion machinery (`append_due_completions`, `build_sleep_completion_events`,
`build_work_completion_events`) lives on the `run_no_human_day` path only. The test then
calls its own `append_generated_duration_terminals`, which invokes the completion
builders directly and appends the results to the log. `Reachability`'s
`duration_terminal` and `interruption` counters (and the `terminal_kinds.len() >= 2`
diversity floor) are therefore satisfied by harness-fabricated events: a regression that
broke scheduler-driven completion scheduling would leave this lock green. The
conformance index row "0018 generative reachability contract"
(`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`) states the generated
sequences "exercise … duration terminals, interruptions," and
`reports/0018_ord_life_cert_scoped_acceptance.md` §4 presents the reachability counters
without disclosing the fabrication — both overstate.

**Why existing gates miss it:** the oracle asserts on the post-append log, conflating
"the builder produces an event when called" with "the advance path reaches it"; nothing
distinguishes advance-emitted events from test-appended ones.

**Required correction:** drive the generative corpus through the completion-bearing
path — either `run_no_human_day` per window, or extend the generative advance entry to
process pending completions — and derive every `Reachability` counter solely from events
the advance call emitted. Demote `append_generated_duration_terminals` to a
builder-unit test or delete it. Correct the conformance-index row to describe what is
proven; add a dated correction section to the 0018 acceptance report (the
`## 2026 correction (spec 0017)` pattern from the 0016 report).

**Structural lock:**

- The reachability assertions recompute from the advance-call return (appended-event
  list), not from the final log, so post-hoc appends cannot feed them.
- A guard asserting `generative_lock.rs` contains no direct calls to the completion
  builders outside the designated builder-unit test (the source-scan census pattern).

### ORD-HARD-045 — The mutation perimeter excludes the duration-terminal builders, the in-diff job swallows tool failure, and no mutation gate covers direct pushes

**Severity:** medium.

**Responsible layers:** `test_oracle`, CI.

**Doctrine breached:** none directly; lock durability.

**Evidence:** `.cargo/mutants.toml` `exclude_globs` lists
`crates/tracewake-core/src/actions/defs/**`; both the scheduled `mutants-lock-layer`
`-f` allowlist and the `mutants-in-diff` guarded-path grep cover only `agent/**`,
`scheduler*`, `projections*`, and `actions/pipeline.rs`. The interruption/proration and
completion logic in `actions/defs/sleep.rs` and `work.rs`
(`sleep_interruption_reason`, `build_sleep_interruption_events`, the work completion
builders) — the exact code `ORD-HARD-044`'s fabricator leans on — is never
mutation-tested; a mutant flipping the severe-need interruption predicate survives every
gate. Separately, the `mutants-in-diff` job runs
`cargo mutants --in-diff … || true` and treats a missing `mutants.out/missed.txt` as
success ("all in-diff mutants caught", `exit 0`): a cargo-mutants crash or timeout
passes green. The job is also gated `if: github.event_name == 'pull_request'`, and the
scheduled job on `workflow_dispatch || schedule` — a direct push to `main` runs neither.

**Why existing gates miss it:** the perimeter is a hand-maintained allowlist; the
source-classification rationale for the defs files claims coverage by "targeted action
and pipeline guards," which mutation does not actually reach; `|| true` discards the
tool's exit semantics.

**Required correction:** remove `actions/defs/**` from `exclude_globs` and add
`actions/defs/sleep.rs` + `actions/defs/work.rs` (and `eat.rs` if nontrivial) to both
jobs' filters, refreshing the miss baseline once; in the in-diff job, distinguish tool
failure from zero-missed (branch on cargo-mutants' documented exit codes; require
evidence that mutants were generated when the guarded-diff flag is true); either add a
push-triggered diff-vs-`HEAD^` run or record the required-PR branch rule as the
compensating control in the conformance index.

**Structural lock:** a guard asserting the mutants filter set and `exclude_globs` stay
consistent with the source-classification table's "mutation-covered" claims (parse both,
the census pattern), so a future exclusion must update the rationale or fail CI.

### ORD-HARD-046 — Embodied workplace knowledge is silently dropped unless its notice tick coincides with the latest current-place perception tick

**Severity:** medium.

**Responsible layers:** `holder_known_context`, `projection`.

**Doctrine breached:** INV-028 (staleness reclassifies knowledge — a stale notice
surfaces as remembered; it does not vanish); INV-024/026 in spirit (an under-grant is
still an epistemic-channel defect); architecture doc 03 (one freshness rule — the rule
here is a visibility gate, not a classifier). This is the residual of the otherwise
substantive `ORD-HARD-042` fix.

**Evidence:** `perception.rs::current_place_knowledge_context` applies
`if !classified.is_latest_current_place_record() { continue }` as a hard visibility
filter to **all** record kinds, including `ActorKnownProjectionRecord::Workplace`.
`epistemics/projection.rs::is_latest_current_place_record` is true only when the
record's source tick equals the latest current-place perception tick. A role-assignment
notice is remembered routine knowledge whose source tick is the notice tick — generally
not the latest perception tick — so the workplace fact survives into the embodied
context only by tick coincidence. The proving test
`current_place_knowledge_context_applies_freshness_to_workplace_notices` emits
perceptions at exactly the two notice ticks (4 and 7, with the context built at tick 8),
masking the coupling — operator-verified.

**Why existing gates miss it:** the supersession fixture uses coincident ticks, and the
masking test's "newest wins" outcome is itself produced by the same-tick filter dropping
the older notice — it cannot distinguish supersession from place-gating; no test
covers notice tick ≠ latest perception tick; the no-human surface acquires workplace
knowledge through a different path (`add_role_assignment_notice`), so canonical-day
goldens never exercise the embodied drop.

**Required correction:** separate supersession (newest record per workplace wins) from
current-place visibility. Routine-assignment-class records are not place-gated: a
remembered role notice surfaces with `Remembered` freshness classification (planning-
available per INV-028) regardless of the latest perception tick; only genuinely
place-bound observation records stay behind the current-place gate.

**Structural lock:** a `perception.rs` test seeding the role notice at tick T and the
latest current-place perception at tick T+5, asserting the workplace fact still appears
in `actor_known_workplaces()` with `acquired_tick() == T` and remembered freshness —
this fails on the current `continue`.

### ORD-HARD-047 — `NeedThresholdCrossed` materializes without a payload version gate, and the 0018 censuses hand-enumerate around it

**Severity:** medium.

**Responsible layers:** `replay`, `events`, `test_oracle`.

**Doctrine breached:** INV-020 (payloads must be versioned enough that replay rejects
unsupported history); lock durability (a census whose membership is hand-typed
overpromises — the recurring `ORD-HARD-034`/`043` shape).

**Evidence:** the `apply.rs` arm for `EventKind::NeedThresholdCrossed` inserts
`NeedThresholdCrossedRecord` into `state.need_threshold_crossings` using bare
`required(&payload, …)` with no `require_payload_version` call — unlike its episode,
candidate, and continue siblings. The census
`materialized_agent_apply_arms_require_payload_schema_version`
(`anti_regression_guards.rs`) enumerates only those sibling arms, and
`materialized_agent_payload_records_keep_payload_fields` checks only the three
payload-bearing record structs — both are hand-typed lists, so the fourth materialized
record (and any future one) is invisible to them. (The noop-allowlist census's
materialized-kind list also names `FoodServiceUsed`; it is already version-gated inside
the multi-kind episode arm — verified at its `require_payload_version` call site — so
the threshold arm is the only ungated materialized kind.) Threshold *values* are
checksum-covered as typed columns, so tampering is caught; the gap is the absence of a
loud unsupported-version rejection — the precise INV-020 concern — plus the census
wording claiming universality it does not check.

**Why existing gates miss it:** census membership is enumerated, not derived from the
set of apply arms that write to `AgentState` projection maps.

**Required correction:** stamp and require `payload_schema_version` on the
`NeedThresholdCrossed` payload (builder + apply arm, mirroring the siblings); derive
both censuses structurally — every apply arm containing an insert into an `AgentState`
map must call the version-requiring helper, and every materialized record struct whose
source events carry payload fields beyond its typed columns must store
`payload_fields` (or register a typed-column-closure exemption with rationale).

**Structural lock:** the derived censuses are the lock; a negative replay gate forging
the threshold payload version and asserting loud rejection (the
`forged_payload_schema_version_rejected_…` pattern).

### ORD-HARD-048 — The generative oracle is all-positive and its diversity floors are degenerately satisfiable

**Severity:** medium.

**Responsible layers:** `test_oracle` (generative tier).

**Doctrine breached:** INV-017/018 (a replay-integrity tier with no negative case never
demonstrates detection); lock durability.

**Evidence:** `generative_lock.rs` contains no tamper/forgery relation — every generated
log is honest, and `assert_live_matches_replay` runs only on well-formed histories; the
existing tamper gates live solely in the content crate's golden tests, so no
seed-swept history is ever required to be *rejected*. The diversity floors
(`masks.len() >= 4`, `sequence_lengths.len() >= 2`, `terminal_kinds.len() >= 2`) are
satisfiable by a near-degenerate corpus: no per-family floor requires sleep-success,
sleep-interruption, work-success, and work-failure each to be independently reached, and
reachability flags OR-accumulate across seeds — operator-verified at
`reachability.interruption |= has_event(…, SleepInterrupted)` (the `agent-reported`
residue is only the count: interruption currently reached by exactly one seed, which is
execution-dependent).

**Why existing gates miss it:** the oracle was built as a positive reachability
contract; nothing measures per-family coverage or requires divergence detection over
generated histories.

**Required correction:** add a per-case metamorphic tamper relation — perturb one
payload field of one agent-stream event per generated case and assert
`!matches_expected` / checksum divergence; replace the aggregate `terminal_kinds` floor
with per-family assertions (sleep-success, sleep-interrupt, work-success, work-fail
each nonzero across the corpus, counted from advance-emitted events per
`ORD-HARD-044`); require at least two seeds to contribute to each reachability flag, or
record the single-seed concession explicitly in the conformance row.

**Structural lock:** the per-family floors and the tamper relation are the lock — a
corpus or generator change that starves a family or stops detecting divergence fails
the suite.

### ORD-HARD-049 — The witness census enforces census⊆surface but not surface⊆census

**Severity:** low.

**Responsible layers:** `test_oracle`.

**Doctrine breached:** none directly; lock durability (the `ORD-HARD-035` census claim
is directionally incomplete).

**Evidence:** `guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms`
iterates the hand-maintained `NO_HUMAN_SURFACE_FACT_STABLE_IDS`, asserting each id
appears in `no_human_surface.rs` and has an explicit `witness_kind_allowed` arm. Nothing
extracts the stable_ids actually minted in `no_human_surface.rs` and asserts they are
all census members — a future fact minted without census registration gets no
compile/CI-time witness-arm requirement. The runtime `_ => false` default still fails
closed, so this is a durability gap, not a live leak; all currently minted ids were
cross-checked present.

**Why existing gates miss it:** the guard only walks the census, never the surface.

**Required correction:** extend the guard to regex-extract every string-literal
stable_id passed to the `ActorKnownFact` constructors in `no_human_surface.rs`
production code and assert that set is a subset of `NO_HUMAN_SURFACE_FACT_STABLE_IDS`.

**Structural lock:** the bidirectional census is the lock (the
`SCANNED_STRING_FIELDS` parity pattern).

### ORD-HARD-050 — Display prose (`effects_summary`) is an authoritative checksum input on all three agent record families

**Severity:** low.

**Responsible layers:** `replay`, `holder_known_context` evidence surface.

**Doctrine breached:** INV-105 in inverse (display strings must not be the
authoritative substrate — here prose is folded into replay identity); the repo-level
"no simulation fact born from prose" recurring test.

**Evidence:** the canonical checksum lines for `ordinary_life_episode`,
`candidate_goal_evaluation`, and `continue_routine_arbitration` (`checksum.rs`) each end
in `…|payload={}|summary={}`, where `summary` is the cloned free-form
`effects_summary` display string. Two replays differing only in summary prose would
diverge — coupling replay identity to display text. All semantic content is already
covered by `payload_fields`, making the prose component redundant as evidence and
hazardous as identity.

**Why existing gates miss it:** tamper tests rely on payload coverage; no test asserts
prose is excluded (contrast `controller_and_debug_metadata_are_excluded`).

**Required correction:** drop `summary` from the canonical checksum lines (keep it on
the records for display); this reprices agent checksums once — batch the golden churn
with `ORD-HARD-047`'s payload addition.

**Structural lock:** a checksum test asserting two states differing only in a record's
`summary` hash identically (the metadata-exclusion pattern).

### ORD-HARD-051 — The omniscience-restoring fixture helper is default-on across the authored corpus

**Severity:** low.

**Responsible layers:** `content_seeding`.

**Doctrine breached:** INV-025/INV-062 in spirit (authored ignorance is now
*expressible* — `ORD-HARD-040` — but the authoring grammar's path of least resistance
re-mints uniform omniscience); anti-contamination posture.

**Evidence:** `schema.rs::FixtureSchema::populate_known_food_sources_for_all_actors`
builds the full actor × food-supply cross product as `known_food_sources` edges and is
called by 51 fixture files — every food-bearing fixture except the hidden-food negative.
This is the explicit-edges-for-existing-fixtures route 0018 §9 itself recommended for
checksum stability, so the helper's existence is sanctioned; the residue is that it is
an unguarded one-liner any future fixture can reach for, and the canonical corpus
contains no partial-knowledge fixture (only all-or-nothing).

**Why existing gates miss it:** the field censuses verify classification, not
selectivity; the fixture census verifies registration, not knowledge posture.

**Required correction:** pin the helper's call sites with an allowlist census (existing
golden fixtures only; a new fixture using it must either join the allowlist with a
rationale or author explicit per-actor edges), and add at least one canonical fixture
with genuinely partial food knowledge (some actors know, some don't) so partial-belief
seeding has a living exemplar in the golden corpus.

**Structural lock:** the call-site allowlist census is the lock.

### ORD-HARD-052 — The embodied `salient_interruption` surface is dead: hardwired `None` at its only producer

**Severity:** low.

**Responsible layers:** `projection`, TUI.

**Doctrine breached:** INV-066/071 (a mechanic hidden from play is unfinished —
sleep/routine interruption is a live Phase 3A mechanic with no embodied surfacing);
INV-070 in spirit.

**Evidence:** `projections.rs::phase3a_status` constructs
`Phase3AEmbodiedStatus { …, salient_interruption: None }` — the field's only producer.
The TUI render arm for it is unreachable. The no-human scheduler computes interruption
metrics and `SleepInterrupted` events are materialized, but none of that signal reaches
the possessed actor's own status line.

**Why existing gates miss it:** no test asserts a non-`None` value; struct shape and
renderer presence satisfy shallow checks. No phase-ladder deferral cite for embodied
interruption surfacing was found in `docs/2-execution/03` or `12`; if the implementer
locates one, this finding downgrades to a recorded deferral instead.

**Required correction:** derive `salient_interruption` from the viewer's own
actor-known interruption evidence (the materialized `SleepInterrupted` /
work-failure episode records, viewer-keyed) — never from other actors' state.

**Structural lock:** a fixture interrupting the bound actor's sleep and asserting
`phase3a_status.salient_interruption.is_some()` and that the embodied render emits the
interruption line.

## 5. Anti-contamination lock layer (consolidated)

Tiers extend the 0016/0017/0018 layer.

1. **Compile-time impossibility:** none required this pass — the type-level locks from
   0018 verified substantive and sufficient.
2. **Runtime gates:** `payload_schema_version` requirement extended to
   `NeedThresholdCrossed` (`ORD-HARD-047`).
3. **Test-oracle corrections:** advance-emitted-only reachability derivation and the
   builder-call ban in the generative suite (`ORD-HARD-044`); per-family terminal floors
   and the per-case tamper relation (`ORD-HARD-048`); the non-coincident-tick workplace
   freshness test (`ORD-HARD-046`); derived materialized-arm/record censuses
   (`ORD-HARD-047`); bidirectional witness census (`ORD-HARD-049`); prose-exclusion
   checksum test (`ORD-HARD-050`); helper call-site allowlist census and a
   partial-knowledge canonical fixture (`ORD-HARD-051`); the embodied interruption
   fixture (`ORD-HARD-052`).
4. **Source guards:** generative-suite census banning direct completion-builder calls
   outside the builder-unit test (`ORD-HARD-044`); mutants-perimeter↔classification
   consistency guard (`ORD-HARD-045`).
5. **Mutation/CI posture:** duration builders inside the mutation perimeter with a
   refreshed baseline; in-diff job failing loud on tool error; push-gap closed or
   recorded as a compensating-control row (`ORD-HARD-045`).
6. **Evidence honesty:** corrected conformance-index generative row; dated correction
   section in the 0018 acceptance report (`ORD-HARD-044`) — the lineage's
   overstatement-correction convention.

## 6. Documentation corrections (housekeeping, same package)

- Conformance index (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`):
  rewrite the "0018 generative reachability contract" row to state what the corrected
  tier proves (advance-emitted reachability, per-family floors, tamper relation); add
  rows for the mutation-perimeter guard and the seed-knowledge helper allowlist.
- `reports/0018_ord_life_cert_scoped_acceptance.md`: add a dated correction section
  (`## 2026 correction (spec 0019)`) recording that §4's duration-terminal and
  interruption reachability was produced by a test-side builder helper, not the
  generative advance path, and that interruption coverage was single-seed.
- No doctrine amendment; INV-001…INV-110 are applied, not changed.

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0019_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. The corrected generative tier's corpus summary: per-family terminal counts derived
   from advance-emitted events only, the tamper-relation divergence proof, seed set and
   masks, zero differential divergences.
2. The mutation run over the expanded perimeter: refreshed baseline, the
   duration-builder mutants caught, the in-diff failure-mode demonstration (tool error
   ≠ pass).
3. The non-coincident-tick workplace freshness proof (notice tick ≠ perception tick,
   fact surfaces as remembered).
4. The `NeedThresholdCrossed` forged-version rejection proof and the derived-census
   outputs.
5. Per-actor need ledgers / checksum diffs for the `ORD-HARD-047` + `ORD-HARD-050`
   golden repricing, every diff explained (the 0016 §7.1 format).
6. The partial-knowledge fixture proof and helper-allowlist census output.
7. The embodied interruption surfacing proof.
8. The corrected conformance row and the 0018 report correction, quoted.
9. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`; not
   full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment.
- Crate direction preserved: core depends on nothing at runtime; content on core; tui on
  core + content. No new dependencies, dev or production (the 0018 tier-6 census makes
  this testable).
- Recommended ticket ordering: `ORD-HARD-047` + `ORD-HARD-050` first and together (both
  reprice agent checksums — batch the golden churn once); then `ORD-HARD-046` (embodied
  context behavior, with its freshness test); then `ORD-HARD-044` + `ORD-HARD-048` (the
  generative tier rebuilt over the corrected surface); then `ORD-HARD-045` (mutation
  perimeter, after the new tests exist so the baseline refresh is honest); then
  `049`/`051`/`052`; documentation corrections (§6) land with `044`.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **ORD-HARD-044's path change may surface latent generator/scheduler mismatches.**
  Driving generated cases through the completion-bearing path may reveal that some
  generated sequences were only replay-stable because completions never fired. Treat any
  such divergence as evidence the lock was hollow, not as a reason to keep the
  fabricator: fix the generator's world/window seeding until the advance path genuinely
  reaches the regimes.
- **ORD-HARD-045's baseline refresh must be honest.** Expanding the perimeter will
  surface new missed mutants in the duration builders; the refreshed
  `.cargo/mutants-baseline-misses.txt` must be reviewed, not bulk-accepted — new misses
  in interruption predicates warrant tests before baselining.
- **ORD-HARD-046 may change embodied context hashes** for fixtures where notice and
  perception ticks differ; the per-actor ledger format covers the review.
- **ORD-HARD-047 + ORD-HARD-050 reprice golden checksums** (payload addition + prose
  removal). One batched repricing, fully explained, per §7.5.
- **ORD-HARD-052's deferral question:** if embodied interruption surfacing turns out to
  be phase-deferred by a cite this audit missed, downgrade to a recorded deferral and
  document it in the conformance index instead of implementing.
- **Next-iteration assessment (the recurring question):** findings were found, so an
  eighth pass is warranted by the lineage's own rule. But the trend is unambiguous:
  zero blockers for two consecutive passes, the product-behavior kernel verified clean,
  and the residue concentrated in the lock layer's honesty about itself. The eighth
  pass should be a **scoped verification-only audit of 0019's deliverables plus the
  evidence documents** — materially cheaper again. If it returns clean or low-only, drop
  the per-pass cadence and move to per-phase-entry audits (Phase 3B / Phase 4
  boundaries), as 0018 §9 anticipated. The one watch item arguing for that eighth pass
  rather than skipping straight to per-phase cadence: this pass found the lineage's
  first evidence overstatement since 0016, and overstatements are exactly what a
  verification pass exists to catch.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (or is explicitly lock-durability)
  and names responsible layers from the execution diagnostic vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted); every finding was independently operator-verified at its
  load-bearing symbols; the one sub-detail resting on delegated evidence alone is
  marked `agent-reported` inline (the `ORD-HARD-048` seed count).
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression.
- [x] Verified-holding items from 0014–0018 are recorded to prevent re-litigation; the
  evidence-overstatement re-appearance is recorded with its correction path.
- [x] No doctrine amendment; no compatibility shims; no new dependencies; crate direction
  preserved.
- [x] Scope stays within the Phase 3A ordinary-life surface and its lock/evidence layer.

## Outcome

Completed 2026-06-11.

Implemented through tickets `0019PHA3AGENREA-001` through `0019PHA3AGENREA-009` and
archived under `archive/tickets/`. The completed work closed payload-version and
checksum coverage gaps, separated workplace freshness from place-gating, corrected the
generative reachability contract and 0018 evidence overstatement, added terminal-family
floors and payload-tamper locks, expanded the sleep/work mutation perimeter with loud CI
failure semantics, made the no-human witness census bidirectional, constrained blanket
seed-knowledge helpers with a partial-knowledge fixture, surfaced embodied
interruptions from viewer-owned episode evidence, and added the scoped acceptance
artifact at `reports/0019_ord_life_cert_scoped_acceptance.md`.

Deviations from plan: none requiring downgrade. The `0019PHA3AGENREA-008` deferral
re-check found no active embodied-interruption deferral cite, so the implementation
branch landed. The focused mutation run over `sleep.rs` and `work.rs` produced no missed
mutants (`83 mutants tested in 3m: 66 caught, 17 unviable`), so no baseline refresh was
needed for that focused perimeter.

Verification included the per-ticket targeted commands recorded in the archived ticket
outcomes and the capstone report. Final gates run for the completed implementation:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace --all-targets --locked`
4. `cargo test --workspace`
