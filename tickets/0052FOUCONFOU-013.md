# 0052FOUCONFOU-013: Capstone — acceptance artifact and per-finding closure evidence

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — acceptance-only; assembles the evidence artifact and re-runs the closure gate. No new production logic.
**Deps**: 0052FOUCONFOU-010, 0052FOUCONFOU-011, 0052FOUCONFOU-012

## Problem

Spec 0052 §8; closure-order step 12. The remediation is not accepted until one session, beginning from a clean baseline at one exact implementation commit, runs the full closure gate and records per-finding closure with real production-path evidence. This capstone produces the acceptance artifact at `reports/0052_foundational_conformance_fourth_hardening_acceptance.md` (following `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, the repo convention beside `reports/0048_foundational_conformance_hardening_acceptance.md`), records per-finding closure (production constructor, public command, observed effect, sensitivity proof), records the required-check names and branch-protection confirmation, and must not call the perimeter green before the standing campaign completes with zero in-surface misses/timeouts and the food-source family resolved or honestly reported.

It introduces no new production logic; it exercises the pipeline tickets 001–012 composed and renders the verdict.

## Assumption Reassessment (2026-06-25)

1. The acceptance template `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` exists; the sibling artifact `reports/0048_foundational_conformance_hardening_acceptance.md` is the convention exemplar. No file exists yet at the target report path (no collision). The preserved focused mutation commands to reproduce are the `0049MUTWIT` / `0050FOUCONSEC` / `0051FOUCONTHI` lines.
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §8. All implementation/evidence/doc tickets (001–012) precede this capstone; the standing campaign disposition is produced by 0052FOUCONFOU-010 and the required CI/branch-protection by 0052FOUCONFOU-011.
3. Cross-artifact boundary under audit: the complete production path under test (validated content → production bootstrap → opaque session → typed commands → core transactions → sealed receipts → rendered output → replay/rebuild) plus the standing mutation perimeter and CI governance. The artifact must name its own exact implementation commit (not this proposal's baseline `6495d7d`).
4. Motivating invariant: INV-098 (harsh acceptance) — a runnable feature is done only when caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe, LLM-independent, non-scripted, and regression-tested. Every executable claim is the implementing session's to produce; this spec asserts no green/red command result.
5. Fail-closed / evidence-consumer surface (evidence basis): the capstone re-proves — without modifying — the replay (INV-018), no-leak (INV-024/093), possession-parity (INV-094), and fail-closed-validation enforcement surfaces of 002–011, and confirms the artifact's debug/observer evidence rows stay observer-only (no actor-knowledge leakage, no nondeterminism introduced by evidence collection). It adds no production behavior. The artifact must not call the perimeter green before the standing campaign completes with zero in-surface misses/timeouts.

## Architecture Check

1. A single acceptance-only capstone that re-enumerates the spec's exit criteria as test sub-cases and renders the verdict is cleaner than scattering acceptance across the implementation tickets because it proves the composed pipeline end-to-end at one exact commit and gates the green claim on the standing campaign + required-check confirmation, rather than on any single ticket's local evidence.
2. No backwards-compatibility shim and no new production logic; the capstone exercises the pipeline 001–012 composed. It does not edit archived artifacts.

## Verification Layers

1. INV-098 (harsh, path-under-test acceptance) -> the full gate from a clean baseline: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`; plus the production-boundary conformance lane (009/011) and the standing mutation campaign (010).
2. Per-finding closure (F4-01…F4-09) -> evidence rows each naming production constructor / public command / observed state-event-projection effect / deliberate mutation or negative-compile sensitivity proof.
3. Standing-green + governance -> recorded selected denominator + complete caught/missed/unviable/timeout disposition + shard/census fingerprints; required-check names + branch-protection confirmation; food-source family resolved or honestly reported.

## What to Change

### 1. Acceptance artifact (`reports/0052_foundational_conformance_fourth_hardening_acceptance.md`)

Author the artifact per `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`: name the exact implementation commit; record the full gate results; record per-finding (F4-01…F4-09) closure with real production-path evidence; reproduce the preserved focused mutation commands from the `0049MUTWIT`/`0050FOUCONSEC`/`0051FOUCONTHI` lines; record the standing campaign disposition (from 010) and the required-check names + branch-protection confirmation (from 011); state the F4-04 implementer-recorded model choice (real transition vs honest demotion); and render the scoped verdict. Do not call the perimeter green before the standing campaign completes with zero in-surface misses/timeouts and the food-source family resolved or honestly reported.

## Files to Touch

- `reports/0052_foundational_conformance_fourth_hardening_acceptance.md` (new)

## Out of Scope

- Any production-code, test, CI, or doc change (001–012 own those; the capstone exercises and records, it does not modify them).
- Editing archived specs/tickets/reports/acceptance artifacts or the `SPEC_LEDGER.md` archived rows (immutable; the ledger row + `archive/specs/` move defer to spec acceptance per `docs/archival-workflow.md` — a Step 6 cross-spec follow-up).
- Asserting any green/red command result at decomposition time — every executable claim is the implementing session's to produce.

## Acceptance Criteria

### Tests That Must Pass

1. The full gate runs clean at the named implementation commit: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.
2. The artifact records per-finding (F4-01…F4-09) closure with production constructor / public command / observed effect / sensitivity proof for each, and reproduces the preserved focused mutation commands.
3. The standing campaign disposition shows zero in-surface misses/timeouts (food-source resolved or honestly reported); required-check names + branch-protection confirmation are recorded; the verdict is rendered scoped (no latest-main / Phase-4 / second-proof certification).

### Invariants

1. The artifact names its own exact implementation commit, not the proposal baseline `6495d7d` (INV-098).
2. The perimeter is not called green before the standing campaign completes with zero in-surface misses/timeouts (INV-092/098).

## Test Plan

### New/Modified Tests

1. `None — verification/evidence-only capstone; it re-runs the existing full gate (fmt/clippy/build/test), the production-boundary conformance lane (009/011), and the standing mutation campaign (010), and records the witnesses. No new test logic is introduced.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
2. `cargo mutants --timeout 183` — the standing campaign whose disposition (from 010) the artifact records; the perimeter is not called green until it completes with zero in-surface misses/timeouts.
