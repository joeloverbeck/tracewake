# 0014PHA3AORDLIF-005: Typed actor-known input refs + provenance-class hidden-truth audit

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`agent/decision.rs` `ActorKnownInputRef` + audit, `agent/generation.rs` producer, `agent/transaction.rs` consumer), new source guard, 1 adversarial test, `tracewake-content` test update
**Deps**: None

## Problem

`DecisionInput.actor_known_inputs` is `Vec<String>` (`crates/tracewake-core/src/agent/decision.rs:14`), and the hidden-truth audit checks whether those strings `.contains("unproven")` / `.contains("debug_omniscience")` / `.contains("physical_truth")`, emitting a note with only counts (`decision.rs:113-126`). A display string can omit the banned token while carrying forbidden-source semantics, or include it in a harmless explanation — so this is not a firewall. INV-105 requires the decision-trace substrate to be typed, not display strings. This is ORD-HARD-004.

## Assumption Reassessment (2026-06-09)

1. `actor_known_inputs: Vec<String>` is at `crates/tracewake-core/src/agent/decision.rs:14`; `hidden_truth_audit_from_actor_known_inputs(&[String])` with the substring checks is at `decision.rs:113-126`; it is called at `decision.rs:83`. The producer assigns `actor_known_inputs: generated.actor_known_inputs_used` at `decision.rs:268`, sourced from `crates/tracewake-core/src/agent/generation.rs`. `transaction.rs` constructs `DecisionInput`. A content test references `actor_known_inputs` at `crates/tracewake-content/tests/golden_fixtures_run.rs`. The provenance enum to reuse, `KnowledgeProvenanceKind`, is at `crates/tracewake-core/src/epistemics/knowledge_context.rs:88`.
2. Spec §ORD-HARD-004 and §5.2 require replacing `Vec<String>` with `Vec<ActorKnownInputRef>` carrying fact id, proposition id, provenance edge ids, source event ids, source class, confidence/staleness, and an explicit-unknown marker; the audit must derive from provenance classes / forbidden-source enums, not text.
3. Shared boundary under audit: the actor-known input contract spanning `generation.rs` (producer of `actor_known_inputs_used`), `decision.rs` (`DecisionInput` field + audit), and `transaction.rs` (consumer). All move in one diff (local compile-atomicity).
4. Invariants motivating this ticket: **INV-102** (cognition inputs require provenance; missing provenance is a rejection condition for action-relevant cognition), **INV-105** (typed diagnostic substrate, not display strings), **INV-107** (debug omniscience quarantined — a debug/unproven-origin input must be detectable by class, not text).
5. Actor-knowledge-firewall enforcement surface: the hidden-truth audit is the firewall that decides `actor_known_only`. After this change it derives `actor_known_only` from each `ActorKnownInputRef`'s source class against a forbidden-source enum, so an input with forbidden provenance fails even when its display text omits the banned word. The audit stays deterministic (enum comparison, stable ordering) and introduces no new leakage path. This feeds the typed `hidden_truth_referenced` diagnostic field (ticket -003) but does not depend on it.
6. Schema reshape — additive-vs-breaking: `actor_known_inputs` changes type `Vec<String>` → `Vec<ActorKnownInputRef>`. Consumers: `generation.rs` (producer), `decision.rs` (audit + trace), `transaction.rs` (construction), and `content/tests/golden_fixtures_run.rs`. Breaking by design; all consumers in-workspace, updated in this diff.
7. Removal blast radius: the `.contains("unproven")` / `.contains("debug_omniscience")` / `.contains("physical_truth")` substring audit and any use of `Vec<String>` as the authoritative actor-known input are removed; grep confirms `decision.rs:113-126` is the audit site. The guard bans reintroduction of substring-based provenance checks in authoritative audit paths.

## Architecture Check

1. A typed `ActorKnownInputRef` with a `KnowledgeProvenanceKind`-classed source makes the firewall key on provenance class, which is unforgeable by display wording — strictly stronger than substring matching and reuses the existing provenance enum rather than inventing a parallel one.
2. No backwards-compatibility shim: the `Vec<String>` field and the substring audit are deleted, not retained alongside the typed path.

## Verification Layers

1. INV-102 / INV-105 (typed provenance substrate) -> codebase grep-proof: `ActorKnownInputRef` carries the required fields; `actor_known_inputs: Vec<ActorKnownInputRef>` replaces the `Vec<String>`; the audit reads source class, not text.
2. INV-107 (debug omniscience detectable by class) -> replay/golden-fixture check: `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` constructs an unproven/debug-origin input whose display text omits the banned words; the audit still fails because the provenance class is forbidden.
3. INV-105 (no substring substrate) -> codebase grep-proof: source guard bans `contains("unproven")` / `contains("debug_omniscience")` / `contains("physical_truth")` and `Vec<String>`-as-authoritative-actor-known-input in `agent/decision.rs`.

## What to Change

### 1. ActorKnownInputRef

In `crates/tracewake-core/src/agent/decision.rs`, define `ActorKnownInputRef { fact_id, proposition_id, provenance_edge_ids, source_event_ids, source_class: KnowledgeProvenanceKind (or a forbidden-source-aware wrapper), confidence, staleness, explicit_unknown: bool }`; change `DecisionInput.actor_known_inputs` to `Vec<ActorKnownInputRef>`.

### 2. Provenance-class audit

Rewrite `hidden_truth_audit_from_actor_known_inputs` to derive `actor_known_only` from each ref's source class against a forbidden-source set (unproven / debug-omniscience / physical-truth classes), not text.

### 3. Producer + consumer updates

Update `generation.rs` (`actor_known_inputs_used`) to emit typed refs and `transaction.rs` / `decision.rs:268` construction accordingly; update `content/tests/golden_fixtures_run.rs`.

### 4. Source guard + negative test

Add the substring-audit-ban guard and the adversarial negative test.

## Files to Touch

- `crates/tracewake-core/src/agent/decision.rs` (modify — `ActorKnownInputRef`, typed field, provenance-class audit; **also touched by 003**)
- `crates/tracewake-core/src/agent/generation.rs` (modify — emit typed refs)
- `crates/tracewake-core/src/agent/transaction.rs` (modify — construct typed refs; **shared with 002/003/004, coordinate**)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — typed-ref usage)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — guard #4; **N-way shared hub**)
- `crates/tracewake-content/src/fixtures/hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register fixture; **shared hub**)

## Out of Scope

- The typed `hidden_truth_referenced` event field (ticket 0014PHA3AORDLIF-003 — populated from this audit's result).
- No-human metrics string-scan removal (ticket 0014PHA3AORDLIF-009).

## Acceptance Criteria

### Tests That Must Pass

1. `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` — an input with a forbidden provenance class but banned-word-free display text fails the audit.
2. `cargo test -p tracewake-core --test anti_regression_guards` — substring-audit-ban guard passes.
3. `cargo test --workspace` — the `Vec<String>` → `Vec<ActorKnownInputRef>` retype compiles across all consumers and existing planner/transaction tests pass.

### Invariants

1. The hidden-truth audit keys on provenance class, never display text (INV-102/105/107).
2. Every actor-known input carries replay/debug-sufficient provenance; missing provenance is a rejection condition for action-relevant cognition (INV-102).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs` — provenance-class rejection independent of display text.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — guard banning substring-based provenance audit and `Vec<String>` authoritative actor-known input.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-content`
3. `cargo test --workspace`

## Outcome

Completed: 2026-06-09

What changed:

- Replaced `DecisionInput.actor_known_inputs: Vec<String>` with typed `Vec<ActorKnownInputRef>`.
- Added `ActorKnownInputSourceClass` and a provenance-class hidden-truth audit that rejects forbidden source classes and explicit unknowns without scanning display text.
- Updated candidate generation to emit typed actor-known input refs from `ActorKnownFact` while rendering trace-facing proof notes from typed refs at the `DecisionTrace` boundary.
- Added read-only `ActorKnownFact` accessors needed to build typed refs without exposing mutation.
- Added a regression test proving forbidden typed provenance is rejected even when display text omits `unproven`, `debug_omniscience`, and `physical_truth`.
- Added a source guard banning the old substring audit and `Vec<String>` decision input contract.
- Added and registered `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` as the ticket fixture contract.

Deviations from original plan:

- `DecisionTrace` continues to store rendered proof-note strings for existing canonical trace compatibility; those strings are derived from typed refs and are not used as the hidden-truth audit substrate.

Verification results:

- `cargo fmt --all --check` — passed.
- `cargo test -p tracewake-core agent::decision` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards` — passed.
- `cargo test -p tracewake-content --test fixtures_load` — passed.
- `cargo test -p tracewake-content` — passed.
- `cargo test -p tracewake-core` — passed.
- `cargo test --workspace` — passed. Rustc emitted transient warnings about corrupt `tracewake-tui` incremental artifacts being ignored/deleted; all tests passed.
