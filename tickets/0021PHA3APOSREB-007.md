# 0021PHA3APOSREB-007: Typed place concealment replacing display-label perception prose

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`state` place record, `agent/perception`), `tracewake-content` (place schema, loader, serialization, fixtures); guard_014 family extension; conformance-index row
**Deps**: `specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-078)

## Problem

Perceptual ground truth is decided by prose: `is_visible_exit_target`
(`agent/perception.rs`) gates `visible_exit` `ObservationRecorded` emission on
`!place.place_id.as_str().contains("hidden") &&
!place.display_label.to_lowercase().contains("hidden")`. A place whose display label
happens to contain "hidden" — or a genuinely hidden place relabeled — silently changes
what actors can perceive, believe, and route through. Letter-level INV-022 (raw prose
is not authoritative state) on a perception channel (`ORD-HARD-078`). The existing
display-text guard (guard_014) covers projections only, and the `hidden_*` fixtures
bake the naming convention in, so every test passes.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: the substring checks at `is_visible_exit_target`;
   `guard_014_no_human_metrics_do_not_scan_display_text` scopes to projections;
   no typed concealment attribute exists on the place schema
   (`crates/tracewake-content/src/schema.rs`) or the `PhysicalState` place record
   (`crates/tracewake-core/src/state.rs`); content serialization is hand-rolled and
   fail-closed (unknown fields rejected), so a new field must be explicitly wired
   through schema, loader (`load.rs`), and canonical serialization
   (`serialization.rs`) with roundtrip coverage.
2. Verified against spec 0021 (reassessed 2026-06-11; operator-verified): finding
   ORD-HARD-078, including the reassessment M1 tightening — the attribute's
   authoring contract must itself be explicit: either required on every place (the
   loader fails closed on absence) or a documented, registered schema default; never
   a silent fallback (which would recreate the `ORD-HARD-085` silent-default shape).
3. Cross-artifact boundary under audit: the place-concealment contract spanning the
   content schema (authoring), `PhysicalState` (authoritative truth), and the
   perception channel (emission gating) — one typed attribute, authored explicitly,
   consumed only by the perception gate.
4. INV-022 restated: raw prose may render claims; it may not define hidden facts —
   a display label deciding visibility is prose defining perceptual ground truth.
   INV-008-adjacent: identifiers are identifiers, not semantics — the `place_id`
   substring check goes too.
5. Perception-emission / replay surface touched: the gate's *criteria* change from
   prose to the typed attribute. Fixtures author the attribute to match current
   intended semantics (today's `hidden_*` places are the concealed set), so canonical
   event logs should be unchanged when authoring is faithful — any divergence is a
   genuine semantic fix and reprices with the per-actor ledger treatment. Emission
   stays deterministic (attribute is authored state). Fail-closed loading per the M1
   contract means a place omitting the attribute is a validation rejection (or a
   registered documented default — the recorded choice), never a silent guess.
6. Schema extension (real item-6 case): the place schema and `PhysicalState` place
   record gain the typed concealment attribute; consumers are the loader, canonical
   serialization/roundtrip, fixture constructors, and the perception gate.
   Additive-vs-breaking: breaking for authoring if `required` is chosen (every
   fixture place must author it — this ticket updates all in-repo fixtures);
   additive if the documented-registered-default is chosen. The choice is the
   spec-assigned recorded decision (M1); either way no silent third option.

## Architecture Check

1. A typed attribute on authoritative state is the only INV-022-compliant carrier
   for concealment semantics: prose and ids return to being presentation/identity
   only, and the perception gate reads modeled state. Extending guard_014's family
   to perception (no `display_label`/id-substring branching in any emission path)
   locks the *class* — the next perception-adjacent emission site cannot reintroduce
   prose gating.
2. No backwards-compatibility aliasing/shims: the substring checks are deleted, not
   kept as a fallback when the attribute is absent — absence is handled by the
   explicit authoring contract, not by prose.

## Verification Layers

1. INV-022 (typed gating) -> fixture with a misleading label (label contains
   "hidden", attribute says visible — and the inverse) proving visibility follows
   the attribute, not the prose.
2. Lock (family closure) -> guard_014 family extension: source guard asserting no
   `display_label` / id-substring branching in `perception.rs` emission paths;
   synthetic violation fails.
3. INV-020 (authoring contract, M1) -> loader negative test: a place omitting the
   attribute is rejected (or the registered default is asserted as documented — per
   the recorded choice); serialization roundtrip covers the new field.
4. Replay stability -> canonical no-human goldens unchanged under faithful
   authoring (or repriced with per-actor ledger diff if semantics genuinely
   corrected).

## What to Change

### 1. Schema + state + loader + serialization

Add the typed concealment attribute to the content place schema, `PhysicalState`'s
place record, the loader, and canonical serialization (with roundtrip and
fail-closed-absence handling per the recorded authoring-contract choice).

### 2. Perception gate

`is_visible_exit_target` reads the attribute; both substring checks deleted.

### 3. Fixtures

Author the attribute across all in-repo fixture places (concealed set = today's
intended-hidden places); add the misleading-label fixture.

### 4. Locks and documentation

guard_014 family extension to perception; conformance row (typed place concealment)
in `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/serialization.rs` (modify)
- `crates/tracewake-content/src/fixtures/` (modify — all fixture files authoring places; discovered set, parent dir exists)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — guard_014 family extension)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- Policy-table dispatch in the same file (ticket 0021PHA3APOSREB-005) — coordinate
  the `perception.rs` mechanical merge.
- Other prose-as-state sites (none found by the audit beyond this gate; the
  content-crate substring-keyed *validation* gate is `ORD-HARD-098b`, ticket
  0021PHA3APOSREB-009).

## Acceptance Criteria

### Tests That Must Pass

1. Misleading-label fixture: visibility follows the typed attribute in both
   directions (prose says hidden / attribute visible, and inverse).
2. Loader negative test per the recorded authoring contract (absence rejected, or
   default registered + documented); serialization roundtrip green.
3. guard_014 perception extension green; synthetic prose-branching violation fails.
4. Canonical no-human goldens unchanged (or repriced with ledger diff + rationale).
5. `cargo test --workspace` green.

### Invariants

1. No perception emission path branches on `display_label` or id substrings.
2. Every authored place's concealment is typed, explicit (or a documented registered
   default) — never inferred from prose.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/perception.rs` (mod tests) — attribute-gated
   visibility tests.
2. `crates/tracewake-content/tests/fixtures_load.rs` — authoring-contract negative
   + roundtrip.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — guard_014 family
   extension + synthetic.

### Commands

1. `cargo test -p tracewake-core perception`
2. `cargo test -p tracewake-content --test fixtures_load`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
