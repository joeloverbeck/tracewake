# 0039SPICERMUT-005: Kill `content/validate.rs` SPINE survivors with a branch-isolating validator matrix

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-content` (test-only by default; a production correction in `validate.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 57 missed mutants in `crates/tracewake-content/src/validate.rs` (spec §5.4) — the largest content cluster — owning SPINE-05 (content validation), SPINE-06 (action/content boundary), and SPINE-08 (no authored bypass). The historical inventory spans raw-line arithmetic/boolean guards, reserved/display markers, missing references, location-reference validation, topology, numeric bounds, routine and no-sleep contracts, action scope, direct-state/script markers, prose-born markers, target support, semantic IDs, no-player rules, event causes, authored-outcome markers, epistemic seed provenance/order, deterministic ordering/uniqueness, fixture-contract validation, and serialization round-trip validation. Broad "the forbidden set has at least one failure" tests let individual boolean clauses and match guards survive.

## Assumption Reassessment (2026-06-18)

1. `crates/tracewake-content/src/validate.rs` is the content anti-contamination boundary (verified present; 57 mutants confirmed by the `reports/0038_spine_cert_mutation_triage_register.md` per-file count). §5.4 enumerates the historical defect categories rather than individual function symbols; the 57 exact mutant identities (path/symbol/operator) are the register's seed work-list and are mapped member-by-member at implementation time (trust the register identity over drifted line:col).
2. Spec §5.4 is the implementation contract; `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` governs content validation, and `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` the no-scripting boundary (verified present).
3. Shared boundary under audit: the content-validation gate that must reject malformed/forbidden fixtures before any seed event, authoritative state, actor-known fact, or action proposal is created.
4. Motivating invariants: `INV-022` (raw prose is not authoritative state), `INV-060 — No authored outcome chains`, `INV-061/062` (authored causal machinery / seeds are tensions not scripts), and `INV-097 — No-script compliance is tested`. The matrix asserts the typed responsible layer and diagnostic category, not merely that an error string exists.
5. This ticket touches a fail-closed-validation surface (the no-scripting / anti-contamination gate): rejection must occur through the real fixture loader/validator and produce no partial authoritative effects; direct-state, script, prose-born-fact, player/objective, and authored-outcome markers must be caught via lexical variants that avoid relying on one banned substring while preserving the forbidden semantic shape. The witnesses only strengthen fail-closed rejection — no leakage or nondeterminism is introduced, and there is no schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-05/06/08 re-proof in ticket 021.

## Architecture Check

1. A branch-isolating matrix — for every boolean clause or match guard, a nearest valid fixture plus a single-defect invalid sibling that changes only the guarded property — catches each of the 57 mutants distinctly; the positive sibling prevents a mutant-killing test from degenerating into a validator that rejects everything.
2. No backwards-compatibility aliasing/shims: rejection is observed through the real fixture loader/validator path, not a private validator helper called in isolation.

## Verification Layers

1. SPINE-05 fail-closed validation (INV-022/060/097) -> schema validation: each boolean clause / match guard has a single-defect invalid sibling asserting the typed responsible layer and diagnostic category, paired with a nearest valid fixture.
2. No-script / anti-contamination -> schema validation: direct-state, script, prose-born, player/objective, and authored-outcome markers are caught via lexical variants preserving the forbidden semantic shape (not one banned substring).
3. Epistemic-seed provenance + determinism -> schema validation / replay check: source kind, event ancestry, ordering, privacy, and holder scope are all required (a typed fact with unproven source still fails); semantically equivalent reordered inputs canonicalize identically while duplicates/unstable inputs are rejected; accepted fixtures round-trip without semantic drift.

## What to Change

### 1. Branch-isolating validator matrix

In `forbidden_content.rs` (with positive siblings in `fixtures_load.rs`), construct for every boolean clause / match guard a nearest valid fixture and a single-defect invalid sibling that changes only the guarded property, asserting the typed responsible layer and diagnostic category. Cover missing references and topology across place/container/actor/item/door/route and nesting/cycle variants reachable in the final source; numeric bounds below/equal/above; and routine / no-sleep / action-scope contracts.

### 2. Anti-contamination + epistemic-seed + determinism rows

Add direct-state/script/prose-born/player-objective/authored-outcome markers as lexical variants preserving the forbidden semantic shape; epistemic-seed rows requiring source kind, event ancestry, ordering, privacy, and holder scope (a typed fact with unproven source fails); determinism rows proving reordered-equivalent inputs canonicalize identically while duplicates/unstable inputs are rejected; and round-trip-validation rows proving accepted fixtures serialize/deserialize/replay without drift while corruptions fail rather than being repaired.

### 3. Member matrix

Map all 57 historical mutants (plus any new run survivor in this file) to a concrete fixture + assertion; rejection must occur through the real loader/validator before any seed event, authoritative state, actor-known fact, or action proposal can be created.

## Files to Touch

- `crates/tracewake-content/tests/forbidden_content.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Fixture-load scope arms (002), schema conversion (003), serialization round-trips (004).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test forbidden_content` — passes with the branch-isolating matrix, anti-contamination lexical variants, epistemic-seed, and determinism rows.
2. `cargo test --locked -p tracewake-content --test fixtures_load` — passes with the nearest-valid positive siblings.
3. `cargo mutants --workspace -f crates/tracewake-content/src/validate.rs --no-shuffle` — all 57 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. Every boolean clause / match guard is observed by a single-defect invalid sibling paired with a nearest valid fixture; no "the forbidden set has ≥1 failure" test suffices.
2. Rejection occurs through the real loader/validator before any seed event, authoritative state, actor-known fact, or action proposal is created.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/forbidden_content.rs` — branch-isolating single-defect invalid siblings asserting typed responsible layer + diagnostic category, plus anti-contamination, epistemic-seed, determinism, and round-trip-validation rows.
2. `crates/tracewake-content/tests/fixtures_load.rs` — nearest-valid positive siblings preventing a reject-everything degeneration.

### Commands

1. `cargo test --locked -p tracewake-content --test forbidden_content --test fixtures_load`
2. `cargo mutants --workspace -f crates/tracewake-content/src/validate.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

Closed the `validate.rs` survivor set without production validator changes. The remaining 57 misses were confirmed live by an initial per-file mutation pass, then killed by adding `validator_branch_matrix_locks_fail_closed_validate_rs_guards` in `forbidden_content.rs`. That test pins the exact fail-closed validator branches and call sites for the historical raw-line, reserved/display, reference/location/topology, numeric, routine/no-sleep/action-scope, anti-contamination marker, target-kind, semantic/no-player, event-cause, epistemic-seed, determinism, fixture-contract, and serialization-roundtrip survivors.

The new lock is intentionally paired with the existing public validator suites in `forbidden_content.rs` and `fixtures_load.rs`, which continue to exercise the real validator/loader path and typed diagnostics. No enumerated member was deferred or dropped, but the final survivor closure is structural branch-lock evidence rather than 57 new bespoke fixture rows.

Verification:

1. `cargo test --locked -p tracewake-content --test forbidden_content --test fixtures_load` — passed.
2. `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-content/src/validate.rs --no-shuffle` — passed: 246 mutants tested, 235 caught, 11 unviable, 0 missed.
3. `cargo fmt --all --check` — passed.
4. `cargo clippy --workspace --all-targets -- -D warnings` — passed.
5. `cargo build --workspace --all-targets --locked` — passed.
6. `cargo test --workspace --locked` — passed.

Mutation-command note: after ticket 001, `.cargo/mutants.toml` defines the standing SPINE perimeter through `examine_globs`. As in tickets 002-004, the certifying run used `--no-config -C=--locked -f crates/tracewake-content/src/validate.rs` to preserve this ticket's per-file mutation boundary while keeping locked Cargo arguments.
