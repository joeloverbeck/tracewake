# 0046TUISIMPLA-002: Hop-2 exhaustive no-wildcard matches at closed-enum presentation owners

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-tui` renderer (`crates/tracewake-tui/src/render.rs`) and seam-conformance guard (`crates/tracewake-tui/tests/tui_seam_conformance.rs`); a closed-enum actor-safe summary may be added in `tracewake-core` only if the presentation owner is core (see What to Change §3).
**Deps**: None

## Problem

Spec 0046 §4.1 `PAR-003`. The renderer already matches `VisibleItemSource`
(`crates/tracewake-tui/src/render.rs:186`, `visible_item_source_label`) exhaustively with no `_` arm —
the pattern this ticket generalizes. The other closed presentation enums consumed at the actor-facing
or debug-facing rendering boundary do not all carry the same guarantee: a new variant on
`ActionAvailability` (`view_models.rs:260`), `ActionAvailabilityProvenanceKind` (`view_models.rs:345`),
`WhyNotFailureKind` (`view_models.rs:68`), or `DebugViewModel` (`view_models.rs:364`) can be added and
fall through a wildcard arm, rendering as a default string with no conscious renderer decision. The
spec requires every closed enum that *selects actor-facing or debug-facing presentation* to be matched
exhaustively with no `_` at its semantic presentation owner, so a new variant forces a compile break.

## Assumption Reassessment (2026-06-22)

1. Verified against code at baseline `1145e109`: `visible_item_source_label` (`render.rs:186`) matches
   `Place`/`Container`/`Carried` exhaustively, no wildcard. Render-side enum consumers present:
   `why_not.failure_kind.stable_id()` (`render.rs:29`), reason `.stable_id()` maps (`render.rs:136`,
   `:282`), `ActionAvailability` (used in render tests + the affordance render path),
   `render_debug_overlay` (`render.rs:153`) consuming `DebugViewModel`. The closed enums live in
   `crates/tracewake-core/src/view_models.rs`: `WhyNotFailureKind:68`, `VisibleItemSource:190`,
   `ActionAvailability:260`, `ActionAvailabilityProvenanceKind:345`, `DebugViewModel:364`.
2. Verified against spec 0046 §4.1 (`PAR-003`) and the §3.1/§4.1 clarification applied during
   reassessment: `ActionEffect` is **not** a `view_models.rs` presentation enum — it lives in
   `crates/tracewake-core/src/actions/registry.rs:6` as an action-pipeline effect classifier
   (`QueryOnly`, `Move`, …, `ContinueRoutine`) with **no** render-boundary presentation owner today
   (`grep ActionEffect` over `render.rs`/`view_models.rs` = 0). Its inclusion in `PAR-003` is
   **forward-conditional**: the obligation binds only if/when a presentation owner for it is
   introduced, and no actor-facing presentation may be invented merely to satisfy this guard (§8).
3. Shared boundary under audit: the closed-enum ↔ presentation-owner contract. The rule binds at the
   **semantic presentation owner**: if `tracewake-core` owns an actor-safe textual summary for an enum,
   core's match must be exhaustive and the TUI calls it; the TUI must not duplicate simulation rules to
   obtain an exhaustive match (`INV-069`). `SemanticActionEntry` action-ID completeness is out of scope
   here — it is `PAR-007`/`PAR-009` (tickets 003/006).
4. Invariant restated (`PAR-003` motivation): `INV-067` embodied mode shows actor-known reality;
   `INV-068` debug mode is visibly non-diegetic; `INV-069` the TUI must not implement simulation rules;
   `INV-070` why-not explanations are mandatory in actor-known terms. Exhaustive matches enforce
   conscious presentation of every variant without adding world logic.
5. Enforcement surface touched: the render boundary as the actor-vs-debug presentation firewall. The
   `DebugViewModel` match stays inside `render_debug_overlay` (non-diegetic, `INV-068`/`INV-107`); the
   actor-facing enum matches stay inside the embodied render path. This ticket adds no hidden-truth read
   and routes no debug-only variant into embodied output; it only removes wildcard fall-through. The
   conditional `view_models.rs` `(modify)` introduces **no schema shape change** — no enum variant or
   struct field is added or altered; it adds an exhaustive match (and, only if core is the owner, a
   summary fn), so the additive-vs-breaking analysis is N/A.

## Architecture Check

1. Exhaustive `match` with no `_` at each presentation owner is cleaner than a runtime "unknown variant"
   default: the compiler forces a decision at the exact site a variant is added, matching the existing
   `visible_item_source_label` discipline rather than inventing a parallel mechanism. Where core owns the
   actor-safe summary, the match lives in core and the TUI delegates — keeping simulation semantics out
   of the TUI (`INV-069`).
2. No backwards-compatibility aliasing/shims: wildcard arms are removed, not retained as a fallback; no
   "default presentation" helper is kept alongside the exhaustive matches.

## Verification Layers

1. `PAR-003`/`INV-067` (actor-facing exhaustiveness) → compiler proof: a temporary variant on
   `ActionAvailability`/`WhyNotFailureKind`/`ActionAvailabilityProvenanceKind` fails to compile at its
   presentation owner until handled (exercised as the controlled compile-break in `0046TUISIMPLA-012`).
2. `INV-068`/`INV-107` (debug quarantine) → manual review + `adversarial_gates.rs` regression: the
   `DebugViewModel` exhaustive match stays in `render_debug_overlay`; no debug variant reaches embodied
   output.
3. `PAR-003` durability → source-conformance test in `tui_seam_conformance.rs`: presentation matches
   over the named enums contain no `_` arm at the protected sites.

## What to Change

### 1. Exhaustive actor-facing enum matches

At the embodied presentation owners for `ActionAvailability`, `ActionAvailabilityProvenanceKind`,
`WhyNotFailureKind`, remove any `_` arm and match every variant explicitly. Preserve the existing
`VisibleItemSource` handling unchanged.

### 2. Exhaustive debug-facing match

In `render_debug_overlay`, match `DebugViewModel` exhaustively with no `_`.

### 3. Semantic-owner placement

For any enum whose actor-safe textual summary is owned by `tracewake-core`, make core's `match`
exhaustive and have the TUI call it; do not duplicate the mapping in the TUI. Add a core summary fn only
where one is the correct owner — do **not** add presentation for `ActionEffect` (no owner today; §
Assumption 2).

### 4. Source-conformance expectations

Extend `crates/tracewake-tui/tests/tui_seam_conformance.rs` with `include_str!`-based checks that the
presentation matches over the named enums contain no bare `_` arm at the protected presentation sites.

## Files to Touch

- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/tests/tui_seam_conformance.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify — only if a closed-enum actor-safe summary's owner is core and one must be added/made exhaustive)

## Out of Scope

- The `EmbodiedViewModel` struct destructure + its guard (`PAR-001`/`PAR-002`) — ticket
  `0046TUISIMPLA-001` (shares `render.rs` + `tui_seam_conformance.rs`; coordinate the merge).
- `SemanticActionEntry` action-ID completeness — tickets `0046TUISIMPLA-003`/`006`.
- Adding any presentation for `ActionEffect` (forward-conditional; no owner exists — Assumption 2).
- Any change to enum variant sets or to action-pipeline semantics.

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: the presentation owners for `ActionAvailability`, `ActionAvailabilityProvenanceKind`,
   `WhyNotFailureKind`, and `DebugViewModel` contain no `_` arm; `visible_item_source_label` is
   unchanged. No presentation for `ActionEffect` is introduced.
2. The `tui_seam_conformance.rs` exhaustiveness expectations pass and fail when a `_` arm is reinstated
   at a protected site (demonstrated via scratch edit during implementation).
3. `cargo test -p tracewake-tui` and the four gates pass; embodied + debug render output unchanged for
   existing variants (`tui_acceptance.rs`, `adversarial_gates.rs` remain green).

### Invariants

1. No closed presentation enum variant can be added without a conscious match arm at its semantic
   presentation owner (compiler-enforced, no `_`).
2. Debug-only variants (`DebugViewModel`) are presented only in the debug overlay, never embodied output.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/tui_seam_conformance.rs` — exhaustiveness source guards for the named
   presentation enums.
2. `crates/tracewake-tui/src/render.rs` — exhaustive matches are themselves the compile-time guard;
   existing render unit tests confirm unchanged per-variant output.

### Commands

1. `cargo test -p tracewake-tui --test tui_seam_conformance`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
