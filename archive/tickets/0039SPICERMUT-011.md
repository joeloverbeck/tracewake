# 0039SPICERMUT-011: Kill `epistemics/knowledge_context.rs` SPINE survivors with a fact-family + noninterference harness

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `epistemics/knowledge_context.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 67 missed mutants in `crates/tracewake-core/src/epistemics/knowledge_context.rs` (spec §5.9) — the largest single cluster — owning SPINE-03 (holder-known projection and non-truth-writer boundary), with SPINE-06 (proposal) and SPINE-07 (embodied/debug) consequences. It covers context mode/source/status stable IDs, scope and provenance canonical keys, source keys, the fields of `ActorKnownCurrentPlaceFact` / `ActorKnownCarriedItemFact` / `ActorKnownWorkplaceFact` / `ActorKnownFoodSourceFact` / `ActorKnownSleepAffordanceFact`, route/door/container/item/local-actor facts, forbidden-truth audit status, and location canonicalization. A hard-coded getter equality alone is not certifying.

## Assumption Reassessment (2026-06-18)

1. `ActorKnownCurrentPlaceFact` (`:160`), `ActorKnownCarriedItemFact` (`:202`), `ActorKnownWorkplaceFact` (`:252`), `ActorKnownFoodSourceFact` (`:326`), `ActorKnownSleepAffordanceFact` (`:374`), `ForbiddenTruthAudit` with `passed()` (`:659`/`:672`), and `KnowledgeContext` (`:699`, exposing `forbidden_truth_audit`) all exist in `crates/tracewake-core/src/epistemics/knowledge_context.rs` (verified by grep). The 67 seed-mutant identities (stable-id/source-key/canonical-key/fact-field/audit) are enumerated in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.9 is the implementation contract; `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` and `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` govern sealed holder-known context and the truth firewall (verified present).
3. Shared boundary under audit: the actor-known context-construction seam where event/projection ancestry seals a `KnowledgeContext` (context ID, hash, frontier, provenance entries, typed facts, forbidden-truth audit).
4. Motivating invariants: `INV-002 — Belief comes before truth`, `INV-024 — No telepathy`, `INV-099 — Truth may validate actions, but truth may not plan them`, `INV-100 — Hidden-truth cognition is forbidden`, `INV-101 — Actor-known context is sealed`, `INV-102 — Cognition inputs require provenance`.
5. This ticket touches the actor-knowledge-firewall enforcement surface: every stable-ID/source-key/canonical-key mutant must show a downstream semantic consequence (context hash/fingerprint, serialization/report identity, projection lookup, or replay comparison); fact-field mutants use two cases differing only in the affected field with a downstream projection/view/proposal difference; `ForbiddenTruthAudit::passed` must fail closed on a typed-but-unproven forbidden-source context even when banned words are absent; and paired worlds identical in actor-known evidence but different in hidden truth must yield identical actor proposals and embodied views until a modeled observation/record event changes the context (a noninterference firewall witness), with debug able to see the difference without feeding it back. The witnesses only strengthen the firewall — no leakage is introduced — and this is a noninterference-style SPINE-03 witness, not a claim that the future EPI substrate is certified. No schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-03/06/07 re-proof in ticket 021.

## Architecture Check

1. A parameterized fact-family harness that constructs each actor-known fact through real event/projection ancestry, seals a `KnowledgeContext`, and observes a downstream consequence (context hash/fingerprint, projection/view/proposal difference, replay comparison) is the only structure that kills 67 stable-ID/key/field mutants member-by-member without getter tautologies.
2. No backwards-compatibility aliasing/shims: facts are constructed through real event/projection ancestry; no test-only context constructor bypasses event/provenance sealing.

## Verification Layers

1. INV-101/102 sealed-context identity -> manual review + replay/golden-fixture check: each stable-ID/source-key/canonical-key mutant changes context hash/fingerprint, serialization/report identity, projection lookup, or replay comparison.
2. INV-099/100 hidden-truth noninterference -> hidden-truth gate (paired worlds): worlds identical in actor-known evidence but differing in hidden truth yield identical actor proposals and embodied views until a causal information event changes the context; debug reveals the difference only in a quarantined channel.
3. INV-024 forbidden-truth fail-closed -> hidden-truth gate: `ForbiddenTruthAudit::passed` fails closed on a typed-but-unproven forbidden-source context even when banned words are absent; absent facts keep the affordance unavailable, present facts make it available only through source-backed context.

## What to Change

### 1. Parameterized fact-family harness

In `hidden_truth_gates.rs` (with seam coverage in `spine_conformance.rs`), construct each actor-known fact through real event/projection ancestry, seal a `KnowledgeContext`, and record holder-known context ID, hash, frontier, provenance entries, and the typed fact; for every stable-ID/source-key/canonical-key mutant prove a downstream semantic consequence; for every fact-field mutant use two cases differing only in the affected field with a downstream projection/view/proposal difference; exercise boolean fields (known/open/carried/available) for both truth values plus a stale/contradictory case where applicable.

### 2. Forbidden-truth + noninterference witnesses

Add an allowed-source vs typed-but-unproven forbidden-source pair proving `ForbiddenTruthAudit::passed` fails closed without banned words; build paired worlds identical in actor-known evidence but differing in hidden truth proving actor proposals and embodied views stay identical until a modeled observation/record event changes the context; prove debug comparison sees the hidden difference without feeding it back into actor-known context, proposals, records, or scheduling.

### 3. Member matrix

Map all 67 historical mutants (plus any new run survivor in this file) to a parameter row and an observed failed consequence; grouping source-key/fact-field mutants is permitted only with the member-by-member kill matrix.

## Files to Touch

- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/spine_conformance.rs` (modify)
- `crates/tracewake-core/src/epistemics/knowledge_context.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Projection-boundary filters in `projections.rs` (ticket 012) and view-model presentation (ticket 016).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).
- Any claim that the future EPI-CERT substrate is certified — this ticket tests only the SPINE holder-known/projection/firewall role of these facts.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — passes with the fact-family harness, forbidden-truth fail-closed pair, and paired-world noninterference witnesses.
2. `cargo test --locked -p tracewake-core --test spine_conformance` — passes with the sealed-context identity/provenance seam assertions.
3. `cargo mutants --workspace -f crates/tracewake-core/src/epistemics/knowledge_context.rs --no-shuffle` — all 67 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. Every stable-ID/key/fact-field mutant is caught by an observed downstream consequence; no hard-coded getter equality suffices.
2. Hidden-truth-only changes never alter actor proposals or embodied views until a modeled information event changes the context; debug may reveal the difference only in a quarantined channel.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — parameterized fact-family harness + forbidden-truth fail-closed pair + paired-world noninterference witnesses.
2. `crates/tracewake-core/tests/spine_conformance.rs` — sealed-context ID/hash/frontier/provenance seam assertions per fact kind.

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates --test spine_conformance`
2. `cargo mutants --workspace -f crates/tracewake-core/src/epistemics/knowledge_context.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

- Added a sealed holder-known fact-family fingerprint harness in `crates/tracewake-core/tests/hidden_truth_gates.rs`. It builds a `KnowledgeContext` through the public sealed context API with current-place, carried-item, workplace, food-source, sleep-affordance, route, door, container, item, and local-actor facts, then pins both the sealed context hash and a report-style fingerprint over public fact accessors.
- Covered one-sided boolean fact mutants by including paired true/false rows for workplace access, door blocking, container open/locked, and item portability.
- Added test-only unit coverage in `crates/tracewake-core/src/epistemics/knowledge_context.rs` for a failed `ForbiddenTruthAudit::passed()` state and food-source fact accessor/canonical identity. No production behavior change was needed.
- Because ticket 001 converted `.cargo/mutants.toml` into the standing SPINE perimeter, the per-file acceptance run used `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/epistemics/knowledge_context.rs --no-shuffle` to preserve this ticket's exact target. Final result: 169 mutants tested, 110 caught, 59 unviable, 0 missed.
- Verification passed:
  - `cargo test --locked -p tracewake-core --test hidden_truth_gates --test spine_conformance`
  - `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/epistemics/knowledge_context.rs --no-shuffle`
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo build --workspace --all-targets --locked`
  - `cargo test --workspace --locked`
