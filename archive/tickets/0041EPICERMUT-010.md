# 0041EPICERMUT-010: Capstone — EPI-01…11 live re-proof + compile-fail corpus + replacement EPI-CERT acceptance artifact

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — verification/evidence-only; re-runs the EPI seam suites, packages the evidence, and produces the replacement acceptance artifact that renders the aggregate verdict. No production logic.
**Deps**: 0041EPICERMUT-009 (and any `0041EPICERMUT-011` onward run-discovered kill tickets the campaign spawns)

## Problem

Spec 0041 §6–§8 require the replacement EPI-CERT acceptance artifact to re-establish every EPI seam (EPI-01…EPI-11) at the exact final implementation commit, package the gate/diagnostic/replay/provenance evidence (§7), and render the aggregate verdict that supersedes the 0040 artifact (§8). The 0040 per-seam rows, command transcripts, and `ba9fe1c…` fingerprints are structural precedent only; every reused witness must be re-run, re-fingerprinted, and reclassified as live evidence (§1.4, §6, §6.13). A copied command result, historical SHA, archived screenshot, or prior `pass` row remains `historical` and cannot satisfy the replacement verdict.

## Assumption Reassessment (2026-06-19)

1. Codebase check: the EPI seam files (`epistemics/{proposition,belief,contradiction,observation,projection,knowledge_context,knowledge_basis,mod}.rs`), consumers (`agent/{actor_known,transaction,perception}.rs`, `events/apply.rs`, `actions/{proposal,pipeline}.rs`, `view_models.rs`, `debug_capability.rs`, `debug_reports.rs`, `replay/rebuild.rs`, `tracewake-tui/src/render.rs`), the 20 named EPI test binaries, and the §6 fixtures (`view_filtering_001`, `expectation_contradiction_001`, `possession_parity_001`, `workplace_assignment_provenance_001`, `forbidden_provenance_input_fails_closed_001`, `prose_born_fact_rejected_001`, and siblings) all exist at the baseline. The acceptance template `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` and the 0040 artifact to supersede (`archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`) are present.
2. Specs/docs check: §6.1–§6.11 are per-seam deliverable contracts (positive + adversarial + replay/provenance + mutation-coupling evidence + typed first-divergence diagnostic); §6.12 requires the `negative_fixture_runner` compile-fail/boundary corpus case-by-case matrix; §6.13 the reuse declaration; §7 the gate-evidence/diagnostic/honesty package (statuses limited to `pass`/`fail`/`pending`/`sampled`/`observer-only`/`historical`); §8 the artifact identity, command/evidence ledgers, per-seam verdict matrix (§8.5), completed-mutation `pass` row (§8.6), and the §8.10 aggregate verdict + supersession rule. §8.1 fixes the exact filename as an owner decision before evidence generation; a convention-compatible candidate is `reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`, archived under `archive/reports/` at accepted closeout.
3. Cross-artifact shared boundary under audit: the EPI-01…11 production seams ↔ their re-run witnesses ↔ the triage register (ticket 009) and per-file kill evidence ↔ the acceptance artifact's per-seam verdict matrix and mutation `pass` row. Each seam row must cite live evidence at the final commit; a caught mutant strengthens the seam it exercises but does not pass a seam whose positive/negative/replay evidence is missing (§8.5).
4. Motivating invariants (INV restate): §10 binds the full EPI invariant set — event authority/replay (`INV-009`–`INV-020`, `INV-092`), typed epistemics + no telepathy (`INV-021`–`INV-031`, `INV-024`), truth firewall (`INV-099`–`INV-107`, `INV-112`), possession parity (`INV-004`–`INV-008`, `INV-108`), embodied/debug quarantine (`INV-065`–`INV-071`, `INV-093`, `INV-105`), and observer-only emergence (`INV-111`). The artifact must verify the exact final text of every cited invariant and record any genuine conflict as an upstream doctrine blocker, not resolve it in evidence.
5. Fail-closed / actor-knowledge / replay / possession / debug-quarantine surface (evidence-consumer basis): this ticket re-proves these enforcement surfaces by re-running their fixtures. Confirm the evidence package introduces no leakage or nondeterminism path — every event-derived claim is reproduced from serialized input with deterministic projection/context/view/proposal evidence; the EPI-11 paired-run matrix proves non-interference before legal reveal; `EMERGE-OBS` and all debug rows stay `observer-only` and outside the verdict (§8.8); no `historical`/`sampled` evidence passes a seam (§7.4). The artifact certifies only its exact final commit and EPI scope and does not claim latest `main`.

## Architecture Check

1. Folding the §6 multi-seam re-proof + §6.12 corpus + §7 packaging + §8 verdict into one acceptance-only capstone is correct for a mutation-remediation spec: the new behavior is the mutation kills (tickets 002–008) and the campaign (009); the EPI seams already exist and are re-run, not rebuilt. One artifact rendering the aggregate verdict keeps the claim-argument-evidence chain and supersession in a single reviewable place. It introduces no production logic.
2. No backwards-compatibility aliasing/shims: no reused-unchanged row certifies the replacement (§6.13); no `passed with exceptions`/score-based/partial verdict; the 0040 artifact is preserved as historical lineage, not deleted or rewritten.

## Verification Layers

1. EPI-01…11 seam re-proof -> replay/golden-fixture check: each seam's positive + adversarial + replay/provenance witnesses re-run live at the final commit and are re-fingerprinted (§6.1–§6.11).
2. Compile-fail boundary corpus -> codebase grep-proof + `cargo test -p tracewake-core --test negative_fixture_runner`: the §6.12 case-by-case matrix records each forbidden capability, failure class, EPI seam, and gate cross-reference.
3. Mutation `pass` row -> manual review against ticket 009's register: configured denominator/completion, tool-outcome counts, certification-disposition counts (historical seed `30`, zero untriaged, zero unsigned exceptions, zero unresolved), and member-level evidence (§8.6).
4. Aggregate verdict + supersession -> invariants alignment check: §8.10 conditions (1)–(7) hold from certifying evidence before `EPI-CERT passed` is rendered and the 0040 artifact is superseded; otherwise the artifact remains `EPI-CERT scoped remediation` naming the failing layer.

## What to Change

### 1. Live EPI-01…11 re-proof (§6)

For each seam, re-run the named positive, adversarial/negative, and replay/provenance witnesses; re-fingerprint output; record the typed first-divergence diagnostic shape; and tie each seam's mutation coupling to the killed/dispositioned survivors from tickets 002–008 and the register (009). Re-run the EPI-11 paired-run matrix (closed container / hidden route / workplace truth / other-actor privacy / debug / possession / freshness) retaining a deterministic witness per pair and concrete minimized counterexamples for any failure.

### 2. Compile-fail boundary corpus + reuse declaration (§6.12–§6.13)

Record the complete `negative_fixture_runner` command and the case-by-case matrix for the listed external-API/capability boundary fixtures, each naming forbidden capability, failure class, EPI seam, and gate cross-reference. Complete the §6.13 reuse table (every reused 0040 witness re-run/re-fingerprinted; no "reuse unchanged" row certifies).

### 3. Evidence package and acceptance artifact (§7–§8)

Author the replacement artifact (owner-fixed filename; candidate `reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`) instantiating `docs/4-specs/0003`: header/environment block (source baseline `7a17447d…` + one exact final commit, no latest-main claim), command ledger, evidence-item ledger, per-seam verdict matrix (§8.5), completed-mutation `pass` row (§8.6), relational/replay/provenance/debug packages (§8.7), `EMERGE-OBS` observer-only section (§8.8), and the staged-abstraction declaration (`none` if unused). Restrict evidence statuses to the six allowed values; compute each row only from certifying evidence at the final commit.

### 4. Aggregate verdict and supersession (§8.10)

Render `EPI-CERT passed` only when all §8.10 conditions (1)–(7) hold, and state that the artifact supersedes the 0040 EPI acceptance artifact for current EPI-CERT use while preserving it as historical lineage. Otherwise render `EPI-CERT scoped remediation` naming the failing requirement, first responsible layer, retained evidence, and next remediation action. State verbatim that archived specs/tickets `0004`–`0040` are historical/precedent only.

## Files to Touch

- `reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md` (new — owner-fixed filename per §8.1; archive move under `archive/reports/` deferred to accepted closeout)

## Out of Scope

- Authoring any behavior-witness test or production change (kill tickets 002–008) or running the campaign (009); this ticket re-runs existing suites and packages evidence.
- Copying 0040 results, SHAs, or pass-shaped rows as current evidence (§6, §6.13, §11.4).
- Rendering a `passed with exceptions`, score-based, or partial verdict, or inventing any new status (§8.10).
- The `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` + `archive/reports/` moves (deferred to accepted closeout per the hardening-series archival convention — a Step 6 cross-spec follow-up, not ticketed).
- Beginning or certifying `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, or any later ladder move inside the artifact (§1.5, §11.2).

## Acceptance Criteria

### Tests That Must Pass

1. Every EPI-01…11 seam row in the §8.5 matrix cites live `pass` evidence (positive + adversarial + replay/provenance) re-run and re-fingerprinted at the exact final commit; no `historical`/`sampled`/`observer-only` evidence passes a seam.
2. `cargo test -p tracewake-core --test negative_fixture_runner` passes and the §6.12 compile-fail corpus matrix is recorded; the EPI-11 paired-run matrix retains a deterministic witness per pair.
3. The completed-mutation `pass` row (§8.6) is satisfied from ticket 009's reconciled register: configured-denominator completion, honest tool-outcome counts, certification-disposition counts (historical seed `30`; zero blocked/untriaged; zero exceptions lacking signoff; zero unresolved timeouts/tool failures), and member-level evidence.

### Invariants

1. `EPI-CERT passed` is rendered only when all §8.10 conditions (1)–(7) hold from certifying evidence at the final commit; otherwise the artifact remains `EPI-CERT scoped remediation` naming the failing layer.
2. The artifact supersedes the 0040 EPI acceptance artifact while preserving it as historical lineage; no reused-unchanged row certifies; `EMERGE-OBS`/debug evidence stays observer-only and outside the verdict.

## Test Plan

### New/Modified Tests

1. `None — verification/evidence-only capstone; verification is re-running the existing EPI seam suites (§4.4's 20 named binaries) and the `negative_fixture_runner` corpus, packaging their live witnesses into the acceptance artifact. No production logic or new tests are added.`

### Commands

1. `cargo test --workspace --locked` (clean re-proof baseline) and the 20 named EPI suites (`cargo test --locked -p tracewake-core --test hidden_truth_gates`, … through `-p tracewake-tui --test command_loop_session`).
2. `cargo test -p tracewake-core --test negative_fixture_runner` (§6.12 compile-fail corpus).
3. Re-running the existing EPI seam suites is the correct verification boundary: this capstone re-proves seams that already exist and renders the aggregate verdict from ticket 009's reconciled mutation evidence; it adds no production logic, so its deliverable is the live-evidence artifact, not new tests.

## Outcome

Completed: 2026-06-19

Outcome amended: 2026-06-19

Evidence-only capstone completed for exact implementation/evidence commit
`726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3`.

Produced
`reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`,
then archived it at
`archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`
on accepted closeout.
The artifact renders `EPI-CERT passed`, supersedes the 0040 EPI-CERT
acceptance artifact for current EPI-CERT use, preserves 0040 as historical
lineage, and states that archived specs/tickets `0004`–`0040` are
historical/precedent only.

Live proof commands passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace --locked`
- core named EPI suites:
  `hidden_truth_gates`, `event_schema_replay_gates`, `acceptance_gates`,
  `anti_regression_guards`, `generative_lock`, `golden_scenarios`,
  `negative_fixture_runner`, `spine_conformance`, `no_human_capstone`,
  `emergence_ledger`
- content named EPI suites:
  `fixtures_load`, `forbidden_content`, `golden_fixtures_run`,
  `schema_conformance`
- TUI named EPI suites:
  `adversarial_gates`, `tui_seam_conformance`, `transcript_snapshot`,
  `tui_acceptance`, `embodied_flow`, `command_loop_session`

The artifact packages the ticket 009 completed mutation evidence:
2774 configured mutants, 2182 caught, 592 unviable, 0 missed, 0 timeout;
all 30 historical identities reconciled; no new survivors; no baseline-miss
entries.
