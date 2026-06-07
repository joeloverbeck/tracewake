# 0005PHA3ANEEROU-013: Routine families and HTN method selection

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the authored Phase 3A routine methods (MorningWake…Wait) and the HTN-like method selection that turns a selected goal into a routine execution proposing ordinary actions.
**Deps**: 0005PHA3ANEEROU-011, 0005PHA3ANEEROU-012

## Problem

Routine method selection must be HTN-like — authored methods for ordinary procedures, not pure utility scoring and not hard-coded schedule rails — selecting from known/applicable methods by goal kind, routine assignment, actor-known facts, window, location/body state, need thresholds, and prior failures, with deterministic tie-breaking and a typed failure/fallback when no method applies (Spec 0005 §11.3, §11.5; `INV-036`, `INV-035`). The required routine families are the ordinary-life vocabulary the no-human day exercises.

## Assumption Reassessment (2026-06-07)

1. The routine template/execution/step model (ticket 003), the candidate-goal/intention engine (ticket 012), and the ordinary actions sleep/eat/work/continue/wait (008–011) are the building blocks; this ticket authors the methods and the selector under the `agent` module. Routine methods are authored possibility (`INV-061`) — their *content* (assignments, windows) is validated by the content schema (ticket 015), while the method *machinery* (selection, step expansion to proposals) is core.
2. Spec §11.5 fixes the required families and their behaviors: `MorningWake` (transition from rest, no teleport out of bed), `EatMeal` (choose known/believed/visible food, route/open/check, consume via `eat`, fail/replan), `GoToWork` (route by known places, no teleport), `WorkBlock` (start at valid workplace, duration, complete/fail), `ReturnHome` (route, fail with diagnostic), `SleepNight` (route, scheduled sleep, no instant refill), `FindFood` (fallback using actor-known memories/visible containers/authored public service; no hidden truth), `ContinueCurrentIntention` (preserve durable commitment across possession), `Wait`/`IdleWithReason` (mandatory reason, bounded, reevaluation). Spec §11.6 optional families are out of scope unless cheap/doctrine-aligned.
3. Shared boundary under audit: each method maps a goal kind to a routine template whose steps reference `SemanticActionId`s validated by the registry (tickets 008–011); a method naming an unknown action is an authoring error. Method selection emits a `DecisionTrace` (ticket 004). The family set is fixed by ticket 003; this ticket gives each a method body.
4. Invariants motivating this ticket: `INV-036` — "HTN methods are procedures, not story scripts" (state-dependent methods with failure, interruption, alternatives, costs, traces) — and `INV-035` — "Routines are defeasible intentions". No method may force success, teleport, or continue after a failed precondition; when no method applies it produces a typed failure/fallback, never synthesized story behavior (Spec §11.3).
5. Actor-knowledge / replay surface: method selection reads actor-known facts only (`INV-024`); `FindFood` explicitly must not inspect hidden true food location (Spec §11.5). Selection is deterministic (stable tie-break) and trace-emitting; it adds no nondeterminism and no leakage. The no-teleport proof is ticket 020; the bounded route/sequence planning `EatMeal`/`GoToWork` need lands in ticket 014 (cited).

## Architecture Check

1. Authored HTN methods selected by applicability (rather than a utility function or a fixed daily schedule) keep routines inspectable and defeasible while avoiding both jitter and schedule-rail teleportation (Spec §25.4, §11.3) — a utility scorer would reintroduce jitter and a hard schedule would teleport. Mapping each step to a validated `SemanticActionId` keeps routines as proposers, never mutators (`INV-036`).
2. No backwards-compatibility shims: methods are authored on the ticket-003 template model; the selector reuses the ticket-012 trace/decision path rather than a parallel selection engine.

## Verification Layers

1. HTN-not-script (`INV-036`) -> unit test: each family selects a method whose steps are action proposals; a goal with no applicable method yields a typed failure/fallback (not a synthesized success), with a trace.
2. Defeasible routines (`INV-035`) -> unit test: a precondition failure mid-method triggers the method's declared fallback/interruption, recording a trace — never a silent proceed.
3. No telepathy (`INV-024`) -> integration test: `FindFood` selects only actor-known/visible/authored-public food knowledge; a hidden true location does not influence selection. Bounded routing is ticket 014; no-teleport fixture is ticket 020 (cited).

## What to Change

### 1. Authored routine methods

Add `crates/tracewake-core/src/agent/methods.rs` authoring each §11.5 family as a `RoutineTemplate` (ticket 003) with applicability conditions, ordered steps mapping to `SemanticActionId`s, declared interruption points, failure modes, and fallback rules. `EatMeal`/`FindFood`/`GoToWork`/`ReturnHome` steps that need routing/checking emit the planner handle (ticket 014).

### 2. HTN method selector

Add the selection logic (same module or `agent/htn.rs`): from a selected `CandidateGoal` (ticket 012), choose the applicable method by the §11.3 inputs with deterministic tie-breaking; instantiate a `RoutineExecution`; emit a method-selection `DecisionTrace`. No applicable method → typed failure/fallback.

### 3. Module registration

Declare the new module(s) in `crates/tracewake-core/src/agent/mod.rs` and re-export the selector entry point used by tickets 011 (continue) and 017 (runner).

## Files to Touch

- `crates/tracewake-core/src/agent/methods.rs` (new)
- `crates/tracewake-core/src/agent/htn.rs` (new — method selection)
- `crates/tracewake-core/src/agent/mod.rs` (modify — file created by 0005PHA3ANEEROU-001; declare new modules)

## Out of Scope

- Bounded local planning of concrete action sequences within a step (ticket 014).
- Optional families `BasicSocialVisitOrTavernStop`/`OfficeHoursRoutine`/`RespondToImportantContradiction` (Spec §11.6) unless trivially cheap and doctrine-aligned.
- The no-human runner orchestration (ticket 017).
- Routine content schema/assignments (ticket 015).

## Acceptance Criteria

### Tests That Must Pass

1. Each required family (MorningWake, EatMeal, GoToWork, WorkBlock, ReturnHome, SleepNight, FindFood, ContinueCurrentIntention, Wait) selects an applicable method whose steps are ordinary action proposals.
2. A goal with no applicable method yields a typed failure/fallback with a trace; a mid-method precondition failure triggers the declared fallback/interruption.
3. `FindFood` selection uses only actor-known/visible/authored-public food knowledge; a hidden true location does not influence the chosen method.

### Invariants

1. No routine method forces success, teleports, or proceeds after a failed precondition (`INV-036`).
2. Method selection is deterministic and trace-emitting; routines remain defeasible (`INV-035`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/methods.rs` (unit tests) — each family's method body, steps-are-proposals audit.
2. `crates/tracewake-core/src/agent/htn.rs` (unit tests) — applicability selection, no-method failure, deterministic tie-break, trace emission.

### Commands

1. `cargo test -p tracewake-core agent::methods`
2. `cargo test -p tracewake-core agent::htn`
3. Core-crate scope is correct; end-to-end routine runs and no-teleport are exercised by tickets 019/020/021/025.
