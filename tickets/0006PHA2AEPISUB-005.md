# 0006PHA2AEPISUB-005: Extend source guards and clippy float/hash ban to epistemic paths

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `hidden_truth_gates` source guard extended to epistemic modules; `clippy.toml` extended to ban raw float confidence in epistemic paths.
**Deps**: 0006PHA2AEPISUB-003

## Problem

The lock layer does not yet guard the epistemic substrate against regression. `crates/tracewake-core/tests/hidden_truth_gates.rs` hardens only `agent/actor_known.rs` and `debug_capability.rs` source strings (`:26-27`, `:117-153`); it does not scan `epistemics/knowledge_context.rs` or `epistemics/projection.rs`, so a future re-introduction of a `pub` authority field or public debug constructor would pass. `clippy.toml` bans nondeterministic maps/time/thread/process/fs/network APIs (`:1-16`) but does not ban `f32`/`f64`, so raw float confidence could re-enter the epistemic paths and break determinism. This ticket extends both existing guards (spec §6 EPI-HARD-004 req. 1–2).

## Assumption Reassessment (2026-06-09)

1. `hidden_truth_gates.rs` currently scans `ACTOR_KNOWN_RS` and `DEBUG_CAPABILITY_RS` only (`:26-27`) and asserts crate-privacy of those surfaces (`:117-153`); it does NOT reference `knowledge_context.rs` or `projection.rs`. `clippy.toml:1-16` disallows `HashMap`/`HashSet`/`SystemTime`/`Instant`/thread/process/fs/network; it has no `f32`/`f64` entry. Both confirmed this session.
2. Spec authority: `specs/0006_…SPEC.md` §6 EPI-HARD-004 req. 1–2 (guard paths + banned tokens). The sealed surfaces these guards assert-against must already exist — hence `Deps: 0006PHA2AEPISUB-003` (transitive head over -002/-001).
3. Shared boundary under audit: the source-guard contract (which files are scanned for which banned `pub`/token patterns) and the `clippy.toml` disallowed-list contract. Both are repo-wide lock-layer surfaces already wired into CI (`.github/workflows/ci.yml` runs `hidden_truth_gates`).
4. Constitutional invariants motivating this ticket: `INV-018` (deterministic replay — float confidence is the determinism risk the clippy ban closes), `INV-024`/`INV-068` (the source guard prevents re-exposing context/projection authority surfaces), `INV-026` (no public record-field reintroduction).
5. Enforcement surface: this is the lock layer for the actor-knowledge-filtering and deterministic-replay surfaces. It adds no production logic; it strengthens regression prevention. The clippy ban must be scoped so it bans `f32`/`f64` in epistemic confidence paths without breaking any legitimate float use elsewhere — confirm scoping during implementation (the spec confines it to epistemic confidence paths).

## Architecture Check

1. Extending the existing `hidden_truth_gates` source guard and `clippy.toml` (rather than creating a parallel scanner) keeps one lock layer, reuses CI wiring, and matches the spec's explicit "extend, do not invent a parallel one" directive. A source-string guard is the right surface for "no `pub` authority field," and the clippy disallowed-list is the right surface for "no raw float."
2. No backwards-compatibility shims: guards are additive assertions over already-sealed code.

## Verification Layers

1. `INV-024`/`INV-068` (no re-exposed context/projection authority) -> codebase grep-proof in `hidden_truth_gates`: scan `epistemics/knowledge_context.rs` and `epistemics/projection.rs` for `pub viewer_actor_id`, `pub mode`, `pub .*_scope`, `pub observations_by_id`, `pub beliefs_by_id`, `pub beliefs_by_holder`, `pub contradictions_by_id`, `pub notebook_entries_by_actor`, and a public `fn debug(` on `KnowledgeContext` unless capability-mediated.
2. `INV-018` (determinism) -> clippy disallowed-list: `f32`/`f64`/`parse::<f32>`/`parse::<f64>` banned in epistemic confidence paths; `HashMap`/`HashSet` already globally banned.
3. Guard self-coverage -> the source guard must fail if a banned pattern is present (paired with the compile-fail fixtures in 0006PHA2AEPISUB-004; a guard with no failing case is incomplete).

## What to Change

### 1. Extend the source guard

In `crates/tracewake-core/tests/hidden_truth_gates.rs`, add `epistemics/knowledge_context.rs`, `epistemics/projection.rs`, and the epistemic record modules to the scanned set, and assert none contain the banned `pub` authority-field patterns or a non-capability-gated `KnowledgeContext::debug`.

### 2. Ban raw float confidence

In `clippy.toml`, add `f32`/`f64` (and float parse/format) to the disallowed surface for epistemic confidence paths, scoped so legitimate non-epistemic float use is unaffected.

### 3. Confirm CI coverage (verify-only)

`.github/workflows/ci.yml` already runs `hidden_truth_gates` and the clippy gate; confirm no CI yaml change is needed (expected: none — the suites are already wired).

## Files to Touch

- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `clippy.toml` (modify)

## Out of Scope

- The sealing itself (0006PHA2AEPISUB-001/-002/-003).
- Compile-fail negative fixtures (0006PHA2AEPISUB-004).
- Runtime adversarial/replay/event-schema tests (0006PHA2AEPISUB-006).
- Any `ci.yml` change (suites already wired; verify-only).

## Acceptance Criteria

### Tests That Must Pass

1. `hidden_truth_gates` fails if a `pub` authority-bearing field or non-capability `KnowledgeContext::debug` is present in the scanned epistemic modules (verified by a temporary re-exposure spot-check during review).
2. `cargo clippy --workspace --all-targets -- -D warnings` flags a deliberately-introduced `f32`/`f64` confidence value in an epistemic path (spot-check), and passes on the real tree.
3. `cargo test -p tracewake-core --test hidden_truth_gates` passes.

### Invariants

1. The source guard scans every sealed epistemic authority surface; the clippy ban covers `f32`/`f64` in epistemic confidence paths.
2. No parallel lock layer is introduced; CI runs the extended guards via the existing job.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — extended scanned-file set + banned-pattern assertions.
2. `None — clippy.toml is configuration; its effect is verified via the clippy gate command below.`

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace` — full-pipeline confirmation the extended guards do not false-positive on existing code.
