# 0022PHA3ABASTRI-004: Meta-lock tier — bijection census, nonzero-witness rule, two-sided ratchets

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — test-oracle meta-guards (`anti_regression_guards.rs`), generative corpus floors (`generative_lock.rs`), disposition-ledger change log
**Deps**: 0022PHA3ABASTRI-001

## Problem

Spec 0022's root pattern is R-29 recurring: 0021 structural locks landed in
artifact-asserting form (self-echo policy test, de-fanged gates, constant-accepting
sweep). The spec's §5.1 prescribes a generic meta-tier so future locks cannot be born
decorative: (1) a lock↔negative bijection census — every structural lock has a
registered synthetic negative routed through the same scan/assertion code path as the
real check; (2) a nonzero-witness rule — every census/scan lock asserts its iterator
matched a nonzero (or pinned-minimum) number of real sites; (3) two-sided ratchets —
the mutation baseline and generative corpus floors fail on increase AND on unrecorded
decrease. Spec §8 orders this early so subsequent tickets' new locks are born under
the meta-rules.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: the guard suite in
   `crates/tracewake-core/tests/anti_regression_guards.rs` carries synthetic negatives
   ad hoc (some routed through production scan paths, some artifact-shaped — the
   `physical_mutating_event_kinds_have_explicit_world_apply_arms` synthetic asserts
   substrings of a hardcoded literal, `ORD-HARD-113`); no bijection registry, no
   nonzero-witness convention, and the baseline/corpus floors are one-sided
   (`MUTANTS_BASELINE_NORMALIZED_COUNT` pins exact, but generative floors like the
   `>= 4` mask-diversity floor in `generative_lock.rs` only fail downward).
2. Verified against `specs/0022_…HARDENING_SPEC.md` §5.1 and §2's external research
   grounding (pseudo-tested methods, rotten green tests, vacuity witnesses, two-sided
   ratchet practice); §9's reflexivity requirement: the meta-locks must apply to
   themselves (the census lists itself with its own negative).
3. Shared contract under audit: the structural-lock inventory across
   `anti_regression_guards.rs`, `generative_lock.rs`, and `hidden_truth_gates.rs` —
   the census enumerates locks by a stable convention (e.g. `guard_*` /
   `*_violations` / registered lock IDs) so later tickets register new locks rather
   than re-deriving the registry.
4. Constitutional motivation restated: the Enforcement reading plus R-29 — "a lock is
   real only if its negative case can actually fire on the behavior"; the meta-tier is
   the standing countermeasure spec §6 adds to the risk register.
5. This ticket touches fail-closed test-oracle surfaces and the deterministic
   mutation-evidence pins: the two-sided ratchet must keep the recomputed pins exact
   and fail closed in both directions; no production, replay, or epistemic code path
   changes (zero-dependency, hand-rolled in the existing integration-test style per
   spec §8).
6. Adjacent contradiction classification: pre-existing artifact-shaped synthetics
   discovered by the census (e.g. the `ORD-HARD-113` world-applier synthetic, the
   decorative self-referential synthetic beside the `checked_facts` guard) are
   *required consequences*: they either get re-routed through production scan paths in
   their owning tickets (`-008`, `-009`) or registered as census-tracked debt with the
   owning ticket named — the census must not green-light them silently.
7. Change rationale (no silent retcon): floors and pins change encoding (one-sided →
   two-sided) without changing their current values; every value change thereafter
   requires a recorded delta.

## Architecture Check

1. A registry-driven census (lock ID → negative ID → shared scan entry point) is
   cleaner than per-lock discipline: it converts "every lock ships a firing negative"
   from a review convention into a failing test, and the nonzero-witness rule kills
   rotten-green scans mechanically. Two-sided ratchets make silent weakening AND
   silent improvement-evaporation impossible — both directions recorded.
2. No backwards-compatibility aliasing/shims; no new dependencies (hand-rolled,
   matching the existing guard style).

## Verification Layers

1. Bijection (R-29) -> codebase grep-proof: census fails on a lock without a
   registered negative; synthetic case (a lock registered with no negative) fires;
   reflexive entry — the census itself is listed with its own negative.
2. Shared-path routing -> guard check: a registered negative that does not invoke the
   same scan/assertion function as its real check fails the census; the inline-literal
   synthetic shape (`ORD-HARD-113`) is the canonical caught case.
3. Nonzero witness -> guard check: each census/scan lock asserts iterator hit-count
   ≥ pinned minimum; synthetic zero-match scan fires.
4. Two-sided ratchet -> schema/contract check: baseline count/hash and generative
   floors fail on unrecorded decrease as well as increase; a recorded-delta entry
   (ledger change-log line) is the only sanctioned change path; synthetic
   silent-shrink and silent-raise cases fire.

## What to Change

### 1. Lock↔negative bijection census

A registry in `anti_regression_guards.rs` mapping every structural lock (this file's
`guard_*`/`*_violations` checks, plus the generative and hidden-truth lock tests
enumerated by stable name) to its synthetic negative and the shared code path both
route through. Census fails on: a lock with no negative, a negative not routed through
the production scan function, or an unregistered lock matching the naming convention.
Known artifact-shaped synthetics are registered as named debt entries carrying their
owning ticket until re-routed. `0022PHA3ABASTRI-008` later re-routed the apply-arm
totality debt through the shared scan; `0022PHA3ABASTRI-009` remains ticket-owned
until its guard is re-routed.

### 2. Nonzero-witness rule

Extend the census so every scan-based lock declares its expected-minimum real-site
count (≥1 default); the census asserts each scan's actual hit count meets it. Apply to
the existing derived censuses (apply-arm, write-shape, checksum-coverage, dead-surface
sweep, perimeter checks).

### 3. Two-sided ratchets

(a) Mutation baseline: extend the `-001`-hardened governance so
`MUTANTS_BASELINE_NORMALIZED_COUNT`/hash changes require a recorded delta line in the
disposition ledger's change log (shrink via retirement, growth via new accepted
entries) — an unrecorded change in either direction fails. (b) Generative corpus
floors in `generative_lock.rs`: floors auto-tighten — the asserted floor must equal
the recorded observed value, failing on unrecorded decrease *or* unrecorded
improvement, with the recorded value updated through the same delta convention.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-core/tests/generative_lock.rs` (modify)
- `reports/0020_mutants_baseline_disposition.md` (modify — change-log section for
  recorded deltas; file reworked by 0022PHA3ABASTRI-001)

## Out of Scope

- Re-routing the known artifact-shaped synthetics themselves (`-008` owns the
  world-applier synthetic; `-009` owns the sweep predicates) — this ticket registers
  them as census debt.
- The per-finding locks of tickets `-005`–`-012` (they register with this tier when
  they land).
- Any change to what the baseline/floors currently measure — encoding only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test anti_regression_guards` — census green over the
   real lock inventory; synthetic negatives fire (lock-without-negative, non-routed
   negative, zero-match scan, unrecorded baseline shrink, unrecorded floor raise).
2. `cargo test -p tracewake-core --test generative_lock` — floors two-sided at current
   observed values.
3. The census output enumerates ≥ the current lock count (nonzero-witness applied to
   itself) and contains a reflexive entry for the census.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No structural lock exists without a registered, production-path-routed synthetic
   negative (or a named, ticket-owned debt entry).
2. Baseline and corpus-floor values change only through recorded deltas — both
   directions.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — bijection census,
   nonzero-witness assertions, baseline two-sided ratchet + synthetics.
2. `crates/tracewake-core/tests/generative_lock.rs` — two-sided floor encoding.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-core --test generative_lock`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed on 2026-06-12.

- Added a registry-backed meta-lock census in
  `crates/tracewake-core/tests/anti_regression_guards.rs` with stable lock IDs,
  registered synthetic negatives, routing class, witness minimums, and a reflexive
  `meta_lock_registry_census` entry.
- Registered known artifact-shaped negatives as ticket-owned debt for
  `0022PHA3ABASTRI-008` and `0022PHA3ABASTRI-009`, without re-routing them in this
  ticket. `0022PHA3ABASTRI-008` later re-routed its apply-arm totality debt through
  the shared scan; `0022PHA3ABASTRI-009` remains ticket-owned.
- Extended mutation-baseline governance so the current normalized count/hash
  (`143`, `bd1855a5ee82b428`) must be recorded in the disposition ledger's baseline
  change log; added synthetics for append, shrink, and unrecorded floor movement.
- Converted generative corpus diversity and contributor floors to exact recorded
  values in `crates/tracewake-core/tests/generative_lock.rs`, with a source-level
  guard that rejects one-sided floor assertions and unrecorded floor raises.

Verification run:

1. `cargo test -p tracewake-core --test generative_lock`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo fmt --all`
4. `cargo test -p tracewake-core --test anti_regression_guards`
5. `cargo test -p tracewake-core --test generative_lock`
6. `cargo fmt --all --check`
7. `cargo clippy --workspace --all-targets -- -D warnings`
8. `cargo build --workspace --all-targets --locked`
9. `cargo test --workspace`
