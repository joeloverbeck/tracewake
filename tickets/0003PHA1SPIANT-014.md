# 0003PHA1SPIANT-014: Named spine conformance suite + scoped acceptance artifact (capstone)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new named conformance test surface mapping each `SPINE-AC-*` to a test by responsible layer; new acceptance-artifact format / scoped-certification-wording doc
**Deps**: 0003PHA1SPIANT-001, 0003PHA1SPIANT-002, 0003PHA1SPIANT-003, 0003PHA1SPIANT-004, 0003PHA1SPIANT-005, 0003PHA1SPIANT-006, 0003PHA1SPIANT-007, 0003PHA1SPIANT-008, 0003PHA1SPIANT-009, 0003PHA1SPIANT-010, 0003PHA1SPIANT-011, 0003PHA1SPIANT-012, 0003PHA1SPIANT-013

## Problem

The spine hardening lands as ~13 separate tickets, each adding its own tests. Without a named, discoverable conformance surface that maps each `SPINE-AC-*` requirement to at least one test by responsible layer, the architectural rules stay scattered and a reviewer cannot confirm one-to-one coverage. And without a scoped acceptance-artifact format, an implementation PR risks overclaiming certification ("P0-CERT passed") instead of the correct scoped wording. Spec `0003` §7 SPINE-AC-013 (named conformance suite) and SPINE-AC-015 (scoped acceptance artifact) are the aggregate-acceptance deliverables this capstone closes.

## Assumption Reassessment (2026-06-08)

1. The prior tickets in this batch (`0003PHA1SPIANT-001`…`013`) each add named tests across `tracewake-core` (events/replay/state/scheduler/actions/agent), `tracewake-content` (schema/validation), `tracewake-tui` (view-model/debug), and `workspace/ci` (clippy + doc linter). Existing conformance-ish suites: `crates/tracewake-core/tests/anti_regression_guards.rs`, `hidden_truth_gates.rs`; `crates/tracewake-content/tests/forbidden_content.rs`; `crates/tracewake-tui/tests/adversarial_gates.rs`. No `spine_conformance.rs` / `schema_conformance.rs` / `tui_seam_conformance.rs` exists yet (verified absent).
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-013 mandates a named conformance surface reporting failures by responsible layer (`core/events`, `core/replay`, `core/state`, `core/scheduler`, `core/actions`, `core/agent`, `content/schema`, `content/validation`, `tui/view-model`, `tui/debug`, `workspace/ci`) with each `SPINE-AC-*` mapped to ≥1 named test. SPINE-AC-015 mandates a scoped acceptance artifact (exact commit, gates run, changed files, per-requirement evidence, residual convention-only items) and scoped certification wording (allowed: "Phase 1 / Phase 1A spine hardening remediation accepted for this commit"; forbidden: "Project is P0 certified" / "SPINE-CERT passed" unless the upstream process declares it).
3. Cross-artifact boundary under audit: the requirement→test mapping spanning all three crates plus CI, and the acceptance-artifact wording contract (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:38-77` typed-evidence/responsible-layer doctrine; `docs/4-specs/README.md` posture discipline). This capstone exercises the prior tickets' surfaces; it introduces no new production logic — only the named aggregation suite (test infra) and the acceptance-artifact doc.
4. INV motivating this ticket: the full cited set (`INV-008`, `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-020`, `INV-058`–`INV-063`, `INV-091`–`INV-108`) plus `P0-CERT` posture discipline. Restated: every architectural rule must have a named test and the acceptance claim must be scoped — no overclaiming, no rule living only in prose.

## Architecture Check

1. A single named conformance suite that maps each `SPINE-AC-*` to its test by responsible layer gives reviewers a one-glance coverage proof and a stable home for future spine rules, while the scoped acceptance-artifact format prevents the certification-overclaim failure the spec explicitly forbids. As a capstone it adds discoverable aggregation (test infra) + a doc, not new mechanics — it exercises and indexes the pipeline the prior tickets built.
2. No backwards-compatibility shim: the suite references the real per-ticket tests (no duplicated/shadow assertions), and the acceptance wording admits no "P0-CERT passed" escape.

## Verification Layers

1. Requirement coverage (all `SPINE-AC-*`) -> conformance test: a mapping test asserts each requirement has ≥1 named test discoverable in the suite, grouped by responsible layer.
2. `INV-018`/`INV-024` (determinism + no-leak end-to-end) -> replay/golden-fixture + adversarial check: the suite runs the prior tickets' determinism and no-leak gates together (no regression across the integrated spine).
3. `P0-CERT` posture discipline -> manual review + grep-proof: the acceptance-artifact doc uses scoped wording and contains the forbidden phrases only inside a "forbidden wording" rejection list, never as a claim.

## What to Change

### 1. Named conformance suite with by-layer reporting

Add the conformance surface (`crates/tracewake-core/tests/spine_conformance.rs`, `crates/tracewake-content/tests/schema_conformance.rs`, `crates/tracewake-tui/tests/tui_seam_conformance.rs`, or equivalent discoverable modules) that maps each `SPINE-AC-001`…`015` requirement to ≥1 named test and reports failures by responsible layer.

### 2. Coverage assertion

Add a test asserting each `SPINE-AC-*` requirement is represented (e.g. a checklist constant cross-checked against named test functions), so a future requirement without a test fails.

### 3. Scoped acceptance-artifact format

Add `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (or a section the implementation PR fills): exact commit, gates run, changed files, per-requirement acceptance evidence, residual convention-only items + rationale, and the allowed/forbidden certification wording.

## Files to Touch

- `crates/tracewake-core/tests/spine_conformance.rs` (new)
- `crates/tracewake-content/tests/schema_conformance.rs` (new)
- `crates/tracewake-tui/tests/tui_seam_conformance.rs` (new)
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (new — acceptance-artifact format + scoped wording)

## Out of Scope

- The per-requirement implementation and its tests (owned by 0003PHA1SPIANT-001…013) — this capstone indexes and exercises them, it does not reimplement them.
- Flipping any ledger/spec status — landing the spec's `docs/4-specs/SPEC_LEDGER.md` entry happens when the spec is promoted to `docs/4-specs/`, outside this batch (noted in the decomposition summary).
- Declaring `P0-CERT`/`SPINE-CERT` results — wording stays scoped.

## Acceptance Criteria

### Tests That Must Pass

1. The named conformance suite runs under `cargo test --workspace` and maps each `SPINE-AC-001`…`015` to ≥1 named test, reporting by responsible layer.
2. A coverage assertion fails if a `SPINE-AC-*` requirement has no mapped test.
3. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, and `cargo test --workspace` all pass (the four workspace gates).

### Invariants

1. Every `SPINE-AC-*` requirement maps to ≥1 named, discoverable test by responsible layer (spec §9.2 acceptance checklist).
2. The acceptance artifact uses scoped certification wording; forbidden phrases appear only in a rejection list (`P0-CERT` posture discipline).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/spine_conformance.rs` — `SPINE-AC-*`→test mapping + by-layer reporting.
2. `crates/tracewake-content/tests/schema_conformance.rs` — content-layer conformance index.
3. `crates/tracewake-tui/tests/tui_seam_conformance.rs` — TUI-seam conformance index.

### Commands

1. `cargo test --workspace`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked`
