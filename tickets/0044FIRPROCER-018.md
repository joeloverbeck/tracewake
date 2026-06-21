# 0044FIRPROCER-018: §10 mutation posture — census, standing perimeter, focused FIRST-PROOF wave, and survivor triage register

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (conditional) — modifies `.cargo/mutants.toml` only if the `--list-files` census shows a §10.2 minimum-union file is absent from `examine_globs`; otherwise None. Creates mutation output artifacts + survivor triage register. A configuration change becomes part of `U` (spec §10.1).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §10 makes mutation testing a certification requirement for the guarded first-proof integration layer at `U`. Two waves are required: (1) the standing checked-in workspace configuration, to prove no cross-gate regression in the protected layer; and (2) a focused FIRST-PROOF wave emphasizing the genuinely new missing-property path and its integration carriers. The `--list-files`/`--list` census — not glob assumptions — determines coverage; if a §10.2 minimum-union file is absent, the session must make the checked-in perimeter honest **or** run an explicit focused file selection and record why (spec §10.1). This ticket runs the census + both waves under command supervision, produces the complete result package and survivor triage register, and records the §12.6 mutation package. Survivor remediation is routed out, not performed here.

## Assumption Reassessment (2026-06-21)

1. The standing config `.cargo/mutants.toml` exists at `U` and pins `cargo-mutants 27.1.0` with an `examine_globs` perimeter; the §10.2 minimum semantic union enumerates the live carriers that census must include. Grepping the standing `examine_globs` this session shows two §10.2 files are **not** covered by the globs: `crates/tracewake-core/src/time.rs` (in neither the standing globs nor the §10.3 focused `-f` list) and `crates/tracewake-core/src/actions/defs/checkcontainer.rs` (present in the §10.3 focused `-f` wave, absent from the standing globs). The `--list-files` census at `U` is authoritative and must be reconciled against the full §10.2 union member-by-member.
2. Spec §10 (scope principle, minimum semantic union, exact commands, completion/triage) and §12.6 (command/mutation package) govern this ticket; the CI mutation jobs in `.github/workflows/ci.yml` load `.cargo/mutants.toml` rather than pinning their own `-f` list (per the config's own comment), so a perimeter change flows through `mutants.toml` and `ci.yml` is **not** in this ticket's Files to Touch.
3. Cross-artifact shared boundary: this ticket creates the mutation outputs + `reports/0044_first_proof_cert_mutation_triage_register.md` and appends the §12.6 mutation-package section to `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`).
4. Motivating rule/invariants (spec §10): mutation is a certifying requirement, not a quality score, and cannot be replaced by line coverage, a baseline file, an in-diff run, or predecessor results; `INV-098` (feature acceptance is harsh — regression-tested) and `INV-018` (the mutated carriers include replay/determinism surfaces) ground it. Restate before trusting the narrative: `missed`, unresolved `timeout`, tool failure, incomplete campaign, lost output, irreconcilable count, or an actionable survivor prevents `FIRST-PROOF-CERT passed`.
5. This ticket touches the standing mutation perimeter (a fail-closed/determinism enforcement surface): expanding `examine_globs` to cover an absent §10.2 file makes the perimeter honest and introduces no leakage/nondeterminism path — it only widens which carriers are mutated; it adds no production code path. **Implementer-recorded choice (spec §10.1, not a defect)**: for each §10.2 file absent from the standing census, either (i) add it to `.cargo/mutants.toml examine_globs` (the make-honest option, becomes part of `U`) or (ii) run an explicit focused `-f` selection and record why it is not in the standing config — record the chosen option and rationale per file in the triage register.

## Architecture Check

1. Reconciling the full §10.2 minimum union against the live `--list-files` census member-by-member (rather than trusting the glob) is the only way to prove the certifying perimeter is honest; the standing-config gap for `time.rs`/`checkcontainer.rs` must be resolved by the recorded make-honest-or-focused-select choice, and the exact final command recorded — no silent smaller substitution.
2. No backwards-compatibility aliasing/shims introduced. A survivor may not be waived by asserting a predecessor gate once passed; a mechanically unviable mutant may be classified only with concrete compile/tool evidence.

## Verification Layers

1. `INV-098` harsh regression coverage -> mutation census + result reconciliation (the `--list-files`/`--list` census covers the full §10.2 union; standing + focused campaigns complete; counts reconcile across `mutants.json`/`outcomes.json`/`caught.txt`/`missed.txt`/`timeout.txt`/`unviable.txt`/`debug.log`).
2. `INV-018` replay/determinism carriers mutated -> mutation result review (the configured union includes the replay/projection/checksum/event carriers; every non-caught outcome records the distinguishing behavior, reachability, classification with tool/source evidence, disposition, and whether it blocks certification).
3. Perimeter honesty -> codebase grep-proof + manual review (each §10.2 file absent from the standing census is resolved by a recorded make-honest config edit or a focused `-f` selection with rationale; the exact final focused command is recorded).

## What to Change

### 1. Run the census and reconcile the §10.2 minimum union

Run `cargo mutants --workspace --no-shuffle --list-files > reports/0044_first_proof_cert_mutation_list_files.txt` and `cargo mutants --workspace --no-shuffle --list > reports/0044_first_proof_cert_mutation_list.txt`. Reconcile the full §10.2 minimum-union list member-by-member against `--list-files`. For each absent member (at minimum `time.rs` and `checkcontainer.rs` per the standing-glob gap), record the implementer choice (make-honest `examine_globs` edit vs focused `-f` + rationale) in the triage register; if making the perimeter honest, edit `.cargo/mutants.toml` and record that the resulting commit becomes `U`.

### 2. Run both waves under supervision and produce the package + register

Run the complete checked-in perimeter (`cargo mutants --workspace --no-shuffle -o reports/0044_first_proof_cert_mutation_full.out`) and the focused new-surface wave (`cargo mutants --workspace --no-shuffle -f <contradiction.rs/projection.rs/perception.rs/checkcontainer.rs/apply.rs/rebuild.rs/validate.rs and any additional integration carriers from the §6.7 census> -o reports/0044_first_proof_cert_mutation_focused.out`) under `tools/supervise-command.sh`. Package `mutants.json`, `outcomes.json`, `caught.txt`, `missed.txt`, `timeout.txt`, `unviable.txt`, `debug.log`, command transcripts, supervisor metadata, source/config hashes, and the exact cargo-mutants version. Create `reports/0044_first_proof_cert_mutation_triage_register.md` (reusing the 0040/0043 register shape): every non-caught outcome records stable mutant identity, exact path/line/symbol/operator, campaign/command, audit points/gates/families/temporal source affected, reproduction command, distinguishing behavior, reachability/assertion, evidence-backed classification, disposition/owner, linked killing evidence or unviability reason, and whether it blocks certification. Append the §12.6 mutation-package section to the acceptance artifact.

## Files to Touch

- `.cargo/mutants.toml` (modify — only if the `--list-files` census shows a §10.2 minimum-union file absent from `examine_globs`, e.g. `time.rs`/`checkcontainer.rs`; the make-honest option per spec §10.1)
- `reports/0044_first_proof_cert_mutation_list_files.txt` (new)
- `reports/0044_first_proof_cert_mutation_list.txt` (new)
- `reports/0044_first_proof_cert_mutation_full.out` (new — campaign output directory)
- `reports/0044_first_proof_cert_mutation_focused.out` (new — campaign output directory)
- `reports/0044_first_proof_cert_mutation_triage_register.md` (new)
- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — §12.6 mutation-package section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation / survivor kills (spec §10.4: a survivor floor yields `FIRST-PROOF-CERT scoped remediation` routed to a later separately-numbered mutation-remediation/replacement spec; this ticket records the posture, it does not fix and does not pre-author that spec).
- Editing `.github/workflows/ci.yml` or `.cargo/mutants-baseline-misses.txt` (the CI jobs load `mutants.toml`; baseline edits are a remediation concern, not this audit).
- The per-point evidence (`-002`..`-017`) and the aggregate verdict (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants --workspace --no-shuffle --list-files` and `--list` produce a census reconciled member-by-member against the §10.2 minimum union; every absent member carries a recorded make-honest config edit or focused `-f` selection with rationale.
2. The standing-perimeter and focused-wave campaigns complete under supervision with all result files present and counts reconciled across `mutants.json`/`outcomes.json`/`caught.txt`/`missed.txt`/`timeout.txt`/`unviable.txt`; the supervisor transcript distinguishes normal exit, nonzero exit, timeout/stall, and supervisor failure.
3. The triage register records every non-caught outcome with stable identity, operator, reproduction command, classification with tool/source evidence, disposition, and whether it blocks certification; a mechanically-unviable mutant is classified only with concrete compile/tool evidence.

### Invariants

1. The configured certifying perimeter is honest: every §10.2 minimum-union member is in the census or carries a recorded focused-selection rationale; the exact final focused command is recorded with no silent smaller substitution.
2. `missed`, unresolved `timeout`, tool failure, incomplete campaign, lost output, irreconcilable count, or an actionable survivor is recorded as blocking; no survivor is waived by citing a predecessor pass.

## Test Plan

### New/Modified Tests

1. `None — mutation-evidence ticket; the verification surface is the cargo-mutants census + campaigns and the reconciled result package, not unit tests. Any `.cargo/mutants.toml` perimeter edit is config, not production logic.`

### Commands

1. `cargo mutants --workspace --no-shuffle --list-files > reports/0044_first_proof_cert_mutation_list_files.txt`
2. `cargo mutants --workspace --no-shuffle --list > reports/0044_first_proof_cert_mutation_list.txt`
3. `tools/supervise-command.sh cargo mutants --workspace --no-shuffle -o reports/0044_first_proof_cert_mutation_full.out`
4. `tools/supervise-command.sh cargo mutants --workspace --no-shuffle -f 'crates/tracewake-core/src/epistemics/contradiction.rs' -f 'crates/tracewake-core/src/epistemics/projection.rs' -f 'crates/tracewake-core/src/agent/perception.rs' -f 'crates/tracewake-core/src/actions/defs/checkcontainer.rs' -f 'crates/tracewake-core/src/events/apply.rs' -f 'crates/tracewake-core/src/replay/rebuild.rs' -f 'crates/tracewake-content/src/validate.rs' -o reports/0044_first_proof_cert_mutation_focused.out`
