# 0042 ORD-LIFE-CERT Mutation Triage Register

Date: 2026-06-20
Ticket: `archive/tickets/0042ORDLIFCER-015.md`
Spec: `specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md`

## Summary

The checked-in cargo-mutants configuration was expanded additively to include the explicit ORD-LIFE §7.2 perimeter entries missing from the standing configuration:

- `crates/tracewake-core/src/need_accounting.rs`
- `crates/tracewake-core/src/actions/registry.rs`
- `crates/tracewake-core/src/actions/defs/need_events.rs`
- `crates/tracewake-core/src/actions/defs/wait.rs`
- `crates/tracewake-core/src/actions/defs/continue_routine.rs`
- `crates/tracewake-core/src/actions/defs/movement.rs`

The expanded file census is 60 files and the expanded mutant census is 2877 mutants. The required `cargo mutants --workspace --no-shuffle` lane did not complete: its PTY wrapper remained active after no cargo-mutants process was visible, so it was interrupted and retained as tool-failure evidence. Before interruption, it found three missed mutants in `crates/tracewake-core/src/need_accounting.rs`. A deterministic `-j 8` rerun, matching the prior EPI-CERT register shape, also failed to produce a complete summary and was interrupted after its wrapper remained active without a visible cargo-mutants process.

Final mutation disposition: `ORD-LIFE-CERT scoped remediation`. The mutation floor is not passable because the configured run did not complete and the partial exact run exposed actionable missed mutants. No missed mutant is accepted as equivalent or non-critical.

The post-config workspace suite remained green after the checked-in perimeter expansion.

## Evidence Artifacts

| Artifact | Purpose |
|---|---|
| `/tmp/0042-015-cargo-install-mutants.txt` | cargo-mutants 27.1.0 install command transcript, approved network rerun |
| `/tmp/0042-015-cargo-mutants-version.txt` | cargo-mutants version transcript |
| `/tmp/0042-015-mutants-list-files.txt` | Expanded file census, 60 files |
| `/tmp/0042-015-mutants-list.txt` | Expanded mutant census, 2877 mutants |
| `/tmp/0042-015-mutants-no-shuffle.txt` | Exact configured full-run transcript; incomplete/tool-failure with three missed mutants |
| `/tmp/0042-015-mutants-no-shuffle-j8.txt` | Deterministic parallel rerun transcript; incomplete/tool-failure |
| `/tmp/0042-015-workspace-test.txt` | Post-config workspace test transcript |

## Fingerprints

```text
258109189fc4500ab88c6fa28f47d06292632ac9b274a237589e49a90ef05b11  .cargo/mutants.toml
f207f6208d64c1a7cce0daa27c00add40a52edb99b1a6ce3a1301cb44ef60a59  Cargo.lock
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  .cargo/mutants-baseline-misses.txt
927b01f06a469ebfdff63c0bdd18d8b755fd948d47b76557f14e93e2d40ad1bc  /tmp/0042-015-cargo-install-mutants.txt
ba3c35073708a1da94eef75cafca1e263013077a73d090b5a798135dac2ffbe5  /tmp/0042-015-cargo-mutants-version.txt
510a1c626efda2b5118a6244184bf071c1a1fcda5237d9fa572861938046e50e  /tmp/0042-015-mutants-list-files.txt
d9b430fbe111f5ea9a8c9db70bec032b3296444b38dac7a78aeecf672444c0a7  /tmp/0042-015-mutants-list.txt
89f1b0632896dae694d1f04b2cdf0041ab8d0cb4b7c6868964bfe799478c6108  /tmp/0042-015-mutants-no-shuffle.txt
203c5371724d970ddb1dbd0ec4bcb5d0b112f87f1dbdd4008ba169f32a71850c  /tmp/0042-015-mutants-no-shuffle-j8.txt
a770a77041c9d6b22dc8f87cef4cb09ed79de2093a74b8caea0989cc26fc4756  /tmp/0042-015-workspace-test.txt
```

## Register

Every missed-mutant row has certified reachability `behavior-relevant until remediated`, behavior witness `missing`, negative/contamination control `not yet killing this mutant`, disposition `routed remediation`, and review signoff `not accepted for certification`. Tool-failure rows are not accepted for certification and require a later complete configured rerun.

| Mutant identity | ORD-LIFE cross-ref | Responsible layer | Tool outcome | Evidence reference |
|---|---|---|---|---|
| `need_accounting.rs:88:25 replace < with <= in DurationInterval::contains_tick` | `ORD-LIFE-01`, `ORD-LIFE-08`, `ORD-LIFE-12` | need accounting; duration interval boundary; replay/accounting derivation | `missed` | `/tmp/0042-015-mutants-no-shuffle.txt` |
| `need_accounting.rs:106:13 replace && with || in duration_intervals` | `ORD-LIFE-01`, `ORD-LIFE-08`, `ORD-LIFE-12` | need accounting; duration interval selection; replay/accounting derivation | `missed` | `/tmp/0042-015-mutants-no-shuffle.txt` |
| `need_accounting.rs:109:45 replace == with != in duration_intervals` | `ORD-LIFE-01`, `ORD-LIFE-08`, `ORD-LIFE-12` | need accounting; actor/event interval ownership; replay/accounting derivation | `missed` | `/tmp/0042-015-mutants-no-shuffle.txt` |
| `cargo mutants --workspace --no-shuffle` exact configured lane | `ORD-LIFE-CERT mutation floor` | mutation infrastructure; configured perimeter completion | `tool-failure/incomplete; interrupted after no cargo-mutants process was visible` | `/tmp/0042-015-mutants-no-shuffle.txt` |
| `cargo mutants --workspace --no-shuffle -j 8` deterministic rerun lane | `ORD-LIFE-CERT mutation floor` | mutation infrastructure; configured perimeter completion | `tool-failure/incomplete; interrupted after no cargo-mutants process was visible` | `/tmp/0042-015-mutants-no-shuffle-j8.txt` |
