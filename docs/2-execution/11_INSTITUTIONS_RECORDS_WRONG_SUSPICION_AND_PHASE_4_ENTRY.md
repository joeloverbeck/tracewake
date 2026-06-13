# Institutions, Records, Wrong Suspicion, and Phase 4 Entry

## Status

Live execution doctrine for Phase 4 entry. Phase 4 is locked until named certification gates pass.

## Authority boundary

This document owns the entry criteria and execution contract for institutions, households, norms, records, reports, and wrong suspicion. It does not authorize immediate implementation.

## Depends on

- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`

## Entry rule

`PHASE-4-ENTRY` is a phase-certification artifact label for this entry
contract. It composes the listed prerequisites; it is not a new canonical gate
code beyond `00_EXECUTION_INDEX_AND_AUTHORITY.md`.

Phase 4 may not proceed until these gates pass:

- `P0-CERT`;
- `SPINE-CERT`;
- `EPI-CERT`;
- `ORD-LIFE-CERT`;
- `DATA-CERT`;
- `FIXTURE-CERT`;
- `DIAG-CERT`;
- `FIRST-PROOF-CERT` unless the spec is a named remediation for one of these gates.

The current implementation history through specs `0005` through `0008` does not satisfy this entry rule by itself.

## Phase 4 purpose

Phase 4 adds local social machinery only after ordinary life and epistemic correctness are certified. It should prove that institutions are fallible world processes, not quest engines or truth oracles.

Core Phase 4 concepts:

- households as minimal institutions;
- roles, powers, permissions, obligations, and prohibitions;
- reports and records as claims with sources;
- local authority procedures;
- proof rules and failure modes;
- suspicion from actor/institution-known evidence;
- wrong suspicion from partial, stale, biased, or misleading information;
- sanctions, refusals, delays, misfiling, and institutional fallibility.

## Institution-known transaction

Institutions must mirror actor-known discipline. An institution procedure consumes institution-known context, not global truth.

Institution-known context includes:

- records accessible to the institution;
- reports received through speech/action events;
- roles and powers active at the time;
- procedure state;
- available staff/resources;
- jurisdiction;
- known norms and proof requirements;
- testimony, rumors, traces, and prior decisions;
- uncertainty and contradiction markers.

Institution procedures may not read culprit truth, missing-item truth, debug truth, or player identity to decide suspicion, enforcement, or payment.

## Phase-4 provenance and freshness future-proofing

Any institution-known fact, record-derived belief, norm-triggered procedure,
wrong-suspicion inference, or artifact interpretation must prove provenance
sufficiency and freshness through the same fail-closed mechanics as
`04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` and the review
artifact standards in `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`.
Missing, dangling, wrong-kind, ambiguous, forbidden-source, stale, or display
text-only provenance fails closed before institution-known use.

Required negative fixtures include:

- an institution reacts to truth without a record;
- a stale record is treated as fresh;
- dangling record provenance;
- wrong-kind provenance;
- record display text mistaken for provenance.

This obligation locks the proof contract before Phase 4 entry. It expands no
Phase 4 scope and decides no new institution mechanics.

## Records

Records are artifacts carrying claims. They are not truth.

A record must preserve:

- author or issuing process;
- institution or household context;
- physical/digital artifact identity;
- claims and sources;
- time and status;
- access rules;
- amendments and contradictions;
- custody and destruction history;
- replay ancestry.

A record can be wrong, stale, forged, incomplete, lost, suppressed, or ignored.

## Wrong suspicion

Wrong suspicion must arise from modeled evidence or procedure, not from a storyteller desire for drama.

Allowed causes include:

- partial witness evidence;
- misleading trace;
- stale record;
- biased testimony;
- motive inference;
- proximity;
- prior reputation;
- institutional pressure;
- lack of exculpatory information;
- social conflict;
- procedural error.

Forbidden causes include culprit flags, quest stages, player behavior scripts, suspicion score seeded from truth, or hidden correction when the player arrives.

## Phase 4 acceptance gates

Phase 4 passes only when:

1. Reports are actions or speech acts through the shared pipeline.
2. Records carry claims, sources, access, and status.
3. Institution procedures consume institution-known context with provenance.
4. Norm violation, detection, report, suspicion, record, proof, sanction, refusal, and appeal/failure remain distinct.
5. Wrong suspicion can arise and persist without hidden truth correction.
6. Human-controlled actors receive no special institutional treatment.
7. Debug panels can show institution truth/audit only non-diegetically.
8. Replay rebuilds records, institution procedure state, and suspicion diagnostics.
9. Golden fixtures include wrong, stale, refused, delayed, and misfiled cases.
10. No notice, report, record, or reward behaves like a quest flag.

## Locked fixture families

These fixture families may be specified as Phase 4 contracts after entry gates pass:

- missing-property report;
- incident ledger entry;
- partial witness statement;
- wrong suspicion from stale evidence;
- refused report due to low proof;
- misfiled or delayed record;
- household access dispute;
- sanction attempt blocked by procedure;
- payment/reward refused because funds/proof/authority fail.

## Non-goals for Phase 4

Phase 4 does not authorize regional travel, public quest boards, global bounties, LLM dialogue, story-sifting intervention, or second-proof expansion. Those remain locked under document `12`.
