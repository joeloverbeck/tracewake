# 0044FIRPROCER-017: FIRST-PROOF-17 — cross-gate relational, property-based, metamorphic, and integration closure

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — test-harness only. May add metamorphic/relational assertions to existing certification suites per spec §5.4/§9 when a §7 relation lacks a reachable witness; adds no production logic.
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-17 proves at `U` that independently certified subsystems retain their contracts when combined: it targets cross-boundary interaction faults, hidden-truth non-interference, deterministic composition, and corpus completeness, not subsystem internals. The 17 required generated/metamorphic relations (hidden-custody permutation, observation intervention, premise deletion/substitution, presence/absence dual, collection/order permutation, unrelated-fact / possession / debug / human-presence non-interference, temporal hidden-truth non-interference, temporal legal-update, window/replay partition, tamper localization, marker monotonicity, content semantic alias rejection, cross-fixture determinism) must each demonstrate branch reachability — a relation that cannot reach the intended branch is a failed witness, not a pass. Per spec §9, at least one generated family must compose source-backed expectation, hidden-custody variation, legal check, presence/absence dual, contradiction creation, possession/debug toggle, ordinary no-human advancement, and replay rebuild in one run. This ticket records the FIRST-PROOF-17 metamorphic relations, generator evidence obligations, and (where a relation lacks a reachable witness) adds the minimal audit-only metamorphic assertion to exercise it.

## Assumption Reassessment (2026-06-21)

1. The audited/extended surfaces exist at `U`: `crates/tracewake-core/tests/support/generative.rs`, `crates/tracewake-core/tests/generative_lock.rs`, `crates/tracewake-core/tests/{acceptance_gates.rs, golden_scenarios.rs, hidden_truth_gates.rs, no_human_capstone.rs}`, `crates/tracewake-content/tests/golden_fixtures_run.rs`, and the TUI adversarial/embodied/transcript suites (all confirmed present this session). All integration carriers identified by the final §6.7 census (recorded in `-001`) are in scope.
2. Spec §7 FIRST-PROOF-17 and §9 (property-based/metamorphic/sampling posture) govern this ticket; execution `02`/`03`/`09`/`10`, architecture `13`, and all nine gates as a coherent set bind it. Generated evidence remains sampled unless the finite perimeter is exhaustively enumerated.
3. Cross-artifact shared boundary under audit/extension: the FIRST-PROOF-17 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`) plus the existing generative certification suites; this ticket appends its section and, only when a relation lacks a reachable witness, adds a minimal observer-only metamorphic assertion to `generative_lock.rs` / `support/generative.rs`.
4. Motivating invariants (spec §7 FIRST-PROOF-17): `INV-024` (no telepathy — hidden-truth/debug/human-privilege non-interference relations), `INV-018` (deterministic composition), `INV-111` (observer-only emergence non-targeting). Restate before trusting the narrative: any cross-gate relation that leaks hidden truth/debug/human privilege, loses provenance, changes nondeterministically, double-counts time, bypasses event/replay, or cannot demonstrate branch reachability fails this point.
5. This ticket touches deterministic-replay / actor-knowledge / fail-closed surfaces as a test-harness consumer: any added assertion exercises the public/runtime path rather than directly constructing protected state, introduces no nondeterminism (seeds varied by case index, never wall-clock/`Math.random`-style sources), adds no production code path, and keeps any debug rows observer-only and excluded from authoritative fingerprints. A vacuous property whose precondition discards every intended case fails the test-oracle requirement.

## Architecture Check

1. Generated/metamorphic relations are mandatory because ordinary example tests can share the same hidden assumption; recording generator implementation/version, seed, case count, shrink path, generated/omitted population, relation preconditions, and finite-exhaustive-vs-sampled scope is the only way to prove branch reachability rather than a single friendly seed.
2. No backwards-compatibility aliasing/shims introduced; the ticket adds test-harness assertions only, never production logic. A relation that directly constructs protected state instead of exercising the runtime path is rejected.

## Verification Layers

1. `INV-024` hidden-truth/debug/human non-interference -> property/metamorphic check (hidden-custody permutation, unrelated-fact, possession, debug, human-presence, and temporal hidden-truth relations keep actor-known contexts/decisions/views/proposals equal before legal evidence).
2. `INV-018` deterministic composition + replay -> property/metamorphic check (collection/order permutation, window-partition, replay-partition, cross-fixture determinism, and tamper-localization relations preserve canonical output / final projections or localize the earliest affected event).
3. `INV-111`/test-oracle reachability -> manual review + codebase grep-proof (observation-intervention, presence/absence dual, premise deletion, temporal legal-update, marker monotonicity, and content semantic-alias relations reach the intended branch; no relation is vacuous; at least one generated family composes all §9 elements in one run).

## What to Change

### 1. Record the 17 cross-gate metamorphic relations

Run the generative/metamorphic suites and record each of the §7 relations 1–17 with its generator implementation/version, seed, input domain and legality constraints, actor/item/place/container/time ranges, number of generated and discarded cases, branch/reachability counters, shrink result and minimal counterexample, relation asserted, projections/checksums compared, and whether the declared perimeter is finite-exhaustive or sampled. Record the §9 composite family that, in one run, exercises source-backed expectation, hidden-custody variation, legal check, presence/absence dual, contradiction creation, possession/debug toggle, ordinary no-human advancement, and replay rebuild via the public/runtime path.

### 2. Add minimal audit-only assertions only where a relation lacks a reachable witness

For any §7 relation whose intended branch is not already exercised by an existing assertion, add the minimal observer-only metamorphic assertion (seed varied by case index) to `generative_lock.rs` / `support/generative.rs` so the relation reaches its branch. Record that a sampled property supports a pass only beside direct certifying witnesses for the same semantic claim and may not be presented as exhaustive; a vacuous property fails the test-oracle requirement. Make no production change.

## Files to Touch

- `crates/tracewake-core/tests/generative_lock.rs` (modify — only if a §7 relation lacks a reachable witness; observer-only metamorphic assertion)
- `crates/tracewake-core/tests/support/generative.rs` (modify — only if a relation needs a new generator/relation helper; test-harness only)
- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-17 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any cross-gate interaction defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Any production-logic change, new gameplay relation, or new fixture (spec §5.4); the per-point evidence (`-002`..`-016`) and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test generative_lock`, `--test hidden_truth_gates`, and `--test acceptance_gates` pass; the hidden-truth/debug/human/temporal non-interference relations keep actor-known outputs equal before legal evidence, each demonstrating branch reachability.
2. `cargo test --locked -p tracewake-core --test golden_scenarios`, `--test no_human_capstone`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; collection/order/window/replay-partition, cross-fixture determinism, and tamper-localization relations hold and the §9 composite family runs in one pass.
3. `cargo test --locked -p tracewake-tui --test adversarial_gates`, `--test embodied_flow`, and `--test transcript_snapshot` pass; possession/debug/human-presence non-interference relations change only the intended artifact class.

### Invariants

1. No cross-gate relation leaks hidden truth/debug/human privilege, loses provenance, changes nondeterministically, double-counts time, or bypasses event/replay; every relation demonstrates branch reachability.
2. Added assertions are observer-only test-harness code exercising the runtime path, introduce no nondeterminism, and add no production logic; sampled properties are not presented as exhaustive.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/generative_lock.rs` — only if a §7 relation lacks a reachable witness: add the minimal observer-only metamorphic assertion exercising that relation's branch.
2. `crates/tracewake-core/tests/support/generative.rs` — only if a relation needs a new generator/relation helper (test-harness only).
3. `None additional — otherwise evidence-only; the existing generative/metamorphic suites below are the verification surface.`

### Commands

1. `cargo test --locked -p tracewake-core --test generative_lock`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-core --test acceptance_gates`
4. `cargo test --locked -p tracewake-core --test golden_scenarios`
5. `cargo test --locked -p tracewake-core --test no_human_capstone`
6. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
7. `cargo test --locked -p tracewake-tui --test adversarial_gates`
8. `cargo test --locked -p tracewake-tui --test embodied_flow`
9. `cargo test --locked -p tracewake-tui --test transcript_snapshot`

## Outcome

Completed: 2026-06-21

Recorded FIRST-PROOF-17 in the shared acceptance artifact as passed for its
cross-gate relational, property-based, metamorphic, and integration closure
scope. Existing runtime-path suites covered the required relations, so no
test-harness code change was needed. The evidence packet now includes
command-ledger rows, gate references, a FIRST-PROOF-17 audit section, and two
evidence ledger items: `E-0044-017-cross-gate-relations` and
`E-0044-017-sampled-metamorphic-posture`.

Verification run:

1. `cargo test --locked -p tracewake-core --test generative_lock` -> pass, 5 passed.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` -> pass, 17 passed.
3. `cargo test --locked -p tracewake-core --test acceptance_gates` -> pass, 12 passed.
4. `cargo test --locked -p tracewake-core --test golden_scenarios` -> pass, 16 passed.
5. `cargo test --locked -p tracewake-core --test no_human_capstone` -> pass, 2 passed.
6. `cargo test --locked -p tracewake-content --test golden_fixtures_run` -> pass, 42 passed.
7. `cargo test --locked -p tracewake-tui --test adversarial_gates` -> pass, 15 passed.
8. `cargo test --locked -p tracewake-tui --test embodied_flow` -> pass, 6 passed.
9. `cargo test --locked -p tracewake-tui --test transcript_snapshot` -> pass, 3 passed.
