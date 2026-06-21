# 0044FIRPROCER-001: Acceptance-artifact scaffold + baseline freeze, command ledger, and FIRST-PROOF-01

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — creates the acceptance-artifact scaffold and records baseline/command evidence. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, never production remediation).
**Deps**: None

## Problem

Spec 0044 commissions a non-executable FIRST-PROOF-CERT audit that must derive every certifying behavior, replay, fixture, diagnostic, content, TUI, and mutation result from one immutable **unified baseline commit** `U` (spec §2.4, §5.1). Before any per-point audit can record evidence, the session must (a) name `U` and freeze the evidence inputs, (b) capture the mandatory baseline + integrated command ledger (spec §5.2) with start/end time, exit status, stdout/stderr, tool versions, and working-directory identity, and (c) prove FIRST-PROOF-01: all nine first-proof gates and all nine scenario families are exercised against one baseline, one fixture/schema registry, one test corpus, one toolchain, and one mutation configuration, with `core <- content <- tui` mechanically confirmed and no evidence spliced across the four predecessor commits. This ticket creates the single acceptance artifact (the create-then-modify hub every later ticket appends to), lays out one empty section per audit point / gate / scenario family / temporal source, and records the §5.1 freeze, the §5.2 command ledger, the §6.7 census table, and the FIRST-PROOF-01 witnesses.

## Assumption Reassessment (2026-06-21)

1. The audited command/config surfaces exist at the target commit `1541da274180ecd40f52583d86704990cb55e74c` (confirmed = `git rev-parse HEAD` this session): `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/mutants.toml`, `.github/workflows/ci.yml`, `tools/supervise-command.sh`, and the full §5.2 named test corpus (core 13 / content 4 / tui 7 integration tests, all present and an exact match to the enumerated `--test` list — no silent discovery drift). The 59 fixture files under `crates/tracewake-content/src/fixtures/` match the §6.5 count.
2. Spec §2.4 (unified-baseline rule), §5.1 (final-baseline identity and evidence freeze), §5.2 (mandatory baseline and integrated commands), §5.3 (evidence honesty), §6 (verified audit inventory) and §6.7 (census output required at `U`), plus FIRST-PROOF-01 (spec §7) govern this ticket. The four consumed certifications and their exact commits (`P0-CERT` `a3b5e3c9…`, `SPINE-CERT` `92ba47f1…`, `EPI-CERT` `726b2a1f…`, `ORD-LIFE-CERT` `c819bbe…`) are admissibility evidence only (spec §1, §4) and verified against `docs/4-specs/SPEC_LEDGER.md` this session; they may not be spliced into `U`'s integrated result.
3. Cross-artifact shared boundary created here: the acceptance artifact `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (spec §12) — created `(new)` by this ticket with one empty anchor per FIRST-PROOF-01..17, per nine gate, per nine scenario family, and per five temporal source. Every sibling ticket (`-002`..`-017`, `-019`) appends only its own section; this ticket owns the scaffold, the §12.1 header/provenance, and the FIRST-PROOF-01 section.
4. Motivating invariants/rules (spec §7 FIRST-PROOF-01): `INV-018` (deterministic replay is foundational — one baseline, byte-identical reconstruction), the spec §2.4 no-splice unified-baseline rule, and architecture `01` one-way crate direction `core <- content <- tui`. Restate before trusting the narrative: one immutable `U`, one command set run against one source tree, no predecessor command output cited as current execution.
5. This ticket audits/reads (does not modify) the determinism/replay and cross-crate dependency enforcement surfaces. It records the baseline identity, the command ledger, and the cross-crate direction check; it introduces no nondeterministic input into any canonical form, adds no production code path, and any audit-only instrumentation stays observer-only per §5.4. Debug/observer rows recorded in the artifact are labeled non-diegetic and excluded from authoritative checksums (spec §5.3, §12.3).

## Architecture Check

1. A single scaffold ticket that creates the artifact and pre-sections it per audit point is the only structure that keeps the 16 downstream evidence appends conflict-light (disjoint anchors) while guaranteeing every gate/family/source has a home before evidence collection begins; spec §8 requires reconciled tables keyed to the same SHA, which is only honest if one ticket fixes `U` first.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No predecessor artifact may be promoted into `U`'s integrated result, and no aggregate fingerprint may substitute for the per-scenario ancestry FIRST-PROOF-01 requires.

## Verification Layers

1. `INV-018` deterministic baseline -> command-ledger + replay/golden-fixture check (`git rev-parse HEAD`, before/after `git status --porcelain=v1`, tool-version capture, and `cargo test --workspace --locked` reconciled to one SHA).
2. Spec §2.4 no-splice -> manual review + grep-proof (every recorded command's SHA equals `U`; no predecessor command transcript is cited as current execution; any audit-only delta is enumerated and the resulting commit recorded as `U`).
3. Architecture `01` crate direction -> codebase grep-proof + conformance check (`tui_seam_conformance`, dependency direction `core <- content <- tui` mechanically checked; no reverse dependency or TUI/core bypass).

## What to Change

### 1. Create and pre-section the acceptance artifact

Create `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` instantiating `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`. Populate the §12.1 header/provenance: spec path; repository; authoring target commit `1541da274180ecd40f52583d86704990cb55e74c`; exact unified code/evidence commit `U`; any later documentation-only closeout commit; clean-worktree evidence; toolchain + cargo-mutants versions; Cargo.lock/toolchain/workspace/CI/mutation-config/fixture-registry/test-source fingerprints; source-discipline statement; predecessor artifact paths/commits/scopes with the "consumed, not rerun as isolated subsystem audits" posture; the freshness statement (not independently verified as latest `main`); and the explicit archived-implementation-specs-are-historical statement. Lay out empty anchors for FIRST-PROOF-01..17, the nine gates, the nine scenario families, and the five temporal sources, plus the §12.3 evidence-item ledger, §12.5 replay package, §12.6 command/mutation package, §12.7 staged-abstraction declaration, and §12.8 EMERGE-OBS package.

### 2. Record the §5.1 freeze and §5.2 command ledger

Run the §5.1 sequence: record candidate `U` (`git rev-parse HEAD`); require an empty `git status --porcelain=v1` worktree except enumerated generated transcript/output directories; record `rustc --version --verbose`, `cargo --version --verbose`, `cargo mutants --version`; hash `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/mutants.toml`, `.github/workflows/ci.yml`, all audited fixture sources, and every named test binary source; freeze the audit inventory; then run the §5.2 mandatory baseline + integrated commands under `tools/supervise-command.sh` for stall-prone commands, distinguishing normal exit / nonzero exit / timeout-stall / supervisor failure; repeat commit/status/config hashes after the run. A changed hash, commit, test selection, or mutation configuration invalidates the corresponding evidence until rerun.

### 3. Record FIRST-PROOF-01 witnesses and the §6.7 census table

Record: one baseline identity + before/after clean-worktree record; the command ledger covering every mandatory command; all named tests discovered and run with no silent ignore/filter drift; the exact fixture registry + fingerprint inventory; the exact mutation file/mutant census (the §10.3 `--list-files` / `--list` outputs); the mechanically-checked cross-crate dependency direction; per-point evidence rows and completeness tables reconciled to the same SHA. Record the §6.7 census table mapping every §6 path to symbols, dependent audit points, mutation inclusion (or reason N/A), positive/adversarial fixtures, event kinds, projections/checksums, responsible diagnostic layer, and changed/unchanged status relative to the authoring commit. Record the FIRST-PROOF-01 adversarial guards (artifact guard rejects mixed SHAs / missing transcripts / stale fixture fingerprints / omitted named tests / predecessor-output-cited-as-current; dependency/conformance tests reject reverse dependency or TUI/core bypass; a post-command audited-hash change makes that command non-certifying until rerun).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (new)

## Out of Scope

- Production remediation of any defect (spec §2.4, §5.4, §11 route a substantive failed seam to a separate, later-numbered FIRST-PROOF-CERT remediation/replacement spec; record `fail` + responsible layer, do not fix).
- The per-point behavioral witnesses for FIRST-PROOF-02..17 (`-002`..`-017`), the §10 mutation posture (`-018`), and the aggregate verdict / reconciled completeness tables (`-019`).
- Latest-`main` verification (spec §14; out of scope for this workflow).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, and `cargo test --workspace --locked` all pass at one recorded `U`; the artifact records each command's exit status, timing, and tool versions.
2. `cargo test --locked -p tracewake-core --test acceptance_gates`, `--test anti_regression_guards`, `--test ci_workflow_guards`, `--test doc_invariant_references`, and `--test spine_conformance` pass; the artifact records the named-test census with no silent ignore/filter drift.
3. `cargo test --locked -p tracewake-content --test fixtures_load` and `cargo test --locked -p tracewake-tui --test tui_seam_conformance` pass; the cross-crate dependency direction `core <- content <- tui` is mechanically confirmed with no reverse dependency.

### Invariants

1. One immutable `U`: every certifying command, fixture fingerprint, and census entry is reconciled to one SHA; no predecessor command output is cited as current execution and no evidence is spliced across the four predecessor commits.
2. Deterministic baseline: identical inputs + versions reproduce the recorded workspace results; before/after worktree status is clean except enumerated generated output directories.

## Test Plan

### New/Modified Tests

1. `None — scaffold/evidence-only ticket; the §5.2 existing workspace + named suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `git rev-parse HEAD && git status --porcelain=v1`
2. `cargo fmt --all --check`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo build --workspace --all-targets --locked`
5. `cargo test --workspace --locked`
6. `cargo test --locked -p tracewake-core --test acceptance_gates --test anti_regression_guards --test ci_workflow_guards --test doc_invariant_references --test spine_conformance`
7. `cargo test --locked -p tracewake-content --test fixtures_load`
8. `cargo test --locked -p tracewake-tui --test tui_seam_conformance`
