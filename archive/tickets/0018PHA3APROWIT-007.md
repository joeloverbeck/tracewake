# 0018PHA3APROWIT-007: Embodied workplace fact freshness and event witness

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`epistemics/knowledge_context`, `agent/perception`, `projections`); one new content fixture; TUI test extension
**Deps**: `archive/tickets/0018PHA3APROWIT-001.md` (witness conventions established there; spec §8 ordering); `archive/specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (ORD-HARD-042)

## Problem

The embodied workplace fact escapes the epistemic hygiene the rest of the embodied surface has. `perception.rs::current_place_knowledge_context` exempts `ActorKnownProjectionRecord::Workplace` from the `is_latest_current_place_record` staleness guard that filters route/food/sleep records, so a stale role notice is always re-surfaced as current to the embodied menu (architecture doc 03: one freshness rule). And `knowledge_context.rs::ActorKnownWorkplaceFact` stores only `workplace_id/place_id/believed_access_open/source_key` — the witness is a free `String` with no `SourceEventIds` and no acquisition tick, so the embodied why-not ("You know that workplace access is closed.") reviews to a context id rather than the witnessing notice event (INV-026, INV-102). This is the epistemic residue of the otherwise-substantive ORD-HARD-029 fix.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `a9c62e0`: the `&& !matches!(… Workplace …)` exemption clause in `perception.rs::current_place_knowledge_context`; `ActorKnownWorkplaceFact`'s four fields with `source_key: String` only; `projections.rs::phase3a_semantic_actions` cites `ActionAvailabilityProvenanceKind::HolderKnownContext` (the context id) for the disabled workplace entry; the shared freshness classifier (`projection.rs::classified_actor_known_records_for_context` + `record_freshness`) already exists and classifies workplace records as `Remembered` for notice-derived sources — the exemption bypasses it, it doesn't predate it.
2. Spec 0018 ORD-HARD-042 (required correction + structural lock); spec §3 verified-holding records ORD-HARD-029's believed-access seam as substantive — this ticket adds witness/freshness honesty on top, not a redesign.
3. Cross-artifact boundary under audit: the embodied-affordance formula (`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — availability from believed conditions; one freshness semantics for all holder-known records).
4. INV-026/INV-102 restated: important beliefs record source, acquisition time, and staleness; provenance must be sufficient for review — an availability decision whose provenance reviews to a context id rather than a witnessing event is not reviewable.
5. Actor-knowledge filtering surface touched: the embodied menu remains belief-derived (no truth read is introduced; `guard_014_embodied_projection_workplaces_are_context_backed` and the workspace-wide truth-accessor ban must stay green). The fact-shape change feeds `canonical_hash_inputs`, so holder-known context hashes may shift where workplace facts appear — golden expectations reprice as surfaced; replay re-derives deterministically. Stale-notice routing reduces no knowledge (a superseded notice surfaces as remembered/superseded, not deleted — INV-028).
6. Schema extension: `ActorKnownWorkplaceFact` gains `SourceEventIds` + acquisition tick — breaking-internal to its constructor (all consumers in-workspace: `perception.rs` builder, `projections.rs` provenance threading, hash inputs), updated in the same diff (local compile-atomicity). The availability-entry provenance gains the witnessing event refs — additive to the view model.

## Architecture Check

1. Routing workplace records through the existing shared freshness classification (deleting the exemption clause) is cleaner than tuning the exemption: the divergence class ORD-HARD-027 closed for food/sleep/routes closes identically here, and `perception.rs` sheds its last bespoke staleness carve-out. Carrying `SourceEventIds` on the fact mirrors `ActorKnownFact` (ORD-HARD-032's lock), making the embodied and no-human fact families witness-consistent.
2. No backwards-compatibility aliasing/shims: the `source_key`-only constructor is replaced; no parallel unwitnessed constructor remains.

## Verification Layers

1. One freshness rule -> new fixture `stale_workplace_notice_superseded_by_newer_001`: an older role notice superseded by a newer one ⇒ the newer wins, the older is not re-surfaced as current; replay byte-match.
2. INV-102 reviewable provenance -> TUI test: the disabled workplace entry's provenance refs include the witnessing role-notice event id (extending `embodied_flow.rs::embodied_workplace_availability_reflects_belief_not_truth`).
3. No-leak preservation -> existing `guard_014_embodied_projection_workplaces_are_context_backed` and `cognition_inputs_are_context_backed` stay green (grep-proof: no `state.workplaces` token enters guarded layers).
4. INV-018 -> golden/context-hash gates green at any repriced expectations; `cargo test --workspace`.

## What to Change

### 1. Witnessed workplace fact

`knowledge_context.rs::ActorKnownWorkplaceFact` gains `SourceEventIds` + acquisition tick (constructor + accessors + `canonical_key`/hash inputs); `perception.rs` threads the projection record's source event ids and tick into it.

### 2. Shared freshness routing

Remove the `Workplace` exemption from `current_place_knowledge_context`'s staleness guard; workplace records flow through `classified_actor_known_records_for_context` like every other record kind (superseded notices resolve to the latest).

### 3. Provenance threading to availability

`projections.rs::phase3a_semantic_actions` includes the fact's witnessing event refs in the workplace entry's availability provenance.

### 4. Fixture + tests

New supersession fixture (registered in `fixtures/mod.rs` + census); extend the TUI divergence test with the provenance-refs assertion.

## Files to Touch

- `crates/tracewake-core/src/epistemics/knowledge_context.rs` (modify)
- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-content/src/fixtures/stale_workplace_notice_superseded_by_newer_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify — fixture census)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify)

## Out of Scope

- The no-human surface's witness table and presence facts (ticket `0018PHA3APROWIT-001`, a dependency).
- Believed-access *semantics* (which conditions disable the entry) — landed in 0017 (ORD-HARD-029); only freshness and witness change here.
- Any reduction of plannable knowledge — superseded notices remain available as remembered beliefs (INV-028).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content stale_workplace_notice_superseded` — newer notice wins; older not double-surfaced; replay byte-match.
2. `cargo test -p tracewake-tui embodied_workplace_availability_reflects_belief_not_truth` — entry present-but-disabled with provenance refs including the role-notice event id.
3. `cargo test -p tracewake-core guard_014` + `cognition_inputs_are_context_backed` — no truth-read regression.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every holder-known record kind — including workplace — flows through the single shared freshness classification; no consumer-side exemptions.
2. Every embodied availability decision sourced from a workplace fact carries witnessing event refs reviewable in debug and replay.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/stale_workplace_notice_superseded_by_newer_001.rs` — supersession fixture.
2. `crates/tracewake-tui/tests/embodied_flow.rs` — provenance-refs assertion on the divergence test.
3. `crates/tracewake-core/src/agent/perception.rs` (unit test) — workplace records subject to the staleness guard.

### Commands

1. `cargo test -p tracewake-content stale_workplace_notice_superseded`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-11

What changed:

- Removed the workplace exemption from `current_place_knowledge_context`, so workplace records now use the same latest-current-place freshness gate as route, food, and sleep records.
- Added `SourceEventIds` and acquisition tick to `ActorKnownWorkplaceFact`, including context-hash participation and accessors.
- Threaded workplace source event ids into disabled work action availability provenance through a new `SourceEvent` provenance kind.
- Added `stale_workplace_notice_superseded_by_newer_001` as a registered fixture and content regression proving a newer modeled role notice supersedes the older seeded notice.
- Extended the TUI workplace availability test to assert the disabled work entry includes the role-notice source event id.
- Added a core unit test proving stale workplace notices are not resurfaced by the current-place knowledge context.

Deviations from original plan:

- The fixture schema still authors a single seed role notice. The superseding newer notice is appended in the test through the normal event/apply path, because duplicate role notices are events, not a separate fixture-schema authoring construct.

Verification:

- `cargo fmt --all --check`
- `cargo test -p tracewake-core current_place_knowledge_context_applies_freshness_to_workplace_notices`
- `cargo test -p tracewake-content stale_workplace_notice_superseded`
- `cargo test -p tracewake-tui embodied_workplace_availability_reflects_belief_not_truth`
- `cargo test -p tracewake-core guard_014`
- `cargo test -p tracewake-core cognition_inputs_are_context_backed`
- `cargo test -p tracewake-content --test fixtures_load`
- `cargo test -p tracewake-content --test golden_fixtures_run`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
