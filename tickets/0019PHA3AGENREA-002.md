# 0019PHA3AGENREA-002: Workplace freshness — separate supersession from place-gating

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`agent/perception`, `epistemics/projection` as surfaced); embodied context-hash expectations as surfaced
**Deps**: `tickets/0019PHA3AGENREA-001.md` (ordering only — serialize any golden/context-hash churn behind the -001 repricing); `specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md` (ORD-HARD-046)

## Problem

`perception.rs::current_place_knowledge_context` applies
`if !classified.is_latest_current_place_record() { continue }` as a hard visibility
filter to **all** record kinds, including `ActorKnownProjectionRecord::Workplace`.
`epistemics/projection.rs::is_latest_current_place_record` is true only when the
record's source tick equals the latest current-place perception tick — but a
role-assignment notice is remembered routine knowledge whose source tick is the notice
tick, generally not the latest perception tick. The embodied path therefore silently
**drops** known workplaces unless the ticks coincide: staleness deletes knowledge
instead of reclassifying it as remembered (INV-028; architecture doc 03's one freshness
rule, here misapplied as a visibility gate). This is the residual of the otherwise
substantive `ORD-HARD-042` fix and one of the spec's two letter-level foundational
violations.

## Assumption Reassessment (2026-06-11)

1. Verified against current code at `main` `5af8660`: the `continue` filter in
   `perception.rs::current_place_knowledge_context` precedes the
   `ActorKnownProjectionRecord::Workplace` arm and gates it; the proving test
   `current_place_knowledge_context_applies_freshness_to_workplace_notices` emits
   perceptions at `SimTick::new(4)` and `SimTick::new(7)` — exactly the two notice
   ticks — with the context built at tick 8, so its "newest notice wins" assertion is
   satisfied by the same-tick filter dropping the older notice; it cannot distinguish
   supersession from place-gating. `ActorKnownWorkplaceFact` already carries
   `source_event_ids` + `acquired_tick` (the 0018 `ORD-HARD-042` shape), so no fact
   schema change is needed — only the visibility/classification split.
2. Verified against `specs/0019_…_HARDENING_SPEC.md` ORD-HARD-046 (reassessed
   2026-06-11): the required correction is to separate supersession (newest record per
   workplace wins) from current-place visibility, with routine-assignment-class records
   not place-gated; genuinely place-bound observation records stay behind the
   current-place gate. Spec §9: embodied context hashes may change for fixtures where
   notice and perception ticks differ — current fixtures use coincident ticks, so the
   expected golden churn is zero; verify rather than assume.
3. Cross-artifact boundary under audit: the freshness-classification contract between
   the shared classifier (`epistemics/projection.rs::classified_actor_known_records_for_context`
   / `record_freshness`) and the embodied context builder
   (`perception.rs::current_place_knowledge_context`) — one freshness rule that
   *classifies* (CurrentlyPerceived vs Remembered), never silently deletes.
4. INV-028 restated: notices, records, and knowledge remain stale until updated through
   modeled channels — staleness reclassifies (a stale notice surfaces as remembered,
   still planning-available), it does not vanish. INV-024 in spirit: an under-grant is
   still an epistemic-channel defect even though no hidden truth leaks.
5. Actor-knowledge surface touched (fail-closed direction preserved): the change
   *restores* the actor's own legitimately-acquired knowledge to the embodied context
   with honest `Remembered` freshness — it introduces no hidden-truth path (the facts
   already carry their witnessing notice event ids and acquisition ticks), and the
   debug-quarantine and no-leak gates (`debug_truth_never_enters_holder_known_context_hash`)
   must remain green. Embodied context hashes are deterministic functions of the
   corrected fact set; any expectation diffs are explained in the -009 ledger.

## Architecture Check

1. Splitting "which record is current for this workplace" (supersession, keyed per
   workplace) from "is this record place-gated" (visibility, applicable only to
   genuinely place-bound observation records) keeps the single shared classifier as the
   one freshness authority instead of adding a workplace special-case to the filter —
   the filter bug came from applying one place-bound rule to a record class that is not
   place-bound; the fix is class-aware classification, not a second rule.
2. No backwards-compatibility aliasing/shims: the same-tick gate for
   routine-assignment-class records is removed, not bypassed behind a flag.

## Verification Layers

1. INV-028 staleness-reclassifies -> new `perception.rs` test: role notice at tick T,
   latest current-place perception at tick T+5; the workplace fact still appears in
   `actor_known_workplaces()` with `acquired_tick() == T` and remembered freshness —
   fails on the current `continue`.
2. Supersession integrity -> existing
   `current_place_knowledge_context_applies_freshness_to_workplace_notices` and the
   `stale_workplace_notice_superseded_by_newer_001` fixture remain green (newest notice
   still wins on coincident ticks).
3. No-leak preservation -> `debug_truth_never_enters_holder_known_context_hash` and the
   hidden-truth gates remain green; no new fact source is introduced.
4. INV-018 deterministic replay -> golden fixture suite green; any embodied
   context-hash diffs enumerated and explained (expected: none, per coincident-tick
   fixtures).

## What to Change

### 1. Class-aware visibility in the embodied context builder

`perception.rs::current_place_knowledge_context`: stop applying the
`is_latest_current_place_record` gate to routine-assignment-class records (Workplace);
apply supersession per workplace (newest record wins) and surface non-current records
with `Remembered` freshness classification. Genuinely place-bound observation records
(routes/food/sleep current-place observations) keep the current-place gate.

### 2. Classifier support as surfaced

`epistemics/projection.rs`: if the split needs a class-aware predicate (place-bound vs
routine-assignment-class), add it on the shared classifier so the rule lives in one
place; otherwise confine the change to the builder.

### 3. The non-coincident-tick lock

New test per Verification Layer 1; keep the existing coincident-tick test as the
supersession lock.

## Files to Touch

- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify — as surfaced by where the class-aware predicate lands)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — only if context-hash expectations shift; expected none)

## Out of Scope

- Workplace fact schema (already carries `source_event_ids` + `acquired_tick` from 0018).
- The no-human surface's workplace knowledge path (`add_role_assignment_notice` —
  witness-honest since 0018; untouched).
- Checksum composition and payload versioning (ticket `-001`).
- Embodied interruption surfacing (ticket `-008`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core current_place_knowledge_context` — non-coincident-tick
   test green (fact surfaces as remembered with original acquisition tick); coincident-tick
   supersession test green.
2. `cargo test -p tracewake-core --test hidden_truth_gates` — no-leak gates green.
3. `cargo test -p tracewake-content --test fixtures_load` —
   `stale_workplace_notice_superseded_by_newer_001` green.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No record class is silently dropped from the embodied context by a tick-coincidence
   condition; staleness maps to `Remembered` classification, never to absence.
2. The freshness rule lives in the single shared classifier; the context builder applies
   class-aware policy, not a second freshness definition.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/perception.rs` (unit test) — notice tick T,
   perception tick T+5, fact surfaces as remembered (`acquired_tick == T`).
2. `crates/tracewake-core/src/agent/perception.rs` — existing coincident-tick test
   retained as the supersession lock.

### Commands

1. `cargo test -p tracewake-core current_place_knowledge_context`
2. `cargo test --workspace`
