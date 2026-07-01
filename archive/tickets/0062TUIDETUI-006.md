# 0062TUIDETUI-006: Intent conformance and script-driver fixtures (capstone)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — new `tracewake-tui` conformance/fixture test binding all three producers to one reducer. No production logic; no core types, no event/schema/view-model changes.
**Deps**: 0062TUIDETUI-005.

## Problem

Spec 0062 §4.6 requires a conformance test binding all three producers (keyboard — future Spec 0065 — key script, semantic script) to one reducer, and §6 requires the fixtures: a key script and an equivalent semantic script producing byte-identical screen dumps; a script exercising wait, continue, focus/selection movement, action selection, and notebook focus with asserted dumps and transcript; a driven debug-authority command asserting the debug marker and no embodied-dump leakage; a source guard that a disallowed direct mutation has no code path; and determinism (same script → same transcript/dumps). This capstone is the spec's acceptance evidence (§7): the key-vs-semantic dump equivalence, the driven transcript, and the no-direct-dispatch guard result.

## Assumption Reassessment (2026-07-01)

1. **Producers and reducer under test (code).** This ticket exercises the landed intent stack: `intent::key_script` (003), `intent::semantic_script` (004), `intent::reducer` (002), and `intent::headless` (005), rendering through the Spec 0061 screen-dump seam (`crates/tracewake-tui/src/screen/`). It adds no production logic; it composes what the prior tickets built.
2. **Test location (code).** TUI integration tests live under `crates/tracewake-tui/tests/` (e.g. `tui_seam_conformance.rs`, `transcript_snapshot.rs`, `embodied_screen_dump.rs`). This ticket adds a new `crates/tracewake-tui/tests/intent_conformance.rs`. The existing `tui_seam_conformance.rs` guards source/compile discipline (wildcard/default laundering) and is the exemplar for the no-direct-dispatch source guard.
3. **Cross-artifact boundary under audit.** The conformance test is the single seam proving §4.1 (one intent stream, three producers cannot fork semantics) and §4.2 (no intent mutates world state directly). It asserts the key and semantic producers converge on byte-identical dumps and that world-advancing intents route only through authorized methods.
4. **Motivating invariants (INV-095, INV-024, INV-018).** `INV-095` — this TUI/view-model test is an acceptance test. `INV-024` — the anti-leak fixture proves embodied dumps carry no debug-only truth. `INV-018` — the determinism fixture proves byte-stable dumps/transcripts. `INV-069` — the source guard proves no world-mutation path bypasses the reducer.
5. **No-leak / determinism / no-bypass enforcement surface.** As an acceptance surface this ticket *reads* the view-model projection, transcript, and dispatch surfaces rather than modifying them: it must include a behavior witness for each guarantee (a negative fixture where debug truth would leak into an embodied dump fails; a determinism assertion across runs; a compile/source guard that a direct `event`-mutating path from an intent does not exist). The tests introduce no leakage or nondeterminism path themselves and add no authoritative surface.

## Architecture Check

1. A single conformance ticket that binds all three producers to one reducer and carries the §6 fixtures is the acceptance capstone (deliverable-doubles-as-capstone: the conformance harness is itself the spec deliverable) — cleaner than scattering the equivalence/anti-leak/determinism proofs across the producer tickets, since the key-vs-semantic equivalence can only be asserted once both parsers and the runner exist.
2. No backwards-compatibility aliasing or shims; the test asserts the legacy line grammar is not a hidden alias into the reducer.

## Verification Layers

1. INV-095 (TUI tests are acceptance) -> replay/golden check: key-script and semantic-script runs produce byte-identical screen dumps; the driven wait/continue/action/notebook script asserts dumps + transcript sections.
2. INV-024 (no telepathy) -> negative-fixture test: an embodied dump/transcript with underlying debug truth shows no debug-only world truth; removing the guard fails.
3. INV-069 / INV-104 (no direct dispatch) -> codebase grep-proof / compile guard: no intent reaches a world change except through the reducer's authorized methods; a disallowed direct mutation has no code path.
4. INV-018 (determinism) -> replay check: the same script yields the same transcript and dumps across runs.

## What to Change

### 1. Add the conformance and fixtures test

Create `crates/tracewake-tui/tests/intent_conformance.rs` with:
- **Dump equivalence**: a key script and an equivalent semantic script driven through the headless runner produce byte-identical screen dumps.
- **Driven flow**: a script exercising wait, continue, focus/selection movement, action selection, and notebook focus; assert the resulting dumps (including focused pane and selection) and transcript sections.
- **Debug authority**: a script driving a debug-authority command (e.g. `overlay`, `log`) via `SubmitDebugCommand`; assert the debug marker is present and no debug output appears in an embodied dump.
- **No-direct-dispatch guard**: a source/compile guard (mirroring `tui_seam_conformance.rs`) that intents route only through authorized methods — a disallowed direct world mutation from an intent has no code path.
- **Determinism**: the same script yields the same transcript and dumps across two runs.

## Files to Touch

- `crates/tracewake-tui/tests/intent_conformance.rs` (new)

## Out of Scope

- Any production-logic change to the intent stack (001–005) — this ticket exercises it, it does not modify it.
- The Architecture-10 transcript-clause doctrine amendment (spec §5; cross-spec, owned by Spec 0070).
- The spec's `SPEC_LEDGER.md` row and `archive/specs/` move (deferred to spec acceptance per the feature-spec archival convention).
- Any `crossterm` / fullscreen-shell acceptance (Spec 0065).

## Acceptance Criteria

### Tests That Must Pass

1. **Key-vs-semantic equivalence**: byte-identical screen dumps from the two producers driving the same scenario.
2. **Driven transcript**: wait, continue, focus/selection, action selection, and notebook focus each produce the asserted dump + transcript sections.
3. **Debug authority**: a driven debug command shows the debug marker and leaks no debug truth into an embodied dump.
4. **No-direct-dispatch guard**: the source/compile guard proves no intent bypasses the reducer's authorized methods.
5. **Determinism**: same script → same transcript and dumps across runs.
6. `cargo test -p tracewake-tui --test intent_conformance` and the full `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`.

### Invariants

1. The three producers cannot fork semantics; all converge on one reducer (INV-095, spec §4.1).
2. Embodied dumps leak no debug-only truth (INV-024); no intent mutates world state directly (INV-069/INV-104); identical scripts are byte-stable (INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/intent_conformance.rs` — the full §6 fixture set plus §4.1–4.2 conformance; rationale: this is the spec's acceptance evidence (§7) binding all producers to one reducer.

### Commands

1. `cargo test -p tracewake-tui --test intent_conformance`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-07-01

Added `crates/tracewake-tui/tests/intent_conformance.rs` as the Spec 0062
capstone fixture set. The test proves key-script and semantic-script producers
converge on byte-identical screen dumps, drives focus/selection/action
submission/wait/continue/notebook through the headless runner, verifies debug
authority through `UiIntent::SubmitDebugCommand` and the reducer while keeping
debug output out of embodied dumps, guards the reducer source against direct
world-mutation tokens, and asserts byte-identical transcript/dump determinism
for repeated runs.

The capstone adds no production logic, core type, event/schema/view-model
change, or new world authority. Debug command coverage is driven directly
through the reducer because the ticket-003/004 script grammars intentionally
remain minimal and closed.

Verification:

- `cargo test -p tracewake-tui --test intent_conformance`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
