# 0067 TUI Conversation Structured Speech-Act Panel Spec

**Status**: PROPOSED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and promoted to `archive/specs/` on acceptance; it is never
promoted to the live `docs/4-specs/` tier. It is not a certification audit, mutation-remediation
pass, Phase-4 entry claim, or latest-`main` certification. It uses the sibling hardening-spec
house structure (`0046`, `0047`, `0057`).

This is **Spec G** of the TUI Experience Overhaul roadmap
(`reports/tui-experience-overhaul-research-report.md` §3.9, §7). It adds a structured conversation
surface — speech-act intent, topic, evidence attachment, utterance preview — that submits **typed
proposals behind validation** and never lets free text create simulation facts.

**Conditionality.** This spec carries an explicit gate (report open-question §9.1 #4): its first
implementation ticket must determine whether existing `semantic_actions` already expose adequate
conversation affordances. If they do, this spec is TUI-only rendering of those affordances. If they do
not, the shortfall routes to a Spec `0063`-style crate-crossing contract (`ConversationAffordanceView`
/ speech-act view-model data owned by architecture 07 and 10) **before** any TUI speech surface is
built — not ad-hoc parser behavior in the TUI.

## 0. Baseline statement and source discipline

- **Driver.** The research report's conversation-UI section and the language-boundary doctrine that a
  prose surface may render or parse candidate speech but may not create facts or bypass validators.
- **Baseline commit.** Re-verified against repository `HEAD`
  `85dd8836508a58305eb84d70caf52bda088a9bde`. Named symbols authoritative; line numbers provenance
  only.
- **Source discipline.** Commit hashes are audit/provenance only. No latest-`main` certification,
  Phase-4 entry, or second-proof entry. A manifest is path inventory only.
- **Authority posture.** Subordinate to foundation → architecture → execution → reference. Routes
  doctrine sharpening as *substance + home* (§5); ratifies no wording; mints no id.
- **Execution admissibility posture.** `P0-CERT not applicable`.
- **Ledger timing.** Archived-row entry at acceptance/closeout, not at proposal. No ledger row now.

## 1. Scope

### 1.1 In scope

In `tracewake-tui` (with architecture-07/10-owned core additions only if the gate above requires them):

1. **Structured conversation panel:** choose speech-act intent (ask/tell/accuse/offer/refuse), topic,
   and optional actor-known evidence attachment; show an utterance preview; submit via `Enter`, cancel
   via `Esc`.
2. **Typed submission only:** the panel submits a typed speech-act proposal through the existing
   validation pipeline; free text is a *preview*/parse candidate, never an authoritative state mutation.
3. **Actor-known evidence only:** attachable evidence is restricted to actor-known observations/records
   already available to the possessed actor.
4. **LLM-disabled determinism:** the panel operates deterministically with the LLM disabled; any LLM use
   is render/parse-only behind validation.

### 1.2 Out of scope (non-goals)

- No freeform parser that mutates state; no prose-born facts.
- No new speech-act *semantics* in the TUI; richer affordances, if needed, are owned by architecture
  07/10 and routed as a crate-crossing contract, not implemented ad hoc.
- No LLM authority over world actions, records, or institutional outcomes.

## 2. Dependencies and ordering

- **Depends on:** `0061` (screen model), `0062` (intents), `0064` (pane layout). If the affordance gate
  routes to a core contract, that core work depends on the `0063`-style seam pattern.
- **Blocks:** contributes the conversation surface to Spec `0069` if implemented.
- **Note:** most conditional item on the roadmap; may reduce to a thin rendering spec or expand to a
  crate-crossing contract depending on the gate outcome.

## 3. Doctrine anchors

- **Foundation 11** (`…/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`): structured speech acts; the
  language boundary; prose may render/parse but not create facts or bypass validators.
- **Architecture 07** (`…/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`) and **Architecture 10**:
  speech-act surfaces and view-model boundaries.

## 4. Findings and remediation requirements

- **4.1 Determine the affordance gate first (seam: `semantic_actions` census).** Ticket 1 audits whether
  existing semantic actions cover the intended speech acts; the verdict decides TUI-only vs crate-crossing.
- **4.2 Typed proposal, validated (seam: submission path).** Submissions build typed speech-act proposals
  routed through validation; the panel constructs no fact directly.
- **4.3 Evidence is actor-known (seam: evidence attachment).** Only actor-known observations/records may be
  attached; no hidden truth is selectable.
- **4.4 Free text is preview-only (seam: utterance preview).** The preview string carries no authority and
  cannot mutate state.
- **4.5 Implementation decomposition.** Affordance-gate census; conversation pane rendering; typed-proposal
  submission wiring; actor-known evidence attachment; (conditional) crate-crossing
  `ConversationAffordanceView` contract. Separate tickets.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Architecture 07 / 10** — if a `ConversationAffordanceView`/speech-act view-model addition is required,
  route it as an architecture-owned closed view-model contract with two-hop disposition (shared with Spec
  `0070`). Substance only; conditional.

## 6. Required fixtures and tests

- LLM-disabled deterministic operation: the panel renders and submits without an LLM.
- A speech-act submission produces a typed proposal through validation (positive).
- Negative: free text / an unmodeled utterance cannot mutate state or create a record.
- Evidence-attachment is limited to actor-known items (negative: a hidden item is not attachable).

## 7. Acceptance artifact and evidence

A review artifact recording the affordance-gate verdict, LLM-disabled determinism, typed-proposal
submission, the no-prose-fact negative, and actor-known evidence restriction, at the exact implementation
commit.

## 8. Implementation constraints

- Default to TUI-only rendering of existing affordances; expand to core only through an architecture-07/10
  contract when the gate requires it.
- No LLM authority; render/parse-only behind validation.

## 9. Risks and open questions

- **Risk: prose-born facts** if free text is treated as authoritative. Mitigation: typed proposals behind
  validation; preview-only text.
- **Open question (§9.1 #4):** are existing semantic actions sufficient? Resolved by the ticket-1 gate.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-072 (speech begins as structured speech acts) | aligns | Panel submits typed speech-act proposals. |
| INV-109 (LLM output is not cognition authority without validation) | aligns | LLM render/parse-only behind validation; no state mutation. |
| INV-022 (raw prose is not authoritative state) | aligns | Utterance preview creates no facts. |
| INV-101 (actor-known context is sealed) | aligns | Evidence attachment is restricted to actor-known items. |
