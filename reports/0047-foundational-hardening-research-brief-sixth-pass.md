# Spec-0047 foundational-conformance hardening — sixth-pass research brief

**You are ChatGPT-Pro Session 2.** This document is your complete, final assignment. It is
self-contained: you see only this prompt plus one uploaded manifest. Produce the deliverable in
§7 directly. **Do not interview, do not ask clarifying questions** — the requirements below are
settled. If a genuine contradiction makes a requirement impossible, state it inside the
deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-27_7660051.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first playability).
Docs are layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` →
`4-specs`; **earlier tiers govern later ones**. Fetch every file from commit
`7660051747aaa1c768cca9dc73385b16573ebe67` — the manifest reflects exactly that tree. The
manifest is a path inventory only; it is not file-content evidence. Branch names,
default-branch lookups, connector namespace labels, repository metadata, and code-search
snippets are not proof of target-commit content; fetch by exact-commit file path.

### This is the sixth audit pass on one recurring seam

The "spec-0047 surface" is the loaded-world / time-control / TUI-authority seam introduced by
`archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`
(TUI-authoritative world advance, duration completion, no-human progression, actor-known
interval summaries). It has now been audited and remediated **five** times in a row, and **each
remediation line that claimed closure was followed by an audit that still found critical
foundational violations.** You are the sixth pass.

The lineage (name each as a delta; do not re-commission completed work):

- **First pass** → `reports/0047-foundational-hardening-research-report.md` → implemented as
  spec `0048` (`archive/specs/0048_FOUNDATIONAL_CONFORMANCE_HARDENING_…`).
- **Second pass** → `reports/0047-foundational-hardening-research-report-second-pass.md` →
  implemented as spec `0050`
  (`archive/specs/0050_FOUNDATIONAL_CONFORMANCE_SECOND_HARDENING_…`).
- **Third pass** → `reports/0047-foundational-hardening-research-report-third-pass.md` →
  implemented as spec `0051`
  (`archive/specs/0051_FOUNDATIONAL_CONFORMANCE_THIRD_HARDENING_CORE_OWNED_LOADED_WORLD_RUNTIME_…`).
- **Fourth pass** → `reports/0047-foundational-hardening-research-report-fourth-pass.md` →
  implemented as spec `0052`
  (`archive/specs/0052_FOUNDATIONAL_CONFORMANCE_FOURTH_HARDENING_PRODUCTION_BOOTSTRAP_CLOSED_COMMAND_BOUNDARY_…`).
- **Fifth pass** (your immediate predecessor, target commit `e9792dc`) →
  `reports/0047-foundational-hardening-research-report-fifth-pass.md` → implemented as spec
  **`0053`**
  (`archive/specs/0053_FOUNDATIONAL_CONFORMANCE_FIFTH_HARDENING_SEALED_BOOTSTRAP_INTERVAL_PRODUCT_TOKENIZED_DEBUG_AUTHORITY_MERGE_ENFORCED_BARRIER_AND_FAIL_CLOSED_ACCEPTANCE_TAXONOMY_HARDENING_SPEC.md`).

The **fifth-pass report is your richest seed** — read it in full. Its six findings
(**F5-01 … F5-06**) are the specification spec 0053 was built to satisfy; its recurrence analysis
diagnosed *why the seam keeps reopening* and proposed the **fail-closed acceptance / verification
mechanism** that spec 0053 then built. Your job is to determine whether spec 0053 actually closed
the authority class — or whether the seam has reopened a sixth time — **and**, as the distinctive
new ask of this pass, to audit the *process-fix itself*: the fail-closed acceptance taxonomy that
0053 implemented (and the 0053 acceptance artifact produced under it) are now first-class audit
targets, not assumed-good machinery.

### The distinctive sixth-pass turn: the recurrence's own fix is now on trial

Every prior pass diagnosed the recurrence and recommended a barrier; the next pass found the
barrier inert or bypassed. The fifth pass recommended, and spec 0053 built, the most direct
counter yet: a machine-checkable **fail-closed acceptance result taxonomy** —
`acceptance_status_manifest.rs` (a parser that recomputes the overall result from the fenced
`tracewake-acceptance-status` block), `acceptance_artifact_wording.rs` (a guard that rejects
closure wording over non-pass status, green-perimeter claims with survivors, governance claims
without an API transcript, historical-as-current claims, and display-only behavior evidence), the
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` shape it enforces, and a merge-enforced `main`
ruleset.

**The 0053 acceptance artifact is the first acceptance produced under that new machinery, and it
rendered an unqualified `overall_result: pass` with all six findings closed.** That is exactly the
shape five prior "pass" artifacts took before a later pass falsified them. So the sixth pass must
treat the taxonomy mechanism as *the thing on trial*: does the parser/wording guard make a
laundered pass genuinely **impossible**, or is it a new, more sophisticated laundering vector — a
self-authored status block, scored by a guard the same line writes and "passes," whose own
correctness is asserted rather than independently proven? This audit of the fix is **co-equal** in
weight with the code re-audit (§3).

### Source-equivalence posture (re-audit of the literal 0053-remediated code)

Spec 0053's commit under test is `34bad05f9ac0c3ca79500c48c8d4b992ad16ee08` (merged to `main` via
PR #65); its capstone code / standing-mutation-campaign commit is
`dacf998ba7aee49818573c8f834175a80ac53da5`. Both are ancestors of your fetch baseline
`7660051`. **A verified `git diff` from the 0053 commit-under-test `34bad05` to the baseline
`7660051` touches only three files — the archival of the 0053 spec and acceptance artifact
(`reports/` → `archive/`) and the `docs/4-specs/SPEC_LEDGER.md` row update. Every file under
`crates/`, `.github/`, and `.cargo/` is byte-identical between `34bad05` and `7660051`.**
Concretely this means: **`archive/reports/0053_foundational_conformance_fifth_hardening_acceptance.md`
describes the exact source you are auditing.** Treat that as an *advantage and a trap*, not a
continuation:

- It is an advantage because there is no intervening code drift to reconcile — the 0053
  acceptance's per-finding closure table (F5-01 … F5-06) and the `tracewake-acceptance-status`
  block map onto the code at `7660051` line-for-line.
- It is a trap because **this is precisely the recurrence you are auditing**: six prior
  acceptance artifacts (0047, 0048, 0050, 0051, 0052, and now 0053 under the new taxonomy)
  rendered "pass" / "scoped pass" over code a later pass found non-conformant. So you must
  **re-derive every property from the post-0053 code itself** and treat the 0053 acceptance's
  verdicts — every `closed` row, the `branch_protection: enforced` claim, the
  `mutation_survivors: none` claim, and the fail-closed-taxonomy self-certification — as
  **unverified claims**, not established facts. Where 0053 genuinely closed a property, record it
  as **present**; where its "pass" does not bear out against the code, report the live defect; and
  remain open to violations none of the six passes (including this acceptance) named.

The 0053 acceptance is immutable historical evidence: you neither edit it nor treat its "pass"
rows as proof of current conformance.

### Two residuals the 0053 acceptance itself flags or leaves implicit

1. The 0053 acceptance records that **classic GitHub branch protection on `main` returns 404 by
   design** and that enforcement is instead a **repository ruleset** (`main-standing-conformance-barrier`,
   id 18200914). The ruleset transcript it quotes shows `enforcement: active`, `bypass_actors: []`,
   `current_user_can_bypass: never`, the seven required status checks, and a `pull_request` rule
   with **`required_approving_review_count: 0`**. The sole maintainer is both implementer and
   acceptor and can open and self-merge a PR with zero approvals once checks are green. Whether a
   **0-approval, self-merge-able barrier** satisfies the *merge-enforcement and independent-acceptance*
   intent (F5-04), or is a governance vacuity, is a first-class question for this pass (§3.5).
2. The 0053 acceptance claims the `food_source_fact_supersedes` survivor family is now fully
   killed (`mutation_survivors: none`) via a full standing campaign at `dacf998`
   (`3423` selected, `2666` caught, `757` unviable, `0` missed, `0` timeouts). Re-verify that the
   **forcing function** (the behavior tests in `food_source_projection.rs` and the standing
   perimeter config) actually prevents this routed-forward pattern from silently recurring, and
   make no equivalence claim about any mutant without a defensible semantic argument (§3.6).

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
  tier-fit test for any amendment proposal (including a *process / verification-integrity*
  amendment — confirm whether such a rule belongs in the constitution at all vs. the execution
  tier) and to confirm what is out of the spec-0047 surface.

**Tier 1 — architecture:**

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **primary.** The conformance
  index; 0053 claims to have truthed its loaded-world rows. Re-verify the rows against the current
  production path.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **primary.**
  Replay/save/restore contract behind the replay-authority findings.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **primary.**
  Firewall/provenance contract behind the sealed-product and interval-leak findings.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` —
  **primary.** Pipeline + one core-owned world-step coordinator; the sealed validated-bootstrap
  evidence row (F5-01).
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` —
  **primary.** Canonical cognition transaction the scheduler must consume.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` —
  **primary.** Read-only client boundary; TUI must not own/mutate aggregates or construct
  embodied/debug products directly (F5-02, F5-03).
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` —
  **primary.** Evidence-honesty / no-decorative-locks contract behind the mutation-honesty
  finding, the anti-regression gate, **and the acceptance-artifact contract** — this is the
  architecture-tier home for "what an acceptance artifact may and may not claim," which the 0053
  taxonomy operationalizes. Re-verify that the doctrine 0053 claims to have strengthened here was
  not weakened to bless a self-scoped "pass with disposition."
- `docs/1-architecture/01,06,07,08,09,11,12,14` — **boundary-awareness.**

**Tier 2 — execution:**

- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **primary.** The
  P0/SPINE/EPI/ORD-LIFE/FIRST-PROOF certification ladder; read it to place any recommended
  standing gate or process mechanism correctly relative to the existing gates **without minting a
  new gate code.**
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` —
  **primary.** Due-work belongs to the core world-step boundary; no parallel possessed-actor
  path; no caller-injected actor lists; bootstrap unforgeability obligation.
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **primary.**
  No-human progression and held-equal possession parity; the no-human-day command's classification.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **primary.** TUI
  stores/renders the core interval product read-only; embodied/debug split; token-gated debug.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **primary
  (the doctrine home for the process-integrity contribution).** Evidence-honesty rule,
  missed-mutant/timeout discipline, real-public-boundary testing, the merge-blocking-red /
  pending-is-not-a-pass rule, **and the fail-closed acceptance result taxonomy 0053 anchored here.**
  Re-verify the taxonomy rule against the code that implements it; this is where the audit of the
  process-fix lands doctrinally.
- `docs/2-execution/00,01,02,04,08,09,11,12,13` — **boundary-awareness.**

**Tier 3 — reference:**

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **primary.** Reviewer pointers
  the report's doc-home recommendations update.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — **primary.** The R-27/R-28/R-29 evidence/status
  rows (mint no new risk ID). `02_GLOSSARY.md` — **boundary-awareness.**

**Tier 4 — specs, plus reports / archive (history & lineage; not live authority):**

- `docs/4-specs/SPEC_LEDGER.md` — **primary.** Source/navigation discipline; the
  0047/0048/0050/0051/0052/0053 archived rows, the `0049MUTWIT` ticket-only record, and the "Next
  known execution move" section. Confirm any new remediation spec routes through the normal ledger
  process; mint nothing.
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — **primary.** The acceptance-artifact
  template the 0053 fail-closed taxonomy enforces (the `tracewake-acceptance-status` block shape,
  the result-computation contract, the forbidden-wording rules). Central to the process-fix audit:
  does the template/parser leave a hole through which a self-scoped pass can still pass?
- `reports/0047-foundational-hardening-research-report-fifth-pass.md` — **primary (richest
  seed).** Your predecessor's full findings (F5-01 … F5-06), recurrence analysis, and the
  fail-closed acceptance mechanism proposal that spec 0053 implemented.
- `reports/0047-foundational-hardening-research-brief-fifth-pass.md` — **boundary-awareness.**
  The locked scope/lineage the fifth pass worked to; confirms conventions and the canonical
  report shape.
- `archive/specs/0053_…_HARDENING_SPEC.md` — **primary.** The promises the fifth remediation line
  made; the spec whose claims you re-verify against the code.
- `archive/reports/0053_foundational_conformance_fifth_hardening_acceptance.md` — **primary.**
  What 0053 *claims* it closed: the `tracewake-acceptance-status` block, the per-finding closure
  table (F5-01 … F5-06), the mutation command ledger and full standing campaign disposition, the
  ruleset / governance API transcripts, and the evidence-item ledger. Your concrete re-verification
  seed; describes the exact source you audit (source-equivalence posture, §1).
- `reports/0047-foundational-hardening-research-report-fourth-pass.md`,
  `archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md`,
  `archive/reports/0051_foundational_conformance_third_hardening_acceptance.md`,
  `archive/reports/0050_foundational_conformance_second_hardening_acceptance.md`,
  `reports/0048_foundational_conformance_hardening_acceptance.md`,
  `archive/reports/0047_tui_authoritative_world_advance_acceptance.md` —
  **boundary-awareness.** Prior remediation epochs and the original feature-surface boundary;
  historical evidence and recurrence raw material. **For the process-integrity analysis (§3.3)
  these are primary corpus:** read each prior acceptance's verdict wording and qualified "pass"
  rows to reconstruct *how* "pass" was rendered each time over code a later pass found
  non-conformant, and judge whether the 0053 taxonomy would have blocked each historical overclaim.
- `archive/specs/0050_…SPEC.md`, `archive/specs/0051_…SPEC.md`,
  `archive/specs/0052_…SPEC.md` — **boundary-awareness.** The promises the prior remediation lines
  made.
- `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt` — **primary.** The existing standing
  mutation perimeter config and the survivor/baseline-miss registry the anti-regression
  recommendation must extend, not duplicate.

### Code seams to inspect (read directly at the baseline; do not expect them pasted here)

Audit the post-0053 source at `7660051` (byte-identical to the 0053 commit-under-test `34bad05`),
not the fifth-pass quotes of it. The fifth pass already named the loaded-world / TUI seams; the
**bolded entries below are the surfaces 0053 newly created or changed**, which carry the highest
re-audit priority because they are unproven by any pass:

- `crates/tracewake-core/src/runtime/{mod,session,command,receipt}.rs` — `LoadedWorldRuntime`,
  **the `ValidatedLoadedWorldBootstrap` sealed constructor and `from_bootstrap` (F5-01 closure)**,
  the closed `RuntimeCommand` dispatch, and **`ContinuedRuntimeReceipt` / the sealed
  embodied/debug receipt products (F5-02 closure)**.
- **`crates/tracewake-core/src/debug_capability.rs`** — the `DebugSessionAuthority` token and
  `debug_only` path that F5-03 introduced; re-verify the token is runtime-minted and unforgeable
  at the public boundary, not a decorative capability minted inside a public constructor.
- `crates/tracewake-core/src/scheduler.rs` — `transact_world_one_tick`, due-actor / due-process
  discovery, the per-tick actor disposition census, replay restore, declared-process authority.
- `crates/tracewake-core/src/state.rs` — `PhysicalState` / `AgentState` seed-part constructors;
  re-verify whether raw seed-part construction remains publicly reachable (F5-01 attack surface).
- `crates/tracewake-core/src/events/{envelope,apply,log}.rs` — declared-process event class &
  application; duplicate-`EventId` fail-closed.
- `crates/tracewake-core/src/agent/{transaction,trace,perception}.rs` — actor decision
  transaction consumption; `StuckDiagnostic`; perception write path.
- `crates/tracewake-core/src/epistemics/projection.rs`,
  `crates/tracewake-core/src/projections.rs` — `insert_observation`, interval-delta/salience,
  **`food_source_fact_supersedes` and `IntervalStopReason::stable_id` (F5-05 closure)**.
- `crates/tracewake-core/src/view_models.rs` — `EmbodiedViewModel`,
  `TypedActorKnownIntervalSummary` (field/constructor/accessor sealing the F5-02 closure claims).
- `crates/tracewake-core/src/replay/{temporal,rebuild,report}.rs`,
  `crates/tracewake-core/src/actions/pipeline.rs`.
- `crates/tracewake-content/src/{load,schema}.rs` — loaded-world handoff into the runtime
  constructor (`into_runtime_bootstrap`); the validated path that must be the *only* way to mint a
  bootstrap.
- `crates/tracewake-tui/src/{app,run,transcript,render,input,launch,debug_panels,lib,main}.rs` —
  `TuiApp::from_golden`, semantic-action submission, advance/continue, command-loop dispatch,
  debug-mode gating; embodied rendering leakage and any residual client-side interval construction.
- **The fail-closed acceptance taxonomy machinery (the distinctive sixth-pass target):**
  `crates/tracewake-core/tests/acceptance_status_manifest.rs`,
  `crates/tracewake-core/tests/acceptance_artifact_wording.rs`,
  `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`, and
  `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`. Audit whether the parser actually
  *recomputes* a fail-closed result rather than reading a self-declared `overall_result`; whether
  the wording guard's rejection set is complete and not bypassable by paraphrase; whether the
  machinery's own correctness is mutation-covered (a guard whose mutants survive is decorative);
  and whether anything forces an acceptance artifact through the parser before archival/merge.
- The rest of the test / negative-fixture / CI infrastructure:
  `crates/tracewake-core/tests/*` (esp. `world_step_coordinator.rs`, `negative_fixture_runner.rs`,
  `anti_regression_guards.rs`, `generative_lock.rs`, `ci_workflow_guards.rs`,
  `replay_temporal_frontier.rs`, `holder_known_interval_projection.rs`,
  `salient_stop_actor_known.rs`, `reservation_body_exclusive_census.rs`,
  `mutation_completion_merge.rs`, `food_source_projection.rs`), `crates/tracewake-tui/tests/*`
  (esp. `tui_seam_conformance.rs`, `command_loop_session.rs`, `playable_capability_parity.rs`,
  `embodied_flow.rs`, `parity_adversarial.rs`, `transcript_snapshot.rs`), the external negative
  fixtures under `tests/negative-fixtures/` (esp. the 0053-added
  `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts`,
  `external_crate_cannot_submit_debug_command_without_token`,
  `external_crate_cannot_construct_actor_interval_summary`,
  `external_crate_cannot_mutate_embodied_temporal_fields`,
  `external_crate_cannot_convert_debug_report_to_interval_summary`), and
  `.github/workflows/ci.yml` (the `public-boundary-conformance`, `full-surface-mutation-trigger`,
  `mutants-lock-layer-reconcile`, and **`governance-required-checks-audit`** jobs).

---

## 3. Settled intentions (these are decisions, not open questions)

1. **Full independent re-audit of the entire spec-0047 surface at `7660051`.** Re-derive
   conformance for the whole loaded-world / time-control / TUI-authority surface from the
   post-0053 code. Treat the 0053 acceptance's `closed` rows, its `overall_result: pass`, the
   `branch_protection: enforced` and `mutation_survivors: none` claims, and every qualified verdict
   as **unverified claims**, not facts. Rationale: five consecutive remediation lines each declared
   closure and were each followed by a pass that found critical violations; a delta-only audit
   would inherit 0053's blind spots. Because the source is byte-identical to the 0053 commit under
   test (§1), there is no code drift to reconcile — the entire re-audit is "does the code the 0053
   acceptance blessed actually satisfy the foundations." Record genuinely-closed properties as
   **present**; report live defects where a "pass" does not hold; and remain open to violations
   none of the six passes named.

2. **Co-equal, load-bearing audit of the process-fix itself (the distinctive sixth-pass ask).**
   Spec 0053 *built* the fail-closed acceptance / verification machinery the fifth pass
   recommended, and the 0053 acceptance is the **first artifact produced under it** — yet it
   rendered an unqualified `pass`. Auditing that machinery is **co-equal in weight** with the code
   re-audit, not an appended note. Determine, from the code:
   - whether `acceptance_status_manifest.rs` genuinely **recomputes** a fail-closed overall result
     from the constituent statuses / mutation disposition / governance state, or merely validates a
     self-declared `overall_result` for internal consistency (a self-consistent block can still be
     a self-authored pass);
   - whether `acceptance_artifact_wording.rs`'s rejection set (closure wording over non-pass
     status, green-perimeter claims with survivors, governance claims without an API transcript,
     historical-as-current claims, display-only behavior evidence) is **complete and not bypassable
     by paraphrase** — i.e., is it a closed grammar or a denylist of phrases;
   - whether the taxonomy machinery's **own correctness is independently proven** (mutation
     coverage of the parser/guard, adversarial/synthetic manifests, negative cases) rather than
     asserted — a guard whose own mutants survive is decorative;
   - whether anything **forces** an acceptance artifact through the parser/guard before
     archival/merge (a forcing function in CI / the ruleset), or whether running them is reviewer
     discipline;
   - and, most pointedly, whether the **0053 acceptance itself would survive a genuinely
     adversarial application** of its own taxonomy — apply the parser/guard's rules to the 0053
     artifact's wording and status block and report any place it should have failed but did not
     (e.g. "scoped pass," "carries forward," "by design," routed/deferred evidence treated as
     certifying). Name the structural laundering vector if one exists (self-authored status block;
     implementer-as-acceptor; guard-scored-by-its-own-author) and specify the durable forcing
     function that closes it, **without minting a new gate code, invariant, risk ID, or glossary
     term**, routing any doctrine strengthening through §3.7 (substance + home only).

3. **Process-integrity recurrence diagnosis across all six acceptance artifacts.** Using the six
   acceptance artifacts (0047, 0048, 0050, 0051, 0052, 0053) and their verdict wording as the
   corpus, diagnose whether the recurrence is now actually arrested or merely moved up a level.
   Concretely: would the 0053 taxonomy have **blocked each historical overclaim** the prior passes
   later falsified? For each prior "pass"/"scoped pass" that a later pass overturned, state whether
   the new parser/guard would have caught it, and where it would not, name the residual hole. The
   goal is a defensible answer to "is the process fixed," not a restatement of the fifth pass.

4. **The enforced standing gate (code-surface anti-regression) remains required and re-verified.**
   Independently of the process mechanism, re-evaluate the enforced standing gate 0053 inherited
   and extended (the `public-boundary-conformance` lane, the `full-surface-mutation-trigger` /
   `mutants-lock-layer-reconcile` jobs, the new `governance-required-checks-audit` job, the
   compile-time unrepresentability layer on the production symbols). Specify what still must change
   to make "read-only client," "production reachability," "sealed bootstrap/receipt/debug
   authority," and "green surface mutation perimeter" *enforced* rather than programmer discipline.
   Demote source-text guards explicitly to topology alarms; keep compile-time unrepresentability
   (private fields / crate-private constructors / unforgeable authority tokens on the real
   production symbols) as the load-bearing layer.

5. **Governance closure (F5-04) is re-scrutinized, not inherited.** The 0053 acceptance closes
   F5-04 via the `main-standing-conformance-barrier` ruleset. Re-derive whether that ruleset
   actually makes the gate merge-enforced and the acceptance independent. Address specifically:
   the `required_approving_review_count: 0` setting and the sole-maintainer-as-both-implementer-
   and-acceptor reality (can a flagged violation still be self-merged once checks are green?); the
   reliance on a ruleset that returns 404 on the classic branch-protection endpoint (does the
   `governance-required-checks-audit` job read the *correct* ruleset-detail endpoint, per the 0053
   closeout, so it cannot silently pass on absent rules?); and whether "the gate exists but the
   acceptor is unconstrained" is itself a fail-closed condition the process mechanism (§3.2) must
   treat as non-pass. Specify the governance remediation as **substance + home**; do not assume the
   ruleset transcript closes the independence question.

6. **The `food_source_fact_supersedes` residual is re-verified, not inherited as closed.** The
   0053 acceptance claims zero survivors via the full standing campaign at `dacf998`. Re-verify
   from the code that `food_source_projection.rs` exercises the replacement semantics through
   public actor-known / projection behavior (not a private-function-only assertion) and that the
   forcing function prevents the routed-forward pattern from silently recurring. Make no
   equivalence claim about any mutant without a defensible semantic argument; "the suite killed it
   historically" is a static-survey historical claim, not a current result (§3, static-survey
   posture).

7. **Conditional foundation/doctrine-amendment branch is retained, re-decided independently, and
   explicitly extended to the process/verification layer.** Determine, with evidence, whether any
   tier (`0-foundation` … `3-reference`) doctrine needs amendment for this surface **or for the
   acceptance/verification process**. The prior six passes concluded **no Tier-0 amendment is
   warranted** (the constitution already says what the code must do); the fifth pass concluded a
   *below-foundation* strengthening was warranted (architecture-13 / execution-10 acceptance-honesty),
   which 0053 claims to have landed. Reach your own verdict — **and re-verify that 0053's doctrine
   edits did not *weaken* any tier** to bless a self-scoped "pass-with-disposition," a 0-approval
   self-merge, exact hidden time, a perpetual non-green perimeter, or caller-seeded authority. If —
   and only if — you find a genuine gap or contradiction that blocks remediation or perpetuates the
   cycle (e.g. the taxonomy doctrine is still too weak to forbid a self-authored status block from
   counting as pass), recommend the amendment as **substance + home** (what doctrine the target doc
   must own, in your own prose at that tier's altitude, and which file/section it lands in) —
   **never** final paste-ready wording and **never** an invented identifier (`INV-###`, gate code,
   risk ID, glossary term); identifier minting and ratified text remain the repo's own
   reassess/amend process.

8. **Static-survey-only.** You cannot run `cargo fmt/clippy/build/test`, replay/golden lanes, CI,
   or `cargo-mutants`. Every statement about current test strength, mutation result, governance
   enforcement, or pass/fail is a **preliminary static judgment about source shape, API authority,
   data flow, witness intent, and CI/ruleset configuration as written** — explicitly non-certifying.
   Authoritative pass/fail belongs to the implementing session that executes the gates from a clean
   baseline. Quote any command outcome, mutation count, or API transcript from a ticket or
   acceptance artifact as a **historical claim by that artifact**, never as a current result.

9. **Extend existing machinery; introduce no new test-framework dependency.** `.cargo/mutants.toml`,
   `anti_regression_guards.rs`, the external-crate negative-fixture pattern +
   `negative_fixture_runner.rs`, `generative_lock.rs`, `ci_workflow_guards.rs`, the
   `acceptance_status_manifest.rs` / `acceptance_artifact_wording.rs` taxonomy machinery, the
   coordinator/replay/interval/salience/reservation/parity suites, and the CI jobs already exist.
   Recommendations extend them. Do **not** recommend adding `proptest`, `quickcheck`, or another
   property-testing framework — the required properties are expressible with the existing
   deterministic fixtures, generated corpus, integration seams, negative fixtures, and mutation
   campaigns. No backwards-compatibility shims or alias paths in any recommended change.

---

## 4. The task

Determine whether the code implementing the spec-0047 surface — as it stands at commit `7660051`,
byte-identical to the spec-0053 commit under test — **still violates the foundations**, and if so,
specify how to remediate it, harden it, and **make future regression as close to impossible as
feasible**. Co-equally, audit the **process-fix itself**: the fail-closed acceptance / verification
taxonomy that spec 0053 built, and the 0053 acceptance produced under it — determine whether that
machinery makes a laundered "pass" genuinely impossible or is a new laundering vector, whether it
would have blocked the prior passes' overturned verdicts, and what durable forcing function closes
any residual hole. This is a cross-cutting **hardening / anti-contamination re-audit** (sixth pass)
with a secondary **foundational/doc-overhaul** branch (conditional amendment determination per
§3.7, extended to the process/verification layer). Re-verify every load-bearing property of the
surface from the current code under the repository's authority order; render an explicit
conformance verdict; for each violation, name the controlling invariants, the current code state,
the conformance verdict, the code + `docs/**` remediation home, the strongest practical
anti-regression guard, and an evidence-honesty check; re-scrutinize the F5-04 governance closure
and the `food_source` residual; re-evaluate the enforced standing gate; and — as a co-equal
load-bearing contribution — deliver the process-integrity diagnosis and the audit of the
fail-closed taxonomy mechanism, with the durable forcing function that makes a "pass" impossible
while a flagged foundational violation persists. Honor the user's standing instruction: **if you
find no foundational violations, state explicitly whether you believe another iteration will be
needed and why.** Claude Code will later analyze your report and produce the numbered remediation
spec; **you do not write the spec.**

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow every code
seam, test, fixture, and CI definition that bears on the surface, and read every prior acceptance
artifact whose verdict wording feeds the process-integrity analysis. Research online as deeply as
needed — similar implementations, research papers, and prior art — wherever it sharpens the
remediation, the regression barrier, or the process-fix audit. The prior passes already opened a
useful external lane you may extend: Rust visibility/privacy and the `non_exhaustive` limitation;
Schneider's state-machine replication; Temporal's replay-from-history model; Sabelfeld & Myers on
language-based information-flow (access control ≠ information-flow control); and cargo-mutants
guidance on missed mutants / timeouts plus the mutation-testing survey on undecidable equivalence.
For the process-fix audit specifically, prior art on **fail-closed verification, independent /
adversarial acceptance (separation of implementer and acceptor), goal displacement (Goodhart's
law), specification-gaming, GitHub branch-protection / ruleset required-status-check enforcement,
and required-review-count / self-merge policy** is directly on point. Keep the external-research
lane strictly separate from repository evidence, and cite every external source that shapes a
decision.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every product-behavior
  decision must satisfy it; a genuine divergence requires amending an invariant first, never
  designing against it silently (and amendment substance only — see §3.7).
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
  production reachability, mutation sensitivity, governance enforcement, or acceptance honesty.
  Those require type boundaries, executable behavior, and (for the process layer) CI/governance
  enforcement that the implementing/accepting actor cannot self-bypass.

---

## 7. Deliverable specification

Produce **exactly one** downloadable markdown document, **new** (not a replacement):

```
0047-foundational-hardening-research-report-sixth-pass.md
```

This continues the lineage filenames `0047-foundational-hardening-research-report.md`,
`-second-pass.md`, `-third-pass.md`, `-fourth-pass.md`, `-fifth-pass.md`. It is an **analysis /
recommendation report, not a numbered spec**: Claude Code derives the numbered remediation spec
from it afterward (it would stage next in the `00xx` sequence after `0053`, i.e. `0054`), so the
numbering / ledger / epoch rules do **not** apply to you and you must not assign a spec number or
write a `specs/`-style artifact.

**Production mode — always produce, with the verdict as a section (mode i).** Produce the report
unconditionally. Open it with an explicit, evidence-based **verdict**: is the spec-0047 surface
foundationally conformant at `7660051`; is the fail-closed acceptance / verification *process* now
sound (does the 0053 taxonomy actually make a laundered pass impossible); is the governance closure
genuinely merge-enforced and independent; and is any doctrine amendment warranted. **The report's
value survives any outcome** — even a clean code verdict locks the now-correct properties, audits
the process-fix, and answers the recurrence question. Per the user's standing instruction, if the
verdict is clean, **state explicitly whether you believe another iteration will be needed, with
reasons** (e.g. residual governance independence, an unforced taxonomy, an unproven survivor
forcing function). Given the six-pass history, a *fully* clean verdict on both the code and the
process is unlikely, but render whatever the current code and process actually support; do not
manufacture findings, and record genuinely-fixed properties as **present**.

Reuse the canonical shape the prior reports established, scaled to what you find, with the
process-integrity / taxonomy-audit section co-equal to the code sections:

1. **Verdict** — code conformant / not conformant, with decisive reasons; the process-integrity
   verdict (is the acceptance/verification process now sound, and would the 0053 taxonomy have
   blocked the prior overturned verdicts); the governance-independence verdict; the explicit
   higher-tier amendment verdict (§3.7); and the "another iteration needed?" judgment.
2. **Disposition table** — one row per finding → primary code/doc/process target → classification
   (violation / vacuity gap / hardening gap / mutation-survivor disposition / evidence-honesty gap
   / process-integrity gap / governance gap) → one-line basis citing invariants or governing
   doctrine.
3. **Method & provenance ledger** — authority order applied; the source-equivalence posture (the
   0053-remediated source is byte-identical to baseline `7660051`, so the 0053 acceptance describes
   the exact audited code, and its `closed`/`pass` rows are re-verified, not inherited);
   exact-commit acquisition discipline (fetch every file by full exact-commit path from `7660051`;
   manifest is inventory only); and the static-survey limitation.
4. **Re-verified present properties from 0053** — the genuinely-closed properties (sealed
   `ValidatedLoadedWorldBootstrap`, `ContinuedRuntimeReceipt`/interval sealing, `DebugSessionAuthority`
   token, the new negative fixtures, the taxonomy machinery as far as it genuinely holds) recorded
   as **present** so the pass does not erase real 0053 progress.
5. **Per-finding sections (code surface)** — for each finding: foundational driver (named
   invariants + governing architecture/execution doctrine) → current `7660051` code state (cite
   the real post-0053 symbols/paths) → conformance verdict → required remediation (code home +
   `docs/**` home, honoring authority order) → strongest practical anti-regression guard
   (compile-time / behavior / differential / mutation) → evidence-honesty check (what a
   non-vacuous closure witness must do, and what would make it vacuous).
6. **Residual disposition** — the `food_source_fact_supersedes` family (re-verified forcing
   function, no unjustified equivalence claims) and the governance independence question (F5-04
   re-scrutiny: 0-approval self-merge / implementer-as-acceptor), per §3.5–§3.6.
7. **Structural anti-regression / enforced standing gate** — why the seam did or did not reopen
   despite the 0053 enforced gate; what still must change to make the gate enforced rather than
   declarative (CI conformance lane through the public boundary; green standing mutation perimeter
   for the surface + its governance; compile-time unrepresentability as the load-bearing layer;
   correct placement relative to the cert ladder without minting identifiers).
8. **Process-integrity + fail-closed-taxonomy audit (co-equal load-bearing contribution)** — the
   recurrence diagnosis across the six acceptance artifacts (would the 0053 taxonomy have blocked
   each historical overclaim); the audit of the taxonomy machinery itself (does
   `acceptance_status_manifest.rs` recompute vs. validate; is the wording guard a closed grammar or
   a phrase denylist; is the machinery's own correctness mutation-covered; is there a forcing
   function before merge/archival); the adversarial application of the 0053 taxonomy to the 0053
   acceptance itself; and the durable forcing function that makes a "pass" impossible while a
   flagged foundational violation persists, anchored in execution-10 / architecture-13 doctrine and
   the existing CI / mutation / governance machinery.
9. **Foundation & documentation determination** — the amendment verdict with reasoning (code and
   process layers, including the explicit check that 0053's doctrine edits did not weaken any
   tier), and the post-implementation live-doc work table (architecture conformance `00`;
   architecture `02/04/05/10/13`; execution `05/06/07/10`; reference `00`/`01` R-27/R-28/R-29
   status only; ledger through the normal process), explicitly **after** executable closure,
   editing no archived material and minting no identifier.
10. **Recommended closure order**, **open maintainer decisions** (implementation choices inside
    settled doctrine), **self-check**, and **references** (repository evidence and external
    research kept in separate lanes).

**Locked / no-questions instruction:** Produce the deliverable directly as a downloadable
markdown document. Do not interview, do not ask clarifying questions — the requirements above are
final. If a genuine contradiction makes a requirement impossible, state it in the deliverable and
proceed with the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The verdict is explicit and evidence-based, covering foundational code conformance, the
      acceptance-process soundness (including whether the 0053 taxonomy would have blocked the prior
      overturned verdicts), governance independence, the higher-tier amendment determination, and
      the "another iteration needed?" judgment.
- [ ] Every file used for target-state claims was fetched from the full exact commit
      `7660051747aaa1c768cca9dc73385b16573ebe67`; every §2 path was present in the uploaded
      manifest.
- [ ] The 0053 acceptance's `closed`/`pass` rows, `branch_protection: enforced`, and
      `mutation_survivors: none` claims were re-verified against the current post-0053 code (which
      is byte-identical to its commit under test), not inherited; genuinely-closed properties are
      recorded as **present**, not re-reported as defects; new violations not named by prior passes
      were sought.
- [ ] The report contains a co-equal, load-bearing audit of the **process-fix itself**: whether the
      fail-closed taxonomy recomputes vs. validates, whether its wording guard is a closed grammar,
      whether its own correctness is mutation-covered, whether a forcing function precedes
      merge/archival, and the result of applying the 0053 taxonomy adversarially to the 0053
      acceptance.
- [ ] Every code finding names controlling invariants and both a code home and a `docs/**` home,
      in authority order, weakening no upstream tier.
- [ ] The F5-04 governance closure is re-scrutinized (0-approval self-merge / implementer-as-acceptor
      / ruleset-detail endpoint), and the `food_source` family is re-verified with no equivalence
      claim made without a semantic argument.
- [ ] Recommendations extend existing machinery (`.cargo/mutants.toml`, `anti_regression_guards.rs`,
      negative-fixture pattern, `generative_lock.rs`, `ci_workflow_guards.rs`,
      `acceptance_status_manifest.rs` / `acceptance_artifact_wording.rs`, CI jobs); no new
      property-testing dependency; no backwards-compat shim.
- [ ] No archived artifact is edited; no invariant/gate/risk/glossary identifier is minted; no
      ratified doctrine wording is authored (amendment substance + home only, if any — code or
      process layer); the check that 0053's doctrine edits did not weaken any tier is stated.
- [ ] Static-survey limits are explicit; no current command, mutation count, or API transcript is
      asserted green or red; ticket/acceptance outcomes are quoted as historical claims.
- [ ] Repository evidence and external research are in separate lanes; every external claim that
      shapes a decision is cited.
- [ ] The deliverable is exactly one new markdown file named
      `0047-foundational-hardening-research-report-sixth-pass.md`; no spec number assigned.
