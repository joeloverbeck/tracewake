# 0056FOUCONSEV-009: Acceptance capstone and evidence artifact

**Status**: COMPLETE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: 0056FOUCONSEV-006, 0056FOUCONSEV-007, 0056FOUCONSEV-008

## Problem

Spec §8. The remediation line is acceptance-complete only when a single acceptance artifact, computed by the doctrine-complete fail-closed parser, records per-finding closure (F7-01…F7-07) with real production-path evidence and renders `pass` only on a computed pass. This capstone exercises the pipeline the prior tickets composed; it introduces no new production logic.

## Assumption Reassessment (2026-06-28)

1. The closure surfaces are produced by 001–008: sealed validated-content bootstrap (001), operator-gated debug authority (002), solo-maintainer governance arm (003), closed verdict grammar (004), self-mutation perimeter + CI-forced doctrine-complete parser (005), the executed standing campaign disposition (006), the §6.1 doctrine synchronization (007), and the §6.2 conformance truthing (008). The acceptance-artifact template is `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (with the field set landed by 007); the repo convention for the artifact path is `reports/NNNN_<slug>_acceptance.md` (e.g. `reports/0048_foundational_conformance_hardening_acceptance.md`).
2. Spec §8 + §9 (closure order steps 7–8: run the full standing gate from a clean baseline, then this capstone). The implementing session names its own exact implementation commit (not the proposal baseline `2720167`).
3. **Boundary under audit**: the acceptance read-model boundary — the artifact must carry the machine-readable status manifest computed by the doctrine-complete parser (003/004/005) and must not render `pass` unless the manifest computes pass.
4. INV-098 (feature acceptance is harsh) and INV-093 evidence-honesty — `pass` requires every required finding `closed`, the governance posture proven (or honestly computing non-pass), current green mutation evidence, and the actual artifact under review ingested; the artifact's prose may not exceed the computed state.
5. **Evidence-consumer enforcement surface**: the capstone audits/re-proves the fail-closed-acceptance, replay, debug-quarantine, and validated-bootstrap surfaces sealed by 001–008 without modifying them. Item-5 on the evidence-consumer basis: confirm the artifact's debug/evidence rows stay observer-only and introduce no leakage/nondeterminism, and that the manifest's computed verdict — not free prose — gates the rendered result.

## Architecture Check

1. An acceptance-only capstone gated on the leaf set (006 mutation evidence, 007 doctrine, 008 conformance truthing) is the correct trailing shape: it renders the verdict from the doctrine-complete parser's computed manifest rather than asserting it, closing the laundering vector the whole line targets. It exercises the upstream tickets end-to-end; it does not modify their files.
2. No backwards-compatibility shim — no production logic; the artifact is generated/verified from evidence, and a non-pass manifest yields a non-pass artifact by construction.

## Verification Layers

1. INV-098 (harsh acceptance) -> the artifact carries the status manifest computed by the doctrine-complete parser (closed verdict-grammar line) and renders `pass` only when every required finding is `closed`, governance is proven, mutation evidence is current/green, and the actual artifact under review was ingested.
2. INV-093 evidence-honesty -> per-finding closure rows with real production-path evidence (sealed validated-content constructor, operator-gated debug authority, ordinary-mode debug noninducibility, observed effect, sensitivity/negative-compile proof) and a ruleset API transcript matching the declared governance posture.
3. Acceptance-only capstone — the proof surface is the assembled artifact + the full standing gate run; it adds no production logic.

## What to Change

### 1. Assemble the acceptance artifact

Author `reports/0056_foundational_conformance_seventh_hardening_acceptance.md` per template `0003`: clean baseline, own exact implementation commit, the full gate (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`), the machine-readable status manifest with the closed verdict-grammar line, per-finding closure evidence, and the required-check names + ruleset API transcript matching the governance posture.

### 2. Gate the rendered verdict on the computed manifest

The artifact must not render `pass` unless the manifest computes pass — i.e. every required finding `closed`, the governance posture proven (or honestly non-pass), mutation evidence current and green (from 006), and the actual artifact ingested.

## Files to Touch

- `reports/0056_foundational_conformance_seventh_hardening_acceptance.md` (new — the acceptance/evidence artifact; do not modify upstream tickets' files)

## Out of Scope

- Any production logic — this capstone exercises 001–008, it does not modify them.
- The `SPEC_LEDGER.md` *Archived implementation specs* row and the `git mv` to `archive/specs/` — deferred to spec acceptance/archival per `docs/archival-workflow.md` (Step 6 cross-spec follow-up).
- Editing archived specs/tickets/reports/acceptances/certifications.

## Acceptance Criteria

### Tests That Must Pass

1. The full standing gate from a clean baseline at one exact commit: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace` — all green.
2. The acceptance artifact's machine-readable status manifest computes `pass` via the doctrine-complete parser (003/004/005), with the closed verdict-grammar line and the governance-posture ruleset transcript; the artifact is ingested by the CI ingestion step (005).

### Invariants

1. The rendered verdict equals the computed manifest verdict; no prose asserts a verdict stronger than the computed state.
2. `pass` requires every required finding `closed`, governance proven (or honestly non-pass), current green mutation evidence (006), and the actual artifact under review ingested.

## Test Plan

### New/Modified Tests

1. `None — acceptance-only capstone; verification is the full standing gate plus the doctrine-complete parser computing the artifact's manifest, named in Assumption Reassessment.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
2. `TRACEWAKE_ACCEPTANCE_ARTIFACT="reports/0056_foundational_conformance_seventh_hardening_acceptance.md" cargo test --locked -p tracewake-core --test acceptance_status_manifest actual_acceptance_artifact_from_ci_env_is_pass_eligible` — the CI-forced parser computes pass over the actual artifact.
3. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` — the closed verdict grammar accepts the artifact's wording.

## Outcome

Completed: 2026-06-30

Outcome amended: 2026-06-30

Created `archive/reports/0056_foundational_conformance_seventh_hardening_acceptance.md`
as the archived capstone evidence artifact for exact implementation commit
`9000c392a7c3a3c13589037e4e4bf55c56364b07`. The artifact was initially
created under `reports/` for ticket verification, then moved to
`archive/reports/` during spec closeout.

The artifact carries one `tracewake-acceptance-status` block with:

- `overall_result: pass`
- `expected_findings: F7-01,F7-02,F7-03,F7-04,F7-05,F7-06,F7-07`
- `governance_independence: solo-maintainer-compensating-control` plus the full parser-enforced compensating-control field set
- current full-campaign mutation evidence from ticket `006`: denominator `3451`, caught `2681`, unviable `770`, missed `0`, timeout `0`, survivors `none`
- per-finding closure rows for sealed bootstrap, operator-gated debug authority, governance posture, closed verdict grammar, CI/parser perimeter, standing mutation disposition, and doctrine/conformance truthing

Verification:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.
- `TRACEWAKE_ACCEPTANCE_ARTIFACT=../../archive/reports/0056_foundational_conformance_seventh_hardening_acceptance.md cargo test --locked -p tracewake-core --test acceptance_status_manifest actual_acceptance_artifact_from_ci_env_is_pass_eligible` passed over the archived artifact.
- `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` passed.
- `git diff --check` passed.
