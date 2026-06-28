# 0056FOUCONSEV-004: Closed verdict grammar (replace the phrase denylist)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` acceptance-artifact wording guard and its support
**Deps**: 0056FOUCONSEV-003

## Problem

Finding F7-04. The wording guard is an open phrase denylist, not a closed verdict grammar. At `2720167`, `crates/tracewake-core/tests/acceptance_artifact_wording.rs` is `FORBIDDEN_RESULT_CLAIMS` (`:31`) plus `CONDITIONAL_CLOSURE_CLAIMS` (`:40`, such as `pass with`, `scoped pass`, `accepted`, `green canonical perimeter`) with ad-hoc branch-protection / historical-results / display-string / green-perimeter checks. Paraphrases ("approved", "validated", "ready to merge", "no blockers remain", "all required evidence is satisfied") bypass it. There is no single computed verdict line keyed to the manifest.

The template already carries the closed-grammar line (`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md:45-50`, "Allowed summary wording" → `Computed result: pass | non-pass`); the enforcement gap is in the **guard code**, not the template.

## Assumption Reassessment (2026-06-28)

1. Live code verified at `2720167`: `acceptance_artifact_wording.rs:31` `FORBIDDEN_RESULT_CLAIMS`, `:40` `CONDITIONAL_CLOSURE_CLAIMS`, `:230` denylist scan, `:262` conditional-closure scan. The computed verdict line already exists in the template (`0003:45-50`) but the guard does not bind prose to it. `support/acceptance_status_manifest.rs` supplies the computed verdict (`compute_result` → pass/non-pass).
2. Spec §4.4 + §6.1. Per the in-session `/reassess-spec` correction (M1), the template's `Computed result:` line is retained, not re-added; this ticket lands the guard that enforces it. The exact named sections and single computed-verdict-line format are owned by template `0003` / execution `10` (§10.5, implementer-recorded) — the non-negotiable is that no prose may assert a verdict stronger than the computed state.
3. **Shared boundary under audit**: `support/acceptance_status_manifest.rs` is edited by 0056FOUCONSEV-003 (governance arm, landed first) and this ticket (verdict-line accessor, if needed) and mutation-covered by 005 — sequential-edit chain (003→004→005), not a parallel merge. Deps 003 declared because the closed grammar keys to the manifest's computed verdict.
4. INV-093 / INV-098 evidence-honesty discipline — a paraphrase-bypassable wording guard lets a closure artifact assert a verdict the computed state does not support; the closed grammar makes the artifact's prose verdict a function of the manifest, not free text.
5. **Fail-closed acceptance enforcement surface**: the wording guard over the acceptance artifact. The closed grammar must be deterministic and blocking: it accepts a correctly-formed `Computed result:` artifact consistent with the manifest and rejects any other verdict-bearing closure claim (paraphrase included) outside the named computed line. No leakage/replay surface is touched. The denylist is retained only as a secondary alarm.

## Architecture Check

1. A closed verdict grammar keyed to computed state is strictly stronger than an open denylist: a denylist can only ban known phrases (and is bypassed by the next paraphrase), whereas a grammar that forbids any verdict-bearing sentence outside the single computed line cannot be paraphrased around. Keeping the denylist as a secondary alarm preserves the existing coverage at zero regression cost.
2. No backwards-compatibility shim: the grammar becomes the primary guard; the denylist is demoted, not run in parallel as an equal authority.

## Verification Layers

1. INV-098 (harsh acceptance) -> synthetic artifacts: the guard rejects paraphrased closure ("approved", "validated", "ready to merge", "no blockers remain", "all required evidence is satisfied") when the computed state is non-pass or the claim sits outside the computed verdict line, and accepts a correctly-formed `Computed result:` artifact consistent with the manifest.
2. INV-093 evidence-honesty -> manual review that no prose verdict can exceed the computed state; the denylist still fires as a secondary alarm on its known phrases.
3. Single-surface ticket — the synthetic wording corpus is the proof surface; the sequential chain covers the adjacent manifest (003) and mutation (005) surfaces.

## What to Change

### 1. Require a single computed verdict line

The guard parses the allowed sections and treats `Computed result: pass` / `Computed result: non-pass` (already required by template `0003`) as the only verdict-bearing line, consistent with the manifest's `compute_result`.

### 2. Reject verdict-bearing prose outside the computed line

Free prose may not contain a verdict-bearing closure claim (paraphrase included) outside the named computed line — either parse the allowed sections and reject any other closure claim, or forbid verdict-bearing sentences outside named sections.

### 3. Demote the denylist to a secondary alarm

Retain `FORBIDDEN_RESULT_CLAIMS` / `CONDITIONAL_CLOSURE_CLAIMS` as a secondary alarm, not the primary grammar.

## Files to Touch

- `crates/tracewake-core/tests/acceptance_artifact_wording.rs` (modify)
- `crates/tracewake-core/tests/support/acceptance_status_manifest.rs` (modify — verdict-line accessor if the grammar reads the computed verdict from support; sequential chain with 003/005)

## Out of Scope

- The governance posture (0056FOUCONSEV-003) and the manifest's compute_result arms.
- Template `0003` and the §6.1 doctrine docs (0056FOUCONSEV-007) — the verdict line already exists there; this ticket enforces it in code.
- Mutation coverage (0056FOUCONSEV-005) and the standing run (0056FOUCONSEV-006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` — paraphrase-rejection cases and the accepted well-formed `Computed result:` case pass.
2. `cargo test --locked -p tracewake-core` — manifest/support tests unaffected.

### Invariants

1. No verdict-bearing closure claim outside the single computed verdict line passes the guard; the computed line must be consistent with the manifest.
2. The denylist remains a secondary alarm; removing it alone cannot make a paraphrased over-claim pass.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_artifact_wording.rs` — closed-grammar accept/reject corpus including the five named paraphrases.

### Commands

1. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording`
2. `cargo test --locked -p tracewake-core`
