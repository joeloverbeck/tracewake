# 0054FOUCONSIX-006: PR-blocking mutation proof for guarded changes

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `.github/workflows/ci.yml` (required PR contexts include actual in-diff mutation proof); `crates/tracewake-core/tests/ci_workflow_guards.rs` (required-context topology guard); `crates/tracewake-core/tests/support/acceptance_status_manifest.rs` (mutation-evidence parser); `.cargo/mutants.toml` / `.cargo/mutants-baseline-misses.txt` / `crates/tracewake-core/tests/mutation_completion_merge.rs` as needed
**Deps**: 0054FOUCONSIX-005

## Problem

Mutation proof is partly declarative rather than PR-enforced for guarded changes. `.github/workflows/ci.yml` defines a `full-surface mutation trigger (lock layer)` topology/signaling job (`:263`) and a `mutation in-diff (lock layer)` (`mutants-in-diff`) job (`:285`-`:286`). The 0053 transcript's seven required contexts (`ci.yml:174`-`:182`) include the trigger and shard reconciliation but **not** `mutation in-diff`. A topology trigger going green is an alarm, not proof a full mutation perimeter is green; in-diff mutation is closer to an enforceable PR check but is not one of the required contexts for guarded changes (finding F6-06). A skipped/scheduled/informational trigger must not count as a PR pass for a code-changing closure artifact.

## Assumption Reassessment (2026-06-27)

1. `.github/workflows/ci.yml:285`-`:286` defines the `mutants-in-diff` job named `mutation in-diff (lock layer)`; `:263` defines `full-surface mutation trigger (lock layer)`; `:174`-`:182` lists the seven `required_checks` contexts (rustfmt, clippy, build & test, lock-layer gates, public-boundary conformance, full-surface mutation trigger, mutation shard reconciliation) — `mutation in-diff` is absent. Confirmed at `7660051`.
2. `crates/tracewake-core/tests/ci_workflow_guards.rs` pins the mutation topology (scheduled-perimeter / reconciliation guards `:179`-`:220`); `crates/tracewake-core/tests/mutation_completion_merge.rs`, `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt` exist. `acceptance_status_manifest.rs` carries `mutation_status`/`mutation_survivors` scalars; ticket 004 introduces the mutation-evidence hook this ticket fills. Confirmed.
3. Shared boundary under audit: the standing-gate enforcement layer — the required-context ruleset (`ci.yml`), its topology pin (`ci_workflow_guards.rs`), the manifest mutation-evidence parser (`acceptance_status_manifest.rs`), and the merge/reconciliation tooling. Sequential-edit chain with tickets 004 (foundation) and 005 (governance) on `ci.yml` + `ci_workflow_guards.rs` + the manifest support file.
4. INV-098 (harsh acceptance) and the evidence-honesty contract (`docs/2-execution/10`): a topology alarm is not mutation proof. Restated: actual mutation proof must be PR-blocking for guarded code; a skipped/scheduled/informational trigger is never PR pass for a code-changing closure artifact.
5. Fail-closed validation surface: this ticket makes the mutation evidence gate fail-closed. Confirm the required PR contexts for guarded paths include `mutation in-diff (lock layer)` (or an equivalent actual mutation-result gate) or a fail-closed check refusing guarded changes without a current accepted mutation artifact; and that the manifest parser rejects `mutation_status: killed` unless the artifact cites current in-diff or full-campaign evidence with denominator and caught/unviable/missed/timeout counts plus baseline-miss reconciliation. The scheduled full-campaign reconciliation remains valuable as standing-perimeter reconciliation, not as the PR gate.

## Architecture Check

1. Promoting actual in-diff mutation to a required PR context (or a fail-closed current-mutation-artifact check) closes the "topology alarm mistaken for mutation proof" pattern: a guarded change cannot merge on a green trigger that proves nothing about whether the diff's mutants are killed. The scheduled full campaign stays as reconciliation, so standing-perimeter coverage is not lost.
2. No backwards-compatibility aliasing/shims: the trigger is reclassified as a labeled alarm, not retained as an alternative pass path. The manifest mutation-evidence parser extends ticket 004's hook rather than adding a second parser.

## Verification Layers

1. PR-blocking (INV-098) → a CI topology guard in `ci_workflow_guards.rs` verifying `mutation in-diff (lock layer)` (or an equivalent actual mutation-result gate) is in the required ruleset contexts for guarded code.
2. Evidence freshness → manifest-parser logic rejecting `mutation_status: killed` unless the artifact cites current in-diff or full-campaign evidence with denominator + caught/unviable/missed/timeout counts + baseline-miss reconciliation.
3. Trigger-is-alarm → an assertion that a skipped/scheduled/informational trigger does not count as PR pass for a code-changing closure artifact.

## What to Change

### 1. Make in-diff mutation a required PR context

Add `mutation in-diff (lock layer)` (or an equivalent actual mutation-result gate) to the required ruleset contexts for guarded paths, or add a fail-closed check that refuses guarded changes without a current accepted mutation artifact. Keep the scheduled full-campaign reconciliation as standing-perimeter reconciliation only.

### 2. Mutation-evidence parser in the manifest

Fill ticket 004's mutation hook so the manifest rejects a green mutation status unless current in-diff/full-campaign evidence with full disposition counts + baseline-miss reconciliation is cited.

### 3. Pin the required-context topology

Extend `ci_workflow_guards.rs` to assert the required-context set and that the trigger is a labeled alarm, never proof.

## Files to Touch

- `.github/workflows/ci.yml` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify)
- `crates/tracewake-core/tests/support/acceptance_status_manifest.rs` (modify)
- `crates/tracewake-core/tests/mutation_completion_merge.rs` (modify — only if the in-diff gate changes the merge/reconciliation contract)
- `.cargo/mutants.toml` (modify — only if the guarded-path perimeter needs adjustment for the in-diff gate)
- `.cargo/mutants-baseline-misses.txt` (modify — only if baseline-miss reconciliation rows change)

## Out of Scope

- The base manifest state machine (ticket 004) and governance approval-field parsing (ticket 005).
- The PR-blocking-mutation doctrine wording (exec 10) — ticket 007.
- Running the standing campaign and recording denominators (ticket 009) — this ticket makes the gate exist; 009 runs it.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test ci_workflow_guards` — the required-context guard confirms `mutation in-diff (lock layer)` (or an equivalent actual mutation-result gate) is required for guarded code, and a synthetic ruleset omitting it fails.
2. `cargo test -p tracewake-core --test acceptance_status_manifest` — `mutation_status: killed` without current in-diff/full-campaign evidence (denominator + caught/unviable/missed/timeout + baseline-miss reconciliation) computes `NonPass`.
3. `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. A guarded code change cannot satisfy the required contexts on a topology trigger alone; actual mutation proof is PR-blocking.
2. No `mutation_status` computes pass with survivors present or with stale/non-current evidence (composes with ticket 004 §4.4.3).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/ci_workflow_guards.rs` — required-context-includes-in-diff guard + synthetic omission failure.
2. `crates/tracewake-core/tests/support/acceptance_status_manifest.rs` (+ driving `acceptance_status_manifest.rs`) — mutation-evidence freshness non-pass cases.

### Commands

1. `cargo test -p tracewake-core --test ci_workflow_guards --test acceptance_status_manifest`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. `cargo mutants -f crates/tracewake-core/tests/support/acceptance_status_manifest.rs` — extend the parser/guard mutation campaign to the mutation-evidence parser.

## Outcome

Completed: 2026-06-27

Promoted actual mutation proof into the guarded required-context set. The governance-required-checks audit now includes `mutation in-diff (lock layer)`, and the full-surface mutation trigger text explicitly labels itself as an alarm, not proof; it points guarded changes at public-boundary conformance, in-diff mutation, and shard reconciliation.

Filled ticket 004's mutation-evidence hook in the acceptance status manifest. A `mutation_status: killed` block now requires `mutation_evidence: current-in-diff` or `current-full-campaign`, a non-zero denominator, caught/unviable/missed/timeout counts that sum to the denominator, zero missed mutants, zero timeouts, and `mutation_baseline_reconciliation: current-reconciled`. Trigger-only or stale baseline evidence fails closed. The wording guard synthetic manifest was updated to carry the new closed fields.

Verification run:

- `cargo test -p tracewake-core --test ci_workflow_guards --test acceptance_status_manifest` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed after `cargo clean -p tracewake-core` removed corrupt generated artifacts left by the interrupted disk-full run.
- `cargo test --workspace` — passed after the same generated-artifact cleanup.

Mutation evidence:

- `cargo mutants -f crates/tracewake-core/tests/support/acceptance_status_manifest.rs --list` was run before execution and selected 3445 workspace mutants under repository config, so the exact ticket command is not a focused parser proof.
- `cargo mutants --no-config -f crates/tracewake-core/tests/support/acceptance_status_manifest.rs --list` selected 0 mutants because the focused file is test support rather than a normal production mutation target.
- The full configured mutation command was not run as ticket-006 proof. Standing mutation completion remains ticket 009 scope.

Unrelated pre-existing dirty paths left untouched: `.claude/skills/spec-to-tickets/SKILL.md`, `.claude/skills/spec-to-tickets/references/decomposition-patterns.md`, `CLAUDE.md`, and `tools/clean-build-scratch.sh`.
