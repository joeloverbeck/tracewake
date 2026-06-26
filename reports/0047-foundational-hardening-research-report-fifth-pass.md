# 0047 foundational-conformance hardening — fifth-pass research report

Target repository: `joeloverbeck/tracewake`  
Target commit: `e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f`  
Report filename: `0047-foundational-hardening-research-report-fifth-pass.md`  
Report mode: static exact-commit survey and process-integrity recommendation; not a numbered remediation spec.

---

## 1. Verdict

### Code-surface verdict

The spec-0047 loaded-world / time-control / TUI-authority surface at `e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f` is **not fully foundationally conformant as a closed surface**.

Spec 0052 did close several real defects. The current source has a core-owned `LoadedWorldRuntime`, a closed `RuntimeCommandKind`, a TUI path that loads fixtures through `LoadedFixture::into_runtime_bootstrap` and `LoadedWorldRuntime::from_bootstrap`, runtime-owned scheduler derivation, normal `continue` rendering that avoids exact stop ticks, and CI workflow jobs that name the intended public-boundary and mutation-lock lanes. Those properties are **present**, and this report does not re-commission them as if absent.

The surface still fails closure for five decisive reasons:

1. **Production bootstrap remains externally forgeable or, at minimum, not proven unforgeable at the public boundary.** `LoadedWorldBootstrap::from_loaded_state` is public and accepts raw `ActionRegistry`, `PhysicalState`, `AgentState`, `EventLog`, and `EpistemicProjection`; `PhysicalState::from_seed_parts` and `AgentState::from_seed_parts` are public seed-part constructors. The validated content path uses that public constructor rather than a constructor that is unrepresentable outside the validated authority path. The existing negative fixture proves scheduler-injection methods are not public; it does **not** prove an external crate cannot fabricate an unvalidated loaded-world bootstrap.

2. **The embodied interval / runtime receipt product is still not fully sealed at the core boundary.** The normal renderer hides exact ticks and stop reasons, which is good, but `RuntimeReceiptKind::Continued(AdvanceUntilResult)` publicly carries the raw advance result; `TuiApp::advance_until` constructs `TypedActorKnownIntervalSummary::from_actor_known_delta` in the TUI; and `EmbodiedViewModel` / `TypedActorKnownIntervalSummary` expose public setters and exact getters for tick, frontier, stop reason, and context metadata. That is materially weaker than the 0052 claim that TUI “consumes the core interval product read-only.”

3. **Debug/operator authority is not structurally required by the public runtime command constructor.** `RuntimeCommandKind` is closed inside the core crate, but public constructors such as `RuntimeCommand::run_no_human_day()` mint the debug capability internally. The TUI command loop checks debug mode before invoking this path, but the core public API itself does not require a caller-held debug/operator token. If `run no-human-day` is debug/operator-only, this is an authority gap; if it is ordinary play, the surface is mislabeled and the debug split is overstated.

4. **The anti-regression gate exists in files but is not enforced at merge.** The 0052 acceptance records that the workflow defines the intended jobs, but also records that a GitHub API check returned `Branch not protected (HTTP 404)` for `main`; therefore required status checks are not operationally enforced for merge. A gate that can be bypassed by merging is a topology alarm, not a standing barrier.

5. **The canonical standing mutation perimeter is still not green, and the seven `food_source_fact_supersedes` survivors have no forcing function that prevents indefinite routing-forward.** This is not an in-surface code-closure blocker for the loaded-world/TUI seam, but it is a cross-cutting mutation-evidence residual that prevents any unqualified “green standing perimeter” claim.

### Process-integrity verdict

The acceptance / verification process is **not sound enough to stop recurrence**. The five-artifact lineage shows a structural failure mode: a remediation line can define its own scope, run focused witnesses, truth docs to the implemented shape, record residuals as “scoped pass,” and archive an acceptance artifact whose prose looks closed even when later audits find that the protected authority class survived. The repository doctrine already contains strong evidence-honesty rules, but the process lacks an enforced, machine-checkable result taxonomy and an operational branch-protection / required-check policy that makes “pending,” “routed-forward,” “historical,” “sampled,” or “observer-only” evidence impossible to count as pass.

### Higher-tier amendment verdict

No Tier-0 foundation amendment is warranted. The constitution already says the relevant things: causality is event-sourced, replay must be authoritative, actor cognition must be holder-known and source-bound, possession must not create privileged truth, debug truth must not feed embodied output, and prose cannot mint simulation facts. The code/process defects are failures to enforce those rules, not contradictions in the rules.

A **doctrine strengthening is warranted below foundation**, as substance and home only:

- **Architecture home:** `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`. Add that an acceptance artifact may not render an overall “pass” while any flagged foundational violation, required governance control, or standing mutation residual remains open unless the artifact’s result is explicitly non-closing and the residual has a machine-tracked forcing function.
- **Execution home:** `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`. Add the fail-closed acceptance manifest / result taxonomy and required-status enforcement rule: `pass` is computed only from certifying current evidence; `pending`, `routed-forward`, `historical`, `sampled`, `observer-only`, and “pass with disposition” are not pass.
- **Reference home:** `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` and `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, status/navigation updates only, using existing R-27/R-28/R-29 evidence/status rows and minting no new risk ID.

The governing docs already establish the authority order, with foundation outranking architecture, execution, reference, and specs; the architecture and execution evidence-honesty docs already reject display-string-only, fixture-only, or historical proof. The missing piece is operationalizing those rules so an acceptance artifact cannot overclaim. citeturn849085view0turn742222view0turn955256view3turn955256view2

---

## 2. Disposition table

| Finding | Primary target | Classification | One-line basis |
|---|---|---|---|
| F5-01 Public loaded-world bootstrap can bypass validated authority | `crates/tracewake-core/src/runtime/session.rs`; `crates/tracewake-core/src/state.rs`; `crates/tracewake-content/src/load.rs`; docs `1-architecture/04`, `2-execution/05`, `1-architecture/13` | Violation / vacuity gap | The validated content path reaches `LoadedWorldRuntime::from_bootstrap`, but the raw `LoadedWorldBootstrap::from_loaded_state` constructor and seed-part constructors are public, so production-bootstrap authority is not unrepresentable outside validation. |
| F5-02 Embodied interval and receipt products remain partially client-assembled and exact | `crates/tracewake-core/src/runtime/receipt.rs`; `crates/tracewake-core/src/view_models.rs`; `crates/tracewake-tui/src/app.rs`; docs `1-architecture/10`, `2-execution/07` | Violation / hardening gap | TUI constructs `TypedActorKnownIntervalSummary` from raw runtime results, and public getters/setters expose exact tick/frontier/stop metadata beyond a sealed read-only embodied product. |
| F5-03 Debug/no-human runtime command authority is not token-gated at core public API | `crates/tracewake-core/src/runtime/command.rs`; `crates/tracewake-core/src/runtime/receipt.rs`; `crates/tracewake-tui/src/run.rs`; docs `1-architecture/10`, `2-execution/07` | Hardening gap / authority gap | UI checks debug availability, but public runtime command constructors can request debug/no-human behavior without a caller-held unforgeable debug/operator authority token. |
| F5-04 Standing gate is not merge-enforced | `.github/workflows/ci.yml`; `crates/tracewake-core/tests/ci_workflow_guards.rs`; repository branch protection/ruleset settings; docs `2-execution/10` | Evidence-honesty / governance gap | 0052 wired named jobs but recorded `main` branch protection as absent; required status checks are not operational enforcement without branch protection or rulesets. |
| F5-05 Seven `food_source_fact_supersedes` survivors remain routed-forward | `crates/tracewake-core/src/projections.rs`; `.cargo/mutants.toml`; mutation evidence docs; docs `2-execution/10`, `docs/4-specs/SPEC_LEDGER.md` | Mutation-survivor disposition | 0052 correctly did not claim a fully green canonical perimeter, but there is no forcing function preventing this cross-cutting survivor family from persisting indefinitely. |
| F5-06 Acceptance process can still launder open defects into scoped pass artifacts | `docs/1-architecture/13`; `docs/2-execution/10`; `.github/workflows/ci.yml`; `crates/tracewake-core/tests/acceptance_artifact_wording.rs`; `crates/tracewake-core/tests/ci_workflow_guards.rs`; spec ledger process | Process-integrity gap | Prior artifacts repeatedly rendered pass/scoped-pass while later audits found live foundational defects; result wording and evidence status are not machine-enforced. |

---

## 3. Method & provenance ledger

### Authority order applied

Repository state claims in this report rely only on files fetched from exact commit URLs for paths present in the uploaded manifest. The manifest is path inventory only. It is not file-content evidence.

Authority was read in the repository’s declared order: `docs/README.md`; Tier 0 foundation; Tier 1 architecture; Tier 2 execution; Tier 3 reference; Tier 4 specs/reports as history and lineage. The report treats earlier tiers as governing later tiers. citeturn849085view0turn512058view0turn903029view0turn792242view0

### Source-equivalence posture

The user brief states that the 0052 keystone source commit `8e84150228e82d29dfddf2e9f52f201c3cf10c9c` is byte-identical to baseline `e9792dc` for `crates/tracewake-core/src` and `crates/tracewake-tui/src`. I did not independently run `git diff`; this was treated as user-supplied control material. The source claims below are based on exact-commit file fetches at `e9792dc`, and 0052 acceptance claims are treated as historical claims to re-verify, not as proof.

### Static-survey limitation

I did not run `cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`, CI, or `cargo mutants`. Every statement about current behavior, test strength, mutation posture, or pass/fail is a static source-shape judgment. Historical command results are quoted only as claims made by the corresponding historical acceptance artifact.

### Mandatory acquisition ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.open on full raw.githubusercontent.com exact-commit URLs
Requested file count: 109
Successfully verified file count: 109
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

Every requested URL contained the exact owner `joeloverbeck`, repository `tracewake`, full commit `e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f`, and the exact manifest path. The fetch tool returned file contents from the requested raw URL, not search results, branch pages, repository overviews, snippets, or metadata substitutes.

#### Fetched repository files

- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/reports/0047-foundational-hardening-research-report-fourth-pass.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/reports/0047-foundational-hardening-research-brief-fourth-pass.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/archive/specs/0052_FOUNDATIONAL_CONFORMANCE_FOURTH_HARDENING_PRODUCTION_BOOTSTRAP_CLOSED_COMMAND_BOUNDARY_REPLAY_AUTHORITY_REAL_PROCESSES_ACTOR_CENSUS_EMBODIED_DEBUG_SPLIT_AND_STANDING_BARRIER_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/reports/0047-foundational-hardening-research-report-third-pass.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/archive/reports/0051_foundational_conformance_third_hardening_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/archive/reports/0050_foundational_conformance_second_hardening_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/reports/0048_foundational_conformance_hardening_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/archive/reports/0047_tui_authoritative_world_advance_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/archive/specs/0050_FOUNDATIONAL_CONFORMANCE_SECOND_HARDENING_LOADED_WORLD_DISCOVERY_ACTOR_TRANSACTION_UNIFICATION_TUI_DEAUTHORITY_AND_REPLAY_FAIL_CLOSED_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/archive/specs/0051_FOUNDATIONAL_CONFORMANCE_THIRD_HARDENING_CORE_OWNED_LOADED_WORLD_RUNTIME_PRODUCTION_REACHABILITY_PROCESS_TRANSACTIONS_ACTOR_CENSUS_AND_TUI_DEAUTHORITY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/.cargo/mutants.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/.cargo/mutants-baseline-misses.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/runtime/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/runtime/session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/runtime/command.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/runtime/receipt.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/events/envelope.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/events/log.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/agent/transaction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/agent/trace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/agent/perception.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/epistemics/projection.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/replay/temporal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-content/src/load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/src/run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/src/launch.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/world_step_coordinator.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/negative_fixture_runner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/generative_lock.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/ci_workflow_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/replay_temporal_frontier.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/holder_known_interval_projection.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/salient_stop_actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/reservation_body_exclusive_census.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-core/tests/mutation_completion_merge.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/tests/command_loop_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/tests/playable_capability_parity.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/tests/tui_seam_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/tests/parity_adversarial.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/crates/tracewake-tui/tests/transcript_snapshot.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f/tests/negative-fixtures/external_crate_cannot_seed_loaded_actor_or_process_eligibility/src/lib.rs

### External research lane

External research was used only for enforcement and prior-art reasoning, not to assert target-repository state. Load-bearing external sources were: Rust visibility/privacy and `non_exhaustive`; GitHub branch protection / required status checks; cargo-mutants missed-mutant guidance; Temporal replay-from-history; Schneider’s state-machine approach; Sabelfeld & Myers on information-flow control; and Goodhart-style metric displacement.

---

## 4. Re-verified present properties from 0052

This pass should not erase real 0052 progress. These properties are present in the exact-commit source and should be preserved while fixing the remaining gaps.

### Present: TUI golden launch enters the runtime bootstrap path

`TuiApp::from_golden` loads the fixture package, calls `LoadedFixture::into_runtime_bootstrap(registry)`, and constructs `LoadedWorldRuntime::from_bootstrap(...)`. The content loader validates fixture/package data before building the runtime bootstrap; the runtime constructor derives its scheduler from the loaded world rather than accepting a caller-seeded scheduler. This is a meaningful closure of the old “TUI-owned scheduler/loaded actors” class. citeturn579985view0turn678354view0turn108540view1

### Present: Runtime owns scheduler derivation and world-step choreography

`LoadedWorldRuntime` contains private state/log/projection/scheduler/controller fields and `from_bootstrap` derives the deterministic scheduler from loaded-world authority. The scheduler has `due_loaded_actor_ids`, `due_process_invocations`, `transact_world_one_tick`, replay restore, proposal sequencing, and per-tick actor dispositions. The architecture and execution docs require exactly this single core-owned world-step coordinator. citeturn108540view1turn708231view1turn771100view1turn771100view4

### Present: Closed `RuntimeCommandKind` reduces direct TUI mutation

`RuntimeCommandKind` is `pub(crate)`, and public command constructors create typed `RuntimeCommand` values consumed by `LoadedWorldRuntime::submit_command`. The TUI paths call runtime commands for semantic action submission, wait/continue, binding, and no-human day instead of directly mutating raw aggregates. This is a real improvement over earlier parallel TUI paths. citeturn254492view0turn579985view0turn724910view0

### Present: Normal command-loop rendering no longer prints exact continuation internals

The command loop renders `continue` with qualitative “actor-known interval updated” wording and prints debug detail only through debug-marked panels. `render_embodied_view` prints the interval summary as an actor-facing qualitative section rather than displaying exact stop tick / frontier details. citeturn724910view0turn724910view3turn191915view3

### Present: 0052 acceptance honestly recorded its two residuals

The 0052 acceptance did not claim a fully green canonical mutation perimeter: it recorded seven routed-forward `food_source_fact_supersedes` survivors and zero timeouts. It also recorded that branch protection was not confirmed and that the GitHub API returned `Branch not protected (HTTP 404)`. Those statements are useful because they are not overclaims; the problem is that the process still allowed an artifact to look like a pass for the line while those residuals remained live. citeturn976700view2turn976700view4

---

## 5. Per-finding sections — code surface

### F5-01 — Public loaded-world bootstrap can bypass validated authority

#### Foundational driver

The relevant foundation is event-sourced causality, replay authority, no fact born from prose, and the truth firewall. Architecture requires a single core-owned loaded-world coordinator and acceptance evidence that starts from the production runtime constructor from a **validated** bootstrap, not from harness fabrication. Execution requires the canonical world-step to own due-work discovery, event append/apply, and replay/projection completion without caller-injected actor lists or raw process envelopes. citeturn742222view0turn771100view1turn955256view4turn771100view4

#### Current `e9792dc` code state

`LoadedWorldBootstrap::from_loaded_state` is a public constructor taking raw `ActionRegistry`, `PhysicalState`, `AgentState`, `EventLog`, and `EpistemicProjection`. `LoadedWorldRuntime::from_bootstrap` then consumes this object and derives scheduler authority. The content loader path does validate and then calls the same public constructor in `LoadedFixture::into_runtime_bootstrap`. `PhysicalState::from_seed_parts` and `AgentState::from_seed_parts` are also public. citeturn108540view1turn678354view0turn678354view1

The existing negative fixture `external_crate_cannot_seed_loaded_actor_or_process_eligibility` checks that external crates cannot call scheduler methods such as `schedule_loaded_actor_decision` or `register_cadenced_world_process`; that is useful but narrower than bootstrap integrity. It does not attempt to construct a raw `LoadedWorldBootstrap` from seed parts or prove such construction is impossible. citeturn479357view0turn410948view1

#### Conformance verdict

**Violation / vacuity gap.** The production path is better, but the authority class is not structurally closed at the public API. If arbitrary external code can assemble a bootstrap from raw seed parts and pass it to `LoadedWorldRuntime::from_bootstrap`, then validation is a convention rather than a type boundary. If some of the raw arguments happen to be hard to construct today, the acceptance evidence is still vacuous because the negative perimeter did not prove unrepresentability for this exact attack.

Rust privacy is the correct enforcement tool here: public items are accessible externally, while private or `pub(crate)` items are not; a public struct with private fields can block direct field construction, but a public constructor reopens construction by API. `#[non_exhaustive]` is not a substitute for authority sealing; within the defining crate it has no effect, and outside the crate it is primarily an evolution/constructor restriction with specific limits. citeturn754265view1turn754265view4

#### Required remediation

Code home:

- `crates/tracewake-core/src/runtime/session.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-content/src/load.rs`
- External negative fixtures under `tests/negative-fixtures/`

The remediation must make “validated loaded-world bootstrap” unrepresentable outside the authority path. Because `tracewake-content` is a separate crate, a simple `pub(crate)` on `LoadedWorldBootstrap::from_loaded_state` may require an authority-boundary restructuring. Viable routes, in preferred order:

1. **Move bootstrap construction and validation into the same authority crate as the runtime.** Core receives authored fixture/package material or a normalized validated package and mints the bootstrap internally. Content no longer calls a raw public constructor.
2. **Introduce an unforgeable validated-bootstrap token whose constructor is private to the runtime authority path.** The public runtime constructor accepts only a sealed `ValidatedLoadedWorldBootstrap` product, not raw state/log/projection parts.
3. **If crate topology prevents either approach, create a dedicated internal authority crate and invert dependencies so only that crate can mint the bootstrap.** Do not use Cargo features as security; external crates can enable public features.

No backwards-compatible alias should remain. The old raw constructor should be deleted, not deprecated.

Docs home after executable closure:

- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — update current evidence row to name the sealed bootstrap constructor, not a public raw constructor.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — update the no-direct-dispatch proof obligation to include bootstrap unforgeability.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — add that production-constructor evidence must include a negative fixture for raw bootstrap fabrication.

#### Strongest practical anti-regression guard

Add an external negative fixture, compiled by `negative_fixture_runner.rs`, named along these lines: `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts`. It should try to:

- import `LoadedWorldBootstrap`;
- call `LoadedWorldBootstrap::from_loaded_state` or any successor raw constructor;
- construct `PhysicalState` / `AgentState` from seed parts;
- pass the result to `LoadedWorldRuntime::from_bootstrap`.

The fixture must fail to compile for authority reasons. A runtime rejection is weaker than unrepresentability; compile failure is the target.

Also add a positive public-boundary witness that the real content loader still produces a runtime through the sealed path and that no scheduler actor/process registration is caller-injected.

#### Evidence-honesty check

A non-vacuous closure artifact must include the failing external fixture, the positive validated-loader witness, and a source-shape check proving no public raw bootstrap constructor remains. A closure artifact is vacuous if it only asserts that `TuiApp::from_golden` uses the validated path while leaving an equally public raw path beside it.

---

### F5-02 — Embodied interval and receipt products remain partially client-assembled and exact

#### Foundational driver

The TUI may render actor-filtered products; it must not become authoritative simulation, re-read hidden truth, own duration terminals, maintain a local possessed-actor clock, or merge debug truth into actor-facing output. Architecture requires core to expose typed actor-filtered view models and explicitly separates embodied interval summaries from debug step reports. Acceptance rejects display-string-only proof of temporal correctness. citeturn404683view2turn404683view3turn955256view4

#### Current `e9792dc` code state

`RuntimeReceiptKind` publicly exposes variants including `Continued(AdvanceUntilResult)` and `OneTickAdvanced(WorldAdvanceResult)`. The embodied/debug receipt wrappers exist, but the raw runtime receipt kind still carries exact internal products to public callers. citeturn254492view1

`TuiApp::advance_until` receives `RuntimeReceiptKind::Continued(result)`, calls `TypedActorKnownIntervalSummary::from_actor_known_delta(&result.actor_known_interval_delta)`, and stores the summary in `last_interval_summary`. That means the TUI still composes the interval summary from a raw runtime result rather than merely rendering a sealed core product. citeturn579985view0

`EmbodiedViewModel` carries exact `sim_tick`, holder-known context ID/hash/frontier/source, and an optional actor-known interval summary; it has public getters for these exact values and public mutators such as `set_actor_known_interval_summary`, `set_debug_available`, and `set_notebook`. `TypedActorKnownIntervalSummary` has a public `from_actor_known_delta` constructor and public getters for start tick, stop tick, start frontier, stop frontier, stop reason, and whether no new actor-known information exists. citeturn724910view4

The normal renderer is disciplined: it prints qualitative interval text and debug panels carry non-diegetic markers. But hiding exact values during the current renderer path is not equivalent to sealing the public product. citeturn724910view3turn191915view3

#### Conformance verdict

**Violation / hardening gap.** The embodied display is improved, but the product boundary remains too wide. The TUI is not just rendering an opaque actor-known interval product; it constructs one from raw result data. External clients can observe exact interval fields through public getters even if the command loop does not print them. This contradicts the architecture claim that the TUI consumes core-owned read-only interval summaries and keeps exact temporal/replay internals debug-only.

#### Required remediation

Code home:

- `crates/tracewake-core/src/runtime/receipt.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/render.rs`

Required changes:

1. Replace public `RuntimeReceiptKind::Continued(AdvanceUntilResult)` exposure with a sealed public receipt surface. Normal callers receive an embodied receipt containing only actor-known, source-bearing, qualitative interval data. Debug/operator callers receive exact tick/frontier/stop/replay details through a separately token-gated debug receipt.
2. Move `TypedActorKnownIntervalSummary` construction fully into core. TUI should receive and store it as an already-sealed product, or receive an opaque renderable enum/data transfer object that cannot expose exact debug fields.
3. Remove or restrict public exact getters on the embodied interval product. Exact ticks, frontiers, stop reasons, and context hashes should be debug/operator-only unless architecture explicitly says a specific exact value is actor-known via a modeled source.
4. Remove public mutators on `EmbodiedViewModel` that let client code append notebook/debug/interval fields after core construction. Core may use crate-private builders; public clients should receive immutable products.

Docs home after executable closure:

- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — update current 0052 evidence row to name the sealed product and distinguish public embodied vs. debug/operator exact fields.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — update the proof obligation to include external-client compile-fail checks for exact interval getters and setters.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — require typed evidence that interval products are core-constructed and immutable from the public client boundary.

#### Strongest practical anti-regression guard

Add external negative fixtures proving external crates cannot:

- call `TypedActorKnownIntervalSummary::from_actor_known_delta`;
- call exact tick/frontier/stop getters on the embodied interval product;
- call `EmbodiedViewModel::set_actor_known_interval_summary` or `set_debug_available`;
- pattern-match public `RuntimeReceiptKind` to extract raw `AdvanceUntilResult` exact fields.

Add positive tests through `TuiApp::advance_until` and command-loop `continue` proving the normal path receives an actor-known update and cannot render exact internals. Keep the existing render tests, but do not let render tests alone certify sealing.

Add mutation sensitivity around the public product accessors: every surviving “return default tick/frontier/stop reason” mutant on the interval product must be either killed through a public behavior witness or removed from the public embodied API.

#### Evidence-honesty check

Closure is non-vacuous only if the TUI source no longer constructs interval summaries from raw advance results. A report that says “normal render hides exact ticks” while exact getters remain public is not a closure witness.

---

### F5-03 — Debug/no-human runtime command authority is not token-gated at the core public API

#### Foundational driver

The TUI must keep debug view models non-diegetic, debug truth must not become actor knowledge, and no future client may become authoritative simulation or create player-only verbs. The execution no-direct-dispatch contract forbids special possessed-player paths and debug/fixture helpers in live gameplay paths. citeturn404683view2turn404683view3turn771100view4

#### Current `e9792dc` code state

`RuntimeCommandKind` is crate-private, which is good. But `RuntimeCommand::run_no_human_day()` is a public constructor and internally mints `DebugCapability`. The TUI parser exposes “debug run no-human-day,” and `run.rs` checks `debug_available()` before calling `app.run_no_human_day()`. The protection is therefore at the TUI layer; the public core command constructor itself does not require the caller to present an unforgeable debug/operator authority token. citeturn254492view0turn665871view1turn724910view0

Debug view constructors also deserve scrutiny: some debug products carry private capability fields, but public constructors that mint the capability internally are not equivalent to caller-held authority. citeturn191915view2turn665871view3

#### Conformance verdict

**Hardening gap / authority gap.** If no-human day is a debug/operator command, then a public command constructor that mints its own debug capability makes the capability decorative. If no-human day is an ordinary player time-control command, then it should not be represented as debug-only in the TUI command vocabulary and evidence. The current split is ambiguous and not fail-closed.

#### Required remediation

Code home:

- `crates/tracewake-core/src/runtime/command.rs`
- `crates/tracewake-core/src/runtime/receipt.rs`
- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/run.rs`
- `crates/tracewake-tui/src/input.rs`

Required decision and implementation:

1. Decide whether `run_no_human_day` is ordinary play acceleration or debug/operator execution.
2. If ordinary: remove the debug marker from the command vocabulary and receipt path, prove it uses the same core world-step boundary, and ensure it does not expose debug metrics to embodied output.
3. If debug/operator: require an unforgeable `DebugSessionAuthority` or equivalent token supplied by the runtime/controller binding state. The public constructor must not mint the token internally.
4. Debug receipt/view constructors should require the same token or be crate-private builders fed by runtime-owned debug APIs.

Docs home after executable closure:

- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`

#### Strongest practical anti-regression guard

Add an external negative fixture proving that an external crate cannot construct or submit a debug/operator runtime command without a runtime-minted debug session token. Add a behavioral TUI test showing the command loop rejects `debug run no-human-day` when not bound in debug mode and succeeds only with the token path.

#### Evidence-honesty check

A closure witness is vacuous if it only asserts that `tracewake-tui` checks debug mode. The protected claim is about the public runtime boundary, so the negative fixture must import `tracewake_core` directly and fail there.

---

### F5-04 — Standing gate is not merge-enforced

#### Foundational driver

Architecture says acceptance must prove causality and anti-contamination through typed, path-under-test evidence; execution says pending, sampled, observer-only, and historical evidence must never silently count as pass, and that red scheduled mutation is merge-blocking until repaired. The 0052 acceptance itself records that branch protection was not confirmed. citeturn955256view3turn955256view2turn976700view4

#### Current `e9792dc` code/process state

`.github/workflows/ci.yml` defines `public-boundary-conformance`, `full-surface-mutation-trigger`, and `mutants-lock-layer-reconcile` jobs. The workflow runs on PR/push and scheduled/manual mutation lanes. `ci_workflow_guards.rs` asserts aspects of the workflow file. citeturn912811view2turn912811view6turn410948view4

But the 0052 acceptance records that a read-only API check for branch protection returned `Branch not protected (HTTP 404)` for `main`. Therefore these checks are not required for merge at the repository setting level. GitHub’s branch-protection documentation is direct on this point: protected branches can require status checks and other requirements before merges, and after required status checks are enabled all required checks must pass before collaborators can merge changes. Without that setting, collaborators can merge without those required checks. citeturn976700view4turn983062view2

#### Conformance verdict

**Evidence-honesty / governance gap.** A workflow job definition is not an enforced standing gate unless repository settings require it before merge. The current repository evidence supports “jobs are defined,” not “the gate is merge-enforced.” This is a first-class anti-regression defect because the recurring seam is specifically an enforcement failure.

#### Required remediation

Governance home:

- GitHub branch protection or repository ruleset for `main`.

Code/doc home:

- `.github/workflows/ci.yml`
- `crates/tracewake-core/tests/ci_workflow_guards.rs`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

Required settings:

- Protect `main` or create an equivalent ruleset.
- Require pull requests for changes to protected code/docs paths.
- Require the workspace gates and `public-boundary conformance` check.
- Require the lock-layer mutation trigger/reconciliation status appropriate to changed paths.
- Do not allow bypassing required checks for normal maintainers; if admin bypass remains possible, every bypass must produce an explicit governance residual and cannot be called pass.
- Require the branch to be up to date with base or use merge queue to rerun required checks on the merge result.

#### Strongest practical anti-regression guard

Add a CI/governance audit job that queries the branch protection/ruleset API and fails if the required status checks are not configured. If API permissions are unavailable for forked PRs, the job must report `pending/unverified`, and acceptance must not count it as pass. Keep `ci_workflow_guards.rs` as a topology alarm for job names and path filters, but do not treat it as merge-enforcement proof.

#### Evidence-honesty check

A non-vacuous closure artifact must include an API transcript showing branch protection/ruleset configuration on the exact repository/branch and a PR merge box / ruleset proof that the named checks are required. A screenshot alone is weaker than an API transcript; a workflow YAML grep is not enough.

---

### F5-05 — Seven `food_source_fact_supersedes` survivors remain routed-forward

#### Foundational driver

Mutation evidence is not a product behavior invariant by itself, but execution doctrine says missed mutants are actionable evidence and pending is not pass. cargo-mutants describes a “missed” mutant as a likely coverage gap or possibly an indistinguishable mutant; the recommended response is to inspect and preferably add a public-level behavior test rather than merely asserting the private function. citeturn955256view2turn585453view0

#### Current `e9792dc` code state

`food_source_fact_supersedes(candidate, chosen)` chooses between facts using presence/absence of `food_source` and then `source_key` ordering. The 0052 acceptance records exactly seven surviving mutants in this function: constant true, constant false, deletion of the two `Some/None` arms, and three replacements of `<` with `==`, `>`, or `<=`. It explicitly routes them forward as out-of-surface and does not call the canonical perimeter fully green. citeturn254492view2turn976700view2

#### Conformance verdict

**Mutation-survivor disposition, not an in-surface closure target.** The 0052 disposition is basically honest: the survivors are cross-cutting and should not be reclassified into a lower tier merely because this pass is about the 0047 surface. But a route-forward disposition without a forcing function is process debt; it keeps the canonical standing perimeter non-green indefinitely.

#### Required remediation

Code home:

- `crates/tracewake-core/src/projections.rs`
- Public behavior tests that exercise food-source belief replacement via actor-known/projection paths, not private-function-only tests.

Mutation/config home:

- `.cargo/mutants.toml`
- `.cargo/mutants-baseline-misses.txt` or a successor survivor registry if the project adopts one.

Docs home:

- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `docs/4-specs/SPEC_LEDGER.md`, “Next known execution move” section.

Required forcing function:

- The next remediation line that touches projection/food-source/freshness must either kill the seven mutants or record a defensible semantic-equivalence rationale for each specific mutant.
- The CI mutation reconciliation must fail if the same survivor family remains after the forcing-function deadline without an explicit maintainer decision.
- No artifact may call the canonical standing perimeter green until this family is resolved.

#### Strongest practical anti-regression guard

Add public projection/actor-known tests where two food-source facts with controlled source presence and source keys are introduced through modeled observation/record paths, then assert the resulting actor-known food-source belief and rendered/decision consequences. The tests should kill constant true/false and ordering mutants. If any mutant is genuinely equivalent, mark it with a narrow `#[mutants::skip]` or registry entry and a semantic proof; do not use “suite did not kill it” as equivalence evidence.

#### Evidence-honesty check

A closure artifact is honest if it prints the survivor list, says whether each was killed or semantically dispositioned, and keeps the overall canonical perimeter non-green until that happens. It is dishonest if it says “zero in-surface misses” in a way that the reader can mistake for “green standing perimeter.”

---

## 6. Structural anti-regression / enforced standing gate

### Why the seam reopened despite 0052

The seam reopened because 0052 improved topology but not every authority boundary became unrepresentable, and because the process still allowed enforcement claims to sit one layer above reality.

The pattern is visible in the exact code:

- A good production path exists (`LoadedFixture::into_runtime_bootstrap` → `LoadedWorldRuntime::from_bootstrap`), but a public raw bootstrap constructor still exists beside it.
- A closed runtime command enum exists, but public constructors can still mint high-authority debug/no-human commands without requiring caller-held debug authority.
- Normal render hides exact interval internals, but raw receipt products and public view-model getters still expose exact temporal fields outside the debug boundary.
- Workflow jobs exist, but branch protection does not require them before merge.
- Focused mutation evidence improved, but the canonical perimeter remains non-green and route-forward debt has no forcing function.

This is the same structural failure the fourth-pass report diagnosed: correct constructor beside injectable constructor, closed command token beside open methods, private fields mistaken for de-authority, and focused evidence over a non-green standing perimeter. citeturn600987view2turn976700view2

### What “enforced” must mean now

The standing gate must have three layers, and all three must be present:

1. **Compile-time unrepresentability on the real public symbols.** External crates cannot construct raw loaded-world bootstraps, exact embodied interval products, debug receipts, or debug/no-human commands without sealed authority. Rust visibility/private constructors are the load-bearing layer; source-text guards are only topology alarms. citeturn754265view1
2. **Behavioral witnesses through the production boundary.** The `public-boundary conformance` lane must run deterministic tests through `TuiApp::from_golden`, public runtime commands, world-step coordinator, replay frontier, TUI command loop, parity/adversarial surfaces, and negative fixtures. Architecture already requires typed path-under-test evidence, not artifact presence or display text. citeturn955256view3turn955256view4
3. **Governance enforcement.** The jobs must be required status checks on `main` through branch protection or rulesets. GitHub required status checks are the mechanism that turns a job from evidence into a merge barrier. citeturn983062view2

### Source-text guards demoted to topology alarms

`anti_regression_guards.rs`, `ci_workflow_guards.rs`, TUI source guards, grep checks, and workflow YAML assertions remain useful. They can detect deleted witnesses, renamed jobs, accidental raw fields, and obvious shortcut reintroduction. They cannot prove atomicity, replay continuation, process semantics, one-opportunity-per-actor, information-flow noninterference, production reachability, mutation sensitivity, or acceptance honesty. Those properties require compiled public-boundary negatives, behavior tests, replay/mutation evidence, and branch-protection enforcement.

### Correct placement relative to the cert ladder

Do not mint a new gate code. The mechanism belongs under existing acceptance/diagnostic evidence doctrine and the existing central diagnostic/conformance machinery. `DIAG-CERT` already consumes canonical diagnostic evidence and no phase/feature gate passes unless that diagnostic evidence passes; the new work strengthens what counts as certifying evidence and prevents shallow smoke checks. citeturn955256view0

---

## 7. Process-integrity — recurrence diagnosis and fail-closed mechanism

### Recurrence diagnosis across the acceptance corpus

The acceptance lineage shows a repeated pattern, not five isolated mistakes.

#### 0047 acceptance: feature-scope pass over a foundational seam

The original 0047 acceptance explicitly scoped itself to “Spec 0047 feature evidence only,” then recorded workspace gates, focused parity evidence, typed causal witnesses, and per-requirement pass rows. Later passes found foundational authority issues in the same seam: loaded-world derivation, replay continuation, TUI authorship, interval leakage, and non-standing mutation sensitivity. The process failure was not that 0047 ran no tests; it ran useful feature tests but allowed a feature-scope pass to look sufficient for an authority-class claim. citeturn166756view0

#### 0048 acceptance: focused closure with survivor triage

0048 recorded useful gate-to-witness mapping and a focused mutation campaign, but the focused denominator was deliberately separate from the standing perimeter; missed mutants were triaged in the artifact. Later work found production reachability and authority gaps still alive. The process failure was that focused post-remediation evidence could close tickets while the standing/production boundary remained under-proven. citeturn166756view1

#### 0050 acceptance: scoped pass while standing mutation was explicitly non-green

0050 accepted the hardening line and recorded many pass rows, while also stating that the configured standing mutation campaign had 48 missed mutants and one timeout preserved as follow-up remediation evidence, “not converted into a pass.” That wording is more honest than a flat pass, but structurally it still let a remediation epoch archive as accepted while the broad lock layer stayed red. citeturn166756view2

#### 0051 acceptance: focused campaigns zero-missed while standing remained non-green

0051 recorded clean gates and multiple focused mutation campaigns with zero selected misses, but the standing campaign still had 23 misses and was “not counted as green.” The standing survivor list included exactly the kinds of public accessor, runtime, replay, and transcript shapes that later 0052 targeted. The process failure was that focused zero-missed evidence could create closure momentum while standing survivors were routed into a later line. citeturn976700view0turn976700view1

#### 0052 acceptance: honest residuals, but still a “pass-shaped” artifact

0052 improved honesty: it recorded seven routed-forward food-source survivors, did not call the canonical perimeter fully green, and recorded branch protection as pending/fail. But the per-finding table still says “Pass,” including “Pass with honest demotion,” and the artifact’s title/status is an acceptance. This produces a human-process ambiguity: the artifact is careful in details, yet the line can still be remembered as “0052 passed,” while the enforcement residual is the exact thing that makes future recurrence possible. citeturn976700view2turn976700view3turn976700view4

### Structural causes

1. **Self-scoped acceptance.** The implementing line writes its own closeout standard and often its own acceptance artifact. That is useful for evidence collection but insufficient for independent certification.
2. **“Pass with disposition” laundering.** Open residuals are recorded accurately but still coexist with pass language. Future readers inherit the pass headline, not the caveat.
3. **Focused-vs-standing mutation goal displacement.** Focused campaigns are necessary, but they can become the target. Goodhart-style metric displacement is predictable when the measure becomes the acceptance target rather than a diagnostic input. citeturn375975search4
4. **Production-reachability witnesses that test a route, not the perimeter.** A witness can prove the good path works while the bad public path remains callable.
5. **Doc-truthing after code, not doctrine enforcement before code.** Conformance rows have repeatedly been updated to name the current implementation shape. That is valid only after executable closure; if done before closure it can ratify the shape the doctrine was meant to reject.
6. **Governance gap.** Jobs and tests can exist without being required at merge. That makes the standing gate advisory.

### Durable fail-closed acceptance / verification mechanism

This mechanism extends existing machinery. It introduces no new test framework, no new gate code, no new invariant, no new risk ID, and no glossary term.

#### 1. Machine-readable acceptance status manifest

Every future remediation acceptance artifact for this seam should include or be accompanied by a machine-readable status block, checked by existing Rust tests, with:

- exact commit under test;
- source acquisition method;
- findings/residuals enumerated by existing finding labels, not new canonical IDs;
- each finding status: `closed`, `open`, `routed-forward`, `pending-governance`, `historical-only`, or `not-in-scope`;
- certifying evidence item for each `closed` status;
- live negative / public-boundary proof for each protected shortcut;
- mutation evidence status and survivor list;
- branch-protection/ruleset enforcement status;
- overall result computed from statuses.

The computed rule is simple: **overall `pass` is legal only when every required finding is `closed`, every required governance control is enforced, and every standing mutation residual is either killed or explicitly non-blocking with a bounded forcing function.** `pending-governance`, `open`, and unbounded `routed-forward` make the overall result non-pass.

#### 2. Acceptance wording guard

Extend `crates/tracewake-core/tests/acceptance_artifact_wording.rs` or a sibling under the same existing pattern so CI rejects acceptance artifacts that:

- contain “pass with,” “scoped pass,” “accepted” or equivalent closure language while the machine-readable status block has open/pending/routed-forward items;
- call the canonical standing perimeter green while the mutation survivor list is non-empty;
- cite branch-protection enforcement without an API transcript or ruleset evidence;
- cite historical acceptance command results as current certification;
- cite display strings, artifact existence, checksums, or source guards as the sole evidence for a behavior claim that architecture requires typed path-under-test evidence for.

This is not a prose-style nit. It is the process anti-regression guard for the exact failure that repeated four times.

#### 3. Independent/adversarial acceptance posture

A remediation session may produce an evidence report, but a foundational certification claim must be re-checked by a separate adversarial pass or by CI-enforced evidence that is independent of the implementation author’s wording. The artifact may say “implementation evidence collected” or “scoped remediation evidence,” but not “foundational pass” unless the fail-closed manifest computes pass.

#### 4. Required status enforcement

The acceptance manifest must include branch-protection/ruleset enforcement as certifying evidence, not as an optional governance note. The CI job definitions should be parsed by `ci_workflow_guards.rs`, and an API/ruleset check should prove the checks are actually required on `main`. If the check cannot run because credentials are unavailable, the status is `pending-governance`, not pass.

#### 5. Routed-forward forcing function

Any routed-forward residual must name:

- owning surface;
- why it is not closed by the current line;
- the next known execution move in `docs/4-specs/SPEC_LEDGER.md`;
- a maximum number of remediation epochs or a concrete trigger after which it becomes blocking;
- the exact CI/mutation test that will fail if the residual remains.

Routed-forward is a temporary disposition, not a disposal bin.

#### 6. Evidence status taxonomy in doctrine

This is the doctrine-strengthening substance for architecture/execution. It does not weaken any upstream rule. It makes the existing evidence-honesty doctrine executable: `pass` is a certifying status; `pending`, `routed-forward`, `historical`, `sampled`, and `observer-only` are useful evidence labels but cannot be silently counted as pass. Execution-10 already says pending/historical evidence is not pass; this mechanism extends the same rule to self-scoped acceptance and governance enforcement. citeturn955256view2

---

## 8. Foundation & documentation determination

### Foundation determination

No foundation amendment is needed. The controlling foundation already forbids the bad states:

- event/replay authority cannot be bypassed by prose or raw fabrication;
- actor-known cognition must be source-bound;
- debug truth is not actor knowledge;
- possession is not privilege;
- TUI/debug surfaces must not create alternate simulation authority.

Changing Tier 0 to bless public raw bootstrap constructors, debug command self-minting, exact embodied interval access, unprotected standing gates, or self-scoped pass artifacts would be a constitutional inversion.

### Architecture / execution doctrine strengthening

The process layer does need a doctrine hardening below foundation:

| File | Substance to add after implementation | Altitude |
|---|---|---|
| `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Acceptance result exclusivity: an artifact cannot render pass while flagged foundational violations, branch-protection evidence, or standing mutation residuals remain open/pending/unbounded. Acceptance must include typed path-under-test evidence and live negatives for every protected authority claim. | Architecture acceptance contract |
| `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Fail-closed acceptance manifest, status taxonomy, wording guard, branch-protection/ruleset evidence, and routed-forward forcing-function procedure. Pending/governance-unverified is not pass. | Execution procedure |
| `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | Update current 0052 evidence once bootstrap construction is truly sealed; name the production constructor and negative proof. | Live conformance row |
| `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Update embodied/debug split evidence after interval products and debug/no-human commands are sealed by type/token boundaries. | Live conformance row |
| `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | Add bootstrap unforgeability and debug/operator command tokenization to no-direct-dispatch evidence. | Execution evidence row |
| `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | Clarify whether no-human day is ordinary play acceleration or debug/operator mode, based on implementation decision. | Execution evidence row |
| `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | Add sealed interval product and exact-debug-only evidence. | Execution evidence row |
| `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | Add reviewer pointer to fail-closed acceptance evidence and branch-protection enforcement proof. | Reference checklist |
| `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Update existing R-27/R-28/R-29 statuses only; do not mint a new risk ID. | Status/navigation |
| `docs/4-specs/SPEC_LEDGER.md` | Route the next remediation through normal ledger process and record `food_source` survivor forcing function in “Next known execution move.” | Navigation/status |

No archived spec, ticket, acceptance artifact, or passed certification should be edited.

---

## 9. Residual disposition

### Branch protection on `main`

Disposition: **first-class governance finding / merge-enforcement failure**.

The 0052 acceptance records branch protection as absent. This is not a footnote because the entire standing-gate design depends on merge enforcement. The remediation is outside Rust code but inside the repository’s acceptance process: configure branch protection/rulesets, require the relevant status checks, and make the governance proof part of acceptance. Until that proof exists, no artifact may say the standing gate is enforced. citeturn976700view4turn983062view2

### Seven `food_source_fact_supersedes` survivors

Disposition: **routed-forward cross-cutting mutation residual with forcing function**.

This pass should not claim those mutants are equivalent. cargo-mutants explicitly treats missed mutants as actionable gaps or possible indistinguishability requiring inspection; “the suite did not kill it” is not proof. The next projection/food-source hardening line must either kill or semantically justify each survivor, and the acceptance mechanism must prevent indefinite rerouting. citeturn585453view0turn976700view2

---

## 10. Recommended closure order

1. **Install fail-closed process enforcement first.** Add the acceptance status manifest, wording guard, routed-forward forcing-function rule, and branch-protection/ruleset API audit. Without this, the next code remediation can repeat the same acceptance failure.
2. **Enable branch protection/ruleset enforcement on `main`.** Require the workspace gates, `public-boundary conformance`, and the relevant mutation-lock statuses. Record API proof.
3. **Seal production bootstrap authority.** Remove the public raw bootstrap constructor or restructure crate authority so only the validated path can mint runtime bootstrap products. Add external negative fixture.
4. **Seal embodied interval / receipt products.** Move interval summary construction into core, remove public exact getters/setters from embodied products, and split exact debug data behind token-gated debug receipts.
5. **Token-gate debug/no-human runtime commands or reclassify no-human day.** Make the command’s authority match its intended surface.
6. **Resolve or force the `food_source_fact_supersedes` survivor family.** Kill mutants through public projection/actor-known tests or produce narrow semantic dispositions.
7. **Run the authoritative gates from a clean baseline.** At minimum: workspace fmt/clippy/build/test; public-boundary conformance; negative fixtures; replay/interval/salience/reservation/parity suites; focused mutation for changed files; scheduled/standing mutation according to doctrine.
8. **Truth live docs after executable closure.** Update architecture/execution/reference/spec-ledger navigation only after the code and gates exist. Do not edit archived artifacts.

---

## 11. Open maintainer decisions

These are implementation choices inside settled doctrine, not open doctrine questions:

1. **Bootstrap authority topology.** Choose whether validation/bootstrap minting moves into core, into an internal authority crate, or into another topology that makes raw runtime bootstraps unrepresentable outside the trusted path.
2. **No-human day classification.** Decide whether it is ordinary play acceleration or debug/operator execution. The code and docs must stop straddling both.
3. **Exact interval public API.** Decide which temporal fields, if any, are legitimately actor-known through modeled sources. Everything else belongs to debug/operator products only.
4. **Food-source semantics.** Decide the intended ordering/replacement rule for source-bearing vs. source-less food-source facts and source-key tie-breaking, then encode that rule in public behavior tests or semantic mutant dispositions.
5. **Governance owner.** Assign the maintainer or admin responsible for branch protection/ruleset configuration and API proof capture.

---

## 12. Self-check

- [x] Verdict covers code conformance, process soundness, and higher-tier amendment determination.
- [x] Target-repository file evidence used for repository claims was fetched from full exact commit URLs at `e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f`.
- [x] Manifest was used only as path inventory; no branch fetch, code search, clone, default-branch lookup, or repository metadata was used for target-state claims.
- [x] 0052 “pass” rows were re-checked against exact-commit source shape and not inherited as proof.
- [x] Present properties are recorded as present rather than re-reported as absent.
- [x] Each code finding names controlling doctrine, current code state, remediation home, anti-regression guard, and evidence-honesty check.
- [x] Branch protection and `food_source_fact_supersedes` residuals are dispositioned explicitly.
- [x] Process-integrity is co-equal and includes a fail-closed mechanism.
- [x] Recommendations extend existing machinery; no new property-testing framework is recommended.
- [x] No archived artifact is edited; no invariant, gate code, risk ID, or glossary term is minted.
- [x] Static-survey limits are explicit; no current gate is asserted green or red.
- [x] Repository and external evidence lanes are kept separate.

---

## 13. References

### Repository evidence lane

Repository evidence is the exact URL ledger in §3. The most load-bearing fetched files were:

- `docs/README.md` — authority order.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` and `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — constitutional and truth-firewall drivers.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`, `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`, and `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — world-step, TUI/debug, and acceptance contracts.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`, `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`, and `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — execution gates and evidence honesty.
- `archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md` — historical 0052 claims and residuals, re-verified rather than inherited.
- `crates/tracewake-core/src/runtime/session.rs`, `runtime/command.rs`, `runtime/receipt.rs`, `scheduler.rs`, `state.rs`, `projections.rs`, `view_models.rs`, `crates/tracewake-content/src/load.rs`, and `crates/tracewake-tui/src/app.rs` / `run.rs` / `render.rs` — code-surface evidence.
- `.github/workflows/ci.yml`, `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, and the negative/public-boundary test suites — standing-gate evidence.

### External research lane

- Rust Reference, “Visibility and privacy” — public/private item accessibility and crate/module visibility. citeturn754265view1
- Rust Reference, “The `non_exhaustive` attribute” — `non_exhaustive` is not an authority-sealing substitute. citeturn754265view4
- GitHub Docs, “About protected branches” — required status checks and branch protection semantics. citeturn983062view2
- cargo-mutants, “Using the results” — missed mutants and timeouts as actionable evidence; public-level tests preferred. citeturn585453view0
- Temporal documentation, “How Workflow replay works” — event history as source of truth and deterministic replay from recorded history. citeturn314344view0
- Schneider, “Implementing Fault-Tolerant Services Using the State Machine Approach” — state-machine approach as log/order/state determinism prior art. citeturn342082search0
- Sabelfeld & Myers, “Language-Based Information-Flow Security” — access control alone does not enforce information-flow policies. citeturn342082search2
- Goodhart/Strathern formulation — a metric that becomes the target ceases to be a good measure; relevant to focused evidence becoming closure theater. citeturn375975search4
