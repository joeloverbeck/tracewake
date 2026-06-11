# 0021PHA3APOSREB-012: Remainder closures — window-credit helper, typed trace diagnostics, eat reason partition, helper clippy lock, INV-087 decision record

**Status**: DONE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`scheduler`, `agent/trace`, `actions/defs/eat`), `tracewake-content` (census fixpoint, clippy allowlisting), `clippy.toml`; INV-087 decision record
**Deps**: `specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-088, 090, 091, 093, 095)

## Problem

Five low-severity closures. (1) Window completion-credit attribution diverges: the
mid-loop site credits the *sweeping* window unconditionally (ignoring the returned
completion tick) while the final sweep credits every window containing the tick — a
late-processed completion can credit a window the actor did nothing in, masking a
genuine `WindowNoProgress` (the inverse of `ORD-HARD-064`); and the two
active-execution selectors apply different filters (`routine_window_family` has the
`deadline_tick` filter, `active_routine_execution_for_actor` does not) — two sites,
one question, again (`ORD-HARD-088`). (2) `DecisionTraceRecord.typed_diagnostic` is
constant defaults (Failed traces say blocker `none`) and the 9/15-field legacy
deserialization shapes fabricate an empty context hash indistinguishable from a
genuinely-empty context (`ORD-HARD-090`; INV-105, INV-020). (3) `eat.rs` failure
reasons encode actor-unobservable distinctions ("carried by another actor" vs
"carrier absent") in the actor-facing `reason` slot — latent INV-106
(`ORD-HARD-091`). (4) The known-food census is depth-1 (wrapper-calling-wrapper
chains in allowlisted files fail open), the helper is `pub` workspace-wide outside
census scope, and the 50-entry legacy allowlist carries one generic rationale
(`ORD-HARD-093`). (5) Bind-time perception events tension INV-087's letter
("possession may not create events") vs doc 07's practice — an explicit owner
decision the spec deliberately does not pre-decide (`ORD-HARD-095`).

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: the mid-loop credit inserts under the sweeping
   `window.window_id` unconditionally while the final sweep uses
   `window.contains_tick(completion_tick)` (`scheduler.rs`); `deadline_tick`
   appears only in `routine_window_family`;
   `TypedDiagnosticFields::decision_default` at `from_trace` and
   `HolderKnownContextHash::from_canonical_lines(&[])` in the legacy
   deserialization arms (`agent/trace.rs`); the reason strings at
   `eat.rs::access_failure`; `function_names_before_helper_calls`
   (`fixtures_load.rs`) is single-pass (no fixpoint); `clippy.toml` carries a
   populated `disallowed-methods` list (per-entry path + reason) that does not list
   `populate_known_food_sources_for_all_actors`; `bind_actor` (`app.rs`) appends
   perception events via `record_current_place_perception_and_project`.
2. Verified against spec 0021 (reassessed 2026-06-11; operator-verified): findings
   ORD-HARD-088/090/091/093/095; 095 is a decision-record deliverable — the spec
   takes no position; the recorded options are (i) move the bind-time perception
   into the actor's own decision transaction, or (ii) record a doctrine
   clarification (a deliberate constitutional touch requiring explicit owner
   sign-off — never amended silently).
3. Cross-artifact boundary under audit: the "two sites, one question" class —
   window-credit attribution and execution selection must each have one shared
   implementation; the trace record's typed fields must be the authoritative
   diagnostic substrate; the food-helper containment must be compiler-enforced
   across the workspace, not census-scoped.
4. INV-105 restated: typed/structurally-inspectable diagnostics are the
   authoritative substrate — constant defaults are decoration. INV-087: possession
   may not create events (the tension under decision). INV-106 (latent): validation
   failure feeds replanning without leaking validator-only truth. INV-020: a
   fabricated empty hash is invented history.
5. Replay/serialization surface touched (ORD-HARD-090): typing the absent hash as
   absent (`Option`) and deriving `typed_diagnostic` from the trace outcome change
   `DecisionTraceRecord`'s canonical serde — roundtrip re-locked; legacy-shape
   removal is conditional on confirming no retained log carries them (grep
   goldens/fixtures first; if any do, the shapes stay with the absent-hash typing
   only — recorded resolution, no-shims rule respected either way). Window-credit
   unification changes diagnostics (which window is credited), not events —
   no-human goldens may reprice diagnostics expectations only.
6. Schema shape (real item-6 case): `DecisionTraceRecord` field retype
   (`HolderKnownContextHash` → `Option<…>` for legacy shapes) and derived
   `typed_diagnostic` — consumers are the trace serializer/deserializer, debug
   views, and trace tests, all in-workspace (local atomic unit: retype + consumers
   in this one ticket). Breaking-internal, no compatibility shim.
7. Removal blast radius: the legacy 9/15-field deserialization shapes (if removed)
   — consumers are `deserialize_canonical` callers and any retained logs (grep
   first, see item 5); the proposer-facing reason-string distinctions in eat.rs —
   consumers: none route `EatFailed.reason` into actor-known context today
   (verified), so the partition move is consumer-free.

## Architecture Check

1. One shared `credit_completion` helper (keyed by `contains_tick`) used by both
   crediting sites, and one shared eligible-execution iterator used by both
   selectors, close the "two sites, one question" class structurally — the same
   shape `ORD-HARD-065`'s fix established for eligibility bounds. Deriving
   `typed_diagnostic` from the trace outcome makes the typed surface authoritative
   (INV-105) instead of decorative. The clippy `disallowed-methods` route converts
   the bespoke text census into a compiler-enforced, rationale-carrying allowlist
   (established repo convention — the list already exists), and the census fixpoint
   closes wrapper-chain depth; the two mechanisms are complementary (compiler for
   the helper, census for wrappers).
2. No backwards-compatibility aliasing/shims: the divergent credit/selector
   implementations are unified, not kept; the legacy trace shapes are removed or
   honestly typed — never silently repaired.

## Verification Layers

1. ORD-HARD-088 -> three-window test with a strictly-mid-window completion (both
   sites agree via the shared helper); deadline-expired-execution selection test
   (both selectors agree).
2. INV-105/020 (ORD-HARD-090) -> Failed-trace test asserting a non-default typed
   blocker; absent-hash roundtrip tests; retained-log grep recorded.
3. INV-106 latent (ORD-HARD-091) -> guard/test: `EatFailed`'s actor-facing reason
   carries only actor-observable codes; the unobservable distinction lives in
   `absence_ancestry`.
4. ORD-HARD-093 -> depth-2 synthetic wrapper-chain fails the fixpoint census;
   clippy `disallowed-methods` entry + `#[expect]` at allowlisted sites compiles
   workspace-wide; per-site rationale carried by the `#[expect]` reasons.
5. INV-087 (ORD-HARD-095) -> the decision record itself: owner decision posed and
   recorded (acceptance artifact + conformance index); if option (ii), the
   doctrine-clarification is a separately-approved change, not landed here.

## What to Change

### 1. Shared credit + selector (ORD-HARD-088)

`credit_completion` helper used by both sites; one eligible-execution iterator;
tests above.

### 2. Typed trace diagnostics (ORD-HARD-090)

Derive `typed_diagnostic` from outcome/rejection items; type the absent hash as
absent; resolve the legacy shapes per the retained-log grep (recorded).

### 3. Eat reason partition (ORD-HARD-091)

Collapse the actor-facing reason to observable codes; move the distinction to
`absence_ancestry`.

### 4. Helper containment (ORD-HARD-093)

Census fixpoint + depth-2 synthetic; `clippy.toml` `disallowed-methods` entry with
containment rationale; `#[expect(clippy::disallowed_methods, reason=…)]` at each
allowlisted call site (the 50 legacy fixture sites + the explicit wrapper —
discovered set within `crates/tracewake-content/src/fixtures/`).

### 5. INV-087 decision record (ORD-HARD-095)

Pose the owner decision (move bind-time perception into the decision transaction vs
doctrine clarification with sign-off); implement option (i) if chosen; record the
decision + rationale in the acceptance artifact and conformance index. No silent
doctrine touch.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/agent/trace.rs` (modify)
- `crates/tracewake-core/src/actions/defs/eat.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)
- `crates/tracewake-content/src/fixtures/` allowlisted call sites (modify — `#[expect]` annotations; discovered set, parent dir exists)
- `clippy.toml` (modify)
- `crates/tracewake-tui/src/app.rs` (modify — only if decision option (i) is chosen; as surfaced)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify — decision record row)

## Out of Scope

- Generative advance flushing (archive/tickets/0021PHA3APOSREB-011.md) and completion crossing
  events (ticket 0021PHA3APOSREB-006) — coordinate the `scheduler.rs` mechanical
  merges.
- Any doctrine amendment text — option (ii), if chosen, is a separately-approved
  constitutional change outside this ticket.

## Acceptance Criteria

### Tests That Must Pass

1. Three-window mid-window-completion test and deadline-expired selection test green
   via the shared implementations; no remaining divergent site (grep-proof).
2. Failed-trace typed-blocker test; absent-hash roundtrips; retained-log grep
   recorded with the legacy-shape resolution.
3. Eat reason partition test green; no actor-facing unobservable distinction
   (grep-proof on the reason construction).
4. Depth-2 census synthetic fails pre-fix, passes post-fixpoint; clippy gate green
   workspace-wide with rationale-carrying `#[expect]`s.
5. The INV-087 decision is recorded (artifact + conformance row); chosen branch
   implemented or correctly deferred. `cargo test --workspace` green.

### Invariants

1. One implementation per question: window crediting and execution eligibility each
   have exactly one shared decision site.
2. Typed diagnostic fields are derived from real outcomes — never constant
   defaults; absent provenance is typed as absent, never fabricated.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/scheduler.rs` (mod tests) — credit/selector tests.
2. `crates/tracewake-core/src/agent/trace.rs` (mod tests) — typed-blocker +
   absent-hash roundtrips.
3. `crates/tracewake-core/src/actions/defs/eat.rs` (mod tests) — reason partition.
4. `crates/tracewake-content/tests/fixtures_load.rs` — fixpoint census + depth-2
   synthetic.

### Commands

1. `cargo test -p tracewake-core scheduler`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo fmt --all --check && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Implementation Outcome (2026-06-11)

Closed all five remainder items:

1. Scheduler window completion credit now goes through one `credit_completion`
   helper keyed by `DayWindow::contains_tick`; both routine-selection paths use
   the same `eligible_routine_execution_for_actor` selector with the deadline
   filter. The no-human TUI count changed from the old over-credited
   `routine_events=8` to the shared-selector result `routine_events=5`.
2. `DecisionTraceRecord.actor_known_context_hash` is `Option<HolderKnownContextHash>`:
   current records carry `Some(hash)`, while retained legacy 9/15-field shapes
   deserialize as absent instead of fabricating an empty hash. Decision trace typed
   diagnostics are derived from the trace outcome, and replay enforces context hash
   re-derivation only when a hash is present.
3. `EatFailed.reason` no longer exposes whether food is carried by another actor
   or by an absent carrier; actor-facing access failures use
   `food source not reachable`, and the unobservable branch lives only in
   `absence_ancestry`.
4. The blanket known-food helper census now follows wrapper chains to fixpoint and
   has a depth-2 synthetic. `clippy.toml` bans
   `FixtureSchema::populate_known_food_sources_for_all_actors`; each legacy fixture
   call carries a local `#[expect(clippy::disallowed_methods, reason = ...)]`, and
   a new negative fixture proves the ban.
5. INV-087 / ORD-HARD-095 is recorded as a deferred owner decision in
   `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`. This ticket did
   not amend `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`; a doctrine
   clarification still requires explicit owner approval.

## Verification (2026-06-11)

Passed:

1. `cargo test -p tracewake-core scheduler`
2. `cargo test -p tracewake-core agent::trace`
3. `cargo test -p tracewake-core actions::defs::eat`
4. `cargo test -p tracewake-content --test fixtures_load`
5. `cargo test -p tracewake-core --test negative_fixture_runner`
6. `cargo test -p tracewake-tui --test command_loop_session`
7. `cargo test -p tracewake-tui --test tui_acceptance`
8. `cargo fmt --all --check`
9. `cargo clippy --workspace --all-targets -- -D warnings`
10. `cargo build --workspace --all-targets --locked`
11. `cargo test --workspace`
