# 0040EPICERHOL-013: EPI-11 — relational anti-contamination & possession-parity capstone harness (+ §6.2 generated/metamorphic)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — test-harness only: paired-run / metamorphic / property assertions added to the existing certification suites (no production logic; spec §2 authorizes evidence instrumentation, not remediation).
**Deps**: 0040EPICERHOL-001

## Problem

Spec 0040 audit point EPI-11 (§5) requires proving the EPI boundary as a **relational** property, not a collection of single-run examples: two executions equal in the focal holder's admissible observation/event history but differing in hidden world truth, another holder's private state, debug state, or controller source must remain equal at every actor-visible and decision-producing EPI surface until a modeled observation makes the difference knowable; after that observation, any divergence must be event/provenance-explained and replay-reproducible. §6.2 requires generated/metamorphic evidence (recorded seeds, population, shrink, omitted domains, retained counterexamples). This ticket builds/runs the paired-run harness across the existing certification suites, assembles the §9.7 relational-capstone package, and writes the EPI-11 section of the artifact created by `-001`. It renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-11 harness homes verified present at `ba9fe1c` (2026-06-19): `crates/tracewake-core/tests/hidden_truth_gates.rs`, `acceptance_gates.rs`, `generative_lock.rs`, `golden_scenarios.rs`, `crates/tracewake-content/tests/golden_fixtures_run.rs`, `crates/tracewake-tui/tests/adversarial_gates.rs`, `embodied_flow.rs`. The spec mandates the harness be committed under these existing certification surfaces rather than a new phase gate (§5 EPI-11), so any added paired-run/property fns land in these files `(modify — as surfaced)`.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-11 section and assembles the §9.7 relational-capstone package. Section wording follows spec §5 EPI-11 / §9.7 and the `0003` evidence fields.
3. Shared boundary under audit: the relational non-interference property over all EPI surfaces named in EPI-01…EPI-10 (actor-known context, candidate set, sealed proposal, embodied view/menu, notebook, why-not). The §EPI-11 matrix enumerates **7 hidden-variable pairs** — Closed container; Hidden route; Workplace truth; Other actor privacy; Debug state; Possession source; Stale truth — each with its required pre-reveal equality domain and its legal reveal trigger; the harness asserts equality before reveal and event-explained divergence after.
4. `INV-093` (actor-knowledge leakage is a high-severity defect) and `INV-099`…`INV-108` (the full truth-firewall / cognition-authority set) motivate this capstone, with `INV-001`, `INV-022`…`INV-031`, `INV-065`…`INV-070`. Restated as a hyperproperty: the audit reduces a two-run information-flow relation to paired self-composed executions (spec §12), so a whole-world checksum is not the equality oracle — the actor-visible/decision-producing surfaces are.
5. This ticket audits the relational no-leak / replay-determinism surfaces as an **evidence producer** that adds test-only harness code: it asserts pre-reveal equality across the 7 pairs (no leakage), event-explained post-reveal divergence reproducible under replay (determinism), and §6.2 metamorphic relations (canonical-order invariance, single-field-change hash sensitivity, source-removal fail-closed). It adds no production logic and weakens no enforcement; generated evidence is labeled `sampled` unless its finite domain is genuinely exhausted. Mutation operators that weaken context parity, allow forbidden sources, invert freshness, bypass debug capability, or render debug tokens into embodied output must be caught by a named capstone witness (cross-ref `-014`). Any failed relation is recorded `fail` with first responsible layer and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated relational-capstone evidence ticket — separate from the acceptance verdict capstone `-015` — is correct because EPI-11 *produces* evidence (it adds paired-run/property harness code), whereas `-015` is acceptance-only with no new logic. `Large` because it spans the 7-pair matrix plus §6.2 generation across seven suites. The harness lives in the existing certification surfaces per spec §5 EPI-11, not a new gate.
2. No backwards-compatibility aliasing or shims; the harness exercises the canonical live seams (spec §2). It introduces no production path and no new always-on simulation behavior.

## Verification Layers

1. `INV-099`/`INV-100` relational pre-reveal equality -> replay/golden-fixture check: for each of the 7 pairs, the focal actor's projection/context, notebook, actions, proposal, and embodied render stay equal before the legal reveal (`hidden_truth_gates`, `acceptance_gates`, `golden_scenarios`, `golden_fixtures_run`, `adversarial_gates`, `embodied_flow`).
2. `INV-018` event-explained divergence -> replay/golden-fixture check: after the legal reveal event, divergence is explainable by the new event/provenance chain and reproducible under replay; both pair members reproduce themselves on rebuild.
3. `INV-093` metamorphic relations (§6.2) -> replay/golden-fixture check + manual review: permuting unordered source/provenance inputs leaves canonical packet/projection/view equal; changing one semantic field changes the relevant hash/validation result; removing/substituting a source event makes the derived fact disappear or fail closed with the correct layer (`generative_lock`).
4. generation honesty -> manual review: generator version, deterministic seeds, population size, shrink/minimization, omitted domains, and all retained counterexamples recorded; every generated claim labeled `sampled` unless its finite domain is exhausted and exhaustion is demonstrated.

## What to Change

### 1. Build/run the paired-run harness and §6.2 generation

Add paired-run assertions to the existing certification suites covering the 7 §EPI-11 pairs: each pair starts from the same focal actor-observable history, differs only in the named hidden variable, asserts the required pre-reveal equality domain, applies the legal reveal trigger, and asserts event-explained divergence + replay reproduction. Add §6.2 generation varying actor IDs, source-event order, frontier, confidence bounds, privacy holder, observation channel, expected/observed proposition pair, hidden-truth perturbation, possession source, and debug state, with recorded seeds and retained minimized counterexamples.

### 2. Assemble the §9.7 package and write the EPI-11 section

Produce the §9.7 paired-run ledger: per pair, the exact hidden input difference, the equality precondition over focal actor-observable history, the exact actor-visible/decision-producing fields compared, pre-reveal result, legal reveal event + provenance, post-reveal comparison, replay confirmation for both runs, generated seed/sample declaration, and first responsible layer on failure. Write the EPI-11 artifact section with the §9.2 ledger fields, each row carrying exactly one status; a whole-world checksum is explicitly not the equality oracle.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-11 section + §9.7 package)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify — as surfaced; paired-run harness fns)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — as surfaced)
- `crates/tracewake-core/tests/generative_lock.rs` (modify — as surfaced; §6.2 generation)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify — as surfaced)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — as surfaced)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify — as surfaced)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify — as surfaced)

## Out of Scope

- The single-run per-audit-point sections EPI-01…EPI-10 and §6.1 (owned by their own tickets) — this ticket proves the relational statement they cannot.
- The §9.4 verdict table, §9.5 package, §9.8 EMERGE-OBS, and aggregate verdict (owned by `-015`); the mutation campaign (owned by `-014`).
- Any production remediation of a failed relation — recorded `fail` with first responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8).

## Acceptance Criteria

### Tests That Must Pass

1. The 7 §EPI-11 pairs (Closed container; Hidden route; Workplace truth; Other actor privacy; Debug state; Possession source; Stale truth) each assert pre-reveal equality over the named actor-visible/decision surfaces and event-explained post-reveal divergence.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `--test acceptance_gates` and `--test generative_lock` and `--test golden_scenarios` — relational + metamorphic harness green.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run`, `cargo test --locked -p tracewake-tui --test adversarial_gates` and `--test embodied_flow` — cross-surface relational witnesses.
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — both pair members reproduce under replay.

### Invariants

1. For every pair, the focal actor's context/notebook/actions/proposal/embodied surfaces are equal before the legal reveal and diverge only via the event/provenance chain after it; the equality oracle is the actor-visible/decision surfaces, never a whole-world checksum (`INV-099`/`INV-093`).
2. Generated evidence records seeds/population/shrink/omitted-domains/counterexamples and is labeled `sampled` unless finite-domain exhaustion is demonstrated (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs`, `acceptance_gates.rs`, `generative_lock.rs`, `golden_scenarios.rs`, `crates/tracewake-content/tests/golden_fixtures_run.rs`, `crates/tracewake-tui/tests/adversarial_gates.rs`, `embodied_flow.rs` — paired-run / metamorphic / §6.2 generation assertions added to the existing certification suites per spec §5 EPI-11 (test-only; no production logic).

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates && cargo test --locked -p tracewake-core --test generative_lock`
2. `cargo test --locked -p tracewake-tui --test adversarial_gates && cargo test --locked -p tracewake-tui --test embodied_flow`
3. `cargo test --workspace --locked` (full relational harness across all touched suites)
