# Institutions, Norms, Households, and Records

## Status

This document defines institutions as fallible social machines. It includes households as first-class domestic institutions and defines the first public institution as a small local authority.

Institutions are not omniscient enforcement scripts. They are people, roles, obligations, permissions, prohibitions, records, procedures, resources, jurisdictions, biases, office hours, and failures.

## Core rule

Institutions act from institutional knowledge, records, role duties, resources, relationships, and procedure. They do not read ground truth.

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

Violation, detection, suspicion, record, proof, and sanction are distinct.

## Household as domestic institution

Households are first-class from the beginning.

A household defines:

- members;
- domestic roles;
- shared and private property expectations;
- beds and storage;
- food access;
- domestic obligations;
- privacy boundaries;
- guest permissions;
- internal reputation;
- childcare/eldercare hooks if modeled;
- inheritance/future property rules;
- household memory and secrets;
- expectation of who should be home and when.

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

Household norms are ordinary-life mechanics, not decoration.

## Institution model

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
    - false_report_if_modeled_later
  office_hours:
    open: morning
    closed: night
  biases:
    status_bias: medium
    outsider_bias: medium
    kinship_bias: variable
```

This is enough for missing-property reports. Broader law is future pattern material.

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

A person can violate a role. Consequences occur only through observation, report, record, inference, or later discovery.

## Norms

Norms are explicit and typed.

Types:

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

Example:

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

The norm engine may classify possible violation for debug/procedure. That classification is not automatically known by guards or clerks.

## Constitutive norms and institutional facts

Some facts exist because institutions create them by procedure.

Examples:

- a report is received;
- a ledger entry exists;
- a notice is officially posted;
- a person is ordered to answer questions;
- funds are reserved for payment;
- a case is closed;
- property is placed in institutional custody.

Institutional facts have provenance and can be wrong, forged, contested, lost, ignored, or overturned.

```yaml
InstitutionalFact:
  kind: IncidentRecordOpened
  institution: reeves_office
  created_by_event: evt_ledger_entry_created
  authorized_by: role_clerk
  claims:
    - ItemMissing(coin_stack_01)
    - LastExpectedIn(strongbox_tomas)
  confidence: partial
  status: open
```

## Records as world artifacts

Records are durable artifacts.

They have:

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

Example:

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
    - ElenaHeardPossibleNoise
  status: open
  evidence_refs:
    - belief_tomas_strongbox_empty
    - hearsay_elena_noise
```

Records can be read, hidden, copied, forged, destroyed, misfiled, contradicted, or ignored.

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
 -> ledger entry written as physical record event
 -> institutional belief projection updates
 -> possible follow-up scheduled
```

A report is not accepted merely because a quest state needs it.

## Case lifecycle

Minimal missing-property case:

```text
report received
 -> ledger entry created
 -> clerk asks basic questions if time/authority
 -> evidence threshold checked from institutional knowledge
 -> record remains open, waiting, rejected, or referred
 -> guard/reeve may question if threshold/resources allow
 -> record can become stale, contradicted, amended, or closed
```

No first-slice court is required.

## Notice lifecycle

A notice is a public artifact.

```text
institution or person has reason to announce
 -> authority/procedure check
 -> structured claims written
 -> physical notice created
 -> notice posted at board
 -> readers observe and form beliefs
 -> notice may become stale, destroyed, forged, copied, ignored, removed, contradicted
```

A notice does not create an objective quest. It creates source-bound public information.

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

First slice needs local jurisdiction only. Borders, guild privilege, temple sanctuary, noble exemption, and inter-settlement conflict are future patterns.

## Bias and corruption

Bias and corruption are causal processes.

Bias may affect:

- credibility;
- priority;
- suspicion;
- evidence threshold;
- willingness to question;
- record wording;
- notice posting;
- sanction severity.

Corruption requires:

- offer or pressure;
- actor motive;
- acceptance/refusal;
- memory or record;
- trace/risk;
- later vulnerability.

Do not implement corruption as a hidden global modifier with no events.

## Procedure failure

Institutions can fail:

- office closed;
- clerk absent;
- guard hungry/tired/afraid;
- paper unavailable;
- funds unavailable;
- report not believed;
- wrong person questioned;
- record misfiled;
- notice left stale;
- witness intimidated;
- bias distorts priority;
- jurisdiction refused;
- procedure interrupted.

Failure is content. It should create events, beliefs, and consequences where meaningful.

## First public institution scope

Implement first:

- one reeve;
- one clerk;
- up to two guards if needed;
- office with hours;
- incident ledger;
- notice board;
- small funds or payment custody only if needed;
- property/theft norm;
- trespass/privacy norm;
- report action;
- simple questioning procedure;
- proof/payment procedure only after ordinary life works.

Do not center v1 on bounties, courts, guilds, temples, bandits, markets, or adventuring companies.

## Future institution patterns

Future institutions may include:

- guilds;
- temples;
- tavern/market institutions;
- courts;
- bandit groups;
- adventuring companies;
- regional authorities;
- households with inheritance and marriage;
- debt/credit networks.

They must use the same role/norm/record/procedure/fallibility architecture.

## Acceptance implications

Institution features must test:

- institution does not read ground truth;
- report requires actor, receiver, speech act, and record decision;
- record has artifact/provenance;
- violation/detection/suspicion/proof/sanction remain distinct;
- office hours and staff availability matter;
- bias/failure is causal and inspectable;
- household permissions affect actions;
- stale records/notices can remain wrong;
- TUI shows actor-known institutional claims;
- debug explains institutional knowledge vs truth.

## Anti-patterns

- Global wanted meter.
- Guards instantly know crimes.
- Report accepted without receiver.
- Ledger entry without writer or page.
- Court outcome from quest state.
- Bounty from nowhere.
- Payment from nowhere.
- Institution uses hidden true culprit.
- Household is just flavor text.
- Sanction without detection/procedure or exceptional authority event.
