# 0023PHA3AEMBLOC-011: Canonical-day intent coherence and embodied sleep positive proof

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — canonical-fixture tests (`golden_fixtures_run.rs`), TUI acceptance tests
**Deps**: archive/tickets/0023PHA3AEMBLOC-004.md, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

Two low-severity evidence gaps (spec 0023 `ORD-HARD-137`, `ORD-HARD-138`):

- The fixture `no_human_day_001` records
  `canonical_mara_recovery_resolution=fail_only_empty_food_source`, but no test
  references the string — a future edit flipping Mara's runner outcome would leave
  the recorded intent contradicting runtime behavior with nothing firing. The
  `ORD-HARD-120` "record the fail-only intent" branch landed as dead documentation.
- Sleep has no positive embodied reachability proof: `tui_acceptance.rs` proves
  eat/work/continue-routine/wait embodied-reachable and submittable, while the sole
  TUI `"Sleep here"` assertion is the negation (`embodied_flow.rs` omission case) —
  leaving the 0005 five-action TUI sweep (INV-066/071) four-fifths positive.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: the intent string exists only in
   `crates/tracewake-content/src/fixtures/no_human_day_001.rs` (repo-wide grep — no
   test reference); the only TUI `"Sleep here"` assertion is
   `assert!(!rendered.contains("Sleep here"))` in
   `crates/tracewake-tui/tests/embodied_flow.rs`; the runner-only ancestry
   assertions in `no_human_day_fixture_has_roster_activity_and_metrics_envelope`
   (`crates/tracewake-content/tests/golden_fixtures_run.rs`) are 0022-verified
   holding and are the natural host for the coherence assertion.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-137`/`138`
   (operator-verified at reassessment / at audit). The sleep *pipeline* is proven in
   core (runner `SleepCompleted`); the gap is purely the embodied-menu positive
   surface.
3. Shared contract under audit: the canonical fixture's contract strings ↔ runner
   behavior coherence, and the 0005 five-ordinary-actions TUI reachability sweep.
   Ordered after -004 because the migration reprices canonical-day expectations and
   churns embodied-render tests — landing these assertions after avoids double
   rework.
4. Constitutional motivation restated: INV-066/071 (every runnable phase's mechanics
   reachable and regression-tested through the TUI — a mechanic proven only by
   omission is not positively reachable) and INV-070 (why-not parity for the blocked
   case already exists; this adds the unblocked positive); R-27 for the intent
   string (recorded evidence must be bound to the behavior it describes).
5. This ticket touches TUI acceptance and canonical-fixture evidence surfaces: both
   additions are new assertions over existing behavior — no production change, no
   epistemic or replay surface weakened. The sleep-affordance fixture must seed the
   actor-known sleep record through event provenance (the post-`ORD-HARD-120`
   allowlist posture: no blanket seeding helpers in new fixtures).
6. Change rationale (no silent retcon): the coherence assertion binds an existing
   recorded intent to behavior; the sleep positive completes an existing acceptance
   sweep. Mandated by `ORD-HARD-137`/`138` required corrections.

## Architecture Check

1. Keying the coherence assertion on the recorded resolution token (assert the
   runner-only Mara outcome — `EatFailed` for `actor_mara`, no autonomous
   `FoodConsumed`, no autonomous move to `food_stew_home_tomas` — matches
   `fail_only_empty_food_source`) makes the documented intent and runtime behavior
   un-divergeable, which is cheaper and stronger than promoting the intent to a
   typed fixture field this phase. The sleep positive extends the existing
   per-action embodied reachability coverage pattern rather than inventing a new
   harness.
2. No backwards-compatibility aliasing/shims: no duplicate intent records; the new
   sleep fixture seeds per-actor knowledge through events (no legacy blanket
   helper).

## Verification Layers

1. Contract-vs-behavior coherence (R-27) -> replay/golden-fixture check: the
   coherence assertion fails if Mara's runner-only outcome diverges from the
   recorded `fail_only_empty_food_source` token (flip-the-fixture synthetic proves
   it fires).
2. INV-066/071 positive reachability -> codebase test-proof: an embodied view over
   an actor-known sleep affordance exposes a submittable `sleep` semantic action
   ("Sleep here" present, submit accepted into the shared pipeline).
3. Five-action sweep closure -> manual review: eat/work/continue-routine/wait/sleep
   each carry a positive embodied proof after this ticket.

## What to Change

### 1. Canonical-intent coherence assertion (`ORD-HARD-137`)

In `no_human_day_fixture_has_roster_activity_and_metrics_envelope` (before any
manual injection, alongside the runner-only ancestry assertions): read the recorded
`canonical_mara_recovery_resolution` token and assert the runner-only Mara outcome
matches it (`EatFailed` present for `actor_mara`; no autonomous `FoodConsumed`; no
autonomous relocation toward `food_stew_home_tomas`).

### 2. Embodied sleep positive (`ORD-HARD-138`)

Add a TUI acceptance case: a fixture whose viewer has an actor-known sleep
affordance (event-seeded, per-actor) renders the sleep semantic action embodied and
submits it through the shared pipeline; extend the per-action reachability coverage
so each ordinary action carries a positive entry.

## Files to Touch

- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify)

## Out of Scope

- Changing Mara's canonical outcome (fail-only intent stands; the assertion binds
  it).
- The runner-only ancestry assertions (0022-landed; verified holding).
- Promoting fixture contract strings to a typed schema (future cleanup if the
  pattern recurs — classified as such, not owed here).

## Acceptance Criteria

### Tests That Must Pass

1. The coherence assertion passes on the current fixture and fails when the recorded
   token and runner outcome diverge (synthetic flip)
   (`cargo test -p tracewake-content --test golden_fixtures_run`).
2. The embodied sleep positive renders and submits `sleep`
   (`cargo test -p tracewake-tui --test tui_acceptance`); the existing omission
   negative in `embodied_flow.rs` stays green.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. Recorded canonical-fixture intent strings are behavior-bound — a contract string
   no test reads cannot recur on this surface.
2. All five 0005 ordinary actions carry positive embodied reachability proofs.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — intent-coherence
   assertion keyed on the recorded resolution token.
2. `crates/tracewake-tui/tests/tui_acceptance.rs` — embodied sleep
   reachability+submit positive over an event-seeded sleep affordance.

### Commands

1. `cargo test -p tracewake-content --test golden_fixtures_run && cargo test -p tracewake-tui --test tui_acceptance`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
