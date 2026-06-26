# Spec-0047 foundational-conformance hardening — fourth-pass research brief

**You are ChatGPT-Pro Session 2.** This document is your complete, final assignment. It is
self-contained: you see only this prompt plus one uploaded manifest. Produce the deliverable in
§7 directly. **Do not interview, do not ask clarifying questions** — the requirements below are
settled. If a genuine contradiction makes a requirement impossible, state it inside the
deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-25_6495d7d.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first playability).
Docs are layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` →
`4-specs`; **earlier tiers govern later ones**. Fetch every file from commit
`6495d7dfe7d2d8887d4bb2ce583074c87fb273e8` — the manifest reflects exactly that tree. The
manifest is a path inventory only; it is not file-content evidence.

### This is the fourth audit pass on one recurring seam

The "spec-0047 surface" is the loaded-world / time-control / TUI-authority seam introduced by
`archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`
(TUI-authoritative world advance, duration completion, no-human progression, actor-known
interval summaries). It has now been audited and remediated three times in a row, and **each
remediation line that claimed closure was followed by an audit that still found critical
foundational violations.** You are the fourth pass.

The lineage (name each as a delta; do not re-commission completed work):

- **First pass** → `reports/0047-foundational-hardening-research-report.md` → implemented as
  spec `0048` (`archive/specs/0048_FOUNDATIONAL_CONFORMANCE_HARDENING_…`).
- **Second pass** → `reports/0047-foundational-hardening-research-report-second-pass.md` →
  implemented as spec `0050`
  (`archive/specs/0050_FOUNDATIONAL_CONFORMANCE_SECOND_HARDENING_…`).
- **Third pass** (your immediate predecessor, target commit `0429a8f`) →
  `reports/0047-foundational-hardening-research-report-third-pass.md` → implemented as spec
  **`0051`** (`archive/specs/0051_FOUNDATIONAL_CONFORMANCE_THIRD_HARDENING_CORE_OWNED_LOADED_WORLD_RUNTIME_…`).

The **third-pass report is your richest seed** — read it in full. Its findings F-01…F-09 are the
specification that spec 0051 was built to satisfy. Its §7 "recurrence and structural-guard
analysis" already diagnosed *why the seam keeps reopening* and proposed the single core-owned
runtime that spec 0051 then implemented. Your job is to determine whether that fix actually
closed the authority class — or whether the seam has reopened a fourth time — and to design the
regression barrier that finally ends the cycle.

### Non-carry-forward posture (this is a re-audit, not a continuation)

Spec 0051's keystone commit is `a3b46c6`; it is merged into your fetch baseline `6495d7d`. The
third-pass findings were the **pre-remediation baseline**; do **not** carry them forward as
still-true. They tell you what 0051 was *supposed* to fix, not what the code now does. The
spec-0047 seams changed wholesale between the third-pass commit `0429a8f` and `6495d7d` — a
verified `git diff --stat` over the load-bearing files shows roughly 780 insertions / 359
deletions across `scheduler.rs`, `app.rs`, `render.rs`, `view_models.rs`, `transaction.rs`,
`events/apply.rs`, and `epistemics/projection.rs`, plus an entirely new
`crates/tracewake-core/src/runtime/{mod,session}.rs` module (`LoadedWorldRuntime`). Treat the
0051-remediated surface as **compliance-unknown**: re-verify every property from the post-0051
code itself. Where 0051 genuinely fixed something, record it as **present** rather than
re-reporting the old defect; where it claimed closure that the code does not bear out, report the
live defect.

The 0051 acceptance artifact
(`archive/reports/0051_foundational_conformance_third_hardening_acceptance.md`) marked all nine
third-pass findings "pass," but **four of them only "pass with standing-miss disposition"**
(F-02 replay reconstruction, F-06 TUI de-authority, F-07 sealed temporal products, F-08
mutation closure), and the **standing mutation campaign completed non-green with 23 missed
mutants**. That artifact is immutable historical evidence; you neither edit it nor treat its
"pass" rows as proof of current conformance.

---

## 2. Read in full (authority order)

Read these before producing. Order is by authority tier; earlier governs later. Entries are
marked **primary (load-bearing conformance target)** or **boundary-awareness (read to bound
scope — what genuinely belongs in this tier vs. another; not a thing to audit or amend)**.

```
docs/README.md — authority order and the layering rule that governs the whole audit. (primary)
```

**Tier 0 — foundation (the constitution; governs everything):**

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — **primary.** INV-001…INV-112; every
  conformance verdict and any amendment proposal must be grounded here. The third-pass report
  leans on INV-001/004/005/009/010/018/041/043/067/068/069/087/088/091/093/094/098/099/101/102/
  105/108/112 — re-verify each against the current code.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **primary.** Event-sourced
  causality + replay/restore contract the runtime and scheduler must satisfy.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **primary.** Waiting as one
  input slot in which other loaded actors / world processes / due consequences advance; embodied
  vs. debug surface boundary.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **primary.**
  The cognition-transaction and truth-firewall doctrine governing the actor decision transaction
  and interval-summary leakage.
- `docs/0-foundation/00,01,04,05,06,07,09,10,11,12,13` — **boundary-awareness.** Read to run the
  tier-fit test for any amendment proposal and to confirm what is out of the spec-0047 surface.

**Tier 1 — architecture:**

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **primary.** The conformance
  index whose loaded-world row the third-pass F-09 flagged as overclaiming; 0051 claims to have
  truthed it. Re-verify the rows against the current production path.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **primary.**
  Replay/save/restore contract behind F-02.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **primary.**
  Firewall/provenance contract behind F-05/F-07.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` —
  **primary.** Pipeline + one core-owned world-step coordinator (F-03/F-04/F-06).
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` —
  **primary.** Canonical cognition transaction the scheduler must consume (F-05).
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` —
  **primary.** Read-only client boundary; TUI must not own/mutate aggregates (F-06/F-07).
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` —
  **primary.** Evidence-honesty / no-decorative-locks contract behind F-08/F-09 and the
  anti-regression gate you will recommend.
- `docs/1-architecture/01,06,07,08,09,11,12,14` — **boundary-awareness.**

**Tier 2 — execution:**

- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` —
  **primary.** Due-work belongs to the core world-step boundary; no parallel possessed-actor
  path; no caller-injected actor lists (F-01…F-06).
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **primary.**
  No-human progression and held-equal possession parity.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **primary.** TUI
  stores/renders the core interval product read-only; embodied/debug split (F-07).
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **primary.**
  Evidence-honesty rule, missed-mutant/timeout discipline, real-public-boundary testing — the
  doctrine home for the standing-gate recommendation.
- `docs/2-execution/00,01,02,03,04,08,09,11,12,13` — **boundary-awareness.** (Note `03` is the
  phase-ladder / certification-sequence doc; read it to place any recommended standing gate
  correctly relative to the existing P0/SPINE/EPI/ORD-LIFE/FIRST-PROOF cert ladder, without
  minting a new gate code.)

**Tier 3 — reference:**

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **primary.** Reviewer pointers
  the report's doc-home recommendations update.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — **primary.** R-27/R-28/R-29 evidence/status
  rows (mint no new risk ID). `02_GLOSSARY.md` — **boundary-awareness.**

**Tier 4 — specs, plus reports / archive (history & lineage; not live authority):**

- `docs/4-specs/SPEC_LEDGER.md` — **primary.** Source/navigation discipline; the 0047/0048/0050/
  0051 archived rows, the `0049MUTWIT` ticket-only record, and the "Next known execution move"
  section. Confirm the new remediation spec routes through the normal ledger process; mint
  nothing.
- `reports/0047-foundational-hardening-research-report-third-pass.md` — **primary (richest
  seed).** Your predecessor's full F-01…F-09 findings, recurrence analysis (§7), and structural
  guard proposal. The pre-remediation baseline.
- `reports/0047-foundational-hardening-research-brief-third-pass.md` — **boundary-awareness.**
  The locked scope/lineage the third pass worked to; confirms conventions.
- `archive/reports/0051_foundational_conformance_third_hardening_acceptance.md` — **primary.**
  What 0051 *claims* it closed, the per-finding closure table, the eight focused mutation
  commands, and the full **23-mutant standing-miss list** (your concrete residual-finding seed).
- `archive/reports/0047_tui_authoritative_world_advance_acceptance.md` — **primary.** The
  original feature acceptance (commit `4228e1e`) that defines the spec-0047 surface boundary.
- `archive/reports/0050_foundational_conformance_second_hardening_acceptance.md`,
  `reports/0048_foundational_conformance_hardening_acceptance.md` — **boundary-awareness.**
  Prior remediation epochs; historical evidence only.
- `archive/specs/0047_…SPEC.md`, `archive/specs/0050_…SPEC.md`,
  `archive/specs/0051_…SPEC.md` — **primary.** The promises each remediation line made; 0051 is
  the one whose claims you re-verify.
- `.cargo/mutants.toml` — **primary.** The existing standing mutation perimeter config the
  anti-regression recommendation must extend, not duplicate.

### Code seams to inspect (read directly at the baseline; do not expect them pasted here)

Audit the post-0051 source, not the third-pass quotes of it:

- `crates/tracewake-core/src/runtime/mod.rs`, `crates/tracewake-core/src/runtime/session.rs`
  (`LoadedWorldRuntime` — the 0051 keystone; the standing-miss list flags
  `rebuild_from_owned_log`, `refresh_actor_current_place_perception`, `assign_proposal_sequence`);
- `crates/tracewake-core/src/scheduler.rs` (`transact_world_one_tick`, `due_loaded_actor_ids`,
  `due_process_invocations`, the actor census, `restore_from_temporal_projection` — flagged
  survivor at `scheduler.rs:550`);
- `crates/tracewake-core/src/events/{envelope,apply,log}.rs` (declared-process event class &
  application; duplicate-`EventId` fail-closed);
- `crates/tracewake-core/src/agent/{transaction,trace,perception}.rs` (actor decision
  transaction consumption; `StuckDiagnostic::deserialize_canonical` flagged at `trace.rs:846/884`;
  perception write path);
- `crates/tracewake-core/src/epistemics/projection.rs`, `crates/tracewake-core/src/projections.rs`
  (`insert_observation`, `embodied_subject_key`, interval-delta/salience; the routed-forward
  `food_source_fact_supersedes` family at `projections.rs:260-263`);
- `crates/tracewake-core/src/view_models.rs` (`EmbodiedViewModel`, `TypedActorKnownIntervalSummary`
  — flagged accessor survivors at `view_models.rs:143/253/257/261/265/277`; check field/constructor
  sealing);
- `crates/tracewake-core/src/replay/{temporal,rebuild,report}.rs`,
  `crates/tracewake-core/src/actions/pipeline.rs`;
- `crates/tracewake-content/src/load.rs` (loaded-world handoff into the runtime constructor);
- `crates/tracewake-tui/src/{app,render,transcript}.rs` (`submit_entry_with_world_advance`
  flagged at `app.rs:259`; `capture_representative_transcript_sections` flagged at
  `transcript.rs:42`; embodied rendering leakage);
- the test / negative-fixture / CI infrastructure: `crates/tracewake-core/tests/*` (esp.
  `world_step_coordinator.rs`, `negative_fixture_runner.rs`, the external-crate negative
  fixtures under `tests/negative-fixtures/`, `anti_regression_guards.rs`, `generative_lock.rs`,
  replay/interval/salience/reservation/parity suites), `crates/tracewake-tui/tests/*`, and
  `.github/workflows/ci.yml`.

---

## 3. Settled intentions (these are decisions, not open questions)

1. **Full independent re-audit of the entire spec-0047 surface.** Re-derive conformance for the
   whole loaded-world / time-control / TUI-authority surface from the post-0051 code. Treat the
   0051 acceptance's "pass" verdicts — especially the four "pass with standing-miss disposition"
   items — as **unverified claims**, not established facts. Rationale: three consecutive
   remediation lines each declared closure and were each followed by a pass that found critical
   violations; a delta-only audit would inherit 0051's blind spots. (You will naturally frame
   findings against the third-pass F-01…F-09 because that is the seed, but a property 0051 marked
   "pass" must earn a fresh **present**/**violation** verdict from the current code, and you must
   remain open to violations none of the prior passes named.)

2. **The anti-regression deliverable is a durable, enforced standing gate — not only per-finding
   guards.** The invocation's central, novel ask is to make it *as impossible as feasible* for
   future code to regress the foundational alignment of this surface. The prior three reports
   already supplied per-finding "strongest practical anti-regression guard" rows; doing only that
   again is insufficient. Your report must additionally design a **first-class, enforced
   regression barrier** for the spec-0047 surface, treating the recurring non-green mutation
   floor as itself the regression vector. Concretely, analyze and recommend (extending existing
   machinery, not inventing parallel frameworks):
   - a **standing CI conformance lane** that exercises the surface only through the public core
     runtime / TUI command boundary (production bootstrap → typed commands → sealed receipts), so
     "read-only client" and "production reachability" become enforced properties rather than
     programmer discipline;
   - a requirement that the **standing mutation perimeter for the spec-0047 files reach and stay
     green** (in-surface survivors driven to zero; out-of-surface survivors explicitly classified
     and routed), rather than a perpetual recorded non-green floor — and the CI/governance
     mechanism that keeps it green;
   - **compile-time unrepresentability** as the load-bearing layer (private fields / crate-private
     constructors / unexported authority tokens on the real production symbols), with source-text
     guards explicitly demoted to topology alarms;
   - where the existing P0/SPINE/EPI/ORD-LIFE/FIRST-PROOF certification ladder
     (`docs/2-execution/03`) is the right structural home for such a standing gate, say so and
     place it correctly **without minting a new gate code, invariant, risk ID, or glossary term**
     (identifier creation remains the repo's own reassess/amend process).

3. **The 23 standing-mutation misses from the 0051 acceptance are first-class findings where
   in-surface.** Dispose of each survivor explicitly. The in-surface families —
   `restore_from_temporal_projection -> None` (`scheduler.rs:550`); the `runtime/session.rs`
   methods `assign_proposal_sequence` / `rebuild_from_owned_log` /
   `refresh_actor_current_place_perception`; the `view_models.rs` accessor returns
   (`sim_tick`/`start_tick`/`stop_tick`/`start_frontier`/`stop_frontier`/`no_new_actor_known_information`);
   `StuckDiagnostic::deserialize_canonical` (`trace.rs:846/884`);
   `submit_entry_with_world_advance` (`app.rs:259`); and
   `capture_representative_transcript_sections` (`transcript.rs:42`) — are on the spec-0047
   surface and must be treated as concrete remediation targets, not laundered. The
   `food_source_fact_supersedes` family (`projections.rs:260-263`) stays **routed-forward** as a
   cross-cutting concern per 0051's own disposition (state this; do not re-route it to a lower
   tier — mutation evidence is cross-cutting, not lowest-tier). Make no equivalence claim about
   any survivor without a defensible semantic argument; "the suite did not kill it" is not proof
   of equivalence.

4. **Static-survey-only.** You cannot run `cargo fmt/clippy/build/test`, replay/golden lanes, CI,
   or `cargo-mutants`. Every statement about current test strength, mutation result, or pass/fail
   is a **preliminary static judgment about source shape, API authority, data flow, and witness
   intent** — explicitly non-certifying. Authoritative pass/fail belongs to the implementing
   session that executes the gates from a clean baseline. Quote any command outcome from a ticket
   or acceptance artifact as a **historical claim by that artifact**, never as a current result.

5. **Conditional foundation/doctrine-amendment branch is retained and independently re-decided.**
   Determine, with evidence, whether any tier (`0-foundation` … `3-reference`) doctrine needs
   amendment for this surface. The prior three passes each concluded **no amendment is
   warranted** (the doctrine already says what the code must do; the defects are the code falling
   below the rules). Reach your own verdict. If — and only if — you find a genuine doctrinal gap
   or contradiction that blocks remediation, recommend the amendment as **substance + home**
   (what doctrine the target doc must own, in your own prose at that tier's altitude, and which
   file/section it lands in) — **never** final paste-ready wording and **never** an invented
   identifier (`INV-###`, gate code, risk ID, glossary term); identifier minting and ratified
   text remain the repo's own reassess/amend process. Do not weaken doctrine to bless
   caller-seeded registries, diagnostic process markers, client-owned aggregates, exact hidden
   time, or a perpetual non-green perimeter — that would be a constitutional inversion.

6. **Extend existing machinery; introduce no new test-framework dependency.** `.cargo/mutants.toml`,
   `anti_regression_guards.rs`, the external-crate negative-fixture pattern + `negative_fixture_runner.rs`,
   `generative_lock.rs`, and the dedicated coordinator/replay/interval/salience/reservation/parity
   suites already exist. Recommendations extend them. Do **not** recommend adding `proptest`,
   `quickcheck`, or another property-testing framework — the required properties are expressible
   with the existing deterministic fixtures, generated corpus, integration seams, negative
   fixtures, and mutation campaigns. No backwards-compatibility shims or alias paths in any
   recommended change.

---

## 4. The task

Determine whether the code implementing the spec-0047 surface — as it stands at commit `6495d7d`
after the spec-0051 remediation — **still violates the foundations**, and if so, specify how to
remediate it, harden it, and **make future regression as close to impossible as feasible**. This
is a cross-cutting **hardening / anti-contamination re-audit** (fourth pass) with a secondary
**foundational/doc-overhaul** branch (conditional amendment determination per §3.5). Re-verify
every load-bearing property of the surface from the current code under the repository's authority
order; render an explicit conformance verdict; for each violation, name the controlling
invariants, the current code state, the conformance verdict, the code + `docs/**` remediation
home, the strongest practical anti-regression guard, and an evidence-honesty check; dispose of
the 23 standing-mutation survivors; and — as the load-bearing contribution of this pass — design
the durable enforced standing gate (§3.2) that ends the four-pass recurrence. Claude Code will
later analyze your report and produce the numbered remediation spec; you do not write the spec.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow every code
seam, test, fixture, and CI definition that bears on the surface. Research online as deeply as
needed — similar implementations, research papers, and prior art — wherever it sharpens the
remediation or the regression barrier. The third-pass report already opened a useful external
lane you may extend: Rust visibility/privacy and the `non_exhaustive` limitation, Schneider's
state-machine replication, Temporal's replay-from-history model, Sabelfeld & Myers on
language-based information-flow (access control ≠ information-flow control), and the cargo-mutants
guidance on missed mutants / timeouts plus the mutation-testing survey on undecidable
equivalence. Keep the external-research lane strictly separate from repository evidence, and cite
every external source that shapes a decision.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every product-behavior
  decision must satisfy it; a genuine divergence requires amending an invariant first, never
  designing against it silently (and amendment substance only — see §3.5).
- **Authority order is absolute:** if execution conflicts with architecture or foundation,
  execution is wrong; if the implementation is more convenient than the accepted gates, the
  implementation is wrong. Never weaken an upstream tier to fit downstream code.
- **No simulation fact may be born from prose.** Preserve event-sourced causality, subjective
  epistemics, ordinary agents, possession parity, fallible institutions, validation/replay, and
  the actor-known truth firewall.
- No backwards-compatibility shims or alias paths in new work.
- Do not edit any archived spec, ticket, acceptance artifact, or passed certification; live-doc
  work is status/navigation/conformance-evidence only, and only after the code and executable
  witnesses exist. Mint no new invariant, gate, risk ID, or glossary term.
- Source-text guards are topology alarms only — never sole proof of atomicity, replay
  continuation, process semantics, one-opportunity-per-actor, information-flow noninterference,
  production reachability, or mutation sensitivity. Those require type boundaries and executable
  behavior.

---

## 7. Deliverable specification

Produce **exactly one** downloadable markdown document, **new** (not a replacement):

```
0047-foundational-hardening-research-report-fourth-pass.md
```

This continues the lineage filenames `0047-foundational-hardening-research-report.md`,
`-second-pass.md`, `-third-pass.md`. It is an **analysis / recommendation report, not a numbered
spec**: Claude Code derives the numbered remediation spec from it afterward, so the
numbering/ledger/epoch rules do **not** apply to you and you must not assign a spec number or
write a `specs/`-style artifact.

**Production mode — always produce, with the verdict as a section (mode i).** Produce the report
unconditionally. Open it with an explicit, evidence-based **verdict**: is the spec-0047 surface
foundationally conformant at `6495d7d`, and is any doctrine amendment warranted. The report's
value survives either outcome — even a clean verdict locks the now-correct properties and
specifies the regression barrier. (Given the residual standing-miss list and the four-pass
history, a *fully* clean verdict is unlikely, but render whatever the current code actually
supports; do not manufacture findings, and record genuinely-fixed properties as **present**.)

Reuse the canonical shape the third-pass report established, scaled to what you find:

1. **Verdict** — conformant / not conformant, with the decisive reasons; plus the explicit
   higher-tier amendment verdict (§3.5).
2. **Disposition table** — one row per finding → primary code/doc target → classification
   (violation / vacuity gap / hardening gap / mutation-survivor disposition / evidence-honesty
   gap) → one-line basis citing invariants.
3. **Method & provenance ledger** — authority order applied; the non-carry-forward posture
   (third-pass findings as pre-remediation baseline); exact-commit acquisition discipline
   (fetch every file by full exact-commit path from `6495d7d`; manifest is inventory only;
   branch/default-branch/code-search/metadata are not target-commit proof); and the
   static-survey limitation.
4. **Per-finding sections** — for each finding: foundational driver (named invariants + governing
   architecture/execution doctrine) → current `6495d7d` code state (cite the real post-0051
   symbols/paths) → conformance verdict → required remediation (code home + `docs/**` home,
   honoring authority order) → strongest practical anti-regression guard (compile-time / behavior
   / differential / mutation) → evidence-honesty check (what a non-vacuous closure witness must
   do, and what would make it vacuous).
5. **Standing-mutation survivor disposition** — survivor-by-survivor / family-by-family handling
   of the 23 misses (in-surface = concrete targets; `food_source` family = routed-forward
   cross-cutting), with no unjustified equivalence claims.
6. **Structural anti-regression / recurrence section (the load-bearing contribution)** — why the
   seam reopened a fourth time despite the 0051 core-owned runtime; and the durable **enforced
   standing gate** design per §3.2 (CI conformance lane through the public boundary; green
   standing mutation perimeter for the surface + its governance; compile-time unrepresentability
   as the load-bearing layer; correct placement relative to the cert ladder without minting
   identifiers).
7. **Foundation & documentation determination** — the amendment verdict with reasoning, and the
   post-implementation live-doc work table (architecture conformance `00`; architecture `02/04/
   05/10/13`; execution `05/06/07/10`; reference `00`/`01` R-27/R-28/R-29 status only; ledger
   through the normal process), explicitly **after** executable closure, editing no archived
   material and minting no identifier.
8. **Recommended closure order**, **open maintainer decisions** (implementation choices inside
   settled doctrine), **self-check**, and **references** (repository evidence and external
   research kept in separate lanes).

**Locked / no-questions instruction:** Produce the deliverable directly as a downloadable
markdown document. Do not interview, do not ask clarifying questions — the requirements above are
final. If a genuine contradiction makes a requirement impossible, state it in the deliverable and
proceed with the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The verdict is explicit and evidence-based, covering both foundational conformance and the
      higher-tier amendment determination.
- [ ] Every file used for target-state claims was fetched from the full exact commit
      `6495d7dfe7d2d8887d4bb2ce583074c87fb273e8`; every §2 path was present in the uploaded
      manifest.
- [ ] The third-pass F-01…F-09 findings were treated only as the pre-remediation baseline and
      re-verified against the current post-0051 code; genuinely-fixed properties are recorded as
      **present**, not re-reported as defects; new violations not named by prior passes were
      sought.
- [ ] Every finding names controlling invariants and both a code home and a `docs/**` home, in
      authority order, weakening no upstream tier.
- [ ] All 23 standing-mutation survivors are disposed survivor-by-survivor; in-surface families
      are concrete targets, the `food_source` family is routed-forward cross-cutting, and no
      survivor is called equivalent without a semantic argument.
- [ ] The report contains a dedicated structural anti-regression section that designs a durable
      **enforced** standing gate (CI conformance lane + green surface mutation perimeter +
      compile-time unrepresentability), not merely per-finding guards.
- [ ] Recommendations extend existing machinery (`.cargo/mutants.toml`, `anti_regression_guards.rs`,
      negative-fixture pattern, `generative_lock.rs`); no new property-testing dependency; no
      backwards-compat shim.
- [ ] No archived artifact is edited; no invariant/gate/risk/glossary identifier is minted; no
      ratified doctrine wording is authored (amendment substance + home only, if any).
- [ ] Static-survey limits are explicit; no current command is asserted green or red; ticket/
      acceptance command outcomes are quoted as historical claims.
- [ ] Repository evidence and external research are in separate lanes; every external claim that
      shapes a decision is cited.
- [ ] The deliverable is exactly one new markdown file named
      `0047-foundational-hardening-research-report-fourth-pass.md`; no spec number assigned.
