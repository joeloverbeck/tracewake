# 0017PHA3ATICLED-009: Generative lock tier — sequence generator, differential and metamorphic oracles

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (new test-side generative harness); no production code, no new dependencies
**Deps**: `archive/tickets/0017PHA3ATICLED-005.md`, `archive/tickets/0017PHA3ATICLED-006.md` (the behavioral surface the oracles exercise is finished; transitive over `-001`…`-004`); `specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (§5 tier 5)

## Problem

Every lock in the suite is exercised only by histories the golden fixtures happen to record — which is precisely why ORD-HARD-026 survived four audits: no recorded history put an autonomous wait against a window boundary at mid-range need values, so the false arithmetic byte-matched its own replay forever. The spec's structural answer is a generative tier: deterministically generated action/window sequences run through the real kernel, checked by oracles that need no golden answer — live-vs-replay differential comparison, metamorphic relations (prefix+suffix ≡ full replay; marker-append changes nothing; single-charge-per-tick over arbitrary histories), and reachability counters proving the corpus actually exercises regime transitions, interruptions, and terminal kinds.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: no `proptest`/`quickcheck`/`arbitrary`/fuzz dependency exists anywhere in the workspace (all `Cargo.toml`s checked during the 0017 audit); the replay machinery the oracles reuse exists and is public to tests — `replay/rebuild.rs::rebuild_projection` / `rebuild_decision_context_hashes`, `report.rs::run_replay`, `compute_physical_checksum` / `compute_agent_state_checksum`; the scheduler's no-human runner (`run_no_human_day`) and content seed paths provide the generation substrate; `crates/tracewake-core/tests/support/` exists for shared test helpers.
2. Spec 0017 §5 tier 5 and §9 (open question resolved as recommended): an **in-tree, explicitly-seeded generator** — no `proptest` dev-dependency — because the oracles need valid sequences and determinism, not shrinking; the choice is recorded in the conformance index by ticket `-010`. The clippy `rand` ban is not implicated: the generator is a hand-rolled deterministic LCG with fixed, named seeds, which is exactly the seedable/auditable posture INV-017 demands.
3. Cross-artifact boundary under audit: the generator ↔ kernel API seam — generated sequences must enter through the same seed/scheduler/pipeline surfaces the golden fixtures use (no synthetic state construction that bypasses validation), or the oracles prove nothing about production paths.
4. INV-017 restated: randomness affecting meaningful outcomes must be seedable, reproducible, and visible — every generated case derives from a fixed seed list committed in the test, and a failure report names the seed, the sequence, and (for the differential oracle) the first divergent window.
5. Deterministic-replay surface touched (strengthened, read-only): the differential oracle byte-compares live vs replayed checksums per window boundary over every generated sequence; the metamorphic prefix+suffix relation exercises `replay_prefix` composition; the single-charge metamorphic property reconciles per-tick charge attribution over arbitrary interleavings (the property form of ticket `-002`'s gate). No production behavior changes; goldens unchanged.
6. Adjacent finding channel classified: a reachability counter that reads zero (an unexercised regime transition or terminal kind) is a corpus gap to fix in this ticket; an oracle failure on a generated sequence is a *kernel bug* — file it as a new finding/ticket rather than weakening the oracle (the spec's enforcement-reading posture: tests must be corrected only when they reward violations, never to pass them).

## Architecture Check

1. An in-tree seeded LCG generator + oracle harness keeps `tracewake-core` at zero dependencies (including dev), keeps generation fully deterministic and auditable (INV-017), and reuses the production replay machinery as the oracle — no parallel reimplementation to drift (the same principle that made the 0016 context-hash gate honest). Metamorphic relations provide correctness checks with no golden answer to maintain, which is what fixture-checksum locks structurally lack.
2. No backwards-compatibility aliasing/shims: not applicable to new test infrastructure; no production surface changes.

## Verification Layers

1. INV-018 differential oracle -> new harness test: for each seeded sequence — run live, serialize, replay, byte-compare physical+agent checksums at every window boundary; first-divergence reporting.
2. Replay composition -> metamorphic relation: prefix-replay + suffix-replay equals full replay for generated split points; appending a payload-free marker event changes no checksum.
3. Single-charge-per-tick (INV-039/045 property form) -> metamorphic relation over generated wait/sleep/work/window interleavings, reconciled against `classify_actor_tick_regimes`.
4. Corpus adequacy -> reachability counters: the suite fails if the corpus exercises zero instances of any named class (regime transition kinds, duration terminals including `WorkBlockFailed`, severe-need interruption, replan).

## What to Change

### 1. Seeded sequence generator (`tests/support/`)

A deterministic LCG-driven generator producing valid bounded scenarios: small seeded worlds (reusing existing fixture/seed helpers), window layouts, and per-actor action pressure (waits, sleeps, work blocks, eats, interruptions) — all entering through the real seed/scheduler/pipeline surfaces. Fixed seed list committed in the test; failure output names seed + sequence.

### 2. Differential and metamorphic oracles (`tests/generative_lock.rs`)

The three oracle families from Verification Layers, each over the full seed corpus; per-window first-divergence diagnostics for the differential oracle.

### 3. Reachability counters

"Sometimes"-style counters asserted nonzero per named class at corpus end, making coverage evidence rather than hope.

## Files to Touch

- `crates/tracewake-core/tests/generative_lock.rs` (new)
- `crates/tracewake-core/tests/support/` (modify — generator module added to the existing support tree; exact file as surfaced by the support layout)

## Out of Scope

- CI workflow changes (ticket `-008` owns `ci.yml`; this harness runs under `cargo test --workspace`).
- `proptest`/`arbitrary`/fuzzing dependencies (resolved against; revisit only if failure triage proves painful — spec §9).
- Kani proof harnesses (spec marks optional; record skip-without-prejudice in the conformance row via ticket `-010` if not attempted).
- Fixing any kernel bug an oracle surfaces (new finding/ticket per Assumption item 6).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test generative_lock` — full seed corpus green across all three oracle families, reachability counters nonzero.
2. Corpus runtime stays within ordinary `cargo test --workspace` tolerance (size the seed list accordingly; record the count in the test header).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every generated case is reproducible from a committed seed; no wall-clock, environment, or platform input reaches generation.
2. Oracles consume production kernel/replay surfaces only — no test-side reimplementation of accounting, application, or hashing.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/generative_lock.rs` — differential oracle; prefix+suffix and marker-append metamorphic relations; single-charge property; reachability assertions.
2. `crates/tracewake-core/tests/support/` generator module — unit tests for generator determinism (same seed ⇒ identical sequence).

### Commands

1. `cargo test -p tracewake-core --test generative_lock`
2. `cargo test --workspace`
