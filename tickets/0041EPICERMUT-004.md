# 0041EPICERMUT-004: Kill `observation.rs` `Confidence` survivors — numeric value and low-classification

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — behavior-witness tests by default; conditional production correction in `crates/tracewake-core/src/epistemics/observation.rs` (and its evidence/view consumer) only if a survivor reveals a real defect or a redundant API must be reconciled
**Deps**: None

## Problem

Spec 0041 §5.5 routes three EPI mutation survivors on `Confidence` in `crates/tracewake-core/src/epistemics/observation.rs` to the projection/replay observation-confidence diagnostics. They are a seed floor (§3.6), killed only by a downstream EPI consequence, not by a direct getter/classifier assertion (§4.9).

**§5.5 — observation confidence (3 identities):**
- `replace Confidence::parts_per_thousand -> u16 with 0`
- `replace Confidence::parts_per_thousand -> u16 with 1`
- `replace Confidence::is_low -> bool with true`

Per §9.5, `serialize_canonical` reads the private numeric field directly (`format!("{:04}", self.0)`) and the projection checksum/debug entries call `serialize_canonical` — so those paths execute **neither** surviving accessor, and a checksum assertion cannot kill these mutants. The kill requires a real production consumer of `parts_per_thousand` / `is_low`, exercised across the contract including the exact 350/351 low-threshold boundary (`is_low` is `self.0 <= 350`) and nontrivial values, or a reviewed dead-API/equivalence disposition.

## Assumption Reassessment (2026-06-19)

1. Codebase check: `Confidence(u16)` with `MIN = 0`, `MAX = 1000`, `new(parts_per_thousand) -> Result<Self, ConfidenceError>` rejecting `> 1000` (`observation.rs:10`–`:22`); `parts_per_thousand(self) -> u16` (`:24`) and `is_low(self) -> bool { self.0 <= 350 }` (`:28`); `serialize_canonical` formats `{:04}` from the private field (`:32`), bypassing both accessors. `ConfidenceError::OutOfRange` exists. The spec's `:25/:29` line numbers are cargo-mutants identities; the verified symbols are authoritative.
2. Specs/docs check: §5.5 requires enumerating every final production call site of both accessors, then — where the EPI contract requires the raw value or low/non-low classification to affect a production surface — exercising at least three valid values spanning the contract (including 350 and 351 plus a nontrivial high value), with a replay/provenance-backed consequence (typed confidence class in an evidence record, notebook/debug/view diagnostic, belief/contradiction policy, or proposal consequence). Use nontrivial values such as 250 and 875 so the `0` and `1` replacements are distinguishable; the `is_low -> true` mutant requires a high-confidence control whose downstream classification stays non-low. A direct round-trip/classifier/checksum assertion is supporting unit evidence only.
3. Cross-artifact shared boundary under audit: `Confidence` accessors ↔ their production consumer (evidence record / view-model diagnostic / belief-or-contradiction policy / proposal) ↔ replay reproduction. The audit must find or doctrinally establish that consumer at the final commit, or reconcile a genuinely redundant API.
4. Motivating invariants (INV restate): §10 maps EPI-03 to `INV-021`/`INV-024`/`INV-026` (epistemic currency is typed; no telepathy; freshness/source attached) plus `INV-018` (deterministic replay of serialized confidence). The kill must witness a typed-confidence consequence, not a literal accessor value.
5. Fail-closed / actor-knowledge / replay surface: the enforcement surface is observation insertion + the confidence-consuming projection/evidence/view path. Confirm invalid values outside the bounded domain stay rejected (`ConfidenceError::OutOfRange`), hidden truth cannot upgrade confidence, and debug may display confidence but may not feed it back into actor-known behavior (§5.5 negative controls) — no leakage, deterministic replay of the confidence value/classification preserved.
6. Schema extension (additive-vs-breaking): if the kill requires surfacing a typed confidence class into a serialized evidence record or view-model diagnostic that currently omits it, that is an **additive** extension of that evidence/view schema. Consumers: replay rebuild (if the value enters the canonical input) and the consuming view/diagnostic. It must stay additive and deterministic; if no production consumer is doctrinally required, no schema change is made and the redundant accessor is reconciled/removed instead (recorded in the triage register, ticket 009). This populates/extends an existing typed value rather than restructuring `Confidence` itself — no breaking change to the `Confidence` shape.

## Architecture Check

1. Driving a real confidence consumer across the 350/351 boundary with nontrivial values (250, 875) is cleaner than a `parts_per_thousand()` round-trip or `is_low()` assertion: it ties the kill to the EPI decision confidence exists to inform and makes the `0`, `1`, and `is_low -> true` mutants individually distinguishable, which a single low-value assertion cannot. The high-confidence non-low control is what kills `is_low -> true`.
2. No backwards-compatibility aliasing/shims: no test-only consumer, no arbitrary new UI label invented to raise the mutation count, no `#[mutants::skip]`. A redundant accessor (if found) is removed with semantic reconciliation.

## Verification Layers

1. INV-021/024/026 (typed confidence, no telepathy, freshness) -> replay/golden-fixture check: a confidence-bearing observation drives a typed downstream consequence reproduced by replay.
2. INV-018 (deterministic replay) -> replay/golden-fixture check: live application and rebuild agree on the confidence value/classification in the consuming surface.
3. Bounded-domain fail-closed -> schema validation: out-of-range confidence is rejected by `Confidence::new`; hidden truth cannot upgrade it.
4. Member-level mutant catch -> `cargo mutants -f crates/tracewake-core/src/epistemics/observation.rs` reports all three identities `caught` (or a reviewed §4.11 disposition).

## What to Change

### 1. Census the confidence consumers and establish the witness

Enumerate every final production call site of `parts_per_thousand` and `is_low`. Where the EPI contract requires the value/classification to affect a production surface, establish that consumer and exercise at least three valid values spanning the contract — including 350 (low) and 351 (non-low) and a nontrivial high value (e.g. 875), plus a nontrivial low value (e.g. 250) — proving a replay/provenance-backed consequence. The `0` and `1` numeric replacements must each be distinguishable from the unmutated value; the `is_low -> true` replacement must be killed by a high-confidence control whose downstream classification remains non-low. If the final code has no real consumer for one or both accessors, do not invent one: establish the doctrinally required consumer in the existing EPI seam, remove/narrow the genuinely redundant API with reconciliation, or present the rare §4.11 exception with complete final-domain/call-site proof and independent signoff (recorded in ticket 009's register).

### 2. Negative/contamination controls

Invalid values outside the bounded confidence domain stay rejected; hidden truth may not upgrade confidence; debug may display confidence but may not feed it back into actor-known behavior.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify — confidence-consequence replay witness across the 350/351 boundary, as surfaced)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — typed-confidence evidence/diagnostic consequence, as surfaced)
- `crates/tracewake-tui/tests/transcript_snapshot.rs` (modify — only where confidence is already user-visible)
- `crates/tracewake-core/src/epistemics/observation.rs` (modify — only if a survivor reveals a real defect or a redundant accessor must be reconciled/removed)
- `crates/tracewake-core/src/view_models.rs` (modify — only if a typed confidence class must be surfaced in a view-model diagnostic to establish the consumer)

## Out of Scope

- Killing survivors in `belief.rs`, `contradiction.rs`, or `proposition.rs` (tickets 002, 003, 005–008).
- Routing confidence through an arbitrary new UI label to improve the mutation count (§4.12).
- Adding any confidence survivor to `.cargo/mutants-baseline-misses.txt` (§4.3).
- The full standing campaign / triage register (ticket 009) and EPI-01…11 re-proof / acceptance artifact (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants -f crates/tracewake-core/src/epistemics/observation.rs`:
   - `parts_per_thousand -> 0` — `caught` (nontrivial value diverges in the consumer).
   - `parts_per_thousand -> 1` — `caught` (distinguishable from the unmutated value).
   - `is_low -> true` — `caught` by the high-confidence non-low control (or a recorded §4.11 disposition with full-domain/call-site proof and independent signoff).
2. Witnesses pass unmutated and fail under each active mutant; member-level evidence retained per identity; the 350/351 boundary is exercised explicitly.
3. `cargo test --workspace --locked` — clean baseline.

### Invariants

1. No witness asserts `parts_per_thousand`/`is_low` in isolation; each observes a typed-confidence downstream consequence reproduced by replay.
2. Out-of-range confidence stays rejected; hidden truth cannot upgrade confidence; debug confidence cannot feed actor-known behavior.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — confidence-bearing observation driving a replay-reproduced typed consequence across 250/350/351/875.
2. `crates/tracewake-core/tests/acceptance_gates.rs` / `crates/tracewake-tui/tests/transcript_snapshot.rs` — the evidence/diagnostic or user-visible confidence consequence, as surfaced by the final consumer.

### Commands

1. `cargo mutants -f crates/tracewake-core/src/epistemics/observation.rs`
2. `cargo test --workspace --locked`
3. `cargo mutants -f <file>` is the correct per-ticket boundary: it regenerates this file's three mutants so each identity's catch/disposition is provable before the full campaign (ticket 009) reconciles them.
