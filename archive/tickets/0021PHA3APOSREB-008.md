# 0021PHA3APOSREB-008: Events/state perimeter — applier totality, mutator deletion, derived guard lists, cause allowlist, divergence families

**Status**: DONE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`events/{apply,envelope}`, `state`, `replay/{rebuild,report}`, action-def builders); derived state-visibility guards
**Deps**: `specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-079, 080, 089, 094)

## Problem

Four perimeter gaps around events and state. (1) The world applier ends in
`_ => Ok(ApplyOutcome::NonWorldNoOp)` — a future World-stream kind added without an
apply arm silently no-ops in both live and replay, checksums agree, nothing fires
(`ORD-HARD-079`; latent INV-009/020; the agent/epistemic appliers fail closed). (2)
`PhysicalState::set_need_model` is a `pub` event-free mutator (zero callers) rewriting
checksummed state, and the guard_001 visibility/forbidden-write lists omit
`need_model`, `sleep_affordances`, `need_tick_charges`, and the episode maps
(`ORD-HARD-080`; INV-009). (3) `requires_cause` covers Phase-3A kinds only — 13
action-emitted kinds (`ActorMoved`, `ActorWaited`, `BeliefUpdated`,
`ContainerChecked`, `ContainerClosed`, `ContainerOpened`, `DoorClosed`, `DoorOpened`,
`ItemPlacedInContainer`, `ItemPlacedInPlace`, `ItemRemovedFromContainer`,
`ItemTakenFromPlace`, `ObservationRecorded`) are built with empty `causes`
(`ORD-HARD-089`; INV-010). (4) The replay divergence diff omits `sleep_affordances`
and `need_model` — a checksum divergence there is detected but unexplainable
(`ORD-HARD-094`; INV-018 diagnostics).

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: the `NonWorldNoOp` catch-all in
   `apply_event_with_capability` (`events/apply.rs`; all 12 current World kinds are
   explicitly handled — the gap is latent); `pub fn set_need_model` (`state.rs`) with
   zero workspace callers; guard_001 function bodies contain zero mentions of the
   four families; the `requires_cause` match (`events/envelope.rs`) ends at
   `NoHumanDayCompleted` with none of the 13 enumerated kinds;
   `EventEnvelope::new_v1` (causeless) at `movement.rs` and `wait.rs` (the waited
   event; the wait need-delta path already uses `new_caused_v1`);
   `diff_physical_state` (`replay/rebuild.rs`) and `classify_state_diff_family`
   (`replay/report.rs`) cover seven families, omitting the two; the physical
   checksum covers both (coverage is total and test-locked).
2. Verified against spec 0021 (reassessed 2026-06-11; operator-verified): findings
   ORD-HARD-079/080/089/094, with the 089 enumeration expanded at decomposition
   (13 kinds) and the per-kind disposition obligation: epistemic-stream kinds
   (`ObservationRecorded`, `BeliefUpdated`) may resolve as justified exclusions if
   their provenance model already carries ancestry — each kind's disposition is
   recorded, none silently skipped.
3. Cross-artifact boundary under audit: the event-perimeter contract — every
   World-stream kind has an explicit apply arm; every authoritative-state family is
   write-protected and guard-enumerated; every action-emitted event carries causal
   ancestry (cause or recorded exclusion); every checksum family is
   divergence-explainable.
4. INV-009/INV-010 restated: meaningful state changes require events with a cause
   model. INV-020: silent repair (a no-op for an unhandled mutating kind) is
   forbidden. INV-018: replay diagnostics must be able to explain what diverged.
5. Replay/perimeter surface touched: populating `causes` on the 13 kinds changes
   canonical envelope bytes → golden repricing, batched with the per-kind
   disposition pass; the applier-totality change affects only future/corrupt
   history (typed error instead of silent no-op); deleting the dead mutator and
   deriving guard lists change no behavior. All changes deterministic.
6. No schema shape change: `causes` is an existing envelope field being populated
   (populating an existing field is not a schema extension — additive-vs-breaking
   N/A); `requires_cause` is construction/application metadata.
7. Removal blast radius (`set_need_model`): zero callers workspace-wide (verified);
   removal touches only `state.rs` and the guard lists that should have named its
   family.

## Implementation Outcome (2026-06-11)

1. Closed the world/epistemic applier perimeter:
   `apply_event_with_capability` now rejects missing causes for cause-required
   world events and returns a typed `UnhandledWorldEventKind` instead of silently
   no-oping an unknown world kind. `apply_epistemic_event` now also rejects
   missing causes for cause-required epistemic events.
2. Removed the dead `PhysicalState::set_need_model` event-free mutator. Guard
   coverage now derives public-mutator and direct-insert checks from checksum
   family coverage, including `need_model`, `sleep_affordances`,
   `need_tick_charges`, and episode maps.
3. Populated cause ancestry for all 13 action-emitted kinds by extending
   `EventKind::requires_cause` and switching production builders/test fixtures to
   caused envelopes:
   - `ActorMoved`, `ActorWaited`, `ContainerChecked`, `ContainerClosed`,
     `ContainerOpened`, `DoorClosed`, `DoorOpened`,
     `ItemPlacedInContainer`, `ItemPlacedInPlace`,
     `ItemRemovedFromContainer`, and `ItemTakenFromPlace` are caused by their
     proposal in action builders.
   - `ObservationRecorded` is caused by the event or process that produced the
     observation: container-check event, item sound event, current-place
     perception process, or deterministic fixture helper process.
   - `BeliefUpdated` is caused by the observation that produced the derived
     belief in production sound handling; replay/test helpers now provide
     deterministic process causes when they synthesize epistemic events.
4. Added replay divergence coverage for `sleep_affordances` and `need_model` in
   `diff_physical_state` and `ReplayDivergenceFieldFamily`, with guards ensuring
   every physical checksum family has an explainable diff path.
5. Added/extended anti-regression guards for explicit physical-mutating apply
   arms, absence of the old world no-op catch-all, source-level cause
   disposition for action-emitted kinds, derived state write perimeter coverage,
   and replay diff-family coverage. Existing golden/content tests accepted the
   caused envelopes; no golden expectation files required changes.

## Verification (2026-06-11)

1. `cargo test -p tracewake-core events`
2. `cargo test -p tracewake-core replay`
3. `cargo test -p tracewake-core --test event_schema_replay_gates`
4. `cargo test -p tracewake-core actions::defs::movement`
5. `cargo test -p tracewake-core actions::defs::wait`
6. `cargo test -p tracewake-core actions::defs::openclose`
7. `cargo test -p tracewake-core actions::defs::checkcontainer`
8. `cargo test -p tracewake-core actions::defs::takeplace`
9. `cargo test -p tracewake-core --test anti_regression_guards`
10. `cargo test -p tracewake-core --test hidden_truth_gates`
11. `cargo test -p tracewake-core --lib`
12. `cargo fmt --all --check`
13. `cargo clippy --workspace --all-targets -- -D warnings`
14. `cargo build --workspace --all-targets --locked`
15. `cargo test --workspace`
16. `git diff --check`

## Architecture Check

1. Deriving the guard_001 lists and the divergence-diff family list from the same
   source the checksum-coverage guard already derives from (`state.rs` source /
   the coverage array) eliminates the hand-maintained-list class: a new state family
   joins write-protection and divergence diagnostics by construction — the
   guard-vacuity lesson applied to state perimeter. Making the world applier's
   catch-all explicit (known non-world kinds → no-op; anything else → typed error)
   converts a silent future failure into a loud one at the cheapest point.
2. No backwards-compatibility aliasing/shims: `set_need_model` is deleted (zero
   callers), not deprecated; the catch-all is restricted, not paralleled.

## Verification Layers

1. INV-009/020 (applier totality) -> guard deriving physical-mutating kinds from
   `EventKind` metadata and asserting an explicit arm per kind; synthetic new-kind
   negative fails.
2. INV-009 (write perimeter) -> derived guard_001 lists; synthetic public-mutator
   case fails; `set_need_model` grep-proof gone.
3. INV-010 (causal ancestry) -> per-kind: cause populated via `new_caused_v1` or a
   recorded justified exclusion; allowlist guard derived from action-emitted kind
   metadata so a new emitted kind cannot skip the decision.
4. INV-018 (divergence explainability) -> diff-family list derived from checksum
   coverage; synthetic divergence in `sleep_affordances`/`need_model` yields a
   populated `state_diff` with the correct `ReplayDivergenceFieldFamily` variant.

## What to Change

### 1. Applier totality (ORD-HARD-079)

Explicit non-world kind matching; typed error for unhandled kinds; metadata-derived
totality guard with synthetic.

### 2. State write perimeter (ORD-HARD-080)

Delete `set_need_model`; derive guard_001's visibility and forbidden-write lists from
the checksum-coverage derivation source; synthetic negatives.

### 3. Cause allowlist (ORD-HARD-089)

Extend `requires_cause` and switch builders to `new_caused_v1` per kind across the 13
enumerated kinds (movement, wait, openclose, container, item, takeplace, observation
builders — host defs as surfaced per kind); record per-kind dispositions (including
any epistemic-stream justified exclusions); reprice goldens once.

### 4. Divergence families (ORD-HARD-094)

Add `sleep_affordances`/`need_model` to `diff_physical_state` and
`ReplayDivergenceFieldFamily`; derive the family list from checksum coverage.

## Files to Touch

- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/events/envelope.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/src/replay/report.rs` (modify)
- `crates/tracewake-core/src/actions/defs/movement.rs` (modify)
- `crates/tracewake-core/src/actions/defs/wait.rs` (modify)
- `crates/tracewake-core/src/actions/defs/` other kind-host defs (modify — as surfaced per the 13-kind disposition pass: openclose, checkcontainer, takeplace, inspect; parent dir exists)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-content/` golden expectations (modify — as surfaced by cause repricing; parent dir exists)

## Out of Scope

- The reject-loudly payload posture inside apply arms (ticket 0021PHA3APOSREB-006) —
  coordinate the `events/apply.rs` mechanical merge.
- Census anchoring (`archive/tickets/0021PHA3APOSREB-003.md`).
- New event kinds or schema fields — this ticket populates existing structures.

## Acceptance Criteria

### Tests That Must Pass

1. Totality guard green with synthetic new-mutating-kind failure; explicit non-world
   no-op set pinned.
2. Derived guard_001 lists cover all checksum families (including the four
   previously omitted); synthetic public-mutator failure; `set_need_model` absent
   (grep-proof).
3. Each of the 13 kinds carries a cause or a recorded justified exclusion — one
   acceptance check per kind: `ActorMoved`, `ActorWaited`, `BeliefUpdated`,
   `ContainerChecked`, `ContainerClosed`, `ContainerOpened`, `DoorClosed`,
   `DoorOpened`, `ItemPlacedInContainer`, `ItemPlacedInPlace`,
   `ItemRemovedFromContainer`, `ItemTakenFromPlace`, `ObservationRecorded`; the
   derived allowlist guard fails on an undispositioned new emitted kind.
4. Synthetic `sleep_affordances`/`need_model` divergences produce populated,
   correctly-classified diffs.
5. Goldens repriced once with per-actor ledger diff; `cargo test --workspace` green.

### Invariants

1. Every physical-mutating event kind has an explicit apply arm; unhandled kinds are
   typed errors, never silent no-ops.
2. Every checksummed state family is write-protected, guard-enumerated, and
   divergence-explainable — all three derived from one source of truth.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — totality guard, derived
   guard_001 lists, derived cause-allowlist guard, with synthetics.
2. `crates/tracewake-core/src/replay/` (mod tests) — divergence-family tests.
3. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — caused-envelope
   replay coverage for repriced kinds.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-core replay`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
