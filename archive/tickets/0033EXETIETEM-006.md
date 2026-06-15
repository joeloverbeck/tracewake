# 0033EXETIETEM-006: exec 09 temporal & quantity fixture families

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` (additive temporal-firewall and quantity/economy fixture-*family* obligations, plus adversarial validation-bypass fixtures, over the existing `FIXTURE`/`REPLAY` taxonomy). No crate/code, no fixtures authored here.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T3, D-R1, D-R5 (report F-03, F-08, F-12), the exec-`09` slices. Exec `09` owns the fixture taxonomy, replay acceptance, adversarial fixture requirements, and semantic-proof posture (`FIXTURE`/`REPLAY`) but names **no** temporal fixture family, no separate quantity/economy fixture family, and no validation-bypass adversarial fixtures: verified 0 `temporal` matches in exec `09` at `c70d119`. Without these the first-playable proof has no fixture family exercising holder-known timing or hidden stock/ledger cases. This ticket adds the fixture-*family* obligations (positive + adversarial), choosing no concrete fixture names and keeping the temporal and quantity packages unbundled (spec §R-E).

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `09` (`09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`) owns the fixture taxonomy, replay acceptance, and adversarial fixture posture, and names no temporal or quantity/economy fixture family (`grep -ciE 'temporal|fungib|custody' docs/2-execution/09_*` → spot-checked absent). The gap is real; this ticket adds family obligations, not fixture files.
2. Verified against spec 0033 §3.1 D-T3 and §3.2 D-R1/D-R5, and ratified upstream A02/A03 temporal-divergence + A09 quantity-lineage seams (spec `0032`). The concrete friendly/adversarial fixture *files* are future scoped specs per spec §6; this ticket stages only the *family* obligations.
3. Shared boundary under audit: exec `09` (fixture families) ↔ exec `08` (the authoring validations these fixtures exercise, ticket 005) ↔ exec `10` (replay-divergence diagnostics, ticket 010) ↔ exec `04` (the temporal-firewall gate the temporal family proves, ticket 001). This ticket states the family obligation; the validations and diagnostics live in `08`/`10`.
4. Constitutional invariant motivating this ticket, restated: `INV-018` (deterministic replay) and the `INV-111` observer-only / `INV-099` truth-firewall lineage applied to fixtures — fixtures must exercise holder-known timing and hidden-stock cases without leaking truth, and must replay deterministically.
5. Fixture / deterministic-replay surface (`FIXTURE`/`REPLAY`): the obligations require (a) a first temporal-firewall fixture **family** with friendly families (holder-known work/routine timing, stale-but-believed notices, institutional records, interruption/wait effects) and adversarial families (raw-clock leakage, debug-panel leakage, omniscient due/closed labels, restamping old knowledge as fresh); (b) separate fixture families for quantity/economy behavior (positive transfer/consume/split/merge/custody; adversarial hidden stock/ledger/procedure); (c) adversarial fixtures that attempt to bypass validation through renamed fields, nested prose, stale strings, generated content, fixture metadata, or review-artifact text. No concrete fixture names are chosen, and the temporal and quantity/economy packages stay unbundled unless one gameplay feature genuinely needs both (spec §R-E). This preserves deterministic replay and introduces no nondeterminism.

## Architecture Check

1. Exec `09` is the correct home: it already owns the fixture taxonomy and adversarial-fixture posture, so the temporal and quantity fixture families are specializations. Stating family obligations (not fixture files) here keeps the concrete fixtures at the lower spec tier where they belong (§6).
2. No backwards-compatibility aliasing/shims: additive family obligations over `FIXTURE`/`REPLAY`; no rename, no weakening, no fixture names chosen.

## Verification Layers

1. `INV-018` deterministic replay / temporal fixtures → invariants alignment check: exec `09` requires a temporal-firewall fixture family (friendly + adversarial) replaying deterministically.
2. No-scripting / `INV-099` quantity fixtures → invariants alignment check: exec `09` requires a separate quantity/economy fixture family (positive + adversarial hidden-stock/ledger/procedure), unbundled from temporal (spec §R-E).
3. `INV-099` validation-bypass adversarial fixtures → invariants alignment check: exec `09` requires adversarial fixtures attempting validation bypass via renamed fields / nested prose / stale strings / generated content / metadata / review-artifact text.
4. Documentation-only doctrine ticket: the validations these fixtures exercise are exec `08` (ticket 005) and the replay-divergence diagnostics are exec `10` (ticket 010); the layers above map each engaged obligation to a distinct surface.

## What to Change

### 1. D-T3 — first temporal-firewall fixture family

Require a first temporal-firewall fixture family with friendly families (holder-known work/routine timing, stale-but-believed notices, institutional records, interruption/wait effects) and adversarial families (raw-clock leakage, debug-panel leakage, omniscient due/closed labels, restamping old knowledge as fresh). No concrete fixture names (§6).

### 2. D-R1 — quantity/economy fixture family (unbundled)

Require separate fixture families for quantity/economy behavior: positive transfer/consume/split/merge/custody; adversarial hidden stock/ledger/procedure. Do not bundle the quantity/economy package with the temporal package unless a single gameplay feature genuinely needs both (spec §R-E). No fixture names (§6).

### 3. D-R5 — adversarial validation-bypass fixtures

Require adversarial fixtures that attempt to bypass validation through renamed fields, nested prose, stale strings, generated content, fixture metadata, or review-artifact text.

## Files to Touch

- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` (modify)

## Out of Scope

- **Concrete friendly/adversarial fixture files, fixture names, the inventory/economy fixture package** — future scoped specs (§6).
- **Bundling the temporal and quantity/economy packages** — forbidden unless one gameplay feature needs both (spec §R-E).
- **The authoring validations these fixtures exercise** — exec `08` (ticket 005); **replay-divergence diagnostics** — exec `10` (ticket 010).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits; minting any new fixture name or gate code.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T3 landing grep** — exec `09` names a temporal-firewall fixture family: `grep -niE 'temporal.*fixture|fixture famil' docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` resolves it.
2. **D-R1/D-R5 landing grep** — quantity/economy fixture family and validation-bypass adversarial fixtures present: `grep -niE 'quantity|economy|custody|bypass' docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` resolves them.
3. **No-bundle + no-name review** — manual review confirms no concrete fixture name entered exec `09` and the temporal/quantity packages are kept separable (spec §R-E); no new gate code.
4. **Invariants alignment review** — upholds `INV-018`/`INV-099`, preserves `FIXTURE`/`REPLAY` (no rename/weaken).

### Invariants

1. The temporal and quantity fixture families are stated as family obligations (positive + adversarial), replay deterministically, and name no concrete fixtures (`INV-018`).
2. The packages stay unbundled unless one gameplay feature needs both (spec §R-E).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing greps) plus a no-bundle/no-name and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal.*fixture|fixture famil|quantity|economy|custody|bypass' docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — confirms D-T3/D-R1/D-R5 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the no-bundle and invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented the exec `09` temporal-firewall, quantity/economy, and
validation-bypass fixture-family obligations in
`docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`. The
edit adds friendly and adversarial temporal-firewall family coverage, a
separate quantity/economy family, and adversarial validation-bypass fixture
requirements without naming concrete fixture files.

The execution-blocking owner-approval precondition in spec 0033 was satisfied
by the user's explicit request to implement the `0033EXETIETEM` ticket series.
No crate/code or fixture files were changed.

Verification:

- `grep -niE 'temporal.*fixture|fixture famil' docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `grep -niE 'quantity|economy|custody|bypass' docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `rg -n 'fixture names|concrete fixture|separable|bundle|new gate|FIXTURE-CERT|filename|filenames' docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `git diff --check`

Manual review confirmed the additions uphold deterministic replay and
truth-firewall fixture discipline, preserve `FIXTURE-CERT`/`REPLAY` without
rename or weakening, keep the temporal and quantity/economy packages separable,
and introduce no concrete fixture name or new gate code.
