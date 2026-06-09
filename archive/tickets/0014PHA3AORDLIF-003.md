# 0014PHA3AORDLIF-003: Typed diagnostic schema (responsible layer / blocker code) + replay rebuild

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`agent/trace.rs` diagnostic structs + new enums, `events/envelope.rs` payload keys, `replay/rebuild.rs` rebuild, diagnostic producers in `agent/transaction.rs`/`agent/decision.rs`)
**Deps**: None

## Problem

Decision and stuck diagnostics are partly carried as English payload strings, so downstream proof surfaces (no-human metrics, fallback classification, hidden-truth audit) must scan text rather than read typed fields (see ORD-HARD-007: `projections.rs:216-217` matches `.contains("planner")` / `.contains("failed")`). INV-105 requires decision/stuck diagnostics to be typed or structurally inspectable substrate, with display strings only summarizing them. This ticket adds the typed diagnostic fields the spec's §5.4 names — `responsible_layer`, `blocker_code`, `input_source`, `actual_source`, `hidden_truth_referenced`, `remediation_hint` — to the decision/stuck events and makes replay rebuild them from those typed fields. It is the schema substrate that tickets 0014PHA3AORDLIF-004 (fallback → stuck naming `method_selection`) and -009 (metrics read typed enums) build on.

## Assumption Reassessment (2026-06-09)

1. `StuckDiagnostic` is at `crates/tracewake-core/src/agent/trace.rs:345`; `BlockerCategory` (an existing typed enum, candidate to extend or sit beside the new `BlockerCode`) is at `trace.rs:254`. The decision-trace record / `DecisionInput` live in `crates/tracewake-core/src/agent/decision.rs` and `trace.rs`. Events `DecisionTraceRecorded` / `StuckDiagnosticRecorded` are at `crates/tracewake-core/src/events/envelope.rs:116,129`; event payload is a `Vec<PayloadField>` of `{key, value: String}`. Replay rebuild lives in `crates/tracewake-core/src/replay/rebuild.rs` (+ `replay/report.rs`).
2. Spec §5.4 requires: "Decision/stuck events must carry typed `responsible_layer`, `blocker_code`, `input_source`, `actual_source`, `hidden_truth_referenced`, and `remediation_hint` fields where applicable" and "No-human metrics must rebuild from typed event fields and byte-match live metrics." §8 item 8 fixes the responsible-layer vocabulary: `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `projection`, `view_model`, `test_oracle`, `debug_quarantine`.
3. Shared boundary under audit: the typed diagnostic schema spanning the domain structs (`trace.rs`), their event serialization (`events/envelope.rs`), and replay reconstruction (`replay/rebuild.rs`). The contract: the typed fields are authoritative; any display string is derived, never the substrate. This is a substrate-only change — its enforcement consumers are sibling tickets, not this one.
4. Invariant motivating this ticket: **INV-105** (actor decision traces / stuck diagnostics / hidden-truth audits / why-not reports must be typed or structurally inspectable; display strings must not be the authoritative diagnostic substrate). Arch `docs/1-architecture/05_…NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md:159` requires stuck diagnostics to include blocker category, attempted action, routine/method ids, fallback/retry status, and debug-only details.
5. Deterministic-replay enforcement surface (substrate, enforced by siblings): the new typed fields serialize as reserved payload keys with enum-validated values, and `replay/rebuild.rs` must parse them back into the enums so replayed diagnostics equal live ones. This must stay deterministic (stable key order, enum round-trip, no wall-clock) so identical inputs + versions reproduce identical diagnostics (INV-018). The deferred enforcement surfaces that *consume* these fields are ticket -009 (no-human metrics byte-match) and ticket -004 (stuck diagnostic naming `method_selection`); this ticket introduces no leakage path (the fields carry only modeled/responsible-layer classification, never hidden truth values).
6. Schema extension — additive-vs-breaking: this **adds** optional typed fields to `StuckDiagnostic` / the decision-trace record and **adds** reserved payload keys to `DecisionTraceRecorded` / `StuckDiagnosticRecorded`. Consumers of these schemas: `no_human_day_metrics` (`projections.rs`), the no-human capstone assertions (`tests/no_human_capstone.rs`), the replay rebuild (`replay/rebuild.rs`), and the event-schema/replay gate (`tests/event_schema_replay_gates.rs`). Additive per the event-schema-evolution rule (INV-020); existing replay history without the keys must still load (older events default the fields rather than failing). Producers in `transaction.rs` / `decision.rs` populate the fields with best-effort typed values in this ticket; precise `method_selection` attribution lands in -004 and the metrics switch in -009.

## Architecture Check

1. Typed `ResponsibleLayer` / `BlockerCode` enums with enum-validated payload round-trip make the diagnostic substrate machine-checkable and replay-stable, eliminating the brittle English-substring proof surface at its root rather than per-consumer.
2. No backwards-compatibility shim: the typed fields are added additively (INV-020), but no parallel string-authority path is retained as the "real" substrate — strings become derived summaries only. Older replay history loads via field defaults, which is schema evolution, not an alias path.

## Verification Layers

1. INV-105 (typed diagnostic substrate) -> codebase grep-proof + schema validation: `ResponsibleLayer` / `BlockerCode` enums exist; `StuckDiagnostic` and the decision-trace record carry the typed fields; the reserved payload keys are emitted.
2. INV-018 / INV-020 (deterministic replay; additive schema evolution) -> replay/golden-fixture check: `tests/event_schema_replay_gates.rs` + `tests/no_human_capstone.rs` rebuild the typed diagnostics from event payloads and byte-match; pre-existing history without the keys still loads.
3. INV-105 (substrate not strings) -> invariants alignment check: the responsible-layer vocabulary matches the §8 execution-layer list exactly.

## What to Change

### 1. Typed diagnostic enums and fields

In `crates/tracewake-core/src/agent/trace.rs`, add `ResponsibleLayer` (the §8 vocabulary) and `BlockerCode` enums, and add `responsible_layer`, `blocker_code`, `input_source`, `actual_source`, `hidden_truth_referenced: bool`, `remediation_hint` to `StuckDiagnostic` and the decision-trace record where applicable.

### 2. Event serialization + replay rebuild

In `crates/tracewake-core/src/events/envelope.rs`, emit the typed values as reserved payload keys on `DecisionTraceRecorded` / `StuckDiagnosticRecorded`. In `crates/tracewake-core/src/replay/rebuild.rs`, parse those keys back into the enums so replayed diagnostics equal live ones; default missing keys for older history.

### 3. Producers populate the fields

In `agent/transaction.rs` and `agent/decision.rs`, set the typed fields at diagnostic-construction time with best-effort typed values (precise `method_selection` attribution is refined in -004).

## Files to Touch

- `crates/tracewake-core/src/agent/trace.rs` (modify — new enums + typed fields)
- `crates/tracewake-core/src/events/envelope.rs` (modify — reserved payload keys)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify — typed rebuild)
- `crates/tracewake-core/src/agent/transaction.rs` (modify — populate fields; **also touched by 002/004/005, coordinate**)
- `crates/tracewake-core/src/agent/decision.rs` (modify — populate fields; **also touched by 005, coordinate**)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify — typed-field replay rebuild assertion)

## Out of Scope

- Switching `no_human_day_metrics` to read the typed enums and removing the string scans (ticket 0014PHA3AORDLIF-009).
- The method-fallback fail-closed/re-run behavior and `method_selection` attribution precision (ticket 0014PHA3AORDLIF-004).
- The typed actor-known input refs / hidden-truth audit (ticket 0014PHA3AORDLIF-005).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test event_schema_replay_gates` — typed diagnostic fields serialize and replay-rebuild identically; older history without the keys still loads (additive).
2. `cargo test -p tracewake-core --test no_human_capstone` — diagnostics rebuild from typed payload under replay and byte-match.
3. `cargo test -p tracewake-core` — `ResponsibleLayer` / `BlockerCode` enums and the typed `StuckDiagnostic` fields compile and round-trip.

### Invariants

1. Decision/stuck diagnostics are typed substrate; display strings are derived, not authoritative (INV-105).
2. Typed fields replay-rebuild deterministically and additively; pre-existing history loads via defaults (INV-018, INV-020).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — assert typed diagnostic fields round-trip through event payload and replay rebuild, and that history lacking the reserved keys still loads.

### Commands

1. `cargo test -p tracewake-core --test event_schema_replay_gates`
2. `cargo test -p tracewake-core --test no_human_capstone`
3. `cargo test --workspace`

## Outcome

Completed on 2026-06-09.

- Added typed diagnostic substrate: `ResponsibleLayer`, `BlockerCode`, and `TypedDiagnosticFields`.
- Extended `DecisionTraceRecord` and `StuckDiagnostic` canonical forms with typed diagnostic fields while preserving legacy shorter-shape replay defaults.
- Emitted reserved typed payload keys on `DecisionTraceRecorded` and `StuckDiagnosticRecorded`.
- Validated typed payload keys during agent-event apply when present, while allowing older events without the keys.
- Populated transaction and scheduler stuck diagnostics with best-effort responsible layer and blocker code values.
- Added replay-gate assertions for typed payload fields, enum rebuild, old-shape decision trace defaults, stuck diagnostic typed round-trip, and exact responsible-layer vocabulary.

Verification run:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-core --test event_schema_replay_gates`
3. `cargo test -p tracewake-core --lib agent::trace::tests::every_blocker_category_serializes_and_round_trips`
4. `cargo test -p tracewake-core --test no_human_capstone`
5. `cargo test -p tracewake-core`
6. `cargo test --workspace`
7. `cargo clippy --workspace --all-targets -- -D warnings`
8. `cargo build --workspace --all-targets --locked`
