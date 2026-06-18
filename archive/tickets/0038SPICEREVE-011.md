# 0038SPICEREVE-011: Capstone — per-seam verdict table, replay/provenance package, and SPINE-CERT verdict

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — consolidates the seam evidence into the §9.4 verdict table, assembles the §9.5 replay/provenance package and §9.7 EMERGE-OBS slot, and renders the verdict; introduces no production logic.
**Deps**: 0038SPICEREVE-001, 0038SPICEREVE-002, 0038SPICEREVE-003, 0038SPICEREVE-004, 0038SPICEREVE-005, 0038SPICEREVE-006, 0038SPICEREVE-007, 0038SPICEREVE-008, 0038SPICEREVE-009, 0038SPICEREVE-010

## Problem

Spec 0038 is non-executable and renders no verdict; the acceptance artifact does (spec §9). This capstone consolidates the eight seam-evidence sections (`-002`…`-009`) and the mutation package (`-010`) into the §9.4 per-seam verdict table, assembles the §9.5 replay/provenance package and §6 fixture-matrix confirmation, packages the §9.7 EMERGE-OBS observer-only evidence, and renders the single `SPINE-CERT passed | SPINE-CERT scoped remediation` verdict at the exact implementation commit. It exercises the pipeline the prior tickets composed; it adds no new production logic.

## Assumption Reassessment (2026-06-16)

1. The upstream evidence sections are produced by `-002`…`-010`, each writing its own section of `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (created by `-001`). This capstone reads those sections and writes only the §9.4 verdict table, §9.5 package, §9.7 slot, and the final verdict; it does not re-derive seam evidence.
2. The §9.7 EMERGE-OBS witness exists and was confirmed during this session's `/reassess-spec`: `crates/tracewake-core/tests/emergence_ledger.rs` builds an `EmergeObsLedger` over the no-human generated corpus. The §6 fixture matrix families were each verified present under `crates/tracewake-content/src/fixtures/`. The verdict-table shape comes from spec §9.4 and `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
3. Shared boundary under audit: the consolidated certification verdict. The capstone must require every seam row to carry its §9.4 required status backed by an `observed run` / `negative fixture` / `mutation evidence` witness (never `historical only` or bare `static review` where a behavior witness is required), and must demote the whole verdict to `scoped remediation` if any seam, the mutation posture, or a fixture family fails or is unproven.
4. `INV-018` (deterministic replay is foundational) and `INV-111` (living-world acceptance requires observer-only emergence evidence) motivate this ticket: the §9.5 replay/provenance package is the cross-seam determinism proof, and EMERGE-OBS is packaged strictly as observer-only, non-gating evidence that cannot feed simulation or set objectives.
5. This capstone audits the consolidated replay/provenance and no-leak surfaces as an **evidence consumer**: it assembles event-log packages, replay reports, state/projection checksums, content-manifest fingerprints, holder-known context records, and the corrupted/adversarial loud-failure reports into §9.5, and confirms debug-quarantine artifacts stay separated from embodied transcripts. It renders the verdict; it weakens no enforcement and writes no production code. EMERGE-OBS rows stay `observer-only` per `INV-111`/`INV-107`; a failing seam yields `scoped remediation`, never a relabel or skip to a later gate (spec §8).

## Architecture Check

1. A single trailing acceptance-only capstone that consolidates the seam sections and renders one verdict is cleaner than letting any seam ticket self-certify: the verdict depends on the full leaf set, and centralizing it prevents a green seam row from being read as a phase pass. This mirrors the accepted `archive/tickets/0036P0CERPOS0008-012` capstone precedent.
2. No backwards-compatibility aliasing or shims; no new production logic — the capstone exercises the pipeline the earlier tickets composed.

## Verification Layers

1. `INV-018` cross-seam determinism -> replay/golden-fixture check: the §9.5 package carries event-log packages, replay reports, and live-vs-replay state/projection checksums for each required fixture; duplicate-run equality holds.
2. Evidence-honesty / verdict integrity -> manual review + invariants alignment check: every §9.4 seam row carries its required status with a behavior witness; any unproven seam or untriaged mutation survivor forces `scoped remediation`.
3. `INV-111`/`INV-107` observer-only emergence, debug quarantine -> manual review: EMERGE-OBS is packaged non-gating and observer-only; debug-quarantine artifacts stay separated from embodied transcripts.

## What to Change

### 1. Assemble the §9.4 verdict table and §9.5 replay/provenance package

Consolidate the eight seam sections and the mutation package into the §9.4 per-seam verdict table (each seam → required status + required artifacts). Assemble the §9.5 package: event-log packages and replay reports per required fixture, state/projection checksums, content-manifest fingerprints, holder-known context IDs/hashes/frontiers/provenance for representative cases, debug-quarantine artifacts separated from embodied transcripts, and the corrupted/adversarial loud-failure reports.

### 2. Package EMERGE-OBS and render the verdict

Package the §9.7 EMERGE-OBS observer-only evidence (citing `emergence_ledger`) as non-gating, non-threshold. Confirm the §6 fixture-matrix families are each exercised. Render the final verdict — `SPINE-CERT passed` only if every seam row, the mutation posture, and every fixture family pass; otherwise `SPINE-CERT scoped remediation` with the failing seam(s) and responsible layer(s) named per spec §8. The verdict scope explicitly excludes EPI-CERT, ORD-LIFE-CERT, and later gates.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001; §9.4/§9.5/§9.7 sections and the final verdict)

## Out of Scope

- New production logic of any kind — the capstone consolidates and renders; it does not implement.
- The per-seam evidence sections (`-002`…`-009`) and the mutation package (`-010`) — this ticket reads them, it does not author them.
- Remediation of any failing seam — the capstone records `scoped remediation` and names the seam/layer; the fix is a separate `SPINE-CERT scoped remediation` ticket/spec. It may not relabel the phase or skip to EPI-CERT/later gates (spec §8).
- Spec promotion / archival: the `archive/specs/` move and the `docs/4-specs/SPEC_LEDGER.md` *Archived implementation specs* row are deferred to spec acceptance/archival (`docs/archival-workflow.md`), a cross-spec follow-up, not this batch.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace --locked` — full suite green at the implementation commit underlying the verdict.
2. `cargo test --locked -p tracewake-core --test emergence_ledger` — EMERGE-OBS observer-only ledger builds (packaged non-gating).
3. The §9.4 verdict table is complete (all eight seams + mutation posture), the §9.5 package links every required fixture's replay report + checksums, and the rendered verdict matches the seam/mutation/fixture evidence.

### Invariants

1. The verdict is `SPINE-CERT passed` only if every seam row, the mutation posture, and every §6 fixture family pass; any failure yields `scoped remediation` with seam + responsible layer named, never a relabel or gate-skip (spec §8).
2. EMERGE-OBS evidence is observer-only and non-gating; the §9.5 replay/provenance package proves cross-seam determinism (`INV-018`/`INV-111`); debug artifacts stay separated from embodied transcripts.

## Test Plan

### New/Modified Tests

1. `None — acceptance/verification-only capstone; it consolidates the seam sections and renders the verdict. Verification re-runs the workspace suite and the EMERGE-OBS ledger; the assembled §9.4/§9.5/§9.7 sections and the rendered verdict are the deliverable.`

### Commands

1. `cargo test --workspace --locked`
2. `cargo test --locked -p tracewake-core --test emergence_ledger`
3. `cargo test --locked -p tracewake-core --test spine_conformance` (cross-seam spine map — the narrower boundary confirming the consolidated seam set resolves)

## Outcome

Completed: 2026-06-18

Rendered the acceptance artifact verdict as `SPINE-CERT scoped remediation`.
The eight SPINE seam rows are consolidated in the §9.4 table as locally passed
evidence packages, but the mutation posture blocks certification because Wave B
found 296 missed mutants across SPINE-CERT files. The final verdict names the
responsible remediation layers from the mutation register and records that the
result is a blocking remediation state only, not a pass and not input to later
ladder gates.

Packaged the §9.5 replay/provenance evidence by linking event-log packages,
replay reports, state/projection checksums, content manifest fingerprints,
holder-known context/provenance records, debug-quarantine artifacts, and
corrupted/adversarial loud-failure reports back to the existing command
transcripts and seam sections. Packaged EMERGE-OBS as observer-only,
non-gating evidence.

Verification passed:

- `cargo test --workspace --locked`
- `cargo test --locked -p tracewake-core --test emergence_ledger`
- `cargo test --locked -p tracewake-core --test spine_conformance`
