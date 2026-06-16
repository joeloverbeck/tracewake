# 0037P0CERTMUTREM-001: Kill 0036-MUTATION-REMEDIATION-001 with actor-known local-actor provenance witness

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new core gate/regression test in `tracewake-core` (no production logic change expected; test-only Arm A disposition). If the final checkout proves the field unreachable by certified behavior, Arm B (proven-equivalent) is recorded instead per spec §6.4.
**Deps**: None

## Problem

The 0036 P0-CERT baseline audit emitted one untriaged missed mutant, `0036-MUTATION-REMEDIATION-001`: cargo-mutants replaced `crates/tracewake-core/src/projections.rs::actor_known_local_actors_for_context(context: &KnowledgeContext) -> Vec<ActorId>` with `vec![]` and no test failed. The helper feeds source-backed actor-known local actors into `EmbodiedViewModel.local_actors` — a certified actor-filtered surface — so a mutation that erases them must be caught by a behavior/provenance witness, not a non-empty-vector assertion. The named core gate tests currently carry no `local_actors`/`VisibleActor` assertion, which is exactly why the mutant survives.

(spec §3, §5, §6)

## Assumption Reassessment (2026-06-16)

1. `actor_known_local_actors_for_context` exists at `crates/tracewake-core/src/projections.rs:335`, returns `Vec<ActorId>`, sorts+dedups. `EmbodiedProjectionSource::from_sealed_context` (`projections.rs:191`) calls it; `build_embodied_view_model` (`projections.rs:539`) maps `source.actor_known_local_actors` → `VisibleActor` → `EmbodiedViewModel.local_actors` (`view_models.rs:31`). Verified by grep this session. No existing `local_actors`/`VisibleActor` assertion in `hidden_truth_gates.rs` / `acceptance_gates.rs`.
2. Spec `specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` §6.3 (Arm A obligations 1–7) and `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md` (finding row) define the disposition; the §6.3 rejected-test-shapes list forbids non-empty-vector / private-helper-sort / physical-co-location / debug-string / shim-bypass witnesses.
3. Cross-artifact boundary under audit: the sealed holder-known context → `EmbodiedProjectionSource` → `build_embodied_view_model` → `EmbodiedViewModel.local_actors` projection contract, carrying `ActorKnownLocalActorFact` (`epistemics/knowledge_context.rs:597`) provenance (`actor_id` + `source_key`, sealed into the context hash by `KnowledgeContext::seal`).
4. Invariant motivation: INV-024 (no telepathy — view models expose only actor-known facts), INV-006 (possession transfers no knowledge), INV-067 / INV-095 / INV-107 (embodied surface / debug quarantine). The witness must prove a source-backed actor-known local actor reaches embodied play while an authoritative-but-not-actor-known actor does not.
5. Enforcement surface: the `EmbodiedViewModel.local_actors` projection in `build_embodied_view_model`. The killing test must include a negative contamination witness (an actor physically present in authoritative state but absent from actor-known facts must NOT appear in `local_actors`), so the fix cannot kill the mutant by leaking truth — preserving belief-before-truth filtering. Provenance ties to the sealed context hash/frontier introduce no replay nondeterminism.

## Architecture Check

1. A behavior/provenance witness at the projection/view-model layer is cleaner than a private-helper assertion: per cargo-mutants guidance the right fix for a private-function miss is a public-behavior test that breaks when the private function is wrong. Asserting the certified `EmbodiedViewModel.local_actors` consequence kills the `vec![]` mutant AND locks the firewall, where an `assert!(!v.is_empty())` helper test would be tautological metric-gaming (spec §6.3 rejected shapes).
2. No backwards-compatibility aliasing/shims, and no test-only branch bypassing the real projection/view-model path (spec §6.3 forbids compile-time shim / alias / test-only branch).

## Verification Layers

1. INV-024 no-telepathy → core test: positive witness asserts a source-backed actor-known local actor appears as a `VisibleActor` in `EmbodiedViewModel.local_actors`; negative witness asserts an authoritative-but-not-actor-known actor is absent.
2. INV-006 possession / INV-107 debug quarantine → core test + manual review: debug availability/comparison does not create the `local_actors` entry in embodied mode.
3. Provenance/replay → replay/golden-fixture or deterministic-context check: the holder-known context ID/hash/frontier + source-key ancestry tie the projection to the sealed context and are reproducible from event/provenance ancestry.
4. Mutation kill → cargo-mutants grep-proof: re-run the targeted mutant on `projections*` and confirm `actor_known_local_actors_for_context -> vec![]` is `caught`.

## What to Change

### 1. Add the killing test

Construct (or load) a scenario where the viewer actor has a legitimate `ActorKnownLocalActorFact` for another actor with a `source_key` tied to modeled perception / observation / contact / fixture-prehistory source (not raw physical co-location). Build the sealed holder-known context, `EmbodiedProjectionSource::from_sealed_context`, and `build_embodied_view_model`; assert the source-backed actor appears in `EmbodiedViewModel.local_actors`. In the same or a paired test, keep another actor authoritative-but-not-actor-known and assert it is absent. Assert the holder-known context ID/hash/frontier (and, where possible, source-event ancestry) tie the view to the sealed context. Prove debug availability does not create the entry. (spec §6.3 properties 1–7)

Implementer-recorded choices (spec §6.3, §11.2): (i) Arm A (kill) vs Arm B (proven-equivalent) — Arm A preferred unless the final checkout proves the field unreachable by certified behavior; (ii) core-only test vs fixture-backed vs both. Record the chosen disposition and witness shape in the triage register / replacement artifact (see -002, -003).

### 2. Confirm no production change is required

The helper already returns the correct value; expected scope is test-only. If the final-checkout call-site inventory (spec §6.2; known top-level caller `crates/tracewake-core/src/controller.rs`, plus the internal `projections.rs` call sites) proves the field genuinely unreachable by certified behavior, switch to Arm B and record the proof instead — do not force a test.

## Files to Touch

- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify — primary candidate home, as surfaced)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — alternative home, as surfaced; choose one per §11.2)
- `crates/tracewake-content/src/fixtures/<new-fixture>_001.rs` (new — optional, only if a fixture-backed witness is chosen)

## Out of Scope

- The full configured mutation posture run and triage of other survivors (→ -002).
- P0-01..P0-10 re-proof and the replacement acceptance artifact (→ -003).
- Any production change to `actor_known_local_actors_for_context` or the projection path (the helper is correct today).
- Adding the survivor to `.cargo/mutants-baseline-misses.txt` (forbidden, spec §10 risks).

## Acceptance Criteria

### Tests That Must Pass

1. New core test: a source-backed actor-known local actor appears in `EmbodiedViewModel.local_actors` (positive witness).
2. New core test: an authoritative-but-not-actor-known actor is absent from `local_actors` (negative contamination witness); debug truth does not create the entry.
3. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --locked` all pass.
4. Targeted mutation rerun shows `actor_known_local_actors_for_context -> vec![]` is `caught`.

### Invariants

1. INV-024 / INV-006: no hidden-truth or possession/debug-privileged actor reaches `EmbodiedViewModel.local_actors`; only actor-known-fact-backed actors appear.
2. The witness asserts certified public behavior, not a private-helper internal (no tautological / shim kill).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` (or `acceptance_gates.rs`) — adds the positive + negative + provenance + debug-quarantine witness for `actor_known_local_actors_for_context`.

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` (or `--test acceptance_gates`, per chosen home)
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --locked`
3. Targeted kill proof: `cargo mutants --workspace -f 'crates/tracewake-core/src/projections*' --no-shuffle` (with `.cargo/mutants.toml` in effect) — confirm the `actor_known_local_actors_for_context -> vec![]` line is `caught`. The full posture is -002's scope.
