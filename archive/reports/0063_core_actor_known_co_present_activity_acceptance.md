# 0063 Core Actor-Known Co-Present Activity Acceptance

Status: accepted for ticket evidence closeout on 2026-07-01.

This artifact records evidence for `0063CORACTKNO-006`, the capstone report for
spec `0063` after tickets `001` through `005` landed. It is observer-only,
non-diegetic evidence: it adds no production logic, no simulation fact, no
doctrine, no new gate, and no fallback path.

## Commit Roles

- Implementation commits:
  - `318648a` `0063CORACTKNO-001` observed activity taxonomy.
  - `fb8bb54` `0063CORACTKNO-002` actor-known activity sealing.
  - `524478c` `0063CORACTKNO-003` visible actor activity transfer.
  - `e199f9e` `0063CORACTKNO-004` content fixtures.
  - `a02f54064fe332a961926da6f0d566c19222d75b` `0063CORACTKNO-005` render disposition and witness suite.
- Evidence/report commit: the commit that adds this report and archives `0063CORACTKNO-006`.

## Gate Evidence

The following commands passed against implementation commit
`a02f54064fe332a961926da6f0d566c19222d75b` with only this report and ticket
006 uncommitted:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

Focused witnesses also passed while ticket 005 was current:

```sh
cargo test -p tracewake-core --test hidden_truth_gates
cargo test -p tracewake-tui
```

## Spec 0063 Witness Matrix

| Spec 0063 obligation | Behavior witness | Status |
|---|---|---|
| Positive co-present observed activity reaches actor-known view model | `actor_known_observed_activity_reaches_embodied_visible_actor` in `crates/tracewake-core/tests/hidden_truth_gates.rs` | Pass |
| Anti-leak: co-presence without perceived activity does not infer activity | `unperceived_activity_truth_does_not_create_visible_actor_activity` in `crates/tracewake-core/tests/hidden_truth_gates.rs` | Pass |
| Occlusion: hidden/separate actor with no sealed fact produces no row | `occluded_actor_absent_from_actor_known_context_produces_no_visible_actor_row` in `crates/tracewake-core/tests/hidden_truth_gates.rs` | Pass |
| Stale remembered activity carries stale wording and uncertainty | `stale_actor_known_activity_preserves_stale_disposition` in `crates/tracewake-core/tests/hidden_truth_gates.rs` and `render_actor_line_disposes_observed_activity_and_presence` in `crates/tracewake-tui/src/screen/text_dump.rs` | Pass |
| Possession parity: each holder sees its own actor-known local activity row | `possession_parity_uses_each_holder_actor_known_local_activity` in `crates/tracewake-core/tests/hidden_truth_gates.rs` | Pass |
| Hop 1: sealed local actor activity reaches `VisibleActor.observed_activity` | `actor_known_observed_activity_reaches_embodied_visible_actor` and the existing `actor_known_local_actor_reaches_embodied_view_model_with_context_provenance` | Pass |
| Hop 2: `VisibleActor.observed_activity` reaches rendered/dumped actor pane | `renderer_prints_actor_known_activity_from_visible_actor`, `render_actor_line_disposes_observed_activity_and_presence`, and `structured_pane_dumps_match_plain_text_pane_content` | Pass |
| Hop-removal/source guard: TUI must use explicit actor formatter and closed enum disposition | `embodied_screen_model_field_disposition` and `closed_presentation_enum_matches_are_exhaustive_without_wildcards` in `crates/tracewake-tui/tests/tui_seam_conformance.rs` | Pass |
| Fixture matrix exists for later scenario reuse | `co_present_actor_visible_working_001`, `co_present_actor_activity_not_apparent_001`, `co_present_actor_occluded_no_row_001`, and `co_present_actor_stale_remembered_activity_001` registered in `crates/tracewake-content/src/fixtures/mod.rs` | Pass |

## Anti-Contamination Results

- The TUI render and dump path reads `VisibleActor` fields only; it does not
  join against actor id, action registry, routine state, scheduler state, or
  physical truth.
- The actor formatter has explicit match arms for every
  `ObservedActorActivityKind` and `ActorKnownActivitySourceKind`; conformance
  fails on an undisposed new variant.
- Ordinary actor-pane output renders actor-safe source kind, tick, staleness,
  and uncertainty, but not raw event ids or source-summary internals.
- The core witnesses use sealed `KnowledgeContext` rows at the existing
  hidden-truth seam. `tracewake-core` does not depend on `tracewake-content`, so
  fixture import is intentionally left to content/TUI integration surfaces.

## Verdict

Spec 0063 is accepted for the scoped feature line at implementation commit
`a02f54064fe332a961926da6f0d566c19222d75b`: positive activity, anti-leak
omission, occlusion omission, stale activity, possession parity, and the
two-hop core-to-TUI path all have named passing behavior witnesses. This report
does not certify latest main after the evidence/report commit and does not
claim broader co-present pane layout, identity-uncertain actors, Spec 0064, or
Spec 0069 parity-registry work.
