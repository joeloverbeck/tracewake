# 0048FOUCONHAR-007: Parity-runner measured outputs and adversarial variants

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — test-harness only: replaces the parity runner's declaration-as-proof fields with measured structured evidence returned by the real path, and adds adversarial scenario variants that must fail the conformance runner. No production code.
**Deps**: 004, 005, 006

## Problem

Spec 0048 §4.7: the 0046/0047 parity scenario runner executes real fixture/TUI operations and renderer checks, but `ScenarioWitnesses::ordered_witnesses` echoes the registry's declared assertion strings and `CoverageRow::typed_witness` is true when the declaration text is non-empty — inventory metadata, not measured typed evidence — and the real scenario assertions establish neither temporal-frontier reconstruction nor holder-known provenance closure. This overclaims against R-27 (reachability overstatement) and R-29 (decorative locks). The remediation: replace declaration-as-proof for load-bearing 0048 properties with measured scenario outputs returned by the real path, and add adversarial variants that must fail the actual conformance runner.

This ticket lands the parity-evidence half of §8 closure step 6, depending on the final authority path (ticket 004), the wired interval/salient surfaces (ticket 005), and the authoritative differential (ticket 006) it references rather than re-derives.

## Assumption Reassessment (2026-06-23)

1. The parity runner lives at `crates/tracewake-tui/tests/parity/runner.rs` (where `ScenarioWitnesses::ordered_witnesses` and `CoverageRow::typed_witness` are defined) with scenarios in `crates/tracewake-tui/tests/playable_capability_parity.rs`. `typed_witness` being true on non-empty declaration text, and `ordered_witnesses` echoing declared strings, are the decorative-evidence defects — confirmed against §4.7's description; the implementer reconciles the exact field shapes at the cited locations.
2. Spec 0048 §4.7 and §8 (closure step 6) own this; §4.7 instructs that the §4.1 differential (ticket 006) becomes the authoritative human/no-human witness and parity scenarios reference that artifact or reproduce its measured checks rather than converting an assertion sentence into a boolean. §5.4 of the report (carried as spec §7) adds: distinguish registry declaration metadata from measured typed evidence; scope the old differential as duration/accounting evidence until replaced (ticket 006 supplies the replacement).
3. Cross-artifact boundary under audit: the parity conformance contract spanning the scenario runner's evidence fields and the real path's measured outputs — reconstructed temporal frontier + marker count (ticket 001), nonzero autonomous actor/process work (tickets 003/004), exact duration terminal + one-charge classifications, sealed context/frontier identifiers and provenance-resolving source IDs (tickets 002/005), typed stop reason (ticket 005), replay match including temporal projection. Pass/fail must derive from these structured outputs, not declaration text.
4. Constitutional invariants motivating this ticket: `INV-091` (no-human tests mandatory) and `INV-094` (possession parity tested) — the parity runner is a load-bearing witness for both, and execution `10`'s typed-before-rendered / anti-vacuity rules (and risks R-27/R-29) require measured evidence, not declaration metadata.
5. Enforcement surface (actor-knowledge + deterministic-replay, evidence-consumer basis): the runner audits the holder-known provenance and replay surfaces; the measured outputs must read the sealed context/frontier identifiers and provenance-resolving source IDs (tickets 002/005) and the reconstructed frontier (ticket 001) without itself leaking hidden state or introducing nondeterminism. Confirm the adversarial variant that passes a hidden event with a matching `actor_id` but no holder-known source *fails* the runner — i.e. the runner cannot be satisfied by a non-holder-known input.

## Architecture Check

1. Deriving pass/fail from structured evidence returned by the real path (reconstructed frontier + marker count, nonzero actor/process work, exact terminals, sealed identifiers, provenance-resolving source IDs, typed stop reason, replay match) makes the parity runner prove what its labels claim, closing R-27/R-29. Referencing the ticket-006 differential as the authoritative human/no-human witness — rather than re-deriving it in the runner — keeps one authoritative behavior artifact and avoids a second, weaker copy. Adversarial variants that must fail are what convert the runner from inventory metadata into a real lock.
2. No backwards-compatibility aliasing/shims: `typed_witness` stops being a non-empty-string boolean; the human-readable witness descriptions may remain, but pass/fail derives from measured evidence — the decorative path is removed, not retained alongside.

## Verification Layers

1. `INV-091`/`INV-094` measured parity -> replay/golden-fixture check: each load-bearing scenario's pass/fail derives from structured outputs (frontier + marker count, nonzero actor/process work, terminals, sealed identifiers, source-ID provenance, typed stop reason, replay-incl-temporal match).
2. R-27/R-29 anti-vacuity -> manual review + codebase grep-proof: `CoverageRow::typed_witness` is no longer satisfied by non-empty declaration text; `ordered_witnesses` reflects measured outcomes, not echoed declarations.
3. Adversarial closure -> replay/golden-fixture check: each variant — omit autonomous actor invocation; omit world-process invocation; hidden event with matching `actor_id` but no holder-known source; external final tick with a removed marker; populated witness description returning empty typed evidence — fails the real conformance runner.
4. Single-layer note N/A — three distinct concerns map to three distinct proof surfaces above.

## What to Change

### 1. Measured structured evidence in the runner

In `crates/tracewake-tui/tests/parity/runner.rs`, replace the declaration-as-proof fields for load-bearing 0048 properties with measured structured outputs returned by the real path: reconstructed temporal frontier + marker count, nonzero autonomous actor/process work where required, exact duration terminal and one-charge classifications, sealed context/frontier identifiers, source IDs that resolve through allowed provenance, typed stop reason, replay match including temporal projection, and explicit debug/embodied type disposition. `CoverageRow::typed_witness` derives from measured evidence; human-readable descriptions may remain but do not constitute proof.

### 2. Adversarial variants

In `crates/tracewake-tui/tests/playable_capability_parity.rs`, add variants that deliberately omit autonomous actor invocation, omit world-process invocation, pass a hidden event with a matching `actor_id` but no holder-known source, supply a desired final tick externally while removing a marker, and leave a witness description populated while returning empty typed evidence — each asserted to fail the conformance runner. Re-scope any mapping that cited the old differential as duration/accounting evidence (now superseded by ticket 006's authoritative differential).

## Files to Touch

- `crates/tracewake-tui/tests/parity/runner.rs` (modify — measured structured evidence; `ScenarioWitnesses`/`CoverageRow`)
- `crates/tracewake-tui/tests/playable_capability_parity.rs` (modify — adversarial must-fail variants; re-scope old-differential mappings)

## Out of Scope

- The authoritative held-equal differential itself — ticket 006 (this ticket references or reproduces its measured checks).
- Any production code change — the runner exercises the final authority path established by tickets 003/004/005.
- Mutation campaigns, the evidence report, and the conformance/risk-doc updates — ticket 008.
- Editing archived 0046/0047 parity artifacts (§0 archived-history discipline — immutable).

## Acceptance Criteria

### Tests That Must Pass

1. Each load-bearing parity scenario's pass/fail derives from measured structured outputs (frontier + marker count, nonzero actor/process work, terminals, sealed identifiers, provenance-resolving source IDs, typed stop reason, replay match), not declaration text.
2. Every adversarial variant fails the real conformance runner.
3. `cargo test -p tracewake-tui` passes with the measured-evidence runner.

### Invariants

1. `CoverageRow::typed_witness` is not satisfiable by non-empty declaration text; pass/fail is measured (`INV-091`/`INV-094`; R-27/R-29).
2. The runner cannot be satisfied by a non-holder-known input (the hidden-same-`actor_id`-no-source variant fails).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/parity/runner.rs` (modify) — measured structured evidence replaces declaration-as-proof.
2. `crates/tracewake-tui/tests/playable_capability_parity.rs` (modify) — adversarial must-fail variants.

### Commands

1. `cargo test -p tracewake-tui --test playable_capability_parity`
2. `cargo test -p tracewake-tui` (full-crate parity-runner build).

## Outcome

Completed: 2026-06-23

Implemented the parity-runner hardening as test-harness only:

1. `ScenarioWitnesses` now returns measured evidence booleans and measured witness strings from the real TUI/app path instead of echoing registry assertion text.
2. `CoverageRow::typed_witness` and rendered status now derive from measured scenario evidence. The runner fails if typed, actor-knowledge, rendered, or replay-required measurements are absent.
3. Load-bearing time-control rows now require measured world-step frontier/marker evidence, duration terminal evidence, source-bearing holder-known interval evidence, typed stop reason evidence, and the no-human autonomy row requires measured autonomous work plus markers.
4. Added adversarial measurement-probe variants for populated witness text with empty typed evidence, omitted autonomous work, omitted no-human marker/process evidence, non-holder-known actor context, and external frontier without marker evidence.

Deviations from original plan:

- The runner reads measured replay-state match through the existing public projection rebuild debug panel (`diffs=0`) rather than adding production accessors for raw replay reports; this keeps the change test-side as scoped.
- The ticket-006 authoritative differential remains the non-vacuous human/no-human actor/process witness; ticket 007 references that closure by making the parity runner fail closed on missing measured no-human/autonomous-work evidence rather than duplicating the full differential.

Verification:

1. `cargo test -p tracewake-tui --test playable_capability_parity` — passed.
2. `cargo test -p tracewake-tui` — passed.
3. `cargo clippy -p tracewake-tui --all-targets -- -D warnings` — passed.
4. `cargo fmt --all --check` — passed.
5. `git diff --check` — passed.
