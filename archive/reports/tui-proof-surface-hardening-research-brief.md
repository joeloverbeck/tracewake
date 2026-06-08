# Research Brief — TUI Proof-Surface Hardening & Debug-Quarantine Certification (Tracewake)

> **For ChatGPT-Pro Session 2 (the deep researcher).** You are *locked*: produce the deliverable
> directly. Do not interview, do not ask clarifying questions — the requirements below are final.
> Upload bundle = this prompt + the manifest `manifest_2026-06-08.txt`.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-08.txt`) is the path inventory of the `joeloverbeck/tracewake`
repository — a causality-first living-world simulation in Rust: an event-sourced kernel, subjective
epistemics, fallible institutions, ordinary agents, and a TUI-first client surface where every event
leaves a replayable trace. The workspace is three crates with a strict one-way dependency direction:
`tracewake-core` (authoritative kernel, zero deps) → `tracewake-content` (fixtures/loading/validation,
depends on core) → `tracewake-tui` (terminal boundary, depends on core + content).

Docs are layered authority: `docs/0-foundation` → `docs/1-architecture` → `docs/2-execution` →
`docs/3-reference` → `docs/4-specs`. **Earlier tiers govern later ones.** If execution conflicts with
architecture or foundation, execution is wrong; if implementation is more convenient than the accepted
gates, implementation is wrong.

**Fetch every file from commit `bf0e3a0392bb1217caf034281913487aa1644ad4`** — the manifest reflects that
exact tree (verified repo HEAD, clean working tree). Construct every raw URL as
`https://raw.githubusercontent.com/joeloverbeck/tracewake/bf0e3a0392bb1217caf034281913487aa1644ad4/<manifest path>`.
The manifest is **path inventory only** — never source text or authority. Note: the two archived specs
you will read (`archive/specs/0002_*`, `archive/specs/0003_*`) cite their own historical baseline commits
(`841deeb…`, `1d27a01…`) inside their evidence ledgers; those are *that spec's* provenance and predate the
foundation/architecture/execution overhaul. **Ignore them as targets — fetch everything from
`bf0e3a0…`.** Branch names, default-branch lookups, repo metadata, and code-search snippets are not proof
of target-commit content.

---

## 2. Read in full (authority order)

Read these completely, in this order, before producing anything. Each line says why it is load-bearing
*for this target* (a hardening + certification audit of the TUI / view-model / debug seam).

**Doctrine — foundation (binds product identity & invariants):**
- `docs/README.md` — the authority-layering rule; how to resolve cross-tier conflicts.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — the `INV-001…INV-098` contract; every requirement
  in your spec must satisfy it. Especially the TUI / possession / debug / why-not / no-script / determinism
  invariants (e.g. INV-004/005/006 possession & no-sacred-player, INV-008 UI-is-not-authority, INV-065/066
  TUI-first & per-phase TUI gate, INV-067/068 embodied-vs-debug & non-diegetic debug, INV-069 view-model
  boundary, INV-070 why-not, INV-071 hidden-mechanics-are-unfinished, INV-009 events, INV-017/018 determinism/replay,
  INV-091/092/093/094 no-human/seedable/debug-quarantine/possession — verify exact numbers against the file).
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **the controlling TUI doctrine.** TUI as
  the first real client and *proof surface* (legible, testable, honest); embodied vs debug mode; the
  view-model boundary flow; two-layer why-not discipline; notebooks/leads wording; "Future graphical client"
  deferral. This is the single most load-bearing doc for the deliverable.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — what must be reachable/visible through
  the TUI; the TUI/view-model acceptance gate; the "definition of done" list. Excludes graphical-client work
  from first proof.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — the truth firewall: where
  truth may *validate* vs. where it is forbidden to *plan or render*. The backbone of anti-contamination for a
  proof surface.

**Doctrine — architecture (binds the boundary the TUI must conform to):**
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — the client-boundary
  architecture: view-model flow, possession, debug separation, future-client reuse contract.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — holder-known contexts and
  provenance, the substrate for "who knows what / from what source / with what confidence" legibility.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — the proposal /
  validation / feedback pipeline the TUI submits into and whose why-not it surfaces; the no-direct-dispatch rule.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — acceptance,
  observability, and review-artifact requirements that set the audit's evidence bar.

**Doctrine — execution (binds current sequencing & gates):**
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — gate vocabulary (`P0-DOC`, `P0-CERT`, `SPINE-CERT`,
  `EPI-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`); universal
  execution posture; the rule that execution may not soften foundation/architecture.
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — the **`P0-CERT`**
  contents (10 proof requirements), the "archived specs are historical evidence only" rule, the code-audit
  boundary (explicitly names "TUI/debug panels" and "view-model rendering" as audit seams), and the three
  admissible spec postures (`P0-CERT passed` / `scoped remediation` / `not applicable`).
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — gate order; `SPINE-CERT`
  (event log, replay, projection, randomness, save, action pipeline, **TUI/debug split**, no-direct-dispatch)
  and `EPI-CERT` (actor-known contexts, possession parity, view models, debug quarantine); the four valid
  future-spec postures and the "unclear posture ⇒ treated as expansion" rule.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — the **mandatory
  anti-contamination gates** every future spec must carry; the load-bearing list your hardening gates extend.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — `EPI-CERT`: view-model filtering,
  possession parity, debug quarantine, and the exact seams a future code audit must inspect (view-model
  construction, notebook projection, possession binding, debug-panel construction, rejection-report construction).
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — testing/diagnostics
  discipline: **typed provenance over string-prefix/substring/display-label diagnostics**; adversarial negative
  gates, not only friendly golden paths. Sets the bar for the spec's required tests.

**Doctrine — reference:**
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — the standing review checklist the spec's
  acceptance section should align to.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — named design risks (PlayerCharacter/Quest/Objective/Reward,
  debug truth leakage, display-string-as-authority, etc.) the hardening must keep closed.
- `docs/3-reference/02_GLOSSARY.md` — canonical terms (holder-known vs actor-known, etc.) to use precisely.

**Spec tier:**
- `docs/4-specs/SPEC_LEDGER.md` — spec subordination, source discipline, archived-spec posture, "next known
  execution move = `P0-CERT`," and the required admissibility-posture vocabulary.
- `docs/4-specs/README.md` — the rules every future spec must obey (posture declaration; gate codes as
  cross-references only; `holder-known context` system-wide term and `actor-known` for the actor case; archived
  specs as history; no files for symmetry).
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — the one live sibling spec;
  model its structure, posture declaration, and tone. The new spec is the next live number (`0002`) after this.

**The two specs whose intent is being hardened (historical evidence only — NOT live authority):**
- `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` — the Phase-1
  kernel/TUI/event-log/replay slice: the TUI shell, view-model separation, mutation-via-events,
  why-not, debug separation, determinism, the shared pipeline, and the explicit boundary that the TUI
  "may be ugly… may not be fake." The *intent* you are hardening.
- `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` — wired the executable
  stdin/stdout command loop; its §Mutation boundary, §Debug boundary, §Numeric-selection requirement, and
  §Dependency policy (the clause forbidding `ratatui`/`crossterm` "unless a later spec explicitly justifies
  the dependency and test burden"). This is the clause your render-tech evaluation must address.

**Prior report — model the audit-artifact depth (style only, history not authority):**
- `archive/reports/PHASE_3A_IMPLEMENTATION_AUDIT.md` — an existing code-audit artifact; mirror its
  evidence rigor (file:line findings by responsible layer).

**Code seams to inspect (read directly from the commit; do not quote wholesale into the spec — cite
file:line). These are where the landed TUI intent actually lives today:**
- TUI crate: `crates/tracewake-tui/src/main.rs`, `lib.rs`, `app.rs`, `run.rs`, `render.rs`, `input.rs`,
  `debug_panels.rs`, `transcript.rs`, `launch.rs`; and `crates/tracewake-tui/tests/` (`command_loop_session.rs`,
  `embodied_flow.rs`, `readme_sample_session.rs`, `transcript_snapshot.rs`, `tui_acceptance.rs`);
  `crates/tracewake-tui/Cargo.toml`.
- Core seams the TUI consumes (decisive for the boundary): the `tracewake-core` modules for **view models**,
  **projections**, **debug reports**, the **action proposal/validation pipeline**, **events/apply**, and
  **replay**. Locate them from the manifest under `crates/tracewake-core/src/**` (e.g. `view_models*`,
  `projections*`, `debug_reports*`, `actions/pipeline*`, `events/apply*`, `replay/**`) and read the ones that
  define the view-model types, debug-report builders, why-not/rejection reports, and the no-direct-dispatch
  boundary. Confirm actual module paths against the manifest — do not assume names.

> Note on scope drift: the TUI crate has accreted panels from later archived specs (0004–0008: epistemic
> notebook, beliefs/observations, needs/routines/planner/stuck, no-human-day). Treat the **whole current TUI
> seam** as the audit surface — the "intent of 0002/0003" must be hardened against everything that now rides on
> the same crate — but keep the deliverable a *boundary* hardening, not a re-spec of epistemics or ordinary-life
> mechanics (those belong to `EPI-CERT`/`ORD-LIFE-CERT`, named only as cross-references).

---

## 3. Settled intentions (final — these pre-empt every question)

These decisions came out of a full repo-grounded interview. Treat them as committed:

1. **The deliverable is ONE hardening + certification spec**, not a build, redesign, or ticket set. Its job
   is to (a) audit the landed *intent* of archived specs 0002 and 0003 as the TUI / view-model / debug seam
   **actually stands today**, and (b) specify how to make that seam **maximally hardened and anti-contaminated
   for the future** — including the eventual richer client.

2. **"Prover TUI" means proof-surface legibility — explicitly NOT visual/render richness.** The hardening
   target is the qualities the docs already demand of the TUI as a proof surface: sharper actor-visible
   **why-not**, **causal-trace / provenance surfacing**, **"who knows what & why / from what source / with
   what confidence / how they may be wrong"** legibility, and **inspectability** — delivered *over the current
   string/stdout renderer*. Do not design panes, layouts, live-screen redraw, or key-driven navigation as the
   goal.

3. **Maximal hardening + anti-contamination is the spine.** The spec must enumerate durable, testable
   anti-contamination gates that lock the intent of 0002/0003 against future drift: debug-truth **quarantine**
   (no debug/ground truth reachable from embodied views, why-not text, or affordances); **no rule inference,
   target discovery, or action-availability computation in the TUI** (view models are the sole source of
   affordances); **no-direct-dispatch** (every world mutation flows TUI → `TuiApp` semantic action → proposal →
   shared pipeline → event apply; the TUI never calls the event applier or edits state); **possession changes
   input binding only** (no knowledge/needs/intention transfer across bodies); **view models strictly
   actor-known**; **typed provenance, not string-prefix/display-label**, for diagnostics; and **determinism**
   (no wall-clock, RNG, terminal-timing, or hashmap-iteration in outcomes; replay/transcript stability).

4. **Visual / render uplift is DEFERRED, not designed here.** Aligns with foundation 08 "Future graphical
   client" and execution 02 excluding graphical-client work from first proof. The spec may state the deferral
   boundary and what a future uplift must reuse (simulation core, action pipeline, actor-knowledge filters,
   view models, debug/replay architecture, acceptance doctrine) so the path stays safe — but it does **not**
   author the uplift.

5. **ratatui/crossterm: evaluate and recommend; do not pre-authorize.** Spec 0003 §Dependency policy forbade
   runtime terminal deps "unless a later spec explicitly justifies the dependency and test burden." Weigh that
   trade-off against doctrine (determinism, headless/scripted-stdin testability, the deferral in #4) and give a
   reasoned recommendation, leaving the decision to the user. Given intentions #2–#4, the expected conclusion is
   to **keep the string/stdout renderer** and not take the dependency in this spec — but state the reasoning and
   the conditions under which a future spec *should* revisit it, rather than asserting it by fiat.

6. **Gate mapping is named, never redefined.** The TUI seam maps to `SPINE-CERT` (TUI/debug split, action
   pipeline, no-direct-dispatch) and `EPI-CERT` (view-model filtering, possession parity, debug quarantine);
   the next execution move is `P0-CERT`. Reference these gate codes as cross-references only — the spec may not
   define or weaken gate semantics. **Declare exactly one execution admissibility posture** chosen from
   evidence: most likely `P0-CERT scoped remediation` (if the audit finds concrete certification failures in
   the TUI seam to repair) or, if the seam is already clean, a `Certification`-style proof of the SPINE-CERT/
   EPI-CERT **TUI-slice** that contributes evidence toward `P0-CERT`. Pick the posture the evidence supports and
   justify it; do not default to `not applicable` (this spec affects tests/fixtures/debug surfaces).

7. **Locked posture, source discipline, no scope creep.** Archived specs are historical evidence only. Add no
   new world mechanics, no Phase-4/institution/epistemic/ordinary-life scope, no LLM surfaces. Use
   `holder-known context` as the system-wide term and `actor-known` for the actor case. No backwards-compat
   shims or alias paths.

`assumption:` the deliverable's intended repository home is `docs/4-specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` (next live spec number after the realigned `0001`). If the maintainer prefers staging under `specs/` first, the filename is unchanged and only the directory moves — one-line correctable.

---

## 4. The task

Produce a **hardening / anti-contamination certification spec** (target type: *hardening*) for the Tracewake
TUI / view-model / debug boundary. The spec must: audit the current `tracewake-tui` crate and the core
view-model/debug/pipeline seams it consumes against the realigned foundation/architecture/execution doctrine,
with file:line evidence; identify every place the boundary is weak, drift-prone, or contamination-prone; and
specify the durable, testable requirements and anti-contamination gates that lock the *intent* of archived
specs 0002 and 0003 against future erosion — while raising the surface's **proof-surface legibility** (why-not,
provenance/causal-trace surfacing, "who knows what & why," inspectability) over the existing string renderer.
It must explicitly fence off visual/render uplift as deferred, deliver a reasoned ratatui/crossterm
recommendation, and declare one evidence-based execution admissibility posture. The result is the kind of spec
that a later `spec-to-tickets` pass could decompose without re-litigating intent.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow `use`/`mod` paths into any
core seam the TUI depends on, and read whatever fixtures/tests prove (or fail to prove) the boundary.

Research online as deeply as needed and **cite sources** for any external claim that shapes a decision. High-value
directions for *this* target:
- TUI / terminal-app architecture where a UI is a **pure projection of a model** and must own no domain rules
  (Elm Architecture / model-view-update; immediate-mode vs retained-mode terminal UIs; ratatui/crossterm
  testability and determinism characteristics; snapshot/golden-output testing for terminal UIs).
- **Event-sourcing read-model / projection** discipline (read models as derived, rebuildable, never
  authoritative) and how UIs avoid leaking write-side or privileged state into read-side views.
- **Epistemic / "fog-of-war" UI** and provenance display: surfacing *who knows what, from what source, with what
  confidence, and how it may be wrong* without leaking ground truth — prior art in detective/immersive-sim and
  belief-modeling games, and in provenance/explainability UX research.
- **Why-not / explanation** surfaces: structured failure explanation, two-layer (player-visible vs operator)
  diagnostics, and avoiding information leakage through error messages.
- Anti-contamination / capability-boundary patterns: how to make it *structurally impossible* (not merely
  tested) for a presentation layer to reach privileged truth — type-level capability separation, sealed
  read-only contexts, newtype/uninstantiable-from-truth patterns in Rust.

Use external prior art to sharpen the spec's *requirements and gates*, not to import scope the intentions forbid.

---

## 6. Doctrine & constraints (honor these)

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every requirement must satisfy it;
  a genuine divergence requires amending an invariant first, never designing against it silently. Cite the exact
  `INV-###` each gate enforces.
- **Authority order:** foundation → architecture → execution → reference → specs. If execution conflicts with
  architecture/foundation, execution is wrong; if implementation is more convenient than the accepted gates,
  implementation is wrong. A spec may operationalize higher-tier doctrine; it may **not** amend invariants,
  replace architecture, define/weaken gate semantics, or promote archived history into certification.
- **Anti-contamination is the point:** no simulation fact may be born from prose; preserve event-sourced
  causality, subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads,
  TUI-first playability, and validation/replay. Debug truth is quarantined and structurally unable to feed
  embodied cognition, affordances, or tests-masquerading-as-player-knowledge.
- **Diagnostics:** typed provenance and typed reason codes — never string-prefix, substring, or display-label
  behavior — as the basis for why-not and audit findings.
- **Determinism:** no wall-clock, OS/process randomness, network, filesystem-during-resolution, thread races,
  hashmap/hashset iteration order, or terminal-frame timing in any outcome-affecting or rendered-for-test path.
- **No new mechanics, no scope creep, no LLM surfaces, no backwards-compat shims or alias paths.**
- Workspace gates the resulting work must ultimately satisfy (state them in the spec's acceptance section, do
  not run them): `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo build --workspace --all-targets --locked`; `cargo test --workspace`.

---

## 7. Deliverable specification

Produce **one new downloadable markdown document**:

- **Filename:** `0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`
- **Intended repository path:** `docs/4-specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`
  (the maintainer may stage it under `specs/` first; the filename is unchanged).
- **Status:** new file. It does not replace any existing file. It must not edit, restate-as-authority, or
  rewrite any doc in `docs/0-foundation`, `docs/1-architecture`, `docs/2-execution`, `docs/3-reference`, the
  archived specs, or code. It references them.

The spec must contain, at minimum:

1. **Header & baseline statement** — repository, the analyzed commit `bf0e3a0392bb1217caf034281913487aa1644ad4`,
   the source-discipline note (manifest = path inventory; archived-spec baselines are historical), and exactly
   one declared execution admissibility posture (`P0-CERT …`) chosen from evidence per intention #6, with
   justification.
2. **Authority chain & gate mapping** — the controlling foundation/architecture/execution/reference docs, and
   the `SPINE-CERT`/`EPI-CERT`/`P0-CERT` mapping as cross-references only.
3. **Scope & non-goals** — TUI/view-model/debug boundary hardening + proof-surface legibility; explicit
   non-goals: visual/render uplift, new mechanics, epistemics/ordinary-life re-spec, LLM, graphical client.
4. **Audit findings** — the current state of the boundary with **file:line evidence**, organized by the
   anti-contamination dimensions in intention #3, each finding tagged with the responsible layer (view-model
   construction / projection / debug-report construction / pipeline / event application / TUI rendering /
   replay / tests-fixtures / docs) and the `INV-###` it bears on. Distinguish *already-satisfied* from
   *needs-hardening* with evidence for both.
5. **Hardening requirements & anti-contamination gates** — durable, testable requirements that lock the intent,
   each stating the invariant(s) it enforces, the failure it prevents, and how it is *structurally* enforced
   where possible (type-level capability separation, sealed read-only contexts) rather than only test-enforced.
6. **Proof-surface legibility requirements** — concrete, render-tech-agnostic improvements to why-not,
   provenance/causal-trace surfacing, "who knows what & why," and inspectability over the string renderer,
   each tied to a doctrine source and an acceptance test.
7. **Deferred-uplift boundary & render-tech recommendation** — the visual/graphical-client deferral fence and
   the reasoned ratatui/crossterm evaluate-and-recommend conclusion (per intentions #4–#5), with the conditions
   under which a future spec should revisit the dependency.
8. **Required fixtures & tests** — positive **and adversarial/negative** gates (debug-truth-leakage attempts,
   forged/privileged-view attempts, possession knowledge-transfer attempts, direct-dispatch attempts,
   determinism/replay/transcript stability, why-not non-leakage), in the spirit of execution doc 10's
   typed-diagnostic and adversarial-gate discipline.
9. **Acceptance checklist** — including the four workspace gates and a per-requirement acceptance condition.

Keep the spec self-consistent with `docs/4-specs/README.md`'s rules for future specs (posture declaration;
gate codes as cross-references; correct holder-known/actor-known terminology; archived specs as history; no
files for symmetry).

> **Locked / no-questions:** Produce the deliverable directly as a downloadable markdown document. Do not
> interview, do not ask clarifying questions — the requirements above are final. If a genuine contradiction
> makes a requirement impossible, state it inside the spec and proceed with the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] Every file in §2 was fetched from commit `bf0e3a0392bb1217caf034281913487aa1644ad4`; the manifest was
      used only to enumerate paths.
- [ ] The deliverable is exactly one new file with the §7 filename; it edits/replaces nothing and restates no
      upstream tier as local authority.
- [ ] Exactly one `P0-CERT …` admissibility posture is declared and justified by evidence (not defaulted to
      `not applicable`).
- [ ] Gate codes (`SPINE-CERT`, `EPI-CERT`, `P0-CERT`, etc.) appear only as cross-references; no gate semantics
      are defined or weakened.
- [ ] "Prover TUI" is treated as **proof-surface legibility**, not visual richness; no panes/layout/live-screen
      design is authored; visual uplift is explicitly deferred.
- [ ] The ratatui/crossterm question is *evaluated and recommended*, not pre-authorized, with reasoning.
- [ ] Audit findings carry file:line evidence and a responsible-layer + `INV-###` tag; already-satisfied vs
      needs-hardening are distinguished.
- [ ] Anti-contamination gates cover debug-truth quarantine, no-rule-inference-in-TUI, no-direct-dispatch,
      possession-input-binding-only, actor-known view models, typed (not string-prefix) diagnostics, and
      determinism — each tied to an invariant and an adversarial test.
- [ ] No new mechanics, no epistemics/ordinary-life re-spec, no LLM, no backwards-compat shims; terminology
      matches the glossary (`holder-known` / `actor-known`).
- [ ] Every external claim that shaped a decision is cited.
- [ ] Foundation/architecture invariants are satisfied; nothing in the spec weakens an upstream tier.
