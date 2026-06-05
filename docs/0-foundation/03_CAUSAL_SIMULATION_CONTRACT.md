# Causal Simulation Contract

## Core claim

Tracewake is not a current-state toy. It is a forensic causal machine.

The authoritative simulation must be able to explain how significant state came to be, who or what caused it, what traces it left, who observed it, who believes what, who is wrong, what records exist, what was forgotten, what was erased, and which later events became possible because of it.

Event sourcing belongs in the foundation layer because Tracewake’s identity depends on causality being inspectable rather than merely implied.

## Authority model

The authoritative world is not prose, UI text, a case board, an LLM answer, a quest flag, or a current-state database row.

The authoritative world is:

```text
a deterministic causal simulation
+ meaningful events
+ materialized state derived from events
+ explicit beliefs and records derived from modeled channels
+ replay/debug machinery capable of explaining significant ancestry
```

Materialized views are allowed. They are disposable accelerators, not truth replacements.

## Meaningful changes require events

A change requires an event if it can affect any of the following:

- physical location, containment, custody, ownership, access, body state, or object state;
- needs, fatigue, hunger, fear, intent, routine, or planning;
- belief, memory, confidence, suspicion, contradiction, or inference;
- testimony, rumor, gossip, report, promise, threat, instruction, or lie;
- records, notices, ledgers, case files, receipts, orders, contracts, warrants, or household accounts;
- norms, obligations, permissions, prohibitions, sanctions, roles, jurisdiction, or institutional facts;
- money, food, storage, work, wages, promises of payment, debt, or custody;
- travel, route state, weather, environment, regional pressure, or LOD tier;
- traces, trace removal, trace interpretation, or forensic inspection;
- TUI leads, actor-known notebook entries, debug explanation, replay, or future reasoning.

A state change that matters but has no event is a bug.

## Event categories

Tracewake events may be:

- physical: movement, opening, taking, placing, hiding, eating, sleeping, locking, cleaning, breaking;
- mental: perception, memory formation, forgetting, confidence decay, inference, contradiction recognition, decision, intention adoption, intention abandonment;
- social: speech act, testimony, rumor, gossip, accusation, refusal, promise, lie, command, threat, apology;
- institutional: report intake, case opening, filing, misfiling, notice posting, refusal, sanction, audit, payment approval, warrant creation;
- economic: purchase, wage payment, debt creation, stock change, theft, loss, custody transfer, promised payment;
- environmental: rain, mud, darkness, crop condition, fire, spoilage, road degradation;
- regional: tax order, caravan arrival, migration wave, disease pressure, road closure, outside authority instruction;
- procedural: action validation failure, search failure, access refusal, office closed, route not found, plan timeout;
- LOD: promotion, demotion, summary interval, summary import, detail restoration;
- erasure: trace cleaning, record destruction, false correction, object relocation, witness intimidation, memory decay.

The system must not privilege physical events as the only “real” events. Belief and record changes are often the most important events in Tracewake.

## Event anatomy

A significant event should preserve, at foundation level, the following kind of information:

- stable event identity;
- event type and domain;
- simulation time and ordering information;
- participants: actors, objects, places, institutions, households, regional processes;
- initiating cause: intention, routine, need pressure, belief, procedure, environment, regional source, or explicit boundary input;
- preconditions considered and whether they were satisfied;
- resources, costs, risks, permissions, prohibitions, and access facts relevant to the attempt;
- result: success, partial success, failure, refusal, interruption, timeout, misclassification, or no-op with consequences;
- direct state changes;
- traces created, altered, removed, or left absent;
- observations made possible;
- belief propositions created, revised, contradicted, weakened, or forgotten;
- records created, revised, lost, contradicted, or made stale;
- random draw references when randomness affected meaningful outcome;
- causal links to antecedent events or summary events;
- LOD tier at execution and any fidelity limits;
- replay/debug notes sufficient for explanation without relying on prose as authority.

Not every low-level micro-change needs human-readable ceremony, but significant consequences must preserve significant ancestry.

## Cause models

Every event needs a cause model. Acceptable causes include:

- actor intention;
- need pressure;
- value, relationship, duty, fear, debt, ambition, habit, or shame;
- subjective belief;
- routine or HTN method;
- bounded local plan;
- physical affordance;
- environmental process;
- institutional procedure;
- household procedure;
- norm activation or violation;
- regional process;
- authored initial condition;
- declared exogenous boundary input;
- prior event or summary event.

Forbidden causes include:

- the player is bored;
- the story needs escalation;
- the mystery needs a clue;
- a quest was accepted;
- an objective needs completion;
- an LLM generated dramatic text;
- the player entered a region;
- the UI needed a reward;
- hidden director logic.

## Failure is eventful

Failure is not absence from the log. Failure is often the point.

Examples of eventful failure:

- an actor tries to open a locked chest and fails;
- a report is refused because office hours ended;
- a clerk misclassifies a theft as a household dispute;
- a witness cannot identify a person confidently;
- a search fails because the actor checked the wrong container;
- a plan fails because hunger interrupts travel;
- a payment fails because the office lacks funds;
- a lie fails because a listener has contradictory evidence;
- an accusation fails because the accuser has no actor-known basis;
- a routine fails because the worker overslept.

If a failure can shape belief, memory, future planning, suspicion, reputation, records, or institutional action, it must be eventful.

## Absence-as-evidence

Absence becomes evidence only through expectation, observation, instruction, or intentional search.

The simulation may contain the fact that coins are absent from a strongbox. Tomas does not discover “coins missing” merely because the global state changed. Tomas discovers it if he expected coins to be there, checked the box, searched the room, was told to look, or otherwise perceived a contradiction.

Absence evidence must record:

- who expected what;
- where the expectation came from;
- what was checked;
- what was absent;
- how confident the actor is;
- whether alternate explanations remain plausible;
- what later beliefs, searches, speech acts, or reports became possible.

## Traces

Events leave traces. Traces are not automatically evidence. They are world state that may be perceived, ignored, misunderstood, erased, recorded, or connected to beliefs later.

Trace types include:

- object displacement;
- open/closed/locked state;
- missing property;
- footprints, mud, scratches, smell, blood, ash, broken latch, disturbed dust;
- witnessed presence or absence;
- remembered sound;
- overheard speech;
- changed route usage;
- changed stock, money, food, or custody;
- record entries, erasures, contradictions, stamps, signatures;
- rumor variants;
- body state: hunger, injury, exhaustion, intoxication, fear;
- institutional state: open case, lost report, unpaid promise, stale notice;
- environmental state: rain washed tracks, darkness hid identity, spoilage.

Trace removal is itself eventful. Cleaning a footprint, moving stolen goods, burning a notice, altering a ledger, intimidating a witness, or forgetting an observation does not make causality disappear. It creates a new causal branch.

## Deterministic replay

Deterministic replay is foundation doctrine.

Replay must be able to reconstruct significant state from:

- initial seed and domain configuration;
- authored initial state or scenario seed;
- ordered event stream;
- random draw records where meaningful;
- ruleset/version metadata needed for compatibility;
- snapshots that preserve ancestry rather than replacing it.

Replay is required for:

- debugging causality;
- inspecting wrong beliefs;
- verifying no player privilege;
- reproducing no-human simulations;
- regression testing the first miracle scenario;
- explaining LOD summary ancestry;
- auditing institutions and records;
- checking that actor-knowledge filters are not leaking truth.

## Randomness

Randomness is allowed when it models uncertainty, variation, or domain processes. It is forbidden when used as a hidden drama director.

Meaningful random draws must be seedable, replayable, and inspectable in debug. Examples:

- whether a sleepy witness wakes from a noise;
- which ambiguous identity a witness infers;
- whether rain washes away weak tracks;
- whether a clerk misfiles a report under pressure;
- whether a low-LOD summary event promotes a regional rumor into local simulation.

The debug inspector should be able to show what distribution or random model was used and why that model was in scope. The foundation does not require final RNG architecture, but it forbids untracked authoritative randomness.

## Snapshots and compaction

Snapshots, indexes, caches, projections, read models, and compaction are allowed. They are necessary for performance and long simulations.

They may not erase:

- active causal chains;
- significant event ancestry;
- belief provenance;
- institutional record provenance;
- trace history relevant to future reasoning;
- LOD promotion ancestry;
- random draw auditability for meaningful outcomes;
- contradiction links;
- source-bound leads;
- no-human acceptance traces.

If a snapshot cannot explain why a suspicious notice exists, who issued it, what it claimed, why it may be stale, and what event chain created it, the snapshot has destroyed too much.

## Boundary inputs

Exogenous events are allowed only as declared causal inputs.

A regional storm, tax order, caravan arrival, plague pressure, migration wave, road closure, institutional order, or bandit movement may enter local simulation if it has:

- declared source;
- cadence or trigger;
- input state;
- random model if any;
- domain authority;
- scope;
- delivery channel;
- traces;
- affected beliefs and records;
- causal ancestry;
- replay/debug visibility.

A regional process must never be a disguised drama director.

## Acceptance implications

Any meaningful feature must pass these checks:

```text
What events represent it?
What caused those events?
What traces did they leave?
Who observed them?
Who formed beliefs from them?
Who could be wrong?
What records or institutions touched them?
What failure events exist?
Can replay reconstruct the significant chain?
Can debug inspection explain the chain without relying on prose authority?
```

If the answer is unclear, the feature is not foundation-compliant.
