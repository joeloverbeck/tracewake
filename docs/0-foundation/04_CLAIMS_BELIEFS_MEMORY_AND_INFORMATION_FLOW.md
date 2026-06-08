# Claims, Beliefs, Memory, and Information Flow

## Core claim

Tracewake is epistemic. Agents do not act from truth. They act from what they believe, remember, infer, read, hear, mishear, misremember, trust, doubt, and socially propagate.

The foundation therefore requires a typed claim/proposition substrate. Without it, beliefs become prose blobs, records become UI text, rumors become vibes, notebooks become quest logs, and LLM output becomes accidental authority.

## Three separated realities

Tracewake keeps these layers distinct:

```text
ground truth
  what the authoritative event-derived simulation says happened

subjective belief
  what an actor, household, institution, office, group, or rumor network believes or treats as plausible

public/institutional/household record
  what artifacts and social machines claim, store, post, file, copy, cite, ignore, or enforce
```

These layers may agree. They often should not.

A record can be false. A witness can be confident and wrong. A household can protect a lie. An actor can know the truth but fail to prove it. An institution can believe a rumor. A notice can remain stale. Ground truth does not repair these layers automatically.

## Typed claims/propositions

A claim/proposition is a structured assertion about the world, a belief-holder, an event, an object, a place, a relationship, a norm, an obligation, a record, or a condition.

The same claim substrate supports:

- actor beliefs;
- memories;
- observations;
- inferences;
- testimony;
- lies;
- rumors and gossip variants;
- accusations and suspicions;
- reports;
- notices;
- ledgers;
- ownership/access/custody claims;
- debts and contracts;
- promises and threats;
- institutional facts;
- household knowledge;
- actor-known lead cards;
- source-bound notebook entries.

Raw prose may render claims. Raw prose must not be authoritative state.

## Claim shape

At foundation level, important claims need enough structure to support truth divergence, provenance, validation, contradiction, rendering, and replay.

A claim should be representable with fields like:

```yaml
Claim:
  claim_id: claim_000123
  type: ItemLocatedInContainer
  arguments:
    item: coin_stack_01
    container: strongbox_tomas
  qualifiers:
    time_scope: believed_at_or_before_142_08_12_morning
    modal: expected_true
    uncertainty: ordinary_property_memory
  domain: physical_property
```

The specific schema belongs in architecture. Foundation requires that claims be typed, referential, comparable, renderable, sourceable, and contradiction-capable.

## Belief shape

A belief is a holder's stance toward a claim.

Important beliefs should preserve:

```text
holder: actor, household, institution, office, group, rumor network, etc.
claim/proposition
stance: believes true, believes false, suspects, doubts, heard, remembers, expects, intends to verify, etc.
confidence/uncertainty
source and source reference
channel: perception, search, testimony, rumor, record, inference, instruction, memory, institutional notification, boundary summary, etc.
acquisition time
believed event time when available
last verification or stale status
provenance chain
contradiction links
identity uncertainty links
scope/privacy/access
speakability/admissibility/action implications where relevant
```

A proposition is not a global flag. `MaraTookCoins` may be true in ground truth, disbelieved by Mara's neighbor, suspected by Tomas, recorded weakly by Anna, distorted by gossip, and unknown to Elias.

## Observations and interpretation

Observation and interpretation are separate.

Elena may hear a sound. She does not thereby know there was a theft. A perception event may create an observation such as:

```text
heard brief low metal/wood sound near Tomas's room around night, low confidence
```

Interpretation may create candidate claims:

```text
SomethingMovedNearTomasRoom
SomeoneWasInHall
HouseSettled
TomasWasAwake
```

Perception must consider channel, distance, lighting, obstruction, noise, fatigue, intoxication, attention, stress, skill, familiarity, expectation, environmental conditions, and identity ambiguity.

## Information channels

Valid information channels include:

- direct perception;
- indirect perception such as hearing through a wall;
- search and inspection;
- expectation contradiction;
- testimony;
- gossip and rumor;
- reading notices, ledgers, letters, contracts, reports, signs, case files, or maps;
- household communication;
- institutional notification;
- instruction, command, or training;
- inference;
- memory retrieval;
- memory distortion;
- social interpretation;
- declared boundary-process import;
- future domain-defined special channels with costs, reliability, provenance, traces, and failure modes.

No telepathy. No global awareness. No update because the player knows.

## Expectation contradiction

Agents discover absence or anomaly through contradiction between expectation and observation/search.

Examples:

- Tomas expects coins in his strongbox, opens it, and sees none.
- A clerk expects the incident ledger in the office chest, but it is missing.
- A spouse expects Mara home by dawn, but Mara is absent.
- A tavern keeper expects payment after service, but payment is not made.
- A worker expects the shop to be open during office hours, but it is closed.

The contradiction is eventful when consequential. It may create surprise, suspicion, search intentions, reports, accusations, revised memories, concealment, or social conflict.

Absence without expectation is not evidence to that actor.

## Search

Search is intentional, costly, bounded, and fallible. It is not a UI query over truth.

A search process should have:

- searcher and motive/question;
- target claim or uncertainty;
- search area;
- method and thoroughness;
- time cost;
- access and norm constraints;
- tools/skills/body state;
- traces inspected;
- possible false negatives;
- possible false positives or irrelevant discoveries;
- observation and belief effects;
- social/institutional consequences;
- interruption and failure events.

The TUI may show actor-possible searches. It may not let a body search exact hidden truth because the human knows where it is.

## Testimony, rumor, gossip, and lies

Speech transmits claims, not truth.

A speech or rumor event should preserve:

```text
speaker
listener/audience
speech act type
claims carried
source belief references
asserted confidence / uncertainty
speaker sincerity or disbelief where relevant
motive, risk, credibility, authority, relationship, fear, bias
listener interpretation
acceptance, rejection, doubt, forgetting, or misremembering
provenance chain
future contradiction potential
```

Rumor may mutate. A low-confidence observation can become a confident false accusation after retelling. That is not a bug. It is one of Tracewake's central mechanisms.

A lie is not invalid because it is false. A lie is a causal speech act in which the speaker asserts or implies a claim they do not believe or have reason to doubt, with motive and risk. A hallucinated unsupported claim from an LLM is not automatically a lie; it is invalid until mapped to a validated speech act with actor motive and authority.

Embodied mode must not label a statement as a lie unless the current actor has modeled grounds. Debug may reveal speaker belief, motive, and contradiction.

## Records as claim carriers

Records are artifacts or institutional state objects that carry claims through time.

Reading a record creates beliefs only if an actor can access, perceive, read, interpret, and at least understand the record. Trust is separate.

A record may be true, false, partial, stale, forged, damaged, biased, incomplete, contradicted, misfiled, inaccessible, or ignored. A record's claims may travel:

```text
victim report
-> office ledger
-> public notice
-> gossip
-> household suspicion
-> accusation
```

Every step can distort, omit, strengthen, weaken, or contradict claims.

## Institutional facts

An institutional fact is a claim treated as true by an institution under its rules and procedures.

Examples:

```text
ReportAcceptedByOffice(report_17)
DebtRecordedAgainst(actor_mara, creditor_iva, amount_x)
ReeveAuthorizedPayment(payment_04)
CaseOpen(missing_property_01)
```

Institutional facts do not require ground-truth correctness. They require modeled authority, procedure, role, record, and provenance. An office can open the wrong case, record the wrong value, trust a liar, refuse a true report, or sanction the wrong person.

## Household knowledge

Households are belief-holding social machines, not passive location groups.

A household may maintain shared knowledge, private knowledge, secrets, expectations, access assumptions, shame, protection, internal accusations, debts, and property norms. Household knowledge may never reach public authority. It may be wrong, stale, biased, or strategically concealed.

## Ownership and claim conflict

Ownership, custody, access, control, proof, and belief-about-ownership are separate claim families.

The system must support claims such as:

```text
Tomas owns coin_stack_01
coin_stack_01 is physically in Mara's pouch
household_tomas treats pantry bread as shared
Anna believes office_ledger is in locked_chest
Mara claims she was given the coins
Tomas cannot prove ownership to the reeve yet
```

Conflict between these claims is expected.

## Memory fallibility

Memory mechanics include formation, retrieval, decay, forgetting, distortion, source confusion, confidence drift, temporal uncertainty, identity uncertainty, emotional salience, rehearsal through retelling, correction, and resistance to correction.

Do not design early structures that assume perfect, permanent, globally indexed memory.

Important memory decay must not delete causal ancestry needed by records, active suspicion, institutional procedure, replay, or debug explanation. Forgetting by an actor is eventful; event history remains inspectable.

## Misidentification and identity uncertainty

The engine must support:

- unknown actor descriptions;
- partial identity claims;
- clothing-based recognition;
- voice recognition;
- social familiarity;
- mistaken identity;
- aliases and roles;
- household/institution identifiers;
- confidence-ranked candidates;
- contradictory identity claims.

A witness may believe “a tall worker in a green cloak left the mill,” not “Mara did it.” Later inference may narrow or mislead.

## Actor knowledge versus human knowledge

Normal play shows the current actor's known/perceived world.

Surfaces must remain separate:

```text
Actor notebook / actor-known leads
  Source-bound actor knowledge. May support actor speech, reports, search, accusations, plans, and obligations.

Human/debug notes
  Out-of-world convenience. Never a source for world action, proof, testimony, or institutional recognition.

Debug inspector
  Non-diegetic truth, event graph, all beliefs, planner state, hidden traces, random draws, and mismatch diagnostics.
```

The human may remember facts from another possession. The current actor may not world-affectingly act from those facts until they arrive through modeled channels.

## Lead cards and notebooks

An actor-known lead is a projection over source-bound claims and beliefs. It must preserve:

```text
viewer/source actor
source event, belief, record, notice, speech act, or observation
claims known to viewer
uncertainty/staleness
possible actor actions
provenance/debug link
```

Bad lead:

```text
Objective: Confront Mara, the thief.
```

Good lead:

```text
Lead: Missing coins from strongbox.
Source: Your search of the strongbox.
Known claims: You expected coins there. They were absent when checked. Elena says she heard a noise.
Uncertainty: No direct witness. Coins may have been moved, borrowed, stolen, or misremembered.
Possible actions: search room, ask household, report to clerk.
```

## UI assistance boundary

Allowed actor-filtered assistance:

- grouping source-bound claims;
- sorting known records;
- showing uncertainty labels;
- explaining why a report cannot be made;
- highlighting contradictions the actor noticed;
- suggesting actor-possible searches or conversations;
- debug-only truth in debug mode.

Forbidden assistance:

- objective culprit labels in embodied mode;
- omniscient quest markers;
- speech options based on facts unknown to the current actor;
- global notebooks that act as evidence;
- progress bars tied to ground truth;
- hidden truth search results;
- automatic institution recognition from human knowledge.

## Acceptance for epistemic features

Any feature involving information must test:

```text
What typed claims exist?
Who holds each stance toward each claim?
What source/channel/provenance produced it?
How can it be false, stale, partial, or contradicted?
What memory effects apply?
What record or artifact carries it?
What speech act can transfer it?
What actor-known UI projection shows it?
What hidden truth must remain hidden?
Can debug explain truth/belief/record divergence?
```

If the answer is “the UI text says so,” the feature is not foundation-compliant.

## 2026 hardening: epistemic taxonomy

The following distinctions are constitutional. Architecture may refine schemas, but must not collapse these concepts.

| Term | Constitutional meaning |
|---|---|
| Authoritative truth | The event-sourced world state and causal history. It validates actions and emits consequences. It is not automatically known. |
| Actor-known truth | Truth that has reached a specific actor through perception, memory, belief, record, testimony, assignment, or modeled inference. It may later become stale. |
| Belief | A holder-specific stance toward a proposition with source, confidence/uncertainty, acquisition time, and contradiction handling. It may be false. |
| Claim | A proposition asserted, implied, spoken, written, reported, remembered, inferred, or rendered by some source. It is not automatically true. |
| Observation | Channel-specific perception or discovery. It is not interpretation. It may be partial, noisy, mistaken, or insufficient. |
| Memory | Retained actor information shaped by source, salience, recency, emotion, repetition, contradiction, relationship, role, and possible forgetting/distortion. |
| Hearsay | A claim received from another holder without direct observation by the receiver. It carries speaker/listener/source provenance. |
| Rumor | Hearsay that may propagate, mutate, lose source quality, gain salience, or contradict other accounts. |
| Lie | A validated speech act where the speaker intentionally asserts something contrary to the speaker's belief or withholds relevant truth under modeled motive/context. A false statement is not automatically a lie. |
| Stale belief | A belief whose source may once have been accurate but may no longer match current truth. It requires time/source tracking and possible verification. |
| Expectation | A belief-like predicted or normative state an actor or institution uses to notice absence, delay, failure, breach, or contradiction. |
| Contradiction | A detected mismatch between belief/expectation/claim/record and observation, report, or later information. It belongs to a holder or procedure. |
| Debug-only omniscience | Non-diegetic inspection of truth and mismatches. It is forbidden as actor/institution cognition input. |

## Provenance requirements

Every action-relevant belief, expectation, contradiction, routine premise, institutional fact, record, notice, rumor, testimony, and memory must be provenance-bearing.

Acceptable provenance includes, when modeled:

- direct sight, sound, touch, smell, or other perception channel;
- observation of an absence under a prior expectation;
- speech act heard or received;
- report filed;
- record read or authored;
- notice posted or read;
- memory retained from a prior event;
- routine/home/work/role assignment known to the actor;
- institutional procedure step;
- social inference from known relationships, norms, or reputation;
- LOD summary with retained source ancestry;
- explicit fixture/prehistory seed marked as starting knowledge, not ordinary-play truth leakage.

Unacceptable provenance includes:

- raw world-state lookup by a planner;
- debug view or hidden-truth audit output;
- branch/test label;
- LLM prose;
- UI marker;
- current repository/session memory;
- an untyped boolean claiming actor-known safety;
- display strings parsed as proof.

## False beliefs and negative information

Tracewake must support false beliefs as first-class causal objects. A false belief can motivate search, accusation, avoidance, secrecy, institutional action, mistaken contracts, stale notices, or failed plans.

Negative information is also first-class when it has an epistemic basis. An actor may believe "the food is absent" only after a search/inspection/perception path or a trusted report, not because the pantry truth table says so. Absence can contradict an expectation only for a holder that had an expectation.

## Belief transfer through social and material carriers

Information can move through:

- speech, including questions, answers, reports, accusations, lies, refusals, promises, testimony, gossip, and rumor;
- artifacts, including ledgers, notes, notices, contracts, maps, signs, seals, damaged objects, containers, and bodies;
- institutions, including intake procedures, offices, role memories, records, evidence thresholds, orders, and sanctions;
- environment traces, including sounds, tracks, missing items, disturbed containers, smoke, road damage, footprints, smell, or changed presence.

Each transfer can degrade, mutate, be misunderstood, be contradicted, be distrusted, be forgotten, or be withheld.

## Actor-known cognition rule

Only provenance-bearing actor-known facts may feed cognition. Planning from unstated truth is forbidden even when the resulting action is plausible. The action validator may later reject a proposal based on truth; the actor does not thereby learn hidden details unless the rejection produces modeled perception, feedback, report, or memory.
