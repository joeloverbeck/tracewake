# Writing the Updated Spec (Step 7)

After all findings are resolved and approved.

## Pre-Apply Verification

Run targeted checks to confirm each finding still holds, and **emit the verification table in chat before any Write/Edit call** — a vague "I checked the findings" is not sufficient and is treated as no verification. For each finding (by its Step 6 key — `I1`, `M1`, `A1`…), record both the command and the result.

Example:

| Finding | Check | Result |
|---------|-------|--------|
| I1 | `grep -nE "INV-0?18" docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` | INV-018 "Deterministic replay is foundational" present — determinism citation valid |
| I2 | `test -f docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | file exists — event-log contract path valid |
| M3 | Judgment — actor-knowledge no-leak reasoning; Q2 delegated | selected (a): the debug view is non-diegetic and must not feed the actor-filtered view model, per INV-024 / INV-031 |

**Row shapes**:
- **Command-backed** (default): `Finding | <grep / test / file-read command> | <result>`. Use whenever a symbol/invariant can be grepped, a path `test`-ed, or a line read.
- **Judgment-only**: `Finding | Judgment — <restated rationale> | <result>`. For purely analytical recommendations, or when the user delegates ("you decide") — append `; Q<N> delegated`. Use sparingly; a bare `Judgment` without rationale is a skipped check.
- **User-answered**: `Finding | User answer Q<N> = (<option>): <one-line paraphrase> | Apply as: <edit description>`. Expand terse replies ("go with (a)") into the canonical form. When the answer confirms existing text and no edit follows, the Result reads `no edit — confirms existing §<section>`.

**Multi-section pre-edit grep**: when a finding's Result names multiple sections to edit, run an exact-string grep for the changed terminology across the entire spec before the first Edit AND before drafting the Result, and record the actual count + line numbers (e.g. `3 instances at lines L1, L2, L3 — apply at all`). Do not estimate ahead of the grep — the grep is authoritative. Cross-section restatements drift silently because the deliverable's number is unchanged.

**Syntax-variant + `replace_all` discipline**: when the changed terminology may appear with different surrounding syntax (parens vs none, capitalization, list markers, trailing punctuation), grep for the BARE token plus `-i` — not the surrounding context. `replace_all: true` matches only the exact `old_string`; it cannot catch sibling sites with different surrounding syntax, so the pre-edit grep is still required to enumerate the variants needing separate Edit calls.

**Mismatch classification** — if a check reveals a finding/codebase mismatch:
- **Recommendation-changing**: the check invalidates the finding's recommendation (the fix no longer applies, the target moved, a different fix is warranted). Re-present the corrected finding and wait for confirmation before applying that finding's edit. A pure retraction (no substitute) needs a transparent `retracted: <reason>` note but not fresh re-approval.
- **Evidence-refining**: the check refines supporting evidence but the recommendation holds. Note the refinement inline in the Result column and proceed.
- **Scope-extending**: the recommendation still applies but fulfilling it requires a new deliverable or change not discussed at question time. Note it inline in the Result column, proceed, and surface it in the Step 8 summary under a dedicated line. (If a Step 6 option already named the consequence, cite the question — it's a confirmation, not fresh approval.)

When in doubt, treat the mismatch as recommendation-changing and re-present — cheaper to ask than to apply the wrong fix.

## Apply Changes

- Incorporate corrections from the user's question responses. Preserve existing structure and voice; change only what was agreed upon. Keep the spec's own section set (take it as authoritative; mirror sibling exemplars `archive/specs/0002_*`, `docs/4-specs/0001_*` for convention) — prefer explicit `not applicable` rows over silent omissions.
- Prefer `Edit` for ≤3 localized changes; prefer full `Write` when insertions cause **cascading renumbering** or the change is a **diffuse rewrite** of contiguous prose. The decision keys on the *shape* of the change, not the count of sections touched — many independent surgical edits across many sections are well-served by targeted Edits.
- **Inserting deliverables / required-area items**: renumber all subsequent items and update intra-spec cross-references (dependency-order references, acceptance-criteria rows that name an item, sequencing). **Removing items**: grep the spec for all references to the removed number/ID (Scope, Deliverables, required areas, acceptance/exit criteria, binding invariants, forbidden changes, assumptions, cross-references) and update or remove them. Exclude citations to OTHER phases'/specs' deliverables (e.g. `Phase 2 §epistemics`) from renumbering — those are external and preserved verbatim.
- **Material mechanism modification (number unchanged)**: grep the spec for the deliverable's old key concepts (function/type/module/fixture names the modification eliminates) and scan Purpose, Scope, Deliverables, binding-invariants alignment, acceptance criteria, assumptions for restatements.
- **Material mechanism redirect** (one approach rejected for another): consider a brief "Why X and not Y" rationale in the affected §Scope or §Deliverables, recording the boundary/contract/invariant reason the rejected approach was insufficient. This authors the spec-level audit trail so future readers don't re-propose it; the §Post-Apply Confirmation audit-trail retention exception then recognizes the rejected token's appearance there as acceptable retention.
- **Assumptions / open-question resolution**: if a finding resolves an entry in the spec's §Assumptions (or a deferred open question), update or remove that entry alongside the primary edit. A "still open" assumption the reassessment actually closed is a misleading audit trail.
- **New deliverable vs. amendment**: when a finding introduces substantial new logic (new mechanism, new type, new module, new contract surface), consider a new numbered deliverable / required-area item rather than expanding an existing one — criteria: distinct implementation site, independently testable, would make the existing item unwieldy if inlined.
- **Late-discovered findings**: if writing reveals minor factual errors not in the plan (wrong symbol names, typos, outdated doc-tier paths), fix them and note in Step 8 as "Also fixed:". If a late finding would be HIGH/CRITICAL, re-present before applying. If discovered during edit *planning*, key it `LD-N` in the pre-apply table; if during application or post-apply, Step 8 "Also fixed:" alone suffices.

## Retroactive Branch (classification (d))

If Step 3 concluded all deliverables already landed, Step 7's output is NOT deliverable refinement. Instead:

1. Flip the spec's **Status** to `Done` (or the repo's done marker; reflect the same in the `docs/4-specs/SPEC_LEDGER.md` entry).
2. Populate the **Acceptance evidence** section (and/or an **Outcome** note) with: completion date (absolute); landed changes (cite file/module paths + line numbers); delivering commit(s) or sibling spec(s); deviations from the original plan; verification / acceptance-gate commands **re-run at reassessment time** with pass/fail status (do not copy from memory — rerun each to catch post-delivery regressions, e.g. test runs, deterministic-replay/hash checks, fixture/golden-trace checks per `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`).
3. Mark the historical **Purpose / motivating context** as such — a short parenthetical noting the gap it describes was closed by the landed implementation, so a future reader doesn't treat a stale condition as live.
4. Cross-reference any later phases/specs/skills that extended or absorbed the original scope.
5. Do NOT apply structural refinements to deliverables that already shipped — the spec is now a historical record; editing deliverable sections to match current code would confuse the causal narrative.
6. **Ledger update**: update the spec's `docs/4-specs/SPEC_LEDGER.md` entry (Status, deliverables-produced, what-it-settles) only if the spec's own documentation-updates content calls for it; otherwise remind the user in Step 8. Tracewake's canonical archival policy is `docs/archival-workflow.md`: a completed spec is marked at top (`**Status**: COMPLETED` or the appropriate final marker), gains an `## Outcome` section, and is moved (`git mv`) to `archive/specs/` — it does not stay in place. Performing the move is outside this skill's deliverable (the updated spec file); remind the user in Step 8 to run the archival workflow once the spec's completion is confirmed.

## Post-Apply Confirmation

Grep the updated spec for:

1. **Eliminated stale references** — should return zero matches. For phrase-elimination, use literal-string `grep` without `-E`; when regex is necessary, prefer `[^.;]*` over `.*` to avoid greedy cross-sentence false positives.
2. **Corrected references** — should return the expected matches.
3. **File/doc/module path references in newly added deliverables** — should resolve to existing paths (or be clearly marked as proposed targets created by a named required-area item).
4. **Re-runnable commands** — if a finding added or edited an acceptance/exit-criterion, verification, or embedded completeness-sweep/gate command, confirm the command text resolves; for an edited gate-command, re-run it to confirm sane output.

**Audit-trail retention exception**: when an eliminated reference appears in a deliberate "this was removed and why" rejection paragraph, check 1's grep shows N≥1 for the term. Matches inside an explanatory rejection paragraph are acceptable retentions; matches in an active deliverable, instruction, or normative statement indicate incomplete elimination. Cite the retention site explicitly in Step 8 so a future reader can distinguish acceptable retention from unfinished elimination.

For classification (d), additionally: grep every concrete artifact named in the spec's Purpose / motivating context (symbols, paths, type names) and prove its landed/corrected form in the current codebase.

Record results for Step 8.
