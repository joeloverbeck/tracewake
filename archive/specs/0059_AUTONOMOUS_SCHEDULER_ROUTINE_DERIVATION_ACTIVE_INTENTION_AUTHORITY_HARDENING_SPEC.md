# 0059 Autonomous Scheduler Routine Derivation Active-Intention Authority Hardening Spec

**Status:** COMPLETED  
**Intended repository path:** `archive/specs/0059_AUTONOMOUS_SCHEDULER_ROUTINE_DERIVATION_ACTIVE_INTENTION_AUTHORITY_HARDENING_SPEC.md`  
**Series:** embodied/autonomous routine-continuation hardening follow-up to archived specs 0057 and 0058  
**Target repository:** `joeloverbeck/tracewake`  
**Target baseline commit:** `dffeefa8e4105b7f4c6637f9bdb29dddea519a99`  
**Baseline freshness claim:** user-supplied target commit only; this spec does **not** independently verify that the commit is current `main`  
**Authority posture:** subordinate to the live doctrine tiers at the target commit; mints no invariant, gate, glossary term, or design-risk ID  
**Certification posture:** no whole-project, latest-main, P0/SPINE/EPI/ORD-LIFE/FIRST-PROOF, Phase-4, or second-proof claim  
**Archive posture:** new staged spec; archive only after acceptance

---

## 0. Status, posture, and non-certification boundaries

This is a scoped implementation hardening spec for the autonomous scheduler routine-family derivation seam. It is intentionally narrow. It does not reopen the embodied routine-continuation fix from 0058, the planner/no-human cadence, the 0047 authoritative world-tick coordinator, the severe-need-interrupt path, or the deferred scheduler-owned time-advancing follow-on routing.

The spec always has value regardless of the verdict. If the autonomous derivation is an active-intention-authority violation, this spec specifies the fix and locks. If the autonomous derivation is already coincident with active-intention authority, this spec specifies the standing metamorphic lock that prevents silent future drift.

No product-behavior decision in this spec may weaken foundation or architecture doctrine for implementation convenience. If an implementing session finds a real conflict with foundation doctrine, the implementation must stop and route the doctrine problem to the correct upstream home rather than quietly designing around it.

---

## 1. Baseline and source-discipline statement

Repository evidence in this document is limited to paths present in the uploaded manifest and fetched through exact full URLs at commit `dffeefa8e4105b7f4c6637f9bdb29dddea519a99`. The manifest is a path inventory only; it is not treated as repository file content. No repository metadata, default branch lookup, branch-name file fetch, target-repository code search, repository snippets, clone, prior chat, or connector namespace label was used to establish target-repository state.

The exact fetch form used for target-repository files was:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/<manifest path>
```

The acquired target-repository file set contains the primary doctrine, lineage, and code-seam files required by the research brief, plus boundary-awareness doctrine/spec files needed for tier-fit. The exact-fetch ledger is Appendix A.

External research is segregated in §12. It may shape test-design judgment and terminology, but it is not used to assert what exists in `joeloverbeck/tracewake` at `dffeefa8e4105b7f4c6637f9bdb29dddea519a99`.

### 1.1 Acquisition summary

```text
Requested repository: joeloverbeck/tracewake
Target commit: dffeefa8e4105b7f4c6637f9bdb29dddea519a99
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open() with full raw.githubusercontent.com exact-commit URLs
Requested file count: 77
Successfully verified file count: 77
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

---

## 2. Scope

### 2.1 In scope

This spec governs the autonomous routine-family derivation boundary, meaning:

- the scheduler-owned autonomous actor transaction paths in `crates/tracewake-core/src/scheduler.rs` — both the per-tick `DeterministicScheduler::transact_world_one_tick` "world_step" path and the no-human-day path `run_no_human_actor_decision_transaction` reached through the `#[cfg(test)]` `run_no_human_day` harness (the no-human golden-fixture acceptance driver);
- any helper that derives a `RoutineFamily` for an actor in the autonomous path, including the present helpers `routine_window_family` and `eligible_routine_execution_for_actor`, or any renamed equivalent;
- the `ActorDecisionTransactionInput::routine_window_family` seam in `crates/tracewake-core/src/agent/transaction.rs`;
- candidate-generation behavior that turns a routine-window family into a `RoutineDuty` or equivalent candidate in `crates/tracewake-core/src/agent/generation.rs` when that code is touched or implicated;
- behavioral tests, metamorphic tests, anti-regression source/behavior guards, and focused mutation over those seams.

### 2.2 Out of scope

The following are settled out of scope and must not be raised as findings under this spec:

- broad audit of whether the scheduler is a cognition authority across all derivations;
- severe-need-interrupt path, except to avoid breaking it and to record any observed dependency as routed-forward work;
- embodied routine continuation, already fixed and locked by 0058;
- planner/no-human cadence redesign;
- 0047 authoritative world-tick coordinator redesign;
- scheduler-owned time-advancing follow-on routing, where 0058's typed-stuck gateway remains the active posture;
- new invariants, gates, glossary terms, risk IDs, or certification claims.

If a real divergence is discoverable only by redesigning planner cadence or the world-tick coordinator, record it in §11 as a routed-forward open question. Do not expand this spec.

---

## 3. Determination / verdict

### 3.1 Verdict: confirmed window-keyed routine-family selector at baseline; fix plus lock

**Static verdict, not certification (corrected at reassessment against the working tree):** the named autonomous helper symbols **are present** in the scheduler at the spec's own target commit `dffeefa8e4105b7f4c6637f9bdb29dddea519a99` and remain present unchanged at the current reassessment baseline. `routine_window_family` is defined at `crates/tracewake-core/src/scheduler.rs:3240` and `eligible_routine_execution_for_actor` at `:3260`. The latter selects a routine execution by `.min_by(start_tick, execution_id)` over clock/window eligibility (`:3284-3288`) and never consults `agent_state.active_intention_by_actor` — exactly the window-keyed family-selection pattern F-0059-01 names as a violation to fix. Its result is passed into `ActorDecisionTransactionInput::routine_window_family` at `:3143`, inside `run_no_human_actor_decision_transaction`.

Two autonomous transaction paths exist and their posture differs. The per-tick `DeterministicScheduler::transact_world_one_tick` "world_step" path (`:1108`) invokes `ActorDecisionTransaction::run` with `routine_window_family: None` and is compliant as written. The window-keyed family flows only through the second path, `run_no_human_actor_decision_transaction` (`:3136`), which is reached through the `#[cfg(test)]` `run_no_human_day` harness (`:2641`) that drives the no-human golden-fixture acceptance runs in `tracewake-content` — squarely within this spec's autonomous-derivation scope. The transaction obtains the actor's active intention through `active_intention_for_actor` (which reads `agent_state.active_intention_by_actor`), but the caller-supplied window family is converted to a `routine_window_goal` candidate alongside it (`crates/tracewake-core/src/agent/transaction.rs:132`).

On this exact-code evidence the operative verdict is **fix plus lock**, not lock-only: the brief's named helper functions are real repository state, not merely a commissioning seed. The production blast radius is bounded — `GoalPriority::ActiveIntentionContinuation` (rank 3) already outranks `GoalPriority::RoutineWindowDuty` (rank 5) in `crates/tracewake-core/src/agent/candidate.rs:59`, so when an active intention exists the continuation candidate wins and the window goal is moot. The live defect is the **no-active-intention** case, where the clock/window selector can still produce a `RoutineDuty` family candidate ahead of cognition; F-0059-02 governs that case.

The fix is necessary and the lock remains mandatory. `ActorDecisionTransactionInput` exposes `routine_window_family: Option<RoutineFamily>`, candidate generation converts that optional routine-window family into a `RoutineDuty`/routine-window goal candidate, and the no-human-day path already supplies it from a clock/window-keyed selector that bypasses the active-intention chain. Even once that selector is repaired, the interface remains a live drift vector: a future scheduler, debug path, no-human path, or renamed helper could reintroduce a window-keyed family selector without touching the shared routine-continuation resolver. Therefore 0059 is a fix-plus-lock hardening/anti-regression spec: **any autonomous routine family must be derived from the actor's single authoritative active-intention chain, or produce a typed, replayable, non-override outcome.** The scheduler may order and validate opportunities; it may not choose routine family ahead of cognition.

This reassessment has already exercised the operative flip the spec was written to tolerate: the named window-keyed selector is present at the implementation baseline, so the verdict is **fix plus lock** rather than lock-only. Per the spec's design this changes nothing about its scope — F-0059-01 through F-0059-05, the tests in §7, the focused mutation in §8, and the decomposition in §10 all apply as written.

### 3.2 Required product decision

Autonomous routine-family derivation has exactly one authority source:

```text
actor_id
  -> agent_state.active_intention_by_actor[actor_id]
  -> intention record
  -> selected routine method and bound unresolved current step/execution
  -> RoutineFamily
```

Clock/window eligibility may decide that an actor is due for a transaction or that a transaction should review routine progress. It must not select `Work`, `Eat`, `Sleep`, or any other routine family in place of the active intention. Actor-known context may parameterize or validate the selected family's step after the active intention binds the family; actor-known context may not pick a tempting family ahead of the active intention.

---

## 4. Evidence table

| Holding | Doctrine anchor | Exact baseline evidence | Required action |
|---|---|---|---|
| Authority tiers govern implementation. | `docs/README.md`; `docs/4-specs/SPEC_LEDGER.md` | Docs declare foundation above architecture, architecture above execution, and specs as subordinate staged work. SPEC_LEDGER has no 0059 entry at the target commit and records 0058 as completed/archive lineage. | Keep 0059 staged as `Status: PROPOSED`; mint no upstream doctrine and claim no certification. |
| Routines are defeasible intentions, not puppet strings. | INV-035; `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Foundation requires routines to behave as defeasible intentions and to route through cognition/planning rather than direct action puppetry. | Routine family may not be chosen merely because a window or schedule is tempting. |
| Single active intention is the planning authority; needs/routines do not directly dispatch. | INV-103, INV-104; `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Architecture requires an active intention in the actor-known planning context and forbids silently overwriting it with a routine label. | Bind routine family through `active_intention_by_actor` and intention record; mismatch is typed and non-override. |
| Scheduler/clock owns ordering and validation, not cognition. | INV-112; `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`; `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | Doctrine allows true time to trigger review or validation but forbids hidden truth/time from selecting goals, plans, routines, HTN methods, or primitive actions. | Window eligibility may wake an actor or open a transaction; it may not derive family independent of active intention. |
| Possession is cognition-neutral and parity-relevant. | INV-094, INV-108; archived specs 0057/0058 | 0057 established embodied continuation and possession parity. 0058 fixed embodied family derivation and required embodied/autonomous equivalence. | 0059 must preserve autonomous/embodied parity while narrowing its proof to autonomous family authority. |
| Feature acceptance is harsh and evidence-driven. | INV-098; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`; `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | Testing doctrine requires behavioral witnesses, typed diagnostics, replay-safe evidence, and honest mutation disposition. | Require metamorphic tests, fail-closed behavior, source/behavior guards, focused mutation, and four local gates. |
| Exact scheduler baseline shows the named window-keyed selector present and live. | `crates/tracewake-core/src/scheduler.rs` | `routine_window_family` (`:3240`) and `eligible_routine_execution_for_actor` (`:3260`, `.min_by(start_tick, execution_id)` at `:3284-3288`) are present at target commit `dffeefa` and unchanged at the reassessment baseline. The per-tick `transact_world_one_tick` "world_step" path passes `routine_window_family: None` (`:1108`); the no-human-day path `run_no_human_actor_decision_transaction` (`:3136`, reached via the `#[cfg(test)]` `run_no_human_day` harness) passes the window-keyed family (`:3143`). | Treat the verdict as **fix plus lock**: repair the window-keyed selector to bind through the active intention, then add the guard so the corrected binding cannot silently regress. |
| Transaction seam still permits a caller-supplied routine-window family. | `crates/tracewake-core/src/agent/transaction.rs`; `crates/tracewake-core/src/agent/generation.rs` | `ActorDecisionTransactionInput` contains `routine_window_family: Option<RoutineFamily>`; `ActorDecisionTransaction::run` reads active intention and passes routine-window goal pressure into candidate generation. | Enforce equality with active-intention-derived family or typed non-override/fail-closed handling; add adversarial tests. |
| 0058 green gates do not certify this seam. | `archive/reports/0058_embodied_routine_continuation_foundational_alignment_acceptance.md` | 0058 acceptance reports local gates passed and embodied/autonomous parity tests passed, but focused mutation remained survivorful and autonomous derivation was deliberately not audited. | Do not rely on 0058 as certification. Use it as lineage and require new 0059 evidence. |

---

## 5. Findings and remediation requirements

### F-0059-01 — Autonomous routine family must be active-intention-bound

**Invariants:** INV-035, INV-103, INV-112

Any autonomous routine-family helper must either be removed or made a thin wrapper around active-intention derivation. A compliant helper has the following semantics:

1. find the single active intention for `actor_id` via `agent_state.active_intention_by_actor`;
2. fetch the corresponding intention record;
3. verify it is a routine-continuation intention with a selected routine method and unresolved current step/execution;
4. derive the `RoutineFamily` from that bound intention/method/step;
5. return a typed non-routine/no-family outcome if the chain is absent, resolved, ambiguous, malformed, or not actor-owned;
6. never rank candidate routine executions by `start_tick`, `deadline`, `execution_id`, world truth, or any other clock/window fact as a family-selection authority.

If the implementation baseline contains a helper named `routine_window_family`, `eligible_routine_execution_for_actor`, or a renamed equivalent, it must satisfy the above contract. A helper that chooses `min_by(start_tick, execution_id)`, prefers a nonterminal scheduled execution, or otherwise selects a family without matching the active-intention chain is a violation to fix under this spec.

### F-0059-02 — Transaction must fail closed or ignore conflict; it must not override

**Invariants:** INV-103, INV-104, INV-105

The transaction seam may keep `ActorDecisionTransactionInput::routine_window_family` only if the value is treated as a consistency check or non-authoritative hint. Required behavior:

- `None` means no caller-supplied routine-window pressure. Routine continuation, if any, must arise from the active intention.
- `Some(family)` that equals the active-intention-derived family may be retained as an evidence annotation or compatibility bridge, but it must not add an independently higher-priority cognition source.
- `Some(family)` that conflicts with the active-intention-derived family must produce a typed, replayable non-override outcome. Acceptable implementations are either:
  - reject the transaction path with a typed stuck/diagnostic event explaining `routine_window_family_conflicts_with_active_intention`; or
  - ignore the conflicting hint while recording a typed diagnostic that the hint was non-authoritative.
- `Some(family)` with no active intention must never cause a `RoutineDuty` candidate or routine-family follow-on. The transaction may still route ordinary non-routine needs/idle fallback according to existing no-human rules when `include_idle_fallback` is true, but the routine-family derivation result is `None` or a typed non-routine outcome.

No implementation may silently let a caller-supplied routine-window family outrank or replace the active intention.

### F-0059-03 — Actor-known context validates and parameterizes after authority is bound

**Invariants:** INV-100, INV-103, INV-104

The active intention chooses the routine family. Actor-known context may then answer actor-known questions such as known workplace, known food source, known sleep affordance, route knowledge, possession-relevant input facts, and blocker explanations. It may not select work merely because the actor is at a workplace, select eat because food is visible, or select sleep because a bed is known when the active intention is a different routine family.

### F-0059-04 — Replay evidence must show source ancestry

**Invariants:** INV-018, INV-098, INV-105

Every fail-closed outcome and every accepted continuation must be replayable from the event log and actor-known context. Required evidence includes:

- actor id;
- tick/window that gave the scheduler opportunity, if any;
- active-intention id;
- selected method id and current routine step/execution id, if present;
- routine family derived from active intention, if present;
- caller-supplied routine-window family, if present;
- conflict/no-chain diagnostic kind;
- source event ids and source event kinds used by the transaction.

### F-0059-05 — No shortcut through shared resolver

**Invariants:** INV-103, INV-104, INV-112

The shared follow-on resolver in `crates/tracewake-core/src/agent/routine_continuation.rs` remains downstream. 0059 must not bypass it, fork it, or create a scheduler-owned primitive-action path. The resolver receives an already-authorized routine method/step/family context; it must not be asked to launder a scheduler-chosen family into actor cognition.

---

## 6. Doctrine clarifications routed as substance + home only

These are implementation guidance and proposed homes. They are not ratified doctrine text.

| Substance | Proposed home | Rationale |
|---|---|---|
| Routine-family authority is a consequence of the active intention, not of schedule/window eligibility. | `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Clarifies INV-035 without creating a new invariant. |
| True time may trigger a review but may not choose a routine family; holder-known temporal facts can only parameterize actor reasoning after the active intention exists. | `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` | Clarifies INV-112's temporal authority boundary. |
| `ActorDecisionTransactionInput::routine_window_family` is non-authoritative; mismatch with active intention is typed and replayable. | `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Clarifies transaction input contracts. |
| Execution tests must include adversarial actor-known states where tempting environmental facts disagree with active intention. | `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Captures acceptance pattern without new gates. |

---

## 7. Required fixtures, tests, and anti-regression measures

### 7.1 New behavioral/metamorphic test module

Create a new focused test module unless the implementing session demonstrates a cleaner existing home:

```text
crates/tracewake-core/tests/scheduler_routine_derivation_authority.rs
```

The test name family must include `0059`, `scheduler`, `routine`, `active_intention`, and `authority` so that source guards and mutation evidence can target it directly.

### 7.2 Metamorphic equivalence property

Define a helper that computes the authoritative routine family from the active-intention chain and a helper that observes the autonomous path's effective routine-family decision. Over a fixed family of actor-known states and active intentions, the autonomous result must equal the active-intention-derived result, or produce a typed, explained non-override outcome. A silent scheduler-chosen override is always failure.

Required adversarial cases:

| Case | Setup | Expected result |
|---|---|---|
| A1: workplace temptation while active intention is eat | Actor is at a known workplace or has a fresh workplace notice; active intention is eat/food routine. | Autonomous routine family is eat, or non-routine typed outcome if the active eat chain is intentionally unresolved; never work. |
| A2: workplace temptation while active intention is sleep | Actor has known workplace/open window facts; active intention is sleep. | Autonomous family is sleep; never work. |
| A3: inactive assigned routine execution | Actor has an assigned but inactive/non-authoritative routine execution with tempting family and earlier start tick. | Inactive execution is ignored; family comes from active intention or typed no-family. |
| A4: resolved execution | Actor has a resolved terminal execution with tempting family. | Resolved execution is ignored; no resurrection by window eligibility. |
| A5: another actor's execution | Another actor owns a due routine execution with tempting family. | Other actor's execution is ignored. |
| A6: no active intention | Actor has schedule/window facts but no active intention. | No routine family is derived; typed non-routine/fail-closed outcome records the absent chain. |
| A7: ambiguous or malformed active-intention chain | `active_intention_by_actor` points to missing/non-routine/resolved or multiple possible method bindings in test fixture. | Typed diagnostic; no guessed family. |
| A8: work step with actor-known route issue | Active intention is work, but actor-known context lacks route/workplace access. | Family remains work; follow-on becomes actor-known stuck/route blocker, not a family switch. |
| A9: hidden workplace truth only | True workplace exists but actor lacks actor-known source. | Hidden truth cannot choose work; family only from active intention. |
| A10: caller-supplied conflicting `routine_window_family` | Input supplies `Some(Work)` while active intention is eat/sleep, and vice versa. | Conflict is rejected or ignored with typed diagnostic; never an override. |

The property must compare at least these dimensions:

- routine family result;
- active-intention id and intention kind;
- selected method id and current unresolved step/execution id;
- candidate source/priority class, including absence of unauthorized `RoutineDuty`;
- chosen action id, target ids, and action parameters when a proposal is emitted;
- stuck diagnostic kind and source event ancestry when fail-closed;
- actor-known context source event ids/kinds;
- hidden-truth audit result or equivalent proof that world truth did not supply cognition.

### 7.3 Behavioral fail-closed test

Add a behavioral test proving that the autonomous derivation fails closed when the active-intention chain is absent, ambiguous, stale, resolved, or actor-mismatched. The test must assert a typed, eventful, replayable outcome rather than a `None` that can be mistaken for ordinary absence.

Acceptable fail-closed forms include:

- `StuckDiagnosticRecorded` with a specific reason such as `routine_family_requires_active_intention`;
- a transaction diagnostic event saying `routine_window_family_ignored_without_active_intention`;
- a typed non-routine idle fallback reason that explicitly records that no routine family was derived.

A silent `Work`, `Eat`, `Sleep`, or other routine-family choice from a schedule/window is failure.

### 7.4 Anti-regression source/behavior guards

Add at least one source guard and one behavioral guard. Recommended guard names:

```rust
guard_0059_scheduler_routine_family_authority_cannot_bypass_active_intention
guard_0059_no_clock_keyed_routine_family_selector_without_active_intention_binding
guard_0059_synthetic_negative_census_is_live
```

Synthetic negative fixtures must use the established negative-id convention. Required synthetic identifiers:

```text
synthetic_0059_window_keyed_routine_family
synthetic_0059_eligible_execution_min_by_start
synthetic_0059_routine_window_family_without_active_intention
synthetic_0059_conflicting_routine_window_hint
synthetic_0059_other_actor_execution_temptation
```

The source guard must fail if future code reintroduces a scheduler/window-keyed routine-family selector that is not visibly bound to `active_intention_for_actor`, `active_intention_by_actor`, or a helper whose only authority is the active-intention chain. The behavioral guard must fail if the synthetic conflicting-window cases can produce a routine-family candidate or action selection ahead of the active intention.

Source guards are not enough by themselves; they are only anti-drift tripwires. Behavioral evidence remains primary.

### 7.5 Local acceptance floor

The implementation must pass all four local gates:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

No gate may be reported as green without transcript or typed command output in the acceptance artifact.

---

## 8. Focused mutation discipline

0059 requires focused mutation over the touched seams. It does not require a full standing mutation campaign.

### 8.1 Mutation perimeter

The minimum mutation perimeter is:

```text
crates/tracewake-core/src/scheduler.rs
crates/tracewake-core/src/agent/transaction.rs
crates/tracewake-core/src/agent/generation.rs     # if touched or if routine-window candidate handling remains there
crates/tracewake-core/tests/scheduler_routine_derivation_authority.rs
crates/tracewake-core/tests/support/*             # only touched support helpers
```

### 8.2 Required mutant classes

The focused run must attempt mutants that would catch these failures:

- replacing active-intention-derived family with caller-supplied/window-derived family;
- removing actor-id ownership checks;
- accepting terminal/resolved routine executions;
- accepting another actor's routine execution;
- preferring earliest `start_tick` or `execution_id` over active-intention binding;
- suppressing conflict diagnostics;
- allowing `routine_window_goal` / `RoutineDuty` candidate generation without active intention;
- deleting actor-known source event ancestry from diagnostics;
- treating conflict as ordinary idle/no-op without typed explanation.

### 8.3 Suggested commands

Implementers may adjust command shape for the repository's current cargo-mutants version, but the report must preserve the file-targeted denominator and survivor disposition.

```text
cargo mutants   --package tracewake-core   --file crates/tracewake-core/src/scheduler.rs   --re 'due_loaded_actor_ids|transact_world_one_tick|routine.*family|eligible.*routine|ActorDecisionTransactionInput'   --output target/mutants/0059-scheduler   -- -p tracewake-core --test scheduler_routine_derivation_authority

cargo mutants   --package tracewake-core   --file crates/tracewake-core/src/agent/transaction.rs   --re 'active_intention_for_actor|goal_for_routine_family|ActorDecisionTransaction::run|routine_window_family'   --output target/mutants/0059-transaction   -- -p tracewake-core --test scheduler_routine_derivation_authority

cargo mutants   --package tracewake-core   --file crates/tracewake-core/src/agent/generation.rs   --re 'routine_window_goal|RoutineDuty|generate_candidate_goals|ContinueCurrentIntention'   --output target/mutants/0059-generation   -- -p tracewake-core --test scheduler_routine_derivation_authority
```

### 8.4 Honest disposition requirements

The acceptance artifact must include:

- command lines and environment summary;
- number of mutants attempted;
- caught/missed/unviable/timeout/error counts;
- exact survivor list;
- for every survivor, disposition as killed-by-added-test, equivalent with rationale, unviable with compiler/output evidence, timeout with reason, or accepted residual risk;
- explicit statement that survivorful focused mutation is **non-pass** unless every survivor is equivalent/unviable with evidence accepted by review.

A mutation run that omits the touched derivation seam or reports only aggregate green without survivor ledger is not evidence.

---

## 9. Acceptance-artifact specification

Use `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` as the acceptance shape. The acceptance artifact must be staged under `reports/` during implementation review and archived only after acceptance.

Minimum required sections:

1. **Header** — spec id 0059, implementation commit, baseline commit, repository, status, non-certification posture.
2. **Exact-source ledger** — target commit, manifest, exact URLs for all repository files used during implementation analysis, and confirmation that no branch/default/search/clone source was used for target-repository claims.
3. **Requirement ledger** — each 0059 requirement mapped to evidence, status, and source path/test path.
4. **Verdict update** — whether implementation found lock-only or fix-plus-lock; if fix-plus-lock, summarize the exact helper or path repaired.
5. **Behavioral evidence** — metamorphic cases A1-A10, expected/actual outcome, action/stuck/diagnostic ids, source event ancestry.
6. **Fail-closed evidence** — absent/ambiguous/mismatched active-intention cases and typed outcomes.
7. **Anti-regression guard evidence** — guard names, synthetic ids, negative-fixture behavior, and proof that the negative fixtures are live.
8. **Local gate transcripts** — four local gates with pass/fail command output.
9. **Focused mutation report** — denominator, results, survivor disposition, unviable/equivalent rationale.
10. **Non-certification wording** — explicit no latest-main, no whole-project, no new invariant, no Phase-4/second-proof claim.
11. **Computed result** — `pass` only if every scoped 0059 requirement passes and focused mutation is non-survivorful or all survivors are accepted equivalent/unviable; otherwise `non-pass` with blockers.

The acceptance artifact must not use casual wording such as "basically green" or "good enough". Use pass/non-pass grammar.

---

## 10. Implementation decomposition

One reviewable diff per ticket. Ticket ids below are recommended; the implementing session may adjust ticket filenames to match the repository's ticket-series tooling.

### 0059SCHROUT-001 — Baseline authority seam and helper contract

- Locate every autonomous routine-family derivation helper or caller.
- Confirm whether `routine_window_family` / `eligible_routine_execution_for_actor` are absent, renamed, or reintroduced at the implementation baseline.
- Implement or expose a single helper that derives routine family only through the active-intention chain.
- Remove or rewrite any clock/window-keyed selection of routine family.

Review focus: no family selection by `start_tick`, `deadline`, `execution_id`, terminal status, or environmental temptation unless the active-intention chain already binds the routine family.

### 0059SCHROUT-002 — Transaction conflict/fail-closed behavior

- Harden `ActorDecisionTransactionInput::routine_window_family` semantics.
- Add typed conflict/no-active-intention diagnostic behavior.
- Ensure ordinary non-routine needs/idle fallback remains intact where already allowed, but routine-family derivation remains absent without active intention.

Review focus: a conflicting `Some(RoutineFamily)` cannot become a `RoutineDuty` candidate or override the active intention.

### 0059SCHROUT-003 — Metamorphic behavioral proof

- Add A1-A10 metamorphic cases.
- Compare active-intention-derived result with autonomous effective result.
- Assert typed non-override outcomes for absent/ambiguous/conflicting chains.

Review focus: adversarial actor-known states are real fixtures, not mocked string checks.

### 0059SCHROUT-004 — Guards and synthetic negatives

- Add `guard_0059_*` source/behavior guards.
- Add `synthetic_0059_*` negative identifiers.
- Prove guard liveness by showing the synthetic negatives fail when the forbidden bypass is present.

Review focus: source guards catch obvious bypasses, but behavioral guard remains primary.

### 0059SCHROUT-005 — Focused mutation

- Run focused mutation over the touched seam files.
- Kill newly revealed real mutants with behavior tests, not brittle implementation assertions.
- Produce survivor disposition.

Review focus: no survivorful greenwashing.

### 0059SCHROUT-006 — Acceptance artifact and archival posture

- Produce the 0059 acceptance artifact under `reports/`.
- Update spec ledger only according to repository archival workflow after acceptance.
- Preserve exact-source discipline and non-certification wording.

Review focus: evidence ledger completeness and no overclaim.

---

## 11. Risks, routed-forward open questions, and invariant alignment

### 11.1 Risks

| Risk | Mitigation |
|---|---|
| The implementation baseline contains a renamed window-keyed helper not present at target commit. | Treat as fix-plus-lock under F-0059-01; do not expand scope. |
| Transaction hardening breaks ordinary non-routine no-human needs/idle fallback. | Tests must separate routine-family derivation from ordinary need routing; no active intention forbids routine family, not all actor behavior. |
| Source guard becomes brittle or cosmetic. | Pair every source guard with a behavioral guard and mutation evidence. |
| Mutation survivors are declared harmless without proof. | Acceptance artifact must include survivor-by-survivor evidence and mark survivorful unresolved mutation as non-pass. |
| Fix requires scheduler cadence or 0047 coordinator redesign. | Route forward as open question; do not hide it inside 0059. |

### 11.2 Routed-forward open questions

- **OQ-RFWD-0059-01:** If equivalence can only be enforced by changing planner/no-human cadence, route a new scheduler-cadence spec. Do not absorb it into 0059.
- **OQ-RFWD-0059-02:** The severe-need-interrupt consumer of routine eligibility remains out of scope. If it independently violates scheduler non-cognition authority, commission a separate severe-need-interrupt audit.
- **OQ-RFWD-0059-03:** Scheduler-owned routing for time-advancing routine follow-ons remains deferred by 0058's typed-stuck gateway.
- **OQ-RFWD-0059-04:** If future doctrine wants to state the active-intention routine-family authority rule explicitly, route the substance in §6 to foundation/architecture docs rather than editing doctrine inside this implementation spec.

### 11.3 Invariant alignment

| Invariant | Alignment |
|---|---|
| INV-035 | Treats routines as defeasible intentions by binding continuation family to active intention rather than schedule puppetry. |
| INV-094 | Preserves possession parity expectation by requiring autonomous/embodied family authority to be comparable without possession privilege. |
| INV-098 | Requires behavioral tests, typed replay-safe evidence, anti-regression locks, and focused mutation. |
| INV-103 | Keeps single active intention as planning authority. |
| INV-104 | Prevents routines/needs/schedules from directly dispatching primitive actions or choosing goals outside transaction/planner/pipeline. |
| INV-108 | Keeps possession input-only and cognition-neutral; no possessed/human route gains hidden family authority. |
| INV-112 | Limits scheduler/time to ordering and validation; forbids clock/window facts from acting as cognition authority. |

---

## 12. External research lane

External research shaped the testing posture only. It is not target-repository evidence.

- BDI/practical-reasoning literature treats intentions as committed, dynamically maintained reasoning structures rather than mere schedules; that supports testing for active-intention authority and against scheduler-chosen routine puppetry.[^bdi-aaai][^bdi-springer]
- Event sourcing practice treats state changes as reconstructible event sequences; that supports requiring source-event ancestry and replayable fail-closed diagnostics for routine-family conflicts.[^fowler-event-sourcing]
- Mutation-testing practice distinguishes caught/missed/unviable/timeout mutants and treats missed mutants as potential behavioral-test gaps; that supports 0059's survivor ledger and non-pass posture for unexplained survivors.[^cargo-mutants-outcomes][^cargo-mutants-timeouts]

[^bdi-aaai]: Anand S. Rao and Michael P. Georgeff, “BDI Agents: From Theory to Practice,” Proceedings of the First International Conference on Multi-Agent Systems, AAAI, 1995. Consulted as external research, not repository evidence: `https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf`.
[^bdi-springer]: Michael P. Georgeff et al., “The Belief-Desire-Intention Model of Agency,” in *Intelligent Agents V*, Springer, 1999. Consulted as external research, not repository evidence: `https://link.springer.com/chapter/10.1007/3-540-49057-4_1`.
[^fowler-event-sourcing]: Martin Fowler, “Event Sourcing,” consulted as external research, not repository evidence: `https://martinfowler.com/eaaDev/EventSourcing.html`.
[^cargo-mutants-outcomes]: `cargo-mutants` documentation, “Output,” consulted as external research for mutation-result categories and survivor interpretation: `https://mutants.rs/output/`.
[^cargo-mutants-timeouts]: `cargo-mutants` documentation, “Timeouts,” consulted as external research for timeout disposition: `https://mutants.rs/timeouts/`.

---

## Appendix A — Complete exact-fetch acquisition ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: dffeefa8e4105b7f4c6637f9bdb29dddea519a99
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open() with full raw.githubusercontent.com exact-commit URLs
Requested file count: 77
Successfully verified file count: 77
Fetched repository files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/docs/archival-workflow.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/archive/specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/archive/specs/0058_EMBODIED_ROUTINE_CONTINUATION_FOUNDATIONAL_ALIGNMENT_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/archive/reports/0058_embodied_routine_continuation_foundational_alignment_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/reports/0057-embodied-continuation-hardening-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/runtime/session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/agent/routine_continuation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/agent/transaction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/agent/generation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/agent/decision.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/agent/intention.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/agent/routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/agent/methods.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/agent/planner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/agent/actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/tests/embodied_autonomous_parity.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/tests/acceptance_artifact_wording.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/tests/acceptance_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/tests/mutation_completion_merge.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/crates/tracewake-core/tests/support/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/.cargo/mutants.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/dffeefa8e4105b7f4c6637f9bdb29dddea519a99/.cargo/mutants-baseline-misses.txt
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

---

## Appendix B — Boundary-awareness note

The boundary-awareness doctrine/spec tree was fetched to run the tier-fit test and confirm scope boundaries. Those files are not treated as conformance targets for 0059 remediation beyond their authority role. This spec corrects no boundary-awareness document and authorizes no downstream doctrine edit by itself.

## Outcome

Completed: 2026-07-01

Implemented the scoped 0059 fix-plus-lock package. The autonomous scheduler routine-family producer now binds family derivation to the actor's active intention before any actor-known or execution-window validation, the transaction/generation consumers treat caller-supplied routine-window family as a non-authoritative compatible hint, and the behavioral/guard/mutation layers lock the seam.

Deviations from the original plan: focused mutation in 005 initially produced real survivors, so a follow-up ticket `0059AUTSCHROU-007` was added and completed before the capstone. The acceptance report was moved to `archive/reports/0059_autonomous_scheduler_routine_derivation_acceptance.md` during final spec closeout.

Verification passed:

- `cargo test -p tracewake-core --test scheduler_routine_derivation_authority`
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo test -p tracewake-core --test anti_regression_guards guard_0059_synthetic_negative_census_is_live`
- Focused no-config mutation reruns: scheduler 49 tested / 44 caught / 5 unviable / 0 missed; transaction 12 tested / 9 caught / 3 unviable / 0 missed; generation 16 tested / 14 caught / 2 unviable / 0 missed.
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
