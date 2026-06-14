# Causal Event, Trace, and Replay Contract

## Core claim

Tracewake is a forensic causal machine, not a current-state toy.

The authoritative simulation must be able to explain how significant state came to be, who or what caused it, what traces it left, who observed it, who believed what, who was wrong, what records exist, what was forgotten, what was erased, and which later events became possible.

Event sourcing belongs in the foundation layer because Tracewake's identity depends on inspectable causality.

## Authority model

The authoritative world is:

```text
deterministic simulation
+ meaningful event stream
+ causal graph / ancestry links
+ materialized state derived from events
+ typed claims, beliefs, traces, and records derived through modeled channels
+ replay/debug machinery capable of explaining significant outcomes
```

The authoritative world is not prose, a quest flag, a UI card, an LLM response, a current mutable save row, or an invisible director.

Materialized projections are allowed. They are rebuildable accelerators, not replacements for event history.

## Meaningful changes require events

A change requires an event if it can affect:

- physical location, containment, custody, ownership, access, body state, object state, or route state;
- needs, hunger, fatigue, fear, safety, intention, routine, project, or planning;
- belief, memory, confidence, suspicion, contradiction, inference, expectation, or forgetting;
- observation, search, testimony, rumor, gossip, report, lie, promise, instruction, refusal, threat, or command;
- records, notices, ledgers, case files, receipts, contracts, debts, warrants, orders, letters, or household accounts;
- norms, roles, obligations, permissions, prohibitions, powers, sanctions, jurisdiction, or institutional facts;
- money, wages, food, stock, service, debt, custody, payment promise, or ownership/access claims;
- travel, weather, environment, regional pressure, LOD tier, boundary import, or long-history summary;
- traces, trace erasure, trace interpretation, or forensic inspection;
- TUI actor-known leads, notebooks, debug explanation, replay, story sifting, or future reasoning.

A meaningful mutation with no event is a bug.

## Event categories

Tracewake events include, at foundation level:

- physical events: move, open, close, lock, unlock, take, place, hide, eat, sleep, damage, clean, repair;
- action attempt events: proposal, validation failure, refusal, interruption, timeout, partial success;
- mental events: perception, observation, memory formation, forgetting, confidence decay, inference, contradiction recognition, intention adoption, intention abandonment;
- social events: speech act, testimony, gossip, rumor mutation, accusation, denial, promise, lie, command, threat, apology, refusal;
- institutional events: report intake, report refusal, case opening, filing, misfiling, notice posting, record copying, payment approval/refusal, sanction, audit, warrant creation;
- economic events: purchase, wage payment, debt creation, stock change, food service, theft, loss, custody transfer, promised payment;
- environmental events: rain, mud, darkness, crop condition, fire, spoilage, road degradation, disease sign;
- regional events: tax order, caravan arrival, migration wave, animal movement, disease pressure, road closure, outside authority instruction, war pressure;
- LOD events: promotion, demotion, summary interval, summary import, detail restoration;
- erasure events: trace cleaning, record destruction, false correction, object relocation, witness intimidation, memory decay, ledger alteration.

Physical events are not more real than belief or record events. In Tracewake, a false report may matter more than a footprint.

## Event anatomy

A significant event should preserve enough structure to answer later questions. At foundation level, an event should carry or link to:

```text
stable event identity
event type/domain/schema version
simulation time and deterministic ordering
participants: actors, objects, places, institutions, households, processes
initiating cause: intention, need, belief, routine, procedure, environment, regional source, boundary input
preconditions considered and validation result
resources, costs, risks, permissions, prohibitions, access facts, and proof requirements considered
result: success, partial success, failure, refusal, interruption, timeout, misclassification, or consequential no-op
direct state changes
traces created, altered, removed, or left absent
observations made possible
claims/beliefs created, revised, contradicted, weakened, or forgotten
records created, copied, read, revised, lost, contradicted, or made stale
random draw references when randomness mattered
causal links to antecedent events or summary events
LOD tier and fidelity limitations
ruleset/content/data version context
replay/debug notes sufficient for explanation without prose authority
```

Not every low-level calculation needs a ceremony. Every significant consequence needs surviving ancestry.

## Cause models

Acceptable causes include:

- actor intention;
- need pressure;
- value, duty, fear, debt, loyalty, shame, ambition, habit, relationship, or role;
- subjective belief, memory, inference, expectation, or contradiction;
- physical affordance;
- routine or HTN method;
- bounded local plan;
- household procedure;
- institutional procedure;
- norm activation, permission, prohibition, power, sanction, or proof rule;
- environmental process;
- regional process;
- LOD summary ancestry;
- authored initial condition;
- authored or generated prehistory with structured provenance;
- declared exogenous boundary input;
- prior event or summary event.

Forbidden causes include:

```text
player accepted quest
player is bored
story needs escalation
mystery needs clue
objective needs completion
LLM produced dramatic text
player entered region
UI needs reward
hidden director chose beat
world needed pacing
```

## Traces

A trace is any durable or semi-durable consequence that can influence later perception, belief, record, procedure, memory, or debug explanation.

Trace categories include:

- physical: missing item, moved object, open door, disturbed dust, footprint, mud, scratch, smell, blood, ash, broken latch, hidden stash;
- perceptual: witnessed presence, heard sound, glimpsed clothing, noticed absence, perceived closed office;
- mental: fear, suspicion, guilt, shame, expectation contradiction, remembered sound;
- social: gossip variant, insult, promise, accusation, reputation shift, silence, refusal;
- institutional: report, ledger entry, notice, case file, receipt, warrant, office queue, stamp, signature, erasure mark;
- economic: missing stock, unpaid debt, wage promise, price pressure, shortage, transfer, lost payment;
- environmental: rain-washed tracks, mud, darkness, spoiled food, blocked road, smoke, disease sign;
- regional: arriving courier, caravan rumor, tax order, refugee report, route warning;
- erasure: cleaned blood, burned notice, forged ledger page, repaired lock, intimidated witness, moved stolen item.

Traces are not automatically evidence. They are world state that may be perceived, ignored, misread, erased, recorded, or later connected to claims.

## Trace removal

Trace removal is a causal branch, not deletion of causality.

Cleaning a footprint, moving hidden goods, burning a notice, altering a ledger, intimidating a witness, bribing a clerk, repairing a latch, or forgetting an observation may reduce available evidence. It must itself leave event history and, when meaningful, traces of erasure.

A perfect cleanup is not a silent cleanup. It is a successful sequence of trace-removal events with surviving ancestry visible in debug/replay.

## Failure and refusal

Failure is not absence from the log. Failure is often the most important event.

Examples:

- an actor tries to open a locked chest and fails;
- a search fails because the actor checked the wrong container or searched too shallowly;
- a report is refused because the clerk is absent, office hours ended, or jurisdiction is unclear;
- a speech act fails because the listener refuses attention;
- a lie fails because the listener holds contradictory claims;
- travel fails because weather blocks the route;
- an accusation fails because the actor lacks actor-known basis;
- a payment fails because the promised funds are unavailable;
- a routine fails because the worker overslept, lost a tool, became hungry, or believed the shop was closed.

If failure can shape belief, memory, future planning, suspicion, reputation, record state, institutional action, or later opportunity, it must be eventful.

## Absence-as-evidence

Absence becomes evidence only through expectation, perception, instruction, or intentional search.

The world may contain an empty strongbox. Tomas does not discover missing coins until he expected coins there and checked, searched, was instructed, or was told by someone who did.

Absence evidence must preserve:

```text
holder of expectation
source of expectation
place/object checked
search/perception method
absent thing or absent condition
confidence
alternative explanations
belief update / claim generated
later actions made possible
```

A missing item should support multiple claims until evidence narrows them: moved, stolen, borrowed, misremembered, paid out, searched poorly, hidden, destroyed, or never present.

## Event-derived claims, beliefs, and records

Beliefs and records may be materialized as projections, but important changes need event-level provenance.

A report chain is not:

```text
record says theft_complete = true
```

It is:

```text
Tomas observed absence
-> Tomas formed ItemMissing claim
-> Tomas reported claim to Anna
-> Anna perceived report
-> Anna accepted/refused/delayed intake
-> Anna created/failed to create record containing claims
-> later actors read, cite, doubt, distort, or ignore the record
```

Every arrow is eventful when it matters.

## Deterministic replay

Replay must be able to reconstruct significant state from:

- initial seed and domain configuration;
- authored initial state, scenario seed, or generated prehistory summary;
- ordered event stream;
- meaningful random draw records or reproducible random stream positions;
- ruleset/content/data/schema versions;
- snapshots that preserve ancestry rather than replacing it;
- migrations/upcasters where old event versions are supported.

Replay is required for debugging causality, inspecting wrong beliefs, verifying no player privilege, reproducing no-human simulations, accepting first miracle scenarios, auditing institutions, rebuilding projections, checking actor filters, and explaining LOD summary ancestry.

## Temporal authority

Simulation time is authoritative for event order, replay, validation, intervals, scheduled consequences, and causal explanation. It is not automatically authoritative for cognition. Tracewake distinguishes:

- authoritative event/replay time: the ordered substrate used to validate actions, reconstruct history, apply due consequences, and explain causality;
- holder-known temporal claims: what an actor, household, institution, group, or region believes, remembers, reads, hears, infers, or expects about when something happened or should happen;
- institution-known procedural time: office windows, filing windows, due states, queue aging, notice lifecycle, payment periods, case delay, and sanctions as record-backed or procedure-backed states;
- routine and social rhythm: work, sleep, meals, patrols, appointments, and market patterns as defeasible premises known through assignment, memory, observation, public cues, or institutional context;
- freshness and staleness authority: temporal risk attached to claims, memories, records, leads, and notices, not automatic truth correction;
- LOD and regional temporal summaries: interval and cadence summaries that preserve both temporal ancestry and information ancestry for later promotion.

The temporal firewall is the truth firewall applied to time: the truth clock may validate; holder-known or institution-known time may plan. A scheduler trigger may open a decision window, but proposal generation, routine continuation, institutional conclusion, embodied rendering, speech interpretation, lead projection, and LOD promotion must consume temporal facts only when those facts have reached the relevant holder through modeled channels.

Foundation deliberately does not choose a tick size, calendar/date syntax, duration unit, scheduler queue structure, UI clock display, exact stale-after value, or first-playable calendar vocabulary. Those choices belong below foundation.

## Randomness

Randomness is allowed for uncertainty, variation, and domain processes. It is forbidden as hidden drama direction.

Meaningful random draws must be seedable and auditable. Examples include:

- whether a sleepy witness wakes from a noise;
- which ambiguous identity a witness infers;
- whether rain washes away weak tracks;
- whether a clerk under pressure misfiles a report;
- whether a low-LOD rumor enters local high-detail simulation.

Debug should show the random stream, purpose, relevant distribution/model, draw result or commitment according to build policy, consuming event, and why that draw was in scope.

## Snapshots, projections, and compaction

Snapshots, read models, projections, indexes, caches, and compaction are required for performance and long simulation.

They may not erase:

- significant event ancestry;
- active causal chains;
- belief and memory provenance;
- institutional record provenance;
- trace creation/removal history relevant to future reasoning;
- random draw auditability for meaningful outcomes;
- contradiction links;
- source-bound leads;
- active obligations, debts, promises, reports, and cases;
- LOD promotion/demotion ancestry;
- no-human acceptance traces.

Compaction should produce summary events, not hidden edits. A summary event must still declare source interval, inputs, outputs, affected entities, affected claims/records/traces, random draw references, fidelity limits, and replay/debug visibility.

## Schema evolution and replay failure

Events are immutable. Corrections are new events, not edits.

Event schemas and projections must have version discipline. Old event streams may require upcasters or migrations. Replay should reject unsupported history in authoritative mode rather than guessing.

Replay failure is a diagnostic event/report, not a silent repair. Failures include unsupported event type, missing data version, projection checksum mismatch, nondeterministic ordering, random stream mismatch, missing causal reference, or invariant violation.

## Boundary inputs

Exogenous events are allowed only as declared causal inputs.

A regional storm, caravan arrival, tax order, plague pressure, animal migration, refugee movement, crop failure, road closure, outside institutional order, or war pressure may enter local simulation only with:

```text
source
cadence or trigger
input state
random model if any
domain authority/scope
delivery channel
local entry event
traces
affected actors/places/institutions/households/records/claims
causal ancestry
LOD tier/fidelity limits
replay/debug visibility
```

A boundary process must never be a disguised storyteller.

## Acceptance checks

Any meaningful feature must pass:

```text
What events represent it?
What caused those events?
What typed claims, beliefs, traces, or records did they create/change?
Who observed them?
Who could be wrong?
What failures or refusals exist?
What ancestry survives snapshots/LOD/compaction?
What random draws mattered?
Can no-human simulation produce it?
Can replay reconstruct the chain?
Can debug explain it without prose authority?
```

If the chain cannot be explained, the feature is not foundation-compliant.

## 2026 hardening: cognition and provenance are replay obligations

Event sourcing in Tracewake is not only for physical state. It is also the audit substrate for cognition.

A replay that reconstructs item locations while losing why an actor proposed an action is not sufficient. A foundation-conformant replay must preserve, rebuild, or fail loudly for the causal ancestry of:

- observations and their perception channels;
- belief updates, stale beliefs, contradictions, and memory changes;
- action proposals and their actor-known inputs;
- selected intentions, routine/method choices, local plans, rejected candidates, and stuck diagnostics;
- validation results and actor-visible versus debug-only why-not information;
- speech acts, reports, notices, records, rumors, testimony, lies, promises, and institutional decisions;
- LOD summaries and promoted-region ancestry.

## Decision trace contract

A decision trace is not flavor text. It is replay-relevant diagnostic state.

A conforming decision trace must be able to answer:

```text
Which actor decided?
What triggered the decision?
What did the actor know, believe, perceive, remember, expect, intend, need, and owe?
Which candidate goals or actions were considered?
Which were rejected and why?
Which intention or routine continued, changed, failed, or completed?
Which proposal entered the action pipeline?
Which authoritative validation checks succeeded or failed?
What did the actor learn from the result, if anything?
What remained debug-only?
```

The exact schema may change, but the answers must not be reconstructed by parsing human display strings.

## Provenance-carrying events

Events that create or change knowledge-bearing state must carry sufficient provenance. This includes observation source, speaker/listener, record author, artifact identity, claim source, confidence or uncertainty where modeled, event time, acquisition time, and links to enabling causes.

A belief update without holder and source is invalid. A record without issuer/author/source is invalid. A notice without origin and lifecycle is invalid. A summary event without ancestry is invalid for any later promoted behavior that depends on it.

## Truth firewall in replay

Replay must prove not merely that final state matches, but that hidden truth did not contaminate proposal generation. At minimum, replay/debug projections must support hidden-truth audits for high-salience or acceptance-gate decisions:

- actor-known inputs contain no debug-only or validator-only facts;
- validator truth used for success/failure does not appear in later cognition unless a modeled observation, report, record, or memory update exposed it;
- embodied view-model derivation uses actor-known context;
- debug-only comparisons stay non-diegetic.

## LOD summaries

LOD compaction and regional summaries may compress low-salience activity, but they must not erase the distinction between what happened and who knew it. Summary events must preserve causal ancestry, information ancestry, and promotion obligations sufficient to prevent promoted actors or institutions from awakening with unexplained truth.
