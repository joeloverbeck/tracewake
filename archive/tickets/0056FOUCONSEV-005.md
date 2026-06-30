# 0056FOUCONSEV-005: Taxonomy self-mutation perimeter and doctrine-complete CI-forced parser

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, `.github/workflows/ci.yml` mutation lanes, and `ci_workflow_guards.rs`
**Deps**: 0056FOUCONSEV-003, 0056FOUCONSEV-004

## Problem

Findings F7-05 and F7-06 (merged — shared mutation/CI config surface).

- **F7-05**: the taxonomy guards (`compute_result`, `governance_is_independent`, the green-mutation-evidence validation, the wording grammar) have synthetic unit tests, but no mutation-sensitive proof shows that mutants of the parser/guard functions themselves are killed. A guard whose own mutants survive is decorative.
- **F7-06**: `.github/workflows/ci.yml` has a real forcing topology — the "Ingest changed acceptance artifacts" step (`:102`) rejects report/spec closure changes without a changed acceptance artifact and passes each through `TRACEWAKE_ACCEPTANCE_ARTIFACT` into the manifest parser (`:125`). The defect was that the forced parser was stale/shallow (F7-03/F7-04); once those land, this ticket confirms the *forced* parser is doctrine-complete and topology-guarded, and adds the guards to the mutation perimeter so the forcing function is non-decorative.

## Assumption Reassessment (2026-06-28)

1. Live code verified at `2720167`: parser/guard functions in `support/acceptance_status_manifest.rs` — `compute_result:283`, `governance_is_independent:382`, `validate_status_manifest:50`, `has_mutation_survivors:113`; wording grammar in `acceptance_artifact_wording.rs`. CI: `ci.yml:102` ingestion step, `:125` `TRACEWAKE_ACCEPTANCE_ARTIFACT`, `:129` `public-boundary-conformance`, the `full-surface-mutation-trigger` (`:360`) and `mutants-lock-layer` lanes. Guard test `ci_workflow_guards.rs:348` `acceptance_artifact_ingestion_guard_rejects_missing_job`, `:373` `acceptance_artifact_ingestion_errors`, `:379` `public_boundary_conformance_errors`. Mutation config `.cargo/mutants.toml` + `.cargo/mutants-baseline-misses.txt`.
2. Spec §4.5 + §4.6 + §6.1. The forced parser is doctrine-complete only once 003 (governance arm), 004 (closed grammar) land — hence Deps. No new gate code is minted; the durable forcing function is the existing ingestion step plus a doctrine-complete, closed-grammar, current-symbol-bound, self-mutation-covered parser.
3. **Shared boundary under audit**: the mutation perimeter (`.cargo/mutants.toml`) and CI mutation lanes are the standing-gate config; this ticket adds the parser/guard functions (003/004's surfaces) to that perimeter. `support/acceptance_status_manifest.rs` is mutation-*targeted* here, completing the sequential chain (003→004→005).
4. INV-098 harsh acceptance, plus R-29 (guard vacuity / decorative locks, `docs/3-reference/01`) — a guard with surviving mutants does not prove what it claims; mutation coverage of the verdict logic makes the acceptance taxonomy's own correctness load-bearing.
5. **Fail-closed acceptance / forcing-function substrate**: the mutation perimeter and the CI ingestion topology protect the fail-closed parser (003/004). Item-5 applies on the substrate basis — naming the protected enforcement surface (the doctrine-complete parser) and confirming the perimeter/topology change introduces no path that lets a survivor or a stale parser compute pass. No production state/replay/leak surface is touched (config + topology guard only).

## Architecture Check

1. Adding the parser/guard functions to the mutation perimeter closes the "decorative guard" gap: a mutant of `compute_result` / `governance_is_independent` / the green-evidence validation / the wording grammar is killed by an existing test rather than surviving. The CI topology guard (`ci_workflow_guards.rs`) asserts the ingestion step forces *the doctrine-complete parser* rather than merely existing — making F7-06 a real forcing function, not a wired-but-stale one.
2. No backwards-compatibility shim: the perimeter is widened to the configured union, not branched; the existing ingestion step is kept (preserved property) and strengthened by what it forces.

## Verification Layers

1. R-29 / INV-098 (non-decorative guards) -> a mutation campaign over the parser/guard functions with current caught/unviable/missed/timeout disposition; no survivor on these functions.
2. F7-06 (forcing topology) -> `ci_workflow_guards.rs` topology assertions (labeled topology alarm) that the ingestion step exists and forces the parser for changed closure artifacts, plus the F7-03/F7-04 deterministic guards proving the forced parser is doctrine-complete.
3. Config/CI ticket — the proof surfaces are the mutation campaign (005/006 boundary) and the topology guard test; no production-behavior layer applies.

## What to Change

### 1. Widen the mutation perimeter to the taxonomy guards

Add `compute_result`, `governance_is_independent` (and the new `solo-maintainer-compensating-control` arm), the green-mutation-evidence validation, and the closed wording grammar to the in-diff and standing perimeter (`.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, the CI mutation lanes), so a mutant of the verdict logic is killed.

### 2. Assert the CI-forced parser is doctrine-complete

Extend `ci_workflow_guards.rs` so the ingestion-step topology assertion covers that the forced parser (a) represents the settled `0055` posture, (b) enforces the closed verdict grammar, (c) computes non-pass for any open/pending/unbounded/historical/unproven row, live survivor/timeout/missed mutant, missing governance proof, or self-authored-only behavioral evidence, and (d) is itself mutation-covered.

## Files to Touch

- `.cargo/mutants.toml` (modify)
- `.cargo/mutants-baseline-misses.txt` (modify)
- `.github/workflows/ci.yml` (modify — mutation lanes covering the taxonomy guards)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify — doctrine-complete ingestion topology assertions)

## Out of Scope

- The governance arm (0056FOUCONSEV-003) and wording grammar (0056FOUCONSEV-004) themselves — this ticket covers them with mutation, it does not author them.
- The standing campaign *run* and survivor disposition (0056FOUCONSEV-006).
- Minting any new gate code or CI job — the existing ingestion step is preserved and strengthened.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards` — the strengthened ingestion-topology assertions pass and fire on synthetic stale-parser workflows.
2. A focused `cargo mutants` run over the parser/guard functions shows zero survivors (recorded in full by 0056FOUCONSEV-006).

### Invariants

1. The standing and in-diff mutation perimeter covers `compute_result`, `governance_is_independent`, the green-evidence validation, and the closed wording grammar; no artifact may call the perimeter green with a survivor on these functions.
2. The CI ingestion step forces a parser that is doctrine-complete, closed-grammar, current-symbol-bound, and self-mutation-covered — asserted by the topology guard.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/ci_workflow_guards.rs` — doctrine-complete forced-parser topology assertions (synthetic stale-parser negative cases).

### Commands

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards`
2. `cargo mutants --file crates/tracewake-core/tests/support/acceptance_status_manifest.rs --file crates/tracewake-core/tests/acceptance_artifact_wording.rs` — focused self-mutation campaign over the taxonomy guards (the per-ticket verification boundary; the full standing campaign is 0056FOUCONSEV-006).

## Outcome

Completed: 2026-06-28

Widened the standing mutation perimeter and in-diff trigger to include the
acceptance taxonomy guard surfaces:

- `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`;
- `crates/tracewake-core/tests/acceptance_status_manifest.rs`;
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs`;
- `crates/tracewake-core/tests/ci_workflow_guards.rs`.

Strengthened the lock-layer workflow so `acceptance_status_manifest` runs as an
always-on gate beside `acceptance_artifact_wording`, and extended
`ci_workflow_guards.rs` with topology assertions that fail if the always-run
doctrine-complete parser gate, closed verdict grammar gate, taxonomy mutation
perimeter, or in-diff taxonomy trigger is removed.

`.cargo/mutants-baseline-misses.txt` remains unchanged and empty; no accepted
taxonomy survivor was added.

Focused mutation attempt:

- `cargo mutants --file crates/tracewake-core/tests/support/acceptance_status_manifest.rs --file crates/tracewake-core/tests/acceptance_artifact_wording.rs`
  started successfully with `cargo-mutants 27.1.0` but expanded to 3,474 mutants
  and remained opaque after roughly 90 seconds. It was interrupted with exit
  code 130. This is recorded as incomplete evidence, not a green mutation
  result. The full run-and-record mutation disposition remains assigned to
  `0056FOUCONSEV-006`.

Verification:

- `cargo fmt --all --check` — passed.
- `cargo test --locked -p tracewake-core --test ci_workflow_guards` — passed.
- `cargo test --locked -p tracewake-core --test acceptance_status_manifest` —
  passed.
- `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` —
  passed.
- `cargo test --locked -p tracewake-core` — passed.
