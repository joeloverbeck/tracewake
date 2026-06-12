# 0022PHA3ABASTRI-005: Planner hidden-truth gate adversarial restoration

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — truth-firewall acceptance gates (`hidden_truth_gates.rs`), test-oracle guards (`anti_regression_guards.rs`)
**Deps**: 0022PHA3ABASTRI-004

## Problem

The 0021 gate rebuild (`ORD-HARD-068`) silently de-fanged the planner-level
hidden-truth gates (`ORD-HARD-105`): `hidden_truth_gates.rs::context` declares
parameters `_known_edges` and `_known_food_sources` and ignores both, so
`hidden_food_closed_container_is_not_actor_known_food_source` and
`hidden_route_edge_absent_from_actor_known_edges_blocks_route_plan` now build contexts
with no food/route knowledge at all and assert the planner rejects — trivially true
with or without the firewall. Doctrine requires adversarial truth present in
authoritative state but absent from holder-known context. Companion residue
(`ORD-HARD-114`): the actor-known constructor guard's stated contract ("no pub
producer outside the pair") is broader than what its literal-list scan enforces.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `_known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>` and
   `_known_food_sources: BTreeSet<String>` are underscore-bound in
   `crates/tracewake-core/tests/hidden_truth_gates.rs::context`; the embodied gates
   (`embodied_affordances_exclude_hidden_food_in_closed_container`,
   `debug_truth_never_enters_holder_known_context_hash`) retain real adversarial truth
   — the regression is planner-side only. `actor_known_context_constructor_violations`
   (`anti_regression_guards.rs`) scans the three retired builder names plus the
   literal `from_observed_parts(`, with allowed sites `agent/actor_known.rs` and
   `agent/no_human_surface.rs`; the real lock is `from_observed_parts`'s `pub(crate)`
   visibility.
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-105`/`114`
   (operator-verified) and `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`'s
   adversarial-truth acceptance requirement.
3. Shared contract under audit: the truth-firewall acceptance surface — gate contexts
   built from real applied events (`apply_epistemic_event` / `role_notice_event`
   pattern) discriminating known from hidden entries, guarded by
   `guard_0021_hidden_truth_gates_use_event_log_provenance`.
4. Constitutional invariants restated: INV-099 (truth may validate, not plan),
   INV-100 (hidden-truth cognition forbidden), INV-101 (sealed actor-known context),
   INV-102 (cognition inputs require provenance) — the gates exist to prove these; a
   gate proving "empty context rejects" proves none of them.
5. This ticket touches the actor-knowledge-filtering acceptance surface directly: the
   restoration strengthens leakage prevention (re-arms discrimination) and must not
   weaken any existing gate; per spec §9, any gate that *fails* once re-armed is
   treated as a potential product defect first (Enforcement reading), not adjusted to
   pass — such a failure is triaged before any test change.
6. Adjacent contradiction classification: a latent planner defect surfaced by
   re-arming would be a separate bug requiring its own ticket (and possibly a spec
   amendment to 0022's verdict); record it rather than absorbing it here.
7. Change rationale (no silent retcon): the gates' assertion surface changes because
   the 0021 rebuild dropped the adversarial dimension; the prior fabricating harness's
   discrimination intent is restored on honest provenance.

## Architecture Check

1. Seeding real fixture food/route knowledge through event-log provenance (the pattern
   the embodied gates already use) restores discrimination without resurrecting
   fabricated provenance — strictly better than both the pre-0021 harness (fabricated)
   and the post-0021 gates (vacuous). The witness self-check makes the discrimination
   itself asserted, so a future parameter going dead fails loudly.
2. No backwards-compatibility aliasing/shims: the dead parameters are either consumed
   or deleted — no tolerated dead surface.

## Verification Layers

1. Adversarial restoration (arch 03) -> replay/gate check: each planner gate runs
   against authoritative truth that exists in `PhysicalState`, is partially known via
   seeded events, and stays hidden where unknown; the gate asserts the known entry IS
   planner-visible and the hidden entry is NOT (discrimination witness).
2. Witness rule (R-29 / §5.1) -> guard check: a harness self-check asserts every
   `context()` caller passing a non-empty adversarial fixture has the fixture's hidden
   entries present in `PhysicalState` and absent from the built context; registered
   with the `-004` bijection census with a firing negative (a gate whose adversarial
   fixture is empty).
3. Constructor-guard contract (`ORD-HARD-114`) -> codebase grep-proof: the guard's
   documented contract matches what it enforces — either reworded to the literal-list
   + `pub(crate)` reality, or extended with a return-type scan
   (`pub fn … -> ActorKnownPlanningContext`) with a synthetic pub re-wrapper negative.
4. No-weakening backstop -> full gate suite: `cargo test -p tracewake-core --test
   hidden_truth_gates` green with strictly more assertions than before.

## What to Change

### 1. Re-arm `context()` in `hidden_truth_gates.rs`

Consume the `known_edges` / `known_food_sources` parameters: seed each entry through
real epistemic events (`apply_epistemic_event`, observation/notice constructors), so
the built context contains exactly the seeded knowledge. Gates that genuinely need no
seeded knowledge drop the dead parameters instead.

### 2. Restore discrimination in the two planner gates

`hidden_food_closed_container_is_not_actor_known_food_source`: authoritative state
carries both a known food source and the hidden closed-container food; assert the
known one is planner-reachable and the hidden one is rejected.
`hidden_route_edge_absent_from_actor_known_edges_blocks_route_plan`: seed a known
route edge; assert the known edge plans and the hidden edge blocks.

### 3. Witness self-check + census registration

Add the harness self-check (verification layer 2) and register the restored gates and
the self-check with the `-004` bijection census.

### 4. Resolve `ORD-HARD-114`

Per the spec's correction: keep `from_observed_parts` `pub(crate)` as the real lock,
and either reword `actor_known_context_constructor_violations`'s contract comment to
what it enforces or add the return-type scan with its synthetic negative (implementer
choice recorded in the implementation notes; the return-type scan is preferred if
cheap, since it upgrades the contract rather than downgrading the comment).

## Files to Touch

- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Any production cognition-path change (`scheduler.rs`, `no_human_surface.rs`,
  `actor_known.rs` stay untouched unless re-arming surfaces a product defect — which
  becomes its own ticket per Assumption 6).
- The policy-table behavioral locks (`0022PHA3ABASTRI-006`).
- The embodied gates (verified intact; not re-litigated).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test hidden_truth_gates` — both planner gates now
   assert known-visible AND hidden-rejected (discrimination, not empty-context
   rejection); witness self-check green.
2. `cargo test -p tracewake-core --test anti_regression_guards` — census registration
   + the empty-adversarial-fixture negative fires; constructor-guard contract
   resolution landed.
3. `grep -c "_known_edges\|_known_food_sources" crates/tracewake-core/tests/hidden_truth_gates.rs`
   returns 0 (parameters consumed or deleted — no underscore-bound survivors).
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every planner hidden-truth gate exercises adversarial truth present in
   `PhysicalState` and absent from the sealed context (INV-099–102 acceptance).
2. Gate contexts remain built exclusively from real applied events — no fabricated
   provenance returns.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — re-armed planner gates +
   witness self-check.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — constructor-guard
   resolution + census entries.

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed on 2026-06-12.

- Re-armed `hidden_truth_gates.rs::context` so `known_edges` and
  `known_food_sources` are consumed through real `ObservationRecorded` events applied
  to the epistemic projection.
- Restored the planner hidden-truth gates to assert known-visible and hidden-rejected
  behavior against authoritative adversarial truth.
- Added `planner_hidden_truth_fixture_witness_errors` and an empty-context negative
  so the harness fails when its adversarial fixture goes vacuous again.
- Registered the restored planner gates and witness with the meta-lock census added
  by `0022PHA3ABASTRI-004`.
- Resolved `ORD-HARD-114` by upgrading
  `actor_known_context_constructor_violations` with a public
  `ActorKnownPlanningContext` return-type scan and a synthetic public re-wrapper
  negative.

Verification run:

1. `cargo test -p tracewake-core --test hidden_truth_gates`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `grep -c "_known_edges\|_known_food_sources" crates/tracewake-core/tests/hidden_truth_gates.rs` (printed `0`; grep exits `1` on zero matches)
4. `cargo fmt --all`
5. `cargo test -p tracewake-core --test hidden_truth_gates`
6. `cargo test -p tracewake-core --test anti_regression_guards`
7. `cargo fmt --all --check`
8. `cargo clippy --workspace --all-targets -- -D warnings`
9. `cargo build --workspace --all-targets --locked`
10. `cargo test --workspace`
