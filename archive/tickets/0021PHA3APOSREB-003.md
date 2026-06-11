# 0021PHA3APOSREB-003: Per-arm census anchoring, inverted write-shape scan, and exemption-key derivation

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` test oracle (`tests/anti_regression_guards.rs`); conformance-index row update
**Deps**: `specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-069, ORD-HARD-070)

## Problem

The derived apply-arm census's per-arm enforcement is vacuous for inline arms:
`enclosing_apply_anchor` refines to an `EventKind::` arm anchor only when the
enclosing fn is exactly `"apply_agent_event"`, but every inline write lives in
`apply_agent_event_with_capability`, so all six inline arms share one function-level
anchor and `apply_write_site_has_version_gate` — which scans from the anchor start to
the write site — lets any earlier arm's gate satisfy any later arm's check. Deleting
`require_payload_version` from arms 2–6 leaves the census green (`ORD-HARD-069`;
INV-020; incomplete closure of `ORD-HARD-054`'s own correction). Secondary gaps
(`ORD-HARD-070`): the write-shape whitelist (`insert(`/`entry(`/`get_mut(` on a
literal `state.` receiver) is an open universe — `retain`/`extend`/rebound
`&mut state.<map>`/helper-fn writes are invisible, and set-equality needs only ≥1
visible site per map; and `TYPED_COLUMN_CLOSURE_EXEMPTIONS.typed_columns` lists are
checked for non-emptiness only — `status` is consumed by `apply_intention_started`
yet absent from its exemption's list, and nothing would catch an exempted helper
retaining `payload_fields(event)`.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: `enclosing_apply_anchor` (anti_regression_guards.rs)
   returns the fn name unless it equals `"apply_agent_event"`; all inline writes are
   in `apply_agent_event_with_capability` (`events/apply.rs`);
   `apply_write_site_has_version_gate` scans `scan_source[start..site.index]` from the
   fn-anchor start; the write-method whitelist is `["insert(", "entry(", "get_mut("]`;
   the exemption meta-guard checks `!exemption.typed_columns.is_empty()` plus anchor
   liveness only. Sweep confirms no *current* production bypass shape exists in
   apply.rs (all arms are gated or exempted today) — this ticket is lock repair, not
   a product-behavior fix.
2. Verified against spec 0021 (reassessed 2026-06-11; operator-verified including the
   `status`-consumed-but-unlisted example): findings ORD-HARD-069/070; §6 requires
   updating the `0020 derived apply-arm payload census` conformance row once per-arm
   anchoring is real.
3. Cross-artifact boundary under audit: the census contract between
   `events/apply.rs` (apply arms), `AGENT_STATE_CHECKSUM_COVERAGE` (checksum.rs), and
   the anti-regression census — every covered-map write is version-gated per arm or
   exempted with a true typed-column closure.
4. INV-020 restated: event schema evolution is mandatory — replay must reject
   unsupported history loudly rather than silently inventing repairs; a census that
   cannot detect an ungated arm cannot enforce that.
5. Deterministic-replay enforcement surface touched (the census is replay's
   reject-loudly lock): changes strengthen enforcement only — no production apply
   logic, checksum, or replay semantics change. If the exemption-key derivation
   reveals consumed keys missing from `typed_columns` lists, the lists are corrected
   to match the actual consumed keys (guard-data fix, not a schema change).

## Architecture Check

1. Anchoring inline sites to the nearest preceding `EventKind::` token whenever the
   enclosing fn name starts with `apply_agent_event` (and scanning gates from that arm
   token) fixes the defect at its root — the anchor — rather than special-casing the
   current fn name again (which would re-break on the next rename). Inverting the
   write-shape scan (flag any non-read method on `state.<map>`, ban rebound
   `&mut state.<map>` and `payload_fields(` outside registered sites) closes the
   open-universe gap by construction: unknown shapes fail closed instead of passing
   silently. Deriving consumed keys per exempted anchor and asserting ⊆
   `typed_columns` makes the exemption claim behavioral, not decorative — the
   guard-vacuity lesson this spec names.
2. No backwards-compatibility aliasing/shims: the old anchor refinement is replaced,
   not kept as a fallback path.

## Verification Layers

1. INV-020 (per-arm gating) -> synthetic two-arm inline match (arm 1 gated, arm 2
   ungated) must fail the census; per-arm behavioral forged-version tests for every
   gated kind.
2. Lock durability (write-shape closure) -> synthetic `retain`-shaped covered-map
   write must fail; synthetic rebound `&mut state.<map>` must fail.
3. Lock durability (exemption truth) -> synthetic exempted-helper-consumes-unlisted-key
   case must fail; `payload_fields(` ban in exempted helpers proven by synthetic.
4. Conformance honesty -> grep-proof: the updated `0020 derived apply-arm payload
   census` row describes per-arm anchoring only after this ticket lands.

## What to Change

### 1. Per-arm anchoring (ORD-HARD-069)

`enclosing_apply_anchor`: refine to the nearest preceding `EventKind::` token whenever
the enclosing fn name starts with `apply_agent_event`;
`apply_write_site_has_version_gate`: start the gate-scan segment at that arm token.
Add the two-arm synthetic and per-kind forged-version behavioral tests (extending the
existing `event_schema_replay_gates` coverage to every gated kind).

### 2. Inverted write-shape scan (ORD-HARD-070a)

Replace the write-method whitelist with: flag any `state.<map>.` followed by a method
not in a read allowlist; assert no `&mut state.<map>` binding and no `payload_fields(`
occurrence outside registered sites. Synthetic negatives for both.

### 3. Exemption-key derivation (ORD-HARD-070b)

Per exempted anchor, derive consumed payload keys (scan for `required(payload, "…")` /
`payload.get("…")`) and assert they ⊆ `typed_columns`; correct any list found short
(e.g. `apply_intention_started`'s `status`); ban `payload_fields(` in exempted
helpers. Synthetic negative.

### 4. Documentation

Update the `0020 derived apply-arm payload census` conformance row
(`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`).

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- Production changes to `events/apply.rs` — all arms are currently gated/exempted;
  the reject-loudly posture fixes (panic→typed error, silent defaults) are
  `ORD-HARD-085` (ticket 0021PHA3APOSREB-006).
- The world-applier fail-open catch-all (`ORD-HARD-079` — ticket
  0021PHA3APOSREB-008).
- Mutation perimeter/baseline guards (ticket 0021PHA3APOSREB-004).

## Acceptance Criteria

### Tests That Must Pass

1. Two-arm inline synthetic (gated + ungated) fails the census pre-fix and the
   real census passes post-fix with per-arm anchors.
2. Write-shape synthetics (`retain`-shaped write; rebound `&mut state.<map>`;
   `payload_fields(` outside registered sites) each fail the inverted scan.
3. Exemption synthetics (unlisted consumed key; `payload_fields(` in an exempted
   helper) each fail; all six real exemptions pass with derived-key ⊆ list proven.
4. Per-kind forged-version behavioral tests pass for every gated kind.
5. `cargo test -p tracewake-core --test anti_regression_guards` and
   `cargo test --workspace` green.

### Invariants

1. Every inline apply arm writing a checksum-covered `AgentState` map is individually
   anchored — deleting any single arm's gate fails the census.
2. Exemption `typed_columns` lists are behaviorally true: every consumed payload key
   of an exempted helper appears in its list.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — per-arm anchor fix +
   five synthetic negatives + derived-key assertion.
2. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — per-kind
   forged-version coverage completion (extend existing patterns; verify path at
   implementation, the suite exists per 0020's behavioral forged-version tests).

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_apply_arms_require_payload_schema_version -- --nocapture`
2. `cargo test -p tracewake-core --test event_schema_replay_gates`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

- Fixed inline apply-arm anchoring so `apply_agent_event*` write sites anchor to
  the nearest preceding `EventKind::` arm instead of the function body.
- Replaced the covered-map write scan with a fail-closed non-read method scan,
  plus guards for `&mut state.<map>` rebinding and raw `payload_fields(` outside
  gated materialized writes.
- Added derived exemption-key checks for `TYPED_COLUMN_CLOSURE_EXEMPTIONS` and
  corrected the exemption key lists for need deltas and intention starts.
- Added synthetic failures for ungated second inline arms, `retain` writes,
  rebound aliases, raw payload retention, and unlisted consumed payload keys.
- Extended event-schema replay gates across all payload-gated materialized agent
  event kinds, plus trace and diagnostic schema gates.
- Updated the `0020 derived apply-arm payload census` architecture conformance
  row to describe the stricter lock.

No production apply/replay behavior changed; this ticket strengthened the source
oracle and behavioral coverage around existing gated/exempted arms.

## Verification

- `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_apply_arms_require_payload_schema_version -- --nocapture`
- `cargo test -p tracewake-core --test anti_regression_guards typed_column_closure_exemptions_are_rationale_bearing_and_live -- --nocapture`
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo test -p tracewake-core --test event_schema_replay_gates`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
