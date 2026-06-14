# Research Brief — Phase 1 Third Hardening Pass: Block & Lock-Layer Re-Audit (Tracewake)

> **For ChatGPT-Pro Session 2 (the deep researcher).** You are *locked*: produce the deliverable
> directly. Do not interview, do not ask clarifying questions — the requirements below are final.
> Upload bundle = this prompt + the manifest `manifest_2026-06-08.txt`.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-08.txt`) is the path inventory of the `joeloverbeck/tracewake`
repository — a causality-first living-world simulation in Rust: an event-sourced kernel, subjective
epistemics, fallible institutions, ordinary agents, and a TUI-first client surface where every meaningful
change leaves a replayable trace. The workspace is three crates with a strict one-way dependency direction:
`tracewake-core` (authoritative kernel, **zero dependencies**) → `tracewake-content` (fixtures / loading /
schema validation, depends on core) → `tracewake-tui` (terminal boundary, depends on core + content).

Docs are layered authority: `docs/0-foundation` → `docs/1-architecture` → `docs/2-execution` →
`docs/3-reference` → `docs/4-specs`. **Earlier tiers govern later ones.** If execution conflicts with
architecture or foundation, execution is wrong; if implementation is more convenient than the accepted
gates, implementation is wrong.

**Fetch every file from commit `82736f5dc9f71d05b32125c26348e4a659c10a53`** — the manifest reflects that
exact tree (verified repo HEAD). Construct every raw URL as
`https://raw.githubusercontent.com/joeloverbeck/tracewake/82736f5dc9f71d05b32125c26348e4a659c10a53/<manifest path>`.
The manifest is **path inventory only** — never source text or authority. The archived specs you will read
cite their own historical baseline commits inside their evidence ledgers (e.g. `841deeb…` Phase 1,
`1d27a01…` Phase 1A, `bf0e3a0…` / `a906a70…` the TUI-proof pass, `a1a31ed…` / `2a37b04…` the spine pass).
Those are *that spec's own* provenance and **predate** the current HEAD and later merges. **Ignore them as
fetch targets — fetch everything from `82736f5…`.** Branch names, default-branch lookups, repository
metadata, and code-search snippets are not proof of target-commit content.

**This brief is the third in an iterative hardening campaign over the same Phase 1 / Phase 1A code block.**
Its two predecessors are *completed* and merged:

- `reports/tui-proof-surface-hardening-research-brief.md` commissioned the TUI/view-model/debug hardening,
  merged as `archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`
  (accepted for the audited TUI seam at `a906a70`).
- `reports/phase1-spine-anti-contamination-research-brief.md` commissioned the kernel-spine structural
  hardening, merged as `archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md`
  (accepted for the remediation commits ending at `2a37b04`). This pass **added a structural-lock layer** —
  conformance capstones, an invariant-reference linter, a banned-API token scanner, a `clippy.toml`
  disallowed-types/methods profile, and content forbidden-construct guards.

Treat both as **completed evidence, not targets to redo.** This third pass is a *delta*: a fresh, skeptical
re-audit of the same block **plus** the new lock layer itself — explicitly under the premise that the two
prior hardening passes did **not** necessarily do everything perfectly.

---

## 2. Read in full (authority order)

Read these completely, in this order, before producing anything. Each line says why it is load-bearing
*for this target* — a third anti-contamination / structural-hardening re-audit of the Phase 1 / Phase 1A
kernel spine, the TUI/view-model/debug seam, **and the structural-lock layer the prior pass introduced**.
Confirm actual module paths against the manifest before citing; do not assume names.

**Doctrine — foundation (binds product identity & invariants):**
- `docs/README.md` — the authority-layering rule; how to resolve cross-tier conflicts.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — the `INV-001…INV-NNN` contract; every finding and
  every requirement must cite the exact invariant(s) it bears on. Especially events / determinism / replay /
  no-script / save and the TUI/debug/possession family (e.g. `INV-008` UI-not-authority, `INV-009` events,
  `INV-011` event-sourced state, `INV-017`/`INV-018` determinism/replay, `INV-060`/`INV-061`
  no-authored-outcome, `INV-068` non-diegetic debug, `INV-069` view-model boundary, `INV-092`
  transcript/replay stability, and the truth-firewall/provenance/typed-diagnostics family used by the prior
  passes — **verify exact numbers against the file**, do not trust numbers quoted in archived specs).
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **the controlling spine doctrine:**
  meaningful change ⇒ event, append-only log, rebuildable projections, deterministic replay. The bar the
  kernel re-audit measures against.
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — the no-scripting / no-authored-outcome
  / seed-not-script doctrine the content schema + validation must enforce; the root of "no simulation fact
  born from prose."
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — the truth firewall: where
  truth may *validate* vs. where it is forbidden to *plan or render*. The anti-contamination backbone for the
  spine (events/projections/debug must not leak truth into actor-known state) and the TUI re-verify.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — the first-proof "definition of done"
  and acceptance gates the block must satisfy; excludes later-phase scope.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI-as-proof-surface, embodied vs debug,
  view-model boundary, two-layer why-not. Load-bearing for the TUI re-verification.

**Doctrine — architecture (binds the contracts the spine must conform to):**
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — how architecture binds; the conformance
  posture the structural-lock layer should align to.
- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — kernel authority,
  the one-way crate dependency direction (`core` zero-dep), the no-hidden-mutation rule — the substrate for
  compile-time / dependency structural gates.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **the controlling spine
  architecture:** event log, replay, projections, save packages, randomness/seeding contract. Decisive for
  the audit's evidence bar.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — holder-known contexts and
  provenance; the substrate proposals/projections carry and the firewall the spine must not breach.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — the shared
  proposal/validation/scheduling/feedback pipeline and the no-direct-dispatch rule (every world mutation
  flows proposal → pipeline → event apply; nothing else mutates state).
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — the client-boundary
  architecture; the contract the TUI re-verify checks against.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — acceptance,
  observability, and review-artifact requirements; the basis for judging whether the lock layer's "teeth"
  are real.

**Doctrine — execution (binds current sequencing & gates):**
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — gate vocabulary (`P0-DOC`, `P0-CERT`, `SPINE-CERT`,
  `EPI-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`); the rule that
  execution may not soften foundation/architecture.
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — the `P0-CERT`
  proof requirements, the "archived specs are historical evidence only" rule, the code-audit boundary, and the
  three admissible spec postures (`P0-CERT passed` / `scoped remediation` / `not applicable`).
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — gate order; **`SPINE-CERT`**
  (event log, replay, projection, randomness, save, action pipeline, TUI/debug split, no-direct-dispatch) —
  the gate this pass maps to most directly — and `EPI-CERT`; the valid future-spec postures.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — the **mandatory
  anti-contamination gates** every future spec must carry; the list this pass's findings/gates extend rather
  than redefine.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — the execution-tier
  contract for scheduler, action pipeline, and no-direct-dispatch — core spine surface.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — `EPI-CERT` seams; load-bearing
  for the TUI/view-model/debug re-verification.
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — the data/schema/provenance/
  validation gates the content crate must satisfy (no prose-born facts; no authored outcomes; typed schema).
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — the golden-fixture and
  replay-acceptance contract the spine's regression proof must meet.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — testing/diagnostics
  discipline: **typed provenance over string-prefix/substring/display-label diagnostics**; adversarial
  negative gates, not only friendly golden paths. The bar both for the spine's tests **and for judging
  whether the prior pass's token/string scanners are sound or brittle**.

**Doctrine — reference:**
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — the standing review checklist the
  deliverable's acceptance section should align to.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — named design risks (debug-truth leakage,
  display-string-as-authority, nondeterminism, weak traceability, `PlayerCharacter`/`Quest`/`Objective`/
  `Reward`, etc.) the hardening must keep closed.
- `docs/3-reference/02_GLOSSARY.md` — canonical terms (holder-known vs actor-known, world vs non-world event,
  etc.) to use precisely.

**Spec tier:**
- `docs/4-specs/SPEC_LEDGER.md` — spec subordination, source discipline, the archived-spec posture table
  (note both prior hardening passes are listed there as accepted-for-their-scope, **not** full-project
  certification), the live spec set (only `0001` is live), and "next known execution move = `P0-CERT`."
- `docs/4-specs/README.md` — the rules every future spec must obey (posture declaration; gate codes as
  cross-references only; holder-known/actor-known terminology; archived specs as history; no files for
  symmetry). **Derive the deliverable's spec number and path from this + the live ledger.**
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — the one live sibling spec;
  model its structure, posture declaration, and tone.

**Prior campaign work (historical evidence only — NOT live authority):**
- `reports/tui-proof-surface-hardening-research-brief.md` — **predecessor brief #1.** Read it to avoid
  re-commissioning settled work and to know exactly what the TUI pass scoped.
- `reports/phase1-spine-anti-contamination-research-brief.md` — **predecessor brief #2** (the immediate
  predecessor). Read it to know what the spine pass scoped, the structural mechanisms it specified, and the
  cross-block suggestions it surfaced.
- `archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` — the
  **completed** TUI hardening (findings `F-01…F-18`, requirements `TUI-AC-001…012`, Outcome). A
  re-verification baseline: confirm its gates landed and still hold.
- `archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` — the **completed**
  spine hardening (its findings, `SPINE-AC-###` requirements, the structural-lock layer, and its
  additional-suggestions appendix). **This is the primary re-verification baseline: confirm every gate landed,
  still holds, and is not bypassable — and scrutinize the lock mechanisms it introduced for brittleness.**
- `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` — the original Phase-1
  kernel/event-log/replay/determinism/pipeline/content-validation implementation **intent** (read its
  determinism, event-log/replay, pipeline-stage, and content-validation sections).
- `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` — the executable command
  loop and its mutation/debug/dependency boundaries (incl. the no-runtime-terminal-dependency policy).

> **Do not read the later-phase archived specs (`0004` epistemics, `0005–0008` needs/routines/no-human) as
> targets.** They are out of scope for this pass (see §3). Consult them only if a spine seam you are auditing
> is shared with later-phase code and you must confirm the Phase-1 boundary still holds.

**Code seams to inspect (read directly from the commit; cite file:line, do not paste wholesale).** Confirm
exact paths against the manifest — names below are observed at this commit but verify before citing.

- **Kernel spine — event log / replay / determinism (primary):**
  `crates/tracewake-core/src/events/{envelope.rs, log.rs, apply.rs, mutation.rs, mod.rs}` (envelopes, schema
  versioning, append-only log, the sole event-application path, the mutation-sealing logic);
  `crates/tracewake-core/src/replay/{rebuild.rs, report.rs, mod.rs}`; `crates/tracewake-core/src/checksum.rs`
  (canonical, insertion-order-independent checksums + coverage); `crates/tracewake-core/src/scheduler.rs`
  (deterministic `OrderingKey`, proposal sequencing, schedule phases, no-human advance);
  `crates/tracewake-core/src/time.rs` (discrete `SimTick`; no wall-clock).
- **Kernel spine — pipeline / state / proposals (primary):**
  `crates/tracewake-core/src/actions/{pipeline.rs, proposal.rs, registry.rs, report.rs, defs/**}`
  (proposal/validation stages, typed reason codes, the no-direct-dispatch boundary, source-context
  validation); `crates/tracewake-core/src/state.rs` (authoritative state — verify collections are
  `BTreeMap`/`BTreeSet` and fields sealed); `crates/tracewake-core/src/{projections.rs, view_models.rs,
  controller.rs, debug_capability.rs, debug_reports.rs}` (view-model construction, embodied projections, the
  debug-truth capability boundary, debug-report construction).
- **Content (primary):** `crates/tracewake-content/src/{schema.rs, load.rs, validate.rs, manifest.rs,
  serialization.rs}` and `crates/tracewake-content/src/fixtures/**` (typed schema, deterministic load, the
  validation phases incl. no-script/determinism-hazard, canonical serialization, golden-fixture contracts).
- **TUI re-verify (secondary):** `crates/tracewake-tui/src/{main.rs, lib.rs, app.rs, run.rs, render.rs,
  input.rs, debug_panels.rs, transcript.rs, launch.rs}` and `crates/tracewake-tui/tests/**`.
- **★ The structural-lock layer introduced by the spine pass (PRIMARY — this is what the prior pass added and
  this pass must scrutinize for bypasses and brittleness):**
  - `crates/tracewake-core/tests/spine_conformance.rs` — maps `SPINE-AC-###` requirements to responsible core
    layers / named test evidence. **Check: does the mapping prove the property, or merely assert a string
    exists?**
  - `crates/tracewake-core/tests/no_human_capstone.rs` — full-system capstone (typed ancestry + replay
    determinism over the no-human-day scenario).
  - `crates/tracewake-core/tests/doc_invariant_references.rs` — the **invariant-reference linter** (scans the
    repo for `INV-###` and fails on references not defined in `02_CONSTITUTIONAL_INVARIANTS.md`). **Check: does
    it only catch *dangling* refs, or does it also detect *unreferenced* / *stale-number* / *miscited*
    invariants? What is its blind spot?**
  - `crates/tracewake-core/tests/anti_regression_guards.rs` — the **banned-API token scanner** (`HashMap`,
    `HashSet`, `SystemTime`, `Instant`, `thread::spawn`, `std::fs::*`, `std::net::*`, etc. in outcome paths).
    **Scrutinize hardest: token/substring scanners are evadable via aliasing (`use … as`), re-export,
    type inference, macro expansion, comments/strings, `cfg(test)` carve-outs, or simply new files the scan
    does not walk. Determine what it provably catches vs. what it misses, and whether `clippy.toml`'s
    `disallowed-types`/`disallowed-methods` is the stronger guarantee.**
  - `crates/tracewake-core/tests/{acceptance_gates.rs, hidden_truth_gates.rs, golden_scenarios.rs}`.
  - `crates/tracewake-content/tests/{forbidden_content.rs, schema_conformance.rs, fixtures_load.rs,
    golden_fixtures_run.rs}` — forbidden-construct guards (quest/reward/player/script + shortcut truth fields)
    and content conformance.
  - `crates/tracewake-tui/tests/{tui_seam_conformance.rs, adversarial_gates.rs, tui_acceptance.rs,
    command_loop_session.rs, embodied_flow.rs, readme_sample_session.rs, transcript_snapshot.rs}`.
  - `clippy.toml` (disallowed types/methods), `.github/workflows/ci.yml` (does CI actually *run* the
    conformance/linter/guard tests and clippy with `-D warnings`, on the locked toolchain?), root `Cargo.toml`,
    `rust-toolchain.toml`.

---

## 3. Settled intentions (final — these pre-empt every question)

These decisions came out of a full repo-grounded interview. Treat them as committed:

1. **Third pass over the same Phase 1 / Phase 1A block — kernel-spine + TUI seam.** Re-audit the full
   non-TUI kernel spine (event log / envelopes / schema versioning, replay + projection rebuild, checksums,
   determinism / ordering / randomness, the action proposal/validation/scheduling pipeline, the
   no-direct-dispatch single mutation boundary, scheduler / no-human advance, authoritative state) **and** the
   TUI / view-model / debug seam, against the realigned `docs/**`. The two predecessor passes
   (`0002_TUI_PROOF`, `0003_SPINE`) are **completed evidence, not targets to redo** — this is a *delta*.
   Approach it skeptically: **do not assume the prior hardening passes did everything perfectly.**

2. **Harden the locks themselves — the structural-lock layer is in scope and under adversarial scrutiny.**
   The conformance capstones (`spine_conformance.rs`, `no_human_capstone.rs`, the per-crate
   `*_conformance.rs`), the invariant-reference linter (`doc_invariant_references.rs`), the banned-API token
   scanner (`anti_regression_guards.rs`), the `clippy.toml` disallowed-types/methods profile, and the content
   forbidden-construct guards (`forbidden_content.rs`) are **first-class audit subjects**. Ask of each: does it
   *prove* the property or merely *assert a string exists*? Can it be bypassed (aliasing, re-export, macros,
   comments/strings, `cfg(test)` carve-outs, unwalked new files, type inference)? Does CI actually run it on
   the locked toolchain? **Token/substring scanners get the most skepticism** — prefer pushing such guarantees
   up the convention → test → compile-time ladder (e.g. `clippy.toml disallowed-*`, visibility/sealing,
   type-state) wherever a scanner is the weakest link.

3. **Warrant bar = material risk only; an honest "no" is acceptable.** A third spec is warranted **only** for
   findings that materially threaten the three campaign goals:
   (a) code fully aligns with `docs/**`;
   (b) because it aligns and is fully hardened, it is the best foundation for further implementation;
   (c) it is *structurally impossible*, to the extent practical, for later implementations to break that
   alignment.
   Genuine `docs/**` misalignment, live contamination paths, and lock weaknesses that let drift slip through
   are warrant-worthy. Cosmetic, stylistic, or nice-to-have items are **not** — they go in a suggestions
   appendix, never the verdict. After two hardening passes, a well-evidenced verdict of **"no third spec is
   warranted"** is a fully acceptable outcome and must not be inflated into one.

4. **Always produce one substantial, self-contained markdown document; its FORM follows the verdict.** Session
   2 first reaches a clearly-labeled, **file:line-grounded verdict** ("is a third hardening spec warranted, and
   why"). Then:
   - **If warranted →** the document is a full **hardening / anti-contamination spec** in the house style:
     findings audit, structurally-enforced requirements, required fixtures/tests, acceptance checklist, one
     declared execution admissibility posture (see #6), and a suggestions appendix.
   - **If clean (not warranted) →** the document is a self-contained **rationale report** that *proves* the
     block is aligned and locked: the verdict, the file:line evidence that closes each anti-contamination
     dimension, an explicit demonstration that the prior passes' gates hold and are not bypassable, and any
     non-warrant-worthy observations relegated to a suggestions appendix.
   Either way the deliverable is one complete, paste-ready markdown document — never a stub, never "it
   depends." The verdict is stated up front; the rest of the document is consistent with it.

5. **Locked posture, source discipline, no scope creep** (inherited from the campaign). Archived specs are
   historical evidence only. **Do not advance the campaign into later blocks** — no epistemics (`0004`),
   needs/routines/no-human/ordinary-life (`0005–0008`), institution, Phase-2+, LLM, or graphical-client scope,
   and no new world mechanics. Gate codes (`SPINE-CERT`, `EPI-CERT`, `P0-CERT`, `NO-DIRECT`, `REPLAY`,
   `FIXTURE`, `DIAG`) appear only as **cross-references** — never redefined or weakened. Use `holder-known
   context` as the system-wide term and `actor-known` for the actor case. No backwards-compat shims or alias
   paths.

6. **If a spec is produced, declare exactly one execution admissibility posture, chosen from evidence.** Most
   likely `P0-CERT scoped remediation` (this pass contributes scoped evidence toward `SPINE-CERT`/`EPI-CERT`/
   `P0-CERT` and remediates concrete items the code-audit boundary names). Do not default to `not applicable`
   if the verdict is positive — a warranted spec affects tests, fixtures, lint/CI config, and possibly core
   type signatures. Justify the posture; do not redefine gate semantics. (A *rationale report* under the
   "clean" branch need not declare a spec posture, but should state, in the live posture vocabulary, what it
   would contribute toward `P0-CERT`.)

7. **Iterative-campaign awareness.** This is one pass in a repeating campaign; the maintainer re-hardens each
   coherent block until nothing more needs hardening. Surface — in an **additional-suggestions appendix** — any
   **cross-block / repo-wide** anti-contamination mechanism that would protect *all* future work (e.g. a
   workspace-wide conformance crate, a shared lint profile extension, a stronger doc↔invariant linter,
   compile-time sealing patterns), flagged as suggestions for the maintainer, **not** as in-scope deliverables.
   Where predecessor brief #2 already surfaced such suggestions, note which landed and which remain open rather
   than re-proposing them blindly.

8. `assumption:` the deliverable is staged as
   `specs/0004_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md` (number chosen to continue the visible
   `0002 → 0003 → 0004` staging sequence and avoid filename collision with the archived `0002_*`/`0003_*`
   specs), with final home `docs/4-specs/`. **Confirm the number and path against the live `SPEC_LEDGER.md` +
   `4-specs/README.md` at the target commit and state your choice.** If the maintainer prefers a different
   number, name, or to place it directly in `docs/4-specs/`, only the header/path changes — one-line
   correctable. (If the verdict is "clean", name the rationale report analogously, e.g.
   `reports/phase1-third-hardening-audit.md`, and say so.)

---

## 4. The task

Conduct a **third hardening / anti-contamination re-audit** (target type: *hardening*; secondary: *new spec*)
of the Tracewake Phase 1 / Phase 1A block — the `tracewake-core` event-log/replay/projection/determinism/
pipeline/scheduler/checksum/state seams, the `tracewake-content` schema/load/validation/serialization seams,
the `tracewake-tui` view-model/debug seam, **and the structural-lock layer the prior spine pass introduced
(conformance capstones, invariant-reference linter, banned-API token scanner, `clippy.toml`,
forbidden-content guards)** — against the realigned foundation/architecture/execution/reference doctrine, with
file:line evidence and skeptical scrutiny that the two prior passes did not necessarily get everything right.
Determine, on a **material-risk-only** bar, whether a third hardening spec is warranted to (a) close any
remaining `docs/**` misalignment, (b) make the block the best possible foundation for further work, and
(c) make it *structurally impossible* — to the extent practical — for later implementations to break that
alignment. Then produce exactly one self-contained markdown document whose **form follows the verdict**: a
full hardening spec if warranted, or an evidence-complete rationale report if the block is clean. A warranted
spec must be the kind a later `spec-to-tickets` pass could decompose without re-litigating intent.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow `use`/`mod` paths into any
seam, and read whatever fixtures/tests prove (or fail to prove) the spine, the TUI seam, and the lock layer.
Where you assess a guard's robustness, *trace what it actually walks and matches* rather than trusting its
name.

Research online as deeply as needed and **cite sources** for any external claim that shapes a decision.
High-value directions for *this* target:

- **Event-sourcing kernel discipline** — append-only logs, command/event separation, versioned events and
  upcasting/schema migration, projections as derived/rebuildable read models, and how production event-sourced
  systems structurally prevent the write side from being bypassed.
- **Deterministic simulation / lockstep** — the canonical sources of nondeterminism (hash-map iteration order,
  floating point, wall-clock, unseeded RNG, thread scheduling) and how deterministic engines *structurally*
  exclude them; seeded/recorded RNG; canonical ordering; replay-equality and desync detection.
- **Making invariants structural, not just tested (Rust)** — type-state / capability / sealed-trait / newtype /
  private-constructor patterns to make illegal states unrepresentable and privileged operations unreachable
  from the wrong layer; "parse, don't validate"; enforcing module/dependency boundaries with visibility,
  `cargo-deny`, and clippy `disallowed-methods`/`disallowed-types`.
- **Limits of lint/grep-style guards** — why source-token / substring scanners are brittle (aliasing,
  re-export, macros, comments/strings, conditional compilation, unwalked files) and what raises a guard from
  "convention/test" to "compile-time guaranteed"; how to test a guard's own coverage (mutation testing,
  negative fixtures that *should* trip it).
- **Architecture-conformance / fitness-function testing** — automated tests that fail a build when code
  violates an architectural rule (dependency direction, layering, banned APIs); "architecture fitness
  functions"; doc-as-code / executable-specification approaches for keeping docs and code aligned (relevant to
  the invariant-reference linter's design).
- **Anti-contamination / information-firewall patterns** — keeping privileged "ground truth" structurally
  unable to reach a derived/presented surface; capability-based security and object-capability discipline.

Use external prior art to sharpen the deliverable's *findings, requirements, and structural gates* — not to
import scope the intentions forbid.

---

## 6. Doctrine & constraints (honor these)

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every finding and requirement must
  cite the exact `INV-###` it bears on (verified against the file, not against numbers quoted in archived
  specs); a genuine divergence requires amending an invariant first, never designing against it silently.
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

Produce **one new downloadable markdown document — always**, whose form is contingent on the verdict (per
intention #4). It is a new file: it does not replace any existing file, and must not edit, restate-as-authority,
or rewrite any doc in `docs/0-foundation`, `docs/1-architecture`, `docs/2-execution`, `docs/3-reference`, the
archived specs, or code. It references them.

**This is a determination-plus-conditional deliverable.** Session 2 must (a) produce a clearly labeled,
evidence-based **verdict** ("is a third hardening spec warranted, and why") on the material-risk-only bar of
intention #3, and (b) shape the document accordingly:

**Branch A — verdict POSITIVE (a third spec is warranted).** Produce a **hardening / anti-contamination spec**:

- **Filename (assumption, confirm against the live ledger):**
  `0004_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md`
- **Intended path:** `specs/0004_…md` (staging), final home `docs/4-specs/`.
- Minimum contents:
  1. **Header & baseline statement** — repository, analyzed commit
     `82736f5dc9f71d05b32125c26348e4a659c10a53`, the source-discipline note (manifest = path inventory;
     archived baselines are historical), and exactly one declared execution admissibility posture (`P0-CERT …`)
     chosen from evidence per intention #6, with justification.
  2. **Authority chain & gate mapping** — controlling foundation/architecture/execution/reference docs, and the
     `SPINE-CERT`/`EPI-CERT`/`P0-CERT`/`NO-DIRECT`/`REPLAY`/`FIXTURE`/`DIAG` mapping as cross-references only.
  3. **Scope & non-goals** — Phase-1 spine + TUI seam + **the lock layer itself**; explicit non-goals: later
     blocks (epistemics/ordinary-life/institution), new mechanics, LLM, graphical client, redefining gate
     semantics.
  4. **Audit findings** — current state of the spine, the TUI seam, **and the lock layer**, with **file:line
     evidence**, organized by anti-contamination dimension (event-log/append-only/versioning; replay +
     projection rebuild; checksum coverage; determinism/ordering/randomness; no-direct-dispatch / single
     mutation boundary; scheduler/no-human; content schema/load/validation/serialization; debug quarantine;
     typed diagnostics; transcript/replay stability; **and a dedicated "lock-layer robustness"
     dimension** — for each prior guard: what it provably catches, what it misses, and whether it is
     bypassable). Each finding tagged with the responsible layer and the `INV-###` it bears on, and marked
     **already-satisfied** vs **needs-hardening** with evidence for both. Include a **prior-gate
     re-verification** subsection confirming `TUI-AC-001…012` and `SPINE-AC-###` still hold (or flagging any
     regression).
  5. **Structural hardening requirements & anti-contamination gates** — durable, testable requirements that
     lock the intent, each stating the invariant(s) it enforces, the failure/drift it prevents, and — crucially
     — **how it is structurally enforced** (compile-time capability/sealing, `clippy.toml disallowed-*`,
     conformance test, schema-version/replay-coverage discipline, CI cross-check), pushed up the
     convention → test → compile-time ladder where practical. Where the finding is a *weak prior guard*,
     specify the stronger replacement (and whether the weak guard is retained, hardened, or removed).
  6. **Required fixtures & tests** — positive **and adversarial/negative** gates, including **negative fixtures
     that should trip each guard** (proving the guard's own coverage), direct-mutation-outside-apply attempts,
     forged/stale/privileged proposal attempts, unsupported-schema-version replay, nondeterminism injection,
     checksum-coverage escape, world-vs-non-world stream leakage, debug-truth leakage, and content with
     authored outcomes / prose-born facts. Harden existing fixtures rather than author parallels.
  7. **Acceptance checklist** — the four workspace gates, any new lint/CI/conformance gate, a per-requirement
     acceptance condition, and certification-result wording that does not overclaim project certification.
  8. **Additional-suggestions appendix** — cross-block / repo-wide mechanisms per intention #7, flagged as
     suggestions, noting which predecessor suggestions landed vs. remain open.

**Branch B — verdict NEGATIVE (the block is clean; no third spec warranted).** Produce a self-contained
**rationale report**:

- **Filename (assumption, confirm/adjust):** `phase1-third-hardening-audit.md` under `reports/`.
- Minimum contents: the labeled verdict and its reasoning; the **file:line evidence** that closes each
  anti-contamination dimension above (including lock-layer robustness); an explicit demonstration that the
  prior passes' gates (`TUI-AC-001…012`, `SPINE-AC-###`) hold and are not bypassable; what the block would
  contribute toward `P0-CERT` in the live posture vocabulary; and a suggestions appendix for any
  non-warrant-worthy observations and cross-block mechanisms.

In both branches, keep the document self-consistent with `docs/4-specs/README.md`'s rules (posture
declaration where applicable; gate codes as cross-references; correct holder-known/actor-known terminology;
archived specs as history; no files for symmetry).

> **Locked / no-questions:** Produce the deliverable directly as a downloadable markdown document. Do not
> interview, do not ask clarifying questions — the requirements above are final. If a genuine contradiction
> makes a requirement impossible, state it inside the document and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] Every file in §2 was fetched from commit `82736f5dc9f71d05b32125c26348e4a659c10a53`; the manifest was
      used only to enumerate paths; archived-spec baselines (`841deeb…`/`1d27a01…`/`bf0e3a0…`/`a906a70…`/
      `a1a31ed…`/`2a37b04…`) were NOT used as fetch targets.
- [ ] The deliverable is exactly one new markdown document whose **form matches the verdict** (spec if
      warranted, rationale report if clean); it edits/replaces nothing and restates no upstream tier as local
      authority.
- [ ] A clearly labeled, **file:line-grounded verdict** is stated up front, decided on the **material-risk-only**
      bar; an honest "no third spec warranted" was not inflated, and a positive verdict is justified by concrete
      threats to the three goals.
- [ ] The **lock layer itself** was audited (conformance capstones, invariant-reference linter, banned-API
      token scanner, `clippy.toml`, forbidden-content guards): for each guard, what it provably catches vs.
      misses, and whether CI runs it on the locked toolchain. Token/substring scanners were scrutinized for
      bypass and, where weak, a stronger (compile-time / `clippy.toml`) replacement was specified.
- [ ] Prior gates (`TUI-AC-001…012`, `SPINE-AC-###`) were re-verified as still holding (or regressions
      flagged); the prior passes were treated skeptically, not assumed perfect.
- [ ] If a spec is produced: exactly one `P0-CERT …` posture is declared and justified (not defaulted to `not
      applicable`); gate codes appear only as cross-references; no gate semantics defined or weakened.
- [ ] Findings carry file:line evidence and a responsible-layer + `INV-###` tag (numbers verified against
      `02_CONSTITUTIONAL_INVARIANTS.md`); already-satisfied vs needs-hardening are distinguished.
- [ ] The campaign was **not** advanced into later blocks; no epistemics/ordinary-life/institution/Phase-2+/
      LLM/graphical-client scope; no new mechanics; no backwards-compat shims; terminology matches the glossary
      (`holder-known` / `actor-known`).
- [ ] Determinism, no-direct-dispatch, event-sourced causality, schema-version, replay/checksum-coverage,
      content no-script/no-prose-born-fact, and debug quarantine are each tied to an invariant and an
      adversarial test (or shown already covered).
- [ ] Any new lint/CI/conformance gate is expressible within the pinned toolchain and named in the acceptance
      section.
- [ ] Every external claim that shaped a decision is cited.
- [ ] Foundation/architecture invariants are satisfied; nothing in the deliverable weakens an upstream tier.
