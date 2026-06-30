# 0057EMBROUCON-007: Acceptance capstone — embodied routine continuation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None
**Deps**: 0057EMBROUCON-005, 0057EMBROUCON-006

## Problem

Spec 0057 §7. The feature is "done" only when it is TUI-playable *and* no-human runnable, replay-safe, with possession parity and an intact hidden-truth firewall (INV-098). This capstone consolidates the §7 acceptance evidence into a single review artifact: the four-gate local suite green, the new embodied go-to-work golden, the parity entry passing, the hidden-truth firewall fixture, and the named exact implementation commit. No certification gate is claimed. It introduces no new production logic — it exercises and records what 0057EMBROUCON-001…006 composed.

## Assumption Reassessment (2026-06-30)

1. The evidence surfaces are produced by the leaf tickets: the shared resolver (001), the follow-on commit + receipt (002), typed-block/wait/stuck (003), marker guards (004), embodied golden + parity + firewall fixture + replay (005), and the doctrine amendments (006). The acceptance-artifact template is `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`; the report convention is `reports/<NNNN>_<slug>_acceptance.md` (sibling `reports/0048_foundational_conformance_hardening_acceptance.md` exists).
2. Spec assumption: `specs/0057_…_SPEC.md` §7 governs the evidence list; §0 "Ledger timing" defers the `docs/4-specs/SPEC_LEDGER.md` archived-row entry and the `archive/specs/` move to spec acceptance/closeout — those are a spec-promotion follow-up, not this ticket (recorded in the decomposition's Step 6 cross-spec follow-ups).
3. **Cross-artifact boundary under audit**: the acceptance artifact consolidates evidence across `tracewake-core`/`-content`/`-tui` (the four-gate suite, the embodied golden, the parity entry, the firewall fixture) plus the named exact implementation commit — it audits the replay / actor-knowledge / no-human enforcement surfaces on the evidence-consumer basis, it does not modify them.
4. INV-098 (feature acceptance is harsh — caused, agent-possible, eventful, epistemically bounded, TUI-playable, no-human runnable, replay-safe, regression-tested), INV-095 (TUI/view-model tests are acceptance tests), INV-094 (possession parity is tested). The capstone enumerates these as acceptance sub-cases mapped to the spec's §7 evidence list.
5. **Evidence-consumer enforcement surface (substrate basis)**: the capstone audits/re-proves the replay, actor-knowledge-firewall, and no-human surfaces. Confirm: the recorded witnesses stay observer-only (no hidden-truth leak into the artifact's evidence rows — INV-093), and the four-gate suite + named tests are **re-run at acceptance time** (not copied from memory) to catch any post-implementation regression.

## Architecture Check

1. A single acceptance capstone gated on the leaf set ({005, 006}, whose transitive `Deps` cover 001–004) renders one reviewable verdict for the spec's §7 evidence, rather than scattering acceptance claims across implementation tickets. It introduces no new production logic; it exercises the pipeline the earlier tickets composed.
2. No backwards-compatibility aliasing or shims: the artifact records evidence and re-runs existing tests/gates; it touches no production path.

## Verification Layers

1. INV-098 (harsh acceptance) -> four-gate suite re-run at acceptance: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo build --locked`, `cargo test --workspace` all green.
2. INV-095 / INV-094 (TUI acceptance; parity) -> the embodied go-to-work golden and the playable-capability parity entry (from 005) pass; bind/unbind preserves the next routine step.
3. INV-018 / INV-099 (replay; firewall) -> the embodied follow-on commit replays physical-checksum-stable, and the hidden-truth firewall fixture yields a typed knowledge blocker — recorded with the named exact implementation commit.

## What to Change

### 1. Author the acceptance artifact

Create `reports/0057_embodied_routine_continuation_acceptance.md` from the `docs/4-specs/0003` review-artifact template, enumerating the §7 evidence: the four-gate suite results (re-run), the embodied go-to-work golden, the parity entry, the hidden-truth firewall fixture, the marker-not-progress guard, the blocked-continuation/stuck eligibility, replay stability, and the named exact implementation commit. Record any deviations from the spec plan. No certification gate is claimed.

## Files to Touch

- `reports/0057_embodied_routine_continuation_acceptance.md` (new — acceptance artifact; archived to `archive/reports/` on spec acceptance per `docs/archival-workflow.md`)

## Out of Scope

- The `docs/4-specs/SPEC_LEDGER.md` archived-row entry and the `specs/` → `archive/specs/` move — deferred to spec acceptance/closeout (spec §0 "Ledger timing"; decomposition Step 6 cross-spec follow-up).
- Any production logic or new test logic — the leaf tickets own those; this capstone only exercises and records.
- Any certification-gate claim (spec §0: `P0-CERT not applicable`).

## Acceptance Criteria

### Tests That Must Pass

1. The four-gate local suite is green at acceptance time (re-run, not copied): `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.
2. The embodied go-to-work golden, the playable-capability parity entry, the possession-parity test, and the hidden-truth firewall fixture (all from 0057EMBROUCON-005) pass; the doctrine amendments (0057EMBROUCON-006) have landed with owner approval.
3. The artifact names the exact implementation commit and records the §7 evidence; no certification gate is claimed.

### Invariants

1. The artifact's recorded witnesses are observer-only — no hidden truth leaks into the evidence rows (INV-093).
2. Acceptance is established by re-running the gates/tests at acceptance time, not by copying prior results.

## Test Plan

### New/Modified Tests

1. `None — acceptance/evidence-only capstone; it adds no production or test logic and re-runs the leaf tickets' existing tests/gates, recording witnesses in the artifact.`

### Commands

1. `cargo test --locked -p tracewake-tui --test embodied_flow --test playable_capability_parity --test tui_acceptance && cargo test --locked -p tracewake-content --test golden_fixtures_run` — the named acceptance tests.
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — the four-gate suite.
3. `grep -nF "0057_embodied_routine_continuation_acceptance" reports/0057_embodied_routine_continuation_acceptance.md` — artifact landed at the convention path.
