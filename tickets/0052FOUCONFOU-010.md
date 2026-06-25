# 0052FOUCONFOU-010: F4-08 — standing-mutation survivor closure (focused + standing-green)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the #17–18 `StuckDiagnostic::deserialize_canonical` witnesses; runs focused per-survivor and full standing mutation campaigns and records the disposition
**Deps**: 0052FOUCONFOU-009

## Problem

Spec 0052 F4-08 / §4.9 / §5; closure-order steps 9–10. The 0051 standing campaign completed with **23 missed mutants** (historical artifact: 3,275 selected / 2,549 caught / 703 unviable / 23 missed / 0 timeout at `a3b46c6`) and was accepted honestly non-green. Most in-surface survivors are closed by the per-finding tickets (§5: #8→005, #9–16→008, #19→003, #20→004+005, #21→004, #22→002+003, #23→008+009), but **#17 and #18** (`StuckDiagnostic::deserialize_canonical` match-arm deletion and `!` deletion) are owned by no finding ticket and need new witnesses here. This ticket adds those witnesses, runs focused per-survivor campaigns for every in-surface survivor (#8–#23) for fast feedback, then runs the full standing campaign from a clean baseline and publishes the complete caught/missed/unviable/timeout disposition that feeds the capstone (013).

The out-of-surface `food_source_fact_supersedes` family (#1–7, in `projections.rs`) is **routed forward** (spec §1.2): it is not fixed here, not moved to a lower tier, and not laundered as equivalent — but the canonical reconciliation must still report it, and the perimeter may not be called green while it survives.

## Assumption Reassessment (2026-06-25)

1. `StuckDiagnostic` and `deserialize_canonical` live in `crates/tracewake-core/src/agent/trace.rs` (StuckDiagnostic at trace.rs:696). `food_source_fact_supersedes` is in `crates/tracewake-core/src/projections.rs`. The canonical mutation perimeter is `.cargo/mutants.toml`; the zero-floor ratchet file `.cargo/mutants-baseline-misses.txt` is empty and must stay empty for this surface.
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §4.9, §5; closure-order steps 9–10. The per-finding survivor witnesses are in 002–008; the production-boundary corpus (009) makes them non-vacuous. This ticket adds only the unowned #17–18 witnesses and runs the campaigns.
3. Cross-artifact boundary under audit: the `StuckDiagnostic` canonical serialization codec (a replay/debug diagnostic) and the standing mutation perimeter (`.cargo/mutants.toml` + `.cargo/mutants-baseline-misses.txt`). Per §4.9 the perimeter already covers the seams — do **not** edit `.cargo/mutants.toml` to restate coverage, and do **not** pad the baseline for killable foundational mutants.
4. Motivating invariants: INV-092 (deterministic replay is tested), INV-093 (leakage regression-tested), INV-094 (possession parity tested), INV-098 (harsh acceptance). "Mutation completed" is not "mutation passed": exit 2 (missed) and exit 3 (timeout) are failures for the standing green claim, as are tool errors, incomplete census, stale artifacts, missing shards, and fingerprint mismatch.
5. Fail-closed / replay-codec surface (evidence basis): the #17–18 witnesses exercise `StuckDiagnostic::deserialize_canonical` with full-field non-default round trips plus malformed/missing/unknown-field fail-closed cases — proving the codec rejects invalid canonical records deterministically (INV-018 replay determinism, INV-105 inspectable diagnostics). The witnesses add test-only code and introduce no leakage/nondeterminism path. No survivor is classified equivalent without a defensible semantic argument.

## Architecture Check

1. Closing #17–18 with full-field round-trip + malformed/missing/unknown-field cases (not a getter-calling test) is the strongest practical closure for a serialization codec, and running the full standing campaign from a clean baseline — rather than a focused 0-miss run — is what certifies the standing perimeter; a focused 0-miss run is evidence only for its selected functions/mutants and exact source tree.
2. No backwards-compatibility shim. The `.cargo/mutants.toml` perimeter is not edited to restate coverage; the baseline stays empty.

## Verification Layers

1. INV-105/INV-018 (inspectable diagnostics, deterministic codec) -> #17–18 witnesses: full-field non-default round trip + malformed/missing/unknown-field fail-closed cases; valid/invalid canonical records differing only in the affected predicate.
2. INV-092/093/094/098 (standing regression sensitivity) -> full standing `cargo mutants --timeout 183` from a clean baseline; publish selected denominator + complete caught/missed/unviable/timeout disposition + shard/census fingerprints.
3. Out-of-surface routing -> recorded disposition: `food_source_fact_supersedes` (#1–7) reported by the canonical reconciliation; the perimeter is not called green while it survives.

## What to Change

### 1. #17–18 StuckDiagnostic canonical-codec witnesses (`agent/trace.rs`)

Add `StuckDiagnostic::deserialize_canonical` witnesses: a full-field non-default round trip, and malformed / missing-field / unknown-field fail-closed cases constructed so a deleted match arm (#17) or a deleted `!` predicate (#18) flips a typed result/error.

### 2. Run focused + standing campaigns (command/record)

Run focused per-survivor campaigns for #8–#23 (fast feedback), then the full standing campaign from a clean baseline. Record the selected denominator and complete disposition (caught/missed/unviable/timeout + shard/census fingerprints) for the capstone (013). Record the `food_source` family's routed-forward status; do not fix or launder it.

## Files to Touch

- `crates/tracewake-core/src/agent/trace.rs` (modify — add #17/#18 canonical-codec round-trip + fail-closed witnesses)

## Out of Scope

- Fixing the `food_source_fact_supersedes` family (#1–7) — routed forward to a future cross-cutting mutation-remediation line (spec §1.2); recorded here, not fixed.
- Editing `.cargo/mutants.toml` coverage or padding `.cargo/mutants-baseline-misses.txt` (§4.9).
- CI wiring / branch-protection of the standing barrier (011).
- The per-finding survivor witnesses #8–16, #19–23 (owned by 002–009).

## Acceptance Criteria

### Tests That Must Pass

1. The #17/#18 witnesses kill match-arm deletion and `!` deletion in `StuckDiagnostic::deserialize_canonical` (focused `cargo mutants -f crates/tracewake-core/src/agent/trace.rs` shows them caught).
2. The full standing campaign completes from a clean baseline with **zero in-surface misses and zero timeouts**; the disposition (selected/caught/missed/unviable/timeout + fingerprints) is recorded.
3. `cargo test -p tracewake-core` passes the new witnesses; the `food_source` family is reported by the canonical reconciliation and the perimeter is not called green while it survives.

### Invariants

1. Every in-surface survivor (#8–#23) is caught or unviable-with-a-defensible-reason; the baseline stays empty for this surface (INV-092/098).
2. No survivor is classified equivalent without a defensible semantic argument; "mutation completed" ≠ "mutation passed" (INV-098).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/trace.rs` test module — #17/#18 `deserialize_canonical` round-trip + fail-closed witnesses.

### Commands

1. `cargo test -p tracewake-core agent::trace` then focused `cargo mutants -f crates/tracewake-core/src/agent/trace.rs --timeout 183` (per-survivor boundary).
2. `cargo mutants --timeout 183` — full standing campaign from a clean baseline (the deliverable run; cannot be cheaply dry-run — its disposition output is the recorded evidence).
