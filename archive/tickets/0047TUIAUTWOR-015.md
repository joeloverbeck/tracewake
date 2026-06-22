# 0047TUIAUTWOR-015: Human work completion real-pipeline witness

**Status**: DONE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — test-only: real-pipeline human work-completion witness through the actual parser/app/core path
**Deps**: 0047TUIAUTWOR-013

## Problem

Spec 0047 §6/§4.9-item-12 requires a real-pipeline human work-completion witness (output or modeled failure; interruption case if supported), through the actual parser/app/core path — `RunNoHumanDay` is not a valid witness. This is the integration proof that a human-started work block reaches its terminal with correct output (or modeled failure) and accounting, completing the duration-completion witness pair alongside the sleep witness (0047TUIAUTWOR-014).

## Assumption Reassessment (2026-06-22)

1. Work duration completion exists via `build_work_completion_events` (`actions/defs/work.rs:119`); a work block starts via `build_work_start_events` (`work.rs:18`). There is **no** `build_work_interruption_events` — work interruption is not modeled (confirmed during decomposition spot-grep). This resolves §4.9-item-12's conditional "interruption case if supported": work is witnessed for completion-with-output and modeled failure only; an interruption sub-case is **not in scope** because the builder does not exist (recorded, not silently dropped).
2. The witness runs through the real parser/app/core path (`tracewake-tui` integration surface), exercising the same composed chain as the sleep witness (0047TUIAUTWOR-014): TUI world-step entry (0047TUIAUTWOR-011), reservation rejection of ordinary `wait` mid-work (0047TUIAUTWOR-009), continuation (0047TUIAUTWOR-012), coordinator completion (0047TUIAUTWOR-007), accounting (0047TUIAUTWOR-008), interval summary (0047TUIAUTWOR-013). No new production logic.
3. Cross-artifact boundary under audit: an integration witness spanning TUI input → core coordinator (work completion) → projection → replay; it asserts composed behavior and owns no production surface. Its only divergence from the sleep witness is the duration kind (work) and the output-vs-failure terminal.
4. Constitutional invariant motivating the ticket: `INV-045` (work completes by elapsed causal ticks; money/work output is real) and `INV-098` (harsh feature acceptance, applied to the work path).
5. Enforcement surface audited (evidence-consumer basis): the witness reads the replay/rebuild, the work output (or modeled failure) accounting, and the actor-known summary. It confirms the work terminal's output/failure accounting is correct, the summary is actor-known (no leak of hidden world activity during the work block), and replay reconstructs frontier, duration state, need ledger, work output, interval summary, and stop reason. No synthetic event; the real-pipeline regression lock for the work path.

## Architecture Check

1. Reusing the sleep witness's composed-chain shape for work — varying only the duration kind and the output/failure terminal — keeps the duration-completion witnesses parallel and proves the coordinator's duration handling is general across kinds, not sleep-special-cased. Recording the absent work-interruption builder (rather than asserting a non-existent case) keeps the witness honest about scope.
2. No backwards-compatibility aliasing/shims: real command path only; no synthetic terminal, no work-interruption stub invented to satisfy the conditional.

## Verification Layers

1. `INV-045` causal work output -> replay/golden-fixture check: a human work block reaches `WorkBlockCompleted` with correct output (or `WorkBlockFailed` with modeled failure) and accounting over the worked ticks.
2. `INV-098` harsh acceptance -> replay/golden-fixture check: the scenario replays/rebuilds to identical frontier, duration state, need ledger, work output, interval summary, and stop reason.
3. No-leak -> manual review: the actor-known work summary at completion carries only holder-known/source-bearing information.

## What to Change

### 1. Human work-completion real-pipeline witness (new TUI integration test)

Add a witness that, through the actual parser/app/core path: starts a human work block; attempts an ordinary `wait` and asserts `ReservationConflict`; issues the typed continuation; advances to the work terminal; asserts output (or modeled failure) + accounting; asserts the actor-known interval summary is correct and leak-free; and replays/rebuilds to identical state. Record explicitly that no interruption sub-case is exercised (no work-interruption builder exists).

## Files to Touch

- `crates/tracewake-tui/tests/embodied_flow.rs` (modify — add the work-completion witness; or the shared `tests/human_duration_witness.rs` introduced by 0047TUIAUTWOR-014 if that split was taken)

## Out of Scope

- Any production logic (0047TUIAUTWOR-005…013).
- Adding a work-interruption builder/case (not modeled; recorded as out of scope).
- The differential suite (0047TUIAUTWOR-016) and parity registry (0047TUIAUTWOR-017).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui` — the human work-completion witness passes through the real parser/app/core path: start → reject ordinary wait (`ReservationConflict`) → continue → terminal (`WorkBlockCompleted` output, or `WorkBlockFailed` modeled failure) → accounting → actor-known summary → replay/rebuild identical.
2. The witness records that no interruption sub-case is exercised (work-interruption is not modeled) and uses no `RunNoHumanDay` / synthetic event.
3. `cargo test --workspace` passes; `cargo clippy -p tracewake-tui --all-targets -- -D warnings` clean.

### Invariants

1. Human-started work reaches its terminal with correct output/failure accounting via the real pipeline (`INV-045`); witness is real-pipeline only.
2. The actor-known work summary contains no hidden-truth leak (`INV-024`/`INV-067`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_flow.rs` (modify) — human work-completion real-pipeline witness (output + modeled-failure variants; replay/rebuild).

### Commands

1. `cargo test -p tracewake-tui work_completion`
2. `cargo test --workspace`
3. The full-workspace boundary is appropriate for an integration witness spanning TUI → core → replay; `cargo test -p tracewake-tui work_completion` is the focused per-witness run.

## Outcome

Completed: 2026-06-22

Evidence:
- Added `human_work_completion_real_pipeline_witness` in `crates/tracewake-tui/tests/embodied_flow.rs`.
- The witness drives `do move.to.workshop_tomas`, `do work.block.workplace_tomas`, rejected ordinary `wait`, `continue`, `bind-debug actor_tomas`, and `debug replay` through `run_command_loop`; it asserts reservation conflict, possessed duration terminal, actor-known work completion summary, work need accounting, replay checksum parity, and absence of no-human-day witness text.
- No work-interruption sub-case was exercised because work interruption is not modeled by a production builder.
- Passed `cargo fmt --all --check`.
- Passed `cargo test -p tracewake-tui human_work_completion_real_pipeline_witness`.
- Passed `cargo test -p tracewake-tui work_completion`.
- Passed `cargo clippy -p tracewake-tui --all-targets -- -D warnings`.
- Passed `cargo test --workspace`.
