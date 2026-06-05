# Institutions, Households, Norms, Records, and Procedures

## Status

This document defines institutions as fallible social machines, including households as first-class domestic institutions and the first public institution as a small local authority.

Institutions are not omniscient enforcement scripts. They are people, roles, obligations, permissions, prohibitions, records, procedures, resources, jurisdictions, biases, office hours, and failures.

## Core rule

Institutions act from institutional knowledge, records, role duties, resources, relationships, norms, and procedure. They do not read ground truth.

```text
violation may occur
 -> traces or observations may exist
 -> suspicion may form
 -> report may be made
 -> record may be opened
 -> evidence may be interpreted
 -> procedure may succeed or fail
 -> sanction may or may not occur
```

Violation, detection, suspicion, report, record, proof, and sanction remain distinct.

## Authority

This subsystem owns:

- household definitions and domestic authority;
- roles, obligations, permissions, prohibitions, constitutive norms, evidence thresholds, jurisdiction, and status;
- institutional records, reports, notices, orders, sanctions, and failures;
- procedures and procedure state;
- institutional belief/knowledge projections;
- bias and corruption as causal processes.

It is denied:

- omniscient crime detection;
- global wanted meters;
- sanction from hidden truth;
- records without authors/artifacts;
- notices as quest boards;
- procedure success because the player needs progress.

## Households as domestic institutions

A household defines:

- members;
- domestic roles;
- shared/private property expectations;
- beds and storage;
- food access;
- domestic obligations;
- privacy boundaries;
- guest permissions;
- internal reputation;
- household memory and secrets;
- expected presence and routines;
- childcare/eldercare/inheritance hooks where modeled.

Minimal first-slice household:

```yaml
Household:
  id: household_tomas
  members: [actor_tomas, actor_elena]
  dwelling: house_tomas
  domestic_roles:
    actor_tomas: head_or_member
    actor_elena: member
  shared_resources:
    food_storage: pantry_tomas
  private_storage:
    strongbox_tomas:
      permitted_users: [actor_tomas]
      known_expected_contents:
        actor_tomas: [coin_stack_01]
  privacy_norms:
    - private_bedroom_access
    - household_storage_access
  routine_expectations:
    sleep_places:
      actor_tomas: bed_tomas
      actor_elena: bed_tomas
```

Household norms are mechanics, not decoration.

## Public institution model

```yaml
Institution:
  id: reeves_office
  kind: local_authority
  jurisdiction:
    places: [village_core, surrounding_farms, north_road_near_village]
    people_scope: residents_and_visitors_under_local_norms
  roles: [reeve, clerk, guard]
  resources:
    money: small_reserve
    staff_time: limited
    guards_available: 0_to_2
    paper: finite
    office: reeves_office_building
  records:
    - incident_ledger
    - notice_board_register
    - property_dispute_notes
  norms:
    - property_theft
    - trespass_privacy
  office_hours:
    open: morning
    closed: night
  biases:
    status_bias: medium
    outsider_bias: medium
    kinship_bias: variable
```

This is enough for missing-property reports. Courts, guilds, temples, armies, and markets are future patterns.

## Roles

Roles confer powers, obligations, credibility, access, and prohibitions.

```yaml
RoleDefinition:
  id: clerk
  institution: reeves_office
  obligations:
    - receive_reports_during_office_hours
    - maintain_incident_ledger
    - post_authorized_notices
  permissions:
    - write_in_incident_ledger
    - ask_basic_questions_about_reports
    - access_office_records
  prohibitions:
    - alter_records_without_entry
    - disclose_private_reports_without_basis
  credibility_effects:
    report_receiver: high
    gossip_source: variable
```

A role-holder can violate role duties. Consequences require observation, record, report, inference, or later discovery.

## Norms

Norm types include:

- prohibition;
- obligation;
- permission;
- constitutive norm;
- status norm;
- procedure rule;
- sanction rule;
- evidence threshold;
- jurisdiction rule;
- privacy/access rule.

```yaml
Norm:
  id: property_theft
  kind: prohibition
  action_pattern: TakePropertyWithoutPermission
  conditions:
    - item_has_recognized_owner
    - actor_lacks_permission
    - no_emergency_exception
  possible_violation_event: PropertyViolationClassified
  detection_requirements:
    - observation OR trace OR admission OR possession_evidence OR report
  institutional_thresholds:
    open_incident: low
    question_named_suspect: medium
    search_private_space: high_or_emergency
    sanction: high
```

The norm engine may classify possible violation for procedure/debug. Guards and clerks know only what channels give them.

## Constitutive norms and institutional facts

Some facts exist because institutions create them by procedure:

- a report is received;
- a ledger entry exists;
- a notice is officially posted;
- a person is ordered to answer questions;
- property enters institutional custody;
- funds are reserved for payment;
- a case is closed.

Institutional facts have provenance and can be wrong, forged, contested, lost, ignored, or overturned.

```yaml
InstitutionalFact:
  kind: IncidentRecordOpened
  institution: reeves_office
  created_by_event: evt_ledger_entry_created
  authorized_by: role_clerk
  claims:
    - ItemMissing(coin_stack_01)
    - LastExpectedIn(coin_stack_01, strongbox_tomas)
  confidence: partial
  status: open
```

## Records as artifacts

Records have:

- author;
- issuer/authority;
- physical or digital location;
- structured claims;
- creation event;
- amendments;
- readers;
- copies;
- destruction/forgery events;
- institutional status;
- access permissions;
- stale risk.

```yaml
IncidentLedgerEntry:
  id: ledger_entry_missing_coins_01
  physical_artifact: ledger_page_43
  reporter: actor_tomas
  received_by: actor_anna_clerk
  institution: reeves_office
  received_at: 142-08-12T09:12
  claims:
    - ItemMissing(coin_stack_01)
    - LastExpectedIn(coin_stack_01, strongbox_tomas)
    - NoiseHeardNearPlace(room_tomas_bedroom)
  status: open
  evidence_refs:
    - belief_tomas_strongbox_empty
    - hearsay_elena_noise
```

Records can be read, hidden, copied, forged, destroyed, misfiled, contradicted, ignored, or cited.

## Report lifecycle

Minimal report lifecycle:

```text
actor forms report intention
 -> actor travels/waits/reaches receiver
 -> report speech act proposed
 -> receiver attention/social/procedure checks
 -> report speech event committed
 -> receiver interprets claims
 -> institution decides whether to record
 -> ledger entry written as artifact event
 -> institutional belief projection updates
 -> possible follow-up scheduled
```

A report is not accepted because a quest state needs it.

## Case lifecycle

Minimal missing-property case:

```text
report received
 -> ledger entry created
 -> clerk asks basic questions if time/authority
 -> evidence threshold checked from institutional knowledge
 -> record remains open, waiting, rejected, referred, contradicted, or closed
 -> guard/reeve may question if threshold/resources allow
 -> notice may or may not be posted
```

No first-slice court is required.

## Notice lifecycle

A notice is a public artifact, not a quest board.

```text
institution/person has reason to announce
 -> authority/procedure check
 -> structured claims written
 -> physical notice created
 -> notice posted at board
 -> readers observe and form beliefs
 -> notice becomes stale, destroyed, forged, copied, ignored, removed, or contradicted
```

A notice creates source-bound public information. It does not create objective progress or targets.

## Jurisdiction

Jurisdiction affects:

- whether an institution cares;
- whether a role has authority;
- what norms apply;
- what record is valid;
- what procedure is available;
- whose testimony is weighted;
- whether evidence is admissible;
- whether sanctions can be enforced;
- whether another institution contests.

First slice needs local jurisdiction only.

## Bias and corruption

Bias may affect credibility, priority, suspicion, evidence threshold, willingness to question, record wording, notice posting, and sanction severity.

Corruption requires:

- offer or pressure;
- actor motive;
- acceptance/refusal;
- memory or record;
- trace/risk;
- later vulnerability.

Do not implement corruption as hidden global modifier with no events.

## Procedure failure

Institutions can fail because:

- office closed;
- clerk absent;
- guard hungry/tired/afraid;
- paper unavailable;
- funds unavailable;
- report not believed;
- wrong person questioned;
- record misfiled;
- notice stale;
- witness intimidated;
- bias distorts priority;
- jurisdiction refused;
- procedure interrupted.

Failure is content and should create events, beliefs, and consequences where meaningful.

## First public institution scope

Implement first:

- one reeve;
- one clerk;
- up to two guards if needed;
- office with hours;
- incident ledger;
- notice board;
- property/theft norm;
- trespass/privacy norm;
- report action;
- simple questioning procedure;
- proof/payment procedure only after ordinary life works.

Do not center v1 on bounties, courts, guilds, temples, bandits, markets, or adventuring companies.

## Acceptance implications

Institution features must test:

- no ground-truth reads;
- report requires actor, receiver, speech act, and record decision;
- record has artifact/provenance;
- violation/detection/suspicion/proof/sanction separation;
- office hours and staff availability;
- bias/failure as causal and inspectable;
- household permissions affect actions;
- stale records/notices can remain wrong;
- TUI shows actor-known institutional claims;
- debug explains institutional knowledge versus truth.

## Anti-patterns

- Global wanted meter.
- Guards instantly know crimes.
- Report accepted without receiver.
- Ledger entry without writer or page.
- Court outcome from quest state.
- Bounty from nowhere.
- Payment from nowhere.
- Institution uses hidden true culprit.
- Household is flavor text.
- Sanction without detection/procedure or explicit exceptional authority event.
