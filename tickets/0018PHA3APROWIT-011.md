# 0018PHA3APROWIT-011: 0018 scoped acceptance artifact

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence artifact (`reports/0018_ord_life_cert_scoped_acceptance.md`); no production code changes
**Deps**: `archive/tickets/0018PHA3APROWIT-008.md`, `archive/tickets/0018PHA3APROWIT-009.md`, `tickets/0018PHA3APROWIT-010.md` (leaf set — their transitive Deps cover tickets 001–007); `specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (§7)

## Problem

Spec 0018 §7 requires a scoped acceptance artifact recording, for the implementation commits, the evidence that each finding's correction and lock actually holds: the witness-census output and corrected presence-fact behavior (§7.1), the episode tamper-flip proofs and derived allowlist census output (§7.2), per-actor need ledgers explaining every golden checksum diff from the `elapsed_ticks` addition (§7.3), the extended generative corpus summary — seeds, masks, sequence count, reachability counters including sleep/terminal/interruption, zero differential divergences, both marker relations (§7.4), the seed-knowledge fixture proof (§7.5), the unvalidated-materialization rejection proof (§7.6), the census outputs and ratcheted mutation configuration (§7.7), and the explicit non-certification statement (§7.8). Without the artifact, the lineage's audit-history integrity rule (acceptance artifacts are authoritative diagnostic data — the ORD-HARD-034 lesson) is unmet for this pass. This is the capstone: it introduces no production logic and exercises everything the prior tickets composed.

## Assumption Reassessment (2026-06-10)

1. Verified against current repo at `main` `a9c62e0`: the 0016/0017 precedents exist (`reports/0016_ord_life_cert_scoped_acceptance.md`, `reports/0017_ord_life_cert_scoped_acceptance.md`) and the artifact template `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` exists; `reports/` is the established destination.
2. Spec 0018 §7 enumerates the eight evidence items; §8 requires all four gates passing before any ticket completes — this capstone re-runs and records them for the finished tree.
3. Cross-artifact boundary under audit: the acceptance-evidence contract — every §7 claim in the artifact must be backed by a command re-run at artifact-authoring time against the finished tree (never copied from earlier ticket runs), the ORD-HARD-034 lesson codified by the 0016 §6 / 0017 precedent.
4. INV-098 restated: a runnable feature is done only when it is caused, eventful, epistemically bounded, replay-safe, no-human runnable, non-scripted, and regression-tested — the artifact is the per-pass record that the 0018 surface meets the engaged subset, scoped (not full certification).
5. Deterministic-replay and validation surfaces touched as *evidence sources only*: the artifact runs the gates and records outputs; it must include the explicit non-certification statement (scoped evidence toward `ORD-LIFE-CERT`; not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`) so the evidence cannot be over-read — the severity-drift risk the lineage tracks.

## Architecture Check

1. A single capstone evidence ticket matching the 0016/0017 artifact precedent is cleaner than distributing evidence across implementation tickets: the §7 items cross-cut every prior ticket and several (corpus summary, census outputs, gate runs) are only meaningful against the finished tree. The deliverable-doubles-as-capstone note does not apply — this is an acceptance-only capstone; all test infrastructure lands in tickets 001–009.
2. No backwards-compatibility aliasing/shims: not applicable to an evidence artifact; the artifact records the no-shims constraint was honored (spec §8).

## Verification Layers

1. Evidence completeness -> manual review against spec §7: one artifact section per item 1–8, each carrying its re-run command(s) and output summary.
2. INV-018 / replay evidence -> the recorded tamper-flip, differential-oracle, and golden-ledger outputs come from commands re-run at authoring time (runbook below), not copied.
3. Four-gates record -> the artifact records `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace` passing on the finished tree.
4. Non-certification scope -> manual review: the §7.8 statement present verbatim in scope.

## What to Change

### 1. Author the artifact (manual runbook)

Create `reports/0018_ord_life_cert_scoped_acceptance.md` following `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` and the 0017 report's structure. Runbook per §7 item:

1. Run the witness census + presence-fact tests (`cargo test -p tracewake-core provenance_witness witness_kind`); record the corrected class/witness behavior and the recorded fix direction from ticket 001.
2. Run the episode tamper gates (`cargo test -p tracewake-content episode_tamper`) and the derived allowlist census; record both.
3. Re-run the golden suite (`cargo test -p tracewake-content --test golden_fixtures_run`); record per-actor need ledgers for every fixture whose checksums changed under tickets 001/002/004 (the 0016 §7.1 ledger format).
4. Run the generative suite (`cargo test -p tracewake-core --test generative_lock`); record seed set, masks, sequence count, reachability counters, zero divergences, both marker relations.
5. Run the seed-knowledge fixture (`cargo test -p tracewake-content seeded_food_source_unknown`); record no-belief + planner-unreachable + replay byte-match.
6. Run the tokenless-materialization rejection proof (`cargo test -p tracewake-content --doc`); record it.
7. Record the content-negative, fixture, and dependency-posture census outputs and the ratcheted scheduled-mutation configuration (cite the `ci.yml` step and pinned version).
8. Close with the verbatim non-certification statement (spec §7 item 8).

### 2. Gate run record

Append the four-gate run results for the finished tree.

## Files to Touch

- `reports/0018_ord_life_cert_scoped_acceptance.md` (new)

## Out of Scope

- Any production or test code change — failures surfaced while gathering evidence route back to the owning ticket.
- The spec's Status/Outcome flip, `docs/4-specs/SPEC_LEDGER.md` row, and `archive/specs/` move — spec acceptance/archival per `docs/archival-workflow.md`, outside this batch.
- Any certification claim beyond the scoped statement.

## Acceptance Criteria

### Tests That Must Pass

1. Every command in the §7 runbook re-run at authoring time with its output recorded in the artifact.
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — passing on the finished tree, recorded in the artifact.

### Invariants

1. Every evidence claim in the artifact is command-backed from the finished tree; no claim is copied from an earlier run (the ORD-HARD-034 lesson).
2. The artifact's scope statement forecloses over-reading: scoped evidence toward `ORD-LIFE-CERT` only.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. The §7 runbook commands (What to Change §1, items 1–7).
2. `cargo test --workspace`
