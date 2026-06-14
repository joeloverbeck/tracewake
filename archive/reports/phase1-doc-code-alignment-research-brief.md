# Research Brief — Phase 1 Doc↔Code Alignment Conformance & Anti-Drift Audit (Tracewake)

> **For ChatGPT-Pro Session 2 (the deep researcher).** You are *locked*: produce the deliverable
> directly. Do not interview, do not ask clarifying questions — the requirements below are final.
> Upload bundle = this prompt + the manifest `manifest_2026-06-09.txt`.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-09.txt`) is the path inventory of the `joeloverbeck/tracewake`
repository — a causality-first living-world simulation in Rust: an event-sourced kernel, subjective
epistemics, fallible institutions, ordinary agents, and a TUI-first client surface where every meaningful
change leaves a replayable trace. The workspace is three crates with a strict one-way dependency direction:
`tracewake-core` (authoritative kernel, **zero dependencies**) → `tracewake-content` (fixtures / loading /
schema validation, depends on core) → `tracewake-tui` (terminal boundary, depends on core + content).

Docs are layered authority: `docs/0-foundation` → `docs/1-architecture` → `docs/2-execution` →
`docs/3-reference` → `docs/4-specs`. **Earlier tiers govern later ones.** If execution conflicts with
architecture or foundation, execution is wrong; if implementation is more convenient than the accepted
gates, implementation is wrong. The whole adjudication rule lives in `docs/README.md`.

**Fetch every file from commit `b210e4069c1ec997ed839dca34840ac72058b477`** — the manifest reflects that
exact tree (verified repo HEAD). Construct every raw URL as
`https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/<manifest path>`.
The manifest is **path inventory only** — never source text or authority. Archived specs cite their own
historical baseline commits inside their evidence ledgers; those are *that spec's own* provenance and
**predate** the current HEAD and later merges. **Ignore them as fetch targets — fetch everything from
`b210e40…`.** Branch names, default-branch lookups, repository metadata, and code-search snippets are not
proof of target-commit content.

**This is a NEW lens, not a continuation of the lock-layer campaign.** The repository has just completed a
three-pass structural-hardening / anti-contamination campaign over the Phase 1 / Phase 1A code block, staged
as the visible sequence `0002 → 0003 → 0004`:

- `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` — the original Phase-1
  kernel/event-log/replay/determinism/pipeline/content-validation implementation.
- `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` — the executable TUI
  command loop; **the campaign's only doc-alignment work to date** (it reconciled a README/`SPEC_LEDGER`
  over-claim — docs were ahead of the binary).
- `archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` and
  `archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` — TUI-seam and
  kernel-spine structural hardening; the spine pass added the **structural-lock layer** (conformance
  capstones, an invariant-reference linter, a banned-API token scanner, a `clippy.toml`
  disallowed-types/methods profile, content forbidden-construct guards).
- `archive/specs/0004_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md` — the third pass, which
  re-audited the spine **and the lock layer itself**, closed structural bypasses (public seed mutators,
  weak conformance proofs, brittle field coverage), and locked the strengthened gates into CI. It surfaced
  **zero** named doc↔code alignment problems and judged the block "substantially clean."

Those three passes were **lock-layer-robustness** work: *can the guards be bypassed; do they prove the
property or merely assert a string exists.* **This pass is a different question:** a systematic,
**doc↔code alignment conformance** audit — walk every mandate in the doctrine tiers that bears on the
Phase-1 code and prove the code conforms, or find where it has drifted. The prior "substantially clean"
verdict is treated **skeptically**, not as a given: it was reached through a structural lens, not an
exhaustive tier-by-tier conformance walk. **Do not simply re-run the lock-layer audit and declare "clean."**

The maintainer's standing motivation: past projects "devolved into entropy" when later implementers built on
**non-aligned** code. So beyond finding and correcting any current drift, the deliverable must, *for each
misalignment found*, specify a structural mechanism that makes it **impossible (to the extent practical)**
for future code to re-introduce that drift.

---

## 2. Read in full (authority order)

Read these completely, in this order. The maintainer explicitly wants the **entire doctrine tree** read for
this audit. Each tier's primary (Phase-1-load-bearing) documents are called out; the remaining documents in
each folder are read for **boundary-awareness** — so that Phase 2A / Phase 3A doctrine is recognized as
**out of scope** and uncertified later-phase code is never mis-flagged as "misaligned" (see §3.2). Confirm
actual module/file paths against the manifest before citing; do not assume names.

**Doctrine — foundation (`docs/0-foundation/`, read 00–14):**
- `docs/README.md` — the authority-layering rule and how to resolve cross-tier conflicts; **the audit's
  adjudication method.**
- `docs/0-foundation/00_FOUNDATION_INDEX.md` — anti-drift rules, reading order, "do not cherry-pick."
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — the `INV-001…INV-110` contract. **Every finding and
  every requirement must cite the exact invariant(s) it bears on, verified against this file — never against
  numbers quoted in archived specs.**
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **primary:** the controlling spine
  doctrine (meaningful change ⇒ event; append-only log; rebuildable projections; deterministic replay).
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — **primary:** TUI-as-proof-surface, embodied
  vs. debug, view-model boundary, two-layer why-not.
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — **primary:** no-scripting /
  no-authored-outcome / seed-not-script; "no simulation fact born from prose" (content seam).
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — **primary:** defines the first-proof
  scope and acceptance gates — i.e. **the Phase-1 boundary** that tells you what is in vs. out of scope.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **primary:** truth may
  *validate* but may not *plan/render*; the anti-contamination backbone.
- `docs/0-foundation/{01_PROJECT_CHARTER, 04_CLAIMS_BELIEFS_MEMORY…, 05_AGENTS_NEEDS_INTENTIONS…,
  06_ACTIONS_AFFORDANCES…, 07_INSTITUTIONS…, 10_SCALE_LOD…, 11_LLM_SPEECH_ACTS…, 13_RESEARCH_DECISIONS…}.md` —
  boundary-awareness; several describe later-phase systems whose code is out of scope.

**Doctrine — architecture (`docs/1-architecture/`, read 00–14):**
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — **primary:** how architecture binds; the universal conformance
  questions the audit should apply.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — **primary:** kernel authority, the one-way
  crate dependency direction (`core` zero-dep), forbidden inversions.
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **primary:** event log, replay, projections, save
  packages, randomness/seeding, schema versioning/migration.
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **primary:** holder-known contexts,
  provenance packets, context sealing, contamination gates.
- `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — **primary:** the proposal → pipeline →
  event-apply flow and the no-direct-dispatch rule.
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — **primary:** the client-boundary contract
  the TUI seam must satisfy.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **primary:** acceptance/observability/
  review-artifact requirements and anti-contamination tests.
- `{05_ACTOR_DECISION…, 06_CLAIMS_BELIEFS…, 07_SPEECH_ACTS…, 08_INSTITUTIONS…, 09_ORDINARY_LIFE…,
  11_INCIDENTS_LEADS…, 12_LOD_REGIONAL…, 14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS}.md` —
  boundary-awareness; `09_ORDINARY_LIFE…` (missing-property first proof, settlement ontology) overlaps
  Phase-1 content and may be load-bearing for the content seam — read it carefully; the rest mostly bound
  later phases. `14` (forbidden misreads) is directly useful for not mis-reading doctrine.

**Doctrine — execution (`docs/2-execution/`, read 00–13):**
- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — **primary:** the canonical gate vocabulary (`P0-DOC`, `P0-CERT`,
  `SPINE-CERT`, `EPI-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`)
  and the rule that execution may not soften foundation/architecture.
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — **primary:** the code-audit
  boundary, "archived specs are historical evidence only," and the three admissible spec postures.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — **primary:** gate order; `SPINE-CERT` /
  `EPI-CERT`; that Phase 4 is blocked until certification gates pass.
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — **primary:** the mandatory
  anti-contamination gates every future spec must carry — the list this pass's gates extend, never redefine.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — **primary:** the execution-tier
  scheduler/pipeline/no-direct-dispatch contract.
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — **primary:** the possession-parity / view-model /
  debug-quarantine proof obligations for the TUI seam.
- `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — **primary:** the data/schema/provenance/validation
  gates the content crate must satisfy.
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — **primary:** the golden-fixture and
  replay-acceptance contract.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **primary:** typed provenance over
  string-prefix/substring/display-label diagnostics; adversarial negative gates, not only friendly goldens.
- `{02_FIRST_PROOF_SCOPE…, 06_ORDINARY_LIFE_NEEDS_ROUTINES…, 11_INSTITUTIONS_RECORDS…,
  12_DEFERRED_SECOND_PROOF…, 13_RESEARCH_DECISIONS…}.md` — boundary-awareness; `02_FIRST_PROOF_SCOPE…` helps
  fix the Phase-1 scope line, `06` bounds Phase-3A ordinary-life and is mostly out of scope.

**Doctrine — reference (`docs/3-reference/`, read 00–02):**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **primary:** the standing review checklist the deliverable's
  acceptance section should align to; exact-source discipline.
- `01_DESIGN_RISK_REGISTER.md` — **primary:** named relapse risks (debug-truth leakage,
  display-string-as-authority, nondeterminism, weak traceability, `PlayerCharacter`/`Quest`/`Objective`/
  `Reward`) the alignment must keep closed.
- `02_GLOSSARY.md` — **primary:** prescriptive terminology (`holder-known` vs `actor-known`, world vs
  non-world event, etc.) — use precisely; mis-terminology is itself a drift vector.

**Spec tier (`docs/4-specs/`):**
- `SPEC_LEDGER.md` — **primary:** the authority posture, source discipline, archived-spec status table, the
  live spec set (only `0001` is live), and "next known execution move = `P0-CERT`." **Derive the deliverable's
  number/path from this + the README.**
- `README.md` — **primary:** the rules every future spec must obey (posture declaration; gate codes as
  cross-references only; holder-known/actor-known terminology; archived specs as history; no files for
  symmetry).
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — the one live sibling spec; model its
  structure, posture declaration, and tone.
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — the live review-artifact template; the acceptance section should
  conform to it (exact-commit scoped wording; no over-claim).

**Prior campaign work (historical evidence only — NOT live authority):**
- `reports/phase1-third-hardening-research-brief.md` — **the immediate predecessor brief** (the *only*
  predecessor brief present at this commit; earlier campaign briefs it mentions were cleaned up and are not in
  the tree). Read it to know exactly what the lock-layer lens already covered, so this alignment pass does not
  re-commission settled structural work.
- `archive/specs/0004_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md` — the completed third pass and
  its `THIRD-AC-###` requirements; the primary statement of what is already structurally locked. **Re-verify
  skeptically rather than assume.**
- `archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` and
  `archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` — the spine and
  TUI hardening passes (`SPINE-AC-###`, `TUI-AC-###`); confirm what they landed.
- `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` and
  `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` — the original Phase-1 /
  Phase-1A implementation **intent** and the one prior doc-alignment reconciliation.

> **Do NOT read the later-phase archived specs (`0004_PHASE_2A` epistemics; `0005–0008` Phase-3A
> needs/routines/no-human) as targets.** Their code is out of scope (see §3.2). Consult them only to confirm a
> Phase-1 boundary line when a seam you are auditing is shared with later-phase code.

**Code seams to inspect (read directly from the commit; cite `file:line`, do not paste wholesale).** Confirm
exact paths against the manifest — names below were observed during repo exploration but verify before citing.
Scope is the **Phase-1 / Phase-1A block + the lock layer**; the Phase-2A/3A modules listed only as boundary
markers are *not* audit targets.

- **Kernel spine — event log / replay / determinism (in scope):**
  `crates/tracewake-core/src/events/{mod, log, envelope, apply, mutation}.rs` (envelopes, schema versioning,
  append-only log, the sole event-application path, mutation sealing); `crates/tracewake-core/src/replay/{mod,
  rebuild, report}.rs`; `crates/tracewake-core/src/checksum.rs` (canonical, order-independent checksums +
  coverage); `crates/tracewake-core/src/scheduler.rs` (deterministic `OrderingKey`, phases, no-human advance);
  `crates/tracewake-core/src/time.rs` (discrete `SimTick`; no wall-clock).
- **Kernel spine — pipeline / state / proposals (in scope):**
  `crates/tracewake-core/src/actions/{mod, pipeline, proposal, registry, report}.rs` and the **Phase-1**
  `actions/defs/**` (movement, openclose, takeplace, inspect, wait — the no-direct-dispatch boundary, typed
  reason codes, source-context validation); `crates/tracewake-core/src/state.rs` (authoritative state — verify
  collections are `BTreeMap`/`BTreeSet` and fields sealed); `crates/tracewake-core/src/{projections,
  view_models, controller, debug_capability, debug_reports}.rs` (view-model construction, embodied
  projections, the debug-truth capability boundary).
- **Content (in scope):** `crates/tracewake-content/src/{schema, load, validate, manifest, serialization}.rs`
  and the **Phase-1** fixtures under `crates/tracewake-content/src/fixtures/**` (typed schema, deterministic
  load, no-script/no-prose-born-fact validation, canonical serialization, golden-fixture contracts).
- **TUI seam (in scope):** `crates/tracewake-tui/src/{main, lib, app, run, render, input, debug_panels,
  transcript, launch}.rs` and `crates/tracewake-tui/tests/**`.
- **The structural-lock layer (in scope — re-verify it actually enforces what the docs require, and is run by
  CI on the pinned toolchain):**
  `crates/tracewake-core/tests/{spine_conformance, no_human_capstone, doc_invariant_references,
  anti_regression_guards, negative_fixture_runner, event_schema_replay_gates, hidden_truth_gates,
  acceptance_gates, acceptance_artifact_wording, golden_scenarios}.rs`;
  `crates/tracewake-content/tests/{forbidden_content, schema_conformance, fixtures_load,
  golden_fixtures_run}.rs` (confirm exact names against the manifest);
  `crates/tracewake-tui/tests/{tui_seam_conformance, adversarial_gates, tui_acceptance, command_loop_session,
  embodied_flow, transcript_snapshot}.rs` (confirm names); `clippy.toml` (at repo root — disallowed
  types/methods); `.github/workflows/ci.yml` (does CI actually run the conformance/linter/guard tests and
  clippy `-D warnings` on the locked toolchain?); root `Cargo.toml`; `rust-toolchain.toml`.
- **Boundary markers only — NOT audit targets** (recognize them so you don't mis-flag uncertified code):
  `crates/tracewake-core/src/epistemics/**`, `crates/tracewake-core/src/agent/{need, routine, htn, intention,
  candidate, generation, methods, planner}.rs`, and the Phase-2A/3A `actions/defs/{eat, sleep, work,
  checkcontainer, continue_routine, accuseprobe}.rs`.

---

## 3. Settled intentions (final — these pre-empt every question)

These decisions came out of a full repo-grounded interview. Treat them as committed.

1. **Lens = doc↔code *alignment conformance*, not another lock-layer re-audit.** The three prior passes asked
   *"can the guards be bypassed."* This pass asks *"does the Phase-1 code actually conform to every doctrine
   mandate that bears on it."* Systematically walk the tiers (foundation → architecture → execution →
   reference → specs) and, for each in-scope mandate, produce a **conformance verdict** with `file:line`
   evidence on both the doc side (tier + file:line + `INV-###`) and the code side. The prior "substantially
   clean" judgment was reached structurally, not by an exhaustive conformance walk — **treat it skeptically;
   do not re-run the lock-layer audit and declare clean.**

2. **Code scope = Phase 1 / Phase 1A block + the lock layer — ONLY.** Audit the kernel spine (event
   log/envelopes/schema versioning, replay + projection rebuild, checksums, determinism/ordering/randomness,
   the proposal/validation/scheduling pipeline, no-direct-dispatch, scheduler/no-human advance, authoritative
   state), the TUI/view-model/debug seam, the content schema/load/validation/serialization seam, and the
   structural-lock layer. **Phase 2A (epistemics) and Phase 3A (needs/routines/planner) code present in
   `crates/` is OUT of scope** — live doctrine brackets it as landed-but-uncertified history. Read later-phase
   docs only to fix the boundary, so uncertified 2A/3A code is **never** flagged as "misaligned." If a seam is
   genuinely shared between Phase-1 and later phases (e.g. a single pipeline or state module), audit only the
   Phase-1 surface of it and say so explicitly.

3. **Drift direction = code yields by default; doc defects flagged separately.** When code and docs diverge,
   the **default corrective action is changing code to match the docs** (foundation/architecture/execution
   outrank code). But where a doc is **genuinely the stale/wrong party**, the spec must NOT silently
   code-around it: it records a **Required upstream amendment** — to the specific foundation/architecture/
   execution document — in a clearly-labeled, separate section, because a `4-specs`-tier spec **cannot** amend
   a higher tier. Justify every doc-is-wrong call with file:line + invariant reasoning; default to
   code-yields unless the doc defect is demonstrable.

4. **Always produce one self-contained markdown document; its FORM follows the verdict.** Session 2 first
   reaches a clearly-labeled, **file:line-grounded verdict** ("are there genuine Phase-1 doc↔code
   misalignments, and what are they"). Then:
   - **If misalignment(s) found →** a full **doc↔code alignment-correction + anti-contamination spec** (Branch
     A, §7).
   - **If clean →** a self-contained **rationale report** (Branch B, §7) that *proves* the Phase-1 code aligns
     with every in-scope mandate.
   Either way the deliverable is one complete, paste-ready markdown document — never a stub, never "it
   depends." Anti-contamination *suggestions* appear in an appendix in **both** branches.

5. **Anti-contamination = scoped structural lock per finding; general harness as a suggestion only.** For each
   misalignment found, specify a durable structural mechanism that makes that drift impossible to reintroduce,
   pushed up the **convention → test → compile-time** ladder wherever practical (type-state, sealing,
   capability tokens, `clippy.toml disallowed-*`, conformance test, schema/replay-coverage parity, CI
   cross-check). Any **repo-wide doc↔code conformance harness** (executable-spec / architecture-fitness-function
   style — binding doctrine mandates to executable checks across the tree) goes in an **additional-suggestions
   appendix**, flagged for the maintainer, **not** as an in-scope deliverable. This preserves campaign
   discipline and avoids scope blow-up. Where the existing lock layer already covers a dimension, reuse/extend
   it rather than inventing a parallel mechanism.

6. **Material-risk-only warrant bar; an honest "no" is acceptable.** A correction spec is warranted **only**
   for genuine misalignments that materially threaten the goals: (a) Phase-1 code conforms to `docs/**`;
   (b) because it conforms and is locked, it is the best foundation for further work; (c) it is *structurally
   impossible*, to the extent practical, for later implementations to break that alignment. Cosmetic/stylistic
   items are not warrant-worthy — they go in the suggestions appendix. After three hardening passes, a
   well-evidenced **"no misalignment; block is aligned"** verdict (Branch B) is a fully acceptable outcome and
   must not be inflated into a spec.

7. **Locked posture, source discipline, no scope creep.** Archived specs are historical evidence only. **Do
   not advance the campaign into later blocks** — no epistemics (`0004_PHASE_2A`), no needs/routines/no-human/
   ordinary-life (`0005–0008`), no institutions, no Phase-2+/LLM/graphical-client scope, no new world
   mechanics. Gate codes (`SPINE-CERT`, `EPI-CERT`, `P0-CERT`, `NO-DIRECT`, `REPLAY`, `FIXTURE`, `DIAG`, …)
   appear only as **cross-references** — never redefined or weakened. Use `holder-known context` as the
   system-wide term and `actor-known` for the actor case. No backwards-compat shims or alias paths.

8. **`assumption:`** the deliverable, if a spec, is staged as
   `specs/0005_PHASE_1_DOC_CODE_ALIGNMENT_CONFORMANCE_AND_ANTI_DRIFT_SPEC.md` (number chosen to continue the
   visible `0002 → 0003 → 0004` staging sequence; the staging directory `specs/` is currently empty), with
   final home `docs/4-specs/`. **Confirm the number and path against the live `SPEC_LEDGER.md` +
   `4-specs/README.md` at the target commit and state your choice** — note that `archive/specs/` already
   contains differently-scoped `0005–0008` files from an earlier epoch, so a same-number collision in the
   archive is expected by the staging→archive convention and is not a blocker; if the maintainer prefers a
   different number, only the header/path changes (one-line correctable). If the verdict is "clean", name the
   rationale report `reports/phase1-doc-code-alignment-audit.md` and say so.

9. **`assumption:`** the "one alignment problem" the maintainer recalled from the recent campaign traces to
   `0003_PHASE_1A` (a README/`SPEC_LEDGER` over-claim reconciliation), **not** to `0004_PHASE_1_THIRD`, which
   surfaced none. Use this only as context; it is not evidence that a current misalignment exists. Reach your
   own verdict from the code and docs at the target commit.

---

## 4. The task

Conduct a **doc↔code alignment conformance audit** (target type: *hardening / anti-contamination*; secondary:
*new spec*) of the Tracewake **Phase 1 / Phase 1A block + the structural-lock layer** against the full live
doctrine tree (`docs/0-foundation` → `docs/4-specs`), with `file:line` evidence on both the doc and code
sides and skeptical scrutiny of the prior "substantially clean" verdict. For each in-scope doctrine mandate,
determine whether the Phase-1 code conforms; for each genuine misalignment, determine the corrective direction
(code yields by default; doc defect flagged upstream per §3.3) and a structural lock that prevents recurrence
(§3.5). On the material-risk-only bar (§3.6), decide whether a correction + anti-contamination spec is
warranted, then produce **exactly one** self-contained markdown document whose **form follows the verdict**
(§3.4, §7). A warranted spec must be the kind a later `spec-to-tickets` pass could decompose without
re-litigating intent.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files in §2 — follow `use`/`mod` paths into any in-scope
seam, and read whatever fixtures/tests prove (or fail to prove) conformance. Where you assess a guard, *trace
what it actually walks and matches* rather than trusting its name. Where you assert a doc mandate, quote the
controlling sentence (tier + file:line).

Research online as deeply as needed and **cite sources** for any external claim that shapes a decision.
High-value directions for *this* target — keeping docs and code provably aligned, and making drift structurally
impossible:

- **Architecture-conformance / fitness-function testing** — automated tests that fail the build when code
  violates an architectural or doctrinal rule (dependency direction, layering, banned APIs, mandated
  invariants); "architecture fitness functions"; ArchUnit-style conformance in other ecosystems and Rust
  analogues.
- **Executable specifications / doc-as-code / specification-by-example** — binding prose mandates to executable
  checks so a doc and its implementation cannot silently diverge; traceability matrices from requirement →
  test; literate/"living documentation" approaches and their failure modes.
- **Making invariants structural, not just tested (Rust)** — type-state, capability, sealed-trait, newtype,
  private-constructor patterns to make illegal states unrepresentable and privileged operations unreachable
  from the wrong layer; "parse, don't validate"; enforcing module/dependency boundaries with visibility,
  `cargo-deny`, and clippy `disallowed-methods`/`disallowed-types`.
- **Limits of lint/grep-style guards** — why source-token / substring scanners are brittle (aliasing,
  re-export, macros, comments/strings, conditional compilation, unwalked files) and what raises a guard from
  "convention/test" to "compile-time guaranteed"; testing a guard's own coverage (mutation testing; negative
  fixtures that *should* trip it).
- **Event-sourcing kernel discipline & deterministic simulation** — append-only logs, command/event separation,
  versioned events + upcasting, projections as rebuildable read models; the canonical sources of nondeterminism
  (hash-map iteration order, floating point, wall-clock, unseeded RNG, thread scheduling) and how engines
  *structurally* exclude them. (Relevant where alignment findings touch the spine.)
- **Anti-contamination / information-firewall patterns** — keeping privileged "ground truth" structurally
  unable to reach a derived/presented surface; capability-based / object-capability discipline.

Use external prior art to sharpen the deliverable's *findings, requirements, and structural gates* — not to
import scope the intentions forbid.

---

## 6. Doctrine & constraints (honor these)

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every finding and requirement must
  cite the exact `INV-###` it bears on (verified against the file, not numbers quoted in archived specs). A
  genuine divergence requires amending an invariant first (a Required-upstream-amendment flag, never designing
  against it silently).
- **Authority order:** foundation → architecture → execution → reference → specs. If execution conflicts with
  architecture/foundation, execution is wrong; if implementation is more convenient than the accepted gates,
  implementation is wrong. A spec may operationalize higher-tier doctrine; it may **not** amend invariants,
  replace architecture, define/weaken gate semantics, or promote archived history into certification.
- **Anti-contamination is the point:** no simulation fact may be born from prose; preserve event-sourced
  causality (meaningful change ⇒ event; one append-only log; rebuildable projections; deterministic replay),
  subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads, TUI-first
  playability, validation/replay. Debug/ground truth is quarantined and structurally unable to feed embodied
  cognition, affordances, projections, or tests-masquerading-as-knowledge.
- **Diagnostics:** typed provenance and typed reason codes — never string-prefix, substring, or display-label
  behavior — as the basis for reason reporting and audit findings.
- **Determinism:** no wall-clock, OS/process randomness, network, filesystem-during-resolution, thread races,
  `HashMap`/`HashSet` iteration order, or terminal-frame timing in any outcome-affecting path. Any randomness
  must be seeded, scoped, recorded in events, and replay-checked.
- **No new mechanics, no scope creep, no later-block advance, no LLM surfaces, no backwards-compat shims or
  alias paths, no graphical client.**
- Workspace gates the resulting work must ultimately satisfy (state them in the deliverable's acceptance
  section; do not run them): `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D
  warnings`; `cargo build --workspace --all-targets --locked`; `cargo test --workspace`. Any new lint/CI/
  conformance gate the deliverable introduces must be expressible within the pinned `rust-toolchain.toml`.

---

## 7. Deliverable specification

Produce **one new downloadable markdown document — always**, whose form is contingent on the verdict (§3.4).
It is a new file: it does not replace any existing file, and must not edit, restate-as-authority, or rewrite
any doc in `docs/0-foundation`, `docs/1-architecture`, `docs/2-execution`, `docs/3-reference`, the archived
specs, or code. It references them.

**This is a determination-plus-conditional deliverable.** Session 2 must (a) produce a clearly labeled,
evidence-based **verdict** (are there genuine Phase-1 doc↔code misalignments, and what are they) on the
material-risk-only bar of §3.6, and (b) shape the document accordingly.

**Branch A — verdict POSITIVE (one or more genuine misalignments).** Produce a **doc↔code alignment-correction
+ anti-contamination spec**:

- **Filename (assumption, confirm against the live ledger):**
  `0005_PHASE_1_DOC_CODE_ALIGNMENT_CONFORMANCE_AND_ANTI_DRIFT_SPEC.md`
- **Intended path:** `specs/0005_…md` (staging), final home `docs/4-specs/`.
- Minimum contents:
  1. **Header & baseline statement** — repository, analyzed commit
     `b210e4069c1ec997ed839dca34840ac72058b477`, the source-discipline note (manifest = path inventory;
     archived baselines are historical), and exactly one declared execution admissibility posture (`P0-CERT
     scoped remediation` / `P0-CERT not applicable` — justified from evidence; a correction spec that changes
     code/tests/CI is almost certainly `scoped remediation`, not `not applicable`).
  2. **Authority chain & gate mapping** — the controlling foundation/architecture/execution/reference docs and
     the `SPINE-CERT`/`EPI-CERT`/`P0-CERT`/`NO-DIRECT`/`REPLAY`/`FIXTURE`/`DIAG` mapping **as cross-references
     only**.
  3. **Scope & non-goals** — Phase-1 / Phase-1A block + the lock layer; explicit non-goals: Phase-2A/3A code,
     later blocks, new mechanics, LLM, graphical client, redefining gate semantics, and **performing**
     higher-tier doc amendments inside this spec (those are flagged, see item 7).
  4. **Alignment findings** — the conformance walk, organized by doctrine dimension (event-log/append-only/
     schema versioning; replay + projection rebuild; checksum coverage; determinism/ordering/randomness;
     no-direct-dispatch / single mutation boundary; scheduler / no-human; content schema/load/validation/
     serialization; possession parity; holder-known/truth-firewall; debug quarantine; typed diagnostics;
     transcript/replay stability; crate dependency direction; terminology/glossary conformance). For **each**
     mandate audited: the **doc side** (tier + file:line + `INV-###`), the **code side** (`file:line`), a
     verdict **aligned** vs **misaligned**, and for each misalignment the **corrective direction** (code-yields
     vs flagged-upstream-doc-defect, per §3.3). Show evidence for *aligned* dimensions too — the value of this
     pass is the explicit walk, not only the exceptions.
  5. **Structural anti-contamination requirements** — for each misalignment, a durable, testable requirement
     that locks the corrected intent: the invariant(s) it enforces, the drift it prevents, and — crucially —
     **how it is structurally enforced**, pushed up the convention → test → compile-time ladder (§3.5). Where
     an existing lock-layer mechanism already covers the dimension, extend it; name the file.
  6. **Required fixtures & tests** — positive **and adversarial/negative** gates, including **negative fixtures
     that should trip each new guard** (proving the guard's own coverage). Harden existing fixtures rather than
     author parallels.
  7. **Required upstream doc amendments** (separate, clearly labeled) — any finding where the **doc** is the
     stale/wrong party: name the exact foundation/architecture/execution document and the amendment the
     maintainer must make **before** the corresponding code requirement is valid. This spec records them; it
     does not perform them.
  8. **Acceptance checklist** — the four workspace gates, any new lint/CI/conformance gate, a per-requirement
     acceptance condition, and certification-result wording that conforms to
     `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` and does not over-claim project certification.
  9. **Additional-suggestions appendix** — the repo-wide doc↔code conformance harness (§3.5) and any
     cross-block mechanisms, flagged as suggestions; note which predecessor suggestions landed vs. remain open.

**Branch B — verdict NEGATIVE (no genuine misalignment; the block is aligned).** Produce a self-contained
**rationale report**:

- **Filename (assumption, confirm/adjust):** `phase1-doc-code-alignment-audit.md` under `reports/`.
- Minimum contents: the labeled verdict and its reasoning; the **conformance walk** with `file:line` doc-side
  and code-side evidence (and `INV-###`) for each dimension above, proving alignment; an explicit demonstration
  that the Phase-2A/3A boundary holds (those systems are correctly out of scope, not silently mis-flagged);
  what the block contributes toward `P0-CERT` in the live posture vocabulary; and a suggestions appendix for
  any non-warrant-worthy observations and the repo-wide doc↔code conformance harness.

In both branches, keep the document self-consistent with `docs/4-specs/README.md`'s rules (posture declaration
where applicable; gate codes as cross-references; correct holder-known/actor-known terminology; archived specs
as history; no files for symmetry).

> **Locked / no-questions:** Produce the deliverable directly as a downloadable markdown document. Do not
> interview, do not ask clarifying questions — the requirements above are final. If a genuine contradiction
> makes a requirement impossible, state it inside the document and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] Every file in §2 was fetched from commit `b210e4069c1ec997ed839dca34840ac72058b477`; the manifest was
      used only to enumerate paths; archived-spec baselines were NOT used as fetch targets.
- [ ] The deliverable is exactly one new markdown document whose **form matches the verdict** (correction spec
      if misaligned, rationale report if aligned); it edits/replaces nothing and restates no upstream tier as
      local authority.
- [ ] A clearly labeled, **file:line-grounded verdict** is stated up front on the **material-risk-only** bar;
      an honest "aligned" verdict was not inflated into a spec, and any positive verdict is justified by
      concrete, evidenced misalignment.
- [ ] The audit is an **alignment-conformance walk** of the full doctrine tree against the Phase-1 code — not a
      re-run of the lock-layer-robustness audit; the prior "substantially clean" verdict was treated
      skeptically.
- [ ] **Scope held to Phase 1 / Phase 1A + the lock layer.** Phase-2A epistemics and Phase-3A
      needs/routines/planner code were NOT audited and were NOT mis-flagged as misaligned; the boundary is shown
      explicitly. No campaign advance into later blocks.
- [ ] Each audited mandate carries doc-side (tier + file:line + `INV-###`, numbers verified against
      `02_CONSTITUTIONAL_INVARIANTS.md`) **and** code-side (`file:line`) evidence; aligned and misaligned
      dimensions are both shown.
- [ ] For each misalignment, the **corrective direction** is stated (code-yields by default; genuine doc
      defects flagged as Required upstream amendments in a separate section — never code-around-a-stale-doc).
- [ ] For each misalignment, a **structural anti-contamination lock** is specified and pushed as far up the
      convention → test → compile-time ladder as practical; existing lock-layer mechanisms were extended rather
      than duplicated; a repo-wide doc↔code conformance harness appears only as a flagged suggestion.
- [ ] If a spec is produced: exactly one `P0-CERT …` posture is declared and justified; gate codes appear only
      as cross-references; no gate semantics defined or weakened; acceptance wording conforms to the live
      acceptance-artifact template and does not over-claim.
- [ ] Determinism, no-direct-dispatch, event-sourced causality, schema-version, replay/checksum coverage,
      content no-script/no-prose-born-fact, possession parity, and debug quarantine are each tied to an
      invariant and to evidence (a test/fixture, or shown already covered).
- [ ] Terminology matches the glossary (`holder-known` / `actor-known`); no backwards-compat shims or alias
      paths; any new gate is expressible within the pinned `rust-toolchain.toml`.
- [ ] Every external claim that shaped a decision is cited; nothing in the deliverable weakens an upstream tier.
