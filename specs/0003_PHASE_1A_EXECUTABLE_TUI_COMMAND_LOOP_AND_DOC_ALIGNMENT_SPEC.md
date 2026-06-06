# Spec 0003 — Phase 1A Executable TUI Command Loop and Documentation Alignment

Status: proposed corrective Phase 1 continuation  
Repository: `joeloverbeck/tracewake`  
Target commit analyzed: `1d27a01e0a5ae6018e9207acff9eed131b06ce1d`  
Spec filename: `0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md`  
Phase: Phase 1A / Phase 1 continuation, not Phase 2

This specification is intentionally narrow. It does not add new simulation mechanics. It wires the already-landed Phase 1 kernel/content/replay/TUI facade into an actually operable executable TUI command loop, and aligns documentation and acceptance tests with that executable surface.

This reassessment used the uploaded manifest only as a path inventory. It does not independently verify the latest `main`, any branch tip, or repository state outside the exact target commit above.

## Evidence ledger

Exact URLs fetched for this reassessment:

- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/01_EXECUTION_CHARTER_SCOPE_AND_FIRST_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/2-execution/12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/4-specs/0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/4-specs/0001_RESEARCH_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/archive/tickets/0002PHA1KERTUI-020.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/archive/tickets/0002PHA1KERTUI-021.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/archive/tickets/0002PHA1KERTUI-022.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/tests/transcript_snapshot.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/tests/tui_acceptance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/tests/acceptance_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/tests/golden_scenarios.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-content/tests/fixtures_load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-content/tests/golden_fixtures_run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-content/tests/forbidden_content.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/src/debug_reports.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/src/replay/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-core/src/projections.rs
- https://github.com/joeloverbeck/tracewake/blob/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/main.rs
- https://github.com/joeloverbeck/tracewake/blob/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/input.rs
- https://github.com/joeloverbeck/tracewake/blob/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/tests/tui_acceptance.rs
- https://github.com/joeloverbeck/tracewake/blob/1d27a01e0a5ae6018e9207acff9eed131b06ce1d/crates/tracewake-tui/src/app.rs

## Reassessment conclusion

Outcome: remediation is required.

Phase 1 is substantially implemented at the kernel, content fixture, replay/debug, view-model, renderer, facade, and deterministic transcript levels. The current executable binary is not yet the Phase 1 operating surface that the foundation, architecture, execution docs, archived Spec 0002, and Ticket 020 intended.

The narrow corrective finding is:

> Spec 0002 landed the Phase 1 kernel/content/replay/TUI facade, but it did not complete the executable TUI app loop. Phase 1 should continue with Spec 0003 before Phase 2 begins.

## Authority findings

The repository authority hierarchy used here is:

1. foundation docs;
2. architecture docs;
3. execution docs;
4. reference docs;
5. spec ledger / archived spec / archived tickets;
6. implementation.

The decisive readings are:

- Foundation and architecture doctrine make the TUI the first real product/client surface, not a disposable debug shell. Headless tests and debug commands are necessary but not sufficient once the kernel is runnable.
- Execution-level Phase 1 requires a playable and inspectable causal machine. The user-facing Phase 1 slice includes binding/attaching to an actor, viewing actor-grounded state, moving, inspecting, opening/closing, taking/placing, waiting, seeing why-not, switching/debug inspection, and replay/projection rebuild checks.
- Archived Spec 0002 explicitly says a passing Phase 1 must be playable through a crude TUI using stable semantic actions, and that test-only/debug-command-only kernel progress is insufficient.
- Ticket 020 is titled around the TUI shell, app loop, render, and semantic input. Its acceptance language requires binding, rendering, listing stable semantic actions, submitting an action, rendering updated state, and surfacing why-not in the TUI. Its outcome narrowed to a facade plus one-frame binary, which is not enough to satisfy its own problem statement and acceptance criteria.
- Ticket 022 ratified facade/transcript coverage as TUI playability. That is useful but too permissive relative to higher-level doctrine and Ticket 020.

## Current implementation state to preserve

The existing implementation is real and should be reused, not discarded.

The implementation already includes:

- `TuiApp::load_default` and fixture loading;
- actor binding through controller bindings;
- actor-grounded `current_view` and `render_current_view`;
- `submit_semantic_action` that resolves from the current view and routes mutation through proposal/pipeline semantics;
- renderer output with numbered action menu entries and stable semantic action IDs;
- parsing for `bind`, `do`, `wait` / `w`, and `quit` / `q`;
- helper resolution from a 1-based menu selection to a stable semantic action ID;
- debug panel renderers marked `DEBUG NON-DIEGETIC`;
- deterministic transcript harness;
- facade-level tests for action identity, why-not, wait, debug panels, replay/projection, non-leakage, and no direct event applier use.

These are the foundation for the remediation. The corrective work is executable wiring and acceptance, not redesign.

## Gap statement

The current binary performs a one-shot startup path: load default fixture, bind `actor_tomas`, print readiness, render one embodied view, and exit. It does not read stdin, does not loop, does not execute commands from the user, does not dispatch debug commands, and does not have binary-level scripted stdin/stdout tests.

The documentation, however, has already been edited ahead of the binary. The working-tree `README.md` now states the binary "enters a simple stdin/stdout command loop" and documents the full `help` / `view` / `<n>` / `do` / `wait` / `debug …` / `quit` vocabulary with a `tracewake>` prompt and a sample session, and `docs/4-specs/SPEC_LEDGER.md` already carries the Spec 0003 corrective entry. None of that command surface is reachable from `main.rs` yet. The README therefore currently over-claims: it documents commands the binary rejects — the opposite of the under-claim it carried at commit `1d27a01`, where it honestly said the vocabulary was "implemented, not yet wired into the binary." In particular, `<n>` is documented as a command, but `parse_command` does not parse bare numeric input; numeric selection exists only as the `semantic_id_for_selection` helper, not as a command path reachable from the binary. Aligning the docs with the executable state therefore means making the binary conform to the already-written documentation and then correcting residual inaccuracies, not authoring a fresh interactive description.

## Goals

1. Make `cargo run -p tracewake-tui` enter a genuine stdin/stdout command loop.
2. Preserve the existing string renderer and facade-first architecture.
3. Keep every world mutation routed through `TuiApp` -> semantic action -> proposal -> shared pipeline.
4. Support deterministic scripted stdin/stdout binary tests.
5. Align README and spec ledger with the corrected Phase 1A status.
6. Keep debug panels visibly non-diegetic and non-mutating.
7. Keep the TUI ugly, simple, deterministic, and operable.

## Non-goals

Do not implement any of the following in this spec:

- Phase 2 epistemics, actor-known notebooks, belief propagation, memory systems, claims, or possession parity beyond the existing Phase 1 actor-filtered view model.
- Needs, routines, institutions, households, records, reports, suspicion, travel, notices, LLM surfaces, graphical UI, or regional simulation.
- A ratatui/crossterm terminal backend unless a later spec justifies the dependency and test burden.
- New domain mechanics beyond what Phase 1 already exposes through existing semantic actions and fixtures.
- Tickets or agent task decomposition.

## Binding invariants

This is a TUI-boundary and acceptance-alignment spec; it adds no new world mechanics, so it must satisfy the TUI, possession, debug, determinism, and acceptance invariants from `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` without weakening any of them. These are constraints to preserve, not new deliverables:

- **INV-065 — The TUI is a primary product interface.** Making `cargo run -p tracewake-tui` operable is the whole point; the executable loop is the Phase 1 operating surface, not a throwaway shell.
- **INV-066 — Every runnable phase has a TUI/view-model gate.** Phase 1 mechanics (bind, view, semantic actions, wait, why-not, debug) must be reachable and regression-tested through the running binary, not only through library tests.
- **INV-071 — Mechanics hidden from play are unfinished.** The kernel is runnable, so mechanics reachable only from headless tests are incomplete until wired into the loop.
- **INV-069 — The TUI must not implement simulation rules.** The command parser only classifies input and delegates; it submits typed semantic actions and never decides legality (see §Mutation boundary).
- **INV-008 — UI assistance is not authority.** Numeric selection, menus, and help text clarify only; they create no facts and bypass no preconditions. The menu index is never action identity (see §Numeric selection requirement).
- **INV-070 — Why-not explanations are mandatory.** Rejected commands render the actor-visible why-not summary (see §Output behavior).
- **INV-067 / INV-024 / INV-031 / INV-068 / INV-093 — Embodied mode shows actor-known reality; no telepathy; debug is non-diegetic.** Debug panels stay marked non-diegetic and never leak hidden truth back into the embodied view (see §Debug boundary, §Required tests #3).
- **INV-005 / INV-006 / INV-094 — Possession changes input binding only.** `bind <actor_id>` re-binds the controller and carries no world knowledge across actors.
- **INV-017 / INV-018 / INV-092 — Deterministic, seedable, replay-tested.** The default startup path and scripted stdin/stdout sessions are deterministic; debug commands do not alter the physical checksum.
- **INV-004 / INV-091 — No-human runnable.** Adding a human command loop must not regress the no-human advance path; the world still runs with no controller bound.

## Assumptions

These are taken as settled for this spec; each is one-line-correctable if implementation discovers otherwise:

- The default startup fixture is `strongbox_001` and the default bound actor is `actor_tomas`, matching the current `TuiApp::load_default` and `main.rs` defaults.
- The user-visible command strings in §Minimum commands are final unless implementation hits a direct conflict with existing stable terminology; if any name changes, README and tests adopt the final name consistently.
- The existing string/stdout renderer and the `TuiApp` facade are reused as-is; no renderer rewrite is in scope.

## Required executable behavior

### Startup

The binary MUST:

1. load the default Phase 1 fixture, currently `strongbox_001`;
2. bind `actor_tomas` by default for deterministic first-run play;
3. print the existing readiness line, `tracewake-tui ready`;
4. render the initial embodied view;
5. print a prompt or otherwise clearly await commands;
6. continue reading commands until `quit`, `q`, or EOF.

EOF MAY exit cleanly without printing an error.

### Minimum commands

The executable command loop MUST support at least:

```text
help
view
bind <actor_id>
do <semantic_action_id>
<n>
wait
w
debug log
debug bindings
debug item <item_id>
debug rejection
debug projection
debug replay
quit
q
```

The exact internal enum names are not important. The user-visible command strings above are required unless implementation discovers a direct conflict with existing stable terminology; if changed, README and tests MUST use the chosen final names consistently.

### Command semantics

`help` prints supported commands and does not mutate world state.

`view` re-renders the current embodied view and does not mutate world state.

`bind <actor_id>` attaches the controller to an actor through the existing binding path, then renders that actor's embodied view. Binding is a controller/debug boundary operation, not a physical world mutation.

`do <semantic_action_id>` submits the stable semantic action ID through the existing `TuiApp::submit_semantic_action` path. It MUST NOT synthesize action IDs, target IDs, or proposal effects outside the facade/pipeline path.

`<n>` resolves the 1-based menu number against the current rendered/current view to the action entry's stable semantic action ID, then submits that stable ID. The menu index is never action identity. Reordering a menu must not cause the wrong stable semantic action to execute.

`wait` and `w` submit the existing wait semantic action through the same semantic-action/pipeline path as other actions. They MUST NOT directly advance time by mutating scheduler or state outside the facade/pipeline path.

`debug log`, `debug bindings`, `debug item <item_id>`, `debug rejection`, `debug projection`, and `debug replay` render the existing debug panels or their direct successors. Every debug panel MUST be clearly marked non-diegetic.

`quit` and `q` exit cleanly.

### Output behavior

After every accepted ordinary command, the loop MUST print a concise accepted/result line and render the updated embodied view.

After every rejected ordinary command, the loop MUST print the actor-visible rejection/why-not result and render the current embodied view with the why-not summary visible.

After every debug command, the loop MUST render the requested debug panel. It SHOULD NOT replace the embodied view's actor-known content. The next `view` or ordinary command must still render only actor-filtered embodied information.

Invalid commands MUST produce an actor-safe, non-mutating error. They MUST NOT reveal hidden world truth, hidden item locations, debug-only checksums, or internal state beyond the safe command error.

### Numeric selection requirement

Bare numeric input MUST be reachable from the executable command loop. It may be implemented in `parse_command`, in a loop-level pre-parser, or through a new parser function. The final design MUST be covered by tests that launch the actual binary.

The README claim that `<n>` submits the action at a menu position is only valid after this requirement is implemented and tested.

### Debug boundary

Debug output MUST continue to use a marker equivalent to `DEBUG NON-DIEGETIC`.

Debug commands MUST NOT alter:

- physical checksum;
- embodied view output;
- current actor binding except for explicit bind/attach operations;
- event log world stream;
- hidden-truth visibility in the next embodied view.

`debug item <item_id>` may reveal true item location only inside the debug panel.

### Mutation boundary

The TUI executable loop MUST NOT call the event applier directly. It MUST NOT edit `PhysicalState` directly. It MUST not duplicate action-specific rules.

Allowed mutation path:

```text
command loop -> TuiApp -> current view semantic action -> Proposal -> run_pipeline -> EventLog/apply path owned by core
```

### Dependency policy

Use the existing string/stdout renderer first. A simple stdin/stdout loop is preferred.

Adding a small dev dependency for binary tests is allowed. Adding runtime terminal dependencies such as `ratatui` or `crossterm` is out of scope unless this spec is explicitly revised.

## Required implementation shape

The implementation SHOULD expose a testable runner function, for example a function accepting a buffered reader and writer, so the binary and tests use the same loop. The exact signature is not prescribed.

The binary's `main` should be a thin wrapper around that runner.

The command parser may be expanded, but it must not become a world-authority layer. It should only classify user input and delegate to `TuiApp` or debug render accessors.

## Required tests

### 1. Binary scripted session test

Add an integration test that launches the actual `tracewake-tui` binary, feeds scripted stdin, and asserts stdout contains at least:

- startup readiness line;
- initial embodied view;
- action list;
- an accepted action result;
- an updated embodied view;
- a rejected action or why-not flow;
- at least one debug panel marked non-diegetic;
- clean quit output or clean zero exit.

### 2. Numeric-selection binary test

Add a binary-level test proving bare `<n>` resolves to the stable semantic action ID from the current view.

This test must prove the menu number itself is not used as the action identity. A good implementation will assert output/effects consistent with the stable semantic action ID printed in the menu, not merely assert that input `1` caused some mutation.

### 3. Debug non-leakage test

Add a binary or facade-level regression showing:

- debug item location can reveal true location inside a non-diegetic panel;
- returning to embodied view does not reveal hidden item location;
- physical checksum is unchanged by debug commands.

### 4. No-direct-mutation regression

Extend the existing no-direct-event-applier regression to cover any new command-loop module and `main.rs` as appropriate.

The TUI crate must still not call `apply_event` directly outside the approved core pipeline path.

### 5. README sample session test

If practical, add a test that runs the README sample commands or representative fragments. The test does not need to snapshot every byte, but it must fail if README-documented command names stop working.

### 6. Existing workspace gates

The corrective implementation is not accepted until these pass:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

## Documentation requirements after implementation

### `README.md`

The working-tree `README.md` has already been rewritten from the original one-frame description to an interactive-usage description (command list, sample session, `tracewake>` prompt). This documentation landed ahead of the binary, so until the command loop is implemented the README over-claims. Do not re-author this section from scratch; reconcile it with the landed binary.

After the command loop lands, the README MUST:

- state that `cargo run -p tracewake-tui` enters an interactive stdin/stdout command loop (already present — verify it is now true);
- list the supported commands using the final command names actually implemented;
- include a small sample session whose every line the binary actually accepts or rejects as shown;
- ensure no command documented in the README is rejected by the binary — in particular, confirm `<n>` numeric selection is reachable, since the original README listed it as an `input.rs` parser command when it was only the `semantic_id_for_selection` helper;
- avoid claiming terminal features that are not implemented.

### `docs/4-specs/SPEC_LEDGER.md`

The Spec 0003 corrective entry has already been added to the working-tree ledger. Verify rather than re-author, and correct it.

The ledger MUST state that Spec 0002 landed the kernel/content/replay/TUI facade, but Spec 0003 closes the executable TUI command-loop gap (already present — verify). It MUST NOT represent this work as Phase 2. The ledger's next-spec guidance MUST block Phase 2 until Spec 0003 passes its acceptance gates (already present — verify).

The existing entry records the Spec 0003 file home as `docs/4-specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md`, but the spec currently lives at `specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md`. Pin the canonical location and make the ledger path match wherever the file actually resides.

### `docs/4-specs/README.md`

The "Current specs" list does not yet mention Spec 0003. Add it, using the same canonical location chosen for the ledger entry above, so a reader routing from `docs/4-specs/` can find the active corrective spec.

### Archived specs/tickets

Do not rewrite archived tickets or archived Spec 0002 unless the repository already has a clear archive-correction policy. Carry the correction forward through Spec 0003 and the ledger.

## Acceptance checklist

Spec 0003 is complete only when all of the following are true:

- `cargo run -p tracewake-tui` is genuinely operable by typing commands.
- A user can bind/switch actor, view, submit stable semantic actions, use numeric selection, wait, see why-not, request debug panels, and quit.
- The default startup path is deterministic and documented.
- Binary integration tests launch the actual binary with scripted stdin.
- Numeric selection is tested at binary level.
- Debug truth does not leak back into embodied view.
- Debug commands do not alter physical checksum.
- No TUI code directly applies events or mutates world state outside the shared proposal/pipeline path.
- README, spec ledger, and specs README are aligned with the executable state.
- Workspace fmt, clippy, build, and tests pass.

## Phase boundary

Do not start Phase 2 from this commit state. Finish Spec 0003 first. Once Spec 0003 lands, Phase 1 can honestly be described as having a crude but real executable TUI surface backed by the existing facade and kernel.
