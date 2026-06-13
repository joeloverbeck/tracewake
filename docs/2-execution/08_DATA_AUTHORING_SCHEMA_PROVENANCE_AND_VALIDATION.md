# Data Authoring, Schema, Provenance, and Validation

## Status

Live execution doctrine for authoring and validation. This document splits authoring/schema/provenance from golden fixture acceptance.

## Authority boundary

This document owns data-authoring acceptance and validation gates. It does not define final file syntax and does not certify current content code.

## Depends on

- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

## Core rule

Author causal possibility space. Do not author outcome chains.

Data may define actors, places, objects, affordances, routines, beliefs, traces, norms, roles, records, speech templates, pressures, fixtures, and boundary conditions. Data may not force plots, mark culprits, create player privilege, or smuggle omniscience into actors.

## Validation pipeline

Content must pass these gates before runtime:

1. Syntax validity.
2. Schema version and compatibility.
3. Stable deterministic identifiers.
4. Referential integrity.
5. Canonicalization determinism.
6. Provenance sufficiency for beliefs, records, routines, workplace/home assignments, expectations, and prehistory.
7. Action registry and affordance parity.
8. No outcome-chain authoring.
9. No player/human privilege fields.
10. No hidden-truth cognition fields.
11. No debug/omniscience fields in embodied data.
12. Fixture replay compatibility.
13. Writer-readable and programmer-actionable error reports.

Validation failures are deliverables. They must name the field, rule, layer, and reason.

## Provenance contract

Any data that can affect cognition, procedure, or view must carry source status:

- authored initial state;
- authored prehistory event;
- generated prehistory summary;
- in-world record;
- direct observation;
- testimony;
- rumor;
- routine assignment;
- institution procedure;
- boundary input;
- debug-only diagnostic.

Debug-only diagnostic is never valid provenance for actor cognition.

## Forbidden data forms

The following are invalid in live content schemas and fixtures, including aliases and clever renames:

- `player_character`;
- `player_memory`;
- `quest_state`;
- `objective_marker`;
- `culprit`;
- `stolen_flag`;
- `npc_knows_truth`;
- `debug_truth_for_actor`;
- `director_trigger`;
- `story_phase`;
- `spawn_when_player`;
- `reward_on_completion`;
- `guaranteed_clue`;
- `routine_success_script`;
- `institution_knows_truth`.

Keyword rejection is not sufficient. The schema must make these concepts unrepresentable or clearly invalid by structure.

## Authoring contracts by domain

| Domain | Required authoring | Forbidden shortcut |
|---|---|---|
| Actors | Bodies, homes or reason for lacking one, needs, durable concerns, routines, beliefs with sources. | Actor exists only as a player-facing NPC. |
| Places/routes | Connectivity, access, privacy, sensory conditions, known/unknown discoverability. | True map marker exposed to embodied view. |
| Items/containers | Possession, custody, ownership, visibility, access, traces, affordances. | Objective item location as actor knowledge. |
| Beliefs/claims | Proposition, stance, confidence, source, holder, time, privacy. | Truth flag copied into belief. |
| Routines | Applicability, steps, failures, interrupts, provenance, known targets. | Guaranteed daily script with no blockers. |
| Records | Physical/digital artifact, author, claims, access, status, provenance. | Record equals truth. |
| Institutions | Roles, powers, procedures, proof rules, resources, fallibility. | Norm violation triggers automatic enforcement. |
| Boundary inputs | Source, cadence, scope, local delivery event, replay visibility. | Regional story beat injection. |

## Schema-version and migration rule

A fixture or content package must declare schema version and compatibility. Replay must reject mismatched content unless a named migration/upcast path exists and produces auditable diagnostics.

Silent migration is forbidden.

## Data certification gate `DATA-CERT`

`DATA-CERT` is a phase-certification artifact label from the execution
sequence. It consumes canonical gate evidence; it is not a new canonical gate
code beyond `00_EXECUTION_INDEX_AND_AUTHORITY.md`.

`DATA-CERT` passes only when:

1. Content validation rejects forbidden fields and outcome-chain structures.
2. Every cognition-relevant authored belief, record, routine target, workplace, home, expectation, and prehistory fact has provenance.
3. Invalid content errors name the responsible rule and authoring layer.
4. Valid content is deterministic under repeated load and canonicalization.
5. Fixture manifests declare compatible doctrine/gate expectations.
6. Actor-visible data and debug-only data are structurally separated.
7. Golden fixture contracts in document `09` can be generated or checked from the content package.
8. Validation tests include adversarial renamed fields and nested/embedded forbidden forms.

## Review question

For every content field, ask:

> Does this field define causal machinery, or does it guarantee an outcome?

Fields that guarantee outcomes must be removed, converted into causal machinery, or rejected by validation.
