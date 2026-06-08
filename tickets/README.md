# Ticket Authoring Contract

This directory contains active implementation tickets.

To keep architecture clean, robust, and extensible, every new ticket must be created from `tickets/_TEMPLATE.md` and must satisfy the checks below.

## Core Architectural Contract

1. No backwards-compatibility shims or alias paths in new work.
2. If current code and ticket assumptions diverge, update the ticket first before implementation.
3. `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (`INV-001`…`INV-110`, including the `## 2026 hardening invariants` truth-firewall set INV-099–110) is the non-negotiable design contract, sitting at the top of the foundation → architecture → execution → reference authority order (`docs/README.md`). Tickets must align with its doctrine: causal event-sourcing and deterministic replay, belief-before-truth epistemics with actor-knowledge filtering, ordinary-life and possession parity, no-scripting authoring, validation and hard-fail gates, and the LLM/language boundary.

## Required Ticket Sections

1. `Assumption Reassessment (YYYY-MM-DD)`:
   - Validate ticket assumptions against current code, skills, and specs.
   - Explicitly call out mismatches and corrected scope.
   - Cite exact files, symbols, skill names, or schema fields for any non-trivial architectural claim.
   - For cross-skill or cross-artifact tickets, name the exact shared boundary, contract, or schema under audit before implementation.
   - Classify newly exposed adjacent contradictions as required consequences of the intended change, separate bugs uncovered during reassessment, or future cleanup that must become its own ticket.
   - For tickets touching invariant-aligned enforcement surfaces (validation/hard-fail gates, actor-knowledge filtering / epistemic-leakage prevention, deterministic replay, no-scripting boundaries), restate the Constitutional Invariant (`INV-NNN`) under audit before trusting the ticket narrative.
2. `Architecture Check`:
   - Explain why the proposed design is cleaner than alternatives.
   - State that no backwards-compatibility aliasing/shims are introduced.
3. `Verification Layers`:
   - Required for any cross-skill or cross-artifact ticket.
   - Map each important invariant to the exact verification surface that proves it.
   - Use one line per invariant. Valid verification surfaces for Tracewake tickets:
     - codebase grep-proof (Rust symbol existence, rename/removal confirmation, schema field presence)
     - schema validation (entity/component, event, and content/domain-pack conformance against `docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md` and `docs/2-execution/09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md`)
     - replay/golden-fixture check (deterministic replay over a golden scenario; no-human run advances)
     - skill dry-run (a `.claude/skills/` workflow invoked with a representative input; deliverable inspected without commit)
     - invariants alignment check (`INV-NNN` principle, rule, or schema cited by section)
     - manual review (prose quality, epistemic-leakage audit, deterministic-replay check)
   - Do not collapse multiple layers into one generic "validation" or "review" surface.
4. `Tests`:
   - List new/modified tests, validators, or dry-run commands and rationale per test.
   - Include targeted and full-pipeline verification commands.
   - Commands must be copy-paste runnable against real skill invocations, real file paths, or real validation commands, not approximate filters.

## Mandatory Pre-Implementation Checks

1. Dependency references point to existing repository files (active or archived paths are both valid when explicit).
2. Type, schema, and data-contract references match current code.
3. Files-to-touch list matches current file layout and ownership.
4. Scope does not duplicate already-delivered architecture.
5. Test/verification commands have been dry-run checked or verified against the current pipeline layout.
6. Claimed helper, skill, or function usage is verified against the exact current symbol location, not inferred from a similarly named artifact elsewhere in the repo.
7. For cross-skill or cross-artifact tickets, confirm the intended invariant, the exact shared boundary under audit, and whether adjacent contradictions belong to this ticket or a follow-up before implementation begins.
8. For information-path refactors (where the same fact is currently transported through multiple paths), confirm whether current code still has multiple lawful transport paths for the same fact, name the canonical end-state path, and verify that the planned proof surface remains strong enough to debug that canonical path after the change.
9. For tickets touching fail-closed validation, actor-knowledge filtering, or deterministic-replay surfaces, verify the change does not weaken epistemic-leakage prevention (belief-before-truth / actor-known filtering) or break deterministic replay.
10. For tickets extending an existing schema (entity/component state, event record, content/domain-pack, view-model projection, save-package entry), verify consumers of that schema have been updated, or the extension is additive-only (new optional field with a default).

## Archival Reminder

Follow `docs/archival-workflow.md` as the canonical archival process.
