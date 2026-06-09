# 0015PHA3AEVECOG-010: Cross-cutting docs — conformance index rows and TFW clarification

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Docs only — `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
**Deps**: 0015PHA3AEVECOG-003, 0015PHA3AEVECOG-005, 0015PHA3AEVECOG-006, 0015PHA3AEVECOG-007, 0015PHA3AEVECOG-008

## Problem

Spec 0015 §5.5 and §6 require documentation to track the channel changes once the implementation lands: the conformance index rows for workplace/sleep/food/route knowledge must change their allowed-source column from "builder reads with label" to "event-sourced notice/observation/record with `source_event_ids`", with new rows for completion continuity and audit enforcement and the need-band kernel-boundary decision; and §6 calls for an optional execution-tier clarification that routine-template presence is not an information channel. This is a cross-cutting docs ticket: it must land atomically once the implementation surfaces it describes exist coherently, so the doc never describes a half-migrated channel.

(§6's CLAUDE.md invariant-range bullet is **not** in this ticket: it was already applied in the working tree post-baseline — confirmed during this spec's in-session reassessment — and only needs to land with the package commit.)

## Assumption Reassessment (2026-06-09)

1. Current code/docs (verified): the conformance index is `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`; per-surface allowed-source evidence rows also appear in `reports/0014_ord_life_cert_scoped_acceptance.md`. The TFW provenance table is in `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`. Confirm the exact conformance table/rows during implementation (the file is the index+conformance home).
2. Specs/docs: spec 0015 §5.5 (conformance index update) and §6 (documentation corrections); INV-104 (routines and needs do not dispatch primitive actions directly — the clarification that routine-template presence is not an information channel follows from it; no doctrine amendment required, execution-tier clarification only).
3. Shared boundary under audit: the conformance index rows are the documented contract for the workplace/sleep/food/route allowed-source columns, which the implementation tickets change from label-read to event-sourced. This is a docs surface that depends on every implementation ticket whose contract it cites (003 channel, 005 audit enforcement, 006 embodied surfaces, 007 completion continuity, 008 kernel/content boundary).
4. INV-104 — routine structure is not an information channel; the optional §6 clarification to execution doc 04 records this explicitly so implementers do not re-treat a routine label as a knowledge source. INV-103 supports it (scheduler/routine is not a cognition authority).
5. Fail-closed / actor-knowledge surface: this ticket touches no enforcement code; it documents enforcement landed elsewhere. No leakage/determinism surface is modified. (Item applies as substrate-doc only: the conformance index is the audit-trail for the no-leak/event-sourced contracts enforced by 003–008; confirm the rows describe the enforced state, not an aspiration.)
6. Schema extension: none — markdown only. (Menu item not applicable.)

## Architecture Check

1. A single trailing docs ticket avoids a staleness window where the conformance index describes a half-migrated channel; the rows must reflect all of 003/005/006/007/008 coherently. Co-locating each row change with its implementation ticket would risk the index showing mixed old/new allowed-source semantics mid-batch.
2. No shims: docs describe the post-cutover state directly; no "transitional" row retained.

## Verification Layers

1. INV-104 → manual review: the execution-doc 04 clarification states routine-template presence is not an information channel.
2. Conformance contract → codebase grep-proof: the conformance index allowed-source column for workplace/sleep/food/route reads "event-sourced notice/observation/record with `source_event_ids`"; rows for completion continuity and audit enforcement and the need-band boundary decision exist.
3. Cross-artifact → manual review: each cited surface (003/005/006/007/008) exists in the post-implementation tree before the row is finalized.
4. Docs-only ticket — additional layer mapping is satisfied by grep-proofs against the post-implementation tree (no runtime behavior to test).

## What to Change

### 1. Conformance index rows

In `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`: change the workplace/sleep/food/route knowledge allowed-source column to "event-sourced notice/observation/record with `source_event_ids`"; add rows for completion continuity (ORD-HARD-011) and audit enforcement (ORD-HARD-009); record the need-band-threshold kernel-boundary decision (ORD-HARD-012).

### 2. TFW clarification (execution tier)

In `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`: add a sentence that routine-template presence is not an information channel (clarification per INV-104; no doctrine amendment).

## Files to Touch

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` (modify)

## Out of Scope

- Any code/test change (the implementation surfaces are owned by 003/005/006/007/008).
- The `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` move (deferred to spec acceptance/archival per `docs/archival-workflow.md` — a cross-spec follow-up, not ticketed).
- The CLAUDE.md invariant-range bullet (already applied in the working tree).
- The acceptance report (`0015PHA3AEVECOG-011`).

## Acceptance Criteria

### Tests That Must Pass

1. `grep` confirms the conformance index allowed-source column for workplace/sleep/food/route reads "event-sourced … `source_event_ids`" and that completion-continuity, audit-enforcement, and need-band-boundary rows exist.
2. `grep` confirms the execution-doc 04 clarification sentence is present.
3. The cited surfaces exist in the tree (the implementation tickets landed).

### Invariants

1. The documented allowed-source contract matches the enforced channel (event-sourced, not label-read).
2. No doctrine amendment is introduced; the clarification is execution-tier only.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep-proofs) and existing pipeline coverage of the enforced surfaces is named in the implementation tickets' Assumption Reassessment.`

### Commands

1. `grep -n "source_event_ids" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md && grep -ni "not an information channel\|routine-template" docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
