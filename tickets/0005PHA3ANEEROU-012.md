# 0005PHA3ANEEROU-012: Candidate-goal generation and intention commitment/switching

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the deterministic candidate-goal generator and the intention adoption/continuation/switching engine, emitting decision traces.
**Deps**: 0005PHA3ANEEROU-004, 0005PHA3ANEEROU-007

## Problem

For each autonomous decision opportunity, the engine must evaluate need bands, threshold crossings, intention status, routine windows, actor-known places/resources, embodied blockers, fallback availability, and planner budget into a deterministic set of candidate goals with traceable selection/rejection — and commit to or switch intentions only on the spec's typed triggers, never silently (Spec 0005 §11.1, §11.2, §8.3; `INV-039`, `INV-040`). This is the decision core the routine selector (013) and runner (017) drive, and the antidote to utility jitter (Spec §25.4).

## Assumption Reassessment (2026-06-07)

1. The candidate-goal/intention types and tie-break comparator (ticket 002), the need model + pressure (001), need effects/reevaluation flag (007), and the decision-trace model (004) are the inputs; this ticket adds the generation/selection behavior under the `agent` module. Phase 2A beliefs/knowledge context (`crates/tracewake-core/src/epistemics/knowledge_context.rs`) supply the actor-known place/resource facts.
2. Spec §11.1 fixes the generation inputs and the required candidate goals (eat meal, find food, sleep/rest, go to work, perform work block, return home, continue current intention, wait/idle-with-reason, leave-unsafe). Spec §11.2 fixes the switch triggers (intention completes/fails/reaches authored interruption point; severe safety; severe hunger/fatigue crossing interruption threshold; routine window expiry; precondition-failure fallback; stuck diagnostic). Spec §8.3 fixes the default rule: prefer the active intention over mild need pressure.
3. Shared boundary under audit: the generator consumes actor-known knowledge only (Phase 2A belief/observation surfaces) — it must not read ground-truth container contents, hidden item/culprit facts, or future schedule outcomes (Spec §3.3). The selection emits a `DecisionTrace` (ticket 004) consumed by the runner (017) and debug (023).
4. Invariants motivating this ticket: `INV-039` — "Needs are pressures, not puppet strings" (needs create candidate goals and pressure, never directly choose the action) — and `INV-040` — "Agents are bounded but competent" (bounded by belief, memory, access, planner budget). Severe/urgent thresholds and explicit interruption points are how an actor changes course; mild pressure does not flip an active intention.
5. Actor-knowledge / deterministic-replay surface: generation reads only actor-known facts (`INV-024` no telepathy) and is deterministic (stable comparator from ticket 002, no RNG). It emits traces and intention-transition events (ticket 005). This ticket adds no leakage — every input is a belief/observation/need/window the actor possesses — and no nondeterminism. The no-hidden-truth-planning proof is ticket 020; replay determinism is ticket 006 (both cited).

## Architecture Check

1. A generator that produces an inspectable candidate set with rejection reasons, feeding a selector that prefers active-intention continuation, directly encodes the jitter antidote (durable intentions + explicit interruption points) the spec mandates (§8.3, §25.4) — a per-tick utility argmax would lose durability and the rejected-option audit. Reading only the Phase 2A actor-known surfaces (never world truth) keeps the planner subjective by construction (`INV-024`).
2. No backwards-compatibility shims: new behavior on existing 001–004 types; it consumes the epistemics knowledge context rather than duplicating belief storage.

## Verification Layers

1. Pressures not puppets (`INV-039`) -> unit test: a rising-but-sub-severe hunger produces an `eat`/`find food` candidate but does not replace an active work intention; a severe hunger crossing the interruption threshold does switch, with a trace.
2. No telepathy (`INV-024`) -> integration test + grep-proof: the generator's inputs are belief/observation/need/window handles only; a test seeds a hidden true food location the actor does not believe and proves it is absent from the candidate inputs.
3. Determinism (`INV-018`, substrate-only) -> unit test: identical actor state yields an identical ordered candidate set and identical selection across runs; switching always emits a `DecisionTrace`. Replay enforcement is ticket 006; no-hidden-truth fixture is ticket 020 (cited).

## What to Change

### 1. Candidate-goal generator

Add `crates/tracewake-core/src/agent/generation.rs` building the §11.1 candidate set from need bands/crossings, intention status, routine-window assignment, actor-known places/resources (Phase 2A knowledge context), embodied blockers, fallback availability, and planner budget. Each candidate carries its applicability result and rejection reason; inputs are actor-known only.

### 2. Intention commitment/switching

Add the selection/commitment logic (same module or `agent/decision.rs`): apply the §8.3 tie-break, prefer active-intention continuation, and switch only on the §11.2 triggers — emitting an intention-transition event (ticket 005) and a `DecisionTrace` (ticket 004) on every adoption/continuation/switch. Silent switching is forbidden.

### 3. Module registration

Declare the new module(s) in `crates/tracewake-core/src/agent/mod.rs` and re-export the decision entry point used by ticket 017.

## Files to Touch

- `crates/tracewake-core/src/agent/generation.rs` (new)
- `crates/tracewake-core/src/agent/decision.rs` (new — selection/commitment/switching)
- `crates/tracewake-core/src/agent/mod.rs` (modify — file created by 0005PHA3ANEEROU-001; declare new modules)

## Out of Scope

- Routine method selection that turns a selected goal into a routine method (ticket 013).
- Bounded local planning of concrete action sequences (ticket 014).
- The no-human runner that calls this per decision opportunity (ticket 017).
- TUI/debug rendering of candidate goals (tickets 022, 023).

## Acceptance Criteria

### Tests That Must Pass

1. Mild (rising) hunger yields an `eat` candidate but the active work intention continues; severe hunger crossing the interruption threshold switches the intention and emits a trace.
2. The candidate set and selection are deterministic across repeated runs for identical actor state.
3. A hidden true food location the actor does not believe never appears among the generator's inputs (no-telepathy guard).

### Invariants

1. Needs create candidate goals/pressure; they never directly select the action (`INV-039`).
2. Every intention adoption/continuation/switch emits a `DecisionTrace`; switching is never silent (`INV-040`, Spec §11.2).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/generation.rs` (unit tests) — candidate set membership, actor-known-only inputs, rejection reasons.
2. `crates/tracewake-core/src/agent/decision.rs` (unit tests) — continuation-vs-switch triggers, trace emission, determinism.

### Commands

1. `cargo test -p tracewake-core agent::generation`
2. `cargo test -p tracewake-core agent::decision`
3. Core-crate scope is correct; end-to-end no-hidden-truth and replay determinism are exercised by tickets 020/006/025.
