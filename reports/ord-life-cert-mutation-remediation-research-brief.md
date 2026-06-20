# ORD-LIFE-CERT mutation-remediation + replacement certification spec — deep-research brief (paste into ChatGPT-Pro)

> **You are Session 2.** Produce the deliverable directly as a downloadable markdown
> document. Do **not** interview, do **not** ask clarifying questions — the requirements
> below are final. If a genuine contradiction makes a requirement impossible, state it in
> the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-20_2befc07.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust (event-sourced
kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers
govern later ones (if execution conflicts with architecture or foundation, execution is wrong; if
an implementation is more convenient than the accepted gates, the implementation is wrong).
**Fetch every file from commit `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd` (short `2befc07`)** —
the manifest reflects exactly that tree. Do not adopt a different "commit of record" from any file
you read: commit hashes that appear *inside* archived specs/acceptance artifacts/triage registers
(e.g. `98dc042`, `726b2a1`, `92ba47f`, `7a17447d`) are that artifact's own audit/authoring
provenance, **not** this baseline. A manifest is path inventory only.

**This brief continues an active certification campaign — it is the *second* brief on the
ORD-LIFE-CERT gate line.** The repo runs a phase-certification ladder (`docs/2-execution/03`):
`P0-DOC → P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT → FIRST-PROOF-CERT → PHASE-4-ENTRY →
SECOND-PROOF-ENTRY`. Every gate so far has been an **audit spec → mutation-remediation/replacement
spec** pair:

- `P0-CERT`: `0036` audit (→ `P0-CERT scoped remediation`) then `0037` remediation (→ `P0-CERT passed`).
- `SPINE-CERT`: `0038` audit (→ scoped remediation) then `0039` remediation (→ `SPINE-CERT passed`).
- `EPI-CERT`: `0040` audit (→ scoped remediation) then `0041` remediation (→ `EPI-CERT passed`).
- `ORD-LIFE-CERT`: `0042` audit (→ **scoped remediation**, the standing state) — and the
  remediation half is **not yet authored**. That is your deliverable.

**Your lineage predecessor (this brief is a *delta* on it — do not re-commission it):** the
**ORD-LIFE-CERT audit spec**, commissioned by `reports/ord-life-cert-research-brief.md` and
delivered as
`archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md`.
Its acceptance artifact
(`reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md`)
and mutation triage register
(`reports/0042_ord_life_cert_mutation_triage_register.md`) render the verdict **`ORD-LIFE-CERT
scoped remediation`, not `ORD-LIFE-CERT passed`**, because (a) the configured
`cargo mutants --workspace --no-shuffle` lane **did not complete** (a PTY/tool-failure: the wrapper
remained active after no cargo-mutants process was visible; a deterministic `-j 8` rerun also failed
to produce a complete summary), and (b) before interruption it exposed **three actionable missed
mutants in `crates/tracewake-core/src/need_accounting.rs`** mapped to `ORD-LIFE-01`, `ORD-LIFE-08`,
and `ORD-LIFE-12`. The audit additively expanded the standing `.cargo/mutants.toml` perimeter to a
60-file / 2877-mutant census. The ledger's "Next known execution move"
(`docs/4-specs/SPEC_LEDGER.md`) states verbatim that this artifact may be cited only as
`ORD-LIFE-CERT scoped remediation`, that no later spec may cite `ORD-LIFE-CERT passed` or proceed to
`FIRST-PROOF-CERT`/Phase-4 until "a separate ORD-LIFE-CERT mutation remediation/replacement spec
resolves the floor," and that **"the next known execution move is a separately numbered
ORD-LIFE-CERT mutation remediation/replacement spec and evidence package."**

**The structural model you must mirror** (cross-line structural precedents — read in full as *shape*
models in §2, **not** delta seeds): the three prior remediation specs and their acceptance
artifacts, **`0041` (EPI-CERT) being the freshest and most-similar** — single-gate mutation
remediation decomposed into a standing-perimeter confirmation, clean-baseline preflight,
development-vs-certifying-run discipline, sharding/timeout/completion-proof rules, a survivor-identity
reconciliation + triage-register schema, kill-with-behavior/provenance coverage doctrine, live
seam re-proof, and a replacement acceptance artifact that supersedes the audit artifact. Your spec is
the **ordinary-life analogue** of `0041`/`0039`/`0037`. The ORD-LIFE-CERT *audit* spec is **already
done** (`0042`) and is **not** re-authored.

---

## 2. Read in full (authority order)

Read these before producing. The user's standing instruction is to read **all** of
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and
`docs/4-specs/*`. Within each tier, entries marked **[primary]** are load-bearing for the
ORD-LIFE-CERT mutation-remediation spec; the remainder are *boundary-awareness* (read to know what is
**out** of scope and must not be re-audited or "corrected" here — SPINE-CERT and EPI-CERT seams are
already certified; FIRST-PROOF-CERT / Phase-4 / later-gate material is downstream).

**Foundation (`docs/0-foundation/`)**
- `docs/README.md` — **[primary]** authority layering and the "execution is wrong if it conflicts with
  architecture/foundation" rule.
- `00_FOUNDATION_INDEX.md` — index/boundary map (boundary-awareness).
- `02_CONSTITUTIONAL_INVARIANTS.md` — **[primary]** `INV-001…INV-112`; every property the remediation
  re-proves must satisfy these; the spec amends none.
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **[primary]** ordinary-life need accounting /
  intentions / routines / stuck state must be event-sourced and replay-reconstructable; the
  `need_accounting.rs` survivors live in the replay/accounting-derivation layer.
- `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — **[primary]** foundation contract for needs,
  intentions, routines, planning — the seams whose kill-witnesses must observe certified consequences.
- `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — **[primary]** foundation contract for
  actions/affordances/durations the ordinary-life perimeter covers.
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **[primary]** first-playable acceptance doctrine
  (the no-human ordinary day; ORD-LIFE-CERT feeds FIRST-PROOF-CERT downstream).
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **[primary]** actor-known cognition
  transaction + truth firewall; kill-witnesses may not source expectations from ground truth.
- `01, 04, 07, 08, 09, 10, 11, 13` — boundary-awareness (charter; claims/beliefs/memory — EPI;
  institutions; TUI/possession — EPI; no-scripting/seeds; LOD/scale; LLM/speech; research notes). Read
  to bound scope; not remediation targets.

**Architecture (`docs/1-architecture/`)**
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **[primary]** canonical gate/review-artifact
  composition; ORD-LIFE-CERT is a phase-certification *label* composing gates defined here, minting no
  new gate code.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — **[primary]** the one-way crate
  dependency rule (`core ← content ← tui`) no remediation delta may invert.
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **[primary]** projection-rebuild and
  determinism contract: ordinary-life metrics/diagnostics are replay-derived projections, never truth
  writers — the consequence a `need_accounting` kill-witness should observe.
- `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — **[primary]** the
  architecture contract for the actor-known decision transaction (needs → candidates → method/HTN
  selection → local planning → sealed proposal) the surviving seams sit within.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **[primary]** what a gate artifact
  must contain; typed-diagnostic and review-artifact obligations the replacement artifact instantiates.
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **[primary]** holder-known/actor-known
  sealed contexts + provenance sufficiency/freshness (the substrate the decision transaction consumes).
- `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — **[primary]** ordinary-life spatial
  model / affordances / no-teleport the duration-interval accounting interacts with.
- `04` — read **only** for the no-direct-dispatch boundary (scheduler may not dispatch ordinary actions
  directly from needs/routines); `06, 07, 08, 10, 11, 12, 14` — boundary-awareness (beliefs/observation
  — EPI; speech/LLM; institutions; possession/TUI — EPI; incidents/leads; LOD/prehistory; forbidden
  misreads). Read to bound scope; not remediation targets.

**Execution (`docs/2-execution/`)**
- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — **[primary]** execution-tier authority and the canonical gate
  index (ORD-LIFE-CERT consumes canonical gate evidence; it is not a new gate).
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — **[primary]** the post-0008
  baseline, code-audit boundary, and three admissibility postures
  (`passed` / `scoped remediation` / `not applicable`).
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — **[primary]** first-proof scope
  and acceptance-contract cross-references the ORD-LIFE line feeds into (FIRST-PROOF-CERT is gate 5,
  downstream and blocked).
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **[primary]** defines `ORD-LIFE-CERT`
  (gate 4), gate-evidence requirements, **gate-failure handling (the named responsible-layer list)**,
  the valid future-spec postures (declare exactly one: this is `Remediation`), the EMERGE-OBS
  observer-only rule, the Temporal-Cascade placement (consolidated temporal bundle is a FIRST-PROOF-CERT
  obligation), and the "Phase 3A landed = evidence for ORD-LIFE-CERT, not certification" mapping.
- `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **[primary]** THE ORD-LIFE-CERT execution
  proof doc: the 10 numbered `ORD-LIFE-CERT`-pass conditions, the required adversarial-fixture-families
  table, the single-charge-per-actor/per-need/per-tick rule, the single-owner derived-accounting seam,
  intention lifecycle, routine-template requirements, routine-temporal-premise mechanism, no-human
  proof evidence list, canonical recovery, and behavioral-progress / cross-tick stuck-detection
  definitions. The seams you re-prove and the `ORD-LIFE-01…NN` cross-references trace back here.
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **[primary]** the anti-contamination
  gates a kill-witness must respect (no fact from prose; no decision from unobserved ground truth;
  validation truth may not propose; no teleport to true food).
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — **[primary]** golden-fixture/replay
  acceptance obligations; the positive + adversarial fixture families a behavior-witness travels through.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **[primary]** the **evidence-honesty
  rule**, diagnostics-by-layer, routine-failure-diagnostic classification, review-artifact obligations,
  EMERGE-OBS observer-only discipline, and the **mutation / phase-entry evidence rule** (a tool failure
  or incomplete run is not a pass) — directly governs how the incomplete-lane blocker is resolved.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — **[primary]** read for the
  no-direct-dispatch boundary the ordinary-life seam must respect (SPINE-CERT certified the pipeline
  itself; do **not** re-audit it).
- `08, 11, 12, 13` — boundary-awareness (data authoring/schema — read only where the ordinary-life seam
  consumes authored seed knowledge / fixtures; institutions/Phase-4; deferred second-proof; research
  notes). Read to bound scope; not remediation targets.
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — boundary-awareness (EPI-CERT execution
  proof — already certified). Consume possession-parity / debug-quarantine guarantees where an
  ordinary-life fixture needs them; do not re-audit the EPI seams.

**Reference (`docs/3-reference/`)**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **[primary]** review-checklist questions.
- `01_DESIGN_RISK_REGISTER.md` — **[primary]** standing risks incl. the **anti-Goodhart rule** (killing
  only the named subset, improving a score, or treating the historical identities as the denominator of
  convenience cannot certify the line) and the contamination / silent-starvation / no-progress risks.
- `02_GLOSSARY.md` — **[primary]** canonical terminology (`actor-known`, `holder-known context`,
  `no-human proof`, `routine`, `intention`, `candidate goal`, `behavioral progress`, `stuck
  diagnostic`, `EMERGE-OBS`, etc.) the spec must use exactly.

**Specs (`docs/4-specs/`)**
- `SPEC_LEDGER.md` — **[primary]** live ledger: authority posture, source discipline, the archived-spec
  status table, and the "Next known execution move" (records `ORD-LIFE-CERT scoped remediation`, names
  the separately-numbered remediation/replacement spec as the next move, and forbids citing
  `ORD-LIFE-CERT passed` until it lands).
- `README.md` — **[primary]** rules for future specs (declare authority posture; declare exactly one
  admissibility posture; gate codes as cross-references only; no files for symmetry).
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — **[primary]** the review-artifact template (evidence-status /
  fingerprint-scope / behavior-witness / replay-provenance / sampling / pending-historical /
  certification-use / staged-abstraction fields) the replacement ORD-LIFE-CERT acceptance artifact must
  instantiate.
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — boundary-awareness (live
  first-proof village/fixture ontology; informs fixture realism; not a remediation target).

**Lineage predecessor — the ORD-LIFE-CERT audit half (read in full; this is the *delta seed*)**
- `archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md`
  — **[primary]** the audit contract: the `ORD-LIFE-01…NN` numbered audit points, the §7.2 mutation
  perimeter, the required positive + adversarial fixtures, and the acceptance-artifact contract. These
  are the seams your remediation **re-proves live** at the final implementation commit; do not redesign
  them.
- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md`
  — **[primary]** the `ORD-LIFE-CERT scoped remediation` acceptance artifact your replacement artifact
  must supersede (study its scope/verdict shape and which seams it already exercised as evidence).
- `reports/0042_ord_life_cert_mutation_triage_register.md` — **[primary]** the **exact survivor floor +
  tool-failure record**: the three `need_accounting.rs` identities (`88:25` `< → <=` in
  `DurationInterval::contains_tick`; `106:13` `&& → ||` in `duration_intervals`; `109:45` `== → !=` in
  `duration_intervals`), their ORD-LIFE cross-refs and responsible layers, the two incomplete configured
  lanes (`--no-shuffle` and `-j 8`), the expanded 60-file / 2877-mutant census, and the
  evidence-artifact + fingerprint format your remediation register reuses and extends.
- `reports/ord-life-cert-research-brief.md` — **[primary]** the brief that commissioned the audit;
  read it for the campaign conventions, the source-discipline language, and the seam/code inventory it
  established (the §5 code-seam list is reusable orientation for the ordinary-life perimeter).

**Cross-line structural precedents — the remediation-spec shape you mirror (read in full; *not* delta seeds)**
- `archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` —
  **[primary]** the **freshest, closest** remediation template: standing-perimeter confirmation,
  authoring-baseline-vs-final-commit discipline, clean-baseline preflight, development-vs-certifying-run
  rules, sharding/timeout/completion-proof, survivor-identity reconciliation, triage-register schema,
  kill-with-behavior/provenance coverage, CI in-diff trigger correction, and required-end-state
  checklist. Clone its structure; retarget every EPI seam to the ordinary-life seam.
- `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` and
  `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` —
  **[primary]** the earlier remediation templates (cross-check the established remediation anatomy and
  source-discipline section; `0039` is the perimeter-widening case, `0037` the original pair).
- `archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md` and
  `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` —
  **[primary]** the **passing** replacement acceptance artifacts (the shape the eventual ORD-LIFE-CERT
  `passed` artifact targets — what the implementing session records to flip the line).
- `archive/reports/0040_epi_cert_mutation_triage_register.md` — **[primary]** the triage-register format
  the ordinary-life register reuses; `archive/reports/0040_epi_cert_mutation_final_missed.txt` — the
  canonical missed-identity manifest format; and
  `archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`
  — a *failed-floor* (scoped-remediation) acceptance artifact, the verdict shape your spec routes to if
  the final run still leaves survivors.
- `archive/reports/epi-cert-mutation-remediation-research-brief.md` — **[primary]** the **sibling-gate
  remediation brief precedent** (the brief that commissioned `0041`); reuse its framing where it fits.

**Configuration & code seams (verify at `2befc07`; expand from exploration)**
- `.cargo/mutants.toml` — **[primary]** the standing certification/lock-layer mutation perimeter
  (additively expanded by `0042` to include `need_accounting.rs`, `actions/registry.rs`,
  `actions/defs/need_events.rs`, `actions/defs/wait.rs`, `actions/defs/continue_routine.rs`,
  `actions/defs/movement.rs`; 60 files / 2877 mutants). The denominator of record — confirm, do not
  silently shrink.
- `.cargo/mutants-baseline-misses.txt` — **[primary]** baseline-miss file; the survivors must **not** be
  parked here as a convenience.
- `.github/workflows/ci.yml` — **[primary]** the scheduled + in-diff mutation jobs; confirm the
  scheduled lane runs the checked-in config and the in-diff trigger covers the ordinary-life perimeter
  files (the `0041` lineage required a minimal in-diff-trigger correction — check for the analogue here).
- `crates/tracewake-core/src/need_accounting.rs` — **[primary]** the survivor-bearing file
  (`DurationInterval::contains_tick`, `duration_intervals`, `TickRegime`,
  `classify_actor_tick_regimes_with_start`, `terminal_ticks_by_start`, etc.). Plus the surrounding
  ordinary-life seams the audit (`0042` §7.2) and `reports/ord-life-cert-research-brief.md` §5 name —
  `agent/need.rs`, `actions/defs/need_events.rs`, `agent/{generation,candidate,routine,methods,htn,
  intention,planner,decision,trace,transaction,actor_known,no_human_surface,perception}.rs`,
  `actions/defs/{sleep,work,eat,wait,continue_routine,movement}.rs`, `actions/{pipeline,proposal,
  registry}.rs`, `scheduler.rs`, `projections.rs`, `events/*`, `replay/*`, `checksum.rs` — and the
  ordinary-life test harnesses under `crates/tracewake-core/tests/` (`no_human_capstone.rs`,
  `acceptance_gates.rs`, `event_schema_replay_gates.rs`, `golden_scenarios.rs`, `hidden_truth_gates.rs`,
  `generative_lock.rs`, `support/generative.rs`) plus `crates/tracewake-content/tests/
  golden_fixtures_run.rs`. Confirm each at `2befc07` and name the real, verified set in the spec.

(Also present for orientation, not load-bearing: `archive/specs/0005…0025` Phase-3A ordinary-life
hardening series — *historical evidence only*, never live certification, per the ledger and
`docs/2-execution/03`. The live doctrine above governs.)

---

## 3. Settled intentions (final — do not reopen)

1. **The determination is pre-settled and doctrine-forced.** `P0-CERT passed` (0037 artifact),
   `SPINE-CERT passed` (0039 artifact), and `EPI-CERT passed` (0041 artifact) are the current
   admissibility state; `ORD-LIFE-CERT` is `scoped remediation` (0042 artifact). The gate ladder
   (`docs/2-execution/03`), the gate-failure-handling rule (a failed gate must produce a remediation
   spec naming the failing layer; it may not be relabeled a phase exception), and the ledger's "Next
   known execution move" make a **separately-numbered ORD-LIFE-CERT mutation-remediation + replacement
   certification spec** the single next admissible move. **Open the deliverable with a short,
   evidence-based confirmation of this determination** (cite the gate ladder + ledger + the three
   passed predecessor gates + the 0042 scoped-remediation verdict). Do **not** survey alternative "next
   features", and do **not** propose gameplay expansion — the ladder explicitly invalidates any
   Phase-4/expansion/feature spec attempted before `ORD-LIFE-CERT` and `FIRST-PROOF-CERT` pass.

2. **Form: a non-executable mutation-remediation + replacement certification spec** in the exact
   `0037`/`0039`/`0041` lineage (`0041` the freshest structural model). You (Session 2) **cannot** run
   `cargo test`, `cargo-mutants`, or replay — so the spec specifies *what the implementing session must
   inspect, change, run, prove, record, review, and package*; it renders **no** pass/fail verdict, does
   **not** assert any survivor is already killed or any run has completed, and **fabricates no
   test/mutation/replay results**. Carry the campaign's source-discipline preamble (the "I am not
   verifying this commit is current `main`; I use the supplied commit as the target of record and fetch
   files only by exact commit URL" statement, adapted to `2befc07`).

3. **Center of gravity is co-equal: "kill, COMPLETE, reconcile, triage, re-prove."** Unlike the `0041`
   case (a *completed* configured run with a clean survivor floor), the 0042 blocker is **dominantly a
   mutation-infrastructure tool-failure**: the configured `cargo mutants --workspace --no-shuffle` lane
   (and its `-j 8` rerun) **never completed**. The spec must therefore make **"get the standing
   configured mutation lane to actually run to completion — diagnose and resolve the PTY/tool-failure
   root cause, retain outputs even on failure, and rigorously distinguish a tool failure / incomplete
   run / timeout from a genuine `caught`/`missed` result"** a **first-class remediation objective,
   co-equal** with (a) killing the three known `need_accounting.rs` survivors and (b) triaging every
   additional survivor the complete run exposes. A complete configured run over the standing perimeter
   is a precondition of acceptance; an incomplete or tool-failed lane can never render
   `ORD-LIFE-CERT passed` (per `docs/2-execution/10`'s evidence-honesty / mutation-evidence rule). Pull
   the completion-proof, sharding, and timeout-disposition machinery forward from `0041` §4.6 and make
   the tool-failure resolution explicit rather than implicit.

4. **The survivor floor is a seed, not the finish line.** The three `need_accounting.rs` identities
   (`88:25` `< → <=` in `DurationInterval::contains_tick`; `106:13` `&& → ||` in `duration_intervals`;
   `109:45` `== → !=` in `duration_intervals`; all cross-referenced `ORD-LIFE-01`/`-08`/`-12`) are
   mandatory reconciliation work, **but do not define acceptance**. Acceptance requires a *complete*
   configured run over the standing perimeter (the 0042-expanded `.cargo/mutants.toml`, ~60 files /
   ~2877 mutants — re-confirm via `--list-files`/`--list` at the final commit, explaining any delta from
   that census by final-tree source changes). Every additional survivor, timeout-turned-miss, or
   relocated identity joins the same remediation obligation. A campaign that kills only the named three,
   improves a mutation score, or parks the rest as baseline misses is anti-Goodhart behavior and cannot
   certify the line (`docs/3-reference/01`).

5. **Kill-with-behavior/provenance coverage is the default** (mirror `0041` §4.9). Each killing witness
   must: pass against the unmutated final implementation; fail when the mutant is active; observe a
   public or certified *ordinary-life* consequence (e.g. a wrong need-charge / duration-interval
   ownership / replay-derived metric), **not** restate the mutated expression or assert a helper literal;
   travel the production seam (no test-only bypass); carry replay/provenance ancestry where the behavior
   is event-derived/projected; include a negative/contamination control where truth/debug/prose/direct
   state could shortcut it; and name the responsible failing layer. For the `need_accounting.rs`
   survivors specifically, the witness should observe the **single-charge-per-actor/per-need/per-tick**
   and **duration-terminal/interval-ownership** consequences via the replay-derived accounting
   projection, not via `DurationInterval`/`duration_intervals` return values asserted in isolation.

6. **Re-prove the ORD-LIFE seam contract live + instantiate the acceptance artifact.** The replacement
   package must re-prove the `0042` `ORD-LIFE-01…NN` audit points at the exact final implementation
   commit (clean preflight: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
   warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace --locked`, plus
   the named ordinary-life gate suites), and produce a replacement acceptance artifact conforming to
   `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (evidence-status / fingerprint-scope /
   behavior-witness / replay-provenance / sampling / pending-historical / certification-use /
   staged-abstraction fields) that renders `ORD-LIFE-CERT passed` and **explicitly supersedes** the 0042
   acceptance artifact. Name the observer-only `EMERGE-OBS` artifact as an evidence-package member,
   never a gate or pass/fail threshold.

7. **Scope is the remediation spec only.** Do **not** re-author or re-derive the `0042` audit contract
   (cite it as the seam list to re-prove). Do **not** advance `FIRST-PROOF-CERT`, Phase-4, or any
   expansion. The routine **temporal-premise mechanism** was audited under `0042`; the consolidated
   temporal evidence bundle that `docs/2-execution/03` assigns to `FIRST-PROOF-CERT` remains a
   downstream obligation — name it in tolerated deferrals as a routed dependency, not an ORD-LIFE-CERT
   pass/fail line. SPINE-CERT and EPI-CERT seams are already certified and are **not** re-audited
   (shared event/replay/pipeline surfaces may be exercised as *ancestry* for an ordinary-life behavior
   witness, but that is continuity evidence, not a second audit).

8. **Numbering & placement.** Filename: `specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`
   (`<DESCRIPTIVE_NAME>` may be refined in the established ALL-CAPS-underscored style). The staging
   series in `archive/specs/` is contiguous `0002 → 0042` with `specs/` currently empty, and recent git
   history shows each spec archived by a plain rename with no renumber/restart; the lineage predecessor
   (the ORD-LIFE-CERT audit) is `0042`, so this remediation continues the sequence at **`0043`** (the
   live ledger's `0001`/`0003` live-tier numbering does not block continuing the staging sequence).
   `assumption:` stage under `specs/` (not directly in `docs/4-specs/`), per the repo's
   stage-then-archive convention; it is archived to `archive/specs/0043_…` on accepted closeout and is
   **not** promoted into live `docs/4-specs/`. The user may relocate if desired. Header block must
   mirror the `0041` header: staging path, archive-on-closeout path, target repository, **target commit
   `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd`**, spec series, status, **Future-spec posture:
   `Remediation`**, **Admissibility posture: `ORD-LIFE-CERT scoped remediation`**, **Consumed predecessor
   gates** (`P0-CERT passed` via 0037, `SPINE-CERT passed` via 0039, `EPI-CERT passed` via 0041), a
   **Certification-line effect** line (successful execution must publish a replacement acceptance artifact
   rendering `ORD-LIFE-CERT passed` that supersedes the 0042 artifact), and a one-line **non-executable**
   statement.

9. **Source discipline (carry verbatim from the ledger).** Manifests are path inventory only;
   branch/default-branch/code-search/repo-metadata are not proof of target-commit content; archived
   specs/tickets/reports are cited as **history or structural precedent only**, never as live
   certification; commit strings embedded in archived artifacts (`98dc042`, `726b2a1`, `92ba47f`,
   `7a17447d`, etc.) are those artifacts' own provenance, not this baseline. The spec may **not** amend
   invariants, redefine/weaken gate semantics, or **mint a new gate code, status enum, obligation code,
   invariant ID, or doctrine-level finding ID** — `ORD-LIFE-CERT` is a phase-certification *label* that
   composes the canonical gates/review-artifacts from architecture `00` and execution `00`. Survivors
   remain cargo-mutants `path:line:column:operator` identities. Use `actor-known` for the actor case and
   `holder-known context` as the system-wide term.

10. **`assumption:`** the spec **may** include a single, clearly-labeled **"preliminary, non-certifying
    static survey"** of what reading the code at `2befc07` suggests about the three survivors'
    reachability and the perimeter's coverage — explicitly marked *informative, not certification*.
    Default is to include it; omit only if it would risk reading as a verdict.

---

## 4. The task

Author the **`ORD-LIFE-CERT` mutation-remediation + replacement certification spec**
(`specs/0043_*`): the single next admissible spec on the Tracewake certification ladder. It is a
*new-spec*, *non-executable*, `Remediation`-posture deliverable — an evidence-complete plan the
implementing session executes to (a) **resolve the mutation-infrastructure tool-failure so the standing
configured `cargo mutants` lane runs to completion**, (b) **kill the three known `need_accounting.rs`
survivors and triage every survivor the complete run exposes**, (c) **re-prove the `0042`
`ORD-LIFE-01…NN` ordinary-life seam contract live** at the exact final implementation commit, and (d)
**publish a replacement acceptance artifact that renders `ORD-LIFE-CERT passed` and supersedes the 0042
artifact** — all against live foundation/architecture/execution doctrine at commit `2befc07`. The spec
must be detailed enough that a Tracewake implementing session executes it without further design.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular the actual
`need_accounting.rs` seam, the surrounding ordinary-life modules, the standing `.cargo/mutants.toml`
perimeter, and the CI mutation jobs at `2befc07`. Verify the survivor identities still map to live
source (line numbers may have shifted; map by enclosing symbol + operator, not by line), confirm the
perimeter census, and name the real, verified file/seam set in the spec.

Research online as deeply as needed — similar implementations and prior art — wherever it sharpens the
remediation design. In particular: **mutation testing of decision/accounting kernels** (scoping a
guarded-layer perimeter, triaging survivors, distinguishing equivalent mutants, repairing misses
through externally observable behavior rather than tests over private implementation details — see the
cargo-mutants guidance on `--list`/`--list-files` denominators, config defaults, sharding,
baseline/timeout semantics, and writing tests that kill via observable behavior); **property-based and
metamorphic testing** for conservation/single-charge invariants (need accounting, duration-interval
ownership) and replay determinism; **flakiness / hang diagnosis in long-running test or mutation
harnesses under PTY wrappers** (how to make a process-supervision lane terminate deterministically,
detect a stalled child, and emit a distinguishable tool-failure vs. timeout vs. miss signal); and
**CI design for expensive mutation lanes** (scheduled full runs vs. fast in-diff change detectors,
retaining artifacts on failure, sharding a single denominator reproducibly). Cite sources for any
external claim that shapes a decision. The *deep* research is yours to perform; the determination of
*which* spec is **not** — that is settled in §3.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every property the
  remediation re-proves must satisfy it; a genuine divergence would require amending an invariant first
  — never design against it. The spec amends nothing.
- Authority order: if execution conflicts with architecture or foundation, execution is wrong; if an
  implementation is more convenient than the accepted gates, the implementation is wrong; if a test
  kills a mutant by asserting a mutated helper's literal return instead of a certified ordinary-life
  consequence, the test is wrong; if an artifact promotes archived evidence into a live pass, the
  artifact is invalid.
- No backwards-compatibility shims or alias paths in new work.
- Anti-contamination: no simulation fact may be born from prose; preserve event-sourced causality,
  subjective epistemics, ordinary agents, possession parity, fallible institutions, validation/replay.
  For ordinary life: a hungry actor must not teleport to true food, starve silently, or wait forever
  without cause; candidate generation and method selection consume **actor-known inputs only**; the
  scheduler may not dispatch ordinary actions directly from needs/routines; validation truth may not
  propose; need/duration deltas are charged exactly once by the single owning seam; planner traces stay
  non-diegetic (debug-visible, never fed back as actor input); possession does not reset
  need/routine/intention/memory/actor-known state.
- The crate dependency direction (`core ← content ← tui`) must never invert; no test-only production
  dependency.
- Mint no new gate code, status enum, obligation code, invariant ID, or finding ID. Cross-reference
  existing ones. `EMERGE-OBS` stays observer-only.

---

## 7. Deliverable specification

Produce **one** downloadable markdown document:

- **`specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`** — **new**
  numbered staging spec (not a replacement of any existing file). This is a *new-spec* deliverable:
  there is **no** paired `-research-report.md`. The numbering/staging-series rules in §3.8 (staging
  series → `0043`) apply; do not treat this as a `reports/` analysis report.

Required structure (follow the `0041`/`0039`/`0037` remediation anatomy, retargeted to the
ordinary-life substrate):

1. **Header block** — per §3.8 (staging + archive-on-closeout paths, target repo, **target commit
   `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd`**, spec series, status, `Future-spec posture:
   Remediation`, `Admissibility posture: ORD-LIFE-CERT scoped remediation`, consumed predecessor gates,
   certification-line effect, non-executable statement) + the source-discipline preamble.
2. **Status, determination, source discipline, freshness, admissibility** — the single posture; the
   certification-line state (the line stays `scoped remediation` until the replacement artifact passes;
   this spec does not flip it); the cited determination confirmation (next admissible move); exact-commit
   source discipline (`2befc07`; embedded hashes are not this baseline); authoring-baseline-vs-final-
   implementation-commit discipline (every delta enumerated; certifying commands run against one exact
   final commit; both SHAs reported separately; neither presented as current-`main` verification); and
   the admissibility lock (no later spec may cite `ORD-LIFE-CERT passed`; FIRST-PROOF-CERT/Phase-4/
   second-proof remain locked).
3. **Authority & dependency declarations** — controlling doctrine order; the primary ordinary-life
   sources; foundation/architecture/execution/reference/live-spec dependencies (the §2 [primary] list);
   crate direction and implementation boundary; the primary code/test/config seams.
4. **Problem statement** — the 0042 verdict and floor: the three `need_accounting.rs` survivors (with
   their ORD-LIFE cross-refs and responsible layers) **and** the two incomplete/tool-failed configured
   lanes; the standing perimeter census (60 files / 2877 mutants); the **central divergence from
   0041** (0041 remediated a *completed-run* survivor floor; 0043 must additionally make the configured
   lane *complete at all*); a static perimeter confirmation at `2befc07` (the perimeter covers
   `need_accounting.rs`; the baseline-miss file's state); and any CI in-diff-trigger durability gap for
   the ordinary-life perimeter files (the 0041-analogue minimal correction, if present).
5. **Remediation approach** — the required end state (a numbered checklist mirroring `0041` §4.1,
   adding the **tool-failure-resolution / complete-configured-run** items as first-class); maintain &
   re-confirm the standing configured perimeter (`--list-files`/`--list` capture; no `--no-config`/`-f`/
   `--exclude`/`--in-diff`/`--iterate` as the final denominator); **mutation-lane completion &
   tool-failure resolution** (diagnose the PTY/stall root cause; ensure deterministic termination;
   distinguish tool-failure vs. timeout vs. miss; retain outputs on failure; a complete run is a
   precondition of acceptance); CI convergence & baseline-miss discipline (the survivors must not be
   parked as baseline misses); clean-baseline + named-preflight; development-vs-certifying-run
   discipline; sharding/timeouts/completion-proof; survivor-identity reconciliation (map historical
   identities through refactors by symbol+operator); the **triage-register schema** (reuse the
   0040/0042 format: historical identity, final identity, tool outcome, ORD-LIFE cross-ref, responsible
   layer, certified reachability, test family, behavior witness, replay/provenance ancestry, negative/
   contamination control, certification disposition, evidence references, review signoff); and the
   **kill-with-behavior/provenance coverage** doctrine with ordinary-life-specific insufficient-example
   list (per §3.5).
6. **Failure handling** — diagnostics named by the `docs/2-execution/03` responsible-layer list (incl.
   need accounting / candidate generation / intention lifecycle / method selection / local planning /
   proposal construction / scheduler ordering / action validation / event application / projection-
   replay / view-model rendering / debug quarantine / tests-fixtures / documentation status, **plus the
   mutation-infrastructure / configured-perimeter-completion layer** the tool-failure maps to); a failed
   seam or incomplete run yields `ORD-LIFE-CERT scoped remediation` routed to a named follow-up, not a
   relabeled phase exception.
7. **Live seam re-proof** — re-prove the `0042` `ORD-LIFE-01…NN` audit points at the exact final
   implementation commit (clean preflight + the named ordinary-life gate suites); a coverage table
   mapping the re-proof onto the 10 numbered `ORD-LIFE-CERT`-pass conditions and required adversarial
   fixture families in `docs/2-execution/06` so completeness is auditable.
8. **Acceptance-artifact contract** — instantiate `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:
   exactly what the implementing session records to render `ORD-LIFE-CERT passed` (evidence-status,
   fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical,
   certification-use, staged-abstraction fields); the replacement artifact must explicitly supersede the
   0042 artifact; name `EMERGE-OBS` as an evidence-package member (never a gate); state that this spec
   specifies the audit/remediation and renders no verdict, and that an incomplete run or surviving floor
   yields `ORD-LIFE-CERT scoped remediation` routed to a named follow-up.
9. **(Optional) Preliminary static survey** — clearly labeled "preliminary, not certification" (per
   §3.10).
10. **Tolerated deferrals** — explicitly out of scope and routed forward: the consolidated temporal
    evidence bundle to `FIRST-PROOF-CERT`; the success-recovery ordinary-life variant per doc `06`'s
    canonical-recovery section; institutions / wrong-suspicion / records to `PHASE-4-ENTRY`; notices /
    travel / regional scale / LOD to `SECOND-PROOF-ENTRY`; and a statement that SPINE-CERT and EPI-CERT
    seams are already certified and are **not** re-audited here.
11. **(Optional) Appendix** — exact-URL evidence ledger for the files fetched at `2befc07` (mirror the
    `0041` appendix), and references for every external claim.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec opens with a cited confirmation that the ORD-LIFE-CERT mutation-remediation/replacement
      spec is the next admissible move (predecessor gates `P0-CERT`, `SPINE-CERT`, `EPI-CERT` passed;
      `ORD-LIFE-CERT` is `scoped remediation`); it proposes no feature/expansion work.
- [ ] **Completing the configured mutation lane (resolving the PTY/tool-failure root cause) is a
      first-class, co-equal objective** with killing the three `need_accounting.rs` survivors; the spec
      makes a complete configured run a precondition of acceptance and distinguishes tool-failure /
      incomplete / timeout from `caught`/`missed`.
- [ ] The three known survivors (`88:25`, `106:13`, `109:45` in `need_accounting.rs`, → `ORD-LIFE-01`/
      `-08`/`-12`) are tracked as a seed work-list, mapped to live source by symbol+operator, and the
      spec requires triaging **every** additional survivor the complete run exposes; the floor is not
      treated as the finish line.
- [ ] Kill-witness doctrine is "kill-with-behavior/provenance" — observing certified ordinary-life
      consequences (single-charge / duration-interval ownership / replay-derived metrics), not helper
      literals; with negative/contamination controls and named responsible layers.
- [ ] The spec re-proves the `0042` `ORD-LIFE-01…NN` seam contract live at one exact final commit and
      includes a coverage table onto the 10 `ORD-LIFE-CERT`-pass conditions + required fixture families
      in `docs/2-execution/06`.
- [ ] The deliverable is non-executable: it specifies the remediation/acceptance contract, renders **no**
      pass/fail verdict, and fabricates no test/mutation/replay results. Any static survey is labeled
      "preliminary, not certification".
- [ ] Scope is the remediation spec only; the `0042` audit is cited (as the seam list to re-prove), not
      re-authored; SPINE-CERT/EPI-CERT seams are not re-audited; the temporal bundle is routed to
      FIRST-PROOF-CERT in tolerated deferrals.
- [ ] Filename is `specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`;
      header carries posture `Remediation`, admissibility `ORD-LIFE-CERT scoped remediation`, consumed
      gates citing the 0037/0039/0041 artifacts, certification-line effect (supersede the 0042 artifact),
      and target commit `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd`.
- [ ] The acceptance-artifact contract instantiates `docs/4-specs/0003`'s fields and explicitly
      supersedes the 0042 acceptance artifact on a passing outcome.
- [ ] No new gate code / invariant ID / status enum / finding ID is minted; `ORD-LIFE-CERT` is treated
      as a label composing existing canonical gates; archived specs (incl. Phase-3A `0005`–`0025`) are
      cited as history/structural precedent only; `EMERGE-OBS` stays observer-only.
- [ ] No doctrine weakens an upstream tier; crate dependency direction is preserved; anti-contamination
      properties hold; the survivors are not parked in `.cargo/mutants-baseline-misses.txt`.
- [ ] Every external claim is cited; every file named in §2 and in the seam inventory exists at commit
      `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd`.
