# 0016PHA3ANEEACC-011: Replay robustness — ordering verification, WorldNoOp census, poisoned rebuild

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` log-ordering verification, agent-event allowlist census, episode-state materialization into `AgentState` (checksum registry growth), fail-fast rebuild, schema-upcast fixture
**Deps**: `archive/tickets/0016PHA3ANEEACC-002.md`, `archive/tickets/0016PHA3ANEEACC-003.md`

## Problem

ORD-HARD-024: replay robustness gaps — trusted ordering, unverifiable lifecycle kinds, error accumulation (INV-018, INV-020; foundation doc 03 — reject unsupported history, replay failure is loud; archived 0005 §15 — replay must reconstruct lifecycle state, and a replay that silently drops a Phase 3A event class is a failure):

- `replay/rebuild.rs::rebuild_projection` iterates stored `Vec` order and never validates `global_order` monotonicity; `EventLog` deserialization re-appends events, *reassigning* `global_order`/`stream_position` from load order — a reordered serialized log is undetectable by construction.
- `events/apply.rs::apply_agent_event` returns `Ok(WorldNoOp)` for `NeedThresholdCrossed`, `CandidateGoalsEvaluated`, `SleepStarted/Completed/Interrupted`, `FoodConsumed`, `FoodServiceUsed`, `EatFailed`, `WorkBlockStarted/Completed/Failed`, `ContinueRoutine{Proposed,Accepted,Rejected}`, `NoHumanDay{Started,Completed}`. Excepting `FoodConsumed` (world-applied on the physical stream and captured by the physical checksum — a dual-stream kind), these kinds mutate no rebuilt projection and sit outside both checksums; their semantic content is unverifiable under replay except via decision-trace strings.
- On agent/epistemic application errors, `rebuild_projection` accumulates and continues, producing a usable `final_agent_state`/`final_checksum` from a partial replay; the capstone institutionalizes filtered "marker" errors.
- `EVENT_SCHEMA_REGISTRY` has a single V1 entry with `CurrentNoMigrationRequired`; the migration/loud-failure branch is vacuously untested.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `rebuild_projection` (`replay/rebuild.rs:45–154`) iterates `log.events()` (:66) without monotonicity checks and accumulates application errors (:107–123) while still emitting `final_agent_state` (:143) and `final_checksum` (:126); `EventLog::deserialize_canonical` re-appends with reassignment (`events/log.rs:56–70`; `append` sets `global_order = self.events.len()` at :28); the `WorldNoOp` arm at `events/apply.rs:338–353` (with `FoodConsumed` world-applied at :167); capstone filter `non_marker_agent_errors` (`tests/no_human_capstone.rs:109–122`); `EVENT_SCHEMA_REGISTRY` single V1 + `CurrentNoMigrationRequired` (`events/envelope.rs:27–30`); two checksums — `PhysicalChecksum` and `AgentStateChecksum` (`checksum.rs:272`); `AgentState` defined at `state.rs:140`; the checksum census test `new_authoritative_field_without_checksum_registry_fails` (`anti_regression_guards.rs:2014–2042`) forces registration of the new episode fields.
2. Spec/docs: spec 0016 §ORD-HARD-024 (evidence incl. the dual-stream `FoodConsumed` parenthetical, required correction, structural lock); `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`; `archive/specs/0005_*` §15 (:1161) / §15.1 (:1165); `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-018, INV-020.
3. Shared boundary under audit: the serialized-log → rebuilt-projection trust boundary. After this ticket, stored ordering is verified (never reassigned), every agent event kind either mutates a checksummed projection or sits on an explicit allowlist citing its capturing surface, and the first application error poisons the rebuild.
4. INV-018 — deterministic replay reconstructs significant outcomes; a rebuild that builds past errors fabricates authority. INV-020 — schema evolution must let replay reject unsupported history loudly. Restated before trusting the ticket narrative.
5. Deterministic-replay enforcement surface (this ticket hardens it): (a) ordering verification makes reordered logs loudly detectable; (b) materializing sleep/work episode and threshold-crossing state into `AgentState` (per 0005 §15.1) moves those kinds inside `AgentStateChecksum` — goldens reprice, diffs ledger-explained; (c) poisoning on first agent/epistemic error makes `matches_expected` fail and the report non-authoritative — the capstone's marker-filter institutionalization is removed; (d) the V0→V1 upcast fixture proves the loud-failure migration branch. Consumes ticket 010's typed payload errors (a corrupt payload now surfaces as a typed application error the poisoning can act on) and coexists with ticket 003's frontier gate in `rebuild.rs`.
6. Schema extension: `AgentState` gains episode/threshold-crossing state (consumers: `checksum.rs` registry — forced by the census test — `debug_reports.rs` as surfaced, and the apply path); additive fields, registered in the checksum census. The allowlist census itself is a new test-layer contract whose entries cite capturing projections (dual-stream `FoodConsumed` cites the physical checksum).
7. Adjacent contradictions: removing the capstone's `non_marker_agent_errors` filter changes the capstone's tolerance contract — a required consequence of loud-failure doctrine, cited here as the change rationale (no silent retcon).

## Architecture Check

1. Verifying stored ordering (instead of trusting reassignment) converts an undetectable corruption class into a loud failure at deserialization — the cheapest place to catch it. The allowlist census makes "this kind is a no-op" a *reviewed claim with a named capturing surface* instead of an accident of the match arm, and shrinking it by materializing episode state follows 0005 §15.1's reconstruction mandate rather than widening trace-string forensics. Fail-fast poisoning is the only honest semantics for an authoritative replay artifact: a partial rebuild that compares checksums anyway is a decorative proof surface.
2. No backwards-compatibility aliasing/shims: deserialization verifies stored ordering rather than keeping reassignment as a fallback; the capstone marker-filter is deleted, not parameterized.

## Verification Layers

1. INV-018 (ordering) → gate `reordered_log_is_detected_001`: a reordered serialized log fails loudly at deserialization/rebuild.
2. INV-018 (loud partial-replay failure) → gate `corrupt_midstream_agent_event_poisons_rebuild_001`: first application error poisons the rebuild; `matches_expected` fails; no authoritative final checksum.
3. 0005 §15.1 (lifecycle reconstruction) → allowlist census test: no agent `EventKind` applies as `WorldNoOp` unless named in the explicit allowlist with its capturing projection/checksum cited; episode/threshold state present in `AgentStateChecksum`.
4. INV-020 (loud migration) → synthetic V0→V1 upcast fixture: unsupported history is rejected loudly, never silently repaired.

## What to Change

### 1. Ordering verification

`rebuild_projection` asserts stored `global_order` monotonicity; `EventLog::deserialize_canonical` verifies (not reassigns) stored `global_order`/`stream_position`, failing loudly on mismatch.

### 2. WorldNoOp allowlist census + episode-state materialization

A registry test asserting no agent `EventKind` applies as `WorldNoOp` unless allowlisted with a citation of the projection/checksum capturing its effect (`FoodConsumed` cited as dual-stream/physical-checksum). Shrink the allowlist by materializing sleep/work episode and threshold-crossing state into `AgentState` (new fields registered in the checksum census), with apply arms replacing the no-ops for those kinds.

### 3. Fail-fast rebuild

On the first agent/epistemic application error, mark the report non-authoritative and fail `matches_expected` rather than building past it; remove the capstone's `non_marker_agent_errors` institutionalization.

### 4. Migration loud-failure fixture

A synthetic V0→V1 upcast fixture locking the `EVENT_SCHEMA_REGISTRY` migration path's loud-failure contract.

## Files to Touch

- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/src/events/log.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/events/envelope.rs` (modify — registry/upcast surface)
- `crates/tracewake-core/src/state.rs` (modify — episode/threshold state on `AgentState`)
- `crates/tracewake-core/src/checksum.rs` (modify — registry entries)
- `crates/tracewake-core/src/debug_reports.rs` (modify — episode-state surfacing, as surfaced)
- `crates/tracewake-core/tests/no_human_capstone.rs` (modify — drop marker-filter)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — allowlist census home, as surfaced)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — reordered/corrupt-log gates; checksum diffs explained)

## Out of Scope

- The per-decision context-hash re-derivation gate (`archive/tickets/0016PHA3ANEEACC-003.md` — shares `rebuild.rs`; this ticket Deps it).
- Typed payload errors in completion builders (0016PHA3ANEEACC-010 — consumed here).
- Workspace census / mutants baseline (0016PHA3ANEEACC-012).

## Acceptance Criteria

### Tests That Must Pass

1. `reordered_log_is_detected_001` and `corrupt_midstream_agent_event_poisons_rebuild_001` fail loudly in exactly the contracted way (detection at deserialization; poisoning at first error).
2. Allowlist census: every `WorldNoOp` agent kind is allowlisted with a cited capturing surface; sleep/work episode and threshold-crossing state are inside `AgentStateChecksum`; the upcast fixture proves loud migration failure.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — repriced goldens ledger-explained.

### Invariants

1. Stored event ordering is verified at every deserialization — a reordered log can never rebuild silently.
2. A rebuild past any agent/epistemic application error is non-authoritative and fails comparison — no partial replay masquerades as proof.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — `reordered_log_is_detected_001`, `corrupt_midstream_agent_event_poisons_rebuild_001`.
2. The `WorldNoOp` allowlist census test (anti_regression_guards.rs or the events test module — single home, no duplication).
3. The V0→V1 upcast fixture against `EVENT_SCHEMA_REGISTRY`.

### Commands

1. `cargo test -p tracewake-core replay && cargo test -p tracewake-core events && cargo test -p tracewake-content golden`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Implemented replay robustness hardening for ORD-HARD-024:

- `EventLog::deserialize_canonical` now verifies stored `global_order` and per-stream positions instead of reassigning them; `rebuild_projection` also rejects in-memory logs whose stored order is inconsistent.
- `rebuild_projection` stops at the first world, agent, or epistemic application error, leaving the replay report non-authoritative through existing error fields and `matches_expected` semantics.
- Agent replay now materializes need-threshold crossings and sleep/eat/work episode events into checksummed `AgentState` projections, shrinking the remaining `WorldNoOp` surface to an explicit allowlist.
- Added focused locks for reordered serialized logs, reordered in-memory logs, corrupt midstream agent events, materialized agent episode state, and the explicit no-op allowlist. Existing unsupported-schema tests continue to cover loud migration/unsupported-history behavior.

Verification passed:

1. `cargo test -p tracewake-core replay && cargo test -p tracewake-core events && cargo test -p tracewake-content golden`
2. `cargo fmt --all --check`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo build --workspace --all-targets --locked`
5. `cargo test --workspace`
