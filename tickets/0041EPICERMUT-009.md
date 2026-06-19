# 0041EPICERMUT-009: Full standing mutation campaign to completion + survivor reconciliation + triage register

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — runs the standing configured mutation posture and produces the preflight/census/run evidence and triage register; any code fix for a newly-discovered behavior-relevant survivor is routed to a reserved follow-up ticket (`0041EPICERMUT-011` onward, count implementation-discovered)
**Deps**: 0041EPICERMUT-001, 0041EPICERMUT-002, 0041EPICERMUT-003, 0041EPICERMUT-004, 0041EPICERMUT-005, 0041EPICERMUT-006, 0041EPICERMUT-007, 0041EPICERMUT-008

## Problem

Spec 0041 §3.6, §4.4–§4.8 require that EPI-CERT acceptance rest on a **complete** configured mutation run over the final standing `.cargo/mutants.toml` perimeter, not a narrow campaign against the 30 historical seed identities. The 30 are a floor, not the finish line: a complete run may enumerate more mutants, expose new misses, turn a timeout into a miss, or reveal a prior identity under a new line/symbol — and every such outcome joins the same remediation obligation (§3.1, §12.1 anti-Goodhart control). This ticket runs the standing campaign to completion after the per-file kill tickets land, reconciles every historical and newly-discovered survivor, and produces the fingerprinted triage register.

## Assumption Reassessment (2026-06-19)

1. Codebase check: `.cargo/mutants.toml` carries the standing `crates/tracewake-core/src/epistemics/**` perimeter with `test_workspace = true` and `additional_cargo_args = ["--locked"]`; `.cargo/mutants-baseline-misses.txt` is empty; the four survivor files and surrounding EPI seam are present. The kill tickets 002–008 add the behavior witnesses; this ticket re-runs the whole configured posture so their catches are reconciled against the complete denominator. `reports/0040_epi_cert_mutation_final_missed.txt` holds the canonical 30 identities (21 proposition / 4 belief / 2 contradiction / 3 observation), and `reports/0040_epi_cert_mutation_triage_register.md` is the seed register/anatomy to reuse.
2. Specs/docs check: §4.2 forbids the certifying run from using `--no-config`/`-f`/`--file`/`--exclude`/`--in-diff`/`--iterate` as the final denominator; §4.4 mandates a clean unmutated baseline (`fmt`/`clippy`/`build`/`test` + the 20 named EPI suites) before interpreting any mutant; §4.5 names the default unsharded run `cargo mutants --workspace --no-shuffle -o reports/0041_epi_cert_mutation_full.out`; §4.6 governs sharding/timeout completion proof; §4.7–§4.8 define survivor reconciliation and the triage register schema; §4.3 keeps the baseline-miss file un-laundered. Census capture (§4.2): `reports/0041_epi_cert_mutation_list_files.txt` and `reports/0041_epi_cert_mutation_list.txt`.
3. Cross-artifact shared boundary under audit: the `.cargo/mutants.toml` denominator ↔ `--list-files`/`--list` census ↔ the complete run outcomes ↔ the historical 30 seed identities ↔ the reconciliation register. The register must map every historical identity to its final-checkout identity (or `removed/source-changed` with semantic mapping) and append every new survivor; tool outcomes and certification dispositions are kept as separate axes (a reviewed equivalent stays a `missed` tool outcome, never counted `caught`).
4. Motivating invariants (INV restate): §10 maps the mutation lock layer to `INV-091`–`INV-095`/`INV-098`/`INV-105` — no-human, replay, leakage, possession, view-model, and diagnostic behavior remain regression-tested, and metric improvement cannot replace behavior evidence. Acceptance requires run completion, honest counts, and zero unresolved/untriaged behavior-relevant outcomes — not a mutation score.
5. Fail-closed / actor-knowledge / replay surface (evidence-consumer basis): this ticket audits the deterministic replay, actor-knowledge filtering, and fail-closed validation surfaces by re-running their mutation posture; confirm the reconciliation introduces no leakage or nondeterminism path (the run loads the checked-in config unchanged, retains all shard/outcome artifacts, and reconciles deterministically) and that no historical EPI survivor is laundered into `.cargo/mutants-baseline-misses.txt` (an entry is permissible only with the §4.11 equivalent/non-critical proof + independent signoff, and is still reported as a reviewed `missed`, never `caught`).

## Architecture Check

1. Running the complete configured posture (not a `-f`/`-F`-filtered subset) to completion and reconciling every identity is the only honest denominator: it catches survivors the per-file `-f` boundaries cannot (cross-file mutants, newly-exposed misses, timeout-to-miss transitions) and prevents the named-30-only anti-Goodhart failure. Keeping tool outcomes and certification dispositions as separate axes prevents counting a reviewed equivalent as a kill.
2. No backwards-compatibility aliasing/shims: no `--no-config`/replacement config, no baseline-miss laundering, no sampled-shard pass. Sharding (if used) is a reproducible partition of one complete denominator with full reconciliation.

## Verification Layers

1. INV-091–095 (mutation lock-layer completion) -> replay/golden-fixture check (the campaign itself): the standing configured run completes unsharded or across a provably complete shard set, with `--list`/union reconciliation.
2. Clean-baseline precondition -> `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked` plus the 20 named EPI suites pass before any mutant is interpreted.
3. Reconciliation completeness -> manual review: all 30 historical identities reconciled; every new survivor triaged; zero untriaged; zero unsigned exceptions; zero unresolved timeouts/tool failures.
4. Baseline-miss integrity -> codebase grep-proof: `.cargo/mutants-baseline-misses.txt` contains no laundered EPI survivor.

## What to Change

### 1. Clean baseline and named preflight (§4.4)

Establish a clean unmutated baseline at the final implementation commit (`fmt`/`clippy --deny warnings`/`build --locked`/`test --workspace --locked`), then run the 20 named EPI binaries (core `hidden_truth_gates`, `event_schema_replay_gates`, `acceptance_gates`, `anti_regression_guards`, `generative_lock`, `golden_scenarios`, `negative_fixture_runner`, `spine_conformance`, `no_human_capstone`, `emergence_ledger`; content `fixtures_load`, `forbidden_content`, `golden_fixtures_run`, `schema_conformance`; TUI `adversarial_gates`, `tui_seam_conformance`, `transcript_snapshot`, `tui_acceptance`, `embodied_flow`, `command_loop_session`). Record exact commands, exit results, toolchain/cargo-mutants versions, and transcript references.

### 2. Census capture and certifying run (§4.2, §4.5–§4.6)

Capture `--list-files` and `--list` to `reports/0041_epi_cert_mutation_list_files.txt` / `reports/0041_epi_cert_mutation_list.txt`, proving the four survivor files and the surrounding `epistemics/**` seam remain in the configured census and no exclusion glob/option removes them. Run the complete configured posture (`cargo mutants --workspace --no-shuffle -o reports/0041_epi_cert_mutation_full.out`, or a provably complete shard set with full reconciliation and retained `mutants.json`/`outcomes.json`/logs). Resolve every timeout under a recorded retry policy to caught / killed-by-remediation / approved-equivalent / approved-non-critical / unviable / blocked.

### 3. Reconciliation and triage register (§4.7–§4.8)

Produce `reports/0041_epi_cert_mutation_triage_register.md` reusing the 0040/0039 schema. Track every historical identity by raw 0040 identity, normalized identity, final-checkout identity (or source-change mapping), EPI/responsible-layer mapping, and final disposition/evidence; append every additional final-run survivor. Route any newly-discovered behavior-relevant survivor needing a code fix to a reserved follow-up ticket (`0041EPICERMUT-011` onward). Add a baseline-miss entry only with the §4.11 proof + signoff (reported as a reviewed `missed`).

## Files to Touch

- `reports/0041_epi_cert_mutation_triage_register.md` (new — or the implementer-recorded register path)
- `reports/0041_epi_cert_mutation_list_files.txt` (new — effective `--list-files` census)
- `reports/0041_epi_cert_mutation_list.txt` (new — effective `--list` census)
- `reports/0041_epi_cert_mutation_full.out` (new — complete-run structured output; or the recorded shard output set)
- `.cargo/mutants-baseline-misses.txt` (modify — only if a reviewer-approved equivalent/non-critical exception lands with its evidence reference; otherwise unchanged/empty)

## Out of Scope

- Adding any behavior-witness test (owned by the kill tickets 002–008); this ticket runs and reconciles, it does not author kills.
- Fixing a newly-discovered behavior-relevant survivor (routed to the reserved `0041EPICERMUT-011` onward range).
- The EPI-01…11 live re-proof, evidence packaging, replacement artifact, and aggregate verdict (ticket 010).
- Narrowing the `.cargo/mutants.toml` denominator, using `--no-config`/`-f`/`--in-diff`/`--iterate` as the final run, or laundering survivors into the baseline-miss file (§4.2, §4.3).
- Any `ORD-LIFE-CERT` or later-gate work (§11.2).

## Acceptance Criteria

### Tests That Must Pass

1. The clean unmutated baseline (`fmt`/`clippy`/`build`/`test --workspace --locked`) and all 20 named EPI suites pass at the final implementation commit, recorded with exact commands and results.
2. `cargo mutants --workspace --no-shuffle -o reports/0041_epi_cert_mutation_full.out` (or the complete shard set) runs to completion; `--list-files`/`--list` censuses prove the four survivor files and `epistemics/**` seam remain in the denominator; per-outcome counts reconcile to the union with no duplicate counting.
3. The triage register reconciles all 30 historical identities, appends every new survivor, and records: zero blocked/untriaged identities, zero exceptions lacking independent signoff, zero unresolved timeouts/tool failures. `.cargo/mutants-baseline-misses.txt` contains no laundered EPI survivor.

### Invariants

1. The certifying run loads the checked-in `.cargo/mutants.toml` as the single denominator of record; no `--no-config`/filter/`--in-diff`/`--iterate` final run.
2. Tool outcomes and certification dispositions stay separate axes; a reviewed equivalent/non-critical mutant remains a `missed` tool outcome and is never counted as caught.

## Test Plan

### New/Modified Tests

1. `None — evidence/run-only ticket; verification is command-based (the standing mutation campaign + the 20 named EPI suites named in §4.4) and the deliverable is the captured census, complete-run output, and reconciliation register.`

### Commands

1. `cargo mutants --workspace --no-shuffle --list-files > reports/0041_epi_cert_mutation_list_files.txt && cargo mutants --workspace --no-shuffle --list > reports/0041_epi_cert_mutation_list.txt`
2. `cargo mutants --workspace --no-shuffle -o reports/0041_epi_cert_mutation_full.out`
3. The complete configured `--workspace --no-shuffle` run (not a `-f`/`-F` filter) is the correct verification boundary: §4.2/§4.5 require the certifying denominator to be the whole checked-in config, with the per-file `-f` runs in tickets 002–008 being non-certifying development evidence only.
