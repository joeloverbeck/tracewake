# 0015PHA3AEVECOG-011: Capstone — scoped ORD-LIFE-CERT acceptance artifact

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Verification-only — new `reports/0015_ord_life_cert_scoped_acceptance.md`; exercises the prior tickets end-to-end, no new production logic
**Deps**: 0015PHA3AEVECOG-004, 0015PHA3AEVECOG-005, 0015PHA3AEVECOG-006, 0015PHA3AEVECOG-007, 0015PHA3AEVECOG-008, 0015PHA3AEVECOG-009, 0015PHA3AEVECOG-010

## Problem

Spec 0015 §7 requires an acceptance artifact recording, for the implementation commits, the scoped evidence toward `ORD-LIFE-CERT`. This capstone produces `reports/0015_ord_life_cert_scoped_acceptance.md` (mirroring the existing `reports/0014_ord_life_cert_scoped_acceptance.md`), enumerating the seven evidence items §7 names and exercising every prior implementation ticket end-to-end. It introduces no new production logic; it assembles and verifies the evidence the earlier tickets composed, and states the explicit non-certification boundary.

## Assumption Reassessment (2026-06-09)

1. Current code (verified): the prior acceptance artifact `reports/0014_ord_life_cert_scoped_acceptance.md` is the format exemplar. The surfaces this report cites all land in 001–010: seed events (`-001`), perception events (`-002`), the sealed builder + `source_event_ids` (`-003`), source guards + context-hash rebuild gate + negative fixtures (`-004`), the fail-closed audit gate (`-005`), embodied context-backed surfaces (`-006`), interruption with prorated deltas (`-007`), authored tuning (`-008`), glob/census guards (`-009`), conformance docs (`-010`).
2. Specs/docs: spec 0015 §7 (the seven evidence items) and §3 "Gate posture" (scoped toward `ORD-LIFE-CERT`, certifies nothing larger); `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` and `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (ORD-LIFE-CERT clauses); `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` and `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` (the acceptance gates this evidence maps to).
3. Shared boundary under audit: the acceptance artifact is the assembled proof surface across all prior tickets; it exercises (does not modify) their outputs. Per `_TEMPLATE.md` the Files-to-Touch is the report itself plus any new smoke/replay test it adds, not the upstream tickets' files.
4. INV-098 — feature acceptance is harsh: a runnable feature is done only when it is caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe, LLM-independent, non-scripted, and regression-tested; the capstone checks the ordinary-life slice against this bar. INV-091 (no-human tests mandatory), INV-092 (deterministic replay tested), INV-093 (leakage tested).
5. Fail-closed / actor-knowledge / deterministic-replay surface: the capstone *verifies* these surfaces rather than implementing them. It asserts: per-decision actor-known context hashes match rebuilt-from-log counterparts (determinism); the no-human run advances with no leakage (the embodied/debug raw-vs-context artifact); the interrupted-sleep trace is byte-identical live and under replay. No code surface is weakened — this ticket only reads.
6. Schema extension: none — a report plus optional verification-only test scaffolding. (Menu item not applicable.)

## Architecture Check

1. A single capstone artifact gives the scoped certification one auditable home that re-enumerates expected counts from fixtures at test start (no hardcoded counts), exercising the whole ordinary-life channel end-to-end — stronger evidence than per-ticket assertions read in isolation. It introduces no production logic, so it cannot mask a gap behind new behavior.
2. No shims: the report records the real post-implementation state; the explicit non-certification statement prevents over-claiming.

## Verification Layers

1. INV-091 → replay/golden-fixture check: a no-human run advances across the ordinary-life fixtures; the report cites the run.
2. INV-092 → replay/golden-fixture check: per-decision actor-known context hashes match their rebuilt-from-log counterparts (byte-match evidence, item §7.3); interrupted-sleep trace byte-identical live and under replay (§7.5).
3. INV-093 → manual review + replay: the TUI embodied/debug artifact shows raw food/sleep/route present in state, absent from context, omitted from embodied output, visible in debug comparison (§7.6).
4. INV-098 → invariants alignment check: the seven §7 evidence items map to the relevant Phase acceptance gates; the explicit non-certification statement (§7.7) bounds the claim.

## What to Change

### 1. Acceptance artifact

Author `reports/0015_ord_life_cert_scoped_acceptance.md` recording: (1) source-guard inventory + passing output incl. the census guard; (2) per adversarial fixture, an event-log excerpt with proposal ancestry or stuck diagnostic + replayed event ids; (3) per-decision actor-known context hashes with rebuilt-from-log counterparts; (4) seed-time knowledge events for one canonical fixture actor with INV-063 prehistory marks; (5) an interrupted-sleep trace with prorated deltas live and under replay; (6) the TUI embodied/debug raw-vs-context artifact; (7) the explicit non-certification statement (scoped evidence toward `ORD-LIFE-CERT`; not full-project, not Phase 4 entry, not `FIRST-PROOF-CERT`).

### 2. Optional verification scaffolding

If an end-to-end smoke/replay test is the right home for the byte-match and no-human-run evidence, add it under `crates/tracewake-core/tests/` (verification-only); otherwise cite existing fixture/replay commands.

## Files to Touch

- `reports/0015_ord_life_cert_scoped_acceptance.md` (new)
- `crates/tracewake-core/tests/` (new — only if an end-to-end smoke/replay test is added for the capstone evidence; otherwise verification is command-based against existing fixtures)

## Out of Scope

- Any production-logic change (owned by 001–009).
- Documentation rows in the conformance index / execution doc (`0015PHA3AEVECOG-010`).
- The `docs/4-specs/SPEC_LEDGER.md` archived-spec row and the `archive/specs/` move (deferred to spec acceptance/archival — cross-spec follow-up).

## Acceptance Criteria

### Tests That Must Pass

1. The report enumerates all seven §7 evidence items with concrete excerpts (no placeholders), counts re-derived from fixtures.
2. The byte-match evidence (context hash vs rebuilt-from-log; interrupted-sleep live vs replay) is reproducible by the cited commands.
3. `cargo test --workspace` green (the exercised surfaces all pass).

### Invariants

1. The capstone adds no production logic; it exercises the pipeline 001–009 composed.
2. The non-certification boundary is stated explicitly (scoped evidence only).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/` — optional end-to-end smoke/replay test for the no-human-run + byte-match evidence (verification-only), or `None — verification-only; evidence assembled by running existing fixtures/replay commands` if no new test is needed.

### Commands

1. `cargo test --workspace` (exercises all ordinary-life fixtures and guards)
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed 2026-06-09.

Produced `reports/0015_ord_life_cert_scoped_acceptance.md` as the scoped
ORD-LIFE-CERT evidence artifact for spec 0015 at target implementation commit
`4672cf20fa32b6caa5e2acb5c044b167c2208e57`. The report records the ticket
manifest for 001–010, re-derived fixture count, all seven required evidence
sections, concrete existing test surfaces, replay/hash evidence, seed-time
knowledge event evidence, interrupted-sleep evidence, embodied/debug
raw-vs-context evidence, and the explicit non-certification boundary.

No production logic or new test was added. The existing capstone, replay,
anti-regression, and fixture tests already exercise the required byte-match and
no-human evidence surfaces.

Verification:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace --all-targets --locked`
4. `cargo test --workspace --quiet`
