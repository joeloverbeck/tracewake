# 0057 Embodied Routine Continuation Behavioral Progress and Possession Parity Spec

**Status**: PROPOSED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and is promoted to `archive/specs/` on acceptance; it is
never promoted to the live `docs/4-specs/` tier. It is **not** a certification audit, a
mutation-remediation pass, a Phase-4 entry claim, or a latest-`main` certification. It does
not amend constitutional invariants, define gate semantics, or weaken execution gates. It
uses the canonical hardening-spec house structure of its sibling specs (e.g. `0046`, `0047`),
not the `docs/NN_*` narrative-document style. It is the direct successor to `0047`
(TUI authoritative world advance): `0047` gave the human path an authoritative world tick;
this spec closes the remaining seam where a *possessed* actor cannot make routine behavioral
progress through the embodied affordance the TUI presents for it.

## 0. Baseline statement and source discipline

- **Driver.** Hands-on TUI play during an iterative bug-hunt loop on
  `ordinary_workday_001`, driving the same code path as the interactive TUI
  (`crates/tracewake-tui/src/run.rs::submit_and_render` → `TuiApp::submit_semantic_action`).
  The reproduction is recorded in §3; it is the originating analysis and mints no doctrine,
  invariant, risk identifier, gate code, or glossary term.
- **Baseline commit.** Re-verified against repository `HEAD`
  `8f024fc43fdad945da4eb78be6ccc0ab93a5050d` at authoring time. The named symbols below are
  authoritative; any line numbers are provenance only and are not relied upon.
- **Source discipline.** A commit hash named here is audit/provenance only. This spec
  recommends and scopes work; it declares no latest-`main` certification, Phase-4 entry, or
  second-proof entry. When executed, the implementation must name its own exact implementation
  commit, not assume this baseline is latest `main`. A manifest is path inventory only;
  branch/default-branch/code-search evidence is not proof of exact-commit content.
- **Authority posture.** Subordinate to `docs/0-foundation/`, `docs/1-architecture/`,
  `docs/2-execution/`, and `docs/3-reference/`, in that order. It operationalizes existing
  doctrine; where it identifies doctrine that must be sharpened to admit the capability, it
  routes the amendment to the owning tier (§5) as *substance + home* and does not ratify
  wording or mint identifiers.
- **Execution admissibility posture.** `P0-CERT not applicable`. This spec certifies no
  certification gate; it names gate codes and prior certifications only as cross-references to
  existing artifacts, never as local definitions, and makes no certification claim of its own.
- **Ledger timing.** Per the sibling-spec convention, this package receives its
  `docs/4-specs/SPEC_LEDGER.md` archived-row entry at acceptance/closeout, not at proposal.
  This spec authors no ledger row now and makes no change to live `0001` or the ledger.

## 1. Scope

### 1.1 In scope

**The defect.** In embodied (possessed) play, the TUI presents a `Continue routine`
semantic action (`SemanticActionId = continue.routine.<intention_id>`, `ActionId =
continue_routine`) whenever the possessed actor has an active intention. A player following
their routine naturally selects it. Submitting it is **Accepted** but produces **no
behavioral progress**: the possessed actor does not advance the routine's current step, does
not move, the active intention's `current_step` does not change, and no ordinary action
event is committed. Repeated submission is an indefinite no-op. On `ordinary_workday_001`
the possessed `actor_tomas` therefore never reaches `workshop_tomas` and never starts a work
block through the routine affordance, even though:

- the fixture contract expects `ActorMoved home_tomas→workshop_tomas` then
  `WorkBlockStarted/Completed`;
- the *autonomous* no-human path on the same fixture reaches the workshop and records
  `work_completed=1`; and
- the *explicit* primitive `move`/`work_block` embodied actions both work and unlock the next
  step.

**Root cause (two cooperating seams).**

1. `continue_routine` is by design a *marker only*: `build_continue_routine_event`
   (`crates/tracewake-core/src/actions/defs/continue_routine.rs`) emits `ContinueRoutineProposed`
   with `behavioral_progress=false` and `intention_mutated=false`, effects-summary
   "continue routine marker only; behavioral progress requires next ordinary action". This is
   correct per doctrine (a pure marker is not progress — `docs/2-execution/06` and
   `docs/1-architecture/04`). The marker carries the resolved next ordinary action id: the
   embodied entry is built in `projections.rs::phase3a_semantic_actions` with
   `target_ids = [intention_id, intention.current_step]` (e.g. `move`, `work_block`).
2. **Nothing commits the follow-on ordinary action in the embodied path.** The autonomous
   no-human scheduler/planner resolves the routine step into a concrete ordinary action
   (move toward the actor-known workplace place, then `work_block`) and commits it. The
   embodied seam stops at the marker. `TuiApp::submit_entry` runs exactly one pipeline
   transaction (the marker) and returns. `DeterministicScheduler::advance_until` advances
   world ticks with `controlled_proposals: Vec::new()` and an
   `actor_known_interval_actor_id = possessed_actor_id`; consistent with `0047` it supplies
   only the possessed actor's input slot and does not run a full routine-planning sweep for
   the possessed actor. So neither the marker submission nor a subsequent world advance ever
   produces the possessed actor's routine move/work_block.

**This spec scopes the single committed solution and its proof:** an embodied
routine-continuation that, when a possessed actor selects `Continue routine` for an active
routine intention whose current step resolves to an ordinary action, **commits that follow-on
ordinary action through the shared action pipeline**, using an **actor-known** target
resolution identical in admissibility to the autonomous planner's, so the possessed actor
makes the same behavioral progress an autonomous actor would — no more, no less — without
weakening the "a pure `continue_routine` marker is not progress" rule.

1. **Routine-step → ordinary-action resolution shared with the planner.** The follow-on
   ordinary action and its target (e.g. the `move` destination place, the `work_block`
   workplace) must be derived from the same actor-known routine/method resolution the
   autonomous decision transaction already uses (the `RoutineStep::MoveTowardPlace` /
   work-block resolution under `agent/`), not from hidden truth and not from a TUI-local
   shortcut. Truth may validate the committed action; truth may not select it (INV-099).
2. **One marker + one committed follow-on, atomically reported.** A single embodied
   `Continue routine` submission yields the `ContinueRoutineProposed` marker *and* the
   resolved follow-on ordinary action committed through the shared pipeline in one
   embodied transaction, so the player observes behavioral progress (moved / work started /
   modeled wait / typed stuck), and the marker still self-reports `behavioral_progress=false`.
   The follow-on, not the marker, is the progress of record.
3. **Blocked / terminal steps surface typed outcomes, not silent no-ops.** When the resolved
   follow-on cannot be committed (precondition unmet, route blocked, workplace closed by
   belief, need-blocked, intention terminal), the embodied submission returns the existing
   typed rejection / why-not or a modeled wait / typed stuck record — never a silent Accepted
   no-op. Repeated submission inside one routine window without progress must be eligible for
   the same cross-tick stuck detection the no-human path uses
   (`past_expected_progress_window`, `repeated_idle_wait`), so an embodied player is not silently
   wedged.
4. **Possession parity of routine capability.** Whatever routine behavioral progress an
   autonomous actor can make from a given actor-known state, a possessed actor can make from
   the equivalent state through embodied affordances, and vice versa. Binding/unbinding a
   controller does not change which routine step is next, does not reset intention/need/memory
   (existing `possession_does_not_reset_intention` family), and does not grant the possessed
   actor capability the autonomous actor lacks.

### 1.2 Out of scope (non-goals)

- Changing the *autonomous* no-human planner, scheduler cadence, or the `0047` authoritative
  world-tick coordinator. They are correct; this spec consumes their resolution, not rewrites
  it.
- Making `continue_routine` itself "count as progress." The marker stays a marker
  (`behavioral_progress=false`). Doctrine forbidding marker-as-progress
  (`docs/0-foundation/12`, `docs/2-execution/06`) is preserved, not loosened.
- Auto-running a possessed actor's whole routine on a single keystroke, multi-step
  fast-forward, or any change to `advance_until` stop-reason semantics. One `Continue
  routine` commits exactly one routine-step's follow-on ordinary action (or its typed
  block/wait), mirroring one autonomous decision.
- New ontology, new affordance kinds, food/sleep redesign, or institutional behavior.
- The secondary observation below (post-work `stuck_actors` accounting), which is recorded
  for a separate determination, not scoped here.

## 2. Doctrine anchors

- **INV-035 — Routines are defeasible intentions.** A routine drives reaching work, not a
  teleport or puppet string; continuation must produce real reach-and-act, with failure /
  interruption / alternatives available.
- **INV-094 — Possession parity is tested.** Binding changes input only; routine capability
  must not diverge between possessed and autonomous control.
- **INV-095 — TUI/view-model tests are acceptance tests.** A human must be able to reach
  mechanics (here: go-to-work then work) through embodied play. A presented affordance that
  cannot reach its mechanic fails this invariant.
- **INV-098 — Feature acceptance is harsh.** A routine is "done" only when it is TUI-playable
  *and* no-human runnable; today it is only the latter.
- **INV-099 — Truth may validate actions but may not plan them.** The follow-on target must be
  resolved from actor-known context, never from hidden truth.
- **`docs/0-foundation/12` line ≈533** places "follow/interrupt routines" inside embodied-play
  scope; the truth-firewall gate (≈line 557) requires every salient possessed action to show,
  as acceptance evidence, the actor-known context used for proposal generation and provenance
  for its cognition inputs — reinforcing the §4.1 INV-099 resolution. (The behavioral-progress
  requirement itself is carried by the next bullet's `docs/1-architecture/04` /
  `docs/2-execution/06` anchors, not line ≈557.)
- **`docs/1-architecture/04` / `docs/2-execution/06`**: a pure `continue_routine` marker is not
  behavioral progress; progress is a committed ordinary action, a modeled wait with typed
  reason, a duration terminal, or a typed stuck/failure record.
- **`0047`**: possession supplies one actor's input slot and does not freeze others or define a
  local timeline; the embodied input slot for "advance my routine" is exactly what this spec
  wires to behavioral progress.

## 3. Determination (reproduction; recorded so it is not re-litigated)

Driving `TuiApp` against `ordinary_workday_001` / `actor_tomas` (the `run.rs` path):

- Initial embodied view offers `continue.routine.<intention_id>` for the active
  `routine_tomas_go_to_work` intention (`action_id=continue_routine`, enabled),
  `move.to.workshop_tomas` (`action_id=move`), `sleep.here`, `wait.1_tick`. Intention
  `active:routine_tomas_go_to_work:move`. (The `<intention_id>` is runtime-derived from the
  routine assignment, not authored in `ordinary_workday_001`; the §6 golden must read it from
  the live view rather than hardcode it. Note that `intention_tomas_go_work` /
  `routine_tomas_go_work` belong to the separate `no_human_day_001` fixture, not this one.)
- Submitting `continue.routine.*` → **Accepted**, `+4` events (marker + epistemic/world
  bookkeeping), but place stays `home_tomas`, intention stays `:move`, needs unchanged. 38
  consecutive submissions: no movement, no time progress observed in the embodied view.
- Submitting `move.to.workshop_tomas` → **Accepted**, `actor_moved` emitted, place →
  `workshop_tomas`, and `work.block.workplace_tomas` becomes enabled. Primitive path works.
- `advance_until(200)` on the embodied binding: place stays `home_tomas`; the possessed actor
  waits, fatigue/hunger rise, a sleep attempt is interrupted; he never reaches work.
- Autonomous `run_no_human_day` on the same fixture (debug binding): place → `workshop_tomas`,
  `work_completed=1`, planner traces `go_to_work` (tick 4, switched) then `perform_work_block`
  (tick 10, switched).

Holdings (no action; recorded): the marker semantics, the autonomous planner resolution, the
`0047` world-advance coordinator, and the primitive `move`/`work_block` embodied actions are
all individually correct. The defect is solely the missing embodied follow-on commit and its
shared actor-known resolution.

## 4. Findings and remediation requirements

### 4.1 Shared actor-known routine-step resolver (seam: `crates/tracewake-core/src/agent/`)
Factor the autonomous routine-step → ordinary-action(+target) resolution so it is callable
for an embodied submission against a named intention's `current_step`, returning a typed
result: a resolved ordinary `Proposal` (action id + actor-known target ids), or a typed
blocker (precondition/route/workplace/need/terminal), or a modeled-wait directive. It must
consume only actor-known context (the same generation boundary the planner uses) and must be
the *same* resolution the autonomous path uses, not a parallel copy.

### 4.2 Embodied continuation commits the follow-on (seam: `tracewake-core` runtime command surface; `crates/tracewake-tui/src/app.rs` forwards only)
When a possessed actor submits `Continue routine` for an active routine intention, the **core
runtime command** (not the TUI) must, in one embodied transaction: (a) commit the
`ContinueRoutineProposed` marker as today, and (b) commit the §4.1 resolved follow-on ordinary
action through the shared action pipeline (or record its typed block / modeled wait). The
marker+follow-on sequencing lives in the core command surface so no simulation authority moves
into the TUI boundary (INV-008, INV-069); `submit_entry` only forwards the entry and surfaces
the returned receipt. That action receipt returned to the TUI must reflect the follow-on
outcome (moved / work started / why-not / waited / stuck), not merely "Accepted" for the
marker.

### 4.3 No silent wedge; embodied stuck eligibility (seam: stuck detection)
An embodied routine continuation that cannot progress must surface a typed why-not or a
modeled wait, and repeated no-progress continuations within one routine window must be
eligible for the existing cross-tick stuck diagnostics (`past_expected_progress_window`,
`repeated_idle_wait`) on the same terms as the no-human path.

### 4.4 Marker invariants preserved (seam: `crates/tracewake-core/src/actions/defs/continue_routine.rs`)
`continue_routine` keeps `behavioral_progress=false` / `intention_mutated=false`. No code may
count the marker as progress. The progress of record is the committed follow-on, asserted by
ancestry to the marker's `intention_id` and the routine step.

### 4.5 Parity surface extension (seam: spec-0046 parity registry/tests, `crates/tracewake-tui/tests/parity/`)
Add a playable-capability parity entry proving embodied routine continuation reaches the same
routine behavioral progress as the autonomous path for at least the go-to-work→work_block
chain, and that controller bind/unbind does not change the next routine step.

### 4.6 Implementation decomposition (one ticket per reviewable diff)
1. §4.1 shared resolver extraction + unit proof that embodied and autonomous resolution are
   identical for a fixed actor-known state.
2. §4.2 embodied follow-on commit wiring + receipt surfacing.
3. §4.3 typed-block / modeled-wait / stuck eligibility.
4. §4.4 marker-invariant guard tests.
5. §4.5 parity entry + fixture/golden updates.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Home: `docs/2-execution/06` (ordinary life / behavioral progress).** Substance: state
  explicitly that an embodied `continue_routine` selection commits the routine step's
  follow-on ordinary action (or a typed block/wait/stuck) so possessed routine-following is
  behavioral progress, while the bare marker remains non-progress. This sharpens, and does not
  weaken, the existing "marker is not progress" rule.
- **Home: `docs/1-architecture/04` (pipeline/scheduling).** Substance: name the embodied
  follow-on commit as the human-input counterpart to the autonomous planner's step commit,
  routed through the same shared pipeline and the same actor-known resolution.
- No constitutional-invariant amendment is required; INV-035/094/095/098/099 already govern.
  If acceptance review finds the embodied/autonomous parity needs an invariant-level sentence,
  route it to `docs/0-foundation/02` separately; this spec does not pre-author it.

## 6. Required fixtures and tests

- **Embodied go-to-work golden.** `ordinary_workday_001` (or a sibling) driven through the TUI
  surface: possessed `actor_tomas` selecting `Continue routine` reaches `workshop_tomas` by
  movement ancestry, then a subsequent `Continue routine` starts/*completes* a work block —
  matching the fixture contract's expected events, with no teleport.
- **Marker-not-progress guard.** The committed follow-on, not the marker, is the progress; the
  marker event still carries `behavioral_progress=false`.
- **Blocked continuation.** A routine step whose follow-on is precondition-blocked (e.g.
  belief-closed workplace, unknown route) returns a typed why-not / modeled wait, and repeated
  attempts emit the stuck diagnostics — no silent Accepted no-op.
- **Possession-parity.** Embodied and autonomous reach the same routine behavioral progress
  from equivalent actor-known state; bind/unbind preserves the next routine step (extends the
  `possession_does_not_reset_intention` family).
- **Hidden-truth firewall.** A fixture with hidden target truth proves the embodied follow-on
  resolves only from actor-known context (INV-099): a target unknown to the actor yields a
  typed knowledge blocker, not a truth-driven move.
- **Replay.** The embodied follow-on commit is replay-reconstructable and physical-checksum
  stable.

## 7. Acceptance artifact and evidence

Closeout uses the scoped review-artifact template (`docs/4-specs/0003_*`). Required evidence:
the four-gate local suite green (`cargo fmt --check`, `cargo clippy -D warnings`,
`cargo build --locked`, `cargo test`), the new embodied go-to-work golden, the parity entry
passing, and the named exact implementation commit. No certification gate is claimed.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths (project convention).
- `tracewake-core` stays dependency-free; the shared resolver lives in core; the TUI consumes
  it through the existing runtime command surface, adding no simulation authority to the TUI
  boundary.
- No new simulation fact may be born from prose or from the TUI; the follow-on is a modeled
  action through the shared pipeline.

## 9. Risks and open questions

- **R1 — Double-charge / double-commit.** The marker + follow-on in one transaction must not
  double-advance time or double-charge needs; reuse the `0047` single-charge-per-tick
  accounting. Open: whether the follow-on shares the marker's tick or the next world tick.
- **R2 — Resolver divergence.** Extraction must not fork autonomous vs embodied resolution;
  a unit test must pin identical output for a fixed actor-known state.
- **R3 — Stop-reason interaction.** Surfacing the follow-on receipt must not change
  `advance_until` stop-reason semantics from `0047`.
- **OQ1 — Intention selection under windowed assignments.** `ordinary_workday_001` assigns two
  *time-windowed* routines (`routine_tomas_go_to_work` ticks 4–8, `routine_tomas_work_block`
  ticks 8–14), but the authoritative model holds exactly one active intention per actor at a
  time (`active_intention_by_actor: BTreeMap<ActorId, IntentionId>` in
  `crates/tracewake-core/src/state.rs`), and the marker already rejects any proposal whose
  intention does not equal that single authoritative active intention
  (`crates/tracewake-core/src/actions/defs/continue_routine.rs`). Confirm the embodied
  `Continue routine` resolves its follow-on from that single authoritative active intention's
  `current_step`, and that the entry's `target_ids[0]` equals it — not from an
  assigned-but-inactive routine.

## 10. Invariants alignment

INV-035, INV-094, INV-095, INV-098, INV-099 are the load-bearing invariants; this spec moves
the routine feature from no-human-only to TUI-playable-and-no-human while preserving the truth
firewall and the marker-is-not-progress rule. It amends no invariant.

## Outcome

On acceptance, a possessed player who follows their routine through the `Continue routine`
affordance makes the same behavioral progress an autonomous actor makes — reaching work by
movement and completing a work block — with typed blockers/waits/stuck where progress is not
possible, full possession parity, an intact hidden-truth firewall, and the `continue_routine`
marker still self-reporting as non-progress.
