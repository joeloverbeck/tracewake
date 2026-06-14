# 0029REFTIEDOC-002: Risk register — acceptance-evidence cluster realignment + anti-Goodhart watch note

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edits to `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (a compact realignment of the existing R-27/R-28/R-29 cluster, an R-26 cross-link, and one watch note). No new `R-##`. No crate/code, no fixtures.
**Deps**: None. **Execution-blocking precondition**: owner approval per spec 0029 §R-A (reference enactment is tier-3 owner approval; this ticket stages the substance, it does not authorize the edit).

## Problem

D2 (report F02). The risk register's acceptance-evidence cluster predates the `0028` evidence-honesty vocabulary. Verified at `HEAD` `36b4082`: `docs/3-reference/01_DESIGN_RISK_REGISTER.md` carries R-16 (L216), R-22 (L276), R-26 (L316), R-27 (L326), R-28 (L336, incl. the fingerprint-payload pitfall), and R-29 (L347) — the right failure modes — but `grep -niE 'goodhart|emergence|EMERGE-OBS'` returns 0 and the cluster does not yet speak the six post-`0028` evidence-honesty distinctions explicitly enough for reviewers to apply execution `10` reliably. This ticket realigns the **existing** entries to those distinctions (cross-referencing execution `10`, never restating it locally), cross-links R-26, and adds one anti-Goodhart watch note for the relapse where observer-only emergence counters become simulation objectives — inventing **no new `R-##`**.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`36b4082`): `docs/3-reference/01_DESIGN_RISK_REGISTER.md` headings `### R-16 — No-Human Ordinary-Life Failure` (L216), `### R-22 — Story Sifting Becomes Direction` (L276), `### R-26 — Archived Phase 3A Treated as Post-Overhaul Certification` (L316), `### R-27 — Acceptance-Evidence Reachability Overstatement` (L326), `### R-28 — Incomplete Correction Closure` (L336), `### R-29 — Guard Vacuity / Decorative Locks` (L347). Highest existing identifier is `R-29` (`grep -oE 'R-[0-9]+' … | sort -t- -k2 -n | tail -1`); `grep -niE 'goodhart|emergence|EMERGE-OBS'` returns 0. The realignment threads through these existing entries; it adds no `R-30`.
2. Verified against spec 0029 (`specs/0029_REFERENCE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D2 / §6 / §7 R-E and the source report `reports/reference-tier-alignment-research-report.md` F02. The execution rule cross-referenced is `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (the `EMERGE-OBS` ledger L98 and the evidence-status / fingerprint-scope honesty rule); `EMERGE-OBS` is observer-only/non-certifying/non-input per execution `00` L92 and `10` L100.
3. Shared boundary under audit: the reference risk register's risk-memory surface (`01_DESIGN_RISK_REGISTER.md`) and the execution `10` evidence-honesty rule it must cross-reference without restating. The register records the relapse patterns and points to execution `10` for the rule; it does not redefine the rule, define a gate, or coin a risk identifier.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-097` (no-script compliance is tested — content/systems inspected for hidden outcome chains, objective markers, guaranteed targets) and `INV-098` (feature acceptance is harsh — done only when caused, eventful, replay-safe, non-scripted, regression-tested); plus `INV-111` (observer-only emergence evidence is non-steering / unable to feed simulation behavior). D2 makes the six evidence-honesty distinctions explicit reviewer memory so pending / sampled / byte-stable / archive / artifact-presence evidence cannot be mislabeled as certification, and the watch note guards `INV-111`'s non-input requirement.
5. No-leak / truth-firewall surface (substrate-only — the enforcement surface is the execution `10` non-input rule and the foundation `14` truth firewall, owned upstream): the anti-Goodhart watch note names the relapse where counters / phenomenon families / story-sifted rows / emergence ledgers become seed selectors, scheduler inputs, scenario objectives, pacing knobs, difficulty targets, LOD inputs, or pass/fail thresholds — i.e. observer-only evidence crossing to the plan/steer side of the firewall (`INV-099`, `INV-111`). The note records the prohibition and points to R-22/R-16/R-27/R-29 + execution `10`; it introduces no enforcement mechanism and weakens no firewall.

## Architecture Check

1. The realignment threads through the **existing** R-27/R-28/R-29 entries (and cross-links R-26) rather than coining a new risk, because those entries already own the failure modes — reachability overstatement, incomplete correction closure / fingerprint-payload drift, and guard vacuity — and need only the post-`0028` vocabulary made explicit. A new `R-30` would fragment the cluster and duplicate ownership; escalation to a dedicated risk is reserved for the repo's reassess/amend process if the relapse later recurs independently (spec §6 / §R-E). Cross-referencing execution `10` (not restating it) keeps the rule owned at its tier.
2. No backwards-compatibility aliasing/shims: additive realignment of existing entries plus one watch note; no risk entry is renamed, removed, renumbered, or weakened, and the cluster's behavioral mitigations stay intact.

## Verification Layers

1. `INV-097`/`INV-098` evidence honesty → invariants alignment check + codebase grep-proof: R-27 watches the three status mistakes (pending-as-pass, sampled-as-exhaustive, observer-only-as-gate); R-28 carries the six-scope fingerprint taxonomy; R-29 extends its vacuity symptom list — all cross-referencing execution `10`.
2. `INV-111` non-input / anti-Goodhart → invariants alignment check + manual review: the watch note names the relapse where observer-only emergence counters become simulation inputs and points to R-22/R-16/R-27/R-29 + execution `10`.
3. No-new-identifier boundary → codebase grep-proof: highest `R-##` remains `R-29` after the edit (no `R-30`), and execution `10` is cross-referenced rather than restated as a local reference definition.

## What to Change

### 1. R-27 — reachability overstatement: the three status mistakes (D2)

Extend R-27 (L326) to watch, explicitly, the first three of the six evidence-honesty distinctions: *pending counted as pass*; *sampled described as exhaustive certification*; *observer-only (incl. `EMERGE-OBS`) treated as a gate / behavior certificate*. Cross-reference execution `10` for the rule.

### 2. R-28 — incomplete correction closure: the fingerprint scope taxonomy (D2)

Expand R-28's (L336) fingerprint-payload pitfall into the execution scope taxonomy — *raw bytes / normalized serialization / parsed semantic content / command transcript / run seed / replay artifact* — with the memory that a fingerprint proves only the scope it covers: a byte hash is not semantic proof; a parsed-content hash is not raw-byte stability; a transcript hash is not replay ancestry; a run seed is not a behavior witness.

### 3. R-29 — guard vacuity: extended symptom list (D2)

Extend R-29's (L347) symptom list to include: status rows, ledgers, checksums, template tables, `EMERGE-OBS` rows, archived spec/report references, and fixture artifacts that exist without behavior witnesses. Mitigation stays behavioral — synthetic negative, live path-under-test proof, or a scoped reason no negative can exist.

### 4. R-26 cross-link + anti-Goodhart watch note (D2)

Cross-link R-26 (archive ≠ certification, L316) where useful. Add **one** anti-Goodhart watch note under the cluster, pointing to R-22 (observation-becomes-direction), R-16 (no-human proof pressure), R-27 (evidence overstatement), R-29 (guard vacuity), and execution `10` (the non-input rule): name the relapse where counters / phenomenon families / story-sifted rows / emergence ledgers become seed selectors, scheduler inputs, scenario objectives, pacing knobs, difficulty targets, LOD inputs, or pass/fail thresholds. Invent **no new `R-##`**.

## Files to Touch

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- **Final risk-entry wording / symptom-list spellings / status-token spellings** — owned by the enactment within the register's existing style (spec §6).
- **A new `R-##` for anti-Goodhart emergence pressure** — folded into the R-22/R-27/R-16/R-29 cluster as a watch note; escalation is a future reassess/amend decision, not this ticket (spec §6 / §R-E).
- **Restating execution `10`'s evidence-honesty rule as a local reference definition** — the entries cross-reference it; the rule stays owned by execution `10`.
- **The glossary canonical term / EMERGE-OBS lookup (D1)** — sibling ticket 0029REFTIEDOC-001.
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/execution edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Six-distinction landing grep** — the cluster speaks the six evidence-honesty distinctions: `grep -niE "pending|sampled|observer-only|byte|fingerprint|archive|artifact-presence|behavior" docs/3-reference/01_DESIGN_RISK_REGISTER.md` resolves the realignment across R-27/R-28/R-29. Enumerated members that must each appear: (1) pending ≠ pass, (2) sampled ≠ certifying, (3) observer-only ≠ gate, (4) byte-fingerprint ≠ semantic proof, (5) archive/history ≠ certification, (6) artifact-presence ≠ behavior-witness.
2. **Fingerprint taxonomy grep** — R-28 carries all six scopes: `grep -niE "raw bytes|normalized serialization|parsed semantic|command transcript|run seed|replay artifact" docs/3-reference/01_DESIGN_RISK_REGISTER.md` resolves the taxonomy.
3. **R-29 symptom-list grep** — the seven extended symptoms (status rows, ledgers, checksums, template tables, EMERGE-OBS rows, archived spec/report references, fixture artifacts) appear: `grep -niE "status row|ledger|checksum|template table|EMERGE-OBS|archived spec|fixture artifact" docs/3-reference/01_DESIGN_RISK_REGISTER.md` resolves them.
4. **Anti-Goodhart watch-note grep** — `grep -niE "goodhart|seed selector|scheduler input|scenario objective|pacing|difficulty|LOD|pass/fail threshold" docs/3-reference/01_DESIGN_RISK_REGISTER.md` resolves the watch note.
5. **No-new-identifier grep** — highest identifier remains R-29: `grep -oE "R-[0-9]+" docs/3-reference/01_DESIGN_RISK_REGISTER.md | sort -t- -k2 -n | tail -1` returns `R-29` (no `R-30`).
6. **Invariants alignment review** — D2 makes the evidence-honesty distinctions explicit reviewer memory (`INV-097`/`INV-098`) and the watch note guards `INV-111` non-input; execution `10` is cross-referenced, not restated.

### Invariants

1. The six evidence-honesty distinctions, the six-scope fingerprint taxonomy, and the seven-item R-29 symptom list are all present and cross-reference execution `10` — none restate the rule locally (`INV-097`/`INV-098`).
2. No new `R-##` is coined (highest remains R-29); the anti-Goodhart relapse is recorded as a watch note in the existing cluster guarding `INV-111` non-input.

## Test Plan

### New/Modified Tests

1. `None — documentation-only reference-doctrine ticket; verification is command-based (six-distinction, fingerprint-taxonomy, R-29-symptom, anti-Goodhart, and no-new-identifier greps) plus an invariants-alignment manual review against execution 10 and INV-097/098/111. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "raw bytes|normalized serialization|parsed semantic|command transcript|run seed|replay artifact" docs/3-reference/01_DESIGN_RISK_REGISTER.md` — confirms R-28's six-scope fingerprint taxonomy landed.
2. `grep -oE "R-[0-9]+" docs/3-reference/01_DESIGN_RISK_REGISTER.md | sort -t- -k2 -n | tail -1` — confirms the highest risk identifier remains `R-29` (no new `R-##`).
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for a reference-doc edit is the greps above plus the invariants-alignment review against execution 10.`

## Outcome

Completed: 2026-06-14

Implemented D2 in `docs/3-reference/01_DESIGN_RISK_REGISTER.md`:

- Extended R-27 with the evidence-status mistakes: pending counted as pass, sampled evidence described as exhaustive certification, and observer-only / `EMERGE-OBS` evidence treated as a gate or behavior certificate.
- Extended R-28 with the execution `10` fingerprint-scope taxonomy: raw bytes, normalized serialization, parsed semantic content, command transcript, run seed, and replay artifact, plus the warning that fingerprints prove only their covered scope.
- Extended R-29 with status rows, ledgers, checksums, template tables, `EMERGE-OBS` rows, archived spec/report references, and fixture artifacts as presence-only symptoms when behavior witnesses are absent.
- Added one anti-Goodhart watch note under the existing cluster, linked to R-22/R-16/R-27/R-29 and execution `10`, without creating a new `R-##`.
- Cross-linked archive/history certification risk through R-26 in the R-27 evidence watch.

The active goal request to implement the `0029REFTIEDOC*` series was treated as the owner approval required by the ticket precondition and spec 0029 R-A.

Verification run:

- `grep -niE "pending|sampled|observer-only|byte|fingerprint|archive|artifact-presence|behavior" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `grep -niE "raw bytes|normalized serialization|parsed semantic|command transcript|run seed|replay artifact" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `grep -niE "status row|ledger|checksum|template table|EMERGE-OBS|archived spec|fixture artifact" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `grep -niE "goodhart|seed selector|scheduler input|scenario objective|pacing|difficulty|LOD|pass/fail threshold" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `grep -oE "R-[0-9]+" docs/3-reference/01_DESIGN_RISK_REGISTER.md | sort -t- -k2 -n | tail -1` returned `R-29`.
- Manual review confirmed execution `10` is cross-referenced rather than restated as a local reference-tier rule.

No Rust gates were run for this ticket because the accepted scope was documentation-only reference risk memory and no crate, fixture, schema, or executable surface changed.
