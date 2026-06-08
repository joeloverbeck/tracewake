# 0002TUIPROOSUR-007: Two-layer non-leaking why-not

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`ValidationReport`/`WhyNotView` actor/debug fact split), `tracewake-tui` (actor vs debug why-not render)
**Deps**: 0002TUIPROOSUR-004, 0002TUIPROOSUR-005, 0002TUIPROOSUR-006

## Problem

`WhyNotView` maps from `ValidationReport` + reason codes (`crates/tracewake-core/src/view_models.rs:54-95`), but actor/debug summaries and checked facts are strings (`crates/tracewake-core/src/actions/report.rs:80-112`), so there is no structural guarantee that actor-visible why-not omits hidden contents, hidden routes, culprit identity, exact hidden food/source, or validation-only truth. Spec 0002 §4 TUI-AC-009 and §5 PSL-001 require actor-visible why-not to be a safe projection of typed validation results over actor-known context, with debug why-not rendered separately in non-diegetic panels behind the debug capability.

## Assumption Reassessment (2026-06-08)

1. `WhyNotView` is at `crates/tracewake-core/src/view_models.rs:55` and maps from `ValidationReport` (`report.rs:96`) + `ReasonCode` (`report.rs:10`). Actor/debug summaries and `CheckedFact` are strings (`report.rs:80-112`); ticket 005 types `CheckedFact` and per-entry availability, but the post-rejection `ValidationReport` actor/debug split is this ticket.
2. Spec 0002 §4 TUI-AC-009 + §5 PSL-001: `ValidationReport` carries typed actor-visible facts and typed debug facts separately; the actor-visible render path can access only the safe subset; the debug render path requires the debug capability; actor-facing why-not states what the actor can tell and what is uncertain ("you do not know whether…") but never hidden contents/routes/culprit/exact source.
3. Cross-artifact boundary under audit: `ValidationReport` (produced by pipeline validation, ticket 006) ↔ `WhyNotView` (view contract) ↔ the debug why-not renderer (`crates/tracewake-tui/src/debug_panels.rs`, capability-gated by ticket 004).
4. Invariants restated: **INV-106** — validation failure feeds replanning/feedback without leakage; the actor learns only modeled consequences; **INV-107** — validator-only details remain debug-only unless a causal channel exposes them; **INV-070** — why-not in actor-known terms; **INV-093** — leakage is high-severity.
5. Fail-closed / no-leak surface: this is the why-not no-leak enforcement point. Confirm the actor-visible `WhyNotView` can be constructed without any debug capability and exposes only the typed safe-fact subset + actor-safe summary + source context id/hash; the debug fact set is reachable only through a capability-gated path (ticket 004). The split must be structural (separate typed collections), not a render-time filter.
6. Schema extension: `ValidationReport` splits its facts/summaries into a typed actor-visible set and a typed debug set (building on ticket 005's typed `CheckedFact`); `WhyNotView` exposes only the safe subset. Consumers: the pipeline producer (ticket 006), `WhyNotView` mapping (`view_models.rs:54-95`), `render_embodied_view` (`render.rs`), the debug renderer (`debug_panels.rs`), and tests. Breaking — internal, all consumers updated here.

## Architecture Check

1. Splitting `ValidationReport` into typed actor-visible vs typed debug fact sets makes non-leakage a property of which collection a renderer can reach (the actor path literally has no access to the debug set), rather than a substring scrub the renderer must remember to apply — eliminating the leak class INV-106/INV-107 target.
2. No backwards-compatibility aliasing/shims: the combined string summary is removed in favor of the two typed sets; the renderer formats typed facts and the debug set is not duplicated into the actor path.

## Verification Layers

1. INV-106 (non-leaking feedback) -> negative test over closed-container/hidden-route/hidden-food/wrong-suspicion/stale fixtures: actor-visible why-not names no hidden truth; the typed actor fact set excludes validator-only facts.
2. INV-107 (debug quarantine) -> codebase grep-proof: the debug fact set / debug why-not render requires the debug capability (ticket 004); the actor path does not.
3. INV-070 (actor-known why-not) -> schema validation: `WhyNotView` carries typed failure category, stable reason codes, actor-safe summary, source context id/hash.

## What to Change

### 1. Typed actor/debug fact split on `ValidationReport`

Separate the report's facts/summaries into a typed actor-visible set (safe facts, reason codes, actor-safe summary, source context id/hash) and a typed debug set (validation-only truth). Have ticket 006's validation populate both.

### 2. `WhyNotView` exposes only the safe subset

Map `WhyNotView` from the actor-visible set only; expose actor-safe uncertainty phrasing from typed fields.

### 3. Debug why-not behind capability

Render debug why-not (validation-only truth) in the non-diegetic debug panel, gated by the debug capability (ticket 004).

## Files to Touch

- `crates/tracewake-core/src/actions/report.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/src/debug_panels.rs` (modify)

## Out of Scope

- Per-entry `ActionAvailability` typing (ticket 005) — this ticket handles the post-rejection report.
- Proposal source-context validation that produces the report (ticket 006).
- Debug capability definition (ticket 004).

## Acceptance Criteria

### Tests That Must Pass

1. Closed-container, hidden-route, hidden-food, wrong-suspicion, and stale-proposal fixtures reject with actor-safe reason codes; the actor-visible why-not names no hidden contents/route/culprit/exact source; the validation-only truth appears only in the debug view.
2. A test proves the actor-visible `WhyNotView` constructs with no debug capability and the debug fact set/render requires one.
3. `cargo test -p tracewake-core && cargo test -p tracewake-tui` pass; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. The actor-visible why-not is a safe projection; validator-only truth is structurally unreachable from it (INV-106/INV-107).
2. Why-not is given in actor-known terms with typed reason codes (INV-070).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — actor-safe why-not non-leakage across the fixture set.
2. `crates/tracewake-tui/tests/tui_acceptance.rs` — actor vs debug why-not rendering; debug path capability-gated.

### Commands

1. `cargo test -p tracewake-core hidden_truth_gates && cargo test -p tracewake-tui tui_acceptance`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
3. The hidden-truth gate + tui-acceptance pair is the correct boundary: one proves the typed non-leak, the other proves the render split.
