# 0039 — SPINE-CERT mutation remediation and replacement certification spec

**Staging path:** `specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
**Archive path on accepted closeout:** `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
**Target repository:** `joeloverbeck/tracewake`  
**Authoring and reassessment baseline:** `9648c8fb75f6de06c77da4b20b4c30b783cd9217`  
**Spec series:** numbered implementation spec, staged under `specs/`, archived on acceptance closeout.  
**Status:** implementation specification; non-executable; no certification result rendered.  
**Work posture:** `Remediation`.  
**Admissibility posture:** `SPINE-CERT scoped remediation`.  
**Certification-line effect:** successful execution must produce a replacement SPINE-CERT acceptance artifact that renders `SPINE-CERT passed` and explicitly supersedes `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md`.

This spec is subordinate to foundation, architecture, execution, reference, and live spec-layer doctrine. It operationalizes the only admissible next move named by the live ledger: remediation of the SPINE-CERT Wave B mutation posture. It does not amend an invariant, redefine a gate, create a new gate or observation obligation, reopen P0-CERT, or advance a later certification line.

This document is non-executable. It specifies what an implementing session must change, run, prove, review, and package. It does **not** assert that any survivor is already killed or equivalent, that the expanded mutation run has completed, or that SPINE-CERT has passed.

> I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

---

## 1. Status, source discipline, freshness, and admissibility

### 1.1 Single posture and line state

This spec declares exactly one future-spec posture: `Remediation`, as defined by `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`. Its admissibility posture is `SPINE-CERT scoped remediation` because it addresses the named failed certification layer rather than expanding gameplay scope.

The line remains `SPINE-CERT scoped remediation` until an implementing session satisfies this spec and publishes the replacement acceptance artifact. The spec itself does not flip the line. The replacement artifact does so only after all mutation and seam evidence is complete and its capstone verdict is `SPINE-CERT passed`.

P0-CERT is already `P0-CERT passed` through the 0037 replacement artifact. This spec consumes that posture as settled context and does not reopen, rescope, or re-certify P0-CERT. The previously configured Wave A perimeter remains protected as continuity coverage, but it is carried forward to avoid regression, not because P0-CERT is under audit again.

### 1.2 Exact-commit discipline

The target of record is the user-supplied exact commit `9648c8fb75f6de06c77da4b20b4c30b783cd9217`. This workflow does not independently verify that the target is current `main`.

The uploaded manifest is path inventory only. Branch names, default-branch lookup, repository metadata, connector namespace labels, code-search snippets, prior chats, old repository names, uploaded filenames, and commit strings embedded inside historical reports are not proof of target-commit content.

The source files used to author this spec were fetched through full URLs of the form:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/<manifest-path>
```

No clone, branch-name fetch, repository-scoped path lookup, default-branch lookup, or code search was used. Appendix A records the exact-commit source inventory.

The strings `0ce59ad…` and `b4b59c92…` in the 0038 acceptance line are 0038 audit provenance only. They are not this remediation's baseline. Likewise, every commit hash inside an archived spec or report is provenance for that historical artifact, not proof of this target tree.

### 1.3 Baseline versus implementation commit

The authoring and reassessment baseline is `9648c8fb75f6de06c77da4b20b4c30b783cd9217`. An implementation that changes tests, production code, mutation configuration, or CI cannot truthfully remain at that unchanged SHA. Therefore:

1. implementation must begin from the target-of-record baseline or a transparently recorded checkout that contains it;
2. every remediation delta from the baseline must be enumerated;
3. the replacement artifact must name and certify its own exact final implementation commit; and
4. that final SHA must not be misrepresented as independent proof of current `main`.

“Re-prove live at `9648c8f`” in this spec means re-prove the SPINE line from the `9648c8f` target baseline, through the actual remediation delta, at the exact final commit under test. Historical 0038 command transcripts cannot be substituted for that live run.

### 1.4 Admissibility lock

`docs/4-specs/SPEC_LEDGER.md` and the 0038 acceptance artifact close the choice of next work. Until the replacement artifact passes:

- EPI-CERT may not start or claim progression;
- ORD-LIFE-CERT may not start or claim progression;
- FIRST-PROOF-CERT may not be advanced;
- PHASE-4-ENTRY and SECOND-PROOF-ENTRY remain locked;
- the 0035 forward-expansion backlog and feature/gameplay work are inadmissible as the next certification move.

This spec does not re-survey alternatives. It turns the closed remediation mandate into an executable contract. The live ledger row for 0039 is a closeout responsibility after accepted implementation; this staged spec does not pre-author its own ledger acceptance entry.

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

Earlier tiers govern later tiers. If a convenient implementation conflicts with architecture or foundation, the implementation is wrong. If a test kills a mutant by bypassing holder-known provenance, replay ancestry, the shared action pipeline, or debug quarantine, the test is wrong. If an artifact obtains a green row by promoting archived evidence into live proof, the artifact is invalid.

### 2.2 Primary remediation sources

The direct objects and precedents are:

- `reports/0038_spine_cert_mutation_triage_register.md` — the 296-survivor seed inventory, Wave A/Wave B commands, no-silent-exclusion proof, per-file responsible seams and diagnostic layers, and full cargo-mutants survivor text.
- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` — the artifact to supersede; verdict `SPINE-CERT scoped remediation`; historical SPINE-01…SPINE-08 evidence shape; mutation blocker; command transcript index.
- `archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md` — the eight-seam certification contract and fixture/audit shape to re-run.
- `docs/4-specs/SPEC_LEDGER.md` — live source discipline, 0038 posture, and next known execution move.
- `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` — near-exact structural precedent for full-run remediation, survivor disposition, re-proof, and replacement artifact construction.
- `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` — historical example of a completed mutation row and superseding replacement artifact.

The archived 0037 and 0038 documents are historical context and structural precedent. They are not live certifying evidence for the implementation of this spec.

### 2.3 Foundation dependencies

The remediation must preserve, and its tests must witness consequences of:

- `docs/README.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`

Boundary-only foundation documents are read to prevent scope creep, not as new conformance targets: institutions, no-scripting authoring, scale/LOD, LLM/speech, and research-source notes remain governed by their own future gates.

### 2.4 Architecture dependencies

The controlling subsystem contracts are:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

Architecture documents for claims depth, speech/LLM, institutions, ordinary-life economy, story-sifting, regional scale/LOD, and research notes are boundary awareness only for this remediation.

### 2.5 Execution dependencies and gate vocabulary

Execution authority is supplied by:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`

The work composes only existing canonical gates: `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, and `DIAG`, under the existing `SPINE-CERT` phase-certification label and the acceptance-contract cross-references in execution document `02`. These names are cross-references, not newly defined gates.

Execution document `02` acceptance-contract labels — `EVENT`, `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `POSSESSION-PARITY`, `NO-HUMAN-ORDINARY-LIFE`, `MISSING-PROPERTY`, `VIEW-DEBUG-SPLIT`, `REPLAY`, and `FIXTURE-NEGATIVE` — may be cited only as existing cross-references. This spec does not redefine or duplicate them.

`EMERGE-OBS` remains an observer-only observation obligation. It is never a mutation-score target, phase gate, pass/fail threshold, scheduler objective, or substitute for a failed SPINE seam.

### 2.6 Reference and live-spec dependencies

The terminology, anti-Goodhart, source-discipline, and artifact-shape controls are:

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/3-reference/02_GLOSSARY.md`
- `docs/4-specs/README.md`
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`

Archived specs and tickets `0005` through `0038` may explain lineage, risk, or seam intent only. They may not be counted as live certification.

### 2.7 Code, test, and configuration seams

The primary remediation files are the 18 Wave B survivor files:

| File | Seed survivors | Primary SPINE ownership |
|---|---:|---|
| `crates/tracewake-core/src/epistemics/knowledge_context.rs` | 67 | SPINE-03, with SPINE-06/07 consequences |
| `crates/tracewake-content/src/validate.rs` | 57 | SPINE-05/06/08 |
| `crates/tracewake-core/src/events/apply.rs` | 44 | SPINE-01/02/06/08 |
| `crates/tracewake-core/src/view_models.rs` | 24 | SPINE-03/07 |
| `crates/tracewake-core/src/debug_reports.rs` | 19 | SPINE-03/07 |
| `crates/tracewake-content/src/serialization.rs` | 17 | SPINE-01/02/05 |
| `crates/tracewake-core/src/actions/report.rs` | 13 | SPINE-06/08 |
| `crates/tracewake-core/src/events/envelope.rs` | 9 | SPINE-01/02/04/05 |
| `crates/tracewake-content/src/load.rs` | 8 | SPINE-01/05/08 |
| `crates/tracewake-core/src/scheduler.rs` | 8 | SPINE-06/08 |
| `crates/tracewake-core/src/replay/rebuild.rs` | 7 | SPINE-02/03 |
| `crates/tracewake-core/src/checksum.rs` | 6 | SPINE-02/03 |
| `crates/tracewake-core/src/projections.rs` | 5 | SPINE-03/06/07 |
| `crates/tracewake-core/src/actions/proposal.rs` | 3 | SPINE-06/07/08 |
| `crates/tracewake-content/src/schema.rs` | 3 | SPINE-05 |
| `crates/tracewake-tui/src/render.rs` | 4 | SPINE-07 |
| `crates/tracewake-core/src/replay/report.rs` | 1 | SPINE-02 |
| `crates/tracewake-tui/src/transcript.rs` | 1 | SPINE-07 |
| **Total historical seed inventory** | **296** | **floor, not finish line** |

Related zero-survivor or ancestry seams remain in the standing perimeter because they are part of the certified path: event/replay module and log files, authoritative `state.rs`, the `epistemics/projection.rs` holder-known projection surface, the `controller.rs` possession seam, action pipeline, debug capability, content manifest, TUI app/debug panels, prior guarded agent/action surfaces, and the cross-crate proof suites listed in sections 4, 6, and 7.

No backwards-compatibility shim, alias path, test-only production branch, or file created merely for symmetry is admissible.

---

## 3. Problem statement

The 0038 audit did not render `SPINE-CERT passed`. Its aggregate verdict is `SPINE-CERT scoped remediation`.

The local SPINE-01 through SPINE-08 evidence packages were individually reported as passing at 0038's own audit provenance, but the mutation lock layer invalidated an aggregate pass claim. Two mutation waves establish the failure:

| Wave | Configuration posture | Mutants | Caught | Missed | Timeout | Unviable | Meaning |
|---|---|---:|---:|---:|---:|---:|---|
| A — guarded continuity | standing `.cargo/mutants.toml` plus narrow CI `-f` filters | 1128 | 896 | 0 | 0 | 232 | prior configured perimeter remained clean |
| B — SPINE expansion | `--no-config` plus explicit SPINE file filters | 1679 | 1057 | 296 | 0 | 326 | SPINE seam coverage was structurally incomplete |

Wave B exited with cargo-mutants' survivor signal, not a tool crash. The 296 misses are not accepted baseline misses. They were discovered only after bypassing broad configured exclusions that omitted content, TUI, events, replay, epistemics, checksum, view-model, debug-report, proposal/report, and related SPINE files.

The failure therefore has two inseparable parts:

1. **survivor remediation:** the 296 historical Wave B survivors require behavior/provenance coverage or rare, reviewable equivalence/non-critical disposition; and
2. **perimeter governance:** the standing mutation configuration and scheduled CI must permanently include the full SPINE seam set so the same classes cannot disappear behind the old perimeter again.

The 296 count is a floor, not the acceptance target. A complete run under the new standing configuration may enumerate additional mutants or reveal additional survivors. Every survivor from that completed run is within this remediation's disposition obligation.

The responsible failing layers are those already named by the 0038 triage register and execution document `03`: content/schema validation, actor-known context construction, proposal construction, scheduler ordering, action validation, event application, projection/replay, view-model rendering, debug quarantine, tests/fixtures, and documentation/CI status. No failed group may be relabeled as a phase exception.

A narrow campaign that kills a representative subset, increases a mutation score, or adds the remainder to `.cargo/mutants-baseline-misses.txt` would reproduce the anti-Goodhart failure. Certification requires a completed census and an explicit disposition for every survivor.

---

## 4. Remediation approach

### 4.1 Required end state

Execution is complete only when all of the following are true:

1. The implementation starts from the `9648c8f` target baseline and records its exact final commit.
2. `.cargo/mutants.toml` names the standing union of the prior guarded continuity perimeter and the full SPINE-CERT seam perimeter.
3. `.github/workflows/ci.yml` uses that checked-in configuration for scheduled mutation runs and detects changes across the same source perimeter for in-diff mutation runs.
4. No exclusion glob, command-line filter, package-local test default, or CI condition silently removes a required SPINE file or its cross-crate behavior witnesses.
5. The unmutated workspace and named SPINE gate suites pass at the final implementation commit.
6. The configured full mutation posture runs to completion over the entire standing perimeter.
7. Every one of the 296 historical seed survivors is reconciled to the final source and evidence, even if line numbers or implementation shape changed.
8. Every additional survivor discovered by the completed run is triaged.
9. Kill-with-behavior/provenance coverage is the default; grouped contract families prove member-by-member kill reach; equivalence or non-criticality is rare and reviewer-signed.
10. No survivor is untriaged, no timeout is silently green, and no required shard is absent.
11. SPINE-01 through SPINE-08 are re-proven from the target baseline at the exact final implementation commit.
12. A replacement acceptance artifact conforms to `docs/4-specs/0003`, renders `SPINE-CERT passed`, and explicitly supersedes the 0038 artifact.

If any item remains blocked, the resulting artifact must remain `SPINE-CERT scoped remediation` and name the responsible layer. It may not render a partial aggregate pass.

### 4.2 Permanent standing mutation perimeter

The new checked-in perimeter must be the union below. The implementation may express the paths through semantically equivalent globs, but the resulting `--list-files` census must include every listed source file and every module file reached by the directory globs.

This union extends the two 0038 mutation waves to additionally enumerate the SPINE-seam files named in section 6 that those waves had not included — authoritative `state.rs`, the `epistemics/projection.rs` holder-known projection surface, and the `controller.rs` possession seam — so no section-6 primary seam stays outside the mutation census.

```text
crates/tracewake-core/src/agent/**
crates/tracewake-core/src/scheduler.rs
crates/tracewake-core/src/projections.rs
crates/tracewake-core/src/actions/pipeline.rs
crates/tracewake-core/src/actions/defs/eat.rs
crates/tracewake-core/src/actions/defs/sleep.rs
crates/tracewake-core/src/actions/defs/work.rs
crates/tracewake-core/src/events/**
crates/tracewake-core/src/replay/**
crates/tracewake-core/src/checksum.rs
crates/tracewake-core/src/state.rs
crates/tracewake-core/src/actions/proposal.rs
crates/tracewake-core/src/actions/report.rs
crates/tracewake-core/src/view_models.rs
crates/tracewake-core/src/debug_capability.rs
crates/tracewake-core/src/controller.rs
crates/tracewake-core/src/debug_reports.rs
crates/tracewake-core/src/epistemics/knowledge_context.rs
crates/tracewake-core/src/epistemics/projection.rs
crates/tracewake-content/src/manifest.rs
crates/tracewake-content/src/load.rs
crates/tracewake-content/src/schema.rs
crates/tracewake-content/src/serialization.rs
crates/tracewake-content/src/validate.rs
crates/tracewake-tui/src/app.rs
crates/tracewake-tui/src/debug_panels.rs
crates/tracewake-tui/src/render.rs
crates/tracewake-tui/src/transcript.rs
```

At minimum, `.cargo/mutants.toml` must implement this posture with:

- `additional_cargo_args = ["--locked"]` or an equivalent locked test invocation;
- `test_workspace = true`, so a mutant in `tracewake-core` is tested against `tracewake-content` and `tracewake-tui` integration witnesses rather than only the mutated package's tests; for cargo-mutants `27.1.0`, this supplies Cargo's workspace-test flag and must not be duplicated in `additional_cargo_args`;
- `examine_globs` or the cargo-mutants version's equivalent checked-in include mechanism for the full standing list; and
- exclusions narrowed so none overlap a required file.

Cargo-mutants supports checked-in configuration, file examination/exclusion globs, file listing, and workspace-wide testing; command-line file filters can diverge from or override the checked-in filter posture, so the scheduled certification command must not maintain a second independent `-f` list.[^cargo-config][^cargo-files][^cargo-workspaces]

A normative configuration shape is:

```toml
additional_cargo_args = ["--locked"]
test_workspace = true

examine_globs = [
  # the standing union listed above
]

exclude_globs = [
  # only reviewed non-perimeter paths; no overlap with examine_globs
]
```

The final file contents, not this illustrative snippet, are the certifying source. The implementation must document any cargo-mutants version-specific syntax change. At the target baseline, CI pins cargo-mutants `27.1.0` (`.github/workflows/ci.yml`); that version is the syntax baseline against which the implementing session must validate `examine_globs` and `test_workspace`, and any upgrade follows the record-and-re-census policy in section 12.2.

### 4.3 CI convergence and durability

The scheduled mutation job must invoke the checked-in configuration rather than re-declare the perimeter in shell. The expected conceptual command is:

```bash
cargo mutants --workspace --no-shuffle
```

Additional timeout, shard, output-directory, or job-control options are allowed when they preserve the same checked-in perimeter and evidence denominator.

The in-diff job may remain an incremental ratchet, but it must:

- detect changes under every standing-perimeter path, not only the old Wave A paths;
- load the same `.cargo/mutants.toml` rather than a divergent path list;
- run workspace-wide witnesses for cross-crate consequences;
- fail on any unapproved new miss, timeout, missing output, or tool failure;
- compare stable mutant identity using path, symbol/function, and mutation operator/diff, not line number alone; and
- upload enough output to reconstruct the decision.

`crates/tracewake-core/tests/ci_workflow_guards.rs` or the repo's existing CI guard layer must be updated so future CI edits cannot silently narrow the scheduled or in-diff perimeter. The guard must inspect the standing list, `test_workspace` posture, no-silent-exclusion rule, configured tool version or version policy, output retention, and baseline-disposition enforcement.

The scheduled frequency, exact in-diff mechanics, and runner topology are owner decisions. The standing coverage itself is not optional.

### 4.4 Preflight: prove the configured census before mutation

Before remediation runs, and again on the final checkout, the implementing session must archive:

```bash
cargo mutants --workspace --list-files
cargo mutants --workspace --list
```

The `--list-files` result must be mechanically compared with the expected standing file census. It must include, among other files, `events/mod.rs` and `replay/mod.rs` reached by the directory globs, plus the explicitly named SPINE-seam files `state.rs`, `controller.rs`, and `epistemics/projection.rs`, even if those files produce no mutants.

The preflight must fail if:

- a required path is absent;
- an excluded glob overlaps a required path;
- a crate or integration test suite is omitted from workspace testing;
- a command-line option silently changes the checked-in perimeter;
- the enumerated mutant population changes without an explained source/config/tool-version delta; or
- the output cannot identify the exact checkout and config fingerprint.

The artifact must retain the list-files output, mutant census, `.cargo/mutants.toml` fingerprint, CI workflow fingerprint, cargo-mutants version, Rust toolchain, Cargo lock fingerprint, and exact implementation SHA.

### 4.5 Clean baseline and named gate preflight

The final mutation campaign requires a clean unmutated baseline. At minimum, record:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
```

The named SPINE suites in section 7 must also pass before a mutation result can be interpreted as certification evidence. Cargo-mutants' baseline is specifically intended to establish that the unmodified tree builds and tests before mutants are applied.[^cargo-baseline]

A failing or flaky baseline blocks the mutation campaign. It may not be hidden by `--baseline=skip`, retries that discard failures, or per-shard filtering.

### 4.6 Run to completion, sharding, timeouts, and reproducibility

The full configured posture must run to completion. Cargo-mutants output must be retained in its structured forms, including the generated mutant census, outcomes, diffs, logs, and outcome text files.[^cargo-output]

Sharding is permitted for cost control only when:

- every shard uses the exact same final source checkout, Rust toolchain, Cargo.lock, cargo-mutants version, configuration, test-workspace posture, timeout policy, and shard denominator;
- all shard indices complete;
- each shard writes to a distinct output directory;
- the artifact proves the union is complete and non-overlapping;
- a clean baseline is recorded immediately before the shard set; and
- all shard outputs are retained and aggregated without dropping tool failures or timeouts.

Cargo-mutants requires consistent shard arguments and denominator for a meaningful aggregate.[^cargo-shards]

Timeouts are not passes. The final certifying campaign must have zero unresolved timeouts. A timeout must be investigated as a test/runtime defect, killed by an appropriate witness, or covered by the same rare review standard as a claimed equivalent/non-critical survivor; changing the timeout alone does not prove behavior.[^cargo-timeouts]

`--iterate` may be used during local remediation to shorten feedback, but the final clean census must run without it because iteration is heuristic and may skip mutants based on earlier outcomes.[^cargo-iterate]

The exact shard count and timeout policy remain owner decisions and must be recorded in the replacement artifact.

### 4.7 Survivor identity and reconciliation

The 0038 register is a seed work-list. Each historical entry must be tracked using:

- raw 0038 cargo-mutants text (`path:line:column:operator`);
- normalized identity (`path`, enclosing symbol/function, mutation operator or diff);
- current final-checkout identity, if still generated;
- source-change mapping when line numbers or functions move; and
- final evidence/disposition.

A historical survivor does not vanish from the obligation merely because a refactor moves its line, renames its helper, or prevents the same syntactic mutant from being generated. The register must map that entry to the production/test change and the behavior witness that closes the original semantic gap.

Any additional survivor from the completed standing run is appended to the same register. Survivors remain cargo-mutants identities, not newly minted doctrine-level finding IDs.

### 4.8 Triage register schema

The replacement package must include or link a complete triage register with, at minimum:

| Field | Required content |
|---|---|
| Historical identity | 0038 raw entry when applicable; otherwise `new in completed run`. |
| Final identity | final path, symbol/function, operator/diff, cargo-mutants structured-output reference. |
| Tool outcome | `caught`, `missed`, `timeout`, `unviable`, or tool failure, as emitted by the tool. |
| Responsible SPINE seam | SPINE-01…SPINE-08 cross-reference, without defining a new gate. |
| Responsible failing layer | one of execution document `03`'s existing diagnostic layers. |
| Certified reachability | invariant/gate/seam consequences and exact consuming call sites. |
| Test family | named existing or new behavior-contract family and exact case that observes this mutant. |
| Behavior witness | certified consequence, not a helper literal or getter tautology. |
| Replay/provenance ancestry | event IDs/log segment, context ID/hash/frontier, projection/replay package, manifest/schema identity, or justified `not applicable`. |
| Negative/contamination control | forbidden shortcut or invalid input that remains failing/omitted. |
| Certification disposition | killed, equivalent, non-critical, or blocked/untriaged; these are register dispositions, not new gate statuses. |
| Evidence references | command transcript, diff, test failure under mutant, baseline pass, output artifact. |
| Review signoff | implementer plus the repository's accepted independent review evidence. |

Tool outcomes and certification dispositions are separate axes. For example, a final `missed` tool outcome can only be accepted when the register supplies a reviewer-approved equivalent or non-critical disposition; it must not be counted as “caught.”

### 4.9 Kill-with-coverage default

The default disposition for every behavior-relevant survivor is kill with behavior/provenance coverage. A valid killing witness must:

1. pass against the unmutated implementation;
2. fail when the specific mutant is active;
3. observe a public or certified subsystem consequence rather than merely restating the mutated expression;
4. travel through the real production seam, not a test-only bypass;
5. include replay/provenance ancestry when the behavior is event-derived or holder-known; and
6. include a negative or contamination control when hidden truth, direct state, prose-born fact, or debug leakage is at risk.

Examples of insufficient tests include:

- asserting only that a vector or string is non-empty;
- asserting `debug_only()` returns the literal already stored by the same getter;
- asserting a stable ID equals a hard-coded literal without showing a serialization, checksum, report, or consumer consequence;
- calling a private parser directly while the production round-trip remains untested;
- constructing actor-known facts in a test-only path that bypasses event/provenance sealing; or
- snapshotting prose while ignoring the typed semantic fields that the certification actually depends on.

Cargo-mutants itself recommends repairing misses through observable behavior rather than overfitting to private implementation details.[^cargo-results]

### 4.10 Grouped behavior-contract families

Kindred survivors may share a parameterized, property-based, table-driven, golden-fixture, or metamorphic contract family. Grouping is allowed to control the 296-survivor scale, but it is not sampling.

For each grouped family, the evidence package must include a member-by-member matrix showing:

- the mutant identity;
- the parameter/case that reaches it;
- the baseline result;
- the test assertion or semantic comparison that fails under that mutant;
- the replay/provenance or negative-control artifact; and
- the cargo-mutants output proving the mutant was caught.

A family that kills only a representative member is insufficient. Property-based and metamorphic methods are useful because they specify relations over many cases, but the certifying package still must demonstrate that every grouped survivor is caught by at least one concrete execution.[^quickcheck][^property-mutation][^metamorphic]

### 4.11 Equivalent and non-critical exceptions

Equivalent-mutant determination is difficult and undecidable in general; redundant mutants can also distort mutation-score interpretation. Consequently, “no test failed,” compiler optimization, or a high mutation score is never sufficient.[^madeyski-equivalent][^papadakis-survey][^just-redundant]

A claimed equivalent mutant must include:

- the exact mutant diff;
- exact reachable call sites and input domain;
- a semantic argument that no reachable input can distinguish original and mutant;
- any compiler/IR equivalence result as supporting evidence only, never the sole proof; and
- independent reviewer signoff.

Compiler-based equality can identify some trivial equivalences but does not solve the general problem and must remain corroborative.[^kintis-compiler]

A claimed non-critical mutant may be behavior-changing, but must prove that no SPINE-CERT invariant, gate, seam, replay/provenance obligation, or debug-quarantine property can observe the difference. A future EPI-CERT-only consequence may justify a narrowly scoped non-critical SPINE disposition only when exact call sites show no current SPINE reach; the artifact must not inflate that conclusion into an EPI-CERT pass.

Neither exception may defer a SPINE survivor, hide an implementation defect, or rely on a single implementer's assertion. The exact reviewer-signoff procedure remains an owner decision.

### 4.12 Baseline-miss discipline

At the target baseline, `.cargo/mutants-baseline-misses.txt` is empty. The remediation must not use it as an intake queue for unresolved survivors.

A baseline entry, if the owners retain this mechanism, may be added only after the same equivalent/non-critical evidence and reviewer signoff required above. The entry must link back to the triage register and preserve normalized identity without collapsing distinct operators or symbols into one permissive pattern.

The scheduled CI gate must fail on:

- any miss absent from the reviewed register/baseline;
- any changed mutant whose prior rationale no longer applies;
- any timeout or tool failure;
- any baseline entry lacking its evidence reference; and
- any broad normalization that launders multiple live survivors into one accepted line.

Whether approved exceptions remain in `.cargo/mutants-baseline-misses.txt` or move to a separately reviewed disposition artifact is an owner decision. Behavior-changing, SPINE-relevant misses may not be accepted in either location.

### 4.13 Production-code integrity and staged abstractions

Production code may be corrected when a survivor reveals a real defect, an untestable ambiguity, or an overbroad private surface. It may not be changed merely to prevent cargo-mutants from generating a mutant, to move code behind an exclusion, or to create an equivalent-looking test-only path.

No `#[mutants::skip]`, exclusion glob, dead-code trick, test-only branch, compatibility alias, or alternate dispatch path may make a survivor disappear without the same reachability and review evidence required for an equivalent/non-critical disposition.

A staged abstraction may bound future institutions, LOD, regional scale, or LLM/speech surfaces. It may not defer any SPINE survivor disposition, event/replay/projection proof, actor-known provenance, debug quarantine, no-direct-dispatch closure, or SPINE-01…SPINE-08 obligation.

---

## 5. Per-survivor-group remediation deliverables

This section turns the 18-row triage table into implementation contracts. Counts are the historical Wave B seed counts. The implementing session must re-enumerate the final population and update each group with any added, removed, moved, or transformed mutants.

### 5.1 `crates/tracewake-content/src/load.rs` — 8 seed survivors

**SPINE ownership:** SPINE-05 save/content identity, SPINE-01 seed event causality, SPINE-08 seed-mutation boundary.  
**Responsible layer:** `content/schema validation`, with `event application` and `tests/fixtures` consequences.

The historical cluster covers `fixture_scope_from_raw_lines`, its `phase1`, `phase2a_historical`, and `phase3a_historical` arms, the event-index arithmetic in `seed_event_log`, and event-ID progression in `append_starting_belief` and `append_role_assignment_notices`.

Required remediation:

- Build a table-driven fixture-load contract that loads each legal scope marker through the real loader and proves the resulting typed `FixtureScope`, manifest identity, and accepted fixture census.
- Add malformed, duplicated, unknown, and contradictory scope inputs that fail at fixture-load validation rather than silently defaulting.
- For seeded beliefs and role notices, prove event IDs/global order are monotonic, unique, causal, and stable across duplicate loads.
- Replay the resulting event log and prove the actor-known belief/assignment consequence cites the expected source event IDs.
- Prove post-load external callers still cannot invoke seed mutators or mutate seed maps directly.

The contract must fail separately for each deleted scope arm and each counter mutation. “The loaded log is non-empty” is not sufficient; the witness must compare exact semantic scope, event count/order, and provenance ancestry.

### 5.2 `crates/tracewake-content/src/schema.rs` — 3 seed survivors

**SPINE ownership:** SPINE-05 schema-to-authoritative-state conversion.  
**Responsible layer:** `content/schema validation`.

The cluster changes the comparison in `FixtureSchema::to_agent_state` that selects the earliest `start_tick` when multiple routine assignments compete to become an actor's active intention.

Required remediation:

- Identify the exact field and bound at the final checkout.
- Add below-bound, exact-bound, and above-bound cases through the public schema conversion path.
- Prove valid boundary cases produce the intended typed state and stable serialization/checksum consequence.
- Prove invalid cases fail with a typed schema diagnostic and produce no partially accepted authoritative state or seed events.
- Pair each invalid case with a nearest valid sibling so a broad “all fail” validator cannot satisfy the family.

Each of the `< -> ==`, `< -> >`, and `< -> <=` mutants must be caught by a distinct parameter row or an explicitly demonstrated relational assertion.

### 5.3 `crates/tracewake-content/src/serialization.rs` — 17 seed survivors

**SPINE ownership:** SPINE-01 event serialization, SPINE-02 replay inputs, SPINE-05 save/schema integrity.  
**Responsible layer:** `content/schema validation` and `projection/replay`.

The cluster includes `serialize_event_log -> vec![]`, parser-arm deletion for channels, stances, routine families, and institution privacy scope, `parse_optional_tick -> Ok(None)`, and `split_usize -> vec![0]`.

Required remediation:

- Build exhaustive typed round-trip families for every affected enum variant: serialize a real typed value, deserialize it, replay or consume it, and compare semantic state rather than only bytes.
- Exercise `touch_or_search`, `absence_marker`, `reading_placeholder_schema_only`; `believes_true`, `believes_false`, `doubts`, `unknown_or_unresolved`; all affected routine families; and institution privacy scope.
- Exercise both present and absent optional ticks, including nonzero values that affect ordering or freshness.
- Exercise the affected routine-family arms `morning_wake`, `return_home`, `leave_unsafe_place`, `continue_current_intention`, `wait`, and `idle_with_reason`, plus unaffected sibling variants that prevent an always-error parser from passing.
- Exercise empty, singleton, and multi-value interruption-point vectors with nonzero members and canonical ordering.
- Prove an empty event-log serialization cannot pass save/replay package validation as a valid nonempty fixture log.
- Add unknown-token, truncated-field, and malformed-list negatives that fail loudly with serialization diagnostics.

The strongest witness is a canonical save/log round trip whose replay reconstructs the same state, holder-known projection, and manifest-scoped fingerprint. Direct parser unit tests may supplement, but not replace, the production round trip.

### 5.4 `crates/tracewake-content/src/validate.rs` — 57 seed survivors

**SPINE ownership:** SPINE-05 content validation, SPINE-06 action/content boundary, SPINE-08 no authored bypass.  
**Responsible layer:** `content/schema validation`; failures may also expose `doctrine mismatch` or `tests/fixtures`.

The historical inventory spans raw-line arithmetic/boolean guards, reserved/display markers, missing references, location reference validation, topology, numeric bounds, routine and no-sleep contracts, action scope, direct-state/script markers, prose-born markers, target support, semantic IDs, no-player rules, event causes, authored-outcome markers, epistemic seed provenance/order, deterministic ordering/uniqueness, fixture-contract validation, and serialization round-trip validation.

Required remediation is a branch-isolating validator matrix:

- For every boolean clause or match guard, construct the nearest valid fixture and a single-defect invalid sibling that changes only the guarded property.
- Assert the typed responsible layer and diagnostic category, not only that an error string exists.
- Prove rejection occurs through the real fixture loader/validator before any seed event, authoritative state, actor-known fact, or action proposal can be created.
- For missing references and topology, cover place, container, actor, item, door, route, and nesting/cycle variants actually reachable in the final source.
- For numeric bounds, cover below/equal/above boundaries.
- For direct-state, script, prose-born fact, player/objective, and authored-outcome markers, include lexical variants that avoid relying on one banned substring while preserving the forbidden semantic shape.
- For epistemic seeds, prove source kind, event ancestry, ordering, privacy, and holder scope are all required; a typed fact with unproven source must still fail.
- For determinism, prove semantically equivalent reordered inputs canonicalize identically while duplicate or unstable inputs are rejected as required.
- For round-trip validation, prove accepted fixtures serialize, deserialize, and replay without semantic drift; corruptions must fail rather than be repaired.

Broad tests such as “the forbidden fixture set has at least one failure” are inadequate. The evidence matrix must map all 57 historical mutants, plus any new ones, to a concrete fixture and assertion. The positive sibling prevents a mutant-killing test from becoming a validator that rejects everything.

### 5.5 `crates/tracewake-core/src/actions/proposal.rs` — 3 seed survivors

**SPINE ownership:** SPINE-06 proposal provenance, SPINE-07 semantic TUI input, SPINE-08 legal dispatch path.  
**Responsible layer:** `proposal construction`.

The cluster mutates `ProposalSource::stable_id` and removes TUI source context.

Required remediation:

- Drive a real TUI semantic action into proposal construction and the shared pipeline.
- Prove the proposal retains `ProposalSource::TuiSemanticAction`, its source holder-known context ID/hash/frontier, and the semantic action context consumed by validation/diagnostics.
- Drive at least one autonomous actor proposal and one non-TUI source through the same stable-ID consumer.
- Make the stable ID observable through canonical serialization, trace/report provenance, or replay package identity; do not assert only the getter literal.
- Remove or corrupt TUI context in an adversarial case and prove the proposal is rejected or diagnosed rather than silently reconstructed from truth or UI state.

Each empty/`xyzzy` stable-ID mutant and the `tui_context -> None` mutant must fail a concrete end-to-end case.

### 5.6 `crates/tracewake-core/src/actions/report.rs` — 13 seed survivors

**SPINE ownership:** SPINE-06 validation feedback and SPINE-08 no-direct-dispatch checked facts.  
**Responsible layer:** `action validation`.

The cluster deletes parser arms in `CheckedFactKey::from_stable_key` for body exclusivity, container, duration, from/to place, item, need kind, pipeline slots, place, reason, sleep affordance, target, and ticks.

Required remediation:

- Construct a table containing every stable key supported by the final enum.
- Round-trip each key through the same serialization/report path used by accepted and rejected pipeline reports.
- For each key, produce at least one real validation report whose checked fact is semantically consumed by a diagnostic or replay/trace artifact.
- Prove unknown, misspelled, duplicate, and unsupported keys fail loudly; no generic fallback may turn them into another fact.
- Include both accepted and rejected proposals so a report implementation that emits all facts unconditionally cannot pass.
- Tie no-direct-dispatch evidence to checked facts: the report must show validation occurred before event append/application.

The member matrix must show each deleted arm is killed. A direct `from_stable_key(key).is_some()` table is supporting evidence only; the certifying witness must include the report's pipeline consequence.

### 5.7 `crates/tracewake-core/src/checksum.rs` — 6 seed survivors

**SPINE ownership:** SPINE-02 replay determinism and SPINE-03 projection/context identity.  
**Responsible layer:** `projection/replay`.

The cluster mutates `AgentStateChecksum::as_str`, location canonicalization, and routine-step status identity.

Required remediation:

- Build pairs of semantically distinct states that differ only in the affected location or routine status and prove their authoritative checksums differ.
- Build semantically equivalent states with different insertion order and prove canonical checksums remain equal.
- Serialize and replay representative logs, then prove live and replay checksums match exactly.
- Intentionally corrupt an affected field and prove `ReplayReport` names a mismatch/first divergence rather than accepting the package.
- Make the checksum string observable as a persisted/replay artifact fingerprint, not a direct `as_str()` getter assertion.

The tests must avoid asserting implementation fragments in isolation. The consequence is replay identity and divergence detection.

### 5.8 `crates/tracewake-core/src/debug_reports.rs` — 19 seed survivors

**SPINE ownership:** SPINE-03 projection diagnostics and SPINE-07 debug quarantine.  
**Responsible layer:** `debug quarantine` and `view-model rendering`.

The cluster covers debug-only markers, actor/item filters, event-type classification, empty stuck views, and decision-trace row rendering.

Required remediation:

- For one authoritative state, build paired embodied and debug channels. The debug channel must contain the expected report facts; the embodied channel must omit them unless independently actor-known.
- Parameterize every debug report type and prove its `debug_only` marker is consumed by the channel/router or rendering quarantine, not merely returned by a getter.
- Exercise actor and item filters with included, excluded, and nonexistent IDs so deleted predicates and boolean flips are caught.
- Exercise every affected event-type classifier through a real report row derived from an event log.
- Exercise nonempty stuck/decision traces and prove typed rows survive report construction; empty/`xyzzy` return mutants must change the debug artifact.
- Prove generating or viewing a debug report does not alter event log, state checksum, holder-known context, proposals, or future scheduling.
- Prove debug capability cannot be forged from content/TUI/external crates through existing negative fixtures.

Exact display prose may be snapshot support, but semantic row fields, non-diegetic marker, and channel separation are the certifying observations.

### 5.9 `crates/tracewake-core/src/epistemics/knowledge_context.rs` — 67 seed survivors

**SPINE ownership:** SPINE-03 holder-known projection and non-truth-writer boundary, with SPINE-06 proposal and SPINE-07 embodied/debug consequences.  
**Responsible layer:** `actor-known context construction`.

This is the largest cluster: context mode/source/status stable IDs, scope and provenance canonical keys, source keys, fields of `ActorKnownCurrentPlaceFact`, `ActorKnownCarriedItemFact`, `ActorKnownWorkplaceFact`, `ActorKnownFoodSourceFact`, `ActorKnownSleepAffordanceFact`, route/door/container/item/local-actor facts, forbidden-truth audit status, and location canonicalization.

Required remediation:

- Build a parameterized fact-family harness that constructs each actor-known fact through real event/projection ancestry, seals a `KnowledgeContext`, and records holder-known context ID, hash, frontier, provenance entries, and the relevant typed fact.
- For every stable ID/source key/canonical key mutant, prove a downstream semantic consequence: context hash/fingerprint, serialization/report identity, projection lookup, or replay comparison. A hard-coded getter equality alone is not certifying.
- For every fact-field mutant, use two cases that differ only in the affected field and prove the sealed context and at least one downstream projection/view/proposal consequence differ as doctrine requires.
- For fact absence, prove the action or embodied affordance remains unavailable; for fact presence, prove it becomes available only through the source-backed context.
- For boolean fields such as known/open/carried/available states, exercise both truth values and a stale/contradictory case where applicable.
- For `ForbiddenTruthAudit::passed`, include an allowed-source context and a typed-but-unproven forbidden-source context; the latter must fail closed even when banned words are absent.
- Build paired worlds identical in actor-known evidence but different in hidden truth. Their actor proposals and embodied views must remain identical until a modeled observation/record event changes the context. This is a noninterference-style firewall witness, not a claim that the entire future EPI substrate is certified.[^noninterference]
- Prove debug comparison can see the hidden difference without feeding it back into actor-known context, proposals, records, or scheduling.

Every one of the 67 historical mutants must map to a parameter row and an observed failed consequence. Grouping all source-key mutants or all fact-field mutants is permitted only with the member-by-member kill matrix.

### 5.10 `crates/tracewake-core/src/events/apply.rs` — 44 seed survivors

**SPINE ownership:** SPINE-01 event application/causality, SPINE-02 replay, SPINE-06 transaction consequences, SPINE-08 mutation authority.  
**Responsible layer:** `event application`, with `action validation` and `projection/replay` diagnostics.

The cluster includes starting-belief application, event allowlist/typed-diagnostic guards, caused-event IDs, parser arms for epistemic and agent payloads, optional routine template IDs, numeric parsers, intention transitions, boolean parsing, item take/place application, and actor/item preconditions.

Required remediation:

- Build an event-kind/application matrix covering every affected parser arm and transition through `EventEnvelope` -> append/log -> `apply_event_stream`/`apply_event` -> authoritative state -> projection/replay.
- For epistemic events, prove belief/observation/contradiction fields and source event IDs survive into sealed actor-known context and replay.
- For caused-event IDs, prove causal ancestry is present, ordered, and stable after serialization/replay; nonexistent causes must fail loudly.
- For typed diagnostic validation, prove malformed payloads cannot be accepted by flipping or deleting the validation predicate.
- For intention status/source and optional routine template fields, exercise every affected variant plus an unknown-token negative.
- For numeric parsers, use nontrivial values whose effect is visible in state, ordering, duration, or trace; zero-only tests are insufficient.
- For item take/place and actor/item preconditions, include accepted and rejected transitions. Rejected events must leave authoritative state/checksum unchanged and emit the responsible typed failure.
- Prove replay application uses the same authoritative path and does not gain an alternate repair path.
- Preserve internal mutation capability closure: tests must not call private mutators directly to kill an apply mutant.

A parser-only assertion may locate a failure, but the certifying case must observe the applied state/provenance or loud rejection consequence.

### 5.11 `crates/tracewake-core/src/events/envelope.rs` — 9 seed survivors

**SPINE ownership:** SPINE-01 envelope identity, SPINE-02 replay input, SPINE-04 random-stream records, SPINE-05 persistence.  
**Responsible layer:** `event application` and `projection/replay`.

The cluster deletes or corrupts stable mappings for controller/replay-debug streams, deferred/replay schedule phases, controller source, validation-report cause, and random-draw serialization/deserialization.

Required remediation:

- Exhaustively round-trip every affected stream, schedule phase, source, and cause variant through canonical envelope serialization.
- Make each field observable in event-log fingerprints, stream ordering, replay application, or cause/provenance reports.
- Exercise random-draw records with nonzero seed/stream/draw values; duplicate replay must preserve identity and outcomes.
- Add malformed, unknown, and out-of-range random-draw inputs that fail with typed envelope/serialization diagnostics.
- Prove a deleted arm cannot silently map to a default stream, source, phase, or cause.
- Include one corrupted envelope whose first divergence is recorded by replay.

Direct stable-ID assertions are supporting only. The required witness is envelope identity as consumed by log ordering, replay, validation ancestry, or random-stream audit.

### 5.12 `crates/tracewake-core/src/projections.rs` — 5 seed survivors

**SPINE ownership:** SPINE-03 projection boundary, SPINE-06 semantic action surface, SPINE-07 embodied view.  
**Responsible layer:** `projection/replay` and `actor-known context construction`.

The cluster flips/deletes semantic-action filters, removes workplace provenance, and changes proposal semantic-entry comparison.

Required remediation:

- Construct sealed contexts containing allowed, disallowed, stale, and source-missing semantic action facts.
- Prove the projection exposes only actions supported by current actor-known context and exact provenance.
- Prove deleting workplace provenance prevents or diagnoses work proposal availability rather than falling back to authoritative assignment truth.
- Use paired hidden-truth worlds to prove projections do not gain extra actions from unseen world state.
- Rebuild the projection from replay and compare context hash/frontier, semantic actions, and provenance vectors.
- Carry at least one projected semantic action into a real proposal and validation trace.

Tests must not make a projection pass by adding truth-derived facts. Missing provenance must remove/reject the affordance and produce the responsible diagnostic.

### 5.13 `crates/tracewake-core/src/replay/rebuild.rs` — 7 seed survivors

**SPINE ownership:** SPINE-02 deterministic rebuild and SPINE-03 reconstructed context/projection.  
**Responsible layer:** `projection/replay`.

The cluster replaces decision-context hash output, removes event-cause handling, and alters latest-perception/latest-need selection predicates.

Required remediation:

- Rebuild the same event package twice and compare authoritative state, projections, context hashes/frontiers, and replay report fields.
- Corrupt or remove a cause and prove rebuild fails with first-divergence/causal diagnostics rather than continuing.
- Construct competing perception and need events across actors, kinds, channels, and ticks; prove the intended latest event is selected.
- Add an unrelated event metamorphically and prove it does not change latest-selection results, while a later relevant event does.[^metamorphic]
- Ensure empty/constant context-hash mutants cause a replay or provenance mismatch in a real fixture.

A helper returning `Ok(())` must be caught because the rebuilt package lacks the required semantic evidence, not because a private helper value is asserted directly.

### 5.14 `crates/tracewake-core/src/replay/report.rs` — 1 seed survivor

**SPINE ownership:** SPINE-02 divergence reporting.  
**Responsible layer:** `projection/replay`.

The historical mutant reverses the equality/mismatch condition in `run_replay`.

Required remediation:

- Run one known-good replay package and prove `matches_expected` plus checksum/report equality.
- Run one intentionally corrupted package and prove non-match, first divergence, expected/actual fingerprints, and responsible layer.
- Assert the semantic report fields, not only rendered wording.
- Prove the corrupt package is not repaired or treated as a passing replay.

The two-arm pair is mandatory: a test containing only the mismatch case can be satisfied by an implementation that always reports failure.

### 5.15 `crates/tracewake-core/src/scheduler.rs` — 8 seed survivors

**SPINE ownership:** SPINE-06 scheduler/event pipeline and SPINE-08 no direct routine dispatch.  
**Responsible layer:** `scheduler ordering`.

The cluster mutates routine-completion append helpers, SleepCompleted/WorkBlockCompleted arms, tick/equality comparisons, and instant-progress boolean conditions.

Required remediation:

- Exercise due events at ticks immediately before, exactly at, and after the boundary.
- Prove sleep and work-block completions append the canonical event exactly once, with deterministic ordering key and cause ancestry.
- Prove unrelated instant events do not progress a routine, while the exact qualifying event does.
- Replay the no-human sequence and compare event count/order, routine state, needs, and diagnostics.
- Prove scheduler activity does not manufacture primitive action proposals from raw truth, rewrite a transaction's wait reason, or become cognition authority.
- Include an accepted scheduled completion and a blocked actor action so event scheduling and actor validation remain distinct.

The witness must observe event append/application and routine lifecycle, not a private predicate alone.

### 5.16 `crates/tracewake-core/src/view_models.rs` — 24 seed survivors

**SPINE ownership:** SPINE-03 actor-filtered projection and SPINE-07 embodied/debug presentation.  
**Responsible layer:** `view-model rendering` and `debug quarantine`.

The cluster includes why-not/provenance stable IDs, empty/`xyzzy` debug diagnostics, debug-only flags across view types, and truth-belief mismatch non-diegetic markers.

Required remediation:

- Carry typed why-not and provenance values from a real rejected/blocked action into view-model and TUI/transcript consumption.
- Make stable IDs observable through typed serialization/snapshot or consumer routing, not only literal getters.
- Build paired embodied/debug view models from the same event frontier and prove all debug view types remain in the debug channel.
- Prove truth-belief mismatch views carry a non-diegetic marker and never appear as actor-known facts or semantic actions.
- Exercise nonempty diagnostic rows so empty/`xyzzy` mutants alter an inspectable debug artifact.
- Prove possession changes viewpoint/input binding without transferring another actor's knowledge or debug facts.
- Add a negative channel-routing case for every grouped `debug_only` mutant and show the member-specific mutant causes leakage or rejection of the expected debug view.

A `debug_only() == true` getter test is expressly insufficient. The certifying observation is channel quarantine and noninterference with embodied behavior.

### 5.17 `crates/tracewake-tui/src/render.rs` — 4 seed survivors

**SPINE ownership:** SPINE-07 TUI-first proof surface.  
**Responsible layer:** `view-model rendering` and `debug quarantine`.

The cluster flips embodied/notebook predicates and replaces an action-rejection rendering value with empty/`xyzzy` text.

Required remediation:

- Render representative embodied, notebook, rejection, and debug sections from real view models created at a known event frontier.
- Assert semantic section presence, typed IDs/reasons/checked facts, and channel markers; prose snapshots may supplement but may not be the only oracle.
- Prove ordinary embodied output excludes debug-only tokens and truth-belief mismatch details.
- Prove notebook content appears only under its modeled availability rules.
- Prove an action rejection retains its typed reason and checked-fact ancestry through rendering.
- Pair each positive render with an adversarial model that must omit or quarantine the section.

The rendering tests must travel from production view-model construction and must not instantiate a special test-only string path.

### 5.18 `crates/tracewake-tui/src/transcript.rs` — 1 seed survivor

**SPINE ownership:** SPINE-07 transcript evidence.  
**Responsible layer:** `view-model rendering`.

The historical mutant reverses the representative-section comparison.

Required remediation:

- Build a deterministic transcript with multiple required and non-required sections.
- Prove the intended representative sections are selected in canonical order and unrelated sections are excluded.
- Run the same state twice and compare the semantic transcript section sequence.
- Pair embodied and debug transcripts to prove debug-only sections do not enter the ordinary transcript.
- Make the selection observable through the archived transcript evidence package, not only an internal filter result.

---

## 6. SPINE-01…SPINE-08 live re-proof contract

The mutation remediation and seam re-proof are one evidence program. Killing survivors strengthens the same behavior witnesses on which the eight seams depend. The 0038 per-seam rows are historical context and structural shape only; they must be re-established at the final implementation commit.

### 6.1 SPINE-01 — event log, envelope, and append-only causal stream

**Composed gates:** `REPLAY`, `FIXTURE`, `DIAG`.  
**Primary files:** `events/envelope.rs`, `events/log.rs`, `events/apply.rs`, `events/mod.rs`, `events/mutation.rs`, `state.rs`, `checksum.rs`, plus event/replay/conformance tests.  
**Mutation groups coupled:** `load.rs`, `serialization.rs`, `events/apply.rs`, `events/envelope.rs`.

The replacement evidence must prove every meaningful tested state transition is represented by an ordered event envelope with stable identity, schema version, stream/order/tick, causes, proposal/validation linkage where applicable, random-draw references where applicable, payload/effects, and content-manifest identity. Correction remains later-event causality, not history rewrite.

Required positive fixture families include `replay_item_location_001`, `container_item_move_001`, `door_access_001`, `strongbox_001`, `ordinary_workday_001`, `sleep_eat_work_001`, and `no_human_day_001`.

Required adversarial evidence includes duplicate IDs/order, malformed streams, missing required causes, unsupported schema versions, payloads that imply untyped mutation, and event attempts to inject prose-born or hidden-truth facts.

Required artifacts include canonical event-log fingerprints, envelope-field coverage, replayed state/projection checksums, and at least one typed first-divergence report. Failures must name `content/schema validation`, `action validation`, `event application`, `projection/replay`, `tests/fixtures`, or `doctrine mismatch` as applicable.

### 6.2 SPINE-02 — replay rebuild, deterministic identity, and divergence reporting

**Composed gates:** `REPLAY`, `FIXTURE`, `DIAG`.  
**Primary files:** `replay/mod.rs`, `replay/rebuild.rs`, `replay/report.rs`, `events/log.rs`, `events/apply.rs`, `checksum.rs`, content serialization/load, replay/golden/generative tests.  
**Mutation groups coupled:** `serialization.rs`, `checksum.rs`, `events/apply.rs`, `events/envelope.rs`, `replay/rebuild.rs`, `replay/report.rs`.

The replacement evidence must replay the required golden fixture corpus from persisted event history and declared content/schema identity. Duplicate runs of the same package must yield identical authoritative and projection fingerprints and the same typed report.

Adversarial packages must include: dropped event, reordered same-tick events with distinct ordering keys, changed manifest identity, unsupported schema version, nonexistent cause, and a semantic payload/checksum corruption. Host time/timezone, environment, filesystem discovery, process, network, and undeclared randomness must remain excluded by the existing negative-fixture posture.

The artifact must record expected/actual state and projection checksums, event/diagnostic counts, schema failures, `matches_expected`, and first divergence. Corrupt packages must fail; there is no repaired-success path.

`generative_lock` and any added property/metamorphic family remain complementary to named fixtures. Their seed set and generation scope must be recorded, and sampled evidence must not be mislabeled exhaustive.

### 6.3 SPINE-03 — holder-known projection, sealed provenance, and non-truth-writer quarantine

**Composed gates:** `TFW`, `POS-PARITY`, `REPLAY`, `DIAG`.  
**Primary files:** `projections.rs`, `epistemics/projection.rs`, `epistemics/knowledge_context.rs`, `view_models.rs`, `debug_reports.rs`, `events/apply.rs`, replay/report and hidden-truth/conformance/TUI tests.  
**Mutation groups coupled:** `knowledge_context.rs`, `projections.rs`, `view_models.rs`, `debug_reports.rs`, `checksum.rs`, `replay/rebuild.rs`.

The replacement evidence must show projections are deterministic readers of event-derived state and sealed holder-known contexts, never authoritative truth writers. Context ID, context hash, event frontier, source/provenance entries, and forbidden-truth audit must travel into projection/view/proposal evidence.

Positive families include `view_filtering_001`, `view_model_local_actions_001`, `possession_parity_001`, `workplace_assignment_provenance_001`, `no_human_observation_facts_cite_log_events_001`, and source-backed local actor/food/sleep/route/workplace cases.

Negative families include hidden food/routes/assignments, unproven typed facts, stale facts, and paired hidden-truth worlds. Actor proposals and embodied views must remain invariant under hidden-truth-only changes until a causal information event changes the holder-known context. Debug may reveal the difference only in a quarantined channel.

Failures must name `actor-known context construction`, `projection/replay`, `view-model rendering`, `debug quarantine`, or `tests/fixtures`.

### 6.4 SPINE-04 — random-stream discipline

**Composed gates:** `REPLAY`, `FIXTURE`, `DIAG`.  
**Primary files:** event envelope/random-draw records, replay/checksum, negative banned-random entry-point fixtures, and any final state-affecting RNG call sites.  
**Mutation groups coupled:** `events/envelope.rs`, with replay/checksum coverage.

The implementing session must first re-establish which branch applies at the final commit:

1. no state-affecting RNG exists in the SPINE perimeter, proven by the existing source/negative-fixture guards; or
2. state-affecting RNG exists and every draw carries seed, stream, draw index, and replay record through the event envelope.

If branch 1 remains true, the artifact must record an exhaustive source/perimeter scan and banned-entry-point tests, while making no claim beyond the scanned/fingerprinted scope. If branch 2 applies, duplicate-run, altered-seed, stream-separation, and malformed-draw replay evidence becomes mandatory.

The random-draw envelope parser/serializer survivors must be killed under either branch because envelope schema integrity is a SPINE property even when current ordinary fixtures produce no state-affecting draw.

### 6.5 SPINE-05 — save package, manifest, schema, serialization, and content validation

**Composed gates:** `REPLAY`, `FIXTURE`, `DIAG`.  
**Primary files:** content `manifest.rs`, `load.rs`, `schema.rs`, `serialization.rs`, `validate.rs`, event/replay/checksum seams, content test suites.  
**Mutation groups coupled:** all five content survivor files plus event envelope/apply persistence fields.

The replacement evidence must prove declared content identity, schema version, fixture scope, canonical serialization, event-log persistence, and validation precede authoritative load. Every fixture in the current registered corpus must load through the real path or fail with a typed responsible-layer diagnostic.

Positive evidence includes full fixture load, canonical serialize-deserialize-replay round trips, stable manifest/fingerprint scope, historical fixture-scope handling, and deterministic ordering.

Negative evidence includes missing/mismatched manifests, unsupported schema, corrupt event bytes, missing references, invalid topology/bounds, direct-state/script markers, prose-born facts, authored outcomes/player objectives, unproven epistemic seeds, nondeterministic ordering/duplicates, and round-trip drift.

Rejected content must produce no partial world state, seed event, actor-known fact, or direct action dispatch. Failures must name `content/schema validation`, `event application`, `projection/replay`, `tests/fixtures`, or `documentation status`.

### 6.6 SPINE-06 — proposal, validation, scheduling, event commitment, application, and feedback

**Composed gates:** `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `REPLAY`, `DIAG`.  
**Primary files:** `actions/proposal.rs`, `actions/report.rs`, `actions/pipeline.rs`, action definitions, `scheduler.rs`, `events/apply.rs`, `projections.rs`, actor transaction/planning surfaces, pipeline/no-human/hidden-truth tests.  
**Mutation groups coupled:** proposal, report, scheduler, apply, projections, knowledge context, relevant content validation.

The replacement evidence must record at least one accepted autonomous proposal, one accepted TUI semantic proposal, one rejected proposal, and one scheduled passive/completion event. For each, capture proposal source, holder-known context ID/hash/frontier, checked facts, validation report ID, appended event IDs, application result, actor-visible feedback, and debug-only diagnostics.

Positive fixture families include `sleep_eat_work_001`, `ordinary_workday_001`, `work_block_failed_then_sleep_succeeds_001`, `sleep_interrupted_by_severe_need_prorates_recovery_001`, `work_completion_fails_when_actor_displaced_001`, `severe_safety_with_known_exit_produces_move_001`, `routine_blocked_diagnostic_001`, and `method_fallback_requires_new_trace_or_stuck_001`.

Negative families include `no_hidden_truth_planning_001`, `forbidden_provenance_input_fails_closed_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`, `prose_born_fact_rejected_001`, and `scheduler_cannot_rewrite_wait_reason_after_transaction_001`.

Truth may validate success or rejection, but it may not select the proposal or leak validator-only facts into replanning. Failures must name actor-known construction, candidate generation, planning/method selection, proposal construction, scheduler ordering, action validation, event application, or projection/replay.

### 6.7 SPINE-07 — embodied TUI, view models, transcripts, and debug split

**Composed gates:** `TFW`, `POS-PARITY`, `PIPE`, `DIAG`.  
**Primary files:** TUI `app.rs`, `render.rs`, `debug_panels.rs`, `transcript.rs`, input/run/launch seams, core `view_models.rs`, `debug_capability.rs`, `debug_reports.rs`, projections/proposal/pipeline, TUI and hidden-truth tests.  
**Mutation groups coupled:** view models, debug reports, TUI render/transcript, knowledge context/projections/proposal.

The replacement evidence must show semantic input -> proposal -> holder-known source context -> shared pipeline -> appended/applied event -> refreshed event-frontier view. The TUI remains a client and proof surface, not simulation authority.

Positive families include `view_model_local_actions_001`, `embodied_view_omits_raw_assignment_without_context_001`, `embodied_menu_lags_truth_change_without_perception_001`, `possession_parity_001`, and `debug_attach_001`, plus command-loop and transcript suites.

Negative families include `debug_omniscience_excluded_001`, hidden food/route/workplace/affordance omissions, direct TUI apply attempts, and paired embodied/debug snapshots from the same state. Ordinary transcript output must not contain debug-only facts or non-diegetic truth comparisons.

The artifact must record channel markers, holder-known context identity, semantic section structure, debug capability construction/quarantine, and possession neutrality. Failures must name view-model rendering, debug quarantine, actor-known construction, proposal construction, or action validation.

### 6.8 SPINE-08 — no-direct-dispatch and mutation-path closure

**Composed gates:** `PIPE`, `NO-DIRECT`, `TFW`, `DIAG`.  
**Primary files:** `events/mutation.rs`, `events/apply.rs`, `actions/pipeline.rs`, `actions/proposal.rs`, `scheduler.rs`, `controller.rs`, `debug_capability.rs`, `state.rs`, content load, TUI app, negative external-crate fixtures, conformance/CI guards.  
**Mutation groups coupled:** apply, proposal, report, scheduler, load, content validation, plus zero-survivor mutation-capability boundaries.

The replacement evidence must prove closure across all legal callers, not only one happy path. Record one legal actor path, one legal TUI path, one legal scheduled path, and one legal replay-application path. In each case, event append/commit must precede authoritative application and the internal mutation capability must remain kernel-owned.

Negative compile/source/runtime evidence must continue to prove external crates cannot forge mutation capability, call post-load seed mutators, mutate seed maps, insert raw epistemic records, read raw projection maps, or directly apply world mutations from TUI/content/debug/projection/action definitions.

A validation failure may yield modeled feedback but may not trigger a hidden-truth fallback action. A scheduler may append permitted due events but may not construct cognition from raw state.

Failures must name doctrine mismatch, proposal construction, scheduler ordering, action validation, event application, view-model rendering, debug quarantine, tests/fixtures, or documentation status.

---

## 7. Gate evidence package and command contract

### 7.1 Mandatory command set

The implementing session must record, at minimum:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked

cargo test --locked -p tracewake-core --test acceptance_artifact_wording
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test ci_workflow_guards
cargo test --locked -p tracewake-core --test doc_invariant_references
cargo test --locked -p tracewake-core --test emergence_ledger
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test spine_conformance
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

cargo mutants --workspace --list-files
cargo mutants --workspace --list
cargo mutants --workspace --no-shuffle
```

Sharded equivalents are allowed under section 4.6. The artifact must preserve the exact commands actually executed, not only this intended vocabulary.

### 7.2 Every gate-evidence requirement from execution document `03`

The replacement package must visibly contain all of the following:

| Required element | 0039 obligation |
|---|---|
| Exact files and seams audited | final perimeter census, changed-file list, and SPINE-01…08 per-seam file lists. |
| Foundation and architecture dependencies | explicit dependency block and invariant cross-references for each seam/group. |
| Required artifact dependencies | command transcripts, mutation outputs, replay/log/checksum packages, provenance/context records, transcript/debug packages, and observer-only `EMERGE-OBS` where the corpus exercises living-world acceptance. |
| Positive and negative fixtures | named fixture matrices in sections 5 and 6, including branch-isolating content negatives and hidden-truth/direct-dispatch adversaries. |
| Event/replay/projection evidence | event envelopes/logs, checksums, replay reports/first divergence, projection/context fingerprints. |
| Actor-known provenance evidence | holder-known context ID/hash/frontier, source keys, source event ancestry, proposal context binding. |
| Debug quarantine evidence | paired embodied/debug channels, non-diegetic markers, capability/API boundary, no feedback into simulation. |
| Failure diagnostics by responsible layer | every failed mutant/test/replay/fixture row names an existing execution `03` layer. |
| Archived history statement | 0005–0038 specs/tickets and 0038 pass rows are historical only. |
| Tolerated deferrals tied to named gates | section 11; no SPINE obligation or survivor is deferred. |

Artifact dependencies are package members, not new phase gates.

### 7.3 Fixture corpus and exhaustiveness declarations

The full registered golden fixture corpus must run. The artifact must separately identify:

- exhaustive fixture registry execution;
- exhaustive mutation perimeter and survivor disposition;
- exhaustive enum/parser/checked-fact/fact-family tables where claimed;
- finite named adversarial matrices;
- multi-seed generative evidence, with seeds/population and omissions; and
- observer-only emergence evidence.

A sampled or generative corpus may add confidence but may not be relabeled exhaustive. The acceptance-template `sampling/exhaustiveness` field governs every evidence item.

### 7.4 Failure handling

A failure must be attached to an existing responsible layer:

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
- documentation status.

A failed mutation or seam may not be recast as a phase exception. The artifact remains `SPINE-CERT scoped remediation` until the blocker is closed.

### 7.5 Evidence-claim packaging

The replacement artifact should organize evidence as an explicit claim-to-evidence structure: each seam or mutation disposition cites the exact command, artifact, behavior witness, negative control, provenance ancestry, and scope that supports it. This follows the repository's own acceptance template and is consistent with structured assurance practice in which evidence is tied to explicit claims rather than presented as an undifferentiated test dump.[^nist-assurance]

---

## 8. Replacement acceptance artifact specification

### 8.1 Identity, path, and supersession

The implementing session must produce one replacement acceptance artifact. The expected hardening-series filename is:

```text
archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md
```

The exact filename is an owner decision, but the artifact must unambiguously identify this spec and the SPINE-CERT replacement role.

The artifact must state:

- exact final implementation commit;
- target/source baseline `9648c8fb75f6de06c77da4b20b4c30b783cd9217`;
- Rust/Cargo/cargo-mutants versions;
- config, CI, Cargo.lock, and relevant source fingerprints;
- all changed files;
- command transcript index;
- explicit supersession of `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md`; and
- the exact scope limitation: user-supplied target lineage, not independent latest-main verification.

### 8.2 Required acceptance-template fields

For every evidence item cited by a pass/fail row, include the fields required by `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:

- evidence status;
- fingerprint scope;
- evidence summary;
- path under test and behavior witness;
- replay/provenance ancestry;
- sampling/exhaustiveness scope;
- pending or historical handling;
- certification use; and
- staged-abstraction declaration when applicable.

Historical, pending, sampled, and observer-only items may inform review but may not silently count as certifying passes.

### 8.3 Mandatory mutation evidence row

The per-seam verdict table must contain a mutation row whose result is `pass` only when the full standing run and survivor register are complete.

The row must report tool outcomes and review dispositions separately.

**Tool-outcome ledger:**

- enumerated mutants;
- tested mutants;
- caught;
- missed;
- timeout;
- unviable;
- tool/run failures;
- shard count/completeness; and
- run-completion statement.

**Historical 296-seed reconciliation ledger:**

- total historical seed identities: 296;
- caught on the final clean posture;
- killed by remediation, as an explicitly identified subset/consequence;
- approved equivalent;
- approved non-critical;
- source-changed identities with explicit semantic mapping;
- blocked/untriaged; and
- reviewer-signoff completeness.

**New-survivor ledger:**

- total additional survivors discovered;
- killed by remediation;
- approved equivalent;
- approved non-critical;
- blocked/untriaged.

A passing row requires zero blocked/untriaged survivors and zero unresolved timeouts/tool failures. Counts must reconcile without double-counting: `killed by remediation` may be reported as a subset of final caught/reconciled seed identities and must be labeled as such.

For every approved equivalent or non-critical survivor, the artifact must cite its exact call-site argument and reviewer evidence. For every killed survivor, it must cite the member-specific failing behavior witness under the mutant.

The row must also state:

- the final run used the checked-in expanded configuration;
- the run completed over all shards or unsharded population;
- `--list-files` proved no silent exclusion;
- `test_workspace = true` or equivalent workspace-wide testing was active;
- the final certifying run did not use `--iterate`;
- baseline misses were not used to launder unresolved behavior-changing survivors; and
- the 296 historical count was treated as a floor, not the denominator of convenience.

### 8.4 Per-seam sections and aggregate verdict

The artifact must contain live SPINE-01 through SPINE-08 evidence sections and a capstone table. A seam row may pass only from certifying evidence at the final implementation commit.

The aggregate verdict may be:

```text
SPINE-CERT passed
```

only when:

- every SPINE-01…SPINE-08 row passes;
- the mutation row passes;
- all gate-evidence requirements are present;
- no pending/historical/observer-only item is counted as a pass; and
- no staged abstraction hides a SPINE obligation.

Otherwise the aggregate verdict remains:

```text
SPINE-CERT scoped remediation
```

with the remaining responsible layer and evidence gap named.

### 8.5 `EMERGE-OBS`

Where the full golden/no-human corpus exercises first-proof living-world behavior, include the required `EMERGE-OBS` artifact as `observer-only`. It may record replayable emergent ordinary-life phenomena and their event ancestry. It must not influence mutation scoring, action selection, scheduling, scenario goals, gate results, or the aggregate verdict.

### 8.6 Staged-abstraction declaration

The artifact must state either:

- no staged abstraction is relied upon for the SPINE pass; or
- exactly what future surface is bounded, what is proved now, what must not be faked, and how overclaiming is prevented.

No staged abstraction may cover an unresolved survivor, SPINE seam, replay/provenance gap, debug-quarantine gap, or no-direct-dispatch gap.

### 8.7 Certification use after supersession

Only after the replacement artifact exists and renders `SPINE-CERT passed` may later specs cite that line state and proceed to EPI-CERT. The artifact does not itself pass EPI-CERT, ORD-LIFE-CERT, FIRST-PROOF-CERT, PHASE-4-ENTRY, or SECOND-PROOF-ENTRY.

---

## 9. Preliminary static survey — informative, not certification

This section records representative exact-commit observations that shape test design. It is **preliminary, not certification**. It cannot substitute for the final source census, cargo-mutants execution, tests, replay, or reviewer disposition.

### 9.1 Actor-known fact and canonical-key cluster

At the target baseline, `knowledge_context.rs` defines stable IDs and canonical/source keys across context modes, source/provenance types, and actor-known fact structures for current place, carried item, workplace, food source, sleep affordance, route, door, container, item, and local actor. Context sealing exposes holder-known context ID, hash, frontier, provenance entries, and typed fact collections.

This shape makes the large cluster plausibly killable through a shared parameterized fact-family harness. The non-tautological consequence is not “the key equals this literal.” It is that changing a source/field/key changes context identity or downstream projection/proposal behavior while preserving hidden-truth isolation. Paired worlds and replayed contexts can make that consequence observable.

### 9.2 Event parser/apply cluster

`events/apply.rs` routes event streams through typed application functions and contains parsers for epistemic stance/channel/proposition, agent need/intention/source/routine fields, numeric/boolean payloads, and item/location transitions. The survivors therefore sit on a real envelope -> apply -> state/provenance -> replay path.

The likely killing strategy is an event-kind matrix with accepted and rejected arms, not isolated parser literal tests. Deleted match arms should produce typed apply/replay failures; precondition mutants should change whether authoritative state is updated and whether the failure diagnostic is emitted.

### 9.3 Serialization and validator clusters

`serialization.rs` includes canonical event-log serialization plus typed parsers for fixture scope, channel, stance, optional tick, routine family, privacy scope, and vector fields. `validate.rs` is a broad anti-contamination boundary. Static shape suggests most survivors are killable through branch-isolating fixture pairs and serialize-deserialize-replay contracts.

The central oracle must remain semantic: accepted content produces the expected typed state/events/provenance; a one-defect sibling fails at content/schema validation and produces no partial authoritative effects.

### 9.4 Debug/view/TUI cluster

`view_models.rs` exposes multiple debug view types whose `debug_only` state delegates to debug capability, plus non-diegetic markers and typed why-not/provenance values. The surviving flag/string mutants are easy to kill badly with getter assertions. The proper witness routes the same state through embodied and debug channels and proves noninterference: debug information is visible to review but cannot change actor-known context, proposals, checksums, or ordinary transcripts.

### 9.5 Replay/checksum/scheduler cluster

The replay and checksum survivors appear to affect context-hash rebuild, latest-event selection, checksum canonicalization, and mismatch reporting. Scheduler survivors affect boundary comparisons and event completion. These are suited to metamorphic relations: same log replayed twice is identical; unrelated events do not change latest relevant selection; semantically distinct status/location does change checksum; before/exact/after tick boundaries produce the specified completion behavior.

No static observation in this section determines equivalence. Final disposition belongs to the completed mutation campaign.

---

## 10. Constitutional-invariant alignment

The remediation may cite only existing invariant IDs. It coins none.

| Remediated surface | Principal invariant cross-references | Required preserved property |
|---|---|---|
| Event log/envelope/apply | `INV-009`, `INV-010`, `INV-011`, `INV-013`, `INV-015`, `INV-017`, `INV-020` | meaningful change is eventful, caused, traceable, schema-versioned, and not current-state-only. |
| Replay/checksum/serialization | `INV-018`, `INV-019`, `INV-092` | deterministic reconstruction and ancestry-preserving fingerprints with loud divergence. |
| Holder-known context/projection | `INV-002`, `INV-021`, `INV-023`, `INV-024`, `INV-026`, `INV-030`, `INV-099`, `INV-100`, `INV-101`, `INV-102`, `INV-112` | actor cognition uses sealed source-backed context; truth validates but does not plan. |
| Proposal/validation/scheduler | `INV-007`, `INV-008`, `INV-035`, `INV-036`, `INV-039`, `INV-043`, `INV-099`, `INV-103`, `INV-104`, `INV-106` | ordinary-agent-possible proposals flow through shared validation/events; scheduler and routines do not dispatch from truth. |
| Content/schema validation | `INV-022`, `INV-060`, `INV-061`, `INV-062`, `INV-097`, `INV-098` | no prose-born facts, authored outcome chains, or direct-state/script bypasses. |
| TUI/possession/view models | `INV-004`, `INV-005`, `INV-006`, `INV-065`, `INV-067`, `INV-068`, `INV-069`, `INV-070`, `INV-094`, `INV-095`, `INV-108` | possession is cognition-neutral; embodied display is actor-known; debug is quarantined; UI is not authority. |
| Diagnostics/debug reports | `INV-031`, `INV-041`, `INV-105`, `INV-107` | diagnostics are typed/inspectable and debug omniscience remains non-diegetic. |
| No-human and ordinary fixture corpus | `INV-001`, `INV-003`, `INV-091`, `INV-098` | causal ordinary behavior runs without human privilege and remains harshly acceptance-tested. |
| Observer-only emergence package | `INV-111` | emergence evidence is retrospective and observer-only, never a behavior objective or gate threshold. |

A genuine conflict with an invariant blocks implementation and requires upstream doctrine work; this spec may not silently weaken the invariant to make a mutant easy to kill.

---

## 11. Out of scope and tolerated deferrals

No forward gate is advanced by this spec. The only deliverable effect is remediation and replacement certification of the SPINE line.

| Deferred surface | Entry/certification lock | 0039 treatment |
|---|---|---|
| Deep epistemic-substrate proof | EPI-CERT, after SPINE-CERT passed | out of scope; `knowledge_context.rs` is tested only for its SPINE holder-known/projection/pipeline role. |
| Ordinary-life certification line | ORD-LIFE-CERT | out of scope; ordinary fixtures are evidence dependencies, not a new ORD-LIFE verdict. |
| Aggregate first-proof certification | FIRST-PROOF-CERT | out of scope; first-proof labels are composed only as existing evidence cross-references. |
| Institutions, records, wrong suspicion | PHASE-4-ENTRY | locked; no institution feature or proof expansion. |
| Notices, travel, regional scale, LOD, second proof | SECOND-PROOF-ENTRY and execution document `12` | locked and deferred. |
| LLM dialogue/speech rendering | later LLM/speech gates | out of scope. |
| Leads/story-sifting and 0035 forward expansion | later admissible expansion after the ladder permits it | out of scope. |

Tolerated staged abstractions may describe these future surfaces, but none may defer a SPINE survivor or evidence obligation. A survivor in a file that also feeds a future gate is remediated only for its reachable SPINE role here.

---

## 12. Risks and open questions

### 12.1 Risks to control

1. **Tautological killing tests.** Getter-asserts-getter, nonempty collection, or exact prose assertions can catch syntax without proving certified behavior.
2. **Equivalent-mutant overclaim.** At this scale, equivalence review is costly and vulnerable to “no test failed” reasoning.
3. **Grouped-family undercoverage.** A table may kill one representative while silently missing other members.
4. **Additional survivors.** The completed configured run may surface more than 296 and re-block the line.
5. **Cross-crate witness omission.** Package-local mutation testing can miss content/TUI consequences of core mutants unless workspace tests are active.
6. **Run-cost inconsistency.** Divergent shard arguments, timeouts, tool versions, or skipped baselines can make aggregate evidence meaningless.
7. **Baseline laundering.** Broad normalized matching or unreviewed baseline entries can turn live defects into accepted misses.
8. **Metric gaming.** Mutation score, killed subset, or disappearance of a syntactic mutant can replace semantic closure.
9. **Source-discipline relapse.** Historical 0038 pass rows or provenance SHAs may be cited as live proof.
10. **Configuration/CI drift.** Checked-in globs and workflow shell lists can diverge unless one is authoritative and guarded.
11. **Seed-identity loss.** Refactors can make historical mutants hard to map and tempt silent omission.
12. **Tool-version drift.** A cargo-mutants upgrade can change the census/operator set; unexplained count changes undermine comparison.
13. **Flaky or non-hermetic tests.** Mutation timeouts and inconsistent outcomes can be mistaken for kills.
14. **Snapshot overfitting.** Exact text snapshots may catch harmless wording while missing semantic provenance or quarantine.
15. **Mutation evasion in production code.** Code can be reshaped solely to suppress mutant generation without strengthening behavior.
16. **Firewall regression while killing a mutant.** A test or fix may make a view/proposal pass by reading truth or debug state.
17. **Overclaim from future-gate overlap.** Closing a SPINE consequence in `knowledge_context.rs` may be wrongly described as EPI-CERT evidence.

### 12.2 Open owner decisions

The following are implementation/closeout decisions not settled by current doctrine and must not be invented by this spec:

- exact shard count and runner allocation;
- explicit timeout values or multipliers;
- exact replacement acceptance-artifact filename;
- exact mutation-output retention location and duration;
- independent reviewer-signoff procedure for equivalent/non-critical dispositions;
- final triage-register path and machine-readable companion format;
- scheduled mutation frequency and exact in-diff workflow mechanics;
- whether approved equivalent/non-critical entries remain in `.cargo/mutants-baseline-misses.txt` or a separate reviewed allowlist;
- cargo-mutants version upgrade policy beyond recording and re-census; and
- whether additional property/generative test libraries are introduced or the existing deterministic harness is extended.

These decisions may change execution cost or evidence packaging. They may not weaken the full perimeter, completed-run, member-by-member disposition, workspace-test, or re-proof requirements.

---

## 13. Completion checklist for the implementing session

### Status and source discipline

- [ ] The work remains `Remediation` with admissibility posture `SPINE-CERT scoped remediation` until replacement closeout.
- [ ] Implementation lineage starts from target baseline `9648c8fb75f6de06c77da4b20b4c30b783cd9217` and records the exact final commit.
- [ ] No claim independently verifies current `main`.
- [ ] 0038 provenance commits and all archived specs/tickets are historical only.
- [ ] P0-CERT is not reopened.

### Permanent mutation posture

- [ ] `.cargo/mutants.toml` contains the full standing union perimeter.
- [ ] No exclusion overlaps a required SPINE file.
- [ ] `test_workspace = true` or an equivalent workspace-wide test posture is active.
- [ ] Scheduled CI uses the checked-in configuration rather than a divergent `-f` list.
- [ ] In-diff CI detects every standing-perimeter path and uses the same config.
- [ ] CI guard tests protect perimeter, workspace testing, baseline discipline, and artifacts.
- [ ] `--list-files` and `--list` outputs are archived and match the expected census.
- [ ] The final run completes without `--iterate`.
- [ ] Every shard is complete and consistent, or the run is unsharded.
- [ ] Zero unresolved timeouts/tool failures remain.

### Survivor remediation

- [ ] All 296 historical survivor identities are reconciled.
- [ ] All additional completed-run survivors are registered.
- [ ] Every behavior-relevant survivor is killed by a real behavior/provenance witness.
- [ ] Every grouped family has a member-by-member kill matrix.
- [ ] No getter tautology, nonempty-only assertion, or test-only bypass is accepted.
- [ ] Hidden-truth/prose/direct-dispatch/debug risks have negative controls.
- [ ] Every equivalent/non-critical exception names exact call sites and has independent signoff.
- [ ] Baseline misses contain no unresolved behavior-changing survivor.
- [ ] Zero blocked/untriaged survivors remain for a pass claim.

### SPINE re-proof and evidence

- [ ] SPINE-01 through SPINE-08 are re-proven at the final implementation commit.
- [ ] Exact audited files/seams and changed files are listed.
- [ ] Foundation/architecture dependencies and invariant cross-references are present.
- [ ] Positive and negative fixtures are recorded.
- [ ] Event/replay/projection evidence is recorded.
- [ ] Actor-known context ID/hash/frontier and source ancestry are recorded.
- [ ] Paired embodied/debug quarantine evidence is recorded.
- [ ] Failures, if any, name an existing responsible layer.
- [ ] Tolerated deferrals are tied to named later gates.
- [ ] `EMERGE-OBS`, where present, is observer-only and non-certifying.

### Replacement artifact

- [ ] The artifact conforms to every `docs/4-specs/0003` evidence field.
- [ ] Tool outcomes and review dispositions are separate and reconcile.
- [ ] The mutation row states full completion and zero untriaged survivors.
- [ ] The per-seam table contains live SPINE-01…08 rows.
- [ ] The artifact explicitly supersedes the 0038 acceptance artifact.
- [ ] The aggregate verdict is `SPINE-CERT passed` only if every condition passes.
- [ ] No later gate is claimed or advanced by the artifact.
- [ ] This spec is archived from `specs/0039_…` to `archive/specs/0039_…` only on accepted closeout.

---

## Appendix A — exact-commit authoring evidence ledger

**Requested repository:** `joeloverbeck/tracewake`  
**Target commit:** `9648c8fb75f6de06c77da4b20b4c30b783cd9217`  
**Freshness claim:** user-supplied target commit only; not independently verified as latest main  
**Manifest role:** path inventory only  
**Repository metadata used:** no  
**Default-branch lookup used:** no  
**Branch-name file fetch used:** no  
**Code search used:** no  
**Clone used:** no  
**URL fetch method:** full exact-commit URL fetch  
**Contamination observed:** no  
**Connector/tool namespace trusted as evidence:** no

Every primary path named in section 2 was present in the uploaded manifest inventory and was fetched at the target commit before being used. Every authoring URL used the exact base:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/
```

Fetched exact-commit paths used as evidence:

- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/4-specs/SPEC_LEDGER.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/reports/0038_spine_cert_mutation_triage_register.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/README.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/3-reference/01_DESIGN_RISK_REGISTER.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/3-reference/02_GLOSSARY.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/4-specs/README.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/.cargo/mutants.toml>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/.cargo/mutants-baseline-misses.txt>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/.github/workflows/ci.yml>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/epistemics/knowledge_context.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-content/src/validate.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/events/apply.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/view_models.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/debug_reports.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-content/src/serialization.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/actions/report.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/events/envelope.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-content/src/load.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/scheduler.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/replay/rebuild.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/checksum.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/projections.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/actions/proposal.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-content/src/schema.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/src/render.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/replay/report.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/src/transcript.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/events/log.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/events/mod.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/events/mutation.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/replay/mod.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/actions/pipeline.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/debug_capability.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-content/src/manifest.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/src/app.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/src/debug_panels.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/agent/actor_known.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/epistemics/projection.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/state.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/agent/no_human_surface.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/agent/transaction.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/src/controller.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/src/run.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/anti_regression_guards.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/hidden_truth_gates.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/event_schema_replay_gates.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/acceptance_gates.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/no_human_capstone.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/spine_conformance.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/generative_lock.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/golden_scenarios.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/negative_fixture_runner.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-core/tests/ci_workflow_guards.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-content/tests/fixtures_load.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-content/tests/forbidden_content.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-content/tests/golden_fixtures_run.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-content/tests/schema_conformance.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/tests/adversarial_gates.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/tests/tui_seam_conformance.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/tests/transcript_snapshot.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/tests/tui_acceptance.rs>
- <https://raw.githubusercontent.com/joeloverbeck/tracewake/9648c8fb75f6de06c77da4b20b4c30b783cd9217/crates/tracewake-tui/tests/command_loop_session.rs>

The exact-URL fetch ledger contained no result, redirect, or visible source from another repository. Unsuccessful readable-view fallbacks were not used as evidence.

---

## Appendix B — external method sources

These sources shape implementation method only. Tracewake doctrine remains controlling.

The external method conclusions used by this spec are deliberately narrow:

- checked-in configuration and file-census commands are used to make the perimeter reviewable;
- workspace-wide tests are required because certified consequences cross crate boundaries;
- sharding is acceptable only as a reproducible partition of one denominator;
- final evidence retains structured per-mutant outputs;
- equivalent/redundant mutants require semantic review rather than score-based acceptance;
- parameterized, property-based, metamorphic, and noninterference-style tests may efficiently express large behavior families, but member-specific mutant evidence remains required; and
- the replacement package maps explicit claims to exact evidence and scope.

[^cargo-config]: cargo-mutants, “Config file,” documents the checked-in `.cargo/mutants.toml` posture and `--no-config`. <https://mutants.rs/config-file.html>

[^cargo-files]: cargo-mutants, “Skip or examine some source files,” documents `examine_globs`, `exclude_globs`, `--list-files`, `--list`, and command-line file filtering. <https://mutants.rs/skip_files.html>

[^cargo-workspaces]: cargo-mutants, “Cargo workspaces,” documents package-local default testing and `test_workspace = true` for running all workspace tests against each mutant. <https://mutants.rs/workspaces.html>

[^cargo-baseline]: cargo-mutants, “Baseline tests,” explains the unmutated baseline and the risk of skipping it without independent proof. <https://mutants.rs/baseline.html>

[^cargo-output]: cargo-mutants, “The mutants.out directory,” lists structured mutant/outcome JSON, diffs, logs, and outcome files used for review. <https://mutants.rs/mutants-out.html>

[^cargo-shards]: cargo-mutants, “Sharding,” requires the same arguments and denominator across all shards. <https://mutants.rs/shards.html>

[^cargo-timeouts]: cargo-mutants, “Hangs and timeouts,” describes timeout derivation and controls; this spec treats timeout as an investigation/disposition requirement, not a pass. <https://mutants.rs/timeouts.html>

[^cargo-iterate]: cargo-mutants, “Iterating through missed mutants,” describes `--iterate` as a feedback optimization; this spec reserves it for remediation and requires a final complete run. <https://mutants.rs/iterate.html>

[^cargo-results]: cargo-mutants, “Using the results,” distinguishes missed mutants from proven defects/equivalents and recommends testing externally observable behavior rather than private implementation details. <https://mutants.rs/using-results.html>

[^quickcheck]: Claessen and Hughes, “QuickCheck: A Lightweight Tool for Random Testing of Haskell Programs,” introduced executable property families over generated cases. <https://dl.acm.org/doi/10.1145/351240.351266>

[^property-mutation]: Rezaalipour and Furia, “Property-Based Mutation Testing,” studies the interaction between property-based testing and mutation adequacy. <https://arxiv.org/abs/2301.13615>

[^metamorphic]: Segura et al., “A Survey on Metamorphic Testing,” surveys relation-based test oracles useful when exact outputs are cumbersome, including invariance and transformation relations. <https://eprints.whiterose.ac.uk/id/eprint/110335/1/segura16-tse.pdf>

[^noninterference]: Hedin and Sands, “Testing Noninterference, Quickly,” describes property-based checks for whether secret/input changes can influence public observations; this spec applies only the testing pattern to hidden-truth versus actor-visible outputs, subordinate to Tracewake doctrine. <https://arxiv.org/abs/1409.0393>

[^madeyski-equivalent]: Madeyski et al., “Overcoming the Equivalent Mutant Problem: A Systematic Literature Review and a Comparative Experiment of Second Order Mutation,” reviews the difficulty and limits of equivalent-mutant techniques. <https://dl.acm.org/doi/10.1109/TSE.2013.44>

[^papadakis-survey]: Papadakis et al., “Mutation Testing Advances: An Analysis and Survey,” reviews equivalent/redundant mutants, mutation-score distortion, and mutation-testing practice. <https://mutationtesting.uni.lu/survey.pdf>

[^just-redundant]: Just, Kapfhammer, and Schweiggert, “Do Redundant Mutants Affect the Effectiveness and Efficiency of Mutation Analysis?” analyzes how redundancy can distort mutation analysis. <https://homes.cs.washington.edu/~rjust/publ/non_redundant_mutants_icst_2012.pdf>

[^kintis-compiler]: Kintis et al., “Detecting Trivial Mutant Equivalences via Compiler Optimisations,” evaluates compiler-based equivalence support and its limits. <https://orbilu.uni.lu/bitstream/10993/31623/1/07882714.pdf>

[^nist-assurance]: NIST IR 7608, *Software Assurance Using Structured Assurance Case Models*, describes explicit claim/argument/evidence organization. <https://nvlpubs.nist.gov/nistpubs/Legacy/IR/nistir7608.pdf>

---

## Expected closeout effect

This spec commissions remediation; it records no implementation outcome. When the implementing session satisfies every checklist item and the replacement artifact renders `SPINE-CERT passed`, the 0038 scoped-remediation artifact is superseded and the certification ladder may advance to EPI-CERT. Until then, the live line remains `SPINE-CERT scoped remediation`.
