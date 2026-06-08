# 0008PHA3AANTCON-010: Debug/TUI typed projections + why-not distinction

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` debug reports / view models and `tracewake-tui` debug panels render from typed records; why-not distinguishes actor-known uncertainty from ground-truth failure
**Deps**: 0008PHA3AANTCON-002, 0008PHA3AANTCON-006

## Problem

Spec 0008 §8.6 + §9.6: debug panels must be strictly read-only projections from typed event/projection state, not string-canonical assertions. The default player-facing and actor-debug panels must answer: what the actor knew/believed, which need/routine/intention candidate won, which HTN method and local plan were selected, which pipeline validator accepted/rejected, and whether failure was actor-known uncertainty vs hidden-world-truth vs access vs reservation/body constraint vs authored-content invalidity. Omniscient debug comparison stays explicitly labeled and non-authoritative.

This consumes the typed records made authoritative in -002 and emitted by the transaction/flip in -006.

## Assumption Reassessment (2026-06-08)

1. `crates/tracewake-core/src/debug_reports.rs` and `view_models.rs` are the projection surfaces; `crates/tracewake-tui/src/debug_panels.rs` and `render.rs` are the TUI panels. After -002, `AgentState` holds typed `DecisionTraceRecord`/`StuckDiagnosticRecord`; after -006 they are emitted causally by the transaction.
2. Spec §8.6 lists the five questions the default panels must answer; §9.6 mandates "Render from typed records", "Remove substring/string-canonical assertions as primary proof", panels for actor-known-uncertainty vs ground-truth-validation failure, and labeled-non-authoritative omniscient comparison. §13 acceptance: "Debug/TUI surfaces derive from typed records" and "Why-not output distinguishes actor-known uncertainty from ground-truth validation failure."
3. Cross-artifact boundary under audit: the **typed record (core state) ↔ view-model projection ↔ TUI panel** seam — projections must read typed fields and apply actor-knowledge filtering for embodied vs debug views.
4. INV-070 (Why-not explanations are mandatory), INV-067 (Embodied mode shows actor-known reality), INV-068 (Debug mode is visibly non-diegetic): embodied why-not is in actor-known terms; debug-only truth is clearly separated and labeled.
5. Enforcement surface: actor-knowledge firewall (no-leak). Confirm embodied/player-facing panels project only actor-known fields (no hidden physical truth in embodied why-not); the omniscient comparison panel is reachable only in debug mode and labeled non-authoritative. No replay determinism impact — projections are read-only derivations.
6. Extends the view-model projection schema: additive view-model fields surfacing the typed decision-trace/stuck-diagnostic record fields + a structured why-not failure-kind. Consumers: `view_models.rs` consumers in `tracewake-tui` (`debug_panels.rs`, `render.rs`) and view-model tests. Additive (new optional projection fields); existing embodied projections keep their actor-known filtering.

## Architecture Check

1. Rendering from typed records is the doctrine-correct read-only projection: the panel shows the actual decision fields, and the embodied/debug split is enforced by which typed fields each projection exposes — not by parsing canonical strings. A structured failure-kind lets why-not distinguish actor-known uncertainty from ground-truth failure precisely.
2. No backwards-compatibility aliasing: string-canonical panel assertions are replaced by typed-field projections, not kept as a fallback.

## Verification Layers

1. INV-070 why-not → unit test: a blocked action's why-not reports the structured failure kind (actor-known uncertainty vs ground-truth validation failure vs access vs reservation vs content-invalidity).
2. INV-067 embodied actor-known → manual review + test: embodied/player-facing panels project no hidden physical truth; only actor-known fields appear.
3. INV-068 debug non-diegetic → manual review: omniscient comparison panel is debug-only and labeled non-authoritative.
4. Typed-source proof → codebase grep-proof: panels read typed record fields; no substring/string-canonical assertion is the primary proof surface.

## What to Change

### 1. Typed projections in core

`debug_reports.rs`/`view_models.rs` project the typed `DecisionTraceRecord`/`StuckDiagnosticRecord` fields and a structured why-not failure-kind; embodied projections keep actor-knowledge filtering.

### 2. TUI panels from typed records

`tracewake-tui` `debug_panels.rs`/`render.rs` render the five §8.6 questions from the typed projections; the omniscient comparison panel is labeled non-authoritative and debug-only.

## Files to Touch

- `crates/tracewake-core/src/debug_reports.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-tui/src/debug_panels.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)

## Out of Scope

- Authoritative typed record storage/replay (0008PHA3AANTCON-002).
- The transaction emitting the records (0008PHA3AANTCON-004/-006).
- Capstone end-to-end ancestry assertions (0008PHA3AANTCON-011).

## Acceptance Criteria

### Tests That Must Pass

1. Why-not test: a blocked no-human action surfaces a structured failure-kind distinguishing actor-known uncertainty from ground-truth validation failure.
2. No-leak test: embodied/player-facing panels expose no hidden physical truth (actor-known fields only).
3. `cargo test -p tracewake-core view_models && cargo test -p tracewake-tui` green.

### Invariants

1. Debug/TUI panels derive from typed records, not canonical strings.
2. Embodied why-not is actor-known; omniscient comparison is debug-only and labeled.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/view_models.rs` tests — typed projection + structured why-not; embodied no-leak.
2. `crates/tracewake-tui/tests/` (existing debug/embodied flow tests) — panels render typed fields.

### Commands

1. `cargo test -p tracewake-core view_models`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
