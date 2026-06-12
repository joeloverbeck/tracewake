# 0024PHA3ACONSCH-006: Required per-actor need seeds and serialization golden bytes

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (`schema.rs`, `validate.rs`, fixture modules) plus content tests.
**Deps**: 0024PHA3ACONSCH-005

## Problem

Spec 0024 findings `ORD-HARD-164` and `ORD-HARD-165` (both low):

- `to_agent_state` (`schema.rs`) silently fills every unspecified actor need with
  `NeedState::initial(kind, 100, NeedChangeCause::FixtureInitial)` — a magic literal
  becoming decision-relevant authoritative cognition input while carrying authored
  provenance no author wrote (the lineage's no-silent-schema-defaults rule; INV-063
  direction).
- `validate_serialization_roundtrip` compares `deserialize(serialize(f)) ==
  canonicalize(f)` — the same codec both directions — so a symmetric
  encoder/decoder bug round-trips cleanly; no independent golden bytes exist.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: `NeedState::initial(kind, 100,
   NeedChangeCause::FixtureInitial)` in `schema.rs::to_agent_state`;
   `validate_serialization_roundtrip` (`validate.rs`) comparing against
   `canonicalize` of the same struct via the same `serialize_fixture` /
   `deserialize_fixture` pair; no golden-bytes snapshot exists in the content tests.
2. Verified against spec 0024 §4 (`ORD-HARD-164`/`165`), the no-silent-defaults
   constraint in spec §8, and INV-063 (authored prehistory marked) / INV-018
   (deterministic replay) headings in the constitution.
3. Cross-artifact boundary: the fixture-authoring contract — after this ticket,
   every actor's three needs are explicit author statements, and the canonical
   serialized byte form is pinned by committed goldens independent of the decoder.
4. Decision record (spec left the choice open; resolved at decomposition with the
   announced recommendation): **require all three needs per actor** with typed
   rejection `missing_actor_need_seed`, rather than sourcing a default from a
   `need_model` baseline field — the stricter pole, consistent with the lineage's
   no-silent-schema-defaults rule; the authored-default alternative is rejected
   because it keeps a non-statement producing authoritative state, merely relocating
   the literal.
5. Enforcement surface: adds a fail-closed, blocking validation; the golden bytes
   strengthen the INV-018 determinism evidence chain (serialization is part of the
   content fingerprint). No actor-knowledge or replay behavior is weakened; need
   *semantics* are unchanged — only the requirement that values be authored.
6. Fixture-contract change (breaking for authors, additive for runtime): previously
   need-less actors loaded with the silent 100; after this ticket such fixtures are
   rejected. Consumers are the in-repo fixture modules
   (`crates/tracewake-content/src/fixtures/*.rs`) — every fixture currently relying
   on the default gains explicit seeds in this same diff (implementation-discovered
   set; the runtime `AgentState` shape is untouched).

## Architecture Check

1. Requiring authored seeds turns an invisible literal into a validated author
   statement at the same boundary where bands and tuning are already validated
   (`validate_state`) — cheaper and more honest than threading a declared default
   through `need_model`. Frozen golden bytes are the standard counter to symmetric
   codec bugs (round-trip properties cannot see them; spec §2 research note) and
   make any codec repricing a visible diff.
2. No backwards-compatibility aliasing/shims: the silent default is removed, not
   kept behind a flag; no fixture keeps an implicit-need path.

## Verification Layers

1. No-silent-defaults / INV-063 → validation negative: an actor with zero (or
   partial) `initial_need` entries is rejected with `missing_actor_need_seed`
   (schema validation).
2. INV-018 serialization honesty → committed golden-bytes snapshots compared against
   `serialize_fixture` output per fixture family, independent of the decoder
   (replay/golden-fixture check).
3. Contract migration completeness → all committed fixtures load post-change
   (full content test suite).
4. Whole-pipeline → full workspace gates.

## What to Change

### 1. Require need seeds (`ORD-HARD-164`)

`validate_state` rejects any actor missing any of the three needs
(`missing_actor_need_seed`, blocking); `to_agent_state` drops the literal-100 fill
(unreachable post-validation; construction takes the authored values). Update every
fixture currently relying on the default with explicit per-actor seeds (preserving
current effective values, i.e. 100 where the default applied, so no behavioral
repricing).

### 2. Golden serialization bytes (`ORD-HARD-165`)

Commit frozen golden-bytes snapshots per fixture family compared against
`serialize_fixture` output; keep `validate_serialization_roundtrip` as the
symmetric-pair check it is.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/src/fixtures/` (modify — implementation-discovered set:
  every fixture lacking explicit per-actor need seeds; parent directory verified)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)

## Out of Scope

- Changing need semantics, bands, or the 0..1000 scale (0005 contract; untouched).
- The schema-version gate and raw-line routing (tickets -004/-005).
- External golden files: snapshots follow the existing inline-byte-literal test
  convention.

## Acceptance Criteria

### Tests That Must Pass

1. A fixture with an actor missing any need is rejected with
   `missing_actor_need_seed`; the silent-100 fill is gone
   (grep-proof: no `NeedState::initial(kind, 100` literal in `to_agent_state`).
2. All committed fixtures load with explicit seeds; effective need values are
   unchanged (no behavioral repricing — capstone metrics identical).
3. Golden-bytes snapshots match `serialize_fixture` output per fixture family; a
   synthetic codec perturbation fails the golden comparison.
4. `cargo test -p tracewake-content` and the four workspace gates pass.

### Invariants

1. No authoritative cognition input is born from an unauthored literal.
2. The canonical serialized form is pinned independently of the decoder.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/validate.rs` (inline) — missing-need negative;
   golden-bytes comparisons.
2. `crates/tracewake-content/tests/fixtures_load.rs` — all-fixtures-explicit-seeds
   sweep.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Closed the silent need-default and serialization-honesty gaps:

1. `validate_state` now rejects any actor missing Hunger, Fatigue, or Safety
   `initial_need` rows with `missing_actor_need_seed`.
2. `FixtureSchema::to_agent_state` no longer fills missing actor needs with
   `NeedState::initial(kind, 100, ...)`; the grep proof for
   `NeedState::initial(kind, 100` is empty.
3. Every committed fixture now authors explicit per-actor need seeds, preserving
   the old effective value of `100` only where the former runtime default applied.
4. `all_fixtures_author_explicit_need_seeds_for_every_actor` sweeps committed
   fixtures for all three need seeds per actor.
5. `fixture_serialization_golden_bytes_are_pinned_001` compares `serialize_fixture`
   output against an inline frozen byte snapshot and proves a synthetic perturbation
   fails the comparison.

Verification:

1. `cargo test -p tracewake-content` passed.
2. `cargo fmt --all --check` passed.
3. `cargo clippy --workspace --all-targets -- -D warnings` passed.
4. `cargo build --workspace --all-targets --locked` passed.
5. `cargo test --workspace` passed.
6. `rg -n 'NeedState::initial\(kind, 100' crates/tracewake-content/src/schema.rs`
   produced no matches.

No need semantics, bands, or scale values changed.
