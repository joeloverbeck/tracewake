# 0040EPICERHOL-015: Acceptance capstone — §9.4 verdict table, §9.5 replay/provenance package, §9.8 EMERGE-OBS, and aggregate EPI-CERT verdict

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — consolidates the audit-point/compile-fail/relational/mutation evidence into the §9.4 verdict table, assembles the §9.5 replay/provenance package and §9.8 EMERGE-OBS slot, and renders the verdict; introduces no production logic.
**Deps**: archive/tickets/0040EPICERHOL-001.md, archive/tickets/0040EPICERHOL-002.md, archive/tickets/0040EPICERHOL-003.md, archive/tickets/0040EPICERHOL-004.md, archive/tickets/0040EPICERHOL-005.md, archive/tickets/0040EPICERHOL-006.md, archive/tickets/0040EPICERHOL-007.md, archive/tickets/0040EPICERHOL-008.md, archive/tickets/0040EPICERHOL-009.md, archive/tickets/0040EPICERHOL-010.md, archive/tickets/0040EPICERHOL-011.md, archive/tickets/0040EPICERHOL-012.md, archive/tickets/0040EPICERHOL-013.md, archive/tickets/0040EPICERHOL-014.md

## Problem

Spec 0040 is non-executable and renders no verdict; the acceptance artifact does (§9). This capstone consolidates the eleven audit-point sections (`-002`…`-011`, `-013`), the §6.1 compile-fail corpus (`-012`), and the §9.6 mutation package (`-014`) into the §9.4 per-seam verdict table, assembles the §9.5 replay/provenance package and §6 fixture-matrix confirmation, packages the §9.8 EMERGE-OBS observer-only evidence, and renders the single `EPI-CERT passed | EPI-CERT scoped remediation` verdict (§9.9) at the exact implementation commit. It exercises the pipeline the prior tickets composed; it adds no new production logic.

## Assumption Reassessment (2026-06-19)

1. The upstream evidence sections are produced by `-002`…`-014`, each writing its own section of `reports/0040_epi_cert_…_acceptance.md` (created by `-001`). This capstone reads those sections and writes only the §9.4 verdict table, §9.5 package, §9.8 slot, and the §9.9 verdict; it does not re-derive seam evidence. The §9.7 relational package is assembled by `-013` (EPI-11), which the capstone cites.
2. The §9.8 EMERGE-OBS witness `crates/tracewake-core/tests/emergence_ledger.rs` verified present at `ba9fe1c` (2026-06-19). The §6 fixture-matrix families were each verified present under `crates/tracewake-content/src/fixtures/` during this session's `/reassess-spec`. The verdict-table shape comes from spec §9.4 and `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
3. Shared boundary under audit: the consolidated certification verdict (§9.9). The capstone must require every §9.4 row (EPI-01…EPI-11, the configured EPI mutation perimeter, and artifact/evidence honesty) to carry certifying evidence at the exact final commit, and must render `EPI-CERT scoped remediation` if any audit point, the mutation posture, or a fixture family fails or is unproven — never a partial aggregate pass, phase exception, or feature-expansion waiver (§9.9).
4. `INV-098` (feature acceptance is harsh) and `INV-111` (living-world acceptance requires observer-only emergence evidence) motivate this ticket: the §9.5 package is the cross-seam determinism proof, and EMERGE-OBS is packaged strictly observer-only, non-gating, never a scheduler objective or quality substitute (§9.8). The verdict consumes the scope-limited 0037/0039 predecessor facts and does not re-certify or infer latest-main.
5. This capstone audits the consolidated replay/provenance and no-leak surfaces as an **evidence consumer**: it assembles serialized event inputs, replay reports, state/projection checksums, content-manifest fingerprints, sealed holder-known packets, proposal-source tuples, embodied/debug artifacts, contradiction/freshness matrices, and relational pair declarations into §9.5, confirming debug-quarantine artifacts stay separated from embodied transcripts. It renders the verdict; it weakens no enforcement and writes no production code. A failing audit point yields `scoped remediation`, never a relabel or skip to a later gate (§8). A pass is blocked if any result depends on an unfetched file, snippet, mutable metadata, prior-chat memory, or hidden local modification (§2).

## Architecture Check

1. A single trailing acceptance-only capstone that consolidates the sections and renders one verdict is cleaner than letting any audit-point ticket self-certify: the verdict depends on the full leaf set, and centralizing it prevents a green seam row from being read as a phase pass. This mirrors the accepted `archive/tickets/0038SPICEREVE-011` / `0036P0CERPOS0008-012` capstone precedent.
2. No backwards-compatibility aliasing or shims; no new production logic — the capstone exercises the pipeline the earlier tickets composed (spec §2).

## Verification Layers

1. `INV-098` verdict integrity -> manual review + invariants alignment check: every §9.4 row (EPI-01…EPI-11 + mutation perimeter + artifact honesty) carries certifying evidence at the exact final commit; pending/sampled/observer-only/historical/static evidence is never counted as a pass; any unproven row or untriaged behavior-relevant survivor forces `scoped remediation`.
2. `INV-018` cross-seam determinism -> replay/golden-fixture check: the §9.5 package carries serialized event inputs, replay reports, and live-vs-replay state/projection checksums for every required fixture and paired-run member; duplicate-run equality holds.
3. `INV-111`/`INV-107` observer-only emergence, debug quarantine -> manual review: EMERGE-OBS is packaged non-gating and observer-only (citing `emergence_ledger`); debug-quarantine artifacts stay separated from embodied transcripts.
4. fixture-matrix completeness -> replay/golden-fixture check: the §6 fixture-matrix families are each exercised by the registered golden runner (`golden_fixtures_run`); no required family is omitted.

## What to Change

### 1. Assemble the §9.4 verdict table and §9.5 replay/provenance package

Consolidate the audit-point sections, the §6.1 corpus, the §9.6 mutation package, and the §9.7 relational package into the §9.4 per-seam verdict table (each row → required status + required positive/adversarial/replay/mutation evidence IDs). Assemble the §9.5 package: serialized accepted event inputs per fixture/pair-member, event envelope/index/fingerprint + source-event witness tables, live/replay authoritative + epistemic checksums with scopes, sealed holder-known packet identities, proposal source tuples + validation + feedback split, embodied + separate debug artifacts, contradiction/freshness matrices, relational pair declarations, and first-divergence reports.

### 2. Package EMERGE-OBS and render the verdict

Package the §9.8 EMERGE-OBS observer-only evidence (citing `emergence_ledger`) as non-gating, non-threshold. Confirm the §6 fixture-matrix families are each exercised. Render the §9.9 verdict — `EPI-CERT passed` only if all nine §9.9 conditions hold; otherwise `EPI-CERT scoped remediation` with the failed EPI rows, responsible layers, exact evidence gaps, and survivor-register entries named, routing any actionable mutation survivor floor to a later separate spec. The verdict scope explicitly excludes ORD-LIFE-CERT, FIRST-PROOF-CERT, and later gates, and does not certify latest main.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; §9.4/§9.5/§9.8 sections and the §9.9 verdict)

## Out of Scope

- New production logic of any kind — the capstone consolidates and renders; it does not implement.
- The per-audit-point sections (`-002`…`-013`), the §6.1 corpus (`-012`), and the §9.6 mutation package (`-014`) — this ticket reads them, it does not author them.
- Remediation of any failing audit point — the capstone records `scoped remediation` and names the EPI row/layer; the fix is a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8). It may not relabel the gate or skip to ORD-LIFE-CERT/later gates.
- Spec promotion / archival: the `archive/specs/` move and the `docs/4-specs/SPEC_LEDGER.md` *Archived implementation specs* row are deferred to spec acceptance/archival (`docs/archival-workflow.md`), a cross-spec follow-up, not this batch.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace --locked` — full suite green at the implementation commit underlying the verdict.
2. `cargo test --locked -p tracewake-core --test emergence_ledger` — EMERGE-OBS observer-only ledger builds (packaged non-gating).
3. The §9.4 verdict table is complete (EPI-01…EPI-11 + mutation perimeter + artifact honesty), the §9.5 package links every required fixture/pair-member's replay report + checksums, and the rendered §9.9 verdict matches the audit-point/mutation/fixture evidence.

### Invariants

1. The verdict is `EPI-CERT passed` only if all nine §9.9 conditions hold; any failure yields `EPI-CERT scoped remediation` with EPI row + responsible layer named, never a relabel, partial pass, or gate-skip (§8/§9.9).
2. EMERGE-OBS evidence is observer-only and non-gating; the §9.5 package proves cross-seam determinism (`INV-018`/`INV-111`); debug artifacts stay separated from embodied transcripts; no pending/sampled/observer-only/historical/static evidence is counted as a certifying pass.

## Test Plan

### New/Modified Tests

1. `None — acceptance/verification-only capstone; it consolidates the sections and renders the verdict. Verification re-runs the workspace suite and the EMERGE-OBS ledger; the assembled §9.4/§9.5/§9.8 sections and the rendered §9.9 verdict are the deliverable.`

### Commands

1. `cargo test --workspace --locked`
2. `cargo test --locked -p tracewake-core --test emergence_ledger`
3. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` (artifact-wording/evidence-honesty boundary for the rendered verdict)
