# Foundation amendment audit and external prior-art survey — post-0006…0025 hardening campaign

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `f7adc0149963ea4a90b58ad05f633fd6e71e8649` (`f7adc01`)  
**Intended destination:** `reports/foundation-amendment-research-report.md`

> I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

## 1. Method and provenance ledger

### 1.1 Mandatory exact-commit ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: f7adc0149963ea4a90b58ad05f633fd6e71e8649
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.open against exact raw.githubusercontent.com URLs; selected paths were first checked against the uploaded manifest path inventory
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0006_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0007_PHASE_3A_SECOND_HARDENING_NO_HUMAN_ORDINARY_LIFE_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0008_PHASE_3A_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0009_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0010_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0011_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0012_PHASE_1_DOC_CODE_ALIGNMENT_CONFORMANCE_AND_ANTI_DRIFT_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0013_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0015_PHASE_3A_EVENTED_COGNITION_CHANNELS_AUDIT_ENFORCEMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0016_PHASE_3A_NEED_ACCOUNTING_REPLAY_EVIDENCE_AUDIT_COVERAGE_AND_LOCK_DURABILITY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0022_PHASE_3A_BASELINE_TRIAGE_INTEGRITY_EMBODIED_DEBUG_QUARANTINE_ADVERSARIAL_GATE_RESTORATION_AND_LOCK_BEHAVIORAL_PROOF_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0024_PHASE_3A_CONTENT_SCHEMA_VERSION_GATE_META_WITNESS_RESIDUE_MUTATION_PERIMETER_DERIVATION_AND_TUI_TIME_DEBUG_QUARANTINE_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/archive/specs/0025_PHASE_3A_META_WITNESS_EXECUTION_PROOF_PERCEPTION_KILL_SET_PROVENANCE_ENVELOPE_READ_PATH_FAIL_CLOSED_AND_MANIFEST_FINGERPRINT_HONESTY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/reports/phase2a-epistemic-substrate-hardening-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/reports/phase3a-ordinary-life-needs-routines-hardening-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/reports/0020_ord_life_cert_scoped_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/reports/0025_ord_life_cert_scoped_acceptance.md
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

The uploaded manifest was used only to verify that each selected repository path was present in the user-supplied tree inventory. It was not used as source text, as a latest-main proof, or as repository metadata. Every repository source named above was requested mechanically as:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/<manifest path>
```

No fetched result, visible URL, or citation used for Tracewake source text pointed to any repository other than `joeloverbeck/tracewake`.

### 1.2 Citation convention inside this report

Tracewake exact-commit sources are cited by short labels that map to the fetched URLs in the ledger:

- `TW-README` = `docs/README.md`.
- `TW-F-00`…`TW-F-14` = the foundation files `docs/0-foundation/00...14`.
- `TW-A-00`, `TW-A-03`, `TW-A-06`, `TW-A-13` = the fetched architecture files.
- `TW-X-00`, `TW-X-03`, `TW-X-04`, `TW-X-10` = the fetched execution files.
- `TW-RISK`, `TW-GLOSS`, `TW-SPEC-LEDGER` = fetched reference/spec-ledger files.
- `TW-SPEC-0006`…`TW-SPEC-0025` = exact-commit archived hardening specs.
- `TW-P2A-BRIEF`, `TW-P3A-BRIEF`, `TW-ACC-0020`, `TW-ACC-0025` = fetched reports.

External prior art is cited with bracketed reference keys, fully listed in §11. The report distinguishes three provenance classes throughout:

1. **Tracewake source doctrine/evidence** — fetched exact-commit repository files, cited by `TW-*` labels.
2. **External research / comparable systems** — papers, books, standards, talks, and documentation cited by reference keys.
3. **Recommendation / inference** — explicitly marked as this report’s judgment, never as already-ratified Tracewake doctrine.

### 1.3 Tier-fit rule applied

The decisive tier rule comes from `TW-README` and `TW-F-00`: foundation may define Tracewake’s identity, invariants, scope, acceptance meaning, and truth-firewall doctrine; it may not define crate layout, module layout, storage engine, serialization format, planning algorithm, fixture shape, gate mechanics, or test procedure. Architecture owns subsystem authority and data-flow contracts. Execution owns gates, certification sequence, fixtures, acceptance artifacts, diagnostics, and proof obligations. Reference owns glossary and design-risk memory. Archive is history, not live authority. `TW-SPEC-LEDGER` repeats that archived specs are historical evidence and do not amend doctrine.

That rule drives the hard line used below: a candidate becomes **Bucket 1** only when the campaign exposed a missing what-level statement about what Tracewake is. If the lesson is a classifier, hash, witness, mutation job, table, negative fixture, gate, or proof path, it is **Bucket 2** even if it is important.

---

## 2. Verdict summary

| Candidate theme | Verdict | One-line basis |
|---|---:|---|
| Provenance sufficiency — “no fact without a modeled source” | **No-hole**; lower-tier machinery remains | Foundation already states the what-level doctrine: action-relevant holder-known cognition requires modeled provenance and fails closed when provenance is absent or insufficient. The non-empty source-event-id newtype, witness tables, and taint proofs belong below foundation. |
| Memory freshness / staleness epistemics | **No-hole** | Foundation already separates authoritative truth, actor-known truth, belief, memory, and stale belief; it explicitly preserves stale beliefs as meaningful, provenance-bearing cognition inputs rather than deleting or restamping them as current truth. |
| Believed-access-vs-truth in affordance generation | **No-hole** | The 2026 truth-firewall spine already says hidden truth must not generate embodied affordance menus, view models, or no-human cognition; affordance generation from sealed holder-known context is already constitutional doctrine. |
| Single-charge accounting authority for derived quantities | **Bucket 2 → architecture / execution** | The campaign exposed a real replay/accounting law, but “which classifier/keying authority charges each tick exactly once” is subsystem authority and proof machinery, not product identity. Foundation’s causal/replay doctrine is sufficient. |
| Emergence-as-evidence / EMERGE-OBS | **Bucket 1 → foundation hole** | Foundation bans scripts and celebrates retrospective story, but it does not yet say that the living-world thesis must be empirically evidenced by observer-only records of unscripted phenomena that actually arise. This belongs in foundation as a what-level acceptance principle, not as a counter schema or gate procedure. |
| Falsifiability / behavioral-proof / anti-vacuity | **Bucket 2 → execution / reference** | The philosophical kernel is important, but the campaign’s concrete lesson is about executable witnesses, mutation perimeters, negative cases, and non-vacuous guards. Those are execution/reference controls. Foundation already has enough acceptance harshness without absorbing test mechanics. |
| Acceptance-evidence honesty / manifest-fingerprint honesty | **Bucket 2 → execution / reference** | This is a cross-cutting hand-off extracted from 0024–0025 rather than a new foundation amendment: artifacts must prove the path under test and cannot overstate pending/passing status. Correct home is execution `10` plus design-risk tracking. |

**Bottom line:** recommend **one foundation-tier amendment package**: an emergence-evidence principle housed primarily in `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`, with a companion invariant-substance addition in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` and a small cross-reference in `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`. Do **not** ratify wording or assign an invariant number here. Everything else is either already covered by the post-0025 foundation or should be routed to later architecture/execution/reference sessions.

---

## 3. Candidate A — Provenance sufficiency: “no fact without a modeled source”

### 3.1 Campaign lesson

The hardening campaign repeatedly caught provenance-shaped facts that could be minted without modeled source evidence. The spec ledger records the progression: 0015 moved cognition toward event-backed source-event enforcement; 0016 found residual unbacked-fact channels and a tautological replay-context hash; 0018 introduced provenance-witness fail-closed behavior; 0021 caught fabricated provenance inside a gate harness; 0025 records provenance-true kill-set work and provenance-envelope read-path fail-closed posture (`TW-SPEC-LEDGER`, `TW-SPEC-0015`, `TW-SPEC-0016`, `TW-SPEC-0018`, `TW-SPEC-0021`, `TW-SPEC-0025`).

The most concrete campaign pressure is 0016’s finding that public helpers could mint `remembered_belief` or `modeled_observation` labels with empty source-event vectors, and that one audit path verified only that cited strings existed rather than that the context was re-derived from the replayed log (`TW-SPEC-0016`). 0018 then turned this family into a fail-closed provenance-witness concern: a fact whose source cannot be shown through a modeled chain must not be accepted by holder-known cognition surfaces (`TW-SPEC-0018`). 0025 sharpened the same lesson by requiring provenance-true taint rather than raw token scans and by making envelope read paths reject unsupported or ambiguous materialization loudly (`TW-SPEC-0025`).

Stripped to what-terms, the lesson is: **Tracewake cannot allow actor-known or institution-known cognition to contain a meaningful fact that has no modeled acquisition route. A label saying “observed” or “remembered” is not itself evidence.**

### 3.2 Tier-fit verdict

**Verdict: No-hole for foundation; lower-tier machinery remains Bucket 2.**

The foundation is already explicit enough after doc-14 promotion. `TW-F-04` says claims, beliefs, observations, memories, and records are distinct epistemic objects; belief shape includes source, acquisition time, last verification/stale status, and provenance chain; acceptable provenance includes modeled observation, testimony, record, memory of a prior observation, or authored prehistory; unacceptable provenance includes raw truth lookup and prose-born facts. `TW-F-14` is even more direct: only provenance-bearing holder-known context may feed cognition; string labels and booleans are insufficient; unproven raw physical truth is forbidden. `TW-F-02` re-derives this in invariant form: `INV-024` requires important beliefs to have provenance; `INV-102` requires actor-known cognition inputs, observations, beliefs, memories, actor-known affordances, and diagnostics to carry source/provenance sufficient for replay/debug, with action-relevant missing provenance rejected.

That is a foundation-level rule. The campaign’s newtype, non-empty vectors, witness-compatibility tables, provenance taint keys, harness shape, and kill sets are not foundation rules. They are the mechanisms by which architecture/execution ensure the already-existing doctrine cannot be faked. `TW-A-03` already owns holder-known context packets and provenance minimums; `TW-X-04` operationalizes the truth-firewall and anti-contamination gates; `TW-X-10` owns testing and review artifacts. `TW-RISK` tracks R-02 provenance collapse as a relapse mode, but here the risk-register entry need not be promoted because the promoted doctrine already exists in `TW-F-04`, `TW-F-14`, and `INV-102`.

### 3.3 External prior-art survey

Truth-maintenance systems are the cleanest precedent for the doctrine but not for a new foundation hole. Doyle’s TMS computes current beliefs from reasons and revises by tracing consequences of changes in those reasons; de Kleer’s ATMS generalizes this by carrying environments/assumption sets under which propositions hold [DOYLE-1979; DE-KLEER-1986]. The relevant constitutional idea is not a Rust witness table; it is the top-level claim that a belief-like proposition is meaningful only as part of a dependency network of reasons. Tracewake already has that idea in holder-known provenance doctrine.

Justification logic gives the same lesson in logical vocabulary: the usual modal “agent knows/believes F” is refined into explicit justification terms, conventionally read as “t is a justification for F” [ARTEMOV-SEP; ARTEMOV-2008]. Tracewake does not need to import justification logic as a formalism. The useful doctrine is that epistemic status should not be an opaque modal label when the product promise is forensic explanation; the source must be representable and inspectable.

Data provenance and lineage standards sharpen the vocabulary. W3C PROV models provenance through entities, activities, and agents involved in producing data; it is meant to support assessment of quality, reliability, or trustworthiness [W3C-PROV-DM; W3C-PROV-O]. Database provenance distinguishes where a tuple came from and why/how a query result exists, so derived facts inherit traceable derivation structure rather than merely appearing in a view [BUNEMAN-2001; GREEN-2007; SIMMHAN-2005]. Again, the constitutional rule is “derived epistemic facts require lineage”; the exact algebra or serialization belongs below foundation.

### 3.4 Recommendation

No foundation amendment. Keep the lower-tier hand-off: later architecture/execution sessions should ensure all holder-known and institution-known constructors, projections, test harnesses, and read paths have a single provenance-sufficiency rule, a fail-closed path for insufficient provenance, and behavior-proving negatives. Do not broaden the foundation to “every fact in every internal table must have source-event IDs”; that would incorrectly encode implementation machinery and would overstate the intended scope. The foundation should remain what-level: action-relevant holder-known cognition must be source-backed and replay-explainable.

---

## 4. Candidate B — Memory freshness / staleness epistemics

### 4.1 Campaign lesson

The campaign had to distinguish currently perceived facts from remembered facts without deleting the remembered fact or restamping it as current truth. `TW-SPEC-0017` and `TW-SPEC-0020` are the key evidence: the spec ledger ties them to projection freshness, actor-known staleness, remembered-food freshness, and supersession parity. `TW-ACC-0020` records that remembered food remains eligible when the actor is away from the source, while current observation is preferred when available, and that stale/current classifications remain per-kind and provenance-backed rather than deleted from cognition surfaces.

The what-level lesson: **a remembered fact remains available for cognition, but as memory: it has acquisition time, provenance, and degraded freshness. Ground truth must not silently update the holder’s memory, and memory must not be treated as currently perceived merely because it is still useful.**

### 4.2 Tier-fit verdict

**Verdict: No-hole.**

This is already foundation doctrine. `TW-F-04` explicitly separates authoritative truth, actor-known truth, belief, memory, stale belief, rumor/testimony, and institutional record. It says memory is fallible and incomplete; stale beliefs are meaningful; ground truth does not automatically repair a mind, record, or rumor network; and actor-known cognition may use only provenance-bearing facts. `TW-F-02` re-derives this in `INV-026` and `INV-028`: memory can be incomplete, stale, contested, or wrong, and stale beliefs remain causally meaningful until corrected by modeled channels. `TW-F-03` adds that replay must preserve the distinction between “what was true” and “what was known.”

Architecture already owns the sharper classifier. `TW-A-03` states that facts observed in the current projection may be classified as current, while older usable knowledge remains remembered belief with its original source/provenance and may not be restamped as current observation. `TW-A-06` owns the architecture of observations, beliefs, memory traces, and information flow. That is exactly where the campaign’s single freshness classifier and parity machinery belong.

### 4.3 External prior-art survey

Memory research supports the foundation’s stance without requiring a new amendment. Ebbinghaus’s classic forgetting work and later recency/frequency models show that memory availability and reliability are temporally conditioned, not equivalent to fresh perception [EBBINGHAUS-1885; ANDERSON-SCHOOLER-1991]. Baddeley and Hitch’s working-memory model reinforces the architectural distinction between currently active working memory and other stored memory structures [BADDELEY-HITCH-1974]. These are not claims that Tracewake needs human-realistic forgetting curves in foundation; they support the doctrine that memory has a time/freshness seam.

Game AI offers a more directly comparable pattern. Orkin’s F.E.A.R. GOAP work uses sensors and shared working memory so actors can record knowledge such as blocked doors and replan from that working memory rather than reading world truth every frame [ORKIN-FEAR]. This maps closely to Tracewake’s holder-known context: perception creates a cognitive surface, and later planning consumes that surface. The constitutional point is already in `TW-F-04` and `TW-F-14`; the classifier lives in architecture.

### 4.4 Recommendation

No foundation amendment. Route any remaining precision work to architecture `03` and `06`, and proof obligations to execution `04` and `10`: one classifier, no restamping of old facts as current observation, and parity across no-human and embodied surfaces. The glossary already has `memory`, `stale information`, `holder-known`, `actor-known`, and `source-backed belief`; no new term is required.

---

## 5. Candidate C — Believed-access-vs-truth in affordance generation

### 5.1 Campaign lesson

The ordinary-life hardening series repeatedly found that embodied menus and no-human proposals could drift toward live truth rather than holder-known context. Early 0006–0008 work exposed no-human and anti-contamination gaps: planner inputs could be string-preconditions, hidden-truth hygiene was tested but not structurally constructed, and no-human ordinary-life proof could overclaim (`TW-SPEC-0006`, `TW-SPEC-0007`, `TW-SPEC-0008`). Later specs forced embodied workplace availability, local exits, perceived blockers, carrier capture, and observation-time snapshots to be computed from what the actor perceived, knew, or remembered, not from live physical truth (`TW-SPEC-0017`, `TW-SPEC-0020`, `TW-SPEC-0023`, `TW-SPEC-0025`).

The sharpest 0025 lesson is observation-time capture: embodied projection should render observed labels/attributes as captured at observation time, not as silently updated live truth (`TW-ACC-0025`).

What-level lesson: **the actions an embodied actor may see or attempt are a function of holder-known conditions. Truth may reject a proposal at validation time, but truth must not whisper extra possibilities into the affordance menu.**

### 5.2 Tier-fit verdict

**Verdict: No-hole.**

This is the exact doctrine that doc 14 promoted. `TW-F-14` says hidden truth must not generate the embodied affordance menu, no-human proposal list, or actor decision trace; truth may validate actions but may not plan them. `TW-F-06` already distinguishes perceived/current affordance from actual validation: affordances are conditional and actor-knowledge-bounded; validators may use authoritative state, but proposal generation must not. `TW-F-08` states that embodied mode shows actor-known reality and may list actor-possible actions but must not list hidden-truth actions, objectives, or facts. `TW-F-02` includes the relevant invariants: `INV-044` on conditional affordances, `INV-067` on embodied actor-known reality, `INV-099`–`INV-110` on truth-firewall/cognition authority, especially `INV-101`, `INV-102`, and `INV-109`.

Architecture/execution also own the proof path. `TW-A-03` defines holder-known context and embodied affordance generation from that context; `TW-X-04` operationalizes TFW gates and negative fixtures. `TW-RISK` R-09 tracks epistemic leakage. The campaign proves these rules need robust lower-tier locks, not that foundation is silent.

### 5.3 External prior-art survey

Partial-observability doctrine in game theory and AI is the broader analogue. Kuhn’s information sets formalize that a player’s available choices are conditioned by what they can distinguish, not by omniscient state labels [KUHN-1953]. In AI planning under partial observability, action selection is mediated by belief state or memory of observations, with hidden state handled by uncertainty rather than by omniscient access [RUSSELL-NORVIG]. Game AI architectures like F.E.A.R. implement the practical version: sensing populates working memory, and planning consults that memory [ORKIN-FEAR].

Networked games provide a useful but imperfect metaphor. Server-authoritative models let a client propose inputs while the authoritative simulation validates the result; the client’s predicted view is not a license to decide truth [UNITY-NETCODE]. Tracewake’s split is stricter because the actor is diegetic: the menu itself is part of the product’s epistemic surface. The prior-art lesson is exactly Tracewake’s existing doctrine: proposal surfaces may be subjective; validators may be authoritative; the two must not collapse.

### 5.4 Recommendation

No foundation amendment. Later tier sessions should continue routing snapshot-at-observation, carrier census, and embodied affordance proof work to architecture `03` / `10` and execution `04` / `10`. Do not add a second foundation clause unless future drift shows implementers are missing doc-14’s explicit affordance-menu sentence. Right now, a new clause would duplicate rather than clarify.

---

## 6. Candidate D — Single-charge accounting authority for derived quantities

### 6.1 Campaign lesson

0016 exposed the clearest defect: passive awake need deltas could double-charge ticks spanned by accepted sleep/work durations. The required correction was a single per-actor elapsed-tick accounting authority that classifies each tick exactly once as awake/asleep/working and routes all need deltas through that classifier (`TW-SPEC-0016`). 0017 and 0021–0022 then re-verified single-charge accounting, shared emitters, duration terminal classification, and evidence-path durability (`TW-SPEC-0017`, `TW-SPEC-0021`, `TW-SPEC-0022`).

What-level lesson: **replay-deterministic derived state must not be causally false. If a tick was spent asleep, Tracewake cannot also charge it as awake simply because two consumers each produced deterministic, byte-stable deltas.**

### 6.2 Tier-fit verdict

**Verdict: Bucket 2 → architecture / execution.**

The foundation already states the product-level truths: meaningful changes are eventful; replay must be deterministic; needs are causal pressures, not puppet strings; ordinary life must be causally honest (`TW-F-02`, `TW-F-03`, `TW-F-05`, `TW-F-06`). That is enough at foundation tier. The actual rule “each tick must be classified by one tick-regime authority and charged exactly once” is an architectural accounting contract. The spec ledger already records this as architecture conformance work: 0017 updated the conformance index for single tick charge, projection freshness, provenance-class audit, and believed-access; 0022 records shared need-delta emitter and debug split (`TW-SPEC-LEDGER`, `TW-A-00`).

The `TW-README` boundary is decisive: foundation does not define algorithms or subsystem layout. A single classifier, duration open/close key, or ledger invariant test is how. Execution `10` owns observability, diagnostics, mutation, and review artifacts; architecture owns the one-source-of-truth data-flow. `TW-RISK` has no need to promote this into a constitutional invariant because the foundation’s causal replay doctrine already condemns double-charged reality.

### 6.3 External prior-art survey

Discrete-event and agent-based simulation V&V literature treats conservation and accounting errors as model-validity problems, not as philosophical identity clauses. Axelrod’s simulation methodology distinguishes an unexpected emergent result from a programming error and emphasizes the extra work required to prove the implementation matches the conceptual model [AXELROD-2003]. ODD documentation exists to make agent-based models reproducible and inspectable, not to put every accumulator into the model’s constitution [GRIMM-2006; GRIMM-2010; GRIMM-2020].

Accounting analogies are useful but should not be overconstitutionalized. Double-entry accounting’s durability comes from an architectural invariant — debits and credits balance through a specific ledger discipline — rather than from the broad mission statement “be financially honest.” Tracewake’s analog is “derived state must be replay-honest”; the per-tick classifier belongs to architecture and execution proof.

### 6.4 Recommendation

No foundation amendment. Bucket-2 hand-off: later architecture work should name the authoritative derived-state accounting seams and their single-owner rule; execution should require replay/ledger evidence proving no double-charge, no drift among consumers, and no stable-golden encoding of semantically false values. This should be routed to architecture conformance and execution `10`, not to foundation `02`.

---

## 7. Candidate E — Emergence-as-evidence / EMERGE-OBS

### 7.1 Campaign lesson

The campaign’s generative-reachability work exposed a mismatch between “guards prove a phenomenon could happen” and “the living world has actually emitted unscripted phenomena.” 0018 introduced generative reachability and seed epistemics; 0019 emphasized generative reachability honesty; 0020 created the first read-only EMERGE-OBS baseline; 0025 carried the EMERGE-OBS derivation forward with a measured `emerge_obs_v1` table (`TW-SPEC-0018`, `TW-SPEC-0019`, `TW-SPEC-0020`, `TW-ACC-0020`, `TW-ACC-0025`).

`TW-X-10` currently treats EMERGE-OBS as an observation obligation, not a certification gate. It explicitly says the ledger is observer-only, never simulation input; it accumulates measured data rather than remaining the only foundational goal with no measured data; its counters are not pass/fail thresholds; and future ratchets must remain carefully bounded. `TW-ACC-0025` records a 12-row `emerge_obs_v1` table with distinct event-kind counts and measured interruptions, but zero expectation contradictions, zero replans/fallbacks, zero belief-truth divergences, and zero modeled-channel corrections in that scoped run.

What-level lesson: **for a living-world simulation whose thesis is unscripted ordinary-life causality, reachability is not enough. Acceptance must include observer-only evidence of what unscripted phenomena actually arose, while preserving the no-director rule and preventing the evidence mechanism from becoming a script.**

### 7.2 Tier-fit verdict

**Verdict: Bucket 1 → genuine foundation hole.**

The foundation already contains most adjacent doctrine. `TW-F-09` forbids authored outcome chains, hidden directors, plot managers, radiant quest machinery, and story-state schedulers. `TW-F-01` and `TW-F-03` define Tracewake as a forensic causal machine where story is retrospective. `TW-F-12` requires no-human runs, actor-knowledge gates, replay, institution gates, and first-playable proof cases. `TW-F-02` includes no-human and validation invariants such as `INV-091`, `INV-098`, and the truth-firewall family.

But the foundation does not yet state the missing what-level acceptance principle: **a world whose claims are about emergent ordinary-life phenomena must be judged partly by observer-only evidence of emitted, unscripted phenomena, not only by static guards and hand-picked reachability fixtures.** The current EMERGE-OBS text lives in execution `10` and expressly does not amend doctrine. That leaves the acceptance-observation principle below the tier that defines what Tracewake is.

This is not a request to move the EMERGE-OBS table, counters, command, threshold, fixture list, or zero-floor ratchet into foundation. Those remain execution. The foundation amendment should be compact and durable: Tracewake’s first-playable/living-world acceptance is incomplete if it cannot show, retrospectively and observer-only, that no-human simulation emits unscripted ordinary-life phenomena from modeled causes.

`TW-RISK` supports promotion. R-16 warns about no-human ordinary-life failure; R-27 warns about acceptance-evidence reachability overstatement. A risk-register entry is not doctrine. In this one case, watch-risk should become a foundation principle because the missing rule is product identity, not a proof mechanism.

### 7.3 External prior-art survey

#### 7.3.1 Agent-based modeling: emergence claims require runs, transparent observation, and reproducible evidence

Agent-based modeling literature distinguishes model description, verification, validation, and analysis of simulation outputs. The ODD protocol was created to make individual-based and agent-based models more understandable, complete, and reproducible [GRIMM-2006; GRIMM-2010; GRIMM-2020]. That is a doctrine of evidence: a model’s claims are not adequately communicated by asserting “agents can interact”; they require a reproducible account of entities, state variables, process overview, scheduling, design concepts, initialization, inputs, and submodels.

Axelrod’s “Advancing the Art of Simulation in the Social Sciences” is especially relevant to Tracewake’s campaign. He argues that unexpected results in simulation must be distinguished from programming mistakes, and that confirming a result is truly a consequence of the model can take more work than programming it [AXELROD-2003]. That maps directly onto EMERGE-OBS: a counter or trace saying a phenomenon occurred is not a director; it is evidence that lets the project tell the difference between real unscripted dynamics, unreachable claims, and accidental artifacts.

VOMAS extends this into a monitoring architecture: validation agents observe a simulation overlay, log constraints, and monitor violations without necessarily becoming part of the simulated world [NIAZI-VOMAS]. This is the closest external analogue to Tracewake’s observer-only EMERGE-OBS posture. The doctrine to carry upward is not “use VOMAS,” but “validation observers can be structurally outside the model while producing evidence about model behavior.”

#### 7.3.2 Weak emergence: if the macro phenomenon is known only by simulation, you must simulate and observe it

Bedau’s account of weak emergence treats emergent macrostate behavior as derivable from microdynamics only by simulation, not by simple inspection [BEDAU-1997]. Tracewake does not need a philosophical dissertation in foundation, but Bedau clarifies why a reachability proof is weaker than an observation ledger. If the product promise is that ordinary agents, institutions, memory, needs, records, and validation produce unscripted chains, then the project must run the world and record what actually happened. Otherwise the claim remains aspirational.

The trade-off is equally important. Emergence evidence cannot become a target that the simulation optimizes toward, because that would reintroduce a hidden director. Bedau-style weak emergence supports observation after the fact; it does not justify an in-world plot manager that forces interesting macrostate counters.

#### 7.3.3 Emergent narrative and living-world systems: bottom-up story must be shown, not declared

Emergent narrative research frames story as a retrospective product of autonomous character behavior in a simulated storyworld. Ryan, Mateas, and Wardrip-Fruin describe interactive emergent narrative as stories emerging bottom-up from autonomous characters rather than from authored plot chains [RYAN-OPEN-DESIGN]. Talk of the Town and related Bad News work are especially close to Tracewake: they simulate small-town histories and character knowledge propagation, with story experience arising through later curation or player interaction, not through a quest script [RYAN-TOTT-KNOWLEDGE; RYAN-BAD-NEWS; RYAN-CURATING]. Neighborly explicitly reconstructs Talk of the Town as a customizable community-scale social simulation for settlements of characters [NEIGHBORLY-2022].

The core prior-art lesson is not “add story sifting.” Tracewake has already chosen not to become a quest engine or director-driven narrative system. The relevant lesson is epistemic: emergent-narrative systems make their credibility through runs, logs, curation surfaces, and post hoc reconstruction. A living-world thesis that cannot present observed unscripted phenomena has not demonstrated the thing it claims to be.

#### 7.3.4 Accepted formulation for Tracewake

The best-practice formulation is narrow:

- **Observation, not steering.** Evidence collection must be outside the simulated world and must not feed agent cognition, scheduler priorities, authored events, or validators.
- **Retrospective, not prospective.** It records what happened; it does not create reasons for something to happen.
- **Phenomenon families, not authored outcomes.** Foundation should say the world must be evidenced to emit unscripted phenomena of the kind Tracewake claims: contradictions, replans, interruptions, stale-belief consequences, modeled-channel corrections, belief/truth divergences, wrong suspicion, record effects. It should not require a specific story beat or threshold in foundation.
- **Causal replay ancestry.** Evidence rows should be explainable by event log ancestry; otherwise they are metrics theatre.
- **No Goodhart target.** Foundation should warn that making counters into dramatic objectives would violate no-scripting doctrine.

### 7.4 Recommendation

**Recommend a Bucket-1 foundation amendment package.**

Recommended home and substance, without final wording or invariant number:

1. **`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`** — add a new invariant-substance item in the validation / no-scripting neighborhood. Substance: Tracewake’s living-world claim is not accepted merely by proving static reachability; acceptance must include replayable, observer-only evidence that unscripted ordinary-life phenomena actually arise from modeled causes in no-human or normal-controller runs. The observer evidence must not feed simulation behavior, author outcomes, or set dramatic objectives.

2. **`docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`** — add the explanatory doctrine. Substance: first-playable proof should include an emergence-evidence record alongside gates, as retrospective acceptance evidence. The foundation should name the principle, not the exact EMERGE-OBS command, schema, table, rows, counters, thresholds, or ratchet policy.

3. **`docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`** — add a short cross-reference. Substance: observing emergent phenomena after the fact is permitted and expected; steering the world to satisfy an emergence counter is forbidden authored outcome machinery.

4. **Glossary hand-off:** `TW-GLOSS` already has `evidence`, `observation`, `projection`, `holder-known`, and `stale information`, but it does not appear to define an “emergence evidence” term. A later reference session should add a canonical term such as `emergence evidence` or `observer-only emergence evidence`; do not silently coin synonyms.

This is a real doc-14-style promotion: the lower tiers discovered that an important product-level idea was living only as mechanics and risk memory. The foundation should state the idea compactly so later architecture/execution amendments can specialize it without inventing it.

---

## 8. Candidate F — Falsifiability / behavioral-proof / anti-vacuity

### 8.1 Campaign lesson

From 0016 onward, a large fraction of work concerned proof surfaces that looked green while proving little. 0016 found a tautological decision-context hash gate: it hashed stored inputs and compared them to a stored hash derived from the same inputs (`TW-SPEC-0016`). 0021 names the family directly as guard vacuity: structural locks asserted artifacts such as table presence, filter lists, pinned hashes, or ledger entry counts rather than behavior (`TW-SPEC-0021`). 0025 repaired executable witness discipline, provenance-true kill sets, envelope fail-closed decisions, manifest fingerprint honesty, embodied carrier census, and mutation phase-entry evidence rules; it also records that pending mutation status is not a pass and readiness requires a dated green scheduled mutation run (`TW-ACC-0025`).

What-level lesson: **a guarantee that cannot fail under the negative case is not a guarantee.** But the concrete campaign output is overwhelmingly about how to prove guarantees, not about what Tracewake is.

### 8.2 Tier-fit verdict

**Verdict: Bucket 2 → execution `10`, architecture `13`, and reference risk-register/glossary.**

Foundation already contains the necessary product-level acceptance harshness. `TW-F-02` includes `INV-091` through `INV-098`: no-human tests are mandatory, deterministic replay is tested, actor-knowledge leakage is high severity, possession parity is tested, and a feature is not accepted until harsh conditions are met. `TW-F-12` defines first-playable proof scope. `TW-F-14` requires authoritative decision traces, diagnostic quarantine, and truth-firewall evidence. That is enough at foundation tier.

The campaign’s lesson is proof discipline. `TW-X-10` is the natural owner: testing, observability, diagnostics, mutation evidence, review artifacts, and forbidden test patterns. `TW-A-13` owns validation and observability contracts at subsystem level. `TW-RISK` R-27, R-28, and R-29 already track reachability overstatement, incomplete correction closure, and decorative locks. Those risk entries should be strengthened and operationalized, but not promoted wholesale into foundation.

A minimal constitutional kernel such as “Tracewake guarantees must be falsifiable” is tempting but too broad. It risks smuggling gate philosophy into the constitution while adding little beyond `INV-091`…`INV-098`. The foundation should not absorb mutation testing, metamorphic testing, meta-witness registries, negative-fixture bijections, CI cache hygiene, or executable witness count floors.

### 8.3 External prior-art survey for tier placement

Popper’s falsifiability criterion gives the philosophical background: a claim has scientific content only when it could be refuted by observation [POPPER-1959]. That supports the intuition behind the campaign’s “decorative lock” concern. But Popper does not tell a software constitution which mutation perimeter or witness registry to use. It justifies a culture of refutable guarantees.

Mutation testing is the strongest software-engineering analogue. Jia and Harman survey mutation testing as a fault-based technique that introduces artificial defects to assess and improve test-suite adequacy [JIA-HARMAN-2011]. Papadakis et al. characterize mutation testing as a mature family of techniques for evaluating adequacy, guiding test generation, and supporting experimentation [PAPADAKIS-2019]. This matches Tracewake’s campaign exactly: a guard that survives the mutation it claims to catch is not evidence. But mutation testing is plainly execution/test machinery.

Metamorphic testing addresses the oracle problem by checking relations among multiple executions when exact expected output is difficult to know [SEGURA-2016; CHEN-2018]. Tracewake’s generative/metamorphic locks are in this family: they prove replay, tamper sensitivity, and invariant relations across generated runs. Again, this is a proof technique, not product identity.

Architecture fitness functions are the architecture-tier analogue. Ford, Parsons, and Kua define fitness functions as objective integrity assessments of architectural characteristics and use them to make architectural choices explicit and testable [FORD-PARSONS-KUA]. That is almost a perfect fit for `TW-A-13` and `TW-X-10`: objective integrity assessments of Tracewake’s truth firewall, replay, mutation perimeter, and artifact honesty. It is not a reason to amend `TW-F-02` with test mechanics.

The test-pattern literature also names the failure mode. xUnit Test Patterns discusses false positives, missing assertions, and test smells where tests pass without detecting faults [MESZAROS-XUNIT]. Tracewake’s “green but vacuous” gates are project-specific instances of a known testing risk.

### 8.4 Recommendation

No foundation amendment. Bucket-2 hand-off:

- **Execution `10`** should be the main target for explicit behavioral-proof doctrine: every lock must have a negative case that can fire; artifact-presence checks must be paired with behavior witnesses; pending evidence is not a pass; path-under-test evidence must be produced by the path under test; mutation and metamorphic proof surfaces must be scoped and reproducible.
- **Architecture `13`** should encode the subsystem-level requirement that validation surfaces expose enough typed observability for execution to test them non-vacuously.
- **Reference `01`** should keep R-29 as the named relapse risk and connect R-27/R-28/R-29 into a single acceptance-evidence honesty cluster.

Do not add “every invariant must be falsifiable” to foundation unless a later reassess session finds a concrete foundation invariant being used as an unfalsifiable slogan. The current defect is lower-tier vacuity.

---

## 9. Added candidate — Acceptance-evidence honesty / manifest-fingerprint honesty

### 9.1 Why this is added

The seeded list’s falsifiability theme covers most of this, but 0024–0025 expose a narrower recurring lesson worth routing explicitly: acceptance artifacts can be honest about a check’s existence while dishonest about what bytes/path/status it actually proves. `TW-SPEC-0024` and `TW-SPEC-0025` record meta-witness residue, mutation-perimeter derivation, manifest fingerprint honesty, envelope fail-closed behavior, CI evidence hygiene, and pending mutation status not being a pass. `TW-ACC-0025` records raw source-byte fingerprinting, fixture fingerprint repricing, CI workflow hygiene, and the rule-layer statement that pending mutation is not a pass.

### 9.2 Tier-fit verdict

**Verdict: Bucket 2 → execution / reference.**

This is evidence discipline, not a foundation truth. `TW-F-03` and `TW-F-12` require forensic causality and acceptance gates, but raw-byte fingerprint shape, manifest parity, evidence-map status language, and pending-status classification belong to execution artifacts. `TW-X-10` is the direct home; `TW-RISK` R-27 and R-28 are the watch-risks.

### 9.3 External prior-art survey

The ABM validation literature again helps. ODD’s purpose is transparent, complete, reproducible model description [GRIMM-2020]. Axelrod’s simulation methodology emphasizes sharing enough information for others to run, interpret, and understand the simulation [AXELROD-2003]. Software fitness functions and mutation testing add the test-evidence angle: an objective check must measure the intended characteristic, not a nearby artifact [FORD-PARSONS-KUA; JIA-HARMAN-2011].

### 9.4 Recommendation

Route to execution `10` and reference `01`. Later sessions should encode artifact honesty as an execution rule: evidence status must distinguish pass, pending, sampled, observation-only, and non-certifying measurements; fingerprints must cover the bytes they claim; and acceptance artifacts must not conflate historical implementation with certification. No foundation amendment.

---

## 10. Bucket-2 routing appendix

| Theme | Target tier | Target document(s) | Hand-off substance |
|---|---|---|---|
| Provenance sufficiency machinery | Architecture + execution | `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`; `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`; `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Single provenance-sufficiency definition; non-empty modeled source evidence for holder-known/institution-known cognition; fail-closed behavior for missing/invalid provenance; harness provenance must be real, not fabricated. |
| Memory freshness classifier | Architecture + execution | Architecture `03`/`06`; execution `04`/`10` | One observed-now vs remembered/stale classifier; preserve acquisition time/provenance; no truth restamping; parity across embodied and no-human surfaces. |
| Believed-access affordance proof | Architecture + execution | Architecture `03` and TUI/view-model boundary docs; execution `04`/`10` | Affordance menus generated from holder-known context snapshots; validators may use truth only to accept/reject; observation-time carrier capture; negative fixtures for wallhack affordances. |
| Single-charge derived accounting | Architecture + execution | Architecture conformance index and the ordinary-life/action/decision architecture docs; execution `10` plus ordinary-life execution proof docs | One authority for derived need deltas/duration lifecycle; no tick double-charge; shared open/close classification; replay evidence that stable goldens are semantically true, not merely byte-stable. |
| Falsifiability / anti-vacuity | Execution + reference | `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`; `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`; `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Every lock has a live negative; artifact-presence checks paired with behavioral witnesses; mutation/metamorphic tests scoped; pending evidence never passes; R-29 remains the named relapse risk. |
| Acceptance-evidence honesty / manifest fingerprints | Execution + reference | Execution `10`; reference `01`; acceptance-artifact template if amended later | Evidence maps distinguish pass/pending/observation-only/non-certifying; fingerprints cover raw bytes or explicitly say what they cover; acceptance reports do not overstate historical work as certification. |
| Emergence-evidence mechanism after foundation promotion | Architecture + execution + reference | After foundation amendment: architecture `13`, execution `10`, reference glossary/risk register | Define observer-only EMERGE-OBS data contract, evidence rows, allowed ratchets, anti-Goodhart constraints, and glossary term. Foundation should authorize the principle only. |

---

## 11. Open questions / residual uncertainty

1. **Exact final amendment wording and invariant placement are deliberately not supplied.** This report recommends substance and home only. Tracewake’s reassess process should decide final language, invariant ordering, and whether to amend an existing validation/no-scripting invariant or add a new one.

2. **EMERGE-OBS thresholds remain unresolved by design.** Foundation should not set numeric floors. Execution may later choose a zero-floor ratchet or other measured obligation, but must keep it observer-only and avoid turning counters into authored drama objectives.

3. **Possession-triggered perception at bind time remains a separate possible doctrine tension.** The architecture conformance index records an unresolved `INV-087`-adjacent decision about modeled perception on possession binding. This report did not promote it because the seeded amendment question was about foundation holes exposed by 0006–0025, and the fetched evidence was enough only to flag it as a later owner-decision topic, not to recommend doctrine.

4. **Institution-known provenance may deserve a future focused review.** Foundation `07` and `04` cover records/institutional knowledge and provenance, but the hardening evidence read here was mostly actor-known/no-human/TUI. If Phase 4 institutional machinery later repeats unbacked facts, that would be a separate amendment audit; today it is not a proven foundation hole.

5. **External prior art supports, but does not mechanically dictate, the amendment.** ABM validation, weak emergence, and emergent-narrative research all favor observer-only evidence of emitted behavior. None of them can answer Tracewake’s product-specific line between “evidence floor” and “hidden director”; that line must stay under Tracewake’s no-scripting doctrine.

---

## 12. References

### 12.1 Tracewake exact-commit sources

All Tracewake source citations in this report refer to the exact raw URLs listed in §1.1, fetched from `joeloverbeck/tracewake` at commit `f7adc0149963ea4a90b58ad05f633fd6e71e8649`.

### 12.2 External prior art

- [ANDERSON-SCHOOLER-1991] John R. Anderson and Lael J. Schooler, “Reflections of the Environment in Memory,” *Psychological Science* 2(6), 1991. DOI: `10.1111/j.1467-9280.1991.tb00174.x`. Source page: `https://journals.sagepub.com/doi/abs/10.1111/j.1467-9280.1991.tb00174.x`.
- [ARTEMOV-2008] Sergei Artemov, “The Logic of Justification,” *Review of Symbolic Logic* 1(4), 2008. DOI: `10.1017/S1755020308090060`. Source page: `https://www.cambridge.org/core/journals/review-of-symbolic-logic/article/logic-of-justification/ABD22C8DEC052CC92E26EC1B6DB03DFF`.
- [ARTEMOV-SEP] Sergei Artemov, Melvin Fitting, and Thomas Studer, “Justification Logic,” *Stanford Encyclopedia of Philosophy*, first published 2011. Source page: `https://plato.stanford.edu/entries/logic-justification/`.
- [AXELROD-2003] Robert Axelrod, “Advancing the Art of Simulation in the Social Sciences,” in *Handbook of Research on Nature-Inspired Computing for Economy and Management*, 2003 version. Source PDF: `https://www-personal.umich.edu/~axe/research/AdvancingArtSim2003.pdf`.
- [BADDELEY-HITCH-1974] Alan D. Baddeley and Graham Hitch, “Working Memory,” in *The Psychology of Learning and Motivation*, vol. 8, 1974. Source PDF mirror: `https://app.nova.edu/toolbox/instructionalproducts/edd8124/fall11/1974-Baddeley-and-Hitch.pdf`.
- [BEDAU-1997] Mark A. Bedau, “Weak Emergence,” *Philosophical Perspectives* 11, 1997, pp. 375–399. DOI: `10.1111/0029-4624.31.s11.17`. Source page: `https://philpapers.org/rec/BEDWE`.
- [BUNEMAN-2001] Peter Buneman, Sanjeev Khanna, and Wang-Chiew Tan, “Why and Where: A Characterization of Data Provenance,” ICDT 2001. DOI: `10.1007/3-540-44503-X_20`. Source page: `https://link.springer.com/chapter/10.1007/3-540-44503-X_20`.
- [CHEN-2018] Tsong Yueh Chen et al., “Metamorphic Testing: A Review of Challenges and Opportunities,” *ACM Computing Surveys*, 2018. DOI: `10.1145/3143561`. Source page: `https://dl.acm.org/doi/abs/10.1145/3143561`.
- [DE-KLEER-1986] Johan de Kleer, “An Assumption-Based TMS,” *Artificial Intelligence* 28(2), 1986, pp. 127–162. DOI: `10.1016/0004-3702(86)90080-9`. Source page: `https://dl.acm.org/doi/abs/10.1016/0004-3702%2886%2990080-9`.
- [DOYLE-1979] Jon Doyle, “A Truth Maintenance System,” *Artificial Intelligence* 12(3), 1979, pp. 231–272. DOI: `10.1016/0004-3702(79)90008-0`. Source page: `https://philpapers.org/rec/DOYATM`.
- [EBBINGHAUS-1885] Hermann Ebbinghaus, *Memory: A Contribution to Experimental Psychology* (1885; English translation 1913). Source page: `https://psychclassics.yorku.ca/Ebbinghaus/`.
- [FORD-PARSONS-KUA] Neal Ford, Rebecca Parsons, and Patrick Kua, *Building Evolutionary Architectures*, O’Reilly, 2017; 2nd ed. with Pramod Sadalage, 2022. Source pages: `https://www.oreilly.com/library/view/building-evolutionary-architectures/9781491986356/ch02.html`; free chapter `https://www.thoughtworks.com/content/dam/thoughtworks/documents/books/bk_building_evolutionary_architectures_second_edition_free_chapter.pdf`.
- [GREEN-2007] Todd J. Green, Grigoris Karvounarakis, and Val Tannen, “Provenance Semirings,” PODS 2007. DOI: `10.1145/1265530.1265535`. Source page: `https://dl.acm.org/doi/10.1145/1265530.1265535`.
- [GRIMM-2006] Volker Grimm et al., “A Standard Protocol for Describing Individual-Based and Agent-Based Models,” *Ecological Modelling* 198(1–2), 2006, pp. 115–126. DOI: `10.1016/j.ecolmodel.2006.04.023`. Source page: `https://www.ufz.de/index.php?de=40429`.
- [GRIMM-2010] Volker Grimm et al., “The ODD Protocol: A Review and First Update,” *Ecological Modelling* 221(23), 2010, pp. 2760–2768. DOI: `10.1016/j.ecolmodel.2010.08.019`. Source page: `https://pubs.usgs.gov/publication/70003622`.
- [GRIMM-2020] Volker Grimm et al., “The ODD Protocol for Describing Agent-Based and Other Simulation Models: A Second Update,” *JASSS* 23(2), 2020. DOI: `10.18564/jasss.4259`. Source page: `https://www.jasss.org/23/2/7.html`.
- [JIA-HARMAN-2011] Yue Jia and Mark Harman, “An Analysis and Survey of the Development of Mutation Testing,” *IEEE Transactions on Software Engineering* 37(5), 2011, pp. 649–678. DOI: `10.1109/TSE.2010.62`. Source page: `https://dl.acm.org/doi/10.1109/TSE.2010.62`.
- [KUHN-1953] H. W. Kuhn, “Extensive Games and the Problem of Information,” in *Contributions to the Theory of Games II*, Annals of Mathematics Studies 28, 1953, pp. 193–216. Source page: `https://www.jstor.org/stable/j.ctt1b9x1zv`.
- [MESZAROS-XUNIT] Gerard Meszaros, *xUnit Test Patterns: Refactoring Test Code*, Addison-Wesley, 2007. Source site: `http://xunitpatterns.com/`.
- [NEIGHBORLY-2022] Ian Johnson-Bey and collaborators, “A Sandbox for Simulation-Based Emergent Narrative,” IEEE CoG 2022. DOI: `10.1109/CoG51982.2022.9893631`. Source abstract: `https://www.kmjn.org/publications/Neighborly_CoG22-abstract.html`.
- [NIAZI-VOMAS] Muaz A. Niazi, Amir Hussain, and Mario Kolberg, “Verification & Validation of Agent Based Simulations using the VOMAS (Virtual Overlay Multi-agent System) approach,” arXiv:1708.02361, 2017. Source page: `https://arxiv.org/abs/1708.02361`.
- [ORKIN-FEAR] Jeff Orkin, “Three States and a Plan: The A.I. of F.E.A.R.,” Game Developers Conference / AAAI AIIDE materials, 2006. Source PDF: `https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf`; GDC page: `https://gdcvault.com/play/1013394/Three-States-and-a-Plan`.
- [PAPADAKIS-2019] Mike Papadakis, Marinos Kintis, Jie Zhang, Yue Jia, Yves Le Traon, and Mark Harman, “Mutation Testing Advances: An Analysis and Survey,” *Advances in Computers* 112, 2019, pp. 275–378. DOI: `10.1016/bs.adcom.2018.03.015`. Source page: `https://discovery.ucl.ac.uk/10056704/`.
- [POPPER-1959] Karl Popper, *The Logic of Scientific Discovery*, English edition, 1959. Publisher page: `https://www.routledge.com/The-Logic-of-Scientific-Discovery/Popper/p/book/9780415278447`.
- [RUSSELL-NORVIG] Stuart Russell and Peter Norvig, *Artificial Intelligence: A Modern Approach*, 4th ed., Pearson, 2020. Publisher page: `https://aima.cs.berkeley.edu/`.
- [RYAN-BAD-NEWS] Ben Samuel et al., “Bad News: An Experiment in Computationally Assisted Performance,” CHI EA 2016. DOI: `10.1145/2851581.2890375`. Source page: `https://www.researchgate.net/publication/307924025_Bad_News_An_Experiment_in_Computationally_Assisted_Performance`.
- [RYAN-CURATING] James Owen Ryan, *Curating Simulated Storyworlds*, PhD dissertation, UC Santa Cruz, 2018. Source PDF: `https://escholarship.org/content/qt1340j5h2/qt1340j5h2_noSplash_7b2b04b56dfeee9e4558c9f47e6cb25b.pdf`.
- [RYAN-OPEN-DESIGN] James Owen Ryan, Michael Mateas, and Noah Wardrip-Fruin, “Open Design Challenges for Interactive Emergent Narrative,” ICIDS 2015. Source PDF: `https://expressiveintelligence.github.io/papers/ryanEtAl_OpenDesignChallengesForInteractiveEmergentNarrative.pdf`.
- [RYAN-TOTT-KNOWLEDGE] James Ryan, Sifty, and Michael Mateas, “Simulating Character Knowledge Phenomena in Talk of the Town,” *Game AI Pro 3*, 2017. DOI: `10.1201/9780429055119-13`. Source page: `https://www.taylorfrancis.com/chapters/edit/10.4324/9781315151700-37/simulating-character-knowledge-phenomena-talk-town-james-ryan-michael-mateas`; PDF mirror: `https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf`.
- [SEGURA-2016] Sergio Segura, Gordon Fraser, Ana B. Sánchez, and Antonio Ruiz-Cortés, “A Survey on Metamorphic Testing,” *IEEE Transactions on Software Engineering* 42(9), 2016, pp. 805–824. DOI: `10.1109/TSE.2016.2532875`. Source page: `https://www.computer.org/csdl/journal/ts/2016/09/07422146/13rRUx0gewQ`.
- [SIMMHAN-2005] Yogesh L. Simmhan, Beth Plale, and Dennis Gannon, “A Survey of Data Provenance in e-Science,” *SIGMOD Record* 34(3), 2005. DOI: `10.1145/1084805.1084812`. Source page: `https://dl.acm.org/doi/10.1145/1084805.1084812`.
- [UNITY-NETCODE] Unity Technologies, “Tricks and patterns to deal with latency,” Netcode for GameObjects documentation. Source page: `https://docs.unity3d.com/Packages/com.unity.netcode.gameobjects@2.7/manual/learn/dealing-with-latency.html`.
- [W3C-PROV-DM] Luc Moreau and Paolo Missier, eds., “PROV-DM: The PROV Data Model,” W3C Recommendation, 30 April 2013. Source page: `https://www.w3.org/TR/prov-dm/`.
- [W3C-PROV-O] Timothy Lebo, Satya Sahoo, and Deborah McGuinness, eds., “PROV-O: The PROV Ontology,” W3C Recommendation, 30 April 2013. Source page: `https://www.w3.org/TR/prov-o/`.
