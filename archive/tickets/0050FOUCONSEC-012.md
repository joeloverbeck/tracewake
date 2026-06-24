# 0050FOUCONSEC-012: Documentation status/evidence truthing + `0049MUTWIT` source record

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: 0050FOUCONSEC-003, 0050FOUCONSEC-005, 0050FOUCONSEC-006, 0050FOUCONSEC-007, 0050FOUCONSEC-008, 0050FOUCONSEC-009, 0050FOUCONSEC-010, 0050FOUCONSEC-011

## Problem

Spec-0050 §7 + §4.8 (driver F-08): once executable evidence exists, the live conformance/evidence homes must be **truthed** (status/evidence only, not doctrine — spec §5 confirms no doctrine amendment is warranted), and the `0049MUTWIT` line must gain its missing source-discipline record. This is a cross-cutting docs ticket: it lands atomically after the implementation + evidence tickets so the doc rows cite repaired production paths and real witnesses rather than stale/overbroad `0048` breadth.

## Assumption Reassessment (2026-06-24)

1. The doc homes exist at `HEAD` (`8d7c119`): `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`, `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`, `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`, `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`, `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (R-27/R-28/R-29 present), `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`, `docs/4-specs/SPEC_LEDGER.md`. The three `0049MUTWIT` tickets are at `archive/tickets/0049MUTWIT-001..003.md`; `0049MUTWIT` is absent from `SPEC_LEDGER.md` (verified — the F-08 gap).
2. Spec-0050 §7/§4.8 and §5 are authoritative: this is status/evidence truthing, **not** doctrine amendment (none warranted). §4.8 forbids inventing a new identifier or authoring paste-ready ratified ledger text in the report-stage spec — the source record's exact home/wording is the reassess/ledger process's call; this ticket lands the navigation record substance per §4.8.
3. Shared boundary under audit: the conformance-index / risk-register / execution-evidence rows and the `SPEC_LEDGER.md` source-discipline graph. Cross-artifact ticket: it touches several docs that cite the `-003`…`-011` surfaces; the truthing must post-date their landing (hence the broad `Deps`).
4. `INV-098` (harsh feature acceptance: caused, no-human runnable, replay-safe, TUI-playable, regression-tested) and the R-27/R-28/R-29 reference guardrails motivate this ticket: stale/overbroad conformance evidence and an orphaned mutation-witness line both weaken the anti-regression posture.
5. Enforcement surface (governance/substrate basis): the conformance index, risk register, and ledger *govern* the fail-closed-validation / actor-knowledge / deterministic-replay evidence posture without touching code. Per the doc-amendment AR-item-5 rule, confirm the truthing introduces no leakage/nondeterminism path and no doctrine weakening — it only re-points rows at the repaired production witnesses and records the `0049` line; R-27/R-28/R-29 are updated in evidence/status only (mint no risk ID).

## Architecture Check

1. A single cross-cutting docs ticket landing after the evidence tickets is cleaner than per-ticket doc edits: the conformance rows, risk-register evidence, and ledger record all require the implementation + evidence surfaces to exist coherently, so co-locating them avoids a staleness window where a row claims an enforcement not yet landed.
2. No backwards-compatibility shims: N/A (docs); no doctrine is amended (spec §5), no risk ID is minted, no archived ticket is edited.

## Verification Layers

1. `INV-098` (acceptance evidence is current) → manual review + codebase grep-proof: each truthed conformance/execution row cites the repaired production witness (grep-proof the witness names resolve); R-27/R-28/R-29 evidence/status updated without a new risk ID.
2. F-08 source-discipline → codebase grep-proof: `SPEC_LEDGER.md` contains the `0049MUTWIT` navigation record (the three tickets, affected surfaces, test-only mutation-witness classification, relation to `0048`), and no new identifier or ratified-doctrine wording was invented.
3. Reference-checklist → invariants alignment check: `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` prompts are answered with the live executable evidence (no doctrinal rewrite).

## What to Change

### 1. Truth the conformance/execution/reference rows

Update `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (loaded-world, time-control, holder-known stop, parity, replay-report rows → repaired production path + exact witness scope); `docs/2-execution/05` (final phase choreography + owned process/actor discovery boundary); `docs/2-execution/06` (human/no-human proof uses core-derived loaded work + same actor-transaction artifacts); `docs/2-execution/07` (complete core-owned interval result + read-only TUI path); `docs/2-execution/10` (map each claim to production-path behavior / mutation / compile-fail / replay / topology evidence). Answer the `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` prompts with live evidence.

### 2. Update the risk register evidence/status

In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, update R-27 (reachability overstatement), R-28 (incomplete correction closure / defect-family completeness), R-29 (guard vacuity / decorative locks) evidence/status to cite spec-0050's production-path closure. Mint no new risk ID.

### 3. Record the `0049MUTWIT` source/navigation record

In `docs/4-specs/SPEC_LEDGER.md`, add the navigation record for the `0049MUTWIT` line per §4.8 substance: identify it as the follow-up to the eight `0048` survivors + adjacent in-diff mutants, name its three archived tickets and affected surfaces, classify it as test-only mutation-witness remediation (not a feature or latest-`main` certification), preserve the historical-vs-fresh-evidence distinction, state its relation to the `0048` acceptance's mutation scope, and route future re-verification to the standing perimeter + named witnesses. Do not invent a new identifier or author paste-ready ratified ledger text; do not edit the archived tickets.

## Files to Touch

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` (modify)
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify)
- `docs/4-specs/SPEC_LEDGER.md` (modify)

## Out of Scope

- Any doctrine amendment (spec §5: none warranted) — no `docs/0-foundation/*` edit, no invariant/risk-ID/gate/glossary minting.
- The `0050` spec's own `SPEC_LEDGER.md` archived row + `archive/specs/` move — deferred to spec acceptance (cross-spec follow-up), not this ticket.
- Editing the archived `0049MUTWIT` tickets or the `0048` acceptance report.

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: each truthed conformance/execution row cites a repaired production witness name that resolves in the post-implementation tree; R-27/R-28/R-29 carry updated evidence/status with no new risk ID.
2. Grep-proof: `docs/4-specs/SPEC_LEDGER.md` contains the `0049MUTWIT` navigation record naming the three tickets and classifying the line; no new identifier and no ratified-doctrine wording were introduced.
3. Grep-proof: no `docs/0-foundation/*` file was modified (no doctrine amendment).

### Invariants

1. The doc edits are status/evidence truthing only — no doctrine weakened, no risk ID minted, no archived artifact edited (spec §5; R-27/R-28/R-29).
2. The `0049MUTWIT` line has an authoritative source/navigation record (F-08 closed).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is grep-proofs against the post-implementation tree and an invariants-alignment review. Existing pipeline coverage is exercised by the implementation/evidence tickets named in Deps.`

### Commands

1. `grep -nE "0049MUTWIT" docs/4-specs/SPEC_LEDGER.md` — the navigation record is present.
2. `grep -rL . docs/0-foundation/ >/dev/null; git diff --name-only -- docs/0-foundation/` — must show no foundation-tier change (no doctrine amendment).
3. `grep -nE "R-2[789]" docs/3-reference/01_DESIGN_RISK_REGISTER.md` — R-27/R-28/R-29 present with updated evidence/status, no new risk ID.

## Outcome

Completed: 2026-06-24

Updated the live conformance/evidence homes without touching
`docs/0-foundation/` or archived tickets. The architecture conformance row now
cites 0050 production-path evidence rather than treating the historical 0048
acceptance as current loaded-world reachability proof. Execution docs `05`,
`06`, `07`, and `10` now identify the core-owned loaded actor/process
discovery, actor transaction outcome consumption, read-only TUI interval
product, separated evidence classes, and the non-green standing mutation
posture from `0050FOUCONSEC-011`.

Updated `docs/3-reference/01_DESIGN_RISK_REGISTER.md` under existing R-27,
R-28, and R-29 only; no new risk ID was minted. Updated
`docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` with the live
0050 executable evidence names. Added the `0049MUTWIT` source/navigation
record to `docs/4-specs/SPEC_LEDGER.md`, naming
`archive/tickets/0049MUTWIT-001.md`,
`archive/tickets/0049MUTWIT-002.md`, and
`archive/tickets/0049MUTWIT-003.md`, and classifying the line as test-only
mutation-witness remediation rather than a feature, doctrine amendment, or
latest-main certification.

Verification:

- `grep -nE "0049MUTWIT" docs/4-specs/SPEC_LEDGER.md`
- `git diff --name-only -- docs/0-foundation/`
- `grep -rL . docs/0-foundation/ >/dev/null`
- `grep -nE "R-2[789]" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `rg -n "0050 loaded-world temporal conformance|due_loaded_actor_ids|due_process_invocations|ActorDecisionTransactionOutcome|EventLogError::DuplicateEventId|ReplayTemporalVerdict|actor_known_interval_delta|negative_fixture_runner|parity_adversarial|generative_lock|0049MUTWIT" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md docs/3-reference/01_DESIGN_RISK_REGISTER.md docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md docs/4-specs/SPEC_LEDGER.md crates/tracewake-core/src/scheduler.rs crates/tracewake-core/src/agent/transaction.rs crates/tracewake-core/src/events/log.rs crates/tracewake-core/tests crates/tracewake-tui/tests`
- `rg -n "R-[0-9]{2}" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `git diff --check`
- `cargo test -p tracewake-core --test doc_invariant_references --test anti_regression_guards`
