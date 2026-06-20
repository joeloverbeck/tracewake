# Research brief — P0-CERT post-0008 baseline certification spec (the next Tracewake spec)

> **You are ChatGPT-Pro, Session 2.** This prompt is self-contained. Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a requirement
> impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-16_9f16222.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first; agents act from
partial belief and every event leaves a replayable trace). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; **earlier tiers
govern later ones.** Fetch every file from commit **`9f16222`** (verified repo `HEAD`) — the
manifest reflects exactly that tree.

**This is a determination already made — not a cold start.** The next spec has been decided by the
repository's own doctrine, and the deciding session's job was only to settle scope. The execution
tier (`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`) and the spec
ledger (`docs/4-specs/SPEC_LEDGER.md`, "Next known execution move") both name **`P0-CERT`** — the
post-0008 baseline certification audit — as the single mandated next implementation target. Specs
`0026`–`0035` (all archived) were doctrine-alignment amendments that certified no code; the
doc-alignment cascade is complete. The `0035` "route-forward backlog" of future expansion specs
(temporal-firewall fixtures, inventory/economy, affect/learning, domain-pack bias, budget/fairness,
authoring/compiler, TUI/play-loop) is **doctrinally blocked** behind the certification gates and
therefore cannot be the next spec. Your task is **not** to re-decide what is next; it is to author
the P0-CERT certification spec itself.

`docs/4-specs/README.md` states plainly: "`P0-CERT` is named by the execution tier as the next major
implementation audit. This directory is ready for that future work, but it does not contain that
certification spec." You are writing exactly that missing spec.

## 2. Read in full (authority order)

Read these before producing. Order is by authority tier. The execution tier is the heart of this
target — `01` and `02` define the P0-CERT contract verbatim.

**Foundation — primary (the constitution and the gates P0-CERT proves):**

```
docs/README.md — authority order and the layering rule (foundation > architecture > execution > reference > specs).
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md — INV-001…INV-NNN; every gate the audit proves must satisfy these.
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md — "truth may validate, truth may not plan"; the spine of P0-CERT gates 2 and 6.
docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md — first-village scope, no-human / replay / TUI gates, canonical regression seeds.
docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md — event authority, replay, randomness; P0-CERT gate 1 and REPLAY.
docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md — action parity, ordinary-life survival; the no-human ordinary-day surface.
docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md — possession parity, actor-filtered view models, debug quarantine; P0-CERT gates 4 and 7.
```

**Architecture — primary (subsystem contracts the audit measures code against):**

```
docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md — universal conformance questions and the replacement/retirement rule.
docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md — event log, replay, projections, save manifests, random-stream discipline, schema versioning.
docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md — holder-known contexts, provenance packets, context sealing, contamination gates.
docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md — proposal pipeline, scheduler limits, validation truth, no-direct-dispatch.
docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md — actor decision transaction, needs, intentions, HTN selection, stuck diagnostics.
docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md — possession, embodied TUI, debug-only truth, client boundaries.
docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md — acceptance artifacts, anti-contamination tests, diagnostics, review checklists.
```

**Execution — primary (the P0-CERT contract itself; this is where the spec is defined):**

```
docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md — canonical gate names (P0-CERT, TFW, PIPE, NO-DIRECT, NO-HUMAN, POS-PARITY, REPLAY, FIXTURE, DIAG), observation obligations, label reconciliation, universal execution posture.
docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md — THE P0-CERT definition: the 10-point artifact requirements, the code audit boundary, and admissibility outcomes. The single most load-bearing file.
docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md — the first-proof acceptance contract (EVENT, TRUTH-FIREWALL, ACTOR-KNOWN, POSSESSION-PARITY, NO-HUMAN-ORDINARY-LIFE, MISSING-PROPERTY, VIEW-DEBUG-SPLIT, REPLAY, FIXTURE-NEGATIVE) and "definition of first-proof done".
docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md — gate order, the "Gate evidence requirements" list every gate artifact must include, and the four valid future-spec postures.
docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md — truth-firewall execution checks and mandatory anti-contamination gates (TFW evidence).
docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md — scheduler/proposal/validation/direct-dispatch audit criteria (PIPE, NO-DIRECT).
docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md — needs/routines/intentions/no-human-day certification (NO-HUMAN).
docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md — epistemic proof, possession parity, view-model filtering, debug quarantine (POS-PARITY, VIEW-DEBUG-SPLIT).
docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md — authoring contracts, schema/provenance validation, no outcome-chain data (content-rejection gate).
docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md — golden fixture families, adversarial scenarios, deterministic replay acceptance (FIXTURE, REPLAY).
docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md — testing, diagnostics by layer, review evidence, and the EMERGE-OBS observation obligation (DIAG).
```

**Reference + specs — primary (rules, numbering, source discipline, structural models):**

```
docs/4-specs/README.md — future-spec rules: declare authority posture, declare one admissibility posture, gate codes as cross-references only, holder-known/actor-known terminology, source discipline, no symmetry files.
docs/4-specs/SPEC_LEDGER.md — active ledger, source-discipline rules, archived-spec posture, and the "Next known execution move" (P0-CERT) you are fulfilling.
docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md — the live first-proof ontology/fixture contract: the system the audit certifies.
docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md — the scoped review-artifact template (evidence-status / fingerprint-scope / behavior-witness / replay-provenance / sampling / pending-historical / certification-use / staged-abstraction declaration fields). The acceptance artifact your spec's evidence package must conform to.
archive/specs/0025_PHASE_3A_META_WITNESS_EXECUTION_PROOF_PERCEPTION_KILL_SET_PROVENANCE_ENVELOPE_READ_PATH_FAIL_CLOSED_AND_MANIFEST_FINGERPRINT_HONESTY_HARDENING_SPEC.md — structural model: the most recent implementation/certification spec; mirror its section shape and rigor.
archive/specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md — structural model: an ORD-LIFE-CERT-scoped audit spec with an acceptance artifact; another shape reference.
```

**Reference — boundary-awareness (read to bound scope; NOT conformance targets):**

```
docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md — review guardrails and gate-code lookup posture.
docs/3-reference/01_DESIGN_RISK_REGISTER.md — the post-0008 relapse risk (treating archived 0005–0008 as certification) the audit must actively resist.
docs/3-reference/02_GLOSSARY.md — prescriptive terminology control (holder-known, observation, belief, debug truth, source discipline).
```

**Boundary-awareness — OUT of the post-0008 baseline (read only to know what the audit must NOT touch):**

```
docs/0-foundation/07,09,10,11,13 and docs/1-architecture/06,07,08,09,11,12,14 — institutions, LLM/speech, LOD/regional scale, no-scripting authoring depth: future-phase or deferred surfaces, not part of P0-CERT scope.
docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md — Phase-4 entry lock; P0-CERT is the gate BEFORE this, do not certify into it.
docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md — deferral contract for notices/travel/regional/LOD/second-proof; explicitly out of first-proof scope.
```

**Code — primary exploration targets (Session 2 reads the crates directly; these are the audit seams):**

The workspace has three crates with strict one-way dependency direction: `tracewake-core`
(authoritative kernel, zero deps) → `tracewake-content` (fixtures, loading, schema validation,
depends on core) → `tracewake-tui` (possession, view models, depends on core + content). Audit
seams, by P0-CERT area (read these and explore outward as needed):

```
Scheduler / no-human loop:        crates/tracewake-core/src/scheduler.rs, scheduler/no_human.rs, need_accounting.rs, time.rs
Actor-known context + provenance: crates/tracewake-core/src/agent/actor_known.rs, agent/no_human_surface.rs, checksum.rs, epistemics/knowledge_context.rs, epistemics/knowledge_basis.rs
Actor decision transaction:       crates/tracewake-core/src/agent/transaction.rs, agent/decision.rs, agent/candidate.rs, agent/trace.rs, agent/perception.rs
Planner / HTN / routines / needs:  crates/tracewake-core/src/agent/planner.rs, agent/htn.rs, agent/routine.rs, agent/methods.rs, agent/intention.rs, agent/need.rs
Action pipeline / no-direct:      crates/tracewake-core/src/actions/proposal.rs, actions/pipeline.rs, actions/report.rs, actions/registry.rs, actions/defs/mod.rs, controller.rs
Event log / apply / types:        crates/tracewake-core/src/events/log.rs, events/envelope.rs, events/apply.rs, events/mutation.rs, crates/tracewake-content/src/load.rs
State / projections:              crates/tracewake-core/src/state.rs, projections.rs, epistemics/projection.rs, view_models.rs
Replay / determinism / save:      crates/tracewake-core/src/replay/rebuild.rs, replay/report.rs, checksum.rs, crates/tracewake-content/src/manifest.rs
TUI possession / debug quarantine: crates/tracewake-tui/src/app.rs, tui/src/debug_panels.rs, tui/src/render.rs, tui/src/input.rs, crates/tracewake-core/src/debug_capability.rs, debug_reports.rs
Content schema / validation:      crates/tracewake-content/src/schema.rs, content/src/validate.rs, content/src/serialization.rs, crates/tracewake-core/src/ids.rs
Fixtures (golden + negative):     crates/tracewake-content/src/fixtures/, content/tests/golden_fixtures_run.rs, core/tests/golden_scenarios.rs, core/tests/no_human_capstone.rs, tests/negative-fixtures/, core/tests/negative_fixture_runner.rs
Acceptance / lock-layer tests:    crates/tracewake-core/tests/acceptance_gates.rs, hidden_truth_gates.rs, event_schema_replay_gates.rs, content/tests/forbidden_content.rs, tui/tests/adversarial_gates.rs
Typed diagnostics by layer:       crates/tracewake-core/src/agent/trace.rs (BlockerCode/StuckDiagnosticRecord), actions/report.rs, events/apply.rs, replay/rebuild.rs, content/src/validate.rs, controller.rs
CI / mutation / lint lock-layer:  .github/workflows/ci.yml, .cargo/mutants.toml, clippy.toml, rust-toolchain.toml
```

## 3. Settled intentions (final — these are decisions, not options)

1. **The deliverable is the P0-CERT certification spec.** A single new numbered spec that
   *specifies the post-0008 baseline certification audit*. Posture = **Certification** (proves gates
   without expanding gameplay scope, per `03`'s posture list). It does not add features.

2. **The decision of "what is next" is already settled — do NOT re-open it.** Doctrine names
   P0-CERT. Do not survey for alternative next-specs, do not propose expansion-backlog work, do not
   argue for a different gate. Your research budget goes entirely to **how to audit and certify** —
   audit method, seam coverage, fixtures, evidence package, acceptance-artifact shape — not **what**
   to build.

3. **The spec specifies the audit; it does not render the pass/fail verdict.** You cannot run
   `cargo fmt/clippy/build/test`, mutation testing, or replay against live code from this session,
   so the spec must define *what the implementing session will prove and how*: the audit scope, the
   seam inventory, each gate's required evidence, the fixture families, and the acceptance artifact —
   not a certified result. **`assumption`:** you MAY include a clearly-labeled **preliminary static
   seam survey** (what a reading of the code at `9f16222` suggests about likely gate
   satisfaction/risk) as *informative, non-certifying* evidence to sharpen the audit plan — but it
   must be explicitly marked "preliminary, not certification," and the spec must state that
   authoritative pass/fail comes from the implementing session executing the gates. If you judge the
   survey would dilute the spec, omit it; the audit-plan spec is the required core either way.

4. **Map all ten P0-CERT proof requirements** from
   `docs/2-execution/01_…CODE_AUDIT_BOUNDARY.md` (§"The `P0-CERT` artifact must prove", points
   1–10) to **concrete code seams** (named files/modules from §2) and to **required evidence**
   (positive fixtures, adversarial/negative fixtures, event/replay/projection artifacts, typed
   diagnostics by layer). Each of the ten is a first-class deliverable line in the spec.

5. **Compose the canonical gates, do not mint new ones.** The audit is expressed through the
   existing canonical gates — `P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`,
   `REPLAY`, `FIXTURE`, `DIAG` (defined in `00_EXECUTION_INDEX_AND_AUTHORITY.md`) — and the
   first-proof acceptance-contract gate labels in `02`. Name gate codes only as cross-references;
   the spec defines no new gate code, observation obligation, or status vocabulary. Treat
   `EMERGE-OBS` correctly: an observer-only observation obligation, never a pass/fail threshold or
   phase gate.

6. **Honor the bootstrapping nuance of the admissibility posture.** Every future implementation spec
   must declare one of `P0-CERT passed` / `P0-CERT scoped remediation` / `P0-CERT not applicable`
   (`docs/2-execution/01`, "Acceptance gates for future specs"). The P0-CERT spec is the audit that
   *produces* the certification, so it cannot cite "P0-CERT passed" as its own consumed input.
   **State this explicitly** in the spec (it is the certification-producing spec, not a consumer of a
   prior certification) rather than forcing an ill-fitting label; if the spec must carry an
   admissibility line for form, explain why none of the three consumer labels applies to the
   producing spec and how the spec instead *defines* the artifact those labels will later cite.

7. **Include every "Gate evidence requirements" element** from
   `docs/2-execution/03_…CERTIFICATION_SEQUENCE.md`: exact files/seams audited; foundation &
   architecture dependencies; artifact dependencies (including the observer-only `EMERGE-OBS`
   artifact where the corpus exercises first-proof living-world acceptance); positive and negative
   fixtures; event/replay/projection evidence; actor-known provenance evidence; debug-quarantine
   evidence; failure diagnostics by responsible layer; a statement that archived specs/tickets are
   used only as historical evidence; and a list of tolerated deferrals tied to named gates.

8. **The acceptance/evidence package conforms to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.**
   The spec's verification/evidence section must specify an acceptance artifact using that template's
   fields (evidence-status, fingerprint-scope, behavior-witness, replay/provenance, sampling/
   exhaustiveness, pending/historical, certification-use, and the staged-abstraction declaration).
   Where the audit stages any abstraction, declare it per the staged-abstraction discipline (what it
   proves now, what it abstracts, what it must not fake, what future tier/feature it must not
   certify by implication).

9. **Follow the sibling-spec structural convention, not the foundation-doc house style.** Mirror the
   section shape and rigor of `archive/specs/0025_*` and `archive/specs/0014_*` (status, admissibility
   posture, authority/dependency declarations, problem statement, audit approach, per-gate
   deliverables, invariants alignment, verification/acceptance artifact, out-of-scope, risks & open
   questions). Do **not** adopt the narrative `docs/NN_*` document structure. If a section the
   sibling specs use does not fit a certification audit, adapt it and say why.

10. **Source discipline (`SPEC_LEDGER.md`).** A commit hash named in a spec is audit/spec provenance
    only. A manifest is path inventory only. Branch names, default-branch lookups, connector
    namespace labels, repository metadata, and code-search snippets are not proof of target-commit
    content. Pin the audit to commit `9f16222`. Cite archived specs `0005`–`0035` and tickets only
    as historical evidence, never as live certification.

11. **Scope boundary is the post-0008 baseline only:** the spine (event log / replay / projection /
    randomness / save / action pipeline / TUI-debug split / no-direct-dispatch), the epistemic
    substrate (actor-known/holder-known contexts, beliefs, observations, provenance, possession
    parity, view models, debug quarantine), and ordinary life (needs, routines, intentions,
    no-human day, planner traces, stuck diagnostics) — i.e. the surfaces historically landed under
    `0005`–`0025`. Institutions, wrong-suspicion, notices, travel, regional scale, LOD, LLM dialogue,
    and story-sifting are **out of scope** (locked behind `PHASE-4-ENTRY` / `SECOND-PROOF-ENTRY`);
    the spec must name them only as deferrals, never audit them.

> `assumption`: the spec is authored at **`specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md`**
> — the implementation-spec staging series (`specs/NNNN_*`), which is archived to `archive/specs/` on
> acceptance and not promoted into live `docs/4-specs/`. Number `0036` is the next free number across
> `specs/`, `docs/4-specs/`, and `archive/specs/` (highest existing is `0035`; the series is
> contiguous with no recent renumbering). If you judge the placement should be live
> `docs/4-specs/0036_*` instead, note it and proceed with the staging path as the default; the
> content requirements are unchanged. The ledger row for this spec is added at acceptance/closeout
> per the repo's hardening-series ledger-timing convention, not authored into the spec itself.

## 4. The task

Author the **P0-CERT post-0008 baseline certification spec** — the next Tracewake spec. This is a
**new-spec** target with a **hardening / anti-contamination** character: it specifies the code audit
that proves the historical `0005`–`0025` implementation satisfies the post-overhaul foundation,
architecture, and execution doctrine before any Phase 4, second-proof, LLM, travel, regional, or
new-gameplay work may proceed. The spec must turn the abstract ten-point `P0-CERT` contract into a
concrete, executable audit plan: which seams are inspected, which gates each proves, which positive
and adversarial fixtures provide evidence, which typed diagnostics name the responsible layer on
failure, and what the conforming acceptance artifact looks like. It is a requirements/plan document
that the repo's reassess → ticket → implement process will execute — not a rendered certification
result.

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — read the crate sources at
the named seams and follow dependencies outward to ground every audit claim in real module and
function names. **Research online as deeply as needed** — event-sourcing replay/determinism
verification, certification and acceptance-test harness design, mutation testing (e.g. cargo-mutants)
as a lock-layer for anti-contamination guards, golden-fixture acceptance, property-based and
adversarial testing for hidden-truth / information-leakage detection, and audit-evidence packaging
for safety-style certification — wherever it sharpens the audit method, the fixture families, or the
evidence package. **Cite every external source that shapes a decision.** The deep research is your
job; this brief only commissions it.

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every gate the audit
  proves must satisfy it; a genuine divergence would require amending an invariant first — never
  design the audit against an invariant silently. The spec is subordinate doctrine: it
  operationalizes the higher tiers and may not amend, replace, weaken, or redefine them.
- **Authority order:** if execution conflicts with architecture or foundation, execution is wrong;
  if implementation is more convenient than the accepted gates, implementation is wrong; if a test
  proves only plausible behavior while bypassing holder-known provenance, the test is wrong. The
  audit measures code against doctrine, never the reverse.
- **The truth firewall is the spine:** *truth may validate actions, but truth may not plan them.* The
  audit must prove cognition/goal-selection/routine-selection/view-rendering are fed only by sealed
  actor-known context with provenance — not validation truth, not debug truth, not fixture-only facts.
- **Anti-contamination:** no simulation fact may be born from prose. Preserve event-sourced
  causality, subjective epistemics, ordinary agents, possession parity, fallible institutions,
  questless leads, validation/replay, the holder-known truth firewall. Adversarial/negative fixtures
  that prove forbidden shortcuts FAIL are first-class audit evidence, not an afterthought.
- **No backwards-compatibility shims or alias paths** in any recommendation; no new files merely for
  symmetry.
- **Terminology:** use `holder-known context` as the system-wide term and `actor-known` for the
  actor case.

## 7. Deliverable specification

Produce **one** downloadable markdown document — a **numbered implementation spec** (this IS a
`specs/`-tier artifact; the numbering/placement rules in §3 apply):

- **`specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md`** — **new** file. (If you
  determine a different, clearly-better title slug, you may adjust the title words but keep the
  `0036_` prefix and the `specs/` staging path unless §3's placement assumption is wrong.)

Required content (adapt section names to the sibling-spec convention in `archive/specs/0025_*` /
`0014_*`; the substance below is mandatory):

1. **Status + admissibility posture** — declare authority beneath foundation/architecture/execution/
   reference; address the bootstrapping nuance per settled intention 6 (this spec *produces* P0-CERT;
   it does not consume a prior P0-CERT).
2. **Authority & dependencies** — the controlling foundation/architecture/execution docs (the §2
   primary set), pinned to commit `9f16222`; archived specs cited as history only.
3. **Problem statement** — the post-0008 baseline gap: historical `0005`–`0025` code is landed but
   uncertified under the replacement doctrine; P0-CERT is the gate that must pass before any
   expansion (cite the phase ladder and `01`).
4. **Audit approach** — the method: seam-by-seam inspection, gate composition, fixture-evidence
   strategy, replay/determinism verification, mutation/lock-layer posture, and how staged
   abstractions (if any) are declared and bounded.
5. **Per-gate deliverables** — one block per P0-CERT proof requirement (the ten points of `01`),
   each mapping to: the concrete seams audited, the canonical gate(s) it composes, the positive
   fixtures, the adversarial/negative fixtures that must fail, the event/replay/projection evidence,
   and the typed failure diagnostic that names the responsible layer. Cover all ten; cover the `02`
   first-proof acceptance-contract gates; include the "Gate evidence requirements" elements from
   `03` (settled intention 7).
6. **Acceptance / evidence artifact** — the conforming acceptance artifact specified against the
   `0003` template's fields (settled intention 8), including the observer-only `EMERGE-OBS` artifact
   where the corpus exercises first-proof living-world acceptance, and any staged-abstraction
   declarations.
7. **Invariants alignment** — which constitutional invariants each audited gate preserves (cite
   `INV-NNN` as cross-references; coin none).
8. **Out of scope / deferrals** — the locked Phase-4 and second-proof surfaces (settled intention
   11), each tied to its named entry gate.
9. **Risks & open questions** — audit risks (e.g. friendly-only fixtures, label-implied passes,
   debug leakage into evidence), and owner decisions you cannot settle from the docs (carry, do not
   invent).

Do **not** write final ticket breakdowns (the repo's spec-to-tickets process owns that), invent new
gate/obligation codes or status enums, or assert a certified pass/fail result.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

## 8. Self-check (run before returning)

- [ ] The deliverable is exactly one new spec at `specs/0036_…` (or a justified title variant with the
      same prefix/path), in the sibling-spec structural convention — not the narrative foundation-doc
      style.
- [ ] All ten `P0-CERT` proof requirements from `docs/2-execution/01` are mapped to concrete named
      code seams and to required positive + adversarial evidence.
- [ ] The canonical gates (`TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`,
      `DIAG`) and the `02` acceptance-contract gates are composed, not redefined; no new gate code,
      obligation code, or status vocabulary is minted; `EMERGE-OBS` is treated as observer-only and
      never a pass/fail threshold.
- [ ] Every "Gate evidence requirements" element from `docs/2-execution/03` is present.
- [ ] The acceptance artifact conforms to the `docs/4-specs/0003` template fields, with
      staged-abstraction declarations where any abstraction is staged.
- [ ] The admissibility-posture bootstrapping nuance is addressed explicitly (the spec produces
      P0-CERT; it does not cite a prior P0-CERT as passed).
- [ ] Scope is the post-0008 baseline only; institutions / wrong-suspicion / notices / travel /
      regional / LOD / LLM / story-sifting appear only as named deferrals, never as audit targets.
- [ ] No new doctrine weakens an upstream tier; the spec stays subordinate to
      foundation/architecture/execution/reference.
- [ ] Source discipline holds: pinned to `9f16222`; archived specs cited as history only; manifest
      treated as path inventory only.
- [ ] Any preliminary static seam survey (if included) is explicitly labeled non-certifying.
- [ ] Every external claim that shaped a decision is cited.
- [ ] Commit `9f16222` contains every file named in §2.
