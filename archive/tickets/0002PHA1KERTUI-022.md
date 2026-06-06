# 0002PHA1KERTUI-022: Capstone — golden scenarios, testing matrix, and Phase 1 acceptance gates

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the integration test suite (and any shared golden-test harness) exercising the full Phase 1 slice; flips the Spec 0002 `Status` once gates pass.
**Deps**: 0002PHA1KERTUI-019, 0002PHA1KERTUI-013, 0002PHA1KERTUI-014, 0002PHA1KERTUI-015, 0002PHA1KERTUI-016, 0002PHA1KERTUI-021

## Problem

Phase 1 passes only when the whole slice runs end-to-end: the seven golden fixtures execute their scenarios, the §19.1 testing matrix passes, and the §19.2 / §21 acceptance gates hold (deterministic load, shared-path parity, structured why-not, event-only mutation, projection rebuild, replay match + drift detection, debug provenance, TUI playability, no-human advance, possession parity, no-leak, LLM-disabled, no authored outcome chains). This capstone exercises every prior ticket end-to-end; it introduces no new production logic beyond the test harness itself (deliverable-doubles-as-capstone — the verification harness is a Phase 1 deliverable, §19/§21).

## Assumption Reassessment (2026-06-06)

1. Every implementation ticket (001–021) has landed: the kernel (003–016), content (017–019), and TUI (020–021). This ticket adds integration tests under each crate's `tests/` plus a top-level golden-scenario suite; it does not modify production modules.
2. The acceptance gates are `specs/0002_…_SPEC.md` §19.2 (23 gates) and §21 exit criteria; the testing matrix is §19.1; the minimum regression test names are §19.3. They map to the Phase 1 acceptance gates in `docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md` (no-human run, deterministic replay, TUI playability, why-not) and `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`.
3. Shared boundary under audit: this ticket exercises the composed pipeline (008), events/replay (006/013), view models (012), debug reports (016), controller binding (014), no-human (015), content/fixtures (017–019), and TUI (020–021) — it is the integration surface, owning no production logic.
4. Invariant motivating this ticket: INV-098 (feature acceptance is harsh — done only when caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human-runnable, replay-safe, LLM-independent, non-scripted, and regression-tested); plus INV-091/092/093/094/095/097 (the no-human, replay, leakage, possession-parity, TUI, and no-script test gates).
5. Fail-closed / replay / no-leak surface: the capstone is where every enforcement surface is exercised together — replay drift fails the run (INV-018), the debug-leakage negative test asserts embodied views never gain hidden facts (INV-093), possession parity asserts binding changes nothing physical (INV-094), the LLM-disabled run asserts no Phase 1 behavior needs an LLM (INV-077), and content-script rejection asserts the no-scripting gate (INV-097). It weakens none of them; it proves them. Re-enumerate expected counts (events, fixtures) from each fixture at test start rather than hardcoding.

## Architecture Check

1. A single capstone suite mapping each §19.2 gate to a test sub-case (re-enumerating counts from fixtures) gives one authoritative pass/fail surface for Phase 1 exit, mirroring the acceptance formula in `docs/2-execution/02_…ACCEPTANCE_GATES.md`. Distributing acceptance across the trailing tickets without a capstone would leave no single "Phase 1 is done" proof. It adds no production logic — it exercises the pipeline the earlier tickets composed.
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. Deterministic replay + drift (INV-018/092) -> replay/golden-fixture check: every eventful golden scenario replays to a matching checksum; a deliberately mutated event log fails replay.
2. No-human advance (INV-004/091) -> replay/golden check: `no_human_advance_001` runs with no controller and replays.
3. Possession parity (INV-094) -> integration test: `debug_attach_001` — binding changes alter no physical checksum and no actor knowledge.
4. No-leak (INV-024/093) -> integration test: embodied view never contains hidden item location; debug reveals it; returning to embodied still hides it; checksum unchanged by debug.
5. Shared-path parity (INV-007) -> integration test: human-origin and no-human/test-origin proposals share validation path and produce identical reports/events.
6. No-scripting + LLM-disabled (INV-077/097) -> schema validation + full run: content scripts/quest/player constructs are rejected; all gates pass with no LLM dependency.
7. TUI playability (INV-095) -> transcript/snapshot test: `view_model_local_actions_001` reaches the mechanics through the TUI deterministically.

## What to Change

### 1. Golden scenario suite

Add integration tests executing each fixture's §18 scenario contract end-to-end: `strongbox_001`, `container_item_move_001`, `door_access_001`, `debug_attach_001`, `no_human_advance_001`, `replay_item_location_001`, `view_model_local_actions_001` — each asserting expected events/reports, replay match, and the per-fixture negative assertions.

### 2. Testing matrix + regression names

Add tests covering every §19.1 matrix row, using the §19.3 minimum regression test names (e.g. `loads_fixtures_deterministically`, `human_and_nonhuman_proposals_share_validation_path`, `replay_detects_missing_or_reordered_event`, `debug_truth_does_not_enter_embodied_view`, `llm_disabled_phase1_still_passes`).

### 3. Acceptance-gate assertion + status flip

Add a gate-checklist test enumerating the §19.2 / §21 gates. On green, flip `specs/0002_…_SPEC.md` `Status` to reflect Phase 1 landed (and note the `docs/4-specs/SPEC_LEDGER.md` Spec 0002 entry — already added during this session's `/reassess-spec`).

## Files to Touch

- `crates/tracewake-core/tests/golden_scenarios.rs` (new)
- `crates/tracewake-core/tests/acceptance_gates.rs` (new)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (new)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (new)
- `specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` (modify — `Status` flip on green; existing file)

## Out of Scope

- Any new production logic (this is a verification capstone; behavior lives in 001–021).
- Phase 2+ systems (beliefs, routines, institutions) — explicitly deferred (§22).

## Acceptance Criteria

### Tests That Must Pass

1. All seven golden fixtures pass their §18 scenario contracts (events, reports, replay, negative assertions).
2. Every §19.1 testing-matrix row has a passing test; the §19.3-named regressions pass.
3. The §19.2 / §21 acceptance gates all hold: deterministic load, shared-path parity, structured why-not, event-only mutation, projection rebuild, replay match + drift detection, debug provenance, TUI playability, no-human advance, possession parity, no-leak, LLM-disabled, no authored outcome chains.

### Invariants

1. Phase 1 acceptance is all-or-nothing: a failure in any gate is a Phase 1 failure, not a known limitation (§19.2).
2. The capstone introduces no production logic; it only exercises composed behavior.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/golden_scenarios.rs` — the seven scenario contracts + replay/drift.
2. `crates/tracewake-core/tests/acceptance_gates.rs` — §19.2/§21 gate checklist.
3. `crates/tracewake-content/tests/golden_fixtures_run.rs` — fixture execution + no-script rejection.
4. `crates/tracewake-tui/tests/tui_acceptance.rs` — TUI playability + transcript determinism + debug non-leakage.

### Commands

1. `cargo test --workspace`
2. `cargo test --workspace -- --nocapture replay no_human possession leakage llm_disabled`
3. A workspace-wide test run is the correct boundary: the capstone's purpose is to prove the whole Phase 1 slice composes, so it must exercise all three crates together.

## Outcome (2026-06-06)

Added the Phase 1 capstone integration suite across all three crates:

1. `crates/tracewake-core/tests/golden_scenarios.rs` covers accepted versioned events, projection rebuild, replay checksum match, replay drift detection, item-location debug provenance, rejection debug provenance, controller/possession non-world state, and no-human advance.
2. `crates/tracewake-core/tests/acceptance_gates.rs` covers shared human/non-human validation path, blocked movement why-not, closed-container take rejection, and deterministic event append order.
3. `crates/tracewake-content/tests/golden_fixtures_run.rs` covers all seven fixtures loading deterministically, validating, declaring contracts, and rejecting missing IDs, unsupported targets, ordering hazards, player-only forms, quest/script forms, and LLM-disabled operation.
4. `crates/tracewake-tui/tests/tui_acceptance.rs` covers semantic action identity, transcript determinism, debug non-leakage, why-not, wait, and debug panels through the TUI facade.

Flipped Spec 0002 status from `Implementable Phase 1 specification` to `Phase 1 landed`.

Verification:

1. `cargo test --workspace` passed.
2. `cargo test --workspace -- --nocapture replay no_human possession leakage llm_disabled` passed and exercised all named gate filters.
3. `cargo build --workspace` passed.
