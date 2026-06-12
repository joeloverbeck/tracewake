# 0024PHA3ACONSCH-002: Deferral content witness, baseline-ledger genesis anchor, and census-exemption validation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` test guards (`crates/tracewake-core/tests/anti_regression_guards.rs`); possibly a normalization edit to `reports/0020_mutants_baseline_disposition.md` (evidence ledger, as surfaced).
**Deps**: 0024PHA3ACONSCH-001

## Problem

Three residual decorative shapes in the meta-lock tier (spec 0024 §4):

- `ORD-HARD-142` (high): the `ORD-HARD-124` deferral repair is self-satisfying — the
  empty-snippet branch of `embodied_field_has_registered_producer` requires only
  `source.contains(entry.field_name)`, and the sole production deferral entry
  (`debug_only_diagnostics`) cites `view_models.rs`, the file that *defines* the
  field, so the check can never fail; the field could be orphaned with the sweep
  green.
- `ORD-HARD-145` (medium): `mutation_baseline_change_log_records` validates interior
  `windows(2)` adjacency and the head pin only — a fabricated single-link ledger
  whose lone `from` equals its `to` equals the pinned head passes, erasing the
  recorded 143→0 retirement history.
- `ORD-HARD-155` (low): `META_LOCK_CENSUS_EXEMPTIONS` accepts any non-empty rationale
  prose; a future structural lock can park itself there and escape enrollment.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: the
   `entry.producer_snippet.is_empty()` branch and the `debug_only_diagnostics`
   entry citing `view_models.rs` (which defines `ActionAvailability::Disabled {
   debug_only_diagnostics }` and `pub fn debug_only_diagnostics()`); the
   `windows(2)` walk + head-only pin in `mutation_baseline_change_log_records`;
   the rationale-only exemption filter — all operator-verified at audit or spec
   reassessment.
2. Verified against spec 0024 §4 (`ORD-HARD-142`/`145`/`155`) and
   `docs/3-reference/01_DESIGN_RISK_REGISTER.md` R-28/R-29; the chain ledger lives
   in `reports/0020_mutants_baseline_disposition.md` (file existence verified).
3. Shared boundaries under audit: the `EMBODIED_SURFACE_FIELD_PRODUCERS` deferral
   contract; the baseline change-log format shared between the guard and the
   `reports/0020` evidence artifact; the census-exemption contract between
   `META_LOCK_CENSUS_EXEMPTIONS` and `meta_lock_registry_errors`.
4. Rule restated (R-29): a content witness pointed at the definition site of the
   thing it witnesses is unfalsifiable; a transition record without a pinned origin
   is replaceable; an exemption validated only by prose presence is a parking spot.
5. Enforcement surface: mutation-CI governance (the ledger chain) and the embodied
   surface sweep are test-oracle/evidence surfaces; this ticket strengthens both and
   relaxes no validation, actor-knowledge, or replay behavior. The genesis-pin
   consts are derived from the *existing recorded history* (first delta `from` =
   143 and its recorded hash) — no history is rewritten; if the current ledger's
   first delta needs a format normalization to carry an explicit genesis `from`,
   that edit is a format completion recorded in the report's change log, not a
   content change.
6. Adjacent-contradiction classification: none expected; if the deferral-entry
   repair reveals `debug_only_diagnostics` genuinely orphaned in production (no
   write/consume site outside its definition), that is a separate bug — file it as
   its own follow-up ticket rather than silently widening this one.

## Architecture Check

1. Write/consume-shaped citation requirements, a pinned genesis, and
   covering-lock-id exemption validation each convert an authored assertion into a
   measured one — the same authored→measured direction ticket -001 establishes for
   witness counts, applied to the three remaining authored surfaces. Cleaner than
   per-entry allowlists because each check derives from the artifact it polices.
2. No backwards-compatibility aliasing/shims: the empty-snippet short-circuit is
   removed, not wrapped; the old single-link-tolerant walk is replaced.

## Verification Layers

1. R-29 deferral durability → orphaned-definition synthetic: a deferral entry whose
   cited source contains the field only as a `struct`/`enum`/`fn` definition (no
   write or consume site) must fail (test).
2. R-29 ledger durability → fabricated-ledger synthetic: a single-link ledger with
   matching head but `from ≠ pinned genesis` must fail; a shortened chain must fail
   (tests).
3. R-28 census closure → exemption synthetic: an exemption wrapping a test body that
   contains a violation-set scan and names no covering `lock_id` must fail (test).
4. Whole-tier regression → `cargo test -p tracewake-core --test
   anti_regression_guards` + full workspace gates.

## What to Change

### 1. Deferral content witness (`ORD-HARD-142`)

The cite-only path requires a write-shaped or consumer-shaped occurrence
(constructor field init, `.field_name(` consumer call, or a named non-test app-layer
caller assertion) outside the field's own definition span. Enroll the per-entry
match in the live-witness rule completed by ticket -001.

### 2. Ledger genesis anchor (`ORD-HARD-145`)

Pin the genesis `from` count + hash as consts (from the recorded 143-era origin);
assert `deltas.first()` matches; assert chain length is monotonically non-decreasing
(or pin a full-list content hash governed by the same two-sided ratchet discipline).
Normalize `reports/0020_mutants_baseline_disposition.md`'s first delta only if its
format lacks an explicit genesis `from` (as surfaced; record any edit in the
report's change log).

### 3. Census-exemption validation (`ORD-HARD-155`)

Each `META_LOCK_CENSUS_EXEMPTIONS` entry must either name a covering registry
`lock_id` (asserted present in `META_LOCK_REGISTRY`) or pass a no-scan-shape
assertion over its test body. The five current entries are re-validated under the
new rule and their rationales updated to carry covering lock_ids where applicable.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `reports/0020_mutants_baseline_disposition.md` (modify — only if genesis-format
  normalization is surfaced as needed; see What to Change §2)

## Out of Scope

- The default-arm witness elimination (ticket -001, this ticket's dependency).
- Re-populating the mutation baseline or changing its governance semantics
  (`ORD-HARD-134` landed in 0023; untouched).
- Fixing `debug_only_diagnostics` itself if found orphaned (separate follow-up
  ticket per Assumption item 6).

## Acceptance Criteria

### Tests That Must Pass

1. Orphaned-definition deferral synthetic fails; the production
   `debug_only_diagnostics` entry passes only via a genuine write/consume citation.
2. Fabricated single-link and shortened-chain ledger synthetics fail; the genesis
   consts match the recorded first delta; the existing 143→0 chain still validates.
3. The exemption synthetic (scan-shaped body, no covering lock_id) fails; all five
   current exemptions validate under the new rule.
4. `cargo test -p tracewake-core --test anti_regression_guards`,
   `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace` pass.

### Invariants

1. No deferral entry can be satisfied by its witnessed field's own definition site.
2. The baseline change-log chain is append-only over a pinned origin; wholesale
   replacement is detectable.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — orphaned-definition
   deferral synthetic; genesis-mismatch and shortened-chain ledger synthetics;
   scan-shaped-exemption census negative.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
