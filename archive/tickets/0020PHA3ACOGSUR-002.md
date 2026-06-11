# 0020PHA3ACOGSUR-002: Derived apply-arm payload-version census and per-arm dispositions

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`events/apply`, derived censuses in `anti_regression_guards.rs`, event builders as surfaced if arms gate); `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` census row; golden repricing in `tracewake-content` if any payload is stamped (batched after `-001`)
**Deps**: `archive/tickets/0020PHA3ACOGSUR-001.md` (golden-churn batching per spec §8); `archive/specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md` (ORD-HARD-054)

## Problem

0019's `ORD-HARD-047` required both payload censuses be **derived structurally**
("every apply arm containing an insert into an `AgentState` map must call the
version-requiring helper… or register a typed-column-closure exemption with
rationale"). The record-struct census was derived; the apply-arm census was not — 
`materialized_agent_apply_arms_require_payload_schema_version`
(`anti_regression_guards.rs`) is still a literal four-name list
(`need_threshold_crossings`, `ordinary_life_episodes`, `candidate_goal_evaluations`,
`continue_routine_arbitrations`), blind to every other arm that materializes
checksum-covered `AgentState` — the precise hand-enumeration defect shape
`ORD-HARD-047` named, surviving its own correction (`ORD-HARD-054`, INV-020).

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `96bc387`: `events/apply.rs` writes exactly **11
   checksum-covered `AgentState` map families** (all present in
   `checksum.rs::AGENT_STATE_CHECKSUM_COVERAGE`): version-gated with
   `payload_schema_version` — `need_threshold_crossings`, `ordinary_life_episodes`
   (multi-kind episode arm incl. `FoodServiceUsed`), `candidate_goal_evaluations`,
   `continue_routine_arbitrations`; gated with alternate keys —
   `decision_traces` (`trace_schema_version`), `stuck_diagnostics`
   (`diagnostic_schema_version`); **ungated** — `needs_by_actor` + `need_tick_charges`
   (`apply_need_delta`), `intentions` + `active_intention_by_actor`
   (`apply_intention_started` / `apply_intention_transition`), `routine_executions`
   (`apply_routine_step_transition`). No exemption registry exists.
2. Verified against spec 0020 (reassessed 2026-06-11): ORD-HARD-054's correction is
   the derived census plus per-arm gate-or-exemption; the census conformance row
   lands with this ticket per the approved distribute-docs decision.
3. Cross-artifact boundary under audit: the materialized-agent version contract
   spanning the apply arms (`events/apply.rs`), the coverage table
   (`checksum.rs::AGENT_STATE_CHECKSUM_COVERAGE`), the record structs (`state.rs`),
   and the censuses (`anti_regression_guards.rs`) — every covered-map write is
   version-gated or explicitly exempted, with no silent third state.
4. INV-020 restated: event kinds and payloads must be versioned enough that replay
   can reject unsupported history rather than silently inventing repairs. The
   census's job is to make "neither gated nor exempted" a CI failure, not a
   reviewer's recollection.
5. Deterministic-replay surface touched: any arm that chooses *gating* stamps
   `payload_schema_version` on its builder's payload, changing event bytes and
   repricing agent checksums/goldens — batched with `-001`'s churn (spec §8/§9);
   forged-version negative gates (the
   `forged_threshold_payload_schema_version_rejected_…` pattern) add loud rejection,
   strengthening replay. Arms that choose *exemption* change no bytes. No
   epistemic-leakage direction is touched.
6. Schema shape: gating arms is an additive payload extension (a new
   `payload_schema_version` entry; consumers are the arm itself and the negative
   gates); exempting arms is no shape change. Per-arm disposition recorded — the
   spec assigns the gate-vs-exempt choice to the implementer as a recorded decision
   (spec §9: "the census forces the choice to be explicit; it does not pre-decide
   it"), recorded in the exemption registry rationales and the acceptance artifact
   (spec §7.3).

## Architecture Check

1. A structurally derived census (scan `events/apply.rs` source for inserts into
   `AGENT_STATE_CHECKSUM_COVERAGE` map names; require an in-arm version-helper call
   accepting the `trace_`/`diagnostic_` key variants, or membership in an explicit
   `TYPED_COLUMN_CLOSURE_EXEMPTIONS` registry with per-arm rationale) is the only
   shape that closes the family: a hand-list, however complete today, re-mints the
   `ORD-HARD-047`→`054` defect on the next materialized kind. Mirrors the proven
   derivation pattern of `materialized_agent_payload_records_keep_payload_fields`.
2. No backwards-compatibility aliasing/shims: no default-version fallback, no silent
   schema defaults (spec §8 constraint); an unversioned payload on a gated arm is a
   loud `BadPayload` rejection live and in replay.

## Verification Layers

1. INV-020 (census closure) -> derived census test: every covered-map write site in
   `events/apply.rs` is gated or exempted; a synthetic-regression case (an arm with
   neither) fails the guard.
2. INV-020 (loud rejection) -> forged-version negative replay gate per newly gated
   arm, asserting live `BadPayload` and replay `!matches_expected` (the
   `forged_threshold…_001` pattern).
3. Exemption honesty -> registry test: every `TYPED_COLUMN_CLOSURE_EXEMPTIONS` entry
   names its arm, its typed columns, and a non-empty rationale; an exemption whose
   record later gains payload fields beyond typed columns fails the record-struct
   census (existing derived census, retained).
4. Replay/golden safety -> `cargo test --workspace` green with the batched repricing
   ledger (if any arm gates) recorded for the acceptance artifact (spec §7.3).

## What to Change

### 1. Derive the apply-arm census

Replace the literal four-name list in
`materialized_agent_apply_arms_require_payload_schema_version` with a structural
scan: extract every `state.<map>.insert(`/`entry(`/`get_mut` write into a map named
in `AGENT_STATE_CHECKSUM_COVERAGE` from `EVENTS_APPLY_RS`, and require the enclosing
arm to call a version-requiring helper (any of the three key variants) or appear in
`TYPED_COLUMN_CLOSURE_EXEMPTIONS` with rationale. Add the synthetic-regression case.

### 2. Disposition the four ungated families (implementer-recorded choice)

For each of `apply_need_delta` (`needs_by_actor`, `need_tick_charges`),
`apply_intention_started` / `apply_intention_transition` (`intentions`,
`active_intention_by_actor`), and `apply_routine_step_transition`
(`routine_executions`): either stamp-and-require `payload_schema_version`
(builder + arm + forged-version gate; reprices goldens once, batched with `-001`) or
register the typed-column-closure exemption with rationale. Record each disposition;
high-frequency payloads (`NeedDeltaApplied`) weigh toward exemption per spec §9 —
the choice is the implementer's, recorded per arm.

### 3. Conformance row

Add the derived apply-arm census row to
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, describing exactly
what the census derives and the exemption registry's role (no overstatement).

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify — gates on arms that choose gating; otherwise comments/exemption anchors only)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify — forged-version gates for newly gated arms)
- `crates/tracewake-core/src/scheduler.rs` and emitters under `crates/tracewake-core/src/actions/defs/` (modify — as surfaced, only for arms that choose gating: payload stamping at builders)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `crates/tracewake-content/src/fixtures/` golden expectations (modify — implementation-discovered set, only if any arm gates; batched with `-001`)

## Out of Scope

- Mutation perimeter and baseline governance (`-003`).
- The checksum composition itself (`AGENT_STATE_CHECKSUM_COVERAGE` membership is
  consumed, not changed).
- Envelope-level field coverage (recorded forward note in spec §3; not load-bearing
  at this baseline).
- Risk-register entries (landed with `-001`).

## Acceptance Criteria

### Tests That Must Pass

1. Derived census green over the current 11 map-writing families, with the
   per-family disposition visible (gate or registry entry) — the enumerated members:
   `need_threshold_crossings`, `ordinary_life_episodes`,
   `candidate_goal_evaluations`, `continue_routine_arbitrations`,
   `decision_traces`, `stuck_diagnostics`, `needs_by_actor`, `need_tick_charges`,
   `intentions`, `active_intention_by_actor`, `routine_executions`.
2. Synthetic-regression case: an arm writing a covered map with neither gate nor
   exemption fails the guard.
3. Forged-version negative gate passes for every arm that chose gating (live +
   replay rejection).
4. Exemption-registry honesty test green; every entry carries a rationale.
5. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. No apply arm can write checksum-covered `AgentState` without a version gate or a
   recorded exemption — the census derives membership from source; hand-lists are
   gone from this surface (INV-020).
2. Replay rejects unsupported payload versions loudly on every gated arm; no silent
   repair (INV-020, INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — derived apply-arm
   census + synthetic-regression case + exemption-registry honesty test.
2. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — forged-version gates
   for newly gated arms.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-core --test event_schema_replay_gates`
3. `cargo test --workspace` (full pipeline; repricing ledger explained if any arm gated)

## Outcome

Completed: 2026-06-11

What changed:

- Replaced the literal apply-arm payload-version list with a structural census over
  `events/apply.rs` writes into maps named by `AGENT_STATE_CHECKSUM_COVERAGE`.
- Added `TYPED_COLUMN_CLOSURE_EXEMPTIONS` with arm, map, typed-column, and rationale
  entries for checksum-covered writes that intentionally remain typed-column-only.
- Added a synthetic regression source proving a covered-map write with neither a
  supported schema-version gate nor an exemption fails the guard.
- Added an exemption-registry honesty test so exemptions must stay live,
  rationale-bearing, and attached to actual covered-map write sites.
- Added the derived apply-arm census row to the architecture conformance index.

Per-arm dispositions:

- `need_threshold_crossings` — gated by `payload_schema_version`.
- `ordinary_life_episodes` — gated by `payload_schema_version`.
- `candidate_goal_evaluations` — gated by `payload_schema_version`.
- `continue_routine_arbitrations` — gated by `payload_schema_version`.
- `decision_traces` — gated by `trace_schema_version`.
- `stuck_diagnostics` — gated by `diagnostic_schema_version`.
- `needs_by_actor` — typed-column-closure exemption for `apply_need_delta`.
- `need_tick_charges` — typed-column-closure exemption for
  `assert_single_tick_delta_charge`.
- `intentions` — typed-column-closure exemptions for
  `apply_intention_started` and `apply_intention_transition`.
- `active_intention_by_actor` — typed-column-closure exemption for
  `apply_intention_started`.
- `routine_executions` — typed-column-closure exemption for
  `apply_routine_step_transition`.

Deviations from original plan:

- The ungated need, intention, active-intention, and routine-execution families were
  recorded as typed-column-closure exemptions instead of being payload-stamped. That
  means no event bytes changed, no golden/checksum repricing was needed, and no new
  forged-version negative gates were added beyond the existing event-schema replay
  gates for versioned arms.
- `events/apply.rs`, event builders, scheduler/actions emitters, and fixture
  goldens did not need code changes because the selected disposition was exemption
  rather than new payload stamping.

Verification results:

- `cargo fmt --all --check` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_apply_arms_require_payload_schema_version` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards typed_column_closure_exemptions_are_rationale_bearing_and_live` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards` — passed.
- `cargo test -p tracewake-core --test event_schema_replay_gates` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
