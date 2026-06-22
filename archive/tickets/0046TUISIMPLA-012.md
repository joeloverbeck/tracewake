# 0046TUISIMPLA-012: Adversarial closure + acceptance-artifact capstone

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adversarial-closure negatives added to the parity suite (`crates/tracewake-tui/tests/parity_adversarial.rs`) and the spec's acceptance artifact (`reports/0046-parity-acceptance-artifact.md`, per the 0003 template extended by ticket 011). No production-crate change.
**Deps**: 0046TUISIMPLA-009, 0046TUISIMPLA-010, 0046TUISIMPLA-011

## Problem

Spec 0046 §4.4 `PAR-012` and §7. The contract is only credible if the guards provably bite and the
evidence is assembled into one acceptance artifact. This capstone proves removing the exhaustive
destructure or the PAR-002 source guard, dropping a capability witness, leaking debug/hidden truth into
an embodied assertion, or registering an action with no coverage each cause a failure — and includes the
controlled compile-break transcript (a temporary added view-model field and a temporary added closed
presentation variant are rejected until dispositioned). It then assembles the §7 acceptance artifact.

## Assumption Reassessment (2026-06-22)

1. Verified the upstream surfaces land before this ticket: the destructure + source guard (001), the
   closed-enum matches + their guards (002), the registry/runner (003/004), the goldens harness (005),
   the baseline matrix (006/007), the CI lane (008), and the doctrine amendments + extended 0003
   template (009/010/011). This ticket `Deps` the doc leaf set {009, 010, 011}, which transitively
   covers the implementation chain (009→001/002/005; 010→005/006/007/008; 011→004/006/007).
2. Verified against spec 0046 §7: the artifact must carry the four-gate results; named parity/conformance
   output (no new canonical gate code); the controlled compile-break transcript; source-conformance
   output proving no `..`/wildcard escape; the baseline capability matrix with zero uncovered entries;
   evidence every `ActionRegistry::definitions()` entry maps to a passing case; focused goldens + diffs;
   paired actor-known/hidden-state tests; debug-quarantine/no-direct-dispatch/replay/no-human/
   determinism results from existing suites; content/fixture fingerprints + the exact implementation
   commit. The artifact uses the 0003 template extended per `PAR-DOC-006` (ticket 011).
3. Shared boundary under audit: this capstone exercises the whole pipeline end-to-end; it introduces no
   new production logic but adds adversarial negative-test infrastructure (dual-role capstone — note
   per the capstone-shape "deliverable-doubles-as-capstone" exception). The controlled compile-break
   cannot run in CI (a deliberate temporary break), so it is a **manual runbook** captured as a
   transcript in the artifact.
4. Invariant restated (`PAR-012`/§7 motivation): `INV-098` acceptance is harsh (caused, agent-possible,
   eventful, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe,
   non-scripted, regression-tested); `INV-095` TUI/view-model tests are acceptance tests. The
   adversarial closure proves the guards are real, not decorative.
5. Enforcement surface re-proven (evidence-consumer basis): this ticket audits/re-proves the no-leak
   (`INV-093`/`INV-099`–`101`), debug-quarantine (`INV-107`), and determinism (`INV-018`) surfaces.
   Confirm the adversarial negatives and the artifact's debug rows stay observer-only — the leak-into-
   embodied negative asserts a *failure* (the guard fires), and no negative test or artifact row creates
   actor knowledge or surfaces hidden truth as a passing embodied witness.

## Architecture Check

1. A single adversarial-closure + artifact capstone is the right shape: it adds no production logic, it
   exercises the guards the earlier tickets composed, and it consolidates §7 evidence into one
   reviewable artifact on the 0003 template. Routing the compile-break as a manual-runbook transcript
   (rather than a permanently-failing build) keeps the tree green while still proving the tripwire.
2. No backwards-compatibility aliasing/shims: the negatives drive the real guards; no mock or bypass is
   introduced; the artifact cites real command output.

## Verification Layers

1. `PAR-012` (guards bite) → executed negatives in `parity_adversarial.rs`: dropping a capability
   witness fails the runner; an action registered with no coverage fails the runner; a debug/hidden-truth
   value injected into an embodied assertion fails the anti-leak witness; removing the source guard's
   target fails the source-conformance test.
2. `PAR-012` (compile-break) → manual runbook: a temporary added `EmbodiedViewModel` field and a
   temporary added closed presentation variant are each rejected by the compiler until dispositioned;
   the transcript is captured in the artifact.
3. §7 (acceptance evidence) → artifact assembly: four-gate results, named conformance output, baseline
   matrix (zero uncovered), goldens + diffs, actor-known/hidden-state pairs, debug-quarantine/replay/
   no-human/determinism results, fingerprints + exact commit, all on the 0003 template.

## What to Change

### 1. Adversarial-closure negatives (`crates/tracewake-tui/tests/parity_adversarial.rs`)

Add executed negatives proving each guard bites: capability-witness drop → runner failure; uncovered
registered action → runner failure; debug/hidden-truth injection into an embodied assertion → anti-leak
failure; source-guard target removal → source-conformance failure.

### 2. Controlled compile-break transcript (manual runbook, in the artifact)

Document the runbook: temporarily add a 22nd `EmbodiedViewModel` field → show `render_embodied_view`
fails to compile until dispositioned; temporarily add a closed presentation variant → show its
presentation owner fails to compile until matched; revert both; capture the transcript.

### 3. Acceptance artifact (`reports/0046-parity-acceptance-artifact.md`)

Assemble the §7 evidence on the 0003 template (extended by ticket 011): four-gate results, named
parity/conformance output, the compile-break transcript, source-conformance output, the baseline matrix
with zero uncovered entries, the `definitions()`-maps-to-passing-case evidence, focused goldens + diffs,
actor-known/hidden-state pairs, debug-quarantine/no-direct-dispatch/replay/no-human/determinism results
from existing suites, content/fixture fingerprints, and the exact implementation commit.

## Files to Touch

- `crates/tracewake-tui/tests/parity_adversarial.rs` (new)
- `reports/0046-parity-acceptance-artifact.md` (new)

## Out of Scope

- New production logic (capstone exercises the pipeline, does not extend it).
- Any cargo-mutants campaign (`PAR-012` is adversarial-closure via compile-break + guard-removal, not
  mutation operators; `.cargo/mutants.toml` is untouched).
- The `docs/4-specs/SPEC_LEDGER.md` archived row + the `archive/specs/` move — deferred to spec
  acceptance/archival (Step 6 cross-spec follow-up), not this ticket.

## Acceptance Criteria

### Tests That Must Pass

1. Each adversarial negative in `parity_adversarial.rs` fails the corresponding guard (witness drop,
   uncovered action, leak injection, source-guard removal) and passes when reverted.
2. The compile-break runbook transcript shows a temporary field and a temporary closed variant rejected
   until dispositioned; the baseline matrix shows zero uncovered entries; every `definitions()` entry
   maps to a passing case.
3. The acceptance artifact is complete on the 0003 template (no reduction to screenshots/display
   strings); `cargo test --workspace` + the four gates pass.

### Invariants

1. Every parity guard provably fires on its violation; the contract is enforced, not decorative.
2. No adversarial negative or artifact row creates actor knowledge or surfaces hidden truth as a passing
   embodied witness (observer-only).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/parity_adversarial.rs` — executed guard-removal/leak-injection/uncovered-
   action/witness-drop negatives.

### Commands

1. `cargo test -p tracewake-tui --test parity_adversarial`
2. `cargo test -p tracewake-tui --test playable_capability_parity` (baseline matrix zero-uncovered)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

Completed: 2026-06-22

## Outcome

Added the adversarial closure target and assembled the spec 0046 acceptance artifact:

1. `crates/tracewake-tui/tests/parity_adversarial.rs` executes four negative checks: dropped
   actor-knowledge witness, uncovered registered action, debug/hidden truth injected into embodied
   anti-leak assertion, and source-guard target/rest-pattern removal. Each violation fails the intended
   guard.
2. `reports/0046-parity-acceptance-artifact.md` records the 0003-template parity evidence block,
   baseline capability matrix summary (`capability_count=21`, zero uncovered), action-definition
   coverage, checked-in golden coverage, anti-leak/debug-quarantine evidence, replay/no-human/
   determinism evidence, fixture fingerprints, CI/workspace lane evidence, and exact command verdicts.
3. Controlled compile-break transcript was captured in the artifact: temporary `EmbodiedViewModel`
   field produced constructor failure then `E0027` at the exhaustive render destructure after temporary
   constructor disposition; temporary `VisibleItemSource` variant produced `E0004` in the upstream match
   and then `E0004` in the TUI renderer after temporary upstream disposition. All temporary edits were
   reverted and `git diff -- crates/tracewake-core/src/view_models.rs crates/tracewake-core/src/projections.rs`
   produced no output.

Verification:

1. `cargo test -p tracewake-tui --test parity_adversarial` passed with 4 tests, 0 ignored.
2. `cargo test -p tracewake-tui --test playable_capability_parity` passed with 8 tests, 0 ignored.
3. `cargo fmt --all --check` passed.
4. `cargo clippy --workspace --all-targets -- -D warnings` passed.
5. `cargo build --workspace --all-targets --locked` passed.
6. `cargo test --workspace` passed and included both parity targets.
7. `grep -nE "parity_adversarial|playable_capability_parity|E0027|E0004|capability_count=21|registered action|screenshots|display strings|DEBUG NON-DIEGETIC|food_hidden_pantry|twf1-|cargo fmt|cargo clippy|cargo build|cargo test --workspace" reports/0046-parity-acceptance-artifact.md`
   showed the required artifact evidence.
8. `git diff --check` passed.
