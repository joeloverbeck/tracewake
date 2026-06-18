# 0039SPICERMUT-002: Kill `content/load.rs` SPINE survivors with fixture-load + seed-causality witnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-content` (test-only by default; a production correction in `load.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 8 missed mutants in `crates/tracewake-content/src/load.rs` (spec §5.1), owning SPINE-05 (save/content identity), SPINE-01 (seed event causality), and SPINE-08 (seed-mutation boundary). The historical cluster covers `fixture_scope_from_raw_lines` and its `phase1` / `phase2a_historical` / `phase3a_historical` arms, the event-index arithmetic in `seed_event_log`, and event-ID progression in `append_starting_belief` and `append_role_assignment_notices`. Current content tests do not pin exact scope semantics, monotonic seed event IDs, or the replay/provenance consequence, so deleting a scope arm or perturbing an event-index counter survives.

## Assumption Reassessment (2026-06-18)

1. `fixture_scope_from_raw_lines` exists at `crates/tracewake-content/src/load.rs:119` returning `Option<FixtureScope>`; `seed_event_log` at `:154`; `append_starting_belief` at `:264`; `append_role_assignment_notices` at `:305` (verified by grep). The exact 8 seed-mutant identities are enumerated in `reports/0038_spine_cert_mutation_triage_register.md` (the seed work-list; trust the symbol/operator identity over the drifted line:col).
2. Spec §5.1 is the implementation contract; `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` and Spec `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` govern fixture-scope and replay-acceptance semantics (verified present).
3. Shared boundary under audit: the content-loader seam where raw fixture lines become a typed `FixtureScope`, a seeded `EventLog`, and seed beliefs/role-assignment notices — feeding SPINE-01/05 event causality and replay.
4. Motivating invariants: `INV-009 — Meaningful state changes require events`, `INV-010 — Every event needs a cause model`, and `INV-061/062` (authored causal machinery / scenario seeds are tensions, not scripts). The witnesses must observe eventful, caused, ordered seed state — not a non-empty log.
5. This ticket touches a deterministic-replay and fail-closed content-validation surface: seed event IDs must be monotonic, unique, causal, and stable across duplicate loads, and malformed/duplicated/unknown/contradictory scope inputs must fail at fixture-load validation rather than silently defaulting. The new witnesses only strengthen these properties; they introduce no leakage or nondeterminism and add no schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-01/SPINE-05 re-proof in ticket 021.

## Architecture Check

1. Driving each legal scope marker through the real loader and asserting the exact typed `FixtureScope`, accepted-fixture census, event count/order, and replayed provenance ancestry is stronger than any single "log is non-empty" assertion: each deleted scope arm and each counter mutation fails its own case.
2. No backwards-compatibility aliasing/shims: witnesses travel the production loader path; no test-only seed constructor or bypass is introduced.

## Verification Layers

1. INV-009/010 (eventful, caused seed state) -> replay/golden-fixture check: replay the seeded log and assert the actor-known belief/assignment consequence cites the expected source event IDs.
2. Scope-arm exactness -> schema validation: table-driven assertion of the typed `FixtureScope` per legal marker, plus a single-defect invalid sibling per arm.
3. INV-008 seed-mutation boundary -> codebase grep-proof / negative test: post-load external callers cannot invoke seed mutators or mutate seed maps directly.

## What to Change

### 1. Table-driven fixture-load contract

In `fixtures_load.rs`, load each legal scope marker through the real loader and assert the resulting typed `FixtureScope`, manifest identity, and accepted-fixture census; add malformed, duplicated, unknown, and contradictory scope inputs that must fail at fixture-load validation rather than silently defaulting.

### 2. Seed causality + replay provenance witnesses

For seeded beliefs and role notices, assert event IDs / global order are monotonic, unique, causal, and stable across duplicate loads; replay the resulting log and prove the actor-known belief/assignment consequence cites the expected source event IDs.

### 3. Member matrix

Map each of the 8 seed mutants (and any new run survivor in this file) to a concrete case that fails for that specific deleted scope arm or counter mutation.

## Files to Touch

- `crates/tracewake-content/tests/fixtures_load.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Schema-conversion bound selection in `schema.rs` (ticket 003).
- Serialization round-trips in `serialization.rs` (ticket 004).
- The full mutation run and survivor reconciliation (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test fixtures_load` — passes with the per-scope-arm matrix and single-defect negatives.
2. `cargo test --locked -p tracewake-content --test golden_fixtures_run` — passes with the seed-causality + replay-provenance witnesses.
3. `cargo mutants --workspace -f crates/tracewake-content/src/load.rs --no-shuffle` — every historical `load.rs` survivor (and any newly enumerated one) is `caught`.

### Invariants

1. Each legal scope arm and each seed event-index counter is observed by a distinct failing case; no "log non-empty" tautology suffices.
2. Malformed/duplicate/unknown/contradictory scope inputs fail at fixture-load validation and produce no partial seed state.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/fixtures_load.rs` — per-scope-arm load contract + single-defect invalid siblings.
2. `crates/tracewake-content/tests/golden_fixtures_run.rs` — monotonic/unique/causal seed event IDs, stable across duplicate loads, and replay cites the expected source event IDs.

### Commands

1. `cargo test --locked -p tracewake-content --test fixtures_load --test golden_fixtures_run`
2. `cargo mutants --workspace -f crates/tracewake-content/src/load.rs --no-shuffle`
3. The per-file `-f` mutation run is the correct verification boundary for this ticket; the full standing-perimeter campaign is ticket 020.

## Outcome

Completed: 2026-06-18

Implemented the `content/load.rs` survivor kill set with a production correction and fixture-load witnesses. The raw `fixture_scope` pre-scan now fails closed through `load_fixture_package`: missing scope, malformed scope, unknown scope, and duplicate or contradictory scope markers return serialization errors instead of silently defaulting to Phase 3A. This production change was required because the previous default masked deleted scope-arm mutants and could let invalid raw fixture identity pass into later validation.

Added a per-scope loader matrix in `fixtures_load.rs` that proves legal Phase 1, Phase 2A, and Phase 3A markers select the exact registry and typed `FixtureScope`, with negative siblings for later-phase actions and defective scope lines. Added a seed-causality witness that loads the same fixture twice, pins stable seed event order/IDs/stream positions/proposal counters, and proves authored beliefs, starting-place beliefs, sleep-place beliefs, and role-assignment notices replay into the epistemic projection with the expected source event IDs.

`golden_fixtures_run.rs` did not require edits; the requested golden-fixture acceptance lane still passed unchanged. No enumerated ticket member was deferred or dropped.

Verification:

1. `cargo test --locked -p tracewake-content --test fixtures_load` — passed.
2. `cargo test --locked -p tracewake-content --test fixtures_load --test golden_fixtures_run` — passed.
3. `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-content/src/load.rs --no-shuffle` — passed: 25 mutants tested, 19 caught, 6 unviable, 0 missed.
4. `cargo fmt --all --check` — passed.
5. `cargo clippy --workspace --all-targets -- -D warnings` — passed.
6. `cargo build --workspace --all-targets --locked` — passed.
7. `cargo test --workspace --locked` — passed.

Mutation-command note: after ticket 001, `.cargo/mutants.toml` defines the standing SPINE perimeter through `examine_globs`. Running the literal ticket command `cargo mutants --workspace -f crates/tracewake-content/src/load.rs --no-shuffle` therefore enumerated the full standing perimeter (`Found 2623 mutants to test`) instead of the intended per-file ticket boundary, and was interrupted after the clean unmutated baseline. The certifying run used `--no-config -C=--locked -f crates/tracewake-content/src/load.rs` to preserve the ticket's per-file boundary while keeping locked Cargo arguments.
