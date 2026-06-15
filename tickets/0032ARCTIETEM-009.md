# 0032ARCTIETEM-009: A09 quantity/granularity/fungibility seam

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` (quantity/granularity/fungibility representation seam). No crate/code, no fixtures.
**Deps**: None (ticket-internal; A09 is not a merge hub in this batch). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-R2 (report R2). Foundation `06` owns ordinary actions, food, money/payment/custody, and physical/resource consequences while routing concrete inventory/economy mechanics below foundation; the completeness determination routes representation seams to A09. A09 already covers ordinary life, places, property/custody, finite food, local-economy placeholders, wages, and event-sourced economy — but it does **not** explicitly say how unique items differ from fungible quantities, capacities, or ledgers at architecture altitude, nor name the lineage expectations for quantity operations.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A09 owns ordinary life, property/custody, finite food, local-economy placeholders, wages, event-sourced economy — but states no quantity/granularity/fungibility representation-class distinction and no split/merge/consume lineage expectation. This ticket adds the representation seam; it does not reinvent A09's property/economy contracts.
2. Verified against spec 0032 §3.2 D-R2 and source report §5 Finding R2 / foundation `06`. D-R2's home is A09. Additive; relaxes nothing. Data structures, unit vocabularies, money denominations, inventory schemas, and economy formulas route to execution/scoped specs per spec §6 (and must not bundle with the temporal proof package per report §6.3).
3. Shared boundary under audit: A09 (ordinary-life / property / economy representation). A09 is touched only by this ticket in the batch — no merge hub. The seam constrains future scoped inventory/economy specs without authoring them.
4. Constitutional invariants motivating this ticket, restated: `INV-009` (meaningful state changes require events), `INV-010` (every event needs a cause model), `INV-013` (meaningful events leave traces), and `INV-014` (trace removal — hide/burn/lose — is eventful). D-R2's lineage expectation is these invariants applied to quantity operations: split/merge/consume/spoil/reserve/share/transfer/hide/discover/pay/refuse/reimburse are eventful, cause-bearing, trace-leaving transformations, not numeric overwrites.
5. Deterministic-replay / provenance surface (governed here; enforcement deferred to execution `08`/`09`/`10` and future scoped inventory/economy specs): quantity operations preserve event ancestry, custody/ownership/procedure context, and holder-known visibility; fungible aggregation is allowed only when it does not erase information needed for action validation, provenance, replay, wrong belief, lead interpretation, institutional record, or later promotion; projections may summarize quantities for UI/debug, but the authoritative lineage for replay/disputes cannot be replaced by a display total. This is `INV-009`/`INV-010`/`INV-013` applied to quantity; it adds doctrine only, no nondeterminism or lineage-erasure path.

## Architecture Check

1. A09 is the correct home: it already owns property/custody/economy, so the quantity/fungibility representation classes are a specialization of contracts A09 carries. A separate home would split quantity representation from the property/custody it constrains.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine naming representation classes and lineage expectations, not schemas.

## Verification Layers

1. `INV-009`/`INV-010`/`INV-013` quantity lineage (D-R2) → invariants alignment check: A09 names unique/lot/stock/capacity/debt/wage/ledger as distinct representation classes and requires split/merge/consume/… operations to preserve event ancestry, custody/procedure context, and holder-known visibility.
2. Fungible-aggregation constraint → invariants alignment check: A09 permits aggregation only when it does not erase info needed for validation/provenance/replay/wrong-belief/lead/record/promotion; projections may summarize but not replace authoritative lineage.
3. Single doc, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `08`/`09`/`10`); the layers above map each engaged invariant to a distinct alignment proof.

## What to Change

### 1. D-R2 — quantity/granularity/fungibility seam in A09

Add to `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`:

- unique objects, countable lots, divisible stocks, capacities, debts, wages, and ledgers are separate representation classes because they preserve different identity/provenance/custody constraints;
- operations such as split, merge, consume, spoil, reserve, share, transfer, hide, discover, pay, refuse, and reimburse must preserve event ancestry, custody/ownership/procedure context, and holder-known visibility;
- fungible aggregation is allowed only when it does not erase information needed for action validation, provenance, replay, wrong belief, lead interpretation, institutional record, or later promotion;
- projections may summarize quantities for UI/debug, but the authoritative lineage needed for replay and disputes cannot be replaced by a display total.

Do not choose data structures, unit vocabularies, money denominations, inventory schemas, or economy formulas.

## Files to Touch

- `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` (modify)

## Out of Scope

- **Data structures, unit vocabularies, money denominations, inventory schemas, economy formulas** — execution/scoped specs (spec §6).
- **Bundling with the temporal proof package** — explicitly forbidden unless a single gameplay feature requires it (report §6.3); this ticket stages only the architecture seam.
- **Split/merge/custody-lineage proof procedures** — execution `08`/`09`/`10` (spec §6, V4).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-R2 landing grep** — A09 carries the quantity/fungibility seam: `grep -niE "unique|lot|stock|capacity|ledger|fungible|split|merge|lineage" docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` resolves the representation classes and operations.
2. **Lineage-preservation review** — operations preserve event ancestry/custody/visibility and aggregation may not erase replay/validation/provenance information.
3. **Invariants alignment review** — upholds `INV-009`/`INV-010`/`INV-013` (quantity operations are eventful, cause-bearing, trace-leaving); no schema/unit/denomination token introduced.

### Invariants

1. Unique/lot/stock/capacity/debt/wage/ledger are distinct representation classes; quantity operations preserve event ancestry, custody/procedure context, and holder-known visibility (`INV-009`/`INV-010`/`INV-013`).
2. Fungible aggregation may not erase information needed for validation/provenance/replay/wrong-belief/lead/record/promotion; a display total never replaces authoritative lineage.

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing grep) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "unique|lot|stock|capacity|ledger|fungible|split|merge|lineage" docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — confirms D-R2 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the invariants-alignment review.`
