# Research brief — Spec 0057 embodied-routine-continuation foundational-alignment hardening

> **You are ChatGPT-Pro Session 2. This prompt is final and self-contained.** Produce the
> deliverable directly as a downloadable markdown document. **Do not interview, do not ask
> clarifying questions** — every decision you need is settled below. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed with
> the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-30_4382f6d.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first play). Docs are
layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` →
`4-specs`; **earlier tiers govern later ones**. **Fetch every file from commit
`4382f6db10b1cad247ea2793c94a6cda81f36d6f` (`4382f6d`)** — the manifest reflects exactly that
tree.

**What this brief asks you to audit and harden.** A recently-completed *feature* spec,
`archive/specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md`,
closed a defect where a *possessed* (human-controlled) actor selecting the `Continue routine`
affordance produced a marker event but **no behavioral progress** — the actor never moved, the
routine step never advanced. The fix introduced a shared actor-known routine-step resolver and
wired an embodied follow-on commit so a possessed actor makes the same routine progress an
autonomous actor would. This is a **feature-forward** spec. The maintainer's standing practice
is to follow any feature-forward spec with a **foundational-alignment hardening pass**: audit
the implemented code against the full doctrine pack for foundational-principle violations, fix
any found, harden the code, and add anti-regression measures that make future regression of the
foundational alignment as infeasible as practical.

**Baseline / provenance.** The 0057 acceptance artifact
(`archive/reports/0057_embodied_routine_continuation_acceptance.md`) pins its *keystone*
implementation commit at `4726527858d027b4559bac607969b2bc6dfee094`. The fetch baseline above
(`4382f6d`) is **later**: it is the merge of the `spec-0057` PR into `main`, and it includes
post-keystone commits (`bacb383` "Killing mutants", `91f1875`/`44cdf67` CI fixes). A
`git diff --stat 4726527 4382f6d` over the seam files shows the embodied path in
`runtime/session.rs` (+328), `routine_continuation.rs` (+68), `app.rs` (+48), and
`embodied_flow.rs` (+28) changed *after* the keystone. **Therefore the audited surface is the
code at `4382f6d`, not the keystone** — read and reason about the baseline tree, and treat the
acceptance artifact's per-requirement evidence as the *pre-hardening claim to re-verify*, not
as established truth about the baseline code.

**Lineage and precedent.** The lineage seed is spec 0057 itself plus its acceptance report
(named above). The **structural precedent** for the deliverable's shape is the sibling
feature-hardening specs `archive/specs/0046_*` (TUI/simulation playable-capability parity
hardening) and `archive/specs/0047_*` (TUI authoritative world advance) — same parallel
`specs/NNNN` series, same house structure, same "scoped feature/hardening accepted for an exact
commit; mints no invariant/gate/glossary/risk-id" posture. The foundational-conformance
hardening series `0048`–`0056` is the precedent for *focused-mutation* hardening discipline
(read for method, not as a delta seed).

---

## 2. Read in full (authority order)

The maintainer's mandate is that you read the **entire doctrine pack** — every file in tiers
0–4 — before producing. Within each tier the **primary (load-bearing)** files for this target
are called out with a reason; the remaining files in that tier are **boundary-awareness reads
(read to bound scope and run the tier-fit test, not as conformance targets)**.

```
docs/README.md — authority order and the layering rule (load-bearing).

=== docs/0-foundation/  (read all; primary called out) ===
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md — PRIMARY. INV-001…INV-112; the constitution.
    INV-035 (routines are defeasible intentions), INV-094 (possession parity is tested),
    INV-095 (TUI/view-model tests are acceptance tests), INV-098 (harsh feature acceptance),
    INV-099 (truth may validate but not plan) are the load-bearing invariants for 0057.
docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md — PRIMARY. Routine/intention
    doctrine the resolver must honor.
docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md — PRIMARY. Behavioral-progress
    vs. marker semantics; ordinary-action doctrine.
docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md — PRIMARY. Possession/embodied-play
    scope; the affordance-must-reach-its-mechanic principle.
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md — PRIMARY. The
    truth-firewall the embodied follow-on resolution must obey (actor-known only, never hidden truth).
docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md — PRIMARY. Event-sourced causality +
    replay reconstructability + temporal authority (INV-112) the marker+follow-on must satisfy.
docs/0-foundation/00,01,04,07,09,10,11,12,13 — boundary-awareness: charter, claims/beliefs/memory,
    institutions, no-scripting, scale/LOD, LLM boundary, first-playable acceptance gates, research notes,
    index. Read to bound what is NOT 0057's concern.

=== docs/1-architecture/  (read all; primary called out) ===
docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md — PRIMARY. Firewall &
    provenance architecture the resolver/embodied commit depend on.
docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md — PRIMARY.
    The shared pipeline; 0057 amended this doc — re-verify the amendment is at the right altitude.
docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md — PRIMARY.
    The actor decision transaction the embodied path reuses; single-charge accounting seam (R1).
docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md — PRIMARY. TUI
    de-authority; "no simulation authority in the TUI boundary".
docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md — PRIMARY.
    Validation/observability/acceptance-artifact contracts and solo-maintainer acceptance mode.
docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md — PRIMARY. Replay/projection
    contract the follow-on commit must remain reconstructable under.
docs/1-architecture/00,01,06,07,08,09,11,12,14 — boundary-awareness: conformance index, workspace/dependency
    rules (note: tracewake-core is dependency-free), claims/observation, speech/LLM, institutions, settlement/
    economy, incidents/leads, LOD, forbidden-misreads. Read to bound scope.

=== docs/2-execution/  (read all; primary called out) ===
docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md — PRIMARY. Anti-contamination
    gates the embodied resolution must pass.
docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md — PRIMARY.
    No-direct-dispatch; the marker+follow-on must route through the shared pipeline (R3 stop-reason seam).
docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md — PRIMARY. 0057 amended this doc
    (embodied continue commits the follow-on). Re-verify wording and the marker-is-not-progress rule.
docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md — PRIMARY. Possession-parity
    proof obligations.
docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md — PRIMARY. Evidence-honesty
    rule, diagnostics-by-layer, acceptance-result taxonomy, mutation-evidence discipline.
docs/2-execution/00,01,02,03,08,09,11,12,13 — boundary-awareness: execution index/authority, post-0008
    baseline boundary, first-proof scope, phase-ladder/certification sequence, data authoring/validation,
    golden fixtures/replay acceptance, institutions/phase-4, deferred second-proof, research notes.

=== docs/3-reference/  (read all) ===
docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md — PRIMARY. Review checklist questions.
docs/3-reference/01_DESIGN_RISK_REGISTER.md — PRIMARY. Existing risk memory; do not mint new risk IDs.
docs/3-reference/02_GLOSSARY.md — PRIMARY. Canonical terms; do not mint new glossary terms.

=== docs/4-specs/  (read all) ===
docs/4-specs/SPEC_LEDGER.md — PRIMARY. Staging-sequence, source discipline, "Next known execution move".
docs/4-specs/README.md — PRIMARY. Spec-tier authority and house rules.
docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md — PRIMARY. The acceptance-artifact template the spec's
    acceptance section must reference.
docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md — boundary-awareness.

=== Implementation code seams (the audited surface) ===
crates/tracewake-core/src/agent/routine_continuation.rs — PRIMARY. The shared actor-known resolver:
    `RoutineContinuationResolution`, `resolve_routine_step_follow_on()`, and helpers
    (`assign_planned_targets_and_parameters`, `empty_plan_proposal`, `planner_goal_for`). Verify it is
    genuinely the SAME resolution the autonomous planner uses (R2 — no fork), and consumes only actor-known
    context (INV-099).
crates/tracewake-core/src/runtime/session.rs — PRIMARY. The embodied command surface (≈L540–790):
    `run_embodied_continue_routine_follow_on()`, `run_embodied_continue_routine_stuck_outcome()`,
    `append_embodied_routine_stuck_diagnostics()`, `embodied_continue_routine_stuck_report()`,
    `embodied_recursive_continue_routine_diagnostic()`. Verify marker+follow-on atomicity, receipt honesty,
    the recursive-continue guard, and single-charge tick/need accounting (R1).
crates/tracewake-core/src/actions/defs/continue_routine.rs — PRIMARY. The marker builder
    `build_continue_routine_event()` — must keep `behavioral_progress=false`/`intention_mutated=false`;
    validates against the single authoritative active intention (OQ1).
crates/tracewake-core/src/scheduler.rs — PRIMARY. Stuck diagnostics: `build_actor_stuck_diagnostic_event()`,
    `append_routine_stuck_diagnostics()`, `append_embodied_routine_stuck_diagnostics()`. R3 (stop-reason
    semantics must be unchanged from 0047) and the post-work `stuck_actors` accounting observation live here.
crates/tracewake-core/src/actions/pipeline.rs — PRIMARY. `ActionEffect::ContinueRoutine` dispatch; wait-event
    emission; lifecycle-control check for `continue_routine`.
crates/tracewake-tui/src/app.rs — PRIMARY. `submit_entry()` (≈L228–259) — confirm it FORWARDS ONLY to the
    runtime command (no simulation authority), incl. `same_action_receipt_requires_target_parity()`.
crates/tracewake-tui/tests/embodied_flow.rs — supporting. Embodied golden + post-keystone additions.

=== Anti-regression machinery (the hooks the deliverable must use/extend) ===
crates/tracewake-core/tests/anti_regression_guards.rs — PRIMARY. Meta-lock registry (SharedScan /
    BehaviorAssertion / TicketOwnedDebt routings; `lock_id`, `negative_id`, `witness_min`). Already holds three
    0057 guards: `guard_006_continue_routine_marker_alone_is_not_behavioral_progress`,
    `guard_0057_embodied_continue_non_proposed_outcome_is_typed_stuck`,
    `guard_0057_continue_routine_progress_of_record_is_follow_on`. New guards are added here.
crates/tracewake-core/tests/ci_workflow_guards.rs — PRIMARY. Diff-based standing-mutation trigger patterns;
    the 0057 seam paths already match. Verify trigger coverage for any new file.
.cargo/mutants.toml — PRIMARY. The standing mutation perimeter. CONFIRMED: all six 0057 seam files are already
    INSIDE the perimeter (`agent/**`, `runtime/**`, `scheduler.rs`, `actions/pipeline.rs`,
    `actions/defs/continue_routine.rs`, `tui/src/app.rs`). So anti-regression is about whether the new
    behaviors are actually *witnessed* (mutants caught), not about wiring the perimeter.
crates/tracewake-tui/tests/parity/ — PRIMARY. Playable-capability parity registry (`mod.rs` `registry()`,
    `census_actions.rs` row `spec0057.routine.embodied_continue_workday`, `runner.rs` `run_conformance()`/
    `measure_entry()`, `scenario.rs`). How a capability row is declared and behaviorally enforced.

=== Spec lineage + structural precedent ===
archive/specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md — PRIMARY.
    The feature spec; its §9 Risks/Open-questions (R1/R2/R3, OQ1) and §1.2 stuck_actors observation are
    first-class audit surfaces.
archive/reports/0057_embodied_routine_continuation_acceptance.md — PRIMARY. Per-requirement evidence to
    RE-VERIFY against baseline code (note line 256: "No mutation campaign was required by spec 0057.
    Mutation-perfect closure is not claimed." — a key gap to close.)
archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md
    — structural model (shape, house structure).
archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md
    — structural model + the world-tick coordinator 0057 consumes (out of scope to rewrite).
archive/specs/0056_FOUNDATIONAL_CONFORMANCE_SEVENTH_HARDENING_..._SPEC.md — read its "Deviation from the
    original plan" section for the focused-mutation / `--iterate` discipline that governs §3's mutation rule.
```

---

## 3. Settled intentions (do not re-open these)

1. **Always produce the spec (production mode (i)).** Author the numbered spec **regardless of
   verdict**. It must contain an evidence-based **determination/verdict section**: "does the
   0057 implementation, as it stands at `4382f6d`, violate any foundational principle — and
   where?" Even on a clean verdict (no strict violations), the spec is still produced and still
   scopes **hardening + anti-regression** work. The verdict is a section, not a gate on whether
   to write the file.

2. **Foundational scope is the doctrine pack, harshly applied.** Measure the code against
   INV-035/094/095/098/099 and INV-112 first, then the architecture/execution contracts.
   "Foundational violation" includes: any path that lets truth (hidden state) *select* (not
   merely validate) the follow-on target (INV-099); any simulation authority leaking into the
   TUI boundary; any way the marker can be counted as progress or the follow-on can silently
   not commit; any double-charge of time/needs; any divergence between the embodied and
   autonomous resolution; any break in replay reconstructability or event-sourced causality;
   any affordance presented that cannot reach its mechanic (INV-095/098).

3. **0057's own deferred risks are first-class audit surfaces.** Explicitly audit and, where
   warranted, harden: **R1** (marker+follow-on must not double-advance time or double-charge
   needs; resolve the open question of whether the follow-on shares the marker's tick or the
   next world tick), **R2** (the resolver must not fork between autonomous and embodied paths —
   pin identical output for a fixed actor-known state), **R3** (surfacing the follow-on receipt
   must not change `advance_until` stop-reason semantics from 0047), **OQ1** (the embodied
   `Continue routine` must resolve its follow-on from the single authoritative active intention's
   `current_step`, not an assigned-but-inactive windowed routine), and the **§1.2 post-work
   `stuck_actors` accounting observation**.

4. **Amendments routed as substance + home, never ratified text.** If the audit shows doctrine
   must be sharpened to admit or constrain the capability, present the amendment **as substance
   + home**: state *what* the doctrine must say and *which* file/tier owns it, with proposed
   substance at the right altitude — but explicitly flag that minting final invariant text
   (`INV-###`), gate codes, glossary terms, or risk IDs is the repository's own reassess/amend
   process, NOT this spec's act. A spec is subordinate to the live tiers and **may not amend
   constitutional invariants, define gate semantics, or weaken execution gates**. Include such
   amendment proposals inside the spec (a dedicated section), consistent with how 0057 §5 and
   the doctrine-alignment specs (0026–0035) routed amendments.

5. **Anti-regression: strongest mechanism per finding.** For each fix or hardened property,
   choose the strongest available lock: behavioral test locks, a playable-capability **parity
   row**, a **source-scan / meta-lock guard** in `anti_regression_guards.rs` (with `negative_id`
   + `witness_min`), and/or a **focused mutation witness**. The goal the maintainer stated:
   make it *as infeasible as practical* for future code to regress the foundational alignment of
   these fixes. Prefer behavioral/causal witnesses over pure source-text scans where a behavior
   can be asserted.

6. **Mutation methodology — focused, not full (hard constraint).** The standing mutation
   perimeter is ~3,400–3,500 mutants; full runs are expensive and, per spec 0056's recorded
   deviation, caused disk pressure from repeated full-run restarts. **Do not design the spec
   around repeated full mutation runs.** A prior convention (spec 0056) says that whenever a
   missed mutant is caught with a new test the run restarts — given the corpus size, the spec
   must instead prescribe **focused mutation** scoped to the 0057 seam files
   (`cargo mutants` with `--in-diff` and/or per-file `--file`/`--examine` globs, converging with
   `cargo mutants --iterate` over the existing `mutants.out`), and reserve **at most one** final
   full standing campaign for the very end, *only if genuinely necessary*. State this as the
   spec's mutation-evidence discipline.

7. **Out of scope (negative settled intentions — do not re-open or "correct"):**
   - The **autonomous no-human planner**, scheduler **cadence**, and the **0047 authoritative
     world-tick coordinator** — 0057 consumes their resolution; this pass does not rewrite them.
   - Making `continue_routine` "count as progress" — the marker stays a marker.
   - Auto-running a whole routine on one keystroke / multi-step fast-forward / `advance_until`
     stop-reason changes.
   - New ontology, new affordance kinds, food/sleep redesign, institutional behavior, notices,
     travel, LOD, LLM/speech, story-sifting.
   - Any whole-project or latest-`main` certification, `P0-CERT`/`FIRST-PROOF-CERT`/Phase-4/
     second-proof claim. This is a scoped feature-hardening spec; it certifies nothing
     project-wide and mints no invariant/gate/glossary/risk-id of its own (amendment *proposals*
     under §3.4 excepted, and even those are routed, not ratified).

8. **Deliverable placement.** The spec is the next contiguous staging number **`0058`** (the
   archived series runs 0001–0057 with 0049 ticket-only; there is no `specs/` staging dir at
   baseline and no recent renumber/restart). It is staged in `specs/` and archived to
   `archive/specs/` on acceptance — never promoted to live `docs/4-specs/`. It receives its
   `SPEC_LEDGER.md` archived-row entry at acceptance/closeout, not now.

---

## 4. The task

This is a **hardening / anti-contamination** pass whose deliverable is a **numbered
implementation spec**. Audit the code that implements spec 0057 (the embodied
routine-continuation feature) at commit `4382f6d` against the full layered doctrine pack,
applying the constitution harshly. Determine — with evidence — whether the implementation
violates any foundational principle, and where. Then author `specs/0058_*.md`: a spec that
(a) renders that determination/verdict, (b) scopes precise remediation for each foundational
violation found (fix the code, never adapt tests to a bug), (c) hardens the implementation
(closing 0057's own R1/R2/R3/OQ1 and the `stuck_actors` observation as first-class surfaces),
and (d) installs anti-regression measures — the strongest lock per property — so future code
cannot quietly regress the foundational alignment. Where doctrine itself must be sharpened,
include the amendment as routed substance + home (not ratified text). Decompose the spec into
one ticket per reviewable diff, in the house structure of siblings 0046/0047.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — follow every symbol,
test, fixture, and CI hook the seam code touches; read whatever adjacent module clarifies the
firewall, pipeline, scheduler, or replay contract. **Research online as deeply as needed** —
how comparable event-sourced / deterministic-replay simulations reconcile a human "autopilot /
follow-routine" input with an autonomous planner without forking resolution; prior art on
possession/avatar parity in agent simulations; the truth-firewall / no-information-leak pattern
(server-authoritative vs. client knowledge); mutation-testing practice for large corpora
(focused / diff-scoped / incremental convergence); and metamorphic / property-based testing for
"embodied output ≡ autonomous output from equivalent state." **Cite every external source that
shapes a decision.** The deep research is yours to perform; the brief only commissions it.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every
  product-behavior decision must satisfy it; a genuine divergence requires **amending an
  invariant first** (routed, per §3.4), never designing against it silently.
- **Authority order is absolute:** if execution conflicts with architecture or foundation,
  execution is wrong; if implementation is more convenient than the accepted gates,
  implementation is wrong.
- **Truth firewall / anti-contamination:** no simulation fact may be born from prose or from the
  TUI; truth may *validate* an action but may never *select/plan* it (INV-099); actor-known
  context only. Preserve event-sourced causality, subjective epistemics, ordinary agents,
  possession parity, fallible institutions, validation/replay.
- `tracewake-core` is **dependency-free** and is the home of the shared resolver; the TUI
  consumes it through the existing runtime command surface and holds **no** simulation authority
  (INV-008/INV-069 per architecture `10`).
- **No backwards-compatibility shims or alias paths** in new work.
- The spec **may not** amend constitutional invariants, define/weaken gate semantics, or promote
  archived history into certification.

---

## 7. Deliverable specification

Produce **one** downloadable markdown document, **new** (not a replacement):

- **`specs/0058_<DESCRIPTIVE_NAME>_SPEC.md`** — choose an ALL-CAPS underscore name in the style
  of the siblings (e.g. `0058_EMBODIED_ROUTINE_CONTINUATION_FOUNDATIONAL_ALIGNMENT_HARDENING_SPEC.md`).
  This is the next contiguous staging number `0058` (per §3.8). There is **no** paired
  `-research-report.md`; the spec is the sole deliverable.

The spec must use the **house structure of siblings `0046`/`0047`** and include at least:

0. A status/posture header (parallel `specs/NNNN` feature/hardening series; staged then archived;
   not a certification audit; `P0-CERT not applicable`; mints no invariant/gate/glossary/risk-id;
   amendment proposals routed, not ratified).
1. Baseline statement & source discipline — name the audited commit (`4382f6d`), state that named
   symbols are authoritative and any line numbers are provenance only, and that the implementing
   session must name its own exact implementation commit.
2. Scope (in / out) — incorporating the §3.7 negatives verbatim in spirit.
3. **Determination / verdict** — evidence-based finding on foundational violations in the 0057
   code at baseline (this is mode (i)'s embedded verdict). Cite the specific invariant/contract
   and the specific file:symbol for each holding (violation *or* confirmed-aligned).
4. Findings & remediation requirements — one per violation/hardening target, each anchored to a
   named doctrine principle, covering R1/R2/R3/OQ1 and the `stuck_actors` observation explicitly.
5. Doctrine amendments (if any) — routed substance + home, per §3.4.
6. Required fixtures & tests + **anti-regression measures** — the strongest lock per property
   (behavioral / parity-row / meta-lock guard / focused mutation witness), per §3.5, with the
   focused-mutation discipline of §3.6.
7. Acceptance artifact & evidence — reference `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
   and the four-gate local suite (`cargo fmt --all --check`, `cargo clippy --workspace
   --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`,
   `cargo test --workspace`).
8. Implementation decomposition — one ticket per reviewable diff.
9. Risks / open questions and invariants alignment.

**Locked / no-questions:** Produce the deliverable directly as a downloadable markdown document.
Do not interview, do not ask clarifying questions — the requirements above are final. If a
genuine contradiction makes a requirement impossible, state it in the deliverable and proceed
with the most faithful interpretation.

---

## 8. Self-check (run before returning)

- [ ] The verdict in §3 of the spec is **evidence-based**: each holding cites a specific
      invariant/contract AND a specific `file:symbol` at baseline `4382f6d`.
- [ ] Every remediation requirement traces to a **named foundational principle** (an INV or a
      tier contract), not to convenience.
- [ ] R1, R2, R3, OQ1, and the post-work `stuck_actors` observation are each addressed (audited,
      and hardened where warranted) — none silently dropped.
- [ ] No proposed amendment **weakens** an upstream tier; all amendments are routed substance +
      home, with no minted `INV-###`/gate/glossary/risk-id presented as ratified.
- [ ] Each fix/hardened property names a concrete **anti-regression lock**; mutation evidence is
      **focused** (no design around repeated full ~3.5K-mutant runs; ≤1 final full campaign only
      if necessary).
- [ ] The spec preserves marker-is-not-progress, the truth firewall (no truth-selected target),
      TUI de-authority, replay reconstructability, and possession/no-human parity.
- [ ] The deliverable set matches §7 exactly: a single new `specs/0058_*.md`, house-structured
      like 0046/0047, claiming no project-wide certification.
- [ ] Every file named in §2 exists at commit `4382f6d` (the manifest tree).
```
