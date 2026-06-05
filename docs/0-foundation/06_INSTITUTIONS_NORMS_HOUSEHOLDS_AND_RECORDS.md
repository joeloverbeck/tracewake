# Institutions, Norms, Households, and Records

## Core claim

Institutions in Tracewake are fallible social machines.

They do not read ground truth. They act through people, roles, authority, procedure, records, custody, funds, office hours, delay, bias, jurisdiction, fear, relationships, corruption, fatigue, misunderstanding, and failure.

Households are not flavor or spawn points. They are the first domestic institutions: social units with members, roles, shared storage, private spaces, obligations, permissions, conflicts, and inherited memory.

Records are not UI panels. They are world artifacts with authors, issuers, custody, claims, timestamps, access rules, provenance, staleness, and contradiction potential.

A notice board is not a quest list. It is a public surface where world artifacts may be posted, ignored, removed, forged, contradicted, or made stale by events.

## Institutional ontology

An institution is any durable social machine that can maintain norms, roles, records, resources, obligations, and procedures across multiple events.

Examples:

- household;
- workshop;
- farm;
- tavern;
- shop;
- reeve or clerk office;
- council;
- market;
- guild;
- caravan;
- militia;
- temple, church, shrine, or non-religious equivalent in a domain pack;
- gang or informal power network;
- court or legal office in a later domain pack.

The foundation does not require all of these in v1. It requires the ontology to support them without rewriting the kernel.

An institution should be representable through:

```text
InstitutionId
members and roles
authority scope
jurisdiction
procedures
resources and budgets
places and office hours
records and record custody
access rules
public/private knowledge surfaces
trusted channels
bias and credibility rules
failure modes
norms and sanctions
```

When the UI says an institution “knows” something, that must mean one or more modeled carriers exist: a record, institutional belief, member belief in role context, public notice, rumor accepted by procedure, or other explicit state. It must not mean the institution queried ground truth.

## The first public institution

The first public institution should be modest: a village reeve/clerk/town-office structure.

It exists to prove that public authority can be causal, useful, wrong, and limited.

The first implementation should support some staged subset of:

- office location;
- office hours;
- clerk or reeve actor;
- limited authority;
- jurisdiction;
- report intake;
- refusal or delay;
- incident/case record creation;
- record custody;
- public notice posting;
- simple evidence/testimony acceptance;
- social-position and credibility effects;
- limited funds;
- failure to pay;
- misclassification;
- wrong suspicion;
- lost, stale, damaged, hidden, or corrupted records;
- procedure that can be interrupted by sleep, food, work, fear, obligation, or absence.

This office must not be a quest giver. It may issue, receive, record, misread, ignore, dispute, reward, accuse, or fail.

## Norms

Norms must be explicit enough to inspect and test.

Tracewake needs at least these norm categories:

```text
obligation     something an actor/role ought to do
permission     something an actor/role may do
prohibition    something an actor/role ought not do
power          something an actor/role can make institutionally true
sanction       possible consequence of violation or recognition
procedure      required path for institutional recognition
constitutive   rule that makes an act count as another act in context
```

Examples:

```text
A household member may take bread from shared household storage.
A guest may not open the locked office chest.
A clerk may create an incident record during office procedure.
A reeve may authorize payment only from funds under office custody.
A witness report counts as testimony only when attributed to a speaker and channel.
A theft accusation becomes an institutional claim only after report intake or equivalent procedure.
```

Norms are not omnipotent. They constrain and evaluate behavior; they do not force behavior.

## Violation, detection, proof, enforcement, and justice are separate

A violation is not detection.

Detection is not proof.

Proof is not enforcement.

Enforcement is not justice.

A theft chain may look like this:

```text
property taken
-> physical trace or absence exists
-> victim expects property and discovers absence
-> victim forms belief
-> victim tells spouse
-> spouse doubts or repeats it
-> report reaches clerk
-> clerk records partial claim
-> reeve assigns suspicion using weak evidence and bias
-> wrong person is questioned
-> record remains stale after culprit leaves
```

Every arrow is a modeled event or channel. Every arrow may fail.

The engine must be able to distinguish:

- the action that violated a norm;
- whether anyone observed it;
- whether anyone inferred it;
- whether anyone believed it;
- whether it was reported;
- whether the report was accepted;
- whether an institution acted;
- whether anyone was sanctioned;
- whether the sanction was justified by ground truth.

## Households as domestic institutions

Households should exist from the beginning, even if simple.

A household is a social unit with:

- members;
- roles;
- shared and private spaces;
- shared and private storage;
- access rules;
- property claims;
- custody practices;
- food routines;
- sleeping routines;
- work coordination;
- care obligations;
- privacy expectations;
- household knowledge;
- conflicts;
- trust and suspicion;
- debt or dependency hooks;
- inheritance hooks for later systems.

A household can be wrong. It can treat an item as shared when an individual claims it privately. It can protect a member, suspect a guest, hide shame, misremember who last used a tool, or enforce a norm without public authority.

The first village needs households because ordinary life needs homes, storage, access, routine, dependency, secrecy, and domestic expectation.

## Ownership, custody, access, and control are distinct

Tracewake must not collapse all property into “inventory owner.”

The foundation needs at least these separations:

```text
ownership claim      who is socially/institutionally treated as owning it
custody              who or what currently holds it
access permission    who may reach/use/open/take it under norms
physical control     who can actually manipulate it right now
belief about status  who believes any of the above
proof/provenance     why a claim can be supported or disputed
```

Examples:

- A miller owns coins, but the coins are in a strongbox under household custody.
- A spouse can physically open the cupboard but may not claim personal ownership of all contents.
- A clerk holds an office ledger, but the town office has custody and access rules.
- A thief controls hidden coins, but ownership recognition has not changed.
- A forged receipt can influence belief without changing ground truth.

Buying, stealing, borrowing, storing, hiding, gifting, promising payment, paying wages, paying rewards, confiscating, and losing property must operate through these distinctions.

## Records

A record is a world artifact or institutional state object that carries claims through time.

Records include:

- ledgers;
- notices;
- reports;
- receipts;
- contracts;
- debts;
- warrants;
- work orders;
- household lists;
- tax records;
- incident files;
- testimony summaries;
- maps;
- route warnings;
- custody logs;
- letters;
- public proclamations.

A record should carry, where relevant:

```text
RecordId
author
issuer
creating actor or process
creation time
claimed event time if known
claims
source/provenance
custodian
physical or institutional location
access rules
intended audience
status
staleness indicators
contradiction links
revision or erasure history
```

Records can be true, false, partial, stale, forged, misfiled, lost, unread, ignored, contradicted, copied, damaged, hidden, destroyed, corrected, or institutionally accepted despite being wrong.

Record creation, copying, reading, filing, loss, destruction, forgery, correction, and erasure are events.

## Notices are public artifacts, not quests

A notice may contain:

```text
issuer
posted by
posting time
claim text or structured claim
reward promise if any
proof requirement
fund/custody reference if known
jurisdiction
expiration or review time if any
source of report
contradictions or amendments
physical notice artifact
```

A notice does not guarantee:

- the claim remains true;
- the target exists;
- the target is where the notice says;
- the reward fund exists;
- the issuer will pay;
- the player is intended to solve it;
- the world will wait;
- completion will be recognized automatically.

NPCs may read, ignore, act on, remove, forge, update, misread, or solve notice claims.

The TUI may project a notice into an actor-known lead card, but the source remains the notice, not objective quest state.

## Reports and testimony

A report is a speech or record action that attempts to move a claim into an institutional or social channel.

A report must have:

- reporter;
- audience or receiving role;
- channel;
- claim;
- source basis if stated;
- time;
- credibility as evaluated by recipient or procedure;
- accepted/refused/delayed status;
- resulting beliefs and records if any.

A report may be sincere, mistaken, malicious, coerced, incomplete, stale, garbled, hearsay, or intentionally false.

Testimony is not ground truth. It is a claim with provenance.

## Economy and ordinary survival

V1 must not attempt a full macroeconomic simulation. It must still make ordinary survival causal.

At minimum, the foundation requires:

- hunger and sleep as pressures;
- food as inventory/custody/service, not a fake meter refill;
- money as custody and ownership claim;
- work as routine, obligation, access, wage, and social identity;
- shops, taverns, workplaces, and households as stock/service abstractions with causal hooks;
- buying, stealing, storing, hiding, paying, promising payment, and refusing payment as real actions;
- travel and access costs that affect timing and opportunity;
- storage and containers that can produce absence, expectation contradiction, theft, search, and evidence.

A job should not just add coins on a schedule. It should imply place, time, role, obligation, access, absence from home, witnesses, fatigue, relationships, wages, possible delay, and possible failure.

A meal should not just fill a bar. It should come from food, service, charity, purchase, theft, household access, or other modeled channel.

## Bias, social position, and credibility

Institutions and agents may weigh claims differently based on:

- relationship;
- role;
- status;
- prior reputation;
- debt;
- fear;
- prejudice;
- kinship;
- occupational trust;
- institutional incentives;
- past reliability;
- current evidence;
- contradiction history.

Bias must be inspectable and testable. It must not be a hidden excuse for drama scripting.

Wrong suspicion is desirable only when it has legible causes.

## Acceptance implications

Institution, household, norm, and record features are not done unless they can answer:

```text
What claim or obligation exists?
Who holds it?
What produced it?
What records or traces carry it?
Who can access it?
Who can be wrong about it?
What procedure recognizes it?
What resources or authority limit action?
How can it fail, go stale, be contradicted, or be abused?
Can NPCs use or suffer the same machinery?
Can the TUI show the actor-known version and debug show the full causal version?
```
