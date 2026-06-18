# 0038SPICEREVE-002: SPINE-01 evidence — event log, envelope, and append-only causal stream

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — fills the SPINE-01 section of the acceptance artifact from existing tests/fixtures.
**Deps**: 0038SPICEREVE-001

## Problem

SPINE-CERT seam SPINE-01 (spec §5) requires proof that every meaningful simulation state transition is represented by an event envelope in an append-only stream, with identity, type, schema version, stream boundary, order, tick, causes, proposal/validation linkages, random-draw references, payload, effects summary, and content-manifest identity sufficient for replay, audit, and provenance — correction by later event, never by rewriting history. This ticket gathers the canonical event-log fingerprints, the envelope field-coverage record, the rebuild-and-compare checksums, and the loud-failure evidence for corrupted/malformed streams, and writes the SPINE-01 section of the acceptance artifact created by `-001`.

## Assumption Reassessment (2026-06-16)

1. The audited seams exist (verified during this session's `/reassess-spec`, 2026-06-16): `crates/tracewake-core/src/events/{envelope,log,apply,mod,mutation}.rs`, `state.rs`, `checksum.rs`, and the tests `crates/tracewake-core/tests/{event_schema_replay_gates,spine_conformance,anti_regression_guards,negative_fixture_runner}.rs`.
2. Spec §5 SPINE-01 names the positive corpus (`replay_item_location_001`, `container_item_move_001`, `door_access_001`, `strongbox_001`, `ordinary_workday_001`, `sleep_eat_work_001`, `no_human_day_001`) and the adversarial families; all fixture modules exist under `crates/tracewake-content/src/fixtures/`. The artifact section and field shapes come from `-001`'s scaffold and `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
3. Shared boundary under audit: the event-envelope + append-only-log contract. The witness must record envelope-field coverage (event ID, type, schema version, stream, stream position, global order, tick, ordering key, actor/process/participants/place, causes, proposal ID, validation report ID, random-draw refs, payload hash/value, effects summary, content-manifest ID) and prove correction-by-later-event, not history rewrite.
4. `INV-009` (meaningful changes require events), `INV-010` (every event needs a cause model), `INV-011` (current-state-only simulation forbidden), `INV-013` (meaningful events leave traces), and `INV-020` (event schema evolution is mandatory) motivate this ticket: a pass requires typed, provenance-bearing, versioned envelopes with causal ancestry, not state assertions.
5. This ticket audits the event-application and append-only surfaces as an **evidence consumer**: it runs `event_schema_replay_gates`, `spine_conformance`, `anti_regression_guards`, `negative_fixture_runner`, and the content golden runner, then records witnesses; it weakens no enforcement. Adversarial cases (duplicate IDs, duplicate global order, wrong stream position, malformed stream keys, missing causes, mutation-without-typed-event, prose-born/hidden-truth payloads) must fail closed or loudly; any survivor is recorded `fail` with responsible layer and routed to remediation, not fixed here.

## Architecture Check

1. Auditing the seam via canonical-fingerprint capture + rebuild-equivalence + adversarial loud-failure is stronger than artifact-presence checks: it proves the append-only/provenance semantics and the divergence diagnostics, not just stable bytes.
2. No backwards-compatibility aliasing or shims introduced.

## Verification Layers

1. `INV-009`/`INV-011` eventful, no current-state authority -> replay/golden-fixture check: rebuild state from the serialized log and compare canonical state/projection checksums for the positive corpus.
2. `INV-020` schema evolution -> codebase grep-proof + replay check: unsupported/missing schema versions are rejected by `event_schema_replay_gates`, not silently repaired.
3. Append-only integrity -> manual review + replay/golden check: first-divergence report captured for at least one intentionally corrupted log (`negative_fixture_runner`); envelope field-coverage table recorded.

## What to Change

### 1. Capture the SPINE-01 event-log and envelope witnesses

Record canonical event-log serialization fingerprints for each required positive fixture; rebuild state from the serialized log and record canonical state/projection checksums; record the envelope field-coverage list (all fields in Assumption Reassessment #3) for representative envelopes. Declare fingerprint scope (raw bytes vs normalized serialization vs parsed semantic content) per row.

### 2. Record adversarial loud-failure evidence

For the adversarial families — missing/unsupported schema versions, duplicate event IDs, duplicate global order, wrong stream position, malformed stream keys, missing required causes, mutation-implying payloads without a typed event/effects summary, and hidden-truth/prose-born payloads — record the rejection or loud-failure witness and the first-divergence report for at least one corrupted log. Map each failure to a typed responsible layer from the §5 SPINE-01 set.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001)

## Out of Scope

- The §4 cross-seam command transcript and §9.3 environment (owned by `-001`).
- Replay rebuild / divergence determinism gates as a seam (owned by `-003`, SPINE-02) — this ticket records the event-log fingerprint and one corrupted-log divergence as SPINE-01 evidence, not the full replay determinism corpus.
- Remediation of any append-only/provenance bypass found — recorded `fail` with responsible layer; fix routed to a separate `SPINE-CERT scoped remediation` ticket/spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — schema/version handling and replay reconstruction.
2. `cargo test --locked -p tracewake-core --test spine_conformance` — spine seams map to named evidence.
3. `cargo test --locked -p tracewake-core --test anti_regression_guards` and `--test negative_fixture_runner` — append-only/adversarial guards fail closed.
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run` — positive fixture corpus appends ordered envelopes that replay reconstructs.

### Invariants

1. Every world mutation in the positive corpus has an appended, typed, versioned event with causal ancestry (`INV-009`/`INV-010`/`INV-011`/`INV-020`).
2. At least one corrupted log produces a captured first-divergence report; no corrupted stream yields a repaired success (`INV-011`).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification runs existing event-log/schema/replay/adversarial tests and the captured fingerprints + envelope field-coverage record are the deliverable.`

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
2. `cargo test --locked -p tracewake-core --test spine_conformance && cargo test --locked -p tracewake-core --test negative_fixture_runner`
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
