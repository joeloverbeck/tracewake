# Post-0008 Baseline, Archived Spec Status, and Code Audit Boundary

## Status

Live execution doctrine. This document defines the current baseline after archived specs `0005`, `0006`, `0007`, and `0008`.

## Authority boundary

This document owns the distinction between historical implementation evidence and live execution doctrine. It defines what future code audit must prove before current implementation can be treated as aligned.

It does not certify code and does not reopen archived specs as live requirements.

## Depends on

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

## Baseline posture

Historical implementation has landed through archived specs `0005` through `0008`, but the implementation is not certified under the post-overhaul foundation, architecture, and execution doctrine.

The current code may inform audit targets and staleness implications. It must not override foundation or architecture. A future audit must compare code against this replacement execution layer.

## Archived specs are historical evidence only

| Archived spec | Historical value | Not live authority for |
|---|---|---|
| `0005` | Introduced Phase 3A needs, routines, intentions, planner traces, no-human day, and ordinary actions. | Certifying the implementation, preserving phase-local shortcuts, or treating Phase 3A as complete under the new doctrine. |
| `0006` | Exposed that ordinary-life work needed stronger actor-known planning, lifecycle, replay, and no-human integration. | Treating first hardening fixes as sufficient certification. |
| `0007` | Exposed further gaps in integrated no-human proof, live agent state, string-heavy diagnostics, and hidden-truth construction resistance. | Treating second hardening as proof that all anti-contamination gates are solved. |
| `0008` | Exposed the crucial anti-contamination problems: actor-known transaction shape, planner inputs, forged/stale proposal parameters, marker actions, friendly tests, and overclaiming docs. | Treating an accepted hardening spec as live doctrine where it conflicts with the replacement foundation, architecture, or execution layer. |

No future session may cite archived specs or tickets as current doctrine when they conflict with this replacement set.

## Durable promotions from specs `0005` through `0008`

The following ideas are promoted into live execution doctrine because they are consistent with foundation and architecture:

- Ordinary life is a proof mechanism, not NPC decoration.
- Needs are pressures, not direct action scripts.
- Routines are defeasible HTN-like causal machinery, not authored outcome chains.
- Intentions are durable commitments with typed lifecycle transitions.
- No-human advancement must use ordinary actor decision transactions and ordinary action proposals.
- `continue_routine` must not count as behavioral progress unless it leads to a concrete ordinary action, a modeled wait, or a typed stuck/failure record.
- Actor-known planning context must have provenance and must not be a thin wrapper over truth.
- Planner traces, rejected candidates, stuck diagnostics, and replay artifacts are acceptance evidence.
- Content validation must reject hidden player privilege, culprit flags, quest state, outcome scripts, and debug/omniscience fields.
- Tests must include adversarial negative gates, not only friendly golden paths.
- Documentation must not overclaim implementation status.

## Historical details not promoted

The following remain historical implementation detail unless a future certified spec reintroduces them under live authority:

- Exact Phase 3A file names, structs, function names, ticket IDs, or fixture implementations.
- Status language that says a phase has landed without a post-overhaul certification artifact.
- Any code path tolerated by a past spec because it was adequate for that spec's narrower audit mode.
- Any string-prefix, substring, display-label, or debug-string behavior used as a substitute for typed provenance or typed diagnostics.
- Any fixture shape that proves only the friendly path while leaving hidden-truth planning possible.

## Post-0008 baseline certification gate: `P0-CERT`

Before Phase 4, second-proof work, LLM surfaces, travel, regional scale, or new gameplay systems may proceed, a code audit must produce a `P0-CERT` artifact.

The `P0-CERT` artifact must prove:

1. All world-affecting behavior enters through proposal, validation, event append, application, projection, and replay boundaries.
2. Autonomous actor decisions use sealed actor-known contexts with provenance, not validation truth or debug truth.
3. Human-origin commands bind to ordinary actors and share the same action rules as autonomous actors.
4. Possession never resets needs, intentions, memories, routines, or access to world facts.
5. Scheduler paths cannot emit primitive world actions from raw truth, routine labels, need thresholds, or fixture tables.
6. Validation truth may accept/reject/mutate through events, but may not propose fallback plans or actor-visible hidden facts.
7. Debug surfaces are non-diegetic and cannot feed embodied affordances, actor decisions, records, institutions, or tests that masquerade as player knowledge.
8. Golden fixtures include adversarial hidden-truth, no-human, possession, replay, view-model, content-validation, and direct-dispatch rejection cases.
9. Failure diagnostics name the responsible layer: authoring, schema validation, actor-known context, candidate generation, planning, proposal construction, scheduling, action validation, event application, projection, TUI rendering, replay, or debug quarantine.
10. Archived specs and tickets are cited only as history.

## Code audit boundary

The next audit must be a code audit, not a documentation assertion. It must examine implementation seams including scheduler, actor-known context builders, transaction modules, planners, action pipeline, event application, state/projections, replay, TUI/debug panels, content schema, validation, and fixtures.

The audit may conclude that some current code already satisfies gates. It may also conclude that code historically landed under specs `0005` through `0008` must be corrected before expansion. Both conclusions must be evidence-based.

## Acceptance gates for future specs

A future implementation spec is admissible only if it includes a baseline statement with one of these exact outcomes:

- `P0-CERT passed`: the spec cites the certification artifact and names the gates it consumes.
- `P0-CERT scoped remediation`: the spec's only purpose is to repair a certification failure.
- `P0-CERT not applicable`: allowed only for non-live documentation or reference changes that cannot affect simulation, tests, fixtures, debug, or authoring.

Any other posture is phase-local loophole language and must be rejected.
