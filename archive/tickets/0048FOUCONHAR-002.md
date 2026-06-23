# 0048FOUCONHAR-002: Sealed holder-known interval-delta projection (additive, unwired)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds a sealed holder-known interval-delta projection to the core epistemic boundary, closed typed notice/stop-reason values, and private constructors, plus compile-fail and observational-equivalence guards. Additive only: the existing `String`-based interval types stay live until ticket 005 performs the flip. No new event kinds, content, or fixtures.
**Deps**: None

## Problem

Spec 0048 §4.5: the actor-known interval summary is built by raw-log redaction through forgeable public fields, not as a sealed holder-known delta. At `cb3102e`, `ActorKnownIntervalSource` (`crates/tracewake-core/src/projections.rs:711`) has public `actor_id`, `source_event_id`, and arbitrary `summary: String`; `build_actor_known_interval_summary` (`projections.rs:717`) accepts any caller-supplied stop-reason (`impl Into<String>`) and any iterable of sources, filters only on `source.actor_id == viewer_actor_id`, moves the arbitrary text into the notice, stringifies the event id, and labels an empty list as "no new actor-known information" — it consumes no `KnowledgeContext`, projection frontier, allowed source set, or provenance witness. Architecture `03` forbids exactly this: actor-known interval summaries are positively constructed holder-known frontier deltas, not a raw world-event diff with forbidden rows redacted. A raw event plus a matching actor id is not a sealed holder-known delta, and a `String` is not provenance (`INV-067`/`INV-099`/`INV-101`/`INV-102`/`INV-112`).

This ticket builds the **new** sealed construction additively in the core epistemic/projection boundary, leaving the existing forgeable path live and unwired; ticket 005 (§8 closure step 4) performs the atomic flip — deletes the TUI raw-log builder and the old types, and wires `advance_until` to this projection. Building additively here keeps the tree compiling and lets the new path land with its full guard suite before the cutover.

## Assumption Reassessment (2026-06-23)

1. `ActorKnownIntervalSource` is a public struct with a public `summary: String` (`crates/tracewake-core/src/projections.rs:711-715`); `build_actor_known_interval_summary` (`projections.rs:717-740`) takes `stop_reason: impl Into<String>` and filters only on `actor_id`, with no `KnowledgeContext` parameter. `ActorKnownIntervalSummary`/`ActorKnownIntervalNotice` carry public display strings (`crates/tracewake-core/src/view_models.rs:46-58`). The sealed actor-known context machinery already exists: `KnowledgeContext` and `EpistemicProjection` live in `crates/tracewake-core/src/epistemics/` (`knowledge_context.rs`, `projection.rs:272` `EpistemicProjection`), so the new projection has a sealed-context home to build against.
2. Spec 0048 §4.5 and §8 (closure step 4) own this; §4.5 names the homes `crates/tracewake-core/src/epistemics/projection.rs`, `projections.rs`, `view_models.rs`, and (for the flip) `TuiApp::advance_until`. Architecture `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` and architecture `10` (actor-known summary vs debug report are separate products) are the controlling contracts.
3. Cross-artifact boundary under audit: the holder-known interval-delta contract between the sealed `KnowledgeContext` / `EpistemicProjection` frontier (producer of allowed sources) and the interval-summary view type (consumer). The new builder must consume a sealed before/after holder-known frontier (or a typed delta from the epistemic projection) and never a caller-assembled source list; the viewer identity is bound by the sealed context, not a caller field.
4. Constitutional invariants motivating this ticket: `INV-067` (embodied mode shows actor-known reality), `INV-099` (truth may validate but may not plan — hidden truth must not select embodied output), `INV-101` (actor-known context is sealed), `INV-102` (cognition inputs require provenance), `INV-112` (the same rule applies to temporal facts). Each emitted notice must derive from a closed fact/notice kind and retain a typed source reference whose holder visibility and fact-kind compatibility were verified.
5. Enforcement surface (actor-knowledge firewall / no-leak): this projection IS the positive side of the truth firewall for embodied interval summaries. Confirm no path lets a non-holder-known event become a notice: the projection consumes only the sealed holder-known frontier/delta, "no new actor-known information" means the *verified* delta is empty (not a filtered-empty raw diff), and there is no conversion from a debug step report into the embodied summary type. Introduces no leakage path the wiring ticket (005) would have to undo, and no nondeterminism (the projection is a pure function of the sealed before/after frontier).
6. Schema extension (view models / projections): adds new typed interval types — a typed delta, a closed `IntervalNotice` value set, a closed `IntervalStopReason` value set, and a typed summary view carrying them — to `projections.rs`/`view_models.rs`. **Additive-only**: the existing `ActorKnownIntervalSource`/`ActorKnownIntervalNotice`/`ActorKnownIntervalSummary` and `build_actor_known_interval_summary` are left untouched and live (their sole consumer, `TuiApp`, is unchanged this ticket), so nothing breaks; ticket 005 deletes them at the flip. String rendering of the new types happens only at the TUI boundary (ticket 005), so the new view types carry closed values, not display strings.

## Architecture Check

1. A sealed positively-constructed delta — consuming a before/after holder-known frontier and emitting closed notice/stop values with verified provenance — makes the forbidden states unrepresentable rather than filtered-out-after-the-fact: a caller cannot hand in arbitrary prose or an other-holder source because the constructors are private to the sealed projection module and the viewer is bound by the context. This is structurally stronger than the current "filter a caller-supplied list on actor_id" path, which is permissive by shape even when its output is intuitively correct for own sleep/work terminals.
2. No backwards-compatibility aliasing/shims: the new types are not aliases of the old; the old path is deleted wholesale at ticket 005's flip, not wrapped. Building additively first is a staging step toward an atomic cutover, not a retained parallel API.

## Verification Layers

1. `INV-101`/`INV-102` sealed context + provenance -> schema validation + unit (provenance closure): every emitted notice resolves through the sealed context/projection — source existence, holder visibility, fact-kind compatibility, interval-frontier membership — and dangling, wrong-kind, other-holder, and debug-only sources fail closed.
2. `INV-067`/`INV-099` no-leak -> replay/golden-fixture check (paired-world observational equivalence): two worlds with identical possessed-actor holder-known inputs and different hidden events produce identical typed interval deltas; adding one modeled source-bearing observation to only one holder-known context changes exactly the permitted notice.
3. `INV-101` unrepresentable inputs -> codebase grep-proof (compile-fail boundary): external crates / the TUI cannot construct source/notice internals, supply arbitrary summary prose, or convert a debug step report into the embodied interval type (`crates/tracewake-core/tests/negative_fixture_runner.rs`).
4. Single-layer note N/A — three distinct invariants map to three distinct proof surfaces above.

## What to Change

### 1. Sealed interval-delta projection (core epistemic boundary)

In `crates/tracewake-core/src/epistemics/projection.rs`, add a projection that consumes a sealed before/after holder-known frontier (or a typed delta the epistemic projection produces) and emits a typed interval delta. Each delta item derives from a closed fact/notice kind and retains a typed source reference whose holder visibility and fact-kind compatibility are verified against the sealed context. "No new actor-known information" is the verified-empty delta, distinct from a raw diff that became empty after filtering.

### 2. Closed typed notice / stop-reason values and private constructors

In `crates/tracewake-core/src/projections.rs`, add the typed delta, a closed `IntervalNotice` enum (notice meaning), and a closed `IntervalStopReason` enum (replacing arbitrary prose), with constructors private to the sealed projection module so no external caller fabricates them. In `crates/tracewake-core/src/view_models.rs`, add a typed interval-summary view carrying the closed values (not display strings); rendering to strings is deferred to the TUI boundary in ticket 005.

### 3. Guard suite

Extend `crates/tracewake-core/tests/negative_fixture_runner.rs` with compile-fail cases (Verification Layer 3). Add `crates/tracewake-core/tests/holder_known_interval_projection.rs` with the paired-world observational-equivalence test (Layer 2) and the provenance-closure test (Layer 1).

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify — sealed before/after holder-known interval-delta projection)
- `crates/tracewake-core/src/projections.rs` (modify — typed delta + closed `IntervalNotice`/`IntervalStopReason` + private constructors; existing `ActorKnownInterval*` left live)
- `crates/tracewake-core/src/view_models.rs` (modify — new typed interval-summary view carrying closed values)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — compile-fail boundary; shared with tickets 004/005 — append distinct cases)
- `crates/tracewake-core/tests/holder_known_interval_projection.rs` (new — paired-world equivalence + provenance closure)

## Out of Scope

- Wiring `TuiApp::advance_until` to the new projection, deleting the TUI raw-log `actor_known_interval_sources` builder, and deleting the old `ActorKnownInterval*` types + `build_actor_known_interval_summary` — all ticket 005 (the atomic flip).
- The salient-stop consumption of the typed delta — ticket 005 (§4.4).
- The in-step perception phase that produces the holder-known delta at runtime — ticket 003 (this ticket builds the projection the step will call; it does not modify the scheduler step).
- Any debug step-report type change (architecture `10` keeps it a separate product; this ticket only ensures no conversion into the embodied type exists).

## Acceptance Criteria

### Tests That Must Pass

1. Paired-world test: identical possessed-actor holder-known inputs with different hidden events yield byte-identical typed interval deltas; adding one modeled holder-known observation to one world changes exactly the permitted notice.
2. Provenance-closure test: a dangling, wrong-fact-kind, other-holder, or debug-only source fails closed (no notice emitted, or a typed error), and every emitted notice resolves to a verified holder-known source.
3. `cargo test -p tracewake-core` passes; the new types compile alongside the retained `ActorKnownInterval*` types with no consumer change.

### Invariants

1. Interval-source/notice constructors are private to the sealed projection module; no external crate or the TUI can fabricate them (`INV-101`).
2. The typed interval delta is a pure function of the sealed before/after holder-known frontier — no hidden-world event and no caller-supplied prose can enter it (`INV-067`/`INV-099`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/holder_known_interval_projection.rs` (new) — paired-world observational equivalence + provenance closure.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify) — compile-fail boundary cases for the new constructors and the no-debug-report-conversion rule.

### Commands

1. `cargo test -p tracewake-core --test holder_known_interval_projection`
2. `cargo test -p tracewake-core --test negative_fixture_runner`
3. `cargo test -p tracewake-core` (confirms additive types compile with the retained path untouched).

## Outcome

Completed: 2026-06-23

Implemented the additive sealed holder-known interval-delta path while leaving
the legacy `ActorKnownInterval*` string summary path live for ticket 005. The
new API adds closed `IntervalNoticeKind` and `IntervalStopReason` values,
`VerifiedActorKnownIntervalNotice` with private fields and a crate-only
constructor, `ActorKnownIntervalDelta` with a crate-only constructor, and
`TypedActorKnownIntervalSummary` carrying the closed values without rendering
prose. `KnowledgeContext` gained a sealed constructor that accepts additional
provenance entries while still routing through the existing seal/hash path.

`EpistemicProjection::actor_known_interval_delta` now consumes sealed before
and after `KnowledgeContext`s, verifies same holder/bound actor, embodied mode,
non-regressing frontier, allowed provenance source class, supported provenance
kind, and source resolution against actor-known projection records for the
sealed viewer. Hidden projection records for other actors do not affect an
unchanged holder-known delta.

Added `holder_known_interval_projection.rs` for the positive verified-source
delta, hidden-world observational equivalence, unresolved/wrong-kind
fail-closed cases, and other-holder rejection. Added external negative fixtures
proving downstream crates cannot call the verified notice constructor and cannot
convert a debug event-log view into the typed embodied interval summary.

Deviation from the original ticket plan: the additive projection derives its
delta from sealed `KnowledgeContext` provenance entries and verifies each new
entry against projection records by source event ID. It does not yet wire the TUI
or delete the old raw-log path; those remain ticket 005 scope as planned.

Verification:

- `cargo test -p tracewake-core --test holder_known_interval_projection` passed.
- `cargo test -p tracewake-core --test negative_fixture_runner` passed.
- `cargo test -p tracewake-core` passed.
- `cargo fmt --all --check` passed.
- `git diff --check` passed.
