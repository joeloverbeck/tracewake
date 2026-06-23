# 0048FOUCONHAR-006: Authoritative held-equal differential, generated mixed schedules, and reservation census

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — test-harness only: the non-vacuous held-equal human/no-human loaded-world differential, generated mixed-schedule extension to the deterministic generative harness, and the derived action-registry body-exclusive reservation census. No production code.
**Deps**: 003, 004

## Problem

Spec 0048 §4.7: the existing differential and parity evidence does not lock the claimed properties. `differential_human_wait_and_no_human_wait_match_authoritative_outcome` (`crates/tracewake-core/tests/world_step_coordinator.rs:418`) compares duration completion, needs, physical state/checksum, and controller/process origin — but the no-human side receives `Vec::new()` scheduled proposals (line 479), the test asserts `ordinary_pipeline_events == 0`, and no due world process is present, so it proves duration/accounting parity for an unpossessed *sleeping* actor only and cannot detect omission of autonomous actor transactions or world processes. §4.1's strongest guard is a held-equal differential with a possessed wait, ≥1 due unpossessed decision committing a typed event, an open duration terminating, passive need accounting for multiple actors, a due world process committing a typed event, and replay — asserting measured nonzero counts. §6.4 extends the deterministic generative harness with world-step schedules; §3.1 preserves the general body-exclusive reservation predicate, which a derived action-registry census locks against sibling-action omission.

This ticket lands the authoritative behavioral lock for the rebuilt step (§8 closure step 6, evidence): the differential that fails if the step omits the unpossessed-actor or world-process phase, plus the generative and reservation-census coverage.

## Assumption Reassessment (2026-06-23)

1. The vacuous differential is `differential_human_wait_and_no_human_wait_match_authoritative_outcome` (`crates/tracewake-core/tests/world_step_coordinator.rs:418`): it builds the no-human side via `advance_no_human(…, Vec::new())` (line 479) and asserts `ordinary_pipeline_events == 0`. `ActionRegistry::new()` is used in that test (line 468) — the registry the reservation census enumerates body-using actions from. The deterministic generative harness is `crates/tracewake-core/tests/generative_lock.rs` with support in `crates/tracewake-core/tests/support/generative.rs` (recorded-seed corpus, replay/tamper, reachability, omitted-population disclosure, duplicate need-charge-key check).
2. Spec 0048 §4.7/§4.1/§6.4 and §3.1 (preserved reservation holding) own this; §8 closure step 6. This ticket consumes the rebuilt one-tick transaction (ticket 003) through its final authority path (ticket 004) — the differential and schedules must exercise the *production* step, not a helper-only model (the spec's evidence-honesty check: the differential must fail when the coordinator omits the unpossessed actor or process phase).
3. Cross-artifact boundary under audit: the human/no-human equivalence contract spanning the possessed wait path (TUI→core, ticket 004), the no-human runner feeding proposals into the one-tick transaction (ticket 004), and the replay/temporal projection (ticket 001). The differential compares the typed event multiset/order (allowing only declared controller-origin differences), final physical/agent state, temporal frontier, projection frontiers, and checksums.
4. Constitutional invariants motivating this ticket: `INV-091` (no-human tests are mandatory), `INV-094` (possession parity is tested), `INV-108` (possession is cognition-neutral — the held-equal outcome must hold whether or not a human is bound). The differential is the executable witness these gates require.
5. Enforcement surface (deterministic-replay): the differential and the generated schedules exercise replay and temporal-frontier reconstruction (ticket 001). Confirm the comparison is deterministic — identical initial state + event log produce byte-identical final state/frontier/checksums across the human and no-human runs — and that the generated schedules record seeds and reachability as the existing harness does, introducing no nondeterministic input into the canonical comparison. Evidence-consumer basis: this ticket reads the replay/accounting enforcement surfaces; it adds no production logic.

## Architecture Check

1. A held-equal differential with all phases simultaneously live (possessed wait + due unpossessed decision + terminating duration + multi-actor accounting + due world process + replay) is the only witness that can detect silent omission of the actor or process phase — an empty autonomous-proposal list or a zero expected-ordinary-event count (the current test) cannot. Exercising the production step (not a helper-only model) is what makes the witness honest. The generated mixed schedules extend coverage across actor-opportunity / duration-boundary / process-due-time combinations the single fixture cannot. The derived action-registry census prevents sibling-action omission in the body-exclusive reservation check without hand-maintained per-action cases.
2. No backwards-compatibility aliasing/shims: this is test-only; it adds the authoritative differential and scopes the old one (ticket 007 re-labels the old differential as duration/accounting evidence). No production surface changes.

## Verification Layers

1. `INV-091`/`INV-094` no-human + parity -> replay/golden-fixture check: the held-equal differential asserts byte-identical final state/frontier/projection/checksums across the human and no-human runs (allowing only declared controller-origin event differences) and measured nonzero `actor_transactions_attempted` / `world_processes_applied`.
2. `INV-108` cognition-neutral -> replay/golden-fixture check (adversarial sub-cases): the differential fails when the coordinator omits the unpossessed-actor or world-process phase (asserted by deliberately-degraded variants that must not pass).
3. `INV-018` determinism + §3.1 reservation -> replay/golden-fixture check + schema validation: generated mixed schedules reproduce under recorded seeds; the derived census proves every ordinary body-using action (human + autonomous origin) is rejected while a body-exclusive duration is open, allowing only typed continuation/lifecycle controls.
4. Single-layer note N/A — three distinct invariants map to three distinct proof surfaces above.

## What to Change

### 1. The authoritative held-equal differential

In `crates/tracewake-core/tests/world_step_coordinator.rs`, add the non-vacuous differential: possessed actor one-tick wait; ≥1 unpossessed actor with a due ordinary decision committing a typed event; an open sleep/work duration due to terminate; passive need accounting for multiple actors; a due world process committing a typed event; replay from the same initial state and event log. Compare the typed event multiset/order (allowing only declared controller-origin differences), final physical state, final agent state, temporal frontier, projection frontiers, and checksums; assert measured nonzero actor/process counts. Add deliberately-degraded sub-variants (omit the unpossessed actor; omit the process) that must fail, proving the witness is non-vacuous.

### 2. Generated mixed schedules

Extend `crates/tracewake-core/tests/generative_lock.rs` and `crates/tracewake-core/tests/support/generative.rs` with world-step schedules varying actor opportunities, duration boundaries, and process due times, recording seeds and reachability as the existing harness does. Assert one marker per accepted increment, one charge per `(actor, need, tick)`, and live/replay equivalence including the temporal frontier.

### 3. Derived body-exclusive reservation census

Add `crates/tracewake-core/tests/reservation_body_exclusive_census.rs`: derive the set of ordinary body-using actions from `ActionRegistry` and assert each is rejected for both human and autonomous origins while a sleep/work duration is open, allowing only typed continuation/lifecycle controls. The enumerated action list is the quantifier expansion (one assertion per registry member; record any justified exclusion).

## Files to Touch

- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — authoritative differential + degraded must-fail variants; shared with tickets 001/003/004)
- `crates/tracewake-core/tests/generative_lock.rs` (modify — mixed world-step schedules)
- `crates/tracewake-core/tests/support/generative.rs` (modify — schedule generation support)
- `crates/tracewake-core/tests/reservation_body_exclusive_census.rs` (new — derived action-registry census)

## Out of Scope

- Any production code change — the step, frontier, interval, and salient surfaces are tickets 003/004/005 (this ticket exercises them).
- The TUI parity-runner measured outputs and adversarial scenario variants — ticket 007.
- Adding `proptest`/`quickcheck` — §6.4 rejects it for this pass; the deterministic harness is extended instead.
- Mutation campaigns — ticket 008.

## Acceptance Criteria

### Tests That Must Pass

1. The held-equal differential asserts byte-identical final state/frontier/projection/checksums across human and no-human runs with measured nonzero actor/process counts; its degraded variants (omit unpossessed actor / omit process) fail.
2. Generated mixed schedules reproduce under recorded seeds and satisfy one-marker-per-increment, one-charge-per-`(actor,need,tick)`, and live/replay-including-frontier equivalence.
3. The reservation census enumerates every `ActionRegistry` body-using action and asserts rejection (human + autonomous) while a body-exclusive duration is open; `cargo test -p tracewake-core` passes.

### Invariants

1. The differential is non-vacuous — it fails if the step omits the unpossessed-actor or world-process phase (`INV-091`/`INV-094`).
2. Generated schedules and the differential are deterministic under recorded seeds — no nondeterministic input enters the canonical comparison (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — authoritative held-equal differential + degraded must-fail variants.
2. `crates/tracewake-core/tests/generative_lock.rs` + `crates/tracewake-core/tests/support/generative.rs` (modify) — mixed world-step schedules.
3. `crates/tracewake-core/tests/reservation_body_exclusive_census.rs` (new) — derived action-registry body-exclusive census.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core --test generative_lock --test reservation_body_exclusive_census`
3. `cargo test -p tracewake-core` (full-crate harness build).

## Outcome

Completed: 2026-06-23

Implemented the ticket as test-harness coverage only, with no production-code changes:

1. Added `authoritative_loaded_world_differential_is_non_vacuous` in `crates/tracewake-core/tests/world_step_coordinator.rs`. The lock runs a real human-origin controlled wait against the no-human/scheduler-origin path with the same loaded world, nonzero due unpossessed actor work, and a due world-process event. It asserts identical final physical state, agent state, reconstructed frontier, replay final state/projection checksums, and degraded missing-actor / missing-process variants fail.
2. Extended the deterministic generative harness with `GeneratedWorldStepSchedule` metadata and `generated_world_step_schedules_replay_frontier_and_charge_once`, covering actor-opportunity, duration-boundary, and process-due schedule classes under recorded seeds while retaining replay and one-charge-per-key checks.
3. Added `crates/tracewake-core/tests/reservation_body_exclusive_census.rs`, deriving the current body-using registry action list as `eat`, `move`, `sleep`, `wait`, and `work_block`, and asserting reservation-conflict rejection for human, scheduler, and agent origins while an open body-exclusive sleep duration exists.

Deviations from original plan:

- The held-equal differential compares replay temporal diagnostics for equality across human/no-human runs rather than asserting the current due-actor wait path has no temporal diagnostics; the asserted frontier, final state, projection, and checksums remain byte-identical across the held-equal pair.

Verification:

1. `cargo test -p tracewake-core --test world_step_coordinator` — passed.
2. `cargo test -p tracewake-core --test generative_lock --test reservation_body_exclusive_census` — passed.
3. `cargo test -p tracewake-core` — passed.
4. `cargo clippy -p tracewake-core --all-targets -- -D warnings` — passed.
5. `cargo fmt --all --check` — passed.
6. `git diff --check` — passed.
