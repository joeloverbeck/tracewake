# 0040EPICERHOL-006: EPI-05 — provenance witnesses, source-event sufficiency, freshness, and hidden-truth audit

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2.
**Deps**: archive/tickets/0040EPICERHOL-001.md

## Problem

Spec 0040 audit point EPI-05 (§5) requires certifying that every actor-known fact, observation, belief, contradiction, notebook entry, and proposal-relevant assertion carries sufficient, event-backed provenance: `SourceEventIds` non-empty where required, canonical, resolvable in the accepted log, not beyond the context frontier, semantically compatible with the fact, and privacy/holder compatible. Validation truth, debug omniscience, unproven physical truth, prose, and seed metadata are **not** actor-known provenance; dangling/wrong-kind/mismatched/forbidden provenance must fail closed with typed diagnostics. This ticket collects the EPI-05 witnesses and writes the EPI-05 section of the artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-05 seam files verified present at `ba9fe1c` (2026-06-19): `agent/actor_known.rs` (`SourceEventIds`, `ActorKnownFact`, `ActorKnownProvenance`), `agent/no_human_surface.rs`, `agent/transaction.rs` (`dangling_provenance_diagnostic`), `agent/trace.rs` (`HiddenTruthAudit`), `epistemics/knowledge_context.rs`, `epistemics/projection.rs`, `events/envelope.rs`, `events/log.rs`, `actions/proposal.rs`, `actions/pipeline.rs`. Adversarial fixtures (`forbidden_provenance_input_fails_closed_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`, `prose_born_fact_rejected_001`) and positive fixtures (`workplace_assignment_provenance_001`, `no_human_known_workplace_requires_provenance_001`, `no_human_workplace_knowledge_requires_notice_event_001`, `no_human_sleep_knowledge_requires_observation_or_record_001`, `no_human_observation_facts_cite_log_events_001`, `stale_workplace_notice_superseded_by_newer_001`) all verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-05 section only. Section wording follows spec §5 EPI-05 and the `0003` evidence fields.
3. Shared boundary under audit: the provenance-sufficiency contract — `SourceEventIds`/`ActorKnownProvenance` on `ActorKnownFact`, the `HiddenTruthAudit` derivation, and `dangling_provenance_diagnostic` in `transaction.rs`, as carried through `proposal.rs`/`pipeline.rs`. The evidence is a machine-readable witness table, one row per derived record/fact.
4. `INV-102` (cognition inputs require provenance) and `INV-022` (raw prose is not authoritative state) motivate this audit point, with `INV-024`/`INV-026`/`INV-028`/`INV-030`/`INV-031` and `INV-099`…`INV-107`/`INV-112`. Restated: loading a content fact must not by itself create actor knowledge; seeded knowledge is either represented by an admissible seed/notice event + source-backed record or remains unknown.
5. This ticket audits the provenance / no-leak / fail-closed surfaces as an **evidence consumer**: it proves semantic-contamination rejection even when no banned string appears (the three adversarial fixtures), that deleting a source event from replay input yields a dangling-provenance/replay failure naming the responsible layer (not a checksum-coincidence retention), and that `HiddenTruthAudit` is derived from structured facts/gaps. It modifies no enforcement. Any failed seam is recorded `fail` and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated EPI-05 evidence ticket writing only its artifact section is the accepted certification-audit shape; it adds the source-event resolution table, `HiddenTruthAudit`, and `dangling_provenance_diagnostic` witnesses without re-running the whole vocabulary.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2 forbids a shim preserving an epistemic bypass).

## Verification Layers

1. `INV-102` event-backed provenance -> replay/golden-fixture check: `workplace_assignment_provenance_001`, `no_human_workplace_knowledge_requires_notice_event_001`, `no_human_observation_facts_cite_log_events_001` resolve every fact's source event into the exact accepted log with semantic agreement between event kind/payload and fact (`hidden_truth_gates`, `no_human_capstone`).
2. `INV-022`/`INV-030` no prose/evidence-as-truth -> negative fixture + manual review: `forbidden_provenance_input_fails_closed_001`, `prose_born_fact_rejected_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` reject semantic contamination with no banned string (`forbidden_content`, `negative_fixture_runner`).
3. fail-closed provenance -> manual review + replay/golden-fixture check: empty/duplicate-only/nonexistent/future-frontier/wrong-kind/wrong-actor/wrong-subject/wrong-place/wrong-version/validation-only/debug-only/unproven-physical witnesses are rejected with a typed blocker and no accepted proposal/event; deleting a source event from replay yields dangling-provenance/replay failure with responsible layer (`event_schema_replay_gates`).
4. `INV-028` freshness -> replay/golden-fixture check: `stale_workplace_notice_superseded_by_newer_001` retains ancestry + deterministic freshness/supersession classification.

## What to Change

### 1. Collect EPI-05 witnesses

Run the EPI-05 commands and produce a machine-readable witness table with one row per derived record/fact: record identity, holder, privacy scope, semantic kind, source event IDs, event type/schema/payload fingerprint, source tick, context frontier, freshness, derivation rule, downstream consumers, and disposition. Include `HiddenTruthAudit` results derived from structured facts/gaps, `dangling_provenance_diagnostic` output, and mutation witnesses proving these checks are behaviorally guarded. Prove a valid proposal retains source context ID/hash/frontier + source-event ancestry through validation, event append, and actor-visible feedback.

### 2. Write the EPI-05 section of the acceptance artifact

Populate the EPI-05 section with the §9.2 ledger fields per witness (positive, adversarial, replay/provenance, contamination controls), each row carrying exactly one §9.2 status.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-05 section only)

## Out of Scope

- Other audit points and the §6.1 compile-fail corpus matrix (owned by their own tickets). The ordinary-life gate is not certified — these fixtures prove only the EPI provenance boundary, not `ORD-LIFE-CERT` (spec §5 EPI-05, §11).
- The §9.4 verdict table and aggregate verdict (owned by `-015`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `--test acceptance_gates` — source-event resolution + `HiddenTruthAudit`.
2. `cargo test --locked -p tracewake-core --test no_human_capstone` — ordinary-life consumers receive only source-backed facts.
3. `cargo test --locked -p tracewake-content --test forbidden_content` and `cargo test --locked -p tracewake-core --test negative_fixture_runner` — contamination rejected without banned strings.
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` — deleting a source event fails closed with responsible layer.

### Invariants

1. Every required fact carries non-empty, canonical, resolvable, frontier-bounded, semantically/privacy-compatible `SourceEventIds`; missing/wrong/forbidden provenance fails closed with a typed blocker and no accepted event (`INV-102`).
2. Prose, validation-only, debug-only, unproven-physical, and seed metadata are not actor-known provenance; contamination is rejected even with no banned string; loading a content fact creates no actor knowledge (`INV-022`/`INV-030`).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-05 existing gates/fixtures and records the per-fact provenance witness table, HiddenTruthAudit results, and dangling-provenance diagnostics as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test no_human_capstone && cargo test --locked -p tracewake-content --test forbidden_content`
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` (source-event deletion / dangling-provenance boundary)

## Outcome

Completed: 2026-06-19

Populated the EPI-05 section of the acceptance artifact with positive provenance, adversarial contamination, and replay/source-event sufficiency evidence. The §9.4 EPI-05 row now cites `EPI05-POS-001`, `EPI05-ADV-001`, and `EPI05-REPLAY-001`, while its aggregate result remains pending for the mutation package and capstone verdict. No production or test code was changed.

Verification results:
- `cargo test --locked -p tracewake-core --test hidden_truth_gates`
- `cargo test --locked -p tracewake-core --test acceptance_gates`
- `cargo test --locked -p tracewake-core --test no_human_capstone`
- `cargo test --locked -p tracewake-content --test forbidden_content`
- `cargo test --locked -p tracewake-core --test negative_fixture_runner`
- `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
- `cargo test --locked -p tracewake-content --test golden_fixtures_run`

No deviations. `negative_fixture_runner` was run because the ticket requires it in addition to the spec's EPI-05 exact-command set.
