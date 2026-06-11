# 0020PHA3ACOGSUR-003: Mutation-perimeter completion and baseline governance

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, `.github/workflows/ci.yml`, perimeter guard in `anti_regression_guards.rs`; `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` rows; no production-code changes unless baseline triage warrants tests
**Deps**: `archive/tickets/0020PHA3ACOGSUR-001.md`, `archive/tickets/0020PHA3ACOGSUR-002.md` (honest baseline refresh after the new tests exist, spec §8); `archive/specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md` (ORD-HARD-055, ORD-HARD-056)

## Problem

Two residues of 0019's `ORD-HARD-045`. First (`ORD-HARD-055`): `actions/defs/eat.rs`
(~465 lines, ~15 branch/return sites, reads authoritative `state.food_supplies`
under `TRUTH_ACCESSOR_ALLOWLIST`) is outside the mutation perimeter — absent from
the scheduled `-f` list, the in-diff grep alternation
(`actions/defs/(sleep|work)\.rs`), and `MUTATION_PERIMETER_DURATION_DEFS` — under
the blanket `CORE_ACTION_RATIONALE` classification asserting mutation coverage that
eat does not have; 0019's own correction said "and `eat.rs` if nontrivial" and the
conditional was silently dropped (spec 0020 resolved it: nontrivial). The
consistency guard verifies only the sleep/work subset, so silently dropping the
`agent/`, `scheduler`, `projections`, or `pipeline.rs` filters would pass it, and
nothing proves the in-diff regex matches a representative guarded path. Second
(`ORD-HARD-056`): `.cargo/mutants-baseline-misses.txt` is a 148-entry accepted-miss
set spanning guarded-layer decision logic with no size, content-hash, or
per-entry-rationale governance — an appendable gate-silencing channel both CI jobs
`comm -23` against.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `96bc387`: `MUTATION_PERIMETER_DURATION_DEFS` lists exactly
   `defs/sleep.rs` + `defs/work.rs` (2 reference sites, both in
   `anti_regression_guards.rs` — full rename blast radius);
   `mutation_perimeter_consistency_violations` iterates only that list; the in-diff
   job's grep alternation covers `agent/`, `scheduler.rs`, `projections.rs`,
   `actions/pipeline.rs`, `actions/defs/(sleep|work).rs`; the scheduled job's `-f`
   filters mirror it; the baseline file is 148 lines; no test references the
   baseline filename. The in-diff job's loud-failure semantics and push-gap closure
   (`pull_request || push`, `HEAD^..HEAD`) verified holding — not re-fixed here.
2. Verified against spec 0020 (reassessed 2026-06-11): ORD-HARD-055/056; §8 orders
   this after `-001`/`-002` so the baseline refresh reflects the new tests; the
   perimeter + baseline conformance rows and the "pinned mutation baseline" wording
   correction land with this ticket per the approved distribution.
3. Cross-artifact boundary under audit: the mutation-gate contract spanning
   `.cargo/mutants.toml`, both CI job filter sets, the perimeter guard, the
   source-classification rationale strings, and the miss baseline — filter
   membership, classification claims, and baseline content must stay mutually
   consistent under CI enforcement.
4. Lock-durability rationale restated (spec §2; INV-091–098 posture): a gate whose
   perimeter excludes the code its rationale claims it covers, or whose baseline
   can absorb arbitrary misses unledgered, asserts more than it proves — the
   lineage's recurring relapse.
5. Conditional clause resolution carried from decomposition: "`eat.rs` if
   nontrivial" → **yes** (operator-verified nontrivial in spec 0020); recorded here
   so implementation cannot re-drop it.
6. Rename blast radius (`MUTATION_PERIMETER_DURATION_DEFS` → a name reflecting the
   wider set): grep-verified 2 sites, both `anti_regression_guards.rs`; no doc,
   spec, or skill references the constant.
7. Adjacent contradictions: triaging the 148 baseline entries may expose real test
   debt (missed mutants in guarded-layer interruption predicates / decision logic).
   Classification: entries warranting tests become **future cleanup as their own
   tickets** (spec §9 budgets for this); this ticket records dispositions and lands
   governance, not the full debt paydown.

## Architecture Check

1. Widening the guard to the full perimeter (all filters in both jobs) plus a regex
   canary (apply the in-diff alternation to each perimeter path; assert a match) and
   a baseline count/hash pin with a ledger turns three hand-maintained surfaces into
   mutually checking ones — the census pattern proven by `CONTENT_NEGATIVE_PROOFS`.
   The alternative (trusting review) is the posture that produced `ORD-HARD-055`.
2. No backwards-compatibility aliasing/shims: the constant is renamed at both sites,
   not aliased; eat's classification rationale is replaced with a true statement,
   not annotated around.

## Verification Layers

1. Perimeter completeness -> guard asserts `eat.rs` + `sleep.rs` + `work.rs` and the
   `agent/`/`scheduler`/`projections`/`pipeline.rs` filters present in BOTH the
   scheduled `-f` list and the in-diff alternation; synthetic case removing one
   filter fails.
2. Regex liveness -> canary test applying the in-diff alternation to every perimeter
   path; a non-matching path fails.
3. Baseline governance -> count + content-hash pin (or per-entry ledger) parity
   test; an appended miss without a ledger update fails CI.
4. Rationale honesty -> classification check: every `defs/*.rs` reading authoritative
   truth (per `TRUTH_ACCESSOR_ALLOWLIST`) is either in the perimeter or carries a
   rationale that does not claim mutation coverage.
5. Run evidence -> focused `cargo mutants` over the expanded perimeter; caught/missed
   recorded with the triaged baseline ledger for the acceptance artifact (spec §7.4).

## What to Change

### 1. Perimeter completion

Add `actions/defs/eat.rs` to the duration-defs constant (renamed to reflect the
wider set), the scheduled job's `-f` filters, and the in-diff grep alternation.
Replace eat's untrue rationale.

### 2. Guard widening + regex canary

Extend `mutation_perimeter_consistency_violations` to the full perimeter set in
both jobs; add the canary applying the in-diff regex to each perimeter path.

### 3. Baseline governance + one honest triage

Add the count/hash pin + ledger (the `CONTENT_NEGATIVE_PROOFS` parity pattern) over
`.cargo/mutants-baseline-misses.txt`. Triage the 148 entries once — per-entry
disposition (justified-baseline with rationale / warrants-test → follow-up ticket);
refresh honestly after `-001`/`-002`'s new tests land (some entries may now be
caught). Record the disposition table for the acceptance artifact.

### 4. Conformance rows

Add the widened-perimeter and baseline-governance rows to
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`; correct the "pinned
mutation baseline" wording to reflect the count/ledger pin now that it exists.

## Files to Touch

- `.cargo/mutants.toml` (modify)
- `.cargo/mutants-baseline-misses.txt` (modify — triaged refresh)
- `.github/workflows/ci.yml` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- Paying down test debt the baseline triage surfaces (follow-up tickets, per
  Assumption Reassessment item 7).
- The in-diff loud-failure semantics and push-gap (0019 deliverables, verified
  holding).
- The generative tier (`-004`); apply-arm censuses (`-002`).

## Acceptance Criteria

### Tests That Must Pass

1. Widened perimeter guard green, with synthetic filter-removal case failing.
2. Regex canary green over every perimeter path.
3. Baseline pin/ledger parity test green; synthetic unledgered-append case failing.
4. Focused `cargo mutants` run over `eat.rs`/`sleep.rs`/`work.rs` recorded
   (caught/missed + triaged dispositions; no bulk-accepted refresh).
5. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. The mutation perimeter, both CI filter sets, and the classification rationales
   cannot diverge silently — the guard enforces their consistency (lock durability).
2. Baseline membership is ledgered; appending a miss without a recorded rationale is
   a CI failure, not a silent acceptance.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — widened perimeter
   guard, regex canary, baseline pin/ledger parity test, rationale-honesty check.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo mutants -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle -j 2` (focused perimeter evidence run)
3. `cargo test --workspace` (full pipeline)

## Outcome

Completed: 2026-06-11

What changed:

- Added `actions/defs/eat.rs` to the scheduled cargo-mutants filter set and to
  the in-diff guarded-path regex alongside `sleep.rs` and `work.rs`.
- Widened `mutation_perimeter_matches_duration_action_rationale_and_ci_filters`
  so it checks the full perimeter filter set, representative in-diff regex
  canaries, action-definition rationale text, cargo-mutants failure semantics, and
  the existing direct-push coverage.
- Added synthetic guard cases proving removal of the scheduled `eat.rs` filter or
  the in-diff `eat` regex arm fails.
- Added mutation-baseline governance over `.cargo/mutants-baseline-misses.txt`:
  the normalized accepted baseline is pinned at 143 entries with content hash
  `bd1855a5ee82b428`, and every normalized entry must appear in
  `reports/0020_mutants_baseline_disposition.md` with rationale text.
- Refreshed the accepted miss baseline with the seven current-tree `eat.rs` misses
  from the focused run and recorded them as future test debt in the ledger.
- Added architecture conformance rows for the completed mutation perimeter and
  governed mutation baseline, and corrected the older "pinned mutation baseline"
  wording to "governed mutation baseline."

Focused mutation evidence:

- Command:
  `cargo mutants -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle -j 2`.
- Result: unmutated baseline passed; 107 mutants tested in 5 minutes; 7 missed,
  76 caught, 24 unviable.
- Miss disposition: all seven misses are `eat.rs` access/location-payload test debt
  and are ledgered in `reports/0020_mutants_baseline_disposition.md`; no bulk
  unreviewed refresh was made.

Deviations from original plan:

- The baseline governance uses a deterministic test-local FNV-1a content hash
  instead of adding a hashing dependency to the zero-dependency core crate.
- The per-entry triage is recorded as a reviewed disposition ledger with all current
  accepted misses marked as justified baseline residence/future focused test debt;
  no production-code changes were made to pay down that debt in this ticket.

Verification results:

- `cargo test -p tracewake-core --test anti_regression_guards mutation_baseline_misses_are_pinned_and_ledgered` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards mutation_perimeter_matches_duration_action_rationale_and_ci_filters` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards` — passed.
- `cargo mutants -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle -j 2` — completed with expected miss status; 7 missed, 76 caught, 24 unviable.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
