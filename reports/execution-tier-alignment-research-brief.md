# Research brief — Execution-tier alignment to the amended foundation + architecture (cascade pass)

**For ChatGPT-Pro Session 2 (the deep researcher).** You are *locked*: produce the deliverable
directly. Do **not** interview, do **not** ask clarifying questions — the requirements below are
final. The interview already happened. **Upload bundle = this prompt + the manifest
`manifest_2026-06-13_64a8367.txt`.**

Your job is an **internal doctrine-conformance analysis with forward routing**, not an
implementation. The two tiers *above* execution both recently moved: the foundation tier
(`docs/0-foundation/*`) was rewritten and then amended (a new constitutional truth-firewall document
and a new emergence-evidence invariant), and the architecture tier (`docs/1-architecture/*`) was just
realigned to that amended foundation (archived spec `0027`). The execution tier
(`docs/2-execution/*`) was last substantively edited during the `0006`–`0025` hardening campaign and
**predates both** the emergence-evidence foundation amendment **and** the architecture realignment.
You are deciding **what changes the execution tier needs so it faithfully realizes the *current*
foundation and architecture** — both the changes prior audits already routed to execution, and any
additional ripple the recent upstream changes created that no prior audit anticipated.

---

## §1 — Context, baseline commit, and source discipline

### What Tracewake is

Tracewake is a causality-first living-world simulation in Rust: agents act from partial belief,
institutions are fallible, and every meaningful event leaves a replayable, forensically explainable
trace. The product thesis is deliberately narrow and hard: a *small ordinary village* (not a quest
engine, not a player-centered mystery, not an LLM chatbot world) in which subjective belief,
event-sourced causality, fallible social machinery, and an actor-filtered TUI produce — with no
human present and no authored outcome chain — a chain like:

> I expected the coins to be in the chest. The chest is empty. I search. I ask. I remember. I
> suspect. I report. A record is made. Someone may be wrongly suspected. I delay payment or change
> my plan. Replay explains why every actor did what they did. The same chain can happen with no
> human present.

### Workspace layout (three crates, strict one-way dependency)

- `tracewake-core` — authoritative simulation kernel: event log, replay, actions/affordances,
  scheduler, projections, view models. **Zero dependencies.**
- `tracewake-content` — fixtures, content loading, schema validation. Depends on core only.
- `tracewake-tui` — terminal UI boundary: possession, embodied/debug view models. Depends on
  core + content.

Core must never depend on content or tui.

### Documentation is layered authority

Docs are tiered; earlier tiers govern later ones (index: `docs/README.md`):

```
0-foundation/  → constitutional doctrine: what Tracewake is and must never become
1-architecture/→ subsystem contracts: foundation translated into data-flow / authority / boundaries
2-execution/   → gate order, certification sequence, fixtures, proof obligations, review artifacts
3-reference/   → compact lookup, glossary (terminology control), design-risk register
4-specs/       → narrow implementation/corrective specs under live doctrine
archive/       → historical evidence (completed specs/tickets/reports), NOT live authority
```

Foundation outranks architecture outranks execution outranks reference outranks specs. A later layer
may *specialize* an upper tier's doctrine but may not *weaken* it. **Execution's job is to turn the
foundation `what` and the architecture subsystem-contract `how` into *proof obligations*** — gate
order and certification sequence, truth-firewall and anti-contamination checks, no-human and
possession-parity proof, golden fixtures and adversarial scenarios, replay acceptance, testing /
observability / diagnostics, and review artifacts. Execution owns *how a property is proven*; it does
**not** own product identity (foundation) or subsystem authority/data-flow contracts (architecture),
and it does **not** descend to crate-internal module layout, storage engine, serialization format, or
algorithm choice (those are implementation). This `foundation + architecture → execution`
realization fidelity is the single most load-bearing fact for your task: you are auditing whether the
execution tier still proves what the *current* foundation and architecture require it to prove.

### Why this pass exists — the two tiers above execution both moved

Three recent upstream changes are the source of the ripple you are tracing:

1. A **full foundation rewrite** expanded and restructured all of `docs/0-foundation/*` and
   introduced a new constitutional document, `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
   (the "truth firewall": *truth may validate actions, but truth may not plan them*).
2. A subsequent **emergence-evidence amendment** added a new invariant to
   `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — **INV-111**, "Living-world acceptance
   requires observer-only emergence evidence" — plus reinforcing additions to
   `docs/0-foundation/09_…` ("Emergence observation is not steering") and
   `docs/0-foundation/12_…` ("Emergence evidence as acceptance evidence"). Ratified and archived as
   `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`.
3. An **architecture-tier realignment** (archived spec `0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`)
   then translated those foundation changes into the architecture subsystem contracts: an
   observer-only emergence-evidence *data contract* and scoped story-sifting evidence wording (arch
   `13`/`11`), a shared provenance-sufficiency rule and memory-freshness classifier (arch `03`/`06`),
   observation-time believed-access snapshots (arch `10`), a single-owner derived-accounting seam
   (arch `00`/`04`/`05`/`09`), and a typed-observability requirement (arch `13`). That spec **routed
   all proof mechanics forward to execution** (its §6 Out-of-Scope and §V4 name `docs/2-execution/04`,
   `06`, `07`, `10`, `11` explicitly).

The execution tier was last substantively edited during the `0006`–`0025` campaign and predates the
emergence-evidence amendment and the architecture realignment. It already carries *some* current
doctrine — e.g. the execution index already declares an `EMERGE-OBS` observation obligation and
depends on foundation doc `14` — so a significant part of your job is to **validate what is already
present against live source** (close it with evidence where conformant), not assume everything is a
gap. The *full* extent of execution's non-conformance to the current foundation+architecture is what
you must determine. (Confirm dates and content against the fetched files — do not assume.)

### This brief continues a campaign — it is a *delta*, not a cold start

This is the **third** session in a tier-by-tier cascade. Read the predecessors' artifacts (named in
§2):

- `reports/foundation-amendment-research-brief.md` / `…-research-report.md` — the **first** session:
  it evaluated seven candidate themes from the `0006`–`0025` hardening campaign, found exactly **one**
  foundation hole (emergence-as-evidence → archived spec `0026`), and judged the other six themes
  (plus the *mechanism* half of emergence) to belong **below** the foundation tier.
- `reports/foundation-amendment-lower-tier-routing.md` — **the routing memo**: it preserves where
  each below-foundation lesson lands, tier by tier. **Its `docs/2-execution/*` session table is a
  validated seed for your pass** (themes A–G + the emergence mechanism E, each with its target
  execution doc(s) and the gate/proof obligation it must encode).
- `reports/architecture-tier-alignment-research-report.md` — the **second** session's deliverable
  (the architecture realignment, enacted as archived spec `0027`). **Its §5 forward-routing appendix
  is your freshest and most specific seed:** it maps each theme to specific execution docs with
  hand-off substance. Treat it as a validated candidate list, not a closed one.

The routing memo and the architecture report's appendix are **routing inputs, not doctrine and not
certification**. Treat each item as a *candidate to validate against live source*, not a settled
requirement — the hardening machinery may already cover much of it (the memo itself says so). Your
pass is the execution tier's turn: validate every routed item, **and** sweep for ripple no prior audit
anticipated (they were scoped to `0006`–`0025` campaign lessons and to the architecture tier, not to
proving the full current foundation+architecture at the execution tier).

### Baseline commit and fetch discipline (read carefully)

- **Fetch baseline commit:** `64a8367ca54f5daf97dac7031a708476d31a3707` (short `64a8367`). Every file
  you read must be fetched from **this exact commit**. Construct every raw URL as
  `https://raw.githubusercontent.com/joeloverbeck/tracewake/64a8367ca54f5daf97dac7031a708476d31a3707/<manifest path>`.
  This commit is *later* than the predecessor sessions' baselines and **contains the merged `0026`
  and `0027` amendments** — the foundation and architecture you measure against are their post-amendment state.
- **The manifest (`manifest_2026-06-13_64a8367.txt`) is path inventory only** — it proves a path
  existed in the tree at `64a8367`. It is never source text, never authority, never proof of latest
  `main`. Use it only to know which paths are fetchable.
- **Do not** fetch from `main`, `HEAD`, a branch name, code search, repository metadata, connector
  namespace labels, or any URL missing this owner/repo/commit. Trust only exact URLs containing
  `joeloverbeck`, `tracewake`, and `64a8367`. Abort and say so if a needed file cannot be fetched
  from this commit.
- **Commit hashes quoted *inside* archived specs and reports are that artifact's own provenance** (the
  commit *it* was accepted at) — they are historical, often predate later merges, and are **never**
  fetch targets. In particular: the foundation-amendment report and routing memo pin themselves to
  `f7adc01`; the architecture-tier report pins itself (and the foundation/architecture files it read)
  to `fdfd0b9`; spec `0026` records ratification at `91cc8a4`. **None of those are your baseline.**
  Fetch everything from `64a8367`, which is later and contains both merged amendments. If a referenced
  artifact cites a different "commit of record," note the divergence and use `64a8367`.
- This discipline exists because Tracewake's own design depends on provenance; the design-risk
  register codifies the failure modes (R-00 exact-commit drift, R-01 repository contamination, R-04
  stale-manifest trust). Mirror that discipline in your own research.

---

## §2 — Read in full (authority-ordered)

Fetch and read these from `64a8367`. Files split into roles. **Governing reference** is the current
foundation *and* architecture you measure execution against. **Amendment target** is the execution
tier under audit — read every one. **Inputs & boundary-awareness** frame the task and keep findings
routed correctly.

### Governing reference — current foundation (post-`0026`)

Execution must faithfully realize this. Primary drivers for this pass are called out; read the rest to
bound scope (boundary-awareness — read to know what foundation already owns, *not* to amend).

- `docs/README.md` — authority map and the per-tier `may define / may not define` table. **The
  contract for what execution is allowed to own vs. what belongs above (foundation/architecture) or
  below (reference/specs) it.** *(primary)*
- `docs/0-foundation/00_FOUNDATION_INDEX.md` — map, authority order, and the **`what` / `how` layer
  boundary** — the test for whether a recommendation belongs in execution at all. *(primary)*
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001`…`INV-111`. **INV-099…INV-110
  (truth-firewall family) and INV-111 (emergence evidence) post-date the execution tier.** Re-derive
  every invariant number you cite from *this file* (see §6). *(primary)*
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — subjective epistemics:
  provenance requirements, memory/staleness, rejected-provenance list. **Driver for the
  provenance-sufficiency and memory-freshness proof obligations.** *(primary)*
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — action authority, the
  perceived-vs-actual affordance split, survival needs. **Driver for believed-access and
  need-accounting proof.** *(primary)*
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — no-quest/no-director
  constitution; carries the **"Emergence observation is not steering"** clause. *(primary)*
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first-playable scope and
  mandatory proof cases; carries the **"Emergence evidence as acceptance evidence"** clause. *(primary
  — this is the foundation's own acceptance-gate doctrine, the closest upstream to execution's gates.)*
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **the new
  constitutional truth firewall.** "Truth may validate actions, but truth may not plan them." Single
  biggest upstream source of execution proof obligations. Read it most carefully. *(primary)*
- `docs/0-foundation/01, 03, 05, 07, 08, 10, 11, 13` — *(boundary-awareness)* read to know what
  foundation already owns (charter, replay contract, agents/planning, institutions, TUI/possession,
  scale/LOD, LLM boundary, research notes), so you do not re-state foundation `what` as an execution
  obligation.

### Governing reference — current architecture (post-`0027`)

The architecture tier was just realigned; it now states the subsystem contracts execution must prove.
Read the post-`0027` files as they stand at `64a8367` (the contracts are now *in* the docs — do not
treat the architecture report's recommendations as the authority; the **live architecture docs** are).

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — conformance index, universal
  conformance questions, single-charge seam pointer, deferred possession-bind row. *(primary)*
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **provenance
  sufficiency, memory-freshness rule, sealed holder-known context** — the contracts execution `04`
  must prove. *(primary)*
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` —
  **single-owner derived-accounting seam; proposal/validation split** — execution `05`/`06` proof.
  *(primary)*
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` —
  **memory-freshness classifier, provenance on beliefs/traces** — execution `04`/`10` proof. *(primary)*
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` —
  **observation-time believed-access snapshots; possession-not-a-knowledge-upgrade** — execution
  `07`/`10` proof. *(primary)*
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **the
  observer-only emergence-evidence data contract and the typed-observability requirement** — execution
  `10` proof. *(primary)*
- `docs/1-architecture/01, 02, 05, 07, 08, 09, 11, 12, 14` — *(boundary-awareness)* read to know the
  current subsystem contracts (authority/dependency, event-log/replay, actor decision transaction,
  speech/LLM, institutions, ordinary-life economy, incidents/leads, LOD, research/misreads), so a
  proof obligation you recommend matches the contract it proves and you do not re-open settled
  architecture.

### Amendment target — the execution tier under audit (read all 14)

Every one is a candidate for a recommended change. Read in order.

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — execution map, retirement rule, **canonical
  gate names (`P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`,
  `DIAG`) and the `EMERGE-OBS` observation obligation**, universal execution posture.
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — archived-
  spec status and certification posture.
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — first-proof
  identity and acceptance contract.
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — gate order, phase
  sequencing, Phase-4 block.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **primary home for
  provenance-sufficiency, memory-freshness, and truth-firewall proof obligations.**
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — actor
  transaction, scheduler, proposal/validation, no-direct-dispatch audit.
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **primary home for
  single-charge derived-accounting proof and no-human-day certification.**
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **primary home for
  believed-access (wallhack negatives, observation-time snapshot proof, embodied carrier census),
  possession parity, debug quarantine.**
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — authoring/schema/provenance
  validation, no outcome-chain data.
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — golden fixture families,
  adversarial scenarios, deterministic replay acceptance.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **the load-bearing
  doc for this pass:** `EMERGE-OBS` mechanism, anti-vacuity / live-negative / behavior-witness
  doctrine, acceptance-evidence honesty (status taxonomy, fingerprint scope), typed observability.
- `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — Phase-4 entry
  contract and institution/record/wrong-suspicion lock (institution-known-provenance watch lands here).
- `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` — locked deferral
  contract (boundary-aware: largely a deferral lock, not a primary amendment target).
- `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` — execution-level
  research decisions and forbidden misreads.

### Inputs & boundary-awareness — the routing seeds, the campaign map, and the forward fences

The first five are **primary inputs** (the validated seeds and their provenance). The remainder are
**boundary-awareness** reads (read to bound scope and route findings forward — *not* amendment targets;
do not propose changes to them here).

- `reports/foundation-amendment-lower-tier-routing.md` — **primary input.** Its `docs/2-execution/*`
  table is a validated candidate seed (themes A provenance-sufficiency, B memory-freshness, C
  believed-access, D single-charge accounting, E emergence mechanism, F falsifiability/anti-vacuity, G
  acceptance-evidence/fingerprint honesty), each with its target execution doc(s) and the gate/proof
  obligation it must encode. Validate every row against live source; treat none as closed.
- `reports/architecture-tier-alignment-research-report.md` — **primary input, freshest seed.** Its §5
  forward-routing appendix maps each theme to specific execution docs with hand-off substance, and its
  per-finding sections (§4) state what the architecture tier now owns (so you know the contract you are
  proving). Its X10/G finding (§4.8) and residual open questions (§6) hand specific items to execution.
- `reports/foundation-amendment-research-report.md` — **primary input.** The first determination's
  §3–§9 per-candidate reasoning and §11 residual open questions (institution-known provenance,
  possession-bind perception, EMERGE-OBS thresholds) — so a routed execution item quotes the real
  campaign pressure, not a paraphrase.
- `reports/foundation-amendment-research-brief.md` — **primary input.** The scope and source discipline
  of the campaign, and the house style these reports follow.
- `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md` and
  `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — **primary inputs.** The two
  ratified amendments. **Read their §6 Out-of-Scope sections most carefully:** they name exactly what
  was deferred *to this execution session* — `0026 §6` (EMERGE-OBS mechanism stays in execution `10`;
  architecture/execution/reference cascade is later, separate work) and `0027 §6` + §V4 (proof mechanics
  for D1–D7, the X10/G acceptance-evidence honesty cluster, and the EMERGE-OBS thresholds/ratchets all
  route to `docs/2-execution/04`, `06`, `07`, `10`, `11`). These define the spine of your routed-item list.
- `docs/4-specs/SPEC_LEDGER.md` — *(boundary-awareness)* the campaign map: one row per archived spec
  stating what each `0006`–`0025` spec exposed. Use it to trace a routed execution obligation back to
  the concrete hardening lesson so your recommendation quotes real pressure.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — *(boundary-awareness)* the operational watchlist
  (R-02 provenance collapse, R-09 epistemic leakage, R-16 no-human ordinary-life failure, R-27/R-28/R-29
  acceptance-evidence honesty cluster). A risk-register entry is not an execution gate; note where an
  item is a watch-risk and route reference-tier changes forward.
- `docs/3-reference/02_GLOSSARY.md` — *(boundary-awareness)* prescriptive terminology control. Use
  canonical terms; the emergence-evidence term is **not yet coined** (spec `0026` D4 routed it to the
  later reference session) — flag any needed glossary term for that session, never silently coin a synonym.
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — *(boundary-awareness)* the acceptance-artifact
  template the X10/G honesty cluster may eventually amend; read to know its current shape so you route
  template changes forward rather than rewriting them here.

You may explore any other file in the manifest as needed (specific `0006`–`0025` archived specs and
their acceptance reports in `reports/`, or `core`/`content`/`tui` and test-harness code seams as
corroboration per §3.6), but the files above are the load-bearing minimum.

---

## §3 — Settled intentions (final — do not re-open)

These were decided in the authoring interview. Treat them as fixed constraints.

1. **Amendment target is the execution tier only.** Your *recommended changes* apply to
   `docs/2-execution/*` and nothing else. Foundation **and** architecture are read as the **immutable
   governing references** — the authority you measure execution against — and are never amended.
   Findings whose correct home is `3-reference/` or `4-specs/` are **routed forward** to those later,
   separate per-tier sessions — flagged and briefly characterized, never encoded as execution
   obligations here.

2. **Recommend substance and home, not ratified text.** For each finding, state (a) *what proof
   obligation, gate check, fixture family, or review-artifact rule the execution tier must own* — in
   your own compact prose, at the execution proof level — and (b) *which execution file* it lands in
   (and whether it is a new section, an addition to an existing gate/obligation, or a correction of
   stale/contradictory text). **Do not** write final paste-ready execution prose, do not invent gate
   codes / observation-obligation codes / `INV-###` / status-taxonomy tokens, and do not redefine
   existing gate semantics. Existing tokens (`P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`,
   `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`, `EMERGE-OBS`) may be *cited* to locate doctrine; never
   redefine, weaken, or invent gate/obligation semantics. A later Tracewake reassess session writes the
   actual diffs.

3. **Full foundation+architecture → execution re-pass — validate *and* sweep.** Do both:
   - **Validate** every execution item the routing memo and the architecture report's §5 appendix name
     (themes A–G + emergence mechanism), and every `0026 §6` / `0027 §6`+§V4 hand-off, against live
     source. Confirm whether the current execution tier already owns the obligation (close it with
     evidence), partially owns it, or is silent/contradictory.
   - **Sweep** all 14 execution docs against the *current* foundation+architecture for ripple no prior
     audit anticipated — they were scoped to `0006`–`0025` campaign lessons and to the architecture
     tier, not to proving the full current foundation+architecture at execution. New or restructured
     upstream doctrine (especially doc 14's truth firewall, INV-111, and the architecture contracts
     `0027` just added) may require execution changes outside the seeded themes. Report those as
     first-class findings.
   The seeded items are a *validated seed*, not a closed list: you may reject a seeded item (with
   evidence the execution tier already proves it, or that it belongs further down) and add findings the
   upstream tiers evidence that the seeds miss.

4. **Execution is the primary destination, not a waystation.** Unlike the architecture pass — which
   routed most proof-mechanics *forward* to execution — this is where they **land**. Expect most
   findings to be **belongs-in-execution**. Forward-routing *from* execution is comparatively thin:
   essentially the emergence-evidence glossary term (→ `3-reference/02`), the R-27/R-28/R-29 risk-cluster
   wiring (→ `3-reference/01`), and any acceptance-artifact-template change (→ `4-specs/0003`). Do not
   manufacture forward-routes to avoid owning a proof obligation that is execution's by the authority table.

5. **A tier-fit verdict is mandatory for every finding.** For each, decide: **belongs-in-execution**
   (recommend the proof-obligation/gate/fixture/review-artifact change), **already-owned-close** (the
   execution tier already proves it — say so with evidence and close it; e.g. `EMERGE-OBS` is already
   declared in the execution index and may already be largely specified in `10`), or **route-forward**
   (the substance is terminology, a risk-register entry, or an implementation/spec artifact → name the
   target tier and document, hand it off). Justify against `docs/README.md`'s per-tier
   `may define / may not define` table and the doc-00 layer boundary.

6. **Doc-tier analysis is primary; code is corroboration.** The analysis is
   foundation/architecture-text vs execution-doc conformance. You **may** cite `tracewake-core` /
   `tracewake-content` / `tracewake-tui` and the existing test/fixture/CI harness as corroborating
   evidence that an obligation is already proven (so a routed item is already covered and can be closed)
   or is unproven — but the deliverable is about execution-doc conformance, not a code/certification
   audit. Do not turn this into the `P0-CERT` baseline-certification work; that is a later, separate
   effort under the execution doctrine you are realigning.

7. **External research is a first-class supporting delta — fresh surveys authorized.** Unlike the
   architecture pass (whose prior-art was mostly already surveyed), execution-proof methodology is new
   ground for this campaign. Research online as deeply as needed on the methods execution doctrine
   depends on, and cite every external claim that shapes a recommendation: **mutation testing and
   metamorphic testing** (how mature systems scope mutation perimeters and avoid vacuous/decorative
   guards), **property-based testing** (oracles, shrinking, invariants), **golden / approval testing and
   deterministic-simulation testing** (semantically-true vs merely byte-stable goldens, seed/replay
   determinism), **observability / telemetry contracts** (typed behavior witnesses, structured
   diagnostics designed to be falsifiable), and the **test-oracle problem / acceptance-evidence honesty**
   (evidence-status taxonomies, pending≠pass, fingerprint-scope honesty, sampled vs certifying evidence).
   The internal foundation/architecture → execution conformance analysis remains the spine; external
   prior art sharpens *how an execution proof obligation should be framed*, never replaces the internal
   analysis. Where the predecessor campaign already surveyed a topic (BDI/epistemic logic, AGM belief
   revision, PROV/data-lineage, information-flow/object-capability security, fog-of-war view derivation,
   event-sourcing read-model discipline, emergent-narrative systems), cite it and add only the
   execution-framing increment.

8. **Locked / no-questions; full source and terminology discipline** (see §1 and §6). No scope creep
   into other tiers' *amendment* work (only forward-routing flags), no new world mechanics, no Phase-4
   expansion, no implementation.

---

## §4 — The task

Produce one consolidated **execution-tier alignment report**: a change-proposal that tells a later
reassess session exactly which `docs/2-execution/*` proof obligations, gates, fixtures, and review
artifacts must change so the execution tier faithfully realizes the *current* foundation and
architecture. Concretely:

1. **Establish the upstream delta.** From the fetched foundation (post-`0026`) and architecture
   (post-`0027`), identify what the current upstream tiers now require that post-dates or restructures
   what execution was written against — at minimum INV-111 + doc 14, the doc 09/12 emergence clauses,
   and the architecture contracts `0027` added (observer-only emergence-evidence data contract, shared
   provenance-sufficiency rule, memory-freshness classifier, observation-time snapshots, single-owner
   accounting seam, typed observability). State each as a proof obligation execution must realize.

2. **Validate the routed execution items** (themes A–G + emergence mechanism; the `0026 §6` / `0027 §6`
   hand-offs). For each, quote the seed's intended gate/proof obligation and the campaign evidence
   behind it (via `SPEC_LEDGER.md` / the named spec), then check the live execution tier: already proven
   (close with evidence), partial, or absent/contradictory.

3. **Sweep for unanticipated ripple.** Walk all 14 execution docs against the current
   foundation+architecture; surface conformance gaps, stale cross-references, retired-doctrine
   leftovers, or contradictions no prior audit named — especially anything flowing from doc 14, INV-111,
   and the new architecture contracts.

4. **Render a tier-fit verdict for every finding** (belongs-in-execution / already-owned-close /
   route-forward), justified against the authority tables.

5. **Recommend substance and home** for every belongs-in-execution finding: which execution file, what
   the proof obligation/gate/fixture/review-artifact must assert (your prose), new-section vs addition
   vs correction — without final wording or invented identifiers.

6. **Carry the residual open questions forward.** The campaign hands execution at least: the
   `EMERGE-OBS` thresholds/ratchets/anti-Goodhart policy (foundation authorizes the principle only; the
   mechanism is execution `10`'s — and must stay observer-only and non-certifying); institution-known
   provenance future proof (execution `11`); and the possession-bind perception decision (an owner
   decision that, once made, execution `07` must later prove — do not decide it here). Address each: is
   it an execution-doctrine decision to make now, or an owner decision to surface? Do not silently drop them.

---

## §5 — Exploration and external-research mandate

> Explore the repository as deeply as needed beyond the files listed in §2 — including specific
> `0006`–`0025` archived specs and acceptance reports the ledger ties to a theme, and `core` /
> `content` / `tui` and test-harness code seams as corroboration (per §3.6). Research online as deeply
> as needed on execution-proof methodology (per §3.7) — mutation/metamorphic/property-based testing,
> golden/approval and deterministic-simulation testing, observability/telemetry contracts, and the
> test-oracle problem / acceptance-evidence honesty — wherever it sharpens a specific execution proof
> obligation. Cite every external claim that shapes a recommendation.

Distinguish clearly, throughout, between (i) fetched Tracewake source doctrine, (ii) external research,
and (iii) your own inference/recommendation — mirroring the repo's own provenance discipline (R-02).

---

## §6 — Doctrine and immutable constraints

- **Authority order is absolute.** Foundation outranks architecture outranks execution. If the current
  execution text conflicts with the current foundation or architecture, *execution is wrong* and must
  change — never recommend weakening or reinterpreting an upper tier to fit existing execution. A later
  layer may *specialize* an upper tier's doctrine; it may not *weaken* it.
- **`what` / `how` boundary** (`docs/0-foundation/00`, "Layer boundary"; `docs/README.md` per-tier
  table). Execution owns proof obligations: gate order, certification sequence, anti-contamination
  checks, fixtures, adversarial scenarios, replay acceptance, testing/observability/diagnostics, review
  artifacts. It does **not** own product identity (foundation) or subsystem authority/data-flow
  contracts (architecture); and it does **not** descend to crate-internal module layout, storage engine,
  serialization format, or planning-algorithm choice (implementation). If a finding's substance is one
  of those, it is *route-forward* or *out-of-scope*, not an execution change.
- **Invariant-citation discipline.** Any `INV-###` you reference must be re-derived from
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` *as fetched from `64a8367`*. **Never** trust
  `INV-###` numbers quoted inside archived specs or reports — they predate doctrine realignments and are
  frequently wrong. When you recommend an execution obligation, describe its *substance and home*; do
  **not** invent `INV-###` numbers or write ratified text.
- **Gate / observation-obligation codes are cross-references only.** Names like `P0-CERT`, `TFW`, `PIPE`,
  `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`, `EMERGE-OBS` may be *cited* to
  locate execution doctrine; never redefine, weaken, or invent gate/obligation semantics or new codes.
  In particular, `EMERGE-OBS` is an **observation obligation, not a certification gate** — it blocks
  nothing and must never become a pass/fail threshold without a dedicated future spec; preserve that.
- **Terminology.** Use the canonical terms in `docs/3-reference/02_GLOSSARY.md`. The emergence-evidence
  term is **not yet coined** (routed to the reference session as spec `0026` D4); flag the needed term
  rather than silently coining a synonym. No backwards-compatibility shims or alias paths in new
  doctrine.
- **Anti-contamination doctrine execution must prove:** no simulation fact may be born from prose;
  preserve event-sourced causality, subjective epistemics, ordinary agents, possession parity, fallible
  institutions, the truth firewall (truth may validate but not plan), and validation/replay. The
  emergence-evidence mechanism execution owns must stay **observer-only**: explainable through event-log
  ancestry, structurally unable to feed cognition / scheduler / validators or set dramatic objectives,
  and **non-certifying** (INV-111; doc 09 "observation is not steering"; doc 12; arch `13` data contract).
  The anti-vacuity doctrine must hold: every lock has a live negative that can fire (or an explicit
  reason one cannot exist); artifact-presence checks pair with behavior witnesses; pending evidence is
  never a pass; path-under-test evidence must be produced by the path under test, never harness-fabricated.
- **Source discipline** per §1 (exact-commit fetch from `64a8367`; manifest is inventory; archived-spec
  / report commit strings such as `f7adc01` / `fdfd0b9` / `91cc8a4` are not fetch targets; abort on
  contamination).
- **Archived specs are history, not certification** (R-26). Use the `0006`–`0025` series and their
  acceptance reports as evidence of *pressure on the obligations*, never as proof an obligation is
  already correctly stated or already certified.

---

## §7 — Deliverable specification

Produce **one** consolidated, paste-ready markdown document, downloadable, named:

> **`execution-tier-alignment-research-report.md`** (the user will place it in `reports/`).

It is an **analysis / recommendation (change-proposal) report for `docs/2-execution/*`** — *not*
rewritten execution docs, *not* a foundation/architecture amendment, *not* a numbered `docs/4-specs/`
spec (the numbered-spec numbering/ledger rules do **not** apply; this is a `reports/` analysis
artifact). It is **always produced** and is never a stub. Recommended structure (mirrors the
predecessor architecture report):

1. **Disposition table (top).** One row per finding → target execution doc → verdict
   (belongs-in-execution / already-owned-close / route-forward[to which tier]) → one-line basis. The
   reader should see the whole change surface at a glance, with the seeded themes and the swept-in
   findings both visible.
2. **Method & provenance ledger.** The exact-commit fetch list you used (all from `64a8367`), the
   contamination statement, and the (i)/(ii)/(iii) provenance convention from §5.
3. **Upstream delta.** What the current foundation+architecture now require that execution was not
   written against (INV-111, doc 14, doc 09/12 emergence clauses, and the `0027` architecture contracts),
   each as a proof obligation execution must realize.
4. **Per-finding sections.** For each finding (seeded and swept-in alike), in a stable order grouped by
   execution doc:
   - *Upstream driver* — the current foundation/architecture requirement (quote the file / INV / arch
     contract) that creates the obligation, plus the campaign evidence where relevant (quote
     `SPEC_LEDGER.md` / the named spec).
   - *Current execution coverage* — what the live execution tier says today: already proves it, partial,
     silent, or contradictory (quote the execution file). Code/harness corroboration where it changes
     the verdict (per §3.6).
   - *Tier-fit verdict* — belongs-in-execution / already-owned-close / route-forward (named tier/doc),
     justified.
   - *Recommendation* — for belongs-in-execution: the proof-obligation substance (your prose) and its
     home (which execution file; new section / addition / correction), without final wording or invented
     identifiers. For already-owned: the evidence that closes it. For route-forward: the target tier/doc
     and a one-paragraph hand-off.
5. **Forward-routing appendix.** A consolidated backlog of everything routed beyond execution (theme →
   target tier: `3-reference` / `4-specs` → target document → the lesson it must encode), so the later
   per-tier sessions inherit a clean, ordered hand-off — the same service the routing memo and the
   architecture report did for this session.
6. **Residual open questions.** The §4.6 carry-forwards (EMERGE-OBS thresholds/ratchets; institution-known
   provenance; possession-bind perception) and anything the fetched evidence could not settle, stated
   honestly.
7. **References.** Full citation list for any external prior art used.

**Locked / no-questions:** Produce the deliverable directly as a downloadable markdown document. Do not
interview, do not ask clarifying questions — the requirements above are final. If a genuine
contradiction makes a requirement impossible, state it in the deliverable and proceed with the most
faithful interpretation.

---

## §8 — Self-check before returning

Run this against your own output:

- [ ] Every Tracewake file was fetched from `64a8367`; no `main`/branch/search/metadata fetches; no
      archived-spec or report commit string (e.g. `f7adc01`, `fdfd0b9`, `91cc8a4`) used as a fetch
      target. Contamination statement present.
- [ ] The upstream-delta section names the post-execution foundation **and** architecture changes
      explicitly (at minimum INV-111, doc 14, and the `0027` architecture contracts), each re-derived
      from the fetched upstream tiers — not from the archived specs' or reports' paraphrases.
- [ ] Every routed execution item (themes A–G + emergence mechanism; the `0026 §6` / `0027 §6`+§V4
      hand-offs) was validated against live source — proven-and-closed, partial, or absent — not merely
      restated.
- [ ] The 14-doc sweep ran: at least the docs with no seeded item were checked for upstream-driven
      ripple, and any swept-in finding is reported as first-class.
- [ ] Every finding has an explicit tier-fit verdict (belongs-in-execution / already-owned / route-
      forward) justified against the authority tables — not just asserted.
- [ ] The `what` / `how` filter held: no recommended execution change re-states foundation product
      identity, re-decides an architecture subsystem contract, or descends to module layout, storage/
      serialization, or a planning algorithm.
- [ ] No `INV-###`, gate codes, observation-obligation codes, or status-taxonomy tokens were invented;
      no final execution prose was authored; recommendations are substance + home. Existing gate/EMERGE-OBS
      semantics were cited, never redefined or weakened.
- [ ] The emergence-evidence mechanism recommendation stays observer-only and **non-certifying**:
      explainable via event-log ancestry, structurally unable to feed cognition/scheduler/validators,
      never a pass/fail threshold; the anti-vacuity doctrine (live negatives, behavior witnesses,
      pending≠pass, path-under-test evidence) is preserved.
- [ ] External prior art is a cited supporting delta (predecessor surveys treated as done; fresh
      execution-proof-methodology research added per §3.7); the (i)/(ii)/(iii) provenance distinction is
      maintained.
- [ ] Residual open questions (EMERGE-OBS thresholds/ratchets; institution-known provenance;
      possession-bind perception) are addressed or explicitly carried, not dropped.
- [ ] The forward-routing appendix is consolidated so the `3-reference` / `4-specs` sessions inherit a
      clean hand-off; glossary terms used canonically, any needed addition (e.g. the emergence-evidence
      term) flagged.
- [ ] The deliverable is one complete, paste-ready `execution-tier-alignment-research-report.md`.
