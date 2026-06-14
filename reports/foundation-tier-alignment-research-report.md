# Foundation-tier alignment research report

**Target repository:** `joeloverbeck/tracewake`
**Target commit:** `9c1203f7cd076b8ded8ff3f56a0083e7ff53620e`
**Deliverable path:** `reports/foundation-tier-alignment-research-report.md`
**Status:** recommendation report only; not ratified doctrine text; no spec number; no `docs/4-specs/SPEC_LEDGER.md` entry.
**Scope:** foundation-tier operationalization of the settled promotion of time / calendar / social rhythm.
**Identifier discipline:** this report recommends substance and placement only. It deliberately mints no new `INV-###` number, no gate code, and no glossary identifier.

I am not verifying that this commit is the current `main`. I am using the supplied commit as the target of record and fetched files only by exact commit URL from `joeloverbeck/tracewake`.

---

## 1. Executive determination

The settled promotion is correct, but it must be encoded more sharply than “time exists in events.” The current foundation already references simulation time, deterministic ordering, acquisition time, believed event time, staleness, routines, office hours, travel time, LOD summary intervals, and truth-firewall boundaries. What it lacks is a named **temporal authority doctrine** that separates authoritative event/replay time from holder-known temporal claims, institution-known procedural time, routine/social rhythm, deadline/lateness semantics, freshness/staleness, and LOD summary intervals. Without that split, the repository invites the exact relapse the campaign is trying to prevent: the scheduler, calendar, or replay clock becoming an oracle for cognition, institutions, embodied views, or LOD promotion.

The recommendation is a **compact cross-document mini-package** with `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` as the primary home, a **new or amended constitutional invariant in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`**, and targeted local cross-references in `04`, `05`, `07`, `08`, `10`, `12`, `13`, and `14`, plus an index update in `00`. This is better than a single isolated section in `03`: a single home preserves authority, but time leaks through every decision surface. The cross-references must be short and doc-local, not duplicated doctrine and not mechanism. They should all preserve the same rule: **authoritative time may validate ordering, intervals, legality, replay, and due effects; it may not plan, infer, render embodied knowledge, or produce institution/procedure conclusions unless the relevant temporal fact has entered the holder-known context through modeled channels.**

---

## 2. Disposition table

| Foundation document checked | Verdict | One-line basis |
|---|---|---|
| `docs/0-foundation/00_FOUNDATION_INDEX.md` | Inline index update | The index maps the foundation spine; if `03` becomes the primary temporal-authority home, the map should say so without adding doctrine. |
| `docs/0-foundation/01_PROJECT_CHARTER.md` | No change | The charter already frames the product as causality-first, belief-bounded, ordinary-life simulation and does not need temporal mechanism or amendment prose. |
| `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` | New or amended invariant warranted | A constitutional temporal doctrine cannot live only in prose; the invariant must bind event/replay time to the truth firewall across cognition, institutions, TUI, and LOD. |
| `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` | New primary section | This is the proper home for the temporal authority model because event anatomy already owns simulation time, deterministic ordering, replay, summary intervals, and boundary inputs. |
| `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` | Targeted cross-reference / inline addition | The doc already owns acquisition time, believed event time, staleness, memory uncertainty, and record time; it needs to name temporal claims as holder-known, not free truth labels. |
| `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Targeted cross-reference / inline addition | The doc already owns routines, schedules, replanning, and long-horizon projects; it must require actor-known or institution-known temporal premises for routine/social rhythm use. |
| `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` | No change | It already makes hunger, fatigue, sleep, wages, work, debt, travel, search, and action duration causal; `03`/`05`/`07`/`14` can carry temporal authority without duplicating action doctrine here. |
| `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` | Targeted cross-reference / inline addition | The doc already owns office hours, records, queues, delays, evidence thresholds, procedure failures, and timestamps; it needs procedural-time firewall language. |
| `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` | Targeted cross-reference / inline addition | The doc already owns time controls, waiting, sleep summaries, actor-filtered views, and debug separation; it needs the temporal-firewall hook for embodied time displays and missed-event summaries. |
| `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` | No change | Seed-time routines, cadences, and boundary inputs are already allowed as possibility-space machinery; adding temporal doctrine here would risk duplicating no-script and prehistory rules. |
| `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` | Targeted cross-reference / inline addition | The doc already owns summary intervals, regional cadence, prehistory, and LOD promotion; it needs to require temporal and information ancestry when summarized time is promoted. |
| `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` | No change | Temporal utterances are already structured speech-act claims behind validation; `04` and `14` cover the needed boundary without adding LLM-specific temporal doctrine. |
| `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` | Targeted cross-reference / acceptance addition | First playable must prove that offices, stale memories, waiting/sleep, routines, and records do not let the clock become an invisible actor oracle. |
| `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | Inline source-note addition | The doc is the foundation precedent for source notes; the amendment should record temporal modeling sources while refusing to import tick/calendar mechanics. |
| `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` | Inline cross-reference / clarification | The truth firewall is already authoritative; it should explicitly classify raw simulation time/current clock/procedural due facts as validator truth unless source-backed for the holder. |

---

## 3. Method & provenance ledger

### 3.1 Exact-commit fetch ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 9c1203f7cd076b8ded8ff3f56a0083e7ff53620e
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run open against exact raw.githubusercontent.com URLs
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/reports/foundations-completeness-determination-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/reports/verdict-on-foundations.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/reports/foundations-completeness-determination-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/archive/reports/foundation-amendment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/archive/reports/foundation-amendment-lower-tier-routing.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/4-specs/SPEC_LEDGER.md
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

The predecessor determination report contains an older baseline commit string in its own provenance. That stale internal string was observed and quarantined because the brief explicitly directs this pass to use `9c1203f7cd076b8ded8ff3f56a0083e7ff53620e`. It is not treated as repository freshness evidence, target-commit evidence, or latest-main evidence.

### 3.2 Tier-fit test applied

The authority ordering is `docs/0-foundation/` before `docs/1-architecture/`, `docs/2-execution/`, `docs/3-reference/`, and `docs/4-specs/`. The live docs also state that implementation convenience never defeats accepted gates and that branch names, repository metadata, and stale source strings are not source authority. This report therefore asks only foundation-tier questions: what conceptual doctrine must be non-negotiable, what file owns it, and which lower-tier mechanisms are deliberately out of scope.

Foundation owns the **temporal authority model**: the distinction between event/replay time, holder-known temporal claims, institution-known procedural time, routine/social rhythm, freshness/staleness, deadline/lateness, and LOD summary intervals. Foundation does **not** own tick size, calendar syntax, duration units, scheduler data structures, fairness algorithms, UI clock rendering, exact staleness thresholds, or the first-playable vocabulary for “morning,” “last night,” “late,” “due,” or “season.” Those route down.

### 3.3 Settled boundaries not re-opened

This report accepts the predecessor determination as settled input. Exactly one theme is promoted: **time / calendar / social rhythm**. The other eight themes from the originating verdict are not promoted here and appear only in the forward-routing appendix. The prior campaign’s settled seven are also not reopened. The prior `0026` amendment is used only as shape precedent: it shows how a foundation amendment can add a compact constitutional hook, a primary doctrine home, and lower-tier follow-through without turning the recommendation report itself into ratified text.

### 3.4 External research use

External research was used only to sharpen the conceptual boundary:

- W3C OWL-Time distinguishes instants, intervals, durations, temporal ordering, temporal position, and non-Gregorian reference systems, which supports naming temporal categories without choosing Tracewake’s calendar syntax. [EXT-W3C-OWL-TIME]
- SimPy’s simulation-time documentation separates simulation time and deterministic event scheduling from wall-clock synchronization, which supports keeping replay/scheduler time out of cognition and UI embodiment unless modeled. [EXT-SIMPY-TIME; EXT-SIMPY-REALTIME]
- The ODD protocol asks model descriptions to specify temporal/spatial resolution and process scheduling — what entities do, at what time, in what order, and when variables update — which supports routing scheduler and reporting detail below foundation rather than encoding it here. [EXT-ODD]
- Continuous-time ABM work notes that fixed timesteps are common but not conceptually mandatory, and that timestep choice and scheduling strategy can affect validity; this supports the foundation’s refusal to pick tick size while requiring a doctrine that lower tiers must honor. [EXT-CT-ABM]
- Foundational ontology work for discrete-event simulation argues that unclear mappings between modeling primitives and domain concepts can produce semantic misunderstanding; this supports giving Tracewake a clear temporal authority vocabulary. [EXT-DESO]

---

## 4. Per-finding sections

### 4.1 `02_CONSTITUTIONAL_INVARIANTS.md` — invariant question

**Driver.** The predecessor determination calls temporal authority constitutional but its hand-off named `03` plus cross-references, not `02`. The brief requires a direct answer: either place temporal authority in the invariants document or justify why a constitutional doctrine can remain outside it.

**Current coverage.** `02` already has strong adjacent invariants. It says actors, households, institutions, and groups act from beliefs, records, memories, observations, testimony, rumors, inferences, and procedures, not hidden ground truth. It makes meaningful changes eventful, deterministic replay foundational, staleness persistent until modeled update, routines defeasible, institutions non-omniscient, records timestamped and stale-risk-bearing, LOD summaries ancestry-preserving, and truth-firewall violations high-severity. It already contains a 2026 hardening block stating that truth may validate actions but may not plan them, that the scheduler is not cognition authority, and that LOD summary processes must preserve the firewall.

**Placement / tier-fit verdict.** A temporal invariant is warranted. The doctrine is not merely a descriptive section for `03`; it is a hard constraint across event ordering, actor cognition, institutional procedure, embodied views, LOD, and acceptance. Leaving it out of `02` would make the foundation say “constitutional” in prose while failing to bind lower tiers in the constitution. That is incoherent for this repository’s authority model.

**Recommendation.** Add one new or amended invariant in `02` without assigning a number. Its content should be placed in the truth-firewall/cognition-authority neighborhood, with a cross-reference back to the events/replay family. It should own this substance:

- Authoritative simulation time, event order, intervals, durations, and due effects may validate replay, ordering, action legality, scheduled consequences, and causal explanation.
- Cognition, routine selection, institutional procedure, embodied view models, speech interpretation, leads, and LOD promotion may use temporal facts only when those facts are available to the relevant holder through modeled channels: perception, memory, record, notice, routine or role assignment, procedure state, testimony, expectation, or summary ancestry.
- Deadline, lateness, staleness, “expected by now,” “yesterday,” “last night,” “office closed,” and similar labels are not free truth labels. They are claims, procedure states, or holder-known/institution-known interpretations with provenance.
- The scheduler and replay clock may order and validate. They must not become cognition authority.

This is one invariant-level hook, not a full temporal specification. It should not choose tick size, date syntax, duration units, exact staleness windows, or UI rendering.

---

### 4.2 `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — primary temporal authority home

**Driver.** The determination named `03` as the primary home for temporal authority. The verdict’s gap is that the current foundation gestures at time everywhere but lacks an authority split among physical/event time, actor-perceived time, institutional time, social rhythm, deadlines/lateness, staleness, and LOD intervals.

**Current coverage.** `03` already says the authoritative world is deterministic simulation plus event stream, causal ancestry, projections, typed claims, beliefs, traces, records, and replay/debug machinery. Event anatomy already includes simulation time and deterministic ordering. It requires cause models, failure/refusal events, absence-as-evidence through expectation/perception/search, replay from ordered events, boundary inputs with cadence or trigger, source intervals for summary events, and LOD summaries that preserve the distinction between what happened and who knew it.

**Placement / tier-fit verdict.** `03` should be the single authoritative home for the temporal authority model. It is the only foundation doc that can bind event/replay time, summary intervals, boundary cadence, and replay without making time a UI, scheduler, or institution-only concern.

**Recommendation.** Add one new section to `03`, at foundation altitude, after event anatomy or after deterministic replay. The section should own the conceptual model, not mechanisms. It should define these categories:

1. **Authoritative event/replay time:** the simulation’s ordered temporal substrate for event ordering, replay, duration validation, summary intervals, and due effects.
2. **Holder-known temporal claims:** “yesterday,” “last night,” “recently,” “late,” “stale,” “expected by now,” remembered timing, perceived darkness, read timestamps, heard rumors, and temporal uncertainty as claims or beliefs held by a specific holder.
3. **Institution-known procedural time:** office windows, filing windows, queues, deadline states, payment due states, sanction windows, procedure aging, notice lifecycle, and institutional delays as procedure/record-backed states.
4. **Routine/social rhythm:** work, sleep, meals, patrols, worship, market days, household duties, and social visits as defeasible patterns that require source-backed temporal expectations.
5. **Freshness/staleness authority:** acquisition time, believed event time, last verification, and stale risk as epistemic/procedural data, never automatic truth correction.
6. **LOD and regional temporal summaries:** summary intervals, cadence, seasonal/regional processes, and promoted history as ancestry-bearing approximations.
7. **Temporal firewall:** truth clock may validate; holder-known time may plan.

This section should also explicitly refuse lower-tier choices: no tick size, calendar/date syntax, duration unit, scheduler queue structure, UI clock display, exact stale-after value, or first-playable calendar vocabulary.

---

### 4.3 `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — temporal claims and staleness

**Driver.** The determination calls out “yesterday,” “last night,” “late,” “stale,” and “expected-by-now” as holder-known claims rather than free truth labels.

**Current coverage.** `04` already distinguishes ground truth, subjective belief/memory, and public/institutional claims. It states that beliefs have holder, stance, confidence, source, channel, acquisition time, believed event time, staleness, provenance chain, contradiction links, and scope. It also treats records as claims with authored time, event time, record time, amendment time, and time uncertainty, and it says stale information remains stale until updated by modeled channels.

**Placement / tier-fit verdict.** `04` should not define the temporal authority model; it should apply it to claims, belief, memory, and records. This is a cross-reference target, not the primary home.

**Recommendation.** Add a compact inline addition or cross-reference near the sections on claim shape, memories, records, and staleness. It should state that temporal expressions and freshness/lateness labels must be represented as holder-known or artifact/institution-known temporal claims with provenance. It should require a distinction among at least these conceptual slots where important: when the claimed event occurred, when the holder acquired or last verified the claim, when the record was created/amended/read, and what uncertainty or staleness risk remains. It should also say that the world clock cannot silently update a memory, record, lead, or notebook merely because time has passed; change requires a modeled update, contradiction, verification, decay, or procedure.

---

### 4.4 `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — routines and social rhythm

**Driver.** The temporal promotion exists partly because workdays, meals, sleep, patrols, office attendance, and social rhythm can easily become hidden scripts or scheduler puppetry.

**Current coverage.** `05` already says agents are symbolic BDI actors with beliefs, needs, intentions, routines, local planning, and replanning. It says projects may span hours or days, routines are recurring intentions with context and interruption, schedules are not teleports or puppet strings, and ordinary agents can fail, replan, wait, or do nothing when blocked.

**Placement / tier-fit verdict.** `05` should apply temporal doctrine to planning without defining scheduler mechanics. The foundation-level rule is not how calendars are represented; it is what temporal premises may enter a decision.

**Recommendation.** Add a short cross-reference near routines/planning. It should require routines, jobs, duties, social appointments, and schedule-following to consume only actor-known, holder-known, or institution-known temporal premises. An actor may go to work because they believe it is work time, remember their assignment, read a notice, hear a bell, infer lateness from darkness, observe a queue, or hold a role obligation. The actor may not go because the scheduler read true office hours or because a global calendar row says the right action is “work.” Scheduler triggers may create a decision opportunity, but candidate generation and routine continuation must still pass through the actor-known transaction and ordinary validation.

---

### 4.5 `07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — procedural time

**Driver.** The verdict identifies institutional/procedural time — office windows, delays, deadlines, filing, records, sanctions, and lateness — as part of the promoted foundation gap.

**Current coverage.** `07` already describes institutions as fallible social machines with roles, records, procedures, norms, resources, delays, office hours, queues, budgets, and failure. It treats reports, ledgers, notices, contracts, debts, warrants, receipts, and orders as records with authors, timestamps, custody, claims, provenance, access, staleness, and contradiction potential.

**Placement / tier-fit verdict.** `07` should not define event/replay time. It should define the institutional application: institutions act from institution-known context, and their temporal statuses must be procedure- and record-backed.

**Recommendation.** Add an inline doctrine hook near institutions/procedures/records. It should state that office hours, filing windows, due dates, lateness, queue aging, record expiration, notice lifecycle, wage/payment periods, case delays, sanction windows, and procedural aging are institution-known or artifact-backed states. A procedure may validate against authoritative time, but an institution’s conclusion or action must arise from records, reports, role knowledge, office procedure, evidence thresholds, observed absence, or modeled delay. A debt should not become “late” as a free label; it becomes late only under a due rule, clock validation, and an institution/holder-facing record/procedure state with provenance.

---

### 4.6 `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — time controls and embodied temporal surfaces

**Driver.** The brief specifically asks whether time controls and embodied views need a temporal-firewall cross-reference.

**Current coverage.** `08` already says the TUI is the product boundary, normal play is actor-filtered, debug is non-diegetic, and time controls are allowed for waiting, sleeping, traveling, working, and long-running actions. It also says advancing time must run the simulation, not skip causality, and that a sleeping or absent actor should receive only actor-known summaries rather than omniscient summaries.

**Placement / tier-fit verdict.** A small cross-reference is warranted. The TUI is a prime leak surface: clocks, timelines, waiting summaries, “missed events,” and “why not” labels can quietly reveal truth. The foundation should not define UI clock rendering, but it must state the boundary.

**Recommendation.** Add a compact inline addition near time controls and embodied/debug separation. It should say that time controls may advance authoritative event/replay time, but embodied views may render temporal facts only when the possessed actor could know or infer them through modeled channels. Debug may display exact simulation time, event order, hidden due effects, or omitted truth, but it must be visibly non-diegetic. “You slept until morning,” “the office is closed,” “the payment is late,” and “a lot happened while you were away” must be actor-known summaries, record-derived conclusions, or debug-only labels, not hidden-truth leakage.

---

### 4.7 `10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — summary intervals and regional cadence

**Driver.** The temporal doctrine must cover LOD summary intervals, regional/seasonal cadence, boundary inputs, and promotion/demotion.

**Current coverage.** `10` already says Tracewake must scale through honest abstraction. It gives summary events, LOD promotion/demotion, regional processes, source intervals, inputs/outputs, affected claims/records/traces, fidelity limits, and replay/debug visibility. It says regional weather, caravans, disease, road closures, tax orders, long-past history, and prehistory enter through declared cadence/trigger and ancestry.

**Placement / tier-fit verdict.** `10` should cross-reference the `03` temporal authority section, not duplicate it. Its foundation job is to make summarized time safe for later high-detail cognition and institutions.

**Recommendation.** Add an inline cross-reference near summary intervals / regional cadence / promotion. It should require every LOD or boundary summary that later affects actor, institution, household, regional, or embodied behavior to preserve both temporal ancestry and information ancestry: interval covered, cadence/trigger, known-to-whom claims, public records/notices, rumor chains, role assignments, last-verification/staleness risk, and fidelity limits. A promoted actor or institution may inherit only temporal knowledge explicitly attributed through preserved ancestry, not the aggregate truth used to maintain the low-detail simulation.

---

### 4.8 `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first-playable proof surface

**Driver.** The determination says first playable must not treat time as an invisible oracle for actor decisions. The brief also says this report must not mint gate codes.

**Current coverage.** `12` already scopes first playable around a small village, ordinary needs/routines, records, office hours, stale memory, actor-filtered TUI, no-human proof, deterministic replay, and truth-firewall acceptance. It requires proof that an actor decision transaction uses actor-known state and that institutions, records, and ordinary life are testable.

**Placement / tier-fit verdict.** `12` should turn temporal doctrine into acceptance pressure without defining execution gates. It should not choose a calendar vocabulary or exact scenario set.

**Recommendation.** Add an acceptance cross-reference that first playable is not complete unless temporal features demonstrate the firewall. The proof should include, at the level of doctrine not gate code: a worker acting from believed work time or assignment rather than true global schedule; a closed/late/stale institutional state arising from procedure/record/attempt/notice; sleep/wait/travel advancing event time without granting omniscient knowledge; a stale record remaining stale until modeled update; and replay showing the difference between validator time and actor/institution-known temporal premises. The doc should explicitly avoid specifying tick size, UI time display, exact day-part labels, or stale-after durations.

---

### 4.9 `13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — temporal source note

**Driver.** The brief identifies `13` as the precedent for how foundation decisions cite research and where a temporal source note would land.

**Current coverage.** `13` currently records foundation research decisions and source notes for doctrine such as event sourcing, institutions, LOD, ordinary life, and anti-misreads. It captures why external sources support the shape of doctrine without importing their implementation details.

**Placement / tier-fit verdict.** A source note is warranted, but it must be a note, not doctrine. The doctrine belongs in `02`/`03` and cross-references.

**Recommendation.** Add a compact source note when the amendment lands. It should cite temporal modeling / simulation reporting sources as rationale for naming temporal authority categories and for refusing to pick a mechanism in the foundation. The note should emphasize: temporal instants/intervals/durations can be named without choosing a Gregorian or wall-clock representation; simulation time and wall-clock time are distinct; ABM scheduling/time resolution belongs in model/execution detail; and unclear temporal concepts cause semantic drift. It should explicitly say these sources do not ratify a tick size, calendar syntax, duration unit, or scheduler implementation.

---

### 4.10 `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — temporal firewall clarification

**Driver.** Every recommendation must preserve the truth firewall. The temporal doctrine exists because time is an especially tempting hidden-truth channel.

**Current coverage.** `14` already states the core doctrine: truth may validate actions, but truth may not plan them. It defines the actor decision transaction, sealed actor-known context, provenance classes, proposal-versus-validation split, scheduler boundary, institutions under the firewall, debug quarantine, and LOD summary contamination boundary. It already allows the scheduler to choose actor/time windows, advance deterministic time, apply due completions, and run time acceleration; it forbids scheduler cognition authority.

**Placement / tier-fit verdict.** `14` does not need a new doctrine family, but it should explicitly name temporal truth as a firewall case. Without that, lower-tier work may treat clock/current-time access as “not hidden truth” because it is not a secret physical fact.

**Recommendation.** Add one clarification near the scheduler boundary, actor-known context, or provenance classes. It should state that raw simulation time, exact event order beyond what a holder can know, future scheduled completions, true office windows, true due states, and exact summary intervals are validator/debug truth unless source-backed for the holder. The holder-known version may be a perceived day-part, remembered appointment, read timestamp, routine assignment, public bell, posted hours, institutional queue state, due notice, stale record, or summary knowledge with ancestry. The cross-reference should point to the new `03` temporal authority section and the new/amended `02` invariant.

---

### 4.11 `00_FOUNDATION_INDEX.md` — index support

**Driver.** The index must remain an accurate map of foundation authority after the temporal amendment lands.

**Current coverage.** `00` currently maps the foundation tier, explains the live foundation files, and points to the updated spine after prior hardening. It does not yet identify `03` as the primary home for temporal authority.

**Placement / tier-fit verdict.** The index should be updated only as a map. It should not define temporal doctrine.

**Recommendation.** Add a short index phrase under `03` noting that it owns causal event/replay authority and temporal authority doctrine, with supporting cross-references to `02`, `04`, `05`, `07`, `08`, `10`, `12`, and `14`. This prevents future readers from missing the primary home while keeping the doctrine itself in the substantive files.

---

## 5. Forward-routing appendix: the eight routed themes

This is the top-tier route-down case. Nothing routes upward. All later sessions must account for the foundation temporal amendment landing first.

| Routed theme | Destination tier(s) | Route-down note after temporal amendment |
|---|---|---|
| Play experience / legibility | Architecture, execution, reference, future TUI/play-loop specs | Later work should show temporal ignorance, stale leads, missed events, waiting, lateness, and “expected by now” in playable embodied surfaces without letting UI clock truth become actor knowledge. |
| Quantity / granularity / fungibility | Architecture, execution, future economy/inventory specs | Later work should handle spoilage, service capacity, wages, debts, partial transfers, stock, payments, and fungibility over time through event/procedure records, not free counters. |
| Bounded affect / emotion | Architecture, reference, future cognition specs | Later work should model persistence, salience, decay, rehearsal, and emotional timing as holder-known memory/cognition dynamics, not as raw clock-driven truth. |
| Learning / adaptation | Architecture, execution, reference | Later work should represent recency, frequency, habituation, correction, skill/relationship changes, and institutional learning through provenance-bearing events and memories under the temporal firewall. |
| Authoring / compiler discipline | Execution, reference, future content/compiler specs | Later work should validate temporal seed fields, routine/cadence declarations, due rules, record timestamps, and prehistory intervals without allowing content to script runtime outcomes. |
| Deterministic performance / fairness budgets | Architecture, execution, reference, future scheduler/LOD specs | Later work should prove no wall-clock dependence, deterministic ordering, fair decision windows, no hidden iteration nondeterminism, and safe time acceleration/summary intervals. |
| Bias / social-harm practicality | Architecture, execution, reference, future domain-procedure specs | Later work should track how delay, deadline rules, office access, queue priority, stale records, patrol cadence, and procedure aging can encode unequal treatment and must remain inspectable. |
| Staged incompleteness as an escape valve | Execution, reference, future spec templates | Later work should state what the first temporal model proves, what it abstracts, what vocabulary is provisional, and what future calendar/social-rhythm mechanisms are deliberately deferred. |

---

## 6. Open questions

These are owner decisions or lower-tier decisions. They are not promoted here.

1. **Minimal first-playable temporal vocabulary.** Architecture must decide the minimal vocabulary for day-parts, sleep/wake periods, office windows, work/meal rhythm, “yesterday/last night,” due/lateness labels, and stale/verified phrasing. Foundation should not pick it.
2. **Calendar representation and duration units.** Architecture/execution must choose whether first playable uses ticks, named day-parts, hours, abstract windows, calendars, or domain-specific periods. Foundation only requires authority separation.
3. **Exact staleness/freshness policy.** Architecture/reference must decide how to encode acquisition time, believed event time, last verification, stale-after hints, confidence drift, memory decay, and record lifecycle. Foundation only requires provenance and modeled update.
4. **Temporal TUI surface.** Product/architecture must decide how much objective time the embodied TUI can show when the actor has clocks, bells, daylight, calendars, routines, or records, and how debug time is visibly quarantined.
5. **Deadline and procedural aging primitives.** Architecture must decide how due rules, filing windows, queues, office hours, sanction windows, debt lateness, wage periods, and notice expiration are modeled as institution-known procedure state.
6. **LOD temporal equivalence.** Architecture/execution must define what summary-interval fidelity is sufficient for promotion: what temporal facts, uncertainty, public memory, and record/procedure ancestry must survive.
7. **Simultaneity and tie-breaking.** Execution must define deterministic ordering for equal-time or same-window events. Foundation requires deterministic replay and no hidden cognition authority, not a tie-break algorithm.
8. **No new non-temporal foundation promotion found.** This pass found no genuinely new foundation-level gap beyond the settled temporal promotion. The other eight themes remain routed down by the predecessor determination.

---

## 7. References

### 7.1 Repository sources fetched at exact commit

- `docs/README.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/README.md`
- `docs/0-foundation/00_FOUNDATION_INDEX.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/00_FOUNDATION_INDEX.md`
- `docs/0-foundation/01_PROJECT_CHARTER.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/01_PROJECT_CHARTER.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`
- `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `reports/foundations-completeness-determination-research-report.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/reports/foundations-completeness-determination-research-report.md`
- `reports/verdict-on-foundations.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/reports/verdict-on-foundations.md`
- `reports/foundations-completeness-determination-research-brief.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/reports/foundations-completeness-determination-research-brief.md`
- `archive/reports/foundation-amendment-research-report.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/archive/reports/foundation-amendment-research-report.md`
- `archive/reports/foundation-amendment-lower-tier-routing.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/archive/reports/foundation-amendment-lower-tier-routing.md`
- `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/3-reference/02_GLOSSARY.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/3-reference/02_GLOSSARY.md`
- `docs/4-specs/SPEC_LEDGER.md` — exact URL fetched: `https://raw.githubusercontent.com/joeloverbeck/tracewake/9c1203f7cd076b8ded8ff3f56a0083e7ff53620e/docs/4-specs/SPEC_LEDGER.md`

### 7.2 External sources

- [EXT-W3C-OWL-TIME] W3C / OGC, **Time Ontology in OWL**, W3C Recommendation / OGC document. Used for the idea that temporal doctrine can distinguish instants, intervals, durations, ordering, temporal positions, and alternative reference systems without committing to a particular calendar syntax. https://www.w3.org/TR/owl-time/
- [EXT-SIMPY-TIME] SimPy documentation, **Time and Scheduling**. Used for the simulation-time / event-scheduling distinction and deterministic ordering precedent. https://simpy.readthedocs.io/en/latest/topical_guides/time_and_scheduling.html
- [EXT-SIMPY-REALTIME] SimPy documentation, **Real-time simulation**. Used to distinguish event-based simulation time from wall-clock synchronized execution. https://simpy.readthedocs.io/en/latest/api_reference/simpy.rt.html
- [EXT-ODD] Grimm et al., **The ODD Protocol for Describing Agent-Based and Other Simulation Models: A Second Update to Improve Clarity, Replication, and Structural Realism**, *Journal of Artificial Societies and Social Simulation* 23(2)7, 2020. Used for the reporting boundary that temporal/spatial resolution and process scheduling must be specified in model detail, not constitutionalized as one universal mechanism. https://www.jasss.org/23/2/7.html
- [EXT-CT-ABM] Köster et al., **A Fast Embedded Language for Continuous-Time Agent-Based Simulation**, *Journal of Artificial Societies and Social Simulation* 27(1)10, 2024. Used for the warning that timestep/scheduling choices affect model validity and therefore belong below foundation. https://www.jasss.org/27/1/10.html
- [EXT-DESO] Guizzardi and Wagner, **Towards an Ontological Foundation of Discrete Event Simulation**, Winter Simulation Conference 2010. Used for the value of clear conceptual mapping in discrete-event simulation language design. https://www.informs-sim.org/wsc10papers/059.pdf
- [EXT-DEMO] Silver et al., **DeMO: An Ontology for Discrete-event Modeling and Simulation**, *SIMULATION* 87(9), 2011. Consulted as prior art for discrete-event modeling ontology; the recommendation does not rely on importing its ontology into Tracewake. https://journals.sagepub.com/doi/10.1177/0037549710386843
