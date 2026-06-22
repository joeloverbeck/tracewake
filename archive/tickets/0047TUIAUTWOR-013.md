# 0047TUIAUTWOR-013: Actor-known interval projection + debug split

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/tracewake-core` (`view_models.rs` new optional `EmbodiedViewModel` field, `projections.rs` interval-summary constructor), `crates/tracewake-tui` (`render.rs` Hop-2 disposition); core + TUI epistemic tests
**Deps**: 0047TUIAUTWOR-012

## Problem

Spec 0047 §1.7/§4.7 requires a typed actor-known interval-summary projection, constructed *positively* from the possessed actor's holder-known frontier delta, own embodied effects, modeled messages/records/observations, and fresh resumption perception — exposed through `EmbodiedViewModel` under spec-0046 Hop-2 exhaustive renderer disposition, and structurally separate from the exact core/debug step report. No constructor may accept raw world state, global event slices, hidden due queues, or the debug report. This ticket adds the projection, the `EmbodiedViewModel` field, and the render disposition, completing the §4.7 projection half (the controller half is 0047TUIAUTWOR-012).

## Assumption Reassessment (2026-06-22)

1. `EmbodiedViewModel` exists at `crates/tracewake-core/src/view_models.rs:19` with 23 fields and **no** interval/summary field today (its epistemics live in `notebook`, `last_rejection_*`, and `holder_known_context_*`). The interval projection is a new optional field on this struct. The exact core/debug step report already exists as the coordinator's typed step result (0047TUIAUTWOR-005) — the "structurally separate debug report" half of the split is in place; this ticket adds the actor-known half.
2. The advance-until controller (0047TUIAUTWOR-012) produces the step/stop results over the interval; the projection summarizes the possessed actor's holder-known delta across that interval. The holder-known context already on `EmbodiedViewModel` (`holder_known_context_*`) is the provenance substrate the projection draws from.
3. Cross-artifact boundary under audit: the architecture `03` holder-known contract (amended in 0047TUIAUTWOR-002) — interval summaries are positively-constructed source-bearing holder-known frontier deltas, never raw-world-diff redaction. The projection constructor's input type is the firewall: it accepts only holder-known/source-bearing inputs, structurally excluding raw world state, global event slices, hidden due queues, and the debug step report.
4. Constitutional invariant motivating the ticket: `INV-024` (no telepathy), `INV-067` (embodied mode shows actor-known reality), `INV-099`/`INV-101`/`INV-102` (truth firewall; sealed actor-known context; cognition inputs require provenance). The summary is built positively from source-bearing deltas, not by redacting an omniscient diff.
5. Enforcement surface (no-leak / actor-knowledge filtering): the projection constructor accepts only holder-known/source-bearing inputs. It may carry own action/duration outcome, own bodily/need change at existing embodied precision, direct perception at resume, modeled messages/notices/records/testimony/public cues with provenance, perceived sounds/interruptions, and an actor-visible stop reason. It must exclude other actors' unobserved private actions/states, hidden failure/interruption reasons, global event counts/due queues/scheduler phase data, claims that an unobserved place/service/item/person did not change, and the omniscient "nothing happened." When nothing reached the actor, the typed result is equivalent to "no new actor-known notices or observations are available," not "nothing happened" — the wording is renderer-owned, the distinction typed. The positive (source-chain present) + negative (hidden other-actor event leaves the summary unchanged) epistemic tests are the regression lock.
6. Schema extension (view-model projection), keyed separately from item 5: the new field on `EmbodiedViewModel` is **additive-only** (a new optional field defaulting to absent), replaced atomically on the next completed advance. Consumers of `EmbodiedViewModel`: the TUI renderer (`render.rs`, the Hop-2 exhaustive destructure that this addition forces to be updated) and any view-model serialization/test. Additive: existing consumers compile and behave unchanged when the field is `None`.

## Architecture Check

1. An explicit optional field on `EmbodiedViewModel` (forcing a Hop-2 exhaustive disposition in `render.rs`) — rather than a side channel — is the design spec §4.7 prefers, because the compiler enforces that the renderer dispositions the new surface (spec-0046 Hop-2), preventing a silently-unrendered or silently-leaking field. A constructor that accepts only holder-known inputs makes leakage a compile-time impossibility rather than a review check.
2. No backwards-compatibility aliasing/shims: the field is additive-optional with a default; no parallel "summary via raw diff" path is introduced, and the debug step report stays structurally separate (it is not fed into the projection constructor).

## Verification Layers

1. `INV-024`/`INV-067` no telepathy -> manual review (epistemic-leakage audit) + codebase grep-proof: the projection constructor's signature accepts only holder-known/source-bearing inputs; no raw world state / global event slice / debug report parameter.
2. `INV-099`/`INV-102` truth firewall + provenance -> replay/golden-fixture check: a hidden other-actor event leaves the embodied summary unchanged (negative test); a modeled message reaching the actor appears with provenance (positive test).
3. Hop-2 exhaustive disposition -> codebase grep-proof: `render.rs` destructures the new field exhaustively (compiler-enforced), with a distinct "no new actor-known notices/observations" rendering vs an empty/"nothing happened" wording.

## What to Change

### 1. Interval-summary projection constructor (`projections.rs`)

Add a typed actor-known interval-summary type and a constructor that accepts only holder-known/source-bearing inputs (the possessed actor's frontier delta, own embodied events, resumption perception, modeled messages/records/observations with provenance, actor-visible stop reason). The constructor must not accept raw world state, a global event slice, a hidden due queue, or the debug step report.

### 2. `EmbodiedViewModel` optional field (`view_models.rs`)

Add a new optional interval-summary field (additive; defaults to absent), replaced atomically on the next completed advance. Distinguish the typed "no new actor-known notices/observations" result from "nothing happened."

### 3. Hop-2 render disposition (`render.rs`)

Update the exhaustive `EmbodiedViewModel` destructure/render so the new field is dispositioned (rendered or explicitly no-render), with renderer-owned wording for the no-new-information case. A persisted summary item must already have modeled observation/record/memory ancestry; rendering prose creates no fact.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — file created by 0047TUIAUTWOR-005) — positive/negative epistemic tests
- `crates/tracewake-tui/tests/transcript_snapshot.rs` (modify — render golden for the new field)

## Out of Scope

- The advance-until controller / step loop (0047TUIAUTWOR-012) — this ticket consumes its results.
- The exact debug step report (already the coordinator's typed result, 0047TUIAUTWOR-005) — kept structurally separate, not modified here.
- Long-term interval persistence beyond the single "last completed advance" field (spec §9 open-question-4 leaves persistence to existing memory/record mechanisms).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core` — positive: a modeled message/observation reaching the possessed actor over the interval appears in the summary with provenance. Negative: a hidden other-actor private event over the same interval leaves the embodied summary unchanged.
2. A "no new actor-known notices/observations" case is typed distinctly from "nothing happened"; the projection constructor cannot be called with raw world state / a global event slice / the debug report (compile-time / signature proof).
3. `cargo test -p tracewake-tui` — the `render.rs` exhaustive disposition compiles and the transcript golden renders the new field (or its no-render disposition); `cargo clippy --workspace --all-targets -- -D warnings` clean.

### Invariants

1. The interval summary is constructible only from holder-known/source-bearing inputs; it excludes hidden world activity and the omniscient "nothing happened" (`INV-024`/`INV-067`/`INV-099`).
2. The `EmbodiedViewModel` field is additive-optional (existing consumers unaffected when absent) and Hop-2 exhaustively dispositioned in `render.rs`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — positive (source-chain present) + negative (hidden event leaves summary unchanged) + "no new notices ≠ nothing happened" epistemic tests.
2. `crates/tracewake-tui/tests/transcript_snapshot.rs` (modify) — render golden for the new interval-summary field.

### Commands

1. `cargo test -p tracewake-core interval`
2. `cargo test -p tracewake-core -p tracewake-tui && cargo clippy --workspace --all-targets -- -D warnings`
3. The core+TUI boundary is correct: the projection is core (with epistemic tests) and the render disposition is TUI (with a golden) — both surfaces of the deliverable.

## Outcome

Completed: 2026-06-22

Added an additive optional `actor_known_interval_summary` field to `EmbodiedViewModel`, a source-bearing `build_actor_known_interval_summary` constructor, and a TUI attachment path from completed `advance_until` results. The constructor accepts actor id, ticks, stop reason, and source records only; hidden other-actor records are filtered out before rendering. The embodied renderer now exhaustively dispositions the field, including a typed "no new actor-known notices or observations" state distinct from "nothing happened."

Verification:

- `cargo fmt --all --check`
- `cargo test -p tracewake-core actor_known_interval`
- `cargo test -p tracewake-tui renderer_prints_actor_known_interval_summary`
- `cargo test -p tracewake-core --test anti_regression_guards embodied_view_option_and_collection_fields_have_reachable_producers`
- `cargo test -p tracewake-core -p tracewake-tui`
- `cargo clippy --workspace --all-targets -- -D warnings`
