# 0044FIRPROCER-008: FIRST-PROOF-08 — possession parity, embodied view, and debug split in the missing-property run

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-08 consumes `EPI-CERT` and certifies participation at `U`: binding a controller changes input routing only. It must not reset intention, needs, routine, memory, expectation, contradiction, provenance, or world access; it must not reveal the hidden item's location; debug remains non-diegetic. Bind/unbind/rebind must change controller ownership and input origin only; the possessed actor must see the same holder-known expectation/observation/contradiction available to the autonomous actor; embodied menus must derive from holder-known context and legal affordances; autonomous and human-origin proposals must traverse the same validators and event path. This ticket records the FIRST-PROOF-08 possession-parity witnesses, debug-non-interference negatives, and pre/post-bind state comparison into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/{controller.rs, view_models.rs, debug_capability.rs, debug_reports.rs}` and TUI `app.rs`/`input.rs`/`render.rs`/`debug_panels.rs`/`run.rs`/`transcript.rs` (all confirmed present this session). Fixtures `possession_parity_001`, `possession_does_not_reset_intention_001`, `debug_attach_001`, `debug_omniscience_excluded_001`, `view_filtering_001`, and `embodied_menu_lags_truth_change_without_perception_001` exist.
2. Spec §7 FIRST-PROOF-08 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; doctrine foundation `08`/`12`, architecture `10`, execution `02` gates `POSSESSION-PARITY`/`VIEW-DEBUG-SPLIT` and execution `07` bind it. This consumes `EPI-CERT` (commit `726b2a1f…`) as a participation relation.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-08 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-08): `INV-094` (possession parity is tested — binding changes input only), `INV-006` (possession transfers no world knowledge), `INV-108` (human possession is cognition-neutral), `INV-067`/`INV-068` (embodied shows actor-known reality; debug is visibly non-diegetic). Restate before trusting the narrative: possession changes input routing only and does not reset, privilege, or reveal hidden custody.
5. This ticket audits/reads (does not modify) the holder-known projection, intention-lifecycle, view-model, and debug-quarantine enforcement surfaces. It runs fixtures and records witnesses only; debug attach/detach must not alter the authoritative event log, actor-known context hash, candidate set, plan, or replay, and debug rows are packaged separately and excluded from embodied/cognition fingerprints. No nondeterminism is introduced.

## Architecture Check

1. Comparing pre/post-bind actor state, context hash/frontier, intention/routine IDs, event sequence, projection checksum, view-model fingerprint, and transcript — across autonomous/possessed/rebound/unpossessed runs — is the only check that proves possession is input-only; spec §7 requires that a truth change without perception leaves the embodied view stale in the modeled way.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may grant a human-only affordance or leak hidden custody on bind.

## Verification Layers

1. `INV-094`/`INV-108` possession parity / cognition-neutral -> replay/golden-fixture check (bind/unbind/rebind change controller ownership and input origin only; pre/post-bind context hash, intention/routine IDs, event sequence, and projection checksum are equal; autonomous and human-origin proposals traverse the same validators and event path).
2. `INV-006` no knowledge transfer -> manual review + codebase grep-proof (possession does not refresh stale belief, add memory, disclose hidden custody, reset intention, or create a special action).
3. `INV-067`/`INV-068` embodied/debug split -> adversarial check (debug attach/detach does not alter authoritative log, context hash, candidate set, plan, or replay; a truth change without perception leaves the embodied view stale; debug rows are excluded from embodied/cognition fingerprints).

## What to Change

### 1. Record positive possession-parity witnesses

Run `possession_parity_001`, `possession_does_not_reset_intention_001`, `view_filtering_001`, and the §7 positive path. Record that bind/unbind/rebind changes controller ownership and input origin only; the possessed actor sees the same holder-known expectation/observation/contradiction as the autonomous actor; embodied menus derive from holder-known context and current legal affordances; debug panels are explicitly marked and may compare truth with actor-known state; and autonomous and human-origin proposals traverse the same validators and event path. Record pre/post-bind actor state, context hash/frontier, intention/routine IDs, event sequence, projection checksum, view-model fingerprint, and transcript.

### 2. Record possession and debug non-interference negatives

Record the §7 adversarial cases: possession does not refresh stale belief, add a memory, disclose hidden custody, reset intention, or create a special action; debug attach/detach does not alter authoritative event log, actor-known context hash, candidate set, plan, or replay; a truth change without perception leaves the embodied view stale in the modeled way; stale/forged semantic input IDs reject rather than falling back to hidden truth; switching which actor is human-controlled does not change no-human-world rules. Debug rows are packaged separately and excluded from embodied/cognition fingerprints; each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-08 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any possession/debug-split defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Re-auditing `EPI-CERT` internals (consumed within scope, spec §4); embodied temporal rendering (`-014`) and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `cargo test --locked -p tracewake-core --test golden_scenarios` pass; bind/unbind/rebind change input routing only and pre/post-bind context hash, intention/routine IDs, and projection checksum are equal.
2. `cargo test --locked -p tracewake-content --test golden_fixtures_run`, `cargo test --locked -p tracewake-tui --test embodied_flow`, and `--test tui_acceptance` pass; the possessed actor sees the same holder-known facts as the autonomous actor and embodied menus derive from holder-known context.
3. `cargo test --locked -p tracewake-tui --test adversarial_gates` and `--test transcript_snapshot` pass; debug attach/detach alters no authoritative/actor-known output, a truth change without perception leaves the view stale, and debug rows are excluded from embodied/cognition fingerprints.

### Invariants

1. No possession-caused cognition/state change, no human-only affordance, no debug leakage, no hidden-truth view update, no unequal validation/event semantics.
2. Debug rows are non-diegetic and excluded from authoritative/embodied checksums; switching the human-controlled actor does not change no-human-world rules.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test golden_scenarios`
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
4. `cargo test --locked -p tracewake-tui --test adversarial_gates`
5. `cargo test --locked -p tracewake-tui --test embodied_flow`
6. `cargo test --locked -p tracewake-tui --test tui_acceptance`
7. `cargo test --locked -p tracewake-tui --test transcript_snapshot`

## Outcome

Completed: 2026-06-21

Recorded FIRST-PROOF-08 in the shared acceptance artifact as passed for its
possession-parity and embodied/debug split scope. The evidence packet now
includes command-ledger rows, gate/scenario references, a FIRST-PROOF-08 audit
section, and two evidence ledger items: `E-0044-008-possession-parity` and
`E-0044-008-debug-split`.

Verification run:

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` -> pass, 17 passed.
2. `cargo test --locked -p tracewake-core --test golden_scenarios` -> pass, 16 passed.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` -> pass, 42 passed.
4. `cargo test --locked -p tracewake-tui --test adversarial_gates` -> pass, 15 passed.
5. `cargo test --locked -p tracewake-tui --test embodied_flow` -> pass, 6 passed.
6. `cargo test --locked -p tracewake-tui --test tui_acceptance` -> pass, 11 passed.
7. `cargo test --locked -p tracewake-tui --test transcript_snapshot` -> pass, 3 passed.
