# 0027ARCTIEDOC-004: State observation-time embodied affordance snapshots in architecture 10

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edit to `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (observation-time snapshot/capture rule for embodied view models + semantic-action entries). No crate/code, no fixtures.
**Deps**: None (ticket-internal; cross-references A03's pre-existing sealed holder-known context). **Execution-blocking precondition**: owner approval per spec 0027 §R-A (architecture tier-1 doctrine; not by convention).

## Problem

D5 (report A10-C). The truth firewall (foundation doc 14; `INV-099`/`INV-108`) forbids hidden truth generating embodied affordance menus, and A10 already says embodied view models are generated from a holder-known context, semantic actions go through the ordinary proposal pipeline, why-not output must not reveal hidden truth, and debug views are separate. But A10 does not explicitly require that embodied view construction and semantic-action entries rely on **observation-time snapshots** of visible place / carried-item / container / attribute facts, rather than a live physical-state handle re-read at render/preflight (verified: A10 generates views "from a holder-known context" but states no observation-time snapshot requirement at `fdfd0b9`). A00 records this as hardening evidence (0024/0025 embodied truth-access removal + carrier capture); A10 should own the view-model contract.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`fdfd0b9`): `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` says embodied view models are generated from a holder-known context for the bound actor plus permitted projections, semantic actions go through the shared proposal pipeline, why-not output must not reveal hidden true targets / validator-only detail, and debug views are structurally separate — but contains no observation-time snapshot/capture requirement. A04 already defines the two affordance layers (perceived/holder-known vs actual validation); A00 records the 0024/0025 embodied truth-access-removal + carrier-capture hardening rows.
2. Verified against spec 0027 §3 D5 + §7 R-E and source report §4.5 (A10-C). D5's home is A10 with a cross-reference to A03's sealed holder-known context (pre-existing — no Dep on the D3 sufficiency ticket). D5 explicitly does NOT decide the deferred possession-bind perception question (spec §7 R-E; A00:129) — that stays an owner decision.
3. Shared boundary under audit: the A10 embodied view-model / semantic-action construction contract against A03's sealed holder-known context. The capture rule must say A10 consumes holder-known context + projection records whose attributes were captured at a modeled observation/bind/preflight/perception boundary — not a live truth handle re-read at render.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-099` (truth may validate, not plan) and `INV-100` (hidden-truth cognition forbidden) — embodied affordance enumeration must come from captured holder-known material, not live truth; `INV-108` (human possession is cognition-neutral) and `INV-006` (possession transfers no world knowledge) — possession is not a knowledge upgrade, so the same capture rule applies to embodied possession and no-human parity surfaces alike; `INV-070` (why-not explanations) — rejection feedback splits actor-visible modeled feedback from debug-only detail. Additive; no invariant weakened (spec §4).
5. Actor-knowledge / no-leak surface (governed here, enforcement deferred to execution `04`/`07`/`10`): the TUI may render observed labels/attributes and actor-known affordances from captured records but must not hold a live physical-state handle or re-read truth to "freshen" labels, carried-item attributes, routes, workplace availability, food sources, or hidden blockers (INV-024 no telepathy / INV-067 embodied shows actor-known reality / INV-093 leakage is high-severity). Validators may still reject a selected semantic action using authoritative truth, but rejection feedback splits actor-visible modeled feedback from debug-only detail (INV-106). This ticket adds doctrine only; no leakage or nondeterminism path. It does not settle the possession-bind perception question (R-E) — once a modeled observation/preflight/capture exists, A10 must use that captured material rather than live truth.

## Architecture Check

1. Stating the snapshot/capture rule directly in A10 (the view-model / possession owner) — rather than leaving it implicit in "generated from holder-known context" or only in A00's hardening rows — gives future view-model authors the contract without reading the conformance table. The alternative (leave it in A00 only) is exactly the fragmentation this spec corrects.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine matching the already-landed 0024/0025 embodied-capture behavior; no compatibility layer.

## Verification Layers

1. `INV-099`/`INV-100`/`INV-108` truth-firewall on embodied menus (D5) → invariants alignment check: A10 requires embodied view models + semantic-action entries to consume observation/bind/preflight-captured holder-known material, never a live truth handle.
2. `INV-024`/`INV-067`/`INV-093` no-leak embodied reality → manual review (epistemic-leakage audit): the TUI renders only captured observed labels/affordances; no truth re-read to freshen labels/attributes/routes/availability/blockers; rejection feedback splits actor-visible from debug-only (INV-106).
3. Already-landed embodied carrier capture (A00 0024/0025 rows) → codebase grep-proof: A00 records the embodied truth-access removal + carrier capture, confirming D5 documents implemented behavior.
4. Single doc, additive: no replay/golden-fixture or skill-dry-run layer here (deferred to execution `04`/`07`/`10`); the layers above map each engaged invariant to a distinct surface.

## What to Change

### 1. D5 — observation-time snapshot/capture rule in A10

Add to A10's embodied view-model / semantic-action sections:

- Embodied view models and semantic-action entries consume holder-known context plus permitted projection records whose visible / carried-item / container / current-place attributes were captured at a modeled observation, bind/preflight, or perception boundary.
- The TUI renders observed labels/attributes and actor-known affordances from those captured records; it must not hold a live physical-state handle or re-read truth to "freshen" labels, carried-item attributes, routes, workplace availability, food sources, or hidden blockers.
- Validators may still reject a selected semantic action using authoritative truth, but rejection feedback splits actor-visible modeled feedback from debug-only detail.
- Snapshot/capture applies to no-human parity surfaces and embodied possession alike (possession is not a knowledge upgrade).
- Cross-reference A03's sealed holder-known context. Does not decide the deferred possession-bind perception question (§7 R-E).

## Files to Touch

- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (modify)

## Out of Scope

- **Possession-bind perception decision (spec §7 R-E; A00:129)** — a deferred owner decision; this ticket preserves it, does not settle it.
- **Wallhack negatives / observation-time snapshot proof / embodied carrier census** — execution `04`/`07`/`10` (spec §6, V4).
- **D3 provenance-sufficiency / D4 freshness** (A03/A06) — sibling ticket 0027ARCTIEDOC-003.
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Landing grep** — A10 carries the observation-time snapshot/capture rule: `grep -niE "observation-time|captured at|snapshot|live .*handle" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` resolves it.
2. **Possession-bind preservation** — the deferred R-E question is not silently decided: the new rule says capture-then-use, not bind-emits-perception; `grep -niE "bind-time perception" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` shows no new resolution of that question.
3. **Invariants alignment review** — D5 keeps hidden truth out of embodied affordance enumeration (INV-099/100/108), no-leak holds (INV-024/067/093), why-not split preserved (INV-106/070).

### Invariants

1. Embodied view models / semantic-action entries consume observation/bind/preflight-captured holder-known material; no live truth re-read to freshen labels/attributes/routes/availability/blockers (INV-099/100/024/067).
2. Possession is not a knowledge upgrade — the capture rule is identical for embodied possession and no-human parity surfaces (INV-108/006); the deferred possession-bind perception question (R-E) is untouched.

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing grep + possession-bind preservation grep) plus an epistemic-leakage / invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "observation-time|captured at|snapshot|live .*handle" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — confirms D5 landed.
2. `grep -niE "bind-time perception" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — confirms the deferred R-E question is not silently decided.
3. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the two greps plus the epistemic-leakage / invariants-alignment review.`

## Outcome

Completed: 2026-06-13

Added A10's observation/preflight/perception capture rule for embodied view
models and semantic action entries. The TUI must render observed labels,
attributes, actor-known affordances, routes, workplace availability, food
sources, and blockers from captured holder-known/projection records rather than
holding a live physical-state handle or truth-refreshed data. The amendment
also records the actor-visible/debug-only rejection split and states that
possession is not a knowledge upgrade.

The user-provided implementation goal was treated as owner approval to enact
the tier-1 architecture amendment. No crate/code, fixture, execution,
foundation, or reference changes were made.

Verification:

- `grep -niE "observation-time|captured at|snapshot|live .*handle" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `grep -niE "bind-time perception" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` returned no matches.
- `rg -n "0024|0025|embodied truth|carrier capture|believed-access embodied availability" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`

Deviation: none.
