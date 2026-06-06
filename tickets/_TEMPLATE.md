# <PREFIX-NNN>: <Ticket title>

**Status**: PENDING
**Priority**: <LOW|MEDIUM|HIGH>
**Effort**: <Small|Medium|Large>
**Engine Changes**: <None|Yes — list affected or newly-introduced crates, events, schemas, fixtures, skills, or docs>
**Deps**: <ticket/spec dependencies that currently exist>

## Problem

<What user-facing or architecture problem this solves>

## Assumption Reassessment (<YYYY-MM-DD>)

<!-- Items 1-3 always required. Items 4+ are a menu; include only those matching this ticket's scope and renumber surviving items sequentially starting from 4. Lists like 1, 2, 3, 14 are malformed output. -->

1. <Assumption checked against current codebase/skills, with exact file/symbol references>
2. <Assumption checked against current specs/docs, with exact file reference>
3. <If this is a cross-skill or cross-artifact ticket: name the exact shared boundary, contract, or schema under audit before implementation>
4. <If a Constitutional Invariant (INV-NNN) or Validation Rule motivates this ticket: restate the intended invariant/rule before trusting the ticket narrative>
5. <If this ticket touches fail-closed validation, actor-knowledge filtering, or deterministic-replay surfaces: name the exact enforcement surface and confirm the change does not weaken epistemic-leakage prevention or break deterministic replay>
6. <If this ticket extends an existing schema (entity/component state, event record, content/domain-pack, view-model projection, save-package entry): name the schema, the consumers of that schema, and whether the extension is additive-only or breaking>
7. <If this ticket renames or removes a skill, validation rule, doc-governed contract, or schema field: grep repo-wide (.claude/skills/, docs/, specs/, and any code tree) and cite the blast radius per area>
8. <If reassessment exposes adjacent contradictions: classify them as required consequences of this ticket, separate bugs, or future cleanup that must become its own ticket>
9. <Mismatch + correction (if any)>

## Architecture Check

1. <Why this approach is cleaner/more robust than alternatives>
2. <No backwards-compatibility aliasing/shims introduced>

## Verification Layers

1. <Invariant> -> <codebase grep-proof | schema validation | replay/golden-fixture check | skill dry-run | invariants alignment check | manual review>
2. <Invariant> -> <verification surface>
3. <If cross-skill or cross-artifact: map each distinct invariant to its own distinct proof surface; do not collapse>
4. <If single-layer ticket, state why additional layer mapping is not applicable>

## What to Change

### 1. <Change area>

<Details>

### 2. <Change area>

<Details>

## Files to Touch

- `<path>` (<new|modify>)

## Out of Scope

- <explicit non-goals>

## Acceptance Criteria

### Tests That Must Pass

1. <specific behavior test or validation command>
2. <specific behavior test or validation command>
3. <full-pipeline verification command>

### Invariants

1. <must-always-hold architectural invariant>
2. <must-always-hold data-contract invariant>

## Test Plan

### New/Modified Tests

1. `<path/to/test-or-validator>` — <short rationale>
2. `<path/to/test-or-validator>` — <short rationale>
3. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.` <use this instead when no tests change>

### Commands

1. `<targeted verification command>`
2. `<full-pipeline verification command>`
3. `<explain why a narrower command is the correct verification boundary, if applicable>`
