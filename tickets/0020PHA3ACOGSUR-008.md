# 0020PHA3ACOGSUR-008: Capstone — 0020 scoped acceptance artifact and the first EMERGE-OBS ledger baseline

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — new acceptance report under `reports/`; new read-only emergence-ledger derivation (test/report code only, no kernel behavior); no schema or event changes
**Deps**: `archive/tickets/0020PHA3ACOGSUR-003.md`, `archive/tickets/0020PHA3ACOGSUR-004.md`, `archive/tickets/0020PHA3ACOGSUR-005.md`, `tickets/0020PHA3ACOGSUR-006.md`, `tickets/0020PHA3ACOGSUR-007.md` (leaf set — `-003` transitively covers `-001`/`-002`); `specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md` (§7)

## Problem

Spec 0020 §7 requires a scoped acceptance artifact recording, for the implementation
commits, the evidence for every correction (items 1–9), the first `EMERGE-OBS`
emergence-evidence ledger baseline (item 10, per `docs/2-execution/10` §Emergence-
evidence ledger — produced last so it measures the corrected surface, spec §8), and
the explicit non-certification statement (item 11). The ledger derivation does not
exist yet; this ticket ships it as read-only reporting code, making this a
deliverable-doubles-as-capstone ticket (it adds verification infrastructure, no
production logic).

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `96bc387`: no `reports/0020_*` exists (no collision); the
   lineage exemplar is `reports/0019_ord_life_cert_scoped_acceptance.md` (verified
   honest by the 0020 audit — the format to match); the canonical substrate for the
   ledger exists (canonical no-human day fixtures + `GENERATIVE_SEEDS`), and every
   counter the ledger needs is a typed materialized record or debug-side projection
   (expectation contradictions, replans/fallbacks, interruptions, intention switches,
   stuck diagnostics by blocker category, truth/belief divergence via the
   INV-107-quarantined debug comparison, modeled-channel corrections, event-kind
   diversity).
2. Verified against live doctrine: `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
   §Emergence-evidence ledger defines `EMERGE-OBS` (registered in
   `docs/2-execution/00` §Canonical observation obligations) — observation
   obligation, NOT a certification gate: per-seed rows + corpus totals, derived only
   from typed records, byte-identical after log serialization/replay, observer-only,
   no thresholds.
3. Cross-artifact boundary under audit: the acceptance-evidence contract — every §7
   claim in the report must be produced by the path under test and quotable against
   code (the lineage's evidence-honesty rule; an overstating acceptance artifact is
   the relapse the risk register now names).
4. INV-098 restated (feature acceptance is harsh); INV-060/INV-087 restated for the
   ledger: no boredom detector, no drama pressure — the ledger is an observer-only
   projection that must never become a simulation, scheduler, content-selection, or
   pacing input, and its counters are not pass/fail thresholds.
5. Deterministic-replay surface touched (read-only): ledger values must be
   byte-identical after log serialization/replay (doc 10 rule, mirroring the
   no-human-metrics rule in `docs/1-architecture/02_*`); the derivation reads
   materialized state and emits a report — it appends no events, mutates no state,
   and introduces no nondeterministic input (no wall-clock, no unordered iteration).
6. Schema shape: none — no schema, event, or view-model change; N/A.

## Architecture Check

1. Deliverable-doubles-as-capstone (the sanctioned exception to "no new production
   logic"): the ledger derivation is verification infrastructure exercising the
   corrected pipeline end-to-end over the canonical corpus — the natural capstone
   vehicle, and producing it last means the baseline measures the post-0020 surface
   (spec §8). The derivation lives in test/report code (observer side), keeping the
   kernel untouched and the INV-060 quarantine structural rather than promised.
2. No backwards-compatibility aliasing/shims; the report follows the 0019 artifact's
   structure (the lineage convention), with the dated-correction pattern available
   if any §7 claim needs qualification.

## Verification Layers

1. Evidence honesty -> every §7 item's claim in the report is backed by a named
   test/command run at the implementation commits; items map 1:1 to §7's list (1
   supersession-parity; 2 remembered-food + per-kind policy; 3 derived census
   dispositions + any repricing ledger; 4 mutation run + triaged baseline ledger;
   5 generative deltas; 6 transitive helper census; 7 exit-blocker + sweep output;
   8 boundary-fixture outcome + bound consistency; 9 risk-register/conformance
   diffs quoted; 10 EMERGE-OBS baseline; 11 non-certification statement).
2. EMERGE-OBS discipline -> ledger derivation test: per-seed counters over the
   canonical corpus; byte-identical across two derivations and across
   serialize/replay; derived only from typed records (no display-string parsing —
   the doc-10 rule and the `forbidden test patterns` list).
3. Quarantine (INV-060/087) -> grep-proof: the ledger module has no call sites in
   kernel/scheduler/content-selection code (observer-only); the report states the
   no-threshold posture.
4. INV-098 -> the four repo gates green at the capstone commit.

## What to Change

### 1. EMERGE-OBS ledger derivation (read-only)

Add a ledger derivation over the canonical no-human days + `GENERATIVE_SEEDS`
corpus emitting per-seed rows and corpus totals for the eight doc-10 counters,
with the byte-identity (re-derivation + post-replay) test. Test/report code only.

### 2. The acceptance artifact

Write `reports/0020_ord_life_cert_scoped_acceptance.md` following the 0019 format:
§7 items 1–11 with quoted evidence, the EMERGE-OBS baseline table (measurement
only — no thresholds, no gating), and the explicit non-certification statement
(scoped evidence toward `ORD-LIFE-CERT`; not full-project certification, not Phase 4
entry, not `FIRST-PROOF-CERT`).

## Files to Touch

- `reports/0020_ord_life_cert_scoped_acceptance.md` (new)
- `crates/tracewake-core/tests/emergence_ledger.rs` (new — or co-located with the no-human capstone suite as surfaced; the derivation + byte-identity tests)

## Out of Scope

- Any threshold, ratchet, or gate on ledger values (a future spec's deliberate
  decision per doc 10 — the zero-floor ratchet is explicitly not this ticket).
- Re-fixing anything the sibling tickets land; this ticket records their evidence.
- `SPEC_LEDGER.md` row and spec archival (deferred to spec acceptance per
  `docs/archival-workflow.md` — cross-spec follow-up, not ticketed).

## Acceptance Criteria

### Tests That Must Pass

1. Ledger derivation green: per-seed rows + totals over the canonical corpus,
   byte-identical across re-derivation and across serialize/replay.
2. Quarantine grep-proof: no kernel/scheduler/content call site consumes the ledger.
3. Report exists with all 11 §7 items, each backed by named evidence; the
   non-certification statement present verbatim in intent.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. The acceptance artifact claims only what the path under test produced — no
   overstatement (the lineage's evidence-honesty contract; risk-register Watch).
2. The EMERGE-OBS ledger is observer-only and threshold-free: a measurement, never
   a simulation input or a pass/fail gate (INV-060, INV-087; doc 10 quarantine).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/emergence_ledger.rs` — derivation, byte-identity,
   and post-replay identity tests.

### Commands

1. `cargo test -p tracewake-core --test emergence_ledger`
2. `cargo test --workspace` (full pipeline at the capstone commit)
3. Targeted re-runs of each sibling ticket's named acceptance tests while composing
   the report (evidence quoted from actual runs, not memory)
