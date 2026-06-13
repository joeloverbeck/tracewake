# 0027ARCTIEDOC-001: Land the observer-only emergence-evidence record and typed non-vacuous observability in architecture 13

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (a new observer-only emergence-evidence artifact family + a typed-observability requirement). No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0027 §R-A — architecture is tier-1 doctrine (lighter than the constitutional sign-off a foundation amendment needs, but the edit must not land by convention). This ticket documents the amendment; it must not be applied to `docs/1-architecture/13_*` until owner approval.

## Problem

D1 (report A13-E1) + D7 (report A13-F). `INV-111` (ratified by archived spec 0026) now requires observer-only emergence evidence for living-world acceptance, but `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (A13) lists no artifact family for an observer-only emergence-evidence record and defines no one-way boundary keeping such a record out of cognition/scheduler/validator inputs (verified: 0 `emergence`/`EMERGE-OBS`/`observer-only` matches in A13 at `fdfd0b9`). Separately, A13 rejects "looks right" / display-string proof but does not require protected subsystems to expose the typed behavior-witness fields execution needs to build a live negative (verified: 0 `behavior-witness`/`live negative` matches), so an artifact can exist yet be unfalsifiable (design risk R-29, guard vacuity). This ticket adds both as compact A13 subsystem contracts, keeping all mechanism (`EMERGE-OBS` table/counters/thresholds/ratchets, test commands, pass statuses) in execution `10`.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`fdfd0b9`): `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` exists and contains no `emergence`/`EMERGE-OBS`/`observer-only` token and no `behavior-witness`/`live negative` requirement (`grep -ciE` returned 0 for each). A13 already owns required artifact families (event logs, replay reports, holder-known context packets, decision traces, validation reports, epistemic projection reports, institution traces, TUI transcript/view-model evidence, debug reports, content validation, anti-regression/static guards), rejects "looks right" / display-string proof, and lists invalid-pass conditions — D1/D7 extend that set, they do not replace it.
2. Verified against spec 0027 (`specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D1 + D7 and §5 V1–V3, and the source report `reports/architecture-tier-alignment-research-report.md` §4.1 (A13-E1) / §4.7 (A13-F). D1's cross-reference to A11 is authored in A13 (a pointer to story sifting); the reciprocal A11 scoping is sibling ticket 0027ARCTIEDOC-002. `EMERGE-OBS` is confirmed to live only in `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` and must not be relocated upward.
3. Shared boundary under audit: the `what`/`how` layer line between architecture A13 (the observer-only data contract + the typed-observability requirement) and execution `10` (the `EMERGE-OBS` mechanism, thresholds, ratchets, commands, pass statuses). A13 carries the data-flow/authority boundary only; none of the mechanism vocabulary.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-111` (living-world acceptance requires replayable, observer-only emergence evidence — retrospective, explainable through event-log ancestry, unable to feed simulation behavior / author outcomes / set dramatic objectives) — D1 gives that principle its architecture data-contract home; `INV-098` (feature acceptance is harsh) and `INV-097` (no-script compliance is tested) — D7 sharpens harsh acceptance with a typed-behavior-witness requirement so guards are falsifiable. The amendment is additive; no invariant is weakened (spec §4).
5. Observer-only / no-leak / deterministic-replay surface (the doctrine this amendment governs; enforcement lives in deferred execution-`10` surfaces, not this edit): D1's one-way path (authoritative simulation → replay/projection/story-sifting observer → review artifact) forbids the record feeding cognition, validators, scheduling/candidate generation, LOD-promotion, event-spawning story sifting, or authoring objectives (INV-024 no telepathy / INV-006 / foundation `09` no-director boundary), and every evidence row must be explainable by event-log ancestry (INV-018 deterministic replay). D7 requires path-under-test typed observability that distinguishes production-path behavior from fixture/harness fabrication — strengthening, not weakening, leakage/determinism discipline. This ticket adds doctrine only: no code path, no leakage, no nondeterminism. The execution-`10` mechanism and later validation work are the enforcers (spec §6, V4).

## Architecture Check

1. D1 and D7 are merged into one A13 diff because both are additive subsections to the same doc's required-artifact-families / review-checklist area; a single reviewable diff to A13 avoids a two-ticket merge hub on one file. They are not interdependent (either could be authored first), but co-locating them keeps A13's acceptance-artifact section coherent in one review.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine. A13 names the data contract + observability requirement; the `EMERGE-OBS` mechanism in execution `10` is untouched — no duplicate, no relocated mechanism, no compatibility layer.

## Verification Layers

1. `INV-111` observer-only emergence evidence (D1) → invariants alignment check: A13 gains an artifact family stating the record is retrospective review-only (not world state / holder-known / institution-known / validation / scheduler input), one-way, and event-log-ancestry-explainable, with invalid-pass conditions for fabricated / steering / untraceable evidence.
2. `INV-098`/`INV-097` harsh, tested acceptance (D7) → invariants alignment check: A13 requires typed path-under-test behavior-witness fields (responsible layer, source IDs, checked facts, accepted/rejected stage, ancestry) sufficient for execution to build a live negative; artifact existence/shape/count/checksum/text alone is insufficient.
3. `what`/`how` layer boundary → codebase grep-proof: the newly added A13 passages contain no `EMERGE-OBS`, table/row/counter/threshold/ratchet/command/pass-status token (those stay in execution `10`).
4. Single-doc additive edit: no replay/golden-fixture or skill-dry-run layer applies; the three layers above each map an engaged invariant to a distinct proof surface.

## What to Change

### 1. D1 — observer-only emergence-evidence record (new A13 artifact family)

Add a required-artifact-family / review subsection to A13 for an observer-only emergence-evidence record. Substance (final wording authored at enactment):

- It is a retrospective review artifact — not world state, holder-known context, institution-known context, validation input, or scheduler input.
- Each evidence row carries event-log ancestry sufficient to replay/explain its phenomenon: source run, seed/random provenance where applicable, controller-mode / no-human-or-normal-controller run identity, phenomenon family, source event IDs / causal-chain references, extraction time, projection/review version.
- It may classify phenomenon families (contradictions, replans, interruptions, stale-belief consequences, modeled-channel corrections, belief/truth divergence, wrong suspicion, record effects) but names no required story beats, dramatic objectives, or numeric floors.
- The data path is one-way: authoritative simulation → replay/projection/story-sifting observer → review artifact; it never feeds cognition, validators, scheduling/candidate generation, LOD-promotion decisions, event-spawning story sifting, or authoring objectives. Cross-reference A11 (a sifter may produce this observer-only record but no diegetic evidence — see 0027ARCTIEDOC-002).
- Add invalid-pass conditions: evidence fabricated from debug/fixtures rather than path-under-test ancestry; emergence counters used to steer the simulation; rows untraceable to modeled causes.
- Route all mechanism (table/rows/counters/thresholds/ratchets, commands, pass statuses) to execution `10`.

### 2. D7 — typed non-vacuous observability requirement

Add to A13's required-artifact-families / review checklist: for every validation / anti-contamination / replay / projection / diagnostic guarantee, the architecture surface must expose typed, path-under-test observability — responsible layer, source event/proposal/context IDs, checked facts, behavior-witness fields, accepted/rejected stage, and enough ancestry to distinguish production-path behavior from fixture/harness fabrication. An artifact's mere existence, shape, count, checksum, or display text is insufficient unless paired with typed behavior evidence appropriate to the protected claim. The surface must let execution attach live negatives / mutation-metamorphic checks, but A13 defines no tests, thresholds, commands, or pass statuses (execution `10`).

## Files to Touch

- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)

## Out of Scope

- **`EMERGE-OBS` mechanism, thresholds, ratchets, counters, commands, pass statuses, anti-Goodhart constraints** — execution `docs/2-execution/10` (spec §6, V4).
- **The A11 story-sifting scoping correction** — sibling ticket 0027ARCTIEDOC-002 (D2).
- **Reference glossary term for emergence evidence** — `docs/3-reference/02` (spec 0026 D4).
- **Acceptance-evidence / manifest-fingerprint honesty (X10/G)** — execution `10` + reference `01` (spec §6).
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Landing grep** — A13 contains the observer-only emergence-evidence artifact family (one-way path + event-log ancestry + invalid-pass conditions) and the typed-observability requirement: `grep -niE "observer-only|emergence|behavior.witness|path-under-test" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` resolves both additions.
2. **V3 boundary grep** — the newly added A13 passages carry no execution-`10` mechanism token: `git diff --unified=0 -- docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md | grep -E '^\+[^+].*(EMERGE-OBS|ratchet|\bcounters?\b|threshold|ledger row|pass status)'` returns no match in the added lines.
3. **Invariants alignment review** — D1 is an observer-only, one-way, ancestry-bound record weakening no `INV-001`…`INV-111`; D7 is an additive typed-observability requirement strengthening harsh acceptance (INV-098/097).

### Invariants

1. The added A13 passages hold the observer-only data contract + the typed-observability requirement but none of the execution-`10` mechanism tokens — the `what`/`how` boundary holds.
2. The emergence-evidence record is structurally one-way (simulation → observer → review artifact), unable to feed cognition/scheduler/validator/authoring (INV-111 / INV-024 / foundation 09), every row traceable to event-log ancestry (INV-018).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing + V3 boundary greps) plus an invariants-alignment manual review. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "observer-only|emergence|behavior.witness|path-under-test" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — confirms D1 + D7 landed.
2. `git diff --unified=0 -- docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md | grep -E '^\+[^+].*(EMERGE-OBS|ratchet|\bcounters?\b|threshold|ledger row|pass status)'` — must show no mechanism-vocab match in the added lines (V3).
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected and is not the verification boundary for an architecture-doc edit; the boundary is the two greps above plus the invariants-alignment review.`
