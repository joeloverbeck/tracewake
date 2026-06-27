# 0053FOUCONFIF-008: Post-closure conformance doc-truthing (cross-cutting docs)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — live-conformance doc edits only (`docs/1-architecture/04`, `docs/1-architecture/10`, `docs/2-execution/05`, `docs/2-execution/06`, `docs/2-execution/07`, `docs/3-reference/00`, `docs/3-reference/01`)
**Deps**: 0053FOUCONFIF-002, 0053FOUCONFIF-003, 0053FOUCONFIF-004, 0053FOUCONFIF-005, 0053FOUCONFIF-006, 0053FOUCONFIF-007

## Problem

Spec 0053 §6.2: do **not** edit conformance rows first. After F5-01…F5-06 are implemented and executed at one exact commit, the live conformance docs must be truthed to name the sealed constructors, the sealed interval product, the resolved no-human-day classification, and the negative/mutation proofs — so the doc pack stops describing a public raw constructor / forgeable product as the current evidence. This is the cross-cutting docs ticket: it lands atomically once all upstream implementation tickets ship, citing each resealed surface, and must not run ahead of the executable witnesses (§6.2, §9 step 8).

## Assumption Reassessment (2026-06-26)

1. The seven target docs exist (verified this session): `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`, `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`, `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`, `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`, `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`, `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`, `docs/3-reference/01_DESIGN_RISK_REGISTER.md`. Ref 00 + ref 01 are also edited by ticket 002 (doctrine strengthening); this ticket `Deps: 002` so the two edits to each stack in order (sequential coordination surface, not a parallel merge hub).
2. Spec authority: `specs/0053_…_HARDENING_SPEC.md` §6.2 (the per-doc evidence updates + the four-question evidence-row rule), §9 step 8 (after executable closure only). Sibling precedent: the 0052 line truthed analogous conformance rows after closure; this repeats the discipline for the fifth-pass seals.
3. Cross-artifact boundary under audit: the live conformance evidence rows across architecture/execution/reference tiers and the resealed code surfaces they describe (004 sealed bootstrap, 005 sealed interval product, 006 token-gated debug, 007 closed survivor family, 003 governance). Each updated row must cite the real post-implementation symbol/proof, not a public raw constructor.
4. Motivating invariants: the truthed surfaces are governed by INV-018 (replay), INV-024/INV-067/INV-093 (no-leak / actor-known), INV-004/INV-006/INV-094 (no-human / possession parity), INV-008 (UI not authority), and the INV-098-class acceptance discipline. The doc rows must describe the enforcement as it now stands.
5. This ticket *audits/records* the resealed enforcement surfaces by doctrine without modifying code (evidence/substrate basis): it names each surface — sealed `ValidatedLoadedWorldBootstrap`, the core-owned immutable interval product, the token-gated debug receipt/command, the killed `food_source` family, the governance audit job — and confirms the evidence rows introduce no leakage/nondeterminism path (they are retrospective evidence, post-closure, with no feedback into simulation). Every updated row answers the §6.2 four questions: which sealed constructor created the runtime, which public command/token crossed the client boundary, what state/event/projection effect was observed, and which deliberate mutation or negative-compile attempt proves sensitivity.

## Architecture Check

1. A single cross-cutting docs ticket that lands after all implementation tickets is cleaner than per-ticket doc edits here: the conformance rows reference surfaces from 004–007 coherently (sealed bootstrap + sealed interval product + token-gated debug + killed survivors must all exist before the rows can honestly cite them), and §6.2 forbids editing the rows ahead of the executable witnesses. Co-locating would risk a dishonest intermediate row describing an unlanded seal.
2. No backwards-compatibility aliasing or shims (doc-only). No archived spec/ticket/report/acceptance/certification is edited (§6.2 / §1.2).

## Verification Layers

1. INV-018/INV-024/INV-004/INV-008 (truthed surfaces) -> invariants alignment check: each updated evidence row cites the real resealed surface and a sensitivity proof (negative fixture or mutation), not a public raw constructor.
2. Doc↔code consistency -> grep-proof against the post-implementation tree: the symbols the rows name (e.g. `ValidatedLoadedWorldBootstrap`, the sealed receipt, `DebugSessionAuthority`) exist in code; no row names a removed public raw constructor (`from_loaded_state`, public `from_seed_parts`, public exact getters/mutators, self-minting debug constructors).
3. Cross-artifact: ref 00/01 navigation/status edits compose with ticket 002's doctrine edits without contradiction (sequential, `Deps: 002`).
4. Documentation-only ticket: verification is grep-based landing + invariants-alignment review, not `cargo` tests (stated in the Test Plan).

## What to Change

### 1. Architecture-tier evidence rows

`docs/1-architecture/04` — update the current evidence row to name the sealed bootstrap constructor and its negative proof (not a public raw constructor). `docs/1-architecture/10` — update the embodied/debug split evidence to name the sealed interval product, distinguish public embodied vs. debug/operator exact fields, and record the resolved no-human-day classification (debug/operator).

### 2. Execution-tier evidence rows

`docs/2-execution/05` — add bootstrap unforgeability and debug/operator command tokenization to the no-direct-dispatch evidence. `docs/2-execution/06` — clarify the no-human-day classification per the Q2/§10.2 decision (debug/operator). `docs/2-execution/07` — add the sealed interval product and exact-debug-only evidence with external-client compile-fail checks.

### 3. Reference-tier navigation/status

`docs/3-reference/00` and `docs/3-reference/01` — update navigation/status to reflect the landed seals + governance enforcement + killed survivor family (composing with ticket 002's doctrine/risk-status edits; no new risk ID).

## Files to Touch

- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (modify)
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (modify)
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` (modify)
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify)
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- The §6.1 below-foundation doctrine strengthening (ticket 002 — distinct content; this ticket truths *conformance* rows, not doctrine).
- The `docs/4-specs/SPEC_LEDGER.md` row + `archive/specs/` move — deferred to spec acceptance/archival per the hardening-spec convention (a Step 6 cross-spec follow-up, not this ticket).
- Any archived spec/ticket/report/acceptance edit; any production `src/` change.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -rnE "ValidatedLoadedWorldBootstrap|sealed interval product|DebugSessionAuthority" docs/1-architecture/04_*.md docs/1-architecture/10_*.md docs/2-execution/05_*.md docs/2-execution/07_*.md` returns the new evidence rows.
2. `grep -rnE "from_loaded_state|pub.*from_seed_parts" docs/1-architecture docs/2-execution` returns no current-evidence row citing a removed public raw constructor.
3. `grep -nE "no-human" docs/2-execution/06_*.md` shows the resolved debug/operator classification.
4. Manual invariants-alignment review confirms each updated row answers the §6.2 four questions and no archived artifact was edited.

### Invariants

1. No conformance row cites a removed public raw constructor or a forgeable product as current evidence (§6.2).
2. Every updated row names a sealed constructor/token, an observed effect, and a sensitivity proof (negative fixture or mutation).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is grep-based landing checks plus an invariants-alignment review. The executable witnesses are the upstream tickets' tests (004–007 negative fixtures + 007 mutation kills + 003 governance audit), named in Assumption Reassessment.`

### Commands

1. `grep -rnE "ValidatedLoadedWorldBootstrap|DebugSessionAuthority|sealed interval" docs/1-architecture docs/2-execution docs/3-reference`
2. `grep -rnE "from_loaded_state|pub fn from_seed_parts" crates/tracewake-core/src crates/tracewake-content/src` — must return nothing (the symbols the docs claim removed are actually gone post-implementation).
3. A narrower command set is correct because the deliverable is conformance prose; the only code-side check is command 2 confirming the docs' removal claims match the tree.

## Outcome

Completed: 2026-06-26

Truthed the live conformance and reference rows in the seven named docs after the upstream implementation tickets landed. The updated rows now cite `ValidatedLoadedWorldBootstrap`, `ContinuedRuntimeReceipt`, the sealed interval product, `DebugSessionAuthority`, the debug/operator no-human-day classification, the 0053 governance API-audit posture, and the closed `food_source_fact_supersedes` mutation family. No archived spec, report, or ticket was edited.

Manual invariants-alignment review: each updated row names the sealed constructor/token, the public client or runtime boundary crossed, the observed event/projection/receipt effect, and the sensitivity proof via negative fixture or targeted mutation run. The rows are retrospective evidence only and introduce no simulation input, actor-known channel, serialization shape, or replay nondeterminism.

Verification passed:

1. `grep -rnE "ValidatedLoadedWorldBootstrap|sealed interval product|DebugSessionAuthority" docs/1-architecture/04_*.md docs/1-architecture/10_*.md docs/2-execution/05_*.md docs/2-execution/07_*.md`
2. `grep -rnE "from_loaded_state|pub.*from_seed_parts" docs/1-architecture docs/2-execution` returned no matches.
3. `grep -nE "no-human" docs/2-execution/06_*.md`
4. `grep -rnE "from_loaded_state|pub fn from_seed_parts" crates/tracewake-core/src crates/tracewake-content/src` returned no matches.
