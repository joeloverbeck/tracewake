# 0046TUISIMPLA-009: Architecture-tier parity amendments (arch 00 / 10 / 13)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — architecture-tier doctrine docs only (`docs/1-architecture/00_*`, `10_*`, `13_*`); no code. Execution precondition: ordinary architecture-tier owner approval before applying (doctrine amendment, not applied by convention).
**Deps**: 0046TUISIMPLA-001, 0046TUISIMPLA-002, 0046TUISIMPLA-005

## Problem

Spec 0046 §5 `PAR-DOC-001`/`PAR-DOC-002`/`PAR-DOC-003`. The two-hop parity contract is enforced in
code (tickets 001–008) but not yet owned at architecture altitude. This ticket lands the architecture
amendments so the contract is discoverable and reviewable doctrine, routed to the highest legitimate
owner and no lower — none amends an invariant or mints a gate code.

## Assumption Reassessment (2026-06-22)

1. Verified the target docs exist at baseline `1145e109`:
   `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`,
   `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`,
   `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`. The two-hop
   mechanisms they will describe land in tickets 001 (`render_embodied_view` destructure), 002
   (closed-enum exhaustive matches), 005 (real-pipeline goldens) — so the amendments describe landed
   reality, not aspiration (Deps).
2. Verified against spec 0046 §5: `PAR-DOC-001`→arch 10 (two-hop decomposition; closed core→client
   contracts are deliberately breaking when they grow; explicit presentation disposition per field +
   closed variant; every playable capability has an actor-filtered surface contract or an
   architecture-justified non-playable classification; prohibition on debug/raw truth satisfying
   embodied parity; semantic actions stay data-driven). `PAR-DOC-002`→arch 13 (a feature's playable
   capabilities form a conformance set mapping to typed/actor-knowledge/surface/rendered/negative/
   replay-no-human evidence; deterministic conformance report; a missing entry/witness is an acceptance
   failure). `PAR-DOC-003`→arch 00 (register the standing parity-completeness obligation in the
   conformance index, pointing to 10 and 13). No test filenames/snapshot libraries/command lines in
   the architecture docs (spec §5 constraint).
3. Shared boundary under audit: the architecture tier is the authority home for the client/view-model
   boundary and the acceptance/evidence contract. Arch 00 is the cross-doc index; flag it as the
   merge surface if a sibling doc-tier ticket also touches it (none in this batch). This is a
   cross-artifact doctrine ticket.
4. Invariant restated (`PAR-DOC` motivation): `INV-065`/`INV-066` TUI is a primary interface with a
   per-phase gate; `INV-069` the TUI must not implement simulation rules; `INV-095` TUI/view-model
   tests are acceptance tests. The amendments operationalize existing constitutional intent — they
   amend no invariant and mint no gate code (spec §1.2).
5. Enforcement surface governed (doctrine, substrate basis): arch 10 governs the no-leak / actor-
   knowledge client boundary and arch 13 the acceptance-evidence gate. Confirm the amendments introduce
   no leakage or nondeterminism path — they require an actor-filtered surface contract per capability,
   forbid debug/raw truth from satisfying embodied parity, and require a *deterministic* conformance
   report; they strengthen the firewall and weaken nothing.

## Architecture Check

1. Owning the two-hop decomposition and the conformance-set evidence at architecture altitude (10 and
   13), indexed by 00, is the correct tier: foundation already owns the principle (spec §3.2 — no
   foundation amendment), and execution/reference own mechanics. Splitting parity doctrine across its
   legitimate owners avoids overfitting the constitution while making the obligation reviewable.
2. No backwards-compatibility aliasing/shims: doctrine prose only; no parallel/legacy contract retained.

## Verification Layers

1. `PAR-DOC-001` → grep-proof on arch 10: the two-hop decomposition, deliberate-breakage, per-field/
   per-variant disposition, per-capability surface contract, debug-truth prohibition, and data-driven
   semantic-actions clauses are present, with no test filenames/command lines.
2. `PAR-DOC-002` → grep-proof on arch 13: the conformance-set evidence mapping, deterministic report,
   and missing-entry-is-acceptance-failure clauses are present.
3. `PAR-DOC-003` → grep-proof on arch 00: the standing parity-completeness obligation is registered in
   the conformance index pointing to 10 and 13. Invariants-alignment review: no `INV-NNN` is amended and
   no gate code is minted.

## What to Change

### 1. `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (`PAR-DOC-001`)

Add the two-hop decomposition; closed core→client presentation contracts grow by deliberate breakage;
explicit presentation disposition for every view-model field and closed presentation variant; every
playable capability has an actor-filtered surface contract or an architecture-justified non-playable
classification; debug/raw truth may not satisfy embodied parity; semantic actions remain data-driven
pipeline entries. No test filenames, snapshot libraries, or command lines.

### 2. `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (`PAR-DOC-002`)

Add that a feature's declared playable capabilities form a conformance set, each member mapping to typed
causal evidence, actor-knowledge evidence, a surface disposition, rendered evidence where playable,
negative evidence where hidden/stale states matter, and replay/no-human evidence where applicable;
require a deterministic conformance report; make a missing entry or witness an acceptance failure.

### 3. `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (`PAR-DOC-003`)

Register the standing parity-completeness obligation in the conformance index, pointing to 10 for
client/view-model boundaries and 13 for evidence closure. Concise; no execution mechanics duplicated.

## Files to Touch

- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (modify)
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- Execution/reference/specs-tier amendments — tickets 010/011.
- Any constitutional-invariant amendment or new gate/risk code (spec §1.2 forbids).
- Any code change; test filenames or command lines in architecture prose.

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: arch 10 carries the two-hop + deliberate-breakage + per-field/variant disposition +
   per-capability surface-contract + debug-truth-prohibition + data-driven-actions clauses; arch 13
   carries the conformance-set + deterministic-report + missing-entry-acceptance-failure clauses; arch
   00 registers the obligation pointing to 10/13.
2. Grep-proof: no new `INV-NNN`, risk ID, or gate code introduced in the architecture tier; no test
   filenames or command lines added to architecture prose.

### Invariants

1. The amendments operationalize existing constitutional intent without amending any invariant or
   minting a gate code.
2. Architecture remains the authority home for client/view-model boundaries (10) and acceptance
   evidence (13); foundation is unchanged.

## Test Plan

### New/Modified Tests

1. `None — documentation-only doctrine amendment; verification is grep-based landing + boundary checks
   plus an invariants-alignment review. Pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE "two-hop|deliberate|conformance set|parity" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
2. `grep -rnE "INV-[0-9]{3}|gate code|risk id" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (confirm no new invariant/gate minted)
