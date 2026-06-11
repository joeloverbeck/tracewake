# 0019PHA3AGENREA-003: Generative reachability honesty and evidence-document corrections

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` test oracle (`tests/generative_lock.rs`, `tests/support/generative.rs`, scheduler entry as surfaced); source guard in `anti_regression_guards.rs`; conformance-index row rewrite; dated correction in the 0018 acceptance report
**Deps**: `tickets/0019PHA3AGENREA-001.md`, `tickets/0019PHA3AGENREA-002.md` (generative tier rebuilt over the corrected surface, spec §8); `specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md` (ORD-HARD-044, §6)

## Problem

The generative lock's duration-terminal and interruption "reachability" is fabricated
by the test harness (`ORD-HARD-044`, high — the spec's headline finding).
`generative_lock.rs` advances each generated case via `no_human::advance_no_human`,
whose body contains no `PendingCompletion` handling — the completion machinery
(`append_due_completions`, `build_sleep_completion_events`,
`build_work_completion_events`) lives only on the `run_no_human_day` path. The test
then calls its own `append_generated_duration_terminals`, which invokes the completion
builders directly and appends the results, so `Reachability.duration_terminal`,
`Reachability.interruption`, and the `terminal_kinds.len() >= 2` floor are satisfied by
harness-manufactured events: a regression breaking scheduler-driven completion would
leave the lock green. The conformance-index row "0018 generative reachability contract"
states the generated sequences "exercise … duration terminals, interruptions," and
`reports/0018_ord_life_cert_scoped_acceptance.md` §4 presents the counters without
disclosing the fabrication — the lineage's first evidence overstatement since 0016.
Per the lineage's enforcement reading: a lock that is green because the harness
manufactured the state it asserts on has failed even though it passes.

## Assumption Reassessment (2026-06-11)

1. Verified against current code at `main` `5af8660`: `advance_no_human`
   (`scheduler.rs`, the free no-human advance entry) contains no completion handling
   across its body; `append_due_completions` is called only from the
   `run_no_human_day` path; `append_generated_duration_terminals` is defined and called
   in `generative_lock.rs` after the advance; the production day loop and the TUI
   (`app.rs::run_no_human_day`) use the completion-bearing path, so this is strictly a
   lock-honesty gap, not a production reach defect.
2. Verified against `specs/0019_…_HARDENING_SPEC.md` ORD-HARD-044 + §6 + §9 (reassessed
   2026-06-11): the correction direction — drive the corpus through
   `run_no_human_day` per window, or extend the generative advance entry to process
   pending completions — is the implementer's recorded choice; spec §9 directs that any
   replay divergence surfaced by the path change is evidence the lock was hollow, to be
   fixed by making the generator's world/window seeding genuinely reach the regimes,
   never by keeping the fabricator.
3. Cross-artifact boundary under audit: the generative oracle's evidence contract —
   every `Reachability` counter and corpus-diversity assertion derives solely from
   events the advance call under test emitted, with the evidence documents
   (conformance index, acceptance report) claiming exactly what the oracle proves.
4. Lock-durability doctrine restated (INV-091–098 spirit): a gate's evidence must be
   produced by the path under test; no-human tests prove the world continues without a
   human only if the asserted events come from the no-human advance path itself.
5. Deterministic-replay surface touched: the generative differential oracle
   (live-vs-replay checksums, prefix replay) now runs over completion-bearing
   histories; live==replay must hold at every boundary, and the
   marker-append relation (both physical and agent checksums) remains green. This
   ticket changes test orchestration and evidence documents — if the implementer
   chooses to extend the generative advance entry, the extension reuses the existing
   `append_due_completions` machinery rather than duplicating completion semantics, so
   no second completion authority is introduced.
6. Adjacent contradictions classified as required consequences handled in this ticket:
   the conformance-index generative row and the 0018 acceptance report §4 overstate the
   fabricated reach — both corrections are §6 deliverables that land with this ticket
   (spec §8), using the lineage's dated-correction convention
   (`## 2026 correction (spec 0019)`, mirroring the 0016 report's correction by 0017).

## Architecture Check

1. Deriving reachability from the advance call's returned/appended event list (not the
   final log) makes post-hoc appends structurally unable to feed the counters — the
   oracle then proves scheduler-path reach by construction. Demoting
   `append_generated_duration_terminals` to a builder-unit test (or deleting it)
   preserves the only legitimate thing it proved (builders produce well-formed events)
   without letting it stand in for reach. A source guard banning direct
   completion-builder calls in `generative_lock.rs` outside the designated builder-unit
   test locks the pattern against reintroduction.
2. No backwards-compatibility aliasing/shims: the fabricator is removed or demoted, not
   kept as a fallback when the advance path fails to reach a regime — an unreached
   regime fails the suite loudly.

## Verification Layers

1. Lock honesty (INV-091–098) -> reworked `generative_lock.rs`: reachability counters
   recomputed from advance-emitted events only; sleep-block, duration-terminal, and
   interruption assertions fail on zero counters with the fabricator absent.
2. Anti-reintroduction -> new guard in `anti_regression_guards.rs`: source scan of
   `generative_lock.rs` asserting no direct calls to
   `build_sleep_completion_events`/`build_work_completion_events` outside the
   designated builder-unit test (the source-scan census pattern).
3. INV-018 differential integrity -> live-vs-replay and prefix-replay assertions green
   over the completion-bearing corpus; `assert_marker_append_does_not_change_physical_checksum`
   (which asserts both checksums) green.
4. Evidence-document honesty -> grep-proofs: the conformance row describes
   advance-emitted reachability; the 0018 report carries the
   `## 2026 correction (spec 0019)` section naming the fabricated terminals and the
   single-seed interruption coverage.

## What to Change

### 1. Drive the corpus through the completion-bearing path

Per the recorded implementer choice: run each generated case via `run_no_human_day`
per window, or extend the generative advance entry to process pending completions by
reusing `append_due_completions`. Adjust `support/generative.rs` world/window seeding
as needed so sleep/work blocks genuinely complete and interrupt on the advance path
(spec §9: fix the generator, never re-add the fabricator).

### 2. Advance-emitted-only reachability

Recompute every `Reachability` counter and `terminal_kinds` from the events the
advance call emitted; delete the post-advance append before counting.

### 3. Demote or delete the fabricator

`append_generated_duration_terminals` becomes a builder-unit test (asserting the
builders produce well-formed terminal events) or is deleted outright.

### 4. Builder-call ban guard

`anti_regression_guards.rs`: the source-scan guard per Verification Layer 2.

### 5. Evidence-document corrections (§6)

Rewrite the "0018 generative reachability contract" row in
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` to state what the
corrected tier proves; add the dated `## 2026 correction (spec 0019)` section to
`reports/0018_ord_life_cert_scoped_acceptance.md` recording that §4's
duration-terminal/interruption reachability was produced by a test-side builder helper
and that interruption coverage was single-seed.

## Files to Touch

- `crates/tracewake-core/tests/generative_lock.rs` (modify)
- `crates/tracewake-core/tests/support/generative.rs` (modify — seeding as surfaced)
- `crates/tracewake-core/src/scheduler.rs` (modify — only under the extend-the-advance-entry direction)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `reports/0018_ord_life_cert_scoped_acceptance.md` (modify — dated correction section)

## Out of Scope

- The tamper relation, per-family floors, and multi-seed requirements
  (ticket `-004`, which depends on this ticket's corrected counter derivation).
- Mutation perimeter over the completion builders (ticket `-005`).
- Production completion semantics — `run_no_human_day`'s machinery is reused, not
  changed.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test generative_lock` — reachability green from
   advance-emitted events only; fabricator absent or demoted; zero counters fail.
2. `cargo test -p tracewake-core --test anti_regression_guards` — builder-call ban
   guard green (and red on a synthetic violation during development).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No `Reachability` counter can be satisfied by an event the advance call under test
   did not emit.
2. The conformance index and acceptance reports claim exactly what the oracle proves —
   any reach concession is recorded, never overstated.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/generative_lock.rs` — completion-bearing corpus;
   advance-emitted-only counters; builder-unit test (if the demote direction is
   chosen).
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — builder-call ban source
   guard.

### Commands

1. `cargo test -p tracewake-core --test generative_lock`
2. `grep -n "2026 correction (spec 0019)" reports/0018_ord_life_cert_scoped_acceptance.md`
3. `cargo test --workspace`

## Outcome

Completed: 2026-06-11

What changed:

- `advance_no_human` now collects scheduled sleep/work starts from the proposal
  pipeline and processes due completions through the shared `append_due_completions`
  path before the advance-completed marker.
- `generative_lock.rs` no longer imports or calls the sleep/work completion builders
  directly and no longer appends generated duration terminals after the advance.
- Added `generative_lock_cannot_fabricate_duration_terminals` to fail if the old
  fabricator symbols or direct completion-builder calls return to the generative
  oracle.
- Rewrote the conformance row as the `0019-corrected generative reachability
  contract`, and added `## 2026 correction (spec 0019)` to the 0018 acceptance report
  documenting the old evidence overstatement.

Deviations from original plan:

- No generator seed/window changes were needed. After the advance path reused the
  shared due-completion machinery, the existing generated corpus reached duration
  terminals and interruption through advance-emitted events.

Verification results:

- `cargo test -p tracewake-core --test generative_lock` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards` — passed.
- `rg -n "2026 correction \(spec 0019\)|test-side|advance_no_human|generative_lock_cannot_fabricate_duration_terminals|0019-corrected generative reachability" reports/0018_ord_life_cert_scoped_acceptance.md docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — found the corrected row and report section.
- `rg -n "build_sleep_completion_events|build_work_completion_events|append_generated_duration_terminals|generated_duration_completion" crates/tracewake-core/tests/generative_lock.rs` — returned no matches, proving the direct builder/fabricator symbols are absent from the oracle.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
