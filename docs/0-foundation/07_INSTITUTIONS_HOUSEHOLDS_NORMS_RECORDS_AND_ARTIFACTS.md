# Institutions, Households, Norms, Records, and Artifacts

## Core claim

Institutions in Tracewake are fallible social machines.

They act through people, roles, norms, records, procedures, custody, funds, office hours, manpower, delay, bias, jurisdiction, fear, relationships, corruption, fatigue, misunderstanding, and failure.

Households are the first institutions. Records are world artifacts or institutional state, not UI panels. Notices are public artifacts, not quest flags.

## Institutional ontology

An institution is any durable social machine that can maintain roles, norms, records, resources, obligations, authority, procedure, and shared belief across events.

Examples:

- household;
- workshop;
- farm;
- tavern;
- shop;
- reeve/clerk office;
- council;
- market;
- guild;
- caravan;
- militia;
- gang or informal power network;
- court;
- temple/church/shrine or secular equivalent in a domain pack;
- company;
- army;
- larger political authority.

The first implementation needs only a modest subset. The kernel must support the general shape without making future organizations exceptions.

An institution should be representable through:

```text
InstitutionId
members and roles
authority scope / jurisdiction
places, office hours, and access rules
procedures
resources and budgets
records and record custody
public/private knowledge surfaces
trusted channels
norms and sanctions
credibility/bias rules
failure modes
```

When the UI says an institution “knows” something, one or more modeled carriers must exist: record, institutional belief, member belief in role context, public notice, procedure result, accepted report, or other explicit state. It must not mean the institution queried ground truth.

## First public institution

The first public institution should be a village reeve/clerk/town-office structure.

It exists to prove that public authority can be useful, causal, wrong, and limited.

It should support staged versions of:

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
- simple testimony/evidence acceptance;
- social-position and credibility effects;
- limited funds;
- failure to pay;
- misclassification;
- wrong suspicion;
- lost, stale, damaged, hidden, forged, or corrupted records;
- procedure interrupted by sleep, food, work, fear, absence, or lack of materials.

The office is not a quest hub. It may issue, receive, record, misread, ignore, dispute, reward, accuse, or fail.

## Households as domestic institutions

Households must exist from the beginning, even if simple.

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
- secrets;
- conflicts;
- trust and suspicion;
- debt/dependency hooks;
- inheritance hooks.

A household can be wrong. It can treat an item as shared when an individual claims it privately. It can protect a member, suspect a guest, hide shame, misremember who last used a tool, or enforce a norm without public authority.

The first village needs households because ordinary life needs homes, storage, access, routine, dependency, secrecy, and expectation contradiction.

## Norm categories

Norms must be explicit enough to inspect and test.

Tracewake needs at least:

```text
obligation      something an actor/role ought to do
permission      something an actor/role may do
prohibition     something an actor/role ought not do
power           something an actor/role can make institutionally true
sanction        possible consequence of violation or recognition
procedure       required path for recognition/action
constitutive    rule that makes one act count as another in context
proof rule      what claim/evidence a role treats as sufficient
access rule     who may reach/use/open/take under conditions
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

Norms constrain and evaluate behavior. They do not force behavior and they do not imply detection.

## Violation, detection, proof, enforcement, and justice are separate

A violation is not detection.

Detection is not proof.

Proof is not enforcement.

Enforcement is not justice.

A theft chain may look like:

```text
property taken
-> absence/trace exists
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

Every arrow is a modeled event/channel. Every arrow may fail.

The engine must distinguish:

- the action that violated a norm;
- whether anyone observed it;
- whether anyone inferred it;
- whether anyone believed it;
- whether it was reported;
- whether the report was accepted;
- whether an institution acted;
- whether anyone was sanctioned;
- whether the sanction was justified by ground truth.

## Ownership, custody, access, control, proof, and belief

Property cannot collapse into inventory ownership.

Tracewake separates:

```text
ownership claim      who is socially/institutionally treated as owning it
custody              who or what currently holds it
access permission    who may reach/use/open/take it under norms
physical control     who can manipulate it right now
proof/provenance     why a claim can be supported or disputed
belief about status  who believes any of the above
```

Buying, stealing, borrowing, storing, hiding, gifting, promising payment, paying wages, paying rewards, confiscating, losing, finding, and inheriting property must operate through these distinctions.

## Records

A record is a world artifact or institutional state object that carries typed claims through time.

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
- public proclamations;
- future court, guild, army, church, company, or council records.

A record should carry, where relevant:

```text
RecordId
kind
author / writer
issuer / authority
creating actor or process
creation event and time
claimed event time if known
typed claims
source/provenance
custodian
physical or institutional location
access rules
intended audience
status
staleness indicators
contradiction links
revision/copy/erasure history
```

Records can be true, false, partial, stale, forged, misfiled, lost, unread, ignored, contradicted, copied, damaged, hidden, destroyed, corrected, or institutionally accepted despite being wrong.

Record creation, copying, reading, filing, loss, destruction, forgery, correction, and erasure are events.

## Runtime artifact rule

After simulation start, a record-like artifact exists only through modeled cause.

A bounty letter, report, notice, accusation, order, rumor, ledger entry, warrant, debt note, contract, or public proclamation must exist because an actor, household, institution, or regional process had:

```text
reason or procedure
means/material/channel
source claims
role or willingness
access/custody path
creation/copy/post/file/carry event
```

Scenario authors may seed old artifacts, old records, unresolved accusations, long-past events, boundary conditions, and prehistory. These must be marked with structured provenance as authored prehistory, generated prehistory, simulated summary ancestry, or declared boundary input.

Runtime authored outcome chains are forbidden.

## Notices

A notice is a public artifact.

A notice may contain:

```text
issuer
writer/poster
authority/procedure
posting time and location
structured claims
source report/record/provenance
intended audience
reward/promise if any
proof requirement
fund/custody reference if any
jurisdiction
expiration/review time if any
status
contradictions/amendments
physical artifact state
```

A notice does not guarantee:

- the claim remains true;
- the target exists;
- the target is where the notice says;
- the issuer is honest;
- the issuer can pay;
- the reward fund exists;
- the player is intended to solve it;
- the world will wait;
- completion will be recognized automatically.

NPCs may read, ignore, act on, remove, forge, update, misread, solve, or be harmed by notices.

The TUI may project a notice into an actor-known lead card. The source remains the notice, not objective quest state.

## Reports and testimony

A report is a speech or record action attempting to move typed claims into a social/institutional channel.

A report must have:

```text
reporter
recipient/audience/receiving role
channel
claims
source basis if stated
time
recipient credibility/procedure evaluation
accepted/refused/delayed/misclassified status
resulting beliefs/records/actions if any
```

A report may be sincere, mistaken, malicious, coerced, incomplete, stale, garbled, hearsay, or intentionally false.

Testimony is not truth. It is a claim with provenance.

## Procedure failure

Institutional procedure may fail because of:

- absent role-holder;
- office closed;
- lack of time/materials;
- poor credibility;
- jurisdiction dispute;
- bias;
- corruption;
- fear of powerful actors;
- insufficient proof;
- misclassification;
- lost or damaged record;
- conflicting claims;
- limited funds;
- fatigue/hunger/illness;
- interruption by routine or emergency.

These failures are not polish. They are required proof that institutions are social machines.

## Bias, social position, and credibility

Agents and institutions may weigh claims through:

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
- contradiction history;
- power and vulnerability.

Bias must be inspectable and testable. It is not an excuse for hidden drama scripting. Wrong suspicion is desirable only when its causes are legible.

## Economy and survival through institutions

V1 does not simulate full macroeconomics. It must still make survival institutional and causal.

Required:

- households manage food/storage/access;
- workplaces create obligations and wages/payment promises;
- shops/taverns expose stock/service abstractions;
- offices can hold funds and refuse/delay payment;
- debts and promises can motivate action;
- money and food move through custody and ownership/access claims;
- records can support or fail to support claims.

## Future organizations

Future organizations must use the same machinery.

A guild is roles, membership, dues, craft norms, records, apprenticeships, sanctions, and reputation.

A gang is relationships, fear, loyalty, territory, debts, threats, secrecy, records or memory, and violence risk.

An army is recruitment, command, pay, supply, loyalty, morale, fear, travel, food, injury, desertion, records, orders, rumor, and institutional authority.

A court is procedure, claims, evidence, testimony, jurisdiction, roles, records, sanctions, bias, delay, and appeal.

A settlement conquest is not a conquest quest. It must arise from ordinary causal machinery.

## Acceptance checks

Institution, household, norm, record, and artifact features are not done unless they answer:

```text
What claim, role, norm, or obligation exists?
Who holds or enforces it?
What produced it?
What record/artifact/trace carries it?
Who can access it?
Who can be wrong about it?
What procedure recognizes it?
What resources, custody, authority, or jurisdiction limit it?
How can it fail, go stale, be contradicted, be forged, be lost, or be abused?
Can NPCs use or suffer the same machinery?
Can the TUI show the actor-known version and debug show the causal version?
```
