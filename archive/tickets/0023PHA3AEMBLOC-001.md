# 0023PHA3AEMBLOC-001: Meta-lock live witness counts and closed census membership

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — test-oracle meta-locks (`anti_regression_guards.rs`), `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
**Deps**: `archive/specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

The 0022 §5.1 meta-lock tier — the standing anti-contamination posture the lineage
plans to rest on when audit cadence drops to phase boundaries — is partially
decorative (spec 0023 `ORD-HARD-122`, `ORD-HARD-123`, both high/medium):

- The nonzero-witness rule compares two hand-typed literals:
  `MetaLockRegistryEntry.witness_count` is authored at registry-writing time and
  `meta_lock_registry_errors` checks `witness_count < witness_min` — no enrolled scan
  ever reports its live matched-site count, so a scan whose anchor goes stale matches
  zero sites and stays green (the anti-rotten-green lock is itself rotten green).
- The bijection census derives membership only from the `fn guard_` prefix
  (`anti_regression_guard_test_names` strips `fn guard_`), so eleven real structural
  locks live outside the census and a new lock named without the prefix is born with
  zero census enforcement — the exact silent-birth gap §5.1 promised to close.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: `META_LOCK_REGISTRY` entries carry literal `witness_count`
   values (`1`, `3`, or consts); `meta_lock_registry_errors` compares the two authored
   numbers; `anti_regression_guard_test_names` collects only `fn guard_*`
   (`crates/tracewake-core/tests/anti_regression_guards.rs`). Vulnerable
   emptiness-only scans confirmed: `physical_mutating_event_kinds_have_explicit_world_apply_arms`,
   `agent_stream_event_kinds_have_explicit_agent_apply_arms`,
   `embodied_view_option_and_collection_fields_have_reachable_producers`.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-122`/`123`
   (operator-verified evidence) and §5 tier 2 (the clippy-`ui_test` pattern: every
   repaired meta-check gains a fixture negative routed through the production scan
   path).
3. Shared contract under audit: the meta-lock registry surface — lock↔negative
   bijection, witness minimums, and census membership — consumed by every structural
   lock in `anti_regression_guards.rs` and the gate files; sibling tickets -002, -003,
   -007, -008, -009, -010, -012 enroll new or repaired locks under the mechanics this
   ticket repairs (which is why they all carry `Deps: 0023PHA3AEMBLOC-001`).
4. Constitutional motivation restated: lock durability under the constitution's
   Enforcement reading and R-29 (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`) — a
   lock is real only if its negative can fire on the behavior; a census ratchet that
   cannot fire in the actual CI path is the R-29 symptom this ticket closes on the
   meta-tier itself.
5. This ticket touches fail-closed test-oracle governance surfaces only: the witness
   and census repairs tighten enforcement (more scans must prove live matches; more
   locks join the census); no epistemic, replay, or product surface is touched, so no
   leakage or determinism path is weakened.
6. The eleven out-of-census locks expanded from the spec's "six named + five more"
   against the codebase (all verified present):
   `scheduler_never_direct_dispatches_primitive_action`,
   `no_direct_apply_event_outside_event_replay_or_pipeline`,
   `event_apply_remains_only_post_seed_mutation_path`,
   `non_world_stream_cannot_change_physical_checksum`,
   `checksum_coverage_is_total_for_authoritative_state`,
   `new_authoritative_field_without_checksum_registry_fails`,
   `new_authoritative_field_without_canonical_checksum_line_fails`,
   `adding_event_schema_version_requires_migrator_registration`,
   `materialized_agent_apply_arms_require_payload_schema_version`,
   `materialized_agent_payload_records_keep_payload_fields`,
   `agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state`.
7. Change rationale (no silent retcon): the witness literals and prefix-scoped census
   change because spec 0023's evidence proves the delivered shapes cannot fire on the
   regressions they claim to police; the change is mandated by `ORD-HARD-122`/`123`'s
   required corrections.

## Architecture Check

1. Measuring the witness inside each scan (the scan returns its real matched-site
   count and asserts `count >= witness_min` in its own body, feeding the same live
   count to the registry) is cleaner than auditing literals harder: the number can no
   longer be authored, only measured — the same reasoning that replaced the
   self-echoing policy table in 0021/0022. Deriving census membership from the full
   `#[test]` population minus an explicit rationale-bearing allowlist closes the
   complement: a new lock cannot be born outside the census because *everything* is in
   the census unless visibly exempted.
2. No backwards-compatibility aliasing/shims: the literal `witness_count` field is
   replaced (renamed to a declared-minimum field with the live count supplied by the
   scan), not kept alongside; the `guard_`-prefix derivation is replaced by the closed
   population derivation, not supplemented.

## Verification Layers

1. Live-witness integrity (R-29) -> codebase test-proof: a synthetic anchor-miss
   fixture (a scan run against a body lacking its anchor) fails via the in-scan
   live-count assertion — routed through the production scan path.
2. Census closure (R-28) -> codebase test-proof: a synthetic unprefixed structural
   lock outside the allowlist fails the census; each of the eleven enumerated locks is
   either census-enrolled with a negative or carries a rationale-bearing allowlist
   entry.
3. Registry self-reflexivity -> codebase grep-proof: the census still lists itself
   with its own negative (the 0022 self-application holds post-repair).
4. Risk-register doc correction -> manual review: R-29 extended with the
   *literal witness count* and *binding laundering* shapes; R-28 watch note on
   convention-scoped membership derivations, per spec §6.

## What to Change

### 1. Live witness counts (`ORD-HARD-122`)

Each enrolled scan-style guard returns its real matched-site count from the scan body
and asserts `count >= witness_min` inside the guard (path-under-test). The registry
comparison consumes the same live count; the authored `witness_count` field is removed
or renamed `witness_min_declared` so an authored number can never stand in for a
measured one. Add the anchor-miss synthetic negative. The structurally-exempt
empty-baseline entry (`mutation_baseline_misses_are_pinned_and_ledgered`,
`witness_min: 0`) keeps its recorded exemption.

### 2. Closed census membership (`ORD-HARD-123`)

Derive guard-test membership from the full `#[test]` population of
`anti_regression_guards.rs` (and the gate files the registry already covers) minus an
explicitly-cited non-lock allowlist, each entry with rationale — replacing the
`fn guard_` prefix derivation. Enroll or allowlist each of the eleven enumerated
locks (Assumption 6). Add the unprefixed-synthetic negative.

### 3. Risk-register extensions (spec §6)

Extend R-29 in `docs/3-reference/01_DESIGN_RISK_REGISTER.md` with the two new shapes
(literal witness count; binding laundering) and add the R-28 watch note
(naming-convention-scoped membership derivation is hand-maintained outside the
convention).

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- The two-sided ratchet's from→to chain and the deferral content witness
  (`ORD-HARD-129`/`124` — tickets -002/-003).
- Any product-code change (the embodied migration is -004/-005).
- New locks for sibling findings (each sibling ticket enrolls its own under the
  repaired mechanics).
- Conformance-index rows (capstone -013, honest-evidence placement).

## Acceptance Criteria

### Tests That Must Pass

1. The anchor-miss synthetic fails the live-count assertion; restoring the anchor
   passes it (`cargo test -p tracewake-core --test anti_regression_guards`).
2. The unprefixed-lock synthetic fails the census; each of the eleven enumerated
   locks (Assumption 6) is census-enrolled with a negative or allowlisted with
   rationale — one criterion per lock, verified by the census output.
3. `cargo test --workspace` green; `cargo fmt --all --check` and
   `cargo clippy --workspace --all-targets -- -D warnings` clean.

### Invariants

1. No registry entry's effective witness count is an authored literal — every
   scan-kind entry's count is produced by the scan it describes.
2. The census membership complement is closed: every `#[test]` in the covered files
   is enrolled or visibly allowlisted with rationale.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — in-scan live-count
   assertions + anchor-miss negative; closed-population census + unprefixed negative;
   allowlist entries for genuine non-lock tests.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Implemented the meta-lock live-witness and closed-census repair in
`crates/tracewake-core/tests/anti_regression_guards.rs`.

- Removed authored `witness_count` literals from `MetaLockRegistryEntry`; effective
  counts are now computed from live scan bodies, registry/test populations, or the
  source surfaces the lock describes.
- Added the anchor-miss synthetic so a stale scan anchor fails the live-count rule.
- Replaced the `fn guard_`-only census with a full `#[test]` population census plus
  rationale-bearing exemptions for non-lock tests.
- Enrolled the eleven previously out-of-census structural locks named in
  Assumption 6, plus the existing typed-column structural test discovered during
  implementation.
- Extended R-28/R-29 in `docs/3-reference/01_DESIGN_RISK_REGISTER.md` with the
  convention-scoped census, literal witness count, and binding-laundering shapes.

Deviations: `typed_column_closure_exemptions_are_rationale_bearing_and_live` was
enrolled as an additional existing structural lock because the repaired full-test
census exposed it as outside the old prefix-scoped membership.

Verification:

- `cargo test -p tracewake-core --test anti_regression_guards` — passed
- `cargo fmt --all --check` — passed after applying `cargo fmt --all`
- `cargo clippy --workspace --all-targets -- -D warnings` — passed
- `cargo build --workspace --all-targets --locked` — passed
- `cargo test --workspace` — passed
