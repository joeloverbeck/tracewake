# 0032ARCTIETEM-011: A13 temporal-firewall observability + authoring/compiler discipline

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (temporal evidence in the validation/observability contract + authoring/compiler-discipline seam). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-003 (temporal validation reports separate the temporal truth checks the A04 scheduler firewall performs from actor-visible temporal reasons). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-T10 (report T10) + D-R7 (report R7), both landing in A13's validation/observability contract. `INV-105`, `INV-111`, and `INV-112` require typed diagnostics, observer-only evidence, and temporal-authority separation; foundation `12` asks first-playable proof to include temporal-firewall proof. The completeness determination separately routes authoring/compiler discipline to A13 as validation seams that protect architecture before runtime. A13 is strong after the `0027` cascade (artifact families, observer-only emergence evidence, typed behavior witnesses) but does **not** yet name temporal evidence + budget/fairness evidence as observability families, nor compiler/static validation as a first-class architecture-protection contract.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A13 owns artifact families, observer-only emergence evidence, typed behavior witnesses, anti-vacuity, content validation, static guards, acceptance artifacts, invalid-pass conditions (the `0027` cascade) — but names no temporal-evidence family and treats validators/static checks as listed artifacts rather than a first-class architecture-protection contract. This ticket adds both; it does not reinvent A13's artifact-family or observer-only contracts.
2. Verified against spec 0032 §3 D-T10 + D-R7 and source report §5 Findings T10 / R7 / `INV-105` / `INV-111` / foundation `09`,`12`. Both homes are A13. Additive; relaxes nothing. Concrete gate names, fixture families, command output, thresholds, schemas, rule languages, error formats route to execution/specs per spec §6.
3. Shared boundary under audit: A13 validation/observability contract. **A13 is a merge hub** with sibling ticket 012 (D-R5 fairness/starvation review artifacts); this ticket lands first (012 `Deps` on it) so the temporal-evidence contract is in place before the fairness artifacts layer on. **Merge rationale**: D-T10 (temporal evidence) and D-R7 (authoring/compiler discipline) are both A13 validation/observability additions (mirroring `0027`'s A13 ticket carrying D1+D7); merging keeps A13 single-owned per landing.
4. Constitutional invariants motivating this ticket, restated: `INV-105` — actor decision traces / diagnostics must be typed or structurally inspectable (D-T10 requires temporal premises, truth-vs-reason separation, and scheduler/budget diagnostics to be typed); `INV-111` — living-world acceptance requires observer-only emergence evidence (D-T10's temporal/LOD artifacts and D-R7's review artifacts stay observer-only; the `0027` carve-out is preserved).
5. Validation / observability surface (governed here; enforcement deferred to execution `10`): D-T10 requires decision traces to identify temporal premises + provenance, validation reports to separate temporal truth checks from actor-visible temporal reasons, scheduler diagnostics to record due effects / deferred-skipped cognition / budget exhaustion / starvation-fairness / layer attribution, TUI reports to prove temporal labels came from holder-known sources, LOD/replay artifacts to preserve temporal + information ancestry, and acceptance to reject display-string-only temporal proof. D-R7 requires content/schema validators, static guards, manifest checks, and review artifacts to be architecture-protecting boundaries (reject impossible/forbidden authoring forms before runtime), with structured, layer-attributed outputs, protecting against aliases / nested forbidden concepts / display-string-only proof / hidden-truth cognition fields / player-human privilege / silent migrations / incompatible content versions / outcome chains. Architecture defines the evidence **shape** only. Adds doctrine only; no leakage path, observer-only evidence preserved.

## Architecture Check

1. D-T10 and D-R7 are merged because both extend A13's validation/observability contract and are reviewed coherently as "A13 gains two new evidence/protection families" — exactly the shape of the `0027` A13 ticket (D1 observer-only evidence + D7 typed observability). Merging keeps A13 single-owned per landing and avoids a redundant two-ticket hub on one doc.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine; defines required evidence shape and the compiler-seam contract, not tests/thresholds/schemas.

## Verification Layers

1. `INV-105` temporal observability (D-T10) → invariants alignment check: A13 adds temporal evidence (decision-trace temporal premises + provenance; truth-vs-reason separation; scheduler/budget diagnostics; TUI holder-known proof; LOD/replay temporal+information ancestry; reject display-string-only temporal proof) as typed observability families.
2. `INV-111` observer-only evidence → invariants alignment check: temporal/LOD review artifacts and D-R7 review artifacts stay observer-only; the `0027` carve-out is preserved.
3. Authoring/compiler discipline (D-R7) → invariants alignment check: A13 names content/schema validators, static guards, manifest checks, and review artifacts as architecture-protecting pre-runtime boundaries with structured, layer-attributed outputs.
4. Single doc, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `10`); the layers above map each engaged invariant to a distinct alignment proof.

## What to Change

### 1. D-T10 — temporal evidence in the A13 validation/observability contract

Add to `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`: decision traces identify temporal premises used by candidate generation/method selection and their provenance; validation reports separate temporal truth checks from actor-visible temporal reasons; scheduler diagnostics record due effects, deferred/skipped cognition, budget exhaustion, starvation/fairness symptoms, and layer attribution; TUI/view-model reports prove temporal display labels came from holder-known sources, not raw clock/debug truth; LOD/replay artifacts preserve temporal and information ancestry; acceptance artifacts reject display-string-only proof of temporal correctness. Define the required evidence **shape** only; execution owns gate names, fixtures, command output, thresholds, and pass/fail policy.

### 2. D-R7 — authoring/compiler-discipline seam in A13

Add: content/schema validators, static guards, manifest checks, and review artifacts are architecture-protecting boundaries that reject impossible or forbidden authoring forms before runtime (not runtime cleanup); validator outputs are structured and layer-attributed (field/path or authored element, violated doctrine, responsible layer, provenance/source status, author-actionable reason); the validation surface protects against aliases, nested forbidden concepts, display-string-only proof, hidden-truth cognition fields, player/human privilege, silent migrations, incompatible content versions, and outcome chains. Architecture requires the seam and evidence shape; execution/specs choose schemas, rule languages, commands, compatibility policies, and error formats.

## Files to Touch

- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)

## Out of Scope

- **Concrete gate names, fixture families, command output, thresholds, pass/fail policy, schemas, rule languages, error formats, compatibility policies** — execution/specs (spec §6).
- **Fairness/starvation review artifacts (A13)** — sibling ticket 012 (D-R5); this ticket states the temporal evidence + compiler seam, 012 the fairness artifacts.
- **Temporal-firewall / authoring-guard proof procedures** — execution `10` (spec §6, V4).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T10 landing grep** — `grep -niE "temporal premise|truth check|temporal reason|temporal ancestry|display-string" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` resolves the temporal-evidence family.
2. **D-R7 landing grep** — `grep -niE "validator|static guard|architecture-protecting|layer-attributed|before runtime" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` resolves the authoring/compiler seam.
3. **Invariants alignment review** — temporal/validation evidence is typed (`INV-105`); temporal/LOD/review artifacts stay observer-only (`INV-111`, `0027` carve-out preserved); only evidence shape (not gate names/thresholds) is defined.

### Invariants

1. Temporal premises, truth-vs-reason separation, scheduler/budget diagnostics, and LOD temporal+information ancestry are typed observability families; display-string-only temporal proof is rejected (`INV-105`).
2. Temporal/LOD/authoring review artifacts are observer-only; validators/static guards are architecture-protecting pre-runtime boundaries with structured, layer-attributed outputs (`INV-111`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (two landing greps) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "temporal premise|truth check|temporal reason|temporal ancestry|display-string" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — confirms D-T10 landed.
2. `grep -niE "validator|static guard|architecture-protecting|layer-attributed|before runtime" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — confirms D-R7 landed.
