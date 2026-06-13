# 0027ARCTIEDOC-003: Consolidate provenance-sufficiency (A03) and the memory-freshness classifier (A06)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` (new provenance-sufficiency subsection) and `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` (cross-link to it + the observed-now/remembered freshness classifier). No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0027 §R-A (architecture tier-1 doctrine; not by convention).

## Problem

D3 (report A03/A06-A) + D4 (report A06-B). The truth-firewall doctrine (`INV-101`/`INV-102`; foundation doc 14) requires action-relevant cognition inputs to carry sufficient provenance and requires holder knowledge to distinguish current observation from remembered/stale belief — but both contracts are fragmented across A00 hardening rows rather than stated as primary subsystem contracts:

- A03 defines a sealed holder-known context packet with `provenance_edges[]` and allowed/forbidden provenance classes, but states no compact **provenance-sufficiency** rule (what makes provenance sufficient; what must fail closed when it is missing/empty/dangling/wrong-kind/label-only). Verified: 0 `sufficien` matches in A03 at `fdfd0b9`.
- The observed-now vs remembered/stale **freshness classifier** lives only in A03 (`observed_now`, A03:67) and as an A00 conformance row (the 0017 projection-freshness rule, code-locked); A06 — the epistemic-data-flow owner — carries belief acquired/last-verified/stale-after ticks but states neither the sufficiency rule nor the freshness classifier as its own contract. Verified: 0 `observed_now` matches in A06.

This ticket states both as compact primary contracts: D3 the provenance-sufficiency rule in A03 (cross-linked from A06), D4 the freshness classifier in A06.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`fdfd0b9`): A03 (`03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`) has the sealed packet with `provenance_edges[]`, allowed/forbidden provenance classes, and the `observed_now` rule (line 67) but no `provenance-sufficiency` rule (`grep -ciE "sufficien"` → 0). A06 (`06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`) carries belief acquired/last-verified/stale-after ticks but no `observed_now` token (`grep -ci "observed_now"` → 0) and no sufficiency rule. The freshness rule is also recorded as an A00 conformance row (0017 projection-freshness, with code locks like `aged_food_record_surfaces_as_remembered_belief_not_observation_001`) — confirming the rule exists and is implemented; D4 adds it to A06 as a primary contract, it does not invent behavior.
2. Verified against spec 0027 §3 D3 + D4 and source report §4.3 (A03/A06-A) / §4.4 (A06-B). D3's home is A03 with a cross-link from A06; D4's home is A06 (A03 already owns the parallel freshness rule). Both are additive consolidations; neither relaxes existing doctrine. Proof mechanics (negative fixtures, fail-closed/parity tests) route to execution `04`/`10` (spec §6, V4).
3. Shared boundary under audit: the A03 (holder-known/institution-known context construction) ↔ A06 (epistemic data flow: claims/beliefs/observation/memory/records) contract pair. A06 is touched by both D3 (a cross-link to A03's sufficiency rule) and D4 (the freshness classifier) — co-locating them in one ticket keeps A06 single-owned and the two epistemic consolidations mutually consistent.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-102` (cognition inputs require provenance sufficient for replay/debug; missing action-relevant provenance is a rejection condition) — D3 states the sufficiency boundary and the fail-closed rule; `INV-101` (actor-known context is sealed, must not contain validator-only truth) — D3's non-provenance list bars raw physical-truth lookup; `INV-026` (important beliefs need provenance) and `INV-028` (staleness is not automatically corrected) — D4's classifier preserves acquisition time/provenance and forbids restamping. Additive; no invariant weakened (spec §4).
5. Actor-knowledge / fail-closed / no-leak surface (governed here; enforcement deferred to execution `04`/`10`): D3 requires missing/empty/dangling/wrong-kind/ambiguous/forbidden-source provenance to **fail closed** before action-relevant cognition/procedure/affordance selection (still debug-available as a non-diegetic failure artifact), and bars label/boolean/display-string/fixture-name/debug-comparison/raw-truth as provenance (INV-102/INV-101 — no telepathy, no validator-only truth in cognition). D4 forbids restamping an aged fact as current observation (validation truth and debug comparison do not refresh it) and requires no-human cognition, embodied TUI view models, notebooks, and holder-known contexts to use the **same** classifier (no fresher epistemic surface for possession/UI than autonomous actors — INV-006/INV-067/INV-093). This ticket adds doctrine only; no leakage or nondeterminism path. Institution-known contexts are explicitly not exempt from D3.

## Architecture Check

1. D3 and D4 are merged into one ticket because both touch A06 (D3 a cross-link, D4 the freshness classifier) and are the same epistemic-consolidation theme (A03/A06 truth-firewall data contracts); merging keeps A06 single-owned and avoids a two-ticket merge hub. D3's primary edit is A03 and D4's is A06, but they are reviewed coherently as one epistemic-contract update.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine consolidating rules already recorded in A00 hardening rows and (for freshness) already code-locked — no behavior change, no compatibility layer.

## Verification Layers

1. `INV-102`/`INV-101` provenance sufficiency (D3) → invariants alignment check: A03 gains a sufficiency rule (≥1 modeled acquisition/derivation route per fact kind; label/boolean/display/raw-truth is not provenance; missing/dangling/wrong-kind fails closed) cross-linked from A06; institution-known contexts not exempt.
2. `INV-026`/`INV-028` memory freshness (D4) → invariants alignment check: A06 gains the observed-now-vs-remembered/stale classifier (acquisition time + provenance preserved, no restamping, parity across no-human/embodied/notebook/holder-known surfaces).
3. Both contracts already recorded as A00 conformance rows / code-locked freshness tests → codebase grep-proof: the A00 0017 projection-freshness row and its lock tests confirm D4 documents implemented behavior; D3's fail-closed posture matches the A00 provenance-witness rows.
4. Two docs, additive: no replay/golden-fixture or skill-dry-run layer applies here (those are the deferred execution `04`/`10` proof surfaces); the layers above map each engaged invariant to a distinct surface.

## What to Change

### 1. D3 — provenance-sufficiency subsection in A03 (cross-linked from A06)

Add a compact provenance-sufficiency subsection to `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`, and a cross-link to it from `06_*`. Substance:

- An input to cognition/procedure/view-model selection is provenance-sufficient only if it cites ≥1 modeled acquisition/derivation route appropriate to the asserted fact kind (observation, search/contact event, absence observation, memory of a prior modeled source, speech/testimony, record/public artifact, routine/role assignment, institutional-procedure state, LOD-summary event with ancestry, or an explicit unknown/unverified placeholder).
- A source label, boolean, display sentence, fixture/branch/test name, debug comparison, validator detail, or raw physical-truth lookup is not provenance.
- Derived facts preserve lineage (the cited source suffices to replay/debug why this holder/institution/projection may use the fact now).
- Missing / empty / dangling / wrong-kind / ambiguous / forbidden-source provenance fails closed before action-relevant use; still available to debug as a non-diegetic failure artifact.
- Institution-known contexts are not exempt.

### 2. D4 — observed-now vs remembered/stale freshness classifier in A06

Add the freshness classifier to `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` as its own epistemic-data-flow contract. Substance:

- `observed_now` is limited to the holder's current modeled perception/contact/search window.
- Older usable facts remain planning-available only as memory/belief/stale information, preserving source event, acquisition time, last verification, and provenance class.
- Selecting an old fact for a new decision does not restamp it as current observation; neither validation truth nor debug comparison refreshes it.
- No-human cognition, embodied TUI view models, notebooks, and holder-known contexts use the same classifier (no fresher epistemic surface for possession/UI than autonomous actors).

## Files to Touch

- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` (modify)
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` (modify)

## Out of Scope

- **Newtype / vector shape / storage table / schema field names / fixtures** — execution/implementation (spec §3, §6).
- **Fail-closed-provenance and freshness-parity proof procedures / negative fixtures** — execution `04`/`10` (spec §6, V4).
- **D5 observation-time affordance snapshots** (A10) — sibling ticket 0027ARCTIEDOC-004.
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D3 landing grep** — A03 carries the provenance-sufficiency rule and A06 cross-links it: `grep -niE "sufficien|fails? closed" docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` resolves the rule, and A06 references it.
2. **D4 landing grep** — A06 carries the freshness classifier: `grep -niE "observed_now|remembered|stale|restamp" docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` resolves it.
3. **Invariants alignment review** — D3 fail-closed posture matches INV-102/INV-101; D4 no-restamp + parity matches INV-026/INV-028/INV-006; no invariant weakened, no execution mechanism token introduced.

### Invariants

1. Provenance-sufficiency is fail-closed for action-relevant cognition and bars label/boolean/display/raw-truth as provenance; institution-known contexts are not exempt (INV-102/INV-101).
2. The freshness classifier preserves acquisition time/provenance, forbids restamping aged facts as current observation, and is identical across no-human/embodied/notebook/holder-known surfaces (INV-026/INV-028/INV-006).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing greps) plus an invariants-alignment manual review and a cross-check against the A00 0017 freshness conformance row. No crate/code or fixture changes.`

### Commands

1. `grep -niE "sufficien|fails? closed" docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — confirms D3 landed.
2. `grep -niE "observed_now|remembered|stale|restamp" docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — confirms D4 landed.
3. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the two landing greps plus the invariants-alignment review and the A00-0017 freshness cross-check.`
