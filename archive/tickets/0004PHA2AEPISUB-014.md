# 0004PHA2AEPISUB-014: Sound observation slice with bounded uncertainty

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — adds the low-confidence sound-observation slice (runtime/seeded) to the `tracewake-core` epistemic path and `tracewake-content` fixture.
**Deps**: 0004PHA2AEPISUB-007, 0004PHA2AEPISUB-013

## Problem

Phase 2A includes the minimum sound-observation slice: Elena may receive a low-confidence sound observation near Tomas's room/strongbox, which must not become knowledge of theft and may support only uncertain propositions (`SoundHeardNearPlace`, `PossibleMovementNearPlace`) (Spec 0004 §2, §9.7, §11.4, §14.6). Evidence is not truth (`INV-030`); a sound is a channel with distortion and alternatives, never a culprit conclusion.

## Assumption Reassessment (2026-06-06)

1. The `Observation` record (ticket 002) already carries `confidence`, `alternatives`, and channel `simple_sound`; the `Proposition` enum (ticket 001) already has `SoundHeardNearPlace` and `PossibleMovementNearPlace`. No runtime sound generation exists yet, and `sound_uncertainty_001` (ticket 013) provides placement/seed only.
2. The sound payload shape (intensity/duration/material-hint, low confidence, alternatives `house_settling`/`person_moving`/`object_shift`/`misheard_sound`) and the forbidden belief updates are fixed by Spec 0004 §9.7/§14.6. Spec §11.4 prefers runtime observation from Mara's action over a seed.
3. Shared boundary under audit: runtime generation hooks into the physical action outcome (Mara moving/taking near Elena) to emit an `ObservationRecorded` with channel `simple_sound`; it consumes the event/projection path (tickets 004/005) and the belief-update path (ticket 007) for the optional low-confidence belief.
4. Invariant motivating this ticket: `INV-030` (evidence is not truth — a sound supports claims with provenance/confidence, not ground truth), `INV-016` (absence/perception via channel), and `INV-024` (no telepathy — Elena learns only what the sound channel carries).
5. Actor-knowledge / no-leak surface: the sound observation must yield at most a low-confidence `SoundHeardNearPlace`/`PossibleMovementNearPlace` belief for Elena and must NOT produce `Mara stole coin_stack_01`, `Mara was in Tomas's room`, or `coin_stack_01 was stolen` (the §9.7 forbidden set), and must not create `ActorWasNearPlace(actor_mara, …)` absent a modeled channel. It introduces no leakage (Elena's belief names no culprit) and no nondeterminism (the runtime observation is a deterministic function of Mara's action + Elena's placement).

## Architecture Check

1. Generating the sound observation as a deterministic side-event of Mara's physical action (rather than a hand-authored "Elena suspects" seed) keeps the observation causal and replayable, and keeps interpretation out of perception — the channel records a low-confidence sound with alternatives, and any belief derived is `plausible`/low-confidence only. A seeded authored-prehistory sound observation is the allowed fallback if runtime generation is too costly (§11.4), still channel/source-backed.
2. No backwards-compatibility shims: the sound hook is additive on the physical action outcome; it creates no new mutation path.

## Verification Layers

1. Evidence ≠ truth (`INV-030`) -> unit + golden test: Elena's sound observation carries low confidence and ≥2 alternatives; any derived belief is `SoundHeardNearPlace`/`PossibleMovementNearPlace` at low confidence, never a theft/culprit proposition.
2. No telepathy (`INV-024`) -> leak test: Elena's notebook after the sound contains no `Mara`/`stole`/`culprit` string and no `ActorWasNearPlace(actor_mara, …)`.
3. Deterministic generation (`INV-018`) -> replay/golden-fixture check: the runtime sound observation reproduces identically on replay of Mara's action.

## What to Change

### 1. Runtime sound observation

Add a deterministic hook in the physical movement/take outcome path so that when Mara acts near a placed observer (Elena), an `ObservationRecorded` with channel `simple_sound` is emitted carrying the §9.7 payload (intensity/duration/material-hint, low confidence, alternatives). Prefer this runtime path; fall back to a seeded authored-prehistory sound observation only if needed.

### 2. Optional uncertain belief

Allow the sound observation to support an optional low-confidence `SoundHeardNearPlace` and/or `PossibleMovementNearPlace` belief for Elena via the belief-update path (ticket 007), with stance `plausible`/low confidence. Forbid any theft/culprit/`ActorWasNearPlace(mara)` belief.

### 3. Fixture wiring

Wire `sound_uncertainty_001` (ticket 013) to place Elena within sound range and exercise the slice (runtime-generated preferred).

## Files to Touch

- `crates/tracewake-core/src/actions/defs/takeplace.rs` (modify — emit deterministic sound observation on qualifying action near an observer)
- `crates/tracewake-core/src/epistemics/observation.rs` (modify — sound-payload helper if needed; file created by 0004PHA2AEPISUB-002)
- `crates/tracewake-content/src/fixtures/sound_uncertainty_001.rs` (modify — Elena sound-range placement; file created by 0004PHA2AEPISUB-013)

## Out of Scope

- Any theft/culprit inference from sound (forbidden — later phases only with modeled sources).
- Gossip/rumor propagation of the sound (Phase 4 non-goal).
- TUI rendering of Elena's notebook (ticket 011 already covers the renderer; assertions in ticket 016).

## Acceptance Criteria

### Tests That Must Pass

1. Mara taking the coin near Elena produces a `simple_sound` observation for Elena with low confidence and ≥2 alternatives.
2. Elena's resulting belief (if any) is `SoundHeardNearPlace`/`PossibleMovementNearPlace` at low confidence; no theft/culprit belief exists.
3. Elena's notebook contains no `Mara`/`stole`/`culprit` substring.

### Invariants

1. A sound observation is channel/source-backed evidence, never ground truth (`INV-030`).
2. The sound slice creates no culprit knowledge and no `ActorWasNearPlace(actor_mara, …)` without a modeled channel.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/golden_scenarios.rs` (extend) — `sound_uncertainty_001`: low-confidence sound, no theft knowledge.
2. `crates/tracewake-core/src/actions/defs/takeplace.rs` (unit tests) — deterministic sound emission near observer.

### Commands

1. `cargo test -p tracewake-core --test golden_scenarios`
2. `cargo test -p tracewake-core actions::defs::takeplace`
3. `cargo build --workspace --all-targets --locked`

## Outcome

Completed on 2026-06-07.

Changed:
- Added deterministic low-confidence simple-sound observation generation for qualifying Mara take/place actions near Elena.
- Added an optional plausible `SoundHeardNearPlace` belief update sourced from the sound observation, with no culprit/theft/`ActorWasNearPlace(actor_mara, ...)` inference.
- Extended epistemic observation application to support place-targeted sound observations and alternatives.
- Added unit and golden-scenario coverage for low confidence, alternatives, deterministic projection updates, and Elena notebook no-leak strings.

Deviations:
- None.

Verification:
- `cargo test -p tracewake-core --test golden_scenarios`
- `cargo test -p tracewake-core actions::defs::takeplace`
- `cargo build --workspace --all-targets --locked`
- `cargo fmt --all --check`
- `git diff --check`
