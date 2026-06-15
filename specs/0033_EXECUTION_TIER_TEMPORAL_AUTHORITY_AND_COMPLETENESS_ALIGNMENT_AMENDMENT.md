# Spec 0033 — Execution-Tier Temporal-Authority and Completeness Alignment Amendment

This spec **proposes a set of execution-tier (`docs/2-execution/*`) amendments**. It is a
design/proposal artifact: it specifies the *substance and home* of each amendment so Tracewake's
reassess / ticket process can author the final execution wording. **It does not itself author
ratified execution prose, and it mints no new gate codes, observation-obligation codes, or fixture
names.** Execution is tier-2 doctrine but is not constitutional, so enactment requires owner
approval rather than the constitutional sign-off that a foundation amendment demands (cf.
`archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md`,
`archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md`,
`archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`).

> Section set: this file uses the canonical `specs/` section set (Problem Statement, Approach,
> Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions) because
> `specs/` carries no template at authoring time and this is not a hardening implementation spec.
> It deliberately does **not** copy the foundation-pack docs' narrative house style.

**Status:** PROPOSED. Stages additive amendments to `docs/2-execution/*`; not yet enacted.

**Admissibility posture:** `P0-CERT not applicable`. This is a doctrine-alignment proposal; it
certifies no code and performs no gate audit.

**Authority:** A spec is subordinate to execution and *may not replace execution doctrine*
(`docs/4-specs/SPEC_LEDGER.md`). This spec does not exercise that forbidden authority. It
**operationalizes higher-tier doctrine** — the temporal-authority invariant `INV-112` and the
foundation `03` "Temporal authority" section (ratified by archived spec `0031`), and the
architecture-tier temporal cascade (T1–T10) and completeness themes (R1–R7) (ratified by archived
spec `0032` into `docs/1-architecture/*`) — by staging *additive* proof obligations, certification
placements, fixture-family requirements, diagnostic fields, and minor cross-references so the
execution tier translates that doctrine into gate procedure. No deliverable weakens or contradicts
existing execution gates; the final wording, once authored, is execution-tier doctrine and this spec
becomes historical provenance.

**Provenance:** derived from `reports/execution-tier-alignment-research-report.md` (external deep
research, pinned to commit `c70d119bca7663e4e1229dcd2012d57b5fe72d44` = current `HEAD` `c70d119`,
the merge of the `0031` and `0032` enactments) and its brief
`reports/execution-tier-alignment-research-brief.md`. The report is the planned `docs/2-execution/*`
session of the temporal-authority + foundations-completeness cascade. Because the report's pinned
commit equals current `HEAD`, no intervening-commit drift applies. The report's verdict-critical
premises were independently re-verified against live `HEAD` during authoring (see Verification §V1);
per the deep-research-spec convention, any fabricated `#Lxxxx` line anchors are ignored in favor of
named symbols and sections.

---

## 1. Problem Statement

Foundation `0031` promoted **temporal authority** to constitutional doctrine: `INV-112` ("Time may
validate, but holder-known time must plan") plus a foundation `03` "Temporal authority" section and
application hooks across foundation `04`/`05`/`07`/`08`/`10`/`12`/`14`. Architecture `0032` then
translated that doctrine into subsystem contracts (Block T, T1–T10) and named compact seams for the
completeness-routed themes (Block R, R1–R7) across `docs/1-architecture/*`. The temporal firewall is
the truth firewall applied to time: the scheduler/replay clock may **order and validate**;
cognition, routine selection, institutional procedure, embodied view models, speech interpretation,
leads, and LOD promotion may use temporal facts only when those facts reached the relevant holder
through **modeled channels**. Both upstream specs deliberately deferred the execution cascade.

The execution tier has **not yet absorbed that delta.** Verified against live `HEAD` `c70d119`, the
execution docs carry their first-cascade gate spine (`P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`,
`NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`, plus the `EMERGE-OBS` observation obligation)
but **zero** temporal-authority, `INV-112`, procedural-time, or staged-declaration language: exec
`04`, `05`, `00`, and `11` all return empty on temporal/`INV-112`/staged probes. The proof homes
exist; the **temporal specialization, the completeness-theme proof obligations, and the
staged-incompleteness declaration discipline do not.** A future implementer cannot derive the
execution proof obligations for `INV-112` from the execution tier as it stands.

This is a **delta, not a cold start.** The execution-tier alignment report dispositions thirteen
findings across three drivers, every one a *partial coverage* or *belongs in execution* verdict — no
finding is a blank hole, and none reopens the settled first cascade (`0028`, `0009…0025`):

- **Driver T — temporal-authority cascade (F-01…F-07).** Translate the now-landed temporal firewall
  (`INV-112`/foundation `03`; architecture T1–T10) into execution gate procedure, certification-
  sequence placement, fixture families, and diagnostics.
- **Driver R — completeness Block-R execution routes (F-08…F-12).** Quantity/granularity/
  fungibility validation, learning/adaptation without hidden truth-cache, deterministic budget/
  fairness proof, practical bias/social harm, and authoring/compiler discipline each need an
  explicit execution proof obligation over the existing validation/fixture/diagnostic spine.
- **Driver S — staged incompleteness (F-13).** Routed **only** to execution (it did not pass through
  the architecture enactment): a stage-declaration discipline so every deliberate abstraction
  declares what it proves now, what it abstracts, what it must not fake, what future feature it must
  not block, and what evidence prevents overclaiming.

None of these is a foundation or architecture hole — `INV-112`, the architecture cascade, and the
completeness routing are all ratified. Each is an **execution proof-obligation** gap: foundation
states the `what`, architecture owns the subsystem contract, and execution must own the gate
procedure, proof obligation, fixture family, and diagnostic — while reference (tier 3) and specs
(tier 4) below execution own the concrete vocabulary, thresholds, schemas, and fixture packages. Two
execution docs (`01`, `13`) are swept *already-owned-close* with no primary amendment required (Out
of Scope §6).

## 2. Approach

Stage fourteen compact, additive execution refinements (Block T: D-T1…D-T8; Block R: D-R1…D-R5;
Block S: D-S1), each landing in the execution doc that *owns* the proof home, with cross-links where
the report identifies a secondary anchor. Keep every change at the `what must be proven / where it
enters the gate ladder / what fixture family and diagnostic field it requires` level. Route every
concrete temporal value (day-part / "yesterday" / "office closed" / "due/late" / "stale" vocabulary,
calendar/date syntax, duration units, stale-after thresholds), scheduler queue/algorithm, fairness
formula/window, UI clock format, missed-summary threshold, simultaneity/tie-break rule,
inventory/economy schema, unit vocabulary, money denomination, affect/learning update rule/decay/
threshold, and every concrete fixture file name **out** of execution to reference `docs/3-reference/*`
and future scoped specs `docs/4-specs/*` (the report's forward-routing appendix §6 enumerates these
hand-offs; carried here in Out of Scope §6).

The fourteen deliverables are the only execution-tier items; the report's forward-routing hand-offs
are deferred to later tier sessions. Final execution wording — and any decision to mint a new gate
code, observation-obligation code, or fixture identifier — is authored on enactment (by reassess /
ticket), not in this spec. This follows the precedent of `0028`, which staged the prior
(`INV-099…INV-111` / truth-firewall) execution cascade in exactly this form, and of `0032`, which
staged the architecture cascade.

This spec stages **all three drivers in one package** because all three are execution-tier,
documentation-only, and derive from the same report; the report itself frames a single execution
amendment. If an owner prefers, the blocks are cleanly separable into two or three specs — the
deliverable IDs and target homes are disjoint enough to split without rework (Risk §R-B).

## 3. Deliverables

All deliverables are **proposed amendment substance**, to be authored into the named execution file
by the reassess / ticket process. None prescribes final wording, gate/observation codes, fixture
names, schema fields, vocabularies, thresholds, or algorithms. Each must be authored *additively*
over the existing first-cascade gate, never as a rename, weakening, or replacement of it.

### 3.1 Block T — temporal-authority cascade (translates `INV-112` + foundation `03` + architecture T1–T10)

| # | Report finding | Target file(s) | Kind | Substance |
|---|---|---|---|---|
| D-T1 | F-01 | `00_EXECUTION_INDEX_AND_AUTHORITY.md`; `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` | Addition (routing map + certification placement) | **`00`:** a compact temporal / Block-R / staged-declaration routing subsection pointing each proof surface to its execution home — temporal firewall & holder-known time → `04`; scheduler trigger-vs-plan & budgets → `05`; routines → `06`; TUI rendering & time controls → `07`; authoring validation → `08`; fixture families → `09`; diagnostics/review artifacts → `10`; institutions/procedural time → `11`; LOD/time-acceleration → `12`. It must **not** mint a new gate code and must state that concrete temporal values/terminology are lower-tier decisions. **`03`:** place the temporal cascade in the existing certification sequence (where temporal evidence must appear before first-proof acceptance, where Phase-4 procedural-time evidence appears, where second-proof LOD/time-acceleration evidence is deferred-but-declared) without creating a new gate code. |
| D-T2 | F-02 | `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`; `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`; `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Addition (temporal firewall proof) | **`04`:** require temporal-firewall evidence in every truth-firewall certification touching time — the source of every temporal premise consumed by actor-known/institution-known code (observation, memory, record, notice, testimony, public cue, artifact, modeled procedure, source-backed inference) — plus adversarial evidence that raw scheduler time, replay order, debug panels, event timestamps, sorted queues, and validator-known future/due states cannot be read as holder knowledge via type shortcuts, cached truth, renamed fields, derived helpers, or prompt/context prose. **`05`:** state the scheduler's execution-altitude temporal authority — it may awaken candidates, order transactions, validate preconditions/effects, and account for budget exhaustion; it may not select intentions, invent reasons, rewrite wait causes, or conclude routines by consulting true time. **`10`:** require temporal-divergence diagnostics with responsible-layer labels (candidate generation, sealed-context assembly, scheduler dispatch, action validation, projection/view rendering, fixture authoring, or review-artifact construction). |
| D-T3 | F-03 | `04_…ANTI_CONTAMINATION_GATES.md`; `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`; `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`; `10_…REVIEW_ARTIFACTS.md` | Addition (holder-known temporal claims at first-playable) | **`04`:** require temporal-claim slots in holder-known context checks when a decision depends on time (event/acquisition/verification/procedure source; valid/expired/stale status as known by the holder; explicit uncertainty where appropriate). **`06`:** require first-playable no-human scenarios where routine behavior succeeds from modeled temporal premises and waits/fails when only ground-truth time would justify action. **`09`:** require a first temporal-firewall fixture **family** with friendly families (holder-known work/routine timing, stale-but-believed notices, institutional records, interruption/wait effects) and adversarial families (raw-clock leakage, debug-panel leakage, omniscient due/closed labels, restamping old knowledge as fresh) — no concrete fixture names. **`10`:** require review artifacts to distinguish "validator time used to validate" from "holder-known temporal premise used to plan," showing both positive-acceptance and fail-closed paths. |
| D-T4 | F-04 | `06_…NO_HUMAN_PROOF.md` (xref `05`, `10`) | Addition (routine/social-rhythm temporal premises) | **`06`:** require each routine/social-rhythm proof to identify the temporal-premise *source category* it used (the fact that a modeled channel supplied the premise, not the vocabulary itself); require negative no-human examples where a routine would be correct under true schedule time but is not selected because the actor lacks the source-backed premise; require positive adaptation examples where repeated modeled experience, contradiction, interruption, or notice changes later routine selection through an actor-/institution-known path. **Xref `05`** so scheduler awakenings/elapsed-time accounting do not count as routine-premise evidence. **Xref `10`** so diagnostics identify whether a routine failure is missing knowledge, stale knowledge, budget exhaustion, blocked affordance, or validation failure. |
| D-T5 | F-05 | `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`; `10_…REVIEW_ARTIFACTS.md` | Addition (temporal rendering / time controls) | **`07`:** require embodied-view temporal labels (late, stale, closed, due, soon, recently, missed) to cite modeled sources, not a readable clock/queue; require possession-parity evidence that possessing an actor does not refresh, reveal, or reinterpret temporal facts beyond what that actor would know; require future player-facing time-control proof to show that advancing time produces events/observations through modeled channels (any missed-event summary must carry source and holder visibility); keep debug exact time, raw queues, replay timestamps, and acceleration internals non-diegetic and excluded from actor-known context. **`10`:** add temporal-rendering diagnostics distinguishing embodied, possession, debug, transcript, and observer-only surfaces. Preserves the existing embodied/debug split and staged time-acceleration note. |
| D-T6 | F-06 | `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` (xref `10`, `08`) | Addition (institution / procedural-time proof) | **`11`:** require Phase-4 procedural-time evidence for any institution label with time semantics (open/closed, due/late, expired/current, pending/resolved, queued/aged, notified/served, paid/unpaid, sanctioned, appealed); require each institutional temporal status to be backed by a modeled institution-known source (record, schedule artifact, notice, ledger entry, procedure state, testimony accepted by procedure, inspection, modeled staff action); require adversarial institution fixtures where true time would justify a label but records do not yet support it, and where stale/mistaken records cause plausible institutional error. **Xref `08`** so content packs cannot author omniscient procedural-time conclusions. **Xref `10`** so diagnostics distinguish record error, procedure delay, source staleness, contradictory testimony, missing artifact, and hidden ground-truth leakage. |
| D-T7 | F-07 | `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`; `10_…REVIEW_ARTIFACTS.md` | Addition (deferred LOD/time-acceleration ancestry) | **`12`:** require future LOD/time-acceleration proof to declare interval, cadence, resolution, fidelity, event ancestry, information ancestry, and known-to-whom status for every summary or promoted state; require equivalence-or-declared-divergence evidence (any accelerated/regional divergence from full-resolution processing must be declared, bounded, and tested against fairness and epistemic constraints); require promotion/demotion proof that actors, institutions, leads, and views gain no omniscient temporal facts during LOD transitions; require skipped/deferred-cognition accounting so acceleration cannot invisibly starve actor classes or silently script outcomes. **`10`:** add diagnostic expectations for LOD temporal-ancestry and fairness review. These are proof obligations for deferred work; they keep `12`'s deferral intact. |
| D-T8 | §4 register (`02`) | `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` | Addition (narrow acceptance cross-reference) | **`02`:** **no broad rewrite** — add only a narrow cross-reference recording that first-playable acceptance now includes temporal-firewall evidence through `04`, routine temporal proof through `06`, embodied temporal rendering through `07`, fixture families through `09`, and diagnostics through `10`. Derives from the report's §4 coverage register (a per-doc recommendation, not an `F-NN` finding); `02`'s first-proof scope and acceptance stance are otherwise kept intact. |

### 3.2 Block R — completeness-routed execution proof obligations

| # | Report finding | Target file(s) | Kind | Substance |
|---|---|---|---|---|
| D-R1 | F-08 | `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`; `09_…REPLAY_ACCEPTANCE.md`; `10_…REVIEW_ARTIFACTS.md` | Addition (quantity / granularity / fungibility / custody validation) | **`08`:** require content validation for quantity-bearing and fungible/partly-fungible entities to preserve identity, quantity, custody, ownership/control, provenance, split/merge lineage, transformation, spoilage/consumption, reservation, transfer, concealment/discovery, and institution-visible record effects as appropriate to the authored domain; require fail-closed behavior on ambiguous fungibility, implicit global pools, untracked disappearance/creation, balance edits without ledger ancestry, and authored facts that make an actor/institution know quantity/custody without a modeled channel. **`09`:** require separate fixture families for quantity/economy behavior (positive transfer/consume/split/merge/custody; adversarial hidden stock/ledger/procedure) — no fixture names. **`10`:** require diagnostics that identify representation error, custody-lineage error, procedure-visibility error, and replay divergence. Do **not** choose units/schemas/denominations. (See Risk §R-E: do not bundle the quantity/economy fixture package with the temporal package unless one gameplay feature genuinely needs both.) |
| D-R2 | F-09 | `04_…ANTI_CONTAMINATION_GATES.md`; `06_…NO_HUMAN_PROOF.md`; `10_…REVIEW_ARTIFACTS.md` | Addition (learning/adaptation without hidden truth-cache) | **`04`:** require negative hidden-truth-learning evidence — neither failed actions, scheduler denials, debug facts, true object locations, true schedules, true institutional status, nor replay-only diagnostics may become future actor belief/expectation unless a modeled experience or communication channel emitted a claim, memory, contradiction, observation, notice, or record. **`06`:** require positive adaptation proof — repeated modeled experience, contradiction, or changed routine outcomes can affect future routine/method/trust selection through holder-known memory/expectation channels. **`10`:** require diagnostics distinguishing learned expectation, remembered event, direct observation, testimony, record-derived belief, and prohibited truth-cache update. Route affect/learning update rules, strength/decay, trust-update semantics, and thresholds to a future scoped spec. |
| D-R3 | F-10 | `05_…NO_DIRECT_DISPATCH.md`; `10_…REVIEW_ARTIFACTS.md` (consolidated home); `12_…SCALE_AND_LOD.md` | Addition (deterministic budget & fairness proof — consolidated) | **`05`:** require scheduler-budget evidence for deterministic candidate ordering, budget exhaustion, deferred/skipped cognition, and no-direct-dispatch behavior under load; state that budget pressure may not become an invisible director choosing outcomes without typed evidence. **`10`:** the **consolidated** home for budget/fairness diagnostics — starvation risk, repeated deferral, actor-class/region-class imbalance, time-acceleration effects, replay determinism. **`12`:** require deferred-scale fairness declarations for any future LOD/time-accelerated process. Do **not** choose fairness formulas, window sizes, budgets, priority algorithms, or thresholds. Enactment must consolidate (not triplicate) the budget/fairness contract — `10` owns it; `05`/`12` and Block-T cross-reference it (Risk §R-D). |
| D-R4 | F-11 | `11_…PHASE_4_ENTRY.md` (xref `08`, `10`) | Addition (practical bias & social harm as fallible institutional mechanics) | **`11`:** require Phase-4 practical-bias evidence where institutional outcomes may be shaped by modeled testimony quality, unequal credibility, access barriers, underfunding, refusal, delay, misfiling, contradictory records, stale records, suppressed/ignored records, or staff procedure; require wrong-suspicion proof to show the actor-/institution-known path that made a suspicion plausible without consulting hidden culprit truth. **`08`:** require domain-pack bias/social-harm assumptions to be explicit, validated, and reviewable rather than implied by prose. **`10`:** require diagnostics that identify the modeled source of a biased/harmful outcome and distinguish it from hidden-truth leakage or arbitrary author fiat. Route concrete domain-pack bias assumptions and evaluation criteria to future scoped specs. Preserves the genre-neutral kernel — no morality oracle, no objective social-harm quest condition. |
| D-R5 | F-12 | `08_…SCHEMA_PROVENANCE_AND_VALIDATION.md`; `09_…REPLAY_ACCEPTANCE.md`; `10_…REVIEW_ARTIFACTS.md` (optional source note `13`) | Addition (authoring/compiler discipline for proof-bearing content) | **`08`:** make explicit that temporal claims, procedural-time records, quantity/custody records, bias/social-harm assumption packets, and staged-abstraction declarations are proof-bearing authored content and must be structurally validated; require fail-closed behavior for malformed provenance, missing source channels, ambiguous authority category, hidden-truth labels in prose fields, restamped freshness, implicit global state, and unreviewable assumptions. **`09`:** require adversarial fixtures that attempt to bypass validation through renamed fields, nested prose, stale strings, generated content, fixture metadata, or review-artifact text. **`10`:** require review evidence that static validation ran, negative fixtures failed for the intended reason, and diagnostics identified the responsible layer. **Optional `13`:** source notes for compiler-like validation / policy-as-code when the amendment is drafted. Do **not** choose schemas, rule languages, or error formats. |

### 3.3 Block S — staged-incompleteness declaration discipline (routed only to execution)

| # | Report finding | Target file(s) | Kind | Substance |
|---|---|---|---|---|
| D-S1 | F-13 | `00_EXECUTION_INDEX_AND_AUTHORITY.md`; `03_…CERTIFICATION_SEQUENCE.md`; `10_…REVIEW_ARTIFACTS.md` | Addition (acceptance-honesty discipline) | **`00`:** add the authority-level rule that staged proof is allowed only when the staged abstraction is declared and bounded, and staged proof must not certify the unimplemented future feature by implication. **`03`:** place staged-declaration review in the certification sequence — the declaration is required before acceptance evidence is treated as sufficient for a stage. **`10`:** add staged-abstraction review fields — proof currently provided; behavior intentionally abstracted; falsehoods the stage must not fake; future feature or tier it must not block; evidence preventing overclaiming; diagnostics that fail if the abstraction leaks into certification. Route the acceptance-artifact *template* addition (`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`) to a future scoped spec. Do **not** mint a new gate label or observation-obligation code. |

D-T4 and D-T6 carry cross-references to secondary homes; all other deliverables are additive proof
obligations or consolidations. D-S1 is the only driver not carried through the architecture cascade,
so execution must not lose it. All fourteen are consistent with existing execution gates and with
foundation/architecture doctrine.

## 4. Invariants Alignment

| Invariant / doctrine | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-112` — time may validate, but holder-known time must plan | aligns (operationalizes) | The whole Block T cascade gives execution the gate procedure and proof obligations for the temporal firewall: D-T2 (temporal-firewall evidence + divergence diagnostics @ truth-firewall/scheduler/observability gates), D-T3 (holder-known temporal-claim fixtures @ first-playable proof), D-T4 (routine premises @ no-human proof), D-T5 (temporal rendering @ view-model/possession proof), D-T6 (procedural time @ Phase-4 institutions), D-T7 (LOD temporal ancestry @ deferred-scale proof), D-T1 (routing map + certification placement @ index/phase-ladder), D-T8 (first-proof acceptance cross-reference @ acceptance contract). No proof obligation feeds simulation behavior with hidden temporal truth. |
| `INV-099` — truth may validate actions, but truth may not plan them | aligns (specializes for time) | D-T2/D-T3 require evidence that validator/scheduler/replay time may validate and order but may not become a holder-known planning premise absent a modeled channel — the temporal case of `INV-099` at the truth-firewall gate. |
| `INV-102` — cognition inputs require provenance | aligns (extends) | D-T3 requires temporal-claim provenance slots in holder-known context checks; D-R2 forbids provenance-less truth-cache learning; D-R5 makes temporal/quantity/bias/staged declarations provenance-bearing, fail-closed authored content. |
| `INV-103` — the scheduler is not a cognition authority | aligns (specializes) | D-T2 (`05`) states the scheduler may awaken/order/validate/account-for-budget but may not select intentions, invent reasons, or conclude routines from true time; D-T4 keeps scheduler awakenings out of routine-premise evidence. |
| `INV-105` — actor decision traces are authoritative typed diagnostics | aligns (extends) | D-T2/D-T7/D-R3 require temporal-divergence, LOD-ancestry, and budget/fairness outcomes to be typed, responsible-layer-attributed diagnostic fields, not display strings, at the observability surface. |
| `INV-110` — LOD/summary processes must preserve the firewall | aligns (specializes) | D-T7 forces deferred LOD/time-acceleration proof to preserve temporal + information ancestry and to bar omniscient temporal fill-in during promotion/demotion, at the deferred-scale surface. |
| `INV-111` — observer-only emergence evidence | aligns (preserves) | D-T5/D-T7/D-R3 keep temporal-rendering, LOD, and budget/fairness review summaries observer-only, non-certifying, and never simulation inputs or actor-known urgency. The existing `EMERGE-OBS` obligation and the prior-cascade observer-only carve-out are preserved, not weakened. |
| `INV-087` — human focus is not player privilege; `INV-006`/`INV-108` possession is cognition-neutral | aligns (preserves) | D-T5 requires possession-parity evidence that possession does not refresh/reveal/reinterpret temporal facts; D-R3 requires fairness/starvation evidence of no human-proximity or possessed-actor priority bias unless explicitly non-diegetic input routing. |
| Questless / no-director, genre-neutral kernel | aligns (preserves) | D-R4 requires bias/social harm to flow through modeled institutions/records with no morality oracle and no objective social-harm quest condition; D-R3 forbids budget pressure as an invisible director; D-S1 forbids staged proof from certifying unimplemented futures by implication. |

No invariant is weakened or tensioned. All deliverables are additive execution proof obligations at
the `what must be proven / where it gates` altitude; no `how`-level mechanism (tick size, calendar
syntax, threshold, fixture name, command, schema, algorithm, fairness formula, unit vocabulary)
enters execution. **No execution file at this commit shows a conflict requiring a foundation or
architecture edit** — the report finds execution gaps and consolidations, not constitutional or
architecture tension. Any apparent upstream discomfort is carried as an open question (§7), not
patched in execution.

## 5. Verification

- **V1 — Premises confirmed against live `HEAD` (done at authoring).** Re-verified on `HEAD`
  `c70d119` (= the report's pinned commit, the merge of the `0031`/`0032` enactments):
  - `INV-112` is present in `docs/0-foundation/02` with the "time may validate, but holder-known
    time must plan" wording; `INV-099`, `INV-102`, `INV-103`, `INV-105`, `INV-110`, `INV-111`,
    `INV-087`, `INV-006`, `INV-108` are present with the meanings cited in §4.
  - The architecture temporal cascade (`0032`: T1–T10, R1–R7) is enacted in `docs/1-architecture/*`
    (the immutable governing reference for this pass).
  - The execution homes carry **no** temporal/`INV-112`/procedural-time/staged-declaration language
    today — exec `04`, `05`, `00` (staged), and `11` (procedural-time) all return empty on targeted
    probes — while the first-cascade gate spine (`P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`,
    `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`, `EMERGE-OBS`) is present. The Block-T/S gaps are real;
    the Block-R themes are proof obligations over existing validation/fixture/diagnostic substrate.
- **V2 — Enactment acceptance (on implementation).** Each deliverable is accepted only when its
  substance is authored into the named execution file as a compact, additive proof obligation /
  certification placement / fixture-family requirement / diagnostic field (not an index-row pointer
  only), the first-cascade gates are preserved (no rename/weaken/replace), no new gate code,
  observation-obligation code, or fixture name is minted by this spec's enactment unless the
  reassess/ticket process deliberately decides one is needed, and no concrete value token (tick
  size, calendar/date syntax, duration unit, day-part/lateness vocabulary, stale-after number,
  scheduler queue/algorithm, fairness formula/window, UI clock format, status enum, schema field,
  fixture name, unit vocabulary, money denomination, command, threshold) enters execution.
- **V3 — Boundary check.** Review each newly added execution passage for mechanism/value tokens;
  execution must hold the proof obligation and gate placement only. A whole-file grep is not the
  proof surface because execution legitimately discusses these concepts in the abstract.
- **V4 — Consolidation check.** Confirm the budget/fairness contract is stated once (D-R3 in `10`)
  and only cross-referenced from `05`/`12`/Block-T, not triplicated (Risk §R-D); confirm the
  quantity/economy fixture package (D-R1) is not bundled with the temporal fixture package (D-T3)
  absent a single gameplay feature that needs both (Risk §R-E).
- **V5 — Forward-routing follow-through (later sessions, not this spec).** The reference `00`/`01`/
  `02` glossary/risk wiring and the future scoped specs (concrete temporal vocabulary/thresholds,
  the first temporal-firewall fixture package, the inventory/economy fixture package, affect/
  learning depth, the domain-pack bias vehicle, budget/fairness numeric targets, and the
  `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` staged-abstraction addition) are enacted by their own tier
  sessions; this spec only records the hand-offs (Out of Scope §6 and report §6).

## 6. Out of Scope

- **Final execution wording, new gate/observation codes, fixture names, schema fields, vocabularies,
  thresholds, commands, status taxonomies.** Owned by the reassess/ticket enactment and by
  reference/specs. This spec may say *that* a fixture family or proof obligation is needed and
  describe its shape; it may not name it.
- **All concrete temporal values.** Day-part vocabulary, "yesterday"/"last night"/"office
  closed/open"/"due/late"/"recently/stale" terms, calendar/date syntax and conversion, duration
  units, stale-after policy / record-validity periods / claim decay, office-hour representation,
  queue-aging, time-acceleration speed, missed-summary thresholds, and simultaneity/tie-break rules
  route to reference and future scoped specs (report §6.1, §6.2, §7).
- **The first temporal-firewall fixture package and the inventory/economy fixture package.** Their
  concrete friendly/adversarial fixture files are future scoped specs; this spec stages only the
  fixture-*family* obligations (D-T3, D-R1). Do not bundle the two packages (report §6.3).
- **Affect/learning depth, domain-pack bias assumptions, budget/fairness numeric targets.** Update
  rules, decay, trust dimensions, thresholds, the bias-assumption vehicle, fairness formulas, and
  scheduler policy route to future scoped specs (D-R2, D-R3, D-R4 substance is the proof obligation
  only).
- **The acceptance-artifact template change.** The staged-abstraction declaration fields added to
  `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` route forward as a future scoped spec; D-S1
  stages only the execution-doctrine discipline in `00`/`03`/`10`.
- **Reference-tier terminology and risk memory.** Temporal-authority / holder-known-temporal-claim /
  procedural-time / freshness / time-acceleration / temporal-ancestry glossary terms, and risk notes
  for clock-oracle leakage, raw-wall-clock contamination, omniscient lateness/office-closed labels,
  UI time-acceleration leaks, debug-time-becoming-diegetic, silent LOD temporal fill-in, truth-cache
  learning, performance-pressure-as-invisible-director, budget starvation hidden by aggregate
  success, staged-abstraction false certification, and quantity/economy lineage collapse, are
  authored by a future reference session after execution wording stabilizes (report §6.2). Reference
  is not yet temporally amended (its last cascade was `0029`).
- **Foundation and architecture edits.** `INV-112`, foundation `03`, and the architecture cascade
  (`0031`/`0032`) are already ratified; this spec changes no `docs/0-foundation/*` or
  `docs/1-architecture/*` file and creates no invariant.
- **Execution docs `01` and `13` — verified already-owned-close, no primary deliverable.** `01`
  correctly keeps historical/archive status from becoming certification; `13` correctly records
  research-source handling and forbidden misreads. `13` may *optionally* receive source notes for
  temporal/deterministic-testing/provenance/fairness/staged-honesty references when D-R5 is enacted
  (report §F-12); that is a minor sub-item of D-R5, not a standalone deliverable. Their absence from
  §3 as primary homes is deliberate, not an oversight.
- **Reopening the settled first cascade.** No `0028` truth-firewall doctrine or `0009…0025`
  hardening work is reopened; every deliverable is the temporal / Block-R / staged delta only.
- **New world mechanics, Phase-4 expansion, LLM surfaces, crate/code changes.**

## 7. Risks & Open Questions

- **R-A — Execution enactment requires owner approval.** Authoring D-T1…D-S1 edits tier-2 doctrine
  across twelve execution docs (`00`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`;
  `01` and `13` carry no primary deliverable). Lighter than constitutional sign-off, but it must not
  proceed by convention; this spec stages the substance, it does not authorize the edits.
- **R-B — Block size / split option.** Fourteen deliverables across three drivers is large. The spec
  bundles them per the report's single-amendment framing and the user's "a spec" directive; Block T
  (temporal), Block R (completeness), and Block S (staged) are cleanly separable into two or three
  specs if an owner prefers smaller enactment units. Splitting requires no rework — the deliverable
  IDs and homes are disjoint.
- **R-C — Mechanism/value leakage during enactment.** The chief failure mode is authoring concrete
  temporal vocabulary, thresholds, schemas, fairness formulas, or fixture names into execution (V2/V3
  guard against this). The temporal block is especially exposed because "morning," "yesterday," and
  "stale-after" feel like execution but are reference/spec-owned values.
- **R-D — Budget/fairness lives in three deliverable touch-points (D-R3 across `05`/`10`/`12`, plus
  Block-T cross-refs).** Enactment must consolidate, not triplicate: `10` owns the cross-cutting
  budget/fairness diagnostics; `05`/`12` and the temporal block cross-reference it. Avoid three
  divergent statements of the same budget contract.
- **R-E — Fixture-package bundling.** The quantity/economy fixture family (D-R1) and the
  temporal-firewall fixture family (D-T3) must not be bundled into one execution proof package unless
  a single gameplay feature truly requires it (report §6.3). This spec stages only the family
  obligations, which is bundling-neutral.
- **R-F — `INV-111` / `EMERGE-OBS` observer-only relapse.** D-T5/D-T7/D-R3 introduce temporal/LOD/
  fairness review summaries. These must stay observer-only and non-certifying; counters or summaries
  becoming simulation objectives or actor-known urgency is forbidden authored-outcome machinery.
- **R-G — Staged-declaration discipline must not become a new gate code (D-S1).** The discipline is
  an acceptance-honesty rule threaded into `00`/`03`/`10`; the report is explicit that no new gate
  label or observation-obligation code is minted in this pass. Whether one is eventually warranted is
  an enactment-time owner decision, not a spec decision.
- **Open questions (carried forward, not answered here).** The report's §7 enumerates fourteen owner
  decisions the docs cannot settle — temporal-vocabulary ownership, calendar/duration syntax,
  staleness thresholds, same-time ordering, scheduler budget policy, time-acceleration modes, LOD
  fidelity floors, the first institution procedure pack, quantity schema choice, learning depth, the
  bias domain pack, fixture bundling, staged-abstraction template wording, and reference-pass timing.
  These route to reference/spec sessions; none is resolved by this spec.

## 8. Provenance & Source Discipline

- The source report is pinned to `c70d119` and was re-verified against that same live `HEAD`; no
  intervening-commit drift applies (`c70d119` is the current `HEAD` merge of the `0031`/`0032`
  enactments).
- External research cited in the report (SimPy/DES time & scheduling, Azure event-sourcing, W3C
  PROV-DM, JSON Schema / OPA policy-as-code, Antithesis deterministic-simulation testing, fair-
  scheduling literature, ODD / Datasheets / NIST AI RMF / technical-debt model-documentation
  practice) shaped tier-fit and proof-shape judgment only; none becomes Tracewake doctrine. The
  report §3.4 and §8.2 hold the full citation list; this spec does not restate it.
- Commit hashes named here are audit/spec provenance only.
- This spec adds **no** `docs/4-specs/SPEC_LEDGER.md` row at proposal time, per the staged-spec
  convention (the `0026`/`0027`/`0031`/`0032` precedent: the ledger row lands at acceptance/closeout,
  not at proposal).
