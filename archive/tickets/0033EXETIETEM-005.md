# 0033EXETIETEM-005: exec 08 quantity/custody, bias & compiler-discipline validation

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` (additive quantity/custody validation, domain-pack bias-assumption review, and compiler-like proof-bearing-content discipline over the existing `DATA-CERT` posture). No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-R1, D-R4, D-R5 (report F-08, F-11, F-12), the exec-`08` slices. Exec `08` owns authoring schema, provenance, validation, forbidden forms, domain contracts, schema migration, and data certification (`DATA-CERT`) but carries **no** quantity/fungibility validation, no explicit domain-pack bias-assumption requirement, and no statement that the new payload classes (temporal/quantity/bias/staged declarations) are proof-bearing content. Without these, content packs can author omniscient quantity/custody or procedural-time conclusions and bias-by-prose. This ticket threads quantity/custody validation, bias-assumption reviewability, and compiler-like fail-closed discipline into `DATA-CERT` — additively, choosing no units/schemas/denominations.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `08` (`08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`) owns `DATA-CERT`, provenance, schema, forbidden forms, domain contracts, and migration, and carries no quantity/fungibility or bias-assumption validation (`grep -ciE 'fungib|custody|quantity|bias' docs/2-execution/08_*` → spot-checked absent for the proof obligations). The gap is real; this ticket adds proof obligations, not schemas.
2. Verified against spec 0033 §3.2 D-R1/D-R4/D-R5 and ratified upstream A09 quantity/fungibility seam, A08 practical-bias seam, and A13 authoring/compiler-discipline seam (spec `0032`). Inventory/economy schemas, unit vocabularies, money denominations, rule languages, and the bias-assumption vehicle route to future scoped specs per spec §6.
3. Shared boundary under audit: exec `08` (authoring validation) ↔ exec `09` (fixture families that exercise these validations, ticket 006) ↔ exec `10` (representation/custody/procedure-visibility diagnostics + review evidence, ticket 010) ↔ exec `11` (procedural-time + practical-bias proof that this validation must not let authors pre-conclude, ticket 007). This ticket states the authoring-validation obligation; fixtures and diagnostics live in `09`/`10`.
4. Constitutional invariants motivating this ticket, restated: `INV-102` (cognition inputs require provenance — quantity/custody/bias/temporal authored facts included); the no-scripting / content-is-possibility doctrine (foundation 09; execution 08) — content must not author behavior or omniscient outcomes. The obligations specialize fail-closed validation for the new payload classes.
5. Fail-closed-validation surface (`DATA-CERT`): the obligations require (a) content validation for quantity-bearing and fungible/partly-fungible entities to preserve identity, quantity, custody, ownership/control, provenance, split/merge lineage, transformation, spoilage/consumption, reservation, transfer, concealment/discovery, and institution-visible record effects as appropriate to the authored domain, failing closed on ambiguous fungibility, implicit global pools, untracked disappearance/creation, balance edits without ledger ancestry, and authored facts that make an actor/institution know quantity/custody without a modeled channel; (b) domain-pack bias/social-harm assumptions to be explicit, validated, and reviewable rather than implied by prose; (c) the explicit statement that temporal claims, procedural-time records, quantity/custody records, bias/social-harm assumption packets, and staged-abstraction declarations are proof-bearing authored content that must be structurally validated, failing closed on malformed provenance, missing source channels, ambiguous authority category, hidden-truth labels in prose fields, restamped freshness, implicit global state, and unreviewable assumptions. This strengthens fail-closed validation and epistemic-leakage prevention; it chooses no schema, rule language, or error format and introduces no determinism change.

## Architecture Check

1. Exec `08` is the correct single home: it already owns `DATA-CERT` and the forbidden-forms posture, so quantity/custody validation, bias-assumption reviewability, and proof-bearing-content discipline are specializations of contracts `08` already carries. Splitting them would divorce the authoring gate from the new payload classes it must reject-closed.
2. No backwards-compatibility aliasing/shims: additive obligations over `DATA-CERT`; no rename, no weakening, no concrete schema/units/denominations chosen.

## Verification Layers

1. `INV-102` / no-scripting quantity-custody validation → invariants alignment check: exec `08` requires quantity/fungibility/custody preservation and fail-closed behavior on the named ambiguities.
2. No-scripting / fallible-institution bias discipline → invariants alignment check: domain-pack bias/social-harm assumptions are explicit, validated, reviewable — not authored omniscient outcomes.
3. Fail-closed proof-bearing content → invariants alignment check: temporal/procedural/quantity/bias/staged declarations are structurally validated, failing closed on the named bypasses.
4. Documentation-only doctrine ticket: the fixture families exercising these validations are exec `09` (ticket 006) and the diagnostics/review evidence are exec `10` (ticket 010); the layers above map each engaged obligation to a distinct surface.

## What to Change

### 1. D-R1 — quantity / granularity / fungibility / custody validation

Require content validation for quantity-bearing and fungible/partly-fungible entities to preserve identity, quantity, custody, ownership/control, provenance, split/merge lineage, transformation, spoilage/consumption, reservation, transfer, concealment/discovery, and institution-visible record effects as appropriate to the authored domain; require fail-closed behavior on ambiguous fungibility, implicit global pools, untracked disappearance/creation, balance edits without ledger ancestry, and authored facts that make an actor/institution know quantity/custody without a modeled channel. Choose no units/schemas/denominations (§6). Do not bundle with the temporal fixture package (spec §R-E; see ticket 006).

### 2. D-R4 — domain-pack bias/social-harm assumptions are explicit and reviewable

Require domain-pack assumptions about bias/social harm to be explicit, validated, and reviewable rather than implied by prose. The Phase-4 practical-bias proof itself is exec `11` (ticket 007); concrete domain-pack bias assumptions and evaluation criteria route to future scoped specs (§6).

### 3. D-R5 — compiler-like discipline for proof-bearing content

State explicitly that temporal claims, procedural-time records, quantity/custody records, bias/social-harm assumption packets, and staged-abstraction declarations are proof-bearing authored content and must be structurally validated; require fail-closed behavior for malformed provenance, missing source channels, ambiguous authority category, hidden-truth labels in prose fields, restamped freshness, implicit global state, and unreviewable assumptions. Choose no schemas, rule languages, or error formats (§6). Optional source notes for compiler-like validation / policy-as-code in exec `13` are implementer-discretion (spec §6), not part of this ticket.

## Files to Touch

- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` (modify)

## Out of Scope

- **Inventory/economy schemas, units, denominations, rule languages, error formats, bias-assumption vehicle** — future scoped specs (§6).
- **Quantity/temporal fixture families** — exec `09` (ticket 006); **diagnostics/review evidence** — exec `10` (ticket 010); **Phase-4 practical-bias proof** — exec `11` (ticket 007).
- **Optional exec `13` source notes** — implementer-discretion, not ticketed (spec §6).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits; minting any new gate code.

## Acceptance Criteria

### Tests That Must Pass

1. **D-R1 landing grep** — exec `08` carries quantity/custody/fungibility validation: `grep -niE 'fungib|custody|quantity|ledger ancestry' docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` resolves it.
2. **D-R4/D-R5 landing grep** — bias-assumption reviewability and proof-bearing-content discipline present: `grep -niE 'bias|proof-bearing|fail[- ]closed' docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` resolves both.
3. **Mechanism-token boundary review** — no units, schema fields, denominations, rule language, or error format entered exec `08`; no new gate code.
4. **Invariants alignment review** — upholds `INV-102` and the no-scripting doctrine, preserves `DATA-CERT` (no rename/weaken).

### Invariants

1. Quantity/custody/fungibility authored facts preserve lineage and fail closed on implicit global pools / untracked creation / unprovenanced knowledge (`INV-102`, no-scripting).
2. Bias/social-harm assumptions and the new payload classes are proof-bearing, structurally validated, fail-closed content — never authored omniscient outcomes (no-scripting).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing greps) plus a mechanism-token-boundary and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'fungib|custody|quantity|bias|proof-bearing|fail[- ]closed' docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — confirms D-R1/D-R4/D-R5 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the mechanism-token-boundary and invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented the exec `08` quantity/custody, practical-bias, and
proof-bearing-content validation obligations in
`docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`. The
edit adds fail-closed validation for quantity-bearing and fungible/partly
fungible content, explicit reviewability for domain-pack bias/social-harm
assumptions, and structural validation for temporal, procedural-time,
quantity/custody, bias/social-harm, and staged-abstraction authored payloads.

The execution-blocking owner-approval precondition in spec 0033 was satisfied
by the user's explicit request to implement the `0033EXETIETEM` ticket series.
No crate/code or fixture files were changed.

Verification:

- `grep -niE 'fungib|custody|quantity|ledger ancestry' docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
- `grep -niE 'bias|proof-bearing|fail[- ]closed' docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
- `rg -n 'unit vocabulary|schema field|denomination|rule language|error format|new gate|DATA-CERT|outcome|quest|morality oracle' docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
- `git diff --check`

Manual review confirmed the additions uphold `INV-102` and no-scripting
doctrine, preserve `DATA-CERT` without rename or weakening, and introduce no
unit vocabulary, schema field, denomination, rule language, error format, or
new gate code.
