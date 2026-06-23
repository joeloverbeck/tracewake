# 0048FOUCONHAR-004: Singular temporal authority — privatize the frontier and flip the callers (atomic cutover)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — privatizes `DeterministicScheduler::current_tick` behind a read-only accessor and a typed restoration constructor; deletes `sync_scheduler_frontier_to_appended_events` and the legacy `advance_world_one_tick`; reroutes the human and no-human callers into the ticket-003 one-tick transaction; removes the TUI frontier assignments. No new event kinds, content, or fixtures.
**Deps**: 001, 003

## Problem

Spec 0048 §4.3: the temporal frontier has multiple writers and can advance without tick ancestry. At `cb3102e`, `DeterministicScheduler` exposes `pub current_tick` (`crates/tracewake-core/src/scheduler.rs:338`); writers beyond the intended authority are `TuiApp::submit_entry_with_world_advance` (`crates/tracewake-tui/src/app.rs:321`, `self.scheduler.current_tick = last_event.sim_tick`), `TuiApp::run_no_human_day` (assigns `report.final_tick`), and `sync_scheduler_frontier_to_appended_events` (`scheduler.rs:1731`, sets the frontier to the max `sim_tick` of arbitrary appended events). §4.2 adds that `submit_entry_with_world_advance` commits the possessed action through `run_pipeline` *before* calling the coordinator, so a later coordinator error leaves a partial commit — the transition is not atomic. This ticket is the atomic flip (§8 closure steps 1-runtime and 3): make the frontier private with a read-only accessor, route all runtime advancement through ticket 003's one-tick transaction, delete the bypasses, and add the failure-atomicity and runtime marker-chain witnesses.

This is a deliberately large single diff. Privatizing a public field is a local compile-atomicity unit (every in-workspace writer/reader must change together or the tree won't compile), and §4.3 is anti-incremental — leaving the sync helper or a TUI setter live behind a wrapper would re-introduce the alternate temporal authority the spec removes. Per spec 0048 §8 the new path (ticket 003) was built additively first; this ticket flips callers onto it and deletes the old path in one reviewable cutover.

## Assumption Reassessment (2026-06-23)

1. `pub current_tick: SimTick` at `crates/tracewake-core/src/scheduler.rs:338`. Read sites: ~14 in `crates/tracewake-tui/src/app.rs` (e.g. `expected_tick = self.scheduler.current_tick`) plus internal scheduler reads. Write sites to remove/reroute: `app.rs:321` (TUI assignment), `run_no_human_day` (`scheduler.rs:1179`, `report.final_tick`), `sync_scheduler_frontier_to_appended_events` (`scheduler.rs:1731`, called at `scheduler.rs:1332` in `run_no_human_day` and in tests). The no-human runner `advance_no_human` (`scheduler.rs:2704`) currently takes a scheduled-proposals `Vec` and runs proposals separately from the coordinator; it is rerouted to feed those proposals as inputs to the ticket-003 transaction. `submit_entry_with_world_advance` (`app.rs:264`) calls `run_pipeline` then `advance_world_one_tick`.
2. Spec 0048 §4.3/§4.2 and §8 (closure steps 1-runtime/3) own this; §4.3 names the homes `scheduler.rs`, `scheduler::no_human`, `crates/tracewake-tui/src/app.rs`, and the replay restoration boundary. Ticket 001 supplies the replay temporal projection the typed restoration constructor verifies against; ticket 003 supplies the one-tick transaction the callers reroute into. The spec's reassessment added (M1) that all `current_tick` read sites migrate to the accessor and only the canonical commit + the restoration constructor write it.
3. Cross-artifact boundary under audit: the temporal-authority surface spanning the scheduler frontier (now private), the runtime commit (ticket 003 transaction), the replay restoration constructor (ticket 001 projection evidence), and the TUI/no-human callers. After this ticket exactly one runtime writer (the canonical commit) and one restoration writer (the verified constructor) exist; event timestamps and TUI/debug paths cannot move the frontier.
4. Constitutional invariants motivating this ticket: `INV-009` (meaningful state changes require events), `INV-010` (every event needs a cause model), `INV-018` (deterministic replay), `INV-092` (replay is tested), `INV-112` (event/replay time is the authoritative ordering; the scheduler clock orders/validates but is not client authority). An ordinary event timestamp must not be usable as a command to move the frontier, and no TUI/debug path may assign it.
5. Enforcement surface (deterministic-replay + fail-closed atomicity): this ticket establishes the all-or-nothing commit. The failure-atomicity test snapshots physical/agent/epistemic/log/frontier/checksums before a forced prepare failure and asserts byte/structural identity after; the runtime marker-chain properties assert exactly one `TimeAdvanced` per increment with `prior_tick == reconstructed_frontier`, `resulting_tick == prior_tick + 1`, envelope/payload/cause agreement, that no ordinary event timestamp alone changes the reconstructed frontier, that a no-human scheduled wait cannot skip the marker, and that accelerating N ticks yields N chained markers. No epistemic-leakage path is introduced (the frontier is world-stream temporal authority, not actor-known state).
6. Schema / visibility change: `current_tick` is **resealed** from a public field to a private field plus a read-only accessor and a typed restoration constructor — a visibility/shape change, **breaking-internal**. Consumers: ~14 TUI read sites + internal scheduler reads, **all migrated to the accessor in this diff** (local compile-atomicity); no out-of-crate consumer exists. The restoration constructor is a *distinct* typed entry point (verifies ticket-001 temporal-projection evidence); it does not share the ordinary advancement setter.
7. Removal blast radius (rename/removal): deletes `sync_scheduler_frontier_to_appended_events` (`scheduler.rs:1731`; call sites `scheduler.rs:1332` + tests at `scheduler.rs:2768,5846,5860`), the legacy `advance_world_one_tick` (`scheduler.rs:360`; callers `advance_until` and `submit_entry_with_world_advance` reroute to the ticket-003 transaction), and the TUI frontier assignments (`app.rs:321` + `run_no_human_day`'s `final_tick` write). Grep blast radius — `.claude/skills/` none, `docs/` referenced only as prose in spec 0048 (no code citation), `specs/` only 0048, code tree the sites above; all updated/removed in this diff.

## Architecture Check

1. Making the frontier unrepresentable-to-write outside the canonical commit (private field + read-only accessor + a separate verified restoration constructor) eliminates the alternate temporal authorities at the type level rather than guarding them at runtime — a downstream caller *cannot* assign the frontier or fabricate a runtime restoration token. Deleting `sync_scheduler_frontier_to_appended_events` and rerouting the no-human proposals into the one-tick transaction means event timestamps are inputs to the commit, never commands that move time. Collapsing `submit_entry_with_world_advance` to request-construction + result-handling makes the world-advancing control one atomic transition instead of "commit action, then attempt world step."
2. No backwards-compatibility aliasing/shims: `advance_world_one_tick`, the sync helper, and the TUI setters are deleted, not wrapped or retained behind a flag. The atomic flip is the whole point — a partial cutover that left any bypass live would violate §4.3.

### Implementer-recorded choice (spec §9.4, settled doctrine)

- **Temporal restoration input**: how the initial frontier is represented in a save/snapshot without letting ordinary runtime callers set it. A verified restoration constructor (verifying ticket-001 temporal-projection evidence) or a snapshot-temporal projection is preferable to a general setter. Record the chosen representation and confirm it does not expose a runtime setter.

## Verification Layers

1. `INV-112` singular authority -> codebase grep-proof (compile-fail): downstream code cannot assign the frontier or construct a runtime restoration token (`crates/tracewake-core/tests/negative_fixture_runner.rs`); `grep "current_tick =" crates/tracewake-tui` returns nothing.
2. `INV-018`/`INV-092` marker chain -> replay/golden-fixture check (runtime properties): one `TimeAdvanced` per increment; `prior_tick == reconstructed_frontier`; `resulting_tick == prior_tick + 1`; no ordinary timestamp moves the frontier; a no-human scheduled wait cannot skip the marker; accelerating N ticks yields N chained markers.
3. `INV-009`/`INV-010` fail-closed atomicity -> manual review + unit (all-or-nothing): a forced prepare failure after the controlled input is inspected leaves physical/agent/epistemic/log/frontier/checksums byte/structurally identical.
4. Single-layer note N/A — three distinct invariants map to three distinct proof surfaces above.

## What to Change

### 1. Privatize the frontier; add accessor + restoration constructor

In `crates/tracewake-core/src/scheduler.rs`, make `current_tick` private; add a read-only accessor; add a typed restoration constructor that sets an initial frontier only after verifying ticket-001 temporal-projection evidence. Migrate every internal read to the accessor.

### 2. Delete the bypasses and reroute callers

Delete `sync_scheduler_frontier_to_appended_events` and the legacy `advance_world_one_tick`. Reroute `scheduler::no_human` (`run_no_human_day`/`advance_no_human`) so scheduled proposals become inputs to the ticket-003 one-tick transaction. In `crates/tracewake-tui/src/app.rs`, collapse `submit_entry_with_world_advance` to request-construction + result-handling against the one-tick transaction, delete the `self.scheduler.current_tick = …` assignment and the `run_no_human_day` `final_tick` write, and migrate the ~14 read sites to the accessor. `advance_until` now loops the ticket-003 transaction (its salient-stop wiring is ticket 005).

### 3. Atomicity + marker-chain + compile-fail witnesses

In `crates/tracewake-core/tests/world_step_coordinator.rs`, add the all-or-nothing failure test (Layer 3), the single-marker success-order witness, and the runtime marker-chain property set (Layer 2). In `crates/tracewake-core/tests/negative_fixture_runner.rs`, add the compile-fail frontier fixture (Layer 1).

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — privatize frontier; accessor; restoration constructor; delete sync helper + `advance_world_one_tick`; reroute no-human; shared with tickets 003/005)
- `crates/tracewake-tui/src/app.rs` (modify — collapse `submit_entry_with_world_advance`; delete assignments; migrate reads to accessor; shared with ticket 005)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — failure-atomicity, single-marker, marker-chain witnesses; shared with tickets 001/003/006)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — compile-fail frontier fixture; shared with tickets 002/005)

## Out of Scope

- Wiring `advance_until`'s salient stop and the interval summary to the typed holder-known delta, and deleting the raw-log interval builder — ticket 005.
- Building the one-tick transaction itself — ticket 003 (this ticket flips callers onto it).
- The replay temporal projection the restoration constructor verifies — ticket 001.
- The non-vacuous held-equal human/no-human differential — ticket 006 (this ticket's tests are the atomicity/marker-chain unit witnesses, not the full differential).

## Acceptance Criteria

### Tests That Must Pass

1. Forced-prepare-failure test: after a step rejects post-input-inspection, physical/agent/epistemic/log/frontier/checksums are byte/structurally identical to the pre-call snapshot.
2. Marker-chain properties: accelerating N ticks yields exactly N chained `TimeAdvanced` markers with valid `prior/resulting/+1` ancestry; a no-human scheduled wait emits its marker; no ordinary event timestamp alone moves the reconstructed frontier.
3. `grep -rn "current_tick *=" crates/tracewake-tui crates/tracewake-core/src/scheduler.rs` shows only the canonical commit + restoration constructor as writers; `cargo test -p tracewake-core && cargo test -p tracewake-tui` pass.

### Invariants

1. Exactly one runtime writer (canonical commit) and one restoration writer (verified constructor) of the frontier; the field is private (`INV-112`).
2. A world-advancing control is one atomic transition — no partial commit survives a step failure (`INV-009`/`INV-010`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — all-or-nothing failure, single-marker order, runtime marker-chain properties.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify) — compile-fail: no downstream frontier assignment / runtime restoration-token construction.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core --test negative_fixture_runner`
3. `cargo test -p tracewake-core && cargo test -p tracewake-tui` (full cutover compiles and passes across both crates — the local compile-atomicity boundary).

## Outcome

Completed: 2026-06-23

Privatized `DeterministicScheduler::current_tick` behind `current_tick()` and added typed restoration constructors from the ticket-001 temporal projection/rebuild evidence. The runtime frontier now advances through the canonical `transact_world_one_tick` commit path; ordinary event timestamps and TUI/debug paths cannot assign the scheduler frontier.

Deleted the legacy `advance_world_one_tick` and `sync_scheduler_frontier_to_appended_events` bypasses, then rerouted the TUI submit/wait path, no-human scheduled wait path, and `advance_until` through the ticket-003 transaction. Human controlled proposal rejection rolls back the staged marker, authorities, log, frontier, and checksums; scheduler/non-human rejected proposals may still commit diagnostic evidence. The no-human replay boundary now restores from a rebuild report rather than assigning `final_tick`.

Added/updated witnesses for the failure-atomic transaction, chained `TimeAdvanced` marker ancestry, ordinary timestamp non-authority, no-human scheduled wait marker emission, and downstream compile-fail frontier assignment. The direct writer grep reports only the canonical commit assignment in `scheduler.rs`; restoration writes happen through typed struct construction rather than a public setter.

Verification run:

1. `cargo test -p tracewake-core` — passed.
2. `cargo test -p tracewake-tui` — passed.
3. `cargo fmt --all --check` — passed.
4. `cargo clippy -p tracewake-core --all-targets -- -D warnings` — passed.
5. `cargo clippy -p tracewake-tui --all-targets -- -D warnings` — passed.
6. `git diff --check` — passed.
7. `grep -rn "current_tick *=" crates/tracewake-tui crates/tracewake-core/src/scheduler.rs` — printed only `crates/tracewake-core/src/scheduler.rs:754:        self.current_tick = resulting_tick;`.
