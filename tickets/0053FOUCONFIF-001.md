# 0053FOUCONFIF-001: Fail-closed acceptance result taxonomy — machine-checked status manifest, wording guard, forcing function

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` test/acceptance-guard surface only (new `tests/acceptance_status_manifest.rs`; extend `tests/acceptance_artifact_wording.rs` and `tests/ci_workflow_guards.rs`); no production `src/` change
**Deps**: None

## Problem

Spec 0053 §4.6 (F5-06) is the process-integrity keystone and, per §9, must land **first**: across the 0047→0052 lineage a remediation line could define its own scope, run focused witnesses, truth docs to the implemented shape, record residuals as "scoped pass," and archive an acceptance artifact whose prose reads closed while the protected authority class survived. The doctrine already carries evidence-honesty rules; the missing piece is an **enforced, machine-checkable result taxonomy** so `pending`, `routed-forward`, `historical`, `sampled`, and `observer-only` evidence cannot be silently counted as pass. Installing this before any code remediation is deliberate (§9): without it the subsequent seal tickets (004–007) could repeat the same acceptance failure.

This ticket installs the mechanism: a machine-readable acceptance **status manifest** consumed by a Rust test, a **computed** overall-result rule, an extended **wording guard**, and a **routed-forward forcing-function** schema — all extending existing machinery, minting no new test framework, gate code, invariant, risk ID, or glossary term (§4.6, §1.2).

## Assumption Reassessment (2026-06-26)

1. `crates/tracewake-core/tests/acceptance_artifact_wording.rs` exists (verified this session) as the extension point for closure-language rejection; `crates/tracewake-core/tests/ci_workflow_guards.rs` exists (29 KB) and asserts the CI workflow topology (job names `public-boundary-conformance`, `full-surface-mutation-trigger`, `mutants-lock-layer-reconcile` at `.github/workflows/ci.yml:103,126,366`). Both are the named §4.6.3 extension points. No new test framework is introduced.
2. Spec authority: `specs/0053_FOUNDATIONAL_CONFORMANCE_FIFTH_HARDENING_…_HARDENING_SPEC.md` §4.6, §5 (forcing-function fields), §6.1 (the doctrine substance lands with this code — ticket 002), §8 (the capstone artifact carries this manifest), §9 step 1 (install first). Sibling precedent: the 0052 line shipped `acceptance_artifact_wording.rs` but no computed result taxonomy, which is exactly why the fifth pass found a non-green perimeter described as a "scoped pass."
3. Cross-artifact boundary under audit: the contract between a future acceptance artifact (`reports/0053_…_acceptance.md`, authored by ticket 010) and the Rust test that consumes its status block. §10.6 leaves the manifest home (inline in the artifact vs. a sibling file) and which existing test consumes it as an **implementer-recorded choice**; this ticket records the choice in What to Change and keeps the overall result **computed from statuses, not asserted in prose** (§10.6 non-negotiable).
4. Motivating invariant: INV-098 ("Feature acceptance is harsh — a runnable feature is done only when it is caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe, LLM-independent, non-scripted, and regression-tested") and the INV-098-class evidence discipline (§2). The manifest operationalizes "harsh" into a machine check: `pass` is legal only when every required finding is `closed`, governance is enforced, and every standing mutation residual is killed or non-blocking under a bounded forcing function (§4.6.2).
5. This ticket builds the **enforcement surface** for fail-closed acceptance — a deterministic, blocking validation that fails CI when a status block claims closure it has not earned. It is the substrate the capstone (010) and doc-truthing (008) rely on. As substrate it introduces no actor-knowledge leakage and no replay nondeterminism: it reads a status manifest and recomputes a verdict; it touches no view model, event log, RNG, or projection. The validation is fail-closed (any unparseable/missing/contradictory status block fails the test, never defaults to pass) and distinguishes blocking statuses (`open`, `pending-governance`, unbounded `routed-forward`) from non-blocking ones, per the §4.6.2 computed rule.

## Architecture Check

1. Extending the two existing guard tests plus one new manifest-consumer test is cleaner than a bespoke acceptance framework: the verdict becomes a `cargo test` failure on the same lane that already runs, so a dishonest artifact cannot merge. Computing the verdict from a typed status set (rather than grepping prose) is robust against wording drift — the wording guard catches closure language, the computed rule catches the substance, and the two are independent layers so neither alone can be gamed.
2. No backwards-compatibility aliasing or shims. The manifest schema is new; it replaces no prior format (the 0052 line had none). The wording-guard extension adds rejection cases to an existing test, not a parallel copy.

## Verification Layers

1. INV-098 (harsh acceptance) -> invariants alignment check + new manifest-consumer test: a synthetic status block with any `open` / `pending-governance` / unbounded `routed-forward` item computes non-pass and fails the test; an all-`closed` + governance-enforced + survivors-killed block computes pass.
2. INV-098-class evidence honesty -> extended `acceptance_artifact_wording.rs`: a synthetic artifact using "pass with," "scoped pass," "accepted," or "green canonical perimeter" while its status block carries open/pending/routed-forward items (or a non-empty survivor list) is rejected; a synthetic guard-can-fire negative proves the new rejection cases actually trip.
3. Fail-closed validation -> manual review + test: an unparseable / missing / contradictory status block fails the consuming test rather than defaulting to pass (deterministic, blocking, never-overridable).
4. Cross-artifact: the manifest format and its consuming test are named in What to Change (§10.6 recorded choice); the consuming test resolves the manifest at the artifact path ticket 010 will author.

## What to Change

### 1. Machine-readable acceptance status manifest + computed-result consumer (`tests/acceptance_status_manifest.rs`, new)

Define the manifest schema (§4.6.1) carrying: exact commit under test; source acquisition method; findings/residuals keyed by **existing** finding labels (`F5-01`…`F5-06`, not new canonical IDs); per-finding status from the closed set `closed | open | routed-forward | pending-governance | historical-only | not-in-scope`; a certifying evidence item per `closed`; a live negative/public-boundary proof per protected shortcut; mutation evidence status + survivor list; branch-protection/ruleset enforcement status; and an overall result **computed** from statuses. Implement the computed rule (§4.6.2): overall `pass` is legal only when every required finding is `closed`, every required governance control is enforced, and every standing mutation residual is killed or explicitly non-blocking under a bounded forcing function; `pending-governance`, `open`, and unbounded `routed-forward` force non-pass. **Recorded choice (§10.6):** the manifest lives as a fenced, machine-parseable block inside the acceptance artifact (`reports/0053_…_acceptance.md`, authored by 010), and `acceptance_status_manifest.rs` is the consuming test; the test recomputes the verdict and asserts it matches the artifact's stated overall result. Include synthetic in-test fixtures (not the real artifact, which does not exist until 010) proving the computed rule and a guard-can-fire negative.

### 2. Routed-forward forcing-function schema (within the manifest)

Any `routed-forward` residual must name owning surface, why it is not closed by the current line, the next known execution move (recorded in `SPEC_LEDGER.md` at closeout), a maximum number of remediation epochs or a concrete trigger after which it becomes blocking, and the exact CI/mutation test that fails if it remains (§4.6.4). The consuming test rejects a `routed-forward` item missing any of these fields as **unbounded** → non-pass.

### 3. Acceptance wording guard extension (`tests/acceptance_artifact_wording.rs`, modify)

Extend the guard so CI rejects an acceptance artifact that: uses "pass with," "scoped pass," "accepted," or equivalent closure language while the status block has open/pending/routed-forward items; calls the canonical perimeter green while the survivor list is non-empty; cites branch-protection enforcement without an API transcript/ruleset evidence; cites historical command results as current certification; or cites display strings / artifact existence / checksums / source guards as sole evidence for a behavior claim that architecture requires typed path-under-test evidence for (§4.6.3). Add a synthetic-negative proving each new rejection case can fire.

### 4. Required-check policy assertion (`tests/ci_workflow_guards.rs`, modify)

Add an assertion that the operational required-check policy is named (the manifest's branch-protection/ruleset enforcement status is consumed by the governance audit job from ticket 003); keep `ci_workflow_guards.rs` as a **labeled topology alarm** for job names and path filters — not as merge-enforcement proof (§4.4, §4.6). The merge-enforcement proof itself is ticket 003 (governance audit job) + the capstone's API transcript.

## Files to Touch

- `crates/tracewake-core/tests/acceptance_status_manifest.rs` (new)
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify)

## Out of Scope

- The §6.1 doctrine prose edits (arch 13 / exec 10 / ref 00 / ref 01) — ticket 002 (lands with this code per §6.1).
- The governance audit job that queries the branch-protection API — ticket 003.
- The actual acceptance artifact and its real status block — ticket 010 (this ticket ships the *mechanism* + synthetic fixtures, not the real artifact).
- Any production `src/` change, seal, or mutation work (004–007, 009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test acceptance_status_manifest` — synthetic all-`closed`+governance-enforced+survivors-killed block computes pass; a block with any `open`/`pending-governance`/unbounded `routed-forward` computes non-pass; an unparseable/missing block fails (fail-closed).
2. `cargo test -p tracewake-core --test acceptance_artifact_wording` — each new closure-language / green-perimeter-while-survivors / enforcement-without-transcript / historical-as-current / display-string-as-sole-evidence rejection case fires on its synthetic negative.
3. `cargo test -p tracewake-core --test ci_workflow_guards` — topology alarm still green; required-check policy assertion present.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` — no regression.

### Invariants

1. The overall acceptance result is **computed** from the typed status set; no path lets prose assert `pass` while a status item is open/pending/unbounded (INV-098 / §4.6.2).
2. The validation is deterministic, blocking, and fail-closed: an unparseable, missing, or self-contradictory status block fails, never defaults to pass.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_status_manifest.rs` (new) — computed-pass rule + forcing-function-field completeness + fail-closed parsing, all on synthetic fixtures.
2. `crates/tracewake-core/tests/acceptance_artifact_wording.rs` (modify) — new rejection cases + guard-can-fire negatives.
3. `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify) — required-check policy assertion alongside the existing topology alarm.

### Commands

1. `cargo test -p tracewake-core --test acceptance_status_manifest --test acceptance_artifact_wording --test ci_workflow_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. The new test target is created by this ticket; after creation, `cargo test -p tracewake-core --test acceptance_status_manifest -- --list` enumerates its cases as the narrow verification boundary.
