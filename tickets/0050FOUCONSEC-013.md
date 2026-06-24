# 0050FOUCONSEC-013: Capstone — acceptance artifact + full re-proof

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: 0050FOUCONSEC-004, 0050FOUCONSEC-011, 0050FOUCONSEC-012

## Problem

Spec-0050 §7 + §8 (closure step 9 close-out): produce the implementation/review acceptance artifact recording the static-survey findings and the executed remediation evidence, and re-prove the whole package end-to-end. This is the acceptance-only capstone: it introduces no new production logic; it exercises the pipeline the `-001`…`-012` tickets composed and renders the verdict.

## Assumption Reassessment (2026-06-24)

1. The repo convention for an acceptance artifact is `reports/<NNNN>_<slug>_acceptance.md` (e.g. `reports/0048_foundational_conformance_hardening_acceptance.md`, referenced by `archive/specs/0048_*` §Outcome); `reports/` exists (holds the driver report). The §6 suites all exist (`world_step_coordinator`, `replay_temporal_frontier`, `holder_known_interval_projection`, `salient_stop_actor_known`, `reservation_body_exclusive_census`, `generative_lock`, `negative_fixture_runner`; `playable_capability_parity`, `parity_adversarial`, `tui_seam_conformance`, `command_loop_session`, `embodied_flow`). Verified at `HEAD` (`8d7c119`).
2. Spec-0050 §7/§8 are authoritative: every pass/fail must be a recorded command result, not an assumed green/red; do not rewrite archived acceptance history. The `0050` spec's own `SPEC_LEDGER.md` archived row + `archive/specs/` move are deferred to spec acceptance (cross-spec follow-up), not this ticket.
3. Shared boundary under audit: the spec's §Exit-criteria / §Acceptance-evidence — the capstone exercises every prior ticket end-to-end; it does not modify their files. Cross-artifact: it consolidates evidence from `-001`…`-012` into one artifact.
4. `INV-098` (harsh feature acceptance: caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe, LLM-independent, non-scripted, regression-tested) motivates this capstone: it is the harness that proves the whole package meets the bar.
5. Enforcement surface (evidence-consumer basis): the capstone re-proves the replay (`INV-018`/`INV-092`), actor-knowledge/no-leak (`INV-067`/`INV-093`), no-human (`INV-004`/`INV-091`), and possession-parity (`INV-094`) enforcement surfaces by running their suites; it introduces no behavior and no leakage/nondeterminism path. For each load-bearing claim it deliberately breaks the production behavior (not the fixture) per the §4 evidence-honesty checks and confirms the witness fails.

## Architecture Check

1. A single acceptance-only capstone gated on the leaf set (`-004` compile-fail, `-011` mutation, `-012` docs) renders one verdict over coherent evidence — cleaner than scattering the verdict across tickets, and it cannot pass until every upstream surface exists. It adds no production logic, only the written artifact + the re-run.
2. No backwards-compatibility shims: N/A (acceptance-only); it edits no upstream file and rewrites no archived history.

## Verification Layers

1. `INV-098` (harsh acceptance) → replay/golden-fixture check + manual review: the artifact enumerates the spec §6/§7 evidence as sub-cases with recorded command results (workspace gates, §6 suites, replay/golden, mutation map), each mapped to its phase acceptance gate.
2. `INV-004`/`INV-091` (no-human) + `INV-094` (possession parity) → replay/golden-fixture check: the no-human and human/no-human differential suites run green and are cited with transcripts.
3. `INV-018`/`INV-092` (deterministic replay) → replay/golden-fixture check: replay/golden lanes (including the fail-closed temporal report from `-008`) run green; the evidence-honesty negatives (disable discovery, drop trace, swallow `Stuck`, re-enable TUI append, omit temporal failure, collapse salience) each fail as expected.

## What to Change

### 1. Author the acceptance artifact

Create `reports/0050_foundational_conformance_second_hardening_acceptance.md` recording: the static-survey driver findings (F-01…F-08); the executed remediation evidence per ticket; the exact workspace-gate transcripts (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace --locked`); the §6 focused-suite results; the replay/golden lane results; the mutation campaign map (from `-011`, with denominators/caught/missed/unviable and the preserved `0049` rerun); and the evidence-honesty negative results. Re-enumerate expected counts from fixtures at test start, not hardcoded. Per execution `10` typed-before-rendered and anti-vacuity rules.

### 2. Render the verdict

State the scoped acceptance verdict for spec-0050 at the exact implementation/evidence commit(s) — named by the implementing session, not assumed latest `main`. Note the deferred cross-spec follow-up (the `0050` `SPEC_LEDGER.md` archived row + `archive/specs/` move at acceptance).

## Files to Touch

- `reports/0050_foundational_conformance_second_hardening_acceptance.md` (new)

## Out of Scope

- Any production-code or test change — owned by `-001`…`-011` (the capstone exercises, does not modify them).
- Documentation truthing + `0049MUTWIT` record — `0050FOUCONSEC-012`.
- The `0050` spec's own ledger row / `archive/specs/` move — deferred to spec acceptance (cross-spec follow-up).

## Acceptance Criteria

### Tests That Must Pass

1. All four workspace gates pass with retained transcripts; the §6 focused suites and replay/golden lanes pass; the mutation map (from `-011`) is recorded with exact denominators.
2. Each evidence-honesty negative (disable actor/process discovery, drop decision-trace append, swallow `Stuck`, re-enable TUI post-step perception, omit temporal failure from replay success, collapse salience novelty) makes its witness fail — recorded.
3. `cargo test --workspace --locked` and `cargo build --workspace --all-targets --locked` are green; the acceptance artifact exists at the convention path and names exact commit(s).

### Invariants

1. Every pass/fail in the artifact is a recorded command result, not an assumed green/red (`INV-098`; execution `10`).
2. The capstone introduces no production logic and rewrites no archived history.

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; it assembles `reports/0050_foundational_conformance_second_hardening_acceptance.md` from the upstream tickets' suites and re-runs them. No new test code.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked`
2. `cargo test -p tracewake-core --test world_step_coordinator --test replay_temporal_frontier --test holder_known_interval_projection --test salient_stop_actor_known --test reservation_body_exclusive_census --test generative_lock --test negative_fixture_runner`
3. `cargo test -p tracewake-tui --test playable_capability_parity --test parity_adversarial --test tui_seam_conformance --test command_loop_session --test embodied_flow`
