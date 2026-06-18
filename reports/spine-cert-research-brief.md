# SPINE-CERT certification spec — deep-research brief (paste into ChatGPT-Pro)

> **You are Session 2.** Produce the deliverable directly as a downloadable markdown
> document. Do **not** interview, do **not** ask clarifying questions — the requirements
> below are final. If a genuine contradiction makes a requirement impossible, state it in
> the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-16_b03ceed.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust (event-sourced
kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers
govern later ones. **Fetch every file from commit `b03ceedc5360d30894f297e40efcbddcc4fd0a7f`
(short `b03ceed`)** — the manifest reflects exactly that tree. Do not adopt a different "commit of
record" from any file you read: commit hashes that appear *inside* archived specs/acceptance
artifacts (e.g. `c54caff`, `9f16222`, `2a37b04`) are that artifact's own audit provenance, not this
baseline. A manifest is path inventory only.

**This brief continues an active certification campaign.** The repo runs a phase-certification
ladder (`docs/2-execution/03`): `P0-DOC → P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT →
FIRST-PROOF-CERT → PHASE-4-ENTRY → SECOND-PROOF-ENTRY`. The immediately preceding work delivered:

- **0036** (`archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md`) — the
  `P0-CERT` audit spec; verdict came back `P0-CERT scoped remediation` because a configured mutation
  run left survivor `0036-MUTATION-REMEDIATION-001`.
- **0037** (`archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`)
  — the scoped remediation + replacement certification; verdict **`P0-CERT passed`** for the
  post-0008 baseline mutation-remediation line at its exact implementation commit. This artifact
  (`archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`)
  supersedes the 0036 artifact for `P0-CERT passed` citation.

`P0-CERT passed` is therefore the current admissibility state. Per the gate ladder and the ledger's
"Next known execution move", the **single next admissible spec is the `SPINE-CERT` certification**
(gate 2). The 0036/0037 specs are the **direct structural precedent** for what you must produce — a
non-executable certification spec. Treat this brief as a *delta* on that pattern: reuse the proven
spec anatomy, retarget it from the P0 ten-point contract to the spine seams. Do **not** re-commission
the P0-CERT work — it is done.

---

## 2. Read in full (authority order)

Read these before producing. The user's standing instruction is to read **all** of
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and
`docs/4-specs/*`. Within each tier, the entries marked **[primary]** are load-bearing for the
SPINE-CERT spec; the remainder are *boundary-awareness* (read to know what is **out** of SPINE-CERT
scope and must not be audited or "corrected" here — they belong to EPI-CERT / ORD-LIFE-CERT / later
gates).

**Foundation (`docs/0-foundation/`)**
- `docs/README.md` — **[primary]** repository authority layering and the "execution is wrong if it
  conflicts with architecture/foundation" rule.
- `00_FOUNDATION_INDEX.md` — index/boundary map.
- `02_CONSTITUTIONAL_INVARIANTS.md` — **[primary]** `INV-001…INV-112`; every certification property
  must satisfy these. The spine gates are downstream of the event-sourcing / replay / no-contamination
  invariants here.
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **[primary]** the foundational event-trace and
  replay contract the spine implements.
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **[primary]** acceptance-gate doctrine.
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **[primary]** truth firewall; the
  basis for the "no direct dispatch / validation-truth may not propose" spine properties.
- `01, 04, 05, 06, 07, 08, 09, 10, 11, 13` — boundary-awareness (charter, beliefs/memory, needs/
  routines, actions/survival, institutions, TUI/possession, no-scripting, LOD/scale, LLM/speech,
  research notes). Read to bound scope; not SPINE-CERT audit targets.

**Architecture (`docs/1-architecture/`)**
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **[primary]** canonical gate/review-artifact
  composition; the SPINE-CERT *label* composes gates defined here.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — **[primary]** the one-way
  crate dependency rule (core ← content ← tui) the spine must not invert.
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **[primary]** THE spine subsystem
  contract: event envelope, stream boundaries, projection rebuild, save manifests, schema
  versioning/upcasting, random-stream discipline, deterministic-replay gates.
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **[primary]** provenance/firewall
  contract bearing on no-direct-dispatch and projection-as-non-truth-writer.
- `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — **[primary]** the action
  pipeline seam (proposal → validation → scheduling → event append → application → feedback).
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — **[primary]** TUI/debug split and
  client boundary the spine must keep non-diegetic.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **[primary]** what a gate
  artifact must contain.
- `05, 06, 07, 08, 09, 11, 12, 14` — boundary-awareness (actor decision/needs, beliefs/observation,
  speech/LLM, institutions, ordinary life/economy, incidents/leads, LOD/prehistory, forbidden
  misreads). Read to bound scope.

**Execution (`docs/2-execution/`)**
- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — **[primary]** execution-tier authority and index.
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — **[primary]** the
  post-0008 baseline, the `P0-CERT` predecessor gate, the code-audit boundary, and the three
  admissibility postures (`P0-CERT passed` / `scoped remediation` / `not applicable`).
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — **[primary]** first-proof
  scope and acceptance contract the spine certification feeds.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **[primary]** defines `SPINE-CERT`
  (gate 2): "Certify event log, replay, projection, randomness, save, action pipeline, TUI/debug
  split, and no direct dispatch." Also: gate-evidence requirements, gate-failure handling (named
  layers), valid future-spec postures, EMERGE-OBS observer-only rule.
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **[primary]** anti-contamination
  gates the spine boundary must hold (no fact born from prose; validation truth may not propose).
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — **[primary]** the
  no-direct-dispatch / scheduler / action-pipeline proof obligations — central SPINE-CERT seam.
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — **[primary]** golden-fixture and replay
  acceptance obligations; positive + adversarial fixture families.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **[primary]** evidence-honesty
  rule, diagnostics-by-layer, review-artifact obligations, EMERGE-OBS observer-only discipline.
- `06, 07, 08, 11, 12, 13` — boundary-awareness (ordinary-life, epistemic view models, data
  authoring/schema, institutions/Phase-4, deferred second-proof, research notes). Read to bound
  scope; not SPINE-CERT targets.

**Reference (`docs/3-reference/`)**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **[primary]** review checklist questions.
- `01_DESIGN_RISK_REGISTER.md` — **[primary]** standing risks (spine-relevant: replay, determinism,
  contamination, Goodharting).
- `02_GLOSSARY.md` — **[primary]** canonical terminology (`holder-known context`, `actor-known`,
  `EMERGE-OBS`, etc.) the spec must use exactly.

**Specs (`docs/4-specs/`)**
- `SPEC_LEDGER.md` — **[primary]** the live ledger: authority posture, source discipline, archived-
  spec status, "Next known execution move", and the numbering/admissibility rules.
- `README.md` — **[primary]** rules for future specs (declare authority posture; declare one
  admissibility posture; gate codes as cross-references only; terminology; no files for symmetry).
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — **[primary]** the review-artifact template (incl.
  evidence-status / fingerprint-scope / behavior-witness / replay-provenance / sampling /
  pending-historical / certification-use / staged-abstraction fields) the SPINE-CERT acceptance
  artifact must instantiate.
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — boundary-awareness (live
  first-proof ontology; informs fixture realism, not a SPINE-CERT target).

**Precedent — the cert-spec pattern you must mirror (read in full)**
- `archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md` — **[primary]** the
  audit-spec template: ten-point audit contract, per-gate evidence commands, fixture obligations,
  failure-diagnostic-by-layer, and the acceptance-artifact section shape. Your spec is the
  spine-seam analogue of this.
- `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` —
  **[primary]** the non-executable spec structure, source-discipline section, mutation posture
  (cargo-mutants, guarded-layer coverage, survivor triage register), and "what the implementing
  session must run/prove/record/package" framing.
- `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md` — **[primary]** a
  *failed-verdict* acceptance artifact (shows how a survivor is surfaced and the verdict scoped).
- `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` —
  **[primary]** a *passing* acceptance artifact (the shape the eventual SPINE-CERT artifact targets).
- `reports/0037_mutation_triage_register.md` — **[primary]** the survivor triage-register format the
  spine mutation posture should reuse.

(Also available for orientation: the campaign's prior `reports/*-research-brief.md` and
`reports/p0-cert-*-research-brief.md`. Not load-bearing — the spec doctrine above governs.)

---

## 3. Settled intentions (final — do not reopen)

1. **The determination is pre-settled and doctrine-forced.** `P0-CERT passed` (per the 0037
   artifact) is the current state; the gate ladder (`docs/2-execution/03`) and the ledger's "Next
   known execution move" make the **`SPINE-CERT` certification** the single next admissible spec.
   The ladder explicitly invalidates any Phase-4/expansion/feature spec attempted before
   `SPINE-CERT` (and the gates after it) pass. **Open the deliverable with a short, evidence-based
   confirmation of this determination** (cite the gate ladder + ledger), then proceed. Do **not**
   survey alternative "next features", and do **not** propose gameplay expansion.

2. **Breadth: one comprehensive spec covering all eight spine seams.** SPINE-CERT, as named in
   `docs/2-execution/03`, certifies: (i) event log, (ii) replay, (iii) projection rebuild,
   (iv) randomness / RNG-stream discipline, (v) save manifests, (vi) action proposal/validation/
   scheduling pipeline, (vii) TUI/debug split, (viii) no direct dispatch. Cover **all eight** as a
   single gate, the way 0036 covered P0-01…P0-10 in one audit spec. Decompose each seam into
   explicitly numbered audit points (e.g. `SPINE-01 … SPINE-NN`) with their own evidence
   obligations.

3. **Form: a non-executable audit / certification spec** in the exact 0036/0037 lineage. The spec
   specifies *what the implementing session must run, prove, record, and package* — it does **not**
   render a pass/fail verdict, and does **not** assert the current code passes. You (Session 2)
   cannot run `cargo test`, `cargo-mutants`, or replay, so do not fabricate results. For each seam
   the spec must define: exact files/seams to audit; foundation + architecture dependencies;
   required positive **and** adversarial fixtures; event/replay/projection/determinism evidence;
   provenance evidence where the seam touches it; debug-quarantine evidence; the mutation-testing
   posture (configured guarded-layer coverage + survivor triage register, per 0037); and
   failure-diagnostics named by responsible layer (the layer list in `docs/2-execution/03` §"Gate
   failure handling"). You **may** include a single clearly-labeled **"preliminary, non-certifying
   static survey"** of what reading the code at `b03ceed` suggests about likely gate
   satisfaction/risk — explicitly marked as informative, not certification — or omit it. Authoritative
   pass/fail belongs to the implementing session.

4. **Numbering & placement.** Filename: `specs/0038_SPINE_CERT_<DESCRIPTIVE_NAME>.md`. The staging
   series in `archive/specs/` is contiguous `0002 → 0037` with `specs/` currently empty, so the next
   staging number is **0038** (no renumbering/restart in recent history changes this). Stage under
   `specs/`; it is archived to `archive/specs/` on acceptance. Header block must mirror 0036/0037:
   staging path, target repository, **target commit `b03ceedc5360d30894f297e40efcbddcc4fd0a7f`**,
   spec series, status, **Work posture: `Certification`**, and **admissibility posture
   `P0-CERT passed`** citing the 0037 acceptance artifact and naming the gates this spec consumes.

5. **Source discipline (carry verbatim from the ledger).** Manifests are path inventory only;
   branch/default-branch/code-search/repo-metadata are not proof of target-commit content; archived
   specs/tickets are cited as **history only**, never as live certification; the spec may **not**
   amend invariants, redefine/weaken gate semantics, or **mint a new gate code** — `SPINE-CERT` is a
   phase-certification *label* that composes the canonical gates/review-artifacts from
   architecture `00`, exactly as `docs/2-execution/03` states. Use `holder-known context` as the
   system-wide term and `actor-known` for the actor case.

6. **EMERGE-OBS stays observer-only.** If first-proof living-world acceptance is exercised by the
   verified corpus, the spec may name the observer-only `EMERGE-OBS` artifact as an evidence-package
   member, but it never becomes a phase gate or pass/fail threshold.

---

## 4. The task

Author the **`SPINE-CERT` certification spec** (`specs/0038_*`): the next admissible spec on the
Tracewake certification ladder. It is a *new-spec*, *non-executable certification* deliverable — an
audit plan plus acceptance contract that the implementing session will execute to certify the
simulation **spine** (event log, replay, projection, randomness, save, action pipeline, TUI/debug
split, no direct dispatch) against live foundation/architecture/execution doctrine at commit
`b03ceed`. The spec must be evidence-complete (every seam → audited files, required fixtures, proof
mechanics, failure-diagnostic layers, acceptance-artifact section) so a Tracewake implementing
session can execute it without further design.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular the actual
`tracewake-core` code seams the spine certification will audit (event log/envelope, replay engine,
projection rebuild, RNG/random-stream plumbing, save/manifest, the action proposal→validation→
scheduler→apply pipeline, and the TUI/debug boundary in `tracewake-tui`). Name the real modules/files
in the spec's audit inventory; verify they exist at `b03ceed`.

Research online as deeply as needed — similar implementations and prior art — wherever it sharpens
the audit design: event-sourcing certification and invariants, **deterministic / reproducible replay**
testing, **property-based and metamorphic testing** for event logs and projections, **mutation
testing** strategy for simulation kernels (and how to scope guarded-layer coverage and triage
survivors), RNG-stream determinism discipline, and snapshot/save-manifest integrity. Cite sources for
any external claim that shapes a decision. The *deep* research is yours to perform; the determination
of *which* spec is not — that is settled in §3.

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
  Projections may not become truth writers; validation truth may accept/reject/mutate via events but
  may not propose fallback plans or actor-visible hidden facts; debug surfaces stay non-diegetic.
- The crate dependency direction (`core` ← `content` ← `tui`) must never invert.
- Mint no new gate code, status enum, obligation code, or invariant ID. Cross-reference existing ones.

---

## 7. Deliverable specification

Produce **one** downloadable markdown document:

- **`specs/0038_SPINE_CERT_<DESCRIPTIVE_NAME>.md`** — **new** numbered staging spec (not a
  replacement). `<DESCRIPTIVE_NAME>` is yours to choose in the established ALL-CAPS-underscored style
  (e.g. `SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC`).

Required structure (follow 0036/0037 anatomy, retargeted to the spine):

1. **Header block** — staging path, target repo, **target commit
   `b03ceedc5360d30894f297e40efcbddcc4fd0a7f`**, spec series (numbered staging spec, archived on
   acceptance), status, `Work posture: Certification`, `P0-CERT admissibility posture: P0-CERT passed`
   (cite the 0037 acceptance artifact), and a one-line statement that the doc is **non-executable**.
2. **Determination confirmation** — short, cited: why SPINE-CERT is the next admissible spec.
3. **Source discipline, freshness, admissibility** — per §3.5.
4. **Authority & dependency declarations** — controlling doctrine order; primary spine sources;
   higher-tier foundation/architecture/execution dependencies (the §2 [primary] list).
5. **The SPINE-CERT audit contract** — all eight seams as numbered audit points (`SPINE-01…`), each
   with: seam definition, audited files/modules (real, verified at `b03ceed`), the doctrine it must
   satisfy, required positive + adversarial fixtures, event/replay/projection/determinism evidence,
   provenance/debug-quarantine evidence where applicable, and the exact commands the implementing
   session runs.
6. **Mutation-testing posture** — configured guarded-layer coverage, survivor-triage-register
   obligation (reuse the `reports/0037_mutation_triage_register.md` format), phase-entry evidence rule.
7. **Failure handling** — diagnostics named by responsible layer (the `docs/2-execution/03` layer
   list); a failed seam produces a named remediation spec/report, not a relabeled phase exception.
8. **Acceptance-artifact contract** — instantiate `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:
   what the implementing session records to render `SPINE-CERT passed`, including evidence-status,
   fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical,
   certification-use, and staged-abstraction fields. State that this spec specifies the audit; it does
   not render the verdict.
9. **(Optional) Preliminary static survey** — clearly labeled "preliminary, not certification".
10. **Tolerated deferrals** — explicitly out of SPINE-CERT scope and routed to EPI-CERT /
    ORD-LIFE-CERT / FIRST-PROOF-CERT / Phase-4 (do not audit them here).

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec opens with a cited confirmation that SPINE-CERT is the next admissible move; it does
      not propose feature/expansion work.
- [ ] All **eight** spine seams are covered as numbered audit points, each with audited files,
      positive + adversarial fixtures, determinism/replay evidence, and failure-diagnostic layers.
- [ ] The deliverable is non-executable: it specifies the audit and acceptance contract, renders **no**
      pass/fail verdict, and fabricates no test/mutation/replay results. Any static survey is labeled
      "preliminary, not certification".
- [ ] Filename is `specs/0038_SPINE_CERT_*.md`; header carries posture `Certification` and
      admissibility `P0-CERT passed` citing the 0037 artifact; target commit is `b03ceed`.
- [ ] No new gate code / invariant ID / status enum is minted; SPINE-CERT is treated as a label that
      composes existing canonical gates; archived specs are cited as history only.
- [ ] No doctrine weakens an upstream tier; crate dependency direction is preserved; anti-contamination
      properties hold.
- [ ] The acceptance-artifact contract instantiates `docs/4-specs/0003`’s fields.
- [ ] Every external claim is cited; every file named in §2 and in the audit inventory exists at
      commit `b03ceedc5360d30894f297e40efcbddcc4fd0a7f`.
