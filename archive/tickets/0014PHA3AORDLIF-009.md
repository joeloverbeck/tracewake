# 0014PHA3AORDLIF-009: Typed no-human metrics — drop planner-failure string scans

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`projections.rs` `no_human_day_metrics`), new source guard, schema/replay gate, 1 adversarial fixture
**Deps**: 0014PHA3AORDLIF-003

## Problem

`no_human_day_metrics` counts most event kinds structurally, but classifies planner failures by scanning event payload text: `field.value.contains("planner_budget_exhausted") || (field.value.contains("planner") && field.value.contains("failed"))` (`crates/tracewake-core/src/projections.rs:211-220`). Metrics and diagnostics are proof artifacts; they must count typed responsible-layer / blocker codes, not English payload fragments (INV-105; the architecture invalid-pass rule against display-string evidence). This is ORD-HARD-007.

## Assumption Reassessment (2026-06-09)

1. The string-scan classification is `projections.rs:211-220` inside `no_human_day_metrics` (`projections.rs:165`); `NoHumanDayMetrics` is at `projections.rs:119` and `serialize_canonical` at `projections.rs:140-163`. The typed `responsible_layer` / `blocker_code` fields on decision/stuck events are introduced by ticket 0014PHA3AORDLIF-003. No-human replay/metric assertions live in `crates/tracewake-core/tests/no_human_capstone.rs` and the event-schema/replay gate in `tests/event_schema_replay_gates.rs`.
2. Spec §ORD-HARD-007 and §5.1 (guard 6) / §5.4 require: metrics read typed fields/enums; a source guard banning `.contains("planner")` / `.contains("failed")` and equivalent text matching in metrics projections; and replay rebuilding metrics from the same typed fields, byte-matching live metrics.
3. Shared boundary under audit: the typed diagnostic event fields (from -003) ↔ `no_human_day_metrics` classification ↔ replay metric rebuild. The contract: planner-failure (and other diagnostic) counts derive from typed enums, and replayed metrics byte-match live ones.
4. Invariant motivating this ticket: **INV-105** (typed/structurally-inspectable diagnostics are authoritative; display strings must not be the substrate). Arch `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` forbids display-string evidence in acceptance.
5. Deterministic-replay enforcement surface: `no_human_day_metrics` switches `planner_failures` (and any other string-classified counter) to read the typed `responsible_layer` / `blocker_code` enums from event payloads; the metrics must rebuild identically under replay and byte-match live (INV-018). Reads only typed classification fields, never hidden truth values.
6. Removal blast radius: the `.contains("planner_budget_exhausted")` / `.contains("planner")` / `.contains("failed")` scan at `projections.rs:211-220` is removed; grep confirms this is the metrics string-classification site (the `serialize_canonical` format string at `projections.rs:143` is output formatting, not classification, and is untouched). The guard bans reintroduction of `.contains("planner")` / `.contains("failed")` in metrics projections.

## Architecture Check

1. Reading typed `ResponsibleLayer` / `BlockerCode` enums makes the metric a deterministic count over machine-checked classification rather than fragile English matching — eliminating both false positives (a harmless explanation containing "planner") and false negatives (a failure phrased without the token).
2. No backwards-compatibility shim: the string scan is deleted, not kept as a fallback when a typed field is absent (history without the field is handled by -003's additive defaults, not by re-scanning text).

## Verification Layers

1. INV-105 (typed metric substrate) -> codebase grep-proof: source guard bans `.contains("planner")` / `.contains("failed")` and equivalent text matching in metrics projections; `no_human_day_metrics` reads the typed enums.
2. INV-018 (replay byte-match) -> replay/golden-fixture check: `tests/no_human_capstone.rs` / `tests/event_schema_replay_gates.rs` prove replay-rebuilt no-human metrics byte-match live metrics from the typed fields.
3. INV-105 (proof not strings) -> replay/golden-fixture check: `no_human_metrics_require_typed_responsible_layer_001` — a planner-failure diagnostic lacking the typed `responsible_layer` is not counted via text, and a typed one is counted.

## What to Change

### 1. Typed planner-failure classification

In `crates/tracewake-core/src/projections.rs`, replace the `projections.rs:211-220` string scan with a count over the typed `responsible_layer` / `blocker_code` enums (from ticket -003) on `DecisionTraceRecorded` / `StuckDiagnosticRecorded` events.

### 2. Replay byte-match gate

Ensure replayed no-human metrics rebuild from the same typed fields and byte-match live; extend the schema/replay gate accordingly.

### 3. Source guard + fixture

Add the metrics-string-scan ban guard and the adversarial fixture.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify — `no_human_day_metrics` typed classification; **also touched by 0014PHA3AORDLIF-008**)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — guard #6; **N-way shared hub**)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify — typed-metric replay byte-match)
- `crates/tracewake-content/src/fixtures/no_human_metrics_require_typed_responsible_layer_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register fixture; **shared hub**)

## Out of Scope

- Adding the typed `responsible_layer` / `blocker_code` fields (ticket 0014PHA3AORDLIF-003 — this ticket consumes them).
- The embodied workplace projection (ticket 0014PHA3AORDLIF-008, also touches `projections.rs`).

## Acceptance Criteria

### Tests That Must Pass

1. `no_human_metrics_require_typed_responsible_layer_001` — a planner-failure diagnostic without the typed `responsible_layer` is not counted by text; a typed one is counted.
2. `cargo test -p tracewake-core --test anti_regression_guards` — metrics-string-scan ban guard passes.
3. `cargo test -p tracewake-core --test no_human_capstone` — replay-rebuilt no-human metrics byte-match live metrics.

### Invariants

1. No-human metrics classify by typed enums, never by payload text (INV-105).
2. Replayed metrics byte-match live metrics from the same typed fields (INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/no_human_metrics_require_typed_responsible_layer_001.rs` — typed-vs-text metric classification.
2. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — replay byte-match of typed metrics.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — guard banning metric string scans.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-core --test no_human_capstone`
3. `cargo test --workspace`

## Outcome (2026-06-09)

Completed. `no_human_day_metrics` now classifies planner failures from typed
diagnostic payload fields (`responsible_layer` / `blocker_code`) instead of
scanning payload English. Untyped planner-looking trace text is ignored; typed
local-planning or planner-specific blocker codes are counted.

Added `no_human_metrics_require_typed_responsible_layer_001` as the content
fixture anchor for this proof surface, extended the core metrics unit test with
typed-vs-text diagnostic events, added a replay gate proving canonical log
replay rebuilds byte-identical metrics from typed fields, and added an
anti-regression guard banning metrics `.contains("planner")` / `.contains("failed")`
classification.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo test -p tracewake-core --test event_schema_replay_gates`
4. `cargo test -p tracewake-core --test no_human_capstone`
5. `cargo test -p tracewake-content`
6. `cargo test --workspace`
