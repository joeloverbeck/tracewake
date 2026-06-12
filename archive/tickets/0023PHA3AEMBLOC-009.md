# 0023PHA3AEMBLOC-009: Multi-line mutation-swallow scan and baseline governance re-arm

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — test-oracle CI guards (`anti_regression_guards.rs`)
**Deps**: archive/tickets/0023PHA3AEMBLOC-001.md, archive/tickets/0023PHA3AEMBLOC-002.md, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

Two mutation-CI residues (spec 0023 `ORD-HARD-128` medium, `ORD-HARD-134` low):

- The swallow scan is single-line: it filters
  `non_comment_lines` for the literal `cargo mutants` and checks that physical line
  for `||`/`&&` — but the scheduled invocation is multi-line
  (`cargo mutants --workspace \` with flag continuation lines), so `|| echo ok`
  appended to a continuation line (which carries a flag, not the literal) is
  invisible. The only swallow synthetic mutates the single-line in-diff invocation —
  the shape the scan already catches (R-29).
- With the baseline retired to zero, the per-entry governance loops (deferral-phrase,
  repetition, ticket-existence) iterate nothing, and the empty-set FNV hash pin
  (`0xcbf2_9ce4_8422_2325`, the offset basis) adds no integrity beyond count=0.
  Benign today; a future re-population would restore entries without live
  governance unless an assertion forces ledgered per-entry dispositions on any
  non-empty baseline.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: the per-line
   `.filter(|line| line.contains("cargo mutants"))` shape in the swallow check
   (`mutation_perimeter_consistency_violations`,
   `crates/tracewake-core/tests/anti_regression_guards.rs`); the empty
   `.cargo/mutants-baseline-misses.txt`; the governance loops filtering ledger entry
   lines; `MUTANTS_BASELINE_NORMALIZED_FNV1A64 = 0xcbf2_9ce4_8422_2325` (the FNV-1a64
   offset basis — the empty-set hash).
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-128`/`134`
   (128 operator-verified at audit; 134 operator-verified at reassessment).
3. Shared contract under audit: the mutation-CI guard perimeter —
   `.github/workflows/ci.yml` scan rules and the baseline governance loops — consumed
   by both CI mutation jobs. Builds on -002's chained change-log mechanics (the
   re-arm assertion references the same ledger structures), hence the -002 Dep.
4. Constitutional motivation restated: lock durability (R-28/R-29) — the
   `ORD-HARD-101` swallow correction covered the physical-line family member only;
   a guard mechanism proven only by synthetics (because its real target set is
   empty) must fail closed when the target set re-populates.
5. This ticket touches fail-closed CI governance surfaces only: logical-line
   scanning tightens the swallow rule; the re-arm assertion tightens re-population.
   No epistemic, replay, or product surface changes. Note: `.github/workflows/ci.yml`
   itself is NOT modified — its multi-line invocation is legitimate; the *scan* is
   what changes.
6. Change rationale (no silent retcon): the scan joins continuation lines because the
   spec's evidence constructs a concrete evasion on the scheduled invocation;
   mandated by `ORD-HARD-128`/`134` required corrections.

## Architecture Check

1. Joining YAML continuation lines (collapsing trailing `\`) before scanning makes
   the swallow rule operate on logical command lines — closing the whole
   continuation family — rather than adding a second physical-line pattern that the
   next formatting variant evades. The re-arm assertion ("any non-empty baseline
   carries ledgered per-entry dispositions") makes governance vacuity self-healing:
   the moment an entry returns, the loops have enforced targets again.
2. No backwards-compatibility aliasing/shims: the single-line check is replaced by
   the logical-line check, not kept as a fallback.

## Verification Layers

1. Swallow-channel closure (R-29) -> codebase test-proof: a synthetic appending
   `|| echo ok` to the scheduled invocation's continuation line fails the
   logical-line scan — routed through the production check, enrolled under -001's
   registry with a live witness.
2. Governance re-arm (R-28) -> codebase test-proof: a synthetic non-empty baseline
   with one unledgered entry fails the re-arm assertion.
3. No-false-positive floor -> codebase test-proof: the real `ci.yml` (legitimate
   multi-line invocation, no swallow suffix) passes the joined-line scan.

## What to Change

### 1. Logical-line swallow scan (`ORD-HARD-128`)

In the swallow check inside `mutation_perimeter_consistency_violations`: join
continuation lines (collapse trailing `\`) before scanning — or scan each `run:`
block containing `cargo mutants` for any logical command line carrying a `||`/`&&`
suffix. Add the continuation-line synthetic negative.

### 2. Non-empty-baseline governance re-arm (`ORD-HARD-134`)

Add an assertion that any non-empty `.cargo/mutants-baseline-misses.txt` carries
ledgered per-entry dispositions in `reports/0020_mutants_baseline_disposition.md`
(consistent with -002's chained format), with a one-unledgered-entry synthetic
negative. Enroll both under the -001 registry.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- `.github/workflows/ci.yml` edits (the workflow is correct; the scan is what
  changes).
- The chained change-log walk itself (-002).
- mutants.toml key/glob rules (landed in 0022; verified holding).

## Acceptance Criteria

### Tests That Must Pass

1. The continuation-line swallow synthetic fails; the real `ci.yml` passes
   (`cargo test -p tracewake-core --test anti_regression_guards`).
2. The unledgered-entry re-arm synthetic fails; the current empty baseline passes
   via its recorded exemption.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. A `cargo mutants` invocation cannot carry a failure-swallowing suffix on any
   physical line of its logical command without failing CI.
2. A non-empty baseline cannot exist without live per-entry governance.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — logical-line join +
   continuation swallow negative; re-arm assertion + unledgered-entry negative;
   registry enrollment.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12
