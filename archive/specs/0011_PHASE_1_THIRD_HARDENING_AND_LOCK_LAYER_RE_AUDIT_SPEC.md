# 0004 — Phase 1 Third Hardening and Lock-Layer Re-Audit Specification

**Status:** Accepted and archived scoped hardening / anti-contamination remediation
**Repository analyzed:** `joeloverbeck/tracewake`  
**Analyzed commit:** `82736f5dc9f71d05b32125c26348e4a659c10a53`  
**Assumed staging path:** `specs/0011_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md`  
**Intended final home on acceptance:** `archive/specs/0011_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md` (scoped remediation specs are archived as historical evidence, per the `0002_TUI_PROOF` / `0003_PHASE_1_SPINE` precedent; they are not promoted into the live `docs/4-specs/` tier)  
**Execution admissibility posture:** `P0-CERT scoped remediation`  
**Verdict:** **Positive — third hardening remediation accepted for this scoped campaign.**

**Implementation closeout:** Ticket series `0004PHA1THIHAR-001` through `0004PHA1THIHAR-012` completed and archived. Final acceptance evidence is recorded in `archive/tickets/0004PHA1THIHAR-012.md`; the §10 CI implementation commit under test was `cbf32c45b82aed83f0b9b21a21ab6e34082bee2a`, and the ticket evidence was archived at `3ac63bdbf90d9557ffc790c231821ac39b49672f`.

I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

This is not a project-wide certification, not a latest-main claim, and not a reopening of later Phase 2+ or Phase 3A scope. It is a scoped Phase 1 / Phase 1A spine, TUI seam, content seam, and structural-lock-layer remediation spec.

---

## Evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 82736f5dc9f71d05b32125c26348e4a659c10a53
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.open / web.find over exact raw.githubusercontent.com and github.com/blob URLs; no repository-scoped fetch arguments
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/reports/tui-proof-surface-hardening-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/reports/phase1-spine-anti-contamination-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/clippy.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/rust-toolchain.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/events/envelope.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/events/log.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/events/mutation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/events/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/replay/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/checksum.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/time.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/proposal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/registry.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/accuseprobe.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/checkcontainer.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/continue_routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/eat.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/inspect.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/movement.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/openclose.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/sleep.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/takeplace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/wait.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/actions/defs/work.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/controller.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/debug_capability.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/src/debug_reports.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/src/schema.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/src/load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/src/validate.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/src/manifest.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/src/serialization.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/src/fixtures/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/src/fixtures/no_human_day_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/src/run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/src/launch.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/tests/spine_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/tests/no_human_capstone.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/tests/doc_invariant_references.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/tests/acceptance_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/tests/hidden_truth_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-core/tests/golden_scenarios.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/tests/forbidden_content.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/tests/schema_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/tests/fixtures_load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-content/tests/golden_fixtures_run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/tests/tui_seam_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/tests/adversarial_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/tests/tui_acceptance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/tests/command_loop_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/tests/readme_sample_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/crates/tracewake-tui/tests/transcript_snapshot.rs
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

**Line-evidence note.** Several Rust files in the exact raw fetch are exposed by the URL fetcher as single logical text lines. Where the raw file is a single logical line, this spec cites `path:1` and names the audited symbol, test, or constant. GitHub blob exact-commit views of the same owner/repo/commit/path were used only to locate those symbols more readably; no branch fetch, repository metadata, default-branch lookup, code search, snippet search, clone, prior-chat memory, or namespace label was used as evidence.

---

## 1. Verdict

**A third hardening spec is warranted.** The core event-log/replay/pipeline/TUI/content spine is substantially aligned with the doctrine, and the two predecessor hardening passes landed meaningful protections. However, this pass found material risks that still threaten the campaign’s third goal: making later drift structurally difficult or impossible.

The decisive risks are not cosmetic:

1. **Public seed-mutation APIs still expose direct mutable access to authoritative world and agent state.** `PhysicalState` and `AgentState` keep their primary maps `pub(crate)` and ordered, which is good, but they also expose public `seed_*_mut` methods returning `&mut BTreeMap` handles (`crates/tracewake-core/src/state.rs:1`, symbols `PhysicalState::seed_actors_mut`, `seed_places_mut`, `seed_doors_mut`, `seed_containers_mut`, `seed_items_mut`, `seed_food_supplies_mut`, `seed_workplaces_mut`, and `AgentState::seed_*_mut`). Because these types are public, external workspace crates can mutate authoritative state after seed construction without event append/apply, replay accounting, checksum provenance, or proposal validation. That is a material event-sourcing and no-direct-dispatch gap. It bears on `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-092`, `INV-101`, and `INV-104`.

2. **The event-application capability exists, but it does not close the public seed-mutation bypass.** `WorldMutationCapability` and `AgentMutationCapability` are minted only inside `crate::events` (`crates/tracewake-core/src/events/mutation.rs:1`), and `apply_event` routes through that event module (`crates/tracewake-core/src/events/apply.rs:1`). That is the right pattern. But the public seed mutators bypass that capability entirely, so the lock is not yet at the same structural level as the doctrine requires.

3. **The lock layer is useful but too string/token-heavy to be the highest-confidence enforcement layer.** `anti_regression_guards.rs` has a recursive source walker and banned-token checks for nondeterministic APIs (`crates/tracewake-core/tests/anti_regression_guards.rs:1`, symbols `production_sources`, `production`, `BANNED_NONDETERMINISM_TOKENS`, `nondeterminism_api_gate`). It also scans for direct `apply_event` calls and display-label diagnostic risks by substring. This catches obvious drift, but it is brittle against aliasing, re-export, macro expansion, comments/strings, generated code, and carve-outs. Its synthetic tests prove that a string containing a token is detected; they do not prove that the guard fails the build for adversarial negative fixtures.

4. **`clippy.toml` is stronger than the scanner, and CI runs it, but no negative fixture proves the policy’s actual coverage on the pinned toolchain.** The root `clippy.toml` declares `disallowed-types` for `HashMap`, `HashSet`, `SystemTime`, and `Instant`, and `disallowed-methods` for thread/process/filesystem/network calls (`clippy.toml:1`). CI runs `cargo clippy --workspace --all-targets -- -D warnings` and the standard build/test gates on the pinned Rust toolchain (`.github/workflows/ci.yml:1`; `rust-toolchain.toml:1`). That is a real lock. But the prior layer does not include adversarial clippy-negative fixtures for aliases, re-exports, macro use, or higher-order method references, so the assurance is assumed rather than demonstrated.

5. **The invariant-reference linter proves only “not dangling,” not “semantically right” or “coverage complete.”** `doc_invariant_references.rs` builds a set of defined `INV-###` IDs and fails if scanned live docs/specs/tests reference an undefined invariant (`crates/tracewake-core/tests/doc_invariant_references.rs:1`, symbols `defined_invariants`, `doc_invariant_references_are_live`, `live_reference_paths`; referenced IDs are recognized inline by scanning for `### INV-` / `**INV-` headings and `INV-` tokens, not via a dedicated function). It does not prove that each finding/requirement cites an invariant, that valid-looking invariant numbers are the right ones, or that high-risk invariants remain referenced by the lock layer. This is acceptable as a dangling-reference guard, but not enough as the only doc↔invariant enforcement.

6. **The conformance capstones are indexes, not property proofs.** `spine_conformance.rs` maps `SPINE-AC-001…015` to layers and named tests, then checks that the named function exists by `source.contains("fn <test_name>(")` (`crates/tracewake-core/tests/spine_conformance.rs:1`, symbols `SPINE_EVIDENCE`, `spine_conformance_maps_every_spine_requirement_to_named_evidence`, `assert_test_exists`). `tui_seam_conformance.rs` does the same style of named-test mapping for the TUI seam (`crates/tracewake-tui/tests/tui_seam_conformance.rs:1`). These are valuable review indexes, but string existence is not property proof. The spec must require evidence kinds and negative tests, not merely named functions.

The positive verdict is deliberately scoped. The event log, replay rebuild, checksum, proposal pipeline, typed diagnostics, TUI semantic-action submission, debug capability boundary, content forbidden-construct validation, CI, and prior gate suites are not broadly broken. The remaining work is to close concrete bypasses and raise weak locks from convention/string-test level toward compile-time capability, clippy-backed policy, and adversarial negative fixtures.

---

## 2. Path and numbering confirmation

The staged filename is confirmed as:

```text
specs/0011_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md
```

Final home on acceptance should be:

```text
archive/specs/0011_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md
```

Reasoning:

- `docs/4-specs/SPEC_LEDGER.md:1` records both predecessor hardening specs — `archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` and `archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` — as archived historical evidence accepted for their scope, not as live `docs/4-specs/` authority. This third pass is the same kind of scoped remediation spec and follows the same disposition: staged in `specs/`, archived to `archive/specs/` on acceptance.
- `docs/4-specs/README.md:1` requires live-tier specs to declare posture, use gate codes only as cross-references, preserve holder-known / actor-known terminology, and avoid files created merely for symmetry; the live tier is not a remediation certification ledger.
- The live `docs/4-specs/` tier currently holds only `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` and `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`; this remediation spec is not promoted there.
- The numeral `0004` here continues the hardening-campaign staging sequence (`specs/0002 → 0003 → 0004`). It is deliberately distinct from the archived implementation-series `archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md`, which already occupies `0004` in that separate series; the two coexist in `archive/specs/` without file collision because their titles differ.

---

## 3. Posture declaration

**Execution admissibility posture:** `P0-CERT scoped remediation`.

This spec contributes scoped evidence toward `SPINE-CERT`, `EPI-CERT`, and `P0-CERT` by hardening the Phase 1 / Phase 1A code-audit boundary named in execution doctrine. It does not certify the whole project, does not certify latest `main`, does not advance later-block implementation, and does not redefine gate semantics. The posture is positive because the findings require concrete code, test, lint, and CI work; it is not `not applicable`.

---

## 4. Authority chain and gate mapping

### 4.1 Controlling doctrine

The controlling authority order is foundation → architecture → execution → reference → specs (`docs/README.md:1`). This spec operationalizes, but does not amend, the higher tiers.

Foundation commitments:

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:1` defines the invariant contract. Findings and requirements below cite verified invariant IDs, especially `INV-008`, `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-020`, `INV-022`, `INV-060`, `INV-061`, `INV-067`, `INV-068`, `INV-069`, `INV-091`, `INV-092`, `INV-093`, `INV-094`, `INV-095`, `INV-097`, `INV-099`, `INV-100`, `INV-101`, `INV-102`, `INV-103`, `INV-104`, `INV-105`, `INV-106`, `INV-107`, and `INV-108`.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md:1` controls event-sourced causality: meaningful change → event; append-only logs; rebuildable projections; deterministic replay; schema-version discipline.
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md:1` controls content no-script / no-authored-outcome / seed-not-script discipline.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md:1` controls the truth firewall: truth may validate but must not plan, render, or contaminate actor-known context.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md:1` controls TUI possession, view models, debug quarantine, and two-layer why-not behavior.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md:1` controls first playable acceptance boundaries and excludes later-phase scope.

Architecture commitments:

- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md:1` controls one-way crate dependencies, core zero dependencies, no hidden mutation, and display-string-not-authority discipline.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md:1` controls event log, replay, projections, save packages, randomness, seeding, and deterministic checksums.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:1` controls holder-known context and provenance boundaries.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md:1` controls proposal → validation → scheduling → event-commit flow and the no-direct-dispatch boundary.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md:1` controls TUI semantic actions, debug split, and client authority boundaries.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md:1` controls observable acceptance artifacts and architectural conformance posture.

Execution and reference commitments:

- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md:1` controls archived-spec posture and the code-audit boundary.
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md:1` controls gate order and certification vocabulary.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md:1` controls mandatory anti-contamination gates.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md:1` controls scheduler, action pipeline, and no-direct-dispatch execution gates.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md:1` controls view-model / possession / debug proof seams.
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md:1` controls content schema and validation.
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md:1` controls golden fixture and replay acceptance.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:1` controls typed diagnostics and adversarial negative gates.
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md:1`, `docs/3-reference/01_DESIGN_RISK_REGISTER.md:1`, and `docs/3-reference/02_GLOSSARY.md:1` control review checklist, known risks, and terminology.

### 4.2 Gate mapping as cross-references only

This spec cross-references these gates but does not redefine them:

- `SPINE-CERT`: event log, replay, projection, checksum, randomness, scheduler, action pipeline, TUI/debug split, no-direct-dispatch.
- `EPI-CERT`: holder-known / actor-known, possession, view-model, and debug separation seams.
- `P0-CERT`: scoped certification posture and admissible remediation record.
- `NO-DIRECT`: every world mutation must flow through proposal/pipeline/event application, not direct dispatch or ad hoc mutation.
- `REPLAY`: deterministic replay equality, projection rebuild, schema-version handling, and checksum coverage.
- `FIXTURE`: golden and adversarial fixture validation.
- `DIAG`: typed reason codes and provenance over display strings.

---

## 5. Scope and non-goals

### In scope

- Phase 1 / Phase 1A kernel spine: event envelope, schema versioning, append-only event log, event application, replay rebuild, projection rebuild, checksums, deterministic ordering, scheduler/no-human advance, action proposal/validation/pipeline, authoritative state, and mutation boundaries.
- Phase 1 / Phase 1A content seam: typed schema, deterministic load, canonical serialization, no-script validation, forbidden content constructs, and golden fixtures.
- TUI proof seam: embodied view models, semantic commands, transcript stability, debug panels, debug capability, and prohibition on TUI authority.
- Structural-lock layer introduced by the prior spine pass: conformance capstones, invariant-reference linter, banned-API token scanner, `clippy.toml`, content forbidden-construct guards, and CI execution of those gates.

### Non-goals

- No later-block implementation: no new epistemic mechanics, ordinary-life/routine expansion, institutions, Phase 2+, Phase 3A+, LLM, or graphical client scope.
- No new world mechanics.
- No gate semantic changes.
- No backward-compatibility aliases, shims, or deprecated paths.
- No promotion of archived specs into live authority.
- No file creation for symmetry.

---

## 6. Audit findings

### F-001 — Public seed-mutation APIs bypass event sourcing

**Status:** needs hardening  
**Layer:** core/state  
**Invariants:** `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-092`, `INV-101`, `INV-104`  
**Evidence:** `crates/tracewake-core/src/state.rs:1` declares authoritative state maps with ordered `BTreeMap` / `BTreeSet` storage, but exposes public mutable seed accessors on `PhysicalState` and `AgentState`: `seed_actors_mut`, `seed_places_mut`, `seed_doors_mut`, `seed_containers_mut`, `seed_items_mut`, `seed_food_supplies_mut`, `seed_workplaces_mut`, `seed_needs_by_actor_mut`, `seed_intentions_mut`, `seed_active_intention_by_actor_mut`, `seed_routine_executions_mut`, `seed_decision_traces_mut`, and `seed_stuck_diagnostics_mut`.

**Assessment:** Ordered collections satisfy determinism pressure, but public `&mut BTreeMap` handles are a live mutation surface. These names imply seed-time use, but the type system does not restrict them to seed-time. Because the methods are `pub`, any content or TUI crate, future workspace crate, or test helper *could* mutate authoritative state after initialization without event append, replay position, proposal validation, or checksum provenance. At this commit the actual call sites are confined to `tracewake-core`'s own integration tests (`crates/tracewake-core/tests/golden_scenarios.rs`, `acceptance_gates.rs`, `no_human_capstone.rs`, `hidden_truth_gates.rs`, and the `anti_regression_guards.rs` coverage check); `tracewake-content` already constructs seed state through `PhysicalState::from_seed_parts` / `AgentState::from_seed_parts` (`crates/tracewake-content/src/schema.rs:398`, `:494`), not the mutators. The exposure is therefore latent rather than exploited, but the `pub` surface still materially violates the one-way causal spine unless sealed.

### F-002 — Event mutation capability is good but incomplete

**Status:** already satisfied in the event module, needs hardening at state API boundary  
**Layer:** core/events + core/state  
**Invariants:** `INV-009`, `INV-011`, `INV-018`, `INV-101`, `INV-104`  
**Evidence:** `crates/tracewake-core/src/events/mutation.rs:1` defines `WorldMutationCapability` and `AgentMutationCapability` with private fields and `pub(in crate::events)` minting. `crates/tracewake-core/src/events/apply.rs:1` uses event-application routines to mutate through those capabilities.

**Assessment:** The event module already uses the right structural shape: a privileged capability controls mutation. The problem is that the capability is not universally required for authoritative state mutation. Seed construction must receive a separate seed/build capability or builder, and post-seed mutable state access must require the event capability.

### F-003 — Event log, schema versioning, and replay rebuild are substantially aligned

**Status:** already satisfied; preserve and extend adversarial coverage  
**Layer:** core/events + core/replay  
**Invariants:** `INV-009`, `INV-011`, `INV-018`, `INV-020`, `INV-092`  
**Evidence:** `crates/tracewake-core/src/events/log.rs:1` stores `EventLog` internally as ordered event envelopes and rejects unsupported schema versions through append validation. `crates/tracewake-core/src/events/envelope.rs:1` provides event kinds, stream classification, schema registry metadata, replay handling, and payload-field metadata. `crates/tracewake-core/src/events/apply.rs:1` rejects unsupported schema versions / stream mismatches and applies events through the event path. `crates/tracewake-core/src/replay/rebuild.rs:1` rebuilds projections by applying ordered event streams and produces structured rebuild reports.

**Assessment:** This is one of the strongest seams. It matches event-sourcing prior art: state changes are represented as event sequences that can rebuild state, and the event log becomes the durable causal record.[^fowler-event-sourcing] The third pass does not require redesign here; it requires closing the public state mutation bypass and adding negative unsupported-version / non-world leakage fixtures.

### F-004 — Checksum coverage exists but remains manually mirrored

**Status:** needs hardening  
**Layer:** core/checksum + core/state  
**Invariants:** `INV-018`, `INV-092`, `INV-105`  
**Evidence:** `crates/tracewake-core/src/checksum.rs:1` defines `StateChecksumCoverage`, `PHYSICAL_STATE_CHECKSUM_COVERAGE`, and `AGENT_STATE_CHECKSUM_COVERAGE`, then emits canonical checksum lines from ordered collections. `crates/tracewake-core/src/state.rs:1` separately declares state fields.

**Assessment:** The checksum implementation is deterministic and ordered, but the field coverage registry is manual. A future state field could be added without being included in the registry and canonical checksum emission unless a guard catches it. The prior guard `checksum_coverage_is_total_for_authoritative_state` exists by name in `anti_regression_guards.rs`, but the structural lock should compare actual authoritative fields to the registry and canonical emission points, not only assert current strings.

### F-005 — Deterministic scheduling and discrete time are aligned

**Status:** already satisfied; preserve with injection tests  
**Layer:** core/scheduler + core/time  
**Invariants:** `INV-017`, `INV-018`, `INV-091`, `INV-092`, `INV-103`, `INV-104`  
**Evidence:** `crates/tracewake-core/src/scheduler.rs:1` defines deterministic ordering keys using simulation tick, phase, source ID, proposal sequence, action ID, ordered target IDs, and stable tie-breakers. `crates/tracewake-core/src/time.rs:1` defines `SimTick` as a discrete tick type, not wall-clock time.

**Assessment:** This seam is aligned. The next lock layer must prove that nondeterminism cannot enter through banned APIs, hidden HashMap/HashSet iteration, wall-clock, thread scheduling, filesystem/network reads, or process calls in outcome-affecting code. Rust’s standard `HashMap` default has a randomly seeded hasher, which justifies the project’s strong preference for ordered maps in deterministic outcome paths.[^rust-hashmap]

### F-006 — Proposal pipeline and typed diagnostics are substantially aligned

**Status:** already satisfied, except for direct state-mutation bypass from F-001  
**Layer:** core/actions  
**Invariants:** `INV-009`, `INV-099`, `INV-100`, `INV-101`, `INV-102`, `INV-104`, `INV-105`, `INV-106`  
**Evidence:** `crates/tracewake-core/src/actions/proposal.rs:1` carries proposal source context, source view-model ID, holder-known context ID/hash/frontier, actor ID, semantic action ID, and provenance ancestry. `crates/tracewake-core/src/actions/pipeline.rs:1` validates proposals, performs dry-run event application to cloned state before acceptance, appends accepted events, and applies through the event path. `crates/tracewake-core/src/actions/report.rs:1` defines typed `ReasonCode`, `CheckedFactKey`, `CheckedFactSource`, `CheckedFact`, and typed pipeline stage/report status structures.

**Assessment:** The pipeline is materially aligned with truth-firewall and typed-diagnostic doctrine. The direct-state public seed APIs are the one material exception because they permit bypassing the pipeline entirely.

### F-007 — Debug capability and debug report quarantine are structurally strong

**Status:** already satisfied; preserve with compile-fail and negative tests  
**Layer:** core/debug + TUI/debug  
**Invariants:** `INV-008`, `INV-067`, `INV-068`, `INV-069`, `INV-093`, `INV-094`, `INV-095`, `INV-099`, `INV-100`, `INV-107`, `INV-108`  
**Evidence:** `crates/tracewake-core/src/debug_capability.rs:1` defines `DebugCapability` with private marker and `pub(crate)` minting. `crates/tracewake-core/src/debug_reports.rs:1` stores `debug_capability: DebugCapability` in item-location, action-rejection, projection-rebuild, replay, controller-binding, Phase 3A debug, and no-human-day debug reports. Public `debug_only()` methods expose classification without exposing minting. `crates/tracewake-tui/src/debug_panels.rs:1` renders debug panels separately from embodied view rendering.

**Assessment:** This is a good capability boundary. It should remain; this spec requires negative tests to ensure debug report construction cannot be forged outside the crate and that debug facts never feed embodied affordances or actor-known cognition.

### F-008 — TUI semantic action boundary is aligned

**Status:** already satisfied; preserve  
**Layer:** tracewake-tui + core/view_models  
**Invariants:** `INV-008`, `INV-067`, `INV-068`, `INV-069`, `INV-093`, `INV-094`, `INV-095`, `INV-108`  
**Evidence:** `crates/tracewake-tui/src/input.rs:1` parses embodied commands into typed `UiCommand::SelectSemanticAction(SemanticActionId)` and separates `UiCommand::Debug(DebugCommand)`. `crates/tracewake-tui/src/app.rs:1` builds an embodied view from sealed context, resolves submitted semantic action IDs against the current view, creates a proposal from current-view semantic action data, builds an `OrderingKey`, and calls `run_pipeline`; it does not call event application directly. `crates/tracewake-tui/tests/embodied_flow.rs:1` checks holder-known context IDs/hashes, disabled why-not output, and lack of hidden truth in embodied rendering. `crates/tracewake-tui/tests/tui_seam_conformance.rs:1` maps the seam to named tests, though by string existence.

**Assessment:** The actual app seam is good. The conformance index needs hardening as an evidence map, not as a proof of the property.

### F-009 — Content validation rejects no-script constructs, but source-string field coverage is weaker than typed coverage

**Status:** already satisfied on core no-script behavior; needs stronger guard shape for future schema drift  
**Layer:** tracewake-content/schema + validation + tests  
**Invariants:** `INV-022`, `INV-060`, `INV-061`, `INV-097`, `INV-102`, `INV-105`  
**Evidence:** `crates/tracewake-content/src/validate.rs:1` rejects reserved IDs and display-name-as-ID patterns for player/protagonist/quest/objective/reward/culprit/director-shaped constructs. `crates/tracewake-content/tests/forbidden_content.rs:1` includes `content_prose_born_fact_rejected` and `content_new_field_requires_typed_validation_and_canonical_serialization_metadata`. `crates/tracewake-content/src/serialization.rs:1` centralizes canonical serialization behavior.

**Assessment:** The semantic policy is correct. The weakest part is the field coverage guard’s use of source-string inspection to prove new schema fields are covered. This should move toward a typed schema-field registry / macro-defined single source of truth, with source scanners retained only as smoke.

### F-010 — Banned-API scanner is useful but evadable

**Status:** needs hardening  
**Layer:** core/tests + workspace/ci + clippy  
**Invariants:** `INV-017`, `INV-018`, `INV-092`, `INV-105`  
**Evidence:** `crates/tracewake-core/tests/anti_regression_guards.rs:1` has `production_sources()` walking `crates/tracewake-core/src`, a `production()` function that splits on `#[cfg(test)]`, a `BANNED_NONDETERMINISM_TOKENS` table, and `nondeterminism_api_gate` performing `source.contains(token)`. It also performs direct-apply and diagnostics guard checks by substring.

**Assessment:** This catches obvious token drift and should be retained as a smoke guard. It is not sufficient as a primary guarantee. It can miss alias/re-export/macro/type-inference/higher-order call cases, and it can false-positive comments/strings. The correct primary enforcement is compiler/linter-backed policy plus negative fixtures that intentionally attempt bypass. Clippy supports `disallowed-methods` and `disallowed-types` as fully qualified path policy, making it the stronger layer for many of these bans.[^clippy-disallowed]

### F-011 — `clippy.toml` and CI are real locks, but unproven against adversarial cases

**Status:** already satisfied on presence/running; needs adversarial coverage  
**Layer:** workspace/ci + clippy  
**Invariants:** `INV-017`, `INV-018`, `INV-092`, `INV-105`  
**Evidence:** `clippy.toml:1` declares disallowed types and methods for unordered/nondeterministic or environment-dependent APIs. `.github/workflows/ci.yml:1` runs format, clippy with `-D warnings`, locked build, and workspace tests. `rust-toolchain.toml:1` pins the toolchain to `1.93.0` with `rustfmt` and `clippy` components.

**Assessment:** This is materially better than grep. It still needs a negative fixture runner that proves the exact pinned toolchain catches aliases, re-exports, macro calls, and method references the project is worried about. Otherwise future maintainers have to trust clippy config by name.

### F-012 — Invariant-reference linter catches dangling refs only

**Status:** needs hardening  
**Layer:** core/tests + docs/spec discipline  
**Invariants:** `INV-102`, `INV-105`, `INV-092`  
**Evidence:** `crates/tracewake-core/tests/doc_invariant_references.rs:1` scans selected live reference paths, builds a defined invariant set from `02_CONSTITUTIONAL_INVARIANTS.md`, recognizes `INV-###` references, and fails when a referenced invariant is not defined.

**Assessment:** Useful, but insufficient. It does not detect a finding with no invariant tag, a requirement with a valid but semantically stale invariant tag, a high-risk invariant abandoned by all tests/specs, or a miscitation that points to the wrong valid invariant. Full semantic correctness cannot be automated cheaply, but structured metadata can require every finding/requirement/test guard to carry invariant IDs and can produce orphan/staleness reports for reviewer signoff.

### F-013 — Spine and TUI conformance capstones prove named-test presence, not property satisfaction

**Status:** needs hardening  
**Layer:** core/tests + TUI/tests + workspace/ci  
**Invariants:** `INV-091`, `INV-092`, `INV-105`  
**Evidence:** `crates/tracewake-core/tests/spine_conformance.rs:1` defines `SpineEvidence` and `SPINE_EVIDENCE`, then checks requirement IDs, layer IDs, and whether the named test function string exists in the referenced file. `crates/tracewake-tui/tests/tui_seam_conformance.rs:1` uses the same named-test-existence style for TUI seam evidence.

**Assessment:** This is a useful review ledger, but it cannot distinguish a rigorous negative test from a no-op function with the right name. It must become an evidence matrix with evidence kinds, negative/positive classification, and per-requirement acceptance criteria.

### F-014 — CI runs the important gates on the pinned toolchain

**Status:** already satisfied; preserve and extend  
**Layer:** workspace/ci  
**Invariants:** `INV-017`, `INV-018`, `INV-091`, `INV-092`, `INV-105`  
**Evidence:** `.github/workflows/ci.yml:1` runs `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, and `cargo test --workspace --locked`; `rust-toolchain.toml:1` pins Rust and includes clippy/rustfmt.

**Assessment:** The CI skeleton is correct. This spec requires adding the lock-negative fixture runner and ensuring no new gate escapes the pinned toolchain.

---

## 7. Prior-gate re-verification

### 7.1 TUI proof surface hardening (`TUI-AC-001…012`)

**Verdict:** No material regression found in the audited TUI seam. The TUI pass remains accepted for its scope, with one third-pass caveat: its conformance mapping is still string/named-test based and should be upgraded by this spec’s lock-layer requirements.

| Prior gate family | Re-verification result | Evidence | Invariants |
|---|---:|---|---|
| Embodied view uses holder-known context, not debug truth | Holds | `crates/tracewake-tui/src/app.rs:1` builds `KnowledgeContext::embodied_at_frontier`, `EmbodiedProjectionSource::from_sealed_context`, and `build_embodied_view_model`; `crates/tracewake-tui/tests/embodied_flow.rs:1` checks holder-known context ID/hash and absence of hidden truth strings. | `INV-067`, `INV-069`, `INV-093`, `INV-095`, `INV-100`, `INV-101` |
| TUI submits semantic actions, not direct mutations | Holds | `crates/tracewake-tui/src/input.rs:1` parses `SemanticActionId`; `crates/tracewake-tui/src/app.rs:1` resolves semantic action in current view and calls `run_pipeline`. | `INV-008`, `INV-009`, `INV-104` |
| Debug panels are non-diegetic | Holds | `crates/tracewake-core/src/debug_capability.rs:1` and `debug_reports.rs:1` require debug capability; `crates/tracewake-tui/src/debug_panels.rs:1` renders debug separately. | `INV-068`, `INV-107` |
| Transcript stability exists | Holds | `crates/tracewake-tui/src/transcript.rs:1` captures representative sections; `crates/tracewake-tui/tests/transcript_snapshot.rs:1` locks snapshot behavior. | `INV-092`, `INV-095` |
| TUI conformance capstone | Partially holds | `crates/tracewake-tui/tests/tui_seam_conformance.rs:1` maps requirements to named tests but only checks `source.contains("fn …(")`. | `INV-091`, `INV-092`, `INV-105` |

### 7.2 Spine hardening (`SPINE-AC-001…015`)

| Prior gate | Re-verification result | Evidence | Third-pass action |
|---|---:|---|---|
| `SPINE-AC-001` authoritative state fields are not publicly mutable | **Partial** | Fields are `pub(crate)` in `state.rs:1`, but public `seed_*_mut` methods return mutable maps. | Harden via `THIRD-AC-001` and `THIRD-AC-002`. |
| `SPINE-AC-002` schema-version/migrator registration | Holds | Event schema registry and append/apply schema validation in `events/envelope.rs:1`, `events/log.rs:1`, `events/apply.rs:1`. | Add negative unsupported-version fixtures via `THIRD-AC-008`. |
| `SPINE-AC-003` event metadata totality / stream behavior | Holds | Event metadata and stream handling in `events/envelope.rs:1` and `events/apply.rs:1`. | Preserve. |
| `SPINE-AC-004` checksum coverage | Partial | Coverage registries exist in `checksum.rs:1`; field-to-registry parity remains manual. | Harden via `THIRD-AC-007`. |
| `SPINE-AC-005` nondeterminism API ban | Partial | Token scanner exists in `anti_regression_guards.rs:1`; stronger clippy config exists in `clippy.toml:1`; adversarial negative clippy fixtures missing. | Harden via `THIRD-AC-003` and `THIRD-AC-004`. |
| `SPINE-AC-006` scheduler no-direct-dispatch | Holds for scheduler | `scheduler.rs:1` uses proposal ordering and no-human execution paths. | Preserve; add direct-state negative tests. |
| `SPINE-AC-007` proposal source context / stale context | Holds | `actions/proposal.rs:1` carries source view-model and holder-known context IDs/hashes. | Preserve. |
| `SPINE-AC-008` append-before-apply and direct apply guard | Partial | Pipeline append/apply path exists in `actions/pipeline.rs:1`; direct apply scanner is substring-based and public seed mutators bypass event path. | Harden via `THIRD-AC-002`. |
| `SPINE-AC-009` actor-known/debug firewall | Holds | `debug_capability.rs:1`, `debug_reports.rs:1`, `hidden_truth_gates.rs:1`, TUI adversarial gates. | Preserve and add compile-fail debug construction test. |
| `SPINE-AC-010` content no-script/schema guard | Holds with brittle field coverage | `validate.rs:1`, `forbidden_content.rs:1`, `serialization.rs:1`. | Harden field coverage via `THIRD-AC-009`. |
| `SPINE-AC-011` typed diagnostics | Holds | `actions/report.rs:1` uses typed reason/provenance. | Preserve and prevent display-label regression. |
| `SPINE-AC-012` TUI seam | Holds | `app.rs:1`, `input.rs:1`, `embodied_flow.rs:1`, TUI adversarial tests. | Preserve; conformance map upgrade under `THIRD-AC-006`. |
| `SPINE-AC-013` conformance suite | Partial | `spine_conformance.rs:1` maps requirements to named tests only. | Harden via `THIRD-AC-006`. |
| `SPINE-AC-014` invariant-reference linter | Partial | `doc_invariant_references.rs:1` catches dangling refs only. | Harden via `THIRD-AC-005`. |
| `SPINE-AC-015` acceptance artifact | Holds | `spine_conformance.rs:1` checks acceptance template phrases. | Preserve, but require no-overclaim wording in `THIRD-AC-012`. |

---

## 8. Structural hardening requirements

### THIRD-AC-001 — Replace public seed mutators with seed-build capability

**Enforces:** `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-092`, `INV-101`, `INV-104`  
**Responsible layer:** core/state + content/load  
**Required change:** Remove public `seed_*_mut` methods from `PhysicalState` and `AgentState`, or change them so they require a non-public `SeedBuildCapability` / `SeedBuilder` that cannot be minted by TUI, content consumers, or downstream crates after initialization.

**Structural enforcement:**

- State fields remain private or `pub(crate)`.
- Post-seed state mutation requires `WorldMutationCapability` / `AgentMutationCapability` from event application.
- Seed-time construction uses `PhysicalSeedBuilder` / `AgentSeedBuilder` or `from_seed_parts` with fully constructed ordered maps.
- Any seed-build capability is minted only by core fixture/load conversion functions or a tightly scoped content loader API, never exported as a general public constructor.
- `PhysicalState::from_seed_parts` / `AgentState::from_seed_parts` already exist and are the canonical seed-construction path `tracewake-content` uses (`crates/tracewake-content/src/schema.rs:398`, `:494`); the remediation reuses them (or a thin test-only seed builder over them) rather than inventing a parallel mechanism. The only current `seed_*_mut` call sites are `tracewake-core`'s integration tests (`golden_scenarios.rs`, `acceptance_gates.rs`, `no_human_capstone.rs`, `hidden_truth_gates.rs`, `anti_regression_guards.rs`), which must be migrated off the removed mutators as part of this change so the suite stays green.

**Failure/drift prevented:** Direct world/agent mutation outside event append/apply and replay accounting.

### THIRD-AC-002 — Make no-direct-mutation compile-time, not scanner-only

**Enforces:** `INV-009`, `INV-011`, `INV-018`, `INV-101`, `INV-104`, `INV-106`  
**Responsible layer:** core/events + core/actions + workspace negative fixtures  
**Required change:** All mutating state methods outside seed construction must require event mutation capabilities. Direct apply scanners remain smoke tests only.

**Structural enforcement:**

- External crates cannot construct or call mutation capabilities.
- `apply_event`, `apply_event_stream`, and pipeline commit remain the only non-seed mutation path.
- A negative fixture crate attempts: field mutation, `seed_*_mut` access, forged capability construction, and direct event application from a forbidden module. The expected outcome is compile failure or clippy failure, depending on the case.

**Failure/drift prevented:** Ad hoc “quick mutation” helpers, TUI/test shortcuts masquerading as state transitions, and scheduler/routine direct dispatch.

### THIRD-AC-003 — Reclassify token scanners as smoke guards and harden their walk/match behavior

**Enforces:** `INV-017`, `INV-018`, `INV-092`, `INV-105`  
**Responsible layer:** core/tests + workspace/ci  
**Required change:** `anti_regression_guards.rs` must state in code comments and conformance metadata that substring/token scanning is smoke-only. Its source walk must either cover every production root (`core/src`, `content/src`, `tui/src`) or list intentionally excluded roots with rationale.

**Structural enforcement:**

- Scanner walks new files recursively from manifest/workspace roots, not hand-maintained include strings as sole coverage.
- Scanner ignores comments/strings or explicitly labels comment/string sensitivity as smoke-only.
- Scanner does not truncate production code at the first `#[cfg(test)]` if non-test production items can appear later in the same file.
- Scanner has tests proving it discovers a newly added source file in a nested module tree.

**Failure/drift prevented:** A new production file or rearranged `cfg(test)` boundary silently evading the scan.

### THIRD-AC-004 — Add clippy-negative fixtures for banned nondeterminism and environment APIs

**Enforces:** `INV-017`, `INV-018`, `INV-092`, `INV-105`  
**Responsible layer:** workspace/ci + clippy policy  
**Required change:** Add an isolated negative-fixture runner that invokes the pinned toolchain’s clippy on intentionally bad snippets/crates and asserts failure for each banned API family. The runner shells out via `std::process::Command` from a dedicated test target or `xtask` harness, reusing the existing `#[allow(clippy::disallowed_methods, reason = …)]` + `Command::new` pattern already used by `crates/tracewake-tui/tests/command_loop_session.rs:9` and `readme_sample_session.rs:79`. It must not introduce a new dependency such as `trybuild` or `compiletest`, preserving `tracewake-core`’s zero-dependency posture and the workspace `clippy.toml` policy.

**Structural enforcement:**

- Fixtures run under `rust-toolchain.toml` and the root `clippy.toml` policy.
- CI has a dedicated job or test harness that expects each negative fixture to fail.
- Required fixture families: direct type path, imported alias, re-export, type alias, macro expansion, function pointer / higher-order method reference where applicable, and nested module use.
- Negative fixture output must assert the lint category (`disallowed_types`, `disallowed_methods`, or configured companion) rather than only non-zero status.

**Failure/drift prevented:** False confidence in `clippy.toml`; aliases/re-exports/macros silently passing despite policy intent.

### THIRD-AC-005 — Upgrade invariant-reference linter to structured coverage linter

**Enforces:** `INV-092`, `INV-102`, `INV-105`  
**Responsible layer:** docs/spec tests + workspace conformance  
**Required change:** Keep dangling-reference detection, but add structured checks that every live spec finding, every live spec requirement, and every conformance guard declares one or more invariant IDs in a machine-detectable field.

**Structural enforcement:**

- Markdown headings or metadata blocks use stable fields: `Invariants: INV-###, INV-###` and `Layer: ...`.
- Missing invariant tags fail tests.
- Unknown invariant IDs fail tests.
- A generated report lists unreferenced high-risk invariants and references whose target invariant was renamed or removed.
- Semantic correctness of “right invariant number” remains reviewer-verified, but the linter forces a review note when a finding cites only broad or unrelated invariants.

**Failure/drift prevented:** Findings/requirements without invariant grounding; dangling refs hidden by valid-looking but stale IDs.

### THIRD-AC-006 — Turn conformance capstones into evidence-kind matrices

**Enforces:** `INV-091`, `INV-092`, `INV-105`  
**Responsible layer:** core/tests + content/tests + TUI/tests + CI  
**Required change:** Replace “function name exists” as the sole proof with a conformance matrix that records evidence kind, positive/negative class, responsible layer, invariants, and acceptance condition.

**Structural enforcement:**

- Each `SPINE-AC-*` and TUI seam entry must have at least one evidence item of a strong kind: `CompileTime`, `RuntimeNegative`, `ReplayDeterminism`, `SchemaRejected`, `ChecksumParity`, `CapabilityBoundary`, or `CIGate`.
- Named-test existence may remain as an index, but cannot be the only evidence for any high-risk requirement.
- A conformance test fails if an entry is `StringPresenceOnly` for event sourcing, no-direct-dispatch, determinism, debug quarantine, content no-script, or typed diagnostics.

**Failure/drift prevented:** No-op tests or renamed placeholders satisfying conformance by string shape.

### THIRD-AC-007 — Add checksum field-to-registry parity

**Enforces:** `INV-018`, `INV-092`, `INV-105`  
**Responsible layer:** core/state + core/checksum  
**Required change:** Add a guard ensuring every authoritative `PhysicalState` and `AgentState` field is represented in checksum coverage and canonical line emission.

**Structural enforcement:**

- Prefer a single-source registry/macro that defines authoritative state fields and checksum coverage together.
- If macros are rejected, add a test that parses or otherwise enumerates field names and compares them to `PHYSICAL_STATE_CHECKSUM_COVERAGE`, `AGENT_STATE_CHECKSUM_COVERAGE`, and canonical line emission labels.
- Adding a new authoritative field without checksum coverage must fail.

**Failure/drift prevented:** Replay/checksum equality ignoring newly added authoritative state.

### THIRD-AC-008 — Add unsupported schema-version and non-world leakage negative fixtures

**Enforces:** `INV-009`, `INV-011`, `INV-018`, `INV-020`, `INV-092`  
**Responsible layer:** core/events + core/replay  
**Required change:** Expand event/replay negative gates to include forged future event schema versions, unsupported payload schema versions, stream mismatches, and non-world stream events attempting to change physical checksum.

**Structural enforcement:**

- Fixture for unsupported schema append rejection.
- Fixture for unsupported schema replay rejection.
- Fixture for non-world event in replay not changing physical checksum.
- Fixture for stream mismatch refusal in apply/replay.

**Failure/drift prevented:** Silent migration drift, replay accepting unknown data, and debug/control events mutating world state.

### THIRD-AC-009 — Make content field coverage typed, not source-string-only

**Enforces:** `INV-022`, `INV-060`, `INV-061`, `INV-097`, `INV-102`, `INV-105`  
**Responsible layer:** content/schema + content/validation + content/serialization  
**Required change:** Replace or supplement source-string field checks with a typed schema field registry that drives validation and canonical serialization coverage.

**Structural enforcement:**

- Every schema field appears in a typed registry with `validation_phase`, `canonical_serialization_key`, and `forbidden_construct_policy` metadata.
- Validation and serialization consume the registry or are generated from it.
- Adding a field without validation/canonical metadata fails.
- Existing string scanner remains only as a smoke guard.

**Failure/drift prevented:** New content fields becoming prose-born facts, shortcut truth, authored outcomes, or nondeterministic serialization gaps.

### THIRD-AC-010 — Add debug quarantine negative tests

**Enforces:** `INV-008`, `INV-067`, `INV-068`, `INV-069`, `INV-093`, `INV-094`, `INV-095`, `INV-099`, `INV-100`, `INV-107`, `INV-108`  
**Responsible layer:** core/debug + core/view_models + TUI  
**Required change:** Add negative tests proving debug truth cannot construct embodied view facts, actor-known context, affordances, or TUI actions.

**Structural enforcement:**

- Compile-fail fixture attempts to construct debug report without `DebugCapability`.
- Runtime negative fixture opens debug panel, then proves embodied affordances and holder-known context hash remain unchanged.
- TUI transcript fixture labels debug sections non-diegetic and prevents debug-only facts from appearing in actor-visible why-not.

**Failure/drift prevented:** Debug truth leaking into embodied cognition, possession, view models, or tests masquerading as actor knowledge.

### THIRD-AC-011 — Lock CI to run the strengthened gates

**Enforces:** `INV-017`, `INV-018`, `INV-091`, `INV-092`, `INV-105`  
**Responsible layer:** workspace/ci  
**Required change:** Extend CI to run the negative fixture runner, conformance evidence matrix, invariant coverage linter, checksum parity guard, and content typed field coverage guard under the pinned toolchain.

**Structural enforcement:**

- The existing four workspace gates remain mandatory.
- New lock-layer gates are run in CI and documented in the acceptance artifact.
- No gate may require an unpinned external service, network access during simulation outcome resolution, or a tool unavailable under `rust-toolchain.toml`.

**Failure/drift prevented:** Hardened guards existing locally but not failing pull-request or branch integration.

### THIRD-AC-012 — Acceptance artifact must not overclaim

**Enforces:** `INV-091`, `INV-092`, `INV-105`  
**Responsible layer:** docs/spec acceptance + CI artifact  
**Required change:** Acceptance wording must say: “Phase 1 / Phase 1A third hardening and lock-layer remediation accepted for this commit.” It must not claim latest main, project-wide certification, or later-phase certification.

**Structural enforcement:**

- Acceptance artifact names exact commit under test.
- Artifact lists gates run and exact output summaries.
- Artifact records residual convention-only items.
- Artifact includes forbidden wording checks for “full project certified,” “latest main verified,” or equivalent overclaims.

**Failure/drift prevented:** Historical scoped evidence being promoted into global authority.

---

## 9. Required fixtures and tests

**Fixture mechanism.** The "compile-fail" and clippy-negative fixtures below are realized as isolated fixture crates compiled/linted by a subprocess harness (a dedicated test target or `xtask`) that asserts the expected build/lint failure, reusing the established `#[allow(clippy::disallowed_methods, reason = …)]` + `std::process::Command` pattern (`crates/tracewake-tui/tests/command_loop_session.rs:9`, `readme_sample_session.rs:79`). No new test dependency (`trybuild`, `compiletest`, etc.) is introduced; `tracewake-core` stays zero-dependency.

### 9.1 Direct mutation / capability fixtures

- `external_crate_cannot_call_seed_mutators_after_load`: compile-fail fixture importing `tracewake_core::state::PhysicalState` and attempting `state.seed_items_mut().insert(...)`.
- `external_crate_cannot_mutate_agent_state_seed_maps`: compile-fail fixture attempting `AgentState::seed_intentions_mut` outside seed-builder scope.
- `external_crate_cannot_forge_world_mutation_capability`: compile-fail fixture attempting to construct `WorldMutationCapability` or `AgentMutationCapability`.
- `event_apply_remains_only_post_seed_mutation_path`: runtime/conformance test proving accepted actions append before authoritative apply and no forbidden module invokes apply directly.

### 9.2 Banned API / nondeterminism negative fixtures

Each fixture must be expected to fail under `cargo clippy --workspace --all-targets -- -D warnings` or the isolated negative fixture runner:

- `banned_hashmap_direct_path_fails`.
- `banned_hashmap_import_alias_fails`.
- `banned_hashset_reexport_fails`.
- `banned_systemtime_alias_fails`.
- `banned_instant_alias_fails`.
- `banned_thread_spawn_direct_fails`.
- `banned_thread_spawn_reexport_fails`.
- `banned_fs_read_and_file_open_fail`.
- `banned_tcp_udp_network_calls_fail`.
- `banned_process_command_new_fails`.
- `banned_macro_expands_to_spawn_or_fs_fails`, with a clear note if clippy cannot catch a macro form and the scanner or a disallowed-macro policy is needed.
- `source_guard_discovers_new_nested_production_file`.
- `source_guard_does_not_silently_skip_production_after_cfg_test`.

### 9.3 Invariant and conformance lock fixtures

- `spec_finding_without_invariants_fails_linter`.
- `spec_requirement_without_invariants_fails_linter`.
- `undefined_invariant_reference_fails_linter`.
- `conformance_entry_without_evidence_kind_fails`.
- `conformance_entry_string_presence_only_rejected_for_high_risk_gate`.
- `conformance_matrix_requires_negative_evidence_for_no_direct_and_debug_quarantine`.

### 9.4 Replay / checksum / schema fixtures

- `new_authoritative_field_without_checksum_registry_fails`.
- `new_authoritative_field_without_canonical_checksum_line_fails`.
- `unsupported_event_schema_append_rejected`.
- `unsupported_event_schema_replay_rejected`.
- `stream_mismatch_replay_rejected`.
- `non_world_event_cannot_change_physical_checksum`.
- `replay_rebuild_checksum_matches_original_after_no_human_day`.

### 9.5 Content negative fixtures

- `content_quest_objective_reward_ids_rejected`.
- `content_player_protagonist_culprit_director_ids_rejected`.
- `content_prose_born_fact_rejected`.
- `content_shortcut_truth_fields_rejected`.
- `content_new_field_requires_typed_validation_and_canonical_serialization_metadata`.
- `content_serialization_is_canonical_independent_of_authoring_order`.

### 9.6 TUI / debug negative fixtures

- `debug_report_construction_without_capability_compile_fails`.
- `debug_panel_does_not_change_embodied_affordances`.
- `debug_truth_never_enters_holder_known_context_hash`.
- `debug_command_strings_are_not_embodied_commands`.
- `tui_current_view_submission_rejects_stale_selection`.
- `tui_transcript_snapshot_remains_byte_stable`.
- `tui_transcript_marks_debug_sections_non_diegetic`.

---

## 10. Acceptance checklist

A remediation implementing this spec is accepted only when all conditions below are met.

### 10.1 Mandatory workspace gates

The acceptance artifact must report these exact commands and successful results:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

### 10.2 New lock-layer gates

The acceptance artifact must additionally report:

- Negative fixture runner for clippy/banned API aliases, re-exports, macros, and direct paths.
- Compile-fail or isolated crate fixtures for seed-mutation and capability-forgery attempts.
- Invariant coverage linter with dangling-reference and missing-invariant checks.
- Conformance evidence matrix gate with evidence kinds and high-risk negative-evidence requirements.
- Checksum field-to-registry parity guard.
- Content typed field coverage guard.
- Debug quarantine negative tests.
- Unsupported schema-version / stream mismatch replay gates.

### 10.3 Per-requirement acceptance conditions

| Requirement | Acceptance condition |
|---|---|
| `THIRD-AC-001` | No public post-seed mutable map access remains; negative fixture cannot call seed mutators from outside permitted seed-builder scope. |
| `THIRD-AC-002` | Event capabilities are required for post-seed mutation; direct mutation and forged capability fixtures fail. |
| `THIRD-AC-003` | Scanner is documented as smoke-only and walks all intended production roots; new-file discovery test passes. |
| `THIRD-AC-004` | Clippy-negative fixtures prove banned type/method policy under pinned toolchain. |
| `THIRD-AC-005` | Every live finding/requirement/guard has machine-detectable invariant IDs; missing/unknown IDs fail. |
| `THIRD-AC-006` | Conformance matrix requires evidence kind and rejects string-presence-only proof for high-risk gates. |
| `THIRD-AC-007` | Authoritative state fields and checksum coverage/canonical emission remain in parity. |
| `THIRD-AC-008` | Unsupported schema and stream mismatch fixtures reject append/replay/apply as appropriate. |
| `THIRD-AC-009` | New content fields require typed validation and canonical serialization metadata. |
| `THIRD-AC-010` | Debug reports cannot be forged and debug facts cannot alter embodied affordances or holder-known context. |
| `THIRD-AC-011` | CI runs the strengthened gates on the pinned toolchain. |
| `THIRD-AC-012` | Acceptance artifact uses scoped wording only and does not overclaim certification. |

### 10.4 Certification wording

Accepted remediation may say:

> Phase 1 / Phase 1A third hardening and lock-layer remediation accepted for exact commit `<commit>`. This contributes scoped evidence toward `SPINE-CERT`, `EPI-CERT`, and `P0-CERT`; it does not certify latest main, later-phase scope, or the full project.

Accepted remediation must not say or imply:

- “Tracewake is fully certified.”
- “Latest main was independently verified.”
- “Later Phase 2+ / Phase 3A+ systems are certified by this pass.”
- “Archived specs are live authority.”

---

## 11. Additional suggestions appendix

These are suggestions for the maintainer, not in-scope acceptance requirements for this spec.

### A. What predecessor suggestions landed

The prior spine pass’s strongest suggestions did materially land:

- Root `clippy.toml` disallowed-type / disallowed-method policy exists.
- CI runs clippy with `-D warnings`, locked build, tests, and formatting.
- `spine_conformance.rs` and `tui_seam_conformance.rs` exist as conformance indexes.
- `doc_invariant_references.rs` catches dangling invariant references.
- `anti_regression_guards.rs` scans production core sources recursively and checks nondeterministic tokens.
- Content forbidden-construct tests reject prose-born facts and reserved story/director vocabulary.

### B. What remains open across blocks

1. **Workspace-wide conformance crate.** Move architecture/lock-layer tests that read multiple crates into a `tracewake-conformance` crate or `xtask`-style harness so cross-crate policy is explicit and not housed inside `tracewake-core` by accident.

2. **Public API boundary audit.** Add a small public API snapshot or rustdoc-based check to detect newly public state mutation methods, debug constructors, alias paths, or capability mints.

3. **Single-source registries.** Use macro- or data-driven registries for event kinds, schema fields, checksum coverage, and conformance evidence to reduce source-string drift.

4. **Cargo metadata dependency-direction guard.** Keep core zero-dependency and one-way crate direction mechanically checked from `cargo metadata`, not only by review.

5. **Policy-negative fixture runner.** Generalize the banned-API negative fixture runner so future blocks can add “this must fail” examples for truth leakage, scripting fields, LLM leakage, institutions, and later simulation seams.

6. **Invariant semantic review report.** The upgraded invariant linter can force coverage, but semantic “right invariant” correctness remains human. Generate a reviewer-facing table of finding → invariant → reason to make miscitation review cheap and consistent.

---

## 12. External prior-art notes

External sources shaped the direction of the hardening requirements only insofar as they support existing Tracewake doctrine:

- Event sourcing stores state changes as event sequences and supports reconstructing prior/current state by replaying those events. This supports strengthening the no-direct-mutation rule rather than weakening the event spine.[^fowler-event-sourcing]
- Rust’s standard `HashMap` default hasher is randomly seeded from host-provided randomness, supporting the ban on `HashMap` / `HashSet` in deterministic outcome paths.[^rust-hashmap]
- Clippy supports configured `disallowed-methods` and `disallowed-types` using fully qualified paths, supporting the move from token scanning to lint-backed policy plus negative fixtures.[^clippy-disallowed]

[^fowler-event-sourcing]: Martin Fowler, “Event Sourcing,” https://martinfowler.com/eaaDev/EventSourcing.html
[^rust-hashmap]: Rust standard library documentation, `std::collections::HashMap`, https://doc.rust-lang.org/beta/std/collections/struct.HashMap.html
[^clippy-disallowed]: Clippy lint configuration documentation, `disallowed-methods` / `disallowed-types`, https://doc.rust-lang.org/clippy/lint_configuration.html

---

## 13. Self-check result

- Exact-commit discipline followed: yes.
- Manifest used only as path inventory: yes.
- Branch/default/repo metadata/code search/clone used: no.
- Contamination observed: no.
- Third spec form matches verdict: yes, positive hardening spec.
- Material-risk bar applied: yes; cosmetic issues relegated to suggestions.
- Lock layer audited: yes, including conformance capstones, invariant-reference linter, token scanner, `clippy.toml`, content guards, and CI.
- Prior gates re-verified: yes; `TUI-AC` holds for scope, `SPINE-AC` mostly holds with specific partials requiring this spec.
- Findings and requirements cite invariants: yes.
- No later-block implementation advanced: yes.
- Workspace gates stated and run during implementation closeout: yes.
