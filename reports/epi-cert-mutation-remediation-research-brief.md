# Research brief — EPI-CERT mutation remediation & replacement certification spec (the next Tracewake spec)

> **You are ChatGPT-Pro, Session 2.** This prompt is self-contained. Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a requirement
> impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-19_7a17447.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first; agents act from
partial belief and every event leaves a replayable trace). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; **earlier tiers
govern later ones.** Fetch every file from commit **`7a17447`** (verified repo `HEAD`,
`Merge pull request #49 … implemented-0040`) — the manifest reflects exactly that tree. Do not adopt
a different "commit of record" from any file you read: commit hashes that appear *inside* archived
specs / acceptance artifacts / triage registers (e.g. `92ba47f`, `ba9fe1c`, `b03ceed`, `2a37b04`)
are that artifact's own audit/spec provenance, not this baseline. A manifest is path inventory only.

**This is a delta successor, not a cold start, and it is one turn of a repeating certification
pattern.** The repo runs a phase-certification ladder (`docs/2-execution/03`):
`P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT → FIRST-PROOF-CERT → PHASE-4-ENTRY →
SECOND-PROOF-ENTRY`. Each gate so far has come in *pairs*: an **audit** spec, then — when the audit's
mutation expansion leaves a survivor floor — a **mutation-remediation + replacement-certification**
spec that kills/triages the survivors, re-proves the seam contract, and produces a replacement
acceptance artifact rendering the gate `passed`. The relevant lineage:

- **Lineage predecessor (your subject): the EPI-CERT audit.** The spec
  `archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`
  defined the EPI-CERT audit (numbered audit points `EPI-01 … EPI-NN` across holder-known/actor-known
  contexts, beliefs, observations, provenance, possession parity, embodied/debug view models, and
  debug quarantine). Its acceptance artifact
  `archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`
  rendered **`EPI-CERT scoped remediation`**, **not** `EPI-CERT passed`, because the configured EPI
  mutation expansion (`MUT-WAVEB-001`) left a **30-mutant survivor floor**, recorded in
  `reports/0040_epi_cert_mutation_final_missed.txt` and
  `reports/0040_epi_cert_mutation_triage_register.md`. **This spec remediates exactly that floor.**

- **Structural template (your near-exact shape model — not a delta seed).** The SPINE-CERT
  mutation-remediation spec
  `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` and the
  P0-CERT mutation-remediation spec
  `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` are the
  prior turns of the *same* mutation-remediation pattern. `0039` remediated the 0038 SPINE-CERT audit's
  Wave-B survivor floor and rendered `SPINE-CERT passed` via
  `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`;
  `0037` did the same for the 0036 P0-CERT audit. **`0039` is the closest model for your deliverable —
  mirror its section shape and rigor**, adapting "SPINE / 296 survivors found outside the perimeter" to
  "EPI / 30 survivors found *inside* the already-expanded perimeter" (see §3.3 for the central
  divergence).

The repository's own doctrine names the single admissible next move. `docs/4-specs/SPEC_LEDGER.md`
("Next known execution move") states it plainly: the EPI-CERT artifact "renders `EPI-CERT scoped
remediation`, not `EPI-CERT passed`, because the configured EPI mutation perimeter left a 30-mutant
survivor floor. The next known execution move is a separately numbered EPI-CERT mutation-remediation
and replacement-certification spec; later specs may cite `EPI-CERT passed` only after such a
replacement artifact passes with exact scope." The 0040 artifact's own verdict adds that later specs
**may not** cite `EPI-CERT passed` or proceed to `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, Phase-4 entry,
or later gates until scoped remediation resolves the survivor posture. Therefore ORD-LIFE-CERT, the
`0035` route-forward expansion backlog, institutions, notices, second proof, and every other forward
move are **doctrinally blocked** and cannot be the next spec. `P0-CERT passed` (0037) and
`SPINE-CERT passed` (0039) are settled context this spec consumes, not re-opens. Your task is **not**
to re-decide what is next — it is to author the EPI-CERT mutation-remediation-and-replacement-
certification spec itself.

Source discipline (`SPEC_LEDGER.md`): a commit hash named inside a spec or report is audit/spec
provenance only. The 0040 spec and artifact pin themselves to `ba9fe1c…`; **treat that as 0040's own
audit provenance, not this remediation's baseline.** This remediation is pinned to `7a17447`. The 0040
spec/report/tickets were archived (staging → `archive/`) at or before this `HEAD`; the epistemics seam
source files and the doctrine read-list below are the live tree at `7a17447`, and the 30-survivor floor
is the live blocker at this baseline.

## 2. Read in full (authority order)

Read these before producing. Order is by authority tier. The 0040 seed artifacts, the 0039/0037
template, and the execution tier are the heart of this target. Every path below is confirmed present at
`7a17447`.

**Seed artifacts — primary (the failed certification this spec remediates; read these first):**

```
reports/0040_epi_cert_mutation_triage_register.md — THE survivor inventory: the expanded configured posture (the additive .cargo/mutants.toml expansion to epistemics/**, 48→54 files, 2763 Wave-B mutants), the exact run commands, the 30-mutant unique missed floor with per-row EPI cross-ref + responsible-layer + evidence reference, and the fingerprint table. This is your concrete work-list; the spec turns it into an executable remediation contract.
reports/0040_epi_cert_mutation_final_missed.txt — the canonical list of the 30 unique missed mutants (path:line:operator). The literal floor the remediation must drive to zero-untriaged.
archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md — THE failing artifact: the `EPI-CERT scoped remediation` verdict, the EPI-01…EPI-NN per-seam evidence sections, the evidence-status ledger legend, and the conforming acceptance-artifact shape your replacement must match and supersede.
archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md — the spec being remediated: the EPI-CERT audit contract, the EPI-01…EPI-NN seam inventory and their audited files, the positive/adversarial fixture corpus, the gate composition, the mutation posture, and the section shape your 0041 spec extends and re-runs to completion.
docs/4-specs/SPEC_LEDGER.md — the live ledger: the 0040 row (verdict `EPI-CERT scoped remediation`), the source-discipline rules, the archived-spec posture, and the "Next known execution move" that defines this spec's mandate.
```

**Structural template — primary (the near-identical predecessor remediations; mirror their rigor and section shape):**

```
archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md — the closest structural model: status + admissibility posture, source-discipline/freshness, authority & dependency declarations, problem statement, remediation approach (full posture run to completion, survivor-triage protocol, grouped behavior-contract test families, both-arm disposition, re-proof of the seam contract), per-finding/per-group deliverables, gate-evidence + re-proof matrix, replacement-acceptance-artifact spec, invariants alignment, out-of-scope/deferrals, risks & open questions, completion checklist. Adapt "SPINE-01…SPINE-08 / 296 survivors" to "EPI-01…EPI-NN / 30 survivors."
archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md — the replacement artifact 0039 produced: the model for how a completed-mutation `pass` row and run-completion + survivor-disposition statement read in practice, and how it supersedes the failed audit artifact.
archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md — the original mutation-remediation template (one survivor); useful for cross-checking the established remediation anatomy.
archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md — the first passing replacement artifact in this pattern.
reports/0039_spine_cert_mutation_triage_register.md — the survivor triage-register format the EPI remediation register should reuse (alongside the 0040 register, which is already in this format).
```

**Foundation — primary (the constitution and the properties the surviving mutants threaten):**

```
docs/README.md — authority order and the layering rule (foundation > architecture > execution > reference > specs).
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md — INV-001…INV-112; the subjective-epistemics, no-contamination, provenance, possession-parity, replay, and debug-quarantine invariants the EPI seams uphold and the survivors threaten.
docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md — event authority, replay, traceability, snapshots, randomness; the trace/replay basis the epistemic projection rebuilds from and the contract behind the freshness/observation-id/contradiction-id survivors.
docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md — THE foundation contract for claims, propositions, beliefs, memory, observation channels, and subjective information flow EPI-CERT certifies; the doctrine behind the belief/observation/proposition/contradiction survivors.
docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md — possession parity, actor-filtered view models, debug quarantine; behind the render/view-model survivors.
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md — "truth may validate, truth may not plan"; the firewall behind the sealed-context and projection survivors.
```

**Architecture — primary (subsystem contracts the survivor-killing coverage measures code against):**

```
docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md — universal conformance questions, the replacement/retirement rule, and the canonical gate/review-artifact composition the EPI-CERT label composes.
docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md — the projection-rebuild contract: the epistemic projection is a replay-derived projection, never a truth writer; the contract for the replay/parse-arm/deserialize-arm survivors.
docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md — holder-known/actor-known sealed contexts, the truth firewall, provenance sufficiency/freshness; the heart of EPI-CERT and the contract behind the provenance-linkage / freshness survivors.
docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md — THE architecture contract for beliefs, observation channels, memory traces, contradiction/absence detection, confidence, and privacy-scoped information flow; the contract behind most of the 30 survivors.
docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md — possession parity, embodied/debug view-model split, debug-only truth, client boundaries; the contract for the rendering survivors.
docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md — acceptance artifacts, anti-contamination tests, diagnostics, review checklists; the home of the lock-layer/mutation posture.
```

**Execution — primary (the EPI-CERT contract, the remediation posture, and the gate-failure rule):**

```
docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md — canonical gate names (TFW, PIPE, NO-DIRECT, NO-HUMAN, POS-PARITY, REPLAY, FIXTURE, DIAG), the phase-certification-label class (EPI-CERT composes gates but is not a new canonical gate), the EMERGE-OBS observer-only obligation, label reconciliation, universal execution posture.
docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md — archived-spec status, current code-audit boundary, and the three admissibility postures (`passed` / `scoped remediation` / `not applicable`).
docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md — first-proof acceptance contract the replacement artifact still composes.
docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md — gate order (EPI-CERT is gate 3), the "Gate evidence requirements" list, "Gate failure handling" (a failed gate must produce a remediation spec/report naming the failing layer), and the four valid future-spec postures (Remediation is yours).
docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md — truth-firewall execution checks and mandatory anti-contamination gates (TFW); the seam class many survivors live in.
docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md — THE EPI-CERT execution proof doc: epistemic view-model proof, possession parity, and debug-quarantine proof obligations the re-proof must satisfy.
docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md — golden fixture families, adversarial scenarios (incl. hidden-truth / contradiction fixtures), deterministic replay acceptance (FIXTURE, REPLAY).
docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md — testing, diagnostics by responsible layer, review evidence, the mutation/lock-layer posture, the evidence-honesty rule, and the EMERGE-OBS observation obligation (DIAG). The home of "what the configured mutation posture is."
```

**Reference + specs — primary (rules, numbering, source discipline, structural models):**

```
docs/4-specs/README.md — future-spec rules: declare authority posture, declare one admissibility posture, gate codes as cross-references only, holder-known/actor-known terminology, source discipline, no symmetry files.
docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md — the acceptance-artifact template (evidence-status / fingerprint-scope / behavior-witness / replay-provenance / sampling-exhaustiveness / pending-historical / certification-use / staged-abstraction fields). The replacement certification artifact MUST conform to this.
docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md — the live first-proof ontology/fixture contract; the system the certification line covers (boundary-awareness for fixture realism, not an audit target).
```

**Reference — boundary-awareness (read to bound scope; NOT conformance targets):**

```
docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md — review guardrails and gate-code lookup posture.
docs/3-reference/01_DESIGN_RISK_REGISTER.md — the post-0008 relapse risk (treating archived 0004–0040 as certification) and the anti-Goodhart watch note the remediation must actively resist (do not let "kill a few named mutants" or "accept survivors as baseline misses" substitute for completing the run and triaging every survivor).
docs/3-reference/02_GLOSSARY.md — prescriptive terminology control (holder-known context, actor-known, observation, belief, contradiction, debug truth, EMERGE-OBS, source discipline).
```

**Boundary-awareness — OUT of scope (read only to know what this spec must NOT touch or advance into):**

```
docs/2-execution/05,06,08,11,12,13 — scheduler/no-direct-dispatch (SPINE, passed); ordinary-life needs/routines (ORD-LIFE); data authoring/schema; institutions/Phase-4 entry; deferred second-proof/notices/travel/LOD; research notes. Read to bound scope; not EPI-CERT remediation targets.
docs/0-foundation/{01,05,06,07,09,10,11,12,13} and docs/1-architecture/{01,04,05,07,08,09,11,12,14} — charter, needs/routines/planning, actions/survival, institutions, no-scripting authoring, LOD/regional scale, LLM/speech, first-playable scope, research notes: future-phase, deferred, or already-certified-elsewhere surfaces. (SPINE-CERT already owns the event/replay/pipeline spine; this spec remediates only the EPI-seam mutation survivors, even where a survivor file also feeds the spine.)
```

**Code + config — primary exploration targets (read the crates directly; these are the remediation seams):**

The workspace has three crates with strict one-way dependency direction: `tracewake-core`
(authoritative kernel, zero deps) → `tracewake-content` (fixtures, loading, schema validation,
depends on core) → `tracewake-tui` (possession, view models, depends on core + content). The 30
survivors are concentrated in the core epistemics module. Per
`reports/0040_epi_cert_mutation_triage_register.md`, the survivor-bearing files are:

```
crates/tracewake-core/src/epistemics/proposition.rs   — the largest survivor cluster: parse/deserialize match-arm deletions (item_carried_by_actor, container_contents_observed, possible_movement_near_place, at_place, carried_by), Display→Ok(default) for Proposition/PropositionParseError/PropositionReferenceError, render_location→xyzzy/empty, validate_location/require_place/require_container→Ok(()), and == / && / || operator flips in contradiction detection.
crates/tracewake-core/src/epistemics/belief.rs        — stale_after_tick→None/Some(default) (freshness policy), observation_ids→empty set, contradiction_ids→empty set (provenance/contradiction linkage).
crates/tracewake-core/src/epistemics/contradiction.rs — is_active→true, || → && in detect_expected_absences (absence/contradiction detection).
crates/tracewake-core/src/epistemics/observation.rs   — parts_per_thousand→0/1, is_low→true (observation confidence).
```

Read these four files fully and follow their dependencies outward to the rest of the epistemic seam:

```
crates/tracewake-core/src/epistemics/{projection.rs,knowledge_context.rs,knowledge_basis.rs,mod.rs} — how propositions/beliefs/observations/contradictions flow into the EpistemicProjection (replay-derived, non-truth-writer) and the sealed holder-known context; the seam the killing tests must witness behavior through, not around.
```

Other seams to inspect (provenance/replay ancestry, lock-layer behavior witnesses, configured posture):

```
Mutation posture configuration:   .cargo/mutants.toml (the configured perimeter — already additively expanded to crates/tracewake-core/src/epistemics/** per the 0040 register, 54 files), .github/workflows/ci.yml (the scheduled-CI mutation command), and any .cargo/mutants-baseline-misses.txt the tree carries (accepted baseline misses — the 30 EPI survivors are NOT accepted baseline misses). Establish what "the full configured posture, run to completion over the standing epistemics perimeter" means, confirm the perimeter already covers the survivor files, and verify no survivor file is silently dropped by an exclusion glob.
Lock-layer / behavior witnesses:  crates/tracewake-core/tests/{hidden_truth_gates.rs, event_schema_replay_gates.rs, acceptance_gates.rs, anti_regression_guards.rs, generative_lock.rs, golden_scenarios.rs, negative_fixture_runner.rs, spine_conformance.rs, no_human_capstone.rs, emergence_ledger.rs}; crates/tracewake-content/tests/{fixtures_load.rs, forbidden_content.rs, golden_fixtures_run.rs, schema_conformance.rs}; crates/tracewake-tui/tests/{adversarial_gates.rs, tui_seam_conformance.rs, transcript_snapshot.rs, tui_acceptance.rs, embodied_flow.rs, command_loop_session.rs} — where new survivor-killing tests belong, and how existing tests already exercise (or miss) each seam.
```

## 3. Settled intentions (final — these are decisions, not options)

1. **The deliverable is the 0041 EPI-CERT mutation-remediation & replacement-certification spec.**
   The "what is next" determination is **doctrine-forced and closed** by `SPEC_LEDGER.md`'s "Next known
   execution move" and the 0040 artifact's admissibility lock. Do NOT re-survey for alternative
   next-specs; do NOT propose ORD-LIFE-CERT, the `0035` expansion backlog, institutions, notices,
   second proof, or any feature/gameplay work — they are *inadmissible* for the certification ladder
   until the replacement `EPI-CERT passed` artifact exists. Your research budget goes entirely to **how
   to remediate and re-certify**, not **what** to build. Open the spec with a short, cited confirmation
   of this determination (gate ladder `docs/2-execution/03` + ledger).

2. **Posture = `Remediation`** (per `docs/2-execution/03`'s valid postures: it fixes the named failed
   gate — the 30 EPI-CERT mutation survivors), with replacement-certification character: it also
   *produces the replacement EPI-CERT acceptance artifact* that supersedes the 0040 artifact and renders
   `EPI-CERT passed`. Declare the single posture explicitly per `docs/4-specs/README.md`. This spec's own
   admissibility posture is **`EPI-CERT scoped remediation`** — it *is* the admissible remediation the
   0040 artifact demands, and its successful execution is what *flips the line to* `EPI-CERT passed` via
   the replacement artifact. The header/admissibility block must cite **both** the 0037 acceptance
   artifact (`P0-CERT passed`) **and** the 0039 acceptance artifact (`SPINE-CERT passed`) as the
   consumed predecessor gates; this spec does not re-open them.

3. **The perimeter is already standing — focus the work on kill + re-proof, run to completion, triage
   every survivor.** This is the central divergence from `0039`. The 0039 SPINE survivors (296) were
   found *outside* the configured perimeter via `--no-config`, so 0039's central mandate was permanently
   *widening* the perimeter. Here, the 0040 audit **already additively expanded** the checked-in
   `.cargo/mutants.toml` to cover `crates/tracewake-core/src/epistemics/**` (48→54 files; Wave-B census
   2763), and the 30 survivors were found *within* that standing configured posture (the final Wave-B run
   carried no `--no-config` flag). Therefore the spec must:
   (a) **maintain** the already-standing epistemics perimeter (no shrinking; no-silent-exclusion
   discipline preserved; confirm every survivor file remains in the configured census and is not laundered
   into `.cargo/mutants-baseline-misses.txt`), and **verify/confirm** it against the live tree at
   `7a17447` rather than re-deriving a fresh expansion;
   (b) require running the standing configured mutation posture **to completion** and **re-enumerating**
   the survivor floor under it — treat the 30-survivor inventory in
   `reports/0040_epi_cert_mutation_triage_register.md` as the **seed work-list, not the finish line**:
   the count is a floor, and a complete re-run after the killing tests land must show zero untriaged
   survivors;
   (c) triage **every** surviving mutant — each is either killed with behavior/provenance coverage or
   proven-and-recorded as equivalent/non-critical with explicit reviewer-signed rationale.
   Confirm whether the standing perimeter (54 files) genuinely covers all 30 survivor files; if you find
   a survivor whose file is *not* in the configured census, flag it and require its addition (a minimal,
   justified perimeter correction — not a wholesale 0039-style re-derivation).

4. **Kill-with-coverage is the default disposition; grouped behavior-contract tests are permitted.**
   For all survivors the default disposition is **kill with behavior/provenance coverage**; proven
   equivalence/non-criticality is the **rare exception** and requires a reviewable argument naming exact
   call sites plus reviewer signoff (a single implementer's "no test failed" is never sufficient). The
   30-survivor floor is small enough that most are individually killable, but the spec **may permit
   grouping kindred survivors under shared behavior-contract test families** — e.g. the
   `proposition.rs` parse/deserialize match-arm deletions under round-trip serialize/deserialize/apply
   contract tests; the `Display → Ok(default)` mutants under canonical-rendering contract tests; the
   `render_location → xyzzy/empty` and `validate_location/require_place/require_container → Ok(())`
   mutants under view-model-rendering and content/schema-validation contract tests; the contradiction
   `==/&&/||` operator flips and `detect_expected_absences` `||→&&` under contradiction/absence-detection
   contract tests; the `belief.rs` freshness/linkage mutants under projection-freshness and
   provenance-linkage contract tests; the `observation.rs` confidence mutants under observation-confidence
   contract tests. A grouped test family is acceptable **only if** it genuinely fails for every member
   mutant (the spec must require the group test demonstrably catches each grouped survivor, not merely a
   representative). Grouping is a scale-management tool, not a coverage shortcut.

5. **Re-prove, do not merely re-cite, the EPI-01…EPI-NN seam contract.** The replacement artifact's
   `EPI-CERT passed` verdict depends on the full EPI seam contract, not the mutation survivors alone. The
   spec must state how the EPI-01…EPI-NN evidence (per the 0040 spec and its acceptance artifact's
   per-seam sections — covering holder-known/actor-known contexts, beliefs, observations, provenance,
   possession parity, embodied/debug view models, and debug quarantine) is re-established live at
   `7a17447`. The 0040 per-seam pass rows are *historical evidence* under source discipline (and
   reference 0040's own `ba9fe1c…` provenance), not live certification, though they may be cited as prior
   context and structural shape. Scope the re-proof proportionately (the EPI seams largely held in 0040;
   the gap was mutation coverage, not seam failure) and declare any reuse explicitly. The mutation
   remediation strengthens exactly the behavior witnesses these seams rely on, so the two efforts are
   coupled, not parallel.

6. **Produce the replacement certification artifact spec.** The spec must define a replacement
   acceptance artifact that (a) conforms to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
   (evidence-status, fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness,
   pending/historical, certification-use, staged-abstraction declaration); (b) carries the completed
   mutation evidence as a `pass` row with explicit run-completion and full survivor-disposition counts
   (caught / killed-by-remediation / equivalent / non-critical, zero untriaged); (c) renders verdict
   `EPI-CERT passed` for the epistemic certification line; and (d) explicitly **supersedes**
   `archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`,
   after which later specs may cite `EPI-CERT passed` and the ladder may advance to ORD-LIFE-CERT.

7. **Compose existing canonical gates and the EPI-CERT phase-label; mint nothing new.** Express the
   work through the existing canonical gates — `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`,
   `REPLAY`, `FIXTURE`, `DIAG` (defined in `docs/2-execution/00`) — composed under the `EPI-CERT`
   phase-certification label, plus the `02` first-proof acceptance-contract labels. Name gate codes only
   as cross-references. Define **no** new gate code, observation obligation, status vocabulary, invariant
   ID, or finding ID. Survivors are identified by cargo-mutants output (path:line:operator) and the triage
   register, not by minted doctrine-level IDs. Treat `EMERGE-OBS` correctly: an observer-only observation
   obligation, never a pass/fail threshold, mutation score, or phase gate.

8. **Include every "Gate evidence requirements" element** from `docs/2-execution/03`: exact files/seams
   audited; foundation & architecture dependencies; artifact dependencies (including the observer-only
   `EMERGE-OBS` artifact where the corpus exercises first-proof living-world acceptance); positive and
   negative fixtures (incl. hidden-truth / contradiction / possession-parity-violation fixtures);
   event/replay/projection evidence; actor-known provenance evidence; debug-quarantine evidence; failure
   diagnostics by responsible layer (the triage register already assigns these per survivor — projection/
   replay, freshness policy, provenance/contradiction linkage, content/schema validation, view-model
   rendering, proposal construction, diagnostics); a statement that archived specs/tickets (0004–0040)
   are used only as historical evidence; and a list of tolerated deferrals tied to named gates. Honor
   "Gate failure handling": map each survivor group to its responsible failing layer.

9. **Follow the sibling-spec structural convention, not the foundation-doc house style.** Mirror the
   section shape and rigor of `archive/specs/0039_*` (closest model) and `0037_*`, and the EPI-seam
   inventory of `0040_*`: status + admissibility posture, source-discipline/freshness, authority &
   dependency declarations, problem statement, remediation approach, per-finding/per-group and per-seam
   deliverables, gate-evidence + re-proof matrix, replacement-acceptance-artifact spec, invariants
   alignment, out-of-scope/deferrals, risks & open questions, completion checklist. Adapt any section that
   does not fit a 30-survivor remediation and say why.

10. **Source discipline (`SPEC_LEDGER.md`).** A commit hash named inside a spec or report is audit/spec
    provenance only. A manifest is path inventory only. Branch names, default-branch lookups, connector
    namespace labels, repository metadata, and code-search snippets are not proof of target-commit
    content. Pin this remediation to commit **`7a17447`**. Treat the 0040 spec/artifact's `ba9fe1c…`
    string (and any older hashes) as their own provenance, not this spec's baseline. Cite archived specs
    `0004`–`0040` and all tickets only as historical evidence, never as live certification.

11. **Scope boundary is the EPI-CERT certification line only.** The certification line covers the
    epistemic substrate: holder-known/actor-known contexts, beliefs, observations, provenance, possession
    parity, embodied/debug view models, and debug quarantine. SPINE-CERT's event/replay/pipeline spine
    (already passed via 0039), ORD-LIFE-CERT, institutions, wrong-suspicion, notices, travel, regional
    scale, LOD, LLM dialogue, and story-sifting are **out of scope** (locked behind their named entry
    gates); the spec names them only as deferrals, never as remediation or audit targets — even where a
    survivor file also feeds a future gate. Remediating an EPI-seam survivor must not be inflated into
    auditing ORD-LIFE-CERT or re-auditing the (passed) SPINE seams.

> `assumption`: the spec is authored at
> **`specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`**, staged under the
> `specs/NNNN_*` series and archived to
> `archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` on acceptance
> closeout (not promoted into live `docs/4-specs/`), mirroring the 0037/0039 hardening-series convention.
> Number `0041` is the next free staging number: the staging series in `archive/specs/` is contiguous
> through `0040` with no recent renumbering or unification (confirmed against `SPEC_LEDGER.md` and
> `git log` — the only recent renames are the routine 0040 staging → archive closeout), and `specs/` is
> currently empty. If you judge a clearer title slug, you may adjust the title words but keep the `0041_`
> prefix and the `specs/` staging path. The ledger row for this spec is added at acceptance/closeout per
> the repo's hardening-series ledger-timing convention, not authored into the spec itself.

## 4. The task

Author the **EPI-CERT mutation-remediation & replacement-certification spec** — the next Tracewake
spec. This is a **new-spec** target with a **hardening / anti-contamination** character and a
**`Remediation`** posture. The 0040 EPI-CERT audit rendered `EPI-CERT scoped remediation` because its
configured mutation expansion left a **30-mutant survivor floor** across the core epistemics module
(`reports/0040_epi_cert_mutation_triage_register.md` / `reports/0040_epi_cert_mutation_final_missed.txt`),
blocking any `EPI-CERT passed` claim and the advance to ORD-LIFE-CERT. The spec must turn the ledger's
"Next known execution move" into a concrete, executable remediation plan: how to maintain and re-run the
already-standing configured epistemics mutation perimeter to completion; how to triage every surviving
mutant (kill with behavior/provenance coverage as the default, grouped behavior-contract test families
permitted for kindred survivors; equivalence the rare reviewer-signed exception); the per-survivor-group
obligations mapped to their responsible seams and diagnostic layers; how the EPI-01…EPI-NN seam contract
is re-proven live at `7a17447`; and the shape of the replacement acceptance artifact that renders
`EPI-CERT passed` and supersedes the 0040 artifact. It is a requirements/plan document that the repo's
reassess → ticket → implement process will execute — **not** a rendered certification result.

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — read the survivor-bearing
epistemics files (`proposition.rs`, `belief.rs`, `contradiction.rs`, `observation.rs`) and the
surrounding seam (`projection.rs`, `knowledge_context.rs`, `knowledge_basis.rs`) and follow
dependencies outward (especially how propositions/beliefs/observations/contradictions flow through the
`EpistemicProjection` into the embodied view models, how proposition parse/deserialize/render arms reach
replay/serialization round-trips, and how freshness/confidence/contradiction logic governs belief
state) so every remediation claim is grounded in real module and function names.

**Research online as deeply as needed** (the user explicitly encourages deep external research — similar
implementations, research papers, and whatever is beneficial): mutation-testing theory and practice
(cargo-mutants configuration, perimeter/filters, timeouts, sharded/incremental runs, baseline
discipline, configured-posture vs. `--no-config`), **the equivalent-mutant problem and equivalent/
redundant-mutant detection**, designing behavior tests that kill "replace return value with
`None`/`Some(Default::default())`/empty collection/empty string/`xyzzy`/`Ok(())`/`Ok(default)`", "delete
match arm", and "flip `==`/`&&`/`||` operator" mutants **without tautology**, behavior-contract /
parameterized test families for survivor clusters, mutation testing as a certification lock-layer /
coverage-adequacy criterion, **subjective belief/knowledge modeling** in agent simulations (epistemic
logic, belief revision, partial observability), **information-flow / non-interference** proof techniques
(proving a hidden-truth channel cannot leak into an actor-visible surface), property-based and
metamorphic testing for belief/observation/contradiction projections and possession-parity invariants,
provenance/witness-chain integrity, and audit-evidence packaging for safety-style certification —
wherever it sharpens the remediation method, the triage protocol, the killing-test and test-grouping
design, or the replacement evidence package. **Cite every external source that shapes a decision.** The
deep research is your job; this brief only commissions it.

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every gate the replacement
  artifact certifies must satisfy it; a genuine divergence would require amending an invariant first —
  never design the remediation against an invariant silently. The spec is subordinate doctrine: it
  operationalizes the higher tiers and may not amend, replace, weaken, or redefine them.
- **Authority order:** if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong; if a test proves
  only plausible behavior while bypassing holder-known provenance, the test is wrong. A killing test that
  asserts a tautology (e.g. only that a parser returns a non-empty result, or that a confidence getter
  returns the literal it returns) is insufficient — it must witness a real certified-behavior consequence
  with provenance/replay ancestry. This is acute for the `Display → Ok(default)`, `render_location →
  xyzzy/empty`, `parts_per_thousand → 0/1`, and `is_active/is_low → true` survivors, which invite
  tautological getter-asserts-getter tests; the spec must demand behavior witnesses (e.g. a confidence
  mutation must change a belief stance / contradiction outcome / view-model surface that a replay or
  serialization round-trip observably depends on; a deleted parse arm must break a proposition
  round-trip the projection rebuild relies on).
- **The truth firewall is the spine:** *truth may validate actions, but truth may not plan them.* The
  survivors live in the epistemic substrate (propositions, beliefs, observations, contradictions) that
  feeds the sealed actor-known context and the embodied projection; killing/equivalence evidence must
  respect that the projection supplies sealed actor-known facts with provenance, and debug truth must
  never reach embodied views. A quick fix that satisfies a view assertion by sourcing a fact from truth
  or debug is forbidden; killing tests need negative/contamination controls where the firewall is
  implicated.
- **Anti-contamination:** no simulation fact may be born from prose. Preserve event-sourced causality,
  subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads,
  validation/replay, the holder-known truth firewall. Adversarial/negative coverage that proves forbidden
  shortcuts FAIL is first-class evidence (especially for the `validate_location/require_place/
  require_container → Ok(())` validator survivors, which must reject forbidden/under-specified
  propositions). Actively resist the post-0008 relapse risk in `docs/3-reference/01` (treating archived
  work as certification) and its anti-Goodhart note (do not let "kill a representative subset" or "accept
  survivors as baseline misses" substitute for completing the run and triaging every survivor).
- **No backwards-compatibility shims or alias paths** in any recommendation; no new files merely for
  symmetry; no test-only branch that bypasses the real seam path to "kill" a mutant.
- The crate dependency direction (`core` ← `content` ← `tui`) must never invert.
- **Terminology:** use `holder-known context` as the system-wide term and `actor-known` for the actor
  case.

## 7. Deliverable specification

Produce **one** downloadable markdown document — a **numbered implementation spec** (this IS a
`specs/`-tier artifact; the numbering/placement assumption in §3 applies). There is **no** paired
`-research-report.md`.

- **`specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`** — the staged spec
  file (archived to `archive/specs/0041_…` on closeout per the hardening-series convention; a justified
  title-slug variant is acceptable if it keeps the `0041_` prefix and `specs/` staging path).

**This deliverable is non-executable.** You cannot run `cargo fmt/clippy/build/test`, `cargo mutants`,
or replay against live code from this session. The spec therefore specifies *what the implementing
session will run and prove, and the acceptance contract it must satisfy* — **not** a rendered pass/fail.
You **MAY** include a clearly-labeled **preliminary static survey** of a few representative survivor
groups (e.g. the `proposition.rs` parse/deserialize match-arm cluster, the `Display → Ok(default)`
cluster, the `belief.rs` freshness/linkage cluster, the contradiction operator-flip cluster) — what
reading the code at `7a17447` suggests about whether each is killable vs. likely-equivalent, where a
killing/contract test belongs, and what behavior witness defeats tautology — as *informative,
non-certifying* evidence, explicitly marked **"preliminary, not certification."** If you judge the
survey would dilute the spec, keep it tight or omit it; the remediation-plan spec is the required core
either way. Authoritative pass/fail belongs to the implementing session that executes the gates.

Required content (adapt section names to the sibling-spec convention in `archive/specs/0039_*`,
`0037_*`, `0040_*`; the substance below is mandatory):

1. **Status + admissibility posture** — declare authority beneath foundation/architecture/execution/
   reference; declare the single posture `EPI-CERT scoped remediation` and explain it is the admissible
   remediation that flips the line to `EPI-CERT passed` via the replacement artifact (settled intention
   2). Cite both `P0-CERT passed` (0037) and `SPINE-CERT passed` (0039) as consumed predecessor gates.
   State the non-executable character explicitly.
2. **Determination confirmation** — short, cited: why this remediation spec is the next admissible move
   (gate ladder `docs/2-execution/03` + ledger "Next known execution move"; 0040 verdict; the
   admissibility lock that blocks ORD-LIFE-CERT and beyond).
3. **Source discipline, freshness & admissibility** — pin to `7a17447`; treat 0040's `ba9fe1c…`
   provenance as audit provenance only; state the admissibility lock (only EPI-CERT remediation is
   admissible until the replacement artifact passes; ORD-LIFE-CERT and beyond are blocked); confirm
   P0-CERT and SPINE-CERT are already passed and not re-opened (settled intentions 2, 10).
4. **Authority & dependencies** — the controlling foundation/architecture/execution/reference/spec docs
   (the §2 primary set); the 0040 spec, 0040 acceptance artifact, and 0040 triage register as the
   remediated source (history/context); the 0039/0037 specs/artifacts as structural precedent; archived
   specs cited as history only; the code/test/config seams in §2.
5. **Problem statement** — the 0040 `EPI-CERT scoped remediation` verdict: the configured posture
   (already additively expanded to `epistemics/**`) was run to completion and `MUT-WAVEB-001` left a
   30-mutant unique survivor floor across `proposition.rs` / `belief.rs` / `contradiction.rs` /
   `observation.rs`, blocking any `EPI-CERT passed` claim and the advance to ORD-LIFE-CERT (cite
   `SPEC_LEDGER.md`, the 0040 acceptance artifact, the triage register, and `docs/2-execution/03`
   gate-failure handling). Name the responsible failing layers per the triage register's per-row mapping.
6. **Remediation approach** — the method: (a) **maintain and re-confirm** the already-standing
   configured `.cargo/mutants.toml` epistemics perimeter + CI command (no shrinking, no-silent-exclusion,
   no baseline-miss laundering; a minimal justified perimeter correction only if a survivor file proves
   uncovered), with a defined run/timeout/sharding strategy, then run to completion and re-enumerate the
   floor (settled intention 3); (b) a survivor-triage protocol — kill-with-behavior/provenance-coverage
   default, grouped behavior-contract test families permitted (with the requirement that each group test
   catches every grouped member), proven-equivalent the rare reviewer-signed exception, all recorded in a
   triage register that the replacement artifact embeds/links and reuses the 0040/0039 register format
   (settled intention 4); (c) per-survivor-group obligations mapped to seams and diagnostic layers (use
   the 0040 register's 30-row inventory as the spine of this section, but require the implementing session
   to re-enumerate under the standing config — the 30 count is a floor); (d) how the EPI-01…EPI-NN seam
   contract is re-proven live at `7a17447` and what (if anything) is reused, declared explicitly (settled
   intention 5); (e) how staged abstractions (if any) are declared and bounded (they may bound future
   ORD-LIFE-CERT/institutions/LOD/LLM surfaces; they may **not** defer any EPI survivor disposition,
   replay/provenance, debug quarantine, or EPI-01…EPI-NN obligation).
7. **Per-finding/per-group & per-seam deliverables** — a first-class block per survivor group (grouped by
   responsible epistemics file and diagnostic layer): the seams, the kill-default obligation, the
   behavior/provenance witness or grouped-contract-test family design that defeats tautology, any
   negative/contamination control, and the responsible layer named; plus, for each EPI seam
   (EPI-01…EPI-NN as defined in the 0040 spec), how the replacement artifact re-establishes its evidence
   at `7a17447` — concrete seams, the canonical gate(s) composed, positive fixtures, adversarial/negative
   fixtures that must fail, event/replay/projection evidence, and the typed failure diagnostic that names
   the responsible layer. Include every "Gate evidence requirements" element from `docs/2-execution/03`
   (settled intention 8).
8. **Replacement acceptance / evidence artifact** — the conforming replacement acceptance artifact
   specified against the `docs/4-specs/0003` template's fields (settled intention 6), including the
   completed-mutation `pass` row with run-completion + full survivor-disposition counts (no untriaged
   survivors), the per-seam EPI-01…EPI-NN evidence sections, the observer-only `EMERGE-OBS` artifact
   where the corpus exercises first-proof living-world acceptance, any staged-abstraction declarations,
   the `EPI-CERT passed` verdict, and the explicit supersession of the 0040 artifact.
9. **Invariants alignment** — which constitutional invariants the remediated seams and each re-proven EPI
   seam preserve (cite `INV-NNN` as cross-references; coin none).
10. **Out of scope / deferrals** — the locked SPINE-CERT (passed, not re-audited) / ORD-LIFE-CERT /
    institutions / Phase-4 / second-proof surfaces (settled intention 11), each tied to its named entry
    gate; an explicit statement that no forward gate is advanced by this spec; and the note that survivor
    files which also feed future gates are remediated only for their EPI-seam role here.
11. **Risks & open questions** — remediation risks (tautological/getter-asserts-getter killing tests; the
    equivalent-mutant problem; grouped-test families that catch a representative but not every member; a
    complete run surfacing *more* survivors that re-block the line; mutation-run cost/timeout/sharding
    consistency; baseline-file abuse; metric-gaming a subset; source-discipline relapse), and owner
    decisions you cannot settle from the docs (e.g. shard count, timeout policy, exact replacement-artifact
    filename, mutation-artifact retention format, reviewer-signoff procedure for equivalence dispositions)
    — carry as open questions, do not invent.

Do **not** write final ticket breakdowns (the repo's spec-to-tickets process owns that), invent new
gate/obligation codes, status enums, invariant IDs, or finding IDs, or assert a certified pass/fail
result.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

## 8. Self-check (run before returning)

- [ ] The deliverable is exactly one new spec at `specs/0041_…` (or a justified title variant with the
      same prefix/path), in the sibling-spec structural convention — not the narrative foundation-doc
      style.
- [ ] The spec's posture is declared as `EPI-CERT scoped remediation`, addressing the 30 EPI mutation
      survivors, and it explains how it flips the line to `EPI-CERT passed` via the replacement artifact;
      the header cites both `P0-CERT passed` (0037) and `SPINE-CERT passed` (0039).
- [ ] The spec maintains (does not re-derive) the already-standing configured `.cargo/mutants.toml`
      epistemics perimeter + CI command, with no-silent-exclusion / no-baseline-laundering discipline,
      and requires the run to complete over that perimeter — treating the 30 count as a floor, not the
      finish line; any minimal perimeter correction (if a survivor file proves uncovered) is justified.
- [ ] Every surviving mutant must be triaged: kill-with-behavior/provenance-coverage by default; grouped
      behavior-contract test families are permitted only when each group test catches every grouped
      member; equivalence/non-criticality is the rare exception with exact call sites + reviewer signoff;
      no untriaged survivors in a passing artifact.
- [ ] Killing tests assert a real certified-behavior consequence with replay/provenance ancestry and,
      where the firewall is implicated, a negative/contamination control — never a tautology or a
      getter-asserts-getter test (acute for the `Display→Ok(default)`, `render_location→xyzzy/empty`,
      `parts_per_thousand→0/1`, `is_active/is_low→true`, and `→Ok(())` survivors).
- [ ] All EPI seams (EPI-01…EPI-NN) are re-proven live at `7a17447`, with the 0040 per-seam evidence
      treated as history not live certification.
- [ ] The canonical gates (`TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`,
      `DIAG`) and the `02` acceptance-contract gates are composed under the `EPI-CERT` phase-label, not
      redefined; no new gate code, obligation code, status vocabulary, invariant ID, or finding ID is
      minted; `EMERGE-OBS` is observer-only.
- [ ] Every "Gate evidence requirements" element from `docs/2-execution/03` is present, and each survivor
      group's responsible failing layer is named per "Gate failure handling".
- [ ] The replacement acceptance artifact conforms to the `docs/4-specs/0003` template fields, carries
      the completed-mutation evidence with full disposition counts, renders `EPI-CERT passed`, and
      explicitly supersedes the 0040 artifact.
- [ ] Scope is the EPI-CERT line only; SPINE-CERT (passed) is not re-audited; ORD-LIFE-CERT /
      institutions / wrong-suspicion / notices / travel / regional / LOD / LLM / story-sifting appear only
      as named deferrals; no forward gate is advanced; survivor files that also feed future gates are
      remediated only for their EPI role.
- [ ] No new doctrine weakens an upstream tier; the spec stays subordinate to
      foundation/architecture/execution/reference; crate dependency direction is preserved.
- [ ] Source discipline holds: pinned to `7a17447`; 0040's `ba9fe1c…` provenance treated as 0040's own;
      archived specs/tickets (0004–0040) cited as history only; manifest treated as path inventory only.
- [ ] Any preliminary static seam survey (if included) is explicitly labeled non-certifying.
- [ ] Every external claim that shaped a decision is cited.
- [ ] Commit `7a17447` contains every file named in §2.
