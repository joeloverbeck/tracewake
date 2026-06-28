# 0054FOUCONSIX-009: Standing mutation campaign — full run + denominator record

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only standing-campaign run; denominators and disposition handed to the capstone (011)
**Deps**: 0054FOUCONSIX-001, 0054FOUCONSIX-002, 0054FOUCONSIX-003, 0054FOUCONSIX-006, 0054FOUCONSIX-008

## Problem

The spec asserts no green/red command or mutation result — every executable claim is the implementing session's to produce (spec §0, §8). After all code/test work lands (sealed bootstrap, sealed wait receipt, non-inducible debug authority, PR-blocking mutation gate, public `food_source` witness), the configured standing mutation campaign must be run from a clean baseline and its full disposition recorded, so the capstone can compute a sound verdict. No artifact may call the canonical standing perimeter green while any survivor remains, and no `mutation_status` may compute pass with survivors present (spec §5, §4.4.3).

## Assumption Reassessment (2026-06-27)

1. `.cargo/mutants.toml` already includes the sealed surfaces' files (`projections.rs` `:17`, `epistemics/**` `:38`-`:40`); the resealed runtime/state/receipt/scheduler/command/debug files are within the standing perimeter or added by their owning tickets. The standing campaign is the configured one — this ticket runs it, it does not redefine the perimeter (the spec says the perimeter already covers the seams). Confirmed at `7660051`.
2. `crates/tracewake-core/tests/mutation_completion_merge.rs`, `.cargo/mutants-baseline-misses.txt`, and the CI mutation jobs exist; the in-diff PR gate is hardened in ticket 006. The full standing campaign runs **after** all code/test work (spec §7, §8). Confirmed.
3. Shared boundary under audit: the standing mutation perimeter as evidence. This is an evidence-only run+record ticket (modeled on the `0053FOUCONFIF-009` shape) — no `.cargo/mutants.toml` change here because the perimeter already covers the seams; its recorded denominators/disposition feed the capstone (011).
4. INV-098 (harsh acceptance) and INV-092 (deterministic replay is tested) — the campaign is part of the harsh-acceptance evidence; restated. The run must begin from a clean baseline and name its own exact implementation commit (not this proposal's baseline `7660051`).
5. Evidence-consumer surface: this ticket re-proves the fail-closed-validation / replay / actor-knowledge enforcement surfaces sealed in tickets 001–003/008 by mutating them; confirm the recorded evidence introduces no leakage/nondeterminism path and that the disposition (caught/missed/unviable/timeout + denominator + baseline-miss reconciliation) is complete. Run-discovered survivors that cannot be killed in-line route to a **bounded forcing function** per the §4.4 state machine (owning surface, why-not-closed, next move, max epochs/trigger, the exact failing CI/mutation test) and to the **reserved open range `0054FOUCONSIX-012+`** (§Ticket-creating deliverables); no survivor is carried forward by default.

## Architecture Check

1. Running the full standing campaign after all code/test work — rather than trusting focused per-ticket runs — is the only way to certify the canonical perimeter green at one exact commit, which the capstone's computed verdict requires. Recording the complete disposition (not just "no survivors") keeps the evidence auditable.
2. No backwards-compatibility aliasing/shims (evidence-only): no config or production change; the perimeter is the configured standing one.

## Verification Layers

1. Zero-survivor certification (INV-098) → the standing `cargo mutants` campaign over the configured perimeter completes with the full caught/missed/unviable/timeout disposition + denominator recorded.
2. Determinism (INV-092) → the four gate commands (`fmt`/`clippy`/`build --locked`/`test`) pass at the exact implementation commit before the campaign.
3. Forcing-function discipline → any residual survivor is recorded under a bounded forcing function and routed to the reserved open range, never as pass.

## What to Change

### 1. Run the four gate commands at the implementation commit

`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace` — all green at the named exact commit.

### 2. Run the standing mutation campaign from a clean baseline

Run the configured standing campaign; publish the selected denominator with the complete caught/missed/unviable/timeout disposition and baseline-miss reconciliation. Do not claim current zero survivors until the run is actually executed.

### 3. Record the disposition for the capstone

Hand the denominators/disposition to the capstone artifact (011). Route any run-discovered survivor to a bounded forcing function + the reserved open range `0054FOUCONSIX-012+`.

## Files to Touch

- `None — evidence-only standing-campaign run; denominators and disposition are recorded in the capstone acceptance artifact (ticket 011). Any run-discovered survivor needing a fix is authored as a new ticket in the reserved range 0054FOUCONSIX-012+.`

## Out of Scope

- Defining or widening the mutation perimeter (the configured perimeter already covers the seams; per-finding `.cargo/mutants.toml` adjustments, if any, land in their owning tickets 006/008).
- Fixing run-discovered survivors in this ticket (routed to the reserved open range).
- Rendering the acceptance verdict (ticket 011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — all green at the named exact implementation commit.
2. The standing mutation campaign completes with a recorded denominator + full caught/missed/unviable/timeout disposition + baseline-miss reconciliation; zero survivors, or every survivor recorded under a bounded forcing function and routed to `0054FOUCONSIX-012+`.

### Invariants

1. No artifact calls the canonical standing perimeter green while any survivor remains (spec §5).
2. The campaign begins from a clean baseline and names its own exact implementation commit, not `7660051`.

## Test Plan

### New/Modified Tests

1. `None — evidence-only standing-campaign run; verification is command-based and the disposition is recorded in the capstone artifact (ticket 011). Existing perimeter coverage is named in Assumption Reassessment.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
2. `cargo mutants` (the configured standing campaign per `.cargo/mutants.toml`) — record denominator + caught/missed/unviable/timeout + baseline-miss reconciliation.
3. The standing campaign is the correct verification boundary here (not a focused `-f` run): only a full-perimeter run at one commit can certify the canonical perimeter for the capstone.

## Outcome

Completed: 2026-06-27

Ran the pre-mutation gate set and the configured standing mutation campaign from a clean detached worktree at exact commit `30678b6e420db98b32cd8edfa8d112f3aad9a07c` (`/tmp/tracewake-mutants-30678b6`). The main checkout retained unrelated pre-existing dirty files; the detached worktree was clean before the evidence run.

Pre-mutation clean-worktree gates:

- `git status --short` — clean.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.

Configured standing mutation campaign:

- `cargo mutants --list | wc -l` — selected 3445 mutants.
- `cargo mutants` — completed in about 4h with 3445 mutants tested: 2673 caught, 766 unviable, 6 missed, 0 timeouts. Command exited non-zero because survivors were present.
- `.cargo/mutants-baseline-misses.txt` is empty, so there are no preapproved baseline-miss rows to reconcile. The six missed rows are live survivors, not green standing evidence.

Missed rows:

- `crates/tracewake-core/src/runtime/receipt.rs:151:45: replace > with >= in OneTickRuntimeReceipt::from_world_advance_result`
- `crates/tracewake-core/src/runtime/receipt.rs:160:9: replace OneTickRuntimeReceipt::advanced -> bool with true`
- `crates/tracewake-core/src/runtime/receipt.rs:164:9: replace OneTickRuntimeReceipt::appended_event_count -> usize with 1`
- `crates/tracewake-core/src/runtime/receipt.rs:168:9: replace OneTickRuntimeReceipt::actor_known_interval_summary -> Option<&TypedActorKnownIntervalSummary> with None`
- `crates/tracewake-tui/src/app.rs:158:9: replace TuiApp::debug_available_for -> bool with true`
- `crates/tracewake-tui/src/app.rs:164:13: replace && with || in TuiApp::debug_available`

Bounded forcing route:

- Created successor ticket `0054FOUCONSIX-012`, now archived at
  `archive/tickets/0054FOUCONSIX-012.md`, to close the runtime receipt and TUI
  debug-availability survivor set with focused public-boundary tests and
  mutation reconciliation.
- Updated capstone ticket `0054FOUCONSIX-011`, now archived at
  `archive/tickets/0054FOUCONSIX-011.md`, so the capstone depends on 012. This
  prevents the acceptance artifact from rendering pass over known live mutation
  survivors.

No production code or mutation configuration was changed in this evidence-only ticket.

Unrelated pre-existing dirty paths left untouched in the main checkout: `.claude/skills/spec-to-tickets/SKILL.md`, `.claude/skills/spec-to-tickets/references/decomposition-patterns.md`, `CLAUDE.md`, and `tools/clean-build-scratch.sh`.
