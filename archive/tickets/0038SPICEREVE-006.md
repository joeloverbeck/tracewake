# 0038SPICEREVE-006: SPINE-05 evidence — save package, manifest integrity, schema versioning, and upcast/read discipline

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — fills the SPINE-05 section of the acceptance artifact from existing tests/fixtures.
**Deps**: 0038SPICEREVE-001

## Problem

SPINE-CERT seam SPINE-05 (spec §5) requires proof that a replayable save package binds event log, content manifest, schema/ruleset identity, snapshot ancestry (if snapshots exist), and replay provenance — that content manifests and fingerprints are part of the replay contract, saves may optimize load but never replace event ancestry, and unsupported schema versions or missing upcasters fail loudly. This ticket gathers the content-manifest identity/fingerprint records, schema-conformance evidence, the manifest/byte/version mismatch loud-failure witnesses, and writes the SPINE-05 section of the acceptance artifact created by `-001`.

## Assumption Reassessment (2026-06-16)

1. The audited seams exist (verified during this session's `/reassess-spec`, 2026-06-16): `crates/tracewake-content/src/{manifest,load,schema,serialization,validate}.rs`, `crates/tracewake-core/src/events/{envelope,log}.rs`, `replay/report.rs`; the tests `crates/tracewake-content/tests/{fixtures_load,schema_conformance,golden_fixtures_run}.rs` and `crates/tracewake-core/tests/event_schema_replay_gates.rs`.
2. **Conditional deliverable resolved at decomposition time (snapshot support):** spec §5 SPINE-05 makes snapshot-assisted-load evidence conditional on snapshot support existing at the implementation commit. The reassessment static survey (spec §10) flags that save/package certification may currently be represented by content manifests and replay packages rather than a full save subsystem. The ticket records the conditional resolution: if no snapshot subsystem exists, mark snapshot-assisted load `not applicable — no snapshot subsystem at this commit` and certify manifest+replay-package integrity; if it exists, add the fixture proving snapshot-assisted load equals event-log replay. Either way the save-manifest gap, if present, is named (not silently skipped) per spec §10.
3. Shared boundary under audit: the content-manifest fingerprint + schema-identity + replay-package contract. The witness must record content-manifest identity, stable fingerprint, source-file list, canonical path ordering, schema versions, validation diagnostics, and the save/replay-package manifest (event-log fingerprint, content-manifest fingerprint, schema/ruleset identity, optional snapshot fingerprint + ancestry pointer).
4. `INV-019` (snapshots/compaction may not erase live ancestry) and `INV-020` (event schema evolution is mandatory) motivate this ticket: saves preserve ancestry and schema identity, and unsupported versions/missing upcasters fail loudly rather than silently downgrading.
5. This ticket audits the content-validation and schema-version surfaces as an **evidence consumer**: it runs `fixtures_load`, `schema_conformance`, `forbidden_content`, `golden_fixtures_run`, and `event_schema_replay_gates`, then records witnesses; it weakens no enforcement. Adversarial cases (file-order change must not change fingerprint, byte change must change fingerprint, missing/extra unmanifested file or mismatched manifest ID must fail loudly, unsupported event/content schema must fail at validation/replay not in a late panic, snapshot without ancestry must be rejected) must fail closed; any survivor is recorded `fail` and routed to remediation.

## Architecture Check

1. Auditing the seam via manifest-fingerprint stability/sensitivity + schema-conformance + loud mismatch failures proves the replay-contract integrity and the no-silent-downgrade semantics; resolving the snapshot conditional at decomposition time prevents a pass-by-silence on a missing save subsystem.
2. No backwards-compatibility aliasing or shims introduced.

## Verification Layers

1. `INV-020` schema evolution -> codebase grep-proof + schema validation: unsupported event/content schema versions fail at content/schema validation or replay (`schema_conformance`, `event_schema_replay_gates`), not in a late panic or silent downgrade.
2. Manifest integrity -> schema validation + manual review: file-order invariance and byte-change sensitivity of the canonical fingerprint recorded; missing/extra/mismatched manifest fails loudly (`fixtures_load`, `forbidden_content`).
3. `INV-019` ancestry preservation -> replay/golden check: the replay package reuses `content_manifest_id` consistently across envelopes and replay reports; snapshot (if any) does not replace event ancestry.

## What to Change

### 1. Capture the SPINE-05 manifest and schema witnesses

Record content-manifest identity, stable fingerprint, source-file list, canonical path ordering, schema versions, and validation diagnostics for the `fixtures/mod.rs` corpus; record the save/replay-package manifest (event-log fingerprint, content-manifest fingerprint, schema/ruleset identity, optional snapshot fingerprint + ancestry pointer) and the consistent `content_manifest_id` reuse across envelopes and replay reports.

### 2. Record adversarial manifest/schema mismatch evidence and the snapshot resolution

Record at least one adversarial manifest/fingerprint mismatch artifact (byte change → fingerprint change; file-order change → fingerprint stable; missing/extra/unmanifested file → loud failure; unsupported schema → validation/replay failure). Record the snapshot-conditional resolution from Assumption Reassessment #2, naming any save-manifest gap explicitly. Map each failure to a §5 SPINE-05 responsible layer.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001)

## Out of Scope

- The §4 cross-seam command transcript and §9.3 environment (owned by `-001`).
- The event-log/envelope append-only seam (owned by `-002`, SPINE-01) and the replay determinism seam (owned by `-003`, SPINE-02) — this ticket records manifest/schema/package integrity, not the full replay corpus.
- Remediation of any manifest/schema/ancestry defect found — recorded `fail` with responsible layer; fix routed to a separate `SPINE-CERT scoped remediation` ticket/spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test fixtures_load` — every content fixture loads under a stable content-manifest fingerprint.
2. `cargo test --locked -p tracewake-content --test schema_conformance` — schema acceptance for all live fixture modules.
3. `cargo test --locked -p tracewake-content --test forbidden_content` and `--test golden_fixtures_run` — unmanifested/byte-change/loud-failure cases and positive corpus.
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — unsupported schema fails at validation/replay; `content_manifest_id` reused consistently.

### Invariants

1. Content-manifest fingerprint is stable under file-order change and sensitive to byte change; missing/extra/mismatched manifest fails loudly (`INV-020`).
2. Unsupported event/content schema fails at content/schema validation or replay, never a silent downgrade or late panic; snapshots (if any) preserve event ancestry (`INV-019`/`INV-020`).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification runs existing manifest/schema/content-validation tests and the captured fingerprints + mismatch artifacts are the deliverable.`

### Commands

1. `cargo test --locked -p tracewake-content --test fixtures_load && cargo test --locked -p tracewake-content --test schema_conformance`
2. `cargo test --locked -p tracewake-content --test forbidden_content && cargo test --locked -p tracewake-content --test golden_fixtures_run`
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`

## Outcome

Completed: 2026-06-18

Filled the SPINE-05 section of the acceptance report with manifest identity,
fingerprint stability/sensitivity, schema-version rejection, replay-package
content-manifest identity, and adversarial mismatch evidence. Recorded the
conditional snapshot resolution explicitly: snapshot-assisted load is not
applicable because no production snapshot subsystem exists at this commit.
Named the current save-manifest gap: certification evidence covers
manifest-bound replay packages and event-log ancestry, not a general persisted
save package with snapshot ancestry pointers.

Verification:

1. `cargo test --locked -p tracewake-content --test fixtures_load` passed.
2. `cargo test --locked -p tracewake-content --test schema_conformance` passed.
3. `cargo test --locked -p tracewake-content --test forbidden_content` passed.
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed.
5. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passed.
