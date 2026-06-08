# 0008PHA3AANTCON-002: Typed decision-trace & stuck-diagnostic state, versioned events, replay

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core`: `AgentState` trace/diagnostic maps `String`→typed records; versioned event payloads/variants for `DecisionTraceRecorded`/`StuckDiagnosticRecorded`; replay rebuilds typed records
**Deps**: None

## Problem

Spec 0008 Finding 6 (D-0008-06): `agent/trace.rs` defines typed `DecisionTrace` (`trace.rs:49`), `HiddenTruthAudit` (`:37`), `StuckDiagnostic` (`:238`), but `AgentState` stores them as strings — `decision_traces: BTreeMap<DecisionTraceId, String>` and `stuck_diagnostics: BTreeMap<StuckDiagnosticId, String>` (`state.rs:136-137`). `events/apply.rs` applies `DecisionTraceRecorded` (`:252`) and `StuckDiagnosticRecorded` (`:265`) by storing canonical strings. A string cannot be authoritative diagnostic state; it blocks typed inspection, replay comparison, and debug projection.

This ticket makes the typed records authoritative in live and replay state **additively**: events carry structured payloads sufficient to rebuild the records, the maps become typed, canonical strings become a derived representation, and replay rebuilds and structurally compares the typed records. Per §4.1 / INV-020, the new/changed event kinds and payloads are versioned.

## Assumption Reassessment (2026-06-08)

1. `crates/tracewake-core/src/state.rs:136-137` holds the two `BTreeMap<..., String>` maps; `events/apply.rs:252,265` apply the two event kinds as strings; `agent/trace.rs:49/:37/:238` define the typed `DecisionTrace`/`HiddenTruthAudit`/`StuckDiagnostic`; `DecisionTraceId`/`StuckDiagnosticId` exist at `ids.rs:153-154`.
2. Spec §8.5 + §9.5 list the required record fields (`DecisionTraceRecord`, `StuckDiagnosticRecord`) and mandate "Preserve deterministic canonical serialization as a derived representation" and "Replay must rebuild typed records … then compare live/replay state structurally." §13 acceptance: "Decision traces and stuck diagnostics are typed authoritative state" and (added during reassessment) "New/changed trace-diagnostic event kinds and payloads are versioned per INV-020." §10.6 supplies the replay-parity gates this ticket carries.
3. Cross-artifact boundary under audit: the **event payload ↔ `AgentState` ↔ replay** contract for the two trace-diagnostic event kinds — the producing event (`events/envelope.rs`), the applier (`events/apply.rs`), the authoritative store (`state.rs`), and the replay rebuilder (`replay/rebuild.rs`, `replay/report.rs`).
4. INV-018 (Deterministic replay is foundational): replay must reconstruct typed records byte-identically from ordered events. INV-020 (Event schema evolution is mandatory): event kinds/payloads must be versioned so replay rejects unsupported history rather than inventing repairs.
5. Enforcement surface: deterministic replay + canonical serialization/checksum. Confirm the change keeps identical inputs+versions byte-identical: the typed records serialize through the existing canonical/sorted boundary; the derived display string is computed deterministically from the typed record (not stored as the source of truth); no wall-clock or hash-map iteration enters the canonical form. No epistemic leakage — these are diagnostic records, rendered to debug/TUI by 0008PHA3AANTCON-010 with actor-knowledge filtering, not here.
6. Extends event-record + state schemas. `DecisionTraceRecorded`/`StuckDiagnosticRecorded` payloads gain structured fields (was: canonical string). Consumers: `events/apply.rs` (applier), `state.rs` (store), `replay/rebuild.rs` + `replay/report.rs` (replay), `checksum.rs` (canonical hashing), `debug_reports.rs`/`view_models.rs` (read — updated in -010), `scheduler.rs` (emitter — rewired in -006). **Breaking** to the event payload shape → handled via INV-020 version bump on the two event kinds; the derived canonical string is retained additively for checksum/display continuity.
7. Changes the authoritative type of `AgentState::decision_traces` / `stuck_diagnostics` (String→typed). Blast radius grep at implementation: `grep -rn "decision_traces\|stuck_diagnostics" crates/` — every read site (replay comparison, debug reports, scheduler emit, tests) joins Files to Touch or is owned by its ticket (scheduler emit → -006; debug read → -010).

## Architecture Check

1. Typed authoritative records with a derived canonical string is strictly more robust than string-authoritative state: replay can compare structurally, debug can project fields, and tampering the display text cannot alter facts. Keeping the string as derived (not removing it) preserves the existing checksum/display path without making it the source of truth.
2. No backwards-compatibility aliasing: the `String` maps are replaced, not kept beside typed maps. The version bump is schema evolution per INV-020, not a compatibility shim.

## Verification Layers

1. INV-018 deterministic replay → replay/golden-fixture check: replay of a no-human golden scenario rebuilds typed `DecisionTraceRecord`/`StuckDiagnosticRecord` maps equal to live state (§10.6 gates 1–2).
2. INV-020 schema evolution → codebase grep-proof: the two event kinds carry an explicit version; replay rejects an unsupported version rather than defaulting.
3. Determinism of derived form → unit test: the canonical display string is a pure function of the typed record; tampering the string does not change the typed facts/checksum, tampering a typed field does change the checksum/replay comparison (§10.6 gates 3–4).
4. Single boundary proof → manual review: `state.rs` stores only typed records; no code path writes an authoritative string diagnostic.

## What to Change

### 1. Typed authoritative maps in `AgentState`

Change `state.rs:136-137` to `BTreeMap<DecisionTraceId, DecisionTraceRecord>` and `BTreeMap<StuckDiagnosticId, StuckDiagnosticRecord>` (record types per §8.5, hosted in `agent/trace.rs`).

### 2. Versioned structured event payloads

Give `DecisionTraceRecorded` / `StuckDiagnosticRecorded` (`events/envelope.rs`, applied at `events/apply.rs:252,265`) structured payloads sufficient to rebuild the records, with an explicit schema version (INV-020). Applier builds the typed record from payload.

### 3. Replay rebuild + structural comparison

`replay/rebuild.rs` reconstructs the typed maps; `replay/report.rs` compares live vs replay structurally (not by string). Canonical serialization/checksum (`checksum.rs`) derives the display string from the typed record.

## Files to Touch

- `crates/tracewake-core/src/agent/trace.rs` (modify — add `DecisionTraceRecord`/`StuckDiagnosticRecord` authoritative types + derived-string fn)
- `crates/tracewake-core/src/state.rs` (modify — typed maps)
- `crates/tracewake-core/src/events/envelope.rs` (modify — versioned structured payloads)
- `crates/tracewake-core/src/events/apply.rs` (modify — build typed records from payload)
- `crates/tracewake-core/src/events/log.rs` (modify — if payload construction helpers live here)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify — rebuild typed maps)
- `crates/tracewake-core/src/replay/report.rs` (modify — structural comparison)
- `crates/tracewake-core/src/checksum.rs` (modify — derive canonical string from typed record)

## Out of Scope

- Emitting these records from the transaction/scheduler (0008PHA3AANTCON-004/-006).
- Debug/TUI rendering of the typed records (0008PHA3AANTCON-010).
- The structured why-not failure-kind taxonomy beyond fields needed to store the record (its consumer surfacing is in -010).

## Acceptance Criteria

### Tests That Must Pass

1. Replay-parity test: live and replay typed `DecisionTraceRecord` maps are structurally equal over a no-human golden scenario; same for `StuckDiagnosticRecord` (§10.6 gates 1–2).
2. Determinism test: tampering canonical string text does not change typed facts; tampering a typed field changes checksum/replay comparison (§10.6 gate 4).
3. `cargo test --workspace` green; deterministic-replay regression tests pass.

### Invariants

1. `AgentState` stores only typed trace/diagnostic records; canonical strings are derived.
2. The two event kinds are versioned; replay rejects unsupported versions.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/replay/report.rs` tests — structural live/replay equality of typed maps.
2. `crates/tracewake-core/tests/golden_scenarios.rs` — extend replay assertions to typed records.
3. `crates/tracewake-core/src/agent/trace.rs` tests — derived-string determinism and tamper sensitivity.

### Commands

1. `cargo test -p tracewake-core replay::`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-08

Implemented typed authoritative trace/diagnostic state. `AgentState::decision_traces` now stores `DecisionTraceRecord`, and `AgentState::stuck_diagnostics` stores `StuckDiagnosticRecord`. Canonical strings remain derived through `serialize_canonical()` for checksum and debug display. `DecisionTraceRecorded` and `StuckDiagnosticRecorded` now require payload schema version `1` and parse typed records before storing them; unsupported versions fail closed. Replay rebuilds typed maps through the normal event applier and existing live/replay structural equality checks now compare typed records.

Deviations from original plan: `StuckDiagnosticRecord` is a type alias to the existing typed `StuckDiagnostic` because that type already had the required authoritative fields and canonical round-trip implementation. Scheduler still emits the current post-proposal/post-window records; transaction-owned emission remains with later tickets.

Verification:

- `cargo test -p tracewake-core replay::`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
