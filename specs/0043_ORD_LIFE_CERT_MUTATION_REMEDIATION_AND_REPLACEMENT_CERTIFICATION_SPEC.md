# 0043 — ORD-LIFE-CERT mutation remediation and replacement certification spec

**Staging path:** `specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
**Archive path on accepted closeout:** `archive/specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
**Target repository:** `joeloverbeck/tracewake`  
**Authoring and reassessment baseline:** `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd`  
**Spec series:** numbered implementation spec `0043`, staged under `specs/`, archived on accepted closeout  
**Status:** STAGING — non-executable remediation specification; no implementation or certification verdict is asserted  
**Future-spec posture:** `Remediation`  
**Admissibility posture:** `ORD-LIFE-CERT scoped remediation`  
**Consumed predecessor gates:** `P0-CERT passed` through the 0037 replacement artifact; `SPINE-CERT passed` through the 0039 replacement artifact; `EPI-CERT passed` through the 0041 replacement artifact  
**Certification-line effect:** successful execution must publish a replacement acceptance artifact rendering `ORD-LIFE-CERT passed` and explicitly superseding the 0042 ORD-LIFE-CERT acceptance artifact  
**Assumption:** stage under `specs/` through the repository's stage-then-archive convention; relocate only by owner direction, without renumbering or changing semantics

**Determination confirmation.** The live phase ladder places `ORD-LIFE-CERT` after the already-passed `P0-CERT`, `SPINE-CERT`, and `EPI-CERT` lines and before `FIRST-PROOF-CERT`. The live ledger records the 0042 ORD-LIFE-CERT audit as `ORD-LIFE-CERT scoped remediation`, forbids later work from citing it as passed, and names a separately numbered ORD-LIFE-CERT mutation-remediation/replacement spec and evidence package as the next known execution move. The corresponding predecessor replacement artifacts record `P0-CERT passed`, `SPINE-CERT passed`, and `EPI-CERT passed`. Therefore this 0043 remediation spec—not feature expansion, Phase 4, or FIRST-PROOF certification—is the single next admissible move.[^repo-ladder][^repo-ledger][^repo-0037-acceptance][^repo-0039-acceptance][^repo-0041-acceptance][^repo-0042-acceptance]

This spec is subordinate to foundation, architecture, execution, reference, and live spec-layer doctrine. It does not amend a constitutional invariant, redefine or weaken a gate, mint a gate code, create a status enum, create an obligation code, create an invariant ID, or create a doctrine-level finding ID. `ORD-LIFE-CERT` remains a phase-certification label composing the canonical gates and review artifacts defined by the architecture and execution indexes.[^repo-docs-readme][^repo-arch-index][^repo-exec-index][^repo-spec-rules]

This document is **non-executable**. It specifies what an implementing session must inspect, change, run, prove, reconcile, review, and package. It renders no pass/fail verdict, does not assert that any survivor is already killed or equivalent, does not assert that any mutation lane has completed, and fabricates no test, replay, mutation, or CI result.

> I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching repository files only by exact commit URL from `joeloverbeck/tracewake`. References to other repositories inside those validly fetched files are treated as file content, not as provenance contamination.

---

## 1. Status, determination, source discipline, freshness, and admissibility

### 1.1 Single posture and certification-line state

This spec declares exactly one future-spec posture: `Remediation`. It fixes a named failed gate and adds no gameplay scope. Its standing admissibility state is `ORD-LIFE-CERT scoped remediation`.[^repo-ladder]

The certification line remains `ORD-LIFE-CERT scoped remediation` until an implementing session satisfies this spec and publishes a complete replacement acceptance artifact. This spec does not flip the line. Only that replacement artifact may render `ORD-LIFE-CERT passed`, and only from live certifying evidence at one exact final implementation commit.

The settled predecessor states are:

| Certification line | Controlling replacement artifact | Settled state consumed here |
|---|---|---|
| `P0-CERT` | `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` | `P0-CERT passed` |
| `SPINE-CERT` | `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` | `SPINE-CERT passed` |
| `EPI-CERT` | `archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md` | `EPI-CERT passed` |
| `ORD-LIFE-CERT` | `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` | `ORD-LIFE-CERT scoped remediation`; supersession target |

The 0042 audit contract is already authored and accepted as an audit artifact. This spec does not re-commission, redesign, or re-derive it. It treats `ORD-LIFE-01` through `ORD-LIFE-12` as the live seam list that must be re-proven after remediation.[^repo-0042-spec]

### 1.2 Why this is the next admissible move

The phase ladder is ordered:

```text
P0-DOC -> P0-CERT -> SPINE-CERT -> EPI-CERT -> ORD-LIFE-CERT
       -> FIRST-PROOF-CERT -> PHASE-4-ENTRY -> SECOND-PROOF-ENTRY
```

Execution doctrine requires a failed gate to produce remediation naming the responsible layer; it may not be relabeled a phase exception. The ledger's next-move entry expressly requires a separately numbered ORD-LIFE-CERT mutation-remediation/replacement spec before `ORD-LIFE-CERT passed`, `FIRST-PROOF-CERT`, Phase-4 entry, or later work may be cited.[^repo-ladder][^repo-ledger]

Accordingly, this spec:

- performs no feature survey;
- proposes no gameplay expansion;
- does not begin `FIRST-PROOF-CERT`;
- does not authorize Phase-4 institutions, records, or wrong-suspicion work;
- does not authorize second-proof notices, travel, regional scale, or LOD work; and
- does not reopen predecessor certification lines.

### 1.3 Exact-commit source discipline

The manifest supplied for this authoring session is path inventory only. It is not repository content, not branch metadata, and not proof that the target commit is latest `main`.

All claims in this spec about repository state at the authoring baseline derive only from manifest-listed files fetched from exact URLs containing:

```text
owner:  joeloverbeck
repo:   tracewake
commit: 2befc078d96c3b221cad3aa7a0d9b04493a2f0cd
path:   the exact manifest path
```

No repository clone, default-branch lookup, branch-name file fetch, repository code search, snippet reconstruction, repository-scoped connector metadata, or connector namespace label was used as target-repository evidence. The acquisition ledger is summarized in Appendix A and retained as a companion text artifact.

Commit strings embedded inside archived specs, tickets, reports, and acceptance artifacts—including `98dc042`, `726b2a1`, `92ba47f`, `7a17447d`, and others—are those artifacts' own historical provenance. They are ordinary fetched content and are not this authoring baseline. A validly fetched file may mention `joeloverbeck/one-more-branch` or any other repository without contaminating fetch provenance.

Archived specs, archived tickets, archived reports, and Phase-3A hardening artifacts are history or structural precedent only. They do not substitute for live certification evidence at the final implementation commit.[^repo-ledger]

### 1.4 Authoring baseline versus final implementation commit

The implementation package must report two SHAs separately:

| Role | Required value and use |
|---|---|
| Authoring/reassessment baseline | `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd`; the source state against which this spec was authored and the 0042 floor was reassessed. |
| Final implementation/evidence commit | One full exact SHA created by the implementing session after every production, test, fixture, CI, documentation, and evidence-path change required by this spec. All certifying commands run against this one commit. |

The implementing session must enumerate every delta between those SHAs, including moved or deleted survivor-bearing symbols, tests, fixtures, `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, `.github/workflows/ci.yml`, and report paths. It must record the final worktree as clean before certifying commands begin. A dirty worktree, mixed commits across commands, or an unrecorded evidence-only edit invalidates aggregate certification.

Neither SHA may be described as independently verified current `main` unless a separate independently generated current-main SHA and matching manifest are supplied outside this workflow.

### 1.5 Admissibility lock

Until a passing replacement artifact lands:

- no later spec may cite `ORD-LIFE-CERT passed`;
- `FIRST-PROOF-CERT` remains locked;
- `PHASE-4-ENTRY` remains locked;
- `SECOND-PROOF-ENTRY` remains locked; and
- no expansion spec may use this remediation plan, a mutation score, a focused run, a partial shard set, or the old 0042 positive evidence as a substitute for the replacement artifact.

On accepted closeout, stage this file under the stated `specs/` path, archive it by plain rename to the stated `archive/specs/` path, archive the replacement report under `archive/reports/`, and update the live ledger without renumbering the staging sequence or promoting 0043 into `docs/4-specs/`.

---

## 2. Authority and dependency declarations

### 2.1 Authority order

The implementing session must resolve conflicts in this order:

1. `docs/0-foundation/`;
2. `docs/1-architecture/`;
3. `docs/2-execution/`;
4. `docs/3-reference/`;
5. `docs/4-specs/` and the live ledger;
6. this remediation spec;
7. implementation convenience.

If execution text conflicts with architecture or foundation, execution is wrong. If an implementation is more convenient than the accepted gate, the implementation is wrong. If a test kills a mutant by asserting the private helper's literal result while failing to witness a certified ordinary-life consequence, the test is wrong. If an artifact promotes historical, sampled, pending, observer-only, or partial evidence into a live pass, the artifact is wrong.[^repo-docs-readme][^repo-invariants][^repo-exec-testing]

### 2.2 Primary remediation sources and structural lineage

The load-bearing remediation inputs are:

- `archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md` — the audit seam list and live re-proof contract;
- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` — the scoped-remediation artifact to supersede;
- `reports/0042_ord_life_cert_mutation_triage_register.md` — the historical survivor seed and two incomplete-lane records;
- `archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` — freshest structural model;
- `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` and `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` — prior remediation anatomy;
- the 0037, 0039, and 0041 passing replacement artifacts — closeout and supersession shape; and
- the 0040 EPI mutation register/final-missed artifacts — register identity and failed-floor shape.

The prior remediation documents are shape models, not delta seeds. The 0042 ORD-LIFE audit is the delta seed. No prior artifact can certify the final implementation commit by inheritance.

### 2.3 Foundation dependencies

The complete `docs/0-foundation/` tier was read for authority and boundary awareness. The primary dependencies for this remediation are:

| Foundation source | Binding use in 0043 |
|---|---|
| `docs/README.md` | Tier authority and conflict rule. |
| `02_CONSTITUTIONAL_INVARIANTS.md` | All properties re-proved; amended by none. |
| `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` | Ordinary-life accounting, intentions, routines, and stuck state remain event-sourced/replay-reconstructable. |
| `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Needs, candidate goals, intentions, routines, planning, and failure semantics. |
| `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` | Actions, affordances, movement, durations, and survival boundaries. |
| `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` | No-human ordinary-day acceptance and downstream first-proof relationship. |
| `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` | Actor-known transaction, sealed inputs, and truth-firewall constraints on witnesses. |

The remaining foundation documents bound institutions, possession/TUI, no-scripting, scale, speech/LLM, claims/beliefs/memory, and research history. They are not remediation targets.

### 2.4 Architecture dependencies

The complete `docs/1-architecture/` tier was read. The primary dependencies are:

| Architecture source | Binding use in 0043 |
|---|---|
| `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Canonical gate/review-artifact composition; `ORD-LIFE-CERT` mints no gate. |
| `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` | Preserve `core <- content <- tui`; no test-only production inversion. |
| `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | Projections and metrics rebuild from events; no truth writing. |
| `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | Sealed holder-known context and actor-known provenance inputs. |
| `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Needs-to-proposal decision transaction and local planning boundary. |
| `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | Spatial, affordance, and no-teleport consequences. |
| `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Typed diagnostics and review-artifact composition. |
| `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | No-direct-dispatch boundary only; SPINE itself is not re-audited. |

Other architecture documents are boundary awareness for epistemics, speech, institutions, possession/TUI, incidents, LOD, and forbidden misreads.

### 2.5 Execution dependencies and gate vocabulary

The complete `docs/2-execution/` tier was read. The primary dependencies are:

| Execution source | Binding use in 0043 |
|---|---|
| `00_EXECUTION_INDEX_AND_AUTHORITY.md` | Canonical gate index and evidence composition. |
| `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` | Admissibility vocabulary: `passed`, `scoped remediation`, `not applicable`. |
| `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` | Downstream first-proof boundary; not executed here. |
| `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` | Gate order, remediation posture, failure-layer list, temporal routing, EMERGE-OBS observer-only status. |
| `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` | No hidden truth, prose-born facts, or validation-truth proposal shortcuts. |
| `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | Scheduler cannot dispatch ordinary actions directly. |
| `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | Ten pass conditions, seven fixtures, single-charge, single owner, progress, stuck, recovery, and ordinary-life semantics. |
| `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` | Positive/adversarial fixture execution and replay evidence. |
| `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Evidence honesty, diagnostics, mutation completion, and review artifact rules. |
| `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | Consume already-certified possession/debug guarantees where needed; do not re-audit EPI. |

`EMERGE-OBS` is an observer-only evidence-package member, never a gate, score, pass/fail threshold, scheduler objective, or scenario goal.

### 2.6 Reference and live-spec dependencies

The controlling reference/live-spec sources are:

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — review questions;
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — anti-Goodhart, contamination, silent starvation, and no-progress risks;
- `docs/3-reference/02_GLOSSARY.md` — canonical terms including `actor-known`, `holder-known context`, `routine`, `intention`, `behavioral progress`, `stuck diagnostic`, and `EMERGE-OBS`;
- `docs/4-specs/SPEC_LEDGER.md` — current certification-line state and next move;
- `docs/4-specs/README.md` — future-spec posture and gate-code rules;
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — replacement artifact fields; and
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — fixture realism boundary only.

### 2.7 Crate direction and implementation boundary

The remediation must preserve:

```text
tracewake-core <- tracewake-content <- tracewake-tui
```

Permitted changes include production corrections in `tracewake-core`, fixture/schema changes in `tracewake-content` that consume core APIs, TUI evidence changes that consume lower layers, tests in their owning crates, CI workflow changes, and report/spec artifacts.

Forbidden changes include:

- core depending on content or TUI;
- production code depending on test modules or fixture-only helpers;
- a test-only bypass around the action pipeline, event log, replay, or projection;
- compatibility aliases or shadow paths introduced solely to preserve an obsolete test shape;
- direct truth/debug/prose input to actor decisions;
- mutation-exclusion tricks that move required code outside the perimeter; and
- a second writer for need/duration accounting.

### 2.8 Primary code, test, and configuration seams

The verified code/test/configuration surface is the one named by 0042 section 5.1 and reassessed at the authoring baseline. It includes:

- `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, `.github/workflows/ci.yml`;
- needs/accounting: `agent/need.rs`, `need_accounting.rs`, `actions/defs/need_events.rs`;
- candidates: `agent/generation.rs`, `agent/candidate.rs`;
- intentions/routines/methods: `agent/intention.rs`, `routine.rs`, `methods.rs`, `htn.rs`;
- planning/decision/trace: `agent/planner.rs`, `decision.rs`, `trace.rs`;
- actor-known transaction: `agent/transaction.rs`, `actor_known.rs`, `no_human_surface.rs`, `perception.rs`;
- ordinary actions: `eat.rs`, `sleep.rs`, `work.rs`, `wait.rs`, `continue_routine.rs`, `movement.rs`;
- proposal/pipeline: `actions/proposal.rs`, `pipeline.rs`, `report.rs`, `registry.rs`;
- state/affordance/time: `state.rs`, `location.rs`, `time.rs`;
- orchestration/projections: `scheduler.rs`, `projections.rs`;
- events/replay/checksum: `events/{envelope,log,apply,mutation}.rs`, `replay/{rebuild,report}.rs`, `checksum.rs`;
- core gate harnesses under `crates/tracewake-core/tests/`, including `no_human_capstone.rs`, `acceptance_gates.rs`, `event_schema_replay_gates.rs`, `golden_scenarios.rs`, `hidden_truth_gates.rs`, `generative_lock.rs`, and `support/generative.rs`; and
- content/TUI harnesses, especially `crates/tracewake-content/tests/golden_fixtures_run.rs` and the named TUI gate binaries.

Appendix B gives the exact 60-file configured mutation perimeter at the authoring baseline. A final implementation may change the file or mutant count only through explained final-tree source changes; it may not silently shrink the semantic perimeter.

---

## 3. Problem statement

### 3.1 The 0042 verdict and standing floor

The 0042 acceptance artifact renders `ORD-LIFE-CERT scoped remediation`, not `ORD-LIFE-CERT passed`. Two independent blockers exist and are co-equal in this spec:

1. **Configured-lane non-completion.** `cargo mutants --workspace --no-shuffle` did not produce a complete summary. Its PTY wrapper remained active after no cargo-mutants process was visible and the run was interrupted. A deterministic `-j 8` rerun likewise failed to produce a complete summary and was interrupted. A process-list observation is not completion proof, and an interrupted wrapper is not a cargo-mutants outcome.
2. **Actionable survivor seed.** Before interruption, the partial run exposed three missed mutants in `crates/tracewake-core/src/need_accounting.rs`, each mapped to `ORD-LIFE-01`, `ORD-LIFE-08`, and `ORD-LIFE-12`.

The historical seed rows are:

| Historical cargo-mutants identity | Enclosing symbol and semantic gap | ORD-LIFE cross-reference | Responsible layer |
|---|---|---|---|
| `need_accounting.rs:88:25 replace < with <= in DurationInterval::contains_tick` | Body-exclusive duration start boundary becomes inclusive. | `ORD-LIFE-01`, `ORD-LIFE-08`, `ORD-LIFE-12` | need accounting; duration interval boundary; projection/replay derivation |
| `need_accounting.rs:106:13 replace && with || in duration_intervals` | Actor ownership and “not already logged” guards cease to be jointly required. | `ORD-LIFE-01`, `ORD-LIFE-08`, `ORD-LIFE-12` | need accounting; duration interval selection; projection/replay derivation |
| `need_accounting.rs:109:45 replace == with != in duration_intervals` | Event-identity membership test reverses, allowing duplicate ownership or excluding a legitimate current start. | `ORD-LIFE-01`, `ORD-LIFE-08`, `ORD-LIFE-12` | need accounting; actor/event interval ownership; projection/replay derivation |

The two infrastructure rows remain historical blockers to reconcile:

| Historical lane | Historical result | Required 0043 disposition |
|---|---|---|
| `cargo mutants --workspace --no-shuffle` | tool-failure/incomplete; interrupted; partial three-miss floor | root cause diagnosed; direct or supervised final lane completes; partial data retained as historical only |
| `cargo mutants --workspace --no-shuffle -j 8` | tool-failure/incomplete; interrupted | concurrency choice reassessed; final lane completion proven rather than inferred |

No historical missed mutant was accepted as equivalent or non-critical. The baseline-miss file was empty. The 0042 register recorded a 60-file, 2877-mutant configured census.[^repo-0042-triage]

### 3.2 Central divergence from 0041

The 0041 EPI remediation started from a **completed configured run** with a known survivor floor. Its dominant task was to kill/reconcile that floor and re-prove EPI.

0043 starts from an **incomplete configured run**. Therefore its center of gravity is co-equal:

```text
kill + COMPLETE + reconcile + triage + re-prove
```

A test change that kills the three historical identities but leaves the configured lane incomplete fails this spec. A completed lane that still leaves actionable survivors fails this spec. A clean mutation score from a filtered or in-diff run fails this spec. Completion and semantic disposition are separate mandatory claims.

### 3.3 Static perimeter confirmation at the authoring baseline

At `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd`:

- `.cargo/mutants.toml` loads the standing certification lock-layer posture with `additional_cargo_args = ["--locked"]` and `test_workspace = true`;
- the configuration contains `crates/tracewake-core/src/need_accounting.rs` and the six ORD-LIFE additions made by 0042;
- mechanical expansion of the manifest-listed globs yields exactly 60 unique source files;
- the 0042 register records 2877 generated mutants;
- `.cargo/mutants-baseline-misses.txt` is zero bytes; and
- the three historical operators still map to `DurationInterval::contains_tick` and `duration_intervals` by symbol plus operator.

This is a static baseline confirmation, not a claim that the final census will remain 60/2877 and not a claim that any mutant is killed.

### 3.4 CI durability gap

The scheduled `mutants-lock-layer` job correctly installs cargo-mutants 27.1.0, invokes `cargo mutants --workspace --no-shuffle`, distinguishes exit 0/2 from other tool/run failures, and uploads `mutants.out` under `if: always()`.

The `mutants-in-diff` guarded-path trigger at the authoring baseline omits all six paths added to the standing perimeter by 0042:

- `crates/tracewake-core/src/need_accounting.rs`;
- `crates/tracewake-core/src/actions/registry.rs`;
- `crates/tracewake-core/src/actions/defs/need_events.rs`;
- `crates/tracewake-core/src/actions/defs/wait.rs`;
- `crates/tracewake-core/src/actions/defs/continue_routine.rs`; and
- `crates/tracewake-core/src/actions/defs/movement.rs`.

This is the ORD-LIFE analogue of the 0041 in-diff trigger gap. The implementation must make the minimal correction so edits to any configured ORD-LIFE path trigger the fast detector. It should also retain in-diff mutation artifacts on failure. The in-diff job remains non-certifying and cannot replace the complete configured run; cargo-mutants itself warns that `--in-diff` can miss coverage regressions outside changed production lines.[^cargo-in-diff]

### 3.5 Responsible failing layers

The historical floor maps primarily to:

- need accounting;
- action-pipeline/ordinary-life accounting ownership;
- event application;
- projection/replay;
- tests/fixtures; and
- mutation infrastructure/configured-perimeter completion.

The last item is a remediation routing layer, not a new canonical gate code. Final failures must still use execution document `03`'s existing responsible-layer vocabulary where applicable, with mutation-infrastructure completion recorded as the transport/tooling layer that prevented canonical evidence from being produced.

### 3.6 The three identities are a seed, not the finish line

The historical rows establish mandatory reconciliation work. They are not the acceptance denominator. The final denominator is the complete configured population at the final implementation commit.

Every newly exposed `missed`, `timeout`, relocated historical identity, unexpected `unviable` cluster, absent shard, wrapper termination, or structured-output mismatch joins the same register. Killing only the named three, improving a percentage, or stopping after a green focused run is anti-Goodhart behavior and cannot certify the line.[^repo-risk]

---

## 4. Remediation approach

### 4.1 Required end state

Execution is complete only when all of the following are true:

1. The package records the authoring baseline `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd` and one exact final implementation/evidence commit.
2. Every implementation delta between those commits is enumerated and the final worktree is clean.
3. `.cargo/mutants.toml` retains the standing configured perimeter without shrinkage, silent exclusion, `#[mutants::skip]` laundering, or replacement config.
4. Final `--list-files` and `--list` outputs are captured, fingerprinted, and reviewed; every count delta from 60 files/2877 mutants is explained by final-tree source change.
5. The scheduled CI lane continues to run the checked-in full configured posture.
6. The in-diff trigger includes all six 0042-added paths and remains explicitly non-certifying; the `STANDING_MUTATION_TRIGGER_FRAGMENTS` expectation in `crates/tracewake-core/tests/ci_workflow_guards.rs` is co-edited to match the corrected trigger, and the guard asserts perimeter↔trigger consistency so the two cannot drift independently again.
7. Mutation outputs are retained even when the launcher, wrapper, baseline, shard, or cargo-mutants process fails.
8. The PTY/wrapper non-completion is investigated and a reproducible root-cause or bounded external-environment diagnosis is recorded.
9. The final certifying lane terminates deterministically and yields transport evidence distinguishing normal tool exit, cargo-mutants timeout outcome, baseline failure, launcher/tool failure, external wall-clock termination, and cancellation.
10. The unmutated baseline and every named ordinary-life gate suite pass at the exact final commit before mutation outcomes are interpreted.
11. The standing configured mutation posture completes unsharded or as a provably complete shard union of the same denominator.
12. Every generated identity in the final denominator has exactly one final tool outcome or an explicit tool-level blockage that prevents certification.
13. All three historical survivor identities are reconciled by path, symbol, and operator through any refactor.
14. Every additional final-run survivor or timeout is appended to the register and triaged.
15. The default resolution is kill-with-behavior/provenance coverage through a public or certified ordinary-life consequence.
16. Each known `need_accounting.rs` seed is caught, or—only if the source transformation makes it genuinely indistinguishable or unreachable—has rigorous equivalent/unreachable proof and independent signoff. The three seed rows may not be dismissed as non-critical convenience cases.
17. No historical or new survivor is parked in `.cargo/mutants-baseline-misses.txt` to make CI green.
18. Zero identities remain blocked or untriaged; zero timeouts, absent shards, incomplete denominators, launcher failures, or unresolved tool failures remain.
19. `ORD-LIFE-01` through `ORD-LIFE-12` are re-proven live at the exact final commit.
20. The ten live ORD-LIFE pass conditions and seven mandatory fixture families have complete certifying evidence.
21. Replay/provenance ancestry, negative/contamination controls, and responsible-layer diagnostics accompany every behavior-relevant mutation witness.
22. The replacement artifact fully instantiates `docs/4-specs/0003`, renders `ORD-LIFE-CERT passed` only if every aggregate condition passes, and explicitly supersedes the 0042 acceptance artifact.
23. `EMERGE-OBS` appears only as observer-only evidence and contributes to no pass/fail calculation.
24. The live ledger is updated only after accepted closeout; no later gate is begun inside this package.

If any item remains incomplete, the replacement artifact must remain `ORD-LIFE-CERT scoped remediation`, name the first responsible layer, retain the evidence obtained, and route a separately numbered follow-up. It may not render a partial aggregate pass.

### 4.2 Freeze the final implementation identity before certification

Development may occur across multiple commits. Certification may not.

Before the final command ledger begins, the implementing session must:

1. create the intended final implementation commit;
2. record `git rev-parse HEAD` and a clean `git status --porcelain` result;
3. record Rust, Cargo, cargo-mutants, OS/runner, CPU/memory, and relevant environment versions;
4. fingerprint `Cargo.lock`, `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, `.github/workflows/ci.yml`, final list outputs, and every evidence register;
5. prohibit source, test, fixture, configuration, or evidence-content edits until the certifying command set completes; and
6. if any edit is required, create a new final commit and rerun the complete certifying set.

Evidence generated at mixed commits is historical/development evidence and cannot be aggregated into the passing replacement row.

### 4.3 Maintain and re-confirm the standing configured perimeter

The checked-in configuration is the denominator of record. The certifying population must load `.cargo/mutants.toml` and must not use any of the following as the final denominator:

- `--no-config`;
- a custom replacement config;
- `-f` or `--file`;
- `--exclude`;
- `--in-diff`;
- `--iterate`;
- function/mutant filters; or
- an undocumented environment override that changes file selection, test selection, output, timeout, or mutation generation.

Capture at the final commit:

```bash
mkdir -p reports/0043_ord_life_cert_command_transcripts

cargo mutants --workspace --no-shuffle --list-files \
  > reports/0043_ord_life_cert_mutation_list_files.txt

cargo mutants --workspace --no-shuffle --list \
  > reports/0043_ord_life_cert_mutation_list.txt
```

JSON forms should also be retained when supported by the pinned tool version, because structured identity reconciliation is less error-prone than terminal text. Text remains required for human review.

The perimeter review must prove:

- `need_accounting.rs` is present;
- all six 0042-added files are present;
- every file in Appendix B that still exists and still belongs to the standing seam remains selected;
- `test_workspace = true` remains effective;
- locked dependency arguments remain effective without duplicate Cargo flags;
- no config or CLI exclusion hides a required path;
- CI loads the same checked-in config; and
- file/mutant count changes are explained, not treated as automatic pass or failure.

Cargo-mutants documents that filename filters define the generated population and that `--list-files`/`--list` preview the effective denominator.[^cargo-filtering][^cargo-workspaces]

### 4.4 Mutation-lane completion and tool-failure resolution

Lane completion is a first-class remediation objective, not an operational footnote.

The implementing session must create:

```text
reports/0043_ord_life_cert_mutation_lane_completion_diagnostic.md
```

That diagnostic must record:

- exact historical symptom and retained 0042 transcript references;
- whether the PTY wrapper was repository-owned, host-owned, CI-owned, or session/tool-owned;
- the command tree, child PID/process group, parent/wrapper PID, start/end timestamps, signals, exit statuses, and output paths for each reproduction attempt;
- whether cargo-mutants exited, was killed, remained blocked, or lost its controlling terminal;
- whether stdout/stderr closure, child reaping, shell pipeline status, or artifact copying was the actual failure point;
- whether `mutants.out/lock.json`, `mutants.json`, `outcomes.json`, outcome text files, and logs existed and advanced;
- a minimal reproducer where the problem is repository- or wrapper-owned;
- the corrective change or the exact external-environment bypass used for certification; and
- a regression control proving that a normally exiting child, a nonzero child, a deliberately stalled child, and a killed child all terminate the supervisor and produce distinct statuses.

A generic conclusion such as `unknown PTY issue` is insufficient. If the historical wrapper is outside the repository and cannot be inspected, the diagnostic must still bound the cause by proving that the same commit, command, config, and environment complete under a direct or replacement-supervised launch; identify the inaccessible wrapper/transport as the non-certifying failing layer; retain its historical evidence; and permanently bypass it for the certifying lane.

The certifying run should be launched without a PTY. A PTY may be used only for diagnosis or when a platform absolutely requires it. If `script` is used, `script -e/--return` must propagate the child status and the raw typescript must be retained.[^util-linux-script]

A repository-owned Rust launcher must explicitly wait for or kill and then wait for every child; dropping `std::process::Child` does not terminate or reap it.[^rust-child]

A wall-clock supervisor must:

- supervise the process group, not merely the immediate shell;
- send a graceful termination signal, then a recorded forced-kill signal after a bounded grace period;
- preserve a wrapper-specific timeout/kill status distinct from cargo-mutants exit code 3;
- copy partial `mutants.out` and raw logs under `always`/finally behavior;
- record why termination occurred; and
- never classify “no cargo-mutants process visible” as completion.

GNU `timeout` is one acceptable primitive because it has distinct timeout/failure statuses, but the implementation must record its exact invocation and must not use `--preserve-status` in a way that erases the wrapper-timeout distinction.[^gnu-timeout]

### 4.5 Outcome vocabulary: keep transport, tool, and certification axes separate

The package must distinguish at least these axes:

| Axis | Allowed/required values | Meaning |
|---|---|---|
| Launcher/supervisor result | normal child exit; wrapper wall timeout; forced kill; cancellation; spawn/IO failure; PTY/script failure | Did the process supervision lane itself terminate honestly? |
| Denominator completion | complete; incomplete | Does the final outcome union cover the complete final `--list` census exactly once? |
| Cargo-mutants run exit | 0, 1, 2, 3, 4, documented diff/internal codes, signal-derived status | What did cargo-mutants report? Exit 2 means misses; exit 3 means cargo-mutants observed test timeouts; neither is a launcher timeout.[^cargo-exit-codes] |
| Per-mutant tool outcome | `caught`; `missed`; `timeout`; `unviable` | Tool result for one generated identity. |
| Certification disposition | killed; approved equivalent; approved non-critical; blocked/untriaged | Human-reviewed semantic disposition. The three historical seeds default to killed and may close only as caught or rigorously equivalent/unreachable. |
| Evidence status | pass; fail; pending; sampled; observer-only; historical | Template honesty label; pending/sampled/observer-only/historical never aggregate as pass. |

A wrapper wall-clock timeout is not a cargo-mutants per-mutant `timeout`. An interrupted denominator is not a large set of `caught` results plus an ignorable remainder. An exit code alone is not completion proof.

### 4.6 CI convergence and baseline-miss discipline

The implementation must minimally correct `.github/workflows/ci.yml` so the guarded-diff trigger covers the complete standing configured perimeter. At minimum, the expression must include the six omitted files listed in section 3.4. A directory-level expression is preferred where it exactly represents the intended seam and avoids future drift.

The same correction must be applied in lockstep to the guarded-trigger expectation encoded in `crates/tracewake-core/tests/ci_workflow_guards.rs`. That test holds a hand-maintained `STANDING_MUTATION_TRIGGER_FRAGMENTS` constant and asserts, by containment, that the in-diff trigger contains each listed fragment. At the authoring baseline that constant mirrors the *incomplete* trigger and therefore lacks the same six 0042-added paths. Two failure modes follow if the constant is not co-edited: an explicit six-fragment addition to `ci.yml` would pass the test while leaving the corrected trigger unguarded (the §3.4 durability gap recurs silently); a preferred directory-level rewrite would instead make the literal-fragment containment check fail until the constant is updated. The implementation must extend `STANDING_MUTATION_TRIGGER_FRAGMENTS` so it represents the corrected trigger exactly, and `crates/tracewake-core/tests/ci_workflow_guards.rs` is a required touched surface enumerated in the final delta.

The root cause of the §3.4 durability gap is structural: `ci_workflow_guards.rs` carries two independent hand-maintained duplicates of one seam — `STANDING_MUTATION_PERIMETER` (checked against `.cargo/mutants.toml`) and `STANDING_MUTATION_TRIGGER_FRAGMENTS` (checked against `.github/workflows/ci.yml`) — so 0042 widened the configured perimeter without the in-diff trigger or its guard following. To prevent a future recurrence, the guard must additionally assert perimeter↔trigger consistency: derive the expected trigger coverage from the standing perimeter, or cross-check the two lists, so the configured perimeter and the in-diff trigger cannot drift independently again. This consistency assertion is part of the deliverable; a one-time fragment correction without it leaves the same drift surface open for the next perimeter change.

The CI behavior must preserve these distinctions:

- scheduled/manual full configured run: certification-supporting only when dated, complete, and reconciled;
- in-diff run: fast non-certifying detector;
- cargo-mutants exit 2: misses exist, not a tool crash;
- any other unexpected exit: tool/run failure;
- no-mutants-in-diff case: legitimate fast-detector no-op only after guarded-path logic and diff evidence are retained; and
- artifact upload: run even when mutation or baseline checking fails.

The in-diff job should upload `mutants.out`, the exact diff, command transcript, and status metadata under `if: always()` when it runs. GitHub Actions supports matrix partitioning and artifact retention; the package must record artifact names and digests rather than relying on transient console display.[^github-matrix][^github-artifacts]

`.cargo/mutants-baseline-misses.txt` must remain empty for this remediation unless an owner explicitly approves a separately justified exception under the same proof and signoff rules as section 4.14. The three historical survivors may not be placed there. A normalized baseline line is not a semantic disposition.

### 4.7 Clean baseline and named preflight

Before interpreting any mutant outcome, run at the exact final implementation commit:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --locked -p tracewake-core --doc
```

Then run the named 0042 gate binaries at minimum:

```bash
cargo test --locked -p tracewake-core \
  --test acceptance_artifact_wording \
  --test acceptance_gates \
  --test anti_regression_guards \
  --test ci_workflow_guards \
  --test doc_invariant_references \
  --test emergence_ledger \
  --test event_schema_replay_gates \
  --test generative_lock \
  --test golden_scenarios \
  --test hidden_truth_gates \
  --test negative_fixture_runner \
  --test no_human_capstone \
  --test spine_conformance

cargo test --locked -p tracewake-content \
  --test fixtures_load \
  --test forbidden_content \
  --test golden_fixtures_run \
  --test schema_conformance

cargo test --locked -p tracewake-tui \
  --test adversarial_gates \
  --test command_loop_session \
  --test embodied_flow \
  --test readme_sample_session \
  --test transcript_snapshot \
  --test tui_acceptance \
  --test tui_seam_conformance
```

Equivalent one-binary-per-command execution is allowed and often preferable for first-divergence attribution. The artifact must record exact commands, exit statuses, start/end times, tool versions, environment facts, and transcript fingerprints.

A mutation run against a failing or hanging baseline is meaningless. Cargo-mutants performs a baseline for the same reason; skipping it without a separately recorded passing baseline makes results misleading.[^cargo-baseline]

### 4.8 Development loops versus certifying run

During remediation, focused runs are allowed:

- exact file/function/mutant filters;
- manually applied mutant diffs;
- a single test or fixture;
- `--iterate`;
- `--in-diff`;
- low-concurrency timeout diagnosis;
- isolated sharding experiments; and
- PTY/supervisor reproductions.

These are development evidence only. They may prove that a candidate witness kills one identity or that a wrapper bug is fixed, but they cannot establish the final denominator or aggregate pass.

The default unsharded certifying form is:

```bash
cargo mutants --workspace --no-shuffle \
  -o reports/0043_ord_life_cert_mutation_full.out
```

Explicit `--jobs`, build/test timeout, or output options are allowed only when recorded and justified. High job counts can increase contention and spurious timeouts; choose concurrency from measured runner capacity rather than copying the failed `-j 8` experiment by habit.[^cargo-parallelism]

The final certifying form must not use `--no-config`, file/function filters, `--in-diff`, or `--iterate`.

### 4.9 Sharding, timeouts, and completion proof

Sharding is permitted only as a reproducible partition of one complete configured denominator. If used:

- compute and fingerprint one unsharded final `--list` census;
- use one denominator `n` and every shard index `0..n-1` exactly once;
- use the same final commit, config, cargo-mutants version, Rust/Cargo versions, arguments, shuffle posture, timeout policy, environment, and source tree on every shard;
- write each shard to a unique stable output path;
- retain each shard's `mutants.json`, `outcomes.json`, `caught.txt`, `missed.txt`, `timeout.txt`, `unviable.txt`, diff files, logs, command transcript, and supervisor metadata;
- reconcile the union of shard identities to the unsharded census with no omissions or duplicates;
- reconcile outcome counts to the identity union;
- treat any absent, cancelled, failed, or structurally inconsistent shard as incomplete; and
- use `--baseline=skip` on shards only after one passing baseline at the same commit/environment and with an explicit timeout policy.

Cargo-mutants states that shards independently discover and partition mutants and that inconsistent arguments or denominators make the result meaningless.[^cargo-shards]

A cargo-mutants per-mutant timeout is not a pass. Each timeout must be retried under a recorded policy and resolved to:

- caught;
- missed, then killed;
- approved equivalent;
- approved non-critical, except the three historical seed identities;
- unviable under the final toolchain; or
- blocked/tool failure, which prevents certification.

The complete-run proof must be machine-checkable. At minimum it must show:

```text
set(final --list identities)
  == disjoint_union(caught, missed, timeout, unviable)
```

subject to any tool-version categories explicitly documented and reconciled. It must also prove that every structured output belongs to the same final commit/config/version and that the launcher terminated normally. Cargo-mutants writes `mutants.json` before testing and updates `outcomes.json` and outcome files during the run; those files are evidence inputs, not a substitute for the final set reconciliation.[^cargo-output]

### 4.10 Historical identity reconciliation

The 0042 register is a seed work-list. Each historical row must retain:

- raw 0042 identity (`path:line:column:operator`);
- normalized path;
- enclosing symbol/function;
- exact operator/diff;
- final generated identity, if still present;
- source-change mapping when line/column or implementation shape moved;
- ORD-LIFE cross-references;
- responsible layer;
- final tool outcome;
- final certification disposition; and
- evidence proving closure.

A historical survivor does not disappear because a refactor moves a line, renames a helper, inlines a predicate, changes cargo-mutants wording, or removes the exact syntactic operator. The register must map the old semantic gap to the final production/test change and behavior witness.

Every additional survivor from the completed final run is appended as `new in completed run`. Survivors remain cargo-mutants identities; no doctrine-level finding IDs are created.

### 4.11 Triage register schema

Produce:

```text
reports/0043_ord_life_cert_mutation_triage_register.md
reports/0043_ord_life_cert_mutation_final_missed.txt
```

The register must contain at least:

| Field | Required content |
|---|---|
| Historical identity | Raw 0042 row when applicable; otherwise `new in completed run`. |
| Final identity | Final path, symbol/function, line/column where emitted, operator/diff, and structured-output reference. |
| Tool outcome | `caught`, `missed`, `timeout`, `unviable`, or explicit launcher/tool failure. |
| ORD-LIFE cross-reference | One or more of `ORD-LIFE-01` through `ORD-LIFE-12`, or mutation/artifact completion where appropriate. |
| Responsible layer | Existing execution-03 layer plus the transport note when mutation infrastructure prevented evidence. |
| Certified reachability | Exact production call sites and certified consequences. |
| Test family | Named behavior/property/metamorphic family and exact retained case. |
| Behavior witness | Public or certified ordinary-life consequence, not the helper literal. |
| Replay/provenance ancestry | Event IDs/log range, projection/replay/checksum package, actor-known sources, manifest/schema identity, or justified `not applicable`. |
| Negative/contamination control | Hidden truth, debug, prose, direct state, wrong actor, duplicate event, stale source, or bypass control. |
| Certification disposition | killed; approved equivalent; approved non-critical; blocked/untriaged. |
| Evidence references | Baseline transcript, mutant-active failure, final output row, logs/diffs, artifact IDs, fingerprints. |
| Review signoff | Implementer and independent reviewer; mandatory for exceptions and final register closeout. |

Tool outcome and certification disposition are separate axes. An approved equivalent remains a `missed` tool outcome and must never be counted as `caught`.

The final-missed manifest must use canonical cargo-mutants identities, one per line, and reconcile exactly to the final `missed` tool set. If it is empty, retain and fingerprint the empty file rather than omitting it.

### 4.12 Kill-with-behavior/provenance coverage is the default

A valid killing witness must:

1. pass against the unmutated final implementation;
2. fail when the specific mutant is active;
3. observe a public or certified ordinary-life consequence;
4. travel the production seam rather than a test-only bypass;
5. carry event/replay/provenance ancestry where behavior is event-derived, projected, serialized, or actor-known;
6. include a negative/contamination control where truth, debug, prose, direct state, wrong ownership, or duplicate input could shortcut the claim;
7. identify the first responsible layer; and
8. retain one concrete mutant-active failure artifact for the identity.

For the three `need_accounting.rs` seeds, the sole or primary witness must **not** assert the return value of `DurationInterval::contains_tick`, `duration_intervals`, `actor_tick_regimes_with_start`, or `classify_actor_tick_regimes_with_start` in isolation. It must observe:

- single-charge-per-actor/per-need/per-tick behavior;
- body-exclusive duration start and terminal-inclusive interval ownership;
- correct actor/event ownership for a current duration start;
- emitted `NeedDeltaApplied`/duration/terminal event ancestry;
- live ordinary-life projection or metrics; and
- equality after rebuilding from the retained event log.

Insufficient examples include:

- `assert!(interval.contains_tick(...))` as the only oracle;
- `assert_eq!(duration_intervals(...).len(), ...)` as the only oracle;
- copying the mutated boolean into the test's expected expression;
- reading a private map or constructing a private interval through test-only access;
- asserting only total tick counts while allowing per-actor/per-need double charge;
- deriving the expected target, food, route, workplace, or schedule from ground truth/debug output;
- bypassing the actor-known transaction, sealed proposal, action pipeline, event append/application, or replay path where those are part of the certified consequence; or
- making a golden byte snapshot the sole proof when semantic accounting drift remains possible.

Property-based mutation research supports treating a mutant as meaningfully killed only when it violates the named property, rather than merely changing incidental output.[^property-mutation]

### 4.13 Property-based and metamorphic behavior families

Kindred mutants may share a parameterized, property-based, table-driven, or metamorphic family. Grouping expresses the contract coherently; it is not sampling away member-level proof.

The ORD-LIFE accounting family must cover at least:

| Property/relation | Required semantic oracle |
|---|---|
| Single-charge conservation | For every actor, need, and eligible tick, exactly one owning accounting event covers that tick; totals equal the event-backed per-tick ledger. |
| Regime partition | For a window, each actor tick is exactly one of awake/asleep/working; counts sum to the number of eligible ticks without overlap. |
| Body-exclusive start / terminal-inclusive end | A duration start at `S` does not own tick `S`; body ticks after `S` through terminal `T` are owned exactly once. |
| Window split/recombine | Splitting an accounting window at a legal boundary and recombining event-backed results equals the unsplit run, with no boundary double charge or omission. |
| Current-start/log representation equivalence | Representing the same accepted start as already logged versus supplied once as `current_start` yields the same public accounting/replay result. |
| Unrelated-actor invariance | Adding a different actor's start or terminal cannot alter the subject actor's need ledger, regimes, metrics, or replay checksum. |
| Duplicate-event idempotence/fail-closed | Supplying a current start whose event ID is already in the log cannot duplicate interval ownership or need charge. |
| Replay determinism | Rebuilding from an empty projection produces the same need states, metrics, diagnostics, and semantic fingerprints as live application. |
| Interruption/terminal conservation | Sleep/work completion, failure, or interruption closes exactly one start and charges the elapsed body ticks once. |
| Hidden-truth non-interference | Changing only unobserved food/route/affordance/workplace/schedule truth cannot change actor-known cognition or the accounting oracle. |

Generated evidence must record seed, generator version, case count, shrink result, retained minimal failing case, and omitted population. It is `sampled` unless an exhaustive finite domain is proven. QuickCheck-style property testing and Proptest state-machine preconditions are suitable for generating legal event/transition sequences; metamorphic relations are suitable when a single literal oracle would duplicate implementation logic.[^quickcheck][^proptest-state][^metamorphic]

For every final survivor, the package must still identify one retained concrete case that kills that exact identity.

### 4.14 Equivalent and non-critical exceptions

“No test failed,” “private helper,” “compiler optimized it,” “high mutation score,” or “seems harmless” is not equivalent proof.

An equivalent/unreachable claim must include:

- exact mutant diff;
- complete reachable call-site census at the final commit;
- complete input/domain argument;
- proof that no legal production execution can distinguish original and mutant;
- source/replay/property evidence supporting the proof; and
- independent reviewer signoff.

A non-critical claim must prove that no constitutional invariant, canonical gate, `ORD-LIFE-01`…`12` seam, ordinary-life actor-visible consequence, event/replay/projection obligation, single-charge rule, stuck/metric rule, possession continuity, truth-firewall rule, or downstream certified consumer can observe the difference.

The three historical seed identities map directly to single-charge and duration ownership. They therefore may not close as “non-critical.” They must be caught or, after a source transformation, rigorously proven equivalent/unreachable.

No exception may be used to hide a missing consumer, defer work, silence a timeout, suppress generation, or populate the baseline-miss file.

### 4.15 Production-code integrity and staged abstractions

Production code may be corrected when a mutant reveals a defect, an unexpressed contract, an obsolete unreachable helper that should be removed, or a missing public consumer. It may not be changed merely to stop generation.

Forbidden tactics include:

- `#[mutants::skip]` on required code;
- exclusion-glob shrinkage;
- dead-code or constant-folding tricks introduced solely for cargo-mutants;
- a test-only branch in production;
- direct state/projection insertion to create an oracle;
- a second need/duration accounting owner;
- a debug-to-actor-known bridge;
- hidden truth or validation truth proposing an action;
- compatibility aliases or shadow implementations; and
- changing fixture prose so a test passes without changing causal behavior.

No staged abstraction may cover mutation completion, survivor disposition, `ORD-LIFE-01`…`12`, replay/provenance, actor-known inputs, duration ownership, single charge, progress/stuck honesty, proposal ancestry, or debug quarantine.

---

## 5. Known-survivor remediation deliverables

### 5.1 Shared witness architecture for the three accounting seeds

The preferred implementation is one production-level accounting contract family with member-specific retained cases. Each case must:

1. construct or load a legal ordinary-life fixture with at least two actors and a duration action;
2. enter through the actor-known transaction and ordinary action/pipeline seam where feasible, or justify the narrowest production entry point that still emits canonical events;
3. retain the accepted duration start, terminal/interruption, need-delta, threshold, proposal, and process/event ancestry;
4. independently expand elapsed duration coverage into actor/need/tick ledger rows from event payloads;
5. assert exactly one owner per actor/need/tick and no cross-actor/event contamination;
6. compare live projection/metrics against an empty-projection replay rebuild;
7. run a negative paired case that changes only hidden truth, debug attachment, unrelated actor input, or duplicate/current-start representation; and
8. retain the mutant-active failure proving the relevant certified consequence changes.

The production oracle must not call the private interval helper to compute its expected answer.

### 5.2 Historical seed: `<` changed to `<=` in `DurationInterval::contains_tick`

Historical identity:

```text
need_accounting.rs:88:25 replace < with <= in DurationInterval::contains_tick
```

The retained case must place a duration start **inside** the requested accounting window. A window whose `from_exclusive` equals the start tick will not distinguish `<` from `<=`, because the first enumerated tick is already `start + 1`.

Required distinguishing shape:

```text
accounting window: (A, B]
duration start:    S, where A < S <= B
duration terminal: T, where S < T <= B or a controlled later boundary
```

The public consequence must prove:

- tick `S` is not owned by the duration body;
- the action-start/awake accounting path and duration-body path do not both charge `S`;
- ticks `S+1` through the legal terminal boundary are owned exactly once;
- terminal inclusion is preserved;
- per-actor/per-need ledger totals and final need values match the event-backed expectation; and
- replay reconstructs the same result.

The mutant must fail because it reclassifies or double-owns the start tick and changes a certified accounting/projection consequence. A helper-level count may be supplemental diagnostics only.

Required negative controls:

- same fixture with duration start exactly at `from_exclusive` to prove the test is not accidentally tied to a vacuous boundary;
- unrelated actor duration inserted at the same tick with no effect on the subject actor; and
- debug/hidden-truth change with no effect on accounting.

### 5.3 Historical seed: `&&` changed to `||` in `duration_intervals`

Historical identity:

```text
need_accounting.rs:106:13 replace && with || in duration_intervals
```

The production guard requires both:

```text
current start belongs to the subject actor
AND
that event ID is not already present in the log
```

The retained case must exercise a truth-table row where exactly one operand is true. The strongest actor-ownership case is:

- subject actor `A` is being accounted;
- `current_start` belongs to actor `B`;
- the start event is not in the log;
- actor `A` and actor `B` have distinguishable needs and/or duration regimes.

Original behavior excludes actor `B`'s start from actor `A`. The `||` mutant includes it. The witness must observe actor `A`'s wrong duration ownership or need/replay metrics, not merely an interval-vector length.

A second retained row should cover:

- same actor;
- current start event already in the log;
- no duplicate need/duration ownership.

Required negative controls:

- actor IDs differ but all hidden-world facts are otherwise equal;
- current start omitted entirely;
- event already logged versus supplied as current start yields equal public results; and
- no direct projection/state mutation is used to create the expected value.

### 5.4 Historical seed: `==` changed to `!=` in current-start event membership

Historical identity:

```text
need_accounting.rs:109:45 replace == with != in duration_intervals
```

The retained cases must make event identity—not vector shape—the distinguishing fact. At minimum use both:

1. **Legitimate current start with unrelated log event.** The subject actor's `current_start` is absent from the log, while an unrelated event is present. Original equality search finds no matching ID and includes the start. The `!=` mutant sees the unrelated ID and wrongly excludes the start. The public consequence is missing duration-body accounting and a live/replay mismatch from the expected event ancestry.
2. **Same event already logged.** The log contains the current start event itself in a shape that exposes duplicate insertion under the reversed predicate. Original behavior is idempotent; the mutant duplicates interval ownership or otherwise changes the public accounting result.

The oracle must compare event-ID ancestry, per-tick ownership, final need values/metrics, and replay reconstruction. It must not be `assert!(log.events().any(...))` or `assert_eq!(duration_intervals.len(), ...)` alone.

Required negative controls:

- two distinct starts at the same tick but different event IDs;
- same event ID with no unrelated event;
- unrelated event with a different actor; and
- deterministic replay from the canonical event order.

### 5.5 Additional survivors exposed by the completed run

Every additional final survivor must receive the same treatment:

- map it to `ORD-LIFE-01`…`12` and a responsible layer;
- establish production reachability;
- identify the public/certified consequence;
- add a behavior/property/metamorphic witness;
- include replay/provenance and a negative control where applicable;
- prove the mutant-active failure and final caught outcome; or
- produce the strict exception proof in section 4.14.

No additional survivor may be postponed because it was not one of the historical three. The anti-Goodhart denominator is the completed final population.

### 5.6 Tool-failure remediation deliverables

The mutation-infrastructure work is complete only when the evidence package contains:

- the lane-completion diagnostic report;
- raw 0042 historical transcript references;
- direct non-PTY reproduction or an explicit statement that the historical wrapper cannot be reproduced outside its host tool;
- supervisor regression cases and transcripts;
- one complete final unsharded run or a complete shard union;
- all partial outputs retained for failed attempts;
- a final completion manifest reconciling the list census to outcomes;
- explicit launcher and cargo-mutants exit statuses;
- a statement that no process-list heuristic was used as completion proof; and
- a reviewer signoff that the former PTY symptom is resolved or bypassed without weakening the denominator.

---

## 6. Failure handling and responsible-layer diagnostics

### 6.1 Failure classification

Every failure must be classified at the first responsible layer and must retain first-divergence evidence.

| Failure class | Required classification |
|---|---|
| Baseline test/build does not pass | baseline failure; no mutants interpreted |
| cargo-mutants returns 2 | completed or partial run with `missed` outcomes; inspect completion separately |
| cargo-mutants returns 3 | one or more per-mutant test timeouts; not a wrapper timeout and not a pass |
| wrapper wall limit fires | launcher/supervisor tool failure; denominator incomplete unless a complete reconciled output already proves otherwise |
| wrapper remains alive after child exit | child-reaping/PTY/supervisor failure |
| cargo-mutants disappears with no final outcome union | tool failure/incomplete |
| shard absent/cancelled/inconsistent | denominator incomplete |
| survivor remains behavior-relevant | named ORD-LIFE responsible layer plus tests/fixtures as applicable |
| replay/live divergence | projection/replay, event application, or earlier writer layer |
| wrong actor/current-start ownership | need accounting/action-pipeline ownership |
| hidden truth changes decision | actor-known context, candidate generation, planning/method selection, or proposal construction |
| direct scheduler dispatch | scheduler ordering/proposal construction |
| forged/stale proposal accepted | action validation |
| diagnostic lacks layer/blocker | tests/fixtures or originating decision/action layer |
| artifact overclaims pending/partial evidence | documentation status/evidence packaging |

### 6.2 Responsible-layer map

Use the existing execution-03 list:

- doctrine mismatch;
- content/schema validation;
- actor-known context construction;
- candidate generation;
- planning/method selection;
- proposal construction;
- scheduler ordering;
- action validation;
- event application;
- projection/replay;
- view-model rendering;
- debug quarantine;
- tests/fixtures; and
- documentation status.

For ORD-LIFE detail, diagnostics may further name the subcomponent without minting a new gate or status:

- need accounting;
- intention lifecycle;
- routine condition/method selection;
- local planning;
- action affordance/duration terminal;
- progress/stuck classification; and
- mutation infrastructure/configured-perimeter completion.

The last is the tooling/transport owner for evidence non-production; it does not replace the canonical semantic layer that owns a mutant's behavior.

### 6.3 Required failure package

Every failed/retried seam or tool attempt must record:

- exact final or development commit;
- exact command and environment;
- start/end timestamps and exit/signal status;
- responsible layer and component;
- expected versus actual certified consequence;
- first divergent event ID/log index/tick/actor/need where applicable;
- proposal/context/source-event ancestry where applicable;
- live versus replay comparison;
- mutant identity/diff or supervisor failure mode;
- retained stdout/stderr, structured outputs, diffs, and logs;
- evidence status (`fail`, `pending`, or historical); and
- remediation or routing decision.

### 6.4 Routing rule

A failed seam, incomplete run, unresolved timeout, unresolved tool failure, untriaged survivor, missing negative control, or incomplete template field yields:

```text
ORD-LIFE-CERT scoped remediation
```

The artifact must name the first responsible layer and a separately numbered follow-up. It may not use `passed with exceptions`, a percentage threshold, a partial pass, “tool flaky but probably clean,” or a phase exception.

---

## 7. Live re-proof of the ORD-LIFE contract

### 7.1 Re-proof rule

After remediation is frozen at the exact final commit, re-run the clean baseline, named test binaries, complete mutation posture, and 0042 seam evidence. Historical 0042 pass rows are prior evidence only. Shared SPINE/EPI event, replay, proposal, possession, and debug seams may appear as ancestry; they are not re-audited as independent predecessor gates.

### 7.2 `ORD-LIFE-01` through `ORD-LIFE-12` routing matrix

The 0042 spec controls the full audit details. This table is the remediation re-proof routing summary and does not replace the 0042 requirements.

| Audit point | Live seam to re-prove at final commit | Minimum remediation-specific evidence |
|---|---|---|
| `ORD-LIFE-01` | Bounded event-sourced needs, single-owner accounting, single-charge ledgers | Accounting mutation family; actor/need/tick ledger; duration boundary/ownership cases; live/replay equality; no second writer. |
| `ORD-LIFE-02` | Actor-known candidate generation, deterministic priority, hidden-target exclusion | Sealed actor-known inputs; hidden-food/route/workplace paired negatives; deterministic candidate ordering. |
| `ORD-LIFE-03` | Durable intention lifecycle, ancestry, replacement, possession neutrality | Legal/illegal lifecycle sequences; typed reasons; possession bind/unbind continuity; replay. |
| `ORD-LIFE-04` | Defeasible routines, HTN methods, interruptors, failure/fallback | Template census; applicability/failure/fallback evidence; no script/guaranteed arc. |
| `ORD-LIFE-05` | Routine temporal premises and modeled adaptation | Source-backed temporal premises; stale/absent premise negatives; no ground-truth schedule cognition. |
| `ORD-LIFE-06` | Actor-known method selection, bounded planning, coherent fallback | Provenance citations; planner budget boundaries; rejected alternatives; dangling-source failure. |
| `ORD-LIFE-07` | Planner/decision trace honesty and debug quarantine | Selected/rejected trace; debug on/off decision equality; no trace feedback to actor input. |
| `ORD-LIFE-08` | Affordances, movement, duration terminals, no teleport | Duration mutation family; movement ancestry; sleep/work/eat affordance rejection; terminal/interruption evidence. |
| `ORD-LIFE-09` | No-human orchestration, canonical recovery, meaningful progress, metric honesty | Integrated day; fail-only empty-food recovery; independent event classification versus metrics; no marker inflation. |
| `ORD-LIFE-10` | Typed stuck diagnostics and cross-tick no-progress | `no-progress-past-expected-window` and `repeated-idle`; blocker/layer/status; no silent loop. |
| `ORD-LIFE-11` | Scheduler no-direct-dispatch, sealed proposal ancestry, forged/stale rejection | Scheduler -> transaction -> proposal/stuck -> pipeline; forged/stale/context/route/affordance perturbations. |
| `ORD-LIFE-12` | Deterministic replay-derived projections/metrics/diagnostics and phase-entry lock | Empty rebuild; checksums/fingerprints; mutation denominator completion; acceptance wording/ledger lock. |

Every row requires positive, adversarial/negative, replay/provenance, mutation relevance, responsible-layer diagnostics, and honestly scoped evidence status.

### 7.3 Coverage of the ten live ORD-LIFE pass conditions

| Condition | Live requirement | Primary audit points | Required aggregate witness |
|---:|---|---|---|
| 1 | Needs, intentions, routines, and stuck state are event-sourced or replay-reconstructable. | `01`, `03`, `04`, `08`, `10`, `12` | Ordered event ancestry plus clean replay equality for all four state families and duration/accounting effects. |
| 2 | Candidate generation uses actor-known inputs only. | `02`, `05` | Sealed actor-known packet, source refs, paired hidden-truth equality, and true-time-without-premise negative. |
| 3 | Method selection and local planning cite actor-known provenance. | `04`, `05`, `06` | Selected/rejected methods, local plan, budget, target/route/affordance/temporal source-event citations, dangling-source negatives. |
| 4 | Scheduler cannot dispatch ordinary actions directly from needs or routines. | `08`, `11` | Scheduler -> actor transaction -> sealed proposal/stuck -> pipeline witness and direct-dispatch negative. |
| 5 | Action validation rejects forged/stale proposal parameters. | `06`, `08`, `11` | Context/actor/target/route/affordance/reservation/duration perturbation matrix with no accepted event on rejection. |
| 6 | No-human metrics count only real progress, modeled waits, or typed stuck/failure outcomes. | `09`, `10` | Independent event classification exactly matching metrics; continue-routine/debug/scheduler markers excluded. |
| 7 | Debug output can compare actor-known input against hidden truth without contaminating decisions. | `02`, `07` | Debug-on/off relational equality and explicit actor-known/hidden comparison under non-diegetic authority. |
| 8 | Replay rebuilds ordinary-life metrics and diagnostics. | `01`, `09`, `10`, `12` | Empty-projection rebuild of accounting, progress metrics, blockers/layers, and decision-context fingerprints. |
| 9 | Fixture failures identify the responsible layer. | all points | Every negative names canonical layer, component, expected/actual, event/source IDs, first divergence, and remediation category. |
| 10 | Phase 4 remains blocked until ORD-LIFE-CERT and preceding certification gates pass. | `09`, `12`, artifact contract | Predecessor citations plus complete final evidence, complete mutation result, wording/ledger guard, and no historical/partial promotion. |

No condition may substitute for another. A complete mutation row does not pass a seam missing positive/replay evidence; a green fixture does not forgive an incomplete denominator.

### 7.4 Mandatory ordinary-life fixture families

Every family must be registry-reachable and execute live at the final commit.

| Mandatory family | Verified baseline fixtures/witnesses | Minimum certifying proof |
|---|---|---|
| Integrated no-human day | `no_human_day_001`, `no_human_advance_001`, `sleep_eat_work_001`, `ordinary_workday_001` | Multiple actors advance without human input through actor-known transactions, real progress or typed failure, and clean replay; fail-only empty-food recovery remains explicit. |
| Food unavailable | `food_unavailable_replan_001`, `partial_food_source_knowledge_001`, `seeded_food_source_unknown_to_all_actors_001`, `hidden_food_unknown_route_001` | Hunger cannot reveal or target hidden food; actor searches, falls back, waits, or becomes stuck with typed evidence. |
| Routine no-teleport | `routine_no_teleport_001`, `embodied_exits_require_perceived_or_known_route_001`, `hidden_route_edge_001`, `work_completion_fails_when_actor_displaced_001` | Remote work/sleep/eat requires movement ancestry or legal duration semantics; hidden route truth/displacement cannot be normalized away. |
| Possession does not reset intention | `possession_does_not_reset_intention_001`, `possession_parity_001` | Bind/unbind changes controller authority only; need, routine, intention, memory, actor-known state, decisions, and replay remain continuous. |
| Hidden-truth planning | `no_hidden_truth_planning_001`, `hidden_food_unknown_route_001`, `no_human_unseen_workplace_assignment_does_not_plan_work_001`, `no_human_known_workplace_requires_provenance_001`, `forbidden_provenance_input_fails_closed_001` | Paired hidden differences do not affect cognition before legal evidence; forbidden/dangling provenance fails closed. |
| Planner trace | `planner_trace_001`, `method_fallback_requires_new_trace_or_stuck_001`, `debug_omniscience_excluded_001` | Selected/rejected candidates/methods, budget, plan/fallback, blockers, and actor-known refs are visible in debug but cannot feed cognition. |
| Routine blocker | `routine_blocked_diagnostic_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`, `scheduler_cannot_rewrite_wait_reason_after_transaction_001`, `work_block_failed_then_sleep_succeeds_001` | Blocked routine emits typed blocker/layer/status and coherent wait/fallback/stuck/failure; no silent loop, reason rewrite, or accounting drift. |

### 7.5 Cross-cutting fixture families

The remediation must also execute and package:

| Cross-cutting family | Baseline fixtures | Required use |
|---|---|---|
| Single-charge/duration boundaries | `sleep_spanning_window_boundary_charges_each_tick_once_001`, `wait_then_window_passive_charges_each_tick_once_001`, `sleep_interrupted_by_severe_need_prorates_recovery_001`, `work_block_failed_then_sleep_succeeds_001` | Primary kill/provenance family for the three historical seeds. |
| Sleep affordance | `sleep_rejects_current_place_without_sleep_affordance_001`, `no_human_current_place_without_sleep_affordance_does_not_sleep_001`, `embodied_view_omits_unknown_sleep_affordance_001` | Actor-known affordance and validation negatives. |
| Temporal premise/freshness | `ordinary_workday_001`, `no_human_workplace_knowledge_requires_notice_event_001`, `stale_workplace_notice_superseded_by_newer_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001` | Routine temporal premise evidence; consolidated temporal bundle remains downstream. |
| Safety interruption | `severe_safety_with_known_exit_produces_move_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001` | Interruptor, movement ancestry, wait/stuck, and duration terminal evidence. |
| Metric/layer honesty | `no_human_metrics_require_typed_responsible_layer_001`, `scheduler_cannot_rewrite_wait_reason_after_transaction_001` | Progress/stuck counting and responsible-layer integrity. |

If existing fixtures do not distinguish a final mutant, add the narrowest evidence fixture/test that travels the production seam. Do not change gameplay doctrine to make the fixture convenient.

### 7.6 Generated and metamorphic package

At minimum retain generated/metamorphic evidence for:

- actor/need/tick single-charge conservation under partitioning, action-duration insertion, interruption, and replay;
- duration start/terminal boundary ownership;
- current-start/log representation equivalence;
- unrelated-actor invariance;
- deterministic input/map/set order;
- hidden-truth non-interference;
- provenance deletion/substitution fail-closed behavior;
- legal/illegal intention and routine lifecycle sequences;
- planner-budget boundaries and coherent fallback;
- progress classification monotonicity/non-inflation;
- debug/possession non-interference; and
- replay prefix/suffix perturbation with first-divergence attribution.

Generated cases are supplemental breadth; member-level mutant-active evidence remains mandatory.

### 7.7 Predecessor continuity without re-audit

SPINE-CERT and EPI-CERT are already passed through their replacement artifacts. This remediation may exercise:

- event append/apply and replay;
- sealed proposal/context ancestry;
- holder-known/actor-known provenance;
- possession parity;
- view/debug separation; and
- action pipeline validation.

Those are continuity/ancestry evidence for ordinary-life consequences. The package must not present them as a second SPINE or EPI audit, alter their gate semantics, or reopen their settled verdicts.

---

## 8. Replacement acceptance and evidence-artifact contract

### 8.1 Artifact identity and package paths

The implementing session must produce a replacement acceptance artifact at the convention-compatible path:

```text
reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md
```

Archive it on accepted closeout as:

```text
archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md
```

The evidence package must also include or link stable paths for:

```text
reports/0043_ord_life_cert_mutation_lane_completion_diagnostic.md
reports/0043_ord_life_cert_mutation_triage_register.md
reports/0043_ord_life_cert_mutation_final_missed.txt
reports/0043_ord_life_cert_mutation_list_files.txt
reports/0043_ord_life_cert_mutation_list.txt
reports/0043_ord_life_cert_mutation_full.out/        # or complete shard directories/archive
reports/0043_ord_life_cert_command_transcripts/
reports/0043_ord_life_cert_emerge_obs.md             # observer-only companion or equivalent section
```

The exact storage layout may be refined before execution, but every reference must be stable, fingerprinted, and unambiguous before certifying commands run.

### 8.2 Header and environment block

The replacement artifact must state:

- spec identity/path;
- authoring baseline SHA;
- exact final implementation/evidence SHA;
- target repository;
- explicit statement that latest `main` was not independently verified;
- clean worktree evidence;
- OS/runner, CPU/memory, Rust, Cargo, cargo-mutants, and relevant tool versions;
- exact `.cargo/mutants.toml`, baseline-miss, Cargo.lock, CI, list, register, and output fingerprints;
- shard/job/timeout/supervisor policy;
- predecessor artifacts consumed;
- archived-spec historical-only statement; and
- aggregate verdict derived from evidence, not predeclared.

### 8.3 Command ledger

For every command, record:

- exact command and working directory;
- final SHA and worktree state;
- start/end timestamps and duration;
- launcher/supervisor status;
- child/cargo-mutants exit status;
- environment-affecting variables/options;
- output/transcript path;
- fingerprint scope and digest; and
- result/evidence status.

The ledger must include clean baseline, all named tests, `--list-files`, `--list`, full/shard mutation runs, retries, completion reconciliation, replay/golden commands, and artifact wording/conformance checks.

### 8.4 Evidence-item ledger fields

For every cited evidence item, instantiate all fields required by `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:[^repo-accept-template]

- evidence item ID;
- ORD-LIFE or mutation/completion cross-reference;
- `evidence-status`: pass/fail/pending/sampled/observer-only/historical;
- `fingerprint-scope`: raw bytes, normalized serialization, parsed semantic content, command transcript, run seed, replay artifact, or not applicable;
- evidence summary;
- path under test;
- behavior witness: command/trigger, responsible layer, accepted/rejected stage, live negative or reason none applies, and checked facts/invariants;
- replay/provenance ancestry: event/log range, serialized input, seed/randomness, manifest/schema/ruleset, projection/extraction version, and source provenance;
- sampling/exhaustiveness scope;
- pending/historical handling;
- certification use; and
- staged-abstraction declaration.

Local evidence IDs are navigation labels, not doctrine-level finding or gate IDs.

### 8.5 Per-seam verdict matrix

The artifact must contain a table at least this expressive:

| Requirement | Responsible layers | Positive evidence | Adversarial/negative evidence | Replay/provenance evidence | Mutation evidence | Result from certifying evidence |
|---|---|---|---|---|---|---|
| `ORD-LIFE-01` | need accounting; event application; projection/replay | IDs | IDs | IDs | three seed family + final survivors | pass/fail/pending |
| `ORD-LIFE-02` | actor-known context; candidate generation | IDs | IDs | IDs | linked final identities | derived result |
| `ORD-LIFE-03` | intention lifecycle; event application; replay | IDs | IDs | IDs | linked identities | derived result |
| `ORD-LIFE-04` | routines; method selection; tests/fixtures | IDs | IDs | IDs | linked identities | derived result |
| `ORD-LIFE-05` | context; method selection; temporal premises | IDs | IDs | IDs | linked identities | derived result |
| `ORD-LIFE-06` | planning/method selection; proposal construction | IDs | IDs | IDs | linked identities | derived result |
| `ORD-LIFE-07` | decision trace; debug quarantine | IDs | IDs | IDs | linked identities | derived result |
| `ORD-LIFE-08` | action validation; accounting; event application | IDs | IDs | IDs | three seed family + final identities | derived result |
| `ORD-LIFE-09` | scheduler; tests/fixtures; projection/replay | IDs | IDs | IDs | linked identities | derived result |
| `ORD-LIFE-10` | planning/decision; projection/replay; diagnostics | IDs | IDs | IDs | linked identities | derived result |
| `ORD-LIFE-11` | proposal; scheduler; action validation | IDs | IDs | IDs | linked identities | derived result |
| `ORD-LIFE-12` | projection/replay; documentation status | IDs | IDs | IDs | full completion/reconciliation | derived result |
| Configured mutation posture | mutation infrastructure plus semantic layer per identity | clean baseline/list IDs | no-shrink/no-laundering/supervisor controls | structured output/register | complete disposition matrix | derived result |
| Artifact/evidence honesty | documentation status; tests/fixtures | template/conformance IDs | pending/sampled/observer-only controls | fingerprints/claim map | register reconciliation | derived result |

Every row must cite live evidence at the final commit. A caught mutant strengthens only the seam it exercises; it cannot substitute for missing positive, negative, replay, or provenance evidence.

### 8.6 Completed mutation `pass` row

The mutation row may be `pass` only if it proves all of the following.

**Configured denominator**

- exact config fingerprint;
- effective final `--list-files` and `--list` outputs/fingerprints;
- presence of required files;
- explained count change from 60/2877;
- no silent filters, exclusions, or baseline laundering; and
- exact cargo-mutants version and invocation.

**Transport and completion**

- supervisor/launcher result;
- exact jobs/shards/timeouts;
- every run/shard completed;
- structured-output set reconciliation;
- zero missing/duplicate identities;
- zero unresolved launcher/tool failures;
- zero unresolved cargo-mutants timeouts; and
- retained outputs even for failed attempts.

**Tool outcome counts**

- total generated;
- caught;
- missed;
- timeout;
- unviable;
- baseline/build failures;
- tool/internal/diff/launcher failures; and
- reconciliation equation for all categories.

**Certification disposition counts**

- historical seed identities: `3`;
- historical identities still generated;
- historical identities source-changed and mapped;
- historical identities killed/caught;
- historical identities approved equivalent/unreachable;
- additional survivors exposed by completed run;
- additional survivors killed;
- approved equivalent identities;
- approved non-critical identities;
- blocked/untriaged identities: `0`;
- exceptions lacking independent signoff: `0`; and
- unresolved historical identities: `0`.

Tool outcomes and certification dispositions must not be added as one partition. An equivalent mutant remains `missed` as a tool outcome but has an approved equivalent disposition; it is never counted as caught.

**Member evidence**

- each historical identity has a reconciliation row;
- every final miss/timeout has a disposition row;
- every grouped family proves each member;
- every exception has complete call-site/domain proof and signoff;
- the final-missed manifest exactly matches the final tool set;
- the baseline-miss file contains no laundered ORD-LIFE identity; and
- the triage register and completion manifest are fingerprinted.

A high percentage, three targeted kills, or zero text lines in `missed.txt` is not a separate doctrine threshold. What matters is complete denominator execution, honest counts, behavior-relevant closure, and zero unresolved evidence gaps.

### 8.7 Ordinary-life behavior, replay, and provenance package

The artifact must include or link:

- actor/need/tick single-charge ledgers;
- duration start/terminal/interruption ownership matrices;
- event IDs and log ranges for all event-derived witnesses;
- live/rebuild projection, metric, need-state, intention/routine, and stuck-diagnostic comparisons;
- actor-known provenance tables for candidate/method/planner/action inputs;
- sealed proposal/context and action validation ancestry;
- hidden-truth/debug/possession paired-run matrices;
- progress/stuck independent classification;
- source/manifest/schema/tool fingerprints; and
- first-divergence packages for failed/retried witnesses.

### 8.8 `EMERGE-OBS` handling

The evidence package must include a separate `EMERGE-OBS` member or section with:

```text
Evidence status: observer-only
```

It must record corpus, observation, extraction method, and fingerprint scope, and state explicitly that it:

- does not pass or fail any ORD-LIFE seam;
- does not contribute to mutation disposition;
- defines no score or threshold;
- is not a scheduler objective or scenario goal; and
- cannot compensate for missing completion, replay/provenance, negative, accounting, or diagnostic evidence.

### 8.9 Staged-abstraction declaration

The artifact must declare either `none` or, for each bounded abstraction:

- what ordinary-life behavior is proven now;
- what future surface is deliberately abstracted;
- what the implementation/proof must not fake;
- which named downstream gate owns it;
- what evidence prevents overclaiming; and
- diagnostics distinguishing not implemented, intentionally abstracted, implemented but broken, and overclaimed.

No abstraction may cover a survivor, mutation completion, `ORD-LIFE-01`…`12`, actor-known causality, single charge, replay/provenance, progress/stuck honesty, or proposal/action ancestry.

### 8.10 Aggregate verdict and supersession

The replacement artifact may render exactly:

```text
ORD-LIFE-CERT passed
```

only when:

1. all `ORD-LIFE-01`…`12` rows pass from certifying evidence at the exact final commit;
2. all ten live pass conditions and seven mandatory fixture families pass;
3. the complete configured mutation posture has run and reconciled;
4. all historical and newly exposed survivors have final dispositions;
5. zero identities remain blocked/untriaged, zero exceptions lack signoff, and zero timeouts/tool failures/incomplete shards remain;
6. no-shrink, no-baseline-laundering, CI-trigger, single-charge, replay/provenance, truth-firewall, no-direct-dispatch, possession-continuity, and debug-quarantine controls pass;
7. all template fields are complete and honestly scoped; and
8. there is no upstream doctrine conflict.

The passing artifact must state that it **supersedes**:

```text
reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md
```

for current ORD-LIFE-CERT certification use. The 0042 artifact and triage register remain historical evidence of the audit, incomplete lane, and three-mutant seed.

If any aggregate condition is unmet, the artifact must remain:

```text
ORD-LIFE-CERT scoped remediation
```

and name the failing requirement, first responsible layer, retained evidence, and next separately numbered remediation action. It may not use a new status.

### 8.11 Closeout and ledger handling

On accepted closeout:

- archive this spec and the replacement report under their numbered paths;
- preserve 0042 and earlier artifacts as historical lineage;
- retain all mutation, replay, fixture, command, supervisor, and register evidence at stable paths;
- record final commit and artifact fingerprints;
- update `docs/4-specs/SPEC_LEDGER.md` according to repository convention;
- state the newly admissible next ladder move without beginning it; and
- do not promote this staging spec into the live doctrine tier.

---

## 9. Preliminary static survey — preliminary, not certification

This section is informative only. It records what static reading at the authoring baseline suggests; it does not prove reachability, kill status, lane completion, or a gate verdict.

### 9.1 Standing perimeter and baseline file

The checked-in mutation configuration expands mechanically to 60 manifest-listed source files and includes `need_accounting.rs` plus the six 0042 ORD-LIFE additions. The baseline-miss file is empty. The configuration uses cargo-mutants 27.1.0 semantics, locked Cargo arguments, and workspace tests.

### 9.2 Historical identities still map to live source

The live source still contains:

```rust
self.start_tick < tick
```

inside `DurationInterval::contains_tick`, and the current-start insertion guard still combines:

```rust
same actor && event id not already present in the log
```

inside `duration_intervals`. The three historical mutation operators therefore still map by enclosing symbol plus operator even if final implementation line numbers later move.

### 9.3 Why existing helper tests did not settle certification

The baseline unit tests exercise duration classification, including a sleep spanning a window boundary and a current start not yet round-tripped through the log. Static inspection suggests at least one existing boundary case begins the accounting window at the same tick as the duration start; that shape cannot distinguish `<` from `<=` because enumeration starts at the next tick. This observation motivates the inside-window witness in section 5.2. It is not a test verdict and does not establish why cargo-mutants missed the identity in every harness.

### 9.4 CI

The scheduled lane is configured to run and retain its output. The in-diff path expression is stale relative to the six 0042-added perimeter paths and requires the minimal correction in section 4.6.

### 9.5 Preliminary reachability conclusion

All three seed operators sit on the replay/accounting derivation path and are behavior-relevant until a final live witness or rigorous equivalent/unreachable proof demonstrates otherwise. The proper oracle is the public event-backed accounting/projection consequence, not the private interval helper.

---

## 10. Constitutional invariant alignment

This spec amends no invariant. The implementation must review every final delta against `INV-001` through `INV-112` and record the affected invariant set in the replacement artifact.

At minimum, the remediation must preserve these constitutional families:

- causal event trace and replay reconstruction;
- subjective/actor-known epistemics and truth firewall;
- ordinary agents rather than scripted arcs;
- action affordance, movement, duration, and no-teleport causality;
- one-way crate dependencies;
- deterministic replay and stable semantic fingerprints;
- debug quarantine and non-diegetic traces;
- possession continuity;
- typed diagnostics and responsible-layer attribution;
- no prose-born facts;
- no direct scheduler dispatch;
- single-owner, single-charge need/duration accounting; and
- evidence honesty for pending, sampled, observer-only, historical, and mutation results.

A genuine implementation/doctrine divergence requires upstream invariant amendment first. 0043 may not design around it.

---

## 11. Out of scope and tolerated deferrals

### 11.1 Settled predecessor scope

`P0-CERT`, `SPINE-CERT`, and `EPI-CERT` remain consumed, settled predecessor states. Shared surfaces may be exercised as ancestry but are not re-audited.

### 11.2 Downstream temporal evidence

The routine temporal-premise mechanism was audited by 0042 and must be re-proven here only to the ORD-LIFE seam contract. The consolidated temporal evidence bundle assigned by execution document `03` remains a `FIRST-PROOF-CERT` obligation. It is a routed dependency, not an ORD-LIFE pass/fail line.

### 11.3 Canonical recovery

The canonical `no_human_day_001` recovery remains `fail_only_empty_food_source`. The success-recovery ordinary-life variant remains deferred until later routine/economy work supplies actor-known public-food or neighbor/public-pantry affordances. This remediation must preserve the explicit fail-only behavior and must not silently stage hidden fallback food.

### 11.4 Later gates

The following remain deferred:

| Deferred surface | Owning future gate |
|---|---|
| Consolidated temporal evidence bundle | `FIRST-PROOF-CERT` |
| Success-recovery ordinary-life variant | downstream routine/economy work, then its owning certification gate |
| Institutions, records, wrong suspicion, procedures | `PHASE-4-ENTRY` |
| Notices, travel, regional scale, LOD, second-proof concerns | `SECOND-PROOF-ENTRY` |

### 11.5 Explicit non-goals

This spec does not implement or certify:

- personality simulation;
- complex social drama;
- full economy;
- institutions/households/norms/records;
- gossip, accusations, or wrong suspicion;
- LLM conversation or speech systems;
- notices/travel/regional scale/LOD;
- story-sifting or incidents;
- Phase-4 entry;
- first-proof aggregate certification; or
- latest-main status.

It also does not require backwards-compatibility shims, alias paths, or repository rearrangement for symmetry.

---

## 12. Risks and mandatory controls

| Risk | Failure mode | Mandatory control |
|---|---|---|
| Anti-Goodhart targeting | Kill only three named mutants or optimize score. | Complete final denominator; append every new survivor; member-level evidence. |
| Tool result conflation | Wrapper timeout recorded as cargo-mutants timeout or completion. | Separate launcher, denominator, tool exit, per-mutant, and certification axes. |
| PTY zombie/stall | Parent remains active after child exits or child remains unreaped. | Non-PTY final run; explicit wait/kill/wait; process-group supervisor; regression cases. |
| Denominator drift | Filter, config replacement, in-diff, iterate, or absent shard shrinks work. | Final list fingerprints; no forbidden options; set reconciliation. |
| Baseline laundering | Put survivors in baseline-miss file. | Empty-file fingerprint; strict exception proof; known seeds barred from convenience baseline. |
| Private-oracle testing | Test repeats helper predicate. | Public accounting/projection/replay behavior witness. |
| Boundary vacuity | Window starts at duration start, so `<`/`<=` is invisible. | Place start strictly inside the accounting window. |
| Cross-actor contamination | Current start for actor B affects actor A. | Unrelated-actor invariance and event-ownership cases. |
| Duplicate current start | Same event represented in log and current input double charges. | Representation equivalence and duplicate-event control. |
| Replay camouflage | Live path wrong but golden normalization hides it. | Independent event ledger plus empty-projection rebuild and first divergence. |
| Truth contamination | Expected target or state sourced from ground truth/debug. | Paired hidden-truth/debug negatives; actor-known provenance. |
| Predecessor scope creep | Re-audit SPINE/EPI or weaken their boundaries. | Treat shared seams as ancestry only. |
| Artifact overclaim | Pending/sampled/observer-only/historical counted as pass. | Template fields and aggregate verdict rule. |
| Concurrency instability | `-j 8` copied without capacity analysis. | Measured concurrency, explicit timeout margin, low-concurrency reproduction. |
| Artifact loss on failure | Failed job discards partial structured outputs. | `if: always()`/finally retention and stable paths. |

Owner choices may refine shard count, wall limit, exact artifact archive format, and test-family naming. They may not weaken any acceptance condition or leave an implementation design question about semantic behavior.

---

## 13. Completion checklist

### 13.1 Spec and posture

- [ ] Staging filename/path is exactly the 0043 path in the header.
- [ ] Future-spec posture is `Remediation` only.
- [ ] Admissibility remains `ORD-LIFE-CERT scoped remediation` until replacement evidence passes.
- [ ] No new gate/status/invariant/obligation/finding ID is minted.

### 13.2 Source discipline and commits

- [ ] Authoring baseline is exactly `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd`.
- [ ] Final implementation/evidence SHA is full, exact, and separately recorded.
- [ ] Latest `main` is not claimed.
- [ ] Final worktree is clean and all deltas are enumerated.
- [ ] Archived embedded SHAs are treated as historical content only.

### 13.3 Mutation perimeter and CI

- [ ] Final `--list-files` and `--list` captured and fingerprinted.
- [ ] Standing config loaded; no forbidden final filters.
- [ ] 60/2877 baseline deltas explained.
- [ ] Six omitted ORD-LIFE paths added to in-diff trigger.
- [ ] `STANDING_MUTATION_TRIGGER_FRAGMENTS` in `crates/tracewake-core/tests/ci_workflow_guards.rs` co-edited to match the corrected trigger, with a perimeter↔trigger consistency assertion preventing future drift.
- [ ] In-diff outputs retained and lane remains non-certifying.
- [ ] Baseline-miss file contains no laundered survivor.

### 13.4 Tool-failure resolution and completion

- [ ] Historical PTY symptom investigated and diagnostic report produced.
- [ ] Supervisor distinguishes normal exit, wall timeout, forced kill, cargo-mutants timeout, and tool failure.
- [ ] Child/process group is waited/reaped.
- [ ] Outputs retained on all failures.
- [ ] Final unsharded run or complete shard union terminates deterministically.
- [ ] Final census equals disjoint outcome union.
- [ ] Zero absent/inconsistent shards, unresolved timeouts, or tool failures remain.

### 13.5 Survivor triage

- [ ] All three historical identities mapped by symbol/operator.
- [ ] Start-boundary witness places start strictly inside the window.
- [ ] Actor/current-start conjunction witness uses a one-true/one-false guard row.
- [ ] Event-ID equality witness uses unrelated-log and duplicate-representation rows.
- [ ] Public accounting/projection/replay consequences kill the seeds.
- [ ] Every new survivor/timeout appended and triaged.
- [ ] Every exception has complete proof and independent signoff.
- [ ] Zero blocked/untriaged identities remain.

### 13.6 Live ORD-LIFE re-proof

- [ ] Clean baseline commands pass at final commit.
- [ ] All named core/content/TUI binaries pass.
- [ ] `ORD-LIFE-01` through `ORD-LIFE-12` have live evidence.
- [ ] All ten pass conditions are covered.
- [ ] All seven mandatory fixture families execute.
- [ ] Cross-cutting accounting/affordance/temporal/safety/metric families execute.
- [ ] Generated/metamorphic seeds and retained cases are recorded.
- [ ] SPINE/EPI are ancestry only, not re-audited.

### 13.7 Replacement artifact

- [ ] `docs/4-specs/0003` fields instantiated for every evidence item.
- [ ] Mutation pass row includes denominator, transport, outcomes, dispositions, and member evidence.
- [ ] `EMERGE-OBS` is observer-only.
- [ ] Staged abstractions declared or `none` recorded.
- [ ] Passing artifact explicitly supersedes the 0042 acceptance artifact.
- [ ] Failure route remains `ORD-LIFE-CERT scoped remediation` with named follow-up.
- [ ] Closeout archives files and updates ledger without beginning later work.

---

## Outcome

This specification authorizes no verdict. Its successful implementation must resolve both halves of the standing floor: the three known `need_accounting.rs` survivors **and** the configured mutation lane's inability to produce a complete, transport-honest denominator. It must then reconcile every final identity, re-prove the complete 0042 ordinary-life seam contract at one exact final commit, and publish the replacement artifact.

Until that artifact exists and satisfies section 8.10, the only admissible certification-line statement is:

```text
ORD-LIFE-CERT scoped remediation
```

---

## Appendix A — exact-commit repository evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 2befc078d96c3b221cad3aa7a0d9b04493a2f0cd
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open on full raw exact-commit URLs; selected already-ledgered text artifacts were also materialized from the same full URLs for synthesis
Requested file count: 223
Successfully verified file count: 223
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

Complete URL ledger companion:

```text
tracewake_2befc07_exact_url_evidence_ledger.txt
SHA-256: 51b008058252b26d984c48affd9382dfbe9dfc948a4ad3180587ae4c1db9288e
```

Every selected repository path appeared exactly in the uploaded manifest. The complete ledger contains all 223 requested exact URLs. No substantive repository claim in this spec depends on a branch fetch, search snippet, memory, prior chat, another repository, or an unfetched path.

## Appendix B — authoring-baseline configured mutation file census

Mechanical expansion of `.cargo/mutants.toml` against the authoring-baseline manifest yields these 60 unique files:

- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/candidate.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/generation.rs`
- `crates/tracewake-core/src/agent/htn.rs`
- `crates/tracewake-core/src/agent/intention.rs`
- `crates/tracewake-core/src/agent/methods.rs`
- `crates/tracewake-core/src/agent/mod.rs`
- `crates/tracewake-core/src/agent/need.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/agent/perception.rs`
- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/need_accounting.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/registry.rs`
- `crates/tracewake-core/src/actions/defs/need_events.rs`
- `crates/tracewake-core/src/actions/defs/eat.rs`
- `crates/tracewake-core/src/actions/defs/sleep.rs`
- `crates/tracewake-core/src/actions/defs/work.rs`
- `crates/tracewake-core/src/actions/defs/wait.rs`
- `crates/tracewake-core/src/actions/defs/continue_routine.rs`
- `crates/tracewake-core/src/actions/defs/movement.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/events/mod.rs`
- `crates/tracewake-core/src/events/mutation.rs`
- `crates/tracewake-core/src/replay/mod.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-core/src/checksum.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-core/src/debug_reports.rs`
- `crates/tracewake-core/src/epistemics/belief.rs`
- `crates/tracewake-core/src/epistemics/contradiction.rs`
- `crates/tracewake-core/src/epistemics/knowledge_basis.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/epistemics/mod.rs`
- `crates/tracewake-core/src/epistemics/observation.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/epistemics/proposition.rs`
- `crates/tracewake-content/src/manifest.rs`
- `crates/tracewake-content/src/load.rs`
- `crates/tracewake-content/src/schema.rs`
- `crates/tracewake-content/src/serialization.rs`
- `crates/tracewake-content/src/validate.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/debug_panels.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/src/transcript.rs`

The final implementation must regenerate this census with cargo-mutants. This appendix is baseline orientation, not final denominator evidence.

## Appendix C — external research sources shaping implementation mechanics

External sources inform tooling/test design only; they do not establish target-repository state.

1. cargo-mutants, “Filtering files” — config filters and `--list-files`/`--list` denominator preview.[^cargo-filtering]
2. cargo-mutants, “Workspaces and packages” — workspace and test-workspace selection.[^cargo-workspaces]
3. cargo-mutants, “Baseline tests” — why skipped/failing baselines make results meaningless.[^cargo-baseline]
4. cargo-mutants, “Hangs and timeouts” — per-mutant timeout semantics.[^cargo-timeouts]
5. cargo-mutants, “Exit codes” — distinct success, miss, timeout, baseline, and tool statuses.[^cargo-exit-codes]
6. cargo-mutants, “The mutants.out directory” — structured/incremental output files.[^cargo-output]
7. cargo-mutants, “Sharding” — identical arguments and complete shard denominator requirements.[^cargo-shards]
8. cargo-mutants, “Parallelism” — contention and timeout variability at high job counts.[^cargo-parallelism]
9. cargo-mutants, “Testing code changed in a diff” — in-diff is not a substitute for full testing.[^cargo-in-diff]
10. Rust standard library `std::process::Child` — child processes are not automatically terminated/reaped on drop.[^rust-child]
11. util-linux `script(1)` — `-e/--return` propagates child status for PTY diagnostics.[^util-linux-script]
12. GNU Coreutils `timeout` — wall-clock termination and distinguishable exit statuses.[^gnu-timeout]
13. GitHub Actions matrix and artifact documentation — reproducible shard jobs and retained evidence.[^github-matrix][^github-artifacts]
14. Claessen and Hughes, QuickCheck — executable properties over generated inputs.[^quickcheck]
15. Proptest state-machine testing — legal transition generation and shrinking under preconditions.[^proptest-state]
16. Bartocci et al., Property-Based Mutation Testing — tie meaningful kills to named property violation.[^property-mutation]
17. Segura et al., Survey on Metamorphic Testing — relations as oracles where direct expected outputs are brittle.[^metamorphic]

---

## Source notes

[^repo-docs-readme]: [Tracewake documentation authority](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/README.md).
[^repo-invariants]: [Constitutional invariants](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md).
[^repo-arch-index]: [Architecture index and conformance](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md).
[^repo-exec-index]: [Execution index and authority](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md).
[^repo-ladder]: [Phase ladder, gate order, and certification sequence](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md).
[^repo-exec-ordinary]: [Ordinary-life needs, routines, and no-human proof](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md).
[^repo-exec-testing]: [Testing, observability, diagnostics, and review artifacts](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md).
[^repo-risk]: [Design risk register](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/3-reference/01_DESIGN_RISK_REGISTER.md).
[^repo-spec-rules]: [Rules for future specs](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/4-specs/README.md).
[^repo-ledger]: [Live spec ledger](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/4-specs/SPEC_LEDGER.md).
[^repo-accept-template]: [Acceptance artifact template](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md).
[^repo-0037-acceptance]: [0037 P0-CERT replacement acceptance](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md).
[^repo-0039-acceptance]: [0039 SPINE-CERT replacement acceptance](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md).
[^repo-0041-acceptance]: [0041 EPI-CERT replacement acceptance](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md).
[^repo-0042-spec]: [0042 ORD-LIFE-CERT audit spec](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md).
[^repo-0042-acceptance]: [0042 ORD-LIFE-CERT acceptance artifact](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md).
[^repo-0042-triage]: [0042 ORD-LIFE-CERT mutation triage register](https://raw.githubusercontent.com/joeloverbeck/tracewake/2befc078d96c3b221cad3aa7a0d9b04493a2f0cd/reports/0042_ord_life_cert_mutation_triage_register.md).

[^cargo-filtering]: [cargo-mutants: Filtering files](https://mutants.rs/skip_files.html).
[^cargo-workspaces]: [cargo-mutants: Workspaces and packages](https://mutants.rs/workspaces.html).
[^cargo-baseline]: [cargo-mutants: Baseline tests](https://mutants.rs/baseline.html).
[^cargo-timeouts]: [cargo-mutants: Hangs and timeouts](https://mutants.rs/timeouts.html).
[^cargo-exit-codes]: [cargo-mutants: Exit codes](https://mutants.rs/exit-codes.html).
[^cargo-output]: [cargo-mutants: The mutants.out directory](https://mutants.rs/mutants-out.html).
[^cargo-shards]: [cargo-mutants: Sharding](https://mutants.rs/shards.html).
[^cargo-parallelism]: [cargo-mutants: Parallelism](https://mutants.rs/parallelism.html).
[^cargo-in-diff]: [cargo-mutants: Testing code changed in a diff](https://mutants.rs/in-diff.html).
[^rust-child]: [Rust standard library: `std::process::Child`](https://doc.rust-lang.org/std/process/struct.Child.html).
[^util-linux-script]: [util-linux `script(1)` manual](https://man7.org/linux/man-pages/man1/script.1.html).
[^gnu-timeout]: [GNU Coreutils manual: `timeout`](https://www.gnu.org/software/coreutils/manual/html_node/timeout-invocation.html).
[^github-matrix]: [GitHub Actions: Running variations of jobs in a workflow](https://docs.github.com/actions/writing-workflows/choosing-what-your-workflow-does/running-variations-of-jobs-in-a-workflow).
[^github-artifacts]: [GitHub Actions: Store and share data with workflow artifacts](https://docs.github.com/actions/using-workflows/storing-workflow-data-as-artifacts).
[^quickcheck]: [Claessen and Hughes, “QuickCheck: A Lightweight Tool for Random Testing of Haskell Programs”](https://dl.acm.org/doi/10.1145/351240.351266).
[^proptest-state]: [Proptest: State machine testing](https://proptest-rs.github.io/proptest/proptest/state-machine.html).
[^property-mutation]: [Bartocci et al., “Property-Based Mutation Testing”](https://arxiv.org/abs/2301.13615).
[^metamorphic]: [Segura et al., “A Survey on Metamorphic Testing”](https://eprints.whiterose.ac.uk/id/eprint/110335/1/segura16-tse.pdf).
