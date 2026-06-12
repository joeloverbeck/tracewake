# 0024PHA3ACONSCH-003: Risk-register extensions — presence-check fallback, self-satisfying citation, type-convention census watch

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — docs only (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`).
**Deps**: 0024PHA3ACONSCH-001, 0024PHA3ACONSCH-002

## Problem

Spec 0024 §6 prescribes recording the two new guard-vacuity shapes this pass
surfaced so future audits and authors recognize them: under R-29, the
**presence-check fallback arm** (an enumerated witness repair whose default arm
degrades to name-presence — `ORD-HARD-141`) and the **self-satisfying citation** (a
content witness pointed at the definition site of the thing it witnesses —
`ORD-HARD-142`); and under R-28, a Watch note that **type-convention censuses**
(`String`-typed field scans, single-file `include_str!` perimeters, hand-picked
token subsets of a public API) are hand-maintained for everything outside the
convention (`ORD-HARD-144`/`151`/`157`). Per the honest-evidence placement rule
(and the 0023 precedent), these doc rows land only once the shapes they describe
have been closed — hence the Deps.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: the three shapes exist as described
   (the `_ =>` arm in `meta_lock_live_witness_count`; the `debug_only_diagnostics`
   definition-site cite; the `String`-typed `discover_schema_fields` census /
   single-file `include_str!("render.rs")` perimeter / two-token apply scan) — all
   operator-verified per spec 0024 §8. Tickets -001/-002 close the first two; the
   census instances are closed by -005/-008/-009.
2. Verified against spec 0024 §6 (first bullet) and the current
   `docs/3-reference/01_DESIGN_RISK_REGISTER.md` R-28/R-29 sections, which carry the
   0023-era extensions (literal witness count, binding laundering) but not these
   shapes.
3. Cross-artifact boundary: the risk register's R-28/R-29 symptom lists are the
   vocabulary future hardening passes grep against — the entries must name the
   shapes with the same terminology the closing tickets use (presence-check fallback
   arm; self-satisfying citation; type-convention census), so audit prompts and
   guard comments cross-resolve.
4. Doctrine restated: the register is reference-tier (`docs/3-reference/`) — it
   records risk shapes, it does not amend doctrine; no invariant's force changes.

## Architecture Check

1. Extending the existing R-28/R-29 entries (rather than minting new risk IDs) keeps
   the register's shape stable and the lineage's grep anchors (`R-28`, `R-29`)
   intact — consistent with how 0023 §6 extended the same entries.
2. No backwards-compatibility aliasing/shims: documentation prose only.

## Verification Layers

1. Recording completeness → grep-proofs: the three shape names present under the
   correct risk entries (codebase grep-proof, exact strings).
2. Honest-evidence placement → Deps gating: the rows land after -001/-002 close the
   described shapes (manual review of merge order).
3. Single-layer ticket: docs-only; no further layer mapping applicable — the content
   it records is proven by the depended-on tickets' test suites.

## What to Change

### 1. R-29 extension

Add two symptom bullets: the presence-check fallback arm (`ORD-HARD-141`) and the
self-satisfying citation (`ORD-HARD-142`), each with a one-line description naming
the failure mechanism.

### 2. R-28 Watch note

Add the type-convention census watch (`ORD-HARD-144`/`151`/`157`): membership
derivations scoped to a type or naming convention are hand-maintained for everything
outside the convention; enumerate the three instances this pass closed.

## Files to Touch

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- Conformance-index rows (`docs/1-architecture/00_*`) — capstone ticket -014, per
  spec §6's landed-symbols rule.
- The `docs/2-execution/07` quarantine/staging record — ticket -009.
- Any guard or production code.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "presence-check fallback" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
   and `grep -n "self-satisfying citation" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
   each match exactly one R-29 bullet.
2. `grep -n "type-convention census" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
   matches one R-28 Watch note citing `ORD-HARD-144`/`151`/`157`.
3. `cargo test --workspace` remains green (doc_invariant_references and any
   register-citing guards unaffected).

### Invariants

1. The register records risk shapes; no doctrine is amended, no invariant's scope
   changes.
2. Every recorded shape cites the ORD-HARD finding(s) that closed it.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n "presence-check fallback\|self-satisfying citation\|type-convention census" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-12

Extended `docs/3-reference/01_DESIGN_RISK_REGISTER.md` with the required R-28
Watch note and R-29 symptom bullets:

```text
340:- **Watch:** A type-convention census can look derived while remaining hand-maintained for everything outside its convention: examples include `String`-typed field scans, single-file `include_str!` perimeters, and hand-picked token subsets of a public API (`ORD-HARD-144`/`151`/`157`).
351:- **presence-check fallback:** An enumerated witness repair can still become decorative when its default arm degrades to lock-name or negative-name presence instead of measured scan execution (`ORD-HARD-141`).
352:- **self-satisfying citation:** A content witness can be unfalsifiable when it cites the definition site of the field or artifact it claims to prove, rather than an independent write, consume, or behavior path (`ORD-HARD-142`).
```

Verification:

1. `grep -n "presence-check fallback\|self-satisfying citation\|type-convention census" docs/3-reference/01_DESIGN_RISK_REGISTER.md` matched the required three rows.
2. `cargo test --workspace` passed.

No code, doctrine, conformance-index rows, or quarantine/staging records were
changed by this docs-only ticket.
