# 0025PHA3AMETWIT-007: Core/TUI census-oracle closures — effect-complete positive census, truth-table policy oracle, support-directory parity

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — test guards only (`tracewake-tui` acceptance suite, `tracewake-core` epistemics test oracle and anti-regression guards); no production crate code.
**Deps**: 0025PHA3AMETWIT-001

## Problem

Spec 0025 findings `ORD-HARD-177` (low), `ORD-HARD-178` (low), `ORD-HARD-179`
(low). The per-ordinary-action positive census
(`tui_acceptance.rs::ordinary_action_positive_census_errors`) filters
`registry.definitions()` through a hand-picked five-variant
`matches!(definition.effect, Wait | Sleep | Eat | Work | ContinueRoutine)` —
`ActionEffect` has more variants, and a future ordinary action with a new effect
is silently excluded, shipping positive-free (the exact defect the census exists
to prevent). The policy generic-loop oracle
(`epistemics/projection.rs::expected_embodied_presence`) re-derives its expected
flag by calling `includes_in_embodied_context` — the method under test — so a
defect inside the predicate shifts both sides identically (the dedicated
stale-path negative independently covers the row-mutation axis only). And the
`tests/support/*` `EventEnvelope` ban scans a hardcoded two-file list
(`SUPPORT_GENERATIVE_RS`/`SUPPORT_MOD_RS` `include_str!`s); a third support file
would be unscanned.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `9e33d7a`: the `matches!` five-variant filter
   (`crates/tracewake-tui/tests/tui_acceptance.rs`);
   `expected_embodied_presence` calling `includes_in_embodied_context` on the
   classified output (`crates/tracewake-core/src/epistemics/projection.rs`); the
   two-file `include_str!` list and `tests/support/` containing exactly
   `generative.rs` + `mod.rs`. All operator-verified per spec 0025 §8 (178 at
   reassessment).
2. Verified against spec 0025 §4 and `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
   R-28 (hand-lists beside derivable populations) / R-29 (oracle re-derivation);
   `enum ActionEffect` confirmed at `crates/tracewake-core/src/actions/registry.rs`.
3. Shared boundary under audit: census-membership derivation — each census's
   member set must derive from the authoritative population (the action registry's
   effect enumeration; the support directory listing) rather than a hand list.
4. Rule restated before trusting the narrative (R-28): derive membership instead
   of trusting review memory — a census scoped to a hand-picked subset covers
   nothing outside the subset, silently.
5. Enforcement surfaces touched: the positive-coverage census and the policy
   oracle are test-oracle surfaces over TUI acceptance and the actor-known
   projection policy; all changes strengthen discrimination and relax no
   validation, leak nothing, and alter no replay semantics. **Resolved either-or
   (announced at Step 4 under the carve-out)**: `ORD-HARD-177` is closed by
   enumerating *all* `ActionEffect` variants with rationale-bearing exemptions for
   the non-ordinary ones — not by adding a registry-level ordinary-life flag —
   keeping the closure test-only with no production schema growth; a new variant
   then forces an explicit census decision (enroll or exempt).

## Architecture Check

1. Enumerating the full effect set with explicit exemptions (and deriving the
   support-file list from a directory walk) turns each hand-list into a parity
   census that fails on unconsidered growth — the same derivation discipline as
   `apply_mutator_tokens_from` and the `ORD-HARD-123` census-closure precedent.
   The truth-table oracle (literal expected values over `embodied_scope` ×
   classifier flags) makes the policy test independent of the predicate under
   test, per the R-29 both-sides-read-one-source rule.
2. No backwards-compatibility aliasing/shims: the hand lists are replaced by
   derivations; no legacy filter path remains.

## Verification Layers

1. Effect-complete census (`ORD-HARD-177`) → a synthetic ordinary action with an
   unlisted effect and no positive must fail the census; removing the sleep
   positive still fails it (existing negative preserved).
2. Truth-table oracle (`ORD-HARD-178`) → a synthetic inversion of
   `includes_in_embodied_context` must fail the generic loop, not only the
   dedicated stale path.
3. Support-directory parity (`ORD-HARD-179`) → the census fails when
   `tests/support/` gains a file absent from the scan list.
4. Whole-suite regression → `cargo test -p tracewake-tui`,
   `cargo test -p tracewake-core --test anti_regression_guards`, four-gate run;
   the repaired censuses enroll under the 0025PHA3AMETWIT-001 witness routing
   where registry-reachable.

## What to Change

### 1. Effect-complete positive census (`ORD-HARD-177`)

`ordinary_action_positive_census_errors` enumerates every `ActionEffect` variant:
ordinary ones demand a submitted positive; non-ordinary ones carry
rationale-bearing exemptions; an unhandled (new) variant fails the census.

### 2. Truth-table policy oracle (`ORD-HARD-178`)

`expected_embodied_presence`'s generic loop derives the expected flag from a
literal truth table over `embodied_scope` × the classifier flags, independent of
`includes_in_embodied_context`; add the predicate-inversion discriminating
negative.

### 3. Support-directory parity (`ORD-HARD-179`)

The `EventEnvelope` ban's source list is asserted equal to the `tests/support/`
directory contents (parity census), so new support files auto-enroll or fail.

## Files to Touch

- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify — test-oracle code)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Adding any production `ActionScope`/ordinary-life flag to the action registry
  (rejected branch of the resolved either-or; Assumption item 5).
- Content-crate censuses (`ORD-HARD-181`/`182`) — ticket `0025PHA3AMETWIT-008`.
- New ordinary actions or policy rows (census/oracle closure only).

## Acceptance Criteria

### Tests That Must Pass

1. The effect-complete census enumerates all `ActionEffect` variants (count
   asserted from the enum at test time, not hardcoded); the unlisted-effect
   synthetic and the sleep-positive-removal negative both fail it.
2. The predicate-inversion synthetic fails the generic policy loop; the
   support-directory parity census fails on a synthetic third support file.
3. `cargo test -p tracewake-tui`, `cargo test -p tracewake-core --test
   anti_regression_guards`, and the four gates (`cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. No census in these suites filters its population through a hand-picked subset;
   membership derives from the authoritative enumeration with rationale-bearing
   exemptions.
2. No oracle in these suites re-derives its expected value by calling the
   predicate under test.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/tui_acceptance.rs` — effect-complete census +
   unlisted-effect synthetic.
2. `crates/tracewake-core/src/epistemics/projection.rs` — truth-table oracle +
   inversion negative.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — support-directory
   parity census + synthetic.

### Commands

1. `cargo test -p tracewake-tui && cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-13

### Files Changed

- `crates/tracewake-tui/tests/tui_acceptance.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/tests/anti_regression_guards.rs`

### Decisions

- Replaced the TUI positive census hand-picked effect filter with an
  `ActionEffect` variant-derived census and an explicit per-effect decision
  table.
- Added a real accepted `close` positive; kept `Place` as a
  rationale-bearing exemption because current TUI fixtures surface it only as a
  disabled negative until an authored enabled target exists.
- Replaced the projection generic expected-value oracle with a truth table over
  `ActorKnownProjectionEmbodiedScope` and classified-record flags, independent
  of `includes_in_embodied_context`.
- Added support-directory parity for the EventEnvelope support-file scan list.

### Acceptance Disposition

- Effect-complete census: completed; variants are read from the enum source and
  every variant requires either a positive or an exemption rationale.
- Sleep-positive-removal negative: completed.
- Unhandled-effect synthetic: completed with a synthetic
  `SyntheticNewEffect` row.
- Predicate-inversion synthetic: completed in the projection policy tests.
- Support-directory parity synthetic: completed with a synthetic third support
  file.
- Deferred or dropped work: no production registry flag or new fixture was added.

### Verification

- `cargo test -p tracewake-tui`
- `cargo test -p tracewake-core --lib epistemics::projection::tests`
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
