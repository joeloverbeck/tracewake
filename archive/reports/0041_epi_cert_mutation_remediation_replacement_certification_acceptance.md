# 0041 EPI-CERT mutation remediation replacement certification acceptance artifact

Spec: `archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
Spec number: `0041`  
Status: COMPLETED  
Source baseline: `7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5`  
Exact implementation/evidence commit under test: `726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3`  
Gate label under certification: `EPI-CERT`  
Verdict: `EPI-CERT passed`

This artifact supersedes
`archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`
for current EPI-CERT use while preserving it as historical lineage. Archived
specs/tickets `0004`-`0040` and their command/results are historical context or
structural precedent only, not live certification.

archived specs/tickets `0004`–`0040` are historical/precedent only.

This artifact does not independently verify latest `main`, does not certify
later `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, Phase-4 entry, second-proof entry,
institutions, notices, travel, LOD, LLM/speech, or story-sifting scope.

## Scope And Worktree Note

Artifact archive path:
`archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`

The exact commit under test is `726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3`.
At command time the worktree also contained unrelated unstaged local edits under
`.claude/skills/spec-to-tickets/`; those files are outside this artifact's
implementation, evidence, Rust workspace, and report scope and are excluded from
the certification claim.

## Environment And Static Fingerprints

| Item | Value | Fingerprint scope |
|---|---|---|
| Rust | `rustc 1.93.0 (254b59607 2026-01-19)` | command transcript |
| Cargo | `cargo 1.93.0 (083ac5135 2025-12-15)` | command transcript |
| cargo-mutants | `cargo-mutants 27.1.0` | command transcript |
| `Cargo.lock` | `f207f6208d64c1a7cce0daa27c00add40a52edb99b1a6ce3a1301cb44ef60a59` | raw bytes |
| `.cargo/mutants.toml` | `3883c275eae1193e41ee5d47992e397ac43a4b05ec1885d0e093ca1fe06b5a93` | raw bytes |
| `.cargo/mutants-baseline-misses.txt` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | raw bytes; empty |
| `reports/0041_epi_cert_mutation_list.txt` | `190906baa9e89be89014af6837f0fcb46532ed878d0b5c134c11da03a9ef387d` | raw bytes |
| `reports/0041_epi_cert_mutation_list_files.txt` | `9f3335f0733d969537595a77e1af0d61e1faf3c92561f1e79f97afbbac917e9d` | raw bytes |
| `reports/0041_epi_cert_mutation_triage_register.md` | `e6a5f48fa064beed79105020b585ffd1f3e6b885bc2157e5635edcd34f23fed3` | raw bytes |
| `reports/0041_epi_cert_mutation_full.out/mutants.out/outcomes.json` | `af1cb141a2fd7683199b07484d57bf04b041a8b6fb1e4f02a085b88119028e2f` | raw bytes |

## Command Ledger

| Command | Result | Evidence ID | Notes |
|---|---|---|---|
| `cargo fmt --all --check` | pass | `CMD-FMT` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | `CMD-CLIPPY` | exit 0; finished dev profile |
| `cargo build --workspace --all-targets --locked` | pass | `CMD-BUILD` | exit 0; finished dev profile |
| `cargo test --workspace --locked` | pass | `CMD-WORKSPACE-TEST` | exit 0; workspace tests and doctests passed |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates --test event_schema_replay_gates --test acceptance_gates --test anti_regression_guards --test generative_lock --test golden_scenarios --test negative_fixture_runner --test spine_conformance --test no_human_capstone --test emergence_ledger` | pass | `CMD-EPI-CORE` | 10 named core EPI binaries passed |
| `cargo test --locked -p tracewake-content --test fixtures_load --test forbidden_content --test golden_fixtures_run --test schema_conformance` | pass | `CMD-EPI-CONTENT` | 4 named content EPI binaries passed |
| `cargo test --locked -p tracewake-tui --test adversarial_gates --test tui_seam_conformance --test transcript_snapshot --test tui_acceptance --test embodied_flow --test command_loop_session` | pass | `CMD-EPI-TUI` | 6 named TUI EPI binaries passed |
| `cargo mutants --workspace --no-shuffle --list-files > reports/0041_epi_cert_mutation_list_files.txt` | pass | `MUT-CENSUS-FILES` | 54 configured files |
| `cargo mutants --workspace --no-shuffle --list > reports/0041_epi_cert_mutation_list.txt` | pass | `MUT-CENSUS-LIST` | 2774 configured mutants |
| `cargo mutants --workspace --no-shuffle -o reports/0041_epi_cert_mutation_full.out` | pass | `MUT-FULL` | 2774 tested; 2182 caught; 592 unviable; 0 missed; 0 timeout |

## Evidence Item Ledger

- Evidence item ID: `CMD-BASELINE`
- EPI cross-references: artifact completeness
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `CMD-FMT`, `CMD-CLIPPY`, `CMD-BUILD`, and `CMD-WORKSPACE-TEST` all passed at exact commit `726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3`.
- Path under test and behavior witness: workspace formatting, lint, locked build, and full test suite; responsible layer `workspace/ci`.
- Replay/provenance ancestry: per-seam replay/provenance ancestry is cited by seam rows below.
- Sampling/exhaustiveness: full workspace command set required by spec 0041 section 4.4 and AGENTS guidance.
- Certification use: counted as certifying pass for artifact completeness.

- Evidence item ID: `EPI-NAMED-SUITES`
- EPI cross-references: EPI-01 through EPI-11
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `CMD-EPI-CORE`, `CMD-EPI-CONTENT`, and `CMD-EPI-TUI` passed; all 20 named EPI binaries were re-run live at `726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3`.
- Path under test and behavior witness: core hidden-truth, event/replay, acceptance, mutation-lock, golden, negative, conformance, no-human, and emergence suites; content fixture/load/forbidden/schema suites; TUI adversarial, seam, transcript, acceptance, embodied, and command-loop suites.
- Replay/provenance ancestry: event-schema, golden scenario, golden fixture, no-human, and transcript suites exercise serialized event logs, projection rebuild, context hashes, fixture fingerprints, and deterministic transcripts.
- Sampling/exhaustiveness: exhaustive over the spec 0041 section 4.4 named binary list.
- Certification use: counted as certifying pass for all seam rows where cited.

- Evidence item ID: `NEG-CORPUS`
- EPI cross-references: EPI-01, EPI-02, EPI-03, EPI-05, EPI-06, EPI-10, EPI-11
- Evidence status: pass
- Fingerprint scope: command transcript and registered fixture corpus
- Evidence summary: `negative_fixture_runner` passed in `CMD-EPI-CORE`; registered fixtures include the EPI external-boundary cases listed in the compile-fail matrix below.
- Path under test and behavior witness: external crates cannot construct debug knowledge contexts or reports, construct belief/observation literals, insert raw epistemic records, mutate context mode/viewer or belief/contradiction internals, read raw projection maps, or introduce banned float confidence types.
- Replay/provenance ancestry: not applicable to compile-fail evidence; the boundary closes direct external construction before replay/projection state can be created.
- Sampling/exhaustiveness: registered negative-fixture corpus; `registered_negative_fixtures_match_directory_census` passed.
- Certification use: counted as certifying pass for compile-fail boundary requirements.

- Evidence item ID: `MUT-REGISTER`
- EPI cross-references: mutation, EPI-02, EPI-03, EPI-04, EPI-05, EPI-06, EPI-07, EPI-09
- Evidence status: pass
- Fingerprint scope: structured mutation output and triage register
- Evidence summary: `reports/0041_epi_cert_mutation_triage_register.md` reconciles all 30 historical identities; `outcomes.json` records 2774 total, 2182 caught, 592 unviable, 0 missed, 0 timeout.
- Path under test and behavior witness: standing `.cargo/mutants.toml` lock-layer denominator, including `crates/tracewake-core/src/epistemics/**`, content validation/serialization, event/replay, action, view-model, and TUI rendering/report paths.
- Replay/provenance ancestry: mutation evidence references the named behavior witnesses from tickets 002-008 and the full configured run from ticket 009.
- Sampling/exhaustiveness: full configured denominator; no `--no-config`, filter, `--in-diff`, `--iterate`, or exclusion in the certifying run.
- Certification use: counted as certifying pass for the completed-mutation row.

- Evidence item ID: `EMERGE-OBS`
- EPI cross-references: observer-only companion evidence
- Evidence status: observer-only
- Fingerprint scope: command transcript
- Evidence summary: `emergence_ledger` passed as part of `CMD-EPI-CORE`; it verifies observer-only emergence ledger behavior and replay byte identity.
- Path under test and behavior witness: observer-only living-world evidence beside EPI proof.
- Replay/provenance ancestry: replay byte identity is checked by the named test.
- Sampling/exhaustiveness: one named observer-only suite.
- Certification use: not counted as certifying evidence for seam pass/fail.

- Evidence item ID: `HIST-0040`
- EPI cross-references: historical lineage
- Evidence status: historical
- Fingerprint scope: archived artifact
- Evidence summary: the 0040 artifact rendered `EPI-CERT scoped remediation` because 30 mutation survivors remained.
- Path under test and behavior witness: historical lineage only.
- Replay/provenance ancestry: historical; not reused as current evidence.
- Sampling/exhaustiveness: not applicable.
- Certification use: not counted as certifying evidence.

## Per-Seam Verdict Matrix

Every row below is computed only from live `pass` evidence at exact commit
`726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3`. Historical and observer-only rows
do not pass a seam.

| Seam | Responsible layer | Positive evidence | Adversarial / negative evidence | Replay / provenance evidence | Mutation coupling | Result |
|---|---|---|---|---|---|---|
| EPI-01 sealed holder-known context | actor-known context construction; projection/replay | `hidden_truth_gates`, `acceptance_gates`, `golden_fixtures_run` via `EPI-NAMED-SUITES` | hidden truth/context injection and compile-fail context boundaries via `EPI-NAMED-SUITES`, `NEG-CORPUS` | `event_schema_replay_gates`, `golden_scenarios`, `golden_fixtures_run` | `MUT-REGISTER` | pass |
| EPI-02 typed propositions and beliefs | content/schema validation; projection/replay; view-model rendering | `golden_fixtures_run`, `hidden_truth_gates`, `event_schema_replay_gates` | malformed canonical propositions, float confidence, raw belief construction, source/scope mutation via `NEG-CORPUS` | event schema replay, belief stale frontier and witness links, fixture fingerprints | `MUT-REGISTER` rows for belief/proposition parse/render/validation | pass |
| EPI-03 observations | action validation; event application; projection/replay | `golden_scenarios`, `golden_fixtures_run`, `event_schema_replay_gates` | hidden truth without observation, raw observation construction, raw epistemic insertion via `NEG-CORPUS` | observation source IDs, confidence, replayed epistemic events | `MUT-REGISTER` rows for confidence, observation links, parser arms | pass |
| EPI-04 contradictions | projection/replay; view-model rendering | `golden_scenarios`, `event_schema_replay_gates` expected absence and contradiction tests | wrong holder/stance/location, hidden culprit/location, contradiction-link compile-fail boundary | contradiction IDs, prior belief/incoming observation links, replayed expected-absence evidence | `MUT-REGISTER` rows for contradiction/proposition truth tables and renderers | pass |
| EPI-05 provenance sufficiency | actor-known context construction; proposal construction; content/schema validation | workplace/no-human provenance and actor-known fact tests via `EPI-NAMED-SUITES` | wrong-kind, hidden/debug/prose, unknown-reference, and direct mutation boundaries via `NEG-CORPUS` | context hash, source event, fixture and replay evidence | `MUT-REGISTER` rows for empty links, validation bypasses, diagnostics | pass |
| EPI-06 replay-derived projection | event application; projection/replay | `event_schema_replay_gates`, `golden_scenarios`, `golden_fixtures_run` | omitted/reordered/corrupted event and raw projection boundary controls | live/rebuild projection and checksum equality | `MUT-REGISTER` rows for parser, serialization, render, and link fields | pass |
| EPI-07 actor decision transaction | candidate generation; proposal construction; action validation | `acceptance_gates`, `hidden_truth_gates`, `golden_fixtures_run` | hidden-route/food/workplace truth controls; validator truth split | sealed context hash/frontier and proposal/event replay evidence | `MUT-REGISTER` movement proposition parser and provenance rows | pass |
| EPI-08 possession parity | actor-known context construction; view-model rendering; action validation | `tui_acceptance`, `adversarial_gates`, `embodied_flow`, `golden_scenarios` | possession rebind does not transfer private knowledge; debug remains separate | semantic event/proposal/view parity and replay evidence | carried-by proposition rows in `MUT-REGISTER` | pass |
| EPI-09 embodied view models | projection/replay; view-model rendering; debug quarantine | `tui_acceptance`, `embodied_flow`, `transcript_snapshot`, `command_loop_session` | hidden food/route/workplace and stale-view controls | transcript determinism, view context/projection evidence | parser/render/confidence/freshness rows in `MUT-REGISTER` | pass |
| EPI-10 debug quarantine | debug quarantine; view-model rendering; event application | `adversarial_gates`, `tui_acceptance`, `command_loop_session` | debug construction/report/capability compile-fail boundaries via `NEG-CORPUS` | debug attach/read-only and transcript/replay noninterference | formatter/render rows reviewed to avoid debug-to-embodied leakage | pass |
| EPI-11 relational anti-contamination | first responsible layer encountered | paired hidden truth, possession, debug, stale-view, and no-human tests across `EPI-NAMED-SUITES` | other-actor privacy, debug attach, hidden route/food/workplace, possession rebind controls | live/replay equality before legal reveal, deterministic transcript and no-human replay evidence | all mutation families with no final survivors | pass |

## Completed Mutation Row

| Requirement | Evidence | Result |
|---|---|---|
| Configured denominator completed | `MUT-CENSUS-FILES`: 54 files; `MUT-CENSUS-LIST`: 2774 mutants; full run loaded checked-in `.cargo/mutants.toml` | pass |
| Full run completed | `MUT-FULL`: start `2026-06-19T21:14:12.33981041Z`, end `2026-06-19T23:39:18.066054579Z` | pass |
| Tool outcomes reconciled | 2774 total, 2182 caught, 592 unviable, 0 missed, 0 timeout, 0 tool failures | pass |
| Historical 30 reconciled | `reports/0041_epi_cert_mutation_triage_register.md`: 29 still-generated historical identities caught, 1 source-changed/removed identity mapped, 0 unresolved | pass |
| Baseline-miss integrity | `.cargo/mutants-baseline-misses.txt` empty | pass |
| Exceptions | 0 equivalent/non-critical exceptions claimed; no independent signoff needed | pass |

## Compile-Fail Boundary Matrix

`cargo test --locked -p tracewake-core --test negative_fixture_runner` passed as
part of `CMD-EPI-CORE`.

| Fixture family | Forbidden capability | Failure class | EPI seams | Gate cross-reference |
|---|---|---|---|---|
| `banned_float_confidence_types` | float confidence in epistemic confidence paths | compile/lint fail | EPI-02, EPI-03 | TFW, FIXTURE, DIAG |
| `external_crate_cannot_build_debug_knowledge_context` | external debug knowledge context construction | compile fail | EPI-01, EPI-10 | NO-DIRECT, TFW |
| `external_crate_cannot_build_debug_projection_view_without_core_debug_api` | external debug projection view construction without capability API | compile fail | EPI-06, EPI-10 | NO-DIRECT, debug quarantine |
| `external_crate_cannot_construct_belief_literal` | raw belief literal construction | compile fail | EPI-02, EPI-05 | NO-DIRECT, TFW |
| `external_crate_cannot_construct_debug_report` | raw debug report construction | compile fail | EPI-10 | debug quarantine |
| `external_crate_cannot_construct_observation_without_source` | source-less observation construction | compile fail | EPI-03, EPI-05 | NO-DIRECT, REPLAY |
| `external_crate_cannot_insert_raw_epistemic_records` | direct raw epistemic projection insertion | compile fail | EPI-03, EPI-06 | NO-DIRECT, REPLAY |
| `external_crate_cannot_mutate_belief_source_or_scope` | mutation of belief source/scope | compile fail | EPI-02, EPI-05 | TFW, DIAG |
| `external_crate_cannot_mutate_contradiction_links` | mutation of contradiction links | compile fail | EPI-04, EPI-05 | REPLAY, DIAG |
| `external_crate_cannot_mutate_knowledge_context_mode` | mutation of sealed context mode | compile fail | EPI-01 | TFW |
| `external_crate_cannot_mutate_knowledge_context_viewer` | mutation of sealed context viewer | compile fail | EPI-01, EPI-11 | TFW, POS-PARITY |
| `external_crate_cannot_read_raw_epistemic_projection_maps` | raw projection map reads | compile fail | EPI-02, EPI-06 | NO-DIRECT, REPLAY |
| debug capability doctests | minting/constructing debug capability outside the core path | compile fail | EPI-10 | debug quarantine |

## Reuse Declaration

| 0040 witness or artifact | Replacement use | Required action | Final evidence status |
|---|---|---|---|
| 0040 seam definitions and fixture names | structural context | re-run live commands at `726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3` | live `pass` in `EPI-NAMED-SUITES`; old rows `historical` |
| 0040 command transcript | historical comparison | run baseline and named commands anew | old transcript `historical`; new ledger live |
| 0040 replay/provenance evidence shape | evidence-shape reference | regenerate via event/replay/golden/TUI suites | old row `historical`; new row live |
| 0040 mutation register and final missed list | seed inventory | reconcile all 30 identities and append new survivors | historical seed plus live `MUT-REGISTER` |
| 0040 scoped-remediation verdict | blocker being closed | supersede only after live seam and mutation rows pass | historical after this replacement pass |

No reused-unchanged row certifies this replacement.

## Staged-Abstraction Declaration

None. This artifact does not rely on a staged abstraction to pass EPI-CERT.
Later certification lines remain locked to their own future gates.

## Observer-Only Evidence

`EMERGE-OBS` is observer-only. It is useful review evidence beside no-human and
living-world fixtures, but it is not a threshold, score, scheduler objective,
scenario goal, mutation substitute, or EPI seam pass/fail input.

## Aggregate Verdict

All required conditions hold:

1. Clean workspace baseline commands passed at the exact implementation/evidence commit.
2. All 20 named EPI suites passed live at the exact implementation/evidence commit.
3. The compile-fail boundary corpus passed and is mapped by forbidden capability.
4. The standing configured mutation run completed with zero missed mutants and zero timeouts.
5. The 30 historical mutation identities were reconciled with zero unresolved entries.
6. No baseline-miss laundering or unsigned exception is present.
7. EPI-01 through EPI-11 each have live positive, adversarial/negative, replay/provenance, and mutation-coupling evidence.

Therefore this replacement artifact renders `EPI-CERT passed` for exact commit
`726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3` and supersedes the 0040 EPI-CERT
acceptance artifact for current EPI-CERT use. It does not certify latest main or
any later certification line.
