# 0047TUIAUTWOR-018: Acceptance artifact + evidence consolidation (capstone)

**Status**: DONE
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new acceptance artifact at `reports/0047_tui_authoritative_world_advance_acceptance.md` (no new production logic)
**Deps**: 0047TUIAUTWOR-017, 0047TUIAUTWOR-004

## Problem

Spec 0047 §7/§4.9-item-15 requires, on execution, an acceptance artifact against the existing `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` at an exact implementation/evidence commit, with: command transcripts; typed event ledgers; replay checksums and rebuild reports (including the interval summary and stop reason); the parity census extension and PAR-010/PAR-011 dispositions; hidden-truth negative witnesses; the human/no-human differential result; and explicit scope limitations (not latest-main, not Phase-4 entry, not second-proof). This capstone consolidates the evidence the prior tickets produced and renders the review-gate verdict; it introduces no new production logic.

## Assumption Reassessment (2026-06-22)

1. The acceptance template exists at `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`; spec §3.2 confirms it "need no change" — its parity/evidence sections package this feature once §4 defines concrete witnesses (now produced by 0047TUIAUTWOR-014…017). The artifact lands at `reports/0047_tui_authoritative_world_advance_acceptance.md` (repo convention: `reports/<NNNN>_<slug>_acceptance.md`, e.g. the archived `archive/reports/0046-parity-acceptance-artifact.md`), archived to `archive/reports/` on spec acceptance per `docs/archival-workflow.md`.
2. The evidence sources are the leaf tickets: the sleep/work witnesses (0047TUIAUTWOR-014/015), the differential suite (0047TUIAUTWOR-016), the parity registry extension (0047TUIAUTWOR-017), and the reference review guardrails (0047TUIAUTWOR-004). This ticket runs/records, it does not implement.
3. Cross-artifact boundary under audit: the capstone re-proves the spec's §7 review gates end-to-end (one tick definition; prior-transaction duration completes after rebuild; one charge classification; sleeping/working continuation emits no `ActorWaited`; unpossessed actors advance on the same timeline; replay reconstructs the frontier for empty ticks; the interval summary is constructible without raw state/global slice; a hidden event leaves the summary unchanged; debug-only details stay structurally unavailable; human/no-human/replay/parity witnesses are all real-pipeline; every changed capability/view-model field has a parity disposition). It owns no production surface — it exercises the pipeline the prior tickets composed.
4. Constitutional invariant motivating the ticket: `INV-098` (feature acceptance is harsh — caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human-runnable, replay-safe, LLM-independent, non-scripted, regression-tested). The artifact is the harness that records each of these for the feature.
5. Enforcement surface audited (evidence-consumer basis): the artifact records the replay checksums/rebuild reports, hidden-truth negative witnesses, and the differential result. It must capture the evidence honestly at an exact implementation/evidence commit, with explicit scope limitations (not latest-main, not Phase-4 entry, not second-proof) — a manifest is path inventory only, not exact-commit proof (spec §0). The artifact introduces no leakage and asserts no certification it does not hold.

## Architecture Check

1. A single acceptance capstone that exercises and records the composed pipeline — rather than re-implementing checks — is the correct shape: the production logic lives in 0047TUIAUTWOR-005…013, the witnesses in 0047TUIAUTWOR-014…017, and this ticket renders the verdict against the `0003` template. Re-enumerating expected counts from the parity registry at record time (not hardcoding) keeps the artifact from going stale.
2. No backwards-compatibility aliasing/shims: the artifact references existing evidence; it adds no parallel verification logic.

## Verification Layers

1. `INV-098` harsh acceptance -> manual review + replay/golden-fixture check: every §7 review-gate answer is backed by a named, re-runnable witness from the prior tickets.
2. Real-pipeline honesty -> codebase grep-proof: the artifact's human/sleep/work/differential evidence cites the real-pipeline tests (0047TUIAUTWOR-014/015/016), not `RunNoHumanDay`.
3. Scope-limitation honesty -> manual review: the artifact explicitly states not-latest-main, not-Phase-4-entry, not-second-proof, and names the exact implementation/evidence commit.

## What to Change

### 1. Author the acceptance artifact (`reports/0047_tui_authoritative_world_advance_acceptance.md`)

Against the `0003` template, populate: command transcripts; typed event ledgers; replay checksums + rebuild reports (incl. interval summary + stop reason); the parity census extension + PAR-010/PAR-011 dispositions; hidden-truth negative witnesses; the human/no-human differential result; and explicit scope limitations. Answer each §7 review gate yes with its backing witness. Record the exact implementation/evidence commit (named at execution, not assumed to be this baseline commit).

## Files to Touch

- `reports/0047_tui_authoritative_world_advance_acceptance.md` (new)

## Out of Scope

- Any production or test logic (0047TUIAUTWOR-005…017 own it).
- The `docs/4-specs/SPEC_LEDGER.md` archived-row entry and the `archive/specs/` move — these are deferred to spec acceptance/closeout per spec §0/§7 and `docs/archival-workflow.md` (this is a staged feature spec, archived on acceptance, never promoted to live `docs/4-specs/`). Recorded as a cross-spec follow-up, not authored here.
- Any latest-main / Phase-4 / second-proof certification claim (§1.2 / §8 prohibition).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings` — the full evidence base is green at the recorded commit; the artifact's transcripts/checksums are reproduced from this run.
2. The artifact answers every §7 review gate yes with a named, re-runnable witness, and re-enumerates the parity capability count from the registry (not hardcoded).
3. The artifact states explicit scope limitations (not latest-main, not Phase-4 entry, not second-proof) and names the exact implementation/evidence commit.

### Invariants

1. Every acceptance claim is backed by a real-pipeline witness (`INV-098`); no claim relies on `RunNoHumanDay` for the human path or on synthetic events.
2. The artifact mints no new invariant/gate/risk ID and makes no certification claim beyond the feature's scope (spec §8).

## Test Plan

### New/Modified Tests

1. `None — capstone/acceptance-evidence ticket; verification is the full-workspace run plus the §7 review-gate checklist recorded in the artifact. The witnesses are owned by 0047TUIAUTWOR-014…017.`

### Commands

1. `cargo test --workspace`
2. `cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all --check`
3. The full-workspace + clippy + fmt run is the correct boundary: the capstone certifies the whole feature's evidence base, so the complete acceptance command set applies (per `CLAUDE.md` build/test/lint contract).

## Outcome

Completed: 2026-06-22

Evidence:
- Authored and archived `archive/reports/0047_tui_authoritative_world_advance_acceptance.md` against the existing acceptance template.
- The artifact records exact implementation/evidence commit `4228e1e2e5efd759e7e7bddb939a599e344742e9`, parity count `21 + 6`, named real-pipeline witnesses, human/no-human differential result, hidden-source anti-leak witness, replay evidence, CI/PAR-011 lane, and explicit scope limitations.
- Verified `.github/workflows/ci.yml` already runs the focused parity target, locked workspace tests, locked workspace build, and warnings-denied clippy.
- Passed `cargo test -p tracewake-tui --test playable_capability_parity`.
- Passed `cargo fmt --all --check`.
- Passed `cargo test --workspace`.
- Passed `cargo clippy --workspace --all-targets -- -D warnings`.
- Passed `cargo build --workspace --all-targets --locked`.
