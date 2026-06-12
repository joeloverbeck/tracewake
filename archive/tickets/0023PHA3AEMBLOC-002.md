# 0023PHA3AEMBLOC-002: Chained baseline change-log ratchet and 0022 report correction

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — test-oracle guards (`anti_regression_guards.rs`), `reports/0020_mutants_baseline_disposition.md` change-log format, `reports/0022_ord_life_cert_scoped_acceptance.md`
**Deps**: archive/tickets/0023PHA3AEMBLOC-001.md, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

Two evidence-integrity residues from 0022 (spec 0023 `ORD-HARD-129` medium,
`ORD-HARD-133` low):

- The two-sided mutation ratchet's "recorded decrease" is a destination-only presence
  check: `mutation_baseline_change_log_records` passes iff any ledger line contains
  `baseline-delta:` plus markers matching *today's* count and hash. There is no
  prior-value field and no chain, so the same hand-edit that silently shrinks the
  baseline also satisfies the record — an unrecorded and a recorded decrease are
  indistinguishable.
- `reports/0022_ord_life_cert_scoped_acceptance.md` §1 still claims "All 143
  remaining normalized baseline entries are ledgered … test debt was filed" — but the
  0022 follow-ups (`0022PHA3ABASTRI-015`–`023`) *retired* the baseline to zero; the
  checklist guard checks anchor presence, not narrative consistency.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: `mutation_baseline_change_log_records`
   (`crates/tracewake-core/tests/anti_regression_guards.rs`) is
   `ledger.lines().any(|line| line.contains("baseline-delta:") && count_marker && hash_marker)`;
   `.cargo/mutants-baseline-misses.txt` is empty;
   `MUTANTS_BASELINE_NORMALIZED_COUNT = 0`,
   `MUTANTS_BASELINE_NORMALIZED_FNV1A64 = 0xcbf2_9ce4_8422_2325`; the stale "All 143"
   sentence sits in the 0022 report §1.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-129`/`133`
   (operator-verified evidence); the spec's either-or for 133 (amend vs supersede)
   resolved at decomposition to amend-in-place, the lineage precedent (0022 amended
   the 0021 report's §2 title and pending status directly).
3. Shared contract under audit: the mutation-baseline evidence chain —
   `.cargo/mutants-baseline-misses.txt` ↔ `reports/0020_mutants_baseline_disposition.md`
   change log ↔ pinned count/hash consts ↔
   `reports/0022_ord_life_cert_scoped_acceptance.md` narrative — consumed by the CI
   mutation ratchets and the acceptance-checklist guard.
4. Constitutional motivation restated: lock durability (R-29) and the R-28 watch note
   ("a perform-work-once correction needs the work product as evidence") — the
   decrease side of a two-sided ratchet must prove a *transition*, not a destination.
5. This ticket touches the deterministic mutation-evidence surface (pinned
   count/hash) and its fail-closed governance: the chain check tightens it (every
   delta must link `from`→`to` back to the pinned head); no epistemic or replay
   surface is touched.
6. Change rationale (no silent retcon): the change-log line format gains `from`
   fields because the delivered format provably cannot distinguish recorded from
   unrecorded shrinks; the 0022 report §1 sentence is corrected because the spec's
   evidence proves it describes a state that no longer exists.

## Architecture Check

1. A verifiable chain (each entry carries `from`→`to` count+hash; each `to` is the
   next entry's `from`; the head equals the pinned consts) is cleaner than adding a
   review-attestation field: the chain is checkable by walking it, needs no trust in
   the author, and reuses the existing pinned-const anchor as the head. The
   alternative — signing or dating entries — adds ceremony without making a silent
   shrink detectable.
2. No backwards-compatibility aliasing/shims: existing change-log lines are rewritten
   into the chained format in the same edit (the ledger is repo-internal evidence,
   not an external API); the presence-only check is replaced, not kept as a fallback.

## Verification Layers

1. Two-sided ratchet integrity (R-29) -> codebase test-proof: a synthetic shrink
   whose change-log line lacks the matching `from` predecessor fails the chain walk —
   routed through the production check path, enrolled under -001's live-witness rule.
2. Chain-head anchoring -> codebase grep-proof: the chain's head entry matches
   `MUTANTS_BASELINE_NORMALIZED_COUNT`/`FNV1A64` exactly.
3. Evidence honesty (`ORD-HARD-133`) -> manual review + grep-proof: 0022 report §1
   states the 143→0 retirement; the literal "All 143 remaining" claim is gone.

## What to Change

### 1. Chained change-log format and walk (`ORD-HARD-129`)

Rewrite `reports/0020_mutants_baseline_disposition.md` change-log entries to carry
`from`→`to` count+hash pairs forming a chain to the pinned head. Replace
`mutation_baseline_change_log_records`'s presence check with a chain walk (each `to`
is the successor's `from`; head equals the pinned consts). Add the
missing-predecessor synthetic negative; enroll the lock under the repaired registry
mechanics from -001.

### 2. 0022 report §1 correction (`ORD-HARD-133`)

Amend `reports/0022_ord_life_cert_scoped_acceptance.md` §1 to state the baseline was
fully retired 143→0 via tickets `0022PHA3ABASTRI-015`–`023`, removing the "All 143
remaining … are ledgered" claim. Extend the acceptance-checklist guard to assert the
§1 narrative count matches `MUTANTS_BASELINE_NORMALIZED_COUNT`.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `reports/0020_mutants_baseline_disposition.md` (modify)
- `reports/0022_ord_life_cert_scoped_acceptance.md` (modify)

## Out of Scope

- The non-empty-baseline governance re-arm assertion (`ORD-HARD-134` — ticket -009).
- The witness/census mechanics themselves (-001).
- The 0023 acceptance artifact (capstone -013).

## Acceptance Criteria

### Tests That Must Pass

1. The missing-predecessor synthetic shrink fails the chain walk; the rewritten real
   chain passes with its head equal to the pinned consts
   (`cargo test -p tracewake-core --test anti_regression_guards`).
2. The checklist guard fails on the old "All 143 remaining" §1 narrative and passes
   on the corrected one.
3. `cargo test --workspace` green; fmt + clippy clean.

### Invariants

1. Every baseline count/hash transition is recorded as a `from`→`to` link reaching
   the pinned head — a shrink without a predecessor link cannot pass CI.
2. Acceptance-report narrative claims about the baseline are machine-checked against
   the pinned consts, not prose-trusted.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — chain-walk check +
   missing-predecessor negative; §1 narrative-count assertion in the checklist guard.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Implemented the chained mutation-baseline evidence check and corrected the stale
0022 acceptance-report baseline narrative.

- Rewrote `reports/0020_mutants_baseline_disposition.md` baseline-delta entries to
  carry `from-count`/`from-fnv1a64` -> `to-count`/`to-fnv1a64` links ending at the
  pinned zero baseline.
- Replaced the destination-only change-log check in
  `crates/tracewake-core/tests/anti_regression_guards.rs` with a parsed chain walk
  whose head must match the pinned count/hash and whose adjacent links must connect.
- Added a missing-predecessor synthetic and updated the floor-raise synthetic for
  the new chained format.
- Corrected `reports/0022_ord_life_cert_scoped_acceptance.md` §1 to state that the
  baseline was retired 143 -> 0 through `0022PHA3ABASTRI-015` through `-023`.
- Extended the 0022 acceptance-artifact guard to reject the old "All 143 remaining"
  narrative and require the pinned `normalized-count 0` claim.

Deviations: none.

Verification:

- `cargo test -p tracewake-core --test anti_regression_guards` — passed
- `cargo fmt --all --check` — passed after applying `cargo fmt --all`
- `cargo clippy --workspace --all-targets -- -D warnings` — passed
- `cargo build --workspace --all-targets --locked` — passed
- `cargo test --workspace` — passed
