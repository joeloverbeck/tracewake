# 0044FIRPROCER-014: FIRST-PROOF-14 — embodied temporal rendering, possession neutrality, and debug quarantine

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-14 certifies the **third routed temporal source** (execution `07`) at `U`: labels such as late, stale, closed, due, soon, recently, or missed may appear in embodied output only when supported by modeled holder-known temporal evidence, and possession and debug do not upgrade temporal knowledge. Each temporal label/affordance must cite holder-known source ancestry or a documented non-temporal derivation; view output must update only after a modeled information event/projection update; possession must preserve the same temporal context/freshness; debug may display current truth alongside actor-known belief with non-diegetic marking; and transcript snapshots must be deterministic. This ticket records the FIRST-PROOF-14 embodied-temporal witnesses, clock/debug non-interference negatives, and transcript/replay determinism into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/{view_models.rs, controller.rs, debug_reports.rs}` and TUI `render.rs`/`debug_panels.rs`/`app.rs`/`input.rs`/`run.rs`/`transcript.rs` (all confirmed present this session). Fixtures `embodied_menu_lags_truth_change_without_perception_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001`, `stale_workplace_notice_superseded_by_newer_001`, `debug_omniscience_excluded_001`, and `possession_parity_001` exist.
2. Spec §7 FIRST-PROOF-14 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; `INV-112`, doctrine foundation `08`, architecture `10`, execution `07` embodied temporal rendering and execution `03` temporal cascade bind it.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-14 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket is temporal source `07` of the five-source bundle, consolidated by `0044FIRPROCER-016`.
4. Motivating invariants (spec §7 FIRST-PROOF-14): `INV-112` (holder-known time must plan; temporal labels are holder-known interpretations with provenance), `INV-067` (embodied mode shows actor-known reality), `INV-068` (debug mode is visibly non-diegetic). Restate before trusting the narrative: a temporal label is actor-legible only through the allowed view and remains unchanged by possession/debug.
5. This ticket audits/reads (does not modify) the holder-known-context, projection, view-model, and debug-quarantine enforcement surfaces. It runs fixtures and records witnesses only; debug panel output must never feed semantic input choices or actor-visible affordances, and observer-only time controls cannot be mistaken for actor knowledge. No nondeterminism is introduced.

## Architecture Check

1. Recording each temporal label/affordance's holder-known source ancestry (or documented non-temporal derivation) and proving the view updates only after a modeled information event — while varying raw clock/queue/true-schedule/debug-timestamp — is the only check that distinguishes holder-known embodied timing from clock/debug-derived timing; spec §7 requires deterministic transcript snapshots and possessed/autonomous equality.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may let bind/unbind refresh/reinterpret/reveal temporal facts.

## Verification Layers

1. `INV-112` holder-known temporal rendering -> view-model + replay/golden-fixture check (each temporal label/affordance cites holder-known source ancestry or a documented non-temporal derivation; view output updates only after a modeled information event/projection update).
2. `INV-067` possession neutrality -> codebase grep-proof + manual review (possession preserves the same temporal context and freshness; bind/unbind does not refresh, reinterpret, or reveal temporal facts).
3. `INV-068` debug quarantine + determinism -> adversarial + replay/golden-fixture check (debug displays current truth alongside actor-known belief with non-diegetic marking; debug panel output never feeds semantic input choices or actor-visible affordances; transcript snapshots are deterministic).

## What to Change

### 1. Record positive embodied-temporal witnesses

Run `embodied_menu_lags_truth_change_without_perception_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001`, `stale_workplace_notice_superseded_by_newer_001`, `possession_parity_001`, and the §7 positive path. Record that each temporal label/affordance in embodied output cites holder-known source ancestry or a documented non-temporal derivation; view output updates only after a modeled information event/projection update; possession preserves the same temporal context and freshness; debug can display current truth alongside actor-known belief with non-diegetic marking; and transcript snapshots are deterministic. Record holder-known context ID/hash/frontier, source event, view-model field, rendered text/semantic action IDs, debug-only rows, transcript hash, and replay output.

### 2. Record clock/debug non-interference negatives

Record the §7 adversarial cases: changing raw clock, queue, true schedule, or debug timestamp without an actor-known source does not change embodied output; bind/unbind does not refresh, reinterpret, or reveal temporal facts; debug panel output never feeds semantic input choices or actor-visible affordances; stale semantic commands reject rather than resolving against truth; observer-only time controls cannot be mistaken for actor knowledge. Compare debug-off/on and possessed/autonomous relations; each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-14 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any embodied-temporal defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Inventing any temporal unit/threshold/source category (spec §5.4); the temporal firewall (`-012`), routine temporal premises (`-013`), temporal fixture families (`-015`), the consolidated five-source line (`-016`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario/temporal tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` passes; each embodied temporal label/affordance cites holder-known source ancestry or a documented non-temporal derivation.
2. `cargo test --locked -p tracewake-content --test golden_fixtures_run`, `cargo test --locked -p tracewake-tui --test embodied_flow`, and `--test tui_acceptance` pass; view output updates only after a modeled information event and possession preserves the same temporal context/freshness.
3. `cargo test --locked -p tracewake-tui --test adversarial_gates` and `--test transcript_snapshot` pass; changing raw clock/queue/true-schedule/debug-timestamp without an actor-known source does not change embodied output, and transcript snapshots are deterministic.

### Invariants

1. No clock/debug-derived embodied timing, no possession refresh, no stale-command truth fallback, no temporal view/replay drift, no unmarked debug truth.
2. Debug output never feeds semantic input choices or actor-visible affordances; observer-only time controls are not actor knowledge.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
3. `cargo test --locked -p tracewake-tui --test adversarial_gates`
4. `cargo test --locked -p tracewake-tui --test embodied_flow`
5. `cargo test --locked -p tracewake-tui --test transcript_snapshot`
6. `cargo test --locked -p tracewake-tui --test tui_acceptance`
