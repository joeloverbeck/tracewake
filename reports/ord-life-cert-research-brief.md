# ORD-LIFE-CERT certification spec — deep-research brief (paste into ChatGPT-Pro)

> **You are Session 2.** Produce the deliverable directly as a downloadable markdown
> document. Do **not** interview, do **not** ask clarifying questions — the requirements
> below are final. If a genuine contradiction makes a requirement impossible, state it in
> the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-20_98dc042.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust (event-sourced
kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers
govern later ones. **Fetch every file from commit `98dc0421211e6c9881d9c6679b9df74525e392bb`
(short `98dc042`)** — the manifest reflects exactly that tree. Do not adopt a different "commit of
record" from any file you read: commit hashes that appear *inside* archived specs/acceptance
artifacts (e.g. `726b2a1`, `92ba47f`, `2a37b04`) are that artifact's own audit provenance, not this
baseline. A manifest is path inventory only.

**This brief continues an active certification campaign.** The repo runs a phase-certification
ladder (`docs/2-execution/03`): `P0-DOC → P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT →
FIRST-PROOF-CERT → PHASE-4-ENTRY → SECOND-PROOF-ENTRY`. The immediately preceding work delivered the
**EPI-CERT pair**, which is your **lineage predecessor** (this brief is a *delta* on it, retargeted
from the epistemic substrate to the ordinary-life substrate — do not re-commission it):

- **0040** (`archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`)
  — the `EPI-CERT` audit spec; verdict came back `EPI-CERT scoped remediation` because Wave-B
  mutation expansion left a survivor floor.
- **0041** (`archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`)
  — the scoped mutation remediation + replacement certification; verdict **`EPI-CERT passed`** for
  the scoped 0041 mutation-remediation line at exact implementation/evidence commit
  `726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3`. Its artifact
  (`archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`)
  supersedes the 0040 artifact for `EPI-CERT passed` citation.

`P0-CERT passed` (0037 artifact), `SPINE-CERT passed` (0039 artifact), **and** `EPI-CERT passed`
(0041 artifact) are therefore the current admissibility state. Per the gate ladder and the ledger's
"Next known execution move" (`docs/4-specs/SPEC_LEDGER.md`, which states verbatim: *"The next known
execution move is a separately numbered `ORD-LIFE-CERT` spec and evidence package"*), the **single
next admissible spec is the `ORD-LIFE-CERT` certification** (gate 4).

**The structural model you must mirror** (the cert-spec anatomy this campaign reuses, read in full —
these are *shape* precedents in §2, not delta seeds): the P0-CERT pair (0036 audit / 0037
remediation), the SPINE-CERT pair (0038 audit / 0039 remediation), and the EPI-CERT pair (0040 audit
/ 0041 remediation). Your spec is the **audit spec** of the next pair — the ordinary-life analogue of
0036 / 0038 / 0040. The eventual ORD-LIFE-CERT mutation-remediation replacement spec is **a later,
separate spec** and is **out of scope here** (it is authored only if/when the implementing session's
mutation run leaves survivors — do not pre-author it).

---

## 2. Read in full (authority order)

Read these before producing. The user's standing instruction is to read **all** of
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and
`docs/4-specs/*`. Within each tier, the entries marked **[primary]** are load-bearing for the
ORD-LIFE-CERT spec; the remainder are *boundary-awareness* (read to know what is **out** of
ORD-LIFE-CERT scope and must not be audited or "corrected" here — they belong to SPINE-CERT and
EPI-CERT (already passed) / FIRST-PROOF-CERT / Phase-4 / later gates).

**Foundation (`docs/0-foundation/`)**
- `docs/README.md` — **[primary]** repository authority layering and the "execution is wrong if it
  conflicts with architecture/foundation" rule.
- `00_FOUNDATION_INDEX.md` — index/boundary map.
- `02_CONSTITUTIONAL_INVARIANTS.md` — **[primary]** `INV-001…INV-112`; every certification property
  must satisfy these. The ordinary-life gates are downstream of the ordinary-agents /
  no-contamination / event-sourced-causality / possession-parity invariants here.
- `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — **[primary]** THE foundation contract for
  needs, intentions, routines, and planning that ORD-LIFE-CERT certifies.
- `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — **[primary]** THE foundation contract for
  actions, affordances, ordinary life, and survival behavior (eat/sleep/work/wait/fail).
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **[primary]** first-playable acceptance-gate
  doctrine (the no-human ordinary day is the first proof the world runs without a human).
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **[primary]** the actor-known
  cognition transaction and truth firewall; the basis for "candidate goals/methods use actor-known
  inputs only; no decision from unobserved ground truth" properties this gate certifies.
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **[primary]** needs/intentions/routines/stuck
  state must be event-sourced and replay-reconstructable (gate condition 1).
- `01, 04, 07, 08, 09, 10, 11, 13` — boundary-awareness (charter; claims/beliefs/memory — EPI;
  institutions; TUI/possession view models — EPI; no-scripting/seeds; LOD/scale; LLM/speech; research
  notes). Read to bound scope; not ORD-LIFE-CERT audit targets. (`04`/`08` epistemics and possession
  are EPI-CERT territory — already certified; consume their guarantees, do not re-audit them.)

**Architecture (`docs/1-architecture/`)**
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **[primary]** canonical gate/review-artifact
  composition; the ORD-LIFE-CERT *label* composes gates defined here, it mints no new gate code.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — **[primary]** the one-way crate
  dependency rule (core ← content ← tui) the ordinary-life seams must not invert.
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **[primary]** the projection-rebuild and
  determinism contract: ordinary-life metrics/diagnostics are replay-derived projections, never truth
  writers (gate conditions 1 and 8).
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **[primary]** holder-known/actor-known
  sealed contexts, the truth firewall, and provenance sufficiency/freshness — the substrate the
  ordinary-life decision transaction consumes (candidate generation/method selection cite actor-known
  provenance).
- `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — **[primary]** THE
  architecture contract for the actor-known decision transaction: needs → candidates → method/HTN
  selection → local planning → sealed proposal, with dangling-provenance discipline. The heart of
  ORD-LIFE-CERT.
- `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — **[primary]** ordinary-life spatial
  model, affordances, places, and property the routines move through (movement ancestry, no-teleport).
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **[primary]** what a gate
  artifact must contain; typed diagnostics and review-artifact obligations.
- `04, 06, 07, 08, 10, 11, 12, 14` — boundary-awareness (action pipeline/scheduling — read `04` only
  for the no-direct-dispatch boundary the gate's condition 4 asserts; beliefs/observation — EPI;
  speech/LLM; institutions; possession/TUI view models — EPI; incidents/leads; LOD/prehistory;
  forbidden misreads). Read to bound scope. (`14`'s forbidden-misreads list is useful orientation for
  ordinary-life terminology discipline, but is not an audit target.)

**Execution (`docs/2-execution/`)**
- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — **[primary]** execution-tier authority and the canonical
  gate index (ORD-LIFE-CERT is a phase-certification label that consumes canonical gate evidence).
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — **[primary]** the
  post-0008 baseline, the code-audit boundary, and the three admissibility postures
  (`passed` / `scoped remediation` / `not applicable`); explicitly states Phase-3A specs are not
  presumed-passed certification.
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — **[primary]** first-proof scope
  and acceptance contract that the ordinary-life certification feeds into (FIRST-PROOF-CERT is gate 5,
  downstream).
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **[primary]** defines `ORD-LIFE-CERT`
  (gate 4): "Certify needs, routines, intentions, no-human ordinary life, planner traces, and stuck
  diagnostics under the actor-known transaction." Also: gate-evidence requirements, gate-failure
  handling (named layers), valid future-spec postures, the EMERGE-OBS observer-only rule, the
  Temporal-Cascade placement (routine temporal proof from `06` feeds FIRST-PROOF-CERT), and the
  "Phase 3A landed = evidence for ORD-LIFE-CERT, not certification" historical mapping.
- `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **[primary]** THE ORD-LIFE-CERT execution
  proof doc. It owns execution acceptance for needs, routines, intentions, no-human advancement,
  routine failure, and ordinary-life diagnostics. It carries: the single-charge-per-actor/per-need/
  per-tick rule; the single-owner derived-accounting seam (action-pipeline/ordinary-life boundary);
  intention lifecycle states; routine-template requirements (a routine without failure modes is
  invalid); the **"Routine Temporal Premises and Adaptation Proof"** section; the no-human proof
  evidence list; the canonical `no_human_day_001` recovery resolution
  (`fail_only_empty_food_source`); behavioral-progress + cross-tick stuck-detection definitions; the
  **required ordinary-life adversarial fixture families table**; and the 10 numbered
  `ORD-LIFE-CERT`-passes conditions. **Your audit points must cover all 10 conditions and every
  fixture family in this doc.**
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **[primary]** THE anti-
  contamination/truth-firewall execution gates the ordinary-life boundary must hold (no fact born from
  prose; no decision from unobserved ground truth; validation truth may not propose; no teleport to
  true food).
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — **[primary]** golden-fixture and replay
  acceptance obligations; positive + adversarial fixture families (integrated no-human day, food
  unavailable, no-teleport, hidden-truth planning, etc.).
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **[primary]** evidence-honesty
  rule, diagnostics-by-layer, routine-failure-diagnostic classification (missing/stale knowledge,
  budget exhaustion, blocked affordance, validation failure), review-artifact obligations, EMERGE-OBS
  observer-only discipline, the single diagnostic home for budget/fairness evidence.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — **[primary]** the
  scheduler/action-pipeline/no-direct-dispatch boundary (gate condition 4: scheduler cannot dispatch
  ordinary actions directly from needs or routines). Note: the SPINE-CERT *certification of the
  pipeline itself* is already passed and is **not** re-audited; read this for the boundary the
  ordinary-life seam must respect, and for the explicit doctrine that scheduler awakenings / elapsed-
  time accounting do **not** count as routine-premise evidence.
- `08, 11, 12, 13` — boundary-awareness (data authoring/schema — read only where the ordinary-life
  seam consumes authored seed knowledge / fixtures; institutions/Phase-4 entry; deferred second-proof;
  research notes). Read to bound scope; not ORD-LIFE-CERT targets.
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — boundary-awareness (EPI-CERT execution
  proof — already certified). Consume the possession-parity and debug-quarantine guarantees where the
  ordinary-life fixture families require them (possession-does-not-reset-intention; planner traces
  visible in debug without feeding actor input), but do not re-audit the EPI seams.

**Reference (`docs/3-reference/`)**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **[primary]** review checklist questions.
- `01_DESIGN_RISK_REGISTER.md` — **[primary]** standing risks (ordinary-life-relevant: contamination /
  hidden-truth leakage into planning, silent starvation/teleport, no-progress loops, single-charge
  drift, Goodharting the no-human metrics, stale-premise routine selection).
- `02_GLOSSARY.md` — **[primary]** canonical terminology (`actor-known`, `holder-known context`,
  `no-human proof`, `routine`, `intention`, `candidate goal`, `behavioral progress`, `stuck
  diagnostic`, `EMERGE-OBS`, etc.) the spec must use exactly.

**Specs (`docs/4-specs/`)**
- `SPEC_LEDGER.md` — **[primary]** the live ledger: authority posture, source discipline, archived-
  spec status (Phase-3A specs `0005`–`0025` contribute scoped evidence toward `ORD-LIFE-CERT` but are
  **not** certification), the "Next known execution move" (records `EPI-CERT passed` and names a
  separately-numbered `ORD-LIFE-CERT` spec as the next move; does **not** yet declare ORD-LIFE-CERT),
  and the numbering/admissibility rules.
- `README.md` — **[primary]** rules for future specs (declare authority posture; declare exactly one
  admissibility posture; gate codes as cross-references only; terminology; no files for symmetry).
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — **[primary]** the review-artifact template (incl.
  evidence-status / fingerprint-scope / behavior-witness / replay-provenance / sampling /
  pending-historical / certification-use / staged-abstraction fields) the ORD-LIFE-CERT acceptance
  artifact must instantiate.
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — boundary-awareness (live
  first-proof village/fixture ontology; informs fixture realism — actors, places, food sources, sleep
  places, workplaces — but is not an ORD-LIFE-CERT target).

**Structural precedent — the cert-spec pattern you must mirror (read in full)**
- `archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`
  — **[primary]** the freshest, most-similar **audit-spec template**: single-gate audit contract
  decomposed into numbered audit points (`EPI-01…EPI-11`), per-seam evidence obligations, fixture
  families, failure-diagnostic-by-layer, mutation posture, and acceptance-artifact section shape. Your
  spec is the ordinary-life-seam analogue of this.
- `archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md`
  and `archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md` — **[primary]** the
  earlier audit-spec templates (`SPINE-01…`, `P0-01…P0-10`); useful for cross-checking the established
  audit anatomy.
- `archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`,
  `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`, and
  `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` —
  **[primary]** the non-executable spec structure, source-discipline section, mutation posture
  (cargo-mutants, configured guarded-layer coverage, survivor triage register), and "what the
  implementing session must run/prove/record/package" framing. (These are remediation specs — read
  them for *structure and mutation discipline only*; the ORD-LIFE-CERT remediation spec itself is a
  later, separate deliverable, not this one.)
- `archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`
  — **[primary]** a *scoped-remediation (failed-floor)* acceptance artifact (shows how a survivor
  floor is surfaced and the verdict scoped).
- `archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md` and
  `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` —
  **[primary]** *passing* acceptance artifacts (the shape the eventual ORD-LIFE-CERT artifact targets).
- `archive/reports/0040_epi_cert_mutation_triage_register.md` — **[primary]** the survivor
  triage-register format the ordinary-life mutation posture should reuse.

**Historical evidence — read as history only (never as live certification)**
- `archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md` — the original
  Phase-3A needs/routines/intentions/planner/no-human-day implementation. Per `docs/2-execution/03`,
  "Phase 3A landed" is *evidence for* ORD-LIFE-CERT, **not** certification. (Its §12 success-recovery
  variant is explicitly staged — see doc `06`'s canonical-recovery section.)
- `archive/specs/0006_*`, `0007_*`, `0008_PHASE_3A_ANTI_CONTAMINATION_HARDENING_SPEC.md` — the early
  Phase-3A hardening specs; per the ledger and `docs/2-execution/03`, these landed as *evidence that
  earlier ordinary-life gates were too weak* and that anti-contamination must be a permanent gate.
- `archive/specs/0014_*` through `archive/specs/0025_*` (the Phase-3A ordinary-life/anti-contamination
  hardening series — e.g. `0014`, `0016`, `0020`, `0022`, `0025`) — each contributes scoped evidence
  toward `ORD-LIFE-CERT` per the ledger, but **is not certification**. Read them for what was hardened
  historically (need accounting, duration terminals, single-charge, stuck diagnostics, planner trace
  honesty, no-human metrics, EMERGE-OBS baselines); the audit must re-prove these under live doctrine,
  not presume them.

(Also available for orientation: the campaign's prior `archive/reports/*-research-brief.md`,
including `archive/reports/epi-cert-research-brief.md` — the brief that commissioned your lineage
predecessor — and `archive/reports/phase3a-ordinary-life-needs-routines-hardening-research-brief.md`.
Not load-bearing — the live spec doctrine above governs.)

---

## 3. Settled intentions (final — do not reopen)

1. **The determination is pre-settled and doctrine-forced.** `P0-CERT passed` (0037 artifact),
   `SPINE-CERT passed` (0039 artifact), **and** `EPI-CERT passed` (0041 artifact) are the current
   admissibility state; the gate ladder (`docs/2-execution/03`) and the ledger's "Next known execution
   move" make the **`ORD-LIFE-CERT` certification** (gate 4) the single next admissible spec. The
   ladder explicitly invalidates any Phase-4/expansion/feature spec attempted before `ORD-LIFE-CERT`
   (and `FIRST-PROOF-CERT` after it) pass. **Open the deliverable with a short, evidence-based
   confirmation of this determination** (cite the gate ladder + ledger + the three passed predecessor
   gates). Do **not** survey alternative "next features", and do **not** propose gameplay expansion.

2. **Breadth: one comprehensive spec covering all ORD-LIFE-CERT seams.** ORD-LIFE-CERT, as named in
   `docs/2-execution/03` and detailed in `docs/2-execution/06`, certifies: needs, routines,
   intentions, no-human ordinary life, planner traces, and stuck diagnostics under the actor-known
   transaction. Cover **all** of these as a single gate, the way 0040 covered the eleven epistemic
   seams (and 0038 the eight spine seams) in one audit spec. Decompose into explicitly numbered audit
   points (e.g. `ORD-LIFE-01 … ORD-LIFE-NN`) with their own evidence obligations. The exact seam
   decomposition is **yours to design** from the live doctrine and the real code seams (§5), but the
   audit points must, between them, cover **all 10 numbered `ORD-LIFE-CERT`-pass conditions in
   `docs/2-execution/06`** and **every fixture family in that doc's required-adversarial-fixtures
   table**. A reasonable starting cut: (i) needs as bounded event-sourced pressures + single-charge-
   per-actor/per-need/per-tick accounting + the single-owner derived-accounting seam; (ii) candidate
   generation from needs/routines using actor-known inputs only (no hidden-truth target); (iii)
   intention lifecycle (active/continued/suspended/interrupted/completed/failed/abandoned/replaced;
   possession does not reset); (iv) routine templates as defeasible HTN machinery (applicability,
   method families, interruptors, failure modes — a routine without failure modes is invalid);
   (v) routine **temporal-premise** mechanism (modeled premise vs. ground-truth time — see intention 5);
   (vi) method/HTN selection + local planning citing actor-known provenance, planner-budget discipline,
   coherent fallback; (vii) planner traces (selected + rejected candidates/methods visible in debug
   without feeding actor input); (viii) ordinary actions / affordances / durations (sleep affordance
   enforcement, duration terminal closure, open-duration authority, action-emitted tick-ledger,
   no-teleport without movement ancestry); (ix) no-human advancement + no-human metrics counting only
   real progress / modeled waits / typed stuck-or-failure outcomes; (x) stuck diagnostics + cross-tick
   stuck detection (no-progress-past-window, repeated-idle) with responsible layer + blocker code;
   (xi) scheduler-no-direct-dispatch boundary (ordinary actions are not dispatched directly from
   needs/routines); (xii) replay rebuilds ordinary-life metrics + diagnostics deterministically.
   Merge/split as the doctrine and code warrant — but cover every gate-4 theme and every doc-`06`
   condition.

3. **Form: a non-executable audit / certification spec** in the exact 0036/0038/0040 lineage. The
   spec specifies *what the implementing session must run, prove, record, and package* — it does
   **not** render a pass/fail verdict, and does **not** assert the current code passes. You (Session 2)
   cannot run `cargo test`, `cargo-mutants`, or replay, so do not fabricate results. For each seam the
   spec must define: exact files/seams to audit; foundation + architecture dependencies; required
   positive **and** adversarial fixtures (incl. food-unavailable / no-teleport / hidden-truth-planning
   / possession-does-not-reset / routine-blocker fixtures); event/replay/projection/determinism
   evidence; actor-known provenance evidence where the seam touches it; debug-quarantine evidence where
   the seam touches it (planner trace honesty); the mutation-testing posture (configured guarded-layer
   coverage + survivor triage register, per 0037/0039/0041); and failure-diagnostics named by
   responsible layer (the layer list in `docs/2-execution/03` §"Gate failure handling"). You **may**
   include a single clearly-labeled **"preliminary, non-certifying static survey"** of what reading the
   code at `98dc042` suggests about likely gate satisfaction/risk — explicitly marked as informative,
   not certification — or omit it. Authoritative pass/fail belongs to the implementing session.

4. **Scope is the ORD-LIFE-CERT *audit* spec only.** Produce the audit spec (the 0036/0038/0040
   analogue), not a remediation spec. If the implementing session's mutation run later leaves
   survivors, that triggers a **separate** ORD-LIFE-CERT mutation-remediation + replacement spec (the
   0037/0039/0041 analogue) — **do not pre-author it here**, and do not fold an anticipatory
   remediation plan into this spec. The spec's acceptance-artifact contract simply names what would
   render `ORD-LIFE-CERT passed` and what a failed-floor verdict (`ORD-LIFE-CERT scoped remediation`)
   would route to.

5. **Routine temporal-premise: audit the mechanism here, route the bundle forward.** `docs/2-execution/06`
   owns the "Routine Temporal Premises and Adaptation Proof" mechanism (a routine must act from a
   *modeled* temporal premise — assignment / memory / observation / public cue / record / testimony /
   institutional context / source-backed inference — and must **wait or fail** when only ground-truth
   schedule time would justify the action; scheduler awakenings and elapsed-time accounting do **not**
   count as routine-premise evidence). **Include this mechanism as an ORD-LIFE-CERT audit point** (a
   "routine-temporal-premise" seam): the audit certifies that routine selection cannot consult true
   schedule time, and that a no-human scenario shows a routine succeeding from a modeled premise and
   waiting/failing without one. **However**, the *consolidated temporal evidence bundle* that
   `docs/2-execution/03` §"Temporal Cascade" assigns to `FIRST-PROOF-CERT` (temporal-firewall from `04`,
   routine temporal proof from `06`, embodied temporal rendering from `07`, temporal fixtures from `09`,
   temporal diagnostics from `10`) **remains a FIRST-PROOF-CERT obligation** — name it explicitly in the
   spec's tolerated-deferrals section as a *routed dependency*, not an ORD-LIFE-CERT pass/fail line. The
   gate certifies the routine-premise *mechanism*; the cross-seam temporal *bundle* is downstream.

6. **Numbering & placement.** Filename: `specs/0042_ORD_LIFE_CERT_<DESCRIPTIVE_NAME>.md`. The staging
   series in `archive/specs/` is contiguous `0002 → 0041` with `specs/` currently empty, so the next
   staging number is **0042** (recent git history shows each spec archived by a plain rename with no
   renumber/restart; the EPI-CERT lineage predecessor chose `0040`/`0041`, so this continues the
   sequence; the live ledger's `0001`/`0003` numbering does not block continuing the staging
   sequence). Stage under `specs/`; it is archived to `archive/specs/` on acceptance. Header block must
   mirror 0040: staging path, target repository, **target commit
   `98dc0421211e6c9881d9c6679b9df74525e392bb`**, spec series, status, **Work posture: `Certification`**,
   and **admissibility posture** citing the 0037 (`P0-CERT passed`), 0039 (`SPINE-CERT passed`), and
   0041 (`EPI-CERT passed`) acceptance artifacts, naming the gates this spec consumes.
   `assumption:` stage under `specs/` (not directly in `docs/4-specs/`), per the repo's
   stage-then-archive convention — the user may relocate if desired.

7. **Source discipline (carry verbatim from the ledger).** Manifests are path inventory only;
   branch/default-branch/code-search/repo-metadata are not proof of target-commit content; archived
   specs/tickets/reports are cited as **history only**, never as live certification; the spec may
   **not** amend invariants, redefine/weaken gate semantics, or **mint a new gate code** —
   `ORD-LIFE-CERT` is a phase-certification *label* that composes the canonical gates/review-artifacts
   from architecture `00` and the execution index `00`, exactly as `docs/2-execution/03` and
   `docs/2-execution/06` state. Use `actor-known` for the actor case and `holder-known context` as the
   system-wide term.

8. **EMERGE-OBS stays observer-only — and ORD-LIFE-CERT is where it is exercised.** The no-human
   ordinary day *is* the first-proof living-world acceptance corpus, and the Phase-3A history records
   repeated `EMERGE-OBS` baselines. The spec may name the observer-only `EMERGE-OBS` artifact as an
   evidence-package member of the ORD-LIFE-CERT package, but it **never** becomes a phase gate or
   pass/fail threshold (per `docs/2-execution/10` and `03`). Treat it as an evidence-package member,
   not an audit point with a pass condition.

9. **`assumption:`** the spec may include the optional preliminary static survey (intention 3); if
   included it is explicitly labeled "preliminary, not certification". Default is to permit it; omit if
   it would risk reading as a verdict.

---

## 4. The task

Author the **`ORD-LIFE-CERT` certification spec** (`specs/0042_*`): the next admissible spec on the
Tracewake certification ladder. It is a *new-spec*, *non-executable certification* deliverable — an
audit plan plus acceptance contract that the implementing session will execute to certify the
simulation's **ordinary-life substrate** (needs, routines, intentions, no-human ordinary life, planner
traces, and stuck diagnostics, all under the actor-known decision transaction) against live
foundation/architecture/execution doctrine at commit `98dc042`. The spec must be evidence-complete
(every seam → audited files, required positive + adversarial fixtures, proof mechanics, failure-
diagnostic layers, acceptance-artifact section) so a Tracewake implementing session can execute it
without further design. It must cover all 10 numbered `ORD-LIFE-CERT`-pass conditions and every
required adversarial fixture family in `docs/2-execution/06`.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular the actual
code seams the ordinary-life certification will audit. Verified-present at `98dc042` and load-bearing
(name the real modules in the audit inventory; confirm each at `98dc042` and add any you find). All
core paths are under `crates/tracewake-core/src/` and tests under `crates/tracewake-core/tests/`:

- **Needs / accounting**: `agent/need.rs` (`NeedKind` {Hunger, Fatigue, Safety}, `NeedBand`,
  `NeedState`, `NeedThresholdCrossing`, `NeedChangeCause`, `NeedPressure`), `need_accounting.rs`
  (`TickRegime` {Awake, Asleep, Working}, `classify_actor_tick_regimes_with_start`,
  `open_body_exclusive_starts`, `is_duration_terminal`, `terminal_ticks_by_start`,
  `DuplicateDurationTerminal`, `DurationInterval`), `actions/defs/need_events.rs`
  (`build_need_delta_and_threshold_events`, `NeedDeltaEventSpec`).
- **Candidate generation / routines / intentions / methods**: `agent/generation.rs` (need-driven
  candidate synthesis), `agent/candidate.rs` (`CandidateGoal`, `CandidateGoalSource`),
  `agent/routine.rs` (`RoutineFamily`, `RoutineStep`, `RoutineCondition`, `RoutineTemplate`,
  `RoutineExecution`, `RoutineStepStatus`, `RoutineDiagnosticKind`), `agent/methods.rs`
  (`phase3a_routine_templates`, `family_for_goal`), `agent/htn.rs` (`select_phase3a_method`,
  `resolve_template_conditions`, `MethodSelectionFailure`), `agent/intention.rs` (`Intention`,
  `IntentionStatus`, `ActorIntentions`).
- **Planning / traces / decision**: `agent/planner.rs` (`PlannerGoal`, `LocalPlanRequest`,
  `plan_local_actions`, `PlannedProposal`, `LocalPlan`, `LocalPlanTrace`, `LocalPlanFailure`,
  `DEFAULT_PLANNER_BUDGET`, the goal-specific `plan_*` fns + coherent fallback), `agent/decision.rs`
  (`DecisionInput`, `select_goal_and_trace`, `ActorKnownInputRef`, `ActorKnownInputSourceClass`),
  `agent/trace.rs` (`DecisionTrace`, `DecisionOutcome`, `DecisionTraceRecord`, `BlockerCategory`,
  `BlockerCode`, `StuckDiagnostic`, `StuckResultingStatus`, `TypedDiagnosticFields`,
  `ResponsibleLayer`).
- **Actor-known transaction tie-ins**: `agent/transaction.rs` (`ActorDecisionTransaction::run`,
  `ActorDecisionTransactionInput`, `ActorDecisionTransactionOutcome` {Proposed, Stuck}, `SealedProposal`,
  `dangling_provenance_diagnostic`), `agent/actor_known.rs` (`ActorKnownFact`, `ActorKnownProvenance`,
  `SourceEventIds`, `ActorKnownPlanningContext`), `agent/no_human_surface.rs`
  (`SealedActorKnownSurface`, `NoHumanActorKnownSurfaceBuilder::from_projection`,
  `NoHumanActorKnownSurfaceRequest`), `agent/perception.rs` (place perception → observation recording).
- **Ordinary actions / affordances / durations**: `actions/defs/sleep.rs` (`require_sleep_affordance`,
  `build_sleep_start_event`, sleep payload), `actions/defs/work.rs`, `actions/defs/eat.rs`,
  `actions/pipeline.rs` (validation, reservation, duration-terminal-duplicate rejection),
  `actions/proposal.rs` (`Proposal`, `ProposalSource`, `ProposalSourceContext` — holder-known context
  id/hash parity), `state.rs` (`SleepAffordanceState`).
- **No-human orchestration / metrics**: `scheduler.rs` (`run_no_human_day`, `advance_no_human`,
  `NoHumanDayConfig`, `NoHumanDayReport`, `DayWindow`, `NoHumanAdvanceReport`, `SchedulePhase`,
  `latest_action_tick_delta_tick`), `projections.rs` (`no_human_day_metrics`, `NoHumanDayMetrics`).
- **Events / replay**: `events/envelope.rs` (`EventEnvelope`, `EventKind` — the ordinary-life event
  family: `NoHumanDayStarted/Completed`, `NoHumanAdvanceStarted/Completed`, `SleepStarted/Completed/
  Interrupted`, `WorkBlockStarted/Completed/Failed`, `NeedDeltaApplied`, `NeedThresholdCrossed`,
  `Intention*`, `RoutineStep*`, `ContinueRoutineProposed/Accepted/Rejected`, `StuckDiagnosticRecorded`,
  `DecisionTraceRecorded`, `ReplayProjectionRebuilt` — and `EventReplayHandling`), `events/log.rs`,
  `events/apply.rs`, `events/mutation.rs`, `replay/rebuild.rs` (`rebuild_projection`,
  `rebuild_decision_context_hashes`), `replay/report.rs` (`run_replay`), `checksum.rs`
  (decision-context / holder-known hashing).
- **Test / fixture harnesses** (`crates/tracewake-core/tests/`): `no_human_capstone.rs`,
  `acceptance_gates.rs`, `emergence_ledger.rs`, `event_schema_replay_gates.rs`, `golden_scenarios.rs`,
  `hidden_truth_gates.rs`, `support/generative.rs`, `support/mod.rs`; plus
  `crates/tracewake-content/tests/golden_fixtures_run.rs`.

Verify these exist at `98dc042` and name them precisely; the type/function names above are orientation
from a baseline read, not authority — confirm against the tree.

Research online as deeply as needed — similar implementations and prior art — wherever it sharpens
the audit design: **defeasible / HTN planning** and goal-task decomposition for agents (how to prove a
routine is "machinery, not a script" with explicit failure modes and fallbacks); **need/drive-based
agent architectures** and utility/motivation models (decay, thresholds, interrupts) in life-sim and
believable-agent systems; **autonomy / "the world runs without a human" proofs** (closed-loop
simulation acceptance, liveness vs. silent-stall detection); **partial-observability planning** (acting
from actor-known belief state, not ground truth — POMDP-adjacent framing, information-set discipline);
**property-based and metamorphic testing** for planners, need accounting (single-charge / conservation
invariants), and replay determinism; **mutation testing** strategy for decision/planning kernels
(scoping guarded-layer coverage, triaging survivors); and **liveness/no-progress detection** techniques
(how to distinguish a legitimate modeled wait from a stuck loop). Cite sources for any external claim
that shapes a decision. The *deep* research is yours to perform; the determination of *which* spec is
not — that is settled in §3.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every property the spec
  certifies must satisfy it; a genuine divergence would require amending an invariant first — never
  design against it. The spec does not amend invariants.
- Authority order: if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong.
- No backwards-compatibility shims or alias paths in new work.
- Anti-contamination: no simulation fact may be born from prose; preserve event-sourced causality,
  subjective epistemics, ordinary agents, possession parity, fallible institutions, validation/replay.
  Specifically for ordinary life: a hungry actor must not teleport to true food, starve silently, or
  wait forever without cause; candidate generation and method selection consume **actor-known inputs
  only**; the scheduler may not dispatch ordinary actions directly from needs/routines; validation
  truth may not propose; need/duration deltas are charged exactly once by the single owning seam;
  planner traces stay non-diegetic (visible in debug, never fed back as actor input); possession does
  not reset need/routine/intention/memory/actor-known state.
- The crate dependency direction (`core` ← `content` ← `tui`) must never invert.
- Mint no new gate code, status enum, obligation code, or invariant ID. Cross-reference existing ones.

---

## 7. Deliverable specification

Produce **one** downloadable markdown document:

- **`specs/0042_ORD_LIFE_CERT_<DESCRIPTIVE_NAME>.md`** — **new** numbered staging spec (not a
  replacement). `<DESCRIPTIVE_NAME>` is yours to choose in the established ALL-CAPS-underscored style
  (e.g. `ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC`).

This is a *new-spec* deliverable: there is **no** paired `-research-report.md`. The numbering/ledger/
staging-series rules in §3.6 (staging series → `0042`) apply; do not treat this as a `reports/`
analysis report.

Required structure (follow 0036/0038/0040 anatomy, retargeted to the ordinary-life substrate):

1. **Header block** — staging path, target repo, **target commit
   `98dc0421211e6c9881d9c6679b9df74525e392bb`**, spec series (numbered staging spec, archived on
   acceptance), status, `Work posture: Certification`, admissibility posture citing `P0-CERT passed`
   (the 0037 acceptance artifact), `SPINE-CERT passed` (the 0039 acceptance artifact), and `EPI-CERT
   passed` (the 0041 acceptance artifact), and a one-line statement that the doc is **non-executable**.
2. **Determination confirmation** — short, cited: why ORD-LIFE-CERT is the next admissible spec (gate
   ladder + ledger; predecessor gates `P0-CERT` / `SPINE-CERT` / `EPI-CERT` passed).
3. **Source discipline, freshness, admissibility** — per §3.7.
4. **Authority & dependency declarations** — controlling doctrine order; primary ordinary-life sources;
   higher-tier foundation/architecture/execution dependencies (the §2 [primary] list).
5. **The ORD-LIFE-CERT audit contract** — all ordinary-life seams as numbered audit points
   (`ORD-LIFE-01…`), each with: seam definition, audited files/modules (real, verified at `98dc042`),
   the doctrine it must satisfy, required positive + adversarial fixtures (incl. food-unavailable,
   no-teleport, hidden-truth-planning, possession-does-not-reset, routine-blocker, planner-trace),
   event/replay/projection/determinism evidence, actor-known-provenance and debug-quarantine evidence
   where applicable, and the exact commands the implementing session runs. **A coverage table** mapping
   each audit point to the 10 numbered `ORD-LIFE-CERT`-pass conditions and the required fixture
   families in `docs/2-execution/06` (so completeness is auditable).
6. **Mutation-testing posture** — configured guarded-layer coverage over the ordinary-life seams,
   survivor-triage-register obligation (reuse the `archive/reports/0040_epi_cert_mutation_triage_register.md`
   format), phase-entry evidence rule.
7. **Failure handling** — diagnostics named by responsible layer (the `docs/2-execution/03` layer list
   — incl. candidate generation, intention lifecycle, method selection, local planning, proposal
   construction, scheduler ordering, action validation, event application, projection/replay,
   view-model rendering, debug quarantine, tests/fixtures, documentation status); a failed seam
   produces a named remediation spec/report (the future ORD-LIFE-CERT mutation-remediation spec), not a
   relabeled phase exception.
8. **Acceptance-artifact contract** — instantiate `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:
   what the implementing session records to render `ORD-LIFE-CERT passed`, including evidence-status,
   fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical,
   certification-use, and staged-abstraction fields. Name the observer-only `EMERGE-OBS` artifact as an
   evidence-package member (never a gate). State that this spec specifies the audit; it does not render
   the verdict; a failed mutation floor yields `ORD-LIFE-CERT scoped remediation` routed to a separate
   remediation spec.
9. **(Optional) Preliminary static survey** — clearly labeled "preliminary, not certification".
10. **Tolerated deferrals** — explicitly out of ORD-LIFE-CERT scope and routed forward: the
    consolidated **temporal evidence bundle** to `FIRST-PROOF-CERT` (per §3.5 — the routine-premise
    *mechanism* is audited here, the cross-seam bundle is downstream); the success-recovery
    ordinary-life variant staged to Phase 3B (per doc `06`'s canonical-recovery section); institutions /
    wrong-suspicion / records to `PHASE-4-ENTRY`; notices / travel / regional scale / LOD to
    `SECOND-PROOF-ENTRY`; and a statement that SPINE-CERT and EPI-CERT seams are already certified and
    are **not** re-audited here. Do not audit them.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec opens with a cited confirmation that ORD-LIFE-CERT is the next admissible move
      (predecessor gates `P0-CERT`, `SPINE-CERT`, and `EPI-CERT` passed); it does not propose
      feature/expansion work.
- [ ] Every gate-4 theme — needs, routines, intentions, no-human ordinary life, planner traces, stuck
      diagnostics under the actor-known transaction — is covered as numbered audit points, each with
      audited files, positive + adversarial fixtures, determinism/replay evidence, and failure-
      diagnostic layers.
- [ ] A coverage table maps the audit points onto **all 10 numbered `ORD-LIFE-CERT`-pass conditions**
      and **every required adversarial fixture family** in `docs/2-execution/06`; none is missing.
- [ ] The routine **temporal-premise mechanism** is audited here, while the consolidated temporal
      evidence bundle is routed forward to FIRST-PROOF-CERT in tolerated deferrals.
- [ ] The deliverable is non-executable: it specifies the audit and acceptance contract, renders **no**
      pass/fail verdict, and fabricates no test/mutation/replay results. Any static survey is labeled
      "preliminary, not certification".
- [ ] Scope is the ORD-LIFE-CERT **audit** spec only; no remediation spec is authored or pre-planned
      beyond naming the route a failed floor would take.
- [ ] Filename is `specs/0042_ORD_LIFE_CERT_*.md`; header carries posture `Certification` and
      admissibility citing the 0037 (`P0-CERT passed`), 0039 (`SPINE-CERT passed`), and 0041 (`EPI-CERT
      passed`) artifacts; target commit is `98dc042`.
- [ ] No new gate code / invariant ID / status enum is minted; ORD-LIFE-CERT is treated as a label that
      composes existing canonical gates; archived specs (incl. Phase-3A `0005`–`0025`) are cited as
      history only; `EMERGE-OBS` stays observer-only and never a gate.
- [ ] No doctrine weakens an upstream tier; crate dependency direction is preserved; anti-contamination
      properties hold (no teleport to true food, single-charge accounting, no-direct-dispatch, actor-
      known-inputs-only, possession-does-not-reset, non-diegetic planner traces).
- [ ] The acceptance-artifact contract instantiates `docs/4-specs/0003`'s fields.
- [ ] Every external claim is cited; every file named in §2 and in the audit inventory exists at commit
      `98dc0421211e6c9881d9c6679b9df74525e392bb`.
