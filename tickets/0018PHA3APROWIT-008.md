# 0018PHA3APROWIT-008: Generative tier Phase 3A reach, swarm masks, and dual-checksum marker relation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — test substrate only (`tracewake-core` `tests/support/generative.rs`, `tests/generative_lock.rs`); no production code changes
**Deps**: `archive/tickets/0018PHA3APROWIT-002.md`, `archive/tickets/0018PHA3APROWIT-003.md`, `archive/tickets/0018PHA3APROWIT-004.md`, `archive/tickets/0018PHA3APROWIT-007.md` (the tier exercises the finished behavioral surface; spec §8 ordering); `specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (ORD-HARD-037)

## Problem

The 0017 generative lock tier cannot reach the regimes it was built to protect. `tests/support/generative.rs::registry()` registers only `register_phase1_inspect_wait()`; `generate_case` emits a fixed four-window wait-only shape over exactly four seeds (`GENERATIVE_SEEDS`); `generative_lock.rs::Reachability` asserts only `actor_waited`, `need_delta`, `no_human_marker`, `replay_round_trip`, `prefix_replay`. The corpus contains no `SleepStarted`, no work blocks, no interruptions, no duration terminals — so the single-charge metamorphic property structurally cannot reach the sleep/work regime where ORD-HARD-026 lived, defeating 0017 §5 tier 5's stated purpose (catch the double-charge "without anyone authoring the adversarial fixture"). Additionally `assert_marker_append_does_not_change_physical_checksum` compares only the physical checksum — a marker perturbing agent-stream materialization would pass. The conformance index honestly concedes the limitation; this ticket removes it.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `a9c62e0`: `registry()` registers only the Phase 1 inspect/wait vocabulary; `GENERATIVE_SEEDS` has 4 entries; `Reachability` has the five fields above and no sleep/terminal/interruption counters; the marker relation checksums physical only; the differential (`assert_live_matches_replay`), prefix (`assert_prefix_replay_matches_full`), and serialization round-trip oracles are real and assert.
2. Spec 0018 ORD-HARD-037 (required correction + structural lock) and §9 cost note: corpus stays bounded (seed count × window count); diversity is asserted, not volume. External technique adopted: swarm-testing feature masks (Groce et al., ISSTA 2012 — per-run omission of action kinds from the vocabulary), recorded in spec §2.
3. Cross-artifact boundary under audit: the generative lock-tier contract (0017 §5 tier 5, extended by 0018 §5 tier 5) — explicitly-seeded generation, live-vs-replay differential oracle at window boundaries, metamorphic relations, reachability counters that fail on zero.
4. Lock durability + INV-018 restated: the differential oracle is the generator-scoped complement to the fixture-scoped golden gates — deterministic replay must hold over *generated* histories, not only recorded ones; a lock whose corpus cannot reach a regime provides no evidence about that regime.
5. Deterministic-replay surface touched (test substrate): the generator must remain explicitly seeded (no ambient time/randomness — the in-tree LCG stays the only entropy source); registering the Phase 3A vocabulary changes no production code and no golden fixture, so golden checksums are untouched; every generated sequence must still satisfy the differential and metamorphic oracles at the extended vocabulary. Epistemic surfaces are exercised, not modified.
6. Adjacent contradiction classified: the conformance-index concession row ("duration terminal reachability remains covered by targeted fixtures") becomes stale once this lands — superseding that row is ticket `0018PHA3APROWIT-010`'s docs surface, not this ticket's.

## Architecture Check

1. Extending the existing in-tree seeded generator (registering the real Phase 3A affordances, seeding generated fixtures with sleep affordances/workplaces/food) preserves the zero-dependency posture and reuses the proven oracle plumbing, rather than introducing `proptest` (0017 §9 recorded that decision; nothing here reopens it). Swarm feature-masks are a ~zero-cost algorithmic tweak to the LCG harness that diversifies reached states far more than adding seeds alone — per-run omission of action kinds prevents dominant actions from suppressing rare interleavings.
2. No backwards-compatibility aliasing/shims: the wait-only `generate_case` shape is replaced by the masked multi-action generator; no "legacy corpus" path remains.

## Verification Layers

1. Regime reachability -> extended `Reachability` with `sleep_block`, at least one duration-terminal kind, and at least one interruption — each asserted non-zero over the corpus (a future change starving the generator of a regime fails the suite).
2. INV-045 single charge over generated histories -> the existing `assert_single_need_charge_per_actor_tick_kind` property now runs over sleep/work-bearing sequences — the regime where ORD-HARD-026 lived.
3. INV-018 differential oracle -> `assert_live_matches_replay` (agent + physical checksums) green over every generated sequence at the extended vocabulary; prefix and serialization relations likewise.
4. Marker invariance (both streams) -> `assert_marker_append_does_not_change_physical_checksum` extended to assert the agent checksum too.
5. Corpus diversity -> new assertions: distinct terminal kinds observed; distinct sequence lengths; expanded seed set with per-run feature masks recorded in the failure message (reproducibility: seed + mask fully determine a run).

## What to Change

### 1. Phase 3A vocabulary in the generative registry

`support/generative.rs`: register sleep/eat/work/continue affordances; generated fixtures seed sleep affordances, workplaces, and food supplies so the actions are actually proposable.

### 2. Swarm feature masks + corpus expansion

Per-run action-kind mask drawn from the seeded LCG; expand `GENERATIVE_SEEDS`; randomize per-window action selection under the mask.

### 3. Extended reachability and diversity assertions

`generative_lock.rs`: `Reachability` gains sleep/terminal/interruption counters with non-zero assertions; corpus-diversity assertions per Verification Layer 5.

### 4. Dual-checksum marker relation

The marker-append relation asserts physical AND agent checksums unchanged.

## Files to Touch

- `crates/tracewake-core/tests/support/generative.rs` (modify)
- `crates/tracewake-core/tests/generative_lock.rs` (modify)

## Out of Scope

- Any production-code change (the tier is test substrate; a generative failure surfacing a real defect becomes its own ticket).
- Mutation-testing cadence and censuses (ticket `0018PHA3APROWIT-009`).
- The conformance-index row superseding the concession (ticket `-010`).
- `proptest`/Kani adoption — decided against in 0017 §9; unchanged.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test generative_lock` — all oracles green at the extended vocabulary; reachability counters non-zero for sleep, a duration terminal, and an interruption; diversity assertions green.
2. Runtime stays bounded: the generative suite completes within the existing workspace test budget (corpus bounded by seed × window counts).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. The generative corpus provably exercises wait, sleep, work, eat, window boundaries, interruptions, and duration terminals — reachability is asserted evidence, not hope.
2. Generation is fully determined by (seed, mask): no ambient entropy; every failure is reproducible from the reported pair.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/support/generative.rs` — Phase 3A vocabulary, masks, seeds.
2. `crates/tracewake-core/tests/generative_lock.rs` — extended reachability, diversity, dual-checksum marker relation.

### Commands

1. `cargo test -p tracewake-core --test generative_lock`
2. `cargo test --workspace`
