# 0018PHA3APROWIT-004: Payload schema-version gates on materialized agent kinds

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`events/apply`, event producers for the materialized agent kinds); new replay gate; apply-arm census; golden checksum updates
**Deps**: `archive/tickets/0018PHA3APROWIT-003.md` (versions stamp the payloads 002/003 finalized; spec §8 ordering); `specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (ORD-HARD-039)

## Problem

The `apply.rs` arms for `CandidateGoalsEvaluated`, `ContinueRoutine{Proposed,Accepted,Rejected}`, and the episode group (`SleepStarted` … `EatFailed`) insert materialized records without any `require_payload_version`-style check — `apply.rs` contains exactly two such call sites plus the helper's definition, covering only the `DecisionTraceRecorded`/`StuckDiagnosticRecorded` family (`trace_schema_version`/`diagnostic_schema_version`). A future payload-shape change to the unversioned agent kinds cannot be loud-rejected; replay would silently absorb it (INV-020: payloads must be versioned enough that replay rejects unsupported history rather than silently inventing repairs). The envelope-level `EVENT_SCHEMA_REGISTRY` gates the envelope version only. Producer asymmetry corroborates the gap: `work.rs::need_delta_event` stamps a `schema_version` payload field while `sleep.rs::need_delta_event` does not.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `a9c62e0`: `require_payload_version` call sites at the `DecisionTraceRecorded`/`StuckDiagnosticRecorded` arms only; the eleven materialized agent kinds' arms insert records unversioned; `EVENT_SCHEMA_V1` is the envelope version constant; `ContinueRoutine*` events are produced by `actions/defs/continue_routine.rs`; the `CandidateGoalsEvaluated` emitter does not appear under that token in `crates/tracewake-core/src` outside `envelope.rs`/`apply.rs`, so the full producer set is an implementation-discovered surface (locate emitters by `EventKind` construction at implementation time).
2. Spec 0018 ORD-HARD-039 (required correction + structural lock), grouped with 036 per spec §8 ordering ("replay evidence over the corrected payloads").
3. Cross-artifact boundary under audit: the event-schema-evolution contract (INV-020; `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`) at the payload tier — producers stamp, apply arms require, replay loud-rejects.
4. INV-020 restated: event kinds and payloads must be versioned enough that replay can reject unsupported history rather than silently inventing repairs.
5. Deterministic-replay surface touched: stamping a `payload_schema_version` field changes event bytes for the affected kinds, so golden checksums reprice once (inside the 002/003 re-baseline window); the forged-version gate makes the loud-rejection path provable. Fail-closed direction only — no existing version semantics weaken, no replay-semantics change, no foundational-doc amendment triggered (mirrors the existing trace/diagnostic version gates).
6. Schema extension: the materialized agent kinds' payloads gain `payload_schema_version` — additive (new payload field; required at apply for newly-written history; stored golden logs are regenerated in-repo, so no migration shim is needed — consistent with the repo's no-shims rule). Consumers: the apply arms (now requiring the field), `event_schema_replay_gates.rs` (forged-version rejection), and golden expectations (reprice).

## Architecture Check

1. Mirroring the existing `trace_schema_version`/`diagnostic_schema_version` mechanism (same helper, same loud `ApplyError`) is cleaner than inventing a parallel payload-version registry: one rejection path, already replay-wired and poison-tested. The apply-arm census then makes version-gating the default for every future materialized kind instead of an opt-in.
2. No backwards-compatibility aliasing/shims: no "missing version ⇒ assume V1" defaulting — a missing or unsupported payload version is a typed apply/replay error; golden logs are regenerated rather than grandfathered.

## Verification Layers

1. INV-020 loud rejection -> new gate in `event_schema_replay_gates.rs`: a copied golden log with a forged `payload_schema_version` on a materialized agent kind fails replay with the typed error (and live apply rejects equally).
2. Census completeness -> new census in `anti_regression_guards.rs`: every materialized agent `EventKind`'s apply arm calls the version-requiring helper (parse the apply-arm table; the existing census pattern), so an unversioned new kind fails CI.
3. INV-018 byte stability -> golden fixtures replay byte-identically at the repriced checksums; capstone stays green.
4. Producer completeness -> grep-proof: every producer of the eleven kinds stamps the field (asserted indirectly by the apply-side requirement — any unstamped emission fails the canonical-day fixtures at apply time).

## What to Change

### 1. Stamp the field at every producer

Producers of `CandidateGoalsEvaluated`, `ContinueRoutine{Proposed,Accepted,Rejected}`, and the episode group add `PayloadField::new("payload_schema_version", "1")` (named producers: `actions/defs/continue_routine.rs`, `actions/defs/sleep.rs`, `actions/defs/work.rs`, `actions/defs/eat.rs`; remaining emitters are an implementation-discovered set under `crates/tracewake-core/src/` — locate by `EventKind` construction).

### 2. Require at apply

The eleven apply arms call `require_payload_version(&payload, "payload_schema_version", "1")` before inserting their records.

### 3. Forged-version replay gate + census

`event_schema_replay_gates.rs` gains the forged-version poisoning test; `anti_regression_guards.rs` gains the apply-arm version-census.

### 4. Golden updates

Regenerate/reprice golden expectations changed by the payload addition.

## Files to Touch

- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/actions/defs/continue_routine.rs` (modify)
- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify)
- `crates/tracewake-core/src/actions/defs/work.rs` (modify)
- `crates/tracewake-core/src/actions/defs/eat.rs` (modify — as surfaced)
- `crates/tracewake-core/src/` (modify — implementation-discovered emitter set for `CandidateGoalsEvaluated`; parent verified)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — expectations, as surfaced)

## Out of Scope

- Envelope-level schema versioning (`EVENT_SCHEMA_REGISTRY`) — unchanged; this ticket adds the payload tier only.
- Episode payload materialization (ticket `0018PHA3APROWIT-003`, a dependency).
- Versioning physical/world event payloads — the finding and spec scope the materialized *agent* kinds.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core payload_schema_version` — forged version loud-rejects at apply and replay.
2. The apply-arm version census in `anti_regression_guards.rs` — every materialized agent kind gated.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No materialized agent record is inserted from an unversioned or unsupported-version payload; rejection is typed and poisons rebuild.
2. Every future materialized agent kind must pass the version census before CI is green.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — forged `payload_schema_version` poisons replay.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — apply-arm version census.

### Commands

1. `cargo test -p tracewake-core payload_schema_version`
2. `cargo test --workspace`
