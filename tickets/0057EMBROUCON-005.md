# 0057EMBROUCON-005: Parity entry, embodied go-to-work golden, and firewall/replay fixtures

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — a new hidden-truth firewall fixture in `tracewake-content` plus its census/golden registration, and embodied go-to-work golden + playable-capability parity + possession-parity tests in `tracewake-tui`
**Deps**: 0057EMBROUCON-002, 0057EMBROUCON-003, 0057EMBROUCON-004

## Problem

Spec 0057 §4.5 and §6. With the embodied follow-on committing (002), surfacing typed blockers/waits/stuck (003), and the marker invariants guarded (004), the feature must be proven TUI-playable-and-no-human with full possession parity and an intact hidden-truth firewall. This ticket lands: the embodied go-to-work golden (possessed `actor_tomas` reaching `workshop_tomas` by movement ancestry through `Continue routine`, then starting/completing a work block); the playable-capability parity entry proving embodied routine continuation reaches the same behavioral progress as the autonomous path for the go-to-work→work_block chain and that bind/unbind does not change the next routine step; the possession-parity extension; the hidden-truth firewall fixture (a target unknown to the actor yields a typed knowledge blocker, not a truth-driven move — INV-099); and replay-reconstructability of the follow-on commit. This is the deliverable-doubles-as-capstone for the feature's behavioral surface.

## Assumption Reassessment (2026-06-30)

1. Registration surfaces verified: `crates/tracewake-content/src/fixtures/mod.rs:178` `all()` and `:451` `by_id` enumerate fixtures; `crates/tracewake-content/tests/fixtures_load.rs:108` holds the source-file census (`"src/fixtures/ordinary_workday_001.rs"` etc.); `crates/tracewake-content/tests/golden_fixtures_run.rs:142` holds the golden-checksum registry (`("ordinary_workday_001", "twf1-…")`). The new firewall fixture registers at all three (structural-consumer model — registration is the wiring), with its golden checksum computed at implementation time. Possession-parity family: `crates/tracewake-tui/tests/tui_acceptance.rs:627/640/1296` `possession_does_not_reset_intention_001`. Parity capability surface: `crates/tracewake-tui/tests/parity/scenario.rs` `run_real_pipeline` / `CapabilityEntry`, with the action census in `parity/census_actions.rs` and the proof in `playable_capability_parity.rs`. Embodied golden home: `crates/tracewake-tui/tests/embodied_flow.rs` (already drives `ordinary_workday_001`).
2. Spec assumption: `specs/0057_…_SPEC.md` §4.5 + §6 govern. §6's marker-not-progress guard is owned by 0057EMBROUCON-004 and the blocked-continuation test by 0057EMBROUCON-003 (referenced, not duplicated here); this ticket owns the embodied golden, possession-parity, hidden-truth firewall, and replay items. The intention id is runtime-derived for `ordinary_workday_001` (routine `routine_tomas_go_to_work`) — the golden reads it from the live view rather than hardcoding (reassessment M3).
3. **Cross-artifact boundary under audit**: the spec-0046 playable-capability parity registry (`tracewake-tui/tests/parity/`) — the embodied routine-continuation capability is added as a parity entry proving embodied behavioral progress equals the autonomous path's for the go-to-work→work_block chain; and the golden/replay census, which gains a new fixture.
4. INV-094 (possession parity is tested — bind/unbind changes input only, not the next routine step), INV-095 (TUI/view-model tests are acceptance tests — a human reaches go-to-work then work through embodied play), INV-099 (truth may validate but not plan — the firewall fixture proves actor-known-only resolution), INV-018 (deterministic replay — the follow-on commit is replay-reconstructable and physical-checksum stable).
5. **Actor-knowledge firewall / no-scripting / replay enforcement surface**: the new hidden-truth firewall fixture and the parity/golden assertions. Confirm: (a) the firewall fixture's hidden workplace/route yields a typed knowledge blocker, never a truth-driven move (INV-099/024); (b) the embodied golden replays byte-identically and is physical-checksum stable (INV-018); (c) the new fixture is content-as-possibility — no behavior-looking fields, no authored outcome chain — passing the `fixtures_load` census and the `forbidden_content` scan (INV-060 / no-scripting); (d) the parity assertions read actor-filtered surfaces only, asserting no hidden-truth leak (INV-093).

## Architecture Check

1. Proving the feature through a parity entry (embodied vs autonomous on one capability surface) plus a hidden-truth firewall fixture makes "possessed can do exactly what autonomous can, from actor-known state, no more" a test contract rather than a claim — the harsh-acceptance bar INV-098 sets. Reusing the existing parity registry and golden/replay census (rather than a bespoke embodied harness) keeps the acceptance surface consistent with spec 0046/0047.
2. No backwards-compatibility aliasing or shims: a new fixture and new test entries only; no production path is wrapped. The new fixture is authored content, not a code shim.

## Verification Layers

1. INV-095 (TUI acceptance) -> embodied golden: possessed `actor_tomas` reaches `workshop_tomas` by movement ancestry via `Continue routine`, then starts/completes a `work_block` — matching `ordinary_workday_001`'s expected `ActorMoved` → `WorkBlockStarted`/`Completed`, no teleport.
2. INV-094 (possession parity) -> parity entry + possession test: embodied and autonomous reach the same routine behavioral progress from equivalent actor-known state; bind/unbind preserves the next routine step (extends `possession_does_not_reset_intention`).
3. INV-099 / INV-093 (firewall; no leak) -> hidden-truth fixture: a target unknown to the actor yields a typed knowledge blocker, not a truth-driven move; parity surfaces leak no hidden truth.
4. INV-018 (replay) -> replay/golden check: the embodied follow-on commit is replay-reconstructable and physical-checksum stable; the new fixture's golden checksum is registered.

## What to Change

### 1. Hidden-truth firewall fixture

Add `crates/tracewake-content/src/fixtures/embodied_continue_hidden_workplace_001.rs` — a sibling of `ordinary_workday_001` where the workplace/route is not in the actor's known context. Register it: declare/return it in `fixtures/mod.rs` `all()`/`by_id`, add its source path to the `fixtures_load.rs` census, and add its golden-checksum entry to `golden_fixtures_run.rs`.

### 2. Embodied go-to-work golden

In `embodied_flow.rs`, drive `ordinary_workday_001` through the TUI surface: `Continue routine` reaches `workshop_tomas` by movement ancestry, then a subsequent `Continue routine` starts/completes a `work_block` — reading the runtime-derived intention id from the live view, asserting events and no teleport.

### 3. Playable-capability parity entry + possession parity

Add the embodied routine-continuation capability entry (`parity/census_actions.rs` + `playable_capability_parity.rs`) proving embodied behavioral progress equals the autonomous path for the go-to-work→work_block chain, and a `tui_acceptance.rs` test that controller bind/unbind does not change the next routine step (extends the `possession_does_not_reset_intention` family).

## Files to Touch

- `crates/tracewake-content/src/fixtures/embodied_continue_hidden_workplace_001.rs` (new — hidden-truth firewall fixture)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register in `all()`/`by_id`)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify — add fixture source path to the census at :108)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — golden checksum + replay entry for the new fixture)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify — embodied go-to-work golden via `Continue routine`)
- `crates/tracewake-tui/tests/parity/census_actions.rs` (modify — embodied routine-continuation capability entry)
- `crates/tracewake-tui/tests/playable_capability_parity.rs` (modify — embodied==autonomous parity proof; bind/unbind preserves next step)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify — possession-parity: bind/unbind preserves next routine step)

## Out of Scope

- The marker-not-progress guard (0057EMBROUCON-004) and the blocked-continuation/stuck test (0057EMBROUCON-003) — referenced by §6, owned there.
- Any production-code change — this ticket is fixtures + tests; the behavior was landed by 002/003.
- The written acceptance artifact (0057EMBROUCON-007).
- The post-work `stuck_actors` accounting observation (spec §1.2).

## Acceptance Criteria

### Tests That Must Pass

1. Embodied go-to-work golden: possessed `actor_tomas` reaches `workshop_tomas` by movement ancestry via `Continue routine`, then starts/completes a `work_block`, matching the fixture contract's expected events with no teleport.
2. Possession-parity: embodied and autonomous reach the same routine behavioral progress from equivalent actor-known state; bind/unbind preserves the next routine step.
3. Hidden-truth firewall: in `embodied_continue_hidden_workplace_001`, a target unknown to the actor yields a typed knowledge blocker, not a truth-driven move.
4. Replay: the embodied follow-on commit is replay-reconstructable and physical-checksum stable; the new fixture loads deterministically and validates (census + golden).
5. `cargo test --locked -p tracewake-content && cargo test --locked -p tracewake-tui` — fixtures/census/golden + embodied golden/parity/possession green.

### Invariants

1. Whatever routine behavioral progress an autonomous actor makes from a given actor-known state, the possessed actor makes from the equivalent state through embodied affordances — and bind/unbind grants no extra capability and changes no next routine step.
2. The embodied follow-on resolves only from actor-known context; a hidden-only target yields a typed knowledge blocker; no parity/golden surface leaks hidden truth.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/embodied_continue_hidden_workplace_001.rs` + `tests/golden_fixtures_run.rs` + `tests/fixtures_load.rs` — new firewall fixture, golden checksum, deterministic-load census.
2. `crates/tracewake-tui/tests/embodied_flow.rs` — embodied go-to-work golden.
3. `crates/tracewake-tui/tests/playable_capability_parity.rs` + `tests/parity/census_actions.rs` — parity entry (embodied==autonomous).
4. `crates/tracewake-tui/tests/tui_acceptance.rs` — bind/unbind preserves next routine step.

### Commands

1. `cargo test --locked -p tracewake-content --test golden_fixtures_run --test fixtures_load` — new fixture census + golden + replay.
2. `cargo test --locked -p tracewake-tui --test embodied_flow --test playable_capability_parity --test tui_acceptance` — embodied golden, parity, possession.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — four-gate suite.
