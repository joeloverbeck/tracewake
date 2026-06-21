# 0045 FIRST-PROOF-CERT mutation remediation replacement certification acceptance

Verdict: FIRST-PROOF-CERT passed

FIRST-PROOF-CERT mutation remediation and replacement certification accepted for exact commit `9a071b6e32ebc5b6126645a9db257d453399c028`.

This artifact supersedes
`reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md`
for current FIRST-PROOF-CERT citation use. It applies only to the exact
implementation/evidence commit above. It does not independently verify latest
`main`, Phase-4 entry, second-proof entry, institutions, notices, travel,
regional scale, LOD, story-sifting, LLM/speech surfaces, or future feature
surfaces.

## Exact implementation/evidence commit under test

- Authoring/reassessment baseline:
  `fd5ae94ff3225d2f989262b95ed8272945861516`
- Final implementation/evidence commit:
  `9a071b6e32ebc5b6126645a9db257d453399c028`
- Evidence-package commit containing retained compact transcripts:
  `0a914fe3322bac292bc221d29737fe35a6195579`
- Branch or PR: local `main`
- Latest-main verification: not independently verified.
- Source status note: `preflight_meta/git_status_porcelain_before_evidence.txt`
  recorded only the newly-created untracked transcript root; the source,
  config, test, fixture, workflow, and mutation inputs under certification were
  the exact commit above.

## Supersession Block

The 0044 artifact remains historical audit evidence and remains truthful for its
own scoped-remediation verdict. This artifact replaces it for
FIRST-PROOF-CERT citation because the 0045 series completed the standing
configured mutation campaign and re-used the integrated first-proof command
corpus at the same implementation/evidence commit.

Predecessor certification artifacts are consumed only within their own exact
scopes:

| Consumed line | Artifact | Use here |
|---|---|---|
| `P0-CERT passed` | `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` | Admissibility only; no command output is spliced into 0045. |
| `SPINE-CERT passed` | `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md` | Admissibility only; event/replay participation is re-proven by 0045 commands. |
| `EPI-CERT passed` | `archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md` | Admissibility only; actor-known and possession participation are re-proven by 0045 commands. |
| `ORD-LIFE-CERT passed` | `archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md` | Admissibility only; no-human ordinary-life participation is re-proven by 0045 commands. |

## Changed Files

The implementation series changed mutation lane governance, supervision,
reconciliation, evidence reports, and acceptance wording surfaces. Principal
files include `.github/workflows/ci.yml`, `.cargo/mutants-baseline-misses.txt`,
`tools/supervise-command.sh`, `tools/merge-mutation-shards.py`,
`crates/tracewake-core/tests/ci_workflow_guards.rs`,
`crates/tracewake-core/tests/anti_regression_guards.rs`,
`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`,
the archived `0045FIRPROCER-001` through `-005` tickets, and the 0045 reports
under `reports/0045_first_proof_cert_*`.

## Gates Run

All commands below were supervised under
`reports/0045_first_proof_cert_command_transcripts/` and record
`exit_status=0`, `supervisor_result=child_exit_0`.

| Gate | Transcript | Result |
|---|---|---|
| `cargo fmt --all --check` | `preflight_fmt/` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | `preflight_clippy/` | pass |
| `cargo build --workspace --all-targets --locked` | `preflight_build/` | pass |
| `cargo test --workspace --locked` | `preflight_test_workspace_locked/` | pass |
| Named core first-proof suites | `named_core_*/` | pass for acceptance, replay, hidden-truth, no-human, emergence, generative, anti-regression, CI, and doc-reference surfaces |
| Named content first-proof suites | `named_content_*/` | pass for fixture load, forbidden content, golden fixture run, and schema conformance |
| Named TUI first-proof suites | `named_tui_*/` | pass for adversarial, command-loop, embodied, README sample, transcript, acceptance, and seam-conformance surfaces |
| `cargo mutants --workspace --no-shuffle --list-files` | `mutation_census_list_files/` | pass, 62 files |
| `cargo mutants --workspace --no-shuffle --list` | `mutation_census_list/` | pass, 2,901 canonical rows |
| Eight deterministic mutation shards | `mutation_shard_0_of_8/` through `mutation_shard_7_of_8/` | pass, all shards `child_exit_0` |
| `tools/merge-mutation-shards.py --canonical-list ... --expected-shards 8` | `reports/0045_first_proof_cert_mutation_completion_manifest.*` | pass, complete disjoint union |

## Evidence Fingerprints

| Artifact | SHA-256 |
|---|---|
| `reports/0045_first_proof_cert_mutation_list_files.txt` | `5473142052dec947e4a238f286a1af659c603ebf678bf22634af997b8570e8e5` |
| `reports/0045_first_proof_cert_mutation_list.txt` | `64cd82ab86330e595f161ba374e59ac559b7b48e25032fd0a739f62e355b59ac` |
| `reports/0045_first_proof_cert_mutation_completion_manifest.json` | `5b65ba1647a00e78530744b31273ad3bd57d1a12b62262f9c7117e33b6a71e76` |
| `reports/0045_first_proof_cert_mutation_completion_manifest.md` | `0c0b9c80e7f43b9f7ff4691f15e696cc885d0f626f34390b6b0539461a3a4ca8` |
| `reports/0045_first_proof_cert_mutation_final_missed.txt` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `reports/0045_first_proof_cert_mutation_final_timeout.txt` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `reports/0045_first_proof_cert_mutation_triage_register.md` | `896aeff86b2c9af93d4006b4e4f05df0ce357c12b1af9f2fefb5e63c2565a833` |

Raw per-mutant logs and diffs were retained in the temporary evidence worktree
during execution and not copied into the repository because the structured
manifest, shard metadata, command transcripts, final missed/timeout manifests,
and triage register carry the certifying result without committing hundreds of
megabytes of raw tool output.

## Per-Requirement Acceptance Evidence

| Requirement | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `FIRST-PROOF-01` | doctrine, fixture contract, test oracle, documentation status | `0045-GATES`, `0045-MUTATION-CENSUS`, `0045-SUPERSESSION` | pass |
| `FIRST-PROOF-02` | physical custody, event/replay, embodied view | `0045-GATES`, `0045-REPLAY-PACKAGE` | pass |
| `FIRST-PROOF-03` | provenance, content rejection, actor-known evidence | `0045-GATES`, `0045-FIXTURE-NEGATIVE` | pass |
| `FIRST-PROOF-04` | observation and hidden-truth firewall | `0045-GATES`, `0045-TRUTH-FIREWALL` | pass |
| `FIRST-PROOF-05` | expectation contradiction and replay | `0045-GATES`, `0045-REPLAY-PACKAGE` | pass |
| `FIRST-PROOF-06` | missing-property no-culprit and content negatives | `0045-GATES`, `0045-FIXTURE-NEGATIVE` | pass |
| `FIRST-PROOF-07` | actor-known non-interference and validation fail-closed | `0045-GATES`, `0045-TRUTH-FIREWALL` | pass |
| `FIRST-PROOF-08` | possession parity, embodied/debug split | `0045-GATES`, `0045-POSSESSION-DEBUG` | pass |
| `FIRST-PROOF-09` | no-human ordinary life and no-direct-dispatch | `0045-GATES`, `0045-REPLAY-PACKAGE` | pass |
| `FIRST-PROOF-10` | replay rebuild and controlled divergence | `0045-GATES`, `0045-REPLAY-PACKAGE` | pass |
| `FIRST-PROOF-11` | fixture loading, schema, semantic rejection | `0045-GATES`, `0045-FIXTURE-NEGATIVE` | pass |
| `FIRST-PROOF-12` | temporal firewall and holder-known temporal premises | `0045-GATES`, `0045-TEMPORAL-BUNDLE` | pass |
| `FIRST-PROOF-13` | routine temporal premise and accounting evidence | `0045-GATES`, `0045-TEMPORAL-BUNDLE` | pass |
| `FIRST-PROOF-14` | embodied temporal rendering and debug quarantine | `0045-GATES`, `0045-TEMPORAL-BUNDLE`, `0045-POSSESSION-DEBUG` | pass |
| `FIRST-PROOF-15` | temporal fixture pairing and anti-contamination | `0045-GATES`, `0045-TEMPORAL-BUNDLE`, `0045-FIXTURE-NEGATIVE` | pass |
| `FIRST-PROOF-16` | diagnostic and acceptance-artifact honesty | `0045-GATES`, `0045-ARTIFACT-HONESTY`, `0045-EMERGE-OBS` | pass |
| `FIRST-PROOF-17` | cross-gate relations and sampled metamorphic posture | `0045-GATES`, `0045-REPLAY-PACKAGE`, `0045-MUTATION-PASS` | pass |
| Configured mutation posture | mutation infrastructure | `0045-MUTATION-PASS`, `0045-MUTATION-RECONCILIATION` | pass |

## Nine-Gate Integrated Coverage

| Gate | Evidence item IDs | Result |
|---|---|---|
| `EVENT` | `0045-GATES`, `0045-REPLAY-PACKAGE`, `0045-MUTATION-PASS` | pass |
| `TRUTH-FIREWALL` | `0045-GATES`, `0045-TRUTH-FIREWALL`, `0045-FIXTURE-NEGATIVE` | pass |
| `ACTOR-KNOWN` | `0045-GATES`, `0045-TRUTH-FIREWALL`, `0045-TEMPORAL-BUNDLE` | pass |
| `POSSESSION-PARITY` | `0045-GATES`, `0045-POSSESSION-DEBUG` | pass |
| `NO-HUMAN-ORDINARY-LIFE` | `0045-GATES`, `0045-REPLAY-PACKAGE`, `0045-TEMPORAL-BUNDLE` | pass |
| `MISSING-PROPERTY` | `0045-GATES`, `0045-FIXTURE-NEGATIVE`, `0045-TRUTH-FIREWALL` | pass |
| `VIEW-DEBUG-SPLIT` | `0045-GATES`, `0045-POSSESSION-DEBUG`, `0045-TRUTH-FIREWALL` | pass |
| `REPLAY` | `0045-GATES`, `0045-REPLAY-PACKAGE`, `0045-MUTATION-PASS` | pass |
| `FIXTURE-NEGATIVE` | `0045-GATES`, `0045-FIXTURE-NEGATIVE` | pass |

## Nine Scenario Families

| Scenario family | Evidence item IDs | Result |
|---|---|---|
| Physical custody baseline | `0045-GATES`, `0045-REPLAY-PACKAGE` | pass |
| Expectation contradiction | `0045-GATES`, `0045-TRUTH-FIREWALL` | pass |
| Possession parity | `0045-GATES`, `0045-POSSESSION-DEBUG` | pass |
| Epistemic filtering | `0045-GATES`, `0045-TRUTH-FIREWALL` | pass |
| No-hidden-truth planning | `0045-GATES`, `0045-TRUTH-FIREWALL`, `0045-TEMPORAL-BUNDLE` | pass |
| No-human ordinary day | `0045-GATES`, `0045-REPLAY-PACKAGE` | pass |
| Routine blocking | `0045-GATES`, `0045-TEMPORAL-BUNDLE` | pass |
| Replay rebuild | `0045-GATES`, `0045-REPLAY-PACKAGE` | pass |
| Content rejection | `0045-GATES`, `0045-FIXTURE-NEGATIVE` | pass |

## Five-Source Temporal Bundle

Representative temporal fact: an actor may plan from holder-known temporal
premises while canonical event time remains the validator and replay authority.

| Temporal source | Evidence | Result |
|---|---|---|
| Canonical event time | `named_core_event_schema_replay_gates/` | pass; validates order and replay legality. |
| Holder-known temporal premise | `named_core_hidden_truth_gates/` | pass; hidden raw time does not leak into actor-known decisions. |
| Routine/no-human temporal use | `named_core_no_human_capstone/` | pass; ordinary routine progress is event-backed. |
| Fixture temporal pairing | `named_content_golden_fixtures_run/` | pass; committed fixtures exercise temporal behavior. |
| Embodied temporal rendering/debug split | `named_tui_embodied_flow/`, `named_tui_transcript_snapshot/`, `named_tui_adversarial_gates/` | pass; TUI surfaces render holder-known context while debug remains non-diegetic. |

Validator time and holder-known time are not collapsed. Validator time proves
legality and replay order; holder-known temporal premises are the planning and
embodied-rendering inputs.

## Mutation Completion Row

| Configured denominator | Shards | Caught | Missed | Timeout | Unviable | Disjoint | Union equals canonical | Result |
|---:|---:|---:|---:|---:|---:|---|---|---|
| 2,901 | 8 / 8 | 2,277 | 0 | 0 | 624 | true | true | pass |

The canonical identity fingerprint is
`c4bf4c6ae9d0184e6b3328db79218bf04e9c7ff1db0e7fd41a872c4627255425`
in display-name identity mode. The final missed and timeout manifests are empty.

## Evidence Item Ledger

### `0045-GATES`

- Requirement IDs: `FIRST-PROOF-01` through `FIRST-PROOF-17`, nine gates, nine
  scenario families.
- Evidence status: pass.
- Fingerprint scope: command transcript.
- Evidence summary: all preflight, named core, named content, and named TUI
  suites in `reports/0045_first_proof_cert_command_transcripts/` passed with
  `exit_status=0`.
- Path under test and behavior witness: workspace tests exercise the core,
  content, and TUI production entry points listed by the named suites.
- Replay/provenance ancestry: replay, golden scenario, hidden-truth, no-human,
  fixture, and transcript suites are included in the command set.
- Sampling/exhaustiveness scope: exhaustive over the finite named first-proof
  command corpus required by spec 0045.
- Pending or historical handling: predecessor artifacts are admissibility
  context only.
- Certification use: counted as certifying pass.

### `0045-REPLAY-PACKAGE`

- Requirement IDs: replay-related audit points, `EVENT`, `REPLAY`,
  no-human/replay scenario families.
- Evidence status: pass.
- Fingerprint scope: command transcript and replay artifact.
- Evidence summary: `named_core_event_schema_replay_gates`,
  `named_core_golden_scenarios`, `named_core_no_human_capstone`,
  `named_content_golden_fixtures_run`, and `named_tui_transcript_snapshot`
  passed.
- Path under test and behavior witness: event log, replay rebuild, projection,
  golden scenario, no-human, and transcript paths.
- Replay/provenance ancestry: event-log ancestry and first-divergence behavior
  are exercised by the replay gate suite and golden/no-human fixtures.
- Sampling/exhaustiveness scope: exhaustive over the named replay corpus.
- Pending or historical handling: none.
- Certification use: counted as certifying pass.

### `0045-TRUTH-FIREWALL`

- Requirement IDs: `TRUTH-FIREWALL`, `ACTOR-KNOWN`,
  `MISSING-PROPERTY`, `VIEW-DEBUG-SPLIT`.
- Evidence status: pass.
- Fingerprint scope: command transcript.
- Evidence summary: `named_core_hidden_truth_gates`,
  `named_core_acceptance_gates`, `named_content_forbidden_content`,
  `named_content_schema_conformance`, and `named_tui_adversarial_gates` passed.
- Path under test and behavior witness: hidden-truth paired interventions,
  fail-closed validation, content rejection, and TUI adversarial views.
- Replay/provenance ancestry: hidden-truth and validation evidence is sourced
  from modeled events and holder-known context rather than prose.
- Sampling/exhaustiveness scope: exhaustive over named anti-contamination suites.
- Pending or historical handling: none.
- Certification use: counted as certifying pass.

### `0045-POSSESSION-DEBUG`

- Requirement IDs: `POSSESSION-PARITY`, `VIEW-DEBUG-SPLIT`,
  `FIRST-PROOF-08`, `FIRST-PROOF-14`.
- Evidence status: pass.
- Fingerprint scope: command transcript.
- Evidence summary: TUI adversarial, embodied, acceptance, seam-conformance, and
  transcript suites passed.
- Path under test and behavior witness: holder-known embodied views and
  non-diegetic debug surfaces.
- Replay/provenance ancestry: transcript and embodied-flow tests derive views
  from committed fixture and event contexts.
- Sampling/exhaustiveness scope: exhaustive over named TUI first-proof suites.
- Pending or historical handling: none.
- Certification use: counted as certifying pass.

### `0045-FIXTURE-NEGATIVE`

- Requirement IDs: `FIXTURE-NEGATIVE`, `FIRST-PROOF-03`,
  `FIRST-PROOF-06`, `FIRST-PROOF-11`, `FIRST-PROOF-15`.
- Evidence status: pass.
- Fingerprint scope: command transcript.
- Evidence summary: `named_core_negative_fixture_runner`,
  `named_content_fixtures_load`, `named_content_forbidden_content`,
  `named_content_golden_fixtures_run`, and `named_content_schema_conformance`
  passed.
- Path under test and behavior witness: content schema, semantic rejection,
  compile-fail/public-boundary fixtures, and golden fixture execution.
- Replay/provenance ancestry: fixture behavior is tied to committed fixture
  corpus and schema validation.
- Sampling/exhaustiveness scope: exhaustive over named fixture and negative
  suites.
- Pending or historical handling: none.
- Certification use: counted as certifying pass.

### `0045-TEMPORAL-BUNDLE`

- Requirement IDs: `FIRST-PROOF-12` through `FIRST-PROOF-15`,
  `ACTOR-KNOWN`, `NO-HUMAN-ORDINARY-LIFE`.
- Evidence status: pass.
- Fingerprint scope: command transcript.
- Evidence summary: temporal evidence is consolidated from hidden-truth,
  event-replay, no-human, golden-fixture, embodied-flow, transcript, and
  adversarial TUI suites.
- Path under test and behavior witness: canonical event time, holder-known
  temporal premises, routine/no-human use, fixture temporal pairing, and
  embodied temporal rendering.
- Replay/provenance ancestry: canonical time remains replay authority while
  holder-known temporal premises remain actor planning/rendering input.
- Sampling/exhaustiveness scope: representative temporal fact traced across all
  five required source classes; the underlying named suites are finite and
  fully run.
- Pending or historical handling: none.
- Certification use: counted as certifying pass.

### `0045-MUTATION-PASS`

- Requirement IDs: configured mutation posture and mutation-linked
  first-proof carriers.
- Evidence status: pass.
- Fingerprint scope: command transcript and structured mutation manifest.
- Evidence summary: 62 files, 2,901 canonical mutants, eight deterministic
  shards, 2,277 caught, 624 unviable, 0 missed, 0 timeout.
- Path under test and behavior witness: full configured perimeter from
  `.cargo/mutants.toml`, including required FIRST-PROOF carriers.
- Replay/provenance ancestry: mutation posture is tool evidence; behavior
  witnesses are attached in row-specific evidence items.
- Sampling/exhaustiveness scope: exhaustive configured denominator; no sampled
  or omitted shard remains.
- Pending or historical handling: 0044's partial classification is superseded.
- Certification use: counted as certifying pass.

### `0045-MUTATION-RECONCILIATION`

- Requirement IDs: configured mutation posture.
- Evidence status: pass.
- Fingerprint scope: parsed semantic content.
- Evidence summary: the shard merge reported pairwise-disjoint shards,
  canonical-union equality, and a complete union over 2,901 identities.
- Path under test and behavior witness: `tools/merge-mutation-shards.py`,
  assigned-mutant metadata, outcome metadata, and final manifest files.
- Replay/provenance ancestry: not applicable for shard reconciliation.
- Sampling/exhaustiveness scope: exhaustive over all eight shards.
- Pending or historical handling: none.
- Certification use: counted as certifying pass.

### `0045-ARTIFACT-HONESTY`

- Requirement IDs: artifact/evidence honesty, `FIRST-PROOF-16`.
- Evidence status: pass.
- Fingerprint scope: artifact text and command transcript.
- Evidence summary: this artifact distinguishes certifying, historical, and
  observer-only evidence; `acceptance_artifact_wording` passed in the 0045
  transcript package and is re-run for this replacement artifact in ticket
  `0045FIRPROCER-006`.
- Path under test and behavior witness: acceptance artifact wording guard and
  this scoped replacement artifact.
- Replay/provenance ancestry: not applicable beyond the replay package cited
  above.
- Sampling/exhaustiveness scope: review-artifact field completeness against
  `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
- Pending or historical handling: none.
- Certification use: counted as certifying pass.

### `0045-EMERGE-OBS`

- Requirement IDs: observer-only emergence companion.
- Evidence status: observer-only.
- Fingerprint scope: artifact text and command transcript.
- Evidence summary: `reports/0045_first_proof_cert_emerge_obs.md` records
  retrospective event-log-backed emergence observations.
- Path under test and behavior witness: no production path; observer-only
  review companion.
- Replay/provenance ancestry: cited observations trace to the replay/no-human
  command corpus.
- Sampling/exhaustiveness scope: observer-only over the named first-proof corpus.
- Pending or historical handling: observer-only by design.
- Certification use: not counted as certifying evidence.

## Reconciled Verdict Tables

| Table | Result | Basis |
|---|---|---|
| `FIRST-PROOF-01` through `17` | pass | Every row has certifying command evidence and no pending blocker. |
| Nine canonical gates | pass | All gate rows cite certifying evidence items. |
| Nine scenario families | pass | All scenario rows cite certifying evidence items. |
| Five-source temporal bundle | pass | Validator time and holder-known time remain distinct. |
| Replay/provenance package | pass | Replay and divergence-localization suites passed. |
| Mutation completion | pass | Complete configured denominator, zero missed, zero timeout. |
| Supersession | pass | 0044 scoped-remediation artifact is explicitly superseded for current use. |
| Observer-only emergence | not certifying | `EMERGE-OBS` is present and non-steering. |

## Sampling, Exhaustiveness, Pending, And Historical Handling

The configured mutation row is exhaustive over the 2,901 canonical identities.
The named first-proof commands are exhaustive over the finite suite set required
by the spec. The temporal bundle uses one representative temporal fact to show
the five required source classes, but the underlying commands are the named
certifying suites. No pending, timeout, missed, sampled, or tool-failed item is
counted as a pass. Historical predecessor artifacts are lineage and
admissibility context only.

## Staged-Abstraction Declaration

What this artifact proves now: the scoped FIRST-PROOF-CERT mutation remediation
and integrated first-proof evidence package pass for the exact commit named
above.

What it deliberately abstracts: Phase-4 institutions, records,
wrong-suspicion, notices, travel, regional scale, LOD, story-sifting,
LLM/speech, second-proof entry, and latest-main certification.

What must not be faked: no prose-born simulation facts, no hidden-truth planning
input, no direct scheduler dispatch, no debug contamination, no mutation
denominator shrink, and no observer-only evidence counted as a pass.

Future work not blocked by the abstraction: later phases may proceed only
through their own scoped specs and gates after citing this artifact with its
exact scope.

Evidence preventing overclaiming: exact commit identity, command transcripts,
structured mutation manifest, empty final missed/timeout manifests, explicit
historical handling, and the observer-only `EMERGE-OBS` companion.

Failure diagnostics: a missing command transcript, nonzero shard, nonempty
missed/timeout manifest, mixed SHA, hidden-truth leak, replay divergence, or
observer-only evidence counted as a pass would route back to
`FIRST-PROOF-CERT scoped remediation` with the earliest responsible layer named.

## Residual Convention-Only Items

Template-field completeness, supersession wording, historical-only predecessor
use, and no-overclaim phrasing remain review-required because Rust tests cannot
prove the narrative scope of every report sentence. The review surfaces are this
artifact, `reports/0045_first_proof_cert_emerge_obs.md`,
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, and
`crates/tracewake-core/tests/acceptance_artifact_wording.rs`.
