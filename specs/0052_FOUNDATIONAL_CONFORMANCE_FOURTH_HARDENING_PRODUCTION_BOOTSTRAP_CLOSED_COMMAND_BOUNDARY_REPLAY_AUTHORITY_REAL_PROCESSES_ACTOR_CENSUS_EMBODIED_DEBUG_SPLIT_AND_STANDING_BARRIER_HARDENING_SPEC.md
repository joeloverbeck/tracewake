# 0052 Foundational Conformance Fourth Hardening: Production Bootstrap Unrepresentability, Closed Runtime Command Boundary, Replay Authority Reconstruction, Real Declared Processes, Exhaustive Actor Census, Sealed Embodied/Debug Split, TUI De-Authority Completion, and an Enforced Standing Conformance Barrier Hardening Spec

**Status**: PROPOSED (staged in `specs/`; not yet implemented; not yet ledger-recorded)

This is a staged hardening spec in the parallel `specs/NNNN` series. It is staged in
`specs/` and is promoted to `archive/specs/` on acceptance; it is never promoted to the
live `docs/4-specs/` tier, and it does not amend constitutional invariants, define gate
semantics, or weaken execution gates. It uses the canonical hardening-spec house structure
of its sibling specs (`0051`/`0050`/`0048`/`0046`/`0025`), not the `docs/NN_*`
narrative-document style. The default canonical `specs/` section set is not used; this
sibling-derived structure is.

It is the **fourth-pass** foundational-conformance hardening of the `0047` surface (TUI
authoritative world-advance, duration completion, actor-known interval summaries, and the
loaded-world / time-control / TUI-authority seam). The first pass was `0048`; the second pass
was `0050`; the third pass was `0051`. This pass closes the residual violations the
**fourth-pass** re-audit found surviving `0051`: `0051` closed important implementation
*instances* (a real core-owned `LoadedWorldRuntime`, several coordinator repairs) but did not
close the authority *class* — the production TUI bypasses the derived runtime handoff, the
public runtime API still permits client-selected transaction choreography, replay reseeds a
default scheduler, the declared process is a counted no-op marker, the actor census is not
exhaustive, exact temporal/control metadata still leaks through normal output and forgeable
products, and the standing mutation perimeter is not a required green barrier.

## 0. Baseline statement and source discipline

- **Driver.** `reports/0047-foundational-hardening-research-report-fourth-pass.md`, a
  recommendation-altitude independent post-`0051` static re-audit of the `0047` surface as
  hardened by `0048`, `0050`, and `0051`. The report is the originating analysis; it is not
  itself doctrine and minted no spec, invariant, risk identifier, gate code, or glossary term.
  Its predecessors are the third-, second-, and first-pass reports (drove `0051`, `0050`, and
  `0048` respectively), used by the driver only as pre-remediation baselines and explicitly
  re-derived rather than carried forward.
- **Report target commit.** The report was conducted against
  `6495d7dfe7d2d8887d4bb2ce583074c87fb273e8` (`6495d7d`), which is the current repository
  `HEAD` at authoring time. Every load-bearing code claim cited below was independently
  re-verified against that working tree:
  - F4-01: `crates/tracewake-tui/src/app.rs` `TuiApp::from_golden` calls
    `LoadedWorldRuntime::from_initial_state(RuntimeInitialState { …, scheduler:
    DeterministicScheduler::new(SimTick::ZERO) })`, bypassing `LoadedFixture::into_runtime_initial_state`
    (`crates/tracewake-content/src/load.rs`), which uses `DeterministicScheduler::from_loaded_world`.
  - F4-02: `RuntimeCommandKind` (in `runtime/session.rs`) contains only `OneTickWait`;
    `submit_controlled_proposal` accepts a caller-selected `advance_world_after_acceptance: bool`;
    `assign_proposal_sequence`, raw aggregate getters (`physical_state`, `agent_state`,
    `event_log`, `epistemic_projection`, `controller_bindings`, `registry`), `advance_until`,
    `run_no_human_day`, `rebuild_from_owned_log`, and `refresh_actor_current_place_perception`
    are all public.
  - F4-03: `scheduler.rs` `restore_from_temporal_projection` and `restore_from_rebuild_report`
    each return `Some(Self::from_loaded_world(reconstructed_final_frontier, …))`.
  - F4-04: `events/apply.rs` maps `EventKind::DeclaredWorldProcessApplied` to
    `ApplyOutcome::WorldNoOp`; `scheduler.rs` increments `world_processes_applied` for
    `ApplyOutcome::Applied | ApplyOutcome::WorldNoOp`.
  - F4-05: `scheduler.rs` `ActorStepStatus` has only `Proposed` and `Stuck`; there is no
    closed disposition for controlled-this-tick, not-due, deferred/reserved, or
    missing-substrate, so the census is not one-row-per-loaded-actor.
  - F4-06: `crates/tracewake-tui/src/run.rs` `ContinueUntil` prints
    `reason=… ticks=… stop_tick={result.stop_tick.value()}` in normal (non-debug) output;
    `tracewake-core` exposes a public `test-support` feature that `tracewake-tui` enables.
  - F4-07: `runtime/session.rs` `run_no_human_day` accepts a caller-supplied `Vec<ActorId>`,
    and `rebuild_from_owned_log` / `refresh_actor_current_place_perception` are public mutable
    methods, while `crates/tracewake-tui/src/app.rs` `current_view` orchestrates view assembly
    over the raw aggregate getters listed under F4-02.
  - F4-08: `.cargo/mutants-baseline-misses.txt` is empty (zero-floor ratchet for the in-diff lane).
  - F4-09: `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` names
    `LoadedWorldRuntime::from_loaded_world` as the current production entry and
    `TuiApp::submit_entry_with_world_advance` as the TUI crossing — both contradicted by the
    F4-01/F4-02 code state above (the live truthing this spec defers to §6.1).
  The report relies on named symbols, not line numbers (its `#Lxxxx` anchors are not relied
  upon and several drift from the live tree).
- **Static-survey limitation inherited from the driver.** The report forbade execution, so its
  pass/fail statements are static readings of source/test intent, not command results. This spec
  therefore states code *structure* facts as verified and defers all green/red command claims to
  the implementing session (§8).
- **Source discipline.** A commit hash named in this spec is audit/provenance only. This spec
  recommends and scopes work; it does not declare latest-`main` certification or any phase entry.
  When executed, the implementation must name its own exact implementation commit, not assume this
  baseline commit is latest `main`.
- **Ledger timing.** Per the sibling-spec convention, this package receives its
  `docs/4-specs/SPEC_LEDGER.md` archived-row entry at acceptance/closeout, not at proposal. This
  spec authors no ledger row now and makes no change to live `0001` or the ledger.
- **Archived-history discipline.** The archived `0047`/`0048`/`0050`/`0051` specs, their
  acceptance artifacts, the `0049MUTWIT` and `0050FOUCONSEC`/`0051FOUCONTHI` tickets, and any
  passed certification are immutable, commit-pinned historical records. This spec does not edit
  them and does not treat their claims as automatic current-state proof. In particular, the `0051`
  acceptance artifact's closure table and its honest non-green standing-mutation statement
  (3,275 selected / 2,549 caught / 703 unviable / **23 missed** / 0 timeout at `a3b46c6`) are
  immutable; the correct response is this new remediation line plus a future acceptance artifact,
  never retrospective alteration.

## 1. Scope

### 1.1 In scope

The nine divergences and warranted gaps the driver found at `6495d7d`, on the
`0047`/`0048`/`0050`/`0051` seams and their named conformance collaborators. Finding IDs are
preserved from the driver (`F4-01`…`F4-09`) for lineage cross-reference.

1. **F4-01 — Production TUI bootstrap bypasses the derived loaded-world runtime handoff
   (critical; vacuity gap; evidence-honesty gap).** `LoadedFixture::into_runtime_initial_state`
   builds a scheduler with `DeterministicScheduler::from_loaded_world`, and a loader unit witness
   exercises that handoff, but `TuiApp::from_golden` manually rebuilds `RuntimeInitialState` and
   injects `DeterministicScheduler::new(SimTick::ZERO)`, whose `loaded_actor_next_decision_tick`
   and `declared_world_processes` maps are empty. The production TUI therefore starts with no
   loaded-actor or declared-process census; the correct handoff is proven only off the production
   path.
2. **F4-02 — The runtime command boundary is incomplete and preserves client-selected transaction
   choreography (critical).** The only closed command is one-tick wait, and even that is not the
   production wait path: `TuiApp::submit_entry` converts `wait.1_tick` into a proposal routed
   through `submit_controlled_proposal` with a caller-selected `advance_world_after_acceptance`
   boolean. Public methods expose raw aggregates, proposal-sequence allocation, direct non-wait
   pipeline dispatch, raw `advance_until`, caller-supplied no-human actor lists, replay rebuild,
   and perception refresh. The TUI composes those itself and decides whether the rest of the
   loaded world advances.
3. **F4-03 — Replay restoration reconstructs a fresh default scheduler instead of replaying
   scheduler authority (critical; mutation-survivor disposition).** Both restore paths call
   `from_loaded_world` at the reconstructed frontier, resetting the proposal sequence to zero,
   re-scheduling every actor at `current_tick + 1`, re-declaring one synthetic every-tick process
   with no source-event IDs or random provenance, and discarding irregular actor opportunity
   intervals, nonzero proposal sequences, multiple/non-unit-cadence processes, process ancestry,
   and reservation/deferral state. The current continuation witness starts both branches from that
   same default topology and compares a shallow next-step projection, so it cannot prove
   preservation of non-default runtime authority.
4. **F4-04 — The declared loaded-world process is a generic no-op marker counted as applied
   (critical).** The default declaration (`process_loaded_world_tick`, first due next tick, cadence
   one, empty source-event IDs, `None` random provenance) builds a `DeclaredWorldProcessApplied`
   event that `events/apply.rs` classifies as `WorldNoOp`, while the world-step loop increments
   `world_processes_applied` for `Applied` **or** `WorldNoOp`. The event is simultaneously treated
   as proof a process transaction occurred and as a no-op at application time. That does not satisfy
   the declared-causal-process burden of INV-088.
5. **F4-05 — The claimed exhaustive actor disposition census is incomplete (hardening gap;
   evidence-honesty gap).** Controlled/autonomous mutual exclusion and `ActorDecisionTransaction`
   consumption are genuinely present, but `ActorStepStatus` has only `Proposed` and `Stuck`,
   `actor_step_summaries` is populated only for actors whose transaction was attempted, and there is
   no closed disposition for *controlled this tick*, *not due*, *deferred/reserved/body-exclusive*,
   *missing required substrate*, *budget-exhausted*, *invalidated*, or *otherwise skipped*.
   `WorldStepDueWorkSummary` gives counts but no one-row-per-loaded-actor identity proof. The
   `0051` "closed exhaustive per-tick actor disposition census" closure claim exceeds the product.
6. **F4-06 — Embodied temporal products and normal command output remain authority-bearing and
   forgeable (high; mutation-survivor disposition).** `render_embodied_view` is repaired
   (qualitative notices only), but the normal `continue` command prints exact `reason`, `ticks`, and
   `stop_tick` outside any debug label; `TypedActorKnownIntervalSummary` and `EmbodiedViewModel`
   expose exact start/stop ticks, frontiers, stop reason, and holder-known context through public
   accessors; the public `test-support` feature makes temporal fields and test constructors public
   (and the TUI enables that feature), so supposedly sealed products are forgeable across supported
   feature combinations; and the TUI mutates the assembled embodied product with public setters and
   performs the internal→embodied interval conversion (`TuiApp::advance_until`) on the client side.
7. **F4-07 — No-human, replay, perception, view construction, checksum, and debug flows remain
   TUI-orchestrated over raw aggregates (high; mutation-survivor disposition).** `current_view`
   reads raw `physical_state`/`agent_state`/registry/projection/tick/manifest/log-length and
   constructs contexts/snapshots itself; `run_no_human_day` accepts a caller `Vec<ActorId>` and the
   TUI passes an empty vector then separately computes a checksum context, calls
   `rebuild_from_owned_log`, and refreshes perception; `rebuild_from_owned_log` and
   `refresh_actor_current_place_perception` are public mutable methods and historical standing
   survivors; debug/checksum methods read raw aggregates with access checked in the TUI, not enforced
   by a core capability-bearing receipt. Other clients could compose a different transaction order or
   read authoritative aggregates to build their own surfaces.
8. **F4-08 — The standing mutation perimeter is not a required, green, full-surface regression
   barrier (mutation-survivor disposition; evidence-honesty gap).** The `0051` standing campaign
   completed with **23 missed mutants** and was accepted honestly non-green. The full canonical
   census / eight shards / reconciliation run only on `workflow_dispatch` or weekly schedule, not as
   a merge-required PR check; in-diff mutation cannot detect witness-only regressions that weaken a
   test/fixture/transcript/CI trigger without touching a protected production line; and
   `anti_regression_guards.rs` source scans remain topology alarms, not proof of atomicity, replay
   continuation, process semantics, one-opportunity-per-actor, or information-flow noninterference.
   The 23 survivors are dispositioned in §5; the out-of-surface `food_source_fact_supersedes` family
   (survivors #1–7) routes forward as cross-cutting and may not be laundered as equivalent.
9. **F4-09 — Live conformance and risk evidence still overstates production reachability and closure
   (evidence-honesty gap; post-implementation doc work only).** Architecture `00` names
   `LoadedWorldRuntime::from_loaded_world` as the production entry and `submit_entry_with_world_advance`
   as the TUI crossing, and R-27 repeats the production-constructor claim; the code contradicts both.
   The doctrine is correct and the spec ledger's historical rows are already honest; only live
   conformance/status/navigation evidence is ahead of the code, and is corrected **after** executable
   closure.

### 1.2 Out of scope

- Any edit to archived specs, tickets, acceptance artifacts, reports, or passed certifications, or
  to the `SPEC_LEDGER.md` archived rows / `0049MUTWIT` source record (already accurate).
- Any constitutional-invariant amendment, gate-semantics change, or new risk-ID / glossary mint
  (see §6). The driver's determination is **no amendment at any tier**.
- The out-of-surface mutation family **#1–7** (`food_source_fact_supersedes`): it routes forward to
  a future cross-cutting mutation-remediation line via owner/reassess, with no equivalence claim and
  no lower-tier routing; the canonical perimeter may not be called green while it survives.
- Any new property-testing dependency (`proptest`/`quickcheck`); the existing deterministic corpus,
  generated lock corpus, integration seams, negative fixtures, and mutation campaigns are sufficient
  (driver §3.5, §6).
- Phase-4 entry, second-proof entry, latest-`main` certification, or feature expansion. No new gate
  rung is created; the barrier is distributed across the existing certification ladder (§4.10).
- Re-litigating properties the fourth pass found structurally present and to be **preserved, not
  rebuilt** (driver §3.5): physical private ownership of `LoadedWorldRuntime` aggregates, scratch-state
  atomic cutover, duplicate-`EventId` fail-closed rejection, controlled/autonomous mutual exclusion,
  `ActorDecisionTransaction` proposed/stuck consumption, `render_embodied_view` qualitative rendering,
  and the non-diegetic debug overlay label (§4.11).

## 2. Doctrine anchors

Authority order applied without inversion: `0-foundation` → `1-architecture` → `2-execution`
→ `3-reference`. Controlling invariants by finding (re-verified against the live constitution):

- **Causality / event sourcing / replay.** INV-001, INV-009, INV-010, INV-018, INV-092.
- **No-human authority and possession parity.** INV-004, INV-005, INV-087, INV-091, INV-094, INV-108.
- **Declared regional/world processes.** INV-088.
- **Cognition authority / decision diagnostics / truth firewall.** INV-041, INV-101, INV-102, INV-105.
- **TUI/embodied/debug boundary and leakage severity.** INV-067, INV-068, INV-069, INV-093.
- **Temporal authority.** INV-112.
- **Harsh acceptance and validation.** INV-043, INV-098.

Architecture homes: `00` (index/conformance), `02` (event log/replay/save/restore), `03`
(holder-known contexts/provenance), `04` (scheduler/world-step/process transaction), `05` (actor
decision transaction), `10` (TUI/client boundary), `12` (declared process at regional/LOD altitude),
`13` (validation/observability/acceptance). Execution homes: `05` (transaction scheduler / no direct
dispatch), `06` (no-human / possession differential), `07` (epistemic view models / possession /
debug proof), `09` (golden/replay acceptance), `10` (testing/evidence discipline), `03` (phase
ladder / certification placement). Reference homes: `00` (review checklist), `01`
(R-27/R-28/R-29 risk memory — status/evidence only).

## 3. Determination

**Not conformant; remediation warranted; no higher-tier doctrinal amendment warranted.**

The `0051` line delivered real and necessary gains — a core-owned `LoadedWorldRuntime` with private
aggregate ownership, scratch-state atomic cutover, controlled/autonomous mutual exclusion, full
`Proposed`/`Stuck` transaction consumption, duplicate-ID fail-closed rejection, and a repaired
embodied renderer. Those are recorded as present (§4.11) and not re-reported as defects. They do not
establish foundational conformance: `0051` **closed important implementation instances without closing
the authority class**.

The root cause is structural and recurrent (driver §6.1): a correct constructor exists *beside* an
injectable one; a closed command token exists *beside* open public methods; private fields were
treated as client de-authority while public getters, raw result types, sequence allocation, and
multi-call composition remain; and focused mutation evidence was allowed to close a family while the
standing floor stayed non-green. This is the fourth consecutive pass to find critical divergence on
this seam after a remediation claimed closure. The cycle-breaking change is therefore not another
per-finding test list but a **three-layer enforced standing barrier** (§4.10) that binds production
reachability, type-level API authority, and standing mutation sensitivity together: compile-time
unrepresentability of client write authority, a required public-boundary conformance CI lane, and a
green canonical standing mutation perimeter.

Independent re-verification at `6495d7d` confirmed every load-bearing code claim (§0). F4-08 is a
measured-evidence gap whose green/red disposition defers to the implementing session's clean standing
run; F4-09 is a documentation-honesty gap remediated only **after** code closure.

## 4. Findings and remediation requirements

Each finding names a **code home**, a **remediation requirement**, and the **strongest practical
anti-regression guard** (compile-time/type ownership first, then production-path behavior, then
focused mutation). Source-text / `include_str!` guards are permitted only as labeled topology alarms
and never as sole proof of atomicity, replay continuation, process effects, one-opportunity-per-actor,
holder-known noninterference, production reachability, or mutation sensitivity (driver §6.4). Every
source guard for this surface must point to the compile-time or executable witness it protects and
include a synthetic negative proving the guard itself can fire.

Subsections are keyed by finding ID, not by implementation order. §9 (closure order) governs
sequencing.

### 4.1 Structural keystone — opaque core-owned session, closed commands, sealed receipts

Build on the existing `LoadedWorldRuntime` rather than wrapping it again. Reduce its public surface
to: an **opaque loader-to-runtime production constructor** consumable only by the production bootstrap;
a **closed set of typed commands** (semantic player intent, one-tick wait, continue/advance-until,
binding/unbind, debug-authorized no-human execution, replay/recovery, view/debug queries); and
**immutable sealed typed receipts** (embodied receipt/view, debug receipt/view under capability, or
typed rejection). Remove from the client-facing API: the injectable `from_initial_state` /
`RuntimeInitialState` path, raw aggregate getters, `assign_proposal_sequence`, the
`advance_world_after_acceptance` boolean, raw `advance_until`/`PipelineResult`/`AdvanceUntilResult`
returns, caller no-human actor lists, and `rebuild_from_owned_log` / `refresh_actor_current_place_perception`
as independent client operations. Core-internal modules remain free to use references; the restriction
is the public production client surface. This keystone is the home for F4-01, F4-02, F4-05, F4-06, and
F4-07 remediations; the finding subsections state the obligations that ride on it. No
backwards-compatibility public alias may be left behind; a temporary internal adapter may exist only
long enough to migrate core tests and must be removed before closeout. `#[non_exhaustive]` alone is
**not** an authority boundary — private fields, crate-private constructors, opaque loader exports, and
unexported authority tokens are the controlling mechanism.

### 4.2 F4-01 — Production loaded-world bootstrap unrepresentability

**Code home:** `tracewake-content/src/load.rs`, `runtime/session.rs`, `crates/tracewake-tui/src/app.rs`.
**Requirement:** content loading returns an opaque core-owned bootstrap/export consumable only by the
production runtime constructor; the externally usable `LoadedWorldRuntime::from_initial_state(RuntimeInitialState)`
authority path is removed or made crate-private behind a core/content integration boundary the TUI
cannot call; `TuiApp::from_golden` calls the same loader-to-runtime production constructor as all other
loaded worlds and receives an already-authoritative runtime/session, not physical state plus a scheduler
choice; the initial replay seed is preserved through an opaque core-owned seed/snapshot so the TUI need
not retain mutable initial aggregates. Atomic cutover; no injectable-path alias. **Guards:** (1) external
negative fixture — a client crate cannot construct `RuntimeInitialState`, inject a `DeterministicScheduler`,
or call an unvalidated constructor; (2) production-boundary integration test — invoke the real content
loader and the same public TUI/application bootstrap, bind one actor, submit the typed wait command, and
assert committed effects from at least one *other* loaded actor and one declared process, asserting event
IDs/types, actor/process identities, resulting state/projection changes, and replay agreement (not just
counters), with **no** test call constructing a scheduler or seeding eligibility; (3) a deliberate local
mutation swapping the production constructor for `DeterministicScheduler::new` must make this test fail.
Extend `negative_fixture_runner.rs`, `generative_lock.rs`, the loader test, `world_step_coordinator.rs`,
and the TUI seam tests; add no new framework.

### 4.3 F4-02 — Closed runtime command boundary

**Code home:** `runtime/session.rs`, `crates/tracewake-tui/src/app.rs`, `actions/pipeline.rs`.
**Requirement:** replace the boolean and raw proposal route with a closed typed command family owned by
core; proposal IDs/sequences and ordering keys are allocated **inside** the runtime — the client supplies
semantic intent and actor-filtered target selection, not scheduler sequence authority; every accepted
world-affecting command routes through one core transaction coordinator, and whether a non-wait action
consumes time is a **core rule**, not a TUI boolean; commands return sealed receipts (embodied result,
debug result under capability, or typed rejection), never raw `PipelineResult`/`AdvanceUntilResult`;
rejected-action atomicity is core-decided (advance nothing, commit a consequential failure event, or
produce actor-visible feedback per event doctrine). The production TUI wait path uses the real command,
not a proposal-with-boolean. **Guards:** compile-fail fixtures proving an external crate cannot call
`run_pipeline` with runtime aggregates, assign proposal sequences, pass a world-advance boolean, or
obtain authoritative aggregate references; a table-driven public-boundary suite sending wait, non-wait
accepted, non-wait rejected, duration start/continue, and pause/continue through one dispatcher and
asserting correct world-step/event behavior; a differential witness where equivalent controlled and
autonomous actions reach the same validation/event rules with controller binding changing input only;
mutation coverage over command dispatch, action-to-time policy, rollback/commit, and sequence allocation
(closes survivors #19, #22). Closure tests use the public production command accepted by the TUI, not
`transact_world_one_tick` directly or a test-sequenced proposal.

### 4.4 F4-03 — Replay-critical runtime authority reconstruction

**Code home:** `scheduler.rs`, `replay/{temporal,rebuild,report}.rs`, `runtime/session.rs`.
**Requirement:** define replay-critical scheduler authority as an explicit projection or
ancestry-preserving snapshot derived from the event log and accepted initial configuration, reconstructing
at least: next proposal sequence (or a deterministic event-derived equivalent); per-loaded-actor next
opportunity and any deferral/reservation disposition; every declared process identity, trigger/cadence,
source-event ancestry, scope, content identity, deterministic random provenance, and next-due state; and
open duration/reservation state relevant to the next world step. Restore **fails closed** when required
authority is absent, ambiguous, unsupported, or inconsistent with physical/agent/projection state, and
does **not** silently synthesize the default process topology; initial loading may derive declarations
from validated content, but continuation must preserve the authority that existed at the replay frontier.
`rebuild_from_owned_log` becomes an internal atomic command that replaces all aggregates and scheduler
authority together or changes nothing. **Guards:** an uninterrupted-vs-prefix-rebuild differential over
the real runtime boundary — ≥3 actors on nonuniform next-opportunity schedules, multiple proposal sequences
consumed before save, ≥2 processes with different cadence/source-events/random provenance, an open
duration/body reservation, and an actor deferred past the immediate next tick; run a prefix, rebuild from
owned history, execute the same next N typed commands, and compare exact event envelopes/order/IDs,
proposal ancestry, process triggers, actor dispositions, receipts, physical/agent/epistemic projections,
and checksum/frontier. Mutants returning `None`, resetting sequences, changing cadence, dropping a
process/source, or moving an actor's next tick must be killed (closes survivors #8, #20). A witness whose
restored topology equals the constructor default by construction is vacuous; it must create replay-critical
state `from_loaded_world` cannot infer from current physical/agent maps alone.

### 4.5 F4-04 — Real declared-process causal transactions

**Code home:** `scheduler.rs`, `events/{envelope,apply,log}.rs`. **Requirement:** choose one honest model.
The recommended model for this surface is a **minimal real process transaction**: validated content
declares a concrete process kind and causal inputs; a due invocation identifies a process kind + trigger
witness and runs a process-specific transition that emits one or more meaningful events/effects with
source/cadence/random ancestry through the **same atomic cutover** as actor work; only committed,
successfully applied effects increment an applied-process count, and the declaration is reconstructed on
replay per §4.4. (The alternative honest model — retain a due-process observation as an explicit
diagnostic/non-`WorldNoOp` marker, rename counters accordingly, and never present it as application — is
acceptable but weaker, because it leaves no-human progression and replay continuation vacuous.) A
diagnostic marker may *accompany* a transition but cannot *be* it; `world_processes_applied` counts
committed process transactions only. **Guards:** a process fixture with pre-due / due / post-due
assertions of process-specific state/projection/event effects, declaration/source/cadence/random
ancestry, and exact replay continuation; a negative fixture proving external crates cannot construct due
invocations or inject raw process events; mutation kills for `WorldNoOp` application, removed concrete
effect, altered cadence/source ancestry, and counting no-op as applied. A marker's presence, a counter of
one, or an event name containing "Applied" is not evidence of a transition.

### 4.6 F4-05 — Closed exhaustive per-loaded-actor disposition census

**Code home:** `scheduler.rs`, actor transaction/trace, coordinator and reservation tests.
**Requirement:** introduce a closed core-owned actor disposition assigning **exactly one** disposition to
every loaded actor at each world step, derived from the runtime-owned loaded actor set (the caller supplies
no actor list), reusing existing concepts without minting a doctrine identifier. Include at least:
controlled-proposal path, autonomous proposed, autonomous stuck, not due, deferred/reserved/body-exclusive,
and invalid/missing-substrate fail-closed states as the implementation requires. Enforce exactly one
disposition per loaded actor, stable ordering, and no duplicate opportunity; diagnostics include
responsible layer and causal/temporal basis without leaking hidden truth into embodied products. **Guards:**
a deterministic generated corpus over actor order, possession placement, due ticks, reservations, active
durations, and stuck/proposed outcomes; for every step `census.len() == loaded_actor_set.len()`, actor IDs
equal as sets, each actor appears once, the controlled actor is never autonomous in the same step, and every
closed disposition is reached by ≥1 fixture or explicitly documented as staged; a differential human/no-human
run holding world state and due-work equal; mutation coverage over filters, disposition arms, and census
cardinality. A witness comparing only `actor_transactions_attempted`, testing a one-actor world, or
preselecting actor IDs is vacuous; it must derive the full loaded actor set from the public runtime receipt
or accepted debug product and prove one disposition per actor.

### 4.7 F4-06 — Sealed embodied/debug temporal split; non-leaking normal output

**Code home:** `view_models.rs`, `projections.rs`, `crates/tracewake-tui/src/{app,run,render}.rs`, crate
feature configuration. **Requirement:** return two distinct core-created products — an **embodied
receipt/view** containing only actor-known qualitative interval consequences and actor-legible stop
information with provenance, and a **debug receipt/view** containing exact ticks, frontiers, event IDs,
scheduler stop reasons, and control bounds, constructible only through the existing non-forgeable debug
capability boundary. Remove exact tick/frontier/control reason from the embodied public type (not merely
hide fields while keeping public getters); make all production fields and constructors private in **every**
feature combination, replacing public `test-support` constructors/fields with internal `#[cfg(test)]`
builders / crate-private test helpers; move interval-summary attachment and notebook/debug composition
into core view-model construction so the TUI receives an immutable product; render normal `continue` output
from the embodied receipt — exact `stop_tick`, raw tick count, and `controller_safety_bound` belong only in
the visibly non-diegetic debug product unless a modeled actor-known clock/procedure supplies them. **Guards:**
external compile-fail fixtures under default **and all supported** feature combinations (cannot construct/
mutate embodied temporal products, cannot call debug constructors without capability, cannot convert a debug
receipt to an embodied receipt); a public command-loop hidden-world pair — two worlds differing only in
hidden exact time/frontier/control metadata produce identical normal output and semantic action surface while
debug output differs and is labeled; a normal-transcript guard banning exact tick/frontier/event IDs and
internal stop-reason tokens, with a debug transcript positively requiring them; mutation tests for all
interval accessors / stop classification / transcript-section selection (closes survivors #9–16, #23). A
`tick` source scan is only a topology alarm; non-vacuous proof compares semantically equivalent hidden-world
pairs through the public command boundary and exercises both products.

### 4.8 F4-07 — Complete TUI de-authority over no-human / replay / perception / view / checksum / debug

**Code home:** `runtime/session.rs`, `crates/tracewake-tui/src/app.rs`, core view/debug facade.
**Requirement:** expose one opaque session handle with typed commands and sealed view/debug receipts;
remove public raw aggregate getters from the client-facing API. No-human advancement becomes a single
runtime command that derives its actor census internally and performs advancement/rebuild/projection/
perception work atomically, returning a typed debug/observer receipt; rebuild becomes an internal recovery
operation (the runtime owns its seed/snapshot and checksum context), not a client command; possession
binding/perception update becomes one core transaction with explicit event/projection effects and a receipt;
debug queries route through the existing non-forgeable debug capability so a debug receipt can expose exact
state safely while the TUI receives no general-purpose authoritative references. Core-internal modules may
still use references; the restriction is the public production client surface. **Guards:** external negative
fixtures proving the production session yields no `&PhysicalState`/`&AgentState`/`&EventLog`/
`&EpistemicProjection`/`&ControllerBindings`/`&ActionRegistry` and cannot call rebuild, perception refresh,
sequence allocation, or no-human with an actor list; public-boundary TUI tests bootstrapping production,
invoking no-human through a typed debug command, and asserting atomic committed receipt/replay equivalence; a
fault-injection/duplicate-event fixture forcing an internal post-step failure and proving no partial
state/log/projection cutover; mutation kills for `rebuild_from_owned_log -> Ok(())` and perception-refresh
deletion (closes survivors #20, #21). A fixture that merely fails to mutate a private field is insufficient;
it must attempt the actual surviving authority routes.

### 4.9 F4-08 — Standing mutation survivor closure (see §5 for per-survivor disposition)

**Code/test home:** extend existing scheduler/replay, epistemic projection, interval, salience, generative,
deserialization-codec, and TUI suites; do **not** edit `.cargo/mutants.toml` to restate existing coverage,
and do **not** pad `.cargo/mutants-baseline-misses.txt` for killable foundational mutants (the baseline stays
empty for this surface). **Requirement:** drive every in-surface survivor (§5: #8–#23) to caught or to
unviable-with-a-defensible-compiler/tool-reason; resolve or semantically justify the out-of-surface
`food_source_fact_supersedes` family (#1–7) before calling the canonical perimeter green, or route it to a
cross-cutting line in a way that preserves one canonical reconciliation and launders no survivor. After all
code/test work, run the full standing campaign (`cargo mutants --timeout 183` with the standing config) from
a clean baseline and publish the selected denominator with the complete caught/missed/unviable/timeout
disposition. **Evidence honesty:** "mutation completed" is not "mutation passed"; a focused 0-miss run is
evidence only for its selected functions/mutants and exact source tree; exit 2 (missed) and exit 3 (timeout)
are failures for the standing green claim, as are tool errors, incomplete census, stale artifacts, missing
shards, and fingerprint mismatch. No survivor is classified equivalent without a defensible semantic argument.

### 4.10 The enforced standing barrier (cross-finding keystone for F4-08, governs F4-01…F4-07 evidence)

Per driver §6.2, the durable barrier has three mutually reinforcing layers; none suffices alone.

- **Layer A — compile-time unrepresentability (load-bearing).** The production public API makes the
  following impossible to express: constructing a loaded runtime with a caller-selected scheduler; obtaining
  or mutating authoritative aggregate handles; allocating proposal sequence/order authority; choosing whether
  a command advances the world via a boolean/callback; supplying the no-human actor/process census; calling
  replay rebuild or perception writers as independent client operations; constructing or mutating embodied
  temporal/context/debug fields; obtaining exact debug/time products without the existing debug capability;
  or compiling a feature combination that reopens those authorities. Implement with private fields,
  crate-private constructors, opaque loader exports, unexported authority tokens, closed command constructors,
  sealed receipts, and separate embodied/debug types; extend the existing external-crate negative-fixture
  pattern with both default-feature and all-supported-feature compile checks; keep `anti_regression_guards.rs`
  only as a secondary topology alarm verifying the fixtures/perimeter remain registered.
- **Layer B — required public-boundary conformance CI lane.** Compose existing tests into one named CI job
  (mint no new doctrine gate code) whose path is: validated content package → production loader/bootstrap →
  opaque runtime/session → typed TUI/application commands → core transaction(s) → sealed embodied/debug
  receipts → rendered normal/debug output → replay/rebuild continuation. The proving tests must not construct
  `DeterministicScheduler`, `RuntimeInitialState`, `PipelineContext`, due actor/process lists, raw proposals
  with assigned sequence, or raw view models. Minimum behavior matrix (driver §6.2): production bootstrap +
  one-tick wait; non-wait accepted (core-owned time policy); rejected action (atomicity + actor-visible
  feedback, no leak); continue/duration (sealed embodied stop + separate debug); no-human (runtime-derived
  census through the same coordinator); possession differential (held-equal worlds differ only by input
  route); replay restore (non-default authority continuation); declared process (concrete effect + ancestry);
  actor census (one disposition per loaded actor); information-flow pair (hidden time/truth changes do not
  alter normal output; debug may differ and is labeled); failure injection (no partial cutover). Make this a
  required branch-protection check; the acceptance artifact records the required-check name and
  repository-governance confirmation (a workflow file alone cannot prove branch protection).
- **Layer C — green standing mutation perimeter with zero-floor governance.** The canonical perimeter remains
  `.cargo/mutants.toml` (do not create a parallel spec-0052 config that can drift): zero accepted in-surface
  misses/timeouts; full canonical reconciliation (commit/config/toolchain/census fingerprints) before
  remediation acceptance; the public-boundary lane runs whenever production files, tests, fixtures, negative
  fixtures, mutation config/baseline, CI workflow, merge/supervisor tooling, or live conformance evidence for
  this surface changes; a full surface campaign is required when a change can weaken old witnesses without
  touching old production lines; the weekly scheduled campaign is kept as drift detection but a red scheduled
  result is treated as merge-blocking until repaired; no laundering (exit 2/3, tool errors, incomplete census,
  stale artifacts, absent output are failures); the food-source family is still reported by the canonical
  reconciliation even if owned by cross-cutting remediation.

**Certification-ladder placement (driver §6.3):** no new gate code is created. The barrier is an artifact
dependency distributed across existing rungs — SPINE-CERT (sole command boundary, event/process causality,
atomic world step, replay restoration, duplicate-ID fail-closed), EPI-CERT (sealed actor-known interval/view
products, temporal noninterference, debug capability/quarantine), ORD-LIFE-CERT (no-human census, full actor
decision transaction, possession-neutral opportunity, stuck diagnostics), FIRST-PROOF-CERT (integrated
production-content bootstrap through TUI commands to sealed receipts/rendering and replay, canonical mutation
perimeter green for the depended-on surface). This does not retroactively rewrite archived certification
artifacts; future acceptance depending on this surface may not treat the relevant rung complete while the
barrier is red or bypassed.

### 4.11 Preserved properties (regression-guard, not rebuild)

The following are present by static reading at `6495d7d` and must be **preserved** through the §4.1 refactor,
re-expressed under the opaque session, and kept as required production-runtime cases (driver §3.5): physical
private ownership of `LoadedWorldRuntime` aggregates; scratch-state atomic cutover before commit (extend
failure injection to each actor/process/perception phase); the world-step request rejecting caller-injected
due actor IDs or prebuilt process events; controlled/autonomous mutual exclusion within a tick;
`ActorDecisionTransaction` `Proposed`/`Stuck` consumption into typed summaries and committed
ancestry/diagnostics; duplicate-`EventId` fail-closed rejection (drive a collision through the world-step
transaction proving atomic rollback); `render_embodied_view` qualitative interval rendering; and the
explicitly non-diegetic debug overlay label.

## 5. Standing-mutation survivor disposition

Per driver §5. The `0051` historical standing run (3,275 selected / 2,549 caught / 703 unviable / **23
missed** / 0 timeout at `a3b46c6`) is a historical artifact claim, not a current result. No survivor is
called equivalent merely because the suite did not kill it; equivalence requires a defensible semantic
argument. The 23 historical survivors are dispositioned as follows.

- **#1–7 `food_source_fact_supersedes` family (out of surface).** Routed-forward cross-cutting (truth-table
  collapse, arm deletions, `<`→`==`/`>`/`<=` ordering flips). Not moved to a lower tier and not removed from
  canonical mutation governance. The implementing line must either resolve it in the same remediation or leave
  the overall canonical perimeter explicitly non-green and therefore not accept the standing gate (§4.9).
- **#8 `restore_from_temporal_projection -> None`.** Concrete F4-03 target (§4.4): non-default
  uninterrupted-vs-restored continuation through the public runtime; exact next-N event/receipt equality.
- **#9–16 view/interval accessors** (`EmbodiedViewModel::sim_tick`, interval `start_tick`/`stop_tick`,
  `start_frontier`→0/1, `stop_frontier`→0/1, `no_new_actor_known_information`→true). Concrete F4-06 targets
  (§4.7): the preferred closure is to **remove** exact authority from embodied public products and retain/test
  exact fields only in internal/debug products — making the forbidden product unrepresentable is stronger than
  a getter-calling test.
- **#17–18 `StuckDiagnostic::deserialize_canonical`** (match-arm deletion; `!` deletion). Concrete targets:
  full-field non-default round trip plus malformed/missing/unknown-field fail-closed cases; valid/invalid
  canonical records differing only in the affected predicate; focused and standing rerun.
- **#19 `assign_proposal_sequence -> default`.** Concrete F4-02 target (§4.3): remove the client method;
  a runtime-command test submits multiple commands and proves unique monotonic/event-derived ordering.
- **#20 `rebuild_from_owned_log -> Ok(())`.** Concrete F4-03/F4-07 target (§4.4/§4.8): a public command
  creates state divergence only an actual rebuild repairs, then compares all aggregates/scheduler — no direct
  helper call from the TUI.
- **#21 delete `refresh_actor_current_place_perception`.** Concrete F4-07 target (§4.8): a binding/post-rebuild
  command causes a provenance-bearing perception/epistemic delta observable in the sealed view and replay.
- **#22 `submit_entry_with_world_advance` branch (`semantic_action_id == "wait.1_tick"`).** Concrete
  F4-01/F4-02 target (§4.3): a public TUI differential proving wait and non-wait use the correct single core
  transaction policy; eliminate the client boolean.
- **#23 transcript section selector `==`→`!=`.** Concrete evidence target (§4.7): a representative transcript
  asserts exact required section membership/exclusion and feeds the real acceptance-artifact path.

## 6. Doctrine amendments

**None warranted at any tier.** The driver's tier-by-tier determination (report §7.1) and independent
re-reading of the live constitution (§2 spot-verification of INV-067/069/088/092/094/098/112) confirm doctrine
already states every load-bearing requirement this spec enforces: one event-sourced causal transition and
deterministic tested replay (INV-001/009/010/018/092); loaded-world actor/process progression under possession
rather than a caller-seeded or empty tick (INV-004/005/087/088/091/094/108); ordinary proposal/validation
parity and core-owned ordering (INV-043); declared regional/world processes with source/cadence/inputs/
random/traces/ancestry (INV-088); full cognition-transaction ancestry and inspectable stuck diagnostics
(INV-041/101/102/105); actor-known embodied reality, visibly non-diegetic debug, leakage-as-high-severity, and
temporal authority bounded to validation/order rather than embodied/cognition authority (INV-067/068/069/093/112);
a read-only client boundary (INV-069); and harsh, path-under-test, non-vacuous acceptance evidence (INV-098).

The defects arise because code and evidence fall **below** those rules, not because the rules are missing or
contradictory. Weakening doctrine to bless an empty production scheduler, caller-selected progression, replay
reseeding, a diagnostic process marker counted as applied, client-owned aggregate choreography, exact hidden
time in normal output, or a permanent mutation-survivor floor would be a constitutional inversion and is
explicitly rejected. This spec mints no invariant, gate code, risk ID, or glossary term, and authors no
ratified doctrine wording. The only documentation work is **post-implementation live-doc truthing** (§6.1),
which corrects status/evidence rows without changing doctrine altitude.

### 6.1 Post-implementation live-doc truthing (F4-09; after executable closure only)

Do not edit docs first. After F4-01…F4-08 are implemented and executed at one exact commit (driver §7.2):
architecture `00` replaces the loaded-world overclaim with the actual single production bootstrap and current
named public-boundary/green-mutation witnesses; architecture `02/04/05/10/13` update implementation pointers
and evidence shape (not doctrine altitude — replay stays non-reseeding, the scheduler stays the sole
coordinator, the client stays read-only, source scans/focused mutation are not exhaustive proof); execution
`05/06/07/09/10` update current commands, products, census, replay restore, and gate procedure; reference `00`
updates reviewer pointers; reference `01` updates **R-27/R-28/R-29 status/evidence only** (mint no risk ID,
retire no general risk); `SPEC_LEDGER.md` routes this remediation spec and closeout through the normal
ledger/archive process. Every updated evidence row must answer: which production constructor created the
runtime, which public command crossed the client boundary, what state/event/projection effect was observed,
and which deliberate mutation or negative compile attempt proves sensitivity. No archived spec/ticket/report/
acceptance/certification is edited.

## 7. Required fixtures, tests, and CI

Extend existing machinery (driver §6); do **not** duplicate it or add a property-testing dependency:

- **External-crate negative fixtures** (`tests/negative-fixtures/*`, `negative_fixture_runner.rs`): add/extend
  fixtures naming the **real** forbidden capabilities (scheduler injection / `RuntimeInitialState` construction,
  raw aggregate getters, proposal-sequence allocation, world-advance boolean, `run_pipeline` over authoritative
  aggregates, no-human actor-list injection, rebuild/perception writers, embodied-product construction/mutation,
  debug→embodied conversion, exact temporal-field access) under **default and all supported feature
  combinations**, each pinned to a privacy/constructor diagnostic rather than a generic "cannot find function";
  add positive in-crate tests proving the single core owner can still perform each operation. Reuse and extend
  the existing `0050`/`0051`-line privacy guards rather than duplicating them.
- **Production-bootstrap conformance lane** (Layer B, §4.10): the named CI job and its minimum behavior matrix.
- **Production-bootstrap differential** (`world_step_coordinator.rs` + a runtime/session test): start through
  the production constructor with **no** scheduler-registration calls; prove loaded actors and declared processes
  advance; deliberately remove the production derivation and require the test to fail.
- **Continuation equivalence** (replay/temporal/rebuild tests): uninterrupted-vs-restored comparison over all
  runtime authority including non-default schedules, multiple/non-unit-cadence processes, proposal ancestry,
  reservations, and mid-duration restore.
- **Per-tick census** behavior + permutation/differential tests (§4.6); **declared-process effect**
  before/at/after cadence + replay (§4.5); **full actor-outcome** ancestry through the real coordinator,
  including `Stuck` and rejection.
- **Embodied/debug separation** (§4.7): default-transcript leak-absence assertions, debug-capability presence
  assertions, and a hidden-context/debug-receipt information-flow differential.
- **Deterministic generated corpus** (`generative_lock.rs`): a production-constructor mode covering zero/one/
  multiple actors, possessed due/not-due/reserved, process absent/before/at/multiple-due, action
  accepted/rejected/partial-phase-failure, duration active/completing/interrupted/restored-mid-duration, empty
  tick and event-ID collision, actor-known delta quiet/novel/hidden, and uninterrupted-vs-restored continuation
  — each entering through the same production constructor and typed command boundary as the TUI.
- **Mutation**: focused campaigns during implementation for fast feedback, then the configured standing campaign
  (§4.9); make in-diff retain its fast-feedback role and add the required full-surface trigger breadth (§4.10
  Layer C).
- **CI / governance**: wire the required public-boundary conformance lane and the full-surface mutation triggers;
  record the required-check names and branch-protection confirmation in the acceptance artifact. `include_str!`
  import/method guards remain labeled topology alarms only.

## 8. Acceptance artifact and evidence

On implementation, the session must: begin from a clean baseline; name its own exact implementation commit (not
this proposal's baseline `6495d7d`); run `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, and
`cargo test --workspace`; reproduce the preserved focused mutation commands from the `0049MUTWIT` /
`0050FOUCONSEC` / `0051FOUCONTHI` lines against a clean `6495d7d`-descendant tree; add and run focused campaigns
for each in-surface survivor (§5: #8–#23); run the full standing `cargo mutants` campaign **after** all code/test
work; and publish the selected denominator with the complete caught/missed/unviable/timeout disposition plus
shard/census fingerprints. Every executable claim is the implementing session's to produce — this spec asserts no
green/red command result. The acceptance artifact follows `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`,
records per-finding closure with real production-path evidence (production constructor, public command, observed
effect, sensitivity proof), records the required-check names and branch-protection confirmation, and must not call
the perimeter green before the standing campaign completes with zero in-surface misses/timeouts and the
food-source family resolved or honestly reported. Doc-truthing (§6.1) lands only after the executable witnesses
exist.

## 9. Implementation constraints

- Follow the driver's recommended **closure order** (report §8): (1) make the wrong production bootstrap
  unrepresentable; (2) close the command boundary; (3) internalize all client orchestration; (4) implement
  replay-critical runtime authority reconstruction; (5) replace the diagnostic process marker with a real process
  transaction (or honestly demote it); (6) complete the actor disposition census; (7) split embodied/debug
  temporal products and repair normal `continue`; (8) strengthen the production-boundary corpus and negative
  fixtures; (9) dispose all 23 historical survivors and run focused campaigns; (10) run the complete canonical
  standing campaign/reconciliation to green; (11) install/verify required CI and branch-protection checks; (12)
  only then truth live docs and produce acceptance evidence. Writing tests around the present public ownership
  shape before replacing it would harden the wrong API.
- No backwards-compatibility shim or public alias path in new work; a temporary internal adapter may exist only to
  migrate core tests and must be removed before closeout.
- Core must not depend on content or tui; the opaque session owner lives in `tracewake-core`.
- Worktree discipline: if implemented in a worktree, all paths resolve against the worktree root.
- Decompose into one ticket per reviewable diff (the closure-order steps are the natural ticket boundaries); the
  reassess/decomposition step determines the ticket prefix continuing the sibling convention
  (`0048FOUCONHAR` → `0050FOUCONSEC` → `0051FOUCONTHI` → a `0052`-keyed successor such as `0052FOUCONFOU`).

## 10. Risks and open questions

These are implementation choices inside settled doctrine; none blocks the determination (driver §9):

1. **Opaque bootstrap shape** — content returns a `LoadedWorldPackage`, a core constructor token, or a
   crate-private conversion module. Non-negotiable: the TUI cannot inject scheduler authority.
2. **Command granularity** — one command enum vs. opaque command constructors/facade methods. Non-negotiable:
   one core dispatcher with no client transaction-policy boolean.
3. **Time consumption for non-wait actions** — action-specific semantics may remain; non-negotiable: core owns the
   rule and event choreography.
4. **Replay authority representation** — event-derived scheduler projection, ancestry-preserving snapshot, or a
   combination. Non-negotiable: exact continuation and fail-closed missing authority.
5. **Minimal real declared process** — which existing content/process effect best supplies a bounded causal
   witness. Non-negotiable: process-specific effect and ancestry, not a generic marker; the §4.5 alternative
   (honest diagnostic demotion) is the fallback only if a real bounded effect proves infeasible this pass — chosen
   path stated in the acceptance artifact.
6. **Actor disposition taxonomy** — exact variant names and whether some are debug-only. Non-negotiable: one
   closed disposition per loaded actor without hidden-truth leakage.
7. **Embodied stop vocabulary** — qualitative actor-legible labels and provenance. Non-negotiable: no exact hidden
   tick/frontier/control reason unless modeled as holder-known.
8. **Full mutation trigger policy** — every PR vs. path-sensitive full runs plus a merge queue. Non-negotiable:
   witness-only / config / CI changes cannot weaken old mutation sensitivity and still merge.
9. **Repository-governance recording** — where branch-protection required-check confirmation is captured. It must
   be operational evidence, not a new gate identifier.

Open recurrence risk: this is the **fourth** consecutive pass to find critical divergence on this seam after a
remediation line claimed closure. `0051` proved that aggregate ownership and a closed command *token* are not the
same as authority closure. The §4.1/§4.10 keystone — type-level unrepresentability of client write authority,
bound to a required production-boundary CI lane and a green standing mutation perimeter — is the structural bet
that breaks the cycle; if the implementation preserves any public mutable-aggregate path, injectable constructor,
client transaction-policy boolean, forgeable embodied product (including via a public feature), or non-green
standing floor, the seam will reopen for a fifth pass.

## 11. Invariants alignment

| Invariant(s) | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-004, INV-005, INV-087, INV-091, INV-094, INV-108 | aligns | §4.2 makes the production bootstrap derive loaded actors/processes through the opaque core constructor, so the TUI runs the same autonomous loaded world under possession with no injectable empty scheduler. |
| INV-009, INV-018, INV-043, INV-069, INV-098 | aligns | §4.3 replaces the client boolean/raw-method route with a closed core command boundary and sealed receipts, so no client owns transaction choreography or proposal-ordering authority. |
| INV-001, INV-009, INV-010, INV-018, INV-092 | aligns | §4.4 reconstructs replay-critical scheduler authority (sequence, actor opportunity, process cadence/source/random) and fails closed, ending the default-reseed; continuation-equivalence proves it. |
| INV-001, INV-009, INV-010, INV-088 | aligns | §4.5 makes a due declared process emit process-specific validated events through the atomic cutover with ancestry; `world_processes_applied` counts only committed transactions, ending the counted `WorldNoOp` marker. |
| INV-004, INV-005, INV-041, INV-043, INV-094, INV-105 | aligns | §4.6's closed census gives each loaded actor exactly one disposition derived from the runtime-owned set, with inspectable diagnostics and possession parity. |
| INV-067, INV-068, INV-069, INV-093, INV-112 | aligns | §4.7/§4.8 move all authoritative mutation behind the opaque session and split sealed embodied/debug products in every feature combination, so normal output cannot leak exact replay/control time and the TUI cannot write state. |
| INV-092, INV-093, INV-094, INV-098 | aligns | §4.9/§4.10 close in-surface survivors and make a green standing mutation perimeter plus a required public-boundary lane an enforced merge property; measured evidence replaces overclaimed closure. |
| (all above) | N/A — no amendment | §6: doctrine already requires every property; no invariant is weakened, minted, or redefined. |
