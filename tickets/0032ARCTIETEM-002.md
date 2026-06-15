# 0032ARCTIETEM-002: A03/A06 holder-known temporal claims, provenance, and freshness

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` (temporal-claim subsection) and `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` (parallel epistemic-data-flow rule + cross-link). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-001 (A02 exports temporal facts to holders only via events/projections — the substrate A03/A06 temporal claims consume). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-T3 (report T3). `INV-112` makes deadline, lateness, staleness, expected-by-now, yesterday, and office-closed states **claims / procedure states / holder-or-institution-known interpretations with provenance, not free truth labels**; foundation `04` treats temporal expressions, acquisition time, event time, record time, and freshness/staleness as claim/qualifier issues rather than silent clock updates. A03 already defines sealed holder-known context and provenance sufficiency, and A06 already owns claims/observations/beliefs/memory/records/freshness — but neither names **temporal claim shape** as a first-class specialization: verified 0 temporal-claim / temporal-provenance matches in A03 at `ea6a05b`. The prior `0027` cascade deliberately strengthened A03/A06 and must not be reopened; this ticket adds a temporal specialization on top of it.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A03 carries the sealed holder-known packet + provenance-sufficiency rule (from `0027`) but no temporal-claim subsection (`grep -niE "temporal claim|temporal-claim|temporal provenance"` → 0). A06 carries claims/beliefs/memory/freshness but states no temporal-claim slot structure as its own contract. This ticket adds the specialization; it does not reinvent the A03/A06 provenance/freshness contracts that `0027` already landed.
2. Verified against spec 0032 §3.1 D-T3 and source report §5 Finding T3 / `INV-112` / foundation `04`. D-T3's home is A03 (primary) cross-linked from A06 (parallel epistemic rule). Additive; relaxes nothing. Concrete shape (struct, field names, stale-after numbers, "morning/evening" vocabulary, calendar syntax) routes out per spec §6.
3. Shared boundary under audit: the A03 (holder-known/institution-known context construction) ↔ A06 (epistemic data flow) pair. A06 is **also touched by sibling ticket 010** (affect/learning) — this ticket lands first (010 `Deps` on it) so A06's temporal-claim contract is in place before affect/learning reference it; coordinate the A06 merge.
4. Constitutional invariant motivating this ticket, restated: `INV-112` — temporal facts may inform cognition only when they reached the relevant holder through modeled channels; deadline/lateness/staleness/yesterday/office-closed are claims with provenance, not truth labels. D-T3 states that any temporal input to cognition/procedure/affordance/speech/lead/LOD must be addressable in the holder-known context with fact-kind-appropriate provenance.
5. Actor-knowledge / no-leak surface (governed here; enforcement deferred to execution `04`/`10`): the contract requires temporal status to be a **source-backed claim/procedure state**, not a display label or true clock read, preserving distinct slots where relevant — event time asserted/inferred, acquisition time, last verification, record/procedure time, valid/due window, stale risk, contradiction status, source lineage. This specializes the `0027` provenance-sufficiency / freshness firewall to time (`INV-112` truth firewall applied to time); it adds doctrine only, no leakage or nondeterminism path.

## Architecture Check

1. D-T3 lands in A03 (primary) with a parallel rule + cross-link in A06 because the two docs already own the holder-known-context and epistemic-data-flow halves of the same contract; co-locating the temporal specialization with the `0027` provenance/freshness contracts keeps the truth-firewall family single-sourced rather than minting a new temporal-only home.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine specializing rules A03/A06 already carry.

## Verification Layers

1. `INV-112` holder-known temporal claims (A03) → invariants alignment check: A03 gains a temporal-claim subsection requiring any temporal cognition/procedure/affordance/speech/lead/LOD input to be addressable in the holder-known context with fact-kind-appropriate provenance.
2. `INV-112` epistemic data flow (A06) → invariants alignment check: A06 gains the parallel rule preserving distinct temporal slots (event/acquisition/verification/record/due time, stale risk, contradiction, lineage) and cross-links A03.
3. Two docs, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `04`/`10`); the layers above map each engaged surface to a distinct alignment proof.

## What to Change

### 1. D-T3 — temporal-claim subsection in A03 (cross-linked from A06)

Add to `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`: any temporal input used by cognition, procedure, affordance selection, speech interpretation, lead interpretation, or LOD promotion must be addressable inside the holder-known context with fact-kind-appropriate provenance. Temporal status is a source-backed claim/procedure state, not a display label or true clock read.

### 2. D-T3 — parallel epistemic-data-flow rule in A06

Add to `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`, cross-linked to A03: temporal claims preserve distinct slots where relevant — event time asserted or inferred, acquisition time, last verification, record/procedure time, valid/due window, stale risk, contradiction status, and source lineage. No struct, field names, stale-after numbers, day-part vocabulary, or calendar syntax.

## Files to Touch

- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` (modify)
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` (modify)

## Out of Scope

- **Struct/field names, stale-after thresholds, day-part / calendar vocabulary, claim-decay policy** — execution/implementation (spec §6).
- **Temporal-provenance fail-closed / freshness proof procedures** — execution `04`/`10` (spec §6, V4).
- **Affect/learning A06 effects** — sibling ticket 010.
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T3 A03 landing grep** — `grep -niE "temporal claim|source-backed|provenance" docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` resolves the temporal-claim subsection.
2. **D-T3 A06 landing grep** — `grep -niE "temporal claim|acquisition time|stale risk|lineage" docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` resolves the parallel rule + cross-link.
3. **Invariants alignment review** — both contracts uphold `INV-112` (temporal facts source-backed, not truth labels); the `0027` provenance/freshness contracts are unchanged; no execution mechanism token introduced.

### Invariants

1. Any temporal input to cognition/procedure/affordance/speech/lead/LOD is addressable in the holder-known context with fact-kind-appropriate provenance (`INV-112`).
2. Temporal claims preserve distinct event/acquisition/verification/record/due slots, stale risk, contradiction status, and lineage; temporal status is never a raw clock read or display label (`INV-112`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (two landing greps) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "temporal claim|source-backed|provenance" docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — confirms D-T3 landed in A03.
2. `grep -niE "temporal claim|acquisition time|stale risk|lineage" docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — confirms D-T3 landed in A06.
