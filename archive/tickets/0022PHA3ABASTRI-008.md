# 0022PHA3ABASTRI-008: Scheduler reject-loudly closure, derived cause-set, and apply totality

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — scheduler error paths (`scheduler.rs`), event metadata (`events/envelope.rs`), apply totality (`events/apply.rs`), test-oracle guards
**Deps**: 0022PHA3ABASTRI-004

## Problem

Three reject-loudly/derivation residues. `ORD-HARD-108`: scheduler completion paths
still panic on event-log-derived data — `actor_has_open_body_exclusive_duration` calls
`open_body_exclusive_starts(log, …).expect("duplicate duration terminals are rejected
before no-human scheduling")` (the callee returns a log-derived
`Err(DuplicateDurationTerminal)`), plus `apply_agent_event(...).expect(...)` on the
stuck-diagnostic completion paths — and the `ORD-HARD-085`-promised guard against
`assert!`/`expect` on log-derived data was never implemented. `ORD-HARD-112`:
`EventKind::requires_cause` is a hand-maintained `matches!` list and
`action_emitted_event_kinds_have_cause_disposition` checks a hardcoded array — a new
action-emitted kind defaults to no-cause-required and passes. `ORD-HARD-113`: the
world-applier totality guard's synthetic asserts substrings of a hardcoded string
literal (artifact-shaped), and no agent-stream totality analogue exists — a new
agent-stream kind routes to the misleading `Err(NonAgentEvent)`.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: the `.expect("duplicate duration terminals are rejected…")`
   site in `crates/tracewake-core/src/scheduler.rs` (19 `expect(` occurrences
   file-wide; the log-derived subset is the target); `requires_cause` is a
   `pub const fn` `matches!` list in `crates/tracewake-core/src/events/envelope.rs`;
   `action_emitted_event_kinds_have_cause_disposition`
   (`anti_regression_guards.rs`) opens with a hardcoded `required = [EventKind::…]`
   array; the world totality synthetic (`synthetic_apply`) is a string literal
   asserted via `.contains`, and `grep agent_stream anti_regression_guards.rs` returns
   nothing.
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-108` (operator-
   verified) and `112`/`113` (operator-verified at reassessment); verified holding:
   the apply-side typed errors (`DuplicateNeedTickCharge`, `MalformedElapsedTicks`)
   and both appliers' fail-closed catch-alls landed in 0021 — this ticket closes the
   scheduler-side and guard-side remainder.
3. Shared contract under audit: the reject-loudly posture (INV-020 — replay rejects
   unsupported history via typed errors, never a process abort) across
   scheduler-read paths, and the event-kind metadata contract (`EventKindMetadata`)
   as the derivation source for cause requirements and apply-arm totality.
4. Constitutional invariants restated: INV-020 (typed rejection), INV-010 (every event
   needs a cause model — the derivation makes the required-cause set total over
   action-emitted kinds).
5. Deterministic-replay surface: converting panics to typed errors on log-derived
   reads strengthens replay rejection without changing accepted-history semantics;
   corrupt-log scheduler fixtures assert identical typed rejection live and on
   replay. No epistemic surface touched.
6. Implementer-recorded choice (spec-assigned): the expect/assert guard carries an
   allowlist for genuinely unreachable states, each entry with a rationale — the
   allowlist contents are the implementer's recorded decision, written into the guard
   with per-entry justification (recorded in the acceptance artifact per spec §7
   item 7).
7. Change rationale (no silent retcon): the scheduler `.expect`s predate the 0021
   reject-loudly conversion and were missed by its apply-side scope (R-28 closure);
   the guard arrays predate the derivation discipline the lineage now requires.

## Architecture Check

1. Deriving the required-cause set and apply-arm totality from `EventKindMetadata`
   (the registry `event_kind_metadata_is_total` already proves total) replaces two
   hand lists with one source of truth — a new kind must declare its cause and apply
   disposition to compile past the guards. Factoring the world-arm scan into a
   function fed both real and synthetic bodies makes the totality negative behavioral
   (the `-004` bijection census's routed-negative requirement).
2. No backwards-compatibility aliasing/shims: panicking paths are converted, not
   wrapped; the hardcoded arrays are deleted.

## Verification Layers

1. INV-020 reject-loudly -> corrupt-history fixtures: a log with duplicate
   body-exclusive duration starts (and a malformed completion payload) reaching the
   scheduler paths yields typed errors live and on replay — no panic.
2. Expect/assert guard (`ORD-HARD-085` lock, delivered here) -> codebase grep-proof:
   the guard scans apply/completion/scheduler paths for `assert!`/`.expect(` on
   log-derived data, with the rationale-carrying allowlist; synthetic violating case
   fires; census-registered.
3. INV-010 derived cause-set -> guard check: required-cause derived from
   action-origin metadata over `EventKind::all()`; synthetic new action-emitted kind
   without a cause disposition fails.
4. Apply totality -> guard check: world-arm scan factored and fed both real and
   synthetic bodies (the `ORD-HARD-113` census debt retired); agent-stream analogue
   derives from `EventKind::all()` agent-stream kinds vs. apply arms ∪ a no-op
   allowlist; synthetic new-kind negatives fire on both streams.

## What to Change

### 1. Scheduler typed-error conversion

Convert the log-derived `.expect`/`assert!` sites in `scheduler.rs` completion and
duration-read paths to propagate typed errors (`DuplicateDurationTerminal`,
`ApplyError`); thread the error type through the no-human runner's existing typed
error channel.

### 2. Implement the expect/assert source guard

In `anti_regression_guards.rs`: scan `events/apply.rs`, `scheduler.rs`, and the
action-def completion builders for `assert!(`/`.expect(` on log-derived values;
allowlist genuinely unreachable states with per-entry rationale (implementer-recorded);
synthetic violating case; census registration.

### 3. Derive the required-cause set

Extend `EventKindMetadata` with (or derive from existing metadata) an action-origin
predicate; make `requires_cause` and
`action_emitted_event_kinds_have_cause_disposition` iterate `EventKind::all()` over
it; synthetic uncovered-kind negative.

### 4. Apply-arm totality symmetry

Factor the world-arm scan into a function fed both the real `events/apply.rs` source
and the synthetic negative body; add the agent-stream totality guard (every
agent-stream kind has an explicit arm in `apply_agent_event_with_capability` or an
allowlisted no-op); synthetic negatives on both streams.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/events/envelope.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify — only if arm registration
  needs explicit no-op arms; otherwise untouched)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- The eat emitter routing (`0022PHA3ABASTRI-007`).
- Apply-side typed errors already landed by 0021 (verified holding — not re-touched).
- Converting non-log-derived `.expect`s (config/static invariants stay; the guard's
  allowlist names them).

## Acceptance Criteria

### Tests That Must Pass

1. Corrupt-log scheduler fixtures: duplicate body-exclusive starts and malformed
   completion payloads yield typed errors live and on replay; `grep -n 'expect("duplicate duration terminals' crates/tracewake-core/src/scheduler.rs` returns 0.
2. `cargo test -p tracewake-core --test anti_regression_guards` — expect/assert guard
   (allowlist rationales present), derived cause-set, factored world totality, and
   agent-stream totality all green with firing synthetics.
3. The `-004` census shows the `ORD-HARD-113` debt entry retired (synthetic now
   routed through the production scan).
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No apply/completion/scheduler path panics on event-log-derived data; rejection is
   typed on both live and replay paths (INV-020).
2. Cause requirements and apply-arm coverage are derived from kind metadata — a new
   `EventKind` cannot land without declaring both.

## Test Plan

### New/Modified Tests

1. Corrupt-log scheduler-path fixtures (live + replay typed rejection).
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — expect/assert guard,
   derived cause-set, two-stream totality + synthetics.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-core scheduler`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

No-human scheduler log-derived failures now report typed `NoHumanSchedulerError`
values instead of panicking on duplicate duration terminals, malformed scheduled
completion inputs, or agent-apply failures on scheduler diagnostics/completions.
`NoHumanDayReport` carries those errors while preserving the existing non-fallible
runner API for accepted histories. Duplicate duration terminal checks also run before
passive need classification, so corrupt duration logs do not reach the older
tick-regime panic path.

`EventKindMetadata` now carries `cause_required`, and `EventKind::requires_cause()`
derives from metadata. The action-emitted cause guard scans action sources for emitted
`EventKind::*` references and validates their metadata, with a synthetic uncovered
kind proving the guard fails.

Apply totality guards are now scanner-backed: the world mutating apply-arm check uses
a reusable missing-arm scanner, and a new agent-stream analogue checks all agent
stream kinds against explicit `apply_agent_event_with_capability` arms or the
documented no-op allowlist. The `ORD-HARD-113` meta-lock debt entry was retired to
`SharedScan`, and the new agent, cause, and panic guards are registered in the
meta-lock census.

Verification:

1. `cargo test -p tracewake-core scheduler --no-fail-fast`
2. `grep -n 'expect("duplicate duration terminals' crates/tracewake-core/src/scheduler.rs`
   returned zero matches and exit 1.
3. `cargo test -p tracewake-core --test anti_regression_guards`
4. `cargo fmt --all --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo build --workspace --all-targets --locked`
7. `cargo test --workspace`
