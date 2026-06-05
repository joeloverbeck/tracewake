# Social Institutions and Law

## Core claim

Institutions are simulated actors made of people, roles, procedures, records, resources, jurisdictions, norms, incentives, and failures.

A town guard is not an omniscient enforcement script. A guard is a person occupying a role, acting from reports, orders, beliefs, fatigue, fear, bias, corruption, and available time.

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
    guards_available: 3
    paper: 40
    jail_cells: 2
  records:
    - incident_reports
    - warrants
    - bounties
    - tax_ledger
  biases:
    status_bias: high
    outsider_bias: medium
    kinship_bias: medium
```

## Roles

Roles confer permissions, obligations, prohibitions, and credibility.

```yaml
Role:
  id: guard
  obligations:
    - patrol_assigned_route
    - respond_to_active_threat
    - record_serious_reports
  permissions:
    - carry_weapon_inside_walls
    - question_suspects
    - detain_under_conditions
  prohibitions:
    - steal_evidence
    - accept_bribes
    - abandon_post_without_reason
```

A person can violate a role. Consequences occur only if violation is detected or later discovered.

## Norms

Norms are explicit:

```yaml
Norm:
  id: norm_no_theft
  kind: prohibition
  action_pattern: TakePropertyWithoutPermission
  conditions:
    - item_has_legal_owner
    - actor_lacks_permission
  violation_event: TheftViolation
  default_sanctions:
    - return_property
    - fine
    - jail_if_severe
```

Types: prohibition, obligation, permission, constitutive norm, status norm, procedural norm.

## Violation is not detection

Pipeline:

```text
action occurs
 -> norm engine classifies possible violation for debug
 -> traces or observations may exist
 -> agents may form suspicion
 -> report may be made
 -> institution may open case
 -> evidence may or may not satisfy procedure
 -> sanction may or may not occur
```

Institutions act from institutional knowledge, not ground truth.

## Records

Records are durable world artifacts with provenance.

```yaml
IncidentReport:
  reporter: actor_tomas
  received_by: actor_anna_clerk
  time_received: 142-08-12T09:12
  claims:
    - coin_stack_01 missing from strongbox
    - last_seen yesterday_evening
    - Elena heard possible night noise
  confidence: 0.48
  status: open
```

## Bounty lifecycle

```text
harm
 -> observation
 -> survivor reaches authority
 -> report received
 -> record created
 -> authority assesses threat
 -> funds reserved
 -> clerk writes notice
 -> notice carried
 -> notice posted
 -> agents read it
 -> agents act on belief
 -> proof submitted
 -> proof verified or disputed
 -> payment, refusal, fraud, or stale closure
```

At no point is this a quest primitive.

## Jurisdiction

Jurisdiction matters:

- where did the event happen;
- who has authority;
- is victim protected;
- is suspect local, outsider, noble, guild member, priest, guard, or outlaw;
- does institution have resources;
- does another authority contest the case.

Gameplay emerges when criminals exploit borders, guards refuse cases, guilds handle internal matters, nobles avoid ordinary courts, and villages hire outsiders.

## Bias and corruption

Bias affects credibility, priority, evidence thresholds, punishment, record tampering, notice posting, and payment. Inputs include class, kinship, culture/species if modeled, wealth, reputation, bribes, fear, political pressure, and personal relationships.

## Institutions beyond law

Households handle shared property and domestic permission. Guilds handle work permissions and contracts. Temples handle ritual, confession, sanctuary, and moral authority. Taverns handle rumor, hiring, debt, and witnesses. Markets handle prices, theft, observation, and public dispute. Adventuring companies handle recruitment, shares, proof, casualties, and payment disputes. Bandit groups handle loot division, scouting, discipline, deserters, and fences.

## Delay

Institutional processes take time. Clerks sleep. Guards are busy. Messengers get lost. Funds run out. A stale notice is a success case, not an error.

## First-slice institution

Implement one local authority: reeve, clerk, two guards, law code, incident ledger, bounty ledger, notice board, money reserve, theft procedure, road-threat procedure, and proof/payment procedure.

## Anti-patterns

Global wanted meter. Instant guard knowledge. Bounty appears from nowhere. Report accepted without receiving actor or record. Court outcome decided by quest state. Guards who never sleep, eat, fear, or misunderstand.
