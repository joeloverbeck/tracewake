# 0019PHA3AGENREA-001: Payload-version closure and prose-free agent checksums

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`events/apply`, `checksum`, `scheduler` threshold emitter, `state` as surfaced); derived censuses in `anti_regression_guards.rs`; one batched golden-checksum repricing in `tracewake-content`
**Deps**: `specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md` (ORD-HARD-047, ORD-HARD-050)

## Problem

Two evidence-layer gaps that both reprice agent checksums, batched so the golden churn
happens once (spec §8). First (`ORD-HARD-047`, INV-020): the `apply.rs` arm for
`EventKind::NeedThresholdCrossed` inserts `NeedThresholdCrossedRecord` into
`state.need_threshold_crossings` with bare `required(&payload, …)` and no
`require_payload_version` call — unlike its episode/candidate/continue siblings — so a
future payload-shape change cannot be loud-rejected; and the censuses
(`materialized_agent_apply_arms_require_payload_schema_version`,
`materialized_agent_payload_records_keep_payload_fields`) are hand-enumerated lists that
silently skip the threshold arm and any future materialized kind. Second
(`ORD-HARD-050`, INV-105 in inverse): the canonical checksum lines for
`ordinary_life_episode`, `candidate_goal_evaluation`, and
`continue_routine_arbitration` (`checksum.rs`) each end in `…|payload={}|summary={}`,
where `summary` is the cloned free-form `effects_summary` display string — replay
identity is coupled to display prose, the inverse of "display strings must not be the
authoritative substrate."

## Assumption Reassessment (2026-06-11)

1. Verified against current code at `main` `5af8660`: the threshold apply arm
   (`apply.rs`, `EventKind::NeedThresholdCrossed`) has no `require_payload_version`
   call (the file's `payload_schema_version` calls sit only in the episode, candidate,
   and continue arms; `trace_schema_version`/`diagnostic_schema_version` cover the
   decision-trace family); `EventKind::FoodServiceUsed` is already version-gated inside
   the multi-kind episode arm, so the threshold arm is the only ungated materialized
   kind. `checksum.rs` carries exactly three `|summary={}` canonical lines;
   `apply.rs` clones `event.effects_summary` into `summary` at exactly three record
   build sites. The threshold event is emitted from `scheduler.rs` (the
   `EventKind::NeedThresholdCrossed` builder near the need-delta application path),
   with `actions/defs/wait.rs` also referencing the kind — confirm the full emitter set
   by grep at implementation.
2. Verified against `specs/0019_…_HARDENING_SPEC.md` ORD-HARD-047/050 (reassessed
   2026-06-11, all findings confirmed): threshold *values* are checksum-covered as
   typed columns, so tampering is already caught — the gap is the loud
   unsupported-version rejection (INV-020) and the censuses' false universality.
3. Cross-artifact boundary under audit: the materialized-agent payload contract spanning
   the apply arms (`events/apply.rs`), the record structs (`state.rs`), the canonical
   checksum lines (`checksum.rs`), and the censuses (`anti_regression_guards.rs`) —
   one version-gated, payload-evidenced, prose-free contract per materialized kind.
4. INV-020 restated: event kinds and payloads must be versioned enough that replay can
   reject unsupported history rather than silently inventing repairs. INV-105 restated:
   decision evidence must be typed or structurally inspectable; display strings may
   summarize but must not be the authoritative substrate — folding prose into checksum
   identity is the same sin in the opposite direction.
5. Deterministic-replay surface touched: the agent checksum composition changes twice in
   one diff (threshold version field added to payloads; `summary` component removed
   from three canonical lines), so agent checksums and golden expectations reprice once.
   Tamper detection is strengthened, not weakened — full `payload_fields` coverage is
   retained, the existing `episode_tamper_*` gates must stay red on payload rewrites,
   and a new forged-version gate adds loud rejection. No epistemic-leakage direction
   changes (no view-model or actor-knowledge surface is touched).
6. Schema extension named: the `NeedThresholdCrossed` event payload gains
   `payload_schema_version` (consumers: the apply arm, `replay/rebuild.rs` re-apply,
   and the reachability/threshold assertions in tests). The extension is deliberately
   breaking-for-stored-history — a forged or stale version must reject loudly; golden
   fixtures regenerate in this same ticket, and the per-actor checksum diff ledger is
   produced here for the 0019 acceptance artifact (ticket `-009`).
7. Removal blast radius (the `summary` checksum component): grep confirms the
   `|summary=` token appears in `checksum.rs` (three canonical lines) and in the
   guards' census strings referencing those line formats; any test asserting the full
   line shape joins this ticket's Files to Touch at implementation. The `summary`
   field stays on the records for display — only the checksum component is removed.

## Architecture Check

1. Deriving both censuses structurally (every apply arm that inserts into an
   `AgentState` projection map must call the version-requiring helper; every
   materialized record struct whose source events carry payload fields beyond its typed
   columns must store `payload_fields` or register a typed-column-closure exemption
   with rationale) turns "forgot the new kind" into a CI failure instead of a silent
   census skip — the same correction shape that fixed the witness table in 0018.
   Removing prose from checksum identity mirrors the existing
   `controller_and_debug_metadata_are_excluded` exclusion precedent rather than
   inventing a new mechanism.
2. No backwards-compatibility aliasing/shims: no optional version fallback, no legacy
   checksum mode — the old unversioned threshold payload and the prose-bearing checksum
   lines are removed, not wrapped.

## Verification Layers

1. INV-020 loud rejection -> negative replay gate in `event_schema_replay_gates.rs`:
   a forged `payload_schema_version` on `NeedThresholdCrossed` yields live
   `ApplyError::BadPayload` and `!matches_expected` under replay (the
   `forged_payload_schema_version_rejected_…` pattern).
2. Census derivation -> reworked guards in `anti_regression_guards.rs` parse
   `apply.rs` apply arms and `state.rs` record structs instead of hand-typed lists;
   a synthetic unregistered-kind probe fails the census.
3. INV-105 prose exclusion -> new checksum unit test: two states differing only in a
   record's `summary` hash identically (the metadata-exclusion pattern).
4. INV-018 deterministic replay -> `golden_fixtures_run.rs` green at the repriced
   checksums; `episode_tamper_output_tag_poisons_replay` and
   `episode_tamper_proration_poisons_replay` remain red on tampered payloads.

## What to Change

### 1. Version-gate the threshold payload

Stamp `payload_schema_version` in the `NeedThresholdCrossed` emitter(s)
(`scheduler.rs` threshold builder; `actions/defs/wait.rs` if it also emits — confirm by
grep) and add `require_payload_version(&payload, "payload_schema_version", "1")` to the
threshold apply arm, mirroring the episode siblings.

### 2. Derive the two materialized-kind censuses

Replace the hand-enumerated arm list in
`materialized_agent_apply_arms_require_payload_schema_version` with a structural
derivation (every arm containing an insert into an `AgentState` projection map must
contain the version-requiring call), and extend
`materialized_agent_payload_records_keep_payload_fields` to derive its struct list from
the `AgentState` map value types. For `NeedThresholdCrossedRecord`, either store
`payload_fields` like its siblings or register a typed-column-closure exemption with
rationale (implementer's recorded choice; the census must enforce whichever contract is
chosen).

### 3. Forged-version negative gate

`event_schema_replay_gates.rs`: forge `payload_schema_version="2"` on a
`NeedThresholdCrossed` event in a copied golden log; assert loud live rejection and
poisoned replay.

### 4. Prose-free canonical checksum lines

`checksum.rs`: drop the `|summary={}` component from the three canonical lines (records
keep `summary` for display); add the equal-hash exclusion test.

### 5. Single golden repricing

Regenerate golden checksum expectations once for both changes
(`golden_fixtures_run.rs` and fixture expectations as surfaced — the touched fixture
set is implementation-discovered under `crates/tracewake-content/src/fixtures/`);
record the per-actor diff ledger data for the acceptance artifact (spec §7 item 5,
recorded in ticket `-009`).

## Files to Touch

- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify — threshold emitter version stamp)
- `crates/tracewake-core/src/actions/defs/wait.rs` (modify — as surfaced, if it emits the kind)
- `crates/tracewake-core/src/checksum.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify — as surfaced by the payload_fields-vs-exemption choice for `NeedThresholdCrossedRecord`)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — repriced expectations)
- `crates/tracewake-content/src/fixtures/` (modify — implementation-discovered expectation set; parent directory verified)

## Out of Scope

- The embodied workplace freshness split (ticket `0019PHA3AGENREA-002`).
- Generative-tier reachability, tamper relation, and floors (tickets `-003`, `-004`).
- Mutation perimeter and CI gate semantics (ticket `-005`).
- The 0019 acceptance artifact itself (ticket `-009`) — this ticket only produces the
  per-actor checksum diff data.
- Envelope-level schema versioning (the envelope gate already exists and is out of the
  0019 spec's findings).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test event_schema_replay_gates` — forged threshold
   version rejected live and under replay.
2. `cargo test -p tracewake-core --test anti_regression_guards` — derived censuses
   green; synthetic unregistered-kind probe red.
3. `cargo test -p tracewake-content --test golden_fixtures_run` — repriced goldens
   green; both episode tamper gates still poison replay.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every apply arm that materializes an agent projection record requires
   `payload_schema_version`; the censuses derive membership structurally, not from
   hand-typed lists.
2. No display prose enters any canonical checksum line; two states differing only in
   record `summary` hash identically.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — forged
   `NeedThresholdCrossed` version negative gate.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — derived arm census +
   derived record-struct census.
3. `crates/tracewake-core/src/checksum.rs` (unit test) — summary-exclusion equal-hash
   test alongside `controller_and_debug_metadata_are_excluded`.
4. `crates/tracewake-content/tests/golden_fixtures_run.rs` — repriced expectations;
   existing tamper gates unchanged and green-red as designed.

### Commands

1. `cargo test -p tracewake-core --test event_schema_replay_gates`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo test --workspace`
