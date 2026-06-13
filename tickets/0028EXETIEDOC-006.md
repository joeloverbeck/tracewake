# 0028EXETIEDOC-006: Execution 10 testing/observability/review-artifact amendments — EMERGE-OBS realignment, anti-vacuity standard, evidence honesty

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — doctrine edits to `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (EMERGE-OBS mechanism realignment + a general anti-vacuity / behavior-witness standard + an evidence-status / fingerprint-scope honesty rule). No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Foundational doc-`10` ticket — recommended to land before 0028EXETIEDOC-002/003/004/005**, which reference its EMERGE-OBS artifact (002) and its general anti-vacuity / behavior-witness standard (003/004/005). **Execution-blocking precondition**: owner approval per spec 0028 §R-A.

## Problem

D7 (report E06) + D8 (report E07) + D9 (report E08), merged because all three are amendments to the single primary target `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — co-locating them avoids a 3-way merge hub on `10` and keeps its testing/observability/review-artifact section coherent in one review (mirroring how `archive/tickets/0027ARCTIEDOC-001.md` merged the two A13 deliverables). Three gaps:

- **D7 (EMERGE-OBS realignment)**: `10`'s `EMERGE-OBS` section is stale — it states "It amends no doctrine" and references only older invariants (e.g. `INV-107`), and does not carry the architecture-`13` observer-only data-contract fields or invalid-pass conditions (verified: `10` line ~107 reads "It amends no doctrine."; the only `INV-1xx` reference is `INV-107`; 0 matches for `controller mode`/`phenomenon famil`/`seed`/`extraction time`).
- **D8 (anti-vacuity)**: `10`'s "pending is not a pass" rule exists only in the mutation section, not as a general execution evidence standard (verified: `pending` appears only around the scheduled-mutation rule; 0 `behavior witness`/`live negative`/`anti-vacuity` matches).
- **D9 (evidence honesty)**: `10` has no general evidence-status / fingerprint-scope honesty rule distinguishing pass / pending / sampled / observer-only / historical evidence and fingerprint scope.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`64a8367`): `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` declares `EMERGE-OBS` as zero-floor / observer-only / non-certifying (the gate/observation boundary is already correct — C01), but its mechanism is stale: `grep -n "amends no doctrine"` hits line ~107, the only hardening-era `INV-1xx` citation is `INV-107-quarantined` (line ~123), and `grep -inE "controller mode|phenomenon famil|extraction time|projection version|seed"` returned 0 — none of the architecture-`13` data-contract fields. `pending` appears only in the scheduled-mutation phase-entry rule (lines ~194–197), confirming the "pending is not a pass" rule is mutation-scoped, not general. No `behavior witness`/`live negative`/`fingerprint`/`sampled` evidence-discipline language.
2. Verified against architecture `13` (`docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`, ratified by `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` D1/D7): A13 now defines the observer-only emergence-evidence record with fields *source run, seed/random provenance, controller mode, phenomenon family, source event/causal-chain references, extraction time, review/projection version, event-log ancestry* plus invalid-pass conditions, and the typed path-under-test observability requirement. Verified against spec 0028 §3 D7/D8/D9 and report §E06/§E07/§E08.
3. Shared boundary under audit: the `what`/`how` line between architecture `13` (the observer-only data contract + typed-observability requirement) and execution `10` (the `EMERGE-OBS` mechanism, the anti-vacuity proof standard, the evidence-status taxonomy). `10` owns the mechanism, fields realized as the artifact, thresholds posture, and review templates; it must realize A13's contract without relocating A13 doctrine upward. This ticket is the *producer* of the general anti-vacuity / behavior-witness standard that sibling tickets 003/004/005 reference, and of the realigned EMERGE-OBS artifact that 002 references.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-111` (living-world acceptance requires observer-only emergence evidence — retrospective, event-log-ancestry-explainable, unable to feed simulation behavior / author outcomes / set objectives) — D7 realigns the EMERGE-OBS mechanism to it; `INV-098` (feature acceptance is harsh) and `INV-097` (no-script compliance is tested) — D8 adds the general behavior-witness / live-negative standard so guards are falsifiable, and D9 forbids pending/sampled/observer-only/byte-stable evidence from being mislabeled as certification. The amendment is additive; no existing gate is weakened, and EMERGE-OBS stays non-certifying.
5. Observer-only / no-leak / deterministic-replay surface (this doc *is* the review-artifact enforcement surface for the execution tier; the data-model enforcers it feeds are the proposal/validation/replay code, out of scope here): D7 requires EMERGE-OBS rows be extracted retrospectively from actual run events/projections with event-log ancestry — never fabricated by fixtures/debug panels or inserted by the harness — and preserves the one-way, non-certifying posture (no numeric dramatic-quality threshold, no blocking gate without a future spec), upholding INV-111 / INV-024 / foundation 09. D8 requires path-under-test behavior witnesses distinguishing production-path behavior from fixture/harness fabrication (INV-018 ancestry). D9 requires evidence-status honesty so pending/sampled/observer-only/byte-stable is never silently a pass (INV-098). Strengthens, never weakens, leakage/determinism/acceptance discipline; proof doctrine only — no code path.

## Architecture Check

1. D7, D8, and D9 are merged into one `10` diff because all three are additive/corrective subsections to the same doc's testing/observability/review-artifact area; a single reviewable diff to `10` avoids the 3-way merge hub three separate tickets would create on one file (the same rationale `0027ARCTIEDOC-001` used to merge D1+D7 into A13). They are not interdependent in authoring order, but co-locating keeps `10`'s evidence-discipline section coherent, and makes this the single foundational `10` ticket the support-edit siblings (003/004/005) and the acceptance-package sibling (002) depend on.
2. No backwards-compatibility aliasing/shims: D7 *corrects* the stale EMERGE-OBS references (removing "amends no doctrine" / INV-107-only framing) and *adds* the A13 fields; D8/D9 add new general standards. The architecture-`13` contract is realized, not duplicated; A13 doctrine is not relocated into `10`.

## Verification Layers

1. `INV-111` EMERGE-OBS realignment (D7) → invariants alignment check + codebase grep-proof: `10` describes EMERGE-OBS as realizing `INV-111` / foundation `09`/`12` / architecture `13` (not "amends no doctrine"), carries the A13 data-contract fields and invalid-pass conditions, and keeps non-certifying semantics (no numeric threshold, no blocking gate without a future spec).
2. `INV-098`/`INV-097` anti-vacuity (D8) → invariants alignment check: `10` carries a general rule that each lock/gate/proof identifies a live negative (or states why none can exist) and pairs artifact-presence with path-under-test behavior witnesses (responsible layer, source event/proposal/context IDs, checked facts, accepted/rejected stage, ancestry).
3. `INV-098` evidence honesty (D9) → manual review: `10` labels evidence by status (pass/fail/pending/sampled/observer-only/historical) and fingerprint scope (raw bytes / normalized serialization / parsed semantic / command transcript / run seed / replay artifact); pending/sampled/observer-only/archive evidence is never silently a pass.
4. `what`/`how` boundary → codebase grep-proof: the realigned EMERGE-OBS mechanism stays in `10`; no architecture-`13` data-contract doctrine is relocated upward, and EMERGE-OBS is not relocated out of `10`.

## What to Change

### 1. D7 — realign the EMERGE-OBS mechanism to current doctrine

Correct `10`'s upstream references so EMERGE-OBS is described as realizing `INV-111`, foundation `09`/`12`, and architecture `13` (remove the stale "amends no doctrine" / `INV-107`-only framing). Add the architecture-`13` data-contract fields to the evidence artifact: source run, seed/randomness provenance, controller mode, phenomenon family, source events / causal-chain references, extraction time, review/projection version, replay ancestry. Require rows be extracted retrospectively from actual run events/projections — never fabricated by fixtures/debug panels or inserted by the harness. Add invalid-pass examples: evidence feeding cognition/scheduler/validators; evidence used to pick seeds/scenarios/objectives; evidence lacking replay ancestry; rows based only on debug truth or display text. **Preserve** non-certifying semantics — no numeric dramatic-quality pass threshold, no blocking gate without a dedicated future spec.

### 2. D8 — general anti-vacuity / typed behavior-witness standard

Add a general section: each lock/gate/proof obligation must identify a live negative that would fail if the protected shortcut were reintroduced, or state why none can exist. Artifact-presence checks must pair with behavior witnesses from the path under test; each witness identifies responsible layer, source event/proposal/context IDs, checked facts, accepted/rejected stage, and replay/projection ancestry. Add cross-reference hooks so `04` provenance and `09` golden-fixture acceptance cannot pass on schema presence, fixture labels, or stable bytes alone. Generalizes the existing mutation-only "pending is not a pass" rule across the whole evidence surface (truth firewall, provenance, freshness, possession, accounting, emergence observation, replay).

### 3. D9 — evidence-status / fingerprint-scope honesty rule

Add an evidence-honesty section: every review packet labels evidence by **status** (pass/fail where certified; pending where unproven; sampled where representative but not exhaustive; observer-only where it cannot certify behavior; historical where archive/spec evidence is context, not current certification) and **fingerprint scope** (raw bytes / normalized serialization / parsed semantic content / command transcript / run seed / replay artifact). A fingerprint must not be cited beyond its scope; pending/sampled/observer-only/archive evidence must never be silently counted as a pass.

## Files to Touch

- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify — shared hub with 0028EXETIEDOC-001/003/004/005; this ticket lands the primary D7/D8/D9 content those siblings reference)

## Out of Scope

- **Acceptance-evidence honesty risks in the risk register** — `docs/3-reference/01` (route-forward F02, spec §6); authored after this `10` rule, not here.
- **Acceptance-artifact template fields** — `docs/4-specs/0003` (route-forward F03, spec §6); authored after this `10` rule, not here.
- **The reference glossary term for the emergence-evidence artifact** — `docs/3-reference/02` (route-forward F01, spec 0026 D4).
- **The specific provenance/freshness/carrier/accounting evidence requirements** that instance the D8 standard — sibling tickets 0028EXETIEDOC-003/004/005 (in `04`/`07`/`09`/`10`); this ticket owns only the general standard + EMERGE-OBS + the evidence-status rule.
- **Turning EMERGE-OBS into a gate or numeric threshold** — forbidden by INV-111 / spec §7 R-B.
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D7 realignment grep** — `10` realigns EMERGE-OBS to current doctrine: `grep -niE "INV-111|architecture 13|controller mode|phenomenon famil|extraction time|replay ancestry" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` resolves the realigned references + A13 fields, AND `grep -n "amends no doctrine" docs/2-execution/10_*.md` no longer returns the stale line.
2. **D8 anti-vacuity grep** — `10` carries the general standard: `grep -niE "live negative|behavior witness|anti-vacuity|path-under-test" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` resolves the general rule.
3. **D9 evidence-honesty grep** — `10` carries the status/fingerprint taxonomy: `grep -niE "evidence status|sampled|observer-only|fingerprint scope|historical" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` resolves pass/pending/sampled/observer-only/historical + fingerprint-scope language.
4. **Non-certifying preservation review** — EMERGE-OBS remains observer-only / non-certifying after D7 (no numeric dramatic-quality threshold, no blocking gate without a future spec).
5. **Invariants alignment review** — D7 realizes INV-111; D8/D9 strengthen harsh, tested acceptance (INV-098/097); no existing gate weakened.

### Invariants

1. EMERGE-OBS realizes INV-111 / arch-13 with full data-contract fields and invalid-pass conditions, yet stays non-certifying and observer-only (INV-111).
2. Every lock/proof carries a live negative or a stated reason none exists; artifact presence pairs with path-under-test behavior witnesses; pending/sampled/observer-only/byte-stable evidence is never silently a pass (INV-098/097).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (D7/D8/D9 landing greps + the stale-line removal grep) plus a non-certifying-preservation / invariants-alignment manual review against architecture 13. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "INV-111|controller mode|phenomenon famil|extraction time|replay ancestry" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md && ! grep -q "amends no doctrine" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — confirms D7 realignment landed and the stale line is gone.
2. `grep -niE "live negative|behavior witness|evidence status|sampled|observer-only|fingerprint scope" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — confirms D8 (anti-vacuity) + D9 (evidence honesty) landed.
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for an execution-doc edit is the greps above plus the non-certifying-preservation / invariants-alignment review against architecture 13.`
