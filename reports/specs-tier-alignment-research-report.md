# Specs tier alignment research report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `36b40823fb07752987531ecd142c78505b8f56da` (`36b4082`)  
**Scope:** `docs/4-specs/*` after ratified amendments `0026`, `0027`, and `0028`  
**Deliverable type:** recommendation report only; not a numbered spec, not a replacement template, and not ratified wording

I am not verifying that this commit is the current `main`. I am using the supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

## 1. Disposition table

| Finding / item | Target home | Verdict | One-line basis |
|---|---|---:|---|
| F03 — acceptance-artifact evidence-status and fingerprint-scope fields | `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | **amend** | The template already requires exact commit, gates, changed files, requirement evidence, and scoped certification wording, but its per-requirement table still has bare `Evidence` and `Result` columns; execution `10` and ratified `0028` D9 now make the evidence-honesty fields actionable. |
| `SPEC_LEDGER.md` | `docs/4-specs/SPEC_LEDGER.md` | **boundary-awareness-no-change** | The ledger already lists `0026`, `0027`, and `0028`, and already leaves reference/spec-template follow-through plus F04 as later work; no ledger correction is needed in this recommendation report. |
| Specs README | `docs/4-specs/README.md` | **boundary-awareness-no-change** | The README already states that specs are the lowest tier and that `0003` is a review-artifact template; no doctrine drift found. |
| First-proof ontology spec | `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` | **boundary-awareness-no-change** | The file remains a subordinate first-proof ontology/fixture package; no concrete emergence-evidence or acceptance-honesty drift was found that belongs in this pass. |
| F04 — possession-bind perception | Owner decision, then possible execution `07` or implementation-spec follow-up | **route-forward** | This is not a specs-tier template decision. It remains an owner decision about whether possession binding emits modeled perception. |
| Future certification implementation work | Future implementation spec if maintainers choose | **route-forward** | This report recommends template fields only. Any future P0 certification or code-enactment package is outside this terminal alignment report. |

## 2. Method / provenance ledger

### 2.1 Repository and freshness posture

- Requested repository: `joeloverbeck/tracewake`
- Target commit: `36b40823fb07752987531ecd142c78505b8f56da`
- Freshness claim: user-supplied target commit only; not independently verified as latest `main`
- Manifest role: path inventory only
- Repository metadata used: no
- Default-branch lookup used: no
- Branch-name file fetch used: no
- Code search used: no
- Clone used: no
- URL fetch method: `web.run open` against exact `raw.githubusercontent.com` URLs
- Connector/tool namespace trusted as evidence: no
- Contamination observed: no
- Contamination check: fetched text was checked for forbidden-repository contamination; no fetched source surfaced a different repository as the evidence source.
- Commit-of-record divergence note: predecessor reports contain older baseline strings as historical content. Those strings were not used as fetch targets. All repo evidence in this report is from `36b40823fb07752987531ecd142c78505b8f56da`.

### 2.2 Exact fetched repository files

Every path below is present in the uploaded manifest and was fetched mechanically as:

`https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/<manifest path>`

- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/reports/execution-tier-alignment-research-report.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/reports/architecture-tier-alignment-research-report.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/reports/foundation-amendment-lower-tier-routing.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/3-reference/02_GLOSSARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/4-specs/SPEC_LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/4-specs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`

### 2.3 External research role

External sources were used only to sharpen the template-field recommendations. They do not define repository doctrine. The controlling rule remains execution `10`, as ratified through `0028` D9.

The external research most relevant to this report is:

- Testing-oracle research supports explicit evidence status: without an oracle or witness, observed output is not automatically correctness.
- Metamorphic, property-based, and mutation testing support explicit sampling/exhaustiveness and behavioral-kill claims.
- Approval/golden-testing practice supports snapshot comparison as useful evidence, but only within a declared comparison scope.
- Deterministic simulation testing reinforces replay/provenance ancestry for simulation evidence.
- Structured observability practice supports typed evidence fields rather than free-text logs.

## 3. Authority boundary for this pass

The docs hierarchy makes `docs/4-specs` the lowest tier. A spec-template recommendation may operationalize execution doctrine, but it may not restate, amend, weaken, or supersede foundation, architecture, or execution.

`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` is a review-packet template. Its job is to make evidence honest enough for reviewers to apply execution `10`, not to define what `EMERGE-OBS`, `G-CERT`, `P0-CERT`, replay proof, or gate passage mean.

## 4. F03 — acceptance-artifact template evidence-honesty fields

### 4.1 Upstream driver

Execution `10` is now the governing source for review-artifact honesty. It requires evidence status and fingerprint scope to be labeled, and it rejects silent overcounting of pending, sampled, observer-only, or historical evidence as pass evidence. It also requires behavior witnesses and provenance/replay ancestry where a packet claims path-under-test behavior.

Spec `0028` D9 routed this exact work to the spec template. Its outcome section records `0028` as completed and enacted, so the earlier “wait until execution doctrine exists” condition is now satisfied. This template change is actionable now, not deferred.

The execution alignment report's F03 finding routes the concrete template fields to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`: status, fingerprint scope, sampling/observer-only/pending/historical labels, path-under-test behavior witness, and replay/provenance ancestry.

### 4.2 Current coverage in `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`

Current coverage is useful but too thin for post-`0028` evidence honesty:

- The template already requires a scoped acceptance report with exact commit under test, gates run, changed files, per-requirement evidence, residual convention-only items, certification wording, and reviewer sign-off (`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, lines 0-4).
- The current per-requirement table has only four columns: requirement, responsible layer, evidence, and result (`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, line 4).
- It does not require evidence status, fingerprint scope, sampling/exhaustiveness, observer-only labeling, pending blockers, historical/archive labeling, path-under-test behavior witness, or replay/provenance ancestry.

The template already has the right skeleton. It lacks the fields needed to prevent evidence from being overclaimed.

### 4.3 Tier-fit verdict

This belongs in `0003`. The template is exactly where review-packet evidence fields should live. The fields should be written as packet-structure requirements that point to execution `10`; they should not define new doctrine, new gates, or new identifiers.

### 4.4 Recommendation — substance + home

Home: `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, in the per-requirement evidence table and/or a companion evidence-item ledger inside the template.

Recommended substance:

1. **Add evidence status for each evidence item.** The status vocabulary should match execution `10`:
   - pass/fail where the evidence actually certifies the requirement;
   - pending where proof is not yet present;
   - sampled where evidence is representative but not exhaustive;
   - observer-only where the item can inform review but cannot certify behavior;
   - historical where archive/spec/report evidence supplies context rather than current certification.

   The template should make clear that pending, sampled, observer-only, and historical evidence cannot be silently counted as a pass.

2. **Add fingerprint scope.** Every fingerprint/hash/checksum/snapshot claim should name its actual scope, using the execution taxonomy:
   - raw bytes;
   - normalized serialization;
   - parsed semantic content;
   - command transcript;
   - run seed;
   - replay artifact.

   The packet should not let a fingerprint be cited beyond that scope. A raw-byte hash is not semantic proof; a normalized serialization hash is not raw-file equality; a command transcript is not replay ancestry; a run seed is not behavior certification.

3. **Add path-under-test behavior-witness fields.** For any claimed behavioral pass, the evidence item should identify:
   - the path under test;
   - the command, event, trigger, emitter, or scheduler entry that exercised it;
   - the responsible layer;
   - the accepted/rejected action or validation stage witnessed;
   - the live negative, mutation-style failure, or reason a negative is not applicable;
   - the checked facts or invariants the witness actually supports.

4. **Add replay/provenance ancestry fields.** The packet should include the causal chain needed for review:
   - relevant event-log segment or event identifiers;
   - replay artifact or serialized log reference;
   - seed/randomness/content version where applicable;
   - extraction/projection version for derived evidence;
   - source provenance for any claim that crosses from artifact to semantic behavior.

5. **Add sampling/exhaustiveness scope.** A sampled result should state the population, sample basis, omitted cases, and why the sample is representative. Exhaustive evidence should say what perimeter it exhausts.

6. **Add observer-only handling.** `EMERGE-OBS` and observer-only emergence evidence should be labelable as observer-only evidence. The template must not allow such rows to become certification, gate pass/fail thresholds, scheduler objectives, scenario goals, or code-quality substitutes unless a future upstream spec explicitly changes the doctrine.

7. **Add pending and historical handling.** Pending rows should name the missing proof and owner/blocker. Historical rows should identify whether the artifact is context, lineage, or archived precedent; they should not be counted as current certification.

8. **Separate requirement result from evidence presence.** The requirement-level result should be computable only from certifying evidence. A row can contain useful evidence while the requirement remains pending or failed.

9. **Preserve existing scoped-certification and residual-convention sections.** The existing template's insistence on scoped acceptance language remains right. The amendment should strengthen evidence fields without loosening the template's existing anti-overclaiming posture.

This recommendation intentionally does not produce final template wording. The repo's own amendment process should write the exact table headings and wording.

## 5. Boundary-awareness — `SPEC_LEDGER.md`

### 5.1 Current coverage

`docs/4-specs/SPEC_LEDGER.md` already records `0026`, `0027`, and `0028`. It also already leaves reference/spec-template follow-through and F04 as pending later work. That is exactly the state this session is meant to close for F01/F02/F03 and leave open for F04.

### 5.2 Verdict

No amendment is required in this recommendation report.

A later actual spec/template amendment might update the ledger as part of that repo workflow. This report is not itself a numbered spec and should not be added to the ledger as if it were one.

## 6. Boundary-awareness — specs README

### 6.1 Current coverage

`docs/4-specs/README.md` already states that specs are the lowest tier and must not override higher-tier doctrine. It also lists `0003` as the scoped acceptance artifact template.

### 6.2 Verdict

No amendment is required. The README remains aligned with this pass.

## 7. Boundary-awareness — `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`

### 7.1 Current coverage

`docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` is a live first-proof ontology/fixture contract. It remains subordinate to the higher tiers and does not try to define `INV-111`, `EMERGE-OBS`, or acceptance-evidence honesty. Its fixture and validation posture can continue to rely on execution `09` and architecture `13` for proof semantics.

### 7.2 Verdict

No amendment is required in this pass.

The only caveat is future-facing: if `0001` later serves as evidence in a scoped acceptance packet, the packet should label that evidence's status and scope under the amended `0003` template. The `0001` source document itself does not need an edit merely to anticipate that use.

## 8. Forward-routing appendix

Specs are the terminal tier for this cascade. Nothing routes further down.

The only forward items are:

1. **F04 possession-bind perception owner decision.** The owner must decide whether possession binding emits modeled perception. If it does, future execution/spec work must model and prove that channel. If it does not, possession binding cannot refresh knowledge by implication. This report does not decide it.

2. **Future implementation-spec or certification package.** If maintainers later create a P0 certification or code-enactment spec, it should adopt the `0003` evidence-status/fingerprint-scope fields. This report recommends the fields but does not author the implementation spec.

3. **Actual template amendment workflow.** This report is a recommendation artifact. The repo's amendment process must still convert it into concrete edits, review, and any ledger updates required by project rules.

## 9. Open questions

1. **Should `0003` use a separate evidence-item ledger or extend the per-requirement table directly?** Recommendation: prefer a companion evidence-item ledger referenced by per-requirement rows. It avoids turning a single table into a cluttered grid and lets multiple requirements cite the same evidence item without duplicating status/fingerprint/provenance fields.

2. **Should `EMERGE-OBS` be a dedicated status or a normal evidence item with `status = observer-only`?** Recommendation: use `observer-only` as the status and `EMERGE-OBS` as the execution label/source obligation. This keeps the status taxonomy small and avoids creating a fake gate.

3. **Should historical evidence include archived specs and prior reports in the same category?** Recommendation: yes, for status purposes. A row may distinguish archive/spec/report lineage in a separate source-type field, but all are historical unless the current packet turns them into live certifying evidence.

4. **F04 possession-bind perception remains undecided.** This is an owner decision, not a template-edit decision.

## 10. External research notes

External research supports the template additions:

- Testing-oracle literature: correctness claims need an oracle or witness; otherwise artifacts risk becoming ungrounded acceptance evidence.
- Metamorphic testing: useful when direct oracles are hard, but the relation and scope must be stated.
- Property-based testing: generated tests can increase confidence, but sampling strategy and failure shrinkage do not imply exhaustive proof unless the perimeter is genuinely exhaustive.
- Mutation testing: the useful question is whether tests fail under meaningful wrong changes, which maps well to R-29's “decorative lock” concern.
- Approval/golden testing: snapshots are valuable regression artifacts, but their meaning is bounded by the compared representation.
- Deterministic simulation testing: reproducibility depends on capturing nondeterminism, seed, environment, and replay ancestry.
- Structured observability: typed logs and semantic conventions help reviewers interpret telemetry, but they must still be connected to requirements and behavior witnesses.

## 11. References

### 11.1 Exact-commit repository sources

- `docs/README.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/README.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`
- `reports/execution-tier-alignment-research-report.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/reports/execution-tier-alignment-research-report.md`
- `reports/architecture-tier-alignment-research-report.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/reports/architecture-tier-alignment-research-report.md`
- `reports/foundation-amendment-lower-tier-routing.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/reports/foundation-amendment-lower-tier-routing.md`
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/3-reference/02_GLOSSARY.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/3-reference/02_GLOSSARY.md`
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
- `docs/4-specs/SPEC_LEDGER.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/4-specs/SPEC_LEDGER.md`
- `docs/4-specs/README.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/4-specs/README.md`
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — `https://raw.githubusercontent.com/joeloverbeck/tracewake/36b40823fb07752987531ecd142c78505b8f56da/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`

### 11.2 External sources

- E. Barr et al., “The Oracle Problem in Software Testing: A Survey.” IEEE Transactions on Software Engineering, 2015. `https://dl.acm.org/doi/10.1109/TSE.2014.2372785`
- S. Segura et al., “A Survey on Metamorphic Testing.” IEEE Transactions on Software Engineering, 2016. `https://eprints.whiterose.ac.uk/id/eprint/110335/1/segura16-tse.pdf`
- Hypothesis documentation. `https://hypothesis.readthedocs.io/`
- Stryker Mutator documentation. `https://stryker-mutator.io/docs/`
- ApprovalTests documentation. `https://approvaltestscpp.readthedocs.io/`
- Antithesis, deterministic simulation testing documentation. `https://antithesis.com/docs/resources/deterministic_simulation_testing/`
- OpenTelemetry Logs Data Model. `https://opentelemetry.io/docs/specs/otel/logs/data-model/`
- OpenTelemetry Semantic Conventions. `https://opentelemetry.io/docs/specs/semconv/`
- A. Kreminski et al., “Composable Story Sifting for Simulated Open Worlds.” `https://eprints.soton.ac.uk/502632/2/3723498.3723809.pdf`
- M. Kreminski, “Felt: A Simple Story Sifter.” `https://mkremins.github.io/publications/Felt_SimpleStorySifter.pdf`
