# FIRST-PROOF-CERT mutation-remediation + replacement certification spec — deep-research brief (paste into ChatGPT-Pro)

> **You are Session 2.** Produce the deliverable directly as a downloadable markdown
> document. Do **not** interview, do **not** ask clarifying questions — the requirements
> below are final. If a genuine contradiction makes a requirement impossible, state it in
> the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-21_fd5ae94.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust (event-sourced
kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers
govern later ones (if execution conflicts with architecture or foundation, execution is wrong; if
an implementation is more convenient than the accepted gates, the implementation is wrong).
**Fetch every file from commit `fd5ae94ff3225d2f989262b95ed8272945861516` (short `fd5ae94`)** —
the manifest reflects exactly that tree. Do **not** adopt a different "commit of record" from any
file you read: commit hashes that appear *inside* archived specs/acceptance artifacts/triage
registers (e.g. `1541da2`, `c819bbe`, `726b2a1`, `92ba47f`) are that artifact's own audit/authoring
provenance, **not** this baseline. A manifest is path inventory only.

**This brief continues an active certification campaign — it is the *second* brief on the
FIRST-PROOF-CERT gate line.** The repo runs a phase-certification ladder (`docs/2-execution/03`):
`P0-DOC → P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT → FIRST-PROOF-CERT → PHASE-4-ENTRY →
SECOND-PROOF-ENTRY`. Every gate so far has been an **audit spec → mutation-remediation/replacement
spec** pair:

- `P0-CERT`: `0036` audit (→ `P0-CERT scoped remediation`) then `0037` remediation (→ `P0-CERT passed`).
- `SPINE-CERT`: `0038` audit (→ scoped remediation) then `0039` remediation (→ `SPINE-CERT passed`).
- `EPI-CERT`: `0040` audit (→ scoped remediation) then `0041` remediation (→ `EPI-CERT passed`).
- `ORD-LIFE-CERT`: `0042` audit (→ scoped remediation) then `0043` remediation (→ `ORD-LIFE-CERT passed`).
- `FIRST-PROOF-CERT`: `0044` audit (→ **scoped remediation**, the standing state) — and the
  remediation half is **not yet authored**. That is your deliverable.

**Your lineage predecessor (this brief is a *delta* on it — do not re-commission it):** the
**FIRST-PROOF-CERT audit spec**, commissioned by `reports/first-proof-cert-research-brief.md` and
delivered as
`archive/specs/0044_FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC.md`.
Its acceptance artifact
(`reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md`)
and mutation triage register
(`reports/0044_first_proof_cert_mutation_triage_register.md`) render the verdict
**`FIRST-PROOF-CERT scoped remediation`, not `FIRST-PROOF-CERT passed`**.

**The exact nature of the 0044 floor — and how it differs from every prior gate.** This is **not**
a survivor-kill floor (0043 had three known `need_accounting.rs` survivors plus a tool failure). The
0044 floor is a **pure campaign-completion blocker**:

- The **standing checked-in perimeter** wave (`cargo mutants --workspace --no-shuffle` over the
  expanded `.cargo/mutants.toml`) **did not complete**: the process supervisor stopped it at the
  **7200-second wall limit** (`wrapper_wall_timeout`, exit 124) after classifying **2,384 of 2,901
  mutants** — `caught 1869`, **`missed 0`, `timeout 0`**, `unviable 515`. It was interrupted during
  `crates/tracewake-core/src/events/apply.rs` build-phase execution (`stderr.txt` records
  `ERROR interrupted` / `scenario execution internal error err=interrupted phase=Build`).
- Because `docs/2-execution/10`'s evidence-honesty / mutation-evidence rule treats an **incomplete
  campaign as blocking**, the register does **not** certify `FIRST-PROOF-CERT passed` — even though no
  survivor or mutant-level timeout was observed before interruption.
- The **focused FIRST-PROOF wave** (a corrected 7-file `--no-config` run) **completed clean**: 719
  mutants, `caught 600`, **`missed 0`, `timeout 0`**, `unviable 119`. It needs **no** survivor
  remediation, but it does **not** substitute for the complete standing run.
- The 0044 audit also performed a **perimeter-honesty correction**, additively adding two
  minimum-union carriers to `.cargo/mutants.toml`: `crates/tracewake-core/src/time.rs` (temporal
  carrier) and `crates/tracewake-core/src/actions/defs/checkcontainer.rs` (missing-property carrier).
  Post-edit census: **62 files / 2,901 mutants**.

The register's "Required Follow-Up" states verbatim: *"`FIRST-PROOF-CERT` cannot be marked passed
until the standing perimeter campaign is completed or replaced by an accepted remediation spec that
changes the mutation evidence contract."* The gate ladder (`docs/2-execution/03`) and the ledger's
"Next known execution move" (`docs/4-specs/SPEC_LEDGER.md`) make a **separately-numbered
FIRST-PROOF-CERT mutation-remediation + replacement certification spec** the single next admissible
move; no later spec may cite `FIRST-PROOF-CERT passed` or proceed to `PHASE-4-ENTRY`/expansion until
it lands. (Note: as of `fd5ae94` the ledger's archived-spec table records 0044's `scoped remediation`
verdict, but its "Next known execution move" section was last updated through the ORD-LIFE-CERT pass
and does **not** yet enumerate a FIRST-PROOF-CERT entry — your spec is the move the ladder + register
imply but the ledger does not yet name. Treat the gate-failure-handling rule and the register's
Required Follow-Up as authoritative.)

**The structural model you must mirror** (cross-line structural precedents — read in full as *shape*
models in §2, **not** delta seeds): the four prior remediation specs and their acceptance artifacts,
**`0043` (ORD-LIFE-CERT) being the freshest and most-similar** — because `0043` is the only prior
remediation whose blocker *also* included an incomplete/non-completing configured lane (a tool
failure there; a wall-timeout here), so its completion-proof / sharding / tool-failure-resolution
machinery is the closest template. `0041`/`0039`/`0037` supply the canonical remediation anatomy
(standing-perimeter confirmation, clean-baseline preflight, development-vs-certifying-run discipline,
survivor-identity reconciliation, triage-register schema, kill-with-behavior/provenance coverage,
live seam re-proof, replacement acceptance artifact that supersedes the audit artifact). Your spec is
the **first-proof-integration analogue** of `0043`/`0041`/`0039`/`0037`. The FIRST-PROOF-CERT *audit*
spec is **already done** (`0044`) and is **not** re-authored.

---

## 2. Read in full (authority order)

Read these before producing. The user's standing instruction is to read **all** of
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and
`docs/4-specs/*`. Within each tier, entries marked **[primary]** are load-bearing for the
FIRST-PROOF-CERT mutation-remediation spec; the remainder are *boundary-awareness* (read to know what
is **out** of scope and must not be re-audited or "corrected" here — SPINE-CERT / EPI-CERT /
ORD-LIFE-CERT seams are already certified; PHASE-4-ENTRY / SECOND-PROOF-ENTRY material is downstream).

**Foundation (`docs/0-foundation/`)**
- `docs/README.md` — **[primary]** authority layering and the "execution is wrong if it conflicts with
  architecture/foundation" rule.
- `00_FOUNDATION_INDEX.md` — index/boundary map (boundary-awareness).
- `02_CONSTITUTIONAL_INVARIANTS.md` — **[primary]** `INV-001…INV-112`; every property the remediation
  re-proves must satisfy these (notably `INV-111` observer-only emergence, `INV-112` temporal
  authority); the spec amends none.
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **[primary]** first-playable acceptance doctrine:
  the missing-property situation, the world that runs without knowing whether a human is present, the
  nine acceptance gates, the explicit non-goals, and the observer-only emergence-evidence requirement —
  the doctrine the integrated proof the remediation re-proves must satisfy.
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **[primary]** event/replay contract; the standing
  mutation perimeter's `events/apply.rs` and `replay/rebuild.rs` carriers live here, and replay
  determinism is part of the integrated re-proof.
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **[primary]** actor-known cognition
  transaction + truth firewall; kill-witnesses may not source expectations from ground truth (esp. for
  the missing-property/expectation carriers `epistemics/contradiction.rs`, `agent/perception.rs`).
- `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — **[primary]** the claims/beliefs/expectation/
  contradiction substrate behind the missing-property mutation carriers.
- `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — **[primary]** the ordinary-life affordance
  contract the `checkcontainer.rs` / no-human carriers exercise.
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **[primary]** possession parity and embodied/debug
  split a behavior-witness may consume as certified consequence.
- `01, 05, 07, 09, 10, 11, 13` — boundary-awareness (charter; needs/planning — ORD-LIFE, certified;
  view models — EPI; institutions; no-scripting/seeds; LOD/scale; LLM/speech; research notes). Read to
  bound scope; not remediation targets.

**Architecture (`docs/1-architecture/`)**
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **[primary]** canonical gate/review-artifact
  composition; FIRST-PROOF-CERT is a phase-certification *label* composing gates defined here, minting
  no new gate code.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — **[primary]** the one-way crate
  dependency rule (`core ← content ← tui`) no remediation delta may invert; no test-only production
  dependency.
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **[primary]** projection-rebuild and
  determinism contract: metrics/diagnostics are replay-derived projections, never truth writers — the
  consequence a kill-witness for the event/replay carriers should observe (replay must localize
  divergence, not only assert mismatch).
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **[primary]** what a gate artifact
  must contain; typed-diagnostic and review-artifact obligations the replacement artifact instantiates.
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **[primary]** holder-known/actor-known
  sealed contexts + provenance sufficiency/freshness — the substrate the missing-property /
  expectation carriers consume; kill-witnesses cite provenance, truth never proposes targets.
- `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — **[primary]** the architecture
  contract for claims/beliefs/observation/memory and **expectation contradiction** — the heart of the
  missing-property mutation carrier (`epistemics/contradiction.rs`).
- `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — **[primary]** the spatial/property/
  custody model the missing-property and `checkcontainer.rs` carriers move through (no teleport to true
  property).
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — **[primary]** embodied view models
  expose only holder-known context; debug surfaces non-diegetic — consumed as certified consequence
  where a witness needs it.
- `04` — read **only** for the no-direct-dispatch boundary the integrated proof must hold;
  `05, 07, 08, 11, 12, 14` — boundary-awareness (decision transaction — ORD-LIFE; speech/LLM;
  institutions; incidents/leads/story-sifting — explicitly out; LOD/prehistory; forbidden misreads).
  Read to bound scope; not remediation targets.

**Execution (`docs/2-execution/`)**
- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — **[primary]** execution-tier authority and the canonical gate
  index (FIRST-PROOF-CERT consumes canonical gate evidence; it is not a new gate).
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — **[primary]** the post-0008
  baseline, code-audit boundary, and the three admissibility postures
  (`passed` / `scoped remediation` / `not applicable`).
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — **[primary]** THE first-proof
  scope and acceptance contract the remediation re-proves: the **nine acceptance gates** (`EVENT`,
  `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `POSSESSION-PARITY`, `NO-HUMAN-ORDINARY-LIFE`, `MISSING-PROPERTY`,
  `VIEW-DEBUG-SPLIT`, `REPLAY`, `FIXTURE-NEGATIVE`), the **nine scenario families**, the observer-only
  emergence artifact, the "definition of first-proof done", and the explicit non-goals.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **[primary]** defines `FIRST-PROOF-CERT`
  (gate 5), the **gate-evidence requirements**, the **gate-failure-handling rule** (a failed/incomplete
  gate must produce a remediation spec/report that names the failing layer; it may not be relabeled a
  phase exception), the **valid future-spec postures** (declare exactly one: this is `Remediation`), the
  EMERGE-OBS observer-only rule, and the **Temporal-Cascade** placement (the consolidated temporal
  bundle is part of the existing FIRST-PROOF-CERT obligation).
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **[primary]** the anti-contamination
  / truth-firewall gates a kill-witness must respect (no fact from prose; no decision from unobserved
  ground truth; validation truth may not propose; no teleport to true food/property; no culprit truth).
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — **[primary]** golden-fixture/replay
  acceptance obligations; the positive + adversarial fixture families a behavior-witness travels
  through (incl. expectation-contradiction, no-hidden-truth planning, no-culprit-truth, no-teleport,
  possession-does-not-reset, no debug leakage, no marker-only life, no replay drift, content rejection,
  and the temporal fixture families).
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **[primary]** THE governing doc for
  this blocker: the **evidence-honesty rule** and the **mutation / phase-entry evidence rule** (a tool
  failure, incomplete run, or timeout is **not** a pass), diagnostics-by-layer, review-artifact
  obligations, the single diagnostic home, and EMERGE-OBS observer-only discipline. Directly governs how
  the incomplete-campaign blocker is resolved.
- `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **[primary]** the no-human ordinary-day
  proof and routine-temporal evidence the integrated re-proof composes (consumed via `ORD-LIFE-CERT
  passed`; read for the no-human acceptance the integrated proof exercises).
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **[primary]** the embodied view-model /
  possession / debug-quarantine proof (`VIEW-DEBUG-SPLIT`, `POSSESSION-PARITY`) and embodied temporal
  rendering the integrated re-proof composes.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — **[primary]** read for the
  no-direct-dispatch boundary the integrated proof must hold (SPINE-CERT certified the pipeline itself;
  do **not** re-audit it).
- `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — boundary-awareness: Phase-4 entry;
  wrong-suspicion/records are **out**. Read precisely to keep the missing-property carriers from
  importing investigation machinery.
- `08, 12, 13` — boundary-awareness (data authoring/schema — read only where the proof consumes
  authored fixtures/seed knowledge; deferred second-proof; research notes). Read to bound scope.

**Reference (`docs/3-reference/`)**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **[primary]** review-checklist questions.
- `01_DESIGN_RISK_REGISTER.md` — **[primary]** standing risks incl. the **anti-Goodhart rule** (killing
  only a named subset, improving a mutation score, or shrinking the denominator of convenience cannot
  certify the line) and the contamination / hidden-truth-leakage / silent-starvation / replay-drift /
  Goodharting-emergence risks the integrated proof must resist.
- `02_GLOSSARY.md` — **[primary]** canonical terminology (`actor-known`, `holder-known context`,
  `missing property`, `expectation contradiction`, `no-human proof`, `possession parity`, `behavioral
  progress`, `stuck diagnostic`, `EMERGE-OBS`, etc.) the spec must use exactly.

**Specs (`docs/4-specs/`)**
- `SPEC_LEDGER.md` — **[primary]** live ledger: authority posture, source discipline, the archived-spec
  status table (records 0044 `FIRST-PROOF-CERT scoped remediation`), and the "Next known execution
  move" section. **Note the staleness flagged in §1**: the "Next known execution move" lists the four
  earlier passed gates but does not yet enumerate FIRST-PROOF-CERT — do not read its silence as
  "nothing to do"; the gate-failure-handling rule + the 0044 register's Required Follow-Up govern.
- `README.md` — **[primary]** rules for future specs (declare authority posture; declare exactly one
  admissibility posture; gate codes as cross-references only; terminology; no files for symmetry).
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — **[primary]** the review-artifact template (evidence-status /
  fingerprint-scope / behavior-witness / replay-provenance / sampling / pending-historical /
  certification-use / staged-abstraction fields) the replacement FIRST-PROOF-CERT acceptance artifact
  must instantiate.
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — **[primary]** the live first-proof
  village/fixture ontology — the substrate the missing-property carriers and scenario families exercise
  (read as a load-bearing fixture-contract source for the integrated re-proof).

**Lineage predecessor — the FIRST-PROOF-CERT audit half (read in full; this is the *delta seed*)**
- `archive/specs/0044_FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC.md`
  — **[primary]** the audit contract: the `FIRST-PROOF-01…NN` numbered audit points, the integrated
  nine-gate / nine-scenario-family coverage table, the temporal-bundle audit points, the mutation
  posture (§ the standing perimeter + focused wave), and the acceptance-artifact contract. These are the
  seams your remediation **re-proves live** at the final implementation commit; do not redesign them.
- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md`
  — **[primary]** the `FIRST-PROOF-CERT scoped remediation` acceptance artifact your replacement
  artifact must supersede (study its scope/verdict shape and which seams it already exercised as
  evidence).
- `reports/0044_first_proof_cert_mutation_triage_register.md` — **[primary]** the **exact
  completion-blocker record**: the standing-wave `wrapper_wall_timeout` (2,384/2,901 classified; 0
  missed / 0 timeout / 515 unviable; interrupted in `events/apply.rs`), the perimeter-honesty additions
  (`time.rs`, `checkcontainer.rs`; 62 files / 2,901 mutants), the completed focused wave (719 mutants,
  0 missed), the stable-identity scheme, and the "Required Follow-Up" that names the completion-or-
  replacement obligation your spec discharges. The evidence/fingerprint formats your remediation register
  reuses and extends.
- `reports/first-proof-cert-research-brief.md` — **[primary]** the brief that commissioned the audit;
  read it for the campaign conventions, the source-discipline language, the §2 [primary] read-list, and
  the §5 code-seam inventory (the integration harness / fixture-family list is reusable orientation for
  the FIRST-PROOF perimeter and the live re-proof).

**Cross-line structural precedents — the remediation-spec shape you mirror (read in full; *not* delta seeds)**
- `archive/specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` —
  **[primary]** the **freshest, closest** remediation template **and the only prior remediation whose
  blocker also included a non-completing configured lane**: its tool-failure-resolution /
  complete-configured-run / completion-proof / sharding / timeout-disposition machinery is the closest
  model for the wall-timeout blocker. Clone its structure; retarget every ordinary-life seam to the
  integrated first-proof seam, and re-frame the "tool failure" objective as "wall-clock completion of a
  ~2,901-mutant denominator."
- `archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`,
  `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`,
  `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` —
  **[primary]** the canonical remediation anatomy (standing-perimeter confirmation, clean-baseline
  preflight, development-vs-certifying-run discipline, sharding/timeout/completion-proof, survivor-
  identity reconciliation, triage-register schema, kill-with-behavior/provenance coverage, live seam
  re-proof, replacement acceptance artifact). `0039` is the perimeter-widening case; `0037` the original
  pair.
- `archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md`,
  `archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`,
  `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`,
  `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` —
  **[primary]** the **passing** replacement acceptance artifacts (the shape the eventual
  FIRST-PROOF-CERT `passed` artifact targets — what the implementing session records to flip the line).
  These are also the artifacts your header's admissibility posture **cites** for the four consumed gates.
- `reports/0043_ord_life_cert_mutation_triage_register.md` and
  `archive/reports/0040_epi_cert_mutation_triage_register.md` — **[primary]** the triage-register format
  the FIRST-PROOF register reuses; `archive/reports/0040_epi_cert_mutation_final_missed.txt` — the
  canonical missed-identity manifest format.
- `reports/ord-life-cert-mutation-remediation-research-brief.md` and
  `archive/reports/epi-cert-mutation-remediation-research-brief.md` — **[primary]** the **sibling-gate
  remediation-brief precedents** (the briefs that commissioned `0043` and `0041`); reuse their framing,
  source-discipline language, and completion-proof emphasis where it fits.

(Also present for orientation, not load-bearing: the campaign's prior `reports/*-research-brief.md`
and `archive/reports/*-research-brief.md`, and `archive/specs/0002…0035` — *historical evidence
only*, never live certification, per the ledger and `docs/2-execution/03`. The live doctrine above
governs.)

---

## 3. Settled intentions (final — do not reopen)

1. **The determination is pre-settled and doctrine-forced.** `P0-CERT passed` (0037 artifact),
   `SPINE-CERT passed` (0039 artifact), `EPI-CERT passed` (0041 artifact), and `ORD-LIFE-CERT passed`
   (0043 artifact) are the current admissibility state; `FIRST-PROOF-CERT` is **`scoped remediation`**
   (0044 artifact). The gate ladder (`docs/2-execution/03`), the **gate-failure-handling rule** (a
   failed/incomplete gate must produce a remediation spec naming the failing layer; it may not be
   relabeled a phase exception), the **mutation/phase-entry evidence rule** (`docs/2-execution/10`: an
   incomplete or tool-failed run is not a pass), and the 0044 register's "Required Follow-Up" make a
   **separately-numbered FIRST-PROOF-CERT mutation-remediation + replacement certification spec** the
   single next admissible move. **Open the deliverable with a short, evidence-based confirmation of this
   determination** (cite the gate ladder + the four passed predecessor gates + the 0044
   scoped-remediation verdict + the register's completion-or-replacement obligation). Do **not** survey
   alternative "next features", and do **not** propose gameplay expansion — the ladder explicitly
   invalidates any Phase-4/expansion/feature spec attempted before FIRST-PROOF-CERT passes.

2. **Center of gravity is campaign COMPLETION, not survivor-killing — this is the central divergence
   from 0043.** Unlike every prior remediation (`0037`/`0039`/`0041`/`0043` all remediated a *survivor
   floor*; `0043` additionally a tool failure), the **0044 floor has zero known actionable survivors**
   (standing wave: `missed 0`, `timeout 0` before interruption; focused wave: clean). The blocker is
   purely that the **standing configured `cargo mutants --workspace --no-shuffle` campaign over the
   ~2,901-mutant perimeter did not complete within the 7200-second wall**. The spec must therefore make
   **"get the full standing configured mutation campaign to run to completion within wall/compute
   limits, with a rigorous completion proof, retaining outputs even on interruption, and distinguishing
   a tool-failure / wall-timeout / incomplete run from a genuine `caught`/`missed`/mutant-level-timeout
   result"** the **first-class, dominant remediation objective**. Pull the completion-proof, sharding,
   timeout-disposition, and run-supervision machinery forward from `0043` §4.6 / `0041` §4.6 and adapt
   it to a **wall-clock-scaling** problem (e.g. sharding the single denominator across multiple
   reproducible CI jobs/runs, raising parallelism/`--jobs`, extending the supervised wall budget, or a
   combination) rather than a PTY-stall diagnosis. The acceptance precondition is a **complete configured
   run over the full standing perimeter**; an incomplete or wall-timed-out lane can never render
   `FIRST-PROOF-CERT passed`.

3. **No-shrink, anti-Goodhart: the ~2,901-mutant standing denominator is the contract.** Acceptance
   requires a *complete* configured run over the **0044-expanded `.cargo/mutants.toml`** (62 files /
   2,901 mutants — re-confirm via `--list-files`/`--list` at the final commit, explaining any delta from
   that census by final-tree source changes only). The spec must **forbid silently shrinking the
   denominator** to force completion (no `--no-config` / `-f` subset / `--exclude` / `--in-diff` /
   `--iterate` as the *final certifying* denominator; the focused 7-file wave is development evidence,
   not the certifying run). The 0044 register notes the run *"may be completed or replaced by an
   accepted remediation spec that changes the mutation evidence contract"* — per the user's settled
   decision, the spec's posture is **complete the full perimeter, do not change the contract to a smaller
   one**; any contract change would itself be anti-Goodhart and is out of scope. Killing only a named
   subset, improving a mutation score, or parking the rest as baseline misses cannot certify the line
   (`docs/3-reference/01`).

4. **Any survivor the now-complete run exposes joins the remediation obligation.** Completing the
   campaign will classify the **~517 previously-unclassified mutants** (2,901 − 2,384), including the
   `events/apply.rs` region where the standing wave was interrupted. If the complete run exposes any
   `missed` survivor or mutant-level `timeout`, the spec requires the implementing session to reconcile,
   triage, and **kill it with behavior/provenance coverage** (intention 5) before acceptance — these are
   not pre-known (do not fabricate a survivor list), but the spec must contain the full triage machinery
   to handle them. If the complete run leaves a genuine survivor floor, the outcome is
   `FIRST-PROOF-CERT scoped remediation` routed to a named follow-up, not a relabeled exception.

5. **Kill-with-behavior/provenance coverage is the default** (mirror `0041` §4.9 / `0043` §3.5). Any
   killing witness (for a survivor the complete run exposes) must: pass against the unmutated final
   implementation; fail when the mutant is active; observe a public or certified *first-proof*
   consequence (e.g. a wrong expectation-contradiction outcome, a wrong duration/temporal interval, a
   wrong replay-derived projection, a wrong missing-property discovery, a content-validation rejection),
   **not** restate the mutated expression or assert a helper literal; travel the production seam (no
   test-only bypass); carry replay/provenance ancestry where the behavior is event-derived/projected;
   include a negative/contamination control where truth/debug/prose/direct state could shortcut it; and
   name the responsible failing layer.

6. **Re-prove the FIRST-PROOF seam contract live + instantiate the replacement acceptance artifact.** The
   replacement package must re-prove the `0044` `FIRST-PROOF-01…NN` integrated audit points — including
   the **nine acceptance gates**, the **nine scenario families**, the **MISSING-PROPERTY** surface, and
   the **consolidated temporal evidence bundle** — at the exact final implementation commit (clean
   preflight: `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
   `cargo build --workspace --all-targets --locked`; `cargo test --workspace --locked`; plus the named
   first-proof gate suites), at **one unified baseline commit**, and produce a replacement acceptance
   artifact conforming to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` that renders
   `FIRST-PROOF-CERT passed` and **explicitly supersedes** the 0044 acceptance artifact. Carry the
   integrated-coherence posture from 0044: the four prior `*-CERT passed` gates are **consumed** as
   certified building blocks (cited, not re-audited); FIRST-PROOF-CERT newly establishes the unified-
   commit integrated coherence proof; consuming a prior pass is **not** a latest-main certification claim.

7. **Form: a non-executable mutation-remediation + replacement certification spec** in the exact
   `0043`/`0041`/`0039`/`0037` lineage (`0043` the freshest structural model). You (Session 2) **cannot**
   run `cargo test`, `cargo-mutants`, or replay — so the spec specifies *what the implementing session
   must inspect, configure, run, prove, record, review, and package*; it renders **no** pass/fail
   verdict, does **not** assert the campaign has completed or any survivor is killed, and **fabricates no
   test/mutation/replay results**. Carry the campaign's source-discipline preamble (the "I am not
   verifying this commit is current `main`; I use the supplied commit as the target of record and fetch
   files only by exact commit URL/export" statement, adapted to `fd5ae94`), plus the
   authoring-baseline-vs-final-implementation-commit discipline (every delta enumerated; certifying
   commands run against one exact final commit; both SHAs reported separately; neither presented as
   current-`main` verification).

8. **EMERGE-OBS stays observer-only.** The no-human / missing-property ordinary day is the first-proof
   living-world acceptance corpus (`docs/0-foundation/12`, `INV-111`). The spec may name the observer-only
   `EMERGE-OBS` artifact as a required evidence-package member of the replacement package, but it **never**
   becomes a phase gate or pass/fail threshold (per `docs/2-execution/10` and `03`).

9. **Numbering & placement.** Filename:
   `specs/0045_FIRST_PROOF_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`
   (`<DESCRIPTIVE_NAME>` may be refined in the established ALL-CAPS-underscored style). The staging
   series in `archive/specs/` is contiguous `0002 → 0044` with `specs/` currently empty, and recent git
   history shows each spec archived by a plain rename with no renumber/restart (e.g.
   `R098 specs/0044… → archive/specs/0044…`); the lineage predecessor (the FIRST-PROOF-CERT audit) is
   `0044`, so this remediation continues the sequence at **`0045`** (the live ledger's `0001`/`0003`
   live-tier numbering does not block continuing the staging sequence). `assumption:` stage under
   `specs/` (not directly in `docs/4-specs/`), per the repo's stage-then-archive convention; it is
   archived to `archive/specs/0045_…` on accepted closeout and is **not** promoted into live
   `docs/4-specs/`. The user may relocate if desired. Header block must mirror the `0043` header:
   staging path, archive-on-closeout path, target repository, **target commit
   `fd5ae94ff3225d2f989262b95ed8272945861516`**, spec series, status, **Future-spec posture:
   `Remediation`**, **Admissibility posture: `FIRST-PROOF-CERT scoped remediation`**, **Consumed
   predecessor gates** (`P0-CERT passed` via 0037, `SPINE-CERT passed` via 0039, `EPI-CERT passed` via
   0041, `ORD-LIFE-CERT passed` via 0043), a **Certification-line effect** line (successful execution
   must publish a replacement acceptance artifact rendering `FIRST-PROOF-CERT passed` that supersedes the
   0044 artifact), and a one-line **non-executable** statement.

10. **Source discipline (carry verbatim from the ledger).** Manifests are path inventory only;
    branch/default-branch/code-search/repo-metadata are not proof of target-commit content; archived
    specs/tickets/reports are cited as **history or structural precedent only**, never as live
    certification; commit strings embedded in archived artifacts (`1541da2`, `c819bbe`, `726b2a1`,
    `92ba47f`, etc.) are those artifacts' own provenance, not this baseline. The spec may **not** amend
    invariants, redefine/weaken gate semantics, or **mint a new gate code, status enum, obligation code,
    invariant ID, or doctrine-level finding ID** — `FIRST-PROOF-CERT` is a phase-certification *label*
    that composes the canonical gates/review-artifacts from architecture `00` and execution `00`/`02`,
    exactly as `docs/2-execution/03` and `02` state. Survivors remain cargo-mutants
    `path:line:column:operator` identities. Use `actor-known` for the actor case and `holder-known
    context` as the system-wide term; use `missing property` and `expectation contradiction` exactly per
    the glossary.

11. **`assumption:`** the spec **may** include a single, clearly-labeled **"preliminary, non-certifying
    static survey"** of what reading the code at `fd5ae94` suggests about the perimeter's coverage, the
    `events/apply.rs` interruption region, and the wall-clock-scaling options — explicitly marked
    *informative, not certification*. Default is to include it; omit only if it would risk reading as a
    verdict.

---

## 4. The task

Author the **`FIRST-PROOF-CERT` mutation-remediation + replacement certification spec**
(`specs/0045_*`): the single next admissible spec on the Tracewake certification ladder. It is a
*new-spec*, *non-executable*, `Remediation`-posture deliverable — an evidence-complete plan the
implementing session executes to (a) **get the full standing configured `cargo mutants` campaign over
the ~2,901-mutant perimeter to run to completion within wall/compute limits, with a rigorous
completion proof and tool-failure/timeout/incomplete-vs-result discipline** (the dominant objective,
since the 0044 floor is a wall-timeout, not a survivor floor); (b) **reconcile, triage, and kill with
behavior/provenance any survivor the now-complete run exposes** among the previously-unclassified
mutants, without shrinking the denominator; (c) **re-prove the `0044` `FIRST-PROOF-01…NN` integrated
nine-gate / nine-scenario-family acceptance contract (incl. the MISSING-PROPERTY surface and the
consolidated temporal bundle) live at one exact final implementation commit**; and (d) **publish a
replacement acceptance artifact that renders `FIRST-PROOF-CERT passed` and supersedes the 0044
artifact** — all against live foundation/architecture/execution doctrine at commit `fd5ae94`. The spec
must be detailed enough that a Tracewake implementing session executes it without further design.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular the standing
`.cargo/mutants.toml` perimeter, the `.cargo/mutants-baseline-misses.txt` state, the CI mutation jobs
in `.github/workflows/ci.yml`, the FIRST-PROOF mutation carriers (`crates/tracewake-core/src/{time.rs,
events/apply.rs, replay/rebuild.rs, epistemics/contradiction.rs, epistemics/projection.rs,
agent/perception.rs, actions/defs/checkcontainer.rs}`, `crates/tracewake-content/src/validate.rs`),
and the integration / first-proof test harnesses under `crates/tracewake-core/tests/` (e.g.
`acceptance_gates.rs`, `acceptance_artifact_wording.rs`, `golden_scenarios.rs`, `hidden_truth_gates.rs`,
`event_schema_replay_gates.rs`, `negative_fixture_runner.rs`, `no_human_capstone.rs`,
`emergence_ledger.rs`, `generative_lock.rs`, `spine_conformance.rs`, `support/`) plus
`crates/tracewake-content/tests/golden_fixtures_run.rs` and the `crates/tracewake-content/src/fixtures/`
corpus — all at `fd5ae94`. Re-confirm the perimeter census (`--list-files`/`--list`) and name the real,
verified file/seam set in the spec; if line numbers in the 0044 register's stable-identity scheme have
shifted, map by enclosing symbol + operator, not by line.

Research online as deeply as needed — similar implementations and prior art — wherever it sharpens the
remediation design. In particular: **scaling and completing large mutation-testing campaigns under a
wall-clock budget** (cargo-mutants `--list`/`--list-files` denominators, `--jobs`/parallelism,
`--shard`/sharding a single denominator reproducibly across CI jobs, baseline/timeout semantics,
`--in-place`/`--iterate` trade-offs, retaining `mutants.out` artifacts on interruption, and the
distinction between a tool/wall timeout and a genuine mutant-level timeout/miss); **CI design for
expensive mutation lanes** (scheduled full runs vs. fast in-diff change detectors; reproducible shard
assignment; merging shard outcomes into one complete-denominator proof; deterministic termination and
stall detection in process-supervision wrappers); **mutation testing of cross-cutting integration
surfaces** (scoping a guarded-layer perimeter over multiple subsystems, triaging survivors, identifying
equivalent mutants, killing via externally observable behavior rather than tests over private
internals); **property-based and metamorphic testing** for the combined corpus and replay determinism
(divergence localization across composite scenario sets); and **anti-Goodhart / observer-only
emergence-evidence** design (how a living-world "emergence" artifact stays descriptive and never becomes
a steered pass/fail target; why the denominator must not be shrunk to force a pass). Cite sources for
any external claim that shapes a decision. The *deep* research is yours to perform; the determination of
*which* spec is **not** — that is settled in §3.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every property the
  remediation re-proves must satisfy it (notably `INV-111` observer-only emergence and `INV-112`
  temporal authority); a genuine divergence would require amending an invariant first — never design
  against it. The spec amends nothing.
- Authority order: if execution conflicts with architecture or foundation, execution is wrong; if an
  implementation is more convenient than the accepted gates, the implementation is wrong; if a test
  kills a mutant by asserting a mutated helper's literal return instead of a certified first-proof
  consequence, the test is wrong; if an artifact promotes archived evidence or an incomplete/shrunken
  mutation run into a live pass, the artifact is invalid.
- No backwards-compatibility shims or alias paths in new work.
- Anti-contamination (first-proof specifics): no simulation fact may be born from prose; the missing
  property is discovered through actor-relative expectation + observation, **never** a global theft
  truth or culprit flag; truth may validate (reachability, presence, access, invariants) but must not
  choose goals, methods, hidden food targets, suspects, witnesses, clues, or view-model affordances; no
  actor teleports to true food/property; the scheduler may not dispatch ordinary actions directly or
  author state deltas; possession binds input only and grants no privileged body/memory/actions/truth;
  embodied view models expose only holder-known context; debug surfaces stay non-diegetic and never feed
  cognition; replay rebuilds projections + diagnostics deterministically and localizes divergence.
- No protagonist gravity / human special-case: no accepted behavior may require knowing which actor, if
  any, is human-controlled.
- The crate dependency direction (`core ← content ← tui`) must never invert; no test-only production
  dependency.
- Mint no new gate code, status enum, obligation code, invariant ID, or finding ID. Cross-reference
  existing ones. `FIRST-PROOF-CERT` is a composing label; `EMERGE-OBS` stays observer-only.
- The certifying mutation denominator may **not** be silently shrunk; the focused 7-file wave is
  development evidence, not the certifying run; an incomplete / wall-timed-out / tool-failed campaign is
  not a pass.
- Explicit non-goals (out of scope, route forward, do not audit): institutions / wrong-suspicion /
  records / full investigation (PHASE-4-ENTRY); notices / travel / regional scale / LOD / story-sifting
  (SECOND-PROOF-ENTRY); LLM dialogue (locked); latest-main certification.

---

## 7. Deliverable specification

Produce **one** downloadable markdown document:

- **`specs/0045_FIRST_PROOF_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`** — **new**
  numbered staging spec (not a replacement of any existing file). This is a *new-spec* deliverable:
  there is **no** paired `-research-report.md`. The numbering/staging-series rules in §3.9 (staging
  series → `0045`) apply; do not treat this as a `reports/` analysis report.

Required structure (follow the `0043`/`0041`/`0039`/`0037` remediation anatomy, retargeted to the
integrated first-proof substrate and the wall-timeout completion blocker):

1. **Header block** — per §3.9 (staging + archive-on-closeout paths, target repo, **target commit
   `fd5ae94ff3225d2f989262b95ed8272945861516`**, spec series, status, `Future-spec posture:
   Remediation`, `Admissibility posture: FIRST-PROOF-CERT scoped remediation`, consumed predecessor
   gates citing the 0037/0039/0041/0043 artifacts, certification-line effect, non-executable statement)
   + the source-discipline preamble.
2. **Status, determination, source discipline, freshness, admissibility** — the single posture; the
   certification-line state (the line stays `scoped remediation` until the replacement artifact passes;
   this spec does not flip it); the cited determination confirmation (next admissible move; gate-5 floor
   is a completion blocker); exact-commit source discipline (`fd5ae94`; embedded hashes are not this
   baseline); authoring-baseline-vs-final-implementation-commit discipline; and the admissibility lock
   (no later spec may cite `FIRST-PROOF-CERT passed`; PHASE-4-ENTRY / SECOND-PROOF-ENTRY remain locked).
3. **Authority & dependency declarations** — controlling doctrine order; the primary first-proof sources
   (foundation `12`, execution `02`/`03`/`10`); the foundation/architecture/execution/reference/live-spec
   dependencies (the §2 [primary] list); crate direction and implementation boundary; the primary
   code/test/config seams.
4. **Problem statement** — the 0044 verdict and floor, stated precisely: the standing configured wave's
   `wrapper_wall_timeout` at the 7200s wall (2,384/2,901 classified; **0 missed / 0 timeout** / 515
   unviable; interrupted in `events/apply.rs`); the completed clean focused wave (719 mutants, 0 missed);
   the perimeter-honesty additions (`time.rs`, `checkcontainer.rs`; 62 files / 2,901 mutants); the
   **central divergence from 0043** (no known survivors — the blocker is *campaign completion within
   wall-clock*, not a survivor kill); a static perimeter confirmation at `fd5ae94` (the perimeter covers
   the carriers; the baseline-miss file's state); and any CI scheduled/in-diff mutation-lane durability
   gap for the FIRST-PROOF perimeter.
5. **Remediation approach** — the required end state (a numbered checklist mirroring `0043`/`0041` §4.1,
   with the **complete-configured-run / wall-clock-completion** items as first-class and dominant);
   maintain & re-confirm the standing configured perimeter (`--list-files`/`--list` capture; **no
   `--no-config`/`-f` subset / `--exclude` / `--in-diff` / `--iterate` as the final certifying
   denominator; no silent shrink**); **mutation-lane completion & wall-clock scaling** (diagnose why the
   ~2,901-mutant lane exceeds the 7200s wall; resolve via sharding the single denominator reproducibly
   across CI jobs/runs and/or raising `--jobs`/parallelism and/or extending the supervised wall budget;
   retain `mutants.out` outputs on interruption; **a complete run over the full perimeter is a
   precondition of acceptance**; distinguish wall-timeout vs. mutant-level timeout vs. miss vs.
   tool-failure); CI convergence & baseline-miss discipline (any survivor exposed must not be parked in
   `.cargo/mutants-baseline-misses.txt`); clean-baseline + named-preflight; development-vs-certifying-run
   discipline; sharding/timeouts/completion-proof; survivor-identity reconciliation (map any exposed
   survivor through refactors by symbol+operator); the **triage-register schema** (reuse the 0040/0042/
   0043 format: historical identity, final identity, tool outcome, FIRST-PROOF cross-ref, responsible
   layer, certified reachability, test family, behavior witness, replay/provenance ancestry, negative/
   contamination control, certification disposition, evidence references, review signoff); and the
   **kill-with-behavior/provenance coverage** doctrine with first-proof-specific insufficient-example
   list (per §3.5). State explicitly that the focused wave's clean result is *development evidence*, not
   the certifying run.
6. **Failure handling** — diagnostics named by the `docs/2-execution/03` responsible-layer list (incl.
   content/schema validation / actor-known context / candidate generation / planning / proposal / event
   application / projection-replay / view-model rendering / debug quarantine / tests-fixtures /
   documentation status, **plus the mutation-infrastructure / configured-perimeter-completion layer** the
   wall-timeout maps to); a failed seam, surviving floor, or incomplete run yields `FIRST-PROOF-CERT
   scoped remediation` routed to a named follow-up, not a relabeled phase exception.
7. **Live seam re-proof** — re-prove the `0044` `FIRST-PROOF-01…NN` audit points at the exact final
   implementation commit (clean preflight + the named first-proof gate suites), at **one unified baseline
   commit**; a coverage table mapping the re-proof onto the **nine first-proof acceptance gates** and the
   **nine scenario families** in `docs/2-execution/02`, plus the temporal bundle's five routed sources
   (`04`/`06`/`07`/`09`/`10`), so completeness is auditable. Reaffirm the integrated-coherence posture
   (four prior gates consumed, not re-audited; unified-commit integrated proof; not a latest-main claim).
8. **Acceptance-artifact contract** — instantiate `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:
   exactly what the implementing session records to render `FIRST-PROOF-CERT passed` (evidence-status,
   fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical,
   certification-use, staged-abstraction fields), including the **complete-configured-run completion
   proof** as a required evidence-package member; the replacement artifact must explicitly supersede the
   0044 artifact; name `EMERGE-OBS` as an evidence-package member (never a gate); state that this spec
   specifies the remediation and renders no verdict, and that an incomplete run or surviving floor yields
   `FIRST-PROOF-CERT scoped remediation` routed to a named follow-up.
9. **(Optional) Preliminary static survey** — clearly labeled "preliminary, not certification" (per
   §3.11).
10. **Tolerated deferrals** — explicitly out of scope and routed forward: institutions / wrong-suspicion
    / records / full investigation to `PHASE-4-ENTRY` (`execution 11`); notices / travel / regional scale
    / LOD / story-sifting to `SECOND-PROOF-ENTRY` (`execution 12`); LLM dialogue (locked); latest-main
    certification; and a statement that the four prior gates' internals are already certified and are
    **not** re-audited here (only their participation in the coherent integrated set is re-proved).
11. **(Optional) Appendix** — exact-URL evidence ledger for the files fetched at `fd5ae94` (mirror the
    `0041`/`0043` appendix), and references for every external claim.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec opens with a cited confirmation that the FIRST-PROOF-CERT mutation-remediation/replacement
      spec is the next admissible move (predecessor gates `P0-CERT`/`SPINE-CERT`/`EPI-CERT`/`ORD-LIFE-CERT`
      passed; `FIRST-PROOF-CERT` is `scoped remediation`); it proposes no feature/expansion work.
- [ ] **Completing the full standing configured mutation campaign within wall/compute limits (with a
      rigorous completion proof) is the first-class, dominant objective** — the spec frames the 0044
      floor as a wall-timeout / completion blocker (2,384/2,901; 0 missed / 0 timeout), **not** a survivor
      floor, and makes a complete run a precondition of acceptance; it distinguishes wall-timeout /
      mutant-level timeout / incomplete / tool-failure from `caught`/`missed`.
- [ ] The ~2,901-mutant standing denominator is **not** shrunk to force completion (no `--no-config`/`-f`
      subset / `--exclude` / `--in-diff` as the final certifying denominator; focused wave = development
      evidence only); the spec requires triaging **every** survivor the complete run exposes among the
      ~517 previously-unclassified mutants; the perimeter additions (`time.rs`, `checkcontainer.rs`) are
      confirmed present; survivors are not parked in `.cargo/mutants-baseline-misses.txt`.
- [ ] Kill-witness doctrine is "kill-with-behavior/provenance" — observing certified first-proof
      consequences (expectation-contradiction / temporal-interval / replay-derived metric /
      missing-property discovery / content rejection), not helper literals; with negative/contamination
      controls and named responsible layers.
- [ ] The spec re-proves the `0044` `FIRST-PROOF-01…NN` integrated seam contract live at one exact final
      commit and includes a coverage table onto the nine acceptance gates + nine scenario families +
      five temporal-bundle sources in `docs/2-execution/02`/`03`; it reaffirms that the four prior gates
      are consumed (not re-audited) and that this is not a latest-main certification claim.
- [ ] The deliverable is non-executable: it specifies the remediation/acceptance contract, renders **no**
      pass/fail verdict, and fabricates no test/mutation/replay results (it does not assert the campaign
      completed or any survivor is killed). Any static survey is labeled "preliminary, not certification".
- [ ] Scope is the remediation spec only; the `0044` audit is cited (as the seam list to re-prove), not
      re-authored; SPINE/EPI/ORD-LIFE seams are not re-audited; institutions/notices/travel/LOD/
      story-sifting are routed forward in tolerated deferrals.
- [ ] Filename is `specs/0045_FIRST_PROOF_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`;
      header carries posture `Remediation`, admissibility `FIRST-PROOF-CERT scoped remediation`, consumed
      gates citing the 0037/0039/0041/0043 artifacts, certification-line effect (supersede the 0044
      artifact), and target commit `fd5ae94ff3225d2f989262b95ed8272945861516`.
- [ ] The acceptance-artifact contract instantiates `docs/4-specs/0003`'s fields, requires the
      complete-run completion proof, and explicitly supersedes the 0044 acceptance artifact on a passing
      outcome.
- [ ] No new gate code / invariant ID / status enum / finding ID is minted; `FIRST-PROOF-CERT` is treated
      as a label composing existing canonical gates; archived specs (incl. Phase-1/2A/3A `0002`–`0035`)
      are cited as history/structural precedent only; `EMERGE-OBS` stays observer-only.
- [ ] No doctrine weakens an upstream tier; crate dependency direction is preserved; anti-contamination
      and no-protagonist-gravity properties hold.
- [ ] Every external claim is cited; every file named in §2 and in the seam inventory exists at commit
      `fd5ae94ff3225d2f989262b95ed8272945861516`.
</content>
</invoke>
