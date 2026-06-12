# 0022PHA3ABASTRI-001: Real mutation-baseline triage and ledger governance hardening

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — test-oracle guards (`anti_regression_guards.rs`), mutation baseline + disposition ledger, focused mutation-killing tests in guarded-layer crates, `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
**Deps**: `specs/0022_PHASE_3A_BASELINE_TRIAGE_INTEGRITY_EMBODIED_DEBUG_QUARANTINE_ADVERSARIAL_GATE_RESTORATION_AND_LOCK_BEHAVIORAL_PROOF_HARDENING_SPEC.md`

## Problem

The mutation-baseline disposition triage required by 0020 and again by 0021 (`ORD-HARD-073`)
has never been performed: commit `549e332` reworded the ledger header of
`reports/0020_mutants_baseline_disposition.md` (5 insertions) while changing zero of the
143 entries, retiring zero baseline misses, and filing none of the promised test-debt
tickets. All 143 entries are `justified-baseline`; 137 share the deferral template
"warrants a future focused assertion before removing from the accepted baseline"
(noun-phrase variants keep identical groups at 17, under
`MUTATION_LEDGER_MAX_IDENTICAL_RATIONALES = 20`). The governance guard additionally
hard-pins `ticket_id.starts_with("0021PHA3APOSREB-")`, structurally rejecting any
future-series `warrants-test` entry. This is spec 0022's only high finding
(`ORD-HARD-099`): the third deferral would make the disposition ledger a permanent
fiction.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `reports/0020_mutants_baseline_disposition.md` holds 143
   `justified-baseline` entries, zero `warrants-test` entries, 137 sharing the deferral
   template; `.cargo/mutants-baseline-misses.txt` is untouched across the entire 0021
   range and the post-audit `-014` ticket (which killed five *new in-diff* mutants —
   `archive/tickets/0021PHA3APOSREB-014.md` — without touching the baseline or ledger).
   The prefix pin sits in `mutation_baseline_governance_errors`
   (`crates/tracewake-core/tests/anti_regression_guards.rs`), alongside the synthetic
   `warrants-test:0021PHA3APOSREB-999:` negative.
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-099` (operator-verified
   evidence; required correction and structural lock as restated here) and §9 (the
   honest-deferral alternative is an explicit recorded owner deferral in the ledger
   header — never a re-costumed bulk accept).
3. Shared contract under audit: the mutation-baseline governance surface —
   `.cargo/mutants-baseline-misses.txt` ↔ `reports/0020_mutants_baseline_disposition.md`
   parity, pinned normalized count (143) and FNV-1a64 hash (`bd1855a5ee82b428`), and the
   `mutation_baseline_governance_errors` rules — consumed by both CI mutation jobs'
   ratchets.
4. Constitutional motivation restated: lock durability under the constitution's
   Enforcement reading and R-28/R-29 (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`) —
   a correction whose required behavior is "perform work once" needs its acceptance
   evidence to be the work product itself, not the governance machinery around it.
5. This ticket touches the deterministic mutation-evidence surface (pinned count/hash)
   and fail-closed governance guards: retiring entries and re-pinning count/hash must
   keep the guard fail-closed (recomputed pins, bidirectional parity preserved); no
   epistemic or replay surface is touched — the changes are test-oracle and evidence
   artifacts plus focused tests that assert existing behavior.
6. Adjacent contradiction classification: the 0021 acceptance report's "Baseline Triage"
   section describing guard capabilities in place of the absent work is a *separate*
   evidence-honesty correction owned by `0022PHA3ABASTRI-002` (ORD-HARD-102/119) — this
   ticket performs the work; -002 corrects the record.
7. Change rationale (no silent retcon): per-entry rationales and tags change because the
   prior content was boilerplate that the spec's evidence proves never encoded a real
   per-entry decision; the change is mandated by `ORD-HARD-099`'s required correction.

## Architecture Check

1. Performing the triage once, with guard rules that make deferral-in-costume
   impossible afterward (deferral-phrase rejection + suffix-repetition bounds + open
   ticket-existence), is cleaner than tightening rationale-text heuristics alone: the
   closed disposition vocabulary plus real ticket IDs makes every future entry either
   genuinely justified or visibly owed to a named ticket. The alternative — another
   header-level governance layer — is exactly the R-29 decorative shape this spec exists
   to kill.
2. No backwards-compatibility aliasing/shims: the prefix pin is replaced (not
   supplemented) by `ticket_exists` over `tickets/` + `archive/tickets/`; old templated
   rationales are rewritten, not grandfathered.

## Verification Layers

1. Lock durability (R-28/R-29) -> codebase grep-proof: zero `justified-baseline` entries
   containing deferral phrasing (`warrants a`, `future`, `follow-up`) after the triage;
   guard rule + synthetic negative firing.
2. Baseline↔ledger integrity -> schema/contract check: bidirectional parity and
   recomputed count/hash pins green in `mutation_baseline_misses_are_pinned_and_ledgered`.
3. Entry retirement honesty -> replay/test check: each retired baseline entry has a
   named focused test that kills its mutant (locally verified via
   `cargo mutants --in-diff` or a targeted `cargo mutants -f` run).
4. Risk-register doc correction -> manual review: R-29 extended with the self-echo-lock
   and artifact-shaped-synthetic shapes; R-28 watch note added (work-product-as-evidence
   rule), per spec §6.

## What to Change

### 1. Perform the real per-entry triage

For each of the 143 ledger entries, assign a genuine disposition under the closed tag
enum: entries in interruption predicates and decision logic of guarded-layer files
(e.g. the `decide_proposal` comparison mutants and `controller_binding_check -> None`
named in the spec) become `warrants-test:<ticket-id>` with the test-debt tickets
actually filed under the reserved range `0022PHA3ABASTRI-015` onward (count
implementation-discovered), or are closed immediately by writing the focused test in
this ticket and retiring the baseline entry. Remaining `justified-baseline` entries get
individually-true rationales stating why *that mutant* is acceptable — never that a
future assertion is warranted. If, after honest triage, the owner decides a class is
not worth testing now, the honest alternative is an explicit recorded owner deferral in
the ledger header (spec §9), not boilerplate.

### 2. Retire entries killed by new focused tests

Write focused tests for the entries triaged as immediately-closable, remove their lines
from `.cargo/mutants-baseline-misses.txt`, and re-pin
`MUTANTS_BASELINE_NORMALIZED_COUNT` / `MUTANTS_BASELINE_NORMALIZED_FNV1A64` to the
recomputed values.

### 3. Harden `mutation_baseline_governance_errors`

(a) Reject any `justified-baseline` rationale containing deferral phrasing
(`warrants a`, `future`, `follow-up`) — deferral intent must carry a
`warrants-test:<ticket-id>` tag. (b) Bound repetition on the rationale *suffix* after
stripping the leading noun phrase. (c) Replace the
`ticket_id.starts_with("0021PHA3APOSREB-")` pin with `ticket_exists` resolution over
`tickets/` and `archive/tickets/`. (d) Synthetic negatives for each new rule, routed
through the same scan code path as the real checks.

### 4. Risk-register corrections (spec §6, group-1 half)

Extend R-29 in `docs/3-reference/01_DESIGN_RISK_REGISTER.md` with the two new shapes
(self-echo lock — `ORD-HARD-106`; artifact-shaped synthetic — `ORD-HARD-113`) and the
meta-lock guardrails as standing countermeasure; add the R-28 watch note (a
perform-work-once correction needs the work product as its acceptance evidence). The
conformance-index rows are owned by `0022PHA3ABASTRI-013`, not this ticket.

## Files to Touch

- `reports/0020_mutants_baseline_disposition.md` (modify)
- `.cargo/mutants-baseline-misses.txt` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)
- `tickets/0022PHA3ABASTRI-015.md` onward (new — test-debt tickets, count
  implementation-discovered)
- Guarded-layer source files gaining focused tests for retired entries (modify — an
  implementation-discovered set within `crates/tracewake-core/src/`, surfaced by the
  triage; parent directory verified)

## Out of Scope

- The 0021 acceptance-report corrections and the §7-checklist parity guard
  (`0022PHA3ABASTRI-002`).
- The two-sided baseline ratchet (auto-tightening) — `0022PHA3ABASTRI-004` (§5.1
  meta-locks); this ticket's guard changes must not conflict with it.
- CI workflow / `mutants.toml` changes (`0022PHA3ABASTRI-003`).
- Conformance-index rows (`0022PHA3ABASTRI-013`).
- Any weakening of the existing count/hash/parity pins.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test anti_regression_guards` — including the new
   deferral-phrase, suffix-repetition, and ticket-exists rules with their synthetic
   negatives firing.
2. Zero ledger entries match `grep -E "justified-baseline.*(warrants a|future|follow-up)"
   reports/0020_mutants_baseline_disposition.md`.
3. Every `warrants-test:<ticket-id>` entry's ticket file exists under `tickets/` or
   `archive/tickets/`; every retired baseline entry's killing test is named in this
   ticket's implementation notes and passes.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Baseline↔ledger bidirectional parity holds at the recomputed pins; the baseline file
   shrinks only via entries retired by passing focused tests.
2. The disposition vocabulary stays closed (`justified-baseline` /
   `warrants-test:<ticket-id>`); no rationale may encode deferral without a ticket.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — hardened
   `mutation_baseline_governance_errors` rules + synthetic negatives.
2. Focused mutation-killing tests at the triaged guarded-layer sites (paths surfaced by
   the triage; inline `#[cfg(test)]` per the `-014` exemplar).

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo mutants -f <retired-entry-file> --no-shuffle` per retired entry (targeted
   kill-proof), or one `cargo mutants --in-diff` run over the ticket diff.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
