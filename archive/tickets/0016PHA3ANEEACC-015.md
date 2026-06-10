# 0016PHA3ANEEACC-015: Unify no-human actor-known surface builder onto epistemic projection

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` no-human cognition surface, epistemic projection accessors, replay hash rebuilding, and related fixtures/tests
**Deps**: `archive/tickets/0016PHA3ANEEACC-008.md`

## Problem

Phase 3A still has a deliberate substrate fork after the 0016PHA3ANEEACC-008 minimum cut: embodied view contexts read retained observations from `EpistemicProjection`, but `NoHumanActorKnownSurfaceBuilder` still parses raw event payloads into builder-local tables for no-human decision inputs.

That fork is event-backed, but it duplicates the epistemic projection's role and keeps cognition-source doctrine split across two implementations. This ticket records the full unification work explicitly instead of letting the deferral remain silent.

## Assumption Reassessment (2026-06-10)

1. Current code after 0016PHA3ANEEACC-008 keeps `NoHumanActorKnownSurfaceBuilder::consume_events` in `crates/tracewake-core/src/agent/no_human_surface.rs` as the no-human actor-known source, while `current_place_knowledge_context` in `crates/tracewake-core/src/agent/perception.rs` reads embodied facts from `EpistemicProjection`.
2. Governing docs require holder-known cognition to come from modeled epistemic channels: `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` and `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`.
3. Shared boundary: no-human decision input construction versus the durable epistemic projection and its replay/hash rebuild path.
4. INV-024 and INV-102 remain the core constraints: information reaches actors through modeled channels, and cognition inputs must be replayable from event-backed evidence.
5. Enforcement surface: decision trace `actor_known_inputs`, `actor_known_context_hash`, hidden-truth audit, and replay rebuild must continue to agree byte-for-byte after the substrate source changes.

## Architecture Check

1. The clean end state is one durable epistemic substrate feeding both embodied views and no-human decisions. Duplicating event-payload parsing in the surface builder is easier locally but keeps two definitions of actor-known facts.
2. No backwards-compatibility shims: remove the parallel builder-local parsing once projection-backed construction covers the required facts.

## Verification Layers

1. Cognition-source doctrine -> source guard or unit test proving no-human actor-known facts are derived from `EpistemicProjection`, not direct raw-event scans.
2. Replay determinism -> decision context hash rebuild tests pass against projection-backed inputs.
3. Hidden-truth firewall -> existing hidden-truth/audit tests remain green and include projection-backed actor-known facts.

## What to Change

### 1. Projection-backed no-human surface

Replace `NoHumanActorKnownSurfaceBuilder`'s parallel raw event parsing for observations, starting beliefs, and role assignment notices with projection-backed accessors or a projection-derived actor-known surface.

### 2. Replay hash rebuild

Update replay decision-context hash rebuilding to use the same projection-backed surface as live no-human cognition.

### 3. Structural locks

Add guard coverage that prevents reintroducing direct event-payload scans as the source of no-human actor-known cognition.

## Files to Touch

- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- Phase 3A fixtures/tests as needed for repriced actor-known inputs

## Out of Scope

- New belief semantics beyond facts already represented in Phase 3A.
- TUI rendering changes; embodied staleness is covered by 0016PHA3ANEEACC-008.

## Acceptance Criteria

### Tests That Must Pass

1. A guard or unit test fails if no-human actor-known input construction scans raw event payloads instead of projection-backed records.
2. Decision context hash rebuilds match live no-human decisions after projection-backed construction.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. No-human cognition and embodied views use the durable epistemic projection as their actor-known substrate.
2. Hidden truth cannot enter actor-known decision inputs through debug state, raw world state, or unmodeled event parsing.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/no_human_surface.rs` — projection-backed actor-known construction.
2. `crates/tracewake-core/src/replay/rebuild.rs` — decision context hash rebuild parity.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — no direct payload-scan cognition source.

### Commands

1. `cargo test -p tracewake-core no_human_surface`
2. `cargo test -p tracewake-core rebuild_decision_context_hashes`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed in this changeset. `NoHumanActorKnownSurfaceBuilder` now builds from
projection-backed actor-known records instead of scanning raw event payloads, and
the scheduler and replay decision-context hash rebuild both use the same
projection-backed surface. `EpistemicProjection` retains typed actor-known
records for role notices, starting beliefs, and retained observations, with
source-event witnesses preserved in the existing decision inputs. Guard coverage
now fails if the no-human surface reintroduces direct event-payload scans for
actor-known cognition.
