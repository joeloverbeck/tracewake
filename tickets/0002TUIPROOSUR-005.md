# 0002TUIPROOSUR-005: Typed action availability + typed checked facts

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`SemanticActionEntry` schema, new `ActionAvailability`, typed `CheckedFact`)
**Deps**: 0002TUIPROOSUR-001, 0002TUIPROOSUR-003

## Problem

`SemanticActionEntry.why_disabled: Option<String>` (`crates/tracewake-core/src/view_models.rs:173-181`) is the canonical availability reason, and `CheckedFact` carries `key`/`value` strings (`crates/tracewake-core/src/actions/report.rs:81-93`) even though `ReasonCode` is already a typed enum (`report.rs:10`). Display strings are acting as diagnostics and can leak hidden truth or be parsed by tests as authority. Spec 0002 §4 TUI-AC-003 requires `SemanticActionEntry` to carry a typed `ActionAvailability` (stable reason codes, actor-visible safe summary, provenance references, optional debug-only diagnostics) and `CheckedFact` to be typed; display text becomes render-only.

## Assumption Reassessment (2026-06-08)

1. `SemanticActionEntry` is at `crates/tracewake-core/src/view_models.rs:174` with `why_disabled: Option<String>` (`:173-181`). `CheckedFact` (`report.rs:81`) has string `key`/`value`; `ReasonCode` (`report.rs:10`) is already a typed enum. `render_embodied_view` formats the disabled reason for display (`crates/tracewake-tui/src/render.rs`).
2. Spec 0002 §4 TUI-AC-003: typed `ActionAvailability` with reason codes, actor-safe summary, provenance refs, debug-only diagnostics; tests assert typed codes/provenance, not substrings; renderers may format typed reasons but formatting is never consumed by validation/affordance selection/lead generation.
3. Cross-artifact boundary under audit: the `SemanticActionEntry` view contract ↔ `ReasonCode`/`CheckedFact` in `actions/report.rs`. Availability is populated by the affordance builder (ticket 003) using sealed-context provenance (ticket 001).
4. Invariants restated: **INV-105** — decision/why-not diagnostics must be typed or structurally inspectable; display strings may summarize but must not be the authoritative substrate; **INV-106** — validation failure feedback exposes only modeled, actor-available consequences; **INV-070** — why-not is given in actor-known terms.
5. Fail-closed / no-leak surface: `ActionAvailability` structurally separates the actor-safe summary + reason codes from debug-only diagnostics. Confirm the actor-visible path exposes only the safe subset and provenance refs the actor can know; debug-only diagnostics are reachable only via debug construction (the capability lands in ticket 004; the two-layer why-not split that consumes both is ticket 007). This ticket must not place hidden-truth detail in the actor-visible `ActionAvailability`.
6. Schema extension: `SemanticActionEntry` — replace `why_disabled: Option<String>` with `availability: ActionAvailability`; type `CheckedFact`. Consumers: `render_embodied_view` (`render.rs`), the affordance builder (`projections.rs`), and `tracewake-tui`/`tracewake-core` tests asserting disabled reasons. Breaking — internal, all consumers updated here.
7. Rename/removal blast radius: `why_disabled` is removed. Pre-implementation grep `why_disabled` across `crates/`, `docs/`, `specs/`: matches in `view_models.rs` (def), `projections.rs` (set), `render.rs` (read), and tests join Files to Touch; no consumer lives outside these crates.

## Architecture Check

1. A typed `ActionAvailability` makes reason codes and provenance the authoritative substrate and relegates strings to rendering, so tests assert semantics and a wording change cannot silently alter behavior — directly satisfying INV-105 and removing the "string-as-diagnostic" leak vector.
2. No backwards-compatibility aliasing/shims: `why_disabled: Option<String>` is removed, not kept beside the typed field; the renderer formats the typed value rather than reading a retained string.

## Verification Layers

1. INV-105 (typed diagnostics) -> codebase grep-proof: `SemanticActionEntry` exposes `availability: ActionAvailability` with typed reason codes; `why_disabled: Option<String>` is gone.
2. INV-106 (non-leaking feedback) -> manual review + test: actor-visible `ActionAvailability` contains only safe summary + actor-knowable provenance; debug-only diagnostics are a separate, debug-gated field.
3. INV-070 (actor-known why-not) -> schema validation: a disabled entry exposes a stable reason code + actor-safe summary with no hidden target id.

## What to Change

### 1. `ActionAvailability` type

Add a typed `ActionAvailability` (available/disabled with stable `ReasonCode`(s), actor-safe summary, provenance references, optional debug-only diagnostics) and put it on `SemanticActionEntry` in place of `why_disabled: Option<String>`.

### 2. Typed `CheckedFact`

Type `CheckedFact` (`report.rs:80-112`) so facts carry a typed key/category + source binding rather than free strings; keep a render formatter for display.

### 3. Populate from sealed context

Have the affordance builder (ticket 003) populate `ActionAvailability` from sealed-context provenance and preflight reason codes.

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-core/src/actions/report.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)

## Out of Scope

- `ValidationReport` actor/debug two-layer split (ticket 007) — this ticket types the per-entry availability, not the post-rejection report.
- Debug capability type (ticket 004).
- Proposal context fields (ticket 006).

## Acceptance Criteria

### Tests That Must Pass

1. A disabled action exposes typed reason code(s) + actor-safe summary + provenance refs; a test mutating only the display text leaves the typed assertions passing (typed code unchanged).
2. A test asserts no actor-visible `ActionAvailability` field contains hidden-target identifiers.
3. `cargo test -p tracewake-core` passes; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Typed reason codes/provenance are authoritative; display strings are render-only (INV-105).
2. Actor-visible availability never carries debug-only/hidden detail (INV-106).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/view_models.rs` / `actions/report.rs` (unit tests) — `ActionAvailability` shape, typed `CheckedFact`.
2. `crates/tracewake-tui/tests/embodied_flow.rs` — disabled-entry rendering asserts typed code, not substring.

### Commands

1. `cargo test -p tracewake-core view_models && cargo test -p tracewake-core report`
2. `grep -rn "why_disabled" crates/` (must return zero after implementation)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
