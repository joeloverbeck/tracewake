# 0040EPICERHOL-002: EPI-01 — sealed holder-known context construction, scope, identity, hash, and frontier

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0040EPICERHOL-001

## Problem

Spec 0040 audit point EPI-01 (§5) requires certifying that every actor-visible decision or view derives from a **sealed holder-known context** whose holder, viewer, mode, tick, event frontier, projection/schema identity, scope filters, admitted/excluded sources, provenance entries, actor-known fact families, forbidden-truth audit, and status are fixed before consumption — and that this identity/hash/frontier travels unchanged into downstream proposal and view evidence. This ticket collects the EPI-01 positive and adversarial witnesses, replay/provenance evidence, and typed first-divergence diagnostics, and writes the EPI-01 section of the acceptance artifact created by `-001`. It runs existing code; it renders no verdict (the capstone `-015` does).

## Assumption Reassessment (2026-06-19)

1. The EPI-01 seam files verified present at `ba9fe1c` (2026-06-19): `crates/tracewake-core/src/epistemics/knowledge_context.rs` (`ViewMode::{Embodied, Debug}`, scope filters, context ID/hash/frontier, provenance entries, fact families, forbidden-truth audit), `epistemics/knowledge_basis.rs`, `checksum.rs`, `agent/actor_known.rs`, `agent/no_human_surface.rs` (`SealedActorKnownSurface`), `epistemics/projection.rs`, `projections.rs`, `actions/proposal.rs` (`ProposalSourceContext`), `actions/pipeline.rs`. The three context compile-fail fixtures (`external_crate_cannot_build_debug_knowledge_context`, `…_mutate_knowledge_context_mode`, `…_mutate_knowledge_context_viewer`) and the positive fixtures (`view_filtering_001`, `no_human_epistemic_check_001`, `no_human_observation_facts_cite_log_events_001`, `workplace_assignment_provenance_001`) all verified present.
2. The acceptance artifact `reports/0040_epi_cert_…_acceptance.md` is created `(new)` by `-001`; this ticket `(modify)`s its EPI-01 section only. Section wording follows spec §5 EPI-01 and the `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` evidence fields.
3. Shared boundary under audit: the sealed holder-known context contract — the `KnowledgeContext` identity/hash/frontier tuple as it is constructed in `knowledge_context.rs`, carried through `SealedActorKnownSurface` into `SealedProposal`, and validated by the pipeline. The evidence must prove the same values survive that path and that a stale/mismatched/forged tuple fails closed.
4. `INV-101` (actor-known context is sealed) and `INV-100`/`INV-099` (hidden-truth cognition forbidden; truth may validate but not plan) motivate this audit point, with `INV-024` (no telepathy), `INV-026`/`INV-102` (provenance), and `INV-107`/`INV-108` (debug quarantine, possession neutrality). Restated: action-proposal generation must consume a sealed context carrying only modeled actor-known information, never validator-only or hidden truth.
5. This ticket audits the actor-knowledge / no-leak firewall and the projection/replay determinism surface as an **evidence consumer**: it builds contexts from accepted event prefixes and compares them, but modifies no enforcement. It confirms the collection introduces no leakage path (paired hidden-truth worlds yield identical focal-actor packet/hash) and no nondeterminism (live vs replay-rebuilt context equality), and that any debug-mode context row stays `observer-only` per `INV-107`. No `from_initial_beliefs` convenience path is allowed to create provenance-free knowledge in the evidence (spec §10). Any failed seam is recorded `fail` with responsible layer and routed to a separate `EPI-CERT scoped remediation` spec (§7.6/§8); it is not fixed here.

## Architecture Check

1. A dedicated EPI-01 evidence ticket writing only its artifact section is the accepted certification-audit shape (`archive/tickets/0038SPICEREVE-002`…): the expensive workspace build/test ran once in `-001`, and this ticket adds the EPI-01-specific packet-construction, hash-sensitivity, and proposal-parity witnesses without re-running the whole vocabulary or self-certifying.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2 forbids a shim/alias/duplicate-constructor that preserves an epistemic bypass).

## Verification Layers

1. `INV-101` sealed context -> replay/golden-fixture check + manual review: build the embodied context twice from the same accepted prefix/projection/actor/tick/versions; prove identical semantic packet, context ID, hash, frontier, fact ordering, source summary, actor-visible reads (`hidden_truth_gates`, `acceptance_gates`).
2. `INV-024` no telepathy / no-leak -> replay/golden-fixture check: paired worlds differing only in an unobserved hidden item/route/workplace truth, another actor's private belief, or attached debug state produce the same focal-actor packet and hash (`hidden_truth_gates`).
3. `INV-102`/`INV-026` provenance & hash sensitivity -> schema validation + manual review: changing any semantic field (holder/viewer, mode, tick, frontier, schema/projection identity, privacy filter, allowed/forbidden source set, provenance source, fact, forbidden-truth result, status) changes the hash or fails closed; canonically-equivalent input orders normalize to the same packet.
4. `INV-107`/`INV-108` debug/possession boundary -> codebase grep-proof + negative fixture: the three context compile-fail fixtures stay negative (`negative_fixture_runner`); a stale/mismatched/forged context tuple in a proposal is rejected with a typed layer diagnosis and appends no accepted event.

## What to Change

### 1. Collect EPI-01 positive and adversarial witnesses

Run the EPI-01 commands and record: the canonical holder-known packet serialization used for comparison and its fingerprint scope; context ID/hash/frontier; projection/event prefix; the complete provenance-to-event table; actor/privacy filter decisions; accepted and rejected proposal source bindings; live vs replay context equality; and the first differing field for every negative perturbation. Exercise `view_filtering_001`, `no_human_epistemic_check_001`, `no_human_observation_facts_cite_log_events_001`, and `workplace_assignment_provenance_001` as concrete packet-construction witnesses. Build `SealedActorKnownSurface` from `EpistemicProjection`, carry its tuple through `SealedProposal`, and show the pipeline validates the same values before accepting.

### 2. Write the EPI-01 section of the acceptance artifact

Populate the EPI-01 section with the §4.3 evidence objects and the §9.2 ledger fields per witness (positive, adversarial, replay/provenance, and — where cited — the debug-context negative controls), each row carrying exactly one §9.2 status. A banned-word string match is explicitly **not** accepted as a hidden-truth proof (§5 EPI-01 evidence mechanics).

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-01 section only)

## Out of Scope

- Other audit points EPI-02…EPI-11 and the §6.1 compile-fail corpus matrix (owned by their own tickets).
- The §9.4 verdict table and aggregate verdict (owned by `-015`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; the fix is a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8). It may not be relabeled or skipped to a later gate.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test acceptance_gates` — captured.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — captured (paired hidden-truth packet/hash equality).
3. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — the three context compile-fail fixtures remain negative.
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` and `--test golden_scenarios`, plus `cargo test --locked -p tracewake-content --test golden_fixtures_run` — live vs replay context equality.

### Invariants

1. Same accepted prefix → identical context ID/hash/frontier/packet across two builds and live-vs-replay; any single semantic-field perturbation changes the hash or fails closed (`INV-101`/`INV-102`).
2. A context for actor A never admits actor B's private belief/observation, prior-possessed knowledge, raw event-log truth, hidden item location, or debug notes; a forged/stale context tuple appends no accepted event (`INV-024`/`INV-107`).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-01 existing gates/fixtures and records the behavior witnesses, provenance-to-event table, and first-divergence diagnostics as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test acceptance_gates && cargo test --locked -p tracewake-core --test negative_fixture_runner`
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` (sealed-context replay-equality boundary)
