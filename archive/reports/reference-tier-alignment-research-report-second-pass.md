# Reference-Tier Alignment Research Report

**Target repository:** `joeloverbeck/tracewake`

**Target commit:** `cda3325b0777f25101c9a04af3daeef24913f137` (`cda3325`)

**Target path:** `reports/reference-tier-alignment-research-report.md`

**Status:** NEW. The live-path report is absent from the uploaded `cda3325` manifest; the archived epoch-1 namesake under `archive/reports/` is historical context only.

**Scope:** recommendation report for `docs/3-reference/*`: `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`, `01_DESIGN_RISK_REGISTER.md`, and `02_GLOSSARY.md`.

**Deliverable type:** recommendation report only; not ratified wording; not a numbered spec.

**Limitation:** This report does not verify that `cda3325` is the current `main`. It uses the user-supplied commit as the target of record and analyzes only files fetched by exact commit URL from `joeloverbeck/tracewake`.

## Disposition table

| Finding | Target doc | Verdict | One-line basis |
|---|---|---:|---|
| Temporal-authority terminology | `docs/3-reference/02_GLOSSARY.md` | add | `INV-112`, foundation `03`, architecture A02/A03/A04/A10/A12/A13, execution E04/E05/E07/E12, and the execution report §6.1 route reference-tier terms for temporal authority without concrete values. |
| Temporal relapse risk memory | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | add | The same upstream cascade names clock-oracle leakage, wall-clock contamination, omniscient temporal labels, TUI acceleration leaks, debug-time leakage, and silent LOD fill-in as reference-tier risks. |
| Temporal review checklist pointers | `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | amend | The checklist currently states reference-tier source discipline but lacks the epoch-2 review questions needed to catch temporal-firewall drift. |
| Quantity, granularity, fungibility, custody, and lineage terminology | `docs/3-reference/02_GLOSSARY.md` | add | A09 and E08 separate unique objects, countable lots, divisible stocks, ledgers, custody, and lineage; reference must name review concepts without choosing schemas, units, denominations, or economy formulas. |
| Quantity/economy lineage-collapse risk | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | add | E08/E09 and the execution report §6.1 route a risk for global pools, display totals, or compiler conveniences erasing custody/provenance. |
| Bounded affect and salience review terms | `docs/3-reference/02_GLOSSARY.md` | add | A05/A06 and E06 allow affect as source-bearing salience/pressure, while deferring affect depth; the glossary needs cautious review-aid terms rather than a full model. |
| Affect-as-hidden-truth/decorative-meter risk | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | add | A05/A06 prohibit affect from revealing truth or bypassing planning; completeness routing also warns against meters that look rich but do not causally bind behavior. |
| Learned-expectation terminology | `docs/3-reference/02_GLOSSARY.md` | add | A05/A06 and E06 distinguish learned expectations from raw memory and hidden truth caches; update rules remain future-spec work. |
| Truth-cache-learning risk | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | add | E04/E06 explicitly require proof that learning cannot absorb validator/debug truth or become a timeless shortcut. |
| Scheduler budget, starvation, and fairness terminology | `docs/3-reference/02_GLOSSARY.md` | add | A13 and E05/E10 require diagnostic vocabulary for budget exhaustion, deferral, skipped/degraded cognition, starvation, and fairness without ratifying numeric targets. |
| Budget/fairness relapse risks | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | add | E05/E10 route risks for performance pressure acting as an invisible director and aggregate success hiding actor/region starvation. |
| Practical bias, social position, credibility, and domain-pack-assumption terminology | `docs/3-reference/02_GLOSSARY.md` | add | A08/E08/E11 place bias and wrong suspicion in modeled testimony, access, records, procedures, and domain packs; reference needs review terms without a morality oracle. |
| Emergent-injustice-as-author-prejudice risk | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | add | A08/E11 require fallible institutions and practical bias to be modeled, inspectable, and domain-pack-bound, not smuggled as author prejudice or objective harm truth. |
| Staged-abstraction and false-certification terminology | `docs/3-reference/02_GLOSSARY.md` | add | Execution `00` and E10 define staged declaration discipline and route terms for staged abstraction and false certification to reference. |
| Staged-abstraction false-certification risk | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | add | E10/F-13 routes a risk that declared abstraction becomes implied certification of future behavior. |
| Play-legibility dual relapse risk | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | add | The completeness determination routes the dual risk: a correct-but-unplayable audit machine versus “playability” achieved by leaking truth or objectives. |
| Authoring/compiler smuggling cross-link | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | amend | Existing authoring/schema drift coverage should be extended to epoch-2 temporal, quantity, bias, and staged-abstraction content, not duplicated as unrelated risk memory. |

## Method & provenance ledger

Requested repository: `joeloverbeck/tracewake`

Target commit: `cda3325b0777f25101c9a04af3daeef24913f137`

Freshness claim: user-supplied target commit only; not independently verified as latest `main`.

Manifest role: path inventory only.

Repository metadata used: no.

Default-branch lookup used: no.

Branch-name file fetch used: no.

Code search used: no.

Clone used: no.

URL fetch method: exact `raw.githubusercontent.com` URLs of the form:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/<manifest path>
```

Before each repository file was used, the requested URL was checked for owner `joeloverbeck`, repository `tracewake`, commit `cda3325b0777f25101c9a04af3daeef24913f137`, and a path present in the uploaded manifest.

Contamination observed: no.

Connector/tool namespace trusted as evidence: no.

The predecessor reports and archived specs were read only as files fetched from `joeloverbeck/tracewake` at the target commit. Any older commit strings inside those historical documents are historical content, not fetch targets and not freshness evidence.


## Authority posture and current reference-tier coverage

The live reference index already states the correct tier posture: reference material summarizes terminology, risk memory, review checklist discipline, and anti-contamination reminders; it does not outrank foundation, architecture, or execution. It also requires findings to cite exact source files and distinguish live doctrine from archived or stale reports.

The current risk register already covers broad exact-commit drift, repository contamination, anti-contamination regressions, stale reference terminology, authoring/schema drift, LOD ancestry loss, story-sifting direction, and the epoch-1 acceptance-evidence-honesty cluster. The current glossary already owns general terms such as `institution`, `institution-known`, `procedure`, `provenance`, `record`, `routine`, `salience`, `scheduler`, `stale information`, `truth firewall`, and `EMERGE-OBS`.

That coverage is not wrong; it is incomplete for epoch-2. The reference tier now needs the narrower vocabulary and relapse memory routed by `0031`/`0032`/`0033`, at review-aid altitude, without restating the upstream doctrine as if the glossary or risk register were constitutional authority.

## Settled context not re-commissioned

This report treats epoch-1 as already landed in the live `cda3325` docs. It does not re-open, re-derive, rename, or strengthen the `EMERGE-OBS` glossary term, the R-26/R-27/R-28/R-29 acceptance-evidence-honesty risk cluster and archive cross-link, or the `0003` evidence-status, fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical, and certification-use fields.

It also does not re-open the settled anti-contamination themes: event-sourced causality, subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads, validation/replay, and observer-only/non-certifying emergence evidence. Those are baseline guardrails for this epoch-2 delta.

The possession-bind perception question remains open. If it is ever adopted, the bind-time perception must be a modeled event/channel for the actor, never a transfer of human/player knowledge into the actor.


## Finding 1 — Temporal-authority terminology belongs in the glossary

**Driver.** `INV-112` and foundation `03` make simulation time authoritative for event order, replay, validation, intervals, scheduled consequences, and causal explanation, but not automatically available to cognition, routines, institutions, embodied views, speech, leads, or LOD promotion. A02/A03/A04/A10/A12/A13 and E04/E05/E07/E12 then translate that split into event/replay time, holder-known temporal claims, scheduler time, institution-known procedural time, temporal rendering, LOD temporal summaries, and typed observability. The execution report §6.1 explicitly routes temporal terminology to the reference tier.

**Current coverage.** The glossary has `scheduler`, `stale information`, `truth firewall`, `provenance`, `institution-known`, and `procedure status`, but it does not yet distinguish authoritative event/replay time from holder-known temporal claims, scheduler trigger time, institution-known procedural time, temporal ancestry, information ancestry, time acceleration, or LOD cadence.

**Tier-fit verdict.** Add reference terminology. Do not define tick size, calendar/date syntax, day-part vocabulary, duration units, stale-after thresholds, office-hour representation, time-acceleration speed, simultaneity, queue tie-breaks, UI clock strings, or concrete labels such as “late,” “closed,” “due,” “yesterday,” or “last night.”

**Recommendation.** In `docs/3-reference/02_GLOSSARY.md`, add review-aid terms for:

- temporal authority;
- event/replay time;
- scheduler time;
- holder-known temporal claim;
- institution-known procedural time;
- temporal firewall;
- freshness/staleness as source-backed status;
- validity window;
- temporal ancestry;
- information ancestry;
- time acceleration;
- LOD summary cadence.

Each entry should name the concept and its authority boundary. Each should point upward to foundation `03`/`INV-112` and laterally to the specific architecture/execution homes rather than redefining them. The glossary should explicitly say that concrete temporal surface vocabulary and thresholds belong to future scoped specs.

External research note: SimPy’s discrete-event scheduling documentation is useful prior art for treating simulation time and same-time ordering as deterministic implementation mechanics, not a cognitive oracle. OWL-Time is useful prior art for separating general temporal concepts from specific reference systems such as calendars and clocks. Neither source overrides Tracewake’s own authority split.

## Finding 2 — Temporal relapse risks belong in the risk register

**Driver.** `0032` and `0033` route reference-tier risk memory for the ways time becomes an illicit oracle: scheduler truth leaking into cognition, wall-clock or debug time entering diegesis, omniscient procedural labels appearing as if known, TUI acceleration leaking future knowledge, and LOD summaries silently filling temporal facts.

**Current coverage.** The risk register already has truth-firewall and debug-contamination risks, but those are broad. They do not give maintainers the new time-specific failure shapes now named upstream.

**Tier-fit verdict.** Add temporal relapse risk memory. The risk register should describe failure modes and review triggers, not final IDs, final phrasing, fixture names, or template text.

**Recommendation.** In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, add time-specific risks or a temporal-relapse cluster covering:

- clock-oracle leakage, where authoritative scheduler/replay time becomes a planning premise;
- raw wall-clock contamination, where host time, environment time, file timestamps, debug timestamps, or transcript time are treated as simulation facts;
- omniscient lateness/office-closed labels, where labels reflect truth rather than holder/institution-known procedure state;
- UI time-acceleration leaks, where player-facing acceleration or skip mechanics reveal facts not modeled for the holder;
- debug-time-becoming-diegetic, where validation/debug panels bleed into embodied views or speech;
- silent LOD temporal fill-in, where summaries generate holder-known time facts without temporal and information ancestry.

## Finding 3 — Temporal review questions belong in the reference checklist

**Driver.** The reference index/checklist currently enforces source discipline and tier boundaries, but epoch-2 introduced recurring temporal review questions that should be visible at review time.

**Current coverage.** `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` tells reviewers to keep reference subordinate and cite source files. It does not ask whether a temporal fact came through a modeled channel, whether validator time is quarantined, or whether UI/debug/LOD surfaces are preserving the temporal firewall.

**Tier-fit verdict.** Amend the checklist with pointers, not new doctrine.

**Recommendation.** In `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`, add a small temporal-authority review block. It should ask whether every planning/procedure/view/speech/lead/LOD temporal premise has a holder-known or institution-known source, whether validator/replay/scheduler time is separated from diegetic cognition, whether time acceleration and debug panels are quarantined from embodied surfaces, and whether temporal ancestry is preserved when summaries, snapshots, projections, or compacted history are used.

## Finding 4 — Quantity, granularity, fungibility, custody, and lineage terms belong in the glossary

**Driver.** A09 and E08 distinguish unique objects, countable lots, divisible stocks, capacities, debts, wages, ledgers, custody, visibility, split/merge operations, and display totals. E09 routes a future inventory/economy fixture package. The reference tier needs stable review language so maintainers can spot lineage collapse without freezing implementation schemas.

**Current coverage.** The glossary has generic property/institution/provenance terms, but it lacks the quantity and fungibility vocabulary needed to review economy/property specs without accidentally treating totals as authoritative lineage.

**Tier-fit verdict.** Add review-aid terms. Do not choose unit vocabularies, denominations, price formulas, storage schemas, inventory table shapes, split/merge event schemas, or content error formats.

**Recommendation.** In `docs/3-reference/02_GLOSSARY.md`, add cautious terms for quantity granularity and custody lineage. The entries should help reviewers distinguish unique things from countable groups and divisible stocks; distinguish a ledger/display total from authoritative custody lineage; and require ancestry preservation across split, merge, transfer, consume, spoil, pay, owe, store, disclose, hide, and discover operations.

External research note: W3C PROV-DM’s separation of entities, activities, agents/responsibility, derivations, and collections is useful prior art for this vocabulary judgment. Tracewake should not import PROV as doctrine; it should use the same general discipline of naming provenance/derivation questions clearly.

## Finding 5 — Quantity/economy lineage-collapse risk belongs in the risk register

**Driver.** The execution report §6.1 explicitly routes “quantity/economy lineage collapse” to reference risk memory.

**Current coverage.** Existing risk entries cover property abstraction, schema drift, and scale/LOD ancestry at a high level, but they do not warn against the specific relapse where a global inventory pool, UI count, fixture shortcut, or compiler aggregation erases custody/provenance.

**Tier-fit verdict.** Add a risk or amend an existing authoring/schema risk with this specific failure shape.

**Recommendation.** In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, record a risk that inventory/economy convenience can collapse lineage. The risk should warn reviewers to reject designs where totals replace event-sourced custody, where fungible aggregation loses actor/institution knowledge, where split/merge operations create untraceable stock, or where display inventory becomes the fact source.

## Finding 6 — Bounded affect and salience terms belong in the glossary

**Driver.** A05/A06 define affect as bounded, source-bearing salience or pressure that can shape attention, memory prominence, and planning priority but cannot reveal truth, select hidden targets, force drama, overwrite beliefs, or bypass causal planning. E06 routes routine/adaptation proof while deferring affect depth.

**Current coverage.** The glossary has `salience`, but it does not yet encode the epoch-2 distinction between affect as modeled pressure and affect as forbidden truth access.

**Tier-fit verdict.** Add a cautious review-aid term set. Do not decide affect dimensions, update equations, emotional categories, intensity ranges, decay rates, or appraisal rules.

**Recommendation.** In `docs/3-reference/02_GLOSSARY.md`, add or extend terms for bounded affect, salience, affect-bearing memory, and affect pressure. The substance should make clear that affect may alter reviewable attention/priority only through modeled sources and must remain subordinate to actor-known cognition and transaction proof.

External research note: the social-simulation literature shows many possible emotion representations, from simple numerical values to appraisal-style models. That variety argues for a narrow reference entry and future-spec deferral, not for choosing a Tracewake affect model in the glossary.

## Finding 7 — Affect relapse risks belong in the risk register

**Driver.** Completeness routing and A05/A06 together name two failure modes: affect that secretly reveals truth and affect that is decorative rather than causally bound.

**Current coverage.** The risk register has broad “ordinary agents” and truth-firewall risks, but not an affect-specific relapse pattern.

**Tier-fit verdict.** Add risk memory.

**Recommendation.** In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, add a risk covering affect-as-hidden-truth and decorative-meter affect. It should ask whether affect values were produced from modeled experience, whether they can explain priority without selecting hidden targets, whether they can be replayed/diagnosed, and whether UI emotional flavor is backed by causal state rather than prose.

## Finding 8 — Learned-expectation terminology belongs in the glossary

**Driver.** A05/A06 and E06 distinguish learned expectations from memory and from truth caches. Learning may derive expectations from repeated modeled experience, but cannot become an all-purpose source of true schedules, hidden facts, future events, or objective success labels.

**Current coverage.** The glossary has `memory`, `belief`, `provenance`, and `routine`, but it does not yet give reviewers a term for provenance-bearing learned expectations distinct from records and memories.

**Tier-fit verdict.** Add review-aid terminology. Defer learning algorithms, trust/decay formulas, generalization rules, and update thresholds to future specs.

**Recommendation.** In `docs/3-reference/02_GLOSSARY.md`, add a learned-expectation term that marks it as derived, source-backed, defeasible actor state. The term should help reviewers ask whether an expectation cites modeled experience and whether its scope/freshness prevents it from operating as timeless truth.

## Finding 9 — Truth-cache-learning risk belongs in the risk register

**Driver.** E04 and E06 require proof that anti-truth-cache learning holds across temporal and routine cases.

**Current coverage.** The risk register catches stale references and hidden-truth planning in general, but not the learning-specific relapse where repeated tests or validation labels train a shortcut that later plans from truth.

**Tier-fit verdict.** Add risk memory.

**Recommendation.** In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, add a risk that learned expectations can become truth caches. The review trigger should include fixture labels, debug outcomes, validation failures, global success rates, or omniscient classifier outputs being stored as actor expectations without a modeled observation, memory, testimony, record, or routine experience.

## Finding 10 — Scheduler budget, starvation, and fairness terms belong in the glossary

**Driver.** A13 and E05/E10 require budget/fairness observability: deterministic candidate order, budget exhaustion, deferred/skipped cognition, blocked actors, regional fairness, no human-proximity priority unless explicit and quarantined, and diagnostics that prevent aggregate success from hiding starvation.

**Current coverage.** The glossary has `scheduler` but not review vocabulary for budget exhaustion, starvation, fairness scope, repeated deferral, aggregate success, or invisible director pressure.

**Tier-fit verdict.** Add review-aid terms. Do not choose numeric budgets, percentages, windows, priority formulas, fairness metrics, starvation thresholds, or scheduler algorithms.

**Recommendation.** In `docs/3-reference/02_GLOSSARY.md`, add terms or term notes for scheduler budget, planner budget, budget exhaustion, deferred cognition, skipped/degraded cognition, starvation, fairness envelope, and aggregate success. The terms should be diagnostic vocabulary only.

## Finding 11 — Budget/fairness relapse risks belong in the risk register

**Driver.** The execution report §6.1 routes performance-pressure-as-invisible-director and budget-starvation-hidden-by-aggregate-success to reference.

**Current coverage.** The risk register’s scale and planner risks are broad. They do not specifically warn that performance optimizations can steer content or starve actors while the visible sample still passes.

**Tier-fit verdict.** Add risk memory.

**Recommendation.** In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, add risks covering:

- performance pressure acting as an invisible director, where the system silently suppresses uninteresting, remote, inconvenient, or non-player-adjacent cognition to create drama or speed;
- budget starvation hidden by aggregate success, where global pass rates or headline metrics mask repeated deferral, skipped cognition, or degraded reasoning for particular actors, regions, institutions, or LOD classes.

## Finding 12 — Practical bias, credibility, social position, and domain-pack-assumption terms belong in the glossary

**Driver.** A08 and E11 define institution/practical bias as fallible procedure and social interpretation grounded in testimony, records, access, norms, credibility, social position, and domain-pack assumptions. E08 makes domain-pack assumptions explicit and reviewable. The kernel remains genre-neutral and must not mint an objective morality oracle.

**Current coverage.** The glossary has `institution`, `institution-known`, `institutional fact`, `record`, `procedure`, `procedure status`, and `wrong suspicion`, but not the review vocabulary for modeled bias and domain-pack assumptions.

**Tier-fit verdict.** Add review-aid terms. Do not choose protected-category taxonomy, harm scoring, moral truth labels, institution-pack schema, credibility algorithm, or bias vehicle implementation.

**Recommendation.** In `docs/3-reference/02_GLOSSARY.md`, add cautious terms for modeled practical bias, credibility, social position, protected/social category as a review concept, domain-pack assumption, and wrong-suspicion evidence. The entries should say that such terms exist to review whether harm/bias emerges from modeled evidence/procedure/access/testimony and explicit content assumptions, not from author fiat or hidden truth.

External research note: NIST’s risk-management framing for risks to individuals, organizations, and society, and the datasheets pattern of documenting motivation/composition/process/uses, support the recommendation that future domain-pack assumptions be explicit and reviewable. They do not supply Tracewake’s doctrine or category set.

## Finding 13 — Emergent-injustice-as-author-prejudice risk belongs in the risk register

**Driver.** Completeness routing names practical bias/social harm. A08/E11 permit wrong suspicion and institutional fallibility only when they have modeled causes and procedure/record/testimony pathways.

**Current coverage.** The risk register covers fallible institutions and story direction generally, but not the specific risk that biased outcomes appear because authors hard-coded prejudice, moral labels, or objective suspicion.

**Tier-fit verdict.** Add risk memory.

**Recommendation.** In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, add a risk for emergent-injustice-as-author-prejudice. It should push reviewers to ask whether the content pack names assumptions, whether the evidence path is modeled, whether credibility/access/procedure are explicit, whether affected actors can possess partial or wrong knowledge, and whether the artifact avoids hidden moral truth or author verdicts.

## Finding 14 — Staged-abstraction and false-certification terms belong in the glossary

**Driver.** Execution `00` and E10 allow staged proof only when the artifact declares what it proves now, what it deliberately abstracts, what it must not fake, what it must not block, what evidence prevents overclaiming, and what failures should diagnose. The execution report §6.1 routes staged-abstraction and false-certification terms to reference.

**Current coverage.** The glossary has evidence-honesty terms from epoch-1, but it does not yet name staged abstraction as a distinct epoch-2 concept.

**Tier-fit verdict.** Add reference terms, but do not duplicate the template field list as final wording. The exact template edit belongs in specs tier `0003`.

**Recommendation.** In `docs/3-reference/02_GLOSSARY.md`, add terms for staged abstraction and false certification/overclaiming. The substance should say that a staged abstraction is a declared, bounded omission or simplification in an acceptance artifact; false certification is the relapse where that omission is treated as proof of omitted future behavior.

External research note: technical-debt literature is useful only as a cautionary analogy: intentional incompleteness is manageable when explicit, dangerous when hidden. Tracewake’s staged-abstraction doctrine remains the upstream execution wording, not a generic debt-management import.

## Finding 15 — Staged-abstraction false-certification risk belongs in the risk register

**Driver.** E10/F-13 routes the acceptance-artifact staged-abstraction hand-off and warns against overclaiming.

**Current coverage.** R-27/R-28/R-29 already cover evidence honesty, but they were epoch-1. They should remain baseline rather than be re-commissioned; epoch-2 needs an additive risk focused on declared incompleteness becoming implied completion.

**Tier-fit verdict.** Add risk memory or amend the existing acceptance-evidence risk cluster with a clearly additive staged-abstraction relapse note. Do not invent a new risk identifier in this report.

**Recommendation.** In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, record that acceptance artifacts can falsely certify staged work when abstractions are buried, when omitted behavior is not named, when future blockers are obscured, when evidence scope is not tied to the abstraction, or when failure diagnostics cannot distinguish “not implemented yet” from “implemented and broken.”

## Finding 16 — Play-legibility dual relapse risk belongs in the risk register

**Driver.** The completeness determination routes a play-legibility risk to reference: one relapse is a correct system that becomes an unplayable audit machine; the opposite relapse is playability achieved by leaking truth, objectives, quest markers, debug labels, or omniscient summaries.

**Current coverage.** The risk register has TUI/debug/possession and story-sifting risks, but not the paired legibility failure where both extremes are wrong.

**Tier-fit verdict.** Add risk memory. This is especially important because reference is the tier that helps reviewers notice drift across multiple later specs.

**Recommendation.** In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, add a dual-relapse play-legibility risk. It should ask whether players can understand and act through holder-known surfaces, leads, notices, and explainable procedure without seeing ground truth; and whether the artifact avoids replacing play with audit-only source inspection.

## Finding 17 — Authoring/compiler smuggling should be cross-linked, not duplicated

**Driver.** E08 and E10 require content validation to fail closed on ambiguous fungibility, untracked creation, authored knowledge without modeled channels, hidden temporal facts, bias assumptions that are not explicit, and proof-bearing staged fields that overclaim. Existing risk memory already has broad schema/authoring drift.

**Current coverage.** The risk register has an authoring/schema risk, but its epoch-2 coverage should now explicitly include temporal, quantity/custody, bias-domain-pack, and staged-abstraction smuggling.

**Tier-fit verdict.** Amend the existing authoring/schema drift coverage or add a cross-link note, not a parallel duplicate risk.

**Recommendation.** In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, amend the authoring/compiler risk memory to mention epoch-2 smuggling channels: fixture labels that become holder knowledge; display totals that become custody truth; domain-pack assumptions hidden in prose; compiler defaults that select temporal/quantity semantics; staged-abstraction fields that certify omitted behavior; and schemas that permit ambiguous representation where execution requires fail-closed proof.

External research note: JSON Schema and OPA are useful prior art for future validation/policy enforcement, but the reference tier should not prescribe either technology.

## Forward-routing appendix

This is a terminal-tier realignment report. Nothing routes further down as a documentation-authority cascade. The remaining actions route only to maintainer/reassess decisions and future implementation specs.

Future implementation specs may own concrete temporal vocabulary, concrete thresholds, temporal-firewall fixtures, inventory/economy fixtures, affect/learning depth, domain-pack bias vehicles, budget/fairness numeric targets, authoring/compiler policy, and TUI/play-loop fixtures. Those specs must stay subordinate to foundation, architecture, execution, and the updated reference tier. They must not bundle the temporal-firewall and inventory/economy fixture packages unless one concrete gameplay feature genuinely needs both.

The possession-bind perception owner question remains carried, not decided. If adopted, it must be represented as a modeled event/channel for the actor and not as a human/player knowledge transfer.

## Open questions

- Should the glossary organize temporal terms in a single temporal-authority block or interleave them with existing truth-firewall, scheduler, stale-information, and institution-known terms?
- Should practical-bias terms include a generic “protected/social category” review phrase now, or wait for the future domain-pack bias vehicle to choose domain-specific language?
- Should staged-abstraction terms live near `EMERGE-OBS`/acceptance-evidence terms or in a separate evidence-honesty group?
- Which future scoped spec owns first concrete temporal vocabulary and which owns thresholds, if they are split?
- How should the maintainer schedule the reference amendment relative to the specs-tier `0003` staged-abstraction edit so that the template can cite the term without creating circular wording?
- The possession-bind perception owner question remains open as stated above.

## References

### Exact-commit repository sources

Primary governing sources:
- `docs/README.md` — authority order and layer boundaries.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — constitutional invariants, including `INV-112`.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — temporal-authority section.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — truth-firewall temporal boundary.
- `docs/1-architecture/02`, `03`, `04`, `05`, `06`, `08`, `09`, `10`, `11`, `12`, `13` — architecture temporal/completeness contracts.
- `docs/2-execution/00`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12` — execution routing and proof homes.
- `reports/execution-tier-alignment-research-report.md` — freshest routing seed, especially §6.1 and §6.2.
- `archive/specs/0031`, `0032`, `0033` — ratified epoch-2 amendments.
- `archive/specs/0029`, `0030` and `archive/reports/reference-tier-alignment-research-report.md`, `archive/reports/specs-tier-alignment-research-report.md` — settled epoch-1 boundary context.

## Repository source ledger

All repository file sources below were fetched through exact raw URLs under the target owner, repository, commit, and manifest path. The uploaded manifest was used only to confirm that these paths existed in the stated inventory. The two report output paths were absent from the manifest and are therefore new report files, not replacements.

- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/execution-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/architecture-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/foundation-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/foundations-completeness-determination-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/verdict-on-foundations.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0033_EXECUTION_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0029_REFERENCE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0030_SPECS_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/reports/reference-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/reports/specs-tier-alignment-research-report.md


### External sources consulted

- SimPy, “Time and Scheduling” — used only as prior art for deterministic discrete-event scheduling, same-time tie-break discipline, and why exact time values belong to implementation rather than terminology: https://simpy.readthedocs.io/en/latest/topical_guides/time_and_scheduling.html
- W3C PROV-DM — used only as prior art for provenance, derivation, responsibility, and extensibility vocabulary: https://www.w3.org/TR/prov-dm/
- W3C/OGC OWL-Time — used only as prior art for separating temporal concepts, intervals/durations/positions, and concrete temporal reference systems: https://www.w3.org/TR/owl-time/
- NIST AI Risk Management Framework page — used only as prior art for risk review language about harms to individuals, organizations, and society; it does not define Tracewake doctrine: https://www.nist.gov/itl/ai-risk-management-framework
- Gebru et al., “Datasheets for Datasets” — used only as prior art for documenting assumptions, composition, collection/process, and intended uses of content/domain packs: https://arxiv.org/abs/1803.09010
- JSON Schema, “Creating your first schema” — used only as prior art for future authoring/compiler validation vocabulary: https://json-schema.org/learn/getting-started-step-by-step
- Open Policy Agent documentation — used only as prior art for future policy-as-code/CI enforcement options: https://www.openpolicyagent.org/docs
- Martin Fowler, “Technical Debt” — used only as prior art for making deliberate staged incompleteness visible instead of letting it masquerade as completion: https://martinfowler.com/bliki/TechnicalDebt.html
- Bourgais, Taillandier, and Vercouter, “Emotion Modeling in Social Simulation” — used only as prior art showing that affect modeling can become deep and model-specific, supporting a cautious reference-altitude term rather than premature Tracewake doctrine: https://www.jasss.org/21/2/5.html

