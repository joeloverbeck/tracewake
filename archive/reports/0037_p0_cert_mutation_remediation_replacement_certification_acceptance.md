# 0037 P0-CERT mutation remediation replacement certification acceptance artifact

Spec: `archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`
Supersedes: `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md`
Supersession condition: this artifact renders P0-CERT passed after completing mutation remediation and live P0-01..P0-10 re-proof.
Repository: `joeloverbeck/tracewake`
Exact implementation commit under test: `a3b5e3c9e896d09ed9f5b6591fb6cd7a4edd0441`
Freshness claim: user-supplied remediation checkout only; not independently verified as latest main.
Verdict: P0-CERT passed
Verdict scope: post-0008 baseline mutation remediation only.
Archived evidence posture: historical only.
EMERGE-OBS posture: observer-only, non-gating, non-threshold, non-input.

P0-CERT post-0008 baseline mutation remediation accepted for exact commit
`a3b5e3c9e896d09ed9f5b6591fb6cd7a4edd0441`. This replacement artifact
certifies only the scoped post-0008 P0-CERT line described by spec `0037`; it
does not advance SPINE-CERT, EPI-CERT, ORD-LIFE-CERT, Phase-4 entry, second
proof, institutions, notices, travel, LOD, or LLM/speech surfaces.

## Evidence Status Legend

- `pass` - the artifact certifies the checked claim from live 0037 evidence.
- `fail` - the checked claim failed.
- `pending` - the check has not produced certifying evidence.
- `sampled` - representative, not exhaustive.
- `observer-only` - review evidence that cannot certify behavior.
- `historical` - archive/spec/ticket evidence used only as context.

Only `pass` rows count as certifying evidence.

## Gates Run

| Command | Result | Concise output summary |
|---|---|---|
| `cargo fmt --all --check` | pass | Passed with no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | Finished dev profile successfully. |
| `cargo build --workspace --all-targets --locked` | pass | Finished dev profile successfully. |
| `cargo test --workspace --locked` | pass | Workspace unit, integration, TUI, content, and doctest suites all reported `ok`; `hidden_truth_gates` reported 14 tests including the new local-actor witness. |
| `cargo test --locked -p tracewake-core --test negative_fixture_runner` | pass | 5 passed, 0 failed. |
| `cargo test --locked -p tracewake-core --test doc_invariant_references` | pass | 4 passed, 0 failed. |
| `cargo test --locked -p tracewake-core --test spine_conformance` | pass | 6 passed, 0 failed. |
| `cargo test --locked -p tracewake-core --test acceptance_gates` | pass | 12 passed, 0 failed. |
| `cargo test --locked -p tracewake-core --test anti_regression_guards` | pass | 80 passed, 0 failed. |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | pass | 17 passed, 0 failed. |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | pass | 14 passed, 0 failed; includes `actor_known_local_actor_reaches_embodied_view_model_with_context_provenance`. |
| `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` | pass | Pre-artifact run passed 4 tests, 0 failed; post-artifact run passed 5 tests, 0 failed, including this report. |
| `cargo test --locked -p tracewake-content --test schema_conformance` | pass | 2 passed, 0 failed. |
| `cargo test --locked -p tracewake-content --test forbidden_content` | pass | 20 passed, 0 failed. |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | pass | 40 passed, 0 failed; includes fixture fingerprints, context-hash, replay, hidden-truth, possession, and no-human witnesses. |
| `cargo test --locked -p tracewake-tui --test adversarial_gates` | pass | 15 passed, 0 failed. |
| `cargo test --locked -p tracewake-tui --test tui_seam_conformance` | pass | 2 passed, 0 failed. |
| `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle` | pass | Full configured unsharded posture completed: 1128 mutants tested in 66m, 896 caught, 232 unviable, 0 missed, 0 timeout. |

## Changed Files

- `crates/tracewake-core/tests/hidden_truth_gates.rs` - adds the public behavior/provenance witness that kills `0036-MUTATION-REMEDIATION-001`.
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs` - adds scoped wording coverage for this replacement artifact.
- `reports/0037_mutation_triage_register.md` - live mutation triage register for the full configured posture.
- `archive/tickets/0037P0CERTMUTREM-001.md` - archived killing-test ticket and outcome.
- `archive/tickets/0037P0CERTMUTREM-002.md` - archived full-mutation-posture ticket and outcome.
- `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` - this replacement artifact.

## Per-Requirement Acceptance Evidence

| Requirement | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `P0-01` proposal -> validation -> event -> application -> projection -> replay boundary | `proposal_construction`, `action_validation`, `event_append`, `event_application`, `projection`, `replay` | `0037-GATES-PREFLIGHT`, `0037-LOCK-CORE`, `0037-REPLAY-FIXTURE` | pass |
| `P0-02` autonomous decisions use sealed actor-known contexts with provenance | `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, `projection` | `0037-LOCAL-ACTOR-KILL`, `0037-HIDDEN-TRUTH`, `0037-REPLAY-FIXTURE` | pass |
| `P0-03` human-origin commands bind to ordinary actors and share action rules | `tui_input_binding`, `view_model`, `proposal_construction`, `action_validation` | `0037-LOCK-CORE`, `0037-TUI-ADVERSARIAL` | pass |
| `P0-04` possession never resets needs, intentions, memories, routines, or world-fact access | `tui_input_binding`, `holder_known_context`, `intention_lifecycle`, `view_model`, `projection` | `0037-TUI-ADVERSARIAL`, `0037-REPLAY-FIXTURE` | pass |
| `P0-05` scheduler paths cannot emit primitive actions from raw truth/routine labels/need thresholds/fixture tables | `scheduler`, `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, `action_validation` | `0037-LOCK-CORE`, `0037-REPLAY-FIXTURE`, `0037-MUTATION-FULL` | pass |
| `P0-06` validation truth may accept/reject/mutate through events but may not propose fallback plans or actor-visible hidden facts | `action_validation`, `event_application`, `holder_known_context`, `debug_quarantine` | `0037-LOCK-CORE`, `0037-HIDDEN-TRUTH`, `0037-TUI-ADVERSARIAL` | pass |
| `P0-07` debug surfaces are non-diegetic and cannot feed embodied/world surfaces | `debug_quarantine`, `view_model`, `holder_known_context`, `tui_input_binding`, `test_oracle` | `0037-HIDDEN-TRUTH`, `0037-TUI-ADVERSARIAL`, `0037-LOCAL-ACTOR-KILL` | pass |
| `P0-08` golden fixtures include positive and adversarial hidden-truth/no-human/possession/replay/view-model/content-validation/direct-dispatch cases | `fixture_contract`, `content_schema`, `content_validation`, `holder_known_context`, `test_oracle`, `view_model`, `replay` | `0037-REPLAY-FIXTURE`, `0037-CONTENT-SCHEMA`, `0037-TUI-ADVERSARIAL` | pass |
| `P0-09` failure diagnostics name responsible layer | `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `projection`, `view_model`, `test_oracle`, `debug_quarantine`, `replay`, `content_validation` | `0037-LOCK-CORE`, `0037-CONTENT-SCHEMA`, `0037-TUI-ADVERSARIAL` | pass |
| `P0-10` archived specs/tickets are historical evidence only | `source_discipline`, `test_oracle` | `0037-SOURCE-DISCIPLINE`, `0037-MUTATION-FULL` | pass |

## Evidence Item Ledger

### `0037-GATES-PREFLIGHT`

- Requirement IDs: `P0-01`, `P0-10`
- Evidence status: `pass`
- Fingerprint scope: command transcript
- Evidence summary: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, and `cargo test --workspace --locked` passed live after the `-001` and `-002` commits.
- Path under test and behavior witness: workspace compile/test surface, all crates, and all doctests under the locked dependency graph.
- Replay/provenance ancestry: workspace test output includes replay, fixture, no-human, hidden-truth, and TUI suites listed in this artifact.
- Sampling/exhaustiveness scope: full workspace test/build command set.
- Pending or historical handling: no pending P0 obligation in this row.
- Certification use: counted as certifying pass.

### `0037-LOCAL-ACTOR-KILL`

- Requirement IDs: `P0-02`, `P0-07`, `P0-10`
- Evidence status: `pass`
- Fingerprint scope: context hash/frontier
- Evidence summary: `0037P0CERTMUTREM-001` commit `2abfce9` added `hidden_truth_gates::actor_known_local_actor_reaches_embodied_view_model_with_context_provenance`; `cargo test --locked -p tracewake-core --test hidden_truth_gates` passed 14 tests.
- Path under test and behavior witness: sealed `KnowledgeContext` -> `EmbodiedProjectionSource::from_sealed_context` -> `build_embodied_view_model` -> `EmbodiedViewModel.local_actors`; the source-backed actor appears, while the physically co-located but not actor-known actor stays absent.
- Replay/provenance ancestry: the witness asserts holder-known context ID, hash, frontier, source-keyed current-place fact, and source-keyed local-actor fact.
- Sampling/exhaustiveness scope: targeted behavior/provenance witness for `0036-MUTATION-REMEDIATION-001`.
- Pending or historical handling: 0036's missed-mutant row is historical context only; it is superseded by this live kill and the full mutation posture.
- Certification use: counted as certifying pass.

### `0037-MUTATION-FULL`

- Requirement IDs: `P0-02`, `P0-05`, `P0-10`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus `mutants.out` result files
- Evidence summary: Full configured unsharded cargo-mutants posture completed with 1128 mutants tested, 896 caught, 232 unviable, 0 missed, and 0 timeout. `reports/0037_mutation_triage_register.md` records the command, counts, output files, and `0036-MUTATION-REMEDIATION-001` disposition.
- Path under test and behavior witness: configured guarded perimeter: `crates/tracewake-core/src/agent/**`, `scheduler*`, `projections*`, `actions/pipeline.rs`, and guarded `eat.rs`, `sleep.rs`, `work.rs`.
- Replay/provenance ancestry: `mutants.out/outcomes.json`, `caught.txt`, `missed.txt`, `timeout.txt`, and `unviable.txt`; `actor_known_local_actors_for_context -> vec![]` is caught, and `missed.txt`/`timeout.txt` are empty.
- Sampling/exhaustiveness scope: full configured posture, unsharded, no skipped denominator.
- Pending or historical handling: no missed or timeout survivors; no `0037P0CERTMUTREM-004` follow-up was required.
- Certification use: counted as certifying pass.

### `0037-LOCK-CORE`

- Requirement IDs: `P0-01`, `P0-03`, `P0-05`, `P0-06`, `P0-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript
- Evidence summary: Core lock-layer tests passed: `negative_fixture_runner` 5, `doc_invariant_references` 4, `spine_conformance` 6, `acceptance_gates` 12, `anti_regression_guards` 80, `event_schema_replay_gates` 17, `hidden_truth_gates` 14, and pre-artifact `acceptance_artifact_wording` 4.
- Path under test and behavior witness: action pipeline, direct-dispatch guards, scheduler/no-human boundaries, event schema/replay, responsible-layer diagnostics, holder-known context, hidden-truth gates, and acceptance wording guards.
- Replay/provenance ancestry: event schema replay gates and hidden-truth gates assert replay checksums, context hashes/frontiers, and typed source/provenance behavior.
- Sampling/exhaustiveness scope: named lock-layer gate set from spec §4.2.
- Pending or historical handling: post-artifact wording rerun is recorded in ticket/spec closeout because this file must exist before the gate can include it.
- Certification use: counted as certifying pass.

### `0037-REPLAY-FIXTURE`

- Requirement IDs: `P0-01`, `P0-02`, `P0-04`, `P0-05`, `P0-08`
- Evidence status: `pass`
- Fingerprint scope: replay artifact, fixture fingerprint, context hash/frontier
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed 40 tests, including fixture fingerprints, no-human replay, context hash, hidden-truth, possession, sleep/eat/work, and tamper negatives.
- Path under test and behavior witness: content fixtures, no-human ordinary-life fixture families, possession fixture, replay projection rebuild, event-log serialization, context-hash rebuild, and fixture fingerprint checks.
- Replay/provenance ancestry: fixture manifest and golden fingerprints; event logs; live/replay physical and agent checksums; decision context hash rebuilds; source-event-backed actor-known facts.
- Sampling/exhaustiveness scope: golden fixture corpus for first-proof ordinary-life and adversarial families.
- Pending or historical handling: EMERGE-OBS evidence from no-human/ordinary-life runs remains observer-only and is not a threshold or simulation input.
- Certification use: counted as certifying pass for P0 replay/fixture requirements; EMERGE-OBS posture is observer-only.

### `0037-HIDDEN-TRUTH`

- Requirement IDs: `P0-02`, `P0-06`, `P0-07`
- Evidence status: `pass`
- Fingerprint scope: context hash/frontier and typed negative guards
- Evidence summary: `hidden_truth_gates` passed 14 tests, covering actor-known context unforgeability, debug omniscience exclusion, hidden food/route/workplace negatives, debug truth exclusion from holder-known context hashes, and the local-actor projection witness.
- Path under test and behavior witness: actor-known context, epistemic projection, embodied view model, debug quarantine, planner local actions, and source-backed local actors.
- Replay/provenance ancestry: holder-known context hashes/frontiers, source-keyed observations/facts, and event-log-backed context construction in existing hidden-truth tests.
- Sampling/exhaustiveness scope: targeted hidden-truth and actor-known negative suite.
- Pending or historical handling: no hidden-truth P0 obligation deferred.
- Certification use: counted as certifying pass.

### `0037-CONTENT-SCHEMA`

- Requirement IDs: `P0-08`, `P0-09`
- Evidence status: `pass`
- Fingerprint scope: parsed semantic content and command transcript
- Evidence summary: `schema_conformance` passed 2 tests and `forbidden_content` passed 20 tests.
- Path under test and behavior witness: content schema mapping, fixture-scope registration, prose-born fact rejection, shortcut truth fields, hidden-truth source rejection, malformed epistemic seeds, player/script markers, and validation-policy classification.
- Replay/provenance ancestry: content fixture validation prevents invalid authored data from entering actor-known, replay, or fixture proof surfaces.
- Sampling/exhaustiveness scope: content/schema lock-layer gate set from spec §4.2.
- Pending or historical handling: no content-validation P0 obligation deferred.
- Certification use: counted as certifying pass.

### `0037-TUI-ADVERSARIAL`

- Requirement IDs: `P0-03`, `P0-04`, `P0-06`, `P0-07`, `P0-08`, `P0-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript and typed TUI behavior witnesses
- Evidence summary: `cargo test --locked -p tracewake-tui --test adversarial_gates` passed 15 tests and `cargo test --locked -p tracewake-tui --test tui_seam_conformance` passed 2 tests.
- Path under test and behavior witness: current-view semantic action submission, forged/stale semantic IDs, debug truth exclusion, possession rebind non-transfer, debug panels, transcript debug marking, TUI/core builder boundary, and TUI seam conformance.
- Replay/provenance ancestry: current-view source context, holder-known context, semantic action IDs, typed diagnostics, and checksum/view comparisons.
- Sampling/exhaustiveness scope: TUI adversarial gate set from spec §4.2.
- Pending or historical handling: debug rows remain non-diegetic and observer-only.
- Certification use: counted as certifying pass.

### `0037-SOURCE-DISCIPLINE`

- Requirement IDs: `P0-10`
- Evidence status: `pass`
- Fingerprint scope: documentation/source-discipline review
- Evidence summary: This artifact cites 0036 as historical supersession target only. All pass rows cite live 0037 commands, commits, report paths, and mutation output from the current remediation checkout.
- Path under test and behavior witness: `docs/4-specs/SPEC_LEDGER.md` source-discipline posture, spec `0037`, tickets `0037P0CERTMUTREM-001` through `-003`, and this replacement artifact.
- Replay/provenance ancestry: not applicable beyond command/report provenance.
- Sampling/exhaustiveness scope: current artifact and active spec/ticket closeout surfaces.
- Pending or historical handling: archived specs/tickets `0005` through `0036` are context only. `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md` remains superseded historical evidence, not live proof.
- Certification use: counted as certifying pass for P0-10.

## Mandatory Mutation Pass Row

| Evidence item | Status | Fingerprint scope | Behavior witness | Replay/provenance ancestry | Sampling/exhaustiveness | Pending/historical | Certification use |
|---|---|---|---|---|---|---|---|
| Full configured P0-CERT mutation posture | pass | `cargo mutants --workspace -f ... --no-shuffle` with `.cargo/mutants.toml`, cargo-mutants 27.1.0, unsharded checkout `a3b5e3c9e896d09ed9f5b6591fb6cd7a4edd0441` | Every critical survivor killed or tool-classified unviable; `0036-MUTATION-REMEDIATION-001` caught by `actor_known_local_actor_reaches_embodied_view_model_with_context_provenance` | `mutants.out` result files and `reports/0037_mutation_triage_register.md`; holder-known context ID/hash/frontier witness in `hidden_truth_gates` | Full configured posture completed; 1128 total, 896 caught, 232 unviable, 0 missed, 0 timeout | 0036 mutation run treated as incomplete historical context | Supports this artifact's `P0-CERT passed` verdict together with P0-01..P0-10 pass rows |

## Staged-Abstraction Declaration

None for P0-CERT obligations. This artifact does not defer mutation survivor
disposition, actor-known provenance, debug quarantine, replay, or P0-01..P0-10
live re-proof behind a staged abstraction.

Future institutions, records, wrong suspicion, courts, sanctions, notices,
travel, regional scale, LOD, LLM dialogue, story-sifting, SPINE-CERT,
EPI-CERT, ORD-LIFE-CERT expansion, Phase-4 entry, and second-proof entry remain
outside this scoped artifact and are not advanced by it.

## EMERGE-OBS Handling

No-human and ordinary-life fixture runs provide replayable observer-only
ordinary-life evidence. That evidence is retrospective review evidence only. It
does not define a gate, threshold, mutation score, scheduler objective, scenario
goal, dramatic target, or simulation input.

## Residual Convention-Only Items

No P0-CERT obligation remains pending in this artifact. Review is still required
for future claims that cite this artifact, because the certification use is
strictly scoped to the post-0008 baseline mutation remediation line at the exact
implementation commit named above.

## Certification Use

Later specs may cite `P0-CERT passed` only by citing this artifact, this exact
scope, and the exact implementation commit under test. If a later checkout
changes protected seams, it must re-run the appropriate gates rather than
treating this artifact as latest-main certification.

Forbidden wording:

- "Tracewake is fully certified."
- "Latest main was independently verified."
- "Later Phase 2+ / Phase 3A+ systems are certified by this pass."
- "Archived specs are live authority."
- "Project is P0 certified."
- "SPINE-CERT passed."
