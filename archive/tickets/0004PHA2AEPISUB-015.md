# 0004PHA2AEPISUB-015: Documentation updates — README and fixture contracts

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — documentation only (`README.md`, fixture-contract docs).
**Deps**: 0004PHA2AEPISUB-006, 0004PHA2AEPISUB-011, 0004PHA2AEPISUB-013

## Problem

Documentation must be updated only after code/tests prove the behavior (Spec 0004 §21). The README must document the new TUI commands (`notebook`, `debug epistemics`, `debug beliefs <actor_id>`, `debug observations <actor_id>`), the `check_container`/semantic-action example, and an example session showing Tomas discovering absence through check/search rather than open-container magic; and Phase 2A fixture contracts must be documented while preserving Phase 1 fixture contracts. No documentation may claim Phase 2 is complete (only Phase 2A).

## Assumption Reassessment (2026-06-06)

1. `README.md` exists at repo root and documents the Phase 1A command loop (`view`, `bind`, `do`, numeric selection, `wait`, debug commands); it has no `notebook`/`debug epistemics`/`check_container` entries. The TUI commands land in ticket 011, the `check_container` action in ticket 006, and the fixtures in ticket 013 — this docs ticket trails them so no doc references an unbuilt surface.
2. The required doc surfaces and the "Phase 2A, not full Phase 2" caveat are fixed by Spec 0004 §21. The SPEC_LEDGER entry is explicitly deferred to post-landing and is carried by the capstone (ticket 016), which gates on exit evidence — this ticket does not flip the ledger.
3. Shared boundary under audit: this is a cross-cutting docs ticket whose Files-to-Touch are markdown only; it cites the command/action/fixture surfaces from tickets 006/011/013 and must land after them to avoid a staleness window.
4. Invariant motivating this ticket: `INV-065` (TUI is a primary product interface — its commands must be documented and reachable) and `INV-071` (mechanics hidden from play are unfinished — documentation of reachable commands is part of the product surface).
5. Documentation-only / no-leak surface: the README example session must show Tomas's embodied discovery without leaking ground truth (no "Mara stole" line in the embodied transcript), mirroring the no-leak contract the code enforces. No code/validation/replay surface is touched, so there is no determinism or enforcement change here; the underlying behavior is already enforced by tickets 006/009/011.

## Architecture Check

1. A single trailing docs ticket lets the README command table, the `check_container` example, and the example session land atomically once the commands, action, and fixtures all exist coherently — avoiding per-ticket doc churn and a staleness window where the README references a not-yet-built command. Fixture contracts are documented alongside, preserving Phase 1 contracts.
2. No backwards-compatibility shims: documentation is additive; no doc claims Phase 2 completion.

## Verification Layers

1. TUI surface documented (`INV-065`) -> grep-proof: `README.md` contains `notebook`, `debug epistemics`, `debug beliefs`, `debug observations`, and a `check_container`/`check.container.` example after this ticket.
2. No-leak example (`INV-067`) -> manual review: the README example session shows embodied discovery via check/search with no ground-truth/culprit string in the embodied portion.
3. Phase scoping correct (`INV-098` honesty) -> grep-proof: no doc string claims "Phase 2 complete"; Phase 2A scope and deferrals are stated.

## What to Change

### 1. README command + action documentation

Update `README.md` to add `notebook`, `debug epistemics`, `debug beliefs <actor_id>`, `debug observations <actor_id>`, and a `check_container` / `check.container.<container_id>` semantic-action example; update the example session to show Tomas discovering absence through check/search, not open-container magic.

### 2. Fixture contracts

Document the Phase 2A fixture contracts (setup, allowed actions, expected events/projections, acceptance assertions, forbidden shortcuts) for the seven Phase 2A fixtures, preserving the Phase 1 fixture contracts. (Co-locate with the existing fixture-contract documentation surface; if fixture contracts live inline in the fixture files, this ticket adds the prose contract blocks there.)

### 3. Phase-scope caveat

State explicitly that this is Phase 2A, not full Phase 2, and what remains deferred.

## Files to Touch

- `README.md` (modify — commands, `check_container` example, example session, Phase 2A caveat)
- `crates/tracewake-content/src/fixtures/strongbox_001.rs` (modify — fixture-contract prose for the Phase 2 extension; file already extended by 0004PHA2AEPISUB-013) — as surfaced; if a separate fixture-contract doc surface is used, document there instead.

## Out of Scope

- The `docs/4-specs/SPEC_LEDGER.md` Spec 0004 entry (carried by the capstone, ticket 016).
- Any code/validation/replay change (tickets 006/011/013 own behavior).
- Doctrine content in `CLAUDE.md`/`AGENTS.md` beyond agent-guidance, per §21 (only if new test command names are introduced).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -E "notebook|debug epistemics|debug beliefs|debug observations|check\.container\." README.md` returns matches for every new command/example.
2. The README example session reflects the Phase 1A→2A command set and contains no embodied ground-truth/culprit leak.
3. `grep -i "phase 2 complete" README.md docs` returns nothing; Phase 2A scoping is present.

### Invariants

1. Documentation describes only landed, reachable behavior (this ticket trails 006/011/013).
2. No doc overstates completion beyond Phase 2A.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and the underlying behavior is covered by tickets 006/009/011/013 and the capstone (016).`

### Commands

1. `grep -nE "notebook|debug epistemics|debug beliefs|debug observations|check\.container\." README.md`
2. `cargo test -p tracewake-tui --test readme_sample_session` (if the README sample session is asserted by the existing harness, it must still pass)
3. A grep/manual-review boundary is correct here because the ticket changes only markdown; behavioral verification lives in the cited implementation tickets and the capstone.

## Outcome

Completed on 2026-06-07.

Changed:

1. Updated `README.md` with Phase 2A scoping, the new notebook/debug epistemic commands, a `check.container.strongbox_tomas` semantic-action example, a no-leak example session, and Phase 2A fixture contracts.
2. Updated the README sample-session test so the documented command set is exercised against the TUI binary.
3. Exposed visible container `check.container.<container_id>` semantic actions in the embodied view model and loaded fixture initial beliefs into the TUI epistemic projection so the documented notebook/check workflow is reachable and truthful.

Deviation:

1. This ticket was scoped as documentation-only, but the README would have documented unreachable behavior without two narrow implementation fixes: visible container check actions were not present in the action menu, and TUI startup did not seed authored initial beliefs into the epistemic projection.

Verification:

1. `grep -nE "notebook|debug epistemics|debug beliefs|debug observations|check\.container\." README.md`
2. `grep -Rni "phase 2 complete" README.md docs` returned no matches.
3. `cargo test -p tracewake-tui --test readme_sample_session`
4. `cargo fmt --all --check`
5. `cargo build --workspace --all-targets --locked`
6. `git diff --check`
