# 0017PHA3ATICLED-010: Audit-history corrections and conformance documentation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — markdown documentation only
**Deps**: `0017PHA3ATICLED-008`, `0017PHA3ATICLED-009` (leaf set — transitively covers `-001`…`-007`; the conformance rows cite every implemented surface); `specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (ORD-HARD-034 + §6)

## Problem

The 0016 acceptance report (`reports/0016_ord_life_cert_scoped_acceptance.md`) overstates three delivered proofs: it claims a synthetic V0→V1 upcast fixture that does not exist (`EVENT_SCHEMA_REGISTRY` holds a single V1 entry with `CurrentNoMigrationRequired`; only the unknown-version loud-rejection path is tested), presents `embodied_workplace_availability_reflects_belief_not_truth_001` as proving belief-over-truth when the fixture tested absence-without-belief, and names two content negative fixtures that were delivered as inline unit tests. Per 0016 §6's own precedent, audit history must record corrections rather than silently inherit them (INV-105 in spirit: acceptance artifacts are authoritative diagnostic data). Separately, spec 0017 §5.6/§6 require conformance-index rows for the new enforcement surfaces and two execution/architecture-tier clarifications.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: `events/envelope.rs::EVENT_SCHEMA_REGISTRY` has exactly one entry (`EVENT_SCHEMA_V1`, `EventSchemaMigration::CurrentNoMigrationRequired`); the divergence fixture seeded `initial_beliefs: Vec::new()` pre-`-006`; the named content negative fixtures exist only as inline `validate.rs` tests; `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` carries the 0016 lock-layer rows this ticket extends. User-approved either-or (option a): the schema contract is recorded as loud-rejection-only — no upcast fixture is implemented.
2. Spec 0017 §ORD-HARD-034 (required correction), §6 (documentation corrections), §5.6 (conformance rows: tick-charge attribution authority extended to action-emitted deltas; projection freshness rule + provenance-class audit; shrunken `WorldNoOp` allowlist contract; believed-access embodied availability; numeric field-policy registry; shared open-duration authority; generative lock tier including the in-tree-generator decision and any Kani skip-without-prejudice).
3. Cross-artifact boundary under audit: the acceptance-report + conformance-index pair as the durable audit-history contract — what later audits trust without re-deriving.
4. INV-105 restated: diagnostics and acceptance evidence are authoritative data; an artifact claiming unproven evidence is a decorative proof surface. Corrections are dated and additive — prior text is corrected visibly, not rewritten silently.

## Architecture Check

1. A dated correction section appended to the 0016 report (mirroring how 0016 itself recorded overturned 0015 claims in the conformance index) keeps audit lineage append-only and auditable — cleaner than editing the original claims in place, which would itself be the silent-rewrite failure mode ORD-HARD-034 names.
2. No backwards-compatibility aliasing/shims: not applicable — documentation-only; no doctrine amendment (both doc-tier additions are execution/architecture-tier clarifications, not invariant changes).

## Verification Layers

1. Audit-history correction -> manual review + grep-proof: the 0016 report contains a dated `## 2026 correction (spec 0017)` section naming the three overstatements with citations to ORD-HARD-034 and the resolving tickets.
2. Conformance completeness -> grep-proof: the conformance index contains one row per §5.6 surface, each citing the implementing symbol/test landed by tickets `-001`…`-009`.
3. Tier discipline -> manual review: the execution doc 06 clause (single-charge-per-tick including action-emitted awake deltas) and the architecture doc 03 note (one projection freshness rule for both consumers) read as clarifications; no `INV-NNN` text is touched.

## What to Change

### 1. 0016 acceptance report correction

Append the dated correction section: (a) upcast-fixture claim corrected to the loud-rejection-only schema contract (option a), citing `unsupported_event_schema_version_replay_fails_loudly`; (b) the divergence-fixture overstatement, citing ORD-HARD-029 and ticket `-006`'s real fixture pair; (c) the inline-vs-named negative-fixture delivery note, citing ticket `-007`'s convention statement.

### 2. Conformance index rows

Add the §5.6 rows (each citing its implementing symbol/test); add overturned-claim rows for the two 0016 overstatements, mirroring the existing 0015-overturn rows.

### 3. Doc-tier clarifications

`docs/2-execution/06_…`: the single-charge clause. `docs/1-architecture/03_…`: the freshness-rule conformance note.

## Files to Touch

- `reports/0016_ord_life_cert_scoped_acceptance.md` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` (modify)

## Out of Scope

- Any code, test, fixture, or CI change (implementation surfaces are tickets `-001`…`-009`).
- The 0017 acceptance artifact itself (ticket `-011`).
- `docs/4-specs/SPEC_LEDGER.md` and the `archive/specs/` move — deferred to spec acceptance/archival per the hardening-series convention.
- Amending any constitutional invariant.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -q "spec 0017" reports/0016_ord_life_cert_scoped_acceptance.md` — dated correction section present.
2. `grep -c "0017" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — conformance rows present (count matches the §5.6 surface list).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` (unchanged-passing; docs-only)

### Invariants

1. Audit history is append-only: corrections are dated additions citing the overturning finding, never in-place rewrites of prior claims.
2. Every 0017 enforcement surface has a conformance-index row citing a real, landed symbol or test.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n "2026 correction" reports/0016_ord_life_cert_scoped_acceptance.md`
2. `cargo test --workspace`
