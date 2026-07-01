# 0059AUTSCHROU-005: Focused mutation run + triage

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: 0059AUTSCHROU-001, 0059AUTSCHROU-002, 0059AUTSCHROU-003, 0059AUTSCHROU-004

## Problem

A green test suite does not prove the 0059 fix is mutation-resistant: the rebound autonomous routine-family derivation and the transaction fail-closed handling could harbor surviving mutants (e.g. replacing the active-intention-derived family with a window-derived one, dropping an actor-id ownership check, accepting a resolved/foreign execution) that the behavioral suite (003) and guards (004) fail to kill. 0059 §8 requires a *focused* mutation run over the touched seams with honest survivor disposition — not a full standing campaign — and treats survivorful focused mutation as non-pass unless every survivor is equivalent/unviable with evidence.

## Assumption Reassessment (2026-07-01)

1. The mutation perimeter is the touched-seam file set, all verified present: `crates/tracewake-core/src/scheduler.rs`, `crates/tracewake-core/src/agent/transaction.rs`, `crates/tracewake-core/src/agent/generation.rs`, the new `crates/tracewake-core/tests/scheduler_routine_derivation_authority.rs` (created by 003), and any touched `crates/tracewake-core/tests/support/*`. The standing config `.cargo/mutants.toml` exists; 0059 §8 is a focused run, not a standing-perimeter widening, so this ticket makes **no** `.cargo/mutants.toml` change.
2. Spec `specs/0059_…_SPEC.md` §8.1 (perimeter), §8.2 (required mutant classes), §8.3 (suggested `cargo mutants --file … --re … --output target/mutants/0059-* -- … --test scheduler_routine_derivation_authority` commands, adjustable to the repo's cargo-mutants version), and §8.4 (honest disposition: command lines, attempted/caught/missed/unviable/timeout counts, exact survivor list, per-survivor disposition, and the explicit statement that survivorful focused mutation is non-pass unless survivors are equivalent/unviable with accepted evidence).
3. Shared boundary under audit: the kill-relationship between the behavioral witnesses (003) + guards (004) and the mutants over the production seams (001/002). This ticket runs the campaign and reconciles survivors; it must depend on 001–004 so the witnesses exist before the run. Survivor identities are cargo-mutants `path:line:column` entries whose line/column anchors are advisory — reconcile each by enclosing symbol + mutation operator.
4. Motivating invariant restated: **INV-098 — Feature acceptance is harsh** — acceptance requires honest mutation disposition, and a run that omits the touched derivation seam or reports only aggregate green without a survivor ledger is not evidence (§8.4).
5. Enforcement surface audited (evidence-consumer basis): the run reads the fail-closed / actor-knowledge / replay seams 001–004 establish without modifying them; it introduces no leakage or nondeterminism (mutation output lands under the gitignored `target/mutants/`). Any run-discovered real mutant requiring a *new* witness is routed to a reserved follow-up ticket rather than silently absorbed.

## Architecture Check

1. A standalone focused run+triage ticket keeps the mutation evidence reproducible and the survivor disposition auditable in one place, and cleanly separates "run the campaign + record disposition" (this ticket, `Engine Changes: None`) from "add a witness to kill a real survivor" (a reserved follow-up, `Engine Changes: Yes`), per the spec's non-pass-on-unexplained-survivor rule.
2. No backwards-compatibility aliasing/shim; no production or config change (the perimeter already covers the touched seams, so `.cargo/mutants.toml` is untouched).

## Verification Layers

1. INV-098 (honest mutation disposition) -> mutation run + manual review: the §8.4 ledger (denominator, caught/missed/unviable/timeout, exact survivor list, per-survivor disposition) is produced and every survivor is killed-by-added-test, equivalent-with-rationale, or unviable-with-evidence.
2. INV-103/INV-104 (no clock/window cognition; no direct dispatch) -> mutation-class coverage: the §8.2 mutant classes (window-derived family substitution, dropped actor-id ownership check, accepted terminal/foreign execution, `start_tick`/`execution_id` preference, suppressed conflict diagnostic, `RoutineDuty` without active intention) are attempted and caught.

## What to Change

### 1. Run focused mutation over the touched seams

Run focused mutation over `scheduler.rs`, `transaction.rs`, `generation.rs`, and the 0059 test surfaces using the §8.3 command shape (adjusted to the installed cargo-mutants version), preserving the file-targeted denominator and the `--test scheduler_routine_derivation_authority` scope. Attempt the §8.2 mutant classes.

### 2. Triage and disposition

Produce the §8.4 honest disposition ledger: command lines + environment summary, attempted/caught/missed/unviable/timeout counts, the exact survivor list, and a per-survivor disposition (killed-by-added-test, equivalent-with-rationale, unviable-with-compiler-evidence, timeout-with-reason, or accepted-residual-risk). Record the explicit non-pass statement for any unexplained survivor. This ledger is handed to 0059AUTSCHROU-006 for the acceptance artifact's focused-mutation-report section.

### 3. Route run-discovered survivors

Any run-discovered real mutant needing a new behavioral witness (rather than an equivalent/unviable disposition) is routed to a reserved follow-up ticket in the reserved open range `0059AUTSCHROU-007+` (a later batch in this namespace must check the actual high-water mark before claiming numbers). Killing such mutants with new witnesses is out of scope for this run+record ticket.

## Files to Touch

- `None — evidence-only run; focused-mutation output lands under the gitignored target/mutants/0059-*, and the §8.4 survivor-disposition ledger is recorded into the 0059 acceptance artifact (0059AUTSCHROU-006).`

## Out of Scope

- Any production or `.cargo/mutants.toml` change (the focused perimeter already covers the touched seams; this is not a standing-perimeter widening).
- Adding new witness tests to kill run-discovered real survivors — those route to the reserved `0059AUTSCHROU-007+` range.
- The final acceptance verdict (006).

## Acceptance Criteria

### Tests That Must Pass

1. The §8.4 ledger is produced with command lines, attempted/caught/missed/unviable/timeout counts, the exact survivor list, and a per-survivor disposition; the run targets the touched derivation seams (not aggregate green).
2. Every §8.2 mutant class is attempted; every survivor is killed-by-added-test, equivalent-with-rationale, or unviable-with-evidence — or the result is recorded `non-pass` with the offending survivors named.
3. `cargo mutants --package tracewake-core --file crates/tracewake-core/src/scheduler.rs --re 'routine.*family|eligible.*routine|transact_world_one_tick|ActorDecisionTransactionInput' --output target/mutants/0059-scheduler -- -p tracewake-core --test scheduler_routine_derivation_authority` — runs to completion with a recorded denominator (command shape adjustable to the installed cargo-mutants version).

### Invariants

1. Focused mutation over the touched seams is non-survivorful, or every survivor carries an accepted equivalent/unviable disposition (INV-098).
2. No survivor implying a clock/window-derived family override or a dropped ownership check is left unexplained (INV-103, INV-104).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification is the focused cargo-mutants run plus the §8.4 disposition ledger. The behavioral witnesses live in 0059AUTSCHROU-003/004 and the production seams in 001/002.`

### Commands

1. `cargo mutants --package tracewake-core --file crates/tracewake-core/src/agent/transaction.rs --re 'active_intention_for_actor|goal_for_routine_family|ActorDecisionTransaction|routine_window_family' --output target/mutants/0059-transaction -- -p tracewake-core --test scheduler_routine_derivation_authority` — focused run over the transaction seam (per-file denominator preserved).
2. `cargo mutants --package tracewake-core --file crates/tracewake-core/src/agent/generation.rs --re 'routine_window_goal|RoutineDuty|generate_candidate_goals' --output target/mutants/0059-generation -- -p tracewake-core --test scheduler_routine_derivation_authority` — focused run over the generation seam.
3. A focused per-file run is the correct verification boundary here (not the full standing campaign): 0059 §8 mandates focused mutation over the touched seams only, with the file-targeted denominator and survivor ledger preserved.
