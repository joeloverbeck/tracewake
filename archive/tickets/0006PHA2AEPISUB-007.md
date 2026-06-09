# 0006PHA2AEPISUB-007: Capstone — scoped acceptance artifact and workspace gates for Phase-2A epistemic hardening

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — verification-only capstone; produces a scoped acceptance-artifact document and runs the workspace + lock-layer gates end-to-end.
**Deps**: 0006PHA2AEPISUB-004, 0006PHA2AEPISUB-005, 0006PHA2AEPISUB-006

## Problem

The Phase-2A epistemic hardening (sealing in -001/-002/-003, lock-layer fixtures/guards in -004/-005, adversarial tests in -006) needs a single end-to-end acceptance pass that exercises the composed pipeline and records a scoped acceptance artifact with the exact-commit, no-overclaim wording the spec mandates (spec §9 acceptance checklist + §9.3 certification wording, §10 item 8). Without this capstone there is no consolidated evidence that the four workspace gates and the extended lock-layer suites all pass together on one commit, and no artifact stating the scoped `EPI-CERT`/`P0-CERT` contribution without overclaiming certification.

## Assumption Reassessment (2026-06-09)

1. The acceptance-artifact wording contract is enforced by `crates/tracewake-core/tests/acceptance_artifact_wording.rs`, which `include_str!`s `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (`:1`) and exposes `validate_acceptance_artifact_wording` (`:45`) requiring scoped exact-commit wording and forbidding overclaim phrases (`:18-43`). The template exists. There is no pre-existing on-disk acceptance-artifact location (none under `reports/`/`archive/`); the artifact is produced fresh recording the commit-under-review.
2. Spec authority: `specs/0006_…SPEC.md` §9.1 (four workspace gates), §9.2 (per-requirement evidence table), §9.3 (allowed/forbidden wording), §10 item 8. The four workspace gates are the project standard (`CLAUDE.md` build/test/lint block).
3. Shared boundary under audit: this capstone exercises — and must not modify — the sealed `KnowledgeContext`/`EpistemicProjection`/record APIs, the extended lock-layer suites (`negative_fixture_runner`, `hidden_truth_gates`, `event_schema_replay_gates`, TUI `adversarial_gates`, content `forbidden_content`), and the acceptance-wording validator. Its `Deps` are the leaf set {-004, -005, -006}, whose transitive deps reach -003 → -002 → -001, covering the full chain.
4. Constitutional invariants motivating this ticket: `INV-098` (feature acceptance is harsh — done only when caused, epistemically bounded, replay-safe, debug-inspectable, no-human-runnable, regression-tested) and the acceptance-gate discipline; the scoped-wording requirement enforces that this pass contributes evidence toward `EPI-CERT`/`P0-CERT` without claiming either gate passed.
5. Enforcement surface (verification-only): this capstone runs the deterministic-replay (`event_schema_replay_gates`), no-leak/actor-knowledge (`hidden_truth_gates`, `adversarial_gates`, negative fixtures), and provenance (`forbidden_content`) surfaces end-to-end. It introduces no production logic and cannot weaken any surface; it only proves the composed result.

## Architecture Check

1. A single trailing acceptance capstone keeps the spec's exit-criteria evidence in one place, mapped to the relevant gates, and produces the scoped artifact through the existing wording validator rather than inventing acceptance prose. It adds no production logic — it exercises the pipeline the prior tickets composed.
2. No backwards-compatibility shims; this ticket writes a documentation artifact and runs commands.

## Verification Layers

1. `INV-098` / acceptance-gate discipline -> manual review + command runbook: the four workspace gates plus the extended lock-layer suites all pass on the commit-under-review, recorded in the per-requirement evidence table.
2. `INV-018` (deterministic replay) -> replay/golden-fixture check: `event_schema_replay_gates` + projection-checksum tests green.
3. `INV-024`/`INV-068` (no leak / debug quarantine) -> lock-layer check: `negative_fixture_runner`, `hidden_truth_gates`, TUI `adversarial_gates` green.
4. Scoped-wording compliance -> the produced artifact passes `validate_acceptance_artifact_wording` (scoped exact-commit phrasing; no overclaim phrase from spec §9.3's forbidden list).

## What to Change

### 1. Run the acceptance runbook (implementer checklist)

Run, recording command + exact commit + result + concise summary for each:
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
- the lock-layer suites: `negative_fixture_runner`, `hidden_truth_gates`, `event_schema_replay_gates`, TUI `adversarial_gates`, content `forbidden_content`.

### 2. Produce the scoped acceptance artifact

Author the artifact using `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` wording, filling the per-requirement evidence table (EPI-HARD-001…004 + event/replay parity + content provenance parity + TUI/debug proof + consumer-seam firewall) with pass/fail and evidence. Use the allowed scoped wording from spec §9.3; include none of the forbidden overclaim phrases. Confirm the produced text passes `validate_acceptance_artifact_wording`.

## Files to Touch

- `reports/0006PHA2A_ACCEPTANCE_ARTIFACT.md` (new — **proposed location**; confirm against any acceptance-artifact placement convention the implementer finds, since no prior artifact exists on disk)
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs` (modify — optional: add an assertion that the produced artifact text passes the wording validator, only if checking the artifact into a test is desired)

## Out of Scope

- Any production logic or sealing change (owned by -001/-002/-003).
- Any new lock-layer fixture/guard/test (owned by -004/-005/-006).
- `docs/4-specs/SPEC_LEDGER.md` entry and the spec's `specs/` → `docs/4-specs/` promotion — deferred to spec promotion, outside this ticket batch (see cross-spec follow-ups).
- Any claim of full certification, latest-`main` verification, or later-phase certification (spec §9.3 forbidden wording).

## Acceptance Criteria

### Tests That Must Pass

1. All four workspace gates pass on the commit-under-review.
2. All extended lock-layer suites pass: `negative_fixture_runner`, `hidden_truth_gates`, `event_schema_replay_gates`, `adversarial_gates`, `forbidden_content`.
3. The produced acceptance artifact passes `validate_acceptance_artifact_wording` (scoped exact-commit wording; no forbidden overclaim phrase).

### Invariants

1. The artifact contributes scoped evidence toward `EPI-CERT`/`P0-CERT` and makes no full-certification / latest-`main` / later-phase claim.
2. The capstone introduces no production logic; it only exercises and records.

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; the artifact is produced and validated against the existing acceptance_artifact_wording validator, and verification is the command runbook below.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
2. `cargo test -p tracewake-core --test negative_fixture_runner --test hidden_truth_gates --test event_schema_replay_gates && cargo test -p tracewake-tui --test adversarial_gates && cargo test -p tracewake-content --test forbidden_content`
3. `cargo test -p tracewake-core --test acceptance_artifact_wording` — confirms the wording contract the produced artifact must satisfy.

## Outcome

Completed: 2026-06-09

What changed:
- Produced `reports/0006PHA2A_ACCEPTANCE_ARTIFACT.md` for exact commit `9e0590d056b15d879ac02eb2556c855c080f27e4`.
- Extended `crates/tracewake-core/tests/acceptance_artifact_wording.rs` so the checked-in Phase-2A artifact is validated by the scoped exact-commit wording gate.
- Recorded per-requirement evidence for `EPI-HARD-001` through `EPI-HARD-004`, event/replay parity, content provenance parity, TUI/debug proof, and consumer-seam firewall.

Deviations from original plan:
- The capstone adds one focused validator assertion for the checked-in artifact so future edits cannot silently weaken the scoped wording.
- The ticket remains verification-only for production behavior; no simulation logic changed.

Verification:
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
- `cargo test -p tracewake-core --test negative_fixture_runner --test hidden_truth_gates --test event_schema_replay_gates` — passed.
- `cargo test -p tracewake-tui --test adversarial_gates` — passed.
- `cargo test -p tracewake-content --test forbidden_content` — passed.
- `cargo test -p tracewake-core --test acceptance_artifact_wording` — passed.
