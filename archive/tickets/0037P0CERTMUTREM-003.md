# 0037P0CERTMUTREM-003: Capstone — live P0-01..P0-10 re-proof + replacement P0-CERT acceptance artifact

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — verification/evidence-only; produces the replacement acceptance artifact and renders the verdict.
**Deps**: 0037P0CERTMUTREM-001, 0037P0CERTMUTREM-002

## Problem

The 0036 artifact renders `P0-CERT scoped remediation`, not `passed`, and its P0-01..P0-10 pass rows are historical 0036 audit provenance, not live current-checkout proof. To restore the certification line, the implementing session must re-prove P0-01..P0-10 live at `c54caff`, embed the completed mutation evidence, render `P0-CERT passed` (only if all criteria pass), and produce a replacement acceptance artifact that explicitly supersedes the 0036 artifact.

(spec §4.5, §7, §8)

## Assumption Reassessment (2026-06-16)

1. The §2.4 audit seams and the §4.2 lock-layer gate test set all exist (verified this session: `negative_fixture_runner`, `doc_invariant_references`, `spine_conformance`, `acceptance_gates`, `anti_regression_guards`, `event_schema_replay_gates`, `hidden_truth_gates`, `acceptance_artifact_wording` in core; `schema_conformance`, `forbidden_content`, `golden_fixtures_run` in content; `adversarial_gates`, `tui_seam_conformance` in tui).
2. `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` carries the required fields (evidence status, fingerprint scope, behavior witness, replay/provenance ancestry, sampling/exhaustiveness, pending/historical, certification use) plus the `## Staged-abstraction declaration` section and `EMERGE-OBS` observer-only labeling — verified this session. Supersession target `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md` exists.
3. Cross-artifact boundary under audit: the `0003` acceptance-artifact template contract + the supersession relationship to the 0036 artifact + the §7.2 ten-point P0-CERT matrix.
4. Invariant motivation: INV-018 (deterministic replay re-proof), INV-098 + the live spec-ledger source-discipline rule (every pass row cites live commands, not 0036 rows — P0-10), INV-024 (no-leak firewall), INV-111 (EMERGE-OBS observer-only).
5. The re-proof audits/reads replay, actor-knowledge, fail-closed-validation, possession, and debug surfaces rather than modifying them (evidence-consumer basis): confirm the evidence collection introduces no leakage/nondeterminism path, that debug rows stay observer-only, and that `EMERGE-OBS` is recorded as observer-only (not a gate / pass-fail threshold / mutation score / simulation input).

## Architecture Check

1. A single capstone that re-proves P0-01..P0-10 and consolidates them into the template-conformant artifact is the correct shape: the re-proof IS the evidence-row population (deliverable-doubles-as-capstone). It introduces no production logic; it exercises the pipeline -001 and -002 composed.
2. No shims; archived 0005–0036 specs/tickets are cited as history only, never as live proof (spec §2.3, §7.2 P0-10).

## Verification Layers

1. INV-018 deterministic replay → replay/golden-fixture check: re-collect event/replay/projection evidence (logs, rebuild reports, projection versions, context hashes/frontiers, checksums) for the first-proof fixture families exercised by this line.
2. INV-024 / truth-firewall → core+content+tui gate tests: re-run the §4.2 lock-layer set; re-collect actor-known provenance and hidden-truth negative evidence plus possession/debug quarantine evidence.
3. INV-098 / P0-10 source discipline → invariants alignment check + manual review: every pass row cites live `c54caff` commands/artifacts; archived material is labeled historical; the artifact renders the verdict per the §8.1 rule.

## What to Change

### 1. Re-prove P0-01..P0-10 live

Run the §4.2 preflight + lock-layer gate set; audit the §7.2 per-row seams; re-collect event/replay/projection evidence, actor-known provenance + hidden-truth negative evidence, possession/debug quarantine evidence, and diagnostics by responsible layer. Record changed files and relevant unchanged-seam audits.

### 2. Assemble the replacement acceptance artifact

Author `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` (exact filename per repo closeout convention) conforming to `0003`: all §8.2 fields, the §8.3 mandatory mutation pass row (embedding/linking -002's triage register and -001's kill proof), `EMERGE-OBS` observer-only handling (§8.4) where the corpus exercises first-proof living-world acceptance, and the supersession header (`Supersedes: archive/reports/0036_…acceptance.md`). Render `P0-CERT passed` only if the full posture completed, every survivor is dispositioned, and P0-01..P0-10 pass live; otherwise render `P0-CERT scoped remediation` naming the remaining blocker under the responsible layer (spec §4.1, §8.1).

## Files to Touch

- `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` (new)

## Out of Scope

- The killing test (→ -001) and the mutation run + triage register (→ -002); this ticket consumes both.
- Editing the 0036 artifact (it is superseded, not modified).
- The `archive/specs/` move + `docs/4-specs/SPEC_LEDGER.md` "Archived implementation specs" row — deferred to spec acceptance/archival per `docs/archival-workflow.md` (cross-spec follow-up, not ticketed).
- Advancing any forward gate (SPINE-CERT / EPI-CERT / ORD-LIFE-CERT, Phase-4 entry, second-proof entry) — blocked behind this artifact passing (spec §10).

## Acceptance Criteria

### Tests That Must Pass

1. The §4.2 preflight + lock-layer gate set all pass live at the remediation checkout (re-run, not copied from 0036).
2. Deterministic replay reproduces state byte-identically for the exercised first-proof fixture families; hidden-truth negative + possession/debug quarantine evidence collected.
3. The artifact conforms to `0003` (all §8.2 fields + §8.3 mutation row), supersedes the 0036 artifact, and renders the verdict per §8.1.

### Invariants

1. INV-098 / P0-10: every pass row cites live `c54caff` commands/artifacts; no 0036 pass row is reused as live proof.
2. INV-018 / INV-024 / INV-111: replay determinism, the no-leak firewall, and `EMERGE-OBS` observer-only status all hold; no P0 obligation is deferred behind a staged abstraction.

## Test Plan

### New/Modified Tests

1. `None — verification/evidence-only capstone; the deliverable is the replacement acceptance artifact. Verification re-runs the existing pipeline gates named below and records witnesses; no production logic changes.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked`
2. Lock-layer gate set (§4.2): `cargo test --locked -p tracewake-core --test negative_fixture_runner` … `--test doc_invariant_references` … `--test spine_conformance` … `--test acceptance_gates` … `--test anti_regression_guards` … `--test event_schema_replay_gates` … `--test hidden_truth_gates` … `--test acceptance_artifact_wording`; `cargo test --locked -p tracewake-content --test schema_conformance` / `--test forbidden_content` / `--test golden_fixtures_run`; `cargo test --locked -p tracewake-tui --test adversarial_gates` / `--test tui_seam_conformance`.
3. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` — confirms the replacement artifact's wording/verdict discipline.

## Outcome

Completed: 2026-06-16

Produced
`archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`.
The artifact explicitly supersedes
`archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md`,
renders `P0-CERT passed` for the scoped post-0008 baseline mutation remediation
line, keeps archived 0036 evidence historical-only, and links the full mutation
register at `reports/0037_mutation_triage_register.md`.

The capstone consumed:

- `0037P0CERTMUTREM-001` / commit `2abfce9`: the
  `actor_known_local_actor_reaches_embodied_view_model_with_context_provenance`
  behavior/provenance witness and targeted mutation kill.
- `0037P0CERTMUTREM-002` / commit `a3b5e3c`: the full unsharded configured
  mutation posture, 1128 mutants tested, 896 caught, 232 unviable, 0 missed,
  0 timeout.

Because the pre-existing `acceptance_artifact_wording` test did not include new
acceptance artifacts, this ticket also added scoped P0-CERT wording coverage for
the new replacement artifact. No production logic changed.

Verification run:

- `cargo fmt --all --check` — passed before and after the artifact/test update.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed after the artifact/test update.
- `cargo build --workspace --all-targets --locked` — passed after the artifact/test update.
- `cargo test --workspace --locked` — passed after the artifact/test update; includes `acceptance_artifact_wording` with 5 tests and `hidden_truth_gates` with 14 tests.
- `cargo test --locked -p tracewake-core --test negative_fixture_runner` — passed, 5 tests.
- `cargo test --locked -p tracewake-core --test doc_invariant_references` — passed, 4 tests.
- `cargo test --locked -p tracewake-core --test spine_conformance` — passed, 6 tests.
- `cargo test --locked -p tracewake-core --test acceptance_gates` — passed, 12 tests.
- `cargo test --locked -p tracewake-core --test anti_regression_guards` — passed, 80 tests.
- `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — passed, 17 tests.
- `cargo test --locked -p tracewake-core --test hidden_truth_gates` — passed, 14 tests.
- `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` — passed post-artifact, 5 tests.
- `cargo test --locked -p tracewake-content --test schema_conformance` — passed, 2 tests.
- `cargo test --locked -p tracewake-content --test forbidden_content` — passed, 20 tests.
- `cargo test --locked -p tracewake-content --test golden_fixtures_run` — passed, 40 tests.
- `cargo test --locked -p tracewake-tui --test adversarial_gates` — passed, 15 tests.
- `cargo test --locked -p tracewake-tui --test tui_seam_conformance` — passed, 2 tests.

No P0-CERT obligation was deferred. Spec archival and final ledger/reference
truthing remain the ticket-series final spec closeout step.
