# 0063CORACTKNO-002: Actor-known observed-activity projection and perception-gated sealing

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — extends `ActorKnownProjectionRecord::LocalActor` and `ActorKnownLocalActorFact` with observed-activity + source + freshness; threads activity through perception capture in `tracewake-core`
**Deps**: 0063CORACTKNO-001

## Problem

Co-present activity may enter presentation **only** as a modeled actor-known observation with
provenance and freshness — never derived from `actor_id`, registry, routine, scheduler, or
physical truth (Spec 0063 §4.1, §4.5; INV-100). Today the actor-known local-actor channel is
identity-only: the `"visible_actor"` observation arm in
`crates/tracewake-core/src/epistemics/projection.rs` builds
`ActorKnownProjectionRecord::LocalActor { actor_id, observed_actor_id, place_id, source,
source_event_id, source_tick }` (no activity), and `current_place_knowledge_context`
(`crates/tracewake-core/src/agent/perception.rs:517`) seals it into
`ActorKnownLocalActorFact { actor_id, source_key }` (no activity). This ticket adds the observed
public activity to the projection record and seals it — **only when the possessed actor's modeled
perception captured it** — into the holder-known fact with source kind and freshness/provenance
(§1.3, §4.2, §4.3). This is the firewall-critical work: it is the one place hidden truth could
leak, so the perception gate and provenance are the whole point.

## Assumption Reassessment (2026-07-01)

1. The non-test producer of `LocalActor` records is the `Some("visible_actor") =>` arm in
   `crates/tracewake-core/src/epistemics/projection.rs` (~line 1749), which maps a typed perception
   `observation` (`observation_payload_value(observation, "target_id")`,
   `observation.observed_tick()`, `ActorKnownProjectionSource::VisibleActor`) into the record. Adding
   fields to the `LocalActor` struct variant (defined at `projection.rs:140`) forces updating that
   producer arm, every accessor `match` arm over the variant (`projection.rs` ~1006, 1022, 1060,
   1077, 1087, 1102, 1125, 1152, 1181, 1349), and the five `#[cfg(test)]` constructors (2734, 2988,
   3034, 3087, 3565). The `{ .. }` sites (`no_human_surface.rs:382`, `perception.rs:517/564`,
   `projection.rs:1006`) are additive-safe. `ActorKnownLocalActorFact`
   (`crates/tracewake-core/src/epistemics/knowledge_context.rs:597`) is `{ actor_id, source_key }`
   with a `new(actor_id, source_key)` constructor and `actor_id()/source_key()` accessors.
2. Spec 0063 §1.3/§4.1/§4.2/§4.3 require the record to carry the observed activity kind, an
   actor-safe summary, the source kind, source summary, observed tick, staleness, and uncertainty,
   and require inclusion to be gated by modeled perception with mandatory stale/uncertain wording
   when not freshly observed. §9.1 #3: identity-uncertain "someone nearby" rows are **out of scope**
   here — start with identified visible actors only.
3. **Shared boundary under audit:** the actor-known information seam
   `perception observation → ActorKnownProjectionRecord::LocalActor → ActorKnownLocalActorFact
   (sealed in KnowledgeContext)`. The activity must ride the perception observation (a typed field
   on the observation, or a payload value alongside `target_id`) so it is captured only when
   perception fired; the exact carrier (typed observation field vs. payload value) is an
   **implementer-recorded choice** in `crates/tracewake-core/src/epistemics/observation.rs` /
   `perception.rs`, recorded in this ticket's What-to-Change on landing. The consuming boundary is
   ticket 003 (`build_embodied_view_model` transfer); this ticket produces and seals, it does not
   present.
4. **Motivating invariants.** INV-100 (hidden-truth cognition forbidden) and INV-024 (no telepathy):
   activity enters actor-known context only through the modeled perception channel. INV-101
   (actor-known context is sealed): activity lives in the holder-known fact, never validator truth.
   INV-102 (cognition inputs require provenance): each sealed activity carries source + acquisition
   provenance. INV-112 (holder-known time must plan): freshness/staleness derives from holder-known
   observed-tick vs. context frontier, not free truth time.
5. **Enforcement surface — actor-knowledge filtering + deterministic replay.** The enforcement
   surfaces are the `"visible_actor"` producer arm (activity stamped only from a perception
   observation, never from truth reads) and `current_place_knowledge_context` at `perception.rs:517`
   (activity sealed into the fact only under `included_by_policy`, exactly as identity is today).
   The change must not add any truth-state read outside the observation, and must not weaken the
   existing perception/policy gate. Determinism: the record and fact already sort/dedup via derived
   `Ord`/`Hash`; new fields must keep those derives and the canonical ordering so replay stays
   byte-identical (the record feeds projection checksums — INV-018).
6. **Schema extension (additive).** Extends two serialized/ordered structures: the
   `ActorKnownProjectionRecord::LocalActor` record and `ActorKnownLocalActorFact`. Consumers: the
   accessor arms and canonical-key/ordering paths in `projection.rs`; `knowledge_context.rs`
   (`ActorKnownLocalActorFact::new`, `canonical_key`, `actor_known_local_actors()`); `perception.rs`
   capture; the provenance path (`provenance_for_projection_record`). Extension is **additive**: new
   fields default to the "no activity observed" shape (`ActivityNotApparent` / `None`), so a record
   or fact carrying no activity behaves exactly as today. No back-compat shim (§8).

## Architecture Check

1. Carrying activity as typed fields on the existing `LocalActor` record + `ActorKnownLocalActorFact`
   — rather than a parallel side-channel or a TUI-side lookup — keeps the single actor-known seam
   authoritative and reuses the existing provenance/freshness plumbing
   (`provenance_for_projection_record`, `source_tick`, the policy gate). Threading activity through
   the same perception→record→fact path that identity already travels means the firewall that
   protects identity automatically protects activity; a new path would need its own firewall proof.
2. No backwards-compatibility aliasing/shims: fields are added additively to existing types; the
   "no activity" case is represented by a closed `ActivityNotApparent` kind / `None` optional, not by
   a legacy fallback or dual code path. Reuses ticket 001's closed enums rather than introducing
   stringly-typed activity.

## Verification Layers

1. INV-100 / INV-024 (no hidden-truth cognition; no telepathy) -> codebase grep-proof + manual
   review: the `"visible_actor"` producer arm reads activity only from the perception `observation`,
   with no new truth-state handle; a negative test (ticket 004 fixture, asserted in 005) proves an
   activity present in truth but unperceived seals no activity.
2. INV-101 / INV-102 (sealed context; provenance) -> codebase grep-proof: activity is a field on
   `ActorKnownLocalActorFact` inside `KnowledgeContext`, and each carries a source kind
   (`ActorKnownActivitySourceKind`) + observed tick + provenance row via
   `provenance_for_projection_record`.
3. INV-112 / INV-018 (holder-known freshness; deterministic replay) -> replay/golden-fixture check:
   freshness is computed from observed-tick vs. context frontier; the record/fact keep derived
   `Ord`/`Hash` and canonical ordering so `cargo test -p tracewake-core` projection/checksum tests
   stay green (byte-identical projection).

## What to Change

### 1. Extend the `LocalActor` projection record

In `crates/tracewake-core/src/epistemics/projection.rs`, add to the
`ActorKnownProjectionRecord::LocalActor` variant (line 140): `observed_activity:
ObservedActorActivityKind`, an actor-safe `activity_summary: Option<String>`, and
`activity_source: ActorKnownActivitySourceKind` (from ticket 001). Update the `"visible_actor"`
producer arm (~line 1749) to populate them from the perception observation; update every accessor
`match` arm and the five test constructors.

Landing decision: observed activity rides the existing modeled perception observation as typed
payload fields (`observed_activity_kind`, `activity_summary`, `activity_source_kind`,
`activity_source_summary`). Missing activity payload maps to `ActivityNotApparent`, so existing
visible-actor perception stays actor-known and checksum-stable without inventing a TUI or truth-state
side channel.

### 2. Emit observed activity from perception (gated)

In `crates/tracewake-core/src/epistemics/observation.rs` / `crates/tracewake-core/src/agent/perception.rs`,
carry the observed public activity on the `"visible_actor"` observation **only when perception
captured it**; when perception cannot resolve an activity, carry `ActivityNotApparent`. No physical
actor-state handle is introduced (§4.2, §4.5).

### 3. Seal activity + freshness into the holder-known fact

In `crates/tracewake-core/src/epistemics/knowledge_context.rs`, extend `ActorKnownLocalActorFact`
with `observed_activity`, `activity_summary`, `activity_source`, the `observed_tick`, and a
freshness/uncertainty representation; update `new`, `canonical_key`, and accessors. In
`current_place_knowledge_context` (`perception.rs:517`), seal these into the fact under the existing
`included_by_policy` gate, attaching the provenance row. Stale observations (no fresh perception)
must seal with mandatory stale/uncertain wording derived from holder-known freshness (§4.3).

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/epistemics/knowledge_context.rs` (modify)
- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/src/epistemics/observation.rs` (modify — if activity rides a typed
  observation field rather than a payload value; confirm at implementation per AR item 3)

## Out of Scope

- `VisibleActor` fields and the `build_embodied_view_model` transfer (ticket 003).
- Content fixtures (ticket 004) and TUI render/anti-leak tests (ticket 005).
- Identity-uncertain "someone nearby" rows without a known `ActorId` (deferred, §9.1 #3).
- Collapsing hidden intentions into activity, or exposing exact hidden routine/target/job/culprit
  (§1.2 non-goals).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --lib epistemics::projection` and `... epistemics::knowledge_context`
   — the record and fact carry the three new activity members with the source kind and freshness,
   and existing accessor/canonical-key/ordering tests stay green.
2. `cargo test -p tracewake-core --lib agent::perception` — a perception test proving activity is
   sealed into the fact when the `"visible_actor"` observation carries it, and `ActivityNotApparent`
   (no activity row) is sealed when it does not.
3. `cargo test -p tracewake-core` — full crate suite, including projection checksum/canonical
   tests, passes (proves the additive fields did not perturb deterministic projection).

### Invariants

1. Activity is populated on the record only from the perception observation and sealed into the
   fact only under the existing perception/policy gate — no new truth-state read (INV-100, INV-024).
2. Every sealed activity carries a closed `ActorKnownActivitySourceKind`, an observed tick, and a
   provenance row; stale activity carries mandatory stale/uncertain wording (INV-102, INV-112).
3. The record and fact retain derived total ordering and hashing; projection checksums are
   unchanged for records carrying no activity (INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/projection.rs` (`#[cfg(test)]`) — record carries activity;
   producer arm stamps it from an observation; accessors return it.
2. `crates/tracewake-core/src/epistemics/knowledge_context.rs` (`#[cfg(test)]`) — fact seals
   activity + source + freshness; canonical key stays stable for the no-activity case.
3. `crates/tracewake-core/src/agent/perception.rs` (`#[cfg(test)]`) — perceived-activity seals a
   typed activity; truth-present-but-unperceived seals `ActivityNotApparent` (firewall witness).

### Commands

1. `cargo test -p tracewake-core --lib epistemics::projection epistemics::knowledge_context agent::perception`
2. `cargo test -p tracewake-core`
3. The crate-scoped suite is the correct boundary: this ticket is core-only; the cross-crate
   view-model/render effects are proven by tickets 003/005 and the capstone (006).

## Outcome

Completed: 2026-07-01

Extended the actor-known visible-actor projection path with typed observed-activity data while
preserving the existing perception gate. `ActorKnownProjectionRecord::LocalActor` now parses activity
payload fields from the modeled `"visible_actor"` observation, `ActorKnownLocalActorFact` carries the
sealed activity/source/tick/staleness fields, and `current_place_knowledge_context` seals those fields
only under the existing `included_by_policy` branch. The no-activity case remains
`ActivityNotApparent` and keeps the prior no-activity canonical keys/checksum strings stable.

Verification:

- `cargo test -p tracewake-core --lib epistemics::projection` — passed.
- `cargo test -p tracewake-core --lib epistemics::knowledge_context` — passed.
- `cargo test -p tracewake-core --lib agent::perception` — passed.
- `cargo test -p tracewake-core` — passed on the final formatted tree.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build -p tracewake-core --all-targets --locked` — passed.

Deviation recorded: because current `ActorBody` has no authoritative activity field, this ticket
does not infer activity from physical/routine/scheduler truth. It establishes the perception-payload
carrier and defaults unannotated visible-actor observations to `ActivityNotApparent`; fixture-level
activity production remains for the dependent fixture/render tickets.
