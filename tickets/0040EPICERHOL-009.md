# 0040EPICERHOL-009: EPI-08 — possession parity and cognition-neutral controller binding

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2.
**Deps**: archive/tickets/0040EPICERHOL-001.md

## Problem

Spec 0040 audit point EPI-08 (§5) requires certifying that possession selects which actor a human may command without replacing the actor's cognition, granting player knowledge, resetting intention state, disclosing prior possessed actors' knowledge, changing semantic action availability, or bypassing normal validation/event application; and that a human-possessed actor and an autonomous actor in the same epistemic and physical state receive the same embodied context and affordances and are judged by the same pipeline. This ticket collects the EPI-08 witnesses and writes the EPI-08 section of the artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-08 seam files verified present at `ba9fe1c` (2026-06-19): `controller.rs`, `state.rs`, `projections.rs`, `view_models.rs`, `epistemics/knowledge_context.rs`, `agent/transaction.rs`, `actions/proposal.rs`, `actions/pipeline.rs`, `crates/tracewake-tui/src/app.rs`, `crates/tracewake-tui/src/input.rs`, `crates/tracewake-tui/src/run.rs`. Fixtures `possession_parity_001`, `possession_does_not_reset_intention_001`, `debug_attach_001` and test binaries `acceptance_gates`, `embodied_flow`, `command_loop_session` all verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-08 section only. Section wording follows spec §5 EPI-08 and the `0003` evidence fields.
3. Shared boundary under audit: the controller-binding-vs-cognition seam — `controller.rs` attach/detach/authorize against the actor cognition owned by `transaction.rs`/`knowledge_context.rs`, with the TUI command source in `app.rs`/`input.rs`/`run.rs`. The acceptance artifact must state the exact fingerprint scopes compared (controller-binding state is the only expected difference; agent/cognition/projection-checksum/context tuple must be equal).
4. `INV-108` (human possession is cognition-neutral) and `INV-006` (possession transfers no world knowledge) motivate this audit point, with `INV-007` (player actions NPC-possible), `INV-065`…`INV-070`, `INV-101`, `INV-107`. Restated: the human path may select among actor-available actions but must not submit a semantic action absent from the autonomous actor's same-state availability set without ordinary proposal rejection.
5. This ticket audits the possession-neutrality / no-leak surfaces as an **evidence consumer**: it proves possess-A-then-B carries none of A's notebook/beliefs/observations/provenance/proposals/why-not/debug-truth into B (no leakage), attach/detach/reattach leaves actor-known context/actions/intentions/projection checksum stable (cognition-neutral), and a human run and autonomous run from the same state are judged by the same pipeline with parity holding pairwise under hidden-truth variation. It modifies no enforcement. Any failed seam is recorded `fail` and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated EPI-08 evidence ticket writing only its artifact section is the accepted certification-audit shape; it adds the human-vs-autonomous parity, prior-actor-carryover, and attach/detach-stability witnesses without re-running the whole vocabulary.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2).

## Verification Layers

1. `INV-108`/`INV-006` cognition-neutral binding -> replay/golden-fixture check: `possession_parity_001` runs human-commanded and autonomous paths from the same actor-known context + physical state, proving equal holder-known context ID/hash/frontier, embodied view, semantic action availability, validation rules, and equivalent event/state outcome for the same semantic action (`acceptance_gates`, `embodied_flow`).
2. intention/cognition persistence -> replay/golden-fixture check: `possession_does_not_reset_intention_001` proves attach/detach/rebind does not reset actor intention/belief/observation/projection; attach→detach→reattach leaves context/actions/intentions/projection checksum stable.
3. `INV-006` no prior-actor carryover -> manual review + replay/golden-fixture check: possess A then B — B receives none of A's notebook/beliefs/observations/context provenance/proposals/why-not/debug truth; no prior-actor source appears in B's provenance table (`hidden_truth_gates`).
4. `INV-107` debug-attach neutrality + authorization -> manual review: `debug_attach_001` shows controller/debug attachment visible only in the authorized debug channel, embodied cognition unchanged; an unauthorized/mismatched/stale/forged binding is rejected without creating actor knowledge or accepted events (`command_loop_session`).

## What to Change

### 1. Collect EPI-08 witnesses

Run the EPI-08 commands and record before/after controller bindings, agent/cognition fingerprints, epistemic projection checksum, context-parity tuple, embodied view/action set, selected semantic action, proposal, validation, event-log segment, and resulting state. Rebuild each autonomous and human-run projection/context from its serialized event input and prove the same-state parity relation survives replay. Explain any expected difference by fingerprint scope. Prove no prior-actor source appears in the newly possessed actor's provenance table. Vary hidden truth while holding actor-observable history fixed in both runs; parity holds pairwise before observation.

### 2. Write the EPI-08 section of the acceptance artifact

Populate the EPI-08 section with the §9.2 ledger fields per witness (positive parity, prior-actor-carryover, attach/detach stability, unauthorized-binding negatives, replay), each row carrying exactly one §9.2 status, with the exact compared fingerprint scopes stated.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-08 section only)

## Out of Scope

- Other audit points and the §6.1 compile-fail corpus matrix (owned by their own tickets).
- The §9.4 verdict table and aggregate verdict (owned by `-015`); the relational capstone possession-source pair (owned by `-013`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test acceptance_gates` — human-vs-autonomous parity.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — no prior-actor carryover; parity under hidden-truth variation.
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` and `--test no_human_capstone` — parity survives replay; autonomous run advances without a human.
4. `cargo test --locked -p tracewake-tui --test embodied_flow` and `--test command_loop_session`, plus `cargo test --locked -p tracewake-content --test golden_fixtures_run` — possession fixtures + unauthorized-binding rejection.

### Invariants

1. Possession changes controller binding/command source only; agent cognition, context tuple, action availability, validation rules, and projection checksum are equal between human and autonomous same-state runs (`INV-108`/`INV-006`).
2. Possess-A-then-B carries none of A's cognition/provenance into B; attach/detach/reattach is cognition-stable; an unauthorized binding creates no actor knowledge or accepted event (`INV-006`/`INV-107`).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-08 existing gates/fixtures and records the before/after binding fingerprints, human-vs-autonomous parity tuple, and no-prior-actor-carryover provenance table as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-core --test acceptance_gates`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates && cargo test --locked -p tracewake-tui --test embodied_flow`
3. `cargo test --locked -p tracewake-tui --test command_loop_session` (controller command-source / unauthorized-binding boundary)
