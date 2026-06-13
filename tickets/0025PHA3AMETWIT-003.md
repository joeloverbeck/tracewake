# 0025PHA3AMETWIT-003: Envelope read path fail-closed — duplicate-key rejection, decode-entry posture, checksum_after decision

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` kernel event log (`crates/tracewake-core/src/events/envelope.rs`, `crates/tracewake-core/src/events/log.rs`) and test guards.
**Deps**: 0025PHA3AMETWIT-001

## Problem

Spec 0025 findings `ORD-HARD-168` (medium — the pass's product-behavior foundation
defect at the letter of INV-020), `ORD-HARD-171` (medium), `ORD-HARD-184` (low).
`EventEnvelope::deserialize_canonical` loads fields via `map.insert(key, value)`
with no occupancy check: a serialized envelope carrying a duplicated field key
(`event_type=` twice, a second `content_manifest_id=`) silently last-wins —
ambiguous history is accepted instead of rejected loudly, while sibling
malformations fail closed (`MalformedField`, `UnknownEventKind`). Separately, the
decode layer accepts any `event_schema_version` string; the gate
(`has_supported_schema_version`) lives only in `EventLog::append`/
`append_deserialized`, and `deserialize_canonical` is `pub` — a future direct
decode path bypasses INV-020. And `checksum_after` is a dead integrity field:
serialized, deserialized, written as `None` by every producer, verified by nothing
— a hollow tamper-evidence signal.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `9e33d7a`: the unguarded `map.insert` loop in
   `deserialize_canonical` (`crates/tracewake-core/src/events/envelope.rs`);
   `has_supported_schema_version` consulted only by `events/log.rs` append paths;
   `checksum_after: None` at the producer default with the decode path present.
   All operator-verified per spec 0025 §8.
2. Verified against spec 0025 §4 and doctrine: INV-020
   (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`) and the reject-loudly
   replay posture (`docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`).
   No in-repo writer emits duplicate keys (`serialize_canonical` writes each field
   once), which is why the severity is medium, not high.
3. Shared boundary under audit: the envelope canonical-serialization contract —
   the kernel's event-log read path must reject malformed or ambiguous history
   with typed errors on every entry point.
4. Invariant restated before trusting the narrative (INV-020 — event schema
   evolution is mandatory): replay must reject unsupported history rather than
   silently inventing repairs; last-wins on a duplicated field is a silent repair
   of ambiguous input.
5. Deterministic-replay surface touched: this ticket adds rejections to the decode
   path and (per the `checksum_after` decision) may add per-event replay
   verification. No accepted input's decoding changes; identical valid inputs
   produce identical envelopes; rejection of previously-silently-accepted
   malformed input cannot weaken determinism or leak knowledge.
6. Schema extension: `EventEnvelopeParseError` (`envelope.rs`) gains a typed
   variant (e.g. `DuplicateField`). Consumers are the decode callers matching on
   the error (the log layer and tests); adding a variant is breaking for
   exhaustive matches — enumerate and update matchers in the same diff. If the
   `checksum_after` decision is **delete**, the envelope serialized shape changes:
   reprice the envelope round-trip goldens once, honestly, in this ticket.
7. Removal blast radius (conditional on the `checksum_after` decision = delete):
   `EventEnvelope.checksum_after` field, its `None` producer defaults
   (`pipeline.rs` ×2, `projections.rs`, `view_models.rs`, guard fixtures), the
   serialize/decode lines. The same-named `checksum_after` fields on
   `debug_reports.rs::…` and `actions/report.rs::…` are **distinct structs and not
   in scope** — grep-verified at decomposition; do not touch them.

## Architecture Check

1. Rejecting duplicates at the decode boundary (and gating the version at decode or
   sealing the entry to `pub(crate)`) is cleaner than relying on the log layer
   alone: the read contract becomes self-protecting, so every future caller
   inherits fail-closed behavior instead of re-implementing it — the same
   perimeter-completion reasoning as the derived apply-mutator census
   (`ORD-HARD-144`, verified holding).
2. No backwards-compatibility aliasing/shims: malformed input is rejected, not
   normalized; if `checksum_after` is deleted, no legacy parse path tolerates the
   old field.

## Verification Layers

1. INV-020 reject-loudly (`ORD-HARD-168`) → negative feeding a duplicated
   `content_manifest_id=` line through `deserialize_canonical`, asserting the
   typed duplicate-field error.
2. Decode-entry posture (`ORD-HARD-171`) → per the recorded decision: a `v999`
   direct-decode negative (gate-at-decode), or a visibility grep-proof that
   `deserialize_canonical` is `pub(crate)` with `log.rs` the sole external entry.
3. `checksum_after` decision (`ORD-HARD-184`) → populate-and-verify: replay
   asserts recomputed == recorded per world/agent event; delete: grep-proof the
   field is gone from `envelope.rs` and goldens repriced.
4. Meta-enrollment → the new negatives enroll in `META_LOCK_REGISTRY` under the
   0025PHA3AMETWIT-001 repaired routing with measured witnesses.
5. Whole-suite regression → four-gate workspace run.

## What to Change

### 1. Duplicate-key rejection (`ORD-HARD-168`)

In `deserialize_canonical`, error with a typed `EventEnvelopeParseError` variant
when `map.insert` returns `Some(_)`; update exhaustive matchers.

### 2. Decode-entry posture (`ORD-HARD-171`) — implementer-recorded choice

Decide and record (per spec 0025 §6, in the kernel conformance-row home): gate
`deserialize_canonical` against the supported-version registry (spec-recommended —
cheaper than a visibility audit), or restrict it to `pub(crate)` and record the log
layer as the sole legal decode entry. Implement the chosen posture with its
negative.

### 3. `checksum_after` decision (`ORD-HARD-184`) — implementer-recorded choice

Decide and record: populate `checksum_after` at commit and verify per-event during
replay (divergence localization), or delete the field and reprice the round-trip
goldens once. Either way the hollow always-`None` signal is eliminated.

### 4. Negatives and enrollment

Add the duplicated-key negative, the posture negative from change area 2, and (per
the decision) the replay-verification positive or deletion grep-proof; enroll in
the meta-lock registry.

## Files to Touch

- `crates/tracewake-core/src/events/envelope.rs` (modify)
- `crates/tracewake-core/src/events/log.rs` (modify — only as the recorded decode-entry posture requires)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- The `checksum_after`-named fields on `debug_reports.rs` and `actions/report.rs`
  (distinct structs; Assumption item 7).
- Content-crate fingerprint honesty (`ORD-HARD-169`/`170`) — ticket
  `0025PHA3AMETWIT-004`.
- Any doctrine amendment (the added rejections enforce INV-020; they do not change
  replay semantics for accepted input).

## Acceptance Criteria

### Tests That Must Pass

1. A duplicated-field envelope fails `deserialize_canonical` with the typed code;
   all existing round-trip and replay tests pass (repriced only if the field is
   deleted).
2. The recorded decode-entry posture's proof passes (the `v999` direct-decode
   negative, or the `pub(crate)` grep-proof); both recorded decisions appear in the
   §6-named doc home.
3. `cargo test -p tracewake-core --test anti_regression_guards` and the four gates
   (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. Every envelope read path rejects ambiguous or unsupported history with a typed
   error — no silent last-wins, no ungated version acceptance reachable from
   outside the log layer.
2. No integrity-shaped field exists that nothing populates or verifies.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/events/envelope.rs` (`#[cfg(test)]`) — duplicated-key
   negative; posture negative; decision-dependent replay-verification positive or
   deletion proof.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — registry enrollment
   rows + measured witnesses for the new locks.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
