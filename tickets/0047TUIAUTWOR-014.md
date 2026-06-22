# 0047TUIAUTWOR-014: Human sleep completion real-pipeline witness

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — test-only: real-pipeline human sleep-completion witness through the actual parser/app/core path
**Deps**: 0047TUIAUTWOR-013

## Problem

Spec 0047 §6/§4.9-item-11 requires a real-pipeline human sleep-completion witness exercising the full chain: start → reject ordinary wait → continue → terminal → recovery/hunger accounting → actor-known summary → replay/rebuild. All evidence must be real-pipeline (no synthetic event insertion); `RunNoHumanDay` is not a valid witness for the human path. This is the integration proof that the per-component work (0047TUIAUTWOR-005…013) composes into the originally-broken behavior — a human-started sleep now reaches its terminal and fatigue is recovered.

## Assumption Reassessment (2026-06-22)

1. The full human path now exists end-to-end: the TUI world-step entry + `WaitOneTick` routing (0047TUIAUTWOR-011), the generalized reservation predicate rejecting ordinary `wait` mid-sleep (0047TUIAUTWOR-009), the typed continuation control (0047TUIAUTWOR-012), the coordinator's duration discovery + completion through `build_sleep_completion_events` (0047TUIAUTWOR-006/007), the unified accounting (0047TUIAUTWOR-008), and the actor-known interval summary (0047TUIAUTWOR-013). This ticket only adds the witness; it implements no new production logic.
2. The witness must run through the actual parser/app/core path (the `tracewake-tui` real-pipeline test surface, e.g. `embodied_flow.rs` / a new witness test), not by inserting synthetic events. `RunNoHumanDay` (`DebugCommand::RunNoHumanDay`) is explicitly not an acceptable witness for the human path (spec §3.1/§6).
3. Cross-artifact boundary under audit: this is an integration witness spanning TUI input → core coordinator → projection → replay. It asserts the composed behavior, so its assumptions are the union of the upstream tickets' contracts; it owns no production surface.
4. Constitutional invariant motivating the ticket: `INV-045` (ordinary survival is causal — fatigue recovers by sleeping) and `INV-098` (feature acceptance is harsh — caused, agent-possible, eventful, epistemically bounded, TUI-playable, replay-safe, regression-tested). This witness is part of meeting `INV-098` for the sleep path.
5. Enforcement surface audited (evidence-consumer basis): the witness reads the replay/rebuild, the need-accounting ledger, and the actor-known summary — it must confirm the recovery/hunger accounting is correct, the summary is actor-known (no hidden leak), and the replay reconstructs the frontier, duration open/closed state, need values, sleep recovery, the interval summary, and the stop reason. The witness introduces no synthetic event and no leakage path; it is the real-pipeline regression lock for the sleep path.

## Architecture Check

1. A single real-pipeline witness exercising the whole sleep chain — rather than asserting each component in isolation again — is the integration proof that the components compose; the isolated unit tests (0047TUIAUTWOR-006/007/008) prove the pieces, this proves the wiring. Using the real parser/app/core path (not synthetic insertion) is mandatory because the original defect was precisely a *wiring* gap no unit test caught.
2. No backwards-compatibility aliasing/shims: the witness uses the real command path; it does not reach into core to synthesize a terminal or bypass the pipeline.

## Verification Layers

1. `INV-045` causal recovery -> replay/golden-fixture check: after start → continue → terminal, the actor's fatigue is recovered and hunger accounting is correct over the slept ticks.
2. `INV-098` harsh acceptance -> replay/golden-fixture check: the scenario replays/rebuilds to identical frontier, duration state, need ledger, recovery, interval summary, and stop reason.
3. No-leak -> manual review: the actor-known sleep summary at resume carries only holder-known/source-bearing information.

## What to Change

### 1. Human sleep-completion real-pipeline witness (new TUI integration test)

Add a witness that, through the actual parser/app/core path: starts a human sleep; attempts an ordinary `wait` and asserts `ReservationConflict`; issues the typed continuation; advances to the sleep terminal; asserts recovery + hunger accounting; asserts the actor-known interval summary is correct and leak-free; and replays/rebuilds to identical state. Assert no `RunNoHumanDay` is used.

## Files to Touch

- `crates/tracewake-tui/tests/embodied_flow.rs` (modify — add the sleep-completion witness; or a new `tests/human_duration_witness.rs` if the suite is clearer split — decide at implementation, keeping it a real-pipeline TUI test)

## Out of Scope

- Any production logic (0047TUIAUTWOR-005…013 own it).
- The work-completion witness (0047TUIAUTWOR-015) and the differential suite (0047TUIAUTWOR-016).
- Parity registry entries (0047TUIAUTWOR-017).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui` — the human sleep-completion witness passes through the real parser/app/core path: start → reject ordinary wait (`ReservationConflict`) → continue → terminal → fatigue recovery + hunger accounting → actor-known summary → replay/rebuild identical.
2. The witness asserts no synthetic event insertion and no `RunNoHumanDay` invocation on the human path.
3. `cargo test --workspace` passes; `cargo clippy -p tracewake-tui --all-targets -- -D warnings` clean.

### Invariants

1. Human-started sleep reaches its terminal and recovers fatigue via the real pipeline (`INV-045`); the witness is real-pipeline only (`RunNoHumanDay` excluded).
2. The actor-known summary in the witness contains no hidden-truth leak (`INV-024`/`INV-067`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_flow.rs` (modify) — human sleep-completion real-pipeline witness (full chain + replay/rebuild).

### Commands

1. `cargo test -p tracewake-tui sleep_completion`
2. `cargo test --workspace`
3. The full-workspace boundary is appropriate for an integration witness spanning TUI → core → replay; the narrower `cargo test -p tracewake-tui sleep_completion` is the focused per-witness run.
