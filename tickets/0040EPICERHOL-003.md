# 0040EPICERHOL-003: EPI-02 — typed propositions, beliefs, stance/confidence, privacy, and freshness

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2.
**Deps**: archive/tickets/0040EPICERHOL-001.md

## Problem

Spec 0040 audit point EPI-02 (§5) requires certifying that beliefs remain typed, holder-scoped, fallible, source-backed epistemic records distinct from observations, memories, records, validation truth, and world state; that stance and confidence are explicit (confidence uses the repository's bounded integer representation, not floats); that actor beliefs are private to their actor absent an upstream-supported public path; and that staleness/supersession are visible rather than silently converted into current truth. This ticket collects the EPI-02 witnesses and writes the EPI-02 section of the acceptance artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-02 seam files verified present at `ba9fe1c` (2026-06-19): `epistemics/proposition.rs`, `epistemics/belief.rs` (`HolderKind`, `Stance`), `epistemics/observation.rs` (`Confidence(u16)` — bounded integer, `Channel`, `TickWindow`, `EPISTEMIC_RECORD_SCHEMA_V1`), `epistemics/projection.rs`, `epistemics/knowledge_context.rs`, `agent/actor_known.rs`, `agent/no_human_surface.rs`, `view_models.rs`. The positive fixtures (`sound_uncertainty_001`, `aged_food_record_surfaces_as_remembered_belief_not_observation_001`, `embodied_workplace_availability_reflects_belief_not_truth_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001`, `stale_workplace_notice_superseded_by_newer_001`, `partial_food_source_knowledge_001`, `seeded_food_source_unknown_to_all_actors_001`) and the four compile-fail fixtures (`banned_float_confidence_types`, `external_crate_cannot_construct_belief_literal`, `…_mutate_belief_source_or_scope`, `…_read_raw_epistemic_projection_maps`) all verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-02 section only. Section wording follows spec §5 EPI-02 and the `0003` evidence fields.
3. Shared boundary under audit: the typed belief/proposition contract — `Belief`/`Stance`/bounded `Confidence`/holder/privacy/source fields and the freshness/supersession classification in `projection.rs`, as filtered by `knowledge_context.rs` and surfaced in the embodied notebook (`view_models.rs`). The evidence must show the record and world truth separately wherever they disagree.
4. `INV-021`…`INV-031` motivate this audit point — especially `INV-021` (typed claims are the epistemic currency), `INV-025` (wrong beliefs are first-class state), `INV-026` (important beliefs need provenance), `INV-028` (staleness is not automatically corrected), `INV-030` (evidence is not truth) — plus `INV-099`…`INV-102` and `INV-112` (holder-known time, not free truth labels). Restated: a wrong/stale belief may shape actor-visible availability while authoritative validation may still reject the attempted action.
5. This ticket audits the actor-knowledge / no-leak firewall and the projection/replay determinism surface as an **evidence consumer**: it confirms changing only hidden world truth creates/strengthens/refreshes no belief without an admissible event-backed source (no leakage), and that belief records replay-reconstruct identically (determinism). Confidence is proven bounded-integer (the float compile-fail fixture stays negative). Two actors may hold different stances for the same proposition without cross-leak. Any failed seam is recorded `fail` and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated EPI-02 evidence ticket writing only its artifact section is the accepted certification-audit shape; it adds the typed-belief, privacy-scope, and freshness/supersession witnesses without re-running the whole vocabulary or self-certifying.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2).

## Verification Layers

1. `INV-021`/`INV-026` typed source-backed belief -> schema validation + replay/golden-fixture check: typed proposition/belief serialization carries holder/privacy scope, stance, integer confidence, source reference + source events, acquired/current tick, freshness classification, contradiction links; replay reconstructs identically (`golden_fixtures_run`, `event_schema_replay_gates`).
2. `INV-025`/`INV-030` wrong belief vs truth -> replay/golden-fixture check: `embodied_workplace_availability_reflects_belief_not_truth_001` and `embodied_workplace_believed_open_truth_closed_commit_fails_001` show a wrong belief shaping availability while validation rejects the commit; the witness shows record and world truth separately.
3. `INV-028` staleness/supersession -> replay/golden-fixture check: `stale_workplace_notice_superseded_by_newer_001` proves deterministic supersession by source tick + stable event identity, older record retained/auditable, not current.
4. `INV-024` no telepathy + bounded confidence -> negative fixture + manual review: `banned_float_confidence_types`, belief-literal, source/scope-mutation, raw-projection-map compile-fail fixtures stay negative (`negative_fixture_runner`); hidden-truth change alone creates no belief.

## What to Change

### 1. Collect EPI-02 positive and adversarial witnesses

Run the EPI-02 commands and package, per record: typed proposition/belief serialization, holder/privacy scope, stance, integer confidence, source reference + source events, acquired/current tick, freshness classification, contradiction links, supersession comparison, context-filter result, replay reconstruction, and embodied notebook rendering — with the record and world truth shown separately where they disagree. Prove two actors' divergent stances do not overwrite or leak; prove a newer-but-wrong belief supersedes per declared policy without being rewritten into truth and ties resolve deterministically.

### 2. Write the EPI-02 section of the acceptance artifact

Populate the EPI-02 section with the §9.2 ledger fields per witness (positive, adversarial, replay, compile-fail controls), each row carrying exactly one §9.2 status.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-02 section only)

## Out of Scope

- Other audit points and the §6.1 compile-fail corpus matrix (owned by their own tickets).
- The §9.4 verdict table and aggregate verdict (owned by `-015`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `--test acceptance_gates` — captured.
2. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — float-confidence, belief-literal, source/scope-mutation, raw-projection-map fixtures remain negative.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-tui --test embodied_flow` — belief/freshness/notebook witnesses.
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — belief-record replay equality.

### Invariants

1. Confidence is the bounded integer representation, never a float; beliefs carry holder/privacy/stance/source/freshness and are distinct from observations and world state (`INV-021`/`INV-026`).
2. Hidden-truth change alone creates/strengthens/refreshes no belief; supersession is deterministic and the stale record is retained-not-truth (`INV-024`/`INV-028`); cross-actor stances never leak.

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-02 existing gates/fixtures and records the typed-record serializations, freshness/supersession table, and record-vs-truth witnesses as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-content --test golden_fixtures_run && cargo test --locked -p tracewake-core --test negative_fixture_runner`
3. `cargo test --locked -p tracewake-tui --test embodied_flow` (notebook/belief rendering boundary)
