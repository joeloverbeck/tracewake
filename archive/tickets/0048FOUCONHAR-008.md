# 0048FOUCONHAR-008: Capstone — focused mutation campaigns, evidence report, and conformance/risk-doc updates

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — acceptance/evidence capstone: runs focused mutation campaigns over the rewritten surfaces and records survivors, authors the implementation/review artifact, and updates conformance rows and risk-register evidence. Adds no production or test-harness code.
**Deps**: 007

## Problem

Spec 0048 §6.3 + §7: after remediation, focused `cargo-mutants` campaigns must run over the rewritten surfaces, a deterministic evidence/review artifact must be produced, and the live conformance/evidence homes must be updated (not doctrine). This capstone is the verdict/consolidation ticket for §8 closure step 7 ("execute and package"): it exercises every prior ticket's surface end-to-end via the workspace gates and replay/golden lanes, runs the configured mutation campaigns, records the evidence, and re-points the conformance and risk rows at the new executable witnesses. It introduces no new production logic — it certifies the pipeline tickets 001–007 composed.

## Assumption Reassessment (2026-06-23)

1. `.cargo/mutants.toml` already includes every §3.2-named surface — `crates/tracewake-core/src/scheduler.rs` (line 14), `need_accounting.rs` (13), `projections.rs` (16), `actions/pipeline.rs` (17), `replay/**` (28), `view_models.rs` (33), `crates/tracewake-tui/src/app.rs` (45), `render.rs` (47) — confirmed by reading the config. So **no perimeter expansion** is required (§3.2 / §5.1 — verified); the delta is behavior that kills omission/zeroing/wrong-order/skipped-phase mutants, landed by tickets 001–007. The conformance home is `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`; the risk register is `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (R-00…R-29 all present — confirmed); the acceptance-artifact convention is `reports/<NNNN>_<slug>_acceptance.md` (e.g. `archive/reports/0047_tui_authoritative_world_advance_acceptance.md`).
2. Spec 0048 §6.3/§7 and §8 (closure step 7) own this. §7 directs: run the workspace gates and replay/golden lanes and retain exact transcripts; run the focused mutation campaigns; produce a new implementation/review artifact per execution `10` (no doctrine change to execution `10`); update conformance rows (arch `00`), risk evidence/status for R-08, R-09, R-10, R-11, R-13, R-15, R-16, R-27, R-28, R-29 (ref `01`, **mint no risk id**); scope the old differential as duration/accounting evidence until replaced (done by ticket 006); distinguish registry declaration metadata from measured typed evidence (done by ticket 007).
3. Cross-artifact boundary under audit: the evidence/conformance surface spanning the new acceptance artifact (`reports/0048_*_acceptance.md`), the architecture conformance index, and the risk register. This ticket reads the witnesses tickets 001–007 produced and records/links them; it does not modify those witnesses or any production code. The `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` move are **deferred to spec acceptance/archival** (staged hardening-spec convention), not ticketed here.
4. Constitutional invariants the capstone re-proves (motivating the evidence package): `INV-018`/`INV-092` (deterministic replay incl. temporal-frontier reconstruction — ticket 001), `INV-067`/`INV-099`/`INV-101`/`INV-102` (holder-known interval firewall — tickets 002/005), `INV-005`/`INV-091`/`INV-094`/`INV-103`/`INV-108` (loaded-world step + no-human/parity — tickets 003/004/006/007), `INV-112` (singular temporal authority — ticket 004). The artifact maps each gate to its executable witness.
5. Enforcement surface (evidence-consumer basis): the capstone audits the replay, actor-knowledge, and accounting enforcement surfaces by running their tests and the mutation campaigns; it must keep the artifact's debug/evidence rows observer-only and introduce no leakage/nondeterminism path. Per execution `10`, evidence is typed-before-rendered and anti-vacuous: the artifact distinguishes "duration/accounting differential", "loaded-world actor/process differential", "temporal replay", and "holder-known noninterference", and cites measured outputs, not declaration metadata.

## Architecture Check

1. A single acceptance-only capstone keeps the verdict and the evidence package in one place gated on every prior ticket passing, rather than scattering closeout evidence across the implementation tickets. Running the focused mutation campaigns here (after the behavior witnesses land) tests witness *strength* — surviving mutants signal a missing witness or an equivalent mutant, classified under the repo's existing mutation evidence process — without expanding the standing perimeter. Re-pointing the conformance/risk rows at the new witnesses closes R-27/R-29 with live evidence instead of leaving stale rows.
2. No backwards-compatibility aliasing/shims: docs and the evidence artifact are updated/created; no code path is added or wrapped.

## Verification Layers

1. `INV-018`/`INV-067`/`INV-091`/`INV-094`/`INV-112` end-to-end -> replay/golden-fixture check: the full workspace gates (`fmt`/`clippy`/`build`/`test`) and replay/golden lanes pass over the rewritten authority path, with exact transcripts retained in the artifact.
2. Witness strength -> mutation campaign: focused `cargo-mutants` over the canonical step phase invocation/ordering, prepare/commit atomicity, temporal chain arithmetic/validation, private-frontier restoration path, holder-known source membership/fact-kind compatibility, salience selection, one-charge reconciliation, and the reservation census; survivors triaged under the existing mutation evidence process (no text-only mutation expectations).
3. Conformance/risk currency -> codebase grep-proof + invariants alignment check: arch `00` conformance rows cite the new temporal/frontier, actor/process differential, and sealed-provenance witnesses; the listed R-NN rows carry updated evidence/status; no new risk id is minted.
4. Single-layer note N/A — three distinct concerns map to three distinct proof surfaces above.

## What to Change

### 1. Run gates + focused mutation campaigns; record evidence

Run the workspace gates and replay/golden lanes; run focused `cargo-mutants` over the rewritten surfaces (Verification Layer 2; perimeter unchanged). Author `reports/0048_foundational_conformance_hardening_acceptance.md` recording the static-survey-vs-executed-remediation evidence, the gate/replay/mutation transcripts or deterministic reports, and the gate→witness map — typed-before-rendered, distinguishing the four evidence classes per execution `10`.

### 2. Update conformance and risk rows (no doctrine change)

In `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, re-point the time-control conformance rows at the new executable temporal/frontier, actor/process differential, and sealed-interval witnesses (keeping the correct 0017 accounting/open-duration statement). In `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, update the evidence/status of R-08, R-09, R-10, R-11, R-13, R-15, R-16, R-27, R-28, R-29; mint no risk id.

## Files to Touch

- `reports/0048_foundational_conformance_hardening_acceptance.md` (new — implementation/review evidence artifact)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify — conformance rows → new witnesses)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify — R-08…R-29 evidence/status; no new id)

## Out of Scope

- Any production or test code change — tickets 001–007 own the witnesses; this capstone exercises and records them.
- Expanding `.cargo/mutants.toml` (§3.2/§5.1 — perimeter already covers the surfaces) or adding `proptest`/`quickcheck` (§6.4).
- Editing execution `10` or any other doctrine (§5/§7 — no doctrine change; the artifact obeys execution `10`, it does not amend it).
- The `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` move — deferred to spec acceptance/archival (staged hardening-spec convention; see the final summary's cross-spec follow-ups).
- Editing archived 0046/0047 artifacts (§0 — immutable historical records).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, and `cargo test --workspace` all pass; the artifact retains their transcripts.
2. Focused `cargo-mutants` campaigns over the rewritten surfaces complete; every surviving mutant is triaged (killed by an added witness, or classified equivalent) and recorded in the artifact.
3. `grep` proves the arch-`00` conformance rows cite the new witnesses and the listed R-NN rows carry updated evidence; no new `R-NN` id appears.

### Invariants

1. The evidence artifact is typed-before-rendered and anti-vacuous — it distinguishes the four evidence classes and cites measured outputs (execution `10`; `INV` gate set in Assumption Reassessment 4).
2. No doctrine, no risk id, no `.cargo/mutants.toml` perimeter, and no archived artifact is changed by this ticket.

## Test Plan

### New/Modified Tests

1. `None — acceptance/evidence capstone; verification is the workspace gates + replay/golden lanes + focused mutation campaigns run here, plus grep-proofs on the conformance/risk rows. The behavior witnesses live in tickets 001–007.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
2. `cargo mutants -f crates/tracewake-core/src/scheduler.rs -f crates/tracewake-core/src/replay/temporal.rs -f crates/tracewake-core/src/projections.rs` (focused campaign over the rewritten surfaces; extend `-f` to the full rewritten set per Verification Layer 2 — the project's configured mutation tool, not a standard `fmt`/`clippy`/`test` gate).
3. `grep -n "temporal\|frontier\|differential" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` and `grep -nE "R-(08|09|10|11|13|15|16|27|28|29)" docs/3-reference/01_DESIGN_RISK_REGISTER.md` confirm the conformance/risk updates landed.

## Outcome

Completed: 2026-06-23

Accepted as an evidence-only capstone. Added
`reports/0048_foundational_conformance_hardening_acceptance.md`, updated the
0048 conformance row in
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, and updated
the existing R-08, R-09, R-10, R-11, R-13, R-15, R-16, R-27, R-28, and R-29
evidence lines in `docs/3-reference/01_DESIGN_RISK_REGISTER.md`. No doctrine
file, archived 0046/0047 artifact, production/test code, new risk id, or
`.cargo/mutants.toml` perimeter changed.

Full workspace gates passed before the report/doc closeout edits:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`

Focused mutation evidence used `cargo-mutants 27.1.0`. The deliberate
`--no-config` denominator check selected 61 mutants across
`crates/tracewake-core/src/scheduler.rs`,
`crates/tracewake-core/src/replay/temporal.rs`, and
`crates/tracewake-core/src/projections.rs` for
`transact_world_one_tick|advance_until|build_time_advanced_event|project_temporal_frontier|validate_time_advanced|ActorKnownIntervalDelta|VerifiedActorKnownIntervalNotice|proposal_from_current_view_semantic_action|build_embodied_view_model`.
The executed campaign completed with 40 caught, 8 missed, and 13 unviable
mutants under `target/mutants-0048-core`; all eight surviving focused mutants
are triaged in the report. The standing checked-in mutation perimeter was not
expanded and the full configured perimeter was not run for this capstone.

Post-edit closeout checks:

- `rg -n "0048_foundational_conformance_hardening_acceptance|0048FOUCONHAR|temporal|frontier|differential" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `rg -n "R-(08|09|10|11|13|15|16|27|28|29)|0048_foundational_conformance_hardening_acceptance" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `rg -n "^### R-[0-9][0-9]" docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `git diff --check`
