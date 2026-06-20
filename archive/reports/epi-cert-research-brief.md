# EPI-CERT certification spec — deep-research brief (paste into ChatGPT-Pro)

> **You are Session 2.** Produce the deliverable directly as a downloadable markdown
> document. Do **not** interview, do **not** ask clarifying questions — the requirements
> below are final. If a genuine contradiction makes a requirement impossible, state it in
> the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-19_ba9fe1c.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust (event-sourced
kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers
govern later ones. **Fetch every file from commit `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`
(short `ba9fe1c`)** — the manifest reflects exactly that tree. Do not adopt a different "commit of
record" from any file you read: commit hashes that appear *inside* archived specs/acceptance
artifacts (e.g. `92ba47f`, `b03ceed`, `2a37b04`) are that artifact's own audit provenance, not this
baseline. A manifest is path inventory only.

**This brief continues an active certification campaign.** The repo runs a phase-certification
ladder (`docs/2-execution/03`): `P0-DOC → P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT →
FIRST-PROOF-CERT → PHASE-4-ENTRY → SECOND-PROOF-ENTRY`. The immediately preceding work delivered the
**SPINE-CERT pair**, which is your **lineage predecessor** (this brief is a *delta* on it, retargeted
to the epistemic substrate — do not re-commission it):

- **0038** (`archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md`)
  — the `SPINE-CERT` audit spec; verdict came back `SPINE-CERT scoped remediation` because Wave-B
  mutation expansion found a survivor floor.
- **0039** (`archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`)
  — the scoped mutation remediation + replacement certification; verdict **`SPINE-CERT passed`** for
  the scoped 0039 mutation-remediation line at exact implementation commit `92ba47f…`. Its artifact
  (`archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`)
  supersedes the 0038 artifact for `SPINE-CERT passed` citation.

`P0-CERT passed` (via the 0037 artifact) **and** `SPINE-CERT passed` (via the 0039 artifact) are
therefore the current admissibility state. Per the gate ladder and the ledger's "Next known execution
move", the **single next admissible spec is the `EPI-CERT` certification** (gate 3).

**The structural model you must mirror** (the cert-spec anatomy this campaign reuses, read in full —
these are *shape* precedents in §2, not delta seeds): the P0-CERT pair (0036 audit / 0037 remediation)
and the SPINE-CERT pair (0038 audit / 0039 remediation). Your spec is the **audit spec** of the next
pair — the epistemic-substrate analogue of 0036/0038. The eventual EPI-CERT mutation-remediation
replacement spec is **a later, separate spec** and is **out of scope here** (author it only if/when the
implementing session's mutation run leaves survivors — do not pre-author it).

---

## 2. Read in full (authority order)

Read these before producing. The user's standing instruction is to read **all** of
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and
`docs/4-specs/*`. Within each tier, the entries marked **[primary]** are load-bearing for the
EPI-CERT spec; the remainder are *boundary-awareness* (read to know what is **out** of EPI-CERT scope
and must not be audited or "corrected" here — they belong to SPINE-CERT (already passed) /
ORD-LIFE-CERT / FIRST-PROOF-CERT / later gates).

**Foundation (`docs/0-foundation/`)**
- `docs/README.md` — **[primary]** repository authority layering and the "execution is wrong if it
  conflicts with architecture/foundation" rule.
- `00_FOUNDATION_INDEX.md` — index/boundary map.
- `02_CONSTITUTIONAL_INVARIANTS.md` — **[primary]** `INV-001…INV-112`; every certification property
  must satisfy these. The epistemic gates are downstream of the subjective-epistemics /
  no-contamination / possession-parity invariants here.
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **[primary]** epistemic state (observations,
  beliefs, contradictions) must be event-sourced and replay-derived; provenance witnesses are
  event-backed. Read for the trace/replay basis the epistemic projection rebuilds from.
- `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — **[primary]** THE foundation contract for
  claims, beliefs, memory, observation channels, and subjective information flow EPI-CERT certifies.
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **[primary]** possession parity, embodied vs. debug
  view models, and debug non-diegesis — core EPI-CERT seams.
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **[primary]** acceptance-gate doctrine.
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **[primary]** the actor-known
  cognition transaction and truth firewall; the basis for "no fact born from prose / no decision from
  unobserved ground truth" epistemic properties.
- `01, 05, 06, 07, 09, 10, 11, 13` — boundary-awareness (charter, needs/routines/planning,
  actions/survival, institutions, no-scripting, LOD/scale, LLM/speech, research notes). Read to bound
  scope; not EPI-CERT audit targets.

**Architecture (`docs/1-architecture/`)**
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **[primary]** canonical gate/review-artifact
  composition; the EPI-CERT *label* composes gates defined here.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — **[primary]** the one-way crate
  dependency rule (core ← content ← tui) the epistemic seams must not invert (debug surfaces in tui
  must not become truth writers in core).
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **[primary]** the projection-rebuild
  contract: the epistemic projection is a replay-derived projection, never a truth writer.
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **[primary]** THE architecture
  contract for holder-known/actor-known sealed contexts, the truth firewall, and provenance
  sufficiency/freshness — the heart of EPI-CERT.
- `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — **[primary]** the
  actor-known decision transaction (sealed proposal, dangling-provenance discipline) the epistemic
  surface feeds.
- `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — **[primary]** THE
  architecture contract for beliefs, observation channels, memory traces, contradiction detection, and
  privacy-scoped information flow.
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — **[primary]** possession parity,
  embodied/debug view-model split, debug quarantine, and client boundaries — core EPI-CERT seams.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **[primary]** what a gate
  artifact must contain.
- `04, 07, 08, 09, 11, 12, 14` — boundary-awareness (action pipeline/scheduling — SPINE; speech/LLM;
  institutions; ordinary life/economy; incidents/leads; LOD/prehistory; forbidden misreads). Read to
  bound scope. (Note: `14`'s forbidden-misreads list is useful orientation for epistemic terminology
  discipline, but is not an audit target.)

**Execution (`docs/2-execution/`)**
- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — **[primary]** execution-tier authority and index.
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — **[primary]** the
  post-0008 baseline, the code-audit boundary, and the three admissibility postures
  (`passed` / `scoped remediation` / `not applicable`).
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — **[primary]** first-proof scope
  and acceptance contract the epistemic certification feeds.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **[primary]** defines `EPI-CERT`
  (gate 3): "Certify actor-known/holder-known contexts, beliefs, observations, provenance, possession
  parity, view models, and debug quarantine." Also: gate-evidence requirements, gate-failure handling
  (named layers), valid future-spec postures, EMERGE-OBS observer-only rule, and the "Phase 2A landed
  = evidence for EPI-CERT, not certification" historical mapping.
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **[primary]** THE anti-
  contamination/truth-firewall execution gates the epistemic boundary must hold (no fact born from
  prose; no decision from unobserved ground truth; validation truth may not propose).
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **[primary]** THE EPI-CERT execution
  proof doc: epistemic view-model proof, possession parity, and debug-quarantine proof obligations.
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — **[primary]** golden-fixture and replay
  acceptance obligations; positive + adversarial fixture families (incl. hidden-truth/contradiction
  fixtures).
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **[primary]** evidence-honesty
  rule, diagnostics-by-layer, review-artifact obligations, EMERGE-OBS observer-only discipline.
- `05, 06, 08, 11, 12, 13` — boundary-awareness (scheduler/no-direct-dispatch — SPINE; ordinary-life
  needs/routines — ORD-LIFE; data authoring/schema; institutions/Phase-4; deferred second-proof;
  research notes). Read to bound scope; not EPI-CERT targets. (`08` data-authoring touches seed-
  knowledge provenance — relevant only where the epistemic seam consumes authored seed knowledge;
  do not audit the authoring/schema pipeline itself, which is SPINE/ORD-LIFE territory.)

**Reference (`docs/3-reference/`)**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **[primary]** review checklist questions.
- `01_DESIGN_RISK_REGISTER.md` — **[primary]** standing risks (epistemic-relevant: contamination,
  hidden-truth leakage, possession-parity drift, Goodharting, freshness/staleness).
- `02_GLOSSARY.md` — **[primary]** canonical terminology (`holder-known context`, `actor-known`,
  `EMERGE-OBS`, `observation channel`, `contradiction`, etc.) the spec must use exactly.

**Specs (`docs/4-specs/`)**
- `SPEC_LEDGER.md` — **[primary]** the live ledger: authority posture, source discipline, archived-
  spec status, "Next known execution move" (which records `SPINE-CERT passed` and does **not** yet
  declare EPI-CERT), and the numbering/admissibility rules.
- `README.md` — **[primary]** rules for future specs (declare authority posture; declare one
  admissibility posture; gate codes as cross-references only; terminology; no files for symmetry).
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — **[primary]** the review-artifact template (incl.
  evidence-status / fingerprint-scope / behavior-witness / replay-provenance / sampling /
  pending-historical / certification-use / staged-abstraction fields) the EPI-CERT acceptance
  artifact must instantiate.
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — boundary-awareness (live
  first-proof ontology; informs fixture realism, not an EPI-CERT target).

**Structural precedent — the cert-spec pattern you must mirror (read in full)**
- `archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md`
  — **[primary]** the freshest, most-similar **audit-spec template**: single-gate audit contract
  decomposed into numbered audit points (`SPINE-01…`), per-seam evidence obligations, fixture
  families, failure-diagnostic-by-layer, mutation posture, and acceptance-artifact section shape.
  Your spec is the epistemic-seam analogue of this.
- `archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md` — **[primary]** the
  original audit-spec template (ten-point audit contract); useful for cross-checking the established
  anatomy.
- `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` and
  `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` —
  **[primary]** the non-executable spec structure, source-discipline section, mutation posture
  (cargo-mutants, guarded-layer coverage, survivor triage register), and "what the implementing
  session must run/prove/record/package" framing. (These are remediation specs — read them for
  *structure and mutation discipline only*; the EPI-CERT remediation spec itself is a later, separate
  deliverable, not this one.)
- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md`
  — **[primary]** a *scoped-remediation (failed-floor)* acceptance artifact (shows how a survivor
  floor is surfaced and the verdict scoped).
- `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` —
  **[primary]** a *passing* acceptance artifact (the shape the eventual EPI-CERT artifact targets).
- `reports/0038_spine_cert_mutation_triage_register.md` and
  `reports/0039_spine_cert_mutation_triage_register.md` — **[primary]** the survivor triage-register
  format the epistemic mutation posture should reuse.

**Historical evidence — read as history only (never as live certification)**
- `archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md`
  — the historical Phase 2A epistemic substrate / expectation-contradiction / possession-parity
  implementation. Per `docs/2-execution/03`, "Phase 2A landed" is *evidence for* EPI-CERT, **not**
  certification.
- `archive/specs/0013_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`
  — the historical Phase 2A epistemic alignment + anti-contamination hardening (contributes scoped
  evidence toward `EPI-CERT`/`P0-CERT` per the ledger, but is not certification). Read for what was
  hardened historically; the audit must re-prove it under live doctrine, not presume it.

(Also available for orientation: the campaign's prior `reports/*-research-brief.md`, including
`reports/spine-cert-research-brief.md` — the brief that commissioned your lineage predecessor. Not
load-bearing — the spec doctrine above governs.)

---

## 3. Settled intentions (final — do not reopen)

1. **The determination is pre-settled and doctrine-forced.** `P0-CERT passed` (0037 artifact) **and**
   `SPINE-CERT passed` (0039 artifact) are the current admissibility state; the gate ladder
   (`docs/2-execution/03`) and the ledger's "Next known execution move" make the **`EPI-CERT`
   certification** (gate 3) the single next admissible spec. The ladder explicitly invalidates any
   Phase-4/expansion/feature spec attempted before `EPI-CERT` (and the gates after it) pass. **Open
   the deliverable with a short, evidence-based confirmation of this determination** (cite the gate
   ladder + ledger). Do **not** survey alternative "next features", and do **not** propose gameplay
   expansion.

2. **Breadth: one comprehensive spec covering all EPI-CERT seams.** EPI-CERT, as named in
   `docs/2-execution/03`, certifies: actor-known/holder-known contexts, beliefs, observations,
   provenance, possession parity, view models, and debug quarantine. Cover **all** of these as a
   single gate, the way 0038 covered the eight spine seams (and 0036 covered P0-01…P0-10) in one audit
   spec. Decompose into explicitly numbered audit points (e.g. `EPI-01 … EPI-NN`) with their own
   evidence obligations. The exact seam decomposition is **yours to design** from the live doctrine and
   the real code seams (§5); a reasonable starting cut is: (i) holder-known/actor-known sealed-context
   construction & hash parity; (ii) beliefs/stances/confidence storage & privacy scoping;
   (iii) observation channels & capture (sight/sound/touch/absence); (iv) contradiction / expectation-
   mismatch detection; (v) provenance & witness sufficiency (event-backed `SourceEventIds`, dangling-
   provenance discipline, hidden-truth audit); (vi) epistemic projection as replay-derived non-truth-
   writer; (vii) possession parity (human-possessed vs. autonomous; embodied truth snapshot);
   (viii) embodied vs. debug view-model split; (ix) debug quarantine (capability token, non-diegetic
   surfaces). Merge/split as the doctrine and code warrant — but cover every gate-3 theme.

3. **Form: a non-executable audit / certification spec** in the exact 0036/0038 lineage. The spec
   specifies *what the implementing session must run, prove, record, and package* — it does **not**
   render a pass/fail verdict, and does **not** assert the current code passes. You (Session 2) cannot
   run `cargo test`, `cargo-mutants`, or replay, so do not fabricate results. For each seam the spec
   must define: exact files/seams to audit; foundation + architecture dependencies; required positive
   **and** adversarial fixtures (incl. hidden-truth / contradiction / parity-violation fixtures);
   event/replay/projection/determinism evidence; provenance evidence where the seam touches it;
   debug-quarantine evidence; the mutation-testing posture (configured guarded-layer coverage +
   survivor triage register, per 0037/0039); and failure-diagnostics named by responsible layer (the
   layer list in `docs/2-execution/03` §"Gate failure handling"). You **may** include a single
   clearly-labeled **"preliminary, non-certifying static survey"** of what reading the code at
   `ba9fe1c` suggests about likely gate satisfaction/risk — explicitly marked as informative, not
   certification — or omit it. Authoritative pass/fail belongs to the implementing session.

4. **Scope is the EPI-CERT *audit* spec only.** Produce the audit spec (the 0036/0038 analogue), not a
   remediation spec. If the implementing session's mutation run later leaves survivors, that triggers a
   **separate** EPI-CERT mutation-remediation + replacement spec (the 0037/0039 analogue) — **do not
   pre-author it here**, and do not fold an anticipatory remediation plan into this spec. The spec's
   acceptance-artifact contract simply names what would render `EPI-CERT passed` and what a failed-floor
   verdict (`EPI-CERT scoped remediation`) would route to.

5. **Numbering & placement.** Filename: `specs/0040_EPI_CERT_<DESCRIPTIVE_NAME>.md`. The staging series
   in `archive/specs/` is contiguous `0002 → 0039` with `specs/` currently empty, so the next staging
   number is **0040** (no renumbering/restart in recent history changes this; the live ledger's
   `0001`/`0003` restart does not block continuing the staging sequence). Stage under `specs/`; it is
   archived to `archive/specs/` on acceptance. Header block must mirror 0038: staging path, target
   repository, **target commit `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`**, spec series, status,
   **Work posture: `Certification`**, and **admissibility posture** citing **both** the 0037 acceptance
   artifact (`P0-CERT passed`) **and** the 0039 acceptance artifact (`SPINE-CERT passed`), naming the
   gates this spec consumes.

6. **Source discipline (carry verbatim from the ledger).** Manifests are path inventory only;
   branch/default-branch/code-search/repo-metadata are not proof of target-commit content; archived
   specs/tickets are cited as **history only**, never as live certification; the spec may **not** amend
   invariants, redefine/weaken gate semantics, or **mint a new gate code** — `EPI-CERT` is a phase-
   certification *label* that composes the canonical gates/review-artifacts from architecture `00`,
   exactly as `docs/2-execution/03` states. Use `holder-known context` as the system-wide term and
   `actor-known` for the actor case.

7. **EMERGE-OBS stays observer-only.** If first-proof living-world acceptance is exercised by the
   verified corpus, the spec may name the observer-only `EMERGE-OBS` artifact as an evidence-package
   member, but it never becomes a phase gate or pass/fail threshold.

---

## 4. The task

Author the **`EPI-CERT` certification spec** (`specs/0040_*`): the next admissible spec on the
Tracewake certification ladder. It is a *new-spec*, *non-executable certification* deliverable — an
audit plan plus acceptance contract that the implementing session will execute to certify the
simulation's **epistemic substrate** (actor-known/holder-known contexts, beliefs, observations,
provenance, possession parity, embodied/debug view models, and debug quarantine) against live
foundation/architecture/execution doctrine at commit `ba9fe1c`. The spec must be evidence-complete
(every seam → audited files, required positive + adversarial fixtures, proof mechanics, failure-
diagnostic layers, acceptance-artifact section) so a Tracewake implementing session can execute it
without further design.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular the actual
code seams the epistemic certification will audit. Verified-present at `ba9fe1c` and load-bearing
(name the real modules in the audit inventory; confirm each at `ba9fe1c` and add any you find):

- **tracewake-core / `epistemics/`**: `knowledge_context.rs` (sealed holder-known context, `ViewMode`
  Embodied/Debug, allowed/forbidden knowledge sources, scope filters), `belief.rs` (Belief, HolderKind,
  Stance, Confidence, privacy scope), `observation.rs` (Observation, Channel, Confidence, TickWindow,
  `EPISTEMIC_RECORD_SCHEMA_V1`), `proposition.rs`, `projection.rs` (`EpistemicProjection` aggregate,
  `ActorKnownProjectionRecord`), `contradiction.rs` (expectation/observation mismatch).
- **tracewake-core / `agent/`**: `actor_known.rs` (`ActorKnownFact`, `SourceEventIds` witness,
  `ActorKnownProvenance`), `no_human_surface.rs` (`SealedActorKnownSurface`, typed-witness builder),
  `perception.rs` (multi-channel perception capture → projection), `transaction.rs` (`SealedProposal`,
  `ActorDecisionTransaction`, `dangling_provenance_diagnostic`), `trace.rs` (`HiddenTruthAudit`,
  `ResponsibleLayer::DebugQuarantine`).
- **tracewake-core**: `projections.rs` (`EmbodiedProjectionSource`, `EmbodiedTruthSnapshot`),
  `view_models.rs` (`EmbodiedViewModel`, `NotebookView`, `DebugViewModel`/`Debug*View`,
  `ActionAvailability` actor-safe vs. debug-only split, `ActionAvailabilityProvenanceKind`,
  `WhyNotView`), `controller.rs` (`build_embodied_view_model` from sealed context),
  `debug_capability.rs` (mint-only `DebugCapability` token), `debug_reports.rs` (debug report types
  sealed by the capability token), `actions/proposal.rs` + `actions/pipeline.rs` (holder-known
  context id/hash parity enforcement at proposal/validation).
- **tracewake-tui**: `debug_panels.rs` (panels assert `debug_only()`, `DEBUG NON-DIEGETIC` marker),
  `render.rs` (`DEBUG_TOKENS`, embodied-view filtering).

Verify these exist at `ba9fe1c` and name them precisely; the file sizes/contents above are orientation
from a baseline read, not authority — confirm against the tree.

Research online as deeply as needed — similar implementations and prior art — wherever it sharpens
the audit design: **subjective belief/knowledge modeling** in agent simulations (epistemic logic,
belief revision, theory-of-mind / partial-observability state), **information-flow / non-interference**
proof techniques (how to prove a hidden-truth channel cannot leak into an actor-visible surface),
**property-based and metamorphic testing** for belief/observation projections and possession-parity
invariants, **mutation testing** strategy for epistemic kernels (scoping guarded-layer coverage,
triaging survivors), provenance/witness-chain integrity, and embodied-vs-debug view-model separation
(capability-based isolation). Cite sources for any external claim that shapes a decision. The *deep*
research is yours to perform; the determination of *which* spec is not — that is settled in §3.

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
  The epistemic projection may not become a truth writer; no decision may rely on unobserved ground
  truth; debug surfaces stay non-diegetic and must never leak hidden truth into an embodied/actor-
  visible surface; possession parity holds (a human-possessed actor sees exactly what an autonomous
  actor in the same epistemic state would).
- The crate dependency direction (`core` ← `content` ← `tui`) must never invert.
- Mint no new gate code, status enum, obligation code, or invariant ID. Cross-reference existing ones.

---

## 7. Deliverable specification

Produce **one** downloadable markdown document:

- **`specs/0040_EPI_CERT_<DESCRIPTIVE_NAME>.md`** — **new** numbered staging spec (not a replacement).
  `<DESCRIPTIVE_NAME>` is yours to choose in the established ALL-CAPS-underscored style (e.g.
  `EPI_CERT_ACTOR_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC`).

This is a *new-spec* deliverable: there is **no** paired `-research-report.md`. The numbering/ledger/
epoch rules in §5 (staging series → `0040`) apply; do not treat this as a `reports/` analysis report.

Required structure (follow 0036/0038 anatomy, retargeted to the epistemic substrate):

1. **Header block** — staging path, target repo, **target commit
   `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`**, spec series (numbered staging spec, archived on
   acceptance), status, `Work posture: Certification`, admissibility posture citing **both**
   `P0-CERT passed` (the 0037 acceptance artifact) **and** `SPINE-CERT passed` (the 0039 acceptance
   artifact), and a one-line statement that the doc is **non-executable**.
2. **Determination confirmation** — short, cited: why EPI-CERT is the next admissible spec (gate
   ladder + ledger; predecessor gates passed).
3. **Source discipline, freshness, admissibility** — per §3.6.
4. **Authority & dependency declarations** — controlling doctrine order; primary epistemic sources;
   higher-tier foundation/architecture/execution dependencies (the §2 [primary] list).
5. **The EPI-CERT audit contract** — all epistemic seams as numbered audit points (`EPI-01…`), each
   with: seam definition, audited files/modules (real, verified at `ba9fe1c`), the doctrine it must
   satisfy, required positive + adversarial fixtures (incl. hidden-truth / contradiction / possession-
   parity-violation fixtures), event/replay/projection/determinism evidence, provenance and debug-
   quarantine evidence where applicable, and the exact commands the implementing session runs.
6. **Mutation-testing posture** — configured guarded-layer coverage over the epistemic seams,
   survivor-triage-register obligation (reuse the `reports/0039_spine_cert_mutation_triage_register.md`
   format), phase-entry evidence rule.
7. **Failure handling** — diagnostics named by responsible layer (the `docs/2-execution/03` layer
   list — incl. actor-known context construction, projection/replay, view-model rendering, debug
   quarantine); a failed seam produces a named remediation spec/report (the future EPI-CERT mutation-
   remediation spec), not a relabeled phase exception.
8. **Acceptance-artifact contract** — instantiate `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:
   what the implementing session records to render `EPI-CERT passed`, including evidence-status,
   fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical,
   certification-use, and staged-abstraction fields. State that this spec specifies the audit; it does
   not render the verdict; a failed mutation floor yields `EPI-CERT scoped remediation` routed to a
   separate remediation spec.
9. **(Optional) Preliminary static survey** — clearly labeled "preliminary, not certification".
10. **Tolerated deferrals** — explicitly out of EPI-CERT scope and routed to ORD-LIFE-CERT /
    FIRST-PROOF-CERT / Phase-4 (and confirm SPINE-CERT seams are already certified, not re-audited
    here). Do not audit them.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec opens with a cited confirmation that EPI-CERT is the next admissible move (predecessor
      gates `P0-CERT` and `SPINE-CERT` passed); it does not propose feature/expansion work.
- [ ] Every gate-3 theme — actor-known/holder-known contexts, beliefs, observations, provenance,
      possession parity, view models, debug quarantine — is covered as numbered audit points, each
      with audited files, positive + adversarial fixtures, determinism/replay evidence, and failure-
      diagnostic layers.
- [ ] The deliverable is non-executable: it specifies the audit and acceptance contract, renders **no**
      pass/fail verdict, and fabricates no test/mutation/replay results. Any static survey is labeled
      "preliminary, not certification".
- [ ] Scope is the EPI-CERT **audit** spec only; no remediation spec is authored or pre-planned beyond
      naming the route a failed floor would take.
- [ ] Filename is `specs/0040_EPI_CERT_*.md`; header carries posture `Certification` and admissibility
      citing **both** the 0037 (`P0-CERT passed`) and 0039 (`SPINE-CERT passed`) artifacts; target
      commit is `ba9fe1c`.
- [ ] No new gate code / invariant ID / status enum is minted; EPI-CERT is treated as a label that
      composes existing canonical gates; archived specs (incl. Phase 2A 0004/0013) are cited as history
      only.
- [ ] No doctrine weakens an upstream tier; crate dependency direction is preserved; anti-contamination
      and possession-parity properties hold; debug surfaces stay non-diegetic.
- [ ] The acceptance-artifact contract instantiates `docs/4-specs/0003`'s fields.
- [ ] Every external claim is cited; every file named in §2 and in the audit inventory exists at commit
      `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`.
