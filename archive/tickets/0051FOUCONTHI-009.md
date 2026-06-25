# 0051FOUCONTHI-009: Witness replacement and production-path conformance harness

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — replace the vacuous perception negative fixture; add the production-bootstrap differential; add a generative production-constructor corpus mode; wire a standing CI conformance lane from production loading.
**Deps**: 0051FOUCONTHI-008

## Problem

Spec `0051` §6 requires real, non-vacuous witnesses that the repaired runtime is reachable only through production loading and the public command boundary. The advertised perception negative fixture is vacuous (it names a no-arg symbol that does not exist; `expected_stderr: "cannot find function"`), there is no production-bootstrap differential proving loaded actors and declared processes advance with no scheduler-registration call, the generated lock corpus does not enter through the production constructor, and no CI lane exercises the public core runtime/TUI command boundary from production loading. This ticket replaces the vacuous fixture with one that names the now-sealed real symbols and adds the cross-cutting production-path evidence.

## Assumption Reassessment (2026-06-24)

1. Codebase: the vacuous fixture `tests/negative-fixtures/external_crate_cannot_call_tui_perception_append_helper/` (no-arg call) + its `"cannot find function"` row in `crates/tracewake-core/tests/negative_fixture_runner.rs:142`–`144`; the real writers `record_current_place_perception_and_project` (`agent/perception.rs:71`) and `record_actor_current_place_perception` (`scheduler.rs:614`), sealed from external reach by `-007`; `crates/tracewake-core/tests/generative_lock.rs` (existing lock corpus); `.github/workflows/ci.yml` (CI); `crates/tracewake-core/tests/ci_workflow_guards.rs` (CI guards). The production bootstrap is `TuiApp::from_golden` → the `-001` runtime constructor.
2. Specs/docs: spec `0051` §6 (required fixtures and tests), §4.1/§4.2 (production reachability). Execution homes `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`, `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`.
3. Shared boundary under audit: the production constructor + typed command boundary as the *only* entry the corpus and CI lane use — the same entry the TUI uses.
4. INV-091 (no-human tests are mandatory), INV-092 (deterministic replay is tested), INV-098 (feature acceptance is harsh): restated — the witnesses must run real production paths and fail when the production derivation is removed.
5. Fail-closed / replay / no-leak surface (evidence-consumer basis): the harness exercises the no-human, replay, and embodied-leak surfaces through the public boundary; it introduces no leakage or nondeterminism path and must fail closed when the derivation is deleted. `include_str!` import/method guards remain labeled topology alarms only, never sole proof.

## Architecture Check

1. A production-bootstrap differential that *fails when the derivation is removed* is the only witness that distinguishes real reachability from coordinator-composition-with-manufactured-inputs (the §3 root cause). Driving the generated corpus and CI lane through the same production constructor + typed commands as the TUI makes the evidence non-vacuous by construction.
2. No backwards-compatibility alias: the vacuous fixture is replaced (named real symbol, realistic args, pinned to a privacy/constructor diagnostic), not kept alongside a real one.

## Verification Layers

1. INV-091 (no-human reachability) -> production-bootstrap differential: a two-actor + one-declared-process package loaded through the production bootstrap advances both with no scheduler-registration call; deleting the derivation makes the test fail.
2. INV-092 (deterministic replay) -> generative corpus: production-constructor mode covers the enumerated case matrix and replays deterministically.
3. Witness replacement -> negative fixture: the perception fixture names the real sealed symbol with realistic arguments and fails with a privacy/constructor diagnostic, not `"cannot find function"`.
4. CI -> `ci_workflow_guards` grep-proof: the standing conformance lane starts from production loading and uses only the public core runtime/TUI command boundary.

## What to Change

### 1. Replace the vacuous perception fixture

Rewrite `external_crate_cannot_call_tui_perception_append_helper` to name the real sealed writer(s) with realistic arguments; update its `negative_fixture_runner.rs` row to the privacy/constructor diagnostic. Add positive in-crate tests proving the single core owner can still perform the operation.

### 2. Production-bootstrap differential

Add a test that starts through the production content/runtime constructor with no scheduler-registration calls, proves loaded actors and declared processes advance, and is wired so deliberately removing the production derivation makes it fail.

### 3. Generative production-constructor corpus

Extend `generative_lock.rs` with a production-constructor mode covering zero/one/multiple actors; possessed due/not-due/reserved; process absent/before/at/multiple-due; action accepted/rejected/partial-phase-failure; duration active/completing/interrupted/restored-mid-duration; empty tick and event-ID collision; actor-known delta quiet/novel/hidden/same-source-replacement/different-subject-separation; and uninterrupted-vs-restored continuation — each entering through the production constructor and typed command boundary.

### 4. CI conformance lane

Wire a standing conformance lane in `ci.yml` starting from production loading and using only the public core runtime/TUI command boundary; the architecture row (cited by `-011`) names that executable lane plus the compile-fail fixtures.

## Files to Touch

- `tests/negative-fixtures/external_crate_cannot_call_tui_perception_append_helper/` (modify) — name the real sealed symbol with realistic args
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify) — update the expected diagnostic from `"cannot find function"`; merge-hub contributor
- `crates/tracewake-core/tests/generative_lock.rs` (modify) — production-constructor corpus mode
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — production-bootstrap differential; merge-hub contributor
- `.github/workflows/ci.yml` (modify) — standing conformance lane
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify — as surfaced) — guard the new lane

## Out of Scope

- Running the mutation campaign (F-08 → `-010`).
- Doc-truthing the conformance index (F-09 → `-011`).

## Acceptance Criteria

### Tests That Must Pass

1. The replaced perception fixture fails with a privacy/constructor diagnostic naming the real symbol; the positive in-crate test passes.
2. The production-bootstrap differential passes and fails when the production derivation is removed (verified by a deliberate local edit during review).
3. `cargo test -p tracewake-core --test generative_lock` and `--test world_step_coordinator` are green; `cargo test -p tracewake-core --test ci_workflow_guards` passes.

### Invariants

1. Every corpus/CI entry enters through the production constructor + typed command boundary.
2. `include_str!`/topology guards are labeled alarms only, never sole proof.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/generative_lock.rs` — production-constructor corpus mode.
2. `crates/tracewake-core/tests/world_step_coordinator.rs` — production-bootstrap differential.
3. `tests/negative-fixtures/external_crate_cannot_call_tui_perception_append_helper/` — real-symbol seal.

### Commands

1. `cargo test -p tracewake-core --test generative_lock`
2. `cargo test -p tracewake-core --test negative_fixture_runner --test ci_workflow_guards` (run each target)
3. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Replaced the vacuous perception negative fixture. `external_crate_cannot_call_tui_perception_append_helper` now depends on `tracewake-core`, names the real sealed `record_current_place_perception_and_project` helper with realistic aggregate arguments, and is pinned to a privacy diagnostic instead of `"cannot find function"`.

Added production-path runtime evidence without inverting crate dependencies. The content loader handoff test now loads a two-actor fixture through `load_fixture_package`/`RuntimeInitialState`, drives `LoadedWorldRuntime::wait_one_tick`, and proves both loaded actors plus the declared process are derived by the runtime with no manual scheduler registration. `generative_lock` now includes `generated_cases_enter_through_loaded_runtime_constructor`, which wraps generated cases in `LoadedWorldRuntime`, enters through a typed runtime command, and rebuilds the runtime-owned log without divergence.

Wired a standing CI conformance lane by adding the focused runtime-constructor generative test and content-loader handoff test to `.github/workflows/ci.yml`, guarded by `ci_workflow_guards`.

Verification run:

1. `cargo test -p tracewake-core --test negative_fixture_runner --quiet` — passed.
2. `cargo test -p tracewake-core --test generative_lock generated_cases_enter_through_loaded_runtime_constructor --quiet` — passed.
3. `cargo test -p tracewake-content load::tests::loaded_fixture_hands_off_derived_runtime_due_work --quiet` — passed.
4. `cargo test -p tracewake-core --test ci_workflow_guards --quiet` — passed.
5. `cargo test -p tracewake-core --test generative_lock --quiet` — passed.
6. `cargo test -p tracewake-core --test world_step_coordinator --quiet` — passed.
7. `cargo test --workspace --quiet` — passed.
8. `cargo build --workspace --all-targets --locked` — passed.
9. `cargo clippy --workspace --all-targets -- -D warnings` — passed.
10. `cargo fmt --all --check` — passed.
