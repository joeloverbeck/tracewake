# 0002TUIPROOSUR-012: Positive proof fixtures and typed acceptance tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-content` (harden existing fixtures), `tracewake-core`/`tracewake-tui` (positive proof tests)
**Deps**: 0002TUIPROOSUR-003, 0002TUIPROOSUR-007, 0002TUIPROOSUR-008, 0002TUIPROOSUR-009, 0002TUIPROOSUR-010, 0002TUIPROOSUR-011

## Problem

Spec 0002 §7.1 requires six positive proof fixtures that assert the hardened seam through typed artifacts first (typed ids, report status, event ids, checksums) and rendered transcript second. Matching fixtures already exist under `crates/tracewake-content/src/fixtures/`; per Spec 0002 §7 the requirement is to *harden the existing fixtures* to the typed bar, not author parallel ones. This ticket lands the positive proof surface that exercises tickets 001–011 end-to-end (the adversarial/negative gates and the §8 acceptance capstone are ticket 013).

## Assumption Reassessment (2026-06-08)

1. Existing fixtures cover the §7.1 scenarios: `ordinary_workday_001`, `view_model_local_actions_001` (embodied action path); `hidden_food_closed_container_001`, `hidden_food_unknown_route_001` (actor-safe why-not); `expectation_contradiction_001` (source-bound notebook); `debug_omniscience_excluded_001` (debug quarantine); `possession_parity_001` (possession parity); `no_human_day_001`, `replay_item_location_001` (deterministic transcript/replay) — all in `crates/tracewake-content/src/fixtures/`. Typed surfaces they must assert are produced by tickets 001–011.
2. Spec 0002 §7.1 enumerates the six positive fixtures and §7.3 the review-artifact format; §7 directs hardening existing fixtures over recreating them.
3. Cross-artifact boundary under audit: the content fixtures ↔ the typed view-model/proposal/report/replay artifacts (tickets 001–010) ↔ the canonical transcript/replay ordering (ticket 011). Each positive fixture asserts typed artifacts first.
4. Invariants restated: **INV-095** — TUI/view-model tests are acceptance tests for every runnable phase; **INV-092** — deterministic replay is tested.
5. Fail-closed / replay / no-leak surface (test-side): these positive fixtures assert the typed ids/report status/event ids/checksums produced by the hardened seam and then the rendered transcript, so the proof is typed-artifact-first (PSL-004). They confirm — not enforce — the no-leak/determinism properties the upstream tickets implement; the adversarial enforcement is ticket 013.

## Architecture Check

1. Hardening the existing fixtures rather than authoring parallel ones keeps one canonical fixture per scenario (no drift between an "old" and "new" hidden-food fixture) and matches Spec 0002 §9's no-duplicate stance; asserting typed artifacts before transcript text makes the proof robust to rendering changes (PSL-004).
2. No backwards-compatibility aliasing/shims: no duplicate fixture files are created; the existing fixtures are upgraded in place.

## Verification Layers

1. INV-095 (acceptance via view models) -> replay/golden-fixture check: each positive fixture asserts typed ids/report status/event ids/checksums through the view-model path.
2. INV-092 (replay tested) -> replay/golden-fixture check: the deterministic transcript/replay fixture reproduces matching physical/agent checksums.
3. INV-067/INV-093 (actor-known) -> manual review: positive notebook/why-not fixtures assert typed actor-known content with no ground-truth leak.

## What to Change

### 1. Harden positive fixtures

Upgrade the existing fixtures listed above so their tests assert typed artifacts (context id/hash, semantic ids, report status, event ids, checksums) first, then rendered transcript.

### 2. Positive proof tests

Add/extend tests realizing §7.1 items 1–6 (embodied action path; actor-safe why-not; source-bound notebook; debug quarantine non-mutation; possession parity; deterministic transcript/replay), each emitting the §7.3 review-artifact fields (responsible layer, scenario id, context/actor ids, proposal/semantic ids, typed reason codes/provenance, debug-capability presence, surfaces checked, expected event/log/replay/checksum result).

## Files to Touch

- `crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs` (modify — as surfaced; harden assertions)
- `crates/tracewake-content/src/fixtures/hidden_food_closed_container_001.rs` (modify — as surfaced)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify)

## Out of Scope

- Adversarial/negative gates and §7.3 artifact *format* enforcement (ticket 013).
- The §8 acceptance checklist capstone (ticket 013).
- New production logic — this ticket is verification-only (fixtures + tests).

## Acceptance Criteria

### Tests That Must Pass

1. The six positive fixtures (§7.1) pass with typed-artifact-first assertions: embodied action path produces typed ids + report status + event ids + checksums; actor-safe why-not, source-bound notebook, debug quarantine non-mutation, possession parity, and deterministic transcript/replay each assert their typed surfaces.
2. Each positive test emits the §7.3 review-artifact fields.
3. `cargo test --workspace` passes; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Acceptance is via typed view-model/proposal/report/replay artifacts first, transcript second (INV-095/PSL-004).
2. Deterministic replay is exercised (INV-092).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_flow.rs`, `tui_acceptance.rs` — typed-artifact-first positive proofs.
2. `crates/tracewake-core/tests/golden_scenarios.rs`, `acceptance_gates.rs` — replay/checksum + typed-id assertions for the hardened fixtures.

### Commands

1. `cargo test -p tracewake-tui && cargo test -p tracewake-core`
2. `cargo test -p tracewake-content`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
