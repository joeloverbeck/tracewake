# 0040EPICERHOL-004: EPI-03 — observation channels, capture boundaries, and event-backed insertion

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2.
**Deps**: 0040EPICERHOL-001

## Problem

Spec 0040 audit point EPI-03 (§5) requires certifying that an observation is a typed, holder-scoped epistemic record produced by a modeled channel and backed by an event/action/cause reference; that direct sight, touch/search, simple sound, and modeled absence have distinct information strength; that observation capture appends causal evidence **before** updating the epistemic projection; and that raw external insertion or world-state inference is forbidden. The reading channel is schema-only unless the staged implementation supplies the full live path. This ticket collects the EPI-03 witnesses and writes the EPI-03 section of the artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-03 seam files verified present at `ba9fe1c` (2026-06-19): `epistemics/observation.rs` (`Channel`, `ReadingPlaceholderSchemaOnly`, `EPISTEMIC_RECORD_SCHEMA_V1`), `agent/perception.rs`, `actions/pipeline.rs`, `events/envelope.rs`, `events/log.rs`, `events/apply.rs` (`"reading_placeholder_schema_only"` arm), `epistemics/projection.rs`, `replay/rebuild.rs`. Negative fixtures (`external_crate_cannot_construct_observation_without_source`, `…_insert_raw_epistemic_records`) and positive fixtures (`no_human_observation_facts_cite_log_events_001`, `sound_uncertainty_001`, `expectation_contradiction_001`, `view_filtering_001`, `hidden_food_closed_container_001`, `hidden_route_edge_001`) all verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-03 section only. Section wording follows spec §5 EPI-03 and the `0003` evidence fields.
3. Shared boundary under audit: the observation capture → event append → projection application path — `perception.rs`/`pipeline.rs` emitting envelopes via `events/log.rs`/`apply.rs` and applying them to `projection.rs`, reconstructible via `replay/rebuild.rs`. The evidence must preserve an append-before-project transcript and prove deleting/bypassing the event breaks the test.
4. `INV-016` (absence becomes evidence only through a channel) and `INV-024` (no telepathy) motivate this audit point, with `INV-009`…`INV-020` (events/traces/replay/schema versioning) and `INV-026`/`INV-028`/`INV-099`…`INV-102`/`INV-112`. Restated: a closed container, unsearched location, unseen route, or out-of-place target must not generate a sight/search/absence observation merely because the state contains the fact.
5. This ticket audits the event-application / projection-replay determinism surface and the no-leak firewall as an **evidence consumer**: it records the append-before-project ordering, replay equality to the same observation set/checksum, and the staged `ReadingPlaceholderSchemaOnly` boundary — confirming the placeholder is not accepted as evidence that reading behavior exists unless the full event/provenance path is supplied and the staged-abstraction change declared (spec §5/§11.9). Out-of-order/duplicate/unsupported-schema/beyond-frontier events fail loudly or record a replay divergence. Any failed seam is recorded `fail` and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated EPI-03 evidence ticket writing only its artifact section is the accepted certification-audit shape; it adds the per-channel capture, append-before-project, and replay-equality witnesses without re-running the whole vocabulary.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2).

## Verification Layers

1. `INV-009`/`INV-018` event-backed, replay-derived -> replay/golden-fixture check: current-place perception appends its event(s), applies them to the projection, records frontier/range, and replays to the same observation set and checksum (`event_schema_replay_gates`, `golden_scenarios`).
2. `INV-016`/`INV-024` channel-gated, no telepathy -> replay/golden-fixture check: `hidden_food_closed_container_001`, `hidden_route_edge_001`, `view_filtering_001` distinguish visible/searchable from hidden facts; an absence record without a completed modeled search/touch (wrong place/target/actor/source) is rejected or omitted (`hidden_truth_gates`).
3. channel information-strength -> schema validation + manual review: `sound_uncertainty_001` exercises `SimpleSound` with bounded confidence + alternatives; `expectation_contradiction_001` exercises `TouchOrSearch`/`AbsenceMarker` through the event path.
4. `INV-020` schema versioning + raw-insert ban -> negative fixture: observation-without-source and raw-epistemic-insert compile-fail fixtures stay negative; a `ReadingPlaceholderSchemaOnly` value is not accepted as completed reading evidence (`negative_fixture_runner`).

## What to Change

### 1. Collect EPI-03 per-channel witnesses

Run the EPI-03 commands and record, per channel: observer, place, tick/window, subject/target, raw payload, bounded confidence, alternatives, schema/privacy, source reference, event envelope, append position, projection application, and replay reconstruction. Preserve an append-before-project transcript and a negative control proving that deleting or bypassing the event breaks the test. Record the `ReadingPlaceholderSchemaOnly` staged-abstraction declaration (proven, abstracted, forbidden-to-fake, future route).

### 2. Write the EPI-03 section of the acceptance artifact

Populate the EPI-03 section with the §9.2 ledger fields per witness (positive, adversarial, replay, compile-fail controls), each row carrying exactly one §9.2 status.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-03 section only)

## Out of Scope

- Other audit points and the §6.1 compile-fail corpus matrix (owned by their own tickets).
- The §9.4 verdict table and aggregate verdict (owned by `-015`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8). Completing the live reading channel is not in scope; the staged placeholder is declared, not implemented.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — append-before-project + replay equality captured.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — channel-gated / no-telepathy witnesses.
3. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — observation-without-source and raw-insert fixtures remain negative.
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-core --test golden_scenarios` — per-channel observation witnesses.

### Invariants

1. Observation capture appends the causal event before projecting; replay reconstructs the same observation set and checksum; deleting/bypassing the event breaks the test (`INV-009`/`INV-018`).
2. No sight/search/absence observation arises merely because the state contains the fact; absence requires a completed modeled channel; `ReadingPlaceholderSchemaOnly` is not completed-reading evidence (`INV-016`/`INV-024`).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-03 existing gates/fixtures and records the per-channel capture rows, append-before-project transcript, and replay reconstruction as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates && cargo test --locked -p tracewake-core --test negative_fixture_runner`
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` (registered observation fixture corpus boundary)
