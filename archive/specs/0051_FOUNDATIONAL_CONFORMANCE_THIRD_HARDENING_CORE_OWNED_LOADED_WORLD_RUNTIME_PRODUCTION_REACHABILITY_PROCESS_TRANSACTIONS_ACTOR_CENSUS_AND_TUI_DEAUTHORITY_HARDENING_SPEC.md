# 0051 Foundational Conformance Third Hardening: Core-Owned Loaded-World Runtime, Production Reachability, Declared-Process Transactions, Per-Tick Actor Census, and TUI De-Authority Hardening Spec

**Status**: COMPLETED

This is a staged hardening spec in the parallel `specs/NNNN` series. It is staged in
`specs/` and is promoted to `archive/specs/` on acceptance; it is never promoted to the
live `docs/4-specs/` tier, and it does not amend constitutional invariants, define gate
semantics, or weaken execution gates. It uses the canonical hardening-spec house structure
of its sibling specs (e.g. `0050`/`0048`/`0046`/`0025`), not the `docs/NN_*`
narrative-document style. The default canonical `specs/` section set is not used; this
sibling-derived structure is.

It is the **third-pass** foundational-conformance hardening of the `0047` surface (TUI
authoritative world-advance, duration completion, actor-known interval summaries). The
first pass was `0048`; the second pass was `0050` (closing residual violations the
second-pass re-audit found surviving `0048`/`0049`). This pass closes the residual
violations the **third-pass** re-audit found surviving `0050`/`0050FOUCONSEC-014..016`.

## 0. Baseline statement and source discipline

- **Driver.** `reports/0047-foundational-hardening-research-report-third-pass.md`, a
  recommendation-altitude deep-research cross-cutting foundational-conformance and
  anti-regression delta re-audit of the `0047` surface as hardened by `0048`, `0050`, the
  `0049MUTWIT` mutation-witness line, and the post-acceptance `0050FOUCONSEC-014/-015/-016`
  focused tickets. The report is the originating analysis; it is not itself doctrine and
  minted no spec, invariant, risk identifier, gate code, or glossary term. Its predecessors
  are `reports/0047-foundational-hardening-research-report-second-pass.md` (drove `0050`) and
  `reports/0047-foundational-hardening-research-report.md` (drove `0048`), used by the driver
  only as pre-remediation baselines.
- **Report target commit.** The report was conducted against
  `0429a8f7e7fed2329319b79818f9b891da91cba2` (`0429a8f`), which is the current repository
  `HEAD` at authoring time. Every load-bearing code claim cited below was independently
  re-verified against that working tree (see §3). The named symbols are authoritative; the
  report relies on named symbols, not line numbers (the report's line numbers are not relied
  upon and several were observed off against the live tree).
- **Static-survey limitation inherited from the driver.** The report forbade execution, so
  its pass/fail statements are static readings of source/test intent, not command results.
  This spec therefore states code *structure* facts as verified and defers all green/red
  command claims to the implementing session (§7).
- **Source discipline.** A commit hash named in this spec is audit/provenance only. This spec
  recommends and scopes work; it does not declare latest-`main` certification or any phase
  entry. When executed, the implementation must name its own exact implementation commit, not
  assume this baseline commit is latest `main`.
- **Ledger timing.** Per the sibling-spec convention, this package receives its
  `docs/4-specs/SPEC_LEDGER.md` archived-row entry at acceptance/closeout, not at proposal.
  This spec authors no ledger row now and makes no change to live `0001` or the ledger.
- **Archived-history discipline.** The archived `0047`/`0048`/`0050` specs, their acceptance
  artifacts, the `0049MUTWIT-001..003` and `0050FOUCONSEC-001..016` tickets, and any passed
  certification are immutable, commit-pinned historical records. This spec does not edit them
  and does not treat their claims as automatic current-state proof. In particular, the `0050`
  acceptance artifact's F-01..F-08 closure table and its honest non-green mutation statement
  are immutable; the correct response is this new remediation line plus a future acceptance
  artifact, never retrospective alteration.

## 1. Scope

### 1.1 In scope

The nine divergences and warranted gaps the driver found at `0429a8f`, on the
`0047`/`0048`/`0050` seams and their named conformance collaborators:

1. **F-01 — Production loaded-world discovery is absent; the replacement reachability witness
   manufactures eligibility (critical; vacuity gap).** `DeterministicScheduler::new`
   initializes `loaded_actor_next_decision_tick` and `declared_world_processes` as empty maps;
   the only writers are the public `schedule_loaded_actor_decision` and
   `register_cadenced_world_process`. The production TUI bootstrap
   (`TuiApp::from_golden`/launch path) constructs `DeterministicScheduler::new(SimTick::ZERO)`
   and never derives or registers actor/process eligibility from loaded world state or
   content, while the "production reachability" differential witness in
   `world_step_coordinator.rs` itself calls the seed methods before invoking the coordinator.
2. **F-02 — Due-work authority is ephemeral, caller-authored, and lost on replay restore
   (critical).** `restore_from_temporal_projection` and `restore_from_rebuild_report` each
   return `Self::new(reconstructed_final_frontier)`, reconstructing only the clock and
   discarding loaded-actor eligibility, process declarations, cadence, trigger provenance, and
   proposal-sequence state. The scheduler never consumes/advances
   `loaded_actor_next_decision_tick` after running an actor transaction, so an actor remains
   perpetually due unless an outside caller rewrites the map.
3. **F-03 — Declared-process "application" is a diagnostic no-op, not a process-specific
   causal transition (critical; evidence overclaim).** `build_declared_world_process_event`
   turns a `DueProcessInvocation` into a generic `NoHumanAdvanceStarted` diagnostic-stream
   event; `apply_event_stream` maps `Diagnostic`/`Controller`/`ReplayDebug` to
   `ApplyOutcome::NonWorldNoOp`. `transact_world_one_tick` nonetheless increments
   `world_processes_applied` as though a process transition occurred.
4. **F-04 — Controlled and autonomous actor opportunities are not mutually exclusive within a
   tick (critical; structural gap).** `transact_world_one_tick` processes controlled proposals
   first, then runs `ActorDecisionTransaction` for every due actor with
   `controller_bindings: None`, with no exclusion for an actor already represented in
   `controlled_proposals`, no controller-binding check in the due census, and no closed
   per-tick actor disposition table. Today's collision is avoided only because F-01 leaves the
   registry empty; closing F-01 makes the double-opportunity reachable.
5. **F-05 — The actor decision transaction is still only partially consumed (high).**
   `ActorDecisionProposalOutcome` carries `proposal`, full `DecisionTrace`,
   `DecisionTraceRecord`, `lifecycle_effects`, and `LocalPlan`, and `SelectedGoalBundle`
   computes a `local_plan_id` and a proposal-ancestry vector. The scheduler's `Proposed` branch
   consumes only the reduced record and lifecycle effects; it never consumes
   `decision_trace` or `local_plan`, and the sealed proposal payload omits `local_plan_id`
   and the complete ancestry vector, so replay/debug cannot rebuild the full
   "why this concrete action" chain required by INV-041.
6. **F-06 — The TUI remains an authoritative writer; its perception compile-fail witness is
   vacuous (critical; vacuity gap).** `TuiApp` owns the authoritative action registry,
   physical state, agent state, `EventLog`, `ControllerBindings`, `DeterministicScheduler`, and
   `EpistemicProjection`, and passes mutable references directly into core mutation functions
   (bind-time perception append, direct `run_pipeline` for non-wait actions, post-action
   perception append, `advance_until` aggregate hand-off, and a separate `run_no_human_day`
   rebuild/replace path). The advertised negative fixture calls
   `tracewake_tui::record_current_place_perception_and_project()` — a no-argument symbol that
   does not exist — while the real writer is the public core
   `record_current_place_perception_and_project` (in `agent/perception.rs`) plus a public
   scheduler forwarding writer, neither of which the fixture exercises.
7. **F-07 — Embodied output leaks exact replay/control metadata, and its public products are
   forgeable (high; hardening gap).** The default embodied renderer prints
   `Actor: <id> | Tick: <exact SimTick>`, exact interval start/stop ticks, the internal
   stop-reason `stable_id`, and each notice's raw source `EventId`; a renderer test locks
   actor-facing `controller_safety_bound` output. Every field of `EmbodiedViewModel` and
   `TypedActorKnownIntervalSummary` is public (including `sim_tick`, frontiers, and
   `stop_reason`), and `IntervalStopReason`'s variants are public, so external code can forge an
   apparently embodied summary by a direct struct literal — empty `notices` with arbitrary
   start/stop ticks and frontiers and `stop_reason: ControllerSafetyBound` — without the verified
   holder-known constructor. (The public `From<ActorKnownIntervalDelta>` is a secondary concern:
   its input `ActorKnownIntervalDelta` already has private fields and a crate-private
   `from_verified` constructor, so that conversion is not itself an external forging vector; its
   defect is that it performs the internal→embodied conversion on the TUI side rather than in
   core.)
8. **F-08 — The standing mutation floor still contains in-scope foundational survivor families
   (high; mutation-evidence gap).** The standing campaign recorded at the `-013` acceptance
   (3,182 selected / 2,458 caught / 675 unviable / 48 missed / 1 timeout) is honestly
   non-green; `-014/-015/-016` add credible focused witnesses but no fresh standing run exists.
   In-scope-for-this-surface survivors that must close: **#10** (`restore_from_temporal_projection`
   → `None`, guards F-02), **#15–17** (`EpistemicProjection::insert_observation` retain
   condition), and **#47–48** (`embodied_subject_key` → constant). Provisionally-addressed
   families (#8–9 `-014`, #11–14+timeout `-015`, #18–46 `-016`) require focused-command reruns
   plus a standing rerun. Out-of-surface family **#1–7** (`food_source_fact_supersedes`) routes
   to a future cross-cutting mutation-remediation spec, not this surface and not a lower tier.
9. **F-09 — Live conformance and risk evidence overstate production closure (high;
   evidence-honesty gap).** The architecture conformance index (`00`) and execution `05`/`07`
   describe core-owned loaded-actor/process discovery, closed actor-outcome consumption, and
   TUI read-only interval consumption as production-proven; the live code contradicts those
   reachability/authority claims. The `SPEC_LEDGER.md` historical source record (0050 archived
   row, 48-missed/1-timeout limit, 0049MUTWIT ticket-only source) is **already accurate** and
   is not in scope to change.

### 1.2 Out of scope

- Any edit to archived specs, tickets, acceptance artifacts, or passed certifications.
- Any constitutional-invariant amendment, gate-semantics change, or new risk-ID mint (see §5).
- The out-of-surface mutation family **#1–7** (`food_source_fact_supersedes`); it routes to a
  future cross-cutting mutation-remediation spec via owner/reassess, with no equivalence claim.
- Any new property-testing dependency (`proptest`/`quickcheck`); the existing deterministic
  corpus, generated lock corpus, integration seams, negative fixtures, and mutation campaigns
  are sufficient (driver §3.3, §6.2).
- Phase-4 entry, second-proof entry, latest-`main` certification, or feature expansion.
- Re-litigating properties the third pass found structurally present (duplicate-`EventId`
  fail-closed rejection, temporal-divergence replay conjunct, scratch-state atomic cutover,
  positive typed holder-known interval construction); they are preserved, not re-built (§4.10).

## 2. Doctrine anchors

Authority order applied without inversion: `0-foundation` → `1-architecture` → `2-execution`
→ `3-reference`. Controlling invariants by finding (re-verified against the live constitution):

- **Causality / event sourcing / replay.** INV-001, INV-009, INV-010, INV-011, INV-018,
  INV-019, INV-020.
- **No-human authority and possession parity.** INV-004, INV-005, INV-006, INV-007, INV-087,
  INV-091, INV-094, INV-108.
- **Declared regional/world processes.** INV-088.
- **Cognition authority and the truth firewall.** INV-041, INV-099, INV-100, INV-101,
  INV-102, INV-103, INV-104, INV-105, INV-106.
- **TUI/embodied/debug boundary.** INV-065, INV-067, INV-068, INV-069, INV-093.
- **Temporal authority.** INV-112.
- **Harsh acceptance and validation.** INV-043, INV-092, INV-098.

Architecture homes: `00` (index/conformance), `01`, `02` (event log/replay/save/restore),
`04` (scheduler/world-step), `10` (TUI boundary), `13`. Execution homes: `05` (transaction
scheduler / action pipeline / no direct dispatch), `06`, `07` (epistemic view models /
possession / debug proof), `10` (evidence discipline). Reference homes: `00` (review
checklist), `01` (risks R-08/R-09/R-10/R-11/R-13/R-15/R-27/R-28/R-29).

## 3. Determination

**Not conformant; remediation warranted; no higher-tier doctrinal amendment warranted.**

The `0050` line delivered real structural gains (scratch-state atomic cutover; duplicate-ID
fail-closed; temporal-divergence replay conjunct; scheduler-owned eligibility maps; typed
positive interval-notice construction). Those gains are recorded as present (§4.10) and not
re-reported as defects. They do not establish foundational conformance: the decisive
authority boundaries remain open exactly as enumerated in §1.1.

The root cause is structural, not a shortage of individual tests: **the scheduler is treated
as a callable utility rather than the owner of a loaded simulation, and the TUI repeatedly
reacquires write authority.** Each prior remediation patched one route while the caller
retained the ability to rebuild or reseed authority, and witnesses proved coordinator
*composition with manufactured inputs* and then labeled it *production reachability*. The
cycle-breaking change is a single **core-owned loaded-world runtime/session boundary** (§4.1),
not another wrapper around `DeterministicScheduler`.

Independent re-verification at `0429a8f` confirmed every load-bearing code claim: empty
registry initialization and public seed methods (F-01); clock-only
`restore_from_temporal_projection`/`restore_from_rebuild_report` (F-02); diagnostic-stream
`NonWorldNoOp` mapping with `world_processes_applied++` (F-03); controlled-then-due ordering
with no exclusion (F-04); the unconsumed `decision_trace`/`local_plan` products (F-05); TUI
aggregate ownership plus the non-existent-symbol negative fixture (F-06); all-public,
struct-literal-forgeable view models and the exact-`Tick:`/`source_event_id`/`controller_safety_bound`
renderer (F-07). F-08 is a measured-evidence gap whose disposition defers to the implementing
session's clean standing run; F-09 is a documentation-honesty gap remediated only **after**
code closure.

## 4. Findings and remediation requirements

Each finding below names a **code home**, a **remediation requirement**, and the
**strongest practical anti-regression guard** (compile-time/type ownership first, then
production-path behavior, then focused mutation). Source-text/`include_str!` guards are
permitted only as labeled topology alarms and never as sole proof of atomicity, replay
continuation, process effects, one-opportunity-per-actor, holder-known noninterference,
production reachability, or mutation sensitivity (driver §6.4).

Subsections are keyed by finding ID, not by implementation order. §8 (closure order) governs
sequencing: in particular, F-03 declared-process transactions (§4.5) are implemented **before**
the F-04 per-tick census (§4.4).

### 4.1 Structural keystone — one core-owned loaded-world runtime/session

Introduce a single core-owned runtime/session type (`SimulationSession` /
`LoadedWorldRuntime` or equivalent, in `crates/tracewake-core`) that owns, with **private
fields**: canonical physical state, agent state, event log and temporal/replay frontier,
epistemic projection, loaded-actor census and next-opportunity projection, declared-process
registry/cadence/trigger state, controller bindings/possession mapping, duration/reservation
state, and deterministic ID/order minting. Its public surface is a small **closed set of typed
commands** (controlled proposal / input slot, one-tick wait, continue-duration /
advance-until, pause policy, debug-authorized no-human execution, save/rebuild) returning
**immutable, sealed typed receipts**. Internally one transaction derives a closed
actor/process disposition census (§4.4), executes selected work on scratch aggregates,
produces full actor/process ancestry (§4.5), constructs separate debug and embodied products
(§4.7), and commits once.

This keystone is the home for F-01, F-02, F-04, F-05, and F-06 remediations; the
finding-specific subsections below state the obligations that ride on it. No
backwards-compatibility public alias may be left behind; a temporary internal adapter may
exist only long enough to migrate core tests.

### 4.2 F-01 — Production loaded-world discovery

**Code home:** the §4.1 runtime adjacent to `scheduler.rs`, fixture/content loading, replay
rebuild, and controller binding. **Requirement:** derive the initial actor census and declared
process registry from authoritative loaded content/state during the same bootstrap the TUI
uses (`TuiApp::from_golden`), and maintain eligibility as part of the core runtime; TUI
construction receives the already-initialized owner rather than a blank scheduler. Remove or
restrict `schedule_loaded_actor_decision` and `register_cadenced_world_process` so external
crates/clients cannot seed actor eligibility or process declarations directly; deterministic
test setup uses a core-crate test-only builder or a real loaded package, not a public seed
method. **Guards:** (1) external-crate negative fixtures fail with a privacy/constructor
diagnostic when attempting to seed eligibility on the real symbols; (2) a two-actor + one
declared-process fixture loaded through the production bootstrap, one actor bound, one wait
submitted, asserting the unpossessed actor transaction and the process effect appear with
**no test call** to scheduler registration; (3) same package with/without a controller binding
compared; (4) mutation kills empty actor discovery, empty process discovery, boundary
inversions, and omission of loader-to-runtime registration.

### 4.3 F-02 — Replay/save reconstruction of runtime authority

**Code home:** §4.1 runtime + `replay/temporal.rs`, `replay/rebuild.rs`, restore path.
**Requirement:** make loaded-actor eligibility and process declarations reconstructable on
replay through one consistently-chosen design — event-sourced runtime declarations/updates
projected during rebuild, deterministic derivation from canonical loaded state/content + event
history, or a dedicated replay-derived runtime projection included in save/restore and
checksums. Reconstruction must include process cadence/trigger provenance and actor
next-decision state; proposal ordering must be reconstructed or replaced by ordering derived
from event frontier + stable identities. After an actor opportunity, core must deterministically
record or derive the actor's **next** opportunity with no outside rescheduling.
`restore_from_temporal_projection` must not claim complete scheduler restoration if its input
cannot reconstruct all runtime authority — either accept the complete runtime projection or
narrow/rename it and prevent production use as a full restore. **Guards:** core-owned runtime
with private due-work state and no external registration; continuation-equivalence (load ≥2
actors + a cadenced process, run N ticks, serialize/rebuild, run K more, compare physical/agent
state, event-log suffix, epistemic projection, temporal frontier, actor-opportunity sequence,
process invocations, and returned summaries); a cadence-boundary case proving an actor is not
perpetually due after one opportunity; mutation kills restore-to-`None`, dropped-registry
restoration, off-by-one next-decision updates, and proposal-order reconstruction changes
(closes survivor #10).

### 4.4 F-04 — Closed exhaustive per-tick actor disposition census

**Code home:** §4.1 runtime / `scheduler.rs` world-step. **Requirement:** before executing any
proposal, build one closed, exhaustive per-tick actor census assigning **exactly one**
disposition to each loaded actor (e.g. controlled attempt for the bound actor, autonomous
decision opportunity, body-exclusive duration continuation/reservation, deferred/not-due,
stuck/blocked). A valid controlled input **consumes** that tick's ordinary actor opportunity;
the possessed actor is not special in rules. Duration reservation takes precedence consistently
for human and autonomous actors. Census enum and constructors remain private to core; external
callers submit commands, not dispositions. **Guards:** a closed internal
`ActorDisposition`/`ActorOpportunity` enum requiring exhaustive handling before commit; a single
tick with a controlled possessed actor + a second unpossessed due actor + a third under
body-exclusive duration, asserting exactly one disposition per actor, ≤1 ordinary proposal per
actor, and no controller-specific validation; permutation/differential reordering proving
canonical output and possession parity; mutation kills possessed-actor exclusion,
duplicate-disposition checks, duration precedence, and one-opportunity cardinality.

### 4.5 F-03 — Declared-process causal transactions

**Code home:** `scheduler.rs`/§4.1 runtime, `events/envelope.rs`, `events/apply.rs`.
**Requirement:** define a closed process declaration/dispatch registry inside core; a due
invocation identifies a process kind + trigger witness; the process transaction builds an
ordinary typed proposal or a closed process-transition command; validation produces
process-specific world/agent/epistemic events appended and applied through the **same atomic
cutover** as actor work. Diagnostic marker events may accompany the transition but cannot be
the transition. The declaration loads from canonical content/runtime state and is reconstructed
on replay per §4.3. `world_processes_applied` counts committed process transactions, not
diagnostic envelopes. **Guards:** private `DueProcessInvocation` + closed process-kind dispatch
that cannot accept a caller-supplied completed `EventEnvelope`; a declared process whose due
transition changes a small authoritative state/projection fact, asserting no effect before
cadence, one effect at cadence, correct ancestry, and identical replay; negative fixture
proving external crates cannot construct due invocations or inject raw process events; mutation
kills process-builder bypass, diagnostic-only substitution, wrong process/source ID, cadence
inversions, and increment-without-commit.

### 4.6 F-05 — Full actor decision-transaction consumption

**Code home:** `agent/transaction.rs`, `agent/trace.rs`, §4.1 runtime/`scheduler.rs`.
**Requirement:** replace the loose public-field outcome with a closed, commit-ready internal
actor-step transition whose private fields include the proposal, full trace, trace
record/render projection, local-plan identity and the plan steps diagnostics require,
lifecycle effects, and explicit ancestry links. The scheduler must exhaustively consume that
transition and return a typed committed summary linking actor-known context → candidate
selection → selected method → local plan → proposal → validation/pipeline result → ordinary
event(s) → lifecycle/trace/stuck artifacts. Persist typed IDs/fields sufficient to rebuild the
chain (including `local_plan_id` and the ancestry vector currently dropped); a too-large full
plan may be persisted as a canonical plan record/projection with stable identity and event
ancestry — never as opaque debug prose. **Guards:** closed non-reducible outcome (external
callers cannot construct it; internal handling is exhaustive); through `transact_world_one_tick`
produce a planned action, a `Stuck`, and a rejected proposal, asserting typed ancestry survives
into the log and returned result; replay rebuild compares canonical trace/plan identities and
links; mutation kills omission of each artifact class, missing local-plan ancestry, trace
reduction, `Stuck` suppression, and ignored pipeline status. A unit test calling
`ActorDecisionTransaction::run` proves capability, not commitment — closure inspects the log and
core result after the real coordinator path.

### 4.7 F-06 — TUI de-authority; F-07 — sealed split temporal products

**F-06 code home:** `crates/tracewake-tui/src/app.rs`, `agent/perception.rs`, the negative
fixture and runner. **Requirement:** the §4.1 runtime owns all authoritative mutable
aggregates; the TUI owns only client/session handle, presentation state, command-parsing
state, and immutable typed results. Every world-affecting semantic action — including non-wait
actions — goes through typed session commands (bind/unbind, submit semantic action, one-tick
wait, duration continuation/advance-until, debug-authorized no-human execution). Bind-time
perception, if retained, occurs inside core as a modeled channel. Remove public
event-appending perception helpers and public scheduler forwarding methods from external
reach; the no-human path becomes a command on the same runtime, not a TUI rebuild/replace
choreography. Replace the vacuous fixture with fixtures that name the **real** former symbols
with realistic arguments and fail for privacy/constructor reasons, plus fixtures proving the
TUI cannot construct `PipelineContext` over authoritative aggregates, append perception,
replace replay state, or reach mutable runtime fields; add a positive in-crate test proving
the single core owner can still perform the operation. Expose no `&mut PhysicalState`,
`&mut AgentState`, `&mut EventLog`, or `&mut EpistemicProjection` across the TUI/core public
boundary.

**F-07 code home:** `view_models.rs`, `crates/tracewake-tui/src/render.rs`. **Requirement:**
split the current product into at least two sealed types — an internal/debug
`WorldAdvanceReceipt` (exact ticks/frontiers, controller stop reason, appended event IDs,
diagnostics) and an actor-facing `EmbodiedIntervalSummary` (only positively constructed
actor-legible notices and source-supported temporal language). `UserPausedBeforeNextTick` is
client UI state; `ControllerSafetyBound` belongs to the operator/debug receipt. Exact ticks/clock
labels appear in embodied mode only when a modeled actor-known clock/calendar/record supports
them; raw event IDs remain internal provenance handles, not diegetic content. Make fields
private and constructors crate-private; expose read-only getters per surface; implement no
debug-to-embodied conversion; the TUI receives the already-separated product from core.
**Guards:** negative fixtures proving external crates cannot construct embodied summaries,
mutate source/frontier fields, or convert debug receipts to embodied products; default embodied
transcripts assert absence of exact replay tick/frontier, raw `EventId`, `controller_safety_bound`,
and pause logistics, with debug-overlay tests asserting presence under explicit debug
capability; information-flow differential feeding identical holder-known contexts with different
hidden ticks/debug receipts and proving embodied output is unchanged unless an actor-known
temporal source differs; mutation kills source omission, debug/embodied conversion, stop-reason
routing, and actor-known temporal-source checks (closes survivors #15–17 and #47–48 at the
production boundary, see §4.8).

### 4.8 F-08 — In-scope mutation families and standing campaign

**Code/test home:** extend existing scheduler/replay, epistemic projection
(`insert_observation`, `embodied_subject_key`), interval, salience, and generative tests; do
**not** edit `.cargo/mutants.toml` to restate existing coverage, and do **not** pad
`.cargo/mutants-baseline-misses.txt` for killable foundational mutants. **Requirement:** close
in-scope survivors **#10** (restore continuation, via §4.3), **#15–17** (`insert_observation`
retain condition — add production-path same-source/same-subject vs different-source/different-
subject witnesses + focused kills), and **#47–48** (`embodied_subject_key` → constant — add
direct and production-path subject-separation witnesses + focused kills). Reproduce the exact
preserved focused commands from `0049MUTWIT` and `0050FOUCONSEC-014/-015/-016` against a clean
`0429a8f`-descendant tree, then run the full standing campaign (`cargo mutants --timeout 183`
with the standing config) **after** all code/test work, publishing the selected denominator and
the complete caught/missed/unviable/timeout disposition. No survivor is classified equivalent
without a defensible semantic argument; out-of-surface survivors (#1–7) route to owner/reassess
and a future cross-cutting spec (not a lower tier). **Evidence honesty:** a focused 0-miss run
is evidence only for its selected functions/mutants and exact source tree and never substitutes
for the standing campaign; a timeout is not converted into an accepted baseline to obtain green.

### 4.9 F-09 — Truth the live conformance/risk evidence (after code closure)

**Documentation home (post-implementation only):** update architecture conformance `00` to name
the actual core runtime entry point and real production witness; align architecture `01/02/04/
05/10/13` and execution `05/06/07/10` status/evidence references with the repaired runtime,
restored eligibility, process transaction, actor census, and split products **without changing
governing doctrine**; update the reference checklist `00` live-evidence pointers; update only
the status/evidence under existing **R-27/R-28/R-29** (mint no new risk ID); add this remediation
spec to `SPEC_LEDGER.md` through the normal acceptance process, preserving all archived rows and
the already-correct 0049 source record. Every updated evidence row must answer: which production
constructor created the runtime, which public command crossed the client boundary, what
state/event/projection effect was observed, and which deliberate mutation or negative compile
attempt proves sensitivity. No archived spec/ticket/acceptance/certification is edited.

### 4.10 Preserved properties (regression-guard, not rebuild)

The following are present by static reading at `0429a8f` and must be **preserved** through the
§4.1 refactor, re-expressed under the core-owned session, and kept as required production-runtime
cases: one shared core world-tick transaction; atomic scratch-state cutover (extend failure
injection to each actor/process/perception phase asserting no partial commit, including
duplicate-ID failure driven through the world-step transaction); duration continuation and
log-derived completion (add restore-mid-duration continuation equivalence); single-charge need
accounting (add a mixed actor/process tick through the production runtime and a restore
boundary); body-exclusive reservation (integrate with the §4.4 census so it cannot be bypassed
by an alternate choreography); accepted empty-tick ancestry (keep as a production-runtime case +
deletion mutant); replay aggregate fail-closed on temporal divergence (preserve + a
physical-equal/temporal-different case); duplicate-`EventId` rejection (drive a collision
through the world-step transaction proving atomic rollback); positive holder-known interval
construction (seal constructors/fields; obtain products only from the core session).

## 5. Doctrine amendments

**None warranted at any tier.** The driver's higher-tier determination (§8.1) and independent
re-reading of the live constitution confirm doctrine already states every load-bearing
requirement this spec enforces: one event-sourced causal transition and deterministic replay
(INV-001/009/010/018); loaded-world actor/process progression rather than a private
possessed-actor tick (INV-004/087/088/091/094/108); ordinary proposal/validation parity
(INV-043); core ownership of due work and time, with the scheduler/replay clock barred as
cognition authority (INV-103/112); full cognition-transaction ancestry (INV-041/099/101/102/105);
positive holder-known construction and debug quarantine (INV-067/068/069/093/106/107); a
read-only client boundary (INV-069); and real-path, mutation-sensitive, non-vacuous evidence
(INV-092/098). The defects arise because code and evidence fall **below** those rules, not
because the rules are missing or contradictory. Weakening doctrine to bless caller-seeded
registries, diagnostic process markers, client-owned aggregates, or exact hidden time would be a
constitutional inversion and is explicitly rejected. This spec therefore mints no invariant,
gate code, risk ID, or glossary term, and authors no ratified doctrine wording.

## 6. Required fixtures and tests

Extend existing machinery (driver §3.3, §6); do **not** duplicate it or add a property-testing
dependency:

- **External-crate negative fixtures** (`tests/negative-fixtures/*`, `negative_fixture_runner.rs`):
  replace the vacuous perception fixture; add fixtures naming the **real** forbidden capabilities
  (perception append, scheduler forwarding, eligibility seeding, due-invocation/raw-process-event
  construction, `PipelineContext` over authoritative aggregates, embodied-summary construction,
  source/frontier mutation, debug-to-embodied conversion), each pinned to a privacy/constructor
  diagnostic rather than generic "cannot find function." Add positive in-crate tests proving the
  single core owner can still perform each operation. Several of these guards already exist from
  the `0050` line and must be **extended, not duplicated**:
  `external_crate_cannot_name_due_process_invocation`,
  `external_crate_cannot_set_world_step_process_events`,
  `external_crate_cannot_set_world_step_due_actor_ids`,
  `external_crate_cannot_reduce_actor_step_outcome_to_option`,
  `external_crate_cannot_convert_debug_report_to_interval_summary`, and
  `external_crate_cannot_assign_scheduler_frontier` pin real privacy/trait-bound diagnostics
  today, but they guard *privacy*, not the production-path correctness the findings target. Only
  `external_crate_cannot_call_tui_perception_append_helper` is genuinely vacuous
  (`expected_stderr: "cannot find function"`) and needs replacement.
- **Production-bootstrap differential** (`world_step_coordinator.rs` + a runtime/session test):
  start through the production content/runtime constructor with **no** scheduler-registration
  calls; prove loaded actors and declared processes advance; deliberately remove the production
  derivation and require the test to fail.
- **Continuation equivalence** (replay/temporal/rebuild tests): uninterrupted-vs-restored
  comparison over all runtime authority including mid-duration restore.
- **Per-tick census** behavior + permutation tests (§4.4); **declared-process effect** before/at/
  after cadence + replay (§4.5); **full actor-outcome** ancestry through the real coordinator,
  including `Stuck` and rejection (§4.6).
- **Embodied/debug separation**: default-transcript leak-absence assertions, debug-capability
  presence assertions, and a hidden-context/debug-receipt **information-flow differential** (§4.7).
- **Deterministic generated corpus** (`generative_lock.rs`): add a production-constructor mode
  covering zero/one/multiple actors, possessed due/not-due/reserved, process absent/before/at/
  multiple-due, action accepted/rejected/partial-phase-failure, duration active/completing/
  interrupted/restored-mid-duration, empty tick and event-ID collision, actor-known delta
  quiet/novel/hidden/same-source-replacement/different-subject-separation, and uninterrupted-vs-
  restored continuation — each entering through the same production constructor and typed command
  boundary as the TUI.
- **Mutation**: focused campaigns during implementation for fast feedback, then the configured
  standing campaign (§4.8).
- **CI**: wire a standing conformance lane starting from production loading and using only the
  public core runtime/TUI command boundary; the architecture row cites that executable lane plus
  the compile-fail fixtures that protect real symbols. `include_str!` import/method guards remain
  labeled topology alarms only.

## 7. Acceptance artifact and evidence

On implementation, the session must: begin from a clean baseline; name its own exact
implementation commit (not this proposal's baseline); run `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo build --workspace --all-targets --locked`, and `cargo test --workspace`; reproduce the
preserved focused mutation commands; add and run focused campaigns for restore continuation,
`insert_observation`, and `embodied_subject_key`; run the full standing `cargo mutants` campaign
after all code/test work; and publish the selected denominator with the complete caught/missed/
unviable/timeout disposition. Every executable claim is the implementing session's to produce —
this spec asserts no green/red command result. The acceptance artifact follows the
`0003_ACCEPTANCE_ARTIFACT_TEMPLATE`, records the per-finding closure with real production-path
evidence (production constructor, public command, observed effect, sensitivity proof), and must
not call the perimeter green before the standing campaign. Doc-truthing (§4.9) lands only after
the executable witnesses exist.

## 8. Implementation constraints

- Follow the driver's recommended **closure order** (§9): runtime/session first; then production
  bootstrap + replay restore; then process transactions; then the per-tick census; then full
  actor-outcome consumption; then TUI write-path removal; then product split; then witness
  replacement; then mutation closure; then doc-truthing. Writing tests around the present public
  ownership shape before replacing it would harden the wrong API.
- No backwards-compatibility shim or public alias path in new work; a temporary internal adapter
  may exist only to migrate core tests and must be removed before closeout.
- Core must not depend on content or tui; the runtime/session owner lives in `tracewake-core`.
- Worktree discipline: if implemented in a worktree, all paths resolve against the worktree root.
- Decompose into one ticket per reviewable diff (the closure-order steps are the natural ticket
  boundaries); honor the existing `0050FOUCONSEC` sibling prefix convention or its successor as
  the reassess/decomposition step determines.

## 9. Risks and open questions

These are implementation choices inside settled doctrine; none blocks the determination:

1. **Runtime owner name/module** — `SimulationSession`, `LoadedWorldRuntime`, or equivalent core
   type. Non-negotiable: it, not the TUI/caller, owns all authoritative mutable aggregates and
   due-work authority.
2. **Eligibility representation** — event-sourced declarations/updates, deterministic
   canonical-state derivation, or a dedicated replay projection. Non-negotiable: save/replay
   reconstructs identical future actor/process selection without caller repair.
3. **Process dispatch representation** — closed enum + registry, core/content-bootstrap-only
   trait objects, or generated closed table. Non-negotiable: callers cannot inject completed raw
   envelopes; a due process produces authoritative effects.
4. **Actor next-decision rule** — fixed cadence, planner-derived typed delay,
   action-completion-derived tick, or explicit event. Non-negotiable: exactly one deterministic
   rule is committed/derived and replay-restored.
5. **Actor disposition data shape** — closed enum per actor, indexed census table, or typestate
   builder. Non-negotiable: one and only one disposition per loaded actor per tick; controlled and
   autonomous are mutually exclusive.
6. **Public receipt ergonomics** — read-only getters, opaque iterators, or core-produced
   serialization DTOs. Non-negotiable: external code cannot forge trusted embodied/debug
   provenance or mutate internal frontier/source data.
7. **Actor-facing temporal language** — qualitative interval, modeled clock/calendar reading,
   remembered source, or explicit "time passed" notice. Non-negotiable: exact simulation/replay
   ticks appear only when an actor-known modeled source supports them; controller stop logistics
   remain debug/operator data.
8. **Save-artifact boundary** — event log + canonical content fingerprint + derived runtime
   projection, or another replay-complete representation. Non-negotiable: continuation equivalence
   covers actor/process cadence, ordering, durations, reservations, and projections, not only
   physical state and tick.
9. **Mutation-survivor ownership** — current remediation for surface-relevant families, future
   cross-cutting line for others. Non-negotiable: no silent drop, no unjustified equivalence, no
   lower-tier routing for this cross-cutting perimeter.

Open recurrence risk: this is the third consecutive pass to find critical divergence on this
seam after a remediation line claimed closure. The §4.1 keystone (type-level unrepresentability
of client write authority) is the structural bet that breaks the cycle; if the implementation
preserves any public mutable-aggregate path, the seam will reopen. `#[non_exhaustive]` alone is
insufficient — public enum variants remain externally constructible, so private fields/crate-
private constructors/unexported tokens are the controlling mechanism.

## 10. Invariants alignment

| Invariant(s) | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-004, INV-087, INV-088, INV-091, INV-108 | aligns | §4.2 derives loaded actors/processes from canonical content in the core runtime, so waiting runs the loaded world with no human and no caller-seeded eligibility. |
| INV-018, INV-019, INV-092, INV-112 | aligns | §4.3 reconstructs all runtime authority (eligibility, cadence, ordering, next-opportunity) on replay/restore, not just the clock frontier; continuation-equivalence tests prove it. |
| INV-001, INV-009, INV-010, INV-088 | aligns | §4.5 makes a due declared process produce process-specific validated events through the atomic cutover; `world_processes_applied` counts committed transactions, ending the diagnostic-marker no-op. |
| INV-004, INV-005, INV-043, INV-094, INV-108 | aligns | §4.4's closed per-tick census gives each loaded actor exactly one disposition; a controlled input consumes the ordinary opportunity, so possession changes input only. |
| INV-041, INV-099, INV-101, INV-102, INV-105 | aligns | §4.6 commits the full decision trace, local plan, ancestry, alternatives/blockers, and resulting events as typed authoritative diagnostic data. |
| INV-009, INV-067, INV-069, INV-093, INV-112 | aligns | §4.7 moves all authoritative mutation behind the core session and splits sealed debug vs. embodied products, so the TUI cannot write state and embodied output cannot leak exact replay/control metadata. |
| INV-018, INV-092, INV-093, INV-098 | aligns | §4.8 closes in-scope survivors and runs a fresh standing campaign; measured mutation evidence replaces stale/overclaimed closure. |
| (all above) | N/A — no amendment | §5: doctrine already requires every property; no invariant is weakened, minted, or redefined. |

## Outcome

Completed: 2026-06-24

Implemented through archived tickets `0051FOUCONTHI-001` through
`0051FOUCONTHI-012`. The line introduced the core-owned
`LoadedWorldRuntime`, restored loaded-world due-work authority from loaded
content and replay rebuilds, converted declared processes into committed
process-specific events, closed per-tick actor disposition and actor decision
lineage gaps, moved TUI mutation authority behind runtime commands, sealed
actor-facing temporal interval products, added production-path conformance and
mutation witnesses, truthed architecture/execution/reference evidence rows,
and produced the scoped acceptance artifact.

Acceptance evidence:

1. `archive/reports/0051_foundational_conformance_third_hardening_acceptance.md`
   names exact implementation commit
   `a3b46c6f3f3cbd4ea02ec685a42d2d61be7ce953`.
2. The clean-worktree gates passed:
   `cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, and
   `cargo test --workspace`.
3. Eight focused mutation campaigns completed with zero missed selected
   mutants.
4. The standing campaign completed and remains honestly non-green:
   `3275 mutants tested in 3h: 23 missed, 2549 caught, 703 unviable`,
   with `0` timeouts.

Deviations from the proposal:

1. The standing mutation perimeter is not called green; the remaining misses
   are recorded in the acceptance artifact and remain future remediation
   surface.
2. No doctrine, invariant, gate, glossary, or risk identifier was minted.
