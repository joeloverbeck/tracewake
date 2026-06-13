# Architecture-tier alignment research report — `joeloverbeck/tracewake` at `fdfd0b9`

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9`  
**Deliverable status:** change-proposal report for `docs/1-architecture/*`; not rewritten architecture doctrine; not a foundation amendment; not execution certification.

I am not verifying that this commit is the current `main`. I am using the supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

---

## 0. Disposition table

| ID | Finding / seed | Target architecture doc(s) | Verdict | One-line basis |
|---|---|---|---|---|
| A13-E1 | **Observer-only emergence-evidence data contract** — routing theme E after `INV-111` | `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`; cross-reference `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | **Belongs in architecture** | Foundation now requires observer-only emergence evidence (`INV-111`; docs 09/12); A13 has typed artifact families but no emergence-evidence record or one-way observer boundary. |
| A11-E2 | **Story-sifting evidence wording now too broad** — swept ripple from `INV-111` | `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | **Belongs in architecture — correction** | A11 correctly says story sifting is observer-only, but its blanket “may not create evidence” needs scoping so it forbids diegetic/holder evidence while allowing observer-only acceptance evidence under A13. |
| A03/A06-A | **Provenance sufficiency** — routing theme A | `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`; `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | **Belongs in architecture — partial coverage** | A03/A06 already require source-bound holder-known contexts; they need a compact, shared sufficiency rule: modeled non-empty source evidence, not labels/booleans/prose, with fail-closed missing/invalid provenance. |
| A06-B | **Memory freshness classifier** — routing theme B | `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`; A03 as already-owned anchor | **Belongs in architecture — partial coverage** | A03 already states the observed-now vs remembered/stale rule; A06 owns epistemic data flow but lacks the same classifier as a first-class epistemic contract. |
| A10-C | **Believed-access affordance snapshots** — routing theme C | `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`; cross-reference A03 | **Belongs in architecture — partial coverage** | A03/A04/A10 already require holder-known affordances, but A10 should explicitly own observation-time carrier/attribute snapshot capture for embodied menus and view models. |
| A04/A05/A09-D | **Single-charge derived accounting seam** — routing theme D | `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`; `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`; `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`; `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | **Belongs in architecture — partial coverage** | A00 records detailed hardening rows; primary subsystem contracts should name the single-owner seam for derived need deltas, tick-regime classification, and duration open/close authority. |
| A13-F | **Typed observability for non-vacuous proof** — routing theme F | `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | **Belongs in architecture, proof mechanics route forward** | A13 already rejects “looks right” and display strings; it should add the subsystem-level contract that validation surfaces expose typed behavior witnesses sufficient for execution to build live negatives. |
| X10/G | **Acceptance-evidence / manifest-fingerprint honesty** — report-added theme G | No architecture change beyond A13-F; route to `docs/2-execution/10` and `docs/3-reference/01` | **Route-forward** | Status taxonomies, raw-byte fingerprint scope, mutation-pending language, and evidence maps are execution/review-artifact rules, not architecture contracts. |
| A08-OQ1 | **Institution-known provenance residual** — predecessor §11 open question | `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`; A03 | **Already-owned-close; route future proof watch** | A08 already requires sealed institution-known context with provenance edges and forbids event-log truth conversion; future Phase 4 proof belongs in execution if new institutional machinery repeats unbacked facts. |
| A00/A10-OQ2 | **Possession-bind perception residual** — predecessor §11 open question / A00 row | `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`; possible future `03`/`10` | **Owner decision; do not silently amend** | Current architecture records the unresolved doctrine tension; A10 says possession does not mutate knowledge. Architecture should not decide whether binding emits modeled perception without owner/foundation clarification. |
| SWEEP | **15-doc rewrite sweep** | All `docs/1-architecture/00`…`14` | **Mostly conformant; specific gaps above** | The truth-firewall rewrite is already translated broadly. No extra contradiction was found in A01/A02/A07/A12/A14; A11/A13 carry the new INV-111 ripple. |

---

## 1. Method and provenance ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.open against exact raw.githubusercontent.com URLs after selected paths were checked against the uploaded manifest path inventory
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/reports/foundation-amendment-lower-tier-routing.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/reports/foundation-amendment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/reports/foundation-amendment-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/docs/3-reference/02_GLOSSARY.md
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

**Stale-provenance quarantine.** The fetched predecessor report and routing memo contain their own historical provenance at commit `f7adc01`. Those strings are part of the fetched artifacts’ internal history only. They were not used as fetch targets. The fetched `0026` spec also records that the amendment was later ratified in commit `91cc8a4`; that is historical amendment provenance, not a source target for this analysis.

**Provenance convention in this report.**

1. **Tracewake source doctrine/evidence** means exact-commit files listed in the ledger above. Short source labels below map to those fetched URLs: `TW-F-00`…`TW-F-14` are foundation files; `TW-A-00`…`TW-A-14` are architecture files; `TW-ROUTE`, `TW-REPORT`, `TW-BRIEF`, `TW-SPEC-0026`, `TW-SPEC-LEDGER`, `TW-X-00`, `TW-RISK`, and `TW-GLOSS` are the fetched input/reference files.
2. **Historical pressure** means archived specs and reports as summarized in the fetched spec ledger and predecessor report. It is evidence that a design pressure existed, not live certification.
3. **Recommendation / inference** means this report’s judgment about what the architecture tier should own. It is not ratified doctrine.

No crate source files were fetched or used as independent implementation evidence. Code-like symbols appearing in A00, the spec ledger, or reports are treated as documentation evidence only.

---

## 2. Foundation delta that architecture must now translate

### D1 — Layer boundary is stricter, not looser

The live documentation map says architecture owns authority ownership, data flows, subsystem boundaries, diagnostics, and acceptance implications, while execution owns gates, fixtures, audit criteria, and review artifacts. The foundation index likewise says foundation defines Tracewake’s no-hidden-director/no-omniscient-cognition/no-quest/no-LLM-authority identity but does not define module layout, storage, serialization, planning algorithms, fixture shape, gate mechanics, or test procedure (`TW-README:L8-L20`; `TW-F-00:L10-L11`).

**Architecture implication:** recommendations here must be subsystem contracts: which component may read, write, derive, expose, or preserve what. Anything that is a threshold, fixture, command, ratchet, mutation perimeter, or gate procedure is routed to execution/reference/specs.

### D2 — New constitutional truth firewall, doc 14

Foundation doc 14 is now the clearest governing source: “Truth may validate actions, but truth may not plan them.” It requires an actor-known cognition transaction with a sealed actor-known context before candidate generation; hidden truth must not choose goals, plans, routines, dialogue, embodied menus, institution outcomes, LOD promotion, prompt packets, or debug-to-actor knowledge (`TW-F-14:L0-L8`). It also defines allowed actor-known sources, forbidden sources, provenance classes, proposal-vs-validation split, scheduler boundary, institution/record firewall, debug quarantine, LOD summary handling, and typed diagnostics (`TW-F-14:L9-L30`). The invariant file re-derives this as the `INV-099`…`INV-110` family, including `INV-101` sealed actor-known context and `INV-102` source/provenance requirements for action-relevant cognition (`TW-F-02:L65-L76`).

**Architecture implication:** the architecture tier must not merely say “don’t leak truth.” It must specify the holder-known context packet, the sealed decision boundary, allowed/forbidden source classes, validator-only truth authority, debug quarantine, and the surfaces that consume the packet: actors, institutions, speech, TUI affordances, leads, LOD promotion, and regional processes.

**Sweep result:** this translation is broadly present already in A00/A03/A04/A05/A08/A10/A12/A13. The remaining doc-14-related refinements are mostly consolidation: provenance sufficiency, freshness parity, believed-access snapshots, and single-charge ownership.

### D3 — Foundation rewrite made provenance, freshness, and epistemic taxonomy more precise

Foundation doc 04 now defines claims, beliefs, observations, memories, records, rumors, testimony, absence evidence, holder-known cognition, and unacceptable provenance in more precise terms. It requires action-relevant beliefs, routine premises, institutional facts, records, notices, rumors, testimony, and memories to carry provenance, and rejects raw state, debug reports, branch/test labels, LLM prose, untyped booleans, and display strings as provenance (`TW-F-04:L30-L40`). Invariant support includes `INV-024`, `INV-026`, `INV-028`, `INV-030`, and `INV-102` (`TW-F-02:L16-L20`; `TW-F-02:L68-L72`).

**Architecture implication:** A03/A06 should share one provenance-sufficiency rule and one freshness classifier, rather than leaving the doctrine scattered across hardening rows and examples.

### D4 — Foundation now makes perceived-vs-actual affordance a core split

Foundation doc 06 states that action proposals pass through actor-knowledge-bounded proposal construction and authoritative validation (`TW-F-06:L2-L6`). Its 2026 perceived-vs-actual clause says perceived affordances and actual validation can diverge; actors may attempt believed-safe or stale options, validators may reject using truth, and feedback may become future knowledge only through modeled channels (`TW-F-06:L25-L33`). Foundation doc 08 adds that embodied view models derive from actor-known context and include affordances the actor perceives or believes possible, excluding hidden facts (`TW-F-08:L28-L34`). Doc 14 then forbids hidden truth from generating embodied affordance menus (`TW-F-14:L1-L3`).

**Architecture implication:** A10 needs an explicit observation-time snapshot/capture contract for embodied menus and view models, not just a generic “generated from holder-known context” statement.

### D5 — `INV-111` and emergence-evidence amendment create the largest new architecture gap

The live invariant file now carries `INV-111`: living-world acceptance requires observer-only emergence evidence; reachability alone is insufficient; evidence must be replayable, observer-only, derived from modeled no-human or normal-controller runs, and unable to feed cognition, scheduler behavior, validators, or dramatic objectives (`TW-F-02:L77-L78`). Foundation doc 09 adds that observing emergent phenomena after the fact is permitted and expected, while steering the world to enrich the observer record is authored-outcome machinery (`TW-F-09:L19-L20`). Foundation doc 12 adds that first-playable proof must include a retrospective emergence-evidence record beside gates; it is acceptance evidence, not a reason generator, certification substitute, or license for authored outcomes (`TW-F-12:L24-L25`). Spec 0026 records the five constraints behind the amendment: observation not steering, retrospective not prospective, phenomenon families not authored outcomes, causal replay ancestry, and no Goodhart target (`TW-SPEC-0026:L10-L18`, `TW-SPEC-0026:L34-L38`).

**Architecture implication:** A13 must now define the subsystem data contract for observer-only emergence evidence. A11 must scope story-sifting language so the observer may produce acceptance evidence without creating diegetic evidence or steering causality.

---

## 3. Architecture sweep coverage register

| Architecture doc | Current foundation alignment verdict | Notes |
|---|---|---|
| `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | **Conformant, but conformance rows are too implementation-specific to be the only home for some rules.** | A00 already has universal conformance questions covering holder, provenance, sealed context, truth role, feedback, and replay evidence (`TW-A-00:L6-L9`). It also records detailed Phase 3A rows for single-charge, freshness, provenance, believed access, and possession-bind deferral (`TW-A-00:L22-L28`, `TW-A-00:L74-L75`). Keep as index/evidence; put compact subsystem doctrine in primary docs. |
| `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` | **No change found.** | Boundary/dependency doctrine is consistent with foundation. No `INV-111` ripple. |
| `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | **No change found.** | Event-log ancestry and replay projections already support the emergence-evidence data contract; A13 should consume them. |
| `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | **Partial refinement needed.** | Strong truth-firewall translation exists (`TW-A-03:L0-L11`); add compact provenance-sufficiency wording and cross-link freshness/snapshot refinements. |
| `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | **Partial refinement needed.** | Proposal/validation split and scheduler limits are strong (`TW-A-04:L0-L12`); add single-owner derived accounting seam. |
| `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | **Partial refinement needed.** | Actor-known transaction is strong (`TW-A-05:L0-L12`); clarify that live need pressures are consumed from event-sourced accounting and do not mint deltas inside cognition. |
| `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | **Partial refinement needed.** | Good epistemic taxonomy (`TW-A-06:L0-L12`); add shared provenance-sufficiency and freshness classifier language. |
| `07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` | **No change found.** | Speech/LLM boundaries already conform to doc 11/doc 14; no additional architecture finding. |
| `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | **Already-owned-close for current residual.** | Sealed institution-known context with `provenance_edges` is present; event-log truth conversion is denied (`TW-A-08:L0-L13`). |
| `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | **Partial refinement needed.** | Ordinary-life proof and actor-known food/sleep/work are strong (`TW-A-09:L0-L13`); add single-charge accounting dependency for food/sleep/work/economy effects. |
| `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | **Partial refinement needed.** | TUI/possession/debug split is strong (`TW-A-10:L0-L11`); add observation-time embodied snapshot/capture rule. |
| `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | **Correction needed for `INV-111` ripple.** | Story sifting is correctly observer-only, but “may not create evidence” should be scoped to diegetic/holder evidence so A13 observer-only emergence evidence remains allowed (`TW-A-11:L6-L10`). |
| `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` | **No change found.** | LOD ancestry/truth-firewall translation is consistent; no `INV-111` ripple beyond A13. |
| `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | **Major change needed.** | A13 owns architecture-level observability and artifacts, but no observer-only emergence-evidence data contract appears in required artifact families or gate groups (`TW-A-13:L0-L11`). |
| `14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` | **No change found.** | Research/forbidden-misread doctrine remains aligned; no new external-prior-art import required. |

---

## 4. Per-finding analysis and recommendations

### 4.1 A13-E1 — Observer-only emergence-evidence data contract

**Target home:** `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`; secondary cross-reference from `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`.

#### Foundation driver

`INV-111` now requires observer-only emergence evidence for living-world acceptance: replayable evidence of unscripted phenomena arising from modeled causes in no-human or normal-controller runs, structurally unable to feed cognition, scheduler behavior, validators, or dramatic objectives (`TW-F-02:L77-L78`). Doc 09 says emergence observation is not steering (`TW-F-09:L19-L20`). Doc 12 requires retrospective emergence evidence beside first-playable gates and warns that it is not a reason generator or standalone certification (`TW-F-12:L24-L25`). Spec 0026 records that foundation authorizes the principle while the mechanism, counters, thresholds, and ratchet policy stay in execution (`TW-SPEC-0026:L10-L18`, `TW-SPEC-0026:L26-L30`).

Campaign pressure is direct: the predecessor report found exactly one foundation hole, emergence-as-evidence, and specified that the lower-tier mechanism still needed architecture/execution/reference specialization (`TW-REPORT:L60-L67`; `TW-REPORT:L77-L83`). The routing memo assigns architecture `13` the observer-only data contract: evidence rows explainable by event-log ancestry and structurally outside the simulation (`TW-ROUTE:L10-L11`). The spec ledger confirms `0026` as a ratified foundation amendment that leaves architecture/execution/reference specialization to later work (`TW-SPEC-LEDGER:L41-L42`).

#### Current architecture coverage

A13 is strong on general observability: typed diagnostics are architecture, plausible behavior without proof fails, and the doc owns architecture-level acceptance artifacts, observability obligations, anti-regression gates, review checklists, and invalid pass conditions (`TW-A-13:L0-L1`). Its required artifact families include event logs, replay reports, holder-known context packets, decision traces, stuck diagnostics, validation reports, epistemic projection reports, institution traces, TUI transcript/view-model evidence, debug reports, content validation, and anti-regression/static guards (`TW-A-13:L2-L7`). It also rejects story/projection systems that spawn events or drive decisions (`TW-A-13:L7`).

But A13 has no artifact family or review checklist entry for an **observer-only emergence-evidence record**. It does not specify the fields that make such a record explainable by event-log ancestry, or the one-way structural boundary that prevents the observer record from feeding cognition/schedulers/validators. This is a real gap created by `INV-111`, not a failure of the older truth-firewall architecture.

A02 already supplies event-log/replay ancestry, and A11 already says story sifting is observer/projection only. Those are support contracts, not the acceptance-evidence data contract.

#### Tier-fit verdict

**Belongs in architecture.** Foundation owns the principle; execution owns the `EMERGE-OBS` table/command/counters/thresholds/ratchets; reference owns terminology. Architecture owns the data-flow and authority boundary between the simulation and observer evidence. The `docs/README.md` table places architecture ownership at data flows, subsystem boundaries, diagnostics, and acceptance implications, exactly where this belongs (`TW-README:L8-L20`).

#### Recommendation

Add a new A13 artifact-family / subsection for **observer-only emergence evidence**. Substance, not final wording:

- An emergence-evidence record is a retrospective review artifact, not world state, not holder-known context, not institution-known context, not validation input, and not scheduler input.
- Each evidence row must carry event-log ancestry sufficient to replay/explain the phenomenon it claims: source run, seed/random provenance where applicable, controller mode/no-human or normal-controller run identity, phenomenon family, source event IDs or causal chain references, extraction time, and review/projection version.
- The record may classify phenomenon families such as contradictions, replans, interruptions, stale-belief consequences, modeled-channel corrections, belief/truth divergence, wrong suspicion, and record effects. It must not name required story beats, dramatic objectives, or numeric floors in architecture.
- The data path must be one-way: authoritative simulation → replay/projection/story-sifting observer → review artifact. It must not feed actor/institution cognition, validators, scheduling priorities, candidate generation, LOD promotion decisions, story sifting that creates future events, or content authoring objectives.
- A13 invalid-pass conditions should include: emergence evidence sourced from debug/fixture fabrication rather than path-under-test event ancestry; emergence counters used to steer simulation behavior; or evidence rows that cannot be traced to modeled causes.

Route thresholds, ratchet policies, table/row names, commands, and execution proof procedure to `docs/2-execution/10`. Route the canonical term to `docs/3-reference/02`.

---

### 4.2 A11-E2 — Story-sifting “may not create evidence” needs scoping after `INV-111`

**Target home:** `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`.

#### Foundation driver

The foundation now distinguishes observer-only emergence evidence from steering. Doc 09 permits observing emergent phenomena after the fact and forbids steering the world to improve the observer record (`TW-F-09:L19-L20`). Doc 12 requires retrospective emergence evidence as acceptance evidence (`TW-F-12:L24-L25`). `INV-111` says the observer evidence must be unable to feed cognition/schedulers/validators or set dramatic objectives (`TW-F-02:L77-L78`).

#### Current architecture coverage

A11’s core rule is correct: incidents, leads, reports, obligations, notices, contracts, rumors, and story summaries are world artifacts or observer projections; they do not spawn truth, repair pacing, select culprits, guarantee rewards, or wait for the player (`TW-A-11:L0-L2`). Its story-sifting section says story sifting may group events, highlight contradictions, summarize a no-human day, identify unresolved questions, compare holder beliefs/truth in debug mode, and generate recaps. It also says story sifting may not spawn events, choose actors’ goals, repair pacing, reveal hidden truth in embodied mode, mark quest completion, distribute rewards, or **create evidence** (`TW-A-11:L6-L7`).

Before `INV-111`, “may not create evidence” was a sensible anti-quest guard. After `INV-111`, unqualified wording can be misread as forbidding the very observer-only acceptance evidence the foundation now requires. The intended boundary is narrower: story sifting must not create **diegetic evidence**, holder-known evidence, institution-known evidence, proof inside the world, or events. It may produce observer-only review evidence when A13’s one-way data contract is satisfied.

#### Tier-fit verdict

**Belongs in architecture as a correction of stale/overbroad text.** This is not a new mechanic; it is an authority boundary correction. A11 owns story-sifting projections; A13 owns acceptance artifacts. The correction keeps the no-director rule intact while making room for the new observer-only acceptance artifact.

#### Recommendation

Amend A11’s story-sifting section and acceptance implications to distinguish:

- forbidden evidence creation: diegetic/holder/institution/world evidence, clues, proof, records, sanctions, rewards, or action reasons produced by a sifter;
- allowed observer-only evidence: retrospective acceptance/review artifacts under A13, reproducible from event/projection input, carrying event-log ancestry, structurally quarantined from cognition/schedulers/validators.

No final wording should be authored in this report. The practical change is to scope “create evidence,” not to weaken the questless incident doctrine.

---

### 4.3 A03/A06-A — Provenance sufficiency definition

**Routing seed:** Theme A, “Provenance sufficiency.”  
**Target home:** `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`; `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`.

#### Foundation driver and campaign evidence

Foundation already states the `what`: important beliefs need provenance; no telepathy; evidence is not truth; actor-known cognition inputs must carry source/provenance sufficient for replay/debug; missing action-relevant provenance is rejection (`TW-F-02:L16-L20`, `TW-F-02:L68-L72`). Doc 04 makes provenance requirements broad and rejects raw state, debug, branch/test labels, LLM prose, untyped booleans, and display strings (`TW-F-04:L35-L40`). Doc 14 adds provenance classes and says string labels or booleans are not enough (`TW-F-14:L12-L15`).

The routing memo asks architecture to encode “a single provenance-sufficiency definition” for holder-known and institution-known cognition, with non-empty modeled source evidence and fail-closed constructors/projections (`TW-ROUTE:L6-L7`). The predecessor report’s campaign evidence traces the pressure through 0015, 0016, 0018, 0021, and 0025: empty source-event vectors, audit paths checking strings instead of replay-derived context, fail-closed provenance witnesses, fabricated harness provenance, and provenance-true taint/read-path fail-closed behavior (`TW-REPORT:L17-L25`). The spec ledger independently summarizes 0016/0018/0021/0025 as need-accounting/source-event witness/provenance-witness/perception-provenance/fail-closed work (`TW-SPEC-LEDGER:L21-L22`, `TW-SPEC-LEDGER:L25-L26`, `TW-SPEC-LEDGER:L31-L32`, `TW-SPEC-LEDGER:L39-L40`).

#### Current architecture coverage

A03 is strong. It says every decision boundary receives a sealed holder-known context with provenance, defines a context packet with `provenance_edges[]`, and requires every item used by candidate generation, method selection, planning, speech interpretation, institutional procedure, view-model affordance selection, lead interpretation, or LOD promotion to be addressable inside the packet (`TW-A-03:L2-L4`). It lists allowed provenance classes and forbidden context sources (`TW-A-03:L6-L7`). It also gives actor-known and institution-known transaction formulas (`TW-A-03:L9-L11`).

A06 is also strong. It owns epistemic data flow, source/provenance rules, memory, records, absence evidence, traces, and display-string limitations; it denies making a holder know a claim without a modeled source and denies promoting debug truth into belief (`TW-A-06:L0-L2`). It requires observations, beliefs, memory, records, rumors/testimony, absence, leads, traces, and privacy/filtering to carry source/links/scope (`TW-A-06:L4-L12`).

The gap is not silence; it is fragmentation. A03/A06 imply sufficiency but do not give the compact cross-doc rule the routing memo asks for: **what makes provenance sufficient**, and what must fail closed when it is missing, dangling, wrong-kind, or merely a label/boolean/display string. The closest explicit fail-closed details live in A00 hardening rows (`TW-A-00:L24-L31`, `TW-A-00:L41-L42`) rather than in the primary architecture contracts.

#### Tier-fit verdict

**Belongs in architecture as a consolidation/refinement.** Foundation says action-relevant cognition must be provenance-bearing. Architecture should define the subsystem-level sufficiency boundary across holder-known contexts, epistemic projection records, institution-known contexts, and TUI affordance inputs. Execution should later prove it with negative fixtures and harness checks.

#### Recommendation

Add a compact **provenance sufficiency** subsection to A03 and cross-link it from A06. Substance:

- A cognition/procedure/view-model input is provenance-sufficient only if it cites at least one modeled acquisition or derivation route appropriate to the asserted fact kind: observation, search/contact event, absence observation, memory of a prior modeled source, speech/testimony, record/public artifact, routine/role assignment, institutional procedure state, LOD summary event with ancestry, or explicit unknown/unverified placeholder.
- A source label, boolean, display sentence, fixture label, branch/test name, debug comparison, validator detail, or raw physical truth lookup is not provenance.
- Derived facts must preserve lineage: the cited source must be enough to replay/debug why this holder, institution, or projection may use this fact now.
- Missing, empty, dangling, wrong-kind, ambiguous, or forbidden-source provenance must fail closed before action-relevant cognition/procedure/affordance selection. It may still be available to debug as a non-diegetic failure artifact.
- Institution-known contexts are not a special exception; their records/reports/roles/procedure states need the same sufficiency rule.

Do not prescribe a newtype, vector shape, storage table, schema field name, or test fixture. Those are execution/implementation details.

---

### 4.4 A06-B — Memory freshness classifier

**Routing seed:** Theme B, “Memory freshness classifier.”  
**Target home:** `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`; A03 is the existing anchor.

#### Foundation driver and campaign evidence

Foundation doc 04 distinguishes truth, actor-known truth, belief, memory, stale belief, rumor/testimony, and institutional record. Memory is fallible, incomplete, and source-bound; stale beliefs are meaningful; ground truth does not automatically repair a mind, record, or rumor network (`TW-F-04:L30-L40`). The invariant file supports this through provenance and staleness invariants, especially `INV-026` and `INV-028` (`TW-F-02:L17-L20`).

The routing memo asks architecture to encode one observed-now vs remembered/stale classifier: remembered facts keep acquisition time and provenance, stay planning-available but downgraded, and ground truth never restamps memory as current (`TW-ROUTE:L7-L8`). The predecessor report identifies 0017 and 0020 as key evidence for projection freshness, actor-known staleness, remembered-food freshness, and supersession parity (`TW-REPORT:L32-L40`). The spec ledger confirms 0017 projection freshness and 0020 cognition-surface freshness/supersession parity as historical pressure (`TW-SPEC-LEDGER:L23-L24`, `TW-SPEC-LEDGER:L29-L30`).

#### Current architecture coverage

A03 already states the rule crisply: projection-backed holder knowledge uses one freshness rule across no-human cognition and embodied affordance views; current perception may surface as `observed_now`; older usable knowledge remains remembered belief with original source timing/provenance; consumers may not restamp aged projection records as current observation merely because they are selected for a later decision (`TW-A-03:L5`).

A06 has the data ingredients. A belief includes acquired tick, last verified tick, stale-after tick, observation links, contradiction links, and privacy scope. Memory requires provenance and can decay, become stale, conflict, be misremembered, or be shaped by role/relationship (`TW-A-06:L4-L6`). But A06, the epistemic doc, does not itself state the same classifier as a contract for observations, memories, belief records, and projection records. That makes A03 the only clear home for a rule that A06 should own as epistemic data flow.

#### Tier-fit verdict

**Belongs in architecture as an A06 addition; A03 is already-owned-close.** The classifier is a data-flow/authority boundary, not a foundation invariant or execution fixture. Execution should later prove parity across no-human and embodied surfaces.

#### Recommendation

Add a small A06 subsection under observation/belief/memory or before diagnostics. Substance:

- `observed_now` / current observation status is limited to the holder’s current modeled perception/contact/search window or equivalent current channel event.
- Older usable facts remain planning-available only as memory/belief/stale information, preserving source event, acquisition time, last verification, and provenance class.
- Selecting an old fact for a new decision does not restamp it as current observation; validation truth does not refresh it; debug comparison does not refresh it.
- No-human cognition, embodied TUI view models, notebooks, and holder-known contexts use the same freshness classifier so possession and UI do not get a fresher epistemic surface than autonomous actors.

Route proof procedures and parity fixtures to execution `04`/`10`.

---

### 4.5 A10-C — Believed-access affordance snapshots

**Routing seed:** Theme C, “Believed-access affordances.”  
**Target home:** `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`; cross-reference A03.

#### Foundation driver and campaign evidence

Doc 14 says hidden truth must not generate embodied affordance menus, no-human proposal lists, or decision traces; truth may validate but not plan (`TW-F-14:L1-L8`). Doc 06 makes perceived/current affordances and actual validation affordances separate layers (`TW-F-06:L25-L33`). Doc 08 says embodied view models derive from actor-known context and include affordances the actor perceives or believes possible, excluding hidden facts (`TW-F-08:L28-L34`).

The routing memo asks A03/A10 to encode holder-known context snapshots captured at observation time, validators using truth only to accept/reject, and observation-time carrier/attribute capture (`TW-ROUTE:L8`). The predecessor report traces this through embodied workplace availability, local exits, perceived blockers, carrier capture, and observation-time snapshots (`TW-REPORT:L41-L50`). The spec ledger specifically records 0024 embodied truth-access removal and 0025 embodied carrier census / observation capture (`TW-SPEC-LEDGER:L37-L40`).

#### Current architecture coverage

A03 requires a sealed holder-known context for view-model affordance selection and lists forbidden raw physical truth and projection completeness assumptions (`TW-A-03:L3-L8`). A04 already defines two affordance layers: perceived/holder-known affordances and actual validation affordances (`TW-A-04:L7-L9`). A10 says embodied view models are generated from a holder-known context for the bound actor plus permitted projections; semantic actions go through the ordinary proposal pipeline; why-not output must not reveal hidden true targets or validator-only details; debug views are structurally separate (`TW-A-10:L3-L11`).

The gap is again specific but important: A10 does not explicitly say that embodied view construction and semantic action entries rely on **observation-time snapshots** of visible place/carried-item/container/attribute facts, rather than live truth read later during rendering/preflight. A00 records this hardening evidence in the conformance table, but A10 should own the view-model contract (`TW-A-00:L60-L68`).

#### Tier-fit verdict

**Belongs in architecture.** The finding concerns the data flow into embodied view models and action menus. Validation truth and fixture proofs remain execution/implementation concerns.

#### Recommendation

Add to A10’s embodied view-model / semantic action sections. Substance:

- Embodied view models and semantic action entries consume holder-known context plus permitted projection records whose visible/carried/current-place attributes were captured at a modeled observation, bind/preflight, or perception boundary.
- The TUI may render observed labels/attributes and actor-known affordances from those captured records. It must not keep a live physical-state handle or re-read truth to “freshen” labels, carried-item attributes, routes, workplace availability, food sources, or hidden blockers.
- Validators may still reject a selected semantic action using authoritative truth, but rejection feedback must split actor-visible modeled feedback from debug-only details.
- Snapshot/capture applies to no-human parity surfaces and embodied possession; possession is not a knowledge upgrade.

Do not settle the separate possession-bind perception question here. This recommendation says only that once a modeled observation/preflight/capture exists, A10 must use that captured holder-known material rather than live truth.

---

### 4.6 A04/A05/A09-D — Single-charge derived accounting seam

**Routing seed:** Theme D, “Single-charge derived accounting.”  
**Target home:** A00 index row already exists; add compact contracts to A04, A05, and A09.

#### Foundation driver and campaign evidence

Foundation requires event-sourced causality, deterministic replay, causal ordinary-life needs, and meaningful state changes through events (`TW-F-02:L6-L13`, `TW-F-03:L0-L8`, `TW-F-05:L3-L5`, `TW-F-06:L9-L14`). It does not define tick classifiers or need-delta emitters. The predecessor report correctly placed single-charge accounting in Bucket 2: a real replay/accounting law, but a subsystem authority and proof problem rather than product-identity doctrine (`TW-REPORT:L51-L59`).

The routing memo assigns architecture A00/A04/A05/A09 the job of naming authoritative seams for derived need deltas and duration lifecycle: one tick-regime classifier, one open/close keying authority, no double charge, no drift across consumers (`TW-ROUTE:L9-L10`). The spec ledger records the campaign path: 0016 single-regime need accounting and duration terminal closure, 0017 shared open-duration authority and tick-ledger coverage, 0022 shared need-delta emitters, and later mutation/proof hardening (`TW-SPEC-LEDGER:L21-L24`, `TW-SPEC-LEDGER:L33-L36`). A00’s conformance rows are detailed: need-tick accounting authority, duration terminal set, tick-charge attribution, shared open-duration authority, and shared need-delta emitter perimeter (`TW-A-00:L16-L17`, `TW-A-00:L22-L23`, `TW-A-00:L45-L46`).

#### Current architecture coverage

A04 owns proposal ancestry, scheduler limits, validation authority, rejection/failure semantics, feedback, and reservations/durations. It says schedulers may complete due durations through events whose start event reserved the resource and may not convert raw needs/routines/tables into primitive actions (`TW-A-04:L0-L12`).

A05 owns actor cognition flow and says needs are pressures, not scripts; hunger/fatigue/safety can produce candidate goals, but a need cannot name the true target (`TW-A-05:L0-L12`).

A09 owns ordinary food/sleep/work/property/economy and says hunger produces pressure, sleep uses body-exclusive reservations, work uses reservations and fatigue/hunger effects, and the local economy must be event-sourced and actor-known (`TW-A-09:L6-L12`).

None of A04/A05/A09 states the compact single-owner accounting contract in a way future subsystem authors can apply without reading A00’s hardening table.

#### Tier-fit verdict

**Belongs in architecture as a subsystem ownership seam.** Execution owns replay/ledger evidence and fixtures proving no double-charge or drift. Architecture owns the rule that derived need deltas and duration lifecycle have one authoritative source per actor/time window and cannot be recomputed independently by consumers.

#### Recommendation

Add compact doctrine, not implementation details:

- **A04:** add a subsection near reservations/durations: derived need deltas, elapsed-time effects, duration completion/interruption, and body-exclusive open/close state must flow through one authoritative accounting seam. Schedulers, validators, action definitions, and projections may consume that seam; they may not independently charge the same tick/window or reconcile duplicate terminals silently.
- **A05:** clarify that actor decision transactions consume live need pressures from event-sourced agent state / authoritative accounting. Candidate generation may explain pressure but must not mint need deltas, supply proposal-side current-need values as authority, or let routine labels charge time.
- **A09:** state that ordinary-life systems such as food, sleep, work, fatigue, hunger, wages, and local-economy placeholders share the same event-sourced accounting and duration lifecycle. Stable replay or stable goldens are not enough if two consumers causally charge the same tick/window twice.
- **A00:** keep the hardening rows as conformance examples; optionally add a one-line index pointer after the detailed rows to the compact A04/A05/A09 seam once written.

Do not prescribe classifier names, data structures, function names, or fixture shapes.

---

### 4.7 A13-F — Typed observability for non-vacuous proof

**Routing seed:** Theme F, “Falsifiability / anti-vacuity.”  
**Target home:** `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`; execution/reference hand-off for proof mechanics.

#### Foundation driver and campaign evidence

Foundation already has harsh acceptance and replay/forensic causality; it does not need test mechanics. The predecessor report places falsifiability/behavioral proof in Bucket 2 because the concrete lesson concerns executable witnesses, mutation perimeters, negative cases, and non-vacuous guards (`TW-REPORT:L84-L97`). The routing memo asks A13 to encode the subsystem-level requirement that validation surfaces expose enough typed observability for execution to test non-vacuously (`TW-ROUTE:L10-L11`). The design-risk register names R-29, guard vacuity/decorative locks: locks that assert presence/count/hash/shape without proving forbidden behavior would fail (`TW-RISK:L106-L114`).

#### Current architecture coverage

A13 already says typed diagnostics are architecture, plausible behavior without proof fails, acceptance may not rely on display strings/manual inspection/looks-right/debug state/branch-specific behavior/LLM outputs, and invalid pass conditions include missing decision ancestry or display strings as only proof (`TW-A-13:L0-L11`). It requires artifact families including decision traces, validation reports, epistemic projection reports, debug reports, and anti-regression/static guards (`TW-A-13:L2-L7`).

The gap is small but worth encoding: A13 does not explicitly require observability surfaces to expose the **behavior-bearing fields** execution needs to build a live negative. Without that, execution can end up with artifacts that exist but cannot be falsified.

#### Tier-fit verdict

**Belongs in architecture as a typed-observability requirement; proof mechanics route forward.** Architecture should say each protected subsystem must expose typed facts that make a regression observable. Execution decides live negatives, mutation/metamorphic tests, sampled runs, pending/pass status, and ratchets.

#### Recommendation

Add to A13’s required artifact families or review checklist. Substance:

- For every validation/anti-contamination/replay/projection/diagnostic guarantee, the architecture surface must expose typed, path-under-test observability: responsible layer, source event/proposal/context IDs, checked facts, behavior witness fields, rejected/accepted stage, and enough ancestry to distinguish production-path behavior from fixture or harness fabrication.
- An artifact’s mere existence, shape, count, checksum, or display text is insufficient unless paired with typed behavior evidence appropriate to the protected claim.
- The architecture surface should be designed so execution can attach live negatives and mutation/metamorphic checks, but A13 must not define those tests, thresholds, commands, or pass statuses.

Route execution proof doctrine to `docs/2-execution/10`; route the R-27/R-28/R-29 evidence-honesty cluster to reference `01`.

---

### 4.8 X10/G — Acceptance-evidence and manifest-fingerprint honesty

**Seed:** Predecessor report-added theme G.  
**Target home:** route-forward, not architecture.

#### Foundation driver and campaign evidence

The predecessor report added this as a narrower sibling to anti-vacuity: acceptance artifacts can be honest about a check’s existence while dishonest about what bytes/path/status the check proves. It cited 0024–0025 for meta-witness residue, mutation-perimeter derivation, manifest fingerprint honesty, envelope fail-closed behavior, CI evidence hygiene, and pending mutation status not being a pass (`TW-REPORT:L98-L102`). The routing memo sends it to execution `10` and reference `01`, not architecture (`TW-ROUTE:L16-L18`). The design-risk register tracks acceptance-evidence reachability overstatement (R-27), incomplete correction closure (R-28), and guard vacuity (R-29) (`TW-RISK:L96-L114`).

#### Current architecture coverage

A13 already says display strings/manual inspection/“looks right” are invalid and demands typed artifacts (`TW-A-13:L0-L11`). That is sufficient architecture posture. Raw-byte fingerprint scope, evidence-map statuses, mutation-pending language, and acceptance-report certification wording are review-artifact/proof details.

#### Tier-fit verdict

**Route-forward.** This is execution/reference evidence discipline. Do not encode fingerprint algorithms, status taxonomy, or acceptance-artifact template details in architecture.

#### Recommendation / hand-off

- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`: evidence statuses must distinguish pass, pending, sampled, observation-only, and non-certifying; fingerprints must state and cover their byte scope; path-under-test evidence must not be harness-fabricated.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: keep R-27/R-28/R-29 connected as one acceptance-evidence honesty relapse cluster.
- Possible later specs/templates may operationalize this after execution/reference decisions.

No direct architecture amendment beyond A13-F is recommended.

---

### 4.9 A08-OQ1 — Institution-known provenance residual

**Residual source:** predecessor report §11 and routing memo residuals.  
**Target home:** A08/A03 already; future proof may route to execution `11` or future Phase 4 docs.

#### Foundation driver

Foundation doc 07 says institutions are fallible social machines and institution “knowledge” must come from records, beliefs, member beliefs, public notices, procedures, or modeled sources, not ground truth (`TW-F-07:L0-L5`, `TW-F-07:L28-L37`). Doc 14 explicitly extends the truth firewall to institutions and records: institutions and procedures must not read hidden world truth; records are belief carriers with source/custody/provenance; institutional outcomes require modeled reports/records/procedure state (`TW-F-14:L21-L22`). The invariant file also says institutions are not omniscient and records need authors/custody/provenance/staleness (`TW-F-02:L31-L34`).

#### Current architecture coverage

A03 has an institution-known formula requiring sealed institution-known context, records/reports/roles/jurisdiction/resources/procedure state/public artifacts, procedure candidate generation or role decision, ordinary or institutional proposal, validation/event/rejection, modeled knowledge updates, and debug truth comparison without leakage (`TW-A-03:L11`).

A08 is direct and sufficient for the current architecture pass. It says institutions and households act through records, roles, resources, jurisdiction, norms, public artifacts, reports, procedures, and institutional memory; they do not read hidden truth (`TW-A-08:L0-L2`). Its minimum institution-known context includes `provenance_edges`, and every institutional decision must cite the context (`TW-A-08:L3`). It separates roles/powers, norms, procedure chain, records, notices/orders, sanctions, households, bias/error/corruption, and diagnostics. It explicitly gives an invalid example where the watch knows culprit truth from event-log truth and sanctions immediately (`TW-A-08:L10-L13`).

#### Tier-fit verdict

**Already-owned-close for architecture doctrine.** The residual concern is real as a future proof risk, but A08 and A03 already state the architecture contract. If future Phase 4 institutions repeat unbacked facts, that is a fresh implementation/execution or focused architecture reassess issue, not a current architecture-tier hole.

#### Recommendation

No architecture amendment now. Carry forward a watch item:

- Future institution work must prove A08’s institution-known context with provenance edges in execution `11` / institution proof surfaces.
- If future mechanics introduce new institution subsystems not covered by records/reports/roles/procedure state/evidence thresholds, revisit A08 then.

---

### 4.10 A00/A10-OQ2 — Possession-bind perception residual

**Residual source:** predecessor report §11 and A00 row.  
**Target home if later decided:** A00/A03/A10, and possibly foundation clarification if the owner changes possession doctrine.

#### Foundation driver

Foundation says human focus is not privilege (`INV-087`), possession transfers no world knowledge (`INV-006`), possession is not a cognition upgrade (`INV-108`), and embodied mode shows actor-known reality (`INV-067`) (`TW-F-02:L3-L4`, `TW-F-02:L45-L46`, `TW-F-02:L56-L57`, `TW-F-02:L73-L74`). Doc 08 says possession changes control/viewpoint, not knowledge/memory/guilt/identity/privilege; if the human acts out of character, the pipeline may allow only actions supportable as actor-known, reckless/speculative/search-based, or socially plausible — never hidden certainty (`TW-F-08:L11-L16`, `TW-F-08:L28-L31`).

#### Current architecture coverage

A10 says possession is controller binding, not physical world state; it cannot reset intentions, needs, memory, routines, location, relationships, institutional status, or knowledge, and a possessed actor acts through the same proposal/validation pipeline as an unpossessed actor (`TW-A-10:L0-L3`).

A00 records the unresolved decision directly: “Bind-time perception events remain an unresolved doctrine tension: either move perception into an actor decision transaction or approve a foundation clarification that possession-triggered modeled perceptions are allowed. This pass records the owner decision as deferred and makes no silent constitutional edit” (`TW-A-00:L74-L75`).

#### Tier-fit verdict

**Owner decision; do not silently amend architecture now.** This is not merely an architecture data-flow gap. It asks whether possession binding itself may emit modeled perception, which could touch foundation possession parity. The correct architecture action is to preserve the unresolved state and route the decision to the owner/reassess path.

#### Recommendation

Do not decide the policy in this report. Carry it forward as:

- If the owner decides bind-time possession may emit modeled perception, architecture home should be A10 possession semantics plus A03 holder-known context construction, with A00 recording the decided stance. The rule must say the perception is a modeled event/channel available to the actor, not a human/player knowledge transfer.
- If the owner rejects bind-time possession perception, architecture should state that binding merely selects an existing actor-known context/view and perception occurs only through ordinary actor decision/perception pipeline.
- Either way, execution must later test possession parity and no hidden truth transfer.

---

## 5. Forward-routing appendix

| Theme | Target tier | Target document(s) | Hand-off substance |
|---|---|---|---|
| Emergence evidence mechanism after A13 data contract | `2-execution` | `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Keep `EMERGE-OBS` observer-only. Define table/command/counters/thresholds/ratchets, path-under-test ancestry, anti-Goodhart constraints, and non-certifying/observation-only status. Do not let counters become simulation objectives. |
| Emergence evidence terminology | `3-reference` | `docs/3-reference/02_GLOSSARY.md` | Add a canonical term such as `emergence evidence` or `observer-only emergence evidence`; current glossary has `evidence`, `observation`, `projection`, and `story sifting`, but no emergence-evidence term (`TW-GLOSS:L46-L47`, `TW-GLOSS:L60-L67`; no match for “emergence evidence”). |
| Provenance sufficiency proof mechanics | `2-execution` | `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`; `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Prove unbacked actor-known/institution-known facts fail closed; harness provenance is real, not fabricated; dangling/wrong-kind/empty provenance fails on the path under test. |
| Memory freshness proof mechanics | `2-execution` | `04`; `10` | Prove one freshness classifier, no truth restamping, acquisition/provenance preservation, and parity across no-human and embodied surfaces. |
| Believed-access affordance proof mechanics | `2-execution` | `04`; `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`; `10` | Add wallhack negatives, observation-time snapshot proof, embodied carrier census, actor-visible/debug split, and possession parity. |
| Single-charge derived accounting proof | `2-execution` | `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`; `10` | Replay/ledger evidence proving no duplicate tick/window charges, no drift among scheduler/action/projection consumers, and no byte-stable-but-causally-false goldens. |
| Anti-vacuity proof machinery | `2-execution`; `3-reference` | `docs/2-execution/10`; `docs/3-reference/01` | Execution: every lock has a live negative or explicit reason why one cannot exist; artifact-presence checks pair with behavior witnesses; pending evidence is not a pass. Reference: keep R-29 connected to R-27/R-28. |
| Acceptance-evidence / fingerprint honesty | `2-execution`; `3-reference`; possible later `4-specs` template | `docs/2-execution/10`; `docs/3-reference/01`; maybe `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | Evidence status taxonomy, raw-byte fingerprint scope, path-under-test source claims, pending/sampled/observation-only/non-certifying language, acceptance-artifact honesty. |
| Institution-known provenance future proof | `2-execution` | `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` or future institution execution docs | A08 doctrine is already present; future institution work must prove records/reports/roles/procedure-state provenance and no event-log truth conversion. |
| Possession-bind perception | Owner / future architecture reassess; then `2-execution` | Possible A00/A03/A10 update after owner decision; execution `07` proof later | Decide whether possession binding may emit modeled perception. Do not implement by convention. After decision, prove no possession knowledge transfer or hidden truth menu exposure. |

---

## 6. Residual open questions

1. **Institution-known provenance.** Closed for current architecture doctrine: A03/A08 already own institution-known provenance. Still a future watch item because Phase 4 institutional systems may introduce new mechanics and repeat unbacked facts.

2. **Possession-bind perception.** Not settled here. A00 records it as deferred; this report preserves that. The decision is owner/foundation-adjacent before architecture should encode final policy.

3. **`EMERGE-OBS` thresholds / ratchets.** Architecture should not set numeric floors, counters, table names, commands, or ratchet policy. A13 should define the observer-only data contract only. Execution `10` owns thresholds/ratchets and must keep them observer-only.

4. **Glossary term for emergence evidence.** Reference session should add the term before execution/architecture specs proliferate synonyms. This report uses “observer-only emergence evidence” descriptively, following the amendment/routing language, not as a ratified glossary entry.

5. **No code corroboration.** This report is doc-tier conformance. I did not fetch Rust code files. The current architecture docs contain many conformance rows with code symbols; those are treated as architecture-document evidence, not as independent code audit results.

---

## 7. References

### Tracewake exact-commit sources

All Tracewake repository source citations in this report refer to the exact raw URLs listed in the ledger in §1 and fetched from `joeloverbeck/tracewake` at commit `fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9`.

### External prior art

No new external prior art was independently researched for this architecture-tier pass. The predecessor foundation-amendment report contains an external survey and is cited here only as a fetched Tracewake input/routing artifact. This report’s recommendations rest on the current Tracewake foundation, architecture, routing memo, spec ledger, and archived amendment provenance.
