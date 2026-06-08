# Spec 0002 — TUI Proof-Surface Hardening and Debug-Quarantine Certification Spec

**Status:** Proposed new active spec-layer file.  
**Target type:** Hardening / anti-contamination certification remediation.  
**Intended repository path:** `docs/4-specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`  
**Deliverable filename:** `0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`  
**Repository analyzed:** `joeloverbeck/tracewake`  
**Commit analyzed:** `bf0e3a0392bb1217caf034281913487aa1644ad4`  
**Commit freshness:** User-supplied target commit only; this spec does not independently verify that the commit is current `main`.  
**Spec posture beneath live doctrine:** This spec operationalizes the live foundation, architecture, execution, and reference tiers. It does not amend constitutional invariants, define execution gate semantics, weaken acceptance gates, promote archived specs into live authority, or certify implementation outside the scoped evidence named here.  
**Single declared execution admissibility posture:** `P0-CERT scoped remediation`.

## 0. Baseline statement and source discipline

This spec analyzes the TUI / view-model / debug seam as it exists at commit `bf0e3a0392bb1217caf034281913487aa1644ad4`. The manifest used for this audit is path inventory only. Every repository file cited below was fetched by exact commit URL from `joeloverbeck/tracewake` at that commit. Branch names, default-branch lookup, repository metadata, connector namespace labels, code-search snippets, and prior chat context are not evidence.

Archived implementation specs are historical evidence only. Their internal historical baseline commits are not the target of this spec. The relevant archived-spec intent being hardened is: a TUI may be crude but must not be fake; the TUI must consume view models, submit stable semantic commands, show why-not, keep debug non-diegetic, avoid direct mutation, and preserve deterministic scripted stdin/stdout play.

The declared posture is `P0-CERT scoped remediation` because the seam shows substantial landed discipline but does not yet satisfy the live post-overhaul certification bar. Evidence supports remediation rather than a clean certification:

- The command loop and normal semantic-action path largely route through `TuiApp` → current view → `Proposal` → `run_pipeline`, and the renderer consumes view models rather than raw state (`crates/tracewake-tui/src/app.rs:183-242`; `crates/tracewake-tui/src/render.rs:1-29`).
- Debug panels are visibly marked non-diegetic and tests cover basic read-only/non-leakage behavior (`crates/tracewake-tui/src/debug_panels.rs:10-37`; `crates/tracewake-tui/tests/tui_acceptance.rs:35-50`).
- However, embodied view models are still built from raw `PhysicalState` / `AgentState` without a sealed holder-known context packet or typed context hash (`crates/tracewake-tui/src/app.rs:151-176`; `crates/tracewake-core/src/projections.rs:212-235`).
- Action availability still contains string-heavy `why_disabled` and some action construction walks raw state directly (`crates/tracewake-core/src/view_models.rs:173-181`; `crates/tracewake-core/src/projections.rs:500-590`).
- Human proposals carry a source view-model ID but not a holder-known context ID/hash or context ancestry (`crates/tracewake-core/src/actions/proposal.rs:15-25`; `crates/tracewake-core/src/projections.rs:806-850`).
- The executable `wait` shortcut hard-codes a semantic action ID instead of resolving through the current view's affordance list (`crates/tracewake-tui/src/run.rs:10-12`, `crates/tracewake-tui/src/run.rs:60-63`).
- Debug separation is enforced mostly by public `debug_only: bool` fields, markers, assertions, and tests, not by an unforgeable capability/type boundary (`crates/tracewake-core/src/view_models.rs:217-320`; `crates/tracewake-tui/src/debug_panels.rs:12-80`).
- Several tests use display-string and source-substring checks as authority boundaries, which the live execution diagnostics doctrine explicitly rejects (`crates/tracewake-tui/tests/embodied_flow.rs:63-76`; `crates/tracewake-tui/tests/tui_acceptance.rs:290-297`).

## 1. Authority chain and cross-reference gate mapping

### 1.1 Controlling authority chain

This spec is subordinate to the live documentation layering rule: foundation outranks architecture, architecture outranks execution, execution outranks active specs, and archived specs are evidence rather than live authority (`docs/README.md:3-14`; `docs/README.md:43-52`). The spec layer may only operationalize higher-tier doctrine and must declare one admissibility posture, use holder-known terminology precisely, keep archived specs historical, and preserve exact-commit source discipline (`docs/4-specs/README.md:28-38`; `docs/4-specs/SPEC_LEDGER.md:5-23`).

Load-bearing doctrine for this spec:

- **Foundation:** `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`, `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`, `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`, and `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`.
- **Architecture:** `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`, `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`, `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`, and `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`.
- **Execution:** `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`, `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`, `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`, `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`, `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`, and `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`.
- **Reference:** `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`, `01_DESIGN_RISK_REGISTER.md`, and `02_GLOSSARY.md`.
- **Spec tier:** `docs/4-specs/SPEC_LEDGER.md`, `docs/4-specs/README.md`, and the live sibling structure of `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`.
- **Archived intent only:** `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` and `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md`.

### 1.2 Gate-code mapping, as cross-references only

This spec names execution gate codes only as cross-references. It does not define them.

- The TUI / debug split, action pipeline, replay, transcript, and no-direct-dispatch portions of this hardening contribute scoped evidence toward the execution tier's `SPINE-CERT` cross-reference.
- The actor-known view-model filtering, possession parity, notebook/provenance, why-not split, and debug-quarantine portions contribute scoped evidence toward the execution tier's `EPI-CERT` cross-reference.
- The declared spec posture is `P0-CERT scoped remediation` because the next known execution move remains `P0-CERT`, and this spec remediates a concrete seam named by the execution audit boundary (`docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md:65-86`; `docs/4-specs/SPEC_LEDGER.md:51-53`).

## 2. Scope and non-goals

### 2.1 Scope

This spec hardens the boundary where:

1. a possessed controller chooses an embodied actor input;
2. core/projection code constructs actor-known embodied view models and safe why-not surfaces;
3. the TUI renders those view models and submits semantic actions;
4. proposals enter the shared validation/scheduling/event pipeline;
5. debug panels expose non-diegetic truth without affecting embodied cognition, affordances, notebooks, proposals, or transcript acceptance;
6. tests prove the seam using typed artifacts, adversarial cases, and deterministic replay/transcripts.

“Proof-surface legibility” means the string/stdout TUI must make the system inspectable and honest: actor-visible why-not, causal/provenance surfacing, actor-known/not-known boundaries, confidence/staleness, and debug quarantine. It does **not** mean panes, full-screen redraw, key-driven navigation, terminal styling, or a richer graphical client.

### 2.2 Non-goals

This spec does not:

- add world mechanics;
- re-spec epistemics, ordinary life, institutions, no-human simulation, LLM speech, graphical clients, or Phase 4 work;
- introduce a second rendering engine;
- authorize `ratatui` or `crossterm` runtime dependencies;
- preserve backwards-compatibility shims, alias paths, or transitional duplicate APIs;
- treat archived implementation specs as certification;
- rely on display strings as proof of safety;
- allow the TUI, renderer, input parser, or debug panel code to become a simulation-rule layer.

## 3. Audit findings

All file/line citations refer to `joeloverbeck/tracewake` at commit `bf0e3a0392bb1217caf034281913487aa1644ad4`.

### 3.1 Doctrine-bearing invariant map

The audit dimensions below bear primarily on these constitutional invariants:

- `INV-004`, `INV-005`, `INV-006`, `INV-094`, `INV-108`: possession is ordinary input binding only; no special protagonist state, knowledge transfer, need/intention reset, or player privilege (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:19-35`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:395-413`).
- `INV-008`, `INV-009`, `INV-069`: UI assistance is not authority; meaningful state changes require events; the TUI must not implement simulation rules (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:41-49`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:281-309`).
- `INV-017`, `INV-018`, `INV-092`: determinism, replay, and transcript stability (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:79-85`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:395-413`).
- `INV-065`, `INV-066`, `INV-067`, `INV-068`, `INV-070`, `INV-071`: TUI-first proof surface, actor-known embodied mode, non-diegetic debug, mandatory why-not, and no hidden mechanics masquerading as complete play (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:281-309`).
- `INV-093`, `INV-099`, `INV-100`, `INV-101`, `INV-102`, `INV-105`, `INV-106`, `INV-107`: actor-knowledge leakage as high severity; truth may validate but not plan/render; sealed holder-known contexts; provenance; typed diagnostics; non-leaking failure feedback; debug quarantine (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:395-413`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:431-438`; `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md:60-126`; `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md:185-220`).

### 3.2 Findings table

| ID | Status | Responsible layer | Invariants | Evidence | Finding |
|---|---|---|---|---|---|
| F-01 | Already satisfied, keep | TUI dependency boundary | `INV-008`, `INV-065`, `INV-069` | `crates/tracewake-tui/Cargo.toml:7-9`; archived intent `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md:297-302` | The TUI crate currently depends only on `tracewake-core` and `tracewake-content`; no runtime terminal framework dependency exists. This supports the current proof-surface focus and headless stdin/stdout tests. |
| F-02 | Already satisfied, keep | TUI rendering | `INV-008`, `INV-067`, `INV-069`, `INV-070` | `crates/tracewake-tui/src/render.rs:1-29`; `crates/tracewake-core/src/view_models.rs:15-35`; `crates/tracewake-core/src/view_models.rs:54-95` | `render_embodied_view` consumes `EmbodiedViewModel` and `NotebookView`; it does not accept `PhysicalState` or `AgentState`. It also renders typed-ish why-not reason codes and a knowledge-context line when present. |
| F-03 | Already satisfied, keep | TUI input parsing | `INV-008`, `INV-069` | `crates/tracewake-tui/src/input.rs:49-95`; `crates/tracewake-tui/src/input.rs:97-138` | The parser classifies commands, IDs, numeric selections, and debug command vocabulary. It does not inspect world state or decide legality. |
| F-04 | Already satisfied, keep | Menu identity / semantic action selection | `INV-008`, `INV-069` | `crates/tracewake-tui/src/run.rs:54-58`; `crates/tracewake-tui/tests/tui_acceptance.rs:10-24` | Numeric selection resolves through the current view's `semantic_actions` and returns a stable `SemanticActionId`; the menu index is not action identity. |
| F-05 | Already satisfied with hardening required around context IDs | Pipeline / no-direct-dispatch | `INV-008`, `INV-009`, `INV-069`, `INV-099` | `crates/tracewake-tui/src/app.rs:183-242`; `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md:287-295` | The normal semantic-action path selects an entry from the current view, builds a proposal, constructs a pipeline context, and calls `run_pipeline`. This preserves the historical no-direct-dispatch intent for ordinary actions. It still lacks holder-known context identity in the proposal. |
| F-06 | Needs hardening | View-model construction / holder-known context | `INV-067`, `INV-069`, `INV-093`, `INV-099`, `INV-100`, `INV-101`, `INV-102` | `crates/tracewake-tui/src/app.rs:151-176`; `crates/tracewake-core/src/projections.rs:212-235`; `crates/tracewake-core/src/epistemics/knowledge_context.rs:37-93`; `crates/tracewake-core/src/agent/actor_known.rs:185-299`; `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:31-61`; `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:93-98` | Embodied views are built directly from raw physical and agent state via `build_embodied_view_model_with_agent_state` and then decorated with a formatted `knowledge.<actor>.<tick>` string. A `KnowledgeContext` type (`epistemics::KnowledgeContext`, with `ViewMode`, typed allowed/forbidden source enums, and scope filters) and an agent-side `ActorKnownPlanningContext` already exist, but the embodied view-model path constructs `KnowledgeContext::embodied()` only to scope the notebook (`app.rs:166-175`) — the affordance, why-not, and other view fields still derive from raw state, and the context identity is flattened to a display string. Neither precursor yet carries the full sealed packet the architecture requires (context hash, provenance edges, forbidden-truth audit result, holder/bound-actor identity, frontier). The hardening must complete and route through these existing types, not a display string or ad hoc context ID. |
| F-07 | Needs hardening | Projection / action availability | `INV-067`, `INV-069`, `INV-070`, `INV-093`, `INV-100`, `INV-102`, `INV-105` | `crates/tracewake-core/src/projections.rs:310-333`; `crates/tracewake-core/src/projections.rs:500-590`; `crates/tracewake-core/src/projections.rs:757-803` | Semantic actions and Phase 3A affordances are generated with access to raw state/agent state. Preflight validation is useful, but availability and disabled reasons are not yet tied to a sealed holder-known provenance packet. |
| F-08 | Needs hardening | View-model schema | `INV-069`, `INV-070`, `INV-105`, `INV-106` | `crates/tracewake-core/src/view_models.rs:173-181`; `crates/tracewake-core/src/actions/report.rs:80-112` | `SemanticActionEntry.why_disabled` and `CheckedFact` remain string-heavy. `ReasonCode` is typed, but facts, disabled reasons, and action provenance are not fully typed or source-bound. |
| F-09 | Needs hardening | Proposal construction / validation provenance | `INV-008`, `INV-009`, `INV-099`, `INV-101`, `INV-102` | `crates/tracewake-core/src/actions/proposal.rs:15-25`; `crates/tracewake-core/src/projections.rs:806-850`; `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md:44-67` | Proposals include `source_view_model_id` but not holder-known context ID/hash, context ancestry, or provenance/visibility proof. A forged, stale, or privileged action ID can be rejected by validation, but the proposal type does not structurally prove that it came from the current actor-known affordance surface. |
| F-10 | Needs hardening | TUI command loop | `INV-008`, `INV-069`, `INV-070`, `INV-099` | `crates/tracewake-tui/src/run.rs:10-12`; `crates/tracewake-tui/src/run.rs:60-63`; archived intent `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md:247-252` | The `wait` shortcut hard-codes `wait.1_tick` instead of resolving the semantic action from the current view. It still enters `submit_semantic_action`, but the shortcut weakens the rule that the TUI submits only currently surfaced affordances. |
| F-11 | Needs hardening / quarantine classification | TUI operator command / no-human run | `INV-004`, `INV-008`, `INV-009`, `INV-068`, `INV-091`, `INV-107` | `crates/tracewake-tui/src/input.rs:12-14`; `crates/tracewake-tui/src/run.rs:64-77`; `crates/tracewake-tui/src/app.rs:249-264` | `run no-human-day` is exposed as a top-level command that advances simulation and then renders a debug panel plus embodied view. It is valuable for proof, but it must be classified and tested as non-diegetic operator/proof tooling, not as an embodied player action. |
| F-12 | Already satisfied but structurally weak | Debug rendering | `INV-068`, `INV-093`, `INV-107` | `crates/tracewake-tui/src/debug_panels.rs:10-80`; `crates/tracewake-tui/src/debug_panels.rs:122-238`; `crates/tracewake-tui/tests/tui_acceptance.rs:35-50`; `crates/tracewake-tui/tests/tui_acceptance.rs:52-109` | Debug panels carry `DEBUG NON-DIEGETIC` markers and tests assert read-only behavior. The boundary is still implemented with public `debug_only: bool` fields and runtime assertions rather than unforgeable debug-capability types. |
| F-13 | Needs hardening | Debug view-model schema | `INV-068`, `INV-093`, `INV-107` | `crates/tracewake-core/src/view_models.rs:217-320`; `crates/tracewake-tui/src/app.rs:374-415` | Debug views are distinct structs, which is good, but their quarantine relies on public booleans and marker strings. Any code can construct a falsely marked debug view unless constructors/capabilities are sealed. |
| F-14 | Needs hardening | Notebook / leads / provenance | `INV-067`, `INV-070`, `INV-093`, `INV-102`, `INV-105` | `crates/tracewake-core/src/projections.rs:390-447`; `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md:197-234` | Notebook rendering is actor-scoped and source-bound, but `possible_leads` are derived by testing whether a rendered belief summary contains the phrase `missing from expected location`. Display text is acting as a semantic classifier. |
| F-15 | Already satisfied with test upgrade required | Deterministic transcript/replay | `INV-017`, `INV-018`, `INV-092` | `crates/tracewake-tui/tests/tui_acceptance.rs:26-33`; `crates/tracewake-tui/tests/transcript_snapshot.rs:6-33`; `crates/tracewake-core/src/replay/rebuild.rs:45-91`; `crates/tracewake-core/src/replay/rebuild.rs:491-540` | The seam has transcript determinism tests and replay rebuild/checksum tests. Hardening must keep this path headless and deterministic and add adversarial transcript tests around debug/possession/stale proposals. |
| F-16 | Needs hardening | Tests / diagnostics | `INV-093`, `INV-105`, `INV-106` | `crates/tracewake-tui/tests/embodied_flow.rs:63-76`; `crates/tracewake-tui/tests/tui_acceptance.rs:290-297`; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:38-52`; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:135-144` | The current tests include substring guards against `apply_event`, culprit words, and debug-only text. These are useful smoke tests but are not sufficient proof under the typed-diagnostic/adversarial-gate doctrine. |
| F-17 | Already satisfied, keep | Possession input binding | `INV-004`, `INV-005`, `INV-006`, `INV-094`, `INV-108` | `crates/tracewake-tui/src/app.rs:135-148`; `crates/tracewake-core/src/controller.rs:40-73`; `crates/tracewake-core/src/controller.rs:239-300`; `crates/tracewake-tui/tests/tui_acceptance.rs:299-330` | Binding attaches a controller and records controller metadata without physical checksum mutation. Tests cover deterministic possession continuation. Hardening must additionally prove no notebook/need/intention/knowledge transfer between bodies. |
| F-18 | Already satisfied, keep | Archived 0002/0003 intent | `INV-008`, `INV-009`, `INV-065`, `INV-066`, `INV-068`, `INV-069`, `INV-070` | `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md:150-157`; `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md:332-338`; `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md:239-302` | The landed intent remains correct: crude string TUI, stable semantic actions, why-not, debug separation, no direct mutation, and deterministic command loop. This spec hardens that intent under the live post-overhaul doctrine rather than rewriting the product direction. |

## 4. Hardening requirements and anti-contamination checks

The following requirements are durable implementation constraints. Their IDs are local spec requirement IDs, not execution gate definitions.

### TUI-AC-001 — Seal and complete the holder-known context packet for embodied view construction

**Invariants enforced:** `INV-067`, `INV-069`, `INV-093`, `INV-099`, `INV-100`, `INV-101`, `INV-102`.  
**Prevents:** embodied views, affordances, why-not, or notebook leads being built directly from privileged physical truth or debug projection state.  
**Requirement:** Embodied view-model construction must take a sealed holder-known context packet, not raw `PhysicalState` / `AgentState` / `EpistemicProjection` directly at the public TUI-consumed boundary. This must **extend the existing `epistemics::KnowledgeContext`** (`crates/tracewake-core/src/epistemics/knowledge_context.rs`) into the full sealed packet below and route view-model construction and proposals through it — and stay consistent with the agent-side sealed context `agent::ActorKnownPlanningContext` (`crates/tracewake-core/src/agent/actor_known.rs`) rather than introducing a third parallel notion. Per §9, no duplicate or alias packet type may be added alongside these.

The packet must include, at minimum:

- stable `holder_known_context_id`;
- deterministic context hash over canonical, actor-known inputs;
- holder identity and bound actor identity;
- tick / event frontier;
- provenance entries for perceptions, memories, observations, records, beliefs, reservations, and relevant action affordance facts;
- forbidden-truth audit result proving no debug truth, fixture truth, raw omniscient item location, hidden actor motive, or player memory entered the packet;
- stale/invalid status used by proposal validation.

**Structural enforcement:**

- Keep the packet constructor in core (extend `epistemics::KnowledgeContext`'s `embodied()`/`debug()` constructors), not in `tracewake-tui`.
- Keep packet fields that would allow forged provenance private behind constructors and typed accessors.
- Remove or narrow the public TUI-facing APIs that build embodied view models from raw state. All three current public builders must be addressed: `build_embodied_view_model` (`crates/tracewake-core/src/projections.rs:193`), `build_embodied_view_model_with_agent_state` (`:212`), and `build_embodied_view_model_with_notebook` (`:363`). Existing test-only helpers may exist only behind explicit test-only modules that still produce sealed packets.
- `EmbodiedViewModel` must carry `holder_known_context_id` and `holder_known_context_hash` as typed IDs, replacing the current `knowledge_context_id: Option<String>` (`crates/tracewake-core/src/view_models.rs:33`).

**Current evidence:** the raw-state build path is public and used by `TuiApp::current_view`, which builds the view from raw state and then constructs `KnowledgeContext::embodied()` only to scope the notebook while flattening context identity to a `knowledge.<actor>.<tick>` display string (`crates/tracewake-tui/src/app.rs:151-176`; `crates/tracewake-core/src/projections.rs:212-235`; `crates/tracewake-core/src/epistemics/knowledge_context.rs:51-93`).  
**Acceptance test:** an adversarial test attempts to build an embodied view from raw state in the TUI crate. The attempt must fail at compile time or through an explicit forbidden API boundary. A positive test proves the same rendered information is available through a sealed actor-known packet.

### TUI-AC-002 — Actor-visible affordances must be derived only from the sealed context

**Invariants enforced:** `INV-008`, `INV-067`, `INV-069`, `INV-070`, `INV-093`, `INV-100`, `INV-102`.  
**Prevents:** TUI-visible action lists being generated from hidden truth and merely visually filtered afterward.  
**Requirement:** `SemanticActionEntry` lists in embodied view models must be computed from sealed context facts plus action-definition preflight rules that do not reveal forbidden truth. Raw-state validation may still reject proposals, but it may not be the source of actor-visible target discovery or actor-visible action availability.

**Structural enforcement:**

- The actor-visible affordance builder receives a `HolderKnownContextRef` or equivalent sealed read-only context and an action registry view filtered to known/reachable candidates.
- Preflight validation may use truth only to classify an actor-safe availability result; its input/output must carry typed reason codes and provenance references, not display strings.
- Phase 3A affordances currently riding on the TUI seam must be refactored so hunger/work/routine action visibility is context-derived and provenance-bearing. This spec does not re-spec ordinary-life mechanics; it hardens their TUI boundary.

**Current evidence:** current Phase 3A action creation walks raw `state.food_supplies`, containers, workplaces, and `agent_state.active_intention_by_actor` (`crates/tracewake-core/src/projections.rs:500-590`).  
**Acceptance test:** a hidden-food fixture must prove no action ID, label, disabled reason, or target ID for hidden food can appear in the embodied view unless the actor-known context contains a valid source-bound observation/record that justifies it.

### TUI-AC-003 — Replace string disabled reasons with typed availability and typed actor-visible why-not

**Invariants enforced:** `INV-070`, `INV-105`, `INV-106`.  
**Prevents:** display strings becoming implicit diagnostics, leaking hidden truth, or being parsed by tests as authority.  
**Requirement:** `SemanticActionEntry` must not expose `why_disabled: Option<String>` as the canonical availability reason. It must carry a typed `ActionAvailability` structure with stable reason codes, actor-visible safe summary, provenance references, and optional debug-only diagnostics accessible only through debug report construction.

**Structural enforcement:**

- Keep display labels/rendered summaries as terminal text only.
- Tests assert typed reason codes and provenance objects, not substrings.
- Renderers may format typed reasons, but formatting must not be used by validation, tests, affordance selection, or notebook lead generation.

**Current evidence:** `SemanticActionEntry` exposes `why_disabled: Option<String>` (`crates/tracewake-core/src/view_models.rs:173-181`), while validation reports have typed `ReasonCode` but string `CheckedFact` and string summaries (`crates/tracewake-core/src/actions/report.rs:80-112`).  
**Acceptance test:** a rejected action must expose typed `WhyNotFailureKind`, stable `ReasonCode`, source context ID/hash, and actor-safe summary. A negative test mutates display text without changing typed codes and confirms semantic assertions still pass.

### TUI-AC-004 — Proposals must carry source context identity and be rejected when stale, forged, or privileged

**Invariants enforced:** `INV-008`, `INV-009`, `INV-069`, `INV-099`, `INV-101`, `INV-102`.  
**Prevents:** forged proposals, stale view models, debug-selected targets, or hidden-rule-inferred action IDs from entering the pipeline as if they were embodied actor choices.  
**Requirement:** Every world-affecting human/TUI proposal must carry the `source_view_model_id`, `holder_known_context_id`, `holder_known_context_hash`, tick/event frontier, semantic action ID, and provenance ancestry for the selected action entry.

**Structural enforcement:**

- Extend `Proposal` with typed context fields and a typed `ProposalSource` enum.
- `proposal_from_semantic_action_entry` must consume an action-entry token that cannot be constructed outside the view-model/affordance builder.
- Validation must check actor binding, source context freshness, semantic action membership, and provenance before action-specific validation.
- Rejections for stale/forged/privileged source must use typed reason codes and actor-safe summaries.

**Current evidence:** `Proposal` has `source_view_model_id` but no context/hash/provenance fields (`crates/tracewake-core/src/actions/proposal.rs:15-25`), and proposal construction copies target IDs and parameters from the entry (`crates/tracewake-core/src/projections.rs:806-850`).  
**Acceptance test:** try to submit an action ID from another actor's view, a debug-only target, a stale view, and a hand-constructed target ID. Each must reject before mutation with typed source/context failure and no hidden-truth detail in actor-visible text.

### TUI-AC-005 — The command loop must submit only current-view semantic actions for embodied commands

**Invariants enforced:** `INV-008`, `INV-069`, `INV-070`, `INV-099`.  
**Prevents:** convenience commands bypassing the actor-visible affordance surface.  
**Requirement:** Every embodied world-affecting command, including `wait`/`w`, must resolve through the current view's typed `semantic_actions` list or a typed actor-safe command alias that maps to a visible current-view action token.

**Structural enforcement:**

- Remove the hard-coded `WAIT_ACTION_ID` shortcut from `run.rs`, or make it a lookup key that must find exactly one current-view action entry.
- Keep `do <semantic_action_id>` legal only if that ID exists in the current view and matches the context hash.
- Keep numeric selection as presentation only.

**Current evidence:** numeric selection resolves through the current view (`crates/tracewake-tui/src/run.rs:54-58`), but `wait` constructs `wait.1_tick` directly (`crates/tracewake-tui/src/run.rs:10-12`; `crates/tracewake-tui/src/run.rs:60-63`).  
**Acceptance test:** remove or disable wait from a fixture's current view and prove `wait` cannot execute. Then restore a view-surfaced wait action and prove `wait` executes through the same proposal/context path as numeric and `do` selection.

### TUI-AC-006 — Quarantine no-human execution as non-diegetic operator/proof tooling

**Invariants enforced:** `INV-004`, `INV-008`, `INV-009`, `INV-068`, `INV-091`, `INV-107`.  
**Prevents:** a proof/debug command being mistaken for embodied player agency or actor cognition.  
**Requirement:** `run no-human-day` must be typed as an operator/proof command, not an embodied action. Its output must be debug/non-diegetic; it must not reuse actor-visible why-not, notebook, or affordance paths as evidence that the actor knows anything.

**Structural enforcement:**

- Move the command under explicit debug/operator command classification or require an `OperatorProofCommand` type.
- Render the result only through debug-panel/proof-report types carrying a sealed debug capability.
- After no-human execution, the next embodied view must be built from the current bound actor's sealed context, not from debug report rows.

**Current evidence:** the parser exposes `RunNoHumanDay` alongside embodied commands (`crates/tracewake-tui/src/input.rs:4-16`, `crates/tracewake-tui/src/input.rs:66-70`), and the loop mutates simulation through `app.run_no_human_day()` before rendering a debug panel and embodied view (`crates/tracewake-tui/src/run.rs:64-77`; `crates/tracewake-tui/src/app.rs:249-264`).  
**Acceptance test:** transcript and type-level tests must prove no actor-visible notebook, action, or why-not output cites no-human debug metrics as actor knowledge unless a later ordinary event/record made it actor-known.

### TUI-AC-007 — Replace public debug booleans with sealed debug capability types

**Invariants enforced:** `INV-068`, `INV-093`, `INV-107`.  
**Prevents:** false debug markers, accidental debug-to-embodied struct reuse, or tests treating markers as the boundary.  
**Requirement:** Debug reports/views that may contain privileged truth must be constructible only with a debug capability generated by core debug/report APIs. Public `debug_only: bool` and marker strings may remain rendered output, but they must not be the authority boundary.

**Structural enforcement:**

- Introduce a `DebugCapability` / `NonDiegeticDebugContext` type with private fields.
- Make privileged debug view constructors private to core/debug modules or require the capability parameter.
- Remove public mutable `debug_only` authority fields from view schemas or make them derived render metadata.
- Renderer functions may accept only debug view/report types, not raw state.

**Current evidence:** debug view structs expose public `debug_only: bool` and `non_diegetic_marker: String` fields (`crates/tracewake-core/src/view_models.rs:217-320`), and debug renderers assert those fields at runtime (`crates/tracewake-tui/src/debug_panels.rs:12-80`; `crates/tracewake-tui/src/debug_panels.rs:122-238`).  
**Acceptance test:** a compile-fail or integration test attempts to construct a fake debug view/report outside debug construction APIs. It must fail or be impossible. A positive test proves real debug reports still render with `DEBUG NON-DIEGETIC`.

### TUI-AC-008 — Notebook and leads must be typed, source-bound, and actor-known

**Invariants enforced:** `INV-067`, `INV-070`, `INV-093`, `INV-102`, `INV-105`.  
**Prevents:** display prose becoming a lead classifier or leaking culprit/ground-truth inference into actor notes.  
**Requirement:** Notebook beliefs, observations, contradictions, and possible leads must be typed actor-known entries with source references, confidence/staleness, and explicit “how this may be wrong” affordances. `possible_leads` must not be derived from substring checks over rendered belief summaries.

**Structural enforcement:**

- Add typed lead/notice/provenance structures to notebook view models.
- Generate leads from typed contradiction/proposition kinds and source refs, not display text.
- Render “source”, “confidence”, “staleness”, and “possible next actor-known actions” from typed fields.

**Current evidence:** notebook entries include source and confidence strings, but `possible_leads` are derived with `belief.summary.contains("missing from expected location")` (`crates/tracewake-core/src/projections.rs:390-447`).  
**Acceptance test:** changing the wording of the missing-property belief summary must not alter whether a lead exists; changing the typed proposition/contradiction kind must.

### TUI-AC-009 — Why-not must remain two-layer and non-leaking

**Invariants enforced:** `INV-070`, `INV-093`, `INV-105`, `INV-106`, `INV-107`.  
**Prevents:** rejection messages revealing hidden truth, exact hidden targets, rule internals, or debug validation facts.  
**Requirement:** Actor-visible why-not must be a safe projection of typed validation results over actor-known context. Debug why-not must be separately rendered in non-diegetic debug panels. Actor-facing why-not may say what the actor can tell and what is uncertain; it must not disclose hidden contents, hidden routes, culprit identity, exact hidden food/source, or validation-only truth.

**Structural enforcement:**

- `ValidationReport` should carry typed actor-visible facts and typed debug facts separately.
- Actor-visible render path can access only the safe subset.
- Debug render path requires debug capability.
- Tests assert typed fact classifications and forbidden-provenance audits, not only forbidden substrings.

**Current evidence:** `WhyNotView` maps from `ValidationReport` and reason codes (`crates/tracewake-core/src/view_models.rs:54-95`), but actor/debug summaries and checked facts are strings (`crates/tracewake-core/src/actions/report.rs:80-112`).  
**Acceptance test:** closed-container, hidden-route, hidden-food, wrong-suspicion, and stale-proposal fixtures must reject actions with actor-safe reason codes and must expose the validation-only truth only in debug views.

### TUI-AC-010 — Deterministic, typed transcript and replay proof must survive rendering changes

**Invariants enforced:** `INV-017`, `INV-018`, `INV-092`, `INV-095`.  
**Prevents:** terminal timing, unordered collections, display-label churn, or debug side effects changing outcomes or test evidence.  
**Requirement:** TUI transcripts remain deterministic, but acceptance must be based on typed view-model/proposal/report/replay artifacts first and rendered transcript snapshots second.

**Structural enforcement:**

- Use ordered collections or canonical sorting for all rendered-for-test lists.
- Keep stdout/scripted runner available even if a later visual renderer is added.
- Ensure debug panels cannot mutate event log, physical checksum, agent checksum, controller binding except explicit bind, or embodied context.

**Current evidence:** deterministic transcript tests exist (`crates/tracewake-tui/tests/tui_acceptance.rs:26-33`; `crates/tracewake-tui/tests/transcript_snapshot.rs:6-33`), and replay rebuild applies ordered event streams and validates checksums (`crates/tracewake-core/src/replay/rebuild.rs:45-91`; `crates/tracewake-core/src/replay/rebuild.rs:491-540`).  
**Acceptance test:** repeat representative embodied/debug/possession/no-human transcripts twice and require identical typed artifacts plus stable rendered transcript. Separately replay from initial fixture plus log and require matching physical and agent checksums.

### TUI-AC-011 — Tests must replace string/source-substring authority with typed adversarial gates

**Invariants enforced:** `INV-093`, `INV-105`, `INV-106`, `INV-107`.  
**Prevents:** friendly golden-path tests passing while the implementation remains structurally contaminable.  
**Requirement:** Existing substring/source scans may remain as smoke tests, but they cannot be the primary proof. New tests must inspect typed view models, typed provenance, typed report reasons, context hashes, and compile-time boundaries.

**Structural enforcement:**

- Replace `include_str!(...).contains("apply_event")` style no-direct-dispatch proof with a module-visibility/dependency or compile-fail boundary where practical.
- Replace forbidden-word culprit tests with typed actor-known/provenance assertions plus a small rendered smoke check.
- Add adversarial tests that intentionally try to exploit debug truth, forged proposals, stale views, possession changes, and invalid terminal input.

**Current evidence:** source and display-string tests exist (`crates/tracewake-tui/tests/embodied_flow.rs:63-76`; `crates/tracewake-tui/tests/tui_acceptance.rs:290-297`), while execution testing doctrine calls for typed responsible layers, typed diagnostics, and adversarial gates (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:38-52`; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:135-144`).  
**Acceptance test:** a review artifact must list each anti-contamination check by responsible layer, fixture, typed IDs checked, and failure mode.

### TUI-AC-012 — Possession must prove input-binding-only across view, notebook, needs, intentions, and debug

**Invariants enforced:** `INV-004`, `INV-005`, `INV-006`, `INV-094`, `INV-108`.  
**Prevents:** controller binding transferring knowledge, needs, intentions, memory, or action affordances across bodies.  
**Requirement:** Possession changes only the controller input binding and the embodied actor whose sealed context is selected. It must not copy notebook entries, belief provenance, needs, active intentions, routine state, or debug learnings across actors.

**Structural enforcement:**

- Controller binding events remain non-world metadata.
- `TuiApp::current_view` chooses the bound actor and requests that actor's sealed context.
- Context packet identity includes holder/bound actor; proposal validation checks actor/controller match.
- Debug panels opened while bound to actor A do not influence actor B's context after rebind.

**Current evidence:** binding uses controller metadata and tests show no physical checksum change (`crates/tracewake-tui/src/app.rs:135-148`; `crates/tracewake-core/src/controller.rs:40-73`; `crates/tracewake-core/src/controller.rs:239-300`), but broader cross-actor notebook/need/intention transfer tests must be explicit.  
**Acceptance test:** bind actor A, open debug item/projection/replay panels, trigger actor-known notebook updates for A, bind actor B, and assert B's view/notebook/needs/intentions/actions contain only B's own typed context and no A-only knowledge or debug-derived fact.

## 5. Proof-surface legibility requirements

These requirements raise proof-surface legibility over the existing string/stdout renderer. They do not prescribe pane layouts or visual redesign.

### PSL-001 — Actor-visible why-not must explain the actor's situation without teaching hidden truth

The TUI must show, for the last failed embodied proposal:

- typed failure category;
- stable reason code(s);
- actor-visible summary;
- source holder-known context ID/hash;
- provenance summary for facts the actor can know;
- actor-safe uncertainty, such as “you do not know whether…” when the actor lacks knowledge;
- no debug-only exact target discovery.

Doctrine basis: two-layer why-not (`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md:179-195`), actor limitation legibility (`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md:147-177`), and validation report split (`docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md:117-134`).

Acceptance: a hidden-route/closed-container fixture must render an actor-safe why-not with reason codes and context ID/hash, while the debug panel may show validation internals only under a debug capability.

### PSL-002 — View models must surface “who knows what, why, from what source, with what confidence, and how it may be wrong”

Embodied notebook and lead views must show typed entries for:

- holder/actor identity;
- belief/observation/contradiction IDs;
- source refs and source kinds;
- confidence and staleness;
- contradiction/mismatch relations;
- possible actor-known next actions.

The renderer may format these as plain lines. The semantic content must be typed in the view model. Provenance is a known UX concept for assessing reliability/trustworthiness, and this maps directly to Tracewake's source-bound cognition doctrine rather than importing new mechanics.[^prov]

Doctrine basis: actor notebook separation (`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md:197-234`), holder-known provenance (`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:31-91`), and risk control against weak traceability (`docs/3-reference/01_DESIGN_RISK_REGISTER.md:199-201`).

Acceptance: a contradiction fixture must prove a source-bound lead appears from typed contradiction/proposition data, not a text phrase, and must also prove culprit identity remains absent unless actor-known.

### PSL-003 — Causal trace must be visible as actor-safe context and debug-only full trace

Embodied views should expose a short actor-safe context line: context ID/hash, tick, event frontier, and source summary. Debug panels may expose full event/projection/replay lineage, physical/agent checksums, and forbidden-truth audit rows.

Doctrine basis: TUI as proof surface (`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md:5-29`), event-sourced causality/replay (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:41-49`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:79-85`), and observability artifacts (`docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md:21-37`). Event-sourcing prior art supports treating event streams as the record from which state/projections can be reconstructed, matching Tracewake's doctrine that read models are derived rather than authoritative.[^event-sourcing]

Acceptance: a transcript must include actor-safe context identity in embodied mode and debug-only replay/projection details in debug mode, with no actor-visible checksum/hidden-location leak unless explicitly actor-known.

### PSL-004 — Inspectability must be render-tech agnostic

The proof surface must be inspectable through:

- typed view-model assertions;
- typed proposal/report artifacts;
- deterministic transcript snapshots;
- debug-only reports;
- replay rebuild reports.

The string renderer remains the canonical proof renderer for this spec. If a future renderer exists, it must render the same typed view models and keep the same scripted/stdout or headless equivalent test path.

The pure-model/projection direction is aligned with widely used UI architecture patterns in which a view is a function of model state and update logic owns state transitions, not the renderer.[^elm] This is external support only; Tracewake's live architecture remains the authority.

Acceptance: test fixtures assert typed artifacts first, then rendered string snapshots. A renderer text change must not break semantic safety tests unless typed artifacts changed.

## 6. Deferred uplift boundary and render-tech recommendation

### 6.1 Deferral boundary

Visual/render uplift is explicitly deferred. This spec must not design panes, keybindings, live full-screen redraw, terminal layouts, alternate-screen UI, mouse support, or graphical-client flow. Future graphical or richer terminal clients must reuse:

- simulation core;
- action proposal/validation/scheduling/event pipeline;
- sealed holder-known context builders;
- actor-known view models;
- debug/replay architecture;
- typed diagnostics and proof artifacts;
- transcript/headless acceptance doctrine.

Foundation doctrine already frames the TUI as first proof surface and defers future graphical clients to reuse the same boundaries (`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md:297-314`). Architecture likewise requires future clients to inherit the same client-boundary rules (`docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md:104-115`).

### 6.2 Recommendation on `ratatui` / `crossterm`

**Recommendation:** Do not add `ratatui` or `crossterm` runtime dependencies in this spec. Keep the string/stdout renderer and invest the hardening budget in typed view models, sealed holder-known contexts, debug capability separation, typed diagnostics, adversarial tests, deterministic transcripts, and replay evidence.

Reasons:

1. The current hardening target is proof-surface legibility, not visual richness. The historical executable-loop spec explicitly preferred the existing string/stdout renderer and kept `ratatui` / `crossterm` out of scope unless a later spec justified the dependency and test burden (`archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md:297-302`).
2. The current crate has no runtime terminal UI dependencies (`crates/tracewake-tui/Cargo.toml:7-9`), and existing tests already exercise deterministic stdin/stdout transcripts (`crates/tracewake-tui/tests/tui_acceptance.rs:26-33`; `crates/tracewake-tui/tests/transcript_snapshot.rs:6-33`).
3. `ratatui` has a `TestBackend` intended for terminal UI integration tests, so it can be made testable, but adding it now would expand the problem from boundary certification into visual renderer certification.[^ratatui-test] Ratatui backend documentation also makes clear that terminal rendering depends on backend behavior; that is a separate test surface.[^ratatui-backends]
4. `crossterm` provides raw mode and alternate-screen terminal manipulation, which is useful for rich interactive terminals but increases the surface area for input timing, terminal state, and headless-script determinism concerns that this spec is deliberately avoiding.[^crossterm]

A future spec should revisit `ratatui` / `crossterm` only if all of the following are true:

- the sealed holder-known context and debug capability boundary are already implemented and certified for the string renderer;
- the future spec's goal is explicitly visual interaction, not proof-surface hardening;
- the stdout/scripted or equivalent headless proof path remains available;
- semantic tests assert typed view models/proposals/reports independently of terminal frames;
- terminal snapshot tests use a deterministic backend such as `ratatui::backend::TestBackend` or equivalent;
- no outcome, proposal ordering, replay result, or transcript acceptance depends on wall-clock time, terminal frame timing, OS randomness, or event-loop races.

## 7. Required fixtures and tests

The tests below are required for acceptance of the remediation. Existing friendly golden paths may remain but must not be the primary proof. Where a fixture already exists for the scenario (under `crates/tracewake-content/src/fixtures/`), the requirement is to **harden the existing fixture** to the typed/adversarial bar below, not to author a parallel one.

### 7.1 Positive proof fixtures

1. **Embodied action path fixture:** current view → typed semantic action → proposal with context ID/hash → pipeline → event log → projection/replay. Assert typed IDs, report status, event IDs, and checksums. (Extend `ordinary_workday_001` / `view_model_local_actions_001`.)
2. **Actor-safe why-not fixture:** closed door or unreachable target rejects with typed reason codes, actor-safe summary, source context ID/hash, and no hidden target discovery. (Extend `hidden_food_closed_container_001`, `hidden_food_unknown_route_001`, `hidden_route_edge_001`, `door_access_001`.)
3. **Source-bound notebook fixture:** actor observes contradiction; notebook shows belief/observation/source/confidence/staleness/lead from typed provenance. (Extend `expectation_contradiction_001`.)
4. **Debug quarantine fixture:** debug item/replay/projection panels show truth only through debug capability and do not mutate physical state, agent state, event log, or embodied context. (Extend `debug_omniscience_excluded_001`, `debug_attach_001`.)
5. **Possession parity fixture:** rebind across actors preserves each actor's own knowledge, needs, intentions, routine state, and action affordances. (Extend `possession_parity_001`, `possession_does_not_reset_intention_001`.)
6. **Deterministic transcript/replay fixture:** repeated runs produce identical typed artifacts and rendered transcript; replay from fixture + ordered log produces matching physical and agent checksums. (Extend `no_human_day_001` / `replay_item_location_001`.)

### 7.2 Adversarial / negative gates

1. **Debug-truth leakage attempt:** open debug item, debug projection, debug epistemics, debug planner, and debug replay panels; then render embodied view/notebook/actions. Assert typed actor-known context has no debug provenance and rendered smoke checks find no forbidden hidden truth.
2. **Forged privileged-view attempt:** manually construct or reuse a `SemanticActionId`/target from another actor, a hidden item, or a debug panel. The pipeline must reject before mutation with a typed source/context failure.
3. **Stale view attempt:** build a view, mutate through a valid event, then submit an old view's action token. The context hash/frontier check must reject as stale.
4. **Possession knowledge-transfer attempt:** bind actor A, generate A-only belief and open debug truth, then bind actor B. B's sealed context, notebook, action list, needs, intentions, and why-not must contain no A-only or debug-only facts.
5. **Direct-dispatch attempt:** TUI crate code must be structurally unable to call event application or mutate `PhysicalState` directly for embodied commands. Prefer Rust visibility/dependency/lint enforcement over `include_str!` substring tests.
6. **Rule-inference-in-TUI attempt:** input/render/debug modules must not import action validators, physical state, agent state, or event appliers except through allowed facade/view/report types. Tests should use module boundaries or compile-fail assertions where practical.
7. **Why-not non-leakage attempt:** hidden route, hidden food, closed container, absent item, and wrong suspicion fixtures must prove actor-visible why-not never names hidden truth while debug why-not can explain it non-diegetically.
8. **Typed diagnostics attempt:** changing display text, order of labels, or punctuation must not change semantic test outcomes. Changing typed reason/provenance must.
9. **No-human operator quarantine attempt:** execute no-human proof command through TUI. Assert it is non-diegetic/operator typed, its debug metrics do not become actor knowledge, and subsequent embodied view is rebuilt through the bound actor's sealed context.
10. **Render-dependency regression attempt:** if any future terminal backend is added, tests must prove terminal backend state cannot affect proposal order, outcomes, replay, actor-known context, or debug visibility.

### 7.3 Required review artifact format

Each fixture/test added under this spec must report:

- responsible layer (`view_model`, `debug_quarantine`, `tui_input_binding`, `proposal_construction`, `action_validation`, `event_application`, `projection`, `replay`, `test_oracle`, etc.);
- fixture/scenario ID;
- context ID/hash and actor/controller IDs;
- proposal/action/semantic IDs;
- typed reason codes and provenance refs;
- whether debug capability was present;
- actor-visible and debug-visible surfaces checked;
- expected event/log/replay/checksum result;
- failure mode if contamination occurs.

This aligns the proof with the execution tier's requirement for typed diagnostics, responsible layers, adversarial evidence, and deterministic/replayable artifacts (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:24-52`; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:54-96`).

## 8. Acceptance checklist

The remediation implementation is accepted only when every item below passes.

### 8.1 Workspace gates

The resulting workspace must pass:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

### 8.2 Per-requirement acceptance

| Requirement | Acceptance condition |
|---|---|
| TUI-AC-001 | Embodied view construction requires sealed holder-known context; TUI cannot build actor-visible views directly from raw truth. |
| TUI-AC-002 | Actor-visible affordances are source-bound to sealed context facts; hidden target discovery tests fail closed. |
| TUI-AC-003 | Disabled actions and why-not use typed availability/reason/provenance objects; display strings are render-only. |
| TUI-AC-004 | Proposals carry source view model, holder-known context ID/hash, frontier, semantic action ID, and provenance ancestry; forged/stale proposals reject before mutation. |
| TUI-AC-005 | `wait`, numeric selection, and `do <id>` all submit only current-view action tokens under the same context check. |
| TUI-AC-006 | No-human execution is typed as non-diegetic operator/proof tooling and cannot create actor knowledge. |
| TUI-AC-007 | Debug truth requires sealed debug capability; public bool/string markers are not the authority boundary. |
| TUI-AC-008 | Notebook beliefs/observations/contradictions/leads are typed, source-bound, actor-known, and wording-independent. |
| TUI-AC-009 | Actor-visible why-not is safe and debug why-not is quarantined; negative fixtures prove non-leakage. |
| TUI-AC-010 | Typed artifacts, transcript, and replay remain deterministic; debug panels remain read-only. |
| TUI-AC-011 | Tests use typed diagnostics/provenance and adversarial gates; substring tests are only smoke tests. |
| TUI-AC-012 | Possession changes input binding only; no cross-body knowledge/needs/intention/debug transfer occurs. |

### 8.3 Certification-result wording

When the implementation for this spec is reviewed, the review artifact must not claim full project certification unless the execution tier separately certifies it. Valid result wording for this spec's scope is:

- “TUI proof-surface hardening remediation accepted for the audited seam at `<commit>`”; or
- “TUI proof-surface hardening remediation rejected; failing requirement(s): `<TUI-AC-###>`.”

It must not say that the repository has passed `P0-CERT`, `SPINE-CERT`, or `EPI-CERT` unless the relevant execution-tier certification artifact independently says so.

## 9. Implementation notes for a later spec-to-tickets pass

A later decomposition should avoid re-litigating intent and should group work roughly as:

1. core holder-known context packet and context hash;
2. embodied view-model builder API rewrite;
3. typed action availability and proposal source/context fields;
4. TUI command loop cleanup for `wait` and no-human operator command classification;
5. debug capability type and debug constructor sealing;
6. typed notebook leads/provenance;
7. adversarial fixtures and typed diagnostic review artifacts;
8. transcript/replay stability updates.

Do not add duplicate APIs “for compatibility.” The spec-tier rules forbid alias paths and stale live companions; once the new boundary exists, old raw-builder paths must be removed or made test-internal under explicit sealed-context construction.

## Appendix A — Exact source ledger summary

Base exact raw URL used for repository files:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/bf0e3a0392bb1217caf034281913487aa1644ad4/
```

The full evidence ledger was reported before analysis. Paths audited for this spec were selected from the uploaded manifest and fetched only at the exact commit. The load-bearing set included the authority files named in §1, the archived intent specs named in §1, `archive/reports/PHASE_3A_IMPLEMENTATION_AUDIT.md` as style/evidence-depth reference, all current `tracewake-tui` source/test files, and core seams for view models, projections, debug reports, proposal/report/pipeline, event application/log/envelope, replay, and controller binding.

## Appendix B — External source notes

External sources sharpened implementation recommendations only; they do not outrank Tracewake doctrine.

[^elm]: The Elm Guide, “The Elm Architecture,” describes the Model/View/Update split in which model state, view rendering, and update logic are separated. URL: `https://guide.elm-lang.org/architecture/`
[^event-sourcing]: Martin Fowler, “Event Sourcing,” describes persisting state changes as an event sequence and reconstructing state from those events. URL: `https://martinfowler.com/eaaDev/EventSourcing.html`
[^prov]: W3C PROV-DM frames provenance as information about entities, activities, and agents that helps assess quality, reliability, and trustworthiness. URL: `https://www.w3.org/TR/prov-dm/`
[^ratatui-test]: Ratatui `TestBackend` documentation identifies it as a backend intended for terminal UI integration tests. URL: `https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html`
[^ratatui-backends]: Ratatui backend documentation describes terminal backends and their role in rendering terminal applications. URL: `https://ratatui.rs/concepts/backends/`
[^crossterm]: Crossterm terminal documentation covers raw mode and alternate-screen terminal behavior. URL: `https://docs.rs/crossterm/latest/crossterm/terminal/`
