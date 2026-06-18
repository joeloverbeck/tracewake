# 0038SPICEREVE-005: SPINE-04 evidence — randomness and random-stream discipline

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — fills the SPINE-04 section of the acceptance artifact from existing tests/static scans.
**Deps**: 0038SPICEREVE-001

## Problem

SPINE-CERT seam SPINE-04 (spec §5) requires proof that all simulation randomness is seedable, stream-scoped, replay-visible, and auditable, with no hidden random entry points: any random draw that can affect state, scheduling, validation, or view-visible outcomes must be recorded so replay can reproduce or verify it. The seam has two admissible branches; this ticket resolves and proves the branch that holds at the implementation commit, and writes the SPINE-04 section of the acceptance artifact created by `-001`.

## Assumption Reassessment (2026-06-16)

1. The audited seams exist (verified during this session's `/reassess-spec`, 2026-06-16): `crates/tracewake-core/src/events/{envelope,log}.rs`, `replay/{rebuild,report}.rs`, `actions/pipeline.rs`, `agent/generation.rs`, `scheduler.rs`; the tests `crates/tracewake-core/tests/{anti_regression_guards,negative_fixture_runner}.rs`; and the negative fixture `tests/negative-fixtures/banned_rand_entry_points/src/lib.rs`.
2. **Conditional deliverable resolved at decomposition time (branch 1 — no state-affecting RNG):** a source scan over `crates/*/src` returns zero `rng`/`rand`/PRNG entry points (`grep -rniE '\brng\b|rand::|Lcg' crates/*/src` → 0, 2026-06-16). The only seeded generator at this commit is the test-support `Lcg` in `crates/tracewake-core/tests/support/generative.rs`, which synthesizes test inputs and is not kernel simulation state. Therefore the spec's branch (1) applies: the SPINE-04 path is marked `not exercised because no state-affecting random draw site exists at this commit`, backed by the static scan and the `banned_rand_entry_points` compile-fail fixture — explicitly, with no pass-by-silence (spec §5 SPINE-04). If a state-affecting RNG site is found at the implementation commit, branch (2) reactivates and a positive RNG behavior witness must be added before claiming the seam.
3. Shared boundary under audit: the random-draw-reference field on the event envelope and the banned-entry-point guard. The witness must record the source-scan result and the negative-fixture outcomes, and (under branch 1) the explicit "no state-affecting RNG site" certification rather than a silent pass.
4. `INV-017` (randomness must be seedable and auditable) and `INV-018` (deterministic replay is foundational) motivate this ticket: hidden unlogged authoritative randomness is forbidden, and absence-of-RNG must itself be a certified, evidenced fact.
5. This ticket audits the RNG/determinism surface as an **evidence consumer**: it runs the banned-entry-point negative fixtures and the determinism guards and records the static-scan result; it weakens no enforcement. Adversarial cases (direct `thread_rng`/OS-entropy/time-seeded RNG/nondeterministic hash iteration, an event with random effects but no draw reference, a replay random-draw record differing from the log, same-seed-different-output) must fail closed; any survivor is recorded `fail` and routed to remediation.

## Architecture Check

1. Resolving the conditional at decomposition time and certifying branch 1 with static-scan + compile-fail evidence is cleaner than forwarding the either-or as prose for the implementer to interpret: it makes the absence an explicit, re-runnable certified fact and prevents pass-by-silence.
2. No backwards-compatibility aliasing or shims introduced.

## Verification Layers

1. `INV-017` seedable/auditable randomness -> codebase grep-proof: `grep -rniE '\brng\b|rand::' crates/*/src` returns zero; the result is recorded as the branch-1 absence witness.
2. Banned-entry-point closure -> codebase grep-proof + negative-fixture: `banned_rand_entry_points` (and the env/time/process/fs negatives) fail to compile/run.
3. `INV-018` determinism under absence -> replay/golden check: duplicate-run determinism over the golden corpus holds with no random divergence (cross-referenced from `-003`).

## What to Change

### 1. Record the SPINE-04 branch resolution and static evidence

Record the source-scan result (`crates/*/src` has no state-affecting RNG site), the `banned_rand_entry_points` compile-fail outcome, and the explicit branch-1 marking `not exercised because no state-affecting random draw site exists at this commit`. If the implementation commit instead contains a state-affecting draw site, switch to branch 2: record stream identity, stable seed/config, draw references in envelopes, and a deterministic positive replay witness.

### 2. Record adversarial randomness rejections

Record the loud-failure witnesses for the banned RNG/time/env/process entry points and (under branch 2 only) for an event with random effects but a missing draw reference and a replay record differing from the log. Map each to a §5 SPINE-04 responsible layer.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001)

## Out of Scope

- The §4 cross-seam command transcript and §9.3 environment (owned by `-001`).
- The full deterministic-replay corpus as a seam (owned by `-003`, SPINE-02) — this ticket cross-references duplicate-run determinism, it does not own it.
- Remediation of any hidden/unlogged randomness found — recorded `fail` with responsible layer; fix routed to a separate `SPINE-CERT scoped remediation` ticket/spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — `banned_rand_entry_points` and the env/time/process/fs negatives fail closed.
2. `cargo test --locked -p tracewake-core --test anti_regression_guards` — randomness/determinism guards hold.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` — golden corpus replays deterministically with no random divergence.

### Invariants

1. No state-affecting random draw site exists in `crates/*/src` (branch-1 certified absence), recorded as an explicit fact, not a silent pass (`INV-017`).
2. Banned random/time/env/process entry points cannot compile or run; any randomness, if present, is seedable, stream-scoped, and replay-visible (`INV-017`/`INV-018`).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification is the recorded static source scan plus existing banned-entry-point and determinism guard runs, and the captured branch-resolution record is the deliverable.`

### Commands

1. `grep -rniE '\brng\b|rand::|Lcg' crates/*/src` (branch-resolution static scan — expected: zero matches at this commit)
2. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
3. `cargo test --locked -p tracewake-core --test anti_regression_guards`
