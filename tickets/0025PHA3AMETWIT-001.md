# 0025PHA3AMETWIT-001: Executable meta-witness discipline — eliminate assert-token-count and anchor-presence witnesses

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` test guards (`crates/tracewake-core/tests/anti_regression_guards.rs`) and `docs/3-reference/01_DESIGN_RISK_REGISTER.md`; no production crate code.
**Deps**: None

## Problem

Spec 0025 findings `ORD-HARD-166` (high), `ORD-HARD-175` (medium), `ORD-HARD-180`
(low): the 0024 meta-witness completion relocated the presence-check shape a third
time. `behavior_assertion_live_witness_count` returns
`usize::from(assertion_count > 0)` where `assertion_count` is
`body.matches("assert!").count() + body.matches("assert_eq!").count() +
body.matches("assert_absent(").count()` — textual presence of assertion macros, not
executed negatives — and `shared_scan_function_witness_count` (the un-cased
SharedScan fall-through) sums the same token counts. Roughly forty registry rows
route `MetaLockRouting::BehaviorAssertion`. Renaming a guard's internal anchor so
its violation set goes vacuously empty leaves its `assert!` macros textually
present, the witness at 1, and the suite green. Separately (`ORD-HARD-175`), the
cased perception witnesses count anchor-function presence
(`perception_visibility_other_emission_inspected_site_count` filters three named
functions by `function_body_if_present(…).is_some()`) or `visibility_default`
literal occurrences, while the violation detector is a per-line lexical scan that
does not partition by those functions — the detector can go vacuous with the
witness pinned. And (`ORD-HARD-180`) `test_body_has_structural_scan_shape`
recognizes only `_errors(`/`_violations(`, so an exempted test parking a real
inline scan (`.filter(…).count()` — the shape this file's own witnesses use)
bypasses covering-lock validation, and any rationale containing the literal
`"Historical acceptance-artifact anchor audit"` skips validation entirely.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `9e33d7a`: `behavior_assertion_live_witness_count`
   and `shared_scan_function_witness_count`
   (`crates/tracewake-core/tests/anti_regression_guards.rs`) carry the
   `matches("assert!")`-count shape; `perception_visibility_other_emission_inspected_site_count`
   filters by `function_body_if_present(…).is_some()`;
   `test_body_has_structural_scan_shape` is
   `body.contains("_errors(") || body.contains("_violations(")`. All
   operator-verified per spec 0025 §8.
2. Verified against spec 0025 §4 (`ORD-HARD-166`/`175`/`180`) and
   `docs/3-reference/01_DESIGN_RISK_REGISTER.md` R-29; the 0024 predecessor repair
   history (`archive/specs/0024_*.md` `ORD-HARD-141`/`146`/`155`) confirms this is
   the third consecutive relocation of the same shape — the repair must change the
   witness *mechanism*, not enumerate more cases.
3. Shared boundary under audit: the `META_LOCK_REGISTRY` ↔ witness-routing
   contract — every enrolled lock's witness value must originate in executed
   detection (a fired negative or an inspected-site count returned from the scan
   body), never in source-text token presence.
4. Rule restated before trusting the narrative (R-29; the adopted who-guards-the-guards
   research posture, spec 0025 §2): second-order assurance is one committed
   known-bad fixture per guard that provably fires — executed negatives, not a
   third meta-layer; a scan that inspected zero sites is a failure.
5. Enforcement surface touched: the meta-lock tier is the
   enforcement-of-enforcement for the repo's fail-closed and anti-contamination
   gates. This ticket strengthens witness measurement and weakens nothing: no
   violation predicate is relaxed; no actor-knowledge or replay surface is touched.
6. Adjacent-contradiction classification (spec 0025 §9 risk): executed-negative
   routing may be expensive across ≈40 rows — the sanctioned fallback is
   conversion to SharedScan with a measured inspected-site count, never back to
   token presence; a row whose negative cannot execute in-process gets a
   rationale-bearing exemption validated by the repaired `ORD-HARD-180` detector.
   Any guard that fails once witnesses are live is triaged inside this ticket as a
   potential masked defect first, per the Enforcement reading.

## Architecture Check

1. Routing BehaviorAssertion rows through actual negative execution (and SharedScan
   rows through inspected-site counts returned from the scan body) is cleaner than
   banning more token strings: the 0023→0024→0025 history proves string bans chase
   relocations, while an executed negative cannot be satisfied by any textual
   shape. The witness-shape ban on token counting is retained only as a backstop
   with its own firing negative.
2. No backwards-compatibility aliasing/shims: `shared_scan_function_witness_count`
   and the assert-token counting are removed, not kept as fallbacks; no row keeps a
   legacy witness path.

## Verification Layers

1. R-29 lock durability (`ORD-HARD-166`) → present-but-vacuous synthetic: a
   BehaviorAssertion guard whose violation set is emptied while its `assert!`
   macros stay textually intact must drop its witness below `witness_min`
   (test in `anti_regression_guards.rs`).
2. Witness-shape backstop → codebase grep-proof: zero `matches("assert!")`-style
   token counting remains in `meta_lock_live_witness_count`'s helpers; the ban
   carries a firing negative.
3. Anchor-presence repair (`ORD-HARD-175`) → per repaired arm, an
   inspected-site-empty synthetic (anchor functions/markers intact, inspected-line
   set emptied) must drop the witness below minimum.
4. Exemption-detector repair (`ORD-HARD-180`) → an inline-scan exemption
   (`.filter(…).count()` body) without a covering `lock_id` must fail
   `meta_lock_census_exemption_errors`; the literal-rationale bypass is gone
   (grep-proof: zero `"Historical acceptance-artifact anchor audit"` early-returns).
5. Whole-tier regression → `cargo test -p tracewake-core --test
   anti_regression_guards` plus the full four-gate workspace run.

## What to Change

### 1. Executed-negative routing for BehaviorAssertion rows (`ORD-HARD-166`)

Replace `behavior_assertion_live_witness_count`'s token counting: each
`MetaLockRouting::BehaviorAssertion` row's witness is the actual execution of the
row's negative (the known-bad fixture driven through the production scan/predicate,
asserted to yield a nonzero violation/failure signal at witness time), or the row is
re-routed to a SharedScan whose witness is the violation predicate's inspected-site
count returned from the scan body. Delete `shared_scan_function_witness_count`;
every SharedScan routes to a measured count. Rows whose negatives cannot execute
in-process get rationale-bearing exemptions validated by the repaired detector
(change area 3).

### 2. Inspected-site witnesses for the cased perception arms (`ORD-HARD-175`)

`perception_visibility_other_emission_inspected_site_count` and the
`visibility_default` literal counter are replaced by counts of sites the violation
predicate actually inspected, returned from
`perception_visibility_prose_branch_violations` itself (lines fed to the per-line
scan). Sweep the remaining cased arms for the same anchor-presence shape while in
the file; repair any sibling found and record each in implementation notes.

### 3. Broaden the exemption scan-shape detector; remove the literal-rationale bypass (`ORD-HARD-180`)

`test_body_has_structural_scan_shape` additionally recognizes inline violation
derivation (`.filter(` + `.count()`, `*_count(`, `matches(`); the
`"Historical acceptance-artifact anchor audit"` unconditional early-return is
replaced by a structural assertion on those entries.

### 4. Firing negatives

Add: (a) the present-but-vacuous BehaviorAssertion synthetic; (b) one
inspected-site-empty synthetic per repaired cased arm; (c) the witness-shape-ban
negative (a token-count witness helper must be rejected); (d) the inline-scan
exemption negative.

### 5. Risk-register extensions (spec 0025 §6)

Extend `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: R-29 gains the
*assertion-token-count witness* (`ORD-HARD-166`) and *prose-keyed taint*
(`ORD-HARD-167`) shapes; R-28's Watch note gains the *fingerprint-payload pitfall*
(`ORD-HARD-170`) and the *positional guard* (`ORD-HARD-174`). (Shapes are recorded
as observed at audit; their locks land in sibling tickets 002/004/006.)

### 6. Triage honest failures

Any scan that fails once its witness is live is investigated as a masked defect
before its anchor is adjusted (Assumption item 6).

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- The perception kill-set provenance repair (`ORD-HARD-167`) — ticket
  `0025PHA3AMETWIT-002` (this ticket only records its R-29 shape).
- New locks introduced by sibling tickets (they enroll under the repaired routing
  but land in their own tickets).
- Any production-crate change; any doctrine amendment.

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: no `matches("assert!")`/`matches("assert_eq!")`-style token counting
   remains in any witness helper; `shared_scan_function_witness_count` is deleted.
   Every `MetaLockRouting::BehaviorAssertion` row (≈40 at decomposition time —
   re-count from `META_LOCK_REGISTRY` at implementation, do not hardcode) routes
   through executed-negative or measured-count witnesses.
2. The present-but-vacuous synthetic fails via the live witness; each repaired
   cased arm fails its inspected-site-empty synthetic; the inline-scan exemption
   negative fires; the literal-rationale bypass is gone.
3. `cargo test -p tracewake-core --test anti_regression_guards` and the four gates
   (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. No `META_LOCK_REGISTRY` entry's witness can be satisfied by source-text token
   presence — only by the detection path executing.
2. The exemption census cannot be satisfied by a parked inline scan or a magic
   rationale string; every scan-shaped exemption names a covering, present
   `lock_id`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — present-but-vacuous
   BehaviorAssertion synthetic; per-arm inspected-site-empty synthetics;
   witness-shape-ban negative; inline-scan exemption negative.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Implemented in `crates/tracewake-core/tests/anti_regression_guards.rs` and
`docs/3-reference/01_DESIGN_RISK_REGISTER.md`.

- Deleted `behavior_assertion_live_witness_count`,
  `ticket_owned_debt_live_witness_count`, and
  `shared_scan_function_witness_count` (the assert-token-count and presence
  shapes). Every `BehaviorAssertion` and `TicketOwnedDebt` row now routes through
  `behavior_assertion_inspected_site_count` — an explicit per-lock match whose
  arms return measured driftable-population counts (guarded-source-set sizes,
  checksum-coverage lengths, runtime registry sizes, production-anchor match
  counts) or executed negatives (`checksum_parity_errors` /
  `direct_duration_need_delta_construction_violations` /
  `acceptance_checklist_anchor_errors` driven against known-bad fixtures); an
  unenrolled new row falls to `_ => 0` and fails its witness floor, forcing an
  explicit routing decision. The former SharedScan fall-through rows
  (`checksum_coverage_is_total_for_authoritative_state`, both acceptance-artifact
  guards, `guard_006_duration_need_deltas_route_through_shared_emitter`) gained
  explicit measured/executed arms; the fall-through now returns `0`.
- Repaired the `ORD-HARD-175` anchor-presence witnesses: both perception arms now
  return `perception_prose_scan_inspected_line_count` (non-comment lines the
  prose-branch scan actually examines);
  `perception_visibility_other_emission_inspected_site_count` (function-presence
  counting) is deleted. Sibling sweep: the `guard_014_embodied_projection_source…`
  token-presence arm and the `typed_column_closure_exemptions…` anchor arm were
  converted to measured counts; the dead early
  `typed_column_closure_payload_alias…` arm (shadowing the measured arm below it)
  was deleted; the three `EPISTEMIC_PROJECTION_RS` policy arms counting
  `assert!` tokens now count production mechanism anchors
  (`includes_in_embodied_context(`, `is_latest_current_place_record`,
  `supersede_workplace_subject` — the original `supersede_newest_by_subject`
  anchor was a test-name token with zero production matches, caught honestly by
  the live witness floor and corrected). Assessed, not repaired (rationales): the
  `event_apply_remains…` arm counts companion-guard invocation sites (the
  detector executions for that meta-row); `guard_011_no_human_day_runner…`
  counts the discrimination sites in the scanned fixture-run source; the
  `map_or(contains)` arms for the hidden-truth-gates family count
  detector-invocation sites in their host files.
- Broadened `test_body_has_structural_scan_shape` to recognize inline violation
  derivation (`_count(`, `.filter(`+`.count()`, `.matches(`+`).count()`) and
  replaced the `"Historical acceptance-artifact anchor audit"` literal-rationale
  bypass with a structural assertion (the exempted body must run
  `acceptance_checklist_anchor_errors`). Honest-failure triage under the
  broadened detector: `workspace_dependency_posture_matches_allowlist` was a
  genuine parity census parked in an exemption — it is now enrolled in
  `META_LOCK_REGISTRY` (negative `synthetic_unlisted_workspace_dependency`,
  measured witness `WORKSPACE_DEPENDENCY_ALLOWLIST.len()`) with a firing
  unlisted-dependency synthetic, and its exemption is deleted.
- Added the firing negatives: the present-but-vacuous BehaviorAssertion synthetic
  (vacuated `witness_kind_allowed` body drops the witness below minimum while the
  guard's `assert!` macros stay textually intact); the inspected-line-empty
  perception synthetic (comment-only source, anchors textually present); the
  `witness_shape_ban_errors` predicate applied to both witness-function bodies
  with a firing assertion-token-count synthetic; the inline-scan exemption
  negative through the broadened detector.
- Extended R-29 with the *assertion-token-count witness* and *prose-keyed taint*
  shapes and R-28's Watch note with the *fingerprint-payload pitfall* and
  *positional guard* shapes (spec 0025 §6).

Deviations: none. One witness anchor premise (`supersede_newest_by_subject` as a
production token) was refuted during implementation and corrected to the real
production mechanism token, recorded above per the spec §8 refute-don't-drop rule.

Enumerated-criterion dispositions:

- Assert-token-count routing eliminated (≈40 BehaviorAssertion rows re-routed,
  re-counted at implementation as 39 + 1 TicketOwnedDebt): completed.
- `ORD-HARD-175` cased perception witnesses: completed (plus the sibling sweep
  recorded above).
- `ORD-HARD-180` detector broadening + bypass removal: completed (one parked
  census enrolled).
- Firing negatives (present-but-vacuous, inspected-line-empty, witness-shape ban,
  inline-scan exemption): completed.
- Deferred or dropped members: none.

Verification:

- `cargo test -p tracewake-core --test anti_regression_guards` — passed (79).
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
