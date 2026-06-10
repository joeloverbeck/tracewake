# 0018 Phase 3A Provenance Witness, Episode Replay Evidence, Generative Reachability, and Seed Epistemics Hardening Spec

**Status**: PROPOSED
**Staging path:** `specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md`

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `a9c62e0` (post-0017 closeout; all `0017PHA3ATICLED` tickets landed, merge PR #25).
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the sixth Phase 3A alignment pass — the verification-leaning pass that 0017 §9
itself warranted. Spec 0014 closed `ORD-HARD-001`–`007`; 0015 closed `008`–`013`; 0016
closed `014`–`025`; 0017 closed `026`–`034`. This audit re-derived the normative contract
from `docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, and
`docs/3-reference/*`, re-examined the full surface introduced by archived spec 0005 at the
post-0017 baseline, and — per the audit directive — treated every prior correction
(including 0017's own deliverables) as unverified until proven in code. Findings continue
the `ORD-HARD` series at `035`.

All evidence below was verified against local sources at the target baseline. Citations use
named symbols, which are grep-stable; line numbers are omitted deliberately. Every finding
in this spec was independently operator-verified at the cited symbols (none rest on
delegated review alone).

The root pattern this pass: **0017 fixed the evidence layer's headline gaps but the
witness/freshness machinery still trusts defaults and the lock tiers still trust the
histories they happen to generate.** The witness-compatibility table fails open for
unlisted fact kinds; the one fact family that is minted `observed_now` from a record slips
through that default; ordinary-life episode payloads remain semantically tamperable; and
the new generative tier cannot reach the sleep/work/interruption regimes it was built to
protect.

Two notes that distinguish this pass from its predecessors:

- For the first time in the lineage, **no acceptance-report overstatement was found**: the
  0017 acceptance report's claims verified clean, and the generative tier's known
  reachability limitation was honestly conceded in the conformance index rather than
  overstated. The limitation is still a finding (`ORD-HARD-037`), but the audit history is
  intact.
- The blocker count is zero. 0017's prediction that a sixth pass "should be materially
  cheaper" held: no canonical-day value is wrong, no gate asserts the negation of its
  name. The residue is provenance-class honesty, replay evidence depth, and lock reach.

## 1. Scope

### In scope

- Witness-compatibility and provenance-class integrity of the no-human actor-known surface
  (`crates/tracewake-core/src/agent/transaction.rs`, `src/agent/no_human_surface.rs`,
  `src/agent/perception.rs`, `src/epistemics/knowledge_context.rs`).
- Replay verifiability of ordinary-life episode payloads and payload schema versioning
  (`src/events/apply.rs`, `src/state.rs`, `src/checksum.rs`).
- Tick-charge runtime-assert coverage for action-emitted duration deltas
  (`src/events/apply.rs`, `src/actions/defs/sleep.rs`, `src/actions/defs/work.rs`, the
  reconciliation gate in `crates/tracewake-content/tests/golden_fixtures_run.rs`).
- Generative lock tier reach and corpus diversity
  (`crates/tracewake-core/tests/support/generative.rs`, `tests/generative_lock.rs`).
- Content-seed epistemic expressiveness and load-path integrity
  (`crates/tracewake-content/src/load.rs`, `src/schema.rs`, `src/validate.rs`).
- Census durability residuals (content-negative registration, fixture-registration
  derivability, the scheduled mutation job in `.github/workflows/ci.yml`).

### Out of scope

- Re-auditing Phase 1 / 1A spine internals (0010–0012) and Phase 2A epistemic internals
  (0013), except where Phase 3A consumes them.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014/0015/0016/0017 fixed that this audit verified as holding
  (see §3 "Verified holding").

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-002, INV-009, INV-018/020,
  INV-022, INV-025/026/028, INV-039/045, INV-063, INV-099…INV-110 (especially INV-102
  provenance sufficiency and INV-105 authoritative diagnostics).
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` —
  provenance classes must be causally honest; "a string label is never sufficient proof";
  witness-compatibility must fail closed.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — one
  holder-known cognition source, one freshness rule.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — replay
  must be able to detect tampered history, not merely re-apply it.
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` and
  `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — authored
  seeds create possibility space; silent migration is forbidden.
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` —
  ORD-LIFE-CERT clauses; need accounting honesty.

External research consulted for lock design, not authority override (extends the 0017
corpus; items adopted in §5 are marked):

- Swarm testing (Groce et al., ISSTA 2012) — per-run feature-mask omission to diversify
  generated states: `https://dl.acm.org/doi/10.1145/2338965.2336763` (adopted, §5 tier 5)
- FoundationDB BUGGIFY-style deterministic chaos points:
  `https://transactional.blog/simulation/buggify` (recorded option, not adopted this pass)
- clippy `disallowed_methods`/`disallowed_types` policy lints:
  `https://rust-lang.github.io/rust-clippy/master/index.html#disallowed_methods`
  (adopted where applicable, §5 tier 4)
- Dylint path-scoped custom lints: `https://github.com/trailofbits/dylint` (recorded
  option; deferred — toolchain-churn cost exceeds current need given the source-census
  guards)
- cargo-deny dependency-policy gates: `https://github.com/EmbarkStudios/cargo-deny`
  (concept adopted as an in-tree zero-dependency census instead, §5 tier 5 — no new
  tooling dependency)
- Sealed traits / capability tokens: already in use (`DebugCapability`); extended pattern
  recorded for the witness table census.
- Research-grade, recorded and not adopted: Cocoon static IFC
  (`https://arxiv.org/pdf/2311.00097`), Flux refinement types
  (`https://arxiv.org/abs/2207.04034`), MCMAS epistemic model checking, checked coverage
  (Schuler & Zeller).

## 3. Determination

**Positive verdict.** The in-scope Phase 3A ordinary-life substrate is not fully aligned
with the foundation pack and not maximally locked. Nine findings follow,
`ORD-HARD-035` … `ORD-HARD-043`: three high, four medium, two low, zero blockers.

### Verified holding (no action; recorded so they are not re-litigated)

- **ORD-HARD-026 (tick attribution) is substantive.** `scheduler.rs::run_no_human_day`
  advances `last_decision_tick_by_actor` via `latest_action_tick_delta_tick`; the
  reconciliation gate `golden_fixtures_run.rs::assert_no_duplicate_need_regime_charges`
  counts occurrences in a `BTreeMap<(actor,need,tick), u64>` (no label-set collapse);
  `wait_then_window_passive_charges_each_tick_once_001` is a real adversarial fixture
  (wait at tick 1, second-window passive `elapsed_ticks == 3`); the release-build
  `apply.rs::assert_single_tick_delta_charge` writes checksummed
  `AgentState::need_tick_charges`. (Coverage gap for `action_effect` deltas is
  `ORD-HARD-038`.)
- **ORD-HARD-033 (duration keying) is substantive.** All three consumers key on
  `EventCause::Event` via `need_accounting.rs::open_body_exclusive_starts` /
  `terminal_ticks_by_start`; duplicate terminals are typed errors at apply
  (`ApplyError::DuplicateDurationTerminal`) and poison rebuild
  (`duplicate_duration_terminal_poisons_rebuild_001`);
  `event_cause_terminal_closes_start_with_divergent_proposal_id` proves forged divergent
  proposal ids cannot reopen.
- **ORD-HARD-027 (freshness rule) is substantive.**
  `projection.rs::classified_actor_known_records_for_context` + `record_freshness` is the
  single classifier; `CurrentlyPerceived` requires a `Visible*` source at the latest
  current-place window; `aged_food_record_surfaces_as_remembered_belief_not_observation`
  proves a tick-4 observation surfaces at tick 9 as `remembered_belief` with original
  acquisition tick. The provenance-class audit
  (`transaction.rs::provenance_class_mismatch_diagnostic`) fails closed with
  `BlockerCode::ProvenanceClassMismatch`. (The workplace-presence bypass is
  `ORD-HARD-035`; the embodied workplace staleness exception is `ORD-HARD-042`.)
- **ORD-HARD-028 (replay payload evidence) landed for the four named kinds.**
  `CandidateGoalEvaluationRecord` / `ContinueRoutineArbitrationRecord` store full
  `payload_fields` and are checksummed (`AGENT_STATE_CHECKSUM_COVERAGE`);
  `AGENT_WORLD_NOOP_ALLOWLIST` shrank to `[FoodConsumed, NoHumanDayStarted,
  NoHumanDayCompleted]`; tamper-flip gates
  (`continue_routine_tamper_kind_flip_poisons_replay`,
  `continue_routine_tamper_reason_rewrite_poisons_replay`) assert `!matches_expected`;
  the capstone asserts `rebuild.decision_context_hash_failures.is_empty()`. (The episode
  family was not covered — `ORD-HARD-036`.)
- **ORD-HARD-029 (believed-access) is substantive — not a placeholder this time.**
  `ActorKnownWorkplaceFact` carries `believed_access_open` into `canonical_key` and hash
  inputs; `projections.rs::phase3a_semantic_actions` derives availability from belief via
  `with_availability` (the literal-`true` ban
  `guard_014_phase3a_semantic_actions_do_not_use_literal_true_availability` holds); the
  divergence fixture pair is real and structurally divergent
  (`embodied_workplace_availability_reflects_belief_not_truth_001` seeds truth-open /
  belief-closed; `embodied_workplace_believed_open_truth_closed_commit_fails_001` asserts
  `Accepted` then `WorkBlockFailed` with `blocker_kind=access` — the INV-106 loop);
  `perception.rs` is a GuardedLayer whose allowlist excludes `state.workplaces`.
- **ORD-HARD-030 (content policy) is substantive.** `validate.rs::NumericFieldPolicy`
  classifies pressure/relief/duration/need-band with caps derived from `NEED_MAX`;
  out-of-band `InitialNeedSchema.value` is rejected (`invalid_need_band`) before
  materialization on the production path; the union scanner
  (`reject_text_by_policy(StringScanPolicy::Union)`) covers `output_tag` and labels; the
  numeric/string censuses parse `schema.rs` via `discover_schema_fields` and fail on
  unclassified fields.
- **ORD-HARD-031 (witness wiring) landed for the framing facts.**
  `scheduler.rs::latest_current_place_perception_event_id` cites the same-tick
  `ObservationRecorded`; `latest_need_event_id` cites the latest `NeedDeltaApplied`;
  structured fields derive from facts via `DerivedActorKnownFields::from_facts` with
  bidirectional reconciliation proven. (The table's fail-open default is
  `ORD-HARD-035`.)
- **ORD-HARD-032 (lock durability) is substantive.**
  `ActorKnownFact.source_event_ids: SourceEventIds` (newtype; empty unconstructable);
  `dangling_provenance_diagnostic` returns `BlockerCode::ProvenanceDangling` on empty;
  `cognition_inputs_are_context_backed` scans all `production_sources()` workspace-wide
  with a rationale-bearing allowlist and a synthetic exempt-file violation proven to
  fail; guard scans whitespace-normalize; `cargo-mutants` pinned at 27.1.0 in both CI
  jobs; the PR-time `mutants-in-diff` job applies a real `comm -23` new-miss ratchet.
- **ORD-HARD-034 (audit history) closed.** The 0016 acceptance report carries the dated
  correction section (`## 2026 correction (spec 0017)`) recording all three
  overstatements; the conformance index
  records the overturns; the upcast question resolved via the spec-permitted
  rejection-only contract (`event_schema_registry_has_one_current_version_with_migration_proof`,
  `unsupported_event_schema_append_rejected`/`…replay_rejected`).
- **Cross-cutting checks clean this pass:** scheduler builds proposals only through
  `ActorDecisionTransaction::run` (INV-103/104); HTN/planner resolve every condition from
  actor-known state (`htn.rs::resolve_condition`, `transaction.rs::planner_goal_for`);
  debug quarantine holds (`DebugCapability::mint()` crate-private, compile-fail doctests,
  `debug_truth_never_enters_holder_known_context_hash`); possession parity holds
  (`human_after_authorization_matches_scheduler_validation`,
  `human_binding_adds_no_extra_semantic_actions`); stored-ordering and poisoned-rebuild
  integrity hold; no authoritative RNG exists in the no-human scheduler (INV-017
  vacuously satisfied); checksum coverage is structurally total over `AgentState` fields
  (`checksum_coverage_is_total_for_authoritative_state` parses `state.rs` and fails on
  uncovered new fields); CI `lock-layer-gates` runs all gate suites `--locked` with no
  `continue-on-error` or path filters.

## 4. Findings and remediation requirements

### ORD-HARD-035 — Workplace-presence facts are minted `observed_now` from a record-derived notice, and the witness-compatibility table fails open

**Severity:** high.

**Responsible layers:** `holder_known_context`, `proposal_construction`.

**Doctrine breached:** INV-102 (false provenance: `observed_now` requires a genuine
perception witness; a record event witnessing a physical-presence claim reviews to
nothing), foundation doc 14 (witness compatibility must fail closed; a string label is
never sufficient proof). This is the residual that `ORD-HARD-031` was meant to close.

**Evidence:** `no_human_surface.rs::add_role_assignment_notice` pushes
`actor_at_workplace`, `assigned_workplace_known`, and `at_workplace` via
`ActorKnownFact::observed_now(...)` with source
`"evented_perception:actor_at_noticed_workplace"`, stamped at `self.decision_tick`, citing
the `RoleAssignmentNoticeRecorded` notice's `source_event_ids` — the only ids the
projection workplace record carries. The notice witnesses neither the actor's location nor
a perception. `transaction.rs::witness_kind_allowed` has no arm for those three stable_ids
and ends with `_ => true`, so the witness audit passes them silently. The work-block HTN
path depends on these facts: `htn.rs::resolve_condition` requires `modeled_fact_proof` for
`ActorAtWorkplace`/`AtWorkplace`/`AssignedWorkplaceKnown`.

**Why existing gates miss it:** `provenance_class_mismatch_diagnostic` fires only on
`fact.tick() < decision_tick`; these facts are stamped at decision tick.
`provenance_witness_mismatch_fails_closed_before_proposal` proves one explicitly-tabled
mismatch fails; nothing asserts unlisted stable_ids fail. The fail-open default is itself
the structural door: any future fact kind citing any real event passes the witness audit.

**Required correction:** mint the presence facts honestly — either require a same-tick
`ObservationRecorded` witness for the `observed_now` class (cite the actor's genuine
current-place perception event, which the surface already locates via
`latest_current_place_perception_event_id`), or downgrade them to the notice-derived
class (`routine_assignment`/remembered) that their actual witness supports, with the HTN
conditions updated to accept the honest class. Invert the table default to `_ => false`
and enumerate every legitimate stable_id explicitly.

**Structural lock:**

- A census test asserting every `ActorKnownFact` stable_id constructed anywhere in
  `no_human_surface.rs` (and any future surface builder) has an explicit
  `witness_kind_allowed` arm — parity in the `SCANNED_STRING_FIELDS` style, so a new fact
  kind without a declared witness set fails CI.
- A test constructing each workplace-presence fact with a notice-only witness and
  asserting the typed `provenance_witness_mismatch` failure (or, under the downgrade
  option, asserting the honest class).
- Guard banning `_ => true` (any wildcard-true arm) in `witness_kind_allowed`.

### ORD-HARD-036 — Ordinary-life episode payloads are replay-unverifiable; the allowlist census citations are hand-typed strings

**Severity:** high.

**Responsible layers:** `replay`, `test_oracle`.

**Doctrine breached:** INV-018, INV-105 (an event class whose semantic content can be
tampered without any gate failing is decorative history). The `ORD-HARD-028` pattern,
persisting for the episode family that pass did not reach.

**Evidence:** `apply.rs` materializes `SleepStarted`/`WorkBlockStarted`/`SleepCompleted`/
`SleepInterrupted`/`WorkBlockCompleted`/`WorkBlockFailed`/`EatFailed` into
`state.rs::OrdinaryLifeEpisodeRecord`, which carries only `summary: String`
(= `effects_summary`) — unlike its siblings `CandidateGoalEvaluationRecord` and
`ContinueRoutineArbitrationRecord`, which store `payload_fields`. The build sites set
fixed summaries ("sleep started; completion is duration scheduled") while the payloads
carry `duration_ticks`, `output_tag`, `workplace_id`, `body_exclusive`,
`non_economic_output`, `sleep_place_id`, and failure `blocker_kind`/`completion_emitted` —
none of which enter `summary`. `checksum.rs` emits
`ordinary_life_episode|…|summary={}` with no payload component, and `replay/rebuild.rs`
never re-derives these fields. A stored log whose `WorkBlockStarted.output_tag` or
`duration_ticks` is rewritten passes `report.rs::run_replay::matches_expected`.
Separately, the allowlist census
(`anti_regression_guards.rs::agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state`)
"cites" each entry's coverage via an in-test literal `match` (e.g.
`EventKind::FoodConsumed => "dual_stream_physical_food_supply_checksum"`, `_ => ""`) that
no code verifies against a real checksum family.

**Why existing gates miss it:** terminal need/physical effects materialize via separate
checksummed events, masking the start/failure markers; no test tampers an episode payload;
the census asserts membership, not verifiability.

**Required correction:** add `payload_fields: Vec<(String, String)>` to
`OrdinaryLifeEpisodeRecord`, populated by the existing `payload_fields(event)` helper, and
emit it in the canonical checksum line (the arbitration-record pattern). Derive the
allowlist census citations: each allowlisted non-marker kind must resolve to an entry in a
checksum-coverage/dual-stream registry; each marker kind must have a registered
payload-free schema.

**Structural lock:**

- Tamper tests rewriting `WorkBlockStarted.output_tag` and `SleepInterrupted` proration
  fields in a copied golden log, asserting `!agent_checksum_matches` and
  `!matches_expected`.
- A guard asserting every materialized agent record type whose source events carry
  payload stores `payload_fields` (parse `state.rs` record structs, the
  checksum-coverage-census pattern).

### ORD-HARD-037 — The generative lock tier cannot reach the regimes it was built to protect

**Severity:** high (hollow reach, honestly conceded but still unfit for purpose).

**Responsible layers:** `test_oracle` (generative tier).

**Doctrine breached:** none directly; lock durability — 0017 §5 tier 5 mandated
reachability evidence for "regime transitions, interruptions, and terminal kinds" and a
single-charge property over generated histories that would have caught `ORD-HARD-026`
"without anyone authoring the adversarial fixture."

**Evidence:** `tests/support/generative.rs::registry()` registers only
`register_phase1_inspect_wait()`; `generate_case` emits only scheduled-wait windows
(fixed four-window shape, durations 1–4) over exactly four seeds (`GENERATIVE_SEEDS`).
The corpus therefore contains only `ActorWaited` + passive `NeedDeltaApplied` — no
`SleepStarted`, no work blocks, no interruptions, no duration terminals.
`generative_lock.rs::Reachability` asserts only `actor_waited`, `need_delta`,
`no_human_marker`, `replay_round_trip`, `prefix_replay`. The single-charge metamorphic
property runs only over wait/passive histories — the sleep/work double-charge regime where
`ORD-HARD-026` lived is structurally unreachable. Additionally,
`assert_marker_append_does_not_change_physical_checksum` compares only the physical
checksum; a marker perturbing agent-stream materialization would pass. The conformance
index concedes the limitation ("duration terminal reachability remains covered by targeted
fixtures").

**Why existing gates miss it:** the reachability counters assert only what the wait-only
corpus can satisfy; nothing fails when the corpus never sleeps, works, or interrupts;
corpus size and diversity are unasserted.

**Required correction:** register the Phase 3A sleep/eat/work/continue affordances in the
generative registry; seed generated fixtures with sleep affordances, workplaces, and food;
extend `Reachability` with `sleep_block`, at least one duration-terminal kind, and at
least one interruption, each assertion failing on a zero counter. Expand the seed set and
randomize per-window action-kind selection; adopt swarm-testing feature masks (per-run
omission of action kinds from the vocabulary) to diversify reached states. Extend the
marker-append relation to assert both physical and agent checksums.

**Structural lock:** the extended reachability assertions are the lock — a future change
that starves the generator of a regime fails the suite; a corpus-diversity assertion
(distinct terminal kinds observed, distinct sequence lengths) backstops seed shrinkage.

### ORD-HARD-038 — The release-build single-charge assert is dead for duration deltas, and the reconciliation gate's interval derivation is forge-fragile

**Severity:** medium.

**Responsible layers:** `action_validation`, `test_oracle`.

**Doctrine breached:** INV-045/INV-039 (the 0017 §5 tier-2 runtime invariant was mandated
to cover all charge classes); INV-009 (a gate that silently skips an interval cannot
detect double attribution).

**Evidence:** `apply.rs::assert_single_tick_delta_charge` early-returns unless the
payload carries `elapsed_ticks`; the sleep/work `NeedDeltaApplied` payloads (built by
`sleep.rs::need_delta_event` and the `work.rs` delta builders) carry
`actor_id/need_kind/delta/cause_kind="action_effect"/cause_action_id` but no
`elapsed_ticks` (the field exists only on the lifecycle terminal events). The assert's
`"action_effect"` match arm is therefore unreachable: asleep/working ticks are never
written to `need_tick_charges`, and a forged same-tick duplicate duration charge would not
trip the runtime assert. Separately, the test-oracle gate
`golden_fixtures_run.rs::assert_no_duplicate_need_regime_charges` derives `action_effect`
intervals by locating a sibling terminal with matching keys and falls back to
`.unwrap_or(0)` — a duration delta with a missing or key-divergent terminal silently
contributes an empty interval to the occurrence count.

**Why existing gates miss it:** the test-oracle gate covers `action_effect` on the golden
histories (masking the runtime gap); all fixtures emit coincident terminals, so the
`unwrap_or(0)` fallback is never exercised.

**Required correction:** add `elapsed_ticks` to the sleep/work need-delta payloads (the
builders already hold the values) so the runtime assert records duration ticks; once
present, derive the reconciliation gate's `action_effect` intervals from the delta event's
own payload (single source), dropping the sibling-terminal lookup, and assert a positive
interval for every `action_effect` charge that resolves to a duration regime.

**Structural lock:** a unit test applying two `action_effect` deltas covering an
overlapping tick and asserting the release-build panic; golden-checksum diffs from the
payload addition explained in the acceptance artifact per-actor ledger.

### ORD-HARD-039 — Materialized agent payloads have no schema-version gate

**Severity:** medium.

**Responsible layers:** `replay`, `events`.

**Doctrine breached:** INV-020 (payloads must be versioned enough that replay rejects
unsupported history rather than silently inventing repairs).

**Evidence:** the `apply.rs` arms for `CandidateGoalsEvaluated`,
`ContinueRoutine{Proposed,Accepted,Rejected}`, and the episode group insert records
without any `require_payload_version`-style check — `apply.rs` contains exactly two
such call sites plus the helper's definition, covering only the
`DecisionTraceRecorded`/`StuckDiagnosticRecorded` family
(`trace_schema_version`/`diagnostic_schema_version`). A future payload-shape change
to the unversioned agent kinds cannot be loud-rejected; replay would silently absorb it.

**Why existing gates miss it:** the event-schema registry gates the envelope version only;
no census requires per-payload versions on materialized agent kinds.

**Required correction:** stamp a `payload_schema_version` on the unversioned materialized
agent payloads and require it at apply (mirroring the decision-trace gate).

**Structural lock:** a census asserting every materialized agent `EventKind`'s apply arm
calls the version-requiring helper (parse the apply-arm table, the existing census
pattern); a negative gate replaying a forged version and asserting loud rejection.

### ORD-HARD-040 — Seeds grant every actor omniscient food-source knowledge; authored ignorance is unrepresentable

**Severity:** medium.

**Responsible layers:** `content_seeding`.

**Doctrine breached:** INV-002/INV-025 in spirit (belief-first doctrine requires partial
and wrong belief to be authorable; a seed grammar that can only express
everyone-knows-everything forecloses the required mechanics), foundation doc 09 (seeds
author possibility space). Provenance marking itself is correct (INV-063 holds).

**Evidence:** `load.rs::seed_event_log` runs a nested loop — for every actor × every
`fixture.food_supplies` entry it appends a `StartingBeliefRecorded`
(`household_food_source`) carrying the food supply's true location. No
`FoodSupplySchema`/fixture field can express an actor who does not know a given food
source; the canonical fixtures therefore start with uniformly true, uniformly shared
food-location belief.

**Why existing gates miss it:** `phase3a_load_emits_authored_prehistory_seed_events`
asserts the seed beliefs exist and are marked `authored_prehistory` — presence, not
selectivity; no fixture seeds an unknown food source.

**Required correction:** add per-actor knowledge authoring to the schema (e.g. authored
`known_food_sources` edges, defaulting to unknown), with the existing union-scan/registry
treatment for the new field; mint starting beliefs only for authored edges.

**Structural lock:** a fixture seeding a food source no actor knows, asserting no
`household_food_source` belief is minted for it and that the actor's planner cannot
target it (the `hidden_food_*` gate pattern); the numeric/string field censuses force the
new schema field through policy classification automatically.

### ORD-HARD-041 — `to_agent_state` silently clamps out-of-band needs when reached without validation

**Severity:** medium.

**Responsible layers:** `content_seeding`.

**Doctrine breached:** INV-022 (no load-time mutation of authored facts); execution doc 08
("silent migration is forbidden"). The 0017 `ORD-HARD-030` fix rejects at the validation
gate, but the materialization path remains independently reachable.

**Evidence:** `schema.rs::FixtureSchema::to_agent_state` is `pub` and calls
`NeedState::initial`, which calls `need.rs::clamp_need_value` — an out-of-band authored
value materializes clamped with no error. The production path
(`load.rs::load_fixture_package`) validates first, so today's fixtures are safe; nothing
structural prevents a future caller from materializing an unvalidated fixture,
reintroducing the authored-vs-materialized disagreement.

**Why existing gates miss it:** `fixture_initial_need_out_of_band_rejected_001` proves the
validation gate; no test asserts the materialization path is unreachable unvalidated.

**Required correction:** make `to_agent_state` consume a proof-of-validation token (the
`accepted_world` pattern) or return `Result` and reject out-of-band values at
construction, so the clamp can never fire on authored seeds.

**Structural lock:** the token/type change is the lock (unvalidated materialization
unconstructable); a compile-fail or typed-error test for the unvalidated path.

### ORD-HARD-042 — The embodied workplace fact bypasses the freshness rule and carries no event witness

**Severity:** low.

**Responsible layers:** `holder_known_context`, `projection`.

**Doctrine breached:** INV-026/INV-102 (acquisition time and review-sufficient provenance
on the believed-access fact); architecture doc 03 (one freshness rule). The epistemic
residue of the otherwise-substantive `ORD-HARD-029` fix.

**Evidence:** `perception.rs::current_place_knowledge_context` exempts
`ActorKnownProjectionRecord::Workplace` from the `is_latest_current_place_record`
staleness guard that filters routes/food/sleep records, so a stale role notice is always
re-surfaced as current to the embodied menu. `knowledge_context.rs::ActorKnownWorkplaceFact`
stores `workplace_id/place_id/believed_access_open/source_key` — the witness is a free
`String`, with no `SourceEventIds` and no acquisition tick, so the embodied why-not
("You know that workplace access is closed.") reviews to a context id rather than the
witnessing notice event.

**Why existing gates miss it:** the divergence fixtures use a single notice (no
supersession case); guards assert the fact is context-backed and un-hard-coded, not that
it is witnessed or fresh.

**Required correction:** route workplace records through the shared freshness
classification (stale notices surface as remembered, still planning-available per
INV-028); carry `SourceEventIds` + acquisition tick on `ActorKnownWorkplaceFact` and
thread them into the action-availability provenance.

**Structural lock:** fixture `stale_workplace_notice_superseded_by_newer_001` asserting
the newer notice wins; a test asserting the disabled workplace entry's provenance refs
include the role-notice event id.

### ORD-HARD-043 — Census durability residuals: unregistered content negatives, underivable fixture registration, brittle merge-ungated scheduled mutation gate

**Severity:** low.

**Responsible layers:** `test_oracle`, CI.

**Doctrine breached:** none directly; lock durability (and the `ORD-HARD-034`
inline-vs-registered shape, recurring).

**Evidence:**

- The 0017-named content negatives (`fixture_workplace_zero_duration_rejected_001`,
  `fixture_initial_need_out_of_band_rejected_001`,
  `fixture_output_tag_script_marker_rejected_001`) exist as inline `validate.rs` unit
  tests; `negative_fixture_runner.rs::registered_negative_fixtures_match_directory_census`
  enforces only `tests/negative-fixtures/`. No census ties `NumericFieldPolicy` /
  `StringScanPolicy` variants to proving rejections.
- `fixtures_load.rs` asserts `fixtures::all()` equals a hand-maintained set; a new
  `pub fn *_001` fixture never added to `all()` is silently unexercised (contrast the
  source-tree census that catches new files).
- The scheduled `mutants-lock-layer` CI job's check is an exact
  `diff -u .cargo/mutants-baseline-misses.txt mutants.out/missed.txt` (fails on
  line-number drift, unlike the PR job's normalize-and-`comm` ratchet) and gates no
  merge; its failures surface only if someone reads the scheduled run.

**Required correction:** a content-negative registry (policy variant → proving rejection
test) with a census; derive the positive-fixture census from the `fixtures` module source
(count `pub fn *_001` constructors) and assert parity with `all()`; switch the scheduled
mutation check to the in-diff job's normalized ratchet and surface failures (e.g. issue
creation), or record the exact-diff brittleness as an accepted forcing function in the
conformance index.

**Structural lock:** the two censuses are the lock; the CI change is recorded either way.

## 5. Anti-contamination lock layer (consolidated)

Tiers 1–5 extend the 0016/0017 layer; tier 6 hardens the meta-posture.

1. **Compile-time impossibility:** witness-table fail-closed default with a
   stable_id↔arm census (`ORD-HARD-035`); `SourceEventIds` + acquisition tick on
   `ActorKnownWorkplaceFact` (`ORD-HARD-042`); `payload_fields` on
   `OrdinaryLifeEpisodeRecord` enforced by a record-struct census (`ORD-HARD-036`);
   proof-of-validation token on `to_agent_state` (`ORD-HARD-041`); authored-knowledge
   edges as a typed schema field auto-captured by the existing field censuses
   (`ORD-HARD-040`).
2. **Runtime gates (release-build asserts):** single-charge invariant extended to
   `action_effect` duration deltas via `elapsed_ticks` payloads (`ORD-HARD-038`);
   payload-schema-version requirement on all materialized agent kinds (`ORD-HARD-039`).
3. **Test-oracle corrections:** episode tamper-flip gates and derived allowlist citations
   (`ORD-HARD-036`); single-source interval derivation in the charge reconciliation gate
   (`ORD-HARD-038`); the supersession fixture and witness-threading test
   (`ORD-HARD-042`); content-negative and fixture-registration censuses
   (`ORD-HARD-043`).
4. **Source guards:** ban wildcard-true arms in `witness_kind_allowed`; clippy
   `disallowed_methods`/`disallowed_types` entries where a banned call has a stable path
   (recorded in `clippy.toml` with fixture parity per the existing clippy↔fixture
   census).
5. **Generative tier extension (`ORD-HARD-037`):** Phase 3A action vocabulary in the
   generative registry; reachability assertions for sleep blocks, duration terminals, and
   interruptions that fail on zero counters; swarm-testing feature masks per run; corpus
   diversity assertions; agent+physical marker-append relation; scheduled mutation gate
   ratcheted (`ORD-HARD-043`).
6. **Posture locks (new):** an in-tree zero-dependency census test asserting
   `tracewake-core`'s `[dependencies]` table is empty and the workspace dependency set
   matches a committed allowlist (the cargo-deny concept without the dependency);
   conformance-index rows for the witness census, episode payload coverage, the
   generative reachability contract, and the seed-knowledge grammar.

## 6. Documentation corrections (housekeeping, same package)

- Conformance index (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`):
  rows for the witness-compatibility census, episode payload checksum coverage, the
  extended generative reachability contract (superseding the conceded limitation row),
  the seed-knowledge grammar, and the zero-dependency census.
- Execution doc 06: extend the single-charge clause to note the runtime assert covers
  duration (`action_effect`) charges (execution-tier clarification; no doctrine
  amendment).
- No acceptance-report corrections are required this pass: the 0017 report's claims
  verified clean (recorded in §3).

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0018_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. The witness-table census output; the workplace-presence facts' corrected class or
   perception witness, live and under replay; the fail-closed unlisted-stable_id case
   demonstrated.
2. Episode payload tamper-flip proofs (`output_tag` rewrite, proration rewrite) poisoning
   replay; the derived allowlist-citation census output.
3. Per-actor need ledgers for any fixture whose golden checksums change under
   `ORD-HARD-038`'s payload addition, with every diff explained (the 0016 §7.1 format).
4. The generative tier's extended corpus summary: seed set, feature masks, sequence
   count, reachability counters including sleep/terminal/interruption, zero differential
   divergences, both marker-append relations passing.
5. The seed-knowledge fixture proof: an unknown food source minting no belief and being
   planner-unreachable.
6. The unvalidated-materialization rejection proof (`ORD-HARD-041`).
7. The content-negative and fixture-registration census outputs (`ORD-HARD-043`); the
   ratcheted scheduled-mutation configuration.
8. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`; not
   full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment; INV-001…INV-110 are applied, not changed.
- Crate direction preserved: core depends on nothing at runtime; content on core; tui on
  core + content. No new dependencies, dev or production (the §5 tier-6 census makes this
  testable).
- Recommended ticket ordering: `ORD-HARD-035` first (witness honesty changes context
  hashes once), with `038` alongside or immediately after (its payload addition changes
  golden checksums once — batch the fixture churn); then `036`/`039` (replay evidence over
  the corrected payloads); then `040`/`041` (content grammar and load path); then `042`;
  then `037` and `043` last so the extended generative tier and censuses exercise the
  finished surface.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **ORD-HARD-035's class correction may reroute the work routine.** If the presence
  facts downgrade to a notice-derived class, `htn.rs::resolve_condition`'s
  `modeled_fact_proof` requirements must accept the honest class or the canonical day
  stalls at the work block. The fix must land with the HTN condition update in the same
  ticket; the decision (perception-witnessed `observed_now` vs. honest downgrade) is the
  implementer's, recorded in the acceptance artifact.
- **ORD-HARD-038 reprices golden checksums.** Adding `elapsed_ticks` to duration delta
  payloads changes event bytes; the per-actor ledger is the honest review surface. Batch
  with 035's context-hash churn where practical.
- **ORD-HARD-040 changes canonical fixture behavior** if the default flips to unknown:
  existing fixtures must author the edges they currently get implicitly. The mechanical
  edit is bounded (fixtures enumerate few food supplies) but touches every golden
  fixture; the alternative — defaulting existing fixtures to known via explicit edges
  generated once — keeps checksums stable and is recommended.
- **Generative-tier cost growth:** registering the Phase 3A vocabulary multiplies corpus
  runtime. Keep the corpus bounded (seed count × window count) and assert diversity
  rather than volume; the differential oracle's per-boundary checksums already cap
  sequence length.
- **Next-iteration assessment (the recurring question):** this pass found zero blockers
  and no overstated acceptance evidence — the first pass in the lineage where the prior
  pass's locks all verified substantive. The residue concentrated in reach (witness
  defaults, generative vocabulary) rather than in correctness of delivered fixes. A
  seventh pass after 0018 lands is recommended but should be **verification-only and
  materially cheaper still**; if it returns clean or low-only, the cadence can drop to
  per-phase-entry audits (e.g. at Phase 3B/4 boundaries) instead of per-pass audits.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (or is explicitly lock-durability)
  and names responsible layers from the execution diagnostic vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted); every finding — including mediums and lows — was independently
  operator-verified against the code at the target baseline, not taken from delegated
  review alone.
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression.
- [x] Verified-holding items from 0014–0017 are recorded to prevent re-litigation; the
  absence of acceptance-report overstatements this pass is recorded.
- [x] No doctrine amendment; no compatibility shims; no new dependencies; crate direction
  preserved.
- [x] Scope stays within the Phase 3A ordinary-life surface.
