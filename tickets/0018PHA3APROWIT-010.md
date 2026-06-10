# 0018PHA3APROWIT-010: Documentation corrections — conformance rows and exec-06 single-charge clause

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — documentation only (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`)
**Deps**: `archive/tickets/0018PHA3APROWIT-001.md`, `archive/tickets/0018PHA3APROWIT-002.md`, `archive/tickets/0018PHA3APROWIT-003.md`, `archive/tickets/0018PHA3APROWIT-005.md`, `archive/tickets/0018PHA3APROWIT-008.md`, `tickets/0018PHA3APROWIT-009.md` (each row cites a surface one of these lands); `specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (§6)

## Problem

Spec 0018 §6 requires the documentation surfaces to reflect the landed locks: the architecture conformance index needs rows for the witness-compatibility census (001), episode payload checksum coverage (003), the extended generative reachability contract — superseding the 0017 concession row ("duration terminal reachability remains covered by targeted fixtures") that ticket 008 makes stale — the seed-knowledge grammar (005), and the zero-dependency census (009); and execution doc 06 needs its single-charge clause extended to note the runtime assert covers duration (`action_effect`) charges (002). These are execution/architecture-tier clarifications recording landed mechanisms; without them the doc pack asserts a weaker lock layer than the code carries.

## Assumption Reassessment (2026-06-10)

1. Verified against current docs at `main` `a9c62e0`: `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` carries the 0017 feature rows, the audit-history overturns section, and the generative-tier concession row (the "duration terminal reachability remains covered by targeted fixtures" text is present — one grep hit); `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` carries the 0017 single-charge clause for action-emitted awake deltas.
2. Spec 0018 §6 enumerates exactly these surfaces; §8 confirms no doctrine amendment is in scope (INV-001…INV-110 applied, not changed — the exec-06 edit is an execution-tier clarification, the conformance rows are records of landed mechanisms).
3. Cross-artifact boundary under audit: the conformance index as the docs↔locks mapping — every lock the 0018 tickets land must be discoverable from the index, and no superseded concession may survive as a live claim.
4. No-amendment statement (in lieu of an INV-motivated item): this ticket changes no invariant's force or scope and no replay/visibility/no-scripting boundary; it records mechanisms that tickets 001–009 land. A future reader must not mistake the new rows for doctrine — they are conformance records, matching the 0017 row precedent.

## Architecture Check

1. A single trailing docs ticket (the cross-cutting docs shape) is cleaner than per-ticket doc edits here because the conformance rows cite surfaces across six implementation tickets and must land coherently — partial rows would record a lock layer that exists only partly, and the concession-row supersession is only true once ticket 008 lands.
2. No backwards-compatibility aliasing/shims: the superseded concession row is replaced by the new reachability-contract row (with the supersession noted inline per the index's audit-history convention), not left alongside it.

## Verification Layers

1. Row completeness -> grep-proofs against the post-implementation tree: exact-string matches for each new row's key mechanism token (witness census, episode `payload_fields` coverage, generative reachability contract, seed-knowledge grammar, zero-dependency census).
2. Concession superseded -> grep-proof: the live concession text no longer asserts targeted-fixtures-only coverage (retained only inside the supersession note, if quoted).
3. Exec-06 clause -> grep-proof: the single-charge clause names duration/`action_effect` coverage alongside window passives.
4. Single-layer ticket: documentation-only; the cited mechanisms are themselves test-enforced by tickets 001–009, so no additional proof surface applies here.

## What to Change

### 1. Conformance index rows

Add the five 0018 rows (witness-compatibility census; episode payload checksum coverage; extended generative reachability contract — superseding the concession row with an inline supersession note; seed-knowledge grammar; zero-dependency census), following the 0017 row format.

### 2. Exec-06 single-charge clause

Extend the clause to record that the runtime single-charge assert covers duration (`action_effect`) charges via the delta payloads' `elapsed_ticks`, completing the action-emitted coverage the 0017 clause began.

## Files to Touch

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)

## Out of Scope

- The acceptance artifact (ticket `0018PHA3APROWIT-011`).
- The spec's `docs/4-specs/SPEC_LEDGER.md` row and `archive/specs/` move — deferred to spec acceptance per the hardening-series staging convention (`docs/archival-workflow.md`).
- Any doctrine (`docs/0-foundation/`) edit.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -c` proofs: each of the five row tokens present in the conformance index; duration-coverage wording present in exec doc 06; the live concession claim absent outside the supersession note.
2. `cargo test -p tracewake-core --test doc_invariant_references` — doc-reference gates stay green after the edits.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every 0018 lock mechanism is discoverable from the conformance index; no superseded coverage claim survives as live text.
2. No foundation-tier document is modified.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n "zero-dependency\|witness" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
2. `cargo test --workspace`
