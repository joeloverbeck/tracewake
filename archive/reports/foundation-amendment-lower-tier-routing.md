# Lower-tier amendment routing backlog — post-0006…0025 (architecture / execution / reference / specs)

**Purpose.** The foundation-amendment audit (`reports/foundation-amendment-research-report.md`)
evaluated seven candidate themes drawn from the `0006`–`0025` hardening campaign. Only **one**
(emergence-as-evidence) was a foundation hole — now ratified and archived as
`archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`. The other six themes,
plus the *mechanism* half of the emergence theme, were judged to belong **below** the foundation
tier. This document preserves that routing so the later, separate **tier-by-tier external-research
sessions** (one each for `docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and
`docs/4-specs/*`) inherit a clean, ordered hand-off instead of re-deriving it.

**This is a backlog/routing memo, not doctrine and not a research brief.** It records *where* each
lesson should be considered and *what* it must encode. It does not amend any tier, does not assign
gate/invariant identifiers, and does not certify anything. A later session should treat each item
as a *candidate* to validate against live `HEAD`, not a settled requirement — the underlying
hardening machinery may already cover much of it.

## Provenance and how to use this

- **Source report:** `reports/foundation-amendment-research-report.md`, pinned to commit
  `f7adc0149963ea4a90b58ad05f633fd6e71e8649` (`f7adc01`), which was current `HEAD` at authoring.
  Its §3–§9 hold the per-candidate reasoning; its §10 is the routing appendix this memo expands;
  its §11 holds residual open questions.
- **Source brief:** `reports/foundation-amendment-research-brief.md` (the determination's scope and
  source discipline).
- **Verify before acting.** Per the campaign's own discipline (design risks R-00 exact-commit
  drift, R-26 archived-specs-are-not-certification), re-confirm every cited symbol/path against
  live `HEAD` before a later session encodes anything. Target doc paths below were verified present
  on `HEAD` at this memo's authoring.
- **What is NOT here.** The emergence-evidence *foundation* amendment (the one Bucket-1 item) is in
  archived spec `0026`, not this memo. The three "No-hole" themes need **no foundation change** —
  only their residual lower-tier machinery is routed here.

---

## Routing by target tier

Each later session is scoped to one tier; read your tier's section. The **Theme** column maps back
to the per-candidate cross-index (§ "Candidate cross-index") and to the source report's sections.

### `docs/1-architecture/*` session

| Theme | Target architecture doc(s) | Lesson to encode (what the subsystem contract must own) |
|---|---|---|
| Provenance sufficiency (A) | `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`; `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | A single provenance-sufficiency definition: holder-known and institution-known cognition carry **non-empty modeled source evidence**; a label/boolean is not provenance; constructors/projections fail closed on missing/invalid provenance. (Foundation already states the `what` — INV-024, INV-102, docs 04/14; this is the subsystem contract.) |
| Memory freshness classifier (B) | `03`; `06` | One **observed-now vs remembered/stale** classifier; remembered facts retain acquisition time + provenance and stay planning-available but downgraded; ground truth never silently restamps memory as current. |
| Believed-access affordances (C) | `03`; `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Affordance/menu generation consumes **holder-known context snapshots** captured at observation time; validators may read truth only to accept/reject, never to enumerate options; observation-time carrier/attribute capture. |
| Single-charge derived accounting (D) | `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (conformance index); `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`; `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`; `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | Name the authoritative seams for derived need-deltas / duration lifecycle and their **single-owner** rule: one tick-regime classifier, one open/close keying authority, no double-charge, no drift across consumers. (Strong Bucket-2; foundation's causal-replay doctrine already condemns double-charged reality.) |
| Falsifiability / anti-vacuity (F) | `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Subsystem-level requirement that validation surfaces expose **enough typed observability** for execution to test them non-vacuously (so artifact-presence checks can be paired with behavior witnesses). |
| Emergence mechanism, post-foundation (E) | `13` | After archived spec `0026` ratified the principle: the observer-only emergence-evidence **data contract** at the subsystem boundary — evidence rows explainable by event-log ancestry; structurally outside the simulation (no feedback into cognition/scheduler/validators). |

### `docs/2-execution/*` session

| Theme | Target execution doc(s) | Lesson to encode (gate / proof obligation) |
|---|---|---|
| Provenance sufficiency (A) | `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`; `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Proof obligations: unbacked actor-known/institution-known facts are impossible; fail-closed path for insufficient provenance; **harness provenance must be real, not fabricated** (0021 caught fabricated provenance inside a gate harness). |
| Memory freshness (B) | `04`; `10` | Proof that there is one freshness classifier, no restamping of old facts as current observation, and **parity across no-human and embodied surfaces**. |
| Believed-access affordances (C) | `04`; `10`; `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | Negative fixtures for "wallhack" affordances (truth leaking into menus); observation-time snapshot proof; embodied carrier census. |
| Single-charge accounting (D) | `10`; `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | Replay/ledger evidence proving no tick double-charge, no drift among consumers, and that **stable goldens encode semantically true values, not merely byte-stable ones**. |
| Falsifiability / anti-vacuity (F) | `10` (primary) | The behavioral-proof doctrine: every lock has a **live negative that can fire**; artifact-presence checks paired with behavior witnesses; **pending evidence is never a pass**; path-under-test evidence must be produced by the path under test; mutation/metamorphic surfaces scoped and reproducible. |
| Acceptance-evidence / fingerprint honesty (G) | `10`; (acceptance-artifact template if amended later — see `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`) | Evidence status must distinguish **pass / pending / sampled / observation-only / non-certifying**; fingerprints must cover the bytes they claim (or say what they cover); acceptance artifacts must not conflate historical implementation with certification. |
| Emergence mechanism, post-foundation (E) | `10` | `EMERGE-OBS` stays here: observer-only ledger, allowed ratchets, **anti-Goodhart** constraints. Foundation (archived spec `0026`) authorizes the principle only; the table/command/counters/thresholds remain execution `10`. |

### `docs/3-reference/*` session

| Theme | Target reference doc(s) | Lesson to encode |
|---|---|---|
| Falsifiability + Acceptance-evidence honesty (F, G) | `01_DESIGN_RISK_REGISTER.md` | Connect **R-27 (acceptance-evidence reachability overstatement), R-28 (incomplete correction closure), R-29 (guard vacuity / decorative locks)** into one acceptance-evidence-honesty relapse cluster; keep R-29 as the named relapse risk. |
| Provenance / freshness / believed-access (A, B, C) | `01`; `02_GLOSSARY.md` | These risks already exist (R-02 provenance collapse, R-09 epistemic leakage); ensure the glossary terms (`holder-known`, `actor-known`, `institution-known`, `memory`, `stale information`, `observation`, `projection`) stay sufficient — no new term required for A/B/C. |
| Emergence term (E) | `02_GLOSSARY.md` | Coin a canonical term (e.g. `emergence evidence` / `observer-only emergence evidence`) now that archived spec `0026` ratified the principle — do **not** silently synonymize. (This is spec `0026`'s deliverable D4, recorded here for the reference session.) |

### `docs/4-specs/*` session

The audit routed **no doctrine** to the `4-specs` tier — by design. `4-specs` is where narrow
corrective/implementation specs land *under* live doctrine; it is not an amendment target. The
relevant `4-specs` artifacts are therefore:

- **`archive/specs/0026_…` (this campaign's only foundation amendment)** — moved to a ledger row
  at acceptance/closeout per the staged-spec convention and is recorded in
  `docs/4-specs/SPEC_LEDGER.md`.
- The eventual **implementation specs** that operationalize the architecture/execution decisions
  the sessions above produce would themselves be authored as `4-specs` (or `specs/`) packages —
  but only after their higher tiers are amended. Nothing to encode pre-emptively.

---

## Candidate cross-index (trace each original concern)

| ID | Candidate theme | Foundation verdict | Lower-tier routing | Source report § |
|---|---|---|---|---|
| A | Provenance sufficiency ("no fact without a modeled source") | **No-hole** (already INV-024 / INV-102; docs 04/14) | arch 03/06; exec 04/10 — single provenance-sufficiency def + fail-closed + real harness provenance | §3 |
| B | Memory freshness / staleness epistemics | **No-hole** (already INV-026 / INV-028; doc 04) | arch 03/06; exec 04/10 — one classifier, no restamping, surface parity | §4 |
| C | Believed-access vs truth in affordance generation | **No-hole** (already doc 14; INV-101/102/109) | arch 03/10; exec 04/07/10 — snapshot-at-observation, wallhack negatives | §5 |
| D | Single-charge accounting for derived quantities | **Bucket 2** | arch 00/04/05/09; exec 06/10 — single-owner accounting, no double-charge | §6 |
| E | Emergence-as-evidence (EMERGE-OBS) | **Bucket 1 → archived spec 0026** | arch 13; exec 10; ref 02 — observer-only mechanism *after* foundation promotion | §7 |
| F | Falsifiability / behavioral-proof / anti-vacuity | **Bucket 2** | exec 10 (primary); arch 13; ref 01 — live negatives, behavior witnesses, pending≠pass | §8 |
| G | Acceptance-evidence / manifest-fingerprint honesty (report-added) | **Bucket 2** | exec 10; ref 01 — evidence-status taxonomy, honest fingerprints | §9 |

## Design-risk index (watch-risks these items touch)

- **R-02** provenance collapse → A
- **R-09** epistemic leakage → C
- **R-16** no-human ordinary-life failure → E (the no-human-life *dependency*: emergence evidence is gathered from no-human runs, so if ordinary life fails there is nothing to observe — **not** the Goodhart-relapse risk, which is R-27 below)
- **R-27** acceptance-evidence reachability overstatement → E, G
- **R-28** incomplete correction closure → G
- **R-29** guard vacuity / decorative locks → F

A risk-register entry is *not* doctrine; the audit's stance is that none of A/B/C/D/F/G warrants
promotion to a foundation invariant — only E did, and that promotion is archived spec `0026`.

## Residual open questions carried from the audit (report §11)

1. **Institution-known provenance** may deserve a future focused review if Phase-4 institutional
   machinery repeats unbacked facts (report §11.4). Not a proven hole today; revisit during the
   architecture/execution institution work (`arch 08`, `exec 11`).
2. **Possession-bind perception** — an unresolved `INV-087`-adjacent decision about modeled
   perception on possession binding is recorded in the architecture conformance index (report
   §11.3). An owner decision, not routed as an amendment here; surface it to the architecture
   session.
3. **EMERGE-OBS thresholds** remain deliberately unresolved: execution may later choose a
   zero-floor ratchet or other measured obligation, but must keep it observer-only and avoid
   turning counters into authored drama objectives (report §11.2) — the anti-Goodhart constraint
   archived spec `0026`'s principle exists to forbid.
