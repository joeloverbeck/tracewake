# 0066 TUI Time-Control Presentation Modernization Spec

**Status**: PROPOSED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and promoted to `archive/specs/` on acceptance; it is never
promoted to the live `docs/4-specs/` tier. It is not a certification audit, mutation-remediation
pass, Phase-4 entry claim, or latest-`main` certification. It uses the sibling hardening-spec
house structure (`0046`, `0047`, `0057`).

This is **Spec F** of the TUI Experience Overhaul roadmap
(`reports/tui-experience-overhaul-research-report.md` §3.8, §7). It makes wait/continue and
interval summaries prominent and pleasant **without weakening** the settled `0047` kernel-owned
one-tick coordinator or its actor-known interval-summary contract.

## 0. Baseline statement and source discipline

- **Driver.** The research report's time-control section, built on archived spec `0047` (kernel-owned
  authoritative world advance and actor-known interval summaries) and its successor `0057`. This spec
  is presentation over that settled contract, not a redefinition of it.
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

In `tracewake-tui` (with a core change only if genuinely required):

1. **Prominent time controls:** a persistent control affordance for `w` wait one authoritative tick,
   `c` continue routine / advance-until-salient-stop, and (during continuation) `p` pause after the
   current tick — mapped to the Spec `0062` `WaitOneTick` / `ContinueUntil` intents and the settled
   kernel-owned continuation path.
2. **Recent actor-known changes pane** renders `actor_known_interval_summary`
   (`TypedActorKnownIntervalSummary`) after wait/continue — never a raw event diff and never a hidden
   due-queue.
3. **Optional closed time-control view metadata** in the view model **only** if presentation genuinely
   needs a field the summary does not already carry; otherwise stays TUI-only.

### 1.2 Out of scope (non-goals)

- No change to the `0047` one-world-tick coordinator, continuation stop conditions, or the definition
  of a wait/continue tick. This spec presents them; it does not move world rules into the TUI.
- No replacement of holder-known interval summaries with raw event diffs.
- Debug exact-tick reports stay in debug/operator mode (Spec `0068`).

## 2. Dependencies and ordering

- **Depends on:** `0061` (screen model), `0062` (`WaitOneTick`/`ContinueUntil` intents). Requires the
  current `0047` behavior to remain green.
- **Blocks:** contributes the time-control surface to Spec `0069`.
- **Parallelizable with:** `0064`/`0065`.

## 3. Doctrine anchors

- **Foundation 08**: waiting runs the simulation; acceleration/continuation is staged as repeated
  authoritative progression; sleep/interval summaries are actor-known with no omniscient leakage.
- **Architecture 10**: world-advancing controls route through core/runtime; actor-facing interval
  displays derive from holder-known/modeled observations/records/public cues, not raw diffs.
- **Archived spec `0047`**: the settled kernel-owned one-tick coordinator and actor-known interval
  summary contract, cited as a cross-reference only.

## 4. Findings and remediation requirements

- **4.1 Presentation, not authority (seam: time-control affordance).** The controls submit the existing
  intents; they do not define ticks or apply events.
- **4.2 Interval summary is holder-known (seam: Recent pane).** The pane renders
  `TypedActorKnownIntervalSummary` only; a mutant substituting a raw event diff fails an embodied test.
- **4.3 One authoritative tick (seam: `WaitOneTick`).** `wait` is one authoritative world tick, not a
  local possessed-actor clock; `continue` is repeated authoritative one-tick progression with
  deterministic stop conditions.
- **4.4 Implementation decomposition.** Time-control affordance + intent wiring; Recent-pane interval
  rendering; optional closed view metadata (only if required). Separate tickets.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Architecture 10** — if closed time-control view metadata is added, route it as a two-hop-disposed
  view-model field (shared with Spec `0070`). Substance only; likely unnecessary.

## 6. Required fixtures and tests

- `wait` script proves one authoritative world-tick progression; `continue` script proves repeated
  authoritative one-tick progression with a deterministic stop.
- The Recent pane shows the actor-known interval summary; negative test: no raw event diff or hidden
  due-queue appears in embodied mode.
- `0047`/`0057` regressions remain green.

## 7. Acceptance artifact and evidence

A review artifact recording the wait/continue progression scripts, the interval-summary rendering, the
no-raw-diff negative, and the preserved `0047`/`0057` behavior, at the exact implementation commit.

## 8. Implementation constraints

- Mostly TUI-only; a core change is admissible only for genuinely required closed time-control view
  metadata, additive with two-hop disposition.
- No weakening of kernel-owned time advancement or continuation stop conditions.

## 9. Risks and open questions

- **Risk: opaque time passage returns** if the interval pane is dropped under layout pressure.
  Mitigation: the Recent pane is a required, non-truncatable safety surface.
- **Open question:** whether any closed time-control metadata is needed at all — default is no.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-112 (holder-known time must plan) | aligns | Interval displays derive from holder-known summaries, not free truth time. |
| INV-069 (TUI must not implement simulation rules) | aligns | Controls submit intents; the kernel owns the tick. |
| INV-099 (truth may validate, not plan) | aligns | No hidden due-queue or raw diff enters embodied presentation. |
| INV-018 (deterministic replay is foundational) | aligns | Wait/continue remain authoritative, deterministic, replayable ticks. |
