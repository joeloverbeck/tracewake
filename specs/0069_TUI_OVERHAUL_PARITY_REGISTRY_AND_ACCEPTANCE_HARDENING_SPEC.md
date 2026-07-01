# 0069 TUI Overhaul Parity Registry and Acceptance Hardening Spec

**Status**: PROPOSED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and promoted to `archive/specs/` on acceptance; it is never
promoted to the live `docs/4-specs/` tier. It is not a certification audit, mutation-remediation
pass, Phase-4 entry claim, or latest-`main` certification. It uses the sibling hardening-spec
house structure (`0046`, `0047`, `0057`).

This is **Spec I** of the TUI Experience Overhaul roadmap
(`reports/tui-experience-overhaul-research-report.md` §6, §7). It updates the playable-capability
parity registry, Hop 1 / Hop 2 evidence, golden/snapshot suites, screen-dump transcripts, and
anti-leak negatives so the whole overhaul is proven — extending the archived `0046` two-hop
playable-capability parity contract to the new fullscreen surface.

## 0. Baseline statement and source discipline

- **Driver.** The research report's agent-drivability/testability plan (§6) and non-vacuity
  requirements (§6.5), plus the archived `0046` parity contract and the existing TUI acceptance regime
  (`crates/tracewake-tui/tests/` — `tui_seam_conformance.rs`, `transcript_snapshot.rs`,
  `playable_capability_parity.rs`, `embodied_flow.rs`, `parity/`, `goldens/`, `adversarial_gates.rs`,
  `parity_adversarial.rs`).
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

Acceptance/conformance hardening across `tracewake-tui` (crate-crossing because it proves
core→view→render obligations):

1. **Parity registry extension:** add co-present observed activity (Spec `0063`) as a declared playable
   surface — core event/perception → `VisibleActor.observed_activity` → rendered actor pane — with Hop 1
   and Hop 2 evidence rows and anti-leak cases where truth has activity the actor does not know.
2. **`tui_seam_conformance` upgrade:** every `EmbodiedViewModel` field and every closed presentation enum
   has an explicit pane disposition; source/compile guards against wildcard/default laundering.
3. **`transcript_snapshot` upgrade:** snapshot `ScreenDump` plus transcript sections (mode, bound actor,
   fixed terminal size, key/intent inputs, semantic action ids, rendered pane text, debug markers,
   receipt/projection ids).
4. **`embodied_flow` upgrade:** drive the fullscreen/headless intent loop through wait/continue/action
   flows and prove the fullscreen UI does not fork semantics from line/script commands.
5. **Goldens:** replace/supplement linear outputs with fixed-size screen dumps and focused-pane snapshots;
   keep a text-only dump for agents at `80x24`, `100x30`, `60x20`.
6. **Debug/adversarial gates:** assert debug panels are non-diegetic and never appear in embodied dumps;
   add negatives for activity leakage and debug-to-embodied contamination.
7. **Default-launch flip confirmation:** confirm fullscreen-as-default is admissible only after the
   headless dump + intent driver are green and CI-stable (report §9.1 #5).

### 1.2 Out of scope (non-goals)

- No new product capability; this spec proves the capabilities delivered by Specs `0061`–`0068`.
- No certification-gate claim; gate codes are cross-references only.

## 2. Dependencies and ordering

- **Depends on / closes after:** `0063` (co-present activity), `0064` (panes), `0066` (time controls),
  `0068` (debug dashboard); it runs alongside all prior specs and closes once their surfaces land.
- **Blocks:** final acceptance of the overhaul; feeds the documentation settlement (Spec `0070`).

## 3. Doctrine anchors

- **Architecture 10 / 13** (`…/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`):
  transcript obligations, two-hop parity, and review-artifact posture.
- **Execution 07 / 10**: view-model proof and testing/observability/review artifacts.
- **Reference risk register** (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`): guards must prove
  forbidden behavior fails, not merely check shape.
- **Archived `0046`**: the two-hop playable-capability parity contract, cited as a cross-reference.

## 4. Findings and remediation requirements

- **4.1 Non-vacuity is mandatory (seam: every new guard).** Each new TUI guard carries a behavior witness
  or synthetic negative: removing `observed_activity` rendering fails Hop 2; building activity from
  physical truth fails anti-contamination; a hidden sleeping actor is not rendered; a debug activity panel
  cannot be invoked without authority nor feed embodied output; a stale activity without stale wording
  fails; a layout clipping all actors/actions fails.
- **4.2 Two-hop for the new surface (seam: parity registry).** Co-present activity gets Hop 1 and Hop 2
  rows plus anti-leak negatives.
- **4.3 Fixed snapshot sizes (seam: goldens).** At minimum `80x24`, `100x30`, `60x20`; deterministic
  ordering; explicit truncation markers; a TTY-free headless mode.
- **4.4 Implementation decomposition.** Parity-registry rows; seam-conformance disposition guard;
  transcript/dump snapshot upgrade; embodied-flow intent-loop drive; golden migration; debug/adversarial
  negatives. Separate tickets.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Execution 10 / Architecture 13** — screen dumps and buffer snapshots join transcript/golden artifacts
  with non-vacuity requirements (shared with Spec `0070`). Substance only.

## 6. Required fixtures and tests

- Parity rows for co-present activity (Hop 1 + Hop 2) with anti-leak negatives.
- Upgraded `tui_seam_conformance`, `transcript_snapshot`, `embodied_flow`, and goldens at fixed sizes.
- The six non-vacuity witnesses in §4.1, each proven to fail on the corresponding mutant/removal.

## 7. Acceptance artifact and evidence

A consolidated review artifact recording the extended parity registry, upgraded conformance/transcript/
flow tests, fixed-size goldens, and the non-vacuity witness results, at the exact implementation commit.
This is the overhaul's acceptance closer.

## 8. Implementation constraints

- Crate-crossing acceptance work; adds no product capability.
- Removal of an actor-activity field, pane disposition, stale label, debug marker, or headless dump must
  fail a real test — no decorative locks.

## 9. Risks and open questions

- **Risk: snapshot tests become brittle decoration.** Mitigation: pair buffer snapshots with structured
  pane dumps and synthetic negatives (report §6.5, risk register).
- **Open question (§9.1 #6):** snapshot churn budget — stable pane dumps are the primary golden; raw
  buffers are secondary visual review.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-094 (possession parity is tested) | aligns | Parity registry proves each possession's actor-known surface. |
| INV-095 (TUI/view-model tests are acceptance tests) | aligns | Screen dumps/transcripts are acceptance evidence. |
| INV-066 (every runnable phase has a TUI/view-model gate) | aligns | The new surface is gated by two-hop evidence. |
| INV-092 (deterministic replay is tested) | aligns | Deterministic dumps/ordering keep the surface replayable. |
| INV-100 (hidden-truth cognition forbidden) | aligns | Anti-leak negatives prove no activity truth leaks to embodied output. |
