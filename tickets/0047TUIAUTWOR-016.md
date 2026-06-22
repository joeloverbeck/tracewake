# 0047TUIAUTWOR-016: Full-world / possession / anti-leak differential suite

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — test-only: human/no-human one-tick differential + possession-parity + anti-leak suite
**Deps**: 0047TUIAUTWOR-013, 0047TUIAUTWOR-010

## Problem

Spec 0047 §4.8 requires a differential scenario: from an identical initial log and deterministic inputs, advance one tick under a human-wait disposition and under the equivalent no-human/controller disposition; with the actor choice held equal, the authoritative physical transition, duration-completion set, need ledger, and replay checksum must match — only controller-origin metadata and the actor-facing command report may differ. Plus the §4.9-item-13 full-world / possession / anti-leak coverage. This proves possession changes input binding only (not world progression) and that a full-world tick advances unpossessed actors/processes identically under both controllers.

## Assumption Reassessment (2026-06-22)

1. Both controllers now loop the *same* coordinator: the human path via the TUI world-step entry (0047TUIAUTWOR-011) and the no-human path via the refactored `run_no_human_day`/`advance_no_human` (0047TUIAUTWOR-010). This shared single tick definition (§8) is the precondition that makes a held-equal differential meaningful — without it (pre-0047TUIAUTWOR-010) the two paths had different progression and the differential would prove nothing.
2. The differential holds the actor choice equal and compares: authoritative physical transition, duration-completion set, need ledger, replay checksum (must match) vs controller-origin metadata + actor-facing command report (may differ). The `TimeAdvanced` marker's controller/process origin is the expected-to-differ metadata.
3. Cross-artifact boundary under audit: this is a relational (paired-run) witness over the human and no-human dispositions of one initial log. It asserts a hyperproperty (two runs equal modulo controller origin) the single-run witnesses (0047TUIAUTWOR-014/015) cannot; it owns no production surface.
4. Constitutional invariant motivating the ticket: `INV-005`/`INV-006`/`INV-094` (possession parity — controller binding changes input only; beliefs/intentions/possessions remain with actors) and `INV-108` (human possession is cognition-neutral), plus `INV-093` (actor-knowledge leakage is a high-severity defect — the anti-leak half).
5. Enforcement surface audited (evidence-consumer basis): the suite reads the replay checksum, the need ledger, the duration-completion set, and the interval summary across both dispositions. It confirms held-equal inputs produce byte-identical authoritative outcomes (only controller metadata differs), that unpossessed actors and due processes advance under the same timeline, and that a hidden other-actor event leaves the embodied summary unchanged. No synthetic events; the differential + anti-leak suite is the possession-parity / no-leak regression lock.

## Architecture Check

1. A held-equal human-vs-no-human differential over one coordinator is the strongest possible proof that possession is cognition-neutral and that the world advances independently of the controller — exactly the property `INV-005`/`INV-108` demand and the property the original single-controller defect threatened. It is only meaningful because 0047TUIAUTWOR-010 unified the tick definition; the ticket dependency encodes that.
2. No backwards-compatibility aliasing/shims: the suite drives both dispositions through their real paths; it does not stub a controller or synthesize a matching log.

## Verification Layers

1. `INV-005`/`INV-094`/`INV-108` possession parity -> replay/golden-fixture check: human and no-human dispositions of an identical initial log produce identical authoritative physical transition, duration-completion set, need ledger, and replay checksum; only controller-origin metadata + the actor-facing report differ.
2. `INV-093` no leak -> manual review (epistemic-leakage audit): a hidden other-actor event leaves the possessed actor's embodied summary unchanged across the advance.
3. Full-world advance -> replay/golden-fixture check: unpossessed actors and due world processes advance under the same timeline in both dispositions.

## What to Change

### 1. Human/no-human one-tick differential (new test)

From an identical initial log + deterministic inputs, advance one tick under the human-wait disposition and under the equivalent no-human/controller disposition; assert the authoritative physical transition, duration-completion set, need ledger, and replay checksum match, and that only controller-origin metadata + the actor-facing command report differ.

### 2. Possession-parity + anti-leak coverage (new tests)

Assert unpossessed actors/processes advance identically under both controllers; assert a hidden other-actor event leaves the embodied interval summary unchanged (no leak).

## Files to Touch

- `crates/tracewake-tui/tests/parity_adversarial.rs` (modify — differential + anti-leak cases) or a new `crates/tracewake-core/tests/human_no_human_differential.rs` (decide at implementation; the differential's checksum comparison can live in core, the anti-leak render check in TUI)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — file created by 0047TUIAUTWOR-005) — core-side checksum/ledger differential

## Out of Scope

- Any production logic (0047TUIAUTWOR-005…013).
- The single-run sleep/work witnesses (0047TUIAUTWOR-014/015).
- Registering these as parity capability entries (0047TUIAUTWOR-017 consumes them).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` — the human/no-human differential over an identical initial log yields matching authoritative physical transition, duration-completion set, need ledger, and replay checksum; only controller-origin metadata + the actor-facing report differ.
2. A hidden other-actor event leaves the possessed actor's embodied interval summary unchanged; unpossessed actors/processes advance identically under both controllers.
3. `cargo clippy --workspace --all-targets -- -D warnings` clean.

### Invariants

1. Possession changes input binding only; held-equal inputs produce byte-identical authoritative outcomes modulo controller metadata (`INV-005`/`INV-094`/`INV-108`).
2. No hidden-truth leak survives the advance into the embodied summary (`INV-093`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — core-side human/no-human checksum + ledger differential.
2. `crates/tracewake-tui/tests/parity_adversarial.rs` (modify) — anti-leak + possession-parity render-side cases.

### Commands

1. `cargo test -p tracewake-core differential`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`
3. The full-workspace boundary is correct: the differential compares core checksums and TUI render dispositions of the same initial log, so both crates are exercised.
