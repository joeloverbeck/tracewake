# Research brief — Spec-0047 foundational-conformance hardening, THIRD PASS (post-0050 re-audit)

> **You are ChatGPT-Pro Session 2. This brief is final and locked. Do not interview, do not ask
> clarifying questions. Produce the deliverable directly as a downloadable markdown document.**

## 1. Context

The uploaded manifest (`manifest_2026-06-24_0429a8f.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered
authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier
tiers govern later ones, and a constitutional invariant set (`INV-001…INV-112`) is the
non-negotiable contract. **Fetch every file from commit `0429a8f` — the manifest reflects exactly
that tree.**

**This is the THIRD foundational-hardening pass on the spec-0047 surface.** Read the lineage so you
treat this as a *delta re-audit*, not a cold start, and — critically — **not** as a carry-forward of
already-true findings:

- **`archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`**
  is the feature spec: a shared kernel one-world-tick coordinator, human wait + duration
  continuation, no-human progression, log-derived duration completion, single-charge accounting,
  body-exclusive reservation, replay-visible tick ancestry, and actor-known interval summaries.
- **First pass — `reports/0047-foundational-hardening-research-report.md`** analyzed commit
  `cb3102e` (static survey) and found **six foundational violations plus one hardening gap**. Its
  remediation spec is **`archive/specs/0048_FOUNDATIONAL_CONFORMANCE_HARDENING_..._HARDENING_SPEC.md`**
  (implemented as the `0048FOUCONHAR-001…008` ticket series); its acceptance is
  **`reports/0048_foundational_conformance_hardening_acceptance.md`** (the focused mutation campaign
  left **eight survivors**, not a mutation-perfect claim). The follow-up `0049MUTWIT` ticket line
  (`archive/tickets/0049MUTWIT-001…003.md`) added narrow mutation witnesses to kill those survivors.
- **Second pass — `reports/0047-foundational-hardening-research-report-second-pass.md`** (the
  *immediate lineage predecessor*) analyzed commit `8d7c119` (static survey) and found **eight
  findings**: F-01 production world steps did not derive loaded actors; F-02 raw caller-authored
  process events bypassed declared-process authority; F-03 the actor decision transaction outcome was
  only partially consumed (two competing choreographies); F-04 the TUI still authored post-step
  perception / rebuilt interval products (incl. a duplicate-`EventId` path); F-05 replay aggregate
  success ignored temporal divergence; F-06 salient-stop accepted routine-observation churn; F-07 the
  0048 behavioral-lock evidence overstated production reachability; F-08 the `0049MUTWIT` line lacked
  a `SPEC_LEDGER.md` source record. Its companion brief is
  `reports/0047-foundational-hardening-research-brief-second-pass.md`.
- **`archive/specs/0050_FOUNDATIONAL_CONFORMANCE_SECOND_HARDENING_..._HARDENING_SPEC.md`** is the
  remediation spec authored from that second-pass report; it was implemented as the
  `0050FOUCONSEC-001…016` ticket series. Its acceptance is
  **`archive/reports/0050_foundational_conformance_second_hardening_acceptance.md`**, which records
  every second-pass finding F-01…F-08 as **closed**, and which **explicitly does NOT certify a green
  standing mutation perimeter**: the configured standing campaign (`cargo mutants --timeout 183`,
  3,182-mutant denominator) recorded **48 missed mutants and 1 timeout**, routed forward to "separate
  survivor remediation before any artifact claims the standing perimeter is green." The 0049MUTWIT
  provenance gap (F-08) was closed — `SPEC_LEDGER.md` now carries a `0049MUTWIT` source/navigation
  record.

**Freshest seed and baseline divergence (verified — do NOT carry findings forward).** The lineage
predecessor (second-pass report) pinned `8d7c119`. The current fetch baseline `0429a8f` is the merge
of the implemented-0050 branch, so `8d7c119` is now an ancestor of HEAD and the second-pass findings
describe the **pre-0050-remediation** code. The seam files were rewritten wholesale between the two
commits — verified with `git diff --stat 8d7c119 0429a8f` over the seam paths:

```
crates/tracewake-core/src/scheduler.rs        | 1230 ++++++++++++++++++++++---
crates/tracewake-core/src/projections.rs      |  193 +++-
crates/tracewake-core/src/agent/perception.rs |   72 +-
crates/tracewake-core/src/events/log.rs       |   55 ++
crates/tracewake-core/src/replay/report.rs    |   23 +-
crates/tracewake-core/src/replay/temporal.rs  |   17 +
crates/tracewake-tui/src/app.rs               |   89 +-
```

Therefore **DO NOT carry the second-pass F-01…F-08 (or the first-pass) findings forward as
still-true.** Use them only as the *prior baseline the 0050 (and 0048/0049) work claimed to fix*.
Your job is to determine, against the live `0429a8f` code and the live doctrine, whether the current
surface actually satisfies the foundations — **assuming nothing about the compliance, completeness,
or correctness of the 0048, 0049, or 0050 work.** Two prior remediation passes each still found
critical violations on this exact seam; treat "previously remediated" as zero evidence of present
conformance.

**Post-acceptance nuance to verify, not assume.** The `0050FOUCONSEC` series landed its acceptance
artifact at ticket `-013` (commit `448ecee`), and the 48-missed/1-timeout standing campaign is
recorded against that point. Tickets `-014` (projection mutation witnesses), `-015` (perception
collision bound), and `-016` (novelty mutation witnesses) landed **after** the acceptance was
written. No fresh standing campaign was recorded after them. So the recorded "48 missed" set may be
partially stale at `0429a8f` — re-verify it; do not treat the acceptance's number as the live floor
without checking what `-014`/`-015`/`-016` changed.

## 2. Read in full (authority order)

Read these before producing. Order is authority order; earlier tiers govern later ones. Entries
marked *primary* are load-bearing conformance targets; *boundary-awareness* entries are read to bound
scope (what is in vs. out), not to audit or "correct."

**Universal**

- `docs/README.md` — authority order and the layering rule.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001…INV-112`; every verdict must measure
  against these (esp. `INV-001`, `INV-004`, `INV-005`, `INV-006`, `INV-009`, `INV-010`, `INV-018`,
  `INV-041`, `INV-067`, `INV-069`, `INV-087`, `INV-088`, `INV-091`, `INV-092`, `INV-094`, `INV-098`,
  `INV-099`, `INV-101`, `INV-102`, `INV-103`, `INV-104`, `INV-105`, `INV-108`, `INV-112`).

**Foundation (`docs/0-foundation/`)**

- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — *primary*; event-sourced causality, deterministic
  replay, and temporal authority (`INV-112`): every accepted step, including an empty one, must carry
  ancestry sufficient to rebuild the temporal frontier.
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — *primary*; "waiting runs the simulation" (one input
  slot; other loaded actors, world processes, and due consequences advance in one shared transition),
  actor-filtered summaries, staged embodied time controls incl. stop-on-salient.
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — *primary*; the cognition-transaction
  and truth-firewall doctrine the interval/perception/actor-decision surfaces must obey.
- `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`, `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`,
  `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — *boundary-awareness*; doctrine the
  now-invoked autonomous-actor/process collaborators must satisfy.
- `00_FOUNDATION_INDEX.md`, `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — *boundary-awareness*.

**Architecture (`docs/1-architecture/`)**

- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — *primary*; empty-tick frontier rebuild,
  replay-derived projections, save/restore boundary.
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — *primary*; actor-known interval
  summaries are positively-constructed holder-known frontier deltas, never raw-diff redaction;
  provenance, not prose.
- `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — *primary*; due-work ordering,
  ordinary proposal routing for human and autonomous actors, due consequences owned by one core
  world-step coordinator; declared-process authority.
- `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — *primary*; the actor
  decision transaction the loaded-world tick must invoke and fully consume without becoming cognition
  authority.
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — *primary*; the TUI asks core for a
  typed world step and never applies events / mutates state / owns durations / holds a local clock;
  actor-known summary vs. debug report are separate products.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — *primary*; the
  evidence/observability contract the anti-regression layer must honor.
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — *primary*; the conformance index 0050 updated; check
  its loaded-world/time-control rows against the live witnesses for staleness/overclaim.
- `06`, `09`, `11`, `12` — *boundary-awareness* as collaborators/processes require.

**Execution (`docs/2-execution/`)**

- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — *primary*; source-bearing
  holder-known construction, anti-contamination gates.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — *primary*; the phase-ordered
  one-tick transaction contract, core-owned actor/process discovery, and no-direct-dispatch closure.
- `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — *primary*; the held-equal human/no-human
  proof must not exercise only a private possessed-actor tick.
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — *primary*; embodied controls use the
  ordinary TUI/core boundary, not debug/no-human gameplay paths; read-only interval product.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — *primary*; typed-before-rendered
  evidence, real path-under-test witnesses, anti-vacuity rules, non-green standing mutation status —
  the spine of the anti-regression and re-verification mandate.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — *boundary-awareness*.

**Reference (`docs/3-reference/`)**

- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — *primary*; the review prompts the implementing
  session must answer with live evidence (0050 named live executable evidence here).
- `01_DESIGN_RISK_REGISTER.md` — *primary*; existing risks incl. R-08, R-09, R-10, R-11, R-13, R-15,
  R-16, R-27 (reachability overstatement), R-28 (defect-family completeness), R-29 (decorative locks).
  **Mint no new risk ID.**
- `02_GLOSSARY.md` — *boundary-awareness*.

**Specs (`docs/4-specs/`)**

- `SPEC_LEDGER.md` — *primary*; the active/archived spec record, the `0049MUTWIT` source record, the
  `0050` archived row, and the source-discipline rules. Confirm the live navigation/status accurately
  reflects the surface at `0429a8f`.
- `README.md` — *boundary-awareness*; live-spec index.

**Lineage, evidence, and machinery**

- `reports/0047-foundational-hardening-research-report-second-pass.md` — *primary*; the immediate
  predecessor's findings (the prior baseline to re-verify, **NOT** a carry-forward).
- `reports/0047-foundational-hardening-research-brief-second-pass.md` — the predecessor brief (scope
  precedent; this brief continues its conventions).
- `reports/0047-foundational-hardening-research-report.md` — *primary*; the first-pass findings
  (earlier pre-remediation baseline).
- `reports/0047-foundational-hardening-research-brief.md` — the first-pass brief (scope precedent).
- `archive/reports/0050_foundational_conformance_second_hardening_acceptance.md` — *primary*; the F-01…F-08
  closure claims, the gate→witness map, the behavior/negative witnesses, and the **48-missed +
  1-timeout** standing-mutation floor.
- `reports/0048_foundational_conformance_hardening_acceptance.md` — *primary*; the eight original
  survivors and the "not classified equivalent" temporal survivors (historical; 0050 superseded its
  overbroad reachability evidence for this seam).
- `archive/specs/0047_..._SPEC.md`, `archive/specs/0048_..._HARDENING_SPEC.md`,
  `archive/specs/0050_..._HARDENING_SPEC.md` — *primary*; the promised properties and named witnesses
  (historical, immutable; not current-state proof).
- `archive/reports/0047_tui_authoritative_world_advance_acceptance.md` — historical 0047 acceptance;
  do not edit; not current proof.
- `archive/tickets/0050FOUCONSEC-011.md` — *primary*; the durable mutation transcript (focused + the
  48-missed/1-timeout standing campaign). Read `archive/tickets/0050FOUCONSEC-001…016.md` as needed
  for exactly what each remediation/witness ticket changed (esp. `-014`/`-015`/`-016`, which postdate
  the acceptance).
- `archive/tickets/0049MUTWIT-001.md`, `-002.md`, `-003.md` — *primary*; the preserved focused
  mutation witnesses for the eight 0048 survivors, to re-verify for vacuity.
- `.cargo/mutants.toml` — *primary*; the standing mutation perimeter (already covers the seam files;
  do not recommend re-adding what exists).

**Code seams to inspect (read as needed; not doctrine to conform to)** — audit the spec-0047 surface
as now realized by 0048/0049/0050, **plus the collaborators the atomic one-tick step actually
invokes**:

- core: `crates/tracewake-core/src/scheduler.rs` (`transact_world_one_tick`, `advance_until`,
  due-actor / due-process discovery, declared-process cadence/invocation, actor-step outcome
  consumption, frontier privacy/restore, salience predicate, interval delta), `replay/temporal.rs`,
  `replay/report.rs` (`run_replay`/`matches_expected` temporal conjunct), `replay/rebuild.rs`,
  `replay/mod.rs`, `projections.rs` (`ActorKnownIntervalDelta`, `VerifiedActorKnownIntervalNotice`,
  `build_embodied_view_model`, source-key accessors), `view_models.rs`, `epistemics/projection.rs`,
  `epistemics/knowledge_context.rs`, `actions/pipeline.rs`, `events/log.rs` (duplicate-`EventId`
  fail-closed), `need_accounting.rs`, and the now-invoked collaborators `agent/transaction.rs`,
  `agent/decision.rs`, `agent/candidate.rs`, `agent/perception.rs`, `agent/planner.rs`, plus the
  world-process registry/cadence the step drives.
- core tests: `tests/world_step_coordinator.rs`, `tests/replay_temporal_frontier.rs`,
  `tests/holder_known_interval_projection.rs`, `tests/salient_stop_actor_known.rs`,
  `tests/reservation_body_exclusive_census.rs`, `tests/generative_lock.rs`,
  `tests/anti_regression_guards.rs`, `tests/negative_fixture_runner.rs`.
- TUI: `crates/tracewake-tui/src/app.rs`, `src/render.rs`,
  `tests/playable_capability_parity.rs`, `tests/parity_adversarial.rs`, `tests/tui_seam_conformance.rs`,
  `tests/command_loop_session.rs`, `tests/embodied_flow.rs`, `tests/parity/*`.

## 3. Settled intentions (final — do not re-open)

These resolve every question Session 2 might otherwise ask.

### 3.1 Audit posture
- **Re-audit the current spec-0047 surface at `0429a8f` for foundations compliance WITHOUT assuming
  it is compliant**, even though specs `0048`, `0049MUTWIT`, and `0050` claim to have remediated it.
  Prior remediation being merged is not evidence the live code satisfies the contract; prove it
  against the live doctrine and the live code. Explicitly: **do not assume the 0048/0050 work is
  fully complete or even correct.**
- **Breadth = the touched seams PLUS the now-invoked collaborators.** Audit the loaded-world tick,
  temporal authority/replay, holder-known interval/salient-stop, atomicity, and the autonomous-actor
  decision/transaction + world-process machinery the atomic step now drives — because loaded-world-tick
  correctness depends on them. A whole-kernel foundations sweep is **out of scope**: do not audit or
  "correct" subsystems unrelated to the spec-0047 transition (institutions, notices, travel, LOD,
  LLM/speech, story-sifting).

### 3.2 Re-verify the prior remediation for vacuity
- **Re-verify, do not assume.** For the 0050 production-path behavior witnesses (loaded-world
  differential, salient-stop quiet/novel/hidden/replay policy, read-only interval-product consumption,
  replay temporal aggregate, declared-process discovery, compile-fail negative fixtures) and the
  preserved 0049MUTWIT focused witnesses, determine from the live code whether each is **non-vacuous**
  — i.e. it exercises the real production path and would actually fail if the protected behavior
  regressed, rather than being a decorative lock / declaration-as-proof / harness-manufactured input
  (the R-27 / R-29 failure modes, execution `10`'s anti-vacuity rule). Where you cannot prove
  non-vacuity by static reading, say so and specify the executable check the implementing session must
  run.

### 3.3 Standing mutation survivor floor — Session 2 decides per-survivor
- The 0050 acceptance records a **non-green standing mutation perimeter: 48 missed mutants + 1
  timeout** (`cargo mutants --timeout 183`, denominator 3,182), recorded against the `-013`
  acceptance point and routed forward to "separate survivor remediation." There is **no pre-set in/out
  boundary** for this pass. For the survivor floor (re-verified against what `-014`/`-015`/`-016`
  changed — see §1 post-acceptance nuance), **decide survivor-by-survivor**: for each survivor (or
  survivor family) that guards a **foundational-correctness property** of the spec-0047 surface, treat
  closing it as **in scope** for this pass's remediation (specify the production-path witness / kill);
  for survivors that are equivalent mutants, out-of-surface, or non-foundational, **route them
  forward** to a separate mutation-remediation line and say so. Justify each disposition; do not
  mechanically scope "kill all 48," and do not silently ignore the floor.

### 3.4 Anti-regression definition
- "Anti-regression" = make it as infeasible as practical for future code to regress the foundations
  alignment of this surface. Prefer, in order: **compile-time unrepresentability** (private frontier,
  private interval-source / due-work constructors, closed typed notice/stop/actor-step-outcome enums,
  single core world-step entry point, no debug→embodied conversion, no public raw-`EventEnvelope`
  process injection), then **non-vacuous behavior witnesses** that exercise the real path, then
  **focused mutation kills**, then narrow `include_str!` source guards as *topology alarms only*
  (never cited as proof of atomicity/replay/noninterference). **Account for existing machinery** —
  `.cargo/mutants.toml` already covers the seam files; `anti_regression_guards.rs`,
  `negative_fixture_runner.rs`, the external-crate negative-fixture pattern, and the deterministic
  `generative_lock.rs` corpus already exist. Recommend *behavior that kills relevant mutants* and
  *extensions to existing harnesses*, not new infrastructure that duplicates these. Do not propose
  adding `proptest`/`quickcheck` unless you make a specific, justified case that the deterministic
  harness cannot express the needed property.

### 3.5 Recurrence / structural-guard analysis (required section)
- This is the **third consecutive pass** to (potentially) find critical foundational violations on the
  same seam — the first pass found 6+1, the second found 8, both remediated, yet this re-audit is
  commissioned because the work is not assumed complete. The report **must** include a dedicated
  **recurrence / structural-guard** section that steps above the per-finding level and analyzes: (a)
  *why* critical foundational violations keep recurring on this loaded-world/time-control/TUI-boundary
  seam (e.g. open API shapes that re-admit caller authority, evidence that proves composition rather
  than production reachability, a client boundary repeatedly able to re-acquire write authority); and
  (b) what **higher-leverage structural guard** would break the cycle rather than patch the latest
  instance — candidates include type-level unrepresentability that makes the violation class impossible
  to express, a single enforced core entry point, a standing conformance/parity lane wired into CI, or
  an architectural boundary that removes the recurring temptation. Name the substance and the home;
  mint no new identifiers and write no ratified doctrine wording.

### 3.6 Foundational-amendment posture
- The deliverable must include a clearly-labeled **determination** on whether any higher-tier doctrine
  (`0-foundation` / `1-architecture` / `2-execution` / `3-reference`) needs amendment. The default,
  inherited expectation (both prior passes concluded this) is that doctrine is correct and the code is
  below it — **changing foundation text to fit incorrect code is forbidden**. Recommend an amendment
  only if you find a genuine doctrine gap or contradiction; if so, state the substance and the home,
  but **do not** write ratified paste-ready wording or invent `INV-###` / gate / risk identifiers.

### 3.7 Static-survey limitation (honesty)
- You cannot run `cargo fmt/clippy/build/test`, replay/golden lanes, or `cargo-mutants`. Every
  pass/fail statement about current tests, builds, or mutation counts is a **preliminary static
  reading of source shape and witness intent**, not an authoritative command result; label it as such.
  Authoritative execution belongs to the implementing session. No finding may be premised on an
  assumed green or red command.

## 4. The task

Produce a foundational-conformance hardening and anti-regression analysis (a **hardening /
anti-contamination** target) for the spec-0047 surface as it exists at commit `0429a8f` — after the
spec-0048 remediation, the `0049MUTWIT` mutation-witness follow-up, and the spec-0050 second
remediation. Determine, against the live constitution and the live architecture/execution/reference
doctrine, whether the current loaded-world tick, temporal-authority/replay, holder-known
interval/salient-stop, atomicity, and now-invoked autonomous-actor/world-process surfaces actually
satisfy the foundations — **assuming nothing about the compliance, completeness, or correctness of the
0048/0049/0050 work.** For every divergence found, specify the remediation (substance + home in
`docs/**` and the code), the strongest practical anti-regression guard, and an evidence-honesty check.
Re-verify the prior remediation's production-path and 0049 witnesses for vacuity; assess the 48-missed
+ 1-timeout standing mutation floor survivor-by-survivor (in-scope to close vs. routed forward);
include a dedicated recurrence / structural-guard analysis; and render a determination on whether any
foundational amendment is warranted.

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above (any code path, test, or doc
the surface touches). Research online as deeply as needed — similar implementations, deterministic-
simulation and replay literature, event-sourcing / single-writer authority patterns, truth-firewall /
information-flow control designs, and mutation-testing / property-based-testing prior art — wherever
it sharpens a recommendation (especially the §3.5 structural-guard candidates). Cite sources for any
external claim that shapes a decision. Keep the repository-evidence lane and the external-research lane
strictly separate, as both prior passes did.

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every product-behavior
  decision must satisfy it; a genuine divergence requires amending an invariant first, never designing
  against it silently.
- Authority order: if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong.
- No backwards-compatibility shims or alias paths in new work; recommend deleting superseded paths
  rather than keeping compatibility wrappers.
- Anti-contamination: no simulation fact may be born from prose; preserve event-sourced causality,
  subjective epistemics, ordinary agents, possession parity, fallible institutions, and
  validation/replay. Holder-known surfaces are positively constructed, never raw-diff redaction.
- Do not edit archived specs, archived acceptance artifacts, or any passed certification; they are
  immutable commit-pinned history and are not current-state proof.
- Mint no new invariant, gate code, risk ID, or glossary term; propose no ratified paste-ready
  doctrine wording. Deliver substance + home; the repo's own reassess/amend process ratifies text.

## 7. Deliverable specification

Produce **one downloadable markdown document**:

- **`reports/0047-foundational-hardening-research-report-third-pass.md`** — **new** (not a
  replacement; it sits beside the first-pass `reports/0047-foundational-hardening-research-report.md`
  and the second-pass `reports/0047-foundational-hardening-research-report-second-pass.md`, neither of
  which you may edit).

This is an **analysis / recommendation report, NOT a numbered spec** — do not apply spec
numbering/ledger/epoch rules; no `docs/4-specs/` artifact is produced here. (Claude Code converts this
report into the next numbered remediation spec afterward.)

**Verdict mode — determination-plus-conditional, mode (ii):**
- **Always render a clearly-labeled, evidence-based verdict** ("does the current spec-0047 surface
  satisfy the foundations at `0429a8f`, and what remains?").
- **Write the report file iff** at least one foundational violation, warranted hardening/anti-regression
  gap, vacuity finding, or in-scope standing-mutation survivor exists.
- **If — and only if — the surface is fully clean** (no violation, no warranted hardening gap, all
  prior witnesses proven non-vacuous, and every standing-mutation survivor justifiably routed forward
  as out-of-surface/equivalent), author **no file** and instead return the evidence-complete
  determination inline as your response, stating why no document is warranted.

**Report shape** (follow the second-pass report's structure, updated to `0429a8f`):
1. **Verdict** — overall determination + freshness/static-survey scope statement.
2. **Disposition table** — one row per finding → primary target (file/doc) → severity
   (violation / hardening-gap / vacuity-gap / mutation-survivor disposition) → one-line basis (cite
   the invariants/doctrine).
3. **Method & provenance ledger** — authority order applied; baseline `0429a8f`; explicit statement
   that the second-pass `8d7c119` (and first-pass `cb3102e`) findings were treated as *pre-remediation*
   baselines and re-verified, not carried forward; existing anti-regression machinery accounted for;
   static-survey limitation.
4. **Per-finding sections** — for each: foundational driver (docs + invariants) → current `0429a8f`
   code state → conformance verdict → required remediation (substance + home) → strongest practical
   guard (compile-time / behavior / mutation, per §3.4) → evidence-honesty check. Include dedicated
   findings for any §3.2 vacuity result and the §3.3 survivor-floor dispositions.
5. **Comprehensive anti-regression layer** — a table covering every load-bearing spec-0047 property
   (including those that are now correct), with current state, what is already guarded, the delta to
   implement, and the strongest practical mechanism; plus mutation-testing and compile-time/source-guard
   disposition, and the survivor-floor verdict.
6. **Recurrence / structural-guard analysis** — the §3.5 section: why criticals recur on this seam
   across three passes, and the higher-leverage structural guard that would break the cycle (substance
   + home; no new identifiers).
7. **Foundation & documentation determination** — §3.6 verdict; substance + home for any doc work
   (conformance rows, risk-register evidence/status using only existing IDs); no ratified wording.
8. **Recommended closure order** — a remediation sequence that avoids writing tests around an API that
   must be replaced.
9. **Open maintainer decisions** — implementation choices inside settled doctrine.
10. **Self-check** (see §8) and **References**.

**Forward-routing:** this is a **cross-cutting** hardening pass (it spans foundation→execution→reference
homes rather than sitting at one tier), so any out-of-scope items — including standing-mutation
survivors routed forward under §3.3 — go only to **owner/reassess decisions and future implementation
specs**, not to a lower tier. State the reason as *cross-cutting*.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

## 8. Self-check (run before returning)

- [ ] The verdict is rendered explicitly; the file-vs-inline branch (§7 mode ii) matches the verdict.
- [ ] Every file was fetched from commit `0429a8f`; the `0429a8f` tree contains every file named in §2.
- [ ] No second-pass (`8d7c119`) or first-pass (`cb3102e`) finding is asserted as currently-true without
      re-verification against live `0429a8f` code.
- [ ] Every finding cites the specific invariants/doctrine it violates and names a `docs/**` + code home.
- [ ] The 0050 production-path witnesses and the preserved 0049MUTWIT witnesses are assessed for
      vacuity, with the executable check named where static reading is insufficient.
- [ ] The 48-missed/1-timeout standing-mutation floor is assessed survivor-by-survivor (in-scope vs.
      routed forward), accounting for what `0050FOUCONSEC-014/015/016` changed after the acceptance.
- [ ] The dedicated recurrence / structural-guard section (§3.5) is present and proposes a
      higher-leverage guard, not just per-finding patches.
- [ ] Recommendations extend existing machinery (`.cargo/mutants.toml`, `anti_regression_guards.rs`,
      negative-fixture pattern, `generative_lock.rs`) rather than duplicating it; no `proptest`/
      `quickcheck` is added without a specific justification.
- [ ] No archived spec/acceptance/certification is edited; no new INV/gate/risk/glossary ID is minted;
      no ratified paste-ready doctrine wording is authored.
- [ ] The foundational-amendment determination is explicit; any amendment is justified by a genuine
      doctrine gap, never to fit incorrect code.
- [ ] Every external claim that shapes a decision is cited; the repository-evidence and external-research
      lanes are kept separate.
- [ ] Static-survey limitation is stated; no finding assumes a green/red command result.
