# Research Brief — Phase 1 Spine Anti-Contamination & Structural Hardening (Tracewake)

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

**Fetch every file from commit `a1a31edb2659bff17fddd5882967d6f3b76381a7`** — the manifest reflects that
exact tree (verified repo HEAD, clean working tree). Construct every raw URL as
`https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/<manifest path>`.
The manifest is **path inventory only** — never source text or authority. The archived specs you will read
cite their own historical baseline commits inside their evidence ledgers (`841deeb…` for Phase 1, `1d27a01…`
for Phase 1A, `bf0e3a0…` for the TUI-proof audit). Those are *that spec's own* provenance and **predate**
the foundation/architecture/execution overhaul and later merges. **Ignore them as fetch targets — fetch
everything from `a1a31ed…`.** Branch names, default-branch lookups, repository metadata, and code-search
snippets are not proof of target-commit content.

This brief is the **successor** to `reports/tui-proof-surface-hardening-research-brief.md`, which commissioned
the now-merged TUI/view-model/debug hardening (archived as
`archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`). Treat that work
as **completed evidence**, not as a target to redo — this pass is a *delta* on the rest of the same block.

---

## 2. Read in full (authority order)

Read these completely, in this order, before producing anything. Each line says why it is load-bearing
*for this target* — a whole-block (Phase 1 / Phase 1A) anti-contamination + structural hardening pass whose
primary surface is the **non-TUI kernel spine** and which also **re-verifies** the already-hardened TUI seam.
Confirm actual module paths against the manifest before citing; do not assume names.

**Doctrine — foundation (binds product identity & invariants):**
- `docs/README.md` — the authority-layering rule; how to resolve cross-tier conflicts.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — the `INV-001…INV-NNN` contract; every requirement
  must cite the exact invariant(s) it enforces. Especially events/determinism/replay/no-script/save and the
  TUI/debug/possession invariants (e.g. `INV-008` UI-not-authority, `INV-009` events, `INV-011`
  event-sourced state, `INV-017`/`INV-018` determinism/replay, `INV-060`/`INV-061` no-authored-outcome,
  `INV-068` non-diegetic debug, `INV-069` view-model boundary, `INV-092` transcript/replay stability,
  `INV-099`–`INV-107` truth-firewall/provenance/typed-diagnostics family — verify exact numbers against the
  file; the TUI-proof spec relied on numbers up to `INV-108`).
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **the controlling spine doctrine.** The
  event-trace / replay contract the kernel must satisfy: meaningful change ⇒ event, append-only log,
  rebuildable projections, deterministic replay. This is the single most load-bearing doc for the deliverable.
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — the no-scripting / no-authored-outcome
  / seed-not-script doctrine that content schema + validation must enforce, and that "no simulation fact born
  from prose" derives from.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — the truth firewall: where
  truth may *validate* vs. where it is forbidden to *plan or render*. The anti-contamination backbone for both
  the spine (events/projections/debug must not leak truth into actor-known state) and the TUI re-verify.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — the first-proof "definition of done"
  and acceptance gates the whole block must satisfy; excludes later-phase scope.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI-as-proof-surface, embodied vs debug,
  view-model boundary, two-layer why-not. Load-bearing for the TUI re-verification subsection.

**Doctrine — architecture (binds the contracts the spine must conform to):**
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — how architecture binds; the conformance
  posture the structural-enforcement layer should align to.
- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — kernel authority,
  the one-way crate dependency direction (`core` zero-dep), and the no-hidden-mutation rule — the foundation
  for compile-time / dependency structural gates.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — **the controlling spine
  architecture:** event log, replay, projections, save packages, and the randomness/seeding contract. The
  decisive doc for the audit's evidence bar.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — the shared
  proposal/validation/scheduling/feedback pipeline and the no-direct-dispatch rule (every world mutation flows
  proposal → pipeline → event apply; nothing else mutates state).
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — holder-known contexts and
  provenance: the substrate proposals/projections must carry and the firewall the spine must not breach.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — the client-boundary
  architecture; the contract the TUI re-verify checks against.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — acceptance,
  observability, and review-artifact requirements; the basis for the conformance-test / CI "teeth" this spec
  specifies.

**Doctrine — execution (binds current sequencing & gates):**
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — gate vocabulary (`P0-DOC`, `P0-CERT`, `SPINE-CERT`,
  `EPI-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`); the rule that
  execution may not soften foundation/architecture.
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — the `P0-CERT`
  proof requirements, the "archived specs are historical evidence only" rule, the code-audit boundary, and the
  three admissible spec postures (`P0-CERT passed` / `scoped remediation` / `not applicable`).
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — gate order; **`SPINE-CERT`**
  (event log, replay, projection, randomness, save, action pipeline, TUI/debug split, no-direct-dispatch) — the
  gate this pass maps to most directly — and `EPI-CERT`; the valid future-spec postures.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — the **mandatory
  anti-contamination gates** every future spec must carry; the load-bearing list this spec's structural gates
  extend rather than redefine.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — the execution-tier
  contract for the scheduler, the action pipeline, and no-direct-dispatch — core spine surface.
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — the data/schema/provenance/
  validation gates the content crate must satisfy (no prose-born facts; no authored outcomes; typed schema).
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — the golden-fixture and
  replay-acceptance contract the spine's regression proof must meet.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — testing/diagnostics
  discipline: **typed provenance over string-prefix/substring/display-label diagnostics**; adversarial negative
  gates, not only friendly golden paths. The bar for the spec's required tests and CI teeth.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — `EPI-CERT` seams; load-bearing
  for the TUI/view-model/debug re-verification.

**Doctrine — reference:**
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — the standing review checklist the spec's
  acceptance section should align to.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — named design risks (debug-truth leakage,
  display-string-as-authority, nondeterminism, weak traceability, `PlayerCharacter`/`Quest`/`Objective`/
  `Reward`, etc.) the hardening must keep closed.
- `docs/3-reference/02_GLOSSARY.md` — canonical terms (holder-known vs actor-known, world vs non-world event,
  etc.) to use precisely.

**Spec tier:**
- `docs/4-specs/SPEC_LEDGER.md` — spec subordination, source discipline, the archived-spec posture table, and
  "next known execution move = `P0-CERT`"; the live spec set (only `0001` is live).
- `docs/4-specs/README.md` — the rules every future spec must obey (posture declaration; gate codes as
  cross-references only; holder-known/actor-known terminology; archived specs as history; no files for symmetry).
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — the one live sibling spec;
  model its structure, posture declaration, and tone.

**Prior work being built on or hardened (historical evidence only — NOT live authority):**
- `reports/tui-proof-surface-hardening-research-brief.md` — the **predecessor brief**. Read it to avoid
  re-commissioning what is done and to frame this pass as the next coherent-block delta.
- `archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` — the
  **completed** TUI hardening (see its findings table `F-01…F-18`, requirements `TUI-AC-001…012`, and Outcome).
  This is the **re-verification baseline**: confirm its gates landed and hold, and that nothing in the spine
  re-introduces the drift it closed.
- `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` — the Phase-1 kernel /
  event-log / replay / determinism / pipeline / content-validation implementation **intent**. The primary
  source of the spine intent being hardened (read its §6–§14 closely: determinism contract, event-log/replay
  contract, pipeline stages, content validation).
- `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` — the executable command
  loop and its mutation/debug/dependency boundaries; relevant to the TUI re-verify and the no-runtime-terminal-dep
  policy.
- `archive/reports/PHASE_3A_IMPLEMENTATION_AUDIT.md` — an existing code-audit artifact; mirror its evidence
  rigor (file:line findings by responsible layer). Style/depth only, not authority.

**Code seams to inspect (read directly from the commit; cite file:line, do not paste wholesale).** Confirm
exact paths against the manifest — module names below are observed at this commit but verify before citing.

- **Kernel spine — event log / replay / determinism (primary):**
  `crates/tracewake-core/src/events/{envelope.rs, log.rs, apply.rs, mod.rs}` (envelopes, schema versioning,
  append-only log, the sole event-application path); `crates/tracewake-core/src/replay/{rebuild.rs, report.rs,
  mod.rs}` (projection rebuild + replay reports); `crates/tracewake-core/src/checksum.rs` (canonical
  physical/agent/context checksums); `crates/tracewake-core/src/scheduler.rs` (deterministic ordering keys,
  no-human advance, schedule phases); `crates/tracewake-core/src/time.rs` (discrete ticks; no wall-clock).
- **Kernel spine — pipeline / state / proposals (primary):**
  `crates/tracewake-core/src/actions/{pipeline.rs, proposal.rs, registry.rs, report.rs, defs/**}`
  (proposal/validation stages, reason codes, mutation plan, no-direct-dispatch boundary, the inert
  architectural slots); `crates/tracewake-core/src/state.rs` (authoritative state; verify all collections are
  `BTreeMap`/`BTreeSet`, fields sealed); `crates/tracewake-core/src/projections.rs`,
  `crates/tracewake-core/src/view_models.rs`, `crates/tracewake-core/src/controller.rs`,
  `crates/tracewake-core/src/debug_capability.rs`, `crates/tracewake-core/src/debug_reports.rs`.
- **Content (primary):** `crates/tracewake-content/src/{schema.rs, load.rs, validate.rs, manifest.rs,
  serialization.rs}` and `crates/tracewake-content/src/fixtures/**` (schema, deterministic load, the validation
  phases incl. no-script/determinism-hazard, canonical serialization, golden-fixture contracts).
- **TUI re-verify (secondary):** `crates/tracewake-tui/src/{main.rs, lib.rs, app.rs, run.rs, render.rs,
  input.rs, debug_panels.rs, transcript.rs, launch.rs}` and `crates/tracewake-tui/tests/**` — confirm the
  `0002_TUI_PROOF` gates (sealed holder-known context, source-context proposal validation, debug capability,
  typed availability/why-not, current-view command submission, no-human quarantine, typed notebook leads,
  deterministic transcript/replay, adversarial gates) are present and not regressed.
- **Tests / conformance / CI surface:** `crates/tracewake-core/tests/**` (e.g. `acceptance_gates.rs`,
  `golden_scenarios.rs`), `crates/tracewake-content/tests/**`, `crates/tracewake-tui/tests/**`; and the
  workspace/CI surface where structural gates would live: root `Cargo.toml`, `rust-toolchain.toml`,
  `.github/workflows/ci.yml`, and any existing `clippy.toml`/`deny.toml`/lint config (locate from the manifest).

> **Scope-drift note.** `tracewake-core` has accreted later-phase modules (`agent/**` needs/routines/HTN
> planner, `epistemics/**` beliefs/observations/contradictions). Those belong to `EPI-CERT`/ordinary-life
> certification, **not** this pass. Inspect them only where they touch the spine you are hardening — e.g. they
> share the event log, replay, checksum, and pipeline — and only to confirm the spine boundary holds. Do **not**
> re-spec epistemics or ordinary-life mechanics. The deliverable is a *spine + whole-block boundary* hardening.

---

## 3. Settled intentions (final — these pre-empt every question)

These decisions came out of a full repo-grounded interview. Treat them as committed:

1. **Target = the whole Phase 1 / Phase 1A block, kernel-spine-first, with a full TUI re-verify.** The primary
   surface is the **non-TUI kernel spine** that the TUI-proof pass did not cover: event log / envelopes /
   schema versioning, replay + projection rebuild, checksums, determinism / randomness / ordering, the action
   proposal/validation/scheduling pipeline core, the no-direct-dispatch mutation boundary, the scheduler /
   no-human advance, and content schema / load / validation / serialization / golden fixtures. **In addition**,
   fully **re-audit** the TUI / view-model / debug seam to confirm the merged `0002_TUI_PROOF` remediation
   held and introduced no new drift.

2. **Deliver ONE hardening / anti-contamination spec — always — with an explicit verdict.** Even though the
   spine appears already disciplined (see intention #4), the deliverable is the spec, not merely a report. It
   must contain a clearly labeled **"Is a new spec warranted" verdict** that states, from evidence, whether the
   block needs hardening and why — but the spec is produced regardless, because its core value is converting
   today's runtime/test guarantees into durable structural locks.

3. **Anti-contamination = structural teeth, grounded in a findings audit (weighted structural).** "Make it
   impossible, to the extent we can, for successive work to break alignment with `docs/**`." The spec must do
   BOTH: (a) a full **file:line findings audit** of the current spine + TUI seam, distinguishing
   *already-satisfied* from *needs-hardening* with evidence for both; and (b) a **structural-enforcement
   layer** that makes drift fail future PRs rather than merely be discouraged. The structural layer is the
   priority. Candidate mechanisms the spec should evaluate and specify where they fit (cite prior art):
   - **compile-time capability / sealing** of the mutation boundary (only the event-application path may mutate
     authoritative state) and the debug-truth boundary (privileged truth constructible only with a sealed
     capability) — newtype / private-field / sealed-trait / uninstantiable-from-truth patterns;
   - **lint / dependency gates** that ban, inside `tracewake-core` outcome paths, `std::collections::HashMap`/
     `HashSet` iteration, `std::time::SystemTime`/`Instant`, `rand`/process randomness, and unrecorded
     nondeterminism (e.g. clippy lints, `#![deny(...)]`, a `deny.toml`/dependency check, or a custom test);
   - a **conformance / anti-contamination test crate or module** that asserts the structural invariants
     (single mutation entry point, schema-version gate, replay-total-coverage, world-vs-non-world stream
     separation) as first-class tests rather than incidental ones;
   - **schema-version migration discipline** — unknown event-schema versions fail loudly; adding a version
     requires a migrator + test;
   - **replay / checksum total-coverage proof** — every field of authoritative state is covered by the
     canonical checksum, and a test fails if new state escapes coverage;
   - **doc↔invariant CI cross-checks** — a mechanism (test or CI step) that fails when an `INV-###` reference
     dangles or when the spine code diverges from a named gate, to the extent practical.

4. **The spine is already substantially clean — harden, do not rewrite.** A repo survey at this commit found:
   authoritative-state collections are `BTreeMap`/`BTreeSet` (no `HashMap`/`HashSet` in outcome paths); no
   `SystemTime`/`Instant`/`rand`/thread usage in core; state is mutated only via `events/apply.rs`; event
   envelopes are schema-versioned (`EVENT_SCHEMA_V1`) and serialized through hand-written canonical line
   formats (no `serde` derive); ordering is a deterministic `OrderingKey` tuple; checksums are canonical and
   insertion-order independent. **Verify each of these independently** at the target commit. Where confirmed,
   the finding is *already-satisfied* and the requirement is to **lock it structurally** so a future PR cannot
   silently regress it — not to re-implement it.

5. **Locked posture, source discipline, no scope creep.** Archived specs are historical evidence only. Add no
   new world mechanics; no Phase-2+/epistemic/ordinary-life/institution/LLM scope; no graphical-client work;
   no backwards-compat shims or alias paths. Gate codes (`SPINE-CERT`, `EPI-CERT`, `P0-CERT`, `NO-DIRECT`,
   `REPLAY`, `FIXTURE`, `DIAG`) appear only as **cross-references** — never redefined or weakened. Use
   `holder-known context` as the system-wide term and `actor-known` for the actor case.

6. **Declare exactly one execution admissibility posture, chosen from evidence.** Most likely
   `P0-CERT scoped remediation` (the spine contributes scoped evidence toward `SPINE-CERT`/`P0-CERT` and this
   spec remediates concrete structural-hardening items the code-audit boundary names). Do not default to
   `not applicable` — this spec affects tests, fixtures, lint/CI config, and possibly core type signatures.
   Justify the chosen posture; do not redefine gate semantics.

7. `assumption:` the deliverable filename is
   `0002_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md`, authored under `specs/` for
   staging, with final home `docs/4-specs/`. It takes the next free **live** spec number `0002` (the live
   ledger shows only `0001`; the archived `0002_*` specs are history and do not block a live `0002`). If the
   maintainer prefers a different number, name, or to place it directly in `docs/4-specs/`, only the
   header/path changes — one-line correctable.

8. **This is one pass in an iterative campaign.** The maintainer will repeat this hardening across each
   coherent block of archived specs (next blocks: `0004` epistemics, `0005–0008` needs/routines/no-human
   ordinary life) until nothing more needs hardening. So beyond the block-specific spec, the deliverable should
   surface — in an **additional-suggestions appendix** — any **cross-block / repo-wide** anti-contamination
   mechanism that would protect *all* future work (e.g. a workspace-wide conformance crate, a shared lint
   profile, a doc-invariant linter), flagged as suggestions for the maintainer, not as in-scope deliverables.

---

## 4. The task

Produce a **hardening / anti-contamination structural spec** (target type: *hardening*; secondary: *new
spec*) for the Tracewake Phase 1 / Phase 1A block, with the **non-TUI kernel spine as the primary surface**
and a **full re-verification of the already-hardened TUI / view-model / debug seam**. The spec must: audit the
current `tracewake-core` event-log/replay/projection/determinism/pipeline/scheduler/checksum seams and the
`tracewake-content` schema/load/validation/serialization seams against the realigned foundation/architecture/
execution doctrine, with file:line evidence; confirm the merged `0002_TUI_PROOF` gates still hold; identify
every place the spine is weak, drift-prone, or contamination-prone (and every place it is already correct);
and specify durable, **structurally enforced** requirements — compile-time capability/sealing, lint/dependency
gates, a conformance test layer, schema-version and replay/checksum-coverage discipline, doc↔invariant CI
cross-checks — that make it *impossible, to the extent practical*, for successive work to break alignment with
`docs/**`. It must include an evidence-based "is a new spec warranted" verdict, declare exactly one execution
admissibility posture, and be the kind of spec a later `spec-to-tickets` pass could decompose without
re-litigating intent.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow `use`/`mod` paths into any
seam, and read whatever fixtures/tests prove (or fail to prove) the spine and the TUI re-verify.

Research online as deeply as needed and **cite sources** for any external claim that shapes a decision.
High-value directions for *this* target:

- **Event-sourcing kernel discipline** — append-only logs, event/command separation, versioned events and
  upcasting/schema migration, projections as derived/rebuildable read models, snapshotting vs. pure replay,
  and how production event-sourced systems prevent the write side from being bypassed.
- **Deterministic simulation / lockstep** — sources of nondeterminism (hash-map iteration order, floating
  point, wall-clock, unseeded RNG, thread scheduling) and how deterministic engines structurally exclude them;
  seeded/recorded RNG; canonical ordering; replay-equality checking and desync detection.
- **Making invariants structural, not just tested (Rust)** — type-state / capability / sealed-trait /
  newtype / private-constructor patterns to make illegal states unrepresentable and privileged operations
  unreachable from the wrong layer; the "parse, don't validate" principle; enforcing module/dependency
  boundaries with visibility, `cargo-deny`, and custom lints; clippy `disallowed-methods`/`disallowed-types`.
- **Architecture-conformance / fitness-function testing** — automated tests that fail a build when code
  violates an architectural rule (dependency direction, layering, banned APIs); "architecture fitness
  functions"; doc-as-code / executable-specification approaches for keeping docs and code aligned.
- **Anti-contamination / information-firewall patterns** — keeping privileged "ground truth" structurally
  unable to reach a derived/presented surface; capability-based security and object-capability discipline.

Use external prior art to sharpen the spec's *requirements and structural gates* — not to import scope the
intentions forbid.

---

## 6. Doctrine & constraints (honor these)

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every requirement must satisfy it;
  a genuine divergence requires amending an invariant first, never designing against it silently. Cite the
  exact `INV-###` each gate enforces.
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
- **No new mechanics, no scope creep, no LLM surfaces, no backwards-compat shims or alias paths, no graphical
  client.**
- Workspace gates the resulting work must ultimately satisfy (state them in the spec's acceptance section, do
  not run them): `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo build --workspace --all-targets --locked`; `cargo test --workspace`. Any new lint/CI gate the spec
  introduces must be expressible within this toolchain (pinned `rust-toolchain.toml`).

---

## 7. Deliverable specification

Produce **one new downloadable markdown document**:

- **Filename:** `0002_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md`
- **Intended repository path:** `specs/0002_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md`
  (staging), final home `docs/4-specs/`. `assumption:` live spec number `0002`; one-line correctable.
- **Status:** new file. It does not replace any existing file. It must not edit, restate-as-authority, or
  rewrite any doc in `docs/0-foundation`, `docs/1-architecture`, `docs/2-execution`, `docs/3-reference`, the
  archived specs, or code. It references them.

The spec must contain, at minimum:

1. **Header & baseline statement** — repository, the analyzed commit
   `a1a31edb2659bff17fddd5882967d6f3b76381a7`, the source-discipline note (manifest = path inventory; archived
   baselines are historical), and exactly one declared execution admissibility posture (`P0-CERT …`) chosen
   from evidence per intention #6, with justification.
2. **Authority chain & gate mapping** — the controlling foundation/architecture/execution/reference docs, and
   the `SPINE-CERT`/`EPI-CERT`/`P0-CERT`/`NO-DIRECT`/`REPLAY`/`FIXTURE`/`DIAG` mapping as cross-references only.
3. **Scope & non-goals** — kernel-spine-first hardening + structural enforcement + TUI re-verification;
   explicit non-goals: new mechanics, epistemics/ordinary-life/institution re-spec, LLM, graphical client,
   redefining gate semantics.
4. **Audit findings** — the current state of the spine **and** the TUI seam with **file:line evidence**,
   organized by anti-contamination dimension (event-log/append-only/versioning; replay + projection rebuild;
   checksum coverage; determinism/ordering/randomness; no-direct-dispatch / single mutation boundary;
   scheduler/no-human; content schema/load/validation/serialization; debug quarantine; typed diagnostics;
   transcript/replay stability). Each finding tagged with the responsible layer and the `INV-###` it bears on,
   and marked **already-satisfied** vs **needs-hardening** with evidence for both. Include a dedicated
   **TUI-seam re-verification** subsection confirming the `0002_TUI_PROOF` `TUI-AC-001…012` gates hold (or
   flagging any regression).
5. **"Is a new spec warranted" verdict** — an explicit, evidence-based determination of whether the block
   needs hardening and why, per intention #2.
6. **Structural hardening requirements & anti-contamination gates** — durable, testable requirements that lock
   the intent, each stating the invariant(s) it enforces, the failure/drift it prevents, and — crucially —
   **how it is structurally enforced** (compile-time capability/sealing, lint/dependency gate, conformance
   test, schema-version/replay-coverage discipline, CI cross-check) rather than only test-enforced. Distinguish
   "structurally guaranteed", "test-guaranteed", and "convention-only" for each, and push items up that ladder
   where practical.
7. **Required fixtures & tests** — positive **and adversarial/negative** gates (direct-mutation-outside-apply
   attempts; forged/stale/privileged proposal attempts; unsupported-schema-version replay; nondeterminism
   injection — e.g. introducing `HashMap` iteration or unseeded draw — caught by a gate; checksum-coverage
   escape; world-vs-non-world stream leakage; debug-truth leakage; content with authored outcomes / prose-born
   facts rejected by validation), in the spirit of execution doc 10's typed-diagnostic and adversarial-gate
   discipline. Where a fixture already exists, harden it rather than author a parallel one.
8. **Acceptance checklist** — including the four workspace gates, any new lint/CI gate the spec introduces, and
   a per-requirement acceptance condition; plus certification-result wording that does not overclaim project
   certification.
9. **Additional-suggestions appendix** — cross-block / repo-wide anti-contamination mechanisms (per intention
   #8), clearly flagged as suggestions, not in-scope deliverables.

Keep the spec self-consistent with `docs/4-specs/README.md`'s rules for future specs (posture declaration;
gate codes as cross-references; correct holder-known/actor-known terminology; archived specs as history; no
files for symmetry).

> **Locked / no-questions:** Produce the deliverable directly as a downloadable markdown document. Do not
> interview, do not ask clarifying questions — the requirements above are final. If a genuine contradiction
> makes a requirement impossible, state it inside the spec and proceed with the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] Every file in §2 was fetched from commit `a1a31edb2659bff17fddd5882967d6f3b76381a7`; the manifest was
      used only to enumerate paths; the `841deeb…`/`1d27a01…`/`bf0e3a0…` baselines in archived specs were NOT
      used as fetch targets.
- [ ] The deliverable is exactly one new file with the §7 filename; it edits/replaces nothing and restates no
      upstream tier as local authority.
- [ ] Exactly one `P0-CERT …` admissibility posture is declared and justified by evidence (not defaulted to
      `not applicable`).
- [ ] Gate codes (`SPINE-CERT`, `EPI-CERT`, `P0-CERT`, `NO-DIRECT`, `REPLAY`, `FIXTURE`, `DIAG`, …) appear only
      as cross-references; no gate semantics are defined or weakened.
- [ ] The primary surface is the **non-TUI kernel spine**; the TUI seam is **re-verified** against the merged
      `0002_TUI_PROOF` gates, not re-specified.
- [ ] Anti-contamination weight is **structural**: each requirement says how it is enforced and is pushed up
      the convention→test→compile-time ladder where practical; the audit grounds it in file:line evidence.
- [ ] Audit findings distinguish **already-satisfied** (with evidence, locked structurally) from
      **needs-hardening**; the claimed-clean properties (BTree collections, no rand/time, single mutation path,
      schema versioning, canonical serialization, deterministic ordering, checksum coverage) were each verified
      at the target commit, not assumed.
- [ ] An explicit evidence-based "is a new spec warranted" verdict is present.
- [ ] Determinism, no-direct-dispatch, event-sourced causality, schema-version, replay/checksum-coverage,
      content no-script/no-prose-born-fact, and debug quarantine are each tied to an invariant and an
      adversarial test.
- [ ] No new mechanics, no epistemics/ordinary-life/institution re-spec, no LLM, no graphical client, no
      backwards-compat shims; terminology matches the glossary (`holder-known` / `actor-known`).
- [ ] Any new lint/CI/conformance gate is expressible within the pinned toolchain and named in the acceptance
      section.
- [ ] Every external claim that shaped a decision is cited.
- [ ] Foundation/architecture invariants are satisfied; nothing in the spec weakens an upstream tier.
