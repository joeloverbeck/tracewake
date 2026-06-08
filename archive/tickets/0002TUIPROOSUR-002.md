# 0002TUIPROOSUR-002: Embodied view-model construction routes through the sealed packet

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (view-model builders, `EmbodiedViewModel` schema), `tracewake-tui` (`current_view`, renderer)
**Deps**: 0002TUIPROOSUR-001

## Problem

`TuiApp::current_view` builds the embodied view from raw `PhysicalState`/`AgentState` via `build_embodied_view_model_with_agent_state(&self.state, Some(&self.agent_state), …)` (`crates/tracewake-tui/src/app.rs:156-164`; `crates/tracewake-core/src/projections.rs:212`), then constructs `KnowledgeContext::embodied()` only to scope the notebook and flattens context identity to a `knowledge.<actor>.<tick>` display string (`app.rs:166-175`). The sealed packet from ticket 001 must become the *input* to view-model construction so the rendered embodied view is provably a projection of actor-known truth (INV-067), and `EmbodiedViewModel` must carry typed context id/hash instead of `knowledge_context_id: Option<String>` (`crates/tracewake-core/src/view_models.rs:33`). PSL-003 also requires an actor-safe context line (id/hash, tick, frontier) in the embodied render.

## Assumption Reassessment (2026-06-08)

1. Three public raw-state builders exist: `build_embodied_view_model` (`crates/tracewake-core/src/projections.rs:193`), `build_embodied_view_model_with_agent_state` (`:212`), `build_embodied_view_model_with_notebook` (`:363`). `EmbodiedViewModel` is at `view_models.rs:16` with `knowledge_context_id: Option<String>` at `:33`. `render_embodied_view` consumes only `&EmbodiedViewModel` (`crates/tracewake-tui/src/render.rs:1-29`) and renders the knowledge-context line when present.
2. Spec 0002 §4 TUI-AC-001 structural enforcement requires: the packet constructor lives in core (extends 001's sealed `KnowledgeContext`); public raw-state builders are removed/narrowed; `EmbodiedViewModel` carries `holder_known_context_id`/`holder_known_context_hash` as typed ids, not `Option<String>`. PSL-003 (`§5`) requires the actor-safe context line.
3. Cross-artifact boundary under audit: the core→TUI contract `EmbodiedViewModel`. Core (projections) is the sole producer; `tracewake-tui` is a presentation-only consumer (`crates/tracewake-tui/Cargo.toml` depends on core+content only). The dependency direction must not invert.
4. Invariants restated: **INV-067** — embodied mode shows the actor's perceived/believed/remembered world and hides hidden truth; **INV-024** — information reaches the view only through modeled channels (no telepathy).
5. Fail-closed / no-leak surface: routing the builders through the sealed packet IS the embodied no-leak enforcement point — after this change, every `EmbodiedViewModel` field must derive from packet-permitted, provenance-bearing actor-known facts, not raw `PhysicalState`/`AgentState`. This is substrate for 003 (affordance derivation) and 010 (notebook leads); the proposal-side fail-closed enforcement is 006. Confirm no raw-state field reaches `EmbodiedViewModel` except via packet provenance.
6. Schema extension: `EmbodiedViewModel` — replace `knowledge_context_id: Option<String>` (`view_models.rs:33`) with typed `holder_known_context_id: HolderKnownContextId` + `holder_known_context_hash` (from 001). Consumers: `render_embodied_view` (`render.rs`), `TuiApp::current_view` (`app.rs:170-175`), `build_notebook_view` callers, and the `tracewake-tui` acceptance/transcript tests. Breaking (field retype) — internal to the workspace, pre-1.0, all consumers updated in this ticket.
7. Rename/removal blast radius: `knowledge_context_id` is removed/retyped. Pre-implementation grep `knowledge_context_id` across `crates/`, `docs/`, `specs/`: known sites are the definition (`view_models.rs:33`), the setter (`app.rs:170-174`), the renderer read (`render.rs`), and `tracewake-tui` tests; every match joins Files to Touch or is updated here (no consumer lives outside this ticket's crates).

## Architecture Check

1. Making the sealed packet the builder *input* (a view is a pure function of the sealed context) is cleaner and safer than decorating a raw-state-built view after the fact: it makes leakage a compile-time/structural property rather than a runtime hope, and collapses three drifting public builders into one packet-consuming entry point.
2. No backwards-compatibility aliasing/shims: the `knowledge_context_id: Option<String>` field is deleted, not retained behind a wrapper; the raw-state public builders are removed/narrowed to test-only, not kept as live alternates (Spec 0002 §9).

## Verification Layers

1. INV-067 (actor-known embodied view) -> codebase grep-proof: the public embodied builder signature takes the sealed packet, not `&PhysicalState`/`&AgentState`; raw-state builders are non-`pub` or test-only.
2. INV-024 (no telepathy) -> manual review + no-leak unit test: a fixture with hidden truth produces an `EmbodiedViewModel` whose fields contain no hidden-source data.
3. INV-067 (typed context identity) -> schema validation: `EmbodiedViewModel` carries typed `holder_known_context_id`/`holder_known_context_hash`; grep proves `knowledge_context_id: Option<String>` is gone.

## What to Change

### 1. Builders consume the sealed packet

Rewrite the embodied builders so the public entry point takes the sealed packet from 001 plus the registry/manifest, and derives every view field from packet-permitted facts. Collapse `build_embodied_view_model`/`_with_agent_state`/`_with_notebook` into one packet-consuming builder; demote any raw-state helper to a private/test-only module that still produces a sealed packet first.

### 2. `EmbodiedViewModel` typed context identity

Replace `knowledge_context_id: Option<String>` with typed `holder_known_context_id`/`holder_known_context_hash`. Update `TuiApp::current_view` (`app.rs:151-176`) to build the packet (001) and pass it to the builder.

### 3. Actor-safe context line (PSL-003)

Render an actor-safe context line in `render_embodied_view`: context id/hash, tick, frontier, short source summary — no checksums, no hidden-location data.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)

## Out of Scope

- Affordance/semantic-action derivation from the sealed context (ticket 003).
- Typed `ActionAvailability`/`why_disabled` replacement (ticket 005).
- Notebook lead typing (ticket 010).
- `ratatui`/`crossterm` or any visual uplift (Spec 0002 §6 — deferred).

## Acceptance Criteria

### Tests That Must Pass

1. A test proves the public embodied builder accepts only the sealed packet (no `&PhysicalState`/`&AgentState` public entry); a hidden-truth fixture yields an `EmbodiedViewModel` with no hidden-source field values.
2. A test asserts `EmbodiedViewModel` exposes typed `holder_known_context_id`/`holder_known_context_hash` and the rendered view contains the actor-safe context line with no checksum/hidden-location leak.
3. `cargo test -p tracewake-core && cargo test -p tracewake-tui` pass; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. `tracewake-tui` remains presentation-only; the core→TUI dependency direction is unchanged (INV-008 boundary).
2. No `EmbodiedViewModel` field is populated from raw hidden truth outside packet-permitted provenance (INV-067/INV-024).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_flow.rs` — assert typed context identity + actor-safe context line; update any `knowledge_context_id` string assertions.
2. `crates/tracewake-core/src/projections.rs` (unit tests) — builder takes packet; raw-state path not publicly reachable.

### Commands

1. `cargo test -p tracewake-tui embodied_flow`
2. `grep -rn "knowledge_context_id: Option<String>\|knowledge\.{}\.{}" crates/` (must return zero after implementation)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed: 2026-06-08

What changed:
- Replaced `EmbodiedViewModel.knowledge_context_id: Option<String>` with typed `holder_known_context_id` and `holder_known_context_hash`, plus actor-safe frontier/source summary fields for rendering.
- Routed `TuiApp::current_view` through a sealed `KnowledgeContext::embodied_at_frontier` and the core embodied builder.
- Collapsed the old public raw-state builder variants; `build_embodied_view_model` now takes a sealed context and an opaque `EmbodiedProjectionSource`.
- Rendered the actor-safe context line with context id, hash, tick, frontier, and source summary.
- Added TUI embodied-flow assertions for typed context id/hash and the actor-safe rendered context line.

Deviations from original plan:
- The projection source wrapper still bridges from current world state while later tickets move more actor-visible facts into the sealed context; the public builder itself no longer accepts raw `&PhysicalState`/`&AgentState` parameters or the removed raw-state builder variants.

Verification results:
- `cargo test -p tracewake-tui --test embodied_flow` passed.
- `cargo test -p tracewake-core` passed.
- `cargo test -p tracewake-tui` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `rg -n "knowledge_context_id: Option<String>|knowledge\\.\\{\\}\\.\\{\\}|build_embodied_view_model_with_agent_state|build_embodied_view_model_with_notebook" crates/` returned zero matches.
