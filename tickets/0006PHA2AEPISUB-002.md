# 0006PHA2AEPISUB-002: Seal EpistemicProjection storage and route all-holder access through a core debug builder

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` epistemic projection storage sealed (private maps, `pub(crate)` inserts); new core capability-mediated debug-view builder; `tracewake-tui` debug epistemics migrated off raw projection maps.
**Deps**: 0006PHA2AEPISUB-001

## Problem

`EpistemicProjection` exposes all of its raw all-holder storage as `pub`: `observations_by_id`, `observations_by_actor`, `beliefs_by_id`, `beliefs_by_holder`, `contradictions_by_id`, `contradictions_by_holder`, `notebook_entries_by_actor`, plus version/range/manifest metadata (`crates/tracewake-core/src/epistemics/projection.rs:27-40`), and its `insert_observation`/`insert_belief`/`insert_contradiction`/`insert_notebook_entry` mutators are `pub` (`:85-121`). Although context-filtered read APIs exist (`:124-180`), any caller can bypass actor-known filtering by reading the raw maps, and any caller can mutate projection state outside the event-apply path. The TUI debug path does exactly this across the crate boundary, hand-building debug views from raw maps (`crates/tracewake-tui/src/app.rs:371-443`), and a core production cognition site reads raw maps too (`crates/tracewake-core/src/actions/pipeline.rs:1144-1148`). This is the EPI-HARD-002 contamination path: all-holder epistemic state is reachable without a non-diegetic debug capability, defeating the no-telepathy firewall.

## Assumption Reassessment (2026-06-09)

1. Raw maps and inserts are `pub`: confirmed `crates/tracewake-core/src/epistemics/projection.rs:27-40` (storage) and `:85-121` (insert helpers); filtered read APIs `observations_for_context`/`beliefs_for_context`/`contradictions_for_context`/`notebook_entries_for_context` exist at `:124-180`. The legitimate event-apply mutation path is `crates/tracewake-core/src/events/apply.rs:204-214` (same crate — stays valid once inserts are `pub(crate)`).
2. Spec authority: `specs/0006_…SPEC.md` §6 EPI-HARD-002 (and §10 item 4, TUI migration, merged here); architecture requires embodied views to filter by holder-known context and debug views to remain structurally separate (`docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md:82-85`); execution requires debug panels to be non-diegetic and unable to feed gameplay (`docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md:84-101`).
3. Shared boundary under audit: the `EpistemicProjection` public API across the `tracewake-core` → `tracewake-tui` crate boundary. The core debug-view types already exist (`crates/tracewake-core/src/view_models.rs:451-482` `DebugEpistemicsView`/`DebugBeliefsView`/`DebugObservationsView`, constructors at `:565,594,612` mint a `DebugCapability`), but the all-holder collection logic currently lives in TUI `app.rs` reading raw maps — the missing piece is a core-owned builder that takes `&EpistemicProjection` + a `DebugCapability` and returns the populated views.
4. Constitutional invariants motivating this ticket: `INV-002`/`INV-024` (no telepathy; cognition only from filtered modeled knowledge), `INV-009`/`INV-012` (state changes only through events — the public inserts are an alternate mutation path), `INV-067` (embodied views actor-known), `INV-068`/`INV-107` (debug non-diegetic and quarantined). `INV-018` (deterministic replay) bounds the change: ordering must stay `BTreeMap`/`BTreeSet`.
5. Enforcement surface: actor-knowledge filtering (the `_for_context` read model becomes the *only* public read path) and the event-apply mutation discipline (inserts become `pub(crate)`). The change strengthens leakage prevention and removes a non-event mutation path; it must not weaken deterministic replay — internal storage stays `BTreeMap`/`BTreeSet` and the checksum (`compute_checksum`, `:183-271`) is unchanged.
6. Visibility/shape change: the projection's field *set* is unchanged; storage fields move `pub` → private, inserts move `pub` → `pub(crate)`. Consumers split into (a) same-crate production: `actions/pipeline.rs:1144-1148` must move to `beliefs_for_context` (or a `pub(crate)` accessor); (b) cross-crate production: TUI `app.rs:371-443` + `app.rs:113` (`insert_belief` during setup) must move to the new core builder / event-apply; (c) tests reading raw maps: `events/apply.rs:1652-1673`, `replay/rebuild.rs:458-483`, `projections.rs:526,1491-1501`, `agent/actor_known.rs:412`, core `golden_scenarios.rs:468-632`, content `golden_fixtures_run.rs:1349-1358`. All change in this diff (cross-crate compile-atomic).
7. Removal blast radius of the old public surface (grep `crates`, excluding `projection.rs`): enumerated in item 6. Note that `view.beliefs_by_holder` reads in `view_models.rs:910` and `debug_panels.rs:131,144` are `DebugEpistemicsView` fields (the debug view's own public shape), NOT raw projection maps — they are correct and out of scope.

## Architecture Check

1. A private raw store with a single public context-filtered read model, plus one core-owned capability-mediated debug builder, makes "read all-holder state" express a `DebugCapability` at the type level rather than relying on every caller to choose the filtered API. The TUI stops being able to reach raw truth at all; it asks core for a non-diegetic view. This is cleaner than leaving the maps public and auditing call sites, because it removes the leak path structurally.
2. This is an atomic-cutover ticket (spec anti-contamination thesis): privatizing the maps must land together with every consumer migration, or the workspace will not compile and a partial change would leave the raw-read bypass live. No wrapper/fallback that keeps raw access reachable is introduced; no backwards-compat alias for the raw maps is kept.

## Verification Layers

1. `INV-024`/`INV-067` (no all-holder read without capability) -> codebase grep-proof: no `pub` storage field on `EpistemicProjection`; TUI `app.rs` no longer names `beliefs_by_id`/`observations_by_id`/`beliefs_by_holder`/`contradictions_by_id` (proven by the source guard in 0006PHA2AEPISUB-005) + core unit test that two actors' beliefs are separable by context while a debug view lists both only via the capability path.
2. `INV-009`/`INV-012` (mutation only via events) -> codebase grep-proof: `insert_*` are `pub(crate)`; the only callers are `events/apply.rs` and test setup.
3. `INV-018` (deterministic replay) -> replay/golden-fixture check: `cargo test -p tracewake-core --test event_schema_replay_gates` + projection checksum tests green; storage remains `BTreeMap`/`BTreeSet`.
4. `INV-068`/`INV-107` (debug quarantined) -> manual review + TUI seam test: `debug epistemics` still renders all-holder data through the core builder; embodied `notebook` cannot import debug rows.

## What to Change

### 1. Privatize projection storage; narrow inserts

In `crates/tracewake-core/src/epistemics/projection.rs`, make the seven storage maps and the version/range/manifest metadata private; make `insert_observation`/`insert_belief`/`insert_contradiction`/`insert_notebook_entry` `pub(crate)`. Keep the `_for_context` filtered APIs public.

### 2. Add a core capability-mediated all-holder debug builder

Add a core function (on `EpistemicProjection` or in `view_models.rs`) that takes `&EpistemicProjection` + `&DebugCapability` and returns the populated `DebugEpistemicsView` / `DebugBeliefsView` / `DebugObservationsView` (moving the collection logic currently in TUI `app.rs:371-443` into core). Preserve deterministic ordering with explicit stable sorting in the returned vectors.

### 3. Migrate consumers

- `actions/pipeline.rs:1144-1148`: replace raw `beliefs_by_holder`/`beliefs_by_id` reads with `beliefs_for_context` (or a `pub(crate)` accessor if a context is not available at that site — confirm during implementation).
- TUI `app.rs`: call the new core debug builder instead of reading raw maps; route setup-time `insert_belief` (`:113`) through the event-apply/seed path rather than direct insert.
- Tests reading raw maps (`events/apply.rs`, `replay/rebuild.rs`, `projections.rs`, `agent/actor_known.rs`, core `golden_scenarios.rs`, content `golden_fixtures_run.rs`): use the new accessors / context-filtered reads or `pub(crate)` test helpers.

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify — core debug-view builder)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify — insert call sites / test reads)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify — test reads)
- `crates/tracewake-core/src/projections.rs` (modify — test reads/inserts)
- `crates/tracewake-core/src/agent/actor_known.rs` (modify — test read)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify — test reads/inserts)
- `crates/tracewake-tui/src/app.rs` (modify — migrate debug views + seed insert; **shared file, also touched by 0006PHA2AEPISUB-003**)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — test reads/insert)

## Out of Scope

- `KnowledgeContext` sealing (0006PHA2AEPISUB-001, dependency).
- Epistemic record field sealing (0006PHA2AEPISUB-003).
- Compile-fail negative fixtures (0006PHA2AEPISUB-004) and source-guard/clippy extension (0006PHA2AEPISUB-005).
- Any change to projection checksum / canonical serialization semantics.

## Acceptance Criteria

### Tests That Must Pass

1. A core unit test constructs two actors' beliefs and proves an embodied context returns only the bound actor's beliefs, while the core debug builder lists both holders only when given a `DebugCapability`.
2. A TUI seam test proves `debug epistemics` still renders non-diegetic all-holder data, and embodied `notebook` output contains no other-holder rows.
3. `cargo build --workspace --all-targets --locked`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. No `pub` storage field remains on `EpistemicProjection`; the only public read path is the context-filtered API + the capability-gated debug builder; `insert_*` are `pub(crate)`.
2. Internal storage remains `BTreeMap`/`BTreeSet`; projection checksum and deterministic replay are byte-identical to pre-change.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/projection.rs` (in-module tests) — two-actor context-separation vs debug-builder all-holder test.
2. `crates/tracewake-tui/tests/tui_seam_conformance.rs` — debug-vs-embodied separation through the new core builder.
3. Existing core/content tests adjusted to use accessors instead of raw maps.

### Commands

1. `cargo test -p tracewake-core --lib epistemics::projection`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test -p tracewake-core --test event_schema_replay_gates` — narrow proof that sealing did not perturb deterministic replay/checksum.
