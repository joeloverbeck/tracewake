# 0042ORDLIFCER-014: §6.4 generated/metamorphic evidence + cross-gate relational hidden-truth / possession / debug harness

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — test-harness only: paired-run / metamorphic / property assertions added to the existing certification suites (no production logic; spec §2 authorizes evidence instrumentation, not remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §6.4 requires generated and metamorphic evidence that the single-run per-gate tickets cannot supply: relational (hyperproperty) statements over paired/generated runs proving hidden-truth non-interference, single-charge conservation, permutation determinism, provenance fail-closed behavior, lifecycle legality, planner-budget bounds, progress-classification monotonicity, debug/possession non-interference, and replay perturbation with first-divergence attribution. This ticket adds the generated/metamorphic assertions to the existing certification suites and records the §6.4 generated-evidence package (seeds, generator version, case count, shrink result, omitted population) into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The harness homes exist at `98dc042`: `crates/tracewake-core/tests/generative_lock.rs`, `tests/hidden_truth_gates.rs`, `tests/no_human_capstone.rs`, and `tests/support/generative.rs` (confirmed present in the 0042 reassessment census / fetch ledger). These are existing certification suites; this ticket adds assertions, not new production code.
2. Spec §6.4 (generated/metamorphic evidence list) and §4.4 (cross-cutting proof rules — relational hidden-truth proof, single owner/single charge) govern this ticket; §6.4 states generated evidence is `sampled` unless an exhaustive finite perimeter is proven, and seeds/generator-version/case-count/shrink-result/omitted-population must be recorded.
3. Cross-artifact shared boundary under audit: the §6.4 generated/metamorphic section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`) plus the four certification suites this ticket extends; these suites are single-owner here (only this ticket modifies them in this batch), so no merge hub.
4. Motivating invariants (spec §6.4 / §4.4): `INV-018` (determinism under permutation/replay), `INV-024` (no telepathy under hidden-truth metamorphism), `INV-091`/`INV-092` (single-charge conservation), `INV-006` (possession non-interference), `INV-040`/`INV-100` (provenance fail-closed). Restate before trusting the narrative: each property is a relation over related executions, not a single-run assertion.
5. Enforcement surface this harness exercises (it adds test assertions only, modifies no production path): the actor-knowledge firewall, the need-accounting owner, the replay rebuild, and the possession/debug quarantine. The added assertions are observer-only, introduce no production code path, no leakage, and no nondeterministic input into any canonical form; they only *observe* related runs and compare typed outputs.

## Architecture Check

1. Concentrating the relational/generated properties in one harness ticket (rather than scattering metamorphic pairs across the per-gate evidence tickets) keeps each per-gate ticket a clean single-run reviewable diff and gives the hyperproperties — which need ≥2 related executions — one coherent home, matching the EPI-CERT §6.2/§EPI-11 harness shape (`archive/tickets/0040EPICERHOL-013`).
2. No backwards-compatibility aliasing/shims introduced; this adds test assertions to existing suites and writes the §6.4 evidence section. No production logic changes.

## Verification Layers

1. `INV-024` hidden-truth metamorphism -> replay/golden-fixture check (alter only unobserved food/route/affordance/workplace/schedule truth; require equal actor-known cognition output).
2. `INV-091`/`INV-092` single-charge conservation -> generative property (actor/need/tick conservation under window partitioning, action-duration insertion, interruption, and replay).
3. `INV-018`/`INV-006` determinism + non-interference -> replay/golden-fixture check (map/set/input-order permutation determinism; debug/possession non-interference; replay prefix/suffix perturbation with first-divergence attribution).

## What to Change

### 1. Add the §6.4 generated/metamorphic assertions to the certification suites

Implement, against the existing suites and `tests/support/generative.rs`, the nine §6.4 property families: (1) actor/need/tick single-charge conservation under window partitioning, action-duration insertion, interruption, and replay; (2) determinism under map/set/input-order permutation; (3) hidden-truth metamorphism (alter only unobserved food/route/affordance/workplace/schedule truth → equal actor-known cognition output); (4) provenance deletion/substitution fail-closed; (5) lifecycle sequence generation (legal transitions replay identically; illegal terminal/reactivation/replacement reject); (6) planner-budget boundaries and coherent fallback; (7) progress classification (a non-progress marker cannot increase progress; replacing it with a permitted committed event changes the metric exactly once); (8) debug/possession non-interference; (9) replay prefix/suffix perturbation with first-divergence attribution.

### 2. Record the §6.4 generated-evidence package

For each property family record: seeds, generator version, case count, shrink result, and omitted population; declare each family `sampled` unless an exhaustive finite perimeter is proven, with the perimeter basis and representativeness stated. Write this into the §6.4 section of the acceptance artifact.

## Files to Touch

- `crates/tracewake-core/tests/generative_lock.rs` (modify)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/no_human_capstone.rs` (modify)
- `crates/tracewake-core/tests/support/generative.rs` (modify)
- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — §6.4 generated/metamorphic section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Any production/engine logic change (spec §2: evidence instrumentation only; a property that fails records `fail` + responsible layer and routes to a later separately-numbered remediation spec).
- The per-gate single-run witnesses (`-002`…`-013`) and the §7 mutation perimeter (`-015`).
- The aggregate verdict (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test generative_lock` passes with the added single-charge-conservation, permutation-determinism, lifecycle-legality, planner-budget, and progress-classification properties.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` passes with the added hidden-truth metamorphism and provenance deletion/substitution properties (equal actor-known cognition output until legal evidence changes).
3. `cargo test --locked -p tracewake-core --test no_human_capstone` passes with the added debug/possession non-interference and replay prefix/suffix perturbation (first-divergence attribution) properties; the §6.4 evidence section records seeds, generator version, case count, shrink result, and omitted population per family.

### Invariants

1. Relational properties are proven over related executions (paired/generated runs), not single-run assertions; each is `sampled` unless an exhaustive finite perimeter is proven and stated.
2. The harness adds test assertions only — no production code path, leakage path, or nondeterministic canonical input is introduced.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/generative_lock.rs` — adds conservation / determinism / lifecycle / budget / progress-classification properties.
2. `crates/tracewake-core/tests/hidden_truth_gates.rs` — adds hidden-truth metamorphism and provenance fail-closed properties.
3. `crates/tracewake-core/tests/no_human_capstone.rs` — adds debug/possession non-interference and replay-perturbation properties; `tests/support/generative.rs` — shared generators/shrinkers.

### Commands

1. `cargo test --locked -p tracewake-core --test generative_lock`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-core --test no_human_capstone`
4. `cargo test --workspace --locked`

## Outcome

Completed: 2026-06-20

Implemented the §6.4 generated/metamorphic evidence package with test-harness-only changes. `generative_lock` now records explicit generated evidence metadata, seed-order determinism, progress-classification, and lifecycle-terminal relational checks. `hidden_truth_gates` now includes a paired hidden-truth metamorphism and provenance deletion/substitution fail-closed test. `no_human_capstone` now includes debug/possession non-interference plus replay perturbation evidence. Shared generator support records the evidence version, shrink result, and omitted population.

Updated `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with the generated/metamorphic command ledger and sampled evidence package. The package is recorded as sampled, not exhaustive, and does not replace the mutation floor or aggregate verdict.

Verification run:

1. `cargo test --locked -p tracewake-core --test generative_lock`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-core --test no_human_capstone`
4. `cargo test --workspace --locked`

All four required commands passed. Transcript files were captured under `/tmp/0042-014-*.txt`; the committed report records byte counts and SHA-256 digests. Production logic was not changed.
