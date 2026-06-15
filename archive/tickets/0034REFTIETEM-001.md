# 0034REFTIETEM-001: reference 02 glossary — epoch-2 temporal & completeness review terms

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — doctrine edit to `docs/3-reference/02_GLOSSARY.md` (additive review-aid terminology for temporal authority + completeness themes over the existing epoch-1 glossary). No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0034 §R-A (reference tier-3 doctrine; ordinary owner approval, not by convention).

## Problem

Spec 0034 D-T1, D-R1, D-R3, D-R5, D-R7, D-R9, D-S1 (report F-01, F-04, F-06, F-08, F-10, F-12, F-14), the glossary slices. The reference glossary owns the epoch-1 baseline terms (`scheduler`, `salience`, `truth firewall`, `provenance`, `institution-known`, `procedure status`, `stale information`, `EMERGE-OBS`) but carries **no** epoch-2 terminology: verified 0 matches for `temporal authority` / `holder-known temporal claim` / `validity window` / `temporal ancestry` / `information ancestry` / `time acceleration` / `LOD summary cadence` in `02_GLOSSARY.md` at `cda3325`. Without the canonical review-aid terms, a reviewer has no stable vocabulary to catch temporal-firewall, quantity/custody-lineage, bounded-affect, learned-expectation, budget/fairness, practical-bias, or staged-abstraction drift across the later specs that land below the reference tier. This ticket authors those terms — each naming the concept and its authority boundary, pointing up to foundation `03`/`INV-112` and laterally to the architecture/execution homes — additively over the epoch-1 glossary, never renaming or weakening an existing term.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`cda3325`): `docs/3-reference/02_GLOSSARY.md` owns the epoch-1/baseline terms (`grep -ioE 'EMERGE-OBS|institution-known|salience|provenance|procedure status|stale information' docs/3-reference/02_GLOSSARY.md` resolves them) and carries no epoch-2 temporal/completeness terms (targeted greps return 0). The gap is real; this ticket adds review-aid terms, it does not restate foundation/architecture/execution doctrine.
2. Verified against spec 0034 §3.1 D-T1 and §3.2 D-R1/D-R3/D-R5/D-R7/D-R9 and §3.3 D-S1, and the ratified upstream: foundation `INV-112` + foundation `03` "Temporal authority" (spec `0031`), the architecture cascade (spec `0032`), and the execution proof obligations (spec `0033`). The glossary is the home for the *terms*; concrete temporal vocabulary, thresholds, schemas, denominations, affect/learning depth, the bias vehicle, and budget numerics route to future scoped specs per spec §6.
3. Cross-artifact shared boundary under audit: the glossary (`02`, this ticket) is the **producer** of the canonical terms that the risk register (`01`, ticket 002) and the checklist (`00`, ticket 003) consume. This ticket establishes the terms; 002/003 reference them and therefore `Deps:` this ticket. Term coherence (a risk or review question must cite a term this glossary defines) is the boundary verified here.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-112` (time may validate, but holder-known time must plan); `INV-099` (truth may validate actions, but truth may not plan them); `INV-102` (cognition inputs require provenance). The terms terminologize these for review; they weaken none and create no doctrine.
5. Governed enforcement surface (doctrine ticket, no code): the terms supply the review vocabulary for the truth/temporal firewall and actor-/institution-known cognition surface (`INV-112`/`INV-099`/`INV-102`/`INV-110`/`INV-111`) that architecture/execution enforce. Per the Substrate-only / doctrine-batch rule, item 5 applies: each term names the concept's *authority boundary* (what validates vs. what may plan) and routes concrete values out, so the terminology introduces no epistemic-leakage path (no term licenses raw clock/replay/debug time, display totals, or fixture labels as holder knowledge) and no nondeterminism path. The matching relapse-risk memory is ticket 002; the review questions are ticket 003.

## Architecture Check

1. `02_GLOSSARY.md` is the correct single home: it already owns prescriptive terminology control, so the epoch-2 temporal and completeness terms are additions to a vocabulary it already maintains — not a new doc or a risk/checklist concern. Authoring the terms here (and only referencing them from `01`/`00`) preserves the single-definition discipline and the per-doc 1:1 ownership that keeps the batch merge-hub-free.
2. No backwards-compatibility aliasing/shims: additive terms over the existing epoch-1 glossary; no rename, no weakening, no redefinition of `EMERGE-OBS`, `salience`, `truth firewall`, or any baseline term.

## Verification Layers

1. `INV-112`/`INV-099` temporal-firewall vocabulary → invariants alignment check + codebase grep-proof: the glossary defines temporal authority, event/replay time, scheduler time, holder-known temporal claim, institution-known procedural time, temporal firewall, freshness/staleness as source-backed status, validity window, temporal ancestry, information ancestry, time acceleration, LOD summary cadence — each with its authority boundary.
2. `INV-102` provenance/derivation vocabulary → invariants alignment check: quantity/custody-lineage, learned-expectation, and practical-bias terms mark facts as source-backed, provenance-bearing, defeasible — not free truth.
3. Mechanism-token boundary → manual review: the authored entries name no concrete temporal value (tick size, calendar/day-part vocabulary, stale-after threshold, UI clock string, "yesterday"/"closed"/"due"), no schema/unit/denomination, no affect/learning/budget formula.
4. Documentation-only doctrine ticket: no replay/golden-fixture or skill-dry-run layer applies (those are code/spec surfaces); the layers above map each engaged invariant to a distinct review surface.

## What to Change

### 1. D-T1 — temporal-authority review terms

Add review-aid terms for: temporal authority; event/replay time; scheduler time; holder-known temporal claim; institution-known procedural time; temporal firewall; freshness/staleness as source-backed status; validity window; temporal ancestry; information ancestry; time acceleration; LOD summary cadence. Each entry names the concept **and its authority boundary** (what may validate/order vs. what may plan/decide/appear), points up to foundation `03`/`INV-112` and laterally to the architecture/execution homes, and states explicitly that concrete temporal surface vocabulary and thresholds belong to future scoped specs. No tick size, calendar/date syntax, day-part vocabulary, duration units, stale-after thresholds, office-hour representation, time-acceleration speed, simultaneity/tie-break, UI clock strings, or concrete labels.

### 2. D-R1 — quantity granularity & custody-lineage terms

Add cautious terms distinguishing unique things from countable lots and divisible stocks; distinguishing a ledger/display total from authoritative custody lineage; and requiring ancestry preservation across split, merge, transfer, consume, spoil, pay, owe, store, disclose, hide, and discover. No unit vocabularies, denominations, price/economy formulas, storage/inventory schemas, or split/merge event schemas.

### 3. D-R3 — bounded affect & salience terms

Add or extend terms for bounded affect, salience, affect-bearing memory, and affect pressure, making clear affect may alter reviewable attention/priority only through modeled sources and stays subordinate to actor-known cognition and transaction proof (no truth reveal, hidden-target selection, forced drama, belief overwrite, or planning bypass). No affect dimensions, update equations, emotional categories, intensity ranges, decay rates, or appraisal rules.

### 4. D-R5 — learned-expectation term

Add a learned-expectation term marking it as derived, source-backed, defeasible actor state distinct from records and memories; the term helps reviewers ask whether an expectation cites modeled experience and whether its scope/freshness prevents it from operating as timeless truth. No learning algorithms, trust/decay formulas, generalization rules, or update thresholds.

### 5. D-R7 — scheduler budget / starvation / fairness terms

Add diagnostic-only terms (or term notes) for scheduler budget, planner budget, budget exhaustion, deferred cognition, skipped/degraded cognition, starvation, fairness envelope, and aggregate success. Diagnostic vocabulary only — no numeric budgets, percentages, windows, priority formulas, fairness metrics, starvation thresholds, or scheduler algorithms.

### 6. D-R9 — practical bias / social position / domain-pack terms

Add cautious terms for modeled practical bias, credibility, social position, protected/social category **as a review concept**, domain-pack assumption, and wrong-suspicion evidence — stating they exist to review whether harm/bias emerges from modeled evidence/procedure/access/testimony and explicit content assumptions, not from author fiat or hidden truth. Preserve the genre-neutral kernel: no protected-category taxonomy, harm scoring, moral-truth labels, institution-pack schema, credibility algorithm, or bias-vehicle implementation. (Owner-decision per spec §7: include a generic "protected/social category" phrase now vs. defer to the future domain-pack bias vehicle — record the decision in the diff.)

### 7. D-S1 — staged-abstraction & false-certification terms

Add terms for staged abstraction and false certification/overclaiming: a staged abstraction is a declared, bounded omission or simplification in an acceptance artifact; false certification is the relapse where that omission is treated as proof of omitted future behavior. Do **not** duplicate the `0003` acceptance-artifact template field list as final wording — the concrete template edit is owned by the specs-tier `0003` route (a separate future spec, recommended `0035`). (Owner-decision per spec §7: place near `EMERGE-OBS`/acceptance-evidence terms vs. a separate evidence-honesty group — record the decision in the diff.)

## Files to Touch

- `docs/3-reference/02_GLOSSARY.md` (modify)

## Out of Scope

- **Concrete temporal values** — day-part/lateness vocabulary, calendar/date syntax, duration units, stale-after thresholds, time-acceleration speed, simultaneity/tie-break, UI clock strings (spec §6; future scoped specs).
- **Schemas, units, denominations, economy formulas, affect/learning depth, the bias vehicle, budget/fairness numerics** — future scoped specs (spec §6).
- **Relapse-risk memory** — risk register `01` (ticket 002).
- **Temporal review questions** — checklist `00` (ticket 003).
- **The `0003` acceptance-artifact template staged-abstraction edit** — specs-tier route, recommended spec `0035` (spec §6).
- **Re-opening epoch-1 reference content** — `EMERGE-OBS`, the R-26 cross-link, the R-27/28/29 cluster are baseline and untouched.
- **Owner approval itself (spec §R-A)** — execution precondition, not a deliverable.
- Crate/code, fixtures; foundation/architecture/execution edits; minting any risk identifier or treating these terms as enacting downstream doctrine.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T1 landing grep** — temporal terms present: `grep -niE 'temporal authority|holder-known temporal claim|temporal ancestry|information ancestry|validity window|time acceleration|LOD summary cadence' docs/3-reference/02_GLOSSARY.md` resolves the terms.
2. **D-R1/D-R3/D-R5/D-R7/D-R9 landing grep** — completeness terms present: `grep -niE 'custody lineage|bounded affect|learned expectation|budget exhaustion|starvation|practical bias|domain-pack assumption' docs/3-reference/02_GLOSSARY.md` resolves them.
3. **D-S1 landing grep** — staged-abstraction terms present: `grep -niE 'staged abstraction|false certification|overclaim' docs/3-reference/02_GLOSSARY.md` resolves them.
4. **Mechanism-token boundary review** — manual review confirms the added entries name no stale-after threshold, day-part/lateness vocabulary, calendar syntax, unit/denomination, affect/learning/budget formula, or schema; and mint no risk identifier.
5. **Epoch-1 preservation review** — `grep -ioE 'EMERGE-OBS|salience|truth firewall' docs/3-reference/02_GLOSSARY.md` still resolves and the baseline definitions are unchanged (no rename/weaken).

### Invariants

1. Every authored term names its authority boundary — what validates/orders vs. what may plan/decide/appear — so no term licenses raw clock/replay/debug time, display totals, or fixture labels as holder knowledge (`INV-112`/`INV-099`/`INV-110`).
2. Temporal, quantity, affect, and learned-expectation facts are named as source-backed and provenance-bearing, not free truth (`INV-102`); the genre-neutral kernel is preserved (no morality oracle).

## Test Plan

### New/Modified Tests

1. `None — documentation-only reference-doctrine ticket; verification is command-based (landing greps) plus a mechanism-token-boundary, epoch-1-preservation, and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal authority|holder-known temporal claim|custody lineage|bounded affect|learned expectation|budget exhaustion|practical bias|staged abstraction' docs/3-reference/02_GLOSSARY.md` — confirms D-T1/D-R1/D-R3/D-R5/D-R7/D-R9/D-S1 landed.
2. `rg -n 'tick|calendar|day-part|yesterday|stale-after|denomination|decay|fairness formula|R-3[0-9]' docs/3-reference/02_GLOSSARY.md` — mechanism-token/identifier boundary check (expected: no concrete value or minted risk ID in the added entries).
3. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the mechanism-token-boundary, epoch-1-preservation, and invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented the glossary portion of spec 0034 by adding compact, additive review-aid terms to `docs/3-reference/02_GLOSSARY.md` for temporal authority, temporal/firewall ancestry, holder- and institution-known temporal claims, quantity/custody lineage, bounded affect, learned expectation, budget/fairness diagnostics, practical bias/domain-pack assumptions, and staged-abstraction / false-certification evidence honesty.

The execution-blocking owner-approval precondition in spec 0034 §R-A was satisfied by the user's explicit `$ticket-series implement the series tickets/0034REFTIETEM*` instruction. Owner-decision points were resolved conservatively in the implementation: `protected/social category` is included only as a restricted review concept, and staged-abstraction / overclaiming / false-certification wording is placed with evidence-honesty terminology without changing `EMERGE-OBS` or the epoch-1 acceptance-evidence baseline.

Verification run:

- `grep -niE 'temporal authority|holder-known temporal claim|custody lineage|bounded affect|learned expectation|budget exhaustion|practical bias|staged abstraction' docs/3-reference/02_GLOSSARY.md`
- `grep -niE 'temporal authority|holder-known temporal claim|temporal ancestry|information ancestry|validity window|time acceleration|LOD summary cadence' docs/3-reference/02_GLOSSARY.md`
- `grep -niE 'custody lineage|bounded affect|learned expectation|budget exhaustion|starvation|practical bias|domain-pack assumption' docs/3-reference/02_GLOSSARY.md`
- `grep -niE 'staged abstraction|false certification|overclaim' docs/3-reference/02_GLOSSARY.md`
- `grep -ioE 'EMERGE-OBS|salience|truth firewall' docs/3-reference/02_GLOSSARY.md`
- `rg -n 'tick|calendar|day-part|yesterday|stale-after|denomination|decay|fairness formula|R-3[0-9]' docs/3-reference/02_GLOSSARY.md`
- `git diff --check`

The mechanism-token boundary review found no concrete temporal values, schemas, denominations, category taxonomy, affect/learning formula, budget formula, or new risk identifier introduced as reference doctrine. The boundary grep's matches were cross-reference wording, one explicit "fairness formulas belong to future scoped specs" exclusion, and pre-existing `tick/window` wording in `Sealed context packet`.
