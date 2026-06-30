# Testing, Observability, Diagnostics, and Review Artifacts

## Status

Live execution doctrine. Testing, debugging, observability, replay, actor-known traces, and anti-contamination checks are proof mechanisms, not support topics.

## Authority boundary

This document owns execution-level acceptance artifacts and diagnostic standards. It does not define implementation code or ticket lists.

## Depends on

- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

## Core rule

A test that proves plausible behavior while bypassing actor-known provenance is a bad test.

A diagnostic that says only that a test failed is a bad diagnostic.

## Required test families

| Family | Proof burden |
|---|---|
| Unit | Local invariants for event envelopes, validation, provenance, action definitions, context sealing, and diagnostics. |
| Integration | End-to-end pipeline from trigger/input to proposal, validation, event, projection, TUI/debug, replay. |
| Golden | Named fixture contracts with positive and negative outcomes. |
| Property/generative | Invariants over varied seeds, ordering, content, projections, and invalid data. |
| Static/anti-regression | Searchable guardrails for forbidden dispatch, forbidden fields, debug leakage, string-typed shortcuts, and branch-on-human privilege. |
| Replay | Deterministic rebuild from events and schema-compatible fixtures. |
| View-model | Embodied/debug separation and possession parity. |
| Content validation | Rejection of scripts, player privilege, culprit flags, hidden truth, and unproven cognition fields. |
| Certification audit | Human-readable gate report mapping code seams to doctrine. |

## Diagnostic standard

Every failure artifact must name:

- gate;
- layer;
- responsible component or data class;
- expected input source;
- actual input source;
- event/projection/checksum identifiers where available;
- actor-visible output, if any;
- debug-only output, if any;
- hidden truth excluded or leaked;
- replay divergence point, if any;
- remediation category.

## Responsible layers

Use these layer names consistently:

- `doctrine`;
- `content_schema`;
- `content_validation`;
- `fixture_contract`;
- `holder_known_context`;
- `candidate_generation`;
- `intention_lifecycle`;
- `method_selection`;
- `local_planning`;
- `proposal_construction`;
- `scheduler`;
- `action_validation`;
- `event_append`;
- `event_application`;
- `projection`;
- `replay`;
- `view_model`;
- `debug_quarantine`;
- `tui_input_binding`;
- `test_oracle`.

## Temporal, Completeness, Fairness, and Staged-Abstraction Diagnostics

The execution tier's temporal-authority and completeness additions use this
document as their consolidated diagnostic and review-artifact home. The
diagnostics remain typed, responsible-layer-attributed evidence; they are not
display strings, player-facing hints, simulation objectives, or actor-known
urgency.

Temporal diagnostics must identify temporal-divergence causes by responsible
layer, including candidate generation, sealed-context assembly, scheduler
dispatch, action validation, projection or view rendering, fixture authoring,
and review-artifact construction. Review artifacts must distinguish validator time
used to validate from holder-known temporal premises used to plan, and must show
both accepted and fail-closed paths where time is material. Temporal rendering
evidence must separate embodied, possession, debug, transcript, and observer-only
surfaces. LOD temporal-ancestry and time-acceleration review must expose temporal
ancestry, information ancestry, known-to-whom status, and fairness implications
without letting summary evidence become cognition input.

Completeness diagnostics must identify representation errors, custody-lineage
errors, procedure-visibility errors, and replay divergence for quantity and
economy evidence. Learning and adaptation diagnostics must distinguish learned
expectations, remembered events, direct observations, testimony,
record-derived beliefs, and prohibited truth-cache updates. Practical-bias and
social-harm diagnostics must identify the modeled source of the outcome and
distinguish it from hidden-truth leakage or arbitrary author fiat. Static
validation review must record that proof-bearing authored content was checked,
that negative fixtures failed for the intended reason, and that diagnostics
named the responsible layer.

Budget and fairness diagnostics are consolidated here. Scheduler, LOD,
time-acceleration, and Block-T proof homes may cross-reference this contract,
but must not restate it as separate competing standards. Review evidence must
cover starvation risk, repeated deferral, actor-class or region-class
imbalance, time-acceleration effects, replay determinism, and absence of
human-proximity or possessed-actor priority bias unless that bias is explicitly
non-diegetic input routing. This section defines no fairness formula, window,
budget, priority algorithm, or threshold.

Staged-abstraction review artifacts must state the proof currently provided,
the behavior intentionally abstracted, the falsehoods the stage must not fake,
the future feature or tier it must not block, the evidence preventing
overclaiming, and the diagnostics that would fail if the abstraction leaked
into certification. These fields are observer-only and non-certifying unless a
future scoped spec promotes a specific check through the normal authority
chain. They do not mint a new gate code or observation-obligation code.

## Observability requirements

Debug and review artifacts must expose enough structure to audit without becoming gameplay authority:

- event log with causal ancestry;
- proposal log with origin and holder-known context ID;
- actor-known context contents and provenance;
- candidate/method/local-plan traces;
- hidden-truth audit comparison rows;
- stuck/failure diagnostics;
- content validation reports;
- replay rebuild and divergence reports;
- view-model filtering reports;
- no-human metrics;
- emergence-evidence ledger (`EMERGE-OBS`, defined below);
- possession binding reports.

Debug output must be structurally separated from actor-visible output.

## Emergence-evidence ledger `EMERGE-OBS`

`EMERGE-OBS` is an observation obligation, not a certification gate. It blocks
nothing and passes/fails nothing. It exists so the emergence axis - whether
ordinary causal life actually produces incidents, wrong beliefs, stale-record
effects, wrong suspicions, and modeled recoveries worth investigating -
accumulates measured data instead of remaining the only foundational goal with
none. It realizes `INV-111`, the observer-only authoring boundary in
`docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`, the
first-playable acceptance doctrine in
`docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`, and the
observer-only emergence-evidence artifact contract in architecture 13
(`docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`).

Definition:

- Substrate: the canonical seeded no-human corpus — the canonical no-human day
  fixtures plus the generative seed set. The substrate is deterministic, so the
  ledger is replay-stable and diffable across versions; an emergence regression
  is visible as a ledger diff without anything gating on it.
- Counters, per seeded day, derived only from typed materialized records and
  debug-side projections (never from display strings):
  - expectation contradictions discovered;
  - replans / fallback selections taken;
  - interruptions (sleep, work, intention);
  - intention switches with recorded causes;
  - stuck diagnostics by blocker category;
  - beliefs diverged from truth (debug-only truth/belief comparison,
    observer-only and non-diegetic);
  - diverged beliefs later corrected through modeled channels;
  - wrong suspicions or stale-record consequences where the corpus produces
    them through modeled channels;
  - distinct event kinds reached.
- Row contract: every row records source run, seed/randomness provenance where
  applicable, controller mode, phenomenon family, source events or causal-chain
  references, extraction time, review/projection version, and replay ancestry
  sufficient to replay and explain the phenomenon.
- Output: a ledger section — per-seed rows plus corpus totals — in every
  subsequent phase or scoped acceptance artifact whose verification exercises
  the canonical no-human corpus.
- Replay discipline: ledger values must be byte-identical after log
  serialization and replay, matching the no-human metrics rule in
  `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`.
  Stuck/failure outcomes are counted per the no-human metrics constraint in
  `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`.
- Extraction discipline: rows are extracted retrospectively from actual run
  events and projections with event-log ancestry. They must not be fabricated
  by fixtures, debug panels, display text, or harness insertion.

Quarantine (hard rules):

- The ledger is an observer-only projection. It must never become a simulation
  input, scheduler input, validator input, content-selection input,
  seed/scenario/objective picker, LOD-promotion input, or difficulty/pacing
  input. A ledger that feeds back into the world is a boredom detector and
  violates the no-scripting and human-focus boundaries.
- Ledger counters are not pass/fail thresholds. No CI gate, test, or
  certification artifact may fail on a ledger value without a future spec that
  justifies the specific threshold against INV-060 (no manufactured drama) and
  records it as a deliberate decision.
- The sanctioned future escalation is a zero-floor ratchet only: once baselines
  exist across phases, a spec may assert that canonical-corpus counters do not
  silently collapse to zero (a dead-world regression guard). Demanding more
  drama than baseline is never a valid ratchet direction.
- Invalid-pass examples include: evidence feeding cognition, scheduling,
  validation, authoring, or scenario choice; evidence lacking replay ancestry;
  rows based only on debug truth or display text; rows with no source event or
  causal-chain reference; and treating the observer-only ledger as a blocking
  gate or numeric dramatic-quality threshold.

`DIAG-CERT` interaction: none. `EMERGE-OBS` produces evidence; it certifies
nothing and blocks nothing.

## Playable-capability conformance evidence

TUI-facing capability parity evidence must invoke the real proposal,
validation, projection, view-model, render, and replay pipeline. Harness-only
construction is useful only as a negative or fixture setup; the passing witness
must come from the path under test.

Assertions must be typed-before-rendered: first prove the causal,
actor-known, availability, rejection, debug-quarantine, replay, and fixture-ID
facts through structured records, then compare any checked-in render
references. Golden stability is not determinism by itself. Determinism means
the same inputs produce the same typed report and rendered reference; a golden
reference means the accepted rendering is intentionally pinned.

The ordinary CI command lane must run the conformance suite with the rest of
the workspace evidence, and review artifacts must capture the command, report
path or digest, capability entries, verdict, and any accepted non-playable
classification. Report capture must be deterministic so reviewers can compare
coverage changes without relying on display strings.

Time-control and interval-summary acceptance artifacts must include a
command/evidence matrix. The matrix records command transcripts, typed event
ledgers, replay checksums, projection rebuild reports, stop reasons,
hidden-truth negative witnesses, human/no-human differential evidence,
duration-terminal/accounting witnesses, reservation-conflict evidence, and
capability-parity dispositions. Each row must state whether the evidence is
embodied, debug-only, replay, no-human, parity, or observer-only, and must
identify the source event, proposal, projection, context, or transcript
references that make it path-under-test evidence.

For the 0052 loaded-world/time-control seam, the matrix must keep five evidence
classes distinct: production behavior and runtime construction
(`LoadedWorldRuntime::from_bootstrap`, `world_step_coordinator.rs`,
`generative_lock.rs`, `embodied_flow.rs`, and loaded-content handoff tests),
temporal replay verdicts (`replay_temporal_frontier.rs` and
`ReplayTemporalVerdict`/`run_replay` integration), core-owned holder-known interval output
(`holder_known_interval_projection.rs` and `salient_stop_actor_known.rs`),
compile-fail authority boundaries (`negative_fixture_runner.rs`), and TUI
parity/adversarial rendering (`playable_capability_parity.rs`,
`parity_adversarial.rs`, and public command-loop world-advance tests). The
required `public-boundary conformance` job composes those existing behavior
tests. Focused mutation campaigns, including preserved 0049/0050 commands and
the 0052 focused repair commands recorded in `archive/tickets/0052FOUCONFOU-010.md`,
are mutation evidence only. The 0052 standing campaign recorded `3400` selected
mutants, `2645` caught, `748` unviable, `7` routed-forward
`food_source_fact_supersedes` misses, and `0` timeout. It may be cited as zero
in-surface misses/timeouts for the 0052 surface, but not as an unqualified green
canonical standing-perimeter certification while those food-source mutants
survive.

## General anti-vacuity and behavior witnesses

Every lock, gate, and proof obligation must identify at least one live negative
that would fail if the protected shortcut were reintroduced, or state why no
live negative can exist for that claim.

Artifact-presence checks are insufficient on their own. Each protected claim
must pair artifacts with a typed behavior witness from the path under test. A
behavior witness identifies:

- responsible layer;
- source event, proposal, projection, or context IDs;
- checked facts and their source status;
- accepted or rejected stage;
- replay/projection ancestry sufficient to distinguish production-path behavior
  from fixture labels, harness fabrication, or display-only evidence.

This standard applies across the truth firewall, provenance, freshness,
possession, derived accounting, emergence observation, replay, fixture, and
diagnostic surfaces. Provenance gates in document `04` cannot pass on schema
shape, fixture labels, or decorative metadata alone. Golden fixtures in
document `09` cannot pass on stable bytes alone when the semantic behavior
witness is missing or contradictory. The mutation-only "pending is not a pass"
rule below is an instance of this general evidence rule, not an exception to it.

Provenance-gate review artifacts must identify the source event, projection, or
sealed context for each action-relevant fact and include at least one live
negative where missing, empty, dangling, wrong-kind, ambiguous,
forbidden-source, or harness-fabricated provenance fails closed on the path
under test. If no live negative can exist, the artifact must state why.

Freshness review artifacts must show acquisition time, last-verified time,
source event/provenance, and freshness classification for the checked facts.
They must include a live negative where an old fact stays old despite a later
replay, possession bind, debug attach, notebook display, or no-human review.

Embodied carrier census artifacts must list every actor-visible datum exposed
by an action menu, possession preflight, notebook, why-not surface, or embodied
view. For each datum, the census records the carrier, holder, modeled capture
boundary, provenance, freshness classification, source event/projection/context
IDs, and whether a live-truth handle is absent from the generation path.

Single-charge review artifacts must identify the owning seam that emitted each
need/duration delta and prove that scheduler, routine planner, projection,
replay, and golden normalization consumed rather than independently charged the
same tick/window. A golden's byte stability is insufficient if the semantic
single-charge ledger drifts, double-counts, or omits a duration terminal.

## Evidence status and fingerprint scope honesty

Every review packet must label each evidence item by status:

- `pass` or `fail` only where the packet actually certifies the checked claim;
- `pending` where a required check has not yet produced certifying evidence;
- `sampled` where evidence is representative but not exhaustive;
- `observer-only` where evidence can inform review but cannot certify behavior;
- `historical` where archive/spec evidence is context rather than current
  certification.

Every review packet must also label fingerprint scope when it cites a
fingerprint or stable artifact:

- raw bytes;
- normalized serialization;
- parsed semantic content;
- command transcript;
- run seed;
- replay artifact.

A fingerprint must not be cited as proof beyond its scope. Pending, sampled,
observer-only, and historical evidence must never be silently counted as a
pass. Byte stability is not semantic truth unless the relevant behavior
witness, replay ancestry, and checked facts prove the semantic claim.

## Fail-closed acceptance manifest

Foundational conformance remediation artifacts must carry a fail-closed
acceptance manifest when they claim closure over protected authority seams. The
manifest is machine-readable, and the artifact verdict is computed from it; prose
cannot upgrade the result. The closed status set is
`closed|open|routed-forward|pending-governance|historical-only|not-in-scope`.
Each finding row records the existing finding label, status, certifying evidence
for `closed`, and a live negative or public-boundary proof for every protected
shortcut. The manifest also records the exact commit under test, source
acquisition method, actual-artifact ingestion status, branch-protection/ruleset
enforcement status, independent-acceptance posture, mutation evidence source,
mutation denominator, caught/unviable/missed/timeout disposition counts,
baseline-miss reconciliation status, survivor list, and overall result.
The independent-acceptance posture vocabulary includes human approving review,
last-push approval by required reviewers where configured, and
`solo-maintainer-compensating-control` for repositories operated by one
maintainer when the architecture-tier compensating-control set is proven by
machine-readable ruleset evidence.

Overall `pass` is legal only when every required finding is `closed`, every
required governance control is enforced by branch protection or active ruleset
and independent acceptance is proven where required, the actual artifact under
review was ingested, and mutation evidence is current and green. Green mutation
evidence requires an actual in-diff or current full-campaign result with
denominator and disposition counts, zero missed mutants, zero timeouts, and
current baseline reconciliation. In solo-maintainer mode, a
`required_approving_review_count: 0` configuration is not an independence gap
only when the manifest records `solo-maintainer-compensating-control` and the
ruleset proof shows every architecture-tier compensating control: all required
standing checks, no bypass actors, `current_user_can_bypass: never`,
non-fast-forward and deletion protection, and up-to-date branch enforcement.
Without that complete proof, zero-approval or status-checks-only independence is
not pass. `open`, `pending-governance`, zero-approval or status-checks-only
independence gaps, self-authored-only evidence, missing actual-artifact
ingestion, unbounded `routed-forward`, missing governance proof, sampled
evidence, observer-only evidence, historical results, survivors, timeouts,
stale mutation evidence, and "pass with disposition" are not pass. A wording
guard must fail artifacts that use pass-shaped closure language while the
manifest computes non-pass, call the canonical standing perimeter green while
survivors remain, cite branch-protection enforcement without an API or ruleset
transcript, cite a topology trigger as mutation proof, cite historical command
results as current certification, or cite display strings, artifact existence,
checksums, or source guards as sole evidence for behavior claims that require
typed path-under-test evidence.

The `solo-maintainer-compensating-control` posture re-specifies the
forward-looking standing independent-acceptance posture that spec `0054`
hardened under a second-human interpretation. The `0054` evidence remains
valid for its exact implementation commit; it is historical ruleset evidence,
not a current requirement that routine solo-maintainer merges retain one
approving review.

A routed-forward residual is bounded only if it names the owning surface, why
the current line did not close it, the next known execution move, a maximum
number of remediation epochs or concrete trigger after which it becomes
blocking, and the exact CI or mutation check that fails if it remains. Missing
any forcing-function field makes the residual unbounded. Branch-protection and
ruleset evidence must come from an API transcript or equivalent machine-readable
repository governance evidence; workflow YAML and Rust topology guards prove job
shape only, not merge enforcement. Source-text guards are topology alarms only:
they may prove that a check is wired or that a workflow says what it should say,
but they do not prove behavior, independent acceptance, or mutation closure.
Guarded code changes require PR-blocking actual mutation proof, such as the
`mutation in-diff (lock layer)` check, or a current accepted full-campaign
artifact ingested by the fail-closed manifest.

## Property and random testing

Randomized testing is allowed only with recorded seeds, scoped random streams, deterministic ordering, and reproducible failure artifacts. A failing generated case must be replayable by seed and input manifest.

Properties should include:

- replay equals live projection;
- actor-visible output is subset of holder-known context;
- debug truth does not alter proposal sequence;
- possession binding does not alter world rules;
- invalid content fails before runtime;
- scheduler never emits primitive actions directly;
- validator truth does not create actor knowledge without modeled events;
- no accepted event depends on wall-clock time or unordered iteration.

## Current CI Job Set

The GitHub Actions workflow is part of the execution proof surface. It may be
stricter than the local four-gate command list only when that stricter posture is
recorded here and guarded against workflow drift.

Workflow-level posture:

- `permissions: contents: read` is declared at top level.
- `RUSTFLAGS: "-D warnings"` applies to all jobs.
- Target-bearing cargo caches key on `rust-toolchain.toml`, `**/Cargo.toml`, and
  `**/Cargo.lock`.
- Gate steps must not use `continue-on-error`, pipes, or `||` masking around the
  documented cargo gate commands.
- Third-party `uses:` actions outside `actions/*` must be pinned by full commit
  SHA.

| Job id | Required posture |
|---|---|
| `fmt` | Runs `cargo fmt --all --check`. |
| `clippy` | Runs `cargo clippy --workspace --all-targets -- -D warnings`. |
| `test` | Runs `cargo build --workspace --all-targets --locked` and `cargo test --workspace --locked`; the locked test invocation is the documented CI superset of the local `cargo test --workspace` completion gate. |
| `lock-layer-gates` | Runs the named lock-layer integration targets with `--locked`, including anti-regression, hidden-truth, replay, content, and TUI seam gates. |
| `public-boundary-conformance` | Runs the required public-boundary conformance lane with `negative_fixture_runner`, `generative_lock`, `world_step_coordinator`, `command_loop_session`, `playable_capability_parity`, and `embodied_flow`, proving the production-bootstrap-to-sealed-receipt/replay matrix through existing tests rather than new doctrine gate code. |
| `governance-required-checks-audit` | Queries the `main` branch-protection and branch-rulesets APIs and fails closed with `pending/unverified` unless the required workspace/public-boundary/mutation contexts, pull-request requirement, independent approval or last-push/required-reviewer posture, up-to-date-or-merge-queue requirement, and no-bypass posture are proven by repository governance settings. This is governance evidence, not a substitute for the workflow-topology alarms. |
| `full-surface-mutation-trigger` | Runs on pull requests and pushes, detects changes to production files, tests, fixtures, negative fixtures, mutation config/baseline, CI workflow, merge/supervisor tooling, and live conformance evidence, and reports that full-surface mutation reconciliation is required before merge. This trigger is an alarm, not mutation proof. |
| `mutants-in-diff` | Runs standing certification-perimeter mutation checks for pull requests and pushes when checked-in perimeter source paths changed, using two concurrent cargo-mutants jobs and an explicit 300 second per-mutant timeout, with accepted baseline misses normalized by file, mutation, and function. The per-mutant timeout is deliberately higher than the scheduled shards' 183 seconds because this job runs the full workspace suite per mutant under two-way concurrency, so control-flow mutants that roughly double the suite must still be allowed to be caught rather than spuriously reported as a timeout survivor; it still bounds genuinely non-terminating mutants. For guarded source changes this is the PR-blocking actual mutation-result context; it cannot be replaced by the full-surface trigger alone. |
| `mutants-lock-layer-baseline` | Runs the scheduled or manual unmutated baseline and canonical standing-perimeter mutation census through `.cargo/mutants.toml`, recording commit, config, and toolchain fingerprints for shard reconciliation. |
| `mutants-lock-layer` | Runs the scheduled or manual standing-perimeter mutation matrix as eight supervised shards after the baseline job, using `--baseline=skip`, two cargo-mutants jobs per shard, a 7200 second supervisor wall, 120 second grace, explicit 183 second mutant timeout, 130 minute GitHub timeout, and always-uploaded per-shard transcripts and mutation output. |
| `mutants-lock-layer-reconcile` | Downloads the scheduled or manual baseline and shard artifacts, invokes `tools/merge-mutation-shards.py`, and fails closed for missing shards, non-normal supervisor status, tool failure, survivor floor, overlap, fingerprint drift, or a shard union that does not equal the canonical denominator. |

Phase-entry mutation rule: clearing a scheduled-mutation `pending` status, or
making any `ORD-LIFE-CERT` readiness claim that depends on the lock layer,
requires a dated green scheduled mutation run where the baseline job, every
scheduled shard, and reconciliation job are green. A red scheduled mutation result is merge-blocking until repaired.
A report may record pending status honestly, but pending is not a pass and does
not satisfy certification readiness.

## Review artifact template

Every certification artifact must include:

1. Gate names reviewed.
2. Files/seams audited.
3. Foundation and architecture dependencies.
4. Positive fixture evidence.
5. Negative fixture evidence.
6. Replay evidence.
7. Actor-known provenance evidence.
8. Debug quarantine evidence.
9. Failure diagnostics with responsible layers.
10. Deferrals tied to named gates.
11. Archived-spec status statement.
12. Certification result: pass, fail, or scoped remediation.

## Central conformance gate `DIAG-CERT`

`DIAG-CERT` is a phase-certification artifact label from the execution
sequence. It consumes the canonical diagnostic gate evidence; it is not a new
canonical gate code beyond `00_EXECUTION_INDEX_AND_AUTHORITY.md`.

No phase or feature gate passes unless `DIAG-CERT` passes. This prevents tests from becoming shallow smoke checks.

`DIAG-CERT` passes only when failure reports are specific enough that a future implementation session can identify the layer to inspect without guessing.

## Forbidden test patterns

- Test passes because final state looks right while event ancestry is wrong.
- Test parses display strings to infer typed facts.
- Test uses debug truth as actor knowledge.
- Test drives scheduler shortcuts not available to normal actors.
- Test treats human command path as exempt from ordinary validation.
- Test asserts only that an actor waited, without modeled reason or stuck diagnostic.
- Test checks only happy path content and ignores forbidden fields.
- Test calls an archived spec sufficient evidence for live certification.
