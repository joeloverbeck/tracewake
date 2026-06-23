# Research brief — spec-0047 foundational-conformance hardening & anti-regression pass

**You are ChatGPT-Pro Session 2. This prompt is final and self-contained. Do not interview,
do not ask clarifying questions. Produce the deliverable directly. If a genuine contradiction
makes a requirement impossible, state it in the deliverable and proceed with the most faithful
interpretation.**

---

## 1. Context

The uploaded manifest (`manifest_2026-06-23_cb3102e.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust:
event-sourced kernel, subjective epistemics, fallible institutions, TUI-first playability, and a
strict rule that no simulation fact may be born from prose. The workspace is three crates with a
one-way dependency direction: `tracewake-core` (authoritative kernel, zero deps) →
`tracewake-content` (fixtures/loading, depends on core) → `tracewake-tui` (terminal boundary,
depends on core + content). Core must never depend on content or tui.

Docs are layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` →
`4-specs`; earlier tiers govern later ones. If execution conflicts with architecture or
foundation, execution is wrong; if implementation is more convenient than the accepted gates,
implementation is wrong.

**Fetch every file from commit `cb3102ef1993a88b2516cec77a145e97a2baae85` (`cb3102e`) — the
manifest reflects exactly that tree.** This is the verified repository `HEAD` and the **analysis
baseline**.

**Baseline divergence you must respect.** The work under audit is spec **0047** ("TUI Authoritative
World Advance, Duration Completion, and Actor-Known Interval Summaries"). Its archived spec and
acceptance artifact pin implementation/evidence commit `4228e1e2e5efd759e7e7bddb939a599e344742e9`
(`4228e1e`). That commit is **not** the current state of the code: after acceptance the feature
merged to `main` (`ff6c6a8`, merge `ff058c1`) and then a commit titled **"Fixing CI"** (`6a3066c`)
added substantive code to the very seams 0047 owns — `crates/tracewake-core/src/scheduler.rs`
(+274 lines), `crates/tracewake-core/src/actions/pipeline.rs` (+73), and
`crates/tracewake-tui/src/app.rs` (+35). Inspection shows `6a3066c` is **not** a pure CI change:
it is mutation-coverage hardening (centralizing the `advance_until` tick subtraction to a single
mutation-covered helper; replacing a `>` frontier guard with a `max(...)` to kill an equivalent
`>=` mutant; adding a frontier unit test). The net effect: **the 0047 seam files are materially
different at `cb3102e` than at the cited `4228e1e`.** Audit the code **as it stands at `cb3102e`**,
and treat any commit hash cited inside the spec/acceptance/driver reports as audit/provenance only,
never as "the code you are reading."

This brief commissions a fresh hardening / anti-contamination pass; it does not continue an
earlier hardening brief. Its **drivers** are the in-repo analysis report
`reports/tui-human-wait-runs-simulation-research-report.md` (the recommendation-altitude analysis
that originated 0047) and its seed defect note `reports/tui-human-wait-runs-simulation-issue.md`.
The **structural precedent** for 0047's own shape (capability-parity contract, two-hop drift guard)
is `archive/specs/0046_…PARITY_CONTRACT…` — read as context for what 0047 extends, not as a thing
to re-commission.

---

## 2. Read in full (authority order)

Read these before producing. Order is by authority tier. Two reading purposes are marked:
**[primary]** = a load-bearing conformance reference you measure the 0047 code against;
**[boundary]** = read to know what is *out* of scope and to run the tier-fit test (do not audit or
"correct" code these exclude). The user has explicitly mandated reading the **entire** `0-foundation`,
`1-architecture`, `2-execution`, `3-reference`, and `4-specs` tiers — the per-file entries below name
the **primary** anchors; read the remaining files in each tier as **[boundary]** unless your analysis
surfaces a direct 0047 dependency on one.

### Universal
- `docs/README.md` — **[primary]** authority order and the layering rule; the spine of every verdict.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — **[primary]** `INV-001…INV-NNN`, the
  non-negotiable contract; every finding must cite the specific invariant(s) it implicates.

### `docs/0-foundation/*` (full tier — user-mandated)
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **[primary]** the controlling
  doctrine ("Waiting runs the simulation"; staged time controls; actor-known sleep summaries with no
  omniscient leakage). 0047 exists to bring the implementation up to this section.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **[primary]** causal trace,
  replay determinism, temporal authority (`INV-112`); the `TimeAdvanced` tick-ancestry marker and
  empty-tick rebuild claims live here.
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — **[primary]** sleep/work
  as first-class time-spanning durations and causal survival/recovery (`INV-045`); duration
  completion/accounting conformance.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **[primary]** the
  actor-known cognition transaction and truth firewall; the actor-known interval summary vs. debug
  step-report split is governed here.
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — **[primary]** needs/
  charging doctrine behind unified single-charge-per-tick accounting.
- `docs/0-foundation/04`, `09`, `10`, `11`, `12`, `13`, `01`, `00` — **[boundary]** read for tier-fit
  and to bound scope (e.g. `12` first-playable scope, `10` LOD/long-sim out-of-scope edges, `04`
  claims/beliefs/info-flow context).

### `docs/1-architecture/*` (full tier)
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — **[primary]**
  the world-advancing-controls-are-pipeline-commands contract; TUI asks core to advance, never mutates.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — **[primary]**
  proposal/validation/scheduling/no-direct-dispatch; the single authoritative world-step coordinator and
  general body-exclusive reservation conflict live against this contract.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **[primary]** event
  log / replay / projection determinism; replay-visible tick ancestry and the no-second-source-of-truth
  rule for open durations.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **[primary]**
  holder-known contexts, truth firewall, provenance; interval summaries as positively-constructed
  source-bearing deltas.
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` —
  **[primary]** actor-decision / need-accounting seam.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **[primary]**
  validation/observability/acceptance; the proof-obligation home for any recommended anti-regression
  measure.
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **[primary]** the conformance rows
  (e.g. `0017` tick-charge attribution authority, shared open-duration authority) 0047 consumes; check
  whether 0047 is correctly named as a consumer.
- `docs/1-architecture/01`, `06`, `07`, `08`, `09`, `11`, `12`, `14` — **[boundary]** tier-fit and
  scope-bounding (`14` forbidden misreads is useful adversarial context).

### `docs/2-execution/*` (full tier)
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — **[primary]**
  the canonical step's ownership, typed request/result, deterministic phase contract, no-direct-dispatch
  guard.
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **[primary]** ordinary-life
  needs/routines and human/no-human equivalence proof.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **[primary]** epistemic
  view models / possession / debug-quarantine proof; note the staging sentence 0047 §5 says must be
  corrected ("embodied time controls remain unavailable outside debug").
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **[primary]** the
  testing/mutation/observability doctrine; the existing `cargo-mutants` posture and evidence-honesty
  rules that any new anti-regression recommendation must fit.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **[primary]** the
  anti-contamination gates the actor-known summary must pass.
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`,
  `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **[primary]** the
  post-`FIRST-PROOF-CERT` baseline/ladder posture; confirms 0047 is a feature spec outside the cert
  ladder (no certification claim of its own).
- `docs/2-execution/00`, `01`, `08`, `09`, `11`, `12`, `13` — **[boundary]** tier-fit and scope edges
  (`09` golden fixtures/replay acceptance is relevant context if you touch goldens).

### `docs/3-reference/*` (full tier)
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **[primary]** the review checklist;
  0047 §5 routes concise prompts here (all time controls use the same world step; one accounting
  classification per `(actor, need, tick)`; durations close through shared authority; interval output
  is positive actor-known evidence).
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — **[primary]** existing risk entries 0047 is a new
  mitigation surface for (pipeline bypass, epistemic leakage, prose authority, debug leakage,
  replay/projection erosion, TUI-playability erosion, temporal-control relapse). Update existing
  entries only; mint no new risk ID.
- `docs/3-reference/02_GLOSSARY.md` — **[boundary/primary]** confirm existing terms suffice; mint no
  new glossary term.

### `docs/4-specs/*`
- `docs/4-specs/SPEC_LEDGER.md` — **[primary]** the live ledger; confirms the cert-ladder "next move"
  is about certification artifacts, not this feature, and that 0047 received its archived row at
  closeout without latest-main overclaim.
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`,
  `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, `docs/4-specs/README.md` — **[boundary]** live
  spec-tier context and the acceptance-artifact template 0047 packaged against.

### Seed, precedent, and code seams
- `archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`
  — **[primary]** the spec under audit: its §3 verified holdings, §4 remediation requirements, §5
  doctrine-amendment routing, §6 fixtures, §8 implementation constraints, and §10 invariant-alignment
  table are the claims you test the code against.
- `archive/reports/0047_tui_authoritative_world_advance_acceptance.md` — **[primary]** the acceptance
  artifact (changed-files list, evidence ledger, per-requirement `0047-AC-###` mapping, named test
  witnesses). Its commit pin is `4228e1e`; see §1 divergence.
- `reports/tui-human-wait-runs-simulation-research-report.md` — **[primary]** the originating analysis
  (driver). Named symbols authoritative; any line numbers are not.
- `reports/tui-human-wait-runs-simulation-issue.md` — **[primary]** the seed defect note.
- `archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md`,
  `archive/reports/0046-parity-acceptance-artifact.md` — **[primary]** the parity-contract precedent
  0047 extends (PAR-010/PAR-011, two-hop drift guard, 21-capability baseline).
- `.cargo/mutants.toml` — **[primary]** the existing `cargo-mutants` configuration; any
  mutation-testing recommendation must extend, not duplicate, this.

**Code seams — the audit surface (scope: 0047-touched files + their conformance collaborators).**
Read all of these at `cb3102e`:

*0047-touched (changed-files list):*
- `crates/tracewake-core/src/scheduler.rs` — **[primary]** the kernel-owned authoritative one-tick
  coordinator (`advance_world_one_tick`, `advance_until`, the `no_human` module, frontier sync). The
  heart of the audit.
- `crates/tracewake-core/src/projections.rs` — **[primary]** the actor-known interval-summary builder
  (`build_actor_known_interval_summary`).
- `crates/tracewake-core/src/view_models.rs` — **[primary]** `EmbodiedViewModel` and the actor-known
  surface field.
- `crates/tracewake-tui/src/app.rs` — **[primary]** the typed `TuiApp` world-step entry point and
  `UiCommand::WaitOneTick` routing; the TUI-sends-typed-requests-only boundary.
- `crates/tracewake-tui/src/run.rs`, `crates/tracewake-tui/src/input.rs`,
  `crates/tracewake-tui/src/render.rs` — **[primary]** command loop, parsing of wait vs. continuation/
  advance-until, and the Hop-2 exhaustive renderer disposition.
- `crates/tracewake-core/tests/world_step_coordinator.rs`,
  `crates/tracewake-core/tests/anti_regression_guards.rs`,
  `crates/tracewake-content/tests/golden_fixtures_run.rs`,
  `crates/tracewake-tui/tests/command_loop_session.rs`,
  `crates/tracewake-tui/tests/embodied_flow.rs`,
  `crates/tracewake-tui/tests/parity_adversarial.rs`,
  `crates/tracewake-tui/tests/playable_capability_parity.rs`,
  `crates/tracewake-tui/tests/parity/*` — **[primary]** the existing proof surface and anti-regression
  scaffolding; assess what each genuinely pins vs. what is asserted-by-text-only.

*Conformance collaborators (unchanged by 0047 but relied upon for correctness — a foundational
violation often lives at the boundary with these):*
- `crates/tracewake-core/src/need_accounting.rs` — **[primary]** the `(actor, need, tick)`
  classification authority (`classify_actor_tick_regimes[_with_start]`); single-charge-per-tick.
- `crates/tracewake-core/src/events/envelope.rs` — **[primary]** `TimeAdvanced` envelope
  classification (`cause_required`, `EventStream::World`, physically-mutating) vs. the spec's
  `INV-010` cause-model stance; `is_duration_terminal`. The spec (§4.5) flagged this as an explicit
  decision the implementation owed — verify how it was resolved.
- `crates/tracewake-core/src/actions/pipeline.rs` — **[primary]** the body-exclusive reservation-conflict
  predicate (the general sleep-then-wait rejection).
- `crates/tracewake-core/src/replay/rebuild.rs`, `crates/tracewake-core/src/replay/report.rs` —
  **[primary]** replay/rebuild of frontier, ledger, recovery, and interval summary.
- `crates/tracewake-core/src/agent/actor_known.rs`, `crates/tracewake-core/src/epistemics/projection.rs`
  — **[primary]** holder-known projection inputs the interval summary must be built from.
- `crates/tracewake-core/src/controller.rs`, `crates/tracewake-core/src/agent/no_human_surface.rs`,
  `crates/tracewake-core/src/debug_reports.rs` — **[boundary/primary]** controller-origin metadata, the
  no-human runner surface refactored onto the coordinator, and the exact debug step report that must
  stay structurally separate from the actor-known summary.

---

## 3. Settled intentions (locked — do not re-open)

These resolve every clarifying question. They are decisions, not options.

1. **Always render a determination/verdict.** Whatever the outcome, produce a clearly-labeled,
   evidence-based verdict: does the 0047 implementation, as it stands at `cb3102e`, foundationally
   diverge — and where, with what severity. The verdict is returned regardless of mode (below).
2. **Form follows the verdict (deliverable mode).**
   - If the audit finds **one or more foundational violations, or warranted hardening / anti-regression
     gaps**, produce the **downloadable** markdown report named in §7.
   - If the audit is **fully clean** (no violation and no warranted hardening), produce the reasoned
     determination **inline in your response, with NO downloadable file**. Do not manufacture a file to
     have one.
3. **Analysis scope = 0047-touched seams + their conformance collaborators** (the two code-seam groups
   in §2). This is **not** a whole-kernel re-audit — the P0-CERT / SPINE-CERT / EPI-CERT /
   ORD-LIFE-CERT / FIRST-PROOF-CERT ladder already certified the kernel at large; do not re-tread it.
   A finding outside the 0047 surface is in scope **only** if 0047 directly depends on it for
   foundational conformance, and you must say so explicitly.
4. **Anti-regression is comprehensive across the 0047 capability surface, not gated on found
   violations.** The user's stated goal is to make future foundational regression *as infeasible as
   feasible*. So for each load-bearing 0047 property — the single semantic definition of a loaded-world
   tick; the actor-known-summary ⇄ debug-report firewall split; single-charge-per-`(actor, need, tick)`
   accounting; log-derived (single-source-of-truth) open-duration discovery; general body-exclusive
   reservation enforcement; replay-visible `TimeAdvanced` ancestry and empty-tick rebuild; human/no-human
   differential equivalence; no-direct-dispatch at the TUI boundary — assess whether a future careless
   change could silently regress it, and recommend the **strongest practical guard**, **even where the
   property is currently correct**.
5. **Account for existing anti-regression scaffolding; do not re-recommend it.** The repo already runs
   `cargo-mutants` (`.cargo/mutants.toml`, a CI lane, doctrine in execution `10`) and source-text guard
   tests (`crates/tracewake-core/tests/anti_regression_guards.rs`). The post-acceptance commit `6a3066c`
   already added mutation-coverage hardening to the coordinator. Survey what is **already** guarded,
   then recommend only the **delta** — and say plainly where a property is already adequately locked.
6. **Evaluate the full anti-regression menu and recommend per-property with rationale.** Candidates:
   invariant/property-based tests (the repo uses **none** today — no `proptest`/`quickcheck`; treat
   adoption as a real, justify-or-reject option, not a default), targeted `cargo-mutants` scope
   extensions, source-guard (`include_str!`) tests, replay/differential goldens, conformance-row
   additions, and typed-API shapes that make the violation unrepresentable. Prefer guards that make a
   regression **fail to compile or fail a deterministic test** over guards that merely document intent.
   Justify each choice against cost and against the repo's evidence-honesty doctrine (execution `10`):
   no guard that asserts a property it does not actually exercise.
7. **Foundation/doc corrections are in scope as first-class findings — substance + home only.** If the
   audit shows a foundational document is itself wrong, stale, or contradicts a higher tier (e.g. the
   execution `07` staging sentence the spec already flagged for correction, or a doc that under-specifies
   what 0047 had to implement), record it as a finding: **what** is wrong, **which file/section** owns
   the fix, and **what doctrine that file must own** — in your own prose at the right altitude for that
   tier. Do **not** write paste-ready ratified wording and do **not** invent identifiers (`INV-###`,
   gate codes, risk IDs, glossary terms); those remain the repo's own reassess/amend process.
8. **Immutable artifacts.** Do not propose amending the archived spec 0047, its acceptance artifact, or
   any passed certification. If the *spec itself* mis-stated something that the code then faithfully
   implemented (a defect that originates in the spec), record it as a finding routed to doctrine/owner,
   not as a spec edit.
9. **Baseline is `cb3102e`; the spec/acceptance commit `4228e1e` is provenance only** (§1). Every code
   claim you make must be true of the tree at `cb3102e`.
10. **Single consolidated report** (not a per-tier split): this is one cross-cutting hardening pass over
    one feature surface, not a multi-tier doc cascade.

---

## 4. The task

This is a **hardening / anti-contamination** pass. Spec 0047 was feature-forward work; the concern is
that, in shipping a TUI-facing time-control capability, the implementation may have introduced
foundational violations — a private possessed-actor clock, omniscient leakage into the actor-known
summary, a second source of truth for open durations, double-charging, a direct-dispatch or
direct-mutation shortcut at the TUI boundary, a replay-ancestry gap, or a doctrine drift — that would
**contaminate development moving forward**. Your job: read the governing doctrine (all five tiers) and
the 0047 code at `cb3102e`, determine with evidence whether such violations exist, and — for every
load-bearing 0047 property whether or not it is currently violated — specify the fixes (where violated)
and the **anti-regression measures** that make future regression as infeasible as practical. Deliver
*substance + home* (what must change, where it lands, and the guard that locks it), not paste-ready
ratified text or invented identifiers. The actual code/doc changes are the repository's later
implementation step; this report commissions and scopes them.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — follow symbols across the
seam, read collaborating modules and tests, and verify every claim against the tree at `cb3102e`
(named symbols are authoritative; cited line numbers in any report or spec are not). Research online
as deeply as needed — similar event-sourced / causal-simulation hardening, deterministic-replay
verification, property-based and metamorphic testing for simulations, mutation-testing strategy, and
truth-firewall / information-flow-confinement techniques — wherever it sharpens a recommendation. Cite
sources for any external claim that shapes a decision. **You cannot execute code** (no `cargo test`,
`cargo build`, `cargo mutants`, or replay): treat any pass/fail you infer from reading as a
*preliminary, non-authoritative static survey*, mark it as such, and route authoritative
execution-backed verification to the implementing session.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every product-behavior
  judgment must satisfy it; a genuine divergence requires amending an invariant first, never designing
  against it silently. Cite the specific invariant(s) each finding implicates.
- Authority order: if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong.
- The invariants 0047 leans on most (verify alignment, do not assume it): `INV-112` (temporal
  authority), `INV-018`/`INV-092` (deterministic replay), `INV-045` (ordinary survival is causal),
  `INV-103`/`INV-104` (scheduler is not cognition; no direct dispatch), `INV-005`/`INV-006`/`INV-094`/
  `INV-108` (possession parity / cognition-neutral), `INV-067`/`INV-099`/`INV-101`/`INV-102`
  (actor-known reality; truth firewall; sealed context; provenance), `INV-010` (every event needs a
  cause model — note the `TimeAdvanced` cause-exempt-vs-cause-required tension the spec flagged),
  `INV-091` (no-human tests mandatory).
- Anti-contamination: no simulation fact may be born from prose; preserve event-sourced causality,
  subjective epistemics, ordinary agents, possession parity, fallible institutions, validation/replay.
  The actor-known interval summary must be **positively constructed** from source-bearing holder-known
  deltas — never a redacted global diff, never the omniscient "nothing happened."
- No backwards-compatibility shims or alias paths; the TUI sends typed requests only and never applies
  events, mutates state, calls terminal/completion builders, or reuses the debug no-human runner for
  gameplay.
- Mint nothing: no new invariant, gate code, risk ID, or glossary term; reopen no passed certification;
  amend no archived spec.

---

## 7. Deliverable specification

**This is a determination-plus-conditional, recommendation-report deliverable — NOT a numbered
`specs/` spec.** Do not apply spec numbering / ledger / epoch rules; this is not a `docs/4-specs/`
artifact.

- **Always:** render the §3.1 determination/verdict.
- **If violations or warranted hardening exist (positive verdict):** produce **one new, downloadable**
  markdown document named **`0047-foundational-hardening-research-report.md`** (a *new* file; it
  replaces nothing). Recommended structure:
  1. **Verdict** — the headline determination (clean / divergences found / hardening warranted), with
     a one-paragraph basis.
  2. **Disposition table** — one row per finding: finding → target file or doc → severity
     (violation / hardening-gap / doc-correction) → one-line basis (invariant/doctrine cited).
  3. **Method & provenance ledger** — what you read, the `cb3102e` baseline, the `4228e1e` divergence,
     the static-survey caveat, and the existing anti-regression scaffolding you accounted for.
  4. **Per-finding sections** — for each: the foundational driver (invariant/doctrine cited) → current
     code state at `cb3102e` (named symbols) → conformance verdict → the fix (substance + home, where
     violated) → the **anti-regression measure** that locks it (mechanism + why it is the strongest
     practical guard + how it fits evidence-honesty doctrine) → its target file/test home.
  5. **Comprehensive anti-regression layer** — per §3.4/§3.6, the per-property guard recommendations
     across the whole 0047 surface, including properties currently correct, with a column stating what
     is **already** guarded (so the repo implements only the delta).
  6. **Foundation/doc-correction findings** — substance + home, no ratified wording, no minted
     identifiers (§3.7).
  7. **Open questions** — anything genuinely needing a maintainer decision.
  8. **References** — cited external sources.
- **If fully clean (negative verdict):** author **no file**; return the evidence-complete determination
  inline as your response, explicitly stating the already-correct properties it confirms and that no
  change is warranted.

**Locked / no-questions:** produce the deliverable directly. Do not interview, do not ask clarifying
questions — the requirements above are final. If a genuine contradiction makes a requirement
impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- The verdict is explicit and evidence-based; the deliverable form matches it (downloadable report iff
  positive; inline-only iff clean) — exactly per §7.
- Every code claim is true of the tree at commit `cb3102e`, not `4228e1e`; named symbols, not line
  numbers, carry the claims.
- Every finding cites the specific invariant or doctrine section it implicates, by tier.
- No recommendation mints an `INV-###`, gate code, risk ID, or glossary term, writes paste-ready
  ratified doc wording, amends an archived spec/acceptance/passed certification, or proposes a
  backwards-compat shim.
- Anti-regression recommendations account for the existing `cargo-mutants` + `anti_regression_guards.rs`
  scaffolding and recommend only the delta; each proposed guard actually exercises the property it
  claims to lock (no evidence-dishonest assertion).
- Every external claim that shaped a decision is cited.
- Analysis stayed within the 0047 surface + its conformance collaborators; any out-of-surface finding
  is explicitly justified by a direct 0047 dependency.
- The `cb3102e` baseline contains every file named in §2 (it does — confirmed at brief authoring).
