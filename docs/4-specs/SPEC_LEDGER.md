# Spec Ledger

This ledger records the active spec-layer package set and the status of archived implementation specs. It is a navigation and source-discipline document, not product doctrine.

## Authority posture

Specs are subordinate to the live tiers in this order:

1. `docs/0-foundation/`
2. `docs/1-architecture/`
3. `docs/2-execution/`
4. `docs/3-reference/`
5. `docs/4-specs/`

A spec may operationalize higher-tier doctrine. It may not amend constitutional invariants, replace architecture, define gate semantics, weaken execution gates, or promote archived history into certification.

## Source discipline

- A commit hash named inside a spec is audit/spec provenance only unless a higher-tier document says otherwise.
- A manifest is path inventory only.
- Branch names, default-branch lookups, connector namespace labels, repository metadata, and code-search snippets are not proof of target-commit content.
- Future exact-commit audits must fetch by exact file URL or by a supplied repository export at the target commit.
- Stale baseline strings in historical specs must not be adopted as current product doctrine.

## Active specs

| Spec | File | Status | Admissibility posture | Notes |
|---|---|---|---|---|
| `0001` | `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` | Live first-proof ontology and fixture contract, realigned after the upper-tier overhaul. | `P0-CERT not applicable` | Documentation-only realignment. It preserves the implemented village/fixture contract surface and cross-references gates without defining them. |

## Removed live companions

The previous live-tier companion-amendment material was removed rather than retained because the completed upper-tier overhaul falsified its main conclusion. The research notes were likewise removed because they were stale provenance for that obsolete conclusion; useful doctrine now lives in the higher tiers and reference layer.

No replacement companion file is kept for symmetry.

## Archived implementation specs

Archived specs remain history. They may explain why the upper-tier overhaul happened, but they are not live authority and they do not certify the implementation under the current doctrine.

| Archived spec | Historical contribution | Current ledger status |
|---|---|---|
| `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` | Historical kernel/TUI/event-log/replay implementation package. | Landed historically; not a current gate definition. |
| `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` | Historical executable TUI command loop and doc-alignment package. | Landed historically; not a current gate definition. |
| `archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md` | Historical epistemic substrate, expectation contradiction, and possession parity package. | Landed historically; not a current gate definition. |
| `archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md` | Introduced historical needs/routines/intentions/planner traces/no-human day/ordinary actions work. | Landed historically; not live authority for certifying implementation or treating historical ordinary-life work as complete. |
| `archive/specs/0006_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_HARDENING_SPEC.md` | Exposed need for stronger actor-known planning, lifecycle, replay, and no-human integration. | Landed historically; not sufficient certification under the current doctrine. |
| `archive/specs/0007_PHASE_3A_SECOND_HARDENING_NO_HUMAN_ORDINARY_LIFE_PROOF.md` | Exposed further gaps in integrated no-human proof, live agent state, string-heavy diagnostics, and hidden-truth resistance. | Landed historically; not proof that current gates are solved. |
| `archive/specs/0008_PHASE_3A_ANTI_CONTAMINATION_HARDENING_SPEC.md` | Exposed anti-contamination problems around actor-known transaction shape, planner inputs, forged/stale proposal parameters, marker actions, friendly tests, and overclaiming docs. | Landed historically; not live doctrine where it conflicts with the realigned spine. |

## Next known execution move

The execution tier names `P0-CERT` as the next major implementation audit. This ledger records that dependency but does not specify it. A future `P0-CERT` spec must use the live execution posture categories: `P0-CERT passed`, `P0-CERT scoped remediation`, or `P0-CERT not applicable`, as applicable to that future work.
