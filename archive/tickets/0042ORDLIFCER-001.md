# 0042ORDLIFCER-001: Acceptance-artifact scaffold + §4.1 clean baseline transcript and environment

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — creates the `reports/` ORD-LIFE-CERT acceptance artifact and captures the §4.1 clean-baseline command transcript and environment; runs existing tests only, changes no engine code.
**Deps**: None

## Problem

The ORD-LIFE-CERT audit (`specs/0042_ORD_LIFE_CERT_…_CERTIFICATION_SPEC.md`) is non-executable: it specifies what an implementing session must run, prove, record, and package into a single acceptance artifact. Every per-gate evidence ticket (`0042ORDLIFCER-002`…`-013`), the generated/metamorphic harness ticket (`-014`), the mutation-posture ticket (`-015`), and the capstone (`-016`) write into one shared artifact and depend on a trustworthy clean baseline. Without a scaffolded artifact and a recorded clean baseline first, later tickets have no stable target section to fill and no proof that the unmutated tree is green — and spec §4.1 states that "a failing, flaky, or selectively retried unmutated baseline blocks interpretation of later evidence."

This ticket creates the acceptance artifact from the canonical template, fills its identity/scope header (spec §9.1), and records the §4.1 clean-baseline transcript and determinism-relevant environment.

## Assumption Reassessment (2026-06-20)

1. The acceptance template exists at `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (verified present at target commit `98dc042`); spec §9 instantiates it into `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md`. The `reports/` directory exists; no `reports/0042_*` file collides.
2. Spec §9.1–§9.2 fix the artifact identity fields (exact tested commit vs. target `98dc042`, clean/dirty status, predecessor artifacts and their scoped use, `Work posture: Certification`, `ORD-LIFE-CERT` as a composed label) and the command-ledger requirement; §1/§2 fix the consumed predecessor passes (`archive/reports/0037_…`, `…/0039_…`, `…/0041_…`, all verified present). This ticket records the header and baseline only; per-gate verdicts are owned by `-002`…`-016`.
3. Cross-artifact shared boundary under audit: the single acceptance artifact `reports/0042_…_acceptance.md`. This ticket creates it `(new)`; all sibling tickets `(modify)` their own distinct sections and `Deps` this ticket. The "merge" is append-only on disjoint sections; sequential landing is recommended.
4. Motivating invariant/discipline: spec §4.1 + §2 source discipline. The clean baseline underpins `INV-018 — Deterministic replay is foundational` interpretation: a non-green or selectively-retried baseline makes every downstream live/replay comparison uninterpretable. Restate before trusting any later evidence: the unmutated tree must be green, with every failure/retry retained in the command ledger.
5. Evidence-consumer surface (audit-reads, does not modify): the deterministic-replay and fail-closed-validation surfaces exercised by `cargo test --workspace`. This ticket only *runs* them and records the transcript; it introduces no production code, no leakage path, and no nondeterministic input into any canonical form. Recording the baseline does not weaken epistemic-leakage prevention or replay determinism — it establishes the reference both depend on.

## Architecture Check

1. Scaffolding the artifact once (create + header + baseline) and having every per-gate ticket append its own section is cleaner than each ticket re-creating or racing on the file: one owner of identity/scope, append-only writes elsewhere, and a reviewable create-diff isolated from evidence content.
2. No backwards-compatibility aliasing/shims introduced; this is a new evidence artifact, not a code change. No production logic is added.

## Verification Layers

1. Artifact exists with required §9.1 identity fields -> codebase grep-proof (`test -f` the artifact; grep the mandatory header fields).
2. `INV-018` deterministic-replay baseline integrity -> replay/golden-fixture check (the §4.1 `cargo test --workspace --locked` run is green and its transcript is recorded; retries/flakes retained).
3. Source-discipline / exact-commit honesty (spec §2/§9.1) -> manual review (recorded tested commit equals or explicitly diverges from `98dc042`; clean/dirty and every evidence-only file change enumerated).

## What to Change

### 1. Instantiate the acceptance artifact from the template

Copy `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` into `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` and populate the §9.1 identity/scope block: exact tested commit (and whether it equals target `98dc042`); branch/PR or detached-worktree identity as context only; clean/dirty status and every evidence-only or instrumentation file changed; target repository and exact-commit source discipline; the three consumed predecessor artifacts (`archive/reports/0037_…`, `0039_…`, `0041_…`) with their scoped certification use; `Work posture: Certification` and `ORD-LIFE-CERT` as a composed label; explicit statement that archived specs/tickets/reports are history only; scope (needs, routines, intentions, no-human life, planner traces, stuck diagnostics, the actor-known ordinary-life transaction); excluded/downstream scope from spec §11; and a placeholder aggregate-verdict line owned by `-016`.

Add empty, clearly-labeled section stubs for ORD-LIFE-01…12 (filled by `-002`…`-013`), the generated/metamorphic package (`-014`), the mutation package (`-015`), and the §9.3–§9.11 capstone tables (`-016`), so siblings append into stable anchors.

### 2. Record the §4.1 clean-baseline command ledger and environment

Run the spec §4.1 commands against the tested tree and record, per command: exact invocation, tested commit/worktree identity, exit status, start/end timestamps, Rust + tool versions (`rustc --version`, `cargo --version`, toolchain from `rust-toolchain.toml`), determinism-relevant environment facts, and complete output or a cryptographically fingerprinted transcript with its actual fingerprint scope. Retain every failure, flake, and retry in the ledger. A failing/flaky/selectively-retried unmutated baseline blocks the audit and must be surfaced, not smoothed over.

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (new)

## Out of Scope

- Any ORD-LIFE-01…12 per-gate verdict, behavior witness, or replay/provenance package (owned by `-002`…`-013`).
- Generated/metamorphic harness assertions (`-014`), mutation perimeter/run/triage (`-015`), and the aggregate verdict / §9.3–§9.11 capstone tables (`-016`).
- Any production/engine code change or remediation (spec §2: evidence instrumentation only; remediation routes to a later separately-numbered spec).
- The `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` move (deferred to spec acceptance/archival per `docs/archival-workflow.md`).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` returns success and the file carries the §9.1 identity fields (tested commit, clean/dirty, predecessors, posture, scope) plus the ORD-LIFE-01…12 / harness / mutation / capstone section stubs.
2. `cargo test --workspace --locked` is green on the tested tree, and its full transcript (or fingerprinted transcript + scope) is recorded in the artifact's §4.1 command ledger.
3. The full §4.1 sequence (`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo build --workspace --all-targets --locked`; `cargo test --workspace --locked`; `cargo test --locked -p tracewake-core --doc`) is recorded with exact invocation, exit status, timestamps, and tool versions; any retry/flake is retained, not elided.

### Invariants

1. The artifact records the exact tested commit and states whether it equals target `98dc042`; no claim is made that an unchanged tree passed unless every recorded command ran against that unchanged tree.
2. `INV-018` baseline: the recorded baseline is reproducible and green; a non-green or selectively-retried baseline is reported as such and blocks downstream interpretation rather than being normalized.

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification is command-based (the §4.1 baseline) and existing pipeline coverage is the workspace test suite named in Assumption Reassessment.`

### Commands

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace --all-targets --locked`
4. `cargo test --workspace --locked`
5. `cargo test --locked -p tracewake-core --doc`

## Outcome

Completed: 2026-06-20

Created `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with the required §9.1 identity/scope fields, predecessor-artifact scoping, clean/dirty and target/tested commit honesty, and stable stubs for ORD-LIFE-01 through ORD-LIFE-12, generated/metamorphic evidence, mutation evidence, staged abstraction, EMERGE-OBS, and the capstone aggregate verdict.

The §4.1 clean baseline ran green against tested commit `f7d8d666a8baa220b87d5e037e3eb50c8bf088c5`, which does not equal the spec target `98dc0421211e6c9881d9c6679b9df74525e392bb`; the artifact records that divergence explicitly. The direct first pass and the fingerprinted transcript-capture rerun both passed. Transcript capture files were written under `/tmp` and are not committed artifacts; their byte counts and SHA-256 values are recorded in the report.

Verification:

- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace --locked` — passed.
- `cargo test --locked -p tracewake-core --doc` — passed.

No production or engine code changed. No ORD-LIFE-01 through ORD-LIFE-12 verdict, mutation verdict, generated/metamorphic verdict, or aggregate certification verdict was rendered by this ticket.
