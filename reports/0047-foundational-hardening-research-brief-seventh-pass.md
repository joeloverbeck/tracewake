# Spec-0047 foundational-conformance hardening — seventh-pass research brief

**You are ChatGPT-Pro Session 2.** This document is your complete, final assignment. It is
self-contained: you see only this prompt plus one uploaded manifest. Produce the deliverable in
§7 directly. **Do not interview, do not ask clarifying questions** — the requirements below are
settled. If a genuine contradiction makes a requirement impossible, state it inside the
deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-28_2720167.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first playability).
Docs are layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` →
`4-specs`; **earlier tiers govern later ones**. Fetch every file from commit
`2720167a0d1a60ac809ae1c670539a1846df031d` — the manifest reflects exactly that tree. The
manifest is a path inventory only; it is not file-content evidence. Branch names,
default-branch lookups, connector namespace labels, repository metadata, and code-search
snippets are not proof of target-commit content; fetch by exact-commit file path.

### This is the seventh audit pass on one recurring seam

The "spec-0047 surface" is the loaded-world / time-control / TUI-authority seam introduced by
`archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`
(TUI-authoritative world advance, duration completion, no-human progression, actor-known
interval summaries). It has now been audited and remediated **six** times in a row, and **each
remediation line that claimed closure was followed by an audit that still found critical
foundational violations.** You are the seventh pass.

The lineage (name each as a delta; do not re-commission completed work):

- **First pass** → `reports/0047-foundational-hardening-research-report.md` → implemented as
  spec `0048` (`archive/specs/0048_FOUNDATIONAL_CONFORMANCE_HARDENING_…`).
- **Second pass** → `reports/0047-foundational-hardening-research-report-second-pass.md` →
  implemented as spec `0050`.
- **Third pass** → `reports/0047-foundational-hardening-research-report-third-pass.md` →
  implemented as spec `0051`.
- **Fourth pass** → `reports/0047-foundational-hardening-research-report-fourth-pass.md` →
  implemented as spec `0052`.
- **Fifth pass** → `reports/0047-foundational-hardening-research-report-fifth-pass.md` →
  implemented as spec `0053`.
- **Sixth pass** (your immediate predecessor) →
  `reports/0047-foundational-hardening-research-report-sixth-pass.md` → implemented as spec
  **`0054`**
  (`archive/specs/0054_FOUNDATIONAL_CONFORMANCE_SIXTH_HARDENING_RESEALED_BOOTSTRAP_SEALED_WAIT_RECEIPT_NON_INDUCIBLE_DEBUG_AUTHORITY_INDEPENDENT_ACCEPTANCE_AND_FAIL_CLOSED_TAXONOMY_HARDENING_SPEC.md`).

The **sixth-pass report is your richest seed** — read it in full. Its seven findings
(**F6-01 … F6-07**) are the specification spec 0054 was built to satisfy; its recurrence analysis
diagnosed *why the seam keeps reopening*. Three of its findings were live code-authority
violations (F6-01 forgeable bootstrap, F6-02 debug-grade one-tick wait receipt, F6-03 inducible
debug authority); the rest were a process-integrity gap in the acceptance taxonomy (F6-04), a
governance gap (F6-05), a standing-gate gap (F6-06), and a food-source residual (F6-07). Your job
is to determine whether spec 0054 actually closed the authority class — or whether the seam has
reopened a seventh time.

### Source-equivalence posture (re-audit of the literal 0054-remediated code, with one governance-layer delta)

Spec 0054's exact implementation commit under test is `24a458243b2d8bcc08c833824cc75cec1c904f42`
(its standing mutation campaign ran at `6d7009f61e3f7d55f81da3be3297160c6f2fb402`), an ancestor of
your fetch baseline `2720167`. A verified `git diff` from the 0054 commit-under-test `24a45824` to
the baseline `2720167` shows that **every file under `crates/`, `.cargo/`, and `tests/` is
byte-identical** between the two commits. The **only** in-scope changes after `24a45824` are:

- `.github/workflows/ci.yml` (modified),
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modified),
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modified),

plus the archival of the 0054 spec/report/tickets and the creation+archival of spec **`0055`** and
its ledger row. All three modified files were changed by spec
`0055_SOLO_MAINTAINER_ACCEPTANCE_GOVERNANCE_AMENDMENT_PROPOSAL_SPEC.md` (see below).

Two consequences, and you must hold both:

1. **The simulation/runtime code surface is source-equivalent to 0054's accepted code.** There is
   no code drift to reconcile — the 0054 acceptance's per-finding closure table (F6-01 … F6-07),
   its `tracewake-acceptance-status` block, and its `mutation_survivors: none` claim map onto the
   code at `2720167` line-for-line. Treat that as an *advantage and a trap*, not a continuation:
   - *Advantage*: `archive/reports/0054_foundational_conformance_sixth_hardening_acceptance.md`
     describes the exact source you are auditing.
   - *Trap*: **this is precisely the recurrence you are auditing.** Seven prior acceptance
     artifacts (0047, 0048, 0050, 0051, 0052, 0053, and now 0054) rendered "pass" / "scoped pass"
     over code a later pass found non-conformant. So you must **re-derive every property from the
     post-0054 code itself** and treat the 0054 acceptance's verdicts — every `closed` row, the
     `overall_result: pass`, and the `mutation_survivors: none` claim — as **unverified claims**,
     not established facts. Where 0054 genuinely closed a property, record it as **present**; where
     its "pass" does not bear out against the code, report the live defect; and remain open to
     violations none of the seven passes named.
2. **The governance + acceptance-doctrine layer is the one thing that changed since 0054.** Spec
   0055 re-specified it (see the next section). Where the sixth pass's F6-05 governance finding,
   the `ci.yml` governance audit, and the `docs/1-architecture/13` / `docs/2-execution/10`
   acceptance doctrine are concerned, **measure against the current `2720167` state, not against
   0054's commit-under-test** — those files are the live delta.

The 0054 and 0055 archived artifacts are immutable historical evidence: you neither edit them nor
treat their "pass" / "completed" rows as proof of current conformance.

### Spec 0055 already re-specified the governance dimension — and you must take that decision as settled

After spec 0054 closed its F6-05 governance finding by requiring **one** approving review on
`main` (a second-human bar), the maintainer — who operates this repository **solo** — could not
satisfy that bar (GitHub forbids a PR author from approving their own PR). Spec
`0055_SOLO_MAINTAINER_ACCEPTANCE_GOVERNANCE_AMENDMENT_PROPOSAL_SPEC.md` therefore re-specified the
*standing* independent-acceptance posture: it defines an explicit **solo-maintainer acceptance
mode** in `docs/1-architecture/13` and `docs/2-execution/10`, substitutes a named
**compensating-control set** (full required-status-check barrier green, active enforcement, no
bypass actors, `current_user_can_bypass: never`, non-fast-forward + deletion protection, strict
required status checks) for the human approver on *routine* merges, relaxes the live ruleset
`main-standing-conformance-barrier` (`18200914`) `required_approving_review_count` from `1` to `0`,
and amends the `ci.yml` governance audit to accept approval-count `0` only when the ruleset proves
that control set. It explicitly preserves behavioral-evidence independence (self-authored-only
mutation / typed-path-under-test evidence remains invalid) and keeps the second-human bar for
multi-maintainer operation and (as a recommended-but-open boundary) for foundational-conformance
acceptance artifacts.

**This is a deliberate, owner-ratified decision, and it bounds your scope. The
governance-independence *mechanism* — whether 0-approval self-merge is acceptable for a solo repo,
the ruleset configuration, and the completeness of the `ci.yml` governance-audit predicate — is OUT
of scope for this pass.** Do **not** re-litigate the sixth pass's F6-05 "0-approval is governance
vacuity" argument; the maintainer has settled the operational posture. You take
`solo-maintainer-compensating-control` as the accepted independent-acceptance posture and audit only
whether the machinery *correctly enforces the posture the doctrine now defines* (§3.2), and whether
0055's doctrine edits weakened any tier (§3.4).

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
  citations. For the §3.4 solo-maintainer constitutional question, this is the tier-fit reference:
  is there (or should there be) any constitutional statement about independent acceptance.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **primary.** Event-sourced
  causality + replay/restore contract the runtime and scheduler must satisfy.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **primary.** Waiting as one
  input slot in which other loaded actors / world processes / due consequences advance; embodied
  vs. debug surface boundary.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **primary.**
  The cognition-transaction and truth-firewall doctrine governing the actor decision transaction
  and interval-summary leakage.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **primary for §3.4.** The
  acceptance-gate doctrine at the constitutional tier; read it to run the tier-fit test on whether
  "independent acceptance for a solo-operated project" belongs in the constitution at all.
- `docs/0-foundation/00,01,04,05,06,07,09,10,11,13` — **boundary-awareness.** Read to run the
  tier-fit test for any amendment proposal and to confirm what is out of the spec-0047 surface.

**Tier 1 — architecture:**

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **primary.** The conformance
  index; re-verify the loaded-world rows against the current production path.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **primary.**
  Replay/save/restore contract behind the replay-authority findings.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **primary.**
  Firewall/provenance contract behind the sealed-product and interval-leak findings (F6-02).
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` —
  **primary.** Pipeline + one core-owned world-step coordinator; the validated-bootstrap boundary
  (F6-01).
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` —
  **primary.** Canonical cognition transaction the scheduler must consume.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` —
  **primary.** Read-only client boundary; TUI must not own/mutate aggregates or construct
  embodied/debug products directly; exact ticks/frontiers/replay detail/due queues belong to
  debug/operator surfaces (F6-02, F6-03).
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` —
  **primary, AND a live 0055-changed file.** Evidence-honesty / no-decorative-locks contract and
  the architecture-tier acceptance-artifact contract that the acceptance taxonomy operationalizes
  (F6-04). This file was **amended by spec 0055** to define the solo-maintainer acceptance mode and
  its compensating-control set. Two obligations: (a) re-verify the fail-closed-acceptance doctrine
  the taxonomy enforces is sound; (b) per §3.4, **re-verify that 0055's edits did not *weaken* this
  tier** — that they preserved "required governance control … open, pending, unbounded, or merely
  historical is invalid pass," preserved behavioral-evidence independence, and scoped the
  solo-maintainer exception to the human-approval dimension only.
- `docs/1-architecture/01,06,07,08,09,11,12,14` — **boundary-awareness.**

**Tier 2 — execution:**

- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **primary.** The
  certification ladder; read it to place any recommended standing gate or process mechanism
  correctly relative to the existing gates **without minting a new gate code.**
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` —
  **primary.** Due-work belongs to the core world-step boundary; no parallel possessed-actor path;
  no caller-injected actor lists; bootstrap unforgeability obligation (F6-01).
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — **primary.**
  No-human progression and held-equal possession parity.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **primary.** TUI
  stores/renders the core interval product read-only; embodied/debug split; token-gated debug
  (F6-02, F6-03).
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **primary, AND
  a live 0055-changed file.** Evidence-honesty rule, missed-mutant/timeout discipline,
  real-public-boundary testing, merge-blocking-red / pending-is-not-a-pass, and the **fail-closed
  acceptance result taxonomy** (F6-04 doctrinal home). This file was **amended by spec 0055** to add
  the `solo-maintainer-compensating-control` independent-acceptance posture and the fail-closed
  solo-mode exception. Re-verify the taxonomy rule against the code that implements it, and per
  §3.4 confirm the solo-mode exception preserved the self-authored-only behavioral-evidence
  rejection unchanged.
- `docs/2-execution/00,01,02,04,08,09,11,12,13` — **boundary-awareness.**

**Tier 3 — reference:**

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **primary.** Reviewer pointers
  the report's doc-home recommendations update.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — **primary.** The R-27/R-28/R-29 evidence/status
  rows (mint no new risk ID). `02_GLOSSARY.md` — **boundary-awareness.**

**Tier 4 — specs, plus reports / archive (history & lineage; not live authority):**

- `docs/4-specs/SPEC_LEDGER.md` — **primary.** Source/navigation discipline; the
  0047/0048/0050/0051/0052/0053/0054/0055 archived rows, the `0049MUTWIT` ticket-only record, and
  the "Next known execution move" section. Confirm any new remediation spec routes through the
  normal ledger process; mint nothing. Note especially the 0054 row's lineage note that the
  one-approving-review clause is now commit-scoped history (0055 relaxed it to `0`).
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — **primary.** The acceptance-artifact
  template the fail-closed taxonomy enforces (the `tracewake-acceptance-status` block shape, the
  result-computation contract, the forbidden-wording rules). Central to the F6-04 process audit:
  does the template/parser leave a hole through which a self-scoped pass can still pass, now that
  the solo-maintainer posture is an accepted value?
- `reports/0047-foundational-hardening-research-report-sixth-pass.md` — **primary (richest
  seed).** Your predecessor's full findings (F6-01 … F6-07), recurrence analysis, and the
  remediation recommendations spec 0054 implemented. **Non-carry-forward**: its findings are the
  *pre-remediation baseline to re-verify*, not facts to inherit.
- `reports/0047-foundational-hardening-research-brief-sixth-pass.md` — **boundary-awareness.**
  The locked scope/lineage the sixth pass worked to; confirms conventions and the canonical
  report shape.
- `archive/specs/0054_…_HARDENING_SPEC.md` — **primary.** The promises the sixth remediation line
  made; the spec whose claims you re-verify against the (source-equivalent) code.
- `archive/reports/0054_foundational_conformance_sixth_hardening_acceptance.md` — **primary.**
  What 0054 *claims* it closed: the `tracewake-acceptance-status` block, the per-finding closure
  table (F6-01 … F6-07), the mutation command ledger and full standing campaign disposition
  (`3445` selected, `2679` caught, `766` unviable, `0` missed, `0` timeout), the ruleset/governance
  transcript (as it stood at 0054 closeout, before 0055), and the evidence-item ledger. Your
  concrete re-verification seed; describes the exact source you audit (source-equivalence posture,
  §1).
- `archive/specs/0055_SOLO_MAINTAINER_ACCEPTANCE_GOVERNANCE_AMENDMENT_PROPOSAL_SPEC.md` —
  **primary.** The governance re-specification that bounds your scope (§1, §3.3) and whose doctrine
  edits you check for tier-weakening (§3.4). Read its Problem Statement, Approach,
  Authority-table reconciliation, Deliverables, Invariants Alignment, and Outcome in full.
- `reports/0047-foundational-hardening-research-report-fifth-pass.md`,
  `archive/reports/0053_foundational_conformance_fifth_hardening_acceptance.md`,
  `archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md`,
  `archive/reports/0051_foundational_conformance_third_hardening_acceptance.md`,
  `archive/reports/0050_foundational_conformance_second_hardening_acceptance.md`,
  `reports/0048_foundational_conformance_hardening_acceptance.md`,
  `archive/reports/0047_tui_authoritative_world_advance_acceptance.md` —
  **boundary-awareness / recurrence corpus.** Prior remediation epochs and the original
  feature-surface boundary; historical evidence and recurrence raw material for the "is the process
  fixed" question.
- `archive/specs/0053_…SPEC.md`, `archive/specs/0050_…SPEC.md`, `archive/specs/0051_…SPEC.md`,
  `archive/specs/0052_…SPEC.md` — **boundary-awareness.** The promises the prior remediation lines
  made.
- `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt` — **primary.** The existing standing
  mutation perimeter config and the survivor/baseline-miss registry any anti-regression
  recommendation must extend, not duplicate.

### Code seams to inspect (read directly at the baseline; do not expect them pasted here)

Audit the post-0054 source at `2720167` (byte-identical to the 0054 commit-under-test `24a45824`
under `crates/`, `.cargo/`, `tests/`). The **bolded entries are the surfaces 0054 changed to close
F6-01 … F6-07**, which carry the highest re-audit priority because the recurrence is precisely that
a closure claim was later falsified:

- **`crates/tracewake-core/src/state.rs`** — `PhysicalState` / `AgentState` seed-part
  constructors; re-verify whether raw "validated" seed-part construction
  (`from_validated_seed_parts`) remains publicly reachable to an external crate (the F6-01 attack
  surface the sixth pass found live).
- **`crates/tracewake-core/src/runtime/{mod,session,command,receipt}.rs`** — `LoadedWorldRuntime`,
  the `ValidatedLoadedWorldBootstrap` constructor and `from_bootstrap` (F6-01), the closed
  `RuntimeCommand` dispatch and `bind_debug_controller` authority requirement (F6-03), and the
  **`OneTickRuntimeReceipt`** / sealed wait-receipt product that replaced the public
  `OneTickAdvanced(WorldAdvanceResult)` (F6-02). Re-verify the wait path no longer exposes
  `WorldAdvanceResult` internals on the public boundary.
- **`crates/tracewake-core/src/debug_capability.rs`** — the `DebugSessionAuthority` token; re-verify
  it is runtime-minted and **non-inducible** by an unprivileged public command (F6-03 closure
  claim).
- `crates/tracewake-core/src/scheduler.rs` — `transact_world_one_tick`, due-actor / due-process
  discovery, the per-tick actor disposition census, replay restore, declared-process authority.
- `crates/tracewake-core/src/events/{envelope,apply,log}.rs` — declared-process event class &
  application; duplicate-`EventId` fail-closed.
- `crates/tracewake-core/src/agent/{transaction,trace,perception}.rs` — actor decision
  transaction consumption; `StuckDiagnostic`; perception write path.
- `crates/tracewake-core/src/epistemics/projection.rs`, `crates/tracewake-core/src/projections.rs`
  — `insert_observation`, interval-delta/salience, the **`food_source_fact_supersedes`** semantics
  (F6-07).
- `crates/tracewake-core/src/view_models.rs` — `EmbodiedViewModel`,
  `TypedActorKnownIntervalSummary` (field/constructor/accessor sealing).
- `crates/tracewake-core/src/replay/{temporal,rebuild,report}.rs`,
  `crates/tracewake-core/src/actions/pipeline.rs`.
- **`crates/tracewake-content/src/{load,schema}.rs`** — loaded-world handoff into the runtime
  constructor (`into_runtime_bootstrap`); the validated path that must be the *only* way to mint a
  bootstrap (F6-01).
- `crates/tracewake-tui/src/{app,run,transcript,render,input,launch,debug_panels,lib,main}.rs` —
  `TuiApp::from_golden`, semantic-action submission, advance/continue, command-loop dispatch,
  debug-mode gating (F6-03 operator entrypoint); embodied rendering leakage and any residual
  client-side interval construction.
- **The fail-closed acceptance taxonomy machinery (the in-scope process axis):**
  `crates/tracewake-core/tests/acceptance_status_manifest.rs`,
  `crates/tracewake-core/tests/acceptance_artifact_wording.rs`,
  `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`, and
  `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`. Audit whether the parser actually
  *recomputes* a fail-closed result rather than reading a self-declared `overall_result`; whether
  it now correctly handles the `solo-maintainer-compensating-control` governance posture **as the
  doctrine defines it** (not whether that posture is itself acceptable — that is settled); whether
  the wording guard's rejection set is a closed grammar or a paraphrase-bypassable denylist; whether
  the machinery's own correctness is mutation-covered; and whether anything **forces** an acceptance
  artifact through the parser before archival/merge. The 0054 acceptance itself is the first
  acceptance produced under the 0054-evolved taxonomy — apply the taxonomy's own rules adversarially
  to the 0054 artifact and report any place it should have failed but did not.
- The **new F6-closure negative fixtures** (re-verify each fixture attacks the *live* public API,
  not a stale/obsolete name, and fails for the authority reason):
  `tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/`,
  `…/external_crate_cannot_read_one_tick_wait_receipt_internals/`,
  `…/external_crate_cannot_induce_debug_authority_via_public_bind/`,
  `…/external_crate_cannot_submit_debug_command_without_token/`,
  `…/external_crate_cannot_construct_actor_interval_summary/`,
  `…/external_crate_cannot_mutate_embodied_temporal_fields/`,
  `…/external_crate_cannot_convert_debug_report_to_interval_summary/`, and the broader
  `tests/negative-fixtures/external_crate_cannot_*` corpus.
- The rest of the test / CI infrastructure:
  `crates/tracewake-core/tests/*` (esp. `world_step_coordinator.rs`, `negative_fixture_runner.rs`,
  `anti_regression_guards.rs`, `generative_lock.rs`, `ci_workflow_guards.rs`,
  `replay_temporal_frontier.rs`, `holder_known_interval_projection.rs`,
  `salient_stop_actor_known.rs`, `reservation_body_exclusive_census.rs`,
  `mutation_completion_merge.rs`, `food_source_projection.rs`), `crates/tracewake-tui/tests/*`
  (esp. `tui_acceptance.rs` — the F6-07 public food-source witness, `tui_seam_conformance.rs`,
  `command_loop_session.rs`, `playable_capability_parity.rs`, `embodied_flow.rs`,
  `parity_adversarial.rs`, `transcript_snapshot.rs`), and `.github/workflows/ci.yml` (current,
  post-0055 — read for topology context, but the governance-audit *predicate completeness* is out
  of scope per §3.3).

---

## 3. Settled intentions (these are decisions, not open questions)

1. **Full independent re-audit of the entire spec-0047 code surface at `2720167`.** Re-derive
   conformance for the whole loaded-world / time-control / TUI-authority surface from the post-0054
   code. Treat the 0054 acceptance's `closed` rows, its `overall_result: pass`, and the
   `mutation_survivors: none` claim as **unverified claims**, not facts. Rationale: six consecutive
   remediation lines each declared closure and were each followed by a pass that found critical
   violations; a delta-only audit would inherit 0054's blind spots. Because the code surface is
   byte-identical to the 0054 commit under test (§1), there is no code drift to reconcile — the
   entire re-audit is "does the code the 0054 acceptance blessed actually satisfy the foundations."
   Record genuinely-closed properties (F6-01 re-sealed bootstrap, F6-02 sealed `OneTickRuntimeReceipt`,
   F6-03 non-inducible debug authority, F6-07 public food-source witness) as **present** where they
   hold; report live defects where a "pass" does not hold; and remain open to violations none of the
   seven passes named.

2. **The fail-closed acceptance taxonomy (the process-fix) is in scope as a load-bearing axis —
   but bounded to "does it enforce the doctrine," not "is the doctrine right."** Determine, from the
   code:
   - whether `acceptance_status_manifest.rs` genuinely **recomputes** a fail-closed overall result
     from the constituent statuses / mutation disposition / governance state, or merely validates a
     self-declared `overall_result` for internal consistency (a self-consistent block can still be a
     self-authored pass);
   - whether the manifest now correctly computes the **`solo-maintainer-compensating-control`**
     governance posture as the 0055 doctrine defines it (an accepted independent-acceptance posture
     *only* when the compensating-control set is proven), and whether a manifest could still launder
     a non-conforming governance state through it — **without re-arguing whether the solo-maintainer
     posture is itself legitimate (that is settled — §3.3)**;
   - whether `acceptance_artifact_wording.rs`'s rejection set is **a closed grammar or a
     paraphrase-bypassable denylist**;
   - whether the taxonomy machinery's **own correctness is independently proven** (mutation coverage
     of the parser/guard, adversarial/synthetic manifests, negative cases) rather than asserted — a
     guard whose own mutants survive is decorative;
   - whether anything **forces** an acceptance artifact through the parser/guard before
     archival/merge (a forcing function in CI / the ruleset), or whether running them is reviewer
     discipline;
   - and whether the **0054 acceptance itself would survive a genuinely adversarial application of
     its own (0054-evolved) taxonomy** — apply the parser/guard's rules to the 0054 artifact's
     wording and status block and report any place it should have failed but did not. Name any
     residual structural laundering vector (self-authored status block; guard-scored-by-its-own-author;
     unforced parser) and specify the durable forcing function that closes it, **without minting a
     new gate code, invariant, risk ID, or glossary term**, routing any doctrine strengthening
     through §3.5 (substance + home only).

3. **Governance-independence *mechanism* is OUT of scope — settled by spec 0055.** Do not
   re-litigate the sixth pass's F6-05 "0-approval self-merge is governance vacuity" argument, the
   ruleset configuration, or the completeness of the `ci.yml` governance-audit predicate. The
   maintainer operates this repository solo and has deliberately ratified the
   `solo-maintainer-compensating-control` posture via spec 0055. Take that posture as the accepted
   independent-acceptance baseline. (The only governance-adjacent work in scope is the
   *taxonomy-enforces-the-posture* check in §3.2 and the *no-weakening* check in §3.4.) If you
   believe the settled posture nonetheless creates a *foundational* problem, the only admissible
   route is the §3.4 conditional Tier-0 determination — not a re-opening of the operational
   mechanism.

4. **Conditional foundation/doctrine-amendment branch, re-decided independently, with an explicit
   solo-maintainer-acceptance focus.** Determine, with evidence, whether any tier
   (`0-foundation` … `3-reference`) doctrine needs amendment for this surface or for the
   acceptance/verification process. Two specific obligations:
   - **No-weakening check (mandatory):** re-verify that 0055's edits to `docs/1-architecture/13` and
     `docs/2-execution/10` did **not** weaken any tier — that they preserved the invalid-pass
     conditions, preserved self-authored-only behavioral-evidence rejection (mutation /
     typed-path-under-test), and scoped the solo-maintainer exception to the *human-approval*
     dimension of independence only. If 0055's edits silently weakened a protection (e.g. made a
     survivor disposition or a self-authored status block bless a pass), report it as a doctrine
     defect with substance + home.
   - **Constitutional re-examination (solo-maintainer focus):** the prior six passes and spec 0055
     all concluded **no Tier-0 amendment is warranted** (no `INV-NNN` mandates a second-human
     approving review). **Reach your own verdict** — do not inherit theirs. Specifically weigh
     whether the constitution should *acknowledge or define* what "independent acceptance" means for
     a solo-operated project (so that the below-foundation solo-maintainer mode rests on a
     constitutional footing rather than only architecture/execution doctrine), or whether the
     foundation is correct to stay silent and leave acceptance governance to the lower tiers. If —
     and only if — you find a genuine gap or contradiction that blocks remediation or perpetuates
     the cycle, recommend the amendment as **substance + home** (what doctrine the target doc must
     own, in your own prose at that tier's altitude, and which file/section it lands in) — **never**
     final paste-ready wording and **never** an invented identifier (`INV-###`, gate code, risk ID,
     glossary term).

5. **The enforced standing gate (code-surface anti-regression) remains required and re-verified.**
   Independently of the governance mechanism (which is settled), re-evaluate the enforced standing
   gate for the *code surface*: the `public-boundary-conformance` lane, the
   `full-surface-mutation-trigger` / in-diff mutation / `mutants-lock-layer-reconcile` jobs, the
   compile-time unrepresentability layer on the production symbols, and the negative-fixture corpus.
   Specify what still must change to make "read-only client," "production reachability," "sealed
   bootstrap/receipt/debug authority," and "green surface mutation perimeter" *enforced* rather than
   programmer discipline. Demote source-text guards explicitly to topology alarms; keep compile-time
   unrepresentability (private fields / crate-private constructors / unforgeable authority tokens on
   the real production symbols) as the load-bearing layer.

6. **The `food_source_fact_supersedes` residual (F6-07) is re-verified, not inherited as closed.**
   The 0054 acceptance claims a public TUI actor-known witness (`tui_acceptance.rs`) plus zero
   survivors via the standing campaign. Re-verify from the code that the witness exercises the
   replacement semantics through public actor-known / embodied behavior (not a private-function-only
   assertion) and that the forcing function prevents the routed-forward pattern from silently
   recurring. Make no equivalence claim about any mutant without a defensible semantic argument;
   "the suite killed it historically" is a static-survey historical claim, not a current result
   (§3.7).

7. **Static-survey-only.** You cannot run `cargo fmt/clippy/build/test`, replay/golden lanes, CI,
   `cargo-mutants`, or GitHub API commands. Every statement about current test strength, mutation
   result, governance enforcement, or pass/fail is a **preliminary static judgment about source
   shape, API authority, data flow, witness intent, and CI/ruleset configuration as written** —
   explicitly non-certifying. Authoritative pass/fail belongs to the implementing session that
   executes the gates from a clean baseline. Quote any command outcome, mutation count, or API
   transcript from a ticket or acceptance artifact as a **historical claim by that artifact**, never
   as a current result.

8. **Extend existing machinery; introduce no new test-framework dependency.** `.cargo/mutants.toml`,
   `anti_regression_guards.rs`, the external-crate negative-fixture pattern +
   `negative_fixture_runner.rs`, `generative_lock.rs`, `ci_workflow_guards.rs`, the
   `acceptance_status_manifest.rs` / `acceptance_artifact_wording.rs` taxonomy machinery, the
   coordinator/replay/interval/salience/reservation/parity suites, and the CI jobs already exist.
   Recommendations extend them. Do **not** recommend adding `proptest`, `quickcheck`, or another
   property-testing framework. No backwards-compatibility shims or alias paths in any recommended
   change.

---

## 4. The task

Determine whether the code implementing the spec-0047 surface — as it stands at commit `2720167`,
byte-identical under `crates/`/`.cargo/`/`tests/` to the spec-0054 commit under test — **still
violates the foundations**, and if so, specify how to remediate it, harden it, and **make future
regression as close to impossible as feasible**. Re-verify every load-bearing property of the
surface from the current code under the repository's authority order, treating 0054's `closed`/`pass`
rows as unverified claims; render an explicit conformance verdict; for each violation, name the
controlling invariants, the current code state, the conformance verdict, the code + `docs/**`
remediation home, the strongest practical anti-regression guard, and an evidence-honesty check. As a
secondary, load-bearing axis, audit the **fail-closed acceptance taxonomy** (whether it recomputes
vs. validates, is a closed grammar, is mutation-covered, is forced before merge, and would survive
adversarial application of its own rules to the 0054 acceptance) — taking the spec-0055
solo-maintainer governance decision as **settled doctrine the taxonomy must enforce, not
re-litigate**. Perform the mandatory no-weakening check on 0055's architecture-13 / execution-10
edits, and reach an independent verdict on the conditional foundation-amendment branch with explicit
attention to whether the constitution should acknowledge independent acceptance for a solo-operated
project. This is a cross-cutting **hardening / anti-contamination re-audit** (seventh pass) with a
secondary **foundational/doc-overhaul** branch (conditional amendment determination per §3.4).
Honor the user's standing instruction: **if you find no foundational violations, state explicitly
whether you believe another iteration will be needed and why.** Claude Code will later analyze your
report and produce the numbered remediation spec; **you do not write the spec.**

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow every code seam,
test, fixture, and CI definition that bears on the surface, and read the prior acceptance artifacts
whose verdict wording feeds the recurrence analysis. Research online as deeply as needed — similar
implementations, research papers, and prior art — wherever it sharpens the remediation, the
regression barrier, or the process-fix audit. The prior passes opened a useful external lane you may
extend: Rust visibility/privacy and the `non_exhaustive` limitation; Schneider's state-machine
replication; Temporal's replay-from-history model; Sabelfeld & Myers on language-based
information-flow (access control ≠ information-flow control); cargo-mutants guidance on missed
mutants / timeouts and the mutation-testing survey on undecidable equivalence. For the process-fix
and the solo-maintainer-acceptance constitutional question specifically, prior art on **fail-closed
verification, goal displacement (Goodhart's law), specification-gaming, and compensating controls /
separation-of-duties in single-operator or small-team settings (e.g. compensating-control regimes in
audit/compliance frameworks)** is on point — but recall that the *operational* solo-maintainer
governance decision is settled (§3.3); cite such prior art only where it informs the §3.4
constitutional verdict or the taxonomy-enforcement audit, not to re-argue the ruleset choice. Keep
the external-research lane strictly separate from repository evidence, and cite every external source
that shapes a decision.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every product-behavior
  decision must satisfy it; a genuine divergence requires amending an invariant first, never
  designing against it silently (and amendment substance only — §3.4).
- **Authority order is absolute:** if execution conflicts with architecture or foundation,
  execution is wrong; if the implementation is more convenient than the accepted gates, the
  implementation is wrong. Never weaken an upstream tier to fit downstream code or to bless a
  convenient acceptance.
- **No simulation fact may be born from prose.** Preserve event-sourced causality, subjective
  epistemics, ordinary agents, possession parity, fallible institutions, validation/replay, and the
  actor-known truth firewall.
- No backwards-compatibility shims or alias paths in new work.
- Do not edit any archived spec, ticket, acceptance artifact, or passed certification; live-doc work
  is status/navigation/conformance-evidence only, and only after the code and executable witnesses
  exist. Mint no new invariant, gate, risk ID, or glossary term.
- Source-text guards are topology alarms only — never sole proof of atomicity, replay continuation,
  process semantics, one-opportunity-per-actor, information-flow noninterference, production
  reachability, mutation sensitivity, or acceptance honesty. Those require type boundaries,
  executable behavior, and (for the process layer) enforcement the implementing actor cannot
  self-bypass.
- **Scope guard:** the solo-maintainer governance *mechanism* (0-approval self-merge, ruleset
  config, governance-audit predicate completeness) is **settled by spec 0055 and out of scope.** Do
  not recommend changes to it. The only governance-adjacent outputs permitted are (a) the
  taxonomy-enforces-the-posture audit (§3.2), (b) the no-weakening check on 0055's doctrine edits
  (§3.4), and (c) the conditional Tier-0 constitutional verdict (§3.4).

---

## 7. Deliverable specification

Produce **exactly one** downloadable markdown document, **new** (not a replacement):

```
0047-foundational-hardening-research-report-seventh-pass.md
```

This continues the lineage filenames `0047-foundational-hardening-research-report.md`,
`-second-pass.md`, `-third-pass.md`, `-fourth-pass.md`, `-fifth-pass.md`, `-sixth-pass.md`. It is an
**analysis / recommendation report, not a numbered spec**: Claude Code derives the numbered
remediation spec from it afterward (it would stage next in the `00xx` sequence after `0055`), so the
numbering / ledger / epoch rules do **not** apply to you and you must not assign a spec number or
write a `specs/`-style artifact.

**Production mode — always produce, with the verdict as a section (mode i).** Produce the report
unconditionally. Open it with an explicit, evidence-based **verdict**: is the spec-0047 code surface
foundationally conformant at `2720167`; is the fail-closed acceptance taxonomy now sound (does it
recompute fail-closed, is it a closed grammar, is it mutation-covered, is it forced before merge,
and does it correctly enforce the 0055 solo-maintainer posture); did 0055's doctrine edits weaken any
tier; and is any Tier-0 (or other) amendment warranted, including the solo-maintainer-acceptance
constitutional question. **The report's value survives any outcome** — even a clean code verdict
locks the now-correct properties, audits the taxonomy, performs the no-weakening check, and answers
the recurrence question. Per the user's standing instruction, if the verdict is clean, **state
explicitly whether you believe another iteration will be needed, with reasons.** Given the six-pass
history, a *fully* clean verdict is unlikely, but render whatever the current code and process
actually support; do not manufacture findings, and record genuinely-fixed properties as **present**.

Reuse the canonical shape the prior reports established, scaled to what you find:

1. **Verdict** — code conformant / not conformant, with decisive reasons; the taxonomy-soundness
   verdict; the no-weakening verdict on 0055's doctrine edits; the explicit higher-tier amendment
   verdict (§3.4, including the solo-maintainer-acceptance constitutional question); and the
   "another iteration needed?" judgment.
2. **Disposition table** — one row per finding → primary code/doc/process target → classification
   (violation / vacuity gap / hardening gap / mutation-survivor disposition / evidence-honesty gap /
   process-integrity gap / doctrine-weakening gap) → one-line basis citing invariants or governing
   doctrine.
3. **Method & provenance ledger** — authority order applied; the source-equivalence posture (the
   0054-remediated `crates/`/`.cargo/`/`tests/` source is byte-identical to baseline `2720167`, so
   the 0054 acceptance describes the exact audited code, and its `closed`/`pass` rows are
   re-verified, not inherited; the governance/doctrine layer is the post-0054 0055 delta);
   exact-commit acquisition discipline (fetch every file by full exact-commit path from `2720167`;
   manifest is inventory only); and the static-survey limitation.
4. **Re-verified present properties from 0054** — the genuinely-closed properties (re-sealed
   `ValidatedLoadedWorldBootstrap`, sealed `OneTickRuntimeReceipt`, non-inducible
   `DebugSessionAuthority`, the public food-source witness, the taxonomy machinery as far as it
   genuinely holds) recorded as **present** so the pass does not erase real 0054 progress.
5. **Per-finding sections (code surface)** — for each finding: foundational driver (named
   invariants + governing architecture/execution doctrine) → current `2720167` code state (cite the
   real post-0054 symbols/paths) → conformance verdict → required remediation (code home + `docs/**`
   home, honoring authority order) → strongest practical anti-regression guard (compile-time /
   behavior / differential / mutation) → evidence-honesty check.
6. **Residual disposition** — the `food_source_fact_supersedes` family (re-verified forcing
   function, no unjustified equivalence claims) per §3.6.
7. **Structural anti-regression / enforced standing gate (code surface)** — why the seam did or did
   not reopen despite the 0054 enforced gate; what still must change to make the code-surface gate
   enforced rather than declarative (CI conformance lane through the public boundary; green standing
   mutation perimeter for the surface; compile-time unrepresentability as the load-bearing layer;
   correct placement relative to the cert ladder without minting identifiers). **Do not** recommend
   governance-mechanism changes (out of scope, §3.3).
8. **Fail-closed acceptance taxonomy audit** — does `acceptance_status_manifest.rs` recompute vs.
   validate; does it correctly enforce the `solo-maintainer-compensating-control` posture as the
   0055 doctrine defines it; is the wording guard a closed grammar or a phrase denylist; is the
   machinery's own correctness mutation-covered; is there a forcing function before merge/archival;
   the adversarial application of the 0054-evolved taxonomy to the 0054 acceptance itself; and the
   durable forcing function that makes a "pass" impossible while a flagged foundational violation
   persists, anchored in execution-10 / architecture-13 doctrine and the existing CI / mutation
   machinery — **without re-arguing the settled governance mechanism.**
9. **Foundation & documentation determination** — the no-weakening verdict on 0055's
   architecture-13 / execution-10 edits; the amendment verdict with reasoning (including the
   explicit solo-maintainer-acceptance constitutional question, §3.4); and the post-implementation
   live-doc work table (architecture conformance `00`; architecture `02/04/05/10/13`; execution
   `05/06/07/10`; reference `00`/`01` R-27/R-28/R-29 status only; ledger through the normal
   process), explicitly **after** executable closure, editing no archived material and minting no
   identifier.
10. **Recommended closure order**, **open maintainer decisions** (implementation choices inside
    settled doctrine), **self-check**, and **references** (repository evidence and external research
    kept in separate lanes).

**Locked / no-questions instruction:** Produce the deliverable directly as a downloadable markdown
document. Do not interview, do not ask clarifying questions — the requirements above are final. If a
genuine contradiction makes a requirement impossible, state it in the deliverable and proceed with
the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The verdict is explicit and evidence-based, covering foundational code conformance, the
      acceptance-taxonomy soundness, the no-weakening check on 0055's doctrine edits, the higher-tier
      amendment determination (including the solo-maintainer-acceptance constitutional question), and
      the "another iteration needed?" judgment.
- [ ] Every file used for target-state claims was fetched from the full exact commit
      `2720167a0d1a60ac809ae1c670539a1846df031d`; every §2 path was present in the uploaded
      manifest.
- [ ] The 0054 acceptance's `closed`/`pass` rows and `mutation_survivors: none` claim were
      re-verified against the current post-0054 code (which is byte-identical under
      `crates/`/`.cargo/`/`tests/` to its commit under test), not inherited; genuinely-closed
      properties are recorded as **present**, not re-reported as defects; new violations not named by
      prior passes were sought.
- [ ] The governance-independence *mechanism* (0-approval self-merge, ruleset config,
      governance-audit predicate completeness) was treated as **settled by spec 0055 and out of
      scope**; no recommendation re-litigates it. The only governance-adjacent outputs are the
      taxonomy-enforces-the-posture audit, the no-weakening check, and the conditional Tier-0
      constitutional verdict.
- [ ] The fail-closed acceptance taxonomy was audited (recompute vs. validate; closed grammar vs.
      denylist; mutation coverage of the guard itself; forcing function before merge/archival;
      adversarial application of the 0054-evolved taxonomy to the 0054 acceptance), and it was
      checked that it correctly enforces the `solo-maintainer-compensating-control` posture as the
      doctrine defines it.
- [ ] Every code finding names controlling invariants and both a code home and a `docs/**` home, in
      authority order, weakening no upstream tier.
- [ ] The `food_source` family is re-verified with no equivalence claim made without a semantic
      argument.
- [ ] Recommendations extend existing machinery (`.cargo/mutants.toml`, `anti_regression_guards.rs`,
      negative-fixture pattern, `generative_lock.rs`, `ci_workflow_guards.rs`,
      `acceptance_status_manifest.rs` / `acceptance_artifact_wording.rs`, CI jobs); no new
      property-testing dependency; no backwards-compat shim.
- [ ] No archived artifact is edited; no invariant/gate/risk/glossary identifier is minted; no
      ratified doctrine wording is authored (amendment substance + home only, if any); the
      no-weakening check on 0055's doctrine edits is stated explicitly.
- [ ] Static-survey limits are explicit; no current command, mutation count, or API transcript is
      asserted green or red; ticket/acceptance outcomes are quoted as historical claims.
- [ ] Repository evidence and external research are in separate lanes; every external claim that
      shapes a decision is cited.
- [ ] The deliverable is exactly one new markdown file named
      `0047-foundational-hardening-research-report-seventh-pass.md`; no spec number assigned.
