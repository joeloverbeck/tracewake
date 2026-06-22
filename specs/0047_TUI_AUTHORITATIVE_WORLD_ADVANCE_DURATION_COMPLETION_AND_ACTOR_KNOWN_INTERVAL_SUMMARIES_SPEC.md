# 0047 TUI Authoritative World Advance, Duration Completion, and Actor-Known Interval Summaries Spec

**Status**: PROPOSED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and is promoted to `archive/specs/` on acceptance; it is
never promoted to the live `docs/4-specs/` tier. It is **not** a certification audit, a
mutation-remediation pass, a Phase-4 entry claim, or a latest-`main` certification. It does
not amend constitutional invariants, define gate semantics, or weaken execution gates. It
uses the canonical hardening-spec house structure of its sibling specs (e.g. `0025`, `0046`),
not the `docs/NN_*` narrative-document style.

## 0. Baseline statement and source discipline

- **Driver.** `reports/tui-human-wait-runs-simulation-research-report.md`, a recommendation-altitude
  consolidated analysis report on the human TUI time-control wiring gap. The report is the
  originating analysis; it is not itself doctrine and minted no spec, invariant, risk identifier,
  gate code, or glossary term. Its seed is the in-repo defect report
  `reports/tui-human-wait-runs-simulation-issue.md`.
- **Report target commit.** The report was conducted against
  `6e91da79b81bf9cbf25acba726e58183920dc441`, which is the current repository `HEAD` at authoring
  time. Every load-bearing code claim cited below was re-verified against that working tree (see §3).
  The named symbols are authoritative; any line numbers in the source report or seed are not relied
  upon.
- **Source discipline.** A commit hash named in this spec is audit/provenance only. This spec
  recommends and scopes work; it does not declare latest-main certification, Phase-4 entry, or
  second-proof entry. When executed, the implementation must name its own exact implementation
  commit, not assume this baseline commit is latest `main`. A manifest is path inventory only;
  branch/default-branch/code-search evidence is not proof of exact-commit content.
- **Authority posture.** This spec is subordinate to `docs/0-foundation/`, `docs/1-architecture/`,
  `docs/2-execution/`, and `docs/3-reference/`, in that order. It operationalizes existing doctrine;
  where it identifies doctrine that must be sharpened to admit the capability, it routes the
  amendment to the owning tier (§5) as *substance + home* and does not ratify wording or mint
  identifiers.
- **Execution admissibility posture.** `P0-CERT not applicable`. This spec certifies no
  certification gate; it names gate codes and `FIRST-PROOF-CERT passed` only as cross-references to
  existing artifacts, never as local definitions, and makes no certification claim of its own.
- **Ledger timing.** Per the sibling-spec convention, this package receives its
  `docs/4-specs/SPEC_LEDGER.md` archived-row entry at acceptance/closeout, not at proposal. This
  spec authors no ledger row now and makes no change to live `0001` or the ledger.

## 1. Scope

### 1.1 In scope

The defect: in human play, `TuiApp::submit_entry` runs one action-pipeline transaction and adopts
the last emitted event tick; `UiCommand::WaitOneTick` merely selects the ordinary `wait` action;
`DeterministicScheduler::advance_one_tick` only increments a counter. No human path invokes an
authoritative world step, so a human-started `sleep` or `work_block` never reaches its terminal —
fatigue cannot be recovered by sleeping and work blocks never complete. Foundation `08` already
states the controlling doctrine ("Waiting runs the simulation"); the implementation is below it.

This spec scopes the single committed solution and its proof:

1. **One kernel-owned authoritative one-world-tick coordinator** in `tracewake-core`. It accepts a
   typed world-advance request, executes exactly one canonical tick across the loaded simulation in
   an atomic-with-respect-to-acceptance transaction boundary, and returns a typed step result.
   Human `wait`, duration continuation, no-human progression, and any future acceleration all loop
   this same primitive.
2. **Full authoritative loaded-world tick semantics.** A human one-tick `wait` advances every loaded
   actor and due world process under one transition from tick `t` to `t+1`, governed by deterministic
   cadence/due predicates — not a private possessed-actor clock and not a full-planner-every-actor
   sweep. Possession supplies one actor's input slot; it neither freezes other actors nor defines a
   local timeline.
3. **Log-derived open-duration discovery and completion.** Due-completion discovery is rebuilt from
   the shared event-log open-duration authority (`open_body_exclusive_starts` / `is_duration_terminal`),
   not from batch-local pending-start vectors. Sleep/work terminals and authorized early interruption
   close through the existing terminal builders with shared keying; duplicate/orphan terminals fail
   closed.
4. **Unified single-charge-per-tick accounting.** The coordinator owns one per-tick reconciliation
   across action-emitted, passive, and duration-regime effects, asserting exactly one applicable
   `(actor, need, tick)` classification. A free actor's `wait` keeps its existing `ActorWaited` event
   and sealed reason; the world step recognizes that evidence rather than re-charging it.
5. **General body-exclusive reservation enforcement.** While an actor has an unresolved body-exclusive
   start, every ordinary embodied action for that actor (including ordinary `wait`) is rejected with
   the existing reservation-conflict report. Only typed lifecycle controls (continue time, pause, and
   any modeled interruption/cancellation) are permitted. The rule applies identically to human and
   autonomous proposal origins.
6. **Replay-visible tick ancestry.** Each executed world step appends an authoritative tick-boundary
   marker (operationalizing the already-present-but-unemitted `TimeAdvanced` event kind, subject to a
   narrow semantic audit) so even otherwise-empty ticks rebuild to the same temporal frontier.
   Acceleration is repeated one-tick markers, never a single `t`→`t+n` jump. The marker is
   world/replay evidence, never actor knowledge.
7. **An actor-known interval-summary projection.** A typed projection constructed positively from the
   possessed actor's holder-known frontier delta, own embodied effects, modeled
   messages/records/observations, and fresh resumption perception — exposed through `EmbodiedViewModel`
   under spec-0046 Hop-2 exhaustive renderer disposition. It is structurally separate from the exact
   core/debug step report; no constructor accepts raw world state, global event slices, hidden due
   queues, or the debug report.
8. **A one-tick `wait` plus an advance-until controller.** `wait` remains a one-tick command when the
   actor is free. A separate typed continuation/advance-until control repeats canonical ticks and
   stops at the first deterministic stop condition (possessed duration terminal; actor-known salient
   observation/interruption; user pause/cancel before the next tick; configured deterministic safety
   bound or kernel error).
9. **No-human runner refactor.** `run_no_human_day` and `advance_no_human` are refactored to loop the
   shared coordinator while preserving their day windows, progress metrics, markers, reports, and
   existing acceptance behavior. They no longer own a separate definition of time progression or a
   second pending-duration authority.
10. **The doctrine amendments** at the legitimate tier homes the report identifies (§5), routed as
    substance + home for the repository's reassess/amend process.
11. **The spec-0046 PAR-010/PAR-011 parity extension** — time-control setup operations, capability
    entries/dispositions, Hop-1 real-pipeline goldens, Hop-2 view-model/render guards, anti-leak
    adversarial cases, replay/no-human dispositions, and ordinary CI integration — extended to
    duration *completion* and time-control summaries rather than a side channel.

### 1.2 Out of scope (non-goals)

- Regional/unloaded-world LOD simulation or how coarsened regions participate in the temporal frontier.
- Generalized high-speed time compaction or storage compaction for very long advances. Any later
  optimization must preserve intermediate causality, per-tick accounting, and replay ancestry.
- New Phase-4 institutions, notices, travel, or regional behavior beyond whatever existing modeled
  channels already deliver.
- Free-form narration or LLM summarization of the interval; a "slept until morning" prose-only summary
  with no typed projection is forbidden.
- Promoting the debug no-human-day runner into gameplay, or any TUI-owned simulation rule, action-kind
  switch, direct event application, or direct call to completion/terminal builders.
- Minting a new constitutional invariant, risk identifier, certification gate code, or glossary term;
  reopening any passed certification; amending any archived spec.
- Any `SPEC_LEDGER.md` declaration or rewrite of live `0001` by this proposal.
- Backwards-compatibility aliases or shims for the incorrect time-control path.

## 2. Doctrine anchors

This capability is governed by existing doctrine; no invariant is amended (see §3, §10).

- **Foundation.** `08` Time controls ("Waiting runs the simulation"; staged sleep/continuation/
  acceleration controls; actor-known sleep summaries with no omniscient leakage); `03` causal event
  trace, replay determinism, and temporal authority (`INV-112`); `06` sleep/work as first-class
  time-spanning actions and causal survival/recovery (`INV-045`); `05` needs/charging doctrine; `12`
  human-driven ordinary day in first-playable scope; `14` actor-known cognition and truth firewall.
- **Architecture.** `10` possession/TUI play loop (world-advancing controls are commands that advance
  authoritative event/replay time through the ordinary pipeline); `04` action proposal/validation/
  scheduling/no-direct-dispatch; `02` event log/replay/projection determinism; `05` actor decision/
  need-accounting seam; `00` conformance rows (`0017` tick-charge attribution authority; `0017` shared
  open-duration authority); `03` holder-known contexts/truth firewall; `13` validation/acceptance
  artifacts.
- **Execution.** `05` transaction/scheduler/pipeline/no-direct-dispatch; `06` ordinary-life needs/
  routines/no-human proof; `07` epistemic view models/possession/debug proof; `10` testing/
  observability/evidence honesty; `02`/`03` baseline + certification-ladder posture
  (post-`FIRST-PROOF-CERT`); `04` truth-firewall/anti-contamination gates.
- **Reference.** `00` review checklist; `01` design risk register.
- **Precedent.** `archive/specs/0046_…PARITY_CONTRACT…` (the two-hop playable-capability parity
  contract, capability registry, conformance runner, and standing PAR-010/PAR-011 per-feature
  obligation) and `archive/reports/0046-parity-acceptance-artifact.md` (21-capability baseline). This
  spec **extends** that contract; it does not replace or weaken it.

## 3. Determination

The seam map was re-verified against `HEAD` = `6e91da7`. Named symbols are authoritative.

### 3.1 Verified holdings (no action; recorded so they are not re-litigated)

- **The duration model is substantial and correct.** `build_sleep_start_event` records the
  body-exclusive reservation, recovery rate, and expected completion tick; work start records a
  scheduled body-exclusive block; completion builders emit typed terminals and use
  `classify_actor_tick_regimes_with_start` to derive elapsed sleep/work effects.
  `open_body_exclusive_starts` scans authoritative log evidence and fails closed on duplicate terminal
  evidence; `is_duration_terminal` (in `events/envelope.rs`) centralizes terminal classification. No
  new duration model is needed.
- **`build_wait_events` is correct.** Wait is an instantaneous `ActorWaited` action with awake need
  deltas for its tick range; it is not an open duration and must remain so.
- **The no-human runner already orchestrates time.** `run_no_human_day` walks ticks, performs passive
  accounting, fires `append_due_completions`, records perception, and runs actor transactions. Its
  completion engine is more factored than the seed suggested, but it is not yet a general step
  primitive: `append_due_completions` drains pending-start vectors local to its enclosing run, so a
  `SleepStarted` from a prior TUI transaction is invisible to it.
- **The `TimeAdvanced` event kind already exists** in the `EventKind` enum and the envelope
  classifiers, but the inspected target code does not emit it.
- **The single-charge fixtures encode the contract to preserve:**
  `wait_then_window_passive_charges_each_tick_once_001`,
  `sleep_spanning_window_boundary_charges_each_tick_once_001`, and
  `scheduler_cannot_rewrite_wait_reason_after_transaction_001`.
- **spec-0046 covers starts and surfaces, not completion.** Its 21-capability census, real-pipeline
  runner, typed/actor-known/rendered/anti-leak/replay dispositions, and Hop-1/Hop-2 guards exist, but
  no operation expresses "advance one human world tick," "continue this human-started duration," or
  "stop on actor-known interruption." `RunNoHumanDay` is not an acceptable witness for the human path.

### 3.2 Validated — no action

- **No constitutional amendment is required.** Existing invariants already require one authoritative
  event-sourced world, deterministic replay (`INV-018`/`INV-092`), causal survival (`INV-045`),
  no-human behavior (`INV-091`), possession parity (`INV-005`/`INV-006`/`INV-094`/`INV-108`),
  subjective knowledge (`INV-067`), the truth firewall (`INV-099`/`INV-101`/`INV-102`), no direct
  dispatch (`INV-103`/`INV-104`), event cause models (`INV-010`), and temporal authority (`INV-112`).
  The capability conforms by using those rules; it does not need an exception. The defect is a
  lower-tier operational gap, not a constitutional divergence.
- **Glossary and acceptance template need no change.** Existing terms are sufficient; the acceptance
  template's parity/evidence sections can package this feature once §4 defines concrete witnesses.

## 4. Findings and remediation requirements

The committed solution's components are mutually necessary: a full-world step without shared duration
discovery cannot resume a human-started sleep; shared completion without unified accounting risks
double-charge; correct accounting without reservation enforcement permits action overlap; correct
mutation without an actor-known projection makes embodied time controls opaque or omniscient; and any
TUI-local implementation violates the pipeline and parity contracts.

### 4.1 Core one-tick coordinator (seam: `scheduler.rs`)

Extract a reusable, kernel-owned authoritative one-tick coordinator. It accepts a typed world-advance
request, executes exactly one canonical tick, and returns a typed step result. The canonical step must
perform, in one owned transaction boundary with a single frozen deterministic intra-tick phase order:
frontier validation (reject a stale/mismatched expected tick); tick-ancestry append; log-derived
open-duration discovery; duration lifecycle (due terminals + authorized interruption through existing
builders); actor dispositions (possessed actor's ordinary action when free + due autonomous proposals,
all through the ordinary pipeline); reservation enforcement; unified need accounting; due world
processes; perception/epistemics; projection/replay finish. The step is atomic with respect to
acceptance — either it appends a coherent ordered event set and updates projections, or it returns a
fail-closed error with no half-applied tick. The existing counter-only `advance_one_tick` becomes the
private clock increment inside the coordinator or is renamed so it cannot be mistaken for a complete
world step; public callers must not advance the authoritative frontier by mutating that counter alone.

### 4.2 Shared log-derived open-duration discovery (seams: `scheduler.rs`, `need_accounting.rs`, `events/envelope.rs`)

Replace or wrap batch-local pending-vector completion discovery with a query/projection over
unresolved body-exclusive starts derived from the shared log authority (rebuildable from the same log),
preserving deterministic ordering and duplicate/orphan fail-closed behavior. Read each start's expected
terminal tick and duration kind from its typed payload; deterministically order due candidates by
existing ordering authorities; invoke the existing sleep/work completion builders; append terminals
through the scheduler/pipeline-owned event seam. A performance index, if later added, must be a derived
cache, never a second source of truth. Replay must reconstruct the same open-duration set without
hidden scheduler memory.

### 4.3 Unified per-tick need accounting (seams: `wait.rs`, need-accounting seam)

The coordinator owns one per-tick reconciliation across action-emitted, passive, and duration-regime
evidence, classifying every `(actor, need, tick)` with the shared awake/asleep/working authority and
emitting only missing effects through the single-owner action-pipeline/ordinary-life seam. It must not
implement "submit wait, then independently apply a world tick" (which double-charges) or treat an actor
as both awake-waiting and asleep across a tick. The accepted `ActorWaited` event and sealed reason for a
free actor's one-tick wait are preserved and recognized, not replaced by an unrelated passive event. If
existing structure cannot express this ordering, introduce a typed tick-accounting plan inside core; do
not move arithmetic into TUI code or add a "human wait already charged" boolean.

### 4.4 General body-exclusive reservation enforcement (seam: `actions/pipeline.rs`)

The current reservation-conflict predicate only derives an actor from a candidate that itself starts a
new body-exclusive duration, so sleep-then-wait is wrongly accepted. Generalize it: if an actor has an
unresolved body-exclusive start at the proposal tick, reject every ordinary embodied action for that
actor with the existing reservation-conflict report; permit only typed continuation/lifecycle controls.
A continuation control is not an `ActorWaited` event and creates no competing reservation. When an
interruption is modeled, close the existing duration through a terminal before any new ordinary action
is accepted. Apply the rule identically to human and autonomous origins. This is part of this spec, not
a follow-up: continuation semantics are incoherent while an overlapping `wait` remains accepted.

### 4.5 Replay-visible tick ancestry (seams: `scheduler.rs`, `events/envelope.rs`, replay)

Operationalize the existing `TimeAdvanced` event kind as the canonical tick-boundary evidence (subject
to a narrow semantic audit in implementation; use an equivalent already-ratified temporal event only if
the audit proves a conflicting intended meaning). The audit must also reconcile the existing envelope
classification of `TimeAdvanced` (in `events/envelope.rs`: today `cause_required` is `false`, the kind
maps to `EventStream::World`, and it is marked physically mutating) with §10's `INV-010` stance that the
marker carries a cause model (controller/process origin and ordering ancestry). The implementation must
record an explicit decision — either keep the kind cause-exempt with a stated rationale, or move it into
the cause-required set and supply the cause payload — so the classifier and the `INV-010` alignment do
not silently diverge; this is part of decomposition ticket 2. Each executed world step appends one marker
carrying prior/resulting tick, controller/process origin as non-epistemic scheduling metadata, ordering
ancestry sufficient to reproduce the step, and causal links to the time-control request where applicable. The
marker is world/replay evidence, never actor knowledge, and must not enter holder-known projections.
Acceleration remains repeated one-tick markers. Replay must rebuild final physical/agent state,
temporal frontier, duration open/closed state, need values and counted tick ledger, work output and
sleep recovery, epistemic projection, and the possessed actor's interval summary and stop reason.

### 4.6 TUI world-advance wiring (seams: `tracewake-tui/src/app.rs`, `run.rs`)

Add a typed `TuiApp` world-step entry point and route `UiCommand::WaitOneTick` through it. A free
actor's `wait` remains an ordinary semantic proposal, but an accepted wait now advances the full world
through the coordinator. The TUI submits a typed request and never calls `run_no_human_day`, terminal
builders, or direct state/event mutation. Add a clearly separate typed continuation/advance-until
control rather than overloading `wait` to sometimes mean "stay asleep"; the typed distinction must
survive parsing even if renderer text keeps the common case convenient.

### 4.7 Advance-until controller and actor-known interval projection (seams: `view_models.rs`, `projections.rs`, `render.rs`)

The advance-until controller is a pure loop over deterministic step results: execute a canonical tick,
project holder-known changes, then evaluate the deterministic stop set against typed terminals and
actor-known salience — never against a hidden due queue. Add a typed actor-known interval-summary
surface, derived positively from the actor's holder-known/source-bearing frontier delta plus own
embodied events and resumption perception. It may carry: own action/duration outcome, own bodily/need
change at existing embodied precision, direct perception at resume, modeled messages/notices/records/
testimony/public cues with provenance, perceived sounds/interruptions, and an actor-visible stop reason.
It must exclude: other actors' unobserved private actions/states, hidden failure/interruption reasons,
global event counts/due queues/scheduler phase data, claims that an unobserved place/service/item/person
did not change, and the omniscient "nothing happened." When nothing reached the actor, the typed result
is equivalent to "no new actor-known notices or observations are available," not "nothing happened";
the wording is renderer-owned, the distinction typed. The preferred surface is an explicit optional
field on `EmbodiedViewModel` (forcing Hop-2 exhaustive disposition in `render.rs`), replaced atomically
on the next completed advance. Keep two structurally separate outputs: an exact core/debug step report
(ticks, appended event IDs, due work, hidden stop diagnostics, replay checks) and the actor-known
interval projection whose constructors accept only holder-known/source-bearing inputs. A persisted
summary item must already have modeled observation/record/memory ancestry; rendering prose creates no
fact.

### 4.8 spec-0046 parity extension (seam: spec-0046 parity tests + runner)

Extend the capability registry, scenario runner, and dispositions (do not replace or weaken spec-0046)
with real TUI/core operations for one world step and advance-until, plus capability entries for at
least: human wait advances one authoritative world tick; human-started sleep reaches a terminal with
correct recovery; human-started work reaches a terminal with output or modeled failure; an open
body-exclusive duration rejects ordinary wait/action but accepts typed continuation; an actor-known
interval summary renders positive source-bearing information and suppresses hidden world activity; and
advance-until stops on an actor-known salient interruption without leaking a hidden cause. Each carries
a typed event/source witness, actor-known positive and negative/anti-leak witnesses, a rendered golden
or explicit no-render disposition, a replay/rebuild witness, a no-human/possession-parity disposition,
and a Hop-2 field/destructure disposition where the view model changes. Add a differential scenario:
from an identical initial log and deterministic inputs, advance one tick under a human-wait disposition
and under the equivalent no-human/controller disposition; with the actor choice held equal, the
authoritative physical transition, duration-completion set, need ledger, and replay checksum match —
only controller-origin metadata and the actor-facing command report may differ.

### 4.9 Implementation decomposition (one ticket per reviewable diff)

A non-binding outline for the later `spec-to-tickets` pass; the decomposition may be reordered after
dependency validation but should preserve separation of concerns:

1. Authority-aligned documentation amendments (§5) — establish approved semantics before code.
2. Core one-tick type boundary + `TimeAdvanced` marker; prove an empty tick rebuilds to the same frontier.
3. Shared log-derived open-duration discovery; resume-from-pre-existing-start unit tests.
4. Canonical duration completion/interruption phase through existing builders; isolated terminal/
   interruption/reservation-closure tests.
5. Unified per-tick need accounting; preserve existing fixtures + add double-charge adversarial cases.
6. General body-exclusive reservation enforcement + continuation exemption plumbing; human/autonomous tests.
7. Refactor `run_no_human_day` / `advance_no_human` onto the coordinator; remove duplicate pending authority.
8. TUI one-tick world advance; route `WaitOneTick`; prohibit direct mutation/debug-runner reuse.
9. Duration continuation + advance-until controller with full stop set; command-loop tests.
10. Actor-known interval projection + debug split; integrate into embodied view; positive/negative epistemic tests.
11. Human sleep completion real-pipeline witness (start → reject ordinary wait → continue → terminal →
    recovery/hunger accounting → actor-known summary → replay/rebuild).
12. Human work completion real-pipeline witness (output or modeled failure; interruption case if supported).
13. Full-world/possession/anti-leak differential suite.
14. spec-0046 parity registry + two-hop closure under PAR-010/PAR-011 + CI.
15. Acceptance artifact + ledger routing at the exact implementation/evidence commit.

## 5. Doctrine amendments

Substance + home only — **no** paste-ready wording, **no** invented `INV-###`/gate/risk/glossary
identifiers, **no** new doctrine that weakens an upstream tier. Each lands through the repository's
reassess/amend process before or alongside code (decomposition ticket 1).

- **Foundation `08` — amend the Time controls section.** Own: an embodied wait/step advances the
  authoritative loaded world, not a private possessed-actor clock; possession supplies one actor's
  input but does not pause other actors/processes; a one-tick actor wait and a controller-level
  continuation/acceleration are distinct; sleep/work remain causal durations and acceleration is
  repeated authoritative progression; stopping and interval summaries are based on perceived/
  holder-known information; sleeping/absent actors receive no global event diff or omniscient negative
  summary. Do not specify Rust types, scheduler phases, event payloads, or test names.
- **Foundation `03`, `05`, `06`, `12`, `14` — no substantive amendment.** Add cross-references only if
  the reassess process finds a discoverability gap.
- **Architecture `10` — amend** the temporal-rendering / embodied-play-loop section: the typed
  world-control → core world-step boundary (TUI asks core to advance, never mutates), repeated one-tick
  acceleration, and the actor-known / debug report split.
- **Architecture `04` — amend:** a single authoritative world-step coordinator; deterministic due-work
  ordering; ordinary proposal routing for human and autonomous actors; log-derived open-duration
  discovery; general body-exclusive conflict; explicit continuation controls that do not masquerade as
  ordinary actor actions.
- **Architecture `02` — amend:** replay-visible tick-boundary evidence for otherwise-empty steps,
  repeated-step acceleration, rebuild of the temporal frontier, and deterministic interval projection.
- **Architecture `03` — amend:** interval summaries as positively constructed source-bearing
  holder-known frontier deltas; prohibit raw-world-diff redaction as the embodied boundary.
- **Architecture `00` — correct/extend conformance rows:** name human world advance as a consumer of
  the existing `0017` tick-charge attribution and shared-open-duration authorities, and name
  time-control/interval-summary parity under the existing parity posture. Do not mint a new conformance
  family unless reassessment proves the existing rows cannot express the obligation.
- **Architecture `13` — add proof obligations:** one-tick human/no-human differential, duration-terminal
  and accounting witnesses, replay/no-op-tick evidence, actor-known summary anti-leak evidence,
  reservation conflict, and the spec-0046 parity extension.
- **Execution `05` — amend:** the canonical step's concrete ownership, typed request/result,
  deterministic phase contract, event append/application route, due-duration scan, actor-transaction
  cadence, and no-direct-dispatch guard.
- **Execution `06` — amend:** extend ordinary-life proof to human-driven ticks — existing wait charge
  evidence, passive suppression, duration-regime classification, open-duration completion, and
  human/no-human equivalence.
- **Execution `07` — correct** the staging sentence that says embodied time controls remain unavailable
  outside debug: preserve the debug no-human-day distinction, but replace the "not exposed outside
  debug" posture once this feature is accepted, and add embodied one-tick/advance-until proof,
  actor-known stop reasons, interval-summary anti-leak rows, and structurally separate debug diagnostics.
- **Execution `10` — add:** command transcripts, typed event ledgers, checksums, replay reports,
  hidden-truth negative witnesses, and parity evidence expected from implementation and acceptance.
- **Reference `00` checklist — add concise prompts:** all time controls use the same world step; every
  `(actor, need, tick)` has one accounting classification; open durations close through shared authority;
  interval output is positive actor-known evidence, not redacted omniscience.
- **Reference `01` risk register — update existing entries; mint no new ID.** This feature is a new
  mitigation/test surface for pipeline bypass, epistemic leakage, prose authority, debug leakage,
  replay/projection erosion, TUI playability erosion, no-human ordinary-life failure, and the
  temporal-control relapse cluster.
- **Glossary `02` and acceptance template `0003` — no change.**

## 6. Required fixtures and tests

The implementation must, at minimum:

- Preserve the existing single-charge *scenario/content* fixtures and their single-charge assertions
  unchanged, and add adversarial combinations that would double-charge or double-recover under a two-pass
  "wait then scheduler" implementation. This "unchanged" obligation is on the scenario definitions and
  charge assertions, not on golden event-log traces or replay checksums: once the coordinator emits the
  `TimeAdvanced` marker every tick and `run_no_human_day`/`advance_no_human` loop it (§1.9), affected
  no-human golden traces and replay checksums will legitimately change and must be regenerated under
  review as an intentional update. The marker must never be suppressed on the no-human path to keep a
  golden green — that would break the single semantic definition of a tick (§8).
- Add a unit test that resumes completion of a duration started in a prior, independent transaction
  (no batch-local pending vector available).
- Add isolated terminal, authorized-interruption, and reservation-closure tests with no TUI changes.
- Add a sleep-then-ordinary-`wait` rejection test (expect `ReservationConflict`) and a
  sleep-then-continuation success test (world advances, no `ActorWaited` for the sleeping actor), for
  both human and autonomous origins.
- Add a replay/rebuild test for an otherwise-empty `TimeAdvanced` tick and a mid-duration
  save/rebuild/resume test that does not depend on an in-memory pending queue.
- Add positive (source-chain present) and negative (hidden other-actor event leaves the embodied
  summary unchanged) epistemic tests for the interval projection, plus a "no new actor-known
  notices/observations" case distinct from "nothing happened."
- Add the spec-0046 parity capability entries, real-pipeline goldens, Hop-2 guards, anti-leak
  adversarial cases, replay/no-human dispositions, and the human/no-human one-tick differential, all in
  ordinary CI.
- Real-pipeline human sleep and work completion witnesses through the actual parser/app/core path.

All evidence must be real-pipeline (no synthetic event insertion); `RunNoHumanDay` is not a valid
witness for the human path.

## 7. Acceptance artifact and evidence

On execution, package an acceptance artifact against the existing `0003` template at an exact
implementation/evidence commit, with: command transcripts; typed event ledgers; replay checksums and
rebuild reports (including the interval summary and stop reason); the parity census extension and
PAR-010/PAR-011 dispositions; hidden-truth negative witnesses; the human/no-human differential result;
and explicit scope limitations (not latest-main, not Phase-4 entry, not second-proof). Update
`SPEC_LEDGER.md` with the archived row **only at acceptance/closeout**, with no latest-main overclaim,
and move this spec to `archive/specs/`.

Recommended review gates (no new gate identifiers) — reviewers answer yes to all before acceptance:
exactly one semantic definition of a loaded-world tick; a duration started before the coordinator
invocation can be found and completed after rebuild; ordinary human wait produces exactly one charge
classification per affected need/tick; sleeping/working continuation produces no `ActorWaited` for that
actor; unpossessed actors and due processes advance under the same timeline; replay reconstructs the
temporal frontier even for ticks with no other emitted effect; the embodied interval summary is
constructible without raw physical state or a global event slice; a hidden other-actor event leaves the
embodied summary unchanged unless modeled information reaches the actor; debug-only exact details remain
structurally unavailable to embodied rendering/planning; human/no-human/replay/parity witnesses are all
real-pipeline; every changed playable capability or view-model field receives a spec-0046 surface
disposition.

## 8. Implementation constraints

- One semantic definition of a loaded-world tick, owned in `tracewake-core`. The no-human runner loops
  it; it does not own a parallel definition.
- The TUI sends typed requests only. It never applies events, mutates state, calls terminal/completion
  builders, or reuses the debug runner for gameplay (`INV-103`/`INV-104`; architecture `10`).
- No second source of truth for open durations: log-derived discovery is authoritative; any index is a
  derived cache.
- Prefer obvious replay correctness over premature batching; acceleration is repeated one-tick markers.
- No backwards-compatibility aliases or shims for the incorrect path.
- Worktree discipline: if executed in a worktree, all paths resolve to the worktree root.
- This spec mints no invariant, gate code, risk ID, or glossary term and reopens no passed
  certification.

## 9. Risks and open questions

Resolved enough to route; the following are implementation decisions for the executing tickets, not
blockers:

1. **Exact intra-tick phase order.** Freeze a single shared order among tick marker, due-terminal
   evaluation, passive accounting, actor decisions, event application, perception, and projection only
   after compatibility tests against existing ordinary-life and replay fixtures. The order must be
   singular and shared; this spec does not invent an untested phase numbering.
2. **Early-interruption semantics per duration kind.** Enumerate which modeled conditions interrupt
   sleep/work before expected completion, at which phase they are detected, and which actor-visible
   reasons are safe; decide whether early interruption flows through the same terminal builder with an
   explicit terminal tick or a narrowly factored lifecycle helper. Either route yields one shared
   terminal and one counted tick expansion.
3. **Salience policy.** A typed policy for which observations stop acceleration, how multiple same-tick
   observations are summarized, and how actor preferences configure salience without changing world
   outcomes. Default conservative and source-bearing; hidden events never become salient because debug
   knows them.
4. **Interval-summary persistence and temporal precision.** Preferred initial design: one source-backed
   "last completed advance interval" field on the embodied projection, replaced on the next completed
   advance. Confirm acknowledgment/clearing behavior and whether exact tick labels are actor-legible
   per temporal-authority doctrine. Internal exact ticks remain available to replay/debug regardless of
   rendered precision; long-term persistence occurs only through existing memory/record mechanisms.
5. **Deterministic safety bound.** Loop fixed ticks with a deterministic maximum and report a
   controller-bound stop as a controller limitation, not in-world knowledge.
6. **Loaded-world boundary.** This spec covers the current authoritative loaded simulation; future
   regional/LOD work must define unloaded/coarsened participation in the same temporal frontier and is
   out of scope here.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-112` (temporal authority) | aligns | The coordinator makes authoritative time validate/order via `TimeAdvanced` ancestry and log-derived due effects; temporal facts reach the actor only through the holder-known interval projection @ scheduler/replay + embodied view. |
| `INV-018` / `INV-092` (deterministic replay) | aligns | Repeated one-tick markers + log-derived discovery give every tick replay-visible ancestry; rebuild reconstructs frontier, ledger, recovery, and interval summary with no TUI-local clock authority @ event log/replay. |
| `INV-045` (ordinary survival is causal) | aligns | Human-started sleep/work now reach terminals through existing builders, so fatigue recovers and work completes by elapsed causal ticks @ scheduler + ordinary-life accounting. |
| `INV-103` / `INV-104` (scheduler not cognition; no direct dispatch) | aligns | The possessed actor's action runs through the ordinary proposal pipeline; the TUI sends typed requests and never dispatches or mutates; the coordinator orders established seams rather than synthesizing proposals @ action pipeline + TUI boundary. |
| `INV-005` / `INV-006` / `INV-094` / `INV-108` (possession parity / cognition-neutral) | aligns | Possession supplies one actor's input slot only; a full-world tick advances unpossessed actors/processes identically under a held-equal human/no-human differential @ scheduler cadence + parity suite. |
| `INV-067` / `INV-099` / `INV-101` / `INV-102` (actor-known reality; truth firewall; sealed context; provenance) | aligns | The interval summary is built positively from source-bearing holder-known deltas with provenance and excludes hidden world activity; the exact step report is structurally separate and never enters embodied rendering @ holder-known projection + debug split. |
| `INV-010` (every event needs a cause model) | aligns | The `TimeAdvanced` marker carries controller/process origin and ordering ancestry; continuation/interruption close through typed terminals, not synthesized records @ event envelope. |
| `INV-091` (no-human tests mandatory) | aligns | The no-human runner is refactored onto the same coordinator and retains its proof; a human/no-human differential becomes a standing parity witness @ scheduler + parity suite. |

No invariant tensions. If implementation surfaces a genuine conflict, that is an invariants-amendment
question to raise before proceeding — not a silent divergence.

## Outcome

PROPOSED. This spec scopes a single committed solution to the human-TUI time-control wiring gap:
a kernel-owned authoritative one-world-tick coordinator looped by all time-advancing modes, full
loaded-world tick semantics, log-derived duration completion, unified single-charge accounting,
general body-exclusive reservation enforcement, `TimeAdvanced` replay ancestry, and an actor-known
interval-summary projection — extending (not bypassing) the spec-0046 parity contract, with doctrine
amendments routed as substance + home to their owning tiers and no constitutional change. It certifies
nothing on its own and receives its ledger row only at acceptance/closeout.
