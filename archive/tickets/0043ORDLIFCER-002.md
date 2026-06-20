# 0043ORDLIFCER-002: Kill the three need_accounting.rs duration-accounting seed survivors with event-backed behavior witnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new behavior/property witnesses in `tracewake-core` (test-additions by default; production correction in `need_accounting.rs` only if a survivor reveals a real defect).
**Deps**: None

## Problem

The 0042 ORD-LIFE-CERT audit exposed three missed mutants in `crates/tracewake-core/src/need_accounting.rs`, mapped to `ORD-LIFE-01`, `ORD-LIFE-08`, and `ORD-LIFE-12`, that no existing test kills:

1. `replace < with <=` in `DurationInterval::contains_tick` — makes the body-exclusive duration start boundary inclusive.
2. `replace && with ||` in `duration_intervals` — actor-ownership and "not-already-logged" guards stop being jointly required.
3. `replace == with !=` in `duration_intervals` — the current-start event-identity membership test reverses, allowing duplicate ownership or excluding a legitimate current start.

Static inspection (spec §9.3) suggests at least one existing helper test begins the accounting window at the same tick as the duration start — a shape that cannot distinguish `<` from `<=` because enumeration starts at `start + 1`. The fix is event-backed public-consequence witnesses, not private-helper assertions.

(spec §3.1, §5.1–§5.4, §9.2–§9.3)

## Assumption Reassessment (2026-06-20)

1. The three operators map exactly at the authoring baseline `2befc078d96c3b221cad3aa7a0d9b04493a2f0cd` (= current HEAD): `self.start_tick < tick` at `need_accounting.rs:88` inside `DurationInterval::contains_tick` (`:87`); the `&&` joining actor-ownership and not-already-logged guards at `:105-106`; the `event.event_id == start.event_id` membership test at `:109`, inside `duration_intervals` (`:92`). Helpers `terminal_ticks_by_start` (`:158`) and `duration_start_interval` (`:185`) build the intervals; `NeedDeltaApplied` is an `EventKind` consumed by replay (`crates/tracewake-core/src/replay/rebuild.rs:415`). Verified by grep this session. Per the deep-researched-spec convention, symbol + operator is authoritative; the cited `:line:col` is advisory and must be re-reconciled by symbol if the final tree moves it.
2. The cross-cutting kill family fixtures named in spec §7.5 exist: `sleep_spanning_window_boundary_charges_each_tick_once_001`, `wait_then_window_passive_charges_each_tick_once_001`, `sleep_interrupted_by_severe_need_prorates_recovery_001`, `work_block_failed_then_sleep_succeeds_001` (under `crates/tracewake-content/src/fixtures/`). Single-charge behavior is asserted today in `crates/tracewake-core/tests/anti_regression_guards.rs` and `crates/tracewake-content/tests/golden_fixtures_run.rs`; `need_accounting.rs` carries a `#[cfg(test)] mod tests` (`:201`). Verified by grep this session.
3. Cross-artifact boundary under audit: the duration-accounting derivation path — accepted `DurationStarted`/terminal/interruption events → `duration_intervals`/`DurationInterval::contains_tick` → per-actor/per-need/per-tick ledger → `NeedDeltaApplied` ancestry → live projection/metrics → empty-projection replay rebuild. The witness must travel this public seam, not assert the private interval helper's literal result (spec §4.12, §5.1).
4. Invariant motivation: INV-001 (bounded, event-sourced needs with single-owner accounting), INV-011 (current-state-only simulation forbidden — accounting must reconstruct from events), INV-018 (deterministic replay reconstructs the same need states/metrics). The three seeds map directly to single-charge and duration-ownership, so per spec §4.14 they may not close as "non-critical"; they are caught, or after a source transformation proven rigorously equivalent/unreachable with independent signoff.
5. Enforcement surfaces touched: event-backed need accounting + deterministic replay (the rebuilt-from-empty-projection equality), and the actor-known/hidden-truth firewall via the mandatory negative controls (INV-099–104, INV-024). The witnesses must (a) compare live projection/metrics against an empty-projection replay rebuild — introducing no replay nondeterminism — and (b) include a paired negative case changing only hidden truth / debug attachment / unrelated-actor input / duplicate-current-start representation, so the kill cannot be achieved by leaking truth. No production accounting owner is duplicated (no second writer).

## Architecture Check

1. An event-backed public-consequence witness is cleaner than a helper-literal assertion: per cargo-mutants guidance and spec §4.12, a private-function miss is killed by a public-behavior test that breaks when the private function is wrong. Asserting single-charge ownership, body-exclusive/terminal-inclusive interval ownership, `NeedDeltaApplied` ancestry, and replay-rebuild equality kills the mutants AND locks the ORD-LIFE accounting contract, where `assert!(interval.contains_tick(...))` or `assert_eq!(duration_intervals(...).len(), ...)` would be tautological (spec §4.12 rejected shapes).
2. No backwards-compatibility aliasing/shims; no test-only branch in production, no direct projection/state insertion to manufacture an oracle, no second need/duration accounting owner. Production code in `need_accounting.rs` changes only if a survivor reveals a real defect (then with change rationale recorded).

## Verification Layers

1. INV-001 single-charge → core test: for every actor/need/eligible tick, exactly one owning accounting event covers that tick; per-actor/per-need ledger totals equal the event-backed per-tick ledger (kills the `&&`→`||` cross-actor-ownership and `==`→`!=` duplicate-ownership seeds).
2. Body-exclusive start / terminal-inclusive end (INV-008 ordinary-life accounting) → core test: a duration start strictly inside the accounting window `(A,B]` (A < S ≤ B) is not owned at tick `S`, body ticks `S+1..=T` are owned exactly once (kills the `<`→`<=` seed); a control with the start exactly at `from_exclusive` proves the test is not tied to a vacuous boundary.
3. INV-018 replay determinism → replay/golden-fixture check: rebuilding need states/metrics/diagnostics from an empty projection over the retained event log equals live application; `NeedDeltaApplied`/duration/terminal ancestry is reconstructed identically.
4. INV-024 / INV-099–104 firewall → core test (paired negative): changing only hidden food/route/affordance/workplace/schedule truth, debug attachment, or an unrelated actor's start/terminal does not change the subject actor's ledger, regimes, metrics, or replay checksum.
5. Mutation kill → cargo-mutants grep-proof: `cargo mutants -f 'crates/tracewake-core/src/need_accounting.rs'` shows the three operators `caught`.

## What to Change

### 1. Shared event-backed accounting witness family

Add a witness family (per spec §5.1) with member-specific retained cases that: load/construct a legal ordinary-life fixture with ≥2 actors and a duration action; enter through the actor-known transaction / action-pipeline seam where feasible (or justify the narrowest production entry that still emits canonical events); retain the accepted start/terminal/need-delta/event ancestry; independently expand elapsed-duration coverage into actor/need/tick ledger rows from event payloads; assert exactly one owner per actor/need/tick with no cross-actor contamination; compare live projection/metrics against an empty-projection replay rebuild; and retain one mutant-active failure artifact per identity. The oracle must not call the private interval helper to compute its expected answer.

### 2. Seed-specific distinguishing cases

- **`<`→`<=` (`contains_tick`, §5.2):** window `(A,B]`, start `S` with `A < S ≤ B`, terminal `T` with `S < T ≤ B`. Prove tick `S` is not owned by the duration body, the action-start/awake path and duration-body path do not both charge `S`, ticks `S+1..=T` are owned exactly once, terminal inclusion holds, ledger totals/final need values match the event-backed expectation, and replay reconstructs it. Negative controls: start exactly at `from_exclusive` (non-vacuous), unrelated-actor start at the same tick (no effect), debug/hidden-truth change (no effect).
- **`&&`→`||` (`duration_intervals`, §5.3):** a one-true/one-false guard row — subject actor `A` accounted, `current_start` belongs to actor `B`, not yet in the log, `A` and `B` distinguishable. Original excludes `B`'s start from `A`; the `||` mutant includes it. Witness `A`'s wrong duration ownership / need-replay metrics, not an interval-vector length. Second row: same actor, current start already logged, no duplicate ownership. Negative controls per §5.3.
- **`==`→`!=` (event-identity membership, §5.4):** (1) legitimate current start absent from log while an unrelated event is present — original includes the start, mutant wrongly excludes it (missing duration-body accounting + live/replay mismatch); (2) same event already logged — original idempotent, mutant duplicates ownership. Oracle compares event-ID ancestry, per-tick ownership, final need values/metrics, replay reconstruction — not `log.events().any(...)` or `duration_intervals.len()` alone. Negative controls per §5.4.

### 3. Production correction only if warranted

`need_accounting.rs` production logic changes only if a survivor reveals a genuine defect (then record the change rationale per `tickets/README.md`). Default disposition is test-only kill.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — primary witness home, as surfaced)
- `crates/tracewake-core/src/need_accounting.rs` (modify — `#[cfg(test)] mod tests` for supplemental helper diagnostics, and production correction only if a survivor reveals a real defect)
- `crates/tracewake-content/src/fixtures/<new-evidence-fixture>_001.rs` (new — only if no existing §7.5 fixture distinguishes a seed; must travel the production seam, no gameplay-doctrine change)

## Out of Scope

- The full configured campaign run, completion proof, and triage register (→ -004); this ticket proves the three seeds with a targeted `-f need_accounting.rs` rerun only.
- The CI in-diff trigger / guard convergence (→ -001).
- PTY/wrapper diagnosis and the supervised launch (→ -003).
- Live ORD-LIFE re-proof and the replacement acceptance artifact (→ -005).
- Adding any survivor to `.cargo/mutants-baseline-misses.txt`, or closing a seed as "non-critical" (forbidden — spec §4.14, §12).
- Changing ordinary-life gameplay doctrine to make a fixture convenient (spec §7.5).

## Acceptance Criteria

### Tests That Must Pass

1. New core witness: body-exclusive start / terminal-inclusive ownership with a start strictly inside the window — kills `contains_tick` `<`→`<=`.
2. New core witness: one-true/one-false guard row (cross-actor current start) — kills `duration_intervals` `&&`→`||`.
3. New core witness: unrelated-log-event + duplicate-representation rows over event identity — kills `duration_intervals` `==`→`!=`.
4. Paired negative controls (hidden-truth / debug / unrelated-actor / duplicate current-start) leave the subject actor's ledger, metrics, and replay checksum unchanged.
5. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --locked` all pass.
6. `cargo mutants -f 'crates/tracewake-core/src/need_accounting.rs' --no-shuffle` (with `.cargo/mutants.toml` in effect) shows all three seed operators `caught`.

### Invariants

1. INV-001 / INV-008: exactly one owning accounting event per actor/need/eligible tick; duration start tick is body-exclusive, terminal is inclusive.
2. INV-018 / INV-024: live accounting equals empty-projection replay rebuild; no hidden-truth, debug, or unrelated-actor input changes the subject actor's accounting (witness asserts certified public behavior, not a private-helper internal).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — the shared event-backed witness family plus the three seed-specific distinguishing cases and their negative controls.
2. `crates/tracewake-core/src/need_accounting.rs` (`mod tests`) — optional supplemental helper-level diagnostics (not the primary oracle).

### Commands

1. `cargo test --locked -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --locked`
3. Targeted kill proof: `cargo mutants -f 'crates/tracewake-core/src/need_accounting.rs' --no-shuffle` — confirm `contains_tick` `<`→`<=`, `duration_intervals` `&&`→`||`, and `duration_intervals` `==`→`!=` are each `caught`. The full configured posture is -004's scope.

## Outcome

Completed: 2026-06-20

Added event-log-backed public behavior witnesses in
`crates/tracewake-core/tests/anti_regression_guards.rs` for the three
`need_accounting.rs` seed survivors:

- `need_accounting_duration_body_start_is_exclusive_and_terminal_is_inclusive`
  proves a duration start tick remains body-exclusive while the terminal tick is
  included in the charged duration body.
- `need_accounting_current_start_requires_subject_actor_and_absent_log_identity`
  proves a current start must belong to the subject actor and must not already
  be represented in the event log, including a duplicate-current-start control.
- `need_accounting_current_start_identity_ignores_unrelated_log_events` proves
  event-id membership is specific to the current start and is not suppressed by
  unrelated log events.

The witnesses use the public `classify_actor_tick_regimes` /
`classify_actor_tick_regimes_with_start` event-log seam and do not add a second
production accounting owner. No production logic changed.

Verification:

- `cargo test --locked -p tracewake-core --test anti_regression_guards need_accounting_` — passed.
- `cargo test --locked -p tracewake-core --test anti_regression_guards` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo test --workspace --locked` — passed.
- `cargo mutants --workspace --no-shuffle -F 'crates/tracewake-core/src/need_accounting\.rs' --list` — confirmed the focused regex lists only `need_accounting.rs` mutants, including the three historical identities.
- `cargo mutants --workspace --no-shuffle -F 'crates/tracewake-core/src/need_accounting\.rs'` — completed with `42 mutants tested in 3m: 36 caught, 6 unviable`; `mutants.out/missed.txt` and `mutants.out/timeout.txt` were empty.
- `rg -n '88:25: replace < with <=|106:13: replace && with \|\||109:45: replace == with !=' mutants.out/caught.txt` — confirmed all three historical seed identities were caught.

Command-shape note: the literal ticket command
`cargo mutants -f 'crates/tracewake-core/src/need_accounting.rs' --no-shuffle`
was attempted first, but cargo-mutants 27.1.0 combined `-f` with the checked-in
`.cargo/mutants.toml` standing config and reported the full 2877-mutant
denominator. That full-denominator attempt was interrupted before mutant
interpretation because it belongs to `0043ORDLIFCER-004`, not this ticket. The
focused `-F` command above supplied the ticket-local seed-kill proof without
shrinking or laundering the configured denominator.
