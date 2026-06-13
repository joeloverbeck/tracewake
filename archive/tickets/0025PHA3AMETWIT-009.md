# 0025PHA3AMETWIT-009: Record the canonical recovery-resolution staging decision

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None by the recommended branch (documentation record); Yes — `tracewake-content` fixtures/tests, only if the recorded decision elects to author the second resolution.
**Deps**: None

## Problem

Spec 0025 finding `ORD-HARD-187` (low). The 0005 contract's §12/§17.1
*recommended* success-recovery demonstration (Mara replans to a public/neighbor
food source, eats, continues) is undemonstrated anywhere: the canonical fixture
pins `canonical_mara_recovery_resolution=fail_only_empty_food_source`
(`no_human_day_001.rs`), the guard arm enforces fail-only, no
`recovered_via_*` resolution token exists in the crates, and no test proves a
successful FindFood replan→eat→continue chain. The fail-only pinning mechanism
itself works as designed (recorded fixture decision; flipping the token to an
unsupported value fails loudly — validated-no-action in spec 0025 §3); the defect
is that the *staging* of the recommended success variant is recorded in no
execution doc — the same unrecorded-deferral class as `ORD-HARD-121`/`152`.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `9e33d7a`: the
   `canonical_mara_recovery_resolution=fail_only_empty_food_source` token in
   `crates/tracewake-content/src/fixtures/no_human_day_001.rs`; the
   `"fail_only_empty_food_source"` arm in
   `crates/tracewake-content/tests/golden_fixtures_run.rs`; zero
   `recovered_via`/recovery-resolution token matches across the crates
   (re-verified at reassessment per spec 0025 §8).
2. Verified against spec 0025 §4/§6 and the 0005 contract
   (`archive/specs/0005_*.md` §12, §17.1: the success variant is *recommended*,
   the minimum "replans OR fails" is met) and
   `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (the
   record's destination).
3. Shared boundary under audit: the canonical-fixture resolution vocabulary ↔ the
   execution-doc record — a divergence from a contract recommendation is lawful
   only as a recorded, owned decision (the `ORD-HARD-161` precedent).
4. Rule restated before trusting the narrative (the lineage's recorded-deferral
   rule, `ORD-HARD-121`/`152` precedent; INV-098's harsh-acceptance posture): a
   staged/deferred feature's absence is lawful only when the staging decision is
   execution-doc-recorded — silence is drift, a record is a decision.

## Architecture Check

1. Recording the staging decision in `docs/2-execution/06` (the recommended
   branch) is cleaner than authoring the success variant now: the contract minimum
   is met, the resolution-token mechanism already makes any future variant an
   explicit, visible edit (new token + guard arm), and building Phase 3B-adjacent
   content to close a documentation gap would be scope inflation. The decision
   remains the implementer's per spec §8 — if it flips to authoring, the token +
   arm + fixture/test land here with the same recording obligation.
2. No backwards-compatibility aliasing/shims: whichever branch is recorded, the
   fail-only arm is either kept as the sole accepted resolution (with the record)
   or joined by a real second token — never a tolerant catch-all.

## Verification Layers

1. Recorded staging (`ORD-HARD-187`, recommended branch) → grep-proof: the
   staging record (fail-only canonical resolution; success variant staged, trigger
   named) present in `docs/2-execution/06`; swept by the doc-honesty pass of the
   0025 acceptance artifact (ticket `0025PHA3AMETWIT-011`).
2. Authoring branch (only if the recorded decision flips) → the
   `recovered_via_find_food` token + guard arm + a fixture/test proving the
   replan→eat→continue chain with `EatFailed` + `ActorMoved` ancestry.
3. Mechanism continuity → the existing resolution-token tests
   (`flipped_resolution`, the tampered-log Mara negative) stay green either way.

## What to Change

### 1. Decide and record (implementer-recorded choice)

Record in `docs/2-execution/06`: the canonical `no_human_day_001` recovery
resolution is fail-only by decision; the §12-recommended success-recovery variant
is staged (name the re-evaluation trigger — e.g. Phase 3B routine/economy work
making public food sources first-class), or — if the implementer elects the
authoring branch — land the second resolution per Verification layer 2 and record
that instead.

## Files to Touch

- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)
- `crates/tracewake-content/src/fixtures/no_human_day_001.rs` (modify — only if the recorded decision authors the second resolution)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — only if the recorded decision authors the second resolution)

## Out of Scope

- Building player-facing or Phase 3B food-economy content (the staging record is
  the deliverable; the variant itself is staged work).
- The resolution-token mechanism (works as designed; spec 0025 §3
  validated-no-action).
- Any doctrine amendment.

## Acceptance Criteria

### Tests That Must Pass

1. The staging record (or the authored second resolution) is present:
   grep-proof in `docs/2-execution/06` naming the fail-only decision and the
   staged variant's trigger — or the new token + arm + chain test passing.
2. Existing golden-fixture resolution tests stay green
   (`cargo test -p tracewake-content`).
3. The four gates (`cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. Every divergence from a 0005-recommended demonstration is an execution-doc-
   recorded decision with a named re-evaluation trigger — never silent drift.
2. The canonical fixture's resolution vocabulary admits only explicitly supported,
   guard-enforced values.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket on the recommended branch; verification is
   command-based (grep-proof + existing golden-fixture suite), and the authoring
   branch's tests are enumerated in Verification layer 2 if elected.`

### Commands

1. `grep -n "fail_only\|staged" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
2. `cargo test -p tracewake-content`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-13

Recorded the canonical `no_human_day_001` recovery resolution in
`docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` as
`fail_only_empty_food_source`, with Mara's fail-only empty-food-source behavior
called out as a fixture decision rather than an implicit proof relaxation. Also
recorded the 0005 section 12 success-recovery variant as staged until Phase 3B
routine/economy work makes public food services or neighbor/public pantry access
first-class actor-known affordances, requiring either an explicit future recovery
token/guard arm or a renewed staging record.

Verification:

1. `grep -n "fail_only\|staged" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
2. `cargo test -p tracewake-content`
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
