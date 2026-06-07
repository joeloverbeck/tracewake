# Spec Ledger

This ledger is not foundation doctrine. It records produced specification packages and their current audit status so future work does not retread settled specification ground or mistake historical implementation claims for phase-exit proof.

## Authority note

This ledger sits beneath foundation, architecture, execution, and reference doctrine. If this ledger conflicts with higher-authority doctrine, the higher-authority doctrine wins and this ledger should be corrected.

Every status here is a status claim at the cited source commit or audit point. A later audit may supersede an earlier status without rewriting archived specs.

## Spec 0001

**Title:** Phase 0: Missing Property Village Ontology and Fixture Contracts

**Status:** Accepted (2026-06-06).

**Phase covered:** Phase 0 paper ontology and fixture contracts.

**Source commit analyzed:** `3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e`.

**Freshness claim:** user-supplied target commit only; not independently verified as latest `main`.

### Deliverables produced

- `README.md` at package root.
- `docs/4-specs/README.md`.
- `docs/4-specs/SPEC_LEDGER.md`.
- `docs/4-specs/0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`.
- `docs/4-specs/0001_RESEARCH_NOTES.md`.
- `docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md`.

No replacement documents are included under `docs-replacements/`.

### What Spec 0001 settles

Spec 0001 settles, for Phase 0 and for Phase 1 entry:

- the identity of the first proof: Missing Expected Property / The Missing Property Village;
- the baseline first-proof actor roster;
- the minimal village place, room, door, and container roster;
- the first physical value-token contract for `coin_stack_01`;
- the distinction between ownership, custody, access, physical control, proof, belief, and institutional recognition;
- primitive action families needed for the first proof;
- primitive event families needed for event-log/replay and later epistemic/institutional projections;
- proposition and claim families;
- belief, observation, memory, expectation, speech, report, household, norm, and institutional record contracts;
- fixture names and purposes for Phase 0 and later executable tests;
- embodied and debug view-model sketches;
- no-scripting review boundaries;
- Phase 1 entry requirements.

### What Spec 0001 explicitly defers

Spec 0001 does not settle:

- Rust crate layout;
- storage engine;
- terminal UI library;
- final content file syntax;
- final serialization format;
- full epistemic implementation;
- full routines and planning implementation;
- full institutional procedures;
- road travel, notices as a product feature, route threat, wilderness/quarry spaces, bounties, companion recruitment, combat, beasts, monsters, magic, graphical client work, large-region simulation, procedural town generation, or quest/objective ontology;
- live/freeform LLM dialogue.

## Spec 0002

**Title:** Phase 1: Kernel, TUI, Event Log, and Replay Implementation Specification

**Status:** Phase 1 landed.

**Phase covered:** Phase 1 runnable kernel, TUI/view-model harness, event log, and replay.

**Spec file:** `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` (intended home `docs/4-specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md`).

**Source commit analyzed:** `841deeb6fc73f8006ed2548530c062067d4f5250`.

**Freshness claim:** user-supplied target commit only; not independently verified as latest `main`.

### What Spec 0002 settles

Spec 0002 turns Phase 1 of the execution ladder into an implementable contract, using Spec 0001 as input.

It settles, for Phase 1:

- the compact Rust workspace shape (`tracewake-core` / `tracewake-content` / `tracewake-tui`, optional `tracewake-cli`) and module boundaries;
- the deterministic simulation core, determinism contract, and canonical physical-state checksum;
- the one shared command/proposal/action validation pipeline, including inert architectural slots that later phases attach to;
- versioned event envelopes, event streams, append-only log, projection rebuild, and replay report;
- the minimal entity/component/state model with single-source item location;
- fixture/content validation phases and required validation failures;
- the TUI/view-model contract, controller-binding/possession parity, and debug/provenance reports;
- the seven required Phase 1 golden fixtures: `strongbox_001`, `container_item_move_001`, `door_access_001`, `debug_attach_001`, `no_human_advance_001`, `replay_item_location_001`, `view_model_local_actions_001`.

Spec 0002 does not implement Phase 2+ systems. It preserves only inert, validated placeholders where Phase 0 fixture shape requires them.

## Spec 0003

**Title:** Phase 1A Executable TUI Command Loop and Documentation Alignment

**Status:** Landed (2026-06-06).

**Spec file:** `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md`.

**Target reassessment commit:** `1d27a01e0a5ae6018e9207acff9eed131b06ce1d`.

**Phase:** Phase 1A; Phase 1 continuation, not Phase 2.

### Correction carried forward

Spec 0002 landed the Phase 1 kernel, content fixtures, replay/debug surface, view models, TUI facade, renderer, input helpers, and deterministic transcript harness. It did not complete the executable TUI command loop that Phase 1 doctrine and Ticket 020 required.

### Required result

`cargo run -p tracewake-tui` must become a genuinely operable stdin/stdout command loop supporting actor binding, embodied view rendering, semantic action submission by stable ID and numeric selection, wait, why-not, debug panels, replay/projection reports, and clean quit.

**Phase 2 dependency:** Satisfied by the landed Spec 0003 acceptance gates.

## Spec 0004

**Title:** Phase 2A Epistemic Substrate, Expectation Contradiction, and Possession Parity

**Status:** Phase 2A landed (2026-06-07).

**Spec file:** `archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md`.

**Phase:** Phase 2A only; this opens executable Phase 2 epistemics but does not complete all Phase 2.

### Required result

Typed propositions, source-backed observations and beliefs, expectation contradiction, `check_container`, bounded sound uncertainty, actor-known notebooks, debug epistemics, possession parity, no-human epistemic checks, content validation, and replay/projection acceptance gates are implemented and covered by the capstone exit evidence.

### Deferred scope

Phase 2B+ perception depth, memory decay, testimony, speech, reports, records, institutions, gossip, wrong suspicion workflows, routines, dialogue, planning-driven candidate goals, graphical client work, and Phase 3/4 systems remain future work.

## Spec 0005

**Title:** Phase 3A Needs, Routines, and No-Human Day

**Status:** Archived as historical implementation work. Exact-commit audit at `ec1e73f91b7330d87ce12ae93f8cf57ea3671306` found that Phase 3A acceptance was **not earned**.

**Spec file:** `archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md`.

**Phase:** Phase 3A only; this is a narrow ordinary-life substrate, not full Phase 3.

### Current meaning

The implementation contained useful scaffolding and partial surfaces, but the no-human ordinary-life substrate was not robust enough to support later phases. Corrective Spec 0006 was required before Phase 3B or Phase 4. See:

- `archive/reports/PHASE_3A_IMPLEMENTATION_AUDIT.md`
- `archive/reports/PHASE_3A_STATUS_ERRATA.md`

This file is historical evidence of intended/attempted work, not proof that Phase 3A is safe; it is not rewritten.

### Intended result

Bounded hunger/fatigue/safety needs, durable intentions, defeasible routines, sleep/eat/work/continue/wait ordinary actions through the shared pipeline, actor-known planning, no-human day runner and metrics, action/guard/canonical fixtures, replay/debug reports, embodied needs/routine surface, non-diegetic Phase 3A debug panels, README runbook updates, and capstone acceptance evidence.

### Audit result carried forward

These were not yet proven behaviorally at the Spec 0005 audit point. The code then over-relied on wait-only no-human advancement, replay-side agent-state application, string-prefix/substring HTN preconditions, synthetic replay logs, and fixture/debug snapshots rather than integrated ordinary-life behavior.

### Deferred scope

Spec 0005 explicitly does not close full Phase 3. The Phase 3 acceptance line requiring agents to speak minimally is deferred to Phase 3B; broader memory decay, testimony, reports, institutions, gossip, wrong suspicion workflows, autonomous missing-property story setup, multi-day depth, economy, graphical client, and Phase 4 regional systems remain future work.

## Spec 0006

**Title:** Phase 3A Needs, Routines, and No-Human Day Hardening

**Status:** Landed useful corrective hardening, but post-0006 static source audit at `93759ded3967936431c359a4ec8eefa1619b518b` finds Phase 3A integrated ordinary-life readiness still **not fully earned**. A second Phase 3A hardening spec is required before Phase 3B or ordinary-life-dependent Phase 4 work treats Phase 3A as safe.

**Spec file:** `archive/specs/0006_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_HARDENING_SPEC.md`.

**Phase:** Phase 3A hardening.

### Why it existed

The exact-commit audit recorded in `archive/reports/PHASE_3A_IMPLEMENTATION_AUDIT.md` found Phase 3A acceptance not earned after Spec 0005. Spec 0006 was the first corrective hardening pass for the no-human / replay / debug / TUI / actor-known gates.

### What Spec 0006 appears to have improved

The post-0006 target contains meaningful improvements:

- a no-human runner that considers multiple actors in deterministic order;
- live `AgentState` in the shared pipeline context;
- agent events for needs, intentions, routine steps, decisions, and stuck diagnostics;
- a canonical `ActorKnownPlanningState` / `VisibleLocalPlanningState` type and local planner functions that can reject unknown food/routes;
- Phase 3A TUI/debug panels and command-loop access;
- replay and metrics surfaces that include agent-state and Phase 3A streams;
- tests that go beyond the original wait-only marker proof.

### Remaining blocking result

The post-0006 source still shows foundation-level shortcut debt:

- the no-human proposal builder dispatches routines by template-name substring;
- autonomous eat/work/move proposals can still be chosen from authoritative physical state rather than actor-known planning state;
- fallback no-human planning injects magic actor-known strings and an empty epistemic projection;
- candidate generation still relies on string prefixes for known food;
- hidden-truth audit fields can be hard-coded to true instead of derived from enforced provenance;
- active intentions and routine execution ancestry are not yet consistently live in the no-human loop;
- `continue_routine` remains a marker unless a separate ordinary action is manually run;
- the strongest action-chain tests still rely partly on manually forced proposals after the no-human run;
- TUI/debug proof asserts reachability and labels but not enough integrated causal ancestry.

### Current allowed next work

Do not use Spec 0006 as proof that Phase 3A has landed far enough to unblock Phase 3B or ordinary-life-dependent Phase 4 work.

Allowed next work is the second Phase 3A hardening spec, plus narrow exploratory work that does not rely on Phase 3A being complete and does not overclaim substrate readiness.

## Next required spec

**Title:** Phase 3A Second Hardening: Integrated No-Human Ordinary-Life Proof

**Status:** Required by post-0006 static source audit at `93759ded3967936431c359a4ec8eefa1619b518b`.

**Required result:** a no-human day that produces wake/eat/move/work/rest/sleep/fail/replan ancestry from live needs, durable intentions, defeasible routines, actor-known planning, shared action validation, debug/TUI visibility, and replayable provenance without synthetic post-run action injection or hidden-truth caller hygiene.

## Retread warning

Do not create another Phase 0 ontology package unless a real doctrine gap, contradiction, or safety issue is discovered. Future work should cite Spec 0001 and move forward.

Do not create Phase 4 or ordinary-life-dependent Phase 3B implementation specs that assume Phase 3A is safe until the second Phase 3A hardening gates pass.

Exact fetch ledgers may appear in spec-package provenance. They are not enduring product doctrine.
