# 0018PHA3APROWIT-001: Fail-closed witness table and honest workplace-presence facts

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`agent/transaction`, `agent/no_human_surface`, `agent/htn`, scheduler wiring as surfaced); new source guards; context-hash / golden checksum updates
**Deps**: `specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (ORD-HARD-035)

## Problem

`no_human_surface.rs::add_role_assignment_notice` mints `actor_at_workplace`, `assigned_workplace_known`, and `at_workplace` as `ActorKnownFact::observed_now(...)` facts stamped at `self.decision_tick`, citing the `RoleAssignmentNoticeRecorded` notice's `source_event_ids` — a record event "witnessing" a physical-presence observation. They pass the witness audit only because `transaction.rs::witness_kind_allowed` ends `_ => true`: any fact stable_id without an explicit arm passes with any cited real event. That default is itself a structural open door — every future fact kind silently bypasses witness compatibility (INV-102 false provenance; foundation doc 14: witness compatibility must fail closed). The work-block HTN path consumes these facts (`htn.rs::resolve_condition` requires `modeled_fact_proof` for `ActorAtWorkplace`/`AtWorkplace`/`AssignedWorkplaceKnown`), so the fix must keep the canonical work routine functioning.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `a9c62e0`: `witness_kind_allowed` (transaction.rs) carries explicit arms for `actor_current_place_visible`, `agent_needs_present`, the frame/intention families — and ends `_ => true`; `add_role_assignment_notice` (no_human_surface.rs) mints the three presence facts as `observed_now` at `self.decision_tick` from the notice's `SourceEventIds` when `place_id == self.current_place_id`; `provenance_class_mismatch_diagnostic` fires only on `fact.tick() < decision_tick`, so decision-tick stamping evades it. The genuine perception witness is locatable: `scheduler.rs::latest_current_place_perception_event_id` already cites the same-tick `ObservationRecorded` for the framing fact.
2. Spec 0018 ORD-HARD-035 (required correction + structural lock) and §9 risk note: the fix direction is the implementer's recorded choice — (a) perception-witnessed `observed_now` (cite the actor's same-tick `ObservationRecorded`) or (b) honest downgrade to the notice-derived class (`routine_assignment`/remembered) with HTN conditions accepting that class. Either way the table default inverts to `_ => false`.
3. Cross-artifact boundary under audit: the witness-compatibility contract between the surface builder (`no_human_surface.rs`, which mints facts and cites sources) and the transaction audit (`transaction.rs::provenance_witness_mismatch_diagnostic`, which enforces source honesty) — one declared witness set per fact stable_id, enforced fail-closed.
4. INV-102 restated: every action-relevant cognition input carries provenance sufficient for replay and debug review; a citation to a non-witnessing event reviews to nothing and is a rejection condition. Foundation doc 14: a string label is never sufficient proof; witness compatibility fails closed.
5. Fail-closed actor-knowledge and deterministic-replay surfaces both touched: the enforcement surface is the decision transaction's witness audit (typed `Stuck`, `blocker_code = provenance_witness_mismatch`) and the table inversion strengthens it; the fact-class/witness change shifts `compute_holder_known_context_hash` inputs, so decision-trace context hashes and golden no-human checksums reprice once — `replay/rebuild.rs::rebuild_decision_context_hash` re-derives the corrected facts deterministically, and byte-stability is re-established at the honest classes. No knowledge is removed from the actor (no availability change), so no leakage-direction change; epistemic-leakage prevention is strengthened, not weakened.
6. Adjacent contradiction classified: the embodied `ActorKnownWorkplaceFact` (string-only `source_key`, no event witness, freshness exemption in `perception.rs`) is the same epistemic-hygiene gap on the embodied path — separate ticket `tickets/0018PHA3APROWIT-007.md` (ORD-HARD-042), which depends on this ticket's witness conventions. This ticket changes the no-human surface only.

## Architecture Check

1. Inverting the table default to `_ => false` with an explicit arm per fact stable_id makes false witnessing structurally impossible for future fact kinds — the census (every stable_id constructed in the surface builder has an arm) turns "forgot to declare a witness set" into a CI failure instead of a silent pass. Fixing the three presence facts at the minting site (honest witness or honest class) is cleaner than special-casing them in the audit: the audit stays generic, the builder carries the semantics. The chosen fix direction (option (a) or (b) from Assumption 2) must be recorded in this ticket's implementation notes and the 0018 acceptance artifact.
2. No backwards-compatibility aliasing/shims: the wildcard-true arm is removed, not wrapped; no "legacy witness mode" remains.

## Verification Layers

1. INV-102 witness honesty -> unit test: each workplace-presence fact citing only the role-notice event yields typed `Stuck(provenance_witness_mismatch)` before proposal construction (option (a)), or the fact is minted in the notice-derived class and passes with its true witness (option (b)).
2. Fail-closed default -> unit test: a fact with an unknown stable_id citing any real event fails the witness audit (the `_ => false` path).
3. Census parity -> new guard in `anti_regression_guards.rs`: every `ActorKnownFact` stable_id constructed in `no_human_surface.rs` has an explicit `witness_kind_allowed` arm; plus a source guard banning a wildcard-true arm in `witness_kind_allowed`.
4. INV-018 deterministic replay -> the decision context-hash gate passes over every golden no-human run at the corrected facts; golden checksum diffs explained in the acceptance-artifact ledger (spec §7 item 1).
5. Work-routine continuity -> `no_human_day_001` still completes its work block (HTN conditions resolve against the honest fact class); `cargo test --workspace`.

## What to Change

### 1. Invert the witness table default

`transaction.rs::witness_kind_allowed`: replace `_ => true` with `_ => false`; add explicit arms for every legitimate fact stable_id currently minted (enumerate from `no_human_surface.rs`), each naming its genuine witnessing `EventKind` set.

### 2. Honest workplace-presence facts

In `add_role_assignment_notice`, per the recorded fix direction: (a) mint the three presence facts as `observed_now` citing the actor's same-tick `ObservationRecorded` (thread the perception event id into the builder — `scheduler.rs` already locates it via `latest_current_place_perception_event_id`), or (b) mint them via `routine_assignment`/remembered with the notice witness and update `htn.rs::resolve_condition`'s `modeled_fact_proof` acceptance for `ActorAtWorkplace`/`AtWorkplace`/`AssignedWorkplaceKnown` to the honest class. The HTN update lands in this same diff (spec §9: the canonical day must not stall at the work block).

### 3. Census guard and wildcard ban

`anti_regression_guards.rs`: a stable_id↔witness-arm census test (parse the builder's constructed stable_ids and the table's arms; fail on any unmatched id) and a source guard asserting `witness_kind_allowed` contains no wildcard-true arm.

### 4. Golden updates

Update context-hash / golden checksum expectations surfaced by the corrected facts (`golden_fixtures_run.rs` and any checksum constants), with per-fixture diffs recorded for the acceptance artifact.

## Files to Touch

- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify)
- `crates/tracewake-core/src/agent/htn.rs` (modify — only under fix direction (b), or to assert class acceptance under (a))
- `crates/tracewake-core/src/scheduler.rs` (modify — witness threading, as surfaced by the chosen direction)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — expectations, as surfaced)

## Out of Scope

- The embodied `ActorKnownWorkplaceFact` witness/freshness residue (ticket `0018PHA3APROWIT-007`).
- Episode payload materialization and version gates (tickets `-003`, `-004`).
- Any change to what the actor knows or can plan — availability is preserved; only witness/class honesty changes (INV-028).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core provenance_witness` — notice-only presence fact fails closed (or passes in its honest class with true witness, per recorded direction); unknown stable_id fails closed.
2. `cargo test -p tracewake-core witness_kind` + the new census/wildcard guards in `anti_regression_guards.rs`.
3. `cargo test -p tracewake-content --test golden_fixtures_run` — canonical day completes its work block; context-hash gate green at corrected facts.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. `witness_kind_allowed` has no wildcard-true arm; every fact stable_id constructed by the surface builder has an explicit declared witness set.
2. Every `observed_now` fact in a decision trace cites an event kind that can genuinely witness it; record-derived facts carry record-derived classes.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/transaction.rs` (unit tests) — notice-only presence-fact mismatch; unknown-stable_id fail-closed path.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — stable_id↔arm census; wildcard-true ban.
3. `crates/tracewake-content/tests/golden_fixtures_run.rs` — updated expectations; existing `no_human_decision_context_hash_gate_fails_when_source_evidence_tampered` must remain green.

### Commands

1. `cargo test -p tracewake-core provenance_witness`
2. `cargo test --workspace`
