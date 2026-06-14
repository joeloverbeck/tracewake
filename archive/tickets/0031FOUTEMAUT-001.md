# 0031FOUTEMAUT-001: Land the temporal-authority doctrine amendment across foundation 02/03/04/05/07/08/10/12/13/14/00

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — constitutional/doctrine edits to `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`, `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`, `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`, `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`, `07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`, `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`, `10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`, `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`, `13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`, `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`, and `00_FOUNDATION_INDEX.md`. No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: explicit constitutional owner sign-off per spec 0031 §R-A — obtained in the spec header ("OWNER: I'm signing off on these amendments. Proceed."). This ticket documents the amendment; it enacts the foundation edits under that sign-off.

## Problem

The predecessor foundations-completeness campaign settled exactly one new theme for promotion to the foundation tier: **time / calendar / social rhythm**. The current foundation *gestures* at time everywhere — simulation time and deterministic ordering in `03`, acquisition/believed event time and staleness in `04`, routines and office hours in `05`/`07`, time controls and sleep summaries in `08`, LOD summary intervals in `10`, the scheduler/time-acceleration boundary in `14` — but no foundation file defines a single, named **temporal authority doctrine** that separates authoritative event/replay time from holder-known temporal claims, institution-known procedural time, routine/social rhythm, freshness/staleness, and LOD summary intervals. A grep of the live foundation confirms the hole: no `temporal authority`, `holder-known temporal`, or `procedural time` doctrine exists in `docs/0-foundation/`. Without that explicit split, the repository invites the relapse the campaign exists to prevent: the scheduler, calendar, or replay clock becoming an oracle for cognition, institutions, embodied views, or LOD promotion — because raw clock access does not *feel* like a secret physical fact. Spec 0031 (reassessed this session) confirms this is a genuine foundation-level hole and promotes a compact temporal authority doctrine as a cross-document mini-package: one new invariant (`02`), one primary doctrine home (`03`), targeted application cross-references (`04`/`05`/`07`/`08`/`10`/`14`), a first-playable acceptance cross-reference (`12`), a source note (`13`), and an index-map update (`00`). This ticket performs that eleven-deliverable amendment as one atomic constitutional edit.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`9c1203f` = current `HEAD`): `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` ends at `INV-111` (the `## 2026 hardening invariants` truth-firewall block is `INV-099`…`INV-110`; emergence acceptance is `INV-111`, added by archived spec 0026). No `INV-112+` exists. A new temporal invariant appended after `INV-111` is `INV-112`; inserting it mid-block would renumber the firewall family and every repo-wide reference — the placement/numbering decision (spec §R-B) is the constitutional process's recorded choice, made at ratification, not pre-baked here. All eleven target files (`00, 02, 03, 04, 05, 07, 08, 10, 12, 13, 14`) exist; `01, 06, 09, 11` are confirmed no-change (spec §V2).
2. Verified against spec 0031 (`specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md`) D1–D11 and §6: D1 → new invariant in `02`; D2 → primary temporal-authority section in `03` (after event anatomy / deterministic replay — both sections confirmed present in `03` at the section headers "Event anatomy" and "Deterministic replay"); D3–D7/D10 → application cross-references in `04`/`05`/`07`/`08`/`10`/`14`; D8 → first-playable acceptance doctrine in `12`; D9 → source note in `13`; D11 → index-map phrase in `00`. Lower-tier mechanism (tick size, calendar syntax, duration units, scheduler structures, UI clock rendering, exact staleness thresholds, first-playable calendar vocabulary), the eight routed themes, and the architecture/execution/reference cascade are explicitly out of scope (spec §6).
3. Shared boundary under audit: the `what`/`how` layer line between foundation doctrine and lower-tier mechanism. Foundation may carry the *authority separation* and the cross-references; it must commit to none of the mechanism — no tick value, date/clock format, duration unit, scheduler data structure, or numeric stale-after. The doctrine prose deliberately *refuses* these (the words "tick", "calendar", "duration" appear only in refusal clauses), so the boundary proof is a manual review plus a concrete-commitment grep, not a bare-word grep (see Verification Layers 2).
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-099` (truth may validate actions, but truth may not plan them) — D1 is its temporal specialization (the clock/replay substrate may validate ordering and due effects but may not plan unless source-backed); `INV-103` (the scheduler is not a cognition authority) — D1/D4/D10 reaffirm the scheduler may advance deterministic time and apply due completions but may not construct proposals from raw temporal state; `INV-102` (cognition inputs require provenance) — D3/D5 require temporal claims to carry provenance; `INV-110` (LOD and summary processes must preserve the firewall) — D7 requires promoted summaries to preserve temporal *and* information ancestry. The amendment also specializes `INV-026`/`INV-028` (D3) and `INV-035`/`INV-104` (D4). It is additive; no invariant is weakened or tensioned (spec §4).
5. Actor-knowledge / no-leak / deterministic-replay surface (the doctrine this amendment governs, enforced by deferred lower-tier surfaces, not by this doc edit): the temporal firewall is the truth-firewall (`INV-099`/`INV-100`/foundation `14`) applied to time — authoritative event/replay time may validate ordering, intervals, legality, replay, and due effects, but cognition/routine/institution/embodied/LOD use of temporal facts is permitted only when the fact entered the holder-known or institution-known context through a modeled channel. This ticket adds doctrine only; it introduces no code path, no leakage, and no nondeterminism. The enforcement surfaces (the scheduler boundary, actor-known context sealing, LOD ancestry preservation) remain `14`'s existing doctrine and later architecture/execution work — not this edit; the amendment must author no wording that licenses planning from raw clock/true office hours/true due states unless source-backed for the holder.

## Architecture Check

1. One atomic eleven-file diff is correct and overrides the default Split rule (spec §2: "a cross-document mini-package" with a single home; spec §V3 accepts the package only when D1+D2+…+D11 all hold together). The edits are not independently landable as separate merged tickets: the D3–D11 application cross-references all point at D1 (the new invariant) and D2 (the `03` doctrine section), so a split would either dangle those cross-references or land the constitution in a state the spec's own acceptance gate rejects, and the whole package shares one constitutional owner sign-off (spec §R-A, header). This is the exact shape precedent of `archive/tickets/0026FOUEMEEVI-001.md` (which landed its three-file foundation amendment atomically) and the doc-doctrine amendment-batch pattern (`decomposition-patterns.md` §Doc-doctrine amendment batch). Splitting would license a doctrinally-incoherent intermediate constitution — unsafe, not merely inconvenient (atomic-cutover exception, `decomposition-patterns.md` §Sizing rules).
2. No backwards-compatibility aliasing/shims: this is additive `what`-level doctrine. `03` is the single authoritative home; `04`/`05`/`07`/`08`/`10`/`12`/`14` carry short application pointers (D3/D4/D5 explicitly specialize named existing invariants — see spec D-rows), `00` an index map, `13` a source note. No duplicated doctrine, no relocated mechanism, no compatibility layer.

## Verification Layers

1. Spec §V3 ratification acceptance → invariants alignment check: D1 adds a single falsifiable `what`-level invariant (no tick size, date syntax, duration unit, staleness window, or UI rendering); D2 adds the conceptual model (the six temporal categories + the temporal firewall) and explicitly refuses the lower-tier choices; D3–D10 add doctrine-local cross-references that *apply* but do not *redefine* the model; D11 updates the index map only; no foundation text introduces a temporal mechanism.
2. Spec §V4 layer-boundary (the `what`/`how` line) → codebase grep-proof + manual review: the amended lines commit to no concrete mechanism — no numeric duration/tick value, no clock/date format string, no named scheduler data structure, no numeric stale-after. Because the doctrine prose legitimately contains the words "tick", "calendar", and "duration" inside refusal clauses, the grep targets concrete-commitment patterns and the manual review is the primary boundary surface (mirrors the `0026` deviation: whole-file/bare-word grep false-positives on doctrine prose).
3. Temporal firewall doctrine (`INV-099`/`INV-103`/`INV-110`, foundation `14`) → manual review (epistemic-leakage audit): the new wording across all eleven files forbids planning, inferring, rendering embodied knowledge, or producing institution/procedure conclusions from authoritative time unless the temporal fact entered the holder/institution-known context through a modeled channel; it weakens no prior firewall invariant.

## What to Change

### 1. D1 — new invariant in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`

Add one new constitutional invariant in the truth-firewall / cognition-authority neighborhood (the `INV-099`…`INV-111` block), cross-referenced to the events/replay family. Substance: authoritative simulation time, event order, intervals, durations, and due effects may validate replay, ordering, action legality, scheduled consequences, and causal explanation; cognition, routine selection, institutional procedure, embodied view models, speech interpretation, leads, and LOD promotion may use temporal facts only when available to the relevant holder through modeled channels; deadline/lateness/staleness/"expected by now"/"yesterday"/"office closed" are claims, procedure states, or holder/institution-known interpretations with provenance, not free truth labels; the scheduler and replay clock may order and validate but must not become cognition authority. **Recorded-choice obligation (spec §R-B):** assign the `INV-###` number and author the final ratified wording at sign-off time; record the placement rationale (append as `INV-112` after the firewall family vs. insert with its renumbering blast radius). One invariant-level hook only — no tick size, date syntax, duration unit, staleness window, or UI rendering.

### 2. D2 — primary temporal-authority section in `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`

Add one new section at foundation altitude, after event anatomy / deterministic replay. Define the conceptual model: (1) authoritative event/replay time; (2) holder-known temporal claims; (3) institution-known procedural time; (4) routine/social rhythm; (5) freshness/staleness authority; (6) LOD and regional temporal summaries; (7) the temporal firewall (truth clock may validate; holder-known time may plan). Explicitly refuse lower-tier choices: no tick size, calendar/date syntax, duration unit, scheduler queue structure, UI clock display, exact stale-after value, or first-playable calendar vocabulary.

### 3. D3 — temporal claims / staleness cross-reference in `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`

Add a compact inline addition near claim shape / memories / records / staleness: temporal expressions and freshness/lateness labels are holder-known or artifact/institution-known temporal claims with provenance, distinguishing claimed-event time, acquisition/last-verification time, record create/amend/read time, and staleness risk; the world clock cannot silently update a memory, record, lead, or notebook merely because time passed (specializes `INV-026`/`INV-028`).

### 4. D4 — routines / social rhythm cross-reference in `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`

Add a short cross-reference near routines/planning: routines, jobs, duties, appointments, and schedule-following consume only actor/holder/institution-known temporal premises; an actor may not act because the scheduler read true office hours or a global calendar row said "work"; a scheduler trigger may create a decision opportunity, but candidate generation and routine continuation pass through the actor-known transaction and ordinary validation (specializes `INV-035`/`INV-103`/`INV-104`).

### 5. D5 — procedural time cross-reference in `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`

Add an inline doctrine hook near institutions/procedures/records: office hours, filing windows, due dates, lateness, queue aging, record expiration, notice lifecycle, wage/payment periods, case delays, sanction windows, and procedural aging are institution-known or artifact-backed states; a procedure may validate against authoritative time, but an institution's conclusion or action arises from records, reports, role knowledge, office procedure, evidence thresholds, observed absence, or modeled delay; a debt becomes "late" only under a due rule, clock validation, and a record/procedure state with provenance (specializes `INV-049`/`INV-053`/`INV-056`).

### 6. D6 — embodied temporal surfaces cross-reference in `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`

Add a compact inline addition near time controls and embodied/debug separation: time controls may advance authoritative event/replay time, but embodied views may render temporal facts only when the possessed actor could know or infer them through modeled channels; debug may show exact simulation time/event order/hidden due effects but must be visibly non-diegetic; "you slept until morning", "the office is closed", "the payment is late", "a lot happened while you were away" are actor-known summaries, record-derived conclusions, or debug-only labels, not hidden-truth leakage.

### 7. D7 — LOD temporal/information ancestry cross-reference in `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`

Add an inline cross-reference near summary intervals / regional cadence / promotion: every LOD or boundary summary that later affects actor/institution/household/regional/embodied behavior must preserve both temporal and information ancestry (interval covered, cadence/trigger, known-to-whom claims, public records/notices, rumor chains, role assignments, last-verification/staleness risk, fidelity limits); a promoted actor/institution inherits only temporal knowledge explicitly attributed through preserved ancestry, never the aggregate truth used to maintain the low-detail simulation (specializes `INV-110`).

### 8. D8 — first-playable acceptance cross-reference in `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`

Add an acceptance cross-reference alongside the existing mandatory proof cases — doctrine only, no gate code: first playable is not complete unless temporal features demonstrate the firewall (a worker acting from believed work time/assignment rather than true global schedule; a closed/late/stale institutional state arising from procedure/record/attempt/notice; sleep/wait/travel advancing event time without omniscient knowledge; a stale record remaining stale until modeled update; replay showing validator time vs. actor/institution-known temporal premises). Avoid specifying tick size, UI time display, exact day-part labels, or stale-after durations.

### 9. D9 — temporal source note in `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`

Add a compact source note (a note, not doctrine): cite temporal-modeling / simulation-reporting sources as rationale for naming temporal authority categories and refusing to pick a mechanism (instants/intervals/durations namable without a Gregorian/wall-clock representation; simulation time vs. wall-clock time distinct; ABM scheduling/time resolution belongs in model/execution detail; unclear temporal concepts cause semantic drift). State the sources do not ratify a tick size, calendar syntax, duration unit, or scheduler implementation.

### 10. D10 — temporal firewall clarification in `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`

Add a clarification near the scheduler boundary / actor-known context / provenance classes, cross-referencing the new `03` section and new `02` invariant: raw simulation time, exact event order beyond what a holder can know, future scheduled completions, true office windows, true due states, and exact summary intervals are validator/debug truth unless source-backed for the holder; the holder-known version may be a perceived day-part, remembered appointment, read timestamp, routine assignment, public bell, posted hours, institutional queue state, due notice, stale record, or summary knowledge with ancestry. No new doctrine family — an explicit temporal case of the existing firewall.

### 11. D11 — index-map phrase in `docs/0-foundation/00_FOUNDATION_INDEX.md`

Add a short index-map phrase (map only, no doctrine): note that `03` owns causal event/replay authority *and* temporal authority doctrine, with supporting cross-references to `02`, `04`, `05`, `07`, `08`, `10`, `12`, and `14`.

## Files to Touch

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (modify) — D1
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` (modify) — D2
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` (modify) — D3
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` (modify) — D4
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` (modify) — D5
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` (modify) — D6
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` (modify) — D7
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` (modify) — D8
- `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` (modify) — D9
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` (modify) — D10
- `docs/0-foundation/00_FOUNDATION_INDEX.md` (modify) — D11

## Out of Scope

- **`INV-###` assignment, final constitutional wording, and any glossary identifier** — owned by the reassess / constitutional-amendment process (spec §6); this ticket carries the substance and the recorded-choice obligation, not the ratified number/wording.
- **The constitutional sign-off itself (spec §R-A)** — a human owner act (obtained in the spec header); this ticket's execution precondition, not its deliverable.
- **No-change foundation files** `01_PROJECT_CHARTER.md`, `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`, `09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`, `11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — verified no-change (spec §V2); not edited.
- **Lower-tier temporal mechanism** — tick size, calendar/date syntax, duration units, scheduler data structures, UI clock rendering, exact staleness thresholds, first-playable calendar vocabulary, simultaneity/tie-break algorithms (spec §6 open questions). Route to architecture/execution/reference.
- **The eight routed themes** (spec §5) and the **architecture/execution/reference cascade** specializing this doctrine — later, separate work gated on this foundation amendment landing first.

## Acceptance Criteria

### Tests That Must Pass

1. **V4 boundary check** — `git diff --unified=0 -- docs/0-foundation/{00,02,03,04,05,07,08,10,12,13,14}_*.md | grep -nE '^\+[^+]' | grep -EI '[0-9]+ *(tick|ms|millisec|sec|min|minute|hour|day)s?\b|[0-9]{1,2}:[0-9]{2}|BinaryHeap|VecDeque|priority queue|stale[_-]?after *[:=]? *[0-9]'` returns no match in the added lines (foundation carries the authority separation, not a concrete mechanism commitment), confirmed by a manual review that every "tick"/"calendar"/"duration" occurrence sits in a *refusal* clause.
2. **Landing check** — the new temporal invariant exists in `02` with an assigned `INV-###`; a temporal-authority section exists in `03`; application cross-references exist in `04`/`05`/`07`/`08`/`10`/`14`; first-playable temporal acceptance doctrine exists in `12`; a temporal source note exists in `13`; the index phrase exists in `00`. All eleven deliverables resolve.
3. **Invariants alignment review** — D1 is a single falsifiable `what`-level invariant (no procedure/fixture/mechanism); no existing invariant (`INV-001`…`INV-111`) is weakened or tensioned.

### Invariants

1. The amended foundation passages contain the temporal authority doctrine and the application cross-references but commit to no lower-tier mechanism — the `what`/`how` layer boundary holds.
2. The new invariant forbids cognition/routine/institution/embodied/LOD use of authoritative time unless the temporal fact entered the holder/institution-known context through a modeled channel, and ties temporal claims to provenance — additive, weakening no prior invariant.

## Test Plan

### New/Modified Tests

1. `None — documentation-only constitutional-amendment ticket; verification is command-based (boundary + landing greps) plus an invariants-alignment manual review. No crate/code or fixture changes, so existing pipeline coverage is unaffected (doc-doctrine amendment batch — code Step 5/6 machinery is N/A: no Rust symbols, no schema-shape change, no cargo tests).`

### Commands

1. `git diff --unified=0 -- docs/0-foundation/00_FOUNDATION_INDEX.md docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md | grep -nE '^\+[^+]' | grep -EI '[0-9]+ *(tick|ms|millisec|sec|min|minute|hour|day)s?\b|[0-9]{1,2}:[0-9]{2}|BinaryHeap|VecDeque|priority queue|stale[_-]?after *[:=]? *[0-9]'` — must show no concrete-mechanism match in the added lines (V4).
2. `grep -nE 'temporal authority|holder-known temporal|procedural time|temporal firewall' docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md docs/0-foundation/00_FOUNDATION_INDEX.md` — confirms the doctrine and cross-references landed across the eleven files (landing check).
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected and is not the verification boundary for a foundation-doc edit; the boundary is the two greps above plus the invariants-alignment manual review.`

## Outcome

Completed: 2026-06-14

Implemented the signed-off foundation temporal-authority doctrine amendment as a documentation-only constitutional package. The requested path `tickets/0032FOUTEMAUT-001.md` did not exist in the live checkout; the only matching live ticket/spec family was `tickets/0031FOUTEMAUT-001.md` plus `specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md`, so this outcome records that selector correction.

What changed:

- Added `INV-112` as the temporal-authority invariant in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`, appended after `INV-111` to avoid renumbering existing invariant references.
- Added the primary `Temporal authority` doctrine section in `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`, including authoritative event/replay time, holder-known temporal claims, procedural time, routine/social rhythm, freshness/staleness, LOD summaries, and the temporal firewall.
- Added application-only cross-references in `04`, `05`, `07`, `08`, `10`, `12`, and `14`, plus the source-note and index-map updates in `13` and `00`.

Deviations from the original plan:

- None to the amendment scope. The ticket's own out-of-scope Rust pipeline remains not run for this documentation-only ticket. The exact selector in the user request was corrected from nonexistent `0032FOUTEMAUT-001` to live `0031FOUTEMAUT-001`.

Verification results:

- `git diff --unified=0 -- docs/0-foundation/00_FOUNDATION_INDEX.md docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md | grep -nE '^\+[^+]' | grep -EI '[0-9]+ *(tick|ms|millisec|sec|min|minute|hour|day)s?\b|[0-9]{1,2}:[0-9]{2}|BinaryHeap|VecDeque|priority queue|stale[_-]?after *[:=]? *[0-9]'` produced no output, proving no added-line concrete mechanism commitment under the ticket's V4 check.
- `grep -nE 'temporal authority|holder-known temporal|procedural time|temporal firewall' ...` across the eleven target files produced hits in all eleven files.
- `git diff --check` passed.
- Manual invariants review: the new `INV-112` and application hooks are additive `what`-level doctrine, preserve the truth-firewall and scheduler-boundary rules, and do not weaken existing `INV-001` through `INV-111`.
