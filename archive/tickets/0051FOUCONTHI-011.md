# 0051FOUCONTHI-011: F-09 truth the live conformance and risk evidence

**Status**: COMPLETE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — documentation only: align the architecture conformance index, architecture/execution status/evidence rows, the reference checklist, and the R-27/R-28/R-29 status with the repaired runtime. No governing doctrine changes.
**Deps**: 0051FOUCONTHI-002, 0051FOUCONTHI-003, 0051FOUCONTHI-004, 0051FOUCONTHI-005, 0051FOUCONTHI-006, 0051FOUCONTHI-007, 0051FOUCONTHI-008

## Problem

The architecture conformance index (`00`) and execution `05`/`07` describe core-owned loaded-actor/process discovery, closed actor-outcome consumption, and TUI read-only interval consumption as production-proven, but the live code contradicted those reachability/authority claims (F-09, evidence-honesty gap). After the code closure (`-002`..`-008`) and the executable witnesses (`-009`/`-010`), this ticket aligns the live conformance and risk evidence with the repaired runtime — status/evidence only, no governing doctrine change. It lands **after** the executable witnesses exist.

## Assumption Reassessment (2026-06-24)

1. Codebase/docs: conformance index `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`; architecture `01`/`02`/`04`/`10`/`13`; execution `docs/2-execution/05_*`/`06_*`/`07_*`/`10_*`; reference checklist `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`; risks `docs/3-reference/01_*.md` (R-27 at `333`, R-28 at `344`, R-29 at `356`). These describe the surfaces repaired by `-002`..`-008`.
2. Specs/docs: spec `0051` §4.9 (post-implementation doc-truthing), §1.1 F-09 (the `SPEC_LEDGER.md` 0050 row / 48-missed-1-timeout / 0049MUTWIT ticket-only record is **already accurate** and not in scope to change).
3. Shared boundary under audit: the doctrine-doc evidence rows that *cite* the code surfaces — they must name the actual core runtime entry point, the real production witness, restored eligibility, the process transaction, the actor census, and the split products, without changing the governing rules.
4. INV-092 (replay tested), INV-098 (harsh acceptance): restated — every updated evidence row must answer which production constructor created the runtime, which public command crossed the client boundary, what state/event/projection effect was observed, and which deliberate mutation or negative-compile attempt proves sensitivity.
5. Fail-closed / replay / actor-knowledge surface (doctrine-substrate basis): these docs *govern* the no-leak, replay, and authority enforcement surfaces; the change updates only status/evidence pointers and introduces no leakage/nondeterminism claim ahead of its proof (each row cites the `-009`/`-010` witness). No governing doctrine is weakened.

## Architecture Check

1. Truthing the evidence rows only after the executable witnesses exist (Deps on `-002`..`-008`, evidence from `-009`/`-010`) is the only sequence that avoids a dishonest intermediate state (a conformance row describing an enforcement that has not landed). Updating status/evidence without touching governing doctrine keeps the doc-tier authority intact.
2. No backwards-compatibility alias: stale "production-proven" claims are corrected, not annotated-but-retained.

## Verification Layers

1. INV-098 (evidence honesty) -> manual review + grep-proof: each updated row names the production constructor, the public command, the observed effect, and the sensitivity proof.
2. R-27/R-28/R-29 -> grep-proof: only the status/evidence under the existing risk IDs changes; no new risk ID is minted.
3. Doctrine integrity -> invariants-alignment check: no `INV-NNN` force/scope is changed; the `SPEC_LEDGER.md` already-correct 0050/0049 records are untouched.

## What to Change

### 1. Architecture conformance index + status rows

Update `00` to name the actual core runtime entry point and the real production witness; align `01`/`02`/`04`/`10`/`13` status/evidence references with the repaired runtime, restored eligibility, process transaction, actor census, and split products — without changing governing doctrine.

### 2. Execution + reference evidence

Align execution `05`/`06`/`07`/`10` status/evidence; update the reference checklist `00` live-evidence pointers; update only the status/evidence under the existing **R-27/R-28/R-29** (mint no new risk ID).

## Files to Touch

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` (modify)
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` (modify)
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (modify)
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (modify)
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` (modify)
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify — R-27/R-28/R-29 status/evidence only)

## Out of Scope

- Any `INV-NNN` force/scope change or new risk-ID mint.
- The `docs/4-specs/SPEC_LEDGER.md` archived row and the `archive/specs/` move (deferred to spec acceptance — Step 6 cross-spec follow-up).
- The already-accurate 0050/0049MUTWIT ledger records.

## Acceptance Criteria

### Tests That Must Pass

1. `grep` confirms the conformance index and execution `05`/`07` name the core runtime entry point and the `-009`/`-010` production witness (no surviving "production-proven" claim contradicted by code).
2. `grep` confirms R-27/R-28/R-29 retain their IDs with updated status/evidence and no new risk ID was added.
3. `cargo test -p tracewake-core --test doc_invariant_references` (and any doc-reference guard) is green.

### Invariants

1. No governing doctrine (`INV-NNN`, gate semantics) is changed; only status/evidence.
2. The `SPEC_LEDGER.md` already-correct records are untouched.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `cargo test -p tracewake-core --test doc_invariant_references`
2. `grep -nE "R-2[789]" docs/3-reference/01_*.md`
3. `cargo test --workspace`

## Outcome

Completed: 2026-06-24

Updated live status/evidence references only. No `INV-NNN` doctrine, gate
semantics, spec ledger rows, archived tickets, or risk identifiers were changed.

Documentation truthing completed:

1. `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
   - Updated the loaded-world temporal conformance row to name
     `LoadedWorldRuntime::from_loaded_world`, the public TUI world-advance
     boundary, the 0051 production witnesses, and the non-green standing
     mutation disposition from `-010`.
2. `docs/1-architecture/01_*`, `02_*`, `04_*`, `10_*`, and `13_*`
   - Added status/evidence pointers for the repaired runtime constructor,
     restored eligibility, process transactions, actor census effects, public
     command crossing, read-only interval products, and sensitivity evidence.
3. `docs/2-execution/05_*`, `06_*`, `07_*`, and `10_*`
   - Replaced stale "current 0050" evidence wording with current 0051
     evidence that separates production behavior, replay verdicts,
     holder-known interval output, compile-fail authority boundaries, TUI
     parity/adversarial evidence, focused mutation proof, and standing
     disposition.
4. `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
   - Updated the live-evidence prompt block to point reviewers at the 0051
     runtime constructor, witness suites, public command tests, and `-009`/`-010`
     archive records.
5. `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
   - Updated only the status/evidence bullets under existing R-27, R-28, and
     R-29. No new risk ID was minted.

Verification:

1. `cargo test -p tracewake-core --test doc_invariant_references` -> passed.
2. `grep -nE "R-2[789]" docs/3-reference/01_*.md` -> confirmed R-27, R-28,
   and R-29 retain their IDs; no new R-2x entry was added.
3. `grep -nE "LoadedWorldRuntime::from_loaded_world|0051FOUCONTHI-00[9]|0051FOUCONTHI-010|TuiApp::submit_entry_with_world_advance|command-loop|3275|23.*missed" ...`
   -> confirmed the conformance index, execution `05`/`07`, reference
   checklist, and risk register name the repaired runtime and 0051 evidence.
4. `cargo test --workspace` -> passed.
5. `git diff --check` -> passed.
