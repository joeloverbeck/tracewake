# TUI/simulation feature-parity drift guard — research report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `1145e109aa50721b37f7d343caef518b3be5fc7f`  
**Freshness boundary:** user-supplied target commit only; this report does not independently establish that the commit is the current `main`  
**Deliverable posture:** recommendation report at substance-and-home altitude; not ratified doctrine and not an implementation spec

## 1. Executive summary

Tracewake already says that the kernel and playable TUI must advance together, but the current implementation does not make that doctrine mechanically complete: `EmbodiedViewModel` can gain a field that `render_embodied_view` silently ignores, and a simulation capability can fail to enter `build_embodied_view_model` at all without leaving any type-level trace. Those are distinct failures: **Hop 2** is view model → rendered surface; **Hop 1** is simulation capability → actor-filtered view model.

The headline recommendation is a **layered standing playable-capability parity contract**:

1. make Hop 2 compiler-enforced by exhaustively destructuring `EmbodiedViewModel` with no `..`, placing a local unused-binding deny at the presentation boundary, and using exhaustive no-wildcard matches wherever closed variants determine presentation;
2. make Hop 1 behaviorally enforced by focused, checked-in golden scenarios that run through the real fixture → simulation → actor-known projection → TUI render/action/re-render pipeline, with typed witnesses before rendered witnesses and paired anti-leak cases for epistemic features; and
3. bind both to a test-side capability registry and conformance runner so every declared playable capability—including every registered semantic action—has an explicit surface disposition and adequate fixtures before a future feature spec may close.

The enforcement posture should be **layered and per-feature**, not a new top-level certification rung and not an amendment confined to the already-passed `EPI-CERT` or `FIRST-PROOF-CERT` packages. Architecture should own the permanent contract; execution should own the compiler, fixture, snapshot, CI, and evidence mechanics; future specs should be required to declare parity impact and carry the resulting evidence. Existing certifications remain historical baseline evidence rather than the standing home of the obligation.

## 2. Disposition table

| Finding | Target home | Verdict | Basis |
|---|---|---:|---|
| Existing foundation doctrine already requires runnable features to reach an actor-filtered TUI, with why-not, replay, no-human, and debug separation | `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`; `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`; `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`; constitutional invariants | **no-change** | The principle is already constitutional; the defect is missing structural enforcement, not missing foundational intent. |
| The architecture does not state a two-hop completeness contract for playable capabilities | `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | **amend** | This document owns view-model shape, possession, rendering boundaries, and the TUI harness. |
| Acceptance architecture does not bind a declared capability set to typed and rendered evidence | `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | **amend** | This is the highest legitimate home for capability-to-evidence closure and review artifacts. |
| The architecture conformance index does not register parity completeness as a standing conformance obligation | `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | **amend** | Cross-cutting architecture obligations must be discoverable from the conformance index. |
| `render_embodied_view` reads fields à la carte, so a new view-model field can be silently ignored | code/test; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | **add / amend** | An exhaustive destructure turns schema growth into a compiler failure; execution owns the precise test and CI mechanic. |
| Closed presentation variants are not governed by one explicit no-wildcard rule; meanwhile `SemanticActionEntry` is data-driven, not an action-kind enum | code/test; execution `10` | **add / amend** | Closed enums need exhaustive presentation matches; action IDs need registry coverage rather than a fictitious enum match. |
| Hop 1 cannot be proven by a field guard and currently has no complete feature-coverage suite | `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`; `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`; `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | **amend** | Actor-known proof, fixture design, and test mechanics are split across these execution owners. |
| The existing transcript “snapshot” proves repeatability, not agreement with a checked-in accepted rendering | code/test; execution `10` | **add / clarify** | Comparing two outputs from the same implementation cannot detect a stable omission. |
| No registry makes “declared core capability ⇒ surface disposition + fixture + rendered witness” mechanically checkable | architecture `13`; execution `09` and `10`; future implementation spec | **add** | The registry is test/conformance metadata, not simulation state, and must be tied to generic validation. |
| A new top-level parity gate would duplicate the ladder, while folding parity only into passed certs would not bind expansion work | execution `00` and `03`; `docs/4-specs/README.md` | **amend** | The correct posture is a standing obligation inside every future feature acceptance package and the ordinary CI lane. |
| Acceptance artifacts have no dedicated place to report capability coverage and two-hop witnesses | `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | **amend** | The template should carry the conformance report, typed witness, rendered golden, anti-leak witness, and command evidence. |
| The reference tier recognizes TUI-first erosion but does not name the two-hop relapse pattern or ask the decisive review question | `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`; existing `R-15` in `01_DESIGN_RISK_REGISTER.md`; optionally `02_GLOSSARY.md` | **amend / record / route-forward** | The checklist and existing risk entry are the correct reference homes; no new risk identifier is needed. |
| Domain packs need composable coverage later, but building pack-specific parity now would exceed scope and risk genre coupling | future implementation/spec route; architecture contract remains composable | **route-forward** | Namespaced capability manifests can compose per active pack without putting flavor knowledge in core. |
| The live spec ledger and current ontology fixture contract should not be rewritten before a future implementation spec is declared | `docs/4-specs/SPEC_LEDGER.md`; `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` | **no-change now** | The ledger records declared work; this report recommends, but does not declare, a numbered spec. |

## 3. Method & provenance ledger

### 3.1 Repository acquisition discipline

The uploaded manifest was used only as a path inventory for the supplied commit. The research brief was used as scope and authority-order control material. Repository state claims below rely only on manifest paths fetched through verified full exact-commit URLs.

```text
Requested repository: joeloverbeck/tracewake
Target commit: 1145e109aa50721b37f7d343caef518b3be5fc7f
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open(full exact raw.githubusercontent.com or exact GitHub blob URL)
Requested file count: 87
Successfully verified file count: 87
Exact URL request count: 91
Successfully verified exact URL request count: 91
Fetched repository files: 91 exact URLs listed in §3.5 (87 unique manifest paths)
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

For every repository file used, the path appeared exactly in the uploaded manifest; the requested and returned source retained owner `joeloverbeck`, repository `tracewake`, full commit `1145e109aa50721b37f7d343caef518b3be5fc7f`, and the exact manifest path; and the result was the requested file rather than a search result, branch page, overview, or substitute. Four already-acquired code/test files were also opened through exact GitHub blob URLs to obtain a second line-oriented view; these are counted as additional URL requests but not additional unique files.

The manifest contains no `reports/tui-parity-research-report.md`, so this deliverable is new at the target commit. The untracked seed note was not treated as repository evidence; it was read only from Appendix A of the user-supplied brief.

### 3.2 Repository reading scope

The acquisition and reading set included:

- `docs/README.md`;
- every file in `docs/0-foundation/`, `docs/1-architecture/`, `docs/2-execution/`, `docs/3-reference/`, and `docs/4-specs/`;
- archived specs `0009`, `0044`, and `0045` for structural precedent only;
- the view-model, projection, action-registry, app, rendering, debug, transcript, and controller seams;
- all five TUI test files named in the brief plus adjacent app/transcript tests;
- representative `GoldenFixture` definitions and the fixture registry;
- workspace/TUI manifests, CI, and lint configuration.

The complete exact URL ledger appears in §3.5.

### 3.3 External research lane

External research was deliberately separate from target-repository evidence. Searches covered:

- Rust struct patterns, rest patterns, exhaustiveness errors, unused-variable lints, and `#[non_exhaustive]` downstream semantics;
- the programming-languages literature on detecting non-exhaustive pattern matches;
- MVU/Elm’s model → view architecture;
- Rust, Jest, Flutter, and Android golden/snapshot practice;
- user-path-oriented UI testing and consumer-driven contract-by-example testing;
- declared-capability conformance systems, especially Kubernetes Gateway API `supportedFeatures` and conformance profiles;
- compile-fail/UI test harnesses in the Rust ecosystem;
- modular game-feature and mod/data lifecycle systems for later pack composition.

Recommendation-shaping claims were checked against primary or project-maintained sources: the Rust Reference and rustc documentation, the original Maranget paper, official project documentation for Elm, Insta, Jest, Pact, Gateway API, Unreal Engine, Factorio, Testing Library, and trybuild/Rust compiler UI tests. External sources were not used to claim what Tracewake contains.

### 3.4 Repository citation convention

Repository findings are cited as `path::symbol` or `path` plus the relevant section. These are exact-commit files from the ledger. External claims use numbered footnotes and are collected in §12.

### 3.5 Complete append-only repository URL ledger

- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/archive/specs/0009_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/archive/specs/0044_FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/archive/specs/0045_FIRST_PROOF_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/tests/tui_seam_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/tests/tui_acceptance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/tests/adversarial_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/tests/transcript_snapshot.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/fixtures/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/fixtures/view_model_local_actions_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/fixtures/no_human_day_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/fixtures/embodied_workplace_believed_open_truth_closed_commit_fails_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/fixtures/possession_parity_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/fixtures/embodied_menu_lags_truth_change_without_perception_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/fixtures/embodied_view_omits_unobserved_food_at_open_place_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-content/src/manifest.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-core/src/actions/registry.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-core/src/actions/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-core/src/agent/no_human_surface.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-core/src/controller.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/src/run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/1145e109aa50721b37f7d343caef518b3be5fc7f/clippy.toml
- https://github.com/joeloverbeck/tracewake/blob/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-core/src/projections.rs
- https://github.com/joeloverbeck/tracewake/blob/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/src/render.rs
- https://github.com/joeloverbeck/tracewake/blob/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/tests/adversarial_gates.rs
- https://github.com/joeloverbeck/tracewake/blob/1145e109aa50721b37f7d343caef518b3be5fc7f/crates/tracewake-tui/tests/tui_acceptance.rs

## 4. Diagnosis: current state

### 4.1 Doctrine is ahead of enforcement

The authority stack is unambiguous. `docs/README.md` establishes the downward authority order. The constitutional invariants make the TUI the primary product surface, require a TUI/view-model gate in every runnable phase, prohibit the TUI from becoming an alternate rules engine, require actor-known filtering and why-not feedback, quarantine omniscient diagnostics, preserve no-human life, and keep core genre-agnostic (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`, especially the existing TUI, epistemic, no-human, truth-firewall, and domain-pack invariants).

Foundation `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` already says that kernel and TUI advance together. Its per-feature review rule asks whether an ordinary actor can use or experience the feature, whether a possessed human can do so through the same actor path, whether the TUI shows known facts and hides unknown facts, whether failure is explained, whether debug remains separate, and whether automated tests prove the behavior. Foundation `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` treats TUI/view-model reachability, actor filtering, replay, regression coverage, and no-human continuation as part of “done.” Foundation `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` forbids using hidden truth as a decision input or an embodied proof shortcut.

The reference tier already records `R-15 — TUI-First Playability Erosion` in `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: a feature accepted while unreachable or uninspectable from the playable surface is a relapse. The defect therefore is not that Tracewake lacks a principle. The defect is that the principle is currently enforced by remembered review and hand-selected tests rather than by a complete structural contract.

### 4.2 The two-hop seam

The current path is clean and testable:

1. simulation and epistemic state are projected by `crates/tracewake-core/src/projections.rs::build_embodied_view_model` (with notebook projection in `build_notebook_view`);
2. `crates/tracewake-tui/src/app.rs::TuiApp::current_view` constructs the sealed embodied context and calls the core projection;
3. `crates/tracewake-tui/src/app.rs::TuiApp::render_current_view` calls the pure renderer;
4. `crates/tracewake-tui/src/render.rs::render_embodied_view` returns a `String` without requiring a terminal; and
5. semantic submissions go back through `crates/tracewake-tui/src/app.rs::TuiApp::submit_semantic_action` and the normal core proposal/validation/event pipeline.

This is an excellent seam for parity enforcement. It also exposes two independent blind spots.

### 4.3 Hop 2 exposure: view model → rendered text

`crates/tracewake-core/src/view_models.rs::EmbodiedViewModel` has 22 public fields at the target commit. They include embodied place and entity views, semantic actions, Phase-3A status, rejection/why-not data, holder-known context metadata, notebook data, and debug availability. It is a closed workspace struct and is not marked `#[non_exhaustive]`.

`crates/tracewake-tui/src/render.rs::render_embodied_view` reads the borrowed model field by field. It does not first destructure `EmbodiedViewModel` exhaustively. Adding a field to the struct therefore does not require any change in the renderer; the TUI crate can compile while the new field is completely absent from the playable text. Existing unit tests such as `renderer_prints_semantic_action_ids` and `renderer_prints_door_endpoints_and_item_sources` verify selected current behavior, but a new field creates no failing test unless an author remembers to add one.

Some current fields are legitimately not part of the main embodied text—for example, internal view identity, holder-known context diagnostics, or notebook material rendered through a separate surface. That does not weaken the diagnosis. The structural requirement is not “print every field.” It is “force an explicit, reviewable presentation disposition for every field.” An exhaustive destructure provides that decision point.

Closed enums already show why this works. `crates/tracewake-tui/src/render.rs::visible_item_source_label` matches `VisibleItemSource` exhaustively without a wildcard. A new source variant would force a compiler error until its presentation is decided. The same rule is not yet stated as a standing contract for every closed type that controls actor-facing or debug-facing presentation.

One correction to the seed’s shorthand matters: `crates/tracewake-core/src/view_models.rs::SemanticActionEntry` is a struct, not an enum of all action kinds. The current action universe is data-driven through `crates/tracewake-core/src/actions/registry.rs::ActionRegistry`, `ActionDefinition`, `ActionEffect`, and action IDs. Exhaustive enum matching can protect `ActionEffect` or `ActionAvailability` presentation semantics, but it cannot enumerate all future semantic action IDs. Action completeness belongs in the capability registry/conformance layer described below.

### 4.4 Why `#[non_exhaustive]` is the wrong tool here

Rust’s `#[non_exhaustive]` is designed to let a defining crate add fields or variants without breaking downstream crates. Outside the defining crate, a downstream struct pattern is required to include `..`, and a downstream enum match requires a wildcard.[^2] That is the opposite of this task. `tracewake-core` and `tracewake-tui` are separate crates in one controlled workspace; deliberate breakage at their contract boundary is desirable. Marking `EmbodiedViewModel` non-exhaustive would disable the exact compiler pressure Tracewake needs.

### 4.5 Hop 1 exposure: simulation capability → actor-filtered view model

`crates/tracewake-core/src/projections.rs::build_embodied_view_model` explicitly gathers actor-known routes, doors, containers, items, local actors, semantic actions, Phase-3A status, rejections, holder-known context data, and notebook information before constructing the model. If a future capability—institutions, notices, travel, regional effects, richer autonomous behavior, or a new ordinary-life mechanic—never enters this projection path, there is no missing field for Rust to diagnose. The model is simply blind.

The truth firewall makes this more subtle than ordinary UI coverage. A correct parity test may not inspect raw world truth and then demand that the embodied TUI display it. It must create or replay the events and observations that make a fact available to the possessed actor, then assert what the actor-filtered projection and renderer expose. When the feature is private cognition or autonomous planning, the embodied witness may be a perceivable consequence rather than the private thought itself; a separate debug witness may prove the internal process, but debug truth cannot satisfy the embodied obligation.

No type-system technique can infer that arbitrary new simulation behavior “should be visible.” Hop 1 therefore needs declared semantic scope plus executable examples.

### 4.6 What existing tests already guard

The existing suite is strong on several adjacent properties and should be retained:

- `crates/tracewake-tui/tests/tui_seam_conformance.rs` protects the debug/embodied seam, stale selection behavior, no direct event application, debug construction through core, and deterministic transcript behavior.
- `crates/tracewake-tui/tests/embodied_flow.rs` runs real fixture-backed possession flows, rendering, semantic action submission, re-rendering, why-not behavior, Phase-3A status, actor-known workplace availability, and hidden-truth omission.
- `crates/tracewake-tui/tests/adversarial_gates.rs` attacks debug leakage, stale selection, debug command isolation, mutation during rendering, and ordering/replay hazards.
- `crates/tracewake-tui/tests/tui_acceptance.rs` verifies no direct dispatch, typed artifacts before display evidence, positive first-proof playability, rejection/wait/debug paths, hidden-debug exclusion, and deterministic transcripts.
- `crates/tracewake-tui/src/app.rs` contains source-conformance guards ensuring public render functions have production callers and debug paths do not become alternate world-advancing routes.
- `crates/tracewake-content/src/fixtures/mod.rs::all` and `by_id` expose a large Rust-defined `GoldenFixture` substrate, including actor-known, no-human, ordinary-life, and belief/truth scenarios.
- `.github/workflows/ci.yml` runs workspace build/test/clippy and named lock-layer suites with warnings denied.

These tests prove important safety and current behavior. They do **not** prove completeness under feature growth.

### 4.7 What existing tests do not guard

No current test fails merely because:

- `EmbodiedViewModel` gains a field that the renderer never reads;
- a presentation-relevant closed enum gains a variant that is swallowed by a wildcard or generic helper;
- `ActionRegistry::definitions` gains an action ID that has no actor-known TUI scenario;
- a new non-action capability exists in core but is absent from `build_embodied_view_model`;
- a fixture exercises new behavior but is not designated as a parity witness; or
- a stable rendering omits a feature consistently.

The last point is particularly important. `crates/tracewake-tui/tests/transcript_snapshot.rs` generates a transcript twice and compares the byte strings to each other. That is a determinism test. It is not an approval/golden test against a checked-in accepted transcript. A renderer that omits institutions on both runs passes.

The current state therefore matches the motivating fear exactly: the seam is architecturally clean, but neither hop has a standing completeness mechanism.

## 5. External research synthesis

### 5.1 Exhaustiveness is a closed-world change detector, not semantic coverage

Rust struct patterns can bind each named field, while `..` explicitly ignores all remaining fields.[^1] Rust also rejects non-exhaustive enum matches (`E0004`) unless every case or a catch-all is supplied.[^3] The underlying programming-languages literature treats missing and useless pattern cases as statically detectable anomalies; Maranget’s classic treatment formalizes the compiler-side value of exhaustiveness checks.[^5]

**Implication for Tracewake:** use the compiler where the world is closed and represented in a type. An exhaustive `EmbodiedViewModel` destructure and no-wildcard matches are ideal Hop-2 tripwires. They cannot detect Hop 1 because an absent projection has no constructor, field, or variant to match.

### 5.2 Warnings can force a second decision after the schema decision

Rust’s `unused_variables` lint is warn-by-default, and an underscore-prefixed binding explicitly suppresses it.[^4] Tracewake’s CI already promotes warnings to errors. An exhaustive destructure therefore has two useful failure stages:

1. adding a field causes an incomplete-pattern compile error until the renderer names it;
2. naming it without using it causes an unused-variable failure until the renderer either uses it or marks it intentionally omitted.

**Implication:** add a local deny at the renderer boundary so the tripwire is not dependent only on CI configuration, and permit underscore-prefixed omissions only with an adjacent rationale. A source-conformance test should reject a rest pattern and bare wildcard field dispositions, preserving the tripwire itself.

### 5.3 `#[non_exhaustive]` solves library compatibility, which is the wrong objective

The Rust Reference requires downstream patterns over a `#[non_exhaustive]` struct to include `..` and downstream matches over a non-exhaustive enum to include a wildcard.[^2]

**Implication:** do not add `#[non_exhaustive]` to the internal core→TUI view contract. Tracewake wants a breaking signal when the core surface grows. Semver-style downstream tolerance would make silent drift easier.

### 5.4 MVU makes the seam testable but does not prove that every model fact is rendered

The Elm Architecture defines a model, a view that turns state into output, and an update path.[^6] Tracewake already has the valuable part of this shape: a pure `render_embodied_view(&EmbodiedViewModel) -> String`. However, a pure view can still ignore record fields, and a projection can still omit domain behavior.

**Implication:** architectural unidirectionality and purity are prerequisites for good parity tests, not the parity guarantee itself. Tracewake should keep the current seam and add completeness pressure rather than redesign the TUI around another framework.

### 5.5 Golden tests need approved references and review discipline

Insta describes snapshot tests as comparisons against stored reference values and provides an explicit review workflow.[^7] Jest’s guidance is to commit snapshots, review them as code, and keep them focused and readable.[^8] UI testing guidance likewise emphasizes tests that resemble actual software use.[^9]

**Implication:** add checked-in expected renderings generated through the real TUI path. Prefer focused per-capability snapshots over a single giant kitchen-sink baseline; a broad smoke transcript may supplement them. Typed assertions must precede rendered assertions so a coincidental string cannot masquerade as correct causality or actor knowledge.

The external tooling does not require Tracewake to adopt `insta`. The repository already favors explicit byte-stable assertions and command evidence. A small checked-in text-golden helper may fit better. `insta` is an optional ergonomics choice, not a design requirement.

### 5.6 Contract-by-example is strong at a seam but only for declared interactions

Pact describes consumer-driven contracts as executable examples and explicitly notes that only interactions used by consumers are tested.[^10]

**Implication:** treating the TUI as a consumer of actor-filtered core projections is useful, but examples alone do not guarantee completeness. A capability declaration/registry is needed to define what interactions must have examples. This is the same boundary between Hop 1’s semantic declaration and its executable proof.

### 5.7 Declared capabilities can drive the correct conformance suite

Kubernetes Gateway API standardized supported feature names and designed its conformance tooling to select tests from the features an implementation declares.[^11] Its conformance profiles compose groups of supported features and produce reports.[^12]

**Implication:** Tracewake should maintain a test-side declared capability set whose entries select and parameterize the fixtures and witnesses that must pass. The registry should not claim to discover arbitrary semantics. It should make the accepted feature declaration executable and make missing evidence an error.

This is the strongest external analogue for later domain packs: a base profile plus composable pack profiles, each with declared features and the same generic conformance runner.

### 5.8 Compile-fail harnesses are available, but the real renderer should remain the proof target

`trybuild` and rustc’s own UI tests demonstrate a mature pattern for asserting that selected programs fail to compile and for reviewing expected diagnostics.[^13][^14]

**Implication:** a controlled compile-break witness can be useful in the future implementation acceptance packet, but adding a mirror type or a synthetic-only compile test would not prove the actual renderer remains exhaustive. The persistent guard should be the real destructure plus a source-conformance test that prevents its removal. Tracewake need not add a new test dependency merely to re-prove Rust’s pattern semantics.

### 5.9 Modular feature systems support namespaced composition, not automatic playability

Unreal’s Game Features system packages standalone features and data registries to reduce accidental dependencies as feature sets evolve.[^15] Factorio’s mod system similarly has an explicit data lifecycle and structured discovery rules for active mods.[^16]

**Implication:** future flavor/domain packs should contribute namespaced capability declarations and fixtures rather than adding genre switches to core. Neither example proves UI reachability automatically; the Tracewake-specific contribution is to require each active pack’s declared playable capabilities to pass the same actor-filtered surface conformance suite.

## 6. Mechanism design

### 6.1 Design objective and honest boundary

The strongest achievable guarantee is:

> No declared playable capability can be accepted without an explicit actor-facing surface disposition and passing real-pipeline evidence; no growth of the closed core→TUI presentation contract can compile without a conscious renderer decision.

It is not possible to statically infer every semantic “feature” from arbitrary Rust behavior. A developer could write a new subsystem and intentionally omit it from the declared capability set. Tracewake closes that remaining gap through the feature-spec rule: every future spec must declare its capability impact, and acceptance tooling must reject a declared capability without evidence. This is structural within the repository’s authority and review system rather than magical program understanding.

### 6.2 Guard A — Hop-2 exhaustive view-model handling

#### Required mechanic

At the start of `crates/tracewake-tui/src/render.rs::render_embodied_view`, destructure `EmbodiedViewModel` by naming every field and do not use `..`. Place `unused_variables` at deny level for the renderer or its module, independent of workspace CI. Render from the resulting bindings rather than returning to scattered `view.field` reads.

Every field must receive one of two dispositions:

- **rendered/consumed** by the embodied output or a delegated presentation helper; or
- **intentionally not rendered in this surface**, represented by an underscore-prefixed named binding and an adjacent reason that identifies the correct alternate owner or explains why the value is internal metadata.

Do not permit a bare `_`, `field: _`, or a rest pattern for omission. Those forms erase the name or allow future fields to arrive silently.

Add a source-conformance test in the existing TUI seam/conformance style that verifies the real renderer contains the exhaustive destructure and that the pattern contains no `..`. This test protects against a later “cleanup” that removes the tripwire while preserving output.

#### Closed enum rule

Where a closed enum variant selects actor-facing or debug-facing presentation, the presentation owner must use an exhaustive match with no `_` arm. Preserve the already-exhaustive `VisibleItemSource` handling and extend the rule to presentation decisions over types such as `ActionAvailability`, `ActionAvailabilityProvenanceKind`, `WhyNotFailureKind`, `DebugViewModel`, and `ActionEffect` when those types are matched at a rendering boundary.

The rule applies at the **semantic presentation owner**, not necessarily in one monolithic function. If core owns an actor-safe textual summary, core’s match must be exhaustive and the TUI may call it. The TUI must not duplicate simulation rules merely to obtain an exhaustive match.

`SemanticActionEntry` itself is not a closed action-kind enum. Its action-ID completeness is covered by Guard C.

#### What it catches

- a new `EmbodiedViewModel` field;
- a newly introduced closed presentation variant;
- a field named in the destructure but accidentally unused;
- removal or weakening of the real exhaustive pattern, through the source-conformance test.

#### What it cannot catch

- a value rendered incorrectly;
- a deliberately underscore-bound field with a poor rationale;
- a core capability that never enters the view model;
- a data-driven action ID absent from the semantic-action projection.

#### Failure mode

The build fails at the core→TUI boundary, or the conformance source guard fails, before feature acceptance. The required repair is a visible code decision, not a snapshot update alone.

#### Composition with existing tests

This adds a structural layer beneath the current renderer unit tests. Existing tests continue to prove formatting and selected semantics. The new guard makes it impossible for schema growth to bypass those review points silently.

### 6.3 Guard B — Hop-1 real-pipeline capability goldens

#### Required test shape

For each playable capability, define at least one focused positive scenario that runs through the real system:

1. obtain a manifest-backed `GoldenFixture` from `crates/tracewake-content/src/fixtures`;
2. load the world through the ordinary content path;
3. possess/bind the designated actor through `TuiApp`;
4. advance or replay the simulation through the normal scheduler/event path as required;
5. build `TuiApp::current_view` and call `render_current_view`;
6. when the capability is actionable, select and submit a real `SemanticActionEntry` through `TuiApp::submit_semantic_action`;
7. re-render and, where the feature’s doctrine requires it, rebuild/replay and compare the actor-filtered result.

The test must assert in this order:

- **typed causal witness:** expected event, proposal, validation report, projection record, view-model field, or action entry;
- **actor-knowledge witness:** the fact is available through the embodied context, with source/freshness where relevant;
- **rendered witness:** checked-in expected text or a focused accepted rendering;
- **negative/anti-leak witness:** for epistemic or hidden-state features, an unknown, stale, contradictory, or unobserved case remains absent or disabled with an actor-safe why-not;
- **debug witness, if required:** separate evidence that debug can diagnose the underlying mechanism without satisfying the embodied assertion.

#### Focused goldens, not one sole kitchen sink

A single kitchen-sink fixture is attractive but fragile: unrelated changes create large diffs, reviewers normalize noise, and one scenario cannot cover mutually exclusive epistemic states. Use a matrix of focused capability scenarios, each with a short readable expected rendering. One broad integrated transcript may remain as a smoke test, but it must not be the only completeness evidence.

The current `transcript_snapshot.rs` determinism check should remain. Add separate checked-in references; do not reinterpret run-A-equals-run-B as acceptance of content.

#### Autonomous-agent features

For “non-human players are thinking and acting”:

- the no-human/core witness proves decision transactions, selected proposals, and resulting events;
- the embodied witness proves what a possessed actor can legitimately observe: an NPC present, moving, working, changing the environment, communicating, leaving a record, or otherwise producing actor-available consequences;
- the debug witness may show planner traces or internal blockers, but only behind the sealed debug capability.

A parity test must not surface private intentions or hidden world truth merely to make autonomy visible. The correct product behavior may be uncertainty plus observable consequence.

#### What it catches

- projection blindness in `build_embodied_view_model`;
- app glue that fails to supply or refresh an actor-known source;
- action definitions not appearing as semantic entries under the right conditions;
- actor-known facts lost between projection and render;
- hidden truth accidentally leaked to make a feature “visible”;
- stable but incomplete output when compared with an accepted reference.

#### What it cannot catch

- a capability never declared for coverage;
- every possible interaction among capabilities;
- presentation quality beyond the accepted scenario;
- a weak or dishonest witness that reviewers accept without scrutiny.

#### Failure mode

The typed assertion, anti-leak assertion, or checked-in rendering differs. The evidence identifies whether the break is causal/projection, epistemic filtering, action wiring, or presentation.

#### Composition with existing tests

This reuses `GoldenFixture`, `TuiApp`, the pure renderer, existing replay machinery, and current actor-known/adversarial tests. It does not replace unit, mutation, no-human, or truth-firewall suites. It adds the missing feature-completeness dimension.

### 6.4 Guard C — playable-capability registry and conformance runner

#### Role and placement

Create a **test-side** capability registry in a downstream crate or integration-test support module that can depend on both `tracewake-core` and `tracewake-content`. It must not be authoritative simulation state, must not be consumed by cognition, scheduling, validation, or event application, and must not make core depend on content or TUI.

The name “playable-capability parity contract” is a working report term, not a ratified glossary term. The owner may choose a final term during reassessment.

#### Minimum entry shape

Each entry should declare:

- a stable capability key local to the conformance system;
- ownership scope: base simulation now, or a namespaced future domain pack;
- capability class: semantic action, actor-observable state, actor-observable consequence, notebook/record surface, debug-only infrastructure, or headless infrastructure;
- the required surface disposition and why it is legitimate;
- one or more fixture IDs;
- the possessed viewer actor and setup/advance operation;
- the typed witness to inspect;
- the rendered witness or checked-in golden;
- required negative/anti-leak fixtures for epistemic capabilities;
- replay and no-human evidence flags where doctrine makes them applicable.

“Debug-only” and “headless infrastructure” must be explicit dispositions, not escape hatches. A feature intended for ordinary play cannot be classified debug-only merely because no embodied design was implemented.

#### Generic conformance checks

The runner should fail when:

- capability keys are duplicated, empty, or non-deterministically ordered;
- an entry has no fixture, typed witness, or required rendered witness;
- a referenced fixture ID is absent from `crates/tracewake-content/src/fixtures/mod.rs::by_id`;
- an epistemic capability lacks a negative/anti-leak witness;
- an actionable capability cannot find the declared action in the actor’s `semantic_actions` under its positive scenario;
- a registered action from `crates/tracewake-core/src/actions/registry.rs::ActionRegistry::definitions` has no capability disposition and fixture coverage;
- a capability is marked embodied but the expected actor-filtered render is empty or unchanged at the declared observation point;
- a capability is declared debug-only but its feature spec or architecture classification says it is ordinary playable behavior;
- a future active pack has no corresponding capability profile or contains uncovered entries.

The runner should emit a deterministic coverage report suitable for the acceptance artifact: capability count, class, fixture IDs, typed witness, rendered witness, negative witness, replay/no-human status, and pass/fail.

#### Avoiding a registry that lies

A registry can become another list someone forgets to update. Three controls reduce that risk:

1. **Spec obligation:** every future feature spec must enumerate added, changed, and deliberately unaffected capabilities. “No parity impact” is an explicit reviewed claim, not an omitted section.
2. **Enumerated seam checks:** all current `ActionRegistry::definitions` must map to coverage; all registry fixture IDs must resolve; all entries must execute.
3. **Acceptance artifact:** the generated coverage report is attached to the feature’s evidence package and reviewed alongside code and snapshots.

For non-action semantics, no reliable automatic code enumeration exists. The spec declaration is the authoritative semantic inventory; the runner makes that inventory non-vacuous.

#### What it catches

- declared capabilities with no fixture or surface proof;
- new registered actions with no TUI coverage;
- stale fixture references and empty evidence;
- per-pack coverage holes once packs exist;
- “tests exist somewhere” claims that do not map to a capability.

#### What it cannot catch

- a developer who falsely declares that a genuine new capability has no parity impact;
- semantic interactions not represented by the selected scenarios;
- an incorrect architectural classification accepted by reviewers.

Those are governance/review failures, which is why the registry must be bound into the spec and evidence process rather than treated as a standalone test file.

### 6.5 Guard D — standing feature-spec and acceptance closure

Every future feature spec should contain a parity-impact section that:

- lists each added or changed playable capability;
- states its surface disposition;
- identifies the Hop-2 schema/variant changes, if any;
- names its positive, negative, replay, and no-human witnesses;
- identifies whether embodied, notebook, and debug surfaces change;
- states “no parity impact” only with a reason tied to architecture;
- includes the conformance report in its acceptance artifact.

A feature cannot be accepted merely because core tests pass. If the capability is playable, parity evidence is part of the feature’s own definition of done. If it is intentionally headless infrastructure, the spec must establish why no ordinary actor-facing surface is owed.

### 6.6 Domain-pack forward compatibility

Do not build pack-specific parity now. Design the registry so later packs contribute namespaced profiles—for example, a base profile plus one profile for an active fantasy pack or post-apocalyptic pack. The generic runner composes active profiles and requires each declared pack capability to carry the same typed, actor-filtered, rendered, and anti-leak evidence.

This preserves the existing constitutional split:

- core remains genre-agnostic;
- packs own flavor, fixtures, labels, and content;
- the TUI consumes the same typed actor-known contracts;
- activating a pack cannot silently activate capabilities with no playable proof.

A pack may specialize presentation text or add pack-owned surfaces, but it may not bypass the base event/projection/action pipeline or grant the TUI hidden truth.

### 6.7 Guard summary

| Guard | Catches | Does not catch | Failure mode | Existing-test composition |
|---|---|---|---|---|
| Exhaustive VM destructure + local lint deny | New fields; unused field bindings | Missing projection; wrong rendering | Compile or source-conformance failure | Sits beneath renderer unit tests |
| Exhaustive closed-enum presentation matches | New presentation variants | Data-driven action IDs; absent feature | `E0004`/build failure | Preserves current helper tests |
| Focused real-pipeline typed + golden scenarios | Projection/wiring/filtering/render omissions | Undeclared semantic capability | Typed assertion, anti-leak, or golden diff | Reuses `GoldenFixture`, `TuiApp`, replay, adversarial suites |
| Capability registry/conformance runner | Declared capability without evidence; new action without coverage | Dishonest “no impact” declaration | Deterministic matrix failure | Adds completeness over existing targeted tests |
| Per-feature spec/acceptance obligation | Omitted or weak parity planning at change time | Bad reviewer judgment | Feature cannot close | Integrates with ordinary evidence packages |

## 7. Enforcement-posture recommendation

### 7.1 Rejected option: a new top-level standing gate code

A new rung beside `P0-CERT`, `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, and `FIRST-PROOF-CERT` would create the wrong lifecycle. Parity is not a one-time state to certify and then leave behind. It is a change-coupled obligation that must fire whenever a feature is added. A separate periodic audit invites the exact lag this task is meant to prevent: features may merge first and await a later parity campaign.

The report therefore does not mint a gate code.

### 7.2 Rejected option: fold only into `EPI-CERT` or `FIRST-PROOF-CERT`

Those packages are already passed historical certifications at their scoped implementation commits, as recorded in `docs/4-specs/SPEC_LEDGER.md`. They are also too narrow as permanent owners. Future parity applies to institutions, notices, travel, LOD-visible consequences, ordinary-life expansion, and eventual domain packs—not only epistemic substrate or the first-proof village.

Future certification packages should include parity evidence where relevant, but historical certs should not be reopened or made the sole standing mechanism.

### 7.3 Recommended option: layered standing obligation

Adopt three mutually reinforcing levels:

1. **always-on code/CI level:** exhaustive renderer handling, no-wildcard presentation matches, source-conformance protection, capability runner, focused goldens;
2. **per-feature spec level:** mandatory capability-impact declaration and named parity evidence; and
3. **phase/certification aggregation level:** any later phase-entry or certification packet includes the parity conformance report for the capabilities in its scope.

This fits the authority order:

- architecture defines the permanent contract;
- execution defines proof mechanics and when feature acceptance is blocked;
- specs enumerate the concrete capabilities and evidence for a change;
- acceptance artifacts preserve the report and commands;
- reference keeps the review question and relapse risk visible.

### 7.4 Position in the existing ladder

The obligation should be registered in `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` and `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` as a standing acceptance dimension for Expansion-posture work. It does not add a rung or change the verdict of a passed rung. It must pass before any future feature spec, Phase-4 entry package, or second-proof package can be accepted.

The implementation hardening spec recommended in §9 should land **before acceptance of the first next expansion feature**, either as a short shared prerequisite spec or as the first bounded work package of that expansion. It should not be deferred until after Phase 4 or second-proof features have accumulated.

## 8. Per-finding recommendations

### Finding 1 — Foundation already owns the principle

**Driver.** The problem is silent implementation drift, not ambiguity about whether the TUI matters.

**Current coverage.** Foundation `08` says kernel and TUI advance together and gives a per-feature review rule; foundation `12` makes TUI/view-model, replay, no-human, and filtering part of done; foundation `14` preserves the actor-known truth firewall. Existing constitutional invariants already cover the TUI product role, parity, why-not, debug quarantine, no-human life, and genre separation.

**Tier-fit verdict.** No foundation amendment. Adding test mechanics or registry fields here would overfit the constitution and violate the instruction to route to the highest legitimate owner and no higher.

**Recommendation.** During reassessment, cite the existing foundation clauses as drivers for the architecture/execution amendments. Do not invent a new invariant and do not weaken the embodied/debug split.

### Finding 2 — Architecture needs a two-hop playable-surface contract

**Driver.** Architecture `10` owns possession, view models, semantic action entries, debug boundaries, and the TUI harness, but it does not state that completeness must be proven independently at projection and rendering boundaries.

**Current coverage.** `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` correctly requires snapshot-like view models, no live truth handles, semantic-action submission through the pipeline, and TUI flow tests. It does not require exhaustive presentation handling or a capability declaration that catches a projection that never existed.

**Tier-fit verdict.** Amend architecture `10`.

**Recommendation.** The document should own, at architecture altitude:

- the two-hop decomposition;
- the rule that closed core→client presentation contracts are deliberately breaking when they grow;
- an explicit presentation disposition for every view-model field and closed presentation variant;
- a requirement that every playable capability have an actor-filtered surface contract or an architecture-justified non-playable classification;
- a prohibition on using debug or raw truth to satisfy embodied parity;
- a requirement that semantic actions remain data-driven pipeline entries rather than TUI-owned rules.

It should not prescribe exact test filenames, snapshot libraries, or command lines.

### Finding 3 — Acceptance architecture needs declared-capability closure

**Driver.** Hop 1 is semantic: the system needs an inventory of what must be proved.

**Current coverage.** Architecture `13` requires validation, observability, acceptance artifacts, and non-string-only evidence, but it does not bind a feature inventory to a complete set of witnesses.

**Tier-fit verdict.** Amend `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`.

**Recommendation.** This document should own the concept that a feature’s declared playable capabilities form a conformance set. Each set member must map to typed causal evidence, actor-knowledge evidence, a surface disposition, rendered evidence where playable, negative evidence where hidden/stale states matter, and replay/no-human evidence where applicable. It should require a deterministic conformance report and make absence of an entry or witness an acceptance failure.

### Finding 4 — Register the contract in the architecture index

**Driver.** Cross-cutting obligations disappear when they are discoverable only by reading one feature document.

**Current coverage.** Architecture `00` indexes conformance responsibilities but does not register a standing parity-completeness contract.

**Tier-fit verdict.** Amend `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`.

**Recommendation.** Add the obligation and point to architecture `10` for client/view-model boundaries and `13` for evidence closure. The index entry should remain concise and should not duplicate execution mechanics.

### Finding 5 — Hop 2 needs a compiler tripwire in the real renderer

**Driver.** `render_embodied_view` can silently ignore a new `EmbodiedViewModel` field.

**Current coverage.** Current renderer tests cover known fields and formatting; CI denies warnings. No exhaustive pattern exists.

**Tier-fit verdict.** Code/test implementation plus execution `10` doctrine.

**Recommendation.** Exhaustively destructure the real `EmbodiedViewModel`, deny unused bindings locally, explicitly justify intentional omissions, and add a conformance guard rejecting `..` or bare wildcard field handling. Do not add `#[non_exhaustive]` to the view model. The future spec should include an acceptance witness demonstrating that a controlled temporary field addition breaks the build until dispositioned.

### Finding 6 — Closed variants and data-driven actions need different guards

**Driver.** Treating every action as an enum variant would misread the code and could push action rules into the TUI.

**Current coverage.** `VisibleItemSource` already has exhaustive TUI handling. `WhyNotFailureKind`, `ActionAvailability`, provenance kinds, `DebugViewModel`, `ActionEffect`, and related enums have closed variants at the target commit. `SemanticActionEntry` carries IDs and typed availability data; `ActionRegistry::definitions` enumerates registered actions.

**Tier-fit verdict.** Code/test plus execution `10`.

**Recommendation.** Require exhaustive no-wildcard matches at the owner of presentation semantics for closed enums. Separately, require the capability runner to enumerate every registered action definition and prove at least one actor-known surface disposition. Do not add a TUI action-kind switch that duplicates core validation or execution logic.

### Finding 7 — Hop 1 needs focused real-pipeline proof

**Driver.** A core feature absent from projection produces no field-level failure.

**Current coverage.** Existing `embodied_flow`, acceptance, adversarial, and fixture tests prove selected features and anti-contamination behavior. They do not assert that the feature set is complete.

**Tier-fit verdict.** Amend execution `07`, `09`, and `10`.

**Recommendation.** Execution `07` should own the actor-known positive/negative and debug-quarantine proof shape. Execution `09` should own fixture coverage, focused golden scenarios, and fixture-ID resolution. Execution `10` should own real-pipeline invocation, typed-before-rendered assertions, checked-in render references, CI commands, and report capture.

### Finding 8 — Determinism is not a golden oracle

**Driver.** Two identical incomplete transcripts are still identical.

**Current coverage.** `transcript_snapshot.rs` proves byte-for-byte repeatability and stable section presence. That is useful replay/determinism evidence.

**Tier-fit verdict.** Add code/tests; clarify execution `10`.

**Recommendation.** Preserve the determinism test and add checked-in accepted output for capability scenarios. Keep snapshots focused, short, and reviewable. Prefer the repository’s explicit byte-stable style unless the owner chooses `insta` for review ergonomics; the doctrine should require an accepted reference, not a particular library.

### Finding 9 — Add a capability registry and non-vacuous runner

**Driver.** Scenario tests without an inventory prove only remembered cases.

**Current coverage.** `GoldenFixture::all` and `by_id` enumerate fixtures; `ActionRegistry::definitions` enumerates actions. There is no cross-surface mapping.

**Tier-fit verdict.** Architecture `13`, execution `09`/`10`, and a future implementation spec.

**Recommendation.** Add test-side capability entries and a generic runner as described in §6.4. Generate a deterministic report. Compare action definitions to coverage entries. Require fixture existence, positive typed/rendered witnesses, anti-leak witnesses for epistemic cases, and explicit non-playable classifications. Keep the registry downstream and non-authoritative.

### Finding 10 — Enforce parity per feature rather than through a new rung

**Driver.** The passed certification ladder cannot by itself govern later expansion, and a separate periodic parity audit would permit lag between audits.

**Current coverage.** Execution `00` and `03` define gate order; the ledger records historical passes; specs README defines future spec posture.

**Tier-fit verdict.** Amend execution `00`/`03` and `docs/4-specs/README.md`; no new gate identifier.

**Recommendation.** State that every Expansion feature spec must carry a parity-impact declaration and passing parity evidence before acceptance. Aggregate the evidence into future phase/cert packets. Run the generic conformance suite in ordinary CI on every change.

### Finding 11 — Acceptance artifacts need a parity evidence block

**Driver.** Evidence must be reviewable and persist with the implementation verdict.

**Current coverage.** `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` has requirement/evidence and typed-witness structures but no capability coverage report.

**Tier-fit verdict.** Amend `0003` or its future generalized successor.

**Recommendation.** The artifact shape should include:

- target implementation commit and fixture/content fingerprints;
- capability entries in scope;
- generated coverage report;
- typed causal and actor-known witnesses;
- rendered golden paths/digests;
- anti-leak and debug-quarantine evidence;
- replay/no-human disposition;
- compiler/source-conformance evidence;
- exact commands and verdicts.

Do not reduce the package to screenshots or display strings.

### Finding 12 — Reference should expose the decisive review question and relapse pattern

**Driver.** Reviewers need a quick way to detect future drift, and the risk register already has the correct entry.

**Current coverage.** Reference `00` asks for TUI/view-model evidence. Existing `R-15` describes TUI-first erosion. The glossary defines view model, possession, and no-human simulation but not this combined mechanism.

**Tier-fit verdict.** Amend reference `00`; update the substance of existing `R-15`; route a glossary term to reassessment.

**Recommendation.** The checklist should ask: “For every capability added or changed, where is its declared surface disposition and real-pipeline actor-filtered witness?” `R-15` should name both relapse modes—projection blindness and renderer blindness—and the expected controls. Do not mint a new risk ID. Add a glossary term only after the owner ratifies the final mechanism name.

### Finding 13 — Domain packs should compose coverage profiles later

**Driver.** Future flavor packs will add feature sets that must not become invisible or force genre knowledge into core.

**Current coverage.** Foundation already assigns flavor to domain packs and keeps core genre-agnostic. No current pack implementation exists in scope.

**Tier-fit verdict.** Route forward to the capability-registry implementation design and later pack specs; no pack work now.

**Recommendation.** Reserve namespacing and profile composition in the registry design. A future pack cannot be considered active/accepted unless its profile passes the same conformance runner. Do not design fantasy/post-apocalyptic UI content or modernization in this work.

### Finding 14 — Live specs and ledger remain unchanged until work is declared

**Driver.** This report is not a numbered spec and must not silently declare execution work.

**Current coverage.** `SPEC_LEDGER.md` says no forward feature work is declared and identifies Phase-4/second-proof entry as next known moves. Live `0001` owns the current ontology/fixture contract.

**Tier-fit verdict.** No immediate amendment to the ledger or `0001`.

**Recommendation.** The owner/reassess process should first accept the doctrine changes and then author a bounded implementation spec. Only that declaration changes the ledger. Existing live fixture doctrine may be reused, not rewritten as if parity were already ratified.

## 9. Sketched future implementation spec

This is an outline, not a numbered spec and not paste-ready wording.

### 9.1 Purpose

Land the standing two-hop parity mechanism before either Phase-4 entry or second-proof feature acceptance can introduce additional drift.

### 9.2 Scope

- amend the doctrine homes identified in §8;
- convert the real embodied renderer to exhaustive field handling;
- make presentation-semantic closed matches exhaustive and no-wildcard;
- add a source-conformance guard that protects those structures;
- create the test-side capability registry and generic runner;
- classify the current base capabilities and every current registered action;
- add focused real-pipeline positive and anti-leak scenarios for the present baseline;
- add checked-in expected renderings while preserving current deterministic transcript checks;
- emit a deterministic parity coverage report for acceptance artifacts;
- wire the suite into ordinary workspace/CI evidence.

### 9.3 Explicit non-goals

- TUI modernization or richer terminal layout;
- new Phase-4 institutions, notices, travel, regional, or LOD behavior;
- implementation of domain packs;
- changes that expose hidden truth to embodied play;
- TUI-owned simulation rules or direct event application;
- reopening passed certifications or minting a new gate code;
- backwards-compatibility aliases or shims.

### 9.4 Suggested work packages

1. **Contract hardening:** renderer destructure, local lint deny, exhaustive presentation matches, source-conformance tests.
2. **Capability model:** test-only entry schema, deterministic ordering, fixture/action resolution, surface classifications.
3. **Baseline migration:** enumerate current registered actions and current playable capability families; document explicit debug/headless exceptions.
4. **Pipeline witnesses:** focused fixture-driven typed + rendered + anti-leak cases; preserve possession parity and replay.
5. **Golden acceptance:** checked-in reference format/helper, review/update policy, deterministic digests.
6. **Evidence integration:** generated conformance report, command transcripts, full workspace checks, doctrine references.
7. **Mutation/adversarial closure:** prove that removing the exhaustive pattern/source guard, dropping a capability witness, leaking debug truth, or registering an uncovered action causes failure.

### 9.5 Acceptance evidence

The implementation package should carry:

- full workspace formatting, build, clippy-with-denied-warnings, and test results;
- named TUI parity/conformance test output without inventing a new canonical gate code;
- a controlled compile-break transcript showing a temporary added view-model field and presentation variant are rejected until handled;
- source-conformance test output proving no `..` or wildcard escape exists in the protected presentation points;
- the generated baseline capability matrix with zero uncovered entries;
- evidence that every `ActionRegistry::definitions` entry maps to a passing capability case;
- focused checked-in rendered goldens and their reviewable diffs;
- paired actor-known/hidden-state tests for epistemic cases;
- debug-quarantine, no-direct-dispatch, replay, no-human, and determinism results from existing suites;
- content/fixture fingerprints and exact implementation commit.

### 9.6 Placement relative to next expansion work

The spec should be accepted as a shared hardening prerequisite before the first Phase-4 or second-proof feature is accepted. It may be sequenced immediately before that feature or as the first bounded package of the same expansion campaign, but the feature must not close first and “add TUI parity later.”

The spec should not claim that the current target commit is latest `main`; it should name its own exact implementation commit when executed.

## 10. Forward-routing appendix

This target is cross-cutting rather than a downward cascade to an unreviewed lower tier. All legitimate documentation homes are already identified above. **No finding routes to a lower documentation tier than the tier that legitimately owns it.** The remaining out-routes are:

### Owner/reassess decisions

- Ratify the final mechanism term for possible glossary inclusion.
- Choose whether checked-in text goldens use a minimal repository helper or `insta`; the report recommends preserving explicit byte-stable style by default.
- Choose the exact downstream test-support location for the capability registry while preserving the one-way crate dependency direction.
- Decide the granularity of capability keys: feature-level, action-level, or grouped actor-observable contract, subject to the anti-vacuity rules here.
- Decide whether a broad integrated smoke transcript supplements the focused matrix.
- Resolve any possession-bind perception-refresh question only through its existing architecture owner. The parity mechanism must test the ratified behavior; it must not silently invent perception on bind.

### Future implementation specs

- Land the base parity hardening outlined in §9.
- Require each later Phase-4/second-proof feature spec to declare and close its parity impact.
- Add namespaced per-domain-pack profiles only when domain packs become real work.
- Let any future TUI-modernization spec consume the same capability matrix rather than replacing it.

### Explicitly not routed

- No new constitutional invariant.
- No new risk identifier.
- No new certification gate code.
- No amendment to archived specs.
- No immediate declaration in `SPEC_LEDGER.md` by this report.

## 11. Open questions

These do not block the recommendation; they are implementation/reassessment choices that the exact-commit repository and external research do not uniquely determine.

1. **Capability granularity.** One entry per action ID is mechanically clear, but non-action features need a principled grouping rule. The owner should prefer actor-observable contracts over internal module names.
2. **Registry location.** A TUI integration-test support module is the least invasive; a content-side conformance module may better reuse fixture metadata. Either is acceptable if core remains unaware of TUI/test metadata.
3. **Golden storage.** Inline strings, adjacent `.golden` files, or `insta` snapshots all work. The decisive requirement is a checked-in accepted reference with review discipline, not the library.
4. **Intentional field omissions.** The implementation spec should settle the exact convention for named underscore bindings and rationale enforcement so omission does not become a dumping ground.
5. **Baseline capability census.** The current action registry is enumerable, but the first migration must decide which non-action capabilities deserve separate entries—epistemic filtering, no-human autonomy, needs/routines, notebook leads, rejection/why-not, and debug quarantine are likely minimum families.
6. **Possession bind and perception.** If a future scenario expects bind-time perception, that behavior must be decided by the existing architecture owner rather than added as a parity-test convenience.

## 12. References

[^1]: The Rust Reference, “Patterns,” especially struct destructuring, wildcard, and rest-pattern semantics. <https://doc.rust-lang.org/reference/patterns.html>

[^2]: The Rust Reference, “The `non_exhaustive` attribute,” especially downstream matching requirements. <https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute>

[^3]: Rust error code `E0004`, non-exhaustive match diagnostics. <https://doc.rust-lang.org/error_codes/E0004.html>

[^4]: The rustc book, “Warn-by-default lints — `unused_variables`.” <https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unused-variables>

[^5]: Luc Maranget, “Warnings for Pattern Matching,” *Journal of Functional Programming* 17(3), 2007. <https://moscova.inria.fr/~maranget/papers/warn/warn.pdf>

[^6]: The Elm Guide, “The Elm Architecture.” <https://guide.elm-lang.org/architecture/>

[^7]: Insta Snapshots, “Getting Started — Reviewing Snapshots.” <https://insta.rs/docs/quickstart/>

[^8]: Jest, “Snapshot Testing — Best Practices.” <https://jestjs.io/docs/snapshot-testing>

[^9]: Testing Library, “Guiding Principles.” <https://testing-library.com/docs/guiding-principles/>

[^10]: Pact, “Introduction — Consumer Driven Contracts.” <https://docs.pact.io/>

[^11]: Kubernetes Gateway API, “GEP-2162: Supported features in GatewayClass Status.” <https://gateway-api.sigs.k8s.io/geps/gep-2162/>

[^12]: Kubernetes Gateway API, “Conformance,” especially conformance profiles and reports. <https://gateway-api.sigs.k8s.io/docs/concepts/conformance/>

[^13]: `trybuild` crate documentation, compile-fail tests. <https://docs.rs/trybuild/latest/trybuild/>

[^14]: Rust Compiler Development Guide, “UI tests.” <https://rustc-dev-guide.rust-lang.org/tests/ui.html>

[^15]: Epic Games, “Game Features and Modular Gameplay in Unreal Engine.” <https://dev.epicgames.com/documentation/unreal-engine/game-features-and-modular-gameplay-in-unreal-engine>

[^16]: Factorio API documentation, “Data lifecycle.” <https://lua-api.factorio.com/latest/auxiliary/data-lifecycle.html>
