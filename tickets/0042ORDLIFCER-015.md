# 0042ORDLIFCER-015: §7 mutation posture — target-baseline continuity, configured ORD-LIFE expansion, and survivor triage register

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — modifies the checked-in mutation perimeter `.cargo/mutants.toml` (and `.cargo/mutants-baseline-misses.txt` as the configured union shifts the accepted baseline); creates `reports/0042_ord_life_cert_mutation_triage_register.md`. No simulation/production logic changes.
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §7 makes mutation testing a certification requirement for the guarded ordinary-life layer, not an optional score: a baseline-only or in-diff-only run is phase-entry evidence, not a substitute for the complete configured perimeter. The standing `.cargo/mutants.toml` covers broad `agent/**`, scheduler, projections, pipeline, etc., but static inspection does not establish complete ORD-LIFE coverage — the final census must explicitly demonstrate inclusion of `need_accounting.rs` and every ordinary action/routine definition used by the gate. This ticket expands the configured perimeter to the §7.2 union, runs the standing-continuity + expansion census, and produces the survivor triage register.

## Assumption Reassessment (2026-06-20)

1. The mutation surfaces exist at `98dc042`: `.cargo/mutants.toml` (`examine_globs`), `.cargo/mutants-baseline-misses.txt`, and the CI jobs `mutants-in-diff` / `mutants-lock-layer` in `.github/workflows/ci.yml` (verified). The CI jobs run `cargo mutants --in-diff …` and `cargo mutants --workspace --no-shuffle` — the perimeter is defined by `.cargo/mutants.toml`, NOT a narrower file list in `ci.yml`; therefore `ci.yml` is **not** modified (the reference's "CI mutation job only if it pins a narrower file list" condition does not fire).
2. Spec §7 (target-baseline finding, required configured union, required commands/waves, required mutation properties, survivor triage register, verdict routing) governs this ticket; §7.6: `ORD-LIFE-CERT passed` is impossible while any in-perimeter mutant remains actionable/untriaged/pending/timed-out, and a completed survivor floor yields `ORD-LIFE-CERT scoped remediation` routed to a later separate spec.
3. Cross-artifact shared boundary under audit: the mutation package section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`) plus the new `reports/0042_ord_life_cert_mutation_triage_register.md`, which reuses the historical `archive/reports/0040_epi_cert_mutation_triage_register.md` shape (`## Summary / Evidence Artifacts / Fingerprints / Register`).
4. Motivating discipline (spec §7): mutation asks whether injected behavior changes are detected, not whether lines execute; a mutant is not closed merely because it lies in tested code. `INV-018` (deterministic replay) underpins the `--no-shuffle` reruns. Restate before trusting the narrative: the configured union must cover the gate's behavioral branches, and every caught mutant family ties to a behavior witness/property that would fail under the mutation.
5. Enforcement surface this ticket touches (config + evidence only): the standing mutation perimeter and accepted baseline. Changing `examine_globs` adds no production code path and no nondeterminism; deterministic `--no-shuffle` reruns preserve replay-stable evidence. The §7.2 file-union is a quantifier enumerated in Acceptance Criteria; survivor *remediation* is routed out, not performed here.

## Architecture Check

1. Expanding the configured perimeter in `.cargo/mutants.toml` (the single perimeter source both CI jobs consume via `--workspace`) — rather than pinning a file list in `ci.yml` — keeps one authoritative perimeter and avoids perimeter drift between local and CI runs.
2. No backwards-compatibility aliasing/shims introduced; this changes mutation config + evidence artifacts only. No production logic changes.

## Verification Layers

1. Configured-union completeness -> codebase grep-proof (`cargo mutants --workspace --list-files` / `--list` census shows the §7.2 union, explicitly including `need_accounting.rs` and every ordinary action/routine definition; included/excluded files + skipped functions + baseline exclusions recorded with rationale).
2. Behavior-linked kill -> replay/golden-fixture check + manual review (each caught mutant family ties to a behavior witness or property that would fail under the mutation; equivalent/unreachable dispositions carry a proof specific to the mutated semantics).
3. `INV-018` deterministic rerun -> manual review (`--no-shuffle` runs against a stable baseline; timeouts/tool-failures/flakes are deterministically rerun with all original evidence retained).

## What to Change

### 1. Expand the configured mutation perimeter to the §7.2 union

Update `.cargo/mutants.toml` `examine_globs` so the semantic union covers at least: `crates/tracewake-core/src/agent/**`, `need_accounting.rs`, `scheduler.rs`, `projections.rs`, `actions/pipeline.rs`, `actions/proposal.rs`, `actions/report.rs`, `actions/registry.rs`, `actions/defs/need_events.rs`, `actions/defs/{eat,sleep,work,wait,continue_routine,movement}.rs`, `events/**`, `replay/**`, `checksum.rs`, `state.rs`. Update `.cargo/mutants-baseline-misses.txt` only as the configured union legitimately shifts the accepted normalized baseline. Record included and excluded files, generated mutant count, skipped functions/reasons, baseline exclusions, and why each exclusion does not remove a gate-4 behavioral branch.

### 2. Run the §7.3 waves and produce the triage register

Run, in order: unmutated workspace + named-suite baseline; committed in-diff/ratchet job (supporting evidence); complete configured ORD-LIFE file + mutant census; complete configured `--no-shuffle` run against a stable baseline; deterministic rerun of any timeout/tool-failure/flaky/ambiguous result; final survivor classification + behavior-witness closure. Create `reports/0042_ord_life_cert_mutation_triage_register.md` (0040 shape) with one row per survivor recording stable+final mutant identity, file/function/operator/span/tool-version, tool outcome + command/transcript, mapped ORD-LIFE-01…12 point, responsible layer, reachability + fixture/property family, behavior witness + replay/provenance evidence, live negative or why none can exist, disposition (caught / rigorously equivalent-unreachable / actionable survivor / timeout-tool-failure / pending), and evidence item IDs. Record the §7.4 required mutation properties and route the verdict per §7.6.

## Files to Touch

- `.cargo/mutants.toml` (modify)
- `.cargo/mutants-baseline-misses.txt` (modify — only as the configured union shifts the accepted normalized baseline)
- `reports/0042_ord_life_cert_mutation_triage_register.md` (new)
- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — §7 mutation package section; file created by 0042ORDLIFCER-001)

## Out of Scope

- `.github/workflows/ci.yml` — not modified; both CI jobs consume the `.cargo/mutants.toml` perimeter via `cargo mutants --workspace`, pinning no narrower file list.
- Survivor *remediation* / production fixes — a completed actionable survivor floor yields `ORD-LIFE-CERT scoped remediation` routed to a later separately-numbered mutation-remediation/replacement spec (spec §7.6); this ticket records the posture, it does not fix.
- The aggregate verdict (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants --workspace --list-files` and `cargo mutants --workspace --list` show the §7.2 configured union, explicitly including `need_accounting.rs` and every ordinary action/routine definition used by the gate; included/excluded files and skipped functions are recorded with rationale.
2. `cargo mutants --workspace --no-shuffle` completes against a stable baseline; every in-perimeter mutant is caught or carries a triage-register disposition (no actionable, pending, untriaged, or timed-out-without-closure floor left unrecorded), with each caught family tied to a behavior witness/property.
3. `reports/0042_ord_life_cert_mutation_triage_register.md` exists in the 0040 register shape with the per-survivor fields, and `cargo test --workspace --locked` remains green after the config change.

### Invariants

1. Configured perimeter, not a score: the certifying evidence is the complete configured `.cargo/mutants.toml` union run; a baseline-only or in-diff-only run is phase-entry evidence, not a pass.
2. No silent equivalence: equivalent/unreachable dispositions carry a proof specific to the mutated semantics; a comment, low risk, or high aggregate score is insufficient; survivor remediation is routed out, not performed here.

## Test Plan

### New/Modified Tests

1. `None — mutation-posture/config + evidence ticket; no unit/integration test is added. Verification is the cargo-mutants census + run commands below plus the green workspace suite.`

### Commands

1. `cargo install cargo-mutants --version 27.1.0 --locked`
2. `cargo mutants --workspace --list-files`
3. `cargo mutants --workspace --list`
4. `cargo mutants --workspace --no-shuffle`
5. `cargo test --workspace --locked`
