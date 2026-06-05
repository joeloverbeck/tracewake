# Social Institutions and Law

## Core claim

Institutions should be simulated actors made of people, roles, procedures, records, resources, jurisdictions, norms, and failures.

A town guard is not an omniscient enforcement script. A guard is a person occupying an institutional role, acting from reports, orders, beliefs, incentives, fatigue, fear, bias, corruption, and available time.

## Institution model

```yaml
Institution:
  id: inst_reeves_office
  kind: local_authority
  jurisdiction:
    locations: [village, north_road, mill, farms]
    people: [residents, visitors_under_town_law]
  roles:
    - reeve
    - clerk
    - guard
    - tax_collector
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
    - court_judgments
  norms_ref: local_law_v1
  procedures_ref: reeves_office_procedures_v1
  biases:
    status_bias: high
    outsider_bias: medium
    kinship_bias: medium
```

## Roles

Roles confer permissions, obligations, prohibitions, and credibility.

```yaml
Role:
  id: role_guard
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
  credibility_modifiers:
    testimony_weight: 0.2
```

A person can violate a role. Violation creates consequences only if detected or later discovered.

## Norms

Norms should be explicit.

```yaml
Norm:
  id: norm_no_theft
  kind: prohibition
  subject_scope: all_people_under_jurisdiction
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

Types:

- prohibition: do not steal;
- obligation: guards must respond to reports;
- permission: reeve may authorize search;
- constitutive norm: a signed record counts as a contract;
- status norm: nobles have privileges;
- procedural norm: conviction requires testimony or evidence.

## Violation is not detection

An actor can violate a norm without anyone knowing.

Pipeline:

```text
Action occurs
  -> norm engine can classify it as violation in ground-truth analysis
  -> traces/observations may exist
  -> agents may form suspicion
  -> report may be made
  -> institution may open case
  -> evidence may or may not satisfy procedure
  -> sanction may or may not occur
```

Ground-truth violation should be available for debug/replay. Institutional action should depend on institutional knowledge.

## Records

Institutional records are durable world artifacts.

Examples:

```yaml
IncidentReport:
  id: report_223
  institution: inst_reeves_office
  reporter: actor_tomas
  received_by: actor_anna_clerk
  time_received: day_143_09_12
  claims:
    - item_coin_stack_01 missing from strongbox
    - last_seen day_142_evening
    - possible night noise heard by Elena
  confidence: 0.48
  status: open
```

```yaml
BountyRecord:
  id: bounty_07
  threat: incident_bandit_attacks_north_road
  authorized_by: actor_reeve
  amount: 80
  proof_required:
    - bandit_leader_token OR witness_confirmation OR recovered_stolen_goods
  notice_entity: notice_bandits_07
  status: posted
```

## Bounty lifecycle

The bounty should be a causal chain.

```text
1. Bandits attack traveler.
2. Survivor or witness observes attack.
3. Survivor reaches village.
4. Survivor reports to authority.
5. Clerk records report.
6. Authority assesses threat.
7. Authority chooses response: patrol, ignore, negotiate, bounty.
8. Funds are reserved.
9. Clerk writes notice.
10. Notice is physically posted.
11. Characters read notice if they pass board and choose/are able to read it.
12. Someone acts on notice.
13. Proof is presented.
14. Institution verifies or disputes.
15. Payment, refusal, fraud case, or stale closure follows.
```

At no point is this a quest primitive.

## Case lifecycle

For theft:

```text
1. Owner discovers missing property.
2. Owner decides whether to report.
3. Clerk receives report.
4. Case record created.
5. Guard assigned if priority sufficient.
6. Guard inspects location if permitted.
7. Guard collects observations and witness statements.
8. Guard updates hypotheses.
9. Suspect is questioned, monitored, searched, ignored, or detained.
10. Property may be recovered, case may go stale, wrong person may be punished, or culprit may escape.
```

## Jurisdiction

Jurisdiction matters.

Questions:

- Where did the event happen?
- Who has authority there?
- Is the victim protected by that authority?
- Is the suspect a local, outsider, noble, guild member, priest, guard, or outlaw?
- Does the institution have resources to act?
- Is there political cost?

This creates gameplay:

- criminals exploit borders;
- guards refuse cases outside jurisdiction;
- guilds handle internal matters;
- nobles avoid ordinary courts;
- villages hire outsiders when under-resourced.

## Bias and corruption

Bias should be systemic, not random flavor.

Inputs:

- class/status;
- kinship;
- species/culture/religion if setting uses them;
- wealth;
- reputation;
- prior crimes;
- bribes;
- fear of retaliation;
- political pressure;
- personal relationships.

Effects:

- report credibility;
- response priority;
- evidence threshold;
- punishment severity;
- chance of record tampering;
- willingness to post notice.

## Institutions as agents

Institutions need goals:

- maintain order;
- collect taxes;
- protect trade;
- preserve legitimacy;
- avoid scandal;
- satisfy superior authority;
- conserve resources;
- protect favored groups;
- punish enemies.

These goals are enacted by people, not by magic.

## Social institutions beyond law

### Household

- shared property;
- kin obligations;
- domestic routines;
- inheritance;
- shelter permission;
- family reputation.

### Guild

- work permissions;
- training;
- contracts;
- quality norms;
- sanctions;
- monopolies.

### Temple

- moral authority;
- ritual schedule;
- confessions;
- sanctuary;
- charity;
- gossip laundering.

### Tavern

- rumor exchange;
- hiring;
- social status;
- debt;
- fights;
- witnesses.

### Market

- prices;
- supply;
- reputation;
- theft;
- contracts;
- public observation.

## Institutional delays

Every institutional process should have delay.

Delay creates:

- stale bounties;
- slow investigations;
- corrupt shortcuts;
- panic before response;
- opportunities for player interference;
- believable bureaucracy.

Example delay model:

```yaml
ProcedureStep:
  kind: WriteNotice
  actor_role: clerk
  required_resources: [paper, ink]
  duration: 20 minutes
  queue_priority: medium
  interruption_policy: can_pause
```

## First-slice institution

For the vertical slice, implement one local authority:

- reeve;
- clerk;
- two guards;
- simple law code;
- incident ledger;
- bounty ledger;
- notice board;
- jail cell optional;
- money reserve;
- investigation procedure;
- road-threat procedure;
- theft report procedure.

## Anti-patterns

- Global wanted meter.
- Instant guard knowledge.
- Bounty appears from nowhere.
- Report accepted without a receiving actor or record.
- Court outcome decided by quest state.
- Guards who do not sleep, eat, fear, or misunderstand.
- Law that applies identically to every class/status unless the fiction explicitly says so and the system models why.
