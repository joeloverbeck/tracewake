# 0034REFTIETEM-002: reference 01 risk register — epoch-2 relapse memory & R-18 cross-link

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — doctrine edit to `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (additive temporal + completeness relapse-risk memory; an additive staged-abstraction note to the existing R-27/28/29 cluster; an epoch-2 cross-link extension to the existing R-18). No crate/code, no fixtures.
**Deps**: 0034REFTIETEM-001 (producer/consumer — the risk entries cite the canonical terms the glossary ticket defines). **Execution-blocking precondition**: owner approval per spec 0034 §R-A (reference tier-3 doctrine; ordinary owner approval, not by convention).

## Problem

Spec 0034 D-T2, D-R2, D-R4, D-R6, D-R8, D-R10, D-R11, D-S2, D-X1 (report F-02, F-05, F-07, F-09, F-11, F-13, F-16, F-15, F-17), the risk-register slices. The risk register has broad truth-firewall, debug-contamination, `R-18` schema/authoring-drift, and the R-27/28/29 acceptance-evidence cluster (verified present at `cda3325`; highest existing identifier `R-29`), but **none** of the epoch-2-specific relapse shapes. Without time-specific and completeness-specific risk memory, a maintainer reviewing a later spec has no recorded watch for clock-oracle leakage, quantity/economy lineage collapse, affect-as-hidden-truth, truth-cache learning, budget starvation hidden by aggregate success, emergent-injustice-as-author-prejudice, the play-legibility dual relapse, staged-abstraction false certification, or the epoch-2 authoring/compiler smuggling channels. This ticket adds that relapse memory — additively, authoring the staged-abstraction note alongside (not re-opening) the R-27/28/29 cluster and extending (not duplicating) the existing R-18.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`cda3325`): `docs/3-reference/01_DESIGN_RISK_REGISTER.md` tops out at `R-29` (`grep -noE 'R-[0-9]+' … | sort -t- -k2 -n | tail` → R-29), carries `R-18 — Schema and Authoring Drift` and the R-27/28/29 acceptance-evidence cluster, and has no epoch-2 temporal/completeness relapse entries. **Change-rationale (no silent retcon):** D-X1 extends `R-18` with epoch-2 smuggling channels (cross-link, not a parallel duplicate); D-S2 adds a staged-abstraction false-certification note *alongside* the epoch-1 R-27/28/29 cluster (additive — the cluster is baseline and is not re-derived, renamed, or strengthened). No existing risk is weakened.
2. Verified against spec 0034 §3.1 D-T2, §3.2 D-R2/D-R4/D-R6/D-R8/D-R10/D-R11, §3.3 D-S2, §3.4 D-X1, and the ratified upstream (`INV-112`/foundation `03` via `0031`; architecture `0032`; execution `0033`). The risk register is the home for the *failure shapes and review triggers*; concrete fixture names, thresholds, schemas, formulas, and final phrasing route to future scoped specs per spec §6. No new `R-NN` identifier is minted by this spec — final numbering is the reassess/ticket enactment's decision.
3. Cross-artifact shared boundary under audit: this ticket (`01`) **consumes** the canonical terms produced by ticket 001 (`02` glossary) — e.g. "temporal ancestry", "custody lineage", "aggregate success", "staged abstraction" — hence `Deps: 0034REFTIETEM-001`. A relapse entry must cite a term the glossary defines; that coherence is the boundary verified here. The matching review questions are ticket 003.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-112` (time may validate, but holder-known time must plan); `INV-099` (truth may validate actions, but truth may not plan them); `INV-111` (living-world acceptance requires observer-only emergence evidence — the budget/fairness and play-legibility review summaries must stay observer-only). The relapse memory protects these at review altitude; it weakens none and creates no doctrine.
5. Governed enforcement surface (doctrine ticket, no code): the risk entries are review memory for the truth/temporal firewall, the observer-only acceptance posture, and authoring/compiler validation (`INV-112`/`INV-099`/`INV-110`/`INV-111`). Per the Substrate-only / doctrine-batch rule, item 5 applies: each relapse describes a *leak/contamination shape to catch* (clock-oracle leakage, wall-clock contamination, hidden-truth learning, display-total-as-custody-truth, performance-pressure-as-director), so the memory strengthens epistemic-leakage and nondeterminism prevention and introduces no new path. D-R8/D-R11 review framing must stay observer-only and non-certifying (Risk §R-E in the spec) — the entries describe review watches, not simulation inputs or actor-known urgency.

## Architecture Check

1. `01_DESIGN_RISK_REGISTER.md` is the correct single home: it already owns the operational relapse watchlist, so epoch-2 temporal/completeness relapse memory is an addition to a register it already maintains. Authoring all risk memory here (and only referencing glossary terms from `02`) keeps per-doc 1:1 ownership and the batch merge-hub-free; extending R-18 and annotating the R-27/28/29 cluster in place (rather than minting parallel duplicates) preserves the register's low-duplication discipline.
2. No backwards-compatibility aliasing/shims: additive risk memory; R-18 gains an epoch-2 cross-link and the acceptance-evidence cluster gains an additive staged-abstraction note — neither is renamed, removed, or weakened, and no parallel duplicate risk is created.

## Verification Layers

1. `INV-112`/`INV-099` temporal-relapse memory → invariants alignment check + codebase grep-proof: the register records clock-oracle leakage, raw-wall-clock contamination, omniscient lateness/office-closed labels, UI time-acceleration leaks, debug-time-becoming-diegetic, and silent LOD temporal fill-in (D-T2).
2. `INV-111` observer-only preservation → invariants alignment check: the budget/fairness (D-R8) and play-legibility (D-R11) relapse entries keep review summaries observer-only and non-certifying; the affect (D-R4) and truth-cache (D-R6) entries keep prohibited sources out of planning.
3. Additive-not-recommission review → codebase grep-proof + manual review: R-27/28/29 and the R-26 cross-link remain present and unchanged (D-S2 is an additive note); R-18 is extended in place (D-X1), not duplicated; no new `R-NN` minted.
4. Documentation-only doctrine ticket: no replay/golden-fixture or skill-dry-run layer applies; the layers above map each engaged invariant to a distinct review surface.

## What to Change

### 1. D-T2 — temporal-relapse risk cluster

Add time-specific risks (or a temporal-relapse cluster) covering clock-oracle leakage, raw wall-clock contamination (host/environment/file/debug/transcript time treated as simulation fact), omniscient lateness/office-closed labels, UI time-acceleration leaks, debug-time-becoming-diegetic, and silent LOD temporal fill-in. Failure shapes and review triggers only — no final IDs, phrasing, fixture names, or template text.

### 2. D-R2 — quantity/economy lineage-collapse risk

Record a risk that inventory/economy convenience can collapse lineage: totals replacing event-sourced custody, fungible aggregation losing actor/institution knowledge, split/merge creating untraceable stock, display inventory becoming the fact source. May extend an authoring/schema risk rather than duplicate.

### 3. D-R4 — affect-as-hidden-truth / decorative-meter risk

Add a risk asking whether affect values were produced from modeled experience, whether they explain priority without selecting hidden targets, whether they can be replayed/diagnosed, and whether UI emotional flavor is backed by causal state rather than prose.

### 4. D-R6 — truth-cache-learning risk

Add a risk that learned expectations can become truth caches: the review trigger includes fixture labels, debug outcomes, validation failures, global success rates, or omniscient classifier outputs being stored as actor expectations without a modeled observation, memory, testimony, record, or routine experience.

### 5. D-R8 — budget/fairness relapse risks

Add risks for performance pressure acting as an invisible director (silently suppressing uninteresting/remote/inconvenient/non-player-adjacent cognition for drama or speed) and budget starvation hidden by aggregate success (global pass rates masking repeated deferral, skipped cognition, or degraded reasoning for particular actors/regions/institutions/LOD classes).

### 6. D-R10 — emergent-injustice-as-author-prejudice risk

Add a risk pushing reviewers to ask whether the content pack names assumptions, whether the evidence path is modeled, whether credibility/access/procedure are explicit, whether affected actors can possess partial or wrong knowledge, and whether the artifact avoids hidden moral truth or author verdicts.

### 7. D-R11 — play-legibility dual-relapse risk

Add a dual-relapse risk: one extreme is a correct-but-unplayable audit machine; the opposite is "playability" achieved by leaking truth, objectives, quest markers, debug labels, or omniscient summaries. Ask whether players can act through holder-known surfaces, leads, notices, and explainable procedure without seeing ground truth, and whether the artifact avoids replacing play with audit-only source inspection.

### 8. D-S2 — staged-abstraction false-certification risk (additive to the acceptance-evidence cluster)

Record that acceptance artifacts can falsely certify staged work when abstractions are buried, omitted behavior is unnamed, future blockers are obscured, evidence scope is untied to the abstraction, or failure diagnostics cannot distinguish "not implemented yet" from "implemented and broken." Author as an **additive** note alongside the epoch-1 R-27/28/29 cluster — do not re-open, rename, or strengthen it, and do not mint a new risk identifier.

### 9. D-X1 — authoring/compiler smuggling cross-link (amend R-18)

Amend the existing `R-18 — Schema and Authoring Drift` to name the epoch-2 smuggling channels: fixture labels that become holder knowledge; display totals that become custody truth; domain-pack assumptions hidden in prose; compiler defaults that select temporal/quantity semantics; staged-abstraction fields that certify omitted behavior; schemas that permit ambiguous representation where execution requires fail-closed proof. Extend the existing risk — do not create a parallel duplicate.

## Files to Touch

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- **Concrete fixture names, thresholds, schemas, fairness formulas, denominations, final risk phrasing, and any new `R-NN` identifier** — future scoped specs and the reassess/ticket enactment (spec §6, §3.5 boundary).
- **Glossary terms** — `02_GLOSSARY.md` (ticket 001); this ticket cites them, it does not define them.
- **Temporal review questions** — checklist `00` (ticket 003).
- **Re-opening epoch-1 reference content** — R-26, R-27, R-28, R-29 stay baseline; D-S2 is additive, D-X1 extends R-18 in place.
- **The `0003` acceptance-artifact template staged-abstraction edit** — specs-tier route, recommended spec `0035` (spec §6).
- **Owner approval itself (spec §R-A)** — execution precondition, not a deliverable.
- Crate/code, fixtures; foundation/architecture/execution edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T2 landing grep** — temporal-relapse memory present: `grep -niE 'clock-oracle|wall-clock|time-acceleration|debug-time|LOD temporal' docs/3-reference/01_DESIGN_RISK_REGISTER.md` resolves the cluster.
2. **D-R2/D-R4/D-R6/D-R8/D-R10/D-R11 landing grep** — completeness relapse memory present: `grep -niE 'lineage collapse|decorative|truth[- ]cache|invisible director|aggregate success|author prejudice|play[- ]legibility' docs/3-reference/01_DESIGN_RISK_REGISTER.md` resolves them.
3. **D-S2/D-X1 landing grep** — staged-abstraction note + R-18 extension present: `grep -niE 'false certification|staged abstraction|smuggl' docs/3-reference/01_DESIGN_RISK_REGISTER.md` resolves them.
4. **Additive-not-recommission review** — `grep -noE 'R-2[6-9]' docs/3-reference/01_DESIGN_RISK_REGISTER.md` still resolves R-26..R-29 with their epoch-1 content intact, and `grep -c 'R-18' docs/3-reference/01_DESIGN_RISK_REGISTER.md` confirms R-18 is extended in place (not duplicated as a new identifier).
5. **No-new-identifier + boundary review** — manual review confirms no `R-30`+ is minted, the added entries name no concrete fixture/threshold/schema/formula, and the budget/fairness and play-legibility entries keep review summaries observer-only (`INV-111`).

### Invariants

1. Each relapse names a leak/contamination/director shape to catch, never licensing raw clock/replay/debug time, display totals, fixture labels, or aggregate success as holder knowledge or simulation input (`INV-112`/`INV-099`/`INV-110`/`INV-111`).
2. Epoch-1 acceptance-evidence memory (R-26..R-29) and R-18 are preserved and only additively extended (no rename/weaken/recommission; no new identifier minted by this spec).

## Test Plan

### New/Modified Tests

1. `None — documentation-only reference-doctrine ticket; verification is command-based (landing greps + R-18/cluster-preservation greps) plus a no-new-identifier and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'clock-oracle|wall-clock|truth[- ]cache|invisible director|aggregate success|author prejudice|play[- ]legibility|false certification|smuggl' docs/3-reference/01_DESIGN_RISK_REGISTER.md` — confirms D-T2/D-R2/D-R4/D-R6/D-R8/D-R10/D-R11/D-S2/D-X1 landed.
2. `grep -noE 'R-[0-9]+' docs/3-reference/01_DESIGN_RISK_REGISTER.md | sort -t- -k2 -n | uniq` — confirms R-26..R-29 and R-18 are present and no `R-30`+ identifier was minted.
3. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the cluster/R-18 preservation greps and the no-new-identifier and invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented the risk-register portion of spec 0034 by extending `docs/3-reference/01_DESIGN_RISK_REGISTER.md` with epoch-2 temporal-authority and completeness relapse notes, adding a staged-abstraction evidence-honesty note beside the existing acceptance-evidence cluster, and extending `R-18 — Schema and Authoring Drift` in place with the epoch-2 smuggling channels.

The execution-blocking owner-approval precondition in spec 0034 §R-A was satisfied by the user's explicit `$ticket-series implement the series tickets/0034REFTIETEM*` instruction. No new `R-NN` identifier was minted; the new relapse memory is an unnumbered register note and the R-18 amendment remains part of the existing risk.

Verification run:

- `grep -niE 'clock-oracle|wall-clock|time-acceleration|debug-time|LOD temporal' docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `grep -niE 'lineage collapse|decorative|truth[- ]cache|invisible director|aggregate success|author prejudice|play[- ]legibility' docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `grep -niE 'false certification|staged abstraction|smuggl' docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `grep -noE 'R-[0-9]+' docs/3-reference/01_DESIGN_RISK_REGISTER.md | sort -t- -k2 -n | uniq`
- `grep -c 'R-18' docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `git diff --check`

Manual review confirmed the added material is relapse memory and review triggers only: it names no concrete fixture, threshold, schema, formula, denomination, or category taxonomy; it keeps the budget/fairness and play-legibility entries observer-only and non-certifying; and it preserves R-26 through R-29 as existing epoch-1 risk memory.
