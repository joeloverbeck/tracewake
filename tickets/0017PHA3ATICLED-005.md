# 0017PHA3ATICLED-005: Materialize agent event payloads for replay verifiability

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`events/apply`, `state`, `checksum`, capstone test); tamper gates in `tracewake-content` golden tests
**Deps**: `archive/tickets/0017PHA3ATICLED-004.md` (replays the corrected durations and provenance; transitive over `-003`/`-002`/`-001`); `specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (ORD-HARD-028)

## Problem

Four agent event kinds remain semantically unverifiable under replay: `CandidateGoalsEvaluated` and `ContinueRoutine{Proposed,Accepted,Rejected}` sit in `events/apply.rs::AGENT_WORLD_NOOP_ALLOWLIST`, mutate no rebuilt state, and appear in no checksum family — flipping `Accepted`↔`Rejected` or rewriting a rejection `reason` in a stored log survives replay byte-identically (checksums unchanged, metrics kind-counts unchanged, context-hash gate untouched). Separately, the flagship acceptance test `no_human_capstone.rs::no_human_capstone_proves_typed_ancestry_and_replay` never asserts `decision_context_hash_failures` — the 0016 gate's result is computed and discarded where it matters most (INV-018, INV-105; 0005 §15.1 requires replay to reconstruct continue-routine and candidate-goal evaluations).

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: `AGENT_WORLD_NOOP_ALLOWLIST` (apply.rs) lists exactly `CandidateGoalsEvaluated`, `FoodConsumed`, `ContinueRoutineProposed/Accepted/Rejected`, `NoHumanDayStarted/Completed`; `checksum.rs::AGENT_STATE_CHECKSUM_COVERAGE` has no continue-routine or candidate-goal family; `no_human_capstone.rs` contains no reference to `decision_context_hash_failures`; the materialization precedent exists — `state.rs` already carries `need_threshold_crossings` and `ordinary_life_episodes` as event-id-keyed `BTreeMap` ledgers from 0016.
2. Spec 0017 §ORD-HARD-028: materialize continue-routine arbitration and candidate-goal evaluation summaries into `AgentState` with checksum coverage; shrink the allowlist to `NoHumanDayStarted/Completed` plus the classified dual-stream `FoodConsumed`; add the capstone assertion; tamper-flip gates. Per the user-approved either-or (option a), no V0→V1 upcast fixture is added — the schema-contract claim is corrected documentation-side in ticket `-010`.
3. Cross-artifact boundary under audit: the event-payload → rebuilt-`AgentState` → checksum-coverage chain — every agent event kind must either materialize into checksummed state or be a registered payload-free marker.
4. INV-018/INV-105 restated: deterministic replay must reconstruct significant outcomes, and diagnostics/arbitration records are authoritative data — an event class whose semantic content can be tampered without any gate failing is decorative history.
5. Deterministic-replay surface touched (strengthened): two new `AgentState` ledger families enter `compute_agent_state_checksum`; the existing census `new_authoritative_field_without_checksum_registry_fails` forces their registration. Replay rebuild gains apply arms for the four kinds (no longer `WorldNoOp`); golden agent checksums change accordingly and replay is byte-stable at the new coverage. No epistemic-leakage surface: the ledgers record what events already state.
6. Schema extension keyed separately from item 5: `AgentState` (entity/component state) gains two additive ledger families — continue-routine arbitration records and candidate-goal evaluation records, event-id-keyed like `ordinary_life_episodes`. Consumers: `events/apply.rs` (writer), `checksum.rs` (coverage), `replay/rebuild.rs` (rebuilt implicitly via apply), debug reports (optional reader). Additive-only: no existing field changes shape; serialization of `AgentState` is internal (not a save-package schema bump).
7. Adjacent contradiction classified: the 0016 acceptance report's unbacked upcast-fixture claim is documentation drift, owned by ticket `-010` (per the approved option a), not by this ticket.

## Architecture Check

1. Materializing into event-id-keyed `AgentState` ledgers follows the proven 0016 pattern (`ordinary_life_episodes`) and is cleaner than folding payload digests into `DecisionTraceRecord`: it keeps arbitration outcomes first-class rebuilt state covered by the agent checksum, rather than expanding the trace record's responsibilities and hash surface. The allowlist shrinks to genuinely payload-free markers, making "allowlisted" mean something again (INV-105).
2. No backwards-compatibility aliasing/shims: the four kinds' `WorldNoOp` arms are removed outright; the allowlist census test is tightened, not given exceptions.

## Verification Layers

1. INV-018 tamper-evidence -> replay tamper gates: flip `ContinueRoutineAccepted`→`Rejected` and rewrite a rejection `reason` in copied golden logs; assert `!matches_expected` in both.
2. INV-105 capstone coverage -> `no_human_capstone.rs` asserts `rebuild.decision_context_hash_failures.is_empty()` and the phase-3A checklist mapping gains a `decision_context_hash_gate` item.
3. Allowlist contract -> census test extension: every remaining allowlist entry either cites its capturing projection/checksum (dual-stream `FoodConsumed` → physical checksum) or is a registered payload-free marker kind.
4. Checksum census -> existing `new_authoritative_field_without_checksum_registry_fails` covers the two new ledger fields (fails until registered).

## What to Change

### 1. Ledger materialization in `state.rs` + `events/apply.rs`

Add `continue_routine_arbitrations` and `candidate_goal_evaluations` event-id-keyed records to `AgentState` (mirroring `OrdinaryLifeEpisodeRecord`); `apply_agent_event_with_capability` gains apply arms for the four kinds; remove them from `AGENT_WORLD_NOOP_ALLOWLIST`.

### 2. Checksum coverage in `checksum.rs`

Register both families in `AGENT_STATE_CHECKSUM_COVERAGE`; include their content in `compute_agent_state_checksum`.

### 3. Capstone assertion

`no_human_capstone.rs`: assert `decision_context_hash_failures.is_empty()` with failure detail; extend `assert_phase3a_checklist_is_mapped` with the gate item.

### 4. Tamper gates + allowlist census tightening

Two tamper tests in `golden_fixtures_run.rs` (flip + reason-rewrite ⇒ poisoned replay); extend the allowlist census in `anti_regression_guards.rs` so each remaining entry carries its capturing-surface citation or payload-free registration.

## Files to Touch

- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/checksum.rs` (modify)
- `crates/tracewake-core/tests/no_human_capstone.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — allowlist census test)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — tamper gates; golden agent-checksum constants as surfaced)

## Out of Scope

- A V0→V1 upcast fixture (user-approved option a: the report claim is corrected in ticket `-010`; the loud-rejection contract is already tested).
- Decision-trace record shape changes; provenance audits (tickets `-003`/`-004`).
- Metrics projection changes (kind-counts remain as they are; verifiability now comes from materialized state).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content continue_routine_tamper` — both tamper gates poison replay (`!matches_expected`).
2. `cargo test -p tracewake-core no_human_capstone` — capstone passes with the context-hash assertion active.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No agent `EventKind` applies as `WorldNoOp` unless it is a registered payload-free marker or a dual-stream kind citing its capturing checksum.
2. Continue-routine arbitration outcomes and candidate-goal evaluations are rebuilt state under the agent checksum — semantic tampering of their payloads is replay-detectable.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — `continue_routine_accepted_flip_poisons_replay` and `continue_routine_reason_rewrite_poisons_replay` tamper gates.
2. `crates/tracewake-core/tests/no_human_capstone.rs` — context-hash-failures assertion + checklist item.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — tightened allowlist census.
4. `crates/tracewake-core/src/events/apply.rs` (unit tests) — apply arms build the ledger records deterministically.

### Commands

1. `cargo test -p tracewake-core no_human_capstone`
2. `cargo test --workspace`
