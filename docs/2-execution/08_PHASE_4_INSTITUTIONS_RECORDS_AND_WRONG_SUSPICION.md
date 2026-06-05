# Phase 4: Institutions, Records, and Wrong Suspicion

## Purpose

Phase 4 completes the first proof.

It adds minimal households as domestic institutions, one fallible local authority, report intake, incident records, property/privacy norms, simple questioning/watch procedure, and wrong suspicion from partial information.

This phase proves that institutions are social machines, not omniscient scripts.

```text
violation may occur
 -> detection may or may not occur
 -> suspicion may form
 -> report may be made
 -> record may be opened
 -> procedure may succeed or fail
 -> suspicion may attach to the wrong actor
 -> sanction may not occur
```

Violation, detection, suspicion, record, proof, and sanction remain distinct.

## Entry requirements

Phase 4 may start only when Phase 3 exits.

Required:

- ordinary daily simulation works without human input;
- sleep/eat/work/wait/routines are TUI-playable;
- belief provenance and actor-known filtering work;
- possession parity works;
- structured speech basics exist or can be added in this phase;
- missing-property setup can occur inside routine life;
- event log/replay/projection rebuild are stable enough for institutional events.

## Deliverables

### Household as minimal institution

Households must be first-class enough to support:

- members;
- domestic roles;
- shared/private storage;
- beds;
- food access;
- privacy boundaries;
- guest permissions;
- household property expectations;
- household memory/secrets if needed;
- expected home/work presence.

Required first households:

```text
household_tomas
household_mara
2-5 supporting households
```

Household access affects actions. A house is not a decorative container.

### Local authority

Implement one small local authority.

Required pieces:

```yaml
Institution:
  id: reeves_office
  kind: local_authority
  roles: [reeve, clerk, guard]
  places: [reeves_office, village_core]
  records: [incident_ledger]
  norms: [property_theft, trespass_privacy]
  resources:
    staff_time: limited
    paper_or_ledger_space: finite_enough_to_model_failure_if_cheap
    guards_available: 0_to_2
  office_hours:
    open: morning_to_afternoon
    closed: night
```

No broad legal system. No court. No bounty proof/payment in first proof.

### Roles

Required role behavior:

Anna, clerk:
: can receive reports, ask basic questions, write incident ledger entries, read office records, refuse/delay outside procedure.

Reeve:
: can authorize simple follow-up, prioritize, question, order guard, ignore or delay.

Elias, guard:
: can receive limited instructions, question, watch, escort, refuse, delay, or misinterpret.

Roles confer powers and obligations. Role actors can fail, be absent, be tired, be biased, or be wrong.

### Report action

Report is a structured speech act through the action pipeline.

Report checks:

- speaker can communicate;
- listener exists/reachable/present or explicit exception applies;
- listener has report-receiving role or accepted informal channel;
- speaker has reportable actor-known belief;
- office/procedure availability;
- time cost;
- listener attention;
- social willingness.

Report can fail:

- office closed;
- clerk absent;
- clerk refuses attention;
- reporter lacks clear claim;
- social fear prevents report;
- language/channel failure if modeled;
- interruption;
- report delayed;
- listener disbelieves or deprioritizes.

### Incident ledger

Records are artifacts.

A ledger entry must include:

- record ID;
- physical artifact or ledger page;
- institution;
- author/writer;
- receiver;
- reporter;
- received time;
- structured claims;
- source beliefs/speech acts;
- confidence/uncertainty;
- status;
- amendments/contradictions;
- readers;
- access permissions.

Bad:

```text
case exists because theft happened
```

Good:

```text
case exists because Tomas reported missing coins to Anna,
Anna heard the report, interpreted it, and wrote a ledger entry.
```

### Norms

Implement at least:

`property_theft`
: prohibition on taking recognized property without permission and without exception.

`trespass_privacy`
: privacy/access rule for homes, private rooms, and private containers.

Norms must distinguish:

```text
violation
detection
suspicion
report
record
proof
sanction
```

The norm engine may classify a possible violation for debug/procedure. That classification is not automatically known by actors.

### Suspicion scoring

Suspicion is from actor/institutional knowledge, not truth.

Inputs may include:

- access opportunity;
- partial observation;
- hearsay;
- motive belief;
- prior relationship/conflict;
- trace interpretation;
- absence from routine;
- record wording;
- trust/credibility;
- bias;
- rumor mutation;
- refusal/lie if detected as such by actor-known basis.

Suspicion output:

```yaml
Suspicion:
  holder: actor_elias | institution_reeves_office | actor_tomas
  suspect: actor_rafi
  proposition: ActorMayHaveTakenItem(actor_rafi, coin_stack_01)
  confidence: 0.46
  sources:
    - report_record_missing_coins_01
    - belief_rafi_near_house_evening
    - rumor_rafi_debt
  threshold_result:
    question: met
    search_private_space: not_met
    sanction: not_met
```

Suspicion may be wrong. Wrong suspicion is a deliverable.

### Simple questioning/watch procedure

Implement minimal procedure:

```text
record opened
 -> clerk/reeve assesses claims
 -> suspicion candidates generated
 -> threshold/resources/office hours checked
 -> guard or reeve may question/watch
 -> speech act or observation occurs
 -> beliefs/records may update
 -> procedure remains open, delayed, contradicted, or stale
```

Questioning does not reveal truth. It creates speech, refusals, lies, contradictions, and new beliefs.

### Institution debug inspector

Debug must show:

- institution knowledge vs ground truth;
- records and their claims;
- report provenance;
- procedure state;
- role/staff availability;
- evidence thresholds;
- suspicion scores;
- why wrong suspicion attached;
- why correct culprit was not known;
- which hidden truth is excluded from institutional reasoning.

## Explicit non-goals

Phase 4 does not include:

- public bounty proof/payment;
- companion recruitment;
- stale-lead travel;
- full notice lifecycle as product proof;
- courts/trials;
- prison system;
- detailed patrol AI;
- full reputation economy;
- regional law;
- corruption beyond simple causal bias if cheap;
- live LLM dialogue.

## TUI/view-model gate

A player can:

1. discover missing property as Tomas or another actor with expectation;
2. ask/tell/report using structured speech;
3. travel to Anna if reachable;
4. report missing property when current actor knows enough;
5. see why-not if report is blocked;
6. watch Anna receive the report;
7. watch a ledger entry be created;
8. read accessible records as actor-known artifacts;
9. watch Elias/Reeve form suspicion from partial institutional knowledge;
10. see wrong suspicion arise;
11. inspect debug institution state and provenance.

Embodied record views show claims as claims. They do not annotate truth.

## No-human simulation gate

No-human Phase 4 must prove:

- missing property can be discovered without player input under fixture conditions;
- report may be made if actor goals and routines support it;
- office availability affects report;
- record may be created from partial claims;
- wrong suspicion may arise;
- no event references player identity;
- debug attach afterward can inspect what happened.

The no-human version may use deterministic fixture pressure. It may not use forced outcome scripts.

## Deterministic replay gate

Replay must rebuild:

- report speech event;
- listener interpretation;
- ledger entry;
- institutional belief projection;
- suspicion scores;
- procedure state;
- record readers;
- actor-known record views;
- debug institution inspector state.

Replay must fail loudly on missing record schema, missing content version, unknown event type, or lost belief source.

## Test gate

### Unit tests

- report action validation;
- listener availability;
- office hours;
- speech act proposition/source validation;
- ledger entry creation;
- record provenance;
- norm classification;
- suspicion input weighting;
- institution knowledge filtering;
- role permission checks.

### Property tests

- no record lacks author/provenance/artifact;
- no institutional belief lacks source;
- no institution reads ground truth;
- violation does not imply detection;
- detection does not imply proof;
- proof does not imply sanction unless procedure supports it;
- records are claims, not truth;
- player possession does not change institution state.

### Golden tests

- `report_record_001`;
- `wrong_suspicion_001`;
- `office_closed_001`;
- `report_refused_001`;
- `record_claims_not_truth_001`;
- `institution_non_omniscience_001`;
- `first_proof_full_chain_001`.

### TUI/view-model tests

- report action visible only with actor-known basis;
- actor can ask/report through structured speech UI;
- record view preserves uncertainty;
- debug institution inspector shows hidden truth separately;
- embodied view never labels true culprit unless current actor knows through modeled channel.

## Data/fixture gate

Data must define:

- households;
- local authority;
- roles;
- office hours;
- incident ledger schema;
- report speech-act templates;
- property/theft norm;
- trespass/privacy norm;
- suspicion rules;
- questioning/watch procedure;
- record access;
- failure modes.

Forbidden fields:

```text
scripted_report_acceptance
scripted_accusation
true_culprit_for_institution
case_reads_ground_truth
global_wanted_meter
reward_spawn_on_completion
```

## Debug/inspection gate

Canonical debug questions:

```text
Why did Anna create this ledger entry?
Why does Elias suspect Rafi?
Why does Elias not know Mara took the coins?
Why could Tomas report missing property?
Why could Tomas not truthfully accuse Mara?
Which record claim lost uncertainty?
Which actor/institution is wrong?
Which later events did the report enable?
```

Debug answers must cite event/belief/record/procedure sources.

## Failure cases to model

Required:

- office closed;
- Anna absent;
- Elias unavailable;
- report too vague;
- report refused;
- report delayed;
- record written with partial uncertainty;
- record misread;
- suspicion threshold not met;
- suspicion threshold met for wrong actor;
- clerk/guard trusts wrong source;
- household privacy blocks search;
- witness refuses;
- actor lies;
- institution attempts ground-truth read and test fails.

## Metrics

Track:

- reports attempted/received/refused/delayed;
- records created/amended/contradicted;
- institutional beliefs by source;
- suspicion scores;
- wrong suspicions;
- questioning actions;
- office closed blockers;
- procedure failures;
- belief leakage failures;
- replay/projection rebuild failures;
- TUI report/record action coverage.

## First proof final acceptance

The first proof is accepted only when the system passes this full chain:

```text
Tomas expects coins in strongbox
Mara takes or moves coins through ordinary action pipeline
Elena may receive partial uncertain observation
Tomas later discovers absence through expectation contradiction
Tomas cannot know Mara's action without information channel
Tomas may ask, search, report, conceal, or suspect from actor-known basis
Anna receives report only through reachable/available/procedural channel
Anna creates a record with claims/provenance
Elias/Reeve may form wrong suspicion from partial institutional knowledge
possession switching does not transfer knowledge
debug explains truth, belief, trace, record, suspicion, and replay
no-human version can run
```

## Forbidden shortcuts

- report accepted without listener;
- case opened because item is truly stolen;
- guard knows culprit from event log;
- suspicion score uses true culprit;
- institution proof threshold reads ground truth;
- record has no physical/artifact provenance;
- UI action "solve case";
- bounty/payment sneaks into first proof;
- LLM decides report credibility;
- debug view contaminates embodied record.

## Phase exit checklist

Phase 4 exits when:

- households affect access and expectations;
- local authority acts from records and procedure;
- report/record/wrong suspicion work through TUI;
- no-human version can produce/inspect the chain;
- replay rebuilds institutional state;
- tests cover success and failure;
- debug explains wrong suspicion;
- no player/quest/LLM/director shortcuts exist;
- first proof definition of done passes.

Only then begin the second proof.
