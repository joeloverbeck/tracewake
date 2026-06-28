# 0056FOUCONSEV-008: §6.2 post-closure conformance-row truthing

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: 0056FOUCONSEV-001, 0056FOUCONSEV-002, 0056FOUCONSEV-006

## Problem

Spec §6.2. After F7-01…F7-07 are implemented and executed at one exact commit, the live conformance rows must be truthed to describe the sealed surfaces — but not before executable closure. Do not edit conformance rows first: a conformance/doc row describing an enforcement that has not landed yet is a dishonest intermediate state.

## Assumption Reassessment (2026-06-28)

1. Doc targets verified at `2720167`: `docs/1-architecture/04` + `docs/2-execution/05` (action-proposal/scheduler — bootstrap handoff rows), `docs/1-architecture/10` + `docs/2-execution/07` (possession/TUI/debug — debug-authority rows), `docs/1-architecture/00` (architecture index/conformance) + `docs/3-reference/00` (reviewer rows). These describe the loaded-world / TUI-debug authority surfaces that 001/002 reseal.
2. Spec §6.2. The truthing lands only after the executable witnesses exist — hence `Deps` on 001 (bootstrap), 002 (debug authority), and 006 (the executed standing campaign at one commit). This is distinct from §6.1 (0056FOUCONSEV-007), which lands *with* the F7-03/F7-04 code.
3. **Shared boundary under audit**: the conformance read-model rows at architecture `04`/`10`/`00` and execution `05`/`07`, plus the reference `00` reviewer rows. Each updated evidence row must answer: which sealed constructor created the runtime, which operator capability crossed the debug boundary, what state/event/projection effect was observed, and which deliberate mutation or negative compile attempt proves sensitivity.
4. INV-008 (UI is not authority) / INV-068 (debug non-diegetic) / INV-098 (harsh acceptance) — the conformance rows must describe the *landed* sealed authority (a validated-content proof object, an operator-gated debug boundary), not a public alias or an ordinary-input mint, so the doc tier tells the truth about the code.
5. **Evidence-consumer enforcement surface**: this doc-only ticket truths rows describing the fail-closed-bootstrap and debug-quarantine surfaces sealed by 001/002 without modifying code. Item-5 on the evidence-consumer basis: confirm the truthed rows cite the sealed constructor / operator capability / observed effect / sensitivity proof and introduce no claim ahead of its executed proof (gated by `Deps` on 006). No item-6 schema-shape change (markdown only).

## Architecture Check

1. Placing the truthing in a trailing ticket gated on executed closure (006) honors the spec's "do not edit conformance rows first" directive: the rows become true only once the sealed surfaces and the standing campaign exist at one commit. A cross-cutting docs ticket keeps the multi-doc row update atomic and legible as a docs-only diff.
2. No backwards-compatibility shim — conformance rows are amended in place to cite the landed sealed surfaces; no archived spec/ticket/report/acceptance/certification is edited; `SPEC_LEDGER.md` routes through the normal ledger/archive process at spec acceptance (Step 6 cross-spec follow-up, not this ticket).

## Verification Layers

1. INV-008/INV-068 (authority boundaries) -> grep-proof that architecture `04`/`10` and execution `05`/`07` rows name the sealed validated-content proof object and the operator-gated debug boundary (not a public alias / ordinary-input mint), each with its live-API compile-fail negative proof.
2. INV-098 (harsh acceptance) -> manual review that every truthed row answers the four evidence questions (which constructor, which capability, what observed effect, which sensitivity proof) and cites the executed commit.
3. Doc-only ticket — verification is grep-based landing + manual conformance review, not `cargo` tests.

## What to Change

### 1. Truth the bootstrap rows

`docs/1-architecture/04` and `docs/2-execution/05`: name the sealed validated-content proof object and its live-API compile-fail negative proof (not a public alias).

### 2. Truth the debug-authority rows

`docs/1-architecture/10` and `docs/2-execution/07`: an ordinary embodied launch cannot induce debug authority; debug entry requires an explicit operator capability.

### 3. Truth the index/reviewer rows

`docs/1-architecture/00` and `docs/3-reference/00`: update the loaded-world / TUI authority and reviewer rows to the sealed end-state.

## Files to Touch

- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (modify)
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` (modify)
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (modify)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify — conformance rows; distinct from the §6.1 reviewer-prompt edit in 0056FOUCONSEV-007)

## Out of Scope

- The §6.1 doctrine synchronization (0056FOUCONSEV-007) — that lands with the F7-03/F7-04 code, not after closure.
- The code reseals themselves (001/002) and the mutation run (006).
- The `SPEC_LEDGER.md` row and `archive/specs/` move — deferred to spec acceptance (Step 6 cross-spec follow-up).

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proofs that architecture `04`/`10`/`00` and execution `05`/`07` conformance rows name the sealed validated-content proof object and the operator-gated debug boundary, each citing its live-API compile-fail / behavioral negative proof.
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings` — repo cleanliness unaffected.

### Invariants

1. No conformance row describes an enforcement ahead of its landed/executed proof (gated by Deps on 006).
2. No archived spec/ticket/report/acceptance/certification is edited; every truthed row answers the four evidence questions.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based grep landing checks and the conformance review named in Assumption Reassessment.`

### Commands

1. `grep -niE "validated-content|sealed bootstrap|operator capability|operator-gated" docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — truthed rows landed.
2. `git grep -nE "local_operator_debug_authority|from_validated_content_parts" docs/` — must return zero (no conformance row cites a now-removed public alias).
