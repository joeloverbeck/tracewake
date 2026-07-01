# 0070 TUI Overhaul Doctrine and Reference Documentation Settlement Spec

**Status**: PROPOSED

This is a post-`FIRST-PROOF-CERT` **doctrine-settlement** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and promoted to `archive/specs/` on acceptance; it is never
promoted to the live `docs/4-specs/` tier. It is not a certification audit, mutation-remediation
pass, Phase-4 entry claim, or latest-`main` certification. It uses the sibling hardening-spec
house structure (`0046`, `0047`, `0057`).

This is **Spec J** of the TUI Experience Overhaul roadmap
(`reports/tui-experience-overhaul-research-report.md` §4.3, §8, §7). It consolidates and ratifies the
lower-tier doctrine updates that Specs `0061`–`0069` route as *substance + home*, so the architecture,
execution, and reference tiers describe the new contracts precisely. **No foundational amendment is
recommended or made** (report §8; foundation 08/14/04 and architecture 10 already sanction the overhaul).

## 0. Baseline statement and source discipline

- **Driver.** The research report's doctrine-home table (§4.3) and below-foundation update list (§8),
  which identify the exact architecture/execution/reference sections that must own the new substance.
- **Baseline commit.** Re-verified against repository `HEAD`
  `85dd8836508a58305eb84d70caf52bda088a9bde`. Named symbols/sections authoritative; line numbers
  provenance only.
- **Source discipline.** Commit hashes are audit/provenance only. No latest-`main` certification,
  Phase-4 entry, or second-proof entry. A manifest is path inventory only.
- **Authority posture.** Subordinate to foundation → architecture → execution → reference. This spec
  proposes doctrine *substance + home*; the ratified wording lands as the owning-tier edit at
  enactment, under ordinary owner approval (architecture/execution/reference), not constitutional
  sign-off. It mints no invariant/gate/glossary id.
- **Execution admissibility posture.** `P0-CERT not applicable`.
- **Ledger timing.** Archived-row entry at acceptance/closeout, not at proposal. No ledger row now.

## 1. Scope

### 1.1 In scope

Consolidated doctrine updates (proposed as substance + home; ratified at the tier edit):

1. **Architecture 10** (`…/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`): expand the
   embodied view-model contract from visible-actor identity to visible-actor awareness (Spec `0063`);
   define screen model / headless dump / `ratatui` buffer as client rendering artifacts (Specs
   `0061`/`0064`); recognize the interactive shell as a presentation-only adapter (Spec `0065`); extend
   transcript obligations to scripted drivers and fullscreen dumps (Spec `0062`); keep menu positions
   presentation-only and closed disposition mandatory.
2. **Architecture 03** (`…/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`): co-present public
   activity observations are holder-known context material when captured by modeled observation, with
   mandatory provenance/freshness (Spec `0063`).
3. **Architecture 06** (`…/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`): define
   public-activity observation flow into observations/beliefs/memories/leads (Spec `0063`).
4. **Architecture 13 / Execution 10**: recognize screen dumps and buffer snapshots as review artifacts
   with non-vacuity requirements (Specs `0064`/`0069`).
5. **Reference risk register** (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`): add/update risks for a
   fullscreen UI swallowing the pure seam, actor-activity truth leakage, and agent-automation regression;
   cross-reference existing TUI parity/debug-contamination risks.

### 1.2 Out of scope (non-goals)

- **No foundation amendment.** Foundation is adequate; only architecture/execution/reference edits.
- No new invariant, gate code, or glossary term is minted.
- No product-code change; this spec is documentation settlement only.

## 2. Dependencies and ordering

- **Depends on:** drafted alongside `0063` and `0069`; **finalized last**, after the implementation
  contracts settle so the ratified wording matches shipped types.
- **Blocks:** nothing downstream in this roadmap; it is the closing doctrine record.

## 3. Doctrine anchors

- **`docs/README.md`**: authority-tier layering (foundation → architecture → execution → reference).
- **Foundation 08 / 14 / 04**: cited to record that the overhaul is already sanctioned and needs no
  foundational amendment.
- **Architecture 10 / 03 / 06 / 13; Execution 04 / 07 / 10; Reference risk register**: the owning homes
  for the new substance.

## 4. Findings and remediation requirements

- **4.1 Consolidate, do not re-decide (seam: per-spec §5 routing).** Each of Specs `0061`–`0069` routes its
  doctrine amendment as substance + home; this spec gathers them into coherent tier edits without adding
  new substance.
- **4.2 Ratify against shipped types (seam: tier edits).** Wording ratifies only after the implementation
  contracts exist, so the doctrine matches real symbols (`VisibleActor` awareness fields, screen-model/dump
  types, intent/driver surfaces).
- **4.3 No invented identifiers (seam: whole spec).** References architecture 10, 03/06, execution 04/07/10,
  and the risk register; invents no invariant/gate/glossary id.
- **4.4 Implementation decomposition.** One ticket per owning-tier document edit (architecture 10; arch 03;
  arch 06; arch 13 / exec 10; reference risk register), each ratifying that tier's routed substance.

## 5. Doctrine amendments (this spec IS the settlement)

The consolidated architecture/execution/reference edits in §1.1 are the deliverable. Foundation is
untouched. Approval bar: ordinary owner approval for the architecture/execution/reference tiers, attached
to each eventual tier edit — not to this spec's proposal write.

## 6. Required fixtures and tests

- Documentation-only: no code tests. Verification is manual review that each tier edit matches the shipped
  symbols from Specs `0061`–`0069` and cites no invented invariant/gate/glossary id.
- Cross-reference integrity: cited section anchors resolve to real documents.

## 7. Acceptance artifact and evidence

A review artifact recording the ratified tier edits (architecture 10/03/06, architecture 13/execution 10,
reference risk register), a confirmation that no foundational amendment was needed, and a no-invented-id
check, at the exact implementation commit.

## 8. Implementation constraints

- Documentation-only; edits only architecture/execution/reference tiers plus the risk register.
- No foundation edit; no new identifiers.
- Ratify wording only after implementation contracts (Specs `0061`–`0069`) are settled.

## 9. Risks and open questions

- **Risk: doctrine drifts from shipped types** if ratified too early. Mitigation: finalize last, against
  real symbols.
- **Open question:** whether any routed amendment reveals a genuine foundational contradiction. Expected
  answer: no (report §8); if one appears, it escalates to a constitutional-amendment proposal separate from
  this spec.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-065 (TUI is a primary product interface) | aligns | Doctrine records the new client contracts precisely. |
| INV-069 (TUI must not implement simulation rules) | aligns | Documents the presentation-only adapter/screen-model boundary. |
| INV-099 (truth may validate, not plan) | N/A | Documentation settlement; introduces no behavior. |
| INV-102 (cognition inputs require provenance) | aligns | Records provenance/freshness requirement for co-present activity. |
