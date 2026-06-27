# 0054FOUCONSIX-013: Standing mutation rerun after survivor closure

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only standing-campaign rerun after 012 survivor closure
**Deps**: 0054FOUCONSIX-012

## Problem

Ticket 009 ran the configured standing mutation campaign and found six live survivors. Ticket 012 adds focused tests that close those survivor rows, but the acceptance capstone still needs a current configured standing campaign at an exact post-012 implementation commit. A focused survivor run is necessary but not sufficient to certify the canonical perimeter green.

## Assumption Reassessment (2026-06-27)

1. Ticket 012 focused proof selected 16 mutants matching the survivor regex and completed with 14 caught, 2 unviable, 0 missed, and 0 timeouts after adding runtime receipt and TUI debug-availability tests.
2. The configured standing denominator from ticket 009 was 3445 mutants under `.cargo/mutants.toml`, with `.cargo/mutants-baseline-misses.txt` empty. The same configured perimeter must be rerun after 012 lands.
3. Shared boundary under audit: the standing mutation perimeter as capstone evidence after survivor closure. This is evidence-only and must not redefine the perimeter.
4. INV-098 (harsh acceptance) requires current, exact-commit, mutation-green evidence before any capstone can render pass.
5. Evidence-consumer surface: the capstone reads this rerun as the current mutation disposition. Any missed/timeout row must be routed to another bounded forcing ticket and must compute non-pass until closed.

## Architecture Check

1. A clean post-012 standing rerun is the only sound way to convert the focused survivor closure into canonical perimeter evidence for the capstone.
2. No backwards-compatibility aliasing/shims: this ticket changes no production code or mutation configuration.

## Verification Layers

1. Current standing denominator -> `cargo mutants --list | wc -l` at the post-012 commit.
2. Current standing disposition -> `cargo mutants` complete with caught/missed/unviable/timeout counts.
3. Clean baseline -> `fmt`/`clippy`/`build --locked`/`test` pass in a clean worktree at the exact post-012 commit.

## What to Change

### 1. Run pre-mutation gates at the post-012 commit

Use a clean worktree or otherwise prove `git status --short` is clean before running gates.

### 2. Run the configured standing campaign

Run `cargo mutants` with `.cargo/mutants.toml`, record denominator and complete disposition.

### 3. Route any new survivor

If any missed or timeout row remains, create a bounded successor ticket and prevent the capstone from rendering pass.

## Files to Touch

- `None — evidence-only standing-campaign rerun.`

## Out of Scope

- Focused survivor fixes (ticket 012).
- Changing `.cargo/mutants.toml` or marking survivors equivalent without proof.
- Rendering the acceptance capstone.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.
2. `cargo mutants --list | wc -l` records the configured denominator.
3. `cargo mutants` completes with full caught/missed/unviable/timeout disposition; pass evidence requires 0 missed and 0 timeouts.

### Invariants

1. No capstone pass over missed or timed-out standing mutants.
2. The run names its exact post-012 implementation commit.

## Test Plan

### New/Modified Tests

1. `None — evidence-only standing-campaign rerun after 012 test additions.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
2. `cargo mutants --list | wc -l`
3. `cargo mutants`

## Outcome

Completed: 2026-06-27

Ran the post-012 configured standing mutation campaign in clean detached worktree `/tmp/tracewake-mutants-6d7009f` at exact commit `6d7009f61e3f7d55f81da3be3297160c6f2fb402`.

Verification:

- `git status --short` was clean in the detached worktree.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.
- `cargo mutants --list | wc -l` reported `3445`.
- `cargo mutants` completed `3445` mutants in about 4h: `2679` caught, `766` unviable, `0` missed, `0` timeout.

No production code, tests, docs, or mutation configuration changed for this evidence-only rerun. No successor survivor ticket was created because the standing campaign completed with zero missed and zero timed-out mutants.
