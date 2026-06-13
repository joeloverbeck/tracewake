# 0025PHA3AMETWIT-002: Provenance-keyed perception kill set — taint by argument derivation, not helper-body prose

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` test guards (`crates/tracewake-core/tests/anti_regression_guards.rs`); no production crate code.
**Deps**: 0025PHA3AMETWIT-001

## Problem

Spec 0025 finding `ORD-HARD-167` (high): the `ORD-HARD-149` kill set is vacuous for
the provenance machinery it was built to prove. Both kill-set synthetics fire via
pre-existing raw-token rules — `renamed_parameter_helper_synthetic`'s asserted
violation line contains `place.display_label` (raw `display_label` rule) and its
helper body contains `.contains("hidden")`; `payload_value_relay_synthetic`'s
asserted line contains `.contains("hidden")` (raw hidden-prose rule). Worse,
`perception_sensitive_helper_params` marks a helper parameter sensitive only when
the helper's own body line carries `.contains("hidden")`/`.to_lowercase()`/
`.to_ascii_lowercase()` — taint keyed to *prose tokens in the helper body*, not to
*argument data provenance*. The exact evasion `ORD-HARD-149` cited —
`fn gate(label: &str) -> bool { label.starts_with("vault") }` fed a relayed
display-label binding — carries no scanned token in any line: the helper is never
marked sensitive, the call site is not flagged, and the laundering hole remains
open while the kill set certifies it closed.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `9e33d7a`: the two kill-set synthetics and
   `perception_sensitive_helper_params`'s prose-token keying
   (`crates/tracewake-core/tests/anti_regression_guards.rs`) confirmed verbatim;
   the provenance helpers (`perception_tainted_let_alias`,
   `branches_on_tainted_binding`,
   `line_calls_sensitive_helper_with_tainted_argument`) exist but are never
   load-bearing for any firing assertion. Operator-verified per spec 0025 §8.
2. Verified against spec 0025 §4 (`ORD-HARD-167`) and the archived predecessor
   (`archive/specs/0024_*.md` `ORD-HARD-149` required correction: "track bindings
   derived from `display_label`/`*_id` projections across `let` and parameter
   boundaries"); `docs/3-reference/01_DESIGN_RISK_REGISTER.md` R-29 (the ticket
   0025PHA3AMETWIT-001 §6 extension records this prose-keyed-taint shape).
3. Shared boundary under audit: the perception channel's INV-022 lexical firewall
   (`perception_visibility_prose_branch_violations` over
   `crates/tracewake-core/src/agent/perception.rs`) — the scan must flag prose/ID
   branching reached through laundered bindings, with the typed `PayloadField`
   write as the only sink.
4. Invariant restated before trusting the narrative (INV-022 — raw prose is not
   authoritative state): prose may render claims; it may not define hidden facts or
   gate perception visibility. The scan exists to keep prose discriminators out of
   the perception channel regardless of how the value is relayed.
5. Enforcement surface touched: this ticket strengthens the INV-022 enforcement
   scan (provenance-aware taint) and weakens nothing — no production perception
   code changes, no actor-knowledge filtering or replay surface is touched; the
   raw-token rules are retained alongside the provenance path.

## Architecture Check

1. Keying taint to argument data provenance (a call site passing an expression
   derived from `display_label`/`*_id` projections, tracked through the existing
   `let`-alias machinery) is cleaner than widening the prose-token list: token
   lists chase spellings (the 0023→0024 history of this scan), while provenance
   tracks the value itself — a helper named anything, branching on anything,
   becomes sensitive the moment a tainted argument reaches it.
2. No backwards-compatibility aliasing/shims: the prose-keyed sensitivity rule is
   replaced by the provenance rule (raw-token rules remain as independent
   detectors, not as a fallback for the provenance path).

## Verification Layers

1. INV-022 lock durability → provenance-only kill-set member: a synthetic whose
   branching lines carry zero raw tokens (no `display_label`, no `hidden`, no
   lowercase calls — e.g. the `starts_with("vault")` helper fed a relayed binding)
   must fail the scan solely via
   `line_calls_sensitive_helper_with_tainted_argument`.
2. Rule-family attribution → a guard asserting each kill-set member's asserted
   violation names the rule family that produced it, proving the provenance path is
   load-bearing (not satisfied by raw-token co-firing).
3. Witness discipline → the scan's registry entry carries the measured
   inspected-site witness under the 0025PHA3AMETWIT-001 repaired routing.
4. Whole-tier regression → `cargo test -p tracewake-core --test
   anti_regression_guards` plus the four-gate workspace run.

## What to Change

### 1. Provenance-keyed sensitivity

Rework `perception_sensitive_helper_params` (and the call-site rule it feeds): a
helper parameter is sensitive when any call site passes an expression derived —
via the existing `perception_tainted_let_alias` tracking, extended across parameter
boundaries within the file — from `display_label`/`*_id` projections, regardless of
the helper body's tokens. The typed `PayloadField` write remains the only exempted
sink. Raw-token rules stay as independent detectors.

### 2. Provenance-only kill-set member

Add the spec-cited evasion as a kill-set synthetic: a helper whose body is
`candidate.starts_with("vault")` (no prose tokens), called with a binding relayed
from `display_label`, asserted to fire — and asserted to fire *only* through the
provenance rule (raw rules disabled for this synthetic source, or attribution
checked per change area 3).

### 3. Rule-family attribution

Violations carry (or are queryable for) the rule family that produced them; each
kill-set member asserts its expected family, so a future refactor cannot silently
shift a provenance proof onto a raw-token co-match.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Production changes to `crates/tracewake-core/src/agent/perception.rs` (the scan
  is the deliverable; if the provenance scan surfaces a real laundering site in
  production perception code, that is triaged per the Enforcement reading as a
  masked defect inside this ticket — none is known at decomposition).
- The cased-witness and exemption-detector repairs (`ORD-HARD-175`/`180`) — ticket
  `0025PHA3AMETWIT-001`.
- Any doctrine amendment.

## Acceptance Criteria

### Tests That Must Pass

1. The provenance-only kill-set member (zero raw tokens in branching lines) fails
   the scan via `line_calls_sensitive_helper_with_tainted_argument`; the two
   existing kill-set members still fail.
2. Rule-family attribution asserts each member's expected family; disabling the
   provenance rule (synthetically) makes the provenance-only member pass —
   proving it is load-bearing.
3. `cargo test -p tracewake-core --test anti_regression_guards` and the four gates
   (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. A prose/ID discriminator reached through any same-file binding or parameter
   relay is flagged by the perception scan; the typed `PayloadField` write is the
   only sink.
2. Every kill-set member's proof names the rule family that produced it; no member
   is provable by raw-token co-firing alone.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — provenance-only
   kill-set synthetic; rule-family attribution assertions; provenance-rule-disabled
   discriminating negative.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-13

Implemented in `crates/tracewake-core/tests/anti_regression_guards.rs`.

- Reworked `perception_sensitive_helper_params` so helper sensitivity is derived
  from tainted same-file call-site arguments plus branch-shaped helper parameters,
  not from `.contains("hidden")` / lowercase tokens in the helper body.
- Added rule-family attribution to
  `perception_visibility_prose_branch_violations`; the kill-set assertions now
  prove whether a failure came from
  `line_calls_sensitive_helper_with_tainted_argument`,
  `branches_on_tainted_binding`, or the raw branch families.
- Added the spec-cited provenance-only `gate(label)` synthetic, where the helper
  branches with `starts_with("vault")` and the call argument is a relayed
  `display_label` alias. The synthetic fails through the tainted-argument rule
  and passes when the provenance rule is disabled.
- Narrowed the raw `display_label` branch detector so plain provenance aliases
  are not themselves treated as violations, while direct comparisons and string
  branch methods remain rejected.

Deviations: none.

Enumerated-criterion dispositions:

- Provenance-only kill-set member: completed.
- Existing renamed-parameter and payload-relay kill-set members: completed with
  explicit rule-family assertions.
- Provenance-rule-disabled discriminating negative: completed.
- Deferred or dropped members: none.

Verification:

- `cargo test -p tracewake-core --test anti_regression_guards` — passed (79).
- `cargo fmt --all --check` — passed after applying formatting.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
