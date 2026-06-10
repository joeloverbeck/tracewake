# 0017PHA3ATICLED-008: Lock-layer durability — witness field type, workspace bans, pinned mutants

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`agent/actor_known`, `agent/transaction`, anti-regression guard suite); CI workflow; mutants baseline data file
**Deps**: `archive/tickets/0017PHA3ATICLED-005.md`, `archive/tickets/0017PHA3ATICLED-006.md`, `archive/tickets/0017PHA3ATICLED-007.md` (locks the finished behavioral surface; transitive over `-001`…`-004`); `specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (ORD-HARD-032)

## Problem

Four lock-layer residuals survive 0016. (1) `ActorKnownFact` stores `source_event_ids: Vec<EventId>` raw even though constructors take the `SourceEventIds` witness; the in-crate `unbacked_for_rejected_test_only` path writes `Vec::new()`, and the runtime backstop `dangling_provenance_diagnostic` flat-maps the ids and `find`s non-resolving entries — an *empty* list yields nothing and passes; the only guard is a single-spelling string count (`guard_018` matching the literal `"source_event_ids: Vec::new()"`), defeated by `vec![]` or `Default::default()`. (2) The truth-read bans (`state.workplaces` etc.) run only against the GuardedLayer globs — cognition added to an already-*Exempt* core file (`state.rs`, `view_models.rs`, `actions/pipeline.rs`) escapes them; the census proves classification completeness, not exempt-file innocence. (3) Several positive-presence guards assert indentation-coupled multi-line literals a rustfmt change voids silently. (4) The `mutants-lock-layer` CI job installs `cargo-mutants` unpinned, and nothing fails when the missed-mutant count grows past the recorded baseline.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: `actor_known.rs` field `source_event_ids: Vec<EventId>` with the `Vec::new()` path; `guard_018_actor_known_facts_require_source_event_witness` asserts `.matches("source_event_ids: Vec::new()").count()`; `dangling_provenance_diagnostic` cannot flag an empty witness list; `WORKSPACE_SOURCE_CLASSIFICATIONS` + `is_guarded_layer_source` scope the content bans to `agent/`, `scheduler*`, `projections*`; `.github/workflows/ci.yml` `mutants-lock-layer` job runs `cargo install cargo-mutants --locked` (no version pin) on `workflow_dispatch`/`schedule`; `.cargo/mutants.toml` exists; the baseline (`reports/0016_ord_hard_025_mutants_baseline.md`) records 581 caught / 145 missed, dispositioned once with no growth gate.
2. Spec 0017 §ORD-HARD-032 and §5 tiers 1/3/5: store the `SourceEventIds` newtype on the fact; empty witness ⇒ `ProvenanceDangling`; workspace-wide truth-accessor bans with a recorded reader allowlist; whitespace-normalized guard scans with documented evasion modes; pinned mutants version + committed miss-set with a growth-failure gate; `cargo mutants --in-diff` on PRs touching guarded layers.
3. Cross-artifact boundary under audit: the lock layer ↔ all-three-crates production tree — the guards' perimeter and normalization rules are the contract that every behavioral ticket in this batch relies on staying closed.
4. INV-102 restated for the witness retype: cognition inputs require provenance, and for action-relevant cognition missing provenance is a rejection condition — an empty witness list is missing provenance and must be unconstructable or rejected, not merely unfashionable.
5. Fail-closed surface touched (strengthened): the transaction's dangling-provenance path gains the empty-witness rejection; the field retype makes the unbacked state unrepresentable in production. Deterministic replay untouched (no event or checksum content changes; guards and CI only). Golden runs must pass unchanged.
6. Schema/shape change keyed separately: `ActorKnownFact.source_event_ids` retypes `Vec<EventId>` → `SourceEventIds` (non-empty by construction). Crate-internal, breaking-internal: all in-workspace consumers (constructors, `source_event_ids()` accessor, serialization into trace records, the rejected/test-only path) update in this same diff — a local compile-atomic unit per the decomposition sizing rules (deliberate size, flagged at approval).
7. Removal blast radius keyed separately: the `unbacked_for_rejected_test_only` empty path is removed (or demoted to an explicit `Option`/`Unproven` representation); repo-wide grep for `unbacked_for_rejected_test_only` and `unproven` confirms all call sites land in this ticket's Files to Touch — any guard or doc citing the old spelling (including `guard_018`'s literal) is updated here.

## Architecture Check

1. Type-level impossibility (the field holds the witness type) converts the highest-value string ban into a compile error — the spec's preferred lock tier — and makes `guard_018`'s fragile spelling-count redundant rather than load-bearing; the guard is retained only as a tripwire with its evasion modes documented inline. Workspace-wide accessor bans with an explicit reader allowlist close the exempt-file relocation hole without re-classifying the whole tree. A committed miss-set diff gate makes the mutants baseline a ratchet instead of a one-time photograph.
2. No backwards-compatibility aliasing/shims: the raw-`Vec` field and the empty-witness constructor path are removed, not deprecated; the unpinned CI install is replaced by a pinned version, not supplemented.

## Verification Layers

1. INV-102 empty-witness rejection -> compile-level: `SourceEventIds` field type (non-empty invariant private to the newtype) + compile-fail doctest for raw construction; runtime backstop unit test: a transaction fed a zero-witness fact (test-constructed via the explicit `Unproven` representation) fails closed `ProvenanceDangling`.
2. Exempt-file relocation hole -> new `cognition_inputs_are_context_backed` guard over all `production_sources()` across the three crates: truth-table accessors (`state.workplaces`, `state.food_supplies`, `state.sleep_affordances`, and siblings) banned outside a recorded reader allowlist; test seeds a synthetic violation to prove the guard can fail.
3. Formatting fragility -> guard scans run over whitespace-normalized source (collapse runs); a normalization unit test proves an indentation-shifted call site still matches.
4. Mutants ratchet -> CI: pinned `cargo-mutants` version; scheduled job diffs `mutants.out` misses against the committed miss-set and fails on growth; `--in-diff` job on PRs touching guarded layers.

## What to Change

### 1. Witness field retype (`agent/actor_known.rs` + consumers)

`source_event_ids: SourceEventIds` on `ActorKnownFact`; constructors stop unwrapping to raw `Vec`; the rejected/test-only path becomes an explicit typed representation that the transaction rejects; `dangling_provenance_diagnostic` (`agent/transaction.rs`) additionally flags any fact whose witness set is empty (defense in depth behind the type).

### 2. Workspace-wide cognition-input bans (`anti_regression_guards.rs`)

`cognition_inputs_are_context_backed` over all three crates' production sources with the recorded reader allowlist (state definitions, projection/perception derivation sites); document each retained text scan's known evasion modes inline; normalize whitespace before substring assertions and rewrite the indentation-coupled call-shape guards against the normalized form.

### 3. Mutants operationalization (`.github/workflows/ci.yml`, `.cargo/`)

Pin the `cargo-mutants` version; commit the baseline miss-set at `.cargo/mutants-baseline-misses.txt`; the scheduled job fails when a previously-caught mutant goes missed; add a PR-scoped `cargo mutants --in-diff` job gated to guarded-layer paths.

## Files to Touch

- `crates/tracewake-core/src/agent/actor_known.rs` (modify)
- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- In-workspace consumers of the retyped field (modify — compile-surfaced set within `crates/tracewake-core/src/agent/`; local compile-atomic unit)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `.github/workflows/ci.yml` (modify)
- `.cargo/mutants-baseline-misses.txt` (new)
- `.cargo/mutants.toml` (modify — only if the in-diff job needs a scoped profile, as surfaced)

## Out of Scope

- New behavioral semantics (this ticket locks surfaces; tickets `-001`…`-007` changed them).
- The generative test tier (ticket `-009`, parallel — disjoint file set; this ticket owns all `ci.yml` changes).
- Re-running the full mutants baseline against the post-batch tree (capstone evidence, ticket `-011`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core cognition_inputs_are_context_backed` — workspace-wide ban passes on the real tree and fails on a seeded synthetic violation.
2. `cargo test -p tracewake-core --test anti_regression_guards` — full guard suite green, including normalized-scan and retained-tripwire forms; compile-fail doctests for raw witness construction.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. An `ActorKnownFact` with an empty witness set is unrepresentable in production code; the runtime transaction rejects it anyway (defense in depth).
2. The truth-accessor ban perimeter is the whole workspace production tree minus a recorded allowlist; the mutants miss-count can only shrink without an explicit dispositioned update to the committed miss-set.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/actor_known.rs` — compile-fail doctest (raw/empty witness construction); retype unit coverage.
2. `crates/tracewake-core/src/agent/transaction.rs` (unit test) — empty-witness `ProvenanceDangling` fail-closed.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — `cognition_inputs_are_context_backed`; whitespace-normalization self-test; updated guard_018-family tripwires.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-10

What changed:

- Retyped `ActorKnownFact.source_event_ids` from raw `Vec<EventId>` storage to the `SourceEventIds` witness type.
- Removed the empty-source rejected/test-only constructor path; the unproven helper now carries an explicit nonempty rejected-test-only sentinel witness.
- Added a compile-fail doctest proving raw `Vec<EventId>` construction cannot satisfy actor-known fact construction.
- Strengthened transaction dangling-provenance detection with an explicit empty-witness backstop before unresolved-event detection.
- Added a workspace-wide cognition truth-accessor guard with an explicit allowlist and synthetic exempt-file violation proof.
- Updated the actor-known provenance guard to use whitespace-normalized source matching and to reject raw/empty source-vector spellings.
- Pinned `cargo-mutants` to `27.1.0`, added a guarded-layer PR `--in-diff` mutation job, and committed the 145-entry accepted missed-mutant baseline for scheduled/manual ratchet comparison.

Deviations from original plan:

- The explicit `UnprovenPhysicalTruth` test helper was kept because existing hidden-truth tests use it as the negative source-class fixture; it now uses a nonempty sentinel source id rather than an empty witness.
- The mutation baseline file was derived from the existing accepted 0016 baseline report instead of rerunning mutants; rerunning the post-batch baseline remains out of scope for this ticket and is owned by the capstone evidence ticket.

Verification:

- `cargo fmt --all --check` — passed
- `cargo test -p tracewake-core cognition_inputs_are_context_backed` — passed
- `cargo test -p tracewake-core --test anti_regression_guards` — passed
- `cargo clippy --workspace --all-targets -- -D warnings` — passed
- `cargo build --workspace --all-targets --locked` — passed
- `cargo test --workspace` — passed
