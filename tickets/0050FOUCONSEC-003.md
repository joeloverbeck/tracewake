# 0050FOUCONSEC-003: Atomic cutover — reshape the world-step request, wire core-owned discovery, flip all callers, delete the raw-envelope path

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — reshapes `WorldStepTransactionRequest`, wires actor/process discovery into the coordinator, flips every production caller, deletes the raw caller-authored process-event path
**Deps**: 0050FOUCONSEC-001, 0050FOUCONSEC-002

## Problem

Spec-0050 §4.1/§4.2 (drivers F-01, F-02). With the core-owned actor-eligibility derivation (`-001`) and declared-process registry/`DueProcessInvocation` (`-002`) landed unwired, this ticket performs the **atomic cutover**: it replaces the public `WorldStepTransactionRequest` shape so the caller supplies only controlled input + authority information, wires the two derivations into the one-tick coordinator, flips every production caller (TUI authoritative path, `advance_until`, the three no-human paths) to the new request, and **deletes** the public `due_actor_ids` / `world_process_events` fields and the raw-envelope append/apply path.

This is a deliberately **Large** single diff: per spec §8 (closure order step 1) and the atomic-cutover / local-compile-atomicity rule, the request reshape plus all in-workspace callers must change together or the tree won't compile, and leaving the old fields behind a wrapper would re-admit the F-01/F-02 defects. Splitting is unsafe, not merely inconvenient.

## Assumption Reassessment (2026-06-24)

1. `WorldStepTransactionRequest` exposes `pub due_actor_ids` (`scheduler.rs:228`) and `pub world_process_events` (`scheduler.rs:230`); production callers passing `Vec::new()` for both are the TUI authoritative action/wait path in `crates/tracewake-tui/src/app.rs`, `DeterministicScheduler::advance_until` (`scheduler.rs:822`), `run_no_human_day` (`scheduler.rs:1706`), `advance_no_human_scheduler_to_tick` (`scheduler.rs:2224`), and `advance_no_human` (`scheduler.rs:3275`). All verified at `HEAD` (`8d7c119`). The coordinator returns `WorldAdvanceResult` (`scheduler.rs:193`).
2. Spec-0050 §4.1/§4.2/§8 are authoritative; `-001` and `-002` provide the wired-in derivations. §9.1/§9.2 implementer-recorded choices were made in those tickets; this ticket consumes them.
3. Shared boundary under audit: the public `WorldStepTransactionRequest` contract and the coordinator's due-work commitment boundary in `crates/tracewake-core/src/scheduler.rs`, plus the TUI submission boundary in `crates/tracewake-tui/src/app.rs`. Adjacent contradiction: the actor-transaction product handling (F-03) and perception/interval ownership (F-04) also live in the coordinator — those are required *consequences* handled in `-005`/`-006`, not this ticket; this ticket changes only the request shape, discovery wiring, and raw-envelope deletion.
4. `INV-004` (authoritative world ignores human existence), `INV-005`/`INV-094`/`INV-108` (possession changes input only), `INV-087` (human focus cannot make the world wait), `INV-088` (declared processes), `INV-001`/`INV-009`/`INV-010` (modeled causes, cause-bearing events) motivate this ticket. After the cutover every human/no-human path advances unpossessed due actors and due declared processes through core derivation, not caller injection.
5. Enforcement surface: deterministic replay (`INV-018`/`INV-092`) and no-direct-dispatch (architecture `04` / execution `05`). The wired step must preserve replay reconstruction and route process work through an owned proposal/transition (no raw envelope). No actor-knowledge leakage is introduced: discovery reads scheduling state/log only. Possession parity (`INV-094`) holds because the controlled-input slot is the only possession-sensitive field.
6. **Schema/contract shape change (additive-vs-breaking)**: `WorldStepTransactionRequest` is restructured — `due_actor_ids` and `world_process_events` are **removed** (breaking) and controlled-input/authority fields are **added**. All consumers are in-workspace (the five production callers above plus test harnesses), so the retype is a *local compile-atomicity* unit: every consumer changes in this diff. This is a breaking contract change by design, not additive.
7. **Removal blast radius**: `pub due_actor_ids`, `pub world_process_events`, and the raw-envelope append/apply loop are deleted. Grep blast radius across the workspace: the five production callers (flipped here), the `#[cfg(test)]` harnesses that populate the fields in `scheduler.rs` and `tests/world_step_coordinator.rs` (migrated to test-only constructors / authored initial history per §4.2), and any negative-fixture references (the compile-fail guard is added in `-004`). No `docs/` or `.claude/skills/` consumer references these private fields.

## Architecture Check

1. One atomic flip — reshape + wire + flip-all-callers + delete-old-path in a single reviewable diff — is the only safe shape: a partial landing that kept `due_actor_ids`/`world_process_events` behind a wrapper or fallback would re-open F-01/F-02, and the request retype cannot compile without all callers migrated (local compile-atomicity). This overrides the default Split rule, per spec §8 and the behavioral-remediation atomic-cutover pattern.
2. No backwards-compatibility aliasing/shims: the old public fields and the raw-envelope apply path are deleted, not deprecated or wrapped. Test fixtures needing synthetic events use explicit test-only constructors or authored initial history, not the production request.

## Verification Layers

1. `INV-004`/`INV-091` (no-human-capable advancement) → replay/golden-fixture check: the production-path human/no-human held-equal differential (below) drives both paths from configured state/registry only and asserts measured nonzero actor/process counts.
2. `INV-088` (declared processes) → replay/golden-fixture check: the declared-process behavior witness — a known-cadence process becomes due from state/log, emits one causally linked event through the real coordinator, absent one tick earlier, replays identically.
3. `INV-094`/`INV-108` (possession parity) → replay/golden-fixture check: the differential compares state/log ancestry/frontier/projection across possessed-wait vs no-human, allowing only declared controller-origin differences.
4. `INV-001`/`INV-009`/`INV-010` (no direct dispatch) → codebase grep-proof: the raw-envelope append/apply loop is gone; the request exposes no `due_actor_ids`/`world_process_events`; process work routes through the owned proposal/transition.

## What to Change

### 1. Reshape `WorldStepTransactionRequest`

Remove `pub due_actor_ids` and `pub world_process_events`. Add the controlled-input/authority fields: expected frontier, origin/controller binding, content/ruleset identity, and an optional ordinary controlled proposal. The struct exposes no caller-supplied due-work population.

### 2. Wire core-owned discovery into the coordinator

In `advance_world_one_tick` / the coordinator, derive in deterministic order: due duration completions (existing), due `DueProcessInvocation`s from the `-002` registry, eligible loaded actors from the `-001` derivation (invoking their sealed decision transactions — full consumption is `-005`), then the possessed actor's controlled proposal, then accounting/observations/projections/interval delta, then one commit/rollback. Report measured nonzero actor/process counts in `WorldAdvanceResult`.

### 3. Route process work through an owned boundary; delete the raw-envelope path

Replace the `world_process_events` append/apply loop with routing each `DueProcessInvocation` through an ordinary process-origin proposal (shared pipeline) or an owned typed process transition committed in the scratch transaction. Delete the raw append/apply loop.

### 4. Flip every production caller

Migrate `app.rs` (TUI authoritative action/wait path), `advance_until`, `run_no_human_day`, `advance_no_human_scheduler_to_tick`, and `advance_no_human` to construct the reshaped request (controlled input only). Migrate `#[cfg(test)]` injectors to test-only constructors / authored initial history.

### 5. Land the production-path differential + declared-process witnesses

In `crates/tracewake-core/tests/world_step_coordinator.rs`, replace the harness-injected differential with one driven from configured world state/registry only, asserting measured nonzero actor/process counts and full state/frontier/projection/checksum + replay comparison across human/no-human; add the declared-process behavior witness. The witness must fail if production discovery is disabled while the fixture is unchanged.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — wires the derivations added by 0050FOUCONSEC-001/-002)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify)

## Out of Scope

- Full consumption of the actor decision transaction outcome and deletion of the no-human append choreography — `0050FOUCONSEC-005`.
- Core ownership of perception/interval output and TUI de-authority — `0050FOUCONSEC-006`.
- Compile-fail boundary fixtures proving callers cannot inject due work — `0050FOUCONSEC-004`.
- Any doctrine amendment (spec §5: none warranted).

## Acceptance Criteria

### Tests That Must Pass

1. The production-path human/no-human differential passes with measured nonzero actor/process counts sourced from core discovery (not request fields); it fails when discovery is disabled with the fixture unchanged.
2. The declared-process behavior witness passes: due-from-state, one causally-linked event via the real coordinator, absent one tick earlier, replays identically.
3. `cargo test -p tracewake-core --test world_step_coordinator`, `cargo test -p tracewake-tui --test command_loop_session`, and `cargo build --workspace --all-targets --locked` are green.

### Invariants

1. `WorldStepTransactionRequest` exposes no caller-supplied due-actor/process population; the raw-envelope apply path is deleted (`INV-001`/`INV-009`/`INV-010`/`INV-088`).
2. Every human/no-human path advances unpossessed due actors and due declared processes through core derivation; possession changes only the controlled-input slot (`INV-004`/`INV-094`/`INV-108`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` — production-path human/no-human held-equal differential (replaces the harness-injected one) + declared-process behavior witness + discovery-disabled negative.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-tui --test command_loop_session`
3. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings` — the full workspace must compile, which is the local-compile-atomicity proof that every caller was flipped.
