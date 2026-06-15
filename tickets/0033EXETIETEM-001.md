# 0033EXETIETEM-001: exec 04 temporal-firewall, temporal-claim & anti-truth-cache gates

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` (additive temporal-firewall proof obligations, temporal-claim slots, and anti-hidden-truth-learning evidence over the existing `TFW` gate). No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T2, D-T3, D-R2 (report F-02, F-03, F-09), the exec-`04` slices. Exec `04` owns the truth-firewall / actor-known / anti-contamination gate (`TFW`) with sealed-context, provenance, staleness, and fail-closed posture, but it carries **no** temporal specialization: verified 0 `temporal`/`INV-112` matches in exec `04` at `c70d119`. Without the temporal-firewall proof obligations a certification can pass while raw scheduler/replay/validator time leaks into actor-known or institution-known planning, and without the anti-hidden-truth-learning obligation a failed action or debug fact can silently become future belief. This ticket threads the temporal case of the truth firewall, holder-known temporal-claim slots, and two-sided learning evidence into the existing `TFW` gate — additively, never renaming or weakening it.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `04` (`04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`) owns the `TFW` gate, sealed-context/provenance/staleness/fail-closed posture, and carries no temporal/`INV-112`/anti-truth-cache-learning language (`grep -ciE 'temporal|INV-112' docs/2-execution/04_*` → 0). The gap is real; this ticket adds proof obligations, it does not restate foundation/architecture doctrine.
2. Verified against spec 0033 §3.1 D-T2/D-T3 and §3.2 D-R2, and the ratified upstream: foundation `INV-112` + foundation `03` "Temporal authority" (spec `0031`), and architecture A03/A04 temporal contracts (spec `0032`). The homes for the exec-`04` slices are this doc; concrete temporal vocabulary, stale-after thresholds, and learning update rules route to reference/future scoped specs per spec §6.
3. Shared boundary under audit: exec `04` (truth-firewall gate) ↔ exec `05` (scheduler temporal authority, ticket 002) ↔ exec `10` (temporal-divergence + learning diagnostics, ticket 010). This ticket states the certification-evidence obligation at the firewall; the scheduler's own temporal-authority statement is 002 and the responsible-layer diagnostics are 010 (cross-referenced, not duplicated).
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-112` (time may validate, but holder-known time must plan); `INV-099` (truth may validate actions, but truth may not plan them — the temporal firewall is its temporal case); `INV-102` (cognition inputs require provenance — temporal premises included). The obligations specialize these for time and learning; they weaken none.
5. Fail-closed / actor-knowledge / deterministic-replay surface (this IS the truth-firewall doc): the obligations require (a) temporal-firewall evidence — the modeled source of every temporal premise consumed by actor-known/institution-known code, (b) adversarial evidence that raw scheduler time, replay order, debug panels, event timestamps, sorted queues, and validator-known future/due states cannot be read as holder knowledge via type shortcuts / cached truth / renamed fields / derived helpers / prompt-context prose, (c) temporal-claim provenance slots in holder-known context checks, and (d) negative hidden-truth-learning evidence (no failed action, scheduler denial, debug fact, true location/schedule/institutional status, or replay-only diagnostic becomes future belief/expectation absent a modeled experience/communication channel). This strengthens epistemic-leakage prevention; it introduces no nondeterminism (raw wall-clock/scheduler input is named as a leak the evidence must catch, not an allowed planning input). Enforcement diagnostics with responsible-layer labels live in exec `10` (ticket 010); this doc states the certification obligation.

## Architecture Check

1. Exec `04` is the correct single home: it already owns the truth-firewall certification and anti-contamination posture, so the temporal firewall (the truth firewall applied to time), temporal-claim provenance, and anti-truth-cache learning are specializations of contracts `04` already carries — not a new gate. Splitting them elsewhere would divorce the temporal leak surface from the firewall that certifies against it.
2. No backwards-compatibility aliasing/shims: additive proof obligations over the existing `TFW` gate; no rename, no weakening, no new gate code.

## Verification Layers

1. `INV-112` / `INV-099` temporal firewall → invariants alignment check + codebase grep-proof: exec `04` requires every temporal premise consumed by actor-/institution-known code to carry a modeled source, and requires adversarial evidence against raw-clock/replay/debug/queue/validator-time leakage.
2. `INV-102` temporal-claim provenance → invariants alignment check: holder-known context checks carry temporal-claim slots (event/acquisition/verification/procedure source; valid/expired/stale status; explicit uncertainty) when a decision depends on time.
3. `INV-099` anti-hidden-truth-learning → invariants alignment check: negative evidence that prohibited sources never become future belief/expectation absent a modeled channel; the matching positive adaptation proof is exec `06` (ticket 003) and the distinguishing diagnostics are exec `10` (ticket 010).
4. Documentation-only doctrine ticket: no replay/golden-fixture or skill-dry-run layer applies here (those proof surfaces are exec `09`/`10`, tickets 006/010); the layers above map each engaged invariant to a distinct surface.

## What to Change

### 1. D-T2 — temporal-firewall evidence in the `TFW` certification

Add to exec `04`, over the existing truth-firewall gate, the obligation that every certification touching time produces temporal-firewall evidence: the modeled source of each temporal premise consumed by actor-known/institution-known code (observation, memory, record, notice, testimony, public cue, artifact, modeled procedure, source-backed inference), plus adversarial evidence that raw scheduler time, replay order, debug panels, event timestamps, sorted queues, and validator-known future/due states cannot be read as holder knowledge via type shortcuts, cached truth, renamed fields, derived convenience helpers, or prompt/context prose. Cross-reference exec `05` (scheduler temporal authority) and exec `10` (responsible-layer temporal-divergence diagnostics); do not duplicate them.

### 2. D-T3 — holder-known temporal-claim slots

Require temporal-claim slots in holder-known context checks when a decision depends on time: event/acquisition/verification/procedure source; valid/expired/stale status as known by the holder; explicit uncertainty where appropriate. State the obligation abstractly — no concrete stale-after threshold, day-part vocabulary, or calendar syntax (those route to reference/spec per §6).

### 3. D-R2 — negative hidden-truth-learning evidence

Require negative evidence that none of failed actions, scheduler denials, debug facts, true object locations, true schedules, true institutional status, or replay-only diagnostics becomes a future actor belief/expectation unless a modeled experience or communication channel emitted a claim, memory, contradiction, observation, notice, or record. The positive adaptation counterpart is exec `06` (ticket 003); learning update rules/decay/thresholds route to a future scoped spec (§6).

## Files to Touch

- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` (modify)

## Out of Scope

- **Concrete temporal vocabulary, stale-after thresholds, calendar/duration syntax** — reference/future scoped specs (spec §6).
- **Scheduler's own temporal-authority statement** — exec `05` (ticket 002).
- **Responsible-layer temporal-divergence + learning diagnostics** — exec `10` (ticket 010).
- **Positive adaptation proof** — exec `06` (ticket 003); **learning update rules/decay/thresholds** — future scoped spec.
- **Temporal-firewall fixture families** — exec `09` (ticket 006).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits; minting any new gate code.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T2 landing grep** — exec `04` carries the temporal-firewall evidence obligation: `grep -niE 'temporal[- ]firewall|temporal premise' docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` resolves the obligation.
2. **D-T3/D-R2 landing grep** — temporal-claim slots and anti-truth-cache learning present: `grep -niE 'temporal claim|hidden[- ]truth|truth[- ]cache' docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` resolves both.
3. **Mechanism-token boundary review** — manual review confirms the added passages name no stale-after threshold, day-part/lateness vocabulary, calendar syntax, or learning update rule; and mint no new gate code.
4. **Invariants alignment review** — the obligations uphold `INV-112`/`INV-099`/`INV-102` and preserve the existing `TFW` gate (no rename/weaken).

### Invariants

1. Every temporal premise reaching actor-/institution-known planning carries a modeled source; raw clock/replay/debug/queue/validator time is a named leak the certification must catch (`INV-112`/`INV-099`).
2. Temporal status is a source-backed holder-known claim, not a free truth label (`INV-102`); prohibited sources never become future belief absent a modeled channel (`INV-099`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing greps) plus a mechanism-token-boundary and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal[- ]firewall|temporal premise|temporal claim|hidden[- ]truth|truth[- ]cache' docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — confirms D-T2/D-T3/D-R2 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the mechanism-token-boundary and invariants-alignment review.`
