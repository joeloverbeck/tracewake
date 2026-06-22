# 0047TUIAUTWOR-002: Architecture amendments (10/04/02/03/00/13)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — docs only: `docs/1-architecture/{10,04,02,03,00,13}_*.md`
**Deps**: None

## Problem

Spec 0047 §5 routes architecture-tier amendments (substance + home) that operationalize the human-authoritative-world-advance capability. Per the driver report's dispositions D-04…D-09, the architecture contracts must make explicit: the typed time-control → core world-step boundary (arch `10`); the single authoritative one-tick coordinator, log-derived open-duration discovery, ordinary proposal routing, general body-exclusive conflict, and explicit continuation controls (arch `04`); replay-visible tick ancestry for otherwise-empty steps and deterministic interval-projection rebuild (arch `02`); interval summaries as positively-constructed source-bearing holder-known frontier deltas (arch `03`); conformance rows naming human world advance as a consumer of the existing `0017` authorities (arch `00`); and the new proof obligations (arch `13`). These approved architecture semantics must exist before code lands (decomposition ticket 1, architecture portion).

## Assumption Reassessment (2026-06-22)

1. All six amendment targets exist at their cited tier/prefix: `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`, `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`, `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`, `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`, `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (all confirmed present). The arch `00` conformance rows the amendment extends already exist: `0017 tick-charge attribution authority` and `0017 shared open-duration authority` (lines 116–117). The amendment names human world advance as a *consumer* of those existing rows and must not mint a new conformance family unless reassessment proves the existing rows cannot express the obligation (§5).
2. Spec 0047 §5 enumerates the per-doc substance and home; §2 lists the same docs as the governing architecture anchors. This ticket carries the architecture tier only; foundation `08` is 0047TUIAUTWOR-001, execution is 0047TUIAUTWOR-003, reference is 0047TUIAUTWOR-004.
3. Cross-artifact boundary under audit: the architecture tier sits below foundation `08` (0047TUIAUTWOR-001) and above execution (0047TUIAUTWOR-003). Arch `04` introduces the "single authoritative world-step coordinator" standard that arch `10` and arch `02` reference; because all six docs land in this one ticket, the internal consistency is checkable within the diff. The amendments must remain consistent with foundation 08's amended Time-controls doctrine and must not specify Rust type names, function signatures, or test names (substance + home only).
4. Constitutional invariants motivating the amendment: `INV-018`/`INV-092` (deterministic replay is foundational and tested — replay-visible tick ancestry for empty steps), `INV-103`/`INV-104` (the scheduler is not cognition authority; routines/needs do not dispatch primitive actions directly — ordinary proposal routing and the coordinator orders established seams), `INV-008` (UI assistance is not authority — the TUI asks core to advance and never mutates).
5. Enforcement surfaces governed by doctrine (substrate-only basis): replay/hash determinism (`INV-018`; arch `02`), the no-direct-dispatch / kernel-authority boundary (`INV-103`/`INV-104`; arch `04`/`10`), and the actor-knowledge / truth-firewall projection boundary (`INV-024`/`INV-099`/`INV-102`; arch `03`). The amendments must require the marker for empty steps to be world/replay evidence that never enters holder-known projections, must keep the world-control → core boundary one-way, and must define interval summaries as positively-constructed source-bearing deltas rather than raw-world-diff redaction — introducing no leakage or nondeterminism path the code tickets (005/013) would have to undo.

## Architecture Check

1. Amending the existing architecture contracts in place keeps each concern in its established home (play loop → `10`, pipeline/scheduling → `04`, replay/projection → `02`, holder-known → `03`, conformance index → `00`, acceptance artifacts → `13`) rather than inventing a new cross-cutting "time controls" architecture doc that would duplicate authority already tiered across these six.
2. No backwards-compatibility aliasing/shims: conformance rows are *extended* to name a new consumer, not duplicated; no parallel old/new contract statement is left standing.

## Verification Layers

1. `INV-103`/`INV-104` no-direct-dispatch -> invariants alignment check: arch `04`/`10` amendments assert ordinary proposal routing and a one-way world-control → core boundary; cross-read against the invariant headings.
2. `INV-018`/`INV-092` deterministic replay -> invariants alignment check: arch `02` amendment requires replay-visible tick ancestry for empty steps and deterministic interval-projection rebuild.
3. `INV-024`/`INV-099`/`INV-102` truth firewall -> manual review (epistemic-leakage audit): arch `03` amendment defines interval summaries as positively-constructed source-bearing holder-known deltas and prohibits raw-world-diff redaction as the embodied boundary.
4. Conformance-row landing -> codebase grep-proof: the amended `0017` rows in arch `00` name human world advance as a consumer; the new proof obligations resolve in arch `13`.

## What to Change

### 1. Architecture `10` — temporal-rendering / embodied-play-loop section

Make explicit: the typed world-control → core world-step boundary (TUI asks core to advance, never mutates state/events directly); acceleration as repeated one-tick progression; the actor-known interval summary vs the debug step report split.

### 2. Architecture `04` — pipeline/scheduling section

Own: a single authoritative world-step coordinator; deterministic due-work ordering; ordinary proposal routing for both human and autonomous actors; log-derived open-duration discovery; the general body-exclusive conflict rule; and explicit continuation controls that do not masquerade as ordinary actor actions.

### 3. Architecture `02` — event-log / replay / projection section

Require: replay-visible tick-boundary evidence for otherwise-empty world steps; repeated-step acceleration; rebuild of the temporal frontier; and deterministic interval projection.

### 4. Architecture `03` — holder-known contexts section

Define interval summaries as positively-constructed source-bearing holder-known frontier deltas; prohibit raw-world-diff redaction as the embodied boundary.

### 5. Architecture `00` — conformance index

Correct/extend the existing `0017 tick-charge attribution authority` and `0017 shared open-duration authority` rows to name human world advance as a consumer; name time-control / interval-summary parity under the existing parity posture. Do not mint a new conformance family unless reassessment proves the existing rows cannot express the obligation.

### 6. Architecture `13` — acceptance/review artifacts

Add proof obligations: one-tick human/no-human differential; duration-terminal and accounting witnesses; replay / no-op-tick evidence; actor-known summary anti-leak evidence; reservation-conflict; and the spec-0046 parity extension.

### 7. Execution precondition (recorded, not auto-applied)

Architecture-tier amendments require ordinary architecture-owner approval before application (`docs/README.md`). The ticket records substance + home; it is not merged by convention.

## Files to Touch

- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (modify)
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (modify)
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` (modify)
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)

## Out of Scope

- Any code change — owned by 0047TUIAUTWOR-005 onward.
- Foundation `08` (0047TUIAUTWOR-001), execution `05`/`06`/`07`/`10` (0047TUIAUTWOR-003), reference `00`/`01` (0047TUIAUTWOR-004).
- Minting a new conformance family, `INV-###`, gate code, or risk ID (§5 / §1.2 prohibition).
- Specifying Rust types, function signatures, scheduler-phase numbering, or test names (substance + home only).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "world step|world-step|advance" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` shows the typed world-control → core boundary and the actor-known/debug split landed.
2. `grep -niE "coordinator|continuation" docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` shows the single coordinator + explicit continuation controls.
3. `grep -niE "tick.boundary|empty|ancestry" docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` and `grep -n "0017" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` confirm the replay-ancestry amendment and the consumer-extended `0017` rows.

### Invariants

1. The arch `00` amendment extends existing `0017` rows (consumer naming), introducing no new conformance-family identifier.
2. No amended section moves world authority into a view/TUI or grants the LLM authority; the world-control → core boundary stays one-way (`INV-008`/`INV-103`/`INV-104`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep landing + invariants-alignment review); the governed enforcement surfaces are implemented/tested by 0047TUIAUTWOR-005/006/009/010/013/016/017.`

### Commands

1. `for d in 10 04 02 03 00 13; do echo "== $d =="; grep -ciE "world step|coordinator|ancestry|holder-known|0017|differential" docs/1-architecture/${d}_*.md; done`
2. `grep -n "0017" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
3. Narrower command is correct: architecture doc amendments are verified by landing greps + a manual `INV-018`/`INV-103`/`INV-024` alignment read, not by `cargo` tests — no code changes in this ticket.
