# Institutions, Households, Norms, Records, and Procedures

## Status

Authoritative architecture contract.

## Purpose / core rule

Institutions and households are fallible social machines. They act through records, roles, resources, jurisdiction, norms, public artifacts, reports, procedures, and institutional memory. They do not read hidden truth.

Institutional procedure uses a parallel holder-known transaction:

```text
institution trigger
 -> sealed institution-known context
 -> procedure candidate generation / role decision
 -> proposal or institutional action through shared validation/event pipeline
 -> records/notices/orders/sanctions/failures through modeled authority and resources
 -> public/actor/institution knowledge updates through modeled channels
 -> debug-visible truth comparison without leakage
```

## Authority owned

This document owns institution-known procedure, households, roles, norms, records, reports, notices, orders, sanctions, proof thresholds, powers, resources, bias/failure, jurisdiction, and institutional debug boundaries.

## Authority denied

Institutions may not auto-detect violations, auto-convict, auto-sanction, auto-correct records, or convert event-log truth into institutional knowledge. Households may not function as script triggers. Norms may not be enforcement scripts.

## Contract

### Institution-known context

Minimum institution context:

```text
institution_known_context:
  institution_id
  context_id
  trigger_id
  tick_or_window
  jurisdiction
  roles_available
  role_holders_known
  procedure_state
  records_known
  reports_received
  notices_posted
  public_artifacts
  resources_available
  permissions_powers_obligations
  evidence_thresholds
  known_conflicts_or_staleness
  institutional_memory
  provenance_edges
  context_hash
```

Every institutional decision must cite this context.

### Roles and powers

Roles are modeled social facts. A role may grant authority to receive a report, post a notice, inspect a property, summon a person, order restitution, levy a fine, or archive a record. The role does not grant omniscience and does not guarantee success.

Powers are constrained by:

- jurisdiction;
- procedure state;
- resource availability;
- evidence threshold;
- public legitimacy;
- role holder presence/knowledge;
- social resistance;
- record completeness;
- time and place.

### Norms

Norms classify obligations, prohibitions, permissions, constitutive facts, evidence thresholds, procedures, and sanctions. A norm can be violated without detection. Detection can occur without proof. Proof can exist without enforcement. Enforcement can be mistaken, biased, delayed, resisted, or reversed.

### Procedure chain

A procedure is an HTN-like institutional routine, not an outcome script.

Example missing-property intake:

```text
trigger: report_received(missing_coin_report)
context: town_watch knows report, reporter, claimed property, role clerk_on_duty
candidate: open intake record
method: record complaint and schedule review
proposal: create institutional record
validation: role authority, office open, record resource available
commit: record_created
feedback: receipt notice to reporter, institution memory update
```

Later stages may include review, interview, search request, watch notice, arbitration, restitution order, or closure. None is automatic.

### Records

Records are typed claim carriers. They include author/source, holder, visibility, creation event, amendment history, stale/expiry rules, and custody. A record can be false. Record correction requires events; it cannot silently mutate history.

### Notices and orders

Notices and orders are public or targeted artifacts with role authority, source record, issuer, scope, duration, visibility, and failure modes. A notice can be ignored, missed, stale, forged, or misunderstood. An order can fail due to lack of resources or social resistance.

### Sanctions

Sanctions are procedure outcomes that require modeled authority, evidence, role action, resources, and event commit. Sanction is not automatic consequence of violation. Mistaken sanctions are allowed and important.

### Households

Households are small institutions with memory, expectations, possessions, routines, duties, private records, access norms, and resource sharing. Household facts also require known context and provenance. A household cannot directly enforce outcomes; members act through ordinary actor proposals or household/institution procedures.

### Bias, error, and corruption

Institutions and households may misinterpret, ignore, overreact, discriminate, lose records, protect insiders, lack staff, or use stale rules. These are modeled states and procedures, not director choices.

## Data examples

Valid town-watch suspicion:

```text
institution_belief:
  holder: town_watch
  proposition: actor_mara_possible_witness_or_suspect
  stance: plausible
  source: report_event + prior public record + clerk inference rule
  confidence: low
  procedure_state: intake_review
```

Invalid institution omniscience:

```text
institution_belief:
  holder: town_watch
  proposition: actor_mara_stole_coin
  source: event_log_truth:item_moved_by_actor_mara
  procedure_state: sanction_now
```

## Required diagnostics / replay / TUI hooks

- Institution-known context packets must be replayable and inspectable.
- Procedure decisions must emit typed traces: candidate procedures, selected method, rejected methods, roles/resources used, evidence threshold, and failure reasons.
- Records/notices/orders/sanctions must be events or event-derived projections.
- Debug views may compare institutional belief and truth but must be non-diegetic.
- Embodied actor views see institutional artifacts only if modeled visibility allows.

## Acceptance implications

The missing-property proof may use a local household or watch process only if reports, records, and procedure steps are modeled. Acceptance fails if a watch system reads culprit truth, auto-posts a clue, or sanctions without institution-known procedure.

Adventure/bounty/expedition systems are later proofs. They must reuse this institution-known model; they cannot introduce quest boards as omniscient task generators.

## Anti-patterns

- “Crime event happened, so the watch knows it.”
- “Report equals proof.”
- “Proof equals sanction.”
- “Notice board means quest list.”
- “Household assignment table means every member knows all household facts.”
- “Institution records self-correct when truth changes.”
- “A sanction is a validator side effect.”

## Cross-document obligations

Institution procedures use holder-known contexts from document 03, action validation from document 04, claims/records from document 06, speech/report acts from document 07, ordinary-life property from document 09, incidents/leads from document 11, LOD institutions from document 12, and acceptance artifacts from document 13.
