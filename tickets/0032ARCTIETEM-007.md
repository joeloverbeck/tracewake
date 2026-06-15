# 0032ARCTIETEM-007: A12 LOD/regional temporal summaries + time-acceleration declarations

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` (LOD temporal-summary contract + time-acceleration declaration). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-001 (LOD temporal ancestry is preserved over the A02 authoritative event/replay-time substrate). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-T9 (report T9). Foundation `10` says LOD/regional summaries must preserve temporal ancestry and information ancestry; `INV-110` and `INV-112` jointly forbid LOD promotion from filling in truth or temporal facts without modeled source. A12 already says LOD is replay-visible ontology, summary events carry inputs/outputs/fidelity limits, promotion/demotion preserve ancestry, and human focus is not privilege — but it lacks the **temporal-authority specialization**: declared source interval/cadence, temporal-vs-information ancestry separation, time-acceleration as a declared mode, and source-backed promotion of temporal claims.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A12 owns LOD-as-replay-visible-ontology, summary events with inputs/outputs/fidelity limits, ancestry-preserving promotion/demotion, and human-focus-is-not-privilege — but no temporal-summary specialization (source interval/cadence, temporal-vs-information ancestry, time-acceleration declaration). This ticket adds the specialization; it does not reinvent the LOD ancestry contract.
2. Verified against spec 0032 §3.1 D-T9 and source report §5 Finding T9 / foundation `10` / `INV-110` / `INV-112`. D-T9's home is A12. Additive; relaxes nothing. LOD equivalence thresholds, promotion algorithms, cadence values, and performance budgets route out per spec §6.
3. Shared boundary under audit: A12 (LOD/regional summaries) ↔ A02 event/replay-time substrate (ticket 001, this ticket's `Deps`). **A12 is also touched by sibling ticket 012** (D-R5 budget/fairness time-acceleration fidelity); this ticket lands first (012 `Deps` on it). Coordinate the A12 merge.
4. Constitutional invariants motivating this ticket, restated: `INV-110` — LOD/summary processes must preserve the firewall (summary events retain enough causal/epistemic ancestry that later promoted simulation is not contaminated by hidden truth); `INV-112` — LOD promotion may use temporal facts only through modeled channels. D-T9 states the temporal-summary obligations that satisfy both.
5. No-leak / promotion surface (governed here; enforcement deferred to execution `12`/`10`): each summary compressing time declares source interval, cadence, affected processes, temporal resolution/fidelity limits, and whether it includes scheduled consequences / absence of events / delayed records / stale claims; it preserves **information ancestry separately from event-time ancestry**; time acceleration is a declared simulation mode/projection policy with replay/debug visibility (not a silent optimization); promotion creates holder-known temporal claims only through modeled summary events/records that are valid channels for the promoted holder. This is `INV-110`/`INV-112` applied to LOD time; it adds doctrine only, no hidden-truth fill-in path.

## Architecture Check

1. A12 is the correct home: it already owns LOD ancestry and summary events, so the temporal-summary contract is a specialization of a contract A12 carries. A separate home would split temporal ancestry from the LOD ancestry it extends.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine; time-acceleration is named as a declared mode, not a performance switch.

## Verification Layers

1. `INV-110`/`INV-112` LOD temporal summaries (D-T9) → invariants alignment check: A12 gains the declared-interval/cadence/fidelity contract and the temporal-vs-information ancestry separation.
2. `INV-112` source-backed promotion → invariants alignment check: A12 states promotion creates holder-known temporal claims only through modeled summary events/records valid for the promoted holder, and time acceleration is a declared replay/debug-visible mode.
3. Single doc, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `12`/`10`); the layers above map each engaged invariant to a distinct alignment proof.

## What to Change

### 1. D-T9 — LOD temporal-summary contract in A12

Add to `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`:

- each regional/LOD summary that compresses time declares source interval, cadence, affected processes, temporal resolution/fidelity limits, and whether it includes scheduled consequences, absence of events, delayed records, or stale claims;
- the summary preserves information ancestry separately from event-time ancestry (what the aggregate could know vs. what it merely summarized as truth for replay vs. what later promoted holders may know);
- time acceleration is a declared simulation mode/projection policy with replay/debug visibility, not a silent performance optimization;
- promotion may create holder-known temporal claims only through modeled summary events or records that are valid information channels for the promoted holder.

Do not choose LOD equivalence thresholds, promotion algorithms, regional cadence values, or performance budgets.

## Files to Touch

- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` (modify)

## Out of Scope

- **LOD equivalence thresholds, promotion algorithms, cadence values, performance budgets** — execution/implementation (spec §6).
- **Budget/fairness time-acceleration fidelity seam (A12)** — sibling ticket 012 (D-R5); this ticket states the temporal-summary obligations, 012 the fairness constraints.
- **LOD temporal-ancestry proof procedures** — execution `12`/`10` (spec §6, V4).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T9 landing grep** — A12 carries the LOD temporal-summary contract: `grep -niE "source interval|cadence|temporal ancestry|information ancestry|time acceleration" docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` resolves it.
2. **Ancestry-separation review** — the contract preserves information ancestry separately from event-time ancestry and requires source-backed promotion of temporal claims.
3. **Invariants alignment review** — upholds `INV-110` (firewall preserved across summaries) and `INV-112` (no hidden temporal fill-in on promotion); no threshold/algorithm token introduced.

### Invariants

1. LOD/regional time-compressing summaries declare interval/cadence/fidelity and preserve temporal + information ancestry separately (`INV-110`).
2. Time acceleration is a declared replay/debug-visible mode; promotion creates holder-known temporal claims only via modeled channels valid for the promoted holder (`INV-112`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing grep) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "source interval|cadence|temporal ancestry|information ancestry|time acceleration" docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` — confirms D-T9 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the invariants-alignment review.`
