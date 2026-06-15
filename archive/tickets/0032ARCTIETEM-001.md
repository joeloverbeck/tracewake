# 0032ARCTIETEM-001: A02 authoritative event/replay-time contract

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` (new authoritative event/replay-time contract near the event-envelope and replay sections). No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0032 §R-A (architecture tier-1 doctrine; not by convention).

## Problem

Spec 0032 D-T2 (report T2). Foundation `03` makes simulation time authoritative for event order, replay, validation, intervals, scheduled consequences, and causal explanation — but **not** automatically authoritative for cognition (`INV-112`). A02 owns the event envelope, replay rebuild, projections, snapshots, randomness, and deterministic metrics, and references `sim_tick`/`tick_window`, but it does **not** yet name temporal authority as a firewall problem: verified 0 temporal-authority / temporal-firewall / temporal-ancestry matches in A02 at `ea6a05b`. Without this contract a subsystem author cannot tell, from architecture alone, which component may use the authoritative clock for ordering/validation and how that authority is quarantined from cognition surfaces.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A02 (`02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`) owns the event envelope, replay, projections, snapshots, and randomness but carries no temporal-authority contract (`grep -niE "temporal authority|temporal firewall|temporal ancestry"` → 0). The gap is real; this ticket adds the contract, it does not restate foundation doctrine.
2. Verified against spec 0032 §3.1 D-T2 and source report §5 Finding T2 / foundation `03` "Temporal authority" section (`docs/0-foundation/03`, ratified by spec `0031`). D-T2's home is A02, near the envelope/replay sections. Additive; relaxes no existing A02 contract. Concrete mechanism (calendar/date syntax, tick size, duration units, priority-queue structure, stable field names) routes to execution/spec per spec §6.
3. Shared boundary under audit: A02 (authoritative event/replay time substrate) ↔ A03/A06 (holder-known temporal claims, sibling ticket 002). A02 owns the ordered substrate and the export-via-events rule; A03/A06 own what reached a holder. This ticket states only the substrate side and the one-way export obligation.
4. Constitutional invariant motivating this ticket, restated before trusting the narrative: `INV-112` — time may validate, but holder-known time must plan; the scheduler/replay clock may order and validate, but must not become cognition authority. D-T2 names the event/replay-time substrate as the validate/order half of that firewall.
5. Deterministic-replay surface (substrate-only; enforcement deferred to execution `10` and to sibling observability ticket 011): the contract requires projection/snapshot/compaction to **preserve temporal ancestry** rather than overwrite it with "current time" labels, and names replay diagnostics for temporal divergence (wrong ordering, missing duration terminals, due-effect drift, unrecorded wall-clock input, unsupported temporal migration). This is `INV-018` deterministic-replay doctrine applied to time; it introduces no nondeterministic input into canonical forms — wall-clock input is named as a divergence the diagnostics must catch, not an allowed canonical-form value.

## Architecture Check

1. A02 is the correct single home: it already owns event/replay/projection/snapshot ancestry, so the temporal-authority substrate is a specialization of contracts A02 already carries, not a new subsystem. Putting it anywhere else would split the ordered-substrate authority from the event log that owns it.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine; no behavior change, no compatibility layer.

## Verification Layers

1. `INV-112` event/replay-time-as-validate-not-plan → invariants alignment check: A02 distinguishes event/replay time as the ordered substrate for validation/scheduling/duration/replay/projection/causal-explanation from temporal facts exported to holders only through events/projections carrying modeled acquisition/record ancestry.
2. `INV-018` deterministic replay / temporal ancestry → invariants alignment check + codebase grep-proof: A02 requires projection/snapshot/compaction to preserve temporal ancestry (not "current time" labels) and names temporal-divergence replay diagnostics.
3. Single doc, additive: no replay/golden-fixture or skill-dry-run layer applies here (those are deferred execution `10` proof surfaces); the layers above map each engaged invariant to a distinct surface.

## What to Change

### 1. D-T2 — authoritative event/replay-time contract in A02

Add a compact authoritative event/replay-time contract to `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`, near the event-envelope and replay sections. Substance:

- event/replay time is the ordered substrate for validation, scheduling due effects, duration accounting, replay, projection rebuild, and causal explanation;
- temporal facts are exported to holders only through events/projections that carry modeled acquisition or record/procedure ancestry;
- projection/snapshot/compaction obligations preserve temporal ancestry rather than replace it with "current time" labels;
- replay diagnostics for temporal divergence: wrong ordering, missing duration terminals, due-effect drift, unrecorded wall-clock input, and unsupported temporal migration.

The doc names the seam and data-flow obligations only. It must not choose calendar/date syntax, tick size, duration units, priority-queue structure, or stable field names.

## Files to Touch

- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` (modify)

## Out of Scope

- **Calendar/date syntax, tick size, duration units, priority-queue / data structures, stable field names** — execution/implementation (spec §6).
- **Temporal-divergence replay proof procedures / negative fixtures** — execution `10` (spec §6, V4).
- **Holder-known temporal claim shape (A03/A06)** — sibling ticket 002.
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T2 landing grep** — A02 carries the event/replay-time contract: `grep -niE "event/replay time|temporal ancestry|temporal divergence" docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` resolves the contract.
2. **Mechanism-token boundary grep** — no execution mechanism token entered A02: a manual review confirms the added passage names no tick size, calendar syntax, duration unit, or queue structure.
3. **Invariants alignment review** — the contract upholds `INV-112` (validate/order, not plan) and `INV-018` (temporal ancestry preserved in replay/projection/snapshot); no invariant weakened.

### Invariants

1. Event/replay time is authoritative for ordering/validation/replay only; temporal facts reach holders solely via events/projections carrying modeled ancestry (`INV-112`).
2. Projection/snapshot/compaction preserve temporal ancestry and never substitute a raw "current time" label; temporal divergence is a named replay diagnostic (`INV-018`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing grep) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "event/replay time|temporal ancestry|temporal divergence" docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — confirms D-T2 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented D-T2 in `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` as a compact `Authoritative event/replay time` contract near the event-envelope section. The new architecture text names event/replay time as the ordered substrate for validation, scheduled due effects, duration accounting, replay, projection rebuild, and causal explanation; requires temporal facts to reach holder-facing surfaces only through modeled ancestry; requires projection/snapshot/compaction temporal ancestry preservation; and names temporal-divergence replay diagnostics.

The execution-blocking owner-approval precondition from spec 0032 §R-A is satisfied by the user's explicit `$ticket-series implement the series tickets/0032ARCTIETEM*` request for this architecture-tier amendment series.

Verification:

- `grep -niE "event/replay time|temporal ancestry|temporal divergence" docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- Manual mechanism-token boundary review of the added passage: no calendar/date syntax, tick size, duration unit, priority-queue structure, or stable field names were introduced.
- Manual invariants alignment review: the passage preserves `INV-112` by separating validation/ordering time from holder-known planning inputs, and preserves `INV-018` by requiring replay/projection/snapshot temporal ancestry rather than current-time replacement.

No crate/code or fixture changes were made for this documentation-only architecture ticket.
