# Spec Ledger

This ledger is not foundation doctrine. It records produced specification packages so future work does not retread settled specification ground.

## Spec 0001

**Title:** Phase 0: Missing Property Village Ontology and Fixture Contracts  
**Status:** Accepted (2026-06-06).  
**Phase covered:** Phase 0 paper ontology and fixture contracts.  
**Source commit analyzed:** `3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e`.  
**Freshness claim:** user-supplied target commit only; not independently verified as latest `main`.

## Deliverables produced

- `README.md` at package root.
- `docs/4-specs/README.md`.
- `docs/4-specs/SPEC_LEDGER.md`.
- `docs/4-specs/0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`.
- `docs/4-specs/0001_RESEARCH_NOTES.md`.
- `docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md`.

No replacement documents are included under `docs-replacements/`.

## What Spec 0001 settles

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

## What Spec 0001 explicitly defers

Spec 0001 does not settle:

- Rust crate layout;
- storage engine;
- terminal UI library;
- final content file syntax;
- final serialization format;
- full epistemic implementation;
- full routines and planning implementation;
- full institutional procedures;
- road travel, notices as a product feature, route threat, wilderness/quarry spaces, bounties, companion recruitment, combat, beasts, monsters, magic, graphical client, large-region simulation, procedural town generation, or quest/objective ontology;
- live/freeform LLM dialogue.

## Spec 0002

**Title:** Phase 1: Kernel, TUI, Event Log, and Replay Implementation Specification  
**Status:** Phase 1 landed.
**Phase covered:** Phase 1 runnable kernel, TUI/view-model harness, event log, and replay.  
**Spec file:** `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` (intended home `docs/4-specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md`).
**Source commit analyzed:** `841deeb6fc73f8006ed2548530c062067d4f5250`.  
**Freshness claim:** user-supplied target commit only; not independently verified as latest `main`.

## What Spec 0002 settles

Spec 0002 turns Phase 1 of the execution ladder into an implementable contract, using Spec 0001 as input. It settles, for Phase 1:

- the compact Rust workspace shape (`tracewake-core` / `tracewake-content` / `tracewake-tui`, optional `tracewake-cli`) and module boundaries;
- the deterministic simulation core, determinism contract, and canonical physical-state checksum;
- the one shared command/proposal/action validation pipeline (including the inert architectural slots that later phases attach to);
- versioned event envelopes, event streams, the append-only log, projection rebuild, and the replay report;
- the minimal entity/component/state model with single-source item location;
- fixture/content validation phases and required validation failures;
- the TUI/view-model contract, controller-binding/possession parity, and debug/provenance reports;
- the seven required Phase 1 golden fixtures: `strongbox_001`, `container_item_move_001`, `door_access_001`, `debug_attach_001`, `no_human_advance_001`, `replay_item_location_001`, `view_model_local_actions_001`.

Spec 0002 does not implement Phase 2+ systems (beliefs, routines, institutions, suspicion, LLM surfaces); it preserves only inert, validated placeholders where Phase 0 fixture shape requires them.

### Spec 0003 — Phase 1A Executable TUI Command Loop and Documentation Alignment

- **Status:** Landed (2026-06-06).
- **Spec file:** `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md`
- **Target reassessment commit:** `1d27a01e0a5ae6018e9207acff9eed131b06ce1d`
- **Phase:** Phase 1A; Phase 1 continuation, not Phase 2.
- **Correction carried forward:** Spec 0002 landed the Phase 1 kernel, content fixtures, replay/debug surface, view models, TUI facade, renderer, input helpers, and deterministic transcript harness. It did not complete the executable TUI command loop that Phase 1 doctrine and Ticket 020 required.
- **Required result:** `cargo run -p tracewake-tui` must become a genuinely operable stdin/stdout command loop supporting actor binding, embodied view rendering, semantic action submission by stable ID and numeric selection, wait, why-not, debug panels, replay/projection reports, and clean quit.
- **Phase 2 dependency:** Satisfied by the landed Spec 0003 acceptance gates.

### Spec 0004 — Phase 2A Epistemic Substrate, Expectation Contradiction, and Possession Parity

- **Status:** Phase 2A landed (2026-06-07).
- **Spec file:** `archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md`
- **Phase:** Phase 2A only; this opens executable Phase 2 epistemics but does not complete all Phase 2.
- **Required result:** typed propositions, source-backed observations and beliefs, expectation contradiction, `check_container`, bounded sound uncertainty, actor-known notebooks, debug epistemics, possession parity, no-human epistemic checks, content validation, and replay/projection acceptance gates are implemented and covered by the capstone exit evidence.
- **Deferred scope:** Phase 2B+ perception depth, memory decay, testimony, speech, reports, records, institutions, gossip, wrong suspicion workflows, routines, dialogue, planning-driven candidate goals, graphical client work, and Phase 3/4 systems remain future work.

### Spec 0005 — Phase 3A Needs, Routines, and No-Human Day

- **Status:** Phase 3A landed (2026-06-07).
- **Spec file:** `archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md`.
- **Phase:** Phase 3A only; this is a narrow ordinary-life substrate, not full Phase 3.
- **Required result:** bounded hunger/fatigue/safety needs, durable intentions, defeasible routines, sleep/eat/work/continue/wait ordinary actions through the shared pipeline, no-human day runner and metrics, action/guard/canonical fixtures, replay/debug reports, embodied needs/routine surface, non-diegetic Phase 3A debug panels, README runbook updates, and capstone acceptance evidence.
- **Exit evidence:** `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`, and the Phase 3A golden/content/TUI capstone tests from `0005PHA3ANEEROU-025`.
- **Deferred scope:** Spec 0005 explicitly does not close full Phase 3. The Phase 3 acceptance line requiring agents to speak minimally is deferred to Phase 3B; broader memory decay, testimony, reports, institutions, gossip, wrong suspicion workflows, autonomous missing-property story setup, multi-day depth, economy, graphical client, and Phase 4 regional systems remain future work.

## Next allowed spec

Spec 0005 has landed Phase 3A only. The next implementation spec may continue Phase 2B+ work, open Phase 3B speech/ordinary-life follow-up, or pursue another later slice only if the Phase 2A/3A boundaries above are preserved and deferred scope is not represented as already complete.

## Retread warning

Do not create another Phase 0 ontology package unless a real doctrine gap, contradiction, or safety issue is discovered. Future work should cite Spec 0001 and move forward.

Exact fetch ledgers may appear in spec-package provenance. They are not enduring product doctrine.
