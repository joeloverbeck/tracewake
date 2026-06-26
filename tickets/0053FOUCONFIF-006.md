# 0053FOUCONFIF-006: Token-gated debug/no-human command authority (debug/operator classification)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` runtime command/receipt + debug-capability authority surface (unforgeable `DebugSessionAuthority` token; resealed self-minting constructors); `tracewake-tui` token-supply migration; new external negative fixture
**Deps**: 0053FOUCONFIF-005

## Problem

Spec 0053 §4.3 (F5-03): `RuntimeCommandKind` is crate-private (good), but `pub fn RuntimeCommand::run_no_human_day()` mints `DebugCapability` internally; the precise hole (refined at reassessment) is that `DebugCapability::mint()` is already `pub(crate)` — so external crates cannot forge a capability and `RuntimeCommand::debug_view(capability)` correctly requires a presented token — while the *self-minting public constructors* bypass it: `run_no_human_day()` and the public `Debug*View::new()` family (`DebugControllerBindingView`, `DebugEventLogView`, `DebugItemLocationView`, `DebugActionRejectionView`) call the crate-private `mint()` internally rather than requiring a runtime-held token. The TUI checks `debug_available()`, so protection lives at the client, not the core public boundary. The split is ambiguous and not fail-closed.

Per the approved classification (Q2, 2026-06-26) — **debug/operator execution** — this ticket requires an unforgeable `DebugSessionAuthority` (or equivalent) token supplied by the runtime/controller binding state: the public constructor must **not** mint the token internally, and debug receipt/view constructors must require the same token or be crate-private builders fed by runtime-owned debug APIs (§4.3, §9 step 5).

## Assumption Reassessment (2026-06-26)

1. `crates/tracewake-core/src/runtime/command.rs` keeps `pub(crate) enum RuntimeCommandKind` (line 17) and `pub fn run_no_human_day()` (115) that mints `DebugCapability::mint()` (118), stored as an unused `_capability`. `crates/tracewake-core/src/debug_capability.rs` defines `pub(crate) const fn mint()` (27). `crates/tracewake-core/src/view_models.rs` has public self-minting `DebugControllerBindingView::new` / `DebugEventLogView::new` / `DebugItemLocationView::new` / `DebugActionRejectionView::new` (each `pub fn new()` calling `DebugCapability::mint()`). `crates/tracewake-tui/src/run.rs:127` checks `debug_available()` and calls `run_no_human_day()` at 184. **Blast radius (grepped this session)**: `run_no_human_day` / the `Debug*View::new` family are consumed across `tui/src/app.rs`, `runtime/command.rs`, and many test files (`golden_scenarios.rs`, `acceptance_gates.rs`, `no_human_capstone.rs`, `parity/scenario.rs`, `fixtures_load.rs`, `golden_fixtures_run.rs`, `event_schema_replay_gates.rs`).
2. Spec authority: `specs/0053_…_HARDENING_SPEC.md` §4.3 (classify + token-gate + anti-regression guard), §10.2 (Q2 classification — **debug/operator chosen**: require runtime-minted token), §9 step 5. This ticket `Deps: 005` because it consumes the embodied/debug receipt split point 005 establishes (exact tick/frontier/stop/replay detail reaches only the token-gated debug receipt).
3. Cross-artifact boundary under audit: the core runtime command boundary (`command.rs`, `receipt.rs`, `debug_capability.rs`) consumed by the TUI (`app.rs`, `run.rs`, `input.rs`). The token is minted by the runtime/controller binding state and presented by callers; the TUI's `debug_available()` check becomes a presentation convenience over a core boundary that is itself fail-closed.
4. Motivating invariants: INV-004 (authoritative world ignores human existence), INV-005 (only normal binding is ordinary possession), INV-006 (possession transfers no world knowledge), INV-094 (possession parity is tested), INV-031/INV-068 (debug is non-diegetic). Debug capability must be caller-held and non-diegetic, never self-minted by a public constructor.
5. This ticket touches the no-human / possession-parity / debug enforcement surface: token-gating strengthens it — debug/operator authority becomes unrepresentable without a runtime-minted token, and no-human-day is classified as debug/operator (not ordinary play), so it exposes no debug metrics to embodied output. No leakage or replay nondeterminism is introduced: the token is an authority marker, not a data channel; command semantics and event commitment are unchanged. Fail-closed: a debug/operator command without a valid token is rejected at the core boundary, not merely hidden by the client.
6. Schema/visibility change (additive-vs-breaking): the constructor signatures for `run_no_human_day` and the `Debug*View::new` family change to require a `DebugSessionAuthority` token (or become crate-private builders); the debug receipt requires the token. Consumers are the TUI and the test call sites in item 1. This is **breaking-internal** (deliberate reseal; no external stability owed; no alias).
7. Removal blast radius (grep-proof, item-7): removing the self-minting from the public constructors (`run_no_human_day`, `Debug*View::new` family) — consumers in item 1 migrate to the token path (production: TUI supplies the runtime-minted token) or to `test-support`-gated minting (tests). No public self-minting constructor survives; `DebugCapability::mint()` stays `pub(crate)` and `DebugSessionAuthority` minting is runtime-internal.

## Architecture Check

1. An unforgeable runtime-minted `DebugSessionAuthority` token at the core boundary is cleaner than the current client-side `debug_available()` check: it makes debug/operator authority a type-level requirement that an external crate cannot satisfy, rather than a convention the client is trusted to honor. Classifying no-human-day as debug/operator (not ordinary play) resolves the straddle the spec flags — the capability stops being decorative.
2. No backwards-compatibility aliasing or durable shim. Test minting uses `#[cfg(feature = "test-support")]`, not a public production self-mint.

## Verification Layers

1. INV-004/INV-005/INV-006/INV-094 (no-human / possession parity) -> external negative fixture (compile-fail / boundary): an external crate importing `tracewake_core` directly cannot construct or submit a debug/operator command (or debug receipt/view) without a runtime-minted token — a fixture that only asserts the TUI checks debug mode is vacuous.
2. INV-031/INV-068 (debug non-diegetic) -> TUI behavior test: the command loop rejects `debug run no-human-day` when not bound in debug mode and succeeds only via the token path.
3. Possession parity -> existing parity suite: binding changes input only; debug authority does not transfer world knowledge (run the parity scenario).
4. Cross-artifact: no public self-minting constructor remains (grep-proof: no `pub fn` in `view_models.rs`/`command.rs` calls `DebugCapability::mint()` outside `#[cfg(feature = "test-support")]`).

## What to Change

### 1. Unforgeable `DebugSessionAuthority` token (`debug_capability.rs`)

Introduce a `DebugSessionAuthority` (or extend `DebugCapability`) minted only by the runtime/controller binding state (runtime-internal). Public constructors must not mint it.

### 2. Token-gate no-human-day + debug commands (`runtime/command.rs`, `runtime/receipt.rs`)

`run_no_human_day` requires a presented `DebugSessionAuthority`; classify it as debug/operator (not ordinary play). The debug receipt is constructible only with the token or via crate-private builders fed by runtime-owned debug APIs (consumes 005's split).

### 3. Reseal self-minting debug-view constructors (`view_models.rs`)

The `Debug*View::new` family (`DebugControllerBindingView`, `DebugEventLogView`, `DebugItemLocationView`, `DebugActionRejectionView`) must require the token or become crate-private builders. Provide `#[cfg(feature = "test-support")]` minting for tests only.

### 4. TUI token supply (`tui/src/app.rs`, `tui/src/run.rs`, `tui/src/input.rs`)

The TUI obtains the runtime-minted token from the controller binding state and presents it when invoking no-human-day / debug views; `debug_available()` remains a presentation convenience over the now-fail-closed core boundary.

### 5. External negative fixture (`tests/negative-fixtures/…`, new) + runner; test-consumer migration

Add `external_crate_cannot_submit_debug_command_without_token` importing `tracewake_core` directly and failing at the boundary; register in `negative_fixture_runner.rs`. Migrate the test call sites in item 1 to `test-support`-gated minting (implementation-discovered set).

## Files to Touch

- `crates/tracewake-core/src/debug_capability.rs` (modify)
- `crates/tracewake-core/src/runtime/command.rs` (modify)
- `crates/tracewake-core/src/runtime/receipt.rs` (modify — debug-receipt token gating; split established in 005, `Deps: 005`)
- `crates/tracewake-core/src/view_models.rs` (modify — reseal `Debug*View::new` family; sealed surface from 005, `Deps: 005`)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-tui/src/input.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify)
- `tests/negative-fixtures/external_crate_cannot_submit_debug_command_without_token/` (new — `Cargo.toml` + `src/lib.rs`)
- Test consumers of `run_no_human_day` / `Debug*View::new` (modify — implementation-discovered set; candidates in Assumption Reassessment item 1)

## Out of Scope

- The embodied interval/receipt seal itself (005 — this ticket consumes its split point).
- Bootstrap reseal (004), `food_source` survivors (007), mutation run (009), doc-truthing (008).
- Reclassifying no-human-day as ordinary play (Q2 selected debug/operator).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test negative_fixture_runner` — `external_crate_cannot_submit_debug_command_without_token` fails at the core boundary (not a TUI-only check) under default and all supported feature combinations.
2. A `tracewake-tui` behavior test: the command loop rejects `debug run no-human-day` when not bound in debug mode and succeeds only via the token path.
3. `grep -nE "pub fn (new|run_no_human_day)" crates/tracewake-core/src/view_models.rs crates/tracewake-core/src/runtime/command.rs | xargs -I{} true` plus a guard that no public constructor calls `DebugCapability::mint()` outside `#[cfg(feature = "test-support")]`.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — clean.

### Invariants

1. Debug/operator command + receipt + view authority is unrepresentable without a runtime-minted token at the core public boundary (INV-004/005/006/094).
2. No-human-day is classified debug/operator and exposes no debug metrics to embodied output (INV-031/068).

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/external_crate_cannot_submit_debug_command_without_token/src/lib.rs` (new) — boundary compile-fail / rejection proof.
2. `crates/tracewake-tui/` behavior test — token-path success + non-debug rejection.
3. Possession-parity suite re-run (binding changes input only).

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo test -p tracewake-tui`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
