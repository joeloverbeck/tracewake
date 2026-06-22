# 0046TUISIMPLA-010: Execution-tier parity amendments + standing per-feature obligation (exec 00/03/07/09/10 + specs README)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — execution-tier doctrine docs (`docs/2-execution/00_*`, `03_*`, `07_*`, `09_*`, `10_*`) and the specs `README.md`; no code. Execution precondition: ordinary execution-tier owner approval before applying.
**Deps**: 0046TUISIMPLA-005, 0046TUISIMPLA-006, 0046TUISIMPLA-007, 0046TUISIMPLA-008

## Problem

Spec 0046 §5 `PAR-DOC-004`/`PAR-DOC-005` and §4.4 `PAR-010`. The execution tier must own the parity
proof shapes, fixture coverage, real-pipeline assertion mechanics, CI command, and report capture, and
the standing obligation that every Expansion feature spec carries a parity-impact declaration + passing
evidence before acceptance. Without these edits the contract has code (003–008) and architecture
doctrine (009) but no execution-tier mechanics home and no per-feature gate.

## Assumption Reassessment (2026-06-22)

1. Verified the target docs exist at baseline `1145e109`:
   `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`,
   `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`,
   `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`,
   `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`,
   `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`, and
   `docs/4-specs/README.md`. The mechanics they describe land in tickets 005 (real-pipeline harness,
   determinism-vs-golden), 006/007 (fixture coverage, census), 008 (CI lane) — Deps.
2. Verified against spec 0046 §5: `PAR-DOC-004` — `07` owns the actor-known positive/negative and
   debug-quarantine proof shape; `09` owns fixture coverage, focused golden scenarios, fixture-ID
   resolution; `10` owns real-pipeline invocation, typed-before-rendered assertions, checked-in render
   references, the determinism-vs-golden clarification, CI commands, report capture. `PAR-DOC-005` —
   `00`/`03` + specs `README` state that every Expansion feature spec carries a parity-impact
   declaration + passing evidence before acceptance, aggregate evidence into future phase/cert packets,
   run the conformance suite in ordinary CI; register the obligation as a standing acceptance dimension;
   **no new gate identifier, no change to any passed rung's verdict**. `PAR-010` (the per-feature
   declaration requirement) is enacted by the `00`/`03`/README edits.
3. Shared boundary under audit: the execution tier owns proof mechanics + phase-ladder acceptance
   sequencing; the specs README owns the per-spec authoring obligation. Cross-artifact doctrine ticket;
   exec `00` is the tier index — flag if a sibling touches it (none in this batch).
4. Invariant restated (`PAR-DOC`/`PAR-010` motivation): `INV-095` TUI/view-model tests are acceptance
   tests; `INV-091` no-human tests mandatory; `INV-098` acceptance is harsh. The standing obligation
   makes parity a per-feature acceptance dimension without minting a gate code (spec §1.2).
5. Enforcement surface governed (doctrine, substrate basis): exec `07` governs the actor-known /
   debug-quarantine proof firewall and `10` the deterministic evidence capture. Confirm the amendments
   require positive+negative actor-known proof, keep debug evidence non-satisfying for embodied
   assertions, and require deterministic report capture — introducing no leakage/nondeterminism path and
   changing no passed rung's verdict.

## Architecture Check

1. Placing proof mechanics in execution (07/09/10) and the standing per-feature obligation in the phase/
   authority docs (00/03) + specs README is the correct tier split: architecture (009) owns the
   contract, execution owns *how it is proven*, the spec layer owns *when each feature must prove it*.
   Registering parity as a standing acceptance dimension (not a new gate) avoids ladder duplication and
   audit lag (spec §3.2).
2. No backwards-compatibility aliasing/shims: doctrine prose only; no passed rung re-opened; no legacy
   acceptance path retained.

## Verification Layers

1. `PAR-DOC-004` → grep-proof: exec 07 carries the actor-known positive/negative + debug-quarantine
   proof shape; exec 09 carries fixture coverage + focused golden + fixture-ID resolution; exec 10
   carries real-pipeline invocation + typed-before-rendered + checked-in render references +
   determinism-vs-golden clarification + CI command + report capture.
2. `PAR-DOC-005`/`PAR-010` → grep-proof: exec 00/03 + specs README require a parity-impact declaration +
   passing evidence before acceptance and register the standing acceptance dimension.
3. Invariants-alignment review: no new gate identifier; no passed rung's verdict changed (grep for
   absence of new gate codes; confirm no edit to existing rung verdicts).

## What to Change

### 1. `docs/2-execution/07_*` (`PAR-DOC-004`)

Own the actor-known positive/negative and debug-quarantine proof shape.

### 2. `docs/2-execution/09_*` (`PAR-DOC-004`)

Own fixture coverage, focused golden scenarios, and fixture-ID resolution.

### 3. `docs/2-execution/10_*` (`PAR-DOC-004`)

Own real-pipeline invocation, typed-before-rendered assertions, checked-in render references, the
determinism-vs-golden clarification, CI commands, and report capture.

### 4. `docs/2-execution/00_*` + `03_*` + `docs/4-specs/README.md` (`PAR-DOC-005` / `PAR-010`)

State that every Expansion feature spec must carry a parity-impact declaration + passing parity evidence
before acceptance; aggregate evidence into future phase/cert packets; run the conformance suite in
ordinary CI; register the obligation as a standing acceptance dimension that must pass before any future
feature spec, Phase-4 entry package, or second-proof package is accepted. No new gate identifier; no
change to any passed rung's verdict.

## Files to Touch

- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify)
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` (modify)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` (modify)
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` (modify)
- `docs/4-specs/README.md` (modify)

## Out of Scope

- Architecture-tier amendments (009); reference/specs-template amendments (011).
- Any new gate code or re-opening of a passed rung (spec §1.2 forbids).
- Any code change; the `docs/4-specs/SPEC_LEDGER.md` archived row (deferred to spec acceptance/archival).

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: exec 07/09/10 carry their assigned proof-shape/fixture/real-pipeline+CI+report clauses;
   exec 00/03 + specs README carry the per-feature parity-impact obligation and the standing acceptance
   dimension.
2. Grep-proof: no new gate identifier introduced; no passed rung verdict edited in exec 03.

### Invariants

1. Parity becomes a standing per-feature acceptance dimension without minting a gate code or re-opening
   any passed certification.
2. Execution tier owns proof mechanics; the spec layer owns the per-feature declaration obligation.

## Test Plan

### New/Modified Tests

1. `None — documentation-only doctrine amendment; verification is grep-based landing + invariants-
   alignment review. Pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE "parity-impact|parity|conformance|typed-before-rendered|debug-quarantine" docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md docs/4-specs/README.md`
2. `grep -nE "parity" docs/4-specs/README.md` (confirm the per-feature obligation landed)

Completed: 2026-06-22

## Outcome

Applied the execution-tier parity amendments authorized by the explicit 0046 ticket-series
implementation request. The amendment keeps parity as a standing acceptance dimension rather than a
new gate code:

1. `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` now owns actor-known
   positive/negative proof shape and debug-quarantine separation for playable-capability parity.
2. `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` now owns fixture coverage,
   focused golden scenarios, fixture-ID resolution, and hidden/stale counterpart coverage.
3. `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` now owns
   real-pipeline invocation, typed-before-rendered assertions, checked-in render references,
   determinism-vs-golden clarification, ordinary CI command lane membership, and deterministic report
   capture.
4. `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`,
   `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`, and
   `docs/4-specs/README.md` now require Expansion feature specs to carry a parity-impact declaration
   and passing parity evidence before acceptance, and to aggregate that evidence into later dependent
   phase/certification packages.

Verification:

1. `grep -nE "parity-impact|parity|conformance|typed-before-rendered|debug-quarantine" docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md docs/4-specs/README.md`
   showed the execution 00/03/07/09/10 and specs README clauses.
2. `grep -nE "parity" docs/4-specs/README.md` showed the per-feature parity-impact obligation.
3. `git diff --unified=0 -- docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md docs/4-specs/README.md`
   confirmed the sequence table and passed-rung verdict entries were not edited; the added prose
   explicitly says no new gate code, observation-obligation code, or passed-rung verdict is minted.
4. `cargo test -p tracewake-core --test doc_invariant_references --locked` passed.
5. `git diff --check` passed.
