# 0003PHA1SPIANT-007: Seal and adversarially test proposal source-context binding

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`actions/proposal.rs` constructor sealing, `actions/report.rs` reason codes), `tracewake-tui` (`app.rs` submission path), adversarial pipeline tests
**Deps**: None

## Problem

Proposals carry a `ProposalSourceContext` with holder-known context identity/hash/frontier/tick (`actions/proposal.rs:82`), and reason codes cover missing/mismatch/stale/forged source conditions (`actions/report.rs` `ReasonCode`, `stable_id` at `:49`). The TUI submits actions from the current view (`app.rs:182-232`). But nothing prevents constructing a privileged semantic-action proposal from raw strings without a current source context, and the adversarial coverage (forged/stale/mismatched/privileged) is not exhaustive. Spec `0003` §5.6 / SPINE-AC-007 require constructor sealing where practical plus exhaustive adversarial tests yielding typed reason codes.

## Assumption Reassessment (2026-06-08)

1. `ProposalSourceContext` is at `crates/tracewake-core/src/actions/proposal.rs:82` (identity/hash/frontier/tick). `ReasonCode` is at `crates/tracewake-core/src/actions/report.rs:10` with `stable_id()` at `:49` (e.g. `ProposalSourceStale` — confirmed at `report.rs:247`). The TUI submits semantic actions from the current view at `crates/tracewake-tui/src/app.rs:182-232`.
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-007 mandates: constructing a TUI semantic-action proposal requires a current embodied view / source-context value (not raw strings alone); the public API exposes no constructor that forges a privileged proposal without a source context except clearly-marked test/fixture origins; negative tests cover missing source context, actor mismatch, stale context hash/frontier/tick, and a semantic action ID not in the current view; negatives produce typed `ReasonCode`s, never substring-matched labels.
3. Cross-artifact boundary under audit: the holder-known view → `ProposalSourceContext` → pipeline-validation contract spanning `tracewake-core` (`actions/proposal.rs`, `actions/report.rs`) and `tracewake-tui` (`app.rs`). Shares `actions/report.rs` with 0003PHA1SPIANT-011 (diagnostic discipline) — coordinate mechanical merges.
4. INV motivating this ticket: `INV-099`–`INV-106` (actor-known cognition / source provenance / no forged or privileged actor authority) and `INV-108` (no bypass), with `INV-065`/`INV-066`/`INV-069` (possession parity; the player has no special affordance). Restated: a proposal's authority derives only from a current, matching holder-known source context — forged, stale, or contextless proposals must be rejected with typed reasons.
5. Actor-knowledge / fail-closed-validation surface touched: source-context validation in the pipeline is the firewall ensuring the TUI cannot submit a privileged or stale action. The change must keep validation fail-closed (reject on missing/mismatch/stale/forged) and must not let a player-origin proposal bypass current-view resolution. No hidden truth enters the proposal; the source context carries only holder-known identity/hash/frontier/tick.

## Architecture Check

1. Requiring a current source-context value at construction (a typed parameter, not raw strings) moves "did this proposal come from the current view?" from a runtime substring check toward a constructor precondition, and exhaustive typed-reason negatives prove the firewall holds under adversarial input. This is stronger than today's correct-but-unforced submission path.
2. No backwards-compatibility shim: no raw-string privileged-proposal constructor is retained for convenience; test/fixture origins are explicitly marked, not a general escape hatch.

## Verification Layers

1. `INV-104`/`INV-108` (no bypass) -> compile/public-API check: no public constructor forges a privileged proposal without a source context (outside marked test/fixture origins).
2. `INV-099`–`INV-106` (source provenance) -> adversarial pipeline test: missing/mismatched/stale/forged source contexts reject with typed `ReasonCode`s.
3. `INV-070`/`DIAG` (typed diagnostics) -> codebase grep-proof: negative assertions check `ReasonCode`/`stable_id`, never actor-facing substrings.

## What to Change

### 1. Seal proposal construction behind a source context

In `crates/tracewake-core/src/actions/proposal.rs`, require a `ProposalSourceContext` (or a current embodied-view value carrying it) to construct a semantic-action proposal; mark any contextless constructor as test/fixture-only.

### 2. TUI submission requires current-view resolution

In `crates/tracewake-tui/src/app.rs`, ensure semantic-action submission resolves through the current embodied view and cannot bypass current-view resolution with raw identifiers.

### 3. Exhaustive adversarial tests with typed reasons

Add `forged_or_stale_source_context_rejected_by_reason_code` covering missing context, actor mismatch, stale hash/frontier/tick, and action-ID-not-in-current-view, asserting the typed `ReasonCode` for each.

## Files to Touch

- `crates/tracewake-core/src/actions/proposal.rs` (modify)
- `crates/tracewake-core/src/actions/report.rs` (modify — reason-code coverage if a case is missing; shared with 0003PHA1SPIANT-011)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify — adversarial submission tests)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — typed-reason pipeline negatives)

## Out of Scope

- General diagnostic typing discipline (0003PHA1SPIANT-011) — this ticket covers source-context reasons specifically.
- TUI stale-current-view selection rejection (0003PHA1SPIANT-012) — related but owned by the TUI seam ticket.
- Actor-known planning-context construction sealing (0003PHA1SPIANT-009).

## Acceptance Criteria

### Tests That Must Pass

1. `forged_or_stale_source_context_rejected_by_reason_code` — missing/stale/mismatched/forged contexts reject with typed `ReasonCode`s.
2. A public-API test proving no privileged proposal can be constructed without a source context outside marked test/fixture origins.
3. `cargo test --workspace` passes.

### Invariants

1. Proposal authority derives only from a current, matching holder-known source context (`INV-104`, `INV-108`).
2. Rejections are typed `ReasonCode`s, never substring-matched labels (`INV-070`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/adversarial_gates.rs` — adversarial current-view submission negatives.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — `forged_or_stale_source_context_rejected_by_reason_code`.

### Commands

1. `cargo test -p tracewake-tui --test adversarial_gates`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-08

What changed:
- Added `proposal_from_current_view_semantic_action`, which requires an `EmbodiedViewModel` source context for human TUI semantic-action proposal construction.
- Switched `TuiApp::submit_entry` to the current-view-only proposal helper.
- Made the optional semantic-action helper fail closed for `ProposalOrigin::Human` when no source view is supplied.
- Added `forged_or_stale_source_context_rejected_by_reason_code`, covering missing source context, stale frontier, stale tick, actor mismatch, context mismatch, and forged semantic action ID using typed `ReasonCode`s.
- Added a source guard proving TUI submission uses the current-view helper.
- Added narrow clippy allows for TUI subprocess smoke-test helpers that use `Command::new` outside simulation outcome code.

Verification:
- `cargo test -p tracewake-core --test anti_regression_guards forged_or_stale_source_context_rejected_by_reason_code`
- `cargo test -p tracewake-core --test anti_regression_guards privileged_tui_proposal_requires_current_view_source_context`
- `cargo test -p tracewake-tui --test adversarial_gates`
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo fmt --all --check`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
