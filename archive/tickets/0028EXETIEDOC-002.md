# 0028EXETIEDOC-002: First-proof acceptance package carries observer-only emergence evidence

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edits to `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` (an acceptance-package obligation) plus cross-reference support in `03` and `09`. No crate/code, no fixtures.
**Deps**: 0028EXETIEDOC-006 (D7 realigns the `EMERGE-OBS` observer-only emergence-evidence artifact in execution `10`; this first-proof package references that artifact, so 10's mechanism must be coherent first). **Execution-blocking precondition**: owner approval per spec 0028 §R-A.

## Problem

D2 (report E01). `INV-111` and foundation `12` now require the first proof to include retrospective observer-only emergence-evidence records *beside* the mandatory gates, but `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` reads as if the first-proof acceptance package can be complete without that artifact (verified: 0 `emergence`/`observer-only`/`EMERGE-OBS` matches in `02` at `64a8367`). `00` and `10` already declare `EMERGE-OBS`, but `02` does not make the artifact part of the first-proof acceptance packet. This ticket adds an acceptance-package obligation to `02`: any first-proof packet claiming living-world acceptance must include the observer-only emergence-evidence artifact produced under `10`, **beside but outside** the blocking gate list — preserving `EMERGE-OBS`'s non-certifying semantics.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`64a8367`): `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` enumerates the first-proof gates and scenario families but contains no `emergence`/`observer-only`/`EMERGE-OBS` token (`grep -inE` returned 0). `00` and `10` already declare `EMERGE-OBS` as a non-certifying observation obligation (so the artifact exists in the execution tier; `02` just does not require it in the acceptance packet).
2. Verified against spec 0028 (`specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D2 and the source report `reports/execution-tier-alignment-research-report.md` §E01. The emergence-evidence artifact's mechanism (fields, retrospective extraction, invalid-pass conditions) is realigned by D7 in execution `10` — sibling ticket 0028EXETIEDOC-006. This ticket consumes that artifact at the acceptance-package level; it does not define the artifact.
3. Shared boundary under audit: the first-proof acceptance contract in `02` (which artifacts the packet must carry) and the `EMERGE-OBS` mechanism in `10` (which produces the artifact). `02` references the artifact as an acceptance-package member; `10` owns its shape. `03` treats it as an artifact dependency of the relevant phases, not a new phase gate.
4. Constitutional invariant motivating this ticket, restated before trusting the narrative: `INV-111` — living-world acceptance requires replayable, observer-only emergence evidence (retrospective, event-log-ancestry-explainable, unable to feed simulation behavior / author outcomes / set dramatic objectives). Foundation `12` requires this evidence beside the mandatory gates. D2 makes the artifact a required first-proof acceptance-packet member at the execution tier.
5. Emergence-acceptance surface (the doctrine this edit governs; the enforcement mechanism is the deferred `EMERGE-OBS` surface in `10`, sibling ticket 006, not this edit): the obligation requires the artifact to be *present* in the packet but keeps it **outside the blocking gate list** — it is never a pass/fail threshold and never feeds cognition/scheduler/validators. This edit adds an acceptance-packet membership rule only; it introduces no leakage or nondeterminism path and creates no new gate. `10` (ticket 006) is the enforcer of the artifact's observer-only, non-certifying, ancestry-bound shape.

## Architecture Check

1. The obligation lives in `02` (the first-proof acceptance contract) because that is where the acceptance-packet membership is defined; placing it there — beside, not inside, the blocking gate list — keeps the artifact required-to-be-present without turning it into a certifying gate. `03` gains an artifact-dependency cross-reference (not a phase gate); `09` ensures golden/fixture families can supply semantic support where relevant.
2. No backwards-compatibility aliasing/shims: additive acceptance-packet doctrine. The `EMERGE-OBS` mechanism in `10` is untouched here; `02` references it, it does not duplicate or relocate it.

## Verification Layers

1. `INV-111` observer-only emergence evidence → invariants alignment check: `02` requires the observer-only emergence-evidence artifact in the first-proof acceptance packet, beside but outside the blocking gate list, preserving non-certifying semantics.
2. Non-gate placement → codebase grep-proof: the `02` obligation places the artifact outside the enumerated blocking-gate list (the artifact is an acceptance-packet member, not a new gate row).
3. Cross-reference coherence → codebase grep-proof: `03` references the artifact as a phase artifact dependency (not a gate); the artifact's mechanism remains owned by `10`.

## What to Change

### 1. Add an acceptance-package obligation to `02`

Add (final wording at enactment): any first-proof acceptance packet that claims living-world acceptance must include the observer-only emergence-evidence artifact produced under `10`, presented **beside but outside** the blocking gate list. The artifact is required-to-be-present, never a pass/fail threshold; it remains retrospective, observer-only, and event-log-ancestry-bound per `INV-111` / foundation `12` and the `10` mechanism.

### 2. Cross-reference support in `03` and `09`

- `03`: treat the emergence-evidence artifact as an artifact dependency of the relevant phase(s), explicitly **not** a new phase gate.
- `09`: note that golden/fixture families may supply semantic support for the emergence evidence where relevant, without making the artifact a fixture-pass threshold.

## Files to Touch

- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` (modify)
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` (modify — artifact-dependency cross-reference; shared with 0028EXETIEDOC-001 light xref, additive)
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` (modify — semantic-support note; shared with 0028EXETIEDOC-005, additive)

## Out of Scope

- **The `EMERGE-OBS` artifact mechanism** (fields, retrospective extraction, invalid-pass conditions, anti-Goodhart constraints) — execution `10`, sibling ticket 0028EXETIEDOC-006 (D7).
- **Turning the artifact into a gate or numeric threshold** — forbidden by INV-111 and spec §3 D2 / §7 R-B.
- **The reference glossary term for the emergence-evidence artifact** — `docs/3-reference/02` (spec 0026 D4, route-forward F01).
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Landing grep** — `02` requires the observer-only emergence-evidence artifact in the first-proof acceptance packet: `grep -niE "emergence|observer-only|EMERGE-OBS" docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` resolves the obligation.
2. **Non-gate placement review** — the artifact is presented beside but outside the blocking gate list in `02`, and `03` references it as a phase artifact dependency, not a new gate.
3. **Invariants alignment review** — the obligation keeps the artifact retrospective, observer-only, non-certifying (INV-111 / foundation 12); it adds an acceptance-packet membership rule, not a threshold.

### Invariants

1. The first-proof packet must carry the observer-only emergence-evidence artifact, but the artifact is never a blocking gate or pass/fail threshold (INV-111).
2. The artifact's mechanism stays owned by `10`; `02` references it without duplicating or relocating it.

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing grep) plus a non-gate-placement / invariants-alignment manual review. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "emergence|observer-only|EMERGE-OBS" docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — confirms the acceptance-package obligation landed.
2. `grep -niE "emergence|observer-only|artifact dependency" docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — confirms the `03` artifact-dependency (non-gate) cross-reference.
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for an execution-doc edit is the greps above plus the invariants-alignment review.`

## Outcome

Completed: 2026-06-13

Owner approval precondition: satisfied by the user's active `$ticket-series`
goal to implement `tickets/0028EXETIEDOC*` against
`specs/0028_EXECUTION*`.

Changed:

- Added the first-proof observer-only emergence-evidence artifact obligation to
  `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`.
- Added `03` artifact-dependency wording that keeps `EMERGE-OBS` beside gates,
  not inside the phase gate sequence.
- Added `09` fixture-contract support language so relevant fixture families can
  supply semantic support for retrospective `EMERGE-OBS` extraction without
  making it a fixture pass threshold.

Verification:

- `grep -niE "emergence|observer-only|EMERGE-OBS" docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
  resolved the acceptance-package obligation.
- `grep -niE "emergence|observer-only|artifact dependency" docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
  resolved the artifact-dependency cross-reference.
- `grep -niE "emergence|observer-only|EMERGE-OBS|semantic support" docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
  resolved the fixture semantic-support note.
- Invariants alignment review: `EMERGE-OBS` remains retrospective,
  observer-only, non-certifying, and outside the blocking gate list.

Deviations:

- None. The Rust gate pipeline was not run for this ticket because the accepted
  verification boundary is documentation grep plus invariants review.
