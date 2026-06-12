# 0024PHA3ACONSCH-012: Oracle closures — Mara log-discrimination, per-action positive census, envelope-ban widening

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` tests (`golden_fixtures_run.rs`), `tracewake-tui` tests (`tui_acceptance.rs`), `tracewake-core` test guards (`anti_regression_guards.rs`).
**Deps**: 0024PHA3ACONSCH-001

## Problem

Spec 0024 findings `ORD-HARD-158`, `159`, `160` (all low):

- `158`: the canonical-day resolution negative (`flipped_resolution`) flips the
  recorded token to an unsupported value, exercising only the unknown-token arm —
  no negative proves an injected autonomous `FoodConsumed` for Mara fails the
  `fail_only_empty_food_source` arm (token recognition, not log discrimination).
- `159`: the embodied sleep positive landed as an instance
  (`accepted_sleep_started_event`); no census ties the ordinary-action registry to
  required submitted-positives, so a new ordinary action can ship with only
  negatives.
- `160`: the support `EventEnvelope` ban is a total token ban scoped to the single
  path `support/generative.rs`; a cross-file type alias
  (`pub type Env = EventEnvelope;` in `tests/support/mod.rs`, unscanned) consumed in
  `generative.rs` leaves no banned token in the scanned file.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: `mara_recovery_resolution_errors`'s
   `fail_only_empty_food_source` arm and the token-flip-only synthetic
   (`golden_fixtures_run.rs`); the lone `"Sleep here"` positive and the zero
   per-action census matches in `tui_acceptance.rs`; the
   `path == "support/generative.rs" && source.contains("EventEnvelope")` ban and the
   existence of `tests/support/mod.rs` — all operator-verified (the `160` evidence
   was corrected at spec reassessment from a same-file to a cross-file alias
   evasion; this ticket implements the corrected shape).
2. Verified against spec 0024 §4 (`ORD-HARD-158`/`159`/`160`, post-reassessment
   text) and the precedents `ORD-HARD-137`/`138`/`136` in archived 0023.
3. Cross-artifact boundary: the canonical-day intent contract
   (`canonical_mara_recovery_resolution` token ↔ event-log behavior); the
   ordinary-action registry ↔ TUI acceptance-coverage contract; the
   support-harness envelope-construction ban across all `tests/support/*` sources.
4. Rule restated (R-29/R-28): a negative that exercises the shape the check already
   catches proves nothing about the discriminating arm; an instance landed without
   its family census re-opens per action; a single-file token ban is a
   type-convention perimeter.
5. Enforcement surface (test oracle / canonical-day evidence): all three corrections
   are test-side; no production behavior, fixture content, or replay surface
   changes. The per-action census derives membership from the action registry (the
   five 0005 ordinary actions today: `sleep`, `eat`, `work_block`,
   `continue_routine`, `wait`) so a sixth action auto-extends the requirement.

## Architecture Check

1. A tampered-log synthetic exercises the discrimination arm the contract exists
   for — the cheapest true-negative for the canonical-day intent. Registry-derived
   positive coverage (with rationale-bearing exemptions) closes the action family
   structurally, mirroring the lineage's census pattern. Widening the envelope ban
   to every `tests/support/*` source closes the cross-file alias hole at the file
   perimeter rather than attempting Rust alias resolution in a string scan.
2. No backwards-compatibility aliasing/shims: no exemption for the current support
   files; the single-path ban is widened, not duplicated.

## Verification Layers

1. Canonical-intent discrimination (`158`) →
   `synthetic_mara_autonomous_food_consumed_fails_resolution`: an injected no-human
   `FoodConsumed` for `actor_mara` in a tampered log clone fails the
   `fail_only_empty_food_source` arm (test).
2. Per-action positive census (`159`) → registry-derived guard: each ordinary
   semantic action has a submitted-positive in the acceptance suite; removing the
   sleep positive fails the census (synthetic) (test).
3. Envelope-ban perimeter (`160`) → a synthetic alias defined in `mod.rs` and
   consumed in `generative.rs` fails the widened ban; the existing `default()` and
   struct-literal synthetics still fire (tests).
4. Whole-pipeline → full workspace gates.

## What to Change

### 1. Mara log-discrimination negative (`158`)

Add the tampered-log synthetic injecting an autonomous `FoodConsumed` for Mara and
asserting `mara_recovery_resolution_errors("fail_only_empty_food_source", …)` yields
the consumed-food violation.

### 2. Per-action positive census (`159`)

A guard enumerating ordinary semantic action ids from the registry (5 today,
enumerated above) and asserting each has a submitted embodied positive in
`tui_acceptance.rs`, with rationale-bearing exemptions; sleep-positive-removal
synthetic.

### 3. Envelope-ban widening (`160`)

Extend the token ban to every `tests/support/*` source (`mod.rs` included);
mod.rs-alias synthetic.

## Files to Touch

- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Changing the canonical fixture's recorded resolution (the fail-only intent stands).
- Adding new ordinary actions or changing action semantics.
- The witness-rule mechanics (ticket -001, dependency — the widened ban and census
  enroll under it).

## Acceptance Criteria

### Tests That Must Pass

1. The tampered-log synthetic fails the resolution check with the consumed-food
   violation; the existing token-flip synthetic still fires.
2. The per-action census passes against the current suite (all 5 enumerated actions
   positive — sleep, eat, work_block, continue_routine, wait) and fails when the
   sleep positive is removed (synthetic).
3. The mod.rs-alias synthetic fails the widened ban; current support sources pass
   (they construct zero envelopes).
4. The four workspace gates pass.

### Invariants

1. The canonical-day recorded intent is enforced against log behavior, not only
   token vocabulary.
2. Ordinary-action TUI positive coverage is registry-derived, never a hand list.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` —
   `synthetic_mara_autonomous_food_consumed_fails_resolution`.
2. `crates/tracewake-tui/tests/tui_acceptance.rs` — per-action positive census +
   removal synthetic.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — widened envelope ban +
   mod.rs-alias synthetic.

### Commands

1. `cargo test -p tracewake-content --test golden_fixtures_run && cargo test -p tracewake-tui --test tui_acceptance && cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
