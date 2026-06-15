# 0033EXETIETEM-011: exec 03 phase-ladder temporal cascade & staged-declaration placement

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` (additive certification-sequence placement for the temporal cascade and staged-declaration review; mints no new gate code). No crate/code, no fixtures.
**Deps**: 0033EXETIETEM-001 (exec `04`), 0033EXETIETEM-003 (exec `06`), 0033EXETIETEM-004 (exec `07`), 0033EXETIETEM-006 (exec `09`), 0033EXETIETEM-007 (exec `11`), 0033EXETIETEM-008 (exec `12`), 0033EXETIETEM-010 (exec `10`) — the sequence places where each of these proof homes' evidence enters certification, so they must land first for the placement to reference real obligations. **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T1, D-S1 (report F-01, F-13), the exec-`03` slices. Exec `03` owns the phase-ladder gate order and certification sequence and protects settled gate order, but says **nothing** about where temporal-cascade evidence or staged-declaration review enter the sequence: verified 0 `temporal`/`staged` matches in exec `03` at `c70d119`. Without placement, an implementer cannot tell from the certification sequence when temporal evidence is required (first-proof), when Phase-4 procedural-time evidence appears, where second-proof LOD evidence is deferred-but-declared, or when staged-declaration review must precede acceptance. This ticket places both in the existing sequence — coordinating, not duplicating, the gates — and mints no new gate code.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `03` (`03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`) owns the certification sequence and protects settled gate order, and carries no temporal-cascade or staged-declaration placement (`grep -ciE 'temporal|staged' docs/2-execution/03_*` → 0). The gap is real; this ticket adds placement, not a new gate.
2. Verified against spec 0033 §3.1 D-T1 and §3.3 D-S1, and §6 (no new gate code). The concrete certification field wording routes to enactment; the placement is the deliverable.
3. Shared boundary under audit: exec `03` (certification sequence) ↔ the proof homes it sequences — exec `04`/`06`/`07`/`09`/`10` (first-proof temporal evidence), `11` (Phase-4 procedural time), `12` (deferred second-proof LOD), and `10` (staged-abstraction review). This ticket places where evidence enters; the obligations themselves live in those homes. Exec `03` also shares the D-T1/D-S1 deliverables with exec `00` (ticket 012) — this ticket owns the `03` certification-sequence slice; `00` owns the index routing-map slice.
4. Constitutional invariant motivating this ticket, restated: `INV-112` — the certification sequence must require temporal-firewall evidence at the right gates so holder-known-time-must-plan is proven before acceptance; and the staged-declaration discipline (spec Driver S) is an acceptance-honesty rule placed in the sequence, not a new gate.
5. Acceptance / certification-sequence surface: the placement requires temporal evidence to appear before first-proof acceptance, Phase-4 procedural-time evidence at Phase-4 entry, second-proof LOD/time-acceleration evidence deferred-but-declared, and staged-declaration review before acceptance evidence is treated as sufficient for a stage. It coordinates existing gates and adds no enforcement of its own and no new gate code; it weakens no settled gate order and introduces no determinism change.

## Architecture Check

1. Exec `03` is the correct home for certification-sequence placement: it already owns gate order and the certification sequence, so placing the temporal cascade and staged-declaration review here coordinates them with the settled sequence rather than duplicating gate content. A pointer-style placement (not a gate restatement) is the correct shape.
2. No backwards-compatibility aliasing/shims: additive placement over the existing sequence; no rename, no weakening of settled gate order, no new gate code.

## Verification Layers

1. `INV-112` temporal certification placement (D-T1) → invariants alignment check: exec `03` places where temporal evidence enters the sequence (first-proof, Phase-4, deferred second-proof) without creating a new gate code.
2. Staged-declaration placement (D-S1) → invariants alignment check: exec `03` requires staged-declaration review before acceptance evidence is treated as sufficient for a stage.
3. Settled-order-preserved check → codebase grep-proof + manual review: the existing gate order is unchanged; the placement coordinates, not duplicates.
4. Documentation-only doctrine ticket: the proof homes are the sibling tickets; the layers above map each engaged obligation to a distinct surface.

## What to Change

### 1. D-T1 — temporal cascade certification placement

Place the temporal cascade in the existing certification sequence: where temporal evidence must appear before first-proof acceptance (through exec `04`/`06`/`07`/`09`/`10`), where Phase-4 procedural-time evidence appears (exec `11`), and where second-proof LOD/time-acceleration evidence is deferred-but-declared (exec `12`). Coordinate, do not duplicate, the budget/fairness proof (consolidated in exec `10`). Create no new gate code.

### 2. D-S1 — staged-declaration review placement

Place staged-declaration review in the certification sequence: the declaration is required before acceptance evidence is treated as sufficient for a stage. Mint no new gate label or observation-obligation code (spec §R-G).

## Files to Touch

- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` (modify)

## Out of Scope

- **The proof obligations themselves** — the sibling tickets (exec `04`/`06`/`07`/`09`/`10`/`11`/`12`).
- **The index routing-map and staged authority rule** — exec `00` (ticket 012).
- **Minting a new gate code / observation-obligation code** — forbidden (spec §6, §R-G).
- **Concrete temporal values** — reference/future scoped specs (§6).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T1 landing grep** — exec `03` places the temporal cascade in the sequence: `grep -niE 'temporal.*(evidence|cascade|certification)|first-proof.*temporal' docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` resolves the placement.
2. **D-S1 landing grep** — staged-declaration review placed: `grep -niE 'staged.*(declaration|abstraction|review)' docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` resolves it.
3. **No-new-gate + settled-order review** — manual review confirms no new gate code was minted and the settled gate order is unchanged.
4. **Invariants alignment review** — upholds `INV-112` and the staged-incompleteness discipline; coordinates, does not duplicate, the consolidated budget/fairness proof.

### Invariants

1. Temporal evidence and staged-declaration review have a defined place in the certification sequence, with no new gate code and no change to settled gate order (`INV-112`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing greps) plus a no-new-gate/settled-order and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal.*(evidence|cascade|certification)|staged.*(declaration|review)' docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — confirms D-T1/D-S1 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the no-new-gate and invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented the exec `03` temporal-cascade and staged-declaration certification
placement in
`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`.
The edit places first-proof temporal evidence before `FIRST-PROOF-CERT`,
Phase-4 procedural-time evidence at `PHASE-4-ENTRY`, deferred second-proof LOD
temporal-ancestry evidence before `SECOND-PROOF-ENTRY`, and staged-declaration
review before stage acceptance evidence is treated as sufficient.

The execution-blocking owner-approval precondition in spec 0033 was satisfied
by the user's explicit request to implement the `0033EXETIETEM` ticket series.
No crate/code or fixture files were changed.

Verification:

- `grep -niE 'temporal.*(evidence|cascade|certification)|first-proof.*temporal' docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `grep -niE 'staged.*(declaration|abstraction|review)' docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `rg -n 'new gate|new canonical|mint|gate label|observation-obligation|Budget and fairness|FIRST-PROOF-CERT|PHASE-4-ENTRY|SECOND-PROOF-ENTRY' docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `git diff --check`

Manual review confirmed the additions uphold `INV-112` and staged
incompleteness discipline, preserve the settled gate order, coordinate rather
than duplicate the budget/fairness proof, and mint no new gate label,
observation-obligation code, or canonical gate code.
