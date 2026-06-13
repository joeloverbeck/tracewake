# Phase Ladder, Gate Order, and Certification Sequence

## Status

Live execution sequencing doctrine. Replaces the old phase ladder.

## Authority boundary

This document owns execution gate order. It does not authorize implementation shortcuts and does not assert the current code has passed any gate.

## Depends on

- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`

## Core rule

Execution order is proof order, not ambition order. A feature cannot proceed because an archived spec landed or because code exists. It proceeds when the required gates pass under live doctrine.

## Certification sequence

The sequence below uses phase-certification artifact labels. These labels
compose the canonical gates and review artifacts from `00`; they are not new
canonical gate codes unless `00` separately names them as such.

| Sequence | Gate | Purpose | Entry from current repository |
|---|---|---|---|
| 0 | `P0-DOC` | Confirm replacement foundation, architecture, and execution authority are the live basis. | Must be true for every future spec. |
| 1 | `P0-CERT` | Audit post-0008 implementation against live doctrine. | Required before Phase 4 or expansion. |
| 2 | `SPINE-CERT` | Certify event log, replay, projection, randomness, save, action pipeline, TUI/debug split, and no direct dispatch. | May be partly satisfied historically; audit must prove it. |
| 3 | `EPI-CERT` | Certify actor-known/holder-known contexts, beliefs, observations, provenance, possession parity, view models, and debug quarantine. | May be partly satisfied historically; audit must prove it. |
| 4 | `ORD-LIFE-CERT` | Certify needs, routines, intentions, no-human ordinary life, planner traces, and stuck diagnostics under the actor-known transaction. | Not presumed passed by specs `0005` through `0008`. |
| 5 | `FIRST-PROOF-CERT` | Certify the first missing-property playable proof as a coherent gate set. | Blocked until gates 1 through 4 pass. |
| 6 | `PHASE-4-ENTRY` | Permit institutions, records, wrong suspicion, reports, and local procedures. | Blocked until `FIRST-PROOF-CERT` or a narrowly scoped remediation spec says otherwise. |
| 7 | `SECOND-PROOF-ENTRY` | Permit notices, travel, regional scale, LOD expansion, and story-sifting projections. | Blocked until document `12` entry gates pass. |

## Historical phase mapping

The repository history uses Phase 1, Phase 2A, Phase 3A, and planned Phase 4 language. That language is useful for historical location. It is not gate certification.

| Historical label | Live interpretation |
|---|---|
| Phase 1 landed | Evidence for `SPINE-CERT`, not certification. |
| Phase 2A landed | Evidence for `EPI-CERT`, not certification. |
| Phase 3A landed | Evidence for `ORD-LIFE-CERT`, not certification. |
| Specs `0006` and `0007` landed | Evidence that earlier gates were too weak. |
| Spec `0008` landed | Evidence that anti-contamination must be a permanent gate. |
| Planned Phase 4 | Locked behind `PHASE-4-ENTRY`. |

## No automatic Phase 4

Phase 4 institutions, records, and wrong-suspicion work may not proceed immediately after this replacement set. The next major implementation target is `P0-CERT` unless a future spec is a narrowly scoped remediation required by `P0-CERT`.

A future spec that starts Phase 4 without naming and satisfying `P0-CERT`, `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, and `FIRST-PROOF-CERT` is invalid.

## Gate evidence requirements

Every gate artifact must include:

- exact files and seams audited;
- foundation and architecture dependencies;
- positive fixtures and negative fixtures;
- event/replay/projection evidence;
- actor-known provenance evidence;
- debug quarantine evidence;
- failure diagnostics by responsible layer;
- a statement that archived specs/tickets were used only as historical evidence;
- a list of tolerated deferrals tied to named gates.

## Gate failure handling

A failed gate must produce a remediation spec or report that names the failing layer:

- doctrine mismatch;
- content/schema validation;
- actor-known context construction;
- candidate generation;
- planning/method selection;
- proposal construction;
- scheduler ordering;
- action validation;
- event application;
- projection/replay;
- view-model rendering;
- debug quarantine;
- tests/fixtures;
- documentation status.

A failed gate may not be relabeled as a phase exception.

## Valid future spec postures

A future spec must declare exactly one posture:

1. `Certification`: proves one or more gates without expanding gameplay scope.
2. `Remediation`: fixes a named failed gate.
3. `Expansion`: adds new live scope only after required entry gates pass.
4. `Documentation alignment`: changes docs without claiming code certification.

Any spec whose posture is unclear is treated as expansion and must satisfy the strictest entry gate.
