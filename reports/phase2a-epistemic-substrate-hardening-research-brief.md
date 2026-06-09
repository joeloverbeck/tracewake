# Research Brief — Phase 2A Epistemic Substrate Alignment & Anti-Contamination Hardening (Tracewake)

> **For ChatGPT-Pro Session 2 (the deep researcher).** You are *locked*: produce the deliverable
> directly. Do not interview, do not ask clarifying questions — the requirements below are final.
> Upload bundle = this prompt + the manifest `manifest_2026-06-09.txt`.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-09.txt`) is the path inventory of the `joeloverbeck/tracewake`
repository — a causality-first living-world simulation in Rust: an event-sourced kernel, **subjective
epistemics** (agents act only on their own beliefs, observations, and modeled channels — never on hidden
ground truth), fallible institutions, ordinary agents, and a TUI-first client surface where every meaningful
change leaves a replayable trace. The workspace is three crates with a strict one-way dependency direction:
`tracewake-core` (authoritative kernel, **zero dependencies**) → `tracewake-content` (fixtures / loading /
schema validation, depends on core) → `tracewake-tui` (terminal boundary, depends on core + content).

Docs are layered authority: `docs/0-foundation` → `docs/1-architecture` → `docs/2-execution` →
`docs/3-reference` → `docs/4-specs`. **Earlier tiers govern later ones.** If execution conflicts with
architecture or foundation, execution is wrong; if implementation is more convenient than the accepted
gates, implementation is wrong. The whole adjudication rule lives in `docs/README.md`.

**Fetch every file from commit `4c4dfff83aae01006e6a3b653a3248179e0f9b25`** — the manifest reflects that
exact tree (verified repo HEAD). Construct every raw URL as
`https://raw.githubusercontent.com/joeloverbeck/tracewake/4c4dfff83aae01006e6a3b653a3248179e0f9b25/<manifest path>`.
The manifest is **path inventory only** — never source text or authority. Archived specs cite their own
historical baseline commits inside their evidence ledgers (e.g. the Phase-2A implementation spec cites
`c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056`); those are *that spec's own* provenance and **predate** the
current HEAD and every later merge — including the post-overhaul doctrine rewrite and the four-pass Phase-1
hardening campaign. **Ignore them as fetch targets — fetch everything from `4c4dfff…`.** Branch names,
default-branch lookups, repository metadata, and code-search snippets are not proof of target-commit content.

**This brief advances an iterative anti-contamination campaign into a NEW block.** The maintainer
re-hardens each coherent code block until nothing more needs hardening. The campaign so far, staged as the
visible sequence `0002 → 0003 → 0004 → 0005` (all completed, merged, and archived under `archive/specs/`),
hardened **only the Phase-1 / Phase-1A physical/kernel/TUI spine**:

- `archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` and
  `archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` — TUI-seam and
  kernel-spine structural hardening; the spine pass added the **structural-lock layer** (conformance
  capstones, an invariant-reference linter, a banned-API token scanner, a `clippy.toml`
  disallowed-types/methods profile, content forbidden-construct guards).
- `archive/specs/0004_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md` — a third pass that re-audited
  the spine **and the lock layer itself**, closing structural bypasses.
- `archive/specs/0005_PHASE_1_DOC_CODE_ALIGNMENT_CONFORMANCE_AND_ANTI_DRIFT_SPEC.md` — a fourth pass: a
  systematic doc↔code alignment-conformance walk of the Phase-1 block.

**Every one of those four passes deliberately bracketed the Phase 2A epistemic substrate OUT of scope.**
That substrate — the code implemented by
`archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md`
— has **never been alignment-audited or hardened.** It landed under the *pre-overhaul* foundation/architecture/
execution doctrine, and the live execution tier now brackets it as **landed-but-uncertified history**
(`docs/2-execution/01` and `…/03`: "Phase 2A landed → Evidence for `EPI-CERT`, **not certification**").

**This pass is therefore the FIRST alignment + hardening + anti-contamination pass over the Phase 2A
block** — the analogue, for the epistemic substrate, of what `0002 → 0005` did for the spine, **compressed
into one combined pass.** It is *not* a continuation of the lock-layer-robustness lineage and *not* a
re-audit of already-hardened Phase-1 code. The epistemic substrate is the load-bearing realization of
Tracewake's central claim — *belief comes before truth; agents act only on what they have modeled-channel
access to, never on authoritative state* — so contamination here is the most dangerous kind in the project.

The maintainer's standing motivation: past projects "devolved into entropy" when later implementers built on
**non-aligned** code. So beyond finding and correcting any current foundations-violation or drift, the
deliverable must, *for each finding*, specify a structural mechanism that makes it **impossible (to the
extent practical)** for future code to re-introduce that violation.

---

## 2. Read in full (authority order)

Read these completely, in this order, before producing anything. Each tier's **primary** (Phase-2A-load-bearing)
documents are called out; the remaining documents in each folder are read for **boundary-awareness** — so that
Phase-1 spine doctrine (already certified-as-hardened) and Phase-3A/4 doctrine (later, uncertified) are
recognized as **out of scope** and never mis-audited (see §3.2–§3.3). Confirm actual module/file paths against
the manifest before citing; do not assume names.

**Doctrine — foundation (`docs/0-foundation/`, read 00–14):**
- `docs/README.md` — the authority-layering rule and how to resolve cross-tier conflicts; **the audit's
  adjudication method.**
- `docs/0-foundation/00_FOUNDATION_INDEX.md` — anti-drift rules, reading order, "do not cherry-pick."
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — the `INV-001…INV-NNN` contract. **Every finding and
  every requirement must cite the exact invariant(s) it bears on, verified against this file — never against
  numbers quoted in the archived Phase-2A spec, whose numbering predates the overhaul.** Especially the
  epistemic family: belief-before-truth, no-privileged-protagonist, possession-transfers-no-knowledge,
  absence-needs-a-channel, typed-propositions-as-epistemic-currency, important-beliefs-need-provenance, and
  the embodied/debug-separation invariants. **Re-derive the exact numbers from the file.**
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — **primary, the controlling epistemics
  doctrine:** claims/propositions, beliefs, observation-vs-interpretation, memory, provenance, information
  flow. The bar the substrate is measured against.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **primary:** the truth
  firewall — where truth may *validate* but is forbidden to *plan or render*; actor-known cognition. The
  anti-contamination backbone for this entire block.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **primary:** TUI-as-proof-surface, embodied
  (actor-known notebook) vs. debug (non-diegetic ground truth) split, view-model boundary, possession parity,
  two-layer why-not.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **primary:** meaningful change ⇒ event;
  append-only log; rebuildable projections; deterministic replay. Governs epistemic events and the epistemic
  projection rebuild.
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — **primary:** no-scripting /
  no-authored-outcome / **seed-not-script**; the rule that an authored-prehistory belief seed must carry
  provenance and never be a smuggled global truth flag. "No simulation fact born from prose."
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **primary:** the first-proof
  (Missing Expected Property) scope and acceptance gates — fixes what Phase 2A is responsible for vs. what
  belongs to later phases.
- `docs/0-foundation/{01_PROJECT_CHARTER, 05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING,
  06_ACTIONS_AFFORDANCES…, 07_INSTITUTIONS…, 10_SCALE_LOD…, 11_LLM_SPEECH_ACTS…, 13_RESEARCH_DECISIONS…}.md`
  — boundary-awareness; `05` describes the Phase-3A planner/needs systems that *consume* epistemics but are
  out of scope (see §3.3); the rest bound later phases.

**Doctrine — architecture (`docs/1-architecture/`, read 00–14):**
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **primary:** how architecture binds; the universal conformance
  questions to apply.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — **primary:** kernel authority, the one-way
  crate dependency direction (`core` zero-dep), no-hidden-mutation — the substrate the epistemic module must
  live within (epistemics belong in `core`, never in TUI/content).
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **primary:** event log, replay, projections, save
  packages, randomness/seeding, schema versioning/migration — directly governs the epistemic event stream and
  epistemic projection rebuild.
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **primary, the controlling firewall
  architecture:** holder-known contexts, provenance packets, context sealing, contamination gates. The
  substrate's `KnowledgeContext`/belief/observation/provenance design must conform to this.
- `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — **primary, the controlling
  epistemics architecture:** the architectural contract for propositions, observations, beliefs,
  contradictions, memory traces, and information flow.
- `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — **primary:** the proposal → pipeline →
  event-apply flow and no-direct-dispatch rule (the `check_container`/accuse-probe actions and every epistemic
  event must flow through it; `KnowledgePreconditionNotMet` is a typed why-not here).
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — **primary:** the client-boundary contract
  the actor-known notebook and debug-epistemics view models must satisfy; possession switches input only.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **primary:** acceptance/observability/
  review-artifact requirements and the anti-contamination test obligations.
- `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — **boundary-awareness, the
  consumer boundary:** read it to understand how the Phase-3A planner consumes actor-known context, so you can
  verify the epistemics-side firewall at that seam **without** auditing planner internals (§3.3).
- `14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` — boundary-awareness; directly useful for not mis-reading
  epistemics doctrine.
- `{07_SPEECH_ACTS…, 08_INSTITUTIONS…, 09_ORDINARY_LIFE…, 11_INCIDENTS_LEADS…, 12_LOD_REGIONAL…}.md` —
  boundary-awareness; later-phase systems. `09_ORDINARY_LIFE…` may overlap the content seam (settlement
  ontology / missing-property first proof) — read it where the content schema touches it.

**Doctrine — execution (`docs/2-execution/`, read 00–13):**
- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — **primary:** the canonical gate vocabulary (`P0-DOC`, `P0-CERT`,
  `SPINE-CERT`, `EPI-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`)
  and the rule that execution may not soften foundation/architecture.
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — **primary:** the code-audit
  boundary, "archived specs are historical evidence only," the durable-promotions list, the `P0-CERT` proof
  obligations, and the three admissible spec postures.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **primary:** the gate order; **`EPI-CERT`**
  ("Certify actor-known/holder-known contexts, beliefs, observations, provenance, possession parity, view
  models, and debug quarantine") — **the gate this pass maps to most directly** — and that "Phase 2A landed"
  is *evidence for*, not satisfaction of, `EPI-CERT`.
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **primary:** the mandatory
  anti-contamination gates every future spec must carry — the list this pass's gates extend, never redefine.
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **primary, the controlling execution doc for this
  block:** the `EPI-CERT` proof obligations — actor-known filtering, possession parity, notebook/debug split,
  debug quarantine — the substrate must meet.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — **primary:** the scheduler/pipeline/
  no-direct-dispatch contract the check/accuse actions and the no-human epistemic run must satisfy.
- `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — **primary:** the data/schema/provenance/validation
  gates the epistemic seed schema (initial beliefs/expectations, observations) and content validator must
  satisfy; no prose-born facts; no shortcut truth fields.
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — **primary:** the golden-fixture and
  replay-acceptance contract the epistemic fixtures must meet.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **primary:** typed provenance over
  string-prefix/substring/display-label diagnostics; adversarial negative gates, not only friendly goldens —
  the bar for the substrate's tests **and** for judging whether epistemic leak-tests are sound or brittle.
- `{02_FIRST_PROOF_SCOPE…, 06_ORDINARY_LIFE_NEEDS_ROUTINES…, 11_INSTITUTIONS_RECORDS…,
  12_DEFERRED_SECOND_PROOF…, 13_RESEARCH_DECISIONS…}.md` — boundary-awareness; `02` helps fix the Phase-2A
  scope line; `06`/`11` bound the Phase-3A/4 systems that are out of scope.

**Doctrine — reference (`docs/3-reference/`, read 00–02):**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **primary:** the standing review checklist the deliverable's
  acceptance section should align to; exact-source discipline.
- `01_DESIGN_RISK_REGISTER.md` — **primary:** named relapse risks (debug-truth leakage,
  display-string-as-authority, nondeterminism, weak traceability, `PlayerCharacter`/`Quest`/`Objective`/
  `Reward`, omniscient-culprit shortcuts) the hardening must keep closed.
- `02_GLOSSARY.md` — **primary:** prescriptive terminology (`holder-known` vs `actor-known`, world vs
  non-world event, observation vs interpretation, etc.) — use precisely; mis-terminology is itself a drift
  vector.

**Spec tier (`docs/4-specs/`):**
- `SPEC_LEDGER.md` — **primary:** authority posture, source discipline, the archived-spec status table (note
  the Phase-2A spec is listed as "landed historically; not a current gate definition"), the live spec set
  (only `0001` is live), and "next known execution move = `P0-CERT`." **Derive the deliverable's number/path
  from this + the README.**
- `README.md` — **primary:** the rules every future spec must obey (posture declaration; gate codes as
  cross-references only; holder-known/actor-known terminology; archived specs as history; no files for
  symmetry).
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — the one live sibling spec; model its
  structure, posture declaration, and tone.
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — the live review-artifact template; the acceptance section should
  conform to it (exact-commit scoped wording; no over-claim).

**Prior campaign work (historical evidence only — NOT live authority):**
- `archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md`
  — **the implementation intent for the block under audit (primary historical target).** Read it in full: its
  data-model, event-model, action/pipeline, content-schema, view-model, replay, and fixture requirements are
  what the code claims to implement. **But treat its doctrine references and `INV-###` numbers as pre-overhaul
  and stale** — re-adjudicate everything against the *live* tiers above. It is not certification; it does not
  define a current gate.
- `reports/phase1-doc-code-alignment-research-brief.md` and `reports/phase1-third-hardening-research-brief.md`
  — **the two predecessor briefs** (the only ones present at this commit). Read them to inherit the campaign's
  method, house style, source discipline, and settled conventions (material-risk bar, form-follows-verdict
  deliverable, structural-lock ladder, posture vocabulary) — and to confirm exactly what the prior passes
  scoped so you do not re-commission settled Phase-1 work.
- `archive/specs/0005_PHASE_1_DOC_CODE_ALIGNMENT_CONFORMANCE_AND_ANTI_DRIFT_SPEC.md`,
  `archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md`,
  `archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`,
  `archive/specs/0004_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md` — the completed Phase-1
  hardening passes. Read them to learn **which structural-lock mechanisms already exist** (conformance
  capstones, invariant-reference linter, banned-API token scanner, `clippy.toml` profile, forbidden-content
  guards, hidden-truth gates, event-schema/replay gates) so your epistemic anti-contamination requirements
  **extend** those mechanisms rather than inventing parallels (§3.6).

> **Do NOT read the Phase-3A archived specs (`archive/specs/0005–0008`, needs/routines/no-human/ordinary-life)
> as targets.** Their code (`crates/tracewake-core/src/agent/**`) is out of scope (§3.3). Consult them only to
> fix a boundary line when a seam you are auditing is shared between the epistemic substrate and its Phase-3A
> consumers.

**Code seams to inspect (read directly from the commit; cite `file:line`, do not paste wholesale).** Confirm
exact paths against the manifest — names below were observed during repo exploration but verify before citing.
Scope is the **Phase-2A epistemic substrate**; the epistemic surfaces of shared Phase-1 files are in scope,
the physical surfaces are not (§3.2); the Phase-3A consumer modules are boundary markers only (§3.3).

- **Epistemic substrate (PRIMARY, in scope):**
  `crates/tracewake-core/src/epistemics/{mod, proposition, observation, belief, contradiction,
  knowledge_context, knowledge_basis, projection}.rs` — typed propositions, observations, source-backed
  beliefs, contradictions, the `KnowledgeContext` actor-known filter, the knowledge-precondition basis for
  why-not, the epistemic projection + actor-known notebook + debug-epistemics builders, canonical
  serialization / stable ordering.
- **Epistemic actions (in scope):**
  `crates/tracewake-core/src/actions/defs/{checkcontainer, accuseprobe}.rs` and the epistemic surfaces of
  `crates/tracewake-core/src/actions/{pipeline, proposal, registry, report}.rs` (the activated
  knowledge/perception slot, `CheckContainer` effect, `KnowledgePreconditionNotMet` typed reason, source-context
  validation, no-direct-dispatch boundary). Audit only the epistemic surface of these shared files.
- **Epistemic event / state / projection surfaces of shared spine files (in scope = epistemic surface only):**
  `crates/tracewake-core/src/events/{envelope, apply, mod}.rs` (epistemic event kinds — `InitialBeliefSeeded`,
  `ObservationRecorded`, `BeliefUpdated`, `ExpectationContradicted`, `ContainerChecked`/sound variants — their
  stream classification, and that physical `apply_event` never silently mutates beliefs);
  `crates/tracewake-core/src/state.rs` (epistemic projection placement; that beliefs are not global flags on
  physical entities; `BTreeMap`/`BTreeSet` ordering); `crates/tracewake-core/src/projections.rs`,
  `crates/tracewake-core/src/view_models.rs` (notebook + debug-epistemics view models; actor-known filtering);
  `crates/tracewake-core/src/replay/{mod, rebuild, report}.rs` (epistemic projection rebuild, version
  rejection, determinism); `crates/tracewake-core/src/scheduler.rs` (no-human epistemic check path);
  `crates/tracewake-core/src/{debug_capability, debug_reports}.rs` (debug-epistemics quarantine, non-diegetic
  labeling); `crates/tracewake-core/src/ids.rs` (epistemic stable-ID discipline).
- **Content (epistemic surface, in scope):**
  `crates/tracewake-content/src/{schema, validate, serialization, load}.rs` — the epistemic seed schema
  (initial beliefs/expectations, observation seeds), the validator's epistemic rules (provenance required,
  reference validity, forbidden shortcut-truth fields, no prose-born facts, schema-version rejection),
  canonical round-trip; and the epistemic fixtures
  `crates/tracewake-content/src/fixtures/{strongbox_001 (Phase-2 surface), expectation_contradiction_001,
  possession_parity_001, view_filtering_001, knowledge_blocker_accuse_001, sound_uncertainty_001,
  no_human_epistemic_check_001}.rs`.
- **TUI (epistemic surface, in scope):**
  `crates/tracewake-tui/src/{input, render, debug_panels, app, transcript}.rs` — `notebook`, `debug
  epistemics`, `debug beliefs <actor>`, `debug observations <actor>` parsing/rendering; embodied leak
  behavior; typed command errors.
- **The lock layer — the epistemic dimensions of it (in scope: re-verify it covers epistemics and is run by
  CI on the pinned toolchain):**
  `crates/tracewake-core/tests/{hidden_truth_gates, forbidden_content (content crate), anti_regression_guards,
  doc_invariant_references, event_schema_replay_gates, spine_conformance, golden_scenarios,
  negative_fixture_runner, acceptance_gates}.rs`;
  `crates/tracewake-content/tests/{forbidden_content, schema_conformance, fixtures_load,
  golden_fixtures_run}.rs`;
  `crates/tracewake-tui/tests/{adversarial_gates, tui_seam_conformance, embodied_flow, transcript_snapshot}.rs`
  (confirm exact names against the manifest). `clippy.toml`, `.github/workflows/ci.yml`, root `Cargo.toml`,
  `rust-toolchain.toml`. **Ask of each guard: does it cover the epistemic substrate at all, or only the
  physical spine? Where epistemics is uncovered, that is a finding.**
- **Boundary markers only — NOT audit targets** (recognize them so you neither audit nor mis-flag uncertified
  later-phase code): `crates/tracewake-core/src/agent/**` (the Phase-3A planner, needs, routines, HTN,
  intentions, candidate generation, `actor_known.rs` decision context, transaction, trace), and the Phase-3A
  `actions/defs/{eat, sleep, work, continue_routine}.rs`. At the epistemics→`agent` seam, verify the
  *epistemics-side public API* only — do not enter planner internals (§3.3).

---

## 3. Settled intentions (final — these pre-empt every question)

These decisions came out of a full repo-grounded interview. Treat them as committed.

1. **One combined pass: alignment-correction + hardening + anti-contamination, in a single deliverable.** Do
   a systematic doc↔code **alignment-conformance walk** of the Phase-2A epistemic substrate against the
   realigned doctrine tiers (foundation → architecture → execution → reference → specs) — for each in-scope
   mandate, a conformance verdict with `file:line` evidence on the doc side (tier + file:line + `INV-###`) and
   the code side — **and** specify the structural hardening + per-finding anti-contamination locks the block
   needs. This is the first pass over this block; do not split it into separate lenses.

2. **Code scope = the Phase-2A-delivered epistemic substrate — ONLY.** In scope: `epistemics/**`; the
   `check_container`/accuse-probe actions; the epistemic event kinds and their apply/projection/replay; the
   epistemic content schema, validation, serialization, and the epistemic fixtures (`expectation_contradiction_001`,
   `possession_parity_001`, `view_filtering_001`, `knowledge_blocker_accuse_001`, `sound_uncertainty_001`,
   `no_human_epistemic_check_001`, and the Phase-2 surface of `strongbox_001`); the actor-known notebook and
   debug-epistemics view models and TUI commands. **For shared Phase-1 files** (events, `state`, `projections`,
   `view_models`, `validate`, `pipeline`, `scheduler`) **audit only the epistemic surface** — the physical
   spine is already hardened across `0002 → 0005` and is **not** re-audited here. If a seam is genuinely shared,
   audit only its Phase-2A surface and say so explicitly.

3. **Consumer seam: enforce the firewall on the epistemics side; verify the seam only.** The point of the
   substrate is "agents act only on beliefs, not authoritative state," and the thing that *acts* is the
   Phase-3A planner/decision code in `crates/tracewake-core/src/agent/**` (incl. `actor_known.rs`) — which is
   **later-phase, landed-but-uncertified, and out of scope.** Prove that the epistemic substrate's **public
   API exposes only actor-known-filtered, provenance-bearing data** (no method hands a consumer raw ground
   truth, another holder's private beliefs, possession history, or debug truth). Verify the boundary holds at
   the epistemics→`agent` seam, but **do not audit planner/agent internals** and do not flag uncertified
   Phase-3A code as "misaligned."

4. **Skeptical-of-clean framing on a material-risk-only warrant bar; an honest "aligned" is acceptable.** The
   substrate landed under *pre-overhaul* doctrine and is bracketed as landed-but-uncertified, so genuine
   foundations-violations or drift are **plausible** — approach the code skeptically and do not presume the
   archived spec's intent was faithfully or fully implemented, nor that it still matches the realigned
   doctrine. But a correction/hardening spec is warranted **only** for findings that materially threaten the
   three goals: (a) the substrate conforms to `docs/**`; (b) because it conforms and is locked, it is the best
   foundation for Phase-2B+/Phase-3 work that builds on beliefs; (c) it is *structurally impossible*, to the
   extent practical, for later implementations to break that alignment. Cosmetic/stylistic items are not
   warrant-worthy — suggestions appendix. A well-evidenced "no material misalignment" verdict is a fully
   acceptable outcome and must not be inflated into a spec.

5. **Drift direction = code yields by default; genuine doc defects flagged separately as Required upstream
   amendments.** When code and docs diverge, the **default corrective action is changing code to match the
   docs** (foundation/architecture/execution outrank code). But where a doc is **genuinely the stale/wrong
   party** — e.g. the realigned doctrine left an epistemics gap, or a foundational invariant needs amending to
   admit a sound design — the spec must NOT silently code-around it: it records a **Required upstream
   amendment** to the specific foundation/architecture/execution document in a clearly-labeled, separate
   section, because a `4-specs`-tier spec **cannot** amend a higher tier. Justify every doc-is-wrong call with
   file:line + invariant reasoning; default to code-yields unless the doc defect is demonstrable. (The
   maintainer explicitly wants foundational-doc-amendment needs surfaced if any exist.)

6. **Anti-contamination = scoped structural lock per finding, extending the existing lock layer; general
   harness as a suggestion only.** For each finding, specify a durable structural mechanism that makes that
   contamination/drift impossible to reintroduce, pushed up the **convention → test → compile-time** ladder
   wherever practical (type-state, sealing, capability tokens, private constructors, `clippy.toml
   disallowed-*`, conformance test, schema/replay-coverage parity, CI cross-check). **Where an existing
   Phase-1 lock-layer mechanism already covers a dimension, extend it to the epistemic substrate** (e.g. add
   epistemic forbidden-construct fields to the content guard; extend hidden-truth/leak gates to notebook and
   debug-epistemics surfaces; extend event-schema/replay gates to epistemic event kinds and the epistemic
   projection version) rather than inventing a parallel mechanism — and name the file. Any **repo-wide
   epistemic-conformance harness** (executable-spec / architecture-fitness-function style) goes in an
   **additional-suggestions appendix**, flagged for the maintainer, not as an in-scope deliverable.

7. **Determinism is a first-class epistemic risk.** Confidence values (on observations and beliefs) must use a
   deterministic canonical encoding — a bounded integer scale or fixed-precision decimal, **not** raw
   floating-point `Display` (a determinism hazard for canonical serialization, checksums, and replay). All
   epistemic collections (observations/beliefs/contradictions/notebook entries by id/holder/actor) must use
   ordered maps/sets or explicit sorting, never hash-map iteration order. Treat any float-formatted confidence
   or hash-ordered epistemic collection as a finding tied to the determinism/replay invariants.

8. **Locked posture, source discipline, no scope creep.** Archived specs are historical evidence only. **Do
   not advance the campaign into later blocks** — no Phase-3A needs/routines/planner/ordinary-life, no
   institutions/records/wrong-suspicion (Phase 4), no Phase-2B richer perception/memory-decay/testimony/speech,
   no LLM surfaces, no graphical client, no new world mechanics. Gate codes (`EPI-CERT`, `SPINE-CERT`,
   `P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`, …) appear only as
   **cross-references** — never redefined or weakened. Use `holder-known context` as the system-wide term and
   `actor-known` for the actor case. No backwards-compat shims or alias paths.

9. **`assumption:`** the deliverable, if a spec, is staged as
   `specs/0006_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md` (number chosen
   to continue the visible `0002 → 0003 → 0004 → 0005` staging epoch — the most recent contiguous series,
   cross-checked against `SPEC_LEDGER.md`; the staging directory `specs/` is currently empty), with accepted
   implementation-spec home `archive/specs/0006_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`.
   **Confirm the number and path against the live `SPEC_LEDGER.md` + `4-specs/README.md` at
   the target commit and state your choice** — note that `archive/specs/` already contains a differently-scoped
   `0006_*` (Phase-3A) from the earlier epoch, so a same-number collision in the archive is expected by the
   staging→archive convention and is **not** a blocker. If the maintainer prefers a different number/name or to
   place it directly in `docs/4-specs/`, only the header/path changes (one-line correctable). If the verdict is
   "aligned", name the rationale report `reports/phase2a-epistemic-substrate-audit.md` and say so.

---

## 4. The task

Conduct the **first alignment + hardening + anti-contamination pass** (target type: *hardening /
anti-contamination*; secondary: *new spec*) over the Tracewake **Phase 2A epistemic substrate** — the code
implemented by the archived Phase-2A spec and present at the target commit — against the full live doctrine
tree (`docs/0-foundation` → `docs/4-specs`), with `file:line` evidence on both the doc and code sides and
skeptical scrutiny appropriate to code that landed under superseded doctrine and was never certified. For each
in-scope doctrine mandate that bears on the substrate, determine whether the code conforms; for each genuine
foundations-violation or misalignment, determine the corrective direction (code-yields by default; doc defects
flagged as Required upstream amendments per §3.5) and a structural lock that prevents recurrence (§3.6). Verify
the truth firewall holds at the substrate's public API where the Phase-3A planner consumes it (§3.3). On the
material-risk-only bar (§3.4), decide whether a correction + anti-contamination spec is warranted, then produce
**exactly one** self-contained markdown document whose **form follows the verdict** (§3.4, §7). A warranted
spec must be the kind a later `spec-to-tickets` pass could decompose without re-litigating intent.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files in §2 — follow `use`/`mod` paths into any in-scope
seam, and read whatever fixtures/tests prove (or fail to prove) conformance and contamination-resistance.
Where you assess a guard, *trace what it actually walks and matches* rather than trusting its name. Where you
assert a doc mandate, quote the controlling sentence (tier + file:line).

Research online as deeply as needed and **cite sources** for any external claim that shapes a decision.
High-value directions for *this* target — subjective epistemics, information firewalls, and making belief/truth
separation structurally impossible to breach:

- **Belief–desire–intention (BDI) and epistemic-logic agent architectures** — modeling agents that act on a
  *belief base* distinct from world state; partial/uncertain knowledge; belief revision (AGM theory) and how
  an observation that contradicts a prior belief is incorporated; provenance/justification of beliefs.
- **Information-firewall / non-interference / capability-based security** — keeping privileged "ground truth"
  structurally unable to reach a derived/presented surface; object-capability discipline; how a type system or
  module boundary can make "the actor-known view cannot read authoritative state" a *compile-time* property
  rather than a tested convention.
- **Per-actor / fog-of-war view derivation in simulations & multiplayer games** — deriving each observer's
  knowledge from a shared authoritative log without leaking unobserved facts; server-authoritative vs.
  client-visible state; how engines prevent "wallhack"-style information leaks structurally.
- **Event-sourcing for derived read models** — observations/beliefs/contradictions as events; projections as
  rebuildable read models; versioned events + upcasting; ensuring a projection rebuild is deterministic and
  rejects unsupported schema versions instead of silently repairing.
- **Deterministic simulation** — canonical sources of nondeterminism (hash-map iteration order, floating-point
  formatting, wall-clock, unseeded RNG) and how engines structurally exclude them; canonical encoding of
  confidence/uncertainty values for replay-stable checksums.
- **Making invariants structural, not just tested (Rust)** — type-state, sealed traits, newtypes,
  private-constructor / "parse, don't validate", capability tokens to make illegal states (a belief without
  provenance; an embodied view reading truth) unrepresentable; enforcing layering with visibility,
  `cargo-deny`, clippy `disallowed-methods`/`disallowed-types`; architecture-fitness-function / ArchUnit-style
  conformance tests and their Rust analogues; limits of grep/token-scanner guards (aliasing, re-export, macros,
  conditional compilation, unwalked files) and how to test a guard's own coverage (negative fixtures that
  *should* trip it; mutation testing).

Use external prior art to sharpen the deliverable's *findings, requirements, and structural gates* — not to
import scope the intentions forbid.

---

## 6. Doctrine & constraints (honor these)

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every finding and requirement must
  cite the exact `INV-###` it bears on (verified against the file, **not** numbers quoted in the archived
  Phase-2A spec, whose numbering predates the overhaul). A genuine divergence requires amending an invariant
  first (a Required-upstream-amendment flag, never designing against it silently).
- **Authority order:** foundation → architecture → execution → reference → specs. If execution conflicts with
  architecture/foundation, execution is wrong; if implementation is more convenient than the accepted gates,
  implementation is wrong. A spec may operationalize higher-tier doctrine; it may **not** amend invariants,
  replace architecture, define/weaken gate semantics, or promote archived history into certification.
- **Anti-contamination is the point — and epistemics is where it bites hardest:** no simulation fact may be
  born from prose; actors act only on modeled-channel beliefs/observations/memory/records/testimony, never on
  hidden ground truth; absence becomes evidence only through expectation/perception/instruction/intentional
  search; important beliefs carry full provenance; possession transfers no world knowledge; observation is not
  interpretation (a sound is not knowledge of theft). Debug/ground truth is quarantined and structurally unable
  to feed the actor-known notebook, embodied affordances, actor decisions, or tests-masquerading-as-knowledge.
- **Diagnostics:** typed provenance and typed reason codes (e.g. `KnowledgePreconditionNotMet`) — never
  string-prefix, substring, or display-label behavior — as the basis for reason reporting and audit findings.
- **Determinism:** no wall-clock, OS/process randomness, network, filesystem-during-resolution, thread races,
  `HashMap`/`HashSet` iteration order, **float-formatted confidence**, or terminal-frame timing in any
  outcome-affecting/serialized/replayed path. Any randomness must be seeded, scoped, recorded in events, and
  replay-checked.
- **No new mechanics, no scope creep, no later-block advance, no LLM surfaces, no backwards-compat shims or
  alias paths, no graphical client.**
- Workspace gates the resulting work must ultimately satisfy (state them in the deliverable's acceptance
  section; do not run them): `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D
  warnings`; `cargo build --workspace --all-targets --locked`; `cargo test --workspace`. Any new lint/CI/
  conformance gate the deliverable introduces must be expressible within the pinned `rust-toolchain.toml`.

---

## 7. Deliverable specification

Produce **one new downloadable markdown document — always**, whose form is contingent on the verdict (§3.4).
It is a new file: it does not replace any existing file, and must not edit, restate-as-authority, or rewrite
any doc in `docs/0-foundation`, `docs/1-architecture`, `docs/2-execution`, `docs/3-reference`, the archived
specs, or code. It references them.

**This is a determination-plus-conditional deliverable.** Session 2 must (a) produce a clearly labeled,
evidence-based **verdict** (are there genuine Phase-2A foundations-violations / doc↔code misalignments / unlocked
contamination paths, and what are they) on the material-risk-only bar of §3.4, and (b) shape the document
accordingly. The verdict is stated up front; the rest of the document is consistent with it.

**Branch A — verdict POSITIVE (one or more material findings warrant a spec).** Produce a **Phase-2A epistemic
alignment-correction + anti-contamination hardening spec**:

- **Filename (assumption, confirm against the live ledger per §3.9):**
  `0006_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`
- **Intended path:** `specs/0006_…md` (staging), accepted implementation-spec home `archive/specs/0006_…md`.
- Minimum contents:
  1. **Header & baseline statement** — repository, analyzed commit
     `4c4dfff83aae01006e6a3b653a3248179e0f9b25`, the source-discipline note (manifest = path inventory;
     archived baselines incl. `c7e562…` are historical), and exactly one declared execution admissibility
     posture — almost certainly **`P0-CERT scoped remediation`** (a spec that changes epistemic code/tests/CI
     and contributes scoped evidence toward `EPI-CERT`/`P0-CERT` is not `not applicable`); justify it from
     evidence; do not redefine gate semantics.
  2. **Authority chain & gate mapping** — the controlling foundation/architecture/execution/reference docs and
     the `EPI-CERT`/`SPINE-CERT`/`P0-CERT`/`TFW`/`PIPE`/`NO-DIRECT`/`POS-PARITY`/`REPLAY`/`FIXTURE`/`DIAG`
     mapping **as cross-references only** (`EPI-CERT` is the primary gate this pass feeds).
  3. **Scope & non-goals** — the Phase-2A epistemic substrate (per §3.2) + the epistemic surface of shared
     spine files + the consumer-seam verification (§3.3); explicit non-goals: the physical Phase-1 spine (already
     hardened), Phase-3A planner/agent internals, Phase-2B/3/4 systems, new mechanics, LLM, graphical client,
     redefining gate semantics, and **performing** higher-tier doc amendments inside this spec (those are
     flagged, item 7).
  4. **Alignment & contamination findings** — the conformance walk, organized by epistemic dimension: typed
     propositions (no stringly global flags); observation model (channel-specific, observation≠interpretation,
     uncertainty/alternatives for sound); source-backed beliefs + provenance completeness; expectation
     contradiction & absence-as-evidence (only with a relevant expectation); `KnowledgeContext` actor-known
     filtering (allowed vs forbidden sources); epistemic event stream & no-silent-belief-mutation; epistemic
     projection rebuild & version rejection; determinism (confidence encoding, ordered collections); content
     epistemic schema/validation/serialization (provenance required, forbidden shortcut-truth fields, no
     prose-born facts); actor-known notebook view model; debug-epistemics quarantine & non-diegetic labeling;
     possession parity (no cross-actor knowledge transfer); `KnowledgePreconditionNotMet` typed why-not;
     no-human epistemic run parity; crate dependency direction (epistemics in `core`); terminology/glossary
     conformance; **and the consumer-seam firewall (epistemics-side public API)**. For **each** mandate
     audited: the **doc side** (tier + file:line + `INV-###`), the **code side** (`file:line`), a verdict
     **aligned** vs **misaligned/unlocked**, and for each finding the **corrective direction** (code-yields vs
     flagged-upstream-doc-defect, §3.5). **Show evidence for *aligned* dimensions too** — the value of a first
     pass is the explicit walk, not only the exceptions. Mark each **already-satisfied** vs **needs-work** with
     evidence for both.
  5. **Structural anti-contamination requirements** — for each finding, a durable, testable requirement that
     locks the corrected intent: the invariant(s) it enforces, the contamination/drift it prevents, and —
     crucially — **how it is structurally enforced**, pushed up the convention → test → compile-time ladder
     (§3.6). Where an existing lock-layer mechanism already covers the dimension, **extend it and name the
     file**; where the epistemic substrate is currently *uncovered* by the lock layer, that is itself a
     requirement.
  6. **Required fixtures & tests** — positive **and adversarial/negative** gates, including **negative fixtures
     that should trip each new/extended guard** (proving the guard's own coverage): belief-without-provenance
     rejected; prose-born/typeless proposition rejected; forbidden shortcut-truth field (`culprit`,
     `stolen_flag`, `npc_knows_truth`, `knows_mara_did_it`, …) rejected; absence-without-expectation produces
     no missing-property belief; possession switch transfers no belief; debug truth never appears in
     embodied/notebook output; unsupported epistemic schema version rejected on replay; float/hash-ordered
     epistemic determinism injection; no-human epistemic check parity. **Harden existing fixtures rather than
     author parallels.**
  7. **Required upstream doc amendments** (separate, clearly labeled) — any finding where the **doc** is the
     stale/wrong party (incl. any **foundational invariant** that must be amended to admit a sound epistemic
     design): name the exact foundation/architecture/execution document and the amendment the maintainer must
     make **before** the corresponding code requirement is valid. This spec records them; it does not perform
     them. If none, say so explicitly.
  8. **Acceptance checklist** — the four workspace gates, any new lint/CI/conformance gate, a per-requirement
     acceptance condition, and certification-result wording that conforms to `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
     and does not over-claim project certification (this pass contributes scoped evidence toward `EPI-CERT`,
     not `EPI-CERT` itself).
  9. **Additional-suggestions appendix** — the repo-wide epistemic-conformance harness (§3.6) and any
     cross-block mechanisms, flagged as suggestions; note which predecessor-brief suggestions landed vs. remain
     open.

**Branch B — verdict NEGATIVE (no material misalignment; the substrate is aligned and adequately locked).**
Produce a self-contained **rationale report**:

- **Filename (assumption, confirm/adjust):** `phase2a-epistemic-substrate-audit.md` under `reports/`.
- Minimum contents: the labeled verdict and its reasoning; the **conformance walk** with `file:line` doc-side
  (+ `INV-###`) and code-side evidence for each epistemic dimension above, proving alignment; an explicit
  demonstration that the truth firewall holds at the consumer seam (the epistemics-side public API exposes no
  raw truth/other-holder/possession/debug data); an explicit demonstration that the existing lock layer
  actually covers the epistemic substrate (or, if it does not but the code is nonetheless aligned, that gap
  noted as a suggestion); what the block contributes toward `EPI-CERT` in the live posture vocabulary; and a
  suggestions appendix for any non-warrant-worthy observations and the repo-wide harness. Given §3.4's
  skeptical-of-clean framing, a Branch-B verdict must be *earned* with an exhaustive walk, not asserted.

In both branches, keep the document self-consistent with `docs/4-specs/README.md`'s rules (posture declaration
where applicable; gate codes as cross-references; correct holder-known/actor-known terminology; archived specs
as history; no files for symmetry).

> **Locked / no-questions:** Produce the deliverable directly as a downloadable markdown document. Do not
> interview, do not ask clarifying questions — the requirements above are final. If a genuine contradiction
> makes a requirement impossible, state it inside the document and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] Every file in §2 was fetched from commit `4c4dfff83aae01006e6a3b653a3248179e0f9b25`; the manifest was
      used only to enumerate paths; archived-spec baselines (incl. the Phase-2A spec's `c7e562…`) were NOT used
      as fetch targets, and pre-overhaul `INV-###` numbers from the archived spec were NOT trusted.
- [ ] The deliverable is exactly one new markdown document whose **form matches the verdict** (correction/
      hardening spec if material findings; rationale report if aligned); it edits/replaces nothing and restates
      no upstream tier as local authority.
- [ ] A clearly labeled, **file:line-grounded verdict** is stated up front on the **material-risk-only** bar;
      the skeptical-of-clean framing was honored (an exhaustive walk, not a presumed "clean"); any positive
      verdict is justified by concrete, evidenced foundations-violation / misalignment / unlocked contamination.
- [ ] The audit is a **combined alignment-conformance + hardening + anti-contamination walk** of the Phase-2A
      substrate (not a Phase-1 re-audit and not a structural-only pass); each epistemic dimension carries
      doc-side (tier + file:line + `INV-###`, numbers verified against the live invariants file) **and**
      code-side (`file:line`) evidence; aligned and misaligned dimensions are both shown.
- [ ] **Scope held to the Phase-2A epistemic substrate.** The physical Phase-1 spine was not re-audited; the
      Phase-3A planner/agent internals were NOT audited and NOT mis-flagged; the consumer seam was verified on
      the **epistemics side only** (public API exposes no raw truth / other-holder beliefs / possession history
      / debug truth). No campaign advance into later blocks.
- [ ] For each finding, the **corrective direction** is stated (code-yields by default; genuine doc defects —
      including any foundational-invariant amendment the maintainer requested be surfaced — flagged as Required
      upstream amendments in a separate section, never code-around-a-stale-doc).
- [ ] For each finding, a **structural anti-contamination lock** is specified and pushed as far up the
      convention → test → compile-time ladder as practical; existing lock-layer mechanisms were **extended to
      cover epistemics** (named by file) rather than duplicated; epistemic dimensions currently uncovered by the
      lock layer are themselves findings; a repo-wide harness appears only as a flagged suggestion.
- [ ] Determinism (deterministic confidence encoding; ordered epistemic collections), event-sourced epistemic
      causality (no silent belief mutation in physical apply), epistemic schema-version/replay rejection,
      provenance-completeness, observation≠interpretation, absence-needs-a-channel, possession parity,
      typed `KnowledgePreconditionNotMet` why-not, and debug-epistemics quarantine are each tied to an invariant
      and to evidence (a test/fixture, or shown already covered).
- [ ] If a spec is produced: exactly one `P0-CERT …` posture is declared and justified (not defaulted to `not
      applicable`); gate codes appear only as cross-references; no gate semantics defined or weakened;
      acceptance wording conforms to the live acceptance-artifact template and does not over-claim certification.
- [ ] Terminology matches the glossary (`holder-known` / `actor-known`); no backwards-compat shims or alias
      paths; any new gate is expressible within the pinned `rust-toolchain.toml`.
- [ ] Every external claim that shaped a decision is cited; nothing in the deliverable weakens an upstream tier.
