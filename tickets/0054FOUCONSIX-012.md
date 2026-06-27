# 0054FOUCONSIX-012: Standing mutation survivor closure — runtime receipt and debug availability

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — focused tests for runtime one-tick receipt semantics and TUI debug-availability authority
**Deps**: 0054FOUCONSIX-009

## Problem

The ticket 009 standing mutation campaign at commit `30678b6e420db98b32cd8edfa8d112f3aad9a07c` completed the configured perimeter but found six live survivors. Four survivors are in `crates/tracewake-core/src/runtime/receipt.rs` and show that tests do not force all public `OneTickRuntimeReceipt` accessors or the zero/positive appended-event boundary. Two survivors are in `crates/tracewake-tui/src/app.rs` and show that TUI tests do not force the exact debug-availability predicate strongly enough.

No acceptance artifact may call the standing perimeter green while these rows remain missed. This ticket is the bounded forcing function for the survivor set routed by 0054FOUCONSIX-009.

## Assumption Reassessment (2026-06-27)

1. `cargo mutants` in `/tmp/tracewake-mutants-30678b6` selected 3445 configured standing mutants and finished with 2673 caught, 766 unviable, 6 missed, and 0 timeouts at commit `30678b6e420db98b32cd8edfa8d112f3aad9a07c`.
2. The missed rows are:
   - `crates/tracewake-core/src/runtime/receipt.rs:151:45: replace > with >= in OneTickRuntimeReceipt::from_world_advance_result`
   - `crates/tracewake-core/src/runtime/receipt.rs:160:9: replace OneTickRuntimeReceipt::advanced -> bool with true`
   - `crates/tracewake-core/src/runtime/receipt.rs:164:9: replace OneTickRuntimeReceipt::appended_event_count -> usize with 1`
   - `crates/tracewake-core/src/runtime/receipt.rs:168:9: replace OneTickRuntimeReceipt::actor_known_interval_summary -> Option<&TypedActorKnownIntervalSummary> with None`
   - `crates/tracewake-tui/src/app.rs:158:9: replace TuiApp::debug_available_for -> bool with true`
   - `crates/tracewake-tui/src/app.rs:164:13: replace && with || in TuiApp::debug_available`
3. Shared boundary under audit: public runtime receipt evidence consumed by normal TUI/world-advance callers, and TUI debug authority gating. These surfaces feed the F6 acceptance capstone, so they must be closed before any computed pass.
4. INV-098 (harsh acceptance), INV-067 (TUI-first playability), and the no-debug-leak evidence contract motivate this ticket. A runtime receipt must report actor-legible one-tick effects accurately, and debug availability must require the real operator authority predicate.
5. Enforcement surface: `OneTickRuntimeReceipt` and `TuiApp` debug-availability accessors. The fix must add tests that fail for the exact missed mutants without widening public authority or leaking debug-only data to embodied views.

## Architecture Check

1. Focused tests on the public receipt/debug APIs are the correct closure seam: they prove the standing survivors matter at the public boundary instead of weakening production code or mutating the standing perimeter.
2. No backwards-compatibility aliasing/shims: the closure must tighten executable proof around the existing APIs, not add alternate accessors or skip rows.

## Verification Layers

1. Runtime receipt survivor closure -> focused tests that force zero-event receipts to remain `advanced == false`, force positive appended-event counts, and force actor-known interval summaries to remain visible when present.
2. Debug availability survivor closure -> focused TUI tests that prove debug availability is false without operator authority and true only when the existing operator-gated predicate is satisfied.
3. Standing perimeter reconciliation -> focused `cargo mutants` re-run for the six survivor regexes, plus the full gate set.

## What to Change

### 1. Runtime receipt tests

Add or extend runtime/core tests so the four `OneTickRuntimeReceipt` survivor mutations fail. Cover the zero-appended-event boundary, the `advanced()` accessor, the `appended_event_count()` accessor, and the optional actor-known interval summary accessor.

### 2. TUI debug availability tests

Add or extend TUI tests so `debug_available_for` cannot always return true and `debug_available` cannot use `||` in place of the required conjunction.

### 3. Mutation reconciliation

Rerun focused mutation proof for the six named survivors. If the focused proof is green, rerun the configured standing campaign or create a successor standing-run ticket if the full campaign cannot be completed in the same session.

## Files to Touch

- `crates/tracewake-core/src/runtime/receipt.rs` (modify only if public receipt semantics need code repair)
- `crates/tracewake-core/tests/` or core unit tests (modify)
- `crates/tracewake-tui/src/app.rs` (modify only if the predicate itself is wrong)
- `crates/tracewake-tui/tests/` or TUI unit tests (modify)

## Out of Scope

- Reclassifying the six rows as equivalent without executable proof.
- Editing `.cargo/mutants.toml` to remove the survivor rows from the standing perimeter.
- Rendering the capstone verdict before these survivors are closed.

## Acceptance Criteria

### Tests That Must Pass

1. Focused tests fail against the six listed survivor mutations and pass on the repaired tree.
2. `cargo mutants -F 'OneTickRuntimeReceipt::from_world_advance_result|OneTickRuntimeReceipt::advanced|OneTickRuntimeReceipt::appended_event_count|OneTickRuntimeReceipt::actor_known_interval_summary|TuiApp::debug_available_for|TuiApp::debug_available'` records 0 missed and 0 timeouts for the survivor set.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. The standing perimeter cannot be called green while any of the six survivor rows remain missed.
2. Runtime receipt and debug-availability tests prove public-boundary behavior, not private implementation details.

## Test Plan

### New/Modified Tests

1. Core runtime receipt test(s) — force the public receipt accessors and zero/positive event-count boundary.
2. TUI debug availability test(s) — force operator-gated debug availability and the conjunction used by `debug_available`.

### Commands

1. `cargo test -p tracewake-core <focused receipt test selector> && cargo test -p tracewake-tui <focused debug availability selector>`
2. `cargo mutants -F 'OneTickRuntimeReceipt::from_world_advance_result|OneTickRuntimeReceipt::advanced|OneTickRuntimeReceipt::appended_event_count|OneTickRuntimeReceipt::actor_known_interval_summary|TuiApp::debug_available_for|TuiApp::debug_available'`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
