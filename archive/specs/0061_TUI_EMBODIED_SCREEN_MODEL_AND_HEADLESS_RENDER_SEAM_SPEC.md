# 0061 TUI Embodied Screen Model and Headless Render Seam Spec

**Status**: COMPLETED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and is promoted to `archive/specs/` on acceptance; it is
never promoted to the live `docs/4-specs/` tier. It is **not** a certification audit, a
mutation-remediation pass, a Phase-4 entry claim, or a latest-`main` certification. It does
not amend constitutional invariants, define gate semantics, or weaken execution gates. It
uses the canonical hardening-spec house structure of its sibling specs (e.g. `0046`, `0047`,
`0057`), not the `docs/NN_*` narrative-document style.

This is **Spec A** of the TUI Experience Overhaul roadmap (`reports/tui-experience-overhaul-research-report.md`
§7). It is the **root** of that roadmap: it introduces the pure screen-model and headless
render/dump seam that every later overhaul spec (`0062`–`0070`) consumes. It deliberately adds
**no** interactive event loop and **no** core view-model change.

## 0. Baseline statement and source discipline

- **Driver.** The TUI Experience Overhaul research report, which recommends replacing the
  line-REPL transcript with a pane-oriented interface built on a preserved pure render seam.
  The report is analysis, not doctrine; it mints no invariant, gate code, risk id, or glossary
  term. This spec operationalizes it.
- **Baseline commit.** Re-verified against repository `HEAD`
  `85dd8836508a58305eb84d70caf52bda088a9bde` at authoring time. The named symbols below are
  authoritative; any line numbers are provenance only and are not relied upon.
- **Source discipline.** A commit hash named here is audit/provenance only. This spec
  recommends and scopes work; it declares no latest-`main` certification, Phase-4 entry, or
  second-proof entry. When executed, the implementation must name its own exact implementation
  commit. A manifest is path inventory only; branch/default-branch/code-search evidence is not
  proof of exact-commit content.
- **Authority posture.** Subordinate to `docs/0-foundation/`, `docs/1-architecture/`,
  `docs/2-execution/`, and `docs/3-reference/`, in that order. It operationalizes existing
  doctrine; where it identifies doctrine that must be sharpened, it routes the amendment to the
  owning tier (§5) as *substance + home* and does not ratify wording or mint identifiers.
- **Execution admissibility posture.** `P0-CERT not applicable`. This spec certifies no
  certification gate; it names gate codes and prior certifications only as cross-references.
- **Ledger timing.** Per the sibling-spec convention, this package receives its
  `docs/4-specs/SPEC_LEDGER.md` archived-row entry at acceptance/closeout, not at proposal.
  This spec authors no ledger row now and makes no change to live `0001` or the ledger.

## 1. Scope

### 1.1 In scope

Introduce, in `tracewake-tui` only, a pure screen-model and headless render/dump layer between
the existing `EmbodiedViewModel` and any future terminal drawing:

1. **`EmbodiedScreenModel`** — a presentation-only structure built purely from an
   `&EmbodiedViewModel`, grouping its existing fields (`place_label`, `place_id`, `visible_exits`,
   `visible_doors`, `visible_containers`, `visible_items`, `carried_items`, `local_actors`,
   `semantic_actions`, `phase3a_status`, `last_rejection_summary`, `last_rejection_why_not`,
   `notebook`, `actor_known_interval_summary`, `debug_available`) into named panes. This list is
   the current public field set; the closed-disposition guard (§4.4) additionally covers the
   crate-private fields `render_embodied_view` already consumes through sealed accessors
   (`viewer_actor_id`, `sim_tick`, `holder_known_context_id`/`_hash`/`_frontier`/`_source_summary`),
   which must each be rendered or explicitly suppressed. It holds no simulation authority and no
   physical-state handle.
2. **`build_embodied_screen_model(view: &EmbodiedViewModel, opts: RenderOptions) -> EmbodiedScreenModel`**
   — deterministic, side-effect-free. `RenderOptions` carries only presentation inputs
   (terminal size, focused pane, theme flags), never a truth handle.
3. **Plain-text screen dump** — `render_embodied_screen_dump(&EmbodiedScreenModel) -> String`
   producing the stable `SCREEN … / PANE …` textual form an agent reads (report §6.2).
4. **Structured screen dump** — a `ScreenDump` value (mode, terminal size, focused pane, per-pane
   dumps, action refs, `debug_marker_present`, view-model id, holder-known context hash) that
   tests and tools parse. Embodied dumps carry only actor-known metadata already on the view
   model; no debug-only world truth. The holder-known *frontier* and *source summary* accessors
   (`holder_known_context_frontier`, `holder_known_context_source_summary`) are deliberately
   excluded from `ScreenDump` to bound snapshot churn (§9.1 #6); only the stable context hash is
   carried.
5. **Preserve the existing pure seam.** `render_embodied_view(&EmbodiedViewModel) -> String`
   (`crates/tracewake-tui/src/render.rs`) remains a public pure function **and remains the
   fat, field-naming site**: it keeps its `#[deny(unused_variables)]` annotation and its sealed
   accessor usage, so the existing `render_embodied_view_uses_sealed_view_model_accessors`
   conformance guard (`crates/tracewake-tui/tests/tui_seam_conformance.rs`) continues to hold
   unchanged. `build_embodied_screen_model` and the dump renderers are added *alongside* it (a
   preserved parallel render), not as a wrapper that would strip the field-naming guard; the seam
   must not be swallowed by an event loop. `TuiApp::render_current_view`
   (`crates/tracewake-tui/src/app.rs`) continues to route through the pure path.
6. **Debug-token hygiene preserved.** The existing guarantee that `DEBUG NON-DIEGETIC` markers do
   not appear in embodied output extends to every new embodied dump.

### 1.2 Out of scope (non-goals)

- No `ratatui` or `crossterm` dependency yet (Specs `0065` add the shell; a `ratatui` buffer path
  is `0064`'s concern). Stage A is framework-free.
- No `UiIntent` reducer or input handling (that is Spec `0062`).
- No change to any `tracewake-core` or `tracewake-content` type. `VisibleActor` stays id-only here;
  its enrichment is Spec `0063`.
- No new pane content that is not already present on `EmbodiedViewModel`.

## 2. Dependencies and ordering

- **Depends on:** none. This spec lands **first**.
- **Blocks:** `0062` (intent driver renders through this seam), `0064` (pane layout consumes the
  screen model), `0065` (shell redraws from it), `0066`, `0068`, and `0069` (acceptance).
- **Parallelizable with:** `0063` may begin once this spec's `EmbodiedScreenModel`/`ScreenDump`
  field contract is published, but `0063`'s rendered actor pane is not complete until `0064`.

## 3. Doctrine anchors

- **Foundation 08** (`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`): the TUI is
  the first real client over typed view models, not a disposable debug console; embodied output is
  actor-filtered.
- **Architecture 10** (`docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`):
  the embodied view-model schema, the two-hop presentation-disposition rule, and the
  captured-projection rule forbidding the TUI from re-reading truth to freshen labels.
- **Execution 07 / 10** (`docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`,
  `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`): view-model
  proof obligations and review-artifact discipline; screen dumps join golden/transcript artifacts.

## 4. Findings and remediation requirements

- **4.1 Pure seam as an asset, not a casualty (seam: `crates/tracewake-tui/src/render.rs`).** The
  current `render_embodied_view` is a deliberately testable pure function with debug-token hygiene.
  The overhaul must build *on top of* it, not replace it with an event-loop render path.
- **4.2 Screen model is derived, never authoritative (seam: new `tracewake-tui` screen-model
  module).** `build_embodied_screen_model` takes `&EmbodiedViewModel` by shared reference and
  produces presentation structure only. It holds no `PhysicalState`/session handle and performs no
  truth read.
- **4.3 Two dumps, one source (seam: dump renderers).** Plain-text and structured `ScreenDump` are
  two projections of the same `EmbodiedScreenModel`; they must not diverge in what they claim.
- **4.4 Every current `EmbodiedViewModel` field has a pane disposition (seam: existing
  `crates/tracewake-tui/tests/tui_seam_conformance.rs`).** A new, distinctly-named test —
  `embodied_screen_model_field_disposition` — is *added to* that existing file and registered in
  its `TUI_SEAM_EVIDENCE` table under a new requirement id, alongside the current guards
  `render_embodied_view_uses_sealed_view_model_accessors` (already proves `render_embodied_view`
  names every field with no rest/wildcard omission) and
  `closed_presentation_enum_matches_are_exhaustive_without_wildcards` (already forbids wildcard
  laundering of closed presentation enums). The new test must prove each existing
  `EmbodiedViewModel` field — public and crate-private — is assigned to a named
  `EmbodiedScreenModel` pane (rendered or explicitly suppressed), so later field additions cannot
  be laundered through a wildcard/default arm. It does not introduce a colliding second
  `tui_seam_conformance` seam.
- **4.5 Implementation decomposition (one ticket per reviewable diff).** Screen-model types +
  builder; plain-text dump; structured `ScreenDump`; `render_embodied_view` reframed as wrapper;
  conformance/disposition guard. Each is a separately reviewable ticket.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Architecture 10** — recognize the screen model and headless dump as legitimate client
  rendering artifacts derived from the embodied view model, with closed pane disposition mandatory.
  Substance only; wording is ratified at the eventual tier edit (also carried by Spec `0070`).

## 6. Required fixtures and tests

- Fixed-size plain-text dump goldens at `80x24`, `100x30`, and a narrow `60x20`.
- Structured `ScreenDump` snapshot for at least one embodied fixture (e.g. `ordinary_workday_001`).
- `embodied_screen_model_field_disposition` (new test added to the existing
  `crates/tracewake-tui/tests/tui_seam_conformance.rs` and registered in `TUI_SEAM_EVIDENCE`):
  every `EmbodiedViewModel` field maps to a pane disposition; source/compile guard against
  wildcard/default laundering. It complements — does not replace — the existing
  `render_embodied_view_uses_sealed_view_model_accessors` guard.
- Negative: no `DEBUG NON-DIEGETIC` token in any embodied dump.
- Determinism: repeated `build_embodied_screen_model` + dump on the same view model is byte-identical.

## 7. Acceptance artifact and evidence

A review artifact recording: the fixed-size dump goldens, the structured `ScreenDump` snapshot, the
field-disposition conformance result, and the debug-token-absence negative — all at the exact
implementation commit. Screen dumps are declared first-class review artifacts, not decorative locks.

## 8. Implementation constraints

- TUI-only. No new dependency on `tracewake-core`/`tracewake-content` types beyond what
  `EmbodiedViewModel` already exposes.
- No `ratatui`/`crossterm` in this spec.
- No physical-state handle in the screen-model layer.
- Deterministic ordering of panes, actors, actions, and leads in dumps.

## 9. Risks and open questions

- **Risk: the shell later swallows the pure seam.** Mitigation: land this seam first and keep the
  event loop (Spec `0065`) strictly an adapter around it.
- **Open question (report §9.1 #6): snapshot churn budget.** Recommendation carried forward: stable
  pane text dumps are the primary golden; raw buffers (Spec `0064`) are secondary visual review.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-065 (TUI is a primary product interface) | aligns | Strengthens the client seam without moving rules into it. |
| INV-069 (TUI must not implement simulation rules) | aligns | Screen model is pure presentation over `EmbodiedViewModel`; no truth read. |
| INV-068 (debug mode visibly non-diegetic) | aligns | Debug-token hygiene extends to every embodied dump. |
| INV-095 (TUI/view-model tests are acceptance tests) | aligns | Screen dumps become acceptance artifacts with disposition conformance. |
| INV-099 (truth may validate, not plan) | aligns | Screen model never reads hidden truth; renders sealed view-model fields only. |

## Outcome

Completed: 2026-07-01

Implemented the scoped Spec 0061 TUI screen-model seam in dependency order:

- `0061TUIEMBSCR-001` added `tracewake-tui::screen::model`, `EmbodiedScreenModel`,
  `RenderOptions`, and `build_embodied_screen_model(&EmbodiedViewModel, RenderOptions)`.
- `0061TUIEMBSCR-002` added the framework-free plain-text `SCREEN` / `PANE` dump renderer.
- `0061TUIEMBSCR-003` added the structured `ScreenDump` projection over the same screen-model pane source.
- `0061TUIEMBSCR-004` added `embodied_screen_model_field_disposition` to the existing
  `tui_seam_conformance` registry.
- `0061TUIEMBSCR-005` added the `ordinary_workday_001` fixed-size goldens, structured snapshot,
  acceptance test, source-census classification update, and acceptance artifact.

Commit roles:

- Spec correction commit: `2dda85f`.
- Implementation baseline commits: `f441cc6`, `0f508b6`, and `8b697b8`.
- Conformance/test guard commit: `5b479c8`.
- Evidence/report commit: `82e40c0ef6202699000076e378bb809230f0aab6`.
- Archive/truthing commit: this commit moving the report/spec and repairing references.

Acceptance artifact:

- `archive/reports/0061_tui_embodied_screen_model_acceptance.md`.

Verification:

- `cargo test -p tracewake-tui --test embodied_screen_dump` passed after the report existed.
- `cargo test -p tracewake-tui` passed after the report existed.
- `cargo test --workspace` passed after the report existed.
- Earlier, `cargo test --workspace` exposed the required source-census update for the new
  `crates/tracewake-tui/src/screen/*.rs` production files; after adding them to the existing TUI
  source classification table,
  `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree`
  passed.

Deviations:

- The plain-text dump remains a preserved parallel render alongside `render_embodied_view`; it does
  not wrap or replace the older pure render seam.
- `ScreenDump` carries the stable holder-known context hash and deliberately omits holder-known
  frontier/source-summary fields to bound snapshot churn, as specified.

This scoped feature evidence does not certify latest main, Phase-4 entry, second-proof entry,
institutions, notices, travel, LOD, LLM/speech, story-sifting, `P0-CERT`, `FIRST-PROOF-CERT`, or
any whole-project status; it mints no invariant, gate, glossary term, or risk ID.
