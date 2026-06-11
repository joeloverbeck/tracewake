# 0018PHA3APROWIT-003: Episode payload materialization, tamper gates, and derived allowlist census

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`state`, `events/apply`, `checksum`); tamper tests and golden checksum updates in `tracewake-content`; lock-layer census rewrite
**Deps**: `archive/tickets/0018PHA3APROWIT-002.md` (replays the corrected duration payloads; spec §8 ordering); `archive/specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (ORD-HARD-036)

## Problem

`state.rs::OrdinaryLifeEpisodeRecord` carries only `summary: String` (= `effects_summary`), unlike its siblings `CandidateGoalEvaluationRecord`/`ContinueRoutineArbitrationRecord`, which store full `payload_fields`. The episode build sites set fixed summaries while their payloads carry `duration_ticks`, `output_tag`, `workplace_id`, `body_exclusive`, `non_economic_output`, `sleep_place_id`, and failure `blocker_kind`/`completion_emitted` — none of which enter `summary`, the canonical checksum line (`ordinary_life_episode|…|summary={}` in `checksum.rs`), or any rebuild path. A stored log whose `WorkBlockStarted.output_tag` or `duration_ticks` is rewritten passes `run_replay::matches_expected` — semantically tamperable history (INV-018, INV-105). Separately, the allowlist census (`anti_regression_guards.rs::agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state`) "cites" each entry's coverage via an in-test literal `match` (`_ => ""`) that no code verifies against a real checksum family.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `a9c62e0`: `OrdinaryLifeEpisodeRecord { event_id, event_kind, actor_id, proposal_id, caused_event_ids, sim_tick, summary }` — no `payload_fields`; `apply.rs::payload_fields(event)` helper exists and already feeds the two sibling record types; `checksum.rs` emits the episode line without a payload component and `AGENT_STATE_CHECKSUM_COVERAGE` carries the `ordinary_life_episode` family; the allowlist census hardcodes citations (`EventKind::FoodConsumed => "dual_stream_physical_food_supply_checksum"`, `_ => ""`).
2. Spec 0018 ORD-HARD-036 (required correction + structural lock); the 0017 ORD-HARD-028 pattern, completed for the episode family that pass did not reach.
3. Cross-artifact boundary under audit: the agent-state checksum coverage contract (`AGENT_STATE_CHECKSUM_COVERAGE` + `checksum_coverage_is_total_for_authoritative_state`) between materialized records (`state.rs`), their apply arms (`events/apply.rs`), and the canonical checksum lines (`checksum.rs`).
4. INV-018/INV-105 restated: replay must be able to detect tampered history, not merely re-apply it; an event class whose semantic content can be altered without any gate failing is decorative history, and diagnostics/evidence must be structurally inspectable.
5. Deterministic-replay enforcement surface touched: extending the episode canonical line changes agent checksums, so golden expectations reprice once (behind 002's window); tamper-flip gates make `!agent_checksum_matches && !matches_expected` provable for episode payload rewrites. The change strengthens replay evidence; no replay/hash *semantics* change (the canonical-line format gains a field, mirroring the arbitration-record precedent), so no foundational-doc amendment is triggered.
6. Schema extension: `OrdinaryLifeEpisodeRecord` gains `payload_fields: Vec<(String, String)>` — additive to the record shape. Consumers: `apply.rs` (the seven episode arms populate it via the existing `payload_fields(event)` helper), `checksum.rs` (the canonical line emits `payload={}` with the existing `join_pairs` length-prefixing), the capstone/golden gates (reprice), and the rewritten allowlist census (reads the coverage registry instead of literals).

## Architecture Check

1. Reusing the sibling-record pattern (`payload_fields` + `join_pairs` canonical emission) makes episode evidence consistent with the arbitration/candidate records rather than inventing a parallel mechanism; the record-struct census then enforces the pattern for every future materialized record. Deriving allowlist citations from the coverage/dual-stream registries converts a hand-typed claim into a checked one — a new allowlist entry must point at a real checksum family or a registered payload-free marker schema, or the census fails.
2. No backwards-compatibility aliasing/shims: no summary-only fallback emission; the literal-citation `match` in the census is replaced, not kept alongside the registry lookup.

## Verification Layers

1. INV-018 tamper detection -> new tamper tests in `golden_fixtures_run.rs`: rewriting `WorkBlockStarted.output_tag` and a `SleepInterrupted` proration field in a copied golden log each yield `!agent_checksum_matches` and `!matches_expected`.
2. INV-105 evidence completeness -> grep-proof + census: every materialized agent record type whose source events carry payload stores `payload_fields` (record-struct census parsing `state.rs`, the `checksum_coverage_is_total_for_authoritative_state` pattern).
3. Lock durability (derived citations) -> rewritten allowlist census: each allowlisted non-marker kind resolves to a checksum-coverage/dual-stream registry entry; each marker kind has a registered payload-free schema; a literal-string citation path no longer exists.
4. INV-018 byte stability -> all golden fixtures replay byte-identically at the repriced checksums; the capstone's `decision_context_hash_failures` assertion stays green.

## What to Change

### 1. Materialize episode payloads

`state.rs::OrdinaryLifeEpisodeRecord` gains `payload_fields`; the seven episode apply arms in `apply.rs` (`SleepStarted`, `SleepCompleted`, `SleepInterrupted`, `WorkBlockStarted`, `WorkBlockCompleted`, `WorkBlockFailed`, `EatFailed`) populate it via `payload_fields(event)`.

### 2. Checksum the payloads

`checksum.rs`: the `ordinary_life_episode` canonical line emits `payload={}` (length-prefixed via `join_pairs`), mirroring the arbitration-record line.

### 3. Tamper-flip gates

`golden_fixtures_run.rs`: two new tamper tests (rewritten `output_tag`; rewritten `SleepInterrupted` proration field), both asserting replay poisoning.

### 4. Derived allowlist census + record-struct census

`anti_regression_guards.rs`: rewrite the allowlist census to resolve citations from the coverage/dual-stream registries; add a census asserting every materialized agent record type with payload-bearing source events stores `payload_fields`.

## Files to Touch

- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/checksum.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)

## Out of Scope

- Payload schema-version stamping/gating (ticket `0018PHA3APROWIT-004`).
- Shrinking `AGENT_WORLD_NOOP_ALLOWLIST` further — its three entries (`FoodConsumed`, `NoHumanDayStarted/Completed`) remain; only the census's citation mechanism changes.
- Generative-tier tamper coverage (ticket `-008`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content episode_tamper` — both tamper rewrites poison replay (`!agent_checksum_matches`, `!matches_expected`).
2. `cargo test -p tracewake-core agent_world_noop_allowlist` — derived-citation census green; literal-citation path gone.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every materialized agent record type whose source events carry non-trivial payload stores and checksums `payload_fields`.
2. Every `AGENT_WORLD_NOOP_ALLOWLIST` entry cites machine-verified coverage: a registry-resolved checksum family or a registered payload-free marker schema.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — `episode_tamper_output_tag_poisons_replay`, `episode_tamper_proration_poisons_replay`.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — derived allowlist census; materialized-record `payload_fields` census.
3. `crates/tracewake-core/src/checksum.rs` (unit test) — episode canonical line includes the payload component.

### Commands

1. `cargo test -p tracewake-content episode_tamper`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-11

What changed:

- `OrdinaryLifeEpisodeRecord` now stores source event `payload_fields`, matching the
  candidate-goal and continue-routine materialized record pattern.
- Ordinary-life episode canonical checksum lines now include caused event ids and
  length-prefixed payload fields.
- Episode apply arms populate payload fields from the committed event payload.
- Added replay tamper gates proving `WorkBlockStarted.output_tag` and
  `SleepInterrupted.fatigue_delta` rewrites poison replay and agent checksums.
- Reworked the noop allowlist guard so `FoodConsumed` resolves through the physical
  `food_supply` checksum family and no-human markers are explicitly registered as
  payload-free.
- Added a materialized-record census requiring payload-bearing agent records to retain
  `payload_fields`.

Deviations:

- No golden checksum or context-hash expectation file needed updating; the full golden
  fixture suite stayed green with the stronger checksum materialization.

Verification:

- `cargo fmt --all --check` — passed.
- `cargo test -p tracewake-content episode_tamper` — passed.
- `cargo test -p tracewake-core agent_world_noop_allowlist` — passed.
- `cargo test -p tracewake-core materialized_agent_payload_records_keep_payload_fields` —
  passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
