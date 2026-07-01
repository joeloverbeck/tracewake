# 0064 TUI Embodied Pane Layout and At-a-Glance Panels Spec

**Status**: PROPOSED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and promoted to `archive/specs/` on acceptance; it is never
promoted to the live `docs/4-specs/` tier. It is not a certification audit, mutation-remediation
pass, Phase-4 entry claim, or latest-`main` certification. It uses the sibling hardening-spec
house structure (`0046`, `0047`, `0057`).

This is **Spec D** of the TUI Experience Overhaul roadmap
(`reports/tui-experience-overhaul-research-report.md` §3.2–3.6, §7). It builds the fullscreen
embodied pane layout — place/situation, self/body/routine, co-present actors, actions/affordances,
details/why-not, notebook/leads, recent actor-known changes, input hints — plus a `ratatui` buffer
render path, over the screen-model seam. It renders existing view-model fields and Spec `0063`'s
enriched actor data; it adds no core rule.

## 0. Baseline statement and source discipline

- **Driver.** The research report's structural finding: today's embodied output is a single vertical
  list with no stable panel positions, no focus/inspection model, and no continuous keybinding/action
  visibility.
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

In `tracewake-tui` only:

1. **Pane composition** over the `EmbodiedScreenModel`: header/mode bar, Place/Situation,
   Self/Body/Routine, Co-present actors, Actions/Affordances, Details/Why-not, Notebook/Leads,
   Recent actor-known changes, and an input-hints footer. Stable regions so a player builds
   peripheral memory.
2. **Pane-to-view-model bindings** (all presentation-only unless noted): Place from `place_label`/
   `visible_exits`/`visible_doors`/`visible_containers`/`visible_items`; Self from `phase3a_status`
   and `carried_items`; Actions from `semantic_actions` (menu index is presentation-only, semantic id
   stable); Details/Why-not from `last_rejection_why_not`/`WhyNotView` (actor-safe only); Notebook from
   `NotebookView`/`NotebookLeadEntry`; Recent from `actor_known_interval_summary`
   (`TypedActorKnownIntervalSummary`); Co-present actors from Spec `0063`'s enriched `VisibleActor`.
3. **`ratatui` buffer render path** (`ratatui` added as a `tracewake-tui` dependency; a snapshot
   dev-dependency such as `insta`/`cargo-insta`): `render_embodied_to_buffer(&EmbodiedScreenModel,
   area) -> Buffer`, snapshotted at fixed sizes. No event loop yet.
4. **Responsive behavior:** collapse to a single-column stack on narrow terminals with headings and
   focus controls; never silently omit safety-critical facts (why-not, actions, needs); explicit
   truncation markers with detail expansion; no color-only semantics.

### 1.2 Out of scope (non-goals)

- No `crossterm` event loop or raw mode (Spec `0065`).
- No `UiIntent` reducer (Spec `0062`) — this spec renders; it does not read live input.
- No new core view-model fields; the actor pane's *final* content depends on Spec `0063` landing, but
  this spec introduces no core change of its own.
- No conversation panel (Spec `0067`); no debug dashboard (Spec `0068`).

## 2. Dependencies and ordering

- **Depends on:** `0061` (screen model/dump). The Co-present actors pane reaches its **final** state
  only after `0063`; until then it renders the id-only fallback with an explicit disposition.
- **Blocks:** `0065` (the shell draws these panes), and contributes surfaces to Spec `0069`.
- **Parallelizable with:** `0065` after the screen model stabilizes; `0063` runs alongside.

## 3. Doctrine anchors

- **Foundation 08**: embodied mode may show perceived place, exits, nearby actors, objects, traces,
  affordances, risks, actor-known beliefs/memories/rumors/records, available actions, event summaries,
  uncertainty/staleness/provenance labels, and why-not — and must hide hidden truth.
- **Architecture 10**: two-hop presentation disposition; captured-projection rule (no re-reading truth
  to freshen labels/routes/blockers); menu positions presentation-only; actor-safe why-not split from
  debug truth.
- **Reference risk register** (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`): non-vacuity — guards must
  prove forbidden behavior fails, not merely check table/list shape.

## 4. Findings and remediation requirements

- **4.1 Stable hierarchy over a flat list (seam: pane layout).** Urgent embodied facts (needs,
  interruption, why-not, actions) get priority regions; background info is peripheral.
- **4.2 Recognition over recall (seam: input-hints footer + always-visible actions).** Available keys/
  actions and focused pane are continuously shown.
- **4.3 Presentation-only fidelity (seam: pane bindings).** No pane invents routes, object contents,
  or blockers; each renders sealed view-model fields. The captured-projection rule holds.
- **4.4 Overflow is explicit (seam: responsive/truncation).** Truncation markers and detail expansion;
  no silent disappearance of safety/why-not/action data; narrow-size collapse tested.
- **4.5 Implementation decomposition.** Pane structs + layout; per-pane binding renderers; `ratatui`
  buffer path + snapshot harness; responsive/truncation behavior; disposition wiring for the `0063`
  actor pane. Separate tickets.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Architecture 13 / Execution 10** — recognize `ratatui` buffer snapshots as review artifacts, paired
  with structured pane dumps and non-vacuity witnesses (shared with Spec `0070`). Substance only.

## 6. Required fixtures and tests

- Snapshot goldens (pane text + `ratatui` buffer) at `80x24`, `100x30`, `60x20`.
- Actor-known-only assertion: no debug tokens in any pane.
- Overflow: a fixture whose actors/actions exceed the pane shows truncation markers, not silent loss.
- Narrow-collapse: `60x20` retains needs/why-not/actions.
- Non-vacuity: a layout that clips all actors/actions fails a snapshot/accessibility check.

## 7. Acceptance artifact and evidence

A review artifact recording the fixed-size pane/buffer snapshots, the actor-known-only negative, the
overflow/collapse tests, and the non-vacuity witnesses, at the exact implementation commit.

## 8. Implementation constraints

- TUI-only rendering/focus state. `ratatui` + snapshot dev-dep are the only new dependencies here; no
  UI dependency reaches `tracewake-core`/`tracewake-content`.
- If a pane discovers a missing core field, it routes back to a Spec `0063`-style contract rather than
  reading core state directly.
- Deterministic ordering of panes, actors, actions, and leads.

## 9. Risks and open questions

- **Risk: pane overload buries critical data.** Mitigation: stable priority ordering, truncation
  markers, detail focus, minimum-size floor.
- **Open question (§9.1 #7):** terminal-size floor — hard-fail below a floor with an actor-safe message;
  snapshot `80x24`, `100x30`, and a narrow fallback.
- **Open question (§9.1 #8):** theming — allowed, TUI-only, but all semantics must be present in text.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-065 (TUI is a primary product interface) | aligns | Delivers the at-a-glance embodied cockpit as a real client. |
| INV-069 (TUI must not implement simulation rules) | aligns | Panes render sealed view-model fields; no truth read or rule. |
| INV-025 (wrong beliefs are first-class) | aligns | Why-not/notebook panes present fallible actor-known belief, not truth. |
| INV-068 (debug mode visibly non-diegetic) | aligns | Embodied panes carry no debug tokens; debug is Spec `0068`. |
| INV-095 (TUI/view-model tests are acceptance tests) | aligns | Fixed-size pane/buffer snapshots are acceptance evidence with non-vacuity witnesses. |
