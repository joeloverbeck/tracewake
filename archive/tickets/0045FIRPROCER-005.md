# 0045FIRPROCER-005: Run the complete configured campaign to completion — census, completion-union proof, and triage register

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — evidence-collection ticket; it runs the certifying campaign and produces reports. Any production/test/fixture correction a survivor demands is routed to a reserved follow-up ticket, not made here.
**Deps**: 004

## Problem

With the durable sharded lane (-004), supervisor (-002), and merger (-003) in place, the certifying campaign must actually run to completion at one exact final implementation/evidence commit `U`, classify every one of the configured population, and prove the result is a complete disjoint union with zero final survivors. This is the deliverable that resolves the 0044 floor: 2,384 of 2,901 classified, 517 unclassified, no recorded miss/timeout but no completion either (spec §3.1). The settled contract is to **complete the full standing denominator**, not shrink it (spec §1.2, §1.5).

This ticket: freezes `U`; runs the mandatory clean preflight and named first-proof suites; captures the canonical `--list-files`/`--list` census and explains every delta from the 62-file / 2,901-mutant authoring baseline by legitimate final-tree source change; executes every shard (or one complete unsharded run if measured evidence proves it completes with margin); runs the -003 merger to prove the completion union; writes the final missed/timeout manifests (empty for a pass) and the census/completion manifests; and reconciles every historical and newly-exposed identity into the triage register. A newly exposed survivor's *fix* is unknowable until the run exists, so survivor remediation is routed to a reserved open range rather than pre-authored (spec §3.6, §5.6).

(spec §4.3, §4.9, §4.11, §4.12, §5.2, §5.3, §5.5, §13.1 phases 6–11)

## Assumption Reassessment (2026-06-21)

1. The certifying inputs all exist at the authoring baseline and are verified this session: `.cargo/mutants.toml` (62-file standing seam, `examine_globs`, both carriers present), `.cargo/mutants-baseline-misses.txt` (0 bytes), and the named first-proof suites of spec §4.9 — `acceptance_gates`, `acceptance_artifact_wording`, `golden_scenarios`, `hidden_truth_gates`, `event_schema_replay_gates`, `negative_fixture_runner`, `no_human_capstone`, `emergence_ledger`, `generative_lock`, `spine_conformance`, `anti_regression_guards`, `ci_workflow_guards`, `doc_invariant_references` (core); `fixtures_load`, `forbidden_content`, `golden_fixtures_run`, `schema_conformance` (content); and the seven TUI integration targets `adversarial_gates`, `command_loop_session`, `embodied_flow`, `readme_sample_session`, `transcript_snapshot`, `tui_acceptance`, `tui_seam_conformance`. All present as `cargo test` targets.
2. The authoring-baseline census to reconcile against is `reports/0044_first_proof_cert_mutation_list_files.txt` (62 files) and `reports/0044_first_proof_cert_mutation_list.txt` (2,901 rows). Verified this session: carrier counts validate 246 / apply 206 / projection 143 / rebuild 58 / perception 34 / contradiction 18 / checkcontainer 14 / time 9, with four selected module files (`agent/mod.rs`, `epistemics/mod.rs`, `events/mod.rs`, `replay/mod.rs`) generating zero rows. These are historical orientation; the final commit must regenerate and explain its own census (a changed count must follow legitimate final-tree source change, not be forced to the old numbers — spec §3.3, §4.3).
3. Cross-artifact boundary under audit: the exact-commit discipline (spec §1.4) — every certifying command (preflight, named suites, `--list-files`/`--list`, baseline, every shard, merger) must run at the same `U` with a clean `git status --porcelain=v1`, identical Cargo.lock/toolchain/cargo-mutants version/config fingerprints; no evidence may predate the last source/test/fixture/config/workflow edit. The campaign loads `.cargo/mutants.toml` and uses no `--no-config`/`-f`/`--exclude`/`--in-diff`/`--iterate`/replacement-config/baseline-suppression to define the accepted denominator (spec §4.3).
4. Invariant/rule motivation: the execution mutation-completion / evidence-honesty doctrine (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`) and the zero-final-survivor end state (spec §4.1 items 14–20, §8.6). A final `missed` cannot be waived as a baseline exception; `.cargo/mutants-baseline-misses.txt` must not become a parking lot for newly exposed survivors (spec §4.8, §9.3). A clean unmutated baseline must precede any outcome interpretation — a baseline failure makes mutation outcomes meaningless (spec §4.9).
5. Enforcement surface (evidence-consumer basis): the campaign *audits/re-proves* the deterministic-replay, actor-knowledge, and fail-closed-validation enforcement surfaces by running the named suites and the configured mutation population — it does not modify them. Confirm the evidence collection introduces no epistemic-leakage or replay-nondeterminism path (the clean preflight runs `hidden_truth_gates`, `event_schema_replay_gates`, `negative_fixture_runner`, `forbidden_content` unchanged), and that any survivor-prompted fix is routed out (reserved range) so this ticket stays evidence-only. The denominator is not shrunk and no `#[mutants::skip]` is added for convenience.

## Architecture Check

1. One evidence ticket that freezes `U`, runs the complete campaign, and proves the union by member-set is cleaner than interleaving survivor fixes into the run: certifying evidence may not be spliced across commits, so any fix forces a new `U` and a complete rerun (spec §4.10). Keeping fixes out of this ticket (routed to the reserved range) preserves a single coherent certifying commit and prevents an old-clean-identity / new-code splice.
2. No backwards-compatibility aliasing/shims. No denominator shrink, no baseline laundering, no `--iterate`/`-f` substitution for the complete final run; the focused 0044 wave and any development runs are labelled historical/development, never certifying.

## Verification Layers

1. Complete configured denominator at `U` → command + census: `cargo mutants --workspace --no-shuffle --list-files` / `--list` regenerate the canonical population; every delta from 62 files / 2,901 rows is explained by final-tree source change, both carriers present, no forbidden filter.
2. Clean baseline precedes interpretation → replay/test command: the four workspace gates and every named first-proof suite of §4.9 pass at `U` before any mutation outcome is read; the unmutated cargo-mutants baseline passes.
3. Disjoint exact union, zero final floor → merger proof: the -003 merger reconciles every shard (or one complete run) to the canonical set — `S_k == O_k`, pairwise disjoint, `⋃ == C`, count equations — and `missed = 0`, `mutant-level timeout = 0`, `caught + unviable = canonical generated total`, with empty final missed/timeout manifests fingerprinted from the complete denominator.
4. Identity reconciliation (replay/provenance for any survivor) → triage register: every intermediate `missed`/`timeout` and every historically relevant 0044 identity is reconciled by enclosing symbol + operator + normalized text (line/column advisory), with responsible layer and FIRST-PROOF cross-reference; a survivor needing a fix routes to the reserved range, not this ticket.

## What to Change

### 1. Freeze the final implementation/evidence commit `U` and run the clean preflight

Create `U`; record `git rev-parse HEAD` and a clean `git status --porcelain=v1`; fingerprint Cargo.lock/toolchain/manifests/`.cargo/mutants.toml`/baseline-miss/`ci.yml`/supervisor/merger/fixtures/named-test sources; run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace --locked`, and every named first-proof suite of §4.9, retaining separate transcripts. Stop on any failure.

### 2. Capture and review the canonical census

Capture `reports/0045_first_proof_cert_mutation_list_files.txt` and `…_list.txt`, fingerprint them, and write `reports/0045_first_proof_cert_mutation_census.md` per §5.2 (final SHA + clean-tree record, config/toolchain/Cargo.lock fingerprints, selected/generated counts, zero-mutant selected files, per-file counts, authoring-baseline-to-final delta explanation, explicit presence of the eight named carriers, canonical identity-set fingerprint, chosen shard denominator + assignment mode, per-shard expected count/fingerprint, reviewer no-shrink signoff).

### 3. Execute the complete campaign and prove the union

Run the dedicated baseline plus every shard (or one complete unsharded run if -002's evidence proves it completes with margin) at `U`, retaining output on every outcome under non-overwriting paths (the §5.3 shard/attempt package). Run the -003 merger to produce `reports/0045_first_proof_cert_mutation_completion_manifest.md`/`.json`. Write `reports/0045_first_proof_cert_mutation_final_missed.txt` and `…_final_timeout.txt` (empty and fingerprinted from the complete canonical denominator for a pass — file existence alone is not proof).

### 4. Reconcile every identity into the triage register

Create/maintain `reports/0045_first_proof_cert_mutation_triage_register.md` per the §4.12 schema (append-only; one row per final `missed`/`timeout` plus reconciliation rows for historically relevant 0044 identities and any source-changed identity needed to explain the census). For each newly exposed survivor, follow the §5.6 loop up to identity reconciliation and responsible-layer assignment, then **route the fix to a reserved follow-up ticket** (`0045FIRPROCER-007` onward, count implementation-discovered) and rerun the complete campaign at a new `U` once it lands — a targeted rerun does not replace the complete final rerun (spec §4.10).

## Files to Touch

- `reports/0045_first_proof_cert_mutation_list_files.txt` (new)
- `reports/0045_first_proof_cert_mutation_list.txt` (new)
- `reports/0045_first_proof_cert_mutation_census.md` (new)
- `reports/0045_first_proof_cert_mutation_completion_manifest.md` (new — emitted by the -003 merger)
- `reports/0045_first_proof_cert_mutation_completion_manifest.json` (new — emitted by the -003 merger)
- `reports/0045_first_proof_cert_mutation_final_missed.txt` (new)
- `reports/0045_first_proof_cert_mutation_final_timeout.txt` (new)
- `reports/0045_first_proof_cert_mutation_triage_register.md` (new)
- `reports/0045_first_proof_cert_command_transcripts/` (new — per-shard/attempt command, metadata, status, stdout, stderr, checksums)

## Out of Scope

- Building the lane, supervisor, or merger (→ -004, -002, -003).
- Fixing any newly exposed survivor — routed to the reserved `0045FIRPROCER-007`+ range; this ticket records the survivor, names its responsible layer, and reruns the campaign once a fix lands, but authors no production/test correction itself.
- Live FIRST-PROOF-01…17 re-proof, the nine-gate/nine-family/temporal evidence, EMERGE-OBS, and the replacement acceptance artifact (→ -006).
- Any denominator shrink, `#[mutants::skip]` for convenience, or addition of a survivor to `.cargo/mutants-baseline-misses.txt` (forbidden — spec §4.3, §4.8, §4.15).

## Acceptance Criteria

### Tests That Must Pass

1. The four workspace gates and every named first-proof suite of §4.9 pass at `U`, with retained transcripts; the unmutated cargo-mutants baseline passes.
2. The -003 merger proves the complete disjoint union against the canonical census (`S_k == O_k`, pairwise disjoint, `⋃ == C`, count equations) and exits zero.
3. `reports/0045_first_proof_cert_mutation_final_missed.txt` and `…_final_timeout.txt` are empty and fingerprinted from the complete denominator; `.cargo/mutants-baseline-misses.txt` remains 0 bytes.

### Invariants

1. Every certifying command runs at the same `U` with a clean tree and identical config/toolchain/cargo-mutants fingerprints; no evidence is spliced across commits.
2. The configured denominator is complete and un-shrunk — `missed = 0`, `mutant-level timeout = 0`, `caught + unviable = canonical generated total` — and every identity is reconciled in the triage register or routed to a reserved fix ticket.

## Test Plan

### New/Modified Tests

1. `None — evidence-collection ticket; verification is command-based (the clean preflight, named first-proof suites, configured `--list`/`--list-files` census, dedicated baseline + shard campaign, and the -003 merger reconciliation). No new `cargo` test target is added here; survivor-prompted test additions land in reserved follow-up tickets.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked` then each named suite, e.g. `cargo test --locked -p tracewake-core --test acceptance_gates` … `--test doc_invariant_references`, `cargo test --locked -p tracewake-content --test forbidden_content` …, `cargo test --locked -p tracewake-tui --test tui_acceptance` … (the full §4.9 list).
2. `cargo mutants --workspace --no-shuffle --list-files > reports/0045_first_proof_cert_mutation_list_files.txt` and `cargo mutants --workspace --no-shuffle --list > reports/0045_first_proof_cert_mutation_list.txt`, then the dedicated baseline + sharded campaign (`cargo mutants --workspace --no-shuffle --shard K/N …` per -004) and the -003 merger over the retained shard set. (Heavy certifying commands; they run the live campaign and are not dry-runnable — that is the stated reason they are executed in full, not pre-listed.)
3. `wc -c reports/0045_first_proof_cert_mutation_final_missed.txt reports/0045_first_proof_cert_mutation_final_timeout.txt .cargo/mutants-baseline-misses.txt` — confirm the certifying manifests are empty and the baseline-miss file is unchanged at 0 bytes.

## Outcome

Completed: 2026-06-21

Completed the configured mutation campaign at final evidence SHA `9a071b6e32ebc5b6126645a9db257d453399c028` from a clean `/tmp` worktree. The preflight gates, all named first-proof suites, canonical census, and eight deterministic shards ran at that SHA. The final census matched the authoring baseline exactly: 62 selected files and 2,901 mutant identities.

The shard merger proved a complete display-name identity union:

- canonical denominator: 2,901
- shards received: 8 / 8
- pairwise disjoint: true
- union equals canonical: true
- caught: 2,277
- unviable: 624
- missed: 0
- timeout: 0

Produced the required concise evidence reports:

- `reports/0045_first_proof_cert_mutation_list_files.txt`
- `reports/0045_first_proof_cert_mutation_list.txt`
- `reports/0045_first_proof_cert_mutation_census.md`
- `reports/0045_first_proof_cert_mutation_completion_manifest.md`
- `reports/0045_first_proof_cert_mutation_completion_manifest.json`
- `reports/0045_first_proof_cert_mutation_final_missed.txt`
- `reports/0045_first_proof_cert_mutation_final_timeout.txt`
- `reports/0045_first_proof_cert_mutation_triage_register.md`
- `reports/0045_first_proof_cert_command_transcripts/` compact command/status metadata

The raw per-mutant build logs and diffs were retained only in the temporary evidence worktree during the run and intentionally not copied into the repository because they were approximately 700 MB. The committed transcript set contains command lines, metadata, environments, stdout/stderr, statuses, shard envs, and supervisor exit files; the completion manifest records the reconciled union proof and fingerprints.

Verification:

- `cargo fmt --all --check` at `9a071b6e32ebc5b6126645a9db257d453399c028`
- `cargo clippy --workspace --all-targets -- -D warnings` at `9a071b6e32ebc5b6126645a9db257d453399c028`
- `cargo build --workspace --all-targets --locked` at `9a071b6e32ebc5b6126645a9db257d453399c028`
- `cargo test --workspace --locked` at `9a071b6e32ebc5b6126645a9db257d453399c028`
- all named first-proof suite commands listed in this ticket at `9a071b6e32ebc5b6126645a9db257d453399c028`
- `cargo mutants --workspace --no-shuffle --list-files`
- `cargo mutants --workspace --no-shuffle --list`
- eight `cargo mutants --workspace --no-shuffle --shard K/8 --jobs 2 --baseline=skip --timeout 183` shard runs under `tools/supervise-command.sh`
- `python3 tools/merge-mutation-shards.py ... --expected-shards 8`
- `wc -c reports/0045_first_proof_cert_mutation_final_missed.txt reports/0045_first_proof_cert_mutation_final_timeout.txt .cargo/mutants-baseline-misses.txt` reported all three files at 0 bytes
