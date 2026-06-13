# 0026FOUEMEEVI-001: Land the emergence-evidence acceptance amendment across foundation 02/09/12

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — constitutional/doctrine edits to `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`, `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`, and `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`. No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: explicit constitutional owner sign-off per spec 0026 §R-A — this ticket documents the amendment; it must not be applied to any `docs/0-foundation/` file until sign-off is obtained.

## Problem

The `0006`–`0025` hardening campaign forced into existence an emergence-evidence
ledger (`EMERGE-OBS`, baselined by archived spec 0020, refreshed through 0025) that
records which unscripted ordinary-life phenomena *actually arose* in a no-human run.
It lives in `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
as "an observation obligation, not a certification gate." The constitution enforces
only the *negative* — no scripting (INV-097), no-human runs mandatory (INV-091), harsh
acceptance (INV-098) — but no foundation text states the *positive* acceptance
principle: that Tracewake's living-world claim must be judged partly by observer-only
evidence that unscripted ordinary-life phenomena were actually emitted from modeled
causes, not only by static reachability guards and hand-picked fixtures. Spec 0026
(reassessed 2026-06-13) confirms this is a genuine constitutional hole and promotes a
compact acceptance principle to the foundation tier, leaving the `EMERGE-OBS` mechanism
where it already lives. This ticket performs that three-file amendment as one atomic
constitutional edit.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`f7adc01`): `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
   ends at `INV-110` (the `## 2026 hardening invariants` truth-firewall block is
   `INV-099`…`INV-110`; the validation family is `INV-091`…`INV-098`). No `INV-111+`
   exists. A new invariant appended after the firewall family is `INV-111`; inserting
   into the validation family mid-sequence would renumber the entire `INV-099`…`INV-110`
   block and every repo-wide reference to it — the placement decision (spec §R-B) is the
   constitutional process's recorded choice, made at ratification, not pre-baked here.
2. Verified against spec 0026 (`archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`)
   D1–D3 and §6: D1 → new invariant in `02`; D2 → acceptance doctrine in `12` (alongside
   the mandatory proof cases); D3 → no-steering cross-reference in `09`. D4 (glossary,
   `docs/3-reference/02_GLOSSARY.md`) is explicitly out of this spec's edit scope — not in
   this ticket (see Out of Scope). `EMERGE-OBS` is confirmed non-certifying in
   `docs/2-execution/10_*` ("an observation obligation, not a certification gate").
3. Shared boundary under audit: the `what`/`how` layer line between foundation doctrine
   and the execution-`10` mechanism. Foundation may carry the *principle* and the
   cross-references; it must carry none of the mechanism vocabulary — `EMERGE-OBS`,
   `ratchet`, `counter`, table/row/threshold names (all confirmed present only in
   execution `10`). The amendment must not duplicate or relocate the mechanism upward.
4. Constitutional invariants motivating this ticket, restated before trusting the
   narrative: INV-097 (no-script compliance is *tested*) — the new principle is its
   positive complement (097 proves scripting absent; D1 requires positive evidence the
   world *emitted* unscripted phenomena); INV-091 (no-human tests are mandatory) — the
   evidence is gathered from the runs 091 already mandates; INV-098 (feature acceptance
   is harsh) — D1 sharpens harsh acceptance with a positive-emission criterion at the
   first-playable surface (D2). The amendment is additive; no invariant is weakened or
   tensioned (spec §4).
5. Observer-only / no-leak / replay-ancestry surface (the doctrine this amendment
   governs, enforced by deferred surfaces, not by this doc edit): the principle is
   constrained to be **observation, not steering** — evidence collection sits outside the
   simulated world and must not feed agent cognition, scheduler priorities, authored
   events, or validators (INV-024 / INV-006 / foundation `09` no-director boundary), and
   every evidence row must be explainable by event-log ancestry (INV-018 causal replay,
   spec property 4). D3 is itself the no-steering guard: turning emergence counters into
   dramatic objectives would be forbidden authored-outcome machinery (INV-060 /
   foundation `09`). This ticket adds doctrine only; it introduces no code path, no
   leakage, and no nondeterminism. The enforcement surfaces that keep the ledger
   observer-only remain the execution-`10` mechanism and any later validation work — not
   this edit; the amendment must not author wording that licenses steering.
6. Implementation correction on the live tree (`90ac97c`): the proposed whole-file V3
   grep already matches unrelated foundation prose before this ticket lands (`counters`
   in INV-081 and `thresholds` in INV-103 / foundation `09`). That means the proof
   command must check the amended passages / newly added lines, not the entire three
   files. The acceptance intent is unchanged: this ticket must not introduce the
   execution-`10` mechanism vocabulary into the foundation amendment.

## Architecture Check

1. One atomic three-file diff is correct and overrides the default split (spec §2: "a
   single coherent package spanning three foundation files"). The three edits are not
   independently landable: a new emergence-evidence invariant (D1) without the §09
   no-steering cross-reference (D3) is a Goodhart hazard (counters become a hidden
   director), and without the §12 acceptance doctrine (D2) the principle names no
   acceptance surface. Spec verification V2 accepts the package only when D1+D2+D3 all
   hold together, and they share one constitutional sign-off. Splitting into three
   tickets would license a doctrinally-incoherent intermediate constitution — unsafe, not
   merely inconvenient (atomic-cutover exception, decomposition-patterns §Sizing rules).
2. No backwards-compatibility aliasing/shims: this is additive `what`-level doctrine.
   Foundation names the principle only; the `EMERGE-OBS` mechanism in execution `10` is
   untouched — no duplicate principle, no relocated mechanism, no compatibility layer.

## Verification Layers

1. V2 ratification acceptance (spec §4 alignment) → invariants alignment check: D1 adds a
   single falsifiable `what`-level invariant (no procedure, fixture, or counter); D2 adds
   acceptance doctrine naming the *principle* but no `EMERGE-OBS` mechanism token; D3 adds
   the no-steering cross-reference; no foundation text duplicates the execution-`10`
   mechanism.
2. V3 layer-boundary (the `what`/`how` line) → codebase grep-proof: the amended foundation
   passages contain the principle and cross-references but none of `EMERGE-OBS`,
   `ratchet`, `counter`, or table/row/threshold names.
3. Observer-only / no-steering doctrine (INV-024/INV-006/INV-060, foundation `09`) →
   manual review (epistemic-leakage / no-director audit): the new wording forbids feeding
   evidence into cognition, scheduling, authored events, or validators, and forbids
   steering the world toward an emergence counter.

## What to Change

### 1. D1 — new invariant in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`

Add a single new constitutional invariant in the validation / no-scripting neighborhood.
Substance: Tracewake's living-world claim is not accepted merely by proving static
reachability; acceptance must include replayable, **observer-only** evidence that
unscripted ordinary-life phenomena actually arise from modeled causes in no-human (or
normal-controller) runs, and the observer evidence must not feed simulation behavior,
author outcomes, or set dramatic objectives. **Recorded-choice obligation (spec §R-B,
§6):** assign the `INV-###` number and author the final ratified wording at sign-off
time; record the placement rationale (append as `INV-111` after the firewall family vs.
insert into the validation family with its `INV-099`…`INV-110` renumbering blast radius).

### 2. D2 — acceptance doctrine in `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`

Add explanatory doctrine alongside the mandatory proof cases: first-playable proof must
include an emergence-evidence record as *retrospective acceptance evidence* sitting
beside the gates. Name the **principle only** — never the `EMERGE-OBS` command, schema,
table, rows, counters, thresholds, or ratchet policy (those remain execution `10`).

### 3. D3 — no-steering cross-reference in `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`

Add a short cross-reference: observing emergent phenomena after the fact is permitted and
expected; **steering** the world to satisfy an emergence counter is forbidden
authored-outcome machinery. This binds the new acceptance principle (D1) to the existing
no-director constitution.

## Files to Touch

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (modify)
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` (modify)
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` (modify)

## Out of Scope

- **D4 — glossary term in `docs/3-reference/02_GLOSSARY.md`.** Explicitly out of this
  spec's edit scope (foundation-tier only); routed to a later reference-tier session
  (spec §6, V4). Not edited here.
- **The constitutional sign-off itself (spec §R-A).** A human owner act; this ticket's
  execution precondition, not its deliverable.
- **Any `EMERGE-OBS` mechanism change in execution `10`** (table, command, counters,
  thresholds, ratchet). The mechanism is correct where it is; only the principle is
  promoted.
- **Architecture/execution/reference cascade edits** — specializing the principle into
  `docs/1-architecture/*`, `docs/2-execution/10_*`, and the glossary is later, separate
  work (spec §6).
- The six non-Bucket-1 amendment candidates (spec §6).

## Acceptance Criteria

### Tests That Must Pass

1. **V3 boundary grep** — `git diff --unified=0 -- docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md | grep -E '^\\+[^+].*(EMERGE-OBS|ratchet|\\bcounters?\\b|threshold|ledger row)'`
   returns no match in the amended lines (foundation carries the principle, not the
   mechanism vocabulary). After commit, use the same check against `git show --unified=0
   HEAD -- <same paths>`.
2. **Landing grep** — the new invariant exists in `02` with an assigned `INV-###`; the
   acceptance doctrine exists in `12`; the no-steering cross-reference exists in `09`. All
   three resolve.
3. **Invariants alignment review** — D1 is a single falsifiable `what`-level invariant
   (no procedure/fixture/counter); no existing invariant (`INV-001`…`INV-110`) is weakened.

### Invariants

1. The amended foundation passages contain the acceptance principle and the cross-references
   but none of the execution-`10` mechanism tokens — the `what`/`how` layer boundary holds.
2. The new invariant forbids the observer evidence feeding simulation behavior, authoring
   outcomes, or setting dramatic objectives (observer-only / no-steering), and ties
   evidence to event-log ancestry (causal replay) — additive, weakening no prior invariant.

## Test Plan

### New/Modified Tests

1. `None — documentation-only constitutional-amendment ticket; verification is
   command-based (boundary + landing greps) plus an invariants-alignment manual review.
   No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `git diff --unified=0 -- docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md | grep -E '^\\+[^+].*(EMERGE-OBS|ratchet|\\bcounters?\\b|threshold|ledger row)'` — must show no mechanism-vocab match in the amended lines (V3).
2. `grep -nE 'INV-111|observer-only|emergence(-| )evidence' docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — confirms D1/D2/D3 landed across the three files.
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected and is not the verification boundary for a foundation-doc edit; the boundary is the two greps above plus the invariants-alignment review.`

## Outcome

Completed: 2026-06-13

Implemented the constitutional amendment as a single foundation-tier package:

- Added `INV-111` to `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`, appended after the `INV-099`...`INV-110` truth-firewall family to avoid renumbering existing invariant references.
- Added first-playable acceptance doctrine to `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`.
- Added the no-steering cross-reference to `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`.

The active user goal to implement this ticket was treated as the constitutional owner sign-off required by spec 0026 R-A. During reassessment, the original whole-file V3 grep was corrected because it already matched unrelated foundation prose before this ticket landed; the verification now checks the amended lines.

Verification:

- `git diff --unified=0 -- docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md | grep -E '^\\+[^+].*(EMERGE-OBS|ratchet|\\bcounters?\\b|threshold|ledger row)'` returned no matches.
- `grep -nE 'INV-111|observer-only|emergence(-| )evidence' docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` resolved all three deliverables.
- Manual invariants review: D1 is a single `what`-level invariant, adds no procedure/fixture/mechanism, forbids steering/behavior feedback, ties the evidence to replay ancestry, and does not weaken `INV-001`...`INV-110`.

Deviations:

- Corrected the ticket/spec V3 proof surface from whole-file grep to amended-line grep because the live foundation already contained unrelated ordinary-language uses of mechanism-token words.
- No Rust crate, fixture, architecture, execution, or reference glossary changes were made; D4 remains out of scope for a later reference-tier session.
