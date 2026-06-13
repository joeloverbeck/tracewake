# Spec 0026 — Foundation Emergence-Evidence Acceptance Doctrine Amendment

This spec **proposes a foundation-tier (constitutional) amendment**. It is a design/proposal
artifact: it specifies the *substance and home* of the amendment so Tracewake's reassess /
amendment process can ratify it. **It does not itself enact doctrine, assign an `INV-###`
number, or author ratified constitutional wording.** That ratification is a deliberate
constitutional act and requires explicit sign-off before any `docs/0-foundation/` file is edited.

> Section set: this file uses the canonical `specs/` section set (Problem Statement, Approach,
> Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions) because
> `specs/` carried no template at authoring time and this is not a hardening implementation spec.

**Status:** proposed; awaiting reassess + constitutional sign-off. Not yet ledger-recorded
(staged specs follow the sibling convention of taking a ledger row at acceptance/closeout, not at
proposal).

**Admissibility posture:** `P0-CERT not applicable`. This is a doctrine-amendment proposal; it
certifies no code and performs no gate audit.

**Authority:** This spec is unusual. A spec is normally subordinate to foundation and *may not*
amend constitutional invariants (`docs/4-specs/SPEC_LEDGER.md`). This spec does not exercise spec
authority to change doctrine — it **stages** an amendment for the foundation-amendment process to
ratify under that process's own authority. If implemented, the resulting foundation edits are
foundation-tier doctrine, not spec-tier text, and this spec becomes historical provenance for why
the amendment was made.

**Provenance:** derived from `reports/foundation-amendment-research-report.md` (external deep
research, target commit `f7adc0149963ea4a90b58ad05f633fd6e71e8649` = current `HEAD` `f7adc01`) and
its brief `reports/foundation-amendment-research-brief.md`. The report's verdict-critical claims
were independently re-verified against live `HEAD` during authoring (see Verification §V1).

---

## 1. Problem Statement

The `0006`–`0025` hardening campaign forced into existence an **emergence-evidence ledger**
(`EMERGE-OBS`, first baselined by archived spec `0020`, refreshed through `0025`) that records
which *unscripted ordinary-life phenomena actually arose* in a no-human run — distinct from guards
that merely prove a phenomenon *could* be reached. `EMERGE-OBS` currently lives in
`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` as, in its own
words, "an observation obligation, not a certification gate… It blocks nothing and passes/fails
nothing."

The constitution celebrates emergence as Tracewake's **thesis** — story is retrospective
(`docs/0-foundation/01`, `03`), authored outcome chains and "emergent plot managers" are forbidden
(`docs/0-foundation/09`), and "adventure may emerge later from ordinary systems"
(`docs/0-foundation/12`). It enforces the *negative*: no scripting (INV-097), no-human runs are
mandatory (INV-091), acceptance is harsh (INV-098). But **no foundation text states the positive
acceptance principle**: that a living-world simulation whose product claim is unscripted
ordinary-life causality must be judged partly by **observer-only evidence that such phenomena were
actually emitted from modeled causes** — not only by static guards and hand-picked reachability
fixtures.

This is a genuine constitutional hole (the report's single Bucket-1 finding). An acceptance
principle that defines *what makes the living-world claim demonstrated* is product identity, and
product identity is foundation's job. Leaving it solely in execution `10` places a `what`-level
acceptance truth below the tier that defines what Tracewake is. This mirrors the precedent of
`docs/0-foundation/14` (the truth firewall), which was itself promoted from hardening insight into
the constitution because the earlier foundation "left too much room for implementation drift."

Existing invariants do not close the hole: INV-091/097/098 prove *absence of scripting* and
*reachability*; none requires *positive emission evidence* that the world generated the
phenomenon families Tracewake claims.

## 2. Approach

Promote a compact, durable **acceptance principle** to the foundation tier, and nothing more.
Foundation authorizes the *principle*; the mechanism (`EMERGE-OBS` table, command, counters,
thresholds, ratchet policy) stays in execution `10` where it already lives. The amendment is a
single coherent package spanning three foundation files plus one cross-tier glossary hand-off.

The accepted formulation (from the report's prior-art survey — ABM validation [GRIMM-2020;
AXELROD-2003], VOMAS observer overlays [NIAZI-VOMAS], weak emergence [BEDAU-1997], emergent-
narrative systems [RYAN-TOTT-KNOWLEDGE; RYAN-CURATING]) constrains the principle to five
properties, all of which the doctrine must preserve:

1. **Observation, not steering.** Evidence collection is outside the simulated world and must not
   feed agent cognition, scheduler priorities, authored events, or validators.
2. **Retrospective, not prospective.** It records what happened; it never creates a reason for
   something to happen.
3. **Phenomenon families, not authored outcomes.** The world must be evidenced to emit *kinds* of
   unscripted phenomena Tracewake claims (contradictions, replans, interruptions, stale-belief
   consequences, modeled-channel corrections, belief/truth divergences, wrong suspicion, record
   effects) — never a specific story beat or numeric threshold in foundation.
4. **Causal replay ancestry.** Evidence must be explainable by event-log ancestry; otherwise it is
   metrics theatre.
5. **No Goodhart target.** Foundation must warn that turning emergence counters into dramatic
   objectives would itself become forbidden authored-outcome machinery (`docs/0-foundation/09`).

This is the only candidate of the seven evaluated that warrants foundation work; the other six are
either already covered or routed below foundation (see Out of Scope §6).

## 3. Deliverables

All deliverables are **proposed amendment substance**, to be ratified by the reassess /
constitutional-amendment process. None assigns an `INV-###` or authors final wording.

| # | Target file | Change | Substance |
|---|---|---|---|
| D1 | `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` | New invariant in the validation / no-scripting neighborhood (currently INV-091…098; firewall family is INV-099…110). | Tracewake's living-world claim is not accepted merely by proving static reachability; acceptance must include replayable, **observer-only** evidence that unscripted ordinary-life phenomena actually arise from modeled causes in no-human (or normal-controller) runs. The observer evidence must not feed simulation behavior, author outcomes, or set dramatic objectives. |
| D2 | `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` | Add explanatory doctrine alongside the mandatory proof cases. | First-playable proof must include an emergence-evidence record as *retrospective acceptance evidence* sitting beside the gates. Foundation names the principle only — never the `EMERGE-OBS` command, schema, table, rows, counters, thresholds, or ratchet policy (those remain execution `10`). |
| D3 | `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` | Add a short cross-reference. | Observing emergent phenomena after the fact is permitted and expected; **steering** the world to satisfy an emergence counter is forbidden authored-outcome machinery. This binds the new acceptance principle to the existing no-director constitution. |
| D4 | `docs/3-reference/02_GLOSSARY.md` (hand-off, **not** authored by this spec) | Glossary addition flagged for a later reference session. | Add a canonical term (e.g. `emergence evidence` / `observer-only emergence evidence`). The reference glossary already carries `evidence`, `observation`, `projection`, `holder-known`, `stale information`; the new term must be coined there, not silently synonymized. This deliverable is a routed hand-off, listed for completeness; it is **out of this spec's edit scope** (foundation-tier only). |

D1–D3 are the constitutional amendment proper (foundation tier). D4 is a downstream reference-tier
hand-off recorded so it is not lost.

## 4. Invariants Alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-097 — No-script compliance is tested | aligns / extends | The new principle is the *positive* complement: 097 proves scripting is absent; D1 requires positive evidence that unscripted phenomena were emitted. Same acceptance surface, opposite polarity. |
| INV-091 — No-human tests are mandatory | aligns | Emergence evidence is gathered from the no-human runs INV-091 already mandates; D1 adds what those runs must *demonstrate*, not a new run type. |
| INV-098 — Feature acceptance is harsh | aligns | D1 sharpens "harsh acceptance" with a positive-emission criterion at the first-playable acceptance surface (D2). |
| No-scripting doctrine (`docs/0-foundation/09`) | aligns (guarded) | D3 closes the Goodhart risk: the evidence mechanism must stay observer-only at the simulation/observer boundary; steering toward counters is forbidden authored output. |
| Forensic causality / replay (`docs/0-foundation/03`, INV-family) | aligns | Property 4 (causal replay ancestry) ties every evidence row to event-log ancestry at the event-log surface — no fact born outside the trace. |

No invariant is weakened or tensioned. The amendment is additive `what`-level doctrine; it
introduces no `how`-level mechanism into foundation (the execution `10` mechanism is untouched).

## 5. Verification

- **V1 — Hole confirmed (done at authoring).** `docs/0-foundation/` contains no acceptance
  principle requiring positive emission evidence. The `emerg*` footprint across the foundation tier
  is entirely thesis-celebration prose (`01:189`, `06:7`, `10:327`, `12:94`, `09:361`), banned-
  director names (`00:17`, `09:38`), and non-binding source notes (`13`); none elevates a positive
  emission-evidence acceptance criterion to doctrine. The two nearest near-misses must be
  distinguished explicitly, because a reviewer will raise them: charter `01:189` ("Emergence is
  only real when the causal and epistemic route to the behavior is modeled") is the truth-firewall
  *negative* — it forbids faking emergence by reading hidden truth, not a *positive* requirement to
  evidence emission; and research-notes `13:187` ("Tracewake cannot accept plausible aggregate
  behavior alone") is non-binding source-note prose in the analysis tier, not a constitutional
  invariant or first-playable acceptance gate. Both gesture at the principle; neither binds it.
  That gap is exactly what D1 fills. `EMERGE-OBS` is confirmed non-certifying in execution `10`.
  Nearest invariants (INV-091/097/098) prove absence/reachability only. Re-verified against live
  `HEAD`, which equals the report's pinned `f7adc01`.
- **V2 — Ratification acceptance (on implementation).** The amendment is accepted only when: D1
  adds a single, falsifiable `what`-level invariant (no procedure, no fixture, no counter); D2 adds
  acceptance doctrine that names the *principle* but no `EMERGE-OBS` mechanism token; D3 adds the
  no-steering cross-reference; and no foundation text duplicates the execution-`10` mechanism.
- **V3 — Boundary check.** Grep the amended foundation files for mechanism tokens (`EMERGE-OBS`,
  table/row/counter/threshold/ratchet names) — foundation must contain the principle and the
  cross-references but none of those tokens, preserving the `what`/`how` layer boundary.
- **V4 — Glossary follow-through (D4, later session).** The new canonical term exists in
  `docs/3-reference/02_GLOSSARY.md` before any architecture/execution session specializes the
  principle.

## 6. Out of Scope

- **`INV-###` assignment and final constitutional wording.** Owned by the reassess /
  constitutional-amendment process (report §11.1). This spec recommends substance and home only.
- **The six non-Bucket-1 candidates.** Provenance sufficiency, memory freshness, believed-access
  affordances (all No-hole — already foundation doctrine), single-charge accounting, falsifiability/
  anti-vacuity, and acceptance-evidence/manifest-fingerprint honesty (all Bucket-2 — routed below
  foundation). These get no deliverable here; their backlog is the report's §10 routing appendix,
  for later tier-by-tier sessions.
- **Any change to the `EMERGE-OBS` mechanism in execution `10`** (table, command, counters,
  thresholds, ratchet). The mechanism is correct where it is; only the governing principle is
  promoted.
- **Architecture/execution/reference cascade edits.** Specializing the new principle into
  `1-architecture/13`, `2-execution/10`, and the glossary is explicitly later, separate work.
- **New world mechanics, Phase-4 expansion, LLM surfaces.**

## 7. Risks & Open Questions

- **R-A — Constitutional change requires sign-off.** Implementing D1–D3 edits the constitution.
  This must not proceed without explicit owner approval; the spec stages the change, it does not
  authorize it.
- **R-B — Invariant placement.** Whether to add a new invariant after INV-098 (validation family)
  or elsewhere, and its exact ordering, is a reassess decision (report §11.1).
- **R-C — Goodhart relapse.** The most likely failure mode is the emergence mechanism quietly
  becoming a target the simulation optimizes toward (a hidden director). D3 + property 5 exist to
  forbid this at the doctrine level; later execution work must keep the ledger observer-only
  (tracked as design-risk R-27 — Acceptance-Evidence Reachability Overstatement — in
  `docs/3-reference/01`).
- **R-D — Institution-known emergence (deferred).** The campaign evidence read was mostly
  actor-known / no-human / TUI. If later Phase-4 institutional machinery shows institution-known
  emergence needs its own acceptance evidence, that is a separate future amendment audit, not this
  spec (report §11.4).
- **R-E — Possession-bind perception (adjacent, unrelated).** The architecture conformance index
  records an unresolved `INV-087`-adjacent decision about modeled perception on possession binding
  (report §11.3). Flagged as a separate owner-decision topic; not promoted here.
