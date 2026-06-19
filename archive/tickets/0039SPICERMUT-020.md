# 0039SPICERMUT-020: Full standing mutation campaign to completion + survivor reconciliation + triage register

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — runs the standing configured mutation posture and produces the preflight/run evidence and triage register; any code fix for a newly-discovered behavior-relevant survivor is routed to a reserved follow-up ticket (`-022` onward).
**Deps**: 0039SPICERMUT-001, 0039SPICERMUT-002, 0039SPICERMUT-003, 0039SPICERMUT-004, 0039SPICERMUT-005, 0039SPICERMUT-006, 0039SPICERMUT-007, 0039SPICERMUT-008, 0039SPICERMUT-009, 0039SPICERMUT-010, 0039SPICERMUT-011, 0039SPICERMUT-012, 0039SPICERMUT-013, 0039SPICERMUT-014, 0039SPICERMUT-015, 0039SPICERMUT-016, 0039SPICERMUT-017, 0039SPICERMUT-018, 0039SPICERMUT-019

## Problem

The 296 Wave B survivors are a floor, not the acceptance target (spec §3, §4.1). After the perimeter is made permanent (ticket 001) and the per-file kills land (tickets 002–019), the standing configured posture must run to completion over the entire perimeter, every historical survivor must be reconciled to the final source, every additional survivor the completed run surfaces must be triaged, and the result recorded in a complete triage register (§4.4–§4.13). Timeouts are not passes; baseline misses may not be used to launder behavior-changing survivors; the 296 count must be treated as a floor, not the denominator of convenience.

## Assumption Reassessment (2026-06-18)

1. After ticket 001, `.cargo/mutants.toml` carries the standing `examine_globs` union with `test_workspace = true`; `.cargo/mutants-baseline-misses.txt` is empty at the target baseline (verified, 0 lines) and must not become an intake queue for unresolved survivors (§4.12). The 296 seed identities are in `reports/0038_spine_cert_mutation_triage_register.md` (raw `path:line:column:operator`); the 0039 register normalizes them to `path` + enclosing symbol/function + operator/diff and maps each to its final-checkout identity (trust symbol/operator identity over drifted line:col).
2. Spec §4.4–§4.13 and §8.3 are the implementation contract; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` and `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` govern evidence honesty and anti-Goodhart review (verified present).
3. Shared boundary under audit: the whole-perimeter mutation evidence program — preflight census, clean baseline, run-to-completion (sharded or not), survivor reconciliation, and the triage register schema (§4.8) whose tool-outcome and certification-disposition axes are kept separate.
4. Motivating doctrine: `INV-098 — Feature acceptance is harsh` and the spec's anti-Goodhart discipline (§3, §4.9–§4.12) — a representative-subset kill, a mutation-score bump, or a baseline-laundered miss reproduces the failure. Certification requires a completed census and an explicit disposition for every survivor.
5. This ticket audits (does not modify) the fail-closed-validation, deterministic-replay, and actor-knowledge-firewall enforcement surfaces by mutating them: the campaign must run under the checked-in expanded configuration with `test_workspace = true`, must complete over all shards or the unsharded population with zero unresolved timeouts/tool failures, and must not use `--iterate` for the final certifying run. The evidence collection introduces no leakage or nondeterminism; tool outcomes and review dispositions are recorded separately and honestly (a `missed` outcome may be accepted only with a reviewer-approved equivalent/non-critical disposition, never counted as `caught`). There is no schema shape change. Implementer-recorded owner decisions (spec §12.2): shard count/runner topology, timeout values, register path + machine-readable companion format, reviewer-signoff procedure, and whether approved exceptions live in `.cargo/mutants-baseline-misses.txt` or a separate reviewed allowlist.

## Architecture Check

1. A single checked-in configuration run to completion with a clean recorded baseline, member-by-member reconciliation of all 296 seeds plus every new survivor, and a two-axis (tool-outcome vs disposition) register is the only structure that certifies the SPINE perimeter without metric gaming or baseline laundering.
2. No backwards-compatibility aliasing/shims: production code is not reshaped to suppress mutant generation; no `#[mutants::skip]`, dead-code trick, or test-only branch makes a survivor disappear without the equivalent/non-critical reachability + review evidence.

## Verification Layers

1. Census completeness (§4.4) -> `cargo mutants --workspace --list-files` + `--list` archived and mechanically compared to the standing file census; no required path absent, no exclusion overlap.
2. Clean baseline (§4.5) -> the §7.1 command set (`fmt`/`clippy`/`build`/`test --workspace --locked`) plus the named SPINE suites pass before any mutation result is interpreted; no `--baseline=skip` or failure-discarding retry.
3. Run completion + disposition (§4.6–§4.12) -> `cargo mutants --workspace --no-shuffle` (sharded per §4.6 if chosen) completes with zero unresolved timeouts/tool failures, no `--iterate` on the final run, and every survivor reconciled to killed / reviewer-approved-equivalent / reviewer-approved-non-critical in the register.
4. Configuration correction surfaced during preflight: for cargo-mutants `27.1.0`, `test_workspace = true` already injects Cargo's workspace-test flag. `.cargo/mutants.toml` must keep `additional_cargo_args = ["--locked"]`, not a duplicate `--workspace`, so the standing run is certifiable instead of classifying every mutant as tool-unviable.

## What to Change

### 1. Preflight census + clean baseline (archived)

Archive `cargo mutants --workspace --list-files` and `--list`, mechanically comparing the census to the §4.2 standing list (including `events/mod.rs`, `replay/mod.rs`, `state.rs`, `controller.rs`, `epistemics/projection.rs`); record the `.cargo/mutants.toml` fingerprint, CI workflow fingerprint, cargo-mutants version, Rust toolchain, Cargo.lock fingerprint, and exact implementation SHA. Record the clean unmutated baseline (the §7.1 command set + named SPINE suites).

### 2. Run to completion + reconciliation

Run the standing configured posture to completion (sharded only under the §4.6 reproducibility constraints; final certifying run without `--iterate`). Reconcile every one of the 296 seed survivors to its final identity and disposition; append and triage every additional survivor the completed run surfaces.

### 3. Triage register (§4.8 schema)

Produce `reports/0039_spine_cert_mutation_triage_register.md` (or the implementer-recorded register path) with, per survivor: historical identity, final identity (path/symbol/operator/diff + structured-output reference), tool outcome, responsible SPINE seam, responsible failing layer, certified reachability, test family + exact case, behavior witness, replay/provenance ancestry, negative/contamination control, certification disposition (killed / equivalent / non-critical / blocked-untriaged), evidence references, and review signoff. Keep tool-outcome and disposition as separate axes. For any reviewed equivalent/non-critical exception, record exact call sites + reviewer evidence; behavior-changing SPINE-relevant misses may not be accepted in `.cargo/mutants-baseline-misses.txt` or any allowlist.

### 4. Route new behavior-relevant survivors

Any newly-discovered behavior-relevant survivor that needs a code/test fix is routed to a reserved follow-up ticket (`0039SPICERMUT-022` onward — open range, count implementation-discovered, since the run's additional-survivor volume is unknowable at decomposition time). The next batch in this namespace must check the actual high-water mark before claiming numbers.

## Files to Touch

- `reports/0039_spine_cert_mutation_triage_register.md` (new — or the implementer-recorded register path)
- `.cargo/mutants-baseline-misses.txt` (modify — only if a reviewer-approved equivalent/non-critical exception lands with its evidence reference; otherwise unchanged/empty)

## Out of Scope

- Per-file behavior-witness authoring (tickets 002–019) and perimeter/CI config (ticket 001).
- SPINE-01…08 live re-proof and the replacement acceptance artifact/verdict (ticket 021).
- Code fixes for newly-discovered survivors — routed to reserved `-022` onward.
- Shard count, timeout values, runner topology, reviewer-signoff procedure, register path/format — implementer-recorded owner decisions (§12.2).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked` — clean baseline at the final implementation commit.
2. `cargo mutants --workspace --list-files` and `cargo mutants --workspace --list` — archived; census matches the standing file list with no silent exclusion.
3. `cargo mutants --workspace --no-shuffle` (sharded per §4.6 if chosen) — completes with zero unresolved timeouts/tool failures; every survivor reconciled in the register (zero blocked/untriaged for a pass claim).

### Invariants

1. Tool outcomes and certification dispositions are separate axes; a `missed` outcome is accepted only with a reviewer-approved equivalent/non-critical disposition, never counted as `caught`.
2. The 296 count is treated as a floor, not the denominator; no timeout is silently green and no behavior-changing SPINE-relevant miss is laundered into a baseline/allowlist.

## Test Plan

### New/Modified Tests

1. `None — evidence/triage ticket; verification is the archived `--list-files`/`--list`/run outputs, the clean-baseline command set, the named SPINE suites (run per §7.1), and the completed triage register. No production logic changes.`

### Commands

1. `cargo mutants --workspace --list-files && cargo mutants --workspace --list`
2. `cargo mutants --workspace --no-shuffle`
3. The whole-workspace standing campaign (not a per-file `-f` run) is the correct verification boundary for this ticket; per-file `-f` runs are tickets 002–019's boundary.

## Outcome

Completed: 2026-06-18

Produced the standing SPINE-CERT mutation evidence package and triage register:

- `reports/0039_spine_cert_mutation_list_files.txt`
- `reports/0039_spine_cert_mutation_list.txt`
- `reports/0039_spine_cert_mutation_run.txt`
- `reports/0039_spine_cert_mutation_missed.txt`
- `reports/0039_spine_cert_mutation_timeout.txt`
- `reports/0039_spine_cert_mutation_timeout_retry.txt`
- `reports/0039_spine_cert_mutation_timeout_retry_missed.txt`
- `reports/0039_spine_cert_mutation_timeout_retry_caught.txt`
- `reports/0039_spine_cert_mutation_triage_register.md`

Preflight found that ticket 001's cargo-mutants posture duplicated Cargo's
workspace-test flag: `test_workspace = true` plus
`additional_cargo_args = ["--workspace", "--locked"]` made cargo-mutants 27.1.0
invoke `cargo test --workspace --workspace --locked`, classifying all 2625
mutants as unviable. Corrected the checked-in posture to the equivalent locked
workspace-test configuration, `test_workspace = true` plus
`additional_cargo_args = ["--locked"]`, and updated the guard tests/spec text so
the bad combination cannot return silently.

The corrected standing census contains 48 files and 2625 mutants. The full
standing run was executed as:

```sh
cargo mutants --workspace --no-shuffle -j 8 -o mutants-cert-j8.out
```

Result: 2625 mutants tested in 4h: 37 missed, 2040 caught, 545 unviable, 3
timeouts. The three timeout identities were retried with:

```sh
cargo mutants --workspace --no-shuffle -j 1 --timeout 600 -F 'current_place_perception_events|visible_item_payload|EpistemicProjectionChecksum::as_str' -o mutants-timeout-retry.out
```

Retry result: 20 mutants tested in 3m: 2 missed, 16 caught, 2 unviable, 0
timeouts. The effective unresolved set is therefore 38 actionable missed mutants
and 0 unresolved timeouts/tool failures after retry. No survivor was accepted as
equivalent/non-critical and `.cargo/mutants-baseline-misses.txt` remains
unchanged/empty. The aggregate mutation row remains `SPINE-CERT scoped
remediation`, with new remediation tickets opened:

- `0039SPICERMUT-022` for controller binding survivors.
- `0039SPICERMUT-023` for state accessor and door connectivity survivors.
- `0039SPICERMUT-024` for epistemic projection/checksum survivors.

`0039SPICERMUT-021` now depends on tickets 022-024 before attempting the
capstone replacement artifact.

Verification run:

- `cargo mutants --workspace --list-files`
- `cargo mutants --workspace --list`
- `cargo mutants --workspace --no-shuffle -j 8 -o mutants-cert-j8.out`
- `cargo mutants --workspace --no-shuffle -j 1 --timeout 600 -F 'current_place_perception_events|visible_item_payload|EpistemicProjectionChecksum::as_str' -o mutants-timeout-retry.out`
- `cargo test --locked -p tracewake-core --test ci_workflow_guards`
- `cargo test --locked -p tracewake-core --test anti_regression_guards mutation_perimeter_matches_duration_action_rationale_and_ci_filters`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace --locked`
