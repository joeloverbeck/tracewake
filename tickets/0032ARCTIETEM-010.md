# 0032ARCTIETEM-010: A05/A06 bounded affect + learning/adaptation seams

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` (affect as bounded decision influence + learned-expectation seam) and `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` (affect-memory effects + learned-vs-remembered distinction). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-002 (affect/learning are provenance-bearing over the A06 temporal-claim/epistemic contract); 0032ARCTIETEM-004 (affect/learning tune salience over the A05 routine temporal premises — shared A05 surface, lands first). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-R3 (report R3) + D-R4 (report R4), both landing in A05/A06 as source-bearing cognition derived-state seams. The completeness determination routes bounded affect and learning/adaptation to A05/A06. A05 has needs, motives, candidate generation, intention lifecycle, routine failure, and planning diagnostics; A06 has memory, belief uptake, contradiction, traces, and privacy/filtering — enough substrate for affect-like and learning behavior, but **neither names affect as a bounded decision influence nor distinguishes a learned expectation from a remembered fact**.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A05 owns needs/motives/candidate generation/intention lifecycle/routine failure/planning diagnostics; A06 owns memory/belief uptake/contradiction/traces/filtering. Neither names affect-as-bounded-influence (D-R3) nor a learned-expectation-vs-raw-memory distinction (D-R4). This ticket adds both seams; it does not reinvent the needs/memory contracts.
2. Verified against spec 0032 §3.2 D-R3 + D-R4 and source report §5 Findings R3 / R4 / foundation `04`,`05`. Homes: A05 (affect influence + learned-expectation influence) + A06 (affect-memory effects + learned-vs-remembered). Additive; relaxes nothing. Affect/learning depth, update/decay rules, thresholds, and fixtures route to execution per spec §6.
3. Shared boundary under audit: A05/A06 cognition seams. **A05 is a 3-way merge hub** (tickets 004, 010, 012) and **A06 a 2-way hub** (tickets 002, 010). This ticket `Deps` on 002 (A06) and 004 (A05) so it lands after both; ticket 012 `Deps` on this one for the A05 chain. **Merge rationale**: D-R3 (affect) and D-R4 (learning) are both source-bearing derived-state seams over A05/A06 — co-locating them keeps the two cognition consolidations mutually consistent and the A05/A06 hubs single-owned per landing.
4. Constitutional invariant motivating this ticket, restated: `INV-112` — affect and learning may tune salience/method preference only after a holder-known premise is available; neither may create a hidden temporal or world fact. (The wider truth-firewall family forbids affect from revealing truth and learning from being a truth cache.)
5. No-leak / no-truth-cache surface (governed here; enforcement deferred to execution `06`/`10`): affect is a source-bearing salience/pressure modifier (over candidate generation, method selection, interruption, concealment/confession/accusation/repair tendencies, routine disruption) that may explain prioritization/avoidance but may not reveal truth, select hidden targets, bypass planning, overwrite beliefs without events, or force dramatic actions; affect-memory effects are provenance-bearing changes to salience/durability/recall-priority/belief-uptake. A learned expectation is derived state (from remembered experiences/instruction/records/testimony/repeated outcomes/institution outcomes), not a raw memory, a truth cache, or a global probability table unless source+scope are modeled; it preserves source events, scope, holder, confidence/uncertainty if represented, contradiction/staleness, and reset/decay/override provenance. Adds doctrine only; no leakage or hidden-fact path.

## Architecture Check

1. D-R3 and D-R4 are merged because both are A05/A06 source-bearing cognition derived-state seams sharing the same two docs and the same "explain-don't-fabricate" discipline; merging keeps A05/A06 single-owned per landing and the affect and learning contracts mutually consistent.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine; narrow seams, not a full emotional or learning architecture.

## Verification Layers

1. `INV-112` bounded affect (D-R3) → invariants alignment check: A05 names affect as a source-bearing salience/pressure modifier that may explain but not reveal truth / select hidden targets / bypass planning / overwrite beliefs without events / force dramatic actions; A06 names affect-memory effects as provenance-bearing.
2. `INV-112` learned expectations (D-R4) → invariants alignment check: A05/A06 distinguish a learned expectation (derived, provenance-bearing) from a raw memory / truth cache / unmodeled probability table; it influences ordering/applicability/trust/risk/skill/route/routine adaptation.
3. Two docs, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `06`/`10`); the layers above map each engaged surface to a distinct alignment proof.

## What to Change

### 1. D-R3 — bounded affect in A05 + affect-memory effects in A06

Add to A05: affect is a bounded decision influence — a source-bearing salience/pressure modifier over candidate generation, method selection, interruption, concealment/confession/accusation/repair tendencies, or routine disruption. Add to A06: affect-memory effects are provenance-bearing changes to salience, durability, recall priority, or belief uptake. Affect may explain why a holder prioritizes or avoids an option; it may not reveal truth, select hidden targets, bypass planning, overwrite beliefs without events, or force dramatic actions.

### 2. D-R4 — learned-expectation seam in A05/A06

Add: a learned expectation is derived state from remembered experiences, modeled instruction, records, testimony, repeated failures/successes, or institution-procedure outcomes; it is not a raw memory, a truth cache, or a global probability table unless its source and scope are modeled; it can influence candidate ordering, method applicability, trust, risk aversion, skill confidence, route preference, and routine adaptation; it preserves source events, scope, holder, confidence/uncertainty if represented, contradiction/staleness, and reset/decay/override provenance.

## Files to Touch

- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` (modify)
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` (modify)

## Out of Scope

- **Affect/learning depth before first proof, update/decay rules, thresholds, fixtures** — execution/implementation (spec §6).
- **Routine/social-rhythm temporal premises (A05)** — sibling ticket 004 (D-T5).
- **Temporal-claim contract (A06)** — sibling ticket 002 (D-T3).
- **Affect/learning negative-fixture proof (no hidden-truth skill/trust updates)** — execution `06`/`10` (spec §6, V4).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-R3 landing grep** — `grep -niE "affect|salience|pressure" docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` and `grep -niE "affect|salience|recall|durability" docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` resolve the affect seam.
2. **D-R4 landing grep** — `grep -niE "learned expectation|reliability|trust|skill|derived state" docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` resolves the learned-expectation seam.
3. **Invariants alignment review** — affect explains but never reveals truth / bypasses planning / overwrites beliefs without events; a learned expectation is provenance-bearing derived state, never a truth cache; upholds `INV-112`; no update-rule/threshold token introduced.

### Invariants

1. Affect is a source-bearing salience/pressure modifier that may explain but not reveal truth, select hidden targets, bypass planning, overwrite beliefs without events, or force dramatic actions (`INV-112`).
2. A learned expectation is provenance-bearing derived state distinct from raw memory and from a truth cache; it preserves source/scope/holder/contradiction/decay provenance (`INV-112`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing greps) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "affect|salience|pressure" docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — confirms D-R3 (A05) landed.
2. `grep -niE "learned expectation|reliability|trust|skill|derived state" docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — confirms D-R4 (A05) landed; the A06 affect/learning greps confirm the A06 half.
