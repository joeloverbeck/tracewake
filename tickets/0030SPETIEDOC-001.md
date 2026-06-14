# 0030SPETIEDOC-001: Acceptance-artifact template — evidence-status & fingerprint-scope honesty fields

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edit to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (specs-tier acceptance-artifact template: per-evidence-item honesty fields, structured per the eight points below, on the per-requirement table and/or a companion evidence-item ledger). No crate/code, no fixtures.
**Deps**: None. **Execution-blocking precondition**: owner approval per spec 0030 §R-A (specs-tier enactment is tier-4 owner approval, not constitutional sign-off; this ticket stages the substance, it does not authorize the edit).

## Problem

D1 (report F03). Execution `10` is now the governing source for review-artifact honesty (ratified through `archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` D9): every review packet must label evidence by **status** (pass/fail / pending / sampled / observer-only / historical) and by **fingerprint scope** (raw bytes / normalized serialization / parsed semantic content / command transcript / run seed / replay artifact), must never silently count pending / sampled / observer-only / historical evidence as a pass, and must carry behavior witnesses and replay/provenance ancestry where it claims path-under-test behavior. The specs-tier acceptance-artifact template has the right skeleton but cannot carry that honesty: verified at `HEAD` `5e053f2`, `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`'s per-requirement table (L27) has exactly four columns — `Requirement | Responsible layer | Evidence | Result` — with bare `Evidence` / `Result` cells, and `grep -niE 'status|fingerprint|observer-only|pending|sampled|provenance|witness'` surfaces no evidence-honesty field (the lone hit is a requirement-row cell, not a field). With no status, scope, witness, or provenance fields, evidence can be silently overclaimed at the packet surface. This ticket enriches the per-requirement table and/or a companion evidence-item ledger with per-evidence-item honesty fields that **point to execution `10` for the rule** — coining no gate, no obligation code, no invariant number, and no status token beyond execution `10`'s existing vocabulary, and loosening none of the template's existing anti-overclaiming posture.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`HEAD` `5e053f2`): `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` carries the exact-commit (L6) / gates-run (L11) / changed-files (L20) / per-requirement-evidence (L25) / residual-convention (L45) / scoped-certification-wording (L51) / forbidden-wording (L57) skeleton, and its per-requirement table header (L27) is the bare four-column `Requirement | Responsible layer | Evidence | Result`. `grep -niE 'status|fingerprint|observer-only|pending|sampled|provenance|witness' docs/4-specs/0003_…md` returns no evidence-honesty field (only the `SPINE-AC-011` row cell at L39). The gap (F03) is real; the amendment is additive and the template is byte-identical to its state at the spec's pinned commit `36b4082` (`git diff 36b4082 -- docs/4-specs/0003` empty).
2. Verified against spec 0030 (`specs/0030_SPECS_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D1 / §2 Approach / §5 V2 / §6 / §7, the source report `reports/specs-tier-alignment-research-report.md` F03, and the upstream rule. The execution rule pointed to is `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` §`Evidence status and fingerprint scope honesty` (status vocabulary at L224–227; the six-scope fingerprint taxonomy at L233–238; the "must never be silently counted as a pass" rule at L240–241), ratified by `archive/specs/0028_…md` D9. `EMERGE-OBS` is observer-only / non-certifying / non-input per execution `00`/`10`.
3. Shared boundary under audit: the specs-tier acceptance-artifact template's review-packet surface (`docs/4-specs/0003`) and the execution `10` evidence-honesty rule it must point to without restating. The template makes the fields mandatory and points upward; it does **not** redefine evidence-honesty, `EMERGE-OBS` semantics, gate passage, replay proof, or `P0-CERT` (those stay owned by execution `10` / the certification tiers — spec §Authority, §R-B).
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-098` (feature acceptance is harsh — "done only when … caused … replay-safe … non-scripted, and regression-tested") and `INV-097` (no-script compliance is tested), which the mandatory status + fingerprint-scope fields serve by forbidding overstated certification at the review-packet surface; and `INV-111` (living-world acceptance requires observer-only emergence evidence that is "unable to feed simulation behavior, author outcomes, or set dramatic objectives"), which point 6 preserves by labeling `EMERGE-OBS` `status = observer-only` and barring it from becoming certification or a gate. The amendment encodes these at the template surface; it neither weakens nor redefines them.
5. Acceptance / no-leak / firewall surface (substrate-only — the enforcement surface is the human reviewer applying the execution `10` evidence-honesty rule, owned upstream by `0028` D9, not by this template): the fields are the *inputs* a reviewer needs to apply execution `10`; the template enforces nothing itself. Confirm the additions introduce no leakage or nondeterminism path the enforcement surface would have to undo — the path-under-test behavior-witness fields (point 3) name the responsible layer and the accepted/rejected stage, keeping acceptance evidence inside the typed/validated regime (`INV-099`: truth may validate, not plan); the replay/provenance ancestry fields (point 4) record event-log segment, replay artifact, seed/content version, and extraction/projection version, so a behavior claim is backed by replayable causal evidence (`INV-018`) rather than artifact presence; and no field grants a presentation/LLM/debug surface cognition or certification authority or exposes actor-hidden truth (`INV-024`/`INV-006`). The observer-only field (point 6) records the `INV-111` non-input prohibition and points to execution `10`; it opens no path by which observer-only evidence becomes certification or steering input.

## Architecture Check

1. The honesty fields are authored as **packet-structure requirements that point to execution `10`**, never as new doctrine — the correct altitude for the lowest authority tier (a spec/template "operationalizes higher-tier doctrine into review packets but may not restate, amend, weaken, or supersede it", spec §Authority). Enriching the existing per-requirement table and/or adding a companion evidence-item ledger referenced by per-requirement rows is cleaner than (a) restating execution `10`'s rule locally — which would fork the doctrine and violate the tier boundary — or (b) coining a new gate/status token, which spec §6 / V2 / R-B forbid. A companion ledger (spec §R-C recommendation) lets multiple requirements cite one evidence item without duplicating status/fingerprint/provenance fields; exact layout is settled at authoring within the template's existing style.
2. No backwards-compatibility aliasing/shims: purely additive fields that strengthen the existing anti-overclaiming posture; every existing section — exact commit, gates run, changed files, residual convention-only items, scoped-certification wording, and the forbidden-wording list — is preserved and none is loosened. No existing column, section, or wording is renamed, removed, or weakened.

## Verification Layers

1. `INV-098` / `INV-097` evidence honesty → invariants alignment check + codebase grep-proof: the template makes evidence **status** and **fingerprint scope** mandatory and states that pending / sampled / observer-only / historical evidence cannot be silently counted as a pass, with behavior-witness and replay/provenance fields for behavioral claims — all cross-referencing execution `10`.
2. `INV-111` observer-only non-certifying → invariants alignment check + manual review: `EMERGE-OBS` / observer-only emergence evidence is labelable as `status = observer-only` and barred from becoming certification, a gate pass/fail threshold, a scheduler objective, a scenario goal, or a code-quality substitute.
3. No-new-doctrine / points-to-execution-`10` boundary → codebase grep-proof + manual review: the fields cross-reference execution `10` for the rule and coin no new gate code, observation-obligation code, invariant number, or status token beyond execution `10`'s vocabulary; the existing scoped-certification and forbidden-wording sections remain intact.

## What to Change

### 1. `0003` — per-evidence-item honesty fields, structured per the eight points (D1)

Add per-evidence-item honesty fields to the per-requirement table and/or a companion evidence-item ledger referenced by per-requirement rows (final column headings, ledger layout, and token spellings settled at authoring within the template's existing style — spec §6). The fields must satisfy the eight packet-structure points (point order/grouping settled at authoring; no point coins a gate or identifier):

1. **Evidence status per item** — execution `10`'s vocabulary: *pass/fail* (actually certifies), *pending* (proof not yet present), *sampled* (representative, not exhaustive), *observer-only* (informs review, cannot certify behavior), *historical* (archive/spec/report context, not current certification). State that pending / sampled / observer-only / historical evidence **cannot be silently counted as a pass**.
2. **Fingerprint scope** for every hash/checksum/snapshot claim — *raw bytes / normalized serialization / parsed semantic content / command transcript / run seed / replay artifact* — with the rule that a fingerprint must not be cited beyond its actual scope.
3. **Path-under-test behavior-witness fields** for any claimed behavioral pass: path under test; the command/event/trigger/emitter/scheduler entry that exercised it; responsible layer; accepted/rejected action or validation stage witnessed; the live negative / mutation-style failure / reason a negative is not applicable; and the checked facts/invariants the witness supports.
4. **Replay/provenance ancestry fields**: relevant event-log segment or event identifiers; replay artifact or serialized-log reference; seed/randomness/content version where applicable; extraction/projection version for derived evidence; source provenance for any claim crossing from artifact to semantic behavior.
5. **Sampling/exhaustiveness scope**: a sampled result states population, sample basis, omitted cases, and why representative; exhaustive evidence states what perimeter it exhausts.
6. **Observer-only handling**: `EMERGE-OBS` / observer-only emergence evidence labelable as `status = observer-only`; the template must not let such rows become certification, gate pass/fail thresholds, scheduler objectives, scenario goals, or code-quality substitutes unless a future upstream spec changes the doctrine.
7. **Pending and historical handling**: pending rows name the missing proof and owner/blocker; historical rows identify whether the artifact is context, lineage, or archived precedent (an optional source-type field may distinguish archive/spec/report lineage) and are not counted as current certification.
8. **Requirement result computed only from certifying evidence**: a row may carry useful evidence while the requirement remains pending or failed.

### 2. `0003` — preserve the existing anti-overclaiming posture (D1)

Preserve every existing section unchanged in force: exact commit under test, gates run, changed files, residual convention-only items, the scoped-certification wording, and the forbidden-wording list. The amendment **strengthens** evidence fields **without loosening** any existing posture; it adds no allowed-wording that overstates certification and removes no forbidden-wording entry.

## Files to Touch

- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (modify)

## Out of Scope

- **Final template wording / exact column headings / evidence-item-ledger layout / status & scope token spellings** — owned by the enactment within the template's existing style (spec §6). Recommendation (spec §R-C): a companion evidence-item ledger referenced by per-requirement rows over a single wide grid.
- **Defining or redefining doctrine** — `EMERGE-OBS` semantics, gate passage, replay proof, `P0-CERT`, and the evidence-honesty rule stay owned by execution `10` / the certification tiers; the template points to them and must not restate them (spec §Authority, §R-B).
- **Any new gate code / observation-obligation code / invariant number / status token** beyond execution `10`'s vocabulary (spec §3, §5 V2).
- **`SPEC_LEDGER.md`, specs `README.md`, and `0001` ontology** — boundary-awareness, no change (spec §6).
- **F04 — possession-bind perception** — forward-routed owner decision; this ticket adds no template field to pre-decide it (spec §6 / §R-F).
- **F01/F02 — reference glossary term & risk-register realignment** — owned by sibling spec `0029` (COMPLETED), not this specs-tier pass.
- **Spec `0030` SPEC_LEDGER row + archival (`specs/` → `archive/specs/`)** — deferred to spec acceptance/closeout per the staged-spec convention (spec §6 / §8); not part of this ticket's deliverable.
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- **Code certification** — no assertion that crates, fixtures, CI, or goldens already satisfy the amended template (spec §6). Crate/code, fixtures, and foundation/architecture/execution edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Status-vocabulary landing grep** — the template makes evidence status mandatory using execution `10`'s vocabulary: `grep -niE "pending|sampled|observer-only|historical" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` resolves the status taxonomy (was absent pre-edit), and the "cannot be silently counted as a pass" rule is present.
2. **Fingerprint-scope taxonomy grep** — `grep -niE "raw bytes|normalized serialization|parsed semantic|command transcript|run seed|replay artifact" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` resolves the six-scope taxonomy.
3. **Behavior-witness + replay/provenance grep** — `grep -niE "path under test|behavior witness|responsible layer|event-log segment|replay artifact|provenance" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` resolves the path-under-test witness and replay/provenance ancestry fields.
4. **Observer-only handling grep** — `grep -nE "EMERGE-OBS|observer-only" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` resolves the `status = observer-only` handling, and the template bars such rows from certification / gate thresholds.
5. **No-new-identifier + posture-preserved review** — the forbidden-wording list and scoped-certification section remain intact and unweakened; the amendment coins no new gate code, observation-obligation code, invariant number, or status token beyond execution `10` (manual review against spec §5 V2 / §6 + `grep -nE "Forbidden wording|Scoped certification" docs/4-specs/0003_…md` confirms the sections persist).
6. **Invariants alignment review** — the fields make status + fingerprint scope mandatory (`INV-098`/`INV-097`) and preserve `EMERGE-OBS` non-certifying / non-input posture (`INV-111`); execution `10` is cross-referenced, not restated.

### Invariants

1. Evidence status and fingerprint scope are mandatory per evidence item; pending / sampled / observer-only / historical evidence cannot be silently counted as a pass; behavioral claims carry a path-under-test witness and replay/provenance ancestry — all pointing to execution `10`, defining no doctrine (`INV-098`/`INV-097`).
2. `EMERGE-OBS` / observer-only emergence evidence is labelable only as `status = observer-only` and never becomes certification, a gate threshold, a scheduler objective, or a scenario goal; no new gate/obligation/invariant/status identifier is coined (`INV-111`; spec §Authority / §R-B).

## Test Plan

### New/Modified Tests

1. `None — documentation-only specs-tier doctrine ticket; verification is command-based (status-vocabulary, fingerprint-taxonomy, behavior-witness/provenance, observer-only, and posture-preserved greps against docs/4-specs/0003) plus an invariants-alignment / points-to-execution-10 manual review against execution 10 §"Evidence status and fingerprint scope honesty" and INV-097/098/111. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "pending|sampled|observer-only|historical" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — confirms the evidence-status vocabulary landed (was absent pre-edit).
2. `grep -niE "raw bytes|normalized serialization|parsed semantic|command transcript|run seed|replay artifact" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — confirms the six-scope fingerprint taxonomy landed.
3. `grep -nE "Forbidden wording|Scoped certification|EMERGE-OBS|observer-only" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — confirms the existing anti-overclaiming sections persist and observer-only handling landed.
4. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for a specs-tier template edit is the greps above plus the invariants-alignment / points-to-execution-10 review against execution 10 and INV-097/098/111.`
