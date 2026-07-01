# TUI Experience Overhaul — Deep-Research Brief (ChatGPT-Pro Session 2)

You are a deep-research session. Produce the deliverable directly. **Do not interview or ask
clarifying questions** — the requirements below are final. If a genuine contradiction makes a
requirement impossible, state it in the deliverable and proceed with the most faithful
interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-07-01_85dd883.txt`) is the exact path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust: an
event-sourced kernel, subjective epistemics, fallible institutions, questless leads, and a
TUI-first playable client. **Fetch every file you read from commit `85dd883`** (`git ls-tree -r
--name-only HEAD` at that commit produced the uploaded manifest, so the manifest is that tree exactly).

Docs are layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` →
`4-specs`; **earlier tiers govern later ones.** If execution conflicts with architecture or
foundation, execution is wrong; if implementation is more convenient than the accepted gates,
implementation is wrong.

**This is a new design direction, not a re-audit.** Two prior report families bear on the TUI, and
you must treat their conclusions as *constraints you build within*, not work to re-commission:

- `reports/tui-parity*` established a **two-hop playable-capability parity contract** (core feature →
  view model → rendered surface) with a compile-time drift guard, later ratified as
  `archive/specs/0046_*`. The current renderer is a **pure function** `render_embodied_view(&EmbodiedViewModel)
  -> String` — a deliberately testable asset. This is settled architecture; the new TUI **consumes**
  it and must not weaken it.
- `reports/tui-human-wait-runs-simulation-*` established that time-advancing controls route through a
  **kernel-owned one-tick coordinator** and that interval/missed-event summaries are **holder-known
  projections**, never raw event diffs or hidden truth; ratified as `archive/specs/0047_*`. Settled
  architecture; do not redesign it.

No predecessor ever proposed a full-screen terminal-UI framework, at-a-glance panels, or presentation
modernization — the TUI has, to date, been treated as an acceptable line-based command REPL. That gap
is exactly this brief's subject. There is **no** paired predecessor research report for this topic to
carry findings forward from; the two families above are constraint context, cited in §2.

---

## 2. Read in full (authority order)

Read these before producing. The user has asked that you read **all** of `docs/0-foundation/*`,
`docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and `docs/4-specs/*`. Below,
**primary (load-bearing)** entries carry a one-line reason; the remaining files in each tier are
**boundary-awareness** reads — read them to ground the doctrine and to know what is *out* of scope
for a TUI/presentation pass (do not treat them as conformance targets or try to "correct" the
subsystems they govern).

**Universal:**
- `docs/README.md` — authority order and the layering rule.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001…INV-NNN`; every recommendation must
  satisfy these; a genuine divergence requires amending an invariant first, never designing against it.

**0-foundation (primary):**
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **the** TUI charter: product surface,
  embodied vs. debug mode, what embodied mode may show ("perceived place, exits, **nearby actors**,
  objects, traces, affordances, risks") and must hide, time controls, lead wording, conversation UI,
  possession parity. Your redesign is governed by this document above all.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — the firewall the
  "see co-present agents sleeping/eating" feature must respect: perceived-actor activity may be shown
  only as actor-known observation, never re-read from ground truth.
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — defines what counts as
  actor-known (perception, memory, inference, testimony, records), bounding what a co-present-awareness
  surface may legitimately display.
- `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — governs any conversation-UI redesign
  and the surface-text boundary (no freeform parser mutates state).
- *Boundary-awareness:* the remaining `docs/0-foundation/*` (charter, causal trace, agents/needs,
  actions/affordances, institutions, no-scripting, scale/LOD, first-playable scope, research decisions).

**1-architecture (primary):**
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — the authoritative
  client contract: the minimum `embodied_view_model` schema (note `visible_actors` already exists),
  the two-hop parity contract, "menu positions are presentation-only," the captured-projection rule
  (the TUI may not hold a live physical-state handle or re-read truth to freshen labels), transcript
  obligations, future-client parity. Your presentation layer lives strictly inside these boundaries.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — the sealed
  holder-known context the enriched co-present surface must derive from.
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — how
  observations of other actors become actor-known, i.e. the seam any "actor X is sleeping" perception
  must flow through.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — the acceptance/
  review-artifact contract your recommendations must remain testable under.
- *Boundary-awareness:* the remaining `docs/1-architecture/*`.

**2-execution (primary):**
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — the current proof
  obligations for embodied/possession/debug view models the redesign must keep satisfying.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — the anti-contamination
  gates a co-present-activity surface must pass.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — the testing/diagnostics
  regime (golden transcripts, snapshots) the new render path must remain drivable under.
- *Boundary-awareness:* the remaining `docs/2-execution/*`.

**3-reference (primary — small tier, read all closely):**
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — review checklist the deliverable is
  measured against.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — existing risks; note any this overhaul raises or retires.
- `docs/3-reference/02_GLOSSARY.md` — canonical terms; use them, do not mint new ones.

**4-specs:**
- `docs/4-specs/SPEC_LEDGER.md` — spec history and the "Next known execution move"; confirms the
  cert ladder state and that recent work (0046/0047/0057/0058/0059) has been TUI/embodied-adjacent.
- `docs/4-specs/README.md` — spec authoring/placement conventions (relevant to your spec-roadmap).
- *Boundary-awareness:* `docs/4-specs/0001_*`, `docs/4-specs/0003_*`.

**Predecessor reports (constraint context — read, do not re-commission):**
- `reports/tui-parity.md`, `reports/tui-parity-research-brief.md`, `reports/tui-parity-research-report.md`
  — the two-hop parity mechanism and why the pure `render_embodied_view -> String` seam is an asset.
- `reports/tui-human-wait-runs-simulation-issue.md`,
  `reports/tui-human-wait-runs-simulation-research-report.md` — the kernel-owned one-tick coordinator
  and holder-known interval summaries.

**Archived specs (settled architecture the new TUI consumes):**
- `archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md`
  — the ratified two-hop drift guard.
- `archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`
  — ratified world-advance / interval-summary contract.
- `archive/specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md`
  and `archive/specs/0058_EMBODIED_ROUTINE_CONTINUATION_FOUNDATIONAL_ALIGNMENT_HARDENING_SPEC.md` —
  the current embodied continuation/possession-parity behavior your loop redesign builds on.

**Code seams (inspect directly; do not paste wholesale into the deliverable):**
- `crates/tracewake-tui/src/main.rs`, `.../launch.rs` — entrypoint, argv-driven launch modes (embodied
  vs. operator-debug), catalog/help. **The current "TUI" has no `ratatui`/`crossterm` dependency** — it
  is a stdin/stdout **line REPL** (`run.rs::run_command_loop` reads lines, prints rendered strings).
- `crates/tracewake-tui/src/run.rs` — the command loop and dispatch (the "mostly a CLI" surface).
- `crates/tracewake-tui/src/render.rs` — `render_embodied_view(&EmbodiedViewModel) -> String`, the pure
  render seam and `DEBUG_TOKENS`.
- `crates/tracewake-tui/src/app.rs`, `.../input.rs`, `.../debug_panels.rs`, `.../transcript.rs` — app
  state, command parsing, debug panels, transcript capture.
- `crates/tracewake-core/src/view_models.rs` — `EmbodiedViewModel`, `NotebookView`, `VisibleItemSource`,
  and the `visible_actors`/local-actors surface you would enrich.
- Tests that pin the acceptance regime a new render path must survive:
  `crates/tracewake-tui/tests/tui_seam_conformance.rs`, `.../transcript_snapshot.rs`,
  `.../playable_capability_parity.rs`, `.../embodied_flow.rs`, and `crates/tracewake-tui/tests/goldens/*`.

---

## 3. Settled intentions (these lock the deliverable)

The following decisions are final. Do not re-open them or survey alternatives for them.

1. **Scope = presentation + core view-model/projection enrichment.** The deliverable covers the full
   stack, not just the TUI crate. The headline goal — glancing at a co-present agent and seeing what
   they are doing (sleeping, eating, working, etc.) — cannot be met by presentation alone; it requires
   the **core** to surface actor-known *observed activity* of co-present actors in the embodied view
   model (the existing `visible_actors`/local-actors surface is the natural home). For every
   recommendation that crosses the crate boundary (into `tracewake-core` or `tracewake-content`),
   **explicitly flag it and route it** — name the view-model field/projection, and the architecture
   doc that owns the contract.

2. **The co-present-activity surface must respect the truth firewall.** A perceived actor's activity
   may be displayed **only** as actor-known observation captured at a modeled perception/observation
   boundary (foundation 08 & 14; architecture 03 & 06). It may never be re-read from ground truth, and
   it must degrade honestly: an activity the possessed actor cannot perceive (a hidden actor, an
   occluded room, an unobservable internal state) is **not** shown, and staleness/uncertainty is
   labeled where actor-relevant. Design the enrichment so it *cannot* become an omniscient roster.

3. **Recommend a concrete terminal-UI framework stack.** Survey the field, then commit to a concrete
   recommended stack with justification (ratatui + crossterm is the Rust-ecosystem standard and the
   expected baseline; evaluate meaningful alternatives and say why they lose). Specify a migration path
   from the current line REPL and the crate-dependency implications (the TUI crate is the boundary
   crate and *may* take dependencies; `tracewake-core` must remain zero-dependency — do not push any UI
   dependency below the TUI crate).

4. **Agent-drivable & headless-legible is a HARD, co-equal requirement — not polish.** The maintainer
   routinely runs automated agents (Claude Code loops) that "play the TUI" to find defects and
   wrong-information bugs, and the entire acceptance regime is golden-transcript / snapshot based. A
   full-screen, stateful TUI must therefore preserve — and ideally improve — the ability for a
   **non-interactive/automated driver** to (a) submit deterministic scripted input and (b) observe and
   reason about the *rendered interface* as inspectable text/data (e.g. a headless render-to-buffer or
   render-to-string snapshot path, or a structured "what is on screen right now" dump). Treat this as a
   first-class design axis with equal weight to human ergonomics. The design must reconcile "sophisticated
   full-screen presentation" with "the pure `view-model → buffer/text` render seam that golden tests and
   agent loops both consume" — do not let the interactive shell swallow the testable/inspectable seam.

5. **Embodied-first, debug/operator second.** Lead with the embodied play experience (the maintainer's
   stated pain: clunky, opaque, awkward to embody, no at-a-glance awareness of co-present agents). Also
   modernize the **debug/operator** surfaces — foundation 08 and architecture 10 mandate them as
   permanent, visibly non-diegetic surfaces — as a secondary section. Keep the embodied/debug split
   structurally intact; debug truth must never leak into embodied presentation.

6. **Single consolidated report** (see §7 for exact shape) carrying, in order: prose recommendations →
   an ordered **spec-decomposition roadmap** → a labeled **foundational/doctrine amendment** section
   for any change genuinely warranted. Amendments are delivered as **substance + home** (what doctrine
   the target doc must own, and which file/section it lands in), **not** paste-ready invariant text and
   **not** invented identifiers (`INV-###`, gate codes, glossary terms) — minting those remains the
   repo's own reassess/amend process.

7. **Negative settled intentions (do NOT re-open):** the two-hop playable-parity mechanism
   (`archive/specs/0046_*`, `reports/tui-parity*`) and the kernel-owned temporal / holder-known
   interval-summary contract (`archive/specs/0047_*`, `reports/tui-human-wait*`) are **settled
   architecture**. The new TUI consumes and must remain conformant to both; it does not redesign,
   relitigate, or weaken either. If the presentation redesign genuinely stresses one of them, *flag the
   tension* and route it — do not silently rework it.

---

## 4. The task

Design a comprehensive, doctrine-aligned overhaul that turns Tracewake's line-oriented command REPL
into a **sophisticated terminal user interface** — legible, at-a-glance, and pleasant to embody an
agent in — while (a) making a possessed actor's *co-present awareness* of other actors' observable
activity first-class and firewall-honest, (b) recommending a concrete framework stack and migration
path, and (c) preserving automated/headless drivability and the existing golden-transcript/snapshot
acceptance regime. This is a **new-spec-generating design pass** whose deliverable is an
analysis/recommendation report (not itself a numbered spec): the maintainer will decompose it into as
many implementation specs as logically necessary, so the report must be decomposition-ready.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2. **Research online as deeply
as needed** — this is the core value of this session. Investigate: mature terminal-UI frameworks and
their trade-offs (ratatui/crossterm and alternatives); presentation patterns from sophisticated TUIs
(e.g. structured multi-pane dashboards, status/awareness panes, modal vs. non-modal interaction,
keybinding discoverability, focus/navigation models); how comparable simulation/roguelike/immersive-sim
interfaces present *co-present entity awareness* and *ambient world state at a glance* while preserving
a subjective/limited-information viewpoint (fog-of-war, perception-gated status, "you see X doing Y"
messaging); relevant HCI/information-design and terminal-UI research or prior art; and testing/automation
patterns for full-screen TUIs (headless rendering, buffer snapshotting, deterministic input injection,
accessibility). **Cite sources for every external claim that shapes a recommendation.**

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution: every product-behavior
  recommendation must satisfy it; a genuine divergence requires **amending an invariant first**, never
  designing against it silently. Where you believe an amendment is warranted, put it in §7's amendment
  section as a routed recommendation — do not assume it.
- **Authority order:** foundation governs architecture governs execution governs reference/specs. A
  presentation change may not induce a lower tier to contradict a higher one.
- **Truth firewall / anti-contamination:** no simulation fact may be born from prose or presentation.
  Preserve event-sourced causality, subjective epistemics, ordinary agents, possession parity (possession
  is a viewpoint/input binding, never a cognition upgrade), fallible institutions, and deterministic
  validation/replay. The TUI may be friendly; it may not be metaphysically privileged. It may not apply
  events, mutate state, read hidden truth for embodied views, hold a live physical-state handle, or add
  player-only verbs.
- **Crate dependency direction is one-way:** `tracewake-core` (zero deps) ← `tracewake-content` ←
  `tracewake-tui`. Any UI framework dependency lives only in `tracewake-tui`. Never invert this.
- **No backwards-compatibility shims or alias paths** in recommended new work.
- **Two-hop parity + captured-projection rules** (architecture 10) bind the presentation layer: clients
  render only typed, actor-filtered view models and closed presentation enums; new view-model fields or
  closed variants need an explicit presentation disposition, not a wildcard/default/debug catchall.

---

## 7. Deliverable specification

Produce **one new downloadable markdown document**:

- **`reports/tui-experience-overhaul-research-report.md`** — **new** (not a replacement). This is an
  **analysis / recommendation report**, *not* a numbered `docs/4-specs/` spec — so **no** spec-number,
  ledger, or epoch rules apply here. The maintainer will turn it into specs afterward.

The report must contain, in this order:

1. **Executive summary** — the target end-state ("what a sophisticated Tracewake TUI looks and feels
   like") in a few paragraphs, and the headline decisions (framework, embodied redesign, co-present
   awareness, agent-drivability).
2. **Current-state assessment** — a grounded read of today's line-REPL TUI (cite the code seams in §2):
   what makes it clunky/opaque, and specifically why co-present agent activity is not glanceable today.
3. **Target design** — the embodied-mode redesign first: layout/panes, at-a-glance status (self body
   state/needs/intention/routine), the **co-present awareness surface** (how "actor X is sleeping/eating"
   is presented, sourced from actor-known observation), navigation/focus/keybinding model, why-not and
   lead presentation, time-control affordances. Then the debug/operator-mode redesign. For every element,
   name the view-model contract it renders and confirm it stays inside the two-hop parity + captured-projection
   rules.
4. **Core enrichment required** — precisely which core view-model fields/projections must be added or
   extended to support the target design (especially the co-present observed-activity surface), each
   routed to the owning architecture doc and framed as *what doctrine must own*, respecting the truth
   firewall. Distinguish "renders existing fields differently" (TUI-only) from "requires new actor-known
   data" (core/architecture).
5. **Framework recommendation** — the concrete recommended stack, alternatives considered and rejected,
   migration path from the current REPL, dependency-direction compliance, and — explicitly — how the
   chosen approach **preserves the pure `view-model → buffer/text` render seam** and the golden/snapshot
   acceptance regime.
6. **Agent-drivability & testability plan** — how a non-interactive/automated driver scripts input and
   inspects rendered output under the new full-screen design (headless render path, buffer/text snapshot,
   deterministic input injection), and how existing golden-transcript / `tui_seam_conformance` /
   `playable_capability_parity` obligations are preserved or upgraded.
7. **Ordered spec-decomposition roadmap** — the sequence of implementation specs the maintainer should
   cut, each with: a one-line objective, its scope boundary, its dependencies/ordering, and whether it is
   TUI-only or crate-crossing. Sequence so that the pure-seam/testability guarantees land before or
   alongside the interactive shell. (Do **not** assign spec numbers — the repo's ledger owns numbering;
   describe ordering and dependencies instead. `assumption:` staging placement `specs/` vs. final
   `docs/4-specs/` follows the repo's existing convention and is the maintainer's call.)
8. **Foundational / doctrine amendments (if warranted)** — a clearly labeled section. For each proposed
   amendment: the driver, the current doctrine text it stresses, **what the target doc must now own**
   (your own prose at the right altitude for that tier), and **which file/section** it lands in. Deliver
   **substance + home only** — no paste-ready invariant wording and no invented `INV-###`/gate/glossary
   identifiers. If, after analysis, **no** amendment is genuinely warranted (much of the intent —
   perceiving nearby actors, staged time controls, friendly-but-not-privileged presentation — already
   appears sanctioned by foundation 08), say so explicitly and explain why the overhaul fits within
   current doctrine; do not manufacture amendments.
9. **Open questions & risks** — anything the maintainer must decide during spec authoring, and risks
   this overhaul introduces or retires (cross-reference the design risk register).
10. **References** — every external source cited.

**Locked / no-questions instruction:** Produce the deliverable directly as a downloadable markdown
document. Do not interview, do not ask clarifying questions — the requirements above are final. If a
genuine contradiction makes a requirement impossible, state it in the deliverable and proceed with the
most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- The deliverable set is exactly one new file, `reports/tui-experience-overhaul-research-report.md`, in
  the analysis/recommendation shape above — no numbered spec authored.
- Every recommendation stays inside the truth firewall: no embodied surface shows anything not
  actor-known; the co-present-activity design cannot become an omniscient roster.
- The two-hop parity contract and the kernel-owned temporal/interval-summary contract are consumed, not
  redesigned or weakened; any tension with them is flagged and routed, not silently reworked.
- Crate dependency direction is respected: no UI dependency is pushed below `tracewake-tui`;
  `tracewake-core` stays zero-dependency.
- The design demonstrably preserves a pure `view-model → buffer/text` render seam that both golden/snapshot
  tests and an automated agent driver can consume; the agent-drivability plan is concrete.
- Every external claim that shapes a recommendation is cited.
- Amendments (if any) are substance + home only — no paste-ready invariant text, no invented identifiers;
  if none is warranted, that verdict is stated with reasons.
- Every file you relied on exists at commit `85dd883` (the manifest's tree).
