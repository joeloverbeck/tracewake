# 0032ARCTIETEM-004: A05 routine/social-rhythm temporal premises

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edit to `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` (routine/social-rhythm temporal-premise subsection). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-002 (routine/social-rhythm premises are holder-known temporal claims, whose shape A03/A06 must own first). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-T5 (report T5). Foundation `05` says routines and schedules consume actor-known or holder-known temporal premises: a worker works because they believe or have source-backed reason to treat it as work time, not because a true global schedule row is read. A05 already defines actor decision transactions, needs, intentions, routine/HTN method selection, and candidate generation from holder-known context, and already protects against routine labels dispatching actions directly — but it carries **no explicit temporal-premise list and source discipline** for routine/social rhythm. Without it, a routine template's mere presence can read as an authoritative "it is work time."

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A05 owns actor decision transactions, needs/intentions, routine/HTN method selection, candidate generation from holder-known context, and the no-routine-label-dispatch protection — but states no temporal-premise source list for routine/social rhythm. This ticket adds the temporal-premise discipline; it does not reinvent the routine/method-selection contract.
2. Verified against spec 0032 §3.1 D-T5 and source report §5 Finding T5 / foundation `05`. D-T5's home is A05. Additive; relaxes nothing. Minimal first-playable schedule vocabulary, concrete routine windows, and interruption policy route to execution per spec §6.
3. Shared boundary under audit: A05 (routine/method selection) ↔ A03/A06 holder-known temporal context (ticket 002, this ticket's `Deps`). **A05 is a 3-way merge hub** — also touched by sibling tickets 010 (affect/learning) and 012 (budget/fairness). This ticket lands first in that chain (010 `Deps` on it, 012 `Deps` on 010); coordinate the A05 merge.
4. Constitutional invariant motivating this ticket, restated: `INV-112` — cognition (here, routine selection) may use temporal facts only when they reached the actor through modeled channels; a routine template's presence is not itself such a channel. D-T5 names the defeasible temporal premises and their permitted source routes.
5. Actor-knowledge surface (governed here; enforcement deferred to execution `06`/`10`): work, sleep, meals, patrols, appointments, market patterns, household obligations, and institutional appointments are defeasible temporal premises inside actor-known context, usable by candidate generation/method selection only if they arrive through assignment, memory, observation, public cues, records, testimony, institutional context, or modeled inference; a routine template may organize method families and expected rhythms but is not an information channel. This is `INV-112` applied to routines; it adds doctrine only and explicitly supports tickets 010 (affect/learning tune salience only after the holder-known premise exists) and 012 — no leakage path.

## Architecture Check

1. A05 is the correct home: it already owns routine/method selection and candidate generation from holder-known context, so the temporal-premise discipline is a specialization of a contract A05 carries. A separate home would split the routine premises from the routine selection they feed.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine reinforcing the existing no-routine-label-dispatch rule with a temporal-source requirement.

## Verification Layers

1. `INV-112` routine/social-rhythm temporal premises (D-T5) → invariants alignment check: A05 gains the defeasible-premise list and the permitted-source-route requirement, and states that a routine template's presence is not an information channel.
2. Cross-deliverable consistency → invariants alignment check: the subsection states affect/learning (tickets 010) may tune salience/method preference only after the same holder-known premise is available.
3. Single doc, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `06`/`10`); the layers above map the engaged invariant to a distinct alignment proof.

## What to Change

### 1. D-T5 — routine/social-rhythm temporal-premise subsection in A05

Add to `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`: work, sleep, meals, patrols, appointments, market patterns, household obligations, and institutional appointments are defeasible temporal premises inside actor-known context. Candidate generation and method selection may use those premises only if they come through assignment, memory, observation, public cues, records, testimony, institutional context, or modeled inference. Routine templates may organize method families and expected temporal rhythms, but a template's presence is not itself an information channel. (Affect and learning — tickets 010 — may tune salience and method preference only after the same holder-known premise is available; they cannot create a hidden temporal fact.)

## Files to Touch

- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` (modify)

## Out of Scope

- **First-playable schedule vocabulary, concrete routine windows, interruption policy, tests** — execution/implementation (spec §6).
- **Affect/learning derived-state seam (A05/A06)** — sibling ticket 010.
- **Budget/fairness planner-degradation seam (A05)** — sibling ticket 012 (D-R5).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T5 landing grep** — A05 carries the routine temporal-premise discipline: `grep -niE "temporal premise|routine|work time|information channel" docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` resolves the subsection.
2. **Source-route review** — the subsection names the permitted source routes (assignment/memory/observation/public cues/records/testimony/institutional context/modeled inference) and states a template's presence is not a channel.
3. **Invariants alignment review** — upholds `INV-112` (routine temporal facts must be holder-known); no schedule vocabulary or window value introduced.

### Invariants

1. Routine/social-rhythm temporal premises are defeasible and usable only via modeled source routes; a routine template's presence is not an information channel (`INV-112`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing grep) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "temporal premise|routine|work time|information channel" docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — confirms D-T5 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented D-T5 in `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` as a `Routine and social-rhythm temporal premises` subsection after the HTN/routine method-selection contract. The new text defines work, sleep, meals, patrols, appointments, market patterns, household obligations, and institutional appointments as defeasible temporal premises inside actor-known context, usable only through modeled assignment, memory, observation, public cues, records, testimony, institutional context, or modeled inference. It also states that routine templates organize method families and expected rhythms but are not information channels, and that affect/learning cannot create hidden temporal facts.

The execution-blocking owner-approval precondition from spec 0032 §R-A is satisfied by the user's explicit `$ticket-series implement the series tickets/0032ARCTIETEM*` request for this architecture-tier amendment series.

Verification:

- `grep -niE "temporal premise|routine|work time|information channel" docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- Manual source-route review: the added subsection names assignment, memory, observation, public cues, records, testimony, institutional context, and modeled inference, and states that a template's presence is not a channel.
- Manual invariants alignment review: the addition preserves `INV-112` by requiring routine/social-rhythm temporal facts to be holder-known before routine candidate generation or method selection can use them.
- Manual mechanism-token boundary review: no first-playable schedule vocabulary, concrete routine windows, or interruption policy was introduced.

No crate/code or fixture changes were made for this documentation-only architecture ticket.
