# 0022PHA3ABASTRI-012: Content typed-diagnostic and generative lock-shape hygiene

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: Yes — routine diagnostic vocabulary (`agent/routine.rs`), content validation (`validate.rs`), state doc-comment (`state.rs`), generative locks (`generative_lock.rs`, `tests/support/generative.rs`)
**Deps**: 0022PHA3ABASTRI-004

## Problem

`ORD-HARD-116`: (a) `RoutineStep::FailWithTypedDiagnostic { diagnostic: String }`
accepts any free text at load, and the no-sleep gate exact-matches two spellings
(`"no_sleep_affordance" | "NoSleepAffordance"`) — better than the substring check
`ORD-HARD-098` replaced, but not a closed vocabulary (INV-105 spirit); (b)
`PlaceState::new`/`Default` hard-default `visibility_default: Visible` (the
permissive direction) — the loader is fail-closed, but in-code construction silently
defaults. `ORD-HARD-117`: (a) the generative fabricator ban is token co-occurrence
(`EventEnvelope::new` + terminal-kind token), dormant on the real corpus — "support
constructs zero `EventEnvelope`" is incidental, not asserted; (b)
`assert_in_block_displacement_ordering` derives its completion-tick bound from
generator arithmetic (`work.start_tick.advance_by(work.duration_ticks)`), not from
engine-emitted events, so the inequality is structurally always true.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `FailWithTypedDiagnostic` lives in
   `crates/tracewake-core/src/agent/routine.rs` with consumers in
   `crates/tracewake-content/src/validate.rs` (the dual-spelling `matches!` at
   `is_no_sleep_diagnostic`), `agent/trace.rs`, and the fixture
   `no_human_sleep_knowledge_requires_observation_or_record_001.rs`;
   `visibility_default: VisibilityDefault::Visible` in `PlaceState::new`
   (`state.rs`); the generator-derived bound at
   `generative_lock.rs::assert_in_block_displacement_ordering`
   (`scheduled_completion_tick = work.start_tick.advance_by(work.duration_ticks)`);
   the fabricator ban's co-occurrence arm in `anti_regression_guards.rs`
   (`context.contains("EventEnvelope::new")`).
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-116`/`117`
   (operator-verified at reassessment) and the verified-holding record that
   `ORD-HARD-084`/`086`/`092`'s corrections are substantive — this is their shape
   residue.
3. Shared contract under audit: the routine-diagnostic vocabulary crossing the
   core↔content boundary (core defines `RoutineStep`; content validates fixture
   text), and the generative tier's engine-sourced-evidence rule (exec 09).
4. Constitutional motivation restated: INV-105 (typed diagnostics are the
   authoritative substrate; display strings are not) and INV-022 (prose does not
   decide gates).
5. Fail-closed validation surface: introducing `RoutineDiagnosticKind` parsed at load
   must reject unknown kinds (fail-closed, no silent fallback — the `ORD-HARD-085`
   no-silent-default rule); replay surfaces untouched (diagnostics are validation/
   trace content, not event payload semantics — confirmed: no event schema change).
6. Schema note (additive-vs-breaking): retyping the *fixture-facing* diagnostic field
   from free `String` to a parsed closed enum is breaking-internal for fixture
   authoring (all current fixtures use `no_sleep_affordance`, which joins the enum) —
   consumers enumerated in item 1 are updated in the same diff (local
   compile-atomicity); the dual camelCase spelling is dropped with no alias.
7. Either-or resolution (recorded at Step-4 approval, Q5 default): the
   `VisibilityDefault::Visible` in-code default is **documented** as loader-only
   convenience (doc-comment provenance note at the type and `PlaceState::new`) rather
   than removing the `Default` path — removal would churn in-code constructors with
   no doctrine gain, since the loader boundary is already fail-closed.
8. Change rationale (no silent retcon): the dual spelling and free-text payload were
   the `ORD-HARD-098(b)` correction's partial landing; the generative bounds were the
   `ORD-HARD-086` correction's generator-arithmetic shortcut — both named by the 0022
   audit as R-28 residue.

## Architecture Check

1. A closed `RoutineDiagnosticKind` enum parsed at load makes the diagnostic
   vocabulary a registry (new kinds are deliberate additions, not strings), and the
   gate dispatch becomes a `match` — the same closed-vocabulary shape the disposition
   tags took in `-001`. Engine-sourcing both bounds of the displacement ordering
   (read the scheduled completion from the emitted event pair) binds the assertion to
   behavior, per the checked-coverage principle in the spec's research grounding.
2. No backwards-compatibility aliasing/shims: the camelCase spelling is dropped, not
   aliased; the free-text field is replaced, not shadowed.

## Verification Layers

1. INV-105 closed vocabulary -> load-rejection negative: a fixture carrying an
   unregistered diagnostic kind fails validation with a typed error.
2. Loader-only default honesty -> doc-comment + grep-proof: the provenance note at
   `VisibilityDefault` / `PlaceState::new`; loader fail-closure unchanged
   (`parse_visibility_default` still rejects absence/unknown).
3. Fabricator-ban explicitness (`ORD-HARD-117a`) -> guard check: "support/generative.rs
   constructs zero `EventEnvelope`" asserted as an explicit invariant (not incidental
   co-occurrence dormancy); synthetic support-file `EventEnvelope::new_v1` case
   fires; census-registered (`-004`).
4. Engine-sourced bounds (`ORD-HARD-117b`, exec 09) -> generative check:
   `assert_in_block_displacement_ordering` reads the scheduled completion tick from
   the emitted `WorkBlockStarted`/terminal pair and asserts both bounds engine-side;
   a generator-arithmetic regression (placing the move outside the engine-emitted
   block) fails.

## What to Change

### 1. `RoutineDiagnosticKind` closed enum

Define the enum in `agent/routine.rs` (kinds: the currently-used `no_sleep_affordance`
plus any other live diagnostic values surfaced by grep); parse-and-reject at content
load; retype `RoutineStep::FailWithTypedDiagnostic`'s payload; update
`is_no_sleep_diagnostic` to a `match` on the enum and drop the `NoSleepAffordance`
spelling; update the consuming fixture and `trace.rs` sites.

### 2. Loader-only default provenance note

Doc-comments at `VisibilityDefault` and `PlaceState::new` recording that the `Visible`
default is in-code construction convenience only — authored fixtures must pass the
fail-closed loader column.

### 3. Explicit zero-envelope invariant

In the fabricator-ban guard: assert `tests/support/generative.rs` contains zero
`EventEnvelope` construction tokens as a standing invariant (alongside the existing
co-occurrence ban); synthetic violating case; census registration.

### 4. Engine-sourced displacement bounds

Rework `assert_in_block_displacement_ordering` to read the scheduled completion tick
from the engine-emitted event pair and assert the movement's `sim_tick` against the
engine-side bound (both bounds behavior-derived).

## Files to Touch

- `crates/tracewake-core/src/agent/routine.rs` (modify)
- `crates/tracewake-core/src/agent/trace.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/src/fixtures/no_human_sleep_knowledge_requires_observation_or_record_001.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify — doc-comments only)
- `crates/tracewake-core/tests/generative_lock.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — fabricator-ban
  invariant + census entries)

## Out of Scope

- New diagnostic kinds beyond those live in the tree today (the enum is a registry;
  additions are future deliberate work).
- Removing the `PlaceState` `Default` path (Q5 resolution: document, not remove).
- The generative floors' two-sided encoding (`0022PHA3ABASTRI-004`).

## Acceptance Criteria

### Tests That Must Pass

1. Load-rejection negative: an unregistered diagnostic kind in a fixture fails
   content validation with a typed error; `grep -rn "NoSleepAffordance" crates/`
   returns 0.
2. `cargo test -p tracewake-core --test generative_lock` — engine-sourced bounds
   green; `cargo test -p tracewake-core --test anti_regression_guards` —
   zero-envelope invariant + synthetic firing.
3. `cargo test -p tracewake-content` green with the retyped fixture.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. The routine-diagnostic vocabulary is closed at load; no gate decision keys off
   free text (INV-105/022).
2. Generative ordering evidence is engine-emitted on both bounds; support constructs
   zero event envelopes by asserted invariant.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/validate.rs` (tests) — unregistered-kind rejection
   negative.
2. `crates/tracewake-core/tests/generative_lock.rs` — engine-sourced
   displacement-ordering assertion.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — zero-envelope invariant
   + synthetic + census entries.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo test -p tracewake-core --test generative_lock && cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Implemented. `RoutineStep::FailWithTypedDiagnostic` now carries a closed
`RoutineDiagnosticKind` registry, canonical serialization emits only the stable
snake-case id, and deserialization fails closed with
`RoutineStepParseError::InvalidDiagnosticKind` for unknown diagnostic ids. Content
validation dispatches on the enum instead of free text, the no-sleep fixture uses
the typed diagnostic, and the camel-case routine diagnostic spelling is no longer
accepted.

Documented `PlaceState::new` and `VisibilityDefault` as in-code fixture/test
assembly conveniences, not authored-content fallbacks. Authored fixture visibility
still goes through the fail-closed loader column.

Updated the generative displacement lock to derive both start and terminal bounds
from engine-emitted events, and registered a meta-lock plus synthetic guard for
support-file `EventEnvelope` construction. The support-generator direct-envelope
case now fails explicitly instead of depending on terminal-token co-occurrence.

Acceptance nuance: the literal `NoSleepAffordance` identifier still exists as an
unrelated action rejection `ReasonCode`; the routine diagnostic spelling was removed
from fixture parsing and validation.

Verification:

1. `cargo test -p tracewake-core --test generative_lock`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo test -p tracewake-content`
4. `cargo test -p tracewake-core --test hidden_truth_gates`
5. `cargo fmt --all --check`
6. `cargo clippy --workspace --all-targets -- -D warnings`
7. `cargo build --workspace --all-targets --locked`
8. `cargo test --workspace`
