# 0023PHA3AEMBLOC-008: Perception prose-scan laundering and consumed-key call-shape closure

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — test-oracle scans (`anti_regression_guards.rs`)
**Deps**: archive/tickets/0023PHA3AEMBLOC-001.md, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

Two scan-evasion residues (spec 0023 `ORD-HARD-127` medium, `ORD-HARD-130` medium):

- The perception prose-branch guard launders: a binding such as
  `let visible = place.display_label != "…";` followed by `if visible { … }` evades
  `perception_visibility_prose_branch_violations` (the `let` line has no
  branch-shape token per `source_line_is_branch_shape`; the `if` line has no
  `display_label`); and `branches_on_id_substring` requires the literal
  `.as_str().contains`/`.as_str().starts_with`, so a bare-`String`
  `display_label.starts_with("hid")` evades. The synthetic exercises only the
  `.to_lowercase().contains` shape. INV-022 latent hole (production emission paths
  verified clean today).
- The consumed-payload-key derivation treats a callee as payload-consuming only when
  the binding appears in the *argument list*
  (`call_arguments_include_payload_binding` strips `&`/`*`, matches
  argument-position `payload`); a method-receiver call (`payload.consume_into(state)`)
  or an alias rebinding (`let view = &payload; helper(view)`) escapes recursion. The
  `synthetic_oblique_payload_helper_call` negative covers the argument shape only.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: `source_line_is_branch_shape`'s token set (`if`/`else if`/
   `while`/`match`/`&&`/`||`/`.filter(`), the `.as_str()`-anchored
   `branches_on_id_substring`, `payload_helper_calls` +
   `call_arguments_include_payload_binding` bodies (all in
   `crates/tracewake-core/tests/anti_regression_guards.rs`); all five current
   exemption anchors verified covered end-to-end — both gaps are future-facing, no
   live false pass.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-127`/`130`
   (operator-verified at reassessment); production perception emission paths
   (`current_place_perception_events`, `observation_event`,
   `is_visible_exit_target`) gate on typed `VisibilityDefault` only — verified clean.
3. Shared contract under audit: two derivation-based scans in the shared guard file —
   the perception prose-branch perimeter (INV-022 enforcement over
   `src/agent/perception.rs`) and the consumed-key exemption-truth derivation
   (replay payload-coverage enforcement). Note: -004 extends
   `perception.rs` with new emitters — the widened prose scan must cover them, which
   it does structurally (module-wide scan), no Deps edge needed beyond -001.
4. Constitutional motivation restated: INV-022 (raw prose is not authoritative state
   — no production branching on display strings) and lock durability (R-28/R-29: the
   negatives cover the shapes the scans already catch — the `ORD-HARD-109` pattern,
   one scan over).
5. This ticket touches fail-closed test-oracle surfaces only (the prose perimeter and
   the replay payload-key derivation): both repairs tighten enforcement; no
   epistemic, replay, or product behavior changes.
6. Change rationale (no silent retcon): the scan predicates change because the
   spec's evidence constructs concrete evasions the current shapes cannot see;
   mandated by `ORD-HARD-127`/`130` required corrections.

## Architecture Check

1. For the prose scan, flagging any production line mentioning `display_label` that
   is not an allowlisted typed-PayloadField write (dropping the branch-shape gate for
   that token) is cleaner than enumerating more branch shapes: laundering through a
   binding is unbounded, while "no `display_label` outside payload writes" is a
   closed rule. For the consumed-key derivation, treating receiver position and
   simple `let` aliases as payload bindings closes the call-shape family rather than
   chasing one more shape.
2. No backwards-compatibility aliasing/shims: the old predicates are replaced; the
   widened rules carry their allowlists (typed-label payload writes) explicitly.

## Verification Layers

1. INV-022 laundering closure -> codebase test-proof: two synthetic negatives — a
   laundered `let`-binding branch and a bare-`String` `starts_with` — fail the
   widened scan, routed through the production scan path.
2. Consumed-key call-shape closure (R-28) -> codebase test-proof: negatives
   `helper_via_payload_receiver` and `helper_via_payload_alias` fail the derivation;
   the existing argument-shape negative stays green.
3. No-false-positive floor -> codebase test-proof: the current clean production
   modules still pass (live witness counts from -001 prove the scans matched real
   sites).

## What to Change

### 1. Widened prose-branch scan (`ORD-HARD-127`)

In `perception_visibility_prose_branch_violations`: flag any production line
mentioning `display_label` that is not an allowlisted typed-PayloadField write;
broaden substring detection to `.starts_with(`/`.ends_with(`/`.contains(` on any
`display_label`/`*_id` projection regardless of `.as_str()`. Add the two laundering
negatives.

### 2. Receiver/alias-aware consumed-key derivation (`ORD-HARD-130`)

In `payload_helper_calls`/`call_arguments_include_payload_binding`: treat a callee as
payload-consuming when the payload (or a binding aliased via
`let x = &payload;`/`let x = payload;`) is the receiver or any argument; track the
alias set. Add the receiver and alias negatives. Enroll all repaired locks/negatives
under the -001 registry.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Production perception changes (clean today; -004 owns the new emitters).
- The mutation-CI swallow scan (-009) and panic-guard token (-010).
- Method-call receivers like `payload.foo()` appearing in *future* anchors beyond
  the enumerated exemption set — covered structurally by the repaired derivation.

## Acceptance Criteria

### Tests That Must Pass

1. The laundered `let`-binding and bare-`String` `starts_with` negatives fail the
   widened prose scan; clean production perception passes
   (`cargo test -p tracewake-core --test anti_regression_guards`).
2. `helper_via_payload_receiver` and `helper_via_payload_alias` negatives fail the
   derivation; all five existing exemption anchors still verify covered.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. No production perception line can branch on `display_label`/id substrings through
   any binding or call shape without tripping the scan or carrying a typed-payload
   allowlist entry.
2. Every payload key consumed through any call shape (argument, receiver, alias) is
   attributed to the exemption derivation.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — widened prose predicate
   + two laundering negatives; receiver/alias derivation + two call-shape negatives;
   registry enrollment.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12
