# 0052FOUCONFOU-009: Production-boundary conformance corpus + all-feature external negative fixtures

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — external compile-fail negative-fixture corpus (default + all supported feature combinations), production-bootstrap conformance behavior matrix, generative-lock production-constructor mode
**Deps**: 0052FOUCONFOU-002, 0052FOUCONFOU-003, 0052FOUCONFOU-004, 0052FOUCONFOU-005, 0052FOUCONFOU-006, 0052FOUCONFOU-007, 0052FOUCONFOU-008

## Problem

Spec 0052 §7 and §4.10 Layer A/B (test side); closure-order step 8 ("strengthen the production-boundary corpus and negative fixtures; make every closure witness non-vacuous and mutation-sensitive"). The per-finding tickets (002–008) each land their targeted witnesses, but the spec requires a consolidated, non-vacuous corpus proving the closed authority boundary holds at the production entry and across **all supported feature combinations**: external compile-fail fixtures naming the *real* forbidden capabilities, a production-bootstrap-to-sealed-receipt behavior matrix entering only through the production constructor and typed commands, and a generative-lock production-constructor mode. The default-feature negative fixtures historically prove only one compilation shape, not unforgeability across supported feature combinations.

This ticket authors that corpus. It builds the **test-side** of the enforced standing barrier's Layer A (compile-time unrepresentability) and Layer B (the behavior matrix); the CI *wiring* of the required conformance lane and branch protection is 0052FOUCONFOU-011.

## Assumption Reassessment (2026-06-25)

1. External negative fixtures live at repo-root `tests/negative-fixtures/<name>/` and are driven by `crates/tracewake-core/tests/negative_fixture_runner.rs` (the runner resolves `CARGO_MANIFEST_DIR/../../tests/negative-fixtures/<name>` via `fixture_root`, line 280, and matches `expected_stderr` per fixture). The behavior suites are `crates/tracewake-core/tests/{generative_lock,world_step_coordinator}.rs` and `crates/tracewake-tui/tests/{command_loop_session,playable_capability_parity}.rs`; topology alarms live in `crates/tracewake-core/tests/anti_regression_guards.rs`. A repo-wide grep confirms no existing fixture exercises an all-feature-combination compile check (none in `crates/tracewake-core/tests/` reference `test-support`).
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §7, §4.10 Layer A/B. The forbidden capabilities this corpus targets are removed by 002 (scheduler injection / `RuntimeInitialState`), 003 (sequence allocation / world-advance boolean / `run_pipeline` over aggregates), 004 (raw getters / rebuild / perception / no-human actor list), and 008 (embodied product construction/mutation / debug→embodied conversion / exact temporal access). It must run after all of them so the fixtures pin real privacy/constructor diagnostics, not generic "cannot find function".
3. Cross-artifact boundary under audit: the full production path — validated content package → production loader/bootstrap → opaque runtime/session → typed TUI/application commands → core transaction(s) → sealed embodied/debug receipts → rendered normal/debug output → replay/rebuild continuation. The proving tests must not construct `DeterministicScheduler`, `RuntimeInitialState`, `PipelineContext`, due actor/process lists, raw proposals with assigned sequence, or raw view models.
4. Motivating invariant: INV-098 ("Feature acceptance is harsh") — non-vacuous, path-under-test acceptance; supported by INV-092/093/094 (replay/leakage/parity regression-tested). A witness is vacuous if it constructs a correct runtime independently of the production bootstrap or asserts only that a correct path *can* work.
5. Fail-closed / evidence-consumer surface (substrate basis): this corpus *audits* the replay (INV-018), no-leak (INV-024/093), possession-parity (INV-094), and fail-closed-validation surfaces the per-finding tickets implement; it adds no production behavior, so it introduces no leakage or nondeterminism path of its own. It confirms the per-finding enforcement surfaces (002–008) hold at the production boundary and across feature combinations; the named enforcement surfaces are those tickets, not this one. The `include_str!` topology alarms in `anti_regression_guards.rs` are labeled secondary alarms pointing to the compile-time/executable witnesses they protect, each with a synthetic negative proving the guard can fire.

## Architecture Check

1. One consolidated production-boundary corpus that enters only through the production constructor and typed commands is cleaner than scattering full matrices across the per-finding tickets because it proves the *composite* path holds (no client can compose a bypass) and adds the all-feature-combination compile checks that single-shape fixtures miss — making the closure witnesses non-vacuous as a set.
2. No backwards-compatibility shim: the corpus adds no production surface and no alias; it reuses and extends the existing negative-fixture / generative-lock / coordinator / command-loop machinery rather than adding a new framework.

## Verification Layers

1. INV-098 (non-vacuous acceptance) -> production-bootstrap behavior matrix: production bootstrap + one-tick wait (other actors + declared process advance); non-wait accepted/rejected; continue/duration sealed stop; no-human runtime-derived census; possession differential; replay restore (non-default continuation); declared process; actor census (one disposition per loaded actor); information-flow pair; failure injection (no partial cutover) — each entering only through the production constructor and typed commands.
2. INV-069 (client boundary) -> external compile-fail fixtures under default **and all supported feature combinations**: scheduler injection / `RuntimeInitialState` construction, raw aggregate getters, proposal-sequence allocation, world-advance boolean, `run_pipeline` over authoritative aggregates, no-human actor-list injection, rebuild/perception writers, embodied-product construction/mutation, debug→embodied conversion, exact temporal-field access — each pinned to a privacy/constructor diagnostic; plus positive in-crate tests proving the single core owner can still perform each operation.
3. INV-092 (deterministic replay) -> generative-lock production-constructor mode (zero/one/multiple actors, possessed due/not-due/reserved, process absent/before/at/multiple-due, action accepted/rejected/partial-phase-failure, duration active/completing/interrupted/restored-mid-duration, empty tick + event-ID collision, actor-known delta quiet/novel/hidden, uninterrupted-vs-restored continuation) — each entering through the production constructor + typed command boundary.

## What to Change

### 1. External all-feature negative fixtures (`tests/negative-fixtures/*`, `negative_fixture_runner.rs`)

Add fixtures naming the real forbidden capabilities (item 2 list) under default and all supported feature combinations, each pinned to a privacy/constructor diagnostic; register them in the runner with `expected_stderr`. Add positive in-crate tests proving the single core owner can still perform each operation. Reuse/extend the existing `0050`/`0051`-line privacy guards.

### 2. Production-bootstrap behavior matrix (`world_step_coordinator.rs`, `command_loop_session.rs`, `playable_capability_parity.rs`)

Author the §4.10 minimum behavior matrix entering only through the production constructor and typed commands, asserting committed effects/receipts/replay agreement (not counters). Include the production-bootstrap differential (no scheduler-registration calls; deliberate-derivation-removal makes it fail).

### 3. Generative-lock production-constructor mode (`generative_lock.rs`)

Add a production-constructor mode covering the full state matrix (item 3), each entering through the same production constructor and typed command boundary as the TUI.

### 4. Topology alarms (`anti_regression_guards.rs`)

Add labeled `include_str!`/import alarms pointing to the witnesses they protect, each with a synthetic negative proving the guard can fire — secondary only.

## Files to Touch

- `tests/negative-fixtures/` (new fixtures — real forbidden-capability compile-fail cases, default + all-feature variants)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — register new fixtures + feature-combination variants)
- `crates/tracewake-core/tests/generative_lock.rs` (modify — production-constructor mode)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — production-bootstrap behavior matrix entries)
- `crates/tracewake-tui/tests/command_loop_session.rs` (modify — production-boundary command matrix)
- `crates/tracewake-tui/tests/playable_capability_parity.rs` (modify — possession differential through production path)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — labeled topology alarms + synthetic negatives)

## Out of Scope

- CI wiring of the required public-boundary conformance lane and branch-protection (011).
- The standing mutation campaign run + survivor closure (010).
- Any production-code change (002–008 own those; this ticket is evidence-only).

## Acceptance Criteria

### Tests That Must Pass

1. External compile-fail fixtures fail to compile under default **and all supported feature combinations** for every forbidden capability, each pinned to a privacy/constructor diagnostic; positive in-crate tests prove the core owner still performs each operation.
2. The production-bootstrap behavior matrix passes entering only through the production constructor + typed commands (no test constructs a scheduler, `RuntimeInitialState`, `PipelineContext`, due lists, raw proposals, or raw view models); the production-bootstrap differential fails when the production derivation is deliberately removed.
3. The generative-lock production-constructor mode passes across the full state matrix; `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Every closure witness enters at the production boundary; none constructs the forbidden internal authority objects (INV-098).
2. The forbidden capabilities are uncompilable from an external crate in every supported feature combination (INV-069).

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/*` + `crates/tracewake-core/tests/negative_fixture_runner.rs` — all-feature forbidden-capability corpus.
2. `crates/tracewake-core/tests/{generative_lock,world_step_coordinator}.rs`, `crates/tracewake-tui/tests/{command_loop_session,playable_capability_parity}.rs` — production-boundary behavior matrix + generative mode.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner && cargo test -p tracewake-core --test generative_lock`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
