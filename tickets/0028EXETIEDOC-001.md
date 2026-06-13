# 0028EXETIEDOC-001: Reconcile gate / certification / observation-obligation vocabulary in execution 00

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` (a vocabulary-reconciliation note) plus light cross-reference cleanup in `02`, `03`, `06`, `07`, `08`, `09`, `10`, `11`. No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Recommended to land first** so the label classification is established before sibling tickets add new sections. **Execution-blocking precondition**: owner approval per spec 0028 §R-A — execution is tier-2 doctrine (ordinary owner approval, lighter than constitutional sign-off, but the edit must not land by convention). This ticket documents the amendment; it must not be applied to `docs/2-execution/*` until owner approval.

## Problem

D1 (report E00). `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` owns the canonical execution gate/obligation vocabulary and declares the canonical gates (`P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`) plus `EMERGE-OBS` as an observation obligation that is explicitly *not* a certification gate (verified: all nine canonical gate tokens present in `00`; `EMERGE-OBS` declared observer-only / never-a-pass-threshold). But later execution docs also use phase-certification labels (`SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `DATA-CERT`, `FIXTURE-CERT`, `DIAG-CERT`, `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY` — verified present across 1–4 execution docs each) without the index making the distinction visible enough. A reader can mistake an observation obligation for a blocking gate, or a phase-cert label for a new doctrine-bearing gate. This ticket adds a vocabulary-reconciliation note to `00` distinguishing four label classes and applies light cross-reference cleanup, inventing no new code and renaming no established label.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`64a8367`): `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` lists the nine canonical gates and declares `EMERGE-OBS` as a non-certifying observation obligation (`grep` confirmed all nine ``-quoted gate tokens in `00`, and the `EMERGE-OBS | … Observer-only; never a simulation input; never a pass/fail threshold` row). The phase-cert labels `SPINE-CERT`/`EPI-CERT`/`ORD-LIFE-CERT`/`FIRST-PROOF-CERT`/`DATA-CERT`/`FIXTURE-CERT`/`DIAG-CERT`/`PHASE-4-ENTRY`/`SECOND-PROOF-ENTRY` each appear in 1–4 execution docs (`grep -rl` counts). No label below is invented by this ticket — each already exists in the tree.
2. Verified against spec 0028 (`specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D1 and §7 R-C, and the source report `reports/execution-tier-alignment-research-report.md` §E00. Sibling precedent for an editorial doctrine-tier amendment ticket: `archive/tickets/0027ARCTIEDOC-002.md` (the A11 scoping correction).
3. Shared boundary under audit: the label taxonomy in execution `00` (canonical gates vs observation obligations vs phase-cert artifact labels vs informal shorthand) and the uses of those labels across `02`/`03`/`06`/`07`/`08`/`09`/`10`/`11`. This ticket classifies the existing taxonomy; it does not redefine any gate's semantics.
4. Constitutional invariant motivating this ticket, restated before trusting the narrative: `INV-111` (living-world acceptance requires *observer-only* emergence evidence, which must be retrospective and unable to feed simulation behavior, author outcomes, or set objectives). The reconciliation note must keep `EMERGE-OBS` legible as a non-certifying observation obligation — the classification protects INV-111's non-certifying boundary; it does not turn `EMERGE-OBS` into a gate.
5. Adjacent contradictions: none required by this ticket. Whether the phase-cert labels (`EPI-CERT`, `ORD-LIFE-CERT`, `DIAG-CERT`, …) should be formally mapped under the canonical gate names, kept as informal names, or left as distinct certification artifacts is an editorial decision the spec (§7 R-C) explicitly defers to a later reassess session — this ticket makes the distinction *visible* without forcing the mapping, so no renaming is in scope.

## Architecture Check

1. The reconciliation lives in `00` because `00` is the execution index that already owns canonical gate/obligation vocabulary; classifying the four label classes at the one authoritative index is cleaner than annotating each label at every use site. The downstream edits are light cross-reference cleanup only, not per-doc redefinition.
2. No backwards-compatibility aliasing/shims: the note classifies existing labels and invents no new code; no label is renamed and no gate semantics change.

## Verification Layers

1. `INV-111` non-certifying observation-obligation boundary → invariants alignment check: the `00` note classifies `EMERGE-OBS` as an observation obligation distinct from canonical gates and phase-cert artifacts, preserving its never-a-pass-threshold status.
2. Label-taxonomy completeness → codebase grep-proof: every label named in the `00` note resolves to an existing token in the execution docs (no invented codes); the canonical-gate set matches `00`'s existing list.
3. Single-tier editorial edit: no replay/golden-fixture or skill-dry-run layer applies; the two layers above each map an engaged concern to a distinct proof surface.

## What to Change

### 1. Add a vocabulary-reconciliation note to execution `00`

Add a note (final wording authored at enactment) that distinguishes four label classes so a reader cannot mistake one for another:

- **Canonical gates** — `P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG` (blocking certification gates the index already owns).
- **Observation obligations** — `EMERGE-OBS` (non-certifying, observer-only; blocks nothing; never a simulation input or pass/fail threshold without a dedicated future spec).
- **Phase-certification artifact labels** — `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `DATA-CERT`, `FIXTURE-CERT`, `DIAG-CERT`, `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY` (phase-scoped certification artifacts, not new doctrine-bearing gates).
- **Informal shorthand** — descriptive names used in prose that are neither gates nor obligations.

State the reader-facing rule: an observation obligation must not be read as a blocking certification gate, and a phase-cert label must not be read as a new canonical gate. Invent no new code; rename no established label.

### 2. Light cross-reference cleanup in affected execution docs

Apply minimal cleanup in `02`, `03`, `06`, `07`, `08`, `09`, `10`, `11` so each label's first/primary use is unambiguous about its class (gate vs observation obligation vs phase-cert artifact). Additive clarification only — no section restructuring, no semantic change, no renaming.

## Files to Touch

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` (modify)
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` (modify — light xref cleanup)
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` (modify — light xref cleanup)
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify — light xref cleanup)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify — light xref cleanup)
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` (modify — light xref cleanup)
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` (modify — light xref cleanup)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify — light xref cleanup; shared with 0028EXETIEDOC-003/004/005/006 — coordinate, additive only)
- `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` (modify — light xref cleanup)

## Out of Scope

- **Renaming or formally mapping phase-cert labels under canonical gate names** — deferred to a later reassess session (spec §7 R-C); this ticket only makes the distinction visible.
- **Inventing any new gate code, observation-obligation code, or invariant number** — forbidden by the spec (§2).
- **Substantive edits to the cross-referenced docs** — those are owned by sibling tickets 0028EXETIEDOC-002…008; this ticket touches them only for light label-class clarification.
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Landing grep** — `00` contains the four-class vocabulary note: `grep -niE "observation obligation|canonical gate|phase-cert|informal" docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` resolves the classification.
2. **No-invention grep** — every label named in the `00` note already exists in the execution tree: for each label token in the note, `grep -rl "<label>" docs/2-execution/` returns ≥1 file (no token unique to the note).
3. **Invariants alignment review** — the note keeps `EMERGE-OBS` classified as a non-certifying observation obligation (INV-111), introduces no new code, and renames no established label.

### Invariants

1. The `00` note classifies existing labels into the four classes without redefining any gate's semantics or coining a new code.
2. `EMERGE-OBS` remains an observation obligation distinct from canonical gates and phase-cert artifacts — never a blocking gate or pass/fail threshold (INV-111).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing + no-invention greps) plus an invariants-alignment manual review. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "observation obligation|canonical gate|phase-cert|informal" docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — confirms the four-class note landed.
2. `for l in P0-CERT TFW PIPE NO-DIRECT NO-HUMAN POS-PARITY REPLAY FIXTURE DIAG EMERGE-OBS SPINE-CERT EPI-CERT ORD-LIFE-CERT FIRST-PROOF-CERT DATA-CERT FIXTURE-CERT DIAG-CERT PHASE-4-ENTRY SECOND-PROOF-ENTRY; do n=$(grep -rl "$l" docs/2-execution/ | wc -l); echo "$l: $n"; done` — every label resolves to ≥1 existing doc (no invented code).
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected and is not the verification boundary for an execution-doc edit; the boundary is the greps above plus the invariants-alignment review.`
