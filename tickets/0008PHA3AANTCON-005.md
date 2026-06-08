# 0008PHA3AANTCON-005: Candidate/BDI/HTN arbitration hardening

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` agent modules: needs/routines/projects as candidate inputs only; durable intention lifecycle; routine methods produce local-planner goals
**Deps**: 0008PHA3AANTCON-001

## Problem

Spec 0008 Finding 4 (D-0008-04) §8.3: needs and routines can bypass BDI/HTN arbitration. Severe hunger/fatigue should interrupt routine duties through explicit candidate scoring, intention interruption/continuation/adoption, and event ancestry — not direct selection. Routines should contribute candidate duties and HTN methods, not act as privileged script-dispatch switches. The arbitration modules exist (`agent/candidate.rs`, `agent/decision.rs`, `agent/generation.rs`, `agent/intention.rs`, `agent/routine.rs`, `agent/htn.rs`, `agent/methods.rs`) but the runtime path is still after-the-fact and direct-dispatch heavy (the scheduler bypass is removed in -006; this ticket hardens the arbitration internals the transaction will rely on).

## Assumption Reassessment (2026-06-08)

1. `agent/candidate.rs` defines `CandidateGoal` (`:108`), `GoalKind` (`:7`), `CandidateGoalSource` (`:36`), `GoalPriority` (`:59`), `ApplicabilityResult` (`:102`); `agent/decision.rs` `select_goal_and_trace` (`:23`); `agent/htn.rs::select_phase3a_method` (`:28`); `agent/generation.rs::generate_candidate_goals_from_agent_state` with `LiveCandidateGenerationInput` (`:27`); `RoutineFamily`/`RoutineStep` referenced from `scheduler.rs` (e.g. `:670-677`).
2. Spec §8.3 + §9.3: current active intention considered first for continuation; severe needs interrupt routines only through explicit arbitration + emitted intention/routine events; routine duties create candidate goals and HTN methods; HTN methods produce typed local-planner goals or primitive intents; HTN preconditions stay typed (no substring conditions); prevent `routine_continue_current_intention` from becoming a no-op progress marker.
3. Cross-artifact boundary under audit: the **candidate-generation ↔ intention-lifecycle ↔ HTN-method** contract — the single arbitration chain (`generation.rs`→`decision.rs`→`intention.rs`/`routine.rs`→`htn.rs`/`methods.rs`) that the transaction (-004) orchestrates and the scheduler (-006) stops bypassing.
4. INV-033 (BDI separation), INV-034 (durable intentions), INV-035 (routines are defeasible intentions, not teleports/puppet strings), INV-036 (HTN methods are procedures, not story scripts), INV-039 (needs are pressures, not puppet strings): needs/routines feed candidate generation and arbitration; they never directly script an action.
5. Enforcement surface: actor-knowledge firewall (arbitration consumes sealed-context facts from -001) + the no-script boundary. Confirm: candidate generation reads actor-known facts via -001's accessors; an intention interruption emits an explicit lifecycle record (no silent override); HTN method preconditions remain typed. No nondeterminism introduced — candidate scoring is the existing deterministic bounded heuristic (INV-038), not a new RNG path.

## Architecture Check

1. Making candidate generation the sole entry point for need/routine/project pressure is the doctrine-correct BDI shape: pressures compete as scored candidates, the winner drives intention lifecycle and method selection. Direct dispatch (current) collapses desire and intention and erases the trace's causal force.
2. No backwards-compatibility aliasing: routine-family→action switches are not retained behind the candidate layer; they are removed from the decision path (the scheduler's copies are deleted in -006).

## Verification Layers

1. INV-039 needs-as-pressures → unit test: severe hunger raises an eat candidate that wins arbitration and emits an intention interruption/adoption record; it does not directly construct an eat proposal.
2. INV-034 durable intentions → unit test: a committed intention survives small score jitter and is continued unless a stronger modeled cause interrupts it (with an emitted lifecycle record).
3. INV-036 typed HTN methods → codebase grep-proof: HTN preconditions are typed conditions, not substring matches (`grep -n "contains\|starts_with" agent/htn.rs` shows none in precondition logic).
4. Continuation-not-progress → unit test: `routine_continue_current_intention` yields a continuation candidate that must resolve to a follow-on ordinary action or a typed blockage (the progress-counting prohibition is enforced in -006).

## What to Change

### 1. Candidate generation as sole pressure entry point

`generation.rs` produces candidates from needs, routine windows, projects, and the current intention; severe-need pressures appear as high-priority candidates, not direct selections.

### 2. Durable intention lifecycle

`intention.rs`/`routine.rs` emit explicit continuation, interruption, completion, failure, and replacement records; active intention is continued unless interrupted by a stronger modeled cause.

### 3. Routine methods → local-planner goals

`routine.rs`/`htn.rs`/`methods.rs`: routine duties create candidate goals and typed HTN methods that produce local-planner goals or primitive intents with actor-known prerequisites; preconditions stay typed.

## Files to Touch

- `crates/tracewake-core/src/agent/generation.rs` (modify)
- `crates/tracewake-core/src/agent/candidate.rs` (modify)
- `crates/tracewake-core/src/agent/decision.rs` (modify)
- `crates/tracewake-core/src/agent/intention.rs` (modify)
- `crates/tracewake-core/src/agent/routine.rs` (modify)
- `crates/tracewake-core/src/agent/htn.rs` (modify)
- `crates/tracewake-core/src/agent/methods.rs` (modify)

## Out of Scope

- Removing the scheduler's direct routine-family/need-threshold dispatch (0008PHA3AANTCON-006).
- The transaction orchestration that calls this chain (0008PHA3AANTCON-004).
- Behavioral no-human gates (carried in 0008PHA3AANTCON-006; arbitration unit tests live here).

## Acceptance Criteria

### Tests That Must Pass

1. Severe-need-interrupts-routine unit test: explicit candidate/intention/routine records precede eating/sleeping; no direct proposal.
2. Mild-need-does-not-override unit test: a committed routine intention survives a mild need (§10.3 gate 3 substrate).
3. Typed-HTN-precondition test: methods select via typed conditions; no substring precondition.
4. `cargo test --workspace` green.

### Invariants

1. Needs/routines/projects enter only through candidate generation.
2. Intention interruptions are always eventful (lifecycle record emitted).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/decision.rs` / `intention.rs` tests — arbitration, durability, interruption ancestry.
2. `crates/tracewake-core/src/agent/htn.rs` tests — typed precondition selection.

### Commands

1. `cargo test -p tracewake-core agent::`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
