# 0021PHA3APOSREB-006: Reject-loudly kernel posture — completion crossing events, authoritative intention validation, typed apply errors

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`events/apply`, `actions/defs/{sleep,work,wait,continue_routine}`, `scheduler` completion paths, shared crossing emitter); golden repricing in `tracewake-content` as surfaced
**Deps**: `specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-076, ORD-HARD-077, ORD-HARD-085)

## Problem

Three kernel-posture gaps. (1) `build_sleep_end_events` and
`build_work_completion_events` emit `NeedDeltaApplied` but never
`NeedThresholdCrossed` — a need band crossed during a duration block leaves no causal
record, though foundation doc 05 names threshold crossings as a replanning trigger
(`ORD-HARD-076`; INV-012/013); `build_threshold_events` (wait.rs) also drops
decreasing-pressure crossings via `delta.max(0)`. (2) `build_continue_routine_event`
trusts proposer-supplied `intention_status` defaulting to `"active"` and never
consults authoritative intention state — the terminal-intention gate can never fire
from the autonomous path (`ORD-HARD-077`; INV-043). (3) Replay reject-loudly gaps
(`ORD-HARD-085`; INV-020): `assert_single_tick_delta_charge` panics on a duplicate
charge and silently skips on malformed `elapsed_ticks`; three payload fields are
silently defaulted (`RoleAssignmentNoticeRecorded.access_open` → `true`,
`IntentionContinued.current_step`, `RoutineStepTransition.progress_tick` → `"0"`);
log-sourced `.expect`s in `work.rs` (`payload_value`, "work start event carries
actor") and the scheduler completion paths abort instead of rejecting. One ticket per
spec §8: the golden repricing these corrections cause is batched once.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: no `NeedThresholdCrossed` emission in
   `sleep.rs`/`work.rs` builders (wait.rs and the scheduler window path both emit);
   `delta.max(0)` in `build_threshold_events`; `intention_status` read with
   `unwrap_or("active")` in `continue_routine.rs` with no production setter (only a
   test sets it) and no `AgentState.intentions` consult;
   `assert_single_tick_delta_charge` (`events/apply.rs`) uses
   `.parse::<u64>().ok()` early-return and `assert!` on duplicates; the three
   `unwrap_or` payload defaults at their apply.rs arms; `.expect` sites at
   `work.rs` (`payload_value`, plus "work start event carries actor" ×3) and
   `scheduler.rs` ("scheduled sleep/work completion payloads are typed and valid").
2. Verified against spec 0021 (reassessed 2026-06-11; operator-verified): findings
   ORD-HARD-076/077/085; §9 notes crossing events reprice canonical days — batch
   once with the per-actor ledger treatment; reassessment M1 establishes the
   no-silent-default standard this ticket's per-field decisions must meet.
3. Cross-artifact boundary under audit: the event-emission contract (every
   band-changing need delta pairs with its crossing event), the proposal-validation
   contract (validators consult authoritative state, never proposer self-report),
   and the replay rejection contract (corrupt history → typed `ApplyError`, never a
   panic, never a repaired default).
4. INV-012/013 restated: mental/threshold events are real and meaningful events
   leave traces. INV-043: action validation is ordinary-agent validation against
   actor state — not proposer self-report. INV-020: replay rejects unsupported
   history loudly via the typed error channel; silent defaults invent history.
5. Deterministic-replay surface touched: new crossing events are deterministic
   functions of already-deterministic deltas (live and replay emit identically);
   converting panics/silent-skips to typed `ApplyError`s changes behavior only on
   corrupt history — the doctrine-required direction; removing silent defaults makes
   previously-tolerated malformed payloads reject. Per-field decision
   (required vs. documented-schema-optional) for the three defaulted fields is a
   spec-assigned implementer-recorded choice ("no silent third option") — each
   field's choice, and whether existing golden logs carry the field (builders appear
   to always set them; verify per field), recorded in the ticket closure and
   acceptance artifact. Golden repricing from crossing events is batched once.
6. Schema shape: no new event kinds and no payload-shape extension —
   `NeedThresholdCrossed` exists with its appliers/consumers; new emission sites
   are additive to logs (goldens reprice). If a per-field decision lands on
   `required`, that is a *tightening* of an existing field's optionality, not an
   extension; consumers are the apply arms this ticket touches. Additive-vs-breaking:
   additive event volume; field tightening breaking-only-for-corrupt-history.

## Architecture Check

1. A single shared "need delta + crossing" emitter (used by wait, sleep, work, and
   the scheduler window path) makes it impossible by construction for a builder to
   apply a band-changing delta without the paired crossing event — the family lock,
   not an instance fix — and handles both crossing directions, closing the
   `delta.max(0)` latent drop in the same stroke. Validating `continue_routine`
   against the authoritative intention record keyed by the actor's active intention
   removes the forgeable parameter rather than validating the forgery. Typed
   `ApplyError` variants keep corrupt-history handling in the error channel replay
   already owns.
2. No backwards-compatibility aliasing/shims: the proposer parameter is removed (or
   derived authoritatively), not validated-as-supplied; defaults are removed, not
   logged-and-kept.

## Verification Layers

1. INV-012/013 (crossing completeness) -> shared-emitter unit tests covering both
   directions; a source guard that `NeedDeltaApplied` construction outside the
   shared emitter fails; golden no-human day shows in-block crossings as events.
2. INV-043 (authoritative validation) -> rejection test: terminal authoritative
   intention with the parameter omitted → typed rejection from the autonomous path.
3. INV-020 (reject loudly) -> corrupt-history fixtures: duplicate need-tick charge,
   malformed `elapsed_ticks`, missing newly-required fields — each yields a typed
   `ApplyError` live AND on replay (no panic, no repair); `.expect` sites converted
   and proven by malformed-payload tests.
4. Repricing honesty -> per-actor ledger diff for repriced goldens in the acceptance
   artifact.

## What to Change

### 1. Shared crossing emitter (ORD-HARD-076)

Introduce the shared need-delta+crossing emitter; route `build_sleep_end_events`,
`build_work_completion_events`, `build_wait_events`/`build_threshold_events`, and the
scheduler window path through it; handle decreasing-pressure crossings.

### 2. Authoritative intention validation (ORD-HARD-077)

`build_continue_routine_event` validates against the actor's authoritative active
intention (via the pipeline's `AgentState` access); remove the proposer-supplied
`intention_status` parameter or derive it authoritatively; update the tests that
currently supply it.

### 3. Typed rejection posture (ORD-HARD-085)

`ApplyError` variants for duplicate need-tick charges and malformed `elapsed_ticks`;
apply the recorded per-field decision to the three defaulted fields; convert the six
log-sourced `.expect` sites (work.rs ×4, scheduler.rs ×2) to typed errors.

## Files to Touch

- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify)
- `crates/tracewake-core/src/actions/defs/work.rs` (modify)
- `crates/tracewake-core/src/actions/defs/wait.rs` (modify)
- `crates/tracewake-core/src/actions/defs/continue_routine.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify — completion-path expects)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — shared-emitter source guard)
- `crates/tracewake-content/` golden fixtures/expectations (modify — as surfaced by repricing; parent dir exists)

## Out of Scope

- The world-applier catch-all and derived guard lists (ticket 0021PHA3APOSREB-008) —
  coordinate the `events/apply.rs` mechanical merge.
- Census mechanics (`archive/tickets/0021PHA3APOSREB-003.md`).
- Window credit attribution (`ORD-HARD-088` — ticket 0021PHA3APOSREB-012) —
  coordinate the `scheduler.rs` merge.

## Acceptance Criteria

### Tests That Must Pass

1. Shared-emitter tests: band-up and band-down crossings emitted from wait, sleep
   completion, work completion, and the window path; source guard's synthetic
   (a builder applying a band-changing delta without the emitter) fails.
2. `continue_routine` rejection test with parameter omitted + terminal authoritative
   intention; acceptance test with active intention (control).
3. Corrupt-history fixtures (duplicate charge / malformed `elapsed_ticks` / missing
   required field / malformed completion payload) each produce the typed
   `ApplyError` live and via replay — process never aborts.
4. Per-field decisions recorded; goldens repriced once with per-actor ledger diff;
   existing no-human and replay suites green.
5. `cargo test --workspace` green.

### Invariants

1. No band-changing need delta lands without its paired crossing event, in either
   direction, from any builder.
2. No apply/completion path panics or silently repairs on log-derived data — corrupt
   history is a typed rejection.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/events/apply.rs` (mod tests) — typed-error and
   removed-default tests.
2. `crates/tracewake-core/src/actions/defs/` (mod tests per def) — crossing-emission
   and rejection tests.
3. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — corrupt-history
   replay rejection fixtures.
4. `crates/tracewake-core/tests/anti_regression_guards.rs` — shared-emitter source
   guard.

### Commands

1. `cargo test -p tracewake-core --test event_schema_replay_gates`
2. `cargo test -p tracewake-core need`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
