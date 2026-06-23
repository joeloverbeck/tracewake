# 0048 Foundational Conformance Hardening: Loaded-World Tick, Temporal Authority, Holder-Known Intervals, and Replay-Frontier Reconstruction Hardening Spec

**Status**: COMPLETED

This is a staged hardening spec in the parallel `specs/NNNN` series. It is staged in
`specs/` and is promoted to `archive/specs/` on acceptance; it is never promoted to the
live `docs/4-specs/` tier, and it does not amend constitutional invariants, define gate
semantics, or weaken execution gates. It uses the canonical hardening-spec house
structure of its sibling specs (e.g. `0046`/`0025`), not the `docs/NN_*` narrative-document
style. The default canonical section set is not used; this sibling-derived structure is.

## 0. Baseline statement and source discipline

- **Driver.** `reports/0047-foundational-hardening-research-report.md`, a recommendation-altitude
  deep-research conformance audit of the archived `0047` surface (TUI authoritative world-advance,
  duration completion, actor-known interval summaries). The report is the originating analysis; it
  is not itself doctrine and minted no spec, invariant, risk identifier, or gate code.
- **Report target commit.** The report was conducted against
  `cb3102ef1993a88b2516cec77a145e97a2baae85` (`cb3102e`), which is the current repository `HEAD` at
  authoring time. Every load-bearing code claim cited below was independently re-verified against
  that working tree (see §3). The named symbols are authoritative; the report relies on named
  symbols, not line numbers.
- **Static-survey limitation inherited from the driver.** The report forbade execution, so its
  pass/fail statements are static readings of source/test intent, not command results. This spec
  therefore states code *structure* facts as verified and defers all green/red command claims to the
  implementing session (§7).
- **Source discipline.** A commit hash named in this spec is audit/provenance only. This spec
  recommends and scopes work; it does not declare latest-`main` certification or any phase entry.
  When executed, the implementation must name its own exact implementation commit, not assume this
  baseline commit is latest `main`.
- **Ledger timing.** Per the sibling-spec convention, this package receives its
  `docs/4-specs/SPEC_LEDGER.md` archived-row entry at acceptance/closeout, not at proposal. This spec
  authors no ledger row now and makes no change to live `0001` or the ledger.
- **Archived-history discipline.** The archived `0047` spec, its acceptance artifact, the `0046`
  artifacts, and any passed certification are immutable, commit-pinned historical records. This spec
  does not edit them and does not treat their claims as automatic current-state proof.

## 1. Scope

### 1.1 In scope

The seven divergences and warranted hardening gaps the driver found at `cb3102e`, all on the `0047`
seams and their named conformance collaborators:

1. The canonical one-tick coordinator (`DeterministicScheduler::advance_world_one_tick`) is a
   duration/accounting tick, not a loaded-world step: it runs zero ordinary actor transactions and
   zero world processes.
2. The TUI commits a possessed-actor action through `run_pipeline` *before* asking the coordinator
   to advance the world, so the combined one-tick transition is not atomic / fail-closed.
3. Temporal-frontier authority is publicly writable (`pub current_tick`) and can advance from
   arbitrary appended-event timestamps (`sync_scheduler_frontier_to_appended_events`, direct TUI
   assignment).
4. The actor-known salient-observation stop branch in `advance_until` is not production-reachable —
   no perception/epistemic projection occurs inside the step.
5. The actor-known interval summary is built by raw-log redaction through forgeable public
   `String`/ID fields, not as a sealed holder-known delta.
6. Replay does not derive or validate the temporal frontier from `TimeAdvanced` (it is a
   `WorldNoOp`); the rebuild report carries no temporal frontier and the empty-tick test supplies the
   final tick externally.
7. The existing human/no-human differential and parity evidence overclaim: they prove
   duration/accounting parity only, with empty autonomous-proposal lists and zero-event expectations.

The corresponding anti-regression layer: stronger differential, temporal-replay corruption suite,
compile-time boundary fixtures, derived reservation census, generated mixed schedules, measured
parity outputs, and focused post-remediation mutation campaigns.

### 1.2 Out of scope (non-goals)

- Any higher-tier doctrine change (foundation/architecture/execution/reference). The driver's
  determination — confirmed in §5 — is that the live docs already state the correct contract; the
  code is below doctrine. Changing doctrine to fit incorrect code is prohibited.
- Minting any new invariant, risk ID, gate, glossary term, or ratified paste-ready wording.
- Adding a property-testing crate (`proptest`/`quickcheck`); see §6.4 and §9.
- Expanding the `.cargo/mutants.toml` perimeter — the named `0047` files are already included; the
  delta is behavior that kills the relevant mutants.
- Re-commissioning the four `0047` properties the driver verified as currently correct (§3.1);
  these are preserved, not redesigned.
- Editing archived/certified artifacts (§0).

## 2. Doctrine anchors

Authority order applied (foundation → architecture → execution → reference). Load-bearing anchors:

- **Foundation `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`** — waiting runs the simulation: one
  possessed actor supplies one input slot while other loaded actors, world processes, and due
  consequences advance in one shared transition; embodied summaries are actor-filtered;
  stop-on-salient-perceived-interruption is a staged embodied control.
- **Foundation `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`** — every accepted step, including an
  empty one, carries ancestry sufficient to rebuild the temporal frontier.
- **Architecture `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`** — empty-tick frontier
  rebuild; stop decisions replay from typed step evidence.
- **Architecture `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`** — actor-known interval
  summaries are positively constructed holder-known frontier deltas, never a raw world-event diff
  with forbidden rows redacted.
- **Architecture `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`** — deterministic
  due-work ordering, ordinary proposal routing for human and autonomous actors, and due consequences
  belong to one core world-step coordinator.
- **Architecture `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`** — the TUI asks core
  to perform a typed world step and never applies events, mutates state, owns duration terminals, or
  maintains a local clock; actor-known summary and debug step report are separate products.
- **Execution `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`** — the full
  phase-ordered one-tick core transaction; ordinary actor/process transactions and due world
  processes are explicit phases.
- **Execution `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`** — rejects a human/no-human
  proof that exercises only a private possessed-actor tick; defines the held-equal proof.
- **Execution `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`** — embodied controls use the
  ordinary TUI/core boundary and must not reuse debug no-human gameplay/report paths.
- **Execution `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`** — typed-before-rendered
  evidence, real path-under-test witnesses, live negatives, explicit limits; no declaration text or
  artifact presence as proof.
- **Reference `01_DESIGN_RISK_REGISTER.md`** — existing R-27 (reachability overstatement) and R-29
  (decorative locks) govern the evidence-honesty findings; closeout updates evidence/status of
  existing risk rows and mints no new risk ID.

Implicated invariants (all verified present in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`):
`INV-001`, `INV-005`, `INV-006`, `INV-009`, `INV-010`, `INV-018`, `INV-067`, `INV-091`, `INV-092`,
`INV-094`, `INV-099`, `INV-101`, `INV-102`, `INV-103`, `INV-104`, `INV-108`, `INV-112`. See §10.

## 3. Determination

**Divergences found; foundational remediation and substantial anti-regression hardening are
warranted.** Six foundational violations (five critical, one high) and one high evidence-overclaim
hardening gap exist on the `0047` surface at `cb3102e`. All seven were independently re-verified in
the working tree at the baseline commit (see each §4 item).

### 3.1 Verified holdings (preserve; do not re-litigate)

The driver found these `0047` properties currently correct, with meaningful tests and mutation scope.
They are preserved, integrated into the rebuilt step, and must not regress:

- **Log-derived open durations.** Starts/terminals are scanned from `EventLog`
  (`discover_due_duration_candidates`); duplicate/orphan evidence fails closed; lifecycle resolution
  uses typed builders. *Delta:* add mid-duration save/rebuild/resume coverage; ensure no cached queue
  becomes authoritative.
- **Cause-bearing, world-stream, replay-significant `TimeAdvanced` envelope.** Marker carries
  `prior_tick`/`resulting_tick`, `EventStream::World`, one cause, no actor; emission/cause
  classification are correct.
- **Centralized need accounting** around counted `(actor, need, tick)` classifications.
- **General body-exclusive reservation predicate** (based on an existing open body-exclusive
  duration, not sleep/wait-specific).

### 3.2 Validated — no action

- **Mutation perimeter.** `.cargo/mutants.toml` already includes `scheduler.rs`, `need_accounting.rs`,
  `projections.rs`, `actions/pipeline.rs`, `events/**`, `replay/**`, `view_models.rs`,
  `tracewake-tui/src/app.rs`, and `render.rs`. No perimeter expansion required (verified present).
- **`anti_regression_guards.rs`** already performs `include_str!`-based topology/source checks across
  the named surfaces. The repository already has external-crate negative fixtures for
  unforgeable/private boundaries; this spec extends that pattern rather than adding a second
  compile-fail framework.
- **`generative_lock.rs`** already runs a deterministic recorded-seed corpus with replay/tamper
  properties, reachability/omitted-population disclosure, and a duplicate need-charge-key check. This
  spec extends that harness; no new property-testing crate (§6.4).
- **Higher-tier doctrine.** No correction warranted — the live foundation/architecture/execution/
  reference tiers already state the required behavior at the right altitude (§5).

## 4. Findings and remediation requirements

Each item names the verified current state, the conformance verdict, the required remediation and
its core home, and the strongest practical guard. The TUI must implement none of these phases.

### 4.1 The canonical one-tick coordinator is not a loaded-world step — **violation (critical)**

- **Verified state (`cb3102e`).** `DeterministicScheduler::advance_world_one_tick` validates the
  expected tick, discovers due body-exclusive durations, appends `TimeAdvanced` and duration/
  need-accounting events, updates `current_tick`, and returns `WorldStepDueWorkSummary` with
  `actor_transactions_attempted: 0` and `world_processes_applied: 0` *unconditionally*. Its physical
  input is `&PhysicalState` (immutable). It invokes no actor decision transaction and no world-process
  registry/cadence. `advance_until` loops this same reduced operation.
- **Verdict.** Foundational violation of `INV-005`, `INV-006`, `INV-091`, `INV-094`, `INV-108`,
  `INV-112` (and `INV-001`: a control labeled "world advance" does not run the modeled causes the
  doctrine assigns to that transition). Contradicts foundation `08`, architecture `04`, execution
  `05`/`06`.
- **Required remediation + home.** Core owns a single typed one-tick transaction receiving all inputs
  due at that tick: the optional controlled-actor input/proposal slot; deterministic autonomous-actor
  opportunities; deterministic world-process work; open-duration lifecycle work; one shared
  need-accounting reconciliation; the perception/epistemic projection needed for typed stop evidence
  (§4.4); and the temporal marker plus all causal ancestry. Implementation may stage against
  cloned/scratch state or use an explicit prepare/commit batch, but must expose one commit boundary.
  The scheduler may select cadence and invoke actor decision transactions under `INV-103`; it must not
  synthesize cognition from raw truth. Existing proposal validation and event application remain the
  owning seams. Homes: `crates/tracewake-core/src/scheduler.rs`, the existing agent
  transaction/decision APIs, `actions/pipeline.rs`, and the existing process-registry/cadence
  abstraction. The result fields must report measured nonzero actor/process counts.
- **Strongest guard.** A deterministic held-equal human/no-human differential in
  `crates/tracewake-core/tests/world_step_coordinator.rs` with all of these simultaneously live: a
  possessed actor whose input is a one-tick wait; ≥1 unpossessed actor with a due ordinary decision
  opportunity that commits a typed event; an open sleep/work duration due to terminate; passive need
  accounting for multiple actors; a due world process that commits a typed event; and replay from the
  same initial state/log. Compare the typed event multiset/order (allowing only declared
  controller-origin differences), final physical/agent state, temporal frontier, projection frontiers,
  and checksums; assert measured nonzero counts. An empty autonomous-proposal list or a zero
  expected-ordinary-event count is **not** an acceptable witness (see §4.7). Extend the deterministic
  generated-sequence harness with world-step schedules varying actor opportunities, duration
  boundaries, and process due times (recorded seeds + reachability). Run focused `cargo-mutants` over
  the rewritten coordinator, cadence invocation, and counters.

### 4.2 TUI wait and authoritative advancement are two commits, not one transaction — **violation (critical)**

- **Verified state.** `TuiApp::submit_entry_with_world_advance` builds the possessed-actor proposal,
  creates a mutable `PipelineContext`, and calls `run_pipeline` (which may append events and mutate
  physical/agent/epistemic state). Only after an accepted result does it call `advance_world_one_tick`
  (via `?`/`map_err`). A subsequent coordinator error returns without rolling back the already-accepted
  action; the wait and its need effects survive without the authoritative tick that was to contain
  them. Ordering is inverted relative to execution `05`: wait pipeline events can be appended before
  the tick-boundary marker.
- **Verdict.** Foundational violation of `INV-001`, `INV-009`, `INV-010`, `INV-018`, `INV-092`,
  `INV-103`, `INV-104`, `INV-112`.
- **Required remediation + home.** For world-advancing controls, the TUI submits one typed request to
  core carrying the controlled-actor input slot and expected frontier. Core validates and prepares the
  complete step before committing any event or state mutation; on any failure (validation, duration,
  accounting, process, append, application, projection, replay-precondition) the pre-step physical
  state, agent state, epistemic projection, log, and temporal frontier remain unchanged. The ordinary
  proposal still travels the existing proposal/validation/pipeline route, but invoked *inside* the
  core-owned one-tick transaction rather than committed by TUI choreography. Continuation/advance-until
  controls remain controller operations and must not be disguised as a second `wait` action. Homes:
  the core world-step request/result API and `TuiApp::submit_entry_with_world_advance` (which collapses
  to request construction + result handling). No compatibility wrapper preserving the two-step
  choreography remains.
- **Strongest guard.** An all-or-nothing failure test: valid possessed wait input; seeded
  malformed/contradictory due-duration evidence that makes the core step reject after it has inspected
  the controlled input; snapshot physical/agent/epistemic/log/frontier/checksums before; assert every
  snapshot byte/structurally identical after. Plus a success-order witness asserting exactly one
  `TimeAdvanced` boundary per increment with valid ordering ancestry for every same-step resulting-tick
  effect. Mutation target: removing the prepare/commit barrier or committing the controlled proposal
  early. A source guard banning the old `run_pipeline`-then-`advance_world_one_tick` choreography is a
  secondary topology alarm; the transactional failure test is the primary evidence. Checking only the
  returned error is insufficient — every mutable authority surface must be compared.

### 4.3 The temporal frontier has multiple writers and can advance without tick ancestry — **violation (critical)**

- **Verified state.** `DeterministicScheduler` exposes `pub current_tick`. Writers beyond the intended
  authority: `TuiApp::submit_entry_with_world_advance` assigns `self.scheduler.current_tick =
  last_event.sim_tick` in the non-advance branch; `TuiApp::run_no_human_day` assigns
  `report.final_tick`; `sync_scheduler_frontier_to_appended_events` sets the frontier to the max
  `sim_tick` among arbitrary newly appended events. The no-human runner executes scheduled proposals
  separately from the coordinator then calls the sync helper — an appended wait at the next tick can
  move the frontier with no `TimeAdvanced` for that increment.
- **Verdict.** Foundational violation of `INV-009`, `INV-010`, `INV-018`, `INV-092`, `INV-112`. Event
  timestamps are being used as commands to move the frontier; TUI/debug paths assign it directly.
- **Required remediation + home.** Make the scheduler frontier private with a read-only accessor;
  all existing `current_tick` read sites (≈14 in `tracewake-tui`, e.g. `expected_tick =
  self.scheduler.current_tick`, plus the internal scheduler reads) migrate to that accessor, and only
  the canonical one-tick commit and the typed restoration constructor below may write it.
  Runtime advancement occurs only through the canonical one-tick commit. Replay/load restoration may
  set an initial frontier only through a distinct typed restoration constructor that verifies temporal
  projection evidence — it must not share the ordinary advancement setter. Delete
  `sync_scheduler_frontier_to_appended_events` as an advancement mechanism; scheduled no-human proposals
  become inputs to the same one-tick transaction (§4.1) so their timestamps cannot move time
  independently. Delete TUI frontier assignments; debug no-human completion returns evidence, not a
  setter. Homes: `crates/tracewake-core/src/scheduler.rs`, `scheduler::no_human`,
  `crates/tracewake-tui/src/app.rs`, the replay restoration boundary.
- **Strongest guard.** Extend the external-crate negative-fixture pattern with a fixture proving
  downstream code cannot assign the frontier or construct a runtime frontier-restoration token
  (compile-time). Runtime temporal-chain properties: every increment has exactly one `TimeAdvanced`;
  each marker payload has `prior_tick == reconstructed_frontier` and `resulting_tick == prior_tick + 1`;
  envelope tick, payload ticks, cause model, and stream ordering agree; no ordinary event timestamp
  alone changes the reconstructed frontier; a no-human scheduled wait cannot skip the marker;
  acceleration of `N` ticks yields `N` chained markers. Mutate marker omission, duplicate markers, `+1`
  arithmetic, payload swaps, and bypass of the canonical setter. A privacy source-assertion alone is
  insufficient: the marker-chain test must prove the remaining legal API cannot create a markerless
  increment.

### 4.4 The actor-known salient-observation stop branch is not production-reachable — **violation (high)**

- **Verified state.** `advance_until` calls `step_appended_actor_known_salient_observation` after each
  `advance_world_one_tick`; the helper returns true only when an appended ID resolves to
  `ObservationRecorded` for the possessed actor. But `advance_world_one_tick` appends only temporal,
  duration-terminal, and accounting events — it never runs perception/epistemic projection.
  `TuiApp::advance_until` builds the interval summary immediately after the loop returns, then calls
  `record_current_place_perception_and_project` — too late to stop the loop or appear in the just-built
  summary. The scheduler unit test inserts an `ObservationRecorded` directly and proves only the
  predicate's actor-ID comparison, not the production route.
- **Verdict.** Foundational violation of `INV-067`, `INV-099`, `INV-101`, `INV-102`, `INV-112`. The
  public stop reason exists but cannot be reached through modeled perception.
- **Required remediation + home.** Perception/epistemic projection needed for a time-control stop
  decision occurs inside the canonical step's typed phase contract — after authoritative effects,
  before stop evaluation. The step result carries a typed holder-known delta or salience decision
  derived from the sealed actor-known context; `advance_until` consumes that typed evidence and must
  not scan the global log for an `ObservationRecorded` row inferring actor knowledge from `actor_id`
  alone. The salience policy stays deterministic and actor-known; exact hidden world events, raw due
  queues, or debug comparisons cannot stop embodied progression unless a modeled channel produces an
  allowed holder-known delta. Homes: `scheduler.rs`, the perception/epistemic projection boundary,
  `TuiApp::advance_until`.
- **Strongest guard.** A real fixture/path-under-test pair: a positive world where at tick `N` a
  modeled source produces an observation available to the possessed actor — `advance_until` stops
  exactly at `N`, returns the typed actor-known stop reason, and the summary cites the source; a
  hidden-world variant where the same event occurs for another actor or without an acquisition channel
  — the possessed actor's stop tick and summary are unchanged; and replay reproducing both the temporal
  frontier and the stop evidence. Mutate the actor/context match, source membership, and salience
  predicate; positive and hidden variants must kill opposite classes of leakage/omission mutants.
  Calling the private helper with a fabricated event is a unit negative, not acceptance evidence.

### 4.5 The interval summary violates the positive-construction boundary — **violation (critical)**

- **Verified state.** `ActorKnownIntervalSource` has public `actor_id`, `source_event_id`, and arbitrary
  `summary: String`. `build_actor_known_interval_summary` accepts any caller-provided stop-reason
  (`impl Into<String>`) and any iterable of sources, filters only on `source.actor_id ==
  viewer_actor_id`, moves the arbitrary text into the notice, stringifies the event ID, and labels an
  empty list as "no new actor-known information". It accepts no `KnowledgeContext`, projection frontier,
  allowed source set, or provenance witness. `TuiApp::actor_known_interval_sources` scans `EventLog` for
  the step's appended IDs, reads raw events, maps selected event kinds to prose
  (`actor_known_interval_summary_for_event`), and supplies the public source objects — the raw-global-
  diff/redaction architecture the controlling doc forbids — and runs before resumption perception is
  projected.
- **Verdict.** Foundational violation of `INV-067`, `INV-099`, `INV-101`, `INV-102`, `INV-112`. A raw
  event plus matching actor ID is not a sealed holder-known delta, and a `String` is not provenance.
- **Required remediation + home.** Move interval-delta construction into the core epistemic/projection
  boundary. The builder consumes a sealed before/after holder-known frontier (or a typed delta produced
  by the epistemic projection). Each output item derives from a closed fact/notice kind and retains a
  typed source reference whose holder visibility and fact-kind compatibility were verified. Required API
  properties: external crates and the TUI cannot construct interval-source records directly; viewer
  identity is bound by the sealed context, not a caller field; notice meaning and stop reason are closed
  enums/typed values, not arbitrary prose; rendering to strings happens only at the TUI boundary; "no new
  actor-known information" means the verified holder-known delta is empty (not a filtered-empty raw
  diff); resumption perception is part of the interval projection before the final summary; the exact
  debug step report stays a separate type with no conversion into the embodied summary. Homes:
  `crates/tracewake-core/src/epistemics/projection.rs`, `crates/tracewake-core/src/projections.rs`,
  `view_models.rs`, `TuiApp::advance_until`. Delete the TUI raw-log source builder rather than keep it as
  a compatibility path.
- **Strongest guard.** Three complementary guards: (1) compile-time — extend external-crate negative
  fixtures proving callers cannot construct source/notice internals, supply arbitrary prose, or convert
  a debug step report into the embodied interval type; (2) paired-world observational equivalence — two
  worlds with identical possessed-actor holder-known inputs and different hidden events must produce
  identical typed interval summaries and rendered embodied output; adding one modeled source-bearing
  observation to only one holder-known context changes exactly the permitted notice; (3) provenance
  closure — every emitted notice resolves through the sealed context/projection (source existence,
  holder visibility, fact-kind compatibility, interval-frontier membership); dangling, wrong-kind,
  other-holder, and debug-only sources fail closed. The paired-world test is the strongest truth-firewall
  witness; both directions (invariance under hidden state + verified source per output) are necessary.

### 4.6 Replay does not reconstruct the temporal frontier — **violation (critical)**

- **Verified state.** `ProjectionRebuildReport` carries physical/agent/epistemic/checksum/event-count/
  stream-position outputs but **no** reconstructed temporal frontier or temporal-divergence report.
  `rebuild_projection` receives a `ChecksumContext` whose `sim_tick` is caller-supplied. In
  `events/apply.rs`, `TimeAdvanced` is `ApplyOutcome::WorldNoOp` and no temporal projection consumes its
  payload; `rebuild.rs` references `TimeAdvanced` nowhere. The test
  `empty_world_step_appends_time_advanced_and_rebuilds_frontier` calls `rebuild_projection` with
  `checksum_context(SimTick::new(42), 1)` — it does not derive tick 42 from the event or compare a
  reconstructed frontier to the live frontier. The test name and archived acceptance claim are stronger
  than the executable assertion.
- **Verdict.** Foundational violation of `INV-018`, `INV-092`, `INV-112`. `TimeAdvanced` is emitted but
  not authoritative during replay; the current frontier is an out-of-band caller input.
- **Required remediation + home.** Introduce a replay-owned temporal projection starting from an explicit
  initial frontier and consuming ordered `TimeAdvanced` events. Per marker it validates: supported
  schema/kind; required cause model and ordering ancestry; payload `prior_tick == current frontier`;
  payload `resulting_tick == prior_tick + 1`; envelope `sim_tick == resulting_tick`; no
  duplicate/gap/backward/multi-tick jump; marker ordering relative to step effects follows the canonical
  transaction contract. `ProjectionRebuildReport` and `ReplayReport` expose the reconstructed final
  frontier and typed temporal violations. The rebuild API must not accept the expected final tick as an
  unverified checksum fact: split static checksum-identity inputs from the final temporal context, derive
  the latter from replay, then compute final checksums. Homes:
  `crates/tracewake-core/src/replay/rebuild.rs`, `replay/report.rs`, the event-application/temporal
  projection boundary.
- **Strongest guard.** Replay tests for: a single otherwise-empty tick (supplied only an initial
  frontier); multiple chained ticks and advance-until; mid-duration save/rebuild/resume; missing marker;
  duplicate marker; prior/result mismatch; `+2` jump; backward marker; envelope/payload disagreement;
  missing/wrong cause ancestry; ordinary future-timestamp event without a marker. Each malformed case
  fails with a typed temporal divergence, not merely a different checksum. Add mutations around marker
  application and chain arithmetic. Include the temporal frontier in the §4.1/§4.7 differential. A test
  that passes the desired final tick into the rebuild context cannot prove rebuild of that tick — the
  expected value is withheld from the system under test and used only in the assertion.

### 4.7 Existing differential and parity evidence does not lock the claimed properties — **hardening-gap (high)**

- **Verified state.** `differential_human_wait_and_no_human_wait_match_authoritative_outcome` compares
  duration completion, needs, physical state/checksum, and controller/process origin — but the no-human
  side receives `Vec::new()` scheduled proposals, the test asserts `ordinary_pipeline_events == 0`, and
  no due world process is present. It proves duration/accounting parity for an unpossessed sleeping actor
  only; it cannot detect omission of autonomous actor transactions or world processes. The `0046`/`0047`
  parity runner executes real fixture/TUI operations and renderer checks, but `ScenarioWitnesses::
  ordered_witnesses` echoes declared assertion strings and `CoverageRow::typed_witness` is true when the
  declaration text is non-empty — inventory metadata, not measured typed evidence. Real scenario
  assertions do not establish temporal-frontier reconstruction or holder-known provenance closure.
- **Verdict.** Warranted hardening gap and current evidence overclaim (R-27/R-29). The tests prove
  narrower properties than their labels suggest; this requires precise scope and stronger witnesses, not
  invalidating the useful duration/accounting/rendering coverage. Implicates weakly-protected `INV-091`,
  `INV-094`, `INV-108`.
- **Required remediation + home.** Replace declaration-as-proof fields for load-bearing `0047` properties
  with measured scenario outputs returned by the real path: reconstructed temporal frontier + marker
  count; nonzero autonomous actor/process work where required; exact duration terminal and one-charge
  classifications; sealed context/frontier identifiers; source IDs resolving through allowed provenance;
  typed stop reason; replay match including temporal projection; explicit debug/embodied type
  disposition. The §4.1 differential becomes the authoritative human/no-human witness; parity scenarios
  reference that artifact or reproduce its measured checks and must not convert an assertion sentence into
  a boolean "typed witness". Homes: `crates/tracewake-core/tests/world_step_coordinator.rs`, the TUI
  parity scenario/runner and live evidence mappings.
- **Strongest guard.** Adversarial variants that deliberately: omit autonomous actor invocation; omit
  world-process invocation; pass a hidden event with the same actor ID but no holder-known source; supply
  a desired final tick externally while removing a marker; leave a witness description populated while
  returning empty typed evidence. Each must fail the actual conformance runner. The live report must
  distinguish "duration/accounting differential", "loaded-world actor/process differential", "temporal
  replay", and "holder-known noninterference" — one passing test must not be cited as all four unless it
  measures all four.

## 5. Doctrine amendments

**None.** The driver's determination, independently confirmed here, is that no higher-tier doctrine
correction is warranted: foundation `08` says waiting advances the loaded world; architecture `04`
assigns actor routing and due processes to the canonical step; architecture `02` requires empty-tick
frontier rebuild; architecture `03` forbids raw-diff/redaction interval summaries; architecture `10`
separates TUI request, actor-known summary, and debug report authority; execution `05` lists the full
phase contract; execution `06` defines the held-equal proof; execution `07` already states the corrected
embodied-controls staging posture; execution `10` already rejects artifact/declaration-as-evidence. The
implementation is below doctrine; changing foundation text would weaken a correct contract to fit
incorrect code. This spec mints no invariant, risk ID, gate, glossary term, or ratified wording.

## 6. Required fixtures and tests

Consolidated from §4 guards plus the cross-cutting anti-regression layer (driver §5).

### 6.1 Behavior witnesses (primary)

- The held-equal human/no-human loaded-world differential (§4.1) with measured nonzero actor/process
  counts, full state/frontier/projection/checksum comparison, and replay.
- The all-or-nothing TUI world-step failure-atomicity test and success-order single-marker witness (§4.2).
- The runtime temporal-marker-chain property set (§4.3) and the replay temporal-projection corruption
  suite (§4.6).
- The positive/hidden/replay salient-stop fixture pair (§4.4).
- The paired-world observational-equivalence test and provenance-closure test for interval summaries
  (§4.5).
- Measured parity scenario outputs and adversarial parity variants (§4.7).
- A derived action-registry reservation census: an exhaustive test across every ordinary body-using
  action for human and autonomous origins while a sleep/work duration is open, allowing only typed
  continuation/lifecycle controls (preserves §3.1 reservation holding without hand-maintained cases).
- Mid-duration save/rebuild/resume coverage (preserves §3.1 log-derived-duration holding).

### 6.2 Compile-time boundaries (extend existing external-crate negative fixtures)

- Scheduler frontier private outside the core temporal authority; no downstream frontier assignment or
  runtime restoration-token construction.
- Interval-source/notice constructors private to the sealed projection module; no arbitrary prose; closed
  typed notice and stop-reason values; no debug-step-report → embodied-summary conversion.
- One core world-step entry point for world-advancing TUI controls.

Retain narrow `include_str!` source guards only as secondary topology alarms (e.g. banning direct TUI
frontier assignment or reintroduction of the deleted raw-log interval builder). A source scan is never
cited as proof of atomicity, replay, or noninterference.

### 6.3 Mutation campaigns (post-remediation, existing perimeter)

No `.cargo/mutants.toml` change. After remediation, run focused campaigns over: canonical step phase
invocation/ordering; prepare/commit atomicity; temporal chain arithmetic/validation; private-frontier
restoration path; holder-known source membership and fact-kind compatibility; salience selection;
one-charge reconciliation under mixed regimes; reservation checks across the derived census. Surviving
mutants are signals that a witness is missing or the mutant is equivalent; classify under the repo's
existing mutation evidence process. No text-only mutation expectations.

### 6.4 Generative / property-testing decision

**Do not add `proptest` or `quickcheck` in this pass.** Extend the existing deterministic
generated-sequence harness (`generative_lock.rs` / `tests/support/generative.rs`) with world-step
schedule generation (varying actor opportunities, duration boundaries, process due times; recorded seeds,
reachability, omitted-population disclosure). The useful properties are state-machine relations — one
marker per increment; one charge per `(actor, need, tick)`; live/replay equivalence including temporal
frontier; hidden-world changes do not alter actor-known output; duration legality across save/resume;
full-step human/no-human equivalence — not broad random value generation. Add a simple domain-specific
shrink/minimization routine only when a failing seed appears; reconsider `proptest` only if manual
shrinking becomes a demonstrated maintenance cost (deliberate rejection-for-now, not a value claim).

## 7. Acceptance artifact and evidence

The implementing session must:

- Run the ordinary workspace gates and retain exact transcripts: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.
- Run replay/golden lanes, the focused new tests of §6, and the configured mutation campaigns of §6.3;
  retain exact transcripts or deterministic reports. No claim in this spec is premised on an assumed
  green or red command; every pass/fail must be a recorded command result.
- Produce a new implementation/review artifact recording the static-survey findings and the executed
  remediation evidence (per execution `10` typed-before-rendered and anti-vacuity rules). Do not rewrite
  archived acceptance history.
- Update live conformance/evidence homes (not doctrine): point the conformance rows in
  `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` at the new executable temporal/frontier,
  actor/process differential, and sealed-provenance witnesses, and expand the time-control posture to
  cover canonical actor/process phases, temporal replay, and sealed interval projection; ensure
  `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` prompts are answered with live evidence
  (no doctrinal rewrite); update the evidence/status of existing risks R-08, R-09, R-10, R-11, R-13,
  R-15, R-16, R-27, R-28, R-29 in `docs/3-reference/01_DESIGN_RISK_REGISTER.md` as appropriate (mint no
  risk ID); accurately scope the old differential as duration/accounting evidence until replaced;
  distinguish registry declaration metadata from measured typed evidence.
- Add the `docs/4-specs/SPEC_LEDGER.md` archived-row entry for this package at acceptance/closeout, and
  move this spec to `archive/specs/` per the hardening-series convention.

## 8. Implementation constraints

- **Recommended closure order** (avoids writing tests around an API that must be replaced; ensures later
  evidence exercises one final authority path):
  1. Make temporal authority singular — private frontier; replay temporal projection; delete
     event-timestamp and TUI assignment paths.
  2. Create the atomic full-world step — controlled input, autonomous opportunities, due processes,
     durations, accounting, perception/projection, one prepare/commit boundary.
  3. Refactor human and no-human callers — both become inputs/loops over the same one-tick transaction;
     eliminate sync shortcuts.
  4. Replace interval-summary authority — sealed holder-known delta, typed notices/stop reason, resumption
     perception, no TUI global-log scan.
  5. Make salient stopping real — consume the typed holder-known step delta; positive/hidden/replay
     fixtures.
  6. Strengthen evidence — non-vacuous differential, temporal-replay corruption suite, compile-fail
     boundaries, derived reservation census, generated mixed schedules, measured parity outputs.
  7. Execute and package — gates, replay/goldens, focused + configured mutation campaigns, deterministic
     evidence report, existing-risk/conformance-reference updates.
- `tracewake-core` keeps zero dependencies; the one-way crate dependency direction
  (core ← content ← tui) is preserved. The TUI implements none of the world-step phases.
- No backwards-compatibility shims or alias paths: delete the two-step TUI choreography and the raw-log
  interval builder rather than retaining them as compatibility paths.
- Source scans remain secondary topology alarms, never substitutes for behavior witnesses.

## 9. Risks and open questions

These are implementation choices inside settled doctrine, not reasons to defer the verdict:

1. **Atomic staging strategy** — clone/scratch aggregate state and log, or a prepared event/application
   batch with explicit commit. Either is acceptable only if the all-or-nothing test (§4.2) proves every
   authority surface unchanged on failure.
2. **Actor/process cadence source** — which existing scheduler/transaction structures enumerate due
   autonomous opportunities and world processes. The coordinator must invoke the owning abstractions
   without becoming cognition authority (`INV-103`).
3. **Typed salience policy** — which holder-known fact kinds stop acceleration vs. remain summary-only.
   Must be deterministic, source-bearing, and replayable. (Severity note: §4.4 is rated *high* rather than
   *critical* despite sharing the truth-firewall invariants of the *critical* §4.5, because the defect is
   an unreachable-but-inert stop branch rather than an active permissive authority path; this follows the
   driver's own calibration.)
4. **Temporal restoration input** — how an initial frontier is represented in a save/snapshot without
   allowing ordinary runtime callers to set it. A verified restoration constructor or snapshot-temporal
   projection is preferable to a general setter.
5. **Rendered temporal labels** — whether exact interval ticks are themselves actor-known in each control
   context. Debug may always show exact replay time; embodied rendering must follow holder-known temporal
   provenance.
6. **Static-survey carryover** — the driver executed no commands; if the implementing session's gate run
   surfaces a current-state fact that contradicts a §4 structural claim, correct this spec first (per the
   repo ticket/spec-divergence convention), then implement.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-005` / `INV-006` / `INV-108` | aligns | The rebuilt one-tick step (§4.1) gives unpossessed actors their ordinary due opportunity through the same seam; possession changes only input/viewpoint, granting no extra phases @ scheduler/world-step. |
| `INV-091` / `INV-094` | aligns | The non-vacuous held-equal human/no-human differential and measured parity outputs (§4.1/§4.7) restore real no-human and possession-parity proof @ core tests/parity runner. |
| `INV-001` / `INV-009` / `INV-010` | aligns | The atomic prepare/commit world step and single-writer marker chain (§4.2/§4.3) ensure a "world advance" runs its modeled causes and every meaningful change is one cause-bearing committed event @ world kernel/event log. |
| `INV-018` / `INV-092` / `INV-112` | aligns | Private frontier + replay temporal projection reconstructing the frontier from `TimeAdvanced` (§4.3/§4.6) make event/replay time the sole authoritative ordering, not a caller input @ scheduler/replay. |
| `INV-067` / `INV-099` / `INV-101` / `INV-102` | aligns | Sealed holder-known interval delta with closed notice/stop kinds and provenance closure, and in-step perception for salient stops (§4.4/§4.5), replace raw-log redaction with positive holder-known construction @ epistemic projection/view models/TUI. |
| `INV-103` / `INV-104` | aligns | The scheduler selects cadence and invokes the decision transaction but synthesizes no cognition; needs/routines still route through candidate generation, validation, and commitment @ scheduler/action pipeline. |

No invariant is tensioned; this spec enforces existing invariants against below-doctrine code and amends
none (§5).

## Outcome

Completed: 2026-06-23

Accepted as scoped foundational conformance hardening for loaded-world tick
authority, temporal frontier reconstruction, holder-known interval summaries,
and measured parity evidence. Implementation baseline commits were
`31889e8`, `3b76142`, `7610ed5`, `50d26d4`, `28e9fd2`, `dbd6ee6`, and
`3964f24`. Evidence/report commit `f46fe32` added
`reports/0048_foundational_conformance_hardening_acceptance.md`, archived
`0048FOUCONHAR-008`, updated the architecture conformance row, and updated the
existing risk rows R-08, R-09, R-10, R-11, R-13, R-15, R-16, R-27, R-28, and
R-29 without minting a risk id.

All tickets `0048FOUCONHAR-001` through `0048FOUCONHAR-008` are archived under
`archive/tickets/`. The acceptance report records the full pre-report
workspace gates, focused mutation campaign result (61 tested, 40 caught, 8
missed, 13 unviable), survivor triage, and the gate-to-witness map. The eight
surviving focused mutants remain recorded as accepted capstone limitations and
future witness gaps where applicable; this closeout does not claim a
mutation-perfect result or a full configured mutation campaign.

Final archive/truthing is owned by the commit that moves this spec to
`archive/specs/`, adds the `docs/4-specs/SPEC_LEDGER.md` archived row, and
runs the required post-closeout gates. This spec mints no invariant, doctrine
change, gate, glossary term, or risk id.
