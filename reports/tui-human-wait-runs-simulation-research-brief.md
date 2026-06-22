# Research brief — "Waiting runs the simulation": wiring duration-completion + world advance into the human TUI path

> **For:** ChatGPT-Pro deep-research session (Session 2). **Locked / no questions** — produce the
> deliverable directly. The requirements below are final.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-22_6e91da7.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust (event-sourced
kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers govern
later ones. **Fetch every file from commit `6e91da7`** — the manifest reflects exactly that tree
(`git ls-tree -r HEAD`).

**Seed (read it, do not reproduce it).** This brief triages an in-repo defect report:
`reports/tui-human-wait-runs-simulation-issue.md`. That seed is a *different file* from the report
you will write (§7). The seed pins baseline commit `241d0d0` and was written before two later
commits landed; **use `6e91da7`, not `241d0d0`**, as your baseline. The seed's evidence-index
`file:line` anchors have drifted slightly under those commits (e.g. `is_duration_terminal` now lives
in `crates/tracewake-core/src/events/envelope.rs:363`, not in `scheduler.rs`). **Trust the named
symbols, not the seed's line numbers**; the verified seam map below supersedes the seed's evidence
index. The cited code paths are otherwise structurally unchanged between `241d0d0` and `6e91da7`
(the intervening commits touched `projections.rs`, a TUI golden, and a TUI test for an unrelated
inspect-token fix, plus skill/ledger files — none altered the duration/scheduler/submit seams this
brief depends on).

This is a **cold-start** brief — there is no predecessor research brief on this defect. The nearest
**cross-line structural precedent** (not a lineage predecessor — read it for *shape and obligation*,
not as a delta seed) is `archive/specs/0046_…PARITY_CONTRACT…` (see §2).

---

## 2. Read in full (authority order)

Read these before producing. Order strictly by authority tier. Entries marked **[primary]** are
load-bearing for this target; entries marked **[boundary]** are read to *bound scope* (know what is
in vs. out, run the tier-fit test) — they are not conformance or amendment targets unless a finding
genuinely lands there.

**Universal (load-bearing for every deliverable):**
- `docs/README.md` — authority order and the layering rule.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001…INV-112`; every recommendation must
  satisfy these (esp. single-charge-per-tick, possession parity, temporal authority `INV-112`,
  no-hidden-truth, no-fact-from-prose). **[primary]**

**`docs/0-foundation/`** (the user asked for the whole tier; primaries called out, rest boundary):
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **the violated doctrine**: "Waiting is not skipping
  causality. Waiting runs the simulation." (L259); staged time controls (L261–272); sleeping-actor
  actor-known summaries, no omniscient leakage (L274, L276). Primary amendment candidate. **[primary]**
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — temporal authority (`INV-112`) primary home; replay
  determinism any new event-emitting advance path must round-trip under. **[primary]**
- `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — sleep/work as first-class actor actions that
  span time; survival/recovery doctrine (fatigue must be recoverable by sleeping). **[primary]**
- `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — needs/charging doctrine the recovery vs.
  charge accounting must respect. **[primary]**
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — where a human-driven "ordinary day" sits in
  first-playable scope and acceptance gates. **[primary]**
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — a slept-through interval must yield
  actor-known / record-derived summaries, never hidden-truth leakage. **[primary]**
- `00`, `01`, `04`, `07`, `09`, `10`, `11`, `13` — **[boundary]** (run the tier-fit test; `09`
  no-scripting / `04` information-flow are relevant anti-contamination context).

**`docs/1-architecture/`:**
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — **the architecture home**: the
  "Temporal rendering and embodied play loop" section already states world-advancing controls are
  commands that advance authoritative event/replay time *through the ordinary pipeline*; this is the
  contract the fix must satisfy and the most likely architecture amendment site. **[primary]**
- `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — pipeline/scheduling/no-direct-
  dispatch contract the advance path must go through. **[primary]**
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — replay/projection determinism for a new
  advance path. **[primary]**
- `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — actor decision/need
  accounting seam. **[primary]**
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — the conformance rows the seed names: "0017 tick-charge
  attribution authority" and "0017 shared open-duration authority". **[primary]**
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — actor-known summary surfacing /
  firewall the missed-event summary must consume. **[primary]**
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — proof/acceptance artifact shape
  for the recommended implementation. **[primary]**
- `01`, `06`, `07`, `08`, `09`, `11`, `12`, `14` — **[boundary]**.

**`docs/2-execution/`:**
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — execution proof mechanics for
  the scheduler/pipeline the advance path drives. **[primary]**
- `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — the no-human per-tick advance the human
  path would reuse; need/duration proof obligations. **[primary]**
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — possession/debug proof shape for the
  actor-known summary surface. **[primary]**
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — current baseline + acceptance
  contract this capability sits relative to. **[primary]**
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — confirms the certification ladder state
  (all cert gates passed; this is post-`FIRST-PROOF-CERT` feature work, not a cert audit). **[primary]**
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — anti-contamination gates the
  summary surface must pass. **[primary]**
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — evidence-honesty + diagnostic
  obligations for the recommended proof. **[primary]**
- `00`, `01`, `08`, `09`, `11`, `12`, `13` — **[boundary]**.

**`docs/3-reference/`:**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — review checklist any recommendation must clear.
  **[primary]**
- `01_DESIGN_RISK_REGISTER.md` — relapse/risk modes a new advance path touches. **[primary]**
- `02_GLOSSARY.md` — **[boundary]** (canonical terms; do not mint new ones).

**`docs/4-specs/`:**
- `SPEC_LEDGER.md` — campaign state, authority posture, "Next known execution move", and the full
  archived-spec table (the cert ladder verdicts). **[primary]**
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — the acceptance-artifact template a recommended
  implementation spec would later produce against. **[primary]**
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`, `README.md` — **[boundary]**.

**Precedent + seed:**
- `archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md`
  — **structural precedent**: the two-hop playable-capability parity contract, the test-side capability
  registry, the deterministic conformance runner, and the **standing per-feature parity obligation
  (PAR-010 / PAR-011)** every future feature must satisfy. It covered sleep/work_block *coverage* but
  **not** duration *completion in the human submit path* — that gap is this brief's subject. **[primary]**
- `archive/reports/0046-parity-acceptance-artifact.md` — the parity baseline (21 capabilities covered),
  so your recommendation *extends* parity rather than duplicating it. **[primary]**
- `reports/tui-human-wait-runs-simulation-issue.md` — **the seed defect report** (the file you triage;
  a different file from the report you write). **[primary]**

**Code seams to inspect** (read them in the repo; do not assume the seed's line numbers — verified map):
- `crates/tracewake-core/src/actions/defs/sleep.rs` — `build_sleep_start_event` (~L18, emits
  `SleepStarted`, `body_exclusive=true`, `fatigue_recovery_per_tick`, `expected_completion_tick`);
  `build_sleep_completion_events` (~L167, emits `SleepCompleted`/`SleepInterrupted`).
- `crates/tracewake-core/src/actions/defs/work.rs` — `build_work_start_events` (~L18);
  `build_work_completion_events` (~L119); the "work block started; completion is duration scheduled"
  effects summary.
- `crates/tracewake-core/src/actions/defs/wait.rs` — `build_wait_events` (~L32; `ActorWaited`, wait is
  instantaneous, not a duration).
- `crates/tracewake-core/src/scheduler.rs` — `run_no_human_day` (~L344, the only routine that walks
  ticks and fires completions, via an `append_due_completions` step); `DeterministicScheduler::advance_one_tick`
  (~L212); `is_due_sleep_start` (~L686); `is_due_work_start` (~L691). **Note the completion engine is
  already factored as `append_due_completions` — the seed's "factor an advance_one_tick step" direction
  may be partly pre-done; assess.**
- `crates/tracewake-core/src/need_accounting.rs` — `classify_actor_tick_regimes_with_start` (~L38),
  `open_body_exclusive_starts`, `actor_has_open_body_exclusive_duration`, `DuplicateDurationTerminal`
  (the single-charge-per-tick and shared-open-duration authority surface).
- `crates/tracewake-core/src/events/envelope.rs` — `is_duration_terminal` (~L363).
- `crates/tracewake-tui/src/app.rs` — `submit_entry` (~L246, the human submit path: calls
  `run_pipeline` once, sets `current_tick`, records perception, returns — **no completion/advance
  call**); `record_current_place_perception_and_project` (~L290); debug-gated `run_no_human_day`
  (~L308).
- `crates/tracewake-tui/src/run.rs` — `UiCommand::WaitOneTick` dispatch (~L72).
- `crates/tracewake-core/src/view_models.rs` — `EmbodiedViewModel` (~L19, the 21-field surface the
  two-hop parity contract destructures exhaustively).
- `crates/tracewake-core/src/projections.rs` — `build_embodied_view_model` (~L567).
- `crates/tracewake-tui/src/render.rs` — embodied render boundary (exhaustive destructure; PAR-002).

---

## 3. Settled intentions (final — do not re-open)

1. **Deliverable is a single consolidated recommendation report**, not a numbered spec. The
   numbered-spec numbering / ledger / epoch rules do **not** apply. File: see §7.
2. **Commit to one primary recommended solution** to the wiring gap, with explicit rationale, and
   document the alternatives you considered and rejected (and why). Do not return an undecided menu.
3. **World-tick semantics is in scope to resolve.** Determine and recommend what "Waiting runs the
   simulation" means for **unpossessed actors** when a human waits — full authoritative world tick vs.
   advancing only the possessed actor's own open durations + perception — and define the **actor-known
   missed-event summary** surface (how other actors' results reach the possessed actor through modeled
   channels), with **zero hidden-truth leakage** (`08`:L274/L276, doc `14`, doc `1-arch/03`).
4. **Recommend the concrete next artifact.** Beyond the design + doc-amendment recommendations, include
   a **forward-routing recommendation** for the implementation vehicle: which scoped artifact should be
   authored next (a new numbered spec — name the scope, not paste-ready spec text), and a **ticket-level
   decomposition outline** (one ticket per reviewable diff). Do **not** author the spec or the tickets.
5. **Doc amendments are in scope where genuinely necessary, across any tier.** For each, deliver
   *substance + home* — what doctrine the target doc must own, written at the right altitude for that
   tier, and which file/section it lands in (new section / addition / correction). Do **not** produce
   paste-ready wording or invent identifiers (`INV-###`, gate codes, risk IDs, glossary terms); minting
   those remains the repo's own reassess/amend process. Foundation `08` is the most likely amendment
   site; assess whether `INV-112`/temporal-authority or any other invariant needs amendment vs. is
   already sufficient (prefer "already sufficient, operationalize downward" unless the constitution
   genuinely cannot bear the capability).
6. **Body-exclusive reservation enforcement in the human path** (seed §8 final bullet — `wait` after an
   open sleep is currently `Accepted`, not `ReservationConflict`): you decide and state whether it
   belongs in the same recommendation or routes as a separate hardening item.
7. **The certification ladder is fully passed** (`P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT →
   FIRST-PROOF-CERT`, per `SPEC_LEDGER.md`). This is **post-`FIRST-PROOF-CERT` capability/feature work**,
   not a certification audit or mutation-remediation pass. Do not frame the deliverable as a cert spec.
8. **The fix must satisfy, not bypass, the spec-0046 two-hop parity contract** and its standing
   per-feature parity obligation (PAR-010 / PAR-011): any new/changed playable capability or view-model
   field must receive an explicit surface disposition and real-pipeline witness. Treat extending parity
   coverage to duration *completion* as part of the recommendation's obligations.

---

## 4. The task

Diagnose the design knot fully and **determine the single best solution** to wiring per-tick
authoritative simulation advance — including duration completion (`sleep`/`work_block`) and correct
need accounting — into the human-possessed TUI `wait` / time-control path, so that "Waiting runs the
simulation" holds in human play (an actor can recover fatigue by sleeping; work blocks complete and
produce output), while preserving event-sourced replay determinism, single-charge-per-tick accounting,
shared open-duration authority, subjective epistemics, and the truth firewall. This is a **thorny
design/spec-level fix** (with a secondary doc-overhaul dimension: propose foundation/architecture/
execution/reference amendments where the doctrine must be sharpened to admit the capability). Resolve
the world-tick-semantics fork and the actor-known summary surface, then route the work forward into a
concrete next implementation spec + ticket outline.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above. **Research online as deeply
as needed** — comparable implementations and research/prior art on: human-driven time advancement in
event-sourced / deterministic simulations; "wait until" / "advance-to-terminal" mechanics and turn
vs. real-time loops in immersive-sim / roguelike / colony-sim possession models; reconciling a
player's local clock with an authoritative world tick; modeled-perception / missed-event "what you
learn while asleep" surfaces that avoid omniscient leakage; deterministic replay of time-skip. Cite
sources for any external claim that shapes a decision.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every product-behavior
  decision must satisfy it; a genuine divergence requires *amending an invariant first* (propose the
  amendment as substance + home), never designing against it silently.
- Authority order: if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong.
- **Single-charge-per-tick (0017 tick-charge attribution):** every tick-delta need charge is reconciled
  by counted `(actor, need, tick)` occurrences — one charge per tick. A naive "wait *and also* run sleep
  recovery" must not double-charge or double-recover a tick.
- **Shared open-duration authority (0017):** body-exclusive duration start/terminal keying is shared
  across scheduler, pipeline, and need-accounting; duplicate/orphan terminals must fail closed.
- **Replay determinism:** any new event-emitting advance path must round-trip identically under
  replay/rebuild.
- **Subjective epistemics / truth firewall:** a slept-through interval (and any other-actor activity a
  human "waits" through) yields actor-known / record-derived summaries only; the possessed actor must
  not gain omniscient missed-event summaries. Possession is not a knowledge upgrade; no player-only
  verbs; the TUI never applies events or mutates state directly (`1-arch/10`).
- **No-fact-from-prose / no scripting:** summaries such as "slept until morning" are actor-known
  conclusions or debug-only labels, never hidden-truth leakage or authored narration treated as fact.
- No backwards-compatibility shims or alias paths in new work.

---

## 7. Deliverable specification

Produce exactly one **new** downloadable markdown document:

- `reports/tui-human-wait-runs-simulation-research-report.md` — **new** (does not replace anything;
  distinct from the seed `reports/tui-human-wait-runs-simulation-issue.md` and from this brief
  `…-research-brief.md`).

This is an **analysis / recommendation report**, not a `docs/4-specs/` artifact — do **not** apply any
spec-numbering / ledger / epoch rules. Deliver **substance + home, not ratified text**. Recommended
shape (reuse the campaign's canonical recommendation-report structure):

1. **Disposition table** — one row per finding → target doc *or* code seam → verdict (amend / add /
   correct / new-code / route-forward) → one-line basis.
2. **Method / provenance ledger** — what you read, what you researched online (with citations), how you
   verified the seam map against `6e91da7`.
3. **Per-finding sections** — driver → current coverage (doc + code) → tier-fit verdict → recommendation
   (Session-2 prose at the right altitude; for code, the intended change in design terms, naming the
   seam, not paste-ready Rust).
4. **Resolved design decision** — the single committed solution to the wiring gap (covering: the advance
   mechanism and where it lives relative to `run_no_human_day` / `append_due_completions` / the human
   submit path; the world-tick-semantics resolution per §3.3; the actor-known missed-event summary
   surface; the body-exclusive reservation resolution per §3.6), with **alternatives considered and
   rejected** and why. Show how it satisfies each constraint in §6 (single-charge, open-duration
   authority, replay determinism, firewall, parity).
5. **Forward-routing appendix** — the recommended next implementation artifact (proposed scoped numbered
   spec: name + scope, *not* spec text) and a ticket-level decomposition outline (one ticket per
   reviewable diff). Because this defect is **cross-cutting** (spans foundation → code), the appendix
   routes only to owner/reassess decisions and future implementation specs — not to a single lower tier.
6. **Open questions** — anything genuinely unresolved after research.
7. **References** — cited sources + repo files.

**Locked / no questions:**

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- The deliverable set matches §7 exactly: one new `reports/tui-human-wait-runs-simulation-research-report.md`.
- A **single primary solution is committed** (not a menu), with rejected alternatives documented.
- **World-tick semantics is resolved** (other-actor advance + actor-known missed-event summary surface),
  and the resolution leaks **no hidden truth** (satisfies `08`:L274/276, foundation `14`, arch `03`).
- Every recommendation satisfies single-charge-per-tick, shared open-duration authority, and replay
  determinism — checked explicitly against the `need_accounting.rs` / `scheduler.rs` / `envelope.rs`
  seams.
- The recommendation **extends, and does not bypass,** the spec-0046 two-hop parity contract and the
  PAR-010/011 per-feature obligation.
- Doc-amendment findings give *substance + home* only — **no** paste-ready wording, **no** invented
  `INV-###` / gate codes / risk IDs / glossary terms; no new doctrine weakens an upstream tier.
- A concrete next artifact (proposed spec scope + ticket outline) is routed forward; no spec/ticket is
  authored.
- Every external claim is cited.
- The `6e91da7` baseline contains every file named in §2 (it does), and line numbers were re-verified
  against the live code rather than copied from the seed.
