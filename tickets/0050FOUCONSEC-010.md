# 0050FOUCONSEC-010: Evidence rebuild & reachability-lock replacement

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — replaces the reachability-overstating `0048` behavioral locks and extends the deterministic generative + compile-fail + parity harnesses
**Deps**: 0050FOUCONSEC-003, 0050FOUCONSEC-005, 0050FOUCONSEC-006, 0050FOUCONSEC-007, 0050FOUCONSEC-008, 0050FOUCONSEC-009

## Problem

Spec-0050 §4.7 (driver F-07) + §6.2/§6.4: the `0048` behavioral locks overstate production reachability — the loaded-world differential manufactured the populations production left empty (F-01), the salient witness was semantically weak (F-06), and the replay temporal tests didn't make the aggregate report fail (F-05). The narrow `0049` mutation witnesses are structurally non-vacuous and are **preserved**; the `0048` reachability-overstating locks must be **replaced, not supplemented**. This ticket consolidates the cross-cutting evidence: extend `generative_lock.rs` with production-shaped schedules + hidden-world pairs + order invariance + prefix replay, add adversarial parity variants distinguishing the four evidence classes, and complete the compile-fail corpus (actor-step outcome boundary, TUI perception export, debug→embodied conversion).

## Assumption Reassessment (2026-06-24)

1. The deterministic generative harness is `crates/tracewake-core/tests/generative_lock.rs`; parity lives in `crates/tracewake-tui/tests/playable_capability_parity.rs` + `parity_adversarial.rs`; the compile-fail corpus is `tests/negative-fixtures/` driven by `crates/tracewake-core/tests/negative_fixture_runner.rs`. All verified at `HEAD` (`8d7c119`). The `0048` differential/salient/replay locks live in `world_step_coordinator.rs` / `salient_stop_actor_known.rs` / `replay_temporal_frontier.rs` (the production-path witnesses landed in `-003`/`-008`/`-009` replace them).
2. Spec-0050 §4.7/§6.2/§6.4 are authoritative: extend the existing deterministic harness — do **not** add `proptest`/`quickcheck` (§6.4 deliberate rejection-for-now). Preserve the `0049` witnesses; replace the overstating `0048` locks.
3. Shared boundary under audit: the cross-cutting evidence surfaces (generative harness, parity runner, compile-fail corpus) that several findings' guards reference. Adjacent contradiction: each finding's *primary* witness already landed in its own ticket (`-003`/`-005`/`-006`/`-008`/`-009`); this ticket owns only the consolidated/relational evidence, not those primaries.
4. `INV-091` (no-human tests), `INV-094` (possession parity tested), `INV-098` (harsh feature acceptance) motivate this ticket: regression evidence must measure the significant causal scenario on the production path, not a test-only substitute; the paired hidden-world case is the truth-firewall noninterference proof.
5. Enforcement surface: this ticket audits/extends the replay, actor-knowledge, and no-direct-dispatch enforcement surfaces (evidence-consumer basis). It introduces no production behavior; it must not weaken any enforcement — only add measuring witnesses. The generative schedules must be recorded-seed deterministic with prefix-replay and storage-order invariance; the hidden-world pairs must prove a hidden fact does not alter actor-known output.

## Architecture Check

1. Extending the existing deterministic harness with production-shaped mixed schedules + hidden-world pairs + order invariance + prefix replay is the spec-mandated path (§6.4) — it reuses the recorded-seed/ratcheted-witness machinery rather than adding a property-testing dependency, and it replaces (not supplements) the overstating locks so a future audit can't cite stale breadth.
2. No backwards-compatibility shims: the reachability-overstating `0048` locks are removed/replaced; no decorative lock is retained.

## Verification Layers

1. `INV-091`/`INV-094` (no-human + possession parity on the real path) → replay/golden-fixture check: generative mixed schedules (controlled input + discovered actors + declared processes + interval stop) with human-vs-no-human origin equivalence, run without test-built final events.
2. `INV-098` (harsh acceptance; noninterference) → manual review + actor-knowledge negative: paired hidden-world cases (same holder-known history, changed hidden counterpart) leave actor-known output identical; adversarial parity variants (omit actor invocation, omit process, hidden same-actor-id event, externally-supplied tick with a removed marker, populated-witness/empty-evidence) each fail the real runner.
3. Compile-time boundary → compile-fail check: corpus fixtures for the actor-step outcome boundary, TUI perception-export, and debug→embodied conversion fail to compile.

## What to Change

### 1. Extend the deterministic generative harness

In `crates/tracewake-core/tests/generative_lock.rs`, add world-step schedule generation: mixed controlled input + discovered actors + declared processes + interval stop, **without** test-built final events; recorded seeds; storage-order invariance (metamorphic); full-run-vs-every-prefix replay; one relevant causal change at tick k with quiet ticks before it; duplicate/missing causal marker or event identity as a negative; same world/seed human-vs-no-human origin.

### 2. Adversarial parity variants

In `crates/tracewake-tui/tests/parity_adversarial.rs` (+ `playable_capability_parity.rs` as needed), add variants that deliberately omit actor invocation, omit world-process invocation, pass a hidden same-actor-id event with no holder-known source, supply a desired final tick while removing a marker, and leave a witness description populated while returning empty typed evidence — each must fail the real conformance runner. Distinguish the four evidence classes (duration/accounting, loaded-world actor/process, temporal replay, holder-known noninterference).

### 3. Complete the compile-fail corpus

In `tests/negative-fixtures/` (+ `negative_fixture_runner.rs`), add fixtures (as surfaced) for: the closed actor-step outcome cannot be reduced to `Option<Proposal>` externally; event-appending perception helpers are not exported to the TUI crate; a debug step report cannot be converted into an embodied interval summary.

### 4. Replace the reachability-overstating `0048` locks

Remove/replace the `0048` differential/salient/replay locks that the production-path witnesses (`-003`/`-008`/`-009`) supersede, so no overstating lock remains cited as current proof.

## Files to Touch

- `crates/tracewake-core/tests/generative_lock.rs` (modify)
- `crates/tracewake-tui/tests/parity_adversarial.rs` (modify)
- `crates/tracewake-tui/tests/playable_capability_parity.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify)
- `tests/negative-fixtures/` (new — actor-step / perception-export / debug→embodied compile-fail fixtures, as surfaced)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — remove the superseded overstating `0048` differential lock)
- `crates/tracewake-core/tests/salient_stop_actor_known.rs` (modify — remove the superseded weak salient lock)

## Out of Scope

- Each finding's primary behavior witness — owned by `-003`/`-005`/`-006`/`-008`/`-009`.
- Mutation campaigns — `0050FOUCONSEC-011`.
- Documentation truthing — `0050FOUCONSEC-012`.

## Acceptance Criteria

### Tests That Must Pass

1. The extended generative harness passes deterministically (recorded seeds, prefix replay, order invariance, human/no-human equivalence) with no test-built final events.
2. Every adversarial parity variant fails the real runner; the paired hidden-world cases prove noninterference; the new compile-fail fixtures fail to compile.
3. No reachability-overstating `0048` lock remains; `cargo test -p tracewake-core --test generative_lock`, `cargo test -p tracewake-tui --test parity_adversarial`, and `cargo build --workspace --all-targets --locked` are green.

### Invariants

1. Regression evidence measures the production-path significant causal scenario, not a test-only substitute (`INV-091`/`INV-098`).
2. A hidden fact never alters actor-known output across paired worlds (`INV-094` noninterference).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/generative_lock.rs` — production-shaped mixed schedules, hidden-world pairs, order invariance, prefix replay.
2. `crates/tracewake-tui/tests/parity_adversarial.rs` / `playable_capability_parity.rs` — adversarial variants distinguishing the four evidence classes.
3. `tests/negative-fixtures/` + `negative_fixture_runner.rs` — actor-step / perception-export / debug→embodied compile-fail fixtures.

### Commands

1. `cargo test -p tracewake-core --test generative_lock --test negative_fixture_runner`
2. `cargo test -p tracewake-tui --test parity_adversarial --test playable_capability_parity`
3. `cargo test --workspace && cargo build --workspace --all-targets --locked`
