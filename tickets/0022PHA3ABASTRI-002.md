# 0022PHA3ABASTRI-002: 0021 acceptance-report corrections and §7-checklist parity guard

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — evidence document (`reports/0021_ord_life_cert_scoped_acceptance.md`), test-oracle guard (`anti_regression_guards.rs`)
**Deps**: `specs/0022_PHASE_3A_BASELINE_TRIAGE_INTEGRITY_EMBODIED_DEBUG_QUARANTINE_ADVERSARIAL_GATE_RESTORATION_AND_LOCK_BEHAVIORAL_PROOF_HARDENING_SPEC.md`

## Problem

Two evidence-honesty gaps in `reports/0021_ord_life_cert_scoped_acceptance.md`
(`ORD-HARD-102`, `ORD-HARD-119`): §2 is titled "Scheduled Ratchet And Guard Outputs"
but its body covers ticket-002's hidden-truth provenance guards, while the 0021 spec's
§7 item 2 evidence (the scheduled mutation job's first live-ratchet run) is recorded
nowhere; and §15 omits the R-29 Guard Vacuity risk-register addition that §7 item 15
required quoting. Nothing machine-checks a report's section claims against its spec's
§7 item list, so costume-mislabels pass silently.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: report §2 heading reads `## 2. Scheduled Ratchet And Guard
   Outputs` with `Primary ticket: archive/tickets/0021PHA3APOSREB-002.md` (the
   harness-provenance ticket); `grep R-29 reports/0021_ord_life_cert_scoped_acceptance.md`
   returns zero hits; §15 exists (`## 15. Risk Register And Conformance Index`).
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-102`/`119`
   (operator-verified evidence) and `archive/specs/0021_…HARDENING_SPEC.md` §7 items 2
   and 15 (the obligations the report under-evidences).
3. Shared contract under audit: the acceptance-artifact evidence map — spec §7 item IDs
   ↔ report sections — consumed by future audit passes as the honesty baseline.
4. Constitutional motivation restated: R-27 (acceptance-evidence reachability
   overstatement) and docs/2-execution/10's rule that review artifacts record what was
   actually run.
5. This ticket touches an evidence-honesty surface only — no validation, epistemic, or
   replay code path changes; the new guard is additive test-oracle machinery that
   fails closed on a missing checklist mapping.
6. Adjacent contradiction classification: the *performance* of the baseline triage that
   §6 of the report mislabels is owned by `0022PHA3ABASTRI-001`; this ticket corrects
   the record and adds the parity lock — required consequence split, not a bug.
7. Change rationale (no silent retcon): the report edits correct claims the 0022 audit
   proved misleading; each correction cites its `ORD-HARD` finding inline in the report
   (the 0021 deviation is recorded explicitly, per spec §5.6 and `ORD-HARD-099`'s
   required correction).

## Architecture Check

1. A checklist guard mapping spec §7 item IDs to required evidence strings in the
   report makes the per-item honesty machine-checkable — cheaper and more durable than
   relying on human §7-by-§7 review, which is exactly what missed both findings. The
   alternative (prose-only correction) leaves the class open.
2. No backwards-compatibility aliasing/shims: the report is corrected in place; no
   parallel "errata" file.

## Verification Layers

1. Evidence honesty (R-27) -> manual review + grep-proof: §2 retitled to its actual
   content; an explicit §7-item-2 record added (the scheduled run's result, or an
   honest "first live-ratchet run pending post-merge" deviation note); §15 quotes the
   R-29 addition.
2. Checklist parity -> codebase grep-proof: the new guard maps each 0021 §7 item to an
   evidence anchor in the report and fails on a missing mapping; synthetic negative
   (an item with no anchor) fires.
3. Forward extension -> invariants alignment check: the guard's mapping table is
   data-driven so `0022PHA3ABASTRI-014` can register the 0022 artifact without
   restructuring (spec §7 item 12 requires running this guard against the 0022
   artifact itself).

## What to Change

### 1. Correct `reports/0021_ord_life_cert_scoped_acceptance.md`

Retitle §2 to match its content (hidden-truth provenance guards); add the explicit §7
item 2 record — the scheduled mutation job's first live-ratchet outcome if one has run
by implementation time, otherwise an honest "pending post-merge; first scheduled run
not yet recorded" deviation line; amend §15 to quote the R-29 risk-register addition
and the five unquoted conformance-row actions; record the `ORD-HARD-099` triage
deviation explicitly under "Deviations From Plan" (cross-referencing
`0022PHA3ABASTRI-001` as the corrective work).

### 2. Add the §7-checklist parity guard

In `crates/tracewake-core/tests/anti_regression_guards.rs`: a guard holding a mapping
from 0021 §7 item IDs (1–17) to required evidence anchor strings in the 0021 report,
failing on any item with no resolving anchor; built data-driven so the 0022 spec's §7
items 1–15 → 0022 artifact mapping can be registered by the capstone ticket. Synthetic
negative: a checklist row whose anchor is absent must fail.

## Files to Touch

- `reports/0021_ord_life_cert_scoped_acceptance.md` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Performing the baseline triage itself (`0022PHA3ABASTRI-001`).
- The 0022 acceptance artifact and its checklist registration (`0022PHA3ABASTRI-014`).
- Any CI workflow change (`0022PHA3ABASTRI-003`).
- Rewriting report sections beyond the named corrections (the report's verified-honest
  claims stay untouched).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test anti_regression_guards` — including the new
   §7-checklist parity guard and its firing synthetic negative.
2. `grep -c "R-29" reports/0021_ord_life_cert_scoped_acceptance.md` ≥ 1; §2's title no
   longer claims scheduled-ratchet evidence; a §7-item-2 record (run result or pending
   deviation) is grep-locatable.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every 0021 §7 item maps to a verifiable evidence anchor in the report (or an
   explicit recorded deviation) — no costume sections.
2. The report's previously-verified-honest content (manifest, EMERGE-OBS, conformance
   rows) is byte-preserved except where a named finding requires correction.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — §7-checklist parity guard
   + synthetic negative.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `grep -n "R-29\|Deviations" reports/0021_ord_life_cert_scoped_acceptance.md`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
