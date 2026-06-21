# 0044FIRPROCER-002: FIRST-PROOF-02 — physical custody baseline in EVENT, embodied play, and replay

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-02 must be proven at `U`: the concrete village substrate (items, containers, doors, access, take/place/open/close/inspect/check) behaves through the ordinary action pipeline, and the missing-property setup starts from physical authoritative state rather than narrative implication. A legal open/check/inspect or take/place sequence must traverse proposal → validation → event append → application → projection; the canonical fixture must place a real item at a real authoritative location/custodian while the expecting actor's source-backed belief points elsewhere; embodied TUI must display only actor-legible local affordances; and replay must rebuild physical state and the embodied projection. This ticket records the FIRST-PROOF-02 positive witnesses, adversarial negatives, replay/projection evidence, and TUI view-model fingerprints into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/{location.rs, state.rs, projections.rs, view_models.rs}`, `crates/tracewake-core/src/actions/defs/{movement.rs, openclose.rs, takeplace.rs, inspect.rs, checkcontainer.rs}`, `crates/tracewake-core/src/actions/pipeline.rs`, `crates/tracewake-core/src/events/{envelope.rs, apply.rs}`, and TUI `input.rs`/`render.rs`/`run.rs`/`transcript.rs` (all confirmed present this session). Fixtures `strongbox_001`, `container_item_move_001`, `door_access_001`, `replay_item_location_001`, and `expectation_contradiction_001` exist under `crates/tracewake-content/src/fixtures/`.
2. Spec §7 FIRST-PROOF-02 (seam, audited files, doctrine, positive/adversarial fixtures, event/replay/projection evidence, exact commands, failure condition) governs this ticket; doctrine foundation `03`/`06`/`08`/`12`, architecture `02`/`04`/`09`/`10`, execution `02`/`05`/`09` bind it. Ownership, custody, access, location, belief, and proof remain distinct surfaces.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-02 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-02): `INV-009` (meaningful changes are eventful with a cause), `INV-048` (travel is causal, not teleportation), `INV-055` (ownership/custody/access/control/proof/belief distinct), `INV-018` (replay rebuilds physical state). Restate before trusting the narrative: physical state is born from events through the pipeline, never from prose; no action teleports an actor or item to a true target.
5. This ticket audits/reads (does not modify) the action-validation, event-application, projection, replay, and embodied view-model enforcement surfaces. It runs fixtures and records witnesses only; it adds no alternate mutation path, introduces no nondeterminism, and any audit-only instrumentation stays observer-only and out of the mutation path. Debug-only item location must not become an embodied affordance (recorded as an adversarial negative).

## Architecture Check

1. Proving the full proposal → validation → event → application → projection traversal (rather than asserting final state) is the only check that catches an alternate mutation path, a prose-born physical fact, or a teleport; spec §7 requires the recorded ancestry plus a first-divergence on a deliberately perturbed replay copy, which a final-state assertion cannot provide.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No compatibility shim may preserve a nonlocal/hidden affordance or an alternate mutation route.

## Verification Layers

1. `INV-009` eventful-with-cause -> replay/golden-fixture check (each physical change is an accepted event with recorded proposal source, validation result, IDs/causes; container/door/item projection deltas reconstruct on replay).
2. `INV-048` no teleport -> codebase grep-proof + manual review (no action moves an actor/item to a true target without legal movement/access; locked/inaccessible/nonlocal operations reject with no accepted mutation event).
3. `INV-055`/`INV-018` custody-vs-belief distinctness + replay -> replay/golden-fixture check (item at real custodian while expecting actor's belief points elsewhere; live/replay physical checksums and TUI view-model fingerprint match; perturbed replay copy yields a localized first divergence).

## What to Change

### 1. Record positive physical-custody and embodied witnesses

Run the §7 positive fixtures (`strongbox_001`, `container_item_move_001`, `door_access_001`, `replay_item_location_001`, and the physical portion of `expectation_contradiction_001`). For a legal open/check/inspect or take/place sequence, record the proposal source, validation result, accepted event IDs/causes, item/container/door projection deltas, live/replay physical checksums, and the embodied TUI view-model fingerprint showing only actor-legible local affordances. Record that the canonical fixture has a real item at a real authoritative location/custodian while the expecting actor's source-backed belief points elsewhere.

### 2. Record adversarial negatives and first-divergence

Record the §7 adversarial cases: locked/inaccessible/nonlocal container operations reject with no accepted mutation event; moving the hidden item between equally unobserved lawful locations does not alter the expecting actor's pre-observation cognition or embodied view; no action teleports an actor/item to a true target; debug-only item location cannot become an embodied affordance. Record a first divergence on a deliberately perturbed replay copy. Each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-02 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any physical-substrate defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Expectation provenance (`-003`), absence observation (`-004`), contradiction (`-005`), possession parity (`-008`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test golden_scenarios` and `cargo test --locked -p tracewake-core --test event_schema_replay_gates` pass; the recorded sequence traverses proposal → validation → event append → application → projection with stable IDs/causes.
2. `cargo test --locked -p tracewake-content --test golden_fixtures_run` passes; the canonical fixture's physical custody is event-backed (not prose-born) and replay rebuilds identical physical state and embodied projection.
3. `cargo test --locked -p tracewake-tui --test embodied_flow`, `--test command_loop_session`, and `--test transcript_snapshot` pass; embodied output shows only actor-legible local affordances and the recorded adversarial negatives (locked/nonlocal reject, no teleport, debug-only location excluded) fail at their typed responsible layer.

### Invariants

1. No prose-born physical state, no accepted alternate mutation path, no nonlocal/hidden affordance leakage, and no teleportation.
2. Live/replay physical checksum and embodied view-model fingerprint equality; a perturbed replay copy produces a localized first divergence.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the mutation path.`

### Commands

1. `cargo test --locked -p tracewake-core --test golden_scenarios`
2. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
4. `cargo test --locked -p tracewake-tui --test embodied_flow`
5. `cargo test --locked -p tracewake-tui --test command_loop_session`
6. `cargo test --locked -p tracewake-tui --test transcript_snapshot`

## Outcome

Completed: 2026-06-21

Updated
`reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md`
to mark `FIRST-PROOF-02` passed for its physical-custody, embodied-view, and
replay scope. The artifact records positive witnesses for event-backed
check/open/take/place/inspect behavior, live/replay projection agreement,
fixture fingerprints, and TUI actor-legible local affordances. It also records
adversarial witnesses for locked/closed/nonlocal/wrong-source rejection,
no-teleport behavior, replay tamper divergence, and debug item non-leakage.

No production code, fixtures, schemas, tests, or mutation configuration changed.
No audit-only instrumentation was needed.

Verification run:

- `cargo test --locked -p tracewake-core --test golden_scenarios`
- `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
- `cargo test --locked -p tracewake-content --test golden_fixtures_run`
- `cargo test --locked -p tracewake-tui --test embodied_flow`
- `cargo test --locked -p tracewake-tui --test command_loop_session`
- `cargo test --locked -p tracewake-tui --test transcript_snapshot`

All required commands passed. The artifact remains pending for the downstream
audit points, full mutation execution, and final FIRST-PROOF-CERT verdict.
