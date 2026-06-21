# 0045FIRPROCER-006: Capstone — live FIRST-PROOF-01…17 re-proof, nine-gate / nine-family / five-source temporal bundle, EMERGE-OBS, and replacement acceptance artifact superseding 0044

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — acceptance-only capstone; it composes and re-proves the work of the prior tickets and authors the replacement evidence artifacts. No new production/simulation logic.
**Deps**: 005

## Problem

Mutation completion (-005) is necessary but not sufficient: a predecessor artifact establishes a subsystem passed at its own historical commit, not that the integrated first-proof system remains coherent at the final commit `U`. The FIRST-PROOF-CERT line can flip only when the complete 0044 integrated contract is re-proven live at `U` and a replacement acceptance artifact renders the verdict from certifying evidence and explicitly supersedes the 0044 artifact (spec §1.1, §7.1, §8.11).

This capstone produces, at one `U`: the `FIRST-PROOF-01`…`17` re-proof matrix; the nine canonical gates as one coherent set; the nine scenario families with required positive and adversarial evidence; the consolidated five-source temporal bundle traced through one representative temporal fact; the replay/provenance package with first-divergence localization; the observer-only `EMERGE-OBS` companion; and the replacement acceptance artifact instantiating the `docs/4-specs/0003` template. It renders `FIRST-PROOF-CERT passed` only if every §8.11 aggregate condition holds; otherwise it stays `FIRST-PROOF-CERT scoped remediation` and names the first failing layer. It is acceptance-only — it introduces no new production logic, only the evidence artifacts that re-prove the pipeline the prior tickets composed.

(spec §7, §8, §8.9, §8.11, §8.12, §13.1 phases 13–15)

## Assumption Reassessment (2026-06-21)

1. The artifacts to author at the spec-named paths are verified absent today (to be created): `reports/0045_first_proof_cert_mutation_remediation_replacement_certification_acceptance.md` and `reports/0045_first_proof_cert_emerge_obs.md`. The template they instantiate, `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, exists (verified this session), as do the predecessor acceptance artifacts in `archive/reports/` (0037/0039/0041/0043) consumed as scoped certified components. The supersession target `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` exists and uses `FIRST-PROOF-18` as a local cross-reference while its navigation labels are `FIRST-PROOF-01`…`17` (verified) — this artifact preserves that historical cross-reference without minting it as a canonical gate.
2. The re-proof exercises existing suites/fixtures, not new logic. Verified this session: the §4.9 named core/content/TUI suites all exist as `cargo test` targets; the nine canonical gates (`EVENT`, `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `POSSESSION-PARITY`, `NO-HUMAN-ORDINARY-LIFE`, `MISSING-PROPERTY`, `VIEW-DEBUG-SPLIT`, `REPLAY`, `FIXTURE-NEGATIVE`) are those of `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`. Counts cited in evidence rows must be re-enumerated from the fixtures at test time, not hardcoded (capstone discipline; hardcoded counts go stale).
3. Cross-artifact boundary under audit: integrated coherence at `U` without predecessor re-audit (spec §7.7) — 0037/0039/0041/0043 are consumed within their own exact-commit scopes, not rerun as isolated audits and not treated as latest-`main` evidence; the new claim is that their seams participate in one coherent final-commit first-proof corpus, and any regression in a consumed seam is a live 0045 failure routed to its earliest responsible layer. The artifact must also state the claim applies only to `U`, not latest `main`, `PHASE-4-ENTRY`, or any deferred surface (spec §8.11, §11).
4. Invariant/rule motivation: `INV-111` — *Living-world acceptance requires observer-only emergence evidence* (verified at `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:459`): emergence evidence must be retrospective, event-log-ancestry-backed, and unable to feed simulation behavior, author outcomes, or set objectives — so `EMERGE-OBS` is `Evidence status: observer-only`, `Certification use: not counted`, and contributes to no threshold/score/seed/gate. And `INV-112` — *Time may validate, but holder-known time must plan* (`:463`): canonical time validates legality/order while holder-known temporal evidence drives planning and embodied rendering — the five-source temporal bundle must keep validator time distinct from holder-known premises. 0045 amends neither invariant (spec §10.1).
5. Enforcement surface (evidence-consumer basis): the capstone *re-proves* the deterministic-replay, actor-knowledge/truth-firewall, no-leak, and possession/debug-split enforcement surfaces rather than modifying them — replay rebuilds from empty projections and localizes first divergence; paired hidden-truth interventions leave proposals/context/view unchanged until a modeled information event; possession/debug changes only non-diegetic output. Confirm the evidence collection introduces no epistemic-leakage or replay-nondeterminism path and that `EMERGE-OBS` remains observer-only and non-steering (no feedback path, no pass/fail predicate over observed values), preserving INV-111 and execution `10`.

## Architecture Check

1. Folding the multi-seam re-proof into one acceptance-only capstone is cleaner than re-auditing each predecessor subsystem in isolation: the claim being certified is *integrated coherence at `U`*, which only a single-commit composite corpus can establish, and isolated re-audits would both overclaim (treating a historical pass as current) and miss cross-seam regressions. The capstone consumes the prior tickets' evidence and renders one reconciled verdict.
2. No backwards-compatibility aliasing/shims, and no new production logic. The capstone adds no gate code, status enum, obligation code, invariant ID, or doctrine finding; `FIRST-PROOF-CERT` stays a composing label and `EMERGE-OBS` stays observer-only. Evidence counts are re-enumerated from fixtures, not hardcoded.

## Verification Layers

1. Integrated FIRST-PROOF re-proof at `U` → replay/golden-fixture + acceptance check: `FIRST-PROOF-01`…`17`, all nine gates, and all nine scenario families have certifying positive, adversarial/negative, and replay/provenance evidence from the named suites run at `U`, each row computed only from certifying evidence.
2. Determinism & first-divergence localization (INV-018 family) → replay check: per-scenario and capstone rebuild from empty projections produces identical semantic projections/diagnostics, and a controlled tamper localizes the first divergence rather than reporting only a terminal mismatch.
3. No-leak / temporal authority (INV-024/INV-006; INV-112) → manual review + negative tests: paired hidden-truth interventions leave actor-known context/view unchanged until a modeled information event; the five-source temporal bundle keeps validator time distinct from holder-known premises through one traceable representative fact.
4. Observer-only emergence (INV-111) → invariants alignment check: `reports/0045_first_proof_cert_emerge_obs.md` is retrospective, event-log-ancestry-backed, reproducible by corpus/seed, non-input, and counted as no certifying evidence — it can make no gate pass or fail.
5. Artifact / evidence honesty → acceptance-wording check: `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` plus template-conformance review confirm evidence-status/fingerprint-scope/sampling/pending-historical/certification-use fields are honest and the verdict is computed only from certifying evidence.

## What to Change

### 1. Author the live FIRST-PROOF re-proof evidence

Produce the `FIRST-PROOF-01`…`17` routing-matrix rows (spec §7.2), the nine-gate integrated coverage (§7.3), the nine scenario-family coverage (§7.4), and the consolidated five-source temporal bundle (§7.5) — each with positive + required adversarial/negative witness, event/replay/projection evidence where applicable, actor-known/holder-known provenance, responsible-layer diagnostic, exact command transcript at `U`, mutation-identity linkage for affected carriers, and evidence-status/fingerprint-scope fields. Re-enumerate all expected counts from fixtures at test time.

### 2. Author the replay/provenance package and EMERGE-OBS companion

Record the per-scenario and integrated-capstone replay-from-empty-projections rebuild with semantic checksums and first-divergence results (spec §8.7), and create `reports/0045_first_proof_cert_emerge_obs.md` with `Evidence status: observer-only` / `Certification use: not counted as certifying evidence` (spec §8.9) — retrospective, event-log-ancestry-backed, reproducible, non-input, steering nothing.

### 3. Author the replacement acceptance artifact and render the verdict

Create `reports/0045_first_proof_cert_mutation_remediation_replacement_certification_acceptance.md` instantiating every `docs/4-specs/0003` field (header/source/environment/supersession block §8.2, command ledger §8.3, evidence-item ledger §8.4, the eight reconciled verdict tables §8.5, the complete configured mutation `pass` row §8.6, sampling/exhaustiveness declarations §8.8, staged-abstraction declaration §8.10). Render `FIRST-PROOF-CERT passed` **only** when every §8.11 aggregate condition holds, with an explicit statement that it supersedes the 0044 acceptance artifact for current FIRST-PROOF-CERT use and applies only to `U` (not latest `main`/Phase-4/second-proof/deferred surfaces). If any condition is unmet, render `FIRST-PROOF-CERT scoped remediation`, name the failed requirement and first responsible layer, retain the evidence, and route a separately numbered follow-up.

## Files to Touch

- `reports/0045_first_proof_cert_mutation_remediation_replacement_certification_acceptance.md` (new)
- `reports/0045_first_proof_cert_emerge_obs.md` (new)

## Out of Scope

- Building the lane/supervisor/merger or running the mutation campaign (→ -004, -002, -003, -005).
- Fixing any survivor (→ reserved `0045FIRPROCER-007`+); the capstone consumes the completed, zero-floor campaign and re-proves coherence — it authors no production correction.
- Re-auditing 0037/0039/0041/0043 as isolated certification campaigns (spec §7.7) — they are consumed within their own scopes.
- The `docs/4-specs/SPEC_LEDGER.md` row update and the `archive/specs/` move — deferred to spec acceptance/archival per `docs/archival-workflow.md` (Step 6 cross-spec follow-up), not authored here.
- Any Phase-4, second-proof, institution, notice, travel, regional, LOD, story-sifting, or LLM feature work (spec §11).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` passes against the new replacement artifact wording (and the named §4.9 suites pass at `U`, re-run as the capstone's integrated evidence).
2. The replay/provenance evidence rebuilds every required scenario and the integrated capstone from empty projections with identical semantic checksums and localizes a controlled tamper's first divergence.
3. The replacement artifact renders `FIRST-PROOF-CERT passed` only if every §8.11 condition holds (else `scoped remediation` with named failing layer), explicitly supersedes the 0044 artifact, and scopes the claim to `U`.

### Invariants

1. `EMERGE-OBS` is present, observer-only, and counted as no certifying evidence — it can make no gate pass or fail (INV-111); the five-source temporal bundle keeps validator time distinct from holder-known premises (INV-112).
2. The verdict is computed only from certifying evidence; pending/sampled/observer-only/historical items appear as context and cannot silently produce a passing row; no new gate code/status/invariant is minted.

## Test Plan

### New/Modified Tests

1. `None — acceptance-only capstone; verification is command-based (the §4.9 named suites and `acceptance_artifact_wording` re-run at `U`, the replay-from-empty-projections + first-divergence checks, and template-conformance review of the replacement artifact). It adds no production logic; any new assertion needed would belong to an upstream kill ticket, not the capstone.`

### Commands

1. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` and the full §4.9 named-suite set re-run at `U` (core/content/TUI), as the capstone's integrated re-proof.
2. `cargo test --locked -p tracewake-core --test event_schema_replay_gates --test hidden_truth_gates --test no_human_capstone --test emergence_ledger` (replay/first-divergence, truth-firewall non-interference, no-human progress, observer-only emergence) — the integrated coherence surfaces.
3. Template-conformance review of `reports/0045_first_proof_cert_mutation_remediation_replacement_certification_acceptance.md` against `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (manual review is the correct boundary for evidence-honesty/field-completeness, which no `cargo` target asserts).
