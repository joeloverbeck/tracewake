# FIRST-PROOF-CERT certification spec — deep-research brief (paste into ChatGPT-Pro)

> **You are Session 2.** Produce the deliverable directly as a downloadable markdown
> document. Do **not** interview, do **not** ask clarifying questions — the requirements
> below are final. If a genuine contradiction makes a requirement impossible, state it in
> the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-21_1541da2.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust (event-sourced
kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers
govern later ones (if execution conflicts with architecture or foundation, execution is wrong).
**Fetch every file from commit `1541da274180ecd40f52583d86704990cb55e74c` (short `1541da2`)** —
the manifest reflects exactly that tree. Do **not** adopt a different "commit of record" from any
file you read: commit hashes that appear *inside* archived specs/acceptance artifacts (e.g.
`c819bbe`, `726b2a1`, `92ba47f`, `2a37b04`) are that artifact's own audit provenance, not this
baseline. A manifest is path inventory only.

**This brief continues an active certification campaign.** The repo runs a phase-certification
ladder (`docs/2-execution/03`): `P0-DOC → P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT →
FIRST-PROOF-CERT → PHASE-4-ENTRY → SECOND-PROOF-ENTRY`. Gates 1–4 are now all passed; this brief
commissions **gate 5, `FIRST-PROOF-CERT`** — the capstone that certifies the first missing-property
playable proof as one coherent gate set. The immediately preceding work delivered the
**ORD-LIFE-CERT pair**, which is your **lineage predecessor** (this brief is a *delta* on it,
retargeted from the ordinary-life substrate to the integrated first-proof acceptance contract — do
not re-commission it):

- **0042** (`archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md`)
  — the `ORD-LIFE-CERT` audit spec; verdict came back `ORD-LIFE-CERT scoped remediation` because the
  configured mutation lane left a survivor floor.
- **0043** (`archive/specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`)
  — the scoped mutation remediation + replacement certification; verdict **`ORD-LIFE-CERT passed`**
  for exact code/evidence command commit `c819bbee0282eb83386f7b58cab752b9e639a4af`. Its artifact
  (`archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md`)
  supersedes the 0042 artifact for `ORD-LIFE-CERT passed` citation.

The full current admissibility state is therefore: `P0-CERT passed` (0037 artifact),
`SPINE-CERT passed` (0039 artifact, commit `92ba47f14998e0ea2fc95502bc3b76c5909478ca`),
`EPI-CERT passed` (0041 artifact, commit `726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3`), **and**
`ORD-LIFE-CERT passed` (0043 artifact, commit `c819bbee0282eb83386f7b58cab752b9e639a4af`). Per the
gate ladder and the ledger's "Next known execution move" (`docs/4-specs/SPEC_LEDGER.md`, which
records all four passed gates and declares no further gate), the **single next admissible spec is the
`FIRST-PROOF-CERT` certification** (gate 5). The ladder states gate 5 is "Blocked until gates 1
through 4 pass" — that block is now lifted.

**The structural model you must mirror** (the cert-spec anatomy this campaign reuses — these are
*shape* precedents in §2, not delta seeds): the ORD-LIFE-CERT pair (0042 audit / 0043 remediation),
the EPI-CERT pair (0040 audit / 0041 remediation), the SPINE-CERT pair (0038 audit / 0039
remediation), and the P0-CERT pair (0036 audit / 0037 remediation). Your spec is the **audit spec**
of the next pair — the first-proof-integration analogue of 0036 / 0038 / 0040 / 0042. The eventual
FIRST-PROOF-CERT mutation-remediation replacement spec is **a later, separate spec** and is **out of
scope here** (it is authored only if/when the implementing session's mutation run leaves survivors —
do not pre-author it).

---

## 2. Read in full (authority order)

Read these before producing. The user's standing instruction is to read **all** of
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and
`docs/4-specs/*`. Within each tier, the entries marked **[primary]** are load-bearing for the
FIRST-PROOF-CERT spec; the remainder are *boundary-awareness* (read to know what is **out** of
FIRST-PROOF-CERT scope and must not be audited or "corrected" here — they belong to PHASE-4-ENTRY,
SECOND-PROOF-ENTRY, or already-passed earlier gates).

**Foundation (`docs/0-foundation/`)**
- `docs/README.md` — **[primary]** repository authority layering and the "execution is wrong if it
  conflicts with architecture/foundation" rule.
- `00_FOUNDATION_INDEX.md` — index/boundary map.
- `02_CONSTITUTIONAL_INVARIANTS.md` — **[primary]** `INV-001…INV-112`; every certified property must
  satisfy these. First-proof is downstream of the event-sourced-causality, subjective-epistemics,
  ordinary-agents, possession-parity, no-contamination, observer-only-emergence (`INV-111`), and
  temporal-authority (`INV-112`) invariants.
- `01_PROJECT_CHARTER.md` — **[primary]** the project's reason to exist; the first proof is the
  charter's executable demonstration (missing property, world runs without a human).
- `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — **[primary]** THE foundation contract for
  claims, beliefs, memory, expectation, and contradiction — the substrate the **MISSING-PROPERTY**
  gate certifies (an actor's source-backed expectation discovers absence and forms a contradiction).
- `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — **[primary]** ordinary-life/survival
  affordance contract; the no-human ordinary day the first proof composes.
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **[primary]** possession parity and the
  embodied/debug split the `POSSESSION-PARITY` and `VIEW-DEBUG-SPLIT` gates certify.
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **[primary]** THE first-playable acceptance-gate
  doctrine: the missing-property situation, the world that runs without knowing whether a human is
  present, the explicit non-goals, and the observer-only emergence-evidence requirement. This is the
  charter of the gate you are certifying.
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **[primary]** the actor-known
  cognition transaction and truth firewall; basis for the `TRUTH-FIREWALL` and `ACTOR-KNOWN` gates
  (no decision from unobserved ground truth; no culprit/hidden-food/suspect chosen by truth).
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **[primary]** the `EVENT` and `REPLAY` gate
  doctrine: every world change is an event with stable identity, causal ancestry, schema/version,
  deterministic ordering, replay effect; current state is projection, not authority.
- `13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — boundary-awareness (source notes; orientation only).
- `05, 07, 09, 10, 11` — boundary-awareness (needs/planning — ORD-LIFE territory, already certified;
  institutions — Phase 4; no-scripting/seeds; LOD/scale — second proof; LLM/speech — locked). Read to
  bound scope; not FIRST-PROOF-CERT audit targets. (`05`/`06`'s ordinary-life machinery is consumed as
  a certified building block via `ORD-LIFE-CERT passed`, not re-audited — see §3.2.)

**Architecture (`docs/1-architecture/`)**
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **[primary]** canonical gate/review-artifact
  composition; the FIRST-PROOF-CERT *label* composes gates defined here, it mints no new gate code.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — **[primary]** the one-way crate
  dependency rule (`core ← content ← tui`) the integrated proof must not invert.
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **[primary]** projection-rebuild and
  determinism contract: the `EVENT`/`REPLAY` gates and the unified-commit integrated re-proof depend
  on it (replay must identify divergence location, not only assert mismatch).
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **[primary]** holder-known/actor-known
  sealed contexts, truth firewall, provenance sufficiency/freshness — the substrate the `ACTOR-KNOWN`
  and `MISSING-PROPERTY` gates consume (expectation/contradiction cite provenance; truth never
  proposes targets).
- `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — **[primary]** THE
  architecture contract for claims/beliefs/observation/memory and **expectation contradiction** — the
  heart of the `MISSING-PROPERTY` certification surface (the genuinely-new surface, see §3.3).
- `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — **[primary]** the spatial/property
  model the missing-property scenario and no-human day move through (items, places, containers, doors,
  access, custody; no teleport to true food/property).
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — **[primary]** embodied view models
  expose only holder-known context; debug surfaces non-diegetic only — the `POSSESSION-PARITY` and
  `VIEW-DEBUG-SPLIT` gates.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **[primary]** what a gate artifact
  must contain; typed diagnostics and review-artifact obligations; the integrated acceptance shape.
- `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — boundary-awareness: story-sifting /
  leads / notices are **out** (second-proof/Phase-4). Read precisely to keep the missing-property gate
  from drifting into investigation/quest machinery — the first proof has **no** culprit flag, suspect,
  clue, or quest board.
- `04, 05, 07, 08, 12, 14` — boundary-awareness (action pipeline — read `04`/`05` only for the
  no-direct-dispatch boundary and the ordinary-life transaction the proof composes; speech/LLM;
  institutions; LOD/prehistory; forbidden misreads — useful terminology discipline, not an audit
  target). Read to bound scope.

**Execution (`docs/2-execution/`)**
- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — **[primary]** execution-tier authority and the canonical
  gate index (FIRST-PROOF-CERT is a phase-certification label that consumes canonical gate evidence).
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — **[primary]** the
  post-0008 baseline, the code-audit boundary, and the three admissibility postures
  (`passed` / `scoped remediation` / `not applicable`).
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — **[primary]** THE first-proof
  scope and acceptance contract you are certifying: first-proof identity, current baseline status, the
  **nine acceptance-contract gates** (`EVENT`, `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `POSSESSION-PARITY`,
  `NO-HUMAN-ORDINARY-LIFE`, `MISSING-PROPERTY`, `VIEW-DEBUG-SPLIT`, `REPLAY`, `FIXTURE-NEGATIVE`), the
  observer-only emergence artifact, the **nine first-proof scenario families** table, the "definition
  of first-proof done", and the explicit non-goals. **Your audit points must cover all nine gates and
  every scenario family in this doc.**
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **[primary]** defines `FIRST-PROOF-CERT`
  (gate 5): "Certify the first missing-property playable proof as a coherent gate set. Blocked until
  gates 1 through 4 pass." Also: the **Temporal Cascade** placement (first-proof acceptance must
  include temporal-firewall evidence from `04`, routine temporal proof from `06`, embodied temporal
  rendering from `07`, temporal fixtures from `09`, temporal diagnostics from `10` before
  FIRST-PROOF-CERT is complete), gate-evidence requirements, gate-failure handling (named layers),
  valid future-spec postures, and the EMERGE-OBS observer-only rule.
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **[primary]** THE anti-
  contamination/truth-firewall execution gates (no fact born from prose; no decision from unobserved
  ground truth; validation truth may not propose; no teleport to true food/property; no culprit
  truth). Owns the **temporal-firewall** evidence routed into the temporal bundle (§3.4).
- `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **[primary]** owns the no-human ordinary
  day proof and the **"Routine Temporal Premises and Adaptation Proof"** routed into the temporal
  bundle. The no-human-day machinery is consumed via `ORD-LIFE-CERT passed`; read this for the
  no-human acceptance the integrated proof composes and for the routine-temporal evidence FIRST-PROOF
  now owns.
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **[primary]** the embodied view-model /
  possession / debug-quarantine execution proof (`VIEW-DEBUG-SPLIT`, `POSSESSION-PARITY`) and the
  **embodied temporal rendering** evidence routed into the temporal bundle.
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — **[primary]** golden-fixture and replay
  acceptance obligations; positive + adversarial fixture families (incl. expectation-contradiction,
  no-hidden-truth planning, no-direct-dispatch, no-human special case, no debug leakage, no
  quest/culprit data, no marker-only ordinary life, no replay drift) and the **temporal fixture
  families** routed into the temporal bundle.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **[primary]** evidence-honesty rule,
  diagnostics-by-layer, review-artifact obligations, EMERGE-OBS observer-only discipline, the single
  diagnostic home, and the **temporal diagnostics** routed into the temporal bundle.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — **[primary]** the
  scheduler/action-pipeline/no-direct-dispatch boundary the integrated proof must hold (ordinary
  actions are not dispatched directly from needs/routines; no scheduler-authored state deltas). The
  SPINE-CERT *certification of the pipeline itself* is passed and is **not** re-audited; read for the
  boundary the first-proof gate set must respect.
- `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — boundary-awareness: Phase-4 entry;
  wrong-suspicion/records are **out**. Read precisely to keep the missing-property gate from importing
  investigation machinery.
- `08, 12, 13` — boundary-awareness (data authoring/schema — read where the proof consumes authored
  fixtures/seed knowledge; deferred second-proof; research notes). Read to bound scope.

**Reference (`docs/3-reference/`)**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **[primary]** review checklist questions.
- `01_DESIGN_RISK_REGISTER.md` — **[primary]** standing risks first-proof must resist (hidden-truth
  leakage into planning/missing-property discovery, protagonist gravity / human special-case,
  silent starvation/teleport, marker-only ordinary life, replay drift, Goodharting emergence/no-human
  metrics, stale-premise routine selection, debug leakage into embodied cognition).
- `02_GLOSSARY.md` — **[primary]** canonical terminology (`actor-known`, `holder-known context`,
  `missing property`, `expectation contradiction`, `no-human proof`, `possession parity`,
  `EMERGE-OBS`, `behavioral progress`, `stuck diagnostic`, etc.) the spec must use exactly.

**Specs (`docs/4-specs/`)**
- `SPEC_LEDGER.md` — **[primary]** the live ledger: authority posture, source discipline, archived-spec
  status, the four passed-gate "Next known execution move" entries, and the numbering/admissibility
  rules. Note: it records all four gates passed and declares **no** FIRST-PROOF-CERT — your spec is the
  move it implies but does not yet name.
- `README.md` — **[primary]** rules for future specs (declare authority posture; declare exactly one
  admissibility posture; gate codes as cross-references only; terminology; no files for symmetry).
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — **[primary]** the review-artifact template (evidence-status
  / fingerprint-scope / behavior-witness / replay-provenance / sampling / pending-historical /
  certification-use / staged-abstraction fields) the FIRST-PROOF-CERT acceptance artifact must
  instantiate.
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — **[primary]** the live first-proof
  village/fixture ontology (actors, places, food/sleep/work sites, the missing-property setup). It is
  the concrete substrate the `MISSING-PROPERTY` gate and the scenario families certify; read it as a
  load-bearing fixture-contract source, not just boundary-awareness.

**Structural precedent — the cert-spec pattern you must mirror (read in full; *shape* model, not a
delta seed)**
- `archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md`
  — **[primary]** the freshest, most-similar **audit-spec template**: single-gate audit contract
  decomposed into numbered audit points (`ORD-LIFE-01…`), per-seam evidence obligations, fixture
  families, failure-diagnostic-by-layer, mutation posture, coverage table, acceptance-artifact section
  shape. Your spec is the first-proof-integration analogue of this.
- `archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`,
  `archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md`,
  `archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md` — **[primary]** the
  earlier audit-spec templates (`EPI-01…`, `SPINE-01…`, `P0-01…P0-10`); cross-check the established
  audit anatomy. (0036 in particular is the closest prior *cross-cutting / multi-seam* audit, like
  FIRST-PROOF.)
- `archive/specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`,
  `archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`,
  `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`,
  `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` —
  **[primary]** the non-executable spec structure, source-discipline section, mutation posture
  (cargo-mutants, configured guarded-layer coverage, survivor triage register), and "what the
  implementing session must run/prove/record/package" framing. Read these for *structure and mutation
  discipline only*; the FIRST-PROOF-CERT remediation spec itself is a later, separate deliverable —
  not this one.
- `archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md`,
  `archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`,
  `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`,
  `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` —
  **[primary]** *passing* acceptance artifacts (the shape the eventual FIRST-PROOF-CERT artifact
  targets). These are also the artifacts your header's admissibility posture **cites** for the four
  consumed gates.
- `archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`
  and `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md`
  — **[primary]** *scoped-remediation (failed-floor)* acceptance artifacts (shows how a survivor floor
  is surfaced and the verdict scoped — the shape a `FIRST-PROOF-CERT scoped remediation` verdict would
  take).
- `archive/reports/0040_epi_cert_mutation_triage_register.md` and
  `reports/0043_ord_life_cert_mutation_triage_register.md` — **[primary]** the survivor triage-register
  format the first-proof mutation posture should reuse.

**Historical evidence — read as history only (never as live certification)**
- `archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md`
  — the original Phase-2A expectation-contradiction + possession-parity implementation; the historical
  root of the **MISSING-PROPERTY** surface. Per `docs/2-execution/03`, "Phase 2A landed" is *evidence
  for* EPI-CERT, not certification — read for what was built, re-prove under live doctrine.
- `archive/specs/0002`, `0003`, `0005`–`0035` — the Phase-1/2A/3A implementation + hardening +
  doctrine-alignment series; per the ledger each contributes scoped historical evidence but **is not
  certification**. Read selectively for what was hardened (event/replay totality, epistemic substrate,
  need accounting, no-human metrics, EMERGE-OBS baselines, temporal/completeness alignment); the audit
  re-proves these as a coherent set under live doctrine, not by presumption.

(Also available for orientation: the campaign's prior `reports/*-research-brief.md` and
`archive/reports/*-research-brief.md`, including `reports/ord-life-cert-research-brief.md` — the brief
that commissioned your lineage predecessor. Not load-bearing — the live spec doctrine above governs.)

---

## 3. Settled intentions (final — do not reopen)

1. **The determination is pre-settled and doctrine-forced.** `P0-CERT passed` (0037 artifact),
   `SPINE-CERT passed` (0039 artifact), `EPI-CERT passed` (0041 artifact), **and** `ORD-LIFE-CERT
   passed` (0043 artifact) are the current admissibility state; the gate ladder (`docs/2-execution/03`)
   makes `FIRST-PROOF-CERT` (gate 5) the single next admissible spec, and explicitly states gate 5 is
   "Blocked until gates 1 through 4 pass" (now unblocked). The ladder invalidates any
   Phase-4/expansion/feature spec attempted before FIRST-PROOF-CERT passes. **Open the deliverable with
   a short, evidence-based confirmation of this determination** (cite the gate ladder + ledger + the
   four passed predecessor gates). Do **not** survey alternative "next features", and do **not** propose
   gameplay expansion.

2. **Composition posture: consume the four passed gates as certified building blocks, and require ONE
   integrated coherence re-proof at a single unified commit.** FIRST-PROOF-CERT's identity is "the first
   missing-property playable proof as a *coherent gate set*". The four prior gates passed at *divergent*
   commits (`2a37b04`-line / `92ba47f` / `726b2a1` / `c819bbe`); the first proof's "definition of done"
   (`docs/2-execution/02`) requires `P0-CERT` plus **all** first-proof gates to pass **in one coherent
   artifact set**. Therefore the spec must: (a) **cite** the prior `*-CERT passed` artifacts as
   certified building blocks and **not re-audit** the settled SPINE/EPI/ORD-LIFE seams; **and** (b)
   require the implementing session to demonstrate, at **one unified baseline commit**, that the nine
   first-proof acceptance gates hold **together** (integrated CI-equivalent review-artifact set, replay
   determinism across the combined corpus, no cross-gate regression). The audit points concentrate on
   the *integration coherence* + the *genuinely-new surface* (intentions 3–4), not on re-proving each
   building block in isolation. Be explicit that consuming a prior `*-CERT passed` artifact is **not**
   a claim of latest-main certification; the unified-commit integrated proof is what FIRST-PROOF-CERT
   newly establishes.

3. **The MISSING-PROPERTY gate is the genuinely-new certification surface — give it first-class audit
   points.** No prior gate certified the missing-property/expectation-contradiction *first-proof
   scenario as a coherent gate*: the holder believes an item should be somewhere, observes absence,
   forms an **actor-known contradiction**, and may search/ask/report/misremember/suspect-wrongly/do
   nothing — **with no culprit flag, suspect, clue, quest, or global theft truth supplied by the
   world** (`docs/0-foundation/12` Gate `MISSING-PROPERTY`; `docs/2-execution/02`). Decompose this into
   numbered audit points covering: source-backed expectation provenance; absence discovered by
   observation, not by truth; contradiction formed and event-sourced; the absence of any culprit/suspect
   truth-write; and the boundary against story-sifting / leads / investigation machinery
   (`architecture 11`, `execution 11` — explicitly **out**). This is the surface that most distinguishes
   FIRST-PROOF-CERT from the prior subsystem gates.

4. **The consolidated temporal evidence bundle is a first-class audit surface here.** ORD-LIFE-CERT
   (0042/0043) audited the routine-temporal *mechanism* but explicitly **routed the consolidated
   temporal evidence bundle forward to FIRST-PROOF-CERT** as a pass/fail obligation. Per
   `docs/2-execution/03` §"Temporal Cascade" and `docs/2-execution/02`, first-playable acceptance must
   include: temporal-firewall evidence from `04`, routine temporal proof from `06`, embodied temporal
   rendering from `07`, temporal fixture families from `09`, and temporal diagnostics from `10`.
   Decompose the bundle into dedicated audit points that prove it **coheres and passes at the unified
   commit** (each routed source → its evidence obligation → the integrated acceptance line). Do not
   treat it as mere collection; FIRST-PROOF-CERT is where the bundle becomes provable. (`INV-112`
   temporal authority governs; mint no temporal vocabulary/threshold/units — those remain lower-tier.)

5. **Cover all nine first-proof gates and all nine scenario families as one coherent gate set.** The
   audit points must, between them, cover every gate in `docs/2-execution/02`'s acceptance contract
   (`EVENT`, `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `POSSESSION-PARITY`, `NO-HUMAN-ORDINARY-LIFE`,
   `MISSING-PROPERTY`, `VIEW-DEBUG-SPLIT`, `REPLAY`, `FIXTURE-NEGATIVE`) and every scenario family in
   that doc's table (physical custody baseline, expectation contradiction, possession parity, epistemic
   filtering, no-hidden-truth planning, no-human ordinary day, routine blocking, replay rebuild, content
   rejection). For gates whose subsystem is already certified (EVENT/REPLAY ← SPINE; ACTOR-KNOWN /
   POSSESSION-PARITY / VIEW-DEBUG-SPLIT ← EPI; NO-HUMAN-ORDINARY-LIFE ← ORD-LIFE), the audit point
   **consumes** the building block and certifies its **participation in the coherent integrated set**
   (it appears in the unified run, holds under the combined corpus, and does not regress), rather than
   re-auditing its internals. Provide **a coverage table** mapping each audit point to the nine gates
   and the nine scenario families (so completeness is auditable).

6. **Form: a non-executable audit / certification spec** in the exact 0036/0038/0040/0042 lineage. The
   spec specifies *what the implementing session must run, prove, record, and package* — it does **not**
   render a pass/fail verdict, and does **not** assert the current code passes. You (Session 2) cannot
   run `cargo test`, `cargo-mutants`, or replay, so do not fabricate results. For each seam the spec must
   define: exact files/seams to audit (real, verified at `1541da2`); foundation + architecture
   dependencies; required positive **and** adversarial fixtures; event/replay/projection/determinism
   evidence; actor-known-provenance and debug-quarantine evidence where the seam touches them; the
   mutation-testing posture; and failure-diagnostics named by responsible layer. You **may** include a
   single clearly-labeled **"preliminary, non-certifying static survey"** of what reading the code at
   `1541da2` suggests about likely gate satisfaction/risk — explicitly marked informative, not
   certification — or omit it. Authoritative pass/fail belongs to the implementing session.

7. **Scope is the FIRST-PROOF-CERT *audit* spec only.** Produce the audit spec (the 0036/0038/0040/0042
   analogue), not a remediation spec. If the implementing session's mutation run later leaves survivors,
   that triggers a **separate** FIRST-PROOF-CERT mutation-remediation + replacement spec (the
   0037/0039/0041/0043 analogue) — **do not pre-author it here**, and do not fold an anticipatory
   remediation plan into this spec. The acceptance-artifact contract simply names what would render
   `FIRST-PROOF-CERT passed` and what a failed-floor verdict (`FIRST-PROOF-CERT scoped remediation`)
   would route to.

8. **EMERGE-OBS stays observer-only — and FIRST-PROOF-CERT is where the first-proof living-world corpus
   is exercised.** The no-human / missing-property ordinary day *is* the first-proof living-world
   acceptance corpus (`docs/0-foundation/12`, `INV-111`). The spec may name the observer-only
   `EMERGE-OBS` artifact as a required evidence-package member of the FIRST-PROOF-CERT package, but it
   **never** becomes a phase gate or pass/fail threshold (per `docs/2-execution/10` and `03`). Treat it
   as an evidence-package member, not an audit point with a pass condition.

9. **Numbering & placement.** Filename: `specs/0044_FIRST_PROOF_CERT_<DESCRIPTIVE_NAME>.md`. The staging
   series in `archive/specs/` is contiguous `0002 → 0043` with `specs/` currently empty, so the next
   staging number is **0044** (recent git history shows each spec archived by a plain rename with no
   renumber/restart; the ORD-LIFE-CERT lineage predecessor chose `0042`/`0043`, so this continues the
   sequence; the live ledger's `0001`/`0003` numbering does not block continuing the staging sequence).
   Stage under `specs/`; it is archived to `archive/specs/` on acceptance. Header block must mirror 0042:
   staging path, target repository, **target commit `1541da274180ecd40f52583d86704990cb55e74c`**, spec
   series, status, **Work posture: `Certification`**, and **admissibility posture** citing the 0037
   (`P0-CERT passed`), 0039 (`SPINE-CERT passed`), 0041 (`EPI-CERT passed`), and 0043 (`ORD-LIFE-CERT
   passed`) acceptance artifacts, naming the gates this spec consumes.
   `assumption:` stage under `specs/` (not directly in `docs/4-specs/`), per the repo's
   stage-then-archive convention — the user may relocate if desired.

10. **Source discipline (carry verbatim from the ledger).** Manifests are path inventory only;
    branch/default-branch/code-search/repo-metadata are not proof of target-commit content; archived
    specs/tickets/reports are cited as **history only**, never as live certification; the spec may
    **not** amend invariants, redefine/weaken gate semantics, or **mint a new gate code** —
    `FIRST-PROOF-CERT` is a phase-certification *label* that composes the canonical gates/review-artifacts
    from architecture `00` and the execution index `00`, exactly as `docs/2-execution/03` and
    `docs/2-execution/02` state. Use `actor-known` for the actor case and `holder-known context` as the
    system-wide term; use `missing property` and `expectation contradiction` exactly per the glossary.

11. **`assumption:`** the spec may include the optional preliminary static survey (intention 6); if
    included it is explicitly labeled "preliminary, not certification". Default is to permit it; omit if
    it would risk reading as a verdict.

---

## 4. The task

Author the **`FIRST-PROOF-CERT` certification spec** (`specs/0044_*`): the next admissible spec on the
Tracewake certification ladder (gate 5, the capstone). It is a *new-spec*, *non-executable
certification* deliverable — an audit plan plus acceptance contract that the implementing session will
execute to certify, **at one unified baseline commit**, that the first missing-property playable proof
holds as **one coherent gate set**: the nine first-proof acceptance gates and nine scenario families
(`docs/2-execution/02`), the genuinely-new **MISSING-PROPERTY** surface, and the consolidated
**temporal evidence bundle** routed forward from ORD-LIFE-CERT — composing the already-passed
`P0-CERT` / `SPINE-CERT` / `EPI-CERT` / `ORD-LIFE-CERT` gates as certified building blocks without
re-auditing them. The spec must be evidence-complete (every seam → audited files, required positive +
adversarial fixtures, proof mechanics, failure-diagnostic layers, acceptance-artifact section) so a
Tracewake implementing session can execute it without further design.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular the actual
code seams the first-proof certification will audit and the fixtures it composes. Verified-present at
`1541da2` and load-bearing (name the real modules in the audit inventory; confirm each at `1541da2`
and add any you find). Core paths are under `crates/tracewake-core/src/`, content/fixtures under
`crates/tracewake-content/src/`, TUI under `crates/tracewake-tui/src/`, tests under each crate's
`tests/`:

- **Missing-property / expectation / belief substrate** (the new surface): `epistemics/belief.rs`,
  `epistemics/contradiction.rs`, `epistemics/observation.rs`, `epistemics/proposition.rs`,
  `epistemics/knowledge_basis.rs`, `epistemics/knowledge_context.rs`, `epistemics/projection.rs`,
  `agent/perception.rs` (place perception → observation recording). The live fixture
  `crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs` is the canonical
  missing-property scenario; `docs/4-specs/0001` is its ontology contract.
- **Actor-known transaction / truth firewall**: `agent/actor_known.rs`, `agent/transaction.rs`,
  `agent/decision.rs`, `agent/no_human_surface.rs`, and the hidden-truth/provenance guards exercised
  by `crates/tracewake-core/tests/hidden_truth_gates.rs`.
- **Possession parity / embodied view models / debug split**: the `crates/tracewake-tui/src/` seams
  (`render.rs`, `debug_panels.rs`, `app.rs`, `input.rs`, `run.rs`, `transcript.rs`) and the
  embodied-view fixtures (`embodied_view_omits_*`, `embodied_menu_lags_truth_change_*`,
  `debug_omniscience_excluded_001`, `debug_attach_001`).
- **Event / replay / projection**: `events/envelope.rs`, `events/log.rs`, `events/apply.rs`,
  `events/mutation.rs`, `replay/rebuild.rs`, `replay/report.rs`, `checksum.rs`, `projections.rs`.
- **No-human ordinary life / scheduler boundary** (consumed via ORD-LIFE-CERT): `scheduler.rs`,
  `agent/need*.rs`, `need_accounting.rs`, `agent/routine.rs`, `agent/planner.rs`, `agent/trace.rs`,
  `actions/pipeline.rs`, and the no-human fixtures (`no_human_day_001`, `no_human_advance_001`,
  `food_unavailable_replan_001`, etc.).
- **Test / fixture harnesses**: `crates/tracewake-core/tests/` — `acceptance_gates.rs`,
  `acceptance_artifact_wording.rs`, `golden_scenarios.rs`, `hidden_truth_gates.rs`,
  `event_schema_replay_gates.rs`, `negative_fixture_runner.rs`, `no_human_capstone.rs`,
  `emergence_ledger.rs`, `generative_lock.rs`, `spine_conformance.rs`, `support/generative.rs`,
  `support/mod.rs`; plus `crates/tracewake-content/tests/golden_fixtures_run.rs` and the full
  `crates/tracewake-content/src/fixtures/` corpus (positive + adversarial families).

Verify these exist at `1541da2` and name them precisely; the names above are orientation from a
baseline read, not authority — confirm against the tree.

Research online as deeply as needed — similar implementations and prior art — wherever it sharpens the
audit design: **belief/expectation discrepancy and "missing object" reasoning** in believable-agent and
interactive-fiction systems (how to prove discovery is actor-relative, not authored-by-fiat);
**partial-observability / information-set discipline** (acting and forming contradictions from
actor-known belief, not ground truth — POMDP-adjacent framing); **closed-loop "the world runs without a
human" liveness/acceptance proofs** (distinguishing legitimate modeled waits from silent stalls across
an *integrated* corpus); **event-sourcing / deterministic replay acceptance** for composite scenario
sets (cross-fixture determinism, divergence localization); **integration / coherence testing of
independently-verified subsystems** (proving the composed whole holds at one baseline without
re-verifying parts — contract/consumer-driven and "big-bang vs incremental integration" literature);
**property-based and metamorphic testing** for the combined corpus and replay; **mutation testing**
strategy for a cross-cutting integration gate (scoping guarded-layer coverage across multiple
subsystems, triaging survivors); and **anti-Goodhart / observer-only emergence-evidence** design (how a
living-world "emergence" artifact stays descriptive and never becomes a steered pass/fail target). Cite
sources for any external claim that shapes a decision. The *deep* research is yours to perform; the
determination of *which* spec is not — that is settled in §3.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every property the spec
  certifies must satisfy it (notably `INV-111` observer-only emergence and `INV-112` temporal
  authority); a genuine divergence would require amending an invariant first — never design against it.
  The spec does not amend invariants.
- Authority order: if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong.
- No backwards-compatibility shims or alias paths in new work.
- Anti-contamination (first-proof specifics): no simulation fact may be born from prose; the missing
  property is discovered through actor-relative expectation + observation, **never** a global theft
  truth or culprit flag; truth may validate (reachability, presence, access, invariants) but must not
  choose goals, methods, hidden food targets, suspects, witnesses, clues, or view-model affordances;
  no actor teleports to true food/property; the scheduler may not dispatch ordinary actions directly or
  author state deltas; possession binds input only and grants no privileged body/memory/actions/truth;
  embodied view models expose only holder-known context, debug surfaces stay non-diegetic and never feed
  cognition; replay rebuilds projections + diagnostics deterministically and localizes divergence.
- No protagonist gravity / human special-case: no accepted behavior may require knowing which actor, if
  any, is human-controlled.
- The crate dependency direction (`core ← content ← tui`) must never invert.
- Mint no new gate code, status enum, obligation code, invariant ID, or temporal vocabulary.
  Cross-reference existing ones. FIRST-PROOF-CERT is a composing label.
- Explicit non-goals (out of scope, route forward, do not audit): LLM dialogue, full institutional
  investigation / wrong-suspicion / records (PHASE-4-ENTRY), regional travel, story sifting, quest
  boards, notices, long-history generation, LOD expansion (SECOND-PROOF-ENTRY).

---

## 7. Deliverable specification

Produce **one** downloadable markdown document:

- **`specs/0044_FIRST_PROOF_CERT_<DESCRIPTIVE_NAME>.md`** — **new** numbered staging spec (not a
  replacement). `<DESCRIPTIVE_NAME>` is yours to choose in the established ALL-CAPS-underscored style
  (e.g. `FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC`).

This is a *new-spec* deliverable: there is **no** paired `-research-report.md`. The numbering/ledger/
staging-series rules in §3.9 (staging series → `0044`) apply; do not treat this as a `reports/`
analysis report.

Required structure (follow 0036/0038/0040/0042 anatomy, retargeted to the integrated first-proof
substrate):

1. **Header block** — staging path, target repo, **target commit
   `1541da274180ecd40f52583d86704990cb55e74c`**, spec series (numbered staging spec, archived on
   acceptance), status, `Work posture: Certification`, admissibility posture citing `P0-CERT passed`
   (0037 artifact), `SPINE-CERT passed` (0039 artifact), `EPI-CERT passed` (0041 artifact), and
   `ORD-LIFE-CERT passed` (0043 artifact), and a one-line statement that the doc is **non-executable**.
2. **Determination confirmation** — short, cited: why FIRST-PROOF-CERT is the next admissible spec (gate
   ladder + ledger; predecessor gates 1–4 passed; gate-5 block now lifted).
3. **Source discipline, freshness, admissibility** — per §3.10.
4. **Authority & dependency declarations** — controlling doctrine order; primary first-proof sources
   (foundation `12`, execution `02`/`03`); the higher-tier foundation/architecture/execution
   dependencies (the §2 [primary] list).
5. **Composition & consumption statement** — explicitly: which gates are **consumed** as certified
   building blocks (cite the four `*-CERT passed` artifacts and their commits) and **not re-audited**;
   and that FIRST-PROOF-CERT newly establishes the **unified-commit integrated coherence proof** (per
   §3.2). State clearly that consuming a prior pass is not a latest-main certification claim.
6. **The FIRST-PROOF-CERT audit contract** — all first-proof seams as numbered audit points
   (`FIRST-PROOF-01…`), each with: seam definition, audited files/modules (real, verified at
   `1541da2`), the doctrine it must satisfy, required positive + adversarial fixtures (incl.
   expectation-contradiction / no-culprit-truth / no-hidden-truth-planning / no-teleport /
   possession-does-not-reset / no-human-special-case / debug-no-leakage / no-marker-only-life /
   no-replay-drift / content-rejection), event/replay/projection/determinism evidence,
   actor-known-provenance and debug-quarantine evidence where applicable, and the exact commands the
   implementing session runs. Concentrate audit points on the **integration coherence**, the **new
   MISSING-PROPERTY surface** (§3.3), and the **temporal bundle** (§3.4); for already-certified
   subsystems, the audit point certifies *participation in the coherent set*, not internals (§3.5).
   **A coverage table** mapping each audit point to the **nine first-proof acceptance gates** and the
   **nine scenario families** in `docs/2-execution/02` (so completeness is auditable), plus a row set
   showing the temporal bundle's five routed sources (`04`/`06`/`07`/`09`/`10`).
7. **Mutation-testing posture** — configured guarded-layer coverage over the FIRST-PROOF seams
   (emphasizing the new missing-property/expectation surface and the integration harness, not
   re-mutating already-certified subsystem internals), survivor-triage-register obligation (reuse the
   triage-register format), phase-entry evidence rule.
8. **Failure handling** — diagnostics named by responsible layer (the `docs/2-execution/03` layer list);
   a failed seam produces a named remediation spec/report (the future FIRST-PROOF-CERT
   mutation-remediation spec), not a relabeled phase exception.
9. **Acceptance-artifact contract** — instantiate `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:
   what the implementing session records to render `FIRST-PROOF-CERT passed`, including evidence-status,
   fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical,
   certification-use, and staged-abstraction fields. Name the observer-only `EMERGE-OBS` artifact as a
   required evidence-package member (never a gate). State that this spec specifies the audit; it does
   not render the verdict; a failed mutation floor yields `FIRST-PROOF-CERT scoped remediation` routed
   to a separate remediation spec.
10. **(Optional) Preliminary static survey** — clearly labeled "preliminary, not certification".
11. **Tolerated deferrals** — explicitly out of FIRST-PROOF-CERT scope and routed forward: institutions
    / wrong-suspicion / records / full investigation to `PHASE-4-ENTRY` (`execution 11`); notices /
    travel / regional scale / LOD / story-sifting to `SECOND-PROOF-ENTRY` (`execution 12`); LLM dialogue
    (locked); latest-main certification; and a statement that the four prior gates' internals are
    already certified and are **not** re-audited here (only their participation in the coherent set is).

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec opens with a cited confirmation that FIRST-PROOF-CERT is the next admissible move
      (predecessor gates `P0-CERT`, `SPINE-CERT`, `EPI-CERT`, and `ORD-LIFE-CERT` passed; gate-5 block
      lifted); it does not propose feature/expansion work.
- [ ] The composition posture is explicit: the four prior gates are **consumed** (cited, not
      re-audited) and FIRST-PROOF-CERT newly establishes a **unified-commit integrated coherence
      proof**; consuming a prior pass is stated to be **not** a latest-main certification claim.
- [ ] The **MISSING-PROPERTY** surface is covered as first-class numbered audit points (source-backed
      expectation, absence by observation not truth, event-sourced contradiction, **no** culprit/suspect
      truth, boundary against story-sifting/investigation).
- [ ] The consolidated **temporal evidence bundle** (`04`/`06`/`07`/`09`/`10`) is covered as dedicated
      audit points proving it coheres and passes at the unified commit — not mere collection.
- [ ] A coverage table maps the audit points onto **all nine first-proof acceptance gates** and **all
      nine scenario families** in `docs/2-execution/02`, plus the temporal bundle's five routed sources;
      none is missing.
- [ ] The deliverable is non-executable: it specifies the audit and acceptance contract, renders **no**
      pass/fail verdict, and fabricates no test/mutation/replay results. Any static survey is labeled
      "preliminary, not certification".
- [ ] Scope is the FIRST-PROOF-CERT **audit** spec only; no remediation spec is authored or pre-planned
      beyond naming the route a failed floor would take.
- [ ] Filename is `specs/0044_FIRST_PROOF_CERT_*.md`; header carries posture `Certification` and
      admissibility citing the 0037 (`P0-CERT`), 0039 (`SPINE-CERT`), 0041 (`EPI-CERT`), and 0043
      (`ORD-LIFE-CERT`) artifacts; target commit is `1541da2`.
- [ ] No new gate code / invariant ID / status enum / temporal vocabulary is minted; FIRST-PROOF-CERT is
      treated as a label that composes existing canonical gates; archived specs (incl. Phase-1/2A/3A
      `0002`–`0035`) are cited as history only; `EMERGE-OBS` stays observer-only and never a gate.
- [ ] No doctrine weakens an upstream tier; crate dependency direction is preserved; anti-contamination
      and no-protagonist-gravity properties hold (no culprit truth, no teleport, no-direct-dispatch,
      actor-known-inputs-only, possession-does-not-reset, non-diegetic debug, no human special-case).
- [ ] The acceptance-artifact contract instantiates `docs/4-specs/0003`'s fields.
- [ ] Every external claim is cited; every file named in §2 and in the audit inventory exists at commit
      `1541da274180ecd40f52583d86704990cb55e74c`.
</content>
</invoke>
