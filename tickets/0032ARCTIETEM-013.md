# 0032ARCTIETEM-013: A00 temporal-authority conformance index entry

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — doctrine edit to `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (compact temporal-authority conformance entry pointing to the cascade owners). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-001, -002, -003, -004, -005, -006, -007, -008, -010, -011, -012 (index capstone — the entry points to the owners each of those tickets authors; it lands once they exist so its pointers resolve). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-T1 (report T1). `INV-112` is now constitutional and foundation `03` is the primary temporal-authority home. A00 is the architecture index/conformance map, so a future subsystem author should find the architecture translation of the temporal cascade there — but A00 carries no temporal-authority entry (it is dense with prior `0027` hardening rows). The risk the report flags: temporal authority becoming discoverable only as a conformance row rather than via its real subsystem owners. This ticket adds a compact entry that **points to** the owners; it must not become the sole home of temporal authority.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A00 (`00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`) is the conformance map carrying the `0027` truth-firewall / single-charge rows but no temporal-authority entry. This ticket adds a pointer entry; it does not restate foundation doctrine or duplicate the owners' contracts.
2. Verified against spec 0032 §3.1 D-T1 and source report §5 Finding T1. D-T1's home is A00. Additive; relaxes nothing. It must not define tick/calendar/duration values (those route out per spec §6).
3. Shared boundary under audit: A00 indexes the whole temporal cascade. This ticket is the **batch capstone** — it `Deps` on every cascade ticket whose owner it points to (001 A02, 002 A03/A06, 003 A04, 004 A05, 005 A08, 006 A07/A11, 007 A12, 008 A10/A11, 010 A05/A06, 011 A13, 012 A04/A05/A12/A13) so its pointers resolve to landed contracts. (009 A09 is the R2 quantity seam, not part of the temporal cascade the A00 entry indexes; it is not a dependency.)
4. Constitutional invariant motivating this ticket, restated: `INV-112` — the A00 entry records that **architecture translates** `INV-112` into the named subsystem owners (A02 event/replay time; A03/A06 holder-known temporal claims; A04 scheduler/validation boundary; A05 routine/social rhythm; A08 institutional/procedural time; A10/A11/A07 temporal rendering, leads, speech; A12 LOD temporal ancestry; A13 observability). The entry points; it does not restate.

## Architecture Check

1. A00 is the correct home for an index/conformance entry, and a *pointer* entry (not a doctrine restatement) is the correct shape: the report explicitly warns against temporal authority living only as an A00 row. The entry routes readers to the real owners authored by the sibling tickets.
2. No backwards-compatibility aliasing/shims: additive conformance pointer; no behavior change, no duplication of the owners' contracts.

## Verification Layers

1. `INV-112` architecture translation map (D-T1) → invariants alignment check: A00 gains a compact entry stating architecture translates `INV-112` and pointing to A02/A03/A06/A04/A05/A08/A10/A11/A07/A12/A13 as owners.
2. Pointer-resolves check → codebase grep-proof: each owner the A00 entry names carries the contract its sibling ticket authored (the sibling landing greps), so the index pointers are not dangling.
3. Single doc, additive index entry: no replay/golden-fixture or skill-dry-run layer applies; the layers above map the engaged invariant and the pointer-integrity check to distinct surfaces.

## What to Change

### 1. D-T1 — temporal-authority conformance entry in A00

Add a compact entry/subsection to `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` recording the temporal-authority architecture cascade and pointing to owners: A02 (authoritative event/replay time); A03/A06 (holder-known temporal claims); A04 (scheduler/validation temporal boundary); A05 (routine/social rhythm); A08 (institutional/procedural time); A10/A11/A07 (temporal rendering, leads, speech); A12 (LOD temporal ancestry); A13 (observability). The entry states that architecture translates `INV-112`; it must not restate foundation doctrine at length, become the sole home of temporal authority, or define tick/calendar/duration values.

## Files to Touch

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- **Restating foundation temporal doctrine in A00, or making A00 the sole home** — explicitly forbidden (report T1); the entry points to owners.
- **Tick size, calendar/duration vocabulary** — execution/implementation (spec §6).
- **The owners' contracts themselves** — sibling tickets 001–012.
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T1 landing grep** — A00 carries the temporal-authority entry pointing to owners: `grep -niE "temporal authority|INV-112" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` resolves the entry.
2. **Pointer-integrity review** — each owner the entry names (A02/A03/A06/A04/A05/A08/A10/A11/A07/A12/A13) carries its sibling ticket's landed contract; no pointer dangles.
3. **Invariants alignment review** — the entry records that architecture translates `INV-112` and does not restate foundation doctrine, become the sole home, or define tick/calendar/duration values.

### Invariants

1. A00 records the temporal-authority cascade as a pointer to subsystem owners, not as the sole home of the doctrine (`INV-112`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing grep) plus a pointer-integrity and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "temporal authority|INV-112" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — confirms D-T1 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the pointer-integrity and invariants-alignment review.`
