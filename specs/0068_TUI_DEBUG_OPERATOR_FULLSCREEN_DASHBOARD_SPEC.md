# 0068 TUI Debug Operator Fullscreen Dashboard Spec

**Status**: PROPOSED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and promoted to `archive/specs/` on acceptance; it is never
promoted to the live `docs/4-specs/` tier. It is not a certification audit, mutation-remediation
pass, Phase-4 entry claim, or latest-`main` certification. It uses the sibling hardening-spec
house structure (`0046`, `0047`, `0057`).

This is **Spec H** of the TUI Experience Overhaul roadmap
(`reports/tui-experience-overhaul-research-report.md` §3.10, §7). It replaces command-emitted debug
text with a **visibly non-diegetic** fullscreen debug/operator dashboard, quarantined from embodied
play and gated behind debug authority.

## 0. Baseline statement and source discipline

- **Driver.** The research report's debug/operator redesign: debug panels exist today
  (`crates/tracewake-tui/src/debug_panels.rs`) but are selected by command and emitted as line text
  rather than as permanent, visibly non-diegetic panes.
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

In `tracewake-tui` (over existing debug view models and debug-authority APIs):

1. **Fullscreen debug dashboard/tabs:** event log / replay frontier; projection rebuild;
   holder-known context inspector (compare actor-known vs provenance); beliefs/observations/
   contradictions (existing `DebugBeliefsView`, `DebugObservationsView`, `DebugEpistemicsView`);
   action validation / why-not (validator reports split actor-facing vs debug-only); scheduler /
   no-human diagnostics; and TUI-seam diagnostics (current screen model, pane dispositions, render
   snapshot metadata).
2. **Unmistakable non-diegetic framing:** every debug screen carries a `DEBUG NON-DIEGETIC` (or
   equivalent locked) marker and cannot be confused with embodied mode.
3. **Authority gating:** debug screens require debug authority; they may show truth for inspection but
   must not write actor-known beliefs, create records/rumors/notices, or alter embodied rendering.

### 1.2 Out of scope (non-goals)

- No new debug *view models* unless explicitly required; the dashboard renders existing debug view
  models and authority APIs.
- No debug output in embodied screen dumps.
- No debug pathway that produces actor knowledge or feeds embodied output.

## 2. Dependencies and ordering

- **Depends on:** `0061` (screen model/dump), `0062` (intents), `0065` (fullscreen shell hosts the
  dashboard). Independent of Spec `0063`.
- **Blocks:** contributes debug-quarantine surfaces to Spec `0069`.

## 3. Doctrine anchors

- **Foundation 08**: permanent debug surfaces that are visibly non-diegetic and quarantined from
  embodied play.
- **Architecture 10**: debug split; debug markers; debug authority; debug outputs never feed embodied
  rendering.
- **Execution 04 / 07**: anti-contamination and debug-quarantine proof obligations.

## 4. Findings and remediation requirements

- **4.1 Permanent, marked, separate (seam: dashboard).** Debug is a separate fullscreen surface with a
  locked non-diegetic marker on every screen, never a mode the player can confuse with embodied play.
- **4.2 Authority-gated inspection (seam: debug APIs).** Debug requires authority and is inspection-only;
  it writes no actor-known belief and creates no diegetic record.
- **4.3 No bleed into embodied (seam: dump boundary).** Debug outputs never appear in embodied screen
  dumps; a mutant that renders a debug panel in embodied mode fails a negative test.
- **4.4 Implementation decomposition.** Dashboard shell + tab model; per-tab renderers over existing
  debug view models; non-diegetic marker enforcement; authority-gate wiring; anti-bleed negatives.
  Separate tickets.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Architecture 10** — recognize the debug dashboard and TUI-seam diagnostics tab as permanent
  non-diegetic client surfaces with authority gating (shared with Spec `0070`). Substance only.

## 6. Required fixtures and tests

- Every debug screen renders the non-diegetic marker (positive).
- Debug is unavailable without authority (negative).
- No debug token appears in any embodied dump (negative; extends `family_debug_quarantine` golden).
- Debug inspection creates no actor-known belief/record (anti-contamination).

## 7. Acceptance artifact and evidence

A review artifact recording marker presence on every debug screen, authority-gating, embodied-dump
quarantine, and the no-actor-knowledge-from-debug negative, at the exact implementation commit.

## 8. Implementation constraints

- Mostly TUI-only over existing debug view models/authority APIs; new debug view models only if
  explicitly required.
- No shared debug/embodied pane component unless the marker policy is enforced on it.

## 9. Risks and open questions

- **Risk: debug truth visually bleeds into embodied mode** as pane count grows. Mitigation: permanent
  mode split, markers, authority checks, negative tests, no unmarked shared components.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-068 (debug mode visibly non-diegetic) | aligns | Locked marker on every debug screen. |
| INV-107 (debug omniscience is quarantined) | aligns | Inspection-only; creates no actor knowledge/records. |
| INV-031 (human/debug notes are non-diegetic) | aligns | Debug dashboard is framed non-diegetic and separate. |
| INV-100 (hidden-truth cognition forbidden) | aligns | Debug truth never feeds embodied cognition/rendering. |
| INV-041 (agent decisions need debug traces) | aligns | Dashboard surfaces typed decision/why-not/scheduler diagnostics. |
