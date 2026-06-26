# 0052FOUCONFOU-012: F4-09 — live-doc truthing (post-executable-closure)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — documentation only (live conformance/status/navigation evidence rows); no code, no doctrine altitude change
**Deps**: 0052FOUCONFOU-002, 0052FOUCONFOU-003, 0052FOUCONFOU-004, 0052FOUCONFOU-005, 0052FOUCONFOU-006, 0052FOUCONFOU-007, 0052FOUCONFOU-008, 0052FOUCONFOU-010, 0052FOUCONFOU-011

## Problem

Spec 0052 F4-09 / §6.1; closure-order step 12 (doc-truthing lands only **after** executable closure). Architecture `00` (line 107) names `LoadedWorldRuntime::from_loaded_world` as the production entry and `TuiApp::submit_entry_with_world_advance` as the TUI crossing, and R-27 repeats the production-constructor claim; the code at `6495d7d` contradicts both (the production TUI uses `from_initial_state` with `DeterministicScheduler::new` and still orchestrates proposal sequencing / no-human / rebuild / perception / view / debug). The doctrine is correct and the spec ledger's historical rows are already honest; only live conformance/status/navigation evidence is ahead of the code.

This ticket truths the live evidence rows **after** F4-01…F4-08 are implemented and executed at one exact commit: it replaces the loaded-world overclaim with the actual single production bootstrap and current named public-boundary/green-mutation witnesses, updates implementation pointers and evidence shape across the architecture/execution/reference tiers, and corrects R-27/R-28/R-29 **status/evidence only**. It changes no doctrine altitude and mints no risk ID, gate code, or glossary term.

## Assumption Reassessment (2026-06-25)

1. The live doc surfaces exist: `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (loaded-world row at line 107 names `from_loaded_world` + `submit_entry_with_world_advance`), `02`, `04`, `05`, `10`, `13`; `docs/2-execution/05`, `06`, `07`, `10`; `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` and `01_DESIGN_RISK_REGISTER.md` (R-27 at line 333, R-28 at 344, R-29 at 356). All confirmed present.
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §6.1; closure-order step 12. The new production constructor / public commands / sealed receipts / green mutation evidence this ticket points to are landed by 002–011.
3. Cross-artifact boundary under audit: the live conformance/status/navigation documentation vs. the post-implementation code. Every updated evidence row must answer: which production constructor created the runtime, which public command crossed the client boundary, what state/event/projection effect was observed, and which deliberate mutation or negative compile attempt proves sensitivity.
4. Motivating invariant: INV-098 (harsh acceptance) and the validation doctrine reject documentation that treats a non-production or mutation-insensitive path as closure. This ticket lands only **after** the executable witnesses exist (driver §7.2; spec §8 "doc-truthing lands only after the executable witnesses exist").
5. Fail-closed / no-leak / replay doc-governance surface (substrate/doctrine basis): this ticket documents — without changing — the replay-non-reseeding, sole-scheduler-coordinator, read-only-client, and sealed-product enforcement surfaces; it confirms the updated rows introduce no overclaim (a row may become current only after the production TUI test and green canonical mutation result at the same implementation commit are attached). Replay stays non-reseeding, the scheduler stays the sole coordinator, the client stays read-only, and source scans/focused mutation are not presented as exhaustive proof — no doctrine altitude changes.

## Architecture Check

1. Truthing live evidence rows only after executable closure is cleaner than editing docs first because a grep proving `from_loaded_world` appears in a live row is not conformance evidence — the row may become current only after the production TUI test and green canonical mutation result are attached. Treating doc-invariant/anti-regression guards as topology alarms (paired with the required public-boundary and mutation jobs) avoids the artifact-presence-without-behavior trap (R-29).
2. No backwards-compatibility shim and no doctrine change: this is status/evidence truthing within existing doctrine altitude; no invariant, gate, risk ID, or glossary term is minted, and no archived spec/ticket/report/acceptance is edited.

## Verification Layers

1. INV-098 (evidence honesty) -> grep-proof against the post-implementation tree: every updated live row names symbols/tests that exist (the actual production constructor, the public command, the named witnesses, the green mutation result).
2. R-27/R-28/R-29 status -> manual review: status/evidence rows updated only; no risk ID minted, no general risk retired.
3. Doctrine altitude unchanged -> invariants alignment check: replay stays non-reseeding, the scheduler stays sole coordinator, the client stays read-only, source scans/focused mutation are not exhaustive proof.

## What to Change

### 1. Architecture tier (`00`, `02`, `04`, `05`, `10`, `13`)

`00`: replace the loaded-world overclaim with the actual single production bootstrap and current named public-boundary/green-mutation witnesses. `02/04/05/10/13`: update implementation pointers and evidence shape (not doctrine altitude) — replay non-reseeding, sole coordinator, read-only client, split sealed products, path-under-test production bootstrap + standing-green evidence requirements.

### 2. Execution tier (`05`, `06`, `07`, `10`)

Update current commands, products, census, replay restore, and gate procedure to the landed surfaces.

### 3. Reference tier (`00`, `01`)

`00`: update reviewer pointers to the actual public boundary and standing barrier. `01`: update R-27/R-28/R-29 **status/evidence only**; mint no risk ID; retire no general risk.

## Files to Touch

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` (modify)
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (modify)
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` (modify)
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (modify)
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` (modify)
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify — R-27/R-28/R-29 status/evidence only)

## Out of Scope

- Any code change (002–011 own those; this is documentation only).
- Editing archived specs, tickets, reports, acceptance artifacts, or the `SPEC_LEDGER.md` archived rows (immutable; the ledger row + `archive/specs/` move defer to spec acceptance per `docs/archival-workflow.md`).
- Minting any invariant, gate code, risk ID, or glossary term; changing doctrine altitude.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "from_loaded_world\|submit_entry_with_world_advance" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` no longer asserts the overclaimed production entry; the row names the actual production constructor + landed witnesses.
2. Every updated row names symbols/tests that exist in the post-implementation tree (grep-proof) and answers the four evidence questions (constructor / command / observed effect / sensitivity proof).
3. R-27/R-28/R-29 carry updated status/evidence only; `grep -c "R-2[789]" docs/3-reference/01_DESIGN_RISK_REGISTER.md` is unchanged (no risk ID minted/retired); the doc-invariant guard suite passes.

### Invariants

1. No doctrine altitude changes; replay stays non-reseeding, scheduler sole coordinator, client read-only (INV-098, INV-018, INV-069).
2. No archived artifact is edited; no risk ID/gate/glossary minted (spec §6).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is grep-based landing/boundary checks against the post-implementation tree plus the existing doc-invariant/anti-regression guard suite (treated as topology alarms), and an invariants-alignment review confirming no doctrine altitude change.`

### Commands

1. `grep -n "from_loaded_world\|submit_entry_with_world_advance" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md && grep -c "R-2[789]" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
2. `cargo test --workspace` — the doc-invariant guard suite (topology alarms) stays green.

## Outcome

Completed: 2026-06-26

Truth-ed the live 0052 conformance/status/navigation rows after executable
closure without changing doctrine altitude or archived historical artifacts.
Updated the architecture, execution, and reference docs listed by this ticket to
name the current production constructor
`LoadedWorldRuntime::from_bootstrap(LoadedWorldBootstrap, SimTick)`, the content
handoff through `into_runtime_bootstrap`, public TUI crossings through
`TuiApp::submit_semantic_action`, `TuiApp::advance_until`, and command-loop
closed `RuntimeCommand` dispatch, observed actor/process/replay/sealed-product
effects, and the 0052 sensitivity/CI evidence in
`archive/tickets/0052FOUCONFOU-009.md` through
`archive/tickets/0052FOUCONFOU-011.md`.

R-27/R-28/R-29 were updated for status/evidence only. The risk register count
for `R-2[789]` stayed unchanged at `5`; no risk ID, glossary term, or new gate
code was minted. The conformance index no longer contains
`from_loaded_world` or `submit_entry_with_world_advance`; the remaining 0051
references are explicitly historical context.

Verification:

- `grep -n "from_loaded_world\|submit_entry_with_world_advance" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — no output, exit 1 as expected.
- `grep -c "R-2[789]" docs/3-reference/01_DESIGN_RISK_REGISTER.md` — `5`.
- `git diff --check` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
