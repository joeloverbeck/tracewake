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
nothing and passes/fails nothing. It exists so the emergence axis — whether
ordinary causal life actually produces incidents, wrong beliefs, and recoveries
worth investigating — accumulates measured data instead of remaining the only
foundational goal with none. It operationalizes INV-040 and INV-098 under the
artifact authority of
`docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`.
It amends no doctrine.

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
    INV-107-quarantined);
  - diverged beliefs later corrected through modeled channels;
  - distinct event kinds reached.
- Output: a ledger section — per-seed rows plus corpus totals — in every
  subsequent phase or scoped acceptance artifact whose verification exercises
  the canonical no-human corpus.
- Replay discipline: ledger values must be byte-identical after log
  serialization and replay, matching the no-human metrics rule in
  `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`.
  Stuck/failure outcomes are counted per the no-human metrics constraint in
  `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`.

Quarantine (hard rules):

- The ledger is an observer-only projection. It must never become a simulation
  input, scheduler input, content-selection input, or difficulty/pacing input.
  A ledger that feeds back into the world is a boredom detector and violates
  INV-060 and INV-087.
- Ledger counters are not pass/fail thresholds. No CI gate, test, or
  certification artifact may fail on a ledger value without a future spec that
  justifies the specific threshold against INV-060 (no manufactured drama) and
  records it as a deliberate decision.
- The sanctioned future escalation is a zero-floor ratchet only: once baselines
  exist across phases, a spec may assert that canonical-corpus counters do not
  silently collapse to zero (a dead-world regression guard). Demanding more
  drama than baseline is never a valid ratchet direction.

`DIAG-CERT` interaction: none. `EMERGE-OBS` produces evidence; it certifies
nothing and blocks nothing.

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
| `mutants-in-diff` | Runs guarded-layer mutation checks for pull requests and pushes when guarded source paths changed, with accepted baseline misses normalized by file, mutation, and function. |
| `mutants-lock-layer` | Runs the scheduled or manual guarded-layer mutation baseline and uploads `mutants.out` while failing on new misses outside the accepted baseline. |

Phase-entry mutation rule: clearing a scheduled-mutation `pending` status, or
making any `ORD-LIFE-CERT` readiness claim that depends on the lock layer,
requires a dated green scheduled mutation run. A report may record pending
status honestly, but pending is not a pass and does not satisfy certification
readiness.

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
