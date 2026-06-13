# 0027ARCTIEDOC-005: Name the single-charge derived-accounting seam across A04/A05/A09 (+A00 pointer)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`, `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`, `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`, and a one-line index pointer in `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`. No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0027 §R-A (architecture tier-1 doctrine; not by convention).

## Problem

D6 (report A04/A05/A09-D). Event-sourced causality and deterministic replay (foundation doc 03; `INV-018`) condemn a tick/window being causally charged twice, but no primary architecture doc names the single-owner accounting seam for derived need-deltas and duration lifecycle — the rule lives only in A00 hardening rows (0016/0017/0022). A04 owns reservations/durations, A05 owns actor cognition (needs are pressures), and A09 owns ordinary food/sleep/work/economy, but none states the compact single-owner contract a future subsystem author can apply without reading A00's conformance table (verified: 0 `single-charge`/`double-charge`/`accounting seam` matches in A04/A09 at `fdfd0b9`). This ticket names the seam in A04/A05/A09 and adds a one-line A00 index pointer.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`fdfd0b9`): A04 (`04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`) owns proposal ancestry, scheduler limits, validation authority, reservations/durations (schedulers may complete due durations whose start event reserved the resource); A05 (`05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`) says needs are pressures, a need cannot name the true target; A09 (`09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`) says hunger produces pressure, sleep uses body-exclusive reservations, work uses reservations + fatigue/hunger, economy is event-sourced/actor-known. None names a single-owner accounting seam (`grep -ciE "single-charge|double-charge|accounting seam"` → 0 in A04 and A09). A00 records the seam as conformance rows (0016 single-regime need accounting / duration terminal closure, 0017 shared open-duration authority / tick-ledger coverage, 0022 shared need-delta emitters) — confirming the rule exists and is implemented; D6 documents it as a primary contract.
2. Verified against spec 0027 §3 D6 and source report §4.6 (A04/A05/A09-D). The spec's conditional "optionally add a one-line pointer in A00" is **resolved YES** at decomposition (cheap, spec-endorsed, improves navigation from A00's detailed rows to the new compact seam). Proof mechanics (replay/ledger no-double-charge evidence) route to execution `06`/`10` (spec §6, V4).
3. Shared boundary under audit: the single-owner accounting seam spanning A04 (reservations/durations + derived deltas), A05 (cognition consumes pressures, does not mint them), and A09 (ordinary-life systems share the same accounting + duration lifecycle), with the A00 index row pointing to it. The three doc subsections must state one mutually-consistent contract, which is why they land in one ticket.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-018` (deterministic replay is foundational) and `INV-009`/`INV-011` (meaningful state changes require events; current-state-only simulation forbidden) — a tick/window causally charged twice breaks replay truth even when byte-stable goldens pass; `INV-045` (ordinary survival is causal; fake meter refills disconnected from world state forbidden) — the food/sleep/work effects share the same event-sourced accounting. Additive; no invariant weakened (spec §4).
5. Deterministic-replay / accounting surface (governed here, enforcement deferred to execution `06`/`10`): derived need-deltas, elapsed-time effects, duration completion/interruption, and body-exclusive open/close state must flow through one authoritative accounting seam; schedulers, validators, action definitions, and projections may *consume* it but may not independently charge the same tick/window or silently reconcile duplicate terminals (INV-018 — identical inputs+versions produce identical output; no consumer recomputes deltas to drift). A05 cognition consumes live event-sourced need pressures and must not mint need-deltas, supply proposal-side current-need values as authority, or let routine labels charge time (INV-103/104 — scheduler/routines are not cognition authorities and do not dispatch directly). This ticket adds doctrine only; no leakage or nondeterminism path — it strengthens single-charge replay discipline.

## Architecture Check

1. The seam is one conceptual contract authored into the three subsystem docs that own its consumers (A04 pipeline/durations, A05 cognition, A09 ordinary life) plus an A00 index pointer — kept in one ticket because the three subsections must state one mutually-consistent rule, and splitting per-doc would risk three drifting fragments of the same seam (and a four-way A00/A04/A05/A09 file spread across tickets). This mirrors the 0026 "single coherent package across N files" precedent.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine consolidating rules already recorded in A00 hardening rows and code-locked; no compatibility layer.

## Verification Layers

1. `INV-018`/`INV-009` single-charge replay (D6 A04/A09) → invariants alignment check: A04 names the one authoritative accounting seam for derived deltas/durations (consumers may not double-charge a tick/window or silently reconcile duplicate terminals); A09 binds food/sleep/work/fatigue/hunger/wages/economy to that same seam + duration lifecycle.
2. `INV-103`/`INV-104` cognition consumes, does not mint (D6 A05) → invariants alignment check: A05 states actor decision transactions consume live event-sourced need pressures; candidate generation may explain pressure but must not mint need-deltas, supply proposal-side current-need values as authority, or let routine labels charge time.
3. Seam already recorded as A00 conformance rows / code-locked → codebase grep-proof: the A00 0016/0017/0022 rows (single-regime need accounting, shared open-duration authority, shared need-delta emitters) confirm D6 documents implemented behavior; the new A00 one-line pointer references the compact A04/A05/A09 seam.
4. Multi-doc additive: no replay/golden-fixture or skill-dry-run layer here (deferred to execution `06`/`10`); the layers above map each engaged invariant to a distinct surface.

## What to Change

### 1. D6 — single-charge derived-accounting seam (A04, A05, A09) + A00 pointer

- **A04** (`04_…PIPELINE.md`), near reservations/durations: derived need-deltas, elapsed-time effects, duration completion/interruption, and body-exclusive open/close state flow through one authoritative accounting seam; schedulers, validators, action definitions, and projections may consume it but may not independently charge the same tick/window or silently reconcile duplicate terminals.
- **A05** (`05_…PLANNING.md`): actor decision transactions consume live need pressures from event-sourced agent state; candidate generation may explain pressure but must not mint need-deltas, supply proposal-side current-need values as authority, or let routine labels charge time.
- **A09** (`09_…PROPERTY.md`): food/sleep/work/fatigue/hunger/wages/local-economy share that same event-sourced accounting + duration lifecycle — stable replay / byte-stable goldens are insufficient if two consumers causally charge the same tick/window twice.
- **A00** (`00_…CONFORMANCE.md`): keep the hardening rows as conformance examples; add a one-line index pointer to the new compact A04/A05/A09 seam (conditional resolved YES).

## Files to Touch

- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (modify)
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` (modify)
- `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- **Classifier names, data structures, function names, fixture shapes** — execution/implementation (spec §3, §6).
- **Replay/ledger no-double-charge proof, drift fixtures** — execution `06`/`10` (spec §6, V4).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Landing grep** — the seam is named in A04/A05/A09 and pointed to from A00: `grep -niE "accounting seam|single.charge|double.charge|charge the same tick" docs/1-architecture/0{0,4,5,9}_*.md` resolves the seam across the four files.
2. **Cognition-consumes review (A05)** — A05 states cognition consumes event-sourced pressures and must not mint need-deltas / supply proposal-side need values as authority / let routine labels charge time.
3. **Invariants alignment review** — the seam strengthens INV-018/009 single-charge replay and INV-103/104 cognition-consumes-not-mints; no invariant weakened, no execution mechanism token introduced.

### Invariants

1. Derived need-deltas / duration lifecycle have one authoritative accounting owner per actor/time window; consumers may not double-charge a tick/window or silently reconcile duplicate terminals (INV-018/009).
2. Cognition consumes live event-sourced pressures and does not mint deltas or let routine labels charge time; ordinary-life food/sleep/work/economy share the one seam (INV-103/104/045).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing grep across the four docs) plus an invariants-alignment manual review and a cross-check against the A00 0016/0017/0022 accounting conformance rows. No crate/code or fixture changes.`

### Commands

1. `grep -niE "accounting seam|single.charge|double.charge|charge the same tick" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — confirms D6 landed across the four files.
2. `git diff -- docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — review the A05 cognition-consumes-not-mints wording.
3. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the invariants-alignment review and the A00 accounting-row cross-check.`

## Outcome

Completed: 2026-06-13

Named the single-charge derived-accounting seam across A04, A05, and A09, and
added the A00 navigation pointer. A04 now owns the authoritative accounting
seam for derived need-deltas, elapsed-time effects, duration
completion/interruption, and body-exclusive open/close state. A05 states that
actor cognition consumes live event-sourced pressures and must not mint
need-deltas, proposal-side need authority, routine-label time charges, or
duration terminal reconciliation. A09 binds food, sleep, work, fatigue, hunger,
wages, and local-economy effects to the same event-sourced accounting seam and
duration lifecycle.

The user-provided implementation goal was treated as owner approval to enact
the tier-1 architecture amendment. No crate/code, fixture, execution,
foundation, or reference changes were made.

Verification:

- `grep -niE "accounting seam|single.charge|double.charge|charge the same tick" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`
- `git diff -- docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` was reviewed for cognition-consumes-not-mints wording.
- `rg -n "0017 tick-charge attribution authority|0017 shared open-duration authority|0022 shared need-delta emitter perimeter|single-regime need accounting" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`

Deviation: none.
