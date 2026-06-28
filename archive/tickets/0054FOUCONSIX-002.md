# 0054FOUCONSIX-002: Sealed actor-legible one-tick wait receipt (atomic cutover)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` runtime-receipt/scheduler authority surface (sealed one-tick wait receipt; full `WorldAdvanceResult` confined to debug-gated path); `tracewake-tui` wait-path consumer migration; new external negative fixture + behavioral tests
**Deps**: 0054FOUCONSIX-001

## Problem

The public one-tick wait receipt hands debug-grade scheduler internals to any caller. `RuntimeReceiptKind` is a `pub enum` whose `OneTickAdvanced(WorldAdvanceResult)` variant (`crates/tracewake-core/src/runtime/receipt.rs:17-18`) is returned through `pub fn RuntimeReceipt::kind()` (`receipt.rs:99`). `WorldAdvanceResult` (`crates/tracewake-core/src/scheduler.rs:270`) exposes public fields — `prior_tick`, `resulting_tick`, `appended_event_ids`, `actor_known_interval_delta`, `due_duration_candidates`, `due_work_summary`, `actor_step_summaries`, `controlled_pipeline_results`; and `ActorStepSummary` (`scheduler.rs:282`) exposes `actor_id`, `status`, `proposal_id`, `decision_trace_id`, `local_plan_id`, `proposal_ancestry`, `pipeline_status`, `committed_event_ids`, `diagnostic_event_id`. `LoadedWorldRuntime::wait_one_tick` (`crates/tracewake-core/src/runtime/session.rs:664`) is public and returns `RuntimeReceipt::one_tick_advanced(result)` (`session.rs:694`). By contrast `ContinuedRuntimeReceipt` (`receipt.rs:36`) is sealed — it exposes only `advanced()`, `appended_event_count()`, and `actor_known_interval_summary()`. The `continue` path is sealed; the one-tick wait path is not (finding F6-02). TUI render discipline cannot repair an information-flow leak already present in the public product (INV-024 no-telepathy, INV-112 temporal authority: exact ticks/frontiers/due queues are not actor-legible on a normal wait receipt).

## Assumption Reassessment (2026-06-27)

1. `crates/tracewake-core/src/runtime/receipt.rs:17-18` defines `pub enum RuntimeReceiptKind { OneTickAdvanced(WorldAdvanceResult), … }`; `kind()` (`:99`) is `pub fn` returning `&RuntimeReceiptKind`; `one_tick_advanced` (`:61`) is `pub(crate)` but the variant it produces is publicly readable through `kind()`. `ContinuedRuntimeReceipt` (`:36`, accessors `:138`-`:146`) is the sealed exemplar to mirror. Confirmed at `7660051`.
2. `crates/tracewake-core/src/scheduler.rs:270`/`:282` confirm every enumerated public field on `WorldAdvanceResult`/`ActorStepSummary`; `AdvanceUntilResult` (`scheduler.rs:405`, sealed behind `ContinuedRuntimeReceipt::from_advance_until_result`) is the existing precedent for confining a raw scheduler product. Confirmed.
3. Shared boundary under audit: the public runtime-receipt boundary (`RuntimeReceipt::kind`) at the `tracewake-core` ↔ external-client seam, plus the TUI wait path (`crates/tracewake-tui/src/app.rs`, `run.rs`, `render.rs`). The leak is in the public core product; the proof must be at the runtime boundary, not the TUI render.
4. INV-024 (no telepathy — information reaches viewers only through modeled channels), INV-002/INV-023 (belief before truth; ground truth/belief/records separate), INV-030/INV-099/INV-101 (evidence is not truth; truth may validate but not plan; actor-known context is sealed), INV-112 (time may validate but holder-known time plans — exact ticks/frontiers/due queues are not free truth labels on a normal wait). Restated: a normal one-tick wait must expose only actor-legible progress.
5. No-leak / determinism surface: this ticket touches the actor-knowledge firewall on the receipt boundary. Confirm the sealed wait receipt exposes only actor-visible progress + a core-built actor-known interval summary (mirroring `ContinuedRuntimeReceipt`), that the full `WorldAdvanceResult` becomes reachable **only** through a `DebugSessionAuthority`-gated debug receipt (ties to ticket 003), and that no nondeterministic input enters the sealed summary. A render/transcript assertion is a labeled witness, not sole proof of sealing.
6. Schema reseal (additive-vs-breaking = **breaking**, intentional): `RuntimeReceiptKind::OneTickAdvanced(WorldAdvanceResult)` is replaced by a sealed actor-legible receipt (a new `OneTickRuntimeReceipt`, or a generalized `WorldAdvanceRuntimeReceipt` reused by wait and continue). Consumers of the variant: `crates/tracewake-core/src/runtime/session.rs:884`/`:948`, `crates/tracewake-core/tests/generative_lock.rs:341`, `crates/tracewake-content/src/load.rs:600`; callers of `wait_one_tick`: `session.rs:876`/`:897`/`:921`, `generative_lock.rs:336`, `load.rs:594`. All in-workspace → local compile-atomicity; every match site migrates in the same diff.
7. Removal blast radius of the old public surface: `RuntimeReceipt::kind()` must no longer return a variant exposing `WorldAdvanceResult` to external callers. Grep confirms `OneTickAdvanced` appears only at the six sites in item 6 plus its `receipt.rs` definition/constructor; no doc or live sibling-spec deliverable names the public `WorldAdvanceResult`-on-wait surface as committed (only archived 0053, immutable). The negative-fixture suite gains a wait-receipt extraction fixture.

## Architecture Check

1. A sealed wait receipt parallel to `ContinuedRuntimeReceipt` makes the leak unrepresentable at the type boundary: external callers cannot pattern-match a wait receipt to reach exact ticks/event IDs/ancestry/trace IDs because the type no longer carries them. Confining the full `WorldAdvanceResult` to a `DebugSessionAuthority`-gated debug receipt keeps the embodied/debug split a type boundary, not render discipline (INV-068/INV-069).
2. No backwards-compatibility aliasing/shims: the public `OneTickAdvanced(WorldAdvanceResult)` variant is removed, not deprecated-in-place. A generalized `WorldAdvanceRuntimeReceipt` reused by both wait and continue avoids duplicating the seal; whichever shape is chosen, no public accessor returns the raw scheduler product.

## Verification Layers

1. No-leak (INV-024/INV-112) → external negative fixture: an external crate cannot match a normal wait receipt and read `prior_tick`/`resulting_tick`/`appended_event_ids`/`due_work_summary`/`actor_step_summaries`/`decision_trace_id`/`pipeline_status` (pinned to a privacy diagnostic).
2. Actor-legibility (INV-067) → behavioral TUI/runtime test: the normal wait path receives only an actor-legible summary and renders no exact internals.
3. Debug reachability (INV-068) → debug/operator-mode test: the full step report is still reachable through the token-gated debug API (ties to ticket 003).
4. Information-flow differential → a hidden-context/debug-receipt differential test proving the sealed wait receipt and the debug receipt diverge exactly on the confined fields (manual-review-backed assertion, not render transcript alone).

## What to Change

### 1. Introduce the sealed wait receipt

Replace `RuntimeReceiptKind::OneTickAdvanced(WorldAdvanceResult)` with a sealed actor-legible receipt carrying only actor-visible progress, an actor-legible appended-event count if appropriate, and a core-built actor-known interval summary — mirroring `ContinuedRuntimeReceipt`. Ensure `RuntimeReceipt::kind()` can no longer return a variant exposing `WorldAdvanceResult`.

### 2. Confine the full scheduler product behind debug authority

Move `WorldAdvanceResult` / `ActorStepSummary` and the exact tick/event/due-work/ancestry/trace/pipeline fields to crate-private scheduler/debug code reachable only through a `DebugSessionAuthority`-gated debug receipt (the gating authority is hardened in ticket 003).

### 3. Migrate `wait_one_tick` and its consumers

Update `wait_one_tick` (`session.rs:664`/`:694`) and the in-workspace match/caller sites (item 6) onto the sealed receipt + the debug-gated detail path. Migrate the TUI wait path (`app.rs`, `run.rs`, `render.rs`) to render only the actor-legible summary.

### 4. Add the wait-receipt extraction negative fixture

New `tests/negative-fixtures/<name>/` (e.g. `external_crate_cannot_read_one_tick_wait_receipt_internals`) registered in `negative_fixture_runner.rs` (FIXTURES + `ALL_FEATURE_PRODUCTION_BOUNDARY_FIXTURES`), pinned to a privacy diagnostic, under default and all supported feature combinations.

## Files to Touch

- `crates/tracewake-core/src/runtime/receipt.rs` (modify)
- `crates/tracewake-core/src/runtime/session.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/tests/generative_lock.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify)
- `tests/negative-fixtures/external_crate_cannot_read_one_tick_wait_receipt_internals/` (new — `Cargo.toml` + `src/lib.rs`)

## Out of Scope

- Hardening how `DebugSessionAuthority` is obtained (F6-03, ticket 003) — this ticket consumes the gated-debug-receipt concept; ticket 003 makes the token non-inducible. Declared as `Deps`.
- The bootstrap reseal (ticket 001) and any acceptance/governance/mutation-CI change (tickets 004–006).
- Live-conformance doc-truthing for the runtime-receipt row (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test negative_fixture_runner` — the new wait-receipt extraction fixture fails to compile with the pinned privacy diagnostic, under default and all supported feature combinations.
2. A behavioral runtime/TUI test: a normal embodied one-tick wait yields only an actor-legible summary (no exact `prior_tick`/`resulting_tick`/`appended_event_ids`/ancestry/trace exposure) and the TUI renders no exact internals.
3. A debug-mode test proving the full step report remains reachable through the token-gated debug API (composes with ticket 003).
4. `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. `RuntimeReceipt::kind()` exposes no variant carrying `WorldAdvanceResult` to external callers; the raw scheduler product is reachable only via a `DebugSessionAuthority`-gated debug receipt.
2. A normal one-tick wait leaks no hidden temporal/world truth as cognition-grade output (INV-024/INV-112).

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/external_crate_cannot_read_one_tick_wait_receipt_internals/src/lib.rs` — external extraction compile-fail.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — register the new fixture + `expected_stderr`.
3. A sealed-wait/debug-separation behavioral test (extend the runtime-receipt/TUI test surface) — default-transcript exact-leak-absence + debug-receipt presence + information-flow differential.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. `cargo mutants -f crates/tracewake-core/src/runtime/receipt.rs -f crates/tracewake-core/src/scheduler.rs` — focused campaign over the resealed receipt path (standing campaign is ticket 009).

## Outcome

Completed: 2026-06-27

Implemented the one-tick wait receipt reseal for F6-02:

- replaced normal `RuntimeReceiptKind::OneTickAdvanced(WorldAdvanceResult)` with `RuntimeReceiptKind::OneTickAdvanced(OneTickRuntimeReceipt)`;
- added private-field `OneTickRuntimeReceipt` accessors for actor-legible progress, appended-event count, and actor-known interval summary only;
- kept exact ticks, event IDs, due-work summary, and actor-step summaries out of the normal receipt boundary;
- added `LoadedWorldRuntime::wait_one_tick_debug(&DebugSessionAuthority, ...)` and a debug receipt constructor that carries the raw one-tick scheduler details behind debug authority;
- migrated content/generative/runtime tests away from raw normal wait internals;
- added `external_crate_cannot_read_one_tick_wait_receipt_internals` and registered it in the default and all-feature negative fixture lanes.

Verification:

- `cargo test -p tracewake-core --test negative_fixture_runner` — passed.
- `cargo test -p tracewake-core --test generative_lock generated_cases_enter_through_loaded_runtime_constructor` — passed.
- `cargo test -p tracewake-core debug_one_tick_receipt_retains_privileged_scheduler_details` — passed.
- `cargo test -p tracewake-core one_tick_wait_advances_world_through_owned_runtime` — passed.
- `cargo test -p tracewake-core debug_receipt_is_capability_gated` — passed.
- `cargo test -p tracewake-content loaded_fixture_exports_scheduler_free_runtime_bootstrap` — passed.
- `cargo fmt --all --check` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed after adding a local `RuntimeReceiptKind` enum-size lint rationale instead of changing the public enum shape.
- `cargo test --workspace` — passed.
- `cargo mutants --list -f crates/tracewake-core/src/runtime/receipt.rs -f crates/tracewake-core/src/scheduler.rs --no-config` — completed as a denominator check for the receipt+scheduler surface.

Deviation / evidence limitation:

- The exact ticket-listed mutation command `cargo mutants -f crates/tracewake-core/src/runtime/receipt.rs -f crates/tracewake-core/src/scheduler.rs` selected `3442` mutants under repository mutation configuration, i.e. standing-size scope rather than a narrow ticket-002 campaign. It was interrupted with Ctrl-C after the selection line so this ticket would not perform ticket 009's standing mutation campaign. No mutation pass is claimed for ticket 002; standing mutation proof remains owned by `0054FOUCONSIX-009`.

Related archive truthing:

- While verifying this ticket, the workspace compile exposed that ticket 001's integration-test helpers cannot use `#[cfg(test)]` library constructors. The archived `0054FOUCONSIX-001` outcome was amended on 2026-06-27 to record that integration tests use the validated-content handoff.
