# Spec-0047 foundational-conformance hardening — fifth-pass research brief

**You are ChatGPT-Pro Session 2.** This document is your complete, final assignment. It is
self-contained: you see only this prompt plus one uploaded manifest. Produce the deliverable in
§7 directly. **Do not interview, do not ask clarifying questions** — the requirements below are
settled. If a genuine contradiction makes a requirement impossible, state it inside the
deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-26_e9792dc.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first playability).
Docs are layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` →
`4-specs`; **earlier tiers govern later ones**. Fetch every file from commit
`e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f` — the manifest reflects exactly that tree. The
manifest is a path inventory only; it is not file-content evidence. Branch names,
default-branch lookups, connector namespace labels, repository metadata, and code-search
snippets are not proof of target-commit content; fetch by exact-commit file path.

### This is the fifth audit pass on one recurring seam

The "spec-0047 surface" is the loaded-world / time-control / TUI-authority seam introduced by
`archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`
(TUI-authoritative world advance, duration completion, no-human progression, actor-known
interval summaries). It has now been audited and remediated **four** times in a row, and **each
remediation line that claimed closure was followed by an audit that still found critical
foundational violations.** You are the fifth pass.

The lineage (name each as a delta; do not re-commission completed work):

- **First pass** → `reports/0047-foundational-hardening-research-report.md` → implemented as
  spec `0048` (`archive/specs/0048_FOUNDATIONAL_CONFORMANCE_HARDENING_…`).
- **Second pass** → `reports/0047-foundational-hardening-research-report-second-pass.md` →
  implemented as spec `0050`
  (`archive/specs/0050_FOUNDATIONAL_CONFORMANCE_SECOND_HARDENING_…`).
- **Third pass** → `reports/0047-foundational-hardening-research-report-third-pass.md` →
  implemented as spec `0051`
  (`archive/specs/0051_FOUNDATIONAL_CONFORMANCE_THIRD_HARDENING_CORE_OWNED_LOADED_WORLD_RUNTIME_…`).
- **Fourth pass** (your immediate predecessor, target commit `6495d7d`) →
  `reports/0047-foundational-hardening-research-report-fourth-pass.md` → implemented as spec
  **`0052`**
  (`archive/specs/0052_FOUNDATIONAL_CONFORMANCE_FOURTH_HARDENING_PRODUCTION_BOOTSTRAP_CLOSED_COMMAND_BOUNDARY_…`).

The **fourth-pass report is your richest seed** — read it in full. Its findings are the
specification that spec 0052 was built to satisfy; its structural / recurrence analysis already
diagnosed *why the seam keeps reopening* and proposed the enforced standing gate spec 0052 then
built. Your job is to determine whether spec 0052 actually closed the authority class — or
whether the seam has reopened a fifth time — **and**, as the distinctive new ask of this pass, to
diagnose and fix the *process-level* recurrence: why four consecutive acceptance artifacts each
rendered "pass" while foundational violations survived into the next pass, and why some
remediation specs flagged violations their own implementation then failed to fix.

### Source-equivalence posture (re-audit of the literal 0052-remediated code)

Spec 0052's keystone implementation commit is `8e84150228e82d29dfddf2e9f52f201c3cf10c9c`. It is
an ancestor of your fetch baseline `e9792dc`. **A verified `git diff --stat
8e84150 e9792dc -- crates/tracewake-core/src crates/tracewake-tui/src` is empty: the production
and TUI source is byte-identical between the 0052 keystone and your baseline.** The only commits
between them are archival / doc-truthing / spec-ledger / skills changes plus the merge of PR #64.
Concretely this means: **the `archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md`
artifact describes the exact source you are auditing.** Treat that as an *advantage and a trap*,
not a continuation:

- It is an advantage because there is no intervening code drift to reconcile — the acceptance's
  per-finding closure table (F4-01…F4-09) maps onto the code at `e9792dc` line-for-line.
- It is a trap because **this is precisely the recurrence you are auditing**: four prior
  acceptance artifacts rendered "pass" over code a later pass found non-conformant. So you must
  **re-derive every property from the post-0052 code itself** and treat the 0052 acceptance's
  verdicts — especially the three it qualified (F4-04 "Pass with honest demotion", F4-08 "Pass
  for 0052 in-surface; canonical perimeter not green because food-source survives", and the
  pending branch-protection governance row) — as **unverified claims**, not established facts.
  Where 0052 genuinely closed a property, record it as **present**; where its "pass" does not
  bear out against the code, report the live defect; and remain open to violations none of the
  five passes (including this acceptance) named.

The 0052 acceptance is immutable historical evidence: you neither edit it nor treat its "pass"
rows as proof of current conformance. The two residuals it explicitly leaves open at your
baseline are first-class inputs for you (see §3.4): repository branch protection is **not**
enabled on `main` (so the standing CI gate 0052 wired is not actually enforced at merge), and the
seven-survivor `food_source_fact_supersedes` mutation family is routed forward.

---

## 2. Read in full (authority order)

Read these before producing. Order is by authority tier; earlier governs later. Entries are
marked **primary (load-bearing conformance target)** or **boundary-awareness (read to bound
scope — what genuinely belongs in this tier vs. another; not a thing to audit or amend)**.

```
docs/README.md — authority order and the layering rule that governs the whole audit. (primary)
```

**Tier 0 — foundation (the constitution; governs everything):**

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — **primary.** The full invariant set;
  every conformance verdict and any amendment proposal must be grounded here. Re-verify each
  invariant the prior passes leaned on against the current code rather than inheriting their
  citations.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **primary.** Event-sourced
  causality + replay/restore contract the runtime and scheduler must satisfy.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **primary.** Waiting as one
  input slot in which other loaded actors / world processes / due consequences advance; embodied
  vs. debug surface boundary.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **primary.**
  The cognition-transaction and truth-firewall doctrine governing the actor decision transaction
  and interval-summary leakage.
- `docs/0-foundation/00,01,04,05,06,07,09,10,11,12,13` — **boundary-awareness.** Read to run the
  tier-fit test for any amendment proposal (including a *process/verification-integrity*
  amendment — confirm whether such a rule belongs in the constitution at all vs. the execution
  tier) and to confirm what is out of the spec-0047 surface.

**Tier 1 — architecture:**

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **primary.** The conformance
  index; 0052 claims to have truthed its loaded-world rows. Re-verify the rows against the current
  production path.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **primary.**
  Replay/save/restore contract behind the replay-authority findings.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **primary.**
  Firewall/provenance contract behind the sealed-product and interval-leak findings.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` —
  **primary.** Pipeline + one core-owned world-step coordinator.
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` —
  **primary.** Canonical cognition transaction the scheduler must consume.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` —
  **primary.** Read-only client boundary; TUI must not own/mutate aggregates or construct
  embodied/debug products directly.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` —
  **primary.** Evidence-honesty / no-decorative-locks contract behind the mutation-honesty
  finding, the anti-regression gate, **and** the process-integrity / acceptance-honesty
  mechanism you will design — this is the architecture-tier home for "what an acceptance
  artifact may and may not claim."
- `docs/1-architecture/01,06,07,08,09,11,12,14` — **boundary-awareness.**

**Tier 2 — execution:**

- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **primary.** The
  P0/SPINE/EPI/ORD-LIFE/FIRST-PROOF certification ladder; read it to place any recommended
  standing gate or process mechanism correctly relative to the existing gates **without minting a
  new gate code.**
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` —
  **primary.** Due-work belongs to the core world-step boundary; no parallel possessed-actor
  path; no caller-injected actor lists.
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **primary.**
  No-human progression and held-equal possession parity.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **primary.** TUI
  stores/renders the core interval product read-only; embodied/debug split.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **primary
  (the doctrine home for the process-integrity contribution).** Evidence-honesty rule,
  missed-mutant/timeout discipline, real-public-boundary testing, the merge-blocking-red /
  pending-is-not-a-pass rule. This is where any fail-closed-acceptance strengthening lands, and
  where the standing-gate governance is anchored.
- `docs/2-execution/00,01,02,04,08,09,11,12,13` — **boundary-awareness.**

**Tier 3 — reference:**

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **primary.** Reviewer pointers
  the report's doc-home recommendations update.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — **primary.** The R-27/R-28/R-29 evidence/status
  rows (mint no new risk ID). `02_GLOSSARY.md` — **boundary-awareness.**

**Tier 4 — specs, plus reports / archive (history & lineage; not live authority):**

- `docs/4-specs/SPEC_LEDGER.md` — **primary.** Source/navigation discipline; the
  0047/0048/0050/0051/0052 archived rows, the `0049MUTWIT` ticket-only record, and the "Next
  known execution move" section. Confirm any new remediation spec routes through the normal
  ledger process; mint nothing.
- `reports/0047-foundational-hardening-research-report-fourth-pass.md` — **primary (richest
  seed).** Your predecessor's full findings, recurrence analysis, and enforced-standing-gate
  proposal that spec 0052 implemented.
- `reports/0047-foundational-hardening-research-brief-fourth-pass.md` — **boundary-awareness.**
  The locked scope/lineage the fourth pass worked to; confirms conventions and the canonical
  report shape.
- `archive/specs/0052_…_HARDENING_SPEC.md` — **primary.** The promises the fourth remediation
  line made; the spec whose claims you re-verify against the code.
- `archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md` — **primary.**
  What 0052 *claims* it closed, the per-finding closure table (F4-01…F4-09), the mutation command
  ledger, the seven routed-forward `food_source_fact_supersedes` survivors, and the explicit
  branch-protection / governance residuals. Your concrete residual-finding seed; describes the
  exact source you audit (source-equivalence posture, §1).
- `reports/0047-foundational-hardening-research-report-third-pass.md`,
  `archive/reports/0051_foundational_conformance_third_hardening_acceptance.md`,
  `archive/reports/0050_foundational_conformance_second_hardening_acceptance.md`,
  `reports/0048_foundational_conformance_hardening_acceptance.md`,
  `archive/reports/0047_tui_authoritative_world_advance_acceptance.md` —
  **boundary-awareness.** Prior remediation epochs and the original feature-surface boundary;
  historical evidence and recurrence raw material only. **For the process-integrity analysis
  (§3.2) these become primary corpus:** read each prior acceptance's verdict wording and qualified
  "pass" rows to reconstruct *how* "pass" was rendered each time over code a later pass found
  non-conformant.
- `archive/specs/0050_…SPEC.md`, `archive/specs/0051_…SPEC.md` — **boundary-awareness.** The
  promises the prior remediation lines made.
- `.cargo/mutants.toml` — **primary.** The existing standing mutation perimeter config the
  anti-regression recommendation must extend, not duplicate.

### Code seams to inspect (read directly at the baseline; do not expect them pasted here)

Audit the post-0052 source at `e9792dc` (byte-identical to the 0052 keystone), not the
fourth-pass quotes of it:

- `crates/tracewake-core/src/runtime/{mod,session,command,receipt}.rs` — `LoadedWorldRuntime`
  (`from_bootstrap`, `LoadedWorldBootstrap`, `rebuild_from_owned_log`,
  `world_stream_position_applied_for_log`, `refresh_actor_current_place_perception`,
  `assign_proposal_sequence`), the closed `RuntimeCommand` dispatch, and `RuntimeActionReceipt` /
  `RuntimeReceiptKind` / sealed embodied/debug receipt products. The 0052 keystone for production
  bootstrap, closed command boundary, and TUI de-authority.
- `crates/tracewake-core/src/scheduler.rs` — `transact_world_one_tick`, `due_loaded_actor_ids`,
  `due_process_invocations`, the per-tick actor disposition census (`ActorStepStatus`),
  `restore_from_temporal_projection`, declared-process authority restore.
- `crates/tracewake-core/src/events/{envelope,apply,log}.rs` — declared-process event class &
  application; duplicate-`EventId` fail-closed.
- `crates/tracewake-core/src/agent/{transaction,trace,perception}.rs` — actor decision
  transaction consumption; `StuckDiagnostic::deserialize_canonical`; perception write path.
- `crates/tracewake-core/src/epistemics/projection.rs`,
  `crates/tracewake-core/src/projections.rs` — `insert_observation`, `embodied_subject_key`,
  interval-delta/salience; the routed-forward `food_source_fact_supersedes` family at
  `projections.rs` (the seven standing-survivor lines).
- `crates/tracewake-core/src/view_models.rs` — `EmbodiedViewModel`,
  `TypedActorKnownIntervalSummary` (field/constructor sealing; the accessor-return families the
  prior mutation campaigns flagged).
- `crates/tracewake-core/src/replay/{temporal,rebuild,report}.rs`,
  `crates/tracewake-core/src/actions/pipeline.rs`.
- `crates/tracewake-content/src/load.rs` — loaded-world handoff into the runtime constructor
  (`into_runtime_bootstrap`).
- `crates/tracewake-tui/src/{app,run,transcript,render,input,launch,debug_panels}.rs` —
  `TuiApp::from_golden`, `submit_semantic_action`, `advance_until`, command-loop dispatch,
  `capture_representative_transcript_sections`; embodied rendering leakage.
- The test / negative-fixture / CI infrastructure: `crates/tracewake-core/tests/*` (esp.
  `world_step_coordinator.rs`, `negative_fixture_runner.rs`, `anti_regression_guards.rs`,
  `generative_lock.rs`, `ci_workflow_guards.rs`, `replay_temporal_frontier.rs`, the external
  negative fixtures under `tests/negative-fixtures/`, plus replay/interval/salience/reservation/
  parity suites), `crates/tracewake-tui/tests/*`, and `.github/workflows/ci.yml` (the
  `public-boundary-conformance`, `full-surface-mutation-trigger`, and
  `mutants-lock-layer-reconcile` jobs).

---

## 3. Settled intentions (these are decisions, not open questions)

1. **Full independent re-audit of the entire spec-0047 surface at `e9792dc`.** Re-derive
   conformance for the whole loaded-world / time-control / TUI-authority surface from the
   post-0052 code. Treat the 0052 acceptance's "pass" verdicts — especially the three qualified
   rows (F4-04 honest demotion, F4-08 in-surface-only, the pending branch-protection governance
   row) — as **unverified claims**, not facts. Rationale: four consecutive remediation lines each
   declared closure and were each followed by a pass that found critical violations; a delta-only
   audit would inherit 0052's blind spots. Because the source is byte-identical to the 0052
   keystone (§1), there is no code drift to reconcile — the entire re-audit is "does the code the
   0052 acceptance blessed actually satisfy the foundations." Record genuinely-closed properties
   as **present**; report live defects where a "pass" does not hold; and remain open to
   violations none of the five passes named.

2. **Co-equal, load-bearing process-integrity diagnosis and fail-closed-acceptance mechanism.**
   This is the distinctive new ask of the fifth pass and is **co-equal in weight** with the code
   re-audit — not an appended note. Two halves:
   - **Diagnosis.** Using the five acceptance artifacts and their qualified "pass" rows as the
     corpus, diagnose *why the cycle persists at the process level*: how did each acceptance
     render "pass" (or "scoped pass") over code a later pass found non-conformant, and what
     structural property of the spec→ticket→implement→accept→archive process let
     remediation specs flag violations their own implementation then failed to fix? Name concrete
     mechanisms (e.g. self-scoped acceptance where the implementing line writes its own pass
     verdict; "scoped remediation"/"pass with disposition" wording that launders an open
     violation into an accepted artifact; focused-vs-standing mutation scoping that moves the
     goalposts; routed-forward dispositions that never get a forcing function; reachability
     witnesses that test a witness path rather than the production boundary; doc-truthing that
     updates the conformance index to match the code instead of the doctrine).
   - **Mechanism.** Design a durable, enforced **fail-closed acceptance/verification** mechanism
     so a "pass" cannot be rendered while a flagged foundational violation persists. It must
     extend the repository's own machinery and doctrine (execution-10 evidence-honesty /
     merge-blocking-red rules; architecture-13 acceptance-artifact contract; `.cargo/mutants.toml`
     and CI jobs; the cert ladder placement), treating the recurring acceptance-honesty failure
     as itself the regression vector. Prefer compile-time / CI-enforced / governance-enforced
     forcing functions over reviewer discipline. Where a doctrine strengthening is the right
     mechanism, route it through §3.5 (substance + home only); where a code/CI/governance
     mechanism suffices, specify it concretely. Place anything ladder-relevant correctly **without
     minting a new gate code, invariant, risk ID, or glossary term.**

3. **The enforced standing gate (code-surface anti-regression) remains required.** Independently
   of the process mechanism, re-evaluate the enforced standing gate 0052 built (the
   `public-boundary-conformance` lane, the `full-surface-mutation-trigger` /
   `mutants-lock-layer-reconcile` jobs, the compile-time unrepresentability layer) and specify
   what still must change to make "read-only client", "production reachability", and "green
   surface mutation perimeter" *enforced* rather than programmer discipline — including the fact
   that **the gate is not enforced at merge because `main` is unprotected** (see §3.4). Demote
   source-text guards explicitly to topology alarms; keep compile-time unrepresentability
   (private fields / crate-private constructors / unexported authority tokens on the real
   production symbols) as the load-bearing layer.

4. **The two 0052 residuals are dispositioned explicitly.**
   - **Branch protection on `main` is unenforced** (the 0052 acceptance recorded `Branch not
     protected (HTTP 404)`). This is a **first-class anti-regression finding**, not a footnote:
     the entire enforced-standing-gate design 0052 built is inert at merge time without it.
     Specify the governance remediation and how the process mechanism (§3.2) treats "the gate
     exists but is not enforced" as a fail-closed condition.
   - The **seven `food_source_fact_supersedes` mutation survivors** stay **routed-forward** as a
     cross-cutting concern (consistent with 0052's own disposition — state this; do not re-route
     to a lower tier, mutation evidence is cross-cutting, not lowest-tier). Require explicit
     handling and a forcing function that prevents the routed-forward disposition from persisting
     indefinitely, but it is **not** an in-surface closure target for this pass. Make no
     equivalence claim about any survivor without a defensible semantic argument; "the suite did
     not kill it" is not proof of equivalence.

5. **Conditional foundation/doctrine-amendment branch is retained, re-decided independently, and
   explicitly extended to the process/verification layer.** Determine, with evidence, whether any
   tier (`0-foundation` … `3-reference`) doctrine needs amendment for this surface **or for the
   acceptance/verification process**. The prior four passes each concluded **no amendment is
   warranted** (the doctrine already says what the code must do; the defects are the code falling
   below the rules). Reach your own verdict — but note the new process angle makes a doctrinal gap
   more plausible than before: if the recurrence is partly *because* execution-10 / architecture-13
   evidence-honesty doctrine is too weak to forbid a self-scoped "pass-with-disposition" over an
   open violation, that is a genuine doctrine gap. If — and only if — you find a genuine gap or
   contradiction that blocks remediation or perpetuates the cycle, recommend the amendment as
   **substance + home** (what doctrine the target doc must own, in your own prose at that tier's
   altitude, and which file/section it lands in) — **never** final paste-ready wording and
   **never** an invented identifier (`INV-###`, gate code, risk ID, glossary term); identifier
   minting and ratified text remain the repo's own reassess/amend process. Do not weaken doctrine
   to bless caller-seeded registries, diagnostic process markers, client-owned aggregates, exact
   hidden time, a perpetual non-green perimeter, or a self-scoped pass over an open violation —
   that would be a constitutional inversion.

6. **Static-survey-only.** You cannot run `cargo fmt/clippy/build/test`, replay/golden lanes, CI,
   or `cargo-mutants`. Every statement about current test strength, mutation result, or pass/fail
   is a **preliminary static judgment about source shape, API authority, data flow, and witness
   intent** — explicitly non-certifying. Authoritative pass/fail belongs to the implementing
   session that executes the gates from a clean baseline. Quote any command outcome from a ticket
   or acceptance artifact as a **historical claim by that artifact**, never as a current result.

7. **Extend existing machinery; introduce no new test-framework dependency.** `.cargo/mutants.toml`,
   `anti_regression_guards.rs`, the external-crate negative-fixture pattern +
   `negative_fixture_runner.rs`, `generative_lock.rs`, `ci_workflow_guards.rs`, the
   coordinator/replay/interval/salience/reservation/parity suites, and the CI jobs already exist.
   Recommendations extend them. Do **not** recommend adding `proptest`, `quickcheck`, or another
   property-testing framework — the required properties are expressible with the existing
   deterministic fixtures, generated corpus, integration seams, negative fixtures, and mutation
   campaigns. No backwards-compatibility shims or alias paths in any recommended change.

---

## 4. The task

Determine whether the code implementing the spec-0047 surface — as it stands at commit `e9792dc`,
byte-identical to the spec-0052 keystone — **still violates the foundations**, and if so, specify
how to remediate it, harden it, and **make future regression as close to impossible as feasible**.
Co-equally, diagnose and design a fix for the *process-level* recurrence: why four consecutive
acceptance artifacts rendered "pass" while violations persisted, and how to make the
acceptance/verification process itself fail-closed so a "pass" cannot coexist with a live flagged
violation. This is a cross-cutting **hardening / anti-contamination re-audit** (fifth pass) with a
secondary **foundational/doc-overhaul** branch (conditional amendment determination per §3.5,
extended to the process/verification layer). Re-verify every load-bearing property of the surface
from the current code under the repository's authority order; render an explicit conformance
verdict; for each violation, name the controlling invariants, the current code state, the
conformance verdict, the code + `docs/**` remediation home, the strongest practical
anti-regression guard, and an evidence-honesty check; disposition the two 0052 residuals (§3.4);
re-evaluate the enforced standing gate; and — as a co-equal load-bearing contribution — deliver
the process-integrity diagnosis and fail-closed-acceptance mechanism (§3.2). Claude Code will
later analyze your report and produce the numbered remediation spec; **you do not write the
spec.**

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow every code
seam, test, fixture, and CI definition that bears on the surface, and read every prior acceptance
artifact whose verdict wording feeds the process-integrity analysis. Research online as deeply as
needed — similar implementations, research papers, and prior art — wherever it sharpens the
remediation, the regression barrier, or the process mechanism. The fourth-pass report already
opened a useful external lane you may extend: Rust visibility/privacy and the `non_exhaustive`
limitation; Schneider's state-machine replication; Temporal's replay-from-history model;
Sabelfeld & Myers on language-based information-flow (access control ≠ information-flow control);
and the cargo-mutants guidance on missed mutants / timeouts plus the mutation-testing survey on
undecidable equivalence. For the process-integrity contribution, prior art on **fail-closed
verification, independent/adversarial acceptance, goal displacement (Goodhart's law), and CI
required-status-check enforcement / branch protection** is directly on point. Keep the
external-research lane strictly separate from repository evidence, and cite every external source
that shapes a decision.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every product-behavior
  decision must satisfy it; a genuine divergence requires amending an invariant first, never
  designing against it silently (and amendment substance only — see §3.5).
- **Authority order is absolute:** if execution conflicts with architecture or foundation,
  execution is wrong; if the implementation is more convenient than the accepted gates, the
  implementation is wrong. Never weaken an upstream tier to fit downstream code or to bless a
  convenient acceptance.
- **No simulation fact may be born from prose.** Preserve event-sourced causality, subjective
  epistemics, ordinary agents, possession parity, fallible institutions, validation/replay, and
  the actor-known truth firewall.
- No backwards-compatibility shims or alias paths in new work.
- Do not edit any archived spec, ticket, acceptance artifact, or passed certification; live-doc
  work is status/navigation/conformance-evidence only, and only after the code and executable
  witnesses exist. Mint no new invariant, gate, risk ID, or glossary term.
- Source-text guards are topology alarms only — never sole proof of atomicity, replay
  continuation, process semantics, one-opportunity-per-actor, information-flow noninterference,
  production reachability, mutation sensitivity, or acceptance honesty. Those require type
  boundaries, executable behavior, and (for the process layer) CI/governance enforcement.

---

## 7. Deliverable specification

Produce **exactly one** downloadable markdown document, **new** (not a replacement):

```
0047-foundational-hardening-research-report-fifth-pass.md
```

This continues the lineage filenames `0047-foundational-hardening-research-report.md`,
`-second-pass.md`, `-third-pass.md`, `-fourth-pass.md`. It is an **analysis / recommendation
report, not a numbered spec**: Claude Code derives the numbered remediation spec from it
afterward (it would stage next in the `00xx` sequence after `0052`), so the numbering / ledger /
epoch rules do **not** apply to you and you must not assign a spec number or write a `specs/`-style
artifact.

**Production mode — always produce, with the verdict as a section (mode i).** Produce the report
unconditionally. Open it with an explicit, evidence-based **verdict**: is the spec-0047 surface
foundationally conformant at `e9792dc`; is the acceptance/verification *process* sound; and is any
doctrine amendment warranted. The report's value survives any outcome — even a clean code verdict
locks the now-correct properties, specifies the regression barrier, and delivers the process
mechanism. (Given the four-pass history, the routed-forward survivors, and the unenforced branch
protection, a *fully* clean verdict is unlikely, but render whatever the current code and process
actually support; do not manufacture findings, and record genuinely-fixed properties as
**present**.)

Reuse the canonical shape the prior reports established, scaled to what you find, with the
process-integrity section co-equal to the code sections:

1. **Verdict** — code conformant / not conformant, with decisive reasons; the
   process-integrity verdict (is the acceptance process sound); and the explicit higher-tier
   amendment verdict (§3.5).
2. **Disposition table** — one row per finding → primary code/doc/process target → classification
   (violation / vacuity gap / hardening gap / mutation-survivor disposition / evidence-honesty
   gap / process-integrity gap) → one-line basis citing invariants or governing doctrine.
3. **Method & provenance ledger** — authority order applied; the source-equivalence posture (the
   0052-remediated source is byte-identical to baseline `e9792dc`, so the 0052 acceptance
   describes the exact audited code, and its "pass" rows are re-verified, not inherited);
   exact-commit acquisition discipline (fetch every file by full exact-commit path from
   `e9792dc`; manifest is inventory only); and the static-survey limitation.
4. **Per-finding sections (code surface)** — for each finding: foundational driver (named
   invariants + governing architecture/execution doctrine) → current `e9792dc` code state (cite
   the real post-0052 symbols/paths) → conformance verdict → required remediation (code home +
   `docs/**` home, honoring authority order) → strongest practical anti-regression guard
   (compile-time / behavior / differential / mutation) → evidence-honesty check (what a
   non-vacuous closure witness must do, and what would make it vacuous).
5. **Residual disposition** — the seven `food_source_fact_supersedes` survivors (routed-forward
   cross-cutting + forcing function) and the unenforced branch protection (first-class governance
   finding), per §3.4, with no unjustified equivalence claims.
6. **Structural anti-regression / enforced standing gate** — why the seam reopened (or did not)
   despite the 0052 enforced gate; what still must change to make the gate enforced rather than
   declarative (CI conformance lane through the public boundary; green standing mutation perimeter
   for the surface + its governance; compile-time unrepresentability as the load-bearing layer;
   correct placement relative to the cert ladder without minting identifiers).
7. **Process-integrity (co-equal load-bearing contribution)** — the recurrence diagnosis across
   the five acceptance artifacts (how "pass" was rendered over later-falsified code; the
   structural cause of flagged-but-unfixed violations) and the durable **fail-closed
   acceptance/verification mechanism** (§3.2) that makes a "pass" impossible while a flagged
   foundational violation persists, anchored in execution-10 / architecture-13 doctrine and the
   existing CI / mutation / governance machinery.
8. **Foundation & documentation determination** — the amendment verdict with reasoning (code and
   process layers), and the post-implementation live-doc work table (architecture conformance
   `00`; architecture `02/04/05/10/13`; execution `05/06/07/10`; reference `00`/`01`
   R-27/R-28/R-29 status only; ledger through the normal process), explicitly **after**
   executable closure, editing no archived material and minting no identifier.
9. **Recommended closure order**, **open maintainer decisions** (implementation choices inside
   settled doctrine), **self-check**, and **references** (repository evidence and external
   research kept in separate lanes).

**Locked / no-questions instruction:** Produce the deliverable directly as a downloadable
markdown document. Do not interview, do not ask clarifying questions — the requirements above are
final. If a genuine contradiction makes a requirement impossible, state it in the deliverable and
proceed with the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The verdict is explicit and evidence-based, covering foundational code conformance, the
      acceptance-process soundness, and the higher-tier amendment determination.
- [ ] Every file used for target-state claims was fetched from the full exact commit
      `e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f`; every §2 path was present in the uploaded
      manifest.
- [ ] The 0052 acceptance's "pass" rows were re-verified against the current post-0052 code
      (which is byte-identical to its keystone), not inherited; genuinely-closed properties are
      recorded as **present**, not re-reported as defects; new violations not named by prior
      passes were sought.
- [ ] Every code finding names controlling invariants and both a code home and a `docs/**` home,
      in authority order, weakening no upstream tier.
- [ ] The two 0052 residuals are dispositioned: branch protection as a first-class governance
      finding, the `food_source` family as routed-forward cross-cutting with a forcing function;
      no survivor is called equivalent without a semantic argument.
- [ ] The report contains a co-equal, load-bearing process-integrity section that diagnoses the
      five-pass acceptance recurrence and designs a durable **fail-closed** acceptance/verification
      mechanism — not merely per-finding guards or a code-only standing gate.
- [ ] Recommendations extend existing machinery (`.cargo/mutants.toml`, `anti_regression_guards.rs`,
      negative-fixture pattern, `generative_lock.rs`, `ci_workflow_guards.rs`, CI jobs); no new
      property-testing dependency; no backwards-compat shim.
- [ ] No archived artifact is edited; no invariant/gate/risk/glossary identifier is minted; no
      ratified doctrine wording is authored (amendment substance + home only, if any — code or
      process layer).
- [ ] Static-survey limits are explicit; no current command is asserted green or red; ticket/
      acceptance command outcomes are quoted as historical claims.
- [ ] Repository evidence and external research are in separate lanes; every external claim that
      shapes a decision is cited.
- [ ] The deliverable is exactly one new markdown file named
      `0047-foundational-hardening-research-report-fifth-pass.md`; no spec number assigned.
