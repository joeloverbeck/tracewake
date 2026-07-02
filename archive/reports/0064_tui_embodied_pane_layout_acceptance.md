# 0064 TUI Embodied Pane Layout Acceptance

Status: accepted for ticket evidence closeout on 2026-07-02.

This archived artifact records evidence for `0064TUIEMBPAN-005`, the capstone
report for spec `0064` after tickets `001` through `004` landed. It is
observer-only, non-diegetic evidence: it adds no production logic, no simulation
fact, no doctrine, no new gate, and no fallback path.

## Commit Roles

- Implementation commits:
  - `edeffdb` `0064TUIEMBPAN-001` pane taxonomy and stable layout regions.
  - `9f49307` `0064TUIEMBPAN-002` actor-safe pane bindings.
  - `84c8b7e` `0064TUIEMBPAN-003` `ratatui` buffer renderer and fixed-size snapshots.
  - `e699dbb` `0064TUIEMBPAN-004` responsive collapse, truncation, non-vacuity, and floor behavior.
- Evidence/report commit: the commit that adds this report and archives
  `0064TUIEMBPAN-005`.
- Archive/truthing commit: the commit that moves this report to
  `archive/reports/0064_tui_embodied_pane_layout_acceptance.md`, archives the
  spec, and adds the ledger row.

## Gate Evidence

The following commands passed against implementation commit `e699dbb` with only
this report, the active spec update, and ticket `0064TUIEMBPAN-005`
uncommitted:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p tracewake-tui
```

The following ticket-scoped witnesses also passed during implementation:

```sh
cargo test -p tracewake-tui pane_layout
cargo build -p tracewake-tui --all-targets --locked
cargo test -p tracewake-tui pane_bindings
cargo test -p tracewake-tui buffer_render
cargo test -p tracewake-tui --test embodied_pane_buffer
cargo test -p tracewake-tui --test embodied_pane_responsive
cargo build --workspace --all-targets --locked
cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree
cargo test -p tracewake-core --test anti_regression_guards workspace_dependency_posture_matches_allowlist
grep -rn "ratatui" crates/tracewake-core crates/tracewake-content
```

The `grep` command produced no output. `ratatui` is confined to
`tracewake-tui`; `insta` is confined to `tracewake-tui` dev-dependencies.

## Spec 0064 Witness Matrix

| Spec 0064 obligation | Behavior witness | Status |
|---|---|---|
| Stable pane taxonomy and priority ordering | `screen::pane_layout::tests::pane_layout_region_order_is_deterministic_and_priority_ordered` | Pass |
| Exhaustive pane-to-region ownership | `screen::pane_layout::tests::every_screen_pane_is_claimed_by_exactly_one_region` | Pass |
| Actor-safe binding lines, including co-present activity disposition and identity-only rows | `screen::pane_bindings::tests::render_pane_region_bindings_emits_actor_safe_region_lines` and `binding_renderer_reuses_activity_disposition_format` | Pass |
| Actor-known-only negative: no non-diegetic debug marker in embodied pane lines | `screen::pane_bindings::tests::embodied_region_lines_carry_no_non_diegetic_debug_marker` | Pass |
| Header debug availability without debug truth | `screen::pane_bindings::tests::header_binding_uses_debug_availability_without_debug_truth` | Pass |
| `ratatui` buffer path is deterministic and actor-safe | `screen::buffer_render::tests::buffer_render_is_deterministic_for_identical_input` and `buffer_render_draws_actor_safe_binding_content` | Pass |
| Fixed-size pane text and buffer snapshots at `80x24`, `100x30`, and `60x20` | `embodied_pane_text_goldens_match_fixed_sizes` and `embodied_pane_buffer_snapshots_match_fixed_sizes` in `crates/tracewake-tui/tests/embodied_pane_buffer.rs` | Pass |
| Buffer/text parity for critical actor-safe content | `embodied_pane_buffer_and_text_expose_same_actor_safe_content` | Pass |
| Narrow collapse retains why-not, actions, and needs at `60x20` | `narrow_60x20_retains_why_not_actions_and_needs` in `crates/tracewake-tui/tests/embodied_pane_responsive.rs` | Pass |
| Overflow is explicit and expansion recovers elided entries | `overflow_emits_marker_and_expansion_recovers_elided_actions` and `visible_region_lines_marks_truncation_and_expansion_recovers_content` | Pass |
| Non-vacuity witness fails when actions clip to zero visible lines | `non_vacuity_fails_when_actions_clip_to_no_visible_lines` | Pass |
| Minimum-size floor produces an actor-safe hard-fail message | `below_floor_rendering_yields_actor_safe_message` | Pass |

## Snapshot Evidence

- Pane-text goldens:
  - `crates/tracewake-tui/tests/goldens/embodied_pane_80x24.txt`
  - `crates/tracewake-tui/tests/goldens/embodied_pane_100x30.txt`
  - `crates/tracewake-tui/tests/goldens/embodied_pane_60x20.txt`
- `ratatui` buffer snapshots:
  - `crates/tracewake-tui/tests/snapshots/embodied_pane_buffer__embodied_pane_buffer_80x24.snap`
  - `crates/tracewake-tui/tests/snapshots/embodied_pane_buffer__embodied_pane_buffer_100x30.snap`
  - `crates/tracewake-tui/tests/snapshots/embodied_pane_buffer__embodied_pane_buffer_60x20.snap`

The snapshots are byte-stable under repeated runs of
`cargo test -p tracewake-tui --test embodied_pane_buffer`.

## Anti-Contamination Results

- Pane layout and buffer rendering consume `EmbodiedScreenModel` and
  presentation bindings only.
- The buffer renderer adds no event loop, raw terminal mode, command dispatch,
  source-context construction, validator bypass, hidden-truth query, or world
  mutation path.
- Debug availability is rendered as an actor-safe availability marker; debug
  truth and the non-diegetic debug marker remain absent from embodied pane
  content.
- Truncation is explicit and presentation-only. It does not change the
  underlying actor-known content set, and expansion helpers recover the elided
  actor-safe lines.
- Core/content remain UI-free: `grep -rn "ratatui" crates/tracewake-core
  crates/tracewake-content` produced no output.

## Verdict

Spec 0064 is accepted for the scoped TUI pane-layout feature line through
implementation commit `e699dbb`: stable embodied pane regions, actor-safe
bindings, a deterministic `ratatui` buffer path, fixed-size snapshots,
buffer/text parity, responsive retention of why-not/actions/needs, explicit
truncation, non-vacuity, and a minimum-size floor all have named passing
witnesses. Evidence/report commit `b8437db` added this report and archived
`0064TUIEMBPAN-005`; the archive/truthing commit moves this report/spec and
adds the ledger row. This report does not certify latest main after the
archive/truthing commit, Phase-4 entry, second-proof entry, institutions,
notices, travel, LOD, LLM/speech, story-sifting, `P0-CERT`,
`FIRST-PROOF-CERT`, or any whole-project status. It mints no invariant, gate,
glossary term, or risk ID.
