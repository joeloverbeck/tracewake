# Spec 0030 — Specs-Tier Doctrine-Alignment Amendment

This spec **proposes an amendment to the specs-tier acceptance-artifact template**
(`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`). It is a design/proposal artifact: it
specifies the *substance and home* of the template fields so Tracewake's reassess / ticket process
can author the final template wording. **It does not itself author the ratified template prose, and
it coins no new gate, obligation, or identifier.** Specs are the lowest tier (they operationalize
higher-tier doctrine into review packets but may not restate, amend, weaken, or supersede it), so
enactment requires ordinary owner approval rather than the constitutional sign-off a foundation
amendment demands (cf. `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`,
`archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`,
`archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`, and the sibling reference-tier
amendment `archive/specs/0029_REFERENCE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`).

> Section set: this file uses the canonical `specs/` section set (Problem Statement, Approach,
> Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions) because
> `specs/` carries no template at authoring time and this is not a hardening implementation spec.
> It deliberately does **not** copy the foundation-pack docs' narrative house style.

**Status**: COMPLETED

**Admissibility posture:** `P0-CERT not applicable`. This is a doctrine-alignment proposal; it
certifies no code, performs no gate audit, and asserts nothing about whether live crates, fixtures,
CI, or goldens already satisfy the amended template.

**Authority:** A spec is subordinate to every higher tier; `docs/4-specs/0003` is a **review-packet
template** whose job is to make evidence honest enough for reviewers to apply execution `10`, **not**
to define what `EMERGE-OBS`, `P0-CERT`, gate passage, replay proof, or acceptance doctrine mean
(`docs/4-specs/README.md`, `docs/4-specs/SPEC_LEDGER.md`). This spec does not exercise that forbidden
authority. It **operationalizes higher-tier doctrine** — the execution evidence-honesty rule
(`docs/2-execution/10`, ratified by `0028` D9), which itself realizes the architecture observability
contract (`docs/1-architecture/13`, ratified by `0027`) and the foundation `INV-111` emergence
posture (`docs/0-foundation/02`, ratified by `0026`) — by staging *additive* packet-structure fields
that point to execution `10` for the rule. No deliverable defines a gate, coins a new gate code /
observation-obligation code / invariant number / status token, or weakens the template's existing
anti-overclaiming posture. Once authored, the final wording is specs-tier template guidance and this
spec becomes historical provenance.

**Provenance:** derived from `reports/specs-tier-alignment-research-report.md` (external deep
research, pinned to commit `36b40823fb07752987531ecd142c78505b8f56da` — the then-current `HEAD`, now an
ancestor of `HEAD` `5e053f2`)
and the shared brief `reports/reference-and-specs-tier-alignment-research-brief.md`. The report is
the planned `docs/4-specs/*` session that follows the foundation (`0026`), architecture (`0027`),
and execution (`0028`) alignments and the routing backlog
`reports/foundation-amendment-lower-tier-routing.md`. The report's verdict-critical template
coverage claims were independently re-verified against live `HEAD` during authoring (see Verification
§V1); per the repo's standing note on externally-researched specs, the report's named symbols are
trusted while any `#Lxxxx`/line-range anchors are ignored.

---

## 1. Problem Statement

Execution `10` is now the governing source for review-artifact honesty (ratified through `0028` D9):
it requires every review packet to label evidence by **status** and **fingerprint scope**, rejects
silently overcounting pending / sampled / observer-only / historical evidence as a pass, and requires
behavior witnesses and replay/provenance ancestry where a packet claims path-under-test behavior.
`0028` D9's "wait until execution doctrine exists" condition is now satisfied, so the template change
is **actionable now, not deferred**.

The specs tier is **not fundamentally misaligned** — `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
already requires the right skeleton: exact commit under test, gates run with command/result/output,
changed files, per-requirement evidence, residual convention-only items, scoped-certification wording
with an explicit forbidden-wording list, and reviewer sign-off. The gap is concentrated and specific:

1. **The per-requirement evidence table is too thin for post-`0028` evidence honesty.** It has only
   four columns — requirement, responsible layer, evidence, result — with bare `Evidence` and
   `Result` cells. It does not require evidence **status** (pass/fail vs pending/sampled/observer-only/
   historical), **fingerprint scope** (raw bytes / normalized serialization / parsed semantic content
   / command transcript / run seed / replay artifact), **sampling/exhaustiveness** scope,
   **observer-only / `EMERGE-OBS`** labeling, **pending** blockers, **historical/archive** labeling,
   **path-under-test behavior witnesses**, or **replay/provenance ancestry**. The template has the
   right anti-overclaiming posture but lacks the fields needed to prevent evidence from being
   silently overclaimed.

This is not a foundation, architecture, or execution hole — those tiers state the doctrine, own the
observability contract, and define the evidence-honesty rule, and they are already aligned (`0028` D9
owns the execution rule; sibling spec `0029` preserves the reference-tier risk-memory and glossary
term in `archive/specs/0029_REFERENCE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`). It is a
**specs-tier packet-structure** gap: the template must carry the fields that let a
reviewer apply execution `10`. The specs-tier alignment report dispositioned the items: one warrants
a template amendment (F03); the `SPEC_LEDGER`, specs `README`, and `0001` ontology need no change
(boundary-awareness); and the possession-bind perception question (F04) and any future certification
implementation spec route **forward**.

## 2. Approach

Stage one additive template amendment in `docs/4-specs/0003`: per-requirement evidence-honesty fields
written as **packet-structure requirements that point to execution `10`**, never as new doctrine, new
gates, or new identifiers. The fields enrich the existing per-requirement table and/or a companion
evidence-item ledger so that evidence status and fingerprint scope become mandatory, sampled/
observer-only/pending/historical evidence can never be silently counted as a pass, behavioral passes
must carry a path-under-test witness, and derived/behavioral claims must carry replay/provenance
ancestry. Preserve every existing template section — exact commit, gates run, changed files, residual
convention-only items, scoped-certification wording, and the forbidden-wording list — and strengthen
evidence fields **without loosening** the template's existing anti-overclaiming posture.

Coin **no** new gate codes, observation-obligation codes, invariant numbers, or status tokens beyond
what execution `10` already names. Keep `EMERGE-OBS` as an evidence item with `status =
observer-only`, not a new gate. The possession-bind perception question (F04) and any future P0
certification / code-enactment spec route **forward** (Out of Scope §6). Final template wording —
exact column headings, ledger layout, token spellings — is authored on enactment (by ticket /
reassess), not in this spec.

## 3. Deliverables

The deliverable is **proposed amendment substance**, to be authored into
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` by the reassess / ticket process. It coins no new
gate/obligation code, invariant number, status token, threshold, or schema field beyond execution
`10`'s existing vocabulary; the exact table headings and wording are settled at enactment.

| # | Report ID | Target file | Kind | Substance |
|---|---|---|---|---|
| D1 | F03 | `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (points to execution `10`; observer-only ties to `EMERGE-OBS`) | Addition (evidence-honesty fields) | Add per-evidence-item honesty fields, structured per the eight points below, to the per-requirement table and/or a companion evidence-item ledger referenced by per-requirement rows. The fields make evidence status and fingerprint scope mandatory, forbid silent overcounting, and require behavior witnesses and replay/provenance ancestry for behavioral claims — all pointing to execution `10` for the rule, defining no doctrine. |

D1's substance, expanded as the eight packet-structure points the enactment must satisfy (point order
and grouping settled at authoring; no point coins a gate or identifier):

1. **Evidence status per item**, using execution `10`'s vocabulary — *pass/fail* where the evidence
   actually certifies the requirement; *pending* where proof is not yet present; *sampled* where
   representative but not exhaustive; *observer-only* where it can inform review but cannot certify
   behavior; *historical* where archive/spec/report evidence supplies context, not current
   certification. The template must state that pending / sampled / observer-only / historical evidence
   **cannot be silently counted as a pass**.
2. **Fingerprint scope** for every hash/checksum/snapshot claim, using the execution taxonomy — *raw
   bytes / normalized serialization / parsed semantic content / command transcript / run seed /
   replay artifact* — with the rule that a fingerprint must not be cited beyond its actual scope (a
   raw-byte hash is not semantic proof; a normalized-serialization hash is not raw-file equality; a
   command transcript is not replay ancestry; a run seed is not behavior certification).
3. **Path-under-test behavior-witness fields** for any claimed behavioral pass: the path under test;
   the command / event / trigger / emitter / scheduler entry that exercised it; the responsible
   layer; the accepted/rejected action or validation stage witnessed; the live negative /
   mutation-style failure / reason a negative is not applicable; and the checked facts or invariants
   the witness actually supports.
4. **Replay/provenance ancestry fields**: the relevant event-log segment or event identifiers; the
   replay artifact or serialized-log reference; seed/randomness/content version where applicable; the
   extraction/projection version for derived evidence; and source provenance for any claim crossing
   from artifact to semantic behavior.
5. **Sampling/exhaustiveness scope**: a sampled result states population, sample basis, omitted cases,
   and why the sample is representative; exhaustive evidence states what perimeter it exhausts.
6. **Observer-only handling**: `EMERGE-OBS` and observer-only emergence evidence are labelable as
   `status = observer-only`. The template must not allow such rows to become certification, gate
   pass/fail thresholds, scheduler objectives, scenario goals, or code-quality substitutes unless a
   future upstream spec explicitly changes the doctrine.
7. **Pending and historical handling**: pending rows name the missing proof and owner/blocker;
   historical rows identify whether the artifact is context, lineage, or archived precedent (an
   optional source-type field may distinguish archive/spec/report lineage) and are not counted as
   current certification.
8. **Requirement result computed only from certifying evidence**: a row may carry useful evidence
   while the requirement remains pending or failed. Preserve the existing scoped-certification and
   residual-convention sections and the forbidden-wording list; the amendment strengthens evidence
   fields without loosening the existing anti-overclaiming posture.

D1 is a single additive template amendment consistent with the template's existing anti-overclaiming
posture and with foundation/architecture/execution doctrine; it weakens no section and defines no
gate.

## 4. Invariants Alignment

| Invariant / doctrine | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-098` — feature acceptance is harsh / `INV-097` no-script compliance is tested | aligns (proves honestly) | D1 makes evidence status and fingerprint scope mandatory and forbids counting pending / sampled / observer-only / historical evidence as a pass at the review-packet surface, so the acceptance artifact cannot overstate what was certified. |
| `INV-111` — observer-only emergence evidence | aligns (preserved) | D1 point 6 labels `EMERGE-OBS` / observer-only emergence evidence as `status = observer-only` and bars it from becoming certification, a gate threshold, or a scheduler/scenario objective at the packet surface — preserving its non-certifying, non-steering posture without redefining it. |
| Event-sourced causality / deterministic replay (`docs/0-foundation/03`; `INV-001…`) | aligns (proves) | D1 point 4 requires replay/provenance ancestry (event-log segment, replay artifact, seed/content version, extraction/projection version) for behavioral and derived claims, so a packet's behavior claim is backed by replayable causal evidence rather than artifact presence. |
| Truth firewall — truth may validate, not plan (`INV-099`; `docs/0-foundation/14`) | aligns (preserved) | D1 point 3's path-under-test behavior witnesses name the responsible layer, source event/proposal/context, and accepted/rejected stage — keeping acceptance evidence inside the typed, validated regime; no field grants a presentation/LLM/debug surface cognition or certification authority. |
| `INV-006` possession transfers no knowledge / `INV-087` human-focus is not privilege | N/A here (preserved) | This spec does **not** decide whether possession bind counts as modeled perception; F04 is preserved as a forward-routed owner decision (§6), no silent template edit. |

No invariant is weakened or tensioned. The deliverable is an additive/clarifying specs-tier
packet-structure amendment; no foundation principle, architecture contract, or execution rule is
re-decided, and no new code/obligation/invariant/status identifier is coined.

## 5. Verification

- **V1 — Coverage gap confirmed against live `HEAD` (done at authoring).** The verdict-critical
  template claim was re-verified on `HEAD` `36b4082` (the report's pinned commit), ignoring any
  fabricated line anchors: `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` carries the exact
  commit / gates-run / changed-files / per-requirement-evidence / residual-convention /
  scoped-certification-wording / forbidden-wording skeleton, and its per-requirement table has
  exactly four columns (requirement, responsible layer, evidence, result) with bare `Evidence` and
  `Result` cells — no evidence status, fingerprint scope, sampling/exhaustiveness, observer-only,
  pending, historical, behavior-witness, or replay/provenance fields (F03 real). Execution `10`
  (ratified, `0028` D9) supplies the status and fingerprint-scope rule the template must point to.
  Re-confirmed at reassessment time against current `HEAD` `5e053f2`: `0003` is byte-identical to its
  state at the pinned commit (`git diff 36b4082 -- docs/4-specs/0003` empty), so the coverage gap is
  unaffected by the commits that advanced `HEAD` past the pin.
- **V2 — Enactment acceptance (on implementation).** The deliverable is accepted only when its
  substance is authored into `0003` as packet-structure requirements that point to execution `10`
  (not a doctrine restatement), the existing scoped-certification / residual-convention / forbidden-
  wording sections are preserved, `EMERGE-OBS` remains an `observer-only` evidence item rather than a
  gate, and no new gate code / observation-obligation code / invariant number / status token beyond
  execution `10` is coined.
- **V3 — Boundary check.** Confirm enactment introduces no foundation, architecture, or execution
  edit and re-decides no doctrine or gate; the only file touched is
  `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`. No `SPEC_LEDGER`, specs `README`, or `0001`
  edit is required by this pass (§6).
- **V4 — Forward-routing follow-through (other sessions, not this spec).** The reference-tier
  glossary term and risk-register realignment (F01/F02) are enacted by the sibling spec `0029`; the
  possession-bind perception owner decision (F04) and any future P0 certification / code-enactment
  spec are enacted after the owner / maintainers decide. This spec only records the hand-offs (§6).

## 6. Out of Scope

- **Final template wording, column headings, evidence-item-ledger layout, status/scope token
  spellings.** Owned by the reassess/ticket enactment within the template's existing style. Open
  question: prefer a companion evidence-item ledger referenced by per-requirement rows over a single
  wide grid (report open question 1) — settled at authoring.
- **`SPEC_LEDGER.md` — boundary-awareness, no change.** It already records `0026`/`0027`/`0028` and
  already leaves reference/spec-template follow-through and F04 as later work; this report is not a
  numbered spec to add. (This spec `0030` and its sibling `0029` add their own ledger rows at
  acceptance/closeout, per the staged-spec convention — §8 — not at proposal.)
- **Specs `README.md` — boundary-awareness, no change.** It already states specs are the lowest tier
  and lists `0003` as the scoped acceptance-artifact template; no doctrine drift found.
- **`0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — boundary-awareness, no
  change.** It remains a subordinate first-proof ontology/fixture package relying on execution `09` /
  architecture `13` for proof semantics. Future-facing caveat only: if `0001` later serves as
  evidence in a scoped acceptance packet, that packet labels the evidence's status and scope under
  the amended `0003`; `0001` itself needs no edit to anticipate that.
- **F01/F02 — reference-tier glossary term and risk-register realignment → `docs/3-reference/*`.**
  Reference-tier work; owned by the sibling spec `0029`, not this specs-tier pass.
- **F04 — possession-bind perception (deferred owner decision; do NOT silently amend).** Whether
  possession *binding* counts as modeled perception is an owner/product-architecture decision, not a
  template-edit decision. This spec adds no template field to pre-decide it.
- **Future implementation-spec or certification package.** If maintainers later create a P0
  certification or code-enactment spec, it should adopt the `0003` evidence-status / fingerprint-scope
  fields proposed here. This spec recommends the fields but does not author that implementation spec.
- **Foundation, architecture, and execution edits.** `INV-111` (ratified, `0026`), the architecture
  contracts (ratified, `0027`), and the execution evidence-honesty rule (ratified, `0028` D9) are
  upstream and unchanged; this spec edits no `docs/0-foundation/*`, `docs/1-architecture/*`, or
  `docs/2-execution/*` file.
- **Code certification.** No assertion that crates, fixtures, CI, or goldens already satisfy the
  amended template; certification is later work.

## 7. Risks & Open Questions

- **R-A — Specs-tier enactment requires owner approval.** Authoring D1 edits the tier-4 template.
  Lighter than constitutional or execution sign-off, but it must not proceed by convention; this spec
  stages the substance, it does not authorize the edit.
- **R-B — Template must not become doctrine.** The fields must point to execution `10`, never restate
  or redefine the evidence-honesty rule, `EMERGE-OBS` semantics, gate passage, or replay proof. The
  template's job is to make a reviewer able to *apply* execution `10`, not to own it.
- **R-C — Ledger vs single table (open question).** Recommendation: a companion evidence-item ledger
  referenced by per-requirement rows, to avoid a cluttered grid and let multiple requirements cite the
  same evidence item without duplicating status/fingerprint/provenance fields. (Report open
  question 1.)
- **R-D — `EMERGE-OBS` status vs dedicated gate (open question).** Recommendation: use `observer-only`
  as the status and `EMERGE-OBS` as the execution label/source obligation, keeping the status taxonomy
  small and avoiding a fake gate. (Report open question 2.)
- **R-E — Historical-evidence categorization (open question).** Recommendation: archived specs and
  prior reports are all `historical` for status purposes; a separate source-type field may distinguish
  archive/spec/report lineage, but none is current certification unless the packet turns it into live
  certifying evidence. (Report open question 3.)
- **R-F — Possession-bind perception (deferred owner decision; do NOT silently amend).** Preserved as
  F04. D1 adds no field to pre-decide it; if the owner decides, the home is execution `07` possession
  proof, and any bind perception must be a modeled event/channel for the actor, never a human/player
  knowledge transfer (`INV-006`/`INV-108`).

## 8. Provenance & Source Discipline

- The source report is pinned to `36b40823fb07752987531ecd142c78505b8f56da` and was re-verified
  against that same live `HEAD` `36b4082` at authoring. Subsequent commits (the `0029` reference-tier
  archival, PR #37) advanced `HEAD` to `5e053f2`, but `docs/4-specs/0003` is byte-identical since the
  pin (`git diff 36b4082 -- docs/4-specs/0003` empty), so no drift affects the V1 coverage claim.
- Commit hashes named here are audit/spec provenance only.
- The report's external proof-methodology references (its §10 — testing-oracle, metamorphic,
  property-based, mutation, approval/golden, deterministic-simulation, structured-observability
  literature) are research support that sharpened the F03 field recommendations; they are not
  Tracewake authority and replace no doctrine.
- This spec adds **no** `docs/4-specs/SPEC_LEDGER.md` row at proposal time, per the staged-spec
  convention (the `0026`/`0027`/`0028`/hardening-series precedent: the ledger row lands at
  acceptance/closeout, not at proposal).

## Outcome

Completed: 2026-06-14

Spec 0030 was enacted by `0030SPETIEDOC-001`, which amended
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` with a companion evidence
item ledger and mandatory per-evidence-item honesty fields. The enacted fields
make evidence status, fingerprint scope, path-under-test behavior witnesses,
replay/provenance ancestry, sampling/exhaustiveness scope, pending/historical
handling, and certification use visible in acceptance packets while pointing to
execution `10` for the governing evidence-honesty rule.

The owner-approval precondition in §R-A was satisfied by the explicit user
request to implement the `0030SPETIEDOC` series. No foundation, architecture,
execution, crate, fixture, specs `README`, or `0001` ontology edits were made.
`EMERGE-OBS` remains observer-only and non-certifying; the template bars
observer-only rows from becoming certification, gate pass/fail thresholds,
scheduler objectives, scenario goals, or code-quality substitutes unless a
future upstream spec changes that doctrine.

Verification recorded by the ticket included the status-vocabulary,
fingerprint-scope, behavior-witness/provenance, observer-only, and
scoped-certification/forbidden-wording greps against
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, plus manual review against
execution `10` and `INV-097`/`INV-098`/`INV-111`. Final closeout gates are run
after spec archive/truthing edits so the gate evidence covers the final
committed closeout state.

Final closeout verification:

- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
- Archive/reference audits found no active live `specs/0030...` or
  `tickets/0030SPETIEDOC...` references, and the archived ticket/spec both
  carry `## Outcome` and `Completed:`.
