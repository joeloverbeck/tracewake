# TUI Human Wait Runs Simulation — Research and Recommendation Report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `6e91da79b81bf9cbf25acba726e58183920dc441`  
**Deliverable type:** consolidated analysis and recommendation report  
**Campaign posture:** post-`FIRST-PROOF-CERT` capability work; not a certification audit  
**Prepared:** 2026-06-22

> **Provenance limitation.** This report does not verify that the target commit is the current `main`. It uses the user-supplied commit as the target of record and makes claims about repository state only from manifest-listed files fetched at full exact-commit URLs. References to other repositories inside those files are treated as file content, not as provenance contamination.

## Executive determination

The defect is real, narrowly diagnosable, and cross-cutting. At the target commit, the action layer can start `sleep` and `work_block` durations, and the no-human scheduler can complete them, reconcile their per-tick need effects, and emit terminal events. The human TUI path cannot do so. `TuiApp::submit_entry` runs one action pipeline transaction and then adopts the last emitted event tick; `UiCommand::WaitOneTick` merely selects the ordinary `wait` action. Neither path invokes a canonical authoritative world step. The only function named `DeterministicScheduler::advance_one_tick` advances an in-memory counter and performs no simulation work. The completion engine inside `run_no_human_day` is not reusable by an independently started human duration because it relies on batch-local pending-start vectors.

The single recommended solution is to introduce a **kernel-owned authoritative one-world-tick coordinator** and make every time-advancing mode loop that same primitive. Human `wait` advances one full authoritative loaded-world tick, not a possessed-actor-only clock. Sleep and work use an **advance-until** controller policy that repeatedly executes the same one-tick coordinator until the possessed actor’s duration terminates, an actor-known salient interruption is processed, the user pauses, or a deterministic safety bound is reached. The TUI submits typed intent to this core boundary; it never calls completion builders, mutates state, or performs accounting itself.

The coordinator must discover open durations from the shared event-log authority, not from a no-human-run-local queue; emit or operationalize the existing `TimeAdvanced` event kind so even otherwise empty ticks have replayable temporal ancestry; route all actor actions through the ordinary proposal pipeline; and reconcile every `(actor, need, tick)` exactly once. An ordinary `wait` action supplies the possessed actor’s action/charge evidence when that actor is free. While a body-exclusive duration is open, `wait` is not another actor action: the human uses a typed continuation control. Other embodied actions reject with the existing reservation-conflict mechanism unless explicitly permitted as lifecycle interruption/cancellation controls.

The world-tick fork resolves in favor of a **full authoritative world tick**. All loaded actors and due world processes advance under one timeline. Possession determines who supplies one actor’s decision; it does not freeze everyone else. This does not require every actor to replan every tick: only actors and processes whose deterministic cadence says they are due receive an action opportunity. The possessed actor learns about the interval only through a typed **actor-known interval summary** derived from that actor’s holder-known frontier, own embodied effects, modeled messages/records/observations, and fresh resumption perception. A raw event-log diff or global “nothing happened” claim is forbidden.

No constitutional invariant needs amendment. Existing temporal authority, replay, possession-neutrality, survival, single-charge, and truth-firewall doctrine is sufficient. The ambiguity should be operationalized in foundation `08`, architecture `02`/`03`/`04`/`10`/`13`, execution `05`/`06`/`07`/`10`, and the reference checklist/risk register. The body-exclusive defect belongs in the same implementation recommendation because continuation semantics are not coherent while an overlapping `wait` remains accepted.

The next implementation vehicle should be a new post-certification numbered spec scoped as **“TUI Authoritative World Advance, Duration Completion, and Actor-Known Interval Summaries.”** The repository’s allocator should assign its number. It must extend the spec-0046 parity registry and real-pipeline witnesses to duration completion and time-control summaries rather than creating a side channel.

---

## 1. Disposition table

| Finding | Target home or seam | Verdict | Basis |
|---|---|---|---|
| D-01 | `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` | **No amendment** | Existing authoritative-time, replay, possession-neutrality, survival, parity, truth-firewall, and temporal-authority invariants can bear the feature without changing constitutional meaning. |
| D-02 | `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **Time controls** | **Amend** | The doctrine says waiting runs the simulation but does not settle full-world tick scope, action-versus-continuation semantics, or the source boundary for interval summaries. |
| D-03 | Foundation `03`, `05`, `06`, `12`, `14` | **Retain; cross-reference only if useful** | They already require authoritative event time, causal durations, survival recovery, first-playable ordinary life, scheduler-owned completions, and actor-known cognition. |
| D-04 | `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — temporal rendering/play loop | **Amend** | Make the typed time-control → core world-step boundary, repeated-step acceleration, and actor-known interval projection explicit. |
| D-05 | `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | **Amend** | Assign ownership of the reusable one-tick coordinator, log-derived duration discovery, deterministic phase order, and reservation lifecycle. |
| D-06 | `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | **Amend** | Require replay-visible tick ancestry for otherwise empty world steps and deterministic reconstruction of interval summaries and stop decisions. |
| D-07 | `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | **Amend** | Define interval summaries as holder-known frontier deltas and source-bearing projections, never raw world deltas. |
| D-08 | `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | **Correct/extend conformance mapping** | Human time controls must be named consumers of the existing tick-charge and shared-open-duration authority rows. |
| D-09 | `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | **Add proof obligations** | The feature needs one-tick equivalence, duration terminal, replay, anti-leak, possession, and parity evidence. |
| D-10 | `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | **Amend** | Specify the canonical step mechanics and prohibit TUI-side completion/dispatch. |
| D-11 | `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | **Amend** | Extend single-charge and duration proofs to human-driven ticks and human/no-human differential witnesses. |
| D-12 | `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | **Correct** | Its current statement that embodied time controls remain unavailable is superseded by the feature routed here; retain debug quarantine while adding embodied proof. |
| D-13 | `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | **Add evidence matrix** | Freeze event ancestry, stop reason, replay, no-hidden-truth, and parity evidence without relying on prose. |
| D-14 | `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | **Add concise checklist prompts** | Review should ask whether human time controls advance the same world step, preserve one charge, and summarize only actor-known changes. |
| D-15 | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | **Update existing risks; no new ID** | The work directly touches pipeline bypass, epistemic leakage, replay erosion, TUI playability, no-human parity, and temporal-control relapse. |
| D-16 | `docs/3-reference/02_GLOSSARY.md` | **No change** | Existing terms are sufficient; the implementation need not mint a new canonical domain term. |
| D-17 | `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | **No structural change** | Its existing parity/evidence sections can package this feature once the future spec defines concrete witnesses. |
| D-18 | `docs/4-specs/SPEC_LEDGER.md` | **Route forward, not amend now** | Add the future spec only through the repository’s normal allocation/acceptance process; this report does not mint a number or verdict. |
| C-01 | `crates/tracewake-core/src/scheduler.rs` | **New core seam/refactor** | Extract a reusable authoritative one-tick coordinator; make no-human and human advancement loop it. |
| C-02 | `scheduler.rs` + `need_accounting.rs` + `events/envelope.rs` | **Correct duration authority** | Replace batch-local pending-start completion discovery with the shared log-derived open-duration authority; preserve duplicate/orphan fail-closed behavior. |
| C-03 | `wait.rs` + ordinary-life accounting seam | **Integrate, do not duplicate** | The accepted wait action may cover the possessed actor’s tick; passive and duration accounting must reconcile around the same per-tick ledger. |
| C-04 | `actions/pipeline.rs` reservation check | **Correct** | Reject ordinary embodied actions during an open body-exclusive duration; allow only explicit lifecycle continuation/interruption controls. |
| C-05 | `crates/tracewake-tui/src/app.rs` and `run.rs` | **New wiring** | Route `wait` and advance-until controls to the core coordinator; do not call `run_no_human_day` or terminal builders from the TUI. |
| C-06 | `view_models.rs`, `projections.rs`, `render.rs` | **Add actor-known surface** | Expose a typed interval summary with exhaustive renderer disposition and a structurally separate debug advance report. |
| C-07 | spec-0046 parity tests and TUI acceptance tests | **Extend** | Existing coverage proves action starts and 21 playable surfaces, not human-path duration completion or interval-summary anti-leak behavior. |
| FWD-01 | New numbered implementation spec | **Route forward** | Cross-cutting post-certification feature work needs an owned spec and reviewable ticket sequence, not an ad hoc patch or cert remediation. |

---

## 2. Method and provenance ledger

### 2.1 Evidence lanes and authority order

The analysis kept three evidence lanes separate:

1. **Target-repository evidence.** Claims about `joeloverbeck/tracewake` at the target commit use only paths present in the uploaded manifest and files fetched through verified exact-commit URLs.
2. **User-supplied control material.** The manifest served only as path inventory. The uploaded research brief supplied scope, authority order, settled intentions, and deliverable constraints. Neither was used as a substitute for repository file contents.
3. **External research.** External sources informed design comparison only. They were not used to assert what exists in the target repository or to replace a failed repository acquisition.

Repository reading followed the declared authority order: `docs/README.md`; the constitution; the complete live foundation tier; the complete live architecture tier; the complete live execution tier; reference and current-spec boundaries; the spec-0046 precedent and acceptance artifact; the seed issue; then implementation and test seams. Boundary files were read to test tier fit, not presumed amendment targets.

### 2.2 Mandatory acquisition ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 6e91da79b81bf9cbf25acba726e58183920dc441
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.open(full exact URL), followed where supported by container.download(full exact URL); exact-commit GitHub blob view used as an alternate representation for derived-type-restricted Rust responses
Requested unique file count: 87
Requested URL ledger entries: 95
Successfully verified unique file count: 87
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

Every selected path appeared exactly in the uploaded manifest. Each request contained the exact owner, repository, full commit, and manifest path. The responses were file payloads or exact-commit blob views, not repository overviews, branch pages, search snippets, or metadata substitutes. Eight blob URLs duplicate raw-file acquisitions where an alternate view was used to verify content or work around a downloader’s derived MIME allowlist; they do not increase the unique-file count.

<details>
<summary>Complete append-only requested-URL ledger (95 entries)</summary>

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/README.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/00_FOUNDATION_INDEX.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/01_PROJECT_CHARTER.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/3-reference/01_DESIGN_RISK_REGISTER.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/3-reference/02_GLOSSARY.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/4-specs/SPEC_LEDGER.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/docs/4-specs/README.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/archive/reports/0046-parity-acceptance-artifact.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/reports/tui-human-wait-runs-simulation-issue.md
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/actions/defs/sleep.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/actions/defs/work.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/actions/defs/wait.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/scheduler.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/need_accounting.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/events/envelope.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/src/app.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/src/run.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/view_models.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/projections.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/src/render.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/actions/pipeline.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/actions/proposal.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/actions/report.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/actions/registry.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/events/apply.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/events/log.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/replay/rebuild.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/state.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/playable_capability_parity.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity/runner.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity/scenario.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity/census_actions.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity/census_families.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity/mod.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity_adversarial.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/tui_acceptance.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/time.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-content/src/fixtures/wait_then_window_passive_charges_each_tick_once_001.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-content/src/fixtures/sleep_spanning_window_boundary_charges_each_tick_once_001.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-content/src/fixtures/scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/command_loop_session.rs
https://github.com/joeloverbeck/tracewake/blob/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/need_accounting.rs
https://github.com/joeloverbeck/tracewake/blob/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/projections.rs
https://github.com/joeloverbeck/tracewake/blob/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-core/src/actions/pipeline.rs
https://github.com/joeloverbeck/tracewake/blob/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/playable_capability_parity.rs
https://github.com/joeloverbeck/tracewake/blob/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity/runner.rs
https://github.com/joeloverbeck/tracewake/blob/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity/scenario.rs
https://github.com/joeloverbeck/tracewake/blob/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity/census_actions.rs
https://github.com/joeloverbeck/tracewake/blob/6e91da79b81bf9cbf25acba726e58183920dc441/crates/tracewake-tui/tests/parity/census_families.rs
```

</details>

### 2.3 Seam-map verification against the target commit

The seed’s line anchors were treated as advisory. Named symbols were re-located in the exact-commit files. The load-bearing map is:

| Path and symbol | Exact-commit observation |
|---|---|
| `actions/defs/sleep.rs` — `build_sleep_start_event` | Emits `SleepStarted` with a body-exclusive reservation, recovery rate, and expected completion tick. |
| `sleep.rs` — `build_sleep_completion_events` / end-event builder | Emits `SleepCompleted` or `SleepInterrupted` and derives per-tick sleep effects through the shared tick-regime classifier. |
| `actions/defs/work.rs` — start/completion builders | Starts a body-exclusive scheduled duration; completion emits success/failure, work output, and duration accounting. |
| `actions/defs/wait.rs` — `build_wait_events` | Emits an instantaneous `ActorWaited` action and awake need deltas for its tick range; it is not an open duration. |
| `scheduler.rs` — `DeterministicScheduler::advance_one_tick` | Only increments `current_tick`; it does not run completions, needs, actors, perceptions, or projections. |
| `scheduler.rs` — `run_no_human_day` | Walks time, performs passive accounting, calls due completion logic, records perception, and runs actor transactions. |
| `scheduler.rs` — `append_due_completions` | Is factored as a helper, but consumes mutable pending sleep/work vectors local to a scheduler run rather than rediscovering all open durations from the event log. |
| `scheduler.rs` — `advance_no_human` | Repeats a similar no-human orchestration with its own local pending vectors; it is not the canonical human-world-step seam. |
| `need_accounting.rs` — `classify_actor_tick_regimes_with_start` | Classifies each elapsed actor tick as awake, asleep, or working and is the shared basis for duration effects. |
| `need_accounting.rs` — `open_body_exclusive_starts` | Derives unresolved body-exclusive starts from the log and fails closed on duplicate terminal evidence. |
| `events/envelope.rs` — `is_duration_terminal` | Centralizes terminal-event classification. The enum already contains `TimeAdvanced`, but the inspected target code does not emit it. |
| `actions/pipeline.rs` — body-exclusive conflict check | Detects conflict only when the candidate itself starts another body-exclusive duration, so an ordinary `wait` after `SleepStarted` is not rejected. |
| `tracewake-tui/src/app.rs` — `submit_entry` | Calls `run_pipeline` once, adopts the last appended event tick, records current-place perception, and returns. No world-step/completion call occurs. |
| `tracewake-tui/src/app.rs` — `run_no_human_day` | Exposes the batch runner only behind debug availability and rebuilds projection afterward. |
| `tracewake-tui/src/run.rs` — `UiCommand::WaitOneTick` | Resolves the current semantic `wait` entry and submits it like any other action. |
| `view_models.rs` / `projections.rs` / `render.rs` | The 21-field embodied model is built and exhaustively rendered; it has no interval-summary field. |
| spec-0046 parity registry/runner | Provides real-pipeline, typed/actor-known/rendered/anti-leak/replay dispositions for existing playable capabilities, but no human duration-completion time-control operation. |

### 2.4 External research and how it influenced the decision

External research was used as comparative pressure, not repository evidence:

- Event-sourcing guidance treats the event log as the basis for rebuilding state and temporal queries. That supports making world-time progression and terminal effects replay-visible rather than storing a TUI-local clock mutation.[^fowler]
- Fixed-step simulation guidance favors bounded, repeated canonical steps over an unconstrained time jump, especially when deterministic state evolution matters.[^fiedler][^bevy-fixed]
- SimPy’s distinction between `step()` and `run(until=event)` is a useful control-model analogue: “advance until completion” can be a loop over ordinary processed events rather than a special instantaneous completion operation.[^simpy-env][^simpy-proc]
- Roguelike command documentation distinguishes a one-turn wait from interruptible rest-until-healed behavior. That supports retaining one-tick `wait` while offering ergonomic advance-until semantics for sleep/work.[^nethack][^angband]
- Command-pattern guidance supports giving human and autonomous controllers a common command/action boundary rather than embedding controller-specific mutation in the UI.[^command-pattern]
- Partial-observability literature models an agent as updating from actions and observations rather than directly reading world state. That supports deriving missed-event output from the possessed actor’s observation/holder-known frontier, not a global event-log diff.[^pomdp]
- Event-sourcing projection guidance emphasizes deterministic, replayable projection functions and replay-focused testing. That supports reconstructing the interval summary and stop reason from recorded ancestry.[^esdb-projection][^esdb-testing]

No external example was treated as authority over Tracewake’s foundation. Where a precedent conflicted with repository doctrine, repository doctrine governed.

---

## 3. Findings and recommendations

### 3.1 Finding: the defect is an orchestration gap, not a missing duration model

#### Driver

Human play must permit ordinary-life actions to finish causally. A sleeping actor must recover fatigue through elapsed sleep ticks; a work block must reach its scheduled terminal and produce its modeled result. Foundation `08` states the controlling doctrine directly: waiting runs the simulation. Foundation `06` treats sleep and work as first-class actions that span time, and foundation `12` places an ordinary human-driven day inside first-playable scope.

#### Current coverage

The duration model is already substantial:

- `SleepStarted` records the expected completion tick, body-exclusive reservation, and recovery rate.
- `WorkStarted` records a scheduled body-exclusive block whose result is terminal-driven.
- Completion builders emit typed terminal events and use `classify_actor_tick_regimes_with_start` to derive elapsed sleep/work effects.
- `open_body_exclusive_starts` and `is_duration_terminal` provide shared lifecycle vocabulary.
- The no-human runner can walk ticks, complete durations, apply passive need effects, execute actor decisions, record perceptions, and produce replayable events.

The human path stops before that orchestration:

1. `TuiApp::submit_entry` creates one proposal and runs the ordinary action pipeline once.
2. On acceptance it sets `scheduler.current_tick` to the last appended action event and records one current-place perception.
3. `UiCommand::WaitOneTick` simply selects the current semantic `wait` action.
4. `build_wait_events` emits `ActorWaited` and its need deltas for the bound actor, but does not advance other actors or inspect due durations.
5. The debug-gated `run_no_human_day` is the only TUI-reachable path that actually orchestrates time, and it is explicitly an operator proof surface rather than an embodied command.

`DeterministicScheduler::advance_one_tick` does not fill this gap: it changes a counter only. Conversely, `append_due_completions` is more factored than the seed suggested, but it is not yet a general step primitive because it drains pending-start vectors populated only inside its enclosing no-human run. A `SleepStarted` produced in a prior TUI transaction is invisible to that local queue.

#### Tier-fit verdict

This is not a constitutional defect. The implementation is below existing foundation and architecture doctrine. It is also not a reason to promote the debug day runner into gameplay. The missing abstraction belongs in core scheduling/orchestration, with TUI wiring and lower-tier proof.

#### Recommendation

Create one kernel-owned authoritative one-tick coordinator. It should accept a typed world-advance request, execute exactly one canonical tick across the loaded simulation, return a typed step report, and be looped by all longer-running controls. Refactor both no-human advancement functions to consume it. Keep duration builders and ordinary action validators where they are; the coordinator orders and invokes established seams rather than recreating their logic.

The existing counter-only `advance_one_tick` should either become the private clock increment inside the new coordinator or be renamed so it cannot be mistaken for a complete world step. Public callers should not be able to advance the authoritative frontier by mutating that counter alone.

### 3.2 Finding: a human wait must advance the authoritative world, not a private actor clock

#### Driver

The world-tick fork is the largest design decision in the brief. The two options are:

- advance only the possessed actor’s open duration and perception; or
- advance one authoritative world tick, including unpossessed actors and due processes.

Foundation `01` and the constitutional invariants say the world continues independently of the human. Foundation `08` calls waiting simulation, not local bookkeeping. Architecture `10` says world-advancing controls advance authoritative event/replay time. Possession is an input-routing privilege, not a temporal partition.

#### Current coverage

The current TUI behaves like the rejected private-clock interpretation: the possessed actor receives an event at a later tick, but no shared scheduler walk occurs. That permits impossible divergence. An office, other actor, due duration, or environmental process can remain frozen while the possessed actor’s needs move forward. A later no-human run then observes a log whose timestamps advanced without the corresponding world work.

The no-human routines establish a partial precedent for a shared timeline, but their window-based cadence is tailored to an ordinary-day proof. They should not become the semantic definition of a world tick.

#### Tier-fit verdict

Foundation `08` needs clarification at its own altitude: time controls advance the authoritative loaded world; possession does not create a local timeline. Architecture must define the controller/core boundary and deterministic due-work selection. Execution must freeze testable phase behavior.

#### Recommendation

A human one-tick wait advances **one full authoritative loaded-world tick**.

“Full” means every loaded entity and due world process is governed by the same transition from tick `t` to tick `t+1`. It does **not** mean running every expensive planner every tick. The coordinator should use deterministic cadence and due predicates to decide which actors/processes receive transaction opportunities. Actors with no due decision still undergo any applicable passive need accounting, open-duration progression, environmental effects, and perception processing required by doctrine.

The possessed actor occupies one slot in the same schedule:

- When free and the user chooses `wait`, that actor contributes the existing ordinary `wait` proposal for the tick.
- When the user chooses another ordinary action, that proposal is the actor’s disposition for the relevant tick under the action’s existing temporal semantics.
- When a body-exclusive duration is already open, the actor contributes no second ordinary action. A continuation control asks the coordinator to advance the world while the reservation remains authoritative.
- Unpossessed actors whose decision cadence is due generate proposals through the existing actor-decision transaction and ordinary pipeline.

This preserves possession parity: replacing a human controller with an autonomous controller may change the chosen proposal, but it does not change what a world tick is or which non-controller processes advance.

### 3.3 Finding: “advance until terminal” should be a controller loop over canonical ticks

#### Driver

Making `wait` one full world tick fixes causality, but repeatedly typing `wait` through a multi-tick sleep or work block is poor embodied UX and obscures the existing time-control doctrine. Foundation `08` already anticipates sleep, continuation, acceleration, and stop-on-salient-perceived-interruption as staged controls.

#### Current coverage

Sleep/work are correctly modeled as duration starts plus later terminals. Making them instantaneous would erase interruption, displacement, need thresholds, work failure, and per-tick accounting. The repository has no embodied advance-until controller today; the only batch control is debug no-human-day.

#### Tier-fit verdict

The distinction belongs in foundation `08` and architecture `10`:

- `wait` is a one-tick actor action when the actor is free;
- advance-until is a controller policy that repeats authoritative ticks;
- neither is actor cognition or a source of knowledge.

Execution should own the concrete stop protocol and bounds.

#### Recommendation

After an accepted duration start, the TUI should offer or automatically enter a typed **continue current duration** control. It repeatedly calls the one-tick coordinator and stops at the first processed condition in a deterministic stop set:

1. the possessed actor’s duration emits a terminal event;
2. an actor-known salient observation or modeled interruption reaches the possessed actor;
3. the user pauses/cancels the controller loop before the next tick;
4. a configured deterministic safety bound or kernel error ends the request.

The controller must inspect the typed step result and the possessed actor’s holder-known projection after each completed tick. It must not peek at a hidden due queue to decide that “something interesting happened.” A hidden event may still cause an authoritative terminal—for example, a duration fails because its physical precondition no longer holds—but the actor-facing stop explanation must be filtered through modeled feedback. The loop may stop because the actor’s own action terminated while withholding a hidden causal detail the actor could not know.

A one-tick `wait` remains available and semantically stable. Advance-until is not a second completion engine; it is repetition with a stop predicate. This mirrors the useful control distinction found in event-driven simulation APIs and interruptible rest commands without importing their domain rules.[^simpy-env][^angband]

### 3.4 Finding: the shared open-duration authority must be the source of completion discovery

#### Driver

A world step must discover durations started in any prior accepted transaction. Completion cannot depend on ephemeral vectors owned by a particular no-human invocation. The shared-open-duration conformance row requires scheduler, pipeline, need accounting, and replay to agree on one lifecycle keying and to fail closed on malformed terminals.

#### Current coverage

`open_body_exclusive_starts` already scans authoritative log evidence and recognizes unresolved body-exclusive starts. `is_duration_terminal` centralizes terminal classification. The completion builders consume the original start and emit typed terminals. Duplicate terminal evidence is treated as poisoned input rather than normalized away.

The mismatch is orchestration state: `run_no_human_day` and `advance_no_human` accumulate pending starts in local vectors and pass those vectors to `append_due_completions`. This works for starts produced during that same batch. It does not establish an independent step that can resume from an arbitrary valid event log.

#### Tier-fit verdict

No new lifecycle doctrine is needed. Architecture `04` and execution `05` should name log-derived open-duration discovery as the coordinator’s authority and prohibit a second queue from becoming authoritative.

#### Recommendation

Refactor due-completion discovery around the shared log authority:

- derive unresolved body-exclusive starts through `open_body_exclusive_starts` or a projection/index whose complete state is itself rebuilt from the same log;
- read each start’s expected terminal tick and duration kind from its typed payload;
- deterministically order due candidates by the repository’s existing ordering authorities;
- invoke the existing sleep/work completion builders;
- append terminals through the scheduler/pipeline-owned event seam;
- reject or poison duplicate/orphan lifecycle evidence rather than guessing;
- ensure replay can reconstruct the same open-duration set without hidden scheduler memory.

A performance index may be introduced later, but it must be a derived cache, never a second source of truth. The first implementation should prefer obvious replay correctness over premature batching.

The coordinator should also evaluate explicitly modeled interruption predicates for open interruptible durations at the cadence required by their lifecycle. An early interruption must close the same duration through its terminal builder and shared keying; it must not create an unrelated TUI cancellation record that leaves the reservation open.

### 3.5 Finding: single-charge-per-tick requires one accounting transaction, not “wait then scheduler”

#### Driver

A naïve patch would keep today’s wait submission and then call a scheduler step. That is unsafe. `wait` already emits awake need deltas and counted tick coverage. A subsequent generic passive charge can debit the same actor/need/tick again. Conversely, treating an actor as both awake-waiting and asleep across the same tick can both charge and recover it.

#### Current coverage

Execution `06` is explicit: need accounting is single-charge per actor, need, and tick across passive windows, action-emitted awake deltas, and duration effects. Duration `elapsed_ticks` expand into per-tick ledger coverage. The single-owner seam is the action-pipeline/ordinary-life boundary; scheduler, projection, replay, and normalization may consume evidence but may not synthesize competing charges.

The exact-commit fixtures reinforce this contract:

- `wait_then_window_passive_charges_each_tick_once_001` protects a wait-covered tick from later passive-window duplication;
- `sleep_spanning_window_boundary_charges_each_tick_once_001` protects duration effects across scheduler windows;
- `scheduler_cannot_rewrite_wait_reason_after_transaction_001` protects the accepted action’s actor-visible reason from scheduler rewriting.

#### Tier-fit verdict

The current doctrine is sufficient. Execution `06` needs a human-time-control proof extension, not a revised accounting rule.

#### Recommendation

The coordinator must own one per-tick accounting reconciliation across all actor dispositions. It should not implement “submit wait, then independently apply a world tick.” Instead:

1. construct the tick’s deterministic schedule and actor dispositions;
2. route the possessed actor’s free-state `wait` through the existing ordinary pipeline;
3. collect the resulting counted charge evidence;
4. classify every actor/tick using the shared awake/asleep/working authority, including newly due duration terminals;
5. emit only missing passive/duration effects through the single-owner seam;
6. assert exactly one applicable charge/recovery classification per actor/need/tick before completing the step.

If existing pipeline structure makes this ordering impossible without an intermediate transaction object, introduce a typed tick-accounting plan inside core. Do not move arithmetic into TUI code or add a “human wait already charged” boolean. Coverage must be derivable from event-backed ledger evidence.

A key compatibility goal is to preserve the accepted `ActorWaited` event and sealed reason for a free actor’s one-tick wait. That keeps existing semantic action behavior and fixtures meaningful. The world-step integration should recognize that evidence, not replace it with an unrelated passive event.

### 3.6 Finding: body-exclusive reservation enforcement is part of this feature, not a separate follow-up

#### Driver

After `SleepStarted`, the human path currently accepts `wait`. That means the possessed actor can perform an awake ordinary action while a body-exclusive duration claims the body. Any time-control design built on that state is incoherent: it cannot tell whether the actor is sleeping, waiting, double-accounted, or silently abandoning the reservation.

#### Current coverage

The autonomous scheduler checks whether an actor has an open body-exclusive duration and skips ordinary decision generation. The action pipeline has a reservation conflict stage, but the inspected predicate only derives an actor from candidate events that themselves start a new body-exclusive duration. Therefore sleep-then-sleep can conflict while sleep-then-wait does not.

#### Tier-fit verdict

This is a lower-tier enforcement defect under existing shared-reservation doctrine. It should be resolved in the same implementation spec because the explicit continuation control depends on a correct distinction between “another actor action” and “advance time while the current action remains open.” Deferring it would leave the primary solution internally contradictory.

#### Recommendation

Generalize body-exclusive conflict enforcement:

- If an actor has an unresolved body-exclusive start at the proposal tick, reject every ordinary embodied action for that actor with the existing reservation-conflict report.
- Permit only typed controls explicitly defined by the duration lifecycle: continue time, pause the controller loop, and any modeled interruption/cancellation action the future spec authorizes.
- A continuation control is not an `ActorWaited` event and does not create a competing body reservation. It advances the world while the existing start remains the actor’s embodied commitment.
- When an interruption is modeled, close the existing duration through a terminal event before any new ordinary action can be accepted.
- Apply the rule identically to human and autonomous proposal origins.

The expected regression witness is direct: start sleep through the real TUI pipeline, submit ordinary `wait`, and receive `ReservationConflict`; then use the explicit continuation control and observe world progress without `ActorWaited` for the sleeping actor.

### 3.7 Finding: interval output must be a holder-known projection, not a world report with redaction

#### Driver

A full world tick means other actors and processes can act while the possessed actor waits, works, travels, or sleeps. The UI needs an intelligible account of what the actor experienced or learned. Foundation `08` forbids omniscient sleep summaries and says temporal facts in embodied views must be actor-known, record-derived, or source-bearing. Foundation `14` and architecture `03` prohibit converting hidden truth into actor cognition by prose.

#### Current coverage

The embodied view model already provides a typed, exhaustively rendered boundary. Current perception/projection code records the actor’s current-place perception after an accepted TUI action. The epistemic subsystem represents observations, beliefs, source references, and holder-known contexts. Debug reports remain structurally separate.

What is missing is an interval-specific projection. A tempting implementation—diff the global event log between start and stop, then hide selected fields—is structurally wrong. It begins from omniscience, risks omission drift, and can make negative claims such as “nothing happened” that the actor cannot establish.

#### Tier-fit verdict

Foundation `08` should own the rule that missed-event summaries are actor-known interval conclusions. Architecture `03` should own the source/projection boundary. Architecture `10` should own how the typed summary enters the embodied view. Execution `07` should own positive and negative witnesses.

#### Recommendation

Add a typed actor-known interval-summary surface, conceptually containing:

- the actor-visible interval boundary at whatever precision the actor can know or infer under existing temporal-display doctrine;
- the possessed actor’s own duration outcome, with source event IDs internally attached;
- actor-legible bodily/need changes, at the same precision already permitted by the embodied view;
- observations actually delivered during the interval;
- messages, testimony, notices, records, sounds, public cues, or institutional outputs that reached the actor through modeled channels, each with provenance;
- fresh resumption perception and actor-known local changes inferred from that perception;
- an actor-visible stop reason when one exists.

It must exclude:

- private actions or states of other actors not observed or communicated;
- raw global event counts, hidden due queues, scheduler phase data, or exact hidden causal ordering;
- a hidden reason for a duration interruption or failed completion;
- claims that an unobserved place, service, item, or person did not change;
- the omniscient negative “nothing happened.”

When no items reached the actor, the safe result is equivalent to **“no new actor-known notices or observations are available”**, not “nothing happened.” The wording remains renderer-owned; the semantic distinction must be typed.

The summary should be derived positively from a delta in the actor’s holder-known/source-bearing frontier plus own embodied events and resumption perception. It should not be built as a redacted `WorldAdvanceReport`. Keep two structurally separate outputs:

1. an exact core/debug step report containing ticks, appended event IDs, due work, hidden stop diagnostics, and replay checks; and
2. an actor-known interval projection whose constructors accept only holder-known/source-bearing inputs.

If a summary item is persisted into memory, notebook, belief, or future planning, it must already have a modeled observation/record/memory ancestry. Rendering prose cannot create a fact.

The preferred surface is an explicit optional field or nested value on `EmbodiedViewModel`, because that forces spec-0046 Hop-2 exhaustive disposition in `render.rs` and makes omission a compile/source-guard failure. A sibling typed command result is acceptable only if it is still integrated into the embodied projection and receives the same exhaustive parity protections; a free-form success string is not acceptable.

### 3.8 Finding: replay needs an authoritative event for otherwise empty ticks

#### Driver

A world tick can be causally meaningful even when no actor action, need threshold, duration terminal, or perception event is emitted. If the scheduler counter alone moves, replay from the event log cannot reconstruct the same authoritative temporal frontier. Advance-until stop behavior can then differ after rebuild.

#### Current coverage

The event enum contains `TimeAdvanced`, but the inspected target code does not emit it. Existing event/replay doctrine requires authoritative temporal ancestry and deterministic rebuild. TUI currently mutates `scheduler.current_tick` from the latest action event, which works only when an event happens to carry the desired tick.

#### Tier-fit verdict

The constitution and foundation replay contract are already sufficient. Architecture `02` should make no-op tick ancestry explicit; execution `05` should freeze the event envelope and replay witness.

#### Recommendation

Use the existing `TimeAdvanced` event kind as the canonical tick-boundary evidence, subject to a narrow semantic audit in the future spec. Each executed world step should append one authoritative marker—or an equivalent already-ratified temporal event if the audit proves `TimeAdvanced` has a conflicting intended meaning—with:

- prior and resulting tick;
- controller/process origin as non-epistemic scheduling metadata;
- ordering ancestry sufficient to reproduce the step;
- causal links to the time-control request where applicable.

The marker is world/replay evidence, not actor knowledge. It must not enter holder-known projections merely because it exists. Exact tick data may remain available to debug/operator surfaces under existing quarantine.

Acceleration remains repeated one-tick markers, not a single jump from `t` to `t+n`. This is intentionally conservative. It gives every intermediate need tick, due terminal, observation, autonomous transaction, and interruption a stable ordering point. Later optimization may compact storage only if replay, ancestry, and per-tick accounting remain observationally identical.

Replay proof must rebuild:

- final physical and agent state;
- scheduler/world temporal frontier;
- duration open/closed state;
- need values and counted tick ledger;
- work output and sleep recovery;
- epistemic projection;
- possessed actor’s interval summary and actor-visible stop reason.

### 3.9 Finding: spec-0046 parity covers starts and surfaces, not human duration completion

#### Driver

The settled brief requires the fix to satisfy spec-0046’s standing per-feature parity obligation. Sleep, wait, and work appear in the current capability census, but capability-name coverage does not prove that a human-started duration reaches its terminal through an embodied time-control path.

#### Current coverage

The accepted spec-0046 baseline covers 21 playable capabilities and establishes:

- a test-side capability registry;
- deterministic real-pipeline scenario operations;
- typed causal witnesses;
- actor-known and rendered witnesses;
- anti-leak and replay/no-human dispositions;
- Hop-1 runtime goldens and Hop-2 exhaustive compiler/source guards;
- standing future-feature declarations under PAR-010/PAR-011.

Its scenario operations can submit semantic actions and run a debug/no-human day. They do not express “advance one human world tick,” “continue this human-started duration,” or “stop on actor-known interruption.” Existing TUI tests assert that wait or duration starts are accepted, not that the world advances equivalently or that terminals/output/recovery appear.

#### Tier-fit verdict

Do not replace or weaken spec-0046. Extend its registry, setup operations, dispositions, and CI lane as part of the future implementation spec.

#### Recommendation

Add explicit capability entries or feature dispositions for at least:

1. human wait advances one authoritative world tick;
2. human-started sleep reaches a terminal and produces correctly accounted recovery;
3. human-started work reaches a terminal and produces output or modeled failure;
4. an open body-exclusive duration rejects ordinary wait/action but accepts typed continuation;
5. an actor-known interval summary renders positive source-bearing information and suppresses hidden world activity;
6. advance-until stops on an actor-known salient interruption without leaking a hidden cause.

Extend the deterministic scenario runner with real TUI/core operations for one world step and advance-until. `RunNoHumanDay` is not an acceptable witness for the human path. Each capability should state:

- typed event/source witness;
- actor-known positive witness;
- actor-known negative/anti-leak witness;
- rendered golden or explicit no-render disposition;
- replay/rebuild witness;
- no-human and possession-parity disposition;
- Hop-2 field/destructure disposition if the view model changes.

Add a differential scenario: from the same initial event log and deterministic inputs, advance one tick under a human wait disposition and under the equivalent no-human/controller disposition. Where the actor choice is held equal, the authoritative physical transition, duration completion set, need ledger, and replay checksum should match; only controller-origin metadata and actor-facing command report may differ.

### 3.10 Finding: doctrine needs clarification downward, not a new constitutional rule

#### Driver

The code gap exposed an ambiguity in how the staged time-control doctrine is operationalized. The correct response is to put each clarification in the tier that owns it, without writing implementation details into foundation or inventing new invariant/gate/risk/glossary identifiers.

#### Constitution and foundation disposition

**Constitutional invariants — no amendment.** Existing invariants already require one authoritative event-sourced world, deterministic replay, survival causality, no-human behavior, possession parity, subjective knowledge, debug quarantine, and temporal authority. The capability conforms by using those rules; it does not need an exception.

**Foundation `08` — amend the Time controls section.** Add substance at this altitude:

- an embodied wait/step advances the authoritative loaded world rather than a private possessed-actor clock;
- possession supplies one actor’s input but does not pause other actors or due processes;
- a one-tick actor wait and a controller-level continuation/acceleration are distinct;
- sleep/work remain causal durations and acceleration is repeated authoritative progression;
- stopping and interval summaries are based on perceived/holder-known information;
- sleeping or absent actors receive no global event diff or omniscient negative summary.

Do not specify Rust types, scheduler phases, event payloads, or test names here.

**Foundation `03`, `05`, `06`, `12`, `14` — no substantive amendment required.** Their current rules already cover temporal authority/replay, need accounting, causal sleep/work, first-playable ordinary life, and scheduler-owned completion. Add cross-references only if the repository’s reassess process finds a discoverability gap.

#### Architecture disposition

**Architecture `10`, temporal rendering and embodied play loop — amend.** Own the typed world-control boundary, the fact that the TUI asks core to advance rather than mutating state, repeated one-tick acceleration, and the actor-known/debug report split.

**Architecture `04`, scheduler/pipeline — amend.** Own a single authoritative world-step coordinator; deterministic due-work ordering; ordinary proposal routing for human and autonomous actors; log-derived open-duration discovery; general body-exclusive conflict; and explicit continuation controls that do not masquerade as ordinary actor actions.

**Architecture `02`, replay/projections — amend.** Own replay-visible tick-boundary evidence, repeated-step acceleration, rebuild of temporal frontier, and deterministic interval projection.

**Architecture `03`, holder-known contexts — amend.** Own positive construction of interval summaries from source-bearing holder-known deltas and prohibit raw-world-diff redaction as the embodied boundary.

**Architecture `00`, conformance index — correct/extend rows.** Name human world advance as a consumer of the existing tick-charge attribution and shared open-duration authorities, and name time-control/interval-summary parity under the existing parity posture. Do not mint a new conformance family unless reassessment proves the existing rows cannot express the obligation.

**Architecture `13`, evidence — add.** Require one-tick human/no-human differential evidence, duration-terminal and accounting witnesses, replay/no-op tick evidence, actor-known summary anti-leak evidence, reservation conflict, and spec-0046 parity extension.

#### Execution disposition

**Execution `05` — amend.** Define the canonical step’s concrete ownership, typed request/result, deterministic phase contract, event append/application route, due duration scan, actor transaction cadence, and no-direct-dispatch guard.

**Execution `06` — amend.** Extend ordinary-life proof to human-driven ticks: existing wait charge evidence, passive suppression, duration-regime classification, open-duration completion, and human/no-human equivalence.

**Execution `07` — correct the staging sentence.** Preserve the debug no-human-day distinction, but replace the “not exposed outside debug” posture once this feature is accepted. Define embodied one-tick and advance-until proof, actor-known stop reasons, interval summary anti-leak rows, and structurally separate debug diagnostics.

**Execution `10` — add.** Define command transcripts, typed event ledgers, checksums, replay reports, hidden-truth negative witnesses, and parity evidence expected from implementation and acceptance artifacts.

#### Reference and current-spec disposition

**Reference checklist — add concise prompts.** Reviewers should check that all time controls use the same world step, every actor/need/tick has one accounting classification, open durations close through shared authority, and interval output is positive actor-known evidence rather than redacted omniscience.

**Risk register — update existing entries.** The feature is a new mitigation/test surface for pipeline bypass, epistemic leakage, prose authority, debug leakage, replay/projection erosion, TUI playability erosion, no-human ordinary-life failure, and the temporal-relapse cluster. Do not invent a new risk ID merely to name this defect.

**Glossary and acceptance template — no change.** Existing terms and the template’s parity/evidence structure are adequate.

---
## 4. Resolved design decision

### 4.1 The committed solution

> **Introduce one kernel-owned authoritative one-world-tick coordinator. Human `wait`, duration continuation, no-human progression, and future acceleration all loop that same primitive. A human tick advances the full loaded world. The possessed actor’s ordinary action is one scheduled input; possession does not define a private clock. Open durations are rediscovered from shared log authority, terminal effects and needs are reconciled exactly once, and the actor receives only a holder-known interval projection.**

This is one solution, not a menu. Its components are mutually necessary:

- A full-world step without shared duration discovery still cannot resume human-started sleep.
- Shared completion without unified accounting risks double charge.
- Correct accounting without reservation enforcement permits mutually exclusive action overlap.
- Correct world mutation without an actor-known projection makes embodied time controls either opaque or omniscient.
- A TUI-local implementation of any of these would violate the pipeline and parity contracts.

### 4.2 Boundary model

The future spec should define conceptual interfaces at three boundaries. Names below describe responsibility and are not ratified API text.

#### A. Core world-advance request

A typed request carries:

- expected current temporal frontier;
- controller/process origin and deterministic request identity;
- one-tick versus advance-until policy;
- possessed actor, if any;
- the possessed actor’s current disposition:
  - ordinary semantic action such as `wait`, or
  - continuation of an identified open duration, or
  - no possessed actor for detached/no-human operation;
- stop policy and deterministic maximum bound;
- content/checksum context required by existing replay rules.

The request is not actor cognition. It is validated core control input. When it contains an ordinary actor action, that action still becomes an ordinary proposal with normal validation, ordering, event generation, feedback, and reservation enforcement.

#### B. Authoritative one-tick result

A core result carries exact non-diegetic evidence:

- prior and resulting tick;
- the tick-boundary event ID;
- accepted/rejected actor proposals and reports;
- appended event IDs in authoritative order;
- duration starts, terminals, and interruption diagnostics;
- per-tick accounting evidence and any fail-closed error;
- due-process and perception work performed;
- replay/projection frontier information;
- a machine-readable stop candidate set.

This result is suitable for scheduler composition, replay proof, diagnostics, and debug. It is not renderable in embodied mode.

#### C. Actor-known interval projection

A separate projection consumes only:

- the possessed actor’s holder-known frontier before and after the interval;
- source-bearing observation/message/record events that reached that actor;
- the actor’s own embodied action/duration events through a safe actor-known interpretation;
- fresh modeled perception at resume;
- an actor-visible stop-reason projection.

It yields the interval-summary value exposed through the embodied view model. No constructor should accept arbitrary physical state, raw global event slices, hidden due queues, or the exact debug step report.

### 4.3 Canonical one-tick responsibilities

The future spec must freeze a single deterministic intra-tick phase order after compatibility tests against existing fixtures. This report does not ratify exact subphase numbers, but the canonical step must perform all of the following in one owned transaction boundary:

| Responsibility | Required behavior | Prohibited shortcut |
|---|---|---|
| Frontier validation | Reject a stale/mismatched expected tick; derive one resulting authoritative tick. | Silently overwrite scheduler time from a UI-local counter. |
| Tick ancestry | Append the existing `TimeAdvanced` event, or correct its unused semantics if necessary, at a frozen point in the step. | Advance a counter with no replay evidence. |
| Open-duration discovery | Derive unresolved starts from log-backed shared authority and deterministic ordering. | Depend on vectors populated only in the current no-human call. |
| Duration lifecycle | Evaluate due terminals and modeled interruption predicates; close through existing sleep/work terminal builders. | Have the TUI synthesize `SleepCompleted`, recovery, wages, or interruption prose. |
| Actor dispositions | Submit the possessed actor’s ordinary action when free; generate due autonomous proposals through the same pipeline. | Direct-dispatch an action or give possessed actors a special validator. |
| Reservation | Block ordinary actions while a body-exclusive duration is open; permit typed continuation/lifecycle controls only. | Accept `ActorWaited` concurrently with sleep/work. |
| Need accounting | Reconcile action-emitted, passive, and duration effects by counted `(actor, need, tick)` coverage. | Charge wait and passive time independently, or recover and charge the same regime twice. |
| World processes | Run every loaded process due under the deterministic cadence. | Freeze all unpossessed actors because a human supplied one action. |
| Perception/epistemics | Record modeled perceptions/communications at the correct phase and project them from source events. | Produce knowledge from a world-state diff or renderer prose. |
| Projection/replay | Finish with a rebuild-equivalent frontier and typed reports. | Let an in-memory cache or TUI field become temporal authority. |

The step should be **atomic with respect to acceptance**: either it appends a coherent ordered event set and updates projections, or it returns a fail-closed error without leaving half a tick applied. The repository may implement this with existing append/apply discipline rather than a database transaction, but the observable result must be all-or-nothing.

### 4.4 Human command semantics

| Human situation | Recommended behavior | Actor event for the tick | World behavior |
|---|---|---|---|
| Free actor selects one-tick `wait` | Core validates and schedules the existing wait semantic action as the actor’s disposition. | `ActorWaited` plus its existing source-bound effects. | One full loaded-world tick executes. |
| Free actor starts `sleep` | Start remains an ordinary accepted action; the duration opens without pretending it completed. | `SleepStarted`. | No skipped terminal; the user may then continue/advance-until. |
| Free actor starts `work_block` | Start remains ordinary and scheduled. | Work start event(s). | Completion remains future and causal. |
| Actor has open sleep/work and invokes time continuation | Core advances ticks under that existing reservation. | No `ActorWaited` for the sleeping/working actor. | Other actors/processes advance; duration accounting/terminal can occur. |
| Actor has open sleep/work and attempts an ordinary action, including ordinary `wait` | Reject through reservation-conflict feedback. | No competing action event. | No tick advances unless the command was explicitly a continuation request. |
| Advance-until reaches duration terminal | Stop after the terminal-containing step. | `SleepCompleted`/`SleepInterrupted` or work success/failure terminal and effects. | Return exact debug result plus actor-known summary. |
| An actor-known salient observation arrives first | Stop after the step that processed it; whether the duration also interrupts follows modeled lifecycle rules. | Source-bearing observation and any valid terminal. | Do not expose hidden events that did not reach the actor. |
| User pauses | Stop before starting another tick. | No pause-as-world-fact unless repository doctrine later models one. | Completed prior ticks remain authoritative. |

The existing `wait` alias can remain a one-tick command. The TUI should add a clearly separate continuation/advance-until control rather than overloading `wait` to sometimes mean an actor action and sometimes mean “stay asleep.” Renderer text may still make the common case convenient, but the typed distinction must survive parsing.

### 4.5 World scope and cadence

The loaded-world interpretation is intentionally bounded by the repository’s current simulation scope. “All actors advance” means all actors and processes represented at the authoritative loaded simulation level. It does not prematurely solve deferred regional level-of-detail or unloaded-world simulation. When future LOD work arrives, it must make its own authoritative transition compatible with the same temporal contract.

Within the loaded world:

- every actor’s needs/durations are eligible for the tick’s regime accounting;
- every open duration whose due/interruption predicate applies is processed;
- every actor whose decision cadence is due may run an actor decision transaction;
- every due deterministic world/institution process runs;
- perception occurs according to modeled availability and phase order;
- actor IDs, process IDs, proposal sequence, event IDs, and target ordering use existing deterministic authorities.

This avoids two opposite failures:

- **possession freeze**, where other actors stop while the player waits; and
- **planner explosion**, where every actor performs a full decision search on every elementary tick regardless of cadence.

The no-human day runner should retain its day windows, progress metrics, and proof report as a higher-level composition. It should no longer own a separate definition of time progression. It loops the world step with a no-human controller policy and aggregates its existing report fields from the returned step evidence.

### 4.6 Duration completion and interruption

The coordinator should query all open body-exclusive starts at the step frontier. For each start it should determine, from typed payload and modeled state:

- whether its expected completion tick is reached;
- whether an authorized interruption predicate is satisfied earlier;
- whether the actor still satisfies completion preconditions;
- which terminal builder owns closure;
- which elapsed ticks belong to awake, asleep, or working regimes.

The existing completion builders remain the source of terminal events and duration effects. The coordinator should not reinterpret their domain arithmetic. Where those builders currently assume they are invoked only at expected completion, the future spec must decide whether early interruption is passed through the same builder with an explicit terminal tick or through a narrowly factored lifecycle helper. Either route must produce one shared terminal and one counted tick expansion.

A duration’s expected completion tick is a scheduling fact, not automatically actor knowledge. The embodied summary may render “you wake” or a source-supported temporal conclusion; it must not reveal the exact hidden schedule simply because the start payload contains it.

### 4.7 Actor-known interval-summary contract

The summary is best understood as a replayable projection over an interval, not narration. Its data contract should distinguish at least these categories:

| Category | May appear in embodied summary? | Source rule |
|---|---|---|
| Own action/duration outcome | Yes | Safe projection of own start/terminal event and modeled feedback. |
| Own bodily/need change | Yes, at existing embodied precision | Proprioceptive/actor-known projection sourced to need/duration events. |
| Direct perception at resume | Yes | Modeled perception event or projection with source ancestry. |
| Message, notice, record, testimony, public cue | Yes | It actually reached the actor through a modeled channel and retains provenance. |
| Sound or interruption perceived during interval | Yes | Source-bearing observation delivered to the actor. |
| Other actor’s private action | No, unless observed/communicated | Never infer from global event presence. |
| Hidden reason a validator or duration failed | No | Use safe actor-visible feedback; exact reason stays debug-only. |
| Global event count or due queue | No | Non-diegetic scheduler data. |
| “Nothing happened” | No | Actor cannot establish absence of hidden events. |
| “No new known notices/observations” | Yes | A statement about the actor-known projection, not the world. |

The projection should preserve source IDs internally even when the default renderer uses concise prose. Debug may show the source chain and omitted hidden rows in a quarantined panel. Actor-facing output must not include “omitted truth” counts, because even the fact that hidden rows exist can leak information.

A robust stop/result sequence is:

1. capture the possessed actor’s holder-known frontier before the first step;
2. execute one or more authoritative ticks;
3. after each tick, update modeled perceptions and holder-known projection;
4. evaluate stop predicates against typed terminals and actor-known salience;
5. on stop, derive the interval summary from positive frontier additions plus own embodied changes;
6. render the summary through exhaustive view-model disposition;
7. retain the exact step sequence only in replay/debug evidence.

The future spec must decide the lifecycle of the last summary—whether it persists until the next world-advancing command, until acknowledged, or as a bounded source-backed history. The primary recommendation is to store an explicit source-backed “last completed advance interval” field in the embodied projection and replace it atomically on the next completed advance. That is simple, testable, and compatible with exhaustive renderer guards. Long-term notebook persistence should occur only through existing memory/record mechanisms, not by accumulating renderer summaries.

### 4.8 Replay and deterministic stop behavior

Advance-until is deterministic when it is defined as a pure controller loop over deterministic step results:

```text
while completed_steps < configured_bound:
    result = advance_one_authoritative_world_tick(request_for_next_tick)
    project holder-known changes
    if possessed duration terminal is in result: stop
    if actor-known salient interruption is in the projection: stop
    if user pause was received before next tick: stop
```

The pseudocode is explanatory, not paste-ready implementation. The important properties are:

- no prediction of a future terminal substitutes for processing intermediate ticks;
- the stop decision after each completed tick is reproducible from event and projection ancestry;
- user pause affects only whether another step begins, not the already completed step;
- a safety-bound stop is reported as a controller limitation, not an in-world belief;
- replay does not need the original UI loop timing, only the recorded world/control events and deterministic policy inputs required by existing doctrine.

For a fully automated duration continuation, the request should record enough deterministic control metadata to prove why the loop continued or stopped. That metadata must remain outside actor knowledge unless independently observed. The future spec should explicitly test save/rebuild in the middle of an open duration and resume continuation from the rebuilt log; success cannot depend on an in-memory pending queue.

### 4.9 Constraint satisfaction matrix

| Constraint | How the committed solution satisfies it | Required witness |
|---|---|---|
| Single-charge-per-tick | One coordinator reconciles action, passive, and duration evidence against shared counted coverage. | Per-actor/per-need/per-tick ledger; wait+passive and sleep-window adversarial tests. |
| Shared open-duration authority | Every step derives unresolved starts/terminals from log-backed shared lifecycle authority. | Resume from pre-existing human start; duplicate/orphan terminal fail-closed tests. |
| Replay determinism | Repeated canonical steps append temporal ancestry and all effects; no TUI-local clock authority. | Rebuild/checksum equality, mid-duration save/rebuild/resume, identical interval projection. |
| Full world causality | Human wait runs loaded actors and due processes under the same tick definition as no-human advancement. | Human/no-human one-tick differential scenario. |
| Possession parity | Possession supplies one actor’s proposal and viewer; it does not alter world cadence or hidden knowledge. | Rebind/controller-origin comparison and no-human parity disposition. |
| Truth firewall | Embodied summary is built positively from holder-known/source-bearing inputs; debug report is structurally separate. | Positive source-chain witness and hidden-other-actor non-interference test. |
| No-fact-from-prose | Summary fields are typed projections; renderer cannot create memory, belief, or world state. | Source assertions and negative test that prose/golden changes do not alter state. |
| No direct dispatch | TUI sends a typed request; actor actions still run through ordinary proposal/validation/event/application/projection. | Source guard and real-pipeline transcript. |
| Body-exclusive reservation | Ordinary actions conflict while duration open; explicit continuation advances without second actor action. | Sleep→wait rejection and sleep→continue success. |
| spec-0046 parity | New controls/field receive registry entries, real-pipeline witnesses, anti-leak, replay, and exhaustive render disposition. | Updated parity census, runner, goldens, adversarial lane, CI. |
| Post-certification posture | Work is scoped as a new feature spec, not a claim that prior certification covered it. | Spec/ledger wording and acceptance artifact scope statement. |

### 4.10 Alternatives considered and rejected

| Alternative | Rejection reason |
|---|---|
| **Advance only the possessed actor’s duration and perception.** | Creates a private temporal island, freezes unpossessed actors/processes, makes possession affect outcomes, and contradicts authoritative world time. |
| **Call debug `run_no_human_day` from `TuiApp::wait`.** | Promotes an operator proof command into gameplay, imports day-window semantics and debug report shape, lacks actor-known stop behavior, and keeps two advancement definitions. |
| **Have the TUI call sleep/work completion builders directly.** | Direct dispatch bypasses scheduler ordering, shared accounting, replay ancestry, and no-human parity. |
| **Keep current wait action, then invoke a generic scheduler tick.** | The wait already emits tick charges; a second passive/duration pass can double-charge or assign conflicting regimes. It also makes action ordering accidental. |
| **Make sleep and work instantaneous in human mode.** | Erases the causal duration, interruptions, per-tick recovery/charge, reservation, other-actor activity, and replay-equivalent world evolution. |
| **Jump directly to the predicted terminal tick.** | Skips intermediate needs, processes, observations, interruptions, and deterministic ordering. It makes the terminal prediction a second simulation engine. |
| **Keep local pending-start queues but persist one in `TuiApp`.** | Makes UI memory a second duration authority and fails save/rebuild/resume or alternate-controller execution. |
| **Build summaries by diffing the global event log and redacting hidden rows.** | Starts from omniscience, makes omission correctness fragile, and enables hidden-event counts and negative-world claims. |
| **Render only “slept until morning” with no typed summary.** | Prose would become an unproven fact and could conceal source/precision errors; it would also evade parity field disposition. |
| **Treat every loaded actor as due for full planning every tick.** | Correct world time does not require wasteful planner invocation; deterministic cadence can advance world state without planner explosion. |
| **Route reservation enforcement as a later hardening item.** | Leaves the primary continuation design contradictory and allows awake `wait` to overlap sleep/work, corrupting regime accounting. |
| **Add a new constitutional invariant or new risk ID.** | Existing doctrine already governs the behavior. New identifiers would inflate authority rather than fix the lower-tier operational gap. |

---

## 5. Forward-routing appendix

### 5.1 Recommended next implementation artifact

Author a new numbered post-`FIRST-PROOF-CERT` feature spec with the scoped title:

> **TUI Authoritative World Advance, Duration Completion, and Actor-Known Interval Summaries**

The repository’s normal spec allocator should assign the number. The spec should not be framed as a certification audit, mutation-remediation replacement, Phase-4 entry claim, or proof that the target commit is latest `main`.

Its scope should include:

- doctrine amendments approved through the repository’s reassess process;
- one canonical kernel world-step shared by human and no-human modes;
- log-derived duration completion and modeled interruption;
- single-charge accounting across human wait, passive ticks, and duration effects;
- body-exclusive reservation and typed continuation semantics;
- `TimeAdvanced` temporal ancestry and replay/rebuild;
- full loaded-world tick semantics with deterministic due-work cadence;
- human one-tick wait and advance-until-duration behavior;
- actor-known interval summary plus debug report quarantine;
- spec-0046 PAR-010/PAR-011 extension and acceptance evidence.

Explicitly out of scope unless implementation evidence forces reconsideration:

- regional/unloaded-world LOD simulation;
- generalized high-speed time compaction;
- institutions/notices beyond whatever existing modeled channels already deliver;
- free-form narration or LLM summarization;
- new glossary terms, constitutional identifiers, certification gates, or risk IDs;
- backwards-compatibility aliases for an incorrect time-control path.

### 5.2 Ticket-level decomposition

Each ticket below is intended as one reviewable diff. The future spec may rename or reorder them after dependency validation, but should preserve separation of concerns.

#### Ticket 1 — Authority-aligned documentation amendments

Amend foundation `08`; architecture `02`, `03`, `04`, `10`, `13`, and conformance mappings; execution `05`, `06`, `07`, `10`; reference checklist/risk entries. Record no constitutional change and no new identifier. This ticket establishes approved semantics before code.

#### Ticket 2 — Core one-tick type boundary and temporal marker

Introduce the typed world-advance request/result boundary, validate expected frontier, operationalize `TimeAdvanced`, and prove one otherwise empty tick rebuilds to the same temporal frontier. Keep actor actions out of this diff except for test stubs needed to demonstrate the boundary.

#### Ticket 3 — Shared log-derived open-duration discovery

Replace or wrap pending-vector completion discovery with a shared query/projection over unresolved starts. Preserve deterministic ordering and duplicate/orphan fail-closed behavior. Add resume-from-pre-existing-start unit tests.

#### Ticket 4 — Canonical duration completion/interruption phase

Wire sleep/work due terminals and authorized early interruption into the one-tick coordinator through existing terminal builders. Add isolated terminal, interruption, and reservation-closure tests without TUI changes.

#### Ticket 5 — Unified per-tick need accounting

Integrate action-emitted, passive, and duration-regime evidence under one counted reconciliation. Preserve the existing wait/window and sleep/window fixtures and add adversarial combinations that would double-charge under a two-pass implementation.

#### Ticket 6 — General body-exclusive reservation enforcement

Generalize pipeline conflict detection to all ordinary embodied actions for an actor with an open body-exclusive duration. Add explicit continuation/lifecycle-control exemption plumbing, plus human- and autonomous-origin tests.

#### Ticket 7 — Refactor no-human runners onto the canonical step

Make `run_no_human_day` and `advance_no_human` loop the shared one-tick coordinator while preserving their windows, progress metrics, markers, reports, and existing acceptance behavior. Remove duplicate pending-duration authority.

#### Ticket 8 — TUI one-tick world advance

Add a typed `TuiApp` world-step entry point and route `UiCommand::WaitOneTick` through it. A free actor’s wait remains an ordinary semantic proposal; accepted wait now advances the full world. Prohibit direct state/event mutation and direct debug-runner reuse.

#### Ticket 9 — Duration continuation and advance-until controller

Add the explicit continuation control and repeated-step loop with terminal, actor-known salience, user-pause, error, and deterministic-bound stops. Add command-loop tests for sleep and work progression.

#### Ticket 10 — Actor-known interval projection and debug split

Implement the source-bearing interval projection, integrate it into the embodied view boundary, render it exhaustively, and keep exact world-step diagnostics in a separate debug-only type. Add positive and negative epistemic tests.

#### Ticket 11 — Human sleep completion real-pipeline witness

Through the actual TUI parser/app/core path, start sleep, reject ordinary wait during the reservation, continue time, reach terminal, verify per-tick recovery and hunger accounting, render an actor-known summary, and replay/rebuild to the same state.

#### Ticket 12 — Human work completion real-pipeline witness

Through the same path, start work, continue time, reach success or modeled failure, verify output and need accounting, render safe feedback, and replay/rebuild. Include a physical-precondition change or interruption case if supported by the accepted lifecycle.

#### Ticket 13 — Full-world/possession/anti-leak differential suite

Hold initial log and deterministic inputs constant; compare human wait with equivalent no-human/controller progression. Verify other actors/processes advance, possession does not alter authoritative outcomes, hidden other-actor activity does not enter the interval summary, and actor-known communication does.

#### Ticket 14 — Spec-0046 parity registry and two-hop closure

Add time-control setup operations, capability entries/dispositions, Hop-1 real-pipeline goldens, Hop-2 exhaustive view-model/render guards, adversarial anti-leak cases, replay/no-human dispositions, and ordinary CI integration under PAR-010/PAR-011.

#### Ticket 15 — Acceptance artifact and ledger routing

Run the future spec’s commands at an exact implementation/evidence commit; package event ledgers, checksums, replay reports, parity census, negative witnesses, and scope limitations using the existing acceptance template. Update `SPEC_LEDGER.md` only after acceptance, with no latest-main overclaim.

### 5.3 Recommended review gates for the future spec

Without inventing new gate identifiers, the future spec should require reviewers to answer yes to all of these before acceptance:

- Is there exactly one semantic definition of a loaded-world tick?
- Can a duration started before the current coordinator invocation be found and completed after rebuild?
- Does ordinary human wait produce one—and only one—charge classification for each affected need/tick?
- Does sleeping/working continuation avoid an `ActorWaited` event for that actor?
- Do unpossessed actors and due processes advance under the same timeline?
- Can replay reconstruct the temporal frontier even for ticks with no other emitted effect?
- Is the embodied interval summary constructible without raw physical state or a global event slice?
- Does a hidden other-actor event leave the embodied summary unchanged unless modeled information reaches the actor?
- Do debug-only exact details remain structurally unavailable to embodied rendering/planning?
- Are human, no-human, replay, and parity witnesses all real-pipeline evidence rather than synthetic event insertion?
- Does every changed playable capability or view-model field receive a spec-0046 surface disposition?

---

## 6. Open questions

The primary architecture is resolved. The following implementation questions remain for the future spec and are not blockers to routing the work:

### 6.1 Exact intra-tick phase order

The coordinator’s responsibilities are clear, but the precise order among tick marker, due terminal evaluation, passive need accounting, actor decisions, event application, perception, and projection should be frozen only after running compatibility tests against existing ordinary-life and replay fixtures. The order must be singular and shared; this report deliberately does not invent an untested phase numbering.

### 6.2 Early interruption semantics for each duration kind

The repository has terminal builders and interruptible duration metadata, but the future spec must enumerate which modeled conditions interrupt sleep/work before expected completion, at which phase they are detected, and which actor-visible reasons are safe. This is lifecycle detail, not a reason to retain batch-local completion.

### 6.3 Salience policy

“Salient perceived interruption” needs a typed policy: which observations stop acceleration, whether multiple same-tick observations are summarized together, and how actor preferences configure salience without changing world outcomes. The default should be conservative and source-bearing. Hidden events must never become salient merely because debug knows them.

### 6.4 Interval-summary persistence and temporal precision

The preferred initial design is one source-backed “last completed advance interval” value on the embodied projection, replaced on the next completed advance. The future spec should confirm acknowledgment/clearing behavior and whether exact tick labels are actor-legible in each fixture under existing temporal-authority doctrine. Internal exact ticks remain available for replay/debug regardless of rendered precision.

### 6.5 Deterministic safety bound and very long advances

The first implementation should loop fixed ticks with a deterministic maximum and report a controller-bound stop without turning it into in-world knowledge. Storage/performance compaction for very long simulation spans should be deferred until evidence shows a need; any later optimization must preserve intermediate causality, per-tick accounting, and replay ancestry.

### 6.6 Loaded-world boundary

The recommendation covers the current authoritative loaded simulation. Future regional/LOD work must define how unloaded or coarsened regions participate in the same temporal frontier, but that deferred architecture should not be pulled into this repair.

---

## 7. References

### 7.1 User-supplied control material

- `manifest_2026-06-22_6e91da7.txt` — path inventory for the user-supplied target commit.
- `tui-human-wait-runs-simulation-research-brief.md` — task scope, authority order, settled intentions, provenance requirements, and deliverable contract.

### 7.2 Load-bearing target-repository sources

All repository sources below were fetched at exact commit `6e91da79b81bf9cbf25acba726e58183920dc441`; the complete exact URL list appears in §2.2.

**Authority and doctrine**

- `docs/README.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — especially **Time controls**, lines 257–276 at the target commit.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — especially lines 96–108.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — especially lines 31–51.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — especially lines 144–149.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/4-specs/SPEC_LEDGER.md`
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`

**Precedent and seed**

- `archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md`
- `archive/reports/0046-parity-acceptance-artifact.md`
- `reports/tui-human-wait-runs-simulation-issue.md`

**Implementation and proof seams**

- `crates/tracewake-core/src/actions/defs/sleep.rs`
- `crates/tracewake-core/src/actions/defs/work.rs`
- `crates/tracewake-core/src/actions/defs/wait.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/src/actions/registry.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/need_accounting.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/time.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/run.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/tests/playable_capability_parity.rs`
- `crates/tracewake-tui/tests/parity/*`
- `crates/tracewake-tui/tests/parity_adversarial.rs`
- `crates/tracewake-tui/tests/tui_acceptance.rs`
- `crates/tracewake-tui/tests/command_loop_session.rs`
- `crates/tracewake-content/src/fixtures/wait_then_window_passive_charges_each_tick_once_001.rs`
- `crates/tracewake-content/src/fixtures/sleep_spanning_window_boundary_charges_each_tick_once_001.rs`
- `crates/tracewake-content/src/fixtures/scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs`

### 7.3 External research

[^fowler]: Martin Fowler, [“Event Sourcing”](https://martinfowler.com/eaaDev/EventSourcing.html). Used for the event-log/rebuild and temporal-query rationale.

[^fiedler]: Glenn Fiedler, [“Fix Your Timestep!”](https://gafferongames.com/post/fix_your_timestep/). Used for the repeated bounded-step rationale rather than unbounded temporal jumps.

[^bevy-fixed]: Bevy documentation, [`FixedUpdate`](https://docs.rs/bevy/latest/bevy/app/struct.FixedUpdate.html). Used as an additional fixed-schedule comparison; it does not define Tracewake’s ordering.

[^simpy-env]: SimPy documentation, [“Environments”](https://simpy.readthedocs.io/en/latest/topical_guides/environments.html). Used for the distinction between one step and run-until-event control.

[^simpy-proc]: SimPy documentation, [“Process Interaction”](https://simpy.readthedocs.io/en/latest/topical_guides/process_interaction.html). Used for event suspension and interruption comparison.

[^nethack]: NetHack 3.6.5 Guidebook, [command reference](https://www.nethack.org/v365/Guidebook.html). Used only as a one-turn wait ergonomics precedent.

[^angband]: Angband documentation, [“Command Descriptions”](https://angband.readthedocs.io/en/latest/command.html). Used for interruptible rest-until behavior as a UX precedent.

[^command-pattern]: Robert Nystrom, *Game Programming Patterns*, [“Command”](https://gameprogrammingpatterns.com/command.html). Used for the shared human/AI command-boundary comparison.

[^pomdp]: Leslie Pack Kaelbling, Michael L. Littman, and Anthony R. Cassandra, [“Planning and Acting in Partially Observable Stochastic Domains”](https://people.csail.mit.edu/lpk/papers/aij98-pomdp.pdf), *Artificial Intelligence* 101 (1998). Used for the observation/belief rather than direct-state-access rationale.

[^esdb-projection]: EventSourcingDB documentation, [“Building Event-Driven Applications”](https://docs.eventsourcingdb.io/best-practices/building-event-driven-applications/). Used for deterministic replayable projection pressure.

[^esdb-testing]: EventSourcingDB documentation, [“Testing Event-Sourced Systems”](https://docs.eventsourcingdb.io/best-practices/testing-event-sourced-systems/). Used for replay/projection testing pressure.
