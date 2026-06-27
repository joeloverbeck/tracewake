# 0053FOUCONFIF-010: Acceptance capstone — fail-closed status manifest and computed verdict

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — acceptance-only verdict artifact (`reports/0053_foundational_conformance_fifth_hardening_acceptance.md`); no new production logic
**Deps**: 0053FOUCONFIF-008, 0053FOUCONFIF-009

## Problem

Spec 0053 §8: the implementing session must assemble the acceptance artifact, carry the §4.6 machine-readable status manifest, record per-finding closure with real production-path evidence, record the required-check names and a branch-protection/ruleset API transcript, and **must not render `pass` unless the manifest computes pass** — i.e. unless every required finding is `closed`, governance is enforced, and the §5 `food_source` family is killed or under a bounded forcing function. This is the acceptance-only capstone: it exercises every prior ticket end-to-end and renders the single computed verdict; it introduces no new production logic.

## Assumption Reassessment (2026-06-26)

1. The repo convention for acceptance artifacts is `reports/<NNNN>_<slug>_acceptance.md` (e.g. `archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md`), archived to `archive/reports/` on spec acceptance per `docs/archival-workflow.md`. `reports/` exists (verified this session). The §4.6 manifest consumer is ticket 001's `crates/tracewake-core/tests/acceptance_status_manifest.rs`; this artifact carries the manifest block that test validates and recomputes.
2. Spec authority: `specs/0053_…_HARDENING_SPEC.md` §8 (acceptance artifact + manifest + computed pass), §4.6 (manifest schema + computed rule), §4.6.5 (independent/adversarial posture — an implementation session may say "implementation evidence collected" but not "foundational pass" unless the manifest computes pass), §9 step 7. Follows `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
3. Cross-artifact boundary under audit: the boundary between this acceptance artifact's status block and the Rust consumer (`acceptance_status_manifest.rs`, ticket 001), and between the recorded denominators (ticket 009) / governance API transcript (ticket 003 operational evidence) and the manifest's computed fields. The leaf set {008, 009} transitively covers 001–007.
4. Motivating invariant: INV-098 (harsh acceptance) and the INV-098-class evidence discipline — `pass` is legal only when computed from certifying current evidence; `pending`, `routed-forward`, `historical`, `sampled`, `observer-only`, and "pass with disposition" are not pass.
5. This ticket *audits/re-proves* the resealed enforcement surfaces (evidence-consumer basis): it names them (sealed bootstrap, sealed interval product, token-gated debug command/receipt, killed `food_source` family, merge-enforced governance) and confirms the artifact introduces no leakage/nondeterminism path — its evidence is retrospective and observer-only (debug rows are non-diegetic and feed no simulation behavior). It computes the verdict from the typed status set; it asserts no `pass` in prose ahead of the computed result (§4.6.5).

## Architecture Check

1. An acceptance-only capstone that computes the verdict from the manifest is cleaner than a prose verdict: it makes the §4.6 fail-closed taxonomy the single source of truth, so a dishonest "scoped pass" cannot be written (the wording guard + manifest consumer from 001 reject it). The capstone exercises the pipeline the prior tickets composed; it adds no production logic.
2. No backwards-compatibility aliasing or shims (acceptance-only). It edits no archived spec/ticket/report (§1.2); the `archive/reports/` move happens at spec acceptance.

## Verification Layers

1. INV-098 (harsh acceptance) -> manifest computation: `acceptance_status_manifest.rs` (001) recomputes the artifact's overall result and asserts it equals the stated verdict; `pass` only if every required finding is `closed`, governance enforced, survivors killed.
2. Per-finding closure -> production-path evidence rows: each of F5-01…F5-06 carries a sealed-constructor / public-command-or-token / observed-effect / sensitivity-or-negative-compile-proof row (not display strings, artifact existence, or historical results).
3. Governance enforcement -> API transcript: the required-check names plus the branch-protection/ruleset API transcript (ticket 003 operational evidence) are recorded; a YAML grep or screenshot is insufficient (§4.4).
4. Capstone exercises, does not modify: Files to Touch is the acceptance artifact only; it does not list or edit the upstream tickets' files.

## What to Change

### 1. Assemble the acceptance artifact (`reports/0053_foundational_conformance_fifth_hardening_acceptance.md`, new)

Following `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`: begin from a clean baseline; name the exact implementation commit (not the proposal baseline `e9792dc`); record the workspace gate results (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`); record the standing mutation denominator disposition from ticket 009; carry the §4.6 machine-readable status manifest.

### 2. Per-finding closure rows

For F5-01…F5-06 record real production-path evidence: which sealed constructor/token created the runtime / crossed the boundary, what state/event/projection effect was observed, and which deliberate mutation or negative-compile attempt proves sensitivity. Record the required-check names and the branch-protection/ruleset API transcript (operational evidence from ticket 003).

### 3. Computed verdict

Render the overall result **computed** from the manifest status set. State "implementation evidence collected" framing unless the manifest computes `pass`; do not render foundational pass in prose ahead of the computed result (§4.6.5).

## Files to Touch

- `reports/0053_foundational_conformance_fifth_hardening_acceptance.md` (new)

## Out of Scope

- The manifest *mechanism* / wording guard (ticket 001) — this ticket produces the artifact the mechanism validates.
- The `docs/4-specs/SPEC_LEDGER.md` row + `archive/specs/` / `archive/reports/` moves — deferred to spec acceptance/archival (a Step 6 cross-spec follow-up).
- Any production `src/` change or upstream-ticket file edit (the capstone exercises, it does not modify).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test acceptance_status_manifest` — the artifact's status block parses, and the recomputed overall result equals the stated verdict (fail-closed: a contradictory block fails).
2. `cargo test -p tracewake-core --test acceptance_artifact_wording` — the artifact uses no closure language inconsistent with its status block (no "scoped pass" while items open; no green-perimeter claim while survivors remain).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — the gate results recorded in the artifact are reproduced.
4. The recorded branch-protection/ruleset API transcript shows the named required checks are merge-required (operational evidence; §4.4).

### Invariants

1. The overall verdict is computed from the manifest status set; no prose `pass` precedes a computed pass (INV-098 / §4.6.5).
2. Every `closed` finding carries a certifying production-path evidence item and a live negative/sensitivity proof; no display string / artifact existence / historical result is cited as sole evidence.

## Test Plan

### New/Modified Tests

1. `None — acceptance-only capstone; it adds no production logic. Verification is the manifest consumer + wording guard (ticket 001) run against this artifact, plus the workspace gates and the mutation denominators (ticket 009).`

### Commands

1. `cargo test -p tracewake-core --test acceptance_status_manifest --test acceptance_artifact_wording`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. The branch-protection API transcript is captured operationally (ticket 003's audit job / `gh api`), not via a local test command — its presence in the artifact is the verification surface.

## Outcome

Completed: 2026-06-26

Created `reports/0053_foundational_conformance_fifth_hardening_acceptance.md` as the capstone evidence artifact for spec 0053.

Important result:

- The artifact's machine-readable manifest computes `non-pass`, not `pass`.
- F5-01, F5-02, F5-03, F5-05, and F5-06 are recorded closed.
- F5-04 remains `pending-governance` because the live GitHub API transcript did not prove branch-protection/ruleset enforcement for `main`.

Governance transcript captured:

- `gh api repos/joeloverbeck/tracewake/branches/main/protection/required_status_checks` returned `Branch not protected (HTTP 404)`.
- `gh api repos/joeloverbeck/tracewake/rulesets --jq '.[] | {name, target, enforcement, conditions, rules: [.rules[]? | {type, parameters}]}'` returned no rows.

Mutation denominator recorded from ticket 009:

- Final `cargo mutants`: `3423 mutants tested in 4h: 2666 caught, 757 unviable`.
- Missed/survivor list: empty.
- Timeouts: none reported.
- Routed-forward residuals: none.

Verification:

- `cargo test -p tracewake-core --test acceptance_status_manifest --test acceptance_artifact_wording` passed.
- The full workspace gates were run after the final 009 repair and before this acceptance-only report was authored:
  - `cargo fmt --all --check` passed.
  - `cargo clippy --workspace --all-targets -- -D warnings` passed.
  - `cargo build --workspace --all-targets --locked` passed.
  - `cargo test --workspace` passed.

Deviation:

- The ticket expected a computed verdict only if the manifest could compute it. The live governance evidence prevents `pass`; the report therefore uses "implementation evidence collected" framing and records the blocking governance residual instead of overclaiming closure.
