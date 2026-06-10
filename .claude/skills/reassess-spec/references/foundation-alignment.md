# Foundation Alignment Check (Step 4)

The foundation doc pack is the Tracewake constitution and its supporting authority tiers. The hard-fail authority is `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (the `INV-NNN` invariants). It sits atop the tiered doc pack indexed by `docs/README.md`, whose binding authority order is:

1. foundation doctrine (`docs/0-foundation/`);
2. architecture contracts (`docs/1-architecture/`);
3. execution phase gates and fixtures (`docs/2-execution/`);
4. reference guardrails (`docs/3-reference/`);
5. current analysis and research synthesis;
6. the spec package (`docs/4-specs/`, `specs/`).

If a spec conflicts with a higher tier, the higher tier wins and the spec is wrong.

## 4.0 Internal Contradictions

Before checking the foundation pack, scan for contradictions inside the spec — between its Purpose, Scope (in scope / out of scope / non-goals / not allowed), Deliverables / required areas, Binding-invariants and Binding-architecture-constraints sections, acceptance/exit criteria, and any forbidden-changes list. If the spec includes a table that classifies state (e.g. "in scope vs. out of scope", "required vs. not-applicable evidence", "allowed vs. forbidden change"), verify consistency across sections. A deliverable that contradicts the spec's own Non-goal, Not-allowed, or Forbidden-changes entry is a CRITICAL Issue.

## 4.1 Binding-Invariants / Constraints Section Verification

Tracewake specs carry binding-invariants and binding-architecture-constraints sections that name the doctrine the spec must satisfy (see `archive/specs/0002_*` §6 "Binding invariants from the foundation docs" and §7 "Binding architecture constraints"). Verify each entry:

- Invariant references must resolve to real `INV-NNN` headings in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`, or to a named architecture/execution contract. A bare gesture at "the foundation docs" without a specific invariant/contract is a MEDIUM Improvement finding.
- Each rationale must be specific — a bare `aligns` without a named mechanism (which deliverable, which boundary, which invariant) is a MEDIUM Improvement finding.
- For deliverables touching kernel authority, the kernel boundary, validation/acceptance, visibility/no-leak, determinism/replay, or the LLM boundary, the rationale must name the mechanism (e.g. "the kernel owns event application; the TUI module only renders an actor-filtered view model — no view decides legality").

**No alignment section at all**: do not treat absence as CRITICAL by default. Surface adding one as an Addition (MEDIUM) for specs whose deliverables don't touch kernel-authority / boundary / acceptance-invariant semantics. Escalate to a HIGH Issue only when a deliverable that *does* touch those semantics ships with no binding-invariant/constraint statement anywhere — there the omission is a grounding gap.

## 4.2 Missing Invariants

Identify constitutional invariants and architecture contracts the spec should address but doesn't. Pay particular attention to:

- **Kernel authority (INV-008, INV-042; `docs/1-architecture/01_*`, `10_*`)** — specs whose deliverables touch setup, validation, legality, state transitions, event application, scoring/terminal detection, RNG, semantic effects, view projection, replay/hash, or serialization must keep all of these in the authoritative kernel and keep TUI/view-models presentation-only and the LLM out of authority. A deliverable letting a view, debug surface, or the LLM decide world legality or invent world state is a CRITICAL Issue.
- **Kernel boundary + dependency direction (`docs/1-architecture/01_*`)** — specs touching the kernel boundary must preserve the doctrine-mandated dependency direction; a presentation/content layer that gains world authority, or an authority inversion, is a boundary-failure Issue.
- **No-scripting / content-is-possibility (foundation 09; `docs/2-execution/08_*`)** — specs introducing content/fixtures must keep them typed possibility, parameters, metadata, fixtures, or traces only. Behavior-looking fields (selectors, rule branches, loops, triggers, conditional effects, arbitrary expressions) and authored outcome chains / director logic are blocked or escalated; an authored script is a CRITICAL Issue.
- **Determinism & replay (INV-017, INV-018, INV-019; `docs/1-architecture/02_*`)** — see 4.3.
- **No-leak / no telepathy (INV-024, INV-006; `docs/1-architecture/06_*`, `10_*`)** — see 4.3.
- **LLM/language boundary (INV-042; foundation 11; `docs/1-architecture/07_*`)** — speech-act/text deliverables must keep the LLM behind validated extraction and out of world authority.
- **Constitutional invariants overall** — the full `INV-NNN` set is the master checklist every substantial change must satisfy. See 4.3.

## 4.3 Constitutional-Invariant Check

For each invariant the spec's deliverables engage, verify the spec upholds it. The load-bearing ones for spec reassessment:

- **Kernel remains world authority; views/LLM do not decide legality** — no deliverable moves legality, validation, or world state into a view model, debug surface, or the LLM. Violation: CRITICAL (INV-008, INV-042).
- **Kernel boundary preserved; dependency direction respected** — see 4.2. Violation: CRITICAL / HIGH.
- **Content is possibility, not script; no authored outcome chains** — content/fixtures introduce no behavior-looking fields and no director logic (no-scripting doctrine, foundation 09). Violation: CRITICAL.
- **Unknown fields rejected by default; behavior-looking fields blocked** — specs proposing hand-authored data/fixture schemas must reject unknown fields and refuse behavior-looking ones. Missing fail-closed handling is a HIGH Issue.
- **Validation is fail-closed and blocking; warnings distinguished from blockers** — specs proposing validation must name what failing means (block? warn?) and keep hard failures non-overridable. Conflating warnings with blockers, or a user-overridable hard failure, is a violation. Unaddressed second-order effects are Improvement findings at minimum (`docs/1-architecture/13_*`; `docs/2-execution/08_*`).
- **Replay, hashes, serialization order, RNG, snapshots, and traces remain deterministic** — see the determinism sub-check below. A change to replay/hash/snapshot semantics also requires a foundational-doc amendment.
- **No telepathy; actor-knowledge filtering holds; possession transfers no knowledge** — for deliverables touching view-model projection, possession, previews, traces, debug views, notebooks, or replay exports, verify no path lets hidden information reach a viewer the actor-knowledge filter forbids. Leakage is CRITICAL (INV-024, INV-006; `docs/1-architecture/06_*`, `10_*`).
- **Events carry causes; meaningful changes are eventful and leave traces** — deliverables touching the event log must keep every meaningful state change eventful with a cause model, and traces consistent (INV-009/010/011/013; `docs/1-architecture/02_*`). Current-state-only shortcuts are violations.
- **Symbolic, inspectable agents with debug traces; no LLM agent brains** — agent-cognition deliverables must stay symbolic/inspectable and keep the LLM out of decision-making (INV-032, INV-041, INV-042). A learning/LLM agent brain without doctrine support is CRITICAL.
- **Evidence coverage** — substantial changes must carry the verification the relevant Phase acceptance gates require (tests, golden traces, deterministic-replay/hash checks, fixtures, no-human runs, why-not checks) per `docs/2-execution/10_*` and `docs/0-foundation/12_*`. A spec missing these is an Issue (see 3.11 in codebase-validation.md).

Record each violation with the specific invariant and contract. Cite the `INV-NNN` id and section heading exactly (e.g. `INV-018 — Deterministic replay is foundational`, `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md §kernel authority`). Bare citations (`invariant violation`) without ids force Step 7's pre-apply verification to disambiguate.

**Determinism sub-check**: if the spec's deliverables or verification steps claim determinism, byte-identical replay, or stable hashing, verify against the determinism doctrine (INV-017, INV-018, INV-019; `docs/1-architecture/02_*`; the spec's own determinism contract, e.g. `archive/specs/0002_*` §10):

- Deterministic, seedable, auditable RNG only — never wall-clock seeding or `std::time` in canonical/replayed forms (INV-017).
- Stable serialization/iteration order — rely on the serialization-boundary contract and sorted/insertion-ordered collections, not incidental hash-map iteration order.
- No nondeterministic inputs in canonical forms (wall-clock timestamps, thread-scheduling-dependent ordering) unless the spec explicitly separates a captured-at value (allowed) from a canonical-form input (forbidden).
- Snapshots and compaction may not erase live ancestry (INV-019).

Flag determinism violations as HIGH Issues citing the specific `INV-NNN`; if the deliverable *changes* replay/hash/snapshot semantics, also flag the foundational-doc-amendment requirement.

## 4.4 Acceptance-Gate Check

The Phase acceptance gates live in `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`, the per-surface execution docs (`docs/2-execution/04–12`), and `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`. For a spec being prepared for implementation, a deliverable that *would break* an acceptance gate the spec's phase must pass is a CRITICAL Issue. Check each engaged gate against the spec:

- the simulation runs coherently with **no human controller** (INV-004);
- **deterministic replay** of the event log reproduces state byte-identically (INV-018);
- the phase is **TUI-playable / runnable** where the phase ladder requires it;
- **why-not explanations** are available where the phase requires them;
- the **missing-expected-property** first-proof obligations (Spec 0001 / `docs/2-execution/02_*`) are met where the phase depends on them;
- no **authored outcome chain** fakes a result the simulation should produce (foundation 09);
- **hidden information does not leak** into view models, traces, debug views, notebooks, or replay exports (INV-024).

Present in Step 6 as a `### Acceptance-Gate Check` section: one line per engaged gate, each `clear | N/A | flag (reason)`. Omit the section when no acceptance gate is engaged.

## 4.5 Foundational-Doc-Amendment Check

Tracewake has no ADR system. The mechanism for an architecture-changing decision is a **foundational-doc amendment** recorded through the spec layer — `docs/README.md` routes architecture replacement, constitutional amendments, and acceptance weakening through the spec package (the former standalone amendment-precedent spec was removed in the doc realignment). When a spec's deliverable makes a decision that contradicts or changes an accepted foundation/architecture/execution doc — without an accepted amendment recording the change — flag it as a HIGH Issue (the spec needs the amendment first, or must drop the decision). Triggers include: changing a constitutional invariant's force or scope; changing the kernel-authority or dependency-direction boundaries; changing event-log / replay / hash / snapshot / serialization semantics; changing the actor-knowledge / visibility contract; changing the no-scripting boundary (introducing selectors, expressions, a content DSL, or data-driven rules); changing the LLM/language boundary to give the LLM world authority; changing the Phase ladder, acceptance gates, or first-proof scope.
