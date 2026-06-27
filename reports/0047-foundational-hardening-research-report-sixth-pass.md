# 0047 Foundational Hardening Research Report — Sixth Pass

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `7660051747aaa1c768cca9dc73385b16573ebe67`  
**Report type:** research / audit / remediation recommendation, not a numbered spec.  
**Mode:** static survey only. I did not run `cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`, CI, GitHub API commands, or `cargo-mutants`.

## 1. Verdict

The spec-0047 surface is **not foundationally conformant** at `7660051747aaa1c768cca9dc73385b16573ebe67`.

The 0053 line made real progress: it introduced a runtime-centered loaded-world path, closed command dispatch, private receipt constructors for continuation/debug products, a runtime-minted debug authority type, compile-fail negative fixtures, food-source replacement tests, and a status-manifest parser. Those properties are recorded as present below. But two core API authority breaks remain live, and the new process machinery does not make a laundered pass impossible.

The decisive code defects are:

1. **The loaded-world bootstrap is still forgeable from public raw aggregate constructors.** `PhysicalState::from_validated_seed_parts`, `AgentState::from_validated_seed_parts`, and `ValidatedLoadedWorldBootstrap::from_validated_content` are public. An external crate can assemble arbitrary `PhysicalState`, `AgentState`, `EventLog`, and `EpistemicProjection` and pass them into `LoadedWorldRuntime::from_bootstrap`. The 0053 compile-fail fixture attacks obsolete names (`from_seed_parts`, `from_loaded_state`) rather than the live public constructors. This falsifies the F5-01 closure claim.

2. **The embodied one-tick wait/public runtime receipt still exposes a debug-grade `WorldAdvanceResult`.** `RuntimeReceiptKind::OneTickAdvanced(WorldAdvanceResult)` is public through `RuntimeReceipt::kind()`, while `WorldAdvanceResult` and its nested summaries expose exact ticks, event IDs, due-work summaries, actor-step summaries, proposal ancestry, decision trace IDs, local plan IDs, and pipeline status. Even if the current TUI renderer ignores that payload, the embodied/public API boundary is not structurally sealed. This contradicts the 0053 architecture claim that exact ticks/frontiers/replay detail/due queues remain debug/operator surfaces and that normal continuation products carry only actor-legible output.

3. **The debug authority token is unforgeable as a struct but still obtainable through an unguarded public debug bind path.** `DebugSessionAuthority::mint()` is crate-private, which is good. But `RuntimeCommand::bind_debug_controller(controller_id, actor_id)` is public and requires no prior authority. After submitting it, a caller can ask `LoadedWorldRuntime::debug_session_authority_for(...)` and receive a runtime-minted authority. The negative fixture proves that a caller cannot call `DebugSessionAuthority::mint()` directly; it does not prove that debug authority cannot be induced by a public command. This makes the token a real Rust privacy barrier for the struct literal but not a durable public-boundary authority barrier.

The process-fix verdict is also **not sound**. The 0053 status taxonomy recomputes consistency from a fenced `tracewake-acceptance-status` block, but the block is self-authored, hardcoded to F5-01 through F5-06, and uses scalar/status strings as evidence. The parser does not validate the cited code, negative fixtures, mutation transcript, or ruleset transcript. The wording guard is a phrase denylist, not a closed grammar of allowed result claims. The parser contains a survivor-pass hole: `mutation_status: non-blocking-bounded-forcing` with explicit survivor rows can still compute `Pass`. The 0053 artifact was not forced through this mechanism as a first-class current artifact before archival in a way that would have caught the live bootstrap/debug/governance defects.

The governance verdict is **partial merge enforcement, not independent acceptance**. The 0053 ruleset transcript shows required status checks, no bypass actors, and strict status checks, which is real status-check enforcement. But the same transcript shows `required_approving_review_count: 0`, `require_last_push_approval: false`, and `required_reviewers: []`. A sole maintainer who implemented the code can self-merge once checks are green. The `governance-required-checks-audit` job checks required contexts and bypass actors, but it does not fail on zero required approvals, lack of last-push approval, lack of required reviewers, or implementer-as-acceptor. F5-04 is therefore not closed as an independent-acceptance requirement.

The `food_source_fact_supersedes` residual appears materially repaired for the named semantics: the tests exercise source-bearing/source-less replacement and deterministic source-key ordering through `build_embodied_view_model`, and `.cargo/mutants.toml` includes the projection files in the standing perimeter. I do **not** certify the historical zero-survivor run as current. The remaining gap is forcing strength: the best closure should add a public actor-known/TUI or fixture-level witness with event/provenance ancestry so the routed-forward pattern cannot recur behind a direct context/projection helper.

**Higher-tier amendment verdict:** no Tier-0 foundation amendment is warranted. The constitution already forbids the live defects: no fact born from prose or raw convenience state, event/replay authority, holder-known truth firewall, embodied/debug separation, no TUI authority, and evidence honesty. A below-foundation doctrine strengthening is warranted in `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`, `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`, and `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`: a computed `pass` must require independent, current, mechanically ingested evidence; no mutation-survivor disposition may compute pass; and 0-approval self-merge-able governance must compute `pending-governance` or `non-pass` where independent acceptance is in scope.

**Another iteration is needed.** It is needed not because the previous work was worthless, but because the sixth audit finds live code authority defects plus process/governance holes that would allow another self-consistent “pass” artifact over non-conforming code. The next line should close the raw bootstrap/debug/wait receipt API breaks first, then repair the acceptance taxonomy and governance forcing function before declaring closure.

## 2. Disposition table

| Finding | Primary target | Classification | One-line basis |
|---|---|---|---|
| F6-01 | `crates/tracewake-core/src/state.rs`; `crates/tracewake-core/src/runtime/session.rs`; `tracewake-content` handoff; bootstrap negative fixture | Foundational code violation | Public “validated” seed-part constructors plus public `ValidatedLoadedWorldBootstrap::from_validated_content` still allow caller-authored loaded state/log/projection to enter the runtime as validated. |
| F6-02 | `crates/tracewake-core/src/runtime/receipt.rs`; `crates/tracewake-core/src/scheduler.rs`; TUI wait path | Foundational code violation / information-flow leak | Public `RuntimeReceiptKind::OneTickAdvanced(WorldAdvanceResult)` exposes exact tick/event/due-work/actor-step internals on a public world-advance receipt. |
| F6-03 | `crates/tracewake-core/src/runtime/command.rs`; `crates/tracewake-core/src/runtime/session.rs`; `crates/tracewake-tui/src/app.rs`; debug negative fixtures | Debug-authority hardening gap rising to boundary violation | `DebugSessionAuthority` is private-minted, but any public caller can submit `bind_debug_controller` and then obtain a token through runtime binding state. |
| F6-04 | `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`; `acceptance_status_manifest.rs`; `acceptance_artifact_wording.rs`; template | Process-integrity gap | The status parser checks self-consistency of a self-authored block; the wording guard is a phrase denylist; survivor rows can still compute pass under bounded-forcing status. |
| F6-05 | `.github/workflows/ci.yml`; `ci_workflow_guards.rs`; repository ruleset doctrine | Governance gap | Required checks are materially present, but `required_approving_review_count: 0` and no last-push/required-reviewer constraint leave implementer-as-acceptor self-merge possible. |
| F6-06 | `.github/workflows/ci.yml`; `.cargo/mutants.toml`; mutation acceptance taxonomy | Enforced standing-gate gap | A required “full-surface mutation trigger” status is a topology alarm; in-diff mutation and full-campaign freshness are not forced as PR-blocking proof for guarded changes. |
| F6-07 | `crates/tracewake-core/src/projections.rs`; `food_source_projection.rs`; `.cargo/mutants.toml` | Present with hardening gap / residual disposition | The named food-source semantics are now represented in focused behavior tests, but the strongest closure needs a public actor-known/TUI fixture path and no current mutation result may be certified by static survey. |

## 3. Method & provenance ledger

### Authority and evidence lanes

I applied the repository authority order from `docs/README.md` and the foundation index: foundation doctrine governs architecture, architecture governs execution, execution governs specs/code/test artifacts, and archived specs/reports are historical evidence, not live authority. The uploaded manifest was used only as a path inventory. Every target-repository claim in this report is based on files fetched from exact commit URLs for paths present in that manifest.

The user-supplied brief states that the 0053 implementation commit under test is byte-identical to this baseline under `crates/`, `.github/`, and `.cargo/`, and that only archival/ledger moves occurred after 0053. I treated that as a scope instruction and advantage, not as proof of repository state. Repository-state claims here still derive from files fetched at `7660051747aaa1c768cca9dc73385b16573ebe67`.

No substantive target-repository analysis began until representative exact-commit fetches had succeeded and provenance preflight showed clean raw URLs for the requested owner, repository, commit, and path. Later fetches were appended to the ledger below.

### Mandatory exact-commit acquisition ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 7660051747aaa1c768cca9dc73385b16573ebe67
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open against full raw.githubusercontent.com exact-commit URLs; selected plain-text files were also downloaded after exact-URL open when the tool accepted the content type
Requested file count: 125
Successfully verified file count: 125
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

**Fetched repository files:**

- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/runtime/session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/debug_capability.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/runtime/receipt.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/runtime/command.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/runtime/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/epistemics/projection.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/events/envelope.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/events/log.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-content/src/load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-content/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-content/src/schema.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/src/run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/src/launch.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/support/acceptance_status_manifest.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/acceptance_status_manifest.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/acceptance_artifact_wording.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/archive/reports/0053_foundational_conformance_fifth_hardening_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/reports/0047-foundational-hardening-research-report-fifth-pass.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/ci_workflow_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/negative_fixture_runner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/food_source_projection.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/world_step_coordinator.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/tests/tui_seam_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/tests/command_loop_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/.cargo/mutants.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/.cargo/mutants-baseline-misses.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/archive/reports/0047_tui_authoritative_world_advance_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/reports/0048_foundational_conformance_hardening_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/archive/reports/0050_foundational_conformance_second_hardening_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/archive/reports/0051_foundational_conformance_third_hardening_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/tests/negative-fixtures/external_crate_cannot_submit_debug_command_without_token/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/tests/negative-fixtures/external_crate_cannot_convert_debug_report_to_interval_summary/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/tests/negative-fixtures/external_crate_cannot_mutate_embodied_temporal_fields/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/archive/specs/0053_FOUNDATIONAL_CONFORMANCE_FIFTH_HARDENING_SEALED_BOOTSTRAP_INTERVAL_PRODUCT_TOKENIZED_DEBUG_AUTHORITY_MERGE_ENFORCED_BARRIER_AND_FAIL_CLOSED_ACCEPTANCE_TAXONOMY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/archive/specs/0050_FOUNDATIONAL_CONFORMANCE_SECOND_HARDENING_LOADED_WORLD_DISCOVERY_ACTOR_TRANSACTION_UNIFICATION_TUI_DEAUTHORITY_AND_REPLAY_FAIL_CLOSED_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/archive/specs/0051_FOUNDATIONAL_CONFORMANCE_THIRD_HARDENING_CORE_OWNED_LOADED_WORLD_RUNTIME_PRODUCTION_REACHABILITY_PROCESS_TRANSACTIONS_ACTOR_CENSUS_AND_TUI_DEAUTHORITY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/archive/specs/0052_FOUNDATIONAL_CONFORMANCE_FOURTH_HARDENING_PRODUCTION_BOOTSTRAP_CLOSED_COMMAND_BOUNDARY_REPLAY_AUTHORITY_REAL_PROCESSES_ACTOR_CENSUS_EMBODIED_DEBUG_SPLIT_AND_STANDING_BARRIER_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/reports/0047-foundational-hardening-research-report-fourth-pass.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/reports/0047-foundational-hardening-research-brief-fifth-pass.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/agent/transaction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/agent/trace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/agent/perception.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/replay/temporal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/tests/playable_capability_parity.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/generative_lock.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/replay_temporal_frontier.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/holder_known_interval_projection.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/salient_stop_actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/reservation_body_exclusive_census.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-core/tests/mutation_completion_merge.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/tests/parity_adversarial.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/7660051747aaa1c768cca9dc73385b16573ebe67/crates/tracewake-tui/tests/transcript_snapshot.rs

### Static-survey limitation

This report does not certify command results, current mutation results, CI state, or live GitHub ruleset state. Historical command outcomes, mutation counts, and API transcripts quoted from acceptance artifacts are treated as claims by those artifacts. The current judgments are static source-shape judgments about visibility, public APIs, data flow, witness intent, and CI/ruleset configuration as written.

## 4. Re-verified present properties from 0053

This pass should not erase real progress. The following properties are present in the fetched source, even though they do not close every claim 0053 made.

| Property | Static evidence | Disposition |
|---|---|---|
| Runtime-centered loaded-world path exists | `TuiApp::from_golden` loads content through `LoadedFixture::into_runtime_bootstrap` and `LoadedWorldRuntime::from_bootstrap`; `LoadedWorldRuntime` owns registry, physical state, agent state, event log, epistemic projection, controller bindings, scheduler, content manifest, fixture id, and initial replay state. | Present, but undermined by public raw constructors. |
| Closed command enum exists | `RuntimeCommand` has a private `kind`, and `RuntimeCommandKind` is `pub(crate)`. Public constructors exist for semantic action, wait, continue, bind/detach, no-human, rebuild, embodied view, and debug view. | Present, with debug-bind caveat. |
| Continuation receipt product is sealed | `ContinuedRuntimeReceipt` has private fields and a private constructor from `AdvanceUntilResult`; accessors expose `advanced`, appended event count, and optional `TypedActorKnownIntervalSummary`, not the raw `AdvanceUntilResult`. | Present for `continue_until`. Not true for one-tick wait receipt. |
| Debug receipt product carries a capability | `DebugRuntimeReceipt::new` is crate-private and stores a `DebugCapability`; public accessors expose debug data only from that debug receipt. | Present as a product seal. Not sufficient while debug authority can be induced by public bind. |
| Debug token struct itself is not externally forgeable | `DebugCapability` and `DebugSessionAuthority` have private fields; `mint()` is `pub(crate)`; the doctests show struct-literal and direct-mint compile failures. | Present as Rust privacy. Public acquisition path still needs closure. |
| Embodied view temporal/debug fields are private | `EmbodiedViewModel` keeps `sim_tick`, holder-known context id/hash/frontier/source summary, interval summary, and `debug_available` as crate-private with read accessors; setters are crate-private. | Present as read-only view-model sealing. |
| Actor decision transaction path exists | `ActorDecisionTransaction::run` consumes an `ActorKnownPlanningContext`, checks provenance diagnostics, generates candidates from actor-known facts, rejects hidden-truth audit failures, selects a method, builds a local plan, and seals the proposal. | Present. No new violation found in the surveyed transaction flow. |
| Scheduler-owned due work exists | The scheduler derives due actors/processes inside `transact_world_one_tick` and emits per-tick actor disposition summaries rather than accepting caller-injected actor lists on the public runtime path. | Present in the surveyed path. |
| Food-source replacement behavior was strengthened | `food_source_projection.rs` tests source-bearing/source-less replacement and deterministic source-key ordering through view-model construction; `.cargo/mutants.toml` includes the relevant projection files. | Present for the named semantics; needs public-boundary strengthening and fresh executed proof. |
| Acceptance taxonomy parser exists | `acceptance_status_manifest.rs` extracts a fenced status block and recomputes a result from statuses and mutation/governance scalars. | Present, but not fail-closed enough. |

## 5. Per-finding sections — code surface

### F6-01 — Public raw aggregate construction still forges “validated” loaded-world bootstrap

**Foundational driver.** The controlling doctrine is event/replay authority, validated content authority, and the prohibition on simulation facts born from prose or convenience state. Foundation `00` says meaningful world change must arise from modeled cause and deterministic replay; foundation `03` makes event traces and replay authoritative; execution `08` requires authored content to pass schema/provenance/validation before becoming simulation state; architecture `04` and execution `05` place the world-step coordinator and bootstrap boundary in core-owned production paths.

**Current code state.** `crates/tracewake-core/src/lib.rs` publicly exports `state` and `runtime`. `runtime/mod.rs` publicly re-exports `LoadedWorldBootstrap`, `LoadedWorldRuntime`, `RuntimeReplaySeed`, and `ValidatedLoadedWorldBootstrap`.

`PhysicalState::from_validated_seed_parts` is public and accepts arbitrary maps of actors, places, doors, containers, items, food supplies, workplaces, sleep affordances, and need model, then installs them directly into `PhysicalState` (`state.rs`, lines 219-245 in the exact fetched file). `AgentState::from_validated_seed_parts` is public and accepts arbitrary needs, intentions, active intentions, routine executions, decision traces, and stuck diagnostics, then installs them directly into `AgentState` (`state.rs`, lines 313-335).

`ValidatedLoadedWorldBootstrap::from_validated_content` is public and accepts a registry, `PhysicalState`, `AgentState`, `EventLog`, `EpistemicProjection`, and content IDs, then wraps them as a `ValidatedLoadedWorldBootstrap` (`runtime/session.rs`, lines 125-148). `LoadedWorldRuntime::from_bootstrap` is public and consumes that wrapper into the authoritative runtime (`runtime/session.rs`, lines 191-210). The wrapper has private fields, but the public constructor takes already-forged aggregates.

The intended content path exists: `tracewake-content` loads a fixture and calls `LoadedFixture::into_runtime_bootstrap(registry)`, which then invokes `ValidatedLoadedWorldBootstrap::from_validated_content`. The problem is not that the content path is absent; the problem is that the same raw-state handoff is public to any downstream crate.

The 0053 negative fixture `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` is stale. It attacks `PhysicalState::from_seed_parts`, `AgentState::from_seed_parts`, `LoadedWorldBootstrap::from_loaded_state`, and `ValidatedLoadedWorldBootstrap::from_loaded_state`, but those are not the live public APIs. It does not attempt the current `from_validated_seed_parts` plus `from_validated_content` route.

**Conformance verdict.** Violation. The closure claim “callers cannot pass raw aggregate state directly to the runtime constructor” is false at the public API boundary. A downstream crate can still fabricate the state/log/projection substrate and call it “validated.”

**Required remediation.** The code home is `crates/tracewake-core/src/state.rs`, `crates/tracewake-core/src/runtime/session.rs`, and `crates/tracewake-content/src/load.rs`. The remediation must remove generic public raw aggregate assembly from the production API. Acceptable shapes:

- Make seed-part constructors crate-private/test-support-only and move validated content assembly behind a production API that never accepts caller-built aggregate maps; or
- Introduce an unforgeable validation/assembly witness token that only the content validation path can obtain, and require that token for any cross-crate state assembly that must remain public because `tracewake-content` is a separate crate; or
- Move the raw aggregate assembly fully into a content-owned package type whose fields are private and whose constructor is bound to schema validation/provenance checks, then have core consume only that package.

The docs home is architecture `04`, architecture `10`, execution `05`, execution `08`, execution `10`, reference `00`, reference `01`, and the spec ledger next-known move. Do not edit archived 0053 acceptance/spec artifacts.

**Anti-regression guard.** Add a compile-fail negative fixture using the live API names: external crate imports `PhysicalState::from_validated_seed_parts`, `AgentState::from_validated_seed_parts`, `EventLog::new`, `EpistemicProjection::new`, and `ValidatedLoadedWorldBootstrap::from_validated_content`, then attempts to call `LoadedWorldRuntime::from_bootstrap`. The fixture must fail for the authority reason, not because of stale method names or missing imports. Add a positive content-loader integration test proving `LoadedFixture::into_runtime_bootstrap` still reaches runtime through the validated path.

**Evidence-honesty check.** A closure artifact is vacuous if it only proves `LoadedWorldBootstrap` fields are private or if it attacks deleted method names. Non-vacuous closure must show that the current public constructors cannot be composed by an external crate to mint a runtime from caller-authored state/log/projection, while the production content loader still works.

### F6-02 — Public one-tick wait receipt exposes debug-grade world-step internals

**Foundational driver.** Foundation `08` requires embodied TUI surfaces to expose actor-grounded knowledge and keep debug separate. Foundation `14` and architecture `03` enforce the truth firewall. Architecture `10` says world-advancing controls are commands through core, not temporal knowledge grants, and that exact ticks/frontiers/replay detail/due queues belong to debug/operator surfaces. Execution `07` requires TUI to store/render core interval products read-only and keep embodied/debug products structurally split.

**Current code state.** `RuntimeReceiptKind` is public and includes `OneTickAdvanced(WorldAdvanceResult)` (`runtime/receipt.rs`, lines 16-24). `RuntimeReceipt::kind()` returns `&RuntimeReceiptKind` to external callers (`receipt.rs`, lines 99-101). `LoadedWorldRuntime::wait_one_tick` is public and calls `RuntimeCommand::one_tick_wait`; `run_one_tick_wait` transacts the scheduler step and returns `RuntimeReceipt::one_tick_advanced(result)` (`runtime/session.rs`, lines 664-695).

`WorldAdvanceResult` is a scheduler product with public fields: prior tick, resulting tick, appended event IDs, actor-known interval delta, due duration candidates, due work summary, actor step summaries, and controlled pipeline results. `ActorStepSummary` exposes actor id, proposal id, decision trace id, local plan id, proposal ancestry, pipeline status, committed event IDs, diagnostic IDs, and status. That is not an actor-legible receipt; it is a debug/operator step report.

`ContinuedRuntimeReceipt` is better: it hides the raw `AdvanceUntilResult` and exposes only `advanced`, appended event count, and optional actor-known interval summary. The one-tick path did not receive the same seal.

**Conformance verdict.** Violation. The public core API hands debug-grade scheduler internals to any caller who performs a one-tick world advance. Rendering discipline in the TUI cannot repair an information-flow leak already present in the public product. This is exactly the distinction emphasized by information-flow research: access-control or display filtering is not equivalent to enforcing noninterference at the data boundary.

**Required remediation.** The code home is `crates/tracewake-core/src/runtime/receipt.rs`, `runtime/session.rs`, `scheduler.rs`, and TUI app/run code that consumes wait receipts. Replace `RuntimeReceiptKind::OneTickAdvanced(WorldAdvanceResult)` with a sealed actor-legible `OneTickRuntimeReceipt` or reuse a generalized `WorldAdvanceRuntimeReceipt` product that contains only actor-visible progress, appended event count if actor-legible, and a core-built actor-known interval summary. Move full `WorldAdvanceResult` to crate-private scheduler/debug code or require `DebugSessionAuthority` to access it.

Docs home: architecture `10`, execution `07`, execution `10`, architecture `13`, reference `00` review prompts.

**Anti-regression guard.** Add a negative fixture proving an external crate cannot match a wait receipt and read `prior_tick`, `resulting_tick`, `appended_event_ids`, `due_work_summary`, `actor_step_summaries`, `decision_trace_id`, or `pipeline_status` from normal embodied wait. Add a behavior test that TUI wait displays only actor-legible summary and that debug/operator mode can still reach the debug report through token-gated API.

**Evidence-honesty check.** A screenshot or rendered transcript that omits hidden fields is not sufficient. Closure requires public API unrepresentability: external code should be unable to compile a read of debug-grade wait internals without debug authority.

### F6-03 — Debug authority token is private-minted but publicly inducible through debug binding

**Foundational driver.** Foundation `08` permits debug, but debug is non-diegetic and must not feed embodied play. Architecture `10` says debug/operator commands cross only with a runtime-minted `DebugSessionAuthority`; execution `07` requires token-gated debug and embodied/debug split.

**Current code state.** `DebugCapability` and `DebugSessionAuthority` have private fields and crate-private `mint()` methods. That is good Rust privacy. But `RuntimeCommand::bind_debug_controller(controller_id, actor_id)` is public and takes no authority (`runtime/command.rs`, lines 99-107). `LoadedWorldRuntime::submit_command` accepts the bind command and calls `bind_actor` with `ControllerMode::Debug` (`runtime/session.rs`, lines 617-626). `LoadedWorldRuntime::debug_session_authority_for` returns `DebugSessionAuthority::mint()` if the controller is bound in debug mode (`session.rs`, lines 221-241). `TuiApp::bind_debug_actor` is also public and delegates to that command (`tui/app.rs`, lines 102-127), after which `TuiApp::debug_authority` obtains the token (`app.rs`, lines 159-167).

The compile-fail fixture `external_crate_cannot_submit_debug_command_without_token` proves that a caller cannot call `RuntimeCommand::run_no_human_day()` with no argument and cannot call `DebugSessionAuthority::mint()`. It does not prove that the caller cannot first submit a public debug bind and then obtain the runtime-minted authority.

**Conformance verdict.** Violation or, at minimum, severe authority hardening gap. The token is not forgeable as a value, but the public command boundary lets a caller cause the runtime to mint one without a prior authority. If debug mode is meant to be a deliberate operator mode, the authority to enter that mode must be explicit and not obtainable by the same unprivileged command surface that it is supposed to guard.

**Required remediation.** The code home is `runtime/command.rs`, `runtime/session.rs`, `debug_capability.rs`, `tui/app.rs`, `tui/run.rs`, and negative fixtures. Options:

- Remove public `bind_debug_controller`; expose it only behind a separate operator/debug bootstrap token not available to ordinary embodied clients.
- Require an already-held operator authority to switch a controller into debug mode; after binding, `DebugSessionAuthority` may be scoped to that controller/actor.
- If the product deliberately permits debug attach from the local TUI, make that an explicit operator entrypoint outside the embodied command surface and ensure it cannot be mistaken for actor authority or ordinary play.

Docs home: architecture `10`, execution `07`, execution `10`, architecture `13`, and the acceptance template’s debug-evidence requirements.

**Anti-regression guard.** Add an external negative fixture that attempts the actual bypass: build a runtime from a golden fixture, call `RuntimeCommand::bind_debug_controller`, submit it, call `debug_session_authority_for`, and then call `RuntimeCommand::run_no_human_day(authority)` or `debug_view(authority)`. It must fail for lack of operator authority, not because the runtime setup is stale. Add a positive test for the approved debug/operator entrypoint.

**Evidence-honesty check.** Direct-mint compile-fail is not enough. Closure must prove that the public command surface cannot induce a debug token unless an independently held debug/operator authority was already present.

### F6-04 — Acceptance taxonomy is self-consistency checking, not fail-closed verification

**Foundational driver.** This is below-foundation process doctrine, but it enforces foundation closure. Architecture `13` owns validation/observability/acceptance artifact honesty. Execution `10` owns evidence status, fingerprint scope, pending-is-not-pass, mutation survivor discipline, merge-blocking red, and the acceptance result taxonomy.

**Current code state.** `acceptance_status_manifest.rs` extracts a fenced `tracewake-acceptance-status` block, parses required scalar keys, parses hardcoded F5-01 through F5-06 rows, computes `Pass` or `NonPass`, and checks the stated `overall_result` against the computed result. That is better than trusting prose.

But the computed result is only as good as the self-authored fields:

- `REQUIRED_FINDINGS` is hardcoded to `F5-01` through `F5-06`; the taxonomy is not generic for the next acceptance line.
- A `closed` finding requires the presence of `evidence=` and `negative=` fields, not validation that the evidence names live code, current method names, public-boundary behavior, or mutation-sensitive tests.
- `branch_protection: enforced` is a scalar string. The parser does not parse the ruleset transcript for required checks, bypass actors, review count, last-push approval, required reviewers, or current-user bypass state.
- The mutation logic treats `mutation_status: killed` with survivors as non-pass, but `mutation_status: non-blocking-bounded-forcing` with explicit survivor rows does not set `pass = false`. That means a manifest can contain survivors and still compute `Pass` if the rows have forcing fields.
- The status block is not independently derived from CI artifacts, ruleset API output, mutation JSON, or a required acceptance manifest. It is author-written text.

`acceptance_artifact_wording.rs` is a phrase denylist. It checks certain forbidden phrases, certain “green perimeter” wording with survivors, a particular governance sentence without a transcript, and display-only behavior claims. It is not a closed grammar. Paraphrase can evade it. It also intentionally strips or ignores wording after “Forbidden wording:” sections, which is sensible for templates but demonstrates that this is a textual heuristic, not semantic verification.

The workflow runs the wording test in `lock-layer gates`, but the test corpus is hardcoded. There is no evidence in the fetched files that every new acceptance artifact must be passed as an input artifact before merge/archive, nor that the 0053 acceptance itself is parsed as a current artifact rather than merely containing a block that would be self-consistent.

**Conformance verdict.** Process-integrity violation. The taxonomy catches some contradictions, but it does not make laundering impossible. It moves the recurrence from prose verdicts to self-authored machine-looking verdicts.

**Required remediation.** The code home is `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`, `acceptance_status_manifest.rs`, `acceptance_artifact_wording.rs`, `ci_workflow_guards.rs`, `.github/workflows/ci.yml`, and `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.

The parser should be converted from a self-authored status consistency check into a positive, fail-closed acceptance state machine:

- Expected findings come from an explicit review manifest or artifact list, not hardcoded F5 labels.
- `Pass` is impossible if any finding is open, pending, routed-forward, historical-only for a current requirement, not-in-scope for an in-scope requirement, or if evidence cannot be tied to current exact-commit files/tests.
- Any mutation survivor, missed mutant, timeout, untriaged baseline miss, or bounded-forcing survivor computes `NonPass` for a green-closure artifact. A bounded-forcing survivor can be honestly recorded, but not as pass.
- Governance status is computed from a parsed transcript or machine-readable artifact, including required checks, bypass actors, review count, last-push approval, required reviewers, and current-user bypass.
- Wording is generated/validated from a closed grammar keyed to computed state. Free prose may explain, but it cannot introduce a contradictory or stronger verdict.
- CI must ingest the actual acceptance artifact proposed for merge/archive, not only a static test corpus.

Docs home: architecture `13`, execution `10`, template `0003`, reference `00`, reference `01` existing R-27/R-28/R-29 status, and `SPEC_LEDGER.md` next-known move.

**Anti-regression guard.** Extend existing deterministic tests, no new framework. Add synthetic adversarial artifacts for survivor-pass, paraphrased closure over open rows, stale method-name negative evidence, branch/ruleset scalar without transcript, zero-approval governance, missing actual artifact ingestion, self-authored evidence strings, historical-current conflation, and display-only evidence. Add a mutation campaign over the parser/guard functions themselves; a guard whose own mutants survive is decorative.

**Evidence-honesty check.** A closure artifact is vacuous if it says “the parser passed” without showing that the exact artifact under review was ingested and that the parser’s own survivor family is killed or honestly non-pass.

### F6-05 — Governance ruleset enforces status checks but not independent acceptance

**Foundational driver.** This is process governance under architecture `13` and execution `10`. It matters because the recurrence is not just a code defect; it is an acceptance defect. A barrier that the same actor can author, satisfy, accept, and merge is not independent acceptance.

**Current source/artifact state.** The 0053 acceptance quotes a ruleset transcript for `main-standing-conformance-barrier`: enforcement active, bypass actors empty, current user cannot bypass, pull request rule present, required status checks present, and strict up-to-date policy. The same transcript has `required_approving_review_count: 0`, `require_last_push_approval: false`, and no required reviewers. It also states that classic branch protection returns 404 by design and that rulesets are the enforcement mechanism.

The workflow’s `governance-required-checks-audit` job reads ruleset detail and required status check contexts. That corrected the earlier list-endpoint bug and is real progress. But the fetched workflow/audit logic does not treat zero approving reviews, no last-push approval, no required reviewers, or sole-maintainer self-acceptance as a failure.

GitHub’s own ruleset API defines `required_approving_review_count` as the number of approving reviews required before a PR can merge, and `require_last_push_approval` as requiring approval by someone other than the latest pusher. GitHub’s ruleset documentation says required status checks must pass before merge. Those checks enforce machine status, not independent human/adversarial acceptance.

**Conformance verdict.** Governance gap. Status-check merge enforcement is partially present. Independent acceptance is not.

**Required remediation.** Code/process home: `.github/workflows/ci.yml`, `ci_workflow_guards.rs`, `acceptance_status_manifest.rs`, and docs `architecture/13`, `execution/10`, template `0003`.

The audit should fail unless foundational conformance PRs either:

- require at least one approving review by a non-author/non-latest-pusher; or
- require last-push approval plus a required reviewer/team rule; or
- explicitly record governance independence as unavailable and compute `NonPass`/`pending-governance` rather than pass.

If the repository’s real-world sole-maintainer status cannot support independent review, the honest artifact can still say “mechanically checked, governance independence pending.” It cannot claim a sound fail-closed acceptance process.

**Anti-regression guard.** Extend `governance-required-checks-audit` to parse `pull_request.parameters.required_approving_review_count`, `require_last_push_approval`, `required_reviewers`, and bypass/current-user fields. Add `ci_workflow_guards.rs` assertions that the audit fails on synthetic 0-approval ruleset JSON. Add status-manifest negative cases where `branch_protection: enforced` plus zero-approval transcript computes non-pass for independent-acceptance-required artifacts.

**Evidence-honesty check.** A ruleset transcript with active status checks is not sufficient evidence for independent acceptance. The artifact must distinguish “merge-required CI” from “independent acceptor constraint.”

### F6-06 — Standing gate remains partly declarative rather than enforced

**Foundational driver.** Execution `10` says pending is not pass, missed mutants/timeouts must be honestly disposed, and source-text guards are alarms rather than proof. Architecture `13` requires validation evidence not to overclaim fingerprint scope.

**Current source/artifact state.** The 0053 acceptance ruleset transcript requires seven status checks: `rustfmt`, `clippy`, `build & test`, `lock-layer gates`, `public-boundary conformance`, `full-surface mutation trigger (lock layer)`, and `mutation shard reconciliation (lock layer)`. It separately states that `mutation in-diff (lock layer)` succeeded in a CI run, but the transcripted required-check list does not include that context.

In `.github/workflows/ci.yml`, the “full-surface mutation trigger” job is a topology/signaling job: it tells maintainers that guarded paths changed and that the full surface should be reconciled. A topology trigger going green is not the same as proving a full mutation perimeter is green. The scheduled/workflow-dispatch mutation reconciliation job is not a normal PR-time proof unless the workflow design makes it PR-blocking with current artifacts. `mutants-in-diff` is closer to an enforceable PR check, but it is not one of the seven required contexts in the 0053 transcript.

**Conformance verdict.** Hardening gap. The standing gate is materially better than before, but the current required-check set can still be satisfied by alarms rather than mutation proof.

**Required remediation.** Code/process home: `.github/workflows/ci.yml`, `ci_workflow_guards.rs`, `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, `mutation_completion_merge.rs`, `acceptance_status_manifest.rs`, and execution `10`.

The required PR contexts for guarded code should include actual in-diff mutation proof or a fail-closed check that refuses guarded changes without a current accepted mutation artifact. The full scheduled campaign remains valuable as standing perimeter reconciliation, but a skipped scheduled job or an informational trigger must not count as PR pass for a code-changing closure artifact.

**Anti-regression guard.** Add a CI topology guard that verifies `mutation in-diff (lock layer)` or an equivalent actual mutation-result gate is in the required ruleset contexts for guarded code. Add manifest parser logic that rejects `mutation_status: killed` unless the artifact cites current in-diff or full campaign evidence with denominator, caught/unviable/missed/timeout counts and baseline-miss reconciliation.

**Evidence-honesty check.** A closure artifact must label “trigger fired” as an alarm, not proof. “Green standing perimeter” requires actual current mutation result disposition; “survivors bounded” cannot be a pass.

### F6-07 — Food-source survivor family is materially addressed but needs stronger public forcing

**Foundational driver.** The food-source residual matters because it is a truth-firewall / actor-known projection issue: a stale or source-less truth path could overwrite source-bearing actor-known knowledge. Foundation `14`, architecture `03`, architecture `10`, and execution `07` govern it. Execution `10` governs mutation survivor honesty.

**Current code state.** The projection function `food_source_fact_supersedes` uses source-bearing and serving-count knowledge to decide replacement; focused tests in `food_source_projection.rs` exercise the source-bearing/source-less replacement semantics and deterministic source-key ordering through `build_embodied_view_model`. `.cargo/mutants.toml` includes `crates/tracewake-core/src/projections.rs` and `crates/tracewake-core/src/epistemics/projection.rs` in the standing perimeter. The 0053 acceptance historically claims focused 9/9 and final standing 0 survivors; that remains historical.

**Conformance verdict.** Present for the named direct behavior; not certified as current mutation closure. No unjustified equivalence claim is made here.

**Required remediation.** Keep the existing projection tests. Add a fixture/TUI or public actor-known path where event/provenance ancestry creates two competing food-source facts and proves the source-bearing later/stronger fact survives into the actor-visible view without reading raw truth. The code home is `projections.rs`, `epistemics/projection.rs`, `food_source_projection.rs`, content fixture tests, and TUI seam/conformance tests.

**Anti-regression guard.** Extend `.cargo/mutants.toml` and `food_source_projection.rs` with exact selected semantic families, then require actual in-diff mutation for changes to projection replacement logic. Do not accept “equivalent mutant” claims without a semantic proof tied to the actor-known output.

**Evidence-honesty check.** Closure is vacuous if it says “the suite killed it historically” without a current run or if it proves only a private helper while the public actor-known path can route around it.

## 6. Residual disposition

### Food-source residual

The named `food_source_fact_supersedes` survivor family no longer looks like an open foundational defect in the static source. The tests directly encode the source-bearing replacement behavior that was missing in the fifth-pass residual, and the function is inside the standing mutation perimeter. The remaining work is forcing quality: close it through a public actor-known or TUI fixture with event/provenance ancestry, then run the existing mutation machinery from a clean baseline. Do not claim current zero survivors until that run is actually executed.

### Governance independence residual

F5-04 must be reopened as a governance independence gap. The 0053 ruleset transcript is enough to support “status checks are intended to be merge-required by ruleset,” not “acceptance is independent.” A 0-approval, last-push-not-required, no-required-reviewer ruleset permits implementer-as-acceptor self-merge after green checks. The acceptance taxonomy should treat this as `pending-governance` for any artifact whose settled intent includes independent acceptance.

## 7. Structural anti-regression / enforced standing gate

The seam reopened because the 0053 gate emphasized topology and self-consistency more than unrepresentability and adversarial forcing.

The load-bearing layers should be:

1. **Compile-time unrepresentability on the production symbols.** Raw bootstrap state, debug authority acquisition, and debug-grade wait receipts must be impossible to construct/read from a downstream crate. Source-text guards only warn about topology drift; they cannot prove authority.

2. **Public-boundary behavior tests.** The negative-fixture pattern is strong when it attacks live public symbols. It failed here because the bootstrap fixture used stale method names and the debug fixture tested direct minting rather than token induction. The next line should add bypass-shaped fixtures.

3. **Real behavior witnesses through the TUI/runtime path.** TUI tests should prove normal wait/continue/render paths receive only actor-legible products, while debug/operator paths require real authority. Screenshots or display strings cannot be the proof by themselves.

4. **Mutation gates that actually block.** `full-surface mutation trigger` is useful as an alarm. The PR-required gate for guarded code should be actual in-diff mutation or an explicit fail-closed proof artifact. Scheduled full-campaign reconciliation is valuable but cannot be silently counted as current PR proof if it did not run for the change.

5. **Acceptance taxonomy as a state machine.** The taxonomy must compute from current evidence artifacts, not status prose. A pass must be impossible if any row is open/pending/routed-forward, any survivor remains, or governance independence is unavailable.

Placement relative to the certification ladder: do not mint a new gate code. This belongs under the existing P0/SPINE/EPI/ORD-LIFE/FIRST-PROOF ladder as a standing enforcement and acceptance-honesty requirement in execution `10`, with architecture `13` owning the artifact contract.

## 8. Process-integrity + fail-closed-taxonomy audit

### Recurrence diagnosis across acceptance artifacts

| Artifact | Verdict wording / disposition shape | Would 0053 taxonomy have blocked the later-overturned overclaim? | Residual hole |
|---|---|---|---|
| 0047 acceptance | Scoped feature accepted; many requirements marked pass; explicit not-latest-main / not-whole-project scope. | Not reliably. The 0053 parser did not exist and is hardcoded to F5 findings. If mechanically required, it would reject for missing block, but it would not know the true 0047 evidence obligations without a separate expected-finding manifest. | Feature-scope pass can still over-rely on display/transcript evidence unless requirements are machine-ingested. |
| 0048 acceptance | Accepted ticket evidence; workspace gates pass; focused mutation ledger. | Not reliably. A status block could be made self-consistent, and the parser would only check field presence. | No semantic validation that named witnesses actually prove the later-found foundational properties. |
| 0050 acceptance | Scoped hardening accepted; records non-green standing perimeter with 48 misses and 1 timeout. | It might reject an explicit green-perimeter claim, but the parser contains a bounded-forcing survivor-pass shape. | Honest survivor disposition can coexist with “accepted” language; taxonomy must distinguish “accepted remediation line” from “foundational pass.” |
| 0051 acceptance | Multiple rows say “Pass with standing miss disposition”; standing mutation evidence records 23 missed mutants. | Not fully. The 0053 mutation mode `non-blocking-bounded-forcing` can allow survivor rows and still compute pass. | Survivor-bearing pass-with-disposition remains structurally possible. |
| 0052 acceptance | Scoped acceptance; public-boundary pass; governance pending/fail; canonical perimeter not green because food-source survives. | It should compute non-pass if governance pending and survivors are represented honestly. But the 0053 parser can be satisfied by later changing scalars without verifying independence semantics. | Ruleset presence/status checks can be laundered into branch-protection pass without independent-acceptance semantics. |
| 0053 acceptance | `overall_result: pass`; all F5 rows closed; branch protection enforced; mutation survivors none; ruleset transcript shows 0 approvals. | It did not block itself. The block is self-consistent but false/incomplete against current public APIs and governance independence. | Self-authored status block, phrase denylist, field-presence evidence, 0-approval governance, no forced adversarial artifact ingestion. |

The process is not fixed; it has moved up one level. Earlier artifacts overclaimed through prose, scoped-pass wording, or honest-but-non-blocking dispositions. 0053 replaces that with a parseable block, but the block is still authored by the same line and accepted by machinery that does not independently derive facts.

### Parser recomputation versus validation

The parser does recompute `overall_result` from constituent status fields. That is real. It rejects some contradictory blocks. But the recomputation is over an author-provided mini-manifest, not over independent evidence. It validates internal consistency, not truth.

The survivor-pass defect is the clearest proof. A manifest with mutation survivors and `mutation_status: non-blocking-bounded-forcing` can pass if survivor rows have `owner`, `reason`, `next_move`, `max_epochs`, and `failing_check`. That is a useful forcing-disposition record, but it must never compute pass for a closure artifact.

### Wording guard completeness

The wording guard is not a closed grammar. It is a denylist of phrases and contextual heuristics. Denylists are appropriate topology alarms but not evidence-honesty proof. A determined or simply different writer can paraphrase “green,” “closed,” “accepted,” “governance proven,” or “historical result carries forward” without matching the exact phrase set. A closed grammar should instead generate or accept only the verdict sentences permitted by the computed state.

### Machinery correctness and mutation coverage

The 0053 acceptance claims parser/wording tests passed and that synthetic negatives exist. This is historical. Static source inspection shows the test intent, but the machinery’s own correctness is not independently proven in a way that kills the defects above. Future closure must mutate the taxonomy functions and include adversarial fixtures for exactly the holes found here: survivor-pass, self-authored evidence strings, stale negative fixture names, zero-approval governance, missing current artifact ingestion, and paraphrased closure.

### Forcing function before merge/archive

The existing CI runs `acceptance_artifact_wording` as part of lock-layer gates. That does not prove that the actual acceptance artifact for the PR is discovered, parsed, and made merge-blocking. The forcing function should be:

- A required PR check that identifies the acceptance artifact(s) changed or proposed for archival/closeout.
- The check parses the exact artifact with the status parser, wording grammar, governance transcript parser, mutation evidence parser, and expected-finding manifest.
- The check fails if no current acceptance artifact is present for a closure PR, if the artifact is not in the expected path, if any status is not pass-eligible, if governance independence is pending, or if mutation evidence is stale/non-green.
- The ruleset audit makes that check required and also proves the ruleset requires independent approval or records governance non-pass.

### Adversarial application to 0053 acceptance

Applied adversarially, the 0053 acceptance should not have been allowed to compute unqualified pass:

- F5-01 says the bootstrap is sealed, but current public APIs still allow raw state/log/projection assembly and the negative fixture targets stale names.
- F5-03 says debug/no-human authority is token-gated, but public debug bind can induce a runtime-minted token.
- F5-04 says governance is closed, but the transcript itself shows zero required approvals and no last-push/required-reviewer independence.
- F5-06 says the taxonomy is fail-closed, but the taxonomy can compute pass with bounded-forcing survivors and does not verify evidence semantics.

The structural laundering vector is: **self-authored status block + field-presence evidence + phrase-denylist wording guard + implementer-as-acceptor + status-check-only ruleset**. The durable close is not more prose; it is independent evidence ingestion plus unrepresentable code boundaries plus governance that the implementer cannot self-satisfy.

## 9. Foundation & documentation determination

### Tier-0 amendment

No Tier-0 amendment is warranted. The foundation already says enough:

- meaningful mutation must be event-caused and replayable;
- content and language/prose cannot create simulation fact;
- actor cognition and embodied views must be holder-known and source-backed;
- debug is non-diegetic and must not leak into embodied surfaces;
- TUI possession is ordinary actor parity, not omniscience;
- validation/evidence must not become decorative.

The code/process defects violate existing doctrine; they do not reveal a constitutional gap.

### Below-foundation doctrine strengthening

Doctrine strengthening is warranted below foundation.

| Home | Substance to own after code closure |
|---|---|
| `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Acceptance artifacts are read models over current evidence, not evidence by themselves. A pass claim requires current exact-commit evidence ingestion, independent acceptance where required, no live survivor/pending rows, and no stronger prose than computed state. |
| `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | The fail-closed taxonomy computes non-pass for survivors, timeouts, pending governance, self-authored-only evidence, missing actual artifact ingestion, and 0-approval independence gaps. Source-text guards are topology alarms only. |
| `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | Replace open prose verdict shapes with a closed grammar keyed to computed status. The status block must be generated or verified from evidence artifacts and expected findings, not authored as free-form certification. |
| `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | After code closure, truth the runtime receipt/debug authority rows: normal one-tick wait cannot expose debug-grade scheduler internals; debug-mode entry requires a real operator authority, not public self-bind. |
| `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` and `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | Truth the bootstrap row after raw aggregate construction is sealed. |
| `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | Add review prompts for stale negative fixtures, public constructor composition, debug token induction, and survivor-pass taxonomy holes. |
| `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Update existing R-27/R-28/R-29 evidence/status rows only. Do not mint a new risk ID. |
| `docs/4-specs/SPEC_LEDGER.md` | Route the next remediation through the normal ledger process. Do not edit archived specs or acceptance artifacts. |

The 0053 doctrine edits did not directly weaken Tier 0, but they are insufficient and in places overly trusting: they blessed “computed pass” over self-authored status and treated status-check ruleset enforcement as if it also proved independent acceptance. The live docs should be truthed only after executable closure exists.

## 10. Recommended closure order

1. **Seal bootstrap authority first.** Remove or token-gate public raw aggregate constructors and public raw bootstrap assembly. Repair negative fixtures to attack current symbols.
2. **Seal normal wait receipts.** Replace public `OneTickAdvanced(WorldAdvanceResult)` with an actor-legible sealed receipt and move full scheduler detail behind debug/operator authority.
3. **Repair debug authority acquisition.** Remove unguarded public debug bind or require an independent operator/debug authority to enter debug mode. Add a bypass-shaped external negative fixture.
4. **Strengthen public-boundary tests.** Add compile-fail fixtures for bootstrap, wait receipt internals, debug token induction, interval/debug conversion, and embodied field mutation using live APIs.
5. **Repair process taxonomy.** Convert parser to expected-finding/evidence-ingestion state machine; make any survivor/pending/governance gap non-pass; replace phrase denylist with closed allowed wording grammar.
6. **Repair governance audit and ruleset.** Require independent approval or last-push/required-reviewer mechanics for foundational closure PRs; otherwise the status block must compute `pending-governance`.
7. **Make mutation proof actually PR-blocking for guarded changes.** Required contexts should include actual in-diff mutation or a fail-closed artifact check; topology trigger remains an alarm.
8. **Harden food-source residual through public actor-known path.** Add fixture/TUI/provenance witness and then run the existing mutation perimeter.
9. **Only then truth docs.** Update architecture/execution/reference/ledger after code and executable witnesses exist; edit no archived artifacts and mint no identifiers.

## 11. Open maintainer decisions

- How to express the cross-crate content validation witness without creating a backwards-compatible alias path. Because `tracewake-content` is a separate crate, pure `pub(crate)` may not be enough; a sealed token/package boundary is likely the cleanest route.
- Whether local TUI debug attach is an operator action available in development builds, a runtime authority given at app launch, or an explicit debug session mode. Any choice must be structurally non-embodied and not mintable through ordinary public commands.
- What independent acceptance means for a sole-maintainer repository. If no independent reviewer exists, the honest result is non-pass/pending-governance for independent-acceptance closure, not a self-accepted pass.
- How fresh a full standing mutation campaign must be for closure over guarded surfaces. The report recommends in-diff PR blocking plus scheduled full reconciliation, but the exact freshness window belongs in execution `10`/CI.

## 12. Self-check

- [x] Verdict covers code conformance, process soundness, governance independence, amendment determination, and whether another iteration is needed.
- [x] Target-state claims use exact-commit URL-fetched files whose paths are in the manifest.
- [x] 0053 `closed`/`pass`, `branch_protection: enforced`, and `mutation_survivors: none` claims are treated as claims and re-checked statically against code/config shape.
- [x] Genuinely present 0053 properties are recorded as present.
- [x] The process-fix itself is audited co-equally: parser recomputation, wording guard, mutation coverage of the guard, forcing before merge/archive, and adversarial application to 0053.
- [x] Governance is re-scrutinized for 0 approvals, self-merge, ruleset detail endpoint, and CI audit scope.
- [x] Food-source is re-verified without unjustified current mutation-result certification.
- [x] Recommendations extend existing negative fixtures, CI guards, mutation config, status parser, wording guard, and docs; no new property-test framework is proposed.
- [x] No archived artifact is edited; no invariant, gate, risk ID, or glossary term is minted.
- [x] Static-survey limits are explicit.
- [x] Repository evidence and external research lanes are kept separate.

## 13. References

### Repository evidence lane

The complete repository evidence lane is the exact URL ledger in section 3. All repository-state claims in this report are grounded in those fetched files only. The uploaded manifest was used only to verify path inventory; the uploaded brief defined the task scope and source-equivalence posture.

### External research lane

These sources sharpened the recommendations but were not used to assert target-repository state:

- Rust Reference, “Visibility and privacy” — public items are externally accessible only if their ancestor modules are accessible, while private items are accessible within the current module and descendants. This supports using Rust privacy/private fields as a real unrepresentability layer.  
  https://doc.rust-lang.org/reference/visibility-and-privacy.html
- Rust Reference, “The `non_exhaustive` attribute” — non-exhaustive indicates future fields/variants may be added; it is not a substitute for sealing constructors/authority tokens.  
  https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
- Fred B. Schneider, “Implementing Fault-Tolerant Services Using the State Machine Approach: A Tutorial” — commands transform state atomically and request ordering/authority matters; useful prior art for event/replay and command-boundary discipline.  
  https://www.cs.cornell.edu/fbs/publications/SMSurvey.pdf
- Temporal documentation, “Event History walkthrough” — persisted event histories recreate workflow state and deterministic command/event mapping is required for replay. Used as external analogy only.  
  https://docs.temporal.io/encyclopedia/event-history/event-history-python
- Andrei Sabelfeld and Andrew C. Myers, “Language-Based Information-Flow Security” — conventional access control does not directly enforce end-to-end information-flow policies. This supports treating public debug-grade receipt payloads as boundary leaks even when the TUI renderer omits them.  
  https://www.cs.cornell.edu/andru/papers/jsac/sm-jsac03.pdf
- GitHub Docs, “REST API endpoints for rules” — ruleset pull-request parameters include `required_approving_review_count` and `require_last_push_approval`.  
  https://docs.github.com/en/rest/repos/rules
- GitHub Docs, “Available rules for rulesets” — required status checks must pass before collaborators can merge changes targeted by the ruleset.  
  https://docs.github.com/en/enterprise-cloud@latest/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets
- GitHub Docs, “Approving a pull request with required reviews” — required-review settings require a specified number of approving reviews before merge.  
  https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/reviewing-changes-in-pull-requests/approving-a-pull-request-with-required-reviews
- cargo-mutants, “Hangs and timeouts” — timeouts are part of mutation result disposition and must not be silently treated as green.  
  https://mutants.rs/timeouts.html
- Victoria Krakovna, “Classifying specification problems as variants of Goodhart’s Law” — useful frame for self-scored acceptance and proxy gaming: optimizing a status block can displace the underlying conformance goal.  
  https://vkrakovna.wordpress.com/2019/08/19/classifying-specification-problems-as-variants-of-goodharts-law/
