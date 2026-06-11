# 0019PHA3AGENREA-005: Mutation perimeter expansion and CI gate semantics

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — CI/config only (`.cargo/mutants.toml`, `.github/workflows/ci.yml`, mutation baseline) plus a consistency guard in `anti_regression_guards.rs` and a conformance-index row; no production code
**Deps**: `tickets/0019PHA3AGENREA-001.md`, `tickets/0019PHA3AGENREA-002.md`, `tickets/0019PHA3AGENREA-003.md`, `tickets/0019PHA3AGENREA-004.md` (baseline refresh runs after the new tests exist so it is honest — spec §8); `specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md` (ORD-HARD-045)

## Problem

Three mutation-gate holes (`ORD-HARD-045`). First, `.cargo/mutants.toml` `exclude_globs`
lists `crates/tracewake-core/src/actions/defs/**`, and both the scheduled
`mutants-lock-layer` `-f` allowlist and the `mutants-in-diff` guarded-path grep cover
only `agent/**`, `scheduler*`, `projections*`, and `actions/pipeline.rs` — so the
interruption/proration and completion logic in `actions/defs/sleep.rs` and `work.rs`
(`sleep_interruption_reason`, `build_sleep_interruption_events`, the work completion
builders — exactly the code the `ORD-HARD-044` fabricator leaned on) is never
mutation-tested; a mutant flipping the severe-need interruption predicate survives
every gate. Second, the `mutants-in-diff` job runs `cargo mutants --in-diff … || true`
and treats a missing `mutants.out/missed.txt` as success (`exit 0`): a cargo-mutants
crash or timeout passes green. Third, the in-diff job is gated
`if: github.event_name == 'pull_request'` and the scheduled job on
`workflow_dispatch || schedule` — a direct push to `main` runs neither.

## Assumption Reassessment (2026-06-11)

1. Verified against current config at `main` `5af8660`: `exclude_globs` carries the
   `actions/defs/**` entry; both jobs' filters are as quoted; the in-diff job's
   `|| true` and missing-`missed.txt` → "all in-diff mutants caught; exit 0" branch are
   live in `.github/workflows/ci.yml`; the scheduled job treats a missing `missed.txt`
   as a failure (asymmetric with the in-diff job). The source-classification table
   (`anti_regression_guards.rs`: `WORKSPACE_SOURCE_CLASSIFICATIONS`, with
   `CORE_ACTION_RATIONALE` as the defs files' exemption rationale) claims the defs
   files are "covered by targeted action and pipeline guards" — a claim mutation does
   not currently reach. `sleep_interruption_reason` and
   `build_sleep_interruption_events` exist in `actions/defs/sleep.rs`;
   `.cargo/mutants-baseline-misses.txt` exists.
2. Verified against `specs/0019_…_HARDENING_SPEC.md` ORD-HARD-045 + §9 (reassessed
   2026-06-11): the perimeter expansion will surface new missed mutants in the duration
   builders — the refreshed baseline must be reviewed, not bulk-accepted; new misses in
   interruption predicates warrant tests before baselining.
3. Cross-artifact boundary under audit: the consistency contract between the mutation
   perimeter (`mutants.toml` + both CI jobs' filters) and the source-classification
   table's coverage rationales — an exclusion must be reflected in the rationale, or
   the guard fails.
4. Lock-durability doctrine restated (motivating rule, INV-091–098 spirit): the
   mutation gates exist to prove the lock layer detects regressions; a perimeter that
   excludes the highest-risk builders, a runner whose tool failure reads as success,
   and a push-shaped enforcement hole each let the lock layer assert more than it
   proves.
5. Adjacent either-or classified (required consequence, implementer's recorded choice
   per the spec): the push gap closes either by adding a push-triggered diff-vs-`HEAD^`
   mutation run, or by recording the required-PR branch rule as the compensating
   control in the conformance index — one of the two must land; the choice and its
   rationale are recorded in this ticket's implementation notes and the conformance
   row.

## Architecture Check

1. Expanding the `-f` filters and removing the `exclude_globs` entry brings the
   duration builders under the same ratcheted baseline as the cognition surfaces —
   reusing the existing normalize-and-`comm` machinery instead of inventing a parallel
   defs-only pipeline. Branching on cargo-mutants' documented exit codes separates
   "tool failed" from "zero missed mutants", which is the same loud-failure discipline
   the apply path applies to payloads (a missing artifact is never success). The
   perimeter↔classification guard makes the next exclusion a visible, rationale-bearing
   decision instead of a quiet config edit.
2. No backwards-compatibility aliasing/shims: the defs exclusion is removed, not
   conditionally re-enabled; the `|| true` is removed, not wrapped in a retry that
   swallows the same failures.

## Verification Layers

1. Perimeter reach -> `cargo mutants --list -f 'crates/tracewake-core/src/actions/defs/sleep.rs'`
   (and `work.rs`) produces mutants — the builders are inside the perimeter; the
   refreshed `.cargo/mutants-baseline-misses.txt` is reviewed line-by-line in the PR.
2. Tool-failure honesty -> CI job logic: a simulated cargo-mutants failure (nonzero
   exit, no `missed.txt`) fails the in-diff job; zero *new* missed mutants with a
   produced `missed.txt` passes (verified by job-log inspection on the first run —
   manual runbook step, since CI semantics are not unit-testable in-repo).
3. Perimeter↔classification consistency -> new guard in `anti_regression_guards.rs`
   parsing `.cargo/mutants.toml` and the ci.yml filter lines against
   `WORKSPACE_SOURCE_CLASSIFICATIONS` coverage rationales; a synthetic mismatch fails.
4. Push-gap closure -> grep-proof: either the push-trigger block exists in ci.yml, or
   the conformance-index row records the required-PR compensating control.

## What to Change

### 1. Perimeter expansion

Remove `crates/tracewake-core/src/actions/defs/**` from `exclude_globs`; add
`actions/defs/sleep.rs` and `actions/defs/work.rs` (and `eat.rs` if its delta logic is
nontrivial) to the scheduled job's `-f` list and the in-diff job's guarded-path grep.
Update the defs files' `CORE_ACTION_RATIONALE` classification rationale to reflect
mutation coverage. Refresh the baseline with a full run; review new misses and add
tests for interruption-predicate misses before baselining (spec §9).

### 2. In-diff job failure semantics

Drop `|| true`; branch on cargo-mutants' documented exit codes to distinguish tool
error (fail the job) from no-new-missed (pass); when the guarded-diff flag is true,
require evidence that mutants were generated (e.g. a nonempty mutants.out outcome
summary) rather than treating an absent `missed.txt` as success.

### 3. Push-gap closure

Add a push-triggered diff-vs-`HEAD^` mutation run, or record the required-PR branch
rule as the compensating control — the recorded choice from Assumption 5.

### 4. Consistency guard and conformance row

The perimeter↔classification guard (Verification Layer 3) and a conformance-index row
for the mutation-perimeter contract.

## Files to Touch

- `.cargo/mutants.toml` (modify)
- `.github/workflows/ci.yml` (modify)
- `.cargo/mutants-baseline-misses.txt` (modify — reviewed refresh)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `crates/tracewake-core/src/actions/defs/sleep.rs` / `work.rs` (modify — only if baseline review surfaces interruption-predicate misses needing tests; tests land beside the predicates' existing test modules)

## Out of Scope

- The generative tier itself (tickets `-003`, `-004`).
- Any production-logic change in the duration builders — this ticket adds coverage and
  gates, not behavior.
- A new full `cargo mutants` tooling pipeline — the existing normalize/ratchet
  machinery is reused.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test anti_regression_guards` —
   perimeter↔classification guard green.
2. `cargo mutants --list -f 'crates/tracewake-core/src/actions/defs/sleep.rs'` —
   nonempty mutant list (perimeter reach proof).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No source file the classification table claims is mutation-covered sits outside the
   mutation perimeter; exclusions carry explicit rationales the guard checks.
2. A mutation-tool failure can never read as a passing gate; absence of an output
   artifact is never success.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — perimeter↔classification
   consistency guard.
2. Manual runbook (CI semantics): trigger the in-diff job on a PR touching a guarded
   path; verify the job log distinguishes tool-error from zero-new-missed; verify the
   push-path decision per Assumption 5.

### Commands

1. `cargo mutants --list -f 'crates/tracewake-core/src/actions/defs/sleep.rs'`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo test --workspace`

## Outcome

Completed on 2026-06-11.

The mutation perimeter now includes the sleep/work duration action definitions:
`.cargo/mutants.toml` no longer excludes `actions/defs/**`, and both the
scheduled mutation baseline and in-diff guarded-path filter include
`actions/defs/sleep.rs` and `actions/defs/work.rs`. The in-diff job now runs for
guarded pull-request and direct-push changes, uses `HEAD^..HEAD` for push
diffs, captures the cargo-mutants exit status, treats tool failures as failed
gates, and requires output artifacts before accepting a no-miss result.

Added `mutation_perimeter_matches_duration_action_rationale_and_ci_filters` to
lock the mutants config, CI filters, failure semantics, push path, and source
classification rationale together. Added the `0019 mutation-perimeter honesty`
conformance row. During the reviewed duration-only mutation refresh, five
surviving `work.rs` mutants surfaced; targeted boundary, elapsed-cost, and
closed-workplace completion tests were added instead of accepting those misses.
The follow-up focused mutation run reported `83 mutants tested in 4m: 66 caught,
17 unviable`, with no missed duration-def mutants, so
`.cargo/mutants-baseline-misses.txt` did not change.

Verification:

1. `cargo test -p tracewake-core actions::defs::work::tests`
2. `cargo mutants --workspace -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle --jobs 4`
3. `cargo test -p tracewake-core --test anti_regression_guards`
4. `cargo mutants --list -f 'crates/tracewake-core/src/actions/defs/sleep.rs'`
5. `cargo fmt --all --check`
6. `cargo clippy --workspace --all-targets -- -D warnings`
7. `cargo build --workspace --all-targets --locked`
8. `cargo test --workspace`
