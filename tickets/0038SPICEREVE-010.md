# 0038SPICEREVE-010: Mutation posture — Wave A continuity + Wave B SPINE-seam expansion + survivor triage register

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — creates the SPINE-CERT mutation triage register and runs cargo-mutants; changes no engine code (any production remediation a survivor demands is routed out).
**Deps**: 0038SPICEREVE-001

## Problem

SPINE-CERT inherits the 0037 posture that mutation evidence is part of certification, but it cannot reuse the 0037 mutation perimeter as complete coverage: the configured `.cargo/mutants.toml` guarded layer is focused on a P0/ordinary-life subset and excludes events, replay, save/content-manifest, TUI/debug, and no-direct-dispatch files the spine also covers (spec §7). This ticket runs the two required mutation waves, creates the survivor triage register, and writes the §9.6 mutation package into the acceptance artifact created by `-001`.

## Assumption Reassessment (2026-06-16)

1. The configured mutation perimeter exists and was verified during this session's `/reassess-spec` (2026-06-16): `.cargo/mutants.toml` (`exclude_globs` exclude `crates/tracewake-content/**`, `crates/tracewake-tui/**`, `crates/tracewake-core/src/events/**`, `replay/**`, `epistemics/**`, `checksum.rs`, `view_models.rs`, `debug_capability.rs`, `debug_reports.rs`, and more), and `.cargo/mutants-baseline-misses.txt`. The Wave A `-f` command in spec §7.1 reproduces the CI scheduled lock-layer job (`.github/workflows/ci.yml` lines 187–196) **verbatim**, and `cargo-mutants 27.1.0` matches CI (lines 133/183) and the 0037 register.
2. The Wave B inclusion list (spec §7.1) names files that all exist (verified): `crates/tracewake-core/src/events/**`, `replay/**`, `projections.rs`, `checksum.rs`, `actions/{pipeline,proposal,report}.rs`, `scheduler.rs`, `view_models.rs`, `debug_capability.rs`, `debug_reports.rs`, `epistemics/knowledge_context.rs`, `crates/tracewake-content/src/{manifest,load,schema,serialization,validate}.rs`, `crates/tracewake-tui/src/{app,debug_panels,render,transcript}.rs`. The register target `reports/0038_spine_cert_mutation_triage_register.md` does not exist (`test -f` → absent, 2026-06-16); its parent `reports/` exists; `reports/0037_mutation_triage_register.md` exists as the structure precedent (spec §7.2).
3. Shared boundary under audit: the mutation perimeter vs the SPINE seam set. The register must record the Wave A and Wave B command transcripts + exit status, baseline misses, and per-survivor entries (mutated file/function, responsible SPINE seam, responsible failure-diagnostic layer, behavior witness, why tests missed it, required remediation test/fixture, any production remediation, equivalent-mutant claim + evidence, disposition).
4. `INV-018` (deterministic replay is foundational) motivates this ticket: mutation testing is how the spine's determinism/event/replay/dispatch guards are proven to actually catch regressions rather than merely pass. The spec forbids a SPINE-CERT pass with an untriaged survivor, a survivor that changes spine behavior, or an equivalent-mutant assertion lacking behavioral + doctrine evidence (spec §7.2).
5. This ticket audits the determinism/event/replay/no-leak guard surfaces via mutation as an **evidence consumer + register author**: it runs cargo-mutants over Wave A and Wave B and records survivors; it weakens no enforcement and authors no production code. If a Wave B file is silently excluded by the cargo-mutants configuration, that is a certification failure (spec §7.1) — the ticket must provide a SPINE-CERT-specific config that includes it or mark the seam as scoped remediation; the artifact records the statement that no Wave B seam was silently excluded.

## Architecture Check

1. Running Wave A (existing guarded continuity) and Wave B (SPINE-seam expansion) as distinct, fully-transcribed waves with a triage register is cleaner than reusing the narrow 0037 perimeter: it makes the coverage gap explicit and forces every event/replay/save/TUI/dispatch survivor to a recorded disposition rather than a silent exclusion.
2. No backwards-compatibility aliasing or shims. The register and any SPINE-CERT-specific cargo-mutants config are new; they introduce no alternate path to existing code.

## Verification Layers

1. `INV-018` determinism guards caught by mutation -> mutation evidence: Wave A reproduces the CI guarded result; Wave B mutates the spine seam set and every survivor is triaged.
2. No-silent-exclusion -> codebase grep-proof + manual review: the Wave B command/config is recorded verbatim and cross-checked against the inclusion list; any `.cargo/mutants.toml` exclusion of a Wave B file is either overridden by a SPINE-CERT config or marked scoped remediation.
3. Triage completeness -> manual review: the register carries one entry per survivor with seam + responsible layer + disposition, mirroring `reports/0037_mutation_triage_register.md`.

## What to Change

### 1. Run the two mutation waves and capture transcripts

Run Wave A (the spec §7.1 verbatim `-f` guarded command, `cargo-mutants 27.1.0`) and Wave B (the SPINE-CERT inclusion list, via a SPINE-CERT-specific cargo-mutants config or explicit `-f` set), capturing each command, exit status, and output-artifact location. Confirm and record that no Wave B file is silently excluded by the active cargo-mutants configuration.

### 2. Create the survivor triage register and write the §9.6 mutation package

Create `reports/0038_spine_cert_mutation_triage_register.md` using the §7.2 / 0037 structure (status, version, baseline misses, command transcript, per-survivor entries). Write the §9.6 mutation package into the acceptance artifact (Wave A command/output/survivor status; Wave B command/config/output/survivor status; link to the register; explanation for every missed/timeout/unviable/equivalent mutant; the no-silent-exclusion statement).

## Files to Touch

- `reports/0038_spine_cert_mutation_triage_register.md` (new)
- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001)
- a SPINE-CERT-specific cargo-mutants config (new — only if `.cargo/mutants.toml` cannot be reused without silently excluding a Wave B file; named in the register/artifact transcript)

## Out of Scope

- The §4 cross-seam command transcript and §9.3 environment (owned by `-001`).
- The compile-fail/type-boundary no-direct-dispatch closure (owned by `-009`, SPINE-08) — this ticket records the cargo-mutants survivor analysis, not the static negative-fixture closure.
- Production remediation of any spine-behavior-changing survivor — recorded in the register with seam + responsible layer and routed to a separate `SPINE-CERT scoped remediation` ticket/spec; a scoped-remediation posture is a blocking state, not a pass (spec §8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo install cargo-mutants --version 27.1.0 --locked` then Wave A (spec §7.1 verbatim command) — transcript + exit status captured; result matches the CI guarded baseline (or new misses triaged).
2. Wave B (SPINE-CERT inclusion list) — transcript + exit status captured; every survivor recorded in the register.
3. `cargo test --workspace --locked` — green tree before mutation runs (mutation evidence is meaningless on a red tree).

### Invariants

1. No SPINE-CERT pass is claimable with an untriaged survivor, a spine-behavior-changing survivor, or an equivalent-mutant assertion lacking behavioral + doctrine evidence (spec §7.2).
2. No Wave B seam file is silently excluded by the cargo-mutants configuration; the no-silent-exclusion statement is recorded (`INV-018`; spec §7.1).

## Test Plan

### New/Modified Tests

1. `reports/0038_spine_cert_mutation_triage_register.md` — new triage register (the deliverable), mirroring `reports/0037_mutation_triage_register.md` structure.

### Commands

1. `cargo install cargo-mutants --version 27.1.0 --locked` (version-pin install — dry-run-resolvable; matches CI)
2. `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle` (Wave A — full mutation run, not cheaply dry-runnable; command resolve-verified verbatim against `.github/workflows/ci.yml`)
3. `cargo mutants --workspace <SPINE-CERT Wave B inclusion list per spec §7.1> --no-shuffle` (Wave B — full mutation run; inclusion files resolve-verified to exist)
