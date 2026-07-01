# 0059AUTSCHROU-003: Metamorphic A1–A10 + fail-closed behavioral proof

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds a new behavioral/metamorphic integration test module (test-only; no production code)
**Deps**: 0059AUTSCHROU-001, 0059AUTSCHROU-002

## Problem

The active-intention authority of autonomous routine-family derivation (fixed in 001/002) needs a behavioral witness that a scheduler/clock fact cannot select a routine family ahead of cognition, and that absent/ambiguous/conflicting chains fail closed with typed, replayable outcomes. 0059 §7.1–7.3 require a focused metamorphic test module proving the autonomous effective routine-family decision equals the active-intention-derived family, or produces a typed non-override outcome, across the adversarial cases A1–A10 — and a behavioral fail-closed test that asserts an eventful typed outcome rather than a bare `None`.

## Assumption Reassessment (2026-07-01)

1. The target test path does not yet exist: `crates/tracewake-core/tests/scheduler_routine_derivation_authority.rs` is absent (verified), so this is a `(new)` module. The behaviors it asserts are produced by 001 (scheduler producer rebind) and 002 (transaction fail-closed/non-override); both must land first. Existing parity coverage lives in `crates/tracewake-core/tests/embodied_autonomous_parity.rs` and the in-module `scheduler.rs` / `transaction.rs` unit tests — this module is the consolidated adversarial suite, not a duplicate.
2. Spec `specs/0059_…_SPEC.md` §7.1 names the module path and the required test-name family (`0059`, `scheduler`, `routine`, `active_intention`, `authority`); §7.2 enumerates cases A1–A10 and the comparison dimensions; §7.3 requires a fail-closed behavioral test asserting a typed eventful outcome (`StuckDiagnosticRecorded` with a specific reason, a transaction diagnostic, or a typed non-routine idle-fallback reason) rather than a `None` mistakable for ordinary absence.
3. Shared boundary under audit: the autonomous decision path's *effective* routine-family decision vs. the active-intention-derived family. The test computes the authoritative family from the active-intention chain and observes the autonomous path's effective decision, asserting equality or a typed non-override outcome — it must drive real fixtures (actor-known states + active intentions), not mock string checks (§7.2).
4. Motivating invariant restated: **INV-098 — Feature acceptance is harsh** ("done only when … caused, agent-possible, eventful, trace-aware, epistemically bounded, … no-human runnable, replay-safe, … regression-tested"). A silent scheduler-chosen override that passes a test is a failure even if green.
5. Enforcement surface audited (evidence-consumer basis): this module reads the actor-knowledge / deterministic-replay / fail-closed surfaces 001/002 establish; it must assert the hidden-truth audit result (world truth supplied no cognition — case A9/A11 dimensions) and source-event ancestry without itself introducing any leakage or nondeterminism (fixtures are deterministic; assertions compare typed records, not wall-clock or iteration-order values).

## Architecture Check

1. A single focused metamorphic module keyed to the `0059`/`scheduler`/`routine`/`active_intention`/`authority` name family lets source guards (004) and focused mutation (005) target it directly, and proves the property across the full adversarial matrix in one place rather than scattering partial assertions across unit tests. The metamorphic equivalence (authoritative-family == effective-family OR typed non-override) is stronger than per-case output assertions because it rejects silent overrides structurally.
2. No backwards-compatibility aliasing/shim (test-only module; introduces no production surface).

## Verification Layers

1. INV-103 (scheduler not cognition authority) -> replay/golden-fixture-style behavioral check: A1/A2 (workplace temptation vs. eat/sleep active intention) and A9 (hidden workplace truth only) assert family follows the active intention, never the environment/world truth.
2. INV-104 (no direct dispatch) -> behavioral check: A3/A4/A5/A6 (inactive/resolved/foreign execution, no active intention) assert no `RoutineDuty` candidate or family is produced from window eligibility.
3. INV-105 (typed diagnostics) -> manual review + behavioral check: A6/A7/A10 (no chain, malformed chain, conflicting hint) assert typed, eventful, replayable outcomes with source-event ancestry, not a bare `None`.

## What to Change

### 1. Author the metamorphic equivalence property

Add `crates/tracewake-core/tests/scheduler_routine_derivation_authority.rs`. Implement a helper computing the authoritative routine family from the active-intention chain and a helper observing the autonomous path's effective routine-family decision; over a fixed family of actor-known states and active intentions, assert the autonomous result equals the active-intention-derived result OR a typed explained non-override outcome. A silent scheduler-chosen override is always a test failure. Compare at minimum the dimensions in §7.2: routine family; active-intention id and kind; selected method id and current unresolved step/execution id; candidate source/priority class (including absence of unauthorized `RoutineDuty`); chosen action id/targets/params when a proposal is emitted; stuck diagnostic kind and source-event ancestry when fail-closed; actor-known context source event ids/kinds; hidden-truth audit result.

### 2. Enumerate adversarial cases A1–A10

Implement A1–A10 per §7.2 (workplace-temptation-vs-eat, workplace-vs-sleep, inactive execution, resolved execution, other actor's execution, no active intention, ambiguous/malformed chain, work step with actor-known route issue, hidden workplace truth only, caller-supplied conflicting `routine_window_family`), each with its expected result.

### 3. Behavioral fail-closed test (§7.3)

Add a test proving the autonomous derivation fails closed when the active-intention chain is absent, ambiguous, stale, resolved, or actor-mismatched — asserting a typed, eventful, replayable outcome (`StuckDiagnosticRecorded` with a specific reason such as `routine_family_requires_active_intention`, a transaction diagnostic, or a typed non-routine idle-fallback reason), not a bare `None`.

## Files to Touch

- `crates/tracewake-core/tests/scheduler_routine_derivation_authority.rs` (new)
- `crates/tracewake-core/tests/support/mod.rs` (modify — only if a shared fixture helper is needed; otherwise keep fixtures local to the new module)

## Out of Scope

- Production changes (owned by 001/002) — this module asserts behavior, it does not implement it.
- Anti-regression source/behavior guards and synthetic negatives (004).
- Focused mutation (005) and the acceptance artifact (006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test scheduler_routine_derivation_authority` — all A1–A10 cases and the fail-closed behavioral test pass; each adversarial case asserts the metamorphic equivalence or a typed non-override outcome (A1 workplace-vs-eat, A2 workplace-vs-sleep, A3 inactive execution, A4 resolved execution, A5 foreign execution, A6 no active intention, A7 malformed chain, A8 work-step route issue, A9 hidden workplace truth, A10 conflicting hint).
2. `cargo test -p tracewake-core --test embodied_autonomous_parity` — parity coverage remains green alongside the new module.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — the four local gates pass.

### Invariants

1. For every A-case, the autonomous effective routine family equals the active-intention-derived family or is a typed explained non-override outcome; no case yields a silent scheduler/clock-chosen family (INV-103, INV-104).
2. Every fail-closed case asserts a typed, eventful, replayable outcome with source-event ancestry — not a bare `None` (INV-105, INV-098).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/scheduler_routine_derivation_authority.rs` — new metamorphic + fail-closed suite (A1–A10 + the §7.3 fail-closed test); rationale: the primary behavioral witness for 0059's active-intention-authority property.

### Commands

1. `cargo test -p tracewake-core --test scheduler_routine_derivation_authority` — the new module is the correct verification boundary for this deliverable (resolves once this ticket creates the target).
2. `cargo test -p tracewake-core --test scheduler_routine_derivation_authority -- --list` — enumerate the case functions to confirm the A1–A10 + fail-closed set is complete before the full run.
3. `cargo test --workspace` — full-pipeline confirmation the new integration target compiles and passes alongside the rest.
