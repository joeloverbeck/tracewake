# 0044FIRPROCER-012: FIRST-PROOF-12 — temporal firewall: modeled temporal premises, ancestry, freshness, and non-contamination

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-12 certifies the **first routed temporal source** (execution `04`) at `U`: any time-sensitive actor belief, expectation, candidate, contradiction, or proposal must derive from an allowed holder-known source with temporal ancestry. Raw scheduler time, replay order, debug timestamps, queue state, and validator-only due/open facts may validate but may not become cognition. Every actor-legible temporal premise must name source category, source event/record/observation, observed/acquired time, freshness state where doctrine requires it, and uncertainty/staleness representation; a newer modeled source may supersede an older one through event/projection logic; validator time must be separately identified; and replay must preserve source ancestry and supersession. This ticket records the FIRST-PROOF-12 temporal-firewall witnesses, raw-time non-contamination negatives, and replay supersession into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/{time.rs, scheduler.rs}`, `crates/tracewake-core/src/agent/{actor_known.rs, transaction.rs, decision.rs}`, the epistemic belief/observation/knowledge files under `crates/tracewake-core/src/epistemics/`, `crates/tracewake-core/src/actions/proposal.rs`, and `crates/tracewake-core/tests/hidden_truth_gates.rs` (all confirmed present this session). Fixtures `stale_workplace_notice_superseded_by_newer_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001`, and `aged_food_record_surfaces_as_remembered_belief_not_observation_001` exist.
2. Spec §7 FIRST-PROOF-12 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; `INV-112`, doctrine foundation `04`/`14`, architecture `03`/`05`/`06`, execution `04` temporal-firewall section and execution `02` integrated acceptance bind it. No new temporal unit or threshold may be invented by the audit (spec §5.4).
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-12 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket is temporal source `04` of the five-source bundle, consolidated by `0044FIRPROCER-016`.
4. Motivating invariant (spec §7 FIRST-PROOF-12): `INV-112` (time may validate, but holder-known time must plan — deadline/lateness/staleness/expected-by-now/office-closed are holder/institution-known interpretations with provenance, not free truth labels). Restate before trusting the narrative: validator time orders and validates; it must not become cognition authority.
5. This ticket audits/reads (does not modify) the holder-known-context, proposal-construction, action-validation, projection, and replay enforcement surfaces. It runs fixtures and records witnesses only; injecting raw clock/scheduler-queue/replay-index/debug-timestamp/validator-future-fact must be rejected or excluded by actor-known construction, and restamping stale content without a modeled information event cannot refresh it. No nondeterminism and no new temporal vocabulary are introduced.

## Architecture Check

1. Recording each temporal premise's source category + source event + acquisition/observation tick + freshness + supersession decision, then varying true schedule/open/due/current time while holding actor-known sources constant, is the only check that separates holder-known temporal cognition from validator/clock time; spec §7 requires that validator time be separately identified and unable to masquerade as holder-known time.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may silently restamp stale content or let possession/debug refresh temporal knowledge.

## Verification Layers

1. `INV-112` holder-known temporal premise -> replay/golden-fixture check (every actor-legible temporal premise names source category, source event/record/observation, observed/acquired time, freshness state where required, and uncertainty/staleness; a newer modeled source supersedes an older one through event/projection logic).
2. `INV-112` validator-time separation -> manual review + codebase grep-proof (validator time is separately identified and cannot masquerade as holder-known; injected raw clock/queue/replay-index/debug-timestamp/validator-future-fact is rejected or excluded by actor-known construction).
3. `INV-112` replay ancestry + no auto-correction -> replay/golden-fixture + adversarial check (replay preserves source ancestry and supersession; restamping stale content without a modeled information event cannot refresh it; possession/debug toggles cannot refresh temporal knowledge).

## What to Change

### 1. Record positive temporal-firewall witnesses

Run `stale_workplace_notice_superseded_by_newer_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001`, `aged_food_record_surfaces_as_remembered_belief_not_observation_001`, and the §7 positive path. Record that every actor-legible temporal premise names source category, source event/record/observation, observed/acquired time, freshness state where doctrine requires it, and uncertainty/staleness representation; that a newer modeled source can supersede an older source through event/projection logic; that validator time is separately identified and cannot masquerade as holder-known time; and that replay preserves source ancestry and supersession. Record source-event chain, temporal premise type, acquisition/observation tick, supersession decision, context hash/frontier, accepted/rejected proposal, and live/replay temporal fields.

### 2. Record raw-time non-contamination negatives

Record the §7 adversarial cases: change true schedule/open/due/current time without changing actor-known sources — candidate/plan/view/contradiction remains unchanged; inject raw clock, scheduler queue, replay index, debug timestamp, or validator future fact — actor-known construction rejects or excludes it; restamping stale content without a modeled information event cannot refresh it; possession/debug toggles cannot refresh temporal knowledge. Each negative names the canonical responsible layer (spec §11 vocabulary). No new unit or threshold is invented.

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-12 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any temporal-firewall defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Inventing any temporal unit, threshold, or source category (spec §5.4); routine temporal premises (`-013`), embodied temporal rendering (`-014`), the temporal fixture families (`-015`), the consolidated five-source line (`-016`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario/temporal tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `cargo test --locked -p tracewake-core --test event_schema_replay_gates` pass; every actor-legible temporal premise names source category/event/acquisition-time/freshness and replay preserves source ancestry and supersession.
2. `cargo test --locked -p tracewake-core --test generative_lock` passes; changing true schedule/open/due/current time without changing actor-known sources leaves candidate/plan/view/contradiction unchanged.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-tui --test adversarial_gates` pass; injected raw clock/queue/replay-index/debug-timestamp/validator-future-fact is rejected and stale content is not silently restamped.

### Invariants

1. No actor cognition derived from raw/validator/debug time; no untraceable freshness; no silent restamping; no temporal replay drift.
2. Validator time is separately identified and cannot masquerade as holder-known time; no new temporal unit/threshold/source category is invented.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test generative_lock`
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
5. `cargo test --locked -p tracewake-tui --test adversarial_gates`

## Outcome

Completed: 2026-06-21

Recorded FIRST-PROOF-12 in the shared acceptance artifact as passed for its
temporal-firewall source scope. The evidence packet now includes command-ledger
rows, gate/scenario references, a FIRST-PROOF-12 audit section, temporal source
`04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`, and two
evidence ledger items: `E-0044-012-temporal-firewall` and
`E-0044-012-raw-time-negative`.

Verification run:

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` -> pass, 17 passed.
2. `cargo test --locked -p tracewake-core --test generative_lock` -> pass, 5 passed.
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` -> pass, 32 passed.
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run` -> pass, 42 passed.
5. `cargo test --locked -p tracewake-tui --test adversarial_gates` -> pass, 15 passed.
