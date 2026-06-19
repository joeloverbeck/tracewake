# 0039SPICERMUT-021: Capstone — SPINE-01…08 live re-proof + replacement acceptance artifact + verdict

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — verification/evidence-only; produces the replacement acceptance artifact and renders the aggregate verdict. No production logic.
**Deps**: 0039SPICERMUT-020, 0039SPICERMUT-022, 0039SPICERMUT-023, 0039SPICERMUT-024

## Problem

Once the perimeter is permanent (001), the per-file seed survivors are killed (002–019), the full campaign + register are complete (020), and the additional standing-run survivors are remediated (022–024), the SPINE-01 through SPINE-08 seams must be re-proven live at the exact final implementation commit, and a replacement acceptance artifact must render `SPINE-CERT passed` (or remain `SPINE-CERT scoped remediation` with the responsible layer named) and explicitly supersede the 0038 acceptance artifact (spec §6, §7, §8). The 0038 per-seam rows are historical shape only; they must be re-established at the final commit.

## Assumption Reassessment (2026-06-18)

1. The named SPINE suites all exist (`spine_conformance`, `event_schema_replay_gates`, `hidden_truth_gates`, `golden_scenarios`, `no_human_capstone`, `generative_lock`, `negative_fixture_runner`, `acceptance_gates`, `acceptance_artifact_wording`, content `fixtures_load`/`forbidden_content`/`golden_fixtures_run`/`schema_conformance`, TUI `tui_seam_conformance`/`adversarial_gates`/`embodied_flow`/`command_loop_session`/`transcript_snapshot`/`tui_acceptance`/`readme_sample_session`; verified present). The acceptance-artifact template `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` defines the required evidence fields (verified present). The artifact to supersede is `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (verified present).
2. Spec §6/§7/§8 are the implementation contract; `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` and `docs/4-specs/SPEC_LEDGER.md` govern the SPINE-CERT line state and supersession (verified present); the ledger row update is deferred to spec acceptance (cross-spec follow-up), not this ticket.
3. Shared boundary under audit: the eight-seam evidence program (one program with ticket 020's mutation evidence) and the replacement acceptance artifact that supersedes the 0038 artifact for SPINE-CERT citation.
4. Motivating doctrine: the SPINE gate invariants composed by §6 (event/replay/projection/firewall/no-direct-dispatch) and `INV-111 — Living-world acceptance requires observer-only emergence evidence` (EMERGE-OBS is observer-only and must not influence mutation scoring, action selection, gate results, or the aggregate verdict).
5. This ticket re-proves (does not modify) the deterministic-replay, actor-knowledge-firewall, and fail-closed-validation enforcement surfaces at the final commit, recording per-seam behavior witnesses, holder-known context ID/hash/frontier + source ancestry, paired embodied/debug quarantine evidence, and typed first-divergence reports. The evidence collection introduces no leakage or nondeterminism; pending/historical/sampled/observer-only items may inform review but may not silently count as certifying passes; EMERGE-OBS stays observer-only. There is no schema shape change. The aggregate verdict is `SPINE-CERT passed` only when every §8.4 condition holds.

## Architecture Check

1. A single acceptance-only capstone that re-runs the named suites, records the §6 per-seam witnesses + the §8.3 mutation evidence row (drawn from ticket 020's register), and renders the §8.4 verdict mirrors the 0037 P0-CERT replacement capstone and keeps the verdict gated on completed evidence rather than re-asserting historical rows.
2. No backwards-compatibility aliasing/shims: no new production logic; the artifact cites the exact final implementation commit and does not promote archived 0038 evidence into live proof.

## Verification Layers

1. SPINE-01…08 live re-proof -> replay/golden-fixture + hidden-truth + TUI seam checks: each seam's positive fixture families and adversarial/negative families pass at the final commit (event log/envelope/causality; replay rebuild/divergence; holder-known projection/firewall; random-stream discipline; save/manifest/schema/validation; proposal/validation/scheduling/feedback; embodied TUI/debug split; no-direct-dispatch closure).
2. Mutation evidence row (§8.3) -> ticket 020 register: tool-outcome ledger + 296-seed reconciliation ledger + new-survivor ledger reconcile without double-counting; a passing row requires zero blocked/untriaged survivors and zero unresolved timeouts/tool failures.
3. Aggregate verdict (§8.4) + acceptance-template fields (§8.2) -> `acceptance_artifact_wording` / `acceptance_gates`: the artifact carries evidence-status / fingerprint-scope / behavior-witness / replay-provenance / sampling-exhaustiveness / pending-historical / certification-use / staged-abstraction fields; verdict is `SPINE-CERT passed` only if every condition holds, else `SPINE-CERT scoped remediation` with the responsible layer named.

## What to Change

### 1. Live SPINE-01…08 re-proof

Run the §7.1 command set at the final implementation commit and record live per-seam evidence sections (§6.1–§6.8): positive fixture families, adversarial/negative families, event/replay/projection artifacts, holder-known context ID/hash/frontier + source ancestry, paired embodied/debug quarantine, and typed first-divergence reports. Re-enumerate expected fixture/seam counts from the fixtures at evaluation time rather than hardcoding.

### 2. Replacement acceptance artifact

Produce `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` (or the implementer-recorded filename) conforming to every `docs/4-specs/0003` evidence field, stating exact final commit, target/source baseline `9648c8fb75f6de06c77da4b20b4c30b783cd9217`, tool versions, config/CI/Cargo.lock/source fingerprints, changed files, command transcript index, and explicit supersession of the 0038 artifact with the exact scope limitation (user-supplied target lineage, not independent latest-main verification). Include the §8.3 mandatory mutation evidence row, the EMERGE-OBS observer-only package where the corpus exercises living-world behavior, and the staged-abstraction declaration.

### 3. Aggregate verdict

Render `SPINE-CERT passed` only when every SPINE-01…08 row passes, the mutation row passes, all gate-evidence requirements are present, no pending/historical/observer-only item is counted as a pass, and no staged abstraction hides a SPINE obligation; otherwise `SPINE-CERT scoped remediation` with the remaining responsible layer and evidence gap named. The artifact advances no later gate (EPI-CERT/ORD-LIFE-CERT/FIRST-PROOF-CERT/PHASE-4-ENTRY/SECOND-PROOF-ENTRY).

## Files to Touch

- `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` (new — or the implementer-recorded filename)

## Out of Scope

- The mutation campaign and triage register (ticket 020); per-file kills (002–019); perimeter/CI config (001).
- The `docs/4-specs/SPEC_LEDGER.md` row update and the `specs/`→`archive/specs/` archival move — deferred to spec acceptance (cross-spec follow-up per the hardening-spec convention).
- Advancing or claiming any later certification line.

## Acceptance Criteria

### Tests That Must Pass

1. The full §7.1 command set (`fmt`/`clippy`/`build`/`test --workspace --locked` + every named SPINE suite) passes at the final implementation commit.
2. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording --test acceptance_gates` — the replacement artifact conforms to the `docs/4-specs/0003` fields and the verdict-wording rules.
3. The mutation evidence row (from ticket 020's register) reconciles without double-counting and shows zero blocked/untriaged survivors and zero unresolved timeouts/tool failures for a `SPINE-CERT passed` verdict.

### Invariants

1. A seam row or the aggregate verdict passes only from certifying evidence at the final implementation commit; no archived 0038 row is promoted into live proof, and no pending/historical/observer-only item is counted as a pass.
2. EMERGE-OBS is observer-only and influences no mutation score, action selection, gate result, or the aggregate verdict; the artifact advances no later gate.

## Test Plan

### New/Modified Tests

1. `None — verification/evidence-only capstone; verification is the §7.1 command set run at the final commit, the named SPINE suites, the `acceptance_artifact_wording`/`acceptance_gates` conformance tests against the new artifact, and the reconciled mutation evidence row from ticket 020. No production logic changes.`

### Commands

1. `cargo test --workspace --locked` (plus the per-crate named SPINE suites enumerated in spec §7.1)
2. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording --test acceptance_gates`
3. The whole-workspace re-proof at the final commit (not a per-file run) is the correct verification boundary for the capstone verdict.

## Outcome

Completed: 2026-06-18

Produced
`archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`
as the replacement SPINE-CERT acceptance artifact. The artifact renders
`SPINE-CERT passed` for exact implementation commit
`92ba47f14998e0ea2fc95502bc3b76c5909478ca`, explicitly supersedes the 0038
artifact, preserves the user-supplied target-lineage limitation, and states that
it advances no later gate.

Updated `reports/0039_spine_cert_mutation_triage_register.md` with the capstone
reconciliation addendum. The final standing mutation run enumerated 48 files and
2625 mutants; the full run reported 2079 caught, 545 unviable, 0 missed, and 1
timeout. The timeout retry over `generate_candidate_goals` completed with 6
caught, 2 unviable, 0 missed, and 0 timeouts. No blocked/untriaged survivor,
unresolved timeout, equivalent disposition, non-critical disposition, or
baseline-miss laundering remains.

Added acceptance wording coverage for the 0039 artifact. Verification passed:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace --all-targets --locked`
4. `cargo test --workspace --locked`
5. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording`
6. `cargo test --locked -p tracewake-core --test acceptance_gates`
7. `cargo test --locked -p tracewake-core --test anti_regression_guards`
8. `cargo test --locked -p tracewake-core --test ci_workflow_guards`
9. `cargo test --locked -p tracewake-core --test doc_invariant_references`
10. `cargo test --locked -p tracewake-core --test emergence_ledger`
11. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
12. `cargo test --locked -p tracewake-core --test generative_lock`
13. `cargo test --locked -p tracewake-core --test golden_scenarios`
14. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
15. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
16. `cargo test --locked -p tracewake-core --test no_human_capstone`
17. `cargo test --locked -p tracewake-core --test spine_conformance`
18. `cargo test --locked -p tracewake-content --test fixtures_load`
19. `cargo test --locked -p tracewake-content --test forbidden_content`
20. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
21. `cargo test --locked -p tracewake-content --test schema_conformance`
22. `cargo test --locked -p tracewake-tui --test adversarial_gates`
23. `cargo test --locked -p tracewake-tui --test command_loop_session`
24. `cargo test --locked -p tracewake-tui --test embodied_flow`
25. `cargo test --locked -p tracewake-tui --test readme_sample_session`
26. `cargo test --locked -p tracewake-tui --test transcript_snapshot`
27. `cargo test --locked -p tracewake-tui --test tui_acceptance`
28. `cargo test --locked -p tracewake-tui --test tui_seam_conformance`
29. `cargo mutants --workspace --list-files`
30. `cargo mutants --workspace --list`
31. `cargo mutants --workspace --no-shuffle -j 8 -o mutants-final-0039.out`
32. `cargo mutants --workspace --no-shuffle -j 1 --timeout 600 -F 'generate_candidate_goals' -o mutants-final-0039-timeout-retry.out`
