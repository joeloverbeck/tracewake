# 0045 — FIRST-PROOF-CERT mutation remediation and replacement certification spec

**Staging path:** `specs/0045_FIRST_PROOF_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
**Archive path on accepted closeout:** `archive/specs/0045_FIRST_PROOF_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
**Target repository:** `joeloverbeck/tracewake`  
**Target commit / authoring and reassessment baseline:** `fd5ae94ff3225d2f989262b95ed8272945861516`  
**Spec series:** numbered staging spec `0045`, staged under `specs/`, archived by plain rename on accepted closeout  
**Status:** PROPOSED — NON-EXECUTED REMEDIATION CONTRACT  
**Future-spec posture:** `Remediation`  
**Admissibility posture:** `FIRST-PROOF-CERT scoped remediation`  
**Consumed predecessor gates:** `P0-CERT passed` through the 0037 replacement artifact; `SPINE-CERT passed` through the 0039 replacement artifact; `EPI-CERT passed` through the 0041 replacement artifact; `ORD-LIFE-CERT passed` through the 0043 replacement artifact  
**Certification-line effect:** successful execution must publish a replacement acceptance artifact rendering `FIRST-PROOF-CERT passed` and explicitly superseding the 0044 FIRST-PROOF-CERT acceptance artifact  
**Assumption:** stage under `specs/`; archive to the path above on accepted closeout; do not promote this staging-series spec into `docs/4-specs/`  
**Execution character:** this is a non-executable specification; it prescribes implementation, execution, evidence, review, and packaging work but performs none of it

**Determination confirmation.** The live phase ladder places `FIRST-PROOF-CERT` after the already-passed `P0-CERT`, `SPINE-CERT`, `EPI-CERT`, and `ORD-LIFE-CERT` lines and before `PHASE-4-ENTRY`. The four replacement artifacts establish those predecessor states within their exact-commit scopes. The 0044 acceptance artifact and mutation register render `FIRST-PROOF-CERT scoped remediation`, identify the incomplete configured mutation campaign as the blocking floor, and require a complete standing campaign or an accepted replacement evidence contract before the line can pass. Execution doctrine requires an incomplete gate to route to remediation naming the failing layer and forbids relabeling it a phase exception. Therefore this 0045 mutation-remediation/replacement-certification spec—not feature expansion, institutions, Phase-4 entry, second-proof work, or another audit—is the single next admissible move.[^repo-ladder][^repo-0037-acceptance][^repo-0039-acceptance][^repo-0041-acceptance][^repo-0043-acceptance][^repo-0044-acceptance][^repo-0044-triage][^repo-ledger]

This spec is subordinate to foundation, architecture, execution, reference, and live spec-layer doctrine. It amends no invariant, weakens no gate, mints no canonical gate code, creates no status enum, creates no obligation code, creates no invariant ID, and creates no doctrine-level finding ID. `FIRST-PROOF-CERT` remains a phase-certification label composing the canonical nine first-proof gates and their review artifacts.[^repo-docs-readme][^repo-arch-index][^repo-exec-index][^repo-first-proof-scope][^repo-spec-rules]

This document renders **no pass/fail verdict**. It does not assert that the standing campaign has completed, that a survivor has been exposed, that any mutant is killed, or that replay, tests, CI, or acceptance artifacts are green. Every command, result field, count, SHA, signature, and verdict to be produced by the implementing session remains pending until actually obtained at the exact final implementation/evidence commit.

> I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching repository files only by exact commit URL from `joeloverbeck/tracewake`. References to other repositories inside those validly fetched files are treated as file content, not as provenance contamination.

---

## 1. Status, determination, source discipline, freshness, and admissibility

### 1.1 Single posture and certification-line state

This spec declares exactly one future-spec posture: `Remediation`. It fixes the named `FIRST-PROOF-CERT` mutation-evidence completion failure and adds no gameplay scope.[^repo-ladder]

The line remains:

```text
FIRST-PROOF-CERT scoped remediation
```

until an implementing session satisfies this contract and publishes a complete replacement acceptance artifact based on live certifying evidence at one exact final implementation/evidence commit. This specification alone cannot flip the line.

| Certification line | Controlling artifact consumed by 0045 | Settled state |
|---|---|---|
| `P0-CERT` | `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` | `P0-CERT passed` within the artifact's exact-commit scope |
| `SPINE-CERT` | `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` | `SPINE-CERT passed` within the artifact's exact-commit scope |
| `EPI-CERT` | `archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md` | `EPI-CERT passed` within the artifact's exact-commit scope |
| `ORD-LIFE-CERT` | `archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md` | `ORD-LIFE-CERT passed` within the artifact's exact-commit scope |
| `FIRST-PROOF-CERT` | `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` | `FIRST-PROOF-CERT scoped remediation`; supersession target |

The 0044 audit is complete as an audit. This spec does not re-commission or redesign it. It treats `FIRST-PROOF-01` through `FIRST-PROOF-17`, the nine first-proof gates, the nine scenario families, the five-source temporal bundle, and the existing 0044 mutation-floor acceptance row as the seams to re-prove after remediation.[^repo-0044-spec][^repo-0044-acceptance]

The 0044 artifact uses `FIRST-PROOF-18` as a local acceptance-report cross-reference for the mutation floor, although the authored 0044 audit's declared navigation labels are `FIRST-PROOF-01` through `FIRST-PROOF-17`. This spec preserves that historical cross-reference when discussing the predecessor artifact but does not mint, promote, or reinterpret it as a canonical gate or a new doctrine obligation.

### 1.2 Why this is the next admissible move

The live sequence is:

```text
P0-DOC -> P0-CERT -> SPINE-CERT -> EPI-CERT -> ORD-LIFE-CERT
       -> FIRST-PROOF-CERT -> PHASE-4-ENTRY -> SECOND-PROOF-ENTRY
```

Execution document `03` says a failed or incomplete gate must produce remediation naming the failing layer and may not be relabeled a phase exception. Execution document `10` says pending, incomplete, tool-failed, sampled, observer-only, or historical evidence cannot be silently counted as a pass. The 0044 register states that `FIRST-PROOF-CERT` cannot pass until the standing perimeter campaign is completed or an accepted remediation spec changes the evidence contract. The settled decision here is to **complete the full standing contract, not shrink or replace it with a smaller denominator**.[^repo-ladder][^repo-testing][^repo-0044-triage]

Accordingly, 0045:

- performs no alternative-feature survey;
- proposes no gameplay expansion;
- does not begin institutions, records, wrong-suspicion, notices, travel, regional scale, LOD, or story-sifting work;
- does not reopen the four predecessor certification audits;
- does not certify latest `main`; and
- does not treat the 0044 focused seven-file wave as the standing certifying denominator.

### 1.3 Exact-commit source discipline

The uploaded manifest is path inventory only. It is not repository content, repository metadata, branch state, or proof that the supplied commit is latest `main`.

All repository-state claims used to author this spec derive only from manifest-listed paths fetched by full exact URL containing:

```text
owner:  joeloverbeck
repo:   tracewake
commit: fd5ae94ff3225d2f989262b95ed8272945861516
path:   the exact manifest path
```

No clone, branch-name file fetch, default-branch lookup, GitHub code search, snippet reconstruction, repository-scoped connector metadata, or connector namespace label was used as target-repository evidence. The complete append-only acquisition ledger contains 238 requested and 238 successfully verified exact URLs; Appendix A summarizes it and the companion ledger retains the complete URL list.

Commit strings embedded in archived specs, acceptance artifacts, reports, tickets, or research briefs—including `1541da2`, `c819bbe`, `726b2a1`, `92ba47f`, and others—are those artifacts' own audit or authoring provenance. They are ordinary fetched content and are not this authoring baseline. A validly fetched target-repository file may mention `joeloverbeck/one-more-branch` or any other repository without contaminating fetch provenance.

Archived specs, tickets, reports, command transcripts, and predecessor acceptance artifacts are history, lineage, or structural precedent unless live doctrine explicitly consumes a passed replacement artifact. They never substitute for live execution at the final 0045 implementation/evidence commit.[^repo-ledger]

### 1.4 Authoring baseline versus final implementation/evidence commit

The replacement package must report at least these commit identities separately:

| Role | Required value and use |
|---|---|
| Authoring/reassessment baseline | `fd5ae94ff3225d2f989262b95ed8272945861516`; exact source state against which this spec and preliminary survey were authored. |
| Final implementation/evidence commit | One full exact SHA created after every production, test, fixture, workflow, mutation-config, documentation, and evidence-path change needed by 0045. Every certifying command and shard must run against this same SHA. |
| Optional documentation-only closeout commit | Permitted only after evidence freeze and only if it changes no audited production, test, fixture, schema, mutation, CI, or evidence content; must be reported separately and never substituted for the final implementation/evidence SHA. |

The implementing session must enumerate every delta from the authoring baseline to the final implementation/evidence commit, including moved/deleted symbols, tests, fixtures, `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, `.github/workflows/ci.yml`, workflow guards, supervisor tooling, shard-merger tooling, and report paths.

Certification may not aggregate:

- commands run at different source/test/config commits;
- shards run with different Cargo.lock, toolchain, cargo-mutants version, config fingerprint, test selection, environment-affecting options, or denominator;
- mutation outcomes generated before a subsequent source, test, fixture, config, or workflow edit; or
- predecessor command output as if it were live 0045 execution.

Neither the authoring baseline nor the final implementation/evidence SHA may be described as independently verified current `main` without a separately supplied current-main SHA and matching manifest.

### 1.5 Admissibility lock

Until the passing replacement artifact lands:

- no later spec may cite `FIRST-PROOF-CERT passed`;
- `PHASE-4-ENTRY` remains locked;
- `SECOND-PROOF-ENTRY` remains locked;
- institutions, records, wrong-suspicion, notices, travel, regional scale, LOD, story-sifting, and LLM dialogue remain outside this package;
- a focused run, partial run, partial shard set, high mutation percentage, zero misses observed so far, or empty baseline-miss file cannot substitute for completion; and
- no implementation convenience may weaken the nine gates, the nine scenario families, the temporal bundle, replay determinism, or the standing mutation denominator.

On accepted closeout, archive this staging spec by plain rename, archive the passing replacement report under the corresponding numbered report path, preserve 0044 as historical audit evidence, and update `docs/4-specs/SPEC_LEDGER.md` according to repository convention.

---

## 2. Authority and dependency declarations

### 2.1 Controlling order

The implementing session resolves conflicts in this order:

1. `docs/0-foundation/`;
2. `docs/1-architecture/`;
3. `docs/2-execution/`;
4. `docs/3-reference/`;
5. `docs/4-specs/` and the live ledger;
6. this remediation spec;
7. implementation convenience.

If execution conflicts with architecture or foundation, execution is wrong. If an implementation is easier than an accepted gate but violates it, the implementation is wrong. If a test kills a mutant by asserting the mutated helper literal instead of a certified first-proof consequence, the test is wrong. If an artifact promotes an incomplete, shrunken, sampled, historical, or observer-only result into a pass, the artifact is invalid.[^repo-docs-readme][^repo-testing]

### 2.2 Primary remediation and lineage sources

The load-bearing delta sources are:

- 0044 audit spec — the integrated seam contract to re-prove;
- 0044 acceptance artifact — the scoped-remediation verdict and behavioral evidence being superseded;
- 0044 mutation triage register — the exact completion blocker, perimeter additions, stable identity conventions, and required follow-up;
- 0043 remediation spec and passing artifact — closest structural precedent for completion proof, supervision, sharding, outcome separation, triage, live re-proof, and replacement supersession;
- 0041, 0039, and 0037 remediation specs and passing artifacts — canonical mutation-remediation anatomy; and
- the 0040/0042/0043 triage formats — register and final-missed-manifest precedent.[^repo-0044-spec][^repo-0044-acceptance][^repo-0044-triage][^repo-0043-spec][^repo-0041-spec][^repo-0039-spec][^repo-0037-spec]

These archived documents shape structure and preserve lineage. They do not establish live results for the final 0045 commit.

### 2.3 Foundation dependencies

The complete foundation tier remains controlling. Primary dependencies are:

| Foundation source | Binding use in 0045 |
|---|---|
| `docs/README.md` | Authority layering and implementation-convenience rule. |
| `02_CONSTITUTIONAL_INVARIANTS.md` | Constitution; especially `INV-111` observer-only emergence and `INV-112` temporal authority. |
| `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` | Event ancestry, append/apply discipline, deterministic replay, and divergence evidence. |
| `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` | Source-backed expectation, belief, observation, contradiction, and memory boundaries. |
| `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` | Ordinary action/affordance behavior consumed by container checks and no-human scenarios. |
| `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` | Possession parity and embodied/debug separation. |
| `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` | Missing-property first playable, no-human world, nine gates, explicit non-goals, and observer-only emergence evidence. |
| `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` | Actor-known transaction and truth firewall; witness expectations may not be sourced from ground truth. |

Foundation documents `01`, `05`, `07`, `09`, `10`, `11`, and `13` bound charter, planning, institutions, authoring, scale, LLM/speech, and research history. They are read for boundary awareness and are not remediation targets.

### 2.4 Architecture dependencies

| Architecture source | Binding use in 0045 |
|---|---|
| `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Canonical gate/review-artifact composition; `FIRST-PROOF-CERT` mints no new gate. |
| `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` | Preserve `core <- content <- tui`; no test-only production dependency. |
| `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | Replay-derived projections/diagnostics and deterministic divergence localization. |
| `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | Sealed holder-known context, actor-known source sufficiency, and freshness. |
| `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | No-direct-dispatch and no alternate state mutation path; consumed, not re-audited in isolation. |
| `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | Expectation contradiction and source-bearing cognition. |
| `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | Property, custody, access, locality, and no-teleport consequences. |
| `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Embodied output derives from holder-known context; debug is non-diegetic. |
| `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Typed diagnostics, review-artifact composition, and responsible layers. |

Architecture `05`, `07`, `08`, `11`, `12`, and `14` are boundary awareness for decision internals, speech, institutions, incidents/leads/story-sifting, LOD, and forbidden misreads. They may not be pulled into first-proof remediation as feature scope.

### 2.5 Execution dependencies and canonical vocabulary

| Execution source | Binding use in 0045 |
|---|---|
| `00_EXECUTION_INDEX_AND_AUTHORITY.md` | Canonical gate index; `FIRST-PROOF-CERT` is a composing label. |
| `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` | Allowed result postures: `passed`, `scoped remediation`, `not applicable`. |
| `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` | Nine gates, nine scenario families, definition of done, missing-property surface, and explicit non-goals. |
| `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` | Gate order, failure routing, `Remediation` posture, temporal cascade, and entry locks. |
| `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` | No fact from prose, no unobserved truth decision, no truth-selected target/suspect/culprit. |
| `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | Scheduler/proposal/event boundary retained by all witnesses. |
| `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | No-human ordinary day and routine temporal evidence consumed inside the integrated proof. |
| `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | Possession, holder-known view, debug quarantine, and embodied temporal rendering. |
| `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` | Forbidden authoring forms, the validation pipeline, provenance sufficiency, and fail-closed typed content rejection governing the `validate.rs` seam and the content-rejection witnesses. |
| `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` | Positive/adversarial fixture families, replay pairing, and content rejection. |
| `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Evidence honesty, behavior witnesses, responsible layers, mutation completion, review artifacts, temporal diagnostics, and `EMERGE-OBS`. |
| `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` | Downstream boundary; institution/investigation work remains locked. |
| `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` | Second-proof deferral boundary. |

Use `actor-known` for the actor case and `holder-known context` as the system-wide term. Use `missing property`, `expectation contradiction`, `no-human proof`, `possession parity`, `behavioral progress`, `stuck diagnostic`, and `EMERGE-OBS` exactly as defined by the glossary.[^repo-glossary]

### 2.6 Reference and live-spec dependencies

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — review questions;
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — exact-commit drift, contamination, hidden-truth leakage, replay drift, silent starvation, incomplete correction closure, decorative locks, acceptance overstatement, and anti-Goodhart controls;
- `docs/3-reference/02_GLOSSARY.md` — binding terms;
- `docs/4-specs/SPEC_LEDGER.md` — current certification-line state and predecessor scopes;
- `docs/4-specs/README.md` — future-spec posture, source discipline, gate-code, and staging rules;
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — evidence-item, fingerprint, replay/provenance, sampling, pending/historical, certification-use, and staged-abstraction fields; and
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — live village/fixture ontology for the integrated missing-property proof.[^repo-risk][^repo-template][^repo-village]

### 2.7 Crate direction and implementation boundary

The remediation must preserve:

```text
tracewake-core <- tracewake-content <- tracewake-tui
```

Permitted work includes production corrections in `tracewake-core`; fixture/schema/validation work in `tracewake-content` that consumes core APIs; TUI evidence changes that consume lower layers; tests in owning crates; workflow, supervisor, merger, and artifact-retention changes; and numbered reports/spec closeout.

Forbidden work includes:

- core depending on content or TUI;
- production code depending on test or fixture-only helpers;
- test-only bypasses around proposal, validation, event append/application, projection, replay, or holder-known context;
- backwards-compatibility aliases or shadow paths created solely to preserve obsolete tests;
- debug, prose, or ground-truth inputs feeding actor cognition;
- scheduler-authored ordinary state deltas or direct ordinary-action dispatch;
- a human-controller special case;
- mutation annotations, exclusions, or source restructuring whose primary purpose is denominator shrink rather than legitimate semantic simplification; and
- acceptance logic that treats an artifact's existence as its behavioral proof.

### 2.8 Primary code, test, configuration, and evidence seams

**Mutation configuration and CI**

- `.cargo/mutants.toml`
- `.cargo/mutants-baseline-misses.txt`
- `.github/workflows/ci.yml`
- `tools/supervise-command.sh`
- `crates/tracewake-core/tests/ci_workflow_guards.rs`
- `Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml`

**FIRST-PROOF mutation carriers**

- `crates/tracewake-core/src/time.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/epistemics/contradiction.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/agent/perception.rs`
- `crates/tracewake-core/src/actions/defs/checkcontainer.rs`
- `crates/tracewake-content/src/validate.rs`

These are the minimum named first-proof carriers, not a substitute for the complete standing perimeter. Appendix B records the full authoring-baseline 62-file selected set.[^repo-mutants-config][^repo-mutants-files]

**Integrated core harnesses**

- `acceptance_gates.rs`
- `acceptance_artifact_wording.rs`
- `golden_scenarios.rs`
- `hidden_truth_gates.rs`
- `event_schema_replay_gates.rs`
- `negative_fixture_runner.rs`
- `no_human_capstone.rs`
- `emergence_ledger.rs`
- `generative_lock.rs`
- `spine_conformance.rs`
- `anti_regression_guards.rs`
- `ci_workflow_guards.rs`
- `doc_invariant_references.rs`
- `tests/support/`

**Content and TUI harnesses**

- `crates/tracewake-content/tests/fixtures_load.rs`
- `forbidden_content.rs`
- `golden_fixtures_run.rs`
- `schema_conformance.rs`
- all seven manifest-listed TUI integration tests
- the complete manifest-listed `crates/tracewake-content/src/fixtures/` corpus
- the compile-fail/negative fixture corpus under `tests/negative-fixtures/`

**0044 evidence inputs**

- 0044 acceptance artifact;
- 0044 mutation triage register;
- 0044 `--list-files` and `--list` outputs;
- standing and focused supervisor command, metadata, status, stdout, and stderr records; and
- the retained focused/standing mutation-output trees where present.

A manifest path, fixture name, snapshot, test name, or static source pattern is orientation only until a live behavior witness at the final implementation/evidence commit exercises the production path.

---

## 3. Problem statement

### 3.1 The 0044 verdict and exact standing floor

The 0044 integrated behavioral audit passed its recorded behavioral scope but could not render `FIRST-PROOF-CERT passed` because the configured standing mutation campaign did not complete.[^repo-0044-acceptance][^repo-0044-triage]

| Field | 0044 recorded result |
|---|---|
| Standing command | supervised `cargo mutants --workspace --no-shuffle` loading checked-in `.cargo/mutants.toml` |
| Supervisor wall | 7,200 seconds, 30-second grace |
| Supervisor result | `wrapper_wall_timeout` |
| Exit status | `124` |
| Effective standing census | 62 selected files / 2,901 generated mutants |
| Classified before interruption | 2,384 |
| Caught before interruption | 1,869 |
| Missed before interruption | 0 |
| Mutant-level timeout before interruption | 0 |
| Unviable before interruption | 515 |
| Unclassified at interruption | 517 |
| Interruption locus | build-phase execution while processing `crates/tracewake-core/src/events/apply.rs`; stderr records `ERROR interrupted` and `scenario execution internal error err=interrupted phase=Build` |
| Certification consequence | incomplete denominator; blocking regardless of zero misses/timeouts observed so far |

The corrected focused FIRST-PROOF development wave completed separately over seven files using `--no-config` and explicit `-f` filters: `epistemics/contradiction.rs`, `epistemics/projection.rs`, `agent/perception.rs`, `actions/defs/checkcontainer.rs`, `events/apply.rs`, `replay/rebuild.rs`, and `tracewake-content/src/validate.rs`. Its recorded summary was `caught 600`, `missed 0`, `timeout 0`, and `unviable 119`:

| Field | Focused result |
|---|---:|
| Mutants | 719 |
| Caught | 600 |
| Missed | 0 |
| Mutant-level timeout | 0 |
| Unviable | 119 |
| Supervisor result | `child_exit_0` |

That focused result demonstrates no known focused survivor floor. It does **not** classify the complete standing population and cannot certify the line.[^repo-0044-triage]

### 3.2 Central divergence from 0043

The 0043 remediation inherited three known actionable `need_accounting.rs` survivors and a non-completing configured lane. Its center of gravity was survivor killing plus tool-failure resolution.

The 0045 starting state is materially different:

- there is no known actionable `missed` survivor in the completed focused wave;
- there is no known `missed` or mutant-level `timeout` in the classified portion of the standing wave;
- the sole established blocker is that the standing denominator was not completely classified within the external wall; and
- approximately 517 generated identities remain unclassified by that interrupted run.

Therefore **campaign completion is the dominant remediation objective**. Survivor handling remains mandatory machinery because completing the campaign may expose misses or mutant-level timeouts, but this spec must not fabricate such a list before the complete run exists.

### 3.3 Static perimeter confirmation at the authoring baseline

At `fd5ae94ff3225d2f989262b95ed8272945861516`:

- `.cargo/mutants.toml` uses `test_workspace = true`, locked Cargo arguments, and the standing guarded-layer globs;
- `time.rs` and `actions/defs/checkcontainer.rs` are present in the checked-in configured perimeter;
- the retained `--list-files` output contains 62 selected files;
- the retained `--list` output contains 2,901 generated mutant rows across 58 files; the four selected module files `agent/mod.rs`, `epistemics/mod.rs`, `events/mod.rs`, and `replay/mod.rs` generate zero rows at this baseline;
- `.cargo/mutants-baseline-misses.txt` is empty; and
- all eight named minimum first-proof carriers are present in the selected set.[^repo-mutants-config][^repo-mutants-baseline][^repo-mutants-files][^repo-mutants-list]

Authoring-baseline generated-mutant counts for the named carriers are:

| Carrier | Mutants in retained 0044 list |
|---|---:|
| `crates/tracewake-content/src/validate.rs` | 246 |
| `crates/tracewake-core/src/events/apply.rs` | 206 |
| `crates/tracewake-core/src/epistemics/projection.rs` | 143 |
| `crates/tracewake-core/src/replay/rebuild.rs` | 58 |
| `crates/tracewake-core/src/agent/perception.rs` | 34 |
| `crates/tracewake-core/src/epistemics/contradiction.rs` | 18 |
| `crates/tracewake-core/src/actions/defs/checkcontainer.rs` | 14 |
| `crates/tracewake-core/src/time.rs` | 9 |

These counts are historical authoring-baseline orientation. The final implementation commit must regenerate and explain its own census. A changed count is neither automatically good nor automatically bad; it must follow legitimate final-tree source change and retain the semantic standing perimeter.

### 3.4 CI durability gaps visible at the authoring baseline

The checked-in CI workflow already has useful controls: a scheduled/manual full configured lane, pinned cargo-mutants `27.1.0`, special handling for exit codes `0` and `2`, a baseline-miss ratchet, `if: always()` upload of `mutants.out`, and concurrency that does not cancel scheduled or manually dispatched runs.[^repo-ci]

However, the static authoring-baseline workflow and guard expose four completion/durability gaps:

1. **The scheduled full lane is one unsharded invocation.** It has no matrix fan-out, denominator-union verifier, or merged outcome manifest.
2. **The workflow does not encode an explicit mutation job budget.** The repository therefore does not make the intended completion window, artifact-upload grace, or wall-timeout classification part of its checked-in evidence contract.
3. **The in-diff trigger omits the two 0044 perimeter additions.** Its guarded-file regular expression includes neither `crates/tracewake-core/src/time.rs` nor `crates/tracewake-core/src/actions/defs/checkcontainer.rs`.
4. **The workflow guard mirrors the same stale omission.** Both `STANDING_MUTATION_PERIMETER` and `STANDING_MUTATION_TRIGGER_FRAGMENTS` in `ci_workflow_guards.rs` omit those paths, so the guard is internally consistent with stale data rather than derived from the configured perimeter.[^repo-ci-guards]

The in-diff lane remains useful as a fast change detector, but it is inherently a changed-code subset and is not the certifying denominator. The scheduled/manual complete lane must become durable enough to classify the full configured population and retain evidence from every shard/attempt.[^cargo-in-diff]

### 3.5 Responsible failing layer

The established 0044 blocker maps first to:

```text
mutation infrastructure / configured-perimeter completion
```

This is a report-level specialization of execution document `03`'s `tests/fixtures` and `documentation status` failure surfaces plus execution document `10`'s mutation-evidence/tool-completion rule. It is not a new canonical doctrine layer or gate code.

If the complete campaign exposes a semantic survivor, the responsible layer becomes the earliest actual behavior layer involved, chosen from the live list:

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
- tests/fixtures; or
- documentation status.[^repo-ladder]

### 3.6 What is not yet known

The following must remain explicitly unknown until final execution:

- which exact identities among the 517 previously unclassified rows are caught, unviable, missed, or mutant-level timeouts;
- whether final-tree changes increase or decrease the generated count;
- whether the optimal complete-run shape is unsharded, sharded, more parallel, longer-budget, or a measured combination;
- whether a newly exposed survivor requires production correction, a stronger behavior witness, fixture correction, or test-oracle repair; and
- the final implementation/evidence SHA and aggregate verdict.

No ticket, register, or acceptance row may pre-populate a fabricated survivor identity or a passing outcome.

---
## 4. Remediation approach

### 4.1 Required end state

Execution is complete only when all of the following are true:

1. The package records the authoring baseline `fd5ae94ff3225d2f989262b95ed8272945861516` and one exact final implementation/evidence commit.
2. Every implementation delta between those commits is enumerated, the final worktree is clean, and all certifying commands use the same source, tests, fixtures, schemas, Cargo.lock, toolchain, mutation config, and workflow state.
3. The checked-in `.cargo/mutants.toml` remains the standing denominator contract without silent shrink, replacement config, hidden CLI exclusion, `#[mutants::skip]` laundering, or a smaller accepted subset.
4. Final configured `--list-files` and `--list` outputs are captured, fingerprinted, and reviewed; every delta from 62 selected files / 2,901 generated rows is explained by legitimate final-tree source change.
5. `time.rs` and `actions/defs/checkcontainer.rs` remain in the effective standing perimeter unless a legitimate semantic source deletion makes a path inapplicable, in which case the delta and replacement carrier are explicitly justified.
6. The scheduled/manual CI lane runs the complete checked-in configured denominator, either in one completing invocation or as a provably complete deterministic shard union.
7. The in-diff trigger and workflow guard cover the complete standing perimeter, including `time.rs` and `checkcontainer.rs`, while remaining explicitly non-certifying.
8. Mutation outputs are retained on normal exit, miss, mutant-level timeout, build/baseline failure, cargo-mutants internal failure, wrapper wall timeout, forced kill, and CI cancellation where the platform permits finalization.
9. The implementation records a measured wall-clock diagnosis and selects enough sharding, conservative local `--jobs` parallelism, runner capacity, supervised budget, or combination to complete the full denominator within the declared compute limits.
10. Every shard/attempt reports launcher/supervisor result separately from cargo-mutants exit status and separately from per-mutant outcomes.
11. The unmutated baseline and all mandatory clean-preflight/named first-proof suites pass at the exact final commit before mutation outcomes are interpreted.
12. The final certifying denominator completes; an incomplete, canceled, wall-timed-out, internally failed, or missing shard cannot contribute a passing mutation row.
13. A canonical unsharded list and all shard lists form an exact set relation: shard sets are pairwise disjoint, their union equals the canonical configured set, and there are no omitted or duplicate identities.
14. Every canonical generated identity has exactly one complete final tool outcome: `caught`, `missed`, `timeout`, or `unviable`; tool/baseline/infrastructure failures are recorded outside that outcome partition and block completion.
15. The aggregate count equation reconciles to the canonical denominator and to the structured output files.
16. Every historical 0044 relevant identity is mapped through final-tree refactors by path, enclosing symbol, mutation operator, and normalized mutant text; line and column are locator data, not sole identity.
17. Every newly exposed final `missed` or mutant-level `timeout` row is appended to the triage register and assigned a responsible semantic layer.
18. Every actionable survivor is killed with behavior/provenance coverage and is `caught` in the final complete rerun.
19. No new survivor is parked in `.cargo/mutants-baseline-misses.txt`; the final baseline-miss file remains empty for this line unless upstream doctrine is separately amended before execution.
20. No final `missed`, mutant-level `timeout`, blocked identity, absent shard, duplicate identity, untriaged row, launcher/tool failure, or unexplained denominator delta remains.
21. `FIRST-PROOF-01` through `FIRST-PROOF-17` are re-proven live at the exact final commit.
22. All nine first-proof gates and all nine scenario families have complete certifying positive, adversarial/negative, replay/provenance, diagnostic, and command evidence as applicable.
23. The temporal bundle closes across execution sources `04`, `06`, `07`, `09`, and `10` at the same commit and through one traceable source-to-behavior-to-view-to-fixture-to-diagnostic/replay chain.
24. The four predecessor passes are consumed as scoped certified components, not rerun as isolated audits and not treated as latest-main evidence.
25. The replacement artifact fully instantiates `docs/4-specs/0003`, includes the complete-run proof, explicitly supersedes the 0044 artifact, and renders `FIRST-PROOF-CERT passed` only if every aggregate condition passes.
26. `EMERGE-OBS` is included as observer-only, retrospective, event-ancestry-backed evidence and contributes to no threshold, score, target, seed selection, or pass/fail calculation.
27. The ledger is updated only after accepted closeout; no downstream gate begins inside this package.

If any item remains incomplete, the replacement artifact must remain `FIRST-PROOF-CERT scoped remediation`, name the first failing layer, retain the evidence obtained, and route a separately numbered follow-up. It may not invent a partial-pass status.

### 4.2 Freeze the final implementation identity before certification

Development may use multiple commits, focused filters, single-mutant runs, and exploratory timing trials. Certification may not.

Before the final command ledger begins, the implementing session must:

1. create the intended final implementation/evidence commit;
2. record `git rev-parse HEAD` and a clean `git status --porcelain=v1` result;
3. record Rust, Cargo, cargo-mutants, OS/runner image, CPU, memory, disk, and relevant environment versions;
4. fingerprint `Cargo.lock`, `rust-toolchain.toml`, workspace manifests, `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, `.github/workflows/ci.yml`, supervisor/merger tooling, fixture registry, and named test sources;
5. capture the canonical configured `--list-files` and `--list` populations before any shard execution;
6. prohibit source, test, fixture, config, workflow, or certifying-evidence-content edits until the full command set finishes; and
7. create a new final commit and rerun the complete certifying set if any such edit is required.

Evidence generated at mixed commits is development/historical evidence and cannot be aggregated into a passing replacement row.

### 4.3 Maintain and re-confirm the standing configured denominator

The checked-in configuration is the denominator of record. The final certifying population must load `.cargo/mutants.toml` and must not use any of the following to define the accepted denominator:

- `--no-config`;
- `-f` / `--file` subsets;
- `--exclude` or function/mutant filters;
- `--in-diff`;
- `--iterate`;
- a custom replacement config;
- a baseline file that suppresses new misses;
- `#[mutants::skip]` or equivalent annotations introduced for convenience;
- an undocumented environment override affecting file selection, mutation generation, tests, timeout, or output; or
- source deletion/refactoring whose primary purpose is to evade generated mutants instead of improve the production model.

Capture at the final commit:

```bash
mkdir -p reports/0045_first_proof_cert_command_transcripts

cargo mutants --workspace --no-shuffle --list-files \
  > reports/0045_first_proof_cert_mutation_list_files.txt

cargo mutants --workspace --no-shuffle --list \
  > reports/0045_first_proof_cert_mutation_list.txt
```

Retain structured list output if supported by the pinned tool version. The human-readable files remain mandatory.

The perimeter review must prove:

- exact `.cargo/mutants.toml` fingerprint;
- exact cargo-mutants version;
- `test_workspace = true` and locked Cargo arguments are effective without duplicate/conflicting flags;
- every authoring-baseline standing file that still exists and still belongs to the guarded layer remains selected;
- `time.rs`, `checkcontainer.rs`, `events/apply.rs`, `replay/rebuild.rs`, `epistemics/contradiction.rs`, `epistemics/projection.rs`, `agent/perception.rs`, and content `validate.rs` are present;
- every final `--list-files` row is in the final tree and every required configured glob expands as intended;
- selected files that generate zero mutants are identified instead of mistaken for missing execution;
- the final generated list is internally deduplicated;
- CI uses the same checked-in config; and
- every count/path delta from Appendix B is explained in the acceptance artifact.

Cargo-mutants' list/filter behavior makes these captures the effective-population preview; a mutation percentage or prior list is not a substitute for the final denominator.[^cargo-filtering][^cargo-shards]

### 4.4 Completion diagnosis and wall-clock scaling plan

Campaign completion is a first-class remediation deliverable. Create:

```text
reports/0045_first_proof_cert_mutation_lane_completion_diagnostic.md
```

The diagnostic must contain:

- the complete 0044 symptom record and exact retained evidence paths;
- the 7,200-second wrapper budget, 30-second grace, exit `124`, and process state at interruption;
- baseline build/test timing and total classified counts;
- elapsed time and classified-mutant progress sampled at fixed intervals from retained logs where available;
- per-file/per-symbol outcome and runtime distribution derivable from `outcomes.json` or logs;
- CPU, memory, disk, cache, and runner constraints for each trial;
- whether the bottleneck is serialized mutant count, cold builds, linker contention, test duration, insufficient local `--jobs`, oversubscription, cache invalidation, output I/O, runner limits, or wrapper budget;
- the alternatives evaluated: longer supervised wall, conservative `--jobs`, deterministic sharding, larger/self-hosted runner, or combination;
- the selected final topology and why it has margin rather than merely matching the historical estimate;
- a completion-risk calculation including slowest shard, artifact-upload grace, and retry cost;
- regression controls for normal child exit, cargo-mutants miss exit, mutant-level-timeout exit, baseline failure, internal/tool failure, wall timeout, forced kill, and cancellation; and
- the exact acceptance test for the supervisor and union merger.

The historical average may inform sizing but is not a guarantee. Workloads are skewed by file, symbol, build invalidation, and mutation operator; the final topology must be measured against representative or full dry-run evidence.

Cargo-mutants can parallelize locally with `--jobs`, but its documentation warns that Rust builds/tests already consume substantial resources and recommends conservative tuning because excessive local concurrency can cause thrash, memory exhaustion, or spurious timeouts.[^cargo-parallelism] Therefore:

- begin local-parallelism trials conservatively;
- record resource saturation and timeout variance;
- do not accept a faster run that creates load-induced mutant-level timeouts or flaky baseline behavior;
- keep the same tested semantics and denominator; and
- prefer distributed shards when a single host cannot complete with reliable margin.

### 4.5 Required outcome vocabulary: keep transport, denominator, tool, and certification axes separate

The package must not collapse unlike outcomes into one `failed` or `passed` field.

| Axis | Required values | Meaning |
|---|---|---|
| Launcher/supervisor result | normal child exit; wrapper wall timeout; forced kill; cancellation; spawn/IO failure; artifact-copy failure | Did the process supervision lane terminate and retain evidence honestly? |
| Denominator completion | complete; incomplete | Did the run/shard classify every identity assigned to it? |
| cargo-mutants process result | exit `0`; exit `1`; exit `2`; exit `3`; exit `4`; exit `5`; exit `6`; exit `70`; signal/unknown | What did cargo-mutants itself report? |
| Per-mutant tool outcome | `caught`; `missed`; `timeout`; `unviable` | What happened to one generated identity? |
| Certification disposition | killed with behavior witness; blocked; untriaged; source-changed/reconciled; mechanically unviable and reviewed | What does the remediation package do with that identity? |
| Evidence status | `pass`; `fail`; `pending`; `sampled`; `observer-only`; `historical` | May the evidence item contribute to certification? |

A wrapper wall timeout is not a cargo-mutants mutant-level timeout. An interrupted run with `missed 0` is not a complete clean run. A cargo-mutants exit `2` means at least one miss, while exit `3` means one or more test timeouts; exit `4` means the unmutated baseline failed or hung; exit `70` is an internal error.[^cargo-exit-codes]

### 4.6 Supervisor and artifact-retention contract

The final supervisor must:

- launch without a PTY unless a documented platform constraint requires one;
- supervise the complete process group, not just an immediate shell;
- record command, cwd, environment-affecting variables, start/end timestamps, process IDs, signals, and exit statuses;
- distinguish its own wall timeout from cargo-mutants exit `3`;
- send graceful termination and then a recorded forced kill after bounded grace;
- wait/reap every child it owns;
- copy or preserve the output directory and raw stdout/stderr in `finally`/`always` behavior;
- retain output even when the child exits nonzero;
- never treat disappearance of the immediate shell as proof that descendants completed;
- never overwrite a previous attempt's output; and
- emit a machine-readable `status.env` or JSON record with separate supervisor and child fields.

The package must retain, when produced:

- `mutants.json` or equivalent complete discovered-population record;
- `outcomes.json`;
- `caught.txt`;
- `missed.txt`;
- `timeout.txt`;
- `unviable.txt`;
- `lock.json`;
- per-mutant logs and debug log;
- command, metadata, status, stdout, and stderr; and
- checksums for every retained file.

Cargo-mutants writes its output population and outcome files incrementally, making partial `mutants.out` useful for diagnosis, but partial state remains non-certifying until denominator completion is independently proven.[^cargo-mutants-out]

The wrapper budget must leave sufficient time for graceful shutdown, output synchronization, checksumming, and CI artifact upload before the host-level job deadline. A host cancellation that prevents retention is a tool/infrastructure failure, not a pass.

### 4.7 Deterministic shard topology and complete-union proof

The preferred completion design is a scheduled/manual matrix of reproducible cargo-mutants shards, unless measured evidence proves a single reliable invocation completes with adequate margin. Cargo-mutants shards independently discover the same population and select `k/n`; all shards must use the same arguments and denominator `n`, and CI is responsible for launching and collecting them.[^cargo-shards]

#### 4.7.1 Canonical population

Before shard runs, create one canonical unsharded identity set from the checked-in config:

```text
reports/0045_first_proof_cert_mutation_list.txt
```

Normalize each row only for set comparison, not to erase semantic identity. The normalized key must retain at least:

```text
path | enclosing symbol/function | mutation operator/replacement | normalized mutant text
```

Line and column remain recorded locator fields. Where structured JSON carries a stable internal name/diff, retain and fingerprint it.

#### 4.7.2 Shard commands

For a chosen denominator `N`, every shard `K` in `0..N-1` must use the same:

- final commit;
- clean tree;
- Cargo.lock/toolchain/cargo-mutants version;
- checked-in `.cargo/mutants.toml`;
- workspace/test posture;
- `--no-shuffle` posture;
- timeout policy;
- local `--jobs` value;
- baseline policy;
- environment-affecting variables; and
- sharding mode, if explicitly selected.

Only shard index and output path may differ.

A representative command shape is:

```bash
cargo mutants --workspace --no-shuffle \
  --shard "${K}/${N}" \
  --jobs "${MUTANTS_JOBS}" \
  -o "reports/0045_first_proof_cert_mutation_shard_${K}_of_${N}.out"
```

This is a command contract, not a preselected `N` or `MUTANTS_JOBS` value. The implementing session must choose and record them from measured evidence.

If `--baseline=skip` is used to avoid rerunning the unmutated suite on every shard, it is allowed only when:

1. one dedicated baseline job ran at the same exact commit/config/toolchain and passed;
2. every shard depends on that baseline job;
3. an explicit mutant test timeout is supplied because baseline-derived timeout calculation is unavailable;
4. the dedicated baseline transcript and fingerprint are package members; and
5. no source/config edit occurs between baseline and shards.

Cargo-mutants warns that results are meaningless if baseline tests are not actually passing and requires an explicit timeout posture when baseline is skipped.[^cargo-baseline]

#### 4.7.3 CI matrix posture

The CI matrix must:

- enumerate all shard indices exactly once;
- set `strategy.fail-fast: false` so one miss/failure does not cancel evidence collection from other shards;
- avoid `continue-on-error: true` on certifying shards;
- give every shard a unique output/artifact name;
- upload each shard's complete output under `if: always()`;
- preserve shard command/status metadata even on failure;
- use an explicit `timeout-minutes` larger than the measured shard wall plus retention grace but within runner limits;
- avoid workflow-level `cancel-in-progress` for scheduled/manual certifying runs;
- run a final reconciliation job after all shards with `if: always()` or equivalent; and
- make the aggregate workflow fail when any shard is missing, canceled, incomplete, tool-failed, contains a miss/timeout, or fails union reconciliation.

GitHub Actions matrices create one job per matrix combination, `fail-fast` defaults to true, and artifact upload under `always()` is the documented pattern for retaining failed test output; these mechanics shape the implementation but do not determine repository state.[^github-matrix][^github-workflow-syntax][^github-artifacts]

#### 4.7.4 Set-union reconciliation

The final merger must independently compute:

```text
C = canonical configured identity set
S_k = identity set assigned to shard k
O_k = identity set with final outcomes in shard k
```

and prove:

```text
for every k: S_k == O_k
for every i != j: S_i intersection S_j == empty
union(S_0 ... S_(N-1)) == C
union(O_0 ... O_(N-1)) == C
sum(|S_k|) == |C|
sum(caught_k + missed_k + timeout_k + unviable_k) == |C|
```

The merger must fail loudly on:

- missing shard artifact;
- duplicate shard index;
- mismatched shard denominator;
- mismatched config/commit/toolchain fingerprints;
- identity in a shard outcome but not canonical list;
- canonical identity absent from all shards;
- identity present in multiple shards;
- outcome duplicate or missing within a shard;
- malformed/truncated structured output;
- shard supervisor result other than normal child exit;
- cargo-mutants internal/baseline/usage/diff error; or
- unexplained mismatch between text and JSON outcome sets.

A summary count without member-set equality is insufficient. The merged proof must retain the canonical and per-shard identity manifests so a reviewer can reproduce the set equations.

### 4.8 CI convergence and baseline-miss discipline

The implementing session must make the checked-in workflow and guards converge on one standing perimeter.

Required workflow/guard changes include:

1. add `time.rs` and `actions/defs/checkcontainer.rs` to the in-diff trigger;
2. add them to `STANDING_MUTATION_PERIMETER` and the trigger-fragment expectations;
3. preferably derive trigger coverage mechanically from one checked-in perimeter representation or add a test that translates every configured path/glob into an accepted trigger fragment with explicit exceptions;
4. add synthetic negatives proving removal of either new path fails the workflow guard;
5. preserve the scheduled/manual complete lane and its `if: always()` artifact upload;
6. implement the selected shard/supervisor/merge topology;
7. label the in-diff lane as development/change-detection evidence, not the final denominator;
8. prevent a new full run from being canceled by a later scheduled/manual run unless the canceled run is explicitly non-certifying and retained as such; and
9. record exact runner/timeout/storage assumptions.

`.cargo/mutants-baseline-misses.txt` is empty at the authoring baseline. It must not become a parking lot for survivors exposed by 0045. Any final miss is a remediation obligation. The normal baseline-ratchet machinery may remain for CI compatibility, but a passing 0045 artifact requires zero final standing misses and zero final standing mutant-level timeouts.

### 4.9 Clean baseline and mandatory named preflight

Before interpreting mutation outcomes, run at the exact final implementation/evidence commit:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
```

Then run and retain separate transcripts for at least the following named suites:

```bash
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test acceptance_artifact_wording
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test emergence_ledger
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test ci_workflow_guards
cargo test --locked -p tracewake-core --test doc_invariant_references

cargo test --locked -p tracewake-content --test fixtures_load
cargo test --locked -p tracewake-content --test forbidden_content
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-content --test schema_conformance

cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test command_loop_session
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-tui --test readme_sample_session
cargo test --locked -p tracewake-tui --test transcript_snapshot
cargo test --locked -p tracewake-tui --test tui_acceptance
cargo test --locked -p tracewake-tui --test tui_seam_conformance
```

If a final test target has been legitimately renamed, moved, split, or consolidated, the package must map the old manifest path/target to the final target and prove no coverage family was silently dropped. A workspace pass alone is not a substitute for named target discovery and transcript retention.

The clean baseline must precede shard interpretation. A cargo-mutants baseline failure is not evidence that mutants were caught; it makes mutation outcomes meaningless.[^cargo-baseline]

### 4.10 Development loops versus certifying execution

The following are useful **development evidence only**:

- the historical 0044 focused seven-file run;
- `--no-config` runs;
- `-f`/`--file` subsets;
- `--iterate` loops;
- `--in-diff` runs;
- single-mutant `-F` reproductions;
- one shard run without the remaining shard set;
- timing pilots; and
- source-local helper tests.

They may guide remediation, identify witnesses, and shorten iteration. They may not populate the final complete mutation pass row except as historical/sampled context.

The certifying run is the complete configured denominator at the final commit, either one completing invocation or the accepted complete shard union. After any production/test/fixture/config correction prompted by a survivor, the implementing session must rerun:

1. clean preflight;
2. named affected behavior suites and the complete mandatory suite set;
3. final `--list-files`/`--list` census; and
4. the complete configured mutation campaign.

A targeted rerun proving one identity caught does not replace the complete final rerun.

### 4.11 Survivor identity reconciliation

The canonical cargo-mutants display form remains:

```text
path:line:column:operator-or-replacement-description
```

Because line/column drift under legitimate edits, every triage row must also capture:

- final path;
- enclosing type/module/symbol;
- mutation operator and replacement;
- normalized mutant text or structured mutation name/diff;
- relevant proposition/event/action/fixture family;
- historical 0044 line identity where applicable; and
- mapping disposition: unchanged, line-shifted, symbol-moved, source-transformed, no-longer-generated, or newly generated.

A no-longer-generated identity is not automatically a kill. The register must explain the legitimate semantic source change, identify the replacement behavior carrier, and prove that the property remains tested and in the configured perimeter. Refactoring solely to remove mutation sites is forbidden.

### 4.12 Required triage-register schema

Create and maintain:

```text
reports/0045_first_proof_cert_mutation_triage_register.md
```

The register is append-only during execution and must contain one row for every final `missed` or mutant-level `timeout`, plus reconciliation rows for historically relevant 0044 identities/families and any source-changed identity needed to explain the census.

Required fields:

| Field | Required content |
|---|---|
| Historical identity | 0044 path:line:column:operator or `not previously classified/new` |
| Final identity | Final path:line:column:operator plus normalized structured key |
| Enclosing symbol | Stable function/type/module used across line movement |
| Tool outcome | `caught`, `missed`, `timeout`, `unviable`, or tool blockage, kept separate from certification disposition |
| Canonical denominator membership | Canonical list row and shard assignment |
| FIRST-PROOF cross-reference | `FIRST-PROOF-01`…`17`, existing gate(s), scenario family/families, and temporal source where applicable |
| Responsible layer | Earliest live execution-document layer |
| Certified reachability | Exact production entry and why the path is live |
| Test family | Unit/integration/golden/property/metamorphic/replay/view/content/static as applicable |
| Behavior witness | Public or certified first-proof consequence observed |
| Mutant-activation witness | Evidence the witness fails when the mutant is active and passes unmutated |
| Replay/provenance ancestry | Event IDs/log range, source refs, holder-known context, projection/replay artifacts, first divergence |
| Negative/contamination control | Hidden-truth/debug/prose/direct-state/alternate-path control |
| Remediation delta | Production/test/fixture/diagnostic change and rationale |
| Final rerun evidence | Complete configured campaign identity/outcome and named suite transcripts |
| Certification disposition | killed; pending; blocked; mechanically unviable; source-changed with proof |
| Evidence references | Stable paths and fingerprints |
| Review signoff | Implementer and independent reviewer/date |

Grouped rows are allowed only when the register enumerates every member identity and proves one behavior witness genuinely kills each member. A family label without member reconciliation is insufficient.

### 4.13 Kill-with-behavior/provenance coverage

For every survivor exposed by the complete run, the default and required resolution is behavior/provenance coverage through a public or already-certified first-proof consequence.

A sufficient witness must:

1. pass against the unmutated final implementation;
2. fail while the target mutant is active;
3. exercise the production path rather than a test-only bypass;
4. observe a semantic first-proof consequence rather than the mutated expression itself;
5. name the responsible layer;
6. retain event/replay/provenance ancestry for event-derived or projected behavior;
7. include a live negative/contamination control where hidden truth, debug state, fixture prose, direct state mutation, or alternate dispatch could fake the result; and
8. remain valid under holder/item/location/seed or replay partition variation where the property supports it.

Acceptable consequence families include:

- source-backed expectation plus modeled absence produces the correct expectation contradiction, while a missing source or wrong holder fails closed;
- a temporal interval mutation changes single-charge duration/accounting or a holder-known temporal premise, and the witness observes the resulting event/projection behavior rather than a raw boundary helper;
- an event-application mutation causes the wrong authoritative state, typed diagnostic, event rejection, or replay divergence at a localized event;
- a replay mutation changes a rebuilt physical, epistemic, agent, diagnostic, or embodied-support projection and produces a first-divergence record;
- a perception/projection mutation leaks unobserved truth, loses provenance/freshness, or changes the actor-known context under a paired hidden-truth intervention;
- a container-check mutation changes legal/rejected action behavior, observation ancestry, or missing-property discovery;
- a content-validation mutation accepts a forbidden authoring field or rejects a valid canonical fixture for the wrong reason; and
- a TUI/view mutation changes embodied output without allowed holder-known support or allows debug truth to affect ordinary action availability.

Insufficient kill examples include:

- asserting the mutated helper returns a literal;
- duplicating the production expression in the test oracle;
- reading a private field that no certified consumer uses;
- snapshotting fixture labels without exercising the path;
- using ground truth as both expected and actual value;
- directly inserting a belief, observation, contradiction, event-applied state, or projection record;
- using debug output as evidence of actor knowledge;
- checking only that a report row or file exists;
- asserting mutation percentage or empty `missed.txt` without complete denominator proof;
- allowing a test-only dependency or bypass; or
- killing a mutant by narrowing/removing the configured perimeter.

Property-based mutation research motivates tying a meaningful kill to violation of a specified property rather than mere incidental test failure; here the binding properties remain Tracewake's live doctrine and first-proof seams.[^property-mutation]

### 4.14 Required property and metamorphic families

Example tests remain mandatory, but cross-cutting integration survivors should preferentially receive relational coverage that does not duplicate the implementation expression. At minimum, the final package must include or cite live tests for these relations:

| Relation | Required invariant |
|---|---|
| Unobserved hidden-item relocation | Moving the true item among equally unobserved lawful locations does not alter the expecting actor's pre-observation proposals, actor-known context, embodied view, or contradiction state. |
| Holder permutation | Reassigning actor IDs/holders while preserving each holder's source graph permutes outcomes without cross-holder leakage. |
| Presence/absence dual | An observed present item does not produce the absence contradiction; an allowed local observation of absence with a source-backed expectation does. |
| Source deletion | Removing required provenance causes fail-closed cognition/content validation rather than truth fallback. |
| Debug off/on | Debug attachment changes only non-diegetic output and never proposals, events, actor-known context, or embodied affordances. |
| Possession off/on | Controller binding changes input routing only; world rules, memory, intention, needs, routine, expectation, and contradiction remain continuous. |
| Replay partition | Replaying the same ordered log from empty projections, or through accepted snapshot/partition boundaries, produces identical semantic projections and diagnostics. |
| Event perturbation | A controlled event deletion/reorder/payload change produces the expected first localized divergence rather than a generic mismatch. |
| Temporal translation | Shifting a scenario and all modeled temporal premises by the same legal offset preserves relative behavior and single-charge accounting. |
| Container/location isomorphism | Renaming equivalent places/containers/items preserves legal behavior and causal structure without revealing true targets. |
| Human-controller removal | Removing possession does not stop the no-human ordinary day or alter autonomous actor semantics. |
| Fixture-order stability | Registry or map iteration order does not change deterministic event/projection/replay results. |

Metamorphic testing is appropriate where a direct oracle is difficult but a relation between executions is binding; generated evidence remains sampled unless the finite domain is exhaustively enumerated, and every failing seed must be retained and replayable.[^metamorphic-review]

### 4.15 Unviable, equivalent, timeout, and source-simplification posture

`unviable` is a normal cargo-mutants tool outcome when the generated replacement does not build. It may be non-blocking only when:

- the complete run classified it normally;
- structured/text outcomes agree;
- it is not a disguised baseline/tool failure;
- its member identity is retained in the denominator accounting; and
- count reconciliation is exact.

A final mutant-level `timeout` is blocking. The implementing session must determine whether it exposes a real liveness fault, an insufficient test timeout, or resource contention. Raising timeout is acceptable only with measured baseline/mutant evidence and cannot hide an infinite loop or overloaded runner. Cargo-mutants distinguishes per-mutant build/test timeouts from external wrapper walls; the report must preserve that distinction.[^cargo-timeouts]

Because the settled 0045 contract requires killing every survivor the complete run exposes, a final `missed` row cannot be waived as an accepted baseline or convenience exception. A suspected equivalent mutant may guide one of these outcomes:

1. a behavior witness demonstrates it is not equivalent and kills it;
2. legitimate production simplification removes the redundant semantic branch, after which the final census delta and replacement property carrier are reviewed; or
3. the package remains `FIRST-PROOF-CERT scoped remediation` because the identity remains missed.

Do not add `#[mutants::skip]`, exclude the file/function, or classify an unproven survivor as non-critical merely to obtain a green line.

### 4.16 Production integrity and staged abstractions

No remediation may:

- introduce a parallel test-only implementation;
- invert crate dependencies;
- let truth choose actor goals, search targets, hidden food/property, suspects, witnesses, clues, temporal interpretations, or embodied affordances;
- let validation author actor knowledge;
- let possession/debug reset or privilege cognition;
- let projections or diagnostics write truth;
- let replay repair divergence from live truth;
- let the scheduler dispatch primitive ordinary actions or author state deltas;
- turn `EMERGE-OBS` into a target; or
- pull Phase-4/second-proof abstractions into first-proof acceptance.

Any bounded staged abstraction must be declared under the template. No staged abstraction may defer campaign completion, a surviving mutant, `FIRST-PROOF-01`…`17`, a first-proof gate, a scenario family, the temporal bundle, replay determinism, or evidence honesty while still producing `FIRST-PROOF-CERT passed`.

---
## 5. Completion-first remediation deliverables

### 5.1 Mutation-lane completion diagnostic

The implementation must produce the diagnostic named in section 4.4. It is a certifying package member only when its statements are backed by retained commands, timing, resource data, supervisor statuses, and output fingerprints.

At minimum it must answer:

1. Why did the historical single invocation exceed 7,200 seconds?
2. What fraction of elapsed time was baseline, build, test, queueing, output, and teardown where measurable?
3. Was progress steady, skewed, or stalled before the wall?
4. Which topology is selected and what measured completion margin does it provide?
5. How does the supervisor retain output after every non-normal termination?
6. How are host wall timeout, cargo-mutants exit, and per-mutant timeout kept distinct?
7. How will the merger prove complete set coverage rather than trust count totals?
8. How does CI ensure one shard's miss/failure does not cancel the remaining evidence collection?
9. What exact condition makes the final mutation row `pass`?
10. What exact condition keeps it `scoped remediation`?

A document that merely says “increase timeout,” “use more runners,” or “the run is probably clean” is insufficient.

### 5.2 Final census package

Required files:

```text
reports/0045_first_proof_cert_mutation_list_files.txt
reports/0045_first_proof_cert_mutation_list.txt
reports/0045_first_proof_cert_mutation_census.md
```

The census report must contain:

- final exact SHA and clean-tree record;
- config/toolchain/Cargo.lock fingerprints;
- selected-file count and generated-mutant count;
- selected files with zero generated mutants;
- per-file generated counts;
- authoring-baseline-to-final path/count delta;
- explicit presence of the eight named carriers;
- canonical identity-set fingerprint;
- chosen shard denominator and assignment mode;
- per-shard expected identity count/list fingerprint; and
- reviewer signoff that no forbidden filter or denominator shrink is present.

### 5.3 Shard/attempt package

Use stable, non-overwriting paths, for example:

```text
reports/0045_first_proof_cert_command_transcripts/mutation_baseline/
reports/0045_first_proof_cert_command_transcripts/mutation_shard_00_of_08/
reports/0045_first_proof_cert_command_transcripts/mutation_shard_01_of_08/
...
reports/0045_first_proof_cert_mutation_shard_00_of_08.out/mutants.out/
...
```

The exact shard count is implementation-selected; path names must encode it. Each shard package must include:

- `command.txt`;
- `metadata.txt` or JSON;
- `status.env` or JSON;
- `stdout.txt`;
- `stderr.txt`;
- output-tree checksum manifest;
- assigned identity manifest;
- final outcome manifest;
- supervisor result;
- cargo-mutants exit result;
- start/end/elapsed time;
- baseline policy and timeout values;
- resource/runner identity; and
- artifact upload/download identity where CI is used.

Failed exploratory attempts must be retained under clearly non-certifying paths rather than overwritten or silently deleted.

### 5.4 Completion-union proof

Create:

```text
reports/0045_first_proof_cert_mutation_completion_manifest.md
reports/0045_first_proof_cert_mutation_completion_manifest.json
```

The human-readable manifest explains the proof; the machine-readable manifest contains member sets and fingerprints.

Required fields:

- final implementation/evidence SHA;
- canonical denominator fingerprint and count;
- shard denominator `N` and exact indices received;
- per-shard command/config/toolchain fingerprints;
- per-shard assigned/outcome set fingerprints and counts;
- pairwise-intersection result;
- union-equality result;
- missing/duplicate identity lists, required empty;
- supervisor and cargo-mutants result per shard;
- aggregate `caught`/`missed`/`timeout`/`unviable` counts;
- aggregate count equation;
- malformed/truncated-output checks;
- output-tree and artifact checksums;
- merger command/version;
- reviewer rerun or independent verification; and
- certification disposition.

The merger must emit a nonzero result for any incomplete or dishonest state. Its own unit/integration tests must include synthetic missing-shard, duplicate-identity, mismatched-commit, mismatched-config, truncated-JSON, and overlapping-shard negatives.

### 5.5 Final missed and timeout manifests

Create:

```text
reports/0045_first_proof_cert_mutation_final_missed.txt
reports/0045_first_proof_cert_mutation_final_timeout.txt
```

For a passing replacement artifact, both files must exist, be fingerprinted, and contain zero identities. File existence alone is not proof; the completion manifest must show they derive from the complete canonical denominator.

During remediation, any nonempty file is an input to the triage register. A targeted kill may produce an intermediate empty file, but only the complete final rerun can populate the certifying empty manifests.

### 5.6 Unknown-survivor remediation loop

For each newly exposed `missed` or mutant-level `timeout`:

1. append the raw final identity and shard evidence to the triage register;
2. reproduce it with the exact final-pre-remediation source/config/toolchain and a single-mutant development command;
3. establish that the failing test is valid under live doctrine;
4. determine whether the defect is in production behavior, fixture/content, test oracle, evidence instrumentation, or resource/timeout policy;
5. name the earliest responsible layer;
6. design a behavior/provenance witness and contamination control;
7. implement the smallest doctrine-correct production/test/fixture correction;
8. prove the unmutated implementation passes and the active mutant fails the witness;
9. run affected named suites and replay checks;
10. update identity mapping if line/symbol moved;
11. freeze a new final implementation/evidence commit; and
12. rerun the entire clean preflight, census, and complete configured campaign.

Do not stop after the targeted mutant changes from missed to caught. The complete population must close again because the remediation may change the denominator or expose another interaction.

### 5.7 Behavior-witness routing by named carrier

This table is guidance for newly exposed survivors, not a predicted survivor list.

| Carrier | Public/certified consequence to prefer | Required contamination/replay control |
|---|---|---|
| `time.rs` | passive need deltas, interval/single-charge accounting, modeled temporal premise behavior | no wall-clock/debug clock input; replay equality; temporal translation relation |
| `events/apply.rs` | accepted/rejected event effect, typed payload failure, authoritative state/projection delta, duplicate-duration rejection | append/application ancestry; controlled replay tamper and first divergence; no direct state setup as oracle |
| `replay/rebuild.rs` | rebuild equivalence and localized `Phase3AReplayFailure`/report behavior | rebuild from empty projections; controlled log perturbation; no truth-based repair |
| `epistemics/contradiction.rs` | source-backed expected absence produces correct actor-owned contradiction; present/wrong-holder/no-source cases do not | expectation + observation provenance; holder permutation; replay-stable links |
| `epistemics/projection.rs` | actor-known record classification/freshness, notebook/debug separation, stable checksum | unobserved-truth relocation; debug off/on; replay equality and source-event ancestry |
| `agent/perception.rs` | local perception emits observation events and holder-known context from projection, not live truth | remote/unobserved item movement non-interference; observation event ancestry; latest-window freshness |
| `actions/defs/checkcontainer.rs` | legal local check event and caused observation; inaccessible/nonlocal/locked cases reject appropriately | no teleport/hidden-target affordance; observation caused by accepted check event; replay |
| `content/validate.rs` | valid fixture accepted and forbidden outcome/knowledge/prose/culprit fields rejected before runtime for the intended typed reason | renamed/nested/generated forbidden equivalent; no text-label-only oracle; registry and fingerprint scope |

### 5.8 CI closeout deliverables

The final repository changes must include:

- durable scheduled/manual complete mutation topology;
- explicit job/shard budgets and retention grace;
- shard artifacts uploaded uniquely under `always()`;
- aggregate completion/ratchet job;
- corrected in-diff trigger;
- corrected/derived workflow guard;
- synthetic guard negatives;
- documentation posture updated only where live execution text must reflect the new durable lane; and
- no CI wording that calls the in-diff lane certifying or calls a partial run green.

The implementation must not rely solely on a one-off local multi-machine run while leaving the checked-in scheduled lane unable to reproduce the accepted evidence contract.

---

## 6. Failure handling and responsible-layer diagnostics

### 6.1 Failure classes

Every failure must be classified before remediation or routing:

| Failure class | Examples | Certification effect |
|---|---|---|
| Doctrine mismatch | proposed witness relies on hidden truth; denominator shrink conflicts with evidence honesty | stop; correct design or escalate upstream; no pass |
| Content/schema validation | forbidden fixture accepted, valid fixture rejected for wrong reason | named content layer remediation; no pass |
| Actor-known context construction | missing/wrong-holder/stale provenance, truth fallback | named context remediation; no pass |
| Candidate generation | unobserved target affects candidates | candidate-layer remediation; no pass |
| Planning/method selection | hidden food/property/time/suspect chosen | planning-layer remediation; no pass |
| Proposal construction | validator/debug truth inserted into proposal | proposal-layer remediation; no pass |
| Scheduler ordering | direct dispatch, wrong interval, starvation, nondeterministic ordering | scheduler remediation; no pass |
| Action validation | illegal container/access action accepted or valid action rejected | validation remediation; no pass |
| Event application | wrong state delta, payload interpretation, duplicate terminal, missing typed rejection | event-application remediation; no pass |
| Projection/replay | live/rebuild mismatch, nonlocalized divergence, truth repair | projection/replay remediation; no pass |
| View-model rendering | unknown/debug truth exposed embodied, temporal label lacks holder-known basis | view-model remediation; no pass |
| Debug quarantine | debug changes proposals/events/knowledge/ordinary output | debug remediation; no pass |
| Tests/fixtures | invalid oracle, harness fabrication, omitted named target, flaky/non-hermetic behavior | test/fixture remediation; no pass |
| Documentation status | artifact overclaims, mixed SHA, missing template field, historical evidence counted live | documentation/evidence remediation; no pass |
| Mutation infrastructure / configured-perimeter completion | missing shard, wall timeout, tool exit, truncated output, stale trigger, union mismatch | retain evidence; `scoped remediation`; no pass |

The local mutation-infrastructure wording is a practical specialization for this package. It does not alter execution document `03`'s canonical list.

### 6.2 Required mutation-infrastructure failure record

For every incomplete or non-normal shard/attempt, record:

- final or trial commit;
- exact command and arguments;
- config/Cargo.lock/toolchain fingerprints;
- runner/resources;
- shard `K/N`;
- expected identity-set fingerprint/count;
- supervisor result;
- child/cargo-mutants result;
- signal/cancellation/wall reason;
- elapsed and last-progress timestamp;
- last classified identity or build/test phase where available;
- output files present/missing and their checksums;
- whether the output can be used for diagnosis only;
- first responsible infrastructure/evidence layer;
- remediation action; and
- whether the complete certifying campaign must restart.

A retry that later passes does not erase the failed attempt; it becomes historical diagnostic evidence and the final artifact must point to the successful replacement attempt.

### 6.3 Required semantic-survivor failure record

For every `missed` or mutant-level `timeout`, record:

- raw and normalized identity;
- shard and canonical-list membership;
- enclosing symbol and mutation operator;
- affected audit point(s), gate(s), scenario family/families, and temporal source where applicable;
- expected and actual public behavior;
- production path and earliest responsible layer;
- event/proposal/context/source ancestry;
- live and replay/projection comparison;
- negative/contamination control result;
- failing test validity assessment;
- production/test/fixture remediation decision; and
- final full-rerun result.

### 6.4 Diagnostic quality standard

A diagnostic must let a future implementation session identify the first layer to inspect without guessing. “Test failed,” “mutation timed out,” “CI was slow,” “replay differed,” or “artifact missing” is insufficient.

Behavioral diagnostics must state:

- holder/actor;
- source/provenance status;
- fixture/seed/content version;
- accepted/rejected stage;
- event/log range;
- expected/actual projection or view;
- first divergence where applicable;
- responsible layer; and
- remediation category.

Temporal diagnostics must distinguish validator time from holder-known temporal premises. Mutation diagnostics must distinguish wrapper wall from mutant-level timeout. Content diagnostics must distinguish schema parse, semantic validation, fixture registry, and runtime behavior.

### 6.5 Routing rule

The package may render `FIRST-PROOF-CERT passed` only after all failures are closed and the complete final campaign/re-proof package is rerun.

If any of these remain:

- incomplete denominator;
- missing/duplicate shard;
- wall timeout/cancellation/tool failure;
- `missed` identity;
- mutant-level `timeout`;
- untriaged/source-unmapped identity;
- failing seam/gate/scenario/temporal source;
- missing replay/provenance or negative control;
- pending mandatory command;
- mixed baseline; or
- incomplete acceptance-template field,

the artifact must state:

```text
FIRST-PROOF-CERT scoped remediation
```

and route to a named separately numbered follow-up. It may not use “phase exception,” “provisional pass,” “pass except mutation,” or any new status.

---
## 7. Live re-proof of the FIRST-PROOF contract

### 7.1 Re-proof rule

Mutation completion is necessary but not sufficient. After every remediation delta is frozen at one exact final implementation/evidence commit `U`, the implementing session must re-prove the complete 0044 integrated contract live.

A predecessor artifact may establish that a subsystem's internals passed at its own historical commit. It cannot establish that the combined first-proof system remains coherent at `U`. Conversely, this re-proof must not reopen those subsystem audits in isolation. It proves their participation in one integrated missing-property/no-human/temporal/replay corpus at `U`.[^repo-0044-spec]

Every point row must include:

- positive behavior witness;
- required adversarial/negative witness;
- event/replay/projection evidence where applicable;
- actor-known/holder-known provenance evidence where applicable;
- responsible-layer diagnostic;
- exact command transcript at `U`;
- mutation identity linkage for affected carriers; and
- evidence-status/fingerprint-scope fields.

### 7.2 `FIRST-PROOF-01` through `FIRST-PROOF-17` routing matrix

| Audit point | Integrated seam to re-prove at `U` | Primary gates / scenario families | Temporal route | Minimum replacement evidence |
|---|---|---|---|---|
| `FIRST-PROOF-01` | Unified baseline, predecessor consumption, complete artifact set, crate direction, fixture/test/mutation census | all nine gates; all nine families | bundle identity | one SHA; clean tree; command ledger; fixture/test/config fingerprints; complete mutation union; mixed-SHA and omitted-target negatives |
| `FIRST-PROOF-02` | Physical custody and legal container/access/action path participates in event log, embodied play, and replay | `EVENT`, `MISSING-PROPERTY`, `REPLAY`; Physical custody baseline | supporting | legal open/check/inspect/take/place path; inaccessible/nonlocal rejection; physical live/rebuild checksums; no teleport/debug affordance leakage |
| `FIRST-PROOF-03` | Source-backed actor expectation exists before absence discovery | `ACTOR-KNOWN`, `MISSING-PROPERTY`, `FIXTURE-NEGATIVE`; Expectation contradiction | source/freshness support | typed expectation/source ancestry; wrong-holder/no-source/prose/debug negatives; replay-stable provenance |
| `FIRST-PROOF-04` | Absence learned through allowed local observation/check, not truth notification | `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `MISSING-PROPERTY`, `EVENT`; Expectation contradiction | supporting | accepted check/perception event and caused observation; remote/hidden truth non-interference; no cognition event on rejected action |
| `FIRST-PROOF-05` | Event-sourced expectation contradiction and belief update | `EVENT`, `ACTOR-KNOWN`, `MISSING-PROPERTY`, `REPLAY`; Expectation contradiction | supporting | both typed premises; stable private links; event application/projection/replay equality; present/no-source/wrong-holder negatives |
| `FIRST-PROOF-06` | No culprit, suspect, clue, theft, quest, or story-sifting truth | `TRUTH-FIREWALL`, `FIXTURE-NEGATIVE`, `MISSING-PROPERTY`; No-hidden-truth planning, Content rejection | boundary | schema/content/runtime negatives; no hidden accusation target; ordinary epistemic uncertainty only |
| `FIRST-PROOF-07` | Truth-firewall and actor-known non-interference across combined corpus | `TRUTH-FIREWALL`, `ACTOR-KNOWN`; Epistemic filtering, No-hidden-truth planning | execution `04` | paired unobserved-truth interventions; same proposals/context/view until modeled information event; validator truth never proposes |
| `FIRST-PROOF-08` | Possession parity, embodied view, and debug split during missing-property run | `POSSESSION-PARITY`, `VIEW-DEBUG-SPLIT`; Possession parity, Epistemic filtering | execution `07` support | possession bind/unbind continuity; debug off/on non-interference; holder-known embodied census; no hidden location leakage |
| `FIRST-PROOF-09` | No-human ordinary life coexists with missing property and no direct dispatch | `NO-HUMAN-ORDINARY-LIFE`, `EVENT`, `TRUTH-FIREWALL`; No-human ordinary day, Routine blocking | execution `06` | autonomous transaction/proposal/action/event path; real action/wait/stuck progress; no human branch, direct dispatch, teleport, or marker-only life |
| `FIRST-PROOF-10` | Composite replay, projection rebuild, deterministic diagnostics, and divergence localization | `EVENT`, `REPLAY`; Replay rebuild plus every family | all sources | per-scenario and capstone rebuild from empty projections; semantic checksums; controlled tamper with first divergence; no truth repair |
| `FIRST-PROOF-11` | Fixture-negative, schema, compile-fail, and semantic content rejection | `FIXTURE-NEGATIVE`, `TRUTH-FIREWALL`; Content rejection | execution `08`+`09` support | forbidden fields/API paths fail before runtime for intended typed reason; renamed/nested/generated negatives; no partial load or text-only guard |
| `FIRST-PROOF-12` | Temporal firewall: modeled holder-known temporal premises, ancestry, freshness | `TRUTH-FIREWALL`, `ACTOR-KNOWN`; Epistemic filtering, No-hidden-truth planning | execution `04` | source/acquisition/verification tick ancestry; stale/unknown states; raw scheduler/debug/validator time cannot become cognition; replay equality |
| `FIRST-PROOF-13` | Routine temporal premises and adaptation in integrated no-human run | `NO-HUMAN-ORDINARY-LIFE`, `EVENT`; No-human ordinary day, Routine blocking | execution `06` | modeled wait/window/interruption/adaptation behavior; single-charge accounting; stale premise negative; no true-time planning/direct dispatch |
| `FIRST-PROOF-14` | Embodied temporal rendering, possession neutrality, debug quarantine | `POSSESSION-PARITY`, `VIEW-DEBUG-SPLIT`, `ACTOR-KNOWN`; Possession parity, Epistemic filtering | execution `07` | temporal labels/affordances backed by holder-known evidence; possession/debug do not refresh; transcript/replay stability |
| `FIRST-PROOF-15` | Temporal positive/adversarial fixture families and replay pairing | `FIXTURE-NEGATIVE`, `REPLAY`; Routine blocking, Replay rebuild, Content rejection | execution `09` | registry reachability; premise/staleness/supersession/duration/wait/interruption pairs; live/replay semantic equality |
| `FIRST-PROOF-16` | Typed temporal diagnostics and consolidated five-source line | `REPLAY` plus affected behavioral gates; all applicable families | execution `10` | responsible-layer diagnostic; validator-time versus holder-known-premise distinction; evidence-status/scope honesty; one five-source reconciliation trace |
| `FIRST-PROOF-17` | Cross-gate relational/property/metamorphic/integration closure | all nine gates; all nine families | whole bundle | required relations in section 4.14; recorded seeds; source bans; pairwise/composite interactions; complete configured mutation linkage |

The existing 0044 mutation-floor row is resolved by sections 4 and 5: complete configured denominator, complete union proof, zero final missed/timeout, and no infrastructure floor. It remains an acceptance-package cross-reference, not a new gate.

### 7.3 Nine first-proof gates: required integrated coverage

The canonical gates are exactly those in execution document `02`:[^repo-first-proof-scope]

1. `EVENT`
2. `TRUTH-FIREWALL`
3. `ACTOR-KNOWN`
4. `POSSESSION-PARITY`
5. `NO-HUMAN-ORDINARY-LIFE`
6. `MISSING-PROPERTY`
7. `VIEW-DEBUG-SPLIT`
8. `REPLAY`
9. `FIXTURE-NEGATIVE`

The table marks required evidence relationships. `D` means direct primary witness, `S` means supporting participation, and `N` means an explicit negative/anti-contamination control. These are presentation marks only, not gate codes or result statuses.

| Gate \ scenario | Physical custody | Expectation contradiction | Possession parity | Epistemic filtering | No-hidden-truth planning | No-human ordinary day | Routine blocking | Replay rebuild | Content rejection |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `EVENT` | D | D | S | S | S | D | D | D | S |
| `TRUTH-FIREWALL` | N | D | S | D | D | D | D | S | D |
| `ACTOR-KNOWN` | S | D | S | D | D | D | D | S | N |
| `POSSESSION-PARITY` | S | S | D | D | S | S | S | S | N |
| `NO-HUMAN-ORDINARY-LIFE` | S | S | S | S | D | D | D | D | S |
| `MISSING-PROPERTY` | D | D | S | D | D | S | S | D | D |
| `VIEW-DEBUG-SPLIT` | S | S | D | D | D | S | S | S | N |
| `REPLAY` | D | D | D | D | D | D | D | D | D |
| `FIXTURE-NEGATIVE` | N | D | N | N | D | N | D | N | D |

For each gate the acceptance artifact must answer:

- which production path carried the witness;
- which positive and adversarial fixture/generator exercised it;
- which event/projection/replay artifacts support it;
- which holder-known/source facts were checked;
- which mutation identities strengthen the path;
- which responsible layer would diagnose failure; and
- why the evidence is exhaustive or, where sampled, why sampled evidence is not used alone to render pass.

### 7.4 Nine scenario-family coverage contract

| Scenario family | Required positive path | Required adversarial/negative path | Replay/provenance package | Primary points |
|---|---|---|---|---|
| Physical custody baseline | item/place/container/door/access and legal open/check/inspect/take/place through proposal -> validation -> event -> projection/TUI | inaccessible, locked, nonlocal, wrong-source, and no-teleport rejection | physical event IDs/causes; live/rebuild state and embodied checksums | 02, 04, 10, 17 |
| Expectation contradiction | source-backed expectation plus legal absence observation produces actor-owned contradiction/belief consequence | item present, no source, wrong holder, remote truth, prose/debug insertion | expectation source, observation event, contradiction links, replay equality | 03, 04, 05, 07, 10, 17 |
| Possession parity | same actor/world behavior under autonomous and controller-bound input | possession does not reset intention/memory/expectation or expose hidden truth | binding/input records, event sequence equality, view fingerprints | 08, 14, 17 |
| Epistemic filtering | notebook/view/action menu expose only holder-known facts with provenance/freshness | debug or unobserved truth cannot appear or alter affordance | context IDs/frontiers, source events, view/replay fingerprints | 07, 08, 12, 14, 17 |
| No-hidden-truth planning | need/routine/planner chooses only actor-known local candidates | hidden food/property/route/workplace/time/suspect changes do not alter output before modeled information | candidate/method/proposal traces and paired-run equality | 06, 07, 09, 12, 17 |
| No-human ordinary day | actors perform sleep/eat/work/wait/check/search/stuck through normal transaction/action pipeline without human | no human special case, scheduler delta, marker-only progress, silent loop, or hidden target | event/action/progress/stuck metrics and replay-derived projections | 09, 10, 13, 17 |
| Routine blocking | blocked routine emits typed reason and modeled fallback/wait/recovery | no loop, teleport, direct dispatch, stale-truth correction, or untyped silent stall | blocker/source context, action/rejection events, replayed diagnostics | 09, 13, 15, 16, 17 |
| Replay rebuild | fixture + ordered log rebuilds all authoritative/derived support state and diagnostics | controlled delete/reorder/payload perturbation localizes first divergence | serialized log, event census, per-domain checksums, divergence report | 02–17, especially 10 and 16 |
| Content rejection | valid canonical corpus loads; forbidden content/public API paths reject before runtime | culprit/quest/prose-born knowledge/debug authority/unproven cognition/forged protected records rejected for intended reason | content/manifest/schema fingerprint, typed validation report, no runtime event | 03, 06, 11, 15, 17 |

Scenario acceptance is for the combined contract, not nine isolated smoke tests. A single integrated capstone may cover several families, but each family still needs separately inspectable evidence and required negative branches.

### 7.5 Consolidated five-source temporal bundle

Temporal evidence is part of the existing `FIRST-PROOF-CERT` obligation, not a new gate.[^repo-ladder][^repo-invariants]

| Routed execution source | Required proof at `U` | Primary audit points | Required linkage |
|---|---|---|---|
| `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` | actor-sensitive time/freshness/expected-by-now facts enter only through modeled holder-known provenance; validator/scheduler/debug time cannot propose | 07, 12 | source event/record/observation -> sealed context -> candidate/proposal or contradiction; paired raw-time negative |
| `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | routine windows, waits, durations, interruptions, stale premises, and adaptation behave through ordinary pipeline with single charge | 09, 13 | modeled premise -> routine/method -> proposal/action/event -> accounting/progress/stuck projection |
| `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | temporal labels and affordances are embodied only when holder-known; possession/debug do not upgrade them | 08, 14 | source/context -> view field/action menu -> possession/debug paired run -> replay/transcript |
| `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` | positive/adversarial temporal fixtures are registry-reachable, semantically paired, and replayable | 11, 15 | fixture fingerprint -> trigger -> event/projection -> replay result and negative reason |
| `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | typed temporal diagnostics distinguish validator time from holder-known premise, name layer, and preserve ancestry/evidence scope | 10, 16 | diagnostic -> first divergence/responsible layer -> evidence-status/fingerprint scope -> five-source reconciliation |

The replacement package must include one consolidated row tracing at least one representative temporal fact through all five sources without changing scenario semantics or commit. Separate artifacts that merely coexist do not close the bundle.

### 7.6 Unified command and evidence execution

At `U`, the implementing session must run:

1. exact-commit and clean-tree commands;
2. four clean workspace gates;
3. all named core/content/TUI suites from section 4.9;
4. fixture registry/fingerprint and negative fixture commands;
5. live/replay capstone commands and controlled divergence checks;
6. configured `--list-files` and `--list` census;
7. dedicated baseline plus every shard, or one complete unsharded campaign;
8. completion union merger and synthetic merger negatives;
9. triage-targeted commands during development, labeled non-certifying;
10. final complete rerun after the last remediation; and
11. acceptance-artifact wording/conformance checks.

Every command record must include:

- exact command text;
- exact SHA and clean-tree fingerprint;
- start/end/elapsed time;
- environment/tool versions;
- exit status and supervisor status where applicable;
- stdout/stderr paths;
- output artifact paths and fingerprints;
- evidence status and fingerprint scope; and
- relation to point/gate/scenario/temporal/mutation rows.

### 7.7 Integrated coherence without predecessor re-audit

The replacement artifact must state:

- 0037, 0039, 0041, and 0043 are consumed within their own exact-commit scopes;
- their internals are not presumed current by memory or branch state;
- 0045 does not rerun them as isolated certification campaigns;
- the final command set does exercise their relevant seams as participants in the unified first-proof corpus;
- the new claim is integrated coherence at `U`, not independent latest-main verification; and
- any regression observed in a consumed seam is a live 0045 failure and routes to its earliest responsible layer rather than being ignored because a predecessor once passed.

---
## 8. Replacement acceptance and evidence-artifact contract

### 8.1 Artifact identity and package paths

The implementing session must create a replacement artifact at:

```text
reports/0045_first_proof_cert_mutation_remediation_replacement_certification_acceptance.md
```

and, on accepted closeout, archive it according to repository convention, normally:

```text
archive/reports/0045_first_proof_cert_mutation_remediation_replacement_certification_acceptance.md
```

Required companion package members include at least:

```text
reports/0045_first_proof_cert_mutation_lane_completion_diagnostic.md
reports/0045_first_proof_cert_mutation_census.md
reports/0045_first_proof_cert_mutation_list_files.txt
reports/0045_first_proof_cert_mutation_list.txt
reports/0045_first_proof_cert_mutation_completion_manifest.md
reports/0045_first_proof_cert_mutation_completion_manifest.json
reports/0045_first_proof_cert_mutation_triage_register.md
reports/0045_first_proof_cert_mutation_final_missed.txt
reports/0045_first_proof_cert_mutation_final_timeout.txt
reports/0045_first_proof_cert_emerge_obs.md
reports/0045_first_proof_cert_command_transcripts/
reports/0045_first_proof_cert_mutation_shard_*_of_*.out/
```

Names may be normalized only where repository convention requires it; every artifact must remain unambiguously tied to spec 0045 and the exact final implementation/evidence commit.

### 8.2 Header, source, environment, and supersession block

The replacement artifact must record:

- staging and archive paths for this spec;
- target repository;
- authoring baseline `fd5ae94ff3225d2f989262b95ed8272945861516`;
- exact final implementation/evidence SHA `U`;
- optional documentation-only closeout SHA, separately labeled;
- clean-worktree evidence;
- branch/PR only as non-provenance convenience metadata;
- OS/runner/CPU/memory/disk;
- Rust, Cargo, cargo-mutants, shell/supervisor, and merger versions;
- fingerprints for Cargo.lock, toolchain, workspace manifests, CI, mutation config, baseline-miss file, fixture registry, test sources, supervisor/merger tooling, canonical list, and each shard artifact;
- the exact-commit source-discipline statement;
- freshness statement: supplied target of record, not independently verified latest `main`;
- predecessor artifact paths, exact scopes, and “consumed, not re-audited in isolation” posture;
- archived-spec/history statement;
- posture `Remediation`;
- supersession target; and
- the aggregate result, computed only after all evidence tables are complete.

A passing artifact must state exactly that it supersedes:

```text
reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md
```

for current `FIRST-PROOF-CERT` certification use. It must preserve 0044 and its mutation register as historical evidence of the audit and incomplete campaign.

### 8.3 Command ledger

The command ledger must include every clean, named, replay, fixture, census, baseline, shard, merger, targeted-development, and artifact-conformance command.

For each command record:

- stable local command ID;
- exact command text;
- cwd;
- exact SHA;
- clean-tree fingerprint;
- tool/environment versions;
- start/end/elapsed timestamps;
- launcher/supervisor result;
- child/cargo-mutants result where applicable;
- stdout/stderr paths;
- output paths;
- file/output fingerprint and scope;
- evidence status;
- points/gates/scenarios/temporal/mutation rows supported; and
- whether it is certifying, sampled/development, observer-only, or historical.

A command transcript hash proves the transcript bytes only. It is not automatically a behavior witness or replay proof.

### 8.4 Evidence-item ledger fields

Every cited evidence item must instantiate all fields required by `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:[^repo-template]

- **Evidence item ID** — local stable navigation ID;
- **Requirement/audit-point/gate/scenario/temporal/mutation cross-references**;
- **Evidence status** — `pass`, `fail`, `pending`, `sampled`, `observer-only`, or `historical`;
- **Fingerprint scope** — raw bytes, normalized serialization, parsed semantic content, command transcript, run seed, replay artifact, or not applicable;
- **Evidence summary**;
- **Path under test and behavior witness**:
  - production path/symbol;
  - command, trigger, event, emitter, scheduler entry, TUI input, or fixture runner;
  - responsible layer;
  - accepted/rejected stage;
  - live negative or reason none applies;
  - checked facts/invariants;
- **Replay/provenance ancestry**:
  - fixture/content/schema/ruleset version/fingerprint;
  - seed/random stream where applicable;
  - event-log segment and IDs;
  - source refs and holder-known context identity;
  - projection/extraction version;
  - live/replay checksums and first divergence;
- **Sampling/exhaustiveness scope**;
- **Pending or historical handling**;
- **Certification use** — counted as certifying pass, counted as certifying fail, or not counted; and
- **Staged-abstraction declaration** where relevant.

Local evidence IDs are navigation labels, not canonical gates, obligations, invariants, statuses, or findings.

### 8.5 Required reconciled verdict tables

The replacement artifact must include separately reconciled tables for:

1. `FIRST-PROOF-01` through `FIRST-PROOF-17`;
2. all nine canonical first-proof gates;
3. all nine scenario families;
4. all five temporal-cascade sources;
5. configured mutation completion;
6. every final missed/timeout identity and disposition;
7. every mandatory command; and
8. aggregate acceptance/supersession.

Each row must cite evidence-item IDs. Result cells must be computed only from certifying evidence. Pending, sampled, observer-only, and historical items may be useful context but cannot silently produce a passing row.

A minimum per-point table is:

| Requirement | Responsible layers | Positive evidence | Adversarial/negative evidence | Replay/provenance evidence | Mutation/completion evidence | Result from certifying evidence |
|---|---|---|---|---|---|---|
| `FIRST-PROOF-01` | doctrine; tests/fixtures; documentation; mutation infrastructure | IDs | IDs | IDs | canonical/shard-union IDs | pass/fail/pending |
| `FIRST-PROOF-02` | action validation; event application; projection/replay; view rendering | IDs | IDs | IDs | linked final identities | derived result |
| `FIRST-PROOF-03` | actor-known context; content/fixtures | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-04` | action validation; event application; actor-known context | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-05` | event application; projection/replay; actor-known context | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-06` | content/schema; actor-known/planning; tests/fixtures | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-07` | actor-known context; candidate/planning/proposal | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-08` | view rendering; debug quarantine; input binding | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-09` | scheduler; planning/proposal/action; event application | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-10` | projection/replay; diagnostics | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-11` | content/schema validation; tests/fixtures | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-12` | actor-known context; candidate/planning/proposal; replay | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-13` | scheduler; routines/planning; event/projection | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-14` | view rendering; debug quarantine; holder-known context | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-15` | content/fixtures; projection/replay | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-16` | owning semantic layer; diagnostics; replay; documentation | IDs | IDs | IDs | linked identities | derived result |
| `FIRST-PROOF-17` | earliest reached semantic layer; test oracle | IDs | IDs | IDs | relation and complete-mutation IDs | derived result |
| Configured mutation completion | mutation infrastructure plus semantic layer per identity | clean baseline/census IDs | no-shrink/synthetic merger negatives | structured outcome/union IDs | complete disposition matrix | derived result |
| Artifact/evidence honesty | documentation status; tests/fixtures | template/conformance IDs | pending/sampled/observer-only controls | fingerprints/claim map | register reconciliation | derived result |

### 8.6 Complete configured mutation `pass` row

The mutation row may be `pass` only when all subsections below are proven.

#### Configured denominator

- exact final `.cargo/mutants.toml` fingerprint;
- exact cargo-mutants version;
- effective `--list-files` and `--list` output/fingerprints;
- final selected and generated counts;
- presence of required carriers;
- all authoring-baseline deltas explained;
- no `--no-config`, file/function/filter/exclude/in-diff/iterate denominator shrink;
- no convenience `#[mutants::skip]`;
- no baseline-miss laundering; and
- exact config loaded in CI and local/review execution.

#### Baseline and transport

- unmutated baseline passed at `U`;
- baseline policy identical/valid across shards;
- explicit timeout policy where baseline was skipped;
- exact supervisor/launcher result per shard;
- exact host and wrapper budgets;
- complete artifact retention; and
- no cancellation, forced kill, wall timeout, internal error, spawn/IO error, or artifact-copy failure in the final certifying set.

#### Shard completion and set proof

- all expected shard indices present exactly once;
- same commit/config/toolchain/options/denominator on every shard;
- every shard assigned set equals its outcome set;
- pairwise shard intersections empty;
- shard union equals canonical set;
- no malformed/truncated structured output;
- no missing/duplicate identities; and
- merger and synthetic-negative tests passed.

#### Tool outcome accounting

Record separately:

- canonical total generated;
- caught;
- missed;
- mutant-level timeout;
- unviable;
- baseline failure count;
- cargo-mutants usage/diff/internal failure count;
- wrapper wall timeout/kill/cancellation count; and
- reconciliation equation.

For a passing row:

```text
missed = 0
mutant-level timeout = 0
baseline/tool/infrastructure failures in final certifying set = 0
caught + unviable = canonical generated total
```

`unviable` remains tool outcome evidence and must not be relabeled caught.

#### Survivor/disposition accounting

Record:

- previously unclassified 0044 population size as historical context: 517 at the authoring baseline;
- final newly exposed misses/timeouts;
- final identities killed with behavior witnesses;
- source-changed identities and mapping proof;
- blocked/untriaged identities;
- identities placed in baseline-miss file;
- unresolved historical identities; and
- independent-review signoff.

For a passing row:

```text
newly exposed final missed = 0
newly exposed final timeout = 0
blocked or untriaged = 0
baseline-laundered = 0
unresolved source mapping = 0
```

#### Member evidence

- every final identity appears exactly once in canonical and outcome sets;
- every intermediate miss/timeout has a triage row;
- every killed survivor has active-mutant failure and unmutated-pass evidence;
- every behavior witness has required provenance/replay/negative controls;
- final missed/timeout manifests exactly match the complete final tool sets and are empty for pass;
- `.cargo/mutants-baseline-misses.txt` has no 0045 survivor; and
- all manifests/registers are fingerprinted.

A high score, zero misses before interruption, a clean focused wave, an empty final text file without union proof, or a complete subset cannot satisfy this row.

### 8.7 First-proof behavior, replay, and provenance package

The artifact must include or link separately inspectable evidence for:

1. physical custody and legal container check;
2. source-backed expectation;
3. modeled local absence observation;
4. event-sourced expectation contradiction and belief consequence;
5. no culprit/suspect/clue/quest/story-sifting truth;
6. hidden custody/food/route/workplace/time non-interference;
7. possession parity and possession-does-not-reset;
8. debug off/on non-interference and embodied filtering;
9. no-human ordinary day with real action/wait/stuck progress;
10. routine blocking and no teleport/direct dispatch;
11. temporal five-source chain;
12. complete replay rebuild for each required scenario and integrated capstone;
13. controlled replay tamper with first divergence;
14. content/schema/compile-fail rejection for intended reasons;
15. complete configured mutation union and zero final floor; and
16. full cross-gate property/metamorphic relation package.

For each required fixture and capstone, record:

- fixture ID and semantic fingerprint;
- initial state/content/schema/ruleset/seed;
- ordered event-kind census and event IDs/causes;
- holder/source/provenance references;
- physical, epistemic, agent/accounting, diagnostic, and embodied-support live checksums;
- corresponding replay checksums;
- serialized log/report hashes and fingerprint scopes;
- replay-from-empty-projections statement;
- first-divergence result;
- non-authoritative debug/observer artifacts and their exclusion from authoritative checksums; and
- exact tests/commands that produced the evidence.

### 8.8 Sampling and exhaustiveness declarations

The artifact must state exhaustiveness separately for each evidence class:

- mutation denominator: exhaustive canonical finite set;
- shard coverage: exhaustive exact union;
- named fixtures: exhaustive required registry families;
- compile-fail/negative fixtures: exhaustive declared corpus plus synthetic negatives;
- property/metamorphic generated runs: sampled unless finite domain exhaustively enumerated;
- replay scenarios: exhaustive required scenario set, with generated additions labeled sampled;
- manual review: sampled or exhaustive as actually performed; and
- `EMERGE-OBS`: observer-only, not certifying.

A sampled property corpus may strengthen a row but cannot replace a missing mandatory named fixture, negative, replay package, or complete mutation denominator.

### 8.9 `EMERGE-OBS` package member

Because the verified corpus exercises the first-proof living world, include:

```text
reports/0045_first_proof_cert_emerge_obs.md
```

with:

```text
Evidence status: observer-only
Certification use: not counted as certifying evidence
```

It must be retrospective, event-log-ancestry-backed, reproducible by corpus/seed, and non-input. Its counters, patterns, narratives, or comparisons:

- are not a gate;
- are not a pass/fail threshold;
- are not a mutation score;
- are not a scheduler/planner objective;
- are not a seed selector;
- are not a scenario goal;
- are not a pacing/difficulty target;
- are not an LOD input; and
- cannot compensate for missing mutation completion, behavior, replay/provenance, negative, or diagnostic evidence.

The package is incomplete if the required observer artifact is omitted, but no observed value inside it can make any gate pass or fail. This preserves `INV-111` and execution `10`.[^repo-invariants][^repo-testing]

### 8.10 Staged-abstraction declaration

The artifact must declare `none` or, for every bounded staged abstraction:

- what the first-proof package proves now;
- what is deliberately abstracted;
- what behavior/proof must not be faked;
- which future gate/tier owns the deferred surface;
- what evidence prevents overclaiming; and
- diagnostics distinguishing not implemented, intentionally abstracted, implemented but broken, and overclaimed.

No abstraction may cover:

- complete configured mutation execution;
- a final miss/timeout/tool failure;
- any `FIRST-PROOF-01`…`17` row;
- any of the nine gates or nine scenario families;
- the temporal bundle;
- replay determinism/divergence localization;
- actor-known/provenance/truth-firewall behavior;
- possession/debug separation;
- no-human progress; or
- acceptance evidence honesty.

### 8.11 Aggregate verdict and supersession

The replacement artifact may render exactly:

```text
FIRST-PROOF-CERT passed
```

only when all of the following are true at `U`:

1. all `FIRST-PROOF-01` through `FIRST-PROOF-17` rows pass from certifying evidence;
2. all nine gates pass as one coherent set;
3. all nine scenario families have required positive and adversarial evidence;
4. the five-source temporal bundle closes;
5. clean workspace and every named command pass;
6. replay/projection/diagnostic determinism and first-divergence evidence pass;
7. the complete configured mutation denominator has executed and reconciled;
8. all shards/one full run completed normally with complete retained outputs;
9. zero final misses, zero final mutant-level timeouts, zero missing/duplicate identities, and zero final tool/infrastructure failures remain;
10. every intermediate survivor has behavior/provenance remediation evidence and is caught in the final complete rerun;
11. no denominator shrink or baseline laundering occurred;
12. CI scheduled/manual and in-diff/guard surfaces converge as required;
13. evidence-status, fingerprint-scope, sampling, pending/historical, and certification-use fields are honest;
14. `EMERGE-OBS` is present and observer-only;
15. no pending mandatory item, mixed commit, evidence splice, or substantive deferral remains; and
16. there is no upstream doctrine conflict.

The passing artifact must explicitly say it supersedes the 0044 acceptance artifact for current `FIRST-PROOF-CERT` certification use. It must also say the claim applies only to the exact final implementation/evidence commit and does not independently verify latest `main`, `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, institutions, notices, travel, regional scale, LOD, story-sifting, or LLM dialogue.

If any aggregate condition is unmet, the artifact must remain:

```text
FIRST-PROOF-CERT scoped remediation
```

and name the failed requirement, first responsible layer, retained evidence, and next separately numbered remediation action.

### 8.12 Closeout and ledger handling

On accepted closeout:

- archive this spec by plain rename to the stated `archive/specs/0045_...` path;
- archive the replacement report under the numbered report path;
- preserve 0044 and all earlier audit/remediation artifacts as historical lineage;
- retain mutation, replay, fixture, command, supervisor, shard, merger, and register evidence at stable paths;
- record final SHA and package fingerprints;
- update `docs/4-specs/SPEC_LEDGER.md` to make the supersession and exact scope unambiguous;
- do not present embedded predecessor SHAs as the new baseline;
- do not promote this staging-series spec into live doctrine; and
- begin no downstream expansion inside the closeout commit.

---

## 9. Preliminary static survey — informative, not certification

### 9.1 Status and limits of this survey

This section is a **preliminary, non-certifying static survey** of the exact authoring baseline `fd5ae94ff3225d2f989262b95ed8272945861516`. It reports what checked-in configuration, retained 0044 evidence, workflow text, guards, source, and test harnesses suggest. It does not execute Rust, cargo-mutants, replay, fixtures, CI, or the supervisor. It supplies implementation orientation only and must not be copied into the replacement artifact as live evidence.

No observation below means that:

- the final denominator will still be 62 files or 2,901 mutants after legitimate source edits;
- the previously unclassified 517 mutants have any particular outcome;
- the complete configured campaign now finishes;
- a named suite passes at the final implementation commit;
- a mutant is killed; or
- `FIRST-PROOF-CERT` passes.

### 9.2 Perimeter and retained census

The checked-in `.cargo/mutants.toml` at the authoring baseline:

- uses `additional_cargo_args = ["--locked"]`;
- uses `test_workspace = true`;
- has no `exclude_globs` entry;
- includes `crates/tracewake-core/src/time.rs`;
- includes `crates/tracewake-core/src/actions/defs/checkcontainer.rs`; and
- spans core agent/action/event/replay/epistemic/state/projection/debug surfaces, content load/schema/validation surfaces, and selected TUI surfaces.[^repo-mutants-config]

The retained 0044 list evidence contains 62 selected files and 2,901 generated mutant identities. Fifty-eight selected files generated at least one mutant. Four selected module carriers generated zero mutants in that capture:

- `crates/tracewake-core/src/agent/mod.rs`;
- `crates/tracewake-core/src/epistemics/mod.rs`;
- `crates/tracewake-core/src/events/mod.rs`; and
- `crates/tracewake-core/src/replay/mod.rs`.

Zero generated mutants is not permission to remove a selected path. A final source edit may make a currently zero-generating carrier mutable, and the standing path inventory is itself part of perimeter honesty.[^repo-mutants-files][^repo-mutants-list]

The named first-proof carriers account for these retained generated counts:

| Carrier | Retained 0044 generated identities | Static orientation |
|---|---:|---|
| `crates/tracewake-content/src/validate.rs` | 246 | Content/schema/provenance rejection and fixture admission. |
| `crates/tracewake-core/src/events/apply.rs` | 206 | Event application and the region active when the configured run was interrupted. |
| `crates/tracewake-core/src/epistemics/projection.rs` | 143 | Replay-derived epistemic projection and provenance. |
| `crates/tracewake-core/src/replay/rebuild.rs` | 58 | Rebuild, checksum, diagnostic, and first-divergence behavior. |
| `crates/tracewake-core/src/agent/perception.rs` | 34 | Observation-to-actor-known ingestion. |
| `crates/tracewake-core/src/epistemics/contradiction.rs` | 18 | Expectation-contradiction classification. |
| `crates/tracewake-core/src/actions/defs/checkcontainer.rs` | 14 | Missing-property/container observation carrier. |
| `crates/tracewake-core/src/time.rs` | 9 | Canonical temporal carrier. |

These counts are historical fingerprints of the authoring baseline, not acceptance targets. The implementing session must regenerate them after every semantic delta and explain differences by final-tree source/tool/config facts rather than force the old numbers.

### 9.3 Baseline-miss posture

`.cargo/mutants-baseline-misses.txt` is empty at the authoring baseline.[^repo-mutants-baseline] The static implication is narrow but important: no checked-in miss is currently accepted by that ratchet file. The remediation must keep it from becoming a parking lot for newly exposed first-proof survivors. A new entry would contradict this spec's zero-final-survivor end state and would require a separately justified doctrine change outside 0045.

### 9.4 What the interruption does and does not localize

The retained configured-wave evidence establishes an external supervisor cutoff after 7,200 seconds plus termination grace, with 2,384 of 2,901 identities classified and execution interrupted during a build associated with `events/apply.rs`. The focused seven-file wave separately classified all 206 retained `events/apply.rs` identities as 164 caught and 42 unviable.[^repo-0044-triage]

That does **not** prove which identities made up the configured wave's 517 unclassified remainder. Cargo-mutants scheduling, build reuse, shard algorithms, and source changes can affect execution order. The implementation must recover the unknown set by a complete final census/outcome set difference; it must not infer the set from the last displayed path, source order, or focused-wave results.

### 9.5 Wall-clock scale estimate

Using the retained wrapper interval only as a rough scalar, `2,384 / 7,210 seconds` is approximately `0.331` classified mutants per second. Linear extrapolation would put 2,901 classifications near 8,774 seconds (about 146 minutes), with the remaining 517 near 1,564 seconds (about 26 minutes). This is **not a runtime prediction**: viable and unviable mutants have different costs, incremental-build locality matters, the interrupted mutant may have been expensive, and runner contention can be nonlinear.

The estimate is still enough to reject a brittle design that merely repeats the same single process under the same 7,200-second wrapper and calls the retry a remedy. The implementing session must measure and create margin.

The most credible preliminary topology is:

1. preserve one canonical configured population at the final SHA;
2. use a deterministic matrix of complete `k/n` shards with identical arguments and toolchain;
3. start local cargo-mutants parallelism conservatively and tune from CPU, memory, disk, timeout, and duration evidence;
4. give each shard an explicit job/wrapper budget with retention grace;
5. disable fail-fast cancellation so every shard can retain evidence;
6. merge identities by set membership, not summary arithmetic; and
7. rerun the complete union after survivor fixes.

Cargo-mutants documents `slice` sharding as potentially friendlier to incremental builds and `round-robin` as more likely to equalize mutant counts. The implementation should trial both on non-certifying measurements if shard skew is material, then freeze one algorithm for the certifying campaign.[^cargo-shards]

`--in-place` may reduce tree-copy overhead, but it changes isolation trade-offs and cannot be combined with local `--jobs`; it is not the default recommendation for this multi-job completion problem. Use it only in an isolated runner after measurement and with the same population/evidence controls.[^cargo-build-dirs][^cargo-parallelism]

`--baseline=skip` may avoid duplicating the unmutated suite on every shard, but only after the exact final SHA passes the mandatory clean baseline and only with explicit timeout values and a baseline fingerprint bound to every shard.[^cargo-baseline] `--iterate`, file filters, and `--in-diff` remain development accelerators, never the final union.[^cargo-iterate][^cargo-in-diff]

### 9.6 CI durability gap

At the authoring baseline, the scheduled/manual `mutants-lock-layer` job:

- invokes one unsharded `cargo mutants --workspace --no-shuffle` process;
- has no explicit job-level `timeout-minutes` in the checked-in YAML;
- accepts cargo-mutants exit `0` or `2` for later miss-baseline processing;
- uploads one `mutants.out` artifact under `if: always()`; and
- has no checked-in merger proving full-population identity equality across attempts or shards.[^repo-ci]

The `mutants-in-diff` trigger regex omits both 0044 perimeter additions:

- `crates/tracewake-core/src/time.rs`; and
- `crates/tracewake-core/src/actions/defs/checkcontainer.rs`.

The `STANDING_MUTATION_PERIMETER` and trigger-fragment constants in `ci_workflow_guards.rs` omit those same paths, so the current guard cannot detect that divergence.[^repo-ci-guards] This is a real static durability gap to close in 0045. It is not the direct reason the historical standalone campaign hit its wrapper wall, but leaving it open would allow first-proof perimeter changes to evade the fast lane and would make config/workflow/guard parity dishonest.

### 9.7 Test and fixture orientation

The authoring tree contains the named integrated harnesses and the content fixture runner listed in Section 2.8, including explicit suites for acceptance wording, hidden-truth controls, replay/event schema, negative fixtures, no-human progress, emergence observation, generative locks, and spine conformance. The fixture corpus contains positive and adversarial cases for expectation contradiction, hidden food/route/property, no-human life, possession continuity, debug exclusion, temporal charging, and content/provenance rejection.

This breadth suggests that many survivor fixes can be expressed at an existing public seam. It does not establish that those suites reach every final mutant. The complete campaign, per-mutant witness proof, and final integrated rerun decide that question.

---

## 10. Constitutional-invariant and doctrine alignment

### 10.1 No amendment posture

0045 amends no constitutional invariant. If implementation evidence exposes a genuine conflict with foundation doctrine, the implementing session must stop and route an upstream amendment before changing code or acceptance semantics. Convenience, runtime cost, or a difficult survivor is never authority to weaken an invariant.

### 10.2 Binding alignment matrix

| Concern | Mandatory 0045 posture |
|---|---|
| Causal event authority and replay | State changes travel through accepted event/application seams; replay deterministically rebuilds projections and diagnostics and localizes first divergence. |
| Truth firewall | Validation truth may reject impossible/invalid proposals but may not choose actor goals, methods, food/property targets, suspects, clues, witnesses, or view affordances. |
| Actor-known cognition | Expectations and decisions derive from actor-known, source-bearing observations, beliefs, memories, or records; a witness cannot seed its expected answer from hidden world truth. |
| Missing property | Absence is discovered by comparing an actor-relative expectation with a legitimate observation; no global theft fact, culprit flag, or omniscient property lookup supplies cognition. |
| Proposal/scheduler boundary | Candidate generation, planning, proposal, validation, append, and application remain distinct; the scheduler does not dispatch ordinary actions directly or author state deltas. |
| No protagonist gravity | Simulation progress and every accepted witness remain valid regardless of which actor, if any, is human-controlled. |
| Possession parity | Possession binds input/view perspective only; it grants no privileged memory, body state, action set, provenance, or truth. |
| Embodied/debug split | Embodied views expose holder-known context; debug surfaces remain non-diegetic and never feed cognition or gate behavior. |
| Observer-only emergence | `EMERGE-OBS` records what happened without steering generation or becoming a threshold, score, or gate. This directly preserves `INV-111`. |
| Temporal authority | Canonical time validates temporal legality while holder-known temporal evidence drives planning and embodied rendering. This directly preserves `INV-112`. |
| Evidence honesty | Historical, sampled, pending, incomplete, shrunken, tool-failed, or observer-only evidence cannot be promoted into a pass. |
| Dependency direction | `tracewake-core <- tracewake-content <- tracewake-tui`; no test-only production dependency or reverse edge. |

### 10.3 Anti-Goodhart interpretation

The denominator, gate set, scenario set, temporal bundle, and behavior-witness level are part of the thing being certified. Optimizing only the visible mutation score, deleting difficult carriers, filtering the unclassified tail, teaching a test the mutated literal, or turning emergence observations into targets would improve a proxy while damaging the intended proof. The repository risk register expressly forbids that move; broader work on Goodhart/specification-gaming failure modes reinforces the same design lesson.[^repo-risk][^goodhart]

Therefore the implementing session must prefer:

- full-set completion over a higher score on a subset;
- public behavioral consequences over expression-shaped assertions;
- negative controls over happy-path-only witnesses;
- member-level evidence over aggregate counts;
- observer-only emergence records over steering metrics; and
- an honest `scoped remediation` outcome over an incomplete pass.

---

## 11. Tolerated deferrals and explicit non-goals

The following are outside 0045 and remain routed forward:

| Deferred surface | Route | 0045 rule |
|---|---|---|
| Institutions, households as procedural institutions, official records, wrong-suspicion machinery, suspect/culprit workflows, and full investigation | `PHASE-4-ENTRY`, execution `11` | Do not import them to make missing-property witnesses easier. |
| Notices as a broad gameplay system, travel, regional simulation, long-distance processes, LOD, prehistory expansion, and story-sifting | `SECOND-PROOF-ENTRY`, execution `12` | Do not begin them while FIRST-PROOF-CERT is blocked. |
| LLM dialogue and language generation | Locked foundation/architecture boundary | No LLM-generated fact, goal, clue, or certification evidence. |
| Latest-`main` verification | Independent provenance workflow | Neither the authoring SHA nor final implementation SHA is presented as latest `main` without independent evidence. |
| Redesign of P0/SPINE/EPI/ORD-LIFE internals | Already certified predecessor lines | Consume the four passing artifacts; re-prove only their participation in one coherent final-commit first-proof set. |
| New gate codes, invariant IDs, status enums, obligation codes, or doctrine findings | Upstream doctrine amendment | 0045 mints none. |
| General mutation expansion beyond the standing configured perimeter | Future separately scoped work | Legitimate final-tree mutations inside existing carriers remain included; expanding unrelated feature scope is not required here. |
| Replacement of the full denominator with a smaller contract | Explicitly rejected settled option | Complete the full configured perimeter instead. |

A deferral is tolerated only when it is genuinely downstream and cannot affect any current first-proof row, gate, scenario, temporal source, mutation identity, replay proof, negative control, or acceptance field. A required first-proof gap cannot be labeled a deferral.

---

## 12. Risks and mandatory controls

| Risk | Failure mode | Mandatory control |
|---|---|---|
| Repeat wall cutoff | The same unsharded command exceeds the same external budget again. | Measure first; shard and/or tune conservatively; set explicit budgets with margin; preserve partial evidence; require normal completion. |
| Denominator shrink | Filters, excludes, `--no-config`, `--in-diff`, `--iterate`, source suppression, or stale config omit identities. | Canonical final `--list-files`/`--list`; member-level equality; config diff review; no final subset flags. |
| Shard gap or overlap | Matrix configuration, argument mismatch, cancellation, or merge bug leaves an incomplete union. | Same SHA/tool/config/arguments; `fail-fast: false`; per-shard lists; disjointness and exact-union checks; duplicate/missing identity failure. |
| Aggregate-count false confidence | Counts add to 2,901 while identities are duplicated and others absent. | Compare canonical normalized identity sets; counts are secondary cross-checks only. |
| Artifact loss on interruption | Wrapper/job cancellation destroys the only logs and outcomes. | Unique output path; periodic snapshot or trap/finalizer; upload under `always()`; retention grace; artifact digest/inventory. GitHub artifacts can carry data between matrix and merge jobs and expose upload digests.[^github-artifacts] |
| Fail-fast cancellation | One miss or infrastructure failure cancels other shards before evidence retention. | Matrix `fail-fast: false`; individual shard outcome capture; separate merger verdict.[^github-matrix] |
| Local oversubscription | Excessive `--jobs` causes memory pressure, disk exhaustion, flakiness, or false mutant timeouts. | Conservative `-j2`/`-j3` trials; capture resource telemetry; freeze a measured value; do not trade completion for timeout noise.[^cargo-parallelism] |
| Wrapper/mutant timeout conflation | Exit 124 or job cancellation is recorded as `timeout.txt` or a clean partial run. | Separate transport, supervisor, cargo-mutants exit, and per-mutant outcome axes; preserve raw status and signals. |
| Baseline laundering | Shards skip baseline without binding to a passing clean run at the same SHA. | Preflight artifact fingerprint; explicit baseline reference in every shard; manual timeouts when skip is used.[^cargo-baseline] |
| Survivor parking | Newly exposed misses are added to the baseline file. | Keep final baseline empty; route every miss/timeout through triage and a complete clean rerun. |
| Literal-shaped kill | A test repeats the mutated return, operator, branch, or helper constant. | Require public/certified consequence, mutant-active failure, production seam, provenance/replay ancestry, and negative control. Cargo-mutants likewise recommends testing the public behavior at the right abstraction rather than targeting a private mutation mechanically.[^cargo-using-results] |
| Equivalent-mutant excuse | A difficult miss is declared equivalent without proof or removed by broad skip. | Semantic equivalence argument, public-boundary analysis, reviewer signoff, and source simplification where appropriate; final certifying posture remains zero misses. |
| Truth contamination | A witness obtains the expected missing property, route, food, culprit, or action from world truth/debug/prose. | Source-bearing actor-known setup; paired hidden-truth control; no truth-fed proposal path. |
| Replay oracle weakness | Tests only compare a terminal checksum and cannot localize a drift. | Event-prefix/projection checkpoints; first-divergence event/index/layer; provenance-bearing diagnostics. |
| Temporal double charge or wall-clock leak | Time mutations survive because only final state is asserted. | Interval conservation, boundary-crossing, once-only charging, replay equality, and no wall-clock source controls. |
| Emergence steering | `EMERGE-OBS` becomes a score or acceptance threshold and agents are optimized to it. | Observer-only generation after simulation; no feedback path; no pass/fail predicate over observed values. |
| Commit/evidence splice | Lists, shard outputs, tests, replay, and acceptance rows come from different SHAs. | One final evidence SHA; fingerprints in every artifact; merger rejects mixed identities/config/toolchains. |
| CI parity drift | Config expands but in-diff trigger/guard constants do not. | Derive or exhaustively guard trigger coverage; add `time.rs` and `checkcontainer.rs`; synthetic guard tests for omission. |
| Historical evidence promotion | 0044 focused or predecessor pass evidence is treated as live final proof. | Label history; rerun all certifying evidence at the final SHA; consume predecessor gate status only within its declared scope. |
| Unreviewed output-format assumption | Merger silently depends on unstable cargo-mutants JSON/text formatting. | Pin cargo-mutants, validate schema/version, preserve raw files, fail closed on unknown fields; cargo-mutants documents output formats as subject to change.[^cargo-mutants-out] |

---

## 13. Required implementation and closeout sequence

### 13.1 Ordered execution phases

The implementing session must execute 0045 in this order:

1. **Freeze scope and history.** Record authoring SHA, current working SHA, 0044 historical artifacts, exact standing config, and this spec's non-shrink rule.
2. **Repair infrastructure parity.** Update scheduled/manual mutation topology, supervisor retention, shard/merge machinery, in-diff trigger, CI guards, and execution documentation together.
3. **Measure non-certifying candidates.** Establish baseline build/test duration, per-mutant throughput, shard skew, local `--jobs` resource profile, and wrapper grace; retain results as development evidence only.
4. **Choose and freeze topology.** Record shard count/algorithm, local jobs, runner class, explicit job/wrapper/mutant timeouts, baseline posture, output layout, and merger version.
5. **Make semantic/test corrections.** Apply only changes justified by discovered behavior gaps; preserve authority, dependency, and anti-contamination boundaries.
6. **Freeze final implementation/evidence SHA `U`.** No certifying evidence may predate the last production, test, fixture, workflow, config, supervisor, merger, or relevant documentation change.
7. **Run clean preflight.** Execute all mandatory workspace and named first-proof suites at `U`; stop on any failure.
8. **Generate canonical population.** Capture configured `--list-files` and `--list` at `U`, fingerprints, counts, normalized identities, and final-tree delta explanation.
9. **Execute complete configured campaign.** Launch every shard or the approved single run at `U`; retain output on all outcomes.
10. **Merge and prove completion.** Verify shard normal termination, same inputs, disjointness, exact union, one outcome per identity, and zero missing/duplicates.
11. **Triage every non-clean outcome.** Reconcile identities; classify responsible layer; write behavior/provenance witnesses; reject parking or subset exceptions.
12. **Rerun after every semantic fix.** Refresh preflight, population, and the complete configured campaign at the new final SHA. Never splice old clean identities with new code.
13. **Run integrated live re-proof.** Produce all `FIRST-PROOF-01`…`17`, nine-gate, nine-scenario, temporal, replay, provenance, negative-control, and `EMERGE-OBS` evidence at one SHA.
14. **Assemble replacement package.** Complete register, command ledger, completion proof, acceptance template fields, fingerprints, and supersession statement.
15. **Review and render posture.** Render `passed` only if every acceptance condition closes; otherwise retain `scoped remediation` and name the next action.
16. **Archive without expansion.** Rename the staging spec, preserve all evidence, update the ledger, and do not begin downstream feature work in the closeout.

### 13.2 Implementing-session completion checklist

#### Identity, scope, and authority

- [ ] The authoring baseline is recorded as `fd5ae94ff3225d2f989262b95ed8272945861516`.
- [ ] One exact final implementation/evidence SHA `U` is recorded separately.
- [ ] No artifact claims either SHA is latest `main`.
- [ ] The final tree preserves foundation/architecture/execution authority and `core <- content <- tui`.
- [ ] No new gate code, status enum, obligation code, invariant ID, or doctrine finding is minted.
- [ ] `FIRST-PROOF-CERT` remains a composing label and `EMERGE-OBS` remains observer-only.

#### Clean baseline and first-proof re-proof

- [ ] `cargo fmt --all --check` passes at `U`.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes at `U`.
- [ ] `cargo build --workspace --all-targets --locked` passes at `U`.
- [ ] `cargo test --workspace --locked` passes at `U`.
- [ ] Every named core, content, and TUI first-proof suite in Section 4.9 passes at `U`.
- [ ] `FIRST-PROOF-01` through `FIRST-PROOF-17` are re-proved live.
- [ ] All nine acceptance gates close as one coherent set.
- [ ] All nine scenario families include required positive/adversarial evidence.
- [ ] The five-source temporal bundle closes.
- [ ] Replay evidence localizes first divergence rather than only reporting a final mismatch.

#### Denominator and campaign completion

- [ ] Final configured `--list-files` and `--list` outputs are retained and fingerprinted.
- [ ] The final configured denominator includes `time.rs` and `checkcontainer.rs`.
- [ ] Any difference from 62 files / 2,901 mutants is explained solely by final-tree source/tool/config facts.
- [ ] No final certifying invocation uses `--no-config`, `-f`, `--exclude`, `--in-diff`, `--iterate`, or another subset mechanism.
- [ ] Every shard uses the same SHA, config, toolchain, cargo-mutants version, arguments, shard denominator, and sharding algorithm.
- [ ] Every shard/single run terminates normally within its declared wall budget.
- [ ] All partial and final `mutants.out` artifacts are retained with raw status metadata.
- [ ] Canonical identity set equals the exact disjoint union of shard identity sets.
- [ ] Outcome identities equal the canonical identity set exactly.
- [ ] There are no missing identities, duplicate identities, canceled shards, truncated outputs, or mixed runs.
- [ ] External wall timeout, job timeout/cancellation, tool failure, baseline failure, miss, and mutant-level timeout are reported separately.
- [ ] Final missed manifest is empty.
- [ ] Final mutant-level timeout manifest is empty.
- [ ] `.cargo/mutants-baseline-misses.txt` remains empty.

#### Survivor remediation

- [ ] Every intermediate miss/timeout has historical and final identity reconciliation.
- [ ] Every semantic survivor names a responsible layer and existing FIRST-PROOF cross-reference.
- [ ] Every kill witness passes unmutated and fails with the target mutant active.
- [ ] Every witness observes a public/certified first-proof consequence, not a helper literal.
- [ ] Every event/projection witness carries replay/provenance ancestry.
- [ ] Every relevant witness includes a negative/contamination control.
- [ ] Every fix travels the production seam and adds no test-only production dependency.
- [ ] A complete final configured rerun, not a focused rerun, confirms closure.

#### CI and evidence durability

- [ ] Scheduled/manual full mutation execution is durable and complete.
- [ ] Matrix fail-fast/cancellation posture preserves every shard's evidence.
- [ ] Explicit job/wrapper budgets and retention grace are documented.
- [ ] In-diff trigger coverage includes every standing perimeter path, including `time.rs` and `checkcontainer.rs`.
- [ ] CI guards fail synthetically when either added carrier is removed from config or trigger coverage.
- [ ] The merger fails closed on version/schema/input/identity mismatch.
- [ ] Artifact inventories and digests bind every shard and merged output.

#### Replacement artifact and closeout

- [ ] The acceptance artifact instantiates every field required by `docs/4-specs/0003`.
- [ ] Evidence status distinguishes certifying, development, historical, pending, sampled, and observer-only evidence.
- [ ] The complete configured campaign proof is a required certifying package member.
- [ ] The mutation register, final missed/timeout manifests, command ledger, replay package, temporal bundle, and `EMERGE-OBS` are present.
- [ ] The replacement artifact explicitly supersedes the 0044 acceptance artifact for current FIRST-PROOF-CERT use.
- [ ] A passing artifact says `FIRST-PROOF-CERT passed` only when every aggregate condition in Section 8.11 is true.
- [ ] Any incomplete run, surviving floor, tool failure, semantic seam failure, or evidence gap leaves `FIRST-PROOF-CERT scoped remediation` and names a follow-up.
- [ ] The spec is archived by plain rename only after accepted closeout.
- [ ] No Phase-4, second-proof, institution, notice, travel, regional, LOD, story-sifting, or LLM feature work is smuggled into the closeout.

---

## 14. Outcome prescribed by this specification

0045 prescribes one admissible transition:

```text
FIRST-PROOF-CERT scoped remediation
    -- complete full configured mutation population;
       remediate every exposed survivor with behavior/provenance coverage;
       re-prove the integrated first-proof set at one final SHA;
       publish complete replacement evidence -->
FIRST-PROOF-CERT passed
```

This document does not assert that the transition has occurred. Until the implementing session supplies the required live evidence and an accepted replacement artifact, the standing state remains `FIRST-PROOF-CERT scoped remediation`, and `PHASE-4-ENTRY` plus all downstream expansion remain locked.

---

## Appendix A — Exact-commit acquisition ledger summary

```text
Requested repository: joeloverbeck/tracewake
Target commit: fd5ae94ff3225d2f989262b95ed8272945861516
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: full raw.githubusercontent.com exact-commit URL fetch
Requested file count: 238
Successfully verified file count: 238
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

Every requested repository URL had this mechanical prefix:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/
```

The complete append-only 238-URL list is supplied with this authored deliverable as `tracewake_fd5ae94_evidence_ledger.md`. It is authoring provenance, not a substitute for the implementing session's final-SHA evidence ledger.

---

## Appendix B — Retained 0044 selected-file census (62 files; historical authoring orientation)

This is the exact retained content of `reports/0044_first_proof_cert_mutation_list_files.txt` at the authoring baseline. It is not the final certifying list; regenerate at `U`.

```text
crates/tracewake-content/src/load.rs
crates/tracewake-content/src/manifest.rs
crates/tracewake-content/src/schema.rs
crates/tracewake-content/src/serialization.rs
crates/tracewake-content/src/validate.rs
crates/tracewake-core/src/agent/mod.rs
crates/tracewake-core/src/checksum.rs
crates/tracewake-core/src/controller.rs
crates/tracewake-core/src/debug_capability.rs
crates/tracewake-core/src/debug_reports.rs
crates/tracewake-core/src/epistemics/mod.rs
crates/tracewake-core/src/events/mod.rs
crates/tracewake-core/src/need_accounting.rs
crates/tracewake-core/src/projections.rs
crates/tracewake-core/src/replay/mod.rs
crates/tracewake-core/src/scheduler.rs
crates/tracewake-core/src/state.rs
crates/tracewake-core/src/time.rs
crates/tracewake-core/src/view_models.rs
crates/tracewake-core/src/actions/pipeline.rs
crates/tracewake-core/src/actions/proposal.rs
crates/tracewake-core/src/actions/registry.rs
crates/tracewake-core/src/actions/report.rs
crates/tracewake-core/src/agent/actor_known.rs
crates/tracewake-core/src/agent/candidate.rs
crates/tracewake-core/src/agent/decision.rs
crates/tracewake-core/src/agent/generation.rs
crates/tracewake-core/src/agent/htn.rs
crates/tracewake-core/src/agent/intention.rs
crates/tracewake-core/src/agent/methods.rs
crates/tracewake-core/src/agent/need.rs
crates/tracewake-core/src/agent/no_human_surface.rs
crates/tracewake-core/src/agent/perception.rs
crates/tracewake-core/src/agent/planner.rs
crates/tracewake-core/src/agent/routine.rs
crates/tracewake-core/src/agent/trace.rs
crates/tracewake-core/src/agent/transaction.rs
crates/tracewake-core/src/epistemics/belief.rs
crates/tracewake-core/src/epistemics/contradiction.rs
crates/tracewake-core/src/epistemics/knowledge_basis.rs
crates/tracewake-core/src/epistemics/knowledge_context.rs
crates/tracewake-core/src/epistemics/observation.rs
crates/tracewake-core/src/epistemics/projection.rs
crates/tracewake-core/src/epistemics/proposition.rs
crates/tracewake-core/src/events/apply.rs
crates/tracewake-core/src/events/envelope.rs
crates/tracewake-core/src/events/log.rs
crates/tracewake-core/src/events/mutation.rs
crates/tracewake-core/src/replay/rebuild.rs
crates/tracewake-core/src/replay/report.rs
crates/tracewake-core/src/actions/defs/checkcontainer.rs
crates/tracewake-core/src/actions/defs/continue_routine.rs
crates/tracewake-core/src/actions/defs/eat.rs
crates/tracewake-core/src/actions/defs/movement.rs
crates/tracewake-core/src/actions/defs/need_events.rs
crates/tracewake-core/src/actions/defs/sleep.rs
crates/tracewake-core/src/actions/defs/wait.rs
crates/tracewake-core/src/actions/defs/work.rs
crates/tracewake-tui/src/app.rs
crates/tracewake-tui/src/debug_panels.rs
crates/tracewake-tui/src/render.rs
crates/tracewake-tui/src/transcript.rs
```

---

## Appendix C — External research basis and design consequences

External sources are decision support only. They do not assert what exists in Tracewake and cannot replace a failed target-repository acquisition.

| Source | Design consequence used by 0045 |
|---|---|
| cargo-mutants sharding documentation[^cargo-shards] | Shards are independent `k/n` selections; every shard must use identical arguments and denominator; CI must launch and collect all shards; complete-union proof is therefore an orchestration responsibility. |
| cargo-mutants parallelism documentation[^cargo-parallelism] | Tune `--jobs` conservatively because Cargo/rustc/tests already parallelize and excessive concurrency can cause thrash, resource exhaustion, or timeout noise. |
| cargo-mutants output documentation[^cargo-mutants-out] | Preserve `mutants.json`, `outcomes.json`, per-outcome lists, logs, and diffs; incremental writes make interrupted output diagnostically useful but not complete evidence. |
| cargo-mutants exit-code and timeout documentation[^cargo-exit-codes][^cargo-timeouts] | Keep external wall/job termination separate from cargo-mutants miss, mutant-level timeout, baseline failure, usage failure, and internal error. |
| cargo-mutants baseline documentation[^cargo-baseline] | Baseline skip is safe only when an exact same-SHA clean baseline exists and explicit timeout posture replaces automatic baseline-derived timing. |
| cargo-mutants filter/in-diff/iterate documentation[^cargo-filtering][^cargo-in-diff][^cargo-iterate] | Lists preview the filtered population; in-diff and iterate are fast development feedback but are not substitutes for a complete final run. |
| cargo-mutants public-behavior guidance[^cargo-using-results] | Kill mutants at the externally meaningful abstraction rather than mechanically testing a private helper or mutation spelling. |
| GitHub Actions matrix/workflow syntax[^github-matrix][^github-workflow-syntax] | Use matrix shards, explicit timeout/cancellation posture, and a separate merge job; avoid fail-fast loss of the remaining evidence. |
| GitHub Actions artifact documentation[^github-artifacts] | Upload every shard under all outcomes, carry artifacts into the merger, record retention and digest/inventory data. |
| Property-Based Mutation Testing[^property-mutation] | A mutant is meaningfully killed when a test exercises and detects violation of a specified property; this supports first-proof consequence witnesses rather than mutation-shaped assertions. |
| Property-based tools for metamorphic testing[^metamorphic-review] | Use generated/metamorphic relations for replay equality, possession invariance, observer independence, and temporal conservation where a single fixed oracle would be brittle. |
| Goodhart/specification-gaming analysis[^goodhart] | Guard against optimizing visible mutation counts or emergence summaries at the expense of the actual behavioral/evidence contract. |

---

## Appendix D — Source notes

[^repo-ladder]: [`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md), exact target commit.
[^repo-0037-acceptance]: [`archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md), exact target commit.
[^repo-0039-acceptance]: [`archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md), exact target commit.
[^repo-0041-acceptance]: [`archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md), exact target commit.
[^repo-0043-acceptance]: [`archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md), exact target commit.
[^repo-0044-acceptance]: [`reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md), exact target commit.
[^repo-0044-triage]: [`reports/0044_first_proof_cert_mutation_triage_register.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/reports/0044_first_proof_cert_mutation_triage_register.md), exact target commit.
[^repo-ledger]: [`docs/4-specs/SPEC_LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/4-specs/SPEC_LEDGER.md), exact target commit.
[^repo-docs-readme]: [`docs/README.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/README.md), exact target commit.
[^repo-arch-index]: [`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md), exact target commit.
[^repo-exec-index]: [`docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md), exact target commit.
[^repo-first-proof-scope]: [`docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md), exact target commit.
[^repo-spec-rules]: [`docs/4-specs/README.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/4-specs/README.md), exact target commit.
[^repo-testing]: [`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md), exact target commit.
[^repo-0044-spec]: [`archive/specs/0044_FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/archive/specs/0044_FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC.md), exact target commit.
[^repo-0043-spec]: [`archive/specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/archive/specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md), exact target commit.
[^repo-0041-spec]: [`archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md), exact target commit.
[^repo-0039-spec]: [`archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md), exact target commit.
[^repo-0037-spec]: [`archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md), exact target commit.
[^repo-glossary]: [`docs/3-reference/02_GLOSSARY.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/3-reference/02_GLOSSARY.md), exact target commit.
[^repo-risk]: [`docs/3-reference/01_DESIGN_RISK_REGISTER.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/3-reference/01_DESIGN_RISK_REGISTER.md), exact target commit.
[^repo-template]: [`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md), exact target commit.
[^repo-village]: [`docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md), exact target commit.
[^repo-mutants-config]: [`.cargo/mutants.toml`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/.cargo/mutants.toml), exact target commit.
[^repo-mutants-files]: [`reports/0044_first_proof_cert_mutation_list_files.txt`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/reports/0044_first_proof_cert_mutation_list_files.txt), exact target commit.
[^repo-mutants-list]: [`reports/0044_first_proof_cert_mutation_list.txt`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/reports/0044_first_proof_cert_mutation_list.txt), exact target commit.
[^repo-mutants-baseline]: [`.cargo/mutants-baseline-misses.txt`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/.cargo/mutants-baseline-misses.txt), exact target commit; file is empty at that commit.
[^repo-ci]: [`.github/workflows/ci.yml`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/.github/workflows/ci.yml), exact target commit.
[^repo-ci-guards]: [`crates/tracewake-core/tests/ci_workflow_guards.rs`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/crates/tracewake-core/tests/ci_workflow_guards.rs), exact target commit.
[^repo-invariants]: [`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/fd5ae94ff3225d2f989262b95ed8272945861516/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md), exact target commit.

[^cargo-filtering]: cargo-mutants, [Filtering files](https://mutants.rs/skip_files.html), including `--list-files`/`--list` population preview and file-filter semantics.
[^cargo-shards]: cargo-mutants, [Sharding](https://mutants.rs/shards.html), including independent `k/n` selection, consistency requirements, shard algorithms, CI collection responsibility, and performance model.
[^cargo-parallelism]: cargo-mutants, [Parallelism](https://mutants.rs/parallelism.html), including conservative `--jobs` guidance and resource/timeout cautions.
[^cargo-exit-codes]: cargo-mutants, [Exit codes](https://mutants.rs/exit-codes.html).
[^cargo-mutants-out]: cargo-mutants, [The `mutants.out` directory](https://mutants.rs/mutants-out.html), including incremental output and outcome-file inventory.
[^cargo-baseline]: cargo-mutants, [Baseline tests](https://mutants.rs/baseline.html).
[^cargo-in-diff]: cargo-mutants, [Testing code changed in a diff](https://mutants.rs/in-diff.html), including the warning that incremental mutation runs do not replace a full run.
[^cargo-timeouts]: cargo-mutants, [Hangs and timeouts](https://mutants.rs/timeouts.html).
[^cargo-iterate]: cargo-mutants, [Iterating on missed mutants](https://mutants.rs/iterate.html), including its heuristic nature and required final non-iterate rerun.
[^cargo-build-dirs]: cargo-mutants, [Copying the tree](https://mutants.rs/build-dirs.html), including separate build directories under `--jobs` and `--in-place` isolation trade-offs.
[^cargo-using-results]: cargo-mutants, [Using the results](https://mutants.rs/using-results.html), recommending tests of meaningful public behavior rather than mutation-shaped private internals.
[^github-matrix]: GitHub Docs, [Running variations of jobs in a workflow](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/run-job-variations).
[^github-workflow-syntax]: GitHub Docs, [Workflow syntax for GitHub Actions](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax), including matrix strategy, `fail-fast`, `max-parallel`, conditions, and timeout syntax.
[^github-artifacts]: GitHub Docs, [Store and share data with workflow artifacts](https://docs.github.com/en/actions/tutorials/store-and-share-data), including matrix-job data transfer, retention, and artifact digests.
[^property-mutation]: Ezio Bartocci, Leonardo Mariani, Dejan Nickovic, and Drishti Yadav, [Property-Based Mutation Testing](https://arxiv.org/abs/2301.13615), 2023.
[^metamorphic-review]: Nasser Alzahrani, Maria Spichkova, and James Harland, [Application of property-based testing tools for metamorphic testing](https://arxiv.org/abs/2211.12003), 2022.
[^goodhart]: David Manheim, [Multiparty Dynamics and Failure Modes for Machine Learning and Artificial Intelligence](https://arxiv.org/abs/1810.10862), 2018; used only as general specification-gaming/Goodhart framing, not as repository evidence.

