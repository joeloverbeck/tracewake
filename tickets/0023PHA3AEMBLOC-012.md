# 0023PHA3AEMBLOC-012: Generative envelope-ban widening and exhaustive cause disposition

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — generative-tier guard (`anti_regression_guards.rs`), `EventKind::cause_required` (`events/envelope.rs`)
**Deps**: 0023PHA3AEMBLOC-001, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

Two low-severity lock-shape residues (spec 0023 `ORD-HARD-136`, `ORD-HARD-139`):

- The generative fabricator scan
  (`generative_duration_terminal_fabricator_errors` in `anti_regression_guards.rs`)
  gates on the `EventEnvelope::new` substring only — `EventEnvelope::default()`,
  struct literals, clones, or aliased re-exports evade. `support/generative.rs`
  verified to contain zero `EventEnvelope` mentions today, so the invariant holds
  but the lock is narrower than its claim.
- `EventKind::cause_required` (`events/envelope.rs`) is a literal `matches!` list —
  any unlisted variant defaults to `false` (no cause required) with no compiler
  pressure; the derivation the 0022 spec promised lives in the guard, not the
  kernel, so a kind born outside the scanned emission paths gets no disposition
  check.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: the `context.contains("EventEnvelope::new")` gate in
   `generative_duration_terminal_fabricator_errors`; zero `EventEnvelope` tokens in
   `crates/tracewake-core/tests/support/generative.rs`; the `matches!`-list
   `cause_required` (`const fn`, `crates/tracewake-core/src/events/envelope.rs`)
   consumed via `requires_cause` → `metadata().cause_required`; the guard-side
   derivation (`action_emitted_kind_cause_disposition_violations`) iterates
   `EventKind::all()` and stays as the behavioral cross-check.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-136`/`139`
   (operator-verified at reassessment / at audit). Note: -004 adds new observation
   event kinds — landing this ticket's exhaustive `match` after (or alongside) -004
   forces an explicit cause disposition for each new kind at the compiler level;
   the build order is handled by each ticket keeping the workspace green
   independently (an exhaustive match added first simply forces -004 to declare
   dispositions, which it must anyway).
3. Shared contract under audit: the generative-tier fabrication ban (no test-support
   construction of engine event envelopes) and the event-kind cause-disposition
   contract consumed by replay validation (`requires_cause`).
4. Constitutional motivation restated: INV-010 (every event needs a cause model — a
   new kind silently defaulting to no-cause-required is the trap) and R-27/R-29 for
   the fabricator ban (evidence produced by the path under test; a ban narrower than
   its stated claim is decorative at the margin).
5. This ticket touches deterministic-replay enforcement surfaces (cause-disposition
   feeding replay validation; the fabricator ban protecting generative evidence
   provenance): the exhaustive `match` and the widened token ban both tighten;
   nothing is weakened. (Compile-time exhaustiveness is the lineage's strongest lock
   tier.)
6. Schema note: no schema shape change — `cause_required` keeps its signature; only
   its body becomes exhaustive (no `_` arm). N/A for additive-vs-breaking beyond
   this: every existing variant keeps its current disposition, asserted by the
   existing guard staying green.
7. Change rationale (no silent retcon): the `matches!` list becomes an exhaustive
   `match` because the default-`false` fallback provably lets a new kind skip its
   disposition; the ban widens because the constructor-substring shape provably
   misses sibling construction forms. Mandated by `ORD-HARD-136`/`139`.

## Architecture Check

1. An exhaustive `match` (no `_` arm) moves the cause-disposition decision to the
   compiler — every future `EventKind` variant fails to build until an explicit
   disposition is written — which is strictly stronger than the guard-side scan and
   keeps the scan as a behavioral cross-check rather than the sole defense. Banning
   the bare `EventEnvelope` token in `support/generative.rs` (allowlisted imports
   excepted) closes the whole construction-form family instead of enumerating
   constructors.
2. No backwards-compatibility aliasing/shims: the `matches!` body is replaced, not
   wrapped; no variant's disposition changes silently (the existing guard pins
   current behavior).

## Verification Layers

1. INV-010 disposition-at-birth -> compile-time proof: `cause_required` is an
   exhaustive `match`; a synthetic new variant without an arm fails compilation
   (documented as the lock; the guard's runtime negative remains as the test-layer
   proof).
2. Fabricator-ban closure (R-29) -> codebase test-proof: negatives constructing via
   `EventEnvelope::default()` and a struct literal in a synthetic
   support/generative body fail the widened ban — routed through the production
   scan, enrolled under -001's registry.
3. Current-disposition pinning -> codebase test-proof:
   `action_emitted_kind_cause_disposition_violations` stays green (no variant's
   disposition drifted during the rewrite).

## What to Change

### 1. Widened envelope ban (`ORD-HARD-136`)

In `generative_duration_terminal_fabricator_errors`: ban the bare `EventEnvelope`
token in `support/generative.rs` (allowlisting explicit imports if any are ever
needed), replacing the `EventEnvelope::new` substring gate. Add `default()` and
struct-literal negatives.

### 2. Exhaustive cause disposition (`ORD-HARD-139`)

Rewrite `EventKind::cause_required` as an exhaustive `match` with no `_` arm, every
variant carrying its current disposition explicitly. Keep
`action_emitted_kind_cause_disposition_violations` as the behavioral cross-check.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-core/src/events/envelope.rs` (modify)

## Out of Scope

- New event kinds themselves (-004 owns the locality observation kinds and their
  dispositions).
- The metadata totality guards (landed in 0022; verified holding).
- Generative displacement-ordering bounds (landed in 0022; verified holding).

## Acceptance Criteria

### Tests That Must Pass

1. The `default()` and struct-literal fabricator negatives fail the widened ban;
   the real `support/generative.rs` passes
   (`cargo test -p tracewake-core --test anti_regression_guards`).
2. `cause_required` compiles as an exhaustive `match`; every existing variant's
   disposition is unchanged (`action_emitted_kind_cause_disposition_violations`
   green).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. A new `EventKind` variant cannot compile without an explicit cause disposition.
2. Generative support code cannot construct an `EventEnvelope` through any
   construction form without tripping the ban.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — widened token ban +
   two construction-form negatives; registry enrollment.
2. `crates/tracewake-core/src/events/envelope.rs` — exhaustive `match` rewrite
   (compile-time lock; existing guard pins behavior).

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
