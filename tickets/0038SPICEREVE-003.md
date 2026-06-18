# 0038SPICEREVE-003: SPINE-02 evidence — replay rebuild, divergence reporting, and deterministic replay gates

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — fills the SPINE-02 section of the acceptance artifact from existing tests/fixtures.
**Deps**: 0038SPICEREVE-001

## Problem

SPINE-CERT seam SPINE-02 (spec §5) requires proof that replay consumes persisted event history plus content/schema identity, rebuilds authoritative state/projections, and either matches expected fingerprints or fails loudly with typed divergence diagnostics — and that replay consults no live mutable truth, external time, nondeterministic iteration, environment, out-of-manifest filesystem discovery, network, process spawning, or UI state. This ticket gathers the deterministic duplicate-run evidence, the corrupted-package loud-failure evidence, and the host-time-invariance evidence across the golden corpus, and writes the SPINE-02 section of the acceptance artifact created by `-001`.

## Assumption Reassessment (2026-06-16)

1. The audited seams exist (verified during this session's `/reassess-spec`, 2026-06-16): `crates/tracewake-core/src/replay/{mod,rebuild,report}.rs`, `events/{log,apply}.rs`, `checksum.rs`; the tests `crates/tracewake-core/tests/{event_schema_replay_gates,golden_scenarios,generative_lock}.rs`, `crates/tracewake-content/tests/{golden_fixtures_run,fixtures_load}.rs`; and the determinism negative-fixtures `tests/negative-fixtures/{banned_env_var,banned_fs_read_and_file_open,banned_process_command_new,banned_systemtime_alias}/src/lib.rs`.
2. Spec §5 SPINE-02 requires replaying all golden fixtures in `crates/tracewake-content/src/fixtures/mod.rs` with explicit coverage for `replay_item_location_001`, `sleep_eat_work_001`, `ordinary_workday_001`, `wait_then_window_passive_charges_each_tick_once_001`, and `sleep_spanning_window_boundary_charges_each_tick_once_001`; all exist. The existing `generative_lock` test is the multi-seed property/duplicate-run determinism witness (added to this seam during `/reassess-spec`).
3. Shared boundary under audit: the persisted-package → replay-rebuild → checksum/divergence contract. The witness must record expected-vs-actual state checksum, projection checksum, event count, diagnostic count, unsupported versions, replay errors, `matches_expected`, first-divergence detail, and pre/post replay package fingerprints.
4. `INV-018` (deterministic replay is foundational), `INV-019` (snapshots/compaction may not erase live ancestry), and `INV-092` (deterministic replay is tested) motivate this ticket: a pass requires same-package → same-checksums-and-report reproduction and host-time invariance, proven by tests, not asserted.
5. This ticket audits the deterministic-replay surface as an **evidence consumer**: it runs the replay gates, golden scenarios, `generative_lock`, fixtures-load, and the banned-entry-point negative fixtures, then records witnesses; it weakens no enforcement. Corrupted-package cases (dropped last event, same-tick reorder with different ordering keys, changed content-manifest ID, unsupported future schema, dangling cause, host-time/timezone perturbation) must produce typed failure or identical output; any nondeterminism or repaired-success is recorded `fail` with responsible layer and routed to remediation.

## Architecture Check

1. Auditing replay via duplicate-run checksum equality + corrupted-package loud failure + host-time invariance is the metamorphic/property evidence shape the seam demands; it proves determinism semantics rather than a single happy-path trace. `generative_lock`'s multi-seed corpus supplies the duplicate-run breadth a fixed example set cannot.
2. No backwards-compatibility aliasing or shims introduced.

## Verification Layers

1. `INV-018` deterministic replay -> replay/golden-fixture check: same package → identical state/projection checksums and identical report across duplicate runs (`event_schema_replay_gates`, `golden_scenarios`, `generative_lock`).
2. `INV-019` ancestry preservation -> manual review + replay check: replay package fingerprints recorded pre/post; no snapshot/compaction path drops replay-critical ancestry.
3. Determinism-input isolation -> codebase grep-proof: the banned env/fs/process/systemtime negative fixtures fail to compile/run, proving replay consults no nondeterministic input.

## What to Change

### 1. Capture the SPINE-02 deterministic-replay witnesses

For each required fixture (and the broader `fixtures/mod.rs` corpus), record expected-vs-actual state checksum, projection checksum, event count, diagnostic count, unsupported versions, replay errors, `matches_expected`, and first-divergence detail; record canonical replay-package fingerprints before and after replay; record duplicate-run evidence (same package → same checksums and report). Include the `generative_lock` multi-seed duplicate-run result.

### 2. Record adversarial and host-invariance evidence

Record typed loud-failure witnesses for the required corruptions (dropped last event, same-tick reorder, changed content-manifest ID, unsupported future schema version, valid-looking event with a dangling cause) and the host-time/timezone re-run showing identical output or fail-closed diagnostics — proving no nondeterministic success. Map each failure to a §5 SPINE-02 responsible layer.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001)

## Out of Scope

- The §4 cross-seam command transcript and §9.3 environment (owned by `-001`).
- Event-envelope field-coverage and append-only integrity as a seam (owned by `-002`, SPINE-01).
- Save-package/manifest/schema-version integrity as a seam (owned by `-006`, SPINE-05) — this ticket records replay-package fingerprints, not the full save-manifest contract.
- Remediation of any replay nondeterminism or repaired-success found — recorded `fail` with responsible layer; fix routed to a separate `SPINE-CERT scoped remediation` ticket/spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — replay rebuild and divergence diagnostics.
2. `cargo test --locked -p tracewake-core --test golden_scenarios` and `--test generative_lock` — golden + multi-seed duplicate-run determinism.
3. `cargo test --locked -p tracewake-content --test fixtures_load` and `--test golden_fixtures_run` — full fixture corpus replays.
4. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — banned env/fs/process/systemtime entry points fail closed.

### Invariants

1. Same package + seed/config → identical state checksum, projection checksum, and replay report across runs (`INV-018`/`INV-092`).
2. Every corrupted package yields a typed failure with a first-divergence record; none yields a repaired success (`INV-018`). Replay package fingerprints preserve ancestry (`INV-019`).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification runs existing replay/golden/property/determinism-guard tests and the captured checksums + divergence reports are the deliverable.`

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates && cargo test --locked -p tracewake-core --test generative_lock`
2. `cargo test --locked -p tracewake-core --test golden_scenarios && cargo test --locked -p tracewake-content --test golden_fixtures_run`
3. `cargo test --locked -p tracewake-content --test fixtures_load`
