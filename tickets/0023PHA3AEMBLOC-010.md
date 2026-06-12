# 0023PHA3AEMBLOC-010: In-context discrimination witness and unwrap panic-guard coverage

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — hidden-truth gate harness (`hidden_truth_gates.rs`), panic guard (`anti_regression_guards.rs`)
**Deps**: 0023PHA3AEMBLOC-001, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

Two low-severity enforcement gaps (spec 0023 `ORD-HARD-131`, `ORD-HARD-132`):

- The planner hidden-truth discrimination witness
  (`planner_hidden_truth_fixture_witness_errors`) is opt-in: it fires only because
  the two existing planner gates call it (plus its own self-check); a future
  `context()` caller seeding a non-empty adversarial fixture has no structural
  obligation to run it, and the witness's own negative proves it fails on an *empty*
  context, not on a leak.
- The log-derived panic guard (`log_derived_panic_violations`) scans `.expect(` and
  `assert!(` but not `.unwrap(` — a future `.unwrap()` on log-derived data in
  scheduler/apply paths would panic on corrupt history (INV-020) yet escape the
  guard. Current production `.unwrap()` sites verified benign (static-string
  constructions).

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: `planner_hidden_truth_fixture_witness_errors` called at
   exactly three sites in `crates/tracewake-core/tests/hidden_truth_gates.rs` (the
   two planner gates + the empty-context self-check); the
   `.expect(`/`assert!(`-only token filter in `log_derived_panic_violations`
   (`crates/tracewake-core/tests/anti_regression_guards.rs`); production
   `scheduler.rs` `.unwrap()` calls are on freshly-constructed static-string
   IDs, not parsed log data.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-131`/`132`
   (operator-verified at reassessment / at audit respectively); the
   `adversarial_truth_world` fixture and the gates' genuine discrimination are
   0022-verified holding — this ticket makes the witness unbypassable, it does not
   rebuild it.
3. Shared contract under audit: the hidden-truth harness's `context()` seam (every
   caller seeding adversarial fixtures) and the reject-loudly panic perimeter over
   scheduler/apply log-derived data — both enforcement surfaces for the truth
   firewall and replay posture.
4. Constitutional motivation restated: INV-099–101 (the planner gates exist to prove
   hidden truth stays out of sealed context — the witness is the proof that the
   discrimination occurred, not just that rejection happened) and INV-020/018
   (reject loudly: corrupt history gets typed errors, not process aborts).
5. This ticket touches actor-knowledge filtering and deterministic-replay
   enforcement surfaces: moving the witness inside `context()` makes leak detection
   unbypassable for every present and future caller; the `.unwrap(` token closes the
   panic-guard family. Both tighten; nothing is weakened.
6. Change rationale (no silent retcon): the witness moves in-`context()` because the
   opt-in shape provably lets future callers skip it; the token filter widens because
   the `.expect`-only set is an incomplete family (R-28). Mandated by
   `ORD-HARD-131`/`132`.

## Architecture Check

1. Running the discrimination check inside `context()` itself (hidden counterparts
   asserted absent from the returned context, unconditionally) is cleaner than a
   registry of callers-who-must-call-the-witness: the check becomes a property of
   constructing a context at all, with no enrollment to forget. Adding `.unwrap(` to
   the existing allowlist-bearing guard reuses proven mechanics rather than a second
   guard.
2. No backwards-compatibility aliasing/shims: the standalone witness calls collapse
   into the in-`context()` check (the helper may remain as the implementation, but
   no caller-side opt-in path survives).

## Verification Layers

1. Unbypassable discrimination (INV-099–101) -> codebase test-proof: a synthetic
   that injects the hidden id into the built surface makes `context()` itself fail —
   no gate-side call required; routed through the production helper, enrolled under
   -001's registry.
2. Reject-loudly closure (INV-020) -> codebase test-proof: a synthetic `.unwrap()`
   on payload data in a scanned path fails the widened guard; the benign
   static-string `.unwrap()` sites pass via the existing rationale-bearing allowlist
   mechanics.
3. Existing-gate continuity -> codebase test-proof: both planner gates and the
   empty-context self-check stay green against the in-`context()` witness.

## What to Change

### 1. In-`context()` discrimination witness (`ORD-HARD-131`)

Move the discrimination check inside `context()` in `hidden_truth_gates.rs`: for any
non-empty adversarial fixture, assert the unseeded hidden counterparts are absent
from the returned context unconditionally. Add the leak synthetic (hidden id injected
into the surface ⇒ `context()` fails).

### 2. `.unwrap(` token coverage (`ORD-HARD-132`)

Add `.unwrap(` to `log_derived_panic_violations`'s token filter with the same
`PANIC_ALLOWLIST` mechanics; allowlist the verified-benign static-string sites with
rationales; add the synthetic `.unwrap()`-on-payload negative. Enroll both repairs
under the -001 registry.

## Files to Touch

- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Production scheduler/apply changes (no live violation exists; the benign
  `.unwrap()` sites are allowlisted, not rewritten).
- The gates' adversarial fixtures themselves (0022-landed; verified holding).

## Acceptance Criteria

### Tests That Must Pass

1. The hidden-id-injection synthetic fails inside `context()`; both planner gates
   and the empty-context self-check stay green
   (`cargo test -p tracewake-core --test hidden_truth_gates`).
2. The `.unwrap()`-on-payload synthetic fails the widened guard; allowlisted benign
   sites pass with rationales
   (`cargo test -p tracewake-core --test anti_regression_guards`).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. No `context()` caller can construct a sealed test context over a non-empty
   adversarial fixture without the leak discrimination running.
2. No panic-shaped read (`.expect(`/`assert!(`/`.unwrap(`) of log-derived data in
   scanned paths escapes the guard without a rationale-bearing allowlist entry.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — in-`context()` witness +
   hidden-id-injection synthetic.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — widened token filter +
   `.unwrap()` negative + allowlist entries; registry enrollment.

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates && cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
