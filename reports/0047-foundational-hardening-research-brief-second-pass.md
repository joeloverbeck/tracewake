# Research brief — Spec-0047 foundational-conformance hardening, SECOND PASS (post-0048/0049 re-audit)

> **You are ChatGPT-Pro Session 2. This brief is final and locked. Do not interview, do not ask
> clarifying questions. Produce the deliverable directly as a downloadable markdown document.**

## 1. Context

The uploaded manifest (`manifest_2026-06-23_8d7c119.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered
authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier
tiers govern later ones, and a constitutional invariant set (`INV-001…INV-112`) is the
non-negotiable contract. **Fetch every file from commit `8d7c119` — the manifest reflects exactly
that tree.**

**This is the SECOND foundational-hardening pass on the spec-0047 surface.** Read the lineage so you
treat this as a *delta re-audit*, not a cold start, and — critically — not as a carry-forward of
already-true findings:

- **`archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`**
  is the feature spec: a shared kernel one-world-tick coordinator, human wait + duration
  continuation, no-human progression, log-derived duration completion, single-charge accounting,
  body-exclusive reservation, replay-visible tick ancestry, and actor-known interval summaries.
- **`reports/0047-foundational-hardening-research-report.md`** is the FIRST-pass research report
  (lineage predecessor). It analyzed commit `cb3102e` (a *static* survey, no execution) and found
  **six foundational violations plus one hardening gap**: (4.1) the canonical one-tick coordinator
  was not a loaded-world step (zero actor/process phases); (4.2) the TUI committed the possessed
  action before the authoritative tick (non-atomic); (4.3) the temporal frontier had multiple public
  writers; (4.4) the actor-known salient-stop branch was unreachable in production; (4.5) interval
  summaries were raw-log redaction with forgeable fields; (4.6) replay did not reconstruct the
  temporal frontier; (4.7) the differential/parity evidence overstated what it proved. Its companion
  brief is `reports/0047-foundational-hardening-research-brief.md`.
- **`archive/specs/0048_FOUNDATIONAL_CONFORMANCE_HARDENING_LOADED_WORLD_TICK_TEMPORAL_AUTHORITY_HOLDER_KNOWN_INTERVALS_AND_REPLAY_FRONTIER_RECONSTRUCTION_HARDENING_SPEC.md`**
  is the remediation spec authored from that report; it was implemented as the `0048FOUCONHAR-001…008`
  ticket series. Its acceptance artifact is **`reports/0048_foundational_conformance_hardening_acceptance.md`**.
  That acceptance explicitly records a **focused mutation campaign that left EIGHT survivors**
  (two `source_key` accessor survivors in `projections.rs`; two `==`→`!=` rejection-condition
  survivors in `scheduler.rs::transact_world_one_tick`; four boundary survivors in
  `replay/temporal.rs`'s `project_temporal_frontier`/`validate_time_advanced`). It states the four
  temporal survivors are **"not classified equivalent"** and that "a future mutation pass should
  either run the workspace/TUI seam or add a narrower core witness." It is **not** a mutation-perfect
  or full-configured-mutation claim.
- **`0049MUTWIT`** is that follow-up: three tickets
  (`archive/tickets/0049MUTWIT-001.md` scheduler, `-002.md` interval, `-003.md` replay) that added
  mutation witnesses to kill the eight survivors. **`0049MUTWIT` has NO spec file (none in `specs/`,
  none in `archive/specs/`) and NO entry in `docs/4-specs/SPEC_LEDGER.md`** — the ledger's last
  archived row is `0048`. Treat that absence as a real provenance/source-discipline fact (see §3.2),
  not an oversight to ignore.

**Freshest seed and baseline divergence (verify, do not assume equivalence).** The lineage
predecessor pinned `cb3102e`. The current fetch baseline `8d7c119` is a *merge* of `cb3102e` and the
0048+0049 branch, so `cb3102e` is now a parent of HEAD and the predecessor's findings describe the
**pre-remediation** code. The seam files were rewritten wholesale between the two commits — verified
with `git diff --stat cb3102e 8d7c119` over the seam paths:

```
crates/tracewake-core/src/scheduler.rs             | 1239 ++++++++++++++++----
crates/tracewake-core/src/projections.rs           |  230 ++--
crates/tracewake-core/src/replay/temporal.rs       |  172 +++  (new file)
crates/tracewake-core/src/replay/rebuild.rs        |   99 +-
crates/tracewake-core/src/epistemics/projection.rs |  122 ++
crates/tracewake-core/src/epistemics/knowledge_context.rs | 149 ++-
crates/tracewake-core/src/actions/pipeline.rs      |   99 +-
crates/tracewake-core/src/view_models.rs           |   26 +-
crates/tracewake-tui/src/app.rs                    |  274 ++---
```

Therefore **DO NOT carry the predecessor's seven findings forward as still-true**. Use them only as
the *prior baseline the 0048/0049 work claimed to fix*. Your job is to determine, against the live
`8d7c119` code and the live doctrine, whether the current surface actually satisfies the foundations
— assuming nothing about its compliance.

## 2. Read in full (authority order)

Read these before producing. Order is authority order; earlier tiers govern later ones. Entries
marked *primary* are load-bearing conformance targets; *boundary-awareness* entries are read to bound
scope (what is in vs. out), not to audit.

**Universal**

- `docs/README.md` — authority order and the layering rule.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001…INV-112`; every verdict must
  measure against these (esp. `INV-001`, `INV-005`, `INV-006`, `INV-009`, `INV-010`, `INV-018`,
  `INV-067`, `INV-091`, `INV-092`, `INV-094`, `INV-099`, `INV-101`, `INV-102`, `INV-103`, `INV-104`,
  `INV-108`, `INV-112`).

**Foundation (`docs/0-foundation/`)**

- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — *primary*; event-sourced causality, deterministic
  replay, and the temporal-authority section (`INV-112`): every accepted step, including an empty
  one, must carry ancestry sufficient to rebuild the temporal frontier.
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — *primary*; "waiting runs the simulation" (one input
  slot; other loaded actors, world processes, and due consequences advance in one shared
  transition), actor-filtered summaries, staged embodied time controls incl. stop-on-salient.
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — *primary*; the cognition-transaction
  and truth-firewall doctrine the interval/perception surfaces must obey.
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
  world-step coordinator.
- `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — *primary*; the actor
  decision transaction the loaded-world tick must invoke without becoming cognition authority.
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — *primary*; the TUI asks core for a
  typed world step and never applies events / mutates state / owns durations / holds a local clock;
  actor-known summary vs. debug report are separate products.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — *primary*; evidence/observability
  contract the anti-regression layer must honor.
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — *primary*; the conformance index 0048 updated; check
  its time-control posture rows against the live witnesses.
- `06`, `09`, `11`, `12` — *boundary-awareness* as collaborators/processes require.

**Execution (`docs/2-execution/`)**

- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — *primary*; source-bearing
  holder-known construction, anti-contamination gates.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — *primary*; the phase-ordered
  one-tick transaction contract and no-direct-dispatch closure.
- `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — *primary*; the held-equal human/no-human
  proof must not exercise only a private possessed-actor tick.
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — *primary*; embodied controls use the
  ordinary TUI/core boundary, not debug/no-human gameplay paths.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — *primary*; typed-before-rendered
  evidence, real path-under-test witnesses, anti-vacuity rules — the spine of the anti-regression
  and re-verification mandate.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — *boundary-awareness*.

**Reference (`docs/3-reference/`)**

- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — *primary*; the review prompts the implementing
  session must answer with live evidence.
- `01_DESIGN_RISK_REGISTER.md` — *primary*; existing risks R-08, R-09, R-10, R-11, R-13, R-15, R-16,
  R-27 (reachability overstatement), R-28, R-29 (decorative locks). **Mint no new risk ID.**
- `02_GLOSSARY.md` — *boundary-awareness*.

**Specs (`docs/4-specs/`)**

- `SPEC_LEDGER.md` — *primary*; the active/archived spec record and source-discipline rules. **Note
  that `0049MUTWIT` is absent from it** — relevant to §3.2.
- `README.md` — *boundary-awareness*; live-spec index.

**Lineage, evidence, and machinery**

- `reports/0047-foundational-hardening-research-report.md` — *primary*; the first-pass findings (the
  prior baseline to re-verify, NOT a carry-forward).
- `reports/0047-foundational-hardening-research-brief.md` — the predecessor brief (scope precedent).
- `reports/0048_foundational_conformance_hardening_acceptance.md` — *primary*; records the eight
  surviving mutants, the gate→witness map, and the explicit "not classified equivalent" temporal
  survivors.
- `archive/specs/0047_TUI_AUTHORITATIVE_...SPEC.md`, `archive/specs/0048_FOUNDATIONAL_CONFORMANCE_HARDENING_...SPEC.md`
  — *primary*; the promised properties and named witnesses (historical, immutable; not current-state proof).
- `archive/reports/0047_tui_authoritative_world_advance_acceptance.md` — historical 0047 acceptance;
  do not edit; not current proof.
- `archive/tickets/0049MUTWIT-001.md`, `-002.md`, `-003.md` — *primary*; exactly what the
  mutation-witness follow-up changed (scheduler / interval / replay), to re-verify for vacuity.
- `.cargo/mutants.toml` — *primary*; the standing mutation perimeter (already covers the seam files;
  do not recommend re-adding what exists).

**Code seams to inspect (read as needed; not doctrine to conform to)** — audit the spec-0047 surface
as now realized by 0048/0049, **plus the collaborators the atomic one-tick step actually invokes**:

- core: `crates/tracewake-core/src/scheduler.rs` (`transact_world_one_tick`, `advance_until`,
  `build_time_advanced_event`, frontier privacy/restore), `replay/temporal.rs`
  (`project_temporal_frontier`, `validate_time_advanced`), `replay/rebuild.rs`, `replay/report.rs`,
  `replay/mod.rs`, `projections.rs` (`ActorKnownIntervalDelta`, `VerifiedActorKnownIntervalNotice`,
  `build_embodied_view_model`, `proposal_from_current_view_semantic_action`), `view_models.rs`,
  `epistemics/projection.rs`, `epistemics/knowledge_context.rs`, `actions/pipeline.rs`,
  `need_accounting.rs`, and the now-invoked collaborators `agent/transaction.rs`, `agent/decision.rs`,
  `agent/candidate.rs`, `agent/perception.rs`, `agent/planner.rs`, plus whatever world-process
  registry/cadence the step drives.
- core tests: `tests/world_step_coordinator.rs`, `tests/replay_temporal_frontier.rs`,
  `tests/holder_known_interval_projection.rs`, `tests/salient_stop_actor_known.rs`,
  `tests/reservation_body_exclusive_census.rs`, `tests/generative_lock.rs`,
  `tests/anti_regression_guards.rs`, `tests/negative_fixture_runner.rs`, `tests/support/generative.rs`.
- TUI: `crates/tracewake-tui/src/app.rs`, `src/render.rs`,
  `tests/playable_capability_parity.rs`, `tests/parity/{runner,scenario,census_actions,census_families,mod}.rs`,
  `tests/parity_adversarial.rs`, `tests/tui_seam_conformance.rs`.

## 3. Settled intentions (final — do not re-open)

These resolve every question Session 2 might otherwise ask.

### 3.1 Audit posture
- **Re-audit the current spec-0047 surface at `8d7c119` for foundations compliance WITHOUT assuming
  it is compliant**, even though spec `0048` and `0049MUTWIT` claim to have remediated it. The prior
  remediation being merged is not evidence that the live code satisfies the contract; prove it
  against the live doctrine and the live code.
- **Breadth = the touched seams PLUS the now-invoked collaborators.** Audit the loaded-world tick,
  temporal authority/replay, holder-known interval/salient-stop, atomicity, and the autonomous-actor
  decision/transaction + world-process machinery the atomic step now drives — because loaded-world-tick
  correctness depends on them. A whole-kernel foundations sweep is **out of scope**: do not audit or
  "correct" subsystems unrelated to the spec-0047 transition (institutions, notices, travel, LOD,
  LLM/speech, story-sifting).

### 3.2 The 0049MUTWIT provenance gap is in scope
- Treat the fact that `0049MUTWIT` landed as archived tickets **with no spec package and no
  `SPEC_LEDGER.md` entry** as an **in-scope, graded finding**. The ledger is the repo's source-discipline
  and navigation authority; mutation-witness code changes that bypass the spec/ledger record weaken
  the anti-regression posture (future readers cannot trace why those witnesses exist or what survivor
  floor they close). Recommend the substance + home of the fix (e.g. a ledger/spec record of the
  0049 line) **without** writing ratified ledger text or inventing identifiers — that remains the
  repo's own reassess/ledger process.

### 3.3 Re-verify the prior remediation for vacuity
- **Re-verify, do not assume.** For the eight survivors the 0048 acceptance recorded and the
  `0049MUTWIT` witnesses that target them, determine from the live code whether the witnesses are
  **non-vacuous** — i.e. they exercise the real production path and would actually fail if the
  protected behavior regressed, rather than being decorative locks / declaration-as-proof (the R-27 /
  R-29 failure modes, execution `10`'s anti-vacuity rule). Apply the same scrutiny to the 0048
  "behavioral locks" generally. Where you cannot prove non-vacuity by static reading, say so and
  specify the executable check the implementing session must run.

### 3.4 Anti-regression definition
- "Anti-regression" = make it as infeasible as practical for future code to regress the foundations
  alignment of this surface. Prefer, in order: **compile-time unrepresentability** (private frontier,
  private interval-source constructors, closed typed notice/stop enums, single core world-step entry
  point, no debug→embodied conversion), then **non-vacuous behavior witnesses** that exercise the real
  path, then **focused mutation kills**, then narrow `include_str!` source guards as *topology alarms
  only* (never cited as proof of atomicity/replay/noninterference). **Account for existing machinery**
  — `.cargo/mutants.toml` already covers the seam files; `anti_regression_guards.rs`,
  `negative_fixture_runner.rs`, and the external-crate negative-fixture pattern, and the deterministic
  `generative_lock.rs` corpus already exist. Recommend *behavior that kills relevant mutants* and
  *extensions to existing harnesses*, not new infrastructure that duplicates these. Do not propose
  adding `proptest`/`quickcheck` unless you make a specific, justified case that the deterministic
  harness cannot express the needed property.

### 3.5 Foundational-amendment posture
- The deliverable must include a clearly-labeled **determination** on whether any higher-tier doctrine
  (`0-foundation` / `1-architecture` / `2-execution` / `3-reference`) needs amendment. The default,
  inherited expectation (the first pass concluded this) is that doctrine is correct and the code is
  below it — **changing foundation text to fit incorrect code is forbidden**. Recommend an amendment
  only if you find a genuine doctrine gap or contradiction; if so, state the substance and the home,
  but **do not** write ratified paste-ready wording or invent `INV-###` / gate / risk identifiers.

### 3.6 Static-survey limitation (honesty)
- You cannot run `cargo fmt/clippy/build/test`, replay/golden lanes, or `cargo-mutants`. Every
  pass/fail statement about current tests is a **preliminary static reading of test intent and source
  shape**, not an authoritative command result; label it as such. Authoritative execution belongs to
  the implementing session. No finding may be premised on an assumed green or red command.

## 4. The task

Produce a foundational-conformance hardening and anti-regression analysis (a **hardening /
anti-contamination** target) for the spec-0047 surface as it exists at commit `8d7c119` — after the
spec-0048 remediation and the `0049MUTWIT` mutation-witness follow-up. Determine, against the live
constitution and the live architecture/execution/reference doctrine, whether the current loaded-world
tick, temporal-authority/replay, holder-known interval/salient-stop, atomicity, and now-invoked
autonomous-actor/world-process surfaces actually satisfy the foundations — assuming nothing about
their compliance. For every divergence found, specify the remediation (substance + home in `docs/**`
and the code), the strongest practical anti-regression guard, and an evidence-honesty check. Re-verify
the prior remediation's witnesses for vacuity, surface the `0049MUTWIT` provenance gap, and render a
determination on whether any foundational amendment is warranted.

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above (any code path, test, or doc
the surface touches). Research online as deeply as needed — similar implementations, deterministic-
simulation and replay literature, mutation-testing and property-based-testing prior art, truth-firewall
/ information-flow designs — wherever it sharpens a recommendation. Cite sources for any external claim
that shapes a decision. Keep the repository-evidence lane and the external-research lane separate, as
the first-pass report did.

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

- **`reports/0047-foundational-hardening-research-report-second-pass.md`** — **new** (not a
  replacement; it sits beside the first-pass `reports/0047-foundational-hardening-research-report.md`,
  which you must not edit).

This is an **analysis / recommendation report, NOT a numbered spec** — do not apply spec
numbering/ledger/epoch rules; no `docs/4-specs/` artifact is produced here.

**Verdict mode — determination-plus-conditional, mode (ii):**
- **Always render a clearly-labeled, evidence-based verdict** ("does the current spec-0047 surface
  satisfy the foundations, and what remains?").
- **Write the report file iff** at least one foundational violation or warranted hardening/anti-
  regression gap (including the §3.2 provenance gap or any §3.3 vacuity finding) exists. Given §3.2 and
  §3.3 this is the expected branch.
- **If — and only if — the surface is fully clean** (no violation, no warranted hardening gap, no
  provenance gap, all prior witnesses proven non-vacuous), author **no file** and instead return the
  evidence-complete determination inline as your response, stating why no document is warranted.

**Report shape** (follow the first-pass report's structure, updated to `8d7c119`):
1. **Verdict** — overall determination + freshness/static-survey scope statement.
2. **Disposition table** — one row per finding → primary target (file/doc) → severity
   (violation / hardening-gap / provenance-gap) → one-line basis (cite the invariants/doctrine).
3. **Method & provenance ledger** — authority order applied; baseline `8d7c119`; explicit statement
   that the predecessor `cb3102e` findings were treated as the *pre-remediation* baseline and
   re-verified, not carried forward; existing anti-regression machinery accounted for; static-survey
   limitation.
4. **Per-finding sections** — for each: foundational driver (docs + invariants) → current `8d7c119`
   code state → conformance verdict → required remediation (substance + home) → strongest practical
   guard (compile-time / behavior / mutation, per §3.4) → evidence-honesty check. Include a dedicated
   finding for the §3.2 `0049MUTWIT` provenance gap and for any §3.3 vacuity result.
5. **Comprehensive anti-regression layer** — a table covering every load-bearing spec-0047 property
   (including those that are now correct), with current state, what is already guarded, the delta to
   implement, and the strongest practical mechanism; plus mutation-testing and compile-time/source-guard
   disposition.
6. **Foundation & documentation determination** — §3.5 verdict; substance + home for any doc work
   (conformance rows, risk-register evidence/status using only existing IDs); no ratified wording.
7. **Recommended closure order** — a remediation sequence that avoids writing tests around an API that
   must be replaced.
8. **Open maintainer decisions** — implementation choices inside settled doctrine.
9. **Self-check** (see §8) and **References**.

**Forward-routing:** this is a **cross-cutting** hardening pass (it spans foundation→execution→reference
homes rather than sitting at one tier), so any out-of-scope items route only to **owner/reassess
decisions and future implementation specs** — not to a lower tier. State the reason as *cross-cutting*.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

## 8. Self-check (run before returning)

- [ ] The verdict is rendered explicitly; the file-vs-inline branch (§7 mode ii) matches the verdict.
- [ ] Every file was fetched from commit `8d7c119`; the `8d7c119` tree contains every file named in §2.
- [ ] No predecessor (`cb3102e`) finding is asserted as currently-true without re-verification against
      live `8d7c119` code.
- [ ] Every finding cites the specific invariants/doctrine it violates and names a `docs/**` + code home.
- [ ] The `0049MUTWIT` provenance gap (no spec, absent from `SPEC_LEDGER.md`) is addressed as a finding.
- [ ] The eight 0048 survivors and the `0049MUTWIT` witnesses are assessed for vacuity, with the
      executable check named where static reading is insufficient.
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
