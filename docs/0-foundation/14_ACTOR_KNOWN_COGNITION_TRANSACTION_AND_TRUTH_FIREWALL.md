# Actor-Known Cognition Transaction and Truth Firewall

## Status

This is a constitutional foundation document for **Tracewake**.

It defines the hard boundary between authoritative truth and actor cognition. It sits above architecture, execution, tests, fixtures, TUI, debug tooling, LOD, institutions, and future LLM language surfaces.

Architecture may choose data structures and algorithms. It may not weaken this boundary.

## Core doctrine

```text
Truth may validate actions, but truth may not plan them.
```

Authoritative world truth may:

- decide whether an attempted action succeeds, fails, or is rejected;
- expose observations when a causal perception path exists;
- update physical, social, institutional, ecological, and economic consequences;
- rebuild replay and debug truth;
- compare truth to beliefs in non-diegetic diagnostics.

Authoritative hidden truth must not:

- choose an actor's goal;
- choose an actor's plan;
- choose an actor's routine or HTN method;
- choose a dialogue interpretation or speech act;
- generate an embodied affordance menu;
- decide an institution's suspicion, notice, bounty, contract, payment, sanction, or case status;
- guide LOD promotion behavior;
- appear in an LLM prompt packet for ordinary actor speech or cognition;
- become actor knowledge through debug inspection.

Truth can stop an action. Truth cannot whisper the right action to an actor.

## Required actor decision transaction

Every ordinary actor decision must pass through a canonical actor decision transaction or an architecture with equivalent guarantees.

The transaction exists to prevent scheduler shortcuts, post-hoc explanations, and hidden-truth planning.

A conforming transaction performs these conceptual stages:

1. **Select actor and trigger.** The scheduler identifies the actor, time window, and trigger: no-human tick, possessed command continuation, routine wakeup, action completion, failed validation, observation, report, need threshold, institutional request, LOD promotion, or other modeled cause.
2. **Build sealed actor-known context.** The transaction constructs a holder-specific context from perception, memory, beliefs, expectations, records read, speech heard, relationship state, role assignments, routine assignments, institutional obligations, and modeled inferences. Validator-only truth and debug truth are excluded.
3. **Gather live actor pressures and commitments.** The transaction reads needs, safety, fatigue, hunger, active projects, current intention, routine state, reservations, obligations, relationships, and salient memories through actor-authorized interfaces.
4. **Generate candidates from actor-known inputs only.** Candidate goals/actions arise from needs, intentions, routines, projects, perceived affordances, contradictions, reports, threats, opportunities, and obligations.
5. **Explain candidates.** Candidate generation and selection must produce typed or structurally inspectable decision traces.
6. **Select action, wait, replan, ask, search, continue, abandon, or stuck outcome.** The transaction may produce an action proposal or a diagnostic/fallback result.
7. **Validate against authoritative state.** The shared action pipeline checks physical truth, social/normative truth, resources, access, reservations, duration, risks, and process authority.
8. **Apply event(s) or rejection/failure semantics.** Meaningful outcomes commit through the event log.
9. **Generate observations and belief/projection updates.** Actors and institutions learn only through modeled channels.
10. **Preserve replay determinism.** Replay must rebuild or verify the decision trace, actor-known context ancestry, validation result, events, and projections.

The exact internal ordering may vary where architecture proves equivalent safety. The boundary may not vary.

## Actor-known context

An actor-known context is a sealed, provenance-bearing, holder-specific view of what can legitimately affect cognition.

It may include:

- current local perception;
- remembered observations;
- source-backed beliefs;
- stale beliefs with acquisition time;
- contradictions and expectations;
- visible traces and discovered absences;
- speech acts received;
- reports heard or filed;
- records, notices, contracts, and ledgers read or authored;
- known places, routes, homes, workplaces, beds, tools, supplies, offices, and people;
- relationship knowledge and reputation;
- routine, job, home, and role assignments known to the actor;
- institutional obligations known to the actor;
- modeled inferences from actor-known facts;
- fixture/prehistory facts marked as starting knowledge with source tags;
- LOD summary knowledge explicitly assigned to the actor, group, or institution.

It may not include:

- raw physical truth not perceived or remembered;
- true item locations unknown to the actor;
- true culprit identity unknown to the actor;
- true route safety unknown to the actor;
- true institutional conclusion before procedure;
- debug-only omniscience;
- validation internals not exposed by modeled feedback;
- LLM hidden context;
- UI/player memory;
- branch names, filenames, tests, labels, or prompt text as ordinary world knowledge.

## Provenance classes

Architecture may name provenance classes differently, but it must preserve these distinctions:

| Provenance class | Meaning | Cognition use |
|---|---|---|
| Observed now | Current perception through modeled channel and visibility rules. | Allowed. |
| Observed previously | Prior observation retained as memory or belief. | Allowed, with staleness risk. |
| Remembered belief | Holder-specific retained belief. | Allowed, with confidence/source. |
| Hearsay/testimony | Claim received from another holder. | Allowed, with speaker/source/trust. |
| Record-derived | Claim read from artifact or institutional record. | Allowed, with artifact/source. |
| Routine/role assignment | Known home, work, duty, procedure, or schedule assignment. | Allowed when source-backed. |
| Inference | Modeled inference from actor-known premises. | Allowed when premises are source-backed. |
| Fixture/prehistory seed | Starting knowledge established by content. | Allowed only with explicit setup provenance. |
| LOD summary knowledge | Abstracted knowledge carried across detail boundaries. | Allowed only with retained ancestry. |
| Pipeline validation truth | Authoritative truth used to accept/reject/resolve proposal. | Forbidden for proposal/planning. |
| Debug omniscience | Non-diegetic truth inspection. | Forbidden for ordinary cognition. |
| Unproven raw physical truth | Direct state lookup masquerading as knowledge. | Forbidden. |
| LLM output | Generated text or parse result. | Proposal only; must validate and source. |

A boolean such as `actor_known_only = true` is never sufficient proof. A string label is never sufficient proof. The provenance graph or equivalent structural evidence must be inspectable.

## Proposal versus validation

The foundation requires a strict split:

```text
actor-known context -> proposal
authoritative truth -> validation/resolution
modeled feedback -> future actor knowledge
```

Examples:

- An actor may propose to eat because they remember food in the pantry. The validator may reject the action because the pantry is empty. The actor may then learn emptiness only if the attempt or inspection produces an observation.
- An actor may report a monster because they saw tracks, heard testimony, or survived an attack. An authority may post a notice only if institutional procedure and resources support it. The authority must not post because hidden truth says a monster exists.
- A guard may accuse the wrong person if actor-known or institution-known evidence supports suspicion. The correct culprit must not be inserted by truth.
- A possessed actor may search a location the human knows from debug only only if the action is valid as search, speculation, reckless behavior, or actor-known inference. The action must not be granted hidden certainty.

## Scheduler boundary

The scheduler may:

- choose which actor or process receives a decision window;
- apply due scheduled completions;
- advance deterministic time;
- manage ordering, reservations, and interruption windows;
- invoke the actor decision transaction;
- run no-human simulation and time acceleration;
- emit scheduler diagnostics.

The scheduler must not:

- choose true food, workplace, sleep place, route, target, culprit, witness, threat, or institution outcome for actor cognition;
- dispatch primitive actions from routine labels;
- dispatch primitive actions from hunger/fatigue thresholds;
- treat lack of an epistemic projection as actor-known safety;
- create post-hoc explanations for choices made from truth;
- count continuation labels as behavioral progress without ordinary action ancestry.

Temporal authority follows the same boundary. Raw simulation time, exact event order beyond what a holder can know, future scheduled completions, true office windows, true due states, and exact summary intervals are validator/debug truth unless source-backed for the holder. The holder-known version may be a perceived day-part, remembered appointment, read timestamp, routine assignment, public bell, posted hours, institutional queue state, due notice, stale record, or summary knowledge with ancestry. See foundation doc 03's temporal authority section and `INV-112`.

## Institutions and records under the firewall

Institutions have institution-known contexts. They are subject to the same firewall.

An institution may act from:

- reports;
- testimony;
- records;
- notices;
- patrol observations;
- role knowledge;
- office procedures;
- budgets/resources;
- evidence thresholds;
- public artifacts;
- institutional memory;
- LOD summary knowledge with ancestry.

An institution may not act from hidden truth. A court finding, bounty, contract, wage, sanction, patrol order, or notice must be produced by procedure and resource state, not narrative need.

## LLM and language boundary

LLMs may operate only as proposal/rendering/parsing surfaces behind validation.

Any prompt packet for ordinary actor speech or interpretation must be actor-filtered. It must not include hidden truth, debug-only state, future plot needs, target outcomes, or other possessed actors' knowledge.

LLM output can become world-relevant only after conversion into validated structured speech acts, records, summaries, or debug artifacts. Unsupported output must be rejected, repaired, or classified as speculation/surface text. It must not become truth.

## Debug quarantine

Debug mode may reveal hidden truth and compare it to beliefs. This is necessary for audit.

Debug mode must be visibly non-diegetic and structurally quarantined:

- debug truth does not enter actor notebooks;
- debug inspection does not create memory;
- debug reports do not create records;
- debug mismatch explanations do not become rumors;
- debug commands are explicit test/debug events, not ordinary play;
- debug strings are not parsed as authoritative diagnostic state.

## LOD and regional summaries

LOD may summarize. It may not contaminate.

Low-detail regional processes may use aggregate truth for simulation maintenance, but later promoted actors and institutions must receive only knowledge that has modeled ancestry: report, rumor, observation, record, role assignment, public notice, or summary knowledge explicitly attributed to them.

Summary events must preserve enough causal and epistemic ancestry to explain later decisions.

## Required diagnostics

A conforming system must support review of:

- actor-known context contents;
- provenance graph or equivalent;
- candidate goals/actions considered;
- rejected candidates;
- selected intention/routine/method;
- proposal submitted;
- validation report;
- actor-visible feedback;
- debug-only validator truth;
- belief/projection updates;
- hidden-truth audit result;
- replay comparison.

Diagnostics must be typed or structurally inspectable. Display prose may summarize; it must not be the source of proof.

## Acceptance implications

A runnable phase is not accepted if it lacks gates for:

- no hidden-truth proposal generation;
- sealed actor-known context;
- provenance-bearing cognition inputs;
- shared pipeline validation;
- typed decision traces;
- stuck/replan diagnostics;
- possession parity;
- embodied view filtering;
- debug quarantine;
- replay of cognition-relevant state;
- LLM-disabled operation.

The strongest test is adversarial: make the true answer available to debug/validation but unavailable to the actor. The actor must not plan from it. The world may still reject the actor's mistaken attempt.

## Anti-patterns

Reject these immediately:

- "The actor goes to the true food location because they are hungry."
- "The guard suspects the culprit because the culprit did it."
- "The routine says work, so the scheduler emits a work action."
- "The planner reads all reachable routes and chooses the safe one although the actor never learned it."
- "The LLM dialogue prompt includes hidden truth so the line sounds better."
- "The notice appears because the player needs a job."
- "Debug tells the actor where to go."
- "A decision trace is a display string assembled after action selection."
- "A low-detail process promotes an actor with unexplained knowledge."
- "Tests pass because the fixture label or branch name implies the answer."

## Design freedom

This document does not require a specific AI framework. Tracewake may change algorithms radically if the replacement preserves the transaction and firewall. Symbolic BDI, HTN methods, bounded local planning, utility heuristics, behavior trees, planners, learned policies, or LLM-assisted speech surfaces are all subordinate to the same rule:

```text
Only provenance-bearing holder-known context may feed cognition.
```
