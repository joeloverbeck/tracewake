# 0033EXETIETEM-004: exec 07 temporal rendering & time-control proof

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (additive temporal-rendering / time-control proof over the existing embodied/debug split and `POS-PARITY` posture). No crate/code, no fixtures.
**Deps**: 0033EXETIETEM-010 (exec `10` temporal-rendering diagnostics; this proof cross-references the per-surface diagnostic home). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T5 (report F-05), the exec-`07` slice. Exec `07` owns epistemic view models, possession, debug proof, freshness display, and a staged note that player-facing time acceleration is debug-only — but it carries **no** temporal-rendering proof obligation: verified 0 `temporal`/`INV-112` matches in exec `07` at `c70d119`. Without it an embodied view can show "late"/"closed"/"due" because the client can read the clock or queue, and possession can become a temporal-knowledge upgrade. This ticket requires temporal labels to cite modeled sources, requires possession-parity temporal evidence, and pre-positions future player-facing time-control proof — additively over the existing embodied/debug split.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `07` (`07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`) owns the embodied/debug split, possession parity, freshness display, and the staged time-acceleration-is-debug-only note, and carries no temporal-rendering obligation (`grep -ciE 'temporal|INV-112' docs/2-execution/07_*` → 0). The gap is real; this ticket adds the obligation and preserves the existing split.
2. Verified against spec 0033 §3.1 D-T5 and ratified upstream A10 temporal-rendering / time-control seam (spec `0032`). UI clock format, acceleration speed, and missed-summary thresholds route to future scoped specs per spec §6.
3. Shared boundary under audit: exec `07` (temporal rendering / possession parity) ↔ exec `10` (temporal-rendering diagnostics distinguishing embodied/possession/debug/transcript/observer-only surfaces, ticket 010). This ticket states the `07` rendering obligation; the diagnostics live in `10`.
4. Constitutional invariants motivating this ticket, restated: `INV-006` (possession transfers no world knowledge) and `INV-108` (human possession is cognition-neutral) — possession must not refresh/reveal/reinterpret temporal facts; `INV-112` — embodied temporal labels are holder-known/observed/source-backed, not clock/queue reads.
5. Actor-knowledge / possession surface: the obligations require embodied-view temporal labels (late, stale, closed, due, soon, recently, missed) to cite modeled sources rather than a readable clock/queue, require possession-parity evidence that possessing an actor does not refresh/reveal/reinterpret temporal facts beyond what that actor would know, require future player-facing time-control proof (advancing time produces events/observations through modeled channels; any missed-event summary carries source and holder visibility), and keep debug exact time / raw queues / replay timestamps / acceleration internals non-diegetic and excluded from actor-known context. This strengthens `POS-PARITY` and epistemic-leakage prevention; it introduces no determinism change. Per-surface diagnostics live in exec `10` (ticket 010).

## Architecture Check

1. Exec `07` is the correct home: it already owns the embodied/debug split and possession parity, so temporal rendering is a specialization. Stating it here keeps the actor-visible/debug boundary intact and prevents temporal labels from becoming a possession-only knowledge upgrade.
2. No backwards-compatibility aliasing/shims: additive obligation over the existing split; no rename, no weakening, no UI clock format chosen.

## Verification Layers

1. `INV-112` temporal rendering → invariants alignment check: exec `07` requires embodied temporal labels to cite modeled sources, not clock/queue reads.
2. `INV-006`/`INV-108` possession parity → invariants alignment check: possession does not refresh/reveal/reinterpret temporal facts beyond actor knowledge; debug exact time/queues/timestamps stay non-diegetic.
3. Cross-reference integrity → codebase grep-proof: exec `07` points to exec `10` for per-surface temporal-rendering diagnostics (ticket 010).
4. Documentation-only doctrine ticket: replay/golden-fixture and skill-dry-run layers are exec `09`/`10` proof surfaces; the layers above map each engaged invariant and the cross-reference to distinct surfaces.

## What to Change

### 1. D-T5 — temporal rendering, possession parity, and future time-control proof

Add to exec `07`: embodied-view temporal labels (late, stale, closed, due, soon, recently, missed) must cite modeled sources, not a readable clock/queue; possession-parity evidence must show possessing an actor does not refresh, reveal, or reinterpret temporal facts beyond what that actor would know; future player-facing time-control proof must show that advancing time produces events/observations through modeled channels (any missed-event summary carries source and holder visibility); debug exact time, raw queues, replay timestamps, and acceleration internals stay non-diegetic and excluded from actor-known context. Cross-reference exec `10` for the per-surface temporal-rendering diagnostics. Preserve the existing embodied/debug split and staged time-acceleration note; choose no UI clock format, acceleration speed, or summary threshold (§6).

## Files to Touch

- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify)

## Out of Scope

- **UI clock format, acceleration speed, missed-summary thresholds** — future scoped specs (§6).
- **Per-surface temporal-rendering diagnostics** — exec `10` (ticket 010).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits; minting any new gate code.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T5 landing grep** — exec `07` carries the temporal-rendering / possession-parity obligation: `grep -niE 'temporal (label|render)|possession.*temporal|time control' docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` resolves it.
2. **Cross-reference check** — exec `07` points to exec `10` for per-surface diagnostics.
3. **Mechanism-token + invariants review** — no UI clock format, acceleration speed, or summary threshold entered exec `07`; upholds `INV-006`/`INV-108`/`INV-112`, preserves the embodied/debug split and `POS-PARITY` (no rename/weaken); no new gate code.

### Invariants

1. Embodied temporal labels cite modeled sources; debug exact time/queues/timestamps stay non-diegetic (`INV-112`).
2. Possession is not a temporal-knowledge upgrade (`INV-006`/`INV-108`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing grep) plus a cross-reference and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal (label|render)|possession.*temporal|time control|non-diegetic' docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — confirms D-T5 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the cross-reference and invariants-alignment review.`
