# 0047TUIAUTWOR-001: Foundation 08 Time-controls amendment

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — docs only: `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` (Time controls section)
**Deps**: None

## Problem

Spec 0047 §1 establishes that, in human play, no path invokes an authoritative world step: a human-started `sleep`/`work_block` never reaches its terminal because waiting does not run the loaded simulation. Foundation `08` already states the controlling doctrine — its `## Time controls` section says "Waiting is not skipping causality. Waiting runs the simulation." — but it does not settle the *scope* of a human wait/step, the distinction between a one-tick actor wait and a controller-level continuation/acceleration, or the source boundary for actor-known interval summaries. Per spec 0047 §5 and the driver report's disposition **D-02**, the Time-controls doctrine must be amended (substance + home, no Rust types / scheduler phases / event payloads / test names) so the capability has approved foundation-tier semantics *before* code lands. This is decomposition ticket 1 of the §4.9 outline: establish approved semantics before code.

## Assumption Reassessment (2026-06-22)

1. The amendment target exists and already carries the seed doctrine: `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` has a `## Time controls` section (heading at line 257) whose body reads "Waiting is not skipping causality. Waiting runs the simulation." (line 259). This ticket amends that section in place; it does not create a new section or a new doc.
2. Spec 0047 §5 routes this as "Foundation `08` — amend the Time controls section" with an explicit substance list and an explicit prohibition: "Do not specify Rust types, scheduler phases, event payloads, or test names." Sibling foundation docs `03`/`05`/`06`/`12`/`14` get **no substantive amendment** (§5) — out of scope here. The driver report `reports/tui-human-wait-runs-simulation-research-report.md` §1 disposition D-02 ("Amend") and D-03 ("Retain; cross-reference only if useful") corroborate.
3. Cross-artifact boundary under audit: this is a foundation-tier doctrine amendment, the highest authority tier (`docs/README.md` order: foundation → architecture → execution → reference). Its wording governs — and must stay consistent with — the architecture/execution amendments in 0047TUIAUTWOR-002/003. Those lower tiers operationalize this section; this section must not contradict them, and must remain implementation-neutral so they retain room to specify mechanism.
4. Constitutional invariants motivating the amendment, restated before trusting the narrative: `INV-112` (temporal authority — "Time may validate, but holder-known time must plan"; the scheduler/replay clock orders and validates but is not cognition authority) and `INV-045` (ordinary survival is causal — sleep/fatigue/work must be real enough to cause behavior; fake meter refills are forbidden). The amendment must keep waiting as authoritative causal progression and keep interval summaries holder-known, never omniscient.
5. Enforcement surface governed by doctrine (substrate-only basis — this ticket changes prose, not code, but the prose governs enforcement surfaces a later ticket implements): the actor-knowledge / truth-firewall boundary (`INV-024` no telepathy, `INV-067` embodied mode shows actor-known reality) and deterministic replay (`INV-018`). The amendment must state that stopping and interval summaries are based on perceived/holder-known information and that sleeping/absent actors receive no global event diff or omniscient negative summary — introducing no leakage path the actor-known interval projection (0047TUIAUTWOR-013) would otherwise have to undo, and no claim that a private possessed-actor clock (rather than authoritative event time) governs progression.

## Architecture Check

1. Amending the existing `## Time controls` section in place — rather than adding a new doctrine section or pushing the semantics down into architecture/execution — keeps the single authoritative home for the "waiting runs the simulation" doctrine at the foundation tier, where the lower tiers already point. A second home would split authority and invite drift.
2. No backwards-compatibility aliasing/shims: doctrine prose is amended, not duplicated; the prior wording is extended/sharpened, with no parallel "old vs new" statement left standing.

## Verification Layers

1. `INV-112` temporal authority -> invariants alignment check: the amended section asserts an embodied wait/step advances the authoritative loaded world (not a private possessed-actor clock) and that possession supplies one actor's input without pausing others — cross-read against `INV-112` heading in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`.
2. `INV-045` causal survival -> invariants alignment check: the amended section keeps sleep/work as causal durations and acceleration as repeated authoritative progression.
3. `INV-024`/`INV-067` no-leak -> manual review (epistemic-leakage audit): the amended section states stopping and interval summaries are holder-known, and sleeping/absent actors get no global event diff or omniscient negative summary.
4. Landing -> codebase grep-proof: the new substance phrases resolve in `docs/0-foundation/08_*` under the `## Time controls` heading.

## What to Change

### 1. Amend the `## Time controls` section of `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`

Extend the section (substance only — no Rust types, scheduler phases, event payloads, or test names) to own, per spec 0047 §5:

- An embodied wait/step advances the authoritative **loaded world**, not a private possessed-actor clock.
- Possession supplies one actor's input slot but does not pause other actors or world processes; a one-tick actor wait advances every loaded actor and due process under one transition.
- A one-tick actor wait and a controller-level continuation/acceleration are **distinct** controls.
- Sleep and work remain causal durations; acceleration is repeated authoritative progression, never a single multi-tick jump.
- Stopping conditions and interval summaries are based on **perceived / holder-known** information.
- Sleeping or absent actors receive **no global event diff** and no omniscient negative ("nothing happened") summary.

Keep the existing "Waiting runs the simulation" claim as the anchoring statement the amendment elaborates.

### 2. Execution precondition (recorded, not auto-applied)

This is a foundation-tier amendment: it requires constitutional/foundation owner sign-off before it is applied, per the doc-pack amendment authority (`docs/README.md`). The ticket records the substance and home; it must not be merged by convention without that approval.

## Files to Touch

- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` (modify)

## Out of Scope

- Any code change (kernel, TUI, fixtures, tests) — owned by 0047TUIAUTWOR-005 onward.
- Foundation `03`/`05`/`06`/`12`/`14` edits — §5 assigns them no substantive amendment (cross-references only if the reassess process finds a discoverability gap).
- Architecture/execution/reference amendments — owned by 0047TUIAUTWOR-002/003/004.
- Specifying Rust types, scheduler phase order, event payloads, or test names (§5 prohibition).
- Any `SPEC_LEDGER.md` or glossary change.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "loaded world" docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` returns the new authoritative-loaded-world statement inside the `## Time controls` section.
2. `grep -niE "continuation|acceleration" docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` shows the one-tick-wait vs controller-continuation distinction is stated.
3. `grep -niE "holder-known|perceived|no global event diff|nothing happened" docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` confirms the holder-known stopping/summary and no-omniscient-negative language landed.

### Invariants

1. The amended section never asserts a private possessed-actor clock as the timeline authority (must read "authoritative loaded world").
2. The amended section contains no Rust type, scheduler-phase, event-payload, or test-name (foundation-tier prohibition; the substance stays implementation-neutral).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep landing + invariants-alignment review) and the governed enforcement surfaces are implemented and tested by 0047TUIAUTWOR-005 onward.`

### Commands

1. `grep -nE "## Time controls" docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
2. `grep -niE "loaded world|continuation|acceleration|holder-known|no global event diff" docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
3. Narrower command is correct here: a foundation doc amendment is verified by landing greps + a manual `INV-112`/`INV-045`/`INV-024` alignment read, not by `cargo` tests — no code changes in this ticket.
