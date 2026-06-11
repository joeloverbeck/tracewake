# 0019PHA3AGENREA-004: Generative tamper relation and per-family reachability floors

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` test oracle only (`tests/generative_lock.rs`, `tests/support/generative.rs`)
**Deps**: `tickets/0019PHA3AGENREA-003.md` (per-family floors count advance-emitted events, which -003 establishes); `specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md` (ORD-HARD-048)

## Problem

The generative oracle is all-positive and its diversity floors are degenerately
satisfiable (`ORD-HARD-048`). `generative_lock.rs` contains no tamper/forgery relation:
every generated log is honest, `assert_live_matches_replay` only ever runs on
well-formed histories, and the existing tamper gates live solely in the content crate's
golden tests — so no seed-swept history is ever required to be *rejected* (INV-017/018:
a replay-integrity tier with no negative case never demonstrates detection). The
diversity floors (`masks.len() >= 4`, `sequence_lengths.len() >= 2`,
`terminal_kinds.len() >= 2`) pass without requiring each terminal family; reachability
flags OR-accumulate across seeds (`reachability.interruption |= has_event(…)`), so a
corpus that, e.g., never produces a work-failure terminal — or reaches interruption on
exactly one seed — still passes.

## Assumption Reassessment (2026-06-11)

1. Verified against current code at `main` `5af8660`: no tamper/perturbation relation
   exists in `generative_lock.rs`; the three floors are as quoted; OR-accumulation is
   verified at `reachability.interruption |= has_event(&run.log, EventKind::SleepInterrupted)`;
   `GENERATIVE_SEEDS` (support/generative.rs) includes `0x18_00_00_13`. The
   "interruption reached by exactly one seed" count is agent-reported in the spec
   (execution-dependent) — confirm the actual per-seed contribution while implementing
   the per-flag multi-seed requirement.
2. Verified against `specs/0019_…_HARDENING_SPEC.md` ORD-HARD-048 (reassessed
   2026-06-11): required corrections are the per-case metamorphic tamper relation, the
   per-family terminal floors (sleep-success, sleep-interrupt, work-success, work-fail
   each nonzero, counted from advance-emitted events per ORD-HARD-044), and ≥2 seeds
   contributing per reachability flag — or an explicit single-seed concession recorded
   in the conformance row. Spec §9 cost note: keep the corpus bounded; assert diversity
   rather than volume.
3. Cross-artifact boundary under audit: the generative oracle's negative-coverage
   contract — the same differential machinery that asserts honest histories replay
   identically must also demonstrate that a perturbed history is *detected*, per
   generated case, not only on hand-authored goldens.
4. INV-017/INV-018 restated: randomness must be auditable and replay deterministic —
   and the proof of detection is a divergence the suite *requires*, not merely an
   absence of divergence on honest input. Lock durability: floors that a degenerate
   corpus satisfies are decorative.
5. Deterministic-replay surface touched (test-only): the tamper relation perturbs one
   payload field of one agent-stream event per generated case in a *copy* and asserts
   `!matches_expected` / checksum divergence — it must never mutate the honest corpus
   used by the positive relations, and the perturbation targets payload fields that are
   checksum-covered by construction (post -001, all materialized payload fields are).

## Architecture Check

1. A per-case metamorphic relation (perturb one field, require divergence) scales
   negative coverage with the corpus instead of with hand-authored fixtures — every new
   seed or action kind automatically brings its own detection proof. Per-family floors
   assert the corpus's *shape* (each terminal family reached) rather than its volume,
   which keeps runtime bounded while making regime starvation a loud failure; requiring
   ≥2 contributing seeds per flag (or a recorded concession) prevents single-point
   coverage from presenting as a corpus property.
2. No backwards-compatibility aliasing/shims: floors replace the aggregate
   `terminal_kinds.len() >= 2` check rather than sitting beside it as a softer
   alternative.

## Verification Layers

1. INV-018 detection -> the new per-case tamper relation: for every generated case, a
   single-field payload perturbation yields `!matches_expected` (or agent-checksum
   divergence); a synthetic no-op perturbation proves the relation itself fails when
   detection is broken.
2. Per-family reach -> floors asserting sleep-success, sleep-interrupt, work-success,
   and work-fail terminals each nonzero across the corpus, counted from advance-emitted
   events (the -003 derivation).
3. Multi-seed contribution -> per-flag assertion that ≥2 distinct seeds contribute, or
   the explicit single-seed concession landed in the conformance row (one of the two —
   the implementer records which, per spec).
4. Bounded cost -> corpus remains seed-count × window-count bounded; no volume
   assertions added (manual review against spec §9's cost note).

## What to Change

### 1. Per-case metamorphic tamper relation

In `generative_lock.rs`: for each generated case, copy the honest log, rewrite one
payload field of one agent-stream event, and assert replay divergence; keep the honest
corpus untouched for the positive relations.

### 2. Per-family terminal floors

Replace `terminal_kinds.len() >= 2` with four per-family nonzero floors counted from
advance-emitted events; extend `support/generative.rs` seeding (seeds, masks, window
shapes) as needed so each family is genuinely reached — never by reintroducing
fabrication (the -003 guard enforces this).

### 3. Multi-seed contribution per flag

Track per-seed contributions to each reachability flag; assert ≥2 seeds per flag, or
record the concession for the named flag in the conformance row (coordinated with the
-003 row rewrite if both land close together).

## Files to Touch

- `crates/tracewake-core/tests/generative_lock.rs` (modify)
- `crates/tracewake-core/tests/support/generative.rs` (modify — seed/mask/window expansion as surfaced)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify — only if the single-seed concession path is taken)

## Out of Scope

- Counter derivation and the fabricator ban (ticket `-003`).
- Golden-fixture tamper gates in the content crate (already landed in 0018; unchanged).
- Mutation testing of the completion builders (ticket `-005`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test generative_lock` — tamper relation green
   (divergence required and observed per case); all four per-family floors green;
   multi-seed (or recorded-concession) check green.
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every generated case carries its own detection proof — an undetected single-field
   perturbation anywhere in the corpus fails the suite.
2. No terminal family can silently starve: a generator change that stops reaching
   sleep-success, sleep-interrupt, work-success, or work-fail fails the floors.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/generative_lock.rs` — per-case tamper relation;
   per-family floors; per-flag seed-contribution assertion.
2. `crates/tracewake-core/tests/support/generative.rs` — expanded seeding as surfaced
   by the floors.

### Commands

1. `cargo test -p tracewake-core --test generative_lock`
2. `cargo test --workspace`

## Outcome

Completed on 2026-06-11.

The generative oracle now requires every generated case to prove replay detection
by perturbing copied event payloads until checksum/replay divergence is observed,
while leaving the honest log used by the positive relations untouched. The old
aggregate `terminal_kinds.len() >= 2` floor was replaced with explicit nonzero
floors for sleep-success, sleep-interrupt, work-success, and work-fail terminals,
and each reachability flag now requires at least two distinct contributing
seeds. The bounded seed corpus was expanded with `0x18_00_00_21`,
`0x18_00_00_23`, and `0x18_00_00_24`; no conformance-row single-seed concession
was needed.

Verification:

1. `cargo test -p tracewake-core --test generative_lock`
2. `cargo test -p tracewake-core --test anti_regression_guards generative_lock_cannot_fabricate_duration_terminals`
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
