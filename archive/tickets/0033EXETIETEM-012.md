# 0033EXETIETEM-012: exec 00 index routing map & staged-incompleteness authority (capstone)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` (additive temporal / Block-R / staged-declaration routing subsection and the staged-incompleteness authority rule; mints no new gate code). No crate/code, no fixtures.
**Deps**: 0033EXETIETEM-001, -002, -003, -004, -005, -006, -007, -008, -009, -010, -011 (index capstone — the routing map points to the proof homes each of those tickets authors; it lands once they exist so its pointers resolve). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T1, D-S1 (report F-01, F-13), the exec-`00` slices. Exec `00` owns execution authority, the canonical gate taxonomy, observation obligations, and label-class reconciliation, but carries **no** map from the temporal/Block-R/staged doctrine to its execution homes, and no authority-level staged-proof rule: verified 0 `temporal`/`INV-112`/`staged` matches in exec `00` at `c70d119`. Without the routing map a future implementer cannot find, from the index, where temporal proof, Block-R proof, and staged declarations land; without the authority rule, staged proof can certify an unimplemented future feature by implication. This is the **batch capstone**: it adds a compact routing subsection pointing to the owners and the staged-incompleteness authority rule, and lands once the homes exist so its pointers resolve.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `00` (`00_EXECUTION_INDEX_AND_AUTHORITY.md`) owns the gate taxonomy (`P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`, `EMERGE-OBS`), observation obligations, and label-class reconciliation, and carries no temporal/staged routing or authority rule (`grep -ciE 'temporal|INV-112|staged' docs/2-execution/00_*` → 0). The gap is real; this ticket adds a pointer subsection and the authority rule, it does not restate the homes' obligations.
2. Verified against spec 0033 §3.1 D-T1 and §3.3 D-S1, and §6 (no new gate code; concrete temporal values are lower-tier). The routing map is a pointer; the staged rule is acceptance-honesty doctrine at index altitude.
3. Shared boundary under audit: exec `00` indexes the whole cascade — this is the **batch capstone**, `Deps` on every sibling whose home it points to (001 `04`, 002 `05`, 003 `06`, 004 `07`, 005 `08`, 006 `09`, 007 `11`, 008 `12`, 009 `02`, 010 `10`, 011 `03`) so its pointers resolve to landed obligations. It shares the D-T1/D-S1 deliverables with exec `03` (ticket 011) — this ticket owns the `00` index/routing + authority-rule slice; `03` owns the certification-sequence-placement slice.
4. Constitutional invariant motivating this ticket, restated: `INV-112` — the index records that execution **operationalizes** `INV-112` into named proof homes (firewall & holder-known time → `04`; scheduler trigger-vs-plan & budgets → `05`; routines → `06`; TUI rendering & time controls → `07`; authoring validation → `08`; fixture families → `09`; diagnostics/review artifacts → `10`; institutions/procedural time → `11`; LOD/time-acceleration → `12`). The map points; it does not restate. The staged rule (Driver S) is an authority-level acceptance-honesty rule, not a new gate.
5. Acceptance / index-authority surface: the routing map must not mint a new gate code and must state that concrete temporal values/terminology are lower-tier decisions; the staged-incompleteness authority rule states that staged proof is allowed only when the staged abstraction is declared and bounded, and must not certify the unimplemented future feature by implication. This adds no enforcement of its own (the homes enforce) and no new gate code, and introduces no determinism change.

## Architecture Check

1. Exec `00` is the correct home for an index routing map and an authority-level rule, and a *pointer* map (not a doctrine restatement) is the correct shape — mirroring the architecture cascade's A00 capstone (`0032ARCTIETEM-013`). The map routes readers to the real owners the sibling tickets author; the staged rule belongs at index/authority altitude because it governs what any staged acceptance may claim.
2. No backwards-compatibility aliasing/shims: additive routing pointer + authority rule; no behavior change, no duplication of owners' obligations, no new gate code.

## Verification Layers

1. `INV-112` execution routing map (D-T1) → invariants alignment check: exec `00` gains a compact subsection stating execution operationalizes `INV-112` and pointing to `04`/`05`/`06`/`07`/`08`/`09`/`10`/`11`/`12` as homes.
2. Pointer-resolves check → codebase grep-proof: each home the routing map names carries the obligation its sibling ticket authored (the sibling landing greps), so the index pointers are not dangling.
3. Staged-incompleteness authority rule (D-S1) → invariants alignment check: exec `00` states staged proof is allowed only when the abstraction is declared and bounded, and must not certify the unimplemented future feature by implication; mints no new gate code (spec §R-G).
4. Documentation-only doctrine ticket: the owners' obligations are the sibling tickets; the layers above map the engaged invariant, the pointer integrity, and the staged rule to distinct surfaces.

## What to Change

### 1. D-T1 — temporal / Block-R / staged-declaration routing subsection

Add a compact routing subsection to exec `00` pointing each proof surface to its execution home: temporal firewall & holder-known time → `04`; scheduler trigger-vs-plan & budgets → `05`; routines → `06`; TUI rendering & time controls → `07`; authoring validation → `08`; fixture families → `09`; diagnostics/review artifacts → `10`; institutions/procedural time → `11`; LOD/time-acceleration → `12`. It must not mint a new gate code and must state that concrete temporal values/terminology are lower-tier decisions.

### 2. D-S1 — staged-incompleteness authority rule

Add the authority-level rule that staged proof is allowed only when the staged abstraction is declared and bounded, and staged proof must not certify the unimplemented future feature by implication. Mint no new gate label or observation-obligation code (spec §R-G).

## Files to Touch

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` (modify)

## Out of Scope

- **Restating the homes' obligations in `00`, or making `00` the sole home** — the subsection points to owners (sibling tickets).
- **The certification-sequence placement** — exec `03` (ticket 011).
- **Minting a new gate code / observation-obligation code** — forbidden (spec §6, §R-G).
- **Concrete temporal values/terminology** — lower-tier (reference/spec, §6).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T1 landing grep** — exec `00` carries the routing subsection pointing to homes: `grep -niE 'temporal.*(routing|home|INV-112)|Block-R|staged' docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` resolves the map.
2. **Pointer-integrity review** — each home the map names (`04`/`05`/`06`/`07`/`08`/`09`/`10`/`11`/`12`) carries its sibling ticket's landed obligation; no pointer dangles.
3. **D-S1 landing grep** — staged-incompleteness authority rule present: `grep -niE 'staged.*(proof|abstraction|declared|bounded)' docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` resolves it.
4. **No-new-gate + invariants review** — no new gate code minted; routing map states concrete temporal values are lower-tier; upholds `INV-112` and the staged discipline.

### Invariants

1. Exec `00` records the cascade as a pointer to execution homes, not as the sole home, and mints no new gate code (`INV-112`).
2. Staged proof is bounded and declared, never certifying an unimplemented future feature by implication (staged-incompleteness discipline).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine capstone ticket; verification is command-based (landing greps) plus pointer-integrity, no-new-gate, and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal.*(routing|home|INV-112)|Block-R|staged' docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — confirms D-T1/D-S1 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the pointer-integrity and invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented the exec `00` temporal/Block-R/staged-proof routing capstone in
`docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`. The edit adds a compact
routing map from each proof surface to its execution home, records that
concrete temporal/mechanism values remain lower-tier decisions, and adds the
authority-level staged-proof rule that staged abstractions must be declared,
bounded, and must not certify unimplemented future features by implication.

The execution-blocking owner-approval precondition in spec 0033 was satisfied
by the user's explicit request to implement the `0033EXETIETEM` ticket series.
No crate/code or fixture files were changed.

Verification:

- `grep -niE 'temporal.*(routing|home|INV-112)|Block-R|staged' docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `grep -niE 'staged.*(proof|abstraction|declared|bounded)' docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `rg -n 'new gate|gate code|observation-obligation|concrete temporal|lower-tier|04_TRUTH|05_TRANSACTION|06_ORDINARY|07_EPISTEMIC|08_DATA|09_GOLDEN|10_TESTING|11_INSTITUTIONS|12_DEFERRED' docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- Pointer integrity greps across exec `04` through `12` confirmed every routed
  home carries the sibling ticket's landed obligation.
- `git diff --check`

Manual review confirmed the route map is a pointer to execution homes rather
than a sole-home restatement, concrete values remain lower-tier, and no new
gate code, gate label, or observation-obligation code was minted.
