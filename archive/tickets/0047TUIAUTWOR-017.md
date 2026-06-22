# 0047TUIAUTWOR-017: spec-0046 parity registry + two-hop closure + CI

**Status**: DONE
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/tracewake-tui/tests/parity/*` (capability registry + scenario runner extension); CI evidence lane
**Deps**: 0047TUIAUTWOR-014, 0047TUIAUTWOR-015, 0047TUIAUTWOR-016

## Problem

Spec 0047 §4.8/§1.11/§4.9-item-14 requires extending the spec-0046 capability registry, scenario runner, and dispositions (without replacing or weakening spec-0046) with real TUI/core operations for one world step and advance-until, plus capability entries for the new playable capabilities, under the standing PAR-010/PAR-011 obligation, wired into ordinary CI. This is the two-hop closure (Hop-1 simulation→view-model, Hop-2 view-model→render) for the new time-control capabilities, extending the 21-capability baseline.

## Assumption Reassessment (2026-06-22)

1. The spec-0046 parity harness exists under `crates/tracewake-tui/tests/parity/` (`mod.rs`, `runner.rs`, `scenario.rs`, `census_families.rs`, `census_actions.rs`) plus `crates/tracewake-tui/tests/playable_capability_parity.rs`. `SetupOperation::RunNoHumanDay` is a registry setup operation there (`parity/scenario.rs:116`, `playable_capability_parity.rs:172`). The 21-capability baseline (`capability_count=21`) is recorded in `archive/reports/0046-parity-acceptance-artifact.md`. This ticket **extends** that registry; it does not replace it.
2. PAR-010 (`archive/specs/0046_*.md:261`) obligates every future feature spec to carry a parity-impact section listing added/changed capabilities + dispositions; PAR-011 (`:270`) obligates wiring the conformance suite + focused goldens into ordinary CI under the warnings-denied posture. This ticket discharges both for spec 0047.
3. Cross-artifact boundary under audit: the parity registry is the spec-0046 two-hop drift guard; this ticket adds time-control/duration-completion/interval-summary capability entries and the real one-world-step + advance-until operations to the scenario runner, then closes Hop-1 (capability→view-model) and Hop-2 (view-model→render) for each. It must not weaken any existing spec-0046 disposition.
4. Constitutional invariant motivating the ticket: `INV-066`/`INV-095` (every runnable phase has a TUI/view-model acceptance gate; mechanics must be reachable, used, inspected, regression-tested through the TUI) and `INV-094` (possession parity tested).
5. Enforcement surface audited (evidence-consumer basis): each new capability entry carries a typed event/source witness, actor-known positive and negative/anti-leak witnesses, a rendered golden or explicit no-render disposition, a replay/rebuild witness, a no-human/possession-parity disposition, and a Hop-2 field/destructure disposition where the view model changes. The registry extension adds no leakage path (the anti-leak witnesses are mandatory per entry) and preserves determinism (focused goldens). CI wiring makes the obligation run on every change.

## Architecture Check

1. Extending the existing spec-0046 registry + runner — rather than building a parallel time-control test harness — keeps one parity authority and one drift guard, so the new capabilities are subject to the same two-hop closure as the 21 baseline capabilities. A separate harness would let the new capabilities drift independently of the baseline guard.
2. No backwards-compatibility aliasing/shims: capability entries are added to the registry; no spec-0046 entry is weakened or duplicated, and the `RunNoHumanDay` setup operation is not repurposed as a human-path witness.

## Verification Layers

1. `INV-066`/`INV-095` TUI acceptance gate -> replay/golden-fixture check: each new capability's Hop-1 (real-pipeline view-model) and Hop-2 (rendered golden / no-render) disposition passes in the parity suite.
2. `INV-094` possession parity -> replay/golden-fixture check: the human/no-human differential (0047TUIAUTWOR-016) is registered as a parity disposition.
3. PAR-011 CI -> skill dry-run / CI config check: the conformance suite + focused goldens run in the ordinary CI evidence lane under warnings-denied.

## What to Change

### 1. Extend the capability registry + scenario runner (`tests/parity/*`)

Add real TUI/core operations for one world step and advance-until to the scenario runner, and add capability entries (each with typed event/source witness, actor-known positive + negative/anti-leak witnesses, rendered golden or no-render disposition, replay/rebuild witness, no-human/possession-parity disposition, Hop-2 field/destructure disposition).

### 2. Register the new capabilities (the ≥6 enumerated entries)

Enumerated capability entries (the §4.8 quantifier expanded): (1) human wait advances one authoritative world tick; (2) human-started sleep reaches a terminal with correct recovery; (3) human-started work reaches a terminal with output or modeled failure; (4) an open body-exclusive duration rejects ordinary wait/action but accepts typed continuation; (5) an actor-known interval summary renders positive source-bearing information and suppresses hidden world activity; (6) advance-until stops on an actor-known salient interruption without leaking a hidden cause. Plus register the human/no-human differential disposition.

### 3. CI wiring (PAR-011)

Wire the extended conformance suite + focused goldens into the ordinary workspace/CI evidence lane under the warnings-denied posture, so the parity obligation runs on every change.

## Files to Touch

- `crates/tracewake-tui/tests/parity/census_families.rs` (modify — new capability entries)
- `crates/tracewake-tui/tests/parity/runner.rs` (modify — new world-step / advance-until operations)
- `crates/tracewake-tui/tests/parity/scenario.rs` (modify — scenario operations)
- `crates/tracewake-tui/tests/playable_capability_parity.rs` (modify — registry + count assertion)
- `.github/workflows/` CI config (modify — PAR-011 evidence lane, as the existing parity lane is configured)

## Out of Scope

- The witnesses themselves (0047TUIAUTWOR-014/015/016 produce them; this ticket registers them as parity entries).
- Weakening or rewriting any spec-0046 capability entry.
- The acceptance artifact (0047TUIAUTWOR-018).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui --test playable_capability_parity` — the registry's capability count increases past the 21 baseline by exactly the new entries (count re-enumerated from the registry, not hardcoded), and each new entry's Hop-1 + Hop-2 dispositions pass.
2. Each of the 6 enumerated capability entries carries a typed event/source witness, actor-known positive + negative/anti-leak witnesses, a rendered golden or explicit no-render disposition, a replay/rebuild witness, and a no-human/possession-parity disposition.
3. The CI evidence lane runs the extended conformance suite under warnings-denied (`cargo test --workspace` + `cargo clippy --workspace --all-targets -- -D warnings` green in the lane).

### Invariants

1. The spec-0046 registry is extended, not weakened; the 21 baseline entries remain present and passing (`INV-066`/`INV-095`).
2. Every new playable capability / changed view-model field has a parity surface disposition (PAR-010), and the suite runs on every change (PAR-011).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/parity/*` (modify) — ≥6 new capability entries + world-step/advance-until runner operations.
2. `crates/tracewake-tui/tests/playable_capability_parity.rs` (modify) — registry count re-enumeration + per-entry two-hop assertions.

### Commands

1. `cargo test -p tracewake-tui --test playable_capability_parity`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`
3. The `--test playable_capability_parity` target is the focused parity boundary; the full-workspace run is the CI evidence-lane boundary PAR-011 requires.

## Outcome

Completed: 2026-06-22

Evidence:
- Extended `SetupOperation` and the real-pipeline parity scenario runner with human wait/world-step, sleep advance-until, work advance-until, and open-duration wait-conflict operations.
- Added six `spec0047_tui_authoritative_world_advance` future-pack capability entries while preserving the 21-entry spec-0046 baseline.
- Added a registry test proving the 21 baseline entries plus exactly the six spec-0047 time-control entries are present.
- Verified `.github/workflows/ci.yml` already runs `cargo test -p tracewake-tui --test playable_capability_parity --locked`, `cargo test --workspace --locked`, and warnings-denied clippy, so no workflow churn was needed.
- Passed `cargo test -p tracewake-tui --test playable_capability_parity`.
- Passed `cargo fmt --all --check`.
- Passed `cargo test --workspace`.
- Passed `cargo clippy --workspace --all-targets -- -D warnings`.
