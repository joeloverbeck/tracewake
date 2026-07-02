# 0065TUIFULCRO-004: Fullscreen shell acceptance artifact

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None
**Deps**: 0065TUIFULCRO-001, 0065TUIFULCRO-002, 0065TUIFULCRO-003

## Problem

Spec 0065 §7 requires a review artifact recording key→intent equivalence, the restore-on-exit/panic guarantee, and the no-semantic-fork regression, captured at the exact implementation commit, with headless tests as the authoritative acceptance lane (§4.4). This capstone assembles that evidence; it introduces no production logic.

## Assumption Reassessment (2026-07-02)

1. The three witnesses land in prior tickets: restore-on-exit/panic in 001 (`shell::terminal` tests), key→intent equivalence in 002 (`shell::key_map` test vs `parse_key_script`), no-semantic-fork regression in 003 (shell-path-vs-line/script-path parity). This ticket runs them plus the workspace gates and records results — no production or test logic is added here. Report-path convention is `reports/NNNN_<slug>_acceptance.md` (siblings `archive/reports/0064_tui_embodied_pane_layout_acceptance.md`, `archive/reports/0062_tui_deterministic_intent_acceptance.md`).
2. Spec 0065 §6 (required fixtures and tests), §7 (acceptance artifact and evidence), §4.4 (no TTY required for CI; headless tests authoritative).
3. Shared boundary under audit: the acceptance artifact `reports/0065_tui_fullscreen_crossterm_shell_acceptance.md` and the three witness surfaces it consolidates (restore guard, key→intent mapping, shell-vs-script parity); no code contract is modified.
4. INV-108 / INV-018 / INV-065: the acceptance lane proves cognition-neutral input (same intents as scripts), replay-safety (the shell holds no world state), and a reachable, robust primary interface.
5. Evidence-consumer basis: this ticket audits the actor-knowledge (`current_view`), no-fork, and restore surfaces rather than modifying them — confirm the recorded evidence introduces no leakage and that the authoritative lane is headless (no acceptance step depends on a TTY).

## Architecture Check

1. A single acceptance artifact plus an evidence run (no new logic) mirrors the sibling 0064 / 0062 closeout shape and yields one reviewable record at the implementation commit.
2. No backwards-compatibility shims: this ticket adds no code.

## Verification Layers

1. INV-108 (cognition-neutral input) -> cite the 002 key→intent equivalence test result.
2. Restore guarantee -> cite the 001 restore + panic-restore test results.
3. No-semantic-fork -> cite the 003 regression result.
4. Aggregate gates -> `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## What to Change

### 1. Author the acceptance artifact

`reports/0065_tui_fullscreen_crossterm_shell_acceptance.md`: record the three witnesses (named tests + results), the four workspace gates, the exact implementation commit, and the headless-authoritative note (§4.4). Follow the sibling artifact structure.

## Files to Touch

- `reports/0065_tui_fullscreen_crossterm_shell_acceptance.md` (new)

## Out of Scope

- Any production or test code change (evidence-only; the witnesses live in 001–003).
- The `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` move (deferred to spec acceptance per `docs/archival-workflow.md`).

## Acceptance Criteria

### Tests That Must Pass

1. All named witness tests pass at the recorded commit: 001 restore/panic, 002 key→intent equivalence, 003 no-fork regression.
2. `cargo test --workspace` is green; `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo build --workspace --all-targets --locked` are clean.

### Invariants

1. Headless tests are the authoritative acceptance lane; no acceptance step requires a TTY.
2. The artifact records only observer-safe evidence — no hidden-truth leakage.

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification is command-based and the witnesses are named in Assumption Reassessment.`

### Commands

1. `cargo test --workspace`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked`

## Outcome

Completed: 2026-07-02

Added `reports/0065_tui_fullscreen_crossterm_shell_acceptance.md`, recording
the terminal restore, key-intent equivalence, redraw, launch, and no-fork
shell-path witnesses for Spec 0065. The report names implementation commits
`43e06fa`, `cd6f162`, and `1df5719`, and records the evidence/report commit
role separately because the report cannot self-reference its final commit hash.

Deviation from plan: the first `cargo test --workspace` run exposed that the
workspace source/dependency guard had not been truthed for the new shell files
and `crossterm`. This ticket therefore updates
`crates/tracewake-core/tests/anti_regression_guards.rs` to classify
`crates/tracewake-tui/src/shell/*.rs` under the existing TUI presentation-layer
rationale and to allow `crossterm` as a TUI dependency. That is test/guard
truthing only; no production logic is added by this ticket.

Verification:

1. `cargo test -p tracewake-core --test anti_regression_guards workspace_` —
   passed after guard truthing; 3 workspace source/dependency guard tests
   passed.
2. `cargo fmt --all --check` — passed after the report and ticket outcome were
   added.
3. `cargo clippy --workspace --all-targets -- -D warnings` — passed after the
   report and ticket outcome were added.
4. `cargo build --workspace --all-targets --locked` — passed after the report
   and ticket outcome were added.
5. `cargo test --workspace` — passed after the report and ticket outcome were
   added.
