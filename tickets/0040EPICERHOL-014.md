# 0040EPICERHOL-014: §7 mutation posture — Wave A continuity, Wave B EPI expansion, and triage register

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — modifies the checked-in mutation perimeter `.cargo/mutants.toml`; creates `reports/0040_epi_cert_mutation_triage_register.md`. No simulation/production logic changes.
**Deps**: 0040EPICERHOL-001

## Problem

Spec 0040 §7 makes mutation testing a certifying guarded-layer requirement for EPI-CERT. The standing `.cargo/mutants.toml` at the target commit names only `epistemics/knowledge_context.rs` and `epistemics/projection.rs` under `epistemics/`; the rest of the epistemic module set is outside the explicit standing EPI path (§7.1). This ticket runs Wave A (standing continuity census), expands the checked-in configuration to the full EPI union and runs Wave B (the certifying expansion census), records the §7.4 outcomes, creates the §7.5 survivor triage register, and writes the §9.6 mutation package into the artifact created by `-001`. It renders no aggregate verdict (the capstone `-015` does), but it determines the mutation-verdict routing input (§7.6).

## Assumption Reassessment (2026-06-19)

1. Verified at `ba9fe1c` (2026-06-19): `.cargo/mutants.toml` is a checked-in standing perimeter using `additional_cargo_args = ["--locked"]`, `test_workspace = true`, cargo-mutants `27.1.0` baseline; its `examine_globs` includes the complete `agent/**` plus the proposal/pipeline/event/replay/projection/view/controller/debug/TUI seams, but under `epistemics/` only `knowledge_context.rs` and `projection.rs`. The §7.2 EPI union therefore must **add 6 enumerated epistemics members** absent from the standing path: `epistemics/belief.rs`, `epistemics/observation.rs`, `epistemics/proposition.rs`, `epistemics/contradiction.rs`, `epistemics/knowledge_basis.rs`, `epistemics/mod.rs` (the `epistemics/**` glob covers all). `.github/workflows/ci.yml` verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its §9.6 section and creates the triage register `reports/0040_epi_cert_mutation_triage_register.md` `(new)` reusing the `reports/0039_spine_cert_mutation_triage_register.md` shape (verified present). Section wording follows spec §7/§9.6 and the `0003` evidence fields.
3. Shared boundary under audit: the checked-in mutation perimeter — `.cargo/mutants.toml` `examine_globs` as the single certifying configuration, with `cargo mutants --workspace --list-files` proving the union. The final certifying run must use one checked-in configuration (no `--no-config`/`-f` final pass, no `--iterate`, no skipped baseline). **Conditional (§7.2)**: if the code audit shows `input.rs`, `run.rs`, or another manifest-listed source owns a debug/possession capability or embodied-data transformation, it joins the configured union with the reason recorded; resolved at this ticket's code-audit step. Whether `.github/workflows/ci.yml` needs editing is likewise resolved then — modified only if the CI mutation job pins a narrower file list than the standing config.
4. `INV-098` (feature acceptance is harsh — regression-tested) motivates this audit point; the configured-union mutation census is the behavioral-guard proof that the EPI seams are not merely statically present. Existing non-EPI standing paths remain as continuity coverage (not removed).
5. This ticket audits the mutation guard of the actor-knowledge / replay / fail-closed-validation seams as an **evidence consumer**: expanding `examine_globs` changes no simulation logic and weakens no enforcement; it widens what is mutated. A behavior-relevant survivor defaults to kill-with-behavior/provenance coverage (the witness passes unmutated, fails under the mutant, executes the production seam, observes a certified consequence, carries replay/provenance + contamination control). An equivalent/non-critical disposition is rare and requires diff + reachability + semantic argument + independent signoff. A timeout/tool-failure/high-score is not a pass. An actionable survivor floor routes to a **separate** EPI-CERT mutation-remediation spec (§7.6) — not authored or fixed here.

## Architecture Check

1. Expanding the standing checked-in config (rather than a one-off `-f` list) is required because the final certifying run must use one configuration that CI also loads (§7.2); a narrower CI/standing config cannot coexist with a wider final pass. Wave A then Wave B on the same config detects regression relative to the predecessor perimeter and proves the EPI expansion separately.
2. No backwards-compatibility aliasing or shims; the config change is additive to `examine_globs` (existing non-EPI paths retained). No directory glob is narrowed to reduce runtime.

## Verification Layers

1. `INV-098` configured-union completeness -> codebase grep-proof + manual review: `cargo mutants --workspace --list-files` shows the full §7.2 EPI union including the 6 added epistemics members; no required EPI file is silently excluded; `.cargo/mutants.toml` diff is additive.
2. behavioral guard -> replay/golden-fixture check + manual review: the completed Wave B population reports caught/missed/timeout/unviable/tool-failure counts; every behavior-relevant survivor has a kill witness (passes unmutated, fails mutated, executes the seam, observes a certified consequence) or a rare reviewed equivalent disposition with independent signoff.
3. census honesty -> manual review: Wave A and Wave B file/mutant censuses retained; denominator changes from source/config/tool version explained; baseline-miss machinery did not launder an actionable EPI survivor; the final run did not use `--iterate` and did not skip the unmutated baseline.
4. `INV-098` triage completeness -> manual review: `reports/0040_epi_cert_mutation_triage_register.md` carries every final survivor/timeout row with the §7.5 fields (historical/final identity, tool outcome, responsible EPI cross-ref, responsible layer, reachability, test/property family, behavior witness, replay/provenance ancestry, negative/contamination control, disposition, evidence references, review signoff).

## What to Change

### 1. Run Wave A, expand the configuration, run Wave B

Run Wave A on the unmodified standing config (retain file/mutant census + full outcome). Resolve the §7.2 conditional via code audit (add `input.rs`/`run.rs`/other only if they own a debug/possession capability or embodied-data transform; record the reason or the negative finding). Edit `.cargo/mutants.toml` to make the full §7.2 EPI union visible — adding `crates/tracewake-core/src/epistemics/**` (covering the 6 currently-absent members) and confirming the proposal/pipeline/report/events/replay/checksum/state/projections/view_models/controller/debug/TUI entries are present. Prove the expanded `--list-files` union, then run the completed Wave B population (no `--iterate`, no skipped baseline; sharded equivalents only under the §7.3 identical-environment conditions).

### 2. Record §7.4 outcomes, build the triage register, write §9.6

Record configured-census files + required-file omissions (if any), enumerated/tested mutants, caught/missed/timeout/unviable/tool-failure counts, shard count/completeness, unresolved timeout retries + commands, denominator changes, baseline-miss use, and survivor dispositions. Create `reports/0040_epi_cert_mutation_triage_register.md` with the §7.5 row fields. Write the §9.6 mutation package (versions; final config + CI fingerprints; `--list-files`/`--list`; complete/sharded output; baseline output; counts; retries; register; no-silent-exclusion proof; `test_workspace`-active proof; no-`--iterate` statement; baseline-miss non-laundering statement). Determine the §7.6 routing input.

## Files to Touch

- `.cargo/mutants.toml` (modify — add the EPI union epistemics glob; existing non-EPI standing paths retained)
- `reports/0040_epi_cert_mutation_triage_register.md` (new — §7.5 survivor triage register, reusing the 0039 register shape)
- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; §9.6 mutation package section)
- `.github/workflows/ci.yml` (modify — conditional, only if the CI mutation job pins a narrower file list than the standing config; resolved at the code-audit step)

## Out of Scope

- Per-audit-point evidence sections and the §9.4 verdict/aggregate verdict (owned by their own tickets / `-015`).
- Authoring or performing the production fix for any actionable survivor floor — that routes to a **separate, later** EPI-CERT mutation-remediation and replacement-certification spec (§7.6); this ticket records the survivor posture and routing only.
- Removing or narrowing existing non-EPI standing perimeter paths (§7.2 forbids it).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants --workspace --list-files` shows the full §7.2 EPI union, including `epistemics/belief.rs`, `observation.rs`, `proposition.rs`, `contradiction.rs`, `knowledge_basis.rs`, `mod.rs` (via `epistemics/**`) — none silently excluded.
2. `cargo test --workspace --locked` baseline green (the unmutated baseline is not skipped); `cargo mutants --version` records `27.1.0` (or the recorded upgrade with before/after censuses).
3. Wave A and Wave B censuses retained with caught/missed/timeout/unviable/tool-failure counts; `reports/0040_epi_cert_mutation_triage_register.md` carries every final survivor/timeout row with all §7.5 fields.
4. The §9.6 mutation package is complete (no-silent-exclusion proof, `test_workspace`-active proof, no-`--iterate` statement, baseline-miss non-laundering statement).

### Invariants

1. One checked-in configuration is the certifying perimeter; the final run uses no `--no-config`/`-f`/`--iterate` and no skipped baseline; existing non-EPI paths are retained, not narrowed (`INV-098`, §7.2).
2. A timeout/tool-failure/high-score is not a pass; every behavior-relevant survivor is killed-with-witness or rarely dispositioned equivalent with independent signoff; an actionable floor routes to a separate remediation spec, never hidden in baseline-misses or a new glob (§7.4/§7.6).

## Test Plan

### New/Modified Tests

1. `.cargo/mutants.toml` — expanded `examine_globs` to the §7.2 EPI union (config, not a test); `reports/0040_epi_cert_mutation_triage_register.md` — survivor triage register (evidence artifact). Behavior-witness tests for any kill route to the separate remediation spec (§7.6), not this ticket.

### Commands

1. `cargo mutants --workspace --list-files` (prove the configured EPI union before the campaign)
2. `cargo test --workspace --locked` (unmutated baseline must be green before mutation interpretation)
3. `cargo mutants --workspace --no-shuffle` (the certifying Wave B census on the checked-in config; sharded equivalents only under §7.3 identical-environment conditions)
