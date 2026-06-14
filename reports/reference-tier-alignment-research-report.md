# Reference tier alignment research report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `36b40823fb07752987531ecd142c78505b8f56da` (`36b4082`)  
**Scope:** `docs/3-reference/*` after ratified amendments `0026`, `0027`, and `0028`  
**Deliverable type:** recommendation report only; not a replacement reference document and not ratified wording

I am not verifying that this commit is the current `main`. I am using the supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

## 1. Disposition table

| Finding / item | Target home | Verdict | One-line basis |
|---|---|---:|---|
| F01 — canonical observer-only emergence-evidence term | `docs/3-reference/02_GLOSSARY.md` | **amend** | The glossary owns compact naming guidance and already distinguishes evidence, observation, projection, and story sifting, but it has no emergence-evidence term and no `EMERGE-OBS` cross-reference; `0026` D4 and the execution hand-off explicitly route this term here. |
| F02 — acceptance-evidence-honesty risk cluster | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | **amend** | R-27/R-28/R-29 already partially own path-under-test overstatement, fingerprint-payload drift, and decorative-lock vacuity; they need a compact realignment to the six evidence-honesty distinctions now enforced by execution `10`, without redefining those execution rules. |
| F02 anti-Goodhart watch note | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | **amend, folded into existing cluster** | The register already has R-22 story-sifting-becomes-direction, R-16 no-human proof failure, and R-27 reachability overstatement; observer-only emergence counters becoming simulation objectives should be watched through those existing homes unless the repo's reassess process later chooses a new risk identifier. |
| Reference index / review checklist | `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | **boundary-awareness-no-change** | The index already owns tier boundaries, source discipline, and checklist routing; a single future checklist question on evidence-status/fingerprint-scope honesty would be acceptable, but this pass does not require it. |
| F04 — possession-bind perception | Owner decision, then possible execution `07` follow-up | **route-forward** | This is explicitly not a reference-tier doctrine decision; it remains an owner choice about whether possession binding emits modeled perception. |

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

External sources were used only to sharpen the evidence-honesty and observation-vs-direction recommendations. They do not define repository doctrine. The repository's foundation, architecture, execution, and archived amendment specs remain authoritative.

The external research most relevant to this report is:

- Testing oracle problem: surveys of oracle automation and its limits support the recommendation that a report distinguish certified behavior from supporting artifacts.
- Metamorphic and property-based testing: these support explicit sampling/exhaustiveness claims rather than vague “tested enough” language.
- Mutation testing: surviving mutants are a useful analogy for decorative locks that exist without behavior-killing proof.
- Approval/golden testing: snapshot/fingerprint stability is useful but scope-limited; it must not be overclaimed as semantic proof.
- Structured observability practice: typed, named observability reduces evidence ambiguity, but telemetry artifacts still require honest status and provenance.
- Emergent narrative / story-sifting literature: emergent-pattern observation is valuable exactly when it is kept separate from directing the generative substrate.

## 3. Authority boundary for this pass

The docs hierarchy says foundation governs architecture, architecture governs execution, and reference/specs are lower-tier operational aids. `docs/README.md` places `docs/3-reference` below execution and describes it as a glossary/risk-register layer, while `docs/4-specs` is the lowest spec-work layer. This report therefore does not redefine `INV-111`, `EMERGE-OBS`, gate semantics, proof mechanics, or acceptance doctrine. It recommends only reference-tier naming and risk-memory changes.

The reference index says the reference layer owns compact review memory, naming, and guardrails; it may point to gate codes as lookups, but it must not turn itself into the authority that defines those gates. That is the controlling tier-fit rule for every recommendation below.

## 4. F01 — canonical observer-only emergence-evidence term

### 4.1 Upstream driver

The constitutional driver is `INV-111`: living-world acceptance may use observer-only emergence evidence, but the evidence is retrospective, ancestry-backed, and non-input to simulation behavior or dramatic objectives. Foundation `09` draws the same bright line: observer layers may record and sift events after the fact, but they may not steer the runtime. Foundation `12` adds the first-playable acceptance posture: the emergence record sits beside mandatory gates, does not certify by itself, and does not license authored outcomes.

Spec `0026` is the direct reference-tier routing source. Its D4 deliverable routes a glossary term to `docs/3-reference/02_GLOSSARY.md` and says the term must not be collapsed into existing entries such as evidence, observation, projection, holder-known context, or stale information. Architecture `13` then gives the observer-only evidence row its one-way architecture contract. Execution `00` classifies `EMERGE-OBS` as an observation obligation rather than a gate, and execution `10` operationalizes it as a non-certifying extraction/ledger mechanism.

The execution alignment report is the freshest seed for this pass. Its F01 finding routes a glossary term with the exact properties this report preserves: after-the-fact, non-certifying, event-log-ancestry-backed, non-steering, and structurally non-input.

### 4.2 Current coverage in `docs/3-reference/02_GLOSSARY.md`

Current coverage is partial but not sufficient:

- The glossary itself sets the correct altitude: it owns naming only, not authority or doctrine (`docs/3-reference/02_GLOSSARY.md`, lines 0-12).
- It already has distinct entries for evidence, evidence ledger, observation, projection, provenance, salience, and story sifting (`docs/3-reference/02_GLOSSARY.md`, lines 31-67).
- It already treats gate codes as exact cross-references to execution, not local definitions (`docs/3-reference/02_GLOSSARY.md`, line 35).
- Searches within the fetched glossary found no `emergence`, no `observer-only`, and no `EMERGE-OBS` entry.

That means the glossary is correctly structured for F01, but the routed term is absent.

### 4.3 Tier-fit verdict

This belongs in the glossary. The glossary must not restate the full `INV-111` doctrine, but it should give reviewers one canonical name for the artifact class and point upward to the governing sources.

The recommended canonical term is **observer-only emergence evidence**.

This term is better than alternatives such as “emergence ledger,” “story evidence,” or “emergent-narrative evidence” because it embeds the two properties most likely to be lost during later implementation: it is observer-only, and it is evidence rather than a control signal. It also avoids overloading “story sifting,” which the current glossary already names as a separate observer-side process.

### 4.4 Recommendation — substance + home

Home: `docs/3-reference/02_GLOSSARY.md`, in the canonical/context-bound terms section near `evidence`, `observation`, `projection`, and `story sifting`.

Recommended substance:

- Add **observer-only emergence evidence** as the canonical term for the after-the-fact artifact used to document emergent outcomes under `INV-111`.
- The term must carry all five required properties:
  - **Retrospective / after-the-fact:** produced from completed event-log/replay ancestry, not planned in advance as a dramatic target.
  - **Non-certifying:** useful acceptance context, but never sufficient by itself to pass behavior gates or certify living-world correctness.
  - **Ancestry-backed:** tied to causal event-log ancestry, replay/extraction provenance, and the phenomenon-family row contract described in architecture/execution docs.
  - **Non-steering:** never feeds cognition, scheduler, validators, authoring choices, seed selection, scenario goals, pacing, LOD, difficulty, or outcome gates.
  - **Structurally non-input:** the simulation must not read it as state, input, planner data, content-selection data, or pass/fail threshold.
- Cross-reference upward, not sideways, to:
  - `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` / `INV-111`
  - `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
  - `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
  - `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
  - `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
  - `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` / `EMERGE-OBS`
- Explicitly distinguish the term from existing glossary entries:
  - It is not ordinary `evidence`; ordinary evidence may support many claims, while this term names a special observer-only acceptance artifact.
  - It is not `observation`; observation can be diegetic or observer-side depending on context, while this term is a non-input post-run artifact.
  - It is not `projection`; a projection is a derived view/model, while this term names an acceptance-evidence class backed by ancestry.
  - It is not `story sifting`; story sifting identifies salient patterns, while this term names the bounded artifact used to report selected emergent phenomena without steering them.
- Add or consider a compact `EMERGE-OBS` cross-reference entry. It should not define new gate semantics. It should say only that `EMERGE-OBS` is the execution-tier observation-obligation label for observer-only emergence-evidence records, and the actual semantics live in execution `00` and `10`.

## 5. F02 — risk-register realignment for acceptance-evidence honesty

### 5.1 Upstream driver

The risk-register driver is the now-ratified evidence-honesty rule in execution `10`, routed by spec `0028` D9 and by the execution alignment report's F02 finding. Execution `10` requires review packets to distinguish status and fingerprint scope: pass/fail where actually certified, pending, sampled, observer-only, and historical; and raw bytes, normalized serialization, parsed semantic content, command transcript, run seed, and replay artifact. It also states that pending, sampled, observer-only, and historical evidence cannot be silently counted as a pass.

Architecture `13` supplies the upstream architecture reason: artifact existence, artifact shape, checksums, display text, and ad hoc visibility are not enough. Acceptance proof needs typed path-under-test observability and behavior witnesses.

Spec `0027` forward-routes the evidence-status/fingerprint-honesty problem to execution, reference, and the acceptance template. Spec `0028` completes execution D9, making the reference update actionable now.

### 5.2 Current coverage in `docs/3-reference/01_DESIGN_RISK_REGISTER.md`

Current coverage is substantial but pre-`0028` in its vocabulary:

- The risk register correctly limits itself to compact risk memory and explicitly says it may cite gate codes only as cross-references, not as local definitions (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`, lines 0-4).
- R-27 already covers acceptance-evidence reachability overstatement. It warns against treating harnesses, fixtures, or adjacent code as proof that the path under test ran, and it asks reviewers to count only path-under-test emissions with named commands/emitters (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`, lines 96-99).
- R-28 already covers incomplete correction closure and includes a fingerprint-payload pitfall: parsed/re-serialized payload fingerprints can hide raw-byte drift or secondary-file content (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`, lines 99-106).
- R-29 already names guard-vacuity / decorative-lock relapse. It warns about artifacts that exist, hash, count, or look structured without a failing negative case or live behavior proof (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`, lines 106-114).
- R-26 already warns against treating archived historical work as current certification (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`, lines 91-95).
- R-22 already owns the story-sifting-becomes-direction failure mode (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`, lines 77-80).
- R-16 already owns no-human ordinary-life proof failure (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`, lines 58-61).
- Searches within the fetched register found no explicit `emergence`, `observer-only`, or `Goodhart` language.

The register is therefore partially aligned. It has the right risk homes, but it does not yet speak the post-`0028` evidence-honesty distinctions explicitly enough for reviewers to use the cluster reliably.

### 5.3 Tier-fit verdict

This belongs in the risk register as a compact cluster realignment, not as new doctrine and not as a gate definition. The register should cross-reference execution `10` for the actual rule, then record the relapse patterns reviewers are likely to miss.

Keep R-29 as the named guard-vacuity / decorative-lock relapse risk. Do not replace it with a new number or move it to the spec template. The evidence-honesty distinction should be threaded through R-27/R-28/R-29 because those entries already own the failure modes.

### 5.4 Recommendation — substance + home

Home: `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, in the existing R-27/R-28/R-29 cluster. Use cross-references to execution `10`; do not restate the execution rule as a local reference-tier definition.

Recommended cluster realignment:

1. **R-27 / acceptance-evidence reachability overstatement** should explicitly watch the first three evidence-status mistakes:
   - pending evidence being counted as a pass;
   - sampled evidence being described as exhaustive certification;
   - observer-only evidence, including `EMERGE-OBS`, being treated as a gate or behavior certificate.

2. **R-28 / incomplete correction closure** should expand its fingerprint-payload warning into the post-`0028` scope taxonomy:
   - raw bytes;
   - normalized serialization;
   - parsed semantic content;
   - command transcript;
   - run seed;
   - replay artifact.

   The risk memory should warn that a fingerprint proves only the scope it actually covers. A byte fingerprint is not a semantic proof; a parsed-content fingerprint is not raw-byte stability; a transcript hash is not replay ancestry; and a run seed is not a behavior witness.

3. **R-29 / guard vacuity and decorative locks** should remain the named home for artifact-presence overclaiming. Extend its symptom list to include status rows, ledgers, checksums, template tables, `EMERGE-OBS` rows, archived spec/report references, and fixture artifacts that exist without behavior witnesses. The mitigation remains behavioral: synthetic negative cases, live path-under-test proof, or a clearly scoped reason why no negative is possible.

4. **R-26 / archived Phase 3A treated as post-overhaul certification** should be cross-linked where useful, because archive/history is context rather than current certification. This does not require a new risk entry.

5. **R-22 + R-27 + R-16 anti-Goodhart watch note:** add a compact watch note under the existing cluster for the specific observer-only emergence-evidence relapse: counters, phenomenon families, story-sifted rows, or emergence ledgers becoming seed selectors, scheduler inputs, scenario objectives, pacing knobs, difficulty targets, LOD inputs, or pass/fail thresholds. The note should point to R-22 for “observation becomes direction,” R-16 for no-human proof pressure, R-27 for evidence overstatement, and execution `10` for the non-input rule.

The six distinctions to make visible in the cluster are:

- **pending ≠ pass**
- **sampled ≠ certifying**
- **observer-only ≠ gate**
- **byte-fingerprint ≠ semantic proof**
- **archive/history ≠ certification**
- **artifact-presence ≠ behavior-witness**

Do not invent a new `R-##` in this report. If maintainers later decide the anti-Goodhart relapse has become common enough to deserve its own entry, that should happen through the repo's reassess/amend process.

## 6. Boundary-awareness — reference index / review checklist

### 6.1 Current coverage

`docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` already sets the reference-tier boundary, identifies the index/glossary/risk-register roles, and includes exact-source discipline and review questions. It does not currently contain a dedicated evidence-status/fingerprint-scope question, but the file is intentionally small and low-duplication.

### 6.2 Verdict

No required amendment.

A future lightweight checklist question would be reasonable if maintainers want one canonical reviewer reminder. The substance would be: when a review depends on acceptance evidence, confirm that the packet labels evidence status and fingerprint scope and does not count pending, sampled, observer-only, or historical artifacts as pass evidence. That would be a pointer to execution `10`, not a local restatement. Because this pass already amends the risk register and spec template, the index can remain unchanged unless maintainers want the extra reviewer prompt.

## 7. Forward-routing appendix

Reference and specs are terminal lower tiers for this cascade. Nothing routes further down.

The only forward items are:

1. **F04 possession-bind perception owner decision.** Architecture and execution both leave this undecided: possession binding may or may not emit modeled perception. The owner must decide. If yes, it must become a modeled perception channel and later execution/spec work must prove it. If no, possession binding must not freshen knowledge by implication. This report does not decide the question.

2. **Possible implementation-spec follow-up.** If maintainers later create a certification or template-enactment implementation spec, it should implement the evidence-status and fingerprint-scope fields recommended here and in the specs-tier report. This report does not create that implementation spec.

## 8. Open questions

1. **Should `EMERGE-OBS` get its own glossary entry?** Recommendation: yes, but only as a compact execution-label cross-reference. The canonical conceptual term should be `observer-only emergence evidence`; `EMERGE-OBS` should remain an execution observation-obligation label, not a new reference-tier gate definition.

2. **Should the reference index gain one acceptance-honesty checklist question?** Recommendation: optional. The index is intentionally small; the risk register and template are better primary homes. Add a single pointer only if reviewers repeatedly miss evidence-status/fingerprint-scope honesty.

3. **Should anti-Goodhart emergence-evidence pressure get a new risk entry?** Recommendation: not in this pass. Fold it into R-22/R-27/R-16/R-29. Escalate to a new risk only if future review work shows the failure mode recurring independently.

4. **F04 possession-bind perception remains undecided.** This is an owner decision, not a reference-tier editorial matter.

## 9. External research notes

External research supports the recommendations but does not override repository doctrine.

- The testing-oracle literature explains why tests need explicit oracles and why artifacts that merely exist do not settle behavioral truth. That supports R-29's behavior-witness emphasis.
- Metamorphic and property-based testing both reinforce the need to state sampling/exhaustiveness scope rather than imply universal certification.
- Mutation testing is a useful review analogy for decorative locks: if a wrong behavior can survive, the guard is not proven by its existence.
- Approval/golden testing is useful for detecting output changes, but snapshot stability is only evidence at the comparison scope. It is not semantic truth unless paired with semantic checks and behavior witnesses.
- Deterministic simulation testing reinforces the importance of reproducible replay ancestry for simulation proof.
- Structured-observability practice supports typed evidence fields and stable names; it does not eliminate the need to say what the telemetry certifies.
- Emergent-narrative and story-sifting research supports the separation between observing/sifting emergent patterns and directing the generative process toward those patterns.

## 10. References

### 10.1 Exact-commit repository sources

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

### 10.2 External sources

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
