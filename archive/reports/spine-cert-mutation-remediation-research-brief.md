# Research brief — SPINE-CERT mutation remediation & replacement certification spec (the next Tracewake spec)

> **You are ChatGPT-Pro, Session 2.** This prompt is self-contained. Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a requirement
> impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-18_9648c8f.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first; agents act from
partial belief and every event leaves a replayable trace). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; **earlier tiers
govern later ones.** Fetch every file from commit **`9648c8f`** (verified repo `HEAD`) — the
manifest reflects exactly that tree.

**This is a delta successor, not a cold start, and it is one turn of a repeating certification
pattern.** The certification ladder is `P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT →
FIRST-PROOF-CERT → PHASE-4-ENTRY → SECOND-PROOF-ENTRY` (`docs/2-execution/03`). The immediately
relevant precedents:

- **P0-CERT line (your structural template).** The brief
  `reports/p0-cert-mutation-remediation-research-brief.md` commissioned the spec
  `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`. That spec
  remediated the `0036` P0-CERT audit, which had rendered **`P0-CERT scoped remediation`** because a
  configured mutation run emitted an untriaged survivor and was stopped early. `0037` ran the full
  posture to completion, killed/triaged the survivors, re-proved the ten-point contract, and produced
  the replacement acceptance artifact
  `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` that
  rendered **`P0-CERT passed`** and superseded the `0036` artifact. **`0037` is the near-exact
  structural model for your deliverable** — adapt its shape from "P0-CERT / one survivor" to
  "SPINE-CERT / 296 survivors."

- **SPINE-CERT line (your subject).** The spec
  `archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md`
  defined the SPINE-CERT audit (SPINE-01…SPINE-08: event log, replay, projection, randomness,
  save/manifest, action pipeline, TUI/debug split, no-direct-dispatch). Its acceptance artifact
  `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md`
  rendered **`SPINE-CERT scoped remediation`**, **not** `SPINE-CERT passed`, because a **Wave B**
  mutation expansion (run with `--no-config` to reach beyond the configured `.cargo/mutants.toml`
  perimeter) found **296 missed mutants** across the SPINE seam files. The full survivor inventory and
  a per-file/per-seam required-remediation table already exist at
  `reports/0038_spine_cert_mutation_triage_register.md`.

The repository's own doctrine now names the single admissible next move. `docs/4-specs/SPEC_LEDGER.md`
("Next known execution move") states it plainly: *"`SPINE-CERT` currently stands at scoped
remediation per [the 0038 acceptance artifact]; the next admissible work is scoped remediation of the
296 Wave B mutation survivors, not later-gate progression."* The 0038 artifact's own verdict block
adds that later specs **may not** cite `SPINE-CERT passed` or proceed to `EPI-CERT`,
`ORD-LIFE-CERT`, Phase-4 entry, or later gates until scoped remediation resolves the survivor
posture. Therefore EPI-CERT, ORD-LIFE-CERT, the `0035` route-forward expansion backlog, and every
other forward move are **doctrinally blocked** and cannot be the next spec. Your task is **not** to
re-decide what is next — it is to author the SPINE-CERT mutation-remediation-and-replacement-
certification spec itself.

This brief is pinned to verified `HEAD` `9648c8f` (`Merge pull request #47 … Archive 0038 SPINE-CERT
scoped remediation spec`). Source discipline (`SPEC_LEDGER.md`): a commit hash named inside a spec or
report is audit/spec provenance only. The 0038 acceptance artifact records its own implementation
commit as `0ce59ad…` and an "implementation commit under test" of `b4b59c92…`; **treat both as 0038's
own audit provenance, not this remediation's baseline.** This remediation is pinned to `9648c8f`. The
0038 spec/report/tickets were archived (staging → `archive/`) at or before this `HEAD`; the SPINE seam
source files and the doctrine read-list below are the live tree at `9648c8f`, and the 296-survivor
finding is the live blocker at this baseline.

## 2. Read in full (authority order)

Read these before producing. Order is by authority tier. The 0038 seed artifacts, the 0037 template,
and the execution tier are the heart of this target.

**Seed artifacts — primary (the failed certification this spec remediates; read these first):**

```
reports/0038_spine_cert_mutation_triage_register.md — THE survivor inventory: the 296 Wave B missed mutants, the Wave A/Wave B commands, the no-silent-exclusion proof, the per-file survivor-group triage table (file → count → responsible SPINE seam → diagnostic layer → why tests missed it → required remediation), and the full line-by-line survivor list. This is your concrete work-list; the spec turns it into an executable remediation contract.
archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md — THE failing artifact: the `SPINE-CERT scoped remediation` verdict, the SPINE-01…SPINE-08 per-seam evidence sections, the evidence-status ledger legend, the command transcript index, and the conforming acceptance-artifact shape your replacement must match and supersede.
archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md — the spec being remediated: the SPINE-CERT audit contract, the SPINE-01…SPINE-08 seam inventory, the positive/adversarial fixture corpus, the gate composition, and the section shape your 0039 spec extends and re-runs to completion.
docs/4-specs/SPEC_LEDGER.md — the live ledger: the 0038 row (verdict `SPINE-CERT scoped remediation`), the source-discipline rules, the archived-spec posture, and the "Next known execution move" that defines this spec's mandate.
```

**Structural template — primary (the near-identical predecessor remediation; mirror its rigor and section shape):**

```
archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md — the structural model: status + admissibility posture, source-discipline/freshness section, authority & dependency declarations, problem statement, remediation approach (full posture run, survivor-triage protocol, both-arm primary-finding disposition, re-proof of the rest of the contract), per-finding deliverable, gate-evidence + re-proof matrix, replacement-acceptance-artifact spec, invariants alignment, out-of-scope/deferrals, risks & open questions, completion checklist. Adapt "one survivor / P0-01…P0-10" to "296 survivors / SPINE-01…SPINE-08."
archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md — the replacement artifact 0037 produced: the model for how a completed-mutation `pass` row and run-completion + survivor-disposition statement read in practice.
```

**Foundation — primary (the constitution and the properties the surviving mutants threaten):**

```
docs/README.md — authority order and the layering rule (foundation > architecture > execution > reference > specs).
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md — INV-001…INV-NNN; the event-sourcing, replay, holder-known-provenance, possession-parity, and debug-quarantine invariants the SPINE seams uphold and the survivors threaten.
docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md — event authority, replay, traceability, snapshots, randomness; the contract behind the events/replay/checksum survivors.
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md — "truth may validate, truth may not plan"; the firewall behind the epistemics/knowledge_context survivors (67 of the 296).
docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md — possession parity, actor-filtered view models, debug quarantine; behind the view_models/debug_reports/render/transcript survivors.
docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md — first-village scope, no-human / replay / TUI gates, canonical regression seeds the SPINE corpus exercises.
```

**Architecture — primary (subsystem contracts the survivor-killing coverage measures code against):**

```
docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md — universal conformance questions and the replacement/retirement rule.
docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md — event log, replay, projections, save manifests, random-stream discipline, schema versioning; the contract for SPINE-01/02/04/05.
docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md — holder-known contexts, provenance packets, context sealing; the contract for SPINE-03 and the knowledge_context survivors.
docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md — proposal pipeline, scheduler limits, validation truth, no-direct-dispatch; the contract for SPINE-06.
docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md — possession, embodied TUI, debug-only truth, client boundaries; the contract for SPINE-07.
docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md — acceptance artifacts, anti-contamination tests, diagnostics, review checklists; the home of the lock-layer/mutation posture.
```

**Execution — primary (the SPINE-CERT contract, the remediation posture, and the gate-failure rule):**

```
docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md — canonical gate names (P0-CERT, TFW, PIPE, NO-DIRECT, NO-HUMAN, POS-PARITY, REPLAY, FIXTURE, DIAG), the phase-certification-label class (SPINE-CERT etc. compose gates but are not new canonical gates), observation obligations (EMERGE-OBS observer-only), label reconciliation, universal execution posture.
docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md — gate order (SPINE-CERT is sequence 2), the "Gate evidence requirements" list, "Gate failure handling" (a failed gate must produce a remediation spec/report naming the failing layer), and the four valid future-spec postures (Remediation is yours).
docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md — archived-spec status, current code audit boundary, certification posture (the basis the SPINE seams are audited against).
docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md — first-proof acceptance contract gates the replacement artifact still composes.
docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md — truth-firewall execution checks and mandatory anti-contamination gates (TFW); the seam class many survivors live in.
docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md — scheduler/proposal/validation/direct-dispatch audit criteria (PIPE, NO-DIRECT); SPINE-06 and the scheduler/proposal/report survivors.
docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md — golden fixture families, adversarial scenarios, deterministic replay acceptance (FIXTURE, REPLAY).
docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md — testing, diagnostics by responsible layer, review evidence, the mutation/lock-layer posture, and the EMERGE-OBS observation obligation (DIAG). The home of "what the configured mutation posture is."
```

**Reference + specs — primary (rules, numbering, source discipline, structural models):**

```
docs/4-specs/README.md — future-spec rules: declare authority posture, declare one admissibility posture, gate codes as cross-references only, holder-known/actor-known terminology, source discipline, no symmetry files.
docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md — the live first-proof ontology/fixture contract; the system the certification line covers.
docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md — the acceptance-artifact template (evidence-status / fingerprint-scope / behavior-witness / replay-provenance / sampling-exhaustiveness / pending-historical / certification-use / staged-abstraction fields). The replacement certification artifact MUST conform to this.
```

**Reference — boundary-awareness (read to bound scope; NOT conformance targets):**

```
docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md — review guardrails and gate-code lookup posture.
docs/3-reference/01_DESIGN_RISK_REGISTER.md — the post-0008 relapse risk (treating archived 0005–0038 as certification) and the anti-Goodhart watch note the remediation must actively resist (do not let "kill a few named mutants" substitute for completing the run).
docs/3-reference/02_GLOSSARY.md — prescriptive terminology control (holder-known, observation, belief, debug truth, source discipline).
```

**Boundary-awareness — OUT of scope (read only to know what this spec must NOT touch or advance into):**

```
docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md — Phase-4 entry lock; blocked behind PHASE-4-ENTRY, downstream of the SPINE-CERT → EPI-CERT → ORD-LIFE-CERT → FIRST-PROOF-CERT chain.
docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md — deferral contract for notices/travel/regional/LOD/second-proof.
docs/0-foundation/{07,09,10,11,13} and docs/1-architecture/{06,07,08,09,11,12,14} — institutions, claims/beliefs depth, LLM/speech, LOD/regional scale, no-scripting authoring, research notes: future-phase, deferred, or already-certified-elsewhere surfaces, not part of the SPINE-CERT remediation. (EPI-CERT will own the deep epistemic-substrate proof; this spec remediates only the SPINE-seam mutation survivors, even where a survivor file like `epistemics/knowledge_context.rs` also feeds EPI-CERT.)
```

**Code — primary exploration targets (read the crates directly; these are the remediation seams):**

The workspace has three crates with strict one-way dependency direction: `tracewake-core`
(authoritative kernel, zero deps) → `tracewake-content` (fixtures, loading, schema validation,
depends on core) → `tracewake-tui` (possession, view models, depends on core + content). The 296
survivors span all three crates. The triage register groups them by file; the per-file survivor
counts (Wave B) are:

```
tracewake-core/src/epistemics/knowledge_context.rs   — 67   (SPINE-03; subjective epistemics / no truth leak)
tracewake-content/src/validate.rs                    — 57   (SPINE-05/06; content validation, no prose-born facts, no direct state/script, determinism)
tracewake-core/src/events/apply.rs                   — 44   (SPINE-01/02; event application and causality)
tracewake-core/src/view_models.rs                    — 24   (SPINE-03/07; view-model / debug disclosure)
tracewake-core/src/debug_reports.rs                  — 19   (SPINE-03/07; debug-only reports, projection/replay diagnostics)
tracewake-content/src/serialization.rs               — 17   (SPINE-02/05; save serialization and replay inputs)
tracewake-core/src/actions/report.rs                 — 13   (SPINE-06; no-direct-dispatch checked facts)
tracewake-core/src/events/envelope.rs                — 9    (SPINE-01; event envelope serialization)
tracewake-content/src/load.rs                        — 8    (SPINE-05; save/content manifest, seed event construction)
tracewake-core/src/scheduler.rs                      — 8    (SPINE-06; scheduler / no-human-day routine progress)
tracewake-core/src/replay/rebuild.rs                 — 7    (SPINE-02; replay rebuild determinism)
tracewake-core/src/checksum.rs                       — 6    (SPINE-02; deterministic replay checksum)
tracewake-core/src/projections.rs                    — 5    (SPINE-03/06; projections / action-proposal surface)
tracewake-core/src/actions/proposal.rs               — 3    (SPINE-06; action proposal provenance)
tracewake-content/src/schema.rs                      — 3    (SPINE-05; content-schema → agent-state conversion)
tracewake-tui/src/render.rs                          — 4    (SPINE-07; TUI-first playability / debug presentation)
tracewake-core/src/replay/report.rs                  — 1    (SPINE-02; replay report mismatch)
tracewake-tui/src/transcript.rs                      — 1    (SPINE-07; TUI transcript evidence)
```

Other seams to inspect (provenance/replay ancestry, lock-layer behavior witnesses, configured posture):

```
Mutation posture configuration:   .cargo/mutants.toml (configured Wave A perimeter + workspace/locked args + exclusions), .github/workflows/ci.yml (the scheduled-CI mutation command), .cargo/mutants-baseline-misses.txt (accepted baseline misses — applies only to the Wave A perimeter; the 296 Wave B misses are NOT accepted baseline misses). Establish what "the full configured posture, run to completion, over the expanded SPINE seam set" means and what the standing perimeter becomes.
Lock-layer / behavior witnesses:  crates/tracewake-core/tests/{anti_regression_guards.rs, hidden_truth_gates.rs, event_schema_replay_gates.rs, acceptance_gates.rs, no_human_capstone.rs, spine_conformance.rs, generative_lock.rs, golden_scenarios.rs, negative_fixture_runner.rs}; crates/tracewake-content/tests/{fixtures_load.rs, forbidden_content.rs, golden_fixtures_run.rs, schema_conformance.rs}; crates/tracewake-tui/tests/{adversarial_gates.rs, tui_seam_conformance.rs, transcript_snapshot.rs, tui_acceptance.rs, command_loop_session.rs} — where new survivor-killing tests belong, and how existing tests already exercise (or miss) each seam. The 0038 command transcript index enumerates the full live gate set.
```

## 3. Settled intentions (final — these are decisions, not options)

1. **The deliverable is the 0039 SPINE-CERT mutation-remediation & replacement-certification spec.**
   The "what is next" determination is **doctrine-forced and closed** by `SPEC_LEDGER.md`'s "Next known
   execution move" and the 0038 artifact's admissibility lock. Do NOT re-survey for alternative
   next-specs; do NOT propose EPI-CERT, ORD-LIFE-CERT, the `0035` expansion backlog, or any feature/
   gameplay work — they are *inadmissible* for the certification ladder until the replacement
   `SPINE-CERT passed` artifact exists. Your research budget goes entirely to **how to remediate and
   re-certify**, not **what** to build.

2. **Posture = `Remediation`** (per `docs/2-execution/03`'s four valid postures: it fixes the named
   failed gate — the 296 SPINE-CERT Wave B mutation survivors), with replacement-certification
   character: it also *produces the replacement SPINE-CERT acceptance artifact* that supersedes the
   0038 artifact and renders `SPINE-CERT passed`. Declare the single posture explicitly per
   `docs/4-specs/README.md`. This spec's own admissibility posture is **`SPINE-CERT scoped
   remediation`** — it *is* the admissible remediation the 0038 artifact demands, and its successful
   execution is what *flips the line to* `SPINE-CERT passed` via the replacement artifact. (Note: the
   P0-CERT line is already `P0-CERT passed` per `0037`; this spec consumes that as settled context, it
   does not re-open P0-CERT.)

3. **Full configured posture, run to completion, over the expanded SPINE seam set — triage every
   survivor.** The spec must require running the mutation posture **to completion** and triaging
   **every** surviving mutant — all 296 Wave B survivors enumerated in
   `reports/0038_spine_cert_mutation_triage_register.md`, plus any further survivors a complete run
   over the expanded perimeter surfaces. Treat the 296-survivor inventory as the **seed work-list, not
   the finish line**: the count is a floor, established by a `--no-config` Wave B run, and the
   completed remediation run must re-enumerate it under the new standing configuration. Each survivor
   is either killed with behavior/provenance coverage or proven-and-recorded as equivalent/non-critical
   with explicit reviewer-signed rationale.

4. **Mandate permanent mutation-perimeter expansion.** The 296 survivors were found *outside* the
   configured `.cargo/mutants.toml` perimeter (Wave A covered only
   `agent/**`, `scheduler*`, `projections*`, `actions/pipeline.rs`, and `actions/defs/{eat,sleep,work}.rs`);
   Wave B reached the SPINE seam files only via `--no-config`. The spec must **require updating
   `.cargo/mutants.toml` and the CI mutation command so the full SPINE-CERT seam set becomes part of
   the STANDING configured/scheduled mutation posture** — so these regressions are caught permanently,
   not in a one-off. The spec must state the perimeter explicitly (the SPINE seam file set in §2), keep
   the no-silent-exclusion discipline the triage register demonstrates (no SPINE seam silently dropped
   by an exclusion glob), and require that `.cargo/mutants-baseline-misses.txt` is not used to launder a
   live survivor into an accepted miss without behavior/equivalence triage. This is the central
   divergence from `0037`, which left CI/perimeter expansion as an *open question*: here it is a settled
   requirement.

5. **Kill-with-coverage is the default disposition; grouped behavior-contract tests are permitted.**
   For all survivors the default disposition is **kill with behavior/provenance coverage**; proven
   equivalence/non-criticality is the **rare exception** and requires a reviewable argument naming exact
   call sites plus reviewer signoff (a single implementer's "no test failed" is never sufficient). To
   manage the 296-count scale, the spec **must permit grouping kindred survivors under shared
   behavior-contract test families** — e.g. all `debug_only -> bool` flag mutants under one
   debug-quarantine contract test, all `stable_id`/`canonical_key` string mutants under one
   canonical-identity contract test, all parser-arm deletions (`parse_channel`/`parse_stance`/
   `parse_routine_family`/`from_stable_key`/etc.) under round-trip serialization/apply contract tests,
   and all `ActorKnown*Fact` field/source-key mutants under sealed-context/provenance contract tests.
   A grouped test family is acceptable **only if** it genuinely fails for every member mutant
   (the spec must require that the group test demonstrably catches each grouped survivor, not merely a
   representative). Grouping is a scale-management tool, not a coverage shortcut.

6. **Produce the replacement certification artifact spec.** The spec must define a replacement
   acceptance artifact that (a) conforms to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
   (evidence-status, fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness,
   pending/historical, certification-use, staged-abstraction declaration); (b) carries the completed
   mutation evidence as a `pass` row with explicit run-completion and full survivor-disposition counts
   (caught/killed-by-remediation/equivalent/non-critical, zero untriaged); (c) renders verdict
   `SPINE-CERT passed` for the spine certification line; and (d) explicitly **supersedes**
   `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md`,
   after which later specs may cite `SPINE-CERT passed` and the ladder may advance to EPI-CERT.

7. **Re-prove, do not merely re-cite, the SPINE-01…SPINE-08 seam contract.** The replacement artifact's
   `SPINE-CERT passed` verdict depends on the full eight-seam contract, not the mutation survivors
   alone. The spec must state how the SPINE-01…SPINE-08 evidence (per the 0038 spec and its acceptance
   artifact's per-seam sections) is re-established live at `9648c8f` — the 0038 per-seam pass rows are
   *historical evidence* under source discipline (and reference 0038's own provenance commits), not live
   certification, though they may be cited as prior context and structural shape. Scope the re-proof
   proportionately (the SPINE seams largely held in 0038; the gap was mutation coverage, not seam
   failure) and declare any reuse explicitly. The mutation remediation strengthens exactly the
   behavior witnesses these seams rely on, so the two efforts are coupled, not parallel.

8. **Compose existing canonical gates and the SPINE-CERT phase-label; mint nothing new.** Express the
   work through the existing canonical gates — `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`,
   `REPLAY`, `FIXTURE`, `DIAG` (defined in `docs/2-execution/00`) — composed under the `SPINE-CERT`
   phase-certification label, plus the `02` first-proof acceptance-contract labels. Name gate codes
   only as cross-references. Define **no** new gate code, observation obligation, status vocabulary, or
   finding ID. Survivors are identified by cargo-mutants output (path:line:operator) and the triage
   register, not by minted doctrine-level IDs. Treat `EMERGE-OBS` correctly: an observer-only
   observation obligation, never a pass/fail threshold, mutation score, or phase gate.

9. **Include every "Gate evidence requirements" element** from `docs/2-execution/03`: exact
   files/seams audited; foundation & architecture dependencies; artifact dependencies (including the
   observer-only `EMERGE-OBS` artifact where the corpus exercises first-proof living-world acceptance);
   positive and negative fixtures; event/replay/projection evidence; actor-known provenance evidence;
   debug-quarantine evidence; failure diagnostics by responsible layer; a statement that archived
   specs/tickets (0005–0038) are used only as historical evidence; and a list of tolerated deferrals
   tied to named gates. Honor "Gate failure handling": the spec maps each survivor group to its
   responsible failing layer (the triage register already assigns these per file — e.g. epistemic
   knowledge diagnostics, event apply diagnostics, schema/content validator diagnostics, replay/checksum
   diagnostics, scheduler diagnostics, view-model diagnostics, debug report layer, TUI render/transcript
   diagnostics).

10. **Follow the sibling-spec structural convention, not the foundation-doc house style.** Mirror the
    section shape and rigor of `archive/specs/0037_*` (the closest model) and `0038_*` (the spec being
    remediated): status + admissibility posture, source-discipline/freshness, authority & dependency
    declarations, problem statement, remediation/audit approach, per-finding/per-group and per-seam
    deliverables, gate-evidence + re-proof matrix, replacement-acceptance-artifact spec, invariants
    alignment, out-of-scope/deferrals, risks & open questions, completion checklist. Adapt any section
    that does not fit a 296-survivor remediation and say why.

11. **Source discipline (`SPEC_LEDGER.md`).** A commit hash named inside a spec or report is audit/spec
    provenance only. A manifest is path inventory only. Branch names, default-branch lookups, connector
    namespace labels, repository metadata, and code-search snippets are not proof of target-commit
    content. Pin this remediation to commit **`9648c8f`**. Treat the 0038 artifact's `0ce59ad…` /
    `b4b59c92…` strings as 0038's own provenance, not this spec's baseline. Cite archived specs
    `0005`–`0038` and all tickets only as historical evidence, never as live certification.

12. **Scope boundary is the SPINE-CERT certification line only.** The certification line covers the
    spine seams SPINE-01…SPINE-08: event log, replay, projection, randomness, save/manifest, action
    pipeline, TUI/debug split, and no-direct-dispatch. EPI-CERT's deep epistemic-substrate proof,
    ORD-LIFE-CERT, institutions, wrong-suspicion, notices, travel, regional scale, LOD, LLM dialogue,
    and story-sifting are **out of scope** (locked behind their named entry gates); the spec names them
    only as deferrals, never as remediation or audit targets — even where a survivor file (e.g.
    `epistemics/knowledge_context.rs`) also feeds a future gate. Remediating a SPINE-seam survivor must
    not be inflated into auditing EPI-CERT.

> `assumption`: the spec is authored at
> **`specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`**, staged under
> the `specs/NNNN_*` series and archived to
> `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` on
> acceptance closeout (not promoted into live `docs/4-specs/`), mirroring the 0037/0038 hardening-series
> convention. Number `0039` is the next free staging number: the staging series in `archive/specs/`
> is contiguous through `0038` with no recent renumbering or unification (confirmed against
> `SPEC_LEDGER.md` and `git log` — the only recent renames are the routine 0038 staging → archive
> closeout). If you judge a clearer title slug, you may adjust the title words but keep the `0039_`
> prefix and the `specs/` staging path. The ledger row for this spec is added at acceptance/closeout per
> the repo's hardening-series ledger-timing convention, not authored into the spec itself.

## 4. The task

Author the **SPINE-CERT mutation-remediation & replacement-certification spec** — the next Tracewake
spec. This is a **new-spec** target with a **hardening / anti-contamination** character and a
**`Remediation`** posture. The 0038 SPINE-CERT audit rendered `SPINE-CERT scoped remediation` because
its Wave B mutation expansion found **296 missed mutants** across the SPINE seam files
(`reports/0038_spine_cert_mutation_triage_register.md`), found *outside* the configured
`.cargo/mutants.toml` perimeter. The spec must turn the ledger's "Next known execution move" into a
concrete, executable remediation plan: how to permanently widen the configured mutation perimeter to
the full SPINE seam set and run it to completion; how to triage every surviving mutant (kill with
behavior/provenance coverage as the default, with grouped behavior-contract test families permitted for
scale; equivalence the rare reviewer-signed exception); the per-survivor-group obligations mapped to
their responsible seams and diagnostic layers; how the SPINE-01…SPINE-08 seam contract is re-proven
live at `9648c8f`; and the shape of the replacement acceptance artifact that renders `SPINE-CERT
passed` and supersedes the 0038 artifact. It is a requirements/plan document that the repo's
reassess → ticket → implement process will execute — **not** a rendered certification result.

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — read the 18 survivor crate
files at the named seams and follow dependencies outward (especially how
`epistemics/knowledge_context.rs` actor-known facts flow through `projections.rs` into `view_models.rs`
and the embodied view, how `events/apply.rs` parser arms and apply functions reach replay/checksum, and
how `content/src/validate.rs` validators gate prose-born/direct-state/determinism contamination) so
every remediation claim is grounded in real module and function names. **Research online as deeply as
needed** (the user explicitly encourages deep external research — similar implementations, research
papers, and whatever is beneficial): mutation-testing theory and practice (cargo-mutants configuration,
perimeter/filters, timeouts, sharded/incremental runs, baseline discipline, `--no-config` vs configured
posture), **the equivalent-mutant problem and equivalent/redundant-mutant detection**, designing
behavior tests that kill "replace return value with empty collection / empty string / `xyzzy`",
"delete match arm", and "flip comparison/boolean operator" mutants **without tautology**, behavior-
contract / parameterized test families for large survivor sets, mutation testing as a certification
lock-layer / coverage-adequacy criterion, golden-fixture and property-based adversarial testing for
hidden-truth / information-leakage detection, and audit-evidence packaging for safety-style
certification — wherever it sharpens the remediation method, the triage protocol, the killing-test and
test-grouping design, the perimeter-governance decision, or the replacement evidence package. **Cite
every external source that shapes a decision.** The deep research is your job; this brief only
commissions it.

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every gate the replacement
  artifact certifies must satisfy it; a genuine divergence would require amending an invariant first —
  never design the remediation against an invariant silently. The spec is subordinate doctrine: it
  operationalizes the higher tiers and may not amend, replace, weaken, or redefine them.
- **Authority order:** if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong; if a test proves
  only plausible behavior while bypassing holder-known provenance, the test is wrong. A killing test
  that asserts a tautology (e.g. only that a helper returns a non-empty vector, or that a `debug_only()`
  getter returns the literal it returns) is insufficient — it must witness a real certified-behavior
  consequence with provenance/replay ancestry. This is acute for the many "stable_id/canonical_key
  string" and "debug_only flag" survivors, which invite tautological getters-assert-getter tests; the
  spec must demand behavior witnesses (e.g. a string-id mutation must change a canonical key that a
  checksum/replay/serialization round-trip or a quarantine contract observably depends on).
- **The truth firewall is the spine:** *truth may validate actions, but truth may not plan them.* Many
  survivors live in `epistemics/knowledge_context.rs` (sealed actor-known facts), `projections.rs`, and
  the debug/view-model seams; killing/equivalence evidence must respect that projections supply sealed
  actor-known facts with provenance, and debug truth must never reach embodied views. A quick fix that
  satisfies a view assertion by sourcing a fact from truth or debug is forbidden; killing tests need
  negative/contamination controls.
- **Anti-contamination:** no simulation fact may be born from prose. Preserve event-sourced causality,
  subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads,
  validation/replay, the holder-known truth firewall. Adversarial/negative coverage that proves
  forbidden shortcuts FAIL is first-class evidence (especially for the 57 `content/src/validate.rs`
  survivors — reference/topology/marker/determinism/roundtrip validators that must reject forbidden
  content). Actively resist the post-0008 relapse risk in `docs/3-reference/01` (treating archived work
  as certification) and its anti-Goodhart note (do not let "kill a representative subset" or "accept
  survivors as baseline misses" substitute for completing the run and triaging every survivor).
- **No backwards-compatibility shims or alias paths** in any recommendation; no new files merely for
  symmetry; no test-only branch that bypasses the real seam path to "kill" a mutant.
- **Terminology:** use `holder-known context` as the system-wide term and `actor-known` for the actor
  case.

## 7. Deliverable specification

Produce **one** downloadable markdown document — a **numbered implementation spec** (this IS a
`specs/`-tier artifact; the numbering/placement assumption in §3 applies). There is **no** paired
`-research-report.md`.

- **`specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`** — the staged
  spec file (archived to `archive/specs/0039_…` on closeout per the hardening-series convention; a
  justified title-slug variant is acceptable if it keeps the `0039_` prefix and `specs/` staging path).

**This deliverable is non-executable.** You cannot run `cargo fmt/clippy/build/test`, `cargo mutants`,
or replay against live code from this session. The spec therefore specifies *what the implementing
session will run and prove, and the acceptance contract it must satisfy* — **not** a rendered pass/fail.
You **MAY** include a clearly-labeled **preliminary static survey** of a few representative survivor
groups (e.g. the `knowledge_context.rs` `ActorKnown*Fact` source-key/canonical-key cluster, the
`view_models.rs`/`debug_reports.rs` `debug_only` flag cluster, a `validate.rs` validator cluster, and
the `events/apply.rs` parser-arm cluster) — what reading the code at `9648c8f` suggests about whether
each is killable vs. likely-equivalent, where a killing/contract test belongs, and what behavior witness
defeats tautology — as *informative, non-certifying* evidence, explicitly marked **"preliminary, not
certification."** If you judge the survey would dilute the spec, keep it tight or omit it; the
remediation-plan spec is the required core either way. Authoritative pass/fail belongs to the
implementing session that executes the gates.

Required content (adapt section names to the sibling-spec convention in `archive/specs/0037_*`,
`0038_*`; the substance below is mandatory):

1. **Status + admissibility posture** — declare authority beneath foundation/architecture/execution/
   reference; declare the single posture `SPINE-CERT scoped remediation` and explain it is the
   admissible remediation that flips the line to `SPINE-CERT passed` via the replacement artifact
   (settled intentions 2). State the non-executable character explicitly.
2. **Source discipline, freshness & admissibility** — pin to `9648c8f`; treat 0038's provenance commits
   as audit provenance only; state the admissibility lock (only SPINE-CERT remediation is admissible
   until the replacement artifact passes; EPI-CERT and beyond are blocked); confirm P0-CERT is already
   passed (0037) and not re-opened (settled intentions 11).
3. **Authority & dependencies** — the controlling foundation/architecture/execution/reference/spec docs
   (the §2 primary set); the 0038 spec, 0038 acceptance artifact, and 0038 triage register as the
   remediated source (history/context); the 0037 spec/artifact as structural precedent; archived specs
   cited as history only; the code/test/config seams in §2.
4. **Problem statement** — the 0038 `SPINE-CERT scoped remediation` verdict: Wave A guarded continuity
   passed with no survivors, but Wave B (run with `--no-config` to expand past the configured perimeter)
   found 296 missed mutants across the SPINE seam files, blocking any `SPINE-CERT passed` claim and the
   advance to EPI-CERT. The configured perimeter never covered the SPINE seams, so the mutation
   lock-layer is structurally incomplete for SPINE-CERT (cite `SPEC_LEDGER.md`, the 0038 acceptance
   artifact, the triage register, and `docs/2-execution/03` gate-failure handling). Name the responsible
   failing layers per the triage register's per-file mapping.
5. **Remediation approach** — the method: (a) **permanently widen the configured `.cargo/mutants.toml`
   perimeter + CI command** to the full SPINE seam set, with a defined run/timeout/sharding strategy and
   no-silent-exclusion discipline, then run to completion (settled intention 4); (b) a survivor-triage
   protocol — kill-with-behavior/provenance-coverage default, grouped behavior-contract test families
   permitted (with the requirement that each group test catches every grouped member), proven-equivalent
   the rare reviewer-signed exception, all recorded in a triage register that the replacement artifact
   embeds/links (settled intention 5); (c) per-survivor-group obligations mapped to seams and diagnostic
   layers (use the triage register's 18-row grouping as the spine of this section, but require the
   implementing session to re-enumerate under the new standing config — the 296 count is a floor); (d)
   how the SPINE-01…SPINE-08 seam contract is re-proven live at `9648c8f` and what (if anything) is
   reused, declared explicitly (settled intention 7); (e) how staged abstractions (if any) are declared
   and bounded (they may bound future EPI-CERT/institutions/LOD/LLM surfaces; they may **not** defer any
   SPINE survivor disposition, replay/provenance, debug quarantine, or SPINE-01…SPINE-08 obligation).
6. **Per-finding/per-group & per-seam deliverables** — a first-class block per survivor group
   (the 18 triage-register files, grouped by responsible SPINE seam and diagnostic layer): the seams,
   the kill-default obligation, the behavior/provenance witness or grouped-contract-test family design
   that defeats tautology, any negative/contamination control, and the responsible layer named; plus,
   for each SPINE seam (SPINE-01…SPINE-08), how the replacement artifact re-establishes its evidence at
   `9648c8f` — concrete seams, the canonical gate(s) composed, positive fixtures, adversarial/negative
   fixtures that must fail, event/replay/projection evidence, and the typed failure diagnostic that
   names the responsible layer. Include every "Gate evidence requirements" element from
   `docs/2-execution/03` (settled intention 9).
7. **Replacement acceptance / evidence artifact** — the conforming replacement acceptance artifact
   specified against the `docs/4-specs/0003` template's fields (settled intention 6), including the
   completed-mutation `pass` row with run-completion + full survivor-disposition counts (no untriaged
   survivors), the per-seam SPINE-01…SPINE-08 evidence sections, the observer-only `EMERGE-OBS` artifact
   where the corpus exercises first-proof living-world acceptance, any staged-abstraction declarations,
   the `SPINE-CERT passed` verdict, and the explicit supersession of the 0038 artifact.
8. **Invariants alignment** — which constitutional invariants the remediated seams and each re-proven
   SPINE seam preserve (cite `INV-NNN` as cross-references; coin none).
9. **Out of scope / deferrals** — the locked EPI-CERT / ORD-LIFE-CERT / Phase-4 / second-proof surfaces
   (settled intention 12), each tied to its named entry gate; an explicit statement that no forward gate
   is advanced by this spec; and the note that survivor files which also feed future gates are
   remediated only for their SPINE-seam role here.
10. **Risks & open questions** — remediation risks (tautological/getter-asserts-getter killing tests;
    the equivalent-mutant problem at 296-survivor scale; grouped-test families that catch a
    representative but not every member; a complete run over the expanded perimeter surfacing *more*
    survivors that re-block the line; mutation-run cost/timeout/sharding consistency; baseline-file abuse;
    metric-gaming a subset; source-discipline relapse), and owner decisions you cannot settle from the
    docs (e.g. shard count, timeout policy, exact replacement-artifact filename, mutation-artifact
    retention format, reviewer-signoff procedure for equivalence dispositions) — carry as open questions,
    do not invent.

Do **not** write final ticket breakdowns (the repo's spec-to-tickets process owns that), invent new
gate/obligation codes, status enums, or finding IDs, or assert a certified pass/fail result.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

## 8. Self-check (run before returning)

- [ ] The deliverable is exactly one new spec at `specs/0039_…` (or a justified title variant with the
      same prefix/path), in the sibling-spec structural convention — not the narrative foundation-doc
      style.
- [ ] The spec's posture is declared as `SPINE-CERT scoped remediation`, addressing the 296 Wave B
      mutation survivors, and it explains how it flips the line to `SPINE-CERT passed` via the
      replacement artifact.
- [ ] The spec mandates permanently widening the configured `.cargo/mutants.toml` perimeter + CI command
      to the full SPINE seam set, with no-silent-exclusion discipline, and requires the run to complete
      over that perimeter — treating the 296 count as a floor, not the finish line.
- [ ] Every surviving mutant must be triaged: kill-with-behavior/provenance-coverage by default;
      grouped behavior-contract test families are permitted only when each group test catches every
      grouped member; equivalence/non-criticality is the rare exception with exact call sites + reviewer
      signoff; no untriaged survivors in a passing artifact.
- [ ] Killing tests assert a real certified-behavior consequence with replay/provenance ancestry and,
      where the firewall is implicated, a negative/contamination control — never a tautology or a
      getter-asserts-getter test.
- [ ] All eight SPINE seams (SPINE-01…SPINE-08) are re-proven live at `9648c8f`, with the 0038 per-seam
      evidence treated as history not live certification.
- [ ] The canonical gates (`TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`,
      `DIAG`) and the `02` acceptance-contract gates are composed under the `SPINE-CERT` phase-label, not
      redefined; no new gate code, obligation code, status vocabulary, or finding ID is minted;
      `EMERGE-OBS` is observer-only.
- [ ] Every "Gate evidence requirements" element from `docs/2-execution/03` is present, and each survivor
      group's responsible failing layer is named per "Gate failure handling".
- [ ] The replacement acceptance artifact conforms to the `docs/4-specs/0003` template fields, carries
      the completed-mutation evidence with full disposition counts, renders `SPINE-CERT passed`, and
      explicitly supersedes the 0038 artifact.
- [ ] Scope is the SPINE-CERT line only; EPI-CERT / ORD-LIFE-CERT / institutions / wrong-suspicion /
      notices / travel / regional / LOD / LLM / story-sifting appear only as named deferrals; no forward
      gate is advanced; survivor files that also feed future gates are remediated only for their SPINE
      role.
- [ ] No new doctrine weakens an upstream tier; the spec stays subordinate to
      foundation/architecture/execution/reference.
- [ ] Source discipline holds: pinned to `9648c8f`; 0038's provenance commits treated as 0038's own;
      archived specs/tickets (0005–0038) cited as history only; manifest treated as path inventory only.
- [ ] Any preliminary static seam survey (if included) is explicitly labeled non-certifying.
- [ ] Every external claim that shaped a decision is cited.
- [ ] Commit `9648c8f` contains every file named in §2.
