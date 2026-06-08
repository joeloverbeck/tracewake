# Tracewake specs

This directory contains the active spec layer. Specs are the lowest doctrine tier: they may operationalize foundation, architecture, execution, and reference doctrine, but they may not amend, replace, weaken, or redefine it.

## Status

The spec layer has been realigned after the foundation / architecture / execution / reference overhaul. It is no longer a pre-overhaul phase-planning layer.

The live set is intentionally compact:

| File | Role |
|---|---|
| `README.md` | This index and usage rule. |
| `SPEC_LEDGER.md` | Spec ledger, source discipline, archival posture, and next known execution move. |
| `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` | Live first-proof missing-property village ontology and fixture contract. |
| `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | Scoped review-artifact template for the archived Phase 1 / Phase 1A spine hardening remediation. |

The former companion notes were removed from the live tier instead of kept for symmetry. Their useful conclusion has been superseded by the completed upper-tier overhaul, and stale research provenance is not product doctrine.

## Reading order

1. Read `../README.md` for repository doctrine layering.
2. Read the relevant higher-tier documents in `../0-foundation/`, `../1-architecture/`, `../2-execution/`, and `../3-reference/`.
3. Read `SPEC_LEDGER.md`.
4. Read the active spec file.

Do not cherry-pick a spec without its higher-tier authority chain.

## Rules for future specs

Future specs must:

- declare their authority posture beneath foundation, architecture, execution, and reference;
- declare one execution admissibility posture: `P0-CERT passed`, `P0-CERT scoped remediation`, or `P0-CERT not applicable`;
- name execution gate codes only as cross-references, never as local definitions;
- use `holder-known context` as the system-wide term and `actor-known` for the actor case;
- keep archived specs as history, not current certification;
- preserve the source-discipline rule that manifests are path inventory only and branch/default-branch/code-search evidence is not proof of exact-commit content;
- avoid new files merely for symmetry.

`P0-CERT` is named by the execution tier as the next major implementation audit. This directory is ready for that future work, but it does not contain that certification spec.
