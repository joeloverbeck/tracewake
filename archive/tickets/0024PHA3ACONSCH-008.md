# 0024PHA3ACONSCH-008: Derive the post-seed mutation perimeter from the apply surface and enroll it with live witnesses

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` test guards (`crates/tracewake-core/tests/anti_regression_guards.rs`) and spec evidence correction; no production crate code.
**Deps**: 0024PHA3ACONSCH-001

## Problem

Spec 0024 `ORD-HARD-144` (medium): the post-seed mutation guard
`no_direct_apply_event_outside_event_replay_or_pipeline` forbids only the literal
tokens `apply_event(` and `apply_event_stream(`, but `events/apply.rs` exposes four
public mutators — the unscanned `apply_agent_event` and `apply_epistemic_event`
write authoritative agent/epistemic state and are called from live production seams
such as `scheduler.rs` and `actions/pipeline.rs`, while `agent/no_human_surface.rs`
only carries a test-only apply helper in the current checkout. A new direct `apply_agent_event` call in a
view model, content loader, or TUI path would mutate authoritative state with zero
guard firing. The scan also has no nonzero-witness floor (not enrolled in
`META_LOCK_REGISTRY`), so a refactor renaming the allowlisted seams' calls leaves it
green with zero real sites.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: the two-token forbidden set and the
   three-file allowlist in `no_direct_apply_event_outside_event_replay_or_pipeline`;
   the four `pub fn apply_*` signatures in `crates/tracewake-core/src/events/apply.rs`
   (`apply_event_stream`, `apply_event`, `apply_epistemic_event`,
   `apply_agent_event`); call-site counts in `scheduler.rs`, `actions/pipeline.rs`,
   `replay/rebuild.rs`, and `events/apply.rs`. Current production call sites are
   lawful — the defect is guard scope, not behavior. Reassessment corrected the
   earlier `agent/no_human_surface.rs` assumption: its apply helper is test-only.
2. Verified against spec 0024 §4 (`ORD-HARD-144`) and R-28
   (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`): a hand-picked token subset of a
   public API is the type-convention census shape.
3. Cross-artifact boundary: the apply-surface ↔ perimeter-guard parity contract —
   the forbidden-token set must be derived from `apply.rs`'s public surface so a
   fifth mutator auto-extends the scan.
4. INV-009/INV-011 restated: meaningful state changes require events;
   current-state-only mutation shortcuts are forbidden — the guard's claim ("event
   application is the only post-seed mutation path") must be enforced for the whole
   mutator population, not half of it.
5. Enforcement surface (mutation perimeter): this ticket widens detection and adds
   witnesses; it changes no production code path, relaxes no validation, and the
   newly-allowlisted production seams (`events/apply.rs`, `scheduler.rs`,
   `actions/pipeline.rs`, `replay/rebuild.rs`) each carry a cited rationale —
   the lawful routing surfaces verified during the audit.

## Architecture Check

1. Deriving the forbidden tokens from a parity census against `apply.rs`'s
   `pub fn apply_*` signatures (failing when the sets diverge) closes the complement
   structurally — the `ORD-HARD-123` census lesson applied to the mutation
   perimeter — instead of growing the hand list by two and waiting for the next
   mutator to slip past. Registry enrollment with a per-seam live witness makes the
   scan's own vacuity detectable (ticket -001's contract).
2. No backwards-compatibility aliasing/shims: no legacy token list retained; the
   derivation is the single source.

## Verification Layers

1. INV-009/011 perimeter closure → injection synthetic: `apply_agent_event(` in a
   non-allowlisted source fails the scan (test).
2. R-28 parity → census synthetic: a new `pub fn apply_*` in `apply.rs` absent from
   the scan's derived set fails the parity check (test).
3. Witness floor → registry enrollment with `witness_min` ≥ 1 live apply call per
   allowlisted seam; a seam whose live call disappears drops the witness
   (live-witness rule from ticket -001).
4. Whole-pipeline → full workspace gates.

## What to Change

### 1. Derived forbidden set

Build the token set from `apply.rs`'s `pub fn apply_*` signatures (4 enumerated
today: `apply_event`, `apply_event_stream`, `apply_epistemic_event`,
`apply_agent_event`) via a parity census; scan all core production sources.

### 2. Rationale-bearing allowlist

Allowlist `events/apply.rs` (definitions), `replay/rebuild.rs`,
`actions/pipeline.rs`, `scheduler.rs` — each entry with a cited rationale naming
its lawful routing role. `agent/no_human_surface.rs` is not allowlisted because its
apply helper is test-only in the live checkout.

### 3. Enrollment + synthetics

Enroll in `META_LOCK_REGISTRY` with per-seam live witnesses; add the
non-allowlisted-injection and parity-divergence synthetics.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Any change to `events/apply.rs` or the apply functions themselves (the mutators
  are correct; the guard is the defect).
- `accepted_action_appends_before_authoritative_apply` ordering semantics
  (untouched).
- The witness-rule mechanics themselves (ticket -001, dependency).

## Acceptance Criteria

### Tests That Must Pass

1. The injection synthetic (`apply_agent_event(` in a non-allowlisted source) fails
   the scan; the parity synthetic (fifth `pub fn apply_*` unscanned) fails the
   census.
2. All four mutator tokens are scanned across core production sources; the
   allowlist carries exactly the enumerated seams, each with a rationale; current
   tree passes (zero violations).
3. The registry entry's per-seam live witnesses are nonzero against the current
   tree; removing a seam's live apply call drops its witness below minimum
   (synthetic).
4. The four workspace gates pass.

### Invariants

1. The forbidden-token set and `apply.rs`'s public mutator surface cannot diverge
   silently.
2. Every allowlisted seam is witnessed by at least one live apply call, measured by
   the scan itself.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — derived-set parity
   census, injection synthetic, per-seam witness enrollment + drop synthetic.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Implemented a derived post-seed mutation perimeter scanner in
`anti_regression_guards.rs`. The guard now derives its mutator token set from
`events/apply.rs` public `pub fn apply_*` signatures, scans core production sources,
requires rationale-bearing allowlist entries for the live production seams, and
counts one live witness per allowlisted seam through `META_LOCK_REGISTRY`.

Added synthetics proving the guard fails for a non-allowlisted
`apply_agent_event(` call, for a new public `apply_*` mutator absent from the scan's
token set, and for a live-seam witness drop. Corrected the spec/ticket evidence for
`agent/no_human_surface.rs`: its apply usage is currently test-only, so it is not a
production allowlist seam.

Verification:

1. `cargo test -p tracewake-core --test anti_regression_guards` passed.
2. `cargo fmt --all --check` passed.
3. `cargo clippy --workspace --all-targets -- -D warnings` passed.
4. `cargo build --workspace --all-targets --locked` passed.
5. `cargo test --workspace` passed.
