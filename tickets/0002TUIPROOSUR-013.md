# 0002TUIPROOSUR-013: Adversarial gates, typed review artifacts, and acceptance capstone

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core`/`tracewake-tui` (adversarial/negative test gates, typed-diagnostic review artifacts); dual-role capstone (ships adversarial test infrastructure)
**Deps**: 0002TUIPROOSUR-012

## Problem

Current tests use display-string and source-substring checks as authority boundaries: `include_str!(...).contains("apply_event")` (`crates/tracewake-tui/tests/embodied_flow.rs:63-76`) and forbidden-culprit-word substring scans (`crates/tracewake-tui/tests/tui_acceptance.rs:290-297`). The execution testing doctrine rejects this (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:38-52,135-144`). Spec 0002 §4 TUI-AC-011 requires substring scans to be demoted to smoke tests and primary proof to use typed view models, typed provenance, typed report reasons, context hashes, and compile-time boundaries; §7.2 enumerates ten adversarial/negative gates; §7.3 defines the per-test review-artifact format; §8 is the acceptance checklist. This capstone exercises tickets 001–012 end-to-end and is the regression lock for the hardened seam.

## Assumption Reassessment (2026-06-08)

1. Substring/source tests exist at `crates/tracewake-tui/tests/embodied_flow.rs:63-76` and `tui_acceptance.rs:290-297`. Execution doctrine for typed diagnostics / responsible layers / adversarial gates / forbidden test patterns is at `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:24-52,54-96,135-144`. Adversarial fixtures available: `hidden_food_closed_container_001`, `hidden_food_unknown_route_001`, `hidden_route_edge_001`, `debug_omniscience_excluded_001`, `debug_attach_001`, `possession_parity_001`, `possession_does_not_reset_intention_001`, `no_human_day_001`.
2. Spec 0002 §4 TUI-AC-011 + §7.2 (ten adversarial gates) + §7.3 (review-artifact format) + §8 (acceptance checklist + per-requirement conditions). TUI-AC-012's possession-parity *proof* is exercised here (its code lands in tickets 001/002/006).
3. Cross-artifact boundary under audit: the adversarial gates ↔ every hardened surface (sealed packet 001, view builder 002, affordances 003, debug capability 004, typed availability 005, proposal validation 006, two-layer why-not 007, command loop 008, no-human 009, notebook 010, transcript/replay 011, positive fixtures 012). No-direct-dispatch and rule-inference gates use module-visibility/compile-fail boundaries, not substring scans.
4. Invariants restated: **INV-093** — actor-knowledge leakage is high-severity and must be tested against hidden truth; **INV-092** — deterministic replay is tested; **INV-105/106/107** — typed diagnostics, non-leaking feedback, debug quarantine.
5. Fail-closed / no-leak surface: each adversarial gate intentionally attempts a contamination (debug-truth leakage, forged/stale/privileged proposal, possession knowledge transfer, direct dispatch, rule-inference-in-TUI, why-not leakage, render-dependency regression) and asserts it fails closed with typed evidence — this is the regression lock that prevents a friendly golden path from passing while the implementation stays contaminable.
6. Rename/removal blast radius: the `include_str!(...).contains("apply_event")` no-direct-dispatch proof and forbidden-word culprit tests are replaced by module-visibility/dependency or compile-fail boundaries plus typed assertions (substring scans demoted to smoke). Pre-implementation grep `include_str!` and `.contains(` across `crates/tracewake-tui/tests/` and `crates/tracewake-core/tests/`: each authority-bearing site is converted; any retained substring check is explicitly a smoke check.

## Architecture Check

1. Replacing substring/source authority with typed assertions + compile-time/module-boundary gates makes the proof structural: a contamination that a wording scan would miss (renamed culprit, reordered labels) cannot pass, and a no-direct-dispatch violation fails to compile rather than relying on a fragile source-text match — directly realizing the execution doctrine's forbidden-pattern rules and INV-105.
2. No backwards-compatibility aliasing/shims: no parallel "old substring suite" is kept as authority; retained substring checks are clearly labeled smoke-only.

## Verification Layers

1. INV-093 (leakage tested) -> negative test: debug-truth leakage, forged/stale/privileged proposal, possession knowledge-transfer, and why-not leakage gates each fail closed with typed evidence.
2. INV-105 (typed diagnostics) -> codebase grep-proof: primary no-direct-dispatch / rule-inference proof is a module-visibility/compile-fail boundary, not `include_str!`/`.contains(...)`.
3. INV-092 (replay) -> replay/golden-fixture check: render-dependency-regression and deterministic-replay gates reproduce identical outcomes.
4. Acceptance gates (`docs/2-execution/02_*`; `docs/0-foundation/12_*`) -> manual review + command: the §8 workspace gates and per-requirement acceptance conditions pass.

## What to Change

### 1. Adversarial/negative gates (§7.2)

Add the ten gates: debug-truth leakage attempt; forged privileged-view attempt; stale view attempt; possession knowledge-transfer attempt (TUI-AC-012 proof); direct-dispatch attempt (module-visibility/compile-fail); rule-inference-in-TUI attempt (import-boundary/compile-fail); why-not non-leakage attempt; typed-diagnostics attempt (wording change must not flip outcome, typed change must); no-human operator quarantine attempt; render-dependency-regression attempt.

### 2. Typed review-artifact format (§7.3)

Have each added test emit the responsible layer, scenario id, context/actor/controller ids, proposal/action/semantic ids, typed reason codes + provenance refs, debug-capability presence, actor-visible/debug surfaces checked, expected event/log/replay/checksum result, and contamination failure mode.

### 3. Demote substring scans

Convert the `include_str!`/culprit-word authority checks to typed/boundary proofs; retain any substring check only as an explicitly-labeled smoke test.

### 4. Acceptance capstone (§8)

Encode the §8 workspace gates and the per-requirement (TUI-AC-001…012) acceptance conditions as the capstone's acceptance criteria; re-enumerate expected counts from fixtures at test start.

## Files to Touch

- `crates/tracewake-tui/tests/embodied_flow.rs` (modify — demote substring authority)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify — demote culprit-word authority; add gates)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify — forged/stale/leakage gates)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — module-boundary/no-direct-dispatch + rule-inference)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (new)

## Out of Scope

- Production logic — this capstone exercises tickets 001–012; it adds no engine behavior (it does ship adversarial test infrastructure, the dual-role exception).
- Positive proof fixtures (ticket 012).
- Any claim of full project certification (`P0-CERT`/`SPINE-CERT`/`EPI-CERT`) — Spec 0002 §8.3: result wording is scoped to "TUI proof-surface hardening remediation accepted/rejected for the audited seam".

## Acceptance Criteria

### Tests That Must Pass

1. All ten §7.2 adversarial gates fail closed with typed evidence; each emits the §7.3 review-artifact fields.
2. The no-direct-dispatch and rule-inference proofs are module-visibility/compile-fail boundaries (not substring scans); a wording-only change to any display string flips no semantic test, while a typed reason/provenance change does.
3. The §8 workspace gates pass: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`; the per-requirement (TUI-AC-001…012) acceptance conditions hold.

### Invariants

1. Typed/boundary proofs are the authority; substring scans are smoke-only (INV-105; execution doctrine forbidden patterns).
2. Every contamination attempt fails closed and leaks no hidden truth (INV-093/INV-106/INV-107).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/adversarial_gates.rs` — the ten §7.2 gates with §7.3 artifacts.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — module-boundary no-direct-dispatch / rule-inference proofs replacing substring authority.

### Commands

1. `cargo test -p tracewake-tui adversarial_gates`
2. `cargo test -p tracewake-core anti_regression_guards hidden_truth_gates`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
