# Research brief — Foundations completeness determination (verdict-on-foundations triage)

> **You are ChatGPT-Pro, Session 2.** This prompt is self-contained. Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a requirement
> impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-14_d7fc746.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first; agents act from
partial belief and every event leaves a replayable trace). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; **earlier tiers
govern later ones.** Fetch every file from commit **`d7fc746`** (verified repo `HEAD`) — the
manifest reflects exactly that tree.

**The seed is both fetchable and reproduced inline.** The document that triggered this task,
`reports/verdict-on-foundations.md`, is tracked at `d7fc746` and present in the manifest; it is also
reproduced verbatim in **Appendix A** for convenience. Read either — they are identical; Appendix A
is authoritative if they ever diverge.

**This brief is a delta, not a cold start.** A prior, structurally identical campaign already ran:

- `archive/reports/foundation-amendment-research-brief.md` + `archive/reports/foundation-amendment-research-report.md`
  — a determination that evaluated **seven** candidate themes drawn from the `0006`–`0025`
  *hardening / anti-contamination* campaign (provenance sufficiency, memory freshness,
  believed-access affordances, single-charge derived accounting, emergence-as-evidence,
  falsifiability/anti-vacuity, acceptance-evidence/fingerprint honesty).
- `archive/reports/foundation-amendment-lower-tier-routing.md` — the routing memo that recorded where each
  non-foundation theme was handed off.
- That determination promoted **exactly one** theme to the constitution (emergence-as-evidence,
  ratified as `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`),
  routed the other six below the foundation, and drove the **now-complete** lower-tier realignment
  specs `0027` (architecture), `0028` (execution), `0029` (reference), `0030` (specs). Their
  research outputs are the four `archive/reports/*-tier-alignment-research-report.md` files.

This pass is the **parallel determination** seeded by a *different* input. `verdict-on-foundations.md`
raises **nine design-completeness themes** (play legibility, time/calendar/social rhythm,
quantity/granularity/fungibility, bounded affect, learning/adaptation, authoring/compiler
discipline, deterministic performance/fairness budgets, bias/social-harm practicality, and
staged-incompleteness as an escape valve from over-constitutionalization). These are largely
**orthogonal** to the prior seven. The freshest, most-specific seed for *this* pass is Appendix A;
the prior campaign's report + routing memo are the **method template and the do-not-re-litigate
boundary**.

## 2. Read in full (authority order)

Read these before producing. Order is by authority tier.

**Primary — the assessment target (the foundation tier in full; completeness is measured here):**

```
docs/README.md — the authority ordering and the layering rule; this IS the tier-fit test you apply.
docs/0-foundation/00_FOUNDATION_INDEX.md — what the foundation currently claims to cover (the map).
docs/0-foundation/01_PROJECT_CHARTER.md — the ambition and the explicitly-refused wrong products.
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md — INV-001…INV-NNN; the non-negotiable contract every recommendation must respect.
docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md — event-sourced causality / replay spine.
docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md — epistemics; touches learning/memory-freshness themes.
docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md — needs/motives/routines; current home of any affect & learning language.
docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md — property/custody/survival; the granularity/fungibility theme lands near here.
docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md — institutions/fallibility; current home of bias/social-harm language.
docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md — TUI-first doctrine; the play-legibility theme lands near here.
docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md — authored possibility vs. forbidden outcome chains; the authoring/compiler theme extends this.
docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md — LOD-as-ontology, long simulation; touches time-cadence and perf/fairness themes.
docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md — language boundary; bounds the play/affect/learning themes against LLM non-authority.
docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md — first-proof scope; current home of any staged-incompleteness language.
docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md — how foundation decisions were sourced (the precedent for citing research).
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md — the truth firewall; every theme must be tested against "truth may validate, never plan."
reports/verdict-on-foundations.md — the SEED; the nine themes this determination triages (also reproduced verbatim in Appendix A).
```

**Delta / current-coverage (read so this pass is a delta and respects settled work):**

```
archive/reports/foundation-amendment-research-report.md — the SEVEN already-adjudicated themes and the method to reuse; do NOT re-open these.
archive/reports/foundation-amendment-lower-tier-routing.md — the routing precedent (target-doc mapping, open questions) your forward-routing appendix should mirror.
archive/reports/architecture-tier-alignment-research-report.md — what the architecture tier just absorbed (current coverage signal).
archive/reports/execution-tier-alignment-research-report.md — what the execution tier just absorbed.
archive/reports/reference-tier-alignment-research-report.md — what the reference tier just absorbed.
archive/reports/specs-tier-alignment-research-report.md — what the specs tier just absorbed; note the specs tier is NOT an amendment target.
archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md — the FORM a successful foundation promotion takes (use as a shape model, not as scope).
```

**Boundary-awareness (read to place findings at the right tier — NOT conformance targets; do not audit or "correct" these docs):**

```
docs/1-architecture/* — the subsystem contracts; candidate home for themes that are mechanism, not constitution.
docs/2-execution/* — gates and proof obligations; candidate home for perf/fairness budgets and authoring/compiler enforcement.
docs/3-reference/01_DESIGN_RISK_REGISTER.md and docs/3-reference/02_GLOSSARY.md — candidate home for watch-risks and canonical terms.
docs/4-specs/SPEC_LEDGER.md — the specs-tier posture; confirms specs is not an amendment target.
```

(Appendix A reproduces `reports/verdict-on-foundations.md` verbatim at the end of this brief; it is also fetchable at the path above.)

## 3. Settled intentions (final — these are decisions, not options)

1. **Adjudicate the verdict's nine themes.** Evaluate each of Appendix A's nine themes individually
   for *genuine necessity* and *tier placement*. Each gets a verdict: **promote** (and to which
   tier/doc), **route** (to a lower tier/doc), or **reject** (nice-to-have / already-covered /
   out-of-scope), each with a one-line basis.
2. **You MAY surface additional genuine holes** discovered through deep research — but place them in
   a **clearly separated section** ("Newly-surfaced candidates") so they never masquerade as the
   verdict's own findings. Apply the same per-theme verdict treatment.
3. **Assess current coverage first, per theme.** Several themes are *partly present already* — e.g.
   affect/motive language in foundation `05`, bias/fallibility in `07`, staged scope in `12`,
   memory/staleness in `04`. Do not declare a gap before showing what the live docs already own and
   why it is or isn't sufficient. "Validate what's present; don't assume every theme is a hole."
4. **The "belongs in the foundation" bar is derived, not pre-set.** Decide each theme's home by
   applying the docs' own authority-tier definitions (`docs/README.md` + each tier's stated purpose)
   and the project's "only what we truly need, not nice-to-have" discipline. Argue the bar per
   theme; the constitution holds only the non-negotiable contract, mechanism belongs in
   architecture, gates/proof in execution, watch-risks/terms in reference.
5. **Candidate homes span all five tiers (0→4).** A theme may belong in foundation, architecture,
   execution, reference, or as a future spec — or nowhere.
6. **Deliver substance + home, not ratified text.** For each promoted/routed theme, state *what
   doctrine the target doc must own* (in your own prose, at that tier's altitude) and *which file*
   it lands in. **Do not** write paste-ready final wording and **do not** invent identifiers
   (`INV-###`, gate codes, glossary entries) — coining those remains the repo's own reassess/amend
   process.
7. **Do not re-open the settled seven.** The prior campaign's seven themes (provenance sufficiency,
   memory freshness, believed-access, single-charge accounting, emergence-as-evidence,
   falsifiability/anti-vacuity, acceptance-evidence honesty) are adjudicated and their realignments
   are complete. If one of the verdict's nine overlaps a settled theme, note the overlap and defer
   to the settled disposition rather than re-litigating it.
8. **This determination feeds later per-tier sessions.** The user will run separate per-tier
   research sessions afterward (one each for any tier that receives findings). Your forward-routing
   appendix is the hand-off they inherit — make it ordered and specific (target doc(s) + the lesson
   to encode), mirroring `archive/reports/foundation-amendment-lower-tier-routing.md`.

> assumption: the deliverable filename is
> `reports/foundations-completeness-determination-research-report.md` and the brief filename is
> `reports/foundations-completeness-determination-research-brief.md`. Rename if you prefer; the
> content requirements are unchanged.

## 4. The task

This is a **foundational / doc-overhaul determination**. Decide what — if anything — from the
verdict's nine themes (plus any genuine holes your research surfaces) the Tracewake docs *truly
need*, and at which authority tier each accepted item belongs. The output is a determination +
routing report, not a ratified amendment: it tells the maintainer which themes to promote/route/
reject, where each lands, and what each target doc must own — without writing the doctrine text
itself. It is the seed for the maintainer's subsequent per-tier amendment sessions.

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2. **Research online as
deeply as needed** — comparable living-world / agent-based social simulations, symbolic-epistemic
agent architectures, event-sourced simulation design, game-design doctrine on legibility under
uncertainty, temporal-ontology and time-modeling literature, affect/emotion modeling that avoids
"meter puppets," content-as-code static-analysis practice, deterministic scheduling/fairness in
large agent sims, and bias-modeling methodology — wherever it sharpens a theme's verdict or its
tier placement. **Cite every external source that shapes a decision.** The deep research is your
job; the brief only commissions it.

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every product-behavior
  recommendation must satisfy it; a genuine divergence requires amending an invariant first, never
  designing against it silently. You recommend *where doctrine should live* — you do not amend.
- **Authority order:** if execution conflicts with architecture or foundation, execution is wrong;
  if implementation is more convenient than the accepted gates, implementation is wrong. Place each
  theme at the highest tier its nature *requires* and no higher.
- **The truth firewall is the spine:** *truth may validate actions, but truth may not plan them.*
  Test every theme against it — especially learning (must not become hidden-truth caching), affect
  (pressure, not omniscient signal), and play legibility (must not leak truth into the view).
- **Anti-contamination:** no simulation fact may be born from prose; preserve event-sourced
  causality, subjective epistemics, ordinary agents, possession parity, fallible institutions,
  validation/replay.
- **No nice-to-have in the foundation.** The constitution is small and brutal by design; the
  maintainer's explicit worry is foundation bloat. Default any uncertain theme *down* a tier or to
  rejection, not up.
- **No backwards-compatibility shims or alias paths** in any recommendation.

## 7. Deliverable specification

Produce **one** downloadable markdown document — an **analysis / recommendation report** (NOT a
numbered `docs/4-specs/` spec; the spec numbering/ledger rules do **not** apply):

- **`reports/foundations-completeness-determination-research-report.md`** — **new** file.

Required structure:

1. **Disposition table** — one row per theme: theme → verdict (**promote** / **route** / **reject**)
   → target tier + target doc(s) (or "—" if rejected) → one-line basis. Include the nine verdict
   themes; list newly-surfaced candidates in their own block within the same table or a sibling
   table, clearly labeled.
2. **Method & provenance ledger** — how you evaluated (tier-fit test, current-coverage check,
   research sources consulted), pinned to commit `d7fc746`; the seed is `reports/verdict-on-foundations.md`
   (also reproduced as Appendix A of the brief).
3. **Per-theme sections** — one per theme, in this shape: *driver* (what the verdict/research
   argues) → *current coverage* (what live docs already own, with file references) → *tier-fit
   verdict* (which tier its nature requires, and why — derived from the authority definitions) →
   *recommendation* (what the target doc must own, in your prose at that tier's altitude; which file;
   new section / addition / correction). No paste-ready wording, no invented identifiers.
4. **Newly-surfaced candidates** — clearly separated; same per-theme treatment; explicitly marked as
   *not from the verdict*.
5. **Forward-routing appendix** — for every promoted/routed theme, the ordered hand-off to the
   later per-tier session that will encode it (target tier → target doc(s) → lesson to encode),
   mirroring `archive/reports/foundation-amendment-lower-tier-routing.md`. Note any theme whose proper home
   is a *future implementation spec* rather than a tier amendment.
6. **Open questions** — owner decisions you cannot settle from the docs (carry, don't invent).
7. **References** — every external source cited.

**Determination posture (always-produce, verdict embedded).** This report is produced
**unconditionally** — even if your verdict is that *nothing* from the verdict warrants a foundation
change. A clean outcome is a valuable result: in that case the report still documents, per theme,
why the live docs already suffice (or why the item belongs below the foundation / is nice-to-have).
The value survives a negative verdict because it locks the reasoning for the maintainer.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

## 8. Self-check (run before returning)

- [ ] All nine verdict themes are adjudicated with an explicit verdict and a current-coverage
      assessment grounded in named live docs.
- [ ] Newly-surfaced candidates (if any) are in a clearly separated section and never blended with
      the verdict's nine.
- [ ] None of the settled seven prior-campaign themes is re-litigated; overlaps defer to the settled
      disposition.
- [ ] No paste-ready doctrine text and no invented identifiers (`INV-###`, gate codes, glossary
      entries) appear.
- [ ] Every theme placed at the highest tier its nature *requires* and no higher; the foundation
      receives only non-negotiable-contract material.
- [ ] Every external claim that shaped a decision is cited.
- [ ] The forward-routing appendix gives an ordered, specific hand-off per accepted theme.
- [ ] The deliverable set matches §7 exactly (one new report).
- [ ] Commit `d7fc746` contains every file named in §2 (including the seed
      `reports/verdict-on-foundations.md`, also reproduced in Appendix A).

---

## Appendix A — `verdict-on-foundations.md` (verbatim seed)

> This is the seed document the determination triages, reproduced verbatim for convenience; it is
> also fetchable at `reports/verdict-on-foundations.md` (commit `d7fc746`). The nine numbered themes
> below are the primary subjects of §3.

---

## **What the ambition really is**

Tracewake’s center is not theft, village life, TUI, or even emergence. Those are proof surfaces. The real ambition is:

Build a world where events matter because they leave traces, traces become claims, claims become beliefs or records, beliefs become actions, actions create further traces, and replay can explain the whole wake.

That is already visible in the charter: ordinary agents live through needs, routines, beliefs, memory, records, institutions, households, traces, work, travel, weather, regional pressures, and long histories, while the human only temporarily controls one ordinary actor rather than becoming a privileged world entity.

The project is also defined by what it refuses. The foundation explicitly rejects the easy wrong product: quest engine, player-centered mystery sim, omniscient investigation UI, LLM chatbot world, current-state toy, giant prop-filled region, TUI-less research prototype, and “story director wearing emergent vocabulary.” Its intended path is narrower: small playable village, ordinary life, subjective belief, event-sourced causality, fallible social machinery, actor-filtered TUI, replayable explanation, then honest expansion.

The most important doctrine is the truth firewall:

**Truth may validate actions, but truth may not plan them.**

That rule is not cosmetic. It is the spine of the whole project. The docs correctly identify that plausible-looking agents are worthless if they find food, work, danger, culprits, routes, witnesses, or opportunities by consulting hidden truth.

## **The best thing in these docs**

The foundation has finally converged on the right four authorities:

1. event authority;
2. epistemic authority;
3. action authority;
4. cognition authority.

The last one is the late but necessary correction: action proposal, goal selection, planning, routine selection, speech interpretation, and embodied view rendering must be fed only by sealed actor-known context.

That addition is what keeps Tracewake from becoming a fake-emergent sim. Without it, the scheduler, planner, TUI, LLM surface, institution layer, or test fixtures will all eventually “help” by reading true state and explaining it afterward. That is exactly the kind of contamination your mutation/remediation reports were fighting.

## **What the first playable is actually proving**

The missing-property/theft proof is not “make a mystery.” It is a compact stress test for the entire ontology:

* physical custody;
* expectation;
* absence-as-evidence;
* partial perception;
* ambiguous traces;
* gossip/testimony;
* lying;
* records;
* institutional fallibility;
* wrong suspicion;
* no-human operation;
* possession without knowledge transfer;
* deterministic replay.

The first playable scope says the first serious target is a small, inspectable, TUI-playable ordinary-life village whose people can be wrong for the right reasons; the missing-property case is only the first proof that physical state, belief, memory, traces, speech, records, households, institutions, TUI view models, possession, and replay form one causal system.

That is the right canonical proof. It is boring in the exact way it needs to be. If missing coins cannot become a wrong suspicion through modeled channels, then monsters, wars, cults, caravans, and courts will all be fake.

## **Do I think you’ve missed something?**

You have not missed the core. The foundation covers the main constitutional axes: event sourcing, epistemics, actor-known cognition, possession parity, LLM non-authority, no scripting, institutions, records, ordinary life, LOD, and first-playable gates.

But I do think several things are still underpowered or under-named. Not necessarily absent — many are implied — but I would give them sharper doctrine before implementation keeps expanding.

## **1. The missing play experience contract**

The docs are excellent at preventing the wrong product. They are less explicit about what makes the right product compelling moment to moment.

You say the TUI is first, playable, actor-filtered, testable, and not a debug shell. Good. But the acceptance gates mostly prove correctness: reachability, no leakage, replay, actor filtering, no-script compliance, no-human operation.

What is still missing is a doctrine for **legible pleasure**:

* What should a player enjoy doing for thirty minutes?
* What is the satisfying loop when there is no quest objective?
* How much friction is acceptable before “actor-bounded realism” becomes tedium?
* What makes a lead card useful rather than bureaucratic?
* What is the TUI’s information hierarchy under epistemic uncertainty?
* How does the game surface “you are ignorant” without making the player feel blind or punished?

My strong recommendation: add a future foundation or reference note called something like **Embodied Play Experience and Legibility Doctrine**. It should not weaken the simulation. It should define how the human can inhabit actor-limited knowledge without drowning in it.

Otherwise you risk building a perfect audit machine that is more admirable than playable.

## **2. A sharper time/calendar/social rhythm doctrine**

Time is everywhere in the docs — routines, office hours, staleness, fatigue, work, travel, long simulation — but I do not see a foundation-level temporal ontology. The system needs one.

Not implementation details like tick size. Doctrine.

Tracewake needs to distinguish:

* physical time;
* actor-perceived time;
* institutional time;
* routine windows;
* social rhythm;
* deadlines;
* lateness;
* “expected by now” thresholds;
* stale record intervals;
* memory aging;
* seasonal/regional cadence;
* LOD summary intervals.

This matters because a huge amount of ordinary life is temporal expectation. A worker absent at 08:00, a shop closed at noon, a debt unpaid after three days, a rumor stale after a week, a witness remembering “last night” rather than tick 913 — all of that needs a consistent conceptual model.

The scale document says the first village should run for days without human input and later support long histories, demography, stale records, and regional processes, but I would still add a dedicated time doctrine before the architecture hardens around ad hoc ticks.

## **3. A quantity/granularity doctrine for matter, money, food, stock, and services**

The property doctrine is strong: ownership, custody, access, control, proof, and belief are separated. The first scope also rightly requires money custody, food under custody, wages/promises, buying, stealing, storing, hiding, payment, and refusal.

But the foundation should probably say more about **granularity**:

* When is money a physical object versus an abstract balance?
* When is food an item versus stock/service capacity?
* When is “bread” countable, divisible, spoilable, shared, reserved, or consumed?
* When does abstraction preserve custody sufficiently?
* How do quantities leave traces?
* How do partial transfers, mixed ownership, pooled household goods, and fungible goods work?

This is not pedantry. Missing-property gameplay will break fast if the system cannot reason about fungibility, partial custody, household stock, and proof of identity for non-unique goods.

I would add a “Material Granularity and Fungibility” architecture contract at minimum.

## **4. A bounded affect/emotion doctrine**

The docs mention fear, shame, loyalty, affection, greed, duty, pride, resentment, social caution, and relationships; agents are explicitly not supposed to be meter puppets.

But I would make affect more explicit, because this project’s best future stories depend on it.

Not a big mood-sim. A bounded doctrine:

* emotions are not truth;
* emotions are pressures and memory modifiers;
* fear affects risk and avoidance;
* shame affects concealment and confession;
* anger affects accusation and retaliation;
* guilt affects lying and repair;
* grief affects routine disruption;
* affection affects help and belief uptake;
* emotional salience affects memory durability and rumor spread.

Right now, motives are covered, but affect risks becoming either decorative or a hidden utility weight. It needs the same treatment as needs: pressure, not puppet string.

## **5. A learning and adaptation doctrine**

The docs cover memory, belief, staleness, plan repair, and durable intentions. But I would separate **learning** from memory.

Agents should eventually learn things like:

* this route is unreliable;
* this clerk refuses weak reports;
* this household member lies under pressure;
* this food source is often empty;
* this search method fails;
* this actor is dangerous;
* this routine needs a buffer;
* this institution is biased or underfunded.

Learning is dangerous because it can quietly become hidden truth caching. So define it early:

learning is a provenance-bearing update to future expectation, method selection, trust, risk, skill, or routine, caused by remembered experience or modeled instruction.

That would extend the current actor-known cognition model without weakening it.

## **6. A stronger authoring/compiler doctrine**

The no-scripting document is excellent as philosophy. It says authored causal machinery is allowed and authored outcome chains are forbidden. But given what happened in your remediation history, I would make content authoring feel more like a compiler discipline.

The foundation already says designers author actions, affordances, needs, routines, traits, methods, norms, procedures, speech acts, trace types, claim vocabularies, regional processes, initial state, seeds, and fixtures — possibility space, not guaranteed arcs.

What I think is missing is the harder corollary:

Content is executable doctrine. Therefore content must be statically inspectable for constitutional violations.

That means eventual authoring tools should flag:

* hidden culprit fields consumed by cognition;
* notice-board task menus;
* records without sources;
* routines without failure modes;
* rewards without funds/procedure;
* seed facts without provenance;
* action options not NPC-possible;
* LLM prompt packets containing hidden truth;
* fixtures that pass because labels imply the answer.

You have this idea scattered across no-script and acceptance gates. I would promote it.

## **7. Deterministic performance/fairness budgets**

LOD is treated correctly as ontology, not just optimization. The scale doc says low detail must emit structured events and preserve ancestry, and that a toy village that cannot scale is failure while a giant truth-contaminated region is also failure.

But for hundreds of agents, I would add a foundational performance/fairness doctrine:

* planner budgets are explicit and replayable;
* actor scheduling is deterministic and auditable;
* starvation of low-salience actors is prohibited or evented;
* skipped cognition is a fidelity decision, not invisible absence;
* time acceleration must not change outcomes except under declared LOD rules;
* expensive debug/projection machinery cannot become required simulation authority;
* generated summaries must declare what they did not simulate.

This probably belongs in architecture/execution, but the foundation should bless the principle.

## **8. Bias/social harm needs a practical doctrine, not corporate ethics prose**

The docs correctly require wrong suspicion, bias, credibility, social position, institutional fallibility, and unjust procedure. That is central.

But once you simulate false accusations, biased institutions, poverty, theft, punishment, lying, and social standing, you need a doctrine that prevents designer bias from hiding inside “emergent” heuristics.

Not a moralizing policy document. A practical one:

* every bias must be represented as an inspectable model input;
* no protected/social category assumptions in the kernel;
* domain packs own culturally specific biases;
* bias effects must be debug-visible;
* wrong suspicion must have legible causes;
* institutional injustice must be causal, not author prejudice disguised as realism.

The institution docs already move in this direction, but I would name it more sharply.

## **9. The biggest risk: over-constitutionalization**

The main danger is not that the docs are weak. The danger is that the foundation is now so strong that every new feature can become a court case.

That is better than the opposite, but you need one operational escape valve:

a doctrine for deliberately staged incompleteness.

The first playable scope says many things may be staged, and full forgetting/misremembering/identity confusion can deepen later as long as early structures do not block them.

I would formalize that pattern. Every staged feature should declare:

* what it proves now;
* what it deliberately abstracts;
* what it must not fake;
* what future feature it must not block;
* what acceptance gate prevents the abstraction from becoming a lie.

That will keep the project implementable.

## **Bottom line**

The ambition is **not too big conceptually**. It is too big only if you try to implement the dream directly.

The current foundation has the right brutal discipline: small village first, ordinary life first, actor-known truth first, TUI first, replay first, LLMs later, scale later, adventure later. The docs explicitly say the final decision is to build a small, inspectable, TUI-playable ordinary-life village using symbolic epistemic agents, event-sourced causality, typed claims, fallible institutions, structured speech acts, actor-filtered view models, no-human simulation, and deterministic replay — and to let story be discovered after the fact.

My verdict: **the foundation is strong enough to build from.** The missing pieces are not new pillars; they are protective sub-doctrines around play legibility, time, granularity, affect, learning, authoring/compiler enforcement, performance fairness, and staged incompleteness. Without those, implementation pressure will keep trying to turn this into either a sterile proof engine or a contaminated “looks smart” simulation.
