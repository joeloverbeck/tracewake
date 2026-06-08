# Testing, Observability, Diagnostics, and Review Artifacts

## Status

Live execution doctrine. Testing, debugging, observability, replay, actor-known traces, and anti-contamination checks are proof mechanisms, not support topics.

## Authority boundary

This document owns execution-level acceptance artifacts and diagnostic standards. It does not define implementation code or ticket lists.

## Depends on

- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

## Core rule

A test that proves plausible behavior while bypassing actor-known provenance is a bad test.

A diagnostic that says only that a test failed is a bad diagnostic.

## Required test families

| Family | Proof burden |
|---|---|
| Unit | Local invariants for event envelopes, validation, provenance, action definitions, context sealing, and diagnostics. |
| Integration | End-to-end pipeline from trigger/input to proposal, validation, event, projection, TUI/debug, replay. |
| Golden | Named fixture contracts with positive and negative outcomes. |
| Property/generative | Invariants over varied seeds, ordering, content, projections, and invalid data. |
| Static/anti-regression | Searchable guardrails for forbidden dispatch, forbidden fields, debug leakage, string-typed shortcuts, and branch-on-human privilege. |
| Replay | Deterministic rebuild from events and schema-compatible fixtures. |
| View-model | Embodied/debug separation and possession parity. |
| Content validation | Rejection of scripts, player privilege, culprit flags, hidden truth, and unproven cognition fields. |
| Certification audit | Human-readable gate report mapping code seams to doctrine. |

## Diagnostic standard

Every failure artifact must name:

- gate;
- layer;
- responsible component or data class;
- expected input source;
- actual input source;
- event/projection/checksum identifiers where available;
- actor-visible output, if any;
- debug-only output, if any;
- hidden truth excluded or leaked;
- replay divergence point, if any;
- remediation category.

## Responsible layers

Use these layer names consistently:

- `doctrine`;
- `content_schema`;
- `content_validation`;
- `fixture_contract`;
- `holder_known_context`;
- `candidate_generation`;
- `intention_lifecycle`;
- `method_selection`;
- `local_planning`;
- `proposal_construction`;
- `scheduler`;
- `action_validation`;
- `event_append`;
- `event_application`;
- `projection`;
- `replay`;
- `view_model`;
- `debug_quarantine`;
- `tui_input_binding`;
- `test_oracle`.

## Observability requirements

Debug and review artifacts must expose enough structure to audit without becoming gameplay authority:

- event log with causal ancestry;
- proposal log with origin and holder-known context ID;
- actor-known context contents and provenance;
- candidate/method/local-plan traces;
- hidden-truth audit comparison rows;
- stuck/failure diagnostics;
- content validation reports;
- replay rebuild and divergence reports;
- view-model filtering reports;
- no-human metrics;
- possession binding reports.

Debug output must be structurally separated from actor-visible output.

## Property and random testing

Randomized testing is allowed only with recorded seeds, scoped random streams, deterministic ordering, and reproducible failure artifacts. A failing generated case must be replayable by seed and input manifest.

Properties should include:

- replay equals live projection;
- actor-visible output is subset of holder-known context;
- debug truth does not alter proposal sequence;
- possession binding does not alter world rules;
- invalid content fails before runtime;
- scheduler never emits primitive actions directly;
- validator truth does not create actor knowledge without modeled events;
- no accepted event depends on wall-clock time or unordered iteration.

## Review artifact template

Every certification artifact must include:

1. Gate names reviewed.
2. Files/seams audited.
3. Foundation and architecture dependencies.
4. Positive fixture evidence.
5. Negative fixture evidence.
6. Replay evidence.
7. Actor-known provenance evidence.
8. Debug quarantine evidence.
9. Failure diagnostics with responsible layers.
10. Deferrals tied to named gates.
11. Archived-spec status statement.
12. Certification result: pass, fail, or scoped remediation.

## Central conformance gate `DIAG-CERT`

No phase or feature gate passes unless `DIAG-CERT` passes. This prevents tests from becoming shallow smoke checks.

`DIAG-CERT` passes only when failure reports are specific enough that a future implementation session can identify the layer to inspect without guessing.

## Forbidden test patterns

- Test passes because final state looks right while event ancestry is wrong.
- Test parses display strings to infer typed facts.
- Test uses debug truth as actor knowledge.
- Test drives scheduler shortcuts not available to normal actors.
- Test treats human command path as exempt from ordinary validation.
- Test asserts only that an actor waited, without modeled reason or stuck diagnostic.
- Test checks only happy path content and ignores forbidden fields.
- Test calls an archived spec sufficient evidence for live certification.
