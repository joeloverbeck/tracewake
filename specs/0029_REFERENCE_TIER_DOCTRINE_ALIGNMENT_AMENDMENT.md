# Spec 0029 — Reference-Tier Doctrine-Alignment Amendment

This spec **proposes a set of reference-tier (`docs/3-reference/*`) amendments**. It is a
design/proposal artifact: it specifies the *substance and home* of each amendment so Tracewake's
reassess / ticket process can author the final reference wording. **It does not itself author
ratified reference prose.** Reference is a tier-3 operational aid below execution (it owns compact
naming control and risk memory, not doctrine, architecture contracts, or gate semantics), so
enactment requires ordinary owner approval rather than the constitutional sign-off a foundation
amendment demands (cf. `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`,
`archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`, and
`archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`).

> Section set: this file uses the canonical `specs/` section set (Problem Statement, Approach,
> Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions) because
> `specs/` carries no template at authoring time and this is not a hardening implementation spec.
> It deliberately does **not** copy the foundation-pack docs' narrative house style.

**Status:** PROPOSED. Awaiting owner approval to enact via the reassess / ticket process.

**Admissibility posture:** `P0-CERT not applicable`. This is a doctrine-alignment proposal; it
certifies no code, performs no gate audit, and asserts nothing about whether live crates, fixtures,
CI, or goldens already satisfy the amended reference doctrine.

**Authority:** A spec is subordinate to every higher tier and the reference layer it amends; it
**may not redefine `INV-111`, `EMERGE-OBS`, gate semantics, proof mechanics, or acceptance
doctrine** (`docs/4-specs/SPEC_LEDGER.md`, `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`).
This spec does not exercise that forbidden authority. It **operationalizes higher-tier doctrine** —
the emergence-evidence invariant `INV-111` (`docs/0-foundation/02`, ratified by archived spec `0026`),
the architecture observer-only-evidence and typed-observability contracts (`docs/1-architecture/13`,
ratified by `0027`), and the execution evidence-honesty / anti-vacuity / `EMERGE-OBS` mechanics
(`docs/2-execution/10`, `00`, ratified by `0028` D7/D8/D9) — by staging *additive* reference-tier
naming and risk-memory changes that point **upward** to those sources. No deliverable defines a
gate, coins a new gate code / observation-obligation code / invariant number / risk identifier, or
restates an execution rule as a local reference-tier definition. Once authored, the final wording is
reference-tier guidance and this spec becomes historical provenance.

**Provenance:** derived from `reports/reference-tier-alignment-research-report.md` (external deep
research, pinned to commit `36b40823fb07752987531ecd142c78505b8f56da` = current `HEAD` `36b4082`)
and the shared brief `reports/reference-and-specs-tier-alignment-research-brief.md`. The report is
the planned `docs/3-reference/*` session that follows the foundation (`0026`), architecture (`0027`),
and execution (`0028`) alignments and the routing backlog
`reports/foundation-amendment-lower-tier-routing.md`. The report's verdict-critical reference-doc
coverage claims were independently re-verified against live `HEAD` during authoring (see Verification
§V1); per the repo's standing note on externally-researched specs, the report's named symbols are
trusted while any `#Lxxxx`/line-range anchors are ignored.

---

## 1. Problem Statement

Foundation `INV-111` (ratified, `0026`), the architecture `13` observer-only emergence-evidence +
typed-observability contract (ratified, `0027`), and the execution `10`/`00` proof mechanics
(ratified, `0028` D7/D8/D9 — `EMERGE-OBS` realignment, the general anti-vacuity / behavior-witness
standard, and the evidence-status / fingerprint-scope honesty rule) have moved upstream doctrine
ahead of what the **reference tier** yet records. The reference tier is **not fundamentally
misaligned** — the glossary already controls the truth/observation/belief/evidence/story-sifting
vocabulary, and the risk register already owns the acceptance-evidence failure modes (R-26, R-27,
R-28, R-29) and the observation-becomes-direction mode (R-22). The gaps are concentrated and
specific:

1. **The glossary has no canonical term for observer-only emergence evidence.** `INV-111` and the
   `0026`/`0027`/`0028` cascade now route a glossary term to `docs/3-reference/02_GLOSSARY.md`
   (`0026` D4 explicitly), but a search of the live glossary finds no `emergence`, no
   `observer-only`, and no `EMERGE-OBS` entry. The glossary already distinguishes `evidence`,
   `observation`, `projection`, and `story sifting`, so the artifact class has no canonical name and
   no upward cross-reference, leaving the term free to be collapsed into one of those neighbors
   during later implementation.

2. **The risk register's acceptance-evidence cluster predates the `0028` evidence-honesty
   vocabulary.** R-27 (reachability overstatement), R-28 (incomplete correction closure, incl. the
   fingerprint-payload pitfall), and R-29 (guard vacuity / decorative locks) already own the right
   failure modes, and R-26 owns archive-as-certification, but the cluster does not yet speak the six
   post-`0028` distinctions explicitly enough for reviewers to apply execution `10` reliably:
   *pending ≠ pass*, *sampled ≠ certifying*, *observer-only ≠ gate*, *byte-fingerprint ≠ semantic
   proof*, *archive/history ≠ certification*, *artifact-presence ≠ behavior-witness*. The register
   also carries no compact watch note for the specific anti-Goodhart relapse in which observer-only
   emergence counters become simulation objectives.

Neither gap is a foundation, architecture, or execution hole — those tiers state the doctrine, own
the data-flow contracts, and define the proof mechanics, and they are already aligned. Each is a
**reference-tier memory** gap: reference owns compact naming and risk memory and must now name the
artifact and record the relapse patterns reviewers are likely to miss. The reference-tier alignment
report dispositioned the items: two warrant reference amendments (F01, F02), the reference
index/review checklist needs no required change (boundary-awareness), and the possession-bind
perception question (F04) routes **out** to an owner decision.

## 2. Approach

Stage two additive reference-tier changes, each landing in the reference doc that *owns* the memory,
with upward cross-references to the governing sources. Keep every change at the reference altitude —
*one canonical name and the compact relapse memory reviewers need* — without restating, amending, or
locally redefining `INV-111`, the architecture observer-only-evidence contract, the execution
`EMERGE-OBS` mechanics, gate semantics, or the execution `10` evidence-honesty rule. The glossary
gets the canonical term plus an `EMERGE-OBS` lookup cross-reference; the risk register gets a compact
realignment threaded through the **existing** R-26/R-27/R-28/R-29/R-22/R-16 entries plus one watch
note — explicitly **no new `R-##`**.

Coin **no** new gate codes, observation-obligation codes, invariant numbers, or risk identifiers.
Cross-reference upward (foundation → architecture → execution), never sideways or downward, and never
turn a reference entry into the authority that defines a gate or proof. The possession-bind
perception question (F04) routes **out** to an owner decision (Out of Scope §6). Final reference
wording is authored on enactment (by ticket / reassess), not in this spec.

## 3. Deliverables

Both deliverables are **proposed amendment substance**, to be authored into the named reference
file(s) by the reassess / ticket process. Neither coins a new gate/obligation code, invariant number,
risk identifier, threshold, or local gate definition; those stay settled at their owning tier.

| # | Report ID | Target file(s) | Kind | Substance |
|---|---|---|---|---|
| D1 | F01 | `docs/3-reference/02_GLOSSARY.md` (xref up to foundation `02`/`09`/`12`, architecture `13`, execution `00`/`10`) | Addition (canonical term) | Add **observer-only emergence evidence** as the canonical term for the after-the-fact artifact (the glossary class name for architecture `13`'s **observer-only emergence-evidence record**, so the tiers do not fork the name) used to document emergent outcomes under `INV-111`, placed in the glossary's `Canonical context-bound and deferred terms` table near `Evidence` and `Proof` (the term is itself context-bound/retrospective; `story sifting` sits one table up among the plain `Canonical terms`), and kept distinguishable from `evidence`, `observation`, `projection`, and `story sifting`. The entry must carry the five required properties — **retrospective / after-the-fact** (produced from completed event-log/replay ancestry, not planned as a dramatic target), **non-certifying** (acceptance context, never sufficient alone to pass behavior gates), **ancestry-backed** (tied to causal event-log ancestry, replay/extraction provenance, and the phenomenon-family row contract), **non-steering** (never feeds cognition, scheduler, validators, authoring, seed/scenario selection, pacing, LOD, difficulty, or outcome gates), and **structurally non-input** (the simulation must not read it as state, planner data, content-selection data, or pass/fail threshold). Cross-reference **upward only**. Explicitly distinguish it from the neighbors it must not be collapsed into: it is not ordinary `evidence` (which supports many holder claims), not `observation` (which can be diegetic), not `projection` (a derived read model), and not `story sifting` (pattern surfacing). Add a compact **`EMERGE-OBS`** glossary entry as an *execution-label lookup only* — it names the execution-tier observation-obligation label for these records and points to execution `00`/`10` for semantics; it defines no gate. Preserves the glossary's naming-only authority boundary (no doctrine, no gate definition). |
| D2 | F02 | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (xref to execution `10`; cross-link R-26) | Addition (cluster realignment) | A compact realignment of the **existing** R-27/R-28/R-29 acceptance-evidence cluster to the six post-`0028` evidence-honesty distinctions, cross-referencing execution `10` for the rule (never restating it as a local definition). **R-27** (reachability overstatement) explicitly watches the first three status mistakes: *pending counted as pass*, *sampled described as exhaustive certification*, *observer-only (incl. `EMERGE-OBS`) treated as a gate / behavior certificate*. **R-28** (incomplete correction closure) expands its fingerprint-payload pitfall into the execution scope taxonomy — *raw bytes / normalized serialization / parsed semantic content / command transcript / run seed / replay artifact* — with the memory that a fingerprint proves only the scope it covers (a byte hash is not semantic proof; a parsed-content hash is not raw-byte stability; a transcript hash is not replay ancestry; a run seed is not a behavior witness). **R-29** (guard vacuity / decorative locks) extends its symptom list to include status rows, ledgers, checksums, template tables, `EMERGE-OBS` rows, archived spec/report references, and fixture artifacts that exist without behavior witnesses; mitigation stays behavioral (synthetic negative, live path-under-test proof, or a scoped reason no negative can exist). **R-26** (archive ≠ certification) is cross-linked where useful. Add **one anti-Goodhart watch note** under the cluster pointing to R-22 (observation-becomes-direction), R-16 (no-human proof pressure), R-27 (evidence overstatement), and execution `10` (the non-input rule), naming the relapse where counters / phenomenon families / story-sifted rows / emergence ledgers become seed selectors, scheduler inputs, scenario objectives, pacing knobs, difficulty targets, LOD inputs, or pass/fail thresholds. **Invent no new `R-##`** — if maintainers later judge the relapse common enough for its own entry, that happens through the repo's reassess/amend process, not this spec. |

D1 and D2 are additive reference-tier memory; both are consistent with the glossary's and register's
existing authority boundaries and with foundation/architecture/execution doctrine. Neither weakens a
lock or defines a gate.

## 4. Invariants Alignment

| Invariant / doctrine | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-111` — observer-only emergence evidence | aligns (names + guards) | D1 gives the artifact class one canonical name carrying its retrospective / non-certifying / ancestry-backed / non-steering / non-input properties at the terminology surface; D2's anti-Goodhart watch note records the relapse where that evidence becomes a steering input, pointing to execution `10`'s non-input rule. Neither defines new emergence semantics — both cross-reference upward. |
| `INV-097` no-script compliance is tested / `INV-098` feature acceptance is harsh | aligns (supports review) | D2 makes the six evidence-honesty distinctions (pending/sampled/observer-only/byte-fingerprint/archive/artifact-presence) explicit reviewer memory at the risk-register surface, so pending / sampled / byte-stable / archive evidence cannot be silently mislabeled as certification during acceptance review. |
| Truth firewall — truth may validate, not plan (`INV-099`; `docs/0-foundation/14`) | aligns (preserved) | D1's non-steering / structurally-non-input properties and D2's watch note keep observer-only emergence evidence on the observe/report side of the firewall at the naming and risk-memory surfaces; no reference entry grants any artifact planning authority. |
| `INV-006` possession transfers no knowledge / `INV-087` human-focus is not privilege | N/A here (preserved) | This spec does **not** decide whether possession bind counts as modeled perception; F04 is preserved as a forward-routed owner decision (§6), no silent edit at the reference tier. |

No invariant is weakened or tensioned. Both deliverables are additive/clarifying reference-tier
memory; no foundation principle, architecture contract, or execution rule is re-decided, and no new
code/obligation/invariant/risk identifier is coined.

## 5. Verification

- **V1 — Coverage gaps confirmed against live `HEAD` (done at authoring).** Verdict-critical
  reference-doc claims were re-verified on `HEAD` `36b4082` (the report's pinned commit), ignoring any
  fabricated line anchors: `docs/3-reference/02_GLOSSARY.md` contains distinct `evidence`,
  `observation`, `projection`, and `story sifting` entries but **no** `emergence` / `observer-only` /
  `EMERGE-OBS` entry (F01 real); `docs/3-reference/01_DESIGN_RISK_REGISTER.md` carries R-26, R-27,
  R-28 (incl. the fingerprint-payload pitfall), R-29, R-22, and R-16, but none yet speaks the six
  post-`0028` evidence-honesty distinctions explicitly and there is no anti-Goodhart emergence
  watch note (F02 real); the glossary and register both already declare a naming-/risk-memory-only
  authority boundary that forbids local gate definitions (the altitude D1/D2 must respect).
- **V2 — Enactment acceptance (on implementation).** Each deliverable is accepted only when its
  substance is authored into the named reference file as compact naming / risk memory (not a doctrine
  restatement), every cross-reference points **upward** to the owning tier, no execution rule is
  restated as a local reference-tier definition, and no new gate code / observation-obligation code /
  invariant number / `R-##` identifier is coined.
- **V3 — Boundary check.** Confirm enactment introduces no foundation, architecture, or execution
  edit and re-decides no doctrine, contract, or gate; the only files touched are
  `docs/3-reference/02_GLOSSARY.md` and `docs/3-reference/01_DESIGN_RISK_REGISTER.md`. The reference
  index/review checklist (`docs/3-reference/00`) needs no required change (§6).
- **V4 — Forward-routing follow-through (other sessions, not this spec).** The specs-tier
  acceptance-artifact template fields (F03, `docs/4-specs/0003`) are enacted by their own session
  (sibling spec `0030`); the possession-bind perception owner decision (F04) is enacted after the
  owner decides. This spec only records the hand-offs (§6).

## 6. Out of Scope

- **Final reference wording, glossary entry phrasing, risk-entry symptom lists, status/scope token
  spellings.** Owned by the reassess/ticket enactment within the reference layer's existing style.
- **Reference index / review checklist (`docs/3-reference/00`) — boundary-awareness, no required
  change.** The index already owns tier boundaries, source discipline, and checklist routing. A
  single optional future checklist question (confirm a review packet labels evidence status and
  fingerprint scope and does not count pending/sampled/observer-only/historical artifacts as pass
  evidence) would be a pointer to execution `10`, not a local restatement; because this pass already
  amends the register, the index can remain unchanged unless reviewers repeatedly miss the honesty
  check. Its absence from §3 is deliberate, not an oversight.
- **A new `R-##` for anti-Goodhart emergence pressure.** Folded into the R-22/R-27/R-16/R-29 cluster
  as a watch note (D2). Escalate to a dedicated risk only if future review work shows the failure mode
  recurring independently — through the repo's reassess/amend process, not this spec.
- **`EMERGE-OBS` as a new reference-tier gate definition.** D1 adds it only as an execution-label
  lookup cross-reference; the gate/obligation semantics remain owned by execution `00`/`10`.
- **F03 — acceptance-artifact template fields → `docs/4-specs/0003`.** Specs-tier work; owned by the
  sibling spec `0030`, not this reference-tier pass.
- **F04 — possession-bind perception (deferred owner decision; do NOT silently amend).** Whether
  possession *binding* itself counts as modeled perception is an owner/product-architecture decision,
  not a reference-tier editorial matter. This spec touches no glossary or risk entry to pre-decide it.
- **Foundation, architecture, and execution edits.** `INV-111` (ratified, `0026`), the architecture
  contracts (ratified, `0027`), and the execution mechanics (ratified, `0028`) are upstream and
  unchanged; this spec edits no `docs/0-foundation/*`, `docs/1-architecture/*`, or
  `docs/2-execution/*` file.
- **Code certification.** No assertion that crates, fixtures, CI, or goldens already satisfy the
  amended reference doctrine; certification is later work.
- **New world mechanics, Phase-4 expansion, LLM surfaces, crate/code changes.**

## 7. Risks & Open Questions

- **R-A — Reference enactment requires owner approval.** Authoring D1–D2 edits tier-3 reference
  guidance. Lighter than constitutional or execution sign-off, but it must not proceed by convention;
  this spec stages the substance, it does not authorize the edits.
- **R-B — Glossary over-reach on D1.** The new term must stay naming-only: it names the artifact and
  its five properties and points upward, but it must not restate the full `INV-111` doctrine or
  define `EMERGE-OBS` gate semantics. The maintenance rule of `02_GLOSSARY.md` (do not smuggle
  doctrine/architecture/gate semantics into reference) governs the final wording.
- **R-C — `EMERGE-OBS` glossary entry scope (open question).** Recommendation: add it, but only as a
  compact execution-label cross-reference. The canonical conceptual term is **observer-only emergence
  evidence**; `EMERGE-OBS` stays an execution observation-obligation label, not a new reference-tier
  gate definition. (Report open question 1.)
- **R-D — Reference index honesty checklist (open question).** Recommendation: optional. The index is
  intentionally small; the risk register is the better primary home. Add a single pointer only if
  reviewers repeatedly miss evidence-status / fingerprint-scope honesty. (Report open question 2.)
- **R-E — Anti-Goodhart escalation (open question).** Recommendation: not in this pass — fold into
  R-22/R-27/R-16/R-29. Escalate to a new risk only if future review work shows the failure mode
  recurring independently. (Report open question 3.)
- **R-F — Possession-bind perception (deferred owner decision; do NOT silently amend).** Preserved as
  F04. D1/D2 do not touch it; if the owner decides, the home is execution `07` possession proof, and
  any bind perception must be a modeled event/channel for the actor, never a human/player knowledge
  transfer (`INV-006`/`INV-108`).

## 8. Provenance & Source Discipline

- The source report is pinned to `36b40823fb07752987531ecd142c78505b8f56da` and was re-verified
  against that same live `HEAD` `36b4082`; no intervening commit drift applies.
- Commit hashes named here are audit/spec provenance only.
- The report's external proof-methodology references (its §9 — testing-oracle, metamorphic,
  property-based, mutation, approval/golden, deterministic-simulation, structured-observability,
  story-sifting literature) are research support that sharpened the F01/F02 recommendations; they are
  not Tracewake authority and replace no doctrine.
- This spec adds **no** `docs/4-specs/SPEC_LEDGER.md` row at proposal time, per the staged-spec
  convention (the `0026`/`0027`/`0028`/hardening-series precedent: the ledger row lands at
  acceptance/closeout, not at proposal).
