# 0004PHA1THIHAR-010: Turn conformance capstones into evidence-kind matrices

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` and `tracewake-tui` conformance tests upgraded from named-test indexes to evidence-kind matrices
**Deps**: 0004PHA1THIHAR-003, 0004PHA1THIHAR-004, 0004PHA1THIHAR-006, 0004PHA1THIHAR-007, 0004PHA1THIHAR-008, 0004PHA1THIHAR-009

## Problem

`spine_conformance.rs` and `tui_seam_conformance.rs` map each requirement to a named test and verify only that the function name exists by `source.contains("fn <name>(")` — a no-op function with the right name satisfies them (spec §6 F-013, §8 THIRD-AC-006). String presence is not property satisfaction. The capstones must become evidence matrices recording evidence kind, positive/negative class, responsible layer, invariants, and acceptance condition, and must reject string-presence-only proof for high-risk gates.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, fail-closed meta-conformance ticket. -->

1. `crates/tracewake-core/tests/spine_conformance.rs` defines `SpineEvidence` (`:4`) and `SPINE_EVIDENCE` (`:43`), with `spine_conformance_maps_every_spine_requirement_to_named_evidence` (`:191`) calling `assert_test_exists` (`:258`), which checks `source.contains("fn <name>(")`. `crates/tracewake-tui/tests/tui_seam_conformance.rs` uses the same named-test-existence style.
2. The remediation is spec §8 `THIRD-AC-006` + the §9.3 conformance fixtures (`conformance_entry_without_evidence_kind_fails`, `conformance_entry_string_presence_only_rejected_for_high_risk_gate`, `conformance_matrix_requires_negative_evidence_for_no_direct_and_debug_quarantine`), reassessed this session.
3. Shared boundary under audit: this is a near-capstone meta-conformance ticket — the matrix asserts that strong-kind evidence exists, so it depends on the negative-test producers shipping first: ticket 003 (no-direct-dispatch / event-sourcing negatives), 004 (determinism scanner smoke), 006 (checksum parity), 007 (schema/replay rejections), 008 (content no-script), 009 (debug quarantine). All are declared in `Deps`.
4. Motivating invariants (restated): the spec assigns `INV-091`, `INV-092`, `INV-105` to conformance/acceptance evidence — notably `INV-092` (deterministic replay is tested). The matrix is the structural proof that each high-risk requirement has real, not nominal, evidence.
5. Fail-closed meta-conformance surface: the matrix test is itself a deterministic, blocking gate. Confirm it hard-fails when an entry has no evidence kind, when a high-risk gate carries only `StringPresenceOnly`, and when no-direct-dispatch or debug-quarantine lacks negative evidence. It reads test sources only — no actor-knowledge or replay state.

## Architecture Check

1. An evidence matrix — each entry carrying an evidence kind (`CompileTime`, `RuntimeNegative`, `ReplayDeterminism`, `SchemaRejected`, `ChecksumParity`, `CapabilityBoundary`, `CIGate`), a positive/negative class, the responsible layer, invariants, and an acceptance condition — that rejects `StringPresenceOnly` for high-risk gates is strictly stronger than name existence: a renamed no-op cannot satisfy a `RuntimeNegative` or `CompileTime` requirement. Named-test existence is retained only as an index, never as the sole evidence for a high-risk gate.
2. No backwards-compatibility shims: the evidence struct is extended in place; the old name-only check becomes one (insufficient-on-its-own) field, not a parallel aliased path.

## Verification Layers

1. `INV-092` (conformance integrity) -> test: `conformance_entry_without_evidence_kind_fails`.
2. High-risk gate rigor -> test: `conformance_entry_string_presence_only_rejected_for_high_risk_gate`.
3. Negative-evidence requirement -> test: `conformance_matrix_requires_negative_evidence_for_no_direct_and_debug_quarantine`.

## What to Change

### 1. Extend the evidence structs

Add `evidence_kind`, positive/negative `class`, `layer`, `invariants`, and `acceptance_condition` fields to `SpineEvidence` and the TUI seam evidence struct.

### 2. Require strong-kind evidence for high-risk gates

For event sourcing, no-direct-dispatch, determinism, debug quarantine, content no-script, and typed diagnostics, require ≥1 strong-kind evidence item and fail any entry that is `StringPresenceOnly`. Map the negative tests produced by tickets 003/006/007/008/009 as the strong-kind evidence.

## Files to Touch

- `crates/tracewake-core/tests/spine_conformance.rs` (modify — evidence-kind matrix; SPINE-AC entries reference the negative tests; coordinate any SPINE-AC-001 evidence update with ticket 001)
- `crates/tracewake-tui/tests/tui_seam_conformance.rs` (modify — evidence-kind matrix for the TUI seam)

## Out of Scope

- The negative/positive tests themselves (owned by tickets 003, 006, 007, 008, 009) — this ticket references them as evidence, it does not author them.
- The CI lock (ticket 012).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test spine_conformance && cargo test -p tracewake-tui --test tui_seam_conformance` — the matrices pass with strong-kind evidence for every high-risk gate.
2. `cargo test --workspace` — a synthetic `StringPresenceOnly` entry for a high-risk gate fails the matrix.
3. `cargo build --workspace --all-targets --locked` — the extended evidence structs compile.

### Invariants

1. Every high-risk requirement carries ≥1 strong-kind evidence item; no high-risk gate is satisfied by name existence alone.
2. No-direct-dispatch and debug quarantine each carry negative evidence.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/spine_conformance.rs` — evidence-kind matrix + `conformance_entry_without_evidence_kind_fails`, `conformance_entry_string_presence_only_rejected_for_high_risk_gate`, `conformance_matrix_requires_negative_evidence_for_no_direct_and_debug_quarantine`; rationale: replace name existence with property-evidence enforcement.
2. `crates/tracewake-tui/tests/tui_seam_conformance.rs` — TUI-seam evidence matrix; rationale: same upgrade for the TUI seam.

### Commands

1. `cargo test -p tracewake-core --test spine_conformance && cargo test -p tracewake-tui --test tui_seam_conformance`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
