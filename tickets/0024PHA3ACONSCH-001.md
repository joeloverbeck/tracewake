# 0024PHA3ACONSCH-001: Complete the meta-lock live-witness rule — eliminate the presence-check default arm and the anchor-presence witnesses

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` test guards (`crates/tracewake-core/tests/anti_regression_guards.rs`); no production crate code.
**Deps**: None

## Problem

Spec 0024 findings `ORD-HARD-141` (high) and `ORD-HARD-146` (medium): the 0023-repaired
live-witness meta-rule is real for only ~14 explicitly-cased scans. Every other
`META_LOCK_REGISTRY` entry falls through to a `_ =>` arm in
`meta_lock_live_witness_count` whose "witness" is
`usize::from(test_names.contains(entry.lock_id) || anti_regression_source.contains(entry.negative_id))`
— a 0/1 presence check proving the test *exists*, not that its scan matched real
sites. Renaming a default-arm scan's internal anchor leaves its violation set
vacuously empty, the witness at 1, and the suite green. Additionally, several
explicitly-cased arms measure a *neighboring anchor population* rather than the
sites the violation predicate inspected (`typed_column_closure_oblique_payload_helper_calls`
counts exemption entries; `guard_011_no_human_day_runner_only_evidence` counts
`run_no_human_day` lines; `guard_014_perception_visibility_other_emission_paths`
counts three marker literals), so a defeated detector leaves those witnesses pinned.
The meta-tier is what the planned per-phase audit cadence rests on; a decorative
witness there is the exact rotten-green class the tier exists to kill.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: `meta_lock_live_witness_count`
   (`crates/tracewake-core/tests/anti_regression_guards.rs`) carries the `_ =>`
   presence arm (`usize::from(test_names.contains(entry.lock_id) || …)`); the three
   anchor-presence cased arms named above were confirmed verbatim during spec
   reassessment. The literal `witness_count` registry field is already gone
   (0023's `ORD-HARD-122` partial repair) — this ticket completes that repair, it
   does not restart it.
2. Verified against spec 0024 §4 (`ORD-HARD-141`/`146`) and
   `docs/3-reference/01_DESIGN_RISK_REGISTER.md` R-29 (guard vacuity / literal
   witness shapes); both findings operator-verified per the spec's §8 verification
   posture.
3. Shared boundary under audit: the `META_LOCK_REGISTRY` ↔ guard-function contract —
   every enrolled lock's witness value must originate in the scan/negative
   execution itself, not in registry authoring or name presence.
4. Rule restated before trusting the narrative (R-29, lineage lock-durability): a
   lock is real only if its negative case can actually fire on the behavior; a
   witness is real only if it is measured by the detection path it vouches for.
5. Enforcement surface touched: the meta-lock tier is the
   enforcement-of-enforcement for the repo's fail-closed validation and
   anti-contamination gates. This ticket strengthens it (measured witnesses) and
   weakens nothing: no scan's violation predicate is relaxed, no actor-knowledge or
   replay surface is touched.
6. Adjacent-contradiction classification (spec §9 risk): once witnesses are live,
   scans that have been matching zero sites will fail honestly. Each such failure is
   triaged **inside this ticket** as a potential masked defect first, per the
   Enforcement reading — fixed if a real defect, or its anchor corrected if drifted;
   neither outcome is deferred silently.

## Architecture Check

1. Routing every entry to a measured count (and giving `BehaviorAssertion` rows an
   executed-negative routing instead of borrowing the nonzero-witness contract) is
   cleaner than enumerating more cased arms onto a presence fallback: the absence of
   a `_ =>` arm makes the compiler force an explicit routing decision for every
   future registry entry — the strongest available tier for test infrastructure,
   mirroring the `cause_required` exhaustive-match precedent (`ORD-HARD-139`).
2. No backwards-compatibility aliasing/shims: the presence arm is removed, not kept
   as a fallback; no scan keeps a legacy witness path.

## Verification Layers

1. R-29 lock durability (witness measured, not authored) → anchor-miss synthetic: a
   formerly-default-arm scan run against a body with its anchor absent must drop its
   witness below `witness_min` and fail (test in `anti_regression_guards.rs`).
2. Registry contract closure → `meta_lock_registry_errors` census rejects any new
   entry that routes to a presence-shaped witness (codebase grep-proof: zero
   `test_names.contains(entry.lock_id)` witness routes remain).
3. Anchor-presence repair (`ORD-HARD-146`) → per repaired arm, an inspected-site-empty
   synthetic (anchors intact, inspected-site set emptied) must drop the witness
   (test per arm).
4. Whole-tier regression → `cargo test -p tracewake-core --test
   anti_regression_guards` plus the full workspace suite.

## What to Change

### 1. Eliminate the presence-check default arm (`ORD-HARD-141`)

Remove `_ =>` from `meta_lock_live_witness_count`. Every `SharedScan`-routed registry
entry returns a real matched/inspected-site count derived from its scan body.
`MetaLockRouting::BehaviorAssertion` rows get a distinct routing whose witness is the
negative's actual execution (run-and-must-fire), not name presence — per the
rotten-green-test counter pattern recorded in spec 0024 §2 (assertion/scan executions
counted; zero ⇒ fail).

### 2. Repair the three anchor-presence cased arms (`ORD-HARD-146`)

For `typed_column_closure_oblique_payload_helper_calls`,
`guard_011_no_human_day_runner_only_evidence`, and
`guard_014_perception_visibility_other_emission_paths`: the witness becomes the count
of sites the violation predicate actually inspected (lines/files examined), not the
exemption-entry, anchor-line, or marker-literal population. Sweep the remaining cased
arms for the same shape while in the file; repair any sibling found (record each in
the ticket's implementation notes).

### 3. Firing negatives

Add: (a) an anchor-miss synthetic exercising a formerly-default-arm scan; (b) one
inspected-site-empty synthetic per repaired cased arm; (c) a census negative proving
a synthetic presence-shaped registry entry is rejected.

### 4. Triage honest failures

Any scan that fails once its witness is live is investigated as a masked defect
before its anchor is adjusted (Assumption item 6).

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- The deferral content witness, ledger genesis anchor, and census-exemption
  validation (`ORD-HARD-142`/`145`/`155`) — ticket `0024PHA3ACONSCH-002`.
- New locks introduced by sibling tickets (they enroll under the repaired rule but
  land in their own tickets).
- Any production-crate change; any doctrine amendment.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -c '_ =>' `-style proof: `meta_lock_live_witness_count` contains no default
   arm; every registry entry (≈29 formerly-default at decomposition time — re-count
   from `META_LOCK_REGISTRY` at implementation, do not hardcode) routes explicitly.
2. The anchor-miss synthetic fails via the live count for a formerly-default-arm
   scan; the three repaired arms each fail their inspected-site-empty synthetic.
3. `cargo test -p tracewake-core --test anti_regression_guards` and
   `cargo test --workspace` pass; `cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked` pass.

### Invariants

1. No `META_LOCK_REGISTRY` entry's witness can be satisfied by registry authoring or
   test-name presence — only by the detection path executing.
2. The meta-tier remains self-reflexive: the census that enforces witness routing is
   itself enrolled with a firing negative.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — anchor-miss synthetic
   (formerly-default arm), per-arm inspected-site-empty synthetics, presence-shaped
   registry-entry census negative.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
