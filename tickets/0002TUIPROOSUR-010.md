# 0002TUIPROOSUR-010: Typed, source-bound notebook leads and provenance

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`NotebookView` typed lead/provenance structures, `build_notebook_view`)
**Deps**: 0002TUIPROOSUR-001, 0002TUIPROOSUR-002

## Problem

Notebook rendering is actor-scoped, but `possible_leads` are derived by testing whether a rendered belief summary contains the phrase `"missing from expected location"` (`crates/tracewake-core/src/projections.rs:435-439`) — display text acting as a semantic classifier. Spec 0002 §4 TUI-AC-008 and §5 PSL-002 require notebook beliefs/observations/contradictions/leads to be typed actor-known entries with source references, confidence/staleness, and explicit "how this may be wrong" affordances, with leads generated from typed contradiction/proposition kinds rather than substring checks, and culprit identity absent unless actor-known.

## Assumption Reassessment (2026-06-08)

1. `build_notebook_view` is at `crates/tracewake-core/src/projections.rs:390`; `possible_leads` use `belief.summary.contains("missing from expected location")` (`:435-439`). `NotebookView` is at `crates/tracewake-core/src/view_models.rs:99`. Typed substrate already exists: `ContradictionKind` (`crates/tracewake-core/src/epistemics/contradiction.rs:14`), `Contradiction` (`:33`), and `Proposition` (`crates/tracewake-core/src/epistemics/proposition.rs:9`).
2. Spec 0002 §4 TUI-AC-008 + §5 PSL-002: typed lead/notice/provenance structures on the notebook view; leads from typed contradiction/proposition kinds + source refs, not display text; render source/confidence/staleness/possible-next-actions from typed fields.
3. Cross-artifact boundary under audit: `NotebookView` (view contract) ↔ the epistemic projection's typed `ContradictionKind`/`Proposition` ↔ the sealed context (ticket 001, which already scopes the notebook via `KnowledgeContext`).
4. Invariants restated: **INV-102** — cognition inputs (beliefs, observations, contradictions, leads) require provenance; **INV-093** — actor-knowledge leakage is high-severity; **INV-067** — embodied notebook shows actor-known reality; **INV-070**/**INV-105** — leads/diagnostics are typed, not display-string-derived.
5. Fail-closed / no-leak surface: the notebook is built from the sealed context (ticket 002); confirm culprit/ground-truth is never inferred into actor notes, and leads derive from typed kinds. A wording change to a belief summary must not alter whether a lead exists; a change to the typed proposition/contradiction kind must. No hidden-truth path enters the notebook.
6. Schema extension: `NotebookView` gains typed lead/notice/provenance structures (source ref, source kind, confidence, staleness, contradiction/mismatch relations, possible actor-known next actions). Consumers: `build_notebook_view` (`projections.rs:390`), `render_embodied_view` (`render.rs`), and `tracewake-tui` tests. Breaking — internal, all consumers updated here.
7. Rename/removal blast radius: the substring-based lead derivation is removed. Pre-implementation grep `"missing from expected location"` across `crates/`, `docs/`, `specs/`: known site is `projections.rs:435-439`; any other match joins Files to Touch. The phrase may remain as belief *display* text but must no longer be a classifier.

## Architecture Check

1. Deriving leads from typed `ContradictionKind`/`Proposition` makes lead generation robust to wording and impossible to spoof via summary text, satisfying INV-105 and removing the "display text is a classifier" smell; provenance/confidence/staleness become typed fields the renderer formats, not prose the logic parses.
2. No backwards-compatibility aliasing/shims: the substring classifier is deleted, not retained as a fallback; the typed lead structures replace it outright.

## Verification Layers

1. INV-105 (typed leads) -> behavior test: changing the wording of the missing-property belief summary does not change whether a lead exists; changing the typed contradiction/proposition kind does.
2. INV-102 (provenance) -> schema validation: each notebook entry carries source ref, source kind, confidence, staleness.
3. INV-093 (no leakage) -> manual review + negative test: culprit identity / ground truth is absent from the notebook unless actor-known via a modeled channel.

## What to Change

### 1. Typed notebook structures

Add typed lead/notice/provenance structures to `NotebookView` (belief/observation/contradiction ids, source refs + kinds, confidence, staleness, contradiction/mismatch relations, possible actor-known next actions).

### 2. Typed lead generation

Generate `possible_leads` in `build_notebook_view` from typed `ContradictionKind`/`Proposition` + source refs; remove the `summary.contains(...)` classifier.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)

## Out of Scope

- Why-not actor/debug split (ticket 007).
- Sealed-context construction (ticket 001) and embodied builder routing (ticket 002).
- New epistemic mechanics — this ticket consumes existing `ContradictionKind`/`Proposition`, it does not add claim families.

## Acceptance Criteria

### Tests That Must Pass

1. A contradiction fixture proves a source-bound lead appears from typed contradiction/proposition data; a wording-only change to the belief summary does not alter lead existence; a typed-kind change does.
2. A test proves culprit identity is absent from the notebook unless actor-known.
3. `cargo test -p tracewake-core` passes; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Leads are a function of typed epistemic kinds + provenance, not display text (INV-105/INV-102).
2. The notebook leaks no ground truth (INV-093/INV-067).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — wording-vs-typed-kind lead behavior over `expectation_contradiction_001`.
2. `crates/tracewake-core/src/projections.rs` (unit tests) — typed lead generation.

### Commands

1. `cargo test -p tracewake-core hidden_truth_gates`
2. `grep -rn "missing from expected location" crates/` (must show no classifier use after implementation — display-only at most)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
