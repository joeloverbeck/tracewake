# 0014PHA3AORDLIF-010: Capstone — ORD-LIFE-CERT-scoped acceptance artifact + conformance index

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: No — verification-only; produces a `reports/` acceptance artifact and the §5.5 conformance index; exercises the pipeline the prior tickets composed
**Deps**: 0014PHA3AORDLIF-004, 0014PHA3AORDLIF-005, 0014PHA3AORDLIF-007, 0014PHA3AORDLIF-008, 0014PHA3AORDLIF-009

## Problem

Spec §8 requires the acceptance artifact for `0014` to be a new report under `reports/` (scoped evidence toward `ORD-LIFE-CERT`, **not** a claim the whole gate passed), and §5.5 requires a Phase 3A ordinary-life conformance index mapping each cognitive input class to exactly one source path, forbidden source, and lock. §6 (TFW checklist) and §7 (fixture strengthening) require the proof obligations realized across tickets -001…-009 to be demonstrated end-to-end. This capstone gathers that evidence; it introduces no new production logic.

## Assumption Reassessment (2026-06-09)

1. The deliverables this capstone exercises are produced by tickets -001 (no-human surface builder), -002 (sealed proposal), -003 (typed diagnostics), -004 (sealed goal bundle), -005 (typed actor-known input refs), -006/-007 (sleep affordance), -008 (embodied projection), -009 (typed metrics). The acceptance artifacts it cites — source guards (`anti_regression_guards.rs`), adversarial fixtures (`crates/tracewake-content/src/fixtures/`), no-human capstone + replay (`tests/no_human_capstone.rs`), event-schema/replay gates (`tests/event_schema_replay_gates.rs`) — exist post-implementation. The `reports/` directory is the established home for scoped acceptance/research artifacts.
2. Spec §8 enumerates the 9 required report contents (target commit + evidence, source-guard passing output, per-fixture event-log excerpts with proposal ancestry / validation / stuck diagnostic / replayed event ids, actor-known context packet ids + hashes + provenance edges, validation reports proving forged/stale/raw parameters reject without leaking hidden targets, typed no-human metrics byte-identical under replay, embodied-vs-debug TUI artifact, the responsible-layer matrix using the §8 execution-layer vocabulary, and the explicit non-certification statement). §5.5 specifies the conformance-index columns (cognitive input | allowed source | forbidden source | lock).
3. Shared boundary under audit: the whole Phase 3A ordinary-life proof surface — that each cognitive input class resolves to exactly one provenance-bearing source and one structural lock, end-to-end, with replay parity. This is a cross-artifact acceptance ticket; its proof surfaces are the prior tickets' tests, not new logic.
4. Invariant motivating this ticket: **INV-098** (feature acceptance is harsh — done only when caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human-runnable, replay-safe, LLM-independent, non-scripted, regression-tested). The capstone asserts the engaged subset for Phase 3A ordinary life.
5. Deterministic-replay / no-leak / fail-closed proof surfaces asserted (not modified): no-human metrics byte-match under replay (INV-018), embodied views omit raw assignment without context (INV-024/067), forged/stale sleep/work/food parameters reject without revealing hidden targets in actor-visible summaries (INV-106), and decision/stuck diagnostics are typed (INV-105). The capstone re-runs these; it weakens none and adds no production path.

## Architecture Check

1. A single scoped acceptance artifact + conformance index gives one reviewable place where the Phase 3A firewall is shown coherent end-to-end, mapped to the execution-layer vocabulary and the relevant `ORD-LIFE-CERT` / `TFW` gate clauses — stronger than scattered per-ticket claims and explicit that this is scoped evidence, not gate certification.
2. No backwards-compatibility shim and no new production logic: the capstone only runs and cites existing tools/tests and authors a report.

## Verification Layers

1. INV-098 (harsh acceptance) -> manual review + replay/golden-fixture check: every §8 report bullet is backed by a named passing test/command, mapped to the responsible-layer matrix.
2. INV-018 (replay) -> replay/golden-fixture check: `cargo test -p tracewake-core --test no_human_capstone` byte-matches replayed metrics/diagnostics.
3. INV-024/067/106 (no leak; bounded rejection) -> replay/golden-fixture check: the -001/-005/-008 adversarial fixtures pass; validation reports reveal no hidden targets in actor-visible summaries.
4. INV-105 (typed diagnostics) -> codebase grep-proof: all six source guards pass and no banned string-scan / raw-truth read remains.

## What to Change

### 1. Acceptance artifact (report)

Author `reports/0014_ord_life_cert_scoped_acceptance.md` containing the 9 §8 contents, with the implementation's exact target commit and manifest evidence, per-fixture event-log excerpts, context packet ids/hashes/provenance edges, validation reports, the typed metrics replay byte-match, the embodied-vs-debug artifact, the responsible-layer matrix (§8 vocabulary), and the explicit non-certification statement (scoped evidence toward `ORD-LIFE-CERT`; not latest-main, not full-project, not Phase 4, not `FIRST-PROOF-CERT`).

### 2. Conformance index (§5.5)

Include the conformance-index table (cognitive input | allowed source | forbidden source | lock) for need pressure, workplace knowledge, sleep/rest knowledge, food knowledge, route knowledge, and wait reason, each row citing the guard + fixture that locks it.

### 3. Runbook

Document the exact commands (below) and the manual TUI embodied/debug walkthrough used to capture the embodied-vs-debug artifact.

## Files to Touch

- `reports/0014_ord_life_cert_scoped_acceptance.md` (new — acceptance artifact + §5.5 conformance index)

## Out of Scope

- Any production-code change — the capstone exercises and reports; the fixes live in tickets -001…-009.
- Promoting `0014` to its archive home or updating `docs/4-specs/SPEC_LEDGER.md` (deferred to spec acceptance/archival — see the cross-spec follow-ups).
- Claiming `ORD-LIFE-CERT` is passed — the report is explicitly scoped evidence only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` — all -001…-009 guards, fixtures, and gates pass together.
2. `cargo test -p tracewake-core --test no_human_capstone` — no-human run advances and replay byte-matches (metrics + diagnostics from typed fields).
3. `cargo test -p tracewake-core --test anti_regression_guards` — all six new source guards pass.
4. Report review — every §8 bullet and §5.5 conformance row is backed by a cited passing test/command; the non-certification statement is present.

### Invariants

1. The artifact is scoped evidence toward `ORD-LIFE-CERT`, never a gate-pass claim (spec §8.9; INV-098).
2. Each cognitive input class maps to exactly one source path and one lock in the conformance index (INV-099…108).

## Test Plan

### New/Modified Tests

1. `None — verification-only acceptance ticket; the proof surfaces are the tests added in tickets -001…-009, exercised end-to-end and cited in the report.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings`
2. `cargo test --workspace`
3. `cargo test -p tracewake-core --test no_human_capstone --test anti_regression_guards --test event_schema_replay_gates` — the named proof surfaces for the report.
