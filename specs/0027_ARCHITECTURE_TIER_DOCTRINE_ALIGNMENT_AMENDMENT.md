# Spec 0027 — Architecture-Tier Doctrine-Alignment Amendment

This spec **proposes a set of architecture-tier (`docs/1-architecture/*`) amendments**. It is a
design/proposal artifact: it specifies the *substance and home* of each amendment so Tracewake's
reassess / ticket process can author the final architecture wording. **It does not itself author
ratified architecture prose.** Architecture is tier-1 doctrine but is not constitutional, so
enactment requires owner approval rather than the constitutional sign-off that a foundation
amendment demands (cf. `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`).

> Section set: this file uses the canonical `specs/` section set (Problem Statement, Approach,
> Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions) because
> `specs/` carries no template at authoring time and this is not a hardening implementation spec.
> It deliberately does **not** copy the foundation-pack docs' narrative house style.

**Status:** PROPOSED. Staged in `specs/`; not yet enacted into `docs/1-architecture/*`.

**Admissibility posture:** `P0-CERT not applicable`. This is a doctrine-alignment proposal; it
certifies no code and performs no gate audit.

**Authority:** A spec is subordinate to architecture and *may not replace architecture*
(`docs/4-specs/SPEC_LEDGER.md`). This spec does not exercise that forbidden authority. It
**operationalizes higher-tier (foundation) doctrine** — the truth-firewall family `INV-099…INV-110`
(`docs/0-foundation/14`) and the emergence-evidence invariant `INV-111`
(`docs/0-foundation/02`, ratified by archived spec `0026`) — by staging *additive* refinements and
one *scoping correction* to architecture so the architecture tier translates that doctrine fully.
No deliverable weakens or contradicts existing architecture; the final wording, once authored, is
architecture-tier doctrine and this spec becomes historical provenance.

**Provenance:** derived from `reports/architecture-tier-alignment-research-report.md` (external
deep research, pinned to commit `fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9` = current `HEAD`
`fdfd0b9`) and its brief `reports/architecture-tier-alignment-research-brief.md`. The report is the
planned `docs/1-architecture/*` session of the routing backlog
`reports/foundation-amendment-lower-tier-routing.md`. The report's eight verdict-critical
architecture-doc gap claims were independently re-verified against live `HEAD` during authoring
(see Verification §V1); the report's fabricated `#Lxxxx` line anchors were ignored in favor of the
named symbols.

---

## 1. Problem Statement

The `0006`–`0025` hardening campaign and the subsequent foundation rewrite hardened the foundation
tier in three ways the architecture tier has only **partly** absorbed:

1. **The truth firewall, `docs/0-foundation/14` + `INV-099…INV-110`** — "truth may validate
   actions, but truth may not plan them." Architecture broadly translated this (A00/A03/A04/A05/
   A08/A10/A12/A13), but four doctrine threads remain **fragmented across A00 hardening rows and
   examples** rather than stated as compact, reusable subsystem contracts in the primary docs:
   provenance sufficiency, memory freshness, believed-access affordance snapshots, and single-charge
   derived accounting. A future subsystem author cannot apply these without reading A00's
   conformance table.

2. **`INV-111` + the emergence-evidence amendment (archived spec `0026`)** — living-world
   acceptance now requires *replayable, observer-only* evidence that unscripted phenomena arose from
   modeled causes. Spec `0026` ratified the *principle* in foundation and explicitly deferred the
   "architecture/execution/reference cascade" as later work. Architecture has **no** home for the
   observer-only data contract: A13 lists no such artifact family, and A11's pre-`INV-111` blanket
   "may not create evidence" wording can now be misread as forbidding the very observer-only
   acceptance evidence the constitution requires.

3. **The foundation rewrite sharpened the anti-vacuity / behavioral-proof posture** (design risks
   R-27/R-28/R-29). A13 rejects "looks right" and display-string proof, but does not require
   protected subsystems to expose the **typed behavior-witness fields** execution needs to build a
   live negative — so an artifact can exist yet be unfalsifiable.

None of these is a foundation hole (only emergence-as-evidence was, and that is closed by `0026`).
Each is a **subsystem-contract** gap: foundation states the `what`; architecture must own the
data-flow / authority boundary; execution and reference own the proof mechanics and terminology.
The architecture-tier alignment report dispositioned eleven findings — seven warrant architecture
amendments, the rest route forward, are already owned, or are deferred owner decisions.

## 2. Approach

Stage seven compact, additive architecture refinements (six additions + one scoping correction),
each landing in the primary subsystem doc that *owns* the contract, with cross-links where the
report identifies a secondary anchor. Keep every change at the `what may read/write/derive/expose/
preserve` level. Route every threshold, fixture, command, ratchet, schema field, newtype, table/row
name, status taxonomy, and gate procedure **out** of architecture to execution `docs/2-execution/*`
and reference `docs/3-reference/*` (the report's forward-routing appendix and the routing backlog
already enumerate these hand-offs).

The seven deliverables are the only architecture-tier items; the report's other findings are routed
or deferred (Out of Scope §6). Final architecture wording is authored on enactment (by ticket /
reassess), not in this spec.

## 3. Deliverables

All deliverables are **proposed amendment substance**, to be authored into the named architecture
file by the reassess / ticket process. None prescribes final wording, schema fields, data
structures, function names, table/row names, or fixtures.

| # | Report ID | Target file | Kind | Substance |
|---|---|---|---|---|
| D1 | A13-E1 | `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (xref `11`) | Addition (new artifact family) | An **observer-only emergence-evidence record**: a retrospective review artifact — not world state, holder-known/institution-known context, validation input, or scheduler input. Each evidence row carries event-log ancestry sufficient to replay/explain its phenomenon (source run, seed/random provenance where applicable, controller-mode / no-human-or-normal-controller run identity, phenomenon family, source event IDs / causal-chain refs, extraction time, projection/review version). It may classify phenomenon *families* (contradictions, replans, interruptions, stale-belief consequences, modeled-channel corrections, belief/truth divergence, wrong suspicion, record effects) but must name no required story beats, dramatic objectives, or numeric floors. The data path is **one-way**: authoritative simulation → replay/projection/story-sifting observer → review artifact; it must never feed cognition, validators, scheduling/candidate generation, LOD-promotion decisions, event-spawning story sifting, or authoring objectives. Add invalid-pass conditions: evidence fabricated from debug/fixtures rather than path-under-test ancestry; counters used to steer the simulation; rows untraceable to modeled causes. |
| D2 | A11-E2 | `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | **Correction** (scope overbroad wording) | Scope the story-sifting "may not **create evidence**" rule so it forbids **diegetic / holder-known / institution-known / in-world evidence, clues, proof, records, sanctions, rewards, or action reasons** produced by a sifter, while explicitly **allowing** observer-only retrospective acceptance/review evidence under D1's one-way contract (reproducible from event/projection input, carrying event-log ancestry, structurally quarantined from cognition/schedulers/validators). This preserves the questless / no-director doctrine; it does not weaken it. |
| D3 | A03/A06-A | `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` (cross-link from `06`) | Addition (consolidation) | A compact **provenance-sufficiency** subsection. An input to cognition/procedure/view-model selection is provenance-sufficient only if it cites ≥1 modeled acquisition/derivation route appropriate to the asserted fact kind (observation, search/contact event, absence observation, memory of a prior modeled source, speech/testimony, record/public artifact, routine/role assignment, institutional-procedure state, LOD-summary event with ancestry, or an explicit unknown/unverified placeholder). A source label, boolean, display sentence, fixture/branch/test name, debug comparison, validator detail, or raw physical-truth lookup is **not** provenance. Derived facts preserve lineage. Missing / empty / dangling / wrong-kind / ambiguous / forbidden-source provenance **fails closed** before action-relevant use (still available to debug as a non-diegetic failure artifact). Institution-known contexts are not exempt. |
| D4 | A06-B | `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` (A03 already owns the parallel rule) | Addition (parity) | State the **observed-now vs remembered/stale freshness classifier** as A06's own epistemic-data-flow contract (today it lives only in A03): `observed_now` is limited to the holder's current modeled perception/contact/search window; older usable facts remain planning-available only as memory/belief/stale information, preserving source event, acquisition time, last verification, and provenance class; selecting an old fact for a new decision does not restamp it as current observation, and neither validation truth nor debug comparison refreshes it; no-human cognition, embodied TUI view models, notebooks, and holder-known contexts use the **same** classifier (no fresher epistemic surface for possession/UI than for autonomous actors). |
| D5 | A10-C | `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (xref `03`) | Addition | Embodied view models and semantic-action entries consume holder-known context plus permitted projection records whose visible / carried-item / container / current-place attributes were **captured at a modeled observation, bind/preflight, or perception boundary**. The TUI renders observed labels/attributes and actor-known affordances from those captured records; it must not hold a live physical-state handle or re-read truth to "freshen" labels, carried-item attributes, routes, workplace availability, food sources, or hidden blockers. Validators may still reject a selected semantic action using authoritative truth, but rejection feedback splits actor-visible modeled feedback from debug-only detail. Snapshot/capture applies to no-human parity surfaces and embodied possession alike (possession is not a knowledge upgrade). Does **not** decide the separate possession-bind perception question (§7). |
| D6 | A04/A05/A09-D | `04_…PIPELINE.md`, `05_…PLANNING.md`, `09_…PROPERTY.md` (+ optional one-line index pointer in `00`) | Addition (single-owner seam) | Name the authoritative **single-charge derived-accounting seam**. **A04** (near reservations/durations): derived need-deltas, elapsed-time effects, duration completion/interruption, and body-exclusive open/close state flow through one authoritative accounting seam; schedulers, validators, action definitions, and projections may *consume* it but may not independently charge the same tick/window or silently reconcile duplicate terminals. **A05**: actor decision transactions consume live need pressures from event-sourced agent state; candidate generation may *explain* pressure but must not mint need-deltas, supply proposal-side current-need values as authority, or let routine labels charge time. **A09**: food/sleep/work/fatigue/hunger/wages/local-economy share that same event-sourced accounting + duration lifecycle — stable replay / byte-stable goldens are insufficient if two consumers causally charge the same tick/window twice. **A00**: keep the hardening rows as conformance examples; optionally add a one-line pointer to the new compact A04/A05/A09 seam. |
| D7 | A13-F | `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Addition (typed observability) | For every validation / anti-contamination / replay / projection / diagnostic guarantee, the architecture surface must expose **typed, path-under-test observability**: responsible layer, source event/proposal/context IDs, checked facts, behavior-witness fields, accepted/rejected stage, and enough ancestry to distinguish production-path behavior from fixture/harness fabrication. An artifact's mere existence, shape, count, checksum, or display text is insufficient unless paired with typed behavior evidence appropriate to the protected claim. The surface must be designed so execution can attach live negatives and mutation/metamorphic checks — but A13 defines no tests, thresholds, commands, or pass statuses (those are execution `10`). |

D2 is a corrective re-scoping; D1 and D3–D7 are additive subsystem contracts. All are consistent
with existing architecture and with foundation doctrine.

## 4. Invariants Alignment

| Invariant / doctrine | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-111` — observer-only emergence evidence | aligns (translates) | D1 gives architecture the data-flow/authority home for the observer-only record at the validation/review-artifact surface; D2 removes the A11 wording that would otherwise read as forbidding it at the story-sifting surface. Neither feeds simulation behavior. |
| `INV-101` sealed holder-known context / `INV-102` source-&-provenance for action-relevant cognition | aligns (consolidates) | D3 states the sufficiency boundary (non-empty modeled source, fail-closed) at the holder-known/institution-known context surface; D4 states the freshness classifier at the epistemic-data-flow surface. Both compact existing scattered doctrine; neither relaxes it. |
| `INV-099/100/109` — truth may validate, not plan; possession is not a cognition upgrade (`INV-108`) | aligns | D5 forces embodied menus/view models onto observation-time captured material at the TUI surface, keeping hidden truth out of affordance enumeration while validators still reject on truth. |
| Event-sourced causality / deterministic replay (`docs/0-foundation/03`; `INV-001…`) | aligns | D6 names a single-owner accounting seam at the action-pipeline / cognition / ordinary-life surfaces so no tick/window is causally charged twice — protecting replay truth, not merely byte-stable goldens. |
| Harsh acceptance / no "looks right" (`INV-098`; `INV-097`) | aligns (extends) | D7 requires typed behavior witnesses at the validation/observability surface so guards are falsifiable (anti-vacuity, design risk R-29). |
| `INV-087` human-focus-is-not-privilege / `INV-006` possession transfers no knowledge | N/A here (preserved) | D5 explicitly does **not** decide bind-time possession perception; the deferred `INV-087`-adjacent tension is preserved (§7), no silent constitutional edit. |

No invariant is weakened or tensioned. All deliverables are additive/clarifying `what`-level
architecture doctrine; no `how`-level mechanism (threshold, fixture, command, schema) enters
architecture.

## 5. Verification

- **V1 — Gaps confirmed against live `HEAD` (done at authoring).** All eight architecture-doc gap
  claims were re-verified on `HEAD` `fdfd0b9` (the report's pinned commit), ignoring the report's
  fabricated line numbers: `INV-111` is present in `docs/0-foundation/02`; A11 carries the bare
  "create evidence." bullet; A13 contains **no** emergence/EMERGE artifact family and no typed
  behavior-witness requirement; A03 has `provenance_edges[]` + allowed/forbidden classes + the
  `observed_now` rule but no compact provenance-sufficiency rule; A06 carries belief acquired/
  last-verified/stale-after ticks but states neither a sufficiency rule nor the freshness classifier
  as its own contract; A10 generates embodied views from holder-known context but does not require
  observation-time snapshots; A04/A05/A09 own their pipelines but name no single-owner accounting
  seam; A00 carries the Phase-3A hardening rows and records the deferred `INV-087` bind-time
  perception decision.
- **V2 — Enactment acceptance (on implementation).** Each deliverable is accepted only when its
  substance is authored into the named architecture file as a compact subsystem contract (not an
  A00 row only), D2 scopes rather than deletes the questless guard, and no execution-tier mechanism
  token (threshold, fixture, command, `EMERGE-OBS` table/row/counter/ratchet name, schema field)
  enters architecture.
- **V3 — Boundary check.** Grep the newly added architecture passages for mechanism tokens
  (`EMERGE-OBS`, table/row/counter/threshold/ratchet/fixture/command names): architecture must hold
  the data contract and authority boundary only; a whole-file grep is not the proof surface because
  architecture legitimately discusses these concepts in the abstract.
- **V4 — Forward-routing follow-through (later sessions, not this spec).** The execution `10`/`04`/
  `06`/`07`/`11` proof obligations, the reference `01` risk-cluster wiring, and the reference `02`
  emergence-evidence glossary term (spec `0026` D4) are enacted by their own tier sessions; this
  spec only records the hand-offs (§6).

## 6. Out of Scope

- **Final architecture wording, schema fields, newtypes, data structures, function/table/row names,
  fixtures, thresholds, commands, ratchets, status taxonomies.** Owned by the reassess/ticket
  enactment and by execution/reference.
- **X10/G — acceptance-evidence / manifest-fingerprint honesty.** Routed forward, **not**
  architecture: A13's existing "no display-string / no looks-right" posture suffices at the
  architecture tier. Evidence status taxonomy (pass/pending/sampled/observation-only/non-certifying),
  raw-byte fingerprint scope, and acceptance-artifact honesty go to `docs/2-execution/10` and
  `docs/3-reference/01`.
- **Proof mechanics for D1–D7.** Negative fixtures, wallhack negatives, observation-time snapshot
  proof, embodied carrier census, replay/ledger no-double-charge evidence, live negatives /
  mutation-metamorphic surfaces, and `EMERGE-OBS` thresholds/ratchets/anti-Goodhart constraints go
  to `docs/2-execution/04`, `06`, `07`, `10`, `11`.
- **Emergence-evidence glossary term.** `docs/3-reference/02` coins the canonical term (spec
  `0026` D4); not authored here.
- **Foundation edits.** The principle (`INV-111`) and the truth firewall (`INV-099…INV-110`) are
  already ratified; this spec changes no `docs/0-foundation/*` file.
- **Architecture docs `01`, `02`, `07`, `08`, `12`, `14` — verified conformant, no deliverable.** The report's 15-doc sweep found these carry no `INV-111` / truth-firewall contradiction requiring amendment (`08` is already-owned-close: its sealed institution-known context with `provenance_edges` already satisfies the doctrine, watched under R-D). Their absence from §3 Deliverables is deliberate, not an oversight.
- **New world mechanics, Phase-4 expansion, LLM surfaces, crate/code changes.**

## 7. Risks & Open Questions

- **R-A — Architecture enactment requires owner approval.** Authoring D1–D7 edits tier-1 doctrine.
  Lighter than constitutional sign-off, but it must not proceed by convention; this spec stages the
  substance, it does not authorize the edits.
- **R-B — D2 mis-scoping relapse.** Over-narrowing "create evidence" could reopen a no-director hole
  (a sifter minting diegetic clues); under-narrowing leaves the `INV-111` contradiction. Enactment
  must preserve the forbidden-evidence list intact and only carve out the observer-only D1 path.
- **R-C — Goodhart relapse on D1.** The emergence-evidence record must stay observer-only; counters
  becoming simulation objectives is forbidden authored-outcome machinery (design risk R-27). D1's
  one-way path + invalid-pass conditions exist to forbid this at the architecture tier; execution
  `10` must keep it non-certifying.
- **R-D — Institution-known provenance residual (already-owned; watch).** A03/A08 already own
  sealed institution-known context with provenance edges and forbid event-log truth conversion. No
  architecture amendment now. If Phase-4 institutional machinery introduces new subsystems that
  repeat unbacked facts, revisit A08 then (report §4.9; routing backlog §residual-1).
- **R-E — Possession-bind perception (deferred owner decision; do NOT silently amend).** A00 records
  an unresolved `INV-087`-adjacent tension: whether possession *binding* may emit modeled perception
  (move it into an actor decision transaction) or whether a foundation clarification permits
  possession-triggered modeled perceptions. This spec preserves the deferred state. If the owner
  decides, the home is A10 possession semantics + A03 holder-known construction with A00 recording
  the stance — and the perception must be a modeled event/channel for the actor, never a
  human/player knowledge transfer (report §4.10; routing backlog §residual-2). D5 does not touch
  this.

## 8. Provenance & Source Discipline

- The source report is pinned to `fdfd0b9` and was re-verified against that same live `HEAD`; no
  intervening commit drift applies.
- Commit hashes named here are audit/spec provenance only.
- This spec adds **no** `docs/4-specs/SPEC_LEDGER.md` row at proposal time, per the staged-spec
  convention (the `0026`/hardening-series precedent: the ledger row lands at acceptance/closeout,
  not at proposal).
