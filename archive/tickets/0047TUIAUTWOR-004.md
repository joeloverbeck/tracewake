# 0047TUIAUTWOR-004: Reference amendments (00/01)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — docs only: `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`, `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
**Deps**: None

## Problem

Spec 0047 §5 routes reference-tier amendments (substance + home). Per the driver report dispositions D-14/D-15: the review checklist (`00`) gains concise prompts so reviewers ask whether human time controls advance the same world step, preserve one charge per `(actor, need, tick)`, close open durations through shared authority, and summarize only actor-known changes; the design risk register (`01`) updates existing risk entries — pipeline bypass, epistemic leakage, prose authority, debug leakage, replay/projection erosion, TUI-playability erosion, no-human ordinary-life failure, and the temporal-control relapse cluster — naming this feature as a new mitigation/test surface, **without minting a new risk ID**. These review guardrails should land alongside the doctrine that introduces the capability.

## Assumption Reassessment (2026-06-22)

1. Both amendment targets exist: `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` and `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (confirmed present; the reference tier holds exactly `00`, `01`, `02_GLOSSARY.md`). Spec 0047 §5 explicitly leaves Glossary `02` unchanged — out of scope here.
2. Spec 0047 §5 enumerates the substance: reference `00` "add concise prompts"; reference `01` "update existing entries; mint no new ID." §2 lists both as reference anchors (`00` review checklist, `01` design risk register). This ticket carries the reference tier only.
3. Cross-artifact boundary under audit: the reference tier is review guardrail authority below foundation/architecture/execution. The new checklist prompts must mirror the obligations the higher tiers establish (one world step, one charge classification, shared open-duration closure, actor-known interval output) without inventing new requirements beyond them.
4. Constitutional invariants motivating the prompts: `INV-045` (one charge per affected need/tick — causal survival), `INV-024`/`INV-067` (actor-known-only interval output — no telepathy), `INV-103`/`INV-104` (same world step / no direct dispatch). The checklist prompts restate these as reviewer questions; the risk register entries map them to the existing risk clusters.
5. Enforcement surface governed by doctrine (substrate-only basis): the review checklist *is* a human enforcement surface for the no-leak / single-charge / no-direct-dispatch invariants. The amendment must keep the prompts framed as actor-known / shared-authority checks (e.g. "interval output is positive actor-known evidence, not redacted omniscience") so a reviewer applying them catches a leakage or double-charge regression rather than waving it through; it introduces no new risk identifier (`INV` / risk-ID minting is forbidden by §1.2).

## Architecture Check

1. Updating the existing risk register entries (rather than minting a new risk ID) keeps the temporal-control-relapse and epistemic-leakage risks in their established rows, where prior mitigations already accrue — a new ID would fragment the risk's history and contradict §5's "mint no new ID."
2. No backwards-compatibility aliasing/shims: checklist prompts are appended to the existing checklist; risk rows are extended in place.

## Verification Layers

1. `INV-045` single-charge -> invariants alignment check: the `00` checklist includes a "every `(actor, need, tick)` has one accounting classification" prompt.
2. `INV-024`/`INV-067` no-leak -> invariants alignment check: the `00` checklist includes an "interval output is positive actor-known evidence, not redacted omniscience" prompt.
3. Risk-register landing -> codebase grep-proof: `01` updates existing entries naming this feature as a mitigation/test surface and adds no new `RISK`/ID token.

## What to Change

### 1. Reference `00` — review checklist

Add concise prompts: all time controls use the same world step; every `(actor, need, tick)` has one accounting classification; open durations close through the shared authority; interval output is positive actor-known evidence, not redacted omniscience.

### 2. Reference `01` — design risk register

Update the existing entries for pipeline bypass, epistemic leakage, prose authority, debug leakage, replay/projection erosion, TUI-playability erosion, no-human ordinary-life failure, and the temporal-control relapse cluster — naming this feature as a new mitigation/test surface. Mint no new risk ID.

### 3. Execution precondition (recorded, not auto-applied)

Reference-tier amendments require ordinary reference-owner approval before application (`docs/README.md`). The ticket records substance + home.

## Files to Touch

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- Any code change — owned by 0047TUIAUTWOR-005 onward.
- Foundation/architecture/execution amendments (0047TUIAUTWOR-001/002/003).
- Glossary `02` change (§5: no change).
- Minting a new risk ID, `INV-###`, or gate code (§1.2 / §5 prohibition).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "world step|one accounting|actor-known|shared authority" docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` shows the four new prompts landed.
2. `grep -niE "temporal|relapse|leakage|pipeline bypass" docs/3-reference/01_DESIGN_RISK_REGISTER.md` shows the updated risk entries reference this feature.

### Invariants

1. No new risk identifier is introduced in `01` (the count of distinct `RISK`-style IDs is unchanged; entries are updated in place).
2. The checklist prompts restate existing higher-tier obligations only; they introduce no requirement absent from foundation/architecture/execution.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep landing + invariants-alignment review); the governed enforcement surfaces are implemented/tested by the code tickets and exercised by the parity/acceptance tickets 0047TUIAUTWOR-017/018.`

### Commands

1. `grep -ciE "world step|one accounting|actor-known|shared authority" docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
2. `grep -niE "relapse|temporal-control|leakage" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
3. Narrower command is correct: reference doc amendments are verified by landing greps + a manual alignment read confirming no new ID was minted — no code changes in this ticket.

## Outcome

Completed: 2026-06-22

Applied the reference-tier amendments:

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` now asks
  reviewers whether human time controls advance the same world step, preserve
  one accounting classification per `(actor, need, tick)`, close open durations
  through shared authority, and build interval output from positive
  actor-known evidence rather than redacted omniscience.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` extends existing risk entries
  for action-pipeline bypass, epistemic leakage, prose/presentation authority,
  debug leakage, replay/projection erosion, TUI-playability erosion,
  no-human ordinary-life failure, and temporal relapse. No new risk ID was
  minted; the register still has the existing `R-00` through `R-29` headings.

The ticket's reference-owner precondition was satisfied by the user's active
instruction to implement the `0047TUIAUTWOR` series against the referenced spec.
No code changes, glossary changes, foundation/architecture/execution
amendments, new risk ID, new invariant, or new gate code were introduced for
this ticket.

Verification:

- `grep -ciE "world step|one accounting|actor-known|shared authority" docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
  returned `5`.
- `grep -niE "relapse|temporal-control|leakage" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
  showed the updated risk and temporal-relapse entries.
- `rg -n '^### R-[0-9][0-9] ' docs/3-reference/01_DESIGN_RISK_REGISTER.md`
  showed the existing `R-00` through `R-29` headings with no new risk ID.
- Manual alignment read: the checklist and risk-register updates restate the
  higher-tier world-step, single-accounting, shared-duration, and actor-known
  interval-summary obligations without adding new doctrine.
