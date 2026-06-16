# Research brief — P0-CERT mutation remediation & replacement certification spec (the next Tracewake spec)

> **You are ChatGPT-Pro, Session 2.** This prompt is self-contained. Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a requirement
> impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-16_c54caff.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first; agents act from
partial belief and every event leaves a replayable trace). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; **earlier tiers
govern later ones.** Fetch every file from commit **`c54caff`** (verified repo `HEAD`) — the
manifest reflects exactly that tree.

**This is a delta successor, not a cold start.** The immediate predecessor brief
`reports/p0-cert-baseline-certification-research-brief.md` commissioned the **P0-CERT baseline
certification spec**; that spec was authored, decomposed into the `0036P0CERPOS0008` ticket series,
implemented, and archived as
`archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md` with the acceptance
artifact `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md`. **0036 did
not pass.** Its capstone verdict is **`P0-CERT scoped remediation`**, because the configured
mutation posture emitted one untriaged surviving mutant and the run was stopped before completion.

The repository's own doctrine now names the single admissible next move. `docs/4-specs/SPEC_LEDGER.md`
("Next known execution move") and the 0036 acceptance artifact both state it plainly: a **scoped
remediation for `0036-MUTATION-REMEDIATION-001`** that either kills the surviving mutant with
behavior/provenance coverage or proves and records why it is equivalent/non-critical, then **reruns
the configured mutation posture and produces a replacement certification artifact** before any later
spec may cite `P0-CERT passed`. The acceptance artifact is explicit (its "Admissibility" section):
*"Until a replacement certification artifact passes, only remediation specs addressing
`0036-MUTATION-REMEDIATION-001` are admissible for this certification line."* Therefore SPINE-CERT,
EPI-CERT, the `0035` route-forward expansion backlog, and every other forward move are **doctrinally
blocked** and cannot be the next spec. Your task is **not** to re-decide what is next — it is to
author the P0-CERT remediation-and-replacement-certification spec itself.

This brief was authored at `c54caff`, one merge after the predecessor brief's baseline `9f16222`.
The intervening diff (`git diff --name-status 9f16222 c54caff`) touched **only** skill files, the
newly-archived 0036 spec/report/tickets, the `SPEC_LEDGER.md` update that records 0036 and the next
move, manifest churn, and the predecessor brief itself. **No `docs/**` doctrine-tier file and no
crate source file changed between the two commits** — so the doctrine read-list and the audit seams
below are identical to what 0036 certified against, and the `projections.rs` mutation finding is
still live at this baseline. (Treat the `9f16222` string that appears throughout the 0036 spec and
acceptance artifact as 0036's own audit-provenance commit; this remediation is pinned to `c54caff`.
Per `SPEC_LEDGER.md` source discipline, a commit hash named inside a spec is audit/spec provenance
only.)

## 2. Read in full (authority order)

Read these before producing. Order is by authority tier. The execution tier and the two 0036 seed
artifacts are the heart of this target.

**Seed artifacts — primary (the failed certification this spec remediates; read these first):**

```
archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md — THE failing artifact: the `P0-CERT scoped remediation` verdict, the mutation transcript row, finding `0036-MUTATION-REMEDIATION-001`, the admissibility lock, and the conforming acceptance-artifact shape your replacement must match and supersede.
archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md — the spec being remediated: the ten-point P0-CERT audit contract, seam inventory, gate composition, and section shape your 0037 spec extends and re-runs to completion.
docs/4-specs/SPEC_LEDGER.md — the live ledger: the 0036 row (verdict `P0-CERT scoped remediation`), the source-discipline rules, and the "Next known execution move" that defines this spec's mandate.
```

**Foundation — primary (the constitution and the properties the surviving mutant threatens):**

```
docs/README.md — authority order and the layering rule (foundation > architecture > execution > reference > specs).
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md — INV-001…INV-NNN; the actor-known/provenance invariants the `actor_known_local_actors_for_context` seam upholds (the audit cited INV-024, INV-099…INV-102 for this area).
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md — "truth may validate, truth may not plan"; the actor-known surface the mutated projection feeds.
docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md — first-village scope, no-human / replay / TUI gates, canonical regression seeds.
docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md — event authority, replay, randomness; the replay/provenance ancestry the mutant-killing coverage must cite.
docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md — action parity, ordinary-life survival; the no-human ordinary-day surface that exercises the projection.
docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md — possession parity, actor-filtered view models, debug quarantine.
```

**Architecture — primary (subsystem contracts the surviving-mutant coverage measures code against):**

```
docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md — universal conformance questions and the replacement/retirement rule.
docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md — event log, replay, projections, save manifests, random-stream discipline, schema versioning.
docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md — holder-known contexts, provenance packets, context sealing; the contract `actor_known_local_actors_for_context` participates in.
docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md — proposal pipeline, scheduler limits, validation truth, no-direct-dispatch.
docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md — actor decision transaction, needs, intentions, HTN selection, stuck diagnostics.
docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md — possession, embodied TUI, debug-only truth, client boundaries.
docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md — acceptance artifacts, anti-contamination tests, diagnostics, review checklists; the home of the lock-layer/mutation posture.
```

**Execution — primary (the P0-CERT contract, the remediation posture, and the gate-failure rule):**

```
docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md — canonical gate names (P0-CERT, TFW, PIPE, NO-DIRECT, NO-HUMAN, POS-PARITY, REPLAY, FIXTURE, DIAG), observation obligations, label reconciliation, universal execution posture.
docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md — THE P0-CERT definition: the ten-point artifact requirements, the code audit boundary, and admissibility outcomes (`P0-CERT passed` / `scoped remediation` / `not applicable`). The single most load-bearing doctrine file.
docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md — gate order, the "Gate evidence requirements" list, "Gate failure handling" (a failed gate must produce a remediation spec/report naming the failing layer), and the four valid future-spec postures (Remediation is yours).
docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md — the first-proof acceptance contract gates the replacement artifact still composes.
docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md — truth-firewall execution checks and mandatory anti-contamination gates (TFW); the seam class the surviving mutant lives in.
docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md — scheduler/proposal/validation/direct-dispatch audit criteria (PIPE, NO-DIRECT).
docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md — needs/routines/intentions/no-human-day certification (NO-HUMAN); the behavior that must witness the projection.
docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md — epistemic proof, possession parity, view-model filtering, debug quarantine (POS-PARITY, VIEW-DEBUG-SPLIT).
docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md — authoring contracts, schema/provenance validation, no outcome-chain data.
docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md — golden fixture families, adversarial scenarios, deterministic replay acceptance (FIXTURE, REPLAY).
docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md — testing, diagnostics by layer, review evidence, the mutation/lock-layer posture, and the EMERGE-OBS observation obligation (DIAG).
```

**Reference + specs — primary (rules, numbering, source discipline, structural models):**

```
docs/4-specs/README.md — future-spec rules: declare authority posture, declare one admissibility posture, gate codes as cross-references only, holder-known/actor-known terminology, source discipline, no symmetry files.
docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md — the live first-proof ontology/fixture contract; the system the certification line covers.
docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md — the acceptance-artifact template (evidence-status / fingerprint-scope / behavior-witness / replay-provenance / sampling / pending-historical / certification-use / staged-abstraction fields). The replacement certification artifact MUST conform to this.
archive/specs/0025_PHASE_3A_META_WITNESS_EXECUTION_PROOF_PERCEPTION_KILL_SET_PROVENANCE_ENVELOPE_READ_PATH_FAIL_CLOSED_AND_MANIFEST_FINGERPRINT_HONESTY_HARDENING_SPEC.md — structural model: a recent hardening spec with mutation/kill-set discipline; mirror its section shape and rigor.
archive/specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md — structural model: a scoped audit/hardening spec carrying an acceptance artifact; another shape reference.
```

**Reference — boundary-awareness (read to bound scope; NOT conformance targets):**

```
docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md — review guardrails and gate-code lookup posture.
docs/3-reference/01_DESIGN_RISK_REGISTER.md — the post-0008 relapse risk (treating archived 0005–0036 as certification) and the anti-Goodhart watch note the remediation must actively resist.
docs/3-reference/02_GLOSSARY.md — prescriptive terminology control (holder-known, observation, belief, debug truth, source discipline).
```

**Boundary-awareness — OUT of scope (read only to know what this spec must NOT touch or advance into):**

```
docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md — Phase-4 entry lock; blocked behind PHASE-4-ENTRY, downstream of P0-CERT.
docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md — deferral contract for notices/travel/regional/LOD/second-proof.
docs/0-foundation/07,09,10,11,13 and docs/1-architecture/06,07,08,09,11,12,14 — institutions, LLM/speech, LOD/regional scale, no-scripting authoring depth: future-phase or deferred surfaces, not part of the P0-CERT certification line.
```

**Code — primary exploration targets (read the crates directly; these are the remediation seams):**

The workspace has three crates with strict one-way dependency direction: `tracewake-core`
(authoritative kernel, zero deps) → `tracewake-content` (fixtures, loading, schema validation,
depends on core) → `tracewake-tui` (possession, view models, depends on core + content). The
surviving mutant lives in `tracewake-core`. Seams, with the primary fix target first:

```
PRIMARY FIX TARGET:               crates/tracewake-core/src/projections.rs — the function `actor_known_local_actors_for_context` (the 0036 acceptance artifact records the mutant at projections.rs:336, `replace … -> Vec<ActorId> with vec![]`; verify the current line/signature at c54caff, do not trust the line number). Identify exactly what this returns, who consumes it, and what behavior change `-> vec![]` would (and currently does not) cause.
Consumers of the seam:            crates/tracewake-core/src/agent/actor_known.rs, agent/no_human_surface.rs, epistemics/projection.rs, epistemics/knowledge_context.rs, view_models.rs — trace how an empty local-actor set would propagate to actor-known context, no-human cognition, and embodied view models, so a killing test asserts a real behavioral/provenance consequence.
Mutation posture configuration:   .cargo/mutants.toml, .github/workflows/ci.yml — the configured `cargo mutants` filter set, timeouts, and the scheduled-CI command (the 0036 run filtered `agent/**`, `scheduler*`, `projections*`, `actions/pipeline.rs`, `actions/defs/{eat,sleep,work}.rs` with `--no-shuffle`, found 1128 mutants, and was stopped early). Establish what "the full configured posture run to completion" means.
Lock-layer / behavior witnesses:  crates/tracewake-core/tests/hidden_truth_gates.rs, no_human_capstone.rs, acceptance_gates.rs, event_schema_replay_gates.rs, anti_regression_guards.rs; crates/tracewake-content/tests/golden_fixtures_run.rs; crates/tracewake-tui/tests/adversarial_gates.rs — where a new killing test for the projection seam belongs, and how existing tests already exercise actor-known local actors.
Replay / determinism / checksums: crates/tracewake-core/src/replay/, checksum.rs — replay/provenance ancestry the remediation evidence cites.
Diagnostics by layer:             crates/tracewake-core/src/agent/trace.rs, actions/report.rs, events/apply.rs, content/src/validate.rs — typed failure diagnostics by responsible layer.
```

## 3. Settled intentions (final — these are decisions, not options)

1. **The deliverable is the 0037 P0-CERT mutation-remediation & replacement-certification spec.** The
   "what is next" determination is **doctrine-forced and closed**. Do NOT re-survey for alternative
   next-specs, do not propose SPINE-CERT / EPI-CERT / the `0035` expansion backlog — they are
   *inadmissible* for this certification line until the replacement P0-CERT artifact passes. Your
   research budget goes entirely to **how to remediate and re-certify**, not **what** to build.

2. **Posture = `Remediation`** (per `docs/2-execution/03`'s four valid postures: it fixes the named
   failed gate `0036-MUTATION-REMEDIATION-001`), with replacement-certification character: it also
   *produces the replacement P0-CERT acceptance artifact* that supersedes the 0036 artifact and
   renders `P0-CERT passed`. Declare the single posture explicitly per `docs/4-specs/README.md`.

3. **Full mutation posture, not narrow.** The spec must require running the **full configured P0-CERT
   mutation posture to completion** (the 0036 run was stopped as "too large for this ticket turn" and
   only emitted one of potentially many misses) and triaging **every** surviving mutant across all
   configured seams — each survivor is either killed with behavior/provenance coverage or proven and
   recorded as equivalent/non-critical with explicit rationale. Treat 0036's mutation evidence as
   *sampled / incomplete*, not as a baseline that only one mutant escaped.

4. **Specify both arms for the primary finding.** For `0036-MUTATION-REMEDIATION-001`
   (`actor_known_local_actors_for_context -> vec![]`): the spec must specify (Arm A) **kill** — add a
   behavior/provenance test that fails when the function returns `vec![]`, asserting a real
   actor-known/no-human/view-model consequence with replay or provenance ancestry, not a tautology;
   **or** (Arm B) **prove equivalence/non-criticality** — a recorded, reviewable argument (with the
   exact consuming call sites) that the mutation cannot change observable certified behavior, logged
   in the mutation-triage register. The implementing session chooses per evidence; the spec defines
   the obligations and acceptance criteria for both.

5. **Produce the replacement certification artifact spec.** The spec must define a replacement
   acceptance artifact that (a) conforms to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
   (evidence-status, fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness,
   pending/historical, certification-use, staged-abstraction declaration); (b) carries the completed
   mutation evidence as a `pass` row with an explicit run-completion and survivor-disposition
   statement; (c) renders verdict `P0-CERT passed` for the post-0008 baseline; and (d) explicitly
   **supersedes** `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md`, after
   which later specs may cite `P0-CERT passed`.

6. **Re-prove, do not merely re-cite, the rest of P0-CERT.** The replacement artifact's `P0-CERT
   passed` verdict depends on the full ten-point contract, not the mutant alone. The spec must state
   how the P0-01…P0-10 gate evidence is re-established at `c54caff` (the 0036 artifact's per-gate pass
   rows are *historical evidence* from `9f16222`, not live certification under source discipline —
   they may be cited as prior context but must be re-run/re-witnessed, since a `P0-CERT passed`
   artifact may cite only live current-checkout proof per the 0036 P0-10 discipline). Scope the
   re-proof effort proportionately and declare any reuse explicitly.

7. **Compose existing canonical gates; mint nothing new.** Express the work through the existing
   canonical gates — `P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`,
   `FIXTURE`, `DIAG` (defined in `docs/2-execution/00`) and the `02` first-proof acceptance-contract
   labels. Name gate codes only as cross-references. Define no new gate code, observation obligation,
   or status vocabulary, and do not coin a new finding ID beyond the existing
   `0036-MUTATION-REMEDIATION-001` and any survivors the full run surfaces (those are mutation-run
   outputs, not minted doctrine). Treat `EMERGE-OBS` correctly: an observer-only observation
   obligation, never a pass/fail threshold or phase gate.

8. **Admissibility posture of THIS spec.** Per `docs/4-specs/README.md` every future spec declares one
   of `P0-CERT passed` / `P0-CERT scoped remediation` / `P0-CERT not applicable`. This spec's own
   posture is **`P0-CERT scoped remediation`** — it *is* the admissible remediation the 0036 artifact
   demands, and it consumes the 0036 finding to produce the replacement certification. State this
   explicitly: the spec carries the `scoped remediation` posture and its successful execution is what
   *flips the line to* `P0-CERT passed` via the replacement artifact.

9. **Include every "Gate evidence requirements" element** from `docs/2-execution/03`: exact
   files/seams audited; foundation & architecture dependencies; artifact dependencies (including the
   observer-only `EMERGE-OBS` artifact where the corpus exercises first-proof living-world
   acceptance); positive and negative fixtures; event/replay/projection evidence; actor-known
   provenance evidence; debug-quarantine evidence; failure diagnostics by responsible layer; a
   statement that archived specs/tickets (0005–0036) are used only as historical evidence; and a list
   of tolerated deferrals tied to named gates. Honor "Gate failure handling": the spec names the
   responsible failing layer for the finding (the 0036 artifact assigns it to `lock layer` /
   `holder_known_context` / `projection` / `test_oracle`).

10. **Follow the sibling-spec structural convention, not the foundation-doc house style.** Mirror the
    section shape and rigor of `archive/specs/0025_*`, `0014_*`, and the 0036 spec itself (status,
    admissibility posture, authority/dependency declarations, problem statement, remediation/audit
    approach, per-finding and per-gate deliverables, invariants alignment, verification/acceptance
    artifact, out-of-scope, risks & open questions). Adapt any section that does not fit a remediation
    spec and say why.

11. **Source discipline (`SPEC_LEDGER.md`).** A commit hash named inside a spec is audit/spec
    provenance only. A manifest is path inventory only. Branch names, default-branch lookups, connector
    namespace labels, repository metadata, and code-search snippets are not proof of target-commit
    content. Pin this remediation to commit **`c54caff`**. Treat `9f16222` as 0036's provenance commit,
    not this spec's baseline. Cite archived specs `0005`–`0036` and all tickets only as historical
    evidence, never as live certification.

12. **Scope boundary is the post-0008 baseline only.** The certification line covers the spine, the
    epistemic substrate, and ordinary life (the surfaces historically landed under `0005`–`0025` and
    audited by 0036). Institutions, wrong-suspicion, notices, travel, regional scale, LOD, LLM
    dialogue, and story-sifting are **out of scope** (locked behind `PHASE-4-ENTRY` /
    `SECOND-PROOF-ENTRY`); the spec names them only as deferrals, never as remediation or audit targets.

> `assumption`: the spec is authored at
> **`specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`** — the
> implementation-spec staging series (`specs/NNNN_*`), which is archived to `archive/specs/` on
> acceptance and not promoted into live `docs/4-specs/`. Number `0037` is the next free number across
> `specs/`, `docs/4-specs/`, and `archive/specs/` (highest existing is `0036`, now archived; the
> series is contiguous with no recent renumbering — confirmed against `SPEC_LEDGER.md` and git
> history). If you judge a clearer title slug, you may adjust the title words but keep the `0037_`
> prefix and the `specs/` staging path. The ledger row for this spec is added at acceptance/closeout
> per the repo's hardening-series ledger-timing convention, not authored into the spec itself.

## 4. The task

Author the **P0-CERT mutation-remediation & replacement-certification spec** — the next Tracewake
spec. This is a **new-spec** target with a **hardening / anti-contamination** character and a
**`Remediation`** posture. The 0036 P0-CERT baseline audit rendered `P0-CERT scoped remediation`
because its configured mutation run emitted an untriaged surviving mutant
(`0036-MUTATION-REMEDIATION-001`, `actor_known_local_actors_for_context -> vec![]` in
`projections.rs`) and was stopped before completing the full posture. The spec must turn the
ledger's "Next known execution move" into a concrete, executable remediation plan: how to run the
full configured mutation posture to completion; how to triage every surviving mutant (kill with
behavior/provenance coverage, or prove equivalence with recorded rationale); the specific
both-arm obligations for the primary finding in the projection seam; how the rest of the ten-point
P0-CERT contract is re-proven live at `c54caff`; and the shape of the replacement acceptance
artifact that renders `P0-CERT passed` and supersedes the 0036 artifact. It is a requirements/plan
document that the repo's reassess → ticket → implement process will execute — **not** a rendered
certification result.

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — read the crate sources at
the named seams (especially `projections.rs::actor_known_local_actors_for_context` and its
consumers) and follow dependencies outward so every remediation claim is grounded in real module and
function names. **Research online as deeply as needed** — mutation testing theory and practice
(cargo-mutants configuration, timeouts, incremental/sharded runs, baseline discipline), **equivalent
mutant detection and the equivalent-mutant problem**, designing behavior/provenance tests that kill
"replace return value with empty collection" mutants without tautology, mutation testing as a
certification lock-layer / coverage-adequacy criterion, golden-fixture and property-based adversarial
testing for hidden-truth / information-leakage detection, and audit-evidence packaging for
safety-style certification — wherever it sharpens the remediation method, the triage protocol, the
killing-test design, or the replacement evidence package. **Cite every external source that shapes a
decision.** The deep research is your job; this brief only commissions it.

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every gate the
  replacement artifact certifies must satisfy it; a genuine divergence would require amending an
  invariant first — never design the remediation against an invariant silently. The spec is
  subordinate doctrine: it operationalizes the higher tiers and may not amend, replace, weaken, or
  redefine them.
- **Authority order:** if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong; if a test
  proves only plausible behavior while bypassing holder-known provenance, the test is wrong. A
  killing test that asserts a tautology (e.g. only that the vector is non-empty) is insufficient — it
  must witness a real certified-behavior consequence with provenance/replay ancestry.
- **The truth firewall is the spine:** *truth may validate actions, but truth may not plan them.* The
  remediated seam feeds actor-known cognition; the killing/equivalence evidence must respect that the
  projection supplies sealed actor-known facts with provenance, not validation or debug truth.
- **Anti-contamination:** no simulation fact may be born from prose. Preserve event-sourced
  causality, subjective epistemics, ordinary agents, possession parity, fallible institutions,
  questless leads, validation/replay, the holder-known truth firewall. Adversarial/negative coverage
  that proves forbidden shortcuts FAIL is first-class evidence. Actively resist the post-0008 relapse
  risk in `docs/3-reference/01` (treating archived work as certification) and its anti-Goodhart note
  (do not let "kill the one named mutant" become a metric-gaming substitute for completing the run).
- **No backwards-compatibility shims or alias paths** in any recommendation; no new files merely for
  symmetry.
- **Terminology:** use `holder-known context` as the system-wide term and `actor-known` for the actor
  case.

## 7. Deliverable specification

Produce **one** downloadable markdown document — a **numbered implementation spec** (this IS a
`specs/`-tier artifact; the numbering/placement assumption in §3 applies). There is no paired
`-research-report.md`.

- **`specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`** — **new** file.
  (If you determine a clearly-better title slug, adjust the title words but keep the `0037_` prefix
  and the `specs/` staging path unless §3's placement assumption is wrong.)

**This deliverable is non-executable.** You cannot run `cargo fmt/clippy/build/test`, `cargo mutants`,
or replay against live code from this session. The spec therefore specifies *what the implementing
session will run and prove, and the acceptance contract it must satisfy* — **not** a rendered
pass/fail. You **MAY** include a clearly-labeled **preliminary static survey** of the
`actor_known_local_actors_for_context` seam and its consumers (what reading the code at `c54caff`
suggests about whether the `vec![]` mutation is killable vs. likely-equivalent, and where a killing
test belongs) as *informative, non-certifying* evidence — explicitly marked **"preliminary, not
certification."** If you judge the survey would dilute the spec, omit it; the remediation-plan spec is
the required core either way. Authoritative pass/fail belongs to the implementing session that
executes the gates.

Required content (adapt section names to the sibling-spec convention in `archive/specs/0036_*`,
`0025_*`, `0014_*`; the substance below is mandatory):

1. **Status + admissibility posture** — declare authority beneath foundation/architecture/execution/
   reference; declare the single posture `P0-CERT scoped remediation` and explain it is the admissible
   remediation that flips the line to `P0-CERT passed` via the replacement artifact (settled
   intentions 2, 8).
2. **Authority & dependencies** — the controlling foundation/architecture/execution docs (the §2
   primary set), pinned to commit `c54caff`; the 0036 spec and acceptance artifact as the remediated
   source (history/context); archived specs cited as history only.
3. **Problem statement** — the 0036 `P0-CERT scoped remediation` verdict: the configured mutation run
   found 1128 mutants, passed the unmutated baseline, emitted the untriaged
   `0036-MUTATION-REMEDIATION-001` miss, and was stopped before completion, leaving the mutation
   lock-layer incomplete and the certification line blocked (cite `SPEC_LEDGER.md`, the acceptance
   artifact, and `docs/2-execution/03` gate-failure handling).
4. **Remediation approach** — the method: (a) full configured mutation-posture run to completion with
   a defined run/timeout/sharding strategy; (b) a survivor-triage protocol (kill vs. proven-equivalent
   with recorded rationale) and a triage register; (c) the both-arm obligations for the primary
   projection-seam finding; (d) how the remaining P0-01…P0-10 gate evidence is re-proven live at
   `c54caff` and what (if anything) is reused, declared explicitly; (e) how staged abstractions (if
   any) are declared and bounded.
5. **Per-finding & per-gate deliverables** — a first-class block for `0036-MUTATION-REMEDIATION-001`
   (seams, both arms, the killing-test design with replay/provenance ancestry or the equivalence
   argument, the naming of the responsible layer); plus, for each P0-CERT proof requirement
   (the ten points of `docs/2-execution/01`), how the replacement artifact re-establishes its
   evidence at `c54caff` — concrete seams, the canonical gate(s) composed, positive fixtures,
   adversarial/negative fixtures that must fail, event/replay/projection evidence, and the typed
   failure diagnostic that names the responsible layer. Include every "Gate evidence requirements"
   element from `docs/2-execution/03` (settled intention 9).
6. **Replacement acceptance / evidence artifact** — the conforming replacement acceptance artifact
   specified against the `docs/4-specs/0003` template's fields (settled intention 5), including the
   completed-mutation `pass` row with run-completion + survivor-disposition statement, the
   observer-only `EMERGE-OBS` artifact where the corpus exercises first-proof living-world acceptance,
   any staged-abstraction declarations, the `P0-CERT passed` verdict, and the explicit supersession of
   the 0036 artifact.
7. **Invariants alignment** — which constitutional invariants the remediated seam and each re-proven
   gate preserve (cite `INV-NNN` as cross-references; coin none).
8. **Out of scope / deferrals** — the locked Phase-4 and second-proof surfaces (settled intention 12),
   each tied to its named entry gate; and an explicit statement that no forward gate (SPINE-CERT etc.)
   is advanced by this spec.
9. **Risks & open questions** — remediation risks (e.g. a tautological killing test; the equivalent-
   mutant problem; the full run surfacing many new survivors that re-block the line; mutation-run cost/
   timeout; metric-gaming the single named mutant), and owner decisions you cannot settle from the docs
   (carry as open questions, do not invent).

Do **not** write final ticket breakdowns (the repo's spec-to-tickets process owns that), invent new
gate/obligation codes or status enums, or assert a certified pass/fail result.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

## 8. Self-check (run before returning)

- [ ] The deliverable is exactly one new spec at `specs/0037_…` (or a justified title variant with the
      same prefix/path), in the sibling-spec structural convention — not the narrative foundation-doc
      style.
- [ ] The spec's posture is declared as `P0-CERT scoped remediation`, addressing
      `0036-MUTATION-REMEDIATION-001`, and it explains how it flips the line to `P0-CERT passed` via the
      replacement artifact.
- [ ] The full configured mutation posture is required to run to completion with every survivor triaged
      (killed with behavior/provenance coverage or proven equivalent with recorded rationale) — not just
      the one named mutant.
- [ ] The primary finding specifies both arms (kill / prove-equivalent); any killing test asserts a real
      certified-behavior consequence with replay/provenance ancestry, not a tautology.
- [ ] All ten P0-CERT proof requirements from `docs/2-execution/01` are addressed for the replacement
      artifact, re-proven live at `c54caff`, with the 0036 per-gate evidence treated as history not live
      certification.
- [ ] The canonical gates (`TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`,
      `DIAG`) and the `02` acceptance-contract gates are composed, not redefined; no new gate code,
      obligation code, status vocabulary, or finding ID is minted; `EMERGE-OBS` is observer-only.
- [ ] Every "Gate evidence requirements" element from `docs/2-execution/03` is present, and the failing
      layer is named per "Gate failure handling".
- [ ] The replacement acceptance artifact conforms to the `docs/4-specs/0003` template fields, carries
      the completed-mutation evidence, renders `P0-CERT passed`, and explicitly supersedes the 0036
      artifact.
- [ ] Scope is the post-0008 baseline only; institutions / wrong-suspicion / notices / travel /
      regional / LOD / LLM / story-sifting appear only as named deferrals; no forward gate is advanced.
- [ ] No new doctrine weakens an upstream tier; the spec stays subordinate to
      foundation/architecture/execution/reference.
- [ ] Source discipline holds: pinned to `c54caff`; `9f16222` treated as 0036's provenance only;
      archived specs/tickets cited as history only; manifest treated as path inventory only.
- [ ] Any preliminary static seam survey (if included) is explicitly labeled non-certifying.
- [ ] Every external claim that shaped a decision is cited.
- [ ] Commit `c54caff` contains every file named in §2.
