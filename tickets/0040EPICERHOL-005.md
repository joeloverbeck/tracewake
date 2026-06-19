# 0040EPICERHOL-005: EPI-04 — expectation contradiction, mismatch linkage, and absence-without-culprit discipline

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2.
**Deps**: 0040EPICERHOL-001

## Problem

Spec 0040 audit point EPI-04 (§5) requires certifying that a contradiction is a typed relation between a prior holder belief/expectation and a later admissible observation; that at this baseline the implemented active case is an expected item absent from a searched container; and that detection preserves holder/belief/observation/proposition/source/tick/schema links and may derive the expected item is missing from the expected location but **may not infer a culprit, hidden destination, or unobserved cause**. This ticket collects the EPI-04 witnesses and writes the EPI-04 section of the artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-04 seam files verified present at `ba9fe1c` (2026-06-19): `epistemics/contradiction.rs` (`ContradictionKind::ExpectedItemAbsentFromContainer`, `detect_expected_absences`), `epistemics/belief.rs`, `epistemics/observation.rs`, `epistemics/proposition.rs`, `epistemics/projection.rs`, `actions/pipeline.rs`, `events/apply.rs`. Negative fixture `external_crate_cannot_mutate_contradiction_links` and positive fixture `expectation_contradiction_001` verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-04 section only. Section wording follows spec §5 EPI-04 and the `0003` evidence fields.
3. Shared boundary under audit: the contradiction-derivation relation — `detect_expected_absences` in `contradiction.rs` linking a prior belief and a new absence observation, reconstructible under replay. The single implemented `ExpectedItemAbsentFromContainer` kind is treated as a **bounded staged abstraction**; the artifact must say so without claiming general belief revision (spec §5 EPI-04 evidence mechanics, §11.broad-social-epistemics).
4. `INV-024` (no telepathy) and `INV-016` (absence becomes evidence only through a channel) motivate this audit point, with `INV-021`…`INV-031` fallible-belief/provenance/contradiction requirements. Restated: absence must not identify a thief, mover, current hidden location, or motive.
5. This ticket audits the event-application / projection-replay determinism surface and the no-leak firewall as an **evidence consumer**: it proves that changing hidden facts (culprit/destination) while preserving the search result leaves the actor-visible contradiction equal (no leakage), and that live and replay rebuilds derive the same contradiction ID/kind/linked-IDs/propositions/tick/checksum (determinism), with no duplication on re-run. Removing/corrupting a linked source record produces a loud provenance/replay failure, not a free-standing contradiction. Any failed seam is recorded `fail` and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated EPI-04 evidence ticket writing only its artifact section is the accepted certification-audit shape; it adds the contradiction-linkage and no-culprit-inference witnesses without re-running the whole vocabulary. EPI-04 is `Small`: one active contradiction kind, one primary fixture.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2).

## Verification Layers

1. `INV-021`/contradiction linkage -> replay/golden-fixture check: `expectation_contradiction_001` creates an expected-true belief, performs the modeled search, appends the absence observation, derives one contradiction linking prior belief + new observation, surfaces the mismatch in the notebook; live and replay derive the same contradiction ID/kind/linked IDs/propositions/tick/checksum (`golden_fixtures_run`, `event_schema_replay_gates`).
2. `INV-024` no culprit inference -> replay/golden-fixture check: changing hidden culprit/destination while preserving the search result leaves the actor-visible contradiction equal; no contradiction for another holder/container/item, non-expectation stance, non-absence observation, out-of-window observation, or missing provenance (`hidden_truth_gates`).
3. provenance integrity -> negative fixture + manual review: `external_crate_cannot_mutate_contradiction_links` stays negative; removing/corrupting either linked source record yields a loud provenance/replay failure, not a free-standing contradiction (`negative_fixture_runner`).
4. Single-layer note: re-running the same accepted prefix must not duplicate the contradiction or derived missing-location belief — covered by the replay/golden layer above; no additional surface applies.

## What to Change

### 1. Collect EPI-04 witnesses

Run the EPI-04 commands and package: the prior belief, search/action event, absence observation, contradiction, derived belief (if any), source-event ancestry, live/replay equality, and an explicit "not inferred" hidden-cause matrix (culprit, mover, hidden location, motive — each shown unchanged across hidden-fact perturbation). Record the bounded-staged-abstraction declaration for the single contradiction kind.

### 2. Write the EPI-04 section of the acceptance artifact

Populate the EPI-04 section with the §9.2 ledger fields per witness (positive, adversarial, replay, compile-fail control), each row carrying exactly one §9.2 status.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-04 section only)

## Out of Scope

- Other audit points and the §6.1 compile-fail corpus matrix (owned by their own tickets).
- The §9.4 verdict table and aggregate verdict (owned by `-015`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8). Generalizing beyond `ExpectedItemAbsentFromContainer` is not in scope; the single kind is declared as a bounded staged abstraction.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test golden_fixtures_run` — `expectation_contradiction_001` linkage captured.
2. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — live/replay contradiction equality + no duplication on re-run.
3. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — no culprit/location inference under hidden-fact perturbation.
4. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — contradiction-link-mutation fixture remains negative.

### Invariants

1. Exactly one contradiction links the prior belief and the absence observation; live and replay derive identical contradiction ID/kind/linked IDs/checksum; re-run does not duplicate (`INV-021`/`INV-018`).
2. Absence derives the missing-location fact only; it identifies no culprit/destination/motive, and hidden-fact perturbation leaves the actor-visible contradiction equal (`INV-016`/`INV-024`).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-04 existing fixture/gates and records the belief→search→absence→contradiction linkage and the "not inferred" hidden-cause matrix as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
2. `cargo test --locked -p tracewake-core --test event_schema_replay_gates && cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-core --test negative_fixture_runner` (contradiction-link mutation boundary)
