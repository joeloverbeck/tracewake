# Spec 0034 — Reference-Tier Temporal-Authority and Completeness Alignment Amendment

This spec **proposes a set of reference-tier (`docs/3-reference/*`) amendments**. It is a
design/proposal artifact: it specifies the *substance and home* of each amendment so Tracewake's
reassess / ticket process can author the final reference wording. **It does not itself author
ratified reference prose, and it mints no new risk-register identifiers (`R-30`, …), glossary term
tokens, or checklist codes.** Reference is tier-3 doctrine but is not constitutional, so enactment
requires ordinary owner approval rather than the constitutional sign-off a foundation amendment
demands (cf. `archive/specs/0029_REFERENCE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`,
`archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md`,
`archive/specs/0033_EXECUTION_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md`).

> Section set: this file uses the canonical `specs/` section set (Problem Statement, Approach,
> Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions) because
> `specs/` carries no template at authoring time and this is not a hardening implementation spec.
> It deliberately does **not** copy the foundation-pack docs' narrative house style.

**Status:** COMPLETED. Additive amendments to `docs/3-reference/*` enacted by the
`0034REFTIETEM` ticket series.

**Admissibility posture:** `P0-CERT not applicable`. This is a doctrine-alignment proposal; it
certifies no code and performs no gate audit.

**Authority:** A reference document is subordinate to foundation, architecture, and execution and
*may not outrank or redefine them* (`docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`).
This spec does not exercise that forbidden authority. It **operationalizes higher-tier doctrine** —
the temporal-authority invariant `INV-112` and the foundation `03` "Temporal authority" section
(ratified by archived spec `0031`), the architecture temporal cascade and completeness themes
(ratified by `0032` into `docs/1-architecture/*`), and the execution proof obligations and
staged-incompleteness discipline (ratified by `0033` into `docs/2-execution/*`) — by staging
*additive* review-aid terminology, relapse-risk memory, and checklist pointers so the reference tier
can help reviewers catch drift across later specs. No deliverable ratifies concrete values or
restates upstream doctrine as if reference were constitutional authority; the final wording, once
authored, is reference-tier review aid and this spec becomes historical provenance.

**Provenance:** derived from `reports/reference-tier-alignment-research-report.md` (external deep
research, pinned to commit `cda3325b0777f25101c9a04af3daeef24913f137` = current `HEAD` `cda3325`,
the merge of the `0033` enactment) and its shared brief
`reports/reference-and-specs-tier-alignment-research-brief.md`. The report is the planned
`docs/3-reference/*` session of the temporal-authority + foundations-completeness cascade (epoch-2).
Because the report's pinned commit equals current `HEAD`, no intervening-commit drift applies. The
report's verdict-critical premises were independently re-verified against live `HEAD` during
authoring (see Verification §V1); per the deep-research-spec convention, any fabricated `#Lxxxx`
line anchors are ignored in favor of named symbols and sections.

---

## 1. Problem Statement

Foundation `0031` promoted **temporal authority** to constitutional doctrine: `INV-112` ("Time may
validate, but holder-known time must plan") plus a foundation `03` "Temporal authority" section.
Architecture `0032` translated that doctrine and the foundation completeness-routed themes into
subsystem contracts. Execution `0033` then translated both into execution proof obligations, fixture
families, diagnostics, and a staged-incompleteness declaration discipline. The temporal firewall is
the truth firewall applied to time: the scheduler/replay clock may **order and validate**; cognition,
routines, institutional procedure, embodied views, speech, leads, and LOD promotion may use temporal
facts only when those facts reached the holder through **modeled channels**. All three upstream
tiers are now ratified and archived.

The **reference tier has not yet absorbed that delta.** Verified against live `HEAD` `cda3325`, the
reference docs are still on their epoch-1 cascade (last realigned by `0029`): the glossary
(`02_GLOSSARY.md`) owns general terms (`scheduler`, `salience`, `truth firewall`, `provenance`,
`institution-known`, `procedure status`, `stale information`, `EMERGE-OBS`) but **no** distinct terms
for event/replay time vs. holder-known temporal claims, scheduler time, institution-known procedural
time, temporal/information ancestry, time acceleration, LOD summary cadence, quantity/custody lineage,
bounded affect, learned expectations, scheduler budget/fairness, practical bias, or staged
abstraction. The risk register (`01_DESIGN_RISK_REGISTER.md`, R-00…R-29) has broad truth-firewall,
debug-contamination, `R-18` schema/authoring-drift, and the R-27/R-28/R-29 acceptance-evidence
cluster, but **none** of the epoch-2-specific failure shapes. The checklist
(`00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`) enforces source discipline and tier boundaries but
asks no temporal-firewall review questions. A reviewer cannot, from the reference tier as it stands,
catch epoch-2 temporal or completeness drift across the later specs that will land below it.

This is a **delta, not a cold start.** The reference-tier alignment report dispositions **seventeen
findings** across three drivers, every one an `add` or `amend` over existing coverage — no finding is
a blank hole, and none reopens the settled epoch-1 reference work (`0029`: the `EMERGE-OBS` term, the
R-26 archive cross-link, the R-27/R-28/R-29 acceptance-evidence cluster):

- **Driver T — temporal authority (F-01…F-03).** Glossary terms, relapse-risk memory, and checklist
  pointers for the now-landed temporal firewall (`INV-112` / foundation `03`; architecture/execution
  cascades), at review-aid altitude.
- **Driver R — completeness themes (F-04…F-13, F-16).** Quantity/granularity/fungibility/custody
  lineage, bounded affect/salience, learned expectations, scheduler budget/starvation/fairness,
  practical bias/social position/domain-pack assumptions, and the dual play-legibility relapse each
  need cautious review terminology and/or relapse-risk memory over the existing glossary/risk
  substrate, conditioned on what `0032`/`0033` actually locked (and deferring affect/learning depth,
  bias vehicle, and budget numerics to future scoped specs).
- **Driver S — staged incompleteness (F-14, F-15).** Staged-abstraction / false-certification
  terminology and an additive false-certification relapse note, without duplicating the `0003`
  template field list as final wording or re-commissioning the epoch-1 evidence-honesty cluster.
- **Cross-cutting (F-17).** Extend (not duplicate) the existing `R-18` authoring/schema-drift risk to
  name the epoch-2 smuggling channels.

None of these is a foundation, architecture, or execution hole — `INV-112`, the architecture cascade,
the execution proof obligations, and the completeness routing are all ratified. Each is a
**reference-tier review-aid** gap: the upper tiers own the doctrine, proof obligation, and gate
procedure; reference owns the stable terminology, relapse memory, and review questions that help
maintainers spot drift — while concrete vocabulary, thresholds, schemas, fixtures, and depth route
*forward* to future scoped specs.

## 2. Approach

Stage seventeen compact, additive reference-tier refinements (Block T: D-T1…D-T3; Block R:
D-R1…D-R11; Block S: D-S1…D-S2; cross-cutting: D-X1), each landing in the reference doc that *owns*
the surface — glossary terms in `02`, relapse-risk memory in `01`, review questions in `00`. Keep
every change at the `name the concept and its authority boundary / describe the failure shape and
review trigger / point upward and laterally to the governing doc` level. Each glossary entry names
the *concept and authority*, points up to foundation `03`/`INV-112` and laterally to the specific
architecture/execution homes rather than redefining them, and states that concrete temporal surface
vocabulary, thresholds, schemas, and depth belong to future scoped specs. Each risk entry describes a
failure mode and review trigger, not a final identifier, final phrasing, or fixture name.

Route every concrete value **out** of reference to future scoped specs `docs/4-specs/*`: day-part /
"yesterday" / "last night" / "office closed/open" / "due/late" / "stale" vocabulary, calendar/date
syntax, duration units, stale-after thresholds, office-hour representation, time-acceleration speed,
simultaneity/tie-break rules, UI clock strings, inventory/economy schemas, unit vocabularies, money
denominations, economy formulas, affect/learning dimensions and update/decay rules, the domain-pack
bias vehicle and category taxonomy, and fairness formulas/budget numerics. The report's
forward-routing appendix is degenerate (reference is a terminal tier): nothing routes further down;
remaining actions route only to owner/reassess decisions and future implementation specs.

This spec stages **all three drivers in one package** because all three are reference-tier,
documentation-only, and derive from the same report; the report itself frames a single reference
amendment, and the user's directive was for "a spec." If an owner prefers, the blocks are cleanly
separable (Block T temporal / Block R completeness / Block S staged) — the deliverable IDs and target
homes are disjoint enough to split without rework (Risk §R-B). Final reference wording — and any
decision to mint a new risk-register identifier or glossary token — is authored on enactment (by
reassess / ticket), not in this spec, following the `0029` reference-cascade precedent.

## 3. Deliverables

All deliverables are **proposed amendment substance**, to be authored into the named reference file
by the reassess / ticket process. None prescribes final wording, risk identifiers, glossary tokens,
checklist codes, fixture names, schemas, vocabularies, thresholds, or algorithms. Each must be
authored *additively* over the existing epoch-1 reference content, never as a rename, weakening, or
replacement of it; the `EMERGE-OBS` term, the R-26 archive cross-link, and the R-27/R-28/R-29
acceptance-evidence cluster are the current baseline and are not re-opened.

### 3.1 Block T — temporal authority (terminologizes `INV-112` + foundation `03`; tracks architecture/execution cascades)

| # | Report finding | Target file | Kind | Substance |
|---|---|---|---|---|
| D-T1 | F-01 | `02_GLOSSARY.md` | Addition (review-aid terminology) | Add review-aid terms for temporal authority, event/replay time, scheduler time, holder-known temporal claim, institution-known procedural time, temporal firewall, freshness/staleness as source-backed status, validity window, temporal ancestry, information ancestry, time acceleration, and LOD summary cadence. Each entry names the concept **and its authority boundary**, points up to foundation `03`/`INV-112` and laterally to the architecture/execution homes, and states that concrete temporal surface vocabulary and thresholds belong to future scoped specs. **Must not** define tick size, calendar/date syntax, day-part vocabulary, duration units, stale-after thresholds, office-hour representation, time-acceleration speed, simultaneity, queue tie-breaks, UI clock strings, or concrete labels ("late," "closed," "due," "yesterday," "last night"). Enactment owner-decides single temporal block vs. interleaving with existing truth-firewall/scheduler/stale terms (Open Question §7). |
| D-T2 | F-02 | `01_DESIGN_RISK_REGISTER.md` | Addition (temporal-relapse risk memory) | Add time-specific risks (or a temporal-relapse cluster) covering: clock-oracle leakage (authoritative scheduler/replay time becoming a planning premise); raw wall-clock contamination (host/environment/file/debug/transcript time treated as simulation fact); omniscient lateness/office-closed labels (labels reflecting truth rather than holder/institution-known procedure state); UI time-acceleration leaks (player-facing acceleration/skip revealing facts not modeled for the holder); debug-time-becoming-diegetic (validation/debug panels bleeding into embodied views or speech); silent LOD temporal fill-in (summaries generating holder-known time facts without temporal + information ancestry). Describe failure shapes and review triggers only — no final IDs, phrasing, fixture names, or template text. |
| D-T3 | F-03 | `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | Amendment (review pointers, not new doctrine) | Add a compact temporal-authority review block asking whether every planning/procedure/view/speech/lead/LOD temporal premise has a holder-known or institution-known source; whether validator/replay/scheduler time is separated from diegetic cognition; whether time acceleration and debug panels are quarantined from embodied surfaces; and whether temporal ancestry is preserved when summaries, snapshots, projections, or compacted history are used. Pointers to the governing docs, not redefinitions. |

### 3.2 Block R — completeness-routed review terminology and risk memory

| # | Report finding | Target file | Kind | Substance |
|---|---|---|---|---|
| D-R1 | F-04 | `02_GLOSSARY.md` | Addition (quantity / custody lineage terms) | Add cautious terms for quantity granularity and custody lineage: distinguish unique things from countable lots and divisible stocks; distinguish a ledger/display total from authoritative custody lineage; and require ancestry preservation across split, merge, transfer, consume, spoil, pay, owe, store, disclose, hide, and discover operations. **Must not** choose unit vocabularies, denominations, price/economy formulas, storage/inventory schemas, split/merge event schemas, or content error formats. |
| D-R2 | F-05 | `01_DESIGN_RISK_REGISTER.md` | Addition or amend (quantity/economy lineage-collapse) | Record a risk that inventory/economy convenience can collapse lineage: totals replacing event-sourced custody, fungible aggregation losing actor/institution knowledge, split/merge creating untraceable stock, or display inventory becoming the fact source. May extend an existing authoring/schema risk rather than duplicate. |
| D-R3 | F-06 | `02_GLOSSARY.md` | Addition (bounded affect / salience) | Add or extend terms for bounded affect, salience, affect-bearing memory, and affect pressure, making clear that affect may alter **reviewable** attention/priority only through modeled sources and must remain subordinate to actor-known cognition and transaction proof — it may not reveal truth, select hidden targets, force drama, overwrite beliefs, or bypass causal planning. **Must not** decide affect dimensions, update equations, emotional categories, intensity ranges, decay rates, or appraisal rules (deferred to future spec). |
| D-R4 | F-07 | `01_DESIGN_RISK_REGISTER.md` | Addition (affect relapse memory) | Add a risk covering affect-as-hidden-truth and decorative-meter affect: ask whether affect values were produced from modeled experience, whether they can explain priority without selecting hidden targets, whether they can be replayed/diagnosed, and whether UI emotional flavor is backed by causal state rather than prose. |
| D-R5 | F-08 | `02_GLOSSARY.md` | Addition (learned expectation) | Add a learned-expectation term marking it as derived, source-backed, defeasible actor state distinct from records and memories; help reviewers ask whether an expectation cites modeled experience and whether its scope/freshness prevents it from operating as timeless truth. Defer learning algorithms, trust/decay formulas, generalization rules, and update thresholds to future specs. |
| D-R6 | F-09 | `01_DESIGN_RISK_REGISTER.md` | Addition (truth-cache-learning) | Add a risk that learned expectations can become truth caches: review trigger includes fixture labels, debug outcomes, validation failures, global success rates, or omniscient classifier outputs being stored as actor expectations without a modeled observation, memory, testimony, record, or routine experience. |
| D-R7 | F-10 | `02_GLOSSARY.md` | Addition (budget / fairness diagnostic terms) | Add diagnostic-only terms (or term notes) for scheduler budget, planner budget, budget exhaustion, deferred cognition, skipped/degraded cognition, starvation, fairness envelope, and aggregate success. **Must not** choose numeric budgets, percentages, windows, priority formulas, fairness metrics, starvation thresholds, or scheduler algorithms. |
| D-R8 | F-11 | `01_DESIGN_RISK_REGISTER.md` | Addition (budget/fairness relapse) | Add risks for performance pressure acting as an invisible director (silently suppressing uninteresting/remote/inconvenient/non-player-adjacent cognition to create drama or speed) and budget starvation hidden by aggregate success (global pass rates masking repeated deferral, skipped cognition, or degraded reasoning for particular actors, regions, institutions, or LOD classes). |
| D-R9 | F-12 | `02_GLOSSARY.md` | Addition (practical bias / social position / domain-pack) | Add cautious terms for modeled practical bias, credibility, social position, protected/social category **as a review concept**, domain-pack assumption, and wrong-suspicion evidence, stating they exist to review whether harm/bias emerges from modeled evidence/procedure/access/testimony and explicit content assumptions, not from author fiat or hidden truth. **Must not** choose a protected-category taxonomy, harm scoring, moral-truth labels, institution-pack schema, credibility algorithm, or bias-vehicle implementation (preserve the genre-neutral kernel; no morality oracle). Enactment owner-decides whether to include a generic "protected/social category" phrase now or defer to the future domain-pack bias vehicle (Open Question §7). |
| D-R10 | F-13 | `01_DESIGN_RISK_REGISTER.md` | Addition (emergent-injustice-as-author-prejudice) | Add a risk pushing reviewers to ask whether the content pack names assumptions, whether the evidence path is modeled, whether credibility/access/procedure are explicit, whether affected actors can possess partial or wrong knowledge, and whether the artifact avoids hidden moral truth or author verdicts. |
| D-R11 | F-16 | `01_DESIGN_RISK_REGISTER.md` | Addition (play-legibility dual relapse) | Add a dual-relapse play-legibility risk: one extreme is a correct-but-unplayable audit machine; the opposite is "playability" achieved by leaking truth, objectives, quest markers, debug labels, or omniscient summaries. Ask whether players can understand and act through holder-known surfaces, leads, notices, and explainable procedure **without** seeing ground truth, and whether the artifact avoids replacing play with audit-only source inspection. (Especially important at the reference tier, which helps reviewers notice drift across many later specs.) |

### 3.3 Block S — staged-incompleteness review terminology and risk memory

| # | Report finding | Target file | Kind | Substance |
|---|---|---|---|---|
| D-S1 | F-14 | `02_GLOSSARY.md` | Addition (staged abstraction / false certification) | Add terms for staged abstraction and false certification/overclaiming: a staged abstraction is a declared, bounded omission or simplification in an acceptance artifact; false certification is the relapse where that omission is treated as proof of omitted future behavior. **Must not** duplicate the `0003` template field list as final wording — the concrete template edit is owned by the specs-tier `0003` route (a separate future spec; see §6). Enactment owner-decides whether these terms sit near `EMERGE-OBS`/acceptance-evidence terms or in a separate evidence-honesty group (Open Question §7). |
| D-S2 | F-15 | `01_DESIGN_RISK_REGISTER.md` | Addition (clearly additive to the acceptance-evidence cluster) | Record that acceptance artifacts can falsely certify staged work when abstractions are buried, omitted behavior is unnamed, future blockers are obscured, evidence scope is not tied to the abstraction, or failure diagnostics cannot distinguish "not implemented yet" from "implemented and broken." Author as an **additive** note alongside the epoch-1 R-27/R-28/R-29 evidence-honesty cluster — those remain baseline and are **not** re-commissioned; do **not** invent a new risk identifier in this spec. |

### 3.4 Cross-cutting — authoring/compiler smuggling cross-link

| # | Report finding | Target file | Kind | Substance |
|---|---|---|---|---|
| D-X1 | F-17 | `01_DESIGN_RISK_REGISTER.md` | Amendment (cross-link, not duplicate) | Amend the existing `R-18` "Schema and Authoring Drift" risk to name the epoch-2 smuggling channels: fixture labels that become holder knowledge; display totals that become custody truth; domain-pack assumptions hidden in prose; compiler defaults that select temporal/quantity semantics; staged-abstraction fields that certify omitted behavior; and schemas that permit ambiguous representation where execution requires fail-closed proof. Extend the existing coverage with these epoch-2 channels — do **not** create a parallel duplicate risk. |

D-X1 is a cross-link extension of an existing risk; D-S2 is an additive note to an existing cluster;
all other deliverables are new additive terms or risks. All seventeen are consistent with existing
reference content and with foundation/architecture/execution doctrine. Block T (D-T1…D-T3), Block R
(D-R1…D-R11), Block S (D-S1…D-S2), and the cross-cutting D-X1 map one-to-one onto the report's
seventeen findings (F-01…F-17).

## 4. Invariants Alignment

The reference tier is review-aid documentation subordinate to the upper tiers; it ratifies no
behavior and creates no invariant. The stances below record that each deliverable *helps reviewers
preserve* the cited invariant at review altitude, never weakening or reinterpreting it.

| Invariant / doctrine | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-112` — time may validate, but holder-known time must plan | aligns (terminologizes for review) | D-T1 names the temporal-authority concepts and their authority boundary; D-T2 records the relapse shapes; D-T3 adds the review questions — all at review-aid altitude, citing concrete temporal values to no one and routing them forward. The reference tier gains vocabulary and relapse memory to catch temporal-firewall drift, supplying no temporal truth to behavior. |
| `INV-099` — truth may validate actions, but truth may not plan them | aligns (preserves) | D-T1/D-T2 keep the validator/scheduler/replay-time-vs-holder-known-planning distinction reviewable; D-R6 (truth-cache learning) and D-X1 (smuggling channels) name the relapses where validation/debug truth would leak into planning. |
| `INV-102` — cognition inputs require provenance | aligns (extends review aid) | D-R1 (custody lineage), D-R5 (learned expectation as source-backed), and D-X1 (provenance-less smuggling) give reviewers terms to demand modeled-channel provenance for temporal, quantity, affect, and learned-expectation facts. |
| `INV-103` — the scheduler is not a cognition authority | aligns (preserves) | D-T1 (scheduler time vs. holder-known temporal claim) and D-R7/D-R8 (budget/fairness terms + invisible-director risk) keep the scheduler's order-and-validate role distinct from cognition at review altitude. |
| `INV-110` — LOD/summary processes must preserve the firewall | aligns (preserves) | D-T1 (LOD summary cadence, temporal/information ancestry), D-T2/D-T3 (silent LOD fill-in risk + ancestry review question) help reviewers catch omniscient temporal fill-in during summary/promotion. |
| `INV-111` — observer-only emergence evidence | aligns (preserves) | D-R8/D-R11 keep budget/fairness and play-legibility review summaries observer-only and non-certifying; the epoch-1 `EMERGE-OBS` term and R-27/R-28/R-29 cluster are preserved, not weakened, and D-S2 is authored additively on top of them. |
| Questless / no-director, genre-neutral kernel | aligns (preserves) | D-R9/D-R10 require bias/social harm to be reviewed as modeled, inspectable, domain-pack-bound mechanics with no morality oracle; D-R8 names performance-pressure-as-invisible-director; D-S1/D-S2 keep staged proof from certifying unimplemented futures by implication. |

No invariant is weakened or tensioned. All deliverables are additive reference-tier review aids at
the `name the concept / describe the relapse / ask the review question` altitude; no concrete value
(tick size, calendar syntax, threshold, fixture name, schema, algorithm, fairness formula, unit
vocabulary, denomination, category taxonomy) enters reference. **No reference file at this commit
shows a conflict requiring a foundation, architecture, or execution edit** — the report finds
reference review-aid gaps, not upstream tension. Any apparent upstream discomfort is carried as an
open question (§7), never patched into a lower-tier recommendation.

## 5. Verification

- **V1 — Premises confirmed against live `HEAD` (done at authoring).** Re-verified on `HEAD`
  `cda3325` (= the report's pinned commit, the merge of the `0033` enactment):
  - `INV-112` is present in `docs/0-foundation/02` with the "time may validate, but holder-known
    time must plan" wording; `INV-099`, `INV-102`, `INV-103`, `INV-110`, `INV-111` are present with
    the meanings cited in §4.
  - The architecture (`0032`) and execution (`0033`) cascades are enacted in `docs/1-architecture/*`
    and `docs/2-execution/*` (the immutable governing reference for this pass).
  - The reference docs carry **no** epoch-2 temporal/completeness terminology, relapse risks, or
    review questions today: `02_GLOSSARY.md` owns the epoch-1/baseline terms (`scheduler`,
    `salience`, `truth firewall`, `provenance`, `institution-known`, `procedure status`, `stale
    information`, `EMERGE-OBS`) but none of the new terms; `01_DESIGN_RISK_REGISTER.md` tops out at
    `R-29` with `R-18` schema/authoring-drift and the R-27/R-28/R-29 acceptance cluster present but
    no epoch-2 relapse shapes; `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` enforces source
    discipline and tier boundaries but asks no temporal-firewall review questions. The gaps are
    real `add`/`amend` deltas, not blank holes.
- **V2 — Enactment acceptance (on implementation).** Each deliverable is accepted only when its
  substance is authored into the named reference file as a compact, additive review-aid term /
  relapse-risk note / checklist pointer (not an index-row pointer only), the epoch-1 baseline is
  preserved (no rename/weaken/replace of `EMERGE-OBS`, the R-26 cross-link, or the R-27/R-28/R-29
  cluster), no new risk identifier or glossary token is minted unless the reassess/ticket process
  deliberately decides one is needed, and no concrete value token (tick size, calendar/date syntax,
  duration unit, day-part/lateness vocabulary, stale-after number, scheduler queue/algorithm,
  fairness formula/window, UI clock format, status enum, schema field, fixture name, unit
  vocabulary, denomination, category taxonomy, threshold) enters reference.
- **V3 — Boundary check.** Review each newly added reference passage for mechanism/value tokens;
  reference must hold terminology, relapse memory, and review pointers only. A whole-file grep is
  not the proof surface because reference legitimately names these concepts in the abstract.
- **V4 — Epoch-1 non-recommission check.** Confirm no deliverable re-opens, re-derives, renames, or
  strengthens the `EMERGE-OBS` term, the R-26 archive cross-link, or the R-27/R-28/R-29
  acceptance-evidence cluster; D-S2 is authored as a clearly additive staged-abstraction note
  alongside that cluster, and the "settled seven" anti-contamination themes remain baseline.
- **V5 — Forward-routing follow-through (later sessions, not this spec).** The specs-tier
  realignment (the `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` staged-abstraction edit + the future-spec
  backlog register, covered by the sibling `reports/specs-tier-alignment-research-report.md`) and the
  future scoped specs (concrete temporal vocabulary/thresholds, the first temporal-firewall fixture
  package, the inventory/economy fixture package, affect/learning depth, the domain-pack bias
  vehicle, budget/fairness numeric targets, authoring/compiler policy, TUI/play-loop fixtures) are
  owned by their own tier sessions; this spec only records the hand-offs (Out of Scope §6 and report
  §forward-routing).

## 6. Out of Scope

- **Final reference wording, new risk identifiers, glossary tokens, checklist codes, fixture names,
  schemas, vocabularies, thresholds, taxonomies.** Owned by the reassess/ticket enactment. This spec
  may say *that* a term, risk, or review question is needed and describe its shape; it may not author
  its final wording or mint its identifier.
- **All concrete temporal values.** Day-part vocabulary, "yesterday"/"last night"/"office
  closed/open"/"due/late"/"recently/stale" terms, calendar/date syntax and conversion, duration
  units, stale-after policy / record-validity periods / claim decay, office-hour representation,
  queue-aging, time-acceleration speed, missed-summary thresholds, and simultaneity/tie-break rules
  route to future scoped specs.
- **Affect/learning depth, domain-pack bias assumptions and category taxonomy, budget/fairness
  numeric targets, inventory/economy schemas and denominations.** Update rules, decay, trust
  dimensions, thresholds, the bias-assumption vehicle, fairness formulas, scheduler policy, and unit
  vocabularies route to future scoped specs (D-R1/D-R3/D-R5/D-R7/D-R9 substance is the review term
  only).
- **The specs-tier realignment.** The `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` staged-abstraction
  declaration-field edit, the future-scoped-spec backlog register, and the README/LEDGER/`0001`
  boundary checks are the sibling `reports/specs-tier-alignment-research-report.md`'s deliverable and
  should be captured as a separate future spec (recommended forward-routing number `0035`); D-S1
  stages only the reference-tier *terminology* for staged abstraction, not the template wording.
- **Foundation, architecture, and execution edits.** `INV-112`, foundation `03`, and the
  architecture/execution cascades (`0031`/`0032`/`0033`) are already ratified; this spec changes no
  `docs/0-foundation/*`, `docs/1-architecture/*`, or `docs/2-execution/*` file and creates no
  invariant.
- **Reopening the settled epoch-1 reference cascade.** No `0029` content (the `EMERGE-OBS` term, the
  R-26 archive cross-link, the R-27/R-28/R-29 acceptance-evidence cluster) and none of the "settled
  seven" anti-contamination themes are reopened; every deliverable is the temporal / completeness /
  staged epoch-2 delta only.
- **The possession-bind perception owner question.** Carried, not decided (epoch-1 F04; `0032` Risk
  R-F). If ever adopted, bind-time perception must be a modeled event/channel for the actor, never a
  transfer of human/player knowledge into the actor.
- **New world mechanics, code/crate changes, fixtures, LLM surfaces.**

## 7. Risks & Open Questions

- **R-A — Reference enactment requires owner approval.** Authoring D-T1…D-X1 edits tier-3 doctrine
  across the three reference docs (`00`, `01`, `02`). Lighter than constitutional or execution
  sign-off, but it must not proceed by convention; this spec stages the substance, it does not
  authorize the edits.
- **R-B — Block size / split option.** Seventeen deliverables across three drivers plus a
  cross-cutting amend is large. The spec bundles them per the report's single-amendment framing and
  the user's "a spec" directive; Block T (temporal), Block R (completeness), and Block S (staged) are
  cleanly separable into two or three specs if an owner prefers smaller enactment units. Splitting
  requires no rework — the deliverable IDs and homes are disjoint.
- **R-C — Mechanism/value leakage during enactment.** The chief failure mode is authoring concrete
  temporal vocabulary, thresholds, schemas, fairness formulas, denominations, category taxonomies, or
  fixture names into reference (V2/V3 guard against this). The temporal block is especially exposed
  because "morning," "yesterday," and "stale-after" feel like reference terms but are spec-owned
  values; reference names the *concept and authority*, not the chosen surface string.
- **R-D — Epoch-1 relapse / over-strengthening.** D-S2 and D-X1 touch the existing acceptance-evidence
  cluster and `R-18`; enactment must author them additively and must not re-open, rename, or strengthen
  the settled `EMERGE-OBS` / R-26 / R-27 / R-28 / R-29 content (V4 guards against this).
- **R-E — Observer-only review-summary relapse.** D-R8/D-R11 introduce budget/fairness and
  play-legibility review framing. These must stay observer-only and non-certifying; review counters
  or summaries becoming simulation objectives or actor-known urgency is forbidden authored-outcome
  machinery (`INV-111`).
- **Open questions (carried forward, not answered here).** From the report's open questions, the
  reference tier cannot settle: whether to organize temporal terms in one block or interleave them
  (D-T1); whether practical-bias terms include a generic "protected/social category" phrase now or
  defer to the domain-pack bias vehicle (D-R9); whether staged-abstraction terms sit near
  `EMERGE-OBS`/acceptance-evidence terms or in a separate group (D-S1); which future scoped spec owns
  first concrete temporal vocabulary vs. thresholds; how to schedule this reference amendment relative
  to the specs-tier `0003` staged-abstraction edit so the template can cite the term without circular
  wording; and the carried possession-bind perception owner question. These route to owner/reassess
  and future-spec sessions; none is resolved by this spec.

## 8. Provenance & Source Discipline

- The source report is pinned to `cda3325` and was re-verified against that same live `HEAD`; no
  intervening-commit drift applies (`cda3325` is the current `HEAD` merge of the `0033` enactment).
- External research cited in the report (SimPy time & scheduling, W3C PROV-DM, W3C/OGC OWL-Time, NIST
  AI Risk Management Framework, Gebru et al. "Datasheets for Datasets," JSON Schema, Open Policy
  Agent, Martin Fowler "Technical Debt," Bourgais/Taillandier/Vercouter emotion-modeling survey)
  shaped tier-fit and terminology judgment only; none becomes Tracewake doctrine and none overrides
  the repo's authority order. The report's References section holds the full citation list; this spec
  does not restate it.
- Commit hashes named here are audit/spec provenance only.
- This spec adds **no** `docs/4-specs/SPEC_LEDGER.md` row at proposal time, per the staged-spec
  convention (the `0026`/`0027`/`0029`/`0031`/`0032`/`0033` precedent: the ledger row lands at
  acceptance/closeout, not at proposal).

## Outcome

Completed: 2026-06-15

The `0034REFTIETEM` ticket series enacted the reference-tier temporal-authority and completeness alignment as compact, additive reference-layer review aids:

- `0034REFTIETEM-001` added glossary terminology in `docs/3-reference/02_GLOSSARY.md` for temporal authority and temporal/firewall ancestry, holder- and institution-known temporal claims, quantity/custody lineage, bounded affect, learned expectation, budget/fairness diagnostics, practical bias/domain-pack assumptions, and staged-abstraction / false-certification evidence honesty.
- `0034REFTIETEM-002` extended `docs/3-reference/01_DESIGN_RISK_REGISTER.md` with unnumbered epoch-2 temporal/completeness relapse notes, an additive staged-abstraction evidence-honesty note beside the existing acceptance-evidence cluster, and an in-place R-18 epoch-2 smuggling-channel extension.
- `0034REFTIETEM-003` added a compact temporal-authority review block to `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`.

The owner-approval precondition in §7 R-A was satisfied by the user's explicit `$ticket-series implement the series tickets/0034REFTIETEM*` instruction. The enactment preserved the reference tier's boundary: it added terminology, risk memory, and review questions only; it did not redefine foundation, architecture, execution gates, concrete temporal vocabulary, thresholds, schemas, denominations, category taxonomies, affect/learning formulas, budget/fairness formulas, fixture names, or risk-register numbering.

Deviations from proposal:

- The enactment made final wording decisions that the proposal left to reassess: `protected/social category` is included only as a restricted review concept; staged-abstraction / overclaiming / false-certification wording sits with evidence-honesty terminology; the temporal terms are added as compact glossary rows rather than a separate narrative section.
- No new `R-NN` identifier was minted; the risk-register additions are unnumbered relapse notes and an in-place extension of R-18.

Verification:

- Ticket-level greps and manual boundary reviews were run and recorded in archived tickets `0034REFTIETEM-001`, `0034REFTIETEM-002`, and `0034REFTIETEM-003`.
- `git diff --check` passed during each ticket closeout.
- Final repository gates are recorded in the spec-archive closeout commit and final response.
