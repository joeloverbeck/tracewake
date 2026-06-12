# 0023PHA3AEMBLOC-004: Embodied-locality epistemic migration — observation events and projection-derived view

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — perception emitters (`agent/perception.rs`), epistemic projection (`epistemics/projection.rs`), embodied view builder (`projections.rs`, `location.rs`, `view_models.rs`), controller/TUI consumers, golden fixtures/logs repricing
**Deps**: `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

The embodied view model's physical locality surface is generated from raw
`PhysicalState`, not the holder-known context (spec 0023 `ORD-HARD-121`, high — this
pass's product-behavior foundation violation). `build_embodied_view_model`
(`crates/tracewake-core/src/projections.rs`) sets `let state = source.state;` and
calls `visible_locality(actor, &state.actors, &state.doors, &state.containers,
&state.items)` (`src/location.rs`); the results feed
`EmbodiedViewModel.{visible_items, visible_containers, carried_items, local_actors,
visible_doors}` and the open/take/inspect semantic actions over true IDs;
`visible_exit_blocker_summary` likewise reads `state`. This is the verbatim failure
shape of the constitution's Enforcement reading ("embodied views generated from truth
rather than actor-known context") and of `docs/1-architecture/10`'s generation rule.
The Phase 3A surfaces (food/sleep/workplace) in the *same function* correctly consume
only `source.actor_known_*`; the legacy Phase 1 locality surface beside them was
never re-pointed. No hidden truth leaks today (concealment filtering holds), but the
surface fails open for every future concealment, identity-uncertainty, or staleness
mechanic.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: the `let state = source.state;` read and `visible_locality`
   call in `build_embodied_view_model`; `visible_locality` iterating authoritative
   `PhysicalState` maps (`src/location.rs`); the lawful pattern one module over —
   `current_place_perception_events` / `record_current_place_perception`
   (`src/agent/perception.rs`, `src/agent/mod.rs`) already mint observation events
   from truth for food/sleep/exits with confidence/source. Consumer blast radius
   (grep): `EmbodiedProjectionSource` is consumed by `projections.rs`,
   `controller.rs`, `crates/tracewake-tui/src/app.rs`, and
   `tests/hidden_truth_gates.rs`; `visible_locality` is additionally consumed by
   `src/actions/defs/inspect.rs` — a *validation-side* consumer that lawfully reads
   truth (INV-099: truth may validate) and is explicitly NOT migrated.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-121`
   (operator-verified evidence; required correction), §8's staging note (emitters +
   projection land before truth-access removal — this ticket is the first stage;
   -005 is the second), and §9's volume note (emit on place entry and on change, not
   per tick, mirroring the existing perception cadence).
3. Shared contract under audit: the embodied view-model generation rule
   (`docs/1-architecture/10` — "generated from a holder-known context for the bound
   actor plus permitted projections") across `EmbodiedProjectionSource` →
   `build_embodied_view_model` → `EmbodiedViewModel` → TUI render; consumed by
   `app.rs`/`controller.rs` and every TUI test that renders locality.
4. Constitutional motivation restated: INV-067 (embodied mode shows actor-known
   reality), INV-024 (no telepathy — information reaches actors only through modeled
   channels), INV-009/012 (meaningful changes — here observations — are eventful),
   INV-008 (UI may not reveal hidden truth in embodied mode), and the Enforcement
   reading's explicit bullet; INV-029 forward-compatibility (the observation carries
   an observed-actor identity slot, resolved to the true id in Phase 3A).
5. This ticket touches actor-knowledge filtering and deterministic-replay surfaces:
   new observation events enter the log (replay-visible, schema-versioned per
   INV-020) and the embodied view becomes projection-derived. Enforcement surfaces:
   the epistemic projection's classification path
   (`classified_actor_known_records_for_context` / `current_place_knowledge_context`)
   and the golden-fixture replay checks. The change strengthens leakage prevention
   (the view can no longer read truth) and preserves determinism (events are emitted
   by the deterministic perception pass; goldens re-derived once, honestly, in this
   ticket — the `ORD-HARD-076`/`107` repricing precedent).
6. Schema extension: new observation-event payload families for
   items/containers/doors/co-located actors (extending the existing perception
   observation kinds in the event vocabulary) and new actor-known record kinds in the
   epistemic projection's policy table. Consumers: `events/apply.rs` arms (totality
   guards force registration), the projection policy table (the 0022 table-driven
   locks force declared policy per kind — and -007 then locks behavior per row),
   `view_models.rs` (fields keep their existing shape — population source changes,
   not the view-model schema). The extension is additive (new kinds/records;
   existing payloads unchanged); goldens reprice because new events appear in
   canonical logs, recorded as the honest re-derivation this ticket batches once.
7. Concealment semantics relocate, not weaken: `is_open` /
   `contents_visible_when_closed` decide what the observation event *records* at
   perception time, replacing post-hoc view filtering — the typed-visibility
   discipline `ORD-HARD-078` established for exits applies to the new emitters.
8. Adjacent contradiction classification: `visible_exit_blocker_summary`'s truth read
   is a required consequence of this ticket (it must derive from actor-known
   why-not inputs per INV-070); `inspect.rs`'s `visible_locality` use is lawful
   validation, explicitly retained (not a bug, not cleanup).
9. Change rationale (no silent retcon): the locality surface moves onto the epistemic
   channel because the constitution's Enforcement reading names its current shape as
   a failure; mandated by `ORD-HARD-121`'s required correction. No doctrine
   amendment: the migration uses existing event channels and the existing generation
   rule — doctrine is being *complied with*, not changed.

## Architecture Check

1. Extending the existing perception emitters and projecting through the existing
   `KnowledgeContext` path is cleaner than inventing a parallel locality projection:
   the sibling Phase 3A surfaces already prove the pattern (events with
   confidence/source → classified actor-known records → view facts), the policy
   table and its locks extend naturally to the new record kinds, and replay/debug get
   locality provenance for free. The rejected alternative — keeping the truth read
   but filtering harder — cannot satisfy the Enforcement reading and leaves the
   fail-open posture.
2. No backwards-compatibility aliasing/shims: the truth-derived population path is
   replaced in this ticket's scope (the `state` field itself is removed by -005,
   completing the cutover); no dual-source fallback is kept. Carried items remain
   body-state by design (the actor knows what it carries) — recorded as a deliberate
   non-migration, not a shim.

## Verification Layers

1. INV-067/024 (actor-known embodied view) -> replay/golden-fixture check: on the
   canonical fixtures, the locality fields rendered embodied equal the
   projection-derived set; the runner and TUI flows stay green post-migration.
2. INV-009/020 (eventful + versioned) -> codebase test-proof: new observation kinds
   carry schema versions and apply arms (the existing world/agent totality guards
   fail any unregistered kind).
3. INV-070 (why-not in actor-known terms) -> codebase test-proof:
   `visible_exit_blocker_summary` derives from actor-known inputs post-migration.
4. Determinism (INV-018) -> replay/golden-fixture check: goldens re-derived once;
   `no_human_day_real_run_replays_metrics_and_trace_projection` and the divergence
   oracles green on the repriced logs.

## What to Change

### 1. Locality observation emitters

Extend `current_place_perception_events` (or sibling emitters beside it in
`src/agent/perception.rs`) to record observation events for items, containers, doors,
and co-located actors at the perceived place, on place entry and on change (the
existing perception cadence/dedup discipline). Concealment decided at recording time
(`is_open` / `contents_visible_when_closed`); co-located-actor observations carry an
observed-actor identity slot (resolved to true id in Phase 3A — the INV-029 seam).

### 2. Projection and policy-table registration

Project the new observations into actor-known records via the existing
classification path; declare each new record kind's policy
(classification/embodied_scope/accessibility_scope) in the
`epistemics/projection.rs` table.

### 3. Projection-derived view derivation

Derive `visible_items`/`visible_containers`/`visible_doors`/co-located actors and
their open/take/inspect semantic actions from the projection in
`build_embodied_view_model`; derive `visible_exit_blocker_summary` from actor-known
inputs. `carried_items` remains body-state. `visible_locality` remains for
validation-side consumers (`inspect.rs`); the embodied builder stops calling it.

### 4. Golden/fixture repricing

Re-derive golden logs/fixtures the new observation events reprice — once, honestly,
in this ticket (`crates/tracewake-content` fixtures and any pinned canonical logs, as
surfaced during implementation; parent crates verified to exist).

## Files to Touch

- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/location.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/events/envelope.rs` (modify)
- `crates/tracewake-core/src/controller.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — repriced expectations, as surfaced)
- golden fixture/log artifacts under `crates/tracewake-content/` (modify — implementation-discovered set; parent verified)

## Out of Scope

- Removing the `state` field from `EmbodiedProjectionSource` and the INV-093
  negatives/staleness positive (ticket -005 — the staging note's second stage).
- Identity-uncertainty mechanics (INV-029 implementation — Phase 3B+; only the seam
  lands here).
- `inspect.rs`'s validation-side `visible_locality` use (lawful; retained).
- Debug overlay wiring (-006).

## Acceptance Criteria

### Tests That Must Pass

1. Embodied locality fields on the canonical fixtures are populated solely from
   projection-derived records (assert provenance: each rendered locality entry maps
   to an actor-known record with real `source_event_id`s).
2. Each of the four families (items, containers, doors, co-located actors) has an
   emit-on-entry and emit-on-change observation proof, with concealment recorded at
   observation time (closed-container contents absent from the event unless
   `contents_visible_when_closed`).
3. World/agent apply-arm totality guards green with the new kinds registered;
   repriced goldens replay deterministically
   (`cargo test -p tracewake-content --test golden_fixtures_run`,
   `cargo test -p tracewake-core`).
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. The embodied locality surface contains no entry lacking actor-known provenance —
   every rendered item/container/door/actor traces to observation events.
2. Concealment semantics are decided at the truth→event boundary, never by post-hoc
   view filtering of raw truth.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/perception.rs` (inline tests) — per-family
   emit-on-entry/emit-on-change + concealment-at-recording proofs.
2. `crates/tracewake-core/src/projections.rs` / `epistemics/projection.rs` tests —
   projection-derived locality with provenance assertions; policy rows for new kinds.
3. `crates/tracewake-content/tests/golden_fixtures_run.rs` — repriced canonical-day
   expectations (runner-only ancestry assertions preserved).

### Commands

1. `cargo test -p tracewake-core && cargo test -p tracewake-content --test golden_fixtures_run`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
