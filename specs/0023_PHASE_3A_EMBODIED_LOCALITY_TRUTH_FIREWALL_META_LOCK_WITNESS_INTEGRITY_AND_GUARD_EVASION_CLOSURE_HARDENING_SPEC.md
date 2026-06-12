# 0023 Phase 3A Embodied-Locality Truth-Firewall, Meta-Lock Witness-Integrity, and Guard-Evasion Closure Hardening Spec

**Status**: PROPOSED

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `db4b53a` (merge PR #30: all `0022PHA3ABASTRI` tickets landed, including the `-015`–`-023` baseline-retirement follow-ups; acceptance artifact `reports/0022_ord_life_cert_scoped_acceptance.md`; spec-ledger truthing `956872d`). All four gates (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) verified green at this baseline before the audit.
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the twelfth Phase 3A alignment pass — the strictly scoped verification of
0022's deliverables that 0022 §9 itself prescribed (above all: did the `ORD-HARD-099`
baseline triage actually happen this time, and do the §5.1 meta-locks fire?), plus the
standing 0005 feature-contract conformance check and a fresh blind foundation sweep of
the Phase 3A production surface. Spec 0014 closed `ORD-HARD-001`–`007`; 0015 closed
`008`–`013`; 0016 closed `014`–`025`; 0017 closed `026`–`034`; 0018 closed `035`–`043`;
0019 closed `044`–`052`; 0020 closed `053`–`065`; 0021 closed `066`–`098`; 0022 closed
`099`–`120`. This audit re-derived the normative contract from `docs/0-foundation/*`,
`docs/1-architecture/*`, `docs/2-execution/*`, and `docs/3-reference/*`, and treated
every 0022 correction as unverified until proven in code at `db4b53a`, per the lineage
rule. Findings continue the `ORD-HARD` series at `121`.

All evidence below was verified against local sources at the target baseline. Citations
use named symbols, which are grep-stable; line numbers are omitted deliberately. The
audit was conducted as an eight-slice delegated review (triage/CI/evidence integrity,
meta-lock tier, epistemics gates, kernel event posture, TUI debug quarantine,
census/content/generative hygiene, canonical-day/0005 conformance, fresh blind
foundation sweep) plus an external lock-design research pass. Every high and medium
finding was independently operator-verified at its load-bearing cited symbols during
triage; the seven findings initially carried on agent evidence alone
(`ORD-HARD-130`/`131`/`134`/`135`/`136`/`137`/`139`) were operator-re-verified at
source during spec reassessment at the same baseline (all confirmed, zero refuted),
so every finding is operator-verified.

The headline answers to 0022 §9's questions:

1. **The `ORD-HARD-099` baseline triage genuinely happened.** The
   `.cargo/mutants-baseline-misses.txt` baseline was retired 143 → 0 through real
   focused tests (tickets `0022PHA3ABASTRI-015`–`023`, all archived), the change log
   records the retirement, the prefix pin is gone, and the deferral-phrase guard is
   live. The lineage's first twice-recurred finding is closed with the work product as
   evidence.
2. **The §5.1 meta-locks landed, but two of their three pillars are partially
   decorative** (`ORD-HARD-122`/`123`/`129`): the nonzero-witness rule compares two
   hand-typed literals and never sees a live scan count; the bijection census derives
   membership only from the `fn guard_` prefix, leaving eleven real structural locks
   outside it; the two-sided ratchet's "recorded decrease" is satisfiable by the same
   hand-edit that performs the shrink. The meta-tier is exactly what the planned
   per-phase audit cadence would rest on, so its integrity is this pass's second
   priority.
3. **One product-behavior foundation violation was found, outside the surfaces the
   lineage repeatedly audited** (`ORD-HARD-121`): the embodied view model's physical
   locality surface (`visible_items`, `visible_containers`, `co_located_actors`,
   `visible_doors`, and the semantic actions derived from them) is generated directly
   from raw `PhysicalState`, not from the holder-known context — the verbatim failure
   shape of the constitution's Enforcement reading ("embodied views generated from
   truth rather than actor-known context") and of
   `docs/1-architecture/10`'s generation rule. The Phase 3A surfaces
   (food/sleep/workplace) in the same builder were correctly migrated onto actor-known
   facts; the legacy Phase 1 locality surface sitting beside them was never re-pointed
   and fell between the lineage's audit boundaries for eleven passes.

## 1. Scope

### In scope

- The embodied view model's locality surface and its epistemic migration
  (`crates/tracewake-core/src/projections.rs::build_embodied_view_model`,
  `src/location.rs::visible_locality`, `EmbodiedProjectionSource`,
  `src/agent/perception.rs::current_place_perception_events`,
  `crates/tracewake-tui` consumers).
- The §5.1 meta-lock tier's own integrity
  (`crates/tracewake-core/tests/anti_regression_guards.rs`: `META_LOCK_REGISTRY`,
  `anti_regression_guard_test_names`, `meta_lock_registry_errors`,
  `mutation_baseline_change_log_records`).
- Guard-evasion closure in the scan layer: perception prose-branch laundering,
  consumed-key call shapes, mutation-CI multi-line swallow, panic-guard `.unwrap`,
  embodied-sweep deferral witness, debug-token derivation.
- The policy behavioral test's remaining self-echo and the debug overlay's production
  reachability (`epistemics/projection.rs`, `tracewake-tui/src/render.rs`, `app.rs`).
- Evidence-honesty residue in `reports/0022_ord_life_cert_scoped_acceptance.md` §1.
- Canonical-day intent coherence and the embodied sleep positive proof
  (`crates/tracewake-content/tests/golden_fixtures_run.rs`,
  `crates/tracewake-tui/tests/`).
- Content/generative lock-shape remainder (`tests/support/generative.rs`,
  `events/envelope.rs::cause_required`).

### Out of scope

- Re-auditing Phase 1 / 1A spine internals (0010–0012) and Phase 2A epistemic
  internals (0013), except where the `ORD-HARD-121` migration touches them — the
  locality surface is consumed by the Phase 3A embodied view and is in scope for that
  reason.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014–0022 fixed that this audit verified as holding
  (see §3 "Verified holding").
- The `ORD-HARD-095` INV-087 owner decision (still deferred; untouched).
- Identity-uncertainty mechanics for co-located actors (INV-029): `ORD-HARD-121`'s
  correction must not *block* them, but building them is Phase 3B+ work.

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-008, INV-024, INV-029,
  INV-067/068/069/071, INV-093, INV-022, INV-020, and the Enforcement reading's
  explicit failure shape: "embodied views generated from truth rather than actor-known
  context" (`ORD-HARD-121`).
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` —
  "Embodied view models are generated from a holder-known context for the bound actor
  plus permitted projections"; the minimum embodied view enumerates `visible_items`/
  `visible_doors`/`visible_containers`/`visible_actors`, so the fields are contractual
  — their *source* is what `ORD-HARD-121` corrects.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — the
  embodied view "must not show hidden item locations, … true contents of
  closed/unknown containers"; no execution doc records the locality surface as an
  intended deferral (verified by sweep).
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — R-28 (incomplete correction
  closure: the census prefix scope, the witness literals, the swallow line shape are
  all one-sibling-over residues), R-29 (guard vacuity: this pass found the meta-tier
  itself carrying the decorative shape it polices, plus two new evasion shapes —
  *binding laundering* and *literal witness counts*).
- The lineage's enforcement reading: a correction to a classified defect is complete
  only when every surface in the defect's class is corrected or explicitly exempted
  with rationale — and a lock is real only if its negative case can actually fire on
  the behavior.

**External research consulted this pass** (extends the 0017/0018/0022 corpus; full
applicability notes in the audit record):

- *Stopping criteria for iterated audits:* capture–recapture defect estimation
  (Wohlin & Runeson, JSS 2004; Briand et al., IEEE TSE) and reliability-growth-curve
  inspection control — applicable as a trend heuristic across the twelve passes;
  OpenSSF baseline practice — event-triggered re-audit (release/phase boundaries)
  layered over continuous automated gates is the industry norm once per-change
  findings saturate; Google Tricorder/Error Prone (Sadowski et al., CACM 2018) — a
  defect class's exit criterion is "automated build-failing check exists," not
  "another manual pass found nothing." These directly support the §9 cadence
  recommendation.
- *Guard hygiene without unbounded accumulation:* clippy's `ui_test` self-testing
  pattern (every custom check has a fixture that must fire — the §5 witness repair is
  this pattern); architecture fitness functions (consolidate per-historical-fix guards
  into one executable check per invariant family); test-suite reduction literature
  (use cargo-mutants kill data to find guards no mutant uniquely needs before
  merging). Labuschagne et al. (FSE 2017): obsolete/incorrect tests caused 26% of
  non-flaky CI failures — schedule guard hygiene at the same phase boundaries as
  audits.
- *2024–2026 tooling:* cargo-mutants `--in-diff`/sharding (already adopted); State
  Field Coverage (ASE 2025) — hand-portable oracle metric: which event/projection
  fields do replay oracles actually assert; OracleGuru oracle-quality checklist.
  Flagged not-applicable (so they are not re-researched): Antithesis DST (commercial
  SaaS), Chrysalis (Python-only), mutest-rs (custom rustc driver; optional deep-audit
  tool only). All adopted techniques survive the zero-dependency constraint.

## 3. Determination

**Positive verdict.** The in-scope Phase 3A surface is not foundation-aligned and not
maximally locked. Nineteen findings follow, `ORD-HARD-121` … `ORD-HARD-139`: two
high, eight medium, nine low, zero blockers. One finding is a product-behavior
foundation violation at the letter of the Enforcement reading and the architecture
generation rule (`ORD-HARD-121`); one is a self-defeating meta-guard
(`ORD-HARD-122`). No INV-099…INV-106 truth-firewall violation exists in production
*cognition* — the cognition firewall verified clean for the **sixth consecutive
pass**; `ORD-HARD-121` lives at the embodied *view* boundary (INV-067/024), not in
proposal generation.

### Verified holding (no action; recorded so they are not re-litigated)

- **ORD-HARD-099 closed for real.** `.cargo/mutants-baseline-misses.txt` is empty
  (normalized count 0); `reports/0020_mutants_baseline_disposition.md`'s change log
  records the genuine 143→0 retirement across `0022PHA3ABASTRI-015`–`023`, every
  ticket archived; the final delta matches `MUTANTS_BASELINE_NORMALIZED_COUNT`/
  `FNV1A64`; the `0021PHA3APOSREB-` prefix pin is replaced by `ticket_exists` over
  `tickets/` ∪ `archive/tickets/`; deferral-phrase rationales are rejected with a
  firing synthetic.
- **ORD-HARD-100/101/118 mutation-CI corrections are substantive.** Concurrency
  `group` includes `github.event_name` (synthetic fires); mutants.toml key allowlist
  fails closed on `exclude_re`; `recognized_mutants_exclude_glob` fails closed on
  `**/`-prefixed globs; captures counted over `non_comment_lines` per step block;
  in-diff filter anchored to the single `git diff --name-only` line;
  `.cargo/mutants.toml` exclusion parity enforced against
  `WORKSPACE_SOURCE_CLASSIFICATIONS`. (The multi-line swallow residue is
  `ORD-HARD-128`.)
- **ORD-HARD-102/119 evidence corrections are substantive.** Report §2 retitled;
  scheduled-run status honestly "pending"; the §7-checklist parity guard
  (`acceptance_artifact_0022_maps_spec_section_7_items_to_report_anchors`) runs
  against the 0022 artifact with a firing missing-anchor synthetic. (The §1 stale-text
  residue is `ORD-HARD-133`.)
- **ORD-HARD-105 adversarial restoration is genuine.** `adversarial_truth_world`
  seeds real hidden food (`food_hidden_pantry` in a container) and `hidden_workshop`
  into `PhysicalState`; `context()` seeds partial knowledge via
  `food_observation_event`/`route_observation_event`/`role_notice_event` through
  `apply_epistemic_event` with verified `source_event_ids`; both planner gates assert
  the positive (known reachable) and the negative (hidden absent) — they discriminate
  and would fail on firewall breakage. The embodied gates route through
  `EmbodiedProjectionSource::from_sealed_context`. (The witness's opt-in enforcement
  is `ORD-HARD-131`.)
- **ORD-HARD-106/115 landed at the classifier with one real behavioral negative.**
  `supersede_newest_by_subject_requires_subject_extractor` proves a synthetic
  non-Workplace record under the supersede policy panics rather than silently
  dropping. (The remaining surface-level self-echo is `ORD-HARD-126`.)
- **ORD-HARD-107 — the 0022 foundation violation — is fixed, and its family is
  closed.** `build_eat_events` routes through `eat_need_events` →
  `build_need_delta_and_threshold_events` (symmetric `NeedState::threshold_crossing`,
  both directions, across-boundary test firing); the guard perimeter
  (`need_delta_guard_perimeter`) derives membership from
  `actions/defs/*.rs` path shape plus a source-marker predicate, so a future def is
  auto-enclosed; an R-28 sweep found every production `NeedDeltaApplied` constructor
  (eat, wait, work, sleep, both scheduler passive-decay sites) routing through the
  shared emitter.
- **ORD-HARD-108/112/113 kernel corrections are substantive.**
  `open_body_exclusive_starts` returns typed `DuplicateDurationTerminal` converted
  into `NoHumanSchedulerError`; the log-derived panic guard exists with a
  rationale-bearing `PANIC_ALLOWLIST` and firing synthetic; both totality scans
  iterate `EventKind::all()` with real and synthetic bodies routed through the same
  helper. (Residues: `.unwrap(` token gap `ORD-HARD-132`; `cause_required` hand-list
  `ORD-HARD-139`.)
- **ORD-HARD-103/104 embodied corrections are substantive at the render layer.**
  `render_embodied_view` reads only embodied fields; the debug overlay is a separate
  `render_debug_overlay` gated on `debug_available`; both presence and absence tests
  exist; `debug_available` is genuinely derived (`debug_available_for` reads live
  `controller_bindings`, excludes `Detached`; `projections.rs` leaves it `false`);
  the constant-literal producer predicate rejects `= true`/`: true` with a firing
  negative; the `EMBODIED_SURFACE_FIELD_PRODUCERS` rationale is corrected. (Residues:
  overlay production reachability `ORD-HARD-125`; deferral content witness
  `ORD-HARD-124`; hand-listed tokens `ORD-HARD-135`.)
- **ORD-HARD-109/110/116/117 census/content corrections are substantive.** The
  consumed-key derivation recurses through `payload`-argument helpers with the
  oblique `helper(state, &payload)` synthetic firing, and every key consumed in all
  five exemption anchors is reachable (no live false-negative); perception emission
  paths (`current_place_perception_events`, `observation_event`,
  `is_visible_exit_target`) gate only on typed `VisibilityDefault`;
  `RoutineDiagnosticKind` is a closed enum with load rejection
  (`InvalidDiagnosticKind`) and no dual spelling; `PlaceSchema.visibility_default` is
  a required typed column; `assert_in_block_displacement_ordering` reads both bounds
  from engine-emitted events. (Residues: receiver/alias call shapes `ORD-HARD-130`;
  laundering shapes `ORD-HARD-127`; envelope-ban narrowness `ORD-HARD-136`.)
- **ORD-HARD-111/120 canonical-day corrections are substantive.**
  `no_human_day_fixture_has_roster_activity_and_metrics_envelope` asserts
  `WorkBlockCompleted`/`SleepCompleted`/`EatFailed` gated on
  `ordering_key.phase == NoHumanProcess` *before* any manual injection — the
  assertions would fail if the runner stopped closing blocks; the fail-only Mara
  intent is recorded on the fixture; the legacy
  `populate_known_food_sources_for_all_actors` is `#[expect]`-gated and pinned by the
  `known_food_source_helper_census_errors` allowlist so new fixtures must author
  per-actor edges. (Residues: the intent string is unasserted `ORD-HARD-137`; sleep's
  embodied positive `ORD-HARD-138`.)
- **The meta-lock tier is partially substantive.** The registry exists and is
  self-reflexive (`meta_lock_registry_errors` lists its own census lock and
  negative); bijection is enforced for the `guard_*` subset with missing-negative,
  duplicate-negative, and artifact-shaped-routing checks firing; the generative
  two-sided ratchet (`generative_floor_ratchet_errors`) genuinely rejects both
  unrecorded raise and one-sided reintroduction; the mutation increase side genuinely
  fires. (The decorative remainder is `ORD-HARD-122`/`123`/`129`.)
- **The 0005 feature contract is intact end-to-end at `db4b53a`.** Need bands at
  exact §8.1 boundaries (`0..=249/250..=499/500..=749/_`); event-sourced
  decay/crossings replay-reconstructable with the tamper gates firing; defeasible
  routines with `MissingFailureModes` rejection; HTN expansion from actor-known
  context only; `run_no_human_day` genuinely autonomous through
  `ActorDecisionTransaction` + `run_pipeline`; typed refill-free failure proofs;
  possession preserves intention/needs/routines; forbidden shortcut/teleport/quest
  fields rejected; no 0022 lock narrowed product behavior below the 0005 contract.
- **The production cognition firewall is clean (sixth consecutive pass).** Fresh
  blind sweep: `ActorDecisionTransaction::run` consumes only
  `ActorKnownPlanningContext` + `AgentState`; `generate_candidate_goals` reads only
  actor-known facts + needs; provenance diagnostics fail closed before any proposal;
  every non-test `from_observed_parts` call is `NoHumanActorKnownSurfaceBuilder::build`;
  `apply_event`/`apply_agent_event` reject loudly on unsupported schema/payload; TUI
  submits typed attempts only. The R-29 doc extensions and 0022 §6 conformance rows
  all landed.
- **0022 §7 acceptance-artifact obligations spot-verified honest** except the §1
  residue (`ORD-HARD-133`): commit manifest, anchor map, pending-status recording,
  and named test symbols check out.

### Validated — no action (checked, found intentional or correctly scoped)

- `from_observed_parts`'s underscore-bound parameters are intentional: facts are
  authoritative and `DerivedActorKnownFields::from_facts` reconstructs the derived
  maps. The `ORD-HARD-105` "dead params" referred to the `context()` test helper,
  whose params are now consumed to seed event-sourced knowledge — confirmed.
- `mutation_baseline_misses_are_pinned_and_ledgered` carries `witness_min: 0` by
  explicit registry exemption because the baseline is legitimately empty; the
  empty-baseline branch is asserted distinctly. Correctly scoped.
- `record_current_place_perception` minting observation events from `PhysicalState`
  is the lawful truth→event boundary (the modeled direct-sight channel with
  confidence/source), not a leak — it is the pattern `ORD-HARD-121`'s correction
  extends.
- Mara knowing both the empty pantry and Tomas's stew via the legacy blanket seeding
  while the canonical intent is fail-only: correctly scoped (recovery is the
  *recommended*, not required, 0005 §12 variant; the over-grant is contained by the
  locked allowlist census).
- `generative_lock` comparing `report.final_tick` against the generated window
  contract is a legitimate authored-schedule assertion, not a prose-born fact — the
  displacement-ordering proof reads engine events.

## 4. Findings and remediation requirements

Severity calibration notes: `ORD-HARD-121` is rated high where its closest precedent
(`ORD-HARD-107`, a letter-violation with currently-correct behavior, medium) — the
divergence is that 121 is not a single-emission-site residue but the INV-067 boundary
surface itself generated from truth (the verbatim Enforcement-reading failure shape),
spanning four field families plus the semantic actions derived from them, and
structurally failing open for every future concealment, identity-uncertainty, or
staleness mechanic; it is not a blocker because concealment/closed-container filtering
means no *hidden* truth leaks today. `ORD-HARD-122` is rated high where the underlying
rotten-green class (`ORD-HARD-104`/`110`) was medium — the meta-lock built specifically
to kill that class is itself the decorative shape R-29 polices, and the planned
per-phase cadence would rest on it. `ORD-HARD-123`/`124`/`127` are rated medium where
the reporting slices proposed high: each is a first-detection derivation/scope gap
with no live false pass (precedents `ORD-HARD-101`/`104`/`110`, all medium).

### ORD-HARD-121 — The embodied locality surface is generated from raw `PhysicalState`, not the holder-known context: the exact failure shape the Enforcement reading names

**Severity:** high (calibration divergence from `ORD-HARD-107` named above).
**Verification:** operator-verified (the `let state = source.state;` read and
`visible_locality(actor, &state.actors, &state.doors, &state.containers, &state.items)`
call in `build_embodied_view_model`; the architecture generation rule; the absence of
any execution-doc deferral record).

**Responsible layers:** projections, TUI boundary, `holder_known_context`.

**Doctrine breached:** the Enforcement reading's letter ("embodied views generated
from truth rather than actor-known context");
`docs/1-architecture/10`'s generation rule ("generated from a holder-known context
for the bound actor plus permitted projections"); INV-067/024 direction; INV-008;
INV-093 (structurally fail-open toward leakage); INV-029 (no identity indirection
slot for co-located actors).

**Evidence:** `build_embodied_view_model` (`crates/tracewake-core/src/projections.rs`)
sets `let state = source.state;` and calls `visible_locality` (`src/location.rs`),
which iterates the authoritative `PhysicalState` maps directly: items at the current
place become `visible_items`; containers become `visible_containers` (contents shown
when `is_open || contents_visible_when_closed`); other actors with matching
`current_place_id` become co-located actors keyed by true `ActorId`; doors by
`connects_place`. These feed `EmbodiedViewModel.{visible_items, visible_containers,
carried_items, local_actors, visible_doors}` and the open/take/inspect semantic
actions over true IDs. `visible_exit_blocker_summary` likewise reads `state`. None of
this routes through `EpistemicProjection`/`KnowledgeContext` — unlike the Phase 3A
food/sleep/workplace surfaces in the *same function* (`phase3a_semantic_actions`),
which correctly consume only `source.actor_known_*`. The lawful pattern already
exists one module over: `current_place_perception_events`
(`src/agent/perception.rs`) mints observation events from truth for food/sleep/exits;
the locality families simply never got the same treatment. Mitigations verified:
concealed items and closed-container contents are filtered, so the surface
approximates direct sight and no *hidden* truth leaks today.

**Why eleven passes missed it:** the hardening lineage audited the Phase 3A cognition
firewall (proposals, planners, sealed contexts) and the Phase 3A view-model fields;
the locality surface predates 0005 (Phase 1), feeds render and player affordances
rather than cognition, and sits between the audit boundaries — the Phase 2A/3A passes
scoped it out as Phase 1 internals while the Phase 1 passes predate the epistemic
substrate it should consume.

**Required correction:** migrate the locality surface onto the epistemic channel the
sibling surfaces use: extend `current_place_perception_events` (or a sibling emitter)
to record observation events for items, containers, doors, and co-located actors at
the perceived place; project them through `KnowledgeContext`/actor-known records; and
derive `visible_items`/`visible_containers`/`visible_doors`/co-located actors and
their semantic actions from the projection, preserving the concealment semantics
(`is_open`/`contents_visible_when_closed` decide what the observation event *records*,
not what the view filters post-hoc). Carried items may remain body-state (the actor
knows what it carries). Re-derive golden fixtures/logs honestly if the new
observation events reprice them. The correction must leave an explicit seam for
INV-029 identity claims (observations reference an observed-actor identity slot, even
if Phase 3A always resolves it to the true id) without implementing identity
uncertainty now.

**Structural lock:** remove raw `PhysicalState` access from the embodied builder —
drop (or quarantine behind a validation-only newtype) the `state` field of
`EmbodiedProjectionSource` so `build_embodied_view_model` *cannot* read truth
(compile-time impossibility, the lineage's strongest tier); an INV-093 negative
asserting an item/actor physically present with no projected observation for the
viewer does not appear in the embodied view; a staleness positive asserting a
projected-then-moved item renders from the stale observation, not live truth.

### ORD-HARD-122 — The nonzero-witness meta-rule compares two hand-typed literals and never sees a live scan count: the anti-rotten-green lock is itself rotten green

**Severity:** high (calibration divergence from `ORD-HARD-104`/`110` named above).
**Verification:** operator-verified (`witness_count` literal initializers in
`META_LOCK_REGISTRY`; `meta_lock_registry_errors` comparing `witness_count <
witness_min`, both authored numbers).

**Responsible layers:** `test_oracle` (meta-lock tier).

**Doctrine breached:** R-29 (the witness rule's own symptom list: "a census ratchet
cannot fire in the actual CI path"); 0022 §5.1's contract ("every census/scan lock
asserts its iterator matched a nonzero (or pinned-minimum) number of real sites");
0022 §9's own warning that the meta-tier must not be decorative.

**Evidence:** `MetaLockRegistryEntry.witness_count` is populated with literals
(`1`, `3`, or consts) at registry-authoring time; `meta_lock_registry_errors` checks
`witness_count < witness_min` — two authored numbers; no enrolled scan reports its
actual matched-site count into the comparison. Vulnerable real scans assert only
emptiness of a violation set: `physical_mutating_event_kinds_have_explicit_world_apply_arms`,
`agent_stream_event_kinds_have_explicit_agent_apply_arms`,
`embodied_view_option_and_collection_fields_have_reachable_producers` (passes if the
`view_models.rs` struct parse yields zero fields). Concrete evasion: rename a scan's
anchor function so `body_after_marker` reads an empty body — the violation set is
vacuously empty, the suite stays green, and the registry's literal `witness_count: 1`
is unchanged and unrelated.

**Why existing gates miss it:** the registry was reviewed as an artifact (entries
present, minimums declared) — the R-29 presence-shape trap applied to the R-29
countermeasure.

**Required correction:** each enrolled scan returns its real matched-site count from
the scan body; the guard asserts `count >= witness_min` inside the guard
(path-under-test) and feeds the same live count to the registry comparison; remove
the literal `witness_count` field or rename it `witness_min_declared` so an authored
number can never stand in for a measured one.

**Structural lock:** a synthetic anchor-miss (a scan run against a body where its
anchor is absent) must fail via the live-count assertion; the bijection census
requires every scan-shaped lock to enroll a live witness, with a firing negative.

### ORD-HARD-123 — The bijection census derives membership only from the `fn guard_` prefix; eleven real structural locks are born and live outside it

**Severity:** medium (calibration divergence from the reporting slice's high named in
the §4 preamble; precedent `ORD-HARD-101`, one-sibling-over closure gap).
**Verification:** operator-verified (`anti_regression_guard_test_names` strips only
`fn guard_`).

**Responsible layers:** `test_oracle` (meta-lock tier).

**Doctrine breached:** R-28 ("derive membership instead of trusting review memory" —
the derivation covers a naming convention, not the lock population); 0022 §5.1's
"every structural lock".

**Evidence:** `anti_regression_guard_test_names` collects `fn guard_*` only;
`meta_lock_registry_errors` iterates that set. Real structural locks without the
prefix and absent from the derivation include
`scheduler_never_direct_dispatches_primitive_action`,
`no_direct_apply_event_outside_event_replay_or_pipeline`,
`event_apply_remains_only_post_seed_mutation_path`,
`non_world_stream_cannot_change_physical_checksum`,
`checksum_coverage_is_total_for_authoritative_state`,
`adding_event_schema_version_requires_migrator_registration`, and five more. A new
author writing `fn enforce_xyz()` receives zero census enforcement — the exact
silent-birth gap §5.1 promised to close.

**Why existing gates miss it:** the census's synthetic negatives all wear the
`guard_` prefix, so the prefix scope is never exercised.

**Required correction:** derive membership from the full `#[test]` population of the
guard/gate files minus an explicitly-cited non-lock allowlist (each entry with
rationale), or enforce the naming convention itself (a scan failing any structural
assertion-bearing test outside the prefix) — either way the complement must be
closed, not open.

**Structural lock:** a synthetic unprefixed structural lock outside the allowlist
must fail the census.

### ORD-HARD-124 — The embodied sweep's cite-only deferral path matches on file path alone: an orphaned debug surface stays green

**Severity:** medium (calibration divergence from the reporting slice's high named in
the §4 preamble; precedent `ORD-HARD-104`, medium).
**Verification:** operator-verified (the `entry.producer_snippet.is_empty() ||`
short-circuit in `embodied_field_has_registered_producer`).

**Responsible layers:** `test_oracle`, TUI.

**Doctrine breached:** lock durability (R-29) — `ORD-HARD-104`'s required "cite-only
exemption path" lost its content witness in implementation.

**Evidence:** the `debug_only_diagnostics` entry in
`EMBODIED_SURFACE_FIELD_PRODUCERS` carries `producer_snippet: ""`; the predicate's
`entry.producer_snippet.is_empty() || source.contains(...)` short-circuits to `true`
whenever the cited path is present in `producer_sources` — which `view_models.rs`
always is. The field could be orphaned in production with the sweep green. The
`synthetic_constant_literal_embodied_surface_producer` negative covers the constant
shape, not the empty-snippet shape.

**Why existing gates miss it:** no negative exercises an orphaned empty-snippet
deferral.

**Required correction:** the cite-only path must still require the field name present
in the cited source (`source.contains(field_name)`), or carry an explicit deferral
record with its own consumer-existence proof.

**Structural lock:** a synthetic deferral entry whose field is absent from its cited
source must fail; enroll the per-entry match in the repaired live-witness rule
(`ORD-HARD-122`).

### ORD-HARD-125 — `render_debug_overlay` has no production caller: the 0022 debug split quarantined the overlay into unreachability

**Severity:** medium (precedent `ORD-HARD-103`, medium structural-channel finding).
**Verification:** operator-verified (callers of `render_debug_overlay` exist only in
`render.rs` tests; `TuiApp::render_current_view` renders only the embodied view).

**Responsible layers:** TUI.

**Doctrine breached:** INV-071 direction (a mechanic hidden from play is unfinished —
here the *debug* surface 0022 built is invisible from the running TUI); INV-041/068
(debug must be reachable to be "visibly non-diegetic"); the `ORD-HARD-103` correction
contract ("a separate `render_debug_overlay` consumed only by the debug path" — no
debug path consumes it).

**Evidence:** `render_debug_overlay` (`crates/tracewake-tui/src/render.rs`) is called
only from its own test module; no `app.rs`/`run.rs` path invokes it, so the derived
`debug_available` (`ORD-HARD-104`'s fix) gates a surface no production path renders.
The dead-surface sweep counts the overlay body's field reads as consumers but never
checks the overlay itself is production-reachable.

**Why existing gates miss it:** the sweep proves field→render-fn reachability, never
render-fn→app reachability.

**Required correction:** wire the overlay into a production debug command/panel (the
architecture doc already sanctions "opening debug panels" as a TUI command), or
record an explicit cite-only deferral under the repaired deferral mechanics
(`ORD-HARD-124`).

**Structural lock:** a TUI guard asserting every `pub fn render_*` in `render.rs` has
a non-test caller in the app layer, with allowlisted rationale-bearing exemptions.

### ORD-HARD-126 — The policy behavioral test still self-echoes on three of four axes and never drives the actual surfaces; `accessibility_scope` has no surface-level behavioral coverage

**Severity:** medium (precedent `ORD-HARD-106`, medium — direct residue).
**Verification:** operator-verified at the test symbol
(`actor_known_projection_policy_table_drives_record_behavior`); the per-assert
self-echo detail is agent-traced.

**Responsible layers:** `holder_known_context`, `test_oracle`.

**Doctrine breached:** lock durability (R-29 self-echo shape) — `ORD-HARD-106`'s
required "behavioral test per kind driven *from* the table" landed as classifier
checks plus one scope assertion, not surface behavior.

**Evidence:** the test's `classification()`/`embodied_scope()`/`accessibility_scope()`
equality asserts compare `record.policy()` against `policies[kind]` — but
`record.policy()` reads the same table (`actor_known_projection_kind_policy`), so
those asserts cannot fail for any table contents. The one non-echo assertion
(`includes_in_embodied_context`) never instantiates `NoHumanActorKnownSurfaceBuilder`
or `current_place_knowledge_context`; `accessibility_scope`'s only behavioral
exercise is the food-only unit test (sleep and workplace `FromAnyPlace` effects
untested at surface level).

**Why existing gates miss it:** the production-caller guard counts `.policy()` call
sites; the meta-lock negative routes a SharedScan shape, not a behavioral mutation.

**Required correction:** per kind in `actor_known_projection_policy_kinds()`, build
the real no-human surface and the real embodied context from event-sourced records
and assert the *emitted facts/affordances* match the declared
classification/embodied_scope/accessibility_scope — including the
`*_believed_accessible` facts for sleep and workplace.

**Structural lock:** a mutation negative flipping one row's `accessibility_scope` (or
`embodied_scope`) and proving the corresponding surface output changes; enroll in the
repaired witness rule.

### ORD-HARD-127 — The perception prose-branch guard launders through `let` bindings and bare-`String` `starts_with`

**Severity:** medium (calibration divergence from the reporting slice's high named in
the §4 preamble; precedent `ORD-HARD-110`, medium latent scope gap; production
emission paths verified clean today).
**Verification:** operator-verified (`source_line_is_branch_shape` token set).

**Responsible layers:** `test_oracle`, perception channel.

**Doctrine breached:** INV-022 lock durability — the `ORD-HARD-110` widening covers
branch-shaped lines only.

**Evidence:** `perception_visibility_prose_branch_violations` flags a `display_label`
line only when `source_line_is_branch_shape` matches (`if`/`else if`/`while`/`match`/
`&&`/`||`/`.filter(`) or the line carries `.to_lowercase()`/`.contains("hidden")`. A
binding such as `let visible = place.display_label != "…";` followed by
`if visible { … }` evades (the `let` line has no branch token; the `if` line has no
`display_label`); `branches_on_id_substring` requires the literal `.as_str().contains`
/`.as_str().starts_with`, so a bare-`String` `display_label.starts_with("hid")`
evades. The synthetic negative exercises only the `.to_lowercase().contains` shape.

**Why existing gates miss it:** the negatives cover the shapes the scan already
catches (the `ORD-HARD-109` pattern, one scan over).

**Required correction:** flag any production line mentioning `display_label` that is
not an allowlisted typed PayloadField write (drop the branch-shape gate for that
token); broaden substring detection to `.starts_with(`/`.ends_with(`/`.contains(` on
any `display_label`/`*_id` projection regardless of `.as_str()`.

**Structural lock:** two synthetic negatives — a laundered `let`-binding branch and a
bare-`String` `starts_with` — must fail.

### ORD-HARD-128 — The mutation-swallow scan is single-line; a multi-line `cargo mutants` continuation evades it

**Severity:** medium (precedent `ORD-HARD-101`, medium — same bypass-channel family).
**Verification:** operator-verified (per-line
`.filter(|line| line.contains("cargo mutants"))` shape).

**Responsible layers:** `test_oracle`, CI.

**Doctrine breached:** lock durability — the `ORD-HARD-101` swallow correction covers
the physical line containing the literal `cargo mutants` only.

**Evidence:** the scheduled invocation is multi-line (`cargo mutants --workspace \`
with flag continuation lines); appending `|| echo ok` to a continuation line (which
contains a flag, not the literal) is invisible to the scan. The only swallow
synthetic mutates the single-line in-diff invocation — the shape the scan already
catches.

**Why existing gates miss it:** R-29: the negative proves the check only for its
existing shape.

**Required correction:** join YAML continuation lines (collapse trailing `\`) before
scanning, or scan each `run:` block containing `cargo mutants` for any logical
command line carrying a `||`/`&&` suffix.

**Structural lock:** a synthetic appending `|| echo ok` to the scheduled invocation's
continuation line must fail.

### ORD-HARD-129 — The two-sided mutation ratchet's "recorded decrease" is a destination-only presence check: the same hand-edit that shrinks the baseline satisfies the record

**Severity:** medium (precedent: `ORD-HARD-099`'s structural-lock clause, which
specified "count comparison against the ledger's change log"; the delivered check is
string presence).
**Verification:** operator-verified (`mutation_baseline_change_log_records` body —
`lines().any(...)` over `baseline-delta:` + current count/hash markers).

**Responsible layers:** `test_oracle`, CI.

**Doctrine breached:** lock durability (R-29) — the decrease side of the two-sided
ratchet records the destination, never the transition.

**Evidence:** the check passes iff any ledger line contains `baseline-delta:` plus
markers matching *today's* count and hash. There is no prior-value field, no chain,
no proof a reviewed transition occurred: shrink the baseline, add the matching
`baseline-delta:` line in the same edit, and both checks pass — an unrecorded and a
recorded decrease are indistinguishable once the line exists.

**Why existing gates miss it:** the increase side fires genuinely (count/hash
mismatch), which made the family look closed.

**Required correction:** change-log entries carry `from`→`to` count+hash pairs
forming a verifiable chain (each `to` is the next `from`; the head equals the pinned
consts), asserted by walking the chain.

**Structural lock:** a synthetic shrink whose change-log line lacks the matching
`from` predecessor must fail.

### ORD-HARD-130 — The consumed-key derivation is blind to method-receiver and alias call shapes

**Severity:** medium (precedent `ORD-HARD-109`, medium — same derivation, next call
shapes over; no live false-negative today).
**Verification:** operator-verified at reassessment
(`call_arguments_include_payload_binding` strips only `&`/`*` and matches
argument-position `payload` bindings; `payload_helper_calls` records a callee only on
an argument match, so receiver and alias shapes go unrecorded).

**Responsible layers:** `test_oracle`, `replay`.

**Doctrine breached:** lock durability (R-28 closure residue, second iteration).

**Evidence:** `payload_helper_calls`/`call_arguments_include_payload_binding` treat a
callee as payload-consuming only when the payload binding appears in the argument
list; a method with the payload as *receiver* (`payload.consume_into(state)`) or an
alias rebinding (`let view = &payload; helper(view)`) escapes recursion. The
`synthetic_oblique_payload_helper_call` negative covers the `helper(state, &payload)`
arg shape only. All five current exemption anchors verified covered end-to-end — the
gap is future-facing.

**Why existing gates miss it:** same R-29 pattern — the negative exercises the
covered shape.

**Required correction:** treat a callee as payload-consuming when the payload (or a
binding aliased from it via simple `let x = &payload;`/`let x = payload;`) is the
receiver or any argument; track the alias set.

**Structural lock:** negatives `helper_via_payload_receiver` and
`helper_via_payload_alias` must fail.

### Low findings

Each low finding retains the full remediation obligation; the compact format lists
evidence → correction → lock inline. Verification status is tagged per finding;
the findings initially agent-reported were operator-re-verified at their cited
symbols during spec reassessment (all confirmed).

**ORD-HARD-131 — The discrimination witness is opt-in per gate, not enforced across
`context()` callers** (`test_oracle` / low; precedent `ORD-HARD-105`;
operator-verified at reassessment). `planner_hidden_truth_fixture_witness_errors` fires only because the
two existing planner gates call it; a future `context()` caller with a non-empty
adversarial fixture has no structural obligation to run it, and the witness's own
negative proves it fails on an *empty* context, not on a leak. Correction: run the
discrimination check inside `context()` itself (hidden counterparts absent from the
returned context, asserted unconditionally). Lock: a synthetic leaking the hidden id
into the surface must fail inside `context()`.

**ORD-HARD-132 — The log-derived panic guard scans `.expect(`/`assert!(` but not
`.unwrap(`** (`test_oracle` / low; precedent `ORD-HARD-108`; operator-verified). A
future `.unwrap()` on log-derived data in `scheduler.rs`/`apply.rs` would panic on
corrupt history (INV-020) yet escape `log_derived_panic_violations`. Current
production `.unwrap()` sites verified benign (static-string constructions).
Correction: add `.unwrap(` to the token filter with the same allowlist mechanism.
Lock: a synthetic `.unwrap()` on payload data must fail.

**ORD-HARD-133 — Report §1 carries stale text contradicting the achieved zero
baseline** (evidence honesty / low; precedent `ORD-HARD-119`; operator-verified).
`reports/0022_ord_life_cert_scoped_acceptance.md` §1 still reads "All 143 remaining
normalized baseline entries are ledgered…; test debt was filed to the follow-up
series" — but the follow-ups *retired* the baseline to zero (the spec Outcome and
change log record this). The checklist guard checks anchor presence, not narrative
consistency. Correction: amend §1 (or supersede in the 0023 artifact) to state the
143→0 retirement. Lock: extend the checklist guard to assert the §1 narrative count
matches `MUTANTS_BASELINE_NORMALIZED_COUNT`.

**ORD-HARD-134 — The per-entry baseline governance is vacuous on the empty ledger
and the hash pin is the trivial FNV offset basis** (CI / low; precedent
`ORD-HARD-099` residue; operator-verified at reassessment). With zero entries, the deferral-phrase,
repetition, and ticket checks iterate nothing; `canonical_lines_hash` of the empty
set equals `0xcbf29ce484222325`, so the hash pin adds nothing beyond count=0. Benign
today; a future re-population must restore live governance. Correction + lock: an
assertion that any non-empty baseline carries ledgered per-entry dispositions, with a
firing synthetic (one unledgered entry).

**ORD-HARD-135 — The embodied debug-token negative uses a hand-maintained literal
list** (TUI / low; precedent `ORD-HARD-103`; operator-verified at reassessment).
`renderer_keeps_debug_tokens_out_of_embodied_view` asserts absence of four literals;
a future debug field hand-pushed into `render_embodied_view` with new tokens would
leak past it (the dead-surface sweep mitigates but does not close). Correction:
derive the forbidden-token set from a single `DEBUG_TOKENS` const also consumed by
`render_debug_overlay`. Lock: adding a debug field auto-extends the negative; a
synthetic new-token leak must fail.

**ORD-HARD-136 — The generative `EventEnvelope` ban is one constructor substring**
(`test_oracle` / low; precedent `ORD-HARD-117`; operator-verified at reassessment). The fabricator scan
(`generative_duration_terminal_fabricator_errors` in `anti_regression_guards.rs`)
checks the `EventEnvelope::new` substring only; `EventEnvelope::default()`, struct literals,
clones, or aliased re-exports evade. `support/generative.rs` verified to construct
zero envelopes today. Correction: ban the bare `EventEnvelope` token in
`support/generative.rs` except allowlisted imports. Lock: negatives via
`default()` and struct literal must fail.

**ORD-HARD-137 — The recorded fail-only canonical-recovery intent is dead
documentation** (content fixtures / low; precedent `ORD-HARD-120`; operator-verified at reassessment).
The fixture records `canonical_mara_recovery_resolution=fail_only_empty_food_source`
but no test references the string; a future edit flipping Mara's outcome would leave
the recorded intent contradicting runtime behavior silently. Correction: assert the
runner-only Mara outcome (`EatFailed`, no autonomous `FoodConsumed`) matches the
recorded resolution token. Lock: the contract-vs-behavior coherence assertion.

**ORD-HARD-138 — Sleep has no positive embodied reachability proof** (TUI /
low; precedent `ORD-HARD-111`; operator-verified — the sole TUI `"Sleep here"`
assertion is the negation). `tui_acceptance.rs` proves eat/work/continue-routine/wait
embodied-reachable and submittable; sleep is proven only by omission plus kernel
pipeline tests, leaving the 0005 five-action TUI sweep (INV-066/071) four-fifths
positive. Correction: a fixture with an actor-known sleep affordance whose embodied
view exposes a submittable `sleep` semantic action. Lock: extend the embodied-action
reachability coverage to require a positive entry per ordinary action.

**ORD-HARD-139 — `EventKind::cause_required` is a hand-maintained `matches!` list;
the derivation the 0022 spec promised lives in the guard, not the kernel** (`events`
/ low; precedent `ORD-HARD-112`; operator-verified at reassessment). A new variant defaults to
no-cause-required at the kernel level; the guard catches kinds emitted via
`new_caused_v1` in scanned sources, leaving other birth paths open. Correction: make
`cause_required` an exhaustive `match` (no `_` arm) so the compiler forces a
disposition per new variant. Lock: compile-time exhaustiveness (strongest tier);
keep the existing guard as the behavioral cross-check.

## 5. Anti-contamination lock layer (consolidated)

Tiers extend the 0016–0022 layer.

1. **Compile-time impossibility:** the `EmbodiedProjectionSource` truth-access
   removal — the embodied builder cannot name raw `PhysicalState`
   (`ORD-HARD-121`); exhaustive `match` for `cause_required` (`ORD-HARD-139`).
2. **Meta-lock repairs (the §5.1 tier made real):** live witness counts measured by
   the scans themselves and fed to the registry (`ORD-HARD-122`); census membership
   derived from the closed test population minus a rationale-bearing allowlist
   (`ORD-HARD-123`); `from`→`to` chained baseline change log (`ORD-HARD-129`);
   deferral entries with content witnesses (`ORD-HARD-124`). Per the clippy `ui_test`
   pattern from the research corpus: every repaired meta-check gains a fixture
   negative that must fire, routed through the production scan path.
3. **Runtime/product gates:** the locality observation events and projection-derived
   embodied surface with concealment recorded at observation time (`ORD-HARD-121`);
   the debug overlay wired into a production debug path (`ORD-HARD-125`).
4. **Test-oracle corrections:** surface-driven policy behavioral test with a
   row-mutation negative (`ORD-HARD-126`); in-`context()` discrimination witness
   (`ORD-HARD-131`); laundering-resistant perception scan (`ORD-HARD-127`);
   receiver/alias-aware consumed-key derivation (`ORD-HARD-130`); derived debug-token
   negative (`ORD-HARD-135`); canonical-intent coherence assertion (`ORD-HARD-137`);
   positive embodied sleep proof (`ORD-HARD-138`); widened envelope ban
   (`ORD-HARD-136`); `.unwrap` token coverage (`ORD-HARD-132`).
5. **Mutation/CI posture:** logical-line swallow scanning (`ORD-HARD-128`);
   non-empty-baseline governance re-arm assertion (`ORD-HARD-134`).
6. **Evidence honesty:** the 0022 §1 correction or 0023-artifact supersession
   (`ORD-HARD-133`); the 0023 acceptance artifact runs the §7-checklist parity guard
   against itself.

## 6. Documentation corrections (housekeeping, same package)

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: extend R-29 with the two shapes this
  pass surfaced — the *literal witness count* (an authored number standing in for a
  measured one: `ORD-HARD-122`) and *binding laundering* (a forbidden decision routed
  through an intermediate `let` so no single line matches the scan: `ORD-HARD-127`).
  Add a Watch note under R-28: a membership derivation scoped to a naming convention
  is hand-maintained for everything outside the convention (`ORD-HARD-123`).
- Conformance index (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`):
  add/update rows for the embodied-locality epistemic migration (`ORD-HARD-121`), the
  meta-lock witness/census/ratchet repairs (`ORD-HARD-122`/`123`/`129`), the policy
  surface-driven behavioral lock (`ORD-HARD-126`), and the debug-overlay production
  wiring (`ORD-HARD-125`).
- No doctrine amendment; INV-001…INV-110 are applied, not changed. The INV-087
  decision (`ORD-HARD-095`) remains deferred; nothing in this spec pre-decides it.
  `ORD-HARD-121`'s identity-slot seam is forward-compatibility for INV-029, not an
  INV-029 implementation.

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0023_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. The embodied-locality migration: the observation-event emitters, the
   projection-derived locality fields, the compile-time truth-access removal, the
   INV-093 absence negative and staleness positive, and any honest golden/fixture
   repricing diff (`ORD-HARD-121`).
2. The meta-lock repairs with their firing negatives: live witness counts (with the
   anchor-miss synthetic), closed census membership (with the unprefixed-lock
   synthetic), chained baseline deltas (with the missing-predecessor synthetic),
   deferral content witnesses (`ORD-HARD-122`/`123`/`129`/`124`).
3. The debug overlay's production wiring and the derived token negative
   (`ORD-HARD-125`/`135`).
4. The surface-driven policy behavioral test output across all kinds and both
   surfaces with the row-mutation negative (`ORD-HARD-126`).
5. The scan-evasion closures with firing synthetics: laundering shapes,
   receiver/alias shapes, multi-line swallow, `.unwrap` coverage, envelope-ban
   widening (`ORD-HARD-127`/`130`/`128`/`132`/`136`).
6. The in-`context()` discrimination witness (`ORD-HARD-131`).
7. The canonical-intent coherence assertion and the positive embodied sleep proof
   (`ORD-HARD-137`/`138`).
8. The `cause_required` exhaustive match (`ORD-HARD-139`) and the
   non-empty-baseline governance re-arm assertion (`ORD-HARD-134`).
9. The 0022-report §1 correction or explicit supersession (`ORD-HARD-133`), and the
   §7-checklist parity guard run against this artifact itself.
10. Confirmation that every finding's premise still held at implementation time
    (all nineteen findings were operator-verified pre-implementation — twelve at
    audit, seven re-verified at spec reassessment); a premise that nonetheless fails
    is recorded as refuted, not silently dropped.
11. The risk-register and conformance-index diffs, quoted (§6).
12. An updated `EMERGE-OBS` ledger derivation over the corrected surface
    (measurement only, no thresholds).
13. The first scheduled mutation run's result under the post-0022 posture if it has
    fired by then (closing the 0022 "pending" status), or its still-pending status
    restated honestly.
14. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`;
    not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment.
- Crate direction preserved: core depends on nothing at runtime; content on core;
  tui on core + content. No new dependencies, dev or production.
- Verification posture: all nineteen findings are operator-verified at their
  load-bearing symbols — `ORD-HARD-121`–`129`, `132`, `133`, `138` during the audit,
  and `ORD-HARD-130`, `131`, `134`, `135`, `136`, `137`, `139` re-verified at source
  during spec reassessment (same baseline `db4b53a`; zero refuted) — so no
  re-verification step is owed; a finding whose premise nonetheless fails at
  implementation time is recorded as refuted in the acceptance artifact, not
  silently dropped.
- `ORD-HARD-121` ordering note: land the observation emitters and projection
  consumption *before* removing truth access from `EmbodiedProjectionSource`, so the
  compile-time lock lands against an already-correct builder (no intermediate broken
  state); re-derive golden logs once, honestly, in the same ticket group.
- Recommended ticket ordering:
  1. `ORD-HARD-122` + `123` + `129` + `124` (meta-lock integrity — first, so every
     subsequent ticket's new locks are born under repaired meta-rules) + `133`
     (evidence correction).
  2. `ORD-HARD-121` (embodied-locality epistemic migration; the foundation
     violation; largest diff; batch golden repricing once).
  3. `ORD-HARD-125` + `135` (debug overlay wiring + derived tokens).
  4. `ORD-HARD-126` (policy surface-driven behavioral lock).
  5. `ORD-HARD-127` + `130` (scan laundering/call-shape closure).
  6. `ORD-HARD-128` + `134` (mutation-CI swallow + governance re-arm).
  7. `ORD-HARD-131` + `132` (witness enforcement + `.unwrap` coverage).
  8. `ORD-HARD-137` + `138` (canonical-day coherence + sleep positive).
  9. `ORD-HARD-136` + `139` (envelope ban + exhaustive cause match).
  Documentation corrections (§6) land with group 1. The acceptance artifact lands
  last, measuring the corrected surface.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **`ORD-HARD-121`'s migration is the largest product diff since 0005.** Routing
  four field families through observation events touches perception emission, the
  epistemic projection, the embodied builder, semantic-action derivation, golden
  fixtures, and every TUI test that renders locality. The staged ordering note in §8
  bounds the risk; budget the ticket group accordingly. If implementation reveals the
  migration genuinely needs a Phase-3B-scale redesign (e.g. observation cadence or
  memory-decay questions doctrine has not yet answered), the honest fallback is a
  recorded owner decision narrowing this pass to the compile-time quarantine of the
  truth read plus the INV-093 negatives, with the full migration ticketed against an
  execution-doc entry — not a silent re-scope.
- **`ORD-HARD-122`/`123` repairs may surface latent vacuous scans.** Once witness
  counts are live and the census complement is closed, scans that have been matching
  zero sites will fail honestly — treat each as a potential masked defect first, per
  the Enforcement reading.
- **Observation-event volume.** Per-tick locality observations for items/doors/actors
  could bloat the log. The lawful pattern (`current_place_perception_events`) already
  exists for food/sleep/exits — reuse its cadence/dedup discipline (emit on place
  entry and on change, not per tick) and measure in the EMERGE-OBS derivation.
- **Next-iteration assessment (the recurring question):** a foundation violation was
  found, so a thirteenth pass is warranted by the lineage's own rule, and it should
  be a **strictly scoped verification of 0023's deliverables** — above all whether
  the `ORD-HARD-121` migration genuinely closed the truth read (the compile-time
  lock is the proof) and whether the repaired meta-locks fire with live witnesses.
  The honest trend, stated plainly: the cognition firewall is clean six passes
  running; the 099 recurrence — the lineage's worst integrity defect — closed with
  the work product as evidence; 0022's fixes held at sixteen of nineteen audited
  surfaces with all residues in the test-oracle/CI/evidence layers; and the one
  product finding came from a *blind* sweep outside the lineage's habitual scopes,
  not from re-auditing known surfaces. Per the external research (defect-discovery
  curves, event-triggered audit practice, the Tricorder graduation criterion), the
  defensible cadence after a clean-or-low-only thirteenth pass is: drop per-pass
  audits, move to phase-entry audits (Phase 3B / Phase 4 boundaries), keep the four
  gates + mutation CI + repaired meta-locks as the continuous layer, and schedule
  guard-consolidation hygiene at the same boundaries. The thirteenth pass should
  also include one more blind sweep of a surface the lineage has never scoped (the
  content loader end-to-end, or the TUI input/command layer) — `ORD-HARD-121`
  demonstrates that the residual risk now lives between audit boundaries, not inside
  them.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive
  rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (or is explicitly
  lock-durability / evidence-honesty) and names responsible layers from the execution
  diagnostic vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted); every finding was operator-verified at its load-bearing
  symbols (the audit pass plus the reassessment re-verification of the seven
  initially agent-reported findings; zero refuted).
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression; `ORD-HARD-121` and `139` carry compile-time impossibility locks.
- [x] Verified-holding items from 0014–0022 are recorded to prevent re-litigation;
  the 0005 feature-contract verification is recorded; severity calibrations that
  diverge from lineage precedent or from the reporting audit slice are named in §4's
  preamble (`ORD-HARD-121`/`122`/`123`/`124`/`127`).
- [x] No doctrine amendment; no compatibility shims; no new dependencies; crate
  direction preserved. The INV-087 decision remains deferred, untouched.
- [x] Scope stays within the Phase 3A ordinary-life surface, its consumed Phase 1
  locality boundary, and its lock/evidence layer.
