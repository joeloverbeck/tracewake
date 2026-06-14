# Execution-tier alignment research report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `64a8367ca54f5daf97dac7031a708476d31a3707`  
**Intended destination:** `reports/execution-tier-alignment-research-report.md`  
**Report type:** execution-tier conformance analysis and change proposal for `docs/2-execution/*`.

I am not verifying that this commit is the current `main`. I am using the supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

---

## 1. Disposition table

| ID | Source | Target execution doc(s) | Verdict | One-line basis |
|---|---|---|---|---|
| E00 | Swept | `docs/2-execution/00`, `02`, `03`, `06`, `07`, `08`, `09`, `10`, `11` | belongs-in-execution | The index identifies canonical gate/obligation names, but later execution docs still use phase-cert labels and older gate-like labels without a clear distinction; execution owns gate vocabulary and certification sequence, so the tier needs a vocabulary reconciliation pass. |
| E01 | Seed E + swept | `docs/2-execution/02`, cross-reference `03`, `09`, `10` | belongs-in-execution | Current foundation acceptance now requires retrospective observer-only emergence evidence beside mandatory proof gates; the first-proof acceptance contract still lists gate/scenario artifacts without making that evidence artifact part of the first-proof package. |
| E02 | Seed A / D3 | `docs/2-execution/04`, `10` | belongs-in-execution | Execution already requires provenance on actor-known inputs, but architecture now requires a shared provenance-sufficiency proof: missing, empty, dangling, wrong-kind, ambiguous, harness-fabricated, or forbidden-source provenance must fail closed on the path under test. |
| E03 | Seed B / D4 | `docs/2-execution/04`, `07`, `10` | belongs-in-execution | Execution mentions staleness, but current architecture requires one memory-freshness classifier across actor-known, no-human, TUI, notebook, and holder-known surfaces, with no restamping-by-replay or possession freshening. |
| E04 | Seed C / D5 + residual | `docs/2-execution/07`, `10`, support in `04` | belongs-in-execution | View/possession proof exists, but architecture now requires observation-time believed-access snapshots and carrier evidence; execution should add wallhack negatives, snapshot proof, and an embodied carrier census, while carrying the unresolved possession-bind perception decision. |
| E05 | Seed D / D6 | `docs/2-execution/05`, `06`, `09`, `10` | belongs-in-execution | Execution has strong single-tick need accounting proof, but architecture now names a single-owner derived-accounting seam; execution should prove no duplicate charge, no scheduler/action/projection drift, and no byte-stable-but-false golden acceptance. |
| E06 | Seed E / D1 / `0026 §6` | `docs/2-execution/10` | belongs-in-execution | `EMERGE-OBS` is already declared and non-certifying, but doc `10` still references older invariants and lacks the current arch `13` observer-only data contract fields and invalid-pass conditions. |
| E07 | Seed F / D7 | `docs/2-execution/10`, support in `04`, `09` | belongs-in-execution | Execution has mutation/pending language, but the current architecture demands typed path-under-test behavior witnesses; execution needs a general anti-vacuity rule for live negatives, behavior witnesses, and explicit no-negative rationales. |
| E08 | Seed G / X10 | `docs/2-execution/10` | belongs-in-execution, with route-forward appendices | Acceptance-evidence honesty is only partial: execution needs a status/fingerprint/sampling honesty rule, while risk-register wiring and template wording belong later in `3-reference/01` and `4-specs/0003`. |
| E09 | Residual + D3/D4 | `docs/2-execution/11` | belongs-in-execution | Phase-4 institution proof already discusses institution-known context and records, but it should explicitly inherit provenance sufficiency and freshness proof mechanics so future institution-known facts cannot be truth-converted. |
| E10 | Swept / external-method delta | `docs/2-execution/13` | belongs-in-execution | The execution research notes cover determinism and property tests, but not the proof-method sources now load-bearing for execution doctrine: mutation, metamorphic, approval/golden semantics, structured observability, and the test-oracle problem. |
| C01 | Seed E | `docs/2-execution/00`, `10` | already-owned-close | Execution already declares `EMERGE-OBS` as an observation obligation, explicitly not a certification gate, non-blocking, observer-only, and barred from becoming input or pass/fail threshold; the mechanism still needs E06. |
| C02 | Swept | `docs/2-execution/01` | already-owned-close | The archived-spec posture is already correct: archive is historical evidence, not live certification, and P0 certification must re-audit live code/docs. |
| C03 | Swept | `docs/2-execution/08` | already-owned-close | Data-authoring/schema proof already forbids outcome-chain data and requires provenance-bearing content facts at the execution proof level. |
| C04 | Swept | `docs/2-execution/12` | already-owned-close | Second-proof, travel, regional scale, and story-sifting remain locked deferrals; current text keeps story-sifting read-only and non-interventionist, matching the upstream boundary. |
| F01 | Seed E / `0026 D4` | `docs/3-reference/02_GLOSSARY.md` | route-forward | The canonical term for the emergence-evidence artifact is intentionally not coined in execution; reference terminology control should add it later. |
| F02 | Seed G / X10 | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | route-forward | The R-27/R-28/R-29 acceptance-evidence honesty cluster belongs in the risk register, not as execution gate prose. |
| F03 | Seed G / X10 | `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | route-forward | Template wording for statuses, sampled evidence, fingerprints, and pending evidence belongs in the acceptance-artifact template after execution doctrine is realigned. |
| F04 | Residual | owner decision, then `docs/2-execution/07` | route-forward to owner decision | Whether possession bind itself counts as perception is not an execution decision; after the owner decides, execution `07` should prove the chosen policy. |

**Summary verdict.** The execution tier is not fundamentally misaligned: the hardening campaign already installed much of the proof posture. The gaps are concentrated in four places: current upstream doctrine now needs sharper proof mechanics in `04`, `06`, `07`, and especially `10`; the first-proof package in `02` does not yet carry observer-only emergence evidence as an artifact; `11` needs institution-known provenance future-proofing; and `13` needs the external proof-method research notes that now justify execution-level evidence doctrine.

---

## 2. Method and provenance ledger

### Required exact-commit ledger

Requested repository: joeloverbeck/tracewake  
Target commit: 64a8367ca54f5daf97dac7031a708476d31a3707  
Freshness claim: user-supplied target commit only; not independently verified as latest main  
Manifest role: path inventory only  
Repository metadata used: no  
Default-branch lookup used: no  
Branch-name file fetch used: no  
Code search used: no  
Clone used: no  
URL fetch method: web.open exact raw.githubusercontent.com URL fetches  
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/reports/foundation-amendment-lower-tier-routing.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/reports/architecture-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/reports/foundation-amendment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/reports/foundation-amendment-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
Contamination observed: no source-fetch contamination; every source file used for Tracewake doctrine was fetched from the exact `joeloverbeck/tracewake` raw URL above. Bibliographic URLs appearing inside fetched Tracewake documents were not treated as repository/source evidence.  
Connector/tool namespace trusted as evidence: no

### Provenance convention used in this report

This report separates three evidence classes.

1. **Tracewake source doctrine** means only the fetched exact-commit files listed above. Inline source references use document path plus the line ranges returned by the exact URL fetches. The uploaded manifest is treated only as path inventory.
2. **External research** means the proof-methodology sources listed in §9. These sources sharpen execution proof design; they do not replace Tracewake doctrine.
3. **Inference/recommendation** means this report's analysis of what execution must prove. Recommendations are deliberately framed as substance plus home. They are not final paste-ready execution prose, do not invent new gate codes or observation-obligation codes, and do not amend foundation or architecture.

No code/certification audit was performed. The task is doc-tier conformance. Existing crates, fixtures, or tests may later corroborate a gate, but this report does not certify `P0-CERT`, replay acceptance, or baseline code status.

---

## 3. Upstream delta: what execution must now realize

The authority boundary is not ambiguous. The repository documentation says foundation is constitutional doctrine, architecture is subsystem/data-flow contract, execution owns gate order, proof contracts, fixtures, audit criteria, and review artifacts; lower tiers may specialize but not weaken higher tiers. If execution conflicts with foundation or architecture, execution is wrong. That means this pass should not re-decide product identity or subsystem ownership; it should translate the current foundation and architecture into proof obligations.

### 3.1 Truth-firewall family is now constitutional and execution-visible

Current foundation `02` defines the truth-firewall family as constitutional invariants. The core rule is `INV-099`: truth may validate actions but may not plan them. It is reinforced by `INV-100` through `INV-110`: hidden-truth cognition is forbidden; the action-decision transaction must use a sealed actor-known context; action-relevant cognition requires provenance; the scheduler is not cognition authority; routines and needs may not dispatch direct behavior; diagnostics must be typed; validation failure may not leak truth; debug is quarantined; possession is cognition-neutral; LLMs have no cognition authority; and LOD/regional processes must preserve the firewall (`docs/0-foundation/02`, lines 65-78). Foundation `14` restates the same doctrine in transaction form: assemble a sealed actor-known context, propose candidates only from that context, validate against authoritative truth, emit feedback/events, and replay from events rather than hidden state (`docs/0-foundation/14`, lines 0-11).

**Execution proof obligation.** Execution must prove that action proposal, routine continuation, no-human behavior, possession, debug panels, LLM/speech boundaries, and diagnostics all respect the plan/validate split. A passing test cannot be accepted if it gets behavior by raw state, debug projection, string diagnostics, direct routine dispatch, or truth-derived proposals.

### 3.2 Provenance sufficiency is now shared, not local

Foundation `04` requires every action-relevant belief, expectation, contradiction, routine premise, institutional fact, record, artifact interpretation, speech claim, inventory/placement assertion, hazard/source/shelter/access fact, and delegated fact to carry provenance. It names unacceptable provenance: raw world-state lookup, debug view, branch/test label, UI marker, convenience boolean, display string, session memory, LLM prose, and fixture intent (`docs/0-foundation/04`, lines 35-40). Architecture `03` now turns this into a shared sufficiency rule: missing, empty, dangling, wrong-kind, ambiguous, or forbidden provenance must fail closed; institution-known facts are not exempt (`docs/1-architecture/03`, lines 3-8).

**Execution proof obligation.** Execution must require tests and review artifacts that show unbacked actor-known and institution-known facts fail closed on the actual path under test. Harness-produced provenance must be real event/projection ancestry, not decorative metadata or fixture labels.

### 3.3 Memory freshness is now one classifier across surfaces

Architecture `06` requires a single classifier: `observed_now` can mean only current modeled perception/contact/search; older facts are remembered, believed, stale, contradicted, hearsay, record-derived, or unknown, and must preserve source event, acquisition time, last verified time, and provenance. The same classifier applies to no-human runs, TUI possession, notebooks, and holder-known contexts; possession/UI may not freshen knowledge by re-reading truth (`docs/1-architecture/06`, lines 7-9).

**Execution proof obligation.** Execution must prove no actor, possessed view, notebook, no-human surface, or institution context silently restamps old memory as current observation. Fixtures need positive and negative cases for stale-but-plausible knowledge, superseded records, and possession/no-human parity.

### 3.4 Believed-access snapshots are now a contract, not a UI convenience

Foundation `06` distinguishes actual affordance from perceived/known affordance: actors propose from actor-known/perceived access and validators then check authoritative truth (`docs/0-foundation/06`, lines 25-27). Architecture `10` now requires embodied view models and possession preflight to consume holder-known plus permitted projection records captured at modeled observation/bind/preflight/perception boundaries. They may not re-read truth or freshen by live state; validation may reject on truth but must keep feedback separate (`docs/1-architecture/10`, lines 3-7).

**Execution proof obligation.** Execution must prove observation-time snapshots: an actor-visible action or possessed view must be backed by a captured modeled-observation carrier, not by a live physical-state handle. Wallhack negatives should fail when a route, workplace, item, sleep affordance, or container content is true in the world but not known/perceived by the holder.

### 3.5 Derived accounting has a single-owner seam

Architecture `04` and `05` preserve proposal/validation/scheduling separation and add a single-owner derived-accounting seam: decision transactions may consume live need pressures and may produce action proposals, but they may not mint need deltas or charge durations directly; derived consequences belong at the action-pipeline/ordinary-life boundary (`docs/1-architecture/04`, lines 0-11; `docs/1-architecture/05`, lines 5-8).

**Execution proof obligation.** Execution must prove that need deltas, duration ticks, work completion/failure, and passive windows are charged exactly once by the owning seam, not independently by scheduler, routine planner, projection, replay, or golden-output normalization.

### 3.6 Observer-only emergence evidence is now foundation acceptance evidence

Current `INV-111` requires living-world acceptance to include replayable observer-only evidence that unscripted ordinary-life phenomena arise from modeled causes in no-human or normal-controller runs. The evidence must be retrospective, event-log-ancestry-backed, and structurally unable to feed simulation behavior, author outcomes, or set objectives (`docs/0-foundation/02`, lines 77-78). Foundation `09` adds that emergence observation is not steering: observer records are allowed and expected, but any steering to enrich the record is authored-outcome machinery (`docs/0-foundation/09`, lines 19-20). Foundation `12` requires the first proof to include retrospective emergence-evidence records beside mandatory gates, with observer-only and non-certifying constraints (`docs/0-foundation/12`, lines 23-25).

Architecture `13` turns that into a data contract: the observer-only record must identify the run, seed/randomness provenance, controller mode, phenomenon family, source events or causal-chain references, extraction time, review/projection version, and enough ancestry for replay explanation. It is invalid if fabricated from debug/fixtures instead of run events, if it feeds scheduler/cognition/validators, or if it is untraceable (`docs/1-architecture/13`, lines 2-8; lines 13-17).

**Execution proof obligation.** Execution must keep `EMERGE-OBS` non-certifying and observer-only, but must define the evidence artifact shape, invalid-pass conditions, anti-Goodhart posture, and review handling at execution level.

### 3.7 Typed observability is now evidence, not decoration

Architecture `13` requires typed path-under-test observability: responsible layer, source event/proposal/context IDs, checked facts, behavior-witness fields, accepted/rejected stage, and enough ancestry to show the evidence was produced by the path under test. Artifact presence, shape, count, checksum, or display text alone is not enough (`docs/1-architecture/13`, lines 6-8, 13-17).

**Execution proof obligation.** Execution must upgrade review artifacts, diagnostics, mutation/metamorphic guards, and fixture acceptance to require behavior witnesses and live negatives. Pending, sampled, observer-only, and byte-stable evidence must not be mislabeled as certification.

---

## 4. Validation of routed items

The predecessor routing memo named execution destinations for themes A-G plus the emergence mechanism: A provenance sufficiency to `04`/`10`; B memory freshness to `04`/`10`; C believed access to `04`/`07`/`10`; D single-charge accounting to `06`/`10`; F anti-vacuity to `10`; G acceptance/fingerprint honesty to `10` plus possible template/risk-register routes; and E emergence mechanism to `10` as `EMERGE-OBS` (`reports/foundation-amendment-lower-tier-routing.md`, lines 12-20). The architecture report's forward-routing appendix sharpened those same hand-offs after the architecture tier adopted the contracts: `04`/`10` for provenance and freshness, `04`/`07`/`10` for believed-access, `06`/`10` for accounting, `10` for anti-vacuity and acceptance honesty, `11` for institution-known future proof, and `07` after the possession-bind decision (`reports/architecture-tier-alignment-research-report.md`, lines 127-137). Spec `0027` confirms that proof mechanics for D1-D7 route to execution docs `04`, `06`, `07`, `10`, and `11`, while glossary/risk/template issues route to lower later sessions (`archive/specs/0027`, lines 42-45).

| Seed | Routed intent | Live execution verdict | Finding(s) |
|---|---|---|---|
| A | Prove provenance sufficiency and fail-closed behavior for unbacked facts. | Partial. `04` has provenance tables and invalid patterns, but does not fully encode the current shared sufficiency rule or path-under-test harness-fabrication negatives. | E02 |
| B | Prove memory freshness / no restamping. | Partial. `04` and `07` mention stale belief, but not the one classifier across actor/no-human/TUI/notebook/institution surfaces. | E03 |
| C | Prove believed access and wallhack negatives. | Partial. `07` has strong view/debug proof, but not architecture's observation-time snapshot/carrier proof. | E04 |
| D | Prove single-charge derived accounting. | Partial-to-strong. `06` already has single tick and duration accounting proof, but not the named single-owner seam across scheduler/action/projection/golden artifacts. | E05 |
| E | Define observer-only emergence-evidence mechanism. | Split. Declaration is already owned and closed in `00`/`10`; mechanism alignment to current `INV-111`/arch `13` is partial. | C01 + E01 + E06 |
| F | Prove anti-vacuity: live negatives and behavior witnesses. | Partial. `10` mentions mutation and pending mutants, but lacks a general every-lock rule and typed behavior-witness standard. | E07 |
| G | Prove acceptance-evidence/fingerprint honesty. | Partial. `10` has review templates and pending mutation language, but lacks a general evidence-status and fingerprint-scope honesty rule. | E08 + F02 + F03 |
| Residual | Institution-known provenance. | Partial. `11` has institution-known context, records, and provenance language but does not explicitly inherit current provenance sufficiency/freshness mechanics. | E09 |
| Residual | Possession-bind perception. | Not decided by execution. `07` can prove a chosen policy later. | F04 |

---

## 5. Per-finding analysis and recommendations

### E00 — Reconcile execution gate, certification, and observation-obligation vocabulary

**Target.** `docs/2-execution/00`, with cleanup references in `02`, `03`, `06`, `07`, `08`, `09`, `10`, and `11`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** Execution owns gate order, certification sequence, fixtures, audit criteria, and review artifacts, while foundation/architecture own product doctrine and subsystem contracts. The authority table allows execution to define phase gates and certification expectations, but not to weaken upper-tier doctrine.

**Current execution coverage.** `docs/2-execution/00` is explicit that the execution index owns the execution map and canonical gate/obligation vocabulary. It lists canonical gates such as `P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, and `DIAG`; it separately declares `EMERGE-OBS` as an observation obligation, not a certification gate (`docs/2-execution/00`, lines 10-14). Later execution documents use additional gate-like labels or phase-certification labels, including examples such as `EVENT`, `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `NO-HUMAN-ORDINARY-LIFE`, `MISSING-PROPERTY`, `VIEW-DEBUG-SPLIT`, `FIXTURE-NEGATIVE`, `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, `DATA-CERT`, `FIXTURE-CERT`, and `DIAG-CERT` (`docs/2-execution/02`, lines 4-9; `03`, lines 2-10; `06`, lines 14-17; `07`, lines 11-13; `08`, lines 9-11; `09`, lines 12-13; `10`, lines 19-21; `11`, lines 8-10). Some of these may be valid phase-cert labels rather than canonical gates, but the index does not currently make that distinction visible enough.

**Tier-fit verdict.** This is execution-owned because it concerns gate order, certification sequence, and review vocabulary. It is not a foundation or architecture amendment.

**Recommendation.** Add a vocabulary reconciliation note in `00`, then apply cross-reference cleanup in the affected execution docs. The substance should distinguish: existing canonical gates; observation obligations such as `EMERGE-OBS`; phase-certification artifacts; and informal shorthand. Do not invent new codes and do not rename established phase artifacts unless a later reassess session chooses to do so. The key proof obligation is that an execution reader must not mistake an observation obligation for a blocking certification gate, or a phase-cert label for a new doctrine-bearing gate.

---

### E01 — First-proof acceptance must carry observer-only emergence evidence beside gates

**Target.** `docs/2-execution/02`, with cross-reference support in `03`, `09`, and `10`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** Foundation `12` now requires first proof to include retrospective emergence-evidence records beside mandatory gates, without turning those records into steering or certification (`docs/0-foundation/12`, lines 23-25). `INV-111` requires replayable observer-only evidence for living-world acceptance (`docs/0-foundation/02`, lines 77-78), and architecture `13` gives the record's data contract (`docs/1-architecture/13`, lines 2-8).

**Current execution coverage.** `docs/2-execution/02` defines the first-proof baseline, target artifact, and acceptance contract. It enumerates event, truth-firewall, actor-known, no-human ordinary life, missing-property, possession parity, view/debug split, replay, fixture, diagnostic, and Phase-4 entry gates, and names scenario families such as hidden food, routine blocking, strongbox contradiction, possession parity, no-human day, missing property, and wrong suspicion (`docs/2-execution/02`, lines 1-12). It does not explicitly require an observer-only emergence-evidence record as part of the first-proof artifact set. `docs/2-execution/00` and `10` already declare `EMERGE-OBS`, but `02` still reads like the first-proof package can be complete without it.

**Tier-fit verdict.** This is execution-owned because it concerns acceptance artifacts and the first-proof certification package. It does not change the foundation principle or architecture data contract.

**Recommendation.** Add an acceptance-package obligation to `02`: any first-proof acceptance packet that claims living-world acceptance must include the observer-only emergence-evidence artifact produced under `10`, alongside but outside the blocking gate list. `03` should treat this as an artifact dependency of relevant phases, not as a new phase gate. `09` should ensure golden/fixture families can supply semantic support where relevant, while `10` remains the mechanism owner.

---

### E02 — Provenance sufficiency must become a fail-closed proof, not just a field expectation

**Target.** `docs/2-execution/04`, with evidence/reporting support in `10`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** Foundation `04` requires provenance-bearing action-relevant knowledge and rejects raw state, debug views, display strings, fixture intent, LLM prose, and similar shortcuts as provenance (`docs/0-foundation/04`, lines 35-40). Architecture `03` strengthens this into a shared sufficiency rule: missing, empty, dangling, wrong-kind, ambiguous, forbidden, or mode-incompatible provenance must fail closed; institution-known contexts are not exempt (`docs/1-architecture/03`, lines 3-8). The routing memo sent this theme to `04`/`10` as provenance-sufficiency proof, requiring unbacked actor/institution facts to be impossible and path-under-test harness provenance to be real (`reports/foundation-amendment-lower-tier-routing.md`, lines 12-13).

**Current execution coverage.** `docs/2-execution/04` is already strong. It requires sealed actor-known packets, knowledge inputs with holder, proposition/fact, provenance, staleness, and evidence-scope fields, and forbids hidden truth, debug panels, test harness intent, raw IDs, global state, and convenience booleans (`docs/2-execution/04`, lines 1-8). It also has a provenance-minimum table and invalid provenance patterns (`docs/2-execution/04`, lines 6-11). The missing piece is the current architecture's single sufficiency proof as a cross-cutting rule: wrong-kind provenance, ambiguous provenance, dangling event IDs, decorative harness metadata, and institution-known provenance gaps should be named as live negative paths, not merely table entries.

**Tier-fit verdict.** This belongs in execution because execution owns anti-contamination checks, fixtures, and proof obligations. The architecture already owns the data-flow contract; execution must prove it.

**Recommendation.** Extend `04` with a provenance-sufficiency proof subsection. The substance should require that every actor-known and institution-known action-relevant fact has a fact-kind-appropriate provenance route; missing, empty, dangling, wrong-kind, ambiguous, forbidden-source, or harness-fabricated provenance fails closed on the real proposal/validation path. Extend `10` review artifacts so any provenance gate evidence identifies the source event/projection/context and includes at least one live negative or a reason no live negative can exist.

---

### E03 — Memory freshness needs one classifier across actor, no-human, TUI, notebook, and institution surfaces

**Target.** `docs/2-execution/04`, `07`, and `10`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** Architecture `06` now states a single freshness classifier: only current modeled perception/contact/search may be `observed_now`; older information must remain remembered, believed, stale, contradicted, hearsay, record-derived, or unknown, with source event, acquisition time, last verification time, and provenance preserved. The classifier applies to no-human runs, TUI possession, notebooks, and holder-known contexts; possession may not freshen knowledge by re-reading truth (`docs/1-architecture/06`, lines 7-9). The architecture report routed this proof to `04` and `10`, with possession parity and no-human surfaces included (`reports/architecture-tier-alignment-research-report.md`, lines 129-131).

**Current execution coverage.** `docs/2-execution/04` includes staleness/acquired-at fields in sealed actor-known packets and requires stale-belief negative coverage (`docs/2-execution/04`, lines 3-8). `docs/2-execution/07` requires embodied view models to derive from holder-known context plus immediate perceptions and to exclude omniscient truth/debug-only facts (`docs/2-execution/07`, lines 1-4). Those are good pieces, but there is no single named proof that the same freshness classifier applies across no-human, TUI, notebook, actor-known, and future institution-known surfaces, nor a live negative for restamping stale knowledge during replay, possession bind, debug attach, or review extraction.

**Tier-fit verdict.** This is execution proof mechanics. Foundation and architecture already define the epistemic rule.

**Recommendation.** Add to `04` a freshness-classifier proof obligation for actor-known contexts and to `07` a parity obligation for possession/view surfaces. `10` should require review artifacts to show acquisition time, last verified time, source event/provenance, freshness classification, and a live negative where an old fact remains old despite a later replay, debug attach, possession bind, notebook display, or no-human review. The obligation should explicitly prohibit restamping by replay, possession, debug, display, or harness extraction.

---

### E04 — Believed-access proof must use observation-time snapshots and carrier evidence

**Target.** `docs/2-execution/07`, with support in `04` and `10`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** Foundation `06` requires proposal from perceived/known affordance and validation against actual affordance (`docs/0-foundation/06`, lines 25-27). Architecture `10` turns this into an observation-time snapshot contract: embodied view models and possession preflight consume holder-known context plus permitted projection records captured at modeled observation/bind/preflight/perception boundaries; they cannot re-read truth or freshen by live physical state (`docs/1-architecture/10`, lines 3-7). The routed seed calls for wallhack negatives, snapshot proof, embodied carrier census, actor-visible/debug split, and possession parity (`reports/architecture-tier-alignment-research-report.md`, lines 130-131).

**Current execution coverage.** `docs/2-execution/07` already requires epistemic view models to derive from holder-known context plus immediate perceptions, excludes hidden truth and debug-only facts, requires possession parity, and includes negative fixture classes for debug omniscience and view filtering (`docs/2-execution/07`, lines 1-13). The gap is the new architecture's observation-time snapshot idea. Execution should prove the existence and sufficiency of captured carriers at the time the action/view/preflight was formed, not merely that the final rendered view omits hidden facts.

**Tier-fit verdict.** Belongs in execution: it is a proof requirement for view models and possession parity, not a subsystem-contract rewrite.

**Recommendation.** Add to `07` an observation-time snapshot proof obligation. For any actor-visible action, menu option, possession preflight, or embodied view, evidence should show the holder, modeled observation/bind/preflight/perception boundary, captured facts, provenance/freshness of those facts, and the absence of live truth handles in the view-generation path. Add wallhack negatives for true-but-unknown routes, workplaces, sleep affordances, container contents, item locations, and routine opportunities. Add an embodied carrier census to `10` review artifacts so reviewers can see every actor-visible datum's carrier and provenance.

**Residual note.** Whether possession bind itself counts as perception is not an execution decision. Until the owner decides, `07` should not silently assume either policy. Once decided, `07` must prove the policy with the same snapshot and parity discipline.

---

### E05 — Single-owner derived accounting must be proven across scheduler, action pipeline, replay, and goldens

**Target.** `docs/2-execution/05`, `06`, `09`, and `10`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** Architecture `04` and `05` state that actors propose from need pressures, routines, intentions, and holder-known contexts, but the actor decision transaction must not mint need deltas or charge durations directly; derived consequences are owned at the action pipeline/ordinary-life boundary (`docs/1-architecture/04`, lines 0-11; `docs/1-architecture/05`, lines 5-8). The routed seed asks execution to prove no duplicate tick/window charges, no drift, and no byte-stable-but-false goldens (`reports/architecture-tier-alignment-research-report.md`, line 131).

**Current execution coverage.** `docs/2-execution/06` already has strong proof: every need delta must trace to an event, active effects must charge at most once per actor/need/tick, passive changes must be deterministic and bounded, and duration actions must be tick-audited (`docs/2-execution/06`, lines 1-5). It also defines no-human ordinary-life proof and capstone evidence (`docs/2-execution/06`, lines 6-17). `docs/2-execution/05` has proposal/validation/scheduler boundaries and no-direct-dispatch proof (`docs/2-execution/05`, lines 1-13). What is missing is the explicit single-owner seam and review evidence tying proposal, scheduler, duration action, passive window, replay, and golden fixtures together.

**Tier-fit verdict.** Belongs in execution: this is proof of an architecture seam, not a new economy or scheduler design.

**Recommendation.** Add to `06` an explicit single-owner derived-accounting proof. The proof should require evidence that the only layer allowed to emit need/duration deltas is the action-pipeline/ordinary-life boundary; scheduler/routine/planner/projection/replay/golden code must not independently charge or normalize deltas. Add supporting cross-references in `05` for proposal/scheduler non-ownership. Add to `09` and `10` an artifact rule that a golden's byte stability is insufficient if the semantic single-charge ledger drifts, double-counts, or omits a duration terminal.

---

### E06 — `EMERGE-OBS` mechanism must be realigned to `INV-111` and architecture `13`

**Target.** `docs/2-execution/10`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** `INV-111` now makes observer-only emergence evidence a living-world acceptance requirement (`docs/0-foundation/02`, lines 77-78). Foundation `09` says observation is not steering (`docs/0-foundation/09`, lines 19-20), and foundation `12` requires this evidence beside mandatory gates (`docs/0-foundation/12`, lines 23-25). Architecture `13` defines required record fields and invalid-pass conditions (`docs/1-architecture/13`, lines 2-8, 13-17). Spec `0026` left the mechanism to execution `10`, and spec `0027` routed the observer-only data contract proof mechanics to execution (`archive/specs/0027`, lines 16-19, 42-45).

**Current execution coverage.** `docs/2-execution/00` and `10` already do the most important constitutional thing: they declare `EMERGE-OBS` as non-certifying and observer-only. `00` says it is not a certification gate, blocks nothing, and must not become a simulation input or pass/fail threshold without future doctrine (`docs/2-execution/00`, lines 10-14). `10` says `EMERGE-OBS` is a zero-floor, observer-only, non-certifying obligation; evidence must remain outside cognition, scheduler, validators, authoring, seed selection, scenario goals, and outcome gates (`docs/2-execution/10`, lines 7-13). That already closes the “is it a gate?” concern.

The mechanism is stale and partial. `10` says it operationalizes older invariants and “amends no doctrine,” but current foundation now includes `INV-111` and current architecture `13` specifies a record contract (`docs/2-execution/10`, lines 7-10). The current execution description mentions substrate, counters, output rows, replay reviewer, and debug-side projections, but not all required fields: run, seed/randomness provenance, controller mode, phenomenon family, source events/causal-chain references, extraction time, review/projection version, and enough ancestry for replay explanation. It also needs to guard harder against debug- or fixture-fabricated observer rows.

**Tier-fit verdict.** Belongs in execution because the foundation and architecture intentionally stop at principle/data contract; execution owns mechanism, artifact, review, and anti-Goodhart proof.

**Recommendation.** Correct `10`'s upstream references so `EMERGE-OBS` is described as realizing current `INV-111`, foundation `09`/`12`, and architecture `13`. Add the architecture data-contract fields to the evidence artifact. Require that rows be extracted retrospectively from actual run events/projections with event-log ancestry, not fabricated by fixtures/debug panels or inserted by the harness. Require invalid-pass examples: observer evidence feeding cognition/scheduler/validators; evidence used to pick seeds, scenarios, or objectives; evidence lacking replay ancestry; and observer rows based only on debug truth or display text. Preserve current non-certifying semantics: no numeric dramatic-quality pass threshold and no blocking gate without future doctrine.

---

### E07 — Anti-vacuity and typed behavior witnesses must be a general execution rule

**Target.** `docs/2-execution/10`, with support in `04` and `09`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** Architecture `13` says typed observability must show responsible layer, source event/proposal/context IDs, checked facts, behavior-witness fields, accepted/rejected stage, and ancestry; artifact existence, count, checksum, shape, or display text is insufficient (`docs/1-architecture/13`, lines 6-8). The routed seed from the foundation campaign names anti-vacuity: every lock should have a live negative or explicit no-negative reason, artifact presence must pair with behavior witnesses, and pending evidence is not a pass (`reports/foundation-amendment-lower-tier-routing.md`, lines 15-17; `reports/architecture-tier-alignment-research-report.md`, lines 132-133).

**Current execution coverage.** `docs/2-execution/10` already includes diagnostics, property/random tests, CI/mutation posture, pending mutation language, and review templates (`docs/2-execution/10`, lines 0-14, 15-22). It explicitly says pending mutation disposition is not a pass. The gap is generality: the rule appears mainly in mutation and review sections, not as a universal execution evidence standard for truth firewall, provenance, freshness, possession, accounting, emergence observation, and replay artifacts.

**External method support.** Mutation testing frameworks describe mutants as deliberate code changes that tests should kill; surviving mutants indicate tests may not actually cover behavior, which is exactly the anti-vacuity pressure here. Metamorphic testing literature addresses oracle limitations by checking relations among multiple executions rather than single expected outputs. Property-based testing generates broad input ranges and shrinks failures to smaller counterexamples. These methods support an execution doctrine that requires behavior-sensitive witnesses and live negatives, not decorative shape checks.

**Tier-fit verdict.** Belongs in execution because execution owns testing, diagnostics, fixtures, and review artifacts.

**Recommendation.** Add a general anti-vacuity section to `10`. The substance should require that each lock/gate/proof obligation identify a live negative that would fail if the protected shortcut were reintroduced, or state why no live negative can exist. Artifact-presence checks must be paired with behavior witnesses from the path under test. For each behavior witness, review artifacts should identify responsible layer, source event/proposal/context IDs, checked facts, acceptance/rejection stage, and replay/projection ancestry. Add cross-reference hooks so `04` provenance and `09` golden fixture acceptance cannot pass on schema presence, fixture labels, or stable bytes alone.

---

### E08 — Acceptance-evidence and fingerprint honesty need execution-level status rules

**Target.** `docs/2-execution/10`; forward routes to `docs/3-reference/01` and `docs/4-specs/0003`.  
**Verdict.** belongs-in-execution, with route-forward appendices.

**Upstream driver.** Architecture `13` rejects invalid pass conditions where acceptance evidence exists without typed behavior evidence, provenance, or replay ancestry (`docs/1-architecture/13`, lines 13-17). The routing memo's G theme and the architecture report's X10 finding route acceptance-evidence honesty, raw-byte fingerprint scope, pending versus pass, sampled versus certifying evidence, and template/risk-register follow-up to execution `10`, reference `01`, and the acceptance-artifact template (`reports/foundation-amendment-lower-tier-routing.md`, lines 16-20; `reports/architecture-tier-alignment-research-report.md`, lines 132-137). Spec `0027` explicitly keeps evidence-status taxonomy and raw-byte fingerprint-scope honesty out of architecture and routes them to execution/reference/template work (`archive/specs/0027`, lines 42-45).

**Current execution coverage.** `docs/2-execution/10` has diagnostic standards, CI suite/mutation posture, acceptance review template, and review rules. It already says pending mutation dispositions are not passes and that a pass requires evidence from canonical execution commands/review packets (`docs/2-execution/10`, lines 15-22). But it does not yet define a general status/fingerprint/sampling honesty rule. The current review template asks for pass/fail/scoped remediation but does not distinguish observer-only evidence, sampled evidence, pending evidence, raw-byte fingerprint scope, parsed semantic scope, historical archive evidence, or certification evidence.

**External method support.** Approval/golden tests are useful for comparing complex outputs to approved baselines, but they become misleading if a byte-stable output is treated as semantic proof. The test-oracle literature warns that deciding whether observed output is correct is a distinct problem from producing output at all; expensive or incomplete oracles create real uncertainty. Deterministic-simulation testing emphasizes reproducibility via controlled nondeterminism and seeds, which supports honest replay/fingerprint scope but does not itself certify semantics.

**Tier-fit verdict.** The status/fingerprint/sampling rule belongs in execution because it governs review artifacts and acceptance evidence. Risk-register wording belongs in reference, and template wording belongs in `4-specs/0003`.

**Recommendation.** Add to `10` an evidence-honesty section. The substance should require every review packet to label evidence by status and scope: pass/fail where actually certified; pending where not yet proven; sampled where evidence is representative but not exhaustive; observer-only where evidence cannot certify behavior; historical where archive/spec evidence is context but not current certification. Fingerprints must state what was fingerprinted: raw bytes, normalized serialization, parsed semantic content, command transcript, run seed, or replay artifact. A fingerprint must not be cited as proof beyond its scope. Pending, sampled, observer-only, or archive evidence must never be silently counted as a pass.

**Forward routes.** `docs/3-reference/01` should receive the risk-cluster wiring. `docs/4-specs/0003` should later receive template fields for status and fingerprint scope once execution doctrine is accepted.

---

### E09 — Institution-known provenance should be future-proofed before Phase 4 expands

**Target.** `docs/2-execution/11`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** Foundation `07` requires institutions, households, records, and artifacts to be fallible social machinery rather than omniscient truth channels. Current architecture `03` says institution-known contexts are not exempt from provenance sufficiency (`docs/1-architecture/03`, lines 3-8), while architecture `06` requires provenance and freshness for beliefs/traces/records (`docs/1-architecture/06`, lines 7-9). The foundation amendment report carried institution-known provenance as a residual open question, and the architecture report routed it to execution `11` (`reports/foundation-amendment-research-report.md`, lines 80-88; `reports/architecture-tier-alignment-research-report.md`, lines 134-135).

**Current execution coverage.** `docs/2-execution/11` defines Phase-4 entry, institution-known context, record surfaces, wrong-suspicion lock, and gates for social/institutional behavior (`docs/2-execution/11`, lines 1-10). It is already pointed in the right direction. The gap is that its dependency and proof language does not explicitly import the newly shared provenance-sufficiency/freshness rules from architecture `03` and `06`, nor does it require live negatives for record truth-conversion, stale institutional records, dangling record provenance, or institution reactions to raw event-log truth.

**Tier-fit verdict.** Belongs in execution because `11` owns Phase-4 entry proof and institutional records/wrong-suspicion fixtures. It does not decide new institution mechanics.

**Recommendation.** Add a Phase-4 provenance future-proofing obligation to `11`. Any institution-known fact, record-derived belief, norm-triggered procedure, wrong-suspicion inference, or artifact interpretation should prove provenance sufficiency and freshness using the same fail-closed mechanics as `04`/`10`. Negative fixtures should include institution reacts to truth without record, stale record treated as fresh, dangling record provenance, wrong-kind provenance, and record display text mistaken for provenance. The recommendation should not expand Phase-4 scope; it only locks the proof contract before Phase-4 entry.

---

### E10 — Execution research notes should add proof-method sources and forbidden misreads

**Target.** `docs/2-execution/13`.  
**Verdict.** belongs-in-execution.

**Upstream driver.** Execution owns proof methodology. The current pass is specifically about how proof obligations should be framed: mutation/metamorphic/property-based testing, golden/approval and deterministic-simulation testing, structured observability, and test-oracle honesty. The predecessor surveys covered epistemics, event-sourcing, object-capability/information-flow, fog-of-war, and emergent narrative; this pass adds execution-proof methodology.

**Current execution coverage.** `docs/2-execution/13` already records decisions for event-sourced replay, provenance/data lineage, deterministic simulation, property-based testing, BDI/affordance/social-simulation/institution precedents, LLM boundary, and TUI/game precedents (`docs/2-execution/13`, lines 0-18). It does not yet record mutation testing as behavior sensitivity, metamorphic testing as relation-based oracle support, approval/golden testing as useful but semantically limited, structured observability as typed evidence, or the test-oracle problem as the reason evidence status must be honest.

**Tier-fit verdict.** Belongs in execution because this is execution-layer research/source-note hygiene; it does not create product doctrine or implementation design.

**Recommendation.** Add a source-notes section to `13` for proof methodology. It should cite mutation testing, metamorphic testing, property-based testing including shrinking, approval/golden testing, deterministic-simulation testing, OpenTelemetry-style structured observability, and test-oracle literature. It should also add forbidden misreads: mutation coverage is not certification by itself; a surviving mutant is not harmless without disposition; a golden's byte stability is not semantic truth; deterministic replay is reproducibility, not correctness; structured log existence is not behavior evidence; and observer-only emergence evidence is not a pass/fail gate.

---

### C01 — `EMERGE-OBS` is already declared as non-certifying and observer-only

**Target.** `docs/2-execution/00` and `10`.  
**Verdict.** already-owned-close.

**Evidence.** `docs/2-execution/00` already identifies `EMERGE-OBS` as the canonical execution observation obligation, says it is not a certification gate, blocks nothing, and cannot become a simulation input or pass/fail threshold without future doctrine (`docs/2-execution/00`, lines 10-14). `docs/2-execution/10` reinforces that it is zero-floor, observer-only, non-certifying, and excluded from cognition, scheduler, validators, authoring, seed selection, scenario goals, and outcome gates (`docs/2-execution/10`, lines 7-13).

**Close rationale.** The routed concern that execution might mistakenly turn emergence observation into a gate is already handled. The mechanism still needs E06, but the gate/observation boundary is already owned.

---

### C02 — Archived-spec status and P0 certification posture are already correct

**Target.** `docs/2-execution/01`.  
**Verdict.** already-owned-close.

**Evidence.** `docs/2-execution/01` says the post-`0008` archive is historical evidence, not live certification, and that later certification must re-audit the live code/docs rather than rely on archived specs (`docs/2-execution/01`, lines 0-8). It also lists P0 proof obligations for workspace boundary, deterministic replay, action authority, debug quarantine, fixture coverage, diagnostic surfaces, and acceptance artifacts (`docs/2-execution/01`, lines 10-16).

**Close rationale.** This matches the authority and archive posture required by current `docs/README.md` and the prompt's anti-stale-manifest discipline. No execution change is recommended.

---

### C03 — Data authoring and schema provenance are already execution-owned

**Target.** `docs/2-execution/08`.  
**Verdict.** already-owned-close.

**Evidence.** `docs/2-execution/08` requires seed/schema provenance, forbids authored outcome-chain fields, and rejects omniscient institutional truth, hidden actor knowledge, outcome flags, and direct scheduler/plan overrides. It defines provenance-bearing content requirements and content-validation gate evidence (`docs/2-execution/08`, lines 1-11).

**Close rationale.** Current upstream does not require a new execution doctrine change here. The broader provenance sufficiency work in E02 should cross-reference authoring evidence if needed, but `08` already owns the content-authoring anti-contamination proof at the right tier.

---

### C04 — Deferred second-proof/story-sifting boundary remains correctly locked

**Target.** `docs/2-execution/12`.  
**Verdict.** already-owned-close.

**Evidence.** `docs/2-execution/12` keeps notices, travel, regional scale, LOD, and story-sifting as deferred second-proof work. It states that story-sifting may cluster/summarize/highlight/explain already-emitted events but may not cause events, repair plots, inject leads, reveal hidden truth, or prioritize dramatic storylines (`docs/2-execution/12`, lines 0-8).

**Close rationale.** This remains aligned with foundation no-director/no-scripting doctrine and architecture's observer-only story-sifting boundary. No execution-tier amendment is needed beyond preserving the lock while E06 defines observer-only emergence records in `10`.

---

### F01 — Emergence-evidence terminology belongs in reference, not execution

**Target.** `docs/3-reference/02_GLOSSARY.md`.  
**Verdict.** route-forward.

**Basis.** The glossary is the prescriptive terminology-control layer. The routing memo and spec `0026` identify the emergence-evidence term as a later reference-tier task, not something execution should silently coin. Execution may cite the existing `EMERGE-OBS` obligation, but it should not invent a new canonical term.

**Hand-off.** The reference session should add a canonical term for the observer-only emergence-evidence artifact, including non-certifying, retrospective, event-log-ancestry-backed, non-steering, and structurally non-input properties. It should cross-reference foundation `INV-111`, foundation `09`/`12`, architecture `13`, and execution `10` after realignment.

---

### F02 — Acceptance-evidence honesty risks belong in the risk register

**Target.** `docs/3-reference/01_DESIGN_RISK_REGISTER.md`.  
**Verdict.** route-forward.

**Basis.** The design-risk register already tracks execution-relevant failure modes such as exact-commit drift, provenance collapse, epistemic leakage, no-human ordinary-life failure, and acceptance-evidence honesty. But risk-register entries are watch risks, not execution gate prose. The execution tier should define the proof standard in `10`; reference should preserve the cross-cutting risk language.

**Hand-off.** Add or realign the R-27/R-28/R-29 cluster so it tracks: pending evidence mislabeled as pass; raw-byte fingerprint treated as semantic proof; sampled/observer-only evidence treated as certifying; artifact presence without behavior witness; and archive/history treated as current certification. Link those risks to execution `10` once amended.

---

### F03 — Acceptance-artifact template changes belong in `docs/4-specs/0003`

**Target.** `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.  
**Verdict.** route-forward.

**Basis.** Execution `10` should own doctrine for evidence status and fingerprint honesty. The concrete acceptance-artifact template is a specs-layer artifact. The current template already has structured acceptance sections, but it does not encode the full status/fingerprint/sampling distinctions implied by the current execution recommendation.

**Hand-off.** After `10` is amended, update the template to require fields for evidence status, evidence scope, fingerprint scope, sampled-vs-certifying scope, pending evidence, observer-only evidence, and path-under-test behavior witnesses. Do not update the template before execution doctrine establishes the rule.

---

### F04 — Possession-bind perception is an owner decision before execution can prove it

**Target.** owner decision, then `docs/2-execution/07`.  
**Verdict.** route-forward to owner decision.

**Basis.** The current architecture requires observation-time snapshots and says possession is not a knowledge upgrade, but the campaign preserved a narrower unresolved question: does possession bind itself count as modeled perception? Execution cannot decide that product/architecture policy without overstepping.

**Hand-off.** Surface the owner decision explicitly. Once decided, amend `07` so its possession proof proves the chosen policy: either bind creates a bounded modeled perception snapshot with provenance/freshness, or bind creates no perception and must not freshen any view/action preflight.

---

## 6. Sweep notes for all execution documents

This section records the 14-document sweep so later work can see which docs were checked and where no first-class change was needed.

- `00_EXECUTION_INDEX_AND_AUTHORITY.md`: aligned on tier authority and `EMERGE-OBS` non-gate semantics; needs E00 vocabulary reconciliation.
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`: aligned; closed by C02.
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`: partial; needs E01 so first-proof acceptance carries observer-only emergence evidence.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`: mostly aligned; include E00 vocabulary cleanup and E01 artifact cross-reference without turning `EMERGE-OBS` into a gate.
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`: strong but partial; needs E02 provenance sufficiency and E03 freshness classifier hooks.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`: strong but partial; needs E05 single-owner accounting seam cross-reference.
- `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`: strong but partial; needs E05 seam evidence and drift/golden honesty.
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`: strong but partial; needs E03/E04 snapshot/freshness proof and carries F04.
- `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`: aligned; closed by C03.
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`: mostly aligned; needs E05/E07/E08 support so golden fixtures prove semantic obligations and live negatives, not just stable output.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`: primary amendment target; needs E06, E07, E08, and external-method E10 support.
- `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md`: partial; needs E09 provenance/freshness future-proofing.
- `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`: aligned; closed by C04.
- `13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md`: partial; needs E10 proof-method source notes.

---

## 7. Forward-routing appendix

| Route | Target tier/doc | Reason | Handoff substance |
|---|---|---|---|
| F01 | `docs/3-reference/02_GLOSSARY.md` | Terminology control belongs to reference. | Add the canonical term for observer-only emergence-evidence artifact; define retrospective, non-certifying, event-log-ancestry, non-steering, non-input properties; cross-reference foundation/architecture/execution after `10` realignment. |
| F02 | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Risk-watch taxonomy belongs to reference. | Realign acceptance-evidence honesty risks around pending≠pass, sampled≠certifying, observer-only≠gate, byte fingerprint≠semantic proof, archive history≠certification, artifact presence≠behavior witness. |
| F03 | `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | Concrete acceptance template belongs to specs/reference workflow, after execution doctrine. | Add template fields for evidence status, scope, fingerprint scope, sampling scope, observer-only/non-certifying flags, pending evidence, path-under-test behavior witness, and replay/provenance ancestry. |
| F04 | Owner decision, then `docs/2-execution/07` | Execution cannot decide whether possession bind is perception. | Owner decides policy; execution later proves no freshening or proves bounded bind snapshot with provenance/freshness. |

No foundation or architecture amendment is recommended in this report.

---

## 8. Residual open questions

1. **`EMERGE-OBS` thresholds, ratchets, and anti-Goodhart policy.** Execution should decide now that the current doctrine remains observer-only, non-certifying, and non-blocking. A zero-floor or “dead-world detector” ratchet may be recorded as an observation/reporting policy only, not as a dramatic-quality pass threshold. Any future numeric threshold or certification use requires a dedicated future spec. This closes the immediate conformance gap without Goodharting the simulation toward the observer record.

2. **Institution-known provenance.** This is an execution-doctrine decision for `11`, not an owner policy gap. `11` should inherit the same provenance sufficiency and freshness proof mechanics as `04`/`10` before Phase 4 expands institution behavior. The exact Phase-4 implementation remains deferred.

3. **Possession-bind perception.** This remains an owner/product-architecture decision. Execution should not decide it. The right execution action is to keep `07` neutral and ready to prove either chosen policy.

4. **Vocabulary reconciliation.** A later reassess session should decide whether existing phase labels such as `EPI-CERT`, `ORD-LIFE-CERT`, and `DIAG-CERT` are formal certification artifact labels, informal names, or should be mapped under the canonical gate names in `00`. This is an execution editorial/doctrine hygiene question, not a new product doctrine.

5. **Code certification.** This report does not assert that code, fixtures, CI, or goldens already satisfy the amended execution doctrine. `P0-CERT`/baseline certification remains later work after the execution tier is realigned.

---

## 9. External references used for execution-proof methodology

These references support the recommendations in E07, E08, and E10. They are not Tracewake authority. They were fetched as external research on 2026-06-13.

1. **Stryker Mutator, “What is mutation testing?”** `https://stryker-mutator.io/docs/` — explains that mutation testing introduces changes to production code, tests are expected to fail against those changes, surviving mutants indicate insufficient behavioral coverage, and code coverage alone can miss assertion-vacuity problems. Used to justify live negatives and mutation-perimeter honesty.
2. **Sergio Segura, Gordon Fraser, Ana B. Sanchez, and Antonio Ruiz-Cortés, “A Survey on Metamorphic Testing,” IEEE Transactions on Software Engineering 42(9), 2016.** `https://eprints.whiterose.ac.uk/id/eprint/110335/1/segura16-tse.pdf` — frames metamorphic testing as an answer to the oracle problem by checking relations across transformed executions rather than only single expected outputs. Used to justify relation-based evidence for simulation behavior and observer-only artifacts.
3. **Mark Harman, Phil McMinn, Muzammil Shahbaz, and Shin Yoo / Barr et al., “The Oracle Problem in Software Testing: A Survey,” UCL Discovery.** `https://discovery.ucl.ac.uk/id/eprint/1471263/` — identifies the test-oracle problem as the problem of determining whether observed test behavior is correct. Used to justify evidence-status honesty, sampled evidence labels, and pending-not-pass doctrine.
4. **Hypothesis documentation.** `https://hypothesis.readthedocs.io/en/latest/` — describes property-based testing as generating many inputs, including edge cases, for properties expected to hold broadly. Used to justify invariant-oriented execution proofs.
5. **ZIO Test, “Shrinking.”** `https://zio.dev/reference/test/property-testing/shrinking/` — describes shrinking as reducing property-test counterexamples to simpler failures. Used to justify review artifacts that preserve minimal behavior witnesses.
6. **ApprovalTests.cpp documentation.** `https://approvaltestscpp.readthedocs.io/en/latest/` — describes approval/golden/snapshot-style comparison of complex outputs against approved baselines. Used with the caution that stable bytes are not semantic proof.
7. **Antithesis, “Deterministic simulation testing — how it works and when to use it.”** `https://antithesis.com/docs/resources/deterministic_simulation_testing/` — describes deterministic simulation as controlling nondeterminism such as clocks, randomness, and scheduling to reproduce complex bugs. Used to support reproducible replay/fingerprint scope without confusing reproducibility with correctness.
8. **OpenTelemetry Logs API.** `https://opentelemetry.io/docs/specs/otel/logs/` — describes logs as structured events and supports semantic conventions. Used to justify structured diagnostic witnesses rather than display-string evidence.
9. **OpenTelemetry Logs Data Model.** `https://opentelemetry.io/docs/specs/otel/logs/data-model/` — defines a stable log data model for preserving event semantics across systems. Used to justify typed behavior-witness fields.
10. **OpenTelemetry semantic conventions for events.** `https://opentelemetry.io/docs/specs/semconv/general/events/` — describes events as distinct occurrences with names, timestamps, and attributes. Used to justify event/proposal/context IDs and typed attributes in observability artifacts.

---

## 10. Self-check

- Every Tracewake file used for doctrine was fetched from the exact `64a8367ca54f5daf97dac7031a708476d31a3707` raw URL listed in §2.
- No branch, default-branch lookup, repository metadata, clone, code search, or connector namespace label was used as evidence.
- The upstream delta explicitly covers `INV-111`, foundation `14`, foundation `09`/`12`, and the current architecture contracts for observer-only emergence evidence, provenance sufficiency, freshness, snapshots, single accounting, and typed observability.
- Every seeded item A-G plus the emergence mechanism and `0026`/`0027` hand-offs was validated as closed, partial, or routed.
- All 14 execution docs were swept.
- Every finding has a tier-fit verdict.
- Recommendations stay at execution proof level: gates, certification artifacts, fixtures, diagnostics, observability, and review artifacts. No product identity, architecture contract, module layout, storage/serialization, or algorithm choice is re-decided.
- No new gate code, observation-obligation code, or invariant number is invented. Existing names are cited only to locate doctrine.
- `EMERGE-OBS` remains observer-only and non-certifying.
- Forward routes are limited to glossary, risk register, acceptance-artifact template, and the owner decision for possession-bind perception.
