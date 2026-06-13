# 0028EXETIEDOC-003: Provenance-sufficiency fail-closed proof and one-classifier memory-freshness proof

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` (provenance-sufficiency proof + actor-known freshness proof), `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (possession/view freshness parity), and `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (review-artifact evidence requirements). No crate/code, no fixtures.
**Deps**: 0028EXETIEDOC-006 (the doc-`10` review-artifact additions here reference the general anti-vacuity / behavior-witness standard established by D8 in `10`). **Execution-blocking precondition**: owner approval per spec 0028 §R-A.

## Problem

D3 (report E02) + D4 (report E03), merged because both are actor-known epistemic-proof obligations centered on execution `04` (mirroring the architecture predecessor's A03/A06 grouping in `archive/tickets/0027ARCTIEDOC-003.md`). Architecture `03` now states a shared **provenance-sufficiency** rule and architecture `06` a single **memory-freshness classifier** (ratified by spec 0027); execution proves neither fully. `04` has a provenance-minimum table and forbidden-substitute list but not the cross-cutting fail-closed sufficiency rule across actor-known **and institution-known** facts, and no live negatives for wrong-kind/dangling/ambiguous/harness-fabricated provenance (verified: 0 `sufficien`/`fail-closed`/`wrong-kind`/`dangling` matches in `04` at `64a8367`). `04`/`07` mention staleness but not the one classifier across actor / no-human / TUI / notebook / institution surfaces, nor a restamping live negative. This ticket adds a provenance-sufficiency proof to `04`, an actor-known freshness proof to `04`, a possession/view freshness-parity proof to `07`, and the corresponding review-artifact requirements to `10`.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`64a8367`): `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` carries a `## Provenance minimum` table (`Input class | Acceptable provenance examples | Forbidden substitute`) and staleness/acquired-at fields, but `grep -inE "sufficien|fail.closed|wrong-kind|dangling|institution-known"` returned 0 — no cross-cutting fail-closed sufficiency rule and no institution-known/wrong-kind/dangling negatives. `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` requires embodied view models to derive from holder-known context + immediate perceptions but states no single freshness classifier shared across no-human/TUI/notebook/institution surfaces.
2. Verified against spec 0028 (`specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D3+D4 and the source report `reports/execution-tier-alignment-research-report.md` §E02/§E03. Upstream contracts: architecture `03` (provenance sufficiency) + `06` (freshness classifier), ratified by `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` D3/D4. The `04`/`10` fail-closed mechanics this ticket establishes are inherited by Phase-4 institution proof in `11` — sibling ticket 0028EXETIEDOC-007 (D10) `Deps` this ticket.
3. Shared boundary under audit: the actor-known cognition-input contract (`04` provenance + freshness), its possession/view parity surface (`07`), and the review-artifact evidence requirements that prove them (`10`). The doc-`10` additions here are *specific* evidence requirements that instance the *general* anti-vacuity / behavior-witness standard owned by D8 (sibling ticket 006) — hence the `Deps`.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-101` (action proposal consumes a sealed actor-known context, no validator-only truth), `INV-102` (cognition inputs require provenance sufficient for replay/debug; missing provenance is a rejection condition for action-relevant cognition), and `INV-024` (no telepathy — information reaches actors only through modeled channels). D3 proves unbacked provenance fails closed on the real path; D4 proves no surface restamps stale belief as current observation.
5. Fail-closed-validation + actor-knowledge-filtering + deterministic-replay surface: the enforcement surfaces are the real proposal/validation path (`04`) and the review artifacts (`10`). D3 requires missing/empty/dangling/wrong-kind/ambiguous/forbidden-source/harness-fabricated provenance to **fail closed** before action-relevant use — strengthening, not weakening, epistemic-leakage prevention (INV-101/102/024); D4 forbids restamping stale memory as `observed_now` by replay/possession/debug/display/harness extraction — preserving deterministic replay (INV-018) and the no-leak firewall. This edit adds proof doctrine + review-artifact requirements only: no code path, no leakage, no nondeterminism. The proposal/validation code and the `10` general standard (ticket 006) are the enforcers.

## Architecture Check

1. D3 and D4 are merged into one ticket because both are actor-known epistemic-proof obligations whose primary surface is `04`, with parallel `07`/`10` support — co-locating them keeps `04`'s anti-contamination proof coherent in one review and avoids a two-ticket merge hub on `04` (mirrors how `0027ARCTIEDOC-003` merged the A03/A06 provenance+freshness contracts). They are independently implementable but tightly cohesive.
2. No backwards-compatibility aliasing/shims: additive proof obligations. `04`'s existing provenance-minimum table is extended with a cross-cutting sufficiency rule; the freshness classifier is stated as proof doctrine. No existing gate is weakened or duplicated.

## Verification Layers

1. `INV-102` provenance sufficiency (D3) → invariants alignment check + codebase grep-proof: `04` gains a fail-closed sufficiency rule naming the seven failure modes (missing/empty/dangling/wrong-kind/ambiguous/forbidden-source/harness-fabricated) across actor-known and institution-known facts.
2. `INV-101`/`INV-024` actor-known freshness (D4) → invariants alignment check: `04` (actor-known) + `07` (possession/view parity) state the single `observed_now`-vs-remembered classifier, with the prohibition on restamping by replay/possession/debug/display/harness extraction.
3. Review-artifact evidence (D3+D4) → manual review against the `10` standard (ticket 006): `10` requires provenance-gate evidence to identify source event/projection/context with ≥1 live negative (or a reason none can exist), and freshness evidence to show acquisition time / last-verified time / source / classification with a restamping live negative.

## What to Change

### 1. `04` — provenance-sufficiency fail-closed proof (D3)

Add a subsection (final wording at enactment): every actor-known **and institution-known** action-relevant fact must have a fact-kind-appropriate provenance route; **missing, empty, dangling, wrong-kind, ambiguous, forbidden-source, or harness-fabricated** provenance must **fail closed** on the real proposal/validation path under test (not merely be a table entry). Harness-produced provenance must be real event/projection ancestry, not decorative metadata or fixture labels.

### 2. `04` — actor-known memory-freshness proof (D4)

Add a freshness-classifier proof obligation: only current modeled perception/contact/search may be `observed_now`; older facts remain remembered/believed/stale/contradicted/hearsay/record-derived/unknown, preserving source event, acquisition time, last-verified time, and provenance. Prohibit restamping by replay, possession, debug, display, or harness extraction.

### 3. `07` — possession/view freshness parity (D4)

Add a parity obligation: the same freshness classifier governs possession, embodied TUI view models, notebooks, and no-human surfaces — no surface is a fresher epistemic surface than autonomous actors, and possession/UI may not freshen knowledge by re-reading truth.

### 4. `10` — review-artifact evidence requirements (D3+D4)

Add (referencing the D8 general standard in ticket 006): provenance-gate evidence identifies source event/projection/context and includes ≥1 live negative or a stated reason none can exist; freshness evidence shows acquisition time, last-verified time, source event/provenance, and freshness classification, with a live negative where an old fact stays old despite a later replay, possession bind, debug attach, notebook display, or no-human review.

## Files to Touch

- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` (modify)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify — shared with 0028EXETIEDOC-004, additive)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify — shared hub with 0028EXETIEDOC-004/005/006; additive, references the D8 standard in 006)

## Out of Scope

- **The general anti-vacuity / behavior-witness standard in `10`** — D8, sibling ticket 0028EXETIEDOC-006 (this ticket's `10` additions instance it).
- **Observation-time snapshot / wallhack-negative / carrier-census proof in `07`/`10`** — D5, sibling ticket 0028EXETIEDOC-004.
- **Phase-4 institution-known provenance/freshness future-proofing in `11`** — D10, sibling ticket 0028EXETIEDOC-007 (inherits these `04`/`10` mechanics).
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Provenance-sufficiency landing grep** — `04` names the seven failure modes failing closed: `grep -niE "fail.closed|missing|empty|dangling|wrong-kind|ambiguous|forbidden-source|harness-fabricated|institution-known" docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` resolves the sufficiency subsection with all seven modes plus institution-known coverage.
2. **Freshness-classifier landing grep** — `04` and `07` state the one classifier + no-restamping rule: `grep -niE "observed_now|restamp|freshness classifier|last.verified" docs/2-execution/04_*.md docs/2-execution/07_*.md` resolves the classifier in `04` (actor-known) and `07` (possession/view parity).
3. **Invariants alignment review** — D3 makes unbacked provenance fail closed (INV-101/102); D4 prevents restamping stale memory as current observation across all surfaces (INV-024/INV-018). Neither weakens any existing gate.

### Invariants

1. Unbacked actor-known and institution-known facts fail closed on the real proposal/validation path — not merely a table entry (INV-102).
2. No actor/possessed view/notebook/no-human/institution surface restamps old memory as current observation; possession is not a fresher epistemic surface (INV-024, INV-101).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (provenance + freshness landing greps) plus an invariants-alignment manual review against the architecture 03/06 contracts. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "fail.closed|wrong-kind|dangling|ambiguous|forbidden-source|harness-fabricated|institution-known" docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — confirms the provenance-sufficiency subsection with all failure modes.
2. `grep -niE "observed_now|restamp|freshness|last.verified" docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — confirms the freshness classifier in `04` and parity in `07`.
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for an execution-doc edit is the greps above plus the invariants-alignment review against architecture 03/06.`
