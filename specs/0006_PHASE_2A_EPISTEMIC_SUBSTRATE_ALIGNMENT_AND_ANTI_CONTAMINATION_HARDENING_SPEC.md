# 0006 Phase 2A Epistemic Substrate Alignment and Anti-Contamination Hardening Spec

- **Intended staging path:** `specs/0006_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`
- **Final home after acceptance:** `docs/4-specs/0006_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`
- **Requested repository:** `joeloverbeck/tracewake`
- **Analyzed commit:** `4c4dfff83aae01006e6a3b653a3248179e0f9b25`
- **Execution admissibility posture:** `P0-CERT scoped remediation`
- **Primary live gate cross-reference:** `EPI-CERT`
- **Secondary gate cross-references:** `P0-CERT`, `SPINE-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`

## 0. Verdict

**Verdict: positive. A Phase-2A epistemic alignment-correction and anti-contamination hardening spec is warranted.**

The Phase-2A substrate is not hollow. The core typed proposition model, deterministic confidence encoding, event-stream separation, expectation-contradiction flow, content seed validation, typed `KnowledgePreconditionNotMet` rejection, actor-visible/debug why-not split, notebook/debug view-model split, replay schema-version rejection, and several possession/debug fixtures are materially aligned.

The material failures are narrower and more dangerous: the substrate’s most important proof surfaces are not sealed. `KnowledgeContext` is called sealed by design, but its authority-bearing fields are public and mutable after hash/audit computation (`crates/tracewake-core/src/epistemics/knowledge_context.rs:193-212`, `:241-308`). It also exposes a public debug context constructor that requires no debug capability (`:310-325`) and its scope filter trusts the mutable `mode` and `viewer_actor_id` fields (`:327-335`). `EpistemicProjection` exposes all-holder raw observations, beliefs, contradictions, notebook indexes, schema/version fields, and mutation helpers as public API (`crates/tracewake-core/src/epistemics/projection.rs:27-40`, `:85-121`). `Belief`, `Observation`, `Contradiction`, and `BeliefDraft` expose provenance/schema/privacy/holder fields publicly (`crates/tracewake-core/src/epistemics/belief.rs:39-55`, `:109-118`; `crates/tracewake-core/src/epistemics/observation.rs:107-123`; `crates/tracewake-core/src/epistemics/contradiction.rs:32-43`). Those choices make cross-holder leakage and post-construction provenance erasure possible through ordinary Rust public access, not through exotic unsafe code.

That violates the live doctrine’s structural bar: actors must act from their own beliefs and modeled channels, not hidden truth (`INV-002`, `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:11-13`); possession must not transfer knowledge (`INV-006`, `:33-35`); ground truth, subjective belief, and records must remain separate (`INV-023`, `:105-107`); important beliefs need provenance (`INV-026`, `:117-119`); debug is non-diegetic (`INV-031`, `:137-139`); embodied views are actor-known (`INV-067`, `:291-293`); and debug must not contaminate embodied play (`INV-068`, `:295-297`). The architecture is even more explicit: a decision-facing holder-known context must be built and then sealed (`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:31-61`, `:93-97`), and every decision boundary must receive a sealed holder-known context with provenance (`:21-24`).

**Corrective direction:** code yields to the live doctrine. No upstream doctrine amendment is required.

## 1. Baseline, source discipline, and numbering choice

This spec is scoped to the user-supplied target commit only. It does not verify that `4c4dfff83aae01006e6a3b653a3248179e0f9b25` is current `main`.

The path inventory was the uploaded manifest only. Every cited repository file was fetched using the exact mechanical form:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/4c4dfff83aae01006e6a3b653a3248179e0f9b25/<manifest path>
```

No clone, default-branch lookup, branch-name fetch, repository metadata, connector namespace label, GitHub code search, or snippet-based repository search was used as evidence. Archived baseline SHAs inside historical specs were not used as fetch targets.

The live source-discipline rule supports this method: the spec ledger says a manifest is path inventory only, branch/default/code-search evidence is not proof, and exact-commit audits must fetch by exact file URL or supplied export (`docs/4-specs/SPEC_LEDGER.md:17-23`). The docs map establishes that archived specs and tickets are evidence, not live authority (`docs/README.md:3-14`, `:43-52`). The ledger lists the Phase-2A archived spec as “landed historically; not a current gate definition” (`docs/4-specs/SPEC_LEDGER.md:45-49`).

The selected staging filename is `specs/0006_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`. This continues the visible `0002` → `0003` → `0004` → `0005` hardening epoch while respecting the live spec tier: the active specs table contains only `0001` as an active package, plus `0003` as a template, not an active spec package (`docs/4-specs/SPEC_LEDGER.md:25-33`). A same-number historical archive collision with an older Phase-3A `0006` is not a blocker because archived specs are historical, not live doctrine (`docs/4-specs/SPEC_LEDGER.md:45-57`). Future specs must declare exactly one admissibility posture and keep gate codes as cross-references only (`docs/4-specs/README.md:29-39`).

## 2. Authority chain and gate mapping

This spec operationalizes, but does not amend, the live authority chain:

- **Foundation:** `docs/README.md`, `docs/0-foundation/02`, `03`, `04`, `08`, `09`, `12`, `14`. Foundation owns the constitutional invariants, truth firewall, event/replay contract, no-scripting/prehistory provenance, TUI/debug split, possession parity, and first-proof scope.
- **Architecture:** `docs/1-architecture/01`, `02`, `03`, `04`, `06`, `10`, `13`. Architecture owns crate authority, event/replay/projection rules, sealed holder-known contexts, action pipeline boundaries, epistemic data flow, and view-model/debug boundaries.
- **Execution:** `docs/2-execution/00`, `01`, `03`, `04`, `05`, `07`, `08`, `09`, `10`. Execution owns gate order, audit posture, anti-contamination gates, action pipeline, `EPI-CERT` proof obligations, data validation, golden fixtures, replay, diagnostics, and review artifacts.
- **Reference:** `docs/3-reference/00`, `01`, `02`. Reference owns checklist, risk memory, and terminology. `holder-known context` is the system-wide term; `actor-known` is the actor case.
- **Spec tier:** `docs/4-specs/README.md`, `SPEC_LEDGER.md`, `0001`, `0003`. The spec tier is subordinate. This spec does not redefine `P0-CERT`, `EPI-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `POS-PARITY`, `REPLAY`, `FIXTURE`, or `DIAG`.

`EPI-CERT` is the primary cross-reference because the execution ladder defines it as certifying actor-known/holder-known contexts, beliefs, observations, provenance, possession parity, view models, and debug quarantine (`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md:26-35`). The same execution doc explicitly says “Phase 2A landed” is evidence for `EPI-CERT`, not certification (`:37-45`).

The posture is `P0-CERT scoped remediation` because this spec requires scoped code, tests, and CI/lock-layer changes in the Phase-2A epistemic substrate and contributes evidence toward `EPI-CERT`/`P0-CERT`; it does not claim either gate has passed.

## 3. Scope and non-goals

### 3.1 In scope

The scope is the Phase-2A epistemic substrate and its public seams:

- `crates/tracewake-core/src/epistemics/**`: typed propositions, observations, beliefs, contradictions, holder-known/actor-known context, knowledge basis, projection, notebook/debug builders, canonical serialization, stable ordering.
- `crates/tracewake-core/src/actions/defs/checkcontainer.rs` and `accuseprobe.rs`; the epistemic surface of action proposal/pipeline/report where it bears on typed why-not and knowledge preconditions.
- Epistemic event surfaces in `events/envelope.rs`, `events/apply.rs`, `state.rs`, `projections.rs`, `view_models.rs`, `replay/**`, `scheduler.rs`, `debug_capability.rs`, `debug_reports.rs`, and `ids.rs`.
- Epistemic content schema, validation, serialization, load, and fixtures in `tracewake-content`.
- TUI notebook/debug epistemics parsing and rendering in `tracewake-tui`.
- Existing lock-layer tests and CI only to determine whether they cover this Phase-2A substrate.
- The epistemics-side public API at the consumer seam with `crates/tracewake-core/src/agent/**`. The Phase-3A planner internals are not audited; they are a boundary marker only.

### 3.2 Out of scope

This spec does not re-audit the Phase-1 physical spine; it audits only the epistemic surface of shared spine files. It does not audit or remediate Phase-3A planner/needs/routines internals, institutions/records/wrong-suspicion, Phase-2B richer perception/testimony/memory decay, LLM surfaces, graphical clients, or new world mechanics. It does not perform higher-tier doc amendments.

## 4. External prior art used to sharpen structural requirements

The external sources below are not authority for Tracewake. They sharpen the structural locks requested here:

- Rao and Georgeff’s BDI work treats beliefs, desires, and intentions as distinct attitudes, matching `INV-033`’s doctrine that beliefs, goals/needs/duties, and intentions are distinct. Source: Rao & Georgeff, “BDI Agents: From Theory to Practice,” ICMAS 1995, https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf.
- Goguen and Meseguer’s noninterference framing captures the same safety intuition as Tracewake’s truth firewall: privileged information must not affect what a lower-authority observer can see unless a permitted channel exists. Source: “Security Policies and Security Models,” https://www.cs.purdue.edu/homes/ninghui/readings/AccessControl/goguen_meseguer_82.pdf.
- Event sourcing prior art supports the live doctrine’s “rebuild from events” bar: all state changes are represented as events, and state can be rebuilt by replaying the event log. Source: Martin Fowler, “Event Sourcing,” https://martinfowler.com/eaaDev/EventSourcing.html.
- Rust’s standard `HashMap` documentation states that `HashMap` is randomly seeded and each instance uses a different seed, which is why the substrate correctly prefers `BTreeMap`/`BTreeSet` for replay-significant epistemic state. Source: https://doc.rust-lang.org/beta/std/collections/struct.HashMap.html.
- Floating-point determinism literature motivates the spec’s requirement that epistemic confidence remain an integer/fixed canonical encoding, not raw `f32`/`f64` display. Source: Glenn Fiedler, “Floating Point Determinism,” https://gafferongames.com/post/floating_point_determinism/.
- Rust privacy is the right structural lever here: Rust checks item visibility and denies external uses of private items, so public fields should be removed from sealed proof types rather than guarded by comments. Source: Rust Reference, “Visibility and privacy,” https://doc.rust-lang.org/reference/visibility-and-privacy.html.
- “Parse, don’t validate” supports pushing provenance and sealed-state proof into types/builders so illegal states become unrepresentable rather than conventionally rejected after mutation. Source: Alexis King, “Parse, don’t validate,” https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/.

## 5. Alignment and contamination findings

### 5.1 Summary finding table

| ID | Verdict | Material risk | Corrective direction | Invariants |
|---|---|---|---|---|
| `EPI-HARD-001` | Misaligned | `KnowledgeContext` is not structurally sealed; debug context is forgeable through public API; scope filters trust mutable fields. | Code yields. Make holder-known contexts private/typed/capability-mediated. | `INV-002`, `INV-006`, `INV-023`, `INV-024`, `INV-026`, `INV-031`, `INV-067`, `INV-068` |
| `EPI-HARD-002` | Misaligned | `EpistemicProjection` exposes raw all-holder maps and public mutation helpers, bypassing actor-known filtering and event-sourced apply. | Code yields. Private raw store, public filtered read model only, debug access through capability. | `INV-002`, `INV-006`, `INV-009`, `INV-012`, `INV-018`, `INV-023`, `INV-026`, `INV-067`, `INV-068` |
| `EPI-HARD-003` | Misaligned | `Belief`, `Observation`, `Contradiction`, and `BeliefDraft` permit post-construction provenance/schema/privacy mutation and literal/field-based construction. | Code yields. Private fields, smart constructors/builders, event-apply-only mutation. | `INV-009`, `INV-012`, `INV-018`, `INV-020`, `INV-021`, `INV-023`, `INV-026` |
| `EPI-HARD-004` | Unlocked contamination path | Existing lock layer is wired into CI, but its strongest structural gates do not cover epistemic context/projection/public-field sealing or float-confidence regression. | Code yields. Extend existing lock layer, do not invent a parallel one. | `INV-002`, `INV-006`, `INV-018`, `INV-020`, `INV-023`, `INV-026`, `INV-031`, `INV-067`, `INV-068`, `INV-070` |

### 5.2 Conformance walk by epistemic dimension

#### Typed propositions and string/prose authority

**Doctrine.** `INV-021` requires important beliefs, observations, actor-known leads, and notebook entries to be grounded in typed claims/propositions (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:97-99`). `INV-022` says raw prose may render claims but may not define belief content or proof (`:101-103`). Foundation requires typed, referential, comparable, renderable, sourceable, contradiction-capable claims (`docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md:28-75`). Architecture says a display sentence is not a proposition unless represented as a typed claim with source, stance, holder, and uncertainty (`docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md:23-42`).

**Code.** `Proposition` is a typed enum with Phase-2A-relevant variants such as `ItemLocatedInContainer`, `ItemMissingFromExpectedLocation`, `SoundHeardNearPlace`, `PossibleMovementNearPlace`, and `ActorWasNearPlace` (`crates/tracewake-core/src/epistemics/proposition.rs:8-39`). It implements reference validation and canonical parse/render support (`:104-330`; the `PropositionReferenceError` type is at `:41-70`). Content seed schema uses `Proposition` directly for initial beliefs (`crates/tracewake-content/src/schema.rs:304-318`).

**Verdict.** Aligned. No code correction is required for proposition typing itself. The hardening requirement is to keep this typed substrate private enough that downstream code cannot replace it with stringly truth fields.

#### Observation model: channel-specific, observation is not interpretation, confidence deterministic

**Doctrine.** `INV-016` says absence becomes evidence only through a channel (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:75-77`). `INV-030` says evidence is not truth (`:133-135`). Foundation separates observation from interpretation and gives the sound example: hearing a sound does not become knowledge of theft (`docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md:102-121`). Architecture requires observation fields: observer, channel, time/window, place, target, source event, confidence, alternatives, and privacy scope; a simple sound supports “sound heard,” not guilt (`docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md:44-49`).

**Code.** `Confidence` is `u16` parts-per-thousand with range-checking and fixed four-digit canonical serialization (`crates/tracewake-core/src/epistemics/observation.rs:9-35`). Channels are typed (`DirectSight`, `TouchOrSearch`, `SimpleSound`, `AbsenceMarker`, `ReadingPlaceholderSchemaOnly`) and stable-id-rendered (`:42-60`). Observation subjects/targets and `SourceRef` are typed (`:78-98`). `Observation::new` sets source, confidence, schema version, and actor-private scope (`:125-153`). `checkcontainer` emits a modeled `ContainerChecked` event and a separate `ObservationRecorded` event sourced from the check event with channel `touch_or_search` and confidence `1000` (`crates/tracewake-core/src/actions/defs/checkcontainer.rs:11-80`, `:83-147`).

**Verdict.** Semantically aligned. Public field mutability is misaligned under `EPI-HARD-003`: after construction, callers can rewrite `source`, `confidence`, `privacy_scope`, `schema_version`, `subject`, or `target` (`crates/tracewake-core/src/epistemics/observation.rs:107-123`). The current good constructor is not a structural guarantee.

#### Source-backed beliefs and provenance completeness

**Doctrine.** `INV-026` requires important beliefs to record holder, claim, stance, confidence, source, channel, acquisition time, staleness, provenance chain, contradiction links, and scope (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:117-119`). Foundation’s belief shape repeats the same fields and says a proposition is not a global flag (`docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md:77-100`). Architecture requires belief holder/proposition/stance/confidence/source/channel/ticks/links/privacy and says to use belief, not knowledge, when correctness is not guaranteed (`docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md:50-55`).

**Code.** `Belief` contains holder, proposition, stance, confidence, source, channel, acquired tick, last verified/stale ticks, observation links, contradiction links, schema version, and privacy scope (`crates/tracewake-core/src/epistemics/belief.rs:39-55`). `Belief::new` derives actor-private scope from the holder and requires a `SourceRef` (`:57-91`). `BeliefDraft::build` rejects missing belief id, holder, proposition, stance, confidence, source, and acquired tick (`:109-133`). Content seed validation rejects missing actor-known provenance and accepts explicit channel provenance (`crates/tracewake-content/tests/forbidden_content.rs:207-223`).

**Verdict.** Partially aligned but not sealed. The shape is right. The public fields and public draft fields let downstream code erase provenance, alter holder/scope, downgrade schema, or fabricate a literal-looking record after validation. That is `EPI-HARD-003`.

#### Expectation contradiction and absence as evidence

**Doctrine.** `INV-016` requires a channel before absence becomes evidence (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:75-77`). Foundation says absence/anomaly is discovered through contradiction between expectation and observation/search, and “Absence without expectation is not evidence to that actor” (`docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md:146-160`). Architecture says absence can become evidence only after a modeled search/observation event over an expected scope and cannot prove who moved the item (`docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md:70-76`).

**Code.** `detect_expected_absences` only considers beliefs held by the holder with `Stance::ExpectsTrue`, only for expected `ItemLocatedInContainer`, only for the checked container, and only when the expected item is absent from observed contents (`crates/tracewake-core/src/epistemics/contradiction.rs:78-105`). It creates an `ItemMissingFromExpectedLocation` proposition, links the prior belief and observation, sources the missing belief from the check event, and marks the channel `AbsenceMarker` (`:106-148`).

**Verdict.** Aligned. The same public-record mutability in `EPI-HARD-003` must be fixed so the links and source cannot be erased after detection.

#### Holder-known / actor-known filtering and sealed contexts

**Doctrine.** `INV-002` requires actors to act from their beliefs, observations, and modeled channels, not hidden truth (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:11-13`). `INV-024` forbids telepathy (`:109-111`). `INV-006` says possession transfers no world knowledge (`:33-35`). Foundation `14` requires the transaction to build a sealed actor-known context and exclude validator-only truth/debug truth (`docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md:39-58`, `:60-116`). Architecture `03` owns the requirement that every decision boundary receives a sealed holder-known context with provenance (`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:21-24`) and says the packet is built and then sealed (`:31-61`, `:93-97`).

**Code.** The positive part is clear: `KnowledgeContext::embodied` seals an actor-private context with allowed/forbidden sources and provenance (`crates/tracewake-core/src/epistemics/knowledge_context.rs:214-239`, `:241-308`). `knowledge_basis` queries `projection.beliefs_for_context(context)` rather than raw maps (`crates/tracewake-core/src/epistemics/knowledge_basis.rs:5-21`). `accuseprobe` builds an embodied context and rejects unsupported accusations with `KnowledgePreconditionNotMet` (`crates/tracewake-core/src/actions/defs/accuseprobe.rs:29-54`).

The misalignment is structural: `KnowledgeContext` exposes `viewer_actor_id`, `bound_actor_id`, `mode`, all scopes, allowed/forbidden sets, schema version, and `debug_non_diegetic` as public mutable fields after the hash is computed (`crates/tracewake-core/src/epistemics/knowledge_context.rs:193-212`, `:241-308`). `permits_scope` trusts those mutable fields (`:327-335`). `KnowledgeContext::debug` is public and requires no `DebugCapability` (`:310-325`).

**Verdict.** Misaligned: `EPI-HARD-001`.

#### Epistemic projection filtering, public raw maps, and consumer seam firewall

**Doctrine.** Architecture requires actor-known contexts to limit cognition to modeled knowledge and rejects raw physical truth/fixture truth/debug omniscience/projection completeness assumptions as context sources (`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:63-91`, `:99-116`). Architecture `06` requires every observation/belief/notebook entry to have a holder/visibility scope and embodied views to filter by holder-known context, while debug views remain structurally separate (`docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md:82-85`). Execution `07` requires actor-known projections to rebuild from events, debug panels to be non-diegetic and unable to feed gameplay/planner/affordances, and why-not fields to split actor/debug audiences (`docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md:84-101`).

**Code.** Filtered APIs exist: `observations_for_context`, `beliefs_for_context`, `contradictions_for_context`, and `notebook_entries_for_context` use the supplied context (`crates/tracewake-core/src/epistemics/projection.rs:124-180`). However, the raw store is fully public: all observation/belief/contradiction/notebook maps and schema/version metadata are public (`:27-40`), and direct insertion helpers are public (`:85-121`). The TUI debug path currently reads raw projection maps directly across the crate boundary (`crates/tracewake-tui/src/app.rs:371-443`). That is acceptable as a debug capability only if the raw access is core-mediated and non-diegetic; here the raw map exposure is a general API.

The Phase-3A consumer seam itself has a positive boundary: `ActorKnownPlanningContext` fields are private, construction from observed parts is crate-private, and compile-fail docs reject construction from raw `PhysicalState` (`crates/tracewake-core/src/agent/actor_known.rs:184-252`). That does not save the epistemics-side API because any core or external consumer can still read/alter projection internals through public fields before or instead of building a proper actor-known planning context.

**Verdict.** Misaligned: `EPI-HARD-002`.

#### Epistemic events, apply path, and no silent belief mutation in physical apply

**Doctrine.** `INV-009` and `INV-012` require meaningful state changes, including belief and projection changes, to be represented by events (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:47-61`). Architecture says epistemic streams include observations and actor-belief changes, not physical truth mutation, and projections may not create facts not implied by validated seeds and events (`docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md:57-72`).

**Code.** `EventKind` includes epistemic variants (`InitialBeliefSeeded`, `ObservationRecorded`, `BeliefUpdated`, `ExpectationContradicted`, `ContainerChecked`) (`crates/tracewake-core/src/events/envelope.rs:77-101`). Those variants map to `EventStream::Epistemic` (`:209-223`). Epistemic payload structs carry schema versions and typed payload records (`:436-520`). `apply_event_stream` routes epistemic events to `apply_epistemic_event` only when an epistemic projection is supplied and otherwise returns a non-world no-op (`crates/tracewake-core/src/events/apply.rs:41-69`). `apply_epistemic_event` rejects unsupported event and payload schema versions, parses typed payloads, and applies them to the projection (`:175-219`).

**Verdict.** Aligned at the event/apply seam. `EPI-HARD-002` and `EPI-HARD-003` are still necessary because public projection inserts and public record fields create alternate mutation paths outside event apply.

#### Replay, schema-version rejection, deterministic ordering, and confidence encoding

**Doctrine.** `INV-018` says replay must be deterministic (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:83-85`). `INV-020` requires event schema evolution and rejection of unsupported historical events unless migrated (`:91-93`). Architecture requires deterministic projection rebuild, checksums, epistemic rebuild, and replay/live match (`docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md:139-165`). Execution testing doctrine requires no unordered iteration in proof paths (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:97-110`).

**Code.** The epistemic projection uses `BTreeMap` and `BTreeSet` (`crates/tracewake-core/src/epistemics/projection.rs:27-40`, `:68-83`). Its checksum renders canonical lines from deterministic maps and uses canonical confidence serialization (`:183-190` and following checksum generation). `Confidence` is integer encoded and canonical (`crates/tracewake-core/src/epistemics/observation.rs:9-35`). Replay builds a fresh `EpistemicProjection`, records unsupported epistemic versions without applying them, and computes a final epistemic checksum (`crates/tracewake-core/src/replay/rebuild.rs:45-90`, `:440-488`).

**Verdict.** Runtime/model code is aligned. Lock-layer coverage is incomplete under `EPI-HARD-004`: `clippy.toml` disallows `HashMap`, `HashSet`, `SystemTime`, `Instant`, thread/process/fs/network APIs (`clippy.toml:1-16`), but no current lock prevents `f32`/`f64` or raw float display from entering epistemic confidence paths. That matters because confidence determinism is a first-class Phase-2A risk.

#### Content epistemic schema, validation, forbidden shortcut truth, and no prose-born facts

**Doctrine.** `INV-022` rejects raw prose as authoritative state (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:101-103`). Foundation `09` says seeds may create initial conditions but not authored arcs, and prehistory must be structured provenance, not lore prose (`docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md:129-176`). Execution `08` requires schema validation to reject outcome chains, culprit/player/omniscience shortcuts, missing provenance, silent migration, and forbidden truth fields including `culprit`, `stolen_flag`, `npc_knows_truth`, and debug truth (`docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md:19-43`, `:63-83`, `:98-115`).

**Code.** `InitialBeliefSchema` stores typed belief id, holder actor, proposition, stance, confidence, source kind, source, channel, ticks, privacy, and schema version (`crates/tracewake-content/src/schema.rs:304-318`). `to_belief` constructs a `Belief` from the schema (`:393-409`). Validation rejects unsupported epistemic schema versions, out-of-range confidence, cross-actor seed scopes, unsupported source kinds, forbidden shortcut-truth source ids, and bad provenance (`crates/tracewake-content/src/validate.rs:1450-1548`). Forbidden-key detection includes `hidden_true_item_location`, `actor_known_hidden_input`, `culprit`, `true_culprit`, `stolen_flag`, `npc_knows_truth`, `knows_mara_did_it`, `quest_state`, and `player_memory` (`:1689-1745`). Tests assert shortcut fields are blocking errors and initial beliefs require provenance (`crates/tracewake-content/tests/forbidden_content.rs:101-124`, `:207-223`, `:350-367`).

**Verdict.** Aligned for current content validation. The new lock layer must add epistemic negative fixtures proving nested/renamed/field-alias shortcut truth and missing provenance continue to trip guards; current tests cover several friendly and adversarial cases but do not cover all API-sealing findings.

#### Notebook view, debug epistemics, possession parity, and TUI boundary

**Doctrine.** `INV-067` requires embodied views to be actor-known (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:291-293`). `INV-068` requires debug tools to be non-diegetic and never feed ordinary play (`:295-297`). Foundation `08` says embodied mode may show only actor-known local perception, knowledge, memory, records, and affordances, and must hide hidden ground truth, other minds, culprit certainty, debug graphs, and possession history (`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md:53-78`). Possession changes input routing and view selection only, not knowledge (`:133-143`). Execution `07` requires debug panels to be quarantined and possession parity fixtures to prove shared rules (`docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md:20-59`, `:84-101`).

**Code.** The core view model carries holder-known context id/hash/frontier/source summary and notebook (`crates/tracewake-core/src/view_models.rs:18-42`). `WhyNotView` separates actor-known summary, reason codes, and actor-visible facts (`:61-107`). Debug epistemics view structs carry a private `DebugCapability` and expose a non-diegetic marker (`:451-482`, `:564-590`). Tests assert notebook entries are source-bound and debug epistemics list all holders only in a debug view (`:835-940`). TUI parses `notebook`, `debug epistemics`, `debug beliefs <actor>`, and `debug observations <actor>` (`crates/tracewake-tui/src/input.rs:60-120`), and debug renderers assert debug-only markers (`crates/tracewake-tui/src/debug_panels.rs:124-194`).

**Verdict.** View-model intent is aligned, but debug quarantine is undermined by `EPI-HARD-002`: TUI debug builders read raw projection maps directly because the projection exposes them publicly (`crates/tracewake-tui/src/app.rs:371-443`). That should move behind core debug APIs that require or internally mint a non-diegetic capability, while embodied/notebook code receives only filtered read models.

#### Typed `KnowledgePreconditionNotMet` and two-layer why-not

**Doctrine.** `INV-070` requires why-not output to distinguish actor-known blockers from hidden validation/debug details (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:303-305`). Execution `07` says why-not has actor-visible and debug/operator audiences and display strings are not an authority boundary (`docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md:73-82`). Execution `10` requires diagnostics to be typed/provenance-bearing and reject tests that rely on display labels (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:38-52`, `:135-144`).

**Code.** `accuseprobe` rejects unsupported accusations with `ReasonCode::KnowledgePreconditionNotMet` and actor-visible text that omits the accused actor/culprit while debug text includes the proposition (`crates/tracewake-core/src/actions/defs/accuseprobe.rs:43-54`, `:145-156`). `WhyNotView` copies stable reason-code ids from typed `ReasonCode` values and classifies `KnowledgePreconditionNotMet` as `ActorKnownUncertainty` (`crates/tracewake-core/src/view_models.rs:91-107`, `:654-710`).

**Verdict.** Aligned. Add adversarial tests under `EPI-HARD-004` to prove actor-visible reasons cannot pick up debug summaries by string reuse.

#### Crate dependency and authority boundary

**Doctrine.** Architecture says `tracewake-core` owns authoritative kernel state, events, replay, projections, action validation, actor-known decision substrate, and semantic view-model inputs; content and TUI are downstream (`docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md:1-80`).

**Code.** The epistemic substrate is in `tracewake-core`; content schema uses core types; TUI consumes core view models. This is the right crate placement. The defect is not crate direction but public API width across the crate boundary.

**Verdict.** Aligned in placement; needs API sealing under `EPI-HARD-001` and `EPI-HARD-002`.

## 6. Structural anti-contamination requirements

### `EPI-HARD-001` — Seal holder-known contexts and capability-gate debug contexts

**Problem.** `KnowledgeContext` claims sealed context semantics, but its mode, holder/viewer ids, scopes, allowed/forbidden sources, schema version, and debug flag are public and mutable after hash/audit computation (`crates/tracewake-core/src/epistemics/knowledge_context.rs:193-212`, `:241-308`). Public `KnowledgeContext::debug` requires no capability (`:310-325`).

**Required change.**

1. Make every authority-bearing `KnowledgeContext` field private. Provide read-only accessors only.
2. Split embodied and debug authority at the type level. Acceptable shapes include:
   - `EmbodiedKnowledgeContext` and `DebugKnowledgeContext` newtypes over a private sealed packet; or
   - a private `KnowledgeContextMode` plus public constructors returning sealed wrappers whose debug constructor requires a `DebugCapability` or crate-private debug factory.
3. Remove or restrict public `KnowledgeContext::debug`. External crates must not mint or request a debug context by ordinary public API. Debug view builders may use a crate-internal debug factory that returns non-diegetic view models, not raw all-holder read handles.
4. Make `permits_scope` depend on immutable private state only. Any context hash/audit must cover all fields that affect filtering and remain stable after construction.
5. Add `try_reseal` only if a future feature truly needs context evolution. Phase-2A should not need it; new context creation after modeled feedback is the safer path.

**Structural lock.**

- Add compile-fail negative fixtures under `tests/negative-fixtures/`:
  - `external_crate_cannot_mutate_knowledge_context_mode` attempts `context.mode = ViewMode::Debug`.
  - `external_crate_cannot_mutate_knowledge_context_viewer` attempts to rewrite `viewer_actor_id` or scope fields.
  - `external_crate_cannot_build_debug_knowledge_context` attempts to call the debug constructor or mint the debug context without a capability.
- Extend `crates/tracewake-core/tests/hidden_truth_gates.rs`, which currently hardens `agent/actor_known.rs` and `debug_capability.rs` source strings (`:26-28`, `:117-153`), to include `src/epistemics/knowledge_context.rs` and assert no `pub` authority-bearing fields and no public debug constructor.
- Add a runtime regression test: an embodied context’s id/hash/audit and scope result remain consistent and cannot be mutated into debug behavior.
- CI already runs `hidden_truth_gates` and negative fixture runner in the lock-layer job (`.github/workflows/ci.yml:64-91`); extend those existing suites rather than creating a parallel gate.

**Acceptance.** External crates cannot alter a holder-known context after creation, cannot forge a debug context, and cannot cause `beliefs_for_context` to return another holder’s private beliefs by changing context fields.

### `EPI-HARD-002` — Hide raw epistemic projection storage behind filtered read models and debug capability

**Problem.** `EpistemicProjection` exposes all-holder raw maps, notebook indexes, schema/version fields, event range, manifest id, and public insert helpers (`crates/tracewake-core/src/epistemics/projection.rs:27-40`, `:85-121`). Filtered APIs exist (`:124-180`), but callers can bypass them. TUI debug code currently reads raw maps directly across the crate boundary (`crates/tracewake-tui/src/app.rs:371-443`).

**Required change.**

1. Make raw projection maps and metadata private.
2. Make `insert_observation`, `insert_belief`, `insert_contradiction`, and `insert_notebook_entry` `pub(crate)` or narrower. The public mutation path must be event apply/replay, not direct projection mutation.
3. Expose public actor-known APIs only through sealed `EmbodiedKnowledgeContext`/holder-known context handles, returning filtered read-only views or iterators.
4. Expose debug all-holder APIs only through a non-diegetic debug-view builder that is core-owned and capability-mediated. The TUI may request/render `DebugEpistemicsView`; it must not read `beliefs_by_id`, `beliefs_by_holder`, `observations_by_id`, or `contradictions_by_id` directly.
5. Preserve deterministic ordering through internal `BTreeMap`/`BTreeSet` and explicit stable sorting in returned vectors.

**Structural lock.**

- Add compile-fail negative fixtures:
  - `external_crate_cannot_read_raw_epistemic_projection_maps` attempts to read `projection.beliefs_by_id` and `projection.observations_by_id`.
  - `external_crate_cannot_insert_raw_epistemic_records` attempts to call `insert_belief` or `insert_observation` from outside core.
  - `external_crate_cannot_build_debug_projection_view_without_core_debug_api` attempts raw all-holder access.
- Add a core unit test that constructs two actors’ beliefs and proves an embodied context returns only the bound actor while a debug view can list all holders only via the debug path.
- Extend TUI seam tests so `debug epistemics` still renders non-diegetic all-holder data, but embodied `notebook` cannot import raw debug rows.
- Extend `hidden_truth_gates` source guard to fail on public projection fields matching `pub .*beliefs_by_id`, `pub .*observations_by_id`, `pub .*contradictions_by_id`, `pub .*notebook_entries_by_actor`, and public insert helpers.

**Acceptance.** No public API can read all-holder epistemic state except debug view/report constructors that carry a non-diegetic capability marker. Normal consumers can only ask filtered, context-bound questions.

### `EPI-HARD-003` — Make epistemic records provenance-preserving abstract data types

**Problem.** `Belief`, `Observation`, `Contradiction`, and `BeliefDraft` expose public mutable fields, including holder, proposition, source, channel, schema version, privacy scope, and contradiction links (`crates/tracewake-core/src/epistemics/belief.rs:39-55`, `:109-118`; `crates/tracewake-core/src/epistemics/observation.rs:107-123`; `crates/tracewake-core/src/epistemics/contradiction.rs:32-43`). That lets code erase provenance or forge stale/schema-invalid records after constructor validation.

**Required change.**

1. Make all epistemic record fields private except genuinely harmless display-only fields, and prefer private for all proof-bearing types.
2. Provide accessors for read paths and `with_*` methods only where they preserve invariants.
3. Replace `BeliefDraft` public options with builder methods or a staged builder that cannot be consumed without source/provenance/holder/proposition/confidence.
4. Any field that changes provenance, holder, privacy, schema, confidence, observation links, or contradiction links must be modified only by a constructor, event-apply function, or internal builder with validation.
5. Content schema conversion must use safe builder methods rather than mutating `belief.channel`, `belief.last_verified_tick`, `belief.privacy_scope`, and `belief.schema_version` directly (`crates/tracewake-content/src/schema.rs:393-409`).

**Structural lock.**

- Add compile-fail negative fixtures:
  - `external_crate_cannot_construct_belief_literal` tries `Belief { ... }`.
  - `external_crate_cannot_mutate_belief_source_or_scope` tries to clear or rewrite `source`, `privacy_scope`, or `schema_version`.
  - `external_crate_cannot_construct_observation_without_source` tries literal construction or source mutation.
  - `external_crate_cannot_mutate_contradiction_links` tries to rewrite expectation/observation ids.
- Add unit tests that content conversion and event apply reject or cannot represent missing provenance, unsupported schema, cross-actor privacy, and absent contradiction links.
- Extend content forbidden/provenance tests rather than creating parallel content fixtures: `crates/tracewake-content/tests/forbidden_content.rs` already proves shortcut fields and missing provenance (`:101-124`, `:207-223`, `:350-367`). Add negative cases for post-parse mutation attempts only through compile-fail tests.

**Acceptance.** A belief/observation/contradiction without provenance, schema, holder, scope, and required links is unconstructable through public API. Rewriting those fields after validation fails to compile outside the trusted module.

### `EPI-HARD-004` — Extend the existing lock layer to epistemic API sealing and confidence determinism

**Problem.** CI runs strong lock-layer suites (`.github/workflows/ci.yml:64-91`), and `clippy.toml` bans nondeterministic maps/time/thread/process/fs/network APIs (`clippy.toml:1-16`). `hidden_truth_gates` hardens the Phase-3A actor-known planning context and debug capability source (`crates/tracewake-core/tests/hidden_truth_gates.rs:26-28`, `:117-153`). But no current guard fails if the epistemic substrate reintroduces public context/projection fields, public all-holder debug contexts, public direct projection inserts, or raw `f32`/`f64` confidence formatting in epistemic paths.

**Required change.**

1. Extend the existing source guards to walk:
   - `crates/tracewake-core/src/epistemics/**/*.rs`
   - `crates/tracewake-core/src/view_models.rs`
   - `crates/tracewake-core/src/debug_reports.rs`
   - `crates/tracewake-content/src/{schema,validate,serialization,load}.rs`
   - epistemic fixture modules under `crates/tracewake-content/src/fixtures/`
   - `crates/tracewake-tui/src/{input,render,debug_panels,app,transcript}.rs` for epistemic command/render seams.
2. Add banned epistemic tokens/patterns for public authority fields and raw confidence floats:
   - `pub viewer_actor_id`, `pub mode`, `pub .*_scope` in `KnowledgeContext`
   - `pub observations_by_id`, `pub beliefs_by_id`, `pub beliefs_by_holder`, `pub contradictions_by_id`, `pub notebook_entries_by_actor`
   - `pub fn debug(` on `KnowledgeContext` unless capability-mediated and explicitly whitelisted
   - `f32`, `f64`, `parse::<f32>`, `parse::<f64>`, float `Display`/formatting in epistemic confidence paths
   - `HashMap`/`HashSet` in epistemic projection/checksum/notebook paths, even though `clippy.toml` already bans the standard types globally.
3. Add negative fixtures that intentionally violate each new guard, and assert the negative fixture runner catches them. Guard self-coverage is required; a scanner without failing fixtures is not enough.
4. Extend event-schema/replay gates to assert every epistemic event kind appears in schema registry/replay tests and unsupported epistemic schema versions remain loud.
5. Extend TUI adversarial gates to assert debug truth never appears in embodied/notebook output and actor-visible why-not never contains debug summary strings.

**Acceptance.** The lock layer fails on deliberate epistemic leaks, public proof-field reintroduction, raw all-holder projection access, raw float confidence, hash-ordered epistemic collections, unsupported epistemic schema versions, and debug-to-embodied contamination.

## 7. Required fixtures and tests

Harden existing fixtures and suites where they already exist. Do not author parallel fixture families merely for symmetry.

| Test/fixture requirement | Existing base to extend | Required positive assertion | Required negative/adversarial assertion |
|---|---|---|---|
| Belief requires provenance and cannot be mutated after construction | `crates/tracewake-content/tests/forbidden_content.rs`; new compile-fail fixture | Valid source-backed initial belief loads | Missing provenance rejected; public mutation/literal construction fails to compile |
| Prose-born or typeless proposition rejected | `crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs`; `forbidden_content.rs` | Typed proposition seeds load | Raw prose/typeless proposition cannot create authoritative belief |
| Shortcut truth fields rejected | `forbidden_content.rs` | Valid fixture canonicalizes | `culprit`, `stolen_flag`, `npc_knows_truth`, `knows_mara_did_it`, nested/renamed aliases fail |
| Absence requires expectation | `expectation_contradiction_001`; `checkcontainer` tests | Expected item absent creates contradiction and missing belief | Container check without matching expectation produces no missing-property belief |
| Possession transfers no belief | `possession_parity_001`; TUI embodied tests | Rebinding selects actor-filtered view | Rebinding cannot transfer prior actor notebook/beliefs/known culprits |
| Debug truth quarantined | `view_filtering_001`; TUI debug/adversarial tests | Debug panel lists non-diegetic all-holder beliefs | Embodied `notebook`/view/why-not does not contain debug truth or debug summary strings |
| Unsupported epistemic schema version rejected loudly | `event_schema_replay_gates`; `replay/rebuild.rs` tests | Supported epistemic v1 rebuilds deterministically | Unsupported epistemic event or record schema is reported and not applied |
| Deterministic confidence and ordering | `projection.rs` checksum tests; `clippy.toml`; source guard | Confidence remains integer/canonical; BTree ordering stable | Deliberate `f32`/`f64`, raw float display, or `HashMap`/`HashSet` in epistemic path trips guard |
| No-human epistemic check parity | `no_human_epistemic_check_001`; scheduler/no-human capstone | No-human actor uses same check/observation/belief path | No-human path cannot read debug truth or bypass actor-known context |
| Accuse probe knowledge blocker | `knowledge_blocker_accuse_001`; `accuseprobe.rs` | Source-backed support permits probe | Unsupported accusation rejects with typed `KnowledgePreconditionNotMet` and no culprit leak |
| Sound uncertainty | `sound_uncertainty_001` | Sound creates low-confidence observation/possible movement, not theft | Sound alone cannot satisfy culprit/proof/accuse precondition |
| API sealing self-coverage | new negative fixtures; `hidden_truth_gates.rs` | External crates can call safe filtered APIs | External crates cannot mutate context, read raw maps, or mint debug context |

## 8. Required upstream doc amendments

None.

The live higher tiers already say what this pass needs:

- contexts are sealed and provenance-bearing (`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:21-24`, `:31-61`, `:93-97`);
- raw truth/debug/other-holder data is forbidden in actor cognition (`docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md:60-116`);
- debug is non-diegetic and quarantined (`docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md:185-196`);
- epistemic data must carry provenance and be holder-filtered (`docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md:44-85`);
- tests must not bypass actor-known provenance (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:18-36`, `:135-144`).

All findings therefore use code-yields corrective direction.

## 9. Acceptance checklist

### 9.1 Workspace gates to run after implementation

Record command, exact commit, result, and concise output summary:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`

CI must continue to run the lock-layer job, and that job must include the extended epistemic guards (`.github/workflows/ci.yml:64-91`). Any new guard must be expressible on the pinned Rust toolchain and normal Cargo test/clippy surface.

### 9.2 Per-requirement acceptance evidence

| Requirement | Responsible surface | Acceptance evidence | Result wording |
|---|---|---|---|
| `EPI-HARD-001` | `core/epistemics/knowledge_context.rs`, `core/debug_capability.rs`, negative fixtures, hidden truth gates | Private authority-bearing context fields; capability-mediated debug context; compile-fail external mutation/forge tests; runtime scope/hash consistency test | `<pass/fail>` |
| `EPI-HARD-002` | `core/epistemics/projection.rs`, `core/view_models.rs`, `tui/app.rs`, TUI/debug tests | Private raw maps; public filtered actor-known API; debug all-holder API only through debug view/capability; compile-fail raw map/read/insert tests | `<pass/fail>` |
| `EPI-HARD-003` | `core/epistemics/{belief,observation,contradiction}.rs`, `content/schema.rs`, event apply tests | Private record fields; provenance-preserving builders; content conversion without post-hoc public mutation; compile-fail literal/mutation tests | `<pass/fail>` |
| `EPI-HARD-004` | `clippy.toml`, `core/tests/*`, `content/tests/*`, `tui/tests/*`, negative fixtures, CI | Source guards walk epistemic paths; float/hash/public-field leaks trip negative fixtures; event schema/replay and debug/notebook gates extended | `<pass/fail>` |
| Epistemic event/replay parity | `events/envelope.rs`, `events/apply.rs`, `replay/rebuild.rs`, `event_schema_replay_gates` | All epistemic event kinds mapped to `EventStream::Epistemic`; unsupported versions loud; projection checksum stable | `<pass/fail>` |
| Content provenance/no-script parity | `content/schema.rs`, `content/validate.rs`, `forbidden_content.rs` | Missing provenance, shortcut truth fields, prose-born facts, unsupported epistemic schema rejected | `<pass/fail>` |
| TUI/debug proof surface | `view_models.rs`, `tui/input.rs`, `tui/debug_panels.rs`, TUI adversarial gates | `notebook` actor-scoped; `debug epistemics/beliefs/observations` non-diegetic; actor-visible output excludes debug truth | `<pass/fail>` |
| Consumer seam firewall | `core/epistemics` public API, `core/agent/actor_known.rs` boundary tests | External consumers receive only sealed actor-known context/filter APIs; planner internals not audited by this spec | `<pass/fail>` |

### 9.3 Scoped certification wording

Allowed result wording after implementation acceptance:

> Phase 2A epistemic substrate alignment and anti-contamination hardening remediation accepted for exact commit `<commit-under-review>`. This contributes scoped evidence toward `EPI-CERT` and `P0-CERT`; it does not certify latest `main`, later-phase scope, or the full project.

Forbidden result wording unless a separate upstream certification process declares it:

- “Tracewake is fully certified.”
- “Latest main was independently verified.”
- “Phase 3A planner/ordinary-life/institutions are certified by this pass.”
- “Archived specs are live authority.”
- “Project is P0 certified.”
- “`EPI-CERT` passed.”

## 10. Implementation notes for a later spec-to-tickets pass

The likely ticket decomposition is:

1. Seal `KnowledgeContext` and debug context construction.
2. Seal `EpistemicProjection` storage and add filtered/debug read APIs.
3. Seal epistemic records and replace public field mutation in content conversion/event apply.
4. Move TUI debug epistemics construction behind core debug view APIs.
5. Add compile-fail negative fixtures for context/projection/record sealing.
6. Extend source guards and CI lock-layer coverage to epistemic paths and float confidence.
7. Extend replay/event-schema/fixture/TUI tests for epistemic adversarial cases.
8. Produce a scoped acceptance artifact using `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` wording.

Do not split these into a broader Phase-3A or institution pass. The planner seam is verified only insofar as the epistemics-side public API cannot leak raw truth/other-holder/debug information.

## 11. Additional suggestions appendix

These are not in-scope acceptance requirements for this spec, but they would reduce future drift:

1. **Repo-wide epistemic architecture fitness function.** Add a Rust test or small internal tool that builds a manifest of epistemic authority surfaces, public fields, context constructors, debug constructors, and fixture fields, then checks them against a maintained allowlist. This is a suggestion because the current pass can use targeted source guards and compile-fail fixtures.
2. **Typed provenance graph v2.** Current provenance entries are inspectable and sorted, but future speech/records/institution work will need richer edge types. Keep this out of Phase-2A unless required by a public API sealing refactor.
3. **Debug-view capability review.** `DebugCapability::mint` is crate-private and tested (`crates/tracewake-core/src/debug_capability.rs:21-40`; `crates/tracewake-core/tests/hidden_truth_gates.rs:139-151`). Future work can centralize all debug view construction behind a single debug-report module to make review easier.
4. **Mutation testing of guards.** Add a controlled negative fixture or script that temporarily injects each banned epistemic public field/float/debug leak and proves the lock layer fails. This is stronger than source scanning but heavier than this pass requires.

## 12. Final self-check

- The spec is scoped to `joeloverbeck/tracewake` at exact commit `4c4dfff83aae01006e6a3b653a3248179e0f9b25` and makes no latest-main claim.
- The manifest was used only as path inventory.
- Archived specs were used only as historical evidence; their stale baseline SHAs and invariant numbers were not adopted.
- Findings cite live invariant numbers from `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`.
- The verdict is positive on material-risk grounds only.
- The physical Phase-1 spine is not re-audited; Phase-3A planner internals are not audited.
- Each material finding has code-yields corrective direction and a structural lock.
- No upstream doc amendment is required.
- The acceptance wording contributes scoped evidence toward `EPI-CERT`/`P0-CERT` and does not overclaim certification.
