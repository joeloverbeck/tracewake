# 0056FOUCONSEV-003: Doctrine-complete `solo-maintainer-compensating-control` manifest posture

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` acceptance-status manifest parser/support and its tests
**Deps**: None

## Problem

Finding F7-03. The acceptance-status manifest parser recomputes a verdict from a machine-readable status block (real progress), but does not know the post-`0055` `solo-maintainer-compensating-control` governance posture that `docs/2-execution/10` now ratifies. At `2720167`, `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`:

- `governance_is_independent(...)` (`:382`) accepts only `independent-review` and `last-push-required-reviewer`.
- `compute_result(...)` (`:283`) marks `pending-governance` / `status-checks-only` / `zero-approval` as non-pass and returns `Err("unknown governance_independence: {other}")` for any other value (`:292`).

A truthful post-`0055` solo-maintainer artifact would be rejected as unknown, or its author would have to mislabel the posture to obtain a pass — a fail-closed implementation failure against settled doctrine (`docs/2-execution/10:358-392` ratifies the posture when the compensating-control set is proven).

## Assumption Reassessment (2026-06-28)

1. Live code verified at `2720167`: `support/acceptance_status_manifest.rs:84` `struct ParsedManifest` with `governance_independence: String` (`:88`); `compute_result:289` matches `independent-review` | `last-push-required-reviewer` (pass-eligible), `pending-governance` | `status-checks-only` | `zero-approval` (non-pass), `other => Err(unknown...)`; `governance_is_independent:382` accepts the two independent values. `validate_status_manifest:50` and `has_mutation_survivors:113` are the surrounding contract. Test file: `crates/tracewake-core/tests/acceptance_status_manifest.rs`.
2. Spec §4.3 + §6.1 govern. `docs/2-execution/10:358-392` ratifies `solo-maintainer-compensating-control` "only when the manifest records `solo-maintainer-compensating-control` and the ruleset proof shows every architecture-tier compensating control." `docs/1-architecture/13:136/155` carries the same solo-maintainer mode at its tier. The solo-maintainer *acceptor* recommendation (§10.3) is process-ratifiable; the taxonomy enforces the proven compensating-control set either way (note only, no code branch on it).
3. **Shared boundary under audit**: `support/acceptance_status_manifest.rs` is also edited by 0056FOUCONSEV-004 (closed verdict grammar) and mutation-covered by 0056FOUCONSEV-005 — a sequential-edit chain (003→004→005), not a parallel merge. This ticket lands the governance arm first.
4. INV-093 (actor-knowledge leakage is high-severity — here the acceptance-evidence analogue: a laundered governance posture is an evidence-honesty defect) and the INV-098 harsh-acceptance discipline — the parser must represent the settled posture truthfully and fail closed on an unproven one, never accept a mislabeled or prose-only governance claim.
5. **Fail-closed acceptance enforcement surface**: `compute_result` / `governance_is_independent` and the new compensating-control validation. The change must stay deterministic and blocking: a fully-proven solo-maintainer artifact computes pass; any missing/unproven/stale/prose-only compensating-control field computes non-pass; bare `zero-approval` / `status-checks-only` remain non-pass; self-authored-only behavioral evidence stays rejected (the `0055` no-weakening conditions stand). No leakage or replay surface is touched (this is acceptance scoring over a status block).
6. **Schema extension (additive)**: extends the manifest's `governance_independence` enum with one accepted value (`solo-maintainer-compensating-control`) and adds machine-readable compensating-control fields (required checks present, active enforcement, no bypass actors, `current_user_can_bypass: never`, non-fast-forward protection, deletion protection, strict/up-to-date enforcement). Consumers of the manifest schema: `validate_status_manifest`, `compute_result`, the CI ingestion test `actual_acceptance_artifact_from_ci_env_is_pass_eligible`, and the template `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (field-set documentation lands in 0056FOUCONSEV-007). The extension is **additive** — new accepted value + new fields required only for that value; existing `independent-review` / `last-push-required-reviewer` artifacts continue to compute pass unchanged.

## Architecture Check

1. Teaching the parser the settled posture (rather than leaving it to reject truthful artifacts as unknown) is the only doctrine-complete option: a fail-closed parser that cannot represent a ratified governance mode forces authors to mislabel, which is the laundering the barrier exists to prevent. The compensating-control field set makes the posture *provable* rather than asserted.
2. No backwards-compatibility shim: the new value is added to the same closed match, not a parallel lenient path. All other unknown values still fail closed (`Err`), preserving the closed-set discipline.

## Verification Layers

1. INV-098 (harsh acceptance) -> deterministic synthetic manifest cases: fully-proven solo-maintainer → pass; any missing/unproven/stale/prose-only compensating-control field → non-pass; bare `zero-approval`/`status-checks-only` → non-pass; `independent-review`/`last-push-required-reviewer` → pass.
2. INV-093 evidence-honesty -> invariants-alignment review that the no-weakening conditions (`0055`) hold: self-authored-only behavioral evidence still rejected; the exception scopes to the human-approval dimension only.
3. Single-surface taxonomy ticket — additional cross-artifact mapping is covered by the sequential chain (004 grammar, 005 mutation, 007 doctrine/template); this ticket's proof surface is the synthetic manifest corpus.

## What to Change

### 1. Accept the settled governance value

Extend `governance_is_independent` / `compute_result` to accept `solo-maintainer-compensating-control` as pass-eligible **only** when the compensating-control fields prove the posture.

### 2. Require and validate the compensating-control field set

Parse and require, for that value: required checks present, active enforcement, no bypass actors, `current_user_can_bypass: never`, non-fast-forward protection, deletion protection, strict/up-to-date enforcement. Fail closed if any field is absent, unproven, stale, or prose-only.

### 3. Preserve the existing rejections

Keep bare `zero-approval` / `status-checks-only` non-pass; continue rejecting self-authored-only behavioral evidence; keep all other values failing closed.

## Files to Touch

- `crates/tracewake-core/tests/support/acceptance_status_manifest.rs` (modify — sequential chain with 004/005)
- `crates/tracewake-core/tests/acceptance_status_manifest.rs` (modify — synthetic governance cases)

## Out of Scope

- The closed verdict grammar (0056FOUCONSEV-004) and the wording guard.
- Template `0003` field-set documentation and the §6.1 doctrine synchronization (0056FOUCONSEV-007).
- Mutation coverage of the parser (0056FOUCONSEV-005) and the standing run (0056FOUCONSEV-006).
- Re-litigating whether solo-maintainer mode is legitimate (spec §1.2 — settled doctrine).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test acceptance_status_manifest` — the four synthetic governance cases (proven solo-maintainer pass; any missing/unproven/stale/prose-only field non-pass; bare zero-approval/status-checks-only non-pass; independent-review/last-push-required-reviewer pass).
2. `cargo test --locked -p tracewake-core` — surrounding manifest/support tests unaffected.

### Invariants

1. A pass for `solo-maintainer-compensating-control` requires every compensating-control field proven; any gap computes non-pass (fail-closed).
2. No previously-rejected posture (`zero-approval`, `status-checks-only`, self-authored-only behavioral evidence) becomes pass-eligible; all other values still fail closed.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_status_manifest.rs` — solo-maintainer pass/non-pass synthetic corpus + regression cases for the preserved rejections.

### Commands

1. `cargo test --locked -p tracewake-core --test acceptance_status_manifest`
2. `cargo test --locked -p tracewake-core`

## Outcome

Completed: 2026-06-28

Implemented the doctrine-complete `solo-maintainer-compensating-control`
manifest posture in `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`.
The manifest parser now recognizes the settled governance value and treats it as
pass-eligible only when the machine-readable compensating-control set is present
with exact current values:

- `required_checks_present: all-standing-required`
- `active_enforcement: active`
- `bypass_actors: none`
- `current_user_can_bypass: never`
- `non_fast_forward_protection: enabled`
- `deletion_protection: enabled`
- `strict_required_status_checks_policy: enabled`

Any missing, stale, prose-only, or otherwise unproven control computes
`non-pass`; if an artifact states `pass` while those controls are incomplete,
validation fails closed through the existing stated-vs-computed result check.
Existing accepted postures `independent-review` and
`last-push-required-reviewer` remain pass-eligible. Existing rejected postures
`zero-approval` and `status-checks-only` remain non-pass, and the existing
self-authored/stale/display-only evidence rejection remains unchanged.

Added synthetic coverage in
`crates/tracewake-core/tests/acceptance_status_manifest.rs` for:

- fully proven solo-maintainer compensating control computing pass;
- every required compensating-control field computing non-pass when missing or
  unproven;
- `last-push-required-reviewer` preserving its previous pass posture;
- existing `zero-approval`, `status-checks-only`, and self-authored evidence
  rejections remaining in force.

Deviations from original plan: none. The docs/template synchronization remains
out of scope for this ticket and is still assigned to later 0056 tickets.

Verification:

- `cargo test --locked -p tracewake-core --test acceptance_status_manifest` —
  passed.
- `cargo test --locked -p tracewake-core` — passed.
- `cargo fmt --all --check` — passed.
