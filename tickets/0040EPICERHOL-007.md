# 0040EPICERHOL-007: EPI-06 — epistemic projection rebuild, checksum determinism, context filtering, and non-truth-writer quarantine

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2.
**Deps**: 0040EPICERHOL-001

## Problem

Spec 0040 audit point EPI-06 (§5) requires certifying that `EpistemicProjection` is a replay-derived aggregate, not a source of simulation truth: its observations, beliefs, contradictions, notebook entries, actor-known records, projection version, content-manifest identity, and applied-event range must be reconstructible from accepted events/declared starting evidence; context-filtered reads must enforce holder/privacy/freshness policy; and canonical checksum equality must be stable under irrelevant insertion order yet sensitive to every semantic projection field. This ticket collects the EPI-06 witnesses and writes the EPI-06 section of the artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-06 seam files verified present at `ba9fe1c` (2026-06-19): `epistemics/projection.rs` (`from_initial_beliefs`, context-filtered reads, checksum), `epistemics/observation.rs`, `epistemics/belief.rs`, `epistemics/contradiction.rs`, `events/apply.rs`, `events/log.rs`, `replay/rebuild.rs`, `replay/report.rs`, `checksum.rs`, `state.rs`. Negative fixtures (`external_crate_cannot_insert_raw_epistemic_records`, `…_read_raw_epistemic_projection_maps`) verified present; `spine_conformance` and `event_schema_replay_gates` test binaries verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-06 section only. Section wording follows spec §5 EPI-06 and the `0003` evidence fields.
3. Shared boundary under audit: the projection-as-replay-derivative contract — `EpistemicProjection` built through the accepted event path vs rebuilt from the serialized accepted log via `replay/rebuild.rs`, compared by the canonical checksum in `checksum.rs`, with context-filtered reads in `projection.rs`. The evidence must prove a projection checksum alone does not prove the projection was not written directly (a negative control demonstrating direct/omitted-event construction would be detected).
4. `INV-011` (current-state-only simulation is forbidden) and `INV-018` (deterministic replay is foundational) motivate this audit point, with `INV-009`…`INV-020`, `INV-023` (ground truth / belief / records separate), `INV-026`/`INV-028`/`INV-099`…`INV-102`/`INV-112`. Restated: changing authoritative hidden state without an observation/belief event must not change the focal actor's epistemic projection or embodied context.
5. This ticket audits the projection/replay determinism surface and the no-leak firewall as an **evidence consumer**: it proves live vs clean-rebuild equality (version, manifest identity, event range/count, typed records, context-filtered reads, freshness/supersession, notebook, actor-known records, canonical checksum), cross-process determinism, and that context-scoped APIs disclose own/permitted-public records while excluding another actor's private records. `from_initial_beliefs` and seed-knowledge paths are proven to consume only explicitly admissible, source-backed starting evidence — a convenience constructor is not permission to create provenance-free knowledge (spec §10). Any failed seam is recorded `fail` and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated EPI-06 evidence ticket writing only its artifact section is the accepted certification-audit shape; it adds the live-vs-rebuild record-by-record diff, checksum-sensitivity, and context-filter witnesses without re-running the whole vocabulary.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2).

## Verification Layers

1. `INV-018` replay-derived projection -> replay/golden-fixture check: for every EPI fixture, build the live projection through the accepted event path, rebuild from the serialized accepted log, and prove equality of version/manifest/event-range/typed-records/context-reads/freshness/notebook/actor-known-records/canonical-checksum; same prefix → same checksum across ≥2 clean processes and after canonical serialize/deserialize (`event_schema_replay_gates`, `spine_conformance`).
2. `INV-011`/`INV-023` non-truth-writer -> replay/golden-fixture check + manual review: changing authoritative hidden state without an observation/belief event does not change the focal actor's projection/embodied context; a negative control shows direct/omitted-event construction would be detected (`hidden_truth_gates`, `golden_scenarios`).
3. context filtering / privacy -> manual review: context-scoped APIs disclose own/private + permitted public records while excluding another actor's private records, for ≥2 actors plus debug mode.
4. raw-write quarantine -> negative fixture: raw insertion/read compile-fail fixtures remain negative; mutating projection bookkeeping (event count/range, checksum serialization, record source identity, location/holder keys, freshness policy, supersession comparison, context filter) is killed by the configured mutation campaign (cross-ref `-014`) (`negative_fixture_runner`).

## What to Change

### 1. Collect EPI-06 witnesses

Run the EPI-06 commands and retain: live and rebuilt projection serializations, checksums + checksum scope, event range, applied-event list/count, record-by-record diff, context-filter traces for ≥2 actors plus debug mode, freshness/supersession table, replay report / first divergence, and a negative control demonstrating direct or omitted-event construction would be detected. Prove `from_initial_beliefs`/seed paths consume only admissible source-backed starting evidence.

### 2. Write the EPI-06 section of the acceptance artifact

Populate the EPI-06 section with the §9.2 ledger fields per witness (positive, adversarial, replay/provenance, raw-write negative controls), each row carrying exactly one §9.2 status. Mutation evidence for projection bookkeeping is cross-referenced to `-014`.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-06 section only)

## Out of Scope

- Other audit points and the §6.1 compile-fail corpus matrix (owned by their own tickets).
- The §9.4 verdict table and aggregate verdict (owned by `-015`); the projection-bookkeeping mutation campaign itself (owned by `-014`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — live-vs-rebuild equality + checksum sensitivity.
2. `cargo test --locked -p tracewake-core --test spine_conformance` — replay-derived projection conformance.
3. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `--test golden_scenarios`, plus `cargo test --locked -p tracewake-content --test golden_fixtures_run` — hidden-state-change non-effect + per-fixture rebuild.
4. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — raw insertion/read fixtures remain negative.

### Invariants

1. Live projection == clean rebuild from the same accepted event stream on every semantic field + canonical checksum; checksum is stable under irrelevant order and sensitive to every semantic field (`INV-018`).
2. Changing hidden authoritative state without an admissible event changes no focal-actor projection/context; `from_initial_beliefs` creates no provenance-free knowledge (`INV-011`/`INV-023`).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-06 existing gates/fixtures and records the live-vs-rebuild record-by-record diff, context-filter traces, and direct-construction negative control as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
2. `cargo test --locked -p tracewake-core --test spine_conformance && cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` (full registered-fixture rebuild boundary)
