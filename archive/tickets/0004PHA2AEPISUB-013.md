# 0004PHA2AEPISUB-013: Phase 2A fixtures — strongbox expectation, Mara, and golden scenarios

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds Phase 2A golden fixtures and extends `strongbox_001` in `tracewake-content`.
**Deps**: 0004PHA2AEPISUB-012

## Problem

Phase 2A requires content fixtures with structured setup, allowed actions, expected events/projections, acceptance assertions, and forbidden shortcuts (Spec 0004 §14). It must extend `strongbox_001` with Tomas's source-backed expectation without corrupting the Phase 1 physical baseline (§14.1), introduce `actor_mara` as an ordinary actor (§11.3), and add the golden scenarios `expectation_contradiction_001`, `possession_parity_001`, `view_filtering_001`, `knowledge_blocker_accuse_001`, `sound_uncertainty_001`, and `no_human_epistemic_check_001` (§14.2–14.7). Scenario seeds are tensions, not scripts (`INV-062`).

## Assumption Reassessment (2026-06-06)

1. `crates/tracewake-content/src/fixtures/strongbox_001.rs` is a Phase 1 physical fixture with `actor_tomas` + `actor_elena` (`strongbox_001.rs:8`), `house_tomas`, `strongbox_tomas`, and `coin_stack_01`, and **no `actor_mara`** (confirmed by grep) — it intentionally asserts no belief/contradiction/suspicion. `crates/tracewake-content/src/fixtures/mod.rs` registers fixtures. New fixture files join `fixtures/`.
2. The fixture names, setups, and forbidden-shortcut lists are fixed by Spec 0004 §14.1–14.7; the seed schema and validation come from ticket 012; the expectation belief shape is §11.2 (`belief_tomas_expects_coin_stack_01_in_strongbox_tomas`, `source_kind: authored_prehistory`, `source_id: prehistory_tomas_checked_strongbox_before_start`).
3. Shared boundary under audit: `fixtures/mod.rs` is the registration hub (each new fixture registers there); `strongbox_001.rs` is *modified* (expectation extension) and must preserve its Phase 1 physical assertions — the Phase 1 golden tests over it must still pass.
4. Invariant motivating this ticket: `INV-061` (authored causal machinery creates possibility space, not arcs), `INV-062` (scenario seeds are tensions, not scripts), and `INV-063` (authored prehistory must be marked — the expectation's `authored_prehistory` provenance).
5. Fail-closed validation / no-leak surface (substrate-only): these fixtures must pass ticket 012's validator — no `culprit`/`stolen_flag`/`npc_knows_truth`, no player memory, no global `coin_missing_known` flag. Mara is an ordinary actor whose theft uses existing physical actions (`INV-007`). The fixtures carry holder/source-backed seeds only; they introduce no leakage and no nondeterminism (canonical ordering via ticket 012). The enforcing validator is ticket 012; the acceptance assertions run in ticket 016.

## Architecture Check

1. Extending `strongbox_001` with an expectation *variant/extension* (not rewriting the Phase 1 fixture) keeps the physical baseline intact for Phase 1 regressions while adding the epistemic seed, matching the spec's "variant or extension" requirement. One fixture file per golden scenario keeps each reviewable and independently loadable.
2. No backwards-compatibility shims: fixtures are data; the expectation seed is additive and Phase 1 assertions are preserved.

## Verification Layers

1. Seeds are tensions not scripts (`INV-062`/`INV-060`) -> schema validation: every fixture passes ticket 012's validator with no shortcut-truth/quest/objective field.
2. Authored prehistory marked (`INV-063`) -> schema validation + grep-proof: the Tomas expectation seed carries `source_kind: authored_prehistory` and a stable `source_id`.
3. Phase 1 baseline preserved (`INV-066` regression) -> replay/golden-fixture check: existing Phase 1 golden tests over `strongbox_001` still pass after the extension.

## What to Change

### 1. strongbox_001 expectation extension

Modify `fixtures/strongbox_001.rs` (or add a Phase 2 variant constructor) to add the §11.2 Tomas expectation seed and preserve all Phase 1 physical assertions and forbidden-shortcut guarantees.

### 2. Mara content

Add `actor_mara` as an ordinary actor at a reachable place in the possession-parity fixture; her theft/movement uses existing physical actions. No `culprit`/`stolen_flag`/`npc_knows_truth` field.

### 3. Golden scenario fixtures

Add fixture files for `expectation_contradiction_001`, `possession_parity_001`, `view_filtering_001`, `knowledge_blocker_accuse_001`, `sound_uncertainty_001`, and `no_human_epistemic_check_001`, each encoding its §14 setup/allowed-actions/expected-projections/forbidden-shortcuts, and register each in `fixtures/mod.rs`.

## Files to Touch

- `crates/tracewake-content/src/fixtures/strongbox_001.rs` (modify — add expectation seed, preserve Phase 1 baseline)
- `crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs` (new)
- `crates/tracewake-content/src/fixtures/possession_parity_001.rs` (new)
- `crates/tracewake-content/src/fixtures/view_filtering_001.rs` (new)
- `crates/tracewake-content/src/fixtures/knowledge_blocker_accuse_001.rs` (new)
- `crates/tracewake-content/src/fixtures/sound_uncertainty_001.rs` (new)
- `crates/tracewake-content/src/fixtures/no_human_epistemic_check_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register new fixtures)

## Out of Scope

- Runtime sound observation generation logic (ticket 014) — `sound_uncertainty_001` provides placement/seed only.
- The golden/transcript test assertions exercising these fixtures (ticket 016).
- Fixture-contract documentation prose (ticket 015).

## Acceptance Criteria

### Tests That Must Pass

1. All seven Phase 2A fixtures load and pass content validation (ticket 012).
2. `strongbox_001` carries the Tomas expectation with `authored_prehistory` provenance and still passes Phase 1 golden tests.
3. `possession_parity_001` contains `actor_mara` as an ordinary actor and no shortcut-truth field.

### Invariants

1. Fixtures are typed possibility with holder/source-backed seeds; no script/quest/culprit field.
2. The Phase 1 `strongbox_001` physical baseline is preserved.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/fixtures_load.rs` (extend) — all seven fixtures load and validate.
2. `crates/tracewake-content/tests/golden_fixtures_run.rs` (extend) — fixtures construct the expected initial epistemic seed/projection (full scenario runs in ticket 016).

### Commands

1. `cargo test -p tracewake-content --test fixtures_load`
2. `cargo test -p tracewake-content`
3. `cargo build --workspace --all-targets --locked`

## Outcome

Completed on 2026-06-07.

Changed:
- Extended `strongbox_001` with Tomas's authored-prehistory expectation seed while preserving the Phase 1 physical baseline.
- Added and registered the Phase 2A fixtures: `expectation_contradiction_001`, `possession_parity_001`, `view_filtering_001`, `knowledge_blocker_accuse_001`, `sound_uncertainty_001`, and `no_human_epistemic_check_001`.
- Added fixture helpers for authored-prehistory expectation and low-confidence sound-lead seeds, registered Phase 2A epistemic actions in content loading, and extended tests for fixture registration, Mara as an ordinary actor, script-free Phase 2A content, and projection construction from seeds.
- Aligned content seed serialization with the required `authored_prehistory` source kind.

Deviations:
- None.

Verification:
- `cargo test -p tracewake-content --test fixtures_load`
- `cargo test -p tracewake-content`
- `cargo build --workspace --all-targets --locked`
- `cargo fmt --all --check`
- `git diff --check`
