# 0006PHA2AEPISUB-006: Extend replay/event-schema/fixture/TUI adversarial epistemic tests

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — extends existing event-schema/replay gates, TUI adversarial gates, content forbidden/provenance tests, and epistemic golden fixtures with adversarial coverage. No production logic.
**Deps**: 0006PHA2AEPISUB-003

## Problem

The sealing tickets (-001/-002/-003) and lock-layer tickets (-004/-005) prove structural API integrity, but the spec's §7 test table and EPI-HARD-004 req. 4–5 also require behavioral adversarial coverage that does not yet exist: every epistemic event kind must appear in the schema-registry/replay tests with unsupported versions staying loud; actor-visible why-not output must be proven not to pick up debug summary strings; debug truth must be proven absent from embodied/notebook output. These extend existing suites rather than adding new ones (spec §7 "Harden existing fixtures and suites where they already exist. Do not author parallel fixture families").

## Assumption Reassessment (2026-06-09)

1. The target suites all exist: `crates/tracewake-core/tests/event_schema_replay_gates.rs`, `crates/tracewake-tui/tests/adversarial_gates.rs`, `crates/tracewake-content/tests/forbidden_content.rs`, and the epistemic golden fixtures under `crates/tracewake-content/src/fixtures/` (e.g. `expectation_contradiction_001.rs`, `sound_uncertainty_001.rs`, `knowledge_blocker_accuse_001.rs`, `possession_parity_001.rs`, `view_filtering_001.rs`, `no_human_epistemic_check_001.rs`, `prose_born_fact_rejected_001.rs`). All confirmed present this session.
2. Spec authority: `specs/0006_…SPEC.md` §7 test/fixture table + §6 EPI-HARD-004 req. 4–5. The sealed APIs these tests exercise must already exist — hence `Deps: 0006PHA2AEPISUB-003` (transitive head over -002/-001).
3. Shared boundary under audit: the epistemic event-kind ↔ `EventStream::Epistemic` ↔ schema-registry/replay contract, the why-not actor-visible/debug audience split, and the embodied-vs-debug view-model boundary. These are exercised end-to-end, not modified.
4. Constitutional invariants motivating this ticket: `INV-018`/`INV-020` (deterministic replay; unsupported schema versions rejected loudly, not silently repaired), `INV-070` (why-not distinguishes actor-known blockers from debug detail), `INV-068`/`INV-107` (debug truth never reaches embodied output), `INV-006`/`INV-094` (possession transfers no belief).
5. Enforcement surface: deterministic-replay and actor-knowledge-filtering behavior. The tests strengthen leakage/determinism prevention by adversarially probing them; they add no production logic and cannot themselves weaken any surface.

## Architecture Check

1. Extending the existing gate suites and golden fixtures keeps the adversarial coverage co-located with the behavior it guards and avoids a parallel fixture family the spec explicitly forbids. Asserting against typed reason codes / structured views (not display strings) matches the doctrine that display strings are not an authority boundary.
2. No backwards-compatibility shims; tests are additive over sealed APIs.

## Verification Layers

1. `INV-020`/`INV-018` (loud schema rejection, deterministic replay) -> replay/golden-fixture check in `event_schema_replay_gates.rs`: every epistemic event kind appears in the schema registry/replay test; an unsupported epistemic event/record schema version is reported and not applied.
2. `INV-070` (why-not audience split) -> manual review + typed assertion in TUI `adversarial_gates.rs`: actor-visible why-not for a blocked accuse contains no debug summary string and no culprit/accused identity.
3. `INV-068`/`INV-006` (debug/possession leakage) -> TUI seam/adversarial test: embodied `notebook`/view output excludes debug truth; rebinding a controller transfers no prior actor's beliefs/notebook.
4. `INV-024` (typed proposition, no prose-born fact) -> content fixture/`forbidden_content.rs`: sound creates a low-confidence observation/possible-movement proposition, not theft/culprit; prose-born/typeless propositions cannot create an authoritative belief.

## What to Change

### 1. Event-schema / replay coverage

Extend `crates/tracewake-core/tests/event_schema_replay_gates.rs` to assert every epistemic `EventKind` maps to `EventStream::Epistemic` and round-trips in replay, and that unsupported epistemic event/record schema versions are reported and not applied.

### 2. TUI adversarial coverage

Extend `crates/tracewake-tui/tests/adversarial_gates.rs` to assert actor-visible why-not strings never contain debug summaries, embodied output never contains debug truth, and possession rebinding transfers no belief.

### 3. Content provenance / no-script coverage

Extend `crates/tracewake-content/tests/forbidden_content.rs` and the relevant epistemic fixtures with negative cases for nested/renamed/field-alias shortcut truth and the sound-uncertainty / absence-requires-expectation cases from the §7 table.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify)
- `crates/tracewake-content/src/fixtures/` (modify — extend existing epistemic fixtures named in Assumption Reassessment item 1; new adversarial fixture files only if an existing one cannot carry the case)

## Out of Scope

- The sealing itself (0006PHA2AEPISUB-001/-002/-003).
- Compile-fail negative fixtures (0006PHA2AEPISUB-004) and source-guard/clippy extension (0006PHA2AEPISUB-005).
- The final acceptance artifact (0006PHA2AEPISUB-007).
- Authoring any new parallel fixture family for symmetry (spec §7 forbids it).

## Acceptance Criteria

### Tests That Must Pass

1. `event_schema_replay_gates` asserts coverage of every epistemic event kind and loud rejection of unsupported epistemic schema versions; counts are re-enumerated from the event-kind set at test start, not hardcoded.
2. TUI `adversarial_gates` proves actor-visible why-not / embodied output carries no debug truth or debug summary string, and possession rebinding transfers no belief.
3. `forbidden_content` + the extended fixtures prove shortcut-truth aliases and prose-born facts are rejected and sound-only evidence cannot satisfy a culprit/accuse precondition.
4. `cargo test --workspace` passes.

### Invariants

1. Adversarial coverage asserts against typed reason codes / structured views, not display strings.
2. No parallel fixture family is created; existing suites/fixtures are extended.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — epistemic event-kind coverage + loud-schema-rejection cases.
2. `crates/tracewake-tui/tests/adversarial_gates.rs` — why-not/embodied debug-leak + possession-parity cases.
3. `crates/tracewake-content/tests/forbidden_content.rs` + `crates/tracewake-content/src/fixtures/*` — alias shortcut-truth + sound-uncertainty + absence-requires-expectation cases.

### Commands

1. `cargo test -p tracewake-core --test event_schema_replay_gates`
2. `cargo test -p tracewake-tui --test adversarial_gates && cargo test -p tracewake-content --test forbidden_content`
3. `cargo test --workspace` — full-pipeline confirmation.

## Outcome

Completed: 2026-06-09

What changed:
- Extended `event_schema_replay_gates` with dynamic epistemic event-kind registry/stream coverage and unsupported epistemic payload-schema replay rejection.
- Strengthened TUI adversarial gates so actor-facing view/notebook/why-not surfaces reject debug markers, debug provenance strings, and prior-actor belief identifiers after possession rebinding.
- Extended content shortcut-truth rejection with nested/renamed/alias keys and added those aliases to the existing forbidden script-key list.

Deviations from original plan:
- No fixture files needed changes; the existing validation and TUI fixtures carried the new cases.
- The content alias rejection required a small validation allowlist hardening, not only test assertions.

Verification:
- `cargo fmt --all --check`
- `cargo test -p tracewake-core --test event_schema_replay_gates`
- `cargo test -p tracewake-tui --test adversarial_gates`
- `cargo test -p tracewake-content --test forbidden_content`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
