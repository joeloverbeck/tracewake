# 0016PHA3ANEEACC-003: From-log context-hash re-derivation gate at every decision

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — replay-frontier context rebuild gate in `tracewake-core` replay path + golden runner; tamper test
**Deps**: `archive/tickets/0016PHA3ANEEACC-002.md`

## Problem

ORD-HARD-016: the decision context-hash replay gate is tautological. `crates/tracewake-content/tests/golden_fixtures_run.rs::no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash` computes `compute_holder_known_context_hash(record.actor_known_inputs.clone())` and asserts equality with `record.actor_known_context_hash` — but `DecisionTraceRecord::from_trace` and `deserialize_canonical` (`agent/trace.rs`) derive the stored hash from that same stored input vector. The assertion is `hash(x) == hash(x)` and can never fail. The only non-tautological part is the cited-`source_events`-exist check (citation integrity, not derivability). No test re-derives any decision's context from the replayed log. A proof artifact that gates nothing is a decorative proof surface (INV-018, INV-102, INV-105); the 0015 ORD-HARD-008 structural lock — "a rebuild test that recomputes every decision's context from replayed events and byte-matches the recorded hash" — was not implemented as specified.

The fix: during replay rebuild, at each recorded decision frontier, re-run the production surface builder (`NoHumanActorKnownSurfaceBuilder::from_event_log`) against the replayed event prefix and rebuilt `AgentState`, serialize the resulting context inputs, and byte-match the recorded `actor_known_context_hash` — every decision in every golden no-human run, not a sample.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: the test at `golden_fixtures_run.rs:1344` asserts stored-input hash equality at :1373–1374 (tautological per `trace.rs` `from_trace` :236–239 and `deserialize_canonical` :278–293) plus citation integrity at :1376–1388. `NoHumanActorKnownSurfaceBuilder::from_event_log` (`agent/no_human_surface.rs:59–73`) is used by production (`scheduler.rs`) and builder unit tests only — never to verify decision-trace hashes. `replay/rebuild.rs::rebuild_projection` already reconstructs `AgentState` (:53, :143), so the rebuilt-state input the gate needs exists.
2. Spec/docs: spec 0016 §ORD-HARD-016 (evidence, required correction, structural lock), §9 (per-decision builder re-runs may be slow on large logs — acceptable for golden runs; do not sample); archived `archive/specs/0015_*` :196–197/:426/:521 (the promised rebuild gate this ticket implements as specified); `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-018, INV-102, INV-105.
3. Shared boundary under audit: the replay rebuild path ↔ the production surface builder. The gate must invoke the production builder — no parallel reimplementation of context derivation, or the gate proves the reimplementation rather than the channel.
4. INV-018 — deterministic replay is foundational: recorded decision context must be reconstructible from the log. INV-102 — cognition inputs require provenance sufficient for replay review. INV-105 — diagnostics are authoritative, not decorative: a gate that cannot fail proves nothing. Restated before trusting the ticket narrative.
5. Deterministic-replay enforcement surface: this ticket *is* the gate. It must (a) cover every decision in every golden no-human run, (b) be proven able to fail via the tamper test (corrupt one seed knowledge event in a copied log ⇒ gate fails at the first affected decision), and (c) not weaken the existing citation-integrity check, which remains as a complementary assertion. Depends on `archive/tickets/0016PHA3ANEEACC-002.md` because the gate replays the repriced goldens and the post-002 duration model (spec §8 ordering).
6. Adjacent contradictions: `deserialize_canonical`'s hash-from-stored-inputs consistency check is internally valid (it detects serialization corruption) — it stays, but the acceptance gate is the from-log re-derivation. The overturned 0015 outcome claim is recorded in the conformance index by ticket 0016PHA3ANEEACC-013.

## Architecture Check

1. Re-running the production builder at the replay frontier is the only design that proves the cognition channel rebuilds from the log: hashing stored strings proves storage round-tripping, not derivability. Placing the gate in the replay path with the production builder (no test-only reimplementation) means any future builder change that breaks log-derivability fails the gate immediately. Full coverage (every decision, no sampling) is mandated by the spec's §9 risk note — golden logs are small enough.
2. No backwards-compatibility aliasing/shims: the tautological assertion is replaced by the re-derivation gate (citation-integrity assertions are kept as distinct checks, not as a fallback for the gate).

## Verification Layers

1. INV-018 (log-derivable context) → replay/golden-fixture check: at every recorded decision frontier in every golden no-human run, the production builder's rebuilt context byte-matches the recorded `actor_known_context_hash`.
2. INV-105 (gate can fail) → tamper test: a copied log with one corrupted seed knowledge event fails the gate at the first affected decision.
3. INV-102 (provenance) → existing citation-integrity check retained: every consumed fact's cited `source_events` exist in the log.
4. Production-builder fidelity → codebase grep-proof: the gate calls `NoHumanActorKnownSurfaceBuilder::from_event_log`; no parallel context-derivation implementation exists in the test/replay path.

## What to Change

### 1. Replay-frontier rebuild gate

In the replay path (`replay/rebuild.rs`, or the golden runner consuming it — keep the builder invocation in one place): at each recorded decision frontier, build the actor-known surface from the replayed event prefix + rebuilt `AgentState` via `NoHumanActorKnownSurfaceBuilder::from_event_log`, serialize the context inputs canonically, and byte-match the recorded `actor_known_context_hash`. Wire into every golden no-human run, covering every decision.

### 2. Tamper test

Copy a golden log, corrupt one seed knowledge event, and assert the gate fails at the first affected decision — proving the gate is not decorative.

### 3. Demote stored-input trust

`agent/trace.rs::deserialize_canonical` keeps its internal consistency check, documented as serialization integrity only; the acceptance gate is the from-log re-derivation. The old tautological assertion in `golden_fixtures_run.rs` is replaced by the gate (citation-integrity checks retained).

## Files to Touch

- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/src/agent/trace.rs` (modify — consistency-check documentation/demotion)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — gate wiring, tamper test, replace tautological assertion)

## Out of Scope

- Golden checksum repricing and the need ledger (`archive/tickets/0016PHA3ANEEACC-002.md`).
- Replay ordering verification, `WorldNoOp` census, and rebuild poisoning (0016PHA3ANEEACC-011 — shares `rebuild.rs`; declares Deps on this ticket).
- Source-event witness types and dangling-provenance fail-closed at the transaction boundary (0016PHA3ANEEACC-004).

## Acceptance Criteria

### Tests That Must Pass

1. The re-derivation gate passes for every decision in every golden no-human run (genuine logs).
2. The tamper test fails the gate at the first affected decision on the corrupted copy (and the test asserts that failure).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. The gate uses the production surface builder — no parallel context-derivation reimplementation in tests or replay.
2. Coverage is every decision in every golden no-human run — no sampling.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — the per-decision re-derivation gate (replacing the tautological hash assertion) + the tamper test.
2. `crates/tracewake-core/src/replay/rebuild.rs` — unit coverage for the decision-frontier rebuild hook (frontier identification, prefix/state handoff).

### Commands

1. `cargo test -p tracewake-content golden`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Implemented the replay-path decision context hash re-derivation gate. `rebuild_projection` and `run_replay` now report `decision_context_hash_failures`, and replay marks the report non-matching when a recorded decision trace cannot be rebuilt from the event-log prefix, rebuilt state, and production `NoHumanActorKnownSurfaceBuilder`.

Decision trace events now persist no-human window bounds so replay can reconstruct the same actor-known window framing. The old stored-input hash assertion was replaced with a from-log rebuild assertion, while the source-event citation check remains. Added a tamper test that changes a seed sleep-place belief in a copied log and asserts the replay gate fails with `decision_context_hash_mismatch`.

Verification passed:

1. `cargo test -p tracewake-content no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`
2. `cargo test -p tracewake-content no_human_decision_context_hash_gate_fails_when_source_evidence_tampered`
3. `cargo test -p tracewake-content --test golden_fixtures_run`
4. `cargo fmt --all --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo build --workspace --all-targets --locked`
7. `cargo test --workspace`
