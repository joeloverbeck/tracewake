# Research Brief — Phase 3A Ordinary-Life (Needs / Routines / No-Human Day) Alignment & Anti-Contamination Hardening (Tracewake)

> **For ChatGPT-Pro Session 2 (the deep researcher).** You are *locked*: produce the deliverable
> directly. Do not interview, do not ask clarifying questions — the requirements below are final.
> Upload bundle = this prompt + the manifest `manifest_2026-06-09_461308a.txt`.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-09_461308a.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust: an
event-sourced kernel, **subjective epistemics** (agents act only on their own beliefs, observations,
and modeled channels — never on hidden ground truth), fallible institutions, ordinary agents, and a
TUI-first client surface where every meaningful change leaves a replayable trace. The workspace is
three crates with a strict one-way dependency direction: `tracewake-core` (authoritative kernel,
**zero dependencies**) → `tracewake-content` (fixtures / loading / schema validation, depends on
core) → `tracewake-tui` (terminal boundary, depends on core + content). Core must **never** depend on
content or tui.

Docs are layered authority: `docs/0-foundation` → `docs/1-architecture` → `docs/2-execution` →
`docs/3-reference` → `docs/4-specs`. **Earlier tiers govern later ones.** If execution conflicts with
architecture or foundation, execution is wrong; if implementation is more convenient than the accepted
gates, implementation is wrong. The whole adjudication rule lives in `docs/README.md`.

**Fetch every file from commit `461308af95940d59c2d56d32ffead35631c9db72`** — the manifest reflects
that exact tree (verified repo HEAD). Construct every raw URL as
`https://raw.githubusercontent.com/joeloverbeck/tracewake/461308af95940d59c2d56d32ffead35631c9db72/<manifest path>`.
The manifest is **path inventory only** — never source text or authority. Archived specs cite their own
historical baseline commits inside their evidence ledgers (e.g. the Phase-3A implementation spec
`archive/specs/0005_…` cites `8fa8d1b473be848a879457d9a5dd06a2c86e24b3`); those are *that spec's own*
provenance and **predate** the current HEAD and every later merge — including the post-overhaul
doctrine rewrite and the Phase-1 / Phase-2A re-hardening campaign. **Ignore them as fetch targets —
fetch everything from `461308a…`.** Branch names, default-branch lookups, repository metadata, and
code-search snippets are not proof of target-commit content.

**This brief advances an iterative anti-contamination campaign into a NEW block — the Phase 3A
ordinary-life substrate.** The maintainer re-hardens each coherent code block until nothing more needs
hardening; past projects "devolved into entropy" when later implementers built on **non-aligned** code,
so beyond finding and correcting any current violation, the deliverable must, *for each finding*,
specify a structural mechanism that makes it **impossible (to the extent practical)** for future code
to re-introduce the violation.

The campaign so far (one unified, contiguous staging series `0002 → 0013`, all completed, merged, and
archived under `archive/specs/`):

- `0002 → 0004` — the original Phase-1 / Phase-1A kernel/TUI/event-log/replay and Phase-2A epistemic
  *implementation* packages, plus the pre-overhaul Phase-3A implementation (`0005`) and its
  pre-overhaul Phase-3A hardenings (`0006`, `0007`, `0008`). These **predate the upper-tier doctrine
  overhaul** and are *historical evidence only*.
- `0009 → 0012` — the post-overhaul re-hardening of the **Phase-1 / Phase-1A physical/kernel/TUI
  spine** (TUI proof-surface, spine anti-contamination + structural-lock layer, third-pass lock-layer
  re-audit, doc↔code alignment-conformance).
- `0013` — the post-overhaul **first alignment + hardening + anti-contamination pass over the Phase-2A
  epistemic substrate**, compressed into one combined pass. Its research brief is a sibling of this one
  (`reports/phase2a-epistemic-substrate-hardening-research-brief.md`) and its spec
  (`archive/specs/0013_…`) is the **closest precedent and template** for *this* deliverable.

**This pass is the FIRST post-overhaul alignment + hardening + anti-contamination pass over the
Phase 3A ordinary-life block** — needs, intentions, routines (HTN-like methods), bounded local
planning, the ordinary actions (`sleep` / `eat` / `work` / `continue_routine` / extended `wait`), the
no-human-day driver, and their diagnostics/replay surface. It is the analogue, for ordinary life, of
what `0013` did for the epistemic substrate. It is *not* a continuation of the Phase-1 lock-layer
lineage and *not* a re-audit of already-hardened spine or epistemic code. The pre-overhaul Phase-3A
hardenings `0006/0007/0008` are historical context — **not** this pass, and **not** certification under
the current doctrine. The live execution tier brackets Phase 3A as **landed-but-uncertified history**
(`docs/2-execution/01` and `…/03`: Phase 3A maps to the **`ORD-LIFE-CERT`** gate, *not yet passed*).

Treat this brief as a **delta**, not a cold start: the Phase-1 spine and the Phase-2A epistemic
substrate are already hardened — do not re-commission that work.

---

## 2. Read in full (authority order)

Read these completely, in this order, before producing anything. Each tier's **primary**
(Phase-3A-load-bearing) documents are called out; the remaining documents are read for
**boundary-awareness** — so that Phase-1 spine doctrine and Phase-2A epistemic doctrine (already
certified-as-hardened) and Phase-4 institutions doctrine (later, uncertified) are recognized as **out
of scope** and never mis-audited or "corrected." Confirm actual module/file paths against the manifest
before citing; do not assume names.

### Doctrine — foundation (`docs/0-foundation/`)

- `docs/README.md` — **boundary/adjudication:** the authority-layering rule and how to resolve
  cross-tier conflicts. The audit's adjudication method.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — **primary.** The `INV-001…INV-110` contract.
  **Every finding and every requirement must cite the exact invariant(s) it bears on, verified against
  this file — never against numbers quoted in the archived Phase-3A specs, whose numbering predates the
  overhaul. Re-derive the exact numbers from this file.** The Phase-3A-load-bearing families:
  **INV-099–110** (truth firewall / cognition authority — *truth may validate actions but may not plan
  them*; hidden-truth cognition forbidden; actor-known context sealed; cognition inputs require
  provenance; **scheduler is not a cognition authority**; **routines and needs do not dispatch
  primitive actions directly**; decision traces are authoritative; validation failure feeds replanning
  without leakage; debug omniscience quarantined; possession is cognition-neutral); **INV-032–041**
  (BDI separation, durable intentions, defeasible routines, HTN methods are procedures not scripts,
  bounded local planning, **needs are pressures not puppet strings**, decision traces, no LLM agent
  brains); **INV-043–048** (action validation parity, conditional affordances, causal survival,
  real work/wages/food, intentional/costly/fallible search, causal travel); **INV-003/004/005/006/007**
  (ordinary life before adventure, authoritative world ignores the human, only normal controller
  binding, possession transfers no knowledge, every player action NPC-possible); the validation family
  **INV-091–098** (no-human tests mandatory, deterministic replay tested, actor-knowledge leakage is
  high-severity, possession parity tested, harsh feature acceptance).
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — **primary, the controlling
  agent doctrine:** BDI separation, needs as bounded pressures, intentions as durable commitments with
  a full lifecycle, routines as defeasible HTN methods, bounded local GOAP/STRIPS planning from
  actor-known facts only, event-driven replanning, the LLM boundary, and the **canonical actor decision
  transaction**. The bar the substrate is measured against.
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — **primary, the controlling
  action/ordinary-life doctrine:** action parity (same validator/pipeline for NPCs and humans), the
  action pipeline shape, conditional affordances, sleep/eat/work/wait/travel/search as *causal* (not
  time-skips or puppet strings), hunger as pressure, and the no-human ordinary-life requirement.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **primary, the
  constitutional cognition boundary:** the canonical actor-decision transaction stages, what the
  sealed actor-known context includes/excludes, provenance classes and their cognition-use
  restrictions, the **proposal-vs-validation split**, the **scheduler boundary**, and debug
  quarantine. The single most load-bearing firewall doctrine for this block.
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — **boundary/support:** the
  epistemic substrate (truth/belief/record separation, typed propositions, provenance) that Phase-3A
  cognition *consumes*. Already hardened by `0013` — read to know what Phase-3A may rely on, not to
  re-audit.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **primary for the Phase-3A view
  surface:** embodied mode shows actor-known state only; debug is non-diegetic; possession is input
  binding only and must not reset needs/intentions/routines.
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — **primary for fixture
  discipline:** forbidden authoring (quest beats, scripts, drama directors, hidden "player does X",
  reward spawns independent of procedure); required authoring (primitive actions, affordances, needs,
  HTN methods, rules, scenario seeds). The bar the `no_human_day_001` and sibling fixtures meet.
- `docs/0-foundation/00_FOUNDATION_INDEX.md` — **boundary-awareness:** anti-drift / reading-order /
  "do not cherry-pick" rules and the four inseparable boundaries (event, epistemic, action, cognition).
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **boundary-awareness:** the
  first-playable scope and required ordinary-life substrate; bounds what Phase 3A must already cover.

### Doctrine — architecture (`docs/1-architecture/`)

- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` —
  **primary, the authoritative subsystem contract** for Phase-3A autonomy: the canonical transaction
  (one review boundary per arrow), candidate generation from actor-known pressures only, intention
  lifecycle, HTN method selection, bounded local planning, ordinary-action proposal with ancestry.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — **primary:**
  the pipeline every Phase-3A action must flow through, and the **scheduler limits** (may order ticks /
  trigger context / choose eligible actors / call transactions / enqueue proposals / record metrics;
  **may not** emit primitives from raw state, need thresholds, or routine labels, read true targets, or
  count markers as progress) and **validation-truth limits** (accept/reject/resolve only; may not
  choose the next goal or reveal hidden truth in actor-visible summaries).
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **primary:** the
  holder-known context packet shape, provenance classes and use-restrictions, context-sealing
  immutability, and the scheduler boundary — the architecture realization of the firewall the Phase-3A
  `actor_known` / `transaction` code must satisfy.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **primary:**
  required artifact families (event log, replay report, holder-known context packet, decision trace,
  stuck diagnostic, validation report, no-human metrics, anti-regression guards), the acceptance gate
  groups, and the review checklist the deliverable's acceptance artifact must instantiate.
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **boundary-awareness:** the eight
  universal conformance questions every feature must answer.

### Doctrine — execution (`docs/2-execution/`)

- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **primary, the controlling
  gate.** Defines **`ORD-LIFE-CERT`**: needs/intentions/routines/stuck are event-sourced &
  replay-reconstructable; candidate generation is actor-known only; method/planning cite provenance;
  the scheduler cannot dispatch primitives from needs/routines; validation rejects forged/stale
  parameters; no-human metrics count real progress/waits/stuck; debug can compare input-vs-truth
  without contaminating; replay rebuilds metrics/diagnostics; fixture failures name the responsible
  layer. Also the **required adversarial fixtures** (integrated no-human day, food unavailable,
  routine no-teleport, possession-does-not-reset-intention, hidden-truth planning, planner trace,
  routine blocker) and the definition of **behavioral progress**.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **primary, the
  per-spec anti-contamination gate.** The **`TFW`** checklist every spec must satisfy (name each holder
  doing cognition; name the sealed-context builder; prove context excludes validation/debug/fixture
  truth; prove selected goal/method cites only context inputs; prove the validator cannot suggest
  hidden targets; prove debug cannot be parsed by embodied code; prove replay rebuilds context) and the
  **forbidden patterns** (need-threshold chooses true food, routine-label chooses true workplace,
  scheduler emits a primitive, planner searches hidden edges, rejection reveals hidden reason, fixture
  carries culprit/quest_state fields, …). The deliverable's anti-contamination section is built here.
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **primary for framing:**
  the gate order `P0-DOC → P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT → FIRST-PROOF-CERT →
  PHASE-4-ENTRY`; Phase 3A maps to `ORD-LIFE-CERT`, which `0005–0008` did **not** pass; no automatic
  Phase 4 without it.
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — **primary
  for posture:** archived specs `0005–0008` are historical evidence only; the `P0-CERT` baseline gate;
  the rule that archived specs are cited as history, never as certification.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **primary for the
  proof shape:** test families, the diagnostic standard (name gate/layer/component/input-source/
  actual-source/event IDs/hidden-truth/replay-divergence/remediation), the enumerated **responsible
  layers** (…`candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`,
  `proposal_construction`, `scheduler`, `action_validation`, …), and observability obligations.

### Reference (`docs/3-reference/`)

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — **primary watchlist:** R-06 (protagonist gravity),
  R-07 (quest relapse), R-08 (action-pipeline bypass), R-09 (epistemic leakage) — the exact relapse
  modes Phase-3A code can regress into; each finding should map to one.
- `docs/3-reference/02_GLOSSARY.md` — **boundary:** canonical terms (`actor-known`, `holder-known
  context`, `truth firewall`, `proposal`, `routine`, `intention`, `needs`, `lead`, `notice`) so the
  deliverable uses doctrine vocabulary exactly.

### Spec layer & predecessors (`docs/4-specs/`, `archive/`, `reports/`)

- `docs/4-specs/SPEC_LEDGER.md` — **primary for placement/posture:** the active/archived spec table,
  the authority posture, and the source-discipline rules (commit hashes are provenance only; manifests
  are path inventory only). Confirms `ORD-LIFE-CERT` / `P0-CERT` as the named next moves.
- `docs/4-specs/README.md` — **primary for placement:** the live spec-layer index and conventions.
- `archive/specs/0013_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md` —
  **boundary/template — the closest precedent.** Read for *structure and rigor* (evidence ledger,
  per-finding invariant citation, anti-contamination lock layer, scoped-evidence acceptance wording),
  **not** to re-audit Phase-2A code.
- `archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md` —
  **boundary/history:** what the Phase-3A slice was *intended* to be (need set, routine families, action
  set, the `no_human_day_001` fixture, the harsh-diagnostic posture, the prior-art vocabulary
  BDI/HTN/GOAP/F.E.A.R./explainable-agents). Pre-overhaul; not live authority.
- `archive/specs/0006_…`, `0007_…`, `0008_…` (Phase-3A hardenings) — **boundary/history:** the gaps the
  *pre-overhaul* hardenings already exposed (actor-known planning, lifecycle, replay, forged/stale
  proposal parameters, marker actions, friendly tests, overclaiming docs). Useful as a checklist of
  known weak points to re-verify under current doctrine; not certification.
- `reports/phase2a-epistemic-substrate-hardening-research-brief.md` — **boundary/sibling:** the
  immediately-preceding brief in this campaign; mirror its locked, self-contained style.
- `archive/reports/PHASE_3A_IMPLEMENTATION_AUDIT.md` and `archive/reports/PHASE_3A_STATUS_ERRATA.md` —
  **boundary/history:** prior Phase-3A audit findings and status corrections; treat as leads to
  re-verify against the current tree, not as current truth.

### Code seams to inspect (read directly from the manifest tree; not a full-tier mandate)

Inspect these as deeply as the audit requires; they are the **in-scope Phase-3A surface** (§3):

- **Agent core:** `crates/tracewake-core/src/agent/{need,intention,routine,methods,htn,planner,
  actor_known,transaction,decision,trace}.rs`.
- **Ordinary actions:** `crates/tracewake-core/src/actions/defs/{sleep,eat,work,continue_routine,
  wait}.rs`, plus `…/actions/registry.rs` and `…/actions/pipeline.rs` (Phase-3A registration and the
  shared pipeline those actions flow through).
- **Scheduler & projections (Phase-3A-introduced surface inside shared files):**
  `crates/tracewake-core/src/scheduler.rs` (the `no_human` driver, duration-completion events, passive
  need-delta events) and `crates/tracewake-core/src/projections.rs` (`no_human_day_metrics`).
- **Fixtures:** `crates/tracewake-content/src/fixtures/{no_human_day_001,food_unavailable_replan_001,
  routine_no_teleport_001,routine_blocked_diagnostic_001,no_hidden_truth_planning_001,planner_trace_001,
  possession_does_not_reset_intention_001,sleep_eat_work_001,ordinary_workday_001,
  no_human_advance_001,no_human_epistemic_check_001}.rs`.
- **Test/lock gates:** `crates/tracewake-core/tests/{golden_scenarios,no_human_capstone,
  hidden_truth_gates,acceptance_gates,negative_fixture_runner,anti_regression_guards,
  doc_invariant_references,event_schema_replay_gates}.rs`.
- **TUI view surface:** `crates/tracewake-tui/src/debug_panels.rs` and the embodied/debug view models
  (locate via manifest) — to verify embodied views expose actor-known Phase-3A state only and debug is
  non-diegetic.

---

## 3. Settled intentions

These decisions are **final** — they are why you are locked. Do not re-open them.

1. **This is the first post-overhaul alignment + hardening + anti-contamination pass over the Phase 3A
   ordinary-life block.** Its scope is the *needs / intentions / routines / bounded-planning / ordinary
   actions / no-human-day* substrate, exactly as `0013` was for the epistemic substrate.

2. **Scope boundary = all Phase-3A-introduced surface, wherever it lives.** IN SCOPE: the agent modules,
   the five ordinary-action defs + their registration + the shared pipeline *as exercised by Phase-3A
   actions*, the **no-human driver / duration-completion events / passive-need deltas in `scheduler.rs`**,
   the **`no_human_day_metrics` in `projections.rs`**, the `no_human_day_001` fixture and its Phase-3A
   siblings, and the Phase-3A test/lock gates. OUT OF SCOPE (boundary-awareness only — read to bound
   scope, never re-audit or "correct"): Phase-1 / Phase-1A spine internals (hardened by `0009–0012`), the
   Phase-2A epistemic substrate internals (hardened by `0013`), and Phase-4 institutions (later,
   uncertified). Where Phase-3A code *consumes* an already-hardened seam (e.g. the sealed
   holder-known/actor-known context, the event-log/replay kernel), audit only the **Phase-3A usage** of
   it, not the seam itself.

3. **Artifact mode = produce-only-if-positive.** First produce a clearly labeled, evidence-based
   **determination / verdict**. Author the spec **iff the verdict is positive**, where *positive* means
   you found **any** of: (a) a foundations/invariant violation or drift in in-scope Phase-3A code,
   (b) a warranted hardening of in-scope code, or (c) an anti-contamination gap (a path by which future
   code could regress a currently-correct Phase-3A property). Only a verdict that is **both** fully
   aligned **and** already maximally locked against regression yields **no spec** — in which case return
   the reasoned, evidence-complete determination and author no spec file. (Given the project's standing
   anti-contamination mandate, a fully-clean-and-fully-locked verdict is unlikely; reach it only if the
   evidence genuinely supports it.)

4. **Anti-contamination is per-finding and structural.** For every violation fixed *and* for every
   already-correct property worth protecting, specify a concrete structural mechanism that makes future
   regression impossible to the extent practical — choosing from the lock vocabulary already used in
   this repo: compile-fail fixtures / negative fixtures, source-guard scanners and banned-API/token
   scanners (`anti_regression_guards.rs`), a `clippy.toml` disallowed-types/methods profile, named
   conformance indices, replay/schema gates, doc↔invariant reference linters, and sealed-context
   private fields with accessor-only construction. Prefer mechanisms that *fail the build or the test
   suite* on regression over mechanisms that merely document intent.

5. **Cert framing = scoped evidence toward `ORD-LIFE-CERT`.** Frame the spec as contributing scoped,
   audited evidence toward the named `ORD-LIFE-CERT` execution gate (and, where relevant, `P0-CERT`),
   while **explicitly disclaiming full-project, later-phase, and latest-main certification** — mirroring
   `0013`'s `EPI-CERT` posture verbatim in intent. Use the live execution posture categories where
   applicable. Do **not** assert that `ORD-LIFE-CERT` is passed.

6. **Number & placement (settled).** The deliverable stages as
   **`specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`**.
   The staging series is a single contiguous sequence whose current highest filename is `0013`
   (`archive/specs/`), so the next staging number is `0014`. Per this repo's convention, hardening specs
   are staged under `specs/` and archived to `archive/specs/` on acceptance — they are **never** promoted
   into the live `docs/4-specs/` tier. `assumption:` the exact filename slug above may be tightened by the
   maintainer; the **number `0014`** and the `specs/` staging location are fixed.

7. **Doctrine is never amended by this spec.** A hardening spec may operationalize higher-tier doctrine;
   it may not amend a constitutional invariant, redefine a gate, or weaken an execution gate. If you find
   a genuine doctrine *gap* (the code is correct but no invariant/gate covers it), record it as a
   recommendation for a future doctrine amendment — do not silently design against the existing tiers.

---

## 4. The task

Audit the in-scope Phase 3A ordinary-life substrate (§2 code seams, §3 boundary) against the full
authority stack (`docs/0-foundation` → `docs/4-specs`) and determine whether it aligns **perfectly**
with current doctrine. Produce an evidence-based **determination** citing exact invariants and the
`ORD-LIFE-CERT` / `TFW` gates. Where the substrate diverges, drifts, is under-hardened, or is
exploitable by future code, **specify the fixes and the structural anti-contamination mechanisms** that
both correct the issue and make its recurrence impossible. The deliverable is a **hardening /
anti-contamination spec** (secondary: a new numbered staging spec) in the lineage of `0013`. This is an
audit-and-remediation-design task — you specify the work precisely; you do not implement it.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — follow `mod.rs` trees, trace
the no-human driver call graph end to end, read the actual fixture bodies and test assertions, and
verify every claim against the source at `461308a…` rather than against any archived spec's prose.

Research online as deeply as needed — similar implementations and research literature on **BDI agent
architectures, HTN planning, GOAP/STRIPS bounded planning, explainable-agent / explainable-NPC
diagnostics, deterministic simulation & replay, and "information-set" / fog-of-war separation between an
agent's knowledge and ground truth** — wherever it sharpens a finding or a lock mechanism (e.g.
established patterns for proving a planner reads only an agent's information set, or for making a
truth/knowledge firewall statically enforceable). The archived `0005` spec already lists a starting
prior-art set (Meneguzzi & Luck BDI-planning survey; Georgievski & Aiello HTN overview; Orkin's
"Three States and a Plan: The AI of F.E.A.R."; explainable-agent literature) — extend it. **Cite every
external source that shapes a decision.** External prior art informs design; it never overrides Tracewake
doctrine.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every product-behavior
  decision must satisfy it; cite the exact `INV-…` numbers, re-derived from the file. A genuine
  divergence requires amending an invariant first, never designing against it silently.
- **Authority order:** if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong.
- **Truth firewall (the spine of this block):** *truth may validate actions, but truth may not plan
  them* (INV-099). Needs and routines may not dispatch primitive actions directly (INV-104); the
  scheduler is not a cognition authority (INV-103); cognition reads only the sealed actor-known context
  with provenance (INV-100–102); validation failure feeds replanning without leakage (INV-106); debug
  omniscience is quarantined (INV-107); possession is cognition-neutral (INV-108).
- **No simulation fact born from prose.** Fixtures author *possibility and initial state*, never
  outcomes; no `culprit` / `quest_state` / authored-success fields.
- Preserve event-sourced causality, subjective epistemics, ordinary agents, possession parity, fallible
  institutions, no-quest leads, TUI-first playability, and deterministic validation/replay.
- **No backwards-compatibility shims or alias paths** in any proposed change.
- Strict crate dependency direction (`core` depends on nothing; never invert).
- **Source discipline:** a commit hash in a spec is provenance only; a manifest is path inventory only;
  branch/metadata/code-search snippets are not proof of target-commit content.

---

## 7. Deliverable specification

Produce, as **downloadable markdown**:

1. **A determination / verdict (always).** A clearly labeled, evidence-based judgment of whether the
   in-scope Phase 3A substrate aligns perfectly with current doctrine — enumerating each finding (or
   affirmatively stating none) with the exact invariant(s) / gate(s) it bears on and the source
   file:line evidence. This is the gate on whether the spec is authored.

2. **The hardening spec — authored iff the verdict is positive** (per §3.3):
   **`0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`**
   — a **new** file (it replaces nothing). Follow the `archive/specs/0013_…` anatomy and rigor:

   - **Header & evidence ledger** — target commit `461308af95940d59c2d56d32ffead35631c9db72`, scope
     statement, and the boundary (in-scope Phase-3A surface vs. boundary-awareness blocks). Freshness is
     user-supplied; do not re-assert independent verification beyond the audit.
   - **Determination** (the §7.1 verdict, embedded as the opening section).
   - **Doctrine alignment** — the Phase-3A doctrine restated in citable form (the canonical decision
     transaction, the firewall, ordinary-action parity), each anchored to exact invariants/gates.
   - **Findings & remediation** — one entry per finding: the violation/drift/weakness, the exact
     invariant(s)/gate(s) breached, the responsible layer (from
     `docs/2-execution/10`'s enumerated layer list), the source evidence, the corrective change
     (no shims), and the **structural anti-contamination lock** that prevents recurrence (compile-fail /
     source-guard / banned-API / clippy / conformance-index / replay-schema / sealed-accessor). Order by
     severity.
   - **Anti-contamination lock layer** — a consolidated section: the new/extended guards, what each
     proves, and how each fails the build/suite on regression. Explicitly cover the `TFW` per-spec
     checklist from `docs/2-execution/04` for this block.
   - **Adversarial fixture & test obligations** — map each required Phase-3A adversarial fixture from
     `docs/2-execution/06` to its current implementation (or a gap), and specify any
     strengthening/negative-fixture additions. Confirm replay byte-identity and stuck-diagnostic
     coverage.
   - **`ORD-LIFE-CERT`-scoped acceptance artifact** — instantiate the review-artifact shape (per
     `docs/1-architecture/13` and `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` if applicable),
     framed as scoped evidence toward `ORD-LIFE-CERT`, with explicit disclaimers (not full-project, not
     later-phase, not latest-main certification) and the live execution posture category.
   - **Self-check** (see §8).

3. **On a clean verdict only:** author **no** spec file; return the reasoned, evidence-complete
   determination (§7.1) as the response, explaining per-property why no hardening or anti-contamination
   mechanism is warranted.

> **Locked / no questions.** Produce the deliverables directly as downloadable markdown documents. Do
> not interview, do not ask clarifying questions — the requirements above are final. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed with the most
> faithful interpretation.

---

## 8. Self-check

Before returning, verify against your own output:

- [ ] The determination explicitly states a positive/clean verdict, and the spec-vs-no-spec decision
      matches §3.3 exactly.
- [ ] Every finding cites exact `INV-…` numbers **re-derived from
      `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` at `461308a`**, not from any archived spec's
      numbering, plus the `ORD-LIFE-CERT` / `TFW` gate clause it bears on.
- [ ] Every finding names its responsible layer from `docs/2-execution/10`'s enumerated list and gives
      source file evidence.
- [ ] Every finding (and every locked already-correct property) carries a **structural** mechanism that
      fails the build or test suite on regression — not just prose.
- [ ] Scope held: no Phase-1 spine internals, Phase-2A epistemic internals, or Phase-4 institutions were
      audited or "corrected"; only Phase-3A-introduced surface (including the in-`scheduler.rs` /
      `projections.rs` Phase-3A surface) was assessed.
- [ ] No proposed change introduces a backwards-compatibility shim or alias path, and none inverts crate
      dependency direction.
- [ ] No new doctrine is asserted; any doctrine gap is flagged as a recommendation, not designed against.
- [ ] The spec is framed as scoped evidence toward `ORD-LIFE-CERT` with explicit non-certification
      disclaimers; it nowhere claims the gate is passed.
- [ ] Filename = `0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`,
      staged for `specs/`, replacing nothing.
- [ ] Every external claim that shaped a decision is cited.
- [ ] The `461308af95940d59c2d56d32ffead35631c9db72` fetch baseline contains every file you relied on.
