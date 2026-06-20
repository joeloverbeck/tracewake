# 0043ORDLIFCER-004: Run the full configured mutation campaign to completion, reconcile every survivor, and produce the triage register

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — campaign execution and evidence artifacts (triage register, completion manifest, list/output captures); no production/simulation logic change.
**Deps**: 0043ORDLIFCER-001, 0043ORDLIFCER-002, 0043ORDLIFCER-003

## Problem

The 0043 floor is co-equal: kill the three `need_accounting.rs` seeds AND complete the configured mutation lane to a transport-honest denominator, then reconcile every identity. A test change that kills the three historical identities but leaves the lane incomplete fails the spec; a completed lane that still leaves actionable survivors fails the spec; a clean score from a filtered or in-diff run fails the spec. This ticket runs the full standing configured posture to completion at one frozen final commit, proves the denominator, reconciles the three historical identities plus every newly-exposed survivor, and produces the triage register and final-missed manifest. Completion and semantic disposition are separate mandatory claims.

(spec §3.2, §3.6, §4.1, §4.7–§4.11, §5.5)

## Assumption Reassessment (2026-06-20)

1. The certifying inputs exist and are settled by Deps: the corrected in-diff trigger + drift-proof guard (-001), the three seed kills with a passing targeted `-f need_accounting.rs` rerun (-002), and the non-PTY supervised launch + completion mechanism (-003). The standing `.cargo/mutants.toml` expands to 60 files / 2877 mutants at the authoring baseline `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd` (= current HEAD); `.cargo/mutants-baseline-misses.txt` is empty. Verified this session.
2. The named preflight gate binaries (spec §4.7) all exist and are runnable — core: `acceptance_artifact_wording`, `acceptance_gates`, `anti_regression_guards`, `ci_workflow_guards`, `doc_invariant_references`, `emergence_ledger`, `event_schema_replay_gates`, `generative_lock`, `golden_scenarios`, `hidden_truth_gates`, `negative_fixture_runner`, `no_human_capstone`, `spine_conformance`; content: `fixtures_load`, `forbidden_content`, `golden_fixtures_run`, `schema_conformance`; tui: `adversarial_gates`, `command_loop_session`, `embodied_flow`, `readme_sample_session`, `transcript_snapshot`, `tui_acceptance`, `tui_seam_conformance`. Verified by `test -f` this session.
3. Cross-artifact boundary under audit: the complete-run reconciliation contract — `set(final --list identities) == disjoint_union(caught, missed, timeout, unviable)` — over the standing denominator, plus the historical-identity reconciliation (raw 0042 `path:line:column:operator` → enclosing symbol/operator → final identity, mapped through any refactor) and the triage-register schema (spec §4.10, §4.11). Per the deep-researched-spec convention, the historical `:line:col` anchors are advisory; reconcile by enclosing symbol + operator.
4. Invariant/rule motivation: INV-098 (harsh acceptance — complete denominator execution, honest counts) and the execution mutation-completion / anti-Goodhart doctrine (`docs/2-execution/03_*`, `docs/2-execution/10_*`, `docs/3-reference/01_DESIGN_RISK_REGISTER.md`). Killing only the named three, improving a percentage, or stopping after a green focused run is anti-Goodhart and cannot certify; the acceptance denominator is the complete configured population at the final commit (spec §3.6).
5. Enforcement surface (evidence-consumer basis): this ticket audits the deterministic-replay, single-charge accounting, and actor-knowledge firewall surfaces by running their gates and the mutation campaign; it modifies none of them. It must freeze a clean worktree before certifying commands (spec §4.2), run the unmutated baseline + named gate suites green before interpreting mutants, keep the launcher/denominator/exit/per-mutant/disposition axes separate (spec §4.5), and introduce no nondeterminism into the captured evidence (same commit/config/version/args/shuffle posture across the run or a provably complete shard union).

## Architecture Check

1. A single frozen-commit campaign + machine-checkable set reconciliation is cleaner than an interrupted-then-inferred run: it makes completion provable (every generated identity has exactly one final outcome) and disposition auditable (each survivor mapped to ORD-LIFE seam + responsible layer). Sharding, if used, is a reproducible partition of one complete denominator, not a coverage shrink.
2. No backwards-compatibility aliasing/shims; the certifying run uses the checked-in config with no `--no-config`, `-f`/`--file`, `--exclude`, `--in-diff`, `--iterate`, or function/mutant filters, and no survivor is laundered into `.cargo/mutants-baseline-misses.txt`.

## Verification Layers

1. Clean baseline (INV-098) → command transcript: `cargo fmt`/`clippy`/`build`/`test --workspace`/`--doc` and the named core/content/tui gate binaries all pass at the frozen final commit before any mutant outcome is interpreted; worktree recorded clean.
2. Denominator completeness → machine-checkable reconciliation: final `--list-files`/`--list` captured and fingerprinted; `set(final --list identities) == disjoint_union(caught, missed, timeout, unviable)`; every count delta from 60/2877 explained by final-tree source change.
3. Historical-identity reconciliation → register rows: the three 0042 seeds reconciled by path/symbol/operator with final tool outcome + certification disposition (default killed; the three may close only as caught or rigorously equivalent/unreachable with independent signoff); every new survivor/timeout appended and triaged.
4. Transport honesty → command transcript: launcher result, denominator completion, cargo-mutants run exit, per-mutant outcome, and certification disposition recorded as separate axes; no process-list heuristic used as completion proof.

## What to Change

### 1. Freeze the final implementation identity

Per spec §4.2: create the intended final implementation commit; record `git rev-parse HEAD` and clean `git status --porcelain`; record Rust/Cargo/cargo-mutants/OS/runner/CPU/memory/environment versions; fingerprint `Cargo.lock`, `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, `.github/workflows/ci.yml`, final list outputs, and every evidence register; prohibit source/test/fixture/config/evidence edits until the certifying set completes (any edit ⇒ new final commit + full rerun).

### 2. Clean baseline + named preflight

Run at the frozen commit (spec §4.7): `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo build --workspace --all-targets --locked`; `cargo test --workspace --locked`; `cargo test --locked -p tracewake-core --doc`; then the named core/content/tui gate binaries (assumption 2). Record exact commands, exit statuses, times, versions, environment, transcript fingerprints. A mutation run against a failing/hanging baseline is meaningless.

### 3. Capture the denominator and run the campaign to completion

Capture `--list-files` → `reports/0043_ord_life_cert_mutation_list_files.txt` and `--list` → `reports/0043_ord_life_cert_mutation_list.txt` (JSON forms retained where supported); prove `need_accounting.rs` + the six 0042 paths present, `test_workspace`/locked args effective, no exclusion hides a required path, every 60/2877 delta explained. Run the default unsharded certifying form `cargo mutants --workspace --no-shuffle -o reports/0043_ord_life_cert_mutation_full.out` under the -003 supervised launch (no forbidden options); concurrency/timeouts only when recorded and justified (do not copy the failed `-j 8` by habit). If sharded, partition one complete denominator with every shard index used exactly once and reconcile the union with no omissions/duplicates. Each per-mutant timeout retried under a recorded policy and resolved (caught / missed-then-killed / approved equivalent / unviable / blocked).

### 4. Reconcile and produce the triage register

Produce `reports/0043_ord_life_cert_mutation_triage_register.md` and `reports/0043_ord_life_cert_mutation_final_missed.txt` (spec §4.10, §4.11): each historical row mapped raw→final by symbol/operator with final outcome + disposition + closure evidence; every new survivor appended as `new in completed run` with ORD-LIFE cross-reference, responsible layer, certified reachability, test family, behavior witness, replay/provenance ancestry, negative control, disposition, and evidence references. The final-missed manifest uses canonical cargo-mutants identities, one per line, reconciling exactly to the final `missed` set (retain + fingerprint even if empty). Tool outcome and certification disposition are separate axes (an approved equivalent stays `missed` as a tool outcome, never counted `caught`).

### 5. Route run-discovered fixes out

Any newly-discovered survivor needing a real kill/fix routes to the reserved `0043ORDLIFCER-006`-onward range (count implementation-discovered) — this ticket records the posture and triage; it does not author those kills.

## Files to Touch

- `reports/0043_ord_life_cert_mutation_triage_register.md` (new)
- `reports/0043_ord_life_cert_mutation_final_missed.txt` (new)
- `reports/0043_ord_life_cert_mutation_list_files.txt` (new)
- `reports/0043_ord_life_cert_mutation_list.txt` (new)
- `reports/0043_ord_life_cert_mutation_full.out/` (new — campaign output directory, or complete shard directories)
- `reports/0043_ord_life_cert_command_transcripts/` (modify — directory created by -003; append baseline/campaign/reconciliation transcripts)

## Out of Scope

- The CI in-diff trigger / guard convergence (→ -001), the seed kills (→ -002), and the lane-completion diagnostic/supervisor mechanism (→ -003, consumed here).
- Authoring kills for run-discovered survivors (→ reserved `0043ORDLIFCER-006` onward).
- Live ORD-LIFE-01…12 seam re-proof matrix and the replacement acceptance artifact + verdict (→ -005).
- Any forbidden denominator option (`--no-config`, `-f`, `--exclude`, `--in-diff`, `--iterate`, filters) on the certifying run, or laundering a survivor into `.cargo/mutants-baseline-misses.txt` (spec §4.3, §4.6).

## Acceptance Criteria

### Tests That Must Pass

1. Clean baseline + all named core/content/tui gate binaries pass at the frozen final commit before any mutant outcome is interpreted; worktree recorded clean.
2. Final `--list-files`/`--list` captured + fingerprinted; `need_accounting.rs` and the six 0042 paths present; every 60/2877 delta explained by final-tree source change.
3. The configured campaign completes unsharded (or as a provably complete shard union); `set(final --list identities) == disjoint_union(caught, missed, timeout, unviable)` holds with zero missing/duplicate identities and zero unresolved launcher/tool failures or timeouts.
4. The three historical seeds are reconciled by symbol/operator with final outcome + disposition; every new survivor is appended and triaged; `reports/0043_ord_life_cert_mutation_final_missed.txt` reconciles exactly to the final `missed` set.

### Invariants

1. The acceptance denominator is the complete configured population at the final commit; no percentage, three-kill, or focused-run shortcut substitutes (anti-Goodhart).
2. Launcher result, denominator completion, cargo-mutants exit, per-mutant outcome, and certification disposition remain separate axes; no process-list heuristic is treated as completion.

## Test Plan

### New/Modified Tests

1. `None — campaign-run/evidence ticket; verification is command-based (the certifying mutation run + named gate suites) and the deliverable is the triage register, completion manifest, and list/output captures.`

### Commands

1. Baseline + preflight: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked && cargo test --locked -p tracewake-core --doc`, then the named gate binaries (e.g. `cargo test --locked -p tracewake-core --test acceptance_gates --test no_human_capstone --test event_schema_replay_gates --test golden_scenarios --test hidden_truth_gates --test generative_lock`, and the content/tui binaries per spec §4.7).
2. Denominator capture: `cargo mutants --workspace --no-shuffle --list-files > reports/0043_ord_life_cert_mutation_list_files.txt` and `cargo mutants --workspace --no-shuffle --list > reports/0043_ord_life_cert_mutation_list.txt`.
3. Certifying run (under the -003 supervised non-PTY launch): `cargo mutants --workspace --no-shuffle -o reports/0043_ord_life_cert_mutation_full.out` — then reconcile `set(--list) == disjoint_union(caught,missed,timeout,unviable)` against `reports/0043_ord_life_cert_mutation_full.out/` and write the triage register + final-missed manifest. (No forbidden filters on this run.)
