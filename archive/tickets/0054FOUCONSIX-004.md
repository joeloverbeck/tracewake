# 0054FOUCONSIX-004: Fail-closed acceptance state machine (process-integrity keystone)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` test/acceptance-guard surface (expected-finding manifest, evidence-ingestion state machine, closed wording grammar, survivor-pass-hole close, parser-mutation campaign) + `.github/workflows/ci.yml` (actual-artifact ingestion); no production `src/` change
**Deps**: None

## Problem

The acceptance taxonomy is self-consistency checking, not fail-closed verification. `crates/tracewake-core/tests/support/acceptance_status_manifest.rs` extracts a fenced `tracewake-acceptance-status` block (`STATUS_FENCE`, `:3`) and recomputes `overall_result` from statuses, but: `REQUIRED_FINDINGS` is hardcoded to `F5-01`…`F5-06` (`:5`), not generic for the next line; a `closed` finding requires only presence of `evidence`/`negative` fields (`compute_result`/`require_field`, `:227`-`:235`/`:271`), not validation that the evidence names live code/current method names/public-boundary behavior/mutation sensitivity; `branch_protection` is a scalar string with no parsed ruleset transcript (`:87`/`:223`); and the `non-blocking-bounded-forcing` mutation arm (`:250`-`:259`) validates survivor rows but **never sets `pass = false`**, so a manifest with explicit survivor rows can still compute `Pass` (the survivor-pass hole). `acceptance_artifact_wording.rs` is a phrase denylist (paraphrase-evadable), not a closed grammar. The block is self-authored and not independently derived from CI/ruleset/mutation artifacts, and nothing forces the **actual** acceptance artifact under review to be ingested and made merge-blocking (finding F6-04, the keystone).

## Assumption Reassessment (2026-06-27)

1. `crates/tracewake-core/tests/support/acceptance_status_manifest.rs` confirms: `REQUIRED_FINDINGS` hardcoded (`:5`); `validate_status_manifest`/`parse_status_block`/`compute_result` (`:51`/`:109`/`:220`); the `closed` arm requires only `evidence`/`negative` presence (`:228`-`:231`); `branch_protection`/`mutation_status` scalars (`:87`-`:88`); the `non-blocking-bounded-forcing` arm requires survivor rows + forcing fields but does not fail the result (`:250`-`:259`). `has_mutation_survivors` (`:104`) exists but the bounded-forcing arm bypasses it. Confirmed at `7660051`.
2. `crates/tracewake-core/tests/acceptance_status_manifest.rs` (the driving test) and `crates/tracewake-core/tests/acceptance_artifact_wording.rs` (phrase denylist) exist; `crates/tracewake-core/tests/ci_workflow_guards.rs` pins CI topology (`governance_audit_errors` `:363`, `CI_YML`/`MUTANTS_TOML`/`DOC10` consts). `.github/workflows/ci.yml` defines the required-context set (`:174`-`:182`). `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` exists (closed-grammar home is ticket 007). Confirmed.
3. Shared boundary under audit: the acceptance-evidence machinery (`acceptance_status_manifest.rs` support + driving test + wording guard + `ci_workflow_guards.rs` + `ci.yml`). Extend existing machinery — no new test framework, gate code, invariant, risk ID, or glossary term. This is the keystone the §3 three-layer barrier binds to; it interlocks with governance (ticket 005) and mutation (ticket 006) on the same files (sequential-edit chain 004→005→006).
4. INV-098 (feature acceptance is harsh — done only when caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe, LLM-independent, non-scripted, regression-tested) and the architecture/execution evidence-honesty contract (`docs/1-architecture/13`, `docs/2-execution/10`). Restated: a computed `pass` must reflect current, complete evidence — a self-consistent block is not proof.
5. Fail-closed validation surface: this ticket IS the fail-closed acceptance validation. Confirm it stays deterministic and blocking, distinguishes a recorded honest survivor (`non-blocking-bounded-forcing`) from a green-closure artifact (never pass with survivors), and that no parser change weakens the existing closed-finding checks. The parser/guard functions get their own mutation campaign — a guard whose own mutants survive is decorative.

## Architecture Check

1. Converting the parser from a consistency recompute into a positive, fail-closed state machine that computes from current evidence makes a laundered "pass" structurally impossible rather than discouraged. Driving the expected finding set from an explicit review/artifact manifest (not hardcoded labels) means the next remediation line needs no source edit per line — the recurring maintenance smell that produced the F5/F6 hardcoding is removed.
2. No backwards-compatibility aliasing/shims: the phrase denylist is replaced by a closed grammar keyed to computed state, not layered beside it; the survivor-pass arm is fixed, not flagged. The closed verdict grammar's doctrine home (template `0003`) lands with ticket 007.

## Verification Layers

1. Fail-closed computation (INV-098) → deterministic adversarial-artifact tests: each synthetic artifact computes `NonPass`/parse-error as specified (codebase grep-proof + `cargo test` over the manifest test).
2. Survivor-pass hole closed → a synthetic `non-blocking-bounded-forcing` artifact with explicit survivor rows computes `NonPass` (regression test for `:250`-`:259`).
3. Guard non-vacuity → a mutation campaign over the parser/guard functions kills their mutants (focused `cargo mutants` over the support module).
4. Actual-artifact ingestion → a `ci_workflow_guards.rs` assertion that CI ingests the real acceptance artifact and fails a synthetic closure PR lacking a current pass-eligible artifact.

## What to Change

### 1. Expected-finding manifest (drop hardcoded labels)

Replace the hardcoded `REQUIRED_FINDINGS` with the current line's finding set sourced from an explicit review/artifact manifest, so the parser accepts `F6-01`…`F6-07` (and future sets) without a source edit per line.

### 2. Fail-closed `Pass` computation

`Pass` is impossible if any finding is open, pending, routed-forward, historical-only for a current requirement, not-in-scope for an in-scope requirement, or if evidence cannot be tied to current exact-commit files/tests. Any mutation survivor, missed mutant, timeout, untriaged baseline miss, or bounded-forcing survivor computes `NonPass` for a green-closure artifact — close the survivor-pass hole in the `non-blocking-bounded-forcing` arm (a bounded-forcing survivor may be recorded honestly but never as pass).

### 3. Closed wording grammar

Replace the `acceptance_artifact_wording.rs` phrase denylist with a closed grammar keyed to the computed state: free prose may explain but cannot introduce a contradictory or stronger verdict than the computed state.

### 4. Actual-artifact CI ingestion

CI must ingest the actual acceptance artifact proposed for merge/archive and parse it with the status parser, closed wording grammar, governance-transcript parser (ticket 005), mutation-evidence parser (ticket 006), and the expected-finding manifest; fail if no current acceptance artifact is present for a closure PR, if it is not at the expected path, if any status is not pass-eligible, if governance independence is pending, or if mutation evidence is stale/non-green.

### 5. Parser/guard mutation campaign

Add a focused mutation campaign over the parser/guard functions themselves.

## Files to Touch

- `crates/tracewake-core/tests/support/acceptance_status_manifest.rs` (modify)
- `crates/tracewake-core/tests/acceptance_status_manifest.rs` (modify)
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify)
- `.github/workflows/ci.yml` (modify)

## Out of Scope

- The governance-transcript approval-field parsing (ticket 005) and the PR-blocking mutation context (ticket 006) — this ticket builds the parser hooks they extend; their CI/guard edits land in 005/006 (sequential-edit chain on the shared files).
- The doctrine home for the closed grammar (template `0003`, arch 13, exec 10) — ticket 007.
- Any production `src/` change; any new test framework or property-testing dependency.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test acceptance_status_manifest` — synthetic adversarial artifacts each compute the specified result: survivor-pass under bounded-forcing → `NonPass`; paraphrased closure over open rows → `NonPass`; stale method-name negative evidence → `NonPass`; branch/ruleset scalar without transcript → `NonPass`; zero-approval governance → `NonPass`; missing actual-artifact ingestion → fail; self-authored-only evidence → `NonPass`; historical-current conflation → `NonPass`; display-only evidence → `NonPass`.
2. `cargo test -p tracewake-core --test acceptance_artifact_wording` — the closed grammar rejects a verdict stronger than the computed state.
3. `cargo test -p tracewake-core --test ci_workflow_guards` — a synthetic closure PR lacking a current pass-eligible artifact fails the ingestion guard.
4. `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. No manifest with any open/pending/routed-forward/non-current row or any live survivor computes `Pass`.
2. The parser/guard functions are non-vacuous (their own mutants are killed).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_status_manifest.rs` — adversarial synthetic-artifact cases (item-1 list) + expected-finding-manifest generality.
2. `crates/tracewake-core/tests/acceptance_artifact_wording.rs` — closed-grammar cases replacing the phrase denylist.
3. `crates/tracewake-core/tests/ci_workflow_guards.rs` — actual-artifact ingestion guard.

### Commands

1. `cargo test -p tracewake-core --test acceptance_status_manifest --test acceptance_artifact_wording --test ci_workflow_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. `cargo mutants -f crates/tracewake-core/tests/support/acceptance_status_manifest.rs` — the parser/guard mutation campaign (a guard whose own mutants survive is decorative).

## Outcome

Completed: 2026-06-27

Converted the acceptance-status parser from hardcoded F5 labels and self-consistency checks into a manifest-driven fail-closed state machine. Status blocks now declare `expected_findings`, the parser accepts the current F6 finding set without source-level label edits, closed rows must cite current file/test evidence plus current negative evidence, legacy scalar branch-protection claims compute `NonPass`, and `non-blocking-bounded-forcing` survivors can be recorded honestly but never compute `Pass`.

Reworked the wording guard cases around the computed state: pass-shaped wording over non-pass manifests fails, green-perimeter wording with survivor rows fails, branch-protection claims without a transcript fail, historical-current conflation fails, and display-only/self-authored evidence is rejected by the parser/grammar path. CI now has an `Ingest changed acceptance artifacts` step that discovers changed acceptance artifacts, requires a `tracewake-acceptance-status` block, and feeds each actual artifact path into the same `acceptance_status_manifest` parser through `TRACEWAKE_ACCEPTANCE_ARTIFACT`; `ci_workflow_guards` now fails if that ingestion hook is removed.

Verification run:

- `cargo test -p tracewake-core --test acceptance_status_manifest --test acceptance_artifact_wording --test ci_workflow_guards` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
- `git diff --check` — passed.

Mutation evidence:

- `cargo mutants --list -f crates/tracewake-core/tests/support/acceptance_status_manifest.rs --no-config` listed 0 focused mutants for the test-support parser file.
- The required command `cargo mutants -f crates/tracewake-core/tests/support/acceptance_status_manifest.rs` selected 3445 mutants under repository config and was interrupted after the selection line. Result recorded as incomplete; no mutation pass is claimed here. Standing mutation completion remains ticket 009 scope.

Scope note: ticket 005 still owns the full independent-governance transcript parser, and ticket 006 still owns PR-blocking mutation-context hardening. This ticket landed their fail-closed parser and CI-ingestion hooks without pulling those later ticket surfaces forward.

Unrelated pre-existing dirty paths left untouched: `.claude/skills/spec-to-tickets/SKILL.md` and `.claude/skills/spec-to-tickets/references/decomposition-patterns.md`.
