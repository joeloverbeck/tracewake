# 0041EPICERMUT-003: Kill `contradiction.rs` survivors — expected-absence eligibility and contradiction activity

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — behavior-witness tests by default; conditional production correction in `crates/tracewake-core/src/epistemics/contradiction.rs` only if a survivor reveals a real defect or the redundant-method-removal disposition is taken
**Deps**: None

## Problem

Spec 0041 §5.3–§5.4 route two EPI mutation survivors in `crates/tracewake-core/src/epistemics/contradiction.rs` to the projection/replay absence-contradiction and contradiction-state diagnostics. They are a seed floor (§3.6), killed only by a certified EPI consequence reached through the consumer, not a direct boolean assertion (§4.9).

**§5.3 — expected-absence eligibility (1 identity):**
- `replace || with && in detect_expected_absences`

**§5.4 — contradiction activity (1 identity):**
- `replace ContradictionKind::is_active -> bool with true`

For §5.3, the eligibility guard is `belief.holder() != &HolderKind::Actor(holder_actor_id) || belief.stance() != Stance::ExpectsTrue` (`contradiction.rs:~127`): the `||`→`&&` mutant is killed only if **both** one-condition mismatch rows are retained and demonstrably fail. For §5.4, the final enum domain is a single active variant — `ContradictionKind::ExpectedItemAbsentFromContainer`, whose `is_active` returns `matches!(self, ExpectedItemAbsentFromContainer)` = `true` (`contradiction.rs:14`, `:27`). This makes the `-> true` mutant **plausibly equivalent today but not established** (§9.6); the disposition is a spec-assigned implementer choice (§5.4), recorded in the triage register (ticket 009) and acceptance artifact (ticket 010).

## Assumption Reassessment (2026-06-19)

1. Codebase check: `detect_expected_absences(...)` exists at `contradiction.rs:114` with the eligibility guard `belief.holder() != &HolderKind::Actor(holder_actor_id.clone()) || belief.stance() != Stance::ExpectsTrue` (the `||` the mutant flips). `ContradictionKind` (`contradiction.rs:14`) has exactly one variant `ExpectedItemAbsentFromContainer`; `is_active(self) -> bool` (`contradiction.rs:27`) returns `matches!(self, ContradictionKind::ExpectedItemAbsentFromContainer)`. Fixture `expectation_contradiction_001` exists (`tracewake-content/src/fixtures/expectation_contradiction_001.rs`) with hidden-container vocabulary (`container_contents_observed`). The spec's `:127/:28` line numbers are cargo-mutants identities; the verified symbols above are authoritative.
2. Specs/docs check: §5.3 requires exercising `detect_expected_absences` through the evented observation/projection path with the full two-condition eligibility matrix (holder-match × stance-match), both one-mismatch rows retained and failing under the mutant, the positive row naming prior belief / contradicting observation / expected & observed propositions / holder / source event / detection tick, and replay reproducing the same contradiction ID/linkage. §5.4 permits exactly two passing dispositions — kill through an existing distinguishable final-domain case, or prove §4.11 equivalence across the complete final enum domain and all reachable call sites with independent signoff — and forbids inventing an inactive kind solely to kill the mutant; a redundant-method removal is legitimate only if all call sites preserve doctrine and the historical gap is reconciled.
3. Cross-artifact shared boundary under audit: `detect_expected_absences` eligibility ↔ the typed contradiction it creates ↔ projection/replay linkage and view-model warning surface; and `ContradictionKind::is_active` ↔ every production call site that drives (or filters) the active-contradiction surface, diagnostics, planner blockers, notebook entries, or view-model warnings. The §5.4 disposition requires a complete final-domain + call-site census before choosing kill vs equivalence.
4. Motivating invariants (INV restate): §10 maps EPI-04 to `INV-016`/`INV-021`/`INV-023`–`INV-026`/`INV-030`/`INV-102` — absence becomes evidence only through expectation/perception/search; observation and contradiction links are typed and source-backed; no culprit/location is inferred from truth. The eligibility guard enforces holder + stance scoping; the kill must witness that scoping through the contradiction consumer, not by asserting the boolean.
5. Fail-closed / actor-knowledge / replay surface: the enforcement surface is expected-absence detection feeding the typed contradiction into projection/replay. Confirm the witness reaches the consumer (contradiction creation, replay linkage, view-model warning) and that hidden container truth does **not** create an absence contradiction until the actor receives the qualifying contents-observed event (§5.3 negative control), another actor's expectation stays private, and no debug-only classification manufactures an inactive case (§5.4 negative control). Replay must reproduce the same contradiction ID/linkage deterministically — no leakage, no nondeterminism.
6. Removal blast radius (conditional, §5.4): if the chosen disposition removes the redundant `ContradictionKind::is_active`, grep every consumer repo-wide before removal — at the final commit `is_active` is referenced in `crates/tracewake-core/src/epistemics/` and its projection/view-model consumers; each call site must be reconciled (the active-contradiction surface still behaves correctly) and the historical semantic gap explained. If the equivalence or kill disposition is taken instead, no removal occurs and this item records `no removal — disposition is <kill|equivalence>`.

## Architecture Check

1. Driving `detect_expected_absences` through the evented projection path with the complete eligibility matrix (and both one-mismatch controls) is cleaner than asserting the boolean directly: it proves the holder/stance scoping creates exactly the contradictions doctrine permits and none it forbids, which a direct `||`/`&&` test cannot. For §5.4, a complete final-domain + call-site census is the only honest way to distinguish a real distinguishing case from a true equivalence — "no test failed" is explicitly insufficient (§4.11, §12.1).
2. No backwards-compatibility aliasing/shims: no invented inactive `ContradictionKind` variant solely to kill the mutant, no `#[mutants::skip]`, no debug-only classification bridge. A redundant-method removal (if taken) deletes with reconciliation, not aliasing.

## Verification Layers

1. INV-016/030 (absence-as-evidence only through expectation/perception) -> replay/golden-fixture check: the positive eligibility row creates the expected-absence contradiction; both one-condition mismatch rows create none and fail under the `&&` mutant.
2. INV-024/026 (no telepathy; typed source-backed links) -> replay/golden-fixture check: hidden container truth creates no absence contradiction until the contents-observed event; another actor's expectation stays private.
3. INV-021/023 (typed contradiction state) -> manual review + `cargo mutants`: the §5.4 disposition (kill via distinguishable case, or §4.11 equivalence with full-domain/call-site proof + independent signoff) is recorded with reachable-call-site reasoning.
4. Member-level mutant catch -> `cargo mutants -f crates/tracewake-core/src/epistemics/contradiction.rs` reports both identities `caught` (or `is_active` carries the reviewed equivalence disposition).

## What to Change

### 1. Expected-absence eligibility matrix (§5.3)

Exercise `detect_expected_absences` through the evented observation/projection path with the full two-condition matrix: (holder matches, stance matches) → eligible, create the expected-absence contradiction when the item is absent; (yes,no), (no,yes), (no,no) → ineligible, no contradiction. Retain both one-mismatch rows so the `||`→`&&` mutant fails (under `&&`, an eligible row would require both mismatches, suppressing the positive contradiction and/or admitting an ineligible one). The positive row names the prior belief, contradicting observation, expected proposition, observed/missing proposition, holder, source event, and detection tick; replay reproduces the same contradiction ID/linkage. Prefer reusing the `expectation_contradiction_001` and hidden-container fixture vocabulary over an isolated helper test.

### 2. Contradiction activity disposition (§5.4)

Enumerate every final-commit `ContradictionKind` variant and every production call site of `is_active`. Then take exactly one passing disposition and record it (register, ticket 009):
- **Kill** through an already-existing distinguishable domain case discovered at the final commit (a public behavior matrix proving inactive contradictions do not drive the active-contradiction surface/diagnostics/blockers/notebook/view warnings while active kinds do), reaching the consumer rather than asserting the helper; or
- **§4.11 equivalence** across the complete final enum domain and all reachable call sites, explaining why returning `true` is extensionally identical today, with independent reviewer signoff; or
- a **redundant-method removal** only if all call sites preserve doctrine and the historical semantic gap is reconciled (AR item 6 blast radius).
Inventing an inactive contradiction kind solely to kill this mutant is forbidden.

### 3. Negative/contamination controls

§5.3: hidden container truth must not create an absence contradiction until the actor receives the qualifying contents-observed event; another actor's expectation stays private. §5.4: no debug-only classification or hidden truth may manufacture an inactive case for embodied behavior.

## Files to Touch

- `crates/tracewake-core/tests/golden_scenarios.rs` (modify — expected-absence eligibility matrix via golden path, as surfaced)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — typed contradiction creation/linkage consequence, as surfaced)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify — replay reproduction of contradiction ID/linkage, as surfaced)
- `crates/tracewake-core/src/epistemics/contradiction.rs` (modify — only if a survivor reveals a real defect or the redundant-method-removal disposition is taken)

## Out of Scope

- Killing survivors in `belief.rs`, `observation.rs`, or `proposition.rs` (tickets 002, 004–008).
- Inventing a new inactive `ContradictionKind` variant or any test-only branch to kill `is_active` (§4.12, §5.4).
- Adding any contradiction survivor to `.cargo/mutants-baseline-misses.txt` (§4.3).
- The full standing campaign / triage register (ticket 009) and EPI-01…11 re-proof / acceptance artifact (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants -f crates/tracewake-core/src/epistemics/contradiction.rs`:
   - `replace || with && in detect_expected_absences` — `caught` (both one-condition mismatch rows fail under the mutant).
   - `replace ContradictionKind::is_active -> bool with true` — `caught` via a distinguishing case, OR a recorded §4.11 equivalence disposition with complete final-domain + call-site proof and independent signoff (a final `missed` tool outcome reported as reviewed-equivalent, never counted as caught — §4.8).
2. The eligibility-matrix witness passes unmutated and fails under the active mutant; the positive and both one-mismatch rows are retained with member-level evidence.
3. `cargo test --workspace --locked` — clean baseline.

### Invariants

1. No witness asserts `is_active` or the eligibility boolean directly; each reaches the contradiction consumer (creation, replay linkage, view-model warning).
2. Hidden container truth creates no absence contradiction before the contents-observed event; replay reproduces the contradiction ID/linkage deterministically.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/golden_scenarios.rs` / `crates/tracewake-core/tests/acceptance_gates.rs` — evented expected-absence eligibility matrix (4 rows × creation consequence) reusing `expectation_contradiction_001`/hidden-container vocabulary.
2. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — replay reproduction of the positive row's contradiction ID/linkage; and, for the §5.4 kill disposition, the active/inactive contradiction-surface behavior matrix (omitted if the equivalence disposition is taken).

### Commands

1. `cargo mutants -f crates/tracewake-core/src/epistemics/contradiction.rs`
2. `cargo test --workspace --locked`
3. `cargo mutants -f <file>` is the correct per-ticket boundary: it regenerates this file's two mutants so each identity's catch/disposition is provable before the full campaign (ticket 009) reconciles them.
