# Research brief — Autonomous scheduler routine-family derivation, active-intention authority hardening

*This file is the prompt to paste into a fresh ChatGPT-Pro deep-research session ("Session 2"). Session 2 sees only this prompt plus the uploaded manifest. It is self-contained and locked: produce the deliverable directly; do not interview.*

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-30_dffeefa.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first playability).
Docs are layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` →
`4-specs`; **earlier tiers govern later ones**. **Fetch every file from commit `dffeefa`
(`dffeefa8e4105b7f4c6637f9bdb29dddea519a99`)** — the manifest reflects exactly that tree.

**This brief is a delta, not a cold start.** It is the follow-up scheduler spec that archived spec
**0058** explicitly deferred (0058 §2.2 and §9.3). The campaign line so far:

- `archive/specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md`
  established the embodied `Continue routine` capability and possession parity.
- `archive/specs/0058_EMBODIED_ROUTINE_CONTINUATION_FOUNDATIONAL_ALIGNMENT_HARDENING_SPEC.md`
  hardened the **embodied** side: it removed a pre-active-intention workplace selector from
  `LoadedWorldRuntime::embodied_routine_window_family` (OQ1 fix — derive routine family from the
  active intention's current step, not from an environmental fact), added a temporal gateway
  (time-advancing follow-ons are typed-stuck until scheduler-owned routing exists), and proved
  embodied/autonomous output equivalence (`crates/tracewake-core/tests/embodied_autonomous_parity.rs`).
- `archive/reports/0058_embodied_routine_continuation_foundational_alignment_acceptance.md` records
  that all four local gates passed at implementation commit
  `2d98a221b547af8b8b687c3a9e36143f2c7cbb73`, that the equivalence tests pass, and — importantly —
  that focused mutation remained **survivorful** and that the **autonomous derivation was deliberately
  not examined**.

The lineage **structural precedent** for *this brief's shape* is
`reports/0057-embodied-continuation-hardening-research-brief.md` (the brief that produced 0058). Read
it for shape only; the delta seed for *content* is 0058 itself.

**The unexamined question this brief commissions.** After 0058, the two paths that answer
"which routine step should this actor do next?" derive the family differently:

- **Embodied path** (`crates/tracewake-core/src/runtime/session.rs::embodied_routine_window_family`)
  now derives the family from the actor's **single authoritative active intention** (its selected
  routine method + bound unresolved current step). This is the 0058 OQ1 fix.
- **Autonomous path** (`crates/tracewake-core/src/scheduler.rs::routine_window_family`, which
  selects via `eligible_routine_execution_for_actor`) derives the family from a **time-keyed
  scheduled window**: `start_tick <= window.start_tick && deadline-not-passed && not-terminal`,
  then `min_by(start_tick, then execution_id)`. **It never consults the active intention.**

Doctrine (INV-035: routines are defeasible intentions; INV-103/104: the single active intention is
the planning authority and routines/needs do not dispatch directly; INV-112: the scheduler may order
and validate but is **not** a cognition authority) implies these two derivations *should* coincide —
the window-eligible execution *ought* to always be the execution bound to the active intention. 0058's
equivalence tests demonstrate coincidence **for the cases they cover**, but no one has audited whether
the autonomous derivation is *foundationally* sound: can the scheduler's clock-driven window selection
ever pick a routine family ahead of, or divergent from, the active intention — making the scheduler a
cognition authority in violation of INV-112/INV-035? This brief commissions exactly that audit and the
spec that closes it.

---

## 2. Read in full (authority order)

Read the **primary (load-bearing)** entries below in full before producing. The user also wants the
whole doc tree available; entries marked *boundary-awareness* are read **only to bound scope** (to run
the tier-fit test and know what is *out* of scope) — they are **not** conformance targets and must not
be "corrected."

**Primary — doctrine (authority order):**

- `docs/README.md` — the authority-tier ordering and the layering rule.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001…INV-NNN`; every decision must satisfy
  these. The anchors here are INV-035, INV-094, INV-098, INV-103, INV-104, INV-108, INV-112.
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — routines as defeasible
  intentions; the single active intention as planning authority.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — the actor-known
  cognition transaction and the truth firewall the derivation must respect.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — temporal authority /
  replay determinism the scheduler owns.
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — the
  actor decision transaction contract that the autonomous path drives.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — scheduler /
  pipeline sequencing and no-direct-dispatch; the execution-tier home for scheduler authority.
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — no-human routine
  derivation and the ordinary-life proof the autonomous path must keep satisfying.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — mutation/evidence
  discipline and review-artifact expectations the spec's acceptance must follow.

**Primary — lineage:**

- `archive/specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md`
  — established the capability and possession-parity contract.
- `archive/specs/0058_EMBODIED_ROUTINE_CONTINUATION_FOUNDATIONAL_ALIGNMENT_HARDENING_SPEC.md` — the
  delta seed: read §2.2, §3 (H-0058-05 R2, H-0058-03 OQ1), §4.4, and §9.3 closely. The autonomous
  derivation `scheduler.rs::routine_window_family` is named there as the equivalence baseline that 0058
  consumed but did **not** rewrite; §4.4/§9.3 flag that any divergence reconcilable only by changing the
  autonomous derivation is "a new scheduler spec, not 0058 ticket creep" — that spec is what this brief
  commissions.
- `archive/reports/0058_embodied_routine_continuation_foundational_alignment_acceptance.md` — what is
  already proven (equivalence tests pass; gates green) vs. left open (survivorful mutation; autonomous
  side unexamined). Treat its per-requirement evidence as established for the *embodied* side and as the
  starting baseline — not a finding to redo.

**Primary — code seams** (read, do not assume; the embodied mirror is already fixed):

- `crates/tracewake-core/src/scheduler.rs` — the audit subject. Focus: `routine_window_family`,
  `eligible_routine_execution_for_actor`, `active_intention_for_actor`, and the autonomous
  `ActorDecisionTransaction` path that calls `routine_window_family(...)`. Note
  `eligible_routine_execution_for_actor` has a second consumer near the severe-need-interrupt path —
  inspect it to understand the derivation's contract, but the interrupt path is **out of scope** (§3).
- `crates/tracewake-core/src/runtime/session.rs` — `embodied_routine_window_family` (the post-0058
  active-intention-authority mirror) and `run_embodied_continue_routine_follow_on`; the reference for
  what "derive family from the active intention" looks like once correct.
- `crates/tracewake-core/src/agent/routine_continuation.rs` — the shared actor-known follow-on resolver
  both paths funnel through.
- `crates/tracewake-core/src/agent/transaction.rs` — the actor decision transaction internals.

**Boundary-awareness (read to bound scope, not conformance targets):** the remainder of
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and
`docs/4-specs/*` (including `docs/4-specs/SPEC_LEDGER.md` for ledger posture and
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` for the acceptance shape). Read these to run the
tier-fit test and confirm scope boundaries; do not audit or amend them.

---

## 3. Settled intentions (locked — do not re-open)

1. **Narrow scope.** Audit **only** the autonomous routine-family derivation
   (`scheduler.rs::routine_window_family` / `eligible_routine_execution_for_actor`) against
   active-intention authority. The core property to settle: does the window-eligible routine execution
   always coincide with the execution bound to the actor's single authoritative active intention, or can
   the time-keyed window selection pick a different routine family ahead of / divergent from the active
   intention (a scheduler-as-cognition-authority violation of INV-112/INV-035)?
2. **Out of scope (negative settled intentions — do not raise as findings).** The broader
   "scheduler is not a cognition authority across *all* derivations" audit; the severe-need-interrupt
   path in `scheduler.rs`; the embodied side (already fixed and locked by 0058); planner/no-human
   cadence; the 0047 authoritative world-tick coordinator; the deferred scheduler-owned time-advancing
   follow-on routing (0058's typed-stuck gateway stands). 0058 §2.2/§9.3 fenced these as separate work.
   If you find a genuine divergence that can only be reconciled by redesigning planner cadence or the
   0047 coordinator, **record it as a routed-forward open question (§7 appendix), do not expand this
   spec to cover it.**
3. **Determination mode (i) — always produce the spec, verdict as a section.** Render a clearly labeled,
   evidence-based **verdict**: is the autonomous derivation a genuine foundational-alignment violation,
   or does it provably coincide with active-intention authority? Then **always** author the numbered spec
   (§7): if a violation, the spec specifies the fix + anti-regression locks; if clean, the spec still
   **locks the coincidence** as a metamorphic anti-regression property (the 0058 "keep locked" pattern —
   the derivation must be unable to silently drift away from active-intention authority in future code).
   The spec has value either way.
4. **Non-certification posture** (mirror 0058). The spec mints **no** new invariant, gate, glossary term,
   or design-risk ID; it is subordinate to live doctrine tiers; it makes no whole-project, latest-main,
   P0/SPINE/EPI/ORD-LIFE/FIRST-PROOF cert, Phase-4, or second-proof claim. Any doctrine clarification is
   routed as **substance + home only**, not ratified text.
5. **You author the spec; you do not execute code.** The authoritative determination of equivalence is
   proven by the *implementing* session's tests/mutation. You may include a clearly-labeled
   **preliminary static analysis** of whether the derivation looks like a real violation (read from the
   code at the baseline), marked *preliminary, not certification*. The spec must specify the behavioral
   tests, metamorphic equivalence proof, anti-regression guard(s), and focused-mutation discipline that
   the implementing session will run — not a rendered pass/fail.
6. `assumption: the spec is numbered 0059 and staged at
   specs/0059_..._SCHEDULER_ROUTINE_DERIVATION_ACTIVE_INTENTION_AUTHORITY_HARDENING_SPEC.md`. The staging
   epoch is contiguous through 0058 with no live staging file and no intervening renumber, so 0059 is the
   next number; the implementing session may confirm against the live ledger and adjust if the epoch moved.

---

## 4. The task

This is a **hardening / anti-contamination** pass whose deliverable is a **numbered implementation
spec**. Audit the autonomous scheduler routine-family derivation against the constitution's
active-intention-authority and scheduler-non-cognition-authority rules, determine whether deriving the
routine family from a time-keyed scheduled window (rather than from the actor's single authoritative
active intention) is a genuine foundational-alignment violation, and author a scoped hardening spec
that either remediates the violation or locks the proven coincidence as a standing anti-regression
property. The spec must satisfy harsh feature acceptance (INV-098): behavioral tests before source
scans, a metamorphic equivalence property over adversarial actor-known states, typed/replayable
evidence, and focused mutation over the touched seam files.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above. Research online as deeply as
needed — similar implementations (event-sourced agent simulations, scheduler/cognition-authority
separation, intention/BDI planners, defeasible-routine systems), research papers, and prior art —
wherever it sharpens the deliverable. **Cite sources for any external claim that shapes a decision.**

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every product-behavior
  decision must satisfy it; a genuine divergence requires amending an invariant **first**, never
  designing against it silently. The load-bearing anchors:
  - **INV-035** — routines are defeasible intentions, not puppet strings.
  - **INV-094 / INV-108** — possession parity is tested; possession is input-only and cognition-neutral.
  - **INV-098** — feature acceptance is harsh (behavioral proof, replay-safe, regression-locked).
  - **INV-103 / INV-104** — the single active intention is the planning authority; routines/needs do not
    dispatch primitive actions directly — they route through the transaction/resolver/planner/pipeline.
  - **INV-112** — temporal authority: time may order and validate, but the scheduler/replay clock must
    not become a cognition authority.
- Authority order: if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong.
- No backwards-compatibility shims or alias paths in new work.
- Anti-contamination: no simulation fact may be born from prose; preserve event-sourced causality,
  subjective epistemics, possession parity, and validation/replay. Actor-known context may parameterize
  or validate the derivation; it may never select a routine family ahead of the active intention.

---

## 7. Deliverable specification

Produce **one** downloadable markdown document:

- **`specs/0059_<UPPERCASE_SNAKE_TITLE>_SCHEDULER_ROUTINE_DERIVATION_ACTIVE_INTENTION_AUTHORITY_HARDENING_SPEC.md`**
  — **new** file (not a replacement). Keep the basename ≤ ~200 characters (abbreviate theme tokens if
  needed, preserving the leading `0059_` and the lineage tail). Stage it in `specs/`; it is archived to
  `archive/specs/` only at acceptance.

Spec structure — follow the sibling 0058 spec's section shape (it is the canonical precedent for this
series): a status/posture header (Status: PROPOSED; **omit** any `Outcome`/post-acceptance sections —
those are added only at acceptance), baseline/source-discipline statement pinned to commit `dffeefa`, a
**determination/verdict** section (per mode (i), §3.3), an evidence table mapping each holding to its
doctrine anchor and baseline evidence and required action, findings & remediation requirements,
doctrine clarifications routed as substance + home only, required fixtures/tests/anti-regression
measures, the focused-mutation discipline, the acceptance-artifact specification (using
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`), an implementation decomposition (one reviewable
diff per ticket), and a risks/open-questions + invariant-alignment section.

The spec must require, at minimum:

- A **metamorphic equivalence property**: over a fixed family of actor-known states + active intentions
  (including adversarial cases — actor at a known workplace while the active intention is eat/sleep; an
  assigned-but-inactive routine execution with a tempting family; a resolved execution; another actor's
  execution; an actor with no active intention), the autonomous window-derived routine family must equal
  the active-intention-derived family, or the divergence must be a typed, explained outcome — never a
  silent scheduler-chosen override. This generalizes 0058's embodied/autonomous equivalence test to
  *prove the autonomous derivation itself is bound to the active intention*.
- A behavioral test that the autonomous derivation **fails closed** (typed, eventful) rather than
  guessing a family when the active-intention chain is absent or ambiguous.
- At least one **anti-regression source/behavioral guard** (house `guard_*` + `synthetic_*` negative-id
  convention) that fails if future code reintroduces a window/clock-keyed family selector that bypasses
  the active intention.
- **Focused mutation** over the touched seam files (`scheduler.rs` derivation functions and any test
  support), with honest disposition of survivors/unviable/equivalent mutants — **not** a full standing
  campaign.
- The four local gates (`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D
  warnings`; `cargo build --workspace --all-targets --locked`; `cargo test --workspace`) as the
  acceptance floor.

If the verdict is **clean** (the derivations provably coincide), the spec's required work is the
**lock**: the metamorphic equivalence property + the anti-regression guard, framed as proving and
preserving an already-correct property, with the verdict section stating the evidence for coincidence.

**Locked / no questions:** Produce the deliverable directly as a downloadable markdown document. Do not
interview, do not ask clarifying questions — the requirements above are final. If a genuine
contradiction makes a requirement impossible, state it in the deliverable and proceed with the most
faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- The deliverable is exactly one new `specs/0059_…_HARDENING_SPEC.md`, staged (not archived), Status
  PROPOSED, with no post-acceptance sections.
- A labeled, evidence-based **verdict** is present (mode (i): spec produced regardless of verdict).
- Scope is the autonomous derivation only; the out-of-scope items in §3 are not raised as findings, and
  any cross-cutting discovery is routed forward as an open question, not folded into the spec.
- Every doctrine reference cites a real invariant by number from
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` at commit `dffeefa`; the spec mints no new
  invariant/gate/glossary/risk ID and weakens no upstream tier.
- The spec requires a metamorphic equivalence property + fail-closed behavior + an anti-regression
  guard + focused mutation + the four local gates.
- Every external claim that shaped a decision is cited.
- The `dffeefa` fetch-baseline commit contains every file named in the §2 read-in-full list.
