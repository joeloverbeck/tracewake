# 0039SPICERMUT-017: Kill `debug_reports.rs` SPINE survivors with debug-quarantine + noninterference witnesses

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `debug_reports.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 19 missed mutants in `crates/tracewake-core/src/debug_reports.rs` (spec §5.8), owning SPINE-03 (projection diagnostics) and SPINE-07 (debug quarantine). The cluster covers debug-only markers, actor/item filters, event-type classification, empty stuck views, and decision-trace row rendering. Exact display prose may be snapshot support, but semantic row fields, the non-diegetic marker, and channel separation are the certifying observations.

## Assumption Reassessment (2026-06-18)

1. `debug_reports.rs` exposes `debug_only()` on multiple report types (`:101`, `:107`, `:113`, `:119`, `:125`, `:131`, `:137`) (verified by grep). The 19 seed-mutant identities (markers, actor/item filters, event-type classifiers, empty stuck views, decision-trace rows) are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.8 is the implementation contract; `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` and `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` govern debug quarantine (verified present).
3. Shared boundary under audit: the debug-report seam producing quarantined diagnostic facts that must never enter the embodied channel or feed simulation.
4. Motivating invariants: `INV-068 — Debug mode is visibly non-diegetic`, `INV-107 — Debug omniscience is quarantined`, `INV-031 — Human/debug notes are non-diegetic`.
5. This ticket touches the debug-quarantine / no-leak enforcement surface: paired embodied and debug channels must keep report facts in the debug channel (the embodied channel omits them unless independently actor-known); every debug report type's `debug_only` marker must be consumed by the channel/router or rendering quarantine (not a getter); actor/item filters must be exercised with included/excluded/nonexistent IDs; event-type classifiers must run through a real report row; nonempty stuck/decision traces must survive while empty/`xyzzy` return mutants change the debug artifact; generating/viewing a debug report must not alter event log, state checksum, holder-known context, proposals, or scheduling; and debug capability must not be forgeable from content/TUI/external crates. The witnesses only strengthen quarantine — no leakage is introduced — and there is no schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-03/07 re-proof in ticket 021.

## Architecture Check

1. Paired embodied/debug channels, parameterized report types, filter included/excluded/nonexistent cases, and a generate-does-not-mutate assertion catch the marker/filter/classifier/empty-view mutants through quarantine + noninterference — not a getter.
2. No backwards-compatibility aliasing/shims: the `debug_only` marker is observed as consumed by the channel/router or rendering quarantine; debug-capability-forge negatives use the existing negative fixtures.

## Verification Layers

1. INV-068/107 quarantine -> hidden-truth gate: for one authoritative state, paired embodied/debug channels — the debug channel contains the expected report facts; the embodied channel omits them unless independently actor-known; each report type's `debug_only` marker is consumed by the channel/router or rendering quarantine.
2. Filter/classifier correctness -> hidden-truth gate: actor and item filters with included/excluded/nonexistent IDs catch deleted predicates and boolean flips; every affected event-type classifier runs through a real report row; nonempty stuck/decision traces survive while empty/`xyzzy` mutants change the artifact.
3. Noninterference + capability -> manual review / negative fixture: generating or viewing a debug report does not alter event log, state checksum, holder-known context, proposals, or scheduling; debug capability cannot be forged from content/TUI/external crates.

## What to Change

### 1. Paired-channel + per-report-type quarantine matrix

In `hidden_truth_gates.rs` (with seam coverage in `spine_conformance.rs`), for one authoritative state build paired embodied and debug channels (debug contains the report facts; embodied omits them unless independently actor-known); parameterize every debug report type and prove its `debug_only` marker is consumed by the channel/router or rendering quarantine.

### 2. Filter/classifier + noninterference + capability rows

Exercise actor and item filters with included/excluded/nonexistent IDs; exercise every affected event-type classifier through a real report row derived from an event log; exercise nonempty stuck/decision traces (empty/`xyzzy` return mutants must change the debug artifact); prove generating/viewing a debug report does not alter event log, checksum, holder-known context, proposals, or scheduling; prove debug capability cannot be forged from content/TUI/external crates through existing negative fixtures.

### 3. Member matrix

Map all 19 historical mutants (plus any new run survivor in this file) to a concrete quarantine, filter, classifier, or noninterference consequence.

## Files to Touch

- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/spine_conformance.rs` (modify)
- `crates/tracewake-core/src/debug_reports.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- View-model presentation (ticket 016) and TUI rendering (ticket 018).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — passes with paired-channel quarantine, per-report-type marker consumption, filter, classifier, and noninterference rows.
2. `cargo test --locked -p tracewake-core --test spine_conformance` — passes with the debug-capability-boundary seam assertions.
3. `cargo mutants --workspace -f crates/tracewake-core/src/debug_reports.rs --no-shuffle` — all 19 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. Semantic row fields, the non-diegetic marker, and channel separation are the certifying observations; exact prose is snapshot support only.
2. Generating/viewing a debug report never alters log/checksum/context/proposals/scheduling; debug capability cannot be forged.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — paired embodied/debug quarantine + per-report-type marker consumption + actor/item filter + event-type classifier + nonempty-trace + generate-does-not-mutate rows.
2. `crates/tracewake-core/tests/spine_conformance.rs` — debug-capability-cannot-be-forged seam assertion via existing negative fixtures.

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates --test spine_conformance`
2. `cargo mutants --workspace -f crates/tracewake-core/src/debug_reports.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.
