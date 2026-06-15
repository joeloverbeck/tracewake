# Holder-Known Contexts, Truth Firewall, and Provenance

## Status

Authoritative architecture contract and organizing spine for cognition/procedure boundaries.

## Purpose / core rule

Truth may validate actions, resolve consequences, mutate authoritative state, generate modeled observations, and support non-diegetic debug comparison. Truth may not plan, suggest, interpret, select, narratively repair, or leak into a holder's cognition/procedure except through modeled knowledge channels.

The canonical split is:

```text
holder-known context -> proposal / cognition / procedure decision
authoritative truth -> validation / resolution / physical mutation / debug comparison
modeled feedback -> future holder knowledge
```

A holder is any entity or process that makes cognition-like or procedure-like decisions: actor, institution, household, role office, speaker, listener, embodied viewer, TUI affordance selector, lead interpreter, LOD promotion recipient, or regional procedure owner.

## Authority owned

This document owns the structural requirement that every decision boundary receives a sealed holder-known context with provenance. It defines the truth firewall, context packet minimums, allowed/forbidden context sources, debug quarantine, and contamination review.

## Authority denied

This document does not say actors are never wrong. It requires the opposite: actors, institutions, records, rumors, and memories may be wrong, stale, biased, incomplete, or contested. It also does not deny validators access to truth. Validators need truth; planners do not.

## Contract

### Holder-known context packet

A decision-facing context packet must be built before proposal/cognition/procedure selection and then sealed.

Minimum shape:

```text
holder_known_context:
  context_id
  holder_id
  holder_kind
  tick_or_window
  trigger_id
  source_event_range
  content_manifest_id
  projection_versions
  claims[]
  observations[]
  memories[]
  records_or_artifacts[]
  roles_and_permissions[]
  resources_and_constraints[]
  procedure_or_intention_state[]
  uncertainty_and_staleness[]
  explicit_unknowns[]
  provenance_edges[]
  forbidden_truth_audit
  context_hash
```

Every item used by candidate generation, method selection, local planning, speech interpretation, institutional procedure, view-model affordance selection, lead interpretation, or LOD promotion must be addressable inside this packet.

### Temporal claims in holder-known contexts

Any temporal input used by cognition, procedure selection, affordance selection, speech interpretation, lead interpretation, view-model construction, or LOD promotion must be addressable inside the holder-known or institution-known context with fact-kind-appropriate provenance. Temporal status is a source-backed claim or procedure state, not a display label, raw truth-clock read, or debug comparison.

Temporal claims may concern asserted or inferred event timing, acquisition timing, last verification, record or procedure timing, validity or due windows, stale risk, contradiction status, or source lineage. The packet must preserve enough ancestry to replay why the holder may use the temporal premise now, and must fail closed when the premise is missing, forbidden-source, dangling, or only available as validator/debug truth.

### Projection freshness

Projection-backed holder knowledge uses one freshness rule across no-human
cognition and embodied affordance views. Knowledge from the actor's current
perception window may surface as `observed_now`; older usable knowledge remains
available only as remembered belief carrying the original source timing and
provenance. A consumer may not restamp an aged projection record as a current
observation merely because it is selected for a later decision.

### Provenance sufficiency

An input to cognition, procedure selection, view-model affordance selection,
lead interpretation, or LOD promotion is provenance-sufficient only when it
cites at least one modeled acquisition or derivation route appropriate to the
asserted fact kind. Valid routes include direct observation, contact/search
events, absence observations, memory of a prior modeled source,
speech/testimony, records or public artifacts, routine or role assignment,
institutional-procedure state, LOD summary events with ancestry, or an explicit
unknown/unverified placeholder.

A source label, boolean, display sentence, fixture name, branch name, test name,
debug comparison, validator detail, or raw physical-truth lookup is not
provenance. Derived facts preserve lineage: the cited source must be enough to
replay and debug why this holder, institution, or projection may use the fact
now.

Missing, empty, dangling, wrong-kind, ambiguous, or forbidden-source provenance
fails closed before action-relevant use. The failure may be exposed to debug as
a non-diegetic artifact, but it may not become holder-known or
institution-known justification. Institution-known contexts are not exempt from
this rule.

### Provenance classes

Allowed provenance classes include:

- direct observation;
- touch/search result;
- sound or other uncertain observation;
- absence observation generated by modeled search;
- memory derived from prior observation or testimony;
- testimony/speech act from a modeled speaker;
- written record or public artifact with author/source and visibility;
- household or institutional record with role authority and creation event;
- routine/schedule/assignment known to the holder through modeled source;
- role/jurisdiction/procedure state known to an institution through modeled records;
- public notice, rumor, or lead with posting/transmission event;
- LOD summary event with promotion ancestry;
- explicit unknown / not observed / unverified placeholder.

Forbidden context sources include:

- physical truth with no modeled source;
- fixture truth not represented as holder knowledge;
- debug omniscience;
- validator internals;
- hidden culprit/outcome fields;
- raw route/workplace/food/sleep tables used directly for cognition;
- story-sifter summaries used as facts;
- LLM-generated facts not validated and modeled;
- projection completeness assumptions such as “empty projection means safe.”

### Context sealing

After sealing, a context packet is immutable for that decision. Validation failure may generate new modeled feedback, but it cannot retroactively add truth to the failed decision.

If an actor proposes to eat known stew and validation discovers the stew is gone, the validator may reject or emit modeled observations/failures. The planner may not instantly switch to the true hidden pantry unless the actor learns about it through modeled evidence.

### Actor-known formula

```text
trigger
 -> sealed actor-known context with provenance
 -> needs / commitments / intentions / routines / obligations
 -> candidate generation
 -> intention lifecycle
 -> method selection
 -> local planning
 -> proposal ancestry
 -> validation truth
 -> feedback / observations / belief updates
 -> diagnostics / replay / TUI
```

Actors can be resourceful only within what they know or can ordinarily discover. “Search locally” is valid if the actor knows a search surface, has a reason, and proposes ordinary search actions. “Go to hidden food because it exists” is invalid.

### Institution-known formula

```text
institution trigger
 -> sealed institution-known context
 -> records / reports / roles / jurisdiction / resources / procedure state / public artifacts
 -> procedure candidate generation or role decision
 -> ordinary or institutional proposal
 -> validation / event commit / rejection
 -> records / notices / orders / sanctions / failures through modeled authority
 -> public, actor, and institution knowledge updates through modeled channels
 -> debug truth comparison without leakage
```

Institutions are not omniscient. They are record-using, role-bound, resource-limited, biased, fallible social machines.

### TUI affordance formula

Embodied affordance menus are holder-known surfaces. They must be generated from the bound actor's visible/perceived/known context plus ordinary action definitions, not debug truth. Disabled actions may expose actor-legible reasons only.

Debug affordance views may compare truth, but they must be structurally separate and visibly non-diegetic.

### Speech and listener contexts

Speech acts have at least two holder-known contexts:

- speaker-known context: what the speaker can assert, ask, report, conceal, lie about, or misunderstand;
- listener-known context: how the listener interprets, trusts, doubts, remembers, or mishears the act.

Speech is not a magical fact transfer. It is an event over typed claims with source, channel, confidence, and possible deception/error.

### LOD promotion contexts

Low-LOD summary processes may maintain aggregate truth. When an actor, institution, record, or incident is promoted to higher detail, the promoted holder receives only modeled knowledge derived from summary events, public artifacts, travel records, reports, memories, or other explicit ancestry. Aggregate truth cannot become personal knowledge by proximity.

## Data examples

Valid actor-known fact:

```text
claim: food_supply_accessible(food_stew_home_tomas)
holder: actor_tomas
source: observation:event_obs_home_table_food
tick: 42
confidence: high
stale_after: 60
used_by: candidate_goal_eat
```

Invalid actor-known fact:

```text
claim: food_supply_accessible(food_hidden_pantry_mara)
holder: actor_tomas
source: physical_truth
used_by: candidate_goal_eat
```

Valid institution-known fact:

```text
claim: report_received(missing_coin_report)
holder: town_watch
source: speech_act:event_tomas_reports_missing_coin
role_receiver: watch_clerk
procedure_state: intake_opened
```

Invalid institution fact:

```text
claim: actor_mara_stole_coin
holder: town_watch
source: debug_truth
procedure_state: sanction_ready
```

## Required diagnostics / replay / TUI hooks

- Context packets must have stable IDs and context hashes.
- Decision traces must list context ID, provenance classes used, selected candidates, rejected candidates, and hidden-truth audit result.
- Stuck diagnostics must distinguish actor-known blocker, validation truth blocker, and debug-only details.
- TUI embodied views must expose context identity or equivalent audit reference.
- Debug views may show truth comparison only with non-diegetic markers.
- Acceptance tests must include adversarial truth present in authoritative state but absent from holder-known context.

## Acceptance implications

A run fails if the actor reaches the true food, workplace, bed, route, clue, witness, suspect, or sanction target without that target existing in the relevant holder-known context with provenance. Plausible behavior is not enough.

## Anti-patterns

- “The actor knows their workplace because the fixture assigned it.”
- “The institution knows the suspect because the event log contains the theft.”
- “The TUI can list every action and hide impossible ones.”
- “The debug panel is only visible to testers, so it can reuse embodied view code.”
- “A summary event says caravan arrived, so every traveler remembers the exact regional truth.”
- “The validator rejected the action, so the planner can use the debug reason next tick.”

## Cross-document obligations

This document constrains all planning/procedure systems in documents 04 through 12 and acceptance requirements in document 13.
