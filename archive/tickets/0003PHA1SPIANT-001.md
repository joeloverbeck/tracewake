# 0003PHA1SPIANT-001: Seal authoritative state mutation behind an event-application capability

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`state.rs` field visibility + read accessors; new `events::mutation` capability module; every in-workspace consumer of `PhysicalState`/`AgentState` fields across `tracewake-core`, `tracewake-content`, `tracewake-tui`)
**Deps**: None

## Problem

`PhysicalState` and `AgentState` expose public mutable fields (`state.rs:122-130`, `:133-140`), so the "single mutation path through event application" is a convention plus test evidence rather than a compile-time property. Any future planner, scheduler, debug panel, content loader, or test harness can write `state.actors.insert(...)` and the tree still compiles, silently bypassing the event log. Spec `0003` §5.5 / SPINE-AC-001 require moving the no-direct-dispatch boundary from doctrine into the type system: authoritative mutation must be unreachable except through the event-application path.

## Assumption Reassessment (2026-06-08)

1. `PhysicalState` (`crates/tracewake-core/src/state.rs:122-130`) and `AgentState` (`:133-140`) currently declare all collection fields `pub` (`actors`, `places`, `doors`, `containers`, `items`, `food_supplies`, `workplaces`; `needs_by_actor`, `intentions`, `active_intention_by_actor`, `routine_executions`, `decision_traces`, `stuck_diagnostics`). Event application already owns the intended mutable context: `EventApplicationContext` at `crates/tracewake-core/src/events/apply.rs:27`, applied via `apply_event_stream` (`apply.rs:40`) / `apply_event` (`apply.rs:106`).
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-001 mandates: private/`pub(crate)` fields behind read-only public accessors, an unforgeable `WorldMutationCapability`/`AgentMutationCapability` constructible only inside `events::apply` (or a private `events::mutation` module), state-changing methods requiring that capability, and fixture/test builders that are not runtime mutation paths.
3. Cross-artifact boundary under audit: the `state.rs` ↔ event-application contract. A repo-wide grep finds **577 field-access sites across ~35 files** in all three crates (`428` physical-field, `149` agent-field). Because consumers live across the whole workspace and Rust visibility is checked at compile time, the field retype + every consumer update is a single **local compile-atomicity** unit — splitting it would land an uncompilable intermediate tree.
4. INV motivating this ticket — restated before trusting the narrative: `INV-009` (meaningful state changes are events), `INV-011` (no hidden mutation outside the event path), `INV-104` (single authoritative mutation path / no direct dispatch), reinforced by `INV-099`/`INV-107` (kernel authority, debug/non-diegetic surfaces hold no mutation power). The seal must enforce these at compile time, at least as strictly as the constitution requires.
5. Deterministic-replay surface touched: `PhysicalState`/`AgentState` are the replayed authoritative state and feed `checksum.rs` canonicalization (`INV-017`/`INV-018`). The seal must preserve byte-identical replay — read accessors return the same data, fixture builders construct the same initial state, and event application mutates through the new capability with identical ordering. No accessor may expose interior-mutable references that bypass the capability. This ticket introduces no new field and no nondeterminism path; checksum totality is enforced separately by 0003PHA1SPIANT-004.
6. Schema-shape change (visibility), not a data-field change: `PhysicalState`/`AgentState` field *visibility* goes `pub` → private/`pub(crate)` + accessors. Consumers = the entire workspace (35 files enumerated in item 3). The change is **breaking but local-atomic** — every consumer is in-workspace and updated in this same diff; no external crate consumes these fields (zero published API). Read sites migrate to accessors; write sites migrate to capability-gated methods or are proven to be event-application / fixture-construction sites.
7. Removal blast radius (public-field-write access removed): grep `\.(actors|places|doors|containers|items|food_supplies|workplaces|needs_by_actor|intentions|active_intention_by_actor|routine_executions|decision_traces|stuck_diagnostics)\b` across `crates/*/src` and `crates/*/tests` — 577 sites / ~35 files. Per area: `tracewake-core/src` (events/apply, replay/rebuild, replay/report, checksum, scheduler, projections, controller, debug_reports, agent/*, actions/pipeline, actions/defs/*), `tracewake-content/src` (load, schema, serialization, validate), `tracewake-tui/src` (app, debug_panels, launch), plus tests in all three crates. Write sites must route through the capability or a fixture builder; read sites adopt accessors.

## Architecture Check

1. Compile-time sealing (object-capability + Rust private fields) makes the invalid operation *unreachable* from the wrong layer, which is strictly stronger than the current convention-plus-tests posture: a planner or debug panel that tries to mutate authoritative state fails to compile instead of being caught by a reviewer remembering doctrine. Read accessors keep projection/checksum/view-model code working unchanged in behavior. Builders confined to fixture/seed construction keep content loading and tests honest without offering a runtime mutation door.
2. No backwards-compatibility aliasing or shims: the public fields are removed outright (not deprecated behind a wrapper or a `pub` re-export). Per spec §3.2 Non-goals and the anti-contamination thesis, leaving the old public-field path live behind any convenience layer would defeat the seal — the cutover is atomic.

## Verification Layers

1. `INV-011` / `INV-104` (no hidden mutation; single mutation path) -> codebase grep-proof: no `(modify)`-path write to `PhysicalState`/`AgentState` fields outside `events::apply` / `events::mutation` / fixture builders; capability constructor call sites allowlisted by source scan.
2. `INV-009` (state changes are events) -> compile/public-API check: state-changing methods require a `WorldMutationCapability`/`AgentMutationCapability` value; external crates cannot construct one.
3. `INV-018` (deterministic replay) -> replay/golden-fixture check: existing golden scenarios and `checksum.rs` insertion-order-independence tests reproduce byte-identical checksums after the reseal (read accessors and builders preserve data and order).
4. `INV-099`/`INV-107` (kernel authority; debug holds no mutation power) -> manual review + source scan: debug/TUI/content layers reach state only through read accessors.

## What to Change

### 1. Make `PhysicalState` / `AgentState` fields non-public and add read accessors

In `crates/tracewake-core/src/state.rs`, change the field visibility of `PhysicalState` and `AgentState` collections to private (or `pub(crate)` where intra-crate read ergonomics demand it) and add `pub fn <field>(&self) -> &<CollectionType>` read accessors for each. Accessors return shared references only — never `&mut` and never an interior-mutable handle.

### 2. Introduce unforgeable mutation capabilities

Add a private `crates/tracewake-core/src/events/mutation.rs` module (registered from `events/mod.rs`) exposing `WorldMutationCapability` and `AgentMutationCapability` newtypes with a private field and a `pub(crate)` (or narrower, confined to `events::apply`) constructor. State-changing methods on `PhysicalState`/`AgentState` (or free functions in the mutation module) take the relevant capability by value/reference so they are callable only where a capability can be minted.

### 3. Route event application through the capability

Update `EventApplicationContext` / `apply_event` / `apply_event_stream` (`events/apply.rs`) to mint and pass the capability when applying world/agent events, preserving current application order and semantics.

### 4. Provide fixture/seed builders distinct from runtime mutation

Add explicit seed/fixture builders (e.g. `PhysicalState::from_seed(...)` / a `state::builders` surface) used by `tracewake-content` loaders and by tests to construct initial state, marked as construction-not-runtime-mutation and audited by content validation. These do not accept or expose a runtime mutation capability.

### 5. Migrate every consumer

Update all 35 consumer files: read sites → accessors; write sites → capability-gated methods (if legitimately event-application) or fixture builders (if construction). The tree must compile and all existing tests pass with no behavioral change.

## Files to Touch

- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/events/mutation.rs` (new)
- `crates/tracewake-core/src/events/mod.rs` (modify — register `mutation`)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/src/replay/report.rs` (modify)
- `crates/tracewake-core/src/checksum.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/controller.rs` (modify)
- `crates/tracewake-core/src/debug_reports.rs` (modify)
- `crates/tracewake-core/src/agent/actor_known.rs` (modify)
- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- `crates/tracewake-core/src/agent/generation.rs` (modify)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/src/actions/defs/*.rs` (modify — movement, takeplace, openclose, wait, checkcontainer, continue_routine, accuseprobe, eat, inspect, sleep, work)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/serialization.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/debug_panels.rs` (modify)
- `crates/tracewake-tui/src/launch.rs` (modify)
- `crates/tracewake-core/tests/*.rs`, `crates/tracewake-content/tests/*.rs`, `crates/tracewake-tui/tests/*.rs` (modify — read/build migration as surfaced by the compiler)

## Out of Scope

- Checksum coverage totality (owned by 0003PHA1SPIANT-004).
- Scheduler no-direct-dispatch conformance test (0003PHA1SPIANT-006) and pipeline append-before-apply test (0003PHA1SPIANT-008) — they consume this ticket's capability but assert their own surfaces.
- Any change to event semantics, ordering, or the set of fields. Visibility-and-mutation-path only; no new mechanics.
- Actor-known / debug-capability sealing (0003PHA1SPIANT-009).

## Acceptance Criteria

### Tests That Must Pass

1. A compile-fail / public-API test proving code outside `events::apply` / `events::mutation` cannot mutate `PhysicalState`/`AgentState` fields and cannot construct a `WorldMutationCapability`/`AgentMutationCapability`.
2. A source-scan conformance test allowlisting the only files permitted to mint a mutation capability (event application; fixture builders for construction).
3. `cargo test --workspace` — existing replay/golden/checksum/TUI suites pass unchanged (no behavioral regression).
4. `cargo build --workspace --all-targets --locked` — the whole workspace compiles after the reseal.

### Invariants

1. Authoritative world/agent mutation is reachable only through the event-application capability path (`INV-011`, `INV-104`).
2. Read accessors and fixture builders preserve byte-identical replay/checksum output (`INV-018`); no accessor exposes a mutation handle.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — add the mutation-capability source-scan allowlist + the "no direct field write outside apply/mutation" scan.
2. `crates/tracewake-core/src/state.rs` (or a `tests/` compile-fail harness) — public-API test that external mutation / capability construction does not compile.
3. Existing `crates/tracewake-core/src/checksum.rs` and golden-scenario tests — migrate to read accessors; assert unchanged checksums.

### Commands

1. `cargo build --workspace --all-targets --locked`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings` — the reseal must not introduce warnings (narrower than full pipeline but the relevant boundary for a visibility refactor).

## Outcome

Completed: 2026-06-08

What changed:
- `PhysicalState` and `AgentState` authoritative collections are no longer public fields; public access is through shared-reference accessors.
- Added explicit seed construction surfaces (`from_seed_parts` and `seed_*_mut`) and migrated fixture/test construction away from public field writes.
- Added private `events::mutation` capabilities and confined world/agent capability minting to `events::apply`.
- Updated content, TUI, and integration-test consumers to use accessors for reads.
- Added anti-regression guards proving state fields are not public, mutation capability minting stays private to event application, production seed mutators stay quarantined to `state.rs`, and production direct collection inserts stay out of non-apply code.

Deviations from original plan:
- The final seal uses `pub(crate)` state fields plus source-scan guards rather than fully private fields, matching the ticket allowance for crate-internal ergonomics while blocking external crates and integration tests from direct mutation.
- Seed construction is exposed as explicitly named construction APIs so existing fixture and regression tests can build initial states without using the runtime event-application path.

Verification:
- `cargo fmt --all --check`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
