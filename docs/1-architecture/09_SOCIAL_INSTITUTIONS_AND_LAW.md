# Social Institutions and Law

## Core claim

Institutions are simulated social machines made of people, roles, procedures, records, resources, jurisdictions, norms, incentives, biases, and failures.

A guard is not an omniscient enforcement script. A guard is a tired person occupying a role, acting from orders, reports, beliefs, fear, bias, relationships, hunger, patrol routes, and available time.

## Institution model

```yaml
Institution:
  id: reeves_office
  kind: local_authority
  jurisdiction:
    places: [village, north_road, mill, farms]
    people_scope: residents_and_visitors_under_town_law
  roles: [reeve, clerk, guard, tax_collector]
  resources:
    money: 1200
    guards_available: 2
    paper: 40
    jail_cells: 1
    staff_time_today: limited
  records:
    - incident_ledger
    - bounty_ledger
    - tax_ledger
    - warrant_box
  norms:
    - norm_no_theft
    - norm_report_serious_threats
    - norm_office_hours
  biases:
    status_bias: high
    outsider_bias: medium
    kinship_bias: medium
```

## Roles

Roles confer obligations, permissions, prohibitions, credibility, access, and procedural powers.

```yaml
Role:
  id: guard
  obligations:
    - patrol_assigned_route
    - respond_to_active_threat_if_ordered
    - report_serious_incidents
  permissions:
    - carry_weapon_inside_walls
    - question_suspects_under_conditions
    - detain_under_conditions
    - enter_private_home_with_warrant_or_emergency
  prohibitions:
    - steal_evidence
    - accept_bribes
    - abandon_post_without_reason
```

A person can violate a role. Consequences occur only if violation is observed, reported, inferred, or later discovered.

## Norms

Norms are explicit.

```yaml
Norm:
  id: norm_no_theft
  kind: prohibition
  action_pattern: TakePropertyWithoutPermission
  conditions:
    - item_has_legal_owner
    - actor_lacks_permission
    - jurisdiction_recognizes_property
  violation_event: TheftViolation
  default_sanctions:
    - return_property
    - fine
    - jail_if_severe
```

Norm types:

- prohibition;
- obligation;
- permission;
- constitutive norm;
- status norm;
- procedural norm;
- sanction rule;
- jurisdiction rule;
- evidence threshold.

## Violation is not detection

The pipeline is:

```text
action occurs
 -> norm engine classifies possible violation for debug/procedure
 -> traces or observations may exist
 -> agents may form suspicion
 -> report may be made
 -> institution may open record
 -> evidence may satisfy or fail procedure
 -> sanction may or may not occur
```

Institutions act from institutional knowledge, not ground truth.

## Institutional facts

Institutional facts are facts because a recognized procedure or role makes them so.

Examples:

- a person is under arrest;
- a notice is officially posted;
- a bounty is authorized;
- a contract is valid;
- a tax debt exists;
- a guild license is revoked;
- a house is seized;
- a witness statement is entered.

Institutional facts have provenance. They can be wrong, forged, contested, overturned, ignored, or lost.

## Records

Records are durable world artifacts.

```yaml
IncidentReport:
  reporter: actor_tomas
  received_by: actor_anna_clerk
  institution: reeves_office
  time_received: 142-08-12T09:12
  claims:
    - coin_stack_01 missing from strongbox
    - last_seen yesterday_evening
    - elena_heard_possible_noise
  confidence: 0.48
  status: open
  physical_record: ledger_page_43
```

Records can be read, hidden, copied, forged, destroyed, misfiled, or contradicted.

## Bounty lifecycle

```text
harm
 -> observation
 -> survivor reaches authority
 -> report received
 -> record created
 -> authority assesses threat
 -> jurisdiction/resource check
 -> funds reserved
 -> proof requirements defined
 -> clerk writes notice
 -> notice carried
 -> notice posted
 -> agents read it
 -> agents act from belief
 -> proof submitted
 -> proof verified/disputed/refused
 -> payment, partial payment, fraud accusation, or stale closure
```

At no point is this a quest primitive.

## Jurisdiction

Jurisdiction affects:

- whether the institution cares;
- what procedure applies;
- whether the victim is protected;
- who has authority;
- what evidence is admissible;
- whether suspect status matters;
- whether another institution contests the case;
- whether sanctions can be enforced.

Gameplay emerges when criminals exploit borders, guards refuse cases, guilds handle internal matters, nobles avoid ordinary courts, households conceal abuse, temples claim sanctuary, and villages hire outsiders.

## Bias and corruption

Bias affects:

- credibility;
- priority;
- evidence thresholds;
- punishment;
- record tampering;
- notice posting;
- payment;
- search willingness;
- protection;
- rumor uptake.

Inputs include wealth, class, kinship, outsider status, reputation, occupation, role, species/culture if modeled by domain pack, bribes, fear, political pressure, and personal relationships.

Corruption is causal. A bribe requires actor, offer, acceptance/refusal, risk, memory, trace, and later vulnerability.

## Institutions beyond law

### Households

Shared property, domestic access, inheritance, meals, storage, childcare, privacy, reputation, internal obligations.

### Guilds

Work permissions, training, prices, contracts, sanctions, apprenticeships, quality disputes, debt.

### Temples

Ritual, sanctuary, confession, moral authority, charity, taboo, record keeping, oaths.

### Taverns

Food, gossip, hiring, witnesses, debt, rumor import/export, social visibility.

### Markets

Prices, public disputes, theft opportunities, customer expectations, credit, supply shock.

### Adventuring companies

Recruitment, pay shares, risk thresholds, proof disputes, casualties, reputation, desertion.

### Bandit groups

Loot division, scouting, discipline, fences, fear, deserters, internal betrayal, route selection.

## Delay

Institutional processes take time.

Clerks sleep. Guards are already assigned. Messengers get lost. Funds run out. Notices remain posted after facts change. A stale notice is a successful causal artifact, not a bug.

## First-slice institution

Implement one local authority:

- reeve;
- clerk;
- two guards;
- office with hours;
- incident ledger;
- bounty ledger;
- notice board;
- small money reserve;
- property/theft norm;
- trespass/privacy norm;
- road-threat procedure;
- report action;
- simple questioning procedure;
- proof/payment procedure.

## Anti-patterns

- global wanted meter;
- instant guard knowledge;
- bounty appears from incident alone;
- report accepted without receiver;
- court outcome decided by quest state;
- guards who never eat, sleep, fear, or misunderstand;
- law UI showing objective guilt;
- institutional records without physical or procedural provenance;
- payment from nowhere;
- sanctions without detection or institutional belief.
