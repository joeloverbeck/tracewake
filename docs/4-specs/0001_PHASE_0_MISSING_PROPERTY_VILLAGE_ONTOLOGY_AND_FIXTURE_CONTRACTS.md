# Spec 0001 — Phase 0: Missing Property Village Ontology and Fixture Contracts

## 1. Status and authority

This is a Phase 0 specification package for Tracewake.

It is subordinate to the foundation, architecture, execution, and reference documents at target commit `3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e`. It is not a replacement constitution, not a revised architecture, not a coding plan, and not an implementation ticket list.

The purpose of this spec is to make Phase 0 concrete enough that a later Phase 1 implementation agent can begin kernel/TUI/event-log/replay work without inventing product doctrine, ontology, fixture semantics, schema boundaries, or test meaning.

Authority order for this spec:

1. foundation doctrine;
2. architecture contracts;
3. execution phase gates and fixtures;
4. reference guardrails;
5. current analysis and research synthesis;
6. this spec package.

If a lower layer conflicts with a higher layer, the higher layer wins. If this spec conflicts with higher authority, correct this spec rather than weakening the higher authority.

## 2. Source and research basis

**Requested repository:** `joeloverbeck/tracewake`  
**Target commit:** `3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e`  
**Freshness claim:** user-supplied target commit only; not independently verified as latest `main`.  
**Manifest role:** path inventory only.

### Exact source discipline ledger

- Repository metadata used: no.
- Default-branch lookup used: no.
- Branch-name file fetch used: no.
- Code search used: no.
- Clone used: no.
- URL fetch method: exact raw GitHub URLs under `joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e`.
- Connector/tool namespace trusted as evidence: no.
- Contamination observed: no source contamination. A local network fetch attempt failed before returning content and was not used as evidence.

Exact fetch ledgers belong to review/spec artifacts, not permanent foundation doctrine. Future source ledgers should be kept with audit material or spec-package provenance, not repeated as product doctrine.

### Fetched repository files

- [docs/0-foundation/00_FOUNDATION_INDEX.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/00_FOUNDATION_INDEX.md)
- [docs/0-foundation/01_PROJECT_CHARTER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/01_PROJECT_CHARTER.md)
- [docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md)
- [docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md)
- [docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md)
- [docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md)
- [docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md)
- [docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md)
- [docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md)
- [docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md)
- [docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md)
- [docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md)
- [docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md)
- [docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md)
- [docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md)
- [docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md)
- [docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md)
- [docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md)
- [docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md)
- [docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md)
- [docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md)
- [docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md)
- [docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md)
- [docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md)
- [docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md)
- [docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md)
- [docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md)
- [docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md)
- [docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md)
- [docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md)
- [docs/2-execution/01_EXECUTION_CHARTER_SCOPE_AND_FIRST_PROOF.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/01_EXECUTION_CHARTER_SCOPE_AND_FIRST_PROOF.md)
- [docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md)
- [docs/2-execution/03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md)
- [docs/2-execution/04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_CONTRACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_CONTRACTS.md)
- [docs/2-execution/05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md)
- [docs/2-execution/06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md)
- [docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md)
- [docs/2-execution/08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md)
- [docs/2-execution/09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md)
- [docs/2-execution/10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md)
- [docs/2-execution/11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md)
- [docs/2-execution/12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/2-execution/12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md)
- [docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md)
- [docs/3-reference/01_DESIGN_RISK_REGISTER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/3-reference/01_DESIGN_RISK_REGISTER.md)
- [docs/3-reference/02_GLOSSARY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/3-reference/02_GLOSSARY.md)
- [docs/README.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e/docs/README.md)

### Research areas consulted

Focused external research was used as pressure and correction, not as permission to override Tracewake doctrine casually. See `0001_RESEARCH_NOTES.md` for source summaries.

Research areas consulted:

- event sourcing, CQRS, replay, event schema/version concerns, and save packages;
- deterministic simulation and seeded/auditable randomness;
- BDI agents, durable intentions, HTN routines, and bounded GOAP/STRIPS-style planning;
- smart objects and affordance-based interaction;
- agent-based social simulation validation;
- Talk of the Town-style character knowledge propagation, misremembering, and lying;
- Prom Week / Comme il Faut-style social state and social rules;
- Neighborly-style community-scale emergent narrative simulation;
- normative multi-agent systems, institutional facts, norms, proof, sanctions, and roles;
- dynamic level-of-detail and multi-resolution simulation;
- emergent narrative and story-sifting as observer/projection layer rather than director;
- game precedents and counterexamples, including Shadows of Doubt, Dwarf Fortress, The Sims-style routines, Ultima-style schedules, RimWorld storytellers, Left 4 Dead's AI Director, and Skyrim Radiant-style quest generation;
- LLM social simulation validation risks and why LLMs must remain behind structured speech/rendering/parsing boundaries.

## 3. First-proof identity

The first proof is:

**Missing Expected Property / The Missing Property Village**

It proves that Tracewake can simulate an ordinary social world where a missing physical thing matters because an actor expected it, searched for it, formed partial beliefs from evidence, propagated claims through structured speech, and could be understood through both embodied and debug views without privileged player knowledge.

The first-proof chain is:

```text
property expected somewhere
-> property moved or taken for modeled reasons
-> absence discovered through expectation/search/perception/instruction
-> partial knowledge propagates
-> report/record may occur
-> wrong suspicion may arise
-> possession parity proves no knowledge leak
-> debug explains truth, belief, traces, records, suspicion, and replay
-> no-human version can occur
```

This is not a notice-board proof, road-threat proof, bounty proof, companion proof, combat proof, graphical proof, procedural-town proof, or LLM conversation proof.

The first proof must work whether the human possesses Tomas, Mara, Anna, Elias, another valid actor, or no actor. Human control changes the chosen action proposals. It does not grant truth access, quest knowledge, special verbs, special consequences, or director-triggered drama.

## 4. Explicit non-goals

Spec 0001 forbids the following within its scope:

- implementation tickets;
- issue lists;
- sprint plans;
- Rust code;
- crate layout decisions;
- storage engine decisions;
- terminal UI library decisions;
- final content file syntax decisions;
- final serialization format decisions;
- detailed combat;
- beasts, monsters, or magic;
- route threat;
- old quarry or wilderness site;
- companion recruitment;
- bounty/proof/payment loop;
- notices as product feature;
- freeform LLM dialogue;
- graphical client;
- large region;
- procedural town generation;
- quest/objective ontology.

These exclusions are not a lack of ambition. They keep the first proof honest. The first proof must not hide ontology gaps behind genre systems, quest scaffolding, LLM improvisation, or UI drama.

## 5. Actor roster contract

The first village contains actors who live ordinary lives. No actor may exist only as a quest giver, culprit token, witness token, reward dispenser, protagonist foil, tutorial puppet, or suspicion marker.

Actor IDs are stable content IDs. Display names are not unique identifiers.

| Actor ID | Display name | Household / residence | Ordinary role / work routine | Access rights | Relationships / trust edges | Debts, obligations, needs, motives, pressures | Initial beliefs and expectations | Initial memories | Speech/report capabilities | Institutional role | Valid debug possession target | First-proof relevance | Failure / ambiguity role |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `actor_tomas` | Tomas | `household_tomas`; sleeps in `house_tomas/bedroom_tomas` | Miller and household head. Opens mill, tracks grain, pays helpers, keeps a private strongbox at home. | May enter all rooms of `house_tomas`, mill workspaces, public square, reeve's public office area. May open `strongbox_tomas`. | High trust: `actor_elena`; procedural trust: `actor_anna`; guarded trust: `actor_elias`; low trust under debt gossip: `actor_mara`, `actor_rafi`. | Must pay a flour-tax share and wages; wants household security; dislikes public embarrassment. | Expects `coin_stack_01` in `strongbox_tomas` because of seeded prior direct observation `obs_tomas_coin_check_dminus1`. Believes only he and Elena may touch the strongbox. | Remembers counting coins the prior evening, closing the strongbox, and intending to pay Rafi after morning milling. | Can ask, answer, tell, report missing property, accuse only with actor-known basis, refuse private details. | None. Household owner/custodian. | Yes. | Primary absence discoverer and property owner. | Can misremember whether he paid Rafi if fixture variant enables ambiguity; may over-suspect Mara or Rafi without proof. |
| `actor_mara` | Mara | `household_mara`; sleeps in `house_mara/sleeping_area` | Seasonal mill hand and domestic worker. Runs errands, sometimes helps Nika and Iva. | Public places; mill worker areas while scheduled; not Tomas's bedroom or strongbox; may enter Tomas main room only by invitation or household errand. | Needs Anna's procedural fairness; owes Iva; wary of Elias; mixed history with Tomas; Lena is confidant. | Debt to `actor_iva`; food insecurity; obligation to help Lena; pressure to hide shame; possible motive to move/take coins in selected fixtures. | Believes Tomas keeps cash somewhere private, but does not know exact contents unless she observes or hears. Believes Iva will press debt. | Remembers Iva's demand, Tomas's house routine, any modeled observation of container access. | Can ask, answer sincerely/uncertainly, deny knowledge, withhold, lie/mislead from modeled pressure, gossip, report. | None. | Yes. | Possible mover/taker, possible innocent suspect, source of motive pressure. | Must not be hard-coded as culprit. Can move property for reasons other than theft if fixture variant chooses. |
| `actor_elena` | Elena | `household_tomas`; sleeps in `house_tomas/bedroom_tomas` or adjoining alcove | Household worker, manages food, errands, and domestic access. | House rooms except Tomas may restrict strongbox access; may open pantry; may enter public square, tavern, reeve's office. | High trust with Tomas; polite trust with Anna; neighborly trust with Petra; uncertain view of Mara. | Wants household order; may want to avoid scandal; may protect Tomas's dignity. | Believes Tomas keeps coins in strongbox generally, but may not know exact amount today unless she saw it. | Remembers household traffic and possibly a sound/door event without visual certainty. | Can answer, report household facts, refuse gossip, repeat uncertain observation. | Household member, not formal authority. | Yes. | Potential partial witness; expectation amplifier if Tomas asks her to check. | Her heard sound is not a culprit pointer; she may conflate time/order. |
| `actor_anna` | Anna | `household_anna` near `reeves_office` or office cot if on duty | Clerk. Receives reports, writes records, maintains incident ledger, schedules follow-up. | Public office, clerk counter, records room, incident ledger; no private homes without invitation/procedure. | Procedural trust from Tomas and Reeve; Elias cooperates; Mara fears institutional exposure. | Duty to record accurately; pressure to avoid overburdening Reeve; reputation for fairness. | Believes reports are claims, not truth. Believes missing-property intake requires reporter, item description, source belief, and uncertainty. | Remembers prior petty disputes and who has complained, but not truth of unresolved claims. | Can ask intake questions, acknowledge reports, refuse incomplete claims, record hearsay, amend ledger. | Clerk of `institution_reeves_office`. | Yes. | Turns speech into institutional record without truth leakage. | May delay/refuse report if insufficient actor-known basis or wrong venue. |
| `actor_elias` | Elias | `household_elias` or guard cot at office | Guard. Watches public spaces, escorts questioning, observes without omniscience. | Public spaces, office, records only with Anna/Reeve; private spaces only by permission/procedure. | Trusts Anna's records; respects Reeve; suspicious of debtors; friendly with Soren. | Duty to maintain order; bias toward visible motive; wants quick containment. | Believes reports require investigation; may suspect actors with motive/access, not truth. | Remembers seeing Mara near the lane or Rafi at the mill if fixtures seed that observation. | Can ask, watch, tell Anna/Reeve, deny knowledge, form/report suspicion with basis. | Guard of `institution_reeves_office`. | Yes. | Institutional suspicion vector and fallible watcher. | Can form wrong suspicion from partial evidence; cannot read event log truth. |
| `actor_reeve` | Reeve | `household_reeve`; office hours in `reeves_office/private_desk` | Local authority. Reviews records, authorizes procedures, mediates property disputes. | Office public and private areas; institutional records; private homes only through procedure/invitation. | Anna is trusted clerk; Elias is guard; Tomas has standing; Mara has lower status; Iva is influential. | Obligation to preserve order and avoid unjust sanction; political pressure from households. | Believes institutional proof requires record plus evidence/testimony, not mere accusation. | Remembers prior disputes and sanctions at a coarse level. | Can receive summary, question, authorize watch, refuse sanction, amend institutional status. | Local authority. | Yes. | Keeps institution fallible but bounded. | May over-weight status unless later checks contradict; cannot resolve automatically. |
| `actor_iva` | Iva | `household_iva`; `house_iva/main_room` | Creditor, household head, small trader. | Own home/storage, public square, tavern; no Tomas private rooms. | Mara owes Iva; Petra trusts Iva's memory; Anna treats Iva formally. | Wants repayment; may pressure Mara; wants reputation as reliable trader. | Believes Mara owes debt; may believe Tomas has money because millers handle payments. | Remembers debt deadline and any public argument. | Can ask for payment, gossip, deny, report unrelated debt, withhold. | None. | Yes. | Creates modeled motive pressure without being director. | Her debt claim can be true while theft suspicion is false. |
| `actor_nika` | Nika | `household_nika`; operates `tavern_nika` | Tavern/food-place keeper. Sees public traffic and hears gossip. | Tavern, storage, public square; no private home access. | Knows Petra, Rafi, Soren; friendly to Anna; wary of Elias. | Needs payment for food; wants custom; avoids being dragged into disputes. | Believes many claims are rumor until sourced. | Remembers who bought food, who seemed anxious, and who spoke loudly. | Can tell rumor as hearsay, answer uncertainly, refuse private customer details. | None. | Yes. | Rumor propagation and public observation node. | Can amplify wrong suspicion if hearsay loses source. |
| `actor_petra` | Petra | `household_petra`; window facing lane | Weaver/neighbor. Works at home, sees lane traffic. | Own home, public square, tavern; no private homes. | Trusts Elena; trades with Iva; limited trust of Mara. | Needs quiet work; wants neighbor order. | Believes she saw movement only if fixture seeds line-of-sight; must not infer more than observed. | Remembers a figure, time window, direction, sound, or color, depending on fixture visibility. | Can answer as observation, repeat uncertainty, gossip if pressured. | None. | Yes. | Partial visual witness. | Ambiguous sightline may point at wrong actor or no actor. |
| `actor_rafi` | Rafi | `household_rafi_lena`; sometimes mill bunk | Mill helper. Moves between mill, Tomas's house for errands, and tavern. | Mill worker areas; Tomas main room by errand; no bedroom/strongbox; public spaces. | Owed wages by Tomas; friend of Soren; known by Nika; neutral with Mara. | Wants payment; resents delay; may need money for food. | Believes Tomas owes him wages; may believe Tomas keeps coins, but not exact location unless observed/heard. | Remembers asking about pay and being told to wait. | Can ask for payment, deny, answer, gossip, accuse only with basis. | None. | Yes. | Plausible wrong suspect due motive/access-adjacent routine. | Must not be treated as guilty because owed wages. |
| `actor_soren` | Soren | `household_soren`; travels local lane only | Carrier/farmhand. Moves goods between mill, square, tavern, and homes. | Public lanes, mill yard, tavern, some thresholds by errand. | Friend of Rafi; casual with Nika; known to Elias. | Must complete deliveries; may avoid being questioned. | Believes he saw errands or bundles only if seeded; does not know contents. | Remembers route timing and load. | Can answer, deny knowledge, repeat route observations, refuse delay. | None. | Yes. | Movement noise and timing ambiguity. | His ordinary route can look suspicious without being theft. |
| `actor_lena` | Lena | `household_rafi_lena` or `household_mara` depending fixture variant | Seamstress/caretaker, confidant to Mara and Rafi. | Own home, public square, tavern; no private strongbox access. | Trust edge to Mara and Rafi; wary of Iva; polite with Anna. | Needs household food; may urge Mara to repay or tell truth. | Believes debt pressure exists if Mara told her; may not know whether any theft occurred. | Remembers Mara's emotional state and conversations. | Can answer, withhold confidences, repeat hearsay with source if modeled. | None. | Yes. | Separates motive knowledge from action knowledge. | Could mislead by omission to protect Mara without knowing truth. |
| `actor_corin` | Corin | `household_corin`; near office | Runner/assistant used only when useful. Carries messages between office and village. | Public spaces, office counter; no records room without Anna; private homes only at threshold. | Looks up to Anna/Elias; gossip source for Nika if careless. | Wants approval; may rush. | Believes messages should be delivered, not interpreted. | Remembers message delivery time and recipient. | Can carry structured notice/report summary, answer limited route questions, refuse private record access. | Optional office runner, not authority. | Yes, if included in fixture. | Supports report delay/amendment variants. | Must not become a quest courier or objective marker. |

## 6. Place, room, door, and container roster

The first village is deliberately small. It is not a generated town, not a region, and not a wilderness map. It is a concrete fixture graph that can be reviewed before implementation.

### 6.1 Place and room roster

| ID | Kind | Parent | Privacy | Jurisdiction | Connections | Visibility profile | Sound profile | Affordances | First-proof role |
|---|---|---|---|---|---|---|---|---|---|
| `village_core` | settlement scope | none | mixed | informal village custom; formal disputes to `reeves_office` | contains public square, homes, mill, tavern, office | high-level map only | ambient village sound | local movement, no-human advance marker | Defines first-proof spatial boundary. |
| `public_square` | public place | `village_core` | public | `institution_reeves_office` has public-order authority | lanes to office, mill, Tomas, Mara, tavern, supporting homes | broad visibility, many observers, poor privacy | conversations audible near actors; crowd noise | move, wait, ask, tell, gossip, go-to-authority | Claim propagation and public movement hub. |
| `reeves_office` | institution building | `village_core` | public front / restricted rear | `institution_reeves_office` | public square, clerk counter, records room, private desk | front room visible; records restricted | speech carries inside; outside muffled | report, receive report, write record, question, refuse | Converts report speech into institutional claims. |
| `office_counter` | room/desk zone | `reeves_office` | semi-public | `institution_reeves_office` | public front, records room door | Anna can see entrant; entrants see desk, not ledger contents | speech audible to nearby office actors | ask, report, acknowledge, refuse, wait | Intake surface for missing-property report. |
| `records_room` | restricted room | `reeves_office` | restricted | `institution_reeves_office` | office counter via `door_records_room` | hidden from public front when door closed | muffled | open ledger, amend record, search records | Holds incident ledger artifact. |
| `private_desk_reeve` | private work zone | `reeves_office` | restricted | Reeve / institution | office interior | seen only by office staff | private conversation if door/spacing allows | review, question, authorize | Later Phase 4 procedure sketch only. |
| `house_tomas` | home | `village_core` | domestic private | `household_tomas`; village trespass norm | public lane, main room, bedroom, pantry | threshold visibility; interior private | door sounds and household voices | enter by permission, inspect, search with rights, take/place household goods | Home of expected property. |
| `tomas_main_room` | room | `house_tomas` | household private | `household_tomas` | front door, bedroom door, pantry | visible from doorway only when open/near | voices audible at threshold | wait, speak, household work | Non-strongbox access ambiguity. |
| `bedroom_tomas` | room | `house_tomas` | high private | `household_tomas` | main room via bedroom door | not visible from lane; strongbox visible only if actor enters/searches | door/footstep sounds possible | search, open strongbox if permitted/able | Location of `strongbox_tomas`. |
| `pantry_tomas` | room/storage nook | `house_tomas` | household private | `household_tomas` | main room | shelves visible when entered | quiet | open pantry, take/place food | Separates household food from private coins. |
| `house_mara` | home | `village_core` | domestic private | `household_mara` | public lane, main room, sleeping area, small cache | threshold only | quiet; door sounds | hide/place item, speak privately, search if permitted/procedure | Possible hiding/motive location. |
| `workplace_mill` | workplace | `village_core` | work-public during hours; restricted storage | customary mill rules; Tomas manages | mill yard, workroom, grain storage, lane | yard visible, workroom partial, storage restricted | loud machinery masks quiet movement | work, move, ask payment, place/take work items | Routine reason for actors to move. |
| `mill_yard` | outdoor work area | `workplace_mill` | work-public | Tomas work authority | square lane, mill workroom | visible from lane and square edge | loud intermittent | move goods, wait, observe | Plausible actor traffic without scripting. |
| `mill_workroom` | room | `workplace_mill` | work-restricted | Tomas work authority | yard, grain storage | visible to workers | machinery masks speech | work, ask, search allowed zones | Rafi/Mara ordinary work context. |
| `grain_storage` | restricted storage | `workplace_mill` | restricted | Tomas work authority | workroom door | low visibility | muffled | open/close, inspect grain bins | Access/control contrast with home strongbox. |
| `tavern_nika` | public food place | `village_core` | public front / private storage | `household_nika`; public norms | square, tavern storage | high public visibility | conversations can spread | buy food as future economy hook, gossip, ask, wait | Rumor and overheard claims. |
| `house_iva` | supporting home | `village_core` | domestic private | `household_iva` | square lane | threshold only | quiet | ask debt, speak, store goods | Motive-pressure source. |
| `house_petra` | supporting home | `village_core` | domestic private | `household_petra` | lane facing Tomas/Mara route | window line-of-sight to lane only | can hear carts/doors faintly | observe from window, answer | Ambiguous witness sightline. |
| `house_rafi_lena` | supporting home | `village_core` | domestic private | `household_rafi_lena` | lane to mill/square | threshold only | quiet | rest, private talk | Wrong-suspicion and wage pressure context. |
| `house_soren` | supporting home | `village_core` | domestic private | `household_soren` | lane to mill/square | threshold only | quiet | route start/end | Ordinary movement ambiguity. |
| `local_lane_mill` | tiny local route | `village_core` | public | public custom | connects square, mill, Tomas, Mara, Petra | seen from Petra window and square edge | carts/footsteps carry | move locally, observe, wait | Local movement only, not road-travel proof. |

### 6.2 Door roster

Doors are physical affordance objects, not quest locks. They mediate privacy, sound, line of sight, access, and trace opportunities.

| Door ID | Endpoints | Initial state | Sound / visibility occlusion | Access / privacy norm | Affordances | Failure cases |
|---|---|---|---|---|---|---|
| `door_tomas_front` | `public_lane_tomas` ↔ `tomas_main_room` | closed, unlocked while household active; may be latched | closed blocks clear sight; opening produces audible door sound nearby | visitors should knock or be invited; household members may enter | open, close, knock-as-ask variant, enter/exit | actor lacks permission; door stuck; actor refuses trespass; someone notices entry. |
| `door_tomas_bedroom` | `tomas_main_room` ↔ `bedroom_tomas` | closed, unlocked | closed blocks sight to strongbox; footstep/hinge sound can be heard nearby | high household privacy; non-household entry requires invitation/procedure | open, close, move through, search from threshold if open | actor cannot justify entry; blocked by occupant; later suspicion from privacy violation. |
| `door_pantry_tomas` | `tomas_main_room` ↔ `pantry_tomas` | open or ajar | low visibility occlusion | household food access, not private coin access | inspect, take/place food | confusing food custody with coin ownership is invalid. |
| `door_mara_front` | lane ↔ `house_mara/main_room` | closed, unlocked/latch | blocks sight; opening audible from inside | domestic privacy | open, close, enter by right/invitation | authority cannot enter without procedure; search without basis rejected. |
| `door_mill_front` | `mill_yard` ↔ `mill_workroom` | open during work, closed after hours | partial sound mask from mill | workers may enter on routine | open, close, move goods | after-hours access becomes suspicious but not proof. |
| `door_grain_storage` | `mill_workroom` ↔ `grain_storage` | closed, unlocked for workers or latched | blocks line of sight | restricted to mill work | open, close, inspect | actor lacks work reason; obstruction by load. |
| `door_office_front` | `public_square` ↔ `office_counter` | open during office hours | little occlusion when open; closed muffles | public report entry | enter, leave, wait, report | office closed; actor refuses public exposure; Anna unavailable. |
| `door_records_room` | `office_counter` ↔ `records_room` | closed, locked/restricted | blocks ledger visibility; muffles conversation | Anna/Reeve/Elias by role; public no access | open, close, lock/unlock as later implementation detail, enter | unauthorized access rejected; record not visible to reporter. |
| `door_tavern_front` | `public_square` ↔ `tavern_nika` | open during service | public visibility | public food place | enter, leave, ask, gossip | closed; Nika refuses disruptive dispute. |
| `door_house_petra_front` | lane ↔ `house_petra/main_room` | closed | no interior visibility; window separate | domestic privacy | open/close/answer door | Petra can witness lane from window without door access. |

### 6.3 Container roster

Containers hold physical items. They are not abstract inventory slots. Contents, expected contents, access permissions, trace profile, and search behavior matter.

| Container ID | Location | Initial contents | Initial state | Owner / custodian | Permitted users | Privacy | Trace profile | Expected contents by actor | Visibility and search behavior | Affordances | Failure cases |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `strongbox_tomas` | `bedroom_tomas` | `coin_stack_01`, optional `receipt_scrap_tomas` | closed, locked or latchable; exact lock mechanism deferred | legal owner and custodian: Tomas | Tomas; Elena only if fixture explicitly grants emergency access | high private | lid marks, dust disturbance, touch trace, missing-position trace, sound if opened | Tomas expects `coin_stack_01`; Elena expects coins only generally if she has source; others no valid expectation without source | contents visible only if actor can see/open/search; empty state is evidence only for actors with expectation | open/close, inspect, search, take/place item | no access; actor does not know to search; lock prevents action; opening creates trace; empty search without expectation is not evidence. |
| `pantry_tomas_shelf` | `pantry_tomas` | household food, jars | open shelves | `household_tomas`; custodian Elena/Tomas | household members | household private | missing food visible, low trace | Elena/Tomas expect food stock; visitors do not | visible when in pantry | take/place food | household food is shared custody, not Tomas's private coin property. |
| `basket_mara` | `house_mara/main_room` or carried by Mara | cloth, food, optional hidden item variant | open or covered | Mara | Mara, Lena by trust | private/personal | cloth disturbance, concealment trace | Mara knows contents; Lena may know if told/observed | visible only when open/search/carry posture reveals | carry, place, hide | public search requires procedure; concealed item not magically known. |
| `small_cache_mara` | `house_mara/sleeping_area` | personal scraps; optional `coin_stack_01` variant | closed, not formal lock | Mara | Mara; Lena maybe | high domestic private | dust/cloth movement, hiding trace | Mara knows if she placed item; others no expectation | search required, privacy norm strong | open/close/search/place/take | unauthorized search rejected; actor may choose to avoid privacy violation. |
| `mill_pay_box` | `mill_workroom` | low-value work tokens or empty | closed | Tomas as work custodian | Tomas; maybe Rafi for work deposits if explicit | work-restricted | visible workplace handling | Rafi may expect wages here only if told; Tomas knows current state | search from workroom if permitted | open/search/take/place work items | cannot be confused with `strongbox_tomas` unless actor has mistaken belief. |
| `grain_bin_mill` | `grain_storage` | grain sacks | open/closed bin | Tomas/workplace | workers during tasks | restricted workplace | footprints/dust | workers expect grain, not coins | visual bulk, search noisy | open/search/place | hiding coins here possible but trace/noise; not default. |
| `incident_ledger_box` | `records_room` | `incident_ledger_01` | closed, restricted | `institution_reeves_office`; custodian Anna/Reeve | Anna, Reeve; Elias with procedure | restricted institutional | page order, ink amendments, access marks | Anna expects current ledger present | ledger visible only to authorized actors | open/search/read/write | records are claims, not truth; public cannot inspect by default. |
| `tavern_till_nika` | `tavern_nika/private_storage` | Nika's takings | closed | Nika | Nika | private commercial | coin-count changes | Nika expects her own money | search requires permission/procedure | open/take/place | prevents global “coins are fungible balance” shortcut. |
| `chest_iva` | `house_iva/main_room` | debt records, goods | closed | Iva | Iva | private | paper/lock traces | Iva expects debt note | search private | open/read/search | debt record is claim source, not theft proof. |
| `satchel_soren` | carried by Soren | delivery goods | carried/closed | Soren or employer depending goods | Soren | personal/work | carry posture, contents disturbed | Soren knows load, others not | search requires direct inspection | carry/open/search | ordinary carry must not be treated as hidden culprit pointer. |

## 7. Physical item/value-token contract

The first-proof valuable item is:

`coin_stack_01`

It is a physical, custody-tracked value token. It is not an abstract balance and must not be collapsed into an economy total.

| Field | Contract |
|---|---|
| ID | `coin_stack_01` |
| Kind | physical value token: small stack/pouch of coins, count may be known exactly by Tomas and unknown/approximate to others |
| Legal/social owner | Tomas / `household_tomas` depending later legal model; for Phase 0, Tomas is recognized owner and primary claimant |
| Initial possessor | `strongbox_tomas` as containing physical object; Tomas is not physically holding it at start |
| Initial custodian | Tomas as strongbox custodian |
| Expected location | `strongbox_tomas`, sourced from `obs_tomas_coin_check_dminus1` and/or prior seeded accounting memory |
| Permitted users | Tomas. Elena only if a fixture variant explicitly grants emergency household access. No general worker/public permission. |
| Value significance | Enough to matter for wages, debt, household security, and reportability; not so large that it creates a regional/criminal-case proof. |
| Portability/carry constraints | Small enough for any actor to carry/conceal in pocket, basket, satchel, or hand; movement should create possible traces depending container and route. |
| Visibility | Visible only when line-of-sight and container/clothing/search conditions allow. Absence visible only after expected-location inspection/search by an actor with a relevant expectation. |
| Transfer/movement actions | `take_item`, `place_item`, `hide_or_move_item`, possibly `item_relocated_by_process` for setup/prehistory only. |
| Observation/search behavior | Presence may create `item_observed_present`; expected absence may create `expected_item_observed_absent`; actor without expectation sees empty strongbox only as an empty container unless another source makes absence salient. |
| Trace possibilities | disturbed strongbox dust, lid sound, pocket/basket bulge, dropped coin, witness of actor near room, record of prior count, inconsistent speech. Traces are not truth and may be absent or ambiguous. |
| Record/expectation references | Tomas's report may reference `coin_stack_01` description and expected location. Anna's ledger records the claim, source, uncertainty, and reporter. |
| How absence becomes evidence | Only through expectation plus observation/search: prior belief `coin_stack_01 in strongbox_tomas` + search of strongbox + observed absence + no immediate accepted explanation. |

Other coin-like objects may exist only if they do not blur the central custody proof. If later fixtures add `tavern_till_nika` or wages, their item IDs must remain distinct from `coin_stack_01` unless a modeled transfer event creates identity continuity.

## 8. Ownership, custody, access, control, proof, and belief model

Tracewake must keep the following concepts separate.

| Concept | Meaning | Example: Tomas's coins | Example: household food | Example: incident ledger | Example: stolen/hidden property |
|---|---|---|---|---|---|
| Legal/social ownership | Who the village recognizes as rightful owner or claimant under norms. | Tomas owns `coin_stack_01`. | Household food may be owned by `household_tomas`, with Elena/Tomas as ordinary users. | `institution_reeves_office` owns/controls the ledger artifact. | Ownership remains Tomas's even if Mara physically hides the coins. |
| Physical possession | Where the thing physically is, including carried-by or contained-in. | Initially inside `strongbox_tomas`; later may be in Mara's basket or cache. | Food on pantry shelf or carried by Elena. | Ledger physically in `incident_ledger_box` or on Anna's desk. | Hidden coins in `small_cache_mara` are possessed by that container, not by the truth system. |
| Custody | Who has accepted practical responsibility for care/access. | Tomas is custodian of the strongbox and coins. | Elena may be custodian of pantry stock. | Anna is day-to-day custodian of ledger; Reeve has authority. | Mara may have de facto custody while hiding property, but not legitimate ownership. |
| Access permission | Who is allowed to enter/open/use under social or institutional rules. | Tomas may open strongbox; Elena only if explicit; Mara/Rafi not permitted. | Household members may take food; visitors need permission. | Anna/Reeve may access ledger; public may report but not browse. | Authority search requires procedure; private search without basis may violate `trespass_privacy`. |
| Physical control | Who can physically act on the object now, regardless of permission. | A trespassing actor may be able to open/take if lock/state permits. | A hungry visitor may physically take food if unobserved. | Anna can write; an unauthorized intruder might physically damage but not legitimately amend. | A thief can move item but cannot make the institution know truth. |
| Proof/provenance | Evidence chain that justifies a claim, suspicion, record, or sanction. | Prior count + search absence + trace + testimony may support report. | Missing food plus household expectation may support minor household suspicion. | Ledger amendment has author/time/source speech references. | Recovery in cache plus access evidence may prove possession but not automatically motive or theft. |
| Belief about state | Actor's held proposition, with source, confidence, and time. | Tomas believes coins should be in strongbox; Anna believes Tomas claims this. | Elena believes pantry should have bread because she stocked it. | Reeve believes the ledger says a report was filed. | Elias may suspect Mara based on motive and location, even if wrong. |
| Institutional recognition | What an institution has accepted as record/procedure/status. | Report accepted as incident record, not proof of theft. | Household complaint may be outside formal incident threshold. | Ledger states claims and amendments. | A sanction requires institutional procedure, not debug truth. |

Forbidden collapse examples:

- Do not treat “Tomas owns coins” as “Tomas knows where coins are.”
- Do not treat “Mara possesses coins” as “Anna knows Mara did it.”
- Do not treat “ledger says Tomas reported theft” as “theft is proven.”
- Do not treat “actor has physical access” as “actor has permission.”
- Do not treat “empty container” as evidence for actors with no prior expectation.

## 9. Action vocabulary

Actions are proposed and adjudicated through the same action pipeline for human-controlled and AI-controlled actors. Human possession changes which actor proposes actions; it does not create special action semantics.

The spec names action families and contracts. It does not choose final Rust types, crate layout, content syntax, TUI library, or serialization.

### 9.1 Common action fields

Every ordinary action family must carry or derive:

- actor requirements;
- physical preconditions;
- belief/knowledge preconditions;
- social/normative preconditions;
- resource/time/duration;
- possible rejection;
- possible failure;
- events emitted on success/failure;
- traces/observations generated;
- TUI affordance visibility;
- debug explanation hook;
- same-action rule for human-controlled and AI-controlled actors.

Debug possession switching is a non-diegetic debug operation, not an ordinary actor action.

### 9.2 Primitive action families

| Action family | Actor requirements | Physical preconditions | Belief / knowledge preconditions | Social / normative preconditions | Resource / duration | Rejection / failure | Events emitted | Traces / observations | TUI affordance visibility | Debug hook | Parity rule |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `debug_attach_or_switch_possession` | Human/debug controller only; not an in-world actor action | Target actor exists and is a valid debug possession target | none | debug mode only | instant debug step | rejected if target invalid or fixture forbids debug attach | `debug_possession_changed` or debug-only marker; no world mutation | no diegetic trace | visible only in debug UI | explains current possessed actor, forbidden state mutation | AI never invokes; it is outside ordinary play and cannot prove anything. |
| `inspect_look` | actor conscious/present | actor has line of sight or sensory channel | none; may be curiosity/routine | privacy may make looking intrusive | short | rejected if no sensory path; failure if darkness/occlusion/noise | `observation_created`, possibly `item_observed_present` | sight/sound observations | shown when actor can plausibly inspect | why visible/not visible | same action for human/AI; filtered by actor senses. |
| `move_locally` | actor can move | connected endpoints; passable door/path | actor knows or can perceive route, or follows routine | trespass/privacy may block private destination | variable local travel | rejected if no route or forbidden; failure if interrupted/delayed | `actor_moved`, `intention_plan_delayed` if delayed | footstep/witness/route traces | visible as exits/routes known to actor | route cause, blockers, witnesses | same movement semantics for all actors. |
| `open_or_close_door` | actor adjacent to door | door exists; not blocked; lock state permits or actor has key/tool if modeled | actor perceives door and wants passage/privacy | access norms may forbid | short | rejected if locked/no permission; failure if stuck/noisy | `door_opened` / `door_closed`, or `action_rejected`/`action_failed` | sound, visibility change, witness | visible when adjacent and actor can interact | lock/norm/noise explanation | same for human/AI. |
| `open_or_close_container` | actor adjacent/holding container | container reachable; state permits | actor knows/perceives container; may require search intention | ownership/privacy norms apply | short | rejected if locked/forbidden; failure if jammed/noisy | `container_opened` / `container_closed`, `container_accessed` | touch/dust/noise/witness | visible for perceived reachable containers | access/norm/lock chain | same for human/AI. |
| `inspect_or_search_place_container` | actor present; enough time | place/container accessible; search space exists | either expectation, instruction, curiosity, routine, or procedure; absence evidence requires expectation/source | privacy/procedure may block | short to medium | rejected if no access/basis; failure if cursory search misses hidden item | `search_performed`, `observation_created`, `expected_item_observed_absent` or `item_observed_present` | disturbance traces; search witnessed | visible as inspect/search when actor has access and reason; intrusive actions marked | explains why empty mattered or did not | same for human/AI. |
| `take_item` | actor able to carry/reach item | item present and accessible; container open if needed | actor perceives item or searches successfully; knows/decides target | permission required unless actor chooses violation | short | rejected if item inaccessible/too heavy/forbidden; failure if interrupted | `item_transferred`, `container_accessed`, `action_rejected/failed` | missing-position trace, carry trace, witness | visible only if actor sees/knows item and can reach | ownership/access/control distinction | same; AI cannot take unseen item. |
| `place_item` | actor possesses item | destination reachable and can contain item | actor knows destination or is searching/hiding | destination permission/privacy applies | short | rejected if no capacity/access; failure if dropped/noisy | `item_relocated`, `container_accessed` | placement trace, possible witness | visible when carried item and destination affordance known | where item moved and why | same. |
| `hide_or_move_item` | actor possesses or can access item | concealment destination reachable | actor intends concealment, temporary storage, repayment avoidance, safekeeping, or theft | may violate ownership/privacy | short/medium | rejected if no concealment; failure if trace/witness/noisy | `item_relocated`, `trace_created`, `belief_updated` if actor interprets success | concealment trace; later search difficulty | visible as hide/move only when actor has motive and item/control | motive vs action vs proof | same; no scripted culprit shortcut. |
| `wait` | actor alive/present | location permits waiting | routine, uncertainty, social timing | loitering may be suspicious in private/restricted spaces | chosen duration | rejected if location forces movement; failure if interrupted | `intention_plan_delayed`, maybe `actor_waited` if later named | observation opportunities | visible generally, with privacy warnings | time advance and interruptions | same. |
| `ask` | speaker and listener/audience present or channel exists | can communicate | speaker has question proposition or information need | politeness/status/privacy may affect | short | listener refuses/unavailable/mishears | `speech_act_committed`, possible `belief_updated` | heard-by observations | visible for reachable actors and actor-known topics | question source and topic | same. |
| `answer` | listener becomes speaker | prior question or conversational context | speaker has belief, memory, uncertainty, or denial | may be obligated, private, or cautious | short | cannot answer; refuses; answers uncertainly | `speech_act_committed`, belief updates in listener | heard claim | visible as response options based on actor beliefs | sincerity/confidence debug | same. |
| `tell` | speaker has claim to communicate | listener/channel available | claim source exists: observation, memory, hearsay, record, inference, lie | speakability/privacy norm | short | listener refuses/does not hear; speaker withholds | `speech_act_committed`, `belief_formed/updated` possible | overheard speech | visible only for actor-known claims | source chain and omitted facts | same. |
| `report_missing_property` | reporter has actor-known missing-property claim | authority receiver available or report can be delayed | reporter believes item expected and absent, or reports hearsay with mode | report venue/role; false report norms later | medium | rejected if no reporter/source/item/expectation; delayed if office unavailable | `speech_act_committed`, `report_received/refused/delayed` | office witness, ledger precursor | visible when actor knows enough and can reach authority | why report allowed/blocked | same; AI Tomas can report without human. |
| `gossip` | speaker/listeners in social setting | conversational channel | claim may be hearsay/speculation; source should be retained | reputation/privacy norms | short | listener uninterested/refuses; claim degrades | `speech_act_committed`, `belief_updated/weakened` | overheard rumor | visible in tavern/square with known rumor | source loss/uncertainty | same. |
| `refuse_or_withhold` | actor addressed or pressured | communicative situation | actor has reason: privacy, fear, uncertainty, duty | norms may permit/forbid refusal | short | refusal may fail under procedure later | `speech_act_committed`, `report_refused` if institutional | suspicion/reputation trace | visible as response if actor has motive/right | reason hidden/visible split | same. |
| `lie_or_mislead` | speaker has claim and motive/pressure | channel available | speaker knows or believes statement conflicts with private belief, or strategically omits | norm violation if discovered; requires modeled motive | short | blocked if no motive/knowledge gap; failure if contradiction exposed | `speech_act_committed`, `belief_updated`, debug sincerity marker | possible contradiction trace | embodied UI shows statement, not hidden truth | debug-only lie/sincerity visibility | same; AI lies only with modeled motive. |
| `accuse_or_form_suspicion` | actor can reason/socially speak | target actor exists; channel if spoken accusation | actor-known basis required: motive/access/observation/hearsay/record; reckless accusation only if modeled | defamation/order norms; institutional threshold | short/medium | rejected if no actor-known basis; blocked if actor tries to use debug truth | `suspicion_formed`, `speech_act_committed`, or `action_rejected` | social trace, record if reported | visible only when basis exists or reckless mode allowed | why-not basis list | same. |
| `go_to_local_authority` | actor can move | route to `reeves_office` | actor believes authority relevant | office hours/access norms | local travel | fails if office closed/interrupted/refusal | `actor_moved`, maybe `intention_plan_updated` | route witness | visible when actor knows office and has reason | route/intent explanation | same. |
| `receive_report` | Anna/Reeve/authorized office actor | reporter present/channel | understands structured report fields | institutional role required | medium | refused/delayed/incomplete | `report_received/refused/delayed`, `speech_act_committed` | office witness | visible to authorized actor when report offered | missing field reasons | same for human-possessed Anna and AI Anna. |
| `create_or_update_institutional_record` | authorized recorder, usually Anna | ledger artifact accessible; writing materials | source speech/claim exists | institutional role and procedure | medium | rejected if no artifact/authorization/source; failure if interrupted | `institutional_record_opened/amended`, `memory_recorded` | ink/page access trace | visible only to authorized actor with source | record-as-claim explanation | same. |
| `question_or_watch_under_procedure` | Phase 4 authority/guard role | target/place accessible; procedure authorized | record/suspicion basis exists | institutional procedure required | medium/long | rejected if no basis/authority; failure if target absent | `speech_act_committed`, `observation_created`, `suspicion_updated`, `record_amended` | watch/question traces | Phase 0 sketch only; not Phase 1 feature | basis/procedure/debug | same once implemented; no player-only interrogation. |
| `continue_or_abandon_current_intention` | actor with current intention/routine | time and conditions change | actor has goal/routine/pressure/conflict | duties/social expectations | instant/short | cannot continue if impossible; abandonment may have consequence | `intention_plan_delayed/updated` | schedule deviation visible | visible as actor notebook/debug; embodied maybe “keep doing/stop” | intention cause chain | same; later cognition uses same event hooks. |

## 10. Event vocabulary

Events are primitive semantic families for Phase 0 contract purposes. They are not final serialized types. They must be sufficient for event-log authority, replay, projections, belief updates, debug explanation, and later tests.

Every event must have a stable event ID, timestamp/tick, responsible actor/process/institution, cause references, and replay-deterministic inputs. Events may be ground truth while projections and actor views remain filtered.

| Event family | Event type | Responsible actor / process / institution | Cause model | Referenced prior items | World mutation | Projection effects | Replay requirements | Visibility | Forbidden causes |
|---|---|---|---|---|---|---|---|---|---|
| Actor moved | `actor_moved` | moving actor or scheduler/routine | accepted `move_locally` action or routine continuation | prior location, route, door state, intention | actor location changes | map view, witness opportunities, routine state | deterministic route/time; seed if stochastic delay | ground-truth; actor-visible to observers with sight/sound; debug | player proximity, quest trigger, hidden director. |
| Door opened/closed | `door_opened`, `door_closed` | acting actor or environmental process if modeled | accepted open/close action | door ID, lock/state, actor access | door state changes | visibility/sound graph updates | deterministic state transition | ground-truth; visible/audible to eligible actors; debug | omniscient reveal, cutscene staging. |
| Container accessed/opened/closed | `container_accessed`, `container_opened`, `container_closed` | acting actor | accepted container action | container, actor, location, access adjudication | container state/access time changes | possible trace, visibility to contents | deterministic container state | ground-truth; visible/audible if observed; debug | “loot UI” without actor access. |
| Item transferred/relocated | `item_transferred`, `item_relocated` | actor/process moving item | take/place/hide action or seeded prehistory | source container/holder, destination, ownership/custody refs | item possession/location changes | inventory/container projections; expectation contradictions later | identity continuity and destination deterministic | ground-truth; actor-visible only if observed/remembered; debug | abstract balance delta, quest reward, director theft. |
| Item observed present | `item_observed_present` | observing actor/process | inspect/search/line-of-sight | item, location, sensory channel | no ground-truth mutation | actor belief candidates, memory | deterministic visibility; no hidden truth injection | actor-visible to observer; debug | seeing through closed containers/rooms. |
| Expected item observed absent | `expected_item_observed_absent` | observing actor | search/inspection plus prior expectation/source | expected item proposition, search event, container state | no direct mutation | contradiction belief, report affordance | must cite expectation and search scope | actor-visible to observer; debug | empty container as evidence without expectation. |
| Search performed | `search_performed` | searching actor | accepted search action | place/container, search intensity, access | may disturb traces/container state | observations, failure/success metrics | deterministic search parameters/seed if uncertain | ground-truth; maybe observed by others; debug | automatic clue discovery. |
| Trace created/observed/erased | `trace_created`, `trace_observed`, `trace_erased` | action actor, observer, or process | physical contact, movement, cleaning, weather if later | source event, surface/container, visibility | trace state changes or observation created | trace projection, belief source | deterministic source and lifespan | ground-truth for trace; actor-visible when observed; debug | culprit pointer, retroactive clue. |
| Observation created | `observation_created` | observer actor/process | sensory perception or search | perceived event/object/place, channel, constraints | no direct world mutation | actor memory/belief candidates | visibility model deterministic | actor-visible to observer; debug | omniscience, truth labels. |
| Belief formed/updated/contradicted/weakened | `belief_formed`, `belief_updated`, `belief_contradicted`, `belief_weakened` | actor cognition/projection | observation, speech, memory, inference, record | source belief/event/claim, proposition | actor belief store changes | embodied notebook, speech options, suspicion basis | deterministic update rules or explicit seed | actor-private; debug can show | global truth overwrite, stale info autocorrect. |
| Memory recorded/recalled | `memory_recorded`, `memory_recalled` | actor cognition | observation, event salience, report, routine | event/belief/proposition refs | actor memory store changes | later speech/reasoning | deterministic retention stage; seed if probabilistic later | actor-private; debug | perfect universal history for actors. |
| Speech act committed | `speech_act_committed` | speaker actor | ask/answer/tell/report/gossip/lie/refuse action | structured propositions, source beliefs, audience | social state maybe changes; no truth mutation | listener interpretations, heard-by observations | deterministic rendering from structured act | heard by listeners; debug shows sincerity only | freeform LLM state mutation. |
| Report received/refused/delayed | `report_received`, `report_refused`, `report_delayed` | institution receiver, usually Anna/Reeve | report action and intake adjudication | reporter, speech act, structured claims | report queue/status changes | institutional view, future record ability | deterministic procedure | institution-visible; reporter-visible; debug | authority reading truth. |
| Institutional record opened/amended | `institutional_record_opened`, `institutional_record_amended` | Anna/Reeve/institution process | authorized write from report/question/amendment | ledger artifact, writer, source speech/belief | ledger artifact state changes | record projection, access views | deterministic order, page refs | institution-visible to authorized; debug | record-as-truth. |
| Suspicion formed | `suspicion_formed` | actor or institution projection | motive/access/observation/hearsay/record inference | target, basis propositions, confidence | actor/institution suspicion state changes | speech/report/procedure affordances | deterministic basis scoring or explicit rule | actor-private/institution-visible if recorded; debug | hidden culprit marker. |
| Intention/plan delayed/updated | `intention_plan_delayed`, `intention_plan_updated` | actor cognition/routine scheduler | conflict, report, fear, interruption, routine | prior intention, cause event, new commitment | actor intention state changes | schedule/routine projections | deterministic planner state | actor-private; visible through behavior; debug | script forcing target outcome. |
| Action rejected | `action_rejected` | adjudicator/system | failed precondition before attempt | proposed action, missing requirement | no ordinary mutation except audit | why-not view, debug | deterministic adjudication | actor-visible as blocked affordance; debug | silent failure hiding doctrine. |
| Action failed | `action_failed` | acting actor/process | attempted action with failed execution | proposed action, risk/seed if stochastic | partial trace possible | failure memory, witnesses | seed/cause logged | actor-visible; ground-truth; debug | random unlogged failure. |
| No-human advance/run marker | `no_human_run_started`, `no_human_run_advanced`, `no_human_run_stopped` | scheduler/simulation process | explicit debug/test run command | seed, tick range, scheduler config | time advances; ordinary events happen | no-human metrics | seed and config logged | debug/test only; no diegetic visibility | player-conditioned schedule. |
| Replay/projection rebuilt | `replay_projection_rebuilt` | replay/projection process | load/rebuild command | event log slice, projection version | projection state rebuilt, not canonical events | debug and test comparison | deterministic from event log and version | debug/test only | correcting event log from projection. |

## 11. Proposition and claim vocabulary

Claims are source-bound. A claim is not truth. A proposition can be true, false, unknown, uncertain, stale, contradicted, or held by different actors with different confidence.

| Proposition family | Arguments | Possible holders | Possible sources | Contradiction patterns | Usable for speech/report/record/suspicion | TUI rendering implications |
|---|---|---|---|---|---|---|
| Item located in container/place | item ID, container/place ID, time or interval | any actor, institution record, debug projection | direct observation, prior placement memory, hearsay, record, inference | same item at incompatible location/time; later absence; source impossibility | speech, record, search target, suspicion if access/motive combine | embodied: “I think the coins are in the strongbox”; debug: truth/belief split. |
| Item missing | item ID, expected location, expected time, observer, search scope | expecting/searching actor, listener after report, institution record | expectation + search/absence observation, hearsay report | item observed present by same actor at same time; search scope invalid; expectation lacks source | report, record, suspicion, questioning | show source and uncertainty; no objective “stolen” label. |
| Actor possesses item | actor/container ID, item ID, time | observer, actor, institution, debug | direct sight, search recovery, confession/claim, inference from container | item location elsewhere; no line-of-sight; possession vs ownership confusion | suspicion/proof if sourced; report amendment | embodied: “Mara may have it” only if basis exists. |
| Actor owns item | actor/household, item, norm/source | owner, household, institution, neighbors | prior social knowledge, records, testimony | competing ownership claim; stolen property possession | report and record; not proof of current possession | distinguish “owned by Tomas” from “with Tomas.” |
| Actor may access container/location | actor, container/place, permission source, time | actor, household, institution, observers | role, invitation, routine, key custody, procedure | access expired; permission source denied; trespass observation | action adjudication, suspicion, record | TUI marks allowed/intrusive/unknown access. |
| Actor saw/heard something at place/time | actor, channel, content, place, time, confidence | witness, listener, record | observation memory, speech report | impossible visibility/sound; conflicting times | speech, report, record, suspicion basis | preserve sensory wording: “heard a door,” not “saw theft.” |
| Actor saw item present/absent | observer, item, location, time, search scope | observer, listener, record | direct observation or search event | item moved after observation; search incomplete; no expectation | report/record/evidence | absence rendering requires expected item and search scope. |
| Actor heard claim from source | hearer, speaker/source, claim ID, time, mode | any listener, institution record | speech act, rumor, record read | source denial, hearsay chain broken, contradicted content | gossip/report/record but with hearsay mode | show “Anna says Tomas reported…” not “coins stolen.” |
| Report was filed | report ID, reporter, receiver, institution, time, status | reporter, institution, readers | report event, ledger record | delayed/refused not filed; record missing/amended | institutional views, future procedure | embodied office view shows status, not truth. |
| Record says X | record ID, proposition, author, source claim | institution, authorized readers, later hearers | ledger read or report summary | amendment contradicts earlier entry; author error | speech/report/procedure | records displayed as text claims with source. |
| Actor may be responsible | target actor, incident, basis propositions, confidence | suspecting actor, institution | motive/access/opportunity/observation/hearsay | alibi, better explanation, source contradiction | suspicion, questioning, record if stated | “suspects” not “culprit”; debug can show wrongness. |
| Actor needs/intends something | actor, need/intention, source, time | actor, close confidant, observer/institution by inference | self memory, speech, observed routine, record | behavior contradiction, denial, stale need | motive analysis, speech, plan update | embodied must show uncertainty for others' minds. |
| Debt/payment/obligation unmet | debtor, creditor, amount/token/description, due time, source | debtor, creditor, hearers, records | debt note, speech, memory | payment record, denial, amount dispute | motive, report context, wrong suspicion | distinguish debt motive from theft proof. |
| Institution opened incident | institution, incident ID, reporter, status | institution, reporter, authorized readers | report received + record opened | report refused/delayed; record amended closed | procedure/status | no “case solved” quest state. |
| Suspicion candidate | incident, actor, basis set, confidence, holder | actor, institution | inference over claims/traces/records | basis disproven, alibi, source weakness | internal suspicion, speech if speakable | debug causal graph must show basis and confidence. |

## 12. Belief, observation, memory, and expectation contract

### 12.1 Belief shape

A Phase 0 belief contract must include at least:

- `belief_id`;
- `holder_id` actor or institution/projection;
- `proposition_family` and structured arguments;
- `stance`: believes, disbelieves, suspects, questions, denies, intends, expects;
- `confidence`: qualitative scale such as certain/high/medium/low/unknown, not a final numeric API;
- `source_refs`: observation, memory, speech act, record, inference, seed/prehistory event;
- `source_channel`: sight, sound, touch/search, self-memory, hearsay, record, inference;
- `acquisition_time`;
- `believed_event_time` or interval if known;
- `contradiction_links` to conflicting beliefs/propositions;
- `privacy` and `speakability`: private, household, public, institutional, confidential;
- `stale_status`: fresh, stale, contradicted, weakened, refreshed;
- `debug_notes`: non-diegetic explanation only.

A belief is not truth. A true proposition is not actor knowledge until acquired through modeled channels.

### 12.2 Observation shape

An observation must include:

- `observation_id`;
- `observer_id`;
- `channel`: sight, sound, touch, smell if later needed, search, record-read;
- `place_id` and actor location;
- `time` or interval;
- sensory object/event observed;
- visibility/sound constraints;
- confidence or ambiguity;
- whether it is direct, inferred, or hearsay-derived;
- generated candidate proposition IDs.

Observation and interpretation are separate. Petra may observe “a figure on the lane near dusk.” She does not thereby observe “Mara stole Tomas's coins.”

### 12.3 Expectation as belief/projection

Expectation is a belief about what should be true now or at a future time, grounded in source/provenance.

Required seeded expectation:

```text
belief_tomas_coin_expected_001
holder: actor_tomas
proposition: item located in container(coin_stack_01, strongbox_tomas, morning_d0)
stance: expects
confidence: high
source_refs:
  - obs_tomas_coin_check_dminus1
  - memory_tomas_closed_strongbox_dminus1
source_channel: prior direct observation + self-memory
speakability: private until Tomas chooses to report/tell
```

`obs_tomas_coin_check_dminus1` is a seeded prior direct observation/prehistory event. It is allowed because prehistory establishes starting facts and actor memories; it must not force future theft, discovery, report, suspicion, or resolution.

### 12.4 Absence-as-evidence

Absence is evidence only when an actor has an expectation or instruction that makes absence meaningful.

Valid chain:

```text
Tomas expects coin_stack_01 in strongbox_tomas
Tomas opens/searches strongbox_tomas
search scope includes where coin_stack_01 should be
coin_stack_01 is not observed there
expected_item_observed_absent event is created
Tomas's belief is contradicted or updated
Tomas may form item_missing claim
```

Invalid chain:

```text
Mara sees an empty strongbox with no prior expectation
=> Mara automatically knows Tomas's coins are missing   # forbidden
```

### 12.5 Memory depth staged for the first proof

Phase 0 specifies memory contracts but does not require full memory simulation in Phase 1.

- Phase 1 may store minimal event-derived observations and actor-current facts necessary for possession/view filtering.
- Phase 2 must make expectation, absence, belief contradiction, and possession parity executable.
- Phase 3 must support routine-driven no-human life and intention updates.
- Phase 4 must support institutional memory, records, suspicion, and wrong-suspicion cases.

Memory must be fallible/stale-capable by contract even if early executable phases use simple retention.

## 13. Speech and report contract

Speech is structured action. Rendered text is a surface. No freeform LLM speech appears in Spec 0001, and LLM text may not mutate game state.

Every speech act must include:

- speaker;
- listener/audience;
- structured propositions carried;
- source beliefs or source gaps;
- claim mode: asserted, hearsay, speculation, question, denial, lie, promise, command;
- confidence/uncertainty;
- listener interpretation;
- possible belief update;
- failure/refusal modes;
- deterministic rendering rule;
- debug-only sincerity/lie visibility.

| Speech act | Speaker / listener | Structured propositions carried | Source beliefs | Claim mode | Confidence / uncertainty | Listener interpretation | Possible belief update | Failure / refusal modes | Deterministic rendering | Debug-only sincerity / lie visibility |
|---|---|---|---|---|---|---|---|---|---|---|
| `ask` | any actor to reachable listener | question target proposition | information need, prior belief, routine | question | may carry uncertainty | listener receives request for claim | listener may recall/search/answer/refuse | listener absent, privacy, no topic basis | template from topic and relationship | no hidden truth shown. |
| `answer_sincerely` | respondent to asker | proposition(s) respondent believes | respondent belief/memory/record | asserted or hearsay | confidence mirrors belief | listener treats as sourced claim | forms/updates belief with speaker/source | respondent lacks memory or refuses | template by proposition family and confidence | debug may mark sincere. |
| `answer_uncertainly` | respondent to asker | weak/ambiguous proposition | stale memory/ambiguous observation | asserted with uncertainty | low/medium | listener receives weak evidence | belief formed weakly or not at all | over-demanding listener may reject | hedged template | debug shows uncertainty source. |
| `deny_knowledge` | respondent to asker | denial of knowledge about proposition, not denial of truth unless explicit | absence of belief, privacy, fear | denial | variable | listener may believe ignorance or suspect withholding | updates speaker-knows proposition, not world truth | contradicted if respondent has known source and chooses lie mode | “I do not know…” style template | debug distinguishes honest ignorance, privacy withholding, lie. |
| `lie_or_mislead` | actor with motive to listener | false assertion or selective omission | private belief conflicts or omission target | lie/mislead | chosen confidence may differ from private belief | listener may form false belief if trust/source enough | belief update with hidden contradiction | blocked if no modeled motive/knowledge gap | same surface as assertion; no “lie” label embodied | debug marks sincerity false and motive. |
| `report_missing_property` | reporter to Anna/Reeve/authorized receiver | item missing, expected location, ownership/custody, source expectation, search result | reporter belief chain | asserted report | confidence from expectation/search | receiver records report as claim | institution gains report/record belief, not truth | refused if no item/source/reporter/venue; delayed if office unavailable | formal intake template | debug shows fields accepted/rejected. |
| `repeat_rumor_or_testimony` | speaker to listener/audience | heard claim with source chain | prior speech act or record read | hearsay/speculation | confidence degraded unless source trusted | listener tracks source if contract works | hearsay belief, rumor propagation | source forgotten if modeled; listener refuses gossip | hearsay template naming source if known | debug shows chain degradation. |
| `institutional_intake_acknowledgment` | Anna/Reeve to reporter | report status, missing fields, next procedure | received report and institutional rules | acknowledgment/command/request | procedural certainty | reporter knows record/refusal/delay status | belief about institution opened/refused incident | incomplete fields, access/time constraints | office-formal template | debug shows no truth validation occurred. |
| `refuse_or_withhold` | any actor | refusal reason if speakable; no proposition if silent | privacy, fear, duty, uncertainty | refusal/withholding | n/a or low | listener may form belief about refusal | suspicion or trust change possible | procedure may later compel/override if Phase 4 rules | relationship/status template | debug shows hidden reason only. |

## 14. Institution, household, norm, and record contracts

Households are first-class domestic institutions. They are not decorative labels.

### 14.1 Household contracts

| Household ID | Members | Places / artifacts | Rights and duties | First-proof role |
|---|---|---|---|---|
| `household_tomas` | Tomas, Elena | `house_tomas`, `pantry_tomas_shelf`, `strongbox_tomas` | domestic privacy, food custody, private strongbox custody | Owns/claims expected property context. |
| `household_mara` | Mara, optionally Lena depending fixture | `house_mara`, `basket_mara`, `small_cache_mara` | domestic privacy, personal goods | Possible hiding location and privacy conflict. |
| `household_iva` | Iva | `house_iva`, `chest_iva` | debt records/goods, domestic privacy | Debt pressure source. |
| `household_nika` | Nika | `tavern_nika`, `tavern_till_nika` | commercial/customary privacy | Rumor/public-food context. |
| `household_rafi_lena` | Rafi, Lena | `house_rafi_lena` | domestic privacy, household support | Wrong-suspicion and wage-pressure context. |
| `household_petra` | Petra | `house_petra`, lane-facing window | domestic privacy; visual access from home | Partial witness context. |
| `household_soren` | Soren | `house_soren`, `satchel_soren` | work/personal goods | Ordinary route ambiguity. |

### 14.2 Local authority

`institution_reeves_office` is the only local authority in Spec 0001.

Roles:

- Anna: clerk; receives reports, writes/amends ledger, tracks source claims and uncertainty.
- Elias: guard; watches, asks, escorts, reports observations; cannot create proof from status bias.
- Reeve: local authority; reviews records, authorizes procedures, refuses unsupported sanction.
- Corin: optional assistant/runner; carries structured messages only if needed; not a quest courier.

### 14.3 Norms

| Norm ID | Type | Applies to | Violation | Detection | Suspicion | Report | Record | Proof | Sanction | Notes |
|---|---|---|---|---|---|---|---|---|---|---|
| `norm_property_theft` | regulative property norm | physical items with owner/custodian and access limits | actor intentionally takes/keeps item without permission or legitimate excuse | observation, missing expected property, trace, confession, recovery | actor-known basis combining motive/access/opportunity/evidence | property owner or witness may report | Anna records claims and sources | requires evidence chain, not debug truth | deferred; Phase 4 sketch only | Violation, detection, suspicion, report, record, proof, sanction, notice, and payment are distinct. |
| `norm_trespass_privacy` | domestic/institution privacy norm | private rooms, homes, restricted records, containers | actor enters/searches/opens without permission/procedure | witness, trace, admission, observation | access violation may support suspicion but not theft proof | household/institution may complain | record if reported | requires source-bound evidence | deferred; Phase 4 sketch only | Prevents search-as-universal-solution. |

### 14.4 Incident ledger record shape

Records are claims, not truth.

Required incident ledger record shape:

```text
record_id: stable record ID
physical_artifact_id: incident_ledger_01 / page or entry reference
institution_id: institution_reeves_office
author_writer_id: actor_anna or other authorized writer
reporter_id: actor_tomas or source actor
receiver_id: actor_anna / actor_reeve
created_time: tick/date
structured_claims:
  - item_missing(...)
  - item_located_expected(...)
  - actor_owns_item(...)
  - actor_saw_item_absent(...)
source_beliefs:
  - reporter belief IDs
source_speech_acts:
  - report speech act IDs
confidence_uncertainty: per claim
status: opened / incomplete / amended / under_review / refused / closed_unresolved / closed_proven_later
amendments:
  - amendment IDs, authors, source claims, contradiction links
readers:
  - authorized actor IDs and read events
access_permissions:
  - Anna/Reeve; Elias by procedure; public no direct read by default
lifecycle_events:
  - report_received
  - institutional_record_opened
  - institutional_record_amended
  - report_delayed/refused if relevant
```

A record can say “Tomas reports that his coins are missing from his strongbox.” It must not say “Mara stole Tomas's coins” unless an actor has made that sourced claim and the record preserves source/confidence/status.

## 15. Fixture suite and variants

The Phase 0 fixture suite defines reviewable contracts. Executable tests arrive at later phase gates. All fixtures below are materially useful and none are deferred.

### 15.1 Fixture summary

| Fixture ID | Purpose | Executable phase where required |
|---|---|---|
| `strongbox_001` | Physical container/item/custody baseline. | Phase 1 physical kernel; Phase 2 beliefs; Phase 4 institutions. |
| `expectation_contradiction_001` | Tomas's expected item absence contradicts prior belief. | Phase 2. |
| `possession_parity_001` | Human possession of different actors does not leak truth. | Phase 2. |
| `report_record_001` | Structured missing-property report creates record-as-claim. | Phase 4. |
| `wrong_suspicion_001` | Fallible suspicion can target wrong actor from partial evidence. | Phase 4. |
| `no_human_day_001` | No-human simulation can produce ordinary movement and possible missing-property chain. | Phase 3 baseline; Phase 4 full institutional variant. |
| `view_filtering_001` | Embodied views are actor-filtered; debug view is non-diegetic. | Phase 2. |
| `replay_rebuild_001` | Event log can rebuild world/projections deterministically. | Phase 1 physical; Phase 2/4 projections. |

### 15.2 Fixture contracts

#### `strongbox_001`

- **Purpose:** Establish `strongbox_tomas` as a physical private container containing physical `coin_stack_01`, with ownership/custody/access separated.
- **Setup:** Tomas's house, bedroom, strongbox, coins, Tomas prior memory, initial container state, access permissions.
- **Initial entities:** `actor_tomas`, `actor_elena`, `strongbox_tomas`, `coin_stack_01`, `house_tomas`, relevant doors.
- **Initial beliefs:** Tomas expects coins in strongbox; Elena may know strongbox usually contains money but not exact state unless variant grants source.
- **Allowed pressures:** Tomas wage/tax obligation; Elena household routine.
- **Forbidden scripts:** no forced theft, no forced discovery, no quest marker over strongbox.
- **Expected positive assertions:** item is physically in strongbox at start; Tomas may open/search; actor without access may be rejected or marked intrusive; ownership != possession.
- **Expected negative assertions:** actor outside room cannot see contents; no actor knows absence before search; coins are not abstract balance.
- **Relevant views:** embodied Tomas before discovery; debug container truth view.
- **Future test mapping:** unit tests for container/item state; content validation for owner/custodian/permitted users.
- **Executable phase:** Phase 1 for physical state and event log; Phase 2 for expectations.

#### `expectation_contradiction_001`

- **Purpose:** Prove absence-as-evidence from prior expectation plus search.
- **Setup:** Seed Tomas's prior observation; move/remove coins through modeled event or prehistory variant; Tomas searches strongbox.
- **Initial entities:** Tomas, strongbox, coins, search action, expectation belief.
- **Initial beliefs:** Tomas high-confidence expectation; others no automatic expectation.
- **Allowed pressures:** need to pay Rafi; instruction from Tomas to Elena to check in variant.
- **Forbidden scripts:** guaranteed search timing; automatic “stolen” label; empty container informing everyone.
- **Expected positive assertions:** `expected_item_observed_absent` references prior expectation and search scope; Tomas forms missing claim.
- **Expected negative assertions:** Mara/Anna do not know absence until told/observed; debug truth not injected.
- **Views:** Tomas after discovery, actor notebook, debug truth/belief mismatch.
- **Future test mapping:** belief update/golden view tests.
- **Executable phase:** Phase 2.

#### `possession_parity_001`

- **Purpose:** Prove that human-controlled actors and AI-controlled actors use identical action/belief filters.
- **Setup:** Run same event prefix while possessing Tomas, Mara, Anna, Elias, and no actor/debug mode.
- **Initial entities:** full village minimum.
- **Initial beliefs:** identical per actor across runs at same event prefix.
- **Allowed pressures:** ordinary routines and user-selected actions.
- **Forbidden scripts:** UI reveals truth because player controls a suspect/victim/clerk; possessed actor gets special verbs.
- **Expected positive assertions:** each possessed actor sees only their own observations/beliefs; debug view shows truth only when explicitly in debug.
- **Expected negative assertions:** possessing Mara after she hid coins reveals only Mara's actual memories/beliefs, not system labels like “culprit”; possessing Anna does not reveal report truth.
- **Views:** embodied views for Tomas/Mara/Anna; debug comparison.
- **Future test mapping:** leakage tests and TUI view-model tests.
- **Executable phase:** Phase 2.

#### `report_record_001`

- **Purpose:** Prove that a missing-property report becomes a structured institutional record without becoming truth.
- **Setup:** Tomas discovers absence and goes to `reeves_office`; Anna receives/refuses/delays depending fields; ledger entry may be opened.
- **Initial entities:** Tomas, Anna, office, incident ledger, report speech action.
- **Initial beliefs:** Tomas missing claim; Anna no direct truth, only receiver role.
- **Allowed pressures:** office hours, Anna workload, Tomas embarrassment.
- **Forbidden scripts:** record auto-fills culprit; report always accepted; report creates reward/objective.
- **Expected positive assertions:** report includes reporter, item, expected location, source belief, search, confidence; record references speech act and beliefs.
- **Expected negative assertions:** ledger does not assert theft proved; Anna does not know truth unless separately sourced.
- **Views:** Anna after report, incident ledger view, why-not blocked report.
- **Future test mapping:** golden record tests, institutional projection tests.
- **Executable phase:** Phase 4.

#### `wrong_suspicion_001`

- **Purpose:** Prove fallible suspicion from partial evidence without truth leakage.
- **Setup:** Seed motive/access-like observations for Rafi or Mara; actual mover may be different or no theft variant; Elias/Reeve/Anna form suspicion from basis.
- **Initial entities:** Elias, Anna, Reeve, suspects, observation/speech/record sources.
- **Initial beliefs:** partial claims only; no actor has complete truth unless they directly caused/observed it.
- **Allowed pressures:** debt, wages, line-of-sight ambiguity, status bias.
- **Forbidden scripts:** hidden culprit pointer, guaranteed accusation, institution solving case automatically.
- **Expected positive assertions:** suspicion event names basis propositions and confidence; later contradiction can weaken suspicion.
- **Expected negative assertions:** suspicion is not proof; no sanction without procedure/evidence.
- **Views:** debug causal graph, why-not blocked accusation, actor notebook.
- **Future test mapping:** suspicion basis tests and leakage tests.
- **Executable phase:** Phase 4.

#### `no_human_day_001`

- **Purpose:** Prove the village can advance without human possession and still produce ordinary life, observations, report opportunities, and possibly the missing-property chain.
- **Setup:** no possessed actor; scheduler runs household/work routines and allowed pressures under seed.
- **Initial entities:** full minimum village.
- **Initial beliefs:** per actor seed memories and roles.
- **Allowed pressures:** debt, wages, food, work, office hours, errands.
- **Forbidden scripts:** events conditioned on player arrival; drama director stealing coins; guaranteed report/accusation.
- **Expected positive assertions:** ordinary movement occurs; if missing-property chain occurs, it uses same actions/events as embodied play.
- **Expected negative assertions:** no player objective/completion state; no special no-human shortcuts.
- **Views:** debug timeline, no-human run markers.
- **Future test mapping:** no-human golden tests, replay tests, metrics.
- **Executable phase:** Phase 3 baseline; Phase 4 full report/suspicion variant.

#### `view_filtering_001`

- **Purpose:** Prove embodied and debug views are separated.
- **Setup:** Same event prefix rendered as Tomas, Mara, Anna, Elias, and debug.
- **Initial entities:** actors, beliefs, event log, projections.
- **Initial beliefs:** actor-specific; debug has ground-truth/projection access.
- **Allowed pressures:** none beyond scenario state.
- **Forbidden scripts:** actor view displays event log truth, culprit labels, or hidden claims.
- **Expected positive assertions:** embodied view preserves uncertainty; debug explicitly marks non-diegetic truth/belief mismatch.
- **Expected negative assertions:** no actor sees records they cannot access; no stale belief auto-corrects.
- **Views:** all required view sketches.
- **Future test mapping:** TUI/view-model golden tests and leakage tests.
- **Executable phase:** Phase 2.

#### `replay_rebuild_001`

- **Purpose:** Prove event log can rebuild physical and projection state deterministically.
- **Setup:** Run fixed seed and action prefix; save event log; rebuild physical world, actor projections, institutional projections when available.
- **Initial entities:** strongbox, coins, doors, actors, seed/run markers.
- **Initial beliefs:** seeded expectations and memories.
- **Allowed pressures:** deterministic scheduler choices under logged seeds.
- **Forbidden scripts:** projection writes correcting canonical log; unstored random choices; branch-dependent hidden state.
- **Expected positive assertions:** rebuilt world matches original projections for phase-supported state; debug can explain from event chain.
- **Expected negative assertions:** final UI state is not source of truth; no unlogged randomness.
- **Views:** replay debug, causal graph excerpt.
- **Future test mapping:** replay/projection tests, property tests.
- **Executable phase:** Phase 1 for physical projection; Phase 2+ for belief projections; Phase 4 for institutional projections.

## 16. Embodied and debug view-model sketches

Embodied views are actor-filtered and uncertainty-preserving. Debug views are visibly non-diegetic.

### 16.1 Embodied view as Tomas before discovery

```text
You are Tomas.
Place: house_tomas / tomas_main_room.
Intentions: open mill soon; pay Rafi after checking household money.
Known nearby: Elena may be in the pantry. The bedroom door is closed.
Private memory: You counted coin_stack_01 in strongbox_tomas last evening and closed the box.
Expected: coin_stack_01 should be in strongbox_tomas.
Visible affordances: go to bedroom; ask Elena; wait; go to mill.
Not shown: whether the coins are actually still there; who will move them; any culprit label.
```

### 16.2 Embodied view as Mara after moving/taking coins

```text
You are Mara.
Place: house_mara / main_room.
Current private state: You remember handling coin_stack_01 if and only if this fixture branch actually had you do so.
Carried/nearby: shown only if the coins are in your possession or visible in your container.
Pressures: Iva's debt demand; fear of Elias; loyalty/fear involving Lena.
Beliefs: Tomas may notice if he checks the strongbox. You do not know whether he has checked yet unless you observed/heard it.
Visible affordances: hide/place carried item; speak to Lena; go to square; wait; deny/withhold if asked.
Not shown: Anna's private record; Tomas's current belief unless learned through speech/observation; “culprit” label.
```

### 16.3 Embodied view as Tomas after discovering absence

```text
You are Tomas.
Place: house_tomas / bedroom_tomas.
Observed: strongbox_tomas is open. coin_stack_01 is not where you expected it.
Contradicted expectation: You remember seeing the coins here last evening.
Current belief: The coins are missing from the strongbox. You do not know who moved them.
Possible next actions: search bedroom; ask Elena; report to Anna; go to mill; wait.
Blocked accusation note: You cannot accuse Mara yet without actor-known basis. You may say you are worried or ask if anyone saw something.
```

### 16.4 Embodied view as Anna after report

```text
You are Anna.
Place: reeves_office / office_counter.
Incoming claim: Tomas reports coin_stack_01 missing from strongbox_tomas.
Source supplied: Tomas says he counted it last evening and searched the strongbox this morning.
Institutional status: report can be accepted as a claim if required fields are complete.
Your knowledge: You do not know whether the coins were stolen. You know Tomas made a structured report.
Visible affordances: ask follow-up; receive report; refuse incomplete report; open/amend incident ledger; tell Elias a sourced summary.
Not shown: actual coin location unless separately observed or recorded through authorized process.
```

### 16.5 Debug view of truth/belief mismatch

```text
[DEBUG / NON-DIEGETIC]
Truth projection:
  coin_stack_01 location: small_cache_mara  (from evt_item_relocated_014)
  strongbox_tomas: empty, closed
Actor beliefs:
  Tomas: expects coin_stack_01 in strongbox_tomas until evt_expected_absent_022; then believes missing, culprit unknown
  Mara: knows/does-not-know based on branch; if mover, remembers relocation event
  Anna: knows only report claim after evt_report_received_031
  Elias: suspects Rafi from basis {wage_motive, seen_near_mill}, confidence low/medium
Records:
  incident_ledger_01/entry_001: Tomas reports missing coins; status opened; no proof
Warnings:
  No embodied actor may read this panel.
```

### 16.6 Actor notebook

```text
Notebook: Tomas
Memories:
  - Last evening: counted coin_stack_01 in strongbox_tomas. Source: direct observation.
  - This morning: searched strongbox_tomas; did not find coin_stack_01. Source: direct search.
Beliefs:
  - coin_stack_01 missing from strongbox_tomas. Confidence: high.
  - unknown actor may have moved it. Confidence: low/speculative.
Speakable claims:
  - report missing property to Anna.
  - ask Elena whether she moved or saw the coins.
Not speakable as fact:
  - “Mara stole it.” No actor-known basis yet.
```

### 16.7 Why-not explanation for blocked accusation

```text
Action blocked: accuse actor_mara of theft.
Actor: Tomas.
Missing requirements:
  - no direct observation of Mara taking coin_stack_01;
  - no sourced claim that Mara possessed coin_stack_01;
  - no record linking Mara to strongbox_tomas;
  - debt/motive claim not currently known by Tomas in this branch;
  - strongbox absence proves absence from expected place, not culprit.
Allowed alternatives:
  - report missing property;
  - ask Elena;
  - search nearby;
  - state worry/speculation if reckless accusation mode is explicitly modeled later.
```

### 16.8 Why-not explanation for blocked report

```text
Action blocked: report missing property.
Actor: Tomas.
Reason variant A: Tomas has not searched the expected container yet.
Reason variant B: office receiver unavailable; report may be delayed, not filed.
Reason variant C: actor is reporting hearsay but cannot name source or item.
Needed fields:
  - reporter;
  - item description or ID known to actor;
  - expected location/source;
  - observation/search basis for absence;
  - confidence/uncertainty.
```

### 16.9 Debug causal graph excerpt

```text
[DEBUG / NON-DIEGETIC]
seed_memory_001: Tomas saw coin_stack_01 in strongbox_tomas last evening
  -> belief_tomas_coin_expected_001

evt_item_relocated_014: actor_mara moved coin_stack_01 from strongbox_tomas to small_cache_mara
  -> truth coin location changed
  -> trace_created strongbox_dust_disturbed

evt_search_021: Tomas searched strongbox_tomas
  + belief_tomas_coin_expected_001
  -> evt_expected_absent_022
  -> belief_tomas_coin_missing_023
  -> report_missing_property affordance visible to Tomas

evt_speech_report_030: Tomas reports missing property to Anna
  -> evt_report_received_031
  -> record_incident_001 opened as claim, not truth

evt_suspicion_041: Elias suspects Rafi
  basis: wage_motive_claim + route_observation
  contradiction: no possession evidence
```

## 17. No-scripting review

Spec 0001 rejects the following shortcuts. A fixture, future implementation, or future spec that uses one of these has drifted from the first proof.

- Forced theft.
- Guaranteed discovery.
- Guaranteed accusation.
- Guaranteed institutional closure.
- Hidden culprit pointer.
- Player-conditioned events.
- Quest flags.
- Reward/completion state.
- Objective markers.
- Direct fixture mutation as ordinary play.
- Institutions reading truth.
- LLM text mutating state.
- Abstract balance replacing physical value token.
- Hidden drama director.
- Stale information auto-correcting.
- Actor view reading debug state.
- Records treated as proof without source/evidence.
- Suspicion treated as proof.
- Motive treated as action evidence.
- Possession treated as ownership.
- Access treated as permission.
- Private search treated as harmless default behavior.

Review question: could the same chain occur, or fail to occur, with no human attached? If not, it is probably scripted.

## 18. Validation and future test mapping

This section maps Phase 0 contracts to later test categories. It is not a ticket list, owner list, estimate, or implementation sequence beyond phase gates.

| Future test category | Contract material from Spec 0001 | Examples of later assertions | Phase gate |
|---|---|---|---|
| Unit tests | Primitive action and event families; container/item state; access adjudication | opening a closed door changes visibility; taking item moves physical possession; rejected action records why-not | Phase 1+. |
| Property tests | Event invariants, item identity, replay determinism, no unowned hidden truth claims | an item cannot be in two physical containers at same tick; actor belief cannot appear without source | Phase 1+ physical; Phase 2+ epistemic. |
| Golden tests | Fixture chains and rendered view sketches | `strongbox_001` expected events; Tomas after discovery view; Anna report record | Phase 1+ physical, Phase 2/4 projected. |
| TUI/view-model tests | Embodied/debug view separation and actor filtering | Tomas sees absence after search; Anna sees report claim only; debug marks truth/belief mismatch | Phase 2+. |
| Replay/projection tests | Event vocabulary and `replay_rebuild_001` | rebuilt physical state equals original; projection rebuild does not mutate event log | Phase 1+. |
| No-human tests | `no_human_day_001`; routine/action parity | ordinary movement occurs without player; missing-property chain uses same action/event families | Phase 3 baseline; Phase 4 full. |
| Content validation checks | stable IDs; required actor/place/container/item fields | every actor has role, relationships, access, beliefs; every container has owner/custodian/expected contents | Phase 0 review; Phase 1+ tooling. |
| Leakage tests | possession parity; actor-filtered claims/beliefs | possessing Anna does not reveal coin location; possessing Mara does not expose debug culprit label | Phase 2+. |

## 19. Phase 1 entry contract

After Spec 0001 is accepted, Phase 1 may start with the following approved inputs:

- approved primitive action vocabulary, with debug possession switching separated from ordinary play;
- approved primitive event vocabulary for physical state, replay, and future projections;
- approved proposition/claim vocabulary;
- first village minimal fixture contract: actors, places, rooms, doors, containers, `coin_stack_01`;
- stable ID policy: IDs are content-stable semantic identifiers, not display names and not final storage keys;
- embodied and debug TUI view-model contracts, including actor filtering and non-diegetic debug separation;
- golden scenario sketches for strongbox, expectation contradiction, possession parity, report/record, wrong suspicion, no-human, view filtering, and replay rebuild;
- no-scripting review checklist;
- data validation requirements for minimal entities: every actor/place/door/container/item must have required fields from this spec before being accepted into the first fixture.

Phase 1 still must not implement or pretend to complete:

- full epistemics;
- full memory decay/misremembering;
- full routines and planning;
- full institutions or sanctions;
- road travel or route threat;
- notices as product feature;
- bounties/payments as quest loop;
- companions;
- combat;
- LLM speech surfaces or live LLM state mutation;
- graphical client;
- procedural town generation.

Phase 1 is allowed to build a minimal kernel/TUI/event-log/replay slice that can host the Phase 0 fixtures without violating them.

## 20. Retread-prevention

Spec 0001 is the stable reference for Phase 0 ontology and fixtures. Future specs should cite it rather than re-litigating:

- first-proof identity;
- actor roster baseline;
- core place/container/item contracts;
- ownership/custody/access/control/proof/belief distinctions;
- action/event/proposition vocabularies;
- fixture names and purposes;
- embodied/debug view-model boundaries;
- forbidden shortcuts;
- Phase 1 entry requirements.

Future specs may refine implementation shape, validation tooling, phase-specific executable tests, schemas, or UI layout only within their phase authority. They should not reopen notice boards, road threats, bounties, companions, quest objectives, LLM dialogue, or drama-director shortcuts as if Phase 0 had not settled the first proof.
