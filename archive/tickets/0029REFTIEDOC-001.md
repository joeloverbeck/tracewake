# 0029REFTIEDOC-001: Glossary canonical term — observer-only emergence evidence + EMERGE-OBS lookup

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edits to `docs/3-reference/02_GLOSSARY.md` (one canonical context-bound term plus a compact execution-label lookup entry). No crate/code, no fixtures.
**Deps**: None. **Execution-blocking precondition**: owner approval per spec 0029 §R-A (reference enactment is tier-3 owner approval, not constitutional sign-off; this ticket stages the substance, it does not authorize the edit).

## Problem

D1 (report F01). Foundation `INV-111` (ratified, `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`), the architecture `13` observer-only emergence-evidence + typed-observability contract (ratified, `0027`), and the execution `00`/`10` `EMERGE-OBS` mechanics (ratified, `0028`) have established an artifact class — after-the-fact, observer-only evidence that unscripted ordinary-life phenomena arise from modeled causes — that the **reference glossary does not yet name**. Verified at `HEAD` `36b4082`: `docs/3-reference/02_GLOSSARY.md` carries distinct `Evidence` (L179), `observation` (L56), `projection` (L32), and `Story sifting` (L161) entries but `grep -niE 'emergence|observer-only|EMERGE-OBS'` returns 0. With no canonical name and no upward cross-reference, the artifact class is free to be collapsed into one of those neighbors during later implementation. This ticket adds the canonical term plus a compact `EMERGE-OBS` execution-label lookup, both cross-referencing upward and both staying inside the glossary's naming-only authority boundary.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`36b4082`): `docs/3-reference/02_GLOSSARY.md` has a `## Canonical terms` table (L62, containing `Story sifting` L161) and a `## Canonical context-bound and deferred terms` table (L173, containing `Evidence` L179 and `Proof` L185); `grep -niE 'emergence|observer-only|EMERGE-OBS' docs/3-reference/02_GLOSSARY.md` returns 0 — the routed term and the label lookup are both absent. The glossary's `## Authority boundary` and `## Maintenance rule` (L228) declare a naming-only authority that forbids smuggling doctrine/architecture/gate semantics into reference (the altitude this term must respect).
2. Verified against spec 0029 (`specs/0029_REFERENCE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D1 / §7 R-B / R-C and the source report `reports/reference-tier-alignment-research-report.md` F01. Upstream sources to cross-reference (all confirmed to exist at the cited tiers): foundation `02` (`INV-111`, L459) / `09` / `12`; architecture `13` (names the **observer-only emergence-evidence record** at L37/39 with the five properties); execution `00` (`EMERGE-OBS` label row L92) / `10` (`## Emergence-evidence ledger \`EMERGE-OBS\`` L98).
3. Shared boundary under audit: the reference glossary's naming surface (`02_GLOSSARY.md`) and the upstream artifact definitions it must point to without restating — `INV-111` (foundation), the architecture `13` observer-only record, and the execution `00`/`10` `EMERGE-OBS` semantics. The glossary names the class and cross-references; it does not redefine `INV-111`, define `EMERGE-OBS` gate/obligation semantics, or coin any gate.
4. Constitutional invariant motivating this ticket, restated before trusting the narrative: `INV-111` — "Living-world acceptance requires observer-only emergence evidence"; that evidence "must be retrospective, explainable through event-log ancestry, and unable to feed simulation behavior, author outcomes, or set dramatic objectives." The glossary term encodes exactly those properties at the terminology surface so the class is not collapsed into `evidence`/`observation`/`projection`/`story sifting` during implementation.
5. No-leak / truth-firewall surface (substrate-only — the enforcement surfaces are the execution `10` non-input rule and the foundation `14` truth firewall, owned upstream, not by this glossary entry): the term's **non-steering** and **structurally-non-input** properties name the firewall obligation (`INV-099`: truth may validate, not plan; `INV-111`: evidence cannot feed simulation behavior). This entry introduces no path by which the artifact reaches cognition/scheduler/validators/authoring/selection; it records the prohibition and points to execution `10` for the rule. It is naming guidance, not an enforcement mechanism — it weakens no firewall and adds none.

## Architecture Check

1. The canonical term lives in the glossary's `Canonical context-bound and deferred terms` table because the artifact is itself context-bound (retrospective, acceptance-context) — co-located with `Evidence` (L179) and `Proof` (L185) where a reviewer looking up acceptance vocabulary will find it; `Story sifting` sits one table up among the plain `Canonical terms`, so the entry explicitly distinguishes itself from that neighbor. Naming the artifact as the glossary class name for architecture `13`'s "observer-only emergence-evidence record" prevents the tiers forking the name. This is cleaner than placing it among generic canonical terms (which would imply context-free authority) or restating `INV-111` doctrine in reference (which the glossary's maintenance rule forbids).
2. No backwards-compatibility aliasing/shims: purely additive glossary entries that point upward by reference rather than restating doctrine; no existing term is renamed, removed, or weakened.

## Verification Layers

1. `INV-111` observer-only emergence evidence → invariants alignment check + codebase grep-proof: the glossary defines **observer-only emergence evidence** carrying the five properties (retrospective / non-certifying / ancestry-backed / non-steering / structurally-non-input) and cross-references foundation `02`, architecture `13`, execution `00`/`10`.
2. Naming-only authority boundary → manual review: the entry names the artifact and points upward; it defines no gate, coins no obligation code, and does not restate `INV-111` or `EMERGE-OBS` semantics as a local reference definition.
3. Neighbor-distinction → manual review + codebase grep-proof: the entry is explicitly distinguished from `evidence`, `observation`, `projection`, and `story sifting`, and the `EMERGE-OBS` entry is an execution-label lookup only (points to execution `00`/`10`, defines no gate).

## What to Change

### 1. `02_GLOSSARY.md` — canonical term "observer-only emergence evidence" (D1)

Add to the `## Canonical context-bound and deferred terms` table (near `Evidence` L179 / `Proof` L185), final wording at enactment: **observer-only emergence evidence** — the canonical term for the after-the-fact artifact used to document emergent ordinary-life outcomes under `INV-111`, and the glossary class name for architecture `13`'s "observer-only emergence-evidence record" (so the tiers do not fork the name). The entry must carry the five required properties:

- **retrospective / after-the-fact** — produced from completed event-log/replay ancestry, not planned as a dramatic target;
- **non-certifying** — acceptance context, never sufficient alone to pass behavior gates;
- **ancestry-backed** — tied to causal event-log ancestry, replay/extraction provenance, and the phenomenon-family row contract;
- **non-steering** — never feeds cognition, scheduler, validators, authoring, seed/scenario selection, pacing, LOD, difficulty, or outcome gates;
- **structurally non-input** — the simulation must not read it as state, planner data, content-selection data, or pass/fail threshold.

Cross-reference **upward only** (foundation `02`/`09`/`12`, architecture `13`, execution `00`/`10`). Explicitly distinguish it from the neighbors it must not collapse into: not ordinary `evidence` (supports many holder claims), not `observation` (can be diegetic), not `projection` (a derived read model), not `story sifting` (pattern surfacing).

### 2. `02_GLOSSARY.md` — compact `EMERGE-OBS` execution-label lookup (D1)

Add a compact **`EMERGE-OBS`** entry as an *execution-label lookup only*: it names the execution-tier observation-obligation label for these records and points to execution `00`/`10` for semantics. It defines no gate and is not a reference-tier gate definition (spec §6 / §R-C). Preserve the glossary's naming-only authority boundary throughout.

## Files to Touch

- `docs/3-reference/02_GLOSSARY.md` (modify)

## Out of Scope

- **Final glossary wording / exact table placement / token spellings** — owned by the enactment within the reference layer's existing style (spec §6).
- **`EMERGE-OBS` as a reference-tier gate definition** — added only as an execution-label lookup; gate/obligation semantics stay owned by execution `00`/`10` (spec §6 / §R-C).
- **Reference index / review checklist (`docs/3-reference/00`)** — boundary-awareness, no required change (spec §6); covered by sibling work only if reviewers repeatedly miss the honesty check.
- **The acceptance-evidence risk-register realignment (D2)** — sibling ticket 0029REFTIEDOC-002.
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/execution edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Canonical-term landing grep** — the glossary names the artifact and its properties: `grep -niE "observer-only emergence evidence|retrospective|non-certifying|ancestry-backed|non-steering|structurally non-input" docs/3-reference/02_GLOSSARY.md` resolves the term and the five-property set.
2. **EMERGE-OBS lookup landing grep** — `grep -nE "EMERGE-OBS" docs/3-reference/02_GLOSSARY.md` resolves the execution-label lookup entry (≥1 match where there were 0).
3. **Upward cross-reference review** — every cross-reference points upward (foundation `02`/`09`/`12`, architecture `13`, execution `00`/`10`); no sideways/downward reference and no local restatement of `INV-111` or `EMERGE-OBS` semantics.
4. **Naming-boundary review** — the entries define no gate, coin no obligation/invariant/risk identifier, and preserve the glossary's naming-only authority boundary (`## Maintenance rule`).
5. **Invariants alignment review** — D1 names the `INV-111` artifact class with its retrospective / non-certifying / ancestry-backed / non-steering / non-input properties; it neither weakens nor redefines the invariant.

### Invariants

1. **observer-only emergence evidence** is named once, carries all five properties, and cross-references upward only — no neighbor-collapse and no local doctrine/gate definition (`INV-111`; glossary naming-only authority boundary).
2. `EMERGE-OBS` appears as an execution-label lookup pointing to execution `00`/`10`, never as a reference-tier gate definition.

## Test Plan

### New/Modified Tests

1. `None — documentation-only reference-doctrine ticket; verification is command-based (canonical-term + EMERGE-OBS landing greps) plus an upward-cross-reference / naming-boundary / invariants-alignment manual review against foundation 02, architecture 13, and execution 00/10. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "observer-only emergence evidence|retrospective|non-certifying|ancestry-backed|non-steering|structurally non-input" docs/3-reference/02_GLOSSARY.md` — confirms the canonical term and its five properties landed.
2. `grep -nE "EMERGE-OBS" docs/3-reference/02_GLOSSARY.md` — confirms the execution-label lookup entry landed (was 0 matches pre-edit).
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for a reference-doc edit is the greps above plus the upward-cross-reference / naming-boundary / invariants-alignment review.`

## Outcome

Completed: 2026-06-14

Implemented D1 in `docs/3-reference/02_GLOSSARY.md`:

- Added `Observer-only emergence evidence` to the canonical context-bound and deferred terms table with the required retrospective, non-certifying, ancestry-backed, non-steering, and structurally non-input properties.
- Added `EMERGE-OBS` as an execution-label lookup that points to execution `00`/`10` and defines no reference-tier gate.
- Kept the edit within the glossary naming boundary: no foundation, architecture, execution, crate, fixture, invariant, risk identifier, gate, or observation-obligation definition was changed.

The active goal request to implement the `0029REFTIEDOC*` series was treated as the owner approval required by the ticket precondition and spec 0029 R-A.

Verification run:

- `grep -niE "observer-only emergence evidence|retrospective|non-certifying|ancestry-backed|non-steering|structurally non-input" docs/3-reference/02_GLOSSARY.md`
- `grep -nE "EMERGE-OBS" docs/3-reference/02_GLOSSARY.md`
- Manual review of the diff confirmed upward-only references to foundation `02`/`09`/`12`, architecture `13`, and execution `00`/`10`, with no local restatement of `INV-111` or `EMERGE-OBS` gate semantics.

No Rust gates were run for this ticket because the accepted scope was documentation-only reference terminology and no crate, fixture, schema, or executable surface changed.
