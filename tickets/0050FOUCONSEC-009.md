# 0050FOUCONSEC-009: Closed typed salience policy + four-case salient-stop witness

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — replaces the over-broad salient-stop predicate with a closed typed salience policy
**Deps**: 0050FOUCONSEC-006

## Problem

Spec-0050 §4.6 (driver F-06): the salient-stop branch is production-reachable, but `actor_known_interval_delta_is_salient` (`crates/tracewake-core/src/scheduler.rs:932`) returns true when **any** notice has kind `Observation`/`Record`/`Belief`. Current-place perception emits a `visible_actor` observation for every co-located actor each tick, so re-observing an unchanged co-located actor satisfies the salient-kind predicate — `advance_until` can stop on routine observational churn. The prominent witness does not distinguish routine re-observation from novel, modeled, holder-visible salience.

This ticket replaces the over-broad predicate with a closed typed salience policy (an `IntervalSalience` classification produced during holder-known delta construction) and proves it with a four-case quiet/novel/hidden/replay witness over production `advance_until`. The salience policy is the implementer's recorded choice per §9.6.

## Assumption Reassessment (2026-06-24)

1. `actor_known_interval_delta_is_salient` is at `crates/tracewake-core/src/scheduler.rs:932`; the typed stop reason `IntervalStopReason::ActorKnownSalientObservation` is used at `scheduler.rs:787`/`894` (enum arm at `scheduler.rs:280`). The holder-known delta is built by `actor_known_interval_delta` (`crates/tracewake-core/src/epistemics/projection.rs:577`) over `ActorKnownIntervalDelta` (`crates/tracewake-core/src/projections.rs:781`). Verified at `HEAD` (`8d7c119`).
2. Spec-0050 §4.6 and §9.6 are authoritative: §9.6 assigns the salience policy (novelty vs contradiction vs confidence/stance change vs newly-available record vs local-state change vs explicit materiality class) to the implementer as a recorded choice; it must remain holder-known and typed, with no debug/raw-world salience oracle. `-006` must land first so the core-owned interval delta exists to classify.
3. Shared boundary under audit: the interval-delta salience classification in `crates/tracewake-core/src/epistemics/projection.rs` / `projections.rs` and the `advance_until` stop policy in `scheduler.rs`. Adjacent contradiction: none — this is a policy refinement on the `-006` delta, not a re-litigation of ownership.
4. `INV-067` (embodied actor-known reality), `INV-102` (cognition inputs require provenance), `INV-112` (holder-known temporal interpretation) motivate this ticket: the stop decision must be a typed, source-bearing, holder-known judgment, not an inference from broad notice category or rendered prose.
5. Enforcement surface: actor-knowledge / no-leak firewall (`INV-067`/`INV-102`) and deterministic replay (`INV-018`/`INV-112`). The salience classification is produced during holder-known delta construction from sealed actor-known context only (no debug/raw oracle); replay reconstructs the same typed stop reason and interval summary. The hidden-world case proves a non-holder-visible source does not stop the interval.
6. **Schema/contract shape change (additive-vs-breaking)**: an `IntervalSalience` classification is added to `ActorKnownIntervalDelta` (`projections.rs:781`); consumers are the `advance_until` stop policy and the interval product. The extension is **additive** (a new typed field on the delta) — existing consumers gain a typed signal; the over-broad boolean predicate is replaced, not removed from a public contract.

## Architecture Check

1. A closed typed `IntervalSalience` produced during sealed holder-known delta construction replaces a category-guess boolean with a provenance-bearing classification — it makes "stop on salient" mean "stop on a typed, holder-known, material change" and is debug/raw-oracle-free by construction. Cleaner and more precise than widening/narrowing the kind list.
2. No backwards-compatibility shims: the over-broad predicate is replaced by the typed policy, not kept as a fallback.

## Verification Layers

1. `INV-067`/`INV-112` (holder-known, typed stop) → replay/golden-fixture check: four paired cases over production `advance_until` — (a) unchanged state for N ticks reaches the controller safety bound; (b) a novel modeled holder-visible source due at tick k>1 stops exactly at k; (c) an otherwise-identical hidden/non-holder-visible source does not stop; (d) replay reconstructs the same typed stop reason + summary.
2. `INV-102` (provenance) → manual review: the salience classification resolves through the sealed context; no debug/raw-world input feeds it.

## What to Change

### 1. Closed typed salience policy

In `crates/tracewake-core/src/epistemics/projection.rs` and `crates/tracewake-core/src/projections.rs`, produce a closed typed `IntervalSalience` classification during holder-known delta construction (implementer-recorded choice per §9.6 — record the selected policy and rationale in the type/doc comment). Add it to `ActorKnownIntervalDelta`.

### 2. Stop policy consumes the typed classification

In `crates/tracewake-core/src/scheduler.rs`, replace `actor_known_interval_delta_is_salient`'s broad kind test with a consult of the typed `IntervalSalience`. Keep stop reasons closed and typed.

### 3. Four-case witness

In `crates/tracewake-core/tests/salient_stop_actor_known.rs`, add the quiet/novel/hidden/replay paired cases over production `advance_until`. The novel event must arise from the production process/actor discovery (`-001`/`-002`/`-003`), and the unchanged case must contain ≥1 routine perception opportunity so "no notices at all" cannot make it vacuously pass.

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/tests/salient_stop_actor_known.rs` (modify)

## Out of Scope

- Core ownership of the interval product — `0050FOUCONSEC-006` (prerequisite).
- Generative / adversarial salience coverage beyond the four-case witness — `0050FOUCONSEC-010`.

## Acceptance Criteria

### Tests That Must Pass

1. Quiet case: unchanged actor-known state for N ticks reaches the controller safety bound (no stop on routine re-observation).
2. Novel case: a novel modeled holder-visible source due at tick k>1 stops exactly at k; hidden case: a non-holder-visible source does not stop; replay reconstructs the same typed stop reason + summary.
3. `cargo test -p tracewake-core --test salient_stop_actor_known` and `cargo build --workspace --all-targets --locked` are green.

### Invariants

1. The stop decision is a typed, holder-known, source-bearing classification with no debug/raw-world oracle (`INV-067`/`INV-102`).
2. Routine re-observation of unchanged co-located state does not stop the interval; a novel holder-visible change does (`INV-112`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/salient_stop_actor_known.rs` — quiet/novel/hidden/replay four-case witness over production `advance_until`.

### Commands

1. `cargo test -p tracewake-core --test salient_stop_actor_known`
2. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`
