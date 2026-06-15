# Incidents, Leads, Notices, and Story-Sifting Projections

## Status

Authoritative architecture contract.

## Purpose / core rule

Tracewake rejects quest ontology. Incidents, leads, reports, obligations, notices, contracts, rumors, and story summaries are world artifacts or observer projections. They do not spawn truth, repair pacing, select culprits, guarantee rewards, or wait for the player.

Story is observed after causality, not directed before it.

## Authority owned

This document owns questless incident modeling, leads, notices, requests, reports, bounties/contracts as later institutional artifacts, story-sifting projections, observer summaries, and second-proof boundaries.

## Authority denied

No incident system may function as a quest manager, drama director, clue spawner, reward spawner, authored outcome chain, or hidden pacing controller.

## Contract

### Incident

An incident is a projection or record of causally related events and claims. It may group missing property, injury, argument, fire, debt, rumor, travel delay, institutional report, or unexplained absence.

Incident grouping is not authority. The underlying events and holder-known claims remain primary.

Minimum incident projection:

```text
incident_projection:
  incident_id
  source_event_ids
  involved_claims
  affected_holders
  public_or_private_visibility
  current_known_status_by_holder
  unresolved_questions
  possible_leads_by_holder
  debug_truth_comparison_optional
```

### Lead

A lead is a source-bound attention direction. It can suggest ordinary next actions available to a holder, such as check a known container, ask a known witness, report a missing property, read a public notice, revisit a stale record, or inspect a visible trace.

Lead minimum:

```text
lead:
  lead_id
  holder_id_or_projection_scope
  source_claim_or_event
  suggested_attention
  allowed_action_families
  confidence
  staleness
  forbidden_truth_audit
```

A lead must not name the true solution unless the holder has modeled provenance for that claim.

Lead and notebook labels such as stale, urgent, overdue, recently seen, old
report, or no longer useful are source-bound projections over holder-known
temporal claims and records. Story-sifting may compute observer-only temporal
summaries for review, but it may not create actor-known urgency, quest
priority, or diegetic proof.

### Notice

A notice is a public or targeted artifact. It has author/issuer, posting event, surface text, typed claims, visibility, expiry/staleness, and procedure source. Notices may be missed, stale, false, forged, or contested.

Notices are not quests. A bounty notice in a later phase is an institutional artifact with claims, issuer authority, conditions, proof rules, and payment resources. It does not guarantee that the target exists, the poster is truthful, or the reward will be paid.

### Request / contract / bounty

Requests and contracts are structured social/institutional artifacts. They may create obligations or incentives if accepted through ordinary speech/action/institution procedure. They may be ignored, misunderstood, declined, failed, renegotiated, or disputed.

They cannot mutate state by completion flags. Fulfillment is inferred from events/proof under modeled procedure.

### Story sifting

Story sifting is observer/projection only. It may:

- group events into a readable chain;
- highlight contradictions;
- summarize a no-human day;
- identify unresolved questions;
- compare holder beliefs and truth in debug mode;
- generate recaps for humans.

Story sifting may not:

- spawn events;
- choose actors' next goals;
- repair pacing;
- reveal hidden truth in embodied mode;
- mark quest completion;
- distribute rewards;
- create diegetic evidence, clues, proof, records, sanctions, rewards, or
  action reasons.

Story sifting may produce observer-only retrospective review evidence under
the one-way artifact contract in
`13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`: reproducible
from event/projection input, carrying event-log ancestry, and structurally
quarantined from cognition, schedulers, validators, and world mutation. That
observer-only evidence is not a holder-known or institution-known artifact and
does not weaken the ban on sifter-created in-world evidence.

### Missing-property first proof

The first proof uses incidents/leads only as projections of ordinary property expectation, search, absence, belief, report, and optional institution intake. It must not become a detective quest.

### Second-proof boundary

Road threats, regional notices, bounties, expeditions, monsters, travel parties, and adventure content are later. When introduced, they must use ordinary actors, institutions, notices, records, travel, resources, proof, and LOD ancestry. They may not bypass the missing-property ordinary-life substrate.

## Required diagnostics / replay / TUI hooks

- Incident/lead projections must list source event IDs and holder visibility.
- TUI notebooks may show actor-known leads, not debug solution trails.
- Debug story views may show truth comparison with non-diegetic markers.
- Story sifter output must be reproducible from event/projection input.
- Acceptance tests must include adversarial hidden truth absent from actor-known leads.

## Acceptance implications

A test fails if an incident creates a clue, culprit, reward, or next action without events and holder-known provenance. A story summary that sounds correct cannot substitute for typed event and claim evidence. Observer-only review evidence may support acceptance only under document 13's one-way ancestry-bound contract; it may not become diegetic evidence, a validator input, a scheduler input, or an action reason.

## Anti-patterns

- “Missing coin starts quest_missing_coin.”
- “Lead points to next required step.”
- “Notice board is a quest hub.”
- “Bounty target is guaranteed guilty.”
- “Reward spawns on completion.”
- “AI director raises tension by creating road danger.”
- “Story sifter marks the culprit for debug, and embodied mode simply hides the field.”

## Cross-document obligations

This document uses event/replay from document 02, holder-known context from document 03, action/speech/institution pathways from documents 04, 07, and 08, ordinary-life proof from document 09, TUI notebooks from document 10, LOD ancestry from document 12, and acceptance gates from document 13.
