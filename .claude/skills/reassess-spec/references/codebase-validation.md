# Codebase Validation (Step 3)

Validate every reference from Step 2 against the current codebase and doctrine.

**Parallel grep-batch guard**: when validating via direct `grep` / `find` / `test` calls batched in one parallel turn, guard each with `|| true`. A non-zero exit from one call (grep finding nothing, `test` on a missing path) cancels its siblings in the batch. Zero-match results are expected and valid during validation.

Substep applicability by Pre-Process classification:

| Substep | (a) new | (b) extension | (c) refactor | (d) retroactive |
|---|---|---|---|---|
| 3.0 Cross-file scope | ✓ | ✓ | ✓ | skip |
| 3.1 File paths | ✓ | ✓ | ✓ | ✓ (rigorous) |
| 3.2 Types / schema / contract fields | ✓ | ✓ | ✓ | ✓ (rigorous) |
| 3.3 Modules / functions / commands | ✓ | ✓ | ✓ | ✓ (rigorous) |
| 3.4 Dependencies | ✓ | ✓ | ✓ | ✓ (rigorous) |
| 3.5 Skill-structure | ✓ | if SKILL.md changes structurally | if content moves between SKILLs | skip |
| 3.6 Downstream consumers | ✓ | ✓ | skip | skip |
| 3.7 Upstream spec / phase refs | ✓ | ✓ | ✓ | skip |
| 3.8 Doctrine-contract fidelity | ✓ | if kernel-authority/boundary/acceptance-invariant/visibility/determinism semantics touched | skip | skip |
| 3.9 New-deliverable consumer verification | ✓ | ✓ | skip | skip |
| 3.10 Source-document completeness | ✓ | ✓ | skip | ✓ (rigorous — verify landing) |
| 3.11 Spec structural completeness | ✓ | ✓ | skip | skip |

For specs with >10 references, consider parallel Explore agents organized by theme (max 3). Spot-check agent claims with direct Grep/Read before including in findings — agent results are leads, not facts; trust a direct tool result over an agent claim. Verdicts of *absence* (a symbol/test "doesn't exist", a visibility downgrade) must be re-proven by direct grep before becoming findings — false-absence is the common Explore-agent failure mode.

**Hardening/audit specs** (classification (b), per the Pre-Process profile in `SKILL.md`): the per-finding **Evidence** blocks are the primary reference set — verify or refute each evidence claim against code (CONFIRMED / REFUTED / PARTIAL; a refuted claim can collapse or reshape its finding). Verbatim-verify overturned/re-opened prior-audit claims against the archived predecessor spec under `archive/specs/`, and spot-check-sample any "verified holding" list rather than re-proving every entry.

**Specs whose surfaces aren't implemented yet**: Tracewake is partially implemented — completed phases have a landed Rust workspace (`crates/tracewake-{core,content,tui}`), but a spec for a future phase, or a future-phase deliverable inside an otherwise-landed area, may have no code to grep yet. When a deliverable's surfaces are unbuilt, the consumer-grep substeps have nothing in *code* to grep. Confirm the surface is actually unbuilt (`find crates -name '*.rs'`, grep the crate tree) rather than assuming it — then handle the unbuilt case explicitly instead of skipping silently:

- **3.6 Downstream Consumers**: when the modified surface is unbuilt, record `N/A — surface not yet implemented; consumers are planned future phases` instead of an empty grep, since there are no call sites yet. When the surface *has* landed, grep the crate/module tree (`crates/`) normally for real call sites.
- **3.9 New-Deliverable Consumer Verification**: the deliverables' consumers are **planned**, not present — the later phases named in `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`, the per-surface execution docs (`docs/2-execution/04–12`), the `docs/4-specs/SPEC_LEDGER.md` "next allowed spec", and the spec's own Relationship-to-prior-spec / Phase-entry sections. That satisfies the "explicitly planned" branch; do NOT fire the zero-consumer HIGH Issue for an unbuilt deliverable whose consumers are unbuilt-but-sequenced future phases. Record the planned consumers (the phase ladder entries) for the audit trail.

This is distinct from `SKILL.md`'s "no greenfield approach proposals" guardrail, which is about not proposing alternative designs — the unbuilt-surface case here is about a *spec surface* (or module) that is not yet implemented, not about proposing new designs.

## 3.0 Cross-File Scope Establishment

For patterns referenced across multiple files (record/field usage, fixture references, event-kind invocations, doc-path citations), run a cross-file count grep first to establish full scope before per-file analysis. Compare the spec's claimed locations against the actual count — this catches files the spec missed and prevents incomplete deliverables.

## 3.1 File Paths

Glob/Grep to confirm each path exists. If moved, renamed, or deleted, record the actual location. Distinguish existing paths (must exist now — doc-pack files, sibling specs, fixtures) from proposed paths (exist after implementation — recommended crate/module files); proposed paths still need their parent directory to exist (or to be created by a named deliverable), must not collide with an existing file, and must follow conventions (`NNNN_TITLE.md` for specs under `specs/` and `docs/4-specs/`, the tiered `docs/{0-foundation,1-architecture,2-execution,3-reference}/NN_*.md` doc layout, kebab-case skill dirs under `.claude/skills/`).

**Doc-tier path drift**: the doc pack is tiered and numbered. When a spec cites a doctrine file, verify both the tier folder and the `NN_` prefix — a doc reorg renumbers or moves files, and a stale `docs/0-foundation/02_FOUNDATIONAL_INVARIANTS.md` (the constitution is `02_CONSTITUTIONAL_INVARIANTS.md`) or a wrong-tier citation passes a loose existence check silently. Confirm against `docs/README.md` and the per-tier `00_*_INDEX.md`.

**Code-tree path prefix**: source-file references resolve under `crates/<crate>/src/...` or `crates/<crate>/tests/...` — the three crates are `crates/tracewake-{core,content,tui}`. A spec that writes a bare `tracewake-core/src/...` (the crate *name* without the `crates/` prefix), or a bare module-relative path (`actions/defs/foo.rs`, `parity/`), does not resolve as written; sibling specs under `archive/specs/` (e.g. `0046_*`) are the convention exemplar. Distinguish a crate-*name* mention (`tracewake-core stays dependency-free`), which is fine as prose, from a *file-path* reference, which needs the full `crates/<crate>/...` prefix — flag the latter, leave the former.

**Name-collision check for proposed paths**: when a spec proposes a NEW file (a new doc, a new fixture file, a new module), list the parent directory for SIMILAR filenames (substring match on the distinctive token, not exact-path). A proposed file whose parent already holds a near-name sibling occupying the same conceptual slot is a HIGH Issue — the spec should MODIFY the existing file instead. An exact-path existence check passes this silently.

## 3.2 Types, Schema, and Contract Fields

Grep for each type, record, struct, or schema/contract field. Confirm existence and current shape (in code once it lands; in doctrine/fixture contracts until then). Check:

- **Field existence and naming** — flag fields the spec assumes but that don't exist or have different names/types. The contract-authoritative sources for Tracewake are the architecture tier — `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` (event log, envelope, causal graph, snapshots, save package, schema migration, random streams), `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` (typed claims, belief, memory, actor-knowledge filtering) — and the execution schema/fixture contracts `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` (entity/component state model, content packs, schemas, fixtures, content validation) and `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` (golden fixtures, replay acceptance), plus Spec 0001's fixture/ontology contracts. When the spec defines, serializes, or validates one of these structures, verify field names against the architecture/execution contracts and any landed code, not against constitutional-invariant prose (the invariants describe *intent*; the architecture/execution docs carry the *actual* contract surface, and the two can drift).
- **Type accuracy** — verify assumed types match actual types.
- **Field-choice drift** — when a contract offers multiple semantically-distinct fields and the spec's algorithm picks one, verify it picked the semantically-correct one (both fields exist, so a name-drift check passes silently); a wrong choice with correctness consequences is HIGH or CRITICAL.
- **Enum / table exhaustiveness** — if the spec includes a lookup keyed by a string enum (e.g. event-kind → applier, claim family → handler, evidence category → check), verify it covers all current values.
- **Schema fidelity** — if the spec proposes a JSON/TOML schema or a content-manifest entry, verify against `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` and `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`. Confirm the schema rejects unknown fields and refuses behavior-looking fields (no-scripting / acceptance-invariant discipline).

## 3.3 Modules, Functions, and Commands

Grep for each module, function, struct method, CLI tool, or command the spec names; confirm placement, signature, and (once code lands) export status. Line-number references are informational — verify they point to the claimed content; if drifted, correct them or replace with grep-stable symbol names. Check:

- **Signature differences** from what the spec assumes; **parameter sufficiency** at every call site.
- **Module / boundary placement** — verify the symbol lives in the layer the spec claims (kernel vs content vs TUI/view-model vs debug), per the spec's recommended workspace shape and `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`. A domain/mechanic symbol the spec places in the authoritative kernel that must instead live in content, or a presentation symbol given world authority, is a boundary finding (kernel-authority / dependency-direction).
- **Reuse opportunities** — for each new function/module the spec proposes, grep for an existing one (or an already-specced one) serving the same purpose; a duplicate is an Issue (prefer reuse) or Improvement (note the alternative).
- **Code-example fidelity** — Before/After snippets and Rust signatures must match the actual (or already-specced) control-flow structure and types.
- **Pseudocode dependency completeness** — each call/constructor in spec pseudocode must either exist or be defined as a deliverable elsewhere in the spec; neither = an incomplete deliverable Issue.
- **Surface-convention fidelity** — proposed CLI tools, flags, module names, or APIs should match the spec's recommended workspace shape and the architecture boundaries; flag deviations.

## 3.4 Dependencies (phases / specs / skills)

- **Spec / phase sequencing**: verify the spec's `authority order` list resolves to existing doc-pack files, and that its Relationship-to-prior-spec section's predecessor resolves to an existing spec (`specs/*`, `docs/4-specs/*`) and to its `docs/4-specs/SPEC_LEDGER.md` entry. For a predecessor spec the target claims is accepted/landed, verify the ledger status actually reads that; a spec that assumes its predecessor is settled when the ledger shows it is still `Proposed` is a sequencing Issue. For a phase reference that resolves to no spec file yet, that is expected for not-yet-specced phases — flag only if the spec treats an unwritten predecessor/successor as if it exists.
  - **Parenthetical scope-claim verification**: for a sequencing entry with a parenthetical scope claim (e.g. `Phase 1 (kernel/TUI/event-log/replay)`), verify the parenthetical against the phase-ladder name (`docs/2-execution/03_*` and the per-surface doc) and the ledger entry. A misattribution propagates misleading provenance into implementation — HIGH Issue; the file existence check passes silently.
  - **Named-list drift**: for a predecessor/successor spec, scan its named lists (deliverable lists, required-area rows, fixture names, acceptance criteria) for entries the target references. A target naming a deliverable/value the sibling's enumeration omits is a HIGH Issue (cross-spec contract drift). Spec 0001's fixture names, roster baseline, and primitive vocabularies are the common drift surface for a Phase 1 spec built on it.
- **Skill dependencies**: verify `.claude/skills/<name>/` directories referenced in the spec exist (e.g. `brainstorm`, `skill-audit`, this skill).

## 3.5 Skill-Structure Validation

For deliverables that propose SKILL.md changes, applicability is gated on the SHAPE of the change. Content-only edits (rewording, prose updates) need no 3.5 validation — report N/A. Structural edits (frontmatter, HARD-GATE block, Step/Phase definitions, Output declarations) require verifying:

- Frontmatter declares `name`, `description`, `user-invocable`, `arguments`; the description names triggers, produces, and mutates.
- A Prerequisites / required-reads block is present.
- The Final Rule (or equivalent) is a single enforceable sentence.
- If the skill mutates files, a `<HARD-GATE>` block and a Write/Commit step are present.

Match the conventions of the repo's existing skills (`brainstorm`, `skill-audit`, this skill) rather than inventing new structure.

## 3.6 Downstream Consumers

For types, records, functions, schemas, fixtures, or contracts the spec modifies, grep all usage points across the repo — the doc pack (`docs/*`), sibling specs (`specs/*`, `docs/4-specs/*`), `.claude/skills/*`, and the code tree (`crates/`) for any landed surface. Record blast radius. For a surface that is not yet implemented, the consumers are doctrine/spec/fixture references rather than call sites (see the unbuilt-surface note above).

For **new, retired, or changed** string-enum values (a new/retired event kind, claim family, visibility scope, view-model field, or dispatch token), grep each affected value across all consumer sites — new values need a new arm at every dispatch/projection site; retired values need every consumer updated (or retired alongside); changed values need both. Surface the consumer count explicitly. (A spec that ONLY retires values is the common case the literal "new enum" framing would steer past — the broadened scope closes that gap.)

For **schema/contract extensions** (event envelope, entity/component record, view model, golden trace/fixture, snapshot, save package, content-manifest entry), confirm consumers of that schema are updated, or the extension is additive-only (new optional field with a default), per the event-schema-evolution rule (INV-020; `docs/1-architecture/02_*`).

**Audit the spec's own completeness-sweep / gate command**: when a spec ships a self-verification `grep`/`find`/`test` "completeness sweep, re-run as a gate" (common in removal/rename specs), validate the gate's coverage against where consumers actually live. A gate that omits a consumer directory or uses a pattern that misses a syntax variant is an under-scoped gate — the spec's own verification passes green while real consumers drift. HIGH Issue; recommend widening the gate's paths/pattern and adding the missed consumers to the deliverables / files-to-touch list.

## 3.7 Upstream Spec / Phase References

Grep `specs/`, `docs/4-specs/`, and `docs/4-specs/SPEC_LEDGER.md` for references to this spec's deliverables or phase; note affected specs. Use matches to refresh the Relationship-to-prior-spec section and any "phase X has not landed yet" claims with accurate status.

**Forward-compat with successor phases**: for specs that define schemas/contracts/fixtures AND whose sequencing names successor phases, read each successor phase's spec (when written) for extensions to the current spec's surfaces (new conditionally-required fields, new event kinds, new claim families). If a successor phase proposes additions the current spec's design would silently reject (strict shape validation, closed enums, no unknown-field tolerance beyond the reject-unknown rule), flag a forward-compat Improvement at MEDIUM. Skip when there are no successor phases specced yet or they don't extend the current spec's surfaces.

## 3.8 Doctrine-Contract Fidelity

For deliverables that touch foundation-governed semantics — kernel authority, the kernel boundary / dependency direction, no-scripting / content-is-possibility, the universal acceptance invariants, determinism/replay, and the no-leak firewall:

- **No invariant weakening**: read the relevant `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` invariants (and the governing architecture doc). For each invariant the deliverable touches, verify the proposal enforces it at least as strictly as the constitution requires. A proposal that weakens an invariant is a CRITICAL Issue.
- **Kernel-authority preservation**: verify no deliverable moves world authority (state transitions, validation, legality, event application, scoring, RNG, semantic effects, projection, replay/hash, serialization) out of the authoritative kernel, and that TUI/view-models stay presentation-only and the LLM stays out of authority (INV-008, INV-042; `docs/1-architecture/01_*`, `10_*`). Letting a view or the LLM decide world legality or invent world state is CRITICAL.
- **Kernel-boundary / dependency-direction preservation**: verify no deliverable inverts the authority direction or pushes domain mechanics into a layer doctrine forbids (`docs/1-architecture/01_*`). A boundary inversion is a CRITICAL Issue.
- **No-scripting preservation**: verify content/fixtures stay possibility, not script — no authored outcome chains, director logic, or behavior-looking content fields (foundation 09; INV no-scripting; `docs/2-execution/08_*`). An authored script is a CRITICAL Issue.
- **Visibility / no-leak preservation**: for deliverables affecting view-model projection, possession, previews, traces, debug views, notebooks, or replay exports, verify no path lets hidden information reach a viewer the actor-knowledge filter forbids (INV-024 no telepathy, INV-006 possession transfers no knowledge; `docs/1-architecture/06_*`, `10_*`). Missing firewall is CRITICAL.
- **Determinism preservation**: for deliverables touching replay, hash/checksum, serialization order, RNG, snapshots, compaction, or traces, verify identical inputs+versions produce identical output and no nondeterministic input enters canonical forms (INV-017 seedable/auditable randomness, INV-018 deterministic replay, INV-019 snapshots/compaction preserve ancestry; `docs/1-architecture/02_*`). Violations are HIGH; a change to replay/hash/snapshot *semantics* additionally requires a foundational-doc amendment.
- **Validation discipline**: for deliverables proposing validation, verify it stays deterministic, fail-closed, and blocking, distinguishes warnings from blockers, rejects unknown fields by default, blocks behavior-looking content fields, and names what failing means (`docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`; `docs/2-execution/08_*`). Unaddressed second-order effects are Improvement findings at minimum.

## 3.9 New-Deliverable Consumer Verification

For each proposed new deliverable (new module, new CLI tool, new validator, new public type, new contract field, new fixture, new reference-file section), verify at least one identifiable consumer exists or is explicitly planned. Grep for references to it by name across the doc pack, `specs/*`, `docs/4-specs/*`, `.claude/skills/*`, and (once it lands) the code tree, and inspect the spec's own Purpose / Scope / Required-areas for a concrete consumer-side workflow.

**Outcome**:
- **≥1 consumer found**: deliverable justified — record the consumers in Step 6 for audit-trail visibility.
- **Zero consumers AND no pending consumer named**: HIGH Issue → present at Step 6 as a Question with three options: (a) drop per YAGNI; (b) keep with explicit rationale naming a near-term consumer; (c) defer to a separate consumer-driven spec/phase. Defer the decision to the user — do not silently drop at Step 3. (Deliverables whose consumers are sequenced but not-yet-built future phases are **not** zero-consumer cases — see the unbuilt-surface note above the substep sections.)

**Structurally-wired deliverables** (e.g. a fixture registered in a fixture manifest, a module registered in a workspace members list) have a structural consumer model — registration *is* the wiring. Confirm the registration site rather than name-grepping for callers; flagging "zero consumers" there is a false positive.

## 3.10 Source-Document Completeness Check

For specs citing a source document (an execution phase doc, an architecture contract, Spec 0001, a report, a brainstorm output) in their Purpose / Scope / Required-areas, the claims were enumerated and tagged at Step 2. For hardening/audit specs, apply the engagement variant from Step 2 instead: verbatim-verify overturned/re-opened prior-audit claims against the archived predecessor; claim-by-claim adjudication applies only to genuinely directive sources. Here:

1. **Verify** each enumerated claim is adjudicated by the spec — **Accepted** in Scope/Deliverables/Required-areas with a per-claim mapping (this covers accept-with-divergence: the spec adopts the intent but deliberately diverges; record the divergence in the mapping), **Rejected** in Non-goals/Not-allowed with a rationale, or **Deferred** with a named follow-up phase.
2. **Surface unadjudicated claims** as MEDIUM Improvement findings — name the claim, cite the source line, recommend the spec add an adjudication. For an execution phase doc specifically, every acceptance-gate and non-goal line of that phase must be mapped: an unmapped phase acceptance gate is a HIGH Issue (the spec's exit/acceptance section must mirror it), and an unmapped phase non-goal is a HIGH Issue (it must carry into the spec's Non-goals / Forbidden-changes sections).
3. **Surface source-internal inconsistency** — when the source contradicts itself AND the spec follows one part, surface as a MEDIUM Improvement (LOW if the spec already handles it) and recommend the spec record which variant it follows and why, so implementation doesn't re-adopt the rejected variant.

For classification (d), apply a stronger variant: verify each "accepted" claim actually landed in the codebase. An accepted-but-unredeemed claim with no delivering citation is a HIGH Issue (silent completion-claim risk).

**Skip** when: (c) refactor classification, or no source document is cited (self-originating specs are scoped by 3.0–3.9 alone).

## 3.11 Spec Structural Completeness Check

For specs introducing new work (classes (a) and (b)), verify the spec carries the sections an implementer needs. Tracewake has no single canonical spec template; take the spec's own section set as the baseline and compare against the conventions of the sibling exemplars (`archive/specs/0002_*`, `docs/4-specs/0001_*`). The load-bearing axes:

- **Scope + required-areas / deliverables** — each implementation area names a concrete target (file/module path, record/type name, fixture name, tool name) an implementer can grep against, with dependency order. A scope that reads as feature description without named targets is incomplete on this axis.
- **Acceptance / exit criteria + verification** — criteria mapped to the relevant Phase acceptance gates (`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` and the per-surface doc; `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`) and naming re-runnable confirmation (tests, golden traces, deterministic-replay/hash checks, fixtures, no-human runs, why-not checks) per `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`. Use explicit `not applicable` rows over silent omissions.
- **Non-goals + binding invariants/constraints + assumptions** — phase-specific prohibitions (carrying the phase's non-goal list), the binding constitutional invariants and architecture constraints the spec must satisfy, and one-line-correctable assumptions surfaced explicitly rather than discovered at implementation time.

**Severity**: a spec missing its scope/required-areas, acceptance/exit criteria, or verification basis entirely is a HIGH Issue (implementation can't proceed). Missing a Non-goals section when the phase has a non-goal list is a HIGH Issue (the prohibition must carry). Missing an explicit Assumptions surface is a MEDIUM Improvement — downgrade to LOW when the spec surfaces its open questions under another section. A docs-only or process spec missing these is LOW or N/A depending on whether its content depends on implementer follow-up.

**Skip** for (c) refactor and (d) retroactive (the latter's structural completeness lives in its Acceptance-evidence / Outcome content).

## Conditional Deliverable Validation

For specs with conditional deliverables ("If root cause X is confirmed, do Y"), validate: (1) **diagnostic sufficiency** — the investigation can distinguish the hypotheses; (2) **fix correctness** — each proposed fix references correct types/functions/module paths regardless of which is selected; (3) **soundness** — each fix respects the constitutional invariants even though conditional (a conditional violation is still a spec defect).
