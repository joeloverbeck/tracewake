# 0041 — EPI-CERT mutation remediation and replacement certification spec

**Staging path:** `specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
**Archive path on accepted closeout:** `archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
**Target repository:** `joeloverbeck/tracewake`  
**Authoring and reassessment baseline:** `7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5`  
**Spec series:** numbered implementation spec, staged under `specs/`, archived on acceptance closeout  
**Status:** proposed; implementation-ready; non-executable in the authoring session  
**Future-spec posture:** `Remediation`  
**Admissibility posture:** `EPI-CERT scoped remediation`  
**Consumed predecessor gates:** `P0-CERT passed` through the 0037 replacement artifact; `SPINE-CERT passed` through the 0039 replacement artifact  
**Certification-line effect:** successful execution must produce a replacement EPI-CERT acceptance artifact that renders `EPI-CERT passed` and explicitly supersedes the 0040 EPI-CERT acceptance artifact

This spec is subordinate to foundation, architecture, execution, reference, and live spec-layer doctrine. It operationalizes the single admissible next move named by the live ledger: remediate the configured EPI-CERT mutation survivor floor, re-prove the complete EPI seam contract, and publish a replacement certification artifact. It does not amend an invariant, redefine a gate, create a gate or observation obligation, reopen `P0-CERT` or `SPINE-CERT`, or advance a later certification line.[^repo-ledger][^repo-ladder][^repo-spec-rules]

This document is non-executable. It specifies what an implementing session must inspect, change, run, prove, review, and package. It does **not** assert that a survivor is already killed or equivalent, that a new mutation run has completed, or that `EPI-CERT` has passed.

> I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

---

## 1. Status, determination, source discipline, freshness, and admissibility

### 1.1 Single posture and certification-line state

This spec declares exactly one future-spec posture: `Remediation`, as defined by `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`. Its admissibility posture is `EPI-CERT scoped remediation` because it fixes the named failed certification layer and adds no gameplay scope.[^repo-ladder]

The EPI line remains `EPI-CERT scoped remediation` until an implementing session satisfies this spec and publishes the replacement acceptance artifact. This spec does not flip the line. Only the completed replacement artifact may render `EPI-CERT passed`, and only when every survivor disposition, every live EPI seam row, and every evidence-honesty requirement passes.

The spec consumes these predecessor states as settled context:

- `P0-CERT passed`, established by `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`;
- `SPINE-CERT passed`, established by `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`.

Neither predecessor is re-opened, re-scoped, or re-certified here. Shared event/replay/pipeline surfaces may be exercised as ancestry for an EPI behavior witness, but that use is continuity evidence for EPI—not a second SPINE audit.[^repo-0037-acceptance][^repo-0039-acceptance]

### 1.2 Determination confirmation: this is the next admissible move

The execution ladder places `EPI-CERT` at gate 3, before `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, and `SECOND-PROOF-ENTRY`. Gate failure handling requires a remediation spec/report naming the failing layer; a failed gate may not be relabeled as a phase exception.[^repo-ladder]

The live ledger states that the 0040 artifact renders `EPI-CERT scoped remediation`, not `EPI-CERT passed`, because the configured EPI mutation perimeter left a 30-mutant survivor floor. It names a separately numbered EPI-CERT mutation-remediation and replacement-certification spec as the next known execution move. The 0040 artifact itself forbids later specs from citing `EPI-CERT passed` or advancing to later ladder stages while the mutation floor remains unresolved.[^repo-ledger][^repo-0040-acceptance]

Therefore this spec does not reconsider alternatives. `ORD-LIFE-CERT`, the 0035 expansion backlog, institutions, wrong-suspicion work, notices, travel, regional scale, LOD, LLM/speech, story-sifting, Phase-4 entry, and second-proof work are inadmissible as the next certification move.

### 1.3 Exact-commit source discipline

The source target of record is the user-supplied exact commit:

```text
7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5
```

The uploaded manifest is path inventory only. Branch names, default-branch lookup, repository metadata, connector namespace labels, code-search snippets, prior chats, old repository names, uploaded filenames, and commit strings embedded inside archived artifacts are not proof of target-commit content.

All repository source used to author this spec was fetched through full exact-commit URLs of the form:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/<manifest-path>
```

No clone, GitHub code search, snippet-based repository search, branch-name file fetch, repository-scoped lookup, default-branch lookup, or current-repository connector call was used. Appendix A records the exact URL evidence ledger.

The `ba9fe1c…` string recorded by the 0040 spec/artifact is 0040 audit provenance only. The `92ba47f…`, `9648c8f…`, and other hashes inside historical artifacts likewise belong to those artifacts. None replaces this spec's authoring baseline.[^repo-ledger][^repo-0040-spec][^repo-0040-acceptance]

### 1.4 Authoring baseline versus final implementation commit

An implementation that adds or changes tests, production code, mutation configuration, CI triggers, or evidence artifacts cannot truthfully retain the unchanged baseline SHA. Accordingly:

1. implementation must begin from the target-of-record baseline or a transparently recorded descendant containing it;
2. every remediation delta from `7a17447d…` must be enumerated;
3. all certifying commands must run against one exact final implementation commit;
4. the replacement artifact must identify that exact final commit and the `7a17447d…` source baseline separately; and
5. neither SHA may be presented as independent verification of current `main`.

“Re-prove live from `7a17447d…`” means: re-establish the EPI contract from this target baseline, through the actual remediation delta, at the final exact commit under test. The historical 0040 pass-shaped seam rows, command transcripts, and `ba9fe1c…` fingerprints are context and structural precedent, not live replacement certification.

### 1.5 Admissibility lock and closeout convention

Until the replacement artifact passes:

- no later spec may cite `EPI-CERT passed`;
- `ORD-LIFE-CERT` may not start or claim progression;
- `FIRST-PROOF-CERT` may not be advanced;
- `PHASE-4-ENTRY` and `SECOND-PROOF-ENTRY` remain locked;
- feature, institution, notice, travel, regional, LOD, LLM, and story-sifting work remains outside this spec and blocked by its named entry gates.

The staged path is `specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`. On accepted closeout it is archived under the corresponding `archive/specs/0041_…` path. This hardening-series artifact is not promoted into live `docs/4-specs/`; only the accepted closeout state is reflected in the live ledger. The ledger row is a closeout responsibility; this spec does not pre-author its own acceptance entry.

---

## 2. Authority and dependency declarations

### 2.1 Authority order

The controlling order is:

1. `docs/0-foundation/`
2. `docs/1-architecture/`
3. `docs/2-execution/`
4. `docs/3-reference/`
5. `docs/4-specs/`
6. this staged numbered implementation spec

Earlier tiers govern later tiers. If execution conflicts with architecture or foundation, execution is wrong. If a convenient implementation bypasses holder-known provenance, event/replay ancestry, possession parity, or debug quarantine, the implementation is wrong. If a test kills a mutant by asserting the mutated helper's literal return instead of an EPI-certified consequence, the test is wrong. If an artifact promotes archived evidence into a live pass, the artifact is invalid.[^repo-docs-readme][^repo-arch-index]

### 2.2 Primary remediation sources and structural precedents

The direct remediation objects are:

- `reports/0040_epi_cert_mutation_triage_register.md` — the configured perimeter change, exact commands, 30-row seed inventory, EPI cross-references, responsible layers, and artifact fingerprints;
- `reports/0040_epi_cert_mutation_final_missed.txt` — the canonical 30 unique missed-mutant identities;
- `archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` — the artifact to supersede and its `EPI-CERT scoped remediation` verdict;
- `archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` — the EPI-01 through EPI-11 seam contract to re-prove;
- `docs/4-specs/SPEC_LEDGER.md` — live source discipline and the mandatory next move.[^repo-0040-triage][^repo-0040-missed][^repo-0040-acceptance][^repo-0040-spec][^repo-ledger]

The structural precedents are:

- `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` and its replacement acceptance artifact — the closest remediation shape, adapted here from “296 survivors found outside the configured perimeter” to “30 survivors found inside the already-expanded perimeter”;
- `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` and its replacement acceptance artifact — the original paired audit/remediation pattern;
- `reports/0039_spine_cert_mutation_triage_register.md` — the register anatomy reused here.[^repo-0039-spec][^repo-0039-acceptance][^repo-0037-spec][^repo-0037-acceptance][^repo-0039-triage]

All archived specs, reports, and tickets are historical evidence or structural precedent. They are not live certification for the final implementation commit.

### 2.3 Foundation dependencies

The remediation must preserve and witness consequences of:

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`;
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`;
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`;
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`;
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`.[^repo-invariants][^repo-causal][^repo-beliefs-foundation][^repo-tui-foundation][^repo-firewall-foundation]

These sources establish the event authority, replay ancestry, typed/fallible belief distinction, subjective information-flow boundary, possession parity, embodied/debug split, and “truth may validate, truth may not plan” firewall. This spec operationalizes them and may not weaken them.

### 2.4 Architecture dependencies

The controlling subsystem contracts are:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`;
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`;
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`;
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`;
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`;
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`.[^repo-arch-index][^repo-arch-replay][^repo-arch-context][^repo-arch-beliefs][^repo-arch-tui][^repo-arch-evidence]

The core rule is that `EpistemicProjection` is replay-derived and may not become a truth writer; holder-known contexts are sealed; provenance is sufficient and frontier-bounded; embodied and debug surfaces are distinct; and acceptance claims require evidence scoped to their real behavior.

### 2.5 Execution dependencies and gate vocabulary

Execution authority is supplied by:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`;
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`;
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`;
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`;
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`;
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`;
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`;
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`.[^repo-exec-index][^repo-exec-boundary][^repo-first-proof][^repo-ladder][^repo-exec-firewall][^repo-exec-epi][^repo-exec-fixtures][^repo-exec-testing]

The work composes only existing canonical gates—`TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, and `DIAG`—under the existing `EPI-CERT` phase-certification label and the acceptance-contract cross-references in execution document `02`. These names are cross-references only. This spec defines no gate code, observation-obligation code, status vocabulary, invariant ID, or doctrine-level finding ID.

`EMERGE-OBS` remains an observer-only observation obligation. It is never a mutation score, pass/fail threshold, phase gate, scheduler objective, scenario goal, or substitute for a failed EPI seam.

### 2.6 Reference and live-spec dependencies

Terminology, risk memory, source discipline, and artifact shape are controlled by:

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`;
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`;
- `docs/3-reference/02_GLOSSARY.md`;
- `docs/4-specs/README.md`;
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`;
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.[^repo-ref-index][^repo-risk][^repo-glossary][^repo-spec-rules][^repo-fixture-contract][^repo-accept-template]

The risk register's post-0008 warning and anti-Goodhart rule are active controls: killing a representative subset, improving a score, treating the 30 historical identities as the denominator of convenience, or copying an archived pass-shaped row cannot substitute for completing the standing run and triaging every survivor.

### 2.7 Crate direction and implementation boundary

The one-way dependency direction remains:

```text
tracewake-core <- tracewake-content <- tracewake-tui
```

`tracewake-core` owns the authoritative kernel and epistemic types; content may validate and load against core; TUI may consume core/content view models. No remediation may introduce a reverse dependency, a TUI-owned epistemic authority, a content-to-TUI backchannel, or a test-only production dependency.

### 2.8 Primary code, test, and configuration seams

The four survivor-bearing files are:

| File | Seed survivors | Primary behavior responsibility |
|---|---:|---|
| `crates/tracewake-core/src/epistemics/proposition.rs` | 21 | typed proposition contradiction, canonical parsing/serialization/rendering, reference validation, diagnostic formatting |
| `crates/tracewake-core/src/epistemics/belief.rs` | 4 | freshness policy and observation/contradiction provenance linkage |
| `crates/tracewake-core/src/epistemics/contradiction.rs` | 2 | contradiction activity and expected-absence eligibility |
| `crates/tracewake-core/src/epistemics/observation.rs` | 3 | bounded confidence value and low-confidence classification |
| **Historical seed total** | **30** | **floor, not finish line** |

The surrounding EPI seam is:

- `crates/tracewake-core/src/epistemics/projection.rs` — replay-derived observations, beliefs, contradictions, actor-known records, context-filtered reads, debug views, and canonical checksums;
- `crates/tracewake-core/src/epistemics/knowledge_context.rs` — sealed viewer/holder/mode/tick/frontier/source/scope/provenance packet, context identity/hash, and forbidden-truth audit;
- `crates/tracewake-core/src/epistemics/knowledge_basis.rs` — source-backed support queries consumed by actor-known logic;
- `crates/tracewake-core/src/epistemics/mod.rs` — public EPI module boundary.[^repo-proposition][^repo-belief][^repo-contradiction][^repo-observation][^repo-projection][^repo-context][^repo-basis][^repo-epistemics-mod]

The standing mutation and lock-layer controls are:

- `.cargo/mutants.toml`;
- `.cargo/mutants-baseline-misses.txt`;
- `.github/workflows/ci.yml`.[^repo-mutants-config][^repo-baseline-misses][^repo-ci]

The existing behavior-witness surfaces include:

- core: `hidden_truth_gates`, `event_schema_replay_gates`, `acceptance_gates`, `anti_regression_guards`, `generative_lock`, `golden_scenarios`, `negative_fixture_runner`, `spine_conformance`, `no_human_capstone`, and `emergence_ledger`;
- content: `fixtures_load`, `forbidden_content`, `golden_fixtures_run`, and `schema_conformance`;
- TUI: `adversarial_gates`, `tui_seam_conformance`, `transcript_snapshot`, `tui_acceptance`, `embodied_flow`, and `command_loop_session`.[^repo-core-hidden-tests][^repo-core-replay-tests][^repo-core-acceptance-tests][^repo-core-regression-tests][^repo-core-generative-tests][^repo-core-golden-tests][^repo-core-negative-tests][^repo-core-spine-tests][^repo-core-nohuman-tests][^repo-core-emergence-tests][^repo-content-tests][^repo-tui-tests]

No backwards-compatibility shim, alias path, symmetry-only file, test-only branch, direct truth read, direct projection write, or alternate behavior path is admissible.

---

## 3. Problem statement

### 3.1 The 0040 verdict and mutation floor

The 0040 audit did not render `EPI-CERT passed`. Its aggregate verdict is `EPI-CERT scoped remediation` because its configured EPI mutation run completed with a unique 30-mutant missed floor and accepted none as equivalent or non-critical.[^repo-0040-acceptance][^repo-0040-triage]

The 0040 acceptance artifact identifies the failing mutation evidence row as `MUT-WAVEB-001`. This spec cites that label only as a historical artifact cross-reference; survivor identities remain cargo-mutants path/symbol/operator entries, and 0041 mints no new finding ID.

The historical configured run was:

```bash
cargo mutants --workspace --no-shuffle -j 8 \
  -o reports/0040_epi_cert_mutation_wave_b_j8.out
```

Its recorded result was:

| Tested | Caught | Unviable | Missed | Timeout |
|---:|---:|---:|---:|---:|
| 2763 | 2143 | 589 | 27 | 4 |

The timeout retry was:

```bash
cargo mutants --workspace --no-shuffle -j 1 --timeout 600 \
  -F 'current_place_perception_events|Confidence::parts_per_thousand|PropositionParseError|PropositionReferenceError' \
  -o reports/0040_epi_cert_mutation_wave_b_timeout_retry.out
```

Its recorded result was 22 tested, 10 caught, 5 unviable, 7 missed, and zero timeouts. Deduplication across the full run and retry produced the canonical 30 identities in `reports/0040_epi_cert_mutation_final_missed.txt`.[^repo-0040-triage][^repo-0040-missed]

### 3.2 Central divergence from 0039

This remediation must not repeat 0039's primary perimeter-expansion problem.

- The 0039 SPINE audit discovered 296 survivors outside its configured perimeter through `--no-config`; the remediation therefore had to widen the standing denominator.
- The 0040 EPI audit already additively expanded `.cargo/mutants.toml` with `crates/tracewake-core/src/epistemics/**`, increasing its file census from 48 to 54 and its configured mutant census to 2763. The 30 survivors were found **inside** that configured posture, with no final `--no-config` or narrow `-f` file list.[^repo-0039-spec][^repo-0040-triage][^repo-mutants-config]

The 0041 center of gravity is therefore **kill, complete, reconcile, triage, and re-prove**, not wholesale perimeter re-derivation.

### 3.3 Static perimeter confirmation at the authoring baseline

At `7a17447d…`, `.cargo/mutants.toml` contains `crates/tracewake-core/src/epistemics/**`. That glob covers all four survivor-bearing files. No exclusion glob is present in the checked-in configuration, and `test_workspace = true` requires workspace tests against each mutant. `.cargo/mutants-baseline-misses.txt` is empty at this baseline.[^repo-mutants-config][^repo-baseline-misses]

This static confirmation is not a certifying census. The implementing session must still run `--list-files` and `--list` against the final implementation commit and prove that all survivor files and all generated identities remain in the configured denominator.

### 3.4 CI durability gap requiring a minimal correction

The scheduled lock-layer job runs:

```bash
cargo mutants --workspace --no-shuffle
```

and therefore consumes the checked-in full configured perimeter. However, the in-diff trigger regex at the authoring baseline names only:

```text
crates/tracewake-core/src/epistemics/(knowledge_context|projection)\.rs
```

It does not trigger on direct changes to `belief.rs`, `contradiction.rs`, `observation.rs`, or `proposition.rs`. This is a documentation/CI durability gap: the certified denominator is present for scheduled runs, but a pull request touching the survivor-bearing files can evade the in-diff mutation job unless another guarded path also changes.[^repo-ci]

The implementing session must make the minimal correction: align the trigger with the standing `epistemics/**` perimeter, preferably by matching `crates/tracewake-core/src/epistemics/`. This correction must not narrow, duplicate, or replace the checked-in configuration; the config remains the single denominator of record.

The configuration's leading comment still calls the union a “Standing SPINE-CERT mutation perimeter.” Because the file now includes the EPI expansion, the implementing session should correct that descriptive comment to a neutral standing certification/lock-layer perimeter. This is documentation status, not a new gate.

### 3.5 Responsible failing layers

The 0040 register routes the seed floor to existing execution-layer diagnostics:

- `projection/replay`;
- freshness policy within the projection/replay responsibility;
- provenance linkage;
- contradiction linkage and contradiction state/detection;
- observation confidence;
- canonical proposition rendering and parsing;
- proposal construction;
- view-model rendering;
- content/schema validation;
- diagnostics.

The complete-run remediation may expose additional survivors. Every additional survivor must be assigned to one of the existing failure layers in execution document `03`; no phase exception or new layer name may be invented.

### 3.6 The 30 identities are a seed floor, not the finish line

The 30 historical identities are mandatory reconciliation work, but they do not define acceptance. Acceptance requires a complete run over the final standing configuration. If that run enumerates more mutants, exposes new misses, changes a timeout into a miss, or reveals a prior identity under a new line/symbol, those outcomes join the same remediation obligation.

A narrow campaign that kills only the named 30, improves a mutation score, or records the rest as baseline misses is anti-Goodhart behavior and cannot certify the line.

---

## 4. Remediation approach

### 4.1 Required end state

Execution is complete only when all of the following are true:

1. The implementation records the `7a17447d…` source baseline and one exact final implementation commit.
2. `.cargo/mutants.toml` retains the standing `epistemics/**` perimeter without shrinkage or silent exclusion.
3. `--list-files` proves that `proposition.rs`, `belief.rs`, `contradiction.rs`, and `observation.rs` remain in the configured census.
4. `--list` captures the complete final mutant population and fingerprints the census.
5. The scheduled CI job continues to run the checked-in full configured posture.
6. The in-diff trigger is minimally corrected to include every `epistemics/**` source change.
7. No historical EPI survivor is added to `.cargo/mutants-baseline-misses.txt` as a convenience.
8. The unmutated workspace and every named EPI gate suite pass at the final implementation commit.
9. The standing configured mutation posture runs to completion, unsharded or across a provably complete shard set.
10. Every one of the 30 historical seed identities is reconciled to the final source and evidence.
11. Every additional final-run survivor is added to the register and triaged.
12. Kill-with-behavior/provenance coverage is the default; every grouped family proves member-by-member catch reach.
13. Every equivalent or non-critical exception carries exact call-site/domain reasoning and independent reviewer signoff.
14. Zero survivors remain blocked or untriaged; zero timeouts or tool failures remain unresolved.
15. EPI-01 through EPI-11 are re-proven at the exact final implementation commit.
16. The replacement artifact conforms to `docs/4-specs/0003`, renders `EPI-CERT passed`, and explicitly supersedes the 0040 artifact.

If any item remains incomplete, the artifact must remain `EPI-CERT scoped remediation` and name the responsible layer. It may not render a partial aggregate pass.

### 4.2 Maintain and re-confirm the standing configured perimeter

The checked-in configuration is the denominator of record. The certifying run must load `.cargo/mutants.toml`; it must not use `--no-config`, a custom replacement config, `-f`, `--file`, `--exclude`, `--in-diff`, or `--iterate` as the final denominator.

Before mutation execution, the implementing session must capture:

```bash
cargo mutants --workspace --no-shuffle --list-files \
  > reports/0041_epi_cert_mutation_list_files.txt

cargo mutants --workspace --no-shuffle --list \
  > reports/0041_epi_cert_mutation_list.txt
```

Equivalent output paths are allowed, but the replacement artifact must identify them exactly.

The file-census review must prove:

- the four survivor files are present;
- all surrounding `crates/tracewake-core/src/epistemics/**` files remain present;
- no exclusion glob or command-line option removes them;
- `test_workspace = true` or its exact equivalent remains active;
- `additional_cargo_args = ["--locked"]` or the final declared locked-dependency posture remains active;
- the CI command loads the same checked-in config; and
- any file-count change from 54 is explained by final-tree source changes rather than treated as automatic failure or automatic pass.

A minimal perimeter correction is required only if the live `--list-files` census proves a survivor-bearing or required EPI seam file is absent. Such a correction must be additive and justified. A wholesale 0039-style re-derivation is out of scope.

Cargo-mutants documents that the checked-in `.cargo/mutants.toml` is the default configuration, `--no-config` disables it, file filters control the mutated population, and `--list-files`/`--list` preview the effective denominator.[^cargo-config][^cargo-files]

### 4.3 CI convergence and baseline-miss discipline

The CI implementation must satisfy all of the following:

- scheduled mutation continues to use the checked-in config with `cargo mutants --workspace --no-shuffle` or an equivalent command that does not replace its perimeter;
- in-diff guarded-path detection includes `crates/tracewake-core/src/epistemics/` as a directory-level trigger;
- in-diff mutation remains a fast change detector, not the replacement for the certifying full run;
- non-zero tool/run failures remain distinguishable from cargo-mutants' missed-mutant result;
- outputs are retained even when the mutation job fails;
- baseline normalization does not collapse distinct symbols/operators into one permissive accepted line; and
- any future baseline exception links to the complete review disposition.

At the authoring baseline the baseline-miss file is empty. The 30 EPI survivors must not be added to it. If owners retain baseline exceptions as a mechanism, an entry is permissible only after the same equivalent/non-critical proof and signoff required by section 4.11, and the replacement artifact must still report it as an approved exception rather than “caught.”

### 4.4 Clean baseline and named preflight

Before interpreting any mutant outcome, the implementing session must establish a clean unmutated baseline at the final implementation commit:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
```

It must then run the EPI-relevant named binaries at minimum:

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test emergence_ledger
cargo test --locked -p tracewake-content --test fixtures_load
cargo test --locked -p tracewake-content --test forbidden_content
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-content --test schema_conformance
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test tui_seam_conformance
cargo test --locked -p tracewake-tui --test transcript_snapshot
cargo test --locked -p tracewake-tui --test tui_acceptance
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-tui --test command_loop_session
```

The final artifact must record exact commands, exit results, toolchain versions, environment-affecting options, and transcript references. A mutation run against a failing baseline is meaningless; cargo-mutants' own baseline guidance makes the same distinction.[^cargo-baseline]

### 4.5 Development loops versus certifying run

During implementation, focused runs are allowed to shorten feedback:

- exact `-F` function/mutant filters;
- exact file filters;
- `--iterate`;
- single-test commands;
- manual application of a mutant diff;
- low-concurrency timeout investigation.

Every such run is non-certifying development evidence. It may prove that a particular test kills a particular mutant, but it cannot establish the final denominator or aggregate pass.

The certifying run must be a complete configured run. The default unsharded form is:

```bash
cargo mutants --workspace --no-shuffle \
  -o reports/0041_epi_cert_mutation_full.out
```

The implementing session may choose explicit job count and timeout options, but must record them and explain deviations from the checked-in/default posture. The final run must not use `--no-config`, a file/function filter, `--in-diff`, or `--iterate`.

### 4.6 Sharding, timeouts, and completion proof

Sharding is permissible only as a reproducible partition of one complete denominator. If used:

- every shard must use the same exact final commit, config, cargo-mutants version, Rust/Cargo versions, arguments, filter set, shuffle posture, and denominator `n`;
- shard indexes must cover exactly `0..n-1` with no duplicates or omissions;
- each shard's `mutants.json`, `outcomes.json`, outcome text files, logs, and command transcript must be retained;
- a precomputed unsharded `--list` census must be reconciled against the union of shard identities;
- per-outcome counts must reconcile to the union without duplicate counting;
- any failed or absent shard blocks run completion; and
- `--baseline=skip`, if used on shards, is permissible only after one separately recorded passing baseline at the same exact commit and environment.

Cargo-mutants documents that shards independently discover and partition the population, and that inconsistent arguments or denominators make results meaningless.[^cargo-shards]

A timeout is not a pass. Each timeout must be retried under an explicit, recorded policy and resolved to one of:

- caught;
- missed and then killed by remediation;
- approved equivalent;
- approved non-critical;
- unviable under the final toolchain; or
- blocked/tool failure, which prevents certification.

A retry may use a narrow exact identity/function filter to diagnose the timeout, but the final aggregate evidence must still point back to the complete configured census. Cargo-mutants' timeout controls prevent hangs; they do not confer semantic disposition.[^cargo-timeouts]

### 4.7 Survivor identity and reconciliation

The 0040 register is a seed work-list. Each historical entry must be tracked using:

- the raw 0040 identity (`path:line:column:operator`);
- normalized identity (`path`, enclosing symbol/function, exact mutation operator/diff);
- final-checkout identity, if still generated;
- source-change mapping when line numbers or implementation shape move;
- EPI seam and responsible-layer mapping; and
- final evidence/disposition.

A historical survivor does not disappear merely because a refactor moves its line, renames a helper, folds a match arm, changes cargo-mutants' generated wording, or removes the exact syntactic mutation. The register must map it to the production/test change and certified behavior witness that closes the original semantic gap.

Every additional survivor from the complete final run is appended to the same register. Survivors remain cargo-mutants identities; this spec mints no doctrine-level finding IDs.

### 4.8 Triage register schema

The replacement package must include or link a complete register, reusing the 0040/0039 format and containing at least:

| Field | Required content |
|---|---|
| Historical identity | Raw 0040 entry when applicable; otherwise `new in completed run`. |
| Final identity | Final path, symbol/function, operator/diff, structured-output reference. |
| Tool outcome | `caught`, `missed`, `timeout`, `unviable`, or tool failure as emitted by the tool. |
| Responsible EPI cross-reference | One or more of EPI-01…EPI-11; local cross-reference only. |
| Responsible failing layer | One of execution document `03`'s existing diagnostic layers. |
| Certified reachability | Exact consuming call sites and invariant/gate/seam consequences. |
| Test family | Named behavior-contract family and exact parameter/case. |
| Behavior witness | Certified consequence—not a helper literal or getter tautology. |
| Replay/provenance ancestry | Event IDs/log range, projection/context/replay package, manifest/schema identity, or justified `not applicable`. |
| Negative/contamination control | Forbidden truth/debug/prose/direct-write/invalid-input shortcut that remains failing or absent. |
| Certification disposition | Killed, equivalent, non-critical, or blocked/untriaged; register disposition only, not a new status vocabulary. |
| Evidence references | Baseline transcript, mutant-active failure, final caught output, artifact IDs, fingerprints. |
| Review signoff | Implementer plus independent reviewer evidence for every exception and register closeout. |

Tool outcomes and certification dispositions are separate axes. A final `missed` tool outcome may be accepted only with approved equivalent or non-critical evidence; it must never be counted as “caught.”

### 4.9 Kill-with-behavior/provenance coverage is the default

A valid killing witness must:

1. pass against the unmutated final implementation;
2. fail when the specific mutant is active;
3. observe a public or certified subsystem consequence rather than restating the mutated expression;
4. travel through the production EPI seam, not a test-only bypass;
5. carry replay/provenance ancestry when the behavior is event-derived, projected, holder-known, or serialized;
6. include a negative/contamination control where truth, debug, prose, direct state, or invalid references could provide a shortcut; and
7. name the responsible layer exposed by the failure.

Insufficient examples include:

- asserting only that a string or set is non-empty;
- asserting `parts_per_thousand()` equals the value inserted without showing a downstream EPI consequence;
- asserting `is_low()` directly on one low value without proving high/low behavior changes an actor/debug/projection contract;
- asserting `Display` emits any text while ignoring canonical semantic content;
- calling `FromStr` directly without exercising the production serialization/replay consumer;
- asserting a validation helper returns an error while bypassing content load/schema rejection and typed diagnostics;
- reading a raw projection map or constructing a belief/observation literal through a test-only path; or
- sourcing the expected result from authoritative truth or debug output.

Cargo-mutants advises repairing misses through externally observable behavior at the appropriate abstraction rather than tests over private implementation details.[^cargo-results] Property-based mutation testing likewise motivates tying a kill to violation of a named property, not merely to incidental output divergence.[^property-mutation]

### 4.10 Grouped behavior-contract families

Kindred survivors may share a parameterized, table-driven, property-based, golden-fixture, or metamorphic contract family. Grouping is permitted to express the 30-survivor floor coherently; it is not sampling.

For each family, the evidence package must include a member-by-member matrix with:

- mutant identity;
- exact parameter/case that reaches it;
- unmutated result;
- assertion or semantic comparison that fails under that mutant;
- replay/provenance and negative-control references; and
- cargo-mutants output proving that identity was caught.

A family that kills only a representative member is insufficient. Generated/property tests may explore more cases, and metamorphic relations may avoid brittle literal oracles, but certification still requires at least one retained concrete execution proving each grouped survivor is caught.[^quickcheck][^property-mutation][^metamorphic]

### 4.11 Equivalent and non-critical exceptions

Equivalent-mutant determination is difficult, and human classification is error-prone. “No test failed,” apparent triviality, compiler optimization, or a high score is never sufficient.[^equivalent-review][^equivalent-empirical][^state-infection]

A claimed equivalent mutant must include:

- exact mutant diff;
- all reachable call sites at the final commit;
- the complete reachable input/domain argument;
- semantic proof that no reachable execution can distinguish original and mutant;
- generated/property or compiler/IR evidence as supporting material only; and
- independent reviewer signoff.

A claimed non-critical mutant may be behavior-changing, but must prove that no EPI invariant, canonical gate, EPI-01…EPI-11 seam, replay/provenance obligation, possession-parity obligation, actor-visible surface, or debug-quarantine property can observe the difference. “Not currently rendered” is not enough when the value participates in serialized/replayed or future-visible EPI state.

Neither exception may be used to defer work, hide a missing behavior consumer, or suppress mutant generation. The exact signoff process remains an owner decision.

### 4.12 Production-code integrity and staged abstractions

Production code may be corrected when a survivor reveals a defect, an unexpressed contract, an unused API that should be removed, or a legitimate missing consumer. It may not be changed merely to stop cargo-mutants from generating a mutant, move code outside the perimeter, or introduce an unobservable test-only branch.

No `#[mutants::skip]`, exclusion glob, dead-code trick, compatibility alias, alternate parser, shadow serializer, direct truth read, raw projection insertion, debug-to-embodied bridge, or test-only production path may make a survivor disappear without the same semantic proof required for an exception.

A staged abstraction may bound future `ORD-LIFE-CERT`, institution, notice, travel, LOD, regional, LLM, or story-sifting surfaces. It may not defer:

- any survivor disposition;
- EPI-01…EPI-11 evidence;
- replay/provenance ancestry;
- holder-known context sealing;
- possession parity;
- embodied/debug separation; or
- debug quarantine.


---

## 5. Per-survivor-group remediation deliverables

This section groups the 30 historical identities into thirteen behavior-contract families. The grouping is organizational only: the final register and cargo-mutants evidence must preserve all 30 identities separately, and each identity must have its own caught or approved-exception evidence. Each family must be implemented through the narrowest existing test surface that still witnesses the real EPI consequence. New test files are allowed only when no existing coherent suite can own the contract; symmetry alone is not a reason to add one.

### 5.1 Belief freshness policy: `stale_after_tick` return-value replacements

**Historical identities (2):**

- `crates/tracewake-core/src/epistemics/belief.rs:150:9: replace Belief::stale_after_tick -> Option with None`
- `crates/tracewake-core/src/epistemics/belief.rs:150:9: replace Belief::stale_after_tick -> Option with Some(Default::default())`

**EPI cross-references:** EPI-02 and EPI-06.  
**Responsible layer:** projection/replay; freshness policy.

**Kill-default obligation.** Begin with a final-commit constructibility and call-site census. At the authoring baseline, `Belief::new` initializes `stale_after_tick` to `None`, `BeliefDraft` exposes no stale-frontier field, and the inspected surrounding projection freshness classifier applies to `ActorKnownProjectionRecord`, not to `Belief::stale_after_tick`. The implementation must not conflate those two freshness models or assume that a nonzero belief frontier is already reachable.[^repo-belief][^repo-projection]

The preferred remediation, when upstream belief doctrine requires this field, is to establish one checked production path that derives or admits a belief stale frontier from typed event/schema policy, persists it through the real projection/replay path, and consumes it in an actor-known or evidence surface. The resulting boundary-valued contract must exercise ticks before, exactly at, and after the frontier and prove a certified consequence such as belief freshness classification, context inclusion, supersession, notebook/view evidence, or another replay-stable EPI decision. It must not merely compare `belief.stale_after_tick()` with a value supplied by the test.

That kill path must include at least:

- a legitimately constructed event-backed belief with an explicit nonzero stale frontier;
- the same holder/proposition observed at a newer tick, where applicable, to prove supersession or reclassification rather than a fixed literal;
- an event/replay rebuild whose belief representation and consuming surface match the live application; and
- a no-new-observation advance that demonstrates stale knowledge does not silently refresh from truth.

Under that established contract, the `None` mutant must fail because the required frontier disappears, and the `Some(Default::default())` mutant must fail because the frontier is wrong. If the final reachable domain still cannot produce `Some(_)`, or no production consumer can observe the accessor, the implementation must treat this as a dead/unfulfilled contract rather than manufacture a test-only consumer: remove or narrow the redundant field/API with complete semantic reconciliation, or submit the rare section 4.11 equivalence/non-critical case with full-domain and call-site proof plus independent signoff. Kill remains the default; equivalence is not inferred merely from the baseline's current `None` initialization.

**Negative/contamination control.** Change authoritative truth after the belief is formed without adding a perception/notice event. The actor-known/embodied surface must retain or age the belief according to its recorded frontier; it must not refresh from truth.

**Likely ownership.** Core projection/replay or hidden-truth suites, with a fixture-backed TUI assertion only when the behavior is already surfaced there.

### 5.2 Belief witness links: `observation_ids` and `contradiction_ids` emptied

**Historical identities (2):**

- `crates/tracewake-core/src/epistemics/belief.rs:154:9: replace Belief::observation_ids -> &BTreeSet with Box::leak(Box::new(BTreeSet::new()))`
- `crates/tracewake-core/src/epistemics/belief.rs:158:9: replace Belief::contradiction_ids -> &BTreeSet with Box::leak(Box::new(BTreeSet::new()))`

**EPI cross-references:** EPI-02, EPI-03, EPI-04, and EPI-05.  
**Responsible layer:** projection/replay; provenance linkage; contradiction linkage.

**Kill-default obligation.** Build one event-backed belief whose support contains a known observation ID, then apply a genuinely contradicting observation that yields a known contradiction ID. Carry the belief through the public/context-filtered projection and a real diagnostic, notebook, checksum/evidence record, or debug view that exposes witness-chain integrity. At the authoring baseline the inspected projection checksum and `DebugBeliefEntry` do not themselves serialize these two link sets; the implementing session must therefore prove the actual final production consumer or close that evidence-path gap rather than assume the accessors are already consequential.[^repo-belief][^repo-projection][^repo-view-models] The test must establish that:

1. the support observation exists in the replayed projection and belongs to the correct holder/channel/source event;
2. the belief links to that observation—not merely to some observation count;
3. the contradiction exists, links the same expectation and contradicting observation, and is linked back to the belief where the production model requires it; and
4. replay produces the same link sets and canonical evidence/fingerprint.

A single table-driven family may kill both mutants, but its evidence must identify the observation-link assertion that fails for the first and the contradiction-link assertion that fails for the second.

**Negative/contamination controls.** An observation for another actor, an observation outside the privacy scope, and an unrelated contradiction must not be attached. A forged raw ID or prose assertion without the source record must fail closed. Debug may inspect the chain but may not supply or repair it.

**Likely ownership.** Core `event_schema_replay_gates`, `hidden_truth_gates`, or `acceptance_gates`, with debug-view verification in an existing TUI adversarial suite when needed.

### 5.3 Expected-absence eligibility: `||` changed to `&&`

**Historical identity (1):**

- `crates/tracewake-core/src/epistemics/contradiction.rs:127:13: replace || with && in detect_expected_absences`

**EPI cross-reference:** EPI-04.  
**Responsible layer:** projection/replay; absence-contradiction detection.

**Kill-default obligation.** Exercise `detect_expected_absences` through the evented observation/projection path with the complete two-condition eligibility matrix:

| Holder matches observation | Belief stance is the required expectation stance | Expected result |
|---|---|---|
| yes | yes | eligible; create the expected-absence contradiction when the item is absent |
| yes | no | ineligible; no contradiction |
| no | yes | ineligible; no contradiction |
| no | no | ineligible; no contradiction |

The operator mutant is killed only if both one-mismatch rows are retained and demonstrably fail under the mutant. The positive row must name the prior belief, contradicting observation, expected proposition, observed/missing proposition, holder, source event, and detection tick. Replay must reproduce the same contradiction ID/linkage or the same deterministic derivation evidence.

**Negative/contamination control.** Hidden container truth must not create an absence contradiction until the actor receives the qualifying contents-observed event. Another actor's expectation must remain private.

**Likely ownership.** Core golden/acceptance/replay tests, preferably reusing the `expectation_contradiction_001` and hidden-container fixture vocabulary rather than constructing an isolated helper test.

### 5.4 Contradiction activity: `ContradictionKind::is_active -> true`

**Historical identity (1):**

- `crates/tracewake-core/src/epistemics/contradiction.rs:28:9: replace ContradictionKind::is_active -> bool with true`

**EPI cross-reference:** EPI-04.  
**Responsible layer:** projection/replay; contradiction state.

**Kill-default obligation.** First enumerate every final-commit `ContradictionKind` variant and every production call site of `is_active`. Where the domain contains both active and inactive kinds, add a public behavior matrix proving that inactive contradictions do not drive the active contradiction surface, diagnostics, planner blockers, notebook entries, or view-model warnings, while active kinds do. The test must reach the consumer, not assert the helper directly.

At the authoring baseline the enum appears to contain only `ExpectedItemAbsentFromContainer`, whose implementation returns `true`. This makes equivalence plausible but **not established**. A passing disposition may therefore take one of two forms:

- kill the mutant through an already-existing distinguishable domain case discovered at the final commit; or
- prove equivalence across the complete final enum domain and all reachable call sites, explain why returning `true` is extensionally identical today, and obtain independent reviewer signoff.

Adding an invented inactive contradiction kind solely to kill this mutant is forbidden. Removing a redundant method may be legitimate only if the final code and all call sites preserve doctrine and the historical semantic gap is reconciled explicitly.

**Negative/contamination control.** No debug-only classification or hidden truth may be used to manufacture an inactive case for embodied behavior.

**Likely ownership.** Core contradiction/projection acceptance tests plus the exception register if equivalence is proven.

### 5.5 Observation confidence: numeric-return and low-classification replacements

**Historical identities (3):**

- `crates/tracewake-core/src/epistemics/observation.rs:25:9: replace Confidence::parts_per_thousand -> u16 with 0`
- `crates/tracewake-core/src/epistemics/observation.rs:25:9: replace Confidence::parts_per_thousand -> u16 with 1`
- `crates/tracewake-core/src/epistemics/observation.rs:29:9: replace Confidence::is_low -> bool with true`

**EPI cross-reference:** EPI-03.  
**Responsible layer:** projection/replay; observation confidence.

**Kill-default obligation.** Begin by enumerating every final production call site of `Confidence::parts_per_thousand` and `Confidence::is_low`. At the authoring baseline, `serialize_canonical` reads the private numeric field directly, and the inspected projection checksum/debug entries call `serialize_canonical`; those paths do **not** execute either surviving accessor. A checksum assertion alone therefore cannot be claimed to kill these mutants.[^repo-observation][^repo-projection]

Where the upstream EPI contract requires the raw numeric value or low/non-low classification to affect a production surface, establish that real consumer and exercise at least three valid confidence values spanning the contract, including the exact `350`/`351` low-threshold boundary and a nontrivial high value. The behavior family must then prove a replay/provenance-backed consequence such as a typed confidence class in an evidence record, notebook/debug/view diagnostic, belief/contradiction policy, or proposal consequence. Use nontrivial values such as `250` and `875` so the `0` and `1` return replacements are distinguishable.

A direct `parts_per_thousand()` round-trip, `is_low()` assertion, or a serialization/checksum assertion that bypasses those methods is supporting unit evidence only. It cannot be the certification witness. The `is_low -> true` mutant requires a high-confidence production control whose downstream classification remains non-low.

If the final code has no real consumer for one or both methods, the implementing session must not invent a test-only consumer or route behavior through an arbitrary new UI label merely to improve the mutation count. It must either establish the missing doctrinally required consumer in the existing EPI seam, remove/narrow a genuinely redundant API with semantic reconciliation, or present the rare section 4.11 exception with complete final-domain/call-site proof and independent signoff.

**Negative/contamination controls.** Invalid values outside the bounded confidence domain must remain rejected. Hidden truth may not upgrade confidence. Debug may display confidence but may not feed it back into actor-known behavior.

**Likely ownership.** Core event-schema/replay and debug/view-model suites, with TUI snapshots only where confidence is already user-visible.

### 5.6 At-place proposition contradiction truth table

**Historical identities (4):**

- `crates/tracewake-core/src/epistemics/proposition.rs:260:13: delete match arm (Proposition::ItemLocatedAtPlace{item_id, place_id}, Proposition::ItemMissingFromExpectedLocation{item_id:missing_item_id, expected_location:Location::AtPlace(expected_place_id),},) in Proposition::contradicts_one_way`
- `crates/tracewake-core/src/epistemics/proposition.rs:266:26: replace == with != in Proposition::contradicts_one_way`
- `crates/tracewake-core/src/epistemics/proposition.rs:266:45: replace && with || in Proposition::contradicts_one_way`
- `crates/tracewake-core/src/epistemics/proposition.rs:266:57: replace == with != in Proposition::contradicts_one_way`

**EPI cross-reference:** EPI-04.  
**Responsible layer:** projection/replay; proposition contradiction.

**Kill-default obligation.** Define a table-driven at-place relation with all four equality combinations:

| Item IDs equal | Place IDs equal | `contradicts` |
|---|---|---|
| yes | yes | true |
| yes | no | false |
| no | yes | false |
| no | no | false |

Run every row in both operand orders because `Proposition::contradicts` promises symmetric behavior by applying the one-way relation in either direction. Then carry the exact-match row through contradiction detection and replay so the matched relation creates the typed contradiction linkage, while each mismatch row creates none. The member matrix must identify which row kills each equality/operator mutant and which positive row kills the deleted arm.

**Negative/contamination controls.** An authoritative item location not represented by an actor observation must not create the contradiction. A proposition for another item or another place must remain unrelated even if the hidden world state happens to match.

**Likely ownership.** Core proposition tests for exhaustive relation support plus core replay/golden tests for the certification witness.

### 5.7 Carried-by proposition contradiction truth table

**Historical identities (4):**

- `crates/tracewake-core/src/epistemics/proposition.rs:267:13: delete match arm (Proposition::ItemCarriedByActor{item_id, actor_id}, Proposition::ItemMissingFromExpectedLocation{item_id:missing_item_id, expected_location:Location::CarriedBy(expected_actor_id),},) in Proposition::contradicts_one_way`
- `crates/tracewake-core/src/epistemics/proposition.rs:273:26: replace == with != in Proposition::contradicts_one_way`
- `crates/tracewake-core/src/epistemics/proposition.rs:273:45: replace && with || in Proposition::contradicts_one_way`
- `crates/tracewake-core/src/epistemics/proposition.rs:273:57: replace == with != in Proposition::contradicts_one_way`

**EPI cross-reference:** EPI-04.  
**Responsible layer:** projection/replay; proposition contradiction.

**Kill-default obligation.** Mirror section 5.6 with the complete item/actor equality matrix. The exact item and actor match must contradict; item-only, actor-only, and double mismatches must not. Both operand orders must be exercised. At least one production witness must arise from an evented carried-item observation and survive replay into the correct holder's contradiction surface. Every grouped identity must have a distinct row or assertion shown to fail under that identity.

**Negative/contamination controls.** Possessing a different actor in the UI/controller must not rewrite the belief holder or make the carried-by proposition belong to the controller. Debug attachment must not create a carried-item observation.

**Likely ownership.** Core proposition relation tests plus possession/replay or hidden-truth acceptance tests.

### 5.8 Canonical proposition display: `Display -> Ok(default)`

**Historical identity (1):**

- `crates/tracewake-core/src/epistemics/proposition.rs:281:9: replace ::fmt -> fmt::Result with Ok(Default::default())`

**EPI cross-references:** EPI-02, EPI-04, and EPI-06.  
**Responsible layer:** projection/replay; canonical proposition rendering.

**Kill-default obligation.** Drive every proposition variant through a production diagnostic, notebook/debug entry, why-not surface, or transcript that relies on `Display`, and compare the rendered meaning with the typed proposition carried by the same event/projection evidence. At minimum, the family must include an expected-location contradiction and one non-contradiction proposition. The visible text must preserve the semantic identifiers and relation needed to diagnose the event chain.

`format!("{proposition}") == proposition.render()` across variants is useful unit support and will kill this mutant, but it is insufficient alone. At the authoring baseline `Display` delegates to `render`, while the inspected projection debug contradiction summary calls `render` directly; a production `Display` consumer is therefore not established merely by those files.[^repo-proposition][^repo-projection] The implementing session must enumerate final call sites and prove that an empty/default formatter erases a real responsible-layer explanation or evidence surface while the underlying typed record remains intact. If no such consumer exists, it must remove/narrow the redundant formatter or submit the rare reviewed exception rather than certify a format-only unit test.

**Negative/contamination control.** The embodied diagnostic must not add hidden/debug-only facts in order to make the message meaningful. Debug output may be richer but must retain capability separation.

**Likely ownership.** Core diagnostic/acceptance tests and an existing TUI transcript or adversarial suite where proposition text is already rendered.

### 5.9 Canonical proposition parser arms

**Historical identities (3):**

- `crates/tracewake-core/src/epistemics/proposition.rs:301:13: delete match arm ["item_carried_by_actor", item_id, actor_id] in ::from_str`
- `crates/tracewake-core/src/epistemics/proposition.rs:305:13: delete match arm ["container_contents_observed", container_id] in ::from_str`
- `crates/tracewake-core/src/epistemics/proposition.rs:319:13: delete match arm ["possible_movement_near_place", place_id] in ::from_str`

**EPI cross-references:** EPI-02, EPI-03, EPI-04, EPI-06, EPI-07, and EPI-09.  
**Responsible layers:** projection/replay; proposition parsing; proposal construction; view-model rendering.

**Kill-default obligation.** Add a parameterized canonical round-trip corpus containing every `Proposition` variant, with retained concrete cases for the three historical arms. For each case:

1. construct the typed proposition through a legitimate event/fixture path;
2. serialize to canonical bytes;
3. deserialize through the same production parser used by content/replay/evidence packaging;
4. apply/rebuild through the projection when that proposition participates in EPI state; and
5. compare typed semantics, source ancestry, checksum input, and the relevant downstream action/view/contradiction consequence.

The three named arms must each have a concrete downstream witness:

- `item_carried_by_actor` preserves carried-item belief/expectation semantics and possession-related evidence;
- `container_contents_observed` enables expected-absence/contradiction detection only after the observation event;
- `possible_movement_near_place` survives into the holder-known proposal or embodied informational surface without becoming truth.

A parser-only “returns `Ok`” assertion is insufficient. The deleted arm must break a round-trip on which the projection, proposal, or view contract actually depends.

**Negative/contamination controls.** Unknown tags, wrong arity, invalid IDs, and prose-shaped inputs must remain typed failures. No fallback parser or alias spelling may be introduced for compatibility.

**Likely ownership.** Core replay/schema gates, content schema/golden fixture tests, and TUI seam tests for the movement proposition where already rendered.

### 5.10 Expected-location deserialization arms

**Historical identities (2):**

- `crates/tracewake-core/src/epistemics/proposition.rs:349:9: delete match arm "at_place" in deserialize_location`
- `crates/tracewake-core/src/epistemics/proposition.rs:351:9: delete match arm "carried_by" in deserialize_location`

**EPI cross-references:** EPI-02 and EPI-04.  
**Responsible layer:** projection/replay; location parsing.

**Kill-default obligation.** Round-trip `ItemMissingFromExpectedLocation` through canonical bytes for all three location forms—`AtPlace`, `InContainer`, and `CarriedBy`—then use each result in the appropriate contradiction relation. Retain separate concrete cases for the two historical arms. Replay must reproduce the typed location and downstream contradiction linkage; canonical reserialization must match the original bytes.

**Negative/contamination controls.** Missing `kind:id` structure, unknown location tags, empty IDs, and mismatched reference kinds must fail with `PropositionParseError::InvalidLocationShape` or the precise typed ID error. No unknown location may silently become a default place, container, or actor.

**Likely ownership.** Core canonical/replay gates, with content schema tests where serialized propositions enter fixture data.

### 5.11 Expected-location rendering: `render_location -> "xyzzy"` or empty

**Historical identities (2):**

- `crates/tracewake-core/src/epistemics/proposition.rs:357:5: replace render_location -> String with "xyzzy".into()`
- `crates/tracewake-core/src/epistemics/proposition.rs:357:5: replace render_location -> String with String::new()`

**EPI cross-references:** EPI-02, EPI-06, and EPI-09.  
**Responsible layers:** projection/replay; view-model rendering.

**Kill-default obligation.** Render expected-missing propositions for `AtPlace`, `InContainer`, and `CarriedBy` through a real contradiction/notebook/view-model or typed diagnostic surface. The surface must retain both the relation kind (`place`, `container`, or `actor`) and the concrete identifier. Pair the text with the underlying typed proposition and source evidence so the assertion is semantic rather than “non-empty string.”

The member matrix must show that the arbitrary-string mutant and empty-string mutant both destroy the responsible-layer explanation. A snapshot/golden may own exact wording, but the test must also assert structured identity so a broadly updated snapshot cannot launder the mutant.

**Negative/contamination controls.** Embodied rendering may report only the holder-known expectation and contradiction. It must not reveal the actual hidden location of the missing item. A debug view may expose additional trace detail only with the debug capability.

**Likely ownership.** Core view-model/diagnostic tests plus TUI transcript/adversarial tests where the message is already displayed.

### 5.12 Reference validation: `validate_location`, `require_place`, and `require_container -> Ok(())`

**Historical identities (3):**

- `crates/tracewake-core/src/epistemics/proposition.rs:370:5: replace validate_location -> Result<(), PropositionReferenceError> with Ok(())`
- `crates/tracewake-core/src/epistemics/proposition.rs:392:5: replace require_place -> Result<(), PropositionReferenceError> with Ok(())`
- `crates/tracewake-core/src/epistemics/proposition.rs:403:5: replace require_container -> Result<(), PropositionReferenceError> with Ok(())`

**EPI cross-references:** EPI-02, EPI-03, and EPI-05.  
**Responsible layers:** content/schema validation; projection/replay.

**Kill-default obligation.** Exercise reference validation through the real content/fixture loading boundary, not solely by calling the private helpers. Build a compact matrix containing:

- a valid proposition for each affected reference shape;
- an expected-location proposition with an unknown place;
- an expected-location proposition with an unknown container;
- a direct place-referencing proposition with an unknown place;
- a container-contents proposition with an unknown container; and
- a valid item with an invalid secondary reference, proving the validator does not stop after checking the item.

Each invalid fixture must fail closed before entering the live projection and must identify the exact typed error (`UnknownPlace` or `UnknownContainer`), fixture/content source, and responsible validation layer. The valid controls must load and replay successfully. The grouped family must retain a case that demonstrably fails under each of the three mutants.

**Negative/contamination controls.** Unknown references must not be repaired from prose labels, debug truth, a similarly named entity, or a default ID. A failed fixture must leave no partial observation, belief, or proposition in the projection.

**Likely ownership.** `tracewake-content` schema/fixture tests and core replay rejection evidence; reuse existing forbidden-provenance/prose-born-fact fixture conventions.

### 5.13 Typed parse/reference diagnostics: error `Display -> Ok(default)`

**Historical identities (2):**

- `crates/tracewake-core/src/epistemics/proposition.rs:51:9: replace ::fmt -> fmt::Result with Ok(Default::default())`
- `crates/tracewake-core/src/epistemics/proposition.rs:83:9: replace ::fmt -> fmt::Result with Ok(Default::default())`

**EPI cross-references:** EPI-02 and EPI-05.  
**Responsible layers:** content/schema validation; diagnostics.

**Kill-default obligation.** Cause one real unknown-reference failure and one real malformed-canonical-proposition failure through the content/replay boundary, then inspect the typed diagnostic delivered to the acceptance/review surface. The diagnostic must identify:

- the responsible layer;
- the fixture/content or replay record being rejected;
- the stable error kind;
- the implicated reference ID or canonical-shape category when safe; and
- the fact that no EPI record was committed.

Directly formatting the error is supporting unit evidence only. The certification witness must show that the default/empty display would make an actual review diagnostic non-actionable or would violate the typed failure contract. Exact prose may be snapshot-tested, but the artifact must preserve the structured error kind independently of wording.

**Negative/contamination controls.** Diagnostics must not dump hidden world state, debug-only context, secrets, or unrelated actor-private records. Error handling must not fall back to accepting the malformed proposition.

**Likely ownership.** Content schema/forbidden-content tests, core negative fixture runner, and acceptance diagnostic tests.

### 5.14 Group closeout rule

A survivor family is complete only when:

- every historical identity appears in the final reconciliation register;
- every still-generated identity is caught or has an approved exception;
- every source-changed identity has a semantic mapping and evidence;
- every new related survivor from the complete run is incorporated;
- every grouped member has mutant-active failure evidence;
- every behavior witness includes replay/provenance ancestry or a justified, reviewed `not applicable`;
- every firewall-relevant family includes a negative/contamination control; and
- the responsible layer is visible in the failure diagnostic.

No family may be declared complete from aggregate cargo-mutants counts alone.

---

## 6. Live re-proof of EPI-01 through EPI-11

The replacement artifact must re-establish every 0040 seam at the exact final implementation commit. The 0040 per-seam rows may be used to identify fixtures, commands, field names, and expected evidence shape, but every reused witness must be re-run, re-fingerprinted, and reclassified as live evidence. A copied command result, historical SHA, archived screenshot, or prior `pass` row remains `historical` and cannot satisfy the replacement verdict.

Each seam subsection below is a deliverable contract. The canonical gate names are composed as cross-references only; the EPI identifiers remain local cross-references inherited from 0040.

### 6.1 EPI-01 — sealed holder-known context construction, scope, identity, hash, and frontier

**Seam to re-prove.** Every actor-visible decision and embodied view is derived from a sealed holder-known context whose holder, viewer, mode, tick, event frontier, projection/schema identity, scope filters, admitted/excluded sources, provenance entries, actor-known fact families, forbidden-truth audit, context ID/hash, and status are fixed before consumption.[^repo-0040-spec]

**Files and consumers.** `epistemics/knowledge_context.rs`, `epistemics/projection.rs`, `agent/actor_known.rs`, `agent/transaction.rs`, proposal construction, embodied view-model construction, and replay rebuild.

**Composed gates.** `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `REPLAY`, `FIXTURE`, and `DIAG` under `EPI-CERT`.

**Required positive evidence.** Re-run context construction and hash/frontier witnesses using `view_filtering_001`, `no_human_epistemic_check_001`, `workplace_assignment_provenance_001`, and the existing hidden-truth/context tests. At least one human-controlled and one autonomous decision must cite the same class of sealed packet and source events.

**Required adversarial/negative evidence.** Another actor's private record, hidden food/route/workplace truth, a stale or forged context tuple, injected counterpart/debug fields, and external attempts to construct or mutate mode/viewer must remain rejected or absent. The registered compile-fail fixtures for debug context construction and mode/viewer mutation must be executed and reported by forbidden capability, not just by matching compiler text.

**Replay/provenance evidence.** Serialize and rebuild the underlying event stream; compare context ID, hash, current tick, frontier event, projection/schema/manifest identities, admitted source-event set, excluded source audit, actor-known fact families, and proposal-relevant fields. Tampering with a cited source event or frontier must fail loudly.

**Mutation coupling.** Belief freshness/linkage and proposition round-trip families must be visible in the context packet or in the evidence that justifies why a field is not part of the packet.

**Typed first-divergence diagnostic.** Actor-known context construction or projection/replay, naming the first differing/missing packet field, source/filter decision, context hash, or frontier witness.

### 6.2 EPI-02 — typed propositions and beliefs, stance, confidence, privacy, freshness, and source integrity

**Seam to re-prove.** Beliefs remain typed, holder-scoped, proposition-bearing, stance-bearing, confidence-bearing, privacy-scoped, source-backed, and freshness-governed. They are not booleans, prose facts, raw world-state mirrors, or freely constructible external records. The explicit prior-belief, incoming-evidence, supersession, and contradiction witnesses are consistent with belief-revision practice: revision evidence must expose what changed and why rather than overwrite the previous epistemic state invisibly.[^belief-revision]

**Files and consumers.** `epistemics/proposition.rs`, `epistemics/belief.rs`, `epistemics/observation.rs`, `epistemics/projection.rs`, `knowledge_context.rs`, content schema/validation, debug entries, notebooks, and embodied view-model adapters.

**Composed gates.** `TFW`, `REPLAY`, `FIXTURE`, `DIAG`, and the first-proof acceptance-contract cross-references.

**Required positive evidence.** Re-run `sound_uncertainty_001`, `aged_food_record_surfaces_as_remembered_belief_not_observation_001`, `partial_food_source_knowledge_001`, and `stale_workplace_notice_superseded_by_newer_001`. Add the section 5 canonical proposition corpus and freshness-boundary family. Evidence must retain proposition type, holder, stance, confidence, privacy, source, observation/contradiction links, stale frontier, and replay identity.

**Required adversarial/negative evidence.** Float confidence types, external literal construction, source/scope mutation, raw projection insertion/read, unknown proposition references, malformed canonical propositions, and hidden truth changes without a belief/observation event must remain rejected or actor-invisible.

**Replay/provenance evidence.** Live application and rebuild must agree on belief IDs, canonical propositions, link sets, privacy, stale policy, confidence, holder, source event, context inclusion, and projection checksum input.

**Mutation coupling.** Sections 5.1, 5.2, 5.8, 5.9, 5.10, 5.11, 5.12, and 5.13 are mandatory EPI-02 mutation witnesses.

**Typed first-divergence diagnostic.** Content/schema validation, actor-known context construction, projection/replay, or view-model rendering, naming the typed belief/proposition/privacy/freshness field and its source record.

### 6.3 EPI-03 — observation channels, event capture, confidence, source references, and projection insertion

**Seam to re-prove.** Observations arise only from declared channels and source-bearing events, preserve observer/place/tick/confidence/privacy/source, and enter the epistemic projection through event application. They cannot be directly forged, inserted externally, or inferred from inaccessible truth.

**Files and consumers.** `epistemics/observation.rs`, `epistemics/projection.rs`, `agent/perception.rs`, `events/apply.rs`, replay rebuild, event schema, content fixtures, and debug/embodied consumers.

**Composed gates.** `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `REPLAY`, `FIXTURE`, and `DIAG`.

**Required positive evidence.** Re-run `no_human_observation_facts_cite_log_events_001`, `sound_uncertainty_001`, and current-place perception cases. Add confidence boundary cases from section 5.5 and `container_contents_observed` canonical round-trip from section 5.9. Every observation must cite its channel, observer, observed place/tick, confidence, source event, schema, and projection application position.

**Required adversarial/negative evidence.** `hidden_food_closed_container_001`, `hidden_route_edge_001`, observation-without-source and raw-insert compile failures, hidden truth perturbation without perception, invalid confidence, and another actor's private observation must not create the focal actor's observation.

**Replay/provenance evidence.** The accepted event stream must reproduce observation IDs, channels, source references, confidence, privacy, actor-known records, context filtering, and checksum. Omitted/duplicated/reordered/corrupted/unsupported events must fail or produce a diagnosed mismatch rather than silent repair.

**Mutation coupling.** Sections 5.2, 5.5, 5.9, 5.12, and 5.13 are mandatory EPI-03 mutation witnesses.

**Typed first-divergence diagnostic.** Action validation, event application, or projection/replay, naming the channel/action, source event, append/application position, confidence or privacy field, and replay difference.

### 6.4 EPI-04 — expectation contradiction, mismatch linkage, and absence-without-culprit discipline

**Seam to re-prove.** A contradiction links a prior expectation belief to a qualifying observation and a typed mismatch. Expected absence is detected only for the correct holder, required stance, exact item/location relation, and legal observation. The system may record absence or contradiction without inventing a culprit, hidden location, or omniscient explanation.

**Files and consumers.** `epistemics/contradiction.rs`, `epistemics/proposition.rs`, `epistemics/belief.rs`, `epistemics/projection.rs`, event application, notebooks/debug entries, view models, and replay.

**Composed gates.** `TFW`, `REPLAY`, `FIXTURE`, and `DIAG`.

**Required positive evidence.** Re-run `expectation_contradiction_001`. Add the holder/stance eligibility matrix, at-place and carried-by relation matrices, and expected-location canonical round-trips from sections 5.3, 5.6, 5.7, and 5.10. The evidence must identify prior belief, supporting observation(s), contradicting observation, expected/observed propositions, holder, source event, detection tick, contradiction ID, activity status, and replay result.

**Required adversarial/negative evidence.** Wrong holder, item, place/container/actor, stance, channel, or missing linked record; an unrelated observation; hidden culprit/location changes; and a closed, unobserved container must not create or attach a contradiction. Each one-condition mismatch in the operator truth tables is mandatory.

**Replay/provenance evidence.** Live and rebuilt contradiction maps, belief link sets, context-visible contradiction sets, canonical checksum inputs, and view-model entries must agree. Link tampering or missing source records must fail closed.

**Mutation coupling.** Sections 5.2, 5.3, 5.4, 5.6, 5.7, 5.8, 5.9, 5.10, and 5.11 are mandatory EPI-04 mutation witnesses.

**Typed first-divergence diagnostic.** Event application, projection/replay, or view-model rendering, naming the prior belief, absence observation, expected and observed propositions, holder/stance filter, link mismatch, or forbidden inference.

### 6.5 EPI-05 — provenance sufficiency, semantic linkage, freshness, and truth-firewall audit

**Seam to re-prove.** Every actor-known fact and proposal-relevant epistemic claim resolves to sufficient, semantically matching, frontier-valid provenance. Provenance cannot be a syntactically present but wrong-kind event, a prose assertion, an inaccessible truth value, a future event, or a debug fact.

**Files and consumers.** `knowledge_context.rs`, `knowledge_basis.rs`, `projection.rs`, proposition reference validation, actor-known context construction, proposal/action validation, fixture/content validation, replay, and diagnostics.

**Composed gates.** `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `REPLAY`, `FIXTURE`, and `DIAG`.

**Required positive evidence.** Re-run `workplace_assignment_provenance_001`, `no_human_known_workplace_requires_provenance_001`, `no_human_workplace_knowledge_requires_notice_event_001`, and `no_human_sleep_knowledge_requires_observation_or_record_001`. Add the observation/contradiction link family and reference-validation/diagnostic families from sections 5.2, 5.12, and 5.13.

**Required adversarial/negative evidence.** Re-run `forbidden_provenance_input_fails_closed_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`, and `prose_born_fact_rejected_001`. Include wrong event kind, wrong actor/holder, wrong subject, future/frontier-exceeding event, unknown reference, missing source, semantic mismatch, and debug-only source cases.

**Replay/provenance evidence.** Resolve each cited witness against the serialized event log and final projection. Record event ID, event kind, holder/actor, subject, tick, frontier relation, schema/manifest identity, and semantic comparison. Rebuild must yield the same accepted/rejected witness set.

**Mutation coupling.** Empty belief-link sets, validation bypasses, and empty/default typed diagnostics must all be tied to a provenance failure that a reviewer can trace.

**Typed first-divergence diagnostic.** Actor-known context construction, proposal construction, action validation, content/schema validation, or projection/replay, naming the unresolved or wrong-kind witness, holder/subject mismatch, frontier, or semantic mismatch and the typed blocker.

### 6.6 EPI-06 — replay-derived epistemic projection, canonical checksum, and non-writer rule

**Seam to re-prove.** `EpistemicProjection` is derived from accepted event history and declared seed/manifest inputs. Live application and deterministic rebuild agree. The projection does not write authoritative truth, repair missing events, or accept direct external epistemic record insertion.

**Files and consumers.** `epistemics/projection.rs`, `events/apply.rs`, `replay/rebuild.rs`, canonical proposition/belief/observation/contradiction serialization, checksums, content manifest/schema identity, and sealed context construction.

**Composed gates.** `PIPE`, `NO-DIRECT`, `REPLAY`, `FIXTURE`, `TFW`, and `DIAG`.

**Required positive evidence.** Re-run projection rebuild/checksum, event schema, golden scenario, and golden fixture suites. Add section 5 round-trips for proposition variants and expected locations, freshness/linkage evidence, and rendering values that participate in evidence packaging. Record live/rebuild projection versions, event ranges, canonical input lines, checksum, context hashes, and relevant view/proposal fingerprints.

**Required adversarial/negative evidence.** Omitted, duplicated, reordered, corrupted, or unsupported events; changed manifest/schema; direct raw insertion; hidden truth change without an event; and parser fallback must be rejected or produce an explicit mismatch. The external raw-map/read and raw-record insertion fixtures remain compile failures.

**Replay/provenance evidence.** This seam's core evidence is the complete serialized input plus deterministic live/rebuild comparison. Every mutation family classified under projection/replay must identify the first canonical record or consumer consequence that diverges.

**Mutation coupling.** Sections 5.1, 5.2, 5.5, 5.6, 5.7, 5.8, 5.9, 5.10, and 5.11 must be represented in the projection/replay evidence package where applicable.

**Typed first-divergence diagnostic.** Event application or projection/replay, naming the first differing event, typed record, link, context field, canonical line, checksum, or forbidden direct-write path.

### 6.7 EPI-07 — actor decision transaction, proposal parity, truth firewall, and validator feedback split

**Seam to re-prove.** Candidate generation, planning/method selection, and proposal construction consume the sealed actor-known context. Truth may validate a proposal at commit time but may not plan, rank, or silently teach the actor. Human and autonomous proposal paths share the same epistemic and validation contract. This matches the partial-observability principle that action selection is based on an information or belief state derived from observation history, not direct access to hidden state.[^pomdp]

**Files and consumers.** `agent/actor_known.rs`, `agent/transaction.rs`, candidate/planner/method code, `actions/proposal.rs`, `actions/pipeline.rs`, action validators, event append/application, and actor-safe feedback.

**Composed gates.** `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `REPLAY`, `FIXTURE`, and `DIAG`.

**Required positive evidence.** Re-run `no_hidden_truth_planning_001`, `knowledge_blocker_accuse_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`, shared human/non-human proposal path tests, and legal-observation reveal cases. `possible_movement_near_place` must round-trip into a legitimate holder-known proposal/view consequence without becoming certainty.

**Required adversarial/negative evidence.** Re-run `hidden_food_unknown_route_001`, `no_human_unseen_workplace_assignment_does_not_plan_work_001`, and typed hidden-input cases. Include believed-open/truth-closed validation: the actor may propose from belief, the validator may reject from truth, and the rejection must not smuggle the hidden truth back into actor knowledge unless an authorized feedback event is appended.

**Replay/provenance evidence.** Record the exact sealed context hash/frontier, candidate set, decision trace, proposal tuple, validator result, feedback event (or absence), and replayed projection. Paired hidden-truth runs must have equal decision/proposal surfaces before legal observation.

**Mutation coupling.** The `possible_movement_near_place` parser survivor and any freshness/provenance mutant that changes proposal reachability must be caught through this real path rather than a parser-only test.

**Typed first-divergence diagnostic.** Candidate generation, planning/method selection, proposal construction, action validation, or event application, naming the actor-known input, decision trace, proposal tuple, validator/feedback split, and first forbidden truth dependency.

### 6.8 EPI-08 — possession parity and cognition-neutral controller binding

**Seam to re-prove.** Possession changes controller binding, not actor cognition, knowledge, intention, event semantics, validation rules, or access to hidden truth. The possessed actor and the same autonomous actor receive equivalent holder-known contexts and legal action surfaces for equivalent histories.

**Files and consumers.** Controller/possession code, sealed context construction, actor decision transaction, proposal/pipeline, view-model selection, TUI input, event application, and replay.

**Composed gates.** `POS-PARITY`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `REPLAY`, `FIXTURE`, and `DIAG`.

**Required positive evidence.** Re-run `possession_parity_001`, `possession_does_not_reset_intention_001`, `debug_attach_001` where relevant, human/non-human shared proposal tests, and same-actor paired runs with and without possession. Compare context identity/hash/frontier, actor-known facts, intentions, candidate/action set, proposal semantics, validator path, accepted events, resulting projection, and embodied view.

**Required adversarial/negative evidence.** Possess actor B after actor A and prove no A-private knowledge transfers; reject unauthorized/mismatched/stale controller bindings; prove no human-only hidden affordance, special parser, validator bypass, or action branch exists; prove debug attachment is independent of possession.

**Replay/provenance evidence.** Both controller modes must produce equivalent semantic event/replay results for the same actor action and history. Differences limited to authorized controller/UI metadata must be declared and excluded from cognition equality with rationale.

**Mutation coupling.** Carried-by contradiction/round-trip evidence must preserve the actor holder rather than conflating controller and actor IDs.

**Typed first-divergence diagnostic.** Actor-known context construction, view-model rendering, proposal construction, action validation, or tests/fixtures, naming the controller-binding delta and the cognition/context/view/event equality field that first diverged.

### 6.9 EPI-09 — embodied projection, notebooks, local actions, why-not, and stale snapshots

**Seam to re-prove.** Embodied surfaces expose only context-permitted actor-known information, with provenance-aware staleness and local action availability. They omit raw assignments, unknown affordances, unobserved contents/routes/food, other actors' private state, and debug truth. They update after legal events, not after invisible truth changes.

**Files and consumers.** `epistemics/projection.rs`, `view_models.rs`, actor-known adapters, `tracewake-tui/src/render.rs`, notebook/why-not/action surfaces, TUI session/transcript code, and replay.

**Composed gates.** `TFW`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`, and `NO-HUMAN` where the same surface is generated without a controller.

**Required positive evidence.** Re-run `view_model_local_actions_001`, `embodied_exits_require_perceived_or_known_route_001`, `embodied_menu_lags_truth_change_without_perception_001`, `embodied_workplace_availability_reflects_belief_not_truth_001`, `view_filtering_001`, and the existing TUI seam/transcript/embodied-flow suites. Include proposition/expected-location rendering and confidence/freshness consequences from section 5 where they reach the view.

**Required adversarial/negative evidence.** Re-run `embodied_view_omits_raw_assignment_without_context_001`, `embodied_view_omits_unknown_sleep_affordance_001`, and `embodied_view_omits_unobserved_food_at_open_place_001`. Add other-actor private records, hidden route, hidden container contents, raw world-state assignment, stale context, and debug token/field injection.

**Replay/provenance evidence.** The view must name the sealed context/projection fingerprint from which it was built. Live and replayed views must agree; truth-only changes must not alter them, while a legal observation/notice event may produce an event-explained update.

**Mutation coupling.** `possible_movement_near_place`, proposition `Display`, `render_location`, confidence, and belief freshness/linkage survivors must be caught here when the production surface consumes them.

**Typed first-divergence diagnostic.** Projection/replay, view-model rendering, or debug quarantine, naming the leaked/omitted field, stale-context mismatch, source record, or actor-safe/debug provenance split.

### 6.10 EPI-10 — debug capability, report/view separation, and quarantine

**Seam to re-prove.** Debug truth is capability-gated, visibly marked, separate from embodied views, non-authoritative for actor cognition, and unable to change simulation state, event history, projection, context, proposal, planner choice, or validator feedback.

**Files and consumers.** `debug_capability.rs`, `debug_reports.rs`, `epistemics/projection.rs` debug views, `view_models.rs`, TUI debug panels/rendering, controller/session code, and compile-fail fixtures.

**Composed gates.** `TFW`, `POS-PARITY`, `NO-DIRECT`, `REPLAY`, `FIXTURE`, and `DIAG`.

**Required positive evidence.** Re-run `debug_attach_001` and authorized debug-panel/report/view tests. Verify explicit debug marker/capability path, typed debug projection/report construction, and useful provenance-rich output.

**Required adversarial/negative evidence.** Re-run `debug_omniscience_excluded_001`; external debug context/projection/report construction compile failures; debug capability minting/literal failures; debug-token injection; debug attach/detach equality over state, events, projection, context, proposals, and actor-visible embodied output.

**Replay/provenance evidence.** Attaching or rendering debug must append no simulation event and must leave replay/checksum/context/proposal fingerprints unchanged. Any debug-only richer rendering must cite the authorized debug channel and remain absent from embodied transcripts.

**Mutation coupling.** Formatter/rendering fixes must not solve a survivor by copying debug truth into an embodied message. Debug may inspect a typed proposition or link chain but may not be the source of the actor-known record.

**Typed first-divergence diagnostic.** Debug quarantine, view-model rendering, or event application, naming the capability path, leaked token/channel, or first state/event/projection/context/proposal delta.

### 6.11 EPI-11 — relational anti-contamination and possession-parity capstone

**Seam to re-prove.** EPI-11 establishes non-interference as a paired-run relation: two executions with equal focal-actor observable histories but controlled differences in hidden truth, other-actor private state, controller binding, or debug attachment must remain equal over the declared actor-visible/decision-producing domain until a legal event explains divergence.[^noninterference]

**Composed gates.** All EPI-relevant canonical gates, with the first responsible layer reported on failure. `EMERGE-OBS`, when present beside a living-world fixture, remains observer-only and outside the equality verdict.

**Required paired-run matrix.** At minimum:

| Pair | Controlled hidden difference | Equality required before legal reveal | Permitted divergence |
|---|---|---|---|
| Closed container | item present versus absent in unobserved closed container | focal context, beliefs/observations/contradictions, proposals, embodied view, accepted events | after an authorized contents-observed/search event |
| Hidden route | route exists versus does not exist but is unobserved | route candidates, movement proposals, embodied exits, why-not | after perception/notice provides route provenance |
| Workplace truth | access truth open versus closed with identical actor belief | decision/proposal and embodied availability | validator outcome may differ; actor knowledge changes only through authorized feedback/notice |
| Other-actor privacy | private record differs for actor B | actor A context, view, proposals, notebook, checksum slice | after a legal communication/observation event to actor A |
| Debug | debug detached versus attached | state, events, projection, sealed context, proposal, embodied view | debug channel only; no simulation divergence |
| Possession | same actor autonomous versus human-controlled | cognition/context/action rules and semantic outcome | authorized controller/UI metadata only |
| Freshness | hidden truth changes after last observation versus remains stable | actor-known classification according to recorded tick/frontier, not truth | after a new legal observation/notice event |

For each pair, retain:

- exact differing hidden input;
- proof that focal actor-observable event histories are equal before reveal;
- exact compared fields and excluded nonsemantic metadata;
- before-reveal comparison result;
- legal reveal event and source provenance;
- after-reveal comparison result;
- live/replay comparison for both executions;
- generated seed/sample declaration when a property harness is used; and
- first responsible layer if the relation fails.

A snapshot pair is insufficient without typed field comparison. A property-based harness may expand the domain, but the acceptance package must retain concrete minimized counterexamples for any failure and at least one deterministic witness for each required pair. Relational non-interference testing through paired executions is an established practical technique for detecting illicit information flow.[^noninterference]

### 6.12 Compile-fail and external API boundary corpus

The complete `negative_fixture_runner` command must be recorded, together with a case-by-case matrix for at least:

- `banned_float_confidence_types`;
- `external_crate_cannot_build_debug_knowledge_context`;
- `external_crate_cannot_build_debug_projection_view_without_core_debug_api`;
- `external_crate_cannot_construct_belief_literal`;
- `external_crate_cannot_construct_debug_report`;
- `external_crate_cannot_construct_observation_without_source`;
- debug capability construction/minting compile-fail doctests;
- `external_crate_cannot_insert_raw_epistemic_records`;
- `external_crate_cannot_mutate_belief_source_or_scope`;
- `external_crate_cannot_mutate_contradiction_links`;
- `external_crate_cannot_mutate_knowledge_context_mode`;
- `external_crate_cannot_mutate_knowledge_context_viewer`; and
- `external_crate_cannot_read_raw_epistemic_projection_maps`.

Each row must name the forbidden capability, compiler failure class, relevant EPI seam, canonical gate cross-reference, and why the rejection closes an external API path rather than merely matching transient compiler wording.

### 6.13 Reuse declaration

The implementing session must include a reuse table with these columns:

| 0040 witness or artifact | Replacement use | Required action | Final evidence status |
|---|---|---|---|
| Prior seam definition/fixture name | structural context | verify against current doctrine and final code; re-run | live `pass`/`fail`, never inherited |
| Prior command transcript | historical comparison | run the command anew and fingerprint new output | old row `historical`; new row live |
| Prior replay/provenance package | test-data/evidence-shape reference | regenerate from final exact commit | old row `historical`; new row live |
| Prior mutation register | seed inventory | reconcile every identity and append all new survivors | historical seed plus live final register |
| Prior `EPI-CERT scoped remediation` verdict | superseded state | cite as the blocker being closed | `historical` after replacement passes |

Any row whose final action is “reuse unchanged” cannot certify the replacement.

---

## 7. Gate-evidence, diagnostics, and review package

### 7.1 Mandatory gate-evidence requirements

The replacement package must instantiate every element of execution document `03`'s “Gate evidence requirements” list.[^repo-ladder]

| Required element | 0041 implementation obligation |
|---|---|
| Exact files and seams audited | Enumerate every changed file; every final file in the configured mutation census; the four historical survivor-bearing files; the surrounding EPI source modules; every test/fixture/CI/config file used; and the EPI-01…EPI-11 production paths exercised. File paths must be tied to an exact final commit, not a branch. |
| Foundation and architecture dependencies | Cite the controlling documents in sections 2.3 and 2.4 and map each EPI seam/mutation family to the applicable invariant/architecture rule. Any conflict blocks implementation and must be escalated as an upstream doctrine issue rather than silently resolved in code. |
| Artifact dependencies beside gates | Identify source manifest/schema, serialized event logs, replay outputs, projection/context checksums, fixture IDs, compile-fail transcripts, mutation structured output, triage register, command transcripts, and any observer-only `EMERGE-OBS` artifact used beside first-proof living-world fixtures. |
| Positive and negative fixtures | For every EPI row, cite at least one positive and one adversarial/negative witness, including hidden-truth, contradiction, invalid-reference/prose-born-fact, possession-parity-violation, and debug-quarantine controls. |
| Event/replay/projection evidence | Retain accepted event sequences or event IDs, replay command/input, projection version/schema/manifest, live/rebuild comparison, context/projection fingerprints, and first divergence for failures. |
| Actor-known provenance evidence | Resolve actor-known claims to typed source events within the context frontier; record holder, source kind, subject, tick, privacy/scope, semantic match, and rejection of wrong-kind/hidden/debug/prose sources. |
| Debug-quarantine evidence | Show authorized debug reachability and explicit marker while proving no change to state, event log, projection, context, proposal, actor cognition, or embodied output. Include external-construction compile failures. |
| Failure diagnostics by responsible layer | Use section 7.3 and the per-EPI/per-mutant diagnostic maps. Every failed or exceptional row names first responsible layer and retains the smallest useful causal package. |
| Archived specs/tickets historical only | State verbatim in the artifact that archived specs/tickets `0004`–`0040` and their command/results are historical context or structural precedent only, not live certification. |
| Tolerated deferrals tied to named gates | Use section 11. No EPI survivor, EPI seam, replay/provenance, possession-parity, or debug-quarantine obligation may be deferred. All tolerated future scope is tied to its later named entry gate. |

### 7.2 Canonical gate composition matrix

This matrix does not redefine the gates. It tells the implementing session which existing gate evidence must appear in the EPI replacement package.

| Existing gate cross-reference | Required EPI use |
|---|---|
| `TFW` | Hidden truth, other-actor privacy, validator truth, and debug truth cannot enter holder-known contexts, planning, proposals, or embodied views without an authorized information event. Paired-run equality and negative fixtures are mandatory. |
| `PIPE` | Actor proposals, validation, accepted events, event application, and epistemic projection update follow the standing pipeline. Parser/provenance fixes cannot introduce a direct side channel. |
| `NO-DIRECT` | No raw epistemic insertion, direct authoritative mutation, direct projection write, test-only bypass, or alternate accepted-action path is added to kill a mutant. Compile-fail and source/runtime guards remain live. |
| `NO-HUMAN` | EPI contexts, observations, beliefs, proposals, diagnostics, and replay evidence remain available for autonomous agents; a human controller is not required to make a witness pass. |
| `POS-PARITY` | Controller binding does not change actor cognition, holder-known evidence, action rules, or semantic outcomes. Possession is explicitly paired in EPI-08/EPI-11. |
| `REPLAY` | All event-derived EPI claims are reproduced from serialized event input with declared schema/manifest and deterministic projection/context/view/proposal evidence. |
| `FIXTURE` | Positive and adversarial fixture corpus covers hidden truth, contradiction, possession parity, malformed/unknown proposition references, debug quarantine, and living-world cases. |
| `DIAG` | Failures identify responsible layer, typed records, source IDs/frontier, mutant identity, and first divergence. Evidence-status and fingerprint-scope honesty are reviewable. |
| `EPI-CERT` | Phase-certification label composing the above evidence. It is not a new gate and does not pass until the replacement artifact's aggregate rule is satisfied. |
| Execution `02` acceptance-contract labels | First-proof scope, replay/provenance, anti-contamination, fixture, and review-artifact expectations continue to constrain the replacement evidence. |
| `EMERGE-OBS` | Observer-only companion evidence only where the corpus exercises first-proof living-world acceptance; never a threshold, gate, score, scenario goal, or substitute for an EPI failure. |

### 7.3 Responsible-layer diagnostic map

Use the existing layer vocabulary from execution document `03`. A test name, fixture name, or file name is not a responsible layer.

| Evidence area | First responsible layers to consider | Minimum failure package |
|---|---|---|
| EPI-01 | actor-known context construction; proposal construction; projection/replay | context packet, differing/missing field, ID/hash/frontier, source/filter decision, source event, final commit |
| EPI-02 | content/schema validation; actor-known context construction; projection/replay; view-model rendering | typed belief/proposition/privacy/freshness/confidence diff, holder, source/link records |
| EPI-03 | action validation; event application; projection/replay | channel/action, source event, observation ID, append/application position, confidence/privacy, replay diff |
| EPI-04 | event application; projection/replay; view-model rendering | prior belief, expected/observed proposition, absence observation, link/holder/stance mismatch, forbidden inference |
| EPI-05 | actor-known context construction; proposal construction; action validation; content/schema validation; projection/replay | unresolved/wrong-kind provenance, holder/subject/frontier/semantic mismatch, typed blocker |
| EPI-06 | event application; projection/replay | event/log input, first live/rebuild record or checksum divergence, omitted/duplicated/direct-write path |
| EPI-07 | candidate generation; planning/method selection; proposal construction; action validation; event application | actor-known input, decision trace, proposal tuple, validator truth input, feedback event/absence |
| EPI-08 | actor-known context construction; view-model rendering; proposal construction; action validation; tests/fixtures | controller delta, actor identity, context/view/action/event equality domain, first semantic divergence |
| EPI-09 | projection/replay; view-model rendering; debug quarantine | leaked/omitted field, stale context, source record, actor-safe/debug split, transcript/view fingerprint |
| EPI-10 | debug quarantine; view-model rendering; event application | capability path, leaked token/channel, state/event/projection/context/proposal before/after comparison |
| EPI-11 | first responsible layer encountered | paired inputs, equality precondition, compared field set, before/after reveal result, replay for both runs |
| Mutation | layer owning the certified consequence | exact mutant diff/identity, unmutated pass, mutant-active failure or exception proof, test parameter/case, disposition/signoff |
| Configuration/CI | tests/fixtures; documentation status; workspace/CI review surface | config/list fingerprints, trigger evaluation, command arguments, denominator reconciliation, output retention |
| Artifact honesty | documentation status; tests/fixtures | evidence-status derivation, fingerprint scope, pending/historical/observer-only exclusion from pass, claim-to-evidence trace |

A failed gate must produce remediation evidence naming the failing layer. It may not be converted to a “known limitation,” baseline miss, sampled pass, or later-phase deferral.

### 7.4 Evidence-status and fingerprint-scope controls

Every evidence item must use only the statuses already allowed by the acceptance template and execution doctrine: `pass`, `fail`, `pending`, `sampled`, `observer-only`, or `historical`. The artifact computes a row result only from certifying evidence at the exact final implementation commit.

Controls:

- `historical` evidence may establish lineage or explain why a test exists; it cannot pass a seam.
- `sampled` property/generative evidence may supplement a finite required matrix; it cannot replace named exhaustive cases or member-level mutant catches.
- `observer-only` evidence—including `EMERGE-OBS`—may inform review but cannot change a verdict.
- `pending` evidence names the missing proof and owner/blocker; a required row with pending evidence does not pass.
- a checksum, snapshot, or fingerprint proves only the bytes/fields in its declared scope; it is not a semantic pass by itself.
- a cargo-mutants `caught` outcome proves only that some test failed under that mutant; the associated evidence item must identify the certified behavior and rule out accidental/flaky failure.
- an aggregate mutation score is descriptive only; it cannot satisfy a survivor row or seam.

### 7.5 Claim–argument–evidence packaging

The replacement artifact should organize each aggregate claim as a reviewable claim–argument–evidence chain:

- **claim:** the exact EPI seam or mutation disposition being asserted;
- **argument:** why the cited positive, adversarial, replay/provenance, and mutation evidence collectively establishes the claim under upstream doctrine;
- **evidence:** exact command transcript, typed behavior witness, event/replay package, structured mutation output, fingerprint scope, and review signoff.

This organization does not introduce a new gate or proof formalism. It makes omissions, scope jumps, and historical-evidence laundering visible. Structured assurance-case practice similarly separates claims, arguments, and supporting evidence rather than treating a test list as self-justifying.[^assurance-case]

### 7.6 Provenance-chain packaging

Where a claim crosses from an event or source artifact into an actor-known proposition, belief, observation, contradiction, context, proposal, or view, the evidence must preserve a witness chain with stable identities for:

1. source entity/artifact or seed/manifest input;
2. generating or transforming activity (event append/application, replay, projection extraction, context build, view/proposal build);
3. resulting typed entity/record;
4. holder/actor and privacy/scope;
5. temporal frontier and source tick;
6. semantic relation between source and claim; and
7. final consumer/behavior consequence.

The W3C PROV data model's entity–activity–agent distinctions provide a useful external vocabulary for packaging derivation and attribution, but repository-native event IDs, typed source references, holders, and frontiers remain authoritative.[^prov-dm]

---

## 8. Replacement acceptance and evidence artifact specification

### 8.1 Artifact identity and purpose

The implementing session must produce one replacement EPI-CERT acceptance artifact in the numbered-report convention. A convention-compatible candidate is:

```text
reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md
```

with archive placement under `archive/reports/` at accepted closeout. The exact final filename is an owner decision and must be fixed before evidence generation so all transcripts, registers, and ledger updates point to one path. Regardless of filename, the artifact must:

- identify this 0041 spec;
- identify `7a17447d…` as the source baseline and one exact final implementation commit as the commit under test;
- state that latest `main` was not independently verified;
- render no pass before execution completes;
- supersede the 0040 EPI artifact only if its aggregate rule passes; and
- preserve 0040 as historical lineage rather than deleting or rewriting it.

### 8.2 Header and environment block

The artifact must record:

- repository owner/name;
- source baseline SHA;
- exact final implementation SHA;
- branch/PR only as non-authoritative context, if recorded at all;
- clean-worktree or enumerated-unrelated-change statement;
- date/time and execution environment;
- operating system/architecture where relevant;
- `rustc`, Cargo, cargo-mutants, and other evidence-tool versions;
- `Cargo.lock`, `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, CI workflow, manifest/schema, and source census fingerprints;
- exact changed-file list for the remediation;
- evidence-root/report directory; and
- limitation statement that the artifact certifies only its exact final commit and EPI scope.

### 8.3 Command ledger

Record every preflight, named suite, list/census, full mutation shard/run, timeout retry, replay/golden fixture command, and evidence-validation command with:

- exact command line and working directory;
- relevant environment variables/options;
- start/end or elapsed time when retained by the execution environment;
- exit code and semantic result;
- transcript/output paths and fingerprints;
- exact final commit;
- status classification and certification use; and
- responsible layer for failures.

A filtered development run must be labeled non-certifying. A complete-run row must identify whether it is unsharded or list every shard and reconciliation artifact.

### 8.4 Evidence-item ledger fields

For every cited evidence item, instantiate all fields required by `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:[^repo-accept-template]

- evidence item ID;
- EPI cross-references or mutation/artifact-completeness cross-reference;
- `evidence-status`;
- `fingerprint-scope`;
- evidence summary;
- path under test and `behavior-witness`, including command/trigger, responsible layer, accepted/rejected stage, live negative or reason none applies, and supported facts/invariants;
- `replay/provenance` ancestry, including event/log range, serialized input, seed/randomness, manifest/schema/ruleset, projection/extraction version, and source provenance;
- `sampling/exhaustiveness` scope;
- `pending/historical` handling;
- `certification-use`; and
- `staged-abstraction` declaration where applicable.

Local evidence IDs are artifact navigation, not doctrine-level finding or gate IDs.

### 8.5 Per-seam verdict matrix

The artifact must contain a table at least this expressive:

| Requirement | Responsible layer(s) | Positive evidence | Adversarial/negative evidence | Replay/provenance evidence | Mutation evidence | Result from certifying evidence |
|---|---|---|---|---|---|---|
| EPI-01 sealed context identity/scope/hash/frontier | actor-known context construction; proposal construction; projection/replay | IDs | IDs | IDs | linked survivor families | pass/fail/pending derived from live evidence |
| EPI-02 beliefs/privacy/freshness | content/schema validation; context; projection/replay; view-model | IDs | IDs | IDs | IDs | derived result |
| EPI-03 observation channels/event capture | action validation; event application; projection/replay | IDs | IDs | IDs | IDs | derived result |
| EPI-04 contradiction/absence discipline | event application; projection/replay; view-model | IDs | IDs | IDs | IDs | derived result |
| EPI-05 provenance/witness sufficiency | context; proposal/action/content validation; projection/replay | IDs | IDs | IDs | IDs | derived result |
| EPI-06 projection rebuild/non-writer | event application; projection/replay | IDs | IDs | IDs | IDs | derived result |
| EPI-07 decision/proposal parity/truth firewall | candidate/planning/proposal/action validation | IDs | IDs | IDs | IDs | derived result |
| EPI-08 possession parity | context; view-model; proposal/action validation; tests/fixtures | IDs | IDs | IDs | IDs | derived result |
| EPI-09 embodied view/notebook/why-not | projection/replay; view-model; debug quarantine | IDs | IDs | IDs | IDs | derived result |
| EPI-10 debug quarantine | debug quarantine; view-model; event application | IDs | IDs | IDs | linked rendering families | derived result |
| EPI-11 relational capstone | first responsible layer | IDs | IDs | paired-run IDs | linked families | derived result |
| Configured EPI mutation posture | layer by survivor | baseline/list/full-run IDs | no-shrink/no-laundering IDs | structured output/register | full disposition matrix | derived result |
| Artifact/evidence honesty | documentation status; tests/fixtures | template/conformance IDs | historical/sampled/observer-only controls | fingerprints/claim map | register reconciliation | derived result |

Every row must cite live evidence at the final commit. Mutation evidence strengthens the seam it exercises; a caught mutant does not automatically pass a seam whose positive, negative, or replay evidence is missing.

### 8.6 Completed-mutation `pass` row

The mutation row may be `pass` only when it states and proves all of the following:

**Configured denominator and completion**

- exact `.cargo/mutants.toml` fingerprint;
- effective `--list-files` and `--list` output/fingerprints;
- presence of all four historical survivor-bearing files and the standing EPI seam;
- no excluded or silently skipped required EPI file;
- exact cargo-mutants command(s), version, jobs/shards, timeout policy, and final commit;
- every shard/run completed and reconciled; and
- zero unresolved tool failures or timeouts.

**Tool outcome counts**

- total generated/tested;
- caught;
- missed;
- timeout;
- unviable;
- baseline/build/tool failures; and
- reconciliation equation explaining any tool-version category differences.

**Certification disposition counts**

- historical seed identities: `30`;
- historical identities still generated at the final source;
- historical identities legitimately source-changed and mapped;
- historical identities killed by remediation;
- final-run caught identities relevant to the seed/new floor;
- approved equivalent identities;
- approved non-critical identities;
- additional survivors first exposed by the completed run;
- additional survivors killed by remediation;
- blocked/untriaged identities: `0` (`zero untriaged`);
- exception identities lacking independent signoff: `0`; and
- unresolved historical identities: `0`.

Tool outcomes and certification dispositions must not be added as if they were one partition. For example, an approved equivalent mutant remains a final tool outcome of `missed` but a reviewed certification disposition of equivalent; it must not be counted as caught. “Killed by remediation” is the historical/reconciliation subset for which the final complete run or exact member evidence reports caught after the remediation delta.

**Member-level evidence**

- each of the 30 historical identities has a reconciliation row;
- each final miss/timeout has a reviewed disposition row;
- each grouped test family proves every member;
- each exception names exact call sites/domain and independent signoff;
- the baseline-miss file contains no laundered EPI survivor; and
- the final register is fingerprinted and linked.

A high percentage or zero raw misses is not required as a separate doctrine threshold. What is required is run completion, honest counts, and zero unresolved/untriaged behavior-relevant outcomes. Kill-with-coverage remains the default and exceptions remain rare.

### 8.7 Relational, replay, provenance, and debug packages

The artifact must include or link:

- the EPI-11 paired-run matrix and concrete inputs/results;
- event logs or event-ID ranges for each event-derived seam;
- live/rebuild projection/context/view/proposal comparisons;
- actor-known provenance resolution tables;
- compile-fail capability matrix;
- debug attach/detach no-effect comparison;
- possession on/off same-actor comparison;
- source/manifest/schema/tool fingerprints; and
- first-divergence packages for every failed or retried witness.

### 8.8 `EMERGE-OBS` handling

When a reused first-proof living-world fixture produces emergence evidence, place it in a separate companion section with `Evidence status: observer-only`. Record the corpus, observation, extraction method, and fingerprint scope. State explicitly that it:

- does not pass or fail any EPI seam;
- does not contribute to mutation disposition;
- does not define a score or threshold;
- does not become a scheduler objective or scenario goal; and
- does not compensate for missing replay/provenance, negative, possession, or debug evidence.

### 8.9 Staged-abstraction declaration

If the artifact relies on any bounded staged abstraction, declare:

- what EPI behavior is proven now;
- what future surface is deliberately abstracted;
- what the implementation/proof must not fake;
- which future named gate owns the abstracted surface;
- what evidence prevents overclaiming; and
- diagnostics distinguishing not implemented, intentionally abstracted, implemented but broken, and overclaimed.

No staged abstraction may cover a survivor disposition, EPI-01…EPI-11 requirement, replay/provenance chain, holder-known context, possession parity, embodied/debug separation, or debug quarantine. If no abstraction is used, state `none` rather than omitting the section.

### 8.10 Aggregate verdict and supersession rule

The replacement artifact may render exactly:

```text
EPI-CERT passed
```

only when:

1. all EPI-01…EPI-11 rows pass from certifying evidence at the exact final implementation commit;
2. the complete configured mutation posture has run and reconciled;
3. all historical and newly exposed survivors have final dispositions;
4. zero identities remain blocked/untriaged, zero exceptions lack required signoff, and zero timeouts/tool failures remain unresolved;
5. the no-shrink, no-silent-exclusion, no-baseline-laundering, CI-trigger, replay/provenance, possession-parity, and debug-quarantine controls pass;
6. all required evidence-template fields are complete and honestly scoped; and
7. there is no upstream doctrine conflict or unresolved EPI failure.

The passing artifact must state that it **supersedes**:

```text
archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md
```

for current EPI-CERT certification use. The 0040 artifact remains historical evidence of the prior audit and 30-mutant blocker. After supersession, later specs may cite `EPI-CERT passed` at the exact replacement commit and the phase ladder may proceed to `ORD-LIFE-CERT` under its own separate spec and evidence.

If any aggregate condition is not met, the artifact must instead remain:

```text
EPI-CERT scoped remediation
```

and name the failing requirement, first responsible layer, retained evidence, and next remediation action. It must not use `passed with exceptions`, a score-based pass, a partial pass, or any newly invented status.

### 8.11 Closeout and ledger handling

On accepted closeout:

- archive this 0041 spec and the replacement report under their numbered archive paths;
- update the live spec ledger according to the repository's closeout convention;
- record exact final commit and artifact fingerprints;
- retain all triage/mutation/replay/command evidence at stable report paths;
- preserve 0040 and earlier artifacts as historical lineage; and
- state the newly admissible next ladder move without beginning or certifying it in the 0041 artifact itself.

---

## 9. Preliminary static survey — preliminary, not certification

This section records authoring-time hypotheses from exact-commit source reading. It is informative only. It does not prove a mutant killed, equivalent, non-critical, or passed, and it must not be copied into the replacement artifact as live execution evidence.

### 9.1 Standing perimeter and CI

At `7a17447d…`, the checked-in mutation configuration includes `crates/tracewake-core/src/epistemics/**`; no exclusion glob removes the four survivor files; and the baseline-miss file is empty. The scheduled mutation command consumes that full configured posture. The in-diff trigger, however, names only `knowledge_context.rs` and `projection.rs`, so direct changes to the four survivor-bearing files are not guaranteed to launch the change-scoped job. This appears to require the minimal trigger correction specified in section 3.4.[^repo-mutants-config][^repo-baseline-misses][^repo-ci]

### 9.2 Proposition parse, deserialize, render, and validation clusters

The three deleted top-level parse arms and two deleted location arms are likely killable through a canonical `serialize -> deserialize -> replay/apply -> project` corpus because the typed variants already have canonical encodings and downstream EPI meanings. The `Display`, `render_location`, and typed-error display survivors are likely killable through real diagnostic/view/transcript consequences, provided the tests pair rendered meaning with typed records and provenance rather than checking only non-empty strings. The three `Ok(())` validation survivors are likely killable through existing content/schema rejection paths with unknown-reference fixtures.[^repo-proposition][^repo-content-validate]

### 9.3 Belief freshness and witness-link clusters

The belief type exposes a stale frontier plus observation and contradiction ID sets, but the baseline constructor/draft leaves the stale frontier at `None`, and the inspected projection's actor-known freshness classifier is a separate record-level policy. The inspected projection checksum and `DebugBeliefEntry` also omit the observation/contradiction link sets. The four survivors therefore identify likely contract/reachability gaps, not automatically killable getter cases. The implementing session must enumerate final constructibility and consumers, then either establish a doctrine-backed replay/provenance consequence or reconcile/remove a dead API; a test-only reader is forbidden.[^repo-belief][^repo-projection][^repo-view-models]

### 9.4 Contradiction operator clusters

The at-place and carried-by relations are exact two-key conjunctions, making the complete four-row equality matrices strong, non-tautological killing families. The `detect_expected_absences` eligibility condition similarly needs both one-condition mismatch controls to distinguish `||` from `&&`. These survivors appear individually killable through typed contradiction creation and replay linkage, not merely through direct boolean tests.[^repo-proposition][^repo-contradiction]

### 9.5 Observation confidence cluster

`Confidence` is bounded and canonically serialized, but the baseline serializer reads the private numeric field directly; the inspected projection checksum and debug entries therefore bypass both `parts_per_thousand` and `is_low`. These three survivors cannot be presumed killable through checksum evidence. They require a final call-site census and either a doctrine-backed production consumer with `350`/`351` and nontrivial-value consequences, or a reviewed dead-API/equivalence disposition. A direct getter/classifier assertion remains non-certifying.[^repo-observation][^repo-projection][^repo-view-models]

### 9.6 `ContradictionKind::is_active -> true`

At the authoring baseline, the visible enum domain appears to contain one kind and that kind is active. The mutant therefore looks plausibly equivalent today. That is only a hypothesis. The final implementation must enumerate the complete final domain and every reachable call site, then either find a distinguishing behavior or produce the section 4.11 proof and independent signoff. No new inactive kind should be invented solely to improve the mutation outcome.[^repo-contradiction]

---

## 10. Constitutional invariant alignment

This spec cites existing invariant IDs as cross-references and creates none. The implementing session must verify the exact final text of every cited invariant and record any genuine conflict as an upstream doctrine blocker.

| Remediation/re-proof area | Primary invariant cross-references | Required preservation |
|---|---|---|
| Event authority and replay ancestry | `INV-001`, `INV-009`–`INV-020`, `INV-092` | EPI records and changes arise from modeled causes/events; absence requires a channel; replay reconstructs significant consequences; unsupported/corrupt history fails loudly. |
| Typed propositions, observations, beliefs, contradictions, and records | `INV-002`, `INV-021`–`INV-031` | belief precedes truth; epistemic currency is typed; prose is non-authoritative; truth/belief/records remain distinct; no telepathy; wrong/stale belief remains possible; important belief has provenance; evidence is not truth; debug/human notes are non-diegetic. |
| Freshness and holder-known time | `INV-026`, `INV-028`, `INV-102`, `INV-112` | source, acquisition/freshness/frontier data remain attached; truth or scheduler time does not silently refresh actor knowledge. |
| Observation and expected-absence detection | `INV-016`, `INV-021`, `INV-024`, `INV-026`, `INV-030`, `INV-102` | absence becomes evidence only through expectation/perception/search; observation and contradiction links are typed and source-backed; no culprit/location is inferred from truth. |
| Sealed actor-known context and truth firewall | `INV-002`, `INV-022`, `INV-024`, `INV-030`, `INV-099`–`INV-107`, `INV-112` | truth may validate but not plan; hidden-truth cognition is forbidden; context is sealed; cognition inputs carry provenance; validation failure does not leak; debug remains quarantined. |
| Actor planning/proposal/validation path | `INV-032`–`INV-043`, `INV-099`–`INV-106` | symbolic inspectable decisions consume beliefs and bounded methods; needs/routines/scheduler do not dispatch directly; decisions have typed traces; ordinary-agent validation is shared. |
| No-human and possession parity | `INV-004`–`INV-008`, `INV-043`, `INV-091`, `INV-094`, `INV-108` | the world runs without a human; possession changes input binding only; no knowledge transfer or player privilege; semantic action path remains shared. |
| Embodied TUI/view models | `INV-006`–`INV-008`, `INV-065`–`INV-071`, `INV-093`, `INV-095`, `INV-098`, `INV-105` | embodied mode shows actor-known reality; TUI does not own rules; why-not is actor-known; leakage is high severity; view-model reachability and diagnostics are acceptance evidence. |
| Debug capability and quarantine | `INV-031`, `INV-041`, `INV-067`–`INV-070`, `INV-093`, `INV-095`, `INV-105`, `INV-107` | debug is useful, visibly non-diegetic, typed, and unable to feed actor knowledge or simulation behavior. |
| Provenance and evidence packaging | `INV-011`, `INV-013`, `INV-018`–`INV-020`, `INV-026`, `INV-041`, `INV-102`, `INV-105` | causal ancestry, schema/version, typed source links, replay, and inspectable diagnostics survive into the review artifact. |
| Mutation lock layer and harsh acceptance | `INV-091`–`INV-095`, `INV-098`, `INV-105` | no-human, replay, leakage, possession, view-model, and diagnostic behavior remain regression-tested; metric improvement cannot replace behavior evidence. |
| Observer-only emergence evidence | `INV-111` | replayable living-world observations may accompany relevant fixtures but remain retrospective and unable to influence behavior or certification thresholds. |

### 10.1 Per-EPI alignment summary

- **EPI-01** chiefly protects `INV-002`, `INV-021`–`INV-026`, `INV-030`, `INV-099`–`INV-102`, and `INV-112`.
- **EPI-02** chiefly protects `INV-002`, `INV-021`–`INV-031`, `INV-102`, and `INV-112`.
- **EPI-03** chiefly protects `INV-009`, `INV-012`, `INV-013`, `INV-016`, `INV-018`, `INV-021`, `INV-024`, `INV-026`, and `INV-102`.
- **EPI-04** chiefly protects `INV-016`, `INV-021`, `INV-023`–`INV-026`, `INV-030`, and `INV-102`.
- **EPI-05** chiefly protects `INV-009`–`INV-020`, `INV-021`–`INV-031`, `INV-099`–`INV-102`, `INV-105`–`INV-107`, and `INV-112`.
- **EPI-06** chiefly protects `INV-009`–`INV-020`, `INV-092`, `INV-102`, and `INV-105`.
- **EPI-07** chiefly protects `INV-002`, `INV-004`, `INV-006`–`INV-008`, `INV-032`–`INV-043`, and `INV-099`–`INV-106`.
- **EPI-08** chiefly protects `INV-004`–`INV-008`, `INV-043`, `INV-067`, `INV-069`, `INV-091`, `INV-094`, and `INV-108`.
- **EPI-09** chiefly protects `INV-006`–`INV-008`, `INV-065`–`INV-071`, `INV-093`, `INV-095`, `INV-098`, and `INV-105`.
- **EPI-10** chiefly protects `INV-031`, `INV-041`, `INV-067`–`INV-070`, `INV-093`, `INV-095`, `INV-105`, and `INV-107`.
- **EPI-11** chiefly protects `INV-002`, `INV-024`, `INV-093`, `INV-094`, and `INV-099`–`INV-108` as relational properties.

---

## 11. Out of scope and tolerated deferrals

### 11.1 Settled predecessor scope

`SPINE-CERT passed` is consumed from the 0039 replacement artifact. Event log, replay, pipeline, append-before-apply, and no-direct-dispatch behavior may be exercised as EPI ancestry, but their certification line is not reopened. A shared-file change that affects a settled SPINE contract must still pass existing regression gates; that regression result does not turn 0041 into a SPINE recertification.

`P0-CERT passed` is likewise consumed from 0037 and not reopened.

### 11.2 Later ladder gates

The following are tolerated only as explicit deferrals to their named entry gates:

| Deferred surface | Owning future gate/posture | 0041 rule |
|---|---|---|
| Ordinary-life needs, routines, work/sleep/food expansion | `ORD-LIFE-CERT` | blocked until the 0041 replacement renders `EPI-CERT passed`; no ordinary-life audit or feature expansion here |
| Whole first-proof aggregate | `FIRST-PROOF-CERT` | this spec supplies prerequisite EPI evidence only; it does not aggregate or pass first proof |
| Institutions, household/institution records/procedures, wrong suspicion, Phase-4 entry | `PHASE-4-ENTRY` and its prerequisite certification sequence | mention as future scope only; do not implement, audit, or certify |
| Notices, travel, regional scale, LOD, deferred second-proof domains | `SECOND-PROOF-ENTRY` or the named later execution gate | remain deferred; no schema, behavior, fixture, or certification expansion here |
| LLM dialogue/speech rendering beyond existing boundary guards | future authorized speech/LLM scope under the ladder | no LLM cognition or feature work; existing no-authority boundaries remain regression constraints |
| Story-sifting, leads/notices expansion, broad 0035 route-forward backlog | later gate explicitly admitting the surface | not the next move and not bundled into remediation |

No forward gate is advanced by this spec. Successful execution only changes the EPI line from scoped remediation to passed through the replacement artifact; later work requires its own numbered spec and evidence.

### 11.3 Survivor files that feed future systems

A survivor may reside in a general proposition, belief, observation, contradiction, replay, validation, or rendering function that future gates also consume. 0041 remediates only the function's certified EPI role. It must neither claim broad future-gate coverage nor defer the current EPI consequence because the same code will later serve another domain.

### 11.4 Explicitly excluded implementation tactics

Out of scope and forbidden:

- widening gameplay or fixture ontology merely to create a mutant consumer;
- a 0039-style wholesale perimeter re-derivation absent a proven census omission;
- new gate/status/invariant/finding identifiers;
- ticket decomposition;
- compatibility aliases, alternate parser spellings, or migration shims;
- test-only production branches or raw insertion APIs;
- changing production semantics solely to suppress mutation generation;
- adding named survivors to the baseline-miss file without the full exception proof;
- copying 0040 results as current evidence; and
- beginning `ORD-LIFE-CERT` or later work inside the replacement report.

---

## 12. Risks and open owner decisions

### 12.1 Remediation risks and mandatory controls

| Risk | Required control |
|---|---|
| Getter-asserts-getter or formatter tautology kills a mutant without proving EPI behavior | Require a projection, replay, provenance, content-validation, proposal, diagnostic, or actor-view consequence; treat helper-level assertions as support only. |
| Equivalent-mutant misclassification | Require final-domain/call-site proof and independent signoff; human intuition and “no test failed” are insufficient. Equivalent detection is a known hard problem, and empirical work shows developers often misclassify such mutants.[^equivalent-review][^equivalent-empirical] |
| Grouped family catches a representative but not all members | Retain member-by-member active-mutant evidence and concrete parameter/case mapping; aggregate family pass is insufficient. |
| Full run exposes more survivors than the historical 30 | Append every new identity to the register and continue remediation; the count is a floor, not a cap. |
| Run cost causes incomplete execution or inconsistent shards | Precompute one census, hold commit/config/tool/arguments constant, reconcile every shard, and block on missing/failed shards. |
| Timeout retries distort the denominator | Keep retries diagnostic; resolve every timeout and link it to the complete-run identity; do not count retry subsets as the full posture. |
| Baseline-miss laundering | Keep the file empty for these survivors unless an exception has the same proof/signoff as the register; report exceptions honestly as misses plus reviewed dispositions. |
| Metric gaming | Do not certify from mutation score, named-30-only campaign, sampled shard, or reduced config; require complete-run and zero-untriaged rules. |
| Source-discipline relapse | Record exact baseline/final SHAs and exact file/artifact fingerprints; archived hashes/results remain historical; no latest-main claim. |
| Test flakiness accidentally reports caught | Re-run/retain the exact mutant failure, prove the unmutated test stable, and tie failure to the named behavior assertion and responsible layer. |
| Production refactor makes an identity disappear without closing the semantic gap | Use historical-to-final normalized identity mapping and explain source changes; disappearance is not disposition. |
| Fix leaks truth/debug into actor surfaces | Pair every firewall-relevant positive with hidden-truth/debug negative controls and EPI-11 equality checks. |
| Snapshot updates launder rendering mutants | Pair snapshots with typed semantic identifiers, provenance, and structured assertions; review diffs under active mutant. |
| Property/generative test is non-reproducible | Record seed, generator domain, case count, shrink result, and a retained deterministic regression case. QuickCheck-style property testing is valuable precisely when properties and reproducible counterexamples are explicit.[^quickcheck] |
| Mutation-specific overfitting | State the upstream behavior property and test the public/production seam. Property-based mutation testing supports judging a kill by whether the mutant violates a named property, not only whether some test fails.[^property-mutation] |

### 12.2 Open owner decisions — do not infer in implementation

The following decisions are not settled by the repository doctrine reviewed for this spec. Owners must decide and record them before or during implementation; none may be silently invented by the acceptance artifact:

1. shard count and sharding algorithm, or whether the final run is unsharded;
2. explicit timeout/multiplier/retry policy and the evidence needed to distinguish true hangs from environment variance;
3. exact replacement acceptance-artifact filename and archive path;
4. exact mutation-artifact retention/index format, including whether raw `mutants.out` trees, structured JSON, compressed archives, or both are retained;
5. independent reviewer-signoff mechanism and required reviewer identity/authority for equivalent or non-critical dispositions;
6. whether cargo-mutants is version-pinned for the certification run and, if so, where that pin is recorded;
7. exact CI expression used to expand the in-diff trigger from two files to `epistemics/**` while preserving existing guarded paths;
8. whether the stale “SPINE-CERT perimeter” configuration comment is corrected in the same implementation change;
9. the repository-specific threshold for distinguishing an approved non-critical behavior difference from a truly equivalent mutant, subject to this spec's prohibition on EPI-impacting exceptions;
10. whether minimized property-test counterexamples become checked-in regression cases or retained evidence artifacts;
11. exact path/name for the final survivor register and machine-readable reconciliation index; and
12. how source-changed historical identities are represented in aggregate counts without double-counting final tool outcomes.

These are execution choices, not clarifying questions for the authoring session. Whatever owners choose must preserve the complete denominator, reproducibility, evidence honesty, and zero-untriaged rule.

---

## 13. Completion checklist

### 13.1 Spec and posture

- [ ] Implementation cites this staged `0041` spec and records its archive path on closeout.
- [ ] The single future-spec posture is `Remediation`.
- [ ] The line remains `EPI-CERT scoped remediation` until the replacement artifact satisfies every aggregate condition.
- [ ] `P0-CERT passed` and `SPINE-CERT passed` are consumed and not reopened.
- [ ] No pass/fail result is inferred from this non-executable authoring document.

### 13.2 Source discipline and exact commits

- [ ] `7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5` is recorded as the source baseline.
- [ ] One exact final implementation commit is recorded separately.
- [ ] No latest-main claim is made.
- [ ] The uploaded manifest is treated only as path inventory.
- [ ] 0040's `ba9fe1c…` and all other embedded hashes are treated as their artifacts' provenance.
- [ ] Archived specs/tickets `0004`–`0040` are labeled historical or structural precedent only.

### 13.3 Mutation perimeter and CI

- [ ] `.cargo/mutants.toml` retains the standing `crates/tracewake-core/src/epistemics/**` perimeter.
- [ ] No required EPI file is excluded or skipped.
- [ ] Final `--list-files` includes `proposition.rs`, `belief.rs`, `contradiction.rs`, and `observation.rs` and the surrounding EPI seam.
- [ ] Final `--list` is retained and fingerprinted.
- [ ] Any census change from the historical 54 files/2763 mutants is explained against the final source/tool version.
- [ ] Scheduled CI uses the checked-in full configured posture.
- [ ] In-diff guarded-path detection covers every `epistemics/**` source change.
- [ ] The 30 EPI survivors are not laundered into `.cargo/mutants-baseline-misses.txt`.
- [ ] Any minimal perimeter correction is additive, live-census-justified, and not a wholesale re-derivation.

### 13.4 Baseline, full run, sharding, and timeouts

- [ ] Format, clippy, build, workspace tests, and every named EPI suite pass at the final commit.
- [ ] Development/filter/iterate runs are labeled non-certifying.
- [ ] The final mutation run loads the checked-in config and has no final `--no-config`, file/function filter, `--in-diff`, or `--iterate` denominator.
- [ ] The complete run finishes unsharded or through a complete reconciled shard set.
- [ ] Commit/config/tool/arguments/denominator are identical across shards.
- [ ] Every shard and structured output is retained; no shard is absent or silently retried under different conditions.
- [ ] Every timeout is resolved; unresolved timeouts/tool failures equal zero.

### 13.5 Survivor triage

- [ ] All 30 historical identities have final reconciliation rows.
- [ ] The 30 count is treated as a seed floor, not the final denominator.
- [ ] Every additional final-run survivor is appended and triaged.
- [ ] Kill-with-behavior/provenance coverage is the default.
- [ ] Every grouped family demonstrates that every member mutant is caught.
- [ ] No killing test relies only on a getter, non-empty string/set, direct helper return, or implementation-detail tautology.
- [ ] Replay/provenance ancestry and negative/contamination controls are present wherever applicable.
- [ ] Equivalent/non-critical exceptions are rare, call-site/domain complete, and independently signed.
- [ ] Blocked/untriaged identities equal zero.
- [ ] Tool outcome counts and certification disposition counts are reported separately and reconcile.

### 13.6 EPI re-proof

- [ ] EPI-01 through EPI-11 are rerun and re-fingerprinted at the exact final commit.
- [ ] 0040 per-seam rows are historical only; no result is inherited.
- [ ] Every seam has exact files/consumers, foundation/architecture dependencies, gate cross-references, positive fixture, adversarial/negative fixture, replay/provenance evidence, mutation evidence, and typed first-divergence diagnostics.
- [ ] The compile-fail boundary corpus is reported case by case.
- [ ] EPI-11 contains all required paired-run relations and exact equality domains.
- [ ] Possession parity and debug attach/detach no-effect relations pass.
- [ ] Hidden truth cannot change actor-visible/decision surfaces before a legal reveal event.

### 13.7 Replacement artifact

- [ ] The artifact conforms to every field in `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
- [ ] Evidence-status and fingerprint-scope honesty are explicit.
- [ ] The completed-mutation row records run completion and full tool/disposition counts.
- [ ] Every evidence claim has a path/behavior witness and replay/provenance ancestry or reviewed `not applicable`.
- [ ] `EMERGE-OBS`, if present, is `observer-only` and does not affect the verdict.
- [ ] Staged abstractions are declared and do not defer EPI obligations; otherwise the artifact states `none`.
- [ ] The artifact renders `EPI-CERT passed` only after every aggregate condition passes.
- [ ] A passing artifact explicitly supersedes the 0040 EPI acceptance artifact.
- [ ] An incomplete artifact remains exactly `EPI-CERT scoped remediation` and names the failing layer.
- [ ] The artifact does not claim latest main, full-project certification, `ORD-LIFE-CERT`, first-proof aggregate, Phase-4, or second-proof passage.

### 13.8 Scope and doctrine

- [ ] Existing canonical gates are composed; none is redefined.
- [ ] No new gate code, observation obligation, status vocabulary, invariant ID, or doctrine-level finding ID is created.
- [ ] `EMERGE-OBS` remains observer-only.
- [ ] The crate direction `core <- content <- tui` remains intact.
- [ ] No compatibility shim, alias path, symmetry-only file, test-only production branch, or truth/debug shortcut is added.
- [ ] SPINE is not re-audited and no later gate is advanced.
- [ ] Ordinary life, institutions, wrong suspicion, notices, travel, regional scale, LOD, LLM dialogue, and story-sifting appear only as named deferrals.
- [ ] Survivor files that feed future gates are remediated only for their EPI role.

### 13.9 Closeout

- [ ] Final spec, report, register, command transcripts, mutation outputs, replay/provenance packages, and fingerprints are archived at stable paths.
- [ ] The ledger is updated only at accepted closeout.
- [ ] 0040 and predecessor artifacts remain intact as historical lineage.
- [ ] The accepted closeout names the next admissible ladder move without implementing it inside 0041.

---

## Outcome

**Proposed.** This document defines the 0041 remediation and replacement-certification contract. No commands were executed and no certification result is asserted by this authoring session.

---

## Appendix A — exact-commit repository evidence ledger
```text
Requested repository: joeloverbeck/tracewake
Target commit: 7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run open/find on full exact-commit URLs
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

### A.1 Raw exact-commit URLs fetched

- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0040_epi_cert_mutation_triage_register.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0040_epi_cert_mutation_final_missed.txt`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/SPEC_LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0039_spine_cert_mutation_triage_register.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/3-reference/02_GLOSSARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/01_PROJECT_CHARTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/proposition.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/belief.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/contradiction.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/observation.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/projection.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/knowledge_basis.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/mod.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.cargo/mutants.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.github/workflows/ci.yml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.cargo/mutants-baseline-misses.txt`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/hidden_truth_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/acceptance_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/anti_regression_guards.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/generative_lock.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/golden_scenarios.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/negative_fixture_runner.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/spine_conformance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/no_human_capstone.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/emergence_ledger.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/tests/fixtures_load.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/tests/forbidden_content.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/tests/golden_fixtures_run.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/tests/schema_conformance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/adversarial_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/tui_seam_conformance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/transcript_snapshot.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/tui_acceptance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/embodied_flow.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/command_loop_session.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/view_models.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/projections.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/agent/actor_known.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/agent/transaction.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/actions/proposal.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/actions/pipeline.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/events/apply.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/replay/rebuild.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/src/validate.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/debug_reports.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/src/render.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/agent/perception.rs`

### A.2 Exact-commit GitHub blob views fetched for line-level inspection

- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/proposition.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0040_epi_cert_mutation_final_missed.txt`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0040_epi_cert_mutation_triage_register.md`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.cargo/mutants.toml`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.github/workflows/ci.yml`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.cargo/mutants-baseline-misses.txt`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/belief.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/contradiction.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/observation.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/projection.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/knowledge_basis.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/projections.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/src/validate.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/view_models.rs`
- `https://github.com/joeloverbeck/tracewake/blob/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/agent/perception.rs`

No repository contamination was observed in the exact-commit fetch evidence.

---

## Appendix B — external research sources consulted

These sources shaped the mutation-run, equivalent-mutant, property/metamorphic, non-interference, provenance, and assurance-evidence methods. Repository doctrine remains authoritative over them.

- `https://mutants.rs/config-file.html`
- `https://mutants.rs/skip_files.html`
- `https://mutants.rs/workspaces.html`
- `https://mutants.rs/baseline.html`
- `https://mutants.rs/mutants-out.html`
- `https://mutants.rs/shards.html`
- `https://mutants.rs/timeouts.html`
- `https://mutants.rs/iterate.html`
- `https://mutants.rs/using-results.html`
- `https://dl.acm.org/doi/10.1145/351240.351266`
- `https://arxiv.org/abs/2301.13615`
- `https://arxiv.org/abs/1409.0393`
- `https://eprints.whiterose.ac.uk/id/eprint/110335/`
- `https://doi.org/10.1109/TSE.2013.44`
- `https://arxiv.org/abs/2404.09241`
- `https://arxiv.org/abs/1303.2784`
- `https://www.w3.org/TR/prov-dm/`
- `https://www.nist.gov/publications/software-assurance-using-structured-assurance-case-models`
- `https://people.csail.mit.edu/lpk/papers/aij98-pomdp.pdf`
- `https://huber.artsci.utoronto.ca/wp-content/uploads/2013/07/Belief-Revision-I-The-AGM-Theory.pdf`

---

## Source notes

[^repo-ledger]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/SPEC_LEDGER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/SPEC_LEDGER.md).
[^repo-ladder]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md).
[^repo-spec-rules]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/README.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/README.md).
[^repo-0037-acceptance]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md).
[^repo-0039-acceptance]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md).
[^repo-0040-acceptance]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md).
[^repo-0040-spec]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md).
[^repo-docs-readme]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/README.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/README.md).
[^repo-arch-index]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md).
[^repo-0040-triage]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0040_epi_cert_mutation_triage_register.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0040_epi_cert_mutation_triage_register.md).
[^repo-0040-missed]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0040_epi_cert_mutation_final_missed.txt](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0040_epi_cert_mutation_final_missed.txt).
[^repo-0039-spec]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md).
[^repo-0037-spec]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md).
[^repo-0039-triage]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0039_spine_cert_mutation_triage_register.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/reports/0039_spine_cert_mutation_triage_register.md).
[^repo-invariants]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md).
[^repo-causal]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md).
[^repo-beliefs-foundation]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md).
[^repo-tui-foundation]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md).
[^repo-firewall-foundation]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md).
[^repo-arch-replay]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md).
[^repo-arch-context]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md).
[^repo-arch-beliefs]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md).
[^repo-arch-tui]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md).
[^repo-arch-evidence]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md).
[^repo-exec-index]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md).
[^repo-exec-boundary]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md).
[^repo-first-proof]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md).
[^repo-exec-firewall]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md).
[^repo-exec-epi]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md).
[^repo-exec-fixtures]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md).
[^repo-exec-testing]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md).
[^repo-ref-index]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md).
[^repo-risk]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/3-reference/01_DESIGN_RISK_REGISTER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/3-reference/01_DESIGN_RISK_REGISTER.md).
[^repo-glossary]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/3-reference/02_GLOSSARY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/3-reference/02_GLOSSARY.md).
[^repo-fixture-contract]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md).
[^repo-accept-template]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md).
[^repo-proposition]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/proposition.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/proposition.rs).
[^repo-belief]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/belief.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/belief.rs).
[^repo-contradiction]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/contradiction.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/contradiction.rs).
[^repo-observation]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/observation.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/observation.rs).
[^repo-projection]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/projection.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/projection.rs).
[^repo-context]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/knowledge_context.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/knowledge_context.rs).
[^repo-basis]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/knowledge_basis.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/knowledge_basis.rs).
[^repo-epistemics-mod]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/mod.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/epistemics/mod.rs).
[^repo-mutants-config]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.cargo/mutants.toml](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.cargo/mutants.toml).
[^repo-baseline-misses]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.cargo/mutants-baseline-misses.txt](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.cargo/mutants-baseline-misses.txt).
[^repo-ci]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.github/workflows/ci.yml](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/.github/workflows/ci.yml).
[^repo-core-hidden-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/hidden_truth_gates.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/hidden_truth_gates.rs).
[^repo-core-replay-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/event_schema_replay_gates.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/event_schema_replay_gates.rs).
[^repo-core-acceptance-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/acceptance_gates.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/acceptance_gates.rs).
[^repo-core-regression-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/anti_regression_guards.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/anti_regression_guards.rs).
[^repo-core-generative-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/generative_lock.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/generative_lock.rs).
[^repo-core-golden-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/golden_scenarios.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/golden_scenarios.rs).
[^repo-core-negative-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/negative_fixture_runner.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/negative_fixture_runner.rs).
[^repo-core-spine-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/spine_conformance.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/spine_conformance.rs).
[^repo-core-nohuman-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/no_human_capstone.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/no_human_capstone.rs).
[^repo-core-emergence-tests]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/emergence_ledger.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/tests/emergence_ledger.rs).
[^repo-content-tests]: Exact-commit repository sources: https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/tests/fixtures_load.rs; https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/tests/forbidden_content.rs; https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/tests/golden_fixtures_run.rs; https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/tests/schema_conformance.rs.
[^repo-tui-tests]: Exact-commit repository sources: https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/adversarial_gates.rs; https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/tui_seam_conformance.rs; https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/transcript_snapshot.rs; https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/tui_acceptance.rs; https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/embodied_flow.rs; https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-tui/tests/command_loop_session.rs.
[^repo-content-validate]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/src/validate.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-content/src/validate.rs).
[^repo-view-models]: Exact-commit repository source: [https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/view_models.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5/crates/tracewake-core/src/view_models.rs).
[^cargo-config]: cargo-mutants, “Config file,” https://mutants.rs/config-file.html.
[^cargo-files]: cargo-mutants, “Filtering files,” including `--list-files` and `--list`, https://mutants.rs/skip_files.html.
[^cargo-baseline]: cargo-mutants, “Baseline tests,” https://mutants.rs/baseline.html.
[^cargo-shards]: cargo-mutants, “Sharding,” https://mutants.rs/shards.html.
[^cargo-timeouts]: cargo-mutants, “Hangs and timeouts,” https://mutants.rs/timeouts.html.
[^cargo-results]: cargo-mutants, “Using the results,” https://mutants.rs/using-results.html.
[^quickcheck]: Koen Claessen and John Hughes, “QuickCheck: A Lightweight Tool for Random Testing of Haskell Programs,” ICFP 2000, https://doi.org/10.1145/351240.351266.
[^property-mutation]: Ezio Bartocci, Leonardo Mariani, Dejan Nickovic, and Drishti Yadav, “Property-Based Mutation Testing,” ICST 2023, https://arxiv.org/abs/2301.13615.
[^metamorphic]: Sergio Segura, Gordon Fraser, Ana B. Sanchez, and Antonio Ruiz-Cortés, “A Survey on Metamorphic Testing,” IEEE TSE 42(9), 2016, https://eprints.whiterose.ac.uk/id/eprint/110335/.
[^equivalent-review]: Lech Madeyski et al., “Overcoming the Equivalent Mutant Problem: A Systematic Literature Review and a Comparative Experiment of Second Order Mutation,” IEEE TSE 40(1), 2014, https://doi.org/10.1109/TSE.2013.44.
[^equivalent-empirical]: Philipp Straubinger, Alexander Degenhart, and Gordon Fraser, “An Empirical Evaluation of Manually Created Equivalent Mutants,” 2024, https://arxiv.org/abs/2404.09241.
[^state-infection]: David Schuler and Andreas Zeller, “Using State Infection Conditions to Detect Equivalent Mutants and Speed up Mutation Analysis,” 2013, https://arxiv.org/abs/1303.2784.
[^noninterference]: Catalin Hritcu et al., “Testing Noninterference, Quickly,” Journal of Functional Programming 26 (2016), https://arxiv.org/abs/1409.0393.
[^prov-dm]: W3C, “PROV-DM: The PROV Data Model,” W3C Recommendation, https://www.w3.org/TR/prov-dm/.
[^assurance-case]: NIST IR 7608, “Software Assurance Using Structured Assurance Case Models,” https://www.nist.gov/publications/software-assurance-using-structured-assurance-case-models.
[^pomdp]: Leslie Pack Kaelbling, Michael L. Littman, and Anthony R. Cassandra, “Planning and Acting in Partially Observable Stochastic Domains,” Artificial Intelligence 101 (1998), https://people.csail.mit.edu/lpk/papers/aij98-pomdp.pdf.
[^belief-revision]: Franz Huber, “Belief Revision I: The AGM Theory,” Philosophy Compass 8(7), 2013, https://huber.artsci.utoronto.ca/wp-content/uploads/2013/07/Belief-Revision-I-The-AGM-Theory.pdf.
