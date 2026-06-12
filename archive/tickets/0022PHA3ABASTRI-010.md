# 0022PHA3ABASTRI-010: Census consumed-key call-shape fix and perception scan widening

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — test-oracle guards only (`anti_regression_guards.rs`)
**Deps**: 0022PHA3ABASTRI-004

## Problem

Two guard scan-closure residues. `ORD-HARD-109`: the consumed-payload-key derivation
(`payload_helper_calls`) finds callees only via the literal substring `(payload` —
calls shaped `helper(state, event, payload, …)` (comma-preceded) or `helper(&payload)`
are invisible, so keys consumed by such helpers are not attributed to the
`typed_columns` subset check; a future unlisted key consumed through an oblique call
shape slips silently. `ORD-HARD-110`: `perception_visibility_prose_branch_violations`
scans only `body_after_marker(…, "fn is_visible_exit_target")` — the other emission
constructors (`current_place_perception_events`, `observation_event`, the food/sleep
gating loops) are outside the window, so a future prose branch there would not fire
any guard despite `ORD-HARD-078`'s lock promising "no display_label/id-substring
branching in any emission path".

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `payload_helper_calls` scans for `"(payload"`
   (`crates/tracewake-core/tests/anti_regression_guards.rs`); the perception guard's
   window is `body_after_marker(&stripped, "fn is_visible_exit_target")`; the
   emission functions named above gate on typed state today (verified clean — both
   gaps are latent, not active).
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-109`/`110`
   (operator-verified) and the verified-holding record that the underlying censuses
   (`ORD-HARD-069`/`070` per-arm anchoring, inverted write-shape scan) and the typed
   concealment migration (`ORD-HARD-078`) are substantive — this ticket closes their
   scan-shape residue.
3. Shared contract under audit: the derived-census discipline — exemption
   `typed_columns` derived from *actually consumed* payload keys, and the perception
   module's prose-free emission contract.
4. Constitutional motivation restated: INV-020/INV-018 (the subset proof is the
   schema-gating durability mechanism) and INV-022 (raw prose is not authoritative
   state — the perception guard is its perception-channel lock).
5. Fail-closed surface: both changes widen detection only — the consumed-key resolver
   must not silently *drop* currently-attributed keys (run before/after attribution
   diff during implementation), and the widened perception scan allowlists only typed
   display-label payload *writes* (label text flowing into payloads is legitimate;
   label text *deciding* emission is not).
6. Change rationale (no silent retcon): both guards' current shapes are the
   0021-landed corrections' own R-28 residue, named by the spec; behavior of
   production code is unchanged (guards-only diff).

## Architecture Check

1. Resolving callees by parsing argument lists (split on `(`/`,`, strip `&`, match the
   payload identifier binding) replaces a substring heuristic with the actual call
   shape — the same fix shape the census's own `ORD-HARD-069` per-arm anchoring took.
   Widening the perception scan to the whole production module with a typed-write
   allowlist is simpler and more durable than enumerating emission functions (a new
   emission fn is covered by default).
2. No backwards-compatibility aliasing/shims; the old scan paths are replaced.

## Verification Layers

1. Consumed-key closure (`ORD-HARD-070` durability) -> guard check: a synthetic
   negative whose unlisted key is consumed by a helper called as
   `helper(state, &payload)` fails the census; existing attribution unchanged
   (before/after diff empty).
2. Perception prose-free emission (INV-022) -> guard check: a synthetic prose branch
   inside a different emission function (e.g. `current_place_perception_events`)
   fails the widened scan; the real module passes with the typed-write allowlist.
3. Census registration (§5.1) -> the `-004` bijection census: both widened guards and
   their new synthetics registered; nonzero-witness minimums set to current real-site
   counts.

## What to Change

### 1. Call-shape-resolved consumed-key derivation

Rewrite `payload_helper_calls` to tokenize call argument lists and recurse into any
callee receiving the payload identifier (with or without `&`, at any argument
position); add the oblique-call synthetic negative; assert the before/after
attribution diff is additive-only.

### 2. Widen the perception prose scan

Scope `perception_visibility_prose_branch_violations` to the whole
`agent/perception.rs` production source (cfg-test stripped), flagging
`display_label` / id-substring *branching* in any emission path, with an explicit
allowlist for typed display-label payload writes; add the other-function synthetic
negative.

### 3. Census registration

Register both guards + synthetics with the `-004` bijection census and set their
nonzero-witness minimums.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Any production change to `events/apply.rs` or `agent/perception.rs` (both verified
  clean today; this ticket hardens the scans that keep them so).
- The typed-concealment attribute itself (`ORD-HARD-078`, landed in 0021 — verified
  holding).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test anti_regression_guards` — both widened guards
   green on the real tree; the oblique-call synthetic and the other-function prose
   synthetic both fire.
2. The consumed-key attribution before/after diff is additive-only (recorded in
   implementation notes).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Exemption `typed_columns` subsets are proven against keys consumed through any
   call shape, not a substring-matched subset.
2. No emission path in `agent/perception.rs` may branch on display prose; the scan
   covers the whole module by default.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — call-shape resolver +
   oblique synthetic; module-wide perception scan + other-function synthetic; census
   entries.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

The consumed-payload-key derivation now scans call argument lists and recurses into
helpers that receive `payload` or `&payload` at any argument position. A synthetic
oblique helper call consuming `oblique_unlisted_key` proves the widened resolver
fails closed. The real attribution changed additively: `apply_need_delta` now lists
the helper-consumed `elapsed_ticks` key in its `needs_by_actor` exemption.

The perception prose-branch guard now scans the whole production perception module
instead of only `is_visible_exit_target`, while allowing typed label payload writes.
A synthetic branch in `current_place_perception_events` proves other emission paths
are covered. Both new synthetics are registered in the meta-lock census.

Deviation: no production source changes were required; this was a test-oracle guard
hardening only.

Verification:

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo build --workspace --all-targets --locked`
5. `cargo test --workspace`
