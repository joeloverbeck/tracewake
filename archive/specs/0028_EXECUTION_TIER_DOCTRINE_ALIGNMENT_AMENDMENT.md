# Spec 0028 — Execution-Tier Doctrine-Alignment Amendment

This spec **proposes a set of execution-tier (`docs/2-execution/*`) amendments**. It is a
design/proposal artifact: it specifies the *substance and home* of each amendment so Tracewake's
reassess / ticket process can author the final execution wording. **It does not itself author
ratified execution prose.** Execution is tier-2 doctrine (it owns gate order, certification
sequence, fixtures, audit criteria, and review artifacts) but is not constitutional, so enactment
requires owner approval rather than the constitutional sign-off a foundation amendment demands
(cf. `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md` and the
sibling architecture amendment `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`).

> Section set: this file uses the canonical `specs/` section set (Problem Statement, Approach,
> Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions) because
> `specs/` carries no template at authoring time and this is not a hardening implementation spec.
> It deliberately does **not** copy the foundation-pack docs' narrative house style.

**Status:** COMPLETED. Enacted by the `0028EXETIEDOC` ticket series.

**Admissibility posture:** `P0-CERT not applicable`. This is a doctrine-alignment proposal; it
certifies no code, performs no gate audit, and asserts nothing about whether live crates, fixtures,
CI, or goldens already satisfy the amended execution doctrine.

**Authority:** A spec is subordinate to execution and *may not weaken execution gates or define gate
semantics* (`docs/4-specs/SPEC_LEDGER.md`). This spec does not exercise that forbidden authority. It
**operationalizes higher-tier doctrine** — the truth-firewall family `INV-099…INV-110`
(`docs/0-foundation/02`, `docs/0-foundation/14`), the emergence-evidence invariant `INV-111`
(`docs/0-foundation/02`, ratified by archived spec `0026`), and the architecture contracts those
invariants drove (`docs/1-architecture/03`, `06`, `10`, `04`/`05`/`09`, `13`, ratified by archived
spec `0027`) — by staging *additive* proof obligations and one *editorial reconciliation* so the
execution tier proves that doctrine fully. No deliverable weakens or contradicts an existing
execution gate; the final wording, once authored, is execution-tier doctrine and this spec becomes
historical provenance. No new gate code, observation-obligation code, or invariant number is coined.

**Provenance:** derived from `reports/execution-tier-alignment-research-report.md` (external deep
research, pinned to commit `64a8367ca54f5daf97dac7031a708476d31a3707` = current `HEAD` `64a8367`)
and its brief `reports/execution-tier-alignment-research-brief.md`. The report is the planned
`docs/2-execution/*` session that follows the foundation (`0026`) and architecture (`0027`)
alignments and the routing backlog `reports/foundation-amendment-lower-tier-routing.md`. The
report's verdict-critical execution-doc gap claims were independently re-verified against live
`HEAD` during authoring (see Verification §V1); the report's fabricated `#Lxxxx`/line-range anchors
were ignored in favor of the named symbols.

---

## 1. Problem Statement

The `0006`–`0025` hardening campaign, the foundation rewrite (ratified `INV-099…INV-111`), and the
architecture alignment (`0027`, which translated those invariants into the A03/A06 provenance-
sufficiency and freshness contracts, the A10 embodied-capture contract, the A04/A05/A09 single-charge
accounting seam, and the A13 observer-only emergence-evidence record + typed observability) have
moved upstream doctrine ahead of what the execution tier yet *proves*. The execution tier is **not
fundamentally misaligned** — the hardening campaign already installed much of the proof posture
(`04` provenance minimums, `06` single-tick need accounting, `07` view/possession proof, `08`
content anti-contamination, `00`/`10` `EMERGE-OBS` as a declared non-certifying observer-only
obligation). The gaps are concentrated and specific:

1. **`10` is the primary amendment target.** Its `EMERGE-OBS` mechanism is stale: it states "It
   amends no doctrine," references only older invariants (e.g. `INV-107`), and does not carry the
   architecture-`13` observer-only data contract (run, seed/randomness provenance, controller mode,
   phenomenon family, source-event/causal-chain references, extraction time, review/projection
   version, replay ancestry) or its invalid-pass conditions. `10` also lacks a *general* anti-vacuity
   / typed-behavior-witness rule and an evidence-status / fingerprint-scope honesty rule — the
   "pending is not a pass" language exists only in the mutation section, not as a universal evidence
   standard.

2. **`02` first-proof acceptance does not yet carry observer-only emergence evidence as an
   artifact.** `INV-111` and foundation `12` now require the first proof to include retrospective
   observer-only emergence-evidence records beside the mandatory gates, but `02` reads as if the
   first-proof package can be complete without that artifact.

3. **Provenance sufficiency, memory freshness, believed-access snapshots, and single-charge
   accounting are proven only partially.** `04` has provenance tables but not the architecture-`03`
   shared fail-closed sufficiency rule across actor-known *and* institution-known facts; `04`/`07`
   mention staleness but not the architecture-`06` one-classifier freshness rule across actor /
   no-human / TUI / notebook / institution surfaces; `07` proves the rendered view omits hidden truth
   but not the architecture-`10` observation-time snapshot/carrier sufficiency; `06` proves
   single-tick need accounting but not the architecture-`04`/`05`/`09` named single-owner seam across
   scheduler / action / projection / replay / golden artifacts.

4. **`11` (Phase-4 entry) and `13` (research notes) need future-proofing.** `11` must inherit the
   shared provenance-sufficiency/freshness mechanics before Phase-4 institution behavior expands;
   `13` lacks the proof-method source notes (mutation, metamorphic, property-based, approval/golden,
   deterministic-simulation, structured observability, the test-oracle problem) now load-bearing for
   execution evidence doctrine.

None of these is a foundation or architecture hole — those tiers state the `what` and own the data-
flow contracts. Each is an **execution proof-mechanics** gap: execution owns gates, fixtures, audit
criteria, diagnostics, and review artifacts, and must now prove the upstream doctrine. The
execution-tier alignment report dispositioned nineteen items — eleven warrant execution amendments
(E00–E10), four are already owned (C01–C04), and four route forward to reference / specs / an owner
decision (F01–F04).

## 2. Approach

Stage eleven additive execution proof obligations (ten additions + one editorial vocabulary
reconciliation), each landing in the execution doc that *owns* the proof, with cross-references
where the report identifies a secondary anchor. Keep every change at the proof-obligation level —
*what execution must prove, with what fixtures/negatives/review evidence* — without re-deciding
product identity, architecture contracts, module layout, storage/serialization, or algorithm choice
(those belong to foundation/architecture and are already settled). Preserve every existing gate and
the `EMERGE-OBS` non-certifying / observer-only semantics; this pass strengthens proof, it does not
relax a single lock.

Coin **no** new gate codes, observation-obligation codes, or invariant numbers. Route terminology
(the canonical emergence-evidence glossary term), risk-register wiring, and the acceptance-artifact
template fields **forward** to reference `docs/3-reference/*` and specs `docs/4-specs/0003`, and the
possession-bind perception question **out** to an owner decision (the report's forward-routing
appendix and the routing backlog already enumerate these hand-offs — Out of Scope §6).

The eleven deliverables are the only execution-tier items; the report's other findings are already
owned, routed, or deferred (Out of Scope §6). Final execution wording is authored on enactment (by
ticket / reassess), not in this spec.

## 3. Deliverables

All deliverables are **proposed amendment substance**, to be authored into the named execution
file(s) by the reassess / ticket process. None coins a new gate/obligation code, invariant number,
threshold, ratchet, schema field, fixture name, or command; those are settled at enactment within
execution's existing vocabulary.

| # | Report ID | Target file(s) | Kind | Substance |
|---|---|---|---|---|
| D1 | E00 | `00` (cleanup xrefs in `02`, `03`, `06`, `07`, `08`, `09`, `10`, `11`) | Editorial reconciliation | A vocabulary-reconciliation note in `00` that distinguishes four label classes so a reader cannot mistake one for another: existing **canonical gates** (`P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`); **observation obligations** (`EMERGE-OBS`, non-certifying); **phase-certification artifact labels** (e.g. `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `DATA-CERT`, `FIXTURE-CERT`, `DIAG-CERT`, `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`); and **informal shorthand**. Then apply cross-reference cleanup in the affected docs. Invent no new code and rename no established label; whether to formally map phase labels under canonical gate names is left to a later reassess session (§7). |
| D2 | E01 | `02` (xref `03`, `09`, `10`) | Addition (acceptance package) | An acceptance-package obligation: any first-proof acceptance packet claiming living-world acceptance must include the observer-only emergence-evidence artifact produced under `10`, **beside but outside** the blocking gate list. `03` treats it as an artifact dependency of the relevant phases, **not** a new phase gate; `09` ensures golden/fixture families can supply semantic support where relevant; `10` remains the mechanism owner. Preserves `EMERGE-OBS` non-certifying semantics — the artifact is required-to-be-present, never a pass/fail threshold. |
| D3 | E02 | `04` (evidence support in `10`) | Addition (fail-closed proof) | A provenance-sufficiency proof subsection in `04`: every actor-known **and institution-known** action-relevant fact must have a fact-kind-appropriate provenance route; **missing, empty, dangling, wrong-kind, ambiguous, forbidden-source, or harness-fabricated** provenance must fail closed on the real proposal/validation path under test (not merely be a table entry). `10` review artifacts for any provenance gate must identify the source event/projection/context and include ≥1 live negative — or an explicit reason no live negative can exist. Harness-produced provenance must be real event/projection ancestry, not decorative metadata or fixture labels. |
| D4 | E03 | `04`, `07`, `10` | Addition (one freshness classifier) | A memory-freshness proof obligation applying the architecture-`06` single classifier across all surfaces: only current modeled perception/contact/search may be `observed_now`; older facts stay remembered/believed/stale/contradicted/hearsay/record-derived/unknown, preserving source event, acquisition time, last-verified time, and provenance. `04` owns the actor-known proof; `07` owns the possession/view parity proof; `10` requires review artifacts to show those fields plus a live negative where an old fact stays old despite a later replay, possession bind, debug attach, notebook display, or no-human review. Explicitly prohibit restamping by replay, possession, debug, display, or harness extraction. Same classifier for no-human, TUI, notebook, actor-known, and institution-known surfaces. |
| D5 | E04 | `07` (support in `04`, `10`); carries F04 residual | Addition (observation-time snapshot) | An observation-time snapshot proof obligation in `07`: for any actor-visible action, menu option, possession preflight, or embodied view, evidence must show the holder, the modeled observation/bind/preflight/perception boundary, the captured facts, their provenance/freshness, and the **absence of live-truth handles** in the view-generation path. Add **wallhack negatives** for true-but-unknown routes, workplaces, sleep affordances, container contents, item locations, and routine opportunities. Add an **embodied carrier census** to `10` review artifacts so reviewers see every actor-visible datum's carrier and provenance. Proves capture *sufficiency at formation time*, not merely that the final view omits hidden facts. Does **not** decide whether possession bind itself counts as perception (§7, F04). |
| D6 | E05 | `06` (xref `05`; artifact rule in `09`, `10`) | Addition (single-owner seam) | A single-owner derived-accounting proof in `06`: the only layer permitted to emit need/duration deltas is the action-pipeline/ordinary-life boundary; scheduler, routine planner, projection, replay, and golden-output normalization may *consume* but must not independently charge or reconcile the same tick/window. `05` cross-references proposal/scheduler non-ownership. `09`/`10` add an artifact rule: a golden's byte stability is **insufficient** if the semantic single-charge ledger drifts, double-counts, or omits a duration terminal. Proves the architecture seam; designs no new economy or scheduler. |
| D7 | E06 | `10` | Addition (mechanism realignment) | Realign the `EMERGE-OBS` mechanism to current doctrine: correct the upstream references so it is described as realizing `INV-111`, foundation `09`/`12`, and architecture `13` (remove the stale "amends no doctrine" / `INV-107`-only framing). Add the architecture-`13` data-contract fields to the evidence artifact (run, seed/randomness provenance, controller mode, phenomenon family, source events / causal-chain references, extraction time, review/projection version, replay ancestry). Require rows be extracted retrospectively from actual run events/projections with event-log ancestry — never fabricated by fixtures/debug panels or inserted by the harness. Add invalid-pass examples (evidence feeding cognition/scheduler/validators; evidence used to pick seeds/scenarios/objectives; evidence lacking replay ancestry; rows based only on debug truth or display text). **Preserve** non-certifying semantics: no numeric dramatic-quality pass threshold, no blocking gate without a dedicated future spec. |
| D8 | E07 | `10` (cross-ref hooks in `04`, `09`) | Addition (general anti-vacuity) | A general anti-vacuity / typed-behavior-witness section in `10`: each lock/gate/proof obligation must identify a live negative that would fail if the protected shortcut were reintroduced, or state why none can exist. Artifact-presence checks must pair with behavior witnesses from the path under test; each witness identifies responsible layer, source event/proposal/context IDs, checked facts, accepted/rejected stage, and replay/projection ancestry. Add cross-reference hooks so `04` provenance and `09` golden-fixture acceptance cannot pass on schema presence, fixture labels, or stable bytes alone. Generalizes the existing mutation-only "pending is not a pass" rule to the whole execution evidence surface (truth firewall, provenance, freshness, possession, accounting, emergence observation, replay). |
| D9 | E08 | `10` (forward routes to `docs/3-reference/01` + `docs/4-specs/0003`) | Addition (evidence honesty) | An evidence-honesty section in `10`: every review packet must label evidence by **status** (pass/fail where actually certified; pending where not yet proven; sampled where representative but not exhaustive; observer-only where it cannot certify behavior; historical where archive/spec evidence is context, not current certification) and by **fingerprint scope** (raw bytes / normalized serialization / parsed semantic content / command transcript / run seed / replay artifact). A fingerprint must not be cited as proof beyond its scope; pending/sampled/observer-only/archive evidence must never be silently counted as a pass. Risk-register wording and acceptance-template fields route forward (§6). |
| D10 | E09 | `11` | Addition (Phase-4 future-proofing) | A Phase-4 provenance future-proofing obligation in `11`: any institution-known fact, record-derived belief, norm-triggered procedure, wrong-suspicion inference, or artifact interpretation must prove provenance sufficiency and freshness using the **same fail-closed mechanics as `04`/`10`**. Negative fixtures: institution reacts to truth without a record; stale record treated as fresh; dangling record provenance; wrong-kind provenance; record display text mistaken for provenance. Locks the proof contract before Phase-4 entry without expanding Phase-4 scope or deciding new institution mechanics. |
| D11 | E10 | `13` | Addition (research source notes) | A proof-methodology source-notes section in `13` citing mutation testing, metamorphic testing, property-based testing (incl. shrinking), approval/golden testing, deterministic-simulation testing, OpenTelemetry-style structured observability, and the test-oracle problem (report §9). Add forbidden misreads: mutation coverage is not certification by itself; a surviving mutant is not harmless without disposition; a golden's byte stability is not semantic truth; deterministic replay is reproducibility, not correctness; structured-log existence is not behavior evidence; observer-only emergence evidence is not a pass/fail gate. Source-note hygiene only — no product doctrine or implementation design. |

D1 is an editorial reconciliation; D2–D11 are additive proof obligations. All are consistent with
existing execution gates and with foundation/architecture doctrine; none weakens a lock.

## 4. Invariants Alignment

| Invariant / doctrine | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-111` — observer-only emergence evidence | aligns (realizes) | D7 gives the `EMERGE-OBS` artifact the architecture-`13` data contract + invalid-pass conditions at the testing/review surface; D2 makes it a required first-proof acceptance artifact at the acceptance-package surface — both keep it retrospective, replay-ancestry-backed, and structurally barred from feeding behavior. |
| `INV-099` — truth may validate, not plan | aligns (proves) | D3 (provenance fail-closed), D5 (observation-time snapshots / wallhack negatives), and D8 (behavior witnesses) prove that proposal/routine/no-human/possession/diagnostic paths get behavior only from actor-known causal information, never from raw truth/debug/display at the action-pipeline, view-model, and review surfaces. |
| `INV-101` sealed actor-known context / `INV-102` cognition inputs require provenance | aligns (proves fail-closed) | D3 proves missing/empty/dangling/wrong-kind/ambiguous/forbidden/harness-fabricated provenance fails closed on the path under test (incl. institution-known, via D10); D4 proves the freshness classifier so no surface restamps stale belief as current observation. |
| `INV-108` — human possession is cognition-neutral | aligns (proves parity) | D4 and D5 require possession/embodied/notebook surfaces to use the same freshness classifier and observation-time capture as autonomous actors — possession is not a fresher epistemic surface and re-reads no truth at the TUI/possession surface. |
| `INV-109` — LLM output is never cognition authority without validation | aligns (preserved) | D1's vocabulary reconciliation and D8's behavior-witness standard keep LLM/speech-boundary proof inside the typed-evidence regime; no deliverable grants an LLM surface cognition authority. |
| Event-sourced causality / deterministic replay (`docs/0-foundation/03`; `INV-001…`) | aligns (proves) | D6 proves a single-owner accounting seam so no tick/window is causally charged twice across scheduler/action/projection/replay/golden — protecting replay truth, with D8/D9 forbidding byte-stable-but-semantically-false golden acceptance. |
| `INV-097` no-script compliance is tested / `INV-098` feature acceptance is harsh | aligns (extends) | D8 (general anti-vacuity, live negatives, behavior witnesses) and D9 (evidence-status / fingerprint-scope honesty) make every lock falsifiable and forbid "looks-right" / pending / sampled / byte-stable evidence from being mislabeled as certification at the review-artifact surface. |
| `INV-087` human-focus-is-not-privilege / `INV-006` possession transfers no knowledge | N/A here (preserved) | D5 explicitly does **not** decide whether possession bind counts as modeled perception; the deferred owner question (F04) is preserved, no silent constitutional edit. |

No invariant is weakened or tensioned. All deliverables are additive/clarifying execution proof
obligations; no foundation principle or architecture contract is re-decided, and no new code/
obligation/invariant identifier is coined.

## 5. Verification

- **V1 — Gaps confirmed against live `HEAD` (done at authoring).** Verdict-critical execution-doc
  gap claims were re-verified on `HEAD` `64a8367` (the report's pinned commit), ignoring the
  report's fabricated line anchors: `INV-111` is present in `docs/0-foundation/02` (with the exact
  observer-only/retrospective/replay-ancestry wording); `02` carries **no** emergence/observer-only
  artifact in its first-proof package (E01 real); `10`'s `EMERGE-OBS` section says "It amends no
  doctrine" and references only `INV-107`, lacking `INV-111`/architecture-`13` fields (E06 real);
  `04` has a provenance-minimum table + forbidden substitutes but no shared fail-closed sufficiency
  rule and no institution-known/wrong-kind/dangling negatives (E02 real); `06` proves single-tick
  need accounting but names no single-owner seam and no byte-stable-but-false golden rule (E05 real);
  `00`/`10` already declare `EMERGE-OBS` as a non-certifying observer-only obligation (C01 confirmed
  already-owned); `10`'s "pending is not a pass" exists only in the mutation section, not as a
  general evidence standard (E07/E08 confirmed partial).
- **V2 — Enactment acceptance (on implementation).** Each deliverable is accepted only when its
  substance is authored into the named execution file as a proof obligation (not a bare index row),
  `EMERGE-OBS` remains non-certifying and observer-only after D7, no existing gate is weakened, and
  no new gate code / observation-obligation code / invariant number is coined.
- **V3 — Boundary check.** Confirm enactment introduces no foundation or architecture edit and
  re-decides no product/architecture contract; the only files touched are `docs/2-execution/*`
  (forward routes to reference/specs are separate later sessions — §6, V4).
- **V4 — Forward-routing follow-through (later sessions, not this spec).** The reference `01`
  acceptance-evidence risk-cluster wiring (F02), the reference `02` emergence-evidence glossary term
  (F01; spec `0026` D4), the `docs/4-specs/0003` acceptance-artifact template fields (F03), and the
  possession-bind perception owner decision (F04) are enacted by their own sessions **after**
  execution doctrine is accepted; this spec only records the hand-offs (§6).

## 6. Out of Scope

- **Final execution wording, fixture names, command lines, thresholds, ratchets, status-taxonomy
  tokens, schema fields, table/row names.** Owned by the reassess/ticket enactment within execution's
  existing vocabulary.
- **C01–C04 — already-owned, no execution change.** `00`/`10` already declare `EMERGE-OBS`
  non-certifying and observer-only (C01; the *mechanism* realignment is D7, but the gate/observation
  boundary is owned). `01` archived-spec posture + P0 re-audit boundary is correct (C02). `08`
  data-authoring/schema provenance already forbids outcome-chain data at the proof level (C03;
  D3 may cross-reference it). `12` deferred second-proof / read-only story-sifting boundary remains
  correctly locked (C04). Their absence from §3 is deliberate, not an oversight.
- **F01 — emergence-evidence glossary term → `docs/3-reference/02`.** Execution may cite the existing
  `EMERGE-OBS` obligation but must not coin a canonical term; reference owns terminology control
  (spec `0026` D4), authored after `10` realignment.
- **F02 — acceptance-evidence honesty risks → `docs/3-reference/01`.** The R-27/R-28/R-29 cluster
  (pending≠pass, sampled≠certifying, observer-only≠gate, byte fingerprint≠semantic proof,
  archive≠certification, artifact presence≠behavior witness) is risk-register wiring, not execution
  gate prose. D9 owns the execution proof standard; reference preserves the watch-risk language.
- **F03 — acceptance-artifact template fields → `docs/4-specs/0003`.** Template fields for evidence
  status, scope, fingerprint scope, sampled-vs-certifying, observer-only/pending, and path-under-test
  behavior witnesses are authored **after** D9 establishes the execution rule — not before.
- **F04 — possession-bind perception (deferred owner decision; do NOT silently amend).** Whether
  possession *binding* itself counts as modeled perception is an owner/product-architecture decision,
  not an execution call. D5 keeps `07` neutral; once the owner decides, `07` proves the chosen policy
  (bounded bind-time perception snapshot with provenance/freshness, **or** no perception and no
  freshening) with the same snapshot/parity discipline.
- **Foundation and architecture edits.** `INV-099…INV-111` (ratified, `0026`) and the architecture
  contracts (ratified, `0027`) are upstream and unchanged; this spec edits no `docs/0-foundation/*`
  or `docs/1-architecture/*` file.
- **Code certification.** No assertion that crates, fixtures, CI, or goldens already satisfy the
  amended execution doctrine; `P0-CERT`/baseline certification is later work after the execution tier
  is realigned.
- **New world mechanics, Phase-4 expansion, LLM surfaces, crate/code changes.**

## 7. Risks & Open Questions

- **R-A — Execution enactment requires owner approval.** Authoring D1–D11 edits tier-2 doctrine.
  Lighter than constitutional sign-off, but it must not proceed by convention; this spec stages the
  substance, it does not authorize the edits.
- **R-B — `EMERGE-OBS` Goodhart relapse on D7.** The emergence-evidence record must stay observer-only
  and non-certifying; a zero-floor / "dead-world detector" may be recorded only as an
  observation/reporting policy, never a dramatic-quality pass threshold. Any future numeric threshold
  or certification use requires a dedicated future spec. D7's one-way path + invalid-pass conditions
  forbid this at the execution tier.
- **R-C — Vocabulary reconciliation scope (D1).** D1 must only *classify* existing labels, not rename
  established phase-cert artifacts or coin new gate codes. Whether `EPI-CERT` / `ORD-LIFE-CERT` /
  `DIAG-CERT` etc. are formal certification labels, informal names, or should map under canonical
  gate names is an editorial decision left to a later reassess session — D1 makes the distinction
  visible without forcing the mapping.
- **R-D — Institution-known provenance residual (D10; watch).** `11` already points the right way
  (institution-known context, records, wrong-suspicion lock). D10 imports the `04`/`10` fail-closed
  mechanics before Phase-4 expands; it must not expand Phase-4 scope or design new institution
  mechanics. Revisit when Phase-4 machinery introduces subsystems that could repeat unbacked facts.
- **R-E — Possession-bind perception (deferred owner decision; do NOT silently amend).** Preserved as
  F04. D5 does not touch it; if the owner decides, the home is `07` possession proof, and any bind
  perception must be a modeled event/channel for the actor, never a human/player knowledge transfer
  (`INV-006`/`INV-108`).
- **R-F — Eleven deliverables is the largest tier-alignment package to date** (vs. `0027`'s seven),
  with `10` carrying four (D7–D9 + the D8 cross-refs). Decomposition into tickets should respect the
  per-doc ownership boundaries and keep `10`'s additions as separable reviewable diffs so a stalled
  sub-item does not block the others.

## 8. Provenance & Source Discipline

- The source report is pinned to `64a8367` and was re-verified against that same live `HEAD`; no
  intervening commit drift applies.
- Commit hashes named here are audit/spec provenance only.
- The report's external proof-methodology references (its §9) are research support for D8/D9/D11, not
  Tracewake authority; they sharpen proof design and replace no doctrine.
- This spec adds **no** `docs/4-specs/SPEC_LEDGER.md` row at proposal time, per the staged-spec
  convention (the `0026`/`0027`/hardening-series precedent: the ledger row lands at acceptance/
  closeout, not at proposal).

## Outcome

Completed: 2026-06-13

Owner approval precondition: satisfied by the user's active `$ticket-series`
goal to implement `tickets/0028EXETIEDOC*` against this spec.

Enacted:

- D1 by `archive/tickets/0028EXETIEDOC-001.md`: execution label taxonomy and
  light cross-reference cleanup.
- D7, D8, and D9 by `archive/tickets/0028EXETIEDOC-006.md`: `EMERGE-OBS`
  realignment, anti-vacuity / behavior-witness standard, and evidence-status /
  fingerprint-scope honesty.
- D11 by `archive/tickets/0028EXETIEDOC-008.md`: proof-methodology source
  notes and forbidden misreads.
- D2 by `archive/tickets/0028EXETIEDOC-002.md`: first-proof acceptance package
  `EMERGE-OBS` artifact requirement.
- D3 and D4 by `archive/tickets/0028EXETIEDOC-003.md`: provenance-sufficiency
  fail-closed proof and freshness classifier.
- D5 by `archive/tickets/0028EXETIEDOC-004.md`: observation-time snapshot
  proof, wallhack negatives, F04 neutrality, and embodied carrier census.
- D6 by `archive/tickets/0028EXETIEDOC-005.md`: single-owner
  derived-accounting seam and byte-stable-insufficient golden rule.
- D10 by `archive/tickets/0028EXETIEDOC-007.md`: Phase-4 institution-known
  provenance/freshness future-proofing.

Verification:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.

Deviations:

- None. The spec remains `P0-CERT not applicable`: it enacted execution-tier
  doctrine alignment only, certifies no code, and does not claim that crates,
  fixtures, CI, or goldens already satisfy the amended execution doctrine.
