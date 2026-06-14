# Spec 0031 — Foundation Temporal Authority Doctrine Amendment

This spec **proposes a foundation-tier (constitutional) amendment**. It is a design/proposal
artifact: it specifies the *substance and home* of the amendment so Tracewake's reassess /
amendment process can ratify it. **It does not itself enact doctrine, assign an `INV-###`
number, author ratified constitutional wording, or coin a glossary identifier.** That
ratification is a deliberate constitutional act and requires explicit sign-off before any
`docs/0-foundation/` file is edited.

> Section set: this file uses the canonical `specs/` section set (Problem Statement, Approach,
> Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions) because
> `specs/` carries no template for doctrine-amendment proposals and this is not a hardening
> implementation spec. It mirrors the shape precedent of archived spec
> `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`.

**Status:** COMPLETED — ratified by the `0031FOUTEMAUT-001` ticket implementation and archived as
historical provenance.

**OWNER:** I'm signing off on these amendments. Proceed.

**Authority posture:** This spec sits beneath foundation, architecture, execution, and reference
doctrine and may not amend, replace, weaken, or redefine them by spec authority. It **stages** a
foundation amendment for the foundation-amendment process to ratify under that process's own
authority. If implemented, the resulting foundation edits are foundation-tier doctrine, not
spec-tier text, and this spec becomes historical provenance for why the amendment was made.

**Admissibility posture:** `P0-CERT not applicable`. This is a doctrine-amendment proposal; it
certifies no code and performs no gate audit. It names no execution gate code as a local
definition.

**Provenance:** derived from `reports/foundation-tier-alignment-research-report.md` (external deep
research, target commit `9c1203f7cd076b8ded8ff3f56a0083e7ff53620e` = current `HEAD` `9c1203f`) and
its brief `reports/foundation-tier-alignment-research-brief.md`. The report's verdict-critical
claims were independently re-verified against live `HEAD` during authoring (see Verification §V1).
The report deliberately mints no `INV-###`, gate code, or glossary identifier; this spec preserves
that discipline.

---

## 1. Problem Statement

The predecessor foundations-completeness campaign settled exactly one new theme for promotion to
the foundation tier: **time / calendar / social rhythm**. The current foundation already *gestures*
at time everywhere — simulation time and deterministic ordering in `03`, acquisition time / believed
event time / staleness in `04`, routines and office hours in `05`/`07`, time controls and sleep
summaries in `08`, LOD summary intervals in `10`, and the scheduler/time-acceleration boundary in
`14`. The 2026 truth-firewall hardening block (`INV-099`…`INV-111`) already binds truth-may-validate-
not-plan (`INV-099`), the scheduler is not cognition authority (`INV-103`), cognition inputs require
provenance (`INV-102`), and LOD summaries must preserve the firewall (`INV-110`).

What the foundation **lacks** is a single, named **temporal authority doctrine** that separates:

- authoritative event/replay time (the validator substrate),
- holder-known temporal claims ("yesterday", "last night", "late", "stale", "expected by now"),
- institution-known procedural time (office windows, filing windows, due/lateness states, queue
  aging, notice lifecycle),
- routine / social rhythm (work, sleep, meals, patrols, market days as defeasible patterns),
- freshness / staleness as epistemic data, never automatic truth correction, and
- LOD / regional temporal summaries as ancestry-bearing approximations.

A grep of the live foundation confirms the hole: no `temporal authority`, `holder-known temporal`,
or `procedural time` doctrine exists in any `docs/0-foundation/` file. Without that explicit split,
the repository invites the exact relapse the campaign is trying to prevent: the **scheduler,
calendar, or replay clock becoming an oracle** for cognition, institutions, embodied views, or LOD
promotion. Time is an especially tempting hidden-truth channel because raw clock access does not
*feel* like a secret physical fact, so lower-tier work may treat "current time" or "true office
hours" as "not hidden truth" and quietly plan from it.

This is a genuine foundation-level hole. The separation of authoritative time from holder-known
temporal knowledge is a `what`-level product-identity claim (a Tracewake actor acts from *believed*
work time, not the true global schedule); product identity is foundation's job. Leaving it implicit
across eight documents places a constitutional truth below the tier that defines what Tracewake is —
the same reasoning that promoted the truth firewall (`docs/0-foundation/14`) and the emergence-
evidence acceptance principle (`INV-111`, archived spec `0026`) into the constitution.

## 2. Approach

Promote a compact, durable **temporal authority doctrine** as a cross-document mini-package, and
nothing more. The package has a single authoritative home, one constitutional hook, and short
doc-local cross-references — never duplicated doctrine and never mechanism.

The shared rule every deliverable must preserve:

> **Authoritative time may validate ordering, intervals, legality, replay, and due effects; it may
> not plan, infer, render embodied knowledge, or produce institution/procedure conclusions unless
> the relevant temporal fact has entered the holder-known or institution-known context through
> modeled channels** (perception, memory, record, notice, routine or role assignment, procedure
> state, testimony, expectation, or summary ancestry).

The package design:

- **Primary home: `03`** owns the conceptual temporal authority model (the six categories above plus
  the temporal firewall). It is the only foundation doc that can bind event/replay time, summary
  intervals, and boundary cadence without making time a UI-, scheduler-, or institution-only concern.
- **Constitutional hook: `02`** carries one new invariant in the truth-firewall / cognition-authority
  neighborhood, because a doctrine that is "constitutional" in prose but absent from the constitution
  fails to bind lower tiers — incoherent for this repository's authority model.
- **Targeted cross-references** in `04`, `05`, `07`, `08`, `10`, `12`, and `14` *apply* the doctrine
  to their surfaces without redefining it, plus an `00` index-map update and a `13` source note.

Foundation owns the *authority separation*. Foundation does **not** own tick size, calendar/date
syntax, duration units, scheduler data structures, fairness algorithms, UI clock rendering, exact
staleness thresholds, or the first-playable vocabulary for "morning", "last night", "late", "due",
or "season". Those route down (Out of Scope §6).

## 3. Deliverables

All deliverables are **proposed amendment substance**, to be ratified by the reassess /
constitutional-amendment process. None assigns an `INV-###`, authors final wording, or coins a
glossary identifier. The package spans nine foundation files (one constitutional hook, one primary
home, six cross-references, one index map, one source note). Documents `01`, `06`, `09`, and `11`
are verified to need **no change** (see Verification §V2).

| # | Target file | Change | Substance |
|---|---|---|---|
| D1 | `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` | New invariant in the truth-firewall / cognition-authority neighborhood (currently `INV-099`…`INV-111`), cross-referenced to the events/replay family. | Authoritative simulation time, event order, intervals, durations, and due effects may validate replay, ordering, action legality, scheduled consequences, and causal explanation. Cognition, routine selection, institutional procedure, embodied view models, speech interpretation, leads, and LOD promotion may use temporal facts **only** when those facts are available to the relevant holder through modeled channels. Deadline / lateness / staleness / "expected by now" / "yesterday" / "office closed" are not free truth labels — they are claims, procedure states, or holder/institution-known interpretations with provenance. The scheduler and replay clock may order and validate; they must not become cognition authority. One invariant-level hook only — no tick size, date syntax, duration unit, staleness window, or UI rendering. |
| D2 | `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` | New primary section at foundation altitude, after event anatomy / deterministic replay. **The single authoritative home.** | Define the conceptual temporal authority model: (1) authoritative event/replay time; (2) holder-known temporal claims; (3) institution-known procedural time; (4) routine / social rhythm; (5) freshness / staleness authority; (6) LOD and regional temporal summaries; (7) the temporal firewall (truth clock may validate; holder-known time may plan). Explicitly refuse lower-tier choices: no tick size, calendar/date syntax, duration unit, scheduler queue structure, UI clock display, exact stale-after value, or first-playable calendar vocabulary. |
| D3 | `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` | Inline addition / cross-reference near claim shape, memories, records, and staleness. | Temporal expressions and freshness/lateness labels must be represented as holder-known or artifact/institution-known temporal claims with provenance, distinguishing where it matters: when the claimed event occurred, when the holder acquired or last verified the claim, when a record was created/amended/read, and what staleness risk remains. The world clock cannot silently update a memory, record, lead, or notebook merely because time passed; change requires a modeled update, contradiction, verification, decay, or procedure. (Specializes `INV-026` and `INV-028` for the temporal case — an application pointer, not new doctrine.) |
| D4 | `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Short cross-reference near routines / planning. | Routines, jobs, duties, social appointments, and schedule-following must consume only actor-known, holder-known, or institution-known temporal premises (believed work time, remembered assignment, read notice, heard bell, inferred lateness from darkness, observed queue, held role obligation). An actor may not act because the scheduler read true office hours or a global calendar row said "work". A scheduler trigger may create a decision opportunity, but candidate generation and routine continuation must still pass through the actor-known transaction and ordinary validation. (Specializes `INV-035`, `INV-103`, and `INV-104` for the temporal case — an application pointer, not new doctrine.) |
| D5 | `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` | Inline doctrine hook near institutions / procedures / records. | Office hours, filing windows, due dates, lateness, queue aging, record expiration, notice lifecycle, wage/payment periods, case delays, sanction windows, and procedural aging are institution-known or artifact-backed states. A procedure may validate against authoritative time, but an institution's conclusion or action must arise from records, reports, role knowledge, office procedure, evidence thresholds, observed absence, or modeled delay. A debt becomes "late" only under a due rule, clock validation, and an institution/holder-facing record/procedure state with provenance — never as a free label. (Specializes `INV-049`, `INV-053`, and `INV-056` for the temporal case — an application pointer, not new doctrine.) |
| D6 | `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` | Inline addition near time controls and embodied / debug separation. | Time controls may advance authoritative event/replay time, but embodied views may render temporal facts only when the possessed actor could know or infer them through modeled channels. Debug may display exact simulation time, event order, hidden due effects, or omitted truth, but must be visibly non-diegetic. "You slept until morning", "the office is closed", "the payment is late", and "a lot happened while you were away" must be actor-known summaries, record-derived conclusions, or debug-only labels — not hidden-truth leakage. |
| D7 | `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` | Inline cross-reference near summary intervals / regional cadence / promotion. | Every LOD or boundary summary that later affects actor, institution, household, regional, or embodied behavior must preserve both temporal ancestry and information ancestry: interval covered, cadence/trigger, known-to-whom claims, public records/notices, rumor chains, role assignments, last-verification/staleness risk, and fidelity limits. A promoted actor or institution may inherit only temporal knowledge explicitly attributed through preserved ancestry — never the aggregate truth used to maintain the low-detail simulation. |
| D8 | `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` | Acceptance cross-reference alongside the existing mandatory proof cases — **doctrine only, no gate code.** | First playable is not complete unless temporal features demonstrate the firewall: a worker acting from believed work time / assignment rather than true global schedule; a closed/late/stale institutional state arising from procedure/record/attempt/notice; sleep/wait/travel advancing event time without granting omniscient knowledge; a stale record remaining stale until modeled update; and replay showing the difference between validator time and actor/institution-known temporal premises. Explicitly avoid specifying tick size, UI time display, exact day-part labels, or stale-after durations. |
| D9 | `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | Compact source note (added when the amendment lands) — a note, not doctrine. | Cite temporal-modeling / simulation-reporting sources as rationale for naming temporal authority categories and for refusing to pick a mechanism in foundation: instants/intervals/durations can be named without a Gregorian or wall-clock representation; simulation time and wall-clock time are distinct; ABM scheduling / time resolution belongs in model/execution detail; unclear temporal concepts cause semantic drift. The note must say these sources do **not** ratify a tick size, calendar syntax, duration unit, or scheduler implementation. |
| D10 | `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` | Clarification near the scheduler boundary / actor-known context / provenance classes, cross-referencing the new `03` section and new `02` invariant. | Raw simulation time, exact event order beyond what a holder can know, future scheduled completions, true office windows, true due states, and exact summary intervals are validator/debug truth unless source-backed for the holder. The holder-known version may be a perceived day-part, remembered appointment, read timestamp, routine assignment, public bell, posted hours, institutional queue state, due notice, stale record, or summary knowledge with ancestry. No new doctrine family — an explicit temporal case of the existing firewall. |
| D11 | `docs/0-foundation/00_FOUNDATION_INDEX.md` | Short index-map phrase — map only, no doctrine. | Note that `03` owns causal event/replay authority **and** temporal authority doctrine, with supporting cross-references to `02`, `04`, `05`, `07`, `08`, `10`, `12`, and `14`, so future readers find the primary home while the doctrine itself stays in the substantive files. |

D1 is the constitutional amendment proper; D2 is the authoritative doctrine home; D3–D10 apply the
doctrine at their surfaces; D11 keeps the index honest; D9 records provenance. None introduces a
`how`-level mechanism into foundation.

## 4. Invariants Alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-099` — Truth may validate actions, but truth may not plan them. | aligns / extends | D1 is the temporal specialization of `INV-099`: the clock/replay-time substrate may validate ordering and due effects (validate) but may not select goals, routines, or institutional conclusions (plan) unless source-backed for the holder. Same firewall surface, applied to time. |
| `INV-103` — The scheduler is not a cognition authority. | aligns / extends | D1, D4, and D10 reaffirm that a scheduler/replay clock may choose actor/time windows, advance deterministic time, and apply due completions, but may not construct proposals or routine continuation from raw temporal state — at the scheduler-boundary / actor-known-transaction surface. |
| `INV-102` — Cognition inputs require provenance. | aligns | D3 and D5 require temporal claims (event time, acquisition/verification time, record/amendment time, lateness/staleness) to carry provenance sufficient for replay and debug — the cognition-input and institutional-record surfaces. |
| `INV-110` — LOD and summary processes must preserve the firewall. | aligns | D7 sharpens `INV-110` for the temporal case: summary intervals/cadence promoted into later cognition must preserve both temporal and information ancestry at the LOD/regional-summary surface, so promoted actors/institutions inherit only attributed temporal knowledge. |
| `INV-028` — Staleness is not automatically corrected. | aligns | D3 specializes `INV-028` for time: the world clock cannot silently update a memory, record, lead, or notebook merely because time passed — change requires a modeled channel — at the belief/record-staleness surface. |
| `INV-035` — Routines are defeasible intentions. | aligns | D4 specializes `INV-035` ("a worker works because they believe it is work time"): routine/social-rhythm following must consume actor/holder/institution-known temporal premises, never a true global schedule, at the planning surface. |
| `INV-026` — Important beliefs need provenance. | aligns | D3 specializes `INV-026`, which already records acquisition time, believed event time, and staleness: temporal claims must carry the same provenance slots, at the claim/memory surface. |

No invariant is weakened or tensioned. The amendment is additive `what`-level doctrine: D1 adds one
new invariant (proposed, unnumbered) in the truth-firewall family; D2–D11 specialize and apply
existing doctrine. No `how`-level mechanism (tick, calendar, scheduler structure, UI rendering,
stale-after value) enters foundation.

## 5. Verification

- **V1 — Hole confirmed (done at authoring).** No `docs/0-foundation/` file defines a temporal
  authority model. A grep for `temporal authority`, `holder-known temporal`, and `procedural time`
  across `docs/0-foundation/` returned no matches. The adjacent firewall family is present and
  verified verbatim: `INV-099` (truth may validate, not plan), `INV-102` (cognition inputs require
  provenance), `INV-103` (scheduler is not a cognition authority), `INV-110` (LOD/summary preserve
  the firewall), `INV-111` (observer-only emergence evidence, highest existing number). `14` carries
  the Scheduler boundary section permitting "advance deterministic time" and "time acceleration".
  Re-verified against live `HEAD`, which equals the report's pinned `9c1203f`.
- **V2 — No-change documents confirmed.** `01` (charter already frames causality-first belief-
  bounded ordinary-life simulation), `06` (already makes hunger/fatigue/sleep/wages/travel/duration
  causal), `09` (seed-time routines/cadences already allowed as possibility-space machinery), and
  `11` (temporal utterances already structured speech-act claims behind validation) need no temporal
  doctrine; adding it would duplicate existing rules. These are deliberately excluded from D1–D11.
- **V3 — Ratification acceptance (on implementation).** The amendment is accepted only when: D1 adds
  a single falsifiable `what`-level invariant (no tick size, date syntax, duration unit, staleness
  window, or UI rendering); D2 adds the conceptual model and explicitly refuses the lower-tier
  choices; D3–D10 add doctrine-local cross-references that *apply* but do not *redefine* the model;
  D11 updates the index map only; and no foundation text introduces a temporal mechanism.
- **V4 — Boundary check.** Grep the amended foundation passages / newly added lines for mechanism
  tokens (tick size, specific date/calendar syntax, duration units, scheduler queue/data-structure
  names, UI clock-format strings, numeric stale-after values) — the amendment must add none of them,
  preserving the `what`/`how` layer boundary. A whole-file grep is not the proof surface because the
  live foundation already contains unrelated ordinary-language uses of "time", "schedule", and
  "stale".
- **V5 — Source-note follow-through (D9).** The temporal source note in `13` cites the temporal-
  modeling/simulation sources as rationale only and explicitly disclaims ratifying any mechanism.

## 6. Out of Scope

- **`INV-###` assignment, final constitutional wording, and any glossary identifier.** Owned by the
  reassess / constitutional-amendment process. This spec recommends substance and home only,
  preserving the report's identifier discipline. (A future temporal invariant would append after
  `INV-111` — likely `INV-112` — to avoid renumbering existing references, but this spec assigns no
  number.)
- **The eight routed themes from the originating verdict** — play experience / legibility; quantity /
  granularity / fungibility; bounded affect; learning / adaptation; authoring / compiler discipline;
  deterministic performance / fairness budgets; bias / social-harm practicality; staged
  incompleteness. The predecessor determination routes these below foundation; this pass found no new
  foundation-level gap beyond the settled temporal promotion (report §5, §6.8). They get no deliverable
  here.
- **Lower-tier temporal mechanism.** Minimal first-playable temporal vocabulary (day-parts,
  yesterday/last-night, due/lateness phrasing); calendar representation and duration units; exact
  staleness/freshness policy; temporal TUI surface budget; deadline/procedural-aging primitives; LOD
  temporal-equivalence fidelity for promotion; simultaneity / tie-breaking algorithms. All route to
  architecture / execution / reference (report §6 open questions).
- **Architecture / execution / reference cascade edits.** Specializing the temporal doctrine into
  `1-architecture`, `2-execution`, and the glossary is explicitly later, separate work, gated on this
  foundation amendment landing first.

## 7. Risks & Open Questions

- **R-A — Constitutional change requires sign-off.** Implementing D1 (and the D2–D11 foundation
  edits) edits the constitution and the foundation tier. This must not proceed without explicit owner
  approval; the spec stages the change, it does not authorize it.
- **R-B — Invariant placement and ordering.** Whether the new invariant lands immediately after
  `INV-111`, inside the firewall family, or elsewhere, and its exact ordering, is a reassess decision.
- **R-C — Cross-reference creep.** The package touches nine files; the risk is that doc-local
  cross-references in D3–D10 drift into duplicated or competing doctrine. Acceptance V3/V4 exist to
  keep `03` the single home and the cross-references short and application-only.
- **R-D — Mechanism leak into foundation.** The most likely failure mode is a well-meaning editor
  pinning a tick size, calendar vocabulary, or stale-after value into the foundation text. D2's
  explicit refusal clause and V4's boundary grep exist to forbid this at the doctrine level; the
  lower-tier open questions (§6) are where those choices belong.
- **R-E — Predecessor freshness string.** The predecessor determination report carries an older
  baseline commit string in its own provenance. That stale string was quarantined; this spec uses
  `9c1203f` (= live `HEAD`) as the target of record and is not derived from branch-name, default-
  branch, or code-search evidence.

## Outcome

Completed: 2026-06-14

The foundation temporal-authority amendment was ratified and landed by
`archive/tickets/0031FOUTEMAUT-001.md` in commit `50ecb03`. The implementation assigned the new
constitutional invariant as `INV-112` after `INV-111` to avoid renumbering existing invariant
references, added the primary temporal-authority doctrine section to foundation `03`, and added
application-only cross-references in foundation `04`, `05`, `07`, `08`, `10`, `12`, and `14`, with
the source note and index-map updates in `13` and `00`.

The signed owner approval in this spec header satisfied Risks §R-A before foundation files were
edited. The implementation preserved the spec's layer boundary: foundation now owns the authority
separation, but no tick size, calendar/date syntax, duration unit, scheduler queue structure, UI
clock display, numeric stale-after value, or first-playable calendar vocabulary was added.

Verification:

- `git diff --unified=0 -- docs/0-foundation/00_FOUNDATION_INDEX.md docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md | grep -nE '^\+[^+]' | grep -EI '[0-9]+ *(tick|ms|millisec|sec|min|minute|hour|day)s?\b|[0-9]{1,2}:[0-9]{2}|BinaryHeap|VecDeque|priority queue|stale[_-]?after *[:=]? *[0-9]'` produced no output.
- `grep -nE 'temporal authority|holder-known temporal|procedural time|temporal firewall' ...` across the eleven foundation targets produced hits in all eleven files.
- `git diff --check` passed before the ticket commit.
- Final closeout gates after spec archival/truthing: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, and `cargo test --workspace` all passed.

Final closeout also archives this spec to `archive/specs/` and records it in
`docs/4-specs/SPEC_LEDGER.md`. The provenance reports and manifest remain in `reports/` because the
spec cites them as the research source chain for this amendment.
