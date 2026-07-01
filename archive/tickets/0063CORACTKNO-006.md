# 0063CORACTKNO-006: Observed co-present activity acceptance-evidence capstone

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — acceptance/evidence-only; assembles the review artifact from the tests landed by tickets 001–005
**Deps**: 0063CORACTKNO-005

## Problem

Spec 0063 §7 requires a single review artifact recording the positive / negative / occlusion /
stale / possession-parity fixtures, the two-hop evidence rows, and the anti-contamination results
at the exact implementation commit, with **no decorative locks** — each guard cited must carry a
behavior witness (report §6.5). This capstone consolidates the evidence produced by tickets
001–005 into `archive/reports/0063_core_actor_known_co_present_activity_acceptance.md`, mapping each Spec 0063
§6 obligation and the engaged acceptance gates to the concrete passing test, and renders the feature
verdict. It introduces no new production logic; it exercises and records the pipeline the earlier
tickets composed (INV-098 — feature acceptance is harsh).

## Assumption Reassessment (2026-07-01)

1. The behavior witnesses this artifact cites are produced by earlier tickets, not this one: the
   firewall/parity/two-hop suite in `crates/tracewake-core/tests/hidden_truth_gates.rs` (ticket 005),
   the render/dump disposition + field-disposition guard in
   `crates/tracewake-tui/tests/tui_seam_conformance.rs` (ticket 005), the fixtures in
   `crates/tracewake-content/src/fixtures/co_present_actor_*_001.rs` (ticket 004), and the core
   pipeline in `view_models.rs` / `epistemics/projection.rs` / `agent/perception.rs` /
   `projections.rs` (tickets 001–003). This ticket only reads and records their results.
2. Spec 0063 §7 fixes the artifact contract; the repo convention is `reports/<NNNN>_<slug>_acceptance.md`
   (exemplar `reports/0048_foundational_conformance_hardening_acceptance.md`), archived to
   `archive/reports/` on spec acceptance per `docs/archival-workflow.md`. Spec 0063 is a
   feature/capability spec in the `specs/NNNN` series (final home `archive/specs/`), so the
   `SPEC_LEDGER.md` archived-implementation row and the `specs/ → archive/specs/` move defer to spec
   acceptance — they are a cross-spec follow-up, **not** part of this ticket.
3. **Shared boundary under audit:** the acceptance artifact aggregates evidence across all three
   crates; it is the create-side of a single `(new)` report file and depends only on ticket 005 (the
   leaf whose transitive `Deps` — 004→002→001 and 003→002→001 — cover every prior ticket).
4. **Motivating invariants.** INV-098 (feature acceptance is harsh): the feature is done only when
   caused, agent-possible, eventful, epistemically bounded, TUI-playable, no-human runnable,
   replay-safe, and regression-tested — the artifact maps each to a witness. INV-093 (leakage tested):
   the anti-leak/occlusion rows are the mandated leakage evidence. INV-111 (observer-only emergence
   evidence): the recorded evidence is retrospective and cannot feed simulation behavior.
5. **Enforcement surface (evidence-consumer basis).** This ticket audits/reads the actor-knowledge
   firewall, possession-parity, deterministic-replay, and view-model surfaces rather than modifying
   them; confirm the artifact's rows stay observer-only (debug/evidence, non-diegetic) and introduce
   no leakage or nondeterminism path — the witnesses are re-run at the recorded commit, not copied
   from memory.

## Architecture Check

1. A single consolidating artifact citing per-obligation behavior witnesses — rather than scattering
   evidence across ticket descriptions — gives one reviewable acceptance surface mapped to the Spec
   0063 §6 matrix and the engaged gates, and satisfies the §6.5 non-vacuity rule (every lock carries
   a witness). Deferring the ledger/archival move keeps this ticket's contract to the evidence
   artifact only.
2. No backwards-compatibility aliasing/shims: this ticket adds a report file and no code; it invents
   no fallback and modifies no production surface.

## Verification Layers

1. INV-098 (harsh acceptance) -> invariants alignment check + replay/golden-fixture check: the
   artifact enumerates every §6 obligation mapped to a passing named test, re-run at the recorded
   commit.
2. INV-093 / INV-024 (leakage tested) -> replay/golden-fixture check: the anti-leak, occlusion, and
   possession-parity witnesses (ticket 005) are cited with pass status.
3. INV-111 (observer-only emergence evidence) -> manual review: the artifact is retrospective and
   cannot feed simulation behavior or author outcomes.

## What to Change

### 1. Assemble the acceptance artifact

Create `archive/reports/0063_core_actor_known_co_present_activity_acceptance.md` recording, at the exact
implementation commit: the two-hop evidence rows (Hop 1 core→`observed_activity`, Hop 2
`observed_activity`→rendered/dumped pane, plus the hop-removal negative); the positive / anti-leak /
occlusion / stale / possession-parity witnesses mapped to their named tests and fixtures; the
anti-contamination results; and the feature verdict. Each cited guard names its behavior witness
(no decorative locks, §6.5). Include the re-run commands and their pass status.

## Files to Touch

- `archive/reports/0063_core_actor_known_co_present_activity_acceptance.md` (new)

## Out of Scope

- Any production or test code change (all in tickets 001–005).
- The `docs/4-specs/SPEC_LEDGER.md` archived-implementation row and the `specs/ → archive/specs/`
  move — deferred to spec acceptance (cross-spec follow-up per `docs/archival-workflow.md`).
- Spec `0064`/`0069` co-present pane-layout and parity-registry work (later specs).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` — the full suite (all witnesses from tickets 001–005) passes at the
   recorded commit; the artifact cites the exact test names.
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build
   --workspace --all-targets --locked` — the four-gate project check is green and recorded.
3. Every Spec 0063 §6 obligation (positive, anti-leak, occlusion, stale, possession-parity, two-hop)
   appears in the artifact mapped to a named passing test — no obligation left as a decorative lock.

### Invariants

1. The artifact is observer-only evidence: retrospective, non-diegetic, unable to feed simulation
   behavior (INV-111, INV-068).
2. Feature acceptance is complete only with every §6 witness passing at the recorded commit
   (INV-098).

## Test Plan

### New/Modified Tests

1. `None — acceptance/evidence-only ticket; verification is command-based and the behavior witnesses
   are owned by tickets 004 (fixtures) and 005 (firewall/render suite), named in this artifact.`

### Commands

1. `cargo test --workspace`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked`
3. A workspace-wide boundary is correct for a capstone: it re-proves the whole feature end-to-end;
   narrower filters belong to the per-surface tickets it consolidates.

## Outcome

Completed: 2026-07-01

Created `archive/reports/0063_core_actor_known_co_present_activity_acceptance.md` as the Spec 0063
acceptance-evidence capstone. The artifact records implementation commit
`a02f54064fe332a961926da6f0d566c19222d75b`, maps every Spec 0063 §6 obligation to a named passing
behavior witness, cites the ticket-004 fixture matrix, records anti-contamination results, and
states the scoped feature verdict. It adds no production or test logic.

Verification:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace --all-targets --locked`
4. `cargo test --workspace`
