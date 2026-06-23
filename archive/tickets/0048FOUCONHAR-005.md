# 0048FOUCONHAR-005: Wire interval summary and salient stop through the step (flip; delete raw-log path)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — wires `advance_until` and the interval summary to the ticket-003 typed holder-known step delta and the ticket-002 sealed projection; deletes the TUI raw-log interval-source builder, the salient-observation global-log scan, and the legacy `String`-based `ActorKnownInterval*` types. No new event kinds, content, or fixtures.
**Deps**: 002, 003, 004

## Problem

Spec 0048 §4.4: the actor-known salient-observation stop branch is not production-reachable. At `cb3102e`, `advance_until` (`crates/tracewake-core/src/scheduler.rs:416`) calls `step_appended_actor_known_salient_observation` (`scheduler.rs:515`), which scans the appended event ids for an `ObservationRecorded` matching the possessed actor — but the world step appends only temporal/duration/accounting events and runs no perception, so the branch never fires; and `TuiApp::advance_until` (`crates/tracewake-tui/src/app.rs:342`) builds the interval summary (`app.rs:359`) *before* calling `record_current_place_perception_and_project` (`app.rs:366`), so perception is too late to stop the loop or appear in the summary. §4.5: the interval summary is built by the TUI raw-log scan `actor_known_interval_sources` (`app.rs:541`) mapping raw event kinds to prose via `actor_known_interval_summary_for_event` (`app.rs:564`) — the raw-global-diff/redaction architecture architecture `03` forbids.

This ticket performs the read-side flip (§8 closure steps 4–5): consume the ticket-003 typed holder-known step delta for both the salient stop and the interval summary, delete the raw-log builder and the global-log salient scan, and delete the legacy `String`-based `ActorKnownInterval*` types that ticket 002 superseded. Because ticket 002 built the sealed path additively, this is the atomic swap onto it — old path deleted, not wrapped.

## Assumption Reassessment (2026-06-23)

1. `step_appended_actor_known_salient_observation` (`crates/tracewake-core/src/scheduler.rs:515`) matches `EventKind::ObservationRecorded` for the possessed actor — a global-log scan, not typed step evidence. `TuiApp::advance_until` builds the summary at `crates/tracewake-tui/src/app.rs:359` then projects perception at `app.rs:366` (too late). `actor_known_interval_sources` (`app.rs:541`) + `actor_known_interval_summary_for_event` (`app.rs:564`) scan `EventLog` and map raw kinds to prose. The legacy `ActorKnownIntervalSource`/`build_actor_known_interval_summary` (`crates/tracewake-core/src/projections.rs:711,717`) and `ActorKnownIntervalNotice`/`ActorKnownIntervalSummary` (`crates/tracewake-core/src/view_models.rs:46-58`) are the String-based types ticket 002 superseded with sealed typed equivalents.
2. Spec 0048 §4.4/§4.5 and §8 (closure steps 4–5) own this; §4.4 names `scheduler.rs`, the perception/epistemic projection boundary, and `TuiApp::advance_until`; §4.5 names `projections.rs`/`view_models.rs`/`TuiApp::advance_until` and instructs deleting the TUI raw-log source builder rather than keeping it as a compatibility path. Depends on ticket 003 (the step result carries the typed delta), ticket 002 (the sealed projection + typed types), and ticket 004 (`advance_until` already loops the one-tick transaction).
3. Cross-artifact boundary under audit: the embodied stop/summary read path spanning the scheduler `advance_until` stop evaluation, the typed holder-known step delta (ticket 003), the sealed interval projection (ticket 002), and `TuiApp::advance_until`. The stop decision and summary must consume the typed delta; neither may scan the global log or infer actor knowledge from `actor_id` alone.
4. Constitutional invariants motivating this ticket: `INV-067` (embodied mode shows actor-known reality), `INV-099` (hidden truth may not select embodied output), `INV-101` (sealed actor-known context), `INV-102` (provenance), `INV-112` (temporal facts follow the same rule). The salient policy must be deterministic and actor-known; exact hidden events / raw due queues cannot stop embodied progression unless a modeled channel produced an allowed holder-known delta.
5. Enforcement surface (actor-knowledge firewall + deterministic-replay): replaces the global-log salient scan and raw-log summary with typed-delta consumption, closing the leakage path where a non-holder-known `ObservationRecorded` (or any raw event with a matching `actor_id`) could stop the loop or appear in the summary. Resumption perception becomes part of the interval projection *before* the final summary is returned. Confirm the stop decision replays from the typed step evidence (deterministic) and that no hidden-world event reaches the embodied output.
6. Removal / schema-swap blast radius (breaking-internal): deletes the TUI raw-log builder `actor_known_interval_sources` + `actor_known_interval_summary_for_event` (`app.rs:541,564`), the global-log scan `step_appended_actor_known_salient_observation` (`scheduler.rs:515`), `build_actor_known_interval_summary` + `ActorKnownIntervalSource` (`projections.rs:711,717`), and the String-based `ActorKnownIntervalNotice`/`ActorKnownIntervalSummary` (`view_models.rs:46-58`). The view-model type swap from the String-based summary to ticket-002's typed summary is **breaking-internal**: the sole consumer is `TuiApp` (rendering to strings now happens only at the TUI boundary, against the closed notice/stop values), updated in this diff. Grep blast radius — `.claude/skills/` none, `docs/`/`specs/` only spec-0048 prose, code tree the sites above + their inline tests.

## Architecture Check

1. Consuming the typed holder-known step delta makes the salient stop a property of modeled perception rather than a global-log coincidence: the loop stops only when a modeled channel produced an allowed holder-known observation, and the summary cites that source's provenance. Moving resumption perception into the interval projection before the summary fixes the ordering defect (summary-before-perception) at its root. Deleting the raw-log builder and the String types removes the forgeable path entirely instead of leaving it as a fallback.
2. No backwards-compatibility aliasing/shims: the raw-log builder, the global-log salient scan, and the String-based types are deleted, not wrapped; `TuiApp` renders the closed typed values directly.

### Implementer-recorded choices (spec §9, settled doctrine)

- **Typed salience policy (§9.3)**: which holder-known fact kinds stop acceleration vs. remain summary-only. Must be deterministic, source-bearing, and replayable. Record the chosen fact-kind set.
- **Rendered temporal labels (§9.5)**: whether exact interval ticks are themselves actor-known in each control context. Debug may show exact replay time; embodied rendering must follow holder-known temporal provenance. Record the chosen rendering rule.

## Verification Layers

1. `INV-067`/`INV-099` reachable + no-leak -> replay/golden-fixture check (positive/hidden pair): a modeled holder-known observation at tick N stops `advance_until` exactly at N with the typed actor-known stop reason and a source-citing summary; the same hidden event for another actor / without an acquisition channel leaves the possessed actor's stop tick and summary unchanged.
2. `INV-018`/`INV-112` replayable stop -> replay/golden-fixture check: rebuilding from the log reproduces both the temporal frontier and the stop evidence.
3. `INV-101`/`INV-102` typed-delta consumption -> codebase grep-proof: `step_appended_actor_known_salient_observation`, `actor_known_interval_sources`, and `build_actor_known_interval_summary` are gone; `advance_until` consumes the typed delta and does not scan the global log.
4. Single-layer note N/A — three distinct invariants map to three distinct proof surfaces above.

## What to Change

### 1. Consume the typed delta for the salient stop

In `crates/tracewake-core/src/scheduler.rs`, replace `step_appended_actor_known_salient_observation` with consumption of the ticket-003 typed holder-known step delta: `advance_until` stops on an allowed holder-known salience decision carried by the step result, per the recorded salience policy. Delete the global-log scan helper.

### 2. Build the summary from the sealed projection; fix ordering

In `crates/tracewake-tui/src/app.rs`, wire `advance_until` to build the interval summary from ticket-002's sealed projection over the typed delta, with resumption perception projected *before* the summary is returned. Delete `actor_known_interval_sources` and `actor_known_interval_summary_for_event`. Render ticket-002's closed notice/stop values to strings at the TUI boundary.

### 3. Delete the legacy String-based types

Delete `ActorKnownIntervalSource` + `build_actor_known_interval_summary` (`crates/tracewake-core/src/projections.rs`) and the String-based `ActorKnownIntervalNotice`/`ActorKnownIntervalSummary` (`crates/tracewake-core/src/view_models.rs`), updating the sole consumer (`TuiApp`) to the typed equivalents.

### 4. Positive/hidden/replay fixture pair

Add `crates/tracewake-core/tests/salient_stop_actor_known.rs` with the positive world, the hidden-world variant, and the replay reproduction (Verification Layers 1–2).

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — typed-delta salient stop; delete the global-log scan; shared with tickets 003/004)
- `crates/tracewake-tui/src/app.rs` (modify — wire summary to the sealed projection; fix perception ordering; delete the raw-log builder; shared with ticket 004)
- `crates/tracewake-core/src/projections.rs` (modify — delete legacy `ActorKnownIntervalSource` + `build_actor_known_interval_summary`)
- `crates/tracewake-core/src/view_models.rs` (modify — delete the String-based `ActorKnownInterval*` view types)
- `crates/tracewake-core/tests/salient_stop_actor_known.rs` (new — positive/hidden/replay fixture pair)

## Out of Scope

- Building the sealed projection / typed types — ticket 002 (this ticket consumes and then deletes the superseded path).
- Producing the typed holder-known delta inside the step — ticket 003.
- Privatizing the frontier / the one-tick transaction caller flip — ticket 004.
- Parity-runner measured outputs — ticket 007 (this ticket adds the core salient/interval fixtures, not the TUI parity scenarios).

## Acceptance Criteria

### Tests That Must Pass

1. Positive world: a modeled holder-known observation at tick N stops `advance_until` exactly at N, returns the typed actor-known stop reason, and the summary cites the source.
2. Hidden-world variant: the same hidden event for another actor / without an acquisition channel leaves the possessed actor's stop tick and summary unchanged; replay reproduces both the frontier and the stop evidence.
3. `grep -rn "step_appended_actor_known_salient_observation\|actor_known_interval_sources\|build_actor_known_interval_summary" crates/` returns nothing; `cargo test -p tracewake-core && cargo test -p tracewake-tui` pass.

### Invariants

1. The salient stop and the interval summary consume only the typed holder-known delta — no global-log scan, no `actor_id`-only inference (`INV-067`/`INV-101`).
2. Resumption perception is projected before the summary is returned; the stop decision replays from the typed step evidence (`INV-102`/`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/salient_stop_actor_known.rs` (new) — positive/hidden/replay fixture pair.

### Commands

1. `cargo test -p tracewake-core --test salient_stop_actor_known`
2. `cargo test -p tracewake-core && cargo test -p tracewake-tui` (the read-side flip compiles and passes across both crates).

## Outcome

Completed: 2026-06-23

Replaced the raw-log salient-observation scan with typed interval-delta evidence carried by `WorldAdvanceResult`. `advance_until` now requests possessed-actor interval evidence from the staged one-tick transaction and stops only on the recorded typed salience policy: `Observation`, `Record`, and `Belief` notices are salient; routine direct `Perception` notices remain summary-only. Projection-derived embodied contexts now carry source-event provenance for included actor-known facts so interval deltas are source-citing and replayable.

Rewired `TuiApp::advance_until` to snapshot the sealed before context, run the scheduler, project resumption perception, build the sealed after context, and store a `TypedActorKnownIntervalSummary` from `EpistemicProjection::actor_known_interval_delta`. The TUI renderer converts only closed typed stop/notice values to strings at the rendering boundary. Exact interval ticks remain rendered in embodied interval summaries as controller/replay interval labels; debug continues to expose exact replay time.

Deleted `step_appended_actor_known_salient_observation`, `actor_known_interval_sources`, `actor_known_interval_summary_for_event`, `ActorKnownIntervalSource`, `build_actor_known_interval_summary`, and the legacy string-based `ActorKnownIntervalNotice` / `ActorKnownIntervalSummary` view types. The new `crates/tracewake-core/tests/salient_stop_actor_known.rs` fixture proves a modeled visible-actor observation stops acceleration with typed source evidence, an other-actor hidden observation is not included in the possessed actor delta, and replay rebuild recomputes the stop evidence.

Verification run:

1. `grep -rn "step_appended_actor_known_salient_observation\|actor_known_interval_sources\|build_actor_known_interval_summary" crates/` — printed nothing.
2. `cargo test -p tracewake-core --test salient_stop_actor_known` — passed.
3. `cargo test -p tracewake-core` — passed.
4. `cargo test -p tracewake-tui` — passed.
5. `cargo fmt --all --check` — passed.
6. `cargo clippy -p tracewake-core --all-targets -- -D warnings` — passed.
7. `cargo clippy -p tracewake-tui --all-targets -- -D warnings` — passed.
8. `git diff --check` — passed.
