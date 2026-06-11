# 0020PHA3ACOGSUR-001: Cognition-surface supersession parity, remembered-food reclassification, and deterministic tie-break

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`epistemics/projection`, `agent/no_human_surface`, `agent/perception`); new freshness/parity gates; `docs/3-reference/01_DESIGN_RISK_REGISTER.md` Watch entries; embodied-context/golden repricing in `tracewake-content` as surfaced
**Deps**: `specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md` (ORD-HARD-053, ORD-HARD-057, ORD-HARD-058)

## Problem

The 0019 workplace-freshness correction landed on one of two epistemic surfaces. The
embodied/validation path (`perception.rs::current_place_knowledge_context`) supersedes
workplace notices newest-wins; the planner-facing no-human surface
(`no_human_surface.rs::NoHumanActorKnownSurfaceBuilder`) performs no supersession at
all, so two contradictory `workplace_believed_accessible` facts (stale `false`, fresh
`true`) both reach HTN condition resolution (`ORD-HARD-053`, high). The same builder
hard-deletes remembered food-source knowledge when the actor is elsewhere (the
`FoodSource` arm early-returns on place mismatch) while `SleepPlace` records survive
from anywhere — staleness must reclassify knowledge as Remembered, never delete it
(`ORD-HARD-057`). And the embodied supersession tie-break at equal ticks
(`previous.acquired_tick() <= fact.acquired_tick()`) lets derived `BTreeSet` field
order decide the winner — provenance-arbitrary (`ORD-HARD-058`). One ticket because
all three share the classifier/builder surface and the spec batches their
embodied-context/golden churn once (spec §8).

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `96bc387`:
   `projection.rs::classified_actor_known_records_for_context` classifies freshness
   per record and performs no supersession; the store is
   `BTreeMap<ActorId, BTreeSet<ActorKnownProjectionRecord>>`, so same-workplace
   records differing in tick or `believed_access_open` coexist;
   `no_human_surface.rs::consume_projection_record`'s `Workplace` arm calls
   `add_role_assignment_notice` once per record; its `FoodSource` arm early-returns
   when `place_id != current_place_id` while the `SleepPlace` arm has no place gate;
   `perception.rs::current_place_knowledge_context` supersedes via
   `newest_workplace_by_id` with the `<=` tie-break. `ActorKnownProjectionRecord` has
   exactly four variants: `Route`, `FoodSource`, `SleepPlace`, `Workplace`.
2. Verified against spec 0020 (reassessed 2026-06-11, all findings operator-verified):
   findings ORD-HARD-053/057/058; §8 orders this trio first with one batched churn;
   the §6 risk-register entries land with this ticket per the approved
   distribute-docs decomposition decision.
3. Cross-artifact boundary under audit: the projection-records → actor-known-facts
   contract spanning `epistemics/projection.rs` (classifier),
   `agent/no_human_surface.rs` (autonomous planner surface), and
   `agent/perception.rs` (embodied/validation surface) — one supersession/freshness
   rule per record kind, applied identically on both consuming surfaces.
4. INV-028 restated: staleness is not automatically corrected — records remain stale
   until updated through modeled channels; the architecture-doc-03 freshness rule is
   that staleness *reclassifies* knowledge (Remembered), it does not delete it.
   INV-026 restated: important beliefs carry provenance — a tie-break decided by
   incidental field order is not provenance. INV-108 restated: possession is
   cognition-neutral — the embodied and autonomous views of the same projection must
   not diverge in epistemic semantics.
5. Actor-knowledge filtering and deterministic-replay surfaces touched: supersession
   changes which facts enter actor-known contexts, so embodied-context hashes and
   golden expectations may reprice (one batched repricing, per-actor ledger diffs
   explained per spec §9). Leakage direction is strictly narrowing — collapsing to
   the newest actor-known record adds no hidden-truth path; the hidden-truth audit
   surface (`decision.rs`) is untouched. The `source_event_id` tie-break is
   replay-deterministic by construction.
6. Schema shape: no schema extension — `ActorKnownProjectionRecord` and
   `ActorKnownFact` shapes are unchanged (no shape change; the change is which
   records mint facts and with what freshness class). Additive-vs-breaking is N/A.

## Architecture Check

1. Supersession is hoisted into the shared classifier
   (`classified_actor_known_records_for_context`, or a helper both surfaces consume)
   rather than duplicated into the builder — the spec's preferred direction, because
   a single classifier makes a third un-superseded consuming surface impossible to
   add, closing the defect *family* rather than the cited instance (the
   correction-closure lesson this spec exists to encode). Per-kind gating moves from
   ad-hoc arm logic to a declared per-kind policy table asserted by a symmetry test.
2. No backwards-compatibility aliasing/shims: the `FoodSource` early-return is
   replaced, not wrapped; the old place-gate behavior is not preserved behind a flag.

## Verification Layers

1. INV-028 (workplace supersession on the planner surface) -> builder-level test:
   closed@T notice + open@T+n notice for one workplace yields exactly one
   `workplace_believed_accessible` fact carrying the newer value and the newer
   notice's source event id.
2. INV-108 (two-surfaces-one-classifier parity) -> parity test asserting the no-human
   surface's workplace fact set equals the embodied
   `current_place_knowledge_context` workplace fact set for the same actor, tick,
   and projection.
3. INV-028 (remembered food) -> builder test: a food record observed at place A
   survives as `remembered_belief` after the actor moves to place B and is
   planner-reachable for `FindFood` candidate generation.
4. INV-026 / INV-018 (deterministic tie-break) -> same-tick contradictory-notice test
   asserting the higher-`source_event_id` notice wins on both surfaces.
5. Per-kind policy closure (all 4 record kinds: Route, FoodSource, SleepPlace,
   Workplace) -> symmetry assertion checking each kind's gating/supersession behavior
   against the declared policy table — a kind with undeclared policy fails.
6. Replay safety -> `cargo test --workspace` green including replay/golden gates,
   with the batched repricing's per-actor ledger diffs recorded for the acceptance
   artifact (spec §7.1–7.2).

## What to Change

### 1. Shared supersession in the classifier

Add newest-`source_tick`-per-`workplace_id` collapse (tie-break: higher
`source_event_id`) inside `classified_actor_known_records_for_context` (or a shared
helper it and `current_place_knowledge_context` both consume), so both the embodied
path and `NoHumanActorKnownSurfaceBuilder::consume_projection_records` iterate
already-superseded records. Remove the now-redundant per-surface supersession in
`perception.rs`, replacing its `<=` tie-break with the shared `source_event_id` rule.

### 2. FoodSource reclassification and the per-kind policy table

Replace the `FoodSource` arm's place-mismatch early-return with freshness
reclassification (current place → observed-now; elsewhere → remembered belief),
mirroring the workplace decoupling, so remembered food is planner-reachable. Declare
a per-kind gating/supersession policy table covering all four
`ActorKnownProjectionRecord` kinds and resolve the sleep/food asymmetry explicitly in
it. Pre-step: re-check `docs/2-execution/` for a remembered-food phase-deferral cite
(the 0020 audit found none); if one is found, downgrade to a recorded deferral in the
conformance index instead — the asymmetry must still be resolved in the policy table.

### 3. Structural locks

The Verification-Layers tests 1–5 are the locks: builder supersession, two-surface
parity, remembered-food reachability, same-tick tie-break, and the per-kind symmetry
assertion.

### 4. Risk-register Watch entries (spec §6)

Add two Watch entries to `docs/3-reference/01_DESIGN_RISK_REGISTER.md`:
acceptance-evidence reachability overstatement (the 0016/0018→0019 relapse, with its
guardrails and escalation trigger) and incomplete correction-closure (a fix landing
on one surface of a multi-surface defect class — 0019's `046`→`053`/`057`,
`047`→`054`, `045`→`055`).

### 5. Batched repricing

Reprice embodied-context hashes and golden expectations once for this trio; record
per-actor ledger diffs, every diff explained.

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify)
- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify — parity/freshness gates land here or in the touched modules' test blocks, as surfaced)
- `crates/tracewake-content/src/fixtures/` golden expectations (modify — implementation-discovered set; parent directory verified; only checksum/context expectations reprice)

## Out of Scope

- The derived apply-arm census and per-arm version dispositions (ORD-HARD-054 → `0020PHA3ACOGSUR-002`).
- Mutation perimeter, baseline governance, generative tier, helper census, TUI dead
  fields, scheduler boundary semantics (sibling tickets `-003`…`-007`).
- Conformance-index rows (land with their implementing tickets `-002`/`-003`/`-006`
  per the approved distribution); only the risk-register entries land here.
- Any change to the hidden-truth audit, debug quarantine, or possession-binding
  machinery.

## Acceptance Criteria

### Tests That Must Pass

1. Builder supersession test: stale+fresh contradictory workplace notices through
   `NoHumanActorKnownSurfaceBuilder` yield exactly one fact (newer value, newer
   source event id).
2. Two-surface parity test: no-human and embodied workplace fact sets identical for
   the same actor/tick/projection.
3. Remembered-food test: place-A food record surfaces as `remembered_belief` at
   place B and is reachable by `FindFood` candidate generation.
4. Same-tick tie-break test passing on both surfaces with the `source_event_id` rule.
5. Per-kind symmetry assertion over all four record kinds against the declared
   policy table.
6. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Staleness reclassifies knowledge (Remembered); no projection-record kind's
   knowledge is silently deleted by a place gate (INV-028; architecture doc 03).
2. The embodied and autonomous surfaces consume one supersession/freshness rule; a
   divergence between them is a failing test, not a latent defect (INV-108 spirit).
3. Supersession only narrows to newest actor-known records — no hidden-truth path is
   introduced (INV-024/099 unchanged).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/no_human_surface.rs` (test block) — builder
   supersession + remembered-food + tie-break tests at the planner surface.
2. `crates/tracewake-core/tests/hidden_truth_gates.rs` (or module test, as surfaced)
   — two-surface parity gate and the per-kind policy symmetry assertion.
3. `crates/tracewake-core/src/agent/perception.rs` (test block) — same-tick tie-break
   on the embodied surface; existing notice@4/perception@9 test stays green.

### Commands

1. `cargo test -p tracewake-core no_human_surface`
2. `cargo test -p tracewake-core --test hidden_truth_gates`
3. `cargo test --workspace` (full pipeline; golden repricing verified with explained ledger diffs)

## Outcome

Completed: 2026-06-11

What changed:

- Hoisted workplace supersession into `EpistemicProjection::classified_actor_known_records_for_context`, with newest `source_tick` winning and higher `source_event_id` breaking equal-tick ties.
- Removed the no-human `FoodSource` place-mismatch deletion so food observations from another place survive as remembered actor-known inputs.
- Simplified `current_place_knowledge_context` to consume the shared superseded classifier output instead of maintaining its own workplace newest-wins map.
- Added a projection policy table and structural tests covering all four `ActorKnownProjectionRecord` kinds.
- Added no-human builder tests for stale/fresh workplace supersession, same-tick tie-break, remembered food, and `FindFood` input reachability.
- Added embodied/no-human workplace parity coverage for the same-tick tie-break.
- Added planner target selection preference for currently observed food over remembered set ordering, after the remembered-food change exposed a no-human golden where a stale remote empty source was chosen ahead of local visible food.
- Added risk-register Watch entries for acceptance-evidence reachability overstatement and incomplete correction closure.

Deviations from original plan:

- No golden fixture files or checksum baselines needed repricing. The full workspace gate initially exposed a behavioral regression in autonomous food target choice; the implementation was corrected by preferring current observed actor-known food before falling back to remembered/set order.
- The requested `hidden_truth_gates` parity lock landed as module-level projection/perception/no-human tests rather than in `crates/tracewake-core/tests/hidden_truth_gates.rs`, because those modules already own the private helper access needed for precise assertions.

Verification results:

- `cargo test -p tracewake-core no_human_surface` — passed.
- `cargo test -p tracewake-core projection` — passed.
- `cargo test -p tracewake-core perception` — passed.
- `cargo test -p tracewake-core food_goal_prefers_currently_observed_source_over_remembered_set_order` — passed.
- `cargo test -p tracewake-content --test golden_fixtures_run no_human_day_real_run_replays_metrics_and_trace_projection` — passed after the planner target-selection fix.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
