# 0004PHA2AEPISUB-016: Capstone — Phase 2A acceptance gates, possession parity, no-human run, and replay determinism

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds Phase 2A acceptance/transcript/property/golden tests and the `docs/4-specs/SPEC_LEDGER.md` Spec 0004 entry.
**Deps**: 0004PHA2AEPISUB-011, 0004PHA2AEPISUB-014, 0004PHA2AEPISUB-015

## Problem

Phase 2A is accepted only when all gates pass (Spec 0004 §16, §22): the mechanic gate (typed propositions, event-backed observations, source-backed beliefs, expectation contradiction, uncertain sound), the TUI/view-model gate, the possession-parity gate (`INV-094`), the no-human gate (`INV-091`), the replay gate (`INV-092`), the validation gate, and the CI gate. This capstone exercises every prior implementation ticket end-to-end, adds the cross-cutting property/transcript/golden tests, and records the Spec 0004 ledger entry. It introduces no new production logic.

## Assumption Reassessment (2026-06-06)

1. The acceptance/test harnesses exist: `crates/tracewake-core/tests/acceptance_gates.rs`, `crates/tracewake-core/tests/golden_scenarios.rs`, `crates/tracewake-tui/tests/tui_acceptance.rs`, `crates/tracewake-tui/tests/transcript_snapshot.rs`, and `crates/tracewake-content/tests/golden_fixtures_run.rs` are the surfaces this capstone extends. `docs/4-specs/SPEC_LEDGER.md` has no Spec 0004 entry yet (confirmed) and its "Next allowed spec" note authorizes opening Phase 2 work.
2. The gate set is fixed by Spec 0004 §16/§22; the golden fixtures come from ticket 013, the sound slice from 014, the TUI commands from 011, the why-not from 008, replay determinism from 005. Counts are re-enumerated from fixtures at test start, not hardcoded.
3. Shared boundary under audit: this capstone is the deliverable-doubles-as-capstone case — it ships test infrastructure (property/transcript/golden tests) and the ledger entry, exercising tickets 001–015 without modifying their files. `Deps` are the leaf set (011, 014, 015) whose transitive `Deps` cover the full DAG.
4. Invariant motivating this ticket: `INV-098` (feature acceptance is harsh — caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe, LLM-independent, non-scripted, regression-tested), plus `INV-091`/`INV-092`/`INV-093`/`INV-094`/`INV-095` (no-human, replay determinism, leakage, possession parity, TUI acceptance are all tested).
5. Actor-knowledge / deterministic-replay surface: the capstone is the enforcing test bed for the no-leak and determinism invariants — it asserts debug truth never appears in any embodied view/notebook (`INV-093`), possession switch copies no beliefs (`INV-094`), the same event log yields different actor-filtered views, replay rebuilds epistemic projections byte-identically (`INV-092`), and the no-human scheduler-origin check runs without controller/player references (`INV-091`). It adds no production logic and no nondeterminism; it pins expected behavior.

## Architecture Check

1. A single capstone exercising the composed pipeline (plus distributed property/transcript tests) verifies the phase end-to-end without duplicating logic, and is the correct home for the cross-cutting possession-parity/no-human/leak/determinism proofs that no single implementation ticket fully exercises. The ledger flip lives here because it is gated on this exit evidence passing.
2. No backwards-compatibility shims: tests and a ledger entry only; no production code changes.

## Verification Layers

1. Possession parity (`INV-094`) -> property test: a possession switch from Mara to Tomas copies no beliefs; `possession_parity_001` proves Tomas cannot truthfully accuse Mara.
2. No leakage (`INV-093`) -> leak/transcript test: across the §15.5 transcript session, no embodied view/notebook contains a culprit/ground-truth/previous-possession string before modeled discovery.
3. No-human run (`INV-091`) -> replay/golden check: `no_human_epistemic_check_001` runs a scheduler-origin check through the same pipeline with no `player`/controller references; replay rebuilds the same epistemic state.
4. Replay determinism (`INV-092`) -> replay/hash check: epistemic projection rebuild is byte-identical; unsupported event/proposition/projection versions reject loudly.
5. TUI acceptance (`INV-095`) -> transcript test: the §15.5 session (`bind actor_mara` → take → `bind actor_tomas` → `view`/`notebook`/`check strongbox_tomas`/`notebook`/`debug epistemics`/`debug beliefs`/`debug observations`/`debug replay`) is reachable and deterministic.

## What to Change

### 1. Mechanic + validation gate tests

Extend `crates/tracewake-core/tests/acceptance_gates.rs` and `crates/tracewake-content/tests/golden_fixtures_run.rs` with the §16.1/§16.6 assertions (typed propositions canonical, observations event-backed, beliefs holder/source-backed, contradiction works, sound uncertain; content validates seeds and rejects shortcut-truth).

### 2. Property + golden tests

Add the §15.2 property tests (no important belief lacks holder/source; debug truth never in embodied view; possession switch never copies beliefs; same log → different actor-filtered views; cannot accuse from hidden truth; replay rebuilds belief projection; deterministic ordering; physical location single-source while beliefs diverge) and the §15.6 golden tests for all seven Phase 2A fixtures, in `acceptance_gates.rs`/`golden_scenarios.rs`.

### 3. TUI transcript acceptance + no-human/replay

Extend `crates/tracewake-tui/tests/tui_acceptance.rs`/`transcript_snapshot.rs` with the §15.5 transcript session and leak assertions, and add the no-human epistemic run + replay-determinism assertions over `no_human_epistemic_check_001`.

### 4. SPEC_LEDGER entry

Add the Spec 0004 entry to `docs/4-specs/SPEC_LEDGER.md` marked Phase 2A (not full Phase 2), stating what remains deferred (Phase 2B+/3/4 scope), gated on this capstone's exit evidence.

## Files to Touch

- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — mechanic/property/gate assertions)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify — seven Phase 2A golden scenarios)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — validation/seed gate assertions)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify — §15.5 transcript session + leak assertions)
- `crates/tracewake-tui/tests/transcript_snapshot.rs` (modify — deterministic epistemic transcript)
- `docs/4-specs/SPEC_LEDGER.md` (modify — Spec 0004 Phase 2A entry)

## Out of Scope

- Any new production logic (this is acceptance/verification only).
- README/fixture-contract prose (ticket 015).
- Phase 2B/3/4 scope (explicit non-goals, Spec §17).

## Acceptance Criteria

### Tests That Must Pass

1. The full §15.5 TUI transcript session runs deterministically; before Tomas checks, his view/notebook lacks Mara-only knowledge; after, the notebook shows the source-backed missing-property belief and names no culprit.
2. Possession switch copies no beliefs; no-human epistemic check runs without player/controller references; replay rebuilds physical + epistemic projections byte-identically and rejects unsupported versions.
3. The CI gate passes: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`; existing Phase 1/1A fixtures and commands still pass.

### Invariants

1. All §16 Phase 2A gates hold; no embodied surface leaks hidden truth.
2. The ledger entry marks Phase 2A only and names deferred scope.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_gates.rs` — property + mechanic gate assertions.
2. `crates/tracewake-core/tests/golden_scenarios.rs` — seven Phase 2A golden scenarios.
3. `crates/tracewake-tui/tests/tui_acceptance.rs` — §15.5 transcript session + leak assertions.

### Commands

1. `cargo test --workspace`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked`
3. `cargo test -p tracewake-tui --test tui_acceptance` (the end-to-end Phase 2A acceptance boundary)

## Outcome

Completed on 2026-06-07.

Changed:

1. Added capstone assertions for Phase 2A epistemic event registration, nonphysical event streams, holder/source-backed initial beliefs, forbidden shortcut-truth validation, all seven Phase 2A golden fixture contracts, no-human scheduler-origin checks, and the TUI no-leak absence-discovery transcript.
2. Added the Spec 0004 entry to `docs/4-specs/SPEC_LEDGER.md`, explicitly marked as Phase 2A only with Phase 2B+/3/4 scope deferred.
3. Repaired Phase 2A fixture reachability where capstone tests found advertised `check_container` paths on closed opaque containers without an ordinary opening/visibility route.
4. Repaired stale TUI command-loop tests so they use stable semantic IDs or the correct current numeric slot after `check.container.*` became visible in the action menu.
5. Cleared clippy findings in the Phase 2A pipeline/detection code needed for the full CI gate.

Deviations:

1. Although the ticket was intended as tests plus ledger only, the capstone exposed fixture-contract mismatches: several Phase 2A fixtures advertised check/search behavior that was physically rejected until an ordinary `open` affordance or closed-visible fixture setup was added.
2. Full clippy was first run at capstone time and surfaced two pre-existing Phase 2A warnings; they were fixed here because the capstone owns the CI exit gate.

Verification:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace --all-targets --locked`
4. `cargo test --workspace`
5. `cargo test -p tracewake-tui --test tui_acceptance`
6. `cargo test -p tracewake-content --test golden_fixtures_run`
7. `cargo test -p tracewake-core --test golden_scenarios`
8. `git diff --check`
