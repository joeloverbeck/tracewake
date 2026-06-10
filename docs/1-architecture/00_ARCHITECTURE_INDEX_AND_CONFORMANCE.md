# Architecture Index and Conformance

## Status

Authoritative architecture contract. This directory is a complete replacement set. Architecture files not listed in this document are retired and must not remain as live doctrine.

The foundation layer outranks this layer. Architecture may make foundation doctrine more operational; it may not soften it.

## Purpose / core rule

Every subsystem must preserve the universal Tracewake authority split:

```text
holder-known context -> proposal / cognition / procedure decision
authoritative truth -> validation / resolution / physical mutation / debug comparison
modeled feedback -> future holder knowledge
```

For ordinary actors, cognition must pass through the actor decision transaction. For institutions, procedure must pass through an institution-known transaction. For LOD and regional processes, aggregate truth may simulate summaries, but promoted holders receive only modeled knowledge with ancestry.

## Replacement map

| Live architecture document | Primary contract |
|---|---|
| `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Architecture map, authority order, replacement rule, and universal conformance questions |
| `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` | Rust-first authority boundaries, dependency direction, content/domain-pack authority, and forbidden inversions |
| `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | Event sourcing, replay, projections, save manifests, random streams, event migration, and diagnostics as replay material |
| `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | Holder-known context sealing, provenance, truth firewall, debug quarantine, and contamination review |
| `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | Shared action pipeline, proposals, validation truth, scheduler limits, actor-legible feedback, and failure semantics |
| `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Actor decision transaction, needs, commitments, intentions, routines, HTN methods, local planning, and stuck diagnostics |
| `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | Typed claims, observation, belief, memory, records, rumors, traces, and information flow |
| `07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` | Structured speech acts, speaker/listener interpretation contexts, language surfaces, and optional LLM boundary |
| `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | Institution-known procedure transaction, households, records, norms, roles, proof, sanctions, and fallibility |
| `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | Missing-property first proof, ordinary life, settlement spatial model, property, work, sleep, food, and local economy |
| `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Possession, TUI-first embodied play, actor-legible why-not, debug-only truth, view models, and client boundaries |
| `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | Questless incidents, leads, notices, reports, observer-only story sifting, and second-proof deferral |
| `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` | LOD as ontology, summary processes, promotion/demotion, regional ancestry, prehistory, and scale |
| `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Acceptance gates, diagnostic artifacts, anti-contamination tests, replay proof, and review checklists |
| `14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` | Research distilled into decisions, consequences, and forbidden misreads |

Retired architecture names, including the old state/content authoring document, the old Phase 3A actor-known autonomy document, and the old research-notes document, are absorbed into this replacement map. Do not keep them as parallel live contracts.

## Authority owned

The architecture layer owns subsystem contracts, canonical data-flow boundaries, diagnostic obligations, and acceptance implications. It may name implementation seams as examples only. It does not prescribe tickets, sprint order, task decomposition, branch names, or code patches.

## Authority denied

The architecture layer may not:

- change the project identity set by foundation;
- weaken ordinary actor parity or TUI-first playability;
- bless hidden-truth planning because current code happens to work that way;
- create player-only world verbs;
- treat display strings as proof;
- convert research precedent into a mandate to copy another game;
- defer the missing-property ordinary-life proof behind adventure systems.

## Universal conformance questions

Every feature, subsystem, fixture, test, view model, and report must answer these questions:

1. **Who is the holder?** Actor, institution, household, speaker, listener, embodied viewer, process, or LOD promotion recipient.
2. **What did that holder know at the decision boundary?** List typed claims, observations, memories, records, roles, procedure state, resources, and stale/uncertain information.
3. **Where did each input come from?** Every cognition/procedure input needs provenance: observation, memory, testimony, record, public artifact, schedule, role assignment, modeled rumor, summary event, or explicit “unknown.”
4. **What was sealed before decision?** The holder-known context must be constructed before candidate generation, method selection, local planning, speech interpretation, affordance menu selection, or institutional procedure choice.
5. **What did authoritative truth do?** Truth may validate, reject, resolve, mutate authoritative state, generate modeled observations, and support non-diegetic debug comparison.
6. **What truth was forbidden?** Truth must not select goals, plans, routine steps, speech meaning, view-model affordances, institutional procedure outcomes, leads, story beats, or LOD promotion knowledge.
7. **What feedback became future knowledge?** Rejections, observations, notices, reports, sanctions, and memories become future holder knowledge only through modeled channels.
8. **What replayable evidence proves this?** Event log, context packet, provenance graph, decision trace, stuck diagnostic, validation report, TUI transcript, and debug artifact.

A feature that cannot answer these questions is not architecture-conformant.

## Phase 3A evented cognition conformance

| Surface | Required allowed source | Required evidence |
|---|---|---|
| Workplace knowledge | Event-sourced notice/observation/record with `source_event_ids`; never a raw workplace table read, routine-template label, or fixture assignment copied into cognition. | Actor-known workplace facts cite live log events; negative fixtures prove raw assignment without notice does not plan work. |
| Sleep-surface knowledge | Event-sourced notice/observation/record with `source_event_ids`; current-place sleep affordance truth alone is not an information channel. | Actor-known sleep facts cite live log events; sleep proposals carry modeled affordance ids through the shared action pipeline. |
| Food-source knowledge | Event-sourced notice/observation/record with `source_event_ids`; nearest-food or hidden-container truth cannot select an eat plan. | Actor-known food facts cite live log events; hidden-food fixtures prove inaccessible truth is absent from planning inputs. |
| Route knowledge | Event-sourced notice/observation/record with `source_event_ids`; unseen adjacency/path truth cannot choose embodied exits or plans. | Actor-known route facts cite live log events; hidden-route fixtures prove unseen edges remain unavailable. |
| Hidden-truth audit enforcement | Decision traces are not decorative; forbidden source classes fail closed before proposal construction, with typed stuck diagnostics and pipeline defense in depth. | `hidden_truth_input` diagnostics carry responsible layer, blocker code, and replayable ancestry; agent-origin proposals with dirty audits are rejected. |
| Completion continuity | Scheduled duration completions for sleep/work preserve action ancestry, body exclusivity, interruption checks, and prorated effects instead of becoming scheduler-authored outcomes. | Completion/interruption events are caused by the start event and replay byte-identically. |
| Ordinary-life tuning boundary | Passive need rates and sleep duration/recovery are content-authored flavor; need-band thresholds remain kernel semantics. | Content schema requires authored `need_model` and sleep tuning fields; guard tests ban the retired kernel tuning constants. |
| need-tick accounting authority | Passive need deltas are classified from authoritative actor state and tick regimes, not from proposal-supplied need values or fixture assumptions. | `classify_actor_tick_regimes` / `classify_actor_tick_regimes_with_start` drive awake/sleep/work deltas; validator guards reject `current_hunger` / `current_fatigue` proposal parameters as authority. |
| duration terminal-set | Body-exclusive duration completion/interruption semantics are shared through the event kind terminal set, not duplicated in scheduler or action definitions. | `is_duration_terminal` is used by scheduler and pipeline continuity checks; sleep/work completion tests prove terminal closure, reservation closure, and replay. |
| context-hash re-derivation gate | A holder-known context hash carried by a proposal is evidence only when it is recomputed from the current sealed context/frontier. | `source_context_check` compares proposal source fields to the current `KnowledgeContext` and `holder_known_context_hash`; replay rebuilds recompute hashes through `compute_holder_known_context_hash`. |
| cognition-substrate source | No-human cognition and embodied views derive actor-known facts from the durable epistemic projection, not raw authoritative tables or builder-local event-payload scans. | `NoHumanActorKnownSurfaceBuilder::from_projection` consumes `EpistemicProjection::actor_known_records_for_context`; source guards ban raw payload scans in the no-human surface. Closed by `archive/tickets/0016PHA3ANEEACC-015.md`. |
| Embodied workplace availability source | Human embodied affordance menus use the holder-known context, not the raw workplace assignment table, when exposing work options. | `actor_known_workplaces_for_context(context: &KnowledgeContext)` backs `phase3a_semantic_actions`; guard tests ban `actor_known_workplaces_for_context(state)` and raw `state.workplaces` access in that surface. |
| lock-layer workspace census | Anti-contamination gates must prove their own perimeter: every production source, negative fixture directory, and clippy ban is registered and justified. | `WORKSPACE_SOURCE_CLASSIFICATIONS`, `registered_negative_fixtures_match_directory_census`, and `clippy_ban_entries_have_proving_negative_fixtures` fail closed; `reports/0016_ord_hard_025_mutants_baseline.md` records mutation counts and miss dispositions. |
| 0017 tick-charge attribution authority | Every tick-delta need charge, including action-emitted awake deltas, is reconciled by counted `(actor, need, tick)` occurrences rather than regime-label sets. | `assert_single_tick_delta_charge`, `AgentState::need_tick_charges`, `assert_no_duplicate_need_regime_charges`, `assert_single_need_charge_per_actor_tick_kind`, and `wait_then_window_passive_charges_each_tick_once_001` lock the one-charge-per-tick rule. |
| 0017 shared open-duration authority | Open body-exclusive duration state uses event-cause start ids across scheduler, pipeline, and need-accounting consumers; duplicate or orphan terminals are not silently reconciled. | `open_body_exclusive_starts`, `body_exclusive_reservation_conflict`, `terminal_ticks_by_start`, `duplicate_duration_terminal_violations`, and `duplicate_duration_terminal_poisons_rebuild_001` prove shared keying and fail-closed duplicate terminal handling. |
| 0017 projection freshness rule | The epistemic projection exposes one freshness classification for no-human cognition and embodied availability: current observations remain `observed_now`, older usable knowledge becomes remembered belief with original source timing. | `EpistemicProjection::actor_known_records_for_context`, `ActorKnownProjectionFreshness`, `ClassifiedActorKnownProjectionRecord::freshness`, `aged_food_record_surfaces_as_remembered_belief_not_observation_001`, and `current_place_knowledge_context_uses_latest_projection_window_not_live_truth` lock the rule. |
| 0017 provenance-class audit | Provenance citations must witness the asserted fact kind; merely citing an existing event is insufficient. | `dangling_provenance_diagnostic`, `provenance_witness_mismatch`, and `guard_018_actor_known_facts_require_source_event_witness` fail closed for empty, dangling, or wrong-kind witnesses. |
| 0017 replay payload evidence | Agent event kinds that carry semantic payloads must be replay-visible through materialized state or checksum coverage; pure markers are the only no-op exception. | `AGENT_WORLD_NOOP_ALLOWLIST`, `AGENT_STATE_CHECKSUM_COVERAGE`, `no_human_capstone_proves_typed_ancestry_and_replay`, and tamper tests for continue-routine/candidate-goal payloads lock replay evidence. |
| 0017 believed-access embodied availability | Embodied Phase 3A affordances are enabled from actor-known believed access and known resources, with debug truth separated and commit-time validation still authoritative. | `ActorKnownWorkplaceFact::believed_access_open`, `phase3a_semantic_actions`, `embodied_workplace_availability_reflects_belief_not_truth_001`, and `embodied_workplace_believed_open_truth_closed_commit_fails_001` prove belief/truth divergence. |
| 0017 content policy registry | Content-authored numeric fields and scanned strings are classified by explicit policy; loaders reject out-of-band values instead of silently clamping or partially scanning. | `NUMERIC_FIELD_REGISTRY`, `SCANNED_STRING_FIELDS`, `schema_numeric_fields_are_classified_for_validation_policy`, and `schema_string_fields_are_classified_for_script_scanning` lock registry parity. |
| 0017 lock-layer durability | The lock perimeter includes typed source-event witnesses, workspace-wide cognition-input truth-reader bans, whitespace-normalized guard checks, and a pinned mutation baseline. | `SourceEventIds` storage on `ActorKnownFact`, `cognition_inputs_are_context_backed`, CI `mutants-in-diff`, and `.cargo/mutants-baseline-misses.txt` record the durable lock surfaces. |
| 0017 generative lock tier | Bounded generated scheduled wait/no-human sequences exercise replay, prefix replay, checksum, payload, and single-charge metamorphic locks; duration terminal reachability remains covered by targeted fixtures and capstone checks. Kani adoption is skipped without prejudice for this pass. | `generated_sequences_replay_and_satisfy_metamorphic_locks` and `support/generative.rs` record the seed set and reachability counters for the first in-tree generator tier. |

### Audit-history overturns

The 0016 audit overturned two 0015 verified-holding/outcome claims: the claim that passive deltas do not double-count sleep ticks and the claim that context-hash rebuild was locked. They are recorded as `ORD-HARD-014` and `ORD-HARD-016` findings respectively; future audits must cite the overturn rather than silently rewriting the 0015 audit history.

Spec 0017 overturned three 0016 acceptance-evidence claims in `ORD-HARD-034`:
the synthetic prior-version event upcast fixture was not delivered and the
schema contract is loud-rejection-only; the workplace belief/truth divergence
fixture was a placeholder until 0017 delivered the real divergence pair; and
the named content negative fixtures were delivered as inline `validate.rs`
tests and registry census tests. Future audits must cite the 0017 correction
section in `reports/0016_ord_life_cert_scoped_acceptance.md` rather than
silently inheriting the older report text.

## Required architectural spine

For ordinary actors:

```text
trigger
 -> sealed actor-known context with provenance
 -> live needs / commitments / intentions / routines / obligations
 -> candidate generation from actor-known pressures only
 -> intention continuation/adoption/interruption/drop
 -> HTN/routine/procedure method selection
 -> bounded local planning from actor-known context
 -> ordinary action proposal with ancestry
 -> shared action pipeline validation against authoritative truth
 -> event commit or rejection/failure semantics
 -> actor-legible feedback and modeled observations
 -> belief/projection updates
 -> typed decision/stuck diagnostics
 -> replay/debug/TUI projections
```

For institutions:

```text
institution trigger
 -> sealed institution-known context from records, reports, role knowledge, resources,
    jurisdiction, procedure state, public artifacts, and institutional memory
 -> procedure candidate generation / role decision
 -> proposal or institutional action through shared validation/event pipeline
 -> records/notices/orders/sanctions/failures only through modeled authority and resources
 -> public/actor/institution knowledge updates only through modeled channels
 -> debug-visible truth comparison without leakage
```

For LOD and regional processes:

```text
summary truth may maintain aggregate simulation
but promoted actors/institutions receive only knowledge with modeled ancestry
```

## Anti-patterns

Reject any implementation that:

- sends a need threshold directly into a primitive action;
- sends a routine label directly into movement, work, sleep, eat, report, sanction, or travel;
- reads a workplace table, household table, fixture truth, or route graph to choose an actor's plan without holder-known provenance;
- uses validation truth to suggest a better action;
- treats “the actor would know” as enough without a typed source;
- returns actor-facing why-not text that reveals hidden truth;
- uses debug truth in embodied affordance menus;
- creates quest states, pacing managers, clue spawners, reward spawners, drama directors, or authored outcome chains;
- requires an LLM for acceptance, replay, planning, or authoritative truth.

## Cross-document obligations

- The event/replay contract in document 02 must record enough evidence to replay decisions and diagnose contamination.
- The holder-known contract in document 03 is the firewall used by documents 04 through 12.
- The action pipeline in document 04 is the only route to world mutation for ordinary actions.
- The actor transaction in document 05 is the only route from ordinary actor pressure to action proposal.
- Institution procedures in document 08 must be as epistemically bounded as actor cognition.
- TUI behavior in document 10 must expose actor-legible views and debug-only truth as separate products.
- Acceptance in document 13 must fail plausible behavior if provenance or replay evidence is missing.
