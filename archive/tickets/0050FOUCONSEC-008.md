# 0050FOUCONSEC-008: Replay fail-closed — temporal violations gate `matches_expected`

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — folds temporal-violation emptiness into the replay aggregate verdict and adds a typed temporal first-divergence
**Deps**: None

## Problem

Spec-0050 §4.5 (driver F-05): `run_replay` computes `matches_expected` from checksum/state/version/error/decision-context conditions but **omits** `rebuild.temporal_violations.is_empty()` (`crates/tracewake-core/src/replay/report.rs:101`); the temporal violations are exposed only as the separate `ReplayReport::temporal_violations` field (`report.rs:53`, populated at `report.rs:140`). A consumer trusting the aggregate boolean can receive `matches_expected == true` alongside a temporal divergence — fail-open evidence that makes `INV-112` enforcement optional for report consumers.

This ticket includes `rebuild.temporal_violations.is_empty()` in `matches_expected` and extends first-divergence reporting with a typed temporal family/detail. This surface is independent of the world-step rework (`replay/report.rs` only), so it carries no `Deps`.

## Assumption Reassessment (2026-06-24)

1. `run_replay` computes `matches_expected` at `crates/tracewake-core/src/replay/report.rs:101` without a `temporal_violations.is_empty()` conjunct; `temporal_violations: Vec<TemporalDivergence>` is the separate field at `report.rs:53`, populated from `rebuild.temporal_violations` at `report.rs:140`; `TemporalDivergence` is at `crates/tracewake-core/src/replay/temporal.rs:12`. The temporal projector (`replay/temporal.rs`) already detects the divergence. Verified at `HEAD` (`8d7c119`).
2. Spec-0050 §4.5 is authoritative; this is a fail-open → fail-closed correction with no doctrine amendment (spec §5). The `0049` replay witnesses cover `validate_time_advanced` / rebuild-marker / duplicate-need-tick predicates but not this report-level aggregation; they are preserved.
3. Shared boundary under audit: the replay aggregate-verdict boundary in `crates/tracewake-core/src/replay/report.rs` and the typed-divergence enum in `replay/temporal.rs`. Consumers of `matches_expected`: acceptance/golden/replay lanes that gate on the aggregate boolean.
4. `INV-018` (deterministic replay is foundational), `INV-092` (deterministic replay is tested), `INV-112` (temporal authority — every accepted step carries ancestry to rebuild the frontier) motivate this ticket: a temporal-ancestry failure must make replay fail, not coexist with success.
5. Enforcement surface: deterministic replay / fail-closed evidence (`INV-018`/`INV-092`/`INV-112`). Folding the temporal conjunct into `matches_expected` makes the aggregate verdict fail-closed on temporal divergence — strictly stricter; it relaxes nothing and adds no nondeterministic input. No actor-knowledge surface is touched.
6. **Schema/contract shape change (`ReplayReport` verdict semantics; additive-vs-breaking)**: `matches_expected`'s *semantics* change (it now returns `false` for a temporal-only divergence that previously read `true`) and first-divergence reporting gains a typed temporal family/detail. The field set is additive (typed first-divergence detail), but the verdict semantics change is **breaking** for any consumer/golden that recorded a passing verdict over a temporally-divergent log — intentional fail-closed tightening, not additive. Consumers: the replay/golden/acceptance lanes; one normal lane must rely on the aggregate verdict (criterion below).

## Architecture Check

1. Gating `matches_expected` on the already-computed `temporal_violations` is the minimal correct fix: the detector exists; only the aggregate omitted its signal. A typed temporal first-divergence means a temporal failure surfaces as a first-class divergence, not only an auxiliary list — so consumers gating on the aggregate need no bespoke secondary check.
2. No backwards-compatibility shims: the aggregate simply incorporates the conjunct; no opt-out flag preserving the old fail-open behavior is added.

## Verification Layers

1. `INV-018`/`INV-112` (fail-closed temporal replay) → replay/golden-fixture check: an integration witness builds a log whose physical/agent/checksum result otherwise matches expected but whose `TimeAdvanced` ancestry is invalid; asserts `temporal_violations` nonempty AND `matches_expected == false`, via `run_replay`.
2. `INV-092` (replay tested at the product boundary) → replay/golden-fixture check: at least one normal replay/golden lane relies on the aggregate verdict, proving the correction reaches the evidence boundary.

## What to Change

### 1. Fold the temporal conjunct into `matches_expected`

In `crates/tracewake-core/src/replay/report.rs`, add `rebuild.temporal_violations.is_empty()` to the `matches_expected` computation (`report.rs:101`).

### 2. Typed temporal first-divergence

Extend first-divergence reporting so a temporal failure surfaces as a typed temporal family/detail (drawing on `TemporalDivergence`, `replay/temporal.rs:12`) rather than only the auxiliary `temporal_violations` list.

### 3. Witness

In `crates/tracewake-core/tests/replay_temporal_frontier.rs`, add the report-level integration witness (otherwise-matching log + invalid `TimeAdvanced` ancestry ⇒ `matches_expected == false`), calling `run_replay` (not just `validate_time_advanced`).

## Files to Touch

- `crates/tracewake-core/src/replay/report.rs` (modify)
- `crates/tracewake-core/src/replay/temporal.rs` (modify)
- `crates/tracewake-core/tests/replay_temporal_frontier.rs` (modify)

## Out of Scope

- The temporal projector / `validate_time_advanced` predicates themselves (already correct; `0049` witnesses preserved).
- Any change to non-temporal `matches_expected` conditions.

## Acceptance Criteria

### Tests That Must Pass

1. The report-level integration witness: a log matching all non-temporal success conditions but with invalid `TimeAdvanced` ancestry yields `temporal_violations` nonempty and `matches_expected == false`, via `run_replay`.
2. A clean log still yields `matches_expected == true` (no false negatives introduced).
3. `cargo test -p tracewake-core --test replay_temporal_frontier` and `cargo build --workspace --all-targets --locked` are green.

### Invariants

1. `matches_expected` is `false` whenever temporal reconstruction diverges (`INV-018`/`INV-092`/`INV-112`).
2. A temporal failure surfaces as a typed first-divergence, not only an auxiliary list.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/replay_temporal_frontier.rs` — report-level fail-closed integration witness via `run_replay` + clean-log no-false-negative check.

### Commands

1. `cargo test -p tracewake-core --test replay_temporal_frontier`
2. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Implemented fail-closed replay aggregate semantics for temporal violations by adding `rebuild.temporal_violations.is_empty()` to `ReplayReport::matches_expected`. Added a typed `Temporal` first-divergence family and derives the first divergent event ID from `TemporalDivergence` when there is no state diff.

Added report-level temporal replay witnesses proving that an otherwise matching log with invalid `TimeAdvanced` ancestry yields nonempty `temporal_violations`, an empty state diff, `matches_expected == false`, and a typed temporal first divergence, while a clean marker still matches. Verification passed:

1. `cargo test -p tracewake-core --test replay_temporal_frontier`
2. `cargo build --workspace --all-targets --locked`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo fmt --all --check`
5. `git diff --check`
